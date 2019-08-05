use crate::{constraints::Constraints, trace_table::TraceTable};

pub trait ConstraintSystem {
    type Public;
    type Private;

    fn constraints(public: &Self::Public) -> Constraints;

    fn trace(public: &Self::Public, private: &Self::Private) -> TraceTable;
}
