// TODO Remove this once the code is stable
#![allow(clippy::undocumented_unsafe_blocks)]

// Note: this module try to mimic the behavior of the pac module, for message SRAM access
// Note: transmute is used to create a Reg from a pointer, because the pac module does not support creating Reg from pointers

use super::{Reg, RegisterField, RegisterFieldBool, RW, hidden::RegValue};
use core::mem::transmute;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct TxMsg(pub(super) *mut u8);
unsafe impl Send for TxMsg {}
unsafe impl Sync for TxMsg {}
impl TxMsg {
    #[inline(always)]
    pub(crate) fn t0(self) -> Reg<T0, RW> {
        let ptr = unsafe { self.0.add(0usize) };
        unsafe { transmute(ptr) }
    }
    #[inline(always)]
    pub(crate) fn t1(self) -> Reg<T1, RW> {
        let ptr = unsafe { self.0.add(4usize) };
        unsafe { transmute(ptr) }
    }
    #[inline(always)]
    pub(crate) fn db(self) -> Reg<Db, RW> {
        let ptr = unsafe { self.0.add(8usize) };
        unsafe { transmute(ptr) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub(crate) struct T0(u32, u32);
impl RegValue for T0 {
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
impl T0 {
    #[inline(always)]
    pub(crate) fn id(self) -> RegisterField<0, 0x1FFFFFFF, 1, 0, u32, T0, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    pub(crate) fn rtr(self) -> RegisterFieldBool<29, 1, 0, T0, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    pub(crate) fn xtd(self) -> RegisterFieldBool<30, 1, 0, T0, RW> {
        unsafe { transmute((self, 1)) }
    }
    #[inline(always)]
    pub(crate) fn esi(self) -> RegisterFieldBool<31, 1, 0, T0, RW> {
        unsafe { transmute((self, 1)) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub(crate) struct T1(u32, u32);
impl RegValue for T1 {
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
impl T1 {
    #[inline(always)]
    pub(crate) fn dlc(self) -> RegisterField<16, 0xf, 1, 0, u8, T1, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    pub(crate) fn brs(self) -> RegisterFieldBool<20, 1, 0, T1, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    pub(crate) fn fdf(self) -> RegisterFieldBool<21, 1, 0, T1, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    pub(crate) fn efc(self) -> RegisterFieldBool<23, 1, 0, T1, RW> {
        unsafe { transmute((self, 1)) }
    }

    #[inline(always)]
    pub(crate) fn mm(self) -> RegisterField<24, 0xff, 1, 0, u8, T1, RW> {
        unsafe { transmute((self, 1)) }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub(crate) struct Db(u32, u32);
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
impl Db {}
