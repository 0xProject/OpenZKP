// TODO: #![deny(missing_docs)]
#![warn(clippy::all)]
#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]
mod field;
mod montgomery;
mod square_root;

pub use field::FieldElement;

// TODO: Make member functions of FieldElement?
pub use field::invert_batch;

// Std/no-std imports
extern crate no_std_compat as std;
