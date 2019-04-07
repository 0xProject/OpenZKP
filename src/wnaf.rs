use crate::curve::Affine;
use crate::jacobian::Jacobian;
use crate::u256::U256;

const W: usize = 5;
const MASK: u64 = ((1 << W) - 1);
const WINDOW: i8 = 1 << W;
const HALF: i8 = 1 << (W - 1);
const ENTRIES: usize = 1usize << (W - 2);

pub fn window_table(p: &Affine) -> [Jacobian; ENTRIES] {
    // naf = P, 3P, 5P, ... 15P
    // OPT: Optimal window size
    let mut naf: [Jacobian; ENTRIES] = Default::default();;
    naf[0] = Jacobian::from(p);
    let p2 = naf[0].double();
    for i in 1..naf.len() {
        naf[i] = &naf[i - 1] + &p2;
    }
    // OPT: Use batch inversion to convert to Affine
    naf
}

// Convert scalar to naf form
// See https://doc-internal.dalek.rs/curve25519_dalek/scalar/struct.Scalar.html#method.non_adjacent_form
// See https://github.com/dalek-cryptography/curve25519-dalek/blob/ca6305232f08ced340819b8ae691f90492d1b054/src/scalar.rs#L872
// Algorithm 3.35 of Guide to Elliptic Curve Cryptography
// OPT: Can we turn this into a left-to-right version of the algorithm
//      so we can consume the values as they are produced and we don't
//      need any allocations?
pub fn non_adjacent_form(mut scalar: U256) -> [i8; 257] {
    let mut snaf = [0i8; 257];
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
        let mut n: i8 = (scalar.c0 & MASK) as i8;
        scalar >>= W;

        // Make negative if n > 2^(w-1)
        if n >= HALF {
            n -= WINDOW;
            scalar += U256::ONE;
        }

        // Store and advance index
        snaf[i] = n;
        i += W;
    }

    snaf
}

// Multiply Affine point using Jacobian accumulator
// See https://doc-internal.dalek.rs/curve25519_dalek/traits/trait.VartimeMultiscalarMul.html
pub fn mul(p: &Affine, scalar: &U256) -> Jacobian {
    // Precomputed odd multiples
    let naf = window_table(p);

    // Get SNAF
    let snaf = non_adjacent_form(scalar.clone());

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
    let nafa = window_table(pa);
    let nafb = window_table(pb);

    // Get SNAF
    let snafa = non_adjacent_form(sa);
    let snafb = non_adjacent_form(sb);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::FieldElement;
    use crate::u256h;
    use hex_literal::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn test_mul() {
        let a = Affine::Point {
            x: FieldElement::new(&[
                0x5bf31eb0, 0xfe50a889, 0x2d1a8a21, 0x3242e28e, 0x0d13fe66, 0xcf63e064, 0x9426e2c3,
                0x0040ffd5,
            ]),
            y: FieldElement::new(&[
                0xe29859d2, 0xd21b931a, 0xea34d27d, 0x296f19b9, 0x6487ae5b, 0x524260f9, 0x069092ca,
                0x060c2257,
            ]),
        };
        let b = U256::from_slice(&[
            0x711a14cf, 0xebe54f04, 0x4729d630, 0xd14a329a, 0xf5480b47, 0x35fdc862, 0xde09131d,
            0x029f7a37,
        ]);
        let c = Jacobian::from(Affine::Point {
            x: FieldElement::new(&[
                0x143de731, 0x4c657d7e, 0x44b99cbf, 0x49dfc2e5, 0x40ea4226, 0xaf6c4895, 0x9a141832,
                0x04851acc,
            ]),
            y: FieldElement::new(&[
                0x138592fd, 0x1377613f, 0xd53c61dd, 0xaa8b32c1, 0xd5bf18bc, 0x3b22a665, 0xf54ed6ae,
                0x07f4bb53,
            ]),
        });
        assert_eq!(mul(&a, &b), c);
    }

    #[test]
    fn test_mul_2() {
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
