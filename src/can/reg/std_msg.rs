use tc37x_pac::common;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StdMsg(pub *mut u8);
unsafe impl Send for StdMsg {}
unsafe impl Sync for StdMsg {}
impl StdMsg {
    #[inline(always)]
    #[allow(unused)]
    pub fn s0(self) -> common::Reg<S0, common::RW> {
        unsafe { core::mem::transmute(self.0.add(0usize)) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct S0(u32, u32);
impl common::hidden::RegValue for S0 {
    type DataType = u32;
    #[inline(always)]
    fn data_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.0
    }
    #[inline(always)]
    fn data(&self) -> Self::DataType {
        self.0
    }
    #[inline(always)]
    fn get_mask_mut_ref(&mut self) -> &mut Self::DataType {
        &mut self.1
    }
    #[inline(always)]
    fn new(data: Self::DataType, write_mask: Self::DataType) -> Self {
        Self(data, write_mask)
    }
}
impl S0 {
    #[inline(always)]
    #[allow(unused)]
    pub fn sfid2(self) -> common::RegisterField<0, 0x7FF, 1, 0, u16, S0, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn sfid1(self) -> common::RegisterField<16, 0x7FF, 1, 0, u16, S0, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn sfec(self) -> common::RegisterField<27, 0x7, 1, 0, u8, S0, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn sft(self) -> common::RegisterField<30, 0x3, 1, 0, u8, S0, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }
}
