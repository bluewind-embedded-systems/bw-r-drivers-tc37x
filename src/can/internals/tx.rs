// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

use crate::can::msg::{MessageId, MessageIdLenght, TxBufferId};
use crate::can::{frame::DataLenghtCode, reg, FrameMode};

pub struct Tx {
    inner: reg::TxMsg,
}

impl Tx {
    pub fn new(ptr: *mut u8) -> Self {
        Self {
            inner: unsafe { core::mem::transmute(ptr) },
        }
    }

    pub fn get_ptr(&self) -> *const u8 {
        unsafe { core::mem::transmute(self.inner) }
    }
}

impl Tx {
    #[inline]
    pub fn set_frame_mode_req(&self, frame_mode: FrameMode) {
        let (fdf, brs) = match frame_mode {
            FrameMode::Standard => (false, false),
            FrameMode::FdLong => (true, false),
            FrameMode::FdLongAndFast => (true, true),
        };
        unsafe { self.inner.t1().modify(|r| r.fdf().set(fdf).brs().set(brs)) };
    }

    #[inline]
    pub fn set_msg_id(&self, message_id: MessageId) {
        let shift = if message_id.length == MessageIdLenght::Standard {
            18
        } else {
            0
        };
        let id = message_id.data << shift;
        unsafe {
            self.inner.t0().modify(|r| {
                r.xtd()
                    .set(message_id.length == MessageIdLenght::Extended)
                    .id()
                    .set(id)
            })
        };
    }

    #[inline]
    pub fn set_tx_event_fifo_ctrl(&self, enable: bool) {
        unsafe { self.inner.t1().modify(|r| r.efc().set(enable)) };
    }

    pub fn set_message_marker(&self, buffer_id: TxBufferId) {
        unsafe { self.inner.t1().modify(|r| r.mm().set(buffer_id.into())) };
    }

    #[inline]
    pub fn set_remote_transmit_req(&self, enable: bool) {
        unsafe { self.inner.t0().modify(|r| r.rtr().set(enable)) };
    }

    #[inline]
    pub fn set_err_state_indicator(&self, enable: bool) {
        unsafe { self.inner.t0().modify(|r| r.esi().set(enable)) };
    }

    #[inline]
    pub fn set_data_length(&self, data_length_code: DataLenghtCode) {
        unsafe {
            self.inner
                .t1()
                .modify(|r| r.dlc().set(data_length_code.into()))
        };
    }

    pub fn write_tx_buf_data(&self, data_length_code: DataLenghtCode, data: *const u8) {
        let destination_address = self.inner.db().ptr() as *mut u8;
        let length = data_length_code.to_length();

        unsafe { core::ptr::copy_nonoverlapping(data, destination_address, length) };
    }
}
