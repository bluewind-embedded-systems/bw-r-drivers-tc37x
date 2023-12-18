// link with defmt implementation
#[cfg(feature = "log")]
#[cfg(target_arch = "tricore")]
extern crate defmt_rtt;

#[cfg(feature = "log")]
#[cfg(target_arch = "tricore")]
mod tricore_log {
    pub use defmt::debug;
    pub use defmt::error;
    pub use defmt::info;
    pub use defmt::trace;
    pub use defmt::warn;
}

#[cfg(not(feature = "log"))]
#[cfg(target_arch = "tricore")]
mod tricore_no_log {
    #[macro_export]
    macro_rules! debug {
        ($($ignored:tt)*) => {};
    }

    #[macro_export]
    macro_rules! trace {
        ($($ignored:tt)*) => {};
    }

    #[macro_export]
    macro_rules! info {
        ($($ignored:tt)*) => {};
    }

    #[macro_export]
    macro_rules! warn {
        ($($ignored:tt)*) => {};
    }

    #[macro_export]
    macro_rules! error {
        ($($ignored:tt)*) => {};
    }
}

#[cfg(feature = "log")]
#[cfg(target_arch = "tricore")]
pub use tricore_log::*;

#[cfg(not(feature = "log"))]
#[cfg(target_arch = "tricore")]
pub use crate::{debug, error, info, trace, warn};
