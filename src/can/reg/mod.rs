// TODO Remove once the unsafe blocks are documented
#![allow(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::cast_possible_truncation)]

pub(crate) mod msg_rx;
pub(crate) mod msg_tx;

use core::convert::From;
use core::marker::PhantomData;

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct RW;
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct R;
#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct W;

mod sealed {
    use super::*;
    pub(crate) trait Access {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
}

pub(crate) trait Access: sealed::Access + Copy {}
impl Access for R {}
impl Access for W {}
impl Access for RW {}

pub(crate) trait Read: Access {}
impl Read for RW {}
impl Read for R {}

pub(crate) trait Write: Access {}
impl Write for RW {}
impl Write for W {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) struct Reg<T, A: Access> {
    ptr: *mut u8,
    phantom: PhantomData<*mut (T, A)>,
}
unsafe impl<T, A: Access> Send for Reg<T, A> {}
unsafe impl<T, A: Access> Sync for Reg<T, A> {}

pub(crate) mod hidden {
    use core::ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shl, Shr};

    pub(crate) trait CastFrom<A> {
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

    pub(crate) trait RegValue: Default {
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

pub(crate) trait RegisterValue: RegValue {
    #[inline(always)]
    fn new(data: <Self as RegValue>::DataType) -> Self {
        RegValue::new(data, 0x0u8.into())
    }
    #[inline(always)]
    fn get_raw(&self) -> <Self as RegValue>::DataType {
        self.data()
    }
    #[inline(always)]
    fn set_raw(mut self, value: <Self as RegValue>::DataType) -> Self {
        *self.data_mut_ref() = value;
        *self.get_mask_mut_ref() = !(Into::<Self::DataType>::into(0x0u8));
        self
    }
}

impl<T: RegValue> RegisterValue for T {}

pub(crate) trait NoBitfieldReg: RegValue
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
    pub(crate) const fn ptr(&self) -> *mut T::DataType {
        self.ptr as _
    }
}

impl<T: RegValue, A: Read> Reg<T, A> {
    #[inline(always)]
    #[must_use]
    pub(crate) unsafe fn read(&self) -> T {
        let v = unsafe { (self.ptr as *mut T::DataType).read_volatile() };
        T::new(v, 0.into())
    }
}

impl<T: RegValue, A: Write> Reg<T, A> {
    #[inline(always)]
    pub(crate) unsafe fn write(&self, reg_value: T) {
        unsafe {
            (self.ptr as *mut T::DataType).write_volatile(reg_value.data());
        }
    }
}

impl<T: Default + RegValue, A: Write> Reg<T, A> {
    #[inline(always)]
    pub(crate) unsafe fn init(&self, f: impl FnOnce(T) -> T) {
        let val = Default::default();
        let res = f(val);
        unsafe { self.write(res) };
    }
}

impl<T: RegValue, A: Read + Write> Reg<T, A> {
    #[inline(always)]
    pub(crate) unsafe fn modify(&self, f: impl FnOnce(T) -> T) {
        let val = unsafe { self.read() };
        let res = f(val);
        unsafe { self.write(res) };
    }
}

pub(crate) struct RegisterField<
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
    pub(crate) fn get(&self) -> ValueType {
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
    pub(crate) fn set(mut self, value: ValueType) -> T {
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

pub(crate) struct RegisterFieldBool<
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
    pub(crate) fn get(&self) -> bool {
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
    pub(crate) fn set(mut self, value: bool) -> T {
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
