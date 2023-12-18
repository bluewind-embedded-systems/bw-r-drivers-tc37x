// no_std is required for the target
#![cfg_attr(target_arch = "tricore", no_std)]

// Deny implicit unsafe blocks
#![deny(unsafe_op_in_unsafe_fn)]

// Catch usage of `print!` and `println!`
#![deny(print_stdout)]

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
mod util;

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
