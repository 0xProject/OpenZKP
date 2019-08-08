use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::{Expr, Lit};

// For use in functions that return TokenStream. Turns syn::Result into
// compile errors.
macro_rules! handle_errors {
    ($e:expr) => {
        match $e {
            Err(err) => return TokenStream::from(err.to_compile_error()),
            Ok(val) => val,
        }
    };
}

fn parse_string(input: TokenStream) -> syn::Result<String> {
    let input: Expr = syn::parse2(input)?;
    let result = match input {
        Expr::Lit(expr_lit) => {
            match expr_lit.lit {
                Lit::Str(s) => Some(s),
                // TODO: A literal large integer (> u64) would show up as
                // Lit::Verbatim(v) =>
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

fn bytes_to_limbs(bytes: &[u8]) -> syn::Result<[u64; 4]> {
    if bytes.len() > 32 {
        return Err(syn::Error::new(
            Span::call_site(),
            format!("expected up to 32 bytes"),
        ));
    }
    let mut result = [0u64; 4];
    let mut iter = bytes.iter().rev();
    for i in 0..4 {
        for j in 0..8 {
            if let Some(byte) = iter.next() {
                result[i] |= (*byte as u64) << (8 * j);
            } else {
                return Ok(result);
            }
        }
    }
    Ok(result)
}

// This function and constants are taken from primefield::montgomery
// TODO: Drop this in favour of a `const fn` call.
fn montgomery_convert(x: (u64, u64, u64, u64)) -> (u64, u64, u64, u64) {
    const M64: u64 = 0xffff_ffff_ffff_ffff;
    const M: (u64, u64, u64, u64) = (1, 0, 0, 576460752303423505);
    const R2: (u64, u64, u64, u64) = (
        18446741271209837569,
        5151653887,
        18446744073700081664,
        576413109808302096,
    );

    pub fn mac(a: u64, b: u64, c: u64, carry: u64) -> (u64, u64) {
        let ret = (a as u128) + ((b as u128) * (c as u128)) + (carry as u128);
        (ret as u64, (ret >> 64) as u64)
    }

    let k = x.0.wrapping_mul(R2.0).wrapping_mul(M64);
    let (a0, carry) = mac(0, x.0, R2.0, 0);
    let (a1, carry) = mac(0, x.0, R2.1, carry);
    let (a2, carry) = mac(0, x.0, R2.2, carry);
    let (a3, carry) = mac(0, x.0, R2.3, carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, k, M.0, 0);
    let (a0, carry) = mac(a1, k, M.1, carry);
    let (a1, carry) = mac(a2, k, M.2, carry);
    let (a2, carry) = mac(a3, k, M.3, carry);
    let a3 = a4 + carry;
    let k = x.1.wrapping_mul(R2.0).wrapping_add(a0).wrapping_mul(M64);
    let (a0, carry) = mac(a0, x.1, R2.0, 0);
    let (a1, carry) = mac(a1, x.1, R2.1, carry);
    let (a2, carry) = mac(a2, x.1, R2.2, carry);
    let (a3, carry) = mac(a3, x.1, R2.3, carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, k, M.0, 0);
    let (a0, carry) = mac(a1, k, M.1, carry);
    let (a1, carry) = mac(a2, k, M.2, carry);
    let (a2, carry) = mac(a3, k, M.3, carry);
    let a3 = a4 + carry;
    let k = x.2.wrapping_mul(R2.0).wrapping_add(a0).wrapping_mul(M64);
    let (a0, carry) = mac(a0, x.2, R2.0, 0);
    let (a1, carry) = mac(a1, x.2, R2.1, carry);
    let (a2, carry) = mac(a2, x.2, R2.2, carry);
    let (a3, carry) = mac(a3, x.2, R2.3, carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, k, M.0, 0);
    let (a0, carry) = mac(a1, k, M.1, carry);
    let (a1, carry) = mac(a2, k, M.2, carry);
    let (a2, carry) = mac(a3, k, M.3, carry);
    let a3 = a4 + carry;
    let k = x.3.wrapping_mul(R2.0).wrapping_add(a0).wrapping_mul(M64);
    let (a0, carry) = mac(a0, x.3, R2.0, 0);
    let (a1, carry) = mac(a1, x.3, R2.1, carry);
    let (a2, carry) = mac(a2, x.3, R2.2, carry);
    let (a3, carry) = mac(a3, x.3, R2.3, carry);
    let a4 = carry;
    let (_a, carry) = mac(a0, k, M.0, 0);
    let (a0, carry) = mac(a1, k, M.1, carry);
    let (a1, carry) = mac(a2, k, M.2, carry);
    let (a2, carry) = mac(a3, k, M.3, carry);
    let a3 = a4 + carry;

    // Final reduction
    if (a3, a2, a1, a0) >= (M.3, M.2, M.1, M.0) {
        (a0 - M.0, a1 - M.1, a2 - M.2, a3 - M.3)
    } else {
        (a0, a1, a2, a3)
    }
}

pub fn hex(input: TokenStream) -> TokenStream {
    let bytes = handle_errors!(parse_hex(input));
    let literal = Literal::byte_string(&bytes);
    quote! { *#literal }
}

pub fn u256h(input: TokenStream) -> TokenStream {
    // TODO: Also accept integer literals
    let bytes = handle_errors!(parse_hex(input));
    let limbs = handle_errors!(bytes_to_limbs(bytes.as_slice()));
    let c0 = Literal::u64_suffixed(limbs[0]);
    let c1 = Literal::u64_suffixed(limbs[1]);
    let c2 = Literal::u64_suffixed(limbs[2]);
    let c3 = Literal::u64_suffixed(limbs[3]);

    // TODO: Ideally we'd locally import U256 here and
    // use $crate::U256 here, but this leads to a circular
    // dependency.
    quote! { U256::from_limbs(#c0, #c1, #c2, #c3) }
}

pub fn field_h(input: TokenStream) -> TokenStream {
    // TODO: Also accept integer literals
    let bytes = handle_errors!(parse_hex(input));
    let limbs = handle_errors!(bytes_to_limbs(bytes.as_slice()));
    let (c0, c1, c2, c3) = montgomery_convert((limbs[0], limbs[1], limbs[2], limbs[3]));
    let c0 = Literal::u64_suffixed(c0);
    let c1 = Literal::u64_suffixed(c1);
    let c2 = Literal::u64_suffixed(c2);
    let c3 = Literal::u64_suffixed(c3);

    quote! { FieldElement::from_montgomery(U256::from_limbs(#c0, #c1, #c2, #c3)) }
}

#[cfg(test)]
mod test {
    use super::*;
    // Note: TokenSteam does not implement Eq, so we can not compare them
    // directly. Instead we go roundabout and compare their `.to_string()`.

    #[test]
    pub fn test_hex() {
        assert_eq!(hex(quote! {""}).to_string(), quote! {*b""}.to_string());
        assert_eq!(hex(quote! {"00"}).to_string(), quote! {*b"\0"}.to_string());
        assert_eq!(
            hex(quote! {"0f"}).to_string(),
            quote! {*b"\x0F"}.to_string()
        );
        assert_eq!(
            hex(quote! {"0F"}).to_string(),
            quote! {*b"\x0F"}.to_string()
        );
        assert_eq!(
            hex(quote! {"0123456789abCDeF"}).to_string(),
            quote! {*b"\x01#Eg\x89\xAB\xCD\xEF"}.to_string()
        );
    }

    #[test]
    pub fn test_hex_negative() {
        assert_eq!(
            hex(quote! {0x00000234320323247423897429387489237498273498237498237489237492384723984783928}).to_string(),
            quote! {compile_error ! { "Expected hexadecimal string" }}.to_string());
        assert_eq!(
            hex(quote! {}).to_string(),
            quote! {compile_error ! { "unexpected end of input, expected expression" }}.to_string()
        );
        assert_eq!(
            hex(quote! {pub fn asd() {}}).to_string(),
            quote! {compile_error ! { "expected expression" }}.to_string()
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

    #[test]
    pub fn test_u256h() {
        assert_eq!(
            u256h(quote! {""}).to_string(),
            quote! {U256::from_limbs(0u64, 0u64, 0u64, 0u64)}.to_string()
        );
        assert_eq!(
            u256h(quote! {"0000000000000004000000000000000300000000000000020000000000000001"})
                .to_string(),
            quote! {U256::from_limbs(1u64, 2u64, 3u64, 4u64)}.to_string()
        );
    }

    #[test]
    pub fn test_field_h() {
        assert_eq!(
            field_h(quote! {""}).to_string(),
            quote! {FieldElement::from_montgomery(
                U256::from_limbs(0u64, 0u64, 0u64, 0u64)
            )}
            .to_string()
        );
        assert_eq!(
            field_h(quote! {"01"}).to_string(),
            quote! {FieldElement::from_montgomery(
                U256::from_limbs(18446744073709551585u64 , 18446744073709551615u64 , 18446744073709551615u64 , 576460752303422960u64)
            )}
            .to_string()
        );
    }
}
