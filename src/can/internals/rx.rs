// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

use crate::can::msg::MessageIdLength;
use crate::can::{frame::DataLenghtCode, reg, FrameMode};
use crate::log::debug;
use core::mem::transmute;

// create RxMsg using pac structure and unsafe transmute

pub(crate) struct Rx {
    inner: reg::msg_rx::RxMsg,
}

impl Rx {
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            inner: unsafe { transmute(ptr) },
        }
    }
}

impl Rx {
    #[inline]
    pub(crate) fn get_message_id(&self) -> u32 {
        // SAFETY: each bit of R0 is RWH
        let r0 = unsafe { self.inner.r0().read() };
        let message_length = if r0.xtd().get() {
            MessageIdLength::Extended
        } else {
            MessageIdLength::Standard
        };

        let id = r0.id().get();
        let shift = if message_length == MessageIdLength::Standard {
            18
        } else {
            0
        };
        id >> shift
    }

    #[inline]
    pub(crate) fn get_message_id_length(&self) -> MessageIdLength {
        // SAFETY: each bit of R0 is RWH
        if unsafe { self.inner.r0().read() }.xtd().get() {
            MessageIdLength::Extended
        } else {
            MessageIdLength::Standard
        }
    }

    #[inline]
    pub(crate) fn get_data_length(&self) -> DataLenghtCode {
        // SAFETY: each bit of R1 is RWH
        let d = unsafe { self.inner.r1().read() }.dlc().get();
        // SAFETY: d is a valid DataLenghtCode, because it is a 4 bit field
        unsafe { DataLenghtCode::try_from(d).unwrap_unchecked() }
    }

    pub(crate) fn get_frame_mode(&self) -> FrameMode {
        // SAFETY: each bit of R1 is RWH
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

    pub(crate) fn read_data(&self, data_length_code: DataLenghtCode, data: *mut u8) {
        let source_address = self.inner.db().ptr() as *const u8;
        let length = data_length_code.to_length();

        debug!("reading {} bytes from {:x}", length, source_address);

        unsafe { core::ptr::copy_nonoverlapping(source_address, data, length) };
    }
}
