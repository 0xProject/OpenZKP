use crate::field::FieldElement;
use macros_decl::u256h;
use u256::U256;

pub fn square_root(a: &FieldElement) -> Option<FieldElement> {
    if is_quadratic_residue(a) {
        Some(tonelli_shanks(a))
    } else {
        None
    }
}

// Returns the result of (a/p) != -1, where (a/p) is the Legendre symbol.
fn is_quadratic_residue(a: &FieldElement) -> bool {
    a.pow(FieldElement::MODULUS >> 1) != FieldElement::NEGATIVE_ONE
}

// These two constants are chosen so that 1 + SIGNIFICAND << BINARY_EXPONENT ==
// MODULUS.
const BINARY_EXPONENT: usize = 3 * 4 * 16;
const SIGNIFICAND: U256 = U256::from_limbs(0x0800_0000_0000_0011u64, 0, 0, 0);
// The starting value of c in the Tonelli Shanks algorithm. We are using 3, a
// generator, as the quadratic nonresidue the algorithm requires.
const INITIAL_C: FieldElement = FieldElement::from_montgomery(u256h!(
    "07222e32c47afc260a35c5be60505574aaada25731fe3be94106bccd64a2bdd8"
));

// What about using algorithm 3.39 instead?
fn tonelli_shanks(a: &FieldElement) -> FieldElement {
    // This algorithm is still correct when the following assertion fails. However,
    // more efficient algorithms exist when MODULUS % 4 == 1 or MODULUS % 8 == 5
    // (3.36 and 3.37 in HAC).
    debug_assert!(&FieldElement::MODULUS & 7u64 == 1);

    if a.is_zero() {
        return FieldElement::ZERO;
    }

    let mut c: FieldElement = INITIAL_C;
    // OPT: Raising a to a fixed power is a good candidate for an addition chain.
    let mut root: FieldElement = a.pow((SIGNIFICAND + U256::from(1u128)) >> 1);

    for i in 1..BINARY_EXPONENT {
        // OPT: Precompute the inverse of a.
        if (root.square() / a).pow(U256::from(1u128) << (BINARY_EXPONENT - i - 1))
            == FieldElement::NEGATIVE_ONE
        {
            root *= &c;
        }
        // OPT: Create lookup table for squares of c.
        c = c.square();
    }
    root
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn binary_exponent_is_correct() {
        assert_eq!(
            BINARY_EXPONENT,
            (FieldElement::MODULUS - U256::from(1u128)).trailing_zeros()
        );
    }

    #[test]
    fn significand_is_correct() {
        assert_eq!(
            SIGNIFICAND << BINARY_EXPONENT,
            FieldElement::MODULUS - U256::from(1u128)
        );
    }

    #[test]
    fn initial_c_is_correct() {
        assert_eq!(INITIAL_C, FieldElement::GENERATOR.pow(SIGNIFICAND));
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
            square_root(&FieldElement::ZERO).unwrap(),
            FieldElement::ZERO
        );
    }

    #[test]
    fn square_root_of_one() {
        assert_eq!(square_root(&FieldElement::ONE).unwrap(), FieldElement::ONE);
    }
}
