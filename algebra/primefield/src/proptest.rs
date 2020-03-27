use crate::{uint::UInt, Parameters, PrimeField};
use proptest::prelude::*;

impl<U, P> Arbitrary for PrimeField<P>
where
    U: UInt + Arbitrary,
    U::Strategy: 'static,
    P: Parameters<UInt = U>,
{
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        U::arbitrary()
            .prop_map(|x| Self::from_uint_reduce(&x))
            .boxed()
    }
}
