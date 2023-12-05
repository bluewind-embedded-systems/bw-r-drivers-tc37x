use crate::scu::wdt;
use tc37x_pac::{self as pac};

pub fn disable_safety_watchdog(passw: u16) {
    wdt::clear_safety_endinit_inline(passw);
    unsafe { pac::SCU.wdtscon1().modify(|p| p.dr().set(true)) };
    wdt::set_safety_endinit_inline(passw);
}

pub fn disable_cpu_watchdog(passw: u16) {
    wdt::clear_cpu_endinit_inline(passw);
    unsafe { pac::SCU.wdtcpu0con1().modify(|p| p.dr().set(true)) };
    wdt::set_cpu_endinit_inline(passw);
}

pub fn call_without_endinit<R>(f: impl FnOnce() -> R) -> R {
    call_without_cpu_endinit(|| call_without_safety_endinit(f))
}

pub fn call_without_cpu_endinit<R>(f: impl FnOnce() -> R) -> R {
    let passw = wdt::get_cpu_watchdog_password();
    call_without_cpu_endinit_passw(passw, f)
}

pub fn call_without_cpu_endinit_passw<R>(passw: u16, f: impl FnOnce() -> R) -> R {
    wdt::clear_cpu_endinit_inline(passw);
    let result = f();
    wdt::set_cpu_endinit_inline(passw);
    result
}

pub fn call_without_safety_endinit<R>(f: impl FnOnce() -> R) -> R {
    let passw = wdt::get_safety_watchdog_password();
    call_without_safety_endinit_passw(passw, f)
}

pub fn call_without_safety_endinit_passw<R>(passw: u16, f: impl FnOnce() -> R) -> R {
    wdt::clear_safety_endinit_inline(passw);
    let result = f();
    wdt::set_safety_endinit_inline(passw);
    result
}