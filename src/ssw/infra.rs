use tc37x_pac::hidden::RegValue;
use tc37x_pac::SCU;

#[inline]
pub fn is_application_reset() -> bool {
    let v = unsafe { SCU.rststat().read() };

    const APP_RESET_MSK: u32 = ((0x1) << (4))
        | ((0x1) << (7))
        | ((0x1) << (6))
        | ((0x1) << (5))
        | ((0x1) << (3))
        | ((0x1) << (1))
        | ((0x1) << (0));

    if v.stbyr().get()
        | v.swd().get()
        | v.evr33().get()
        | v.evrc().get()
        | v.cb1().get()
        | v.cb0().get()
        | v.porst().get()
    {
        false
    } else if (v.data() & APP_RESET_MSK) > 0 {
        let v = v.data() & APP_RESET_MSK;
        let v = (unsafe { SCU.rstcon().read() }.data() >> ((31 - v.leading_zeros()) << 1)) & 3;
        v == 2
    } else if v.cb3().get() {
        true
    } else if (unsafe { (0xF880D000 as *const u32).read_volatile() } & (0x3 << 1)) != 0 {
        true
    } else {
        false
    }
}
