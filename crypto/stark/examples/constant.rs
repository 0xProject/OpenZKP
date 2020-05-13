#![warn(clippy::all)]
use env_logger;
use log::info;
use std::time::Instant;
use zkp_macros_decl::field_element;
use zkp_primefield::{FieldElement, Root};
use zkp_stark::{generate, Constraints, Provable, RationalExpression, TraceTable, Verifiable};
use zkp_u256::U256;

#[derive(Clone, Debug)]
struct Claim(FieldElement);

#[derive(Clone, Debug)]
struct Witness(FieldElement);

impl Verifiable for Claim {
    fn constraints(&self) -> Constraints {
        use RationalExpression::*;
        Constraints::from_expressions((1, 1), self.0.as_montgomery().to_bytes_be().to_vec(), vec![
            (Trace(0, 0) - ClaimPolynomial(0, 0, Box::new(X))) / (X - 1),
        ])
        .unwrap()
    }
}

impl Provable<&Witness> for Claim {
    fn trace(&self, witness: &Witness) -> TraceTable {
        let mut trace = TraceTable::new(1, 1);
        trace[(0, 0)] = witness.0.clone();
        trace
    }
}

fn main() {
    let claim = Claim(field_element!("1325123410"));
    let witness = Witness(claim.0.clone());

    use RationalExpression::*;

    let _ = generate(
        1,
        &[&Constant(claim.0.clone())],
        &[(Trace(0, 0) - Constant(claim.0)) / (X - 1)],
        1,
        16,
        "../stark-verifier-ethereum/contracts/constant"
    );
}
