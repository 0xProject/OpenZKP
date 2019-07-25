use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::{Expr, Lit};

fn parse_string(input: TokenStream) -> syn::Result<String> {
    let input: Expr = syn::parse2(input)?;
    let result = match input {
        Expr::Lit(expr_lit) => {
            match expr_lit.lit {
                Lit::Str(s) => Some(s),
                _ => None,
            }
        }
        _ => None,
    };
    match result {
        Some(b) => Ok(b.value()),
        None => {
            return Err(syn::Error::new(
                Span::call_site(),
                "Expected hexadecimal string",
            ))
        }
    }
}

fn parse_hex(input: TokenStream) -> syn::Result<Vec<u8>> {
    let string = parse_string(input)?;
    hex::decode(&string).map_err(|err| {
        syn::Error::new(
            Span::call_site(),
            format!("Invalid hexadecimal string: {}", err),
        )
    })
}

pub fn hex(input: TokenStream) -> TokenStream {
    match parse_hex(input) {
        Ok(bytes) => {
            let literal = Literal::byte_string(&bytes);
            quote! { #literal }
        }
        Err(err) => TokenStream::from(err.to_compile_error()),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_hex() {
        assert_eq!(hex(quote! {""}).to_string(), quote! {b""}.to_string());
        assert_eq!(hex(quote! {"00"}).to_string(), quote! {b"\0"}.to_string());
        assert_eq!(hex(quote! {"0f"}).to_string(), quote! {b"\x0F"}.to_string());
        assert_eq!(hex(quote! {"0F"}).to_string(), quote! {b"\x0F"}.to_string());
        assert_eq!(
            hex(quote! {"0123456789abCDeF"}).to_string(),
            quote! {b"\x01#Eg\x89\xAB\xCD\xEF"}.to_string()
        );
    }

    #[test]
    pub fn test_hex_negative() {
        assert_eq!(hex(quote! {0x00000234320323247423897429387489237498273498237498237489237492384723984783928}).to_string(), quote! {compile_error ! { "unexpected end of input, unsupported expression; enable syn\'s features=[\"full\"]" }}.to_string());
        assert_eq!(hex(quote! {}).to_string(), quote! {compile_error ! { "unexpected end of input, unsupported expression; enable syn\'s features=[\"full\"]" }}.to_string());
        assert_eq!(
            hex(quote! {pub fn asd() {}}).to_string(),
            quote! {compile_error ! { "unsupported expression; enable syn\'s features=[\"full\"]" }}.to_string()
        );
        assert_eq!(
            hex(quote! {123}).to_string(),
            quote! {compile_error ! { "Expected hexadecimal string" }}.to_string()
        );
        assert_eq!(
            hex(quote! {"hello!"}).to_string(),
            quote! {compile_error ! { "Invalid hexadecimal string: Invalid character \'h\' at position 0" }}.to_string()
        );
    }
}
