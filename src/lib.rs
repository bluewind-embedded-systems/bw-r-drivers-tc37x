#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(all(target_arch = "tricore", feature = "panic_handler"))]
mod panic_handler;

#[cfg(not(target_arch = "tricore"))]
pub mod tracing;
