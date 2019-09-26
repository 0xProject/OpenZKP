#[cfg(feature = "prover")]
use crate::TraceTable;
use crate::{
    constraint_system::{Provable, Verifiable},
    constraints::Constraints,
    rational_expression::RationalExpression,
};
use macros_decl::field_element;
use primefield::{FieldElement};
use u256::U256;

#[derive(Debug)]
pub struct Claim {
    pub c0_start: FieldElement,
    pub c1_start: FieldElement,
    pub c0_end: FieldElement,
    pub c1_end: FieldElement,
}

pub const R : FieldElement = field_element!("064d86ee99c06ccb55648b536b29a7ae9cecc50c6aac5519dc71da89f5cce6dc");

impl From<&Claim> for Vec<u8> {
    fn from(input: &Claim) -> Self {
        let mut ret = input.c0_start.as_montgomery().to_bytes_be().to_vec();
        ret.extend_from_slice(&input.c1_start.as_montgomery().to_bytes_be());
        ret.extend_from_slice(&input.c0_end.as_montgomery().to_bytes_be());
        ret.extend_from_slice(&input.c1_end.as_montgomery().to_bytes_be());
        ret
    }
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

        Constraints::new(vec![
            // Square (Trace(0,0), Trace(1, 0)) and check that it equals (Trace(2,0), Trace(3,0))
            (Exp(Trace(0,0).into(), 2) + Constant(R)*Exp(Trace(1,0).into(), 2)  -Trace(2, 0)) * reevery_row(),
            (Constant(2.into())*Trace(0,0)*Trace(1,0)  - Trace(3, 0)) * reevery_row(),
            // Multiply the square by the single and the square and enforce it on the next row
            (Trace(0,0)*Trace(2,0) + Constant(R)*Trace(1,0)*Trace(3,0) - Trace(0, 1)) * reevery_row(),
            (Trace(0,0)*Trace(2,0) + Trace(1,0)*Trace(3,0) - Trace(0, 2)) * reevery_row(),
            // Boundary Constraints
            (Trace(1, 0) - (&self.c0_start).into()) * on_row(0),
            (Trace(0, 0) - (&self.c1_start).into()) * on_row(0),
            (Trace(0, 0) - (&self.c0_end).into()) * on_row(trace_length - 1),
            (Trace(1, 0) - (&self.c1_end).into()) * on_row(trace_length - 1),
        ])
    }

    fn trace_length(&self) -> usize {
        1048576
    }

    fn trace_columns(&self) -> usize {
        4
    }
}

impl Provable<Claim> for () {
    #[cfg(feature = "prover")]
    fn trace(&self, claim: &Claim) -> TraceTable {
        let mut trace = TraceTable::new(1048576, 4);

        let mut prev_c0 = claim.c0_start.clone();
        let mut prev_c1 = claim.c1_start.clone();
        for i in 0..1048576 {
            trace[(i, 0)] = prev_c0.clone();
            trace[(i, 1)] = prev_c1.clone();
            trace[(i, 2)] = (&trace[(i, 0)]).square() + &R*(&trace[(i, 1)].square());
            trace[(i, 3)] = FieldElement::from(2)*&trace[(i, 0)]*&trace[(i, 1)];
            prev_c0 = &trace[(i, 0)]*&trace[(i, 2)] + &R*&trace[(i, 1)]*&trace[(i, 3)];
            prev_c1 = &trace[(i, 0)]*&trace[(i, 2)] + &trace[(i, 1)]*&trace[(i, 3)]
        }
        assert_eq!(trace[(1048576 - 1, 0)], claim.c0_end);
        assert_eq!(trace[(1048576 - 1, 1)], claim.c1_end);
        trace
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{proof_params::ProofParams, proofs::stark_proof, verifier::check_proof};
    use macros_decl::field_element;

    #[ignore]
    #[test]
    fn matter_vfd_test() {
        let c0_start =
            field_element!("00a74f2a70da4ea3723cabd2acc55d03f9ff6d0e7acef0fc63263b12c10dd837");
        let c1_start =
            field_element!("02ba0d3dfeb1ee83889c5ad8534ba15723a42b306e2f44d5eee10bfa939ae756");
        let c0_end =
            field_element!("02c190f26be11bc330401087c92214777ca6e2d25183303d0b0ec4feb7277f64");
        let c1_end =
            field_element!("05e1e4162ab76832cc21610cc20c25b998ecbf53d0825b9ccd7f80037c532856");
        let input = Claim { c0_start, c1_start, c0_end, c1_end};
        let params = ProofParams::suggested(1048576);
        let potential_proof = stark_proof(&input, &(), &params);
        assert_eq!(
            check_proof(potential_proof.proof.as_slice(), &input, &params),
            Ok(())
        );
    }
}