use super::elliptic_helpers::*;
#[cfg(feature = "prover")]
use crate::TraceTable;
use crate::{
    constraint_system::{Provable, Verifiable},
    constraints::Constraints,
    polynomial::DensePolynomial,
    rational_expression::RationalExpression::{self, *},
};
use macros_decl::field_element;
use primefield::{fft::ifft, FieldElement};
use u256::U256;
use elliptic_curve_crypto as ecc;

struct Claim {
    hash: FieldElement,
    who:  (FieldElement, FieldElement),
}

struct Witness {
    signature: (FieldElement, FieldElement),
}

impl Verifiable for Claim {
    fn constraints(&self) -> Constraints {
        use RationalExpression::*;

        let trace_length = self.trace_length();
        let trace_generator = FieldElement::root(trace_length).unwrap();

        // Constraint repetitions
        let g = Constant(trace_generator);
        let on_row = |index| (X - g.pow(index)).inv();
        let reevery_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1.into());

        Constraints::new(vec![])
    }

    fn trace_length(&self) -> usize {
        512
    }

    fn trace_columns(&self) -> usize {
        5
    }
}

impl Provable<Claim> for Witness {
    #[cfg(feature = "prover")]
    fn trace(&self, claim: &Claim) -> TraceTable {
        let mut trace = TraceTable::new(512, 5);
        // u_1 = hash * s inverse
        let u_1 = U256::from(&claim.hash).mulmod(&U256::from(&self.signature.1), &elliptic_curve::ORDER);
        // u_2 = r * s inverse
        let u_2 = U256::from(&self.signature.0).mulmod(&U256::from(&self.signature.1), &elliptic_curve::ORDER);
        // u_1 x G
        scalar_mult(&mut trace, GENERATOR.clone(), &u_1, 0, 0, false);
        // u_2 x PublicKey
        scalar_mult(&mut trace, claim.who.clone(), &u_2, 256, 0, true);
        trace
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use elliptic_curve::Affine;
    use macros_decl::{field_element, u256h};

    #[test]
    fn test_sign_table() {
        let private_key =
            u256h!("03c1e9550e66958296d11b60f8e8e7a7ad990d07fa65d5f7652c4a6c87d4e3cc");
        let message_hash = u256h!("01921ce52df68f0185ade7572776513304bdd4a07faf6cf28cefc65a86fc496c");
        let public_affine = ecc::private_to_public(&private_key);
        let public = match public_affine.clone() {
            Affine::Zero => (FieldElement::ZERO, FieldElement::ZERO),
            Affine::Point { x, y } => (x, y),
        };

        let (r, w) = ecc::sign(&U256::from(message_hash.clone()), &private_key);

        let u_1 = message_hash.mulmod(&w, &elliptic_curve::ORDER);
        let u_2 = &r.mulmod(&w, &elliptic_curve::ORDER);

        let first_expected_affine = elliptic_curve::GENERATOR * u_1;
        let second_expected_affine = public_affine * u_2;
        let first_expected = match first_expected_affine.clone() {
            Affine::Zero => (FieldElement::ZERO, FieldElement::ZERO),
            Affine::Point { x, y } => (x, y),
        };
        let second_expected = match second_expected_affine.clone() {
            Affine::Zero => (FieldElement::ZERO, FieldElement::ZERO),
            Affine::Point { x, y } => (x, y),
        };

        let claim = Claim {
            hash: FieldElement::from(message_hash),
            who:  public,
        };
        let witness = Witness {
            signature: (FieldElement::from(r.clone()), FieldElement::from(w)),
        };
        let trace = witness.trace(&claim);

        let mut neg_shift = SHIFT_POINT.clone();
        neg_shift.1 = -&neg_shift.1;
        // First check, checks that the proper scalar mults are put in place
        let shifted_trace1 = add(
            &trace[(255, 3)],
            &trace[(255, 4)],
            &neg_shift.0,
            &neg_shift.1,
        );
        let shifted_trace2 = add(
            &trace[(511, 3)],
            &trace[(511, 4)],
            &SHIFT_POINT.0,
            &SHIFT_POINT.1,
        );
        assert_eq!(first_expected, shifted_trace1.clone());
        assert_eq!(second_expected, shifted_trace2.clone());

        let mut final_check = add(
            &trace[(255, 3)],
            &trace[(255, 4)],
            &trace[(511, 3)],
            &trace[(511, 4)],
        );
        assert_eq!(FieldElement::from(r), final_check.0);
    }
}
