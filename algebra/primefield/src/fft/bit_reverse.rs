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
pub fn permute<T>(v: &mut [T]) {
    let n = v.len();
    for i in 0..n {
        let j = permute_index(n, i);
        if j > i {
            v.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_permute() {
        assert_eq!(permute_index(4, 0), 0);
        assert_eq!(permute_index(4, 1), 2);
        assert_eq!(permute_index(4, 2), 1);
        assert_eq!(permute_index(4, 3), 3);
    }

    proptest!(
        #[test]
        fn check_permute(size: usize, index: usize) {
            prop_assume!(size != 0);
            prop_assume!(size <= usize::max_value() / 2);
            let size = size.next_power_of_two();
            let index = index % size;
            let permuted = permute_index(size, index);
            prop_assert!(permuted < size);
            prop_assert_eq!(permute_index(size, permuted), index);
        }
    );
}
