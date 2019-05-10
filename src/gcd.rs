use crate::u256::U256;

#[inline(always)]
fn euclid_step(
    a: U256,
    b: U256,
    data: (U256, U256, U256, U256),
) -> (U256, U256, (U256, U256, U256, U256)) {
    let (q, rem) = a.divrem(&b).unwrap();
    let hold1 = &data.0 + &data.1 * &q;
    let hold2 = &data.2 + &data.3 * q;
    (b, rem, (data.1, hold1, data.3, hold2))
}

pub fn gcd_euclid(a: &U256, b: &U256) -> (U256, U256, U256, bool) {
    let mut a_prime;
    let mut b_prime;

    if b > a {
        //Note : Alg assumes a >= b, and gcd(a,b) = gcd(b,a)
        a_prime = b.clone(); //Gets correct ordering of mutable data
        b_prime = a.clone();
    } else {
        a_prime = a.clone(); //Gets correct ordering of mutable data
        b_prime = b.clone();
    }

    let mut consquences = (U256::ONE, U256::ZERO, U256::ZERO, U256::ONE);
    let mut even = true;

    while b_prime != U256::ZERO {
        let (hold1, hold2, hold3) = euclid_step(a_prime, b_prime, consquences);
        a_prime = hold1;
        b_prime = hold2;
        consquences = hold3;
        even = !even;
    }
    (a_prime, consquences.0, consquences.2, even)
}

/// Division optimized for small values
/// Requires a > b > 0. Returns a / b.
#[inline(always)]
fn div1(a: u64, b: u64) -> u64 {
    let mut r = a - b;
    if r < b {
        1
    } else {
        r -= b;
        if r < b {
            2
        } else {
            r -= b;
            if r < b {
                3
            } else {
                r -= b;
                if r < b {
                    4
                } else {
                    r -= b;
                    if r < b {
                        5
                    } else {
                        r -= b;
                        if r < b {
                            6
                        } else {
                        r -= b;
                        if r < b {
                            7
                        } else {
                        r -= b;
                        if r < b {
                            8
                        } else {
                        r -= b;
                        if r < b {
                            9
                        } else {
                        r -= b;
                        if r < b {
                            10
                        } else {
                        r -= b;
                        if r < b {
                            11
                        } else {
                            a / b
                        }
                        }
                        }
                        }
                        }
                        }
                    }
                }
            }
        }
    }
}

/// Compute the Lehmer update matrix for small values.
/// This is essentialy Euclids extended GCD algorithm for 64 bits.
/// OPT: Would this be faster using extended binary gcd?
#[inline(never)]
fn lehmer_small(mut r0: u64, mut r1: u64) -> (u64, u64, u64, u64, bool) {
    let mut q00 = 1u64;
    let mut q01 = 0u64;
    let mut q10 = 0u64;
    let mut q11 = 1u64;
    loop {
        // Loop is unrolled once to avoid swapping variables and tracking parity.
        if r1 == 0u64 {
            return (q00, q01, q10, q11, true);
        }
        let q = if r0 < r1 { 0 } else { div1(r0, r1) };
        r0 -= q * r1;
        q00 += q * q10;
        q01 += q * q11;
        if r0 == 0u64 {
            return (q10, q11, q00, q01, false);
        }
        let q = div1(r1, r0);
        r1 -= q * r0;
        q10 += q * q00;
        q11 += q * q01;
    }
}

/// Compute the Lehmer update matrix for the most significant 64-bits of r0 and r1.
/// OPT: Would a variation of binary gcd apply here and be faster?
/// OPT: Use a division optimized for small quotients, like in GMPs
///      https://github.com/ryepdx/gmp/blob/master/mpn/generic/hgcd2.c
#[inline(never)]
fn lehmer_loop(
    mut r0: u64,
    mut r1: u64,
    mut q00: u64,
    mut q01: u64,
    mut q10: u64,
    mut q11: u64,
) -> (u64, u64, u64, u64, bool) {
    const LIMIT: u64 = 1u64 << 32;
    if r1 == 0u64 {
        return (q00, q01, q10, q11, true);
    }
    // The r values are one step ahead of the q values so we can test the stopping condition.
    // FAILS: debug_assert!(r1 >= r0);
    // let mut q0 = div1(r0, r1);
    let mut q0 = if r0 < r1 { 0 } else { div1(r0, r1) };
    r0 -= q0 * r1;
    if r0 < LIMIT {
        return (q00, q01, q10, q11, true);
    }
    loop {
        // Loop is unrolled once to avoid swapping variables and tracking parity.
        // OPT: Unroll into subtraction only rounds
        let q1 = div1(r1, r0);
        r1 -= q1 * r0;
        if r1 < LIMIT {
            return (q00, q01, q10, q11, true);
        }
        q00 += q0 * q10;
        q01 += q0 * q11;

        // Repeat with indices 0 and 1 flipped
        q0 = div1(r0, r1);
        r0 -= q0 * r1;
        if r0 < LIMIT {
            return (q10, q11, q00, q01, false);
        }
        q10 += q1 * q00;
        q11 += q1 * q01;
    }
}

/// Compute the Lehmer update matrix using double words
/// See https://github.com/ryepdx/gmp/blob/090b098806bc1a8f3af777b862369f58be465dd9/mpn/generic/hgcd2.c#L226
#[inline(never)]
fn lehmer_double(mut r0: U256, mut r1: U256) -> (u64, u64, u64, u64, bool) {
    debug_assert!(r0 >= r1);
    if r0.bits() < 64 {
        return lehmer_small(r0.c0, r1.c0);
    }
    let s = r0.leading_zeros();
    let r0s = r0.clone() << s;
    let r1s = r1.clone() << s;
    let q = lehmer_loop(r0s.c3, r1s.c3, 1, 0, 0, 1);
    // We can return q here and have a perfectly valid single-word Lehmer GCD.
    // return q;
    if q.2 == 0u64 {
        return q;
    }

    // Recompute r0 and r1 and take the high bits.
    // OPT: This does not need full precision.
    // OPT: Can we reuse the shifted variables here?
    // TODO: Should we use lehmer_small here when r0 is one word?
    lehmer_update(&mut r0, &mut r1, q);
    let s = r0.leading_zeros();
    let r0s = r0.clone() << s;
    let r1s = r1.clone() << s;
    let mut qn = lehmer_loop(r0s.c3, r1s.c3, q.0, q.1, q.2, q.3);
    qn.4 ^= !q.4;
    qn
}

#[inline(never)]
fn lehmer_update(
    a0: &mut U256,
    a1: &mut U256,
    (q00, q01, q10, q11, even): (u64, u64, u64, u64, bool),
) {
    // OPT: Inplace clone-free multiply, like GMP's addaddmul_1msb
    if even {
        let b0 = q00 * a0.clone() - q01 * a1.clone();
        let b1 = q11 * a1.clone() - q10 * a0.clone();
        *a0 = b0;
        *a1 = b1;
    } else {
        let b0 = q01 * a1.clone() - q00 * a0.clone();
        let b1 = q10 * a0.clone() - q11 * a1.clone();
        *a0 = b0;
        *a1 = b1;
    }
}

#[rustfmt::skip]
pub fn gcd_lehmer(mut r0: U256, mut r1: U256) -> (U256, U256, U256, bool) {
    debug_assert!(r0 >= r1);
    // TODO: Support r1 >= r0
    let mut s0 = U256::ONE;
    let mut s1 = U256::ZERO;
    let mut t0 = U256::ZERO;
    let mut t1 = U256::ONE;
    let mut even = true;
    while r1 != U256::ZERO {
        let q = lehmer_double(r0.clone(), r1.clone());
        if q.2 != 0u64 {
            lehmer_update(&mut r0, &mut r1, q);
            lehmer_update(&mut s0, &mut s1, q);
            lehmer_update(&mut t0, &mut t1, q);
            even ^= !q.4;
        } else {
            // Do a full precision Euclid step. q is at least a halfword.
            // This should happen zero or one time, seldom more.
            // OPT: use single limb version when q is small enough?
            let q = &r0 / &r1;
            let t = r0 - &q * &r1; r0 = r1; r1 = t;
            let t = s0 - &q * &s1; s0 = s1; s1 = t;
            let t = t0 -  q * &t1; t0 = t1; t1 = t;
            even = !even;
        }
    }
    // TODO: Compute using absolute value instead of patching sign.
    if even {
        // t negative
        t0 = U256::ZERO - t0;
    } else {
        // s negative
        s0 = U256::ZERO - s0;
    }
    (r0, s0, t0, even)
}

#[rustfmt::skip]
pub fn inv_lehmer(modulus: &U256, num: &U256) -> Option<U256> {
    debug_assert!(modulus > num);
    let mut r0 = modulus.clone();
    let mut r1 = num.clone();
    let mut t0 = U256::ZERO;
    let mut t1 = U256::ONE;
    let mut even = true;
    while r1 != U256::ZERO {
        let q = lehmer_double(r0.clone(), r1.clone());
        if q.2 != 0u64 {
            lehmer_update(&mut r0, &mut r1, q);
            lehmer_update(&mut t0, &mut t1, q);
            even ^= !q.4;
        } else {
            // Do a full precision Euclid step. q is at least a halfword.
            // This should happen zero or one time, seldom more.
            // OPT: use single limb version when q is small enough?
            let q = &r0 / &r1;
            let t = r0 - &q * &r1; r0 = r1; r1 = t;
            let t = t0 -  q * &t1; t0 = t1; t1 = t;
            even = !even;
        }
    }
    if r0 == U256::ONE {
        // When `even` t0 is negative and in twos-complement form
        Some(if even { modulus + t0 } else { t0 })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unreadable_litteral)]

    use super::*;
    use crate::field::{FieldElement, MODULUS};
    use crate::u256h;
    use hex_literal::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_lehmer_small() {
        assert_eq!(lehmer_small(0, 0), (1, 0, 0, 1, true));
        assert_eq!(lehmer_small(0, 1), (0, 1, 1, 0, false));
        assert_eq!(
            lehmer_small(5818365597666026993, 14535145444257436950),
            (
                947685836737753349,
                379355176803460069,
                2076449349179633850,
                831195085380860999,
                true
            )
        );
        assert_eq!(
            lehmer_small(10841422679839906593, 15507080595343815048),
            (
                57434639988632077,
                40154122160696118,
                5169026865114605016,
                3613807559946635531,
                false
            )
        );
    }

    #[test]
    fn test_lehmer_loop() {
        assert_eq!(lehmer_loop(0, 0, 1, 0, 0, 1), (1, 0, 0, 1, true));
        assert_eq!(
            lehmer_loop(5818365597666026993, 14535145444257436950, 1, 0, 0, 1),
            (139667543, 55908407, 174687518, 69926775, false)
        );
        assert_eq!(
            lehmer_loop(
                6044159827974199924,
                6325623274722585764,
                4189569209,
                21585722,
                1706813914,
                1897815210
            ),
            (
                1130534579495951597,
                356413338079229448,
                1604599888673401540,
                505867589524443154,
                false
            )
        );
    }

    #[test]
    fn test_lehmer_double() {
        assert_eq!(lehmer_double(U256::ZERO, U256::ZERO), (1, 0, 0, 1, true));
        assert_eq!(
            lehmer_double(
                u256h!("518a5cc4c55ac5b050a0831b65e827e5e39fd4515e4e094961c61509e7870814"),
                u256h!("018a5cc4c55ac5b050a0831b65e827e5e39fd4515e4e094961c61509e7870814")
            ),
            (
                24753544726280,
                1310252935479731,
                64710401929971,
                3425246566597885,
                false
            )
        );
    }

    #[test]
    fn test_gcd_lehmer() {
        assert_eq!(
            gcd_lehmer(U256::ZERO, U256::ZERO),
            (U256::ZERO, U256::ONE, U256::ZERO, true)
        );
        assert_eq!(
            gcd_lehmer(
                u256h!("518a5cc4c55ac5b050a0831b65e827e5e39fd4515e4e094961c61509e7870814"),
                u256h!("018a5cc4c55ac5b050a0831b65e827e5e39fd4515e4e094961c61509e7870814")
            ),
            (
                U256::from(4u64),
                u256h!("002c851a0dddfaa03b9db2e39d48067d9b57fa0d238b70c7feddf8d267accc41"),
                u256h!("0934869c752ae9c7d2ed8aa55e7754e5492aaac49f8c9f3416156313a16c1174"),
                true
            )
        );
        assert_eq!(
            gcd_lehmer(
                u256h!("7dfd26515f3cd365ea32e1a43dbac87a25d0326fd834a889cb1e4c6c3c8d368c"),
                u256h!("3d341ef315cbe5b9f0ab79255f9684e153deaf5f460a8425819c84ec1e80a2f3")
            ),
            (
                u256h!("0000000000000000000000000000000000000000000000000000000000000001"),
                u256h!("0bbc35a0c1fd8f1ae85377ead5a901d4fbf0345fa303a87a4b4b68429cd69293"),
                u256h!("18283a24821b7de14cf22afb0e1a7efb4212b7f373988f5a0d75f6ee0b936347"),
                false
            )
        );
        assert_eq!(
            gcd_lehmer(
                u256h!("836fab5d425345751b3425e733e8150a17fdab2d5fb840ede5e0879f41497a4f"),
                u256h!("196e875b381eb95d9b5c6c3f198c5092b3ccc21279a7e68bc42cb6bca2d2644d")
            ),
            (
                u256h!("000000000000000000000000000000000000000000000000c59f8490536754fd"),
                u256h!("000000000000000006865401d85836d50a2bd608f152186fb24072a122d0dc5d"),
                u256h!("000000000000000021b8940f60792f546cbeb17f8b852d33a00b14b323d6de70"),
                false
            )
        );
        assert_eq!(
            gcd_lehmer(
                u256h!("00253222ed7b612113dbea0be0e1a0b88f2c0c16250f54bf1ec35d62671bf83a"),
                u256h!("0000000000025d4e064960ef2964b2170f1cd63ab931968621dde8a867079fd4")
            ),
            (
                u256h!("000000000000000000000000000505b22b0a9fd5a6e2166e3486f0109e6f60b2"),
                u256h!("0000000000000000000000000000000000000000000000001f16d40433587ae9"),
                u256h!("0000000000000000000000000000000000000001e91177fbec66b1233e79662e"),
                true
            )
        );
    }
}
