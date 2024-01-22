// FIXME Remove
#![allow(dead_code)]

use tc37x_pac::{self as pac};

fn rststat_stbyr_to_bool(reg: pac::scu::rststat::Stbyr) -> bool {
    reg == pac::scu::rststat::Stbyr::CONST_11
}
fn rststat_swd_to_bool(reg: pac::scu::rststat::Swd) -> bool {
    reg == pac::scu::rststat::Swd::CONST_11
}
fn rststat_evr33_to_bool(reg: pac::scu::rststat::Evr33) -> bool {
    reg == pac::scu::rststat::Evr33::CONST_11
}
fn rststat_evrc_to_bool(reg: pac::scu::rststat::Evrc) -> bool {
    reg == pac::scu::rststat::Evrc::CONST_11
}
fn rststat_cb3_to_bool(reg: pac::scu::rststat::Cb3) -> bool {
    reg == pac::scu::rststat::Cb3::CONST_11
}
fn rststat_cb1_to_bool(reg: pac::scu::rststat::Cb1) -> bool {
    reg == pac::scu::rststat::Cb1::CONST_11
}
fn rststat_cb0_to_bool(reg: pac::scu::rststat::Cb0) -> bool {
    reg == pac::scu::rststat::Cb0::CONST_11
}
fn rststat_porst_to_bool(reg: pac::scu::rststat::Porst) -> bool {
    reg == pac::scu::rststat::Porst::CONST_11
}

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

    if rststat_stbyr_to_bool(v.stbyr().get())
        | rststat_swd_to_bool(v.swd().get())
        | rststat_evr33_to_bool(v.evr33().get())
        | rststat_evrc_to_bool(v.evrc().get())
        | rststat_cb1_to_bool(v.cb1().get())
        | rststat_cb0_to_bool(v.cb0().get())
        | rststat_porst_to_bool(v.porst().get())
    {
        false
    } else if (v.get_raw() & APP_RESET_MSK) > 0 {
        let v = v.get_raw() & APP_RESET_MSK;
        let v = (unsafe { SCU.rstcon().read() }.get_raw() >> ((31 - v.leading_zeros()) << 1)) & 3;
        v == 2
    } else if rststat_cb3_to_bool(v.cb3().get()) {
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
