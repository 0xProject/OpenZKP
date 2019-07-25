use crate::{
    fft::{bit_reversal_fft_cofactor, ifft},
    mmap_vec::MmapVec,
    pedersen_merkle::{
        input::{get_private_input, get_public_input},
        trace_table::get_trace,
    },
    utils::Reversible,
};
use primefield::FieldElement;
use rayon::prelude::*;
use u256::U256;

pub fn get_trace_polynomials() -> Vec<Vec<FieldElement>> {
    let public_input = get_public_input();
    let trace_table = get_trace(
        public_input.path_length,
        public_input.leaf,
        &get_private_input(),
    );

    let mut columns: [Vec<FieldElement>; 8] = Default::default();
    let r1 = FieldElement::ONE.0;
    for (i, value) in trace_table.iter().enumerate() {
        if i % 4 == 0 {
            columns[i % 8].push(FieldElement::from(&r1) * value.clone());
        } else {
            columns[i % 8].push(value.clone());
        }
    }

    let trace_length: usize = public_input.path_length * 256;

    let mut trace_polynomials: Vec<Vec<FieldElement>> = vec![Vec::with_capacity(trace_length); 8];
    columns
        .into_par_iter()
        .map(|c| ifft(&c))
        .collect_into_vec(&mut trace_polynomials);

    trace_polynomials
}

pub fn get_extended_trace_table() -> MmapVec<[FieldElement; 8]> {
    let trace_polynomials = get_trace_polynomials();

    let trace_length = trace_polynomials[0].len();

    let beta = 16usize;
    let evaluation_length = trace_length * beta;
    let evaluation_generator = FieldElement::root(U256::from(evaluation_length as u64)).unwrap();
    let evaluation_offset = FieldElement::GENERATOR;

    let mut extended_trace_table: MmapVec<[FieldElement; 8]> =
        MmapVec::with_capacity(evaluation_length);

    for i in 0..beta {
        let mut coset_leaves: Vec<Vec<FieldElement>> = vec![Vec::with_capacity(trace_length); 8];
        trace_polynomials
            .clone()
            .into_par_iter()
            .map(|p| {
                let reverse_i = i.bit_reverse() >> (64 - 4);
                let cofactor =
                    &evaluation_offset * evaluation_generator.pow(U256::from(reverse_i as u64));
                bit_reversal_fft_cofactor(&p, &cofactor)
            })
            .collect_into_vec(&mut coset_leaves);

        for j in 0..trace_length {
            extended_trace_table.push([
                coset_leaves[0][j].clone(),
                coset_leaves[1][j].clone(),
                coset_leaves[2][j].clone(),
                coset_leaves[3][j].clone(),
                coset_leaves[4][j].clone(),
                coset_leaves[5][j].clone(),
                coset_leaves[6][j].clone(),
                coset_leaves[7][j].clone(),
            ])
        }
    }
    extended_trace_table
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::merkle::make_tree;
    use hex_literal::*;

    #[test]
    fn merkle_root_is_correct() {
        let extended_trace_table = get_extended_trace_table();
        let merkle_tree = make_tree(&extended_trace_table);
        assert_eq!(
            merkle_tree[1],
            hex!("b00a4c7f03959e01df2504fb73d2b238a8ab08b2000000000000000000000000")
        );
    }
}
