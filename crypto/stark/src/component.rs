use crate::{
    constraint_check::check_constraints, primefield::FieldElement, Constraints, Provable,
    RationalExpression, TraceTable, Verifiable,
};
use std::{collections::HashMap, convert::TryFrom};

// TODO: Introduce prover/verifier distinction

// TODO: Pass by reference, or rather, have a high level structure.

// OPT: Don't reallocate trace table so many times, instead use a view
// that is passed top-down for writing.

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Component {
    // TODO: Make private
    pub trace:       TraceTable,
    pub constraints: Vec<RationalExpression>,
    pub labels:      HashMap<String, (usize, RationalExpression)>,
}

/// Utility function to add offsets on indices
// Valid indices will be substantially less than type limits
#[allow(clippy::cast_possible_wrap)]
// rem_euclid result is always positive
#[allow(clippy::cast_sign_loss)]
fn index_rotate(len: usize, index: usize, offset: isize) -> usize {
    let len = len as isize;
    let index = index as isize;
    (index + offset).rem_euclid(len) as usize
}

impl Component {
    /// Constructs an empty component of given size.
    ///
    /// This is useful in combination with composition combinators to pad out a
    /// component to a required size.
    pub fn empty(rows: usize, columns: usize) -> Self {
        Self {
            trace:       TraceTable::new(rows, columns),
            constraints: Vec::new(),
            labels:      HashMap::new(),
        }
    }

    pub fn check(&self) -> bool {
        let constraints = Constraints::from_expressions(
            (self.trace.num_rows(), self.trace.num_columns()),
            Vec::new(),
            self.constraints.clone(),
        )
        .unwrap();
        check_constraints(&constraints, &self.trace).is_ok()
    }

    pub fn generator(&self) -> FieldElement {
        FieldElement::root(self.trace.num_rows()).expect("no generator for trace length")
    }

    // TODO: Generic eval for given X that interpolates the columns

    pub fn eval_row(&self, expression: &RationalExpression, row: usize) -> FieldElement {
        assert!(row < self.trace.num_rows());
        let x = self.generator().pow(row);
        expression.evaluate(&x, &|col, offset| {
            self.trace[(index_rotate(self.trace.num_rows(), row, offset), col)].clone()
        })
    }

    pub fn eval_label(&self, label: &str) -> FieldElement {
        let (row, expression) = &self.labels[label];
        self.eval_row(expression, *row)
    }

    pub fn rename_label(&mut self, old: &str, new: &str) {
        if let Some(value) = self.labels.remove(old) {
            let _ = self.labels.insert(new.to_string(), value);
        } else {
            panic!("Label '{}' not found", old);
        }
    }

    pub fn remove_label(&mut self, label: &str) {
        if self.labels.remove(label).is_none() {
            panic!("Label '{}' not found", label);
        }
    }

    fn project_into(
        &self,
        target: &mut Self,
        trace_map: impl Fn(usize, usize) -> (usize, usize),
        expr_map: impl Fn(RationalExpression) -> RationalExpression,
    ) {
        // Copy over TraceTable
        for i in 0..self.trace.num_rows() {
            for j in 0..self.trace.num_columns() {
                target.trace[trace_map(i, j)] = self.trace[(i, j)].clone();
            }
        }
        // Copy over Constraints
        target.constraints.extend(
            self.constraints
                .iter()
                .map(|expr| expr.clone().map(&expr_map)),
        );
        // Copy over Labels
        // TODO: Rename colliding labels?
        // TODO: Row numbers?
        target.labels.extend(
            self.labels
                .iter()
                .map(|(label, (row, expr))| (label.clone(), (*row, expr.map(&expr_map)))),
        )
    }
}

impl Verifiable for Component {
    fn constraints(&self) -> Constraints {
        Constraints::from_expressions(
            (self.trace.num_rows(), self.trace.num_columns()),
            Vec::new(), // TODO: create a meaningful seed value
            self.constraints.clone(),
        )
        .expect("Could not produce Constraint object for Component")
    }
}

impl Provable<()> for Component {
    fn trace(&self, _witness: ()) -> TraceTable {
        self.trace.clone()
    }
}

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
        .pow(-amount);
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

pub fn compose_horizontal(mut a: Component, mut b: Component) -> Component {
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
pub fn compose_vertical(mut a: Component, mut b: Component) -> Component {
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
            result = compose_horizontal(result, Component::empty(rows, 1));
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
pub fn compose_folded(mut a: Component, mut b: Component) -> Component {
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
            compose_horizontal(fold_many(a, folds), b)
        }
        Equal => compose_horizontal(a, b),
        Greater => {
            let folds = usize::try_from((a_len / b_len).trailing_zeros()).unwrap();
            compose_horizontal(a, fold_many(b, folds))
        }
    }
}

#[cfg(feature = "test")]
impl Component {
    /// Creates an example constraint system of given size
    ///
    /// It takes two seed values, one to make the constraints unique
    /// and one to make the witness unique.
    pub fn example(
        rows: usize,
        columns: usize,
        constraint_seed: &FieldElement,
        witness_seed: &FieldElement,
    ) -> Self {
        use RationalExpression::*;

        // Construct a sequence using the quadratic recurrence relation:
        //     x[0]   = constraint_seed     (part of constraints)
        //     x[1]   = witness_seed        (not part of constraints)
        //     x[i+2] = x[i] * x[i + 1] + constraint_seed
        let mut x0 = constraint_seed.clone();
        let mut x1 = witness_seed.clone();
        let mut next = || {
            let result = x0.clone();
            let x2 = &x0 * &x1 + constraint_seed;
            x0 = x1.clone();
            x1 = x2;
            result
        };

        // Fill in the trace table with the sequence
        // the sequence is written left-to-right, then top-to-bottom.
        let mut trace = TraceTable::new(rows, columns);
        for i in 0..(rows * columns) {
            trace[(i / columns, i % columns)] = next();
        }

        // Construct the constraint system for the sequence.
        let omega = Constant(FieldElement::root(rows).unwrap());
        let mut constraints = Vec::new();
        let mut labels = HashMap::new();
        // x[0] = start
        if rows * columns >= 1 {
            constraints.push((Trace(0, 0) - constraint_seed.into()) / (X - omega.pow(0)));
            let _ = labels.insert("start".to_owned(), (0, Trace(0, 0)));
        }
        if rows * columns >= 3 {
            let _ = labels.insert("final".to_owned(), (rows - 1, Trace(columns - 1, 0)));
            let _ = labels.insert(
                "next".to_owned(),
                (
                    rows - 1,
                    if columns == 1 {
                        Trace(0, -1) * Trace(0, 0) + constraint_seed.into()
                    } else {
                        Trace(columns - 2, 0) * Trace(columns - 1, 0) + constraint_seed.into()
                    },
                ),
            );
            // For each column we need to add a constraint
            for i in 0..columns {
                // Find the previous two cells in the table
                let (x0, x1) = match (i, columns) {
                    (0, 1) => (Trace(0, -2), Trace(0, -1)),
                    (0, _) => (Trace(columns - 2, -1), Trace(columns - 1, -1)),
                    (1, _) => (Trace(columns - 1, -1), Trace(0, 0)),
                    (..) => (Trace(i - 2, 0), Trace(i - 1, 0)),
                };
                // Exempt the first two cells
                let exceptions = match (i, columns) {
                    (0, 1) => (X - omega.pow(0)) * (X - omega.pow(1)),
                    (0, _) | (1, _) => (X - omega.pow(0)),
                    (..) => 1.into(),
                };
                // x[i+2] = x[i] * x[i + 1] + offset
                constraints.push(
                    (Trace(i, 0) - x0 * x1 - constraint_seed.into()) * exceptions
                        / (X.pow(rows) - 1.into()),
                )
            }
        }

        Self {
            trace,
            constraints,
            labels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use zkp_macros_decl::field_element;
    use zkp_primefield::u256::U256;

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

    /// Generates powers of two including 0 and 2^max
    fn arb_2exp(max_exponent: usize) -> impl Strategy<Value = usize> {
        (0..max_exponent + 2).prop_map(move |v| (1 << v) >> 1)
    }

    /// Generates an arbitrary field element
    fn arb_field_element() -> impl Strategy<Value = FieldElement> {
        (any::<u64>(), any::<u64>(), any::<u64>(), any::<u64>())
            .prop_map(move |(a, b, c, d)| FieldElement::from(U256::from_limbs(a, b, c, d)))
    }

    /// Generates an arbitrary component of given size
    fn arb_component_size(rows: usize, cols: usize) -> impl Strategy<Value = Component> {
        (arb_field_element(), arb_field_element()).prop_map(
            move |(constraint_seed, witness_seed)| {
                Component::example(rows, cols, &constraint_seed, &witness_seed)
            },
        )
    }

    /// Generates an arbitrary component
    fn arb_component() -> impl Strategy<Value = Component> {
        (arb_2exp(10), 0_usize..=10).prop_flat_map(|(rows, cols)| arb_component_size(rows, cols))
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

    #[test]
    fn test_labels() {
        let component = Component::example(4, 2, &2.into(), &3.into());
        assert_eq!(
            component.eval_label("start"),
            field_element!("0000000000000000000000000000000000000000000000000000000000000002")
        );
        assert_eq!(
            component.eval_label("final"),
            field_element!("00000000000000000000000000000000000000000000000000000001756cd5b6")
        );
        assert_eq!(
            component.eval_label("next"),
            field_element!("000000000000000000000000000000000000000000000000001987bfbe4f8af6")
        );
    }

    proptest! {

        #[test]
        fn test_empty(rows in arb_2exp(10), cols in 0_usize..=10) {
            let component = Component::empty(rows, cols);
            assert!(component.check())
        }

        #[test]
        fn test_arb_component(mut component in arb_component(), row: usize, col: usize) {
            assert!(component.check());
            if component.trace.num_rows() * component.trace.num_columns() > 2 {
                // Spotcheck to make sure constraints constraint the table
                let row = row % component.trace.num_rows();
                let col = col % component.trace.num_columns();
                component.trace[(row, col)] += FieldElement::ONE;
                assert!(!component.check());
            }
        }

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
        fn test_compose_horizontal((left, right) in arb_hor_components()) {
            prop_assume!(left.trace.num_rows() == right.trace.num_rows());
            let result = compose_horizontal(left.clone(), right.clone());
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
        fn test_compose_vertical((top, bottom) in arb_ver_components()) {
            prop_assume!(top.trace.num_rows() == bottom.trace.num_rows());
            prop_assume!(top.trace.num_columns() == bottom.trace.num_columns());
            prop_assume!(top.constraints.len() == bottom.constraints.len());
            let result = compose_vertical(top.clone(), bottom.clone());
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
        fn test_compose_folded(left in arb_component(), right in arb_component()) {
            let result = compose_folded(left.clone(), right.clone());
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

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10))]

        #[test]
        fn test_component_provable(component in arb_component()) {
            // TODO: Make prove and verify support empty/tiny traces correctly
            prop_assume!(component.trace.num_rows() >= 2);
            prop_assume!(component.trace.num_columns() >= 1);
            let proof = component.prove(()).unwrap();
            component.verify(&proof).unwrap();
        }
    }
}
