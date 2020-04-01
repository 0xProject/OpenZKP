use super::Component;
use crate::RationalExpression;
use std::convert::TryFrom;
use zkp_primefield::{FieldElement, Pow, Root};

// OPT: Use the degree of freedom provided by shift_x + shift_trace to
// minimize the number of trace values to reveal.

// OPT: In addition to this, we can also permute (rotate) the values in
// the trace table to add a further degree of freedom.

/// Change the order of the columns
///
/// `new_column_index = permutation[old_column_index]`
// TODO: Figure out a better strategy for passing traces around, for now we
// always pass by value.
#[allow(clippy::needless_pass_by_value)]
pub fn permute_columns(a: Component, permutation: &[usize]) -> Component {
    use RationalExpression::*;

    // Validate the permutation
    // TODO: Check that there are nu duplicate values in permutation.
    assert_eq!(a.trace.num_columns(), permutation.len());
    assert_eq!(
        permutation.iter().find(|&e| *e >= a.trace.num_columns()),
        None
    );

    // Create a new trace table that permutes the columns
    let mut result = Component::empty(a.trace.num_rows(), a.trace.num_columns());
    a.project_into(
        &mut result,
        |i, j| (i, permutation[j]),
        |expression| {
            match expression {
                Trace(column, offset) => Trace(permutation[column], offset),
                other => other,
            }
        },
    );
    result
}

/// Rotate around the row indices
///
/// `new_row_index = old_row_index + amount`
pub fn shift(a: Component, amount: isize) -> Component {
    use RationalExpression::*;
    if a.trace.num_rows() <= 1 {
        return a;
    }
    let amount_abs: usize =
        usize::try_from(amount.rem_euclid(isize::try_from(a.trace.num_rows()).unwrap())).unwrap();
    let mut result = Component::empty(a.trace.num_rows(), a.trace.num_columns());
    let factor = FieldElement::root(a.trace.num_rows())
        .expect("No generator for trace length")
        .pow(-amount)
        .unwrap(); // Root can not be zero
    a.project_into(
        &mut result,
        |i, j| ((i + amount_abs) % a.trace.num_rows(), j),
        |expression| {
            match expression {
                X => Constant(factor.clone()) * X,
                other => other,
            }
        },
    );
    for (row, _expr) in result.labels.values_mut() {
        *row += amount_abs;
        *row %= a.trace.num_rows();
    }
    result
}

/// TODO: Reverse the order of the rows

/// Half the number of columns and double the number of rows.
///
/// Folds even columns into even rows and odd columns into odd rows.
///
/// **Note.** The number of columns is required to be even. To make it even,
/// you can horizontally compose with an empty component of size n x 1.
// TODO: Figure out a better strategy for passing traces around, for now we
// always pass by value.
#[allow(clippy::needless_pass_by_value)]
// Valid indices will be substantially less than type limits
#[allow(clippy::cast_possible_wrap)]
pub fn fold(a: Component) -> Component {
    use RationalExpression::*;
    assert_eq!(a.trace.num_columns() % 2, 0);
    let mut result = Component::empty(2 * a.trace.num_rows(), a.trace.num_columns() / 2);
    a.project_into(
        &mut result,
        |i, j| (2 * i + (j % 2), j / 2),
        |expression| {
            match expression {
                Trace(i, j) => Trace(i / 2, 2 * j + ((i % 2) as isize)),
                other => other,
            }
        },
    );
    for (row, _expr) in result.labels.values_mut() {
        *row *= 2;
    }
    result
}

pub fn horizontal(mut a: Component, mut b: Component) -> Component {
    use RationalExpression::*;
    assert_eq!(a.trace.num_rows(), b.trace.num_rows());
    let mut result = Component::empty(
        a.trace.num_rows(),
        a.trace.num_columns() + b.trace.num_columns(),
    );
    a.labels = a
        .labels
        .into_iter()
        .map(|(key, val)| (format!("left_{}", key), val))
        .collect();
    a.project_into(&mut result, |i, j| (i, j), |expression| expression);
    b.labels = b
        .labels
        .into_iter()
        .map(|(key, val)| (format!("right_{}", key), val))
        .collect();
    b.project_into(
        &mut result,
        |i, j| (i, j + a.trace.num_columns()),
        |expression| {
            match expression {
                Trace(i, j) => Trace(i + a.trace.num_columns(), j),
                other => other,
            }
        },
    );
    result
}

// We allow this for symmetry
#[allow(clippy::needless_pass_by_value)]
pub fn vertical(mut a: Component, mut b: Component) -> Component {
    use RationalExpression::*;
    assert_eq!(a.trace.num_rows(), b.trace.num_rows());
    assert_eq!(a.trace.num_columns(), b.trace.num_columns());
    assert_eq!(a.constraints.len(), b.constraints.len());
    // TODO: assert_eq!(Set(a.constraints), Set(b.constraints));
    let expr_map = |expression| {
        match expression {
            X => X.pow(2),
            other => other,
        }
    };
    let mut result = Component::empty(2 * a.trace.num_rows(), a.trace.num_columns());
    a.labels = a
        .labels
        .into_iter()
        .map(|(key, val)| (format!("top_{}", key), val))
        .collect();
    a.project_into(&mut result, |i, j| (i, j), expr_map);
    b.labels = b
        .labels
        .into_iter()
        .map(|(key, (row, expr))| (format!("bottom_{}", key), (row + a.trace.num_rows(), expr)))
        .collect();
    b.project_into(&mut result, |i, j| (i + a.trace.num_rows(), j), expr_map);
    // Remove b's constraints (but keep the mapped labels)
    result.constraints.truncate(a.constraints.len());
    result
}

/// Fold a component a number of times, padding if necessary.
pub fn fold_many(a: Component, folds: usize) -> Component {
    let mut result = a;
    for _ in 0..folds {
        if result.trace.num_columns() % 2 == 1 {
            let rows = result.trace.num_rows();
            result = horizontal(result, Component::empty(rows, 1));
            result.labels = result
                .labels
                .into_iter()
                .map(|(label, val)|
                // Remove `left_` prefix from labels
                (label.trim_start_matches("left_").to_owned(), val))
                .collect();
        }
        result = fold(result)
    }
    result
}

/// Horizontally compose two components of potentially unequal length
pub fn folded(mut a: Component, mut b: Component) -> Component {
    use std::cmp::Ordering::*;
    let a_len = a.trace.num_rows();
    let b_len = b.trace.num_rows();
    if a_len == 0 {
        b.labels = b
            .labels
            .into_iter()
            .map(|(key, val)| (format!("right_{}", key), val))
            .collect();
        return b;
    }
    if b_len == 0 {
        a.labels = a
            .labels
            .into_iter()
            .map(|(key, val)| (format!("left_{}", key), val))
            .collect();
        return a;
    }
    match a_len.cmp(&b_len) {
        Less => {
            let folds = usize::try_from((b_len / a_len).trailing_zeros()).unwrap();
            horizontal(fold_many(a, folds), b)
        }
        Equal => horizontal(a, b),
        Greater => {
            let folds = usize::try_from((a_len / b_len).trailing_zeros()).unwrap();
            horizontal(a, fold_many(b, folds))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::tests::{arb_2exp, arb_component, arb_component_size, arb_field_element},
        *,
    };
    use proptest::prelude::*;

    /// Generates an arbitrary permutation on n numbers
    fn arb_permutation(n: usize) -> impl Strategy<Value = Vec<usize>> {
        prop::collection::vec(any::<usize>(), if n > 1 { n - 1 } else { 0 }).prop_map(
            move |random| {
                let mut result: Vec<usize> = (0..n).collect();
                // Fisher-Yates shuffle
                if n > 1 {
                    for (i, random) in random.into_iter().enumerate() {
                        let j = i + random % (n - i);
                        result.swap(i, j);
                    }
                }
                result
            },
        )
    }

    /// Generates an arbitrary component and column permutation
    fn arb_component_and_permutation() -> impl Strategy<Value = (Component, Vec<usize>)> {
        arb_component().prop_flat_map(|component| {
            let permutation = arb_permutation(component.trace.num_columns());
            (Just(component), permutation)
        })
    }

    /// Generate two components with equal number of rows
    fn arb_hor_components() -> impl Strategy<Value = (Component, Component)> {
        (arb_2exp(10), 0_usize..=10, 0_usize..=10).prop_flat_map(|(rows, cols_left, cols_right)| {
            (
                arb_component_size(rows, cols_left),
                arb_component_size(rows, cols_right),
            )
        })
    }

    /// Generate two components with equal number of rows
    fn arb_ver_components() -> impl Strategy<Value = (Component, Component)> {
        (
            arb_2exp(10),
            0_usize..=10,
            arb_field_element(),
            arb_field_element(),
            arb_field_element(),
        )
            .prop_map(|(rows, cols, constraint_seed, top_seed, bottom_seed)| {
                (
                    Component::example(rows, cols, &constraint_seed, &top_seed),
                    Component::example(rows, cols, &constraint_seed, &bottom_seed),
                )
            })
    }

    proptest! {

        // For transformations and combinations we would ideally
        // like to check that the new Component proofs the same claims
        // as the input ones. Unfortunately, this is difficult to verify.
        // Instead, we verify:
        //
        // * The new Component is internally consistent.
        // * Labeled values remain and have the same values.
        // * The new Component's trace has at least as many values as the
        //   input traces. Although you could imagine a row inlining transform
        //   that violates this.
        // * The new Component has at least as many constraints as the inputs
        //   combined. Except for vertical composition, where the inputs have
        //   identical constraints and only one copy results. We could also
        //   imagine an optimization pass that combines constraints if possible
        //   and removes redundant ones.

        #[test]
        fn test_permute_columns((component, permutation) in arb_component_and_permutation()) {
            let result = permute_columns(component.clone(), &permutation);
            assert!(result.check());
            assert_eq!(result.trace.num_rows(), component.trace.num_rows());
            assert_eq!(result.trace.num_columns(), component.trace.num_columns());
            assert_eq!(result.constraints.len(), component.constraints.len());
            for label in component.labels.keys() {
                assert_eq!(component.eval_label(label), result.eval_label(label))
            }
        }

        #[test]
        fn test_shift(component in arb_component(), amount in -10000_isize..10000) {
            let result = shift(component.clone(), amount);
            assert!(result.check());
            assert_eq!(result.trace.num_rows(), component.trace.num_rows());
            assert_eq!(result.trace.num_columns(), component.trace.num_columns());
            assert_eq!(result.constraints.len(), component.constraints.len());
            for label in component.labels.keys() {
                assert_eq!(component.eval_label(label), result.eval_label(label))
            }
        }

        #[test]
        fn test_fold(component in arb_component()) {
            prop_assume!(component.trace.num_columns() % 2 == 0);
            let result = fold(component.clone());
            assert!(result.check());
            assert_eq!(result.trace.num_rows(), component.trace.num_rows() * 2);
            assert_eq!(result.trace.num_columns(), component.trace.num_columns() / 2);
            assert_eq!(result.constraints.len(), component.constraints.len());
            for label in component.labels.keys() {
                assert_eq!(component.eval_label(label), result.eval_label(label))
            }
        }

        #[test]
        fn test_fold_many(component in arb_component(), folds in 0_usize..4) {
            let result = fold_many(component.clone(), folds);
            assert!(result.check());
            assert_eq!(result.trace.num_rows(), component.trace.num_rows() << folds);
            let col_delta = result.trace.num_columns() - (component.trace.num_columns() >> folds);
            assert!(col_delta == 0 || col_delta == 1);
            assert_eq!(result.constraints.len(), component.constraints.len());
            for label in component.labels.keys() {
                assert_eq!(component.eval_label(label), result.eval_label(label))
            }
        }

        #[test]
        fn test_horizontal((left, right) in arb_hor_components()) {
            prop_assume!(left.trace.num_rows() == right.trace.num_rows());
            let result = horizontal(left.clone(), right.clone());
            assert!(result.check());
            assert_eq!(result.trace.num_rows(), left.trace.num_rows());
            assert_eq!(result.trace.num_columns(),
                left.trace.num_columns() + right.trace.num_columns());
            assert_eq!(result.constraints.len(),
                left.constraints.len() + right.constraints.len());
            for label in left.labels.keys() {
                assert_eq!(
                    left.eval_label(label),
                    result.eval_label(&format!("left_{}", label))
                )
            }
            for label in right.labels.keys() {
                assert_eq!(
                    right.eval_label(label),
                    result.eval_label(&format!("right_{}", label))
                )
            }
        }

        #[test]
        fn test_vertical((top, bottom) in arb_ver_components()) {
            prop_assume!(top.trace.num_rows() == bottom.trace.num_rows());
            prop_assume!(top.trace.num_columns() == bottom.trace.num_columns());
            prop_assume!(top.constraints.len() == bottom.constraints.len());
            let result = vertical(top.clone(), bottom.clone());
            assert!(result.check());
            assert_eq!(result.trace.num_rows(), 2 * top.trace.num_rows());
            assert_eq!(result.trace.num_columns(), top.trace.num_columns());
            assert_eq!(result.constraints.len(), top.constraints.len());
            for label in top.labels.keys() {
                assert_eq!(
                    top.eval_label(label),
                    result.eval_label(&format!("top_{}", label))
                )
            }
            for label in bottom.labels.keys() {
                assert_eq!(
                    bottom.eval_label(label),
                    result.eval_label(&format!("bottom_{}", label))
                )
            }
        }

        #[test]
        fn test_folded(left in arb_component(), right in arb_component()) {
            let result = folded(left.clone(), right.clone());
            assert!(result.check());
            assert_eq!(result.trace.num_rows(),
                std::cmp::max(left.trace.num_rows(), right.trace.num_rows()));
            assert!(result.trace.num_columns() <=
                left.trace.num_columns() + right.trace.num_columns());
            assert_eq!(result.constraints.len(),
                left.constraints.len() + right.constraints.len());
            for label in left.labels.keys() {
                assert_eq!(
                    left.eval_label(label),
                    result.eval_label(&format!("left_{}", label))
                )
            }
            for label in right.labels.keys() {
                assert_eq!(
                    right.eval_label(label),
                    result.eval_label(&format!("right_{}", label))
                )
            }
        }
    }
}
