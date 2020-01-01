use crate::{FieldParameters, PrimeField, UInt as FieldUInt};
use rand::{
    distributions::{uniform::SampleUniform, Distribution, Standard, Uniform},
    Rng,
};

/// Draw from a uniform distribution over all values.
///
/// Requires `UInt` to implement [`SampleUniform`].
impl<UInt, Parameters> Distribution<PrimeField<UInt, Parameters>> for Standard
where
    UInt: FieldUInt + SampleUniform,
    Parameters: FieldParameters<UInt>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PrimeField<UInt, Parameters> {
        let uniform = Uniform::new(UInt::zero(), Parameters::MODULUS);
        PrimeField::<UInt, Parameters>::from_montgomery(uniform.sample(rng))
    }
}
