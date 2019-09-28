#[cfg(feature = "prover")]
use crate::TraceTable;
use crate::{polynomial::DensePolynomial, Constraints, Provable, RationalExpression, Verifiable};
use macros_decl::field_element;
use primefield::{fft::ifft, FieldElement};
use u256::U256;

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

impl From<&Claim> for Vec<u8> {
    fn from(input: &Claim) -> Self {
        let mut ret = input.before.as_montgomery().to_bytes_be().to_vec();
        ret.extend_from_slice(&input.after.as_montgomery().to_bytes_be());
        ret
    }
}

impl Verifiable for Claim {
    fn constraints(&self) -> Constraints {
        use RationalExpression::*;

        let trace_length = ROUNDS;
        let trace_generator = FieldElement::root(trace_length).unwrap();

        // Constraint repetitions
        let g = Constant(trace_generator);
        let on_row = |index| (X - g.pow(index)).inv();
        let reevery_row = || (X - g.pow(trace_length - 1)) / (X.pow(trace_length) - 1.into());

        let periodic = |coefficients| {
            Polynomial(
                DensePolynomial::new(coefficients),
                Box::new(X.pow(trace_length / 16)),
            )
        };
        let k_coef = periodic(&ifft(&K_COEF.to_vec()));

        Constraints::from_expressions((trace_length, 3), self.into(), vec![
            // Says x_1 = x_0^2
            (Trace(0, 0) * Trace(0, 0) - Trace(1, 0)) * reevery_row(),
            // Says x_2 = x_1*x_0
            (Trace(0, 0) * Trace(1, 0) - Trace(2, 0)) * reevery_row(),
            // Says next row's x_0 = prev row x_2 + k_this row
            (Trace(0, 1) - (Trace(2, 0) + k_coef.clone())) * reevery_row(),
            // Says the first x_0 is the before
            (Trace(0, 0) - (&self.before).into()) * on_row(0),
            // Says the the x_0 on row ROUNDS
            (Trace(0, 0) - (&self.after).into()) * on_row(trace_length - 1),
        ])
        .unwrap()
    }
}

impl Provable<()> for Claim {
    #[cfg(feature = "prover")]
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

#[allow(dead_code)]
fn mimc(start: &FieldElement) -> FieldElement {
    let mut prev = start.clone();
    for i in 1..ROUNDS {
        prev = prev.pow(3) + &K_COEF[(i - 1) % 16];
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{proof, verify};
    use macros_decl::field_element;

    #[test]
    fn mimc_hash_memory_test() {
        let before =
            field_element!("00a74f2a70da4ea3723cabd2acc55d03f9ff6d0e7acef0fc63263b12c10dd837");
        let after = mimc(&before);
        let input = Claim { before, after };
        let constraints = input.constraints();
        let trace = input.trace(());
        let potential_proof = proof(&constraints, &trace);
        assert_eq!(
            verify(&constraints, potential_proof.proof.as_slice()),
            Ok(())
        );
    }
}
