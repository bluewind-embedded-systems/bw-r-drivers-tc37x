use crate::can::{reg, msg::{FilterType, FilterElementConfiguration, RxBufferId}};

pub struct ExtMsg {
    inner: reg::ExtMsg,
}

impl ExtMsg {
    pub fn new(ptr: usize) -> Self {
        Self {
            inner: reg::ExtMsg(ptr as _),
        }
    }
}

impl ExtMsg {
    #[inline]
    pub fn set_extended_filter_id2(&self, id: u32) {
        unsafe { self.inner.f1().modify(|r| r.efid2().set(id)) };
    }

    #[inline]
    pub fn set_extended_filter_rx_buffer_offset(&self, rx_buffer_number: RxBufferId) {
        unsafe {
            self.inner
                .f1()
                .modify(|r| r.efid2().set(rx_buffer_number.into()))
        };
    }

    #[inline]
    pub fn set_extended_filter_id1(&self, id: u32) {
        unsafe { self.inner.f0().modify(|r| r.efid1().set(id)) };
    }

    #[inline]
    pub fn set_extended_filter_configuration(
        &self,
        filter_element_configuration: FilterElementConfiguration,
    ) {
        unsafe {
            self.inner
                .f0()
                .modify(|r| r.efec().set(filter_element_configuration as _))
        };
    }

    #[inline]
    pub fn set_extended_filter_type(&self, filter_type: FilterType) {
        unsafe { self.inner.f1().modify(|r| r.eft().set(filter_type as _)) };
    }
}
