// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

// TODO Is this module needed?

use tc37x_pac::common;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxMsg(pub(super) *mut u8);
unsafe impl Send for RxMsg {}
unsafe impl Sync for RxMsg {}
impl RxMsg {
    #[inline(always)]
    #[allow(unused)]
    pub fn r0(self) -> common::Reg<R0, common::RW> {
        let ptr = unsafe { self.0.add(0usize) };
        unsafe { core::mem::transmute(ptr) }
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn r1(self) -> common::Reg<R1, common::RW> {
        let ptr = unsafe { self.0.add(4usize) };
        unsafe { core::mem::transmute(ptr) }
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn db(self) -> common::Reg<Db, common::RW> {
        let ptr = unsafe { self.0.add(8usize) };
        unsafe { core::mem::transmute(ptr) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct R0(u32, u32);
impl common::hidden::RegValue for R0 {
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
impl R0 {
    #[inline(always)]
    #[allow(unused)]
    pub fn id(self) -> common::RegisterField<0, 0x1FFFFFFF, 1, 0, u32, R0, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn rtr(self) -> common::RegisterFieldBool<29, 1, 0, R0, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn xtd(self) -> common::RegisterFieldBool<30, 1, 0, R0, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn esi(self) -> common::RegisterFieldBool<31, 1, 0, R0, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct R1(u32, u32);
impl common::hidden::RegValue for R1 {
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
impl R1 {
    #[inline(always)]
    #[allow(unused)]
    pub fn rxts(self) -> common::RegisterField<0, 0xffff, 1, 0, u16, R1, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn dlc(self) -> common::RegisterField<16, 0xf, 1, 0, u8, R1, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn brs(self) -> common::RegisterFieldBool<20, 1, 0, R1, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn fdf(self) -> common::RegisterFieldBool<21, 1, 0, R1, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn fidx(self) -> common::RegisterField<24, 0x7F, 1, 0, u8, R1, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn anmf(self) -> common::RegisterFieldBool<31, 1, 0, R1, common::RW> {
        unsafe { core::mem::transmute((self, 1)) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Db(u32, u32);
impl common::hidden::RegValue for Db {
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
impl Db {}
