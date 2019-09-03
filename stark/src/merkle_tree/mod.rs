mod commitment;
/// Implements Vector Commitments using Merkle Trees.
///
/// <https://eprint.iacr.org/2011/495.pdf>
// TODO: Spin of to it's own crate.
// TODO: Implement sparse Merkle trees.
// TODO: Generalize over hash implementations.
mod index;
mod node;
mod proof;
mod result;
mod vector_commitment;

#[cfg(feature = "prover")]
mod tree;

use crate::{hash::Hash, hashable::Hashable};

pub use commitment::Commitment;
pub use proof::Proof;
pub use result::{Error, Result};
pub use tree::Tree;
pub use vector_commitment::VectorCommitment;

use index::Index;
use node::Node;
