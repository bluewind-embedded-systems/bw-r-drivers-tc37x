#![allow(clippy::identity_op)]
#![allow(clippy::eq_op)]
#![allow(clippy::result_unit_err)]
#![allow(clippy::float_arithmetic)]
// TODO Remove this once the code is stable
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_sign_loss)]

use super::wdt;
use crate::log::debug;
use crate::pac::{RegisterValue, SCU, SMU};

const SYSPLLSTAT_PWDSTAT_TIMEOUT_COUNT: usize = 0x3000;
const OSCCON_PLLLV_OR_HV_TIMEOUT_COUNT: usize = 0x493E0;
const PLL_LOCK_TIMEOUT_COUNT: usize = 0x3000;

const CCUCON_LCK_BIT_TIMEOUT_COUNT: usize = 0x1000;
const PLL_KRDY_TIMEOUT_COUNT: usize = 0x6000;

pub enum InitError {
    ConfigureCCUInitialStep,
    DistributeClockInline,
    ThrottleSysPllClockInline,
}

pub(crate) fn init(config: &Config) -> Result<(), InitError> {
    configure_ccu_initial_step(config).map_err(|()| InitError::ConfigureCCUInitialStep)?;
    modulation_init(config);
    distribute_clock_inline(config).map_err(|()| InitError::DistributeClockInline)?;
    throttle_sys_pll_clock_inline(config).map_err(|()| InitError::ThrottleSysPllClockInline)?;
    Ok(())
}

fn wait_ccucon0_lock() -> Result<(), ()> {
    wait_cond(CCUCON_LCK_BIT_TIMEOUT_COUNT, || {
        // SAFETY: each bit of CCUCON0 is at least R, except for bit UP (W, if read always return 0)
        unsafe { SCU.ccucon0().read() }.lck().get() == true
    })
}

fn wait_ccucon1_lock() -> Result<(), ()> {
    wait_cond(CCUCON_LCK_BIT_TIMEOUT_COUNT, || {
        // SAFETY: each bit of CCUCON1 is at least R
        unsafe { SCU.ccucon1().read() }.lck().get() == true
    })
}

fn wait_ccucon2_lock() -> Result<(), ()> {
    wait_cond(CCUCON_LCK_BIT_TIMEOUT_COUNT, || {
        // SAFETY: each bit of CCUCON2 is at least R
        unsafe { SCU.ccucon2().read() }.lck().get() == true
    })
}

fn wait_ccucon5_lock() -> Result<(), ()> {
    wait_cond(CCUCON_LCK_BIT_TIMEOUT_COUNT, || {
        // SAFETY: each bit of CCUCON5 is at least R, except for bit UP (W, if read always return 0)
        unsafe { SCU.ccucon5().read() }.lck().get() == true
    })
}

fn wait_divider() -> Result<(), ()> {
    wait_cond(PLL_KRDY_TIMEOUT_COUNT, || {
        // SAFETY: each bit of SYSPLLSTAT is at least R
        let sys = unsafe { SCU.syspllstat().read() };
        // SAFETY: each bit of PERPLLSTAT is at least R
        let per = unsafe { SCU.perpllstat().read() };
        let sys_k2 = sys.k2rdy().get();
        let per_k2 = per.k2rdy().get();
        let per_k3 = per.k3rdy().get();
        sys_k2 == false || per_k2 == false || per_k3 == false
    })
}

fn set_pll_power(syspllpower: bool, perpllpower: bool) -> Result<(), ()> {
    // SAFETY: PLLPWD is a RW bit, syspllpower takes only values in range [0, 1]
    unsafe {
        SCU.syspllcon0()
            .modify(|r| r.pllpwd().set(syspllpower))
    };
    // SAFETY: PLLPWD is a RW bit, syspllpower takes only values in range [0, 1]
    unsafe {
        SCU.perpllcon0()
            .modify(|r| r.pllpwd().set(perpllpower))
    };

    wait_cond(SYSPLLSTAT_PWDSTAT_TIMEOUT_COUNT, || {
        // SAFETY: each bit of SYSPLLSTAT is at least R
        let sys = unsafe { SCU.syspllstat().read() };
        // SAFETY: each bit of PERPLLSTAT is at least R
        let per = unsafe { SCU.perpllstat().read() };
        syspllpower == sys.pwdstat().get() || perpllpower == per.pwdstat().get()
    })
}

pub(crate) fn configure_ccu_initial_step(config: &Config) -> Result<(), ()> {
    // TODO Should be an enum variant in the pac crate
    const CLKSEL_BACKUP: u8 = 0;

    wdt::clear_safety_endinit_inline();

    wait_ccucon0_lock()?;

    // TODO Explain this
    // SAFETY: CLKSEL is RWH and takes values in range [0, 3], UP is a W bit. Written respectively with 0 and 1
    unsafe {
        SCU.ccucon0()
            .modify(|r| r.clksel().set(CLKSEL_BACKUP).up().set(true))
    };
    wait_ccucon0_lock()?;

    // disable SMU
    {
        // The SMU_core configuration is only possible if KEYS.CFGLCK (bits 0:7) is set to 0xBC
        // SAFETY: CFGLCK and PERLCK are RW, bits 16:31 are written with 0
        unsafe { SMU.keys().init(|r| r.cfglck().set(0xBC)) };

        // unsafe { SMU.ag8cfj()[0].modify(|r| r.set_raw(r.get_raw() & !0x1D)) };
        // unsafe { SMU.ag8cfj()[1].modify(|r| r.set_raw(r.get_raw() & !0x1D)) };
        // unsafe { SMU.ag8cfj()[2].modify(|r| r.set_raw(r.get_raw() & !0x1D)) };

        // TODO Check if this is correct, see above the previous version
        // Clear CF0, CF2, CF3 and CF4
        // SAFETY: Each bit of AgiCFj is RW
        unsafe { SMU.agicfj()[8].agicfj_()[0].modify(|r| r.set_raw(r.get_raw() & !0x1D)) };
        // SAFETY: Each bit of AgiCFj is RW
        unsafe { SMU.agicfj()[8].agicfj_()[1].modify(|r| r.set_raw(r.get_raw() & !0x1D)) };
        // SAFETY: Each bit of AgiCFj is RW
        unsafe { SMU.agicfj()[8].agicfj_()[2].modify(|r| r.set_raw(r.get_raw() & !0x1D)) };

        // Disable SMU_core configuration
        // SAFETY: CFGLCK and PERLCK are RW, bits 16:31 are written with 0
        unsafe { SMU.keys().init(|r| r.cfglck().set(0)) };
    }

    // Power down the both the PLLs before configuring registers
    // Both the PLLs are powered down to be sure for asynchronous PLL registers
    // update cause no glitches.
    set_pll_power(false, false)?;

    let plls_params = &config.pll_initial_step.plls_parameters;

    // Configure the oscillator, required oscillator mode is external crystal
    if let PllInputClockSelection::F0sc0 | PllInputClockSelection::FSynclk =
        plls_params.pll_input_clock_selection
    {
        // TODO Should be an enum variant in the pac crate
        const MODE_EXTERNALCRYSTAL: u8 = 0;

        let mode = MODE_EXTERNALCRYSTAL;
        // TODO: xtal_frequency should be in range [16 MHz, 40 MHz]
        let oscval: u8 = ((plls_params.xtal_frequency / 1_000_000) - 15)
            .try_into()
            .map_err(|_| ())?;

        // SAFETY: MODE is RW and takes values in range [0, 3], OSCVAL is RW and takes values in range [1, 25] (TODO: is it right?)
        // MODE is written with 0
        unsafe {
            SCU.osccon()
                .modify(|r| r.mode().set(mode).oscval().set(oscval))
        };
    }

    // Configure the initial steps for the system PLL
    // SAFETY: PDIV is RW and takes values in range [0, 7], TODO: p_divider should be in range [0, 7]
    // NDIV is RW and takes values in range [0, 127], TODO: n_divider should be in range [0, 127]
    // INSEL is RW and takes values in range [0, 2], this is guaranteed by pll_input_clock_selection's type PllInputClockSelection
    unsafe {
        SCU.syspllcon0().modify(|r| {
            r.pdiv()
                .set(plls_params.sys_pll.p_divider)
                .ndiv()
                .set(plls_params.sys_pll.n_divider)
                .insel()
                .set(plls_params.pll_input_clock_selection as u8)
        })
    }

    // Configure the initial steps for the peripheral PLL
    // SAFETY: DIVBY is a RW bit, TODO: k3_divider_bypass should be in range [0, 1]
    // PDIV is RW and takes values in range [0, 7], TODO: p_divider should be in range [0, 7]
    // NDIV is RW and takes values in range [0, 127], TODO: n_divider should be in range [0, 127]
    unsafe {
        SCU.perpllcon0().modify(|r| {
            r.divby()
                .set(plls_params.per_pll.k3_divider_bypass)
                .pdiv()
                .set(plls_params.per_pll.p_divider)
                .ndiv()
                .set(plls_params.per_pll.n_divider)
        })
    }

    set_pll_power(true, true)?;

    wait_divider()?;

    // SAFETY: K2DIV is RW and takes values in range [0, 7], TODO: k2_divider should be in range [0, 7]
    // K2DIV is locked while SYSPLLSTAT.K2RDY is equal to 0, wait is performed by wait_divider
    unsafe {
        SCU.syspllcon1()
            .modify(|r| r.k2div().set(plls_params.sys_pll.k2_divider));
    }

    // SAFETY: K2DIV is RW and takes values in range [0, 7], TODO: k2_divider should be in range [0, 7]
    // K3DIV is RW and takes values in range [0, 7], TODO: k3_divider should be in range [0, 7]
    // K2DIV and K3DIV are respectively locked while PERPLLSTAT.K2RDY and PERPLLSTAT.K3RDY are equal to 0, wait is performed by wait_divider
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
        // SAFETY: each bit of OSCCON is at least R, except for bit OSCRES (W, if read always return 0)
        let osccon = unsafe { SCU.osccon().read() };
        osccon.plllv().get() == false && osccon.pllhv().get() == false
    })?;

    // Start PLL locking for latest set values
    {
        // SAFETY: RESLD is a W bit
        unsafe { SCU.syspllcon0().modify(|r| r.resld().set(true)) };
        // SAFETY: RESLD is a W bit
        unsafe { SCU.perpllcon0().modify(|r| r.resld().set(true)) };

        wait_cond(PLL_LOCK_TIMEOUT_COUNT, || {
            // SAFETY: each bit of SYSPLLSTAT is at least R
            let sys = unsafe { SCU.syspllstat().read() };
            // SAFETY: each bit of PERPLLSTAT is at least R
            let per = unsafe { SCU.perpllstat().read() };
            sys.lock().get() == false || per.lock().get() == false
        })?;
    }

    // enable SMU alarms
    {
        // The SMU_core configuration is only possible if KEYS.CFGLCK (bits 0:7) is set to 0xBC
        // SAFETY: CFGLCK and PERLCK are RW, bits 16:31 are written with 0
        unsafe { SMU.keys().write(RegisterValue::new(0xBC)) };
        // SMU Alarm Status Clear Enable command (SMU_ASCE(ARG), ARG shall be set to 0)
        // SAFETY: CMD and ARG are W, bits 8:31 are written with 0
        unsafe { SMU.cmd().write(RegisterValue::new(0x0000_0005)) };
        // Set SF0, SF2, SF3 and SF4
        // SAFETY: Each bit of AGi is RWh
        unsafe {
            SMU.agi()[8].write(RegisterValue::new(0x1D));
        }
        // Disable SMU_core configuration
        // SAFETY: CFGLCK and PERLCK are RW, bits 16:31 are written with 0
        unsafe { SMU.keys().write(RegisterValue::new(0)) };
    }

    {
        // SAFETY: each bit of CCUCON0 is at least R, UP is a W bit and CLKSEL is RWH and takes values in range [0, 1]
        let ccucon0 = unsafe { SCU.ccucon0().read() }
            .clksel()
            .set(1)
            .up()
            .set(true);

        wait_ccucon0_lock()?;

        // SAFETY: each bit of CCUCON0 is W except for LCK (RH), TODO: what in this case?
        // updates of CCUCON0 are locked while LCK == 1, wait is performed by wait_ccucon0_lock
        unsafe { SCU.ccucon0().write(ccucon0) };

        wait_ccucon0_lock()?;
    }

    wdt::set_safety_endinit_inline();

    Ok(())
}

pub(crate) fn modulation_init(config: &Config) {
    if let ModulationEn::Enabled = config.modulation.enable {
        let rgain_p = calc_rgain_parameters(config.modulation.amp);

        wdt::clear_safety_endinit_inline();

        // SAFETY: Bits MODCFG[9:5] are treated as integer part and bits MODCFG[4:0] as fractional part.
        // Bits MODCFG[15:10] have to be configured with the following setting: 0x111101B.
        // TODO: check value of rgain_hex
        unsafe {
            SCU.syspllcon2()
                .modify(|r| r.modcfg().set((0x3D << 10) | rgain_p.rgain_hex))
        };

        // SAFETY: MODEN is a RW bit
        unsafe {
            SCU.syspllcon0()
                .modify(|r| r.moden().set(true))
        };

        wdt::set_safety_endinit_inline();
    }
}

pub struct RGainValues {
    pub rgain_nom: f32,
    pub rgain_hex: u16,
}

fn calc_rgain_parameters(mod_amp: ModulationAmplitude) -> RGainValues {
    const MA_PERCENT: [f32; 6] = [0.5, 1.0, 1.25, 1.5, 2.0, 2.5];

    #[allow(clippy::indexing_slicing)]
    let mod_amp = MA_PERCENT[mod_amp as usize];

    let fosc_hz = get_osc_frequency();
    // SAFETY: each bit of SYSPLLCON0 is at least R, except for bit RESLD (W, if read always return 0)
    let syspllcon0 = unsafe { SCU.syspllcon0().read() };
    let fdco_hz = (fosc_hz * (f32::from(syspllcon0.ndiv().get()) + 1.0))
        / (f32::from(syspllcon0.pdiv().get()) + 1.0);

    let rgain_nom = 2.0 * (mod_amp / 100.0) * (fdco_hz / 3_600_000.0);
    let rgain_hex = ((rgain_nom * 32.0) + 0.5) as u16;

    RGainValues {
        rgain_nom,
        rgain_hex,
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn distribute_clock_inline(config: &Config) -> Result<(), ()> {
    wdt::clear_safety_endinit_inline();

    // CCUCON0 config
    {
        // SAFETY: each bit of CCUCON0 is at least R, except for bit UP (W, if read always return 0)
        // Each bitfield in the following instructions is RW, and takes specific value ranges that are guaranteed by the types used
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

        // SAFETY: each bit of CCUCON0 is W except for LCK (RH), TODO: what in this case?
        // updates of CCUCON0 are locked while LCK == 1, wait is performed by wait_ccucon0_lock
        unsafe { SCU.ccucon0().write(cuccon0) };

        wait_ccucon0_lock()?;
    }

    // CCUCON1 config
    {
        // SAFETY: each bit of CCUCON1 is at least R
        let mut ccucon1 = unsafe { SCU.ccucon1().read() };
        if ccucon1.clkselmcan().get() != 0
            || ccucon1.clkselmsc().get() != 1
            || ccucon1.clkselqspi().get() != 2
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
                .set(0)
                .clkselmsc()
                .set(1)
                .clkselqspi()
                .set(2);

            wait_ccucon1_lock()?;

            // SAFETY: each bit of CCUCON1 is at least W, except for bit LCK (RH) TODO: what in this case?
            // Each bitfield set above is RW, and takes specific value ranges that are guaranteed by the types used
            // updates of CCUCON1 are locked while LCK == 1, wait is performed by wait_ccucon1_lock
            unsafe { SCU.ccucon1().write(ccucon1) };

            wait_ccucon1_lock()?;
        }

        // SAFETY: each bit of CCUCON1 is at least R
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

        // SAFETY: each bit of CCUCON1 is at least W, except for bit LCK (RH) TODO: what in this case?
        // Each bitfield set above is RW, and takes specific value ranges that are guaranteed by the types used
        // updates of CCUCON1 are locked while LCK == 1, wait is performed by wait_ccucon1_lock
        unsafe { SCU.ccucon1().write(ccucon1) };

        wait_ccucon1_lock()?;
    }

    // CCUCON2 config
    {
        // SAFETY: each bit of CCUCON2 is at least R
        let mut ccucon2 = unsafe { SCU.ccucon2().read() };

        if ccucon2.clkselasclins().get() != 0 {
            // TODO Why is this read again?
            // SAFETY: each bit of CCUCON2 is at least R
            ccucon2 = unsafe { SCU.ccucon2().read() }
                .asclinfdiv()
                .set(config.clock_distribution.ccucon2.asclinf_div)
                .asclinsdiv()
                .set(config.clock_distribution.ccucon2.asclins_div)
                .clkselasclins()
                .set(config.clock_distribution.ccucon2.clksel_asclins);

            ccucon2 = ccucon2
                .clkselasclins()
                .set(0);

            wait_ccucon2_lock()?;

            // SAFETY: each bit of CCUCON2 is at least W, except for bit LCK (RH) TODO: what in this case?
            // Each bitfield set above is RW, and takes specific value ranges that are guaranteed by the types used
            // updates of CCUCON2 are locked while LCK == 1, wait is performed by wait_ccucon2_lock
            unsafe { SCU.ccucon2().write(ccucon2) };

            wait_ccucon2_lock()?;
        }

        // SAFETY: each bit of CCUCON2 is at least R
        ccucon2 = unsafe { SCU.ccucon2().read() }
            .asclinfdiv()
            .set(config.clock_distribution.ccucon2.asclinf_div)
            .asclinsdiv()
            .set(config.clock_distribution.ccucon2.asclins_div)
            .clkselasclins()
            .set(config.clock_distribution.ccucon2.clksel_asclins);

        wait_ccucon2_lock()?;

        // SAFETY: each bit of CCUCON2 is at least W, except for bit LCK (RH) TODO: what in this case?
        // Each bitfield set above is RW, and takes specific value ranges that are guaranteed by the types used
        // updates of CCUCON2 are locked while LCK == 1, wait is performed by wait_ccucon2_lock
        unsafe { SCU.ccucon2().write(ccucon2) };

        wait_ccucon2_lock()?;
    }

    // CUCCON5 config
    {
        // SAFETY: each bit of CCUCON5 is at least R, except for bit UP (W, if read always return 0)
        let mut ccucon5 = unsafe { SCU.ccucon5().read() }
            .gethdiv()
            .set(config.clock_distribution.ccucon5.geth_div)
            .mcanhdiv()
            .set(config.clock_distribution.ccucon5.mcanh_div);

        ccucon5 = ccucon5.up().set(true);

        wait_ccucon5_lock()?;

        // SAFETY: each bit of CCUCON5 is at least W, except for bit LCK (RH) TODO: what in this case?
        // Each bitfield set above is RW, and takes specific value ranges that are guaranteed by the types used
        // updates of CCUCON5 are locked while LCK == 1, wait is performed by wait_ccucon5_lock
        unsafe { SCU.ccucon5().write(ccucon5) };

        wait_ccucon5_lock()?;
    }

    // CUCCON6 config
    {
        // SAFETY: CPU0DIV is RW, bits [6:31] are written with 0
        // TODO: cpu0_div should be in range [0, 2^6)
        unsafe {
            SCU.ccucon6()
                .modify(|r| r.cpu0div().set(config.clock_distribution.ccucon6.cpu0_div))
        };
    }

    // CUCCON7 config
    {
        // SAFETY: CPU1DIV is RW, bits [6:31] are written with 0
        // TODO: cpu1_div should be in range [0, 2^6)
        unsafe {
            SCU.ccucon7()
                .modify(|r| r.cpu1div().set(config.clock_distribution.ccucon7.cpu1_div))
        };
    }

    // CUCCON8 config
    {
        // SAFETY: CPU2DIV is RW, bits [6:31] are written with 0
        // TODO: cpu2_div should be in range [0, 2^6)
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
            // SAFETY: each bit of SYSPLLSTAT is at least R
            unsafe { SCU.syspllstat().read() }.k2rdy().get() != true
        })?;

        #[allow(clippy::indexing_slicing)]
        let k2div = config.sys_pll_throttle[pll_step_count].k2_step;

        // SAFETY: K2DIV is RW and takes values in range [0, 7], TODO: k2div should be in range [0, 7]
        // K2DIV is locked while SYSPLLSTAT.K2RDY is equal to 0, wait is performed by wait_cond
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
    // SAFETY: each bit of SYSPLLCON0 is at least R, except for bit RESLD (W, if read always return 0)
    let f = match unsafe { SCU.syspllcon0().read() }.insel().get() {
        0 => EVR_OSC_FREQUENCY,
        1 => XTAL_FREQUENCY,
        2 => SYSCLK_FREQUENCY,
        _ => 0,
    };
    f as f32
}

pub(crate) fn get_pll_frequency() -> u32 {
    let osc_freq = get_osc_frequency();
    // SAFETY: each bit of SYSPLLCON0 is at least R, except for bit RESLD (W, if read always return 0)
    let syspllcon0 = unsafe { SCU.syspllcon0().read() };
    // SAFETY: each bit of SYSPLLCON1 is at least R
    let syspllcon1 = unsafe { SCU.syspllcon1().read() };
    let f = (osc_freq * f32::from(syspllcon0.ndiv().get() + 1))
        / f32::from((syspllcon1.k2div().get() + 1) * (syspllcon0.pdiv().get() + 1));
    f as u32
}

pub(crate) fn get_per_pll_frequency1() -> u32 {
    let osc_freq = get_osc_frequency();
    // SAFETY: each bit of PERPLLCON0 is at least R, except for bit RESLD (W, if read always return 0)
    let perpllcon0 = unsafe { SCU.perpllcon0().read() };
    // SAFETY: each bit of PERPLLCON1 is at least R
    let perpllcon1 = unsafe { SCU.perpllcon1().read() };
    let f = (osc_freq * f32::from(perpllcon0.ndiv().get() + 1))
        / f32::from((perpllcon0.pdiv().get() + 1) * (perpllcon1.k2div().get() + 1));
    f as u32
}

pub(crate) fn get_per_pll_frequency2() -> u32 {
    let osc_freq = get_osc_frequency();
    // SAFETY: each bit of PERPLLCON0 is at least R, except for bit RESLD (W, if read always return 0)
    let perpllcon0 = unsafe { SCU.perpllcon0().read() };
    // SAFETY: each bit of PERPLLCON1 is at least R
    let perpllcon1 = unsafe { SCU.perpllcon1().read() };

    let multiplier = if perpllcon0.divby().get() == true {
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
    pub k3_divider_bypass: bool,
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
    pub stm_div: u8,
    pub gtm_div: u8,
    pub sri_div: u8,
    pub lp_div: u8,
    pub spb_div: u8,
    pub bbb_div: u8,
    pub fsi_div: u8,
    pub fsi2_div: u8,
}

pub struct Con1RegConfig {
    pub mcan_div: u8,
    pub clksel_mcan: u8,
    pub pll1_div_dis: bool,
    pub i2c_div: u8,
    pub msc_div: u8,
    pub clksel_msc: u8,
    pub qspi_div: u8,
    pub clksel_qspi: u8,
}

pub struct Con2RegConfig {
    pub asclinf_div: u8,
    pub asclins_div: u8,
    pub clksel_asclins: u8,
}

pub struct Con5RegConfig {
    pub geth_div: u8,
    pub mcanh_div: u8,
    pub adas_div: u8, // TODO: missing adas in pac
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
        wait_time: 0.000_100,
    },
    PllStepConfig {
        k2_step: 3 - 1,
        wait_time: 0.000_100,
    },
    PllStepConfig {
        k2_step: 2 - 1,
        wait_time: 0.000_100,
    },
];

pub const DEFAULT_CLOCK_CONFIG: Config = Config {
    pll_initial_step: InitialConfigStep {
        plls_parameters: PllsParameterConfig {
            xtal_frequency: 20_000_000,
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
                k3_divider_bypass: false,
            },
        },
        wait_time: 0.000_200,
    },
    sys_pll_throttle: &DEFAULT_PLL_CONFIG_STEPS,
    clock_distribution: ClockDistributionConfig {
        ccucon0: Con0RegConfig {
            stm_div: 3,
            gtm_div: 1,
            sri_div: 1,
            lp_div: 0,
            spb_div: 3,
            bbb_div: 2,
            fsi_div: 3,
            fsi2_div: 1,
        },
        ccucon1: Con1RegConfig {
            mcan_div: 2,
            clksel_mcan: 1,
            pll1_div_dis: false,
            i2c_div: 2,
            msc_div: 1,
            clksel_msc: 1,
            qspi_div: 1,
            clksel_qspi: 2,
        },
        ccucon2: Con2RegConfig {
            asclinf_div: 1,
            asclins_div: 2,
            clksel_asclins: 1,
        },
        ccucon5: Con5RegConfig {
            geth_div: 2,
            mcanh_div: 3,
            adas_div: 0,
        },
        ccucon6: Con6RegConfig { cpu0_div: 0u8 },
        ccucon7: Con7RegConfig { cpu1_div: 0u8 },
        ccucon8: Con8RegConfig { cpu2_div: 0u8 },
    },
    flash_wait_state: FlashWaitStateConfig {
        value: 0x0000_0105,
        mask: 0x0000_073F,
    },
    modulation: ModulationConfig {
        enable: ModulationEn::Disabled,
        amp: ModulationAmplitude::_0p5,
    },
};

pub(crate) fn get_mcan_frequency() -> u32 {
    //TODO create enum!
    const CLKSELMCAN_USEMCANI: u8 = 1;
    const CLKSELMCAN_USEOSCILLATOR: u8 = 2;
    const MCANDIV_STOPPED: u8 = 0;

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
    const CLKSEL_BACKUP: u8 = 0; // TODO create enum
    const CLKSEL_PLL: u8 = 1;

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
                if ccucon1.pll1divdis().get() == true {
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
