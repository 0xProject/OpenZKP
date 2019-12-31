use crate::{Field, FieldParameters, FieldUInt};
use rand::{
    distributions::{uniform::SampleUniform, Distribution, Standard, Uniform},
    Rng,
};

/// Draw from a uniform distribution over all values.
///
/// Requires `UInt` to implement
/// [`rand::distributions::uniform::SampleUniform`].
impl<UInt, Parameters> Distribution<Field<UInt, Parameters>> for Standard
where
    UInt: FieldUInt + SampleUniform,
    Parameters: FieldParameters<UInt>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Field<UInt, Parameters> {
        let uniform = Uniform::new(UInt::zero(), Parameters::MODULUS);
        Field::<UInt, Parameters>::from_montgomery(uniform.sample(rng))
    }
}
