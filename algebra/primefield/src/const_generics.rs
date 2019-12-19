use zkp_u256::{U256};
use std::marker::PhantomData;
use std::ops::Add;

pub trait FieldParameters {
    const MODULUS: U256;
}

pub struct Element<F: FieldParameters> {
    value: U256,
    _field: PhantomData<F>,
}

impl<F: FieldParameters> Element<F> {
    pub const MODULUS: U256 = F::MODULUS;
    pub const ZERO: Self = Element { value: U256::ZERO, _field: PhantomData };
    pub const ONE: Self = Element { value: U256::ONE, _field: PhantomData };
    
    pub fn from_u256(mut value: U256) -> Self {
        value %= Self::MODULUS;
        Element { value, _field: PhantomData }
    }
}

impl<F: FieldParameters> Add for &Element<F> {
    type Output = Element<F>;
    fn add(self, other: Self) -> Self::Output {
        // TODO: Overflow
        Element::from_u256(self.value.clone() + &other.value)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::u256h;

    struct SmallFieldParameters();

    impl FieldParameters for SmallFieldParameters {
        // 11
        const MODULUS: U256 = u256h!("000000000000000000000000000000000000000000000000000000000000000a");
    }

    type F = Element<SmallFieldParameters>;

    #[test]
    fn test_element() {
        let a = F::from_u256(U256::from(5));
        let b = F::from_u256(U256::from(234234323));
        let c = &a + &b;
        dbg!(&c.value);
        let c = &c + &F::ONE;
        dbg!(&c.value);
    }
}