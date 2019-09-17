pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Error {
    TreeToLarge,
    NumLeavesNotPowerOfTwo,
    IndexOutOfRange,
    IndicesUnsortedOrDuplicate,
    DuplicateLeafMismatch,
    NotEnoughHashes,
    RootHashMismatch,
}

// Allows error bubbling up to string with '?' operator
impl From<Error> for &'static str {
    fn from(res: Error) -> &'static str {
        match res {
            Error::TreeToLarge => "Tree too large",
            Error::NumLeavesNotPowerOfTwo => "Number of leaves not power of 2",
            Error::IndexOutOfRange => "Index out of range",
            Error::IndicesUnsortedOrDuplicate => "Indices Unsorted or Duplicate",
            Error::DuplicateLeafMismatch => "Duplicate Leaf Mismatch",
            Error::NotEnoughHashes => "Not Enough Hashes",
            Error::RootHashMismatch => "Root Hash Mismatch",
        }
    }
}
