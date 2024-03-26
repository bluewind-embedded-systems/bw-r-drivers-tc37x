//! Intrinsic functions for the TriCore architecture.
//!
//! These functions are used to access the hardware directly and are
//! architecture specific.
//! When the tracing feature is enabled, these functions are replaced with
//! tracing functions that log the access to the hardware.

// TODO Remove once the unsafe blocks are documented
#![allow(clippy::undocumented_unsafe_blocks)]

#[cfg(feature = "tracing")]
use crate::tracing;

/// Atomic Load-Modify-Store, store under a `mask` of a `value` to the address `addr`.
/// This is needed to atomically update a memory location or to
/// track the changes to a memory location when tracing feature is enabled.
#[allow(unreachable_code)]
#[inline(always)]
pub(crate) unsafe fn load_modify_store(addr: *mut u32, v: u32, m: u32) {
    #[cfg(feature = "tracing")]
    {
        return tracing::load_modify_store(addr as usize, u64::from(v) | (u64::from(m) << 32));
    }

    #[cfg(target_arch = "tricore")]
    {
        return unsafe {
            core::arch::tricore::intrinsics::__ldmst(addr, v, m);
        };
    }

    panic!("unsupported architecture");
}

/// Volatile write to a memory location.
/// This is the equivalent of ptr.write_volatile(val) but it is tracked when the tracing feature is enabled.
#[allow(unreachable_code)]
#[allow(dead_code)]
#[inline(always)]
pub(crate) unsafe fn write_volatile<T>(addr: *mut T, val: T)
where
    u64: From<T>,
{
    #[cfg(feature = "tracing")]
    {
        return tracing::write_volatile(addr as usize, core::mem::size_of::<T>(), val.into());
    }

    #[cfg(target_arch = "tricore")]
    {
        return unsafe { addr.write_volatile(val) };
    }

    panic!("unsupported architecture");
}

/// Volatile read from a memory location.
/// This is the equivalent of ptr.read_volatile() but it is tracked when the tracing feature is enabled.
#[allow(unreachable_code)]
#[inline(always)]
pub(crate) unsafe fn read_volatile<T>(addr: *const T) -> T
where
    T: From<u32>,
{
    #[cfg(feature = "tracing")]
    {
        let val: u64 = tracing::read_volatile(addr as usize, core::mem::size_of::<T>());
        let val: u32 = val as u32;
        return val.into();
    }

    #[cfg(target_arch = "tricore")]
    {
        return unsafe { addr.read_volatile() };
    }

    panic!("unsupported architecture");
}

#[inline(always)]
pub(crate) fn read_cpu_core_id() -> u32 {
    #[cfg(feature = "tracing")]
    {
        0
    }

    #[cfg(target_arch = "tricore")]
    unsafe {
        let value: u32;
        core::arch::asm!("mfcr {0}, 0xFE1C", out(reg32) value);
        value
    }
}

#[cfg(feature = "tracing")]
#[cfg(test)]
mod tests {
    use super::*;
    use std::string::ToString;

    #[test]
    fn test_load_modify_store() {
        let report = tracing::log::Report::new();
        unsafe {
            load_modify_store(0x1000 as *mut u32, 0x1234, 0x5678);
        }
        assert_eq!(
            report.take_log().to_string(),
            "ldms 0x00001000 0x00005678 0x00001234\n"
        );
    }

    #[test]
    fn test_write_volatile() {
        let report = tracing::log::Report::new();
        let v_ptr = 0x1000 as *mut u32;
        unsafe {
            write_volatile(v_ptr, 0x5678);
        }
        assert_eq!(
            report.take_log().to_string(),
            "w    0x00001000 04 0x00005678\n"
        );
    }

    #[test]
    fn test_read_volatile() {
        let report = tracing::log::Report::new();
        let v_ptr = 0x1000 as *mut u32;
        report.expect_read(0x1000, 4, 0x1234);
        let val = unsafe { read_volatile(v_ptr) };
        assert_eq!(val, 0x1234);
        assert_eq!(
            report.take_log().to_string(),
            "r    0x00001000 04 0x00001234\n"
        );
    }
}
