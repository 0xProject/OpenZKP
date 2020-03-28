use crate::{Affine, Jacobian, BETA};
use proptest::prelude::*;
use zkp_primefield::{FieldElement, Inv, Pow, SquareRoot};

impl Arbitrary for Affine {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        prop_oneof![
            Just(Affine::Zero),
            <(bool, FieldElement)>::arbitrary().prop_filter_map("x not on curve", |(sign, x)| {
                (x.pow(3_usize) + &x + BETA)
                    .square_root()
                    .map(|y| Affine::new(x, if sign { y } else { -y }))
            })
        ]
        .boxed()
    }
}

impl Arbitrary for Jacobian {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
        <(FieldElement, Affine)>::arbitrary()
            .prop_filter_map("z is zero", |(z, a)| {
                let mut j = Self::from(a);
                z.inv().map(|inverse| {
                    j.x /= z.pow(2_usize);
                    j.y /= z.pow(3_usize);
                    j.z *= inverse;
                    j
                })
            })
            .boxed()
    }
}

mod tests {
    use super::*;

    proptest!(
        #[test]
        fn affine_on_curve(a: Affine) {
            prop_assert!(a.on_curve());
        }
    );

    proptest!(
        #[test]
        fn jacobian_on_curve(j: Jacobian) {
            prop_assert!(j.on_curve());
        }
    );
}
