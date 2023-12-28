use tc37x_pac::common;
use tc37x_pac::common::hidden::RegValue; 

    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TxMsg(pub(super) *mut u8);
    unsafe impl Send for TxMsg {}
    unsafe impl Sync for TxMsg {}
    impl TxMsg {
        #[inline(always)]
        pub fn t0(self) -> common::Reg<T0, common::RW> {
            unsafe { core::mem::transmute(self.0.add(0usize)) }
        }
        #[inline(always)]
        pub fn t1(self) -> common::Reg<T1, common::RW> {
            unsafe { core::mem::transmute(self.0.add(4usize)) }
        }
        #[inline(always)]
        pub fn db(self) -> common::Reg<Db, common::RW> {
            unsafe { core::mem::transmute(self.0.add(8usize)) }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Default)]
    pub struct T0(u32, u32);
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
        pub fn id(self) -> common::RegisterField<0, 0x1FFFFFFF, 1, 0, u32, T0, common::RW> {
            unsafe { core::mem::transmute((self, 1)) }
        }

        #[inline(always)]
        pub fn rtr(self) -> common::RegisterFieldBool<29, 1, 0, T0, common::RW> {
            unsafe { core::mem::transmute((self, 1)) }
        }

        #[inline(always)]
        pub fn xtd(self) -> common::RegisterFieldBool<30, 1, 0, T0, common::RW> {
            unsafe { core::mem::transmute((self, 1)) }
        }
        #[inline(always)]
        pub fn esi(self) -> common::RegisterFieldBool<31, 1, 0, T0, common::RW> {
            unsafe { core::mem::transmute((self, 1)) }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Default)]
    pub struct T1(u32, u32);
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
        pub fn dlc(self) -> common::RegisterField<16, 0xf, 1, 0, u8, T1, common::RW> {
            unsafe { core::mem::transmute((self, 1)) }
        }

        #[inline(always)]
        pub fn brs(self) -> common::RegisterFieldBool<20, 1, 0, T1, common::RW> {
            unsafe { core::mem::transmute((self, 1)) }
        }

        #[inline(always)]
        pub fn fdf(self) -> common::RegisterFieldBool<21, 1, 0, T1, common::RW> {
            unsafe { core::mem::transmute((self, 1)) }
        }

        #[inline(always)]
        pub fn efc(self) -> common::RegisterFieldBool<23, 1, 0, T1, common::RW> {
            unsafe { core::mem::transmute((self, 1)) }
        }

        #[inline(always)]
        pub fn mm(self) -> common::RegisterField<24, 0xff, 1, 0, u8, T1, common::RW> {
            unsafe { core::mem::transmute((self, 1)) }
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
    impl Db {}