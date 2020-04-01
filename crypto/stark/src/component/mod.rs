pub(crate) mod compose;
#[cfg(test)]
mod test;

use crate::{
    constraint_check::check_constraints, Constraints, Provable, RationalExpression, TraceTable,
    Verifiable,
};
use std::collections::HashMap;
use zkp_primefield::{FieldElement, Pow, Root};

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

    pub fn project_into(
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

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use zkp_macros_decl::field_element;
    use zkp_primefield::{u256::U256, One};

    /// Generates powers of two including 0 and 2^max
    pub(super) fn arb_2exp(max_exponent: usize) -> impl Strategy<Value = usize> {
        (0..max_exponent + 2).prop_map(move |v| (1 << v) >> 1)
    }

    /// Generates an arbitrary field element
    // TODO: Rejection sample
    pub(super) fn arb_field_element() -> impl Strategy<Value = FieldElement> {
        (any::<u64>(), any::<u64>(), any::<u64>(), any::<u64>())
            .prop_map(move |(a, b, c, d)| FieldElement::from(U256::from_limbs([a, b, c, d])))
    }

    /// Generates an arbitrary component of given size
    pub(super) fn arb_component_size(rows: usize, cols: usize) -> impl Strategy<Value = Component> {
        (arb_field_element(), arb_field_element()).prop_map(
            move |(constraint_seed, witness_seed)| {
                Component::example(rows, cols, &constraint_seed, &witness_seed)
            },
        )
    }

    /// Generates an arbitrary component
    pub(super) fn arb_component() -> impl Strategy<Value = Component> {
        (arb_2exp(10), 0_usize..=10).prop_flat_map(|(rows, cols)| arb_component_size(rows, cols))
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
                component.trace[(row, col)] += FieldElement::one();
                assert!(!component.check());
            }
        }

        #[test]
        fn test_component_provable(component in arb_component()) {
            // TODO: Make prove and verify support empty/tiny traces correctly
            prop_assume!(component.trace.num_rows() >= 2);
            prop_assume!(component.trace.num_columns() >= 1);
            let proof = component.prove(());
            let proof = proof.expect("Expected proof");
            component.verify(&proof).unwrap();
        }
    }
}
