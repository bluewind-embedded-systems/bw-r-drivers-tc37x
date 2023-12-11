use crate::cpu::asm::read_cpu_core_id;
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

    password ^ 0x003F
}

#[inline]
pub fn get_safety_watchdog_password() -> u16 {
    let password = unsafe { pac::SCU.wdtscon0().read() }.pw().get();
    password ^ 0x003F
}

#[inline]
unsafe fn get_wdt_con0(core_id: u8) -> pac::Reg<pac::scu::Wdtcpu0Con0, pac::RW> {
    // unsafe cast to get the valid SCU WDT based on the core id
    let off: *mut u8 = unsafe { core::mem::transmute(pac::SCU.wdtcpu0con0()) };
    let off = unsafe { off.add(core::mem::size_of::<u32>() * 3 * core_id as usize) };
    unsafe { core::mem::transmute(off) }
}

#[inline]
pub fn clear_cpu_endinit_inline(password: u16) {
    let core_id = read_cpu_core_id();
    let con0 = unsafe { get_wdt_con0(core_id as _) };

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

#[inline]
pub fn set_cpu_endinit_inline(_password: u16) {
    // TODO
}

#[inline]
unsafe fn get_wdt_con0_cpu2() -> pac::Reg<pac::scu::Wdtcpu2Con0, pac::RW> {
    // unsafe cast to get the valid SCU WDT based on the core id
    let off: *mut u8 = unsafe { core::mem::transmute(pac::SCU.wdtcpu0con0()) };
    let off = unsafe { off.add(core::mem::size_of::<u32>() * 3 * 2usize) };
    unsafe { core::mem::transmute(off) }
}
#[inline]
unsafe fn get_wdt_con0_cpu1() -> pac::Reg<pac::scu::Wdtcpu1Con0, pac::RW> {
    // unsafe cast to get the valid SCU WDT based on the core id
    let off: *mut u8 = unsafe { core::mem::transmute(pac::SCU.wdtcpu0con0()) };
    let off = unsafe { off.add(core::mem::size_of::<u32>() * 3) };
    unsafe { core::mem::transmute(off) }
}
#[inline]
unsafe fn get_wdt_con0_cpu0() -> pac::Reg<pac::scu::Wdtcpu0Con0, pac::RW> {
    // unsafe cast to get the valid SCU WDT based on the core id
    let off: *mut u8 = unsafe { core::mem::transmute(pac::SCU.wdtcpu0con0()) };
    unsafe { core::mem::transmute(off) }
}

#[inline]
pub fn set_cpu_endinit_inline_cpu0(password: u16) {
    let con0 = unsafe { get_wdt_con0_cpu0() };

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

    #[cfg(tricore_arch = "tricore")]
    while !unsafe { con0.read() }.endinit().get() {}
}

#[inline]
pub fn clear_safety_endinit_inline(password: u16) {
    let con0 = pac::SCU.wdtscon0();

    if unsafe { con0.read() }.lck().get() {
        unsafe { con0.modify(|r| r.endinit().set(true).lck().set(false).pw().set(password)) };
    }
    unsafe { con0.modify(|r| r.endinit().set(false).lck().set(true).pw().set(password)) }

    #[cfg(tricore_arch = "tricore")]
    while unsafe { con0.read() }.endinit().get() {}
}

#[inline]
pub fn set_cpu_endinit_inline_cpu1(password: u16) {
    let con0 = unsafe { get_wdt_con0_cpu1() };

    if unsafe { con0.read() }.lck().get() {
        let rel = unsafe { con0.read() }.rel().get();
        let data = pac::scu::Wdtcpu1Con0::default()
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
    let data = pac::scu::Wdtcpu1Con0::default()
        .endinit()
        .set(true)
        .lck()
        .set(true)
        .pw()
        .set(password)
        .rel()
        .set(rel);

    unsafe { con0.write(data) };

    #[cfg(tricore_arch = "tricore")]
    while !unsafe { con0.read() }.endinit().get() {}
}

#[inline]
pub fn set_cpu_endinit_inline_cpu2(password: u16) {
    let con0 = unsafe { get_wdt_con0_cpu2() };

    if unsafe { con0.read() }.lck().get() {
        let rel = unsafe { con0.read() }.rel().get();
        let data = pac::scu::Wdtcpu2Con0::default()
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
    let data = pac::scu::Wdtcpu2Con0::default()
        .endinit()
        .set(true)
        .lck()
        .set(true)
        .pw()
        .set(password)
        .rel()
        .set(rel);

    unsafe { con0.write(data) };

    #[cfg(tricore_arch = "tricore")]
    while !unsafe { con0.read() }.endinit().get() {}
}

#[inline]
pub fn clear_safety_endinit_inline_cpu0(password: u16) {
    let con0 = pac::SCU.wdtscon0();

    if unsafe { con0.read() }.lck().get() {
        unsafe { con0.modify(|r| r.endinit().set(true).lck().set(false).pw().set(password)) };
    }
    unsafe { con0.modify(|r| r.endinit().set(false).lck().set(true).pw().set(password)) }

    #[cfg(tricore_arch = "tricore")]
    while unsafe { con0.read() }.endinit().get() {}
}

#[inline]
pub fn set_safety_endinit_inline(password: u16) {
    let con0 = pac::SCU.wdtscon0();

    if unsafe { con0.read() }.lck().get() {
        unsafe { con0.modify(|r| r.endinit().set(true).lck().set(false).pw().set(password)) };
    }

    unsafe { con0.modify(|r| r.endinit().set(true).lck().set(true).pw().set(password)) }
    #[cfg(tricore_arch = "tricore")]
    while !unsafe { con0.read() }.endinit().get() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_wdt_con0() {
        let pwd = unsafe { get_cpu_watchdog_password() };
        assert_eq!(pwd, 0x3F);
    }
}
