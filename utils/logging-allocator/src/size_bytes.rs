use std::fmt;

/// Wrapper for `usize` to represent number of bytes.
/// 
/// Provides a pretty implementation of `std::fmt::Display`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
pub(crate) struct SizeBytes(usize);

impl fmt::Display for SizeBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size = self.0 as f32;
        if size < 1.0e3 {
            write!(f, "{} B", size)
        } else if size < 1.0e6 {
            write!(f, "{:.1$} kB", size / 1.0e3, 5 - (size.log10().floor() as usize))
        } else if size < 1.0e9 {
            write!(f, "{:.1$} MB", size / 1.0e6, 8 - (size.log10().floor() as usize))
        } else if size < 1.0e12 {
            write!(f, "{:.1$} GB", size / 1.0e9, 11 - (size.log10().floor() as usize))
        } else {
            write!(f, "{} TB", size / 1.0e12)
        }
    }
}

impl From<usize> for SizeBytes {
    fn from(size: usize) -> Self {
        Self(size)
    }
}
