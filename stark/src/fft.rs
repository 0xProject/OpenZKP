// We want these functions to be called `fft`
#![allow(clippy::module_name_repetitions)]
use crate::utils::Reversible;
use primefield::FieldElement;
use std::prelude::v1::*;
use u256::U256;

// OPT: Implement parallel strategies: https://inf.ethz.ch/personal/markusp/teaching/263-2300-ETH-spring12/slides/class19.pdf

pub fn fft(a: &[FieldElement]) -> Vec<FieldElement> {
    let mut result = a.to_vec();
    let root = FieldElement::root(result.len()).expect("No root of unity for input length");
    bit_reversal_fft(result.as_mut_slice(), &root);
    bit_reversal_permute(result.as_mut_slice());
    result
}

// TODO: Create a dedicated type for bit reversed vectors
pub fn fft_cofactor_bit_reversed(a: &[FieldElement], cofactor: &FieldElement) -> Vec<FieldElement> {
    let mut result = a.to_vec();
    let mut c = FieldElement::ONE;
    for element in &mut result {
        *element *= &c;
        c *= cofactor;
    }

    let root = FieldElement::root(result.len()).expect("No root of unity for input length");
    bit_reversal_fft(result.as_mut_slice(), &root);
    result
}

pub fn ifft(a: &[FieldElement]) -> Vec<FieldElement> {
    let mut result = a.to_vec();
    let n_elements = U256::from(a.len());
    // OPT: make inv_root function.
    let inverse_root = FieldElement::root(n_elements.clone())
        .expect("No root of unity for input length")
        .inv()
        .expect("No inverse for FieldElement::ZERO");
    bit_reversal_fft(result.as_mut_slice(), &inverse_root);
    bit_reversal_permute(result.as_mut_slice());
    let inverse_length = FieldElement::from(n_elements)
        .inv()
        .expect("No inverse length for empty list");
    for e in &mut result {
        *e *= &inverse_length;
    }
    result
}

fn bit_reversal_fft(coefficients: &mut [FieldElement], root: &FieldElement) {
    let n_elements = coefficients.len();
    debug_assert!(n_elements.is_power_of_two());
    debug_assert!(root.pow(n_elements).is_one());
    for layer in 0..n_elements.trailing_zeros() {
        let n_blocks = 1_usize << layer;
        let mut twiddle_factor = FieldElement::ONE;
        // OPT: In place combined update like gcd::mat_mul.
        let block_size = n_elements >> (layer + 1);
        let twiddle_factor_update = root.pow(block_size);
        for block in 0..n_blocks {
            // TODO: Do without casts.
            let block_start = 2 * reverse(block as u64, layer) as usize * block_size;
            for i in block_start..block_start + block_size {
                let j = i + block_size;
                let (left, right) = coefficients.split_at_mut(j);
                radix_2(&twiddle_factor, &mut left[i], &mut right[0]);
            }
            twiddle_factor *= &twiddle_factor_update;
        }
    }
}

/// Transforms (x0, x1) to (x0 + x1, x0 - x1)
fn butterfly(x0: &mut FieldElement, x1: &mut FieldElement) {
    // OPT: Inplace +- operation like in gcd::mat_mul.
    let t = x0.clone();
    *x0 += &*x1;
    // OPT: sub_from_assign
    *x1 -= t;
    x1.neg_assign();
}

fn radix_2(omega: &FieldElement, x0: &mut FieldElement, x1: &mut FieldElement) {
    *x1 *= omega;
    butterfly(x0, x1);
}

// See https://math.stackexchange.com/questions/1626897/whats-the-formulation-of-n-point-radix-n-for-ntt/1627247
// TODO: use
#[allow(dead_code)]
fn radix_4(
    omega: &FieldElement,
    x0: &mut FieldElement,
    x1: &mut FieldElement,
    x2: &mut FieldElement,
    x3: &mut FieldElement,
) {
    butterfly(x0, x2);
    butterfly(x1, x3);
    *x3 *= omega;
    butterfly(x0, x1);
    butterfly(x2, x3);
}

// TODO expose public ifft function which accepts bit-reversed input instead.
pub fn bit_reversal_permute<T>(v: &mut [T]) {
    let n = v.len() as u64;
    let n_bits = 63 - n.leading_zeros();
    debug_assert_eq!(1 << n_bits, n);

    for i in 0..n {
        let j = reverse(i, n_bits);
        if j > i {
            // TODO - potentially implement pure safe version
            v.swap(j as usize, i as usize) // swap is unsafe when i == j but
                                           // this is impossible here
        }
    }
}

fn reverse(x: u64, bits: u32) -> u64 {
    debug_assert!(bits <= 64);
    debug_assert!(bits == 64 || x < (1_u64 << bits));
    if bits == 0 {
        0
    } else {
        x.bit_reverse() >> (64 - bits)
    }
}

// Quickcheck needs pass by value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::polynomial::DensePolynomial;
    use macros_decl::u256h;
    use quickcheck_macros::quickcheck;

    #[test]
    fn fft_one_element_test() {
        let v = vec![FieldElement::from_hex_str("435767")];
        assert_eq!(fft(&v), v);
    }

    #[test]
    fn fft_two_element_test() {
        let a = FieldElement::from_hex_str("435767");
        let b = FieldElement::from_hex_str("123430");
        let v = vec![a.clone(), b.clone()];
        assert_eq!(fft(&v), vec![&a + &b, &a - &b]);
    }

    #[test]
    fn fft_four_element_test() {
        let v = vec![
            FieldElement::from_hex_str("4357670"),
            FieldElement::from_hex_str("1353542"),
            FieldElement::from_hex_str("3123423"),
            FieldElement::from_hex_str("9986432"),
        ];
        let expected: Vec<FieldElement> = (0..4_u64)
            .map(|i| DensePolynomial::new(&v).evaluate(&FieldElement::root(4).unwrap().pow(i)))
            .collect();

        assert_eq!(fft(&v), expected);
    }

    #[test]
    fn fft_eight_element_test() {
        let v = vec![
            FieldElement::from_hex_str("4357670"),
            FieldElement::from_hex_str("1353542"),
            FieldElement::from_hex_str("3123423"),
            FieldElement::from_hex_str("9986432"),
            FieldElement::from_hex_str("43576702"),
            FieldElement::from_hex_str("23452346"),
            FieldElement::from_hex_str("31234230"),
            FieldElement::from_hex_str("99864321"),
        ];
        let eighth_root_of_unity = FieldElement::root(8).unwrap();
        let expected: Vec<FieldElement> = (0..8_u64)
            .map(|i| DensePolynomial::new(&v).evaluate(&eighth_root_of_unity.pow(i)))
            .collect();

        assert_eq!(fft(&v), expected);
    }

    #[test]
    fn fft_test() {
        let root = FieldElement::from(u256h!(
            "063365fe0de874d9c90adb1e2f9c676e98c62155e4412e873ada5e1dee6feebb"
        ));
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

        let mut res = fft(&vector);

        for (i, x) in fft(&vector).into_iter().enumerate() {
            assert_eq!(x, DensePolynomial::new(&vector).evaluate(&root.pow(i)));
        }

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

        res = fft_cofactor_bit_reversed(&vector, &cofactor);
        bit_reversal_permute(&mut res);

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
}
