use super::traits::Component;
use crate::{RationalExpression, TraceTable};
use std::collections::HashMap;
use zkp_primefield::{FieldElement, Root};

struct Example {
    rows:    usize,
    columns: usize,
    seed:    FieldElement,
}

impl Component for Example {
    type Claim = FieldElement;
    type Witness = FieldElement;

    fn constraints(
        &self,
        claim: &Self::Claim,
    ) -> (
        (usize, usize),
        Vec<RationalExpression>,
        HashMap<String, (usize, RationalExpression)>,
    ) {
        use RationalExpression::*;

        // Construct the constraint system for the sequence.
        let omega = Constant(FieldElement::root(self.rows).unwrap());
        let mut constraints = Vec::new();
        let mut labels = HashMap::new();
        // x[0] = start
        if self.rows * self.columns >= 1 {
            constraints.push((Trace(0, 0) - (&self.seed).into()) / (X - omega.pow(0)));
            let _ = labels.insert("start".to_owned(), (0, Trace(0, 0)));
        }
        if self.rows * self.columns >= 3 {
            let _ = labels.insert(
                "final".to_owned(),
                (self.rows - 1, Trace(self.columns - 1, 0)),
            );
            let _ = labels.insert(
                "next".to_owned(),
                (
                    self.rows - 1,
                    if self.columns == 1 {
                        Trace(0, -1) * Trace(0, 0) + claim.into()
                    } else {
                        Trace(self.columns - 2, 0) * Trace(self.columns - 1, 0) + claim.into()
                    },
                ),
            );
            // For each column we need to add a constraint
            for i in 0..self.columns {
                // Find the previous two cells in the table
                let (x0, x1) = match (i, self.columns) {
                    (0, 1) => (Trace(0, -2), Trace(0, -1)),
                    (0, _) => (Trace(self.columns - 2, -1), Trace(self.columns - 1, -1)),
                    (1, _) => (Trace(self.columns - 1, -1), Trace(0, 0)),
                    (..) => (Trace(i - 2, 0), Trace(i - 1, 0)),
                };
                // Exempt the first two cells
                let exceptions = match (i, self.columns) {
                    (0, 1) => (X - omega.pow(0)) * (X - omega.pow(1)),
                    (0, _) | (1, _) => (X - omega.pow(0)),
                    (..) => 1.into(),
                };
                // x[i+2] = x[i] * x[i + 1] + offset
                constraints.push(
                    (Trace(i, 0) - x0 * x1 - claim.into()) * exceptions
                        / (X.pow(self.rows) - 1.into()),
                )
            }
        }

        ((self.rows, self.columns), constraints, labels)
    }

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
        // Construct a sequence using the quadratic recurrence relation:
        //     x[0]   = seed           (part of constraints)
        //     x[1]   = witness        (not part of constraints)
        //     x[i+2] = x[i] * x[i + 1] + claim
        let mut x0 = self.seed.clone();
        let mut x1 = witness.clone();
        let mut next = || {
            let result = x0.clone();
            let x2 = &x0 * &x1 + claim;
            x0 = x1.clone();
            x1 = x2;
            result
        };

        // Fill in the trace table with the sequence
        // the sequence is written left-to-right, then top-to-bottom.
        let mut trace = TraceTable::new(self.rows, self.columns);
        for i in 0..(self.rows * self.columns) {
            trace[(i / self.columns, i % self.columns)] = next();
        }
        trace
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{Provable, Verifiable};
    use proptest::prelude::*;
    use zkp_macros_decl::field_element;
    use zkp_primefield::{u256::U256, One};

    /// Generates an arbitrary field element
    fn arb_field_element() -> impl Strategy<Value = FieldElement> {
        (any::<u64>(), any::<u64>(), any::<u64>(), any::<u64>())
            .prop_map(move |(a, b, c, d)| FieldElement::from(U256::from_limbs([a, b, c, d])))
    }

    /// Generates an arbitrary component of given size
    fn arb_component_size(rows: usize, cols: usize) -> impl Strategy<Value = Example> {
        (arb_field_element()).prop_map(move |(seed)| Example { rows, cols, seed })
    }

    /// Generates an arbitrary component
    fn arb_component() -> impl Strategy<Value = FixedComponent> {
        (arb_2exp(10), 0_usize..=10).prop_flat_map(|(rows, cols)| arb_component_size(rows, cols))
    }

    #[test]
    fn test_labels() {
        let component = FixedComponent::example(4, 2, &2.into(), &3.into());
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
            let component = FixedComponent::empty(rows, cols);
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
            let proof = component.prove(&());
            let proof = proof.expect("Expected proof");
            component.verify(&proof).unwrap();
        }
    }
}
