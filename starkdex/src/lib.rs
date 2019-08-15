// TODO: #![deny(missing_docs)]
#![warn(clippy::all)]
#![deny(warnings)]
#![cfg_attr(not(feature = "std"), no_std)]
mod orders;
mod pedersen;
mod pedersen_points;
pub mod wrappers;

pub use orders::{hash_maker, hash_taker, MakerMessage};
pub use pedersen::{hash, old_hash};
pub use pedersen_points::PEDERSEN_POINTS;
extern crate no_std_compat as std;