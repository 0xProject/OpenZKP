use proc_macro_hack::proc_macro_hack;

/// Hex literal.
///
/// (Documentation goes here on the re-export, not in the other crate.)
#[proc_macro_hack]
pub use macros_impl::hex;

/// U256 hexadecimal literal
///
/// (Documentation goes here on the re-export, not in the other crate.)
#[proc_macro_hack]
pub use macros_impl::u256h;

/// FieldElement hexadecimal literal
///
/// (Documentation goes here on the re-export, not in the other crate.)
#[proc_macro_hack]
pub use macros_impl::field_element;
