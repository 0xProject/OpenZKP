use crate::RationalExpression;
use rand::random;
use tiny_keccak::{Hasher, Keccak};
use zkp_primefield::FieldElement;
use zkp_u256::U256;

impl RationalExpression {
    /// Probabilistic extrinsic equality check
    pub fn equals(&self, other: &Self) -> bool {
        // Random evaluation point, also serves as random seed for trace.
        let x = random::<FieldElement>();
        let trace = |column: usize, offset: isize| {
            let mut hasher = Keccak::v256();
            hasher.update(&x.as_montgomery().to_bytes_be());
            hasher.update(&column.to_be_bytes());
            hasher.update(&offset.to_be_bytes());
            let mut output = [0_u8; 32];
            hasher.finalize(&mut output);
            U256::from_bytes_be(&output).into()
        };

        // Check equality by evaluating at a random point
        let lhs = self.evaluate(&x, &trace);
        let rhs = other.evaluate(&x, &trace);
        lhs == rhs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        use RationalExpression::*;
        let left = X;
        let right = X.pow(2) / X;
        assert!(left.equals(&right));
    }

    #[test]
    fn test_unequal() {
        use RationalExpression::*;
        let left = X;
        let right = X.pow(3) / X;
        assert!(!left.equals(&right));
    }
}
