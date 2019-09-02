use super::{Commitment, Hash, Hashable, Tree};

// TODO: Rename to VectorCommitment
pub trait Merkelizable
where
    Self::Leaf: Hashable,
{
    type Leaf;

    fn len(&self) -> usize;

    fn leaf(&self, index: usize) -> &Self::Leaf;

    fn leaf_hash(&self, index: usize) -> Hash {
        self.leaf(index).hash()
    }

    // TODO
    // fn commit(&self) -> (Commitment, Tree<'a, Self>) {
    //    let tree = Tree::from_leaves(self);
    //    (tree.commitment(), tree)
    //}
}

// TODO ExactSizeIterator + Index<usize>
