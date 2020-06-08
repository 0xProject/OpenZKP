use zkp_macros_decl::field_element;
use zkp_primefield::FieldElement;
use zkp_stark::{
    generate, proof_serialize, prove, Constraints, Provable, RationalExpression, TraceTable,
    Verifiable,
};
use zkp_u256::U256;

#[derive(Clone, Debug)]
struct Claim(FieldElement);

#[derive(Clone, Debug)]
struct Witness(FieldElement);

impl Verifiable for Claim {
    fn constraints(&self) -> Constraints {
        use RationalExpression::*;
        Constraints::from_expressions((2, 1), self.0.as_montgomery().to_bytes_be().to_vec(), vec![
            (Trace(0, 0) - Constant(self.0.clone())) / (X - 1),
        ])
        .unwrap()
    }
}

impl Provable<&Witness> for Claim {
    fn trace(&self, witness: &Witness) -> TraceTable {
        let mut trace = TraceTable::new(2, 1);
        trace[(0, 0)] = witness.0.clone();
        trace[(1, 0)] = witness.0.clone() + FieldElement::from(100);
        trace
    }
}

fn main() {
    let claim = Claim(field_element!("1325123410"));
    let witness = Witness(claim.0.clone());

    let mut constraints = claim.constraints();
    constraints.num_queries = 2;
    constraints.pow_bits = 10;

    let trace = claim.trace(&witness);

    println!("claim: 0x{}", claim.0.as_montgomery());

    let mut proof_string = "".to_string();
    proof_serialize(
        &constraints,
        &prove(&constraints, &trace).unwrap(),
        &mut proof_string,
    )
    .unwrap();
    println!("{}", proof_string);

    let system = claim.constraints();

    let _ = generate(
        system.trace_nrows(),
        // &[&Constant(claim.0.clone())],
        &system.expressions(),
        system.trace_ncolumns(),
        16,
        "../stark-verifier-ethereum/contracts/constant",
    );
}
