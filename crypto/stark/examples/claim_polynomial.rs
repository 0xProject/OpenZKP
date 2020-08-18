use zkp_macros_decl::field_element;
use zkp_primefield::FieldElement;
use zkp_stark::{
    generate, proof_serialize, prove, Constraints, DensePolynomial, Provable, RationalExpression,
    TraceTable, Verifiable,
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
            (Trace(0, 0) - ClaimPolynomial(0, 0, Box::new(X), Some("MyClaimPoly"))) / (X - 1),
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

impl Claim {
    fn concrete_system(&self) -> Constraints {
        let claim_polynomials = vec![DensePolynomial::new(&[self.0.clone()])];
        let expressions = self
            .constraints()
            .expressions()
            .iter()
            .map(|x| x.substitute_claim(&claim_polynomials))
            .collect();

        Constraints::from_expressions(
            (2, 1),
            self.0.as_montgomery().to_bytes_be().to_vec(),
            expressions,
        )
        .unwrap()
    }
}

fn main() {
    let claim = Claim(field_element!("1325123410"));
    let witness = Witness(claim.0.clone());

    println!("claim: 0x{}", claim.0.as_montgomery());

    let concrete_system = claim.concrete_system();
    let trace = claim.trace(&witness);
    let proof = prove(&concrete_system, &trace).unwrap();

    let mut proof_string = "".to_string();
    proof_serialize(&concrete_system, &proof, &mut proof_string).unwrap();
    println!("{}", proof_string);

    let system = claim.constraints();
    let _ = generate(
        &system,
        "../stark-verifier-ethereum/contracts/claim_polynomial",
        "Claim",
    );
}
