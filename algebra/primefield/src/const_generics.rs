use zkp_u256::{U256};
use std::marker::PhantomData;
use std::ops::Add;
use std::fmt;

// Compile time constant modulus (constant modulus, resolved compile time)
// Compile time constant field type (static write-once modulus, resolved compile time)
// Fully dynamic (dynamic write-once modulus, resolved run time)

pub trait Field: Sized + fmt::Debug + Add<Output = Self> {
    fn modulus() -> U256;
    fn zero() -> Self;
    fn one() -> Self;

    fn from_u256(value: U256) -> Self;
}

pub struct PrimeFieldParameters {
    modulus: U256,
}

//
// Constant modulus
//

pub trait ConstantPrimeFieldParameters {
    const PARAMETERS: PrimeFieldParameters;
}

pub struct PrimeField<F: ConstantPrimeFieldParameters> {
    value: U256,
    _field: PhantomData<F>,
}

impl<F: ConstantPrimeFieldParameters> PrimeField<F> {
    const MODULUS: U256 = F::PARAMETERS.modulus;
    const ZERO: Self = PrimeField { value: U256::ZERO, _field: PhantomData };
    const ONE: Self = PrimeField { value: U256::ONE, _field: PhantomData };
}

impl<F: ConstantPrimeFieldParameters> Field for PrimeField<F> {
    fn modulus() -> U256 {
        Self::MODULUS
    }

    fn zero() -> Self {
        Self::ZERO
    }

    fn one() -> Self {
        Self::ONE
    }

    fn from_u256(mut value: U256) -> Self {
        value %= Self::MODULUS;
        PrimeField { value, _field: PhantomData }
    }
}

impl<F: ConstantPrimeFieldParameters> fmt::Debug for PrimeField<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<F: ConstantPrimeFieldParameters> Add for PrimeField<F> {
    type Output = PrimeField<F>;
    fn add(self, other: Self) -> Self::Output {
        // TODO: Overflow
        PrimeField::from_u256(self.value.clone() + &other.value)
    }
}

//
// Static modulus
//

pub trait StaticPrimeFieldParameters {
    // Ideally we'd have a 
    // const PARAMATERS: &'a PrimeFieldParameters
    // here, but Rust does not allow this.
    fn parameters() -> &'static PrimeFieldParameters;
}

pub struct StaticField<F: StaticPrimeFieldParameters> {
    value: U256,
    _field: PhantomData<F>,
}

impl<F: StaticPrimeFieldParameters> StaticField<F> {
    // const PARAMETERS: &'static PrimeFieldParameters = F::PARAMETERS;
    const ZERO: Self = StaticField { value: U256::ZERO, _field: PhantomData };
    const ONE: Self = StaticField { value: U256::ONE, _field: PhantomData };
}

impl<F: StaticPrimeFieldParameters> Field for StaticField<F> {
    fn modulus() -> U256 {
        F::parameters().modulus.clone()
    }

    fn zero() -> Self {
        Self::ZERO
    }

    fn one() -> Self {
        Self::ONE
    }

    fn from_u256(mut value: U256) -> Self {
        value %= &F::parameters().modulus;
        StaticField { value, _field: PhantomData }
    }
}

impl<F: StaticPrimeFieldParameters> fmt::Debug for StaticField<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<F: StaticPrimeFieldParameters> Add for StaticField<F> {
    type Output = StaticField<F>;
    fn add(self, other: Self) -> Self::Output {
        // TODO: Overflow
        StaticField::from_u256(self.value.clone() + &other.value)
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
        let c = c + F::one();
        dbg!(&c);
    }

    #[test]
    fn test_constant() {
        // Modulus is compile time constant
        const MODULUS: U256 = u256h!("000000000000000000000000000000000000000000000000000000000000000a");

        // Create a type F that implements a Prime Field 
        struct SmallFieldParameters();
        impl ConstantPrimeFieldParameters for SmallFieldParameters {
            // 11
            const PARAMETERS: PrimeFieldParameters = PrimeFieldParameters {
                modulus: MODULUS
            };
        }
        type SmallField = PrimeField<SmallFieldParameters>;

        generic_test::<SmallField>();
    }

    #[test]
    fn test_static() {
        // Modulus is runtime computed
        let prime = vec![2,3,5,7,11,13,17,19];
        for modulus in prime {
            let modulus = U256::from(modulus);

            static mut FIELD_PARAMETERS: PrimeFieldParameters = PrimeFieldParameters {
                modulus: U256::ZERO
            };
            unsafe {
                // Setting modulus is unsafe while field elements exist.
                FIELD_PARAMETERS.modulus = modulus;
            }

            // Create a type F that implements a Prime Field
            struct SmallFieldParameters();
            impl StaticPrimeFieldParameters for SmallFieldParameters {
                fn parameters() -> &'static PrimeFieldParameters {
                    unsafe {
                        &FIELD_PARAMETERS
                    }
                }
            }
            type SmallField = StaticField<SmallFieldParameters>;

            generic_test::<SmallField>();
        }
    }
}
