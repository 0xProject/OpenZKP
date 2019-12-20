use crate::FieldElement;
use rand::{
    distributions::{Distribution, Standard, Uniform},
    Rng,
};
use zkp_u256::U256;

/// Draw from a uniform distribution over all values.
impl Distribution<FieldElement> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FieldElement {
        let uniform = Uniform::new(U256::ZERO, FieldElement::MODULUS);
        FieldElement::from_montgomery(uniform.sample(rng))
    }
}
