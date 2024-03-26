// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]
// TODO Remove this once the code is stable
#![allow(dead_code)]
// TODO Remove this once the code is stable
#![allow(clippy::needless_bool)]
// TODO Remove this once the code is stable
#![allow(clippy::if_same_then_else)]

use crate::intrinsics::read_volatile;

#[inline]
pub(crate) fn is_application_reset() -> bool {
    use crate::pac::RegisterValue;
    use crate::pac::SCU;

    const APP_RESET_MSK: u32 = ((0x1) << (4))
        | ((0x1) << (7))
        | ((0x1) << (6))
        | ((0x1) << (5))
        | ((0x1) << (3))
        | ((0x1) << (1))
        | ((0x1) << (0));

    let v = unsafe { SCU.rststat().read() };

    if v.stbyr().get() == true
        || v.swd().get() == true
        || v.evr33().get() == true
        || v.evrc().get() == true
        || v.cb1().get() == true
        || v.cb0().get() == true
        || v.porst().get() == true
    {
        false
    } else if (v.get_raw() & APP_RESET_MSK) > 0 {
        let v = v.get_raw() & APP_RESET_MSK;
        let v = (unsafe { SCU.rstcon().read() }.get_raw() >> ((31 - v.leading_zeros()) << 1)) & 3;
        v == 2
    } else if v.cb3().get() == true {
        true
    } else if (unsafe { read_volatile(0xF880D000 as *const u32) } & (0x3 << 1)) != 0 {
        true
    } else {
        false
    }
}
