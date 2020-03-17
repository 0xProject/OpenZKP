use crate::{Parameters, PrimeField, UInt};
use rand::{
    distributions::{uniform::SampleUniform, Distribution, Standard, Uniform},
    Rng,
};

/// Draw from a uniform distribution over all values.
///
/// Requires `UInt` to implement [`SampleUniform`].
impl<U, P> Distribution<PrimeField<P>> for Standard
where
    U: UInt + SampleUniform,
    P: Parameters<UInt = U>,
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PrimeField<P> {
        let uniform = Uniform::new(U::zero(), P::MODULUS);
        PrimeField::<P>::from_montgomery(uniform.sample(rng))
    }
}
