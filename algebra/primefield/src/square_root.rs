use crate::{field::FieldElement, One, Pow, SquareInline, Zero};
use zkp_macros_decl::u256h;
use zkp_u256::U256;

pub(crate) fn square_root(a: &FieldElement) -> Option<FieldElement> {
    if is_quadratic_residue(a) {
        Some(tonelli_shanks(a))
    } else {
        None
    }
}

// Returns the result of (a/p) != -1, where (a/p) is the Legendre symbol.
fn is_quadratic_residue(a: &FieldElement) -> bool {
    a.pow(&(FieldElement::MODULUS >> 1)) != -FieldElement::one()
}

// These two constants are chosen so that 1 + SIGNIFICAND << BINARY_EXPONENT ==
// MODULUS.
// TODO: Provide as constant parameters
const BINARY_EXPONENT: usize = 3 * 4 * 16;
const SIGNIFICAND: U256 = U256::from_limbs([0x0800_0000_0000_0011_u64, 0, 0, 0]);

// What about using algorithm 3.39 instead?
fn tonelli_shanks(a: &FieldElement) -> FieldElement {
    // This algorithm is still correct when the following assertion fails. However,
    // more efficient algorithms exist when MODULUS % 4 == 1 or MODULUS % 8 == 5
    // (3.36 and 3.37 in HAC).
    debug_assert!(&FieldElement::MODULUS & 7_u64 == 1);

    if a.is_zero() {
        return FieldElement::zero();
    }

    // The starting value of c in the Tonelli Shanks algorithm. We are using the
    // prefered generator, as the quadratic nonresidue the algorithm requires.
    // TODO: Provide as a constant parameter
    let mut c: FieldElement = FieldElement::generator().pow(&SIGNIFICAND);

    // OPT: Raising a to a fixed power is a good candidate for an addition chain.
    let mut root: FieldElement = a.pow(&((SIGNIFICAND + U256::ONE) >> 1));

    for i in 1..BINARY_EXPONENT {
        // OPT: Precompute the inverse of a.
        if (root.square() / a).pow(&(U256::ONE << (BINARY_EXPONENT - i - 1)))
            == -FieldElement::one()
        {
            root *= &c;
        }
        // OPT: Create lookup table for squares of c.
        c = c.square();
    }
    root
}

// Quickcheck needs pass by value
#[allow(clippy::needless_pass_by_value)]
#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;
    use zkp_u256::Binary;

    #[test]
    fn binary_exponent_is_correct() {
        assert_eq!(
            BINARY_EXPONENT,
            (FieldElement::MODULUS - U256::ONE).trailing_zeros()
        );
    }

    #[test]
    fn significand_is_correct() {
        assert_eq!(
            SIGNIFICAND << BINARY_EXPONENT,
            FieldElement::MODULUS - U256::ONE
        );
    }

    #[test]
    fn initial_c_is_correct() {
        // TODO: assert_eq!(INITIAL_C,
        // FieldElement::generator().pow(SIGNIFICAND));
    }

    #[quickcheck]
    fn squares_are_quadratic_residues(x: FieldElement) -> bool {
        is_quadratic_residue(&x.square())
    }

    #[quickcheck]
    fn inverse(x: FieldElement) -> bool {
        match square_root(&x) {
            None => !is_quadratic_residue(&x),
            Some(result) => result.square() == x,
        }
    }

    #[test]
    fn square_root_of_zero() {
        assert_eq!(
            square_root(&FieldElement::zero()).unwrap(),
            FieldElement::zero()
        );
    }

    #[test]
    fn square_root_of_one() {
        assert_eq!(
            square_root(&FieldElement::one()).unwrap(),
            FieldElement::one()
        );
    }
}
