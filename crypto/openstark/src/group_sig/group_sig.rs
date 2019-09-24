#[cfg(feature = "prover")]
use crate::TraceTable;
use crate::{
    constraint_system::{Provable, Verifiable},
    constraints::Constraints,
    polynomial::DensePolynomial,
    rational_expression::RationalExpression::*,
    rational_expression::RationalExpression,
};
use macros_decl::field_element;
use primefield::{fft::ifft, FieldElement};
use u256::U256;

struct Claim {
    how_many: usize,
    hash: U256,
    who: Vec<(FieldElement, FieldElement)>,
}

struct Witness {
    signatures: Vec<(FieldElement, FieldElement)>,
}

// Elliptic curve param.
const A : usize = 1;

// Provide this function with the input point and the output location 
// and it produces a constraint for point doubling first on the x output and one one the y
// In default usage each input should be trace type.
fn point_double(input_x: RationalExpression, input_y: RationalExpression, output_x: RationalExpression, output_y: RationalExpression) -> [RationalExpression; 2] {
    let two = Constant(2.into());
    let three = Constant(3.into());
    let four = Constant(4.into());
    let eight = Constant(8.into());
    
    // These constraints take the lambda = (3x_old^2 + a)/ 2(y_old) and multiply through to clear divisions.
    // This is a multiplied through form of x_new = lambda^2 - 2x_old, which is asserted to be output_x
    let lambda_numerator : RationalExpression = three*Exp(input_x.clone().into(), 2) + Constant(A.into());
    let new_x = Exp(lambda_numerator.clone().into(), 2) - eight*Exp(input_y.clone().into(), 2)*input_x.clone() - four*Exp(input_y.clone().into(), 2)*output_x.clone();
    // This is a multipled through form of y_new = lambda*(x_old - x_new) - y_old, which is asserted to be output y.
    let new_y = lambda_numerator*(input_x - output_x) - two.clone()*Exp(input_y.clone().into(), 2) - two*input_y*output_y;
    [new_x, new_y]
}

// Provide this function the two points P and Q to add plus the asserted output location
// It provides constraints that express that Out = P + Q
fn point_add(x_p: RationalExpression, y_p: RationalExpression, x_q: RationalExpression, y_q: RationalExpression, x_out: RationalExpression, y_out: RationalExpression) -> [RationalExpression; 2] {
    // These constraints take the lambda = (y_q - y_p)/ (x_q - x_p) and multiply through to clear divisions.
    let lambda_numerator = y_q.clone() - y_p.clone();
    let lambda_denominator = x_q.clone() - x_p.clone();
    let new_x = Exp(lambda_numerator.clone().into(), 2) - lambda_denominator.clone()*(x_p.clone() - x_q.clone() - x_out.clone());
    let new_y = lambda_numerator*(x_p - x_out) - lambda_denominator*(y_p + y_out);
    [new_x, new_y]
}

// Full conditional bool check that location is a if test and b if !test [secured by the check that test = 1 or 0]
fn conditional(a: RationalExpression, b: RationalExpression, location: RationalExpression, test: RationalExpression) -> [RationalExpression; 2] {
    [one_or_zero(test.clone()), simple_conditional(a, b, location, test)]
}

// Tests if a rational expression is not one or zero
fn one_or_zero(test: RationalExpression) -> RationalExpression {
    test.clone()*(Constant(FieldElement::ONE) - test.clone())
}

// Non secured conditional check.
fn simple_conditional(a: RationalExpression, b: RationalExpression, location: RationalExpression, test: RationalExpression) -> RationalExpression {
    a*test.clone() + (Constant(FieldElement::ONE) - test.clone())*b - location.clone()
}

// This function takes in a target and a claimed bit decomposition vector and returns constraints which check that
// (1) the decomp is all ones and zeros and (2) that the target equals the sum of increasing powers of two of the decomp.
// Note given the size limit of field elements we expect that decomp is len < 256
fn bit_decomp_test(target: RationalExpression, decomp: Vec<RationalExpression>) -> [RationalExpression; 2] {
    let mut power = FieldElement::ONE;
    let mut consistency = Constant(FieldElement::ZERO);
    let mut sum = Constant(FieldElement::ZERO);

    for bit in decomp.iter() {
        consistency = consistency + one_or_zero(bit.clone());
        sum = sum + bit.clone()*Constant(power.clone());
        power  *= FieldElement::from(2);
    }
    [consistency, sum - target]
}