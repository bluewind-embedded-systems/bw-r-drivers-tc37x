/*
*****************************************************************************
	*
	* Copyright (C) 2023 Infineon Technologies AG. All rights reserved.
	*
	* Infineon Technologies AG (Infineon) is supplying this software for use with
	* Infineon's microcontrollers. This file can be freely distributed within
	* development tools that are supporting such microcontrollers.
	*
	* THIS SOFTWARE IS PROVIDED "AS IS". NO WARRANTIES, WHETHER EXPRESS, IMPLIED
	* OR STATUTORY, INCLUDING, BUT NOT LIMITED TO, IMPLIED WARRANTIES OF
	* MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE APPLY TO THIS SOFTWARE.
	* INFINEON SHALL NOT, IN ANY CIRCUMSTANCES, BE LIABLE FOR SPECIAL, INCIDENTAL,
	* OR CONSEQUENTIAL DAMAGES, FOR ANY REASON WHATSOEVER.
	*
	******************************************************************************
*/
use core::convert::From;
use core::marker::PhantomData;

/// All side effects (I/O on peripherals) must pass through an implementation
/// of this trait.
pub trait Effect {
    unsafe fn read_volatile<T: RegValue>(addr: usize) -> T;
    unsafe fn write_volatile<T: RegValue>(addr: usize, val: T);
    unsafe fn load_modify_store(addr: usize, val: u64);
}

#[cfg(target_arch = "tricore")]
mod tricore {
    use super::hidden::RegValue;

    pub struct EffectImpl;

    impl super::Effect for EffectImpl {
        #[inline(always)]
        unsafe fn read_volatile<T: RegValue>(addr: usize) -> T {
            let val = unsafe { (addr as *mut T::DataType).read_volatile() };
            T::new(val, 0.into())
        }

        #[inline(always)]
        unsafe fn write_volatile<T: RegValue>(addr: usize, val: T) {
            unsafe { (addr as *mut T::DataType).write_volatile(val.data()) }
        }

        #[inline(always)]
        unsafe fn load_modify_store(addr: usize, val: u64) {
            unsafe {
                core::arch::asm!("ldmst [{}],0, {}", in(reg_ptr) addr,in(reg64) val);
            }
        }
    }
}

#[cfg(target_arch = "tricore")]
pub use tricore::EffectImpl;

#[cfg(not(target_arch = "tricore"))]
use super::tracing::TracingEffect as EffectImpl;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RW;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct R;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct W;

mod sealed {
    use super::*;
    pub trait Access {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
}

pub trait Access: sealed::Access + Copy {}
impl Access for R {}
impl Access for W {}
impl Access for RW {}

pub trait Read: Access {}
impl Read for RW {}
impl Read for R {}

pub trait Write: Access {}
impl Write for RW {}
impl Write for W {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Reg<T, A: Access> {
    ptr: *mut u8,
    phantom: PhantomData<*mut (T, A)>,
}
unsafe impl<T, A: Access> Send for Reg<T, A> {}
unsafe impl<T, A: Access> Sync for Reg<T, A> {}

pub mod hidden {
    use core::ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shl, Shr};

    pub trait CastFrom<A> {
        fn cast_from(val: A) -> Self;
    }

    impl CastFrom<u64> for u8 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    impl CastFrom<u64> for u16 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    impl CastFrom<u64> for u32 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    impl CastFrom<u64> for u64 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    pub trait RegValue: Default {
        type DataType: Copy
            + From<u8>
            + Into<u64>
            + CastFrom<u64>
            + Shr<usize, Output = Self::DataType>
            + Shl<usize, Output = Self::DataType>
            + BitAndAssign
            + BitAnd<Output = Self::DataType>
            + Not<Output = Self::DataType>
            + BitOrAssign;
        fn data_mut_ref(&mut self) -> &mut Self::DataType;
        fn data(&self) -> Self::DataType;
        fn get_mask_mut_ref(&mut self) -> &mut Self::DataType;
        fn new(data: Self::DataType, write_mask: Self::DataType) -> Self;
    }
}

use hidden::{CastFrom, RegValue};

pub trait RegisterValue: RegValue {
    /// Create a register value from raw value
    #[inline(always)]
    fn new(data: <Self as RegValue>::DataType) -> Self {
        RegValue::new(data, 0x0u8.into())
    }
    /// Get raw register value
    #[inline(always)]
    fn get_raw(&self) -> <Self as RegValue>::DataType {
        self.data()
    }
    /// Return a copy with register value set to `value` and write mask fully set
    #[inline(always)]
    fn set_raw(mut self, value: <Self as RegValue>::DataType) -> Self {
        *self.data_mut_ref() = value;
        *self.get_mask_mut_ref() = !(Into::<Self::DataType>::into(0x0u8));
        self
    }
}

impl<T: RegValue> RegisterValue for T {}

pub trait NoBitfieldReg: RegValue
where
    Self: Sized,
{
    #[inline(always)]
    fn get(&self) -> Self::DataType {
        self.get_raw()
    }
    #[inline(always)]
    fn set(self, value: Self::DataType) -> Self {
        self.set_raw(value)
    }
}

impl<T: RegValue, A: Access> Reg<T, A> {
    #[inline(always)]
    pub(crate) const fn from_ptr(ptr: *mut u8) -> Self {
        Self {
            ptr,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn ptr(&self) -> *mut T::DataType {
        self.ptr as _
    }
}

impl<T: RegValue, A: Read> Reg<T, A> {
    /// Read register and return a register value
    ///
    /// # Safety
    /// Read operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    #[inline(always)]
    #[must_use]
    pub unsafe fn read(&self) -> T {
        unsafe { EffectImpl::read_volatile(self.ptr as _) }
    }
}

impl<T: RegValue, A: Write> Reg<T, A> {
    /// Write register value back to register
    ///
    /// # Arguments
    ///
    /// * `reg_value` - A string slice that holds the name of the person
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    #[inline(always)]
    pub unsafe fn write(&self, reg_value: T) {
        unsafe { EffectImpl::write_volatile(self.ptr as _, reg_value) }
    }
}

impl<T: Default + RegValue, A: Write> Reg<T, A> {
    /// Init register with value returned by the closure.
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receive as input a register value initialized with register value at Power On Reset.
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    #[inline(always)]
    /// Write value computed by closure that receive as input the reset value of register
    pub unsafe fn init(&self, f: impl FnOnce(T) -> T) {
        let val = Default::default();
        let res = f(val);
        unsafe { self.write(res) };
    }
}

impl<T: RegValue, A: Read + Write> Reg<T, A> {
    #[inline(always)]
    /// Write register with value returned by the closure.
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receive as input a register value read from register.
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    pub unsafe fn modify(&self, f: impl FnOnce(T) -> T) {
        let val = unsafe { self.read() };
        let res = f(val);
        unsafe { self.write(res) };
    }
}

impl<T: Default + RegValue<DataType = u32>, A: Write> Reg<T, A> {
    #[inline(always)]
    /// Write register bitfield atomically using value returned by closure.
    /// Only the bitfield updated by closure are written back to the register.
    /// `modify_atomic` use `ldmst` assembly instruction that stall the bus until update completion.
    /// This function can be used only with 32 bits register.
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receive as input register value initialized with register value at Power On Reset.
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    pub unsafe fn modify_atomic(&self, f: impl FnOnce(T) -> T) {
        let val = Default::default();
        let mut res = f(val);
        let value: u64 = res.data() as u64 | ((*res.get_mask_mut_ref() as u64) << 32);
        let addr = self.ptr as *mut u32;
        unsafe { EffectImpl::load_modify_store(addr as _, value) }
    }
}

pub struct RegisterField<
    const START_OFFSET: usize,
    const MASK: u64,
    const DIM: u8,
    const DIM_INCREMENT: u8,
    ValueType,
    T,
    A,
> where
    T: RegValue,
    A: Access,
{
    data: T,
    index: u8,
    marker: PhantomData<(ValueType, A)>,
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueType,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>
where
    T: RegValue,
    A: Access,
{
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn from_register(data: T, index: u8) -> Self {
        Self {
            data,
            index,
            marker: PhantomData,
        }
    }
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueType,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>
where
    T: RegValue,
    A: Read,
    ValueType: CastFrom<u64>,
{
    #[inline(always)]
    pub fn get(&self) -> ValueType {
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let filtered: T::DataType = (self.data.data() >> offset) & T::DataType::cast_from(MASK);
        ValueType::cast_from(filtered.into())
    }
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueType,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>
where
    T: RegValue,
    A: Write,
    u64: From<ValueType>,
{
    #[inline(always)]
    #[must_use]
    pub fn set(mut self, value: ValueType) -> T {
        let mask = T::DataType::cast_from(MASK);
        let value: T::DataType = T::DataType::cast_from(Into::<u64>::into(value)) & mask;
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let masked_offset: T::DataType = mask << offset;
        *self.data.get_mask_mut_ref() |= masked_offset;
        *self.data.data_mut_ref() &= !masked_offset;
        *self.data.data_mut_ref() |= value << offset;
        self.data
    }
}

pub struct RegisterFieldBool<
    const START_OFFSET: usize,
    const DIM: u8,
    const DIM_INCREMENT: u8,
    T: RegValue,
    A: Access,
> {
    data: T,
    index: u8,
    marker: PhantomData<A>,
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T: RegValue, A: Read>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
{
    #[inline(always)]
    pub fn get(&self) -> bool {
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let filtered = (self.data.data().into() >> offset) & 1;
        filtered == 1
    }
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T: RegValue, A: Write>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
{
    #[inline(always)]
    #[must_use]
    pub fn set(mut self, value: bool) -> T {
        let value: T::DataType = if value {
            T::DataType::cast_from(1u64)
        } else {
            T::DataType::cast_from(0u64)
        };
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let masked_offset = T::DataType::cast_from(0x1u64) << offset;
        *self.data.get_mask_mut_ref() |= masked_offset;
        *self.data.data_mut_ref() &= !masked_offset;
        *self.data.data_mut_ref() |= value << offset;
        self.data
    }
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T: RegValue, A: Access>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
{
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn from_register(data: T, index: u8) -> Self {
        Self {
            data,
            index,
            marker: PhantomData,
        }
    }
}

pub trait FromPtr<A> {
    unsafe fn from_ptr_unchecked(ptr: *mut u8) -> Self;
}

impl<T: RegValue, A: Access> FromPtr<A> for Reg<T, A> {
    #[inline(always)]
    unsafe fn from_ptr_unchecked(ptr: *mut u8) -> Self {
        Reg::<T, A>::from_ptr(ptr)
    }
}