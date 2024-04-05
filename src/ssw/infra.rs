// TODO Remove this once the code is stable
#![allow(clippy::if_same_then_else)]

#[inline]
pub(crate) fn is_application_reset() -> bool {
    use crate::pac::RegisterValue;
    use crate::pac::SCU;

    // Reset Request Trigger Reset Status for ESR0, ESR1, SMU, SW, STM0, STM1 and STM2
    const APP_RESET_MSK: u32 = ((0x1) << (4))
        | ((0x1) << (7))
        | ((0x1) << (6))
        | ((0x1) << (5))
        | ((0x1) << (3))
        | ((0x1) << (1))
        | ((0x1) << (0));

    // SAFETY: Reset Status Register RSTSTAT is RH (no privilege required)
    let v = unsafe { SCU.rststat().read() };

    if v.stbyr().get()
        || v.swd().get()
        || v.evr33().get()
        || v.evrc().get()
        || v.cb1().get()
        || v.cb0().get()
        || v.porst().get()
    {
        false
    } else if (v.get_raw() & APP_RESET_MSK) > 0 {
        let v = v.get_raw() & APP_RESET_MSK;
        // SAFETY: Reset Configuration Register is R (no privilege required)
        let v = (unsafe { SCU.rstcon().read() }.get_raw() >> ((31 - v.leading_zeros()) << 1)) & 3;
        v == 2
    } else if v.cb3().get() {
        true
    } else if
    // SAFETY: KRST0.RSTSTAT is R (no privilege required)
    unsafe { crate::pac::CPU0.krst0().read() }.rststat().get() != 0 {
        true
    } else {
        false
    }
}
