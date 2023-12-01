use super::wdt; 
use tc37x_pac::SCU;
use tc37x_pac::scu;


const SYSPLLSTAT_PWDSTAT_TIMEOUT_COUNT: usize = 0x3000;
const OSCCON_PLLLV_OR_HV_TIMEOUT_COUNT: usize = 0x493E0;
const PLL_LOCK_TIMEOUT_COUNT: usize = 0x3000;

const CCUCON_LCK_BIT_TIMEOUT_COUNT: usize = 0x1000;
const PLL_KRDY_TIMEOUT_COUNT: usize = 0x6000;


pub fn init() -> Result<(), ()> {
    let config = DEFAULT_CLOCK_CONFIG; 
    configure_ccu_initial_step(&config)?;

    modulation_init(&config);

    distribute_clock_inline(&config)?;

    throttle_sys_pll_clock_inline(&config)?;

    Ok(())
}

pub fn configure_ccu_initial_step(config: &Config) -> Result<(), ()> 
{
    let endinit_sfty_pw = wdt::get_safety_watchdog_password();
    wdt::clear_safety_endinit_inline(endinit_sfty_pw);
   
    wait_cond::<CCUCON_LCK_BIT_TIMEOUT_COUNT>(|| {
        unsafe { SCU.ccucon0().read() }.lck().get()
    })?;

    Ok(())
}

pub fn modulation_init(config: &Config) -> Result<(), ()> 
{
    Ok(())
}

pub fn distribute_clock_inline(config: &Config) -> Result<(), ()> 
{
    Ok(())
}

pub fn throttle_sys_pll_clock_inline(config: &Config) -> Result<(), ()> 
{
    Ok(())
}

#[inline]
pub fn wait_cond<const C: usize>(cond: impl Fn() -> bool) -> Result<(),()> {
    let mut timeout_cycle_count = C;
    while cond() {
        timeout_cycle_count -= 1;
        if timeout_cycle_count <= 0 {
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
