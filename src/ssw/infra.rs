// FIXME Remove
#![allow(dead_code)]
#![allow(clippy::needless_bool)]
#![allow(clippy::if_same_then_else)]

#[cfg(target_arch = "tricore")]
#[inline]
pub fn is_application_reset() -> bool {
    use tc37x_pac::RegisterValue;
    use tc37x_pac::SCU;

    let v = unsafe { SCU.rststat().read() };

    const APP_RESET_MSK: u32 = ((0x1) << (4))
        | ((0x1) << (7))
        | ((0x1) << (6))
        | ((0x1) << (5))
        | ((0x1) << (3))
        | ((0x1) << (1))
        | ((0x1) << (0));

    if v.stbyr().get().0 == 1
        || v.swd().get().0 == 1
        || v.evr33().get().0 == 1
        || v.evrc().get().0 == 1
        || v.cb1().get().0 == 1
        || v.cb0().get().0 == 1
        || v.porst().get().0 == 1
    {
        false
    } else if (v.get_raw() & APP_RESET_MSK) > 0 {
        let v = v.get_raw() & APP_RESET_MSK;
        let v = (unsafe { SCU.rstcon().read() }.get_raw() >> ((31 - v.leading_zeros()) << 1)) & 3;
        v == 2
    } else if v.cb3().get().0 == 1 {
        true
    } else if (unsafe { (0xF880D000 as *const u32).read_volatile() } & (0x3 << 1)) != 0 {
        true
    } else {
        false
    }
}

#[cfg(not(target_arch = "tricore"))]
#[inline]
pub fn is_application_reset() -> bool {
    false
}
