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
        match *self {
            Error::TreeToLarge => write!(f, "Tree too large"),
            Error::NumLeavesNotPowerOfTwo => write!(f, "Doesn't have a power of two of leaves"),
            Error::IndexOutOfRange => write!(f, "Index out of range"),
            Error::IndicesUnsortedOrDuplicate => write!(f, "Indices are unsorted or duplicate"),
            Error::DuplicateLeafMismatch => write!(f, "Duplicate leaf mismatch"),
            Error::NotEnoughHashes => write!(f, "Not enough hashes to verify proof"),
            Error::RootHashMismatch => {
                write!(f, "Verification failed since root hashes don't match")
            }
        }
    }
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            _ => None,
        }
    }
}
