use crate::{Affine, Jacobian, BETA};
use proptest::prelude::*;
use zkp_primefield::{FieldElement, Pow, SquareInline, SquareRoot, Zero};

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
                if z.is_zero() {
                    None
                } else {
                    let mut j = Self::from(a);
                    let square = z.square();
                    j.x *= &square;
                    j.y *= square * &z;
                    j.z *= z;
                    Some(j)
                }
            })
            .boxed()
    }
}

mod tests {
    use super::*;

    proptest!(
        #[test]
        fn affine_on_curve(a: Affine) {
            prop_assert!(a.is_on_curve());
        }
    );

    proptest!(
        #[test]
        fn jacobian_on_curve(j: Jacobian) {
            prop_assert!(j.is_on_curve());
        }
    );
}
