use tc37x_pac::common;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtMsg(pub *mut u8);
unsafe impl Send for ExtMsg {}
unsafe impl Sync for ExtMsg {}
impl ExtMsg {
    #[inline(always)]
    #[allow(unused)]
    pub fn f0(self) -> common::Reg<F0, common::RW> {
        unsafe { core::mem::transmute(self.0.add(0usize)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn f1(self) -> common::Reg<F1, common::RW> {
        unsafe { core::mem::transmute(self.0.add(4usize)) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct F0(u32, u32);
impl common::hidden::RegValue for F0 {
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
impl F0 {
    #[inline(always)]
    #[allow(unused)]
    pub fn efid1(self) -> common::RegisterField<0, 0x1FFFFFFF, 1, 0, u32, F0, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn efec(self) -> common::RegisterField<29, 0x7, 1, 0, u8, F0, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct F1(u32, u32);
impl common::hidden::RegValue for F1 {
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
impl F1 {
    #[inline(always)]
    #[allow(unused)]
    pub fn efid2(self) -> common::RegisterField<0, 0x1FFFFFFF, 1, 0, u32, F1, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn eft(self) -> common::RegisterField<30, 0x3, 1, 0, u8, F1, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }
}
