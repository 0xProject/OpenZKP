use zkp_u256::{U256};
use std::marker::PhantomData;
use std::ops::Add;
use std::fmt;

pub trait Field: Sized + fmt::Debug + Add<Output = Self> {
    const MODULUS: U256;
    const ZERO: Self;
    const ONE: Self;

    fn from_u256(value: U256) -> Self;
}

pub trait PrimeFieldParameters {
    const MODULUS: U256;
}

pub struct PrimeField<F: PrimeFieldParameters> {
    value: U256,
    _field: PhantomData<F>,
}

impl<F: PrimeFieldParameters> Field for PrimeField<F> {
    const MODULUS: U256 = F::MODULUS;
    const ZERO: Self = PrimeField { value: U256::ZERO, _field: PhantomData };
    const ONE: Self = PrimeField { value: U256::ONE, _field: PhantomData };

    fn from_u256(mut value: U256) -> Self {
        value %= Self::MODULUS;
        PrimeField { value, _field: PhantomData }
    }
}

impl<F: PrimeFieldParameters> fmt::Debug for PrimeField<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<F: PrimeFieldParameters> Add for PrimeField<F> {
    type Output = PrimeField<F>;
    fn add(self, other: Self) -> Self::Output {
        // TODO: Overflow
        PrimeField::from_u256(self.value.clone() + &other.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::u256h;

    // Function is completely abstract over the field implementation.
    fn generic_test<F: Field>() {
        let a = F::from_u256(U256::from(5));
        let b = F::from_u256(U256::from(234234323));
        let c = a + b;
        dbg!(&c);
        let c = c + F::ONE;
        dbg!(&c);
    }

    #[test]
    fn test_element() {
        // Create a type F that implements a Prime Field 
        struct SmallFieldParameters();
        impl PrimeFieldParameters for SmallFieldParameters {
            // 11
            const MODULUS: U256 = u256h!("000000000000000000000000000000000000000000000000000000000000000a");
        }
        type SmallField = PrimeField<SmallFieldParameters>;

        generic_test::<SmallField>();
    }
}
