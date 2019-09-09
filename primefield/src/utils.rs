#[macro_export]
macro_rules! require {
    ($condition:expr, $err:expr) => {
        if !$condition {
            return Err($err);
        }
    };
}

pub trait Reversible {
    fn bit_reverse(self) -> Self;
    fn bit_reverse_at(self, len: usize) -> Self;
}

impl Reversible for u64 {
    fn bit_reverse(mut self) -> Self {
        const BITS: usize = 64;
        debug_assert_eq!(1_u64.leading_zeros() as usize, BITS - 1);
        let mut reversed = 0;
        for _i in 0..BITS {
            reversed <<= 1;
            reversed |= self & 1;
            self >>= 1;
        }
        reversed
    }

    fn bit_reverse_at(self, len: usize) -> Self {
        self.bit_reverse() >> (len.leading_zeros() + 1)
    }
}

// False positive, const can not use Self
#[allow(clippy::use_self)]
impl Reversible for usize {
    fn bit_reverse(mut self) -> Self {
        const BITS: usize = 64;
        debug_assert_eq!(1_usize.leading_zeros() as Self, BITS - 1);
        let mut reversed = 0;
        for _i in 0..BITS {
            reversed <<= 1;
            reversed |= self & 1;
            self >>= 1;
        }
        reversed
    }

    fn bit_reverse_at(self, len: usize) -> Self {
        self.bit_reverse() >> (len.leading_zeros() + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[test]
    fn usize_bit_reverse() {
        assert_eq!(0_usize.bit_reverse(), 0);
        assert_eq!(1_usize.bit_reverse(), 1 << 63);
        assert_eq!(2_usize.bit_reverse(), 1 << 62);
        assert_eq!(3_usize.bit_reverse(), 3 << 62);
        assert_eq!(4_usize.bit_reverse(), 1 << 61);
    }

    #[quickcheck]
    fn usize_bit_reverse_is_involution(i: usize) -> bool {
        i == i.bit_reverse().bit_reverse()
    }

    #[quickcheck]
    fn u64_bit_reverse_is_involution(i: usize) -> bool {
        i == i.bit_reverse().bit_reverse()
    }
}
