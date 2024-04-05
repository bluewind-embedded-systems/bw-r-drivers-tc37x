#![allow(clippy::cast_possible_truncation)]

use crate::intrinsics::read_cpu_core_id;
use crate::pac;

// If PAS=0: WDTxCON0.PW[7:2] must be written with inverted current value read from WDTxCON0.PW[7:2]
fn correct_password(password: u16) -> u16 {
    password ^ 0x003F
}

#[inline]
fn get_wdt_con0(core_id: u8) -> pac::Reg<pac::scu::wdtcpu::WdtcpUyCon0_SPEC, pac::RW> {
    pac::SCU.wdtcpu()[usize::from(core_id)].wdtcpuycon0()
}

#[inline]
fn get_wdt_con1(core_id: u8) -> pac::Reg<pac::scu::wdtcpu::WdtcpUyCon1_SPEC, pac::RW> {
    pac::SCU.wdtcpu()[usize::from(core_id)].wdtcpuycon1()
}

#[inline]
fn get_wdts_con0() -> pac::Reg<pac::scu::wdts::Wdtscon0_SPEC, pac::RW> {
    pac::SCU.wdts().wdtscon0()
}

#[inline]
fn get_wdts_con1() -> pac::Reg<pac::scu::wdts::Wdtscon1_SPEC, pac::RW> {
    pac::SCU.wdts().wdtscon1()
}

// TODO Duplicate? Bad function name?
#[inline]
pub(crate) fn clear_cpu_endinit_inline() {
    let core_id = read_cpu_core_id();
    let con0 = get_wdt_con0(core_id as u8);

    // SAFETY: core_id is read through assembly instruction MFCR
    let mut wdtcon0 = unsafe { con0.read() };
    // If PAS=0: WDTxCON0.PW[7:2] must be written with inverted current value read from WDTxCON0.PW[7:2]
    let password = correct_password(wdtcon0.pw().get());

    let rel = wdtcon0.rel().get();

    if wdtcon0.lck().get() {
        wdtcon0 = wdtcon0
            .endinit()
            .set(true)
            .lck()
            .set(false)
            .pw()
            .set(password)
            .rel()
            .set(rel);
        // SAFETY: Each bit of WDTCPUyCON0 is at least W
        unsafe { con0.write(wdtcon0) };
    }

    wdtcon0 = wdtcon0
        .endinit()
        .set(false)
        .lck()
        .set(true)
        .pw()
        .set(password)
        .rel()
        .set(rel);
    // SAFETY: Each bit of WDTCPUyCON0 is at least W
    unsafe { con0.write(wdtcon0) };

    // TODO This conditional compilation can be removed, now that tracing can set the register value
    #[cfg(tricore_arch = "tricore")]
    while unsafe { con0.read() }.endinit().get() {}
}

#[inline]
pub(crate) fn set_cpu_endinit_inline() {
    let core_id = read_cpu_core_id();
    let con0 = get_wdt_con0(core_id as u8);
    // SAFETY: core_id is read through assembly instruction MFCR
    let mut wdtcon0 = unsafe { con0.read() };

    let password = correct_password(wdtcon0.pw().get());

    let rel = wdtcon0.rel().get();

    if wdtcon0.lck().get() {
        wdtcon0 = wdtcon0
            .endinit()
            .set(true)
            .lck()
            .set(false)
            .pw()
            .set(password)
            .rel()
            .set(rel);
        // SAFETY: Each bit of WDTCPUyCON0 is at least W
        unsafe { con0.write(wdtcon0) };
    }

    wdtcon0 = wdtcon0
        .endinit()
        .set(true)
        .lck()
        .set(true)
        .pw()
        .set(password)
        .rel()
        .set(rel);

    // SAFETY: Each bit of WDTCPUyCON0 is at least W
    unsafe { con0.write(wdtcon0) };

    // TODO This conditional compilation can be removed, now that tracing can set the register value
    #[cfg(tricore_arch = "tricore")]
    while !unsafe { con0.read() }.endinit().get() {}
}

#[inline]
pub(crate) fn clear_safety_endinit_inline() {
    let con0 = get_wdts_con0();

    let mut wdtcon0 = unsafe { con0.read() };
    let password = correct_password(wdtcon0.pw().get());

    let rel = wdtcon0.rel().get();

    // SAFETY: LCK is a RWH bit
    if wdtcon0.lck().get() {
        wdtcon0 = wdtcon0
            .endinit()
            .set(true)
            .lck()
            .set(false)
            .pw()
            .set(password)
            .rel()
            .set(rel);
        unsafe { con0.write(wdtcon0) };
    }

    wdtcon0 = wdtcon0
        .endinit()
        .set(false)
        .lck()
        .set(true)
        .pw()
        .set(password)
        .rel()
        .set(rel);

    // SAFETY: Each bit of WDTSCON0 is RW
    unsafe { con0.write(wdtcon0) };

    #[cfg(tricore_arch = "tricore")]
    while unsafe { con0.read() }.endinit().get() {}
}

#[inline]
pub(crate) fn set_safety_endinit_inline() {
    let con0 = get_wdts_con0();

    let mut wdtcon0 = unsafe { con0.read() };
    let password = correct_password(wdtcon0.pw().get());

    let rel = wdtcon0.rel().get();

    if wdtcon0.lck().get() {
        wdtcon0 = wdtcon0
            .endinit()
            .set(true)
            .lck()
            .set(false)
            .pw()
            .set(password)
            .rel()
            .set(rel);
        unsafe { con0.write(wdtcon0) };
    }

    wdtcon0 = wdtcon0
        .endinit()
        .set(true)
        .lck()
        .set(true)
        .pw()
        .set(password)
        .rel()
        .set(rel);
    unsafe { con0.write(wdtcon0) };

    #[cfg(tricore_arch = "tricore")]
    while !unsafe { con0.read() }.endinit().get() {}
}

pub fn disable_safety_watchdog() {
    clear_safety_endinit_inline();

    let con1 = get_wdts_con1();
    // SAFETY: DR is a RW bit, it can be modified only when safety endinit is de-asserted (clear_safety_endinit_inline)
    unsafe { con1.modify(|p| p.dr().set(true)) };

    set_safety_endinit_inline();
}

pub fn disable_cpu_watchdog() {
    clear_cpu_endinit_inline();

    let core_id = read_cpu_core_id();
    // SAFETY: core_id is read through assembly instruction MFCR
    let con1 = get_wdt_con1(core_id as u8);
    // SAFETY: DR is a RW bit, it can be modified only when safety endinit is de-asserted (clear_cpu_endinit_inline)
    unsafe { con1.modify(|p| p.dr().set(true)) };

    set_cpu_endinit_inline();
}

#[cfg(feature = "tracing")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::log::Report;

    #[test]
    fn test_clear_cpu_endinit_inline() {
        let report = Report::new();
        let con0 = get_wdt_con0(0);
        report.expect_read(con0.ptr() as usize, 4, 0b11);
        clear_cpu_endinit_inline();
        insta::assert_snapshot!(report.take_log());
    }

    #[test]
    fn test_set_cpu_endinit_inline() {
        let report = Report::new();
        let con0 = get_wdt_con0(0);
        report.expect_read(con0.ptr() as usize, 4, 0b11);
        set_cpu_endinit_inline();
        insta::assert_snapshot!(report.take_log());
    }

    #[test]
    fn test_clear_then_set_cpu_endinit_inline() {
        let report = Report::new();
        let con0 = get_wdt_con0(0);
        report.expect_read(con0.ptr() as usize, 4, 0b11);
        clear_cpu_endinit_inline();
        report.expect_read(con0.ptr() as usize, 4, 0b10);
        set_cpu_endinit_inline();
        insta::assert_snapshot!(report.take_log());
    }

    #[test]
    fn test_set_then_clear_cpu_endinit_inline() {
        let report = Report::new();
        let con0 = get_wdt_con0(0);
        report.expect_read(con0.ptr() as usize, 4, 0b11);
        set_cpu_endinit_inline();
        report.expect_read(con0.ptr() as usize, 4, 0b11);
        clear_cpu_endinit_inline();
        insta::assert_snapshot!(report.take_log());
    }

    #[test]
    fn test_clear_safety_endinit_inline() {
        let report = Report::new();
        let con0 = get_wdts_con0();
        report.expect_read(con0.ptr() as usize, 4, 0b11);
        clear_safety_endinit_inline();
        insta::assert_snapshot!(report.take_log());
    }

    #[test]
    fn test_set_safety_endinit_inline() {
        let report = Report::new();
        let con0 = get_wdts_con0();
        report.expect_read(con0.ptr() as usize, 4, 0b11);
        set_safety_endinit_inline();
        insta::assert_snapshot!(report.take_log());
    }

    #[test]
    fn test_clear_then_set_safety_endinit_inline() {
        let report = Report::new();
        let con0 = get_wdts_con0();
        report.expect_read(con0.ptr() as usize, 4, 0b11);
        clear_safety_endinit_inline();
        report.expect_read(con0.ptr() as usize, 4, 0b10);
        set_safety_endinit_inline();
        insta::assert_snapshot!(report.take_log());
    }

    #[test]
    fn test_set_then_clear_safety_endinit_inline() {
        let report = Report::new();
        let con0 = get_wdts_con0();
        report.expect_read(con0.ptr() as usize, 4, 0b11);
        set_safety_endinit_inline();
        report.expect_read(con0.ptr() as usize, 4, 0b11);
        clear_safety_endinit_inline();
        insta::assert_snapshot!(report.take_log());
    }

    #[test]
    fn test_disable_cpu_watchdog() {
        let report = Report::new();
        let con0 = get_wdt_con0(0);
        let con1 = get_wdt_con1(0);

        report.expect_read(con0.ptr() as usize, 4, 0b11);
        report.expect_read(con1.ptr() as usize, 4, 0b00);
        report.expect_read(con0.ptr() as usize, 4, 0b10);

        disable_cpu_watchdog();
        insta::assert_snapshot!(report.take_log());
    }

    #[test]
    fn test_disable_safety_watchdog() {
        let report = Report::new();
        let con0 = get_wdts_con0();
        let con1 = get_wdts_con1();

        report.expect_read(con0.ptr() as usize, 4, 0b11);
        report.expect_read(con1.ptr() as usize, 4, 0b00);
        report.expect_read(con0.ptr() as usize, 4, 0b10);

        disable_safety_watchdog();
        insta::assert_snapshot!(report.take_log());
    }
}
