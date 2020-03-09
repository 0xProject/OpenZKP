use super::{bit_reverse::permute_index, small::radix_2};
use crate::{FieldLike, Pow, RefFieldLike};

pub fn fft_permuted_root<Field>(root: &Field, coefficients: &mut [Field])
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let n_elements = coefficients.len();
    debug_assert!(n_elements.is_power_of_two());
    debug_assert_eq!(root.pow(n_elements), Field::one());
    for layer in 0..n_elements.trailing_zeros() {
        let n_blocks = 1_usize << layer;
        let mut twiddle_factor = Field::one();
        let block_size = n_elements >> (layer + 1);
        let twiddle_factor_update = root.pow(block_size);
        for block in 0..n_blocks {
            debug_assert!(block < n_blocks);
            let block_start = 2 * permute_index(n_blocks, block) * block_size;
            for i in block_start..block_start + block_size {
                coefficients[i + block_size] *= &twiddle_factor;
                radix_2(coefficients, i, block_size);
            }
            twiddle_factor *= &twiddle_factor_update;
        }
    }
}
