use crate::utils::{adc, msb};
use std::u64;

#[inline(always)]
const fn val_2(lo: u64, hi: u64) -> u128 {
    ((hi as u128) << 64) | (lo as u128)
}

#[inline(always)]
const fn mul_2(a: u64, b: u64) -> u128 {
    (a as u128) * (b as u128)
}

/// Compute <hi, lo> / d, returning the quotient and the remainder.
// TODO: Make sure it uses divq on x86_64.
// See http://lists.llvm.org/pipermail/llvm-dev/2017-October/118323.html
// (Note that we require d < hi for this)
// TODO: If divq is not supported, use a fast software implementation:
// See https://gmplib.org/~tege/division-paper.pdf
#[inline(always)]
const fn divrem_2by1(lo: u64, hi: u64, d: u64) -> (u64, u64) {
    // TODO: debug_assert!(d > 0);
    let d = d as u128;
    let n = val_2(lo, hi);
    let q = n / d;
    let r = n % d;
    (q as u64, r as u64)
}

pub fn divrem_nby1(numerator: &mut [u64], divisor: u64) -> u64 {
    debug_assert!(divisor > 0);
    let mut remainder = 0;
    for i in (0..numerator.len()).rev() {
        let (ni, ri) = divrem_2by1(numerator[i], remainder, divisor);
        numerator[i] = ni;
        remainder = ri;
    }
    remainder
}

//      |  n2 n1 n0  |
//  q = |  --------  |
//      |_    d1 d0 _|
fn div_3by2(n: &[u64; 3], d: &[u64; 2]) -> u64 {
    // The highest bit of d needs to be set
    debug_assert!(d[1] >> 63 == 1);

    // The quotient needs to fit u64. For this we need <n2 n1> < <d1 d0>
    debug_assert!(n[2] < d[1] || (n[2] == d[1] && n[1] < d[0]));

    // Compute quotient and remainder
    // TODO: Use GMP's reciprocal computation.
    let (mut q, mut r) = divrem_2by1(n[1], n[2], d[1]);

    if mul_2(q, d[0]) > val_2(n[0], r) {
        q -= 1;
        r += d[1];
        if mul_2(q, d[0]) > val_2(n[0], r) {
            q -= 1;
            // UNUSED: r += d[1];
        }
    }
    q
}

// Turns numerator into remainder, returns quotient.
// Implements Knuth's division algorithm.
// See D. Knuth "The Art of Computer Programming". Sec. 4.3.1. Algorithm D.
// See https://github.com/chfast/intx/blob/master/lib/intx/div.cpp
fn divrem_nbym(numerator: &mut [u64], divisor: &mut [u64]) -> Vec<u64> {
    assert!(numerator.len() > 2);
    assert!(divisor.len() >= 2);
    assert!(divisor.last().unwrap() > &0);
    // TODO: Assert numerator < divisor when word offset is ignored.

    // D1. Normalize.
    assert!(numerator.last().unwrap().leading_zeros() >= divisor.last().unwrap().leading_zeros());
    let shift = divisor.last().unwrap().leading_zeros();
    if shift > 0 {
        for i in (1..numerator.len()).rev() {
            numerator[i] <<= shift;
            numerator[i] |= numerator[i - 1] >> (64 - shift);
        }
        numerator[0] <<= shift;
        for i in (1..divisor.len()).rev() {
            divisor[i] <<= shift;
            divisor[i] |= divisor[i - 1] >> (64 - shift);
        }
        divisor[0] <<= shift;
    }

    let n = divisor.len();
    let m = numerator.len() - n;

    // Allocate quotient.
    let mut quotient = vec![0u64; m + 1];

    // D2. Loop over quotient digits
    // TODO:
    for j in (0..m).rev() {
        // D3. Calculate approximate quotient word
        let mut qhat = div_3by2(
            &[numerator[j + n - 2], numerator[j + n - 1], numerator[j + n]],
            &[divisor[n - 2], divisor[n - 1]],
        );

        // D4. Multiply and subtract.
        let mut borrow = 0;
        for i in 0..n {
            let (a, b) = msb(numerator[j + i], qhat, divisor[i], borrow);
            numerator[j + i] = a;
            borrow = 0u64.wrapping_sub(b); // TODO: Why this inversion?
        }
        let negative = numerator[j + n] < borrow;
        numerator[j + n] = numerator[j + n].wrapping_sub(borrow);

        // D5. Test remainder for negative result.
        if negative {
            // D6. Add back. (happens rarely)
            let mut carry = 0;
            for i in 0..n {
                let (a, b) = adc(numerator[j + i], divisor[i], carry);
                numerator[j + i] = a;
                carry = b;
            }
            numerator[j + n] = numerator[j + n].wrapping_add(carry);
            qhat -= 1;
        }

        // Store remainder
        quotient[j] = qhat;
    }

    // D8. Unnormalize.
    if shift > 0 {
        for i in (0..numerator.len() - 1) {
            numerator[i] >>= shift;
            numerator[i] |= numerator[i + 1] << (64 - shift);
        }
        numerator[numerator.len() - 1] >>= shift;
    }

    quotient
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::u256::U256;
    use quickcheck_macros::quickcheck;

    const HALF: u64 = 1u64 << 63;

    #[test]
    fn div_3by2_max() {
        let q = div_3by2(&[u64::MAX, u64::MAX, HALF - 1], &[0, HALF]);
        assert_eq!(q, u64::MAX);
    }

    #[test]
    fn test_divrem_8by4() {
        let mut numerator = [
            0x9c2bcebfa9cca2c6u64,
            0x274e154bb5e24f7au64,
            0xe1442d5d3842be2bu64,
            0xf18f5adfd420853fu64,
            0x04ed6127eba3b594u64,
            0xc5c179973cdb1663u64,
            0x7d7f67780bb268ffu64,
            0x0000000000000003u64,
        ];
        let mut divisor = [
            0x0181880b078ab6a1u64,
            0x62d67f6b7b0bda6bu64,
            0x92b1840f9c792dedu64,
            0x0000000000000019u64,
        ];
        let expected_quotient = [
            0x9128464e61d6b5b3u64,
            0xd9eea4fc30c5ac6cu64,
            0x944a2d832d5a6a08u64,
            0x22f06722e8d883b1u64,
            0x0000000000000000u64,
        ];
        let expected_remainder = [
            0x1dfa5a7ea5191b33u64,
            0xb5aeb3f9ad5e294eu64,
            0xfc710038c13e4eedu64,
            0x000000000000000bu64,
            0x0000000000000000u64,
            0x0000000000000000u64,
            0x0000000000000000u64,
            0x0000000000000000u64,
        ];
        let quotient = divrem_nbym(&mut numerator, &mut divisor);
        let remainder = numerator;
        assert_eq!(quotient.len(), 5);
        assert_eq!(quotient, expected_quotient);
        assert_eq!(remainder, expected_remainder);
    }

    #[quickcheck]
    #[test]
    fn div_3by2_correct(q: u64, d0: u64, d1: u64) -> bool {
        let d1 = d1 | (1 << 63);
        let n = U256::new(d0, d1, 0, 0) * &U256::from(q);
        let qhat = div_3by2(&[n.c0, n.c1, n.c2], &[d0, d1]);
        qhat == q
    }
}
