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

#[cfg(test)]
mod tests {
    use super::*;
    use macros_decl::field_element;
    use u256::U256;

    #[test]
    fn geometric_series_test() {
        let base =
            field_element!("0142c45e5d743d10eae7ebb70f1526c65de7dbcdb65b322b6ddc36a812591e8f");
        let step =
            field_element!("00000000000000000000000000000000000000000000000f00dbabe0cafebabe");

        let domain = geometric_series(&base, &step, 32);
        let mut hold = base.clone();
        for item in domain {
            assert_eq!(item, hold);
            hold *= &step;
        }
    }
}
