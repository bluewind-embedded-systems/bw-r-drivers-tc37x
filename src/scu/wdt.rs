use core::default;

use tc37x_pac as pac;
use crate::cpu::asm::read_cpu_core_id;

#[inline]
pub fn get_cpu_watchdog_password() -> u16 {
    let core_id = read_cpu_core_id();
    let password = match core_id {
        0 =>  unsafe {pac::SCU.wdtcpu0con0().read()}.pw().get(),
        1 =>  unsafe {pac::SCU.wdtcpu1con0().read()}.pw().get(),
        2 =>  unsafe {pac::SCU.wdtcpu2con0().read()}.pw().get(),
        default => 0
    };

    password ^ 0x003F
}

#[inline]
pub fn get_safety_watchdog_password() -> u16 {
    let password = unsafe { pac::SCU.wdtscon0().read() }.pw().get();
    password ^ 0x003F
}


#[inline]
pub fn clear_cpu_endinit_inline(password: u16) {

}


#[inline]
pub fn set_cpu_endinit_inline(password: u16) {


}


#[inline]
pub fn clear_safety_endinit_inline(password: u16) {
    // let con0 = pac::SCU.wdtscon0();

    // if unsafe { con0.read() }.lck().get() {
    //     unsafe { con0.modify(|r| r.endinit().set(true).lck().set(false).pw().set(password)) };
    // }

    // unsafe { con0.modify(|r| r.endinit().set(false).lck().set(true).pw().set(password)) }
    
    // #[cfg(tricore_arch = "tricore")]
    // while unsafe { con0.read() }.endinit().get() {}
}

#[inline]
pub fn set_safety_endinit_inline(password: u16) {
    // let con0 = pac::SCU.wdtscon0();

    // if unsafe { con0.read() }.lck().get() {
    //     unsafe { con0.modify(|r| r.endinit().set(true).lck().set(false).pw().set(password)) };
    // }

    // unsafe { con0.modify(|r| r.endinit().set(true).lck().set(true).pw().set(password)) }
    // #[cfg(tricore_arch = "tricore")]
    // while !unsafe { con0.read() }.endinit().get() {}
}



// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     pub fn test_get_wdt_con0() {
//         use core::mem::transmute;

//         assert_eq!(
//             unsafe { transmute::<_, *const u8>(get_wdt_con0(0)) },
//             unsafe { transmute(pac::SCU.wdtcpu0con0()) }
//         );
//         assert_eq!(
//             unsafe { transmute::<_, *const u8>(get_wdt_con0(1)) },
//             unsafe { transmute(pac::SCU.wdtcpu1con0()) }
//         );
//         assert_eq!(
//             unsafe { transmute::<_, *const u8>(get_wdt_con0(2)) },
//             unsafe { transmute(pac::SCU.wdtcpu2con0()) }
//         );
//     }
// }
