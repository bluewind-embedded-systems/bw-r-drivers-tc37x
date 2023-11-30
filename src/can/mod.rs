use core::fmt;
use tc37x_pac::{self as pac};
use core::marker::PhantomData;
//use embedded_hal::can::{Frame, Error, nb};
use tc37x_pac::RegisterValue;
mod can;

use crate::scu; 
pub use can::*;



//pub use embedded_hal::can::Can; 
pub struct CanNode; 
#[derive(Debug, Default)]
struct CanError;
struct Result; 

// impl Error for CanError{
//     fn kind(&self) -> embedded_hal::can::ErrorKind {
//         todo!()
//     }
// }

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MessageIdLenght {
    Standard,
    Extended,
    Both,
}

// #[derive(Debug, PartialEq, Clone, Copy)]
// pub struct MessageId {
//     pub data: u32,
//     pub lenght: MessageIdLenght,
// }

pub struct TmpType(u8);

pub struct CanFrame {
    //pub buffer_id: embedded_hal::can::Id, //RxBufferId,
    pub id: TmpType, //MessageId,
    pub data_lenght_code: TmpType, //DataLenghtCode,
    pub from: TmpType, // ReadFrom,
    pub frame_mode: TmpType, // FrameMode,
}

pub struct CanNodeCfg {}


//impl Frame for CanFrame{
    // fn new(id: impl Into<embedded_hal::can::Id>, data: &[u8]) -> Option<Self> {
    //     todo!()
    // }

    // fn new_remote(id: impl Into<embedded_hal::can::Id>, dlc: usize) -> Option<Self> {
    //     todo!()
    // }

    // fn is_extended(&self) -> bool {
    //     false
    // }

    // fn is_remote_frame(&self) -> bool {
    //     todo!()
    // }

    // fn id(&self) -> embedded_hal::can::Id {
    //     todo!()
    // }

    // fn dlc(&self) -> usize {
    //     todo!()
    // }

    // fn data(&self) -> &[u8] {
    //     todo!()
    // }
//}

// impl Can for CanNode {

impl CanNode{

   // type Frame = CanFrame; 
    //type Error = CanError;

    fn transmit(&mut self, frame: CanFrame) -> Result {
        todo!()
    }

    fn receive(&mut self) -> Result {
        todo!()
    }
}

struct CanModule {
    inner: pac::can0::Can0,
}

impl CanModule {
    pub fn is_enabled(&self) -> bool {
        !unsafe { self.inner.clc().read() }.diss().get()
    }
    pub fn is_suspended(&self) -> bool {
        unsafe { self.inner.ocs().read() }.sussta().get()
    }

    pub fn enable_module(&self) {
        let passw = scu::wdt::get_cpu_watchdog_password();
        #[cfg(feature = "log")]
        defmt::debug!("enable module watchdog passw: {:x}", passw);

        scu::wdt::clear_cpu_endinit_inline(passw);

        unsafe { self.inner.clc().modify_atomic(|r| r.disr().set(false)) };
        while !self.is_enabled() {}

        scu::wdt::set_cpu_endinit_inline(passw);
    }

}