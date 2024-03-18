// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

// Note: this module try to mimic the behavior of the pac module, for message SRAM access
// Note: transmute is used to create a Reg from a pointer, because the pac module does not support creating Reg from pointers

use crate::common::{hidden::RegValue, Reg, RegisterField, RegisterFieldBool, RW};
use core::mem::transmute;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxMsg(pub(super) *mut u8);

unsafe impl Send for RxMsg {}
unsafe impl Sync for RxMsg {}

impl RxMsg {
    #[inline(always)]
    #[allow(unused)]
    pub fn r0(self) -> Reg<R0, RW> {
        let ptr = unsafe { self.0.add(0usize) };
        unsafe { transmute(ptr) }
        // TODO Instead of transmute, the following code should be used (once pac supports it)
        // unsafe { Reg::<R0, RW>::from_ptr_unchecked(ptr) }
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn r1(self) -> Reg<R1, RW> {
        let ptr = unsafe { self.0.add(4usize) };
        unsafe { transmute(ptr) }
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn db(self) -> Reg<Db, RW> {
        let ptr = unsafe { self.0.add(8usize) };
        unsafe { transmute(ptr) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct R0(u32, u32);

impl RegValue for R0 {
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
    pub fn id(self) -> RegisterField<0, 0x1FFFFFFF, 1, 0, u32, R0, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn rtr(self) -> RegisterFieldBool<29, 1, 0, R0, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn xtd(self) -> RegisterFieldBool<30, 1, 0, R0, RW> {
        unsafe { transmute((self, 1)) }
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn esi(self) -> RegisterFieldBool<31, 1, 0, R0, RW> {
        unsafe { transmute((self, 1)) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct R1(u32, u32);

impl RegValue for R1 {
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
    pub fn rxts(self) -> RegisterField<0, 0xffff, 1, 0, u16, R1, RW> {
        unsafe { transmute((self, 1)) }
    }
    #[inline(always)]
    #[allow(unused)]
    pub fn dlc(self) -> RegisterField<16, 0xf, 1, 0, u8, R1, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn brs(self) -> RegisterFieldBool<20, 1, 0, R1, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn fdf(self) -> RegisterFieldBool<21, 1, 0, R1, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn fidx(self) -> RegisterField<24, 0x7F, 1, 0, u8, R1, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    #[allow(unused)]
    pub fn anmf(self) -> RegisterFieldBool<31, 1, 0, R1, RW> {
        unsafe { transmute((self, 1)) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Db(u32, u32);

impl RegValue for Db {
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
