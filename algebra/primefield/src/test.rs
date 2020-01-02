use crate::{Parameters, PrimeField, UInt};
use quickcheck::{Arbitrary, Gen};

impl<U, P> Arbitrary for PrimeField<P>
where
    U: UInt + Arbitrary,
    P: Parameters<UInt = U>,
{
    #[inline(always)]
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Self::from_uint_reduce(&U::arbitrary(g))
    }
}

// TODO: Proptest
