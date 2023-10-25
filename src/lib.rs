#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

mod panic_handler;

#[cfg(not(target_arch = "tricore"))]
pub mod tracing;
