#![allow(clippy::identity_op)]
#![allow(clippy::eq_op)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::float_arithmetic)]
// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

use super::wdt;
use crate::log::debug;
use tc37x_pac::hidden::RegValue;
use tc37x_pac::scu;
use tc37x_pac::scu::ccucon0::{Bbbdiv, Fsi2Div, Fsidiv, Gtmdiv, Lpdiv, Spbdiv, Sridiv, Stmdiv};
use tc37x_pac::scu::ccucon1::{
    Clkselmcan, Clkselmsc, Clkselqspi, I2Cdiv, Mcandiv, Mscdiv, Pll1Divdis, Qspidiv,
};
use tc37x_pac::scu::ccucon2::{Asclinfdiv, Asclinsdiv, Clkselasclins};
use tc37x_pac::scu::ccucon5::{Gethdiv, Mcanhdiv};
use tc37x_pac::{RegisterValue, SCU, SMU};

const SYSPLLSTAT_PWDSTAT_TIMEOUT_COUNT: usize = 0x3000;
const OSCCON_PLLLV_OR_HV_TIMEOUT_COUNT: usize = 0x493E0;
const PLL_LOCK_TIMEOUT_COUNT: usize = 0x3000;

const CCUCON_LCK_BIT_TIMEOUT_COUNT: usize = 0x1000;
const PLL_KRDY_TIMEOUT_COUNT: usize = 0x6000;

pub enum InitError {
    ConfigureCCUInitialStep,
    ModulationInit,
    DistributeClockInline,
    ThrottleSysPllClockInline,
}

pub(crate) fn init(config: &Config) -> Result<(), InitError> {
    configure_ccu_initial_step(config).map_err(|()| InitError::ConfigureCCUInitialStep)?;
    modulation_init(config).map_err(|()| InitError::ModulationInit)?;
    distribute_clock_inline(config).map_err(|()| InitError::DistributeClockInline)?;
    throttle_sys_pll_clock_inline(config).map_err(|()| InitError::ThrottleSysPllClockInline)?;
    Ok(())
}

fn wait_ccucon0_lock() -> Result<(), ()> {
    wait_cond(CCUCON_LCK_BIT_TIMEOUT_COUNT, || {
        unsafe { SCU.ccucon0().read() }.lck().get() == scu::ccucon0::Lck::CONST_11
    })
}

fn wait_ccucon1_lock() -> Result<(), ()> {
    wait_cond(CCUCON_LCK_BIT_TIMEOUT_COUNT, || {
        unsafe { SCU.ccucon1().read() }.lck().get() == scu::ccucon1::Lck::CONST_11
    })
}

fn wait_ccucon2_lock() -> Result<(), ()> {
    wait_cond(CCUCON_LCK_BIT_TIMEOUT_COUNT, || {
        unsafe { SCU.ccucon2().read() }.lck().get() == scu::ccucon2::Lck::CONST_11
    })
}

fn wait_ccucon5_lock() -> Result<(), ()> {
    wait_cond(CCUCON_LCK_BIT_TIMEOUT_COUNT, || {
        unsafe { SCU.ccucon5().read() }.lck().get() == scu::ccucon5::Lck::CONST_11
    })
}

fn wait_divider() -> Result<(), ()> {
    wait_cond(PLL_KRDY_TIMEOUT_COUNT, || {
        let sys = unsafe { SCU.syspllstat().read() };
        let per = unsafe { SCU.perpllstat().read() };
        let sys_k2 = sys.k2rdy().get();
        let per_k2 = sys.k2rdy().get();
        let per_k3 = per.k3rdy().get();
        sys_k2.0 == 0 || per_k2.0 == 0 || per_k3.0 == 0
    })
}

fn set_pll_power(
    syspllpower: scu::syspllcon0::Pllpwd,
    perpllpower: scu::perpllcon0::Pllpwd,
) -> Result<(), ()> {
    unsafe { SCU.syspllcon0().modify(|r| r.pllpwd().set(syspllpower)) };
    unsafe { SCU.perpllcon0().modify(|r| r.pllpwd().set(perpllpower)) };

    wait_cond(SYSPLLSTAT_PWDSTAT_TIMEOUT_COUNT, || {
        let sys = unsafe { SCU.syspllstat().read() };
        let per = unsafe { SCU.perpllstat().read() };
        (syspllpower.0) == (sys.pwdstat().get().0) || (perpllpower.0) == (per.pwdstat().get().0)
    })
}

pub(crate) fn configure_ccu_initial_step(config: &Config) -> Result<(), ()> {
    // TODO Should be an enum variant in the pac crate
    const CLKSEL_BACKUP: u8 = 0;

    wdt::clear_safety_endinit_inline();

    wait_ccucon0_lock()?;

    // TODO Explain this
    unsafe {
        SCU.ccucon0().modify(|r| {
            r.clksel()
                .set(scu::ccucon0::Clksel(CLKSEL_BACKUP))
                .up()
                .set(scu::ccucon0::Up::CONST_11)
        })
    };
    wait_ccucon0_lock()?;

    // disable SMU
    {
        // The SMU core configuration is only possible if this field is set to 0xBC
        unsafe { SMU.keys().init(|r| r.cfglck().set(0xBC)) };

        // FIXME After pac update, this is a BW patch on pac
        unsafe { SMU.ag8cfj()[0].modify(|r| r.set_raw(r.get_raw() & !0x1D)) };
        unsafe { SMU.ag8cfj()[1].modify(|r| r.set_raw(r.get_raw() & !0x1D)) };
        unsafe { SMU.ag8cfj()[2].modify(|r| r.set_raw(r.get_raw() & !0x1D)) };

        unsafe { SMU.keys().init(|r| r.cfglck().set(0)) };
    }

    // Power down the both the PLLs before configuring registers
    // Both the PLLs are powered down to be sure for asynchronous PLL registers
    // update cause no glitches.
    set_pll_power(
        scu::syspllcon0::Pllpwd::CONST_00,
        scu::perpllcon0::Pllpwd::CONST_00,
    )?;

    let plls_params = &config.pll_initial_step.plls_parameters;

    // Configure the oscillator, required oscillator mode is external crystal
    if let PllInputClockSelection::F0sc0 | PllInputClockSelection::FSynclk =
        plls_params.pll_input_clock_selection
    {
        // TODO Should be an enum variant in the pac crate
        const MODE_EXTERNALCRYSTAL: u8 = 0;

        let mode = MODE_EXTERNALCRYSTAL;
        let oscval: u8 = ((plls_params.xtal_frequency / 1000000) - 15)
            .try_into()
            .map_err(|_| ())?;

        unsafe {
            SCU.osccon()
                .modify(|r| r.mode().set(scu::osccon::Mode(mode)).oscval().set(oscval))
        };
    }

    // Configure the initial steps for the system PLL
    unsafe {
        SCU.syspllcon0().modify(|r| {
            r.pdiv()
                .set(plls_params.sys_pll.p_divider)
                .ndiv()
                .set(plls_params.sys_pll.n_divider)
                .insel()
                .set(scu::syspllcon0::Insel(
                    plls_params.pll_input_clock_selection as u8,
                ))
        })
    }

    // Configure the initial steps for the peripheral PLL
    unsafe {
        SCU.perpllcon0().modify(|r| {
            r.divby()
                .set(plls_params.per_pll.k3_divider_bypass.into())
                .pdiv()
                .set(plls_params.per_pll.p_divider)
                .ndiv()
                .set(plls_params.per_pll.n_divider)
        })
    }

    set_pll_power(
        scu::syspllcon0::Pllpwd::CONST_11,
        scu::perpllcon0::Pllpwd::CONST_11,
    )?;

    wait_divider()?;

    unsafe {
        SCU.syspllcon1()
            .modify(|r| r.k2div().set(plls_params.sys_pll.k2_divider));
    }

    unsafe {
        SCU.perpllcon1().modify(|r| {
            r.k2div()
                .set(plls_params.per_pll.k2_divider)
                .k3div()
                .set(plls_params.per_pll.k3_divider)
        })
    };

    wait_divider()?;

    // Check if OSC frequencies are in the limit
    wait_cond(OSCCON_PLLLV_OR_HV_TIMEOUT_COUNT, || {
        let osccon = unsafe { SCU.osccon().read() };
        osccon.plllv().get().0 == 0 && osccon.pllhv().get().0 == 0
    })?;

    // Start PLL locking for latest set values
    {
        unsafe { SCU.syspllcon0().modify(|r| r.resld().set(true)) };
        unsafe { SCU.perpllcon0().modify(|r| r.resld().set(true)) };

        wait_cond(PLL_LOCK_TIMEOUT_COUNT, || {
            let sys = unsafe { SCU.syspllstat().read() };
            let per = unsafe { SCU.perpllstat().read() };
            sys.lock().get().0 == 0 || per.lock().get().0 == 0
        })?;
    }

    // enable SMU alarms
    {
        // TODO Explain these magic numbers
        unsafe { SMU.keys().write(RegValue::new(0xBC, 0)) };
        unsafe { SMU.cmd().write(RegValue::new(0x00000005, 0)) };
        unsafe {
            SMU.agi()[8].write(RegValue::new(0x1D, 0));
        }
        unsafe { SMU.keys().write(RegValue::new(0, 0)) };
    }

    {
        let ccucon0 = unsafe { SCU.ccucon0().read() }
            .clksel()
            .set(scu::ccucon0::Clksel::CONST_11)
            .up()
            .set(scu::ccucon0::Up::CONST_11);

        wait_ccucon0_lock()?;

        unsafe { SCU.ccucon0().write(ccucon0) };

        wait_ccucon0_lock()?;
    }

    wdt::set_safety_endinit_inline();

    Ok(())
}

pub(crate) fn modulation_init(config: &Config) -> Result<(), ()> {
    if let ModulationEn::Enabled = config.modulation.enable {
        let rgain_p = calc_rgain_parameters(config.modulation.amp);

        wdt::clear_safety_endinit_inline();

        unsafe {
            SCU.syspllcon2()
                .modify(|r| r.modcfg().set((0x3 << 10) | rgain_p.rgain_hex))
        };

        unsafe {
            SCU.syspllcon0()
                .modify(|r| r.moden().set(scu::syspllcon0::Moden::CONST_11))
        };

        wdt::set_safety_endinit_inline();
    }
    Ok(())
}

pub struct RGainValues {
    pub rgain_nom: f32,
    pub rgain_hex: u16,
}

fn calc_rgain_parameters(modamp: ModulationAmplitude) -> RGainValues {
    const MA_PERCENT: [f32; 6] = [0.5, 1.0, 1.25, 1.5, 2.0, 2.5];

    #[allow(clippy::indexing_slicing)]
    let mod_amp = MA_PERCENT[modamp as usize];

    let fosc_hz = get_osc_frequency();
    let syspllcon0 = unsafe { SCU.syspllcon0().read() };
    let fdco_hz = (fosc_hz * (f32::from(syspllcon0.ndiv().get()) + 1.0))
        / (f32::from(syspllcon0.pdiv().get()) + 1.0);

    let rgain_nom = 2.0 * (mod_amp / 100.0) * (fdco_hz / 3600000.0);
    let rgain_hex = ((rgain_nom * 32.0) + 0.5) as u16;

    RGainValues {
        rgain_nom,
        rgain_hex,
    }
}

pub(crate) fn distribute_clock_inline(config: &Config) -> Result<(), ()> {
    wdt::clear_safety_endinit_inline();

    // CCUCON0 config
    {
        let cuccon0 = unsafe { SCU.ccucon0().read() }
            .stmdiv()
            .set(config.clock_distribution.ccucon0.stm_div)
            .gtmdiv()
            .set(config.clock_distribution.ccucon0.gtm_div)
            .sridiv()
            .set(config.clock_distribution.ccucon0.sri_div)
            // TODO: check this
            .lpdiv()
            .set(config.clock_distribution.ccucon0.lp_div)
            .spbdiv()
            .set(config.clock_distribution.ccucon0.spb_div)
            .bbbdiv()
            .set(config.clock_distribution.ccucon0.bbb_div)
            .fsidiv()
            .set(config.clock_distribution.ccucon0.fsi_div)
            .fsi2div()
            .set(config.clock_distribution.ccucon0.fsi2_div);

        wait_ccucon0_lock()?;

        unsafe { SCU.ccucon0().write(cuccon0) };

        wait_ccucon0_lock()?;
    }
    // CCUCON1 config
    {
        let mut ccucon1 = unsafe { SCU.ccucon1().read() };
        if ccucon1.clkselmcan().get() !=  scu::ccucon1::Clkselmcan::CONST_00 /*ccucon1::Clkselmcan::CLKSELMCAN_STOPPED*/
            || ccucon1.clkselmsc().get() != scu::ccucon1::Clkselmsc::CONST_11 /*ccucon1::Clkselmsc::CLKSELMSC_STOPPED*/
            || ccucon1.clkselqspi().get() != scu::ccucon1::Clkselqspi::CONST_22
        /*ccucon1::Clkselqspi::CLKSELQSPI_STOPPED*/
        {
            ccucon1 = ccucon1
                .mcandiv()
                .set(config.clock_distribution.ccucon1.mcan_div)
                .clkselmcan()
                .set(config.clock_distribution.ccucon1.clksel_mcan)
                .pll1divdis()
                .set(config.clock_distribution.ccucon1.pll1_div_dis)
                .i2cdiv()
                .set(config.clock_distribution.ccucon1.i2c_div)
                .mscdiv()
                .set(config.clock_distribution.ccucon1.msc_div)
                .clkselmsc()
                .set(config.clock_distribution.ccucon1.clksel_msc)
                .qspidiv()
                .set(config.clock_distribution.ccucon1.qspi_div)
                .clkselqspi()
                .set(config.clock_distribution.ccucon1.clksel_qspi);

            ccucon1 = ccucon1
                .clkselmcan()
                .set(
                    scu::ccucon1::Clkselmcan::CONST_00, /*ccucon1::Clkselmcan::CLKSELMCAN_STOPPED*/
                )
                .clkselmsc()
                .set(
                    scu::ccucon1::Clkselmsc::CONST_11, /*ccucon1::Clkselmsc::CLKSELMSC_STOPPED*/
                )
                .clkselqspi()
                .set(
                    scu::ccucon1::Clkselqspi::CONST_22, /*ccucon1::Clkselqspi::CLKSELQSPI_STOPPED*/
                );

            wait_ccucon1_lock()?;
            unsafe { SCU.ccucon1().write(ccucon1) };
            wait_ccucon1_lock()?;
        }

        ccucon1 = unsafe { SCU.ccucon1().read() }
            .mcandiv()
            .set(config.clock_distribution.ccucon1.mcan_div)
            .clkselmcan()
            .set(config.clock_distribution.ccucon1.clksel_mcan)
            .pll1divdis()
            .set(config.clock_distribution.ccucon1.pll1_div_dis)
            .i2cdiv()
            .set(config.clock_distribution.ccucon1.i2c_div)
            .mscdiv()
            .set(config.clock_distribution.ccucon1.msc_div)
            .clkselmsc()
            .set(config.clock_distribution.ccucon1.clksel_msc)
            .qspidiv()
            .set(config.clock_distribution.ccucon1.qspi_div)
            .clkselqspi()
            .set(config.clock_distribution.ccucon1.clksel_qspi);

        wait_ccucon1_lock()?;
        unsafe { SCU.ccucon1().write(ccucon1) };
        wait_ccucon1_lock()?;
    }

    // CCUCON2 config
    {
        let mut ccucon2 = unsafe { SCU.ccucon2().read() };
        if ccucon2.clkselasclins().get() != scu::ccucon2::Clkselasclins::CONST_00
        /*scu::Ccucon2::Clkselasclins::CLKSELASCLINS_STOPPED*/
        {
            ccucon2 = unsafe { SCU.ccucon2().read() }
                .asclinfdiv()
                .set(config.clock_distribution.ccucon2.asclinf_div)
                .asclinsdiv()
                .set(config.clock_distribution.ccucon2.asclins_div)
                .clkselasclins()
                .set(config.clock_distribution.ccucon2.clksel_asclins);

            ccucon2 = ccucon2.clkselasclins().set(
                scu::ccucon2::Clkselasclins::CONST_00, /*scu::ccucon2::Clkselasclins::CLKSELASCLINS_STOPPED*/
            );

            wait_ccucon2_lock()?;

            unsafe { SCU.ccucon2().write(ccucon2) };

            wait_ccucon2_lock()?;
        }

        ccucon2 = unsafe { SCU.ccucon2().read() }
            .asclinfdiv()
            .set(config.clock_distribution.ccucon2.asclinf_div)
            .asclinsdiv()
            .set(config.clock_distribution.ccucon2.asclins_div)
            .clkselasclins()
            .set(config.clock_distribution.ccucon2.clksel_asclins);

        wait_ccucon2_lock()?;
        unsafe { SCU.ccucon2().write(ccucon2) };
        wait_ccucon2_lock()?;
    }

    // CUCCON5 config
    {
        let mut ccucon5 = unsafe { SCU.ccucon5().read() }
            .gethdiv()
            .set(config.clock_distribution.ccucon5.geth_div)
            .mcanhdiv()
            .set(config.clock_distribution.ccucon5.mcanh_div);

        ccucon5 = ccucon5.up().set(scu::ccucon5::Up::CONST_11);

        wait_ccucon5_lock()?;

        unsafe { SCU.ccucon5().write(ccucon5) };

        wait_ccucon5_lock()?;
    }

    // CUCCON6 config
    {
        unsafe {
            SCU.ccucon6()
                .modify(|r| r.cpu0div().set(config.clock_distribution.ccucon6.cpu0_div))
        };
    }

    // CUCCON7 config
    {
        unsafe {
            SCU.ccucon7()
                .modify(|r| r.cpu1div().set(config.clock_distribution.ccucon7.cpu1_div))
        };
    }

    // CUCCON8 config
    {
        unsafe {
            SCU.ccucon8()
                .modify(|r| r.cpu2div().set(config.clock_distribution.ccucon8.cpu2_div))
        };
    }

    wdt::set_safety_endinit_inline();

    Ok(())
}

pub(crate) fn throttle_sys_pll_clock_inline(config: &Config) -> Result<(), ()> {
    for pll_step_count in 0..config.sys_pll_throttle.len() {
        wdt::clear_safety_endinit_inline();

        wait_cond(PLL_KRDY_TIMEOUT_COUNT, || {
            unsafe { SCU.syspllstat().read() }.k2rdy().get().0 != 1
        })?;

        #[allow(clippy::indexing_slicing)]
        let k2div = config.sys_pll_throttle[pll_step_count].k2_step;

        unsafe { SCU.syspllcon1().modify(|r| r.k2div().set(k2div)) };

        wdt::set_safety_endinit_inline();
    }
    Ok(())
}

/// Wait until cond return true or timeout
#[inline]
pub(crate) fn wait_cond(timeout_cycle_count: usize, cond: impl Fn() -> bool) -> Result<(), ()> {
    let mut timeout_cycle_count = timeout_cycle_count;
    while cond() {
        timeout_cycle_count -= 1;
        if timeout_cycle_count == 0 {
            return Err(());
        }
    }

    Ok(())
}

// PLL management
const EVR_OSC_FREQUENCY: u32 = 100_000_000;
const XTAL_FREQUENCY: u32 = 20_000_000;
const SYSCLK_FREQUENCY: u32 = 20_000_000;

#[inline]
pub(crate) fn get_osc_frequency() -> f32 {
    let f = match unsafe { SCU.syspllcon0().read() }.insel().get() {
        scu::syspllcon0::Insel::CONST_00 => EVR_OSC_FREQUENCY,
        scu::syspllcon0::Insel::CONST_11 => XTAL_FREQUENCY,
        scu::syspllcon0::Insel::CONST_22 => SYSCLK_FREQUENCY,
        _ => 0,
    };
    f as f32
}

pub(crate) fn get_pll_frequency() -> u32 {
    let osc_freq = get_osc_frequency();
    let syspllcon0 = unsafe { SCU.syspllcon0().read() };
    let syspllcon1 = unsafe { SCU.syspllcon1().read() };
    let f = (osc_freq * f32::from(syspllcon0.ndiv().get() + 1))
        / f32::from((syspllcon1.k2div().get() + 1) * (syspllcon0.pdiv().get() + 1));
    f as u32
}

pub(crate) fn get_per_pll_frequency1() -> u32 {
    let osc_freq = get_osc_frequency();
    let perpllcon0 = unsafe { SCU.perpllcon0().read() };
    let perpllcon1 = unsafe { SCU.perpllcon1().read() };
    let f = (osc_freq * f32::from(perpllcon0.ndiv().get() + 1))
        / f32::from((perpllcon0.pdiv().get() + 1) * (perpllcon1.k2div().get() + 1));
    f as u32
}

pub(crate) fn get_per_pll_frequency2() -> u32 {
    let osc_freq = get_osc_frequency();
    let perpllcon0 = unsafe { SCU.perpllcon0().read() };
    let perpllcon1 = unsafe { SCU.perpllcon1().read() };

    let multiplier = if perpllcon0.divby().get().0 == 1 {
        2.0
    } else {
        1.6
    };

    let f = (osc_freq * f32::from(perpllcon0.ndiv().get() + 1))
        / (f32::from(perpllcon0.pdiv().get() + 1)
            * f32::from(perpllcon1.k2div().get() + 1)
            * multiplier);
    f as u32
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

pub struct Con0RegConfig {
    pub stm_div: Stmdiv,
    pub gtm_div: Gtmdiv,
    pub sri_div: Sridiv,
    pub lp_div: Lpdiv,
    pub spb_div: Spbdiv,
    pub bbb_div: Bbbdiv,
    pub fsi_div: Fsidiv,
    pub fsi2_div: Fsi2Div,
}

pub struct Con1RegConfig {
    pub mcan_div: Mcandiv,
    pub clksel_mcan: Clkselmcan,
    pub pll1_div_dis: Pll1Divdis,
    pub i2c_div: I2Cdiv,
    pub msc_div: Mscdiv,
    pub clksel_msc: Clkselmsc,
    pub qspi_div: Qspidiv,
    pub clksel_qspi: Clkselqspi,
}

pub struct Con2RegConfig {
    pub asclinf_div: Asclinfdiv,
    pub asclins_div: Asclinsdiv,
    pub clksel_asclins: Clkselasclins,
}

pub struct Con5RegConfig {
    pub geth_div: Gethdiv,
    pub mcanh_div: Mcanhdiv,
    pub adas_div: Mcanhdiv, // TODO: missing adas in pac
}

pub struct Con6RegConfig {
    pub cpu0_div: u8,
}

pub struct Con7RegConfig {
    pub cpu1_div: u8,
}

pub struct Con8RegConfig {
    pub cpu2_div: u8,
}

pub struct ClockDistributionConfig {
    pub ccucon0: Con0RegConfig,
    pub ccucon1: Con1RegConfig,
    pub ccucon2: Con2RegConfig,
    pub ccucon5: Con5RegConfig,
    pub ccucon6: Con6RegConfig,
    pub ccucon7: Con7RegConfig,
    pub ccucon8: Con8RegConfig,
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
        ccucon0: Con0RegConfig {
            stm_div: Stmdiv::CONST_33,
            gtm_div: Gtmdiv::CONST_11,
            sri_div: Sridiv::CONST_11,
            lp_div: Lpdiv::CONST_00,
            spb_div: Spbdiv::CONST_33,
            bbb_div: Bbbdiv::CONST_22,
            fsi_div: Fsidiv::CONST_33,
            fsi2_div: Fsi2Div::CONST_11,
        },
        ccucon1: Con1RegConfig {
            mcan_div: Mcandiv::CONST_22,
            clksel_mcan: Clkselmcan::CONST_11,
            pll1_div_dis: Pll1Divdis::CONST_00,
            i2c_div: I2Cdiv::CONST_22,
            msc_div: Mscdiv::CONST_11,
            clksel_msc: Clkselmsc::CONST_11,
            qspi_div: Qspidiv::CONST_11,
            clksel_qspi: Clkselqspi::CONST_22,
        },
        ccucon2: Con2RegConfig {
            asclinf_div: Asclinfdiv::CONST_11,
            asclins_div: Asclinsdiv::CONST_22,
            clksel_asclins: Clkselasclins::CONST_11,
        },
        ccucon5: Con5RegConfig {
            geth_div: Gethdiv::CONST_22,
            mcanh_div: Mcanhdiv::CONST_33,
            adas_div: Mcanhdiv::CONST_00,
        },
        ccucon6: Con6RegConfig { cpu0_div: 0u8 },
        ccucon7: Con7RegConfig { cpu1_div: 0u8 },
        ccucon8: Con8RegConfig { cpu2_div: 0u8 },
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

pub(crate) fn get_mcan_frequency() -> u32 {
    const CLKSELMCAN_USEMCANI: scu::ccucon1::Clkselmcan = scu::ccucon1::Clkselmcan::CONST_11;
    const CLKSELMCAN_USEOSCILLATOR: scu::ccucon1::Clkselmcan = scu::ccucon1::Clkselmcan::CONST_22;
    const MCANDIV_STOPPED: scu::ccucon1::Mcandiv = scu::ccucon1::Mcandiv::CONST_00;

    // SAFETY: each bit of CCUCON1 is at least R
    let ccucon1 = unsafe { SCU.ccucon1().read() };
    let clkselmcan = ccucon1.clkselmcan().get();
    let mcandiv = ccucon1.mcandiv().get();

    //info!("clkselmcan: {}, mcandiv: {}", clkselmcan, mcandiv);

    match clkselmcan {
        CLKSELMCAN_USEMCANI => {
            let source = get_source_frequency(1);
            debug!("source: {}", source);
            if mcandiv == MCANDIV_STOPPED {
                source
            } else {
                let div: u64 = mcandiv.into();
                let div: u32 = div as u32;
                source / div
            }
        }
        CLKSELMCAN_USEOSCILLATOR => get_osc0_frequency(),
        _ => 0,
    }
}

fn get_source_frequency(source: u32) -> u32 {
    const CLKSEL_BACKUP: scu::ccucon0::Clksel = scu::ccucon0::Clksel::CONST_00;
    const CLKSEL_PLL: scu::ccucon0::Clksel = scu::ccucon0::Clksel::CONST_11;

    // SAFETY: each bit of CCUCON0 is at least R
    let clksel = unsafe { SCU.ccucon0().read() }.clksel().get();
    //info!("clksel: {}", clksel);

    match clksel {
        CLKSEL_BACKUP => get_evr_frequency(),
        CLKSEL_PLL => match source {
            0 => get_pll_frequency(),
            1 => {
                let source_freq = get_per_pll_frequency1();
                // SAFETY: each bit of CCUCON1 is at least R
                let ccucon1 = unsafe { SCU.ccucon1().read() };
                if ccucon1.pll1divdis().get() == scu::ccucon1::Pll1Divdis::CONST_11 {
                    source_freq
                } else {
                    source_freq / 2
                }
            }
            2 => get_per_pll_frequency2(),
            _ => unreachable!(),
        },
        _ => 0,
    }
}

fn get_evr_frequency() -> u32 {
    EVR_OSC_FREQUENCY
}

pub(crate) fn get_osc0_frequency() -> u32 {
    XTAL_FREQUENCY
}
