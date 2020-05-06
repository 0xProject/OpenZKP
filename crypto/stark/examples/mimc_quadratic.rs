use std::time::Instant;
use zkp_macros_decl::field_element;
use zkp_primefield::{fft::permute, Fft, FieldElement, Pow, Root, SquareInline};
use zkp_stark::{
    Constraints, DensePolynomial, Provable, RationalExpression, TraceTable, Verifiable,
};
use zkp_u256::U256;

// Note - this higher memory MiMC uses a fixed alpha = 3
const ROUNDS: usize = 8192; // 2^13 to match Guild of Weavers
                            // These round coefficents are the hex of those used by Guild of Weavers
const K_COEF: [FieldElement; 16] = [
    field_element!("2A"),
    field_element!("2B"),
    field_element!("AA"),
    field_element!("08A1"),
    field_element!("402A"),
    field_element!("013107"),
    field_element!("0445AA"),
    field_element!("0C90DD"),
    field_element!("20002A"),
    field_element!("48FB53"),
    field_element!("9896AA"),
    field_element!("012959E9"),
    field_element!("0222C02A"),
    field_element!("03BD774F"),
    field_element!("06487BAA"),
    field_element!("0A2F1B45"),
];
// Proves that 'after' is the ALPHA MiMC applied to 'before' after 'rounds'
// iterations of the cypher
#[derive(Debug)]
pub struct Claim {
    before: FieldElement,
    after:  FieldElement,
}

impl Verifiable for Claim {
    fn constraints(&self) -> Constraints {
        use RationalExpression::*;

        // Seed
        let mut seed = self.before.as_montgomery().to_bytes_be().to_vec();
        seed.extend_from_slice(&self.after.as_montgomery().to_bytes_be());

        // Constraint repetitions
        let trace_length = ROUNDS;
        let trace_generator = FieldElement::root(trace_length).unwrap();
        let g = Constant(trace_generator);
        let on_row = |index| (X - g.pow(index)).inv();
        let every_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1);

        let periodic = |coefficients| {
            Polynomial(
                DensePolynomial::new(coefficients),
                Box::new(X.pow(trace_length / 16)),
            )
        };
        let mut k_coef = K_COEF.to_vec();
        k_coef.ifft();
        permute(&mut k_coef);
        let k_coef = periodic(&k_coef);

        Constraints::from_expressions((trace_length, 3), seed, vec![
            // Says x_1 = x_0^2
            (Trace(0, 0) * Trace(0, 0) - Trace(1, 0)) * every_row(),
            // Says x_2 = x_1*x_0
            (Trace(0, 0) * Trace(1, 0) - Trace(2, 0)) * every_row(),
            // Says next row's x_0 = prev row x_2 + k_this row
            (Trace(0, 1) - (Trace(2, 0) + k_coef)) * every_row(),
            // Says the first x_0 is the before
            (Trace(0, 0) - &self.before) * on_row(0),
            // Says the the x_0 on row ROUNDS
            (Trace(0, 0) - &self.after) * on_row(trace_length - 1),
        ])
        .unwrap()
    }
}

impl Provable<()> for Claim {
    fn trace(&self, _witness: ()) -> TraceTable {
        let mut trace = TraceTable::new(ROUNDS, 3);

        let mut prev = self.before.clone();
        for i in 0..ROUNDS {
            trace[(i, 0)] = prev.clone();
            trace[(i, 1)] = (&trace[(i, 0)]).square();
            trace[(i, 2)] = &trace[(i, 0)] * &trace[(i, 1)];
            prev = &trace[(i, 2)] + &K_COEF[i % 16];
        }
        assert_eq!(trace[(ROUNDS - 1, 0)], self.after);
        trace
    }
}

fn mimc(start: &FieldElement) -> FieldElement {
    let mut prev = start.clone();
    for i in 1..ROUNDS {
        prev = prev.pow(3_usize) + &K_COEF[(i - 1) % 16];
    }
    prev
}

fn main() {
    let before = field_element!("00a74f2a70da4ea3723cabd2acc55d03f9ff6d0e7acef0fc63263b12c10dd837");
    let after = mimc(&before);
    let start = Instant::now();
    let claim = Claim { before, after };
    assert_eq!(claim.check(()), Ok(()));
    let proof = claim.prove(()).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed in proof function is: {:?}", duration);
    println!("The proof length is {}", proof.as_bytes().len());
    claim.verify(&proof).unwrap();
}
