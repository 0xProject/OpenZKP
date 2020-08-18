// We want these functions to be called `fft`
#![allow(clippy::module_name_repetitions)]
// Many false positives from trait bounds
#![allow(single_use_lifetimes)]

// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::prelude::v1::*;

mod bit_reverse;
mod prefetch;
mod recursive;
pub mod small;
mod transpose;

#[cfg(feature = "std")]
mod radix_sqrt;

use crate::{Fft, FieldLike, Inv, Pow, RefFieldLike};
use log::trace;
#[cfg(feature = "rayon")]
use rayon::{current_num_threads, prelude::*};
#[cfg(feature = "rayon")]
use std::cmp::max;

// Re-exports
// TODO: Only re-export for bench
pub use bit_reverse::{permute, permute_index};
#[cfg(feature = "memadvise")]
pub use memadvise::Advice;
#[cfg(feature = "memadvise")]
pub use prefetch::MemoryAdvise;
pub use prefetch::{Prefetch, PrefetchIndex};
#[cfg(feature = "std")]
pub use radix_sqrt::radix_sqrt;
pub use recursive::fft_vec_recursive;
pub use transpose::transpose_square_stretch;

/// Relevant papers:
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

// OPT: Implement parallel strategies: https://inf.ethz.ch/personal/markusp/teaching/263-2300-ETH-spring12/slides/class19.pdf

// TODO: Implement "A modified split-radix FFT with fewer arithmetic operations"
// See http://www.fftw.org/newsplit.pdf

// TODO: Winograd FFT? https://pdfs.semanticscholar.org/cdfc/fed48f6f7e26a2986df8890f3f67087336d5.pdf

/// <https://www.csd.uwo.ca/~moreno/CS433-CS9624/Resources/Implementing_FFTs_in_Practice.pdf>

// https://ocw.mit.edu/courses/electrical-engineering-and-computer-science/6-973-communication-system-design-spring-2006/lecture-notes/lecture_8.pdf

/// Blanket implementation of [`Fft`] for all slices of a [`FieldLike`]
impl<Field> Fft<Field> for [Field]
where
    Field: FieldLike + From<usize> + Send + Sync,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    fn fft(&mut self) {
        let root = Field::root(self.len()).expect("No root of unity for input length");
        self.fft_root(&root);
    }

    fn ifft(&mut self) {
        let inverse_root = Field::root(self.len())
            .expect("No root of unity for input length")
            .pow(self.len() - 1);
        let inverse_length = Field::from(self.len())
            .inv()
            .expect("No inverse length for empty list");
        self.fft_root(&inverse_root);
        trace!("BEGIN Inverse shift");
        for e in self.iter_mut() {
            *e *= &inverse_length;
        }
        trace!("END Inverse shift");
    }

    #[cfg(not(feature = "rayon"))]
    fn clone_shifted(&mut self, source: &[Field], cofactor: &Field) {
        trace!("BEGIN Clone shifted");
        let mut c = Field::one();
        for (destination, source) in self.iter_mut().zip(source.iter()) {
            *destination = source * &c;
            c *= cofactor;
        }
        trace!("END Clone shifted");
    }

    #[cfg(feature = "rayon")]
    fn clone_shifted(&mut self, source: &[Field], cofactor: &Field) {
        // TODO: Write benchmark and tune chunk size
        const MIN_CHUNK_SIZE: usize = 1024;
        trace!("BEGIN Clone shifted");
        let chunk_size = max(MIN_CHUNK_SIZE, source.len() / current_num_threads());
        let chunks = self
            .par_chunks_mut(chunk_size)
            .zip(source.par_chunks(chunk_size));
        chunks.enumerate().for_each(|(i, (destination, source))| {
            let mut c = cofactor.pow(i * chunk_size);
            for (destination, source) in destination.iter_mut().zip(source.iter()) {
                *destination = source * &c;
                c *= cofactor;
            }
        });
        trace!("END Clone shifted");
    }

    fn fft_cofactor(&mut self, cofactor: &Field) {
        // TODO: This patterns happens often, abstract?
        trace!("BEGIN Cofactor shift");
        let mut c = Field::one();
        for element in self.iter_mut() {
            *element *= &c;
            c *= cofactor;
        }
        trace!("END Cofactor shift");
        self.fft();
    }

    fn ifft_cofactor(&mut self, cofactor: &Field) {
        self.ifft();
        let cofactor = cofactor.inv().expect("Can not invert cofactor");
        trace!("BEGIN Cofactor shift");
        let mut c = Field::one();
        for element in self.iter_mut() {
            *element *= &c;
            c *= &cofactor;
        }
        trace!("END Cofactor shift");
    }

    fn fft_root(&mut self, root: &Field) {
        const RADIX_SQRT_TRESHOLD: usize = 1 << 10;
        if cfg!(feature = "std") && self.len() >= RADIX_SQRT_TRESHOLD {
            #[cfg(feature = "std")]
            radix_sqrt(self, root);
        } else {
            let twiddles = get_twiddles(root, self.len());
            trace!("Recursive FFT of size {}", self.len());
            fft_vec_recursive(self, &twiddles, 0, 1, 1);
        }
    }
}

// TODO: Memoize
pub fn get_twiddles<Field>(root: &Field, size: usize) -> Vec<Field>
where
    Field: FieldLike,
    for<'a> &'a Field: RefFieldLike<Field>,
{
    debug_assert!(size.is_power_of_two());
    debug_assert!(root.pow(size).is_one());
    trace!("BEGIN FFT Twiddles");
    trace!("Computing {} twiddles", size / 2);
    let mut twiddles = (0..size / 2).map(|i| root.pow(i)).collect::<Vec<_>>();
    permute(&mut twiddles);
    trace!("END FFT Twiddles");
    twiddles
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FieldElement, One, Root, Zero};
    use proptest::prelude::*;
    use zkp_macros_decl::field_element;
    use zkp_u256::U256;

    pub(super) fn arb_elem() -> impl Strategy<Value = FieldElement> {
        (any::<u64>(), any::<u64>(), any::<u64>(), any::<u64>())
            .prop_map(move |(a, b, c, d)| FieldElement::from(U256::from_limbs([a, b, c, d])))
    }

    // Generate a power-of-two size
    pub(super) fn arb_vec_size(size: usize) -> impl Strategy<Value = Vec<FieldElement>> {
        prop::collection::vec(arb_elem(), size)
    }

    // Generate a power-of-two size
    pub(super) fn arb_vec() -> impl Strategy<Value = Vec<FieldElement>> {
        (0_usize..=9).prop_flat_map(|size| arb_vec_size(1_usize << size))
    }

    // O(n^2) reference implementation evaluating
    //     x_i' = Sum_j x_j * omega_n^(ij)
    // directly using Horner's method.
    // New lint in nightly
    #[allow(clippy::unknown_clippy_lints)]
    // False positive
    #[allow(clippy::same_item_push)]
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

    #[allow(dead_code)]
    pub(super) fn ref_fft_inplace(values: &mut [FieldElement]) {
        let result = reference_fft(values, false);
        values.clone_from_slice(&result);
    }

    pub(super) fn ref_fft_permuted(values: &mut [FieldElement]) {
        let result = reference_fft(values, false);
        values.clone_from_slice(&result);
        permute(values);
    }

    proptest! {
        #[test]
        fn fft_ref_inv(orig in arb_vec()) {
            let f = reference_fft(&orig, false);
            let f2 = reference_fft(&f, true);
            prop_assert_eq!(f2, orig);
        }

        #[test]
        fn fft_ref(values in arb_vec()) {
            let mut expected = values.clone();
            ref_fft_permuted(&mut expected);
            let mut result = values;
            result.fft();
            prop_assert_eq!(result, expected);
        }
    }

    #[test]
    fn fft_test() {
        let cofactor =
            field_element!("07696b8ff70e8e9285c76bef95d3ad76cdb29e213e4b5d9a9cd0afbd7cb29b5c");
        let vector = vec![
            field_element!("008ee28fdbe9f1a7983bc1b600dfb9177c2d82d825023022ab4965d999bd3faf"),
            field_element!("037fa3db272cc54444894042223dcf260e1d1ec73fa9baea0e4572817fdf5751"),
            field_element!("054483fc9bcc150b421fae26530f8d3d2e97cf1918f534e67ef593038f683241"),
            field_element!("005b695b9001e5e62549557c48a23fd7f1706c1acdae093909d81451cd455b43"),
            field_element!("025079cb6cb547b63b67614dd2c78474c8a7b17b3bc53f7f7276984b6b67b18a"),
            field_element!("044729b25360c0025d244d31a5f144917e59f728a3d03dd4685c634d2b0e7cda"),
            field_element!("079b0e14d0bae81ff4fe55328fb09c4117bcd961cb60581eb6f2a770a42240ed"),
            field_element!("06c0926a786abb30b8f6e0eb9ef2278b910862717ed4beb35121d4741717e0e0"),
        ];

        let mut res = vector.clone();
        res.fft();
        permute(&mut res);
        let expected = reference_fft(&vector, false);
        assert_eq!(res, expected);

        assert_eq!(res, vec![
            field_element!("06a1b7c038205cb38aaeea38662ae2259a19c14a7519bd522543f72dc7fa74b2"),
            field_element!("017884f169b20153de79a9c642d4e3259263f2e7ac5f85f5a8191f28d8f14544"),
            field_element!("03112a352e474819d491a13b700a07161eee580ff40098df978fa19f39b4fd2d"),
            field_element!("011606a821f418d13914c72b424141c5b88bdb184b0b5a55fc537587346c78a2"),
            field_element!("00dc2519322c102b8ad3628106a3ebef7c39f85215203bfc820c7a04a9645419"),
            field_element!("01df6a70d033d89376c96c45ce8dbbe4eeedce2d32636c29d3cb87b9e2074d00"),
            field_element!("00ee6a5e89e9307e64789e1a71c42105de12bfa104e32c5a381fe5c2697ffeec"),
            field_element!("048bad0760f8b52ee4f9a46964bcf1ba9439a9467b2576176b1319cec9f12db0"),
        ]);

        let mut res = vector;
        res.fft_cofactor(&cofactor);
        permute(&mut res);

        assert_eq!(res, vec![
            field_element!("05d817ee1af8beff1880aad163a9912704d66e0c717a670c52db93da5ea34455"),
            field_element!("0631b16aceb1ee5711066df1ffafd9f5f451b0dc44c86e90005bc78e8bb4f861"),
            field_element!("01a30c98c149179cd16059ba201b99cf629d3e04844a50936006a185a67ad354"),
            field_element!("07a17b9035ff1ffd1f9e0bc52982effcd957bc07230830c10e51e906ed092f9e"),
            field_element!("01381787eccc6c77b0c5dff0b4b66dc0bb7d911bd705baf85f62001976e6ff27"),
            field_element!("009defa0822d287ce55035bb705319eb34e78180157e5297e6a46df9af8ef042"),
            field_element!("020b8317360c61abbc0bdce513eb42295402eb5dde3d13abfc0325f277f507bc"),
            field_element!("034738bd5956b1df55369cdc211109fd67e6ffd2ffbb08e856b1b4d1b1a2c6ae"),
        ]);
    }

    proptest!(
        #[test]
        fn ifft_is_inverse(v: Vec<FieldElement>) {
            prop_assume!(!v.is_empty());

            let truncated = &v[0..(1 + v.len()).next_power_of_two() / 2].to_vec();
            let mut result = truncated.clone();
            result.fft();
            permute(&mut result);
            result.ifft();
            permute(&mut result);

            prop_assert_eq!(&result, truncated);
        }
    );
}
