// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

use crate::cpu::asm::read_cpu_core_id;
use core::mem::transmute;
use tc37x_pac as pac;

#[inline]
pub fn get_cpu_watchdog_password() -> u16 {
    let core_id = read_cpu_core_id();
    let password = match core_id {
        0 => unsafe { pac::SCU.wdtcpu0con0().read() }.pw().get(),
        1 => unsafe { pac::SCU.wdtcpu1con0().read() }.pw().get(),
        2 => unsafe { pac::SCU.wdtcpu2con0().read() }.pw().get(),
        _ => unreachable!(),
    };

    // If PAS=0: WDTxCON0.PW[7:2] must be written with inverted current value read from WDTxCON0.PW[7:2]
    password ^ 0x003F
}

#[inline]
pub fn get_safety_watchdog_password() -> u16 {
    let password = unsafe { pac::SCU.wdts().wdtscon0().read() }.pw().get();

    // If PAS=0: WDTxCON0.PW[7:2] must be written with inverted current value read from WDTxCON0.PW[7:2]
    password ^ 0x003F
}

#[inline]
unsafe fn get_wdt_con0(core_id: u8) -> pac::Reg<pac::scu::Wdtcpu0Con0, pac::RW> {
    // unsafe cast to get the valid SCU WDT based on the core id
    let off: *mut u8 = unsafe { transmute(pac::SCU.wdtcpu0con0()) };
    let off = unsafe { off.add(core::mem::size_of::<u32>() * 3 * core_id as usize) };
    unsafe { transmute(off) }
}

#[inline]
unsafe fn get_wdt_con1(core_id: u8) -> pac::Reg<pac::scu::Wdtcpu0Con1, pac::RW> {
    // unsafe cast to get the valid SCU WDT based on the core id
    let off: *mut u8 = unsafe { transmute(pac::SCU.wdtcpu0con1()) };
    let off = unsafe { off.add(core::mem::size_of::<u32>() * 3 * core_id as usize) };
    unsafe { transmute(off) }
}

// TODO Duplicate? Bad function name?
#[inline]
pub fn clear_cpu_endinit_inline() {
    let password = get_cpu_watchdog_password();
    let core_id = read_cpu_core_id();
    let con0 = unsafe { get_wdt_con0(core_id as u8) };

    if unsafe { con0.read() }.lck().get() {
        let rel = unsafe { con0.read() }.rel().get();
        let data = pac::scu::Wdtcpu0Con0::default()
            .endinit()
            .set(true)
            .lck()
            .set(false)
            .pw()
            .set(password)
            .rel()
            .set(rel);
        unsafe { con0.write(data) };
    }

    let rel = unsafe { con0.read() }.rel().get();
    let data = pac::scu::Wdtcpu0Con0::default()
        .endinit()
        .set(false)
        .lck()
        .set(true)
        .pw()
        .set(password)
        .rel()
        .set(rel);
    unsafe { con0.write(data) };

    #[cfg(tricore_arch = "tricore")]
    while unsafe { con0.read() }.endinit().get() {}
}

// TODO Duplicate? Bad function name?
#[inline]
pub fn set_cpu_endinit_inline() {
    let password = get_cpu_watchdog_password();
    let core_id = read_cpu_core_id();
    let con0 = unsafe { get_wdt_con0(core_id as u8) };

    if unsafe { con0.read() }.lck().get() {
        let rel = unsafe { con0.read() }.rel().get();
        let data = pac::scu::Wdtcpu0Con0::default()
            .endinit()
            .set(true)
            .lck()
            .set(false)
            .pw()
            .set(password)
            .rel()
            .set(rel);
        unsafe { con0.write(data) };
    }

    let rel = unsafe { con0.read() }.rel().get();
    let data = pac::scu::Wdtcpu0Con0::default()
        .endinit()
        .set(true)
        .lck()
        .set(true)
        .pw()
        .set(password)
        .rel()
        .set(rel);

    unsafe { con0.write(data) };

    while !unsafe { con0.read() }.endinit().get() {}
}

// TODO Duplicate? Bad function name?
#[inline]
pub fn clear_safety_endinit_inline() {
    let password = get_safety_watchdog_password();
    let con0 = pac::SCU.wdts().wdtscon0();

    if unsafe { con0.read() }.lck().get() == pac::scu::wdts::wdtscon0::Lck::CONST_11 {
        unsafe {
            con0.modify(|r| {
                r.endinit()
                    .set(pac::scu::wdts::wdtscon0::Endinit::CONST_11)
                    .lck()
                    .set(pac::scu::wdts::wdtscon0::Lck::CONST_00)
                    .pw()
                    .set(password)
            })
        };
    }
    unsafe {
        con0.modify(|r| {
            r.endinit()
                .set(pac::scu::wdts::wdtscon0::Endinit::CONST_00)
                .lck()
                .set(pac::scu::wdts::wdtscon0::Lck::CONST_11)
                .pw()
                .set(password)
        })
    }

    #[cfg(tricore_arch = "tricore")]
    while unsafe { con0.read() }.endinit().get() {}
}

// TODO Duplicate? Bad function name?
#[inline]
pub fn set_safety_endinit_inline() {
    let password = get_safety_watchdog_password();
    let con0 = pac::SCU.wdts().wdtscon0();

    if unsafe { con0.read() }.lck().get() == pac::scu::wdts::wdtscon0::Lck::CONST_11 {
        unsafe {
            con0.modify(|r| {
                r.endinit()
                    .set(pac::scu::wdts::wdtscon0::Endinit::CONST_11)
                    .lck()
                    .set(pac::scu::wdts::wdtscon0::Lck::CONST_00)
                    .pw()
                    .set(password)
            })
        };
    }

    unsafe {
        con0.modify(|r| {
            r.endinit()
                .set(pac::scu::wdts::wdtscon0::Endinit::CONST_00)
                .lck()
                .set(pac::scu::wdts::wdtscon0::Lck::CONST_11)
                .pw()
                .set(password)
        })
    }
    #[cfg(tricore_arch = "tricore")]
    while !unsafe { con0.read() }.endinit().get() {}
}

pub fn disable_safety_watchdog() {
    clear_safety_endinit_inline();
    unsafe {
        pac::SCU
            .wdts()
            .wdtscon1()
            .modify(|p| p.dr().set(pac::scu::wdts::wdtscon1::Dr::CONST_11))
    };
    set_safety_endinit_inline();
}

pub fn disable_cpu_watchdog() {
    clear_cpu_endinit_inline();

    let core_id = read_cpu_core_id();
    let con1 = unsafe { get_wdt_con1(core_id as u8) };
    unsafe { con1.modify(|p| p.dr().set(true)) };

    set_cpu_endinit_inline();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_wdt_con0() {
        let pwd = get_cpu_watchdog_password();
        assert_eq!(pwd, 0x3F);
    }
}
