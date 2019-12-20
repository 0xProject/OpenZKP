// This sequence needs to be repeated in each project as a workaround.
//       See https://github.com/rust-lang/cargo/issues/5034
// For clippy lints see: https://rust-lang.github.io/rust-clippy/master
// For rustc lints see: https://doc.rust-lang.org/rustc/lints/index.html
#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![warn(
    // Enable sets of warnings
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    rust_2018_idioms,
    future_incompatible,
    unused,

    // Additional unused warnings (not included in `unused`)
    unused_lifetimes,
    unused_qualifications,
    unused_results,

    // Additional misc. warnings
    anonymous_parameters,
    deprecated_in_future,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    // missing_docs,
    missing_doc_code_examples,
    private_doc_tests,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    variant_size_differences
)]
#![cfg_attr(feature = "std", warn(missing_debug_implementations,))]
// rand_xoshiro v0.4.0 is required for a zkp-stark example and v0.3.1 for criterion
#![allow(clippy::multiple_crate_versions)]

use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::{Expr, Lit};

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
            Err(syn::Error::new(
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
            "expected up to 32 bytes",
        ));
    }
    let mut result = [0_u64; 4];
    let mut iter = bytes.iter().rev();
    for i in 0..4 {
        for j in 0..8 {
            if let Some(byte) = iter.next() {
                result[i] |= u64::from(*byte) << (8 * j);
            } else {
                return Ok(result);
            }
        }
    }
    Ok(result)
}

// This function and constants are taken from primefield::montgomery
// TODO: Drop this in favour of a `const fn` call.
// These constants are copied, but we don't have u256h! to format them here.
#[allow(clippy::unreadable_literal)]
// Variables are relabeled in ways that confuse clippy.
#[allow(clippy::shadow_unrelated)]
fn montgomery_convert(x: (u64, u64, u64, u64)) -> (u64, u64, u64, u64) {
    const M64: u64 = 0xffff_ffff_ffff_ffff;
    const M: (u64, u64, u64, u64) = (1, 0, 0, 576460752303423505);
    const R2: (u64, u64, u64, u64) = (
        18446741271209837569,
        5151653887,
        18446744073700081664,
        576413109808302096,
    );

    // Clippy thinks casting u64 to u128 is lossy
    #[allow(clippy::cast_lossless)]
    const fn mac(a: u64, b: u64, c: u64, carry: u64) -> (u64, u64) {
        let ret = (a as u128) + ((b as u128) * (c as u128)) + (carry as u128);
        // We want truncation here
        #[allow(clippy::cast_possible_truncation)]
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
        const fn sbb(a: u64, b: u64, borrow: u64) -> (u64, u64) {
            let ret = (a as u128).wrapping_sub((b as u128) + ((borrow >> 63) as u128));
            // We want truncation here
            #[allow(clippy::cast_possible_truncation)]
            (ret as u64, (ret >> 64) as u64)
        }

        let (a0, borrow) = sbb(a0, M.0, 0);
        let (a1, borrow) = sbb(a1, M.1, borrow);
        let (a2, borrow) = sbb(a2, M.2, borrow);
        let (a3, _) = sbb(a3, M.3, borrow);
        (a0, a1, a2, a3)
    } else {
        (a0, a1, a2, a3)
    }
}

pub fn hex(input: TokenStream) -> TokenStream {
    // Wrapped in a closure so we can use `?` and
    // capture the Result<T,E>.
    (|| {
        let bytes = parse_hex(input)?;
        let literal = Literal::byte_string(&bytes);
        Ok(quote! { *#literal })
    })()
    .unwrap_or_else(|err: syn::Error| err.to_compile_error())
}

pub fn u256h(input: TokenStream) -> TokenStream {
    (|| {
        // TODO: Also accept integer literals
        let bytes = parse_hex(input)?;
        let limbs = bytes_to_limbs(bytes.as_slice())?;
        let c0 = Literal::u64_suffixed(limbs[0]);
        let c1 = Literal::u64_suffixed(limbs[1]);
        let c2 = Literal::u64_suffixed(limbs[2]);
        let c3 = Literal::u64_suffixed(limbs[3]);

        // TODO: Ideally we'd locally import U256 here and
        // use $crate::U256 here, but this leads to a circular
        // dependency.
        Ok(quote! { U256::from_limbs(#c0, #c1, #c2, #c3) })
    })()
    .unwrap_or_else(|err: syn::Error| err.to_compile_error())
}

pub fn field_element(input: TokenStream) -> TokenStream {
    (|| {
        // TODO: Also accept integer literals
        let bytes = parse_hex(input)?;
        let limbs = bytes_to_limbs(bytes.as_slice())?;
        let (c0, c1, c2, c3) = montgomery_convert((limbs[0], limbs[1], limbs[2], limbs[3]));
        let c0 = Literal::u64_suffixed(c0);
        let c1 = Literal::u64_suffixed(c1);
        let c2 = Literal::u64_suffixed(c2);
        let c3 = Literal::u64_suffixed(c3);

        Ok(quote! { FieldElement::from_montgomery(U256::from_limbs(#c0, #c1, #c2, #c3)) })
    })()
    .unwrap_or_else(|err: syn::Error| err.to_compile_error())
}

#[cfg(test)]
mod test {
    use super::*;
    // Note: TokenSteam does not implement Eq, so we can not compare them
    // directly. Instead we go roundabout and compare their `.to_string()`.

    #[test]
    fn hex_positive() {
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
    fn hex_negative() {
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
        // TODO: This test unstable, depending on the build environment
        // (rustc version?) it requires the single quotes to be escaped or not.
        let result = hex(quote! {"hello!"}).to_string();
        let expected_1 = quote! {compile_error ! { "Invalid hexadecimal string: Invalid character 'h' at position 0" }}.to_string();
        let expected_2 = quote! {compile_error ! { "Invalid hexadecimal string: Invalid character \'h\' at position 0" }}.to_string();
        assert!(result == expected_1 || result == expected_2);
    }

    #[test]
    fn u256h_positive() {
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
    fn field_element_positive() {
        assert_eq!(
            field_element(quote! {""}).to_string(),
            quote! {FieldElement::from_montgomery(
                U256::from_limbs(0u64, 0u64, 0u64, 0u64)
            )}
            .to_string()
        );
        assert_eq!(
            field_element(quote! {"01"}).to_string(),
            quote! {FieldElement::from_montgomery(
                U256::from_limbs(18446744073709551585u64 , 18446744073709551615u64 , 18446744073709551615u64 , 576460752303422960u64)
            )}
            .to_string()
        );
    }
}
