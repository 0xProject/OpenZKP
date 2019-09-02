mod commitment;
/// Implements Vector Commitments using Merkle Trees.
///
/// <https://eprint.iacr.org/2011/495.pdf>
// TODO: Spin of to it's own crate.
// TOOD: Implement sparse Merkle trees.
// TODO: Generalize over hash implementations.
mod index;
mod merkelizable;
mod node;
mod proof;

#[cfg(feature = "prover")]
mod tree;

use crate::{hash::Hash, hashable::Hashable};

pub use commitment::Commitment;
pub use merkelizable::Merkelizable;
pub use proof::Proof;
pub use tree::Tree;

use index::Index;
use node::Node;
