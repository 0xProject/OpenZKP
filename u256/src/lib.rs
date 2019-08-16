// TODO: #![deny(missing_docs)]
#![warn(clippy::all)]
#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]
mod binops;
mod division;
mod gcd;
mod u256;

// TODO: This seems out of scope for U256 to export.
pub mod utils;

pub use crate::u256::U256;

// TODO: Make member functions of U256?
pub use gcd::{gcd, gcd_extended};
#[cfg(not(feature = "std"))]
extern crate no_std_compat as std;