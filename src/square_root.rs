use crate::field::{FieldElement, MODULUS};
use crate::u256::U256;

pub fn square_root(a: &FieldElement) -> Option<FieldElement> {
    if is_quadratic_residue(a) {
        Some(tonelli_shanks(a))
    } else {
        None
    }
}

const QUADRATIC_NONRESIDUE: FieldElement = FieldElement(u256h!(
    "028ad127451958b2b5667daad6c2fd516640381f2abac83fba5054da02fdf054"
));

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

// What about using algorithm 3.39 instead?
fn tonelli_shanks(a: &FieldElement) -> FieldElement {
    assert!(&MODULUS & 7u64 == 1);

    let s: usize = (MODULUS - U256::from(1u128)).trailing_zeros();
    let t: U256 = (MODULUS - U256::from(1u128)) >> s;

    let mut c: FieldElement = QUADRATIC_NONRESIDUE.pow(t).unwrap();
    let mut root: FieldElement = a.pow((t + U256::from(1 as u128)) >> 1).unwrap();

    let negative_one = FieldElement::ZERO - FieldElement::ONE;

    for i in 1..s {
        if (root.square() / a)
            .pow(U256::from(1u128) << (s - i - 1))
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
    fn square_root_of_one() {
        assert!(square_root(&FieldElement::ONE).unwrap() == FieldElement::ONE);
    }
}
