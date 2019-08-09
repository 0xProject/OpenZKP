extern crate proc_macro;
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub fn hex(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros_lib::hex(input.into()).into()
}

#[proc_macro_hack]
pub fn u256h(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros_lib::u256h(input.into()).into()
}

#[proc_macro_hack]
pub fn field_element(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    macros_lib::field_element(input.into()).into()
}
