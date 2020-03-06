// We want these functions to be called `fft`
#![allow(clippy::module_name_repetitions)]
// Many false positives from trait bounds
#![allow(single_use_lifetimes)]
use crate::{
    geometric_series::root_series, transpose::transpose_inplace, FieldLike, Inv, Pow, RefFieldLike,
};
use rayon::prelude::*;
use std::{mem::size_of, prelude::v1::*};
use zkp_macros_decl::field_element;
use zkp_u256::U256;

// OPT: Implement parallel strategies: https://inf.ethz.ch/personal/markusp/teaching/263-2300-ETH-spring12/slides/class19.pdf

// TODO: Implement "A modified split-radix FFT with fewer arithmetic operations"
// See http://www.fftw.org/newsplit.pdf

// TODO: Winograd FFT? https://pdfs.semanticscholar.org/cdfc/fed48f6f7e26a2986df8890f3f67087336d5.pdf

/// <https://www.csd.uwo.ca/~moreno/CS433-CS9624/Resources/Implementing_FFTs_in_Practice.pdf>

// https://ocw.mit.edu/courses/electrical-engineering-and-computer-science/6-973-communication-system-design-spring-2006/lecture-notes/lecture_8.pdf

// TODO: Create a dedicated type for permuted vectors

/// Permute index for an FFT of `size`
///
/// The permutation is it's own inverse. The permutation is currently
/// a 'bit-reversal' one, where each index has its binary representation
/// reversed.
pub fn permute_index(size: usize, index: usize) -> usize {
    const USIZE_BITS: usize = 0_usize.count_zeros() as usize;
    debug_assert!(index < size);
    if size == 1 {
        0
    } else {
        debug_assert!(size.is_power_of_two());
        let bits = size.trailing_zeros() as usize;
        index.reverse_bits() >> (USIZE_BITS - bits)
    }
}

/// Permute an array of FFT results.
// TODO expose public ifft function which accepts bit-reversed input instead.
pub fn permute<T>(v: &mut [T]) {
    let n = v.len();
    for i in 0..n {
        let j = permute_index(n, i);
        if j > i {
            v.swap(i, j);
        }
    }
}

/// Out-of-place FFT with non-permuted result.
pub fn fft<Field>(a: &[Field]) -> Vec<Field>
where
    Field: FieldLike + From<usize> + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let mut result = a.to_owned();
    fft_permuted(&mut result);
    permute(&mut result);
    result
}

/// Out-of-place inverse FFT with non-permuted result.
pub fn ifft<Field>(a: &[Field]) -> Vec<Field>
where
    Field: FieldLike + From<usize> + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let mut result = a.to_owned();
    ifft_permuted(&mut result);
    permute(&mut result);
    result
}

/// In-place permuted FFT.
pub fn fft_permuted<Field>(x: &mut [Field])
where
    Field: FieldLike + From<usize> + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let root = Field::root(x.len()).expect("No root of unity for input length");
    fft_permuted_root(&root, x);
}

/// Out-of-place permuted FFT with a cofactor.
pub fn fft_cofactor_permuted_out<Field>(cofactor: &Field, x: &[Field], out: &mut [Field])
where
    Field: FieldLike + From<usize> + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    // TODO: Use geometric_series
    let mut c = Field::one();
    for (x, out) in x.iter().zip(out.iter_mut()) {
        *out = x * &c;
        c *= cofactor;
    }
    fft_permuted(out);
}

/// In-place permuted FFT with a cofactor.
pub fn fft_cofactor_permuted<Field>(cofactor: &Field, x: &mut [Field])
where
    Field: FieldLike + From<usize> + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    // TODO: Use geometric_series
    let mut c = Field::one();
    for element in x.iter_mut() {
        *element *= &c;
        c *= cofactor;
    }
    fft_permuted(x);
}

/// In-place permuted inverse FFT with cofactor.
pub fn ifft_permuted<Field>(x: &mut [Field])
where
    Field: FieldLike + From<usize> + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    // OPT: make inv_root function.
    let inverse_root = Field::root(x.len())
        .expect("No root of unity for input length")
        .inv()
        .expect("No inverse for Field::zero()");
    let inverse_length = Field::from(x.len())
        .inv()
        .expect("No inverse length for empty list");
    fft_permuted_root(&inverse_root, x);
    for e in x {
        *e *= &inverse_length;
    }
}

// TODO: Cache-oblivious FFT
// See https://www.csd.uwo.ca/~moreno/CS433-CS9624/Resources/Implementing_FFTs_in_Practice.pdf
// See https://cs.uwaterloo.ca/~imunro/cs840/Notes16/frigo.pdf
// My `sysctl hw` cache sizes: 32kiB, 256kiB, 8MiB, or 1k, 8k, 256k
// FieldElements.

// TODO: https://cnx.org/contents/4kChocHM@6/Efficient-FFT-Algorithm-and-Programming-Tricks

// TODO: Radix-4 and/or Split-radix FFT
// See https://en.wikipedia.org/wiki/Split-radix_FFT_algorithm
// See http://www.fftw.org/newsplit.pdf

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
                radix_2(i, block_size, coefficients);
            }
            twiddle_factor *= &twiddle_factor_update;
        }
    }
}

/// Depth-first in-place bit-reversed FFT.
pub fn fft_depth_first<Field>(values: &mut [Field])
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    depth_first_recurse(values, 0, 1);
}

fn depth_first_recurse<Field>(values: &mut [Field], offset: usize, stride: usize)
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let size = values.len() / stride;
    let half = size / 2;
    debug_assert!(size.is_power_of_two());
    debug_assert!(offset < stride);
    debug_assert_eq!(values.len() % size, 0);
    if size > 1 {
        depth_first_recurse(values, offset, stride * 2);
        depth_first_recurse(values, offset + stride, stride * 2);
        let mut twiddle = Field::one();
        let root = Field::root(size).expect("No root found");
        for i in (0..size).step_by(2) {
            let twiddle = root.pow(permute_index(half, i / 2));
            let i = offset + i * stride;
            let j = i + stride;
            let a = values[i].clone();
            let b = twiddle * &values[j];
            values[i] = &a + &b;
            values[j] = a - b;
        }
    }
}

pub fn fft2<Field>(values: &[Field]) -> Vec<Field>
where
    Field: FieldLike + std::fmt::Debug + From<usize> + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    assert!(values.len().is_power_of_two());
    let root = Field::root(values.len()).expect("No root of unity for input length");
    let mut result = values.to_vec();
    radix_sqrt(&mut result, &root);
    // permute(&mut result);
    result
}

pub fn fft2_inplace<Field>(values: &mut [Field])
where
    Field: FieldLike + std::fmt::Debug + From<usize> + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    assert!(values.len().is_power_of_two());
    let root = Field::root(values.len()).expect("No root of unity for input length");
    radix_sqrt(values, &root);
}

// See https://github.com/awelkie/RustFFT

/// In-place FFT with permuted output.
///
/// Implement's the four step FFT in a cache-oblivious manner.
///
/// * D. H. Bailey (1990). FFTs in external or hierarchical memory. <https://www.davidhbailey.com/dhbpapers/fftq.pdf>
/// * W. M. Gentleman & G. Sande (1966). Fast Fourier Transforms: for fun and
///   profit. <https://doi.org/10.1145/1464291.1464352> <http://cis.rit.edu/class/simg716/FFT_Fun_Profit.pdf>
/// * M. Frigo, C.E. Leiserson, H. Prokop & S. Ramachandran (1999).
///   Cache-oblivious algorithms. <http://supertech.csail.mit.edu/papers/FrigoLePr99.pdf>
/// * S. Johnson, M. Frigo (2005). The Design and Implementation of FFTW3. <http://www.fftw.org/fftw-paper-ieee.pdf>
/// * S. Johnson, M. Frigo (2012). Implementing FFTs in Practice. <https://cnx.org/contents/ulXtQbN7@15/Implementing-FFTs-in-Practice>
///   <https://www.csd.uwo.ca/~moreno/CS433-CS9624/Resources/Implementing_FFTs_in_Practice.pdf>
///
/// <https://doi.org/10.1007/978-981-13-9965-7_6>
/// <https://eprint.iacr.org/2016/504.pdf>
///
/// There is also a six-step version that outputs the result in normal order,
/// for this see <http://wwwa.pikara.ne.jp/okojisan/otfft-en/sixstepfft.html>.
// TODO: Bit-reversed order
pub fn radix_sqrt<Field>(values: &mut [Field], root: &Field)
where
    Field: FieldLike + std::fmt::Debug + From<usize> + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    // Recurse by splitting along the square root
    let length = values.len();
    let outer = 1_usize << (length.trailing_zeros() / 2);
    let inner = length / outer;
    debug_assert!(outer == inner || inner == 2 * outer);
    debug_assert_eq!(outer * inner, length);
    let inner_root = root.pow(outer);
    let outer_root = root.pow(inner);
    parallel_recurse_inplace_permuted(
        values,
        root,
        outer,
        inner,
        |row| fft_permuted_root(&inner_root, row),
        |row| fft_permuted_root(&outer_root, row),
    );
}

/// Generic recursive six-point FFT.
fn recurse_inplace_inorder<Field, F, G>(
    values: &mut [Field],
    root: &Field,
    outer: usize,
    inner: usize,
    inner_fft: F,
    outer_fft: G,
) where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
    F: Fn(&mut [Field]),
    G: Fn(&mut [Field]),
{
    let length = values.len();
    debug_assert!(root.pow(length).is_one());
    debug_assert_eq!(outer * inner, length);

    // 1 Transpose inner * outer sized matrix
    transpose_inplace(values, outer);

    // 2 Apply inner FFTs continguously
    // 3 Apply twiddle factors
    values
        .chunks_exact_mut(inner)
        .enumerate()
        .for_each(|(j, row)| {
            inner_fft(row);
            if j > 0 {
                let outer_twiddle = root.pow(j);
                let mut inner_twiddle = outer_twiddle.clone();
                for x in row.iter_mut().skip(1) {
                    *x *= &inner_twiddle;
                    inner_twiddle *= &outer_twiddle;
                }
            }
        });

    // 4 Transpose outer * inner sized matrix
    transpose_inplace(values, inner);

    // 5 Apply outer FFTs contiguously
    values
        .chunks_exact_mut(outer)
        .for_each(|row| outer_fft(row));

    // 6 Transpose back to get results in output order
    transpose_inplace(values, outer);
}

/// Generic parallel recursive six-point FFT.
fn parallel_recurse_inplace_inorder<Field, F, G>(
    values: &mut [Field],
    root: &Field,
    outer: usize,
    inner: usize,
    inner_fft: F,
    outer_fft: G,
) where
    Field: FieldLike + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
    F: Fn(&mut [Field]) + Sync,
    G: Fn(&mut [Field]) + Sync,
{
    let length = values.len();
    debug_assert!(root.pow(length).is_one());
    debug_assert_eq!(outer * inner, length);

    // 1 Transpose inner * outer sized matrix
    transpose_inplace(values, outer);

    // 2 Apply inner FFTs continguously
    // 3 Apply twiddle factors
    let inner_root = root.pow(outer);
    values
        .par_chunks_mut(inner)
        .enumerate()
        .for_each(|(j, row)| {
            inner_fft(row);
            if j > 0 {
                let outer_twiddle = root.pow(j);
                let mut inner_twiddle = outer_twiddle.clone();
                for x in row.iter_mut().skip(1) {
                    *x *= &inner_twiddle;
                    inner_twiddle *= &outer_twiddle;
                }
            }
        });

    // 4 Transpose outer * inner sized matrix
    transpose_inplace(values, inner);

    // 5 Apply outer FFTs contiguously
    values.par_chunks_mut(outer).for_each(|row| outer_fft(row));

    // 6 Transpose back to get results in output order
    transpose_inplace(values, outer);
}

/// Generic recursive six-point FFT with permuted output.
///
/// Advantages:
///  * The inner and outer FFT functions can be in permuted order
///  * Only two transpositions are required instead of three.
fn recurse_inplace_permuted<Field, F, G>(
    values: &mut [Field],
    root: &Field,
    outer: usize,
    inner: usize,
    inner_fft: F,
    outer_fft: G,
) where
    Field: FieldLike + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
    F: Fn(&mut [Field]) + Sync,
    G: Fn(&mut [Field]) + Sync,
{
    let length = values.len();
    debug_assert!(root.pow(length).is_one());
    debug_assert_eq!(outer * inner, length);

    // 1 Transpose inner * outer sized matrix
    transpose_inplace(values, outer);

    // 2 Apply inner FFTs continguously
    // 3 Apply twiddle factors
    let inner_root = root.pow(outer);
    values
        .chunks_exact_mut(inner)
        .enumerate()
        .for_each(|(j, row)| {
            inner_fft(row);
            if j > 0 {
                let outer_twiddle = root.pow(j);
                for (i, x) in row.iter_mut().enumerate() {
                    let i = permute_index(inner, i);
                    let inner_twiddle = outer_twiddle.pow(i);
                    *x *= inner_twiddle;
                }
            }
        });

    // 4 Transpose outer * inner sized matrix
    transpose_inplace(values, inner);

    // 5 Apply outer FFTs contiguously
    values
        .chunks_exact_mut(outer)
        .for_each(|row| outer_fft(row));
}

/// Generic recursive six-point FFT with permuted output.
///
/// Advantages:
///  * The inner and outer FFT functions can be in permuted order
///  * Only two transpositions are required instead of three.
fn parallel_recurse_inplace_permuted<Field, F, G>(
    values: &mut [Field],
    root: &Field,
    outer: usize,
    inner: usize,
    inner_fft: F,
    outer_fft: G,
) where
    Field: FieldLike + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
    F: Fn(&mut [Field]) + Sync,
    G: Fn(&mut [Field]) + Sync,
{
    let length = values.len();
    debug_assert!(root.pow(length).is_one());
    debug_assert_eq!(outer * inner, length);

    // 1 Transpose inner * outer sized matrix
    transpose_inplace(values, outer);

    // 2 Apply inner FFTs continguously
    // 3 Apply twiddle factors
    let inner_root = root.pow(outer);
    values
        .par_chunks_mut(inner)
        .enumerate()
        .for_each(|(j, row)| {
            inner_fft(row);
            if j > 0 {
                let outer_twiddle = root.pow(j);
                let mut inner_twiddle = outer_twiddle.clone();
                for i in 1..inner {
                    let i = permute_index(inner, i);
                    row[i] *= &inner_twiddle;
                    inner_twiddle *= &outer_twiddle;
                }
            }
        });

    // 4 Transpose outer * inner sized matrix
    transpose_inplace(values, inner);

    // 5 Apply outer FFTs contiguously
    values.par_chunks_mut(outer).for_each(|row| outer_fft(row));
}

/// Transforms (x0, x1) to (x0 + x1, x0 - x1)
#[inline(always)]
pub fn radix_2_simple<Field>(x0: &mut Field, x1: &mut Field)
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    // OPT: Inplace +- operation like in gcd::mat_mul.
    // OPT: Use Dev's combined REDC

    let t = x0.clone();
    *x0 += &*x1;
    // OPT: sub_from_assign
    *x1 -= t;
    x1.neg_assign();
}

/// Transforms (x0, x1) to (x0 + x1, x0 - x1)
#[inline(always)]
pub fn radix_2<Field>(offset: usize, stride: usize, values: &mut [Field])
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    // OPT: Inplace +- operation like in gcd::mat_mul.
    // OPT: Use Dev's combined REDC

    let (left, right) = values.split_at_mut(offset + stride);
    let t = left[offset].clone();
    left[offset] += &right[0];
    // OPT: sub_from_assign
    right[0] -= t;
    right[0].neg_assign();
}

// See https://math.stackexchange.com/questions/1626897/whats-the-formulation-of-n-point-radix-n-for-ntt/1627247
#[inline(always)]
pub fn radix_4<Field>(offset: usize, stride: usize, values: &mut [Field])
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let omega = Field::root(4).expect("No root of order 4 found");
    radix_2(0, 2, values);
    radix_2(1, 2, values);
    values[offset + 3 * stride] *= omega;
    radix_2(0, 1, values);
    radix_2(2, 1, values);
}

#[inline(always)]
pub fn radix_8<Field>(offset: usize, stride: usize, values: &mut [Field])
where
    Field: FieldLike + std::fmt::Debug,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    let omega = Field::root(4).expect("No root of order 4 found");
    radix_4(0, 2, values);
    radix_4(1, 2, values);
    values[offset + 3 * stride] *= omega;
    radix_2(0, 1, values);
    radix_2(2, 1, values);
    radix_2(2, 1, values);
    radix_2(2, 1, values);
}

// Quickcheck needs pass by value
#[allow(clippy::needless_pass_by_value)]
// We don't care about this in tests
#[allow(clippy::redundant_clone)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FieldElement, One, Root, Zero};
    use proptest::prelude::*;
    use quickcheck_macros::quickcheck;
    use std::cmp::{max, min};
    use zkp_macros_decl::u256h;
    use zkp_u256::U256;

    fn arb_elem() -> impl Strategy<Value = FieldElement> {
        (any::<u64>(), any::<u64>(), any::<u64>(), any::<u64>())
            .prop_map(move |(a, b, c, d)| FieldElement::from(U256::from_limbs([a, b, c, d])))
    }

    // Generate a power-of-two size
    fn arb_vec() -> impl Strategy<Value = Vec<FieldElement>> {
        (0_usize..=9).prop_flat_map(|size| prop::collection::vec(arb_elem(), 1_usize << size))
    }

    // O(n^2) reference implementation evaluating
    //     x_i' = Sum_j x_j * omega_n^(ij)
    // directly using Horner's method.
    fn reference_fft(x: &[FieldElement], inverse: bool) -> Vec<FieldElement> {
        let mut root = FieldElement::root(x.len()).unwrap();
        if inverse {
            root = root.inv().expect("Root should be invertible.");
        }
        let mut result = Vec::with_capacity(x.len());
        let mut root_i = FieldElement::one();
        for _ in 0..x.len() {
            let mut sum = FieldElement::zero();
            let mut root_ij = FieldElement::one();
            for xj in x {
                sum += xj * &root_ij;
                root_ij *= &root_i;
            }
            result.push(sum);
            root_i *= &root;
        }
        if inverse {
            if let Some(inverse_length) = FieldElement::from(x.len()).inv() {
                for x in &mut result {
                    *x *= &inverse_length;
                }
            }
        }
        result
    }

    fn ref_fft_inplace(values: &mut [FieldElement]) {
        let result = reference_fft(values, false);
        values.clone_from_slice(&result);
    }

    fn ref_fft_permuted(values: &mut [FieldElement]) {
        let result = reference_fft(values, false);
        values.clone_from_slice(&result);
        permute(values);
    }

    proptest! {

        #[test]
        fn fft_ref_inv(orig in arb_vec()) {
            let f = reference_fft(&orig, false);
            let mut f2 = reference_fft(&f, true);
            prop_assert_eq!(f2, orig);
        }

        #[test]
        fn test_recurse_inplace_inorder(orig in arb_vec()) {
            // TODO: Test different splittings
            const SPLIT: usize = 4;
            let reference = reference_fft(&orig, false);
            let root = FieldElement::root(orig.len()).unwrap();
            let mut result = orig.clone();
            let inner = max(1, orig.len() / SPLIT);
            let outer = min(orig.len(), SPLIT);
            recurse_inplace_inorder(
                &mut result,
                &root,
                outer,
                inner,
                ref_fft_inplace,
                ref_fft_inplace,
            );
            prop_assert_eq!(result, reference);
        }

        #[test]
        fn test_recurse_inplace_permuted(orig in arb_vec()) {
            // TODO: Test different splittings
            const SPLIT: usize = 4;
            let mut expected = orig.clone();
            ref_fft_permuted(&mut expected);
            let root = FieldElement::root(orig.len()).unwrap();
            let mut result = orig.clone();
            let inner = max(1, orig.len() / SPLIT);
            let outer = min(orig.len(), SPLIT);
            recurse_inplace_permuted(
                &mut result,
                &root,
                outer,
                inner,
                ref_fft_permuted,
                ref_fft_permuted,
            );
            prop_assert_eq!(result, expected);
        }

        #[test]
        fn test_parallel_recurse_inplace_inorder(orig in arb_vec()) {
            // TODO: Test different splittings
            const SPLIT: usize = 4;
            let reference = reference_fft(&orig, false);
            let root = FieldElement::root(orig.len()).unwrap();
            let mut result = orig.clone();
            let inner = max(1, orig.len() / SPLIT);
            let outer = min(orig.len(), SPLIT);
            parallel_recurse_inplace_inorder(
                &mut result,
                &root,
                outer,
                inner,
                ref_fft_inplace,
                ref_fft_inplace,
            );
            prop_assert_eq!(result, reference);
        }

        #[test]
        fn fft2_ref(orig in arb_vec()) {
            let reference = reference_fft(&orig, false);
            let result = fft2(&orig);
            prop_assert_eq!(result, reference);
        }

        #[test]
        fn fft_df_ref(orig in arb_vec()) {
            let mut reference = orig.clone();
            let mut result = orig.clone();
            ref_fft_permuted(&mut reference);
            fft_depth_first(&mut result);
            prop_assert_eq!(result, reference);
        }
    }

    #[test]
    fn test_permute() {
        assert_eq!(permute_index(4, 0), 0);
        assert_eq!(permute_index(4, 1), 2);
        assert_eq!(permute_index(4, 2), 1);
        assert_eq!(permute_index(4, 3), 3);
    }

    #[quickcheck]
    fn check_permute(size: usize, index: usize) {
        let size = size.next_power_of_two();
        let index = index % size;
        let permuted = permute_index(size, index);
        assert!(permuted < size);
        assert_eq!(permute_index(size, permuted), index);
    }

    #[test]
    fn test_radix_2() {
        let mut x = [
            field_element!("0234287dcbaffe7f969c748655fca9e58fa8120b6d56eb0c1080d17957ebe47b"),
            field_element!("06c81c707ecc44b5f60297ec08d2d585513c1ba022dd93af66a1dbacb162a3f3"),
        ];
        radix_2(0, 1, &mut x);
        assert_eq!(x, [
            field_element!("00fc44ee4a7c43248c9f0c725ecf7f6ae0e42dab90347ebb7722ad26094e886d"),
            field_element!("036c0c0d4ce3b9daa099dc9a4d29d4603e6bf66b4a79575ca9def5cca6894089")
        ]);
    }
    #[test]
    fn fft_test() {
        let cofactor = FieldElement::from(u256h!(
            "07696b8ff70e8e9285c76bef95d3ad76cdb29e213e4b5d9a9cd0afbd7cb29b5c"
        ));
        let vector = [
            FieldElement::from(u256h!(
                "008ee28fdbe9f1a7983bc1b600dfb9177c2d82d825023022ab4965d999bd3faf"
            )),
            FieldElement::from(u256h!(
                "037fa3db272cc54444894042223dcf260e1d1ec73fa9baea0e4572817fdf5751"
            )),
            FieldElement::from(u256h!(
                "054483fc9bcc150b421fae26530f8d3d2e97cf1918f534e67ef593038f683241"
            )),
            FieldElement::from(u256h!(
                "005b695b9001e5e62549557c48a23fd7f1706c1acdae093909d81451cd455b43"
            )),
            FieldElement::from(u256h!(
                "025079cb6cb547b63b67614dd2c78474c8a7b17b3bc53f7f7276984b6b67b18a"
            )),
            FieldElement::from(u256h!(
                "044729b25360c0025d244d31a5f144917e59f728a3d03dd4685c634d2b0e7cda"
            )),
            FieldElement::from(u256h!(
                "079b0e14d0bae81ff4fe55328fb09c4117bcd961cb60581eb6f2a770a42240ed"
            )),
            FieldElement::from(u256h!(
                "06c0926a786abb30b8f6e0eb9ef2278b910862717ed4beb35121d4741717e0e0"
            )),
        ];

        let res = fft(&vector);
        let expected = reference_fft(&vector, false);
        assert_eq!(res, expected);

        assert_eq!(
            U256::from(&res[0]),
            u256h!("06a1b7c038205cb38aaeea38662ae2259a19c14a7519bd522543f72dc7fa74b2")
        );
        assert_eq!(
            U256::from(&res[1]),
            u256h!("017884f169b20153de79a9c642d4e3259263f2e7ac5f85f5a8191f28d8f14544")
        );
        assert_eq!(
            U256::from(&res[2]),
            u256h!("03112a352e474819d491a13b700a07161eee580ff40098df978fa19f39b4fd2d")
        );
        assert_eq!(
            U256::from(&res[3]),
            u256h!("011606a821f418d13914c72b424141c5b88bdb184b0b5a55fc537587346c78a2")
        );
        assert_eq!(
            U256::from(&res[4]),
            u256h!("00dc2519322c102b8ad3628106a3ebef7c39f85215203bfc820c7a04a9645419")
        );
        assert_eq!(
            U256::from(&res[5]),
            u256h!("01df6a70d033d89376c96c45ce8dbbe4eeedce2d32636c29d3cb87b9e2074d00")
        );
        assert_eq!(
            U256::from(&res[6]),
            u256h!("00ee6a5e89e9307e64789e1a71c42105de12bfa104e32c5a381fe5c2697ffeec")
        );
        assert_eq!(
            U256::from(&res[7]),
            u256h!("048bad0760f8b52ee4f9a46964bcf1ba9439a9467b2576176b1319cec9f12db0")
        );

        let mut res = vector.clone();
        fft_cofactor_permuted(&cofactor, &mut res);
        permute(&mut res);

        assert_eq!(
            U256::from(&res[0]),
            u256h!("05d817ee1af8beff1880aad163a9912704d66e0c717a670c52db93da5ea34455")
        );
        assert_eq!(
            U256::from(&res[1]),
            u256h!("0631b16aceb1ee5711066df1ffafd9f5f451b0dc44c86e90005bc78e8bb4f861")
        );
        assert_eq!(
            U256::from(&res[2]),
            u256h!("01a30c98c149179cd16059ba201b99cf629d3e04844a50936006a185a67ad354")
        );
        assert_eq!(
            U256::from(&res[3]),
            u256h!("07a17b9035ff1ffd1f9e0bc52982effcd957bc07230830c10e51e906ed092f9e")
        );
        assert_eq!(
            U256::from(&res[4]),
            u256h!("01381787eccc6c77b0c5dff0b4b66dc0bb7d911bd705baf85f62001976e6ff27")
        );
        assert_eq!(
            U256::from(&res[5]),
            u256h!("009defa0822d287ce55035bb705319eb34e78180157e5297e6a46df9af8ef042")
        );
        assert_eq!(
            U256::from(&res[6]),
            u256h!("020b8317360c61abbc0bdce513eb42295402eb5dde3d13abfc0325f277f507bc")
        );
        assert_eq!(
            U256::from(&res[7]),
            u256h!("034738bd5956b1df55369cdc211109fd67e6ffd2ffbb08e856b1b4d1b1a2c6ae")
        );
    }

    #[quickcheck]
    fn ifft_is_inverse(v: Vec<FieldElement>) -> bool {
        if v.is_empty() {
            return true;
        }
        let truncated = &v[0..(1 + v.len()).next_power_of_two() / 2];
        truncated.to_vec() == ifft(&fft(truncated))
    }

    #[quickcheck]
    fn ifft_permuted_is_inverse(v: Vec<FieldElement>) {
        if v.is_empty() {
            return;
        }
        let original = &v[0..(1 + v.len()).next_power_of_two() / 2];
        let mut copy = original.to_owned();

        // TODO: Make it work without the permutes in between
        fft_permuted(&mut copy);
        permute(&mut copy);
        ifft_permuted(&mut copy);
        permute(&mut copy);

        assert_eq!(copy, original)
    }
}
