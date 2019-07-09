use primefield::FieldElement;

pub fn eval_poly(x: FieldElement, coefficients: &[FieldElement]) -> FieldElement {
    let mut b = FieldElement::ZERO;
    for coefficient in coefficients.iter().rev() {
        b = coefficient + b * &x;
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::*;
    use primefield::{u256h, U256};

    #[test]
    fn poly_eval_test() {
        let x = FieldElement::from(u256h!(
            "04d59eebac89518453d226545efb550870f641831aaf0ed1fa2ec54499eb2183"
        ));
        let mut coef = Vec::with_capacity(100);
        for i in 0..100 {
            coef.push(FieldElement::from(U256::from(123_456_u64 + i)));
        }
        let res = eval_poly(x, coef.as_slice());
        assert_eq!(
            U256::from(res),
            u256h!("00e2d3ab8631086d0680da2c28d48b4b5248c0484eae8a04f39c646483d09f09")
        );
    }
}
