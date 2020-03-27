use crate::{u256::U256, Parameters, PrimeField};
use quickcheck::{Arbitrary, Gen};

impl<P> Arbitrary for PrimeField<P>
where
    P: Parameters<UInt = U256>,
{
    #[inline(always)]
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Self::from_uint_reduce(&U256::from_limbs([
            u64::arbitrary(g),
            u64::arbitrary(g),
            u64::arbitrary(g),
            u64::arbitrary(g),
        ]))
    }
}

// TODO: Proptest
