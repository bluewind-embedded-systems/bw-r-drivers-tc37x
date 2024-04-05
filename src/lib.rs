//! TC37x drivers
//! This crate provides a set of drivers for the TC37x family of microcontrollers.
//! For more information, please refer to the [GitHub repository](https://github.com/bluewind-embedded-systems/bw-r-drivers-tc37x)

// Allow unknown lints to be compatible with different versions of Rust
#![allow(unknown_lints)]
// Configure clippy to be very strict
#![warn(clippy::all, clippy::pedantic)]
// We want to deny floating point arithmetic when possible. It should be enabled only for specific cases.
#![deny(clippy::float_arithmetic)]
// This is enabled for readability
#![deny(clippy::if_not_else)]
// Indexing and slicing can panic at runtime and there are safe alternatives.
#![deny(clippy::indexing_slicing)]
// This is enabled for readability
#![deny(clippy::items_after_statements)]
// Deny implicit unsafe blocks
#![deny(unsafe_op_in_unsafe_fn)]
// Catch usage of `print!` and `println!`
#![deny(clippy::print_stdout)]
// Floating point calculations are usually imprecise, so asking if two values
// are exactly equal is asking for trouble.
#![deny(clippy::float_cmp)]
// All errors should be described
#![deny(clippy::result_unit_err)]
// This lint ensures that each unsafe operation must be independently justified.
#![deny(clippy::multiple_unsafe_ops_per_block)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unused_unsafe)]
#![deny(clippy::unnecessary_safety_comment)]
#![allow(clippy::absolute_paths)]
// TODO This should be enabled to achieve better documentation
// TODO Fix undocumented errors in public API
#![allow(clippy::missing_errors_doc)]
// TODO Fix undocumented panics in public API
#![allow(clippy::missing_panics_doc)]
// TODO Fix code by using ptr_cast which is safer
#![allow(clippy::ptr_as_ptr)]
// We prefer absolute paths in some contexts
#![allow(clippy::doc_markdown)]
// We want exhaustive enums
#![allow(clippy::exhaustive_enums)]
// We want exhaustive structs
#![allow(clippy::exhaustive_structs)]
// We know what we are doing here, we allow inline always because we are working with embedded systems
#![allow(clippy::inline_always)]
// This is too strict
#![allow(clippy::needless_pass_by_value)]
// This is too strict and only about style
#![allow(clippy::semicolon_if_nothing_returned)]
// This is too strict
#![allow(clippy::wildcard_imports)]
// Allow uninlined format args. We need to switch seamlessly between defmt and log
#![allow(clippy::uninlined_format_args)]
// no_std is required for the tricore bare metal target
#![cfg_attr(target_os = "none", no_std)]
// This feature is only needed for tricore targets and enable the intrinsics module.
#![cfg_attr(target_arch = "tricore", feature(stdsimd))]

#[cfg(feature = "tracing")]
pub mod tracing;

pub mod can;
pub mod cpu;
pub mod gpio;
mod intrinsics;
pub mod log;
pub mod scu;
pub mod ssw;
pub mod util;

pub use embedded_can;
pub use embedded_hal;
pub use tc375_pac as pac;

mod sealed {
    pub trait Sealed {}
}

pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}
