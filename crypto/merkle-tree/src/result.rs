#[cfg(feature = "std")]
use std::error;
use std::fmt;
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // False positive
        #[allow(clippy::useless_attribute)]
        #[allow(clippy::enum_glob_use)]
        use Error::*;
        match *self {
            TreeToLarge => write!(f, "Tree too large"),
            NumLeavesNotPowerOfTwo => write!(f, "Doesn't have a power of two of leaves"),
            IndexOutOfRange => write!(f, "Index out of range"),
            IndicesUnsortedOrDuplicate => write!(f, "Indices are unsorted or duplicate"),
            DuplicateLeafMismatch => write!(f, "Duplicate leaf mismatch"),
            NotEnoughHashes => write!(f, "Not enough hashes to verify proof"),
            RootHashMismatch => write!(f, "Verification failed since root hashes don't match"),
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
