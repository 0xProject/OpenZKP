use crate::field::FieldElement;
use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
pub struct PublicInput {
    pub path_length: usize,
    pub leaf: FieldElement,
    pub root: FieldElement,
}

pub fn get_public_input() -> PublicInput {
    from_str(include_str!("public_input.json")).unwrap()
}

#[derive(Deserialize)]
pub struct PrivateInput {
    pub directions: Vec<bool>,
    pub path: Vec<FieldElement>,
}

pub fn get_private_input() -> PrivateInput {
    from_str(include_str!("private_input.json")).unwrap()
}

#[derive(Deserialize)]
pub struct PeriodicColumns {
    left_x_coefficients: Vec<FieldElement>,
    left_y_coefficients: Vec<FieldElement>,
    right_x_coefficients: Vec<FieldElement>,
    right_y_coefficients: Vec<FieldElement>,
}

pub fn get_periodic_columns() -> PeriodicColumns {
    from_str(include_str!("periodic_columns.json")).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve::Affine;
    use crate::pedersen_points::PEDERSEN_POINTS;
    use crate::polynomial::eval_poly;
    use crate::U256;
    use crate::proofs::geometric_series;

    #[test]
    fn test_get_public_input() {
        get_public_input();
    }

    #[test]
    fn test_get_private_input() {
        get_private_input();
    }

    #[test]
    fn e() {
        let omega: FieldElement = FieldElement::root(U256::from(256u128)).unwrap();

        let periodic_columns = get_periodic_columns();
        let left_x_coefficients = periodic_columns.left_x_coefficients;
        let left_y_coefficients = periodic_columns.left_y_coefficients;

        let evaluation_points = geometric_series(&FieldElement::ONE, &omega, 252);
        let left_points = evaluation_points.iter().map(|f: &FieldElement| Affine::Point {
            x: eval_poly(f.clone(), &left_x_coefficients),
            y: eval_poly(f.clone(), &left_y_coefficients),
        });

        for (i, point) in left_points.enumerate() {
            assert_eq!(point, PEDERSEN_POINTS[i + 1]);
        }
    }

    #[test]
    fn f() {
        let omega: FieldElement = FieldElement::root(U256::from(256u128)).unwrap();

        let periodic_columns = get_periodic_columns();
        let right_x_coefficients = periodic_columns.right_x_coefficients;
        let right_y_coefficients = periodic_columns.right_y_coefficients;

        let evaluation_points = geometric_series(&FieldElement::ONE, &omega, 252);
        let right_points = evaluation_points.iter().map(|f: &FieldElement| Affine::Point {
            x: eval_poly(f.clone(), &right_x_coefficients),
            y: eval_poly(f.clone(), &right_y_coefficients),
        });

        for (i, point) in right_points.enumerate() {
            assert_eq!(point, PEDERSEN_POINTS[i + 253]);
        }
    }
}
