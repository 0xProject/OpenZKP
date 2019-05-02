// use crate::u256::U256;
// use crate::field::{FieldElement, MODULUS};
// use crate::montgomery::R2;
// use std::mem::swap;

// pub fn gcd_euler(n: U256) -> Option<U256> {
//     if n == U256::ZERO {
//         return None;
//     }
//     let mut a0 = MODULUS;
//     let mut a1 = n;
//     // OPT: R2 for montgomery version (or free mul?)
//     let mut v0 = U256::ZERO;
//     let mut v1 = U256::ONE;
//     debug_assert!(a1 < a0);

//     while a1 > U256::ZERO {
//         let (q, _) = a0.divrem(&a1).unwrap();
//         // Store new values in _0
//         a0 -= &q * &a1;
//         v0 += &q * &v1;
//         swap(&mut a0, &mut a1); // OPT: Unroll once to avoid swap
//         swap(&mut v0, &mut v1);
//     }
//     // TODO: Distinguish odd and even for positive/negative.
//     // (Can be handled by once unrolling)
//     Some(MODULUS - v0)
// }

// // Computes a one-limb gcd step matrix efficiently
// // See Jebelean 1994 "A Double-Digit Lehmer-Euclid Algorithm for Finding the GCD of Long Integers"
// // See "Handbook of Elliptic and Hyperelliptic Curves" Algorithm 10.46
// pub fn lehmer_gcd_approximate(a: &U256, b: &U256) -> (u64, u64, u64, u64, bool) {
//     debug_assert!(a > b);
//     debug_assert!(b > U256::ZERO);
//     let shift = a.leading_zeros();
//     let mut a0 = (a.clone() << shift).c3;
//     let mut a1 = (b.clone() << shift).c3;
//     if a1 == 0 {
        
//     }
//     let mut u0 = 1u64;
//     let mut u1 = 0u64;
//     let mut v0 = 0u64;
//     let mut v1 = 1u64;
//     let mut even = false;
//     println!("{:016x?} {:016x?} {:016x?}", a0, u0, v0);
//     println!("{:016x?} {:016x?} {:016x?}", a1, u1, v1);

//     loop {
//         // Compute new values
//         let q = a0h / a1h;
//         let a2 = a0 - q * a1;
//         let u2 = u0 + q * u1;
//         let v2 = v0 + q * v1;
//         even = !even;
//         println!("{:016x?} {:016x?} {:016x?}", a2, u2, v2);

//         // Collins stopping condition
//         if !(a2 >= v2 && a1 - a2 >= v1 + v2) { // TODO: v1 - v2?
//             break;
//         }

//         // Keep new values
//         a0 = a1; a1 = a2;
//         u0 = u1; u1 = u2;
//         v0 = v1; v1 = v2;
//     }

//     (u0, u1, v0, v1, even)
// }

// pub fn lehmer_update(&mut a0, &mut a1, &(u0, u1, v0, v1, even): (u64, u64, u64, u64, bool)) {
//     // TODO: Result may be negative
//     let (a0n, a1n) = if even {
//         (u0h * &a0 - v0h * &a1, a1n = v1h * a1 - u1h * a0)
//     } else {
//         (v0h * &a1 - u0h * &a0, u1h * a0 - v1h * a1)
//     }
//     a0 = a0n;
//     a1 = a1n;
// }

// // See "Handbook of Elliptic and Hyperelliptic Curves" Algorithm 10.45
// pub fn gcd_lehmer(n: U256) -> (U256, U256, U256) {
//     if n == U256::ZERO {
//         return None;
//     }
//     let mut a0 = MODULUS;
//     let mut a1 = n;
//     let mut u0 = U256::ONE; // OPT: Remove u
//     let mut u1 = U256::ZERO;
//     // OPT: Initialize with R2 for montgomery version (or free mul?)
//     let mut v0 = U256::ZERO;
//     let mut v1 = U256::ONE;
//     debug_assert!(a1 < a0);

//     while a1.bits() > 64 {
//         let m = lehmer_gcd_approximate(a0, a1);
//         lehmer_update(&mut a0, &mut a1, m);
//         lehmer_update(&mut u0, &mut u1, m);
//         lehmer_update(&mut v0, &mut v1, m);
//         println!("{:?}", a0);
//         println!("{:?}", a1);
//     }
    

//     (v0, v1, a1)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::u256::U256;
//     use crate::u256h;
//     use hex_literal::*;
//     use quickcheck_macros::quickcheck;

//     #[test]
//     fn test_gcd_inv()
//     {
//         let n = u256h!("018a5cc4c55ac5b050a0831b65e827e5e39fd4515e4e094961c61509e7870814");
//         let expected = u256h!("0713ccbc2d1786937a9854a7c169625681304f782ec8426660f1ba26988fc815");
//         let result = gcd_euler(n).unwrap();
//         assert_eq!(result, expected)
//     }

//     #[test]
//     fn test_gcd_lehmer_inv()
//     {
//         let n = u256h!("018a5cc4c55ac5b050a0831b65e827e5e39fd4515e4e094961c61509e7870814");
//         let expected = u256h!("0713ccbc2d1786937a9854a7c169625681304f782ec8426660f1ba26988fc815");
//         let result = gcd_lehmer(n).3;
//         assert_eq!(result, expected)
//     }
// }
