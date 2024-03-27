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

    // TODO: why only bits [0:7]?
    const APP_RESET_MSK: u32 = ((0x1) << (4))
        | ((0x1) << (7))
        | ((0x1) << (6))
        | ((0x1) << (5))
        | ((0x1) << (3))
        | ((0x1) << (1))
        | ((0x1) << (0));

    // SAFETY: Reset Status Register RSTSTAT is RH (no privilege required)
    let v = unsafe { SCU.rststat().read() };

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
        // SAFETY: Reset Configuration Register is R (no privilege required)
        let v = (unsafe { SCU.rstcon().read() }.get_raw() >> ((31 - v.leading_zeros()) << 1)) & 3;
        v == 2
    } else if v.cb3().get().0 == 1 {
        true
    } else if
        // SAFETY: KRST0.RSTSTAT is R (no privilege required)
        unsafe { tc37x::CPU0.krst0().read().rststat().get().0 } != 0 {
        true
    } else {
        false
    }
}
