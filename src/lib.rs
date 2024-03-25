// Configure clippy to be very strict
#![warn(clippy::all, clippy::pedantic)]
// Allow some clippy lints
// We prefer absolute paths in some contexts
#![allow(clippy::absolute_paths)]
// TODO For safety reasons, clippy::arithmetic_side_effects should be enabled
#![allow(clippy::arithmetic_side_effects)]
// TODO For safety reasons, clippy::as_conversions should be enabled
#![allow(clippy::as_conversions)]
// TODO For safety reasons, clippy::cast_possible_truncation should be enabled
#![allow(clippy::cast_possible_truncation)]
// TODO For safety reasons, clippy::cast_precision_loss should be enabled
#![allow(clippy::cast_precision_loss)]
// TODO For safety reasons, clippy::cast_sign_loss should be enabled
#![allow(clippy::cast_sign_loss)]
// TODO For safety reasons, clippy::default_numeric_fallback should be enabled
#![allow(clippy::default_numeric_fallback)]
// TODO This should be enabled to achieve better documentation
#![allow(clippy::doc_markdown)]
// We want exhaustive enums
#![allow(clippy::exhaustive_enums)]
// We want exhaustive structs
#![allow(clippy::exhaustive_structs)]
// We want to deny floating point arithmetic when possible. It should be enable only for specific cases.
#![deny(clippy::float_arithmetic)]
// This is enabled for readability
#![warn(clippy::if_not_else)]
// Indexing and slicing can panic at runtime and there are safe alternatives.
#![deny(clippy::indexing_slicing)]
// We know what we are doing here, we allow inline always because we are working with embedded systems
#![allow(clippy::inline_always)]
// This is enabled for readability
#![deny(clippy::items_after_statements)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::manual_assert)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::map_err_ignore)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::min_ident_chars)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::missing_assert_message)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::missing_docs_in_private_items)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::missing_errors_doc)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::missing_inline_in_public_items)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::missing_panics_doc)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::missing_trait_methods)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::mod_module_files)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::module_name_repetitions)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::multiple_inherent_impl)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::multiple_unsafe_ops_per_block)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::must_use_candidate)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::needless_pass_by_value)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::panic)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::partial_pub_fields)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::pattern_type_mismatch)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::ptr_as_ptr)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::pub_use)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::pub_with_shorthand)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::question_mark_used)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::return_self_not_must_use)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::same_name_method)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::self_named_module_files)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::semicolon_if_nothing_returned)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::semicolon_inside_block)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::semicolon_outside_block)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::shadow_reuse)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::shadow_same)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::similar_names)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::single_call_fn)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::single_char_lifetime_names)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::std_instead_of_alloc)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::std_instead_of_core)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::too_many_lines)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::unimplemented)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::unreachable)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::unreadable_literal)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::unseparated_literal_suffix)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::unused_self)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::unwrap_used)]
// TODO This lint should probably be enabled. Check what happens when it is enabled. Fix or suppress it as locally as possible.
#![allow(clippy::wildcard_imports)]
// Deny implicit unsafe blocks
#![deny(unsafe_op_in_unsafe_fn)]
// Catch usage of `print!` and `println!`
#![deny(clippy::print_stdout)]
// Floating point calculations are usually imprecise, so asking if two values
// are exactly equal is asking for trouble.
#![deny(clippy::float_cmp)]
// All errors should be described
#![deny(clippy::result_unit_err)]
// Allow uninlined format args. We need to switch seamlessly between defmt and log
#![allow(clippy::uninlined_format_args)]
// no_std is required for the target
#![cfg_attr(target_arch = "tricore", no_std)]
// This lint ensures that each unsafe operation must be independently justified.
#![warn(clippy::multiple_unsafe_ops_per_block)] // Since Rust 1.69
#![warn(clippy::undocumented_unsafe_blocks)] // Since Rust 1.58
#![warn(unused_unsafe)]
#![warn(clippy::unnecessary_safety_comment)]
// Since Rust 1.67

// Add the documentation from the README.md file to the crate root documentation
#![doc = include_str!("../README.md")]
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
pub use tc37x as pac;

mod sealed {
    pub trait Sealed {}
}

pub(crate) use sealed::Sealed;

fn stripped_type_name<T>() -> &'static str {
    let s = core::any::type_name::<T>();
    let p = s.split("::");
    p.last().unwrap()
}
