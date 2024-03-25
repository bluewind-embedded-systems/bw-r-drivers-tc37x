//! Logging macros and utilities.

// Log with defmt
#[cfg(all(not(feature = "log_with_env_logger"), feature = "log_with_defmt"))]
pub use tricore_log::*;

// No log system
#[cfg(all(not(feature = "log_with_defmt"), not(feature = "log_with_env_logger")))]
pub use crate::{
    no_log_debug as debug, no_log_error as error, no_log_info as info, no_log_trace as trace,
    no_log_warn as warn,
};

// Log with log crate
#[cfg(all(not(feature = "log_with_defmt"), feature = "log_with_env_logger"))]
pub use ::log::{debug, error, info, trace, warn};

// link with defmt implementation
#[cfg(feature = "log_with_defmt")]
extern crate defmt_rtt;

#[cfg(feature = "log_with_defmt")]
mod tricore_log {
    pub use defmt::debug;
    pub use defmt::error;
    pub use defmt::info;
    pub use defmt::trace;
    pub use defmt::warn;
}

#[macro_export]
macro_rules! no_log_debug {
    ($($ignored:tt)*) => {};
}

#[macro_export]
macro_rules! no_log_trace {
    ($($ignored:tt)*) => {};
}

#[macro_export]
macro_rules! no_log_info {
    ($($ignored:tt)*) => {};
}

#[macro_export]
macro_rules! no_log_warn {
    ($($ignored:tt)*) => {};
}

#[macro_export]
macro_rules! no_log_error {
    ($($ignored:tt)*) => {};
}

/// A wrapper around a slice of bytes that implements `Display` to print the
/// bytes as a hex string.
///
/// ```
/// use bw_r_drivers_tc37x::log::HexSlice;
/// let bytes = &[1u8, 2, 3, 4];
/// assert_eq!(format!("{}", HexSlice::from(bytes)), "01020304");
/// ```
pub struct HexSlice<'a>(&'a [u8]);

impl core::fmt::Display for HexSlice<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

#[cfg(feature = "log_with_defmt")]
impl defmt::Format for HexSlice<'_> {
    fn format(&self, f: defmt::Formatter) {
        for byte in self.0 {
            defmt::write!(f, "{:02x}", byte);
        }
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for HexSlice<'a> {
    fn from(value: &'a [u8; N]) -> Self {
        HexSlice(value.as_slice())
    }
}
impl<'a> From<&'a [u8]> for HexSlice<'a> {
    fn from(value: &'a [u8]) -> Self {
        HexSlice(value)
    }
}
