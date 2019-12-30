// We want these functions to be called `fft`
#![allow(clippy::module_name_repetitions)]
use crate::{FieldElement, Pow};
use std::prelude::v1::*;

// TODO: Create a dedicated type for permuted vectors

/// Permute index for an FFT of `size`
///
/// The permutation is it's own inverse. The permutation is currently
/// a 'bit-reversal' one, where each index has its binary representation
/// reversed.
pub fn permute_index(size: usize, index: usize) -> usize {
    const USIZE_BITS: usize = 0_usize.count_zeros() as usize;
    debug_assert!(index < size);
    if size == 1 {
        0
    } else {
        debug_assert!(size.is_power_of_two());
        let bits = size.trailing_zeros() as usize;
        index.reverse_bits() >> (USIZE_BITS - bits)
    }
}

/// Permute an array of FFT results.
// TODO expose public ifft function which accepts bit-reversed input instead.
pub fn permute<T>(v: &mut [T]) {
    let n = v.len();
    for i in 0..n {
        let j = permute_index(n, i);
        if j > i {
            v.swap(i, j);
        }
    }
}

/// Out-of-place FFT with non-permuted result.
pub fn fft(a: &[FieldElement]) -> Vec<FieldElement> {
    let mut result = a.to_owned();
    fft_permuted(&mut result);
    permute(&mut result);
    result
}

/// Out-of-place inverse FFT with non-permuted result.
pub fn ifft(a: &[FieldElement]) -> Vec<FieldElement> {
    let mut result = a.to_owned();
    ifft_permuted(&mut result);
    permute(&mut result);
    result
}

/// In-place permuted FFT.
pub fn fft_permuted(x: &mut [FieldElement]) {
    let root = FieldElement::root(x.len()).expect("No root of unity for input length");
    fft_permuted_root(&root, x);
}

/// Out-of-place permuted FFT with a cofactor.
pub fn fft_cofactor_permuted_out(
    cofactor: &FieldElement,
    x: &[FieldElement],
    out: &mut [FieldElement],
) {
    // TODO: Use geometric_series
    let mut c = FieldElement::ONE;
    for (x, out) in x.iter().zip(out.iter_mut()) {
        *out = x * &c;
        c *= cofactor;
    }
    fft_permuted(out);
}

/// In-place permuted FFT with a cofactor.
pub fn fft_cofactor_permuted(cofactor: &FieldElement, x: &mut [FieldElement]) {
    // TODO: Use geometric_series
    let mut c = FieldElement::ONE;
    for element in x.iter_mut() {
        *element *= &c;
        c *= cofactor;
    }
    fft_permuted(x);
}

/// In-place permuted inverse FFT with cofactor.
pub fn ifft_permuted(x: &mut [FieldElement]) {
    // OPT: make inv_root function.
    let inverse_root = FieldElement::root(x.len())
        .expect("No root of unity for input length")
        .inv()
        .expect("No inverse for FieldElement::ZERO");
    let inverse_length = FieldElement::from(x.len())
        .inv()
        .expect("No inverse length for empty list");
    fft_permuted_root(&inverse_root, x);
    for e in x {
        *e *= &inverse_length;
    }
}

// TODO: Cache-oblivious FFT
// See https://www.csd.uwo.ca/~moreno/CS433-CS9624/Resources/Implementing_FFTs_in_Practice.pdf
// My `sysctl hw` cache sizes: 32kiB, 256kiB, 8MiB, or 1k, 8k, 256k
// FieldElements.

// TODO: https://cnx.org/contents/4kChocHM@6/Efficient-FFT-Algorithm-and-Programming-Tricks

// TODO: Radix-4 and/or Split-radix FFT
// See https://en.wikipedia.org/wiki/Split-radix_FFT_algorithm
// See http://www.fftw.org/newsplit.pdf

fn fft_permuted_root(root: &FieldElement, coefficients: &mut [FieldElement]) {
    let n_elements = coefficients.len();
    debug_assert!(n_elements.is_power_of_two());
    debug_assert_eq!(root.pow(n_elements), FieldElement::ONE);
    for layer in 0..n_elements.trailing_zeros() {
        let n_blocks = 1_usize << layer;
        let mut twiddle_factor = FieldElement::ONE;
        // OPT: In place combined update like gcd::mat_mul.
        let block_size = n_elements >> (layer + 1);
        let twiddle_factor_update = root.pow(block_size);
        for block in 0..n_blocks {
            // TODO: Do without casts.
            debug_assert!(block < n_blocks);
            let block_start = 2 * permute_index(n_blocks, block) * block_size;
            for i in block_start..block_start + block_size {
                let j = i + block_size;
                let left = coefficients[i].clone();
                let right = &coefficients[j] * &twiddle_factor;
                coefficients[i] = &left + &right;
                coefficients[j] = left - right;
            }
            twiddle_factor *= &twiddle_factor_update;
        }
    }
}

// Quickcheck needs pass by value
#[allow(clippy::needless_pass_by_value)]
// We don't care about this in tests
#[allow(clippy::redundant_clone)]
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use zkp_macros_decl::u256h;
    use zkp_u256::U256;

    // O(n^2) reference implementation evaluating
    //     x_i' = Sum_j x_j * omega_n^(ij)
    // directly using Horner's method.
    fn reference_fft(x: &[FieldElement]) -> Vec<FieldElement> {
        let root = FieldElement::root(x.len()).unwrap();
        let mut result = Vec::with_capacity(x.len());
        let mut root_i = FieldElement::ONE;
        for _ in 0..x.len() {
            let mut sum = FieldElement::ZERO;
            let mut root_ij = FieldElement::ONE;
            for xj in x {
                sum += xj * &root_ij;
                root_ij *= &root_i;
            }
            result.push(sum);
            root_i *= &root;
        }
        result
    }

    #[test]
    fn test_permute() {
        assert_eq!(permute_index(4, 0), 0);
        assert_eq!(permute_index(4, 1), 2);
        assert_eq!(permute_index(4, 2), 1);
        assert_eq!(permute_index(4, 3), 3);
    }

    #[quickcheck]
    fn check_permute(size: usize, index: usize) {
        let size = size.next_power_of_two();
        let index = index % size;
        let permuted = permute_index(size, index);
        assert!(permuted < size);
        assert_eq!(permute_index(size, permuted), index);
    }

    // TODO
    // #[test]
    // fn fft_one_element_test() {
    // let v = vec![FieldElement::from_hex_str("435767")];
    // assert_eq!(fft(&v), v);
    // }
    //
    // #[test]
    // fn fft_two_element_test() {
    // let a = FieldElement::from_hex_str("435767");
    // let b = FieldElement::from_hex_str("123430");
    // let v = vec![a.clone(), b.clone()];
    // assert_eq!(fft(&v), vec![&a + &b, &a - &b]);
    // }
    //
    // #[test]
    // fn fft_four_element_test() {
    // let v = vec![
    // FieldElement::from_hex_str("4357670"),
    // FieldElement::from_hex_str("1353542"),
    // FieldElement::from_hex_str("3123423"),
    // FieldElement::from_hex_str("9986432"),
    // ];
    // assert_eq!(fft(&v), reference_fft(&v));
    // }
    //
    // #[test]
    // fn fft_eight_element_test() {
    // let v = vec![
    // FieldElement::from_hex_str("4357670"),
    // FieldElement::from_hex_str("1353542"),
    // FieldElement::from_hex_str("3123423"),
    // FieldElement::from_hex_str("9986432"),
    // FieldElement::from_hex_str("43576702"),
    // FieldElement::from_hex_str("23452346"),
    // FieldElement::from_hex_str("31234230"),
    // FieldElement::from_hex_str("99864321"),
    // ];
    // let expected = reference_fft(&v);
    // assert_eq!(fft(&v), expected);
    // }
    //

    #[test]
    fn fft_test() {
        let cofactor = FieldElement::from(u256h!(
            "07696b8ff70e8e9285c76bef95d3ad76cdb29e213e4b5d9a9cd0afbd7cb29b5c"
        ));
        let vector = [
            FieldElement::from(u256h!(
                "008ee28fdbe9f1a7983bc1b600dfb9177c2d82d825023022ab4965d999bd3faf"
            )),
            FieldElement::from(u256h!(
                "037fa3db272cc54444894042223dcf260e1d1ec73fa9baea0e4572817fdf5751"
            )),
            FieldElement::from(u256h!(
                "054483fc9bcc150b421fae26530f8d3d2e97cf1918f534e67ef593038f683241"
            )),
            FieldElement::from(u256h!(
                "005b695b9001e5e62549557c48a23fd7f1706c1acdae093909d81451cd455b43"
            )),
            FieldElement::from(u256h!(
                "025079cb6cb547b63b67614dd2c78474c8a7b17b3bc53f7f7276984b6b67b18a"
            )),
            FieldElement::from(u256h!(
                "044729b25360c0025d244d31a5f144917e59f728a3d03dd4685c634d2b0e7cda"
            )),
            FieldElement::from(u256h!(
                "079b0e14d0bae81ff4fe55328fb09c4117bcd961cb60581eb6f2a770a42240ed"
            )),
            FieldElement::from(u256h!(
                "06c0926a786abb30b8f6e0eb9ef2278b910862717ed4beb35121d4741717e0e0"
            )),
        ];

        let res = fft(&vector);
        let expected = reference_fft(&vector);
        assert_eq!(res, expected);

        assert_eq!(
            U256::from(&res[0]),
            u256h!("06a1b7c038205cb38aaeea38662ae2259a19c14a7519bd522543f72dc7fa74b2")
        );
        assert_eq!(
            U256::from(&res[1]),
            u256h!("017884f169b20153de79a9c642d4e3259263f2e7ac5f85f5a8191f28d8f14544")
        );
        assert_eq!(
            U256::from(&res[2]),
            u256h!("03112a352e474819d491a13b700a07161eee580ff40098df978fa19f39b4fd2d")
        );
        assert_eq!(
            U256::from(&res[3]),
            u256h!("011606a821f418d13914c72b424141c5b88bdb184b0b5a55fc537587346c78a2")
        );
        assert_eq!(
            U256::from(&res[4]),
            u256h!("00dc2519322c102b8ad3628106a3ebef7c39f85215203bfc820c7a04a9645419")
        );
        assert_eq!(
            U256::from(&res[5]),
            u256h!("01df6a70d033d89376c96c45ce8dbbe4eeedce2d32636c29d3cb87b9e2074d00")
        );
        assert_eq!(
            U256::from(&res[6]),
            u256h!("00ee6a5e89e9307e64789e1a71c42105de12bfa104e32c5a381fe5c2697ffeec")
        );
        assert_eq!(
            U256::from(&res[7]),
            u256h!("048bad0760f8b52ee4f9a46964bcf1ba9439a9467b2576176b1319cec9f12db0")
        );

        let mut res = vector.clone();
        fft_cofactor_permuted(&cofactor, &mut res);
        permute(&mut res);

        assert_eq!(
            U256::from(&res[0]),
            u256h!("05d817ee1af8beff1880aad163a9912704d66e0c717a670c52db93da5ea34455")
        );
        assert_eq!(
            U256::from(&res[1]),
            u256h!("0631b16aceb1ee5711066df1ffafd9f5f451b0dc44c86e90005bc78e8bb4f861")
        );
        assert_eq!(
            U256::from(&res[2]),
            u256h!("01a30c98c149179cd16059ba201b99cf629d3e04844a50936006a185a67ad354")
        );
        assert_eq!(
            U256::from(&res[3]),
            u256h!("07a17b9035ff1ffd1f9e0bc52982effcd957bc07230830c10e51e906ed092f9e")
        );
        assert_eq!(
            U256::from(&res[4]),
            u256h!("01381787eccc6c77b0c5dff0b4b66dc0bb7d911bd705baf85f62001976e6ff27")
        );
        assert_eq!(
            U256::from(&res[5]),
            u256h!("009defa0822d287ce55035bb705319eb34e78180157e5297e6a46df9af8ef042")
        );
        assert_eq!(
            U256::from(&res[6]),
            u256h!("020b8317360c61abbc0bdce513eb42295402eb5dde3d13abfc0325f277f507bc")
        );
        assert_eq!(
            U256::from(&res[7]),
            u256h!("034738bd5956b1df55369cdc211109fd67e6ffd2ffbb08e856b1b4d1b1a2c6ae")
        );
    }

    #[quickcheck]
    fn ifft_is_inverse(v: Vec<FieldElement>) -> bool {
        if v.is_empty() {
            return true;
        }
        let truncated = &v[0..(1 + v.len()).next_power_of_two() / 2];
        truncated.to_vec() == ifft(&fft(truncated))
    }

    #[quickcheck]
    fn ifft_permuted_is_inverse(v: Vec<FieldElement>) {
        if v.is_empty() {
            return;
        }
        let original = &v[0..(1 + v.len()).next_power_of_two() / 2];
        let mut copy = original.to_owned();

        // TODO: Make it work without the permutes in between
        fft_permuted(&mut copy);
        permute(&mut copy);
        ifft_permuted(&mut copy);
        permute(&mut copy);

        assert_eq!(copy, original)
    }
}
