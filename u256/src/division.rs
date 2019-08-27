use crate::utils::{adc, msb};
use core::u64;

const fn val_2(lo: u64, hi: u64) -> u128 {
    ((hi as u128) << 64) | (lo as u128)
}

const fn mul_2(a: u64, b: u64) -> u128 {
    (a as u128) * (b as u128)
}

/// Compute <hi, lo> / d, returning the quotient and the remainder.
// TODO: Make sure it uses divq on x86_64.
// See http://lists.llvm.org/pipermail/llvm-dev/2017-October/118323.html
// (Note that we require d > hi for this)
// TODO: If divq is not supported, use a fast software implementation:
// See https://gmplib.org/~tege/division-paper.pdf
fn divrem_2by1(lo: u64, hi: u64, d: u64) -> (u64, u64) {
    debug_assert!(d > 0);
    debug_assert!(d > hi);
    let d = u128::from(d);
    let n = val_2(lo, hi);
    let q = n / d;
    let r = n % d;
    debug_assert!(q < val_2(0, 1));
    debug_assert!(r < d);
    // There should not be any truncaction.
    #[allow(clippy::cast_possible_truncation)]
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
pub fn div_3by2(n: &[u64; 3], d: &[u64; 2]) -> u64 {
    // The highest bit of d needs to be set
    debug_assert!(d[1] >> 63 == 1);

    // The quotient needs to fit u64. For this we need <n2 n1> < <d1 d0>
    debug_assert!(n[2] < d[1] || (n[2] == d[1] && n[1] < d[0]));

    // Compute quotient and remainder
    // TODO: Use GMP's reciprocal computation.
    let (mut q, mut r) = divrem_2by1(n[1], n[2], d[1]);

    if mul_2(q, d[0]) > val_2(n[0], r) {
        q -= 1;
        r = r.wrapping_add(d[1]);
        let overflow = r < d[1];
        if !overflow && mul_2(q, d[0]) > val_2(n[0], r) {
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
// NOTE: numerator must have one additional zero at the end.
// The result will be computed in-place in numerator.
// The divisor will be normalized.
pub fn divrem_nbym(numerator: &mut [u64], divisor: &mut [u64]) {
    debug_assert!(divisor.len() >= 2);
    debug_assert!(numerator.len() > divisor.len());
    debug_assert!(*divisor.last().unwrap() > 0);
    debug_assert!(*numerator.last().unwrap() == 0);
    let n = divisor.len();
    let m = numerator.len() - n - 1;

    // D1. Normalize.
    let shift = divisor[n - 1].leading_zeros();
    if shift > 0 {
        numerator[n + m] = numerator[n + m - 1] >> (64 - shift);
        for i in (1..n + m).rev() {
            numerator[i] <<= shift;
            numerator[i] |= numerator[i - 1] >> (64 - shift);
        }
        numerator[0] <<= shift;
        for i in (1..n).rev() {
            divisor[i] <<= shift;
            divisor[i] |= divisor[i - 1] >> (64 - shift);
        }
        divisor[0] <<= shift;
    }

    // D2. Loop over quotient digits
    for j in (0..=m).rev() {
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
            borrow = b;
        }
        let negative = numerator[j + n] < borrow;
        // IGNORED: numerator[j + n] = numerator[j + n].wrapping_sub(borrow);

        // D5. Test remainder for negative result.
        if negative {
            // D6. Add back. (happens rarely)
            let mut carry = 0;
            for i in 0..n {
                let (a, b) = adc(numerator[j + i], divisor[i], carry);
                numerator[j + i] = a;
                carry = b;
            }
            // This should alwayst be zero, so we don't compute:
            // numerator[j + n] = numerator[j + n].wrapping_add(carry);
            qhat -= 1;
        }

        // Store remainder in the now vacant bits of numerator
        numerator[j + n] = qhat;
    }

    // D8. Unnormalize.
    if shift > 0 {
        // Make sure to only normalize the remainder part, the quotient
        // is alreadt normalized.
        for i in 0..(n - 1) {
            numerator[i] >>= shift;
            numerator[i] |= numerator[i + 1] << (64 - shift);
        }
        numerator[n - 1] >>= shift;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::u256::U256;
    use macros_decl::u256h;
    use quickcheck_macros::quickcheck;

    const HALF: u64 = 1_u64 << 63;

    #[test]
    fn div_3by2_max() {
        let q = div_3by2(&[u64::max_value(), u64::max_value(), HALF - 1], &[0, HALF]);
        assert_eq!(q, u64::max_value());
    }

    #[allow(clippy::unreadable_literal)]
    #[test]
    fn test_divrem_8by4() {
        let mut numerator = [
            0x9c2bcebfa9cca2c6_u64,
            0x274e154bb5e24f7a_u64,
            0xe1442d5d3842be2b_u64,
            0xf18f5adfd420853f_u64,
            0x04ed6127eba3b594_u64,
            0xc5c179973cdb1663_u64,
            0x7d7f67780bb268ff_u64,
            0x0000000000000003_u64,
            0x0000000000000000_u64,
        ];
        let mut divisor = [
            0x0181880b078ab6a1_u64,
            0x62d67f6b7b0bda6b_u64,
            0x92b1840f9c792ded_u64,
            0x0000000000000019_u64,
        ];
        let expected_quotient = [
            0x9128464e61d6b5b3_u64,
            0xd9eea4fc30c5ac6c_u64,
            0x944a2d832d5a6a08_u64,
            0x22f06722e8d883b1_u64,
            0x0000000000000000_u64,
        ];
        let expected_remainder = [
            0x1dfa5a7ea5191b33_u64,
            0xb5aeb3f9ad5e294e_u64,
            0xfc710038c13e4eed_u64,
            0x000000000000000b_u64,
        ];
        divrem_nbym(&mut numerator, &mut divisor);
        let remainder = &numerator[0..4];
        let quotient = &numerator[4..9];
        assert_eq!(remainder, expected_remainder);
        assert_eq!(quotient, expected_quotient);
    }

    #[test]
    fn test_divrem_4by4() {
        let a = u256h!("6f1480e63854afa41868b9a7d418e9c64edef514135f5899e72530a3d4e91ea3");
        let b = u256h!("3ba5ddaec5090ef0b87126f34ee28533ffb08af4108f9aeaa62b65900d2a62bb");
        let r = a.clone() - &b;
        let mut numerator = [a.c0, a.c1, a.c2, a.c3, 0];
        let mut divisor = [b.c0, b.c1, b.c2, b.c3];
        divrem_nbym(&mut numerator, &mut divisor);
        let remainder = &numerator[0..4];
        let quotient = numerator[4];
        assert_eq!(remainder, [r.c0, r.c1, r.c2, r.c3]);
        assert_eq!(quotient, 1);
    }

    #[quickcheck]
    fn div_3by2_correct(q: u64, d0: u64, d1: u64) -> bool {
        let d1 = d1 | (1 << 63);
        let n = U256::from_limbs(d0, d1, 0, 0) * &U256::from(q);
        let qhat = div_3by2(&[n.c0, n.c1, n.c2], &[d0, d1]);
        qhat == q
    }
}
