// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

use crate::can::msg::{MessageId, MessageIdLenght, TxBufferId};
use crate::can::{frame::DataLenghtCode, reg, FrameMode};
use core::mem::transmute;

pub(crate) struct Tx {
    inner: reg::msg_tx::TxMsg,
}

impl Tx {
    pub(crate) fn new(ptr: *mut u8) -> Self {
        Self {
            inner: unsafe { transmute(ptr) },
        }
    }
}

impl Tx {
    #[inline]
    pub(crate) fn set_frame_mode_req(&self, frame_mode: FrameMode) {
        let (fdf, brs) = match frame_mode {
            FrameMode::Standard => (false, false),
            FrameMode::FdLong => (true, false),
            FrameMode::FdLongAndFast => (true, true),
        };
        // SAFETY: bits 15:0 and 22 are written with 0, fdf and brs are in range [0, 1]
        unsafe { self.inner.t1().modify(|r| r.fdf().set(fdf).brs().set(brs)) };
    }

    #[inline]
    pub(crate) fn set_msg_id(&self, message_id: MessageId) {
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
    pub(crate) fn set_tx_event_fifo_ctrl(&self, enable: bool) {
        // SAFETY: bits 15:0 and 22 are written with 0, enable is in range [0, 1]
        unsafe { self.inner.t1().modify(|r| r.efc().set(enable)) };
    }

    pub(crate) fn set_message_marker(&self, buffer_id: TxBufferId) {
        // SAFETY: bits 15:0 and 22 are written with 0, buffer_id is in range [0, 2^8)
        unsafe { self.inner.t1().modify(|r| r.mm().set(buffer_id.into())) };
    }

    #[inline]
    pub(crate) fn set_remote_transmit_req(&self, enable: bool) {
        // SAFETY: enable is in range [0, 1]
        unsafe { self.inner.t0().modify(|r| r.rtr().set(enable)) };
    }

    #[inline]
    pub(crate) fn set_err_state_indicator(&self, enable: bool) {
        // SAFETY: enable is in range [0, 1]
        unsafe { self.inner.t0().modify(|r| r.esi().set(enable)) };
    }

    #[inline]
    pub(crate) fn set_data_length(&self, data_length_code: DataLenghtCode) {
        // SAFETY: bits 15:0 and 22 are written with 0, data_length_code takes only allowed values
        unsafe {
            self.inner
                .t1()
                .modify(|r| r.dlc().set(data_length_code.into()))
        };
    }

    pub(crate) fn write_tx_buf_data(&self, data_length_code: DataLenghtCode, data: *const u8) {
        let destination_address = self.inner.db().ptr() as *mut u8;
        let length = data_length_code.to_length();

        unsafe { core::ptr::copy_nonoverlapping(data, destination_address, length) };
    }
}
