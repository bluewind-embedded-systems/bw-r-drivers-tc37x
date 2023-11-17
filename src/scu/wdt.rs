use core::default;
use pac::scu::Wdtcpu0Con0;
use pac::scu::Wdtcpu1Con0;
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

pub trait WdtcpuXcon0
{
      fn endinit(self) -> pac::RegisterFieldBool<0,1,0,Wdtcpu1Con0, pac::RW>; 
      fn lck(self) ->  pac::RegisterFieldBool<1,1,0,Wdtcpu1Con0, pac::RW>;
      fn pw(self)  ->  pac::RegisterField<2,0x3fff,1,0,u16, Wdtcpu1Con0, pac::RW>;
      fn rel(self) ->  pac::RegisterField<16,0xffff,1,0,u16, Wdtcpu1Con0, pac::RW>;
}


// macro_rules! WdtCPUXcon0{
//     ($name:ident) => {
//         impl WdtcpuXcon0 for $name{
//             fn endinit(self) -> RegisterFieldBool<0,1,0,$name,crate::common::RW>{
//                 self.endinit()
//             }
//             fn lck(self) -> RegisterFieldBool<1,1,0,$name,crate::common::RW>
//             {
//                 self.lck()
//             }
//             fn pw(self)  -> crate::common::RegisterField<2,0x3fff,1,0,u16, $name,crate::common::RW>
//             {
//                 self.pw()
//             }
//             fn rel(self) -> crate::common::RegisterField<16,0xffff,1,0,u16,$name,crate::common::RW>
//             {
//                 self.rel()
//             }
//         }
//     };
// }
// WdtCPUXcon0!(Wdtcpu0Con0);
// WdtCPUXcon0!(Wdtcpu1Con0);



// #[inline]
pub fn set_cpu_endinit_inline(password: u16) {

//     let con0 = unsafe { get_wdt_con0(core_id as _) };

//     if unsafe { con0.read() }.lck().get() {
//         let rel = unsafe { con0.read() }.rel().get();
//         let data = pac::scu::Wdtcpu0con0::default()
//             .endinit()
//             .set(true)
//             .lck()
//             .set(false)
//             .pw()
//             .set(password)
//             .rel()
//             .set(rel);
//         unsafe { con0.write(data) };
//     }

//     let rel = unsafe { con0.read() }.rel().get();
//     let data = pac::scu::Wdtcpu0con0::default()
//         .endinit()
//         .set(true)
//         .lck()
//         .set(true)
//         .pw()
//         .set(password)
//         .rel()
//         .set(rel);

//     unsafe { con0.write(data) };

//     #[cfg(tricore_arch = "tricore")]
//       while !unsafe { con0.read() }.endinit().get() {}
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
pub fn set_safety_endinit_inline(password: u16) {
     let con0 = pac::SCU.wdtscon0();

    if unsafe { con0.read() }.lck().get() {
         unsafe { con0.modify(|r| r.endinit().set(true).lck().set(false).pw().set(password)) };
    }

    unsafe { con0.modify(|r| r.endinit().set(true).lck().set(true).pw().set(password)) }
    #[cfg(tricore_arch = "tricore")]
    while !unsafe { con0.read() }.endinit().get() {}
}


mod tests {
    use super::*;

    #[test]
    pub fn test_get_wdt_con0() {
        use core::mem::transmute;

        assert_eq!(
            unsafe { get_cpu_watchdog_password() },
            0 
        );
      
    }
}
