use crate::U256;
use rand::{
    distributions::{
        uniform::{SampleBorrow, SampleUniform, UniformSampler},
        Distribution, Standard,
    },
    Rng,
};

/// Draw from a uniform distribution over all values.
impl Distribution<U256> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> U256 {
        U256::from_limbs(rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }
}

/// Helper struct for uniform sampling using `rand`
#[derive(Clone, Debug)]
pub enum UniformU256 {
    Full,
    Ranged { low: U256, range: U256 },
}

impl SampleUniform for U256 {
    type Sampler = UniformU256;
}

impl UniformSampler for UniformU256 {
    type X = U256;

    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<U256> + Sized,
        B2: SampleBorrow<U256> + Sized,
    {
        let low = low.borrow().clone();
        let range = high.borrow() - &low;
        Self::Ranged { low, range }
    }

    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<U256> + Sized,
        B2: SampleBorrow<U256> + Sized,
    {
        if low.borrow() == &U256::ZERO && high.borrow() == &U256::MAX {
            Self::Full
        } else {
            let low = low.borrow().clone();
            let range = high.borrow() - &low + U256::ONE;
            Self::Ranged { low, range }
        }
    }

    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> U256 {
        match self {
            Self::Full => rng.gen(),
            Self::Ranged { low, range } => {
                // Strategy: bitshift to within 2x then rejection sample.
                let shift = range.leading_zeros();
                let mut result = U256::MAX;
                while result >= *range {
                    result = rng.gen();
                    result >>= shift;
                }
                result + low
            }
        }
    }
}

// TODO: Tests
