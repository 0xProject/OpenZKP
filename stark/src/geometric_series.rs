use primefield::FieldElement;
use std::prelude::v1::*;

// TODO: Return an iterator instead, perhaps a seekable one.
pub fn geometric_series(base: &FieldElement, step: &FieldElement, len: usize) -> Vec<FieldElement> {
    let mut accumulator = base.clone();
    (0..)
        .map(move |_| {
            let current = accumulator.clone();
            accumulator *= step;
            current
        })
        .take(len)
        .collect()
}
