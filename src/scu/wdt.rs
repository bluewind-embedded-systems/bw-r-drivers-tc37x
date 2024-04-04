#![allow(clippy::cast_possible_truncation)]

use crate::intrinsics::read_cpu_core_id;
use crate::pac;
use core::mem::transmute;

// TODO Are we sure we want to publish this function?
#[inline]
pub(crate) fn get_cpu_watchdog_password() -> u16 {
    let core_id = read_cpu_core_id();
    let password = match core_id {
        // SAFETY: Each bit of WDTCPU0CON0 is at least R
        0 => unsafe { pac::SCU.wdtcpu()[0].wdtcpuycon0().read() }
            .pw()
            .get(),
        // SAFETY: Each bit of WDTCPU1CON0 is at least R
        1 => unsafe { pac::SCU.wdtcpu()[1].wdtcpuycon0().read() }
            .pw()
            .get(),
        // SAFETY: Each bit of WDTCPU2CON0 is at least R
        2 => unsafe { pac::SCU.wdtcpu()[2].wdtcpuycon0().read() }
            .pw()
            .get(),
        _ => unreachable!(),
    };

    // If PAS=0: WDTxCON0.PW[7:2] must be written with inverted current value read from WDTxCON0.PW[7:2]
    password ^ 0x003F
}

// TODO Are we sure we want to publish this function?
#[inline]
pub(crate) fn get_safety_watchdog_password() -> u16 {
    // SAFETY: Each bit of WDTSCON0 is at least R
    let password = unsafe { pac::SCU.wdts().wdtscon0().read() }.pw().get();

    // If PAS=0: WDTxCON0.PW[7:2] must be written with inverted current value read from WDTxCON0.PW[7:2]
    password ^ 0x003F
}

#[inline]
unsafe fn get_wdt_con0(core_id: u8) -> pac::Reg<pac::scu::wdtcpu::WdtcpUyCon0_SPEC, pac::RW> {
    // SAFETY: The following transmute is safe, getting WDTCPU0CON0 base address
    let off: *mut u8 = unsafe { transmute(pac::SCU.wdtcpu()[0].wdtcpuycon0()) };
    // SAFETY: The following operation is safe, TODO: core_id should be less than available cores
    let off = unsafe { off.add(core::mem::size_of::<u32>() * 3 * core_id as usize) };
    // SAFETY: The following transmute is safe since WDTCPUyCON0 have the same layout
    unsafe { transmute(off) }
}

#[inline]
unsafe fn get_wdt_con1(core_id: u8) -> pac::Reg<pac::scu::wdtcpu::WdtcpUyCon1_SPEC, pac::RW> {
    // SAFETY: The following transmute is safe, getting WDTCPU0CON1 base address
    let off: *mut u8 = unsafe { transmute(pac::SCU.wdtcpu()[0].wdtcpuycon1()) };
    // SAFETY: The following operation is safe, TODO: core_id should be less than available cores
    let off = unsafe { off.add(core::mem::size_of::<u32>() * 3 * core_id as usize) };
    // SAFETY: The following transmute is safe since WDTCPUyCON1 have the same layout
    unsafe { transmute(off) }
}

// TODO Duplicate? Bad function name?
#[inline]
pub(crate) fn clear_cpu_endinit_inline() {
    let password = get_cpu_watchdog_password();
    let core_id = read_cpu_core_id();
    // SAFETY: core_id is read through assembly instruction MFCR
    let con0 = unsafe { get_wdt_con0(core_id as u8) };

    // FIXME con0 is read twice
    // SAFETY: LCK is a RWH bit
    if unsafe { con0.read() }.lck().get() {
        // SAFETY: REL is a RW bitfield
        let rel = unsafe { con0.read() }.rel().get();
        let data = pac::scu::wdtcpu::WdtcpUyCon0::default()
            .endinit()
            .set(true)
            .lck()
            .set(false)
            .pw()
            .set(password)
            .rel()
            .set(rel);
        // SAFETY: Each bit of WDTCPUyCON0 is at least W
        unsafe { con0.write(data) };
    }

    // SAFETY: REL is a RW bitfield
    let rel = unsafe { con0.read() }.rel().get();
    let data = pac::scu::wdtcpu::WdtcpUyCon0::default()
        .endinit()
        .set(false)
        .lck()
        .set(true)
        .pw()
        .set(password)
        .rel()
        .set(rel);
    // SAFETY: Each bit of WDTCPUyCON0 is at least W
    unsafe { con0.write(data) };

    #[cfg(tricore_arch = "tricore")]
    while unsafe { con0.read() }.endinit().get() {}
}

// TODO Duplicate? Bad function name?
#[inline]
pub(crate) fn set_cpu_endinit_inline() {
    let password = get_cpu_watchdog_password();
    let core_id = read_cpu_core_id();
    // SAFETY: core_id is read through assembly instruction MFCR
    let con0 = unsafe { get_wdt_con0(core_id as u8) };

    // FIXME con0 is read twice
    // SAFETY: LCK is a RWH bit
    if unsafe { con0.read() }.lck().get() {
        // SAFETY: REL is a RW bitfield
        let rel = unsafe { con0.read() }.rel().get();
        let data = pac::scu::wdtcpu::WdtcpUyCon0::default()
            .endinit()
            .set(true)
            .lck()
            .set(false)
            .pw()
            .set(password)
            .rel()
            .set(rel);
        // SAFETY: Each bit of WDTCPUyCON0 is at least W
        unsafe { con0.write(data) };
    }

    // SAFETY: REL is a RW bitfield
    let rel = unsafe { con0.read() }.rel().get();
    let data = pac::scu::wdtcpu::WdtcpUyCon0::default()
        .endinit()
        .set(true)
        .lck()
        .set(true)
        .pw()
        .set(password)
        .rel()
        .set(rel);

    // SAFETY: Each bit of WDTCPUyCON0 is at least W
    unsafe { con0.write(data) };

    // FIXME do we need to enable it only with tricore like clear_cpu_endinit_inline?
    // SAFETY: ENDINIT is a RWH bit
    while !unsafe { con0.read() }.endinit().get() {}
}

// TODO Duplicate? Bad function name?
#[inline]
pub(crate) fn clear_safety_endinit_inline() {
    let password = get_safety_watchdog_password();
    let con0 = pac::SCU.wdts().wdtscon0();

    // SAFETY: LCK is a RWH bit
    if unsafe { con0.read() }.lck().get() {
        // SAFETY: Each bit of WDTSCON0 is RW
        unsafe { con0.modify(|r| r.endinit().set(true).lck().set(false).pw().set(password)) };
    }
    // SAFETY: Each bit of WDTSCON0 is RW
    unsafe { con0.modify(|r| r.endinit().set(false).lck().set(true).pw().set(password)) }

    #[cfg(tricore_arch = "tricore")]
    while unsafe { con0.read() }.endinit().get() {}
}

// TODO Duplicate? Bad function name?
#[inline]
pub(crate) fn set_safety_endinit_inline() {
    let password = get_safety_watchdog_password();
    let con0 = pac::SCU.wdts().wdtscon0();

    // SAFETY: LCK is a RWH bit
    if unsafe { con0.read() }.lck().get() {
        // SAFETY: Each bit of WDTSCON0 is RW
        unsafe { con0.modify(|r| r.endinit().set(true).lck().set(false).pw().set(password)) };
    }

    // SAFETY: Each bit of WDTSCON0 is RW
    unsafe { con0.modify(|r| r.endinit().set(true).lck().set(true).pw().set(password)) }
    #[cfg(tricore_arch = "tricore")]
    while !unsafe { con0.read() }.endinit().get() {}
}

pub fn disable_safety_watchdog() {
    clear_safety_endinit_inline();
    // SAFETY: DR is a RW bit, it can be modified only when safety endinit is de-asserted (clear_safety_endinit_inline)
    unsafe { pac::SCU.wdts().wdtscon1().modify(|p| p.dr().set(true)) };
    set_safety_endinit_inline();
}

pub fn disable_cpu_watchdog() {
    clear_cpu_endinit_inline();

    let core_id = read_cpu_core_id();
    // SAFETY: core_id is read through assembly instruction MFCR
    let con1 = unsafe { get_wdt_con1(core_id as u8) };
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
    fn test_get_wdt_con0() {
        let report = Report::new();
        report.expect_read(0xF003624C, 4, 0x00000000);
        let pwd = get_cpu_watchdog_password();
        assert_eq!(pwd, 0x3F);
    }
}
