#![allow(clippy::identity_op)]
#![allow(clippy::eq_op)]
#![allow(clippy::result_unit_err)]

use super::wdt;
use tc37x_pac::hidden::RegValue;
use tc37x_pac::SCU;

// const SYSPLLSTAT_PWDSTAT_TIMEOUT_COUNT: usize = 0x3000;
// const OSCCON_PLLLV_OR_HV_TIMEOUT_COUNT: usize = 0x493E0;
// const PLL_LOCK_TIMEOUT_COUNT: usize = 0x3000;

const CCUCON_LCK_BIT_TIMEOUT_COUNT: usize = 0x1000;
const PLL_KRDY_TIMEOUT_COUNT: usize = 0x6000;

pub enum InitError {
    ConfigureCCUInitialStep,
    ModulationInit,
    DistributeClockInline,
    ThrottleSysPllClockInline,
}

pub fn init(config: &Config) -> Result<(), InitError> {
    configure_ccu_initial_step(&config).map_err(|_| InitError::ConfigureCCUInitialStep)?;
    modulation_init(&config).map_err(|_| InitError::ModulationInit)?;
    distribute_clock_inline(&config).map_err(|_| InitError::DistributeClockInline)?;
    throttle_sys_pll_clock_inline(&config).map_err(|_| InitError::ThrottleSysPllClockInline)?;
    Ok(())
}

pub fn configure_ccu_initial_step(_config: &Config) -> Result<(), ()> {
    // TODO Use config
    let endinit_sfty_pw = wdt::get_safety_watchdog_password();
    wdt::clear_safety_endinit_inline(endinit_sfty_pw);

    wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| unsafe { SCU.ccucon0().read() }.lck().get())?;

    Ok(())
}

pub fn modulation_init(config: &Config) -> Result<(), ()> {
    if let ModulationEn::Enabled = config.modulation.enable {
        let rgain_p = calc_rgain_parameters(config.modulation.amp);

        let endinit_sfty_pw = wdt::get_safety_watchdog_password();
        wdt::clear_safety_endinit_inline(endinit_sfty_pw);

        unsafe {
            SCU.syspllcon2()
                .modify(|r| r.modcfg().set((0x3 << 10) | rgain_p.rgain_hex))
        };

        unsafe { SCU.syspllcon0().modify(|r| r.moden().set(true)) };

        wdt::set_safety_endinit_inline(endinit_sfty_pw);
    }
    Ok(())
}

// TODO revise this struct (annabo)
pub struct RGainValues {
    pub rgain_nom: f32,
    pub rgain_hex: u16,
}

fn calc_rgain_parameters(modamp: ModulationAmplitude) -> RGainValues {
    const MA_PERCENT: [f32; 6] = [0.5, 1.0, 1.25, 1.5, 2.0, 2.5];

    let mod_amp = MA_PERCENT[modamp as usize];
    let fosc_hz = get_osc_frequency();
    let syspllcon0 = unsafe { SCU.syspllcon0().read() };
    let fdco_hz =
        (fosc_hz * (syspllcon0.ndiv().get() as f32 + 1.0)) / (syspllcon0.pdiv().get() as f32 + 1.0);

    let rgain_nom = 2.0 * (mod_amp / 100.0) * (fdco_hz / 3600000.0);
    let rgain_hex = ((rgain_nom * 32.0) + 0.5) as u16;

    RGainValues {
        rgain_nom,
        rgain_hex,
    }
}

pub fn distribute_clock_inline(config: &Config) -> Result<(), ()> {
    let endinit_sfty_pw = wdt::get_safety_watchdog_password();
    wdt::clear_safety_endinit_inline(endinit_sfty_pw);

    // CCUCON0 config
    {
        let mut cuccon0 = unsafe { SCU.ccucon0().read() };
        *cuccon0.data_mut_ref() &= !(config.clock_distribution.ccucon0.mask);
        *cuccon0.data_mut_ref() |=
            config.clock_distribution.ccucon0.mask & config.clock_distribution.ccucon0.value;

        wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| unsafe { SCU.ccucon0().read() }.lck().get())?;

        unsafe { SCU.ccucon0().write(cuccon0) };

        wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| unsafe { SCU.ccucon0().read() }.lck().get())?;
    }
    // CCUCON1 config
    {
        let mut ccucon1 = unsafe { SCU.ccucon1().read() };
        if ccucon1.clkselmcan().get() !=  0 /*ccucon1::Clkselmcan::CLKSELMCAN_STOPPED*/
            || ccucon1.clkselmsc().get() != 1/*ccucon1::Clkselmsc::CLKSELMSC_STOPPED*/
            || ccucon1.clkselqspi().get() != 2
        /*ccucon1::Clkselqspi::CLKSELQSPI_STOPPED*/
        {
            *ccucon1.data_mut_ref() &= !config.clock_distribution.ccucon1.mask;
            *ccucon1.data_mut_ref() |=
                config.clock_distribution.ccucon1.mask & config.clock_distribution.ccucon1.value;

            ccucon1 = ccucon1
                .clkselmcan()
                .set(0 /*ccucon1::Clkselmcan::CLKSELMCAN_STOPPED*/)
                .clkselmsc()
                .set(1 /*ccucon1::Clkselmsc::CLKSELMSC_STOPPED*/)
                .clkselqspi()
                .set(2 /*ccucon1::Clkselqspi::CLKSELQSPI_STOPPED*/);

            wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| {
                unsafe { SCU.ccucon1().read() }.lck().get()
            })?;
            unsafe { SCU.ccucon1().write(ccucon1) };
            wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| {
                unsafe { SCU.ccucon1().read() }.lck().get()
            })?;
        }

        ccucon1 = unsafe { SCU.ccucon1().read() };
        *ccucon1.data_mut_ref() &= !config.clock_distribution.ccucon1.mask;
        *ccucon1.data_mut_ref() |=
            config.clock_distribution.ccucon1.mask & config.clock_distribution.ccucon1.value;

        wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| unsafe { SCU.ccucon1().read() }.lck().get())?;
        unsafe { SCU.ccucon1().write(ccucon1) };
        wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| unsafe { SCU.ccucon1().read() }.lck().get())?;
    }

    // CCUCON2 config
    {
        let mut ccucon2 = unsafe { SCU.ccucon2().read() };
        if ccucon2.clkselasclins().get() != 0
        /*scu::Ccucon2::Clkselasclins::CLKSELASCLINS_STOPPED*/
        {
            ccucon2 = unsafe { SCU.ccucon2().read() };
            *ccucon2.data_mut_ref() &= !config.clock_distribution.ccucon2.mask;
            *ccucon2.data_mut_ref() =
                config.clock_distribution.ccucon2.mask & config.clock_distribution.ccucon2.value;

            ccucon2 = ccucon2.clkselasclins().set(
                0, /*scu::ccucon2::Clkselasclins::CLKSELASCLINS_STOPPED*/
            );

            wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| {
                unsafe { SCU.ccucon2().read() }.lck().get()
            })?;

            unsafe { SCU.ccucon2().write(ccucon2) };

            wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| {
                unsafe { SCU.ccucon2().read() }.lck().get()
            })?;
        }

        ccucon2 = unsafe { SCU.ccucon2().read() };
        *ccucon2.data_mut_ref() &= !config.clock_distribution.ccucon2.mask;
        *ccucon2.data_mut_ref() |=
            config.clock_distribution.ccucon2.mask & config.clock_distribution.ccucon2.value;

        wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| unsafe { SCU.ccucon2().read() }.lck().get())?;
        unsafe { SCU.ccucon2().write(ccucon2) };
        wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| unsafe { SCU.ccucon2().read() }.lck().get())?;
    }

    // CUCCON5 config
    {
        let mut ccucon5 = unsafe { SCU.ccucon5().read() };
        *ccucon5.data_mut_ref() &= !config.clock_distribution.ccucon5.mask;
        *ccucon5.data_mut_ref() |=
            config.clock_distribution.ccucon5.mask & config.clock_distribution.ccucon5.value;
        ccucon5 = ccucon5.up().set(true);

        wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| unsafe { SCU.ccucon5().read() }.lck().get())?;

        unsafe { SCU.ccucon5().write(ccucon5) };

        wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| unsafe { SCU.ccucon5().read() }.lck().get())?;
    }

    // CUCCON6 config
    {
        unsafe {
            SCU.ccucon6().modify(|mut r| {
                *r.data_mut_ref() &= !config.clock_distribution.ccucon6.mask;
                *r.data_mut_ref() |= config.clock_distribution.ccucon6.mask
                    & config.clock_distribution.ccucon6.value;
                r
            })
        };
    }

    // CUCCON7 config
    {
        unsafe {
            SCU.ccucon7().modify(|mut r| {
                *r.data_mut_ref() &= !config.clock_distribution.ccucon7.mask;
                *r.data_mut_ref() |= config.clock_distribution.ccucon7.mask
                    & config.clock_distribution.ccucon7.value;
                r
            })
        };
    }

    // CUCCON8 config
    {
        unsafe {
            SCU.ccucon8().modify(|mut r| {
                *r.data_mut_ref() &= !config.clock_distribution.ccucon8.mask;
                *r.data_mut_ref() |= config.clock_distribution.ccucon8.mask
                    & config.clock_distribution.ccucon8.value;
                r
            })
        };
    }

    wdt::set_safety_endinit_inline(endinit_sfty_pw);

    Ok(())
}

pub fn throttle_sys_pll_clock_inline(config: &Config) -> Result<(), ()> {
    let endinit_sfty_pw = wdt::get_safety_watchdog_password();

    for pll_step_count in 0..config.sys_pll_throttle.len() {
        wdt::clear_safety_endinit_inline(endinit_sfty_pw);

        wait_cond::<PLL_KRDY_TIMEOUT_COUNT>(|| !unsafe { SCU.syspllstat().read() }.k2rdy().get())?;

        unsafe {
            SCU.syspllcon1().modify(|r| {
                r.k2div()
                    .set(config.sys_pll_throttle[pll_step_count].k2_step)
            })
        };

        wdt::set_safety_endinit_inline(endinit_sfty_pw);
    }
    Ok(())
}

#[inline]
pub fn wait_cond<const C: usize>(cond: impl Fn() -> bool) -> Result<(), ()> {
    let mut timeout_cycle_count = C;
    while cond() {
        timeout_cycle_count -= 1;
        if timeout_cycle_count == 0 {
            return Err(());
        }
    }

    Ok(())
}

// PLL management
const EVR_OSC_FREQUENCY: f32 = 100000000.0;
const XTAL_FREQUENCY: u32 = 20000000;
const SYSCLK_FREQUENCY: u32 = 20000000;

#[inline]
pub fn get_osc_frequency() -> f32 {
    match unsafe { SCU.syspllcon0().read() }.insel().get() {
        0 => EVR_OSC_FREQUENCY,
        1 => XTAL_FREQUENCY as f32,
        2 => SYSCLK_FREQUENCY as f32,
        _ => 0.0,
    }
}

pub fn get_pll_frequency() -> f32 {
    let osc_freq = get_osc_frequency();
    let syspllcon0 = unsafe { SCU.syspllcon0().read() };
    let syspllcon1 = unsafe { SCU.syspllcon1().read() };
    (osc_freq * (syspllcon0.ndiv().get() + 1) as f32)
        / ((syspllcon1.k2div().get() + 1) * (syspllcon0.pdiv().get() + 1)) as f32
}

pub fn get_per_pll_frequency1() -> f32 {
    let osc_freq = get_osc_frequency();
    let perpllcon0 = unsafe { SCU.perpllcon0().read() };
    let perpllcon1 = unsafe { SCU.perpllcon1().read() };
    (osc_freq * (perpllcon0.ndiv().get() + 1) as f32)
        / ((perpllcon0.pdiv().get() + 1) * (perpllcon1.k2div().get() + 1)) as f32
}

pub fn get_per_pll_frequency2() -> f32 {
    let osc_freq = get_osc_frequency();
    let perpllcon0 = unsafe { SCU.perpllcon0().read() };
    let perpllcon1 = unsafe { SCU.perpllcon1().read() };

    let multiplier = if !perpllcon0.divby().get() { 1.6 } else { 2.0 };
    (osc_freq * (perpllcon0.ndiv().get() + 1) as f32)
        / ((perpllcon0.pdiv().get() + 1) as f32
            * (perpllcon1.k2div().get() + 1) as f32
            * multiplier)
}

pub struct SysPllConfig {
    pub p_divider: u8,
    pub n_divider: u8,
    pub k2_divider: u8,
}

pub struct PerPllConfig {
    pub p_divider: u8,
    pub n_divider: u8,
    pub k2_divider: u8,
    pub k3_divider: u8,
    pub k3_divider_bypass: u8,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum PllInputClockSelection {
    F0sc1,
    F0sc0,
    FSynclk,
}

pub struct PllsParameterConfig {
    pub xtal_frequency: u32,
    pub pll_input_clock_selection: PllInputClockSelection,
    pub sys_pll: SysPllConfig,
    pub per_pll: PerPllConfig,
}

pub struct InitialConfigStep {
    pub plls_parameters: PllsParameterConfig,
    pub wait_time: f32,
}

pub struct PllStepConfig {
    pub k2_step: u8,
    pub wait_time: f32,
}

pub struct ConRegConfig {
    pub value: u32,
    pub mask: u32,
}

pub struct ClockDistributionConfig {
    pub ccucon0: ConRegConfig,
    pub ccucon1: ConRegConfig,
    pub ccucon2: ConRegConfig,
    pub ccucon5: ConRegConfig,
    pub ccucon6: ConRegConfig,
    pub ccucon7: ConRegConfig,
    pub ccucon8: ConRegConfig,
}

pub struct FlashWaitStateConfig {
    pub value: u32,
    pub mask: u32,
}

#[repr(u8)]
pub enum ModulationEn {
    Disabled,
    Enabled,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ModulationAmplitude {
    _0p5,
    _1p0,
    _1p25,
    _1p5,
    _2p0,
    _2p5,
    Count,
}

pub struct ModulationConfig {
    pub enable: ModulationEn,
    pub amp: ModulationAmplitude,
}

pub struct Config {
    pub pll_initial_step: InitialConfigStep,
    pub sys_pll_throttle: &'static [PllStepConfig],
    pub clock_distribution: ClockDistributionConfig,
    pub flash_wait_state: FlashWaitStateConfig,
    pub modulation: ModulationConfig,
}

pub const DEFAULT_PLL_CONFIG_STEPS: [PllStepConfig; 3] = [
    PllStepConfig {
        k2_step: 4 - 1,
        wait_time: 0.000100,
    },
    PllStepConfig {
        k2_step: 3 - 1,
        wait_time: 0.000100,
    },
    PllStepConfig {
        k2_step: 2 - 1,
        wait_time: 0.000100,
    },
];

pub const DEFAULT_CLOCK_CONFIG: Config = Config {
    pll_initial_step: InitialConfigStep {
        plls_parameters: PllsParameterConfig {
            xtal_frequency: 20000000,
            pll_input_clock_selection: PllInputClockSelection::F0sc0,
            sys_pll: SysPllConfig {
                p_divider: 1 - 1,
                n_divider: 30 - 1,
                k2_divider: 6 - 1,
            },
            per_pll: PerPllConfig {
                p_divider: 1 - 1,
                n_divider: 32 - 1,
                k2_divider: 2 - 1,
                k3_divider: 2 - 1,
                k3_divider_bypass: 0,
            },
        },
        wait_time: 0.000200,
    },
    sys_pll_throttle: &DEFAULT_PLL_CONFIG_STEPS,
    clock_distribution: ClockDistributionConfig {
        ccucon0: ConRegConfig {
            value: ((3) << (0))
                | ((1) << (4))
                | ((1) << (8))
                | ((3) << (16))
                | ((2) << (20))
                | (((1) * 3) << (24))
                | (((1) * 1) << (26)),
            mask: ((0xf) << (0))
                | ((0xf) << (4))
                | ((0xf) << (8))
                | ((0xf) << (16))
                | ((0xf) << (20))
                | ((0x3) << (24))
                | ((0x3) << (26)),
        },
        ccucon1: ConRegConfig {
            value: ((2) << (0))
                | ((1) << (4))
                | ((0) << (7))
                | ((2) << (8))
                | ((1) << (16))
                | ((1) << (20))
                | ((1) << (24))
                | ((2) << (28)),
            mask: ((0xf) << (0))
                | ((0x3) << (4))
                | ((0x1) << (7))
                | ((0xf) << (8))
                | ((0xf) << (16))
                | ((0x3) << (20))
                | ((0xf) << (24))
                | ((0x3) << (28)),
        },
        ccucon2: ConRegConfig {
            value: ((1) << (0)) | ((2) << (8)) | ((1) << (12)),
            mask: ((0xf) << (0)) | ((0xf) << (8)) | ((0x3) << (12)),
        },
        ccucon5: ConRegConfig {
            value: (((2) << (0)) | ((3) << (4))),
            mask: ((0xf) << (0)) | ((0xf) << (4)),
        },
        ccucon6: ConRegConfig {
            value: 0 << 0,
            mask: 0x3f << 0,
        },
        ccucon7: ConRegConfig {
            value: 0 << 0,
            mask: 0x3f << 0,
        },
        ccucon8: ConRegConfig {
            value: 0 << 0,
            mask: 0x3f << 0,
        },
    },
    flash_wait_state: FlashWaitStateConfig {
        value: 0x00000105,
        mask: 0x0000073F,
    },
    modulation: ModulationConfig {
        enable: ModulationEn::Disabled,
        amp: ModulationAmplitude::_0p5,
    },
};
