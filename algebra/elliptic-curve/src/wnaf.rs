// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

use crate::{curve::Affine, jacobian::Jacobian, ScalarFieldElement};
use itertools::izip;
use zkp_primefield::{FieldElement, Inv, One, SquareInline};
use zkp_u256::{Binary, U256};

pub(crate) fn window_table(p: &Affine, naf: &mut [Jacobian]) {
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
pub(crate) fn batch_convert(jacobians: &[Jacobian], affines: &mut [Affine]) {
    debug_assert!(jacobians.len() == affines.len());

    // Intermediate values
    let mut vals = vec![FieldElement::one(); jacobians.len()];

    // Accumulate all z values
    let mut acc = FieldElement::one();
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
pub(crate) fn non_adjacent_form(scalar: &ScalarFieldElement, window: usize) -> [i16; 257] {
    let mut scalar = scalar.to_uint();
    let mask = (1_u64 << window) - 1;
    let half = 1_i16 << (window - 1);
    let mut snaf = [0_i16; 257];
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
        // The mask prevents truncations
        #[allow(clippy::cast_possible_truncation)]
        let mut n: i16 = (scalar.limb(0) & mask) as i16;
        scalar >>= window;

        // Make negative if n > 2^(w-1)
        if n >= half {
            n -= 1_i16 << window;
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
// Signs are explicitly handled
#[allow(clippy::cast_sign_loss)]
#[must_use]
// TODO: [refactor] [beginner] [small] rewrite
#[allow(clippy::comparison_chain)]
pub fn mul(p: &Affine, scalar: &ScalarFieldElement) -> Jacobian {
    // Precomputed odd multiples
    let mut naf_table: [Jacobian; 8] = Default::default();
    window_table(p, &mut naf_table);

    // Get SNAF
    let snaf_expansion = non_adjacent_form(scalar, 5);

    // Algorithm 3.36 of Guide to Elliptic Curve Cryptography
    let mut r = Jacobian::ZERO;
    for i in (0..snaf_expansion.len()).rev() {
        // OPT: Avoid doubling zeros
        // OPT: Use A + A -> J formula for first
        r.double_assign();
        if snaf_expansion[i] > 0 {
            r += &naf_table[(snaf_expansion[i] >> 1) as usize];
        } else if snaf_expansion[i] < 0 {
            r -= &naf_table[(-snaf_expansion[i] >> 1) as usize];
        }
    }
    r
}

// Signs are explicitly handled
#[allow(clippy::cast_sign_loss)]
#[must_use]
// TODO: [refactor] [beginner] [small] rewrite
#[allow(clippy::comparison_chain)]
pub fn double_mul(
    point_a: &Affine,
    scalar_a: &ScalarFieldElement,
    point_b: &Affine,
    scalar_b: &ScalarFieldElement,
) -> Jacobian {
    // Precomputed odd multiples
    let mut naf_table_a: [Jacobian; 8] = Default::default();
    let mut naf_table_b: [Jacobian; 8] = Default::default();
    window_table(point_a, &mut naf_table_a);
    window_table(point_b, &mut naf_table_b);

    // Get SNAF
    let snaf_expansion_a = non_adjacent_form(scalar_a, 5);
    let snaf_expansion_b = non_adjacent_form(scalar_b, 5);

    // Algorithm 3.36 of Guide to Elliptic Curve Cryptography
    let mut r = Jacobian::ZERO;
    debug_assert_eq!(snaf_expansion_a.len(), snaf_expansion_b.len());
    for i in (0..snaf_expansion_a.len()).rev() {
        // OPT: Avoid doubling zeros
        // OPT: Use A + A -> J formular for first
        r.double_assign();
        if snaf_expansion_a[i] > 0 {
            r += &naf_table_a[(snaf_expansion_a[i] >> 1) as usize];
        } else if snaf_expansion_b[i] < 0 {
            r -= &naf_table_a[(-snaf_expansion_a[i] >> 1) as usize];
        }
        if snaf_expansion_b[i] > 0 {
            r += &naf_table_b[(snaf_expansion_b[i] >> 1) as usize];
        } else if snaf_expansion_b[i] < 0 {
            r -= &naf_table_b[(-snaf_expansion_b[i] >> 1) as usize];
        }
    }
    r
}

// Signs are explicitly handled
#[allow(clippy::cast_sign_loss)]
// TODO: [refactor] [beginner] [small] rewrite
#[allow(clippy::comparison_chain)]
#[must_use]
pub fn base_mul(naf_table: &[Affine], scalar: &ScalarFieldElement) -> Jacobian {
    // Get SNAF
    let snaf_expansion = non_adjacent_form(scalar, 7);

    // Algorithm 3.36 of Guide to Elliptic Curve Cryptography
    let mut r = Jacobian::ZERO;
    for i in (0..snaf_expansion.len()).rev() {
        // OPT: Avoid doubling zeros
        // OPT: Use A + A -> J formular for first
        r.double_assign();
        if snaf_expansion[i] > 0 {
            r += &naf_table[(snaf_expansion[i] >> 1) as usize];
        } else if snaf_expansion[i] < 0 {
            r -= &naf_table[(-snaf_expansion[i] >> 1) as usize];
        }
    }
    r
}

// Signs are explicitly handled
#[allow(clippy::cast_sign_loss)]
// Rebind naf_table after affine batch conversion
#[allow(clippy::shadow_unrelated)]
// TODO: [refactor] [beginner] [small] rewrite
#[allow(clippy::comparison_chain)]
#[must_use]
pub fn double_base_mul(
    naf_table_a: &[Affine],
    scalar_a: &ScalarFieldElement,
    point_b: &Affine,
    scalar_b: &ScalarFieldElement,
) -> Jacobian {
    // Precomputed odd multiples
    let mut naf_table_b: [Jacobian; 8] = Default::default();
    window_table(point_b, &mut naf_table_b);

    // Batch convert to affine
    // OPT: Right now this doesn't hurt or improve performance. It should be
    // better with more points.
    let mut temp: [Affine; 8] = Default::default();
    batch_convert(&naf_table_b, &mut temp);
    let naf_table_b = temp;

    // Get SNAF
    let snaf_expansion_a = non_adjacent_form(scalar_a, 7);
    let snaf_expansion_b = non_adjacent_form(scalar_b, 5);

    // Algorithm 3.36 of Guide to Elliptic Curve Cryptography
    let mut r = Jacobian::ZERO;
    debug_assert_eq!(snaf_expansion_a.len(), snaf_expansion_b.len());
    for i in (0..snaf_expansion_a.len()).rev() {
        // OPT: Avoid doubling zeros
        // OPT: Use A + A -> J formula for first
        r.double_assign();
        if snaf_expansion_a[i] > 0 {
            r += &naf_table_a[(snaf_expansion_a[i] >> 1) as usize];
        } else if snaf_expansion_a[i] < 0 {
            r -= &naf_table_a[(-snaf_expansion_a[i] >> 1) as usize];
        }
        if snaf_expansion_b[i] > 0 {
            r += &naf_table_b[(snaf_expansion_b[i] >> 1) as usize];
        } else if snaf_expansion_b[i] < 0 {
            r -= &naf_table_b[(-snaf_expansion_b[i] >> 1) as usize];
        }
    }
    r
}

// TODO: https://github.com/dalek-cryptography/curve25519-dalek/blob/8b2742cb9dae6a365915021ac7474227d610f09a/src/backend/vector/scalar_mul/vartime_double_base.rs

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::{field_element, u256h};
    use zkp_primefield::FieldElement;

    #[test]
    fn test_mul() {
        let p = Affine::new(
            field_element!("01ef15c18599971b7beced415a40f0c7deacfd9b0d1819e03d723d8bc943cfca"),
            field_element!("005668060aa49730b7be4801df46ec62de53ecd11abe43a32873000c36e8dc1f"),
        );
        let c = ScalarFieldElement::from(u256h!(
            "07374b7d69dc9825fc758b28913c8d2a27be5e7c32412f612b20c9c97afbe4dd"
        ));
        let expected = Jacobian::from(Affine::new(
            field_element!("00f24921907180cd42c9d2d4f9490a7bc19ac987242e80ac09a8ac2bcf0445de"),
            field_element!("018a7a2ab4e795405f924de277b0e723d90eac55f2a470d8532113d735bdedd4"),
        ));
        let result = mul(&p, &c);
        assert_eq!(result, expected);
    }
}
