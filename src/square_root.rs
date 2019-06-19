use crate::field::{FieldElement, MODULUS};
use crate::u256::U256;

pub fn square_root(a: &FieldElement) -> Option<FieldElement> {
    if is_quadratic_residue(a) {
        Some(tonelli_shanks(a))
    } else {
        None
    }
}

// Returns the result of (a/p) != -1, where (a/p) is the Legendre symbol.
fn is_quadratic_residue(a: &FieldElement) -> bool {
    match a.pow(MODULUS >> 1) {
        None => panic!(),
        Some(value) => value != FieldElement::NEGATIVE_ONE,
    }
}

// The generator, 3, is the smallest quadratic nonresidue in the finite field.
const QUADRATIC_NONRESIDUE: FieldElement = FieldElement::GENERATOR;

// These two constants are chosen so that 1 + SIGNIFICAND << BINARY_EXPONENT == MODULUS.
const BINARY_EXPONENT: usize = 3 * 4 * 16;
const SIGNIFICAND: U256 = U256::new(0x0800000000000011u64, 0, 0, 0);

// What about using algorithm 3.39 instead?
fn tonelli_shanks(a: &FieldElement) -> FieldElement {
    // This algorithm is still correct when the following assertion fails. However,
    // more efficient algorithms exist when MODULUS % 4 == 1 or MODULUS % 8 == 5
    // (3.36 and 3.37 in HAC).
    debug_assert!(&MODULUS & 7u64 == 1);

    if a.is_zero() {
        return FieldElement::ZERO;
    }

    // OPT: Good candidate for an addition chain. Declare constant values as such once
    // conditionals are allowed inside const fn's: https://github.com/rust-lang/rust/issues/49146
    let mut c: FieldElement = QUADRATIC_NONRESIDUE.pow(SIGNIFICAND).unwrap();
    // Because a is not 0 at this point, it's safe to divide by it and exponentiate it.
    let mut root: FieldElement = a.pow((SIGNIFICAND + U256::from(1u128)) >> 1).unwrap();

    for i in 1..BINARY_EXPONENT {
        // OPT: Precompute the inverse of a.
        if (root.square() / a)
            .pow(U256::from(1u128) << (BINARY_EXPONENT - i - 1))
            .unwrap()
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
        assert!(BINARY_EXPONENT == (MODULUS - U256::from(1u128)).trailing_zeros());
    }

    #[test]
    fn significand_is_correct() {
        assert!(SIGNIFICAND << BINARY_EXPONENT == MODULUS - U256::from(1u128));
    }

    #[test]
    fn quadratic_nonresidue_is_three() {
        assert_eq!(QUADRATIC_NONRESIDUE, FieldElement::from(U256::from(3u128)));
    }

    #[test]
    fn quadratic_nonresidue_is_as_claimed() {
        assert!(!is_quadratic_residue(&QUADRATIC_NONRESIDUE));
    }

    #[test]
    fn quadratic_nonresidue_is_smallest() {
        let mut i = 0u128;
        while U256::from(i) < QUADRATIC_NONRESIDUE.into() {
            assert!(is_quadratic_residue(&FieldElement::from(U256::from(i))));
            i += 1;
        }
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
        assert!(square_root(&FieldElement::ZERO).unwrap() == FieldElement::ZERO);
    }

    #[test]
    fn square_root_of_one() {
        assert!(square_root(&FieldElement::ONE).unwrap() == FieldElement::ONE);
    }
}
