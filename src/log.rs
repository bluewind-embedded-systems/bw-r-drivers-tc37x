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
