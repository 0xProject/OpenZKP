#![warn(clippy::all)]
#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]

use proc_macro_hack::proc_macro_hack;

/// Hex literal.
///
/// (Documentation goes here on the re-export, not in the other crate.)
#[proc_macro_hack]
pub use macros_impl::hex;
