use crate::{FieldParameters, One, PrimeField, UInt as FieldUInt, Zero};
use quickcheck::{Arbitrary, Gen};

impl<UInt, Parameters> Arbitrary for PrimeField<UInt, Parameters>
where
    UInt: FieldUInt + Arbitrary,
    Parameters: 'static + Send + FieldParameters<UInt>,
{
    #[inline(always)]
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        Self::from_uint_reduce(&UInt::arbitrary(g))
    }
}

// TODO: Proptest
