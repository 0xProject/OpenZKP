use crate::rational_expression::RationalExpression;
use primefield::FieldElement;

pub struct Constraints {
    trace_degree: usize,
    num_columns:  usize,
    constraints:  Vec<RationalExpression>,
}

impl Constraints {
    pub fn combine(&mut self, coefficients: &[FieldElement]) {
        assert_eq!(coefficients.len(), 2 * self.constraints.len());
    }
}
