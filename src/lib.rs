// no_std is required for the target
#![cfg_attr(target_arch = "tricore", no_std)]
// Deny implicit unsafe blocks
#![deny(unsafe_op_in_unsafe_fn)]
// Catch usage of `print!` and `println!`
#![deny(clippy::print_stdout)]
// Allow uninlined format args. We need to switch seamlessly between defmt and log
#![allow(clippy::uninlined_format_args)]
// Floating point calculations are usually imprecise, so asking if two values
// are exactly equal is asking for trouble.
#![deny(clippy::float_cmp)]
// TODO Remove this once log module correctly handles ignored arguments
#![allow(unused_imports)]

#[cfg(target_arch = "tricore")]
mod runtime;

#[cfg(not(target_arch = "tricore"))]
pub mod tracing;

pub mod can;
pub mod cpu;
pub mod gpio;
pub mod log;
pub mod scu;
pub mod ssw;
pub mod util;

pub use tc37x_pac as pac;

mod sealed {
    pub trait Sealed {}
}

pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}
