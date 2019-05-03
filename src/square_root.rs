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
    if *a == FieldElement::ZERO {
        true
    } else {
        match a.pow(MODULUS >> 1) {
            None => panic!(),
            Some(value) => value == FieldElement::ONE,
        }
    }
}

const QUADRATIC_NONRESIDUE: FieldElement = FieldElement(u256h!(
    "028ad127451958b2b5667daad6c2fd516640381f2abac83fba5054da02fdf054"
));

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

    // Because a is not 0 at this point, it's safe to divide by it and exponentiate it.
    let mut c: FieldElement = QUADRATIC_NONRESIDUE.pow(SIGNIFICAND).unwrap();
    let mut root: FieldElement = a.pow((SIGNIFICAND + U256::from(1u128)) >> 1).unwrap();

    let negative_one = FieldElement::ZERO - FieldElement::ONE;

    for i in 1..BINARY_EXPONENT {
        if (root.square() / a)
            .pow(U256::from(1u128) << (BINARY_EXPONENT - i - 1))
            .unwrap()
            == negative_one
        {
            root *= &c;
        }
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
    fn quadratic_nonresidue_is_as_claimed() {
        assert!(!is_quadratic_residue(&QUADRATIC_NONRESIDUE));
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
