use crate::RationalExpression;
use crate::Constraints;
use crate::TraceTable;
use crate::prove;
use crate::Proof;

#[cfg_attr(feature = "std", derive(Debug))]
pub struct Component {
    trace:       TraceTable,
    constraints: Vec<RationalExpression>,
}

/*
trait Component {
    type Claim;
    type Witness;

    fn trace(&self, claim: &Claim, witness: &Witness) -> TraceTable;
    fn constraints(&self, claim: &Claim) -> Vec<RationalExpression>;
}
*/

impl Component {
    pub fn prove(&self, channel_seed: Vec<u8>) -> Result<Proof, ()> {
        prove(
            &Constraints::from_expressions(
                (self.trace.num_rows(), self.trace.num_columns()),
                channel_seed,
                self.constraints
            )?,
            &self.trace
        )
    }
}

pub fn compose_horizontal(a: Component, b: Component) -> Component {
    assert_eq!(a.trace.num_rows(), b.trace.num_rows());

    // Create a new trace table that horizontally concatenates a and b
    let trace = TraceTable::new(a.trace.num_rows(), a.trace.num_columns() + b.trace.num_columns());
    for i in 0..a.trace.num_rows() {
        for j in 0..a.trace.num_columns() {
            trace[(i, j)] = a.trace[(i, j)];
        }
        for j in 0..b.trace.num_columns() {
            trace[(i, j + a.trace.num_columns())] = b.trace[(i, j)];
        }
    }

    // Shift b's constraints over by a's columns.
    let constraints = a.constraints;
    constraints.extend(b.constraints.into_iter().map(|constraint|
        constraint.map(&mut |expression| {
            use RationalExpression::*;
            match expression {
                Trace(i, j) => Trace(i + a.trace.num_columns(), j),
                e => e,
            }
        })
    ));

    Component { trace, constraints }
}
