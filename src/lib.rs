#![cfg_attr(target_arch = "tricore", no_std)]

// TODO Enable asap
// TODO #![deny(unsafe_op_in_unsafe_fn)]

// TODO Remove asap
#![allow(unused)]
// TODO Remove asap
#![allow(clippy::extra_unused_type_parameters)]

#[cfg(target_arch = "tricore")]
mod runtime;

#[cfg(not(target_arch = "tricore"))]
pub mod tracing;

pub mod gpio;

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
