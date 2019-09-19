use crate::TraceTable;
use primefield::FieldElement;
use u256::U256;

const ALPHA: usize = 3;
const ROUNDS: usize = 8192; // 2^13 to match Guild of Weavers
const K_COEF: [usize; 16] = [
    42, 43, 170, 2209, 16426, 78087, 279978, 823517, 2097194, 4782931, 10000042, 19487209,
    35831850, 62748495, 105413546, 170859333,
];
// Proves that after is the alpha MiMC on before after rounds
#[derive(Debug)]
pub struct PublicInput {
    before: FieldElement,
    after:  FieldElement,
}

// Private input included for consistent function signatures
pub fn get_trace_table(public_input: &PublicInput) -> TraceTable {
    let mut trace = TraceTable::new(ROUNDS + 1, 1);
    trace[(0, 0)] = public_input.before.clone();
    let mut prev = public_input.before.clone();
    for i in 1..(ROUNDS + 1) {
        let hold = prev.pow(ALPHA) + FieldElement::from(U256::from(K_COEF[(i - 1) % 16]));
        trace[(i, 0)] = hold.clone();
        prev = hold;
    }
    assert_eq!(trace[(ROUNDS, 0)], public_input.after);
    trace
}

pub fn mimc(start: &FieldElement) -> FieldElement {
    let mut prev = start.clone();
    for i in 0..ROUNDS {
        prev = prev.pow(ALPHA) + FieldElement::from(U256::from(K_COEF[i % 16]));
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;
    use macros_decl::field_element;

    #[test]
    fn mimc_hash_test() {
        let before =
            field_element!("00a74f2a70da4ea3723cabd2acc55d03f9ff6d0e7acef0fc63263b12c10dd837");
        let after = mimc(&before);
        let input = PublicInput { before, after };
        let _trace_table = get_trace_table(&input);
    }
}
