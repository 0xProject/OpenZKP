use crate::RationalExpression;

#[derive(Clone, Debug)]
pub struct Constraints {
    pub trace_length: usize,
    pub num_columns:  usize,
    pub constraints:  Vec<RationalExpression>,
}
