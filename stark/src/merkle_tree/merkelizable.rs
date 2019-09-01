use super::Hashable;

pub trait Merkelizable
where
    Self::Leaf: Hashable,
{
    type Leaf;

    fn len(&self) -> usize;

    fn leaf(&self, index: usize) -> &Self::Leaf;
}

// TODO ExactSizeIterator + Index<usize>
