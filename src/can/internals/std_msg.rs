use crate::can::{reg, msg::{FilterElementConfiguration, FilterType, RxBufferId}};

pub struct StdMsg {
    inner: reg::StdMsg,
}

impl StdMsg {
    pub fn new(ptr: usize) -> Self {
        Self {
            inner: reg::StdMsg(ptr as _),
        }
    }
}

impl StdMsg {
    #[inline]
    pub fn set_standard_filter_id2(&self, id: u32) {
        unsafe { self.inner.s0().modify(|r| r.sfid2().set(id as _)) };
    }

    #[inline]
    pub fn set_standard_filter_rx_buffer_offset(&self, rx_buffer_number: RxBufferId) {
        unsafe {
            self.inner
                .s0()
                .modify(|r| r.sfid2().set(rx_buffer_number.into()))
        };
    }

    #[inline]
    pub fn set_standard_filter_id1(&self, id: u32) {
        unsafe { self.inner.s0().modify(|r| r.sfid1().set(id as _)) };
    }

    #[inline]
    pub fn set_standard_filter_configuration(
        &self,
        filter_element_configuration: FilterElementConfiguration,
    ) {
        unsafe {
            self.inner
                .s0()
                .modify(|r| r.sfec().set(filter_element_configuration as _))
        };
    }

    #[inline]
    pub fn set_standard_filter_type(&self, filter_type: FilterType) {
        unsafe { self.inner.s0().modify(|r| r.sft().set(filter_type as _)) };
    }
}
