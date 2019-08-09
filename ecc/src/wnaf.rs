use crate::{curve::Affine, jacobian::Jacobian};
use itertools::izip;
use primefield::FieldElement;
use u256::U256;

pub fn window_table(p: &Affine, naf: &mut [Jacobian]) {
    // naf = P, 3P, 5P, ... 15P
    // OPT: Optimal window size
    naf[0] = Jacobian::from(p);
    let p2 = naf[0].double();
    for i in 1..naf.len() {
        naf[i] = &naf[i - 1] + &p2;
    }
    // OPT: Use batch inversion to convert to Affine
}

pub fn window_table_affine(p: &Affine, naf: &mut [Affine]) {
    // naf = P, 3P, 5P, ... 15P
    // OPT: Optimal window size
    naf[0] = p.clone();
    let p2 = naf[0].double();
    for i in 1..naf.len() {
        naf[i] = &naf[i - 1] + &p2;
    }
}

// TODO: https://link.springer.com/content/pdf/10.1007/3-540-36400-5_41.pdf
pub fn batch_convert(jacobians: &[Jacobian], affines: &mut [Affine]) {
    debug_assert!(jacobians.len() == affines.len());

    // Intermediate values
    let mut vals = vec![FieldElement::ONE; jacobians.len()];

    // Accumulate all z values
    let mut acc = FieldElement::ONE;
    // OPT: Check if `izip!` has overhead
    for (jac, val) in izip!(jacobians.iter(), vals.iter_mut()) {
        // TODO: Handle zeros
        // OPT: First mul is with one, can be removed
        *val = acc.clone();
        acc *= &jac.z;
    }

    // Invert accumulator
    // OPT: inv_assign
    acc = acc.inv().unwrap();

    // Compute inverses and affine points
    for (jac, val, aff) in izip!(jacobians.iter(), vals.into_iter(), affines.iter_mut()).rev() {
        // Compute zi
        // OPT: Last mul is with one, can be removed
        let zi = &acc * val;
        acc *= &jac.z;

        // Compute affine point
        let zi2 = zi.square();
        let zi3 = zi * &zi2;
        *aff = Affine::Point {
            x: &jac.x * zi2,
            y: &jac.y * zi3,
        }
    }
}

// Convert scalar to naf form
// See https://doc-internal.dalek.rs/curve25519_dalek/scalar/struct.Scalar.html#method.non_adjacent_form
// See https://github.com/dalek-cryptography/curve25519-dalek/blob/ca6305232f08ced340819b8ae691f90492d1b054/src/scalar.rs#L872
// Algorithm 3.35 of Guide to Elliptic Curve Cryptography
// OPT: Can we turn this into a left-to-right version of the algorithm
//      so we can consume the values as they are produced and we don't
//      need any allocations?
pub fn non_adjacent_form(mut scalar: U256, window: usize) -> [i16; 257] {
    let mask = (1u64 << window) - 1;
    let half = 1i16 << (window - 1);
    let mut snaf = [0i16; 257];
    let mut i: usize = 0;
    loop {
        // Shift to next set bit (and hence make k odd)
        match scalar.trailing_zeros() {
            0 => {}
            256 => break,
            shift => {
                scalar >>= shift;
                i += shift;
            }
        }

        // Extract window and shift W buts
        let mut n: i16 = (scalar.c0 & mask) as i16;
        scalar >>= window;

        // Make negative if n > 2^(w-1)
        if n >= half {
            n -= 1i16 << window;
            scalar += U256::ONE;
        }

        // Store and advance index
        snaf[i] = n;
        i += window;
    }
    snaf
}

// Multiply Affine point using Jacobian accumulator
// See https://doc-internal.dalek.rs/curve25519_dalek/traits/trait.VartimeMultiscalarMul.html
pub fn mul(p: &Affine, scalar: &U256) -> Jacobian {
    // Precomputed odd multiples
    let mut naf: [Jacobian; 8] = Default::default();
    window_table(p, &mut naf);

    // Get SNAF
    let snaf = non_adjacent_form(scalar.clone(), 5);

    // Algorithm 3.36 of Guide to Elliptic Curve Cryptography
    let mut r = Jacobian::ZERO;
    for i in (0..snaf.len()).rev() {
        // OPT: Avoid doubling zeros
        // OPT: Use A + A -> J formular for first
        r.double_assign();
        if snaf[i] > 0 {
            r += &naf[(snaf[i] >> 1) as usize];
        } else if snaf[i] < 0 {
            r -= &naf[(-snaf[i] >> 1) as usize];
        }
    }
    r
}

pub fn double_mul(pa: &Affine, sa: U256, pb: &Affine, sb: U256) -> Jacobian {
    // Precomputed odd multiples
    let mut nafa: [Jacobian; 8] = Default::default();
    let mut nafb: [Jacobian; 8] = Default::default();
    window_table(pa, &mut nafa);
    window_table(pb, &mut nafb);

    // Get SNAF
    let snafa = non_adjacent_form(sa, 5);
    let snafb = non_adjacent_form(sb, 5);

    // Algorithm 3.36 of Guide to Elliptic Curve Cryptography
    let mut r = Jacobian::ZERO;
    for i in (0..snafa.len()).rev() {
        // OPT: Avoid doubling zeros
        // OPT: Use A + A -> J formular for first
        r.double_assign();
        if snafa[i] > 0 {
            r += &nafa[(snafa[i] >> 1) as usize];
        } else if snafa[i] < 0 {
            r -= &nafa[(-snafa[i] >> 1) as usize];
        }
        if snafb[i] > 0 {
            r += &nafb[(snafb[i] >> 1) as usize];
        } else if snafb[i] < 0 {
            r -= &nafb[(-snafb[i] >> 1) as usize];
        }
    }
    r
}

pub fn base_mul(naf: &[Affine], s: U256) -> Jacobian {
    // Get SNAF
    let snaf = non_adjacent_form(s, 7);

    // Algorithm 3.36 of Guide to Elliptic Curve Cryptography
    let mut r = Jacobian::ZERO;
    for i in (0..snaf.len()).rev() {
        // OPT: Avoid doubling zeros
        // OPT: Use A + A -> J formular for first
        r.double_assign();
        if snaf[i] > 0 {
            r += &naf[(snaf[i] >> 1) as usize];
        } else if snaf[i] < 0 {
            r -= &naf[(-snaf[i] >> 1) as usize];
        }
    }
    r
}

pub fn double_base_mul(nafa: &[Affine], sa: U256, pb: &Affine, sb: U256) -> Jacobian {
    // Precomputed odd multiples
    let mut nafb: [Jacobian; 8] = Default::default();
    window_table(pb, &mut nafb);

    // Batch convert to affine
    // OPT: Right now this doesn't hurt or improve performance. It should be
    // better with more points.
    let mut naf: [Affine; 8] = Default::default();
    batch_convert(&nafb, &mut naf);
    let nafb = naf;

    // Get SNAF
    let snafa = non_adjacent_form(sa, 7);
    let snafb = non_adjacent_form(sb, 5);

    // Algorithm 3.36 of Guide to Elliptic Curve Cryptography
    let mut r = Jacobian::ZERO;
    for i in (0..snafa.len()).rev() {
        // OPT: Avoid doubling zeros
        // OPT: Use A + A -> J formular for first
        r.double_assign();
        if snafa[i] > 0 {
            r += &nafa[(snafa[i] >> 1) as usize];
        } else if snafa[i] < 0 {
            r -= &nafa[(-snafa[i] >> 1) as usize];
        }
        if snafb[i] > 0 {
            r += &nafb[(snafb[i] >> 1) as usize];
        } else if snafb[i] < 0 {
            r -= &nafb[(-snafb[i] >> 1) as usize];
        }
    }
    r
}

// TODO: https://github.com/dalek-cryptography/curve25519-dalek/blob/8b2742cb9dae6a365915021ac7474227d610f09a/src/backend/vector/scalar_mul/vartime_double_base.rs

// TODO: Replace literals with u256h!
#[allow(clippy::unreadable_literal)]
#[cfg(test)]
mod tests {
    use super::*;
    use macros_decl::u256h;
    use primefield::FieldElement;

    #[test]
    fn test_mul() {
        let p = Affine::Point {
            x: FieldElement::from(u256h!(
                "01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca"
            )),
            y: FieldElement::from(u256h!(
                "005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f"
            )),
        };
        let c = u256h!("07374b7d69dc9825fc758b28913c8d2a27be5e7c32412f612b20c9c97afbe4dd");
        let expected = Jacobian::from(Affine::Point {
            x: FieldElement::from(u256h!(
                "00f24921907180cd42c9d2d4f9490a7bc19ac987242e80ac09a8ac2bcf0445de"
            )),
            y: FieldElement::from(u256h!(
                "018a7a2ab4e795405f924de277b0e723d90eac55f2a470d8532113d735bdedd4"
            )),
        });
        let result = mul(&p, &c);
        assert_eq!(result, expected);
    }
}
