// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

use crate::can::msg::MessageIdLenght;
use crate::can::{frame::DataLenghtCode, reg, FrameMode};
use crate::log::debug;
use core::mem::transmute;

// create RxMsg using pac structure and unsafe transmute

pub struct Rx {
    inner: reg::RxMsg,
}

impl Rx {
    // TODO Are we sure we want to publish this function?
    pub fn new(ptr: *mut u8) -> Self {
        Self {
            inner: unsafe { transmute(ptr) },
        }
    }

    // TODO Are we sure we want to publish this function?
    pub fn get_ptr(&self) -> *mut u8 {
        unsafe { transmute(self.inner) }
    }
}

impl Rx {
    // TODO Are we sure we want to publish this function?
    #[inline]
    pub fn get_message_id(&self) -> u32 {
        let r0 = unsafe { self.inner.r0().read() };
        let message_length = if r0.xtd().get() {
            MessageIdLenght::Extended
        } else {
            MessageIdLenght::Standard
        };

        let id = r0.id().get();
        let shift = if message_length == MessageIdLenght::Standard {
            18
        } else {
            0
        };
        id >> shift
    }

    // TODO Are we sure we want to publish this function?
    #[inline]
    pub fn get_message_id_length(&self) -> MessageIdLenght {
        if unsafe { self.inner.r0().read() }.xtd().get() {
            MessageIdLenght::Extended
        } else {
            MessageIdLenght::Standard
        }
    }

    // TODO Are we sure we want to publish this function?
    #[inline]
    pub fn get_data_length(&self) -> DataLenghtCode {
        let d = unsafe { self.inner.r1().read() }.dlc().get();
        // SAFETY: d is a valid DataLenghtCode, because it is a 4 bit field
        unsafe { DataLenghtCode::try_from(d).unwrap_unchecked() }
    }

    // TODO Are we sure we want to publish this function?
    pub fn get_frame_mode(&self) -> FrameMode {
        let r1 = unsafe { self.inner.r1().read() };

        if r1.fdf().get() {
            if r1.brs().get() {
                FrameMode::FdLongAndFast
            } else {
                FrameMode::FdLong
            }
        } else {
            FrameMode::Standard
        }
    }

    // TODO Are we sure we want to publish this function?
    pub fn read_data(&self, data_length_code: DataLenghtCode, data: *mut u8) {
        let source_address = self.inner.db().ptr() as *const u8;
        let length = data_length_code.to_length();

        debug!("reading {} bytes from {:x}", length, source_address);

        unsafe { core::ptr::copy_nonoverlapping(source_address, data, length) };
    }
}
