use super::traits::Component;
use crate::{RationalExpression, TraceTable};
use std::collections::HashMap;
use zkp_primefield::{FieldElement, Root};
struct Horizontal<Left, Right>
where
    Left: Component,
    Right: Component,
{
    left:  Left,
    right: Right,
}

impl<Left, Right> Horizontal<Left, Right>
where
    Left: Component,
    Right: Component,
{
    fn new(left: Left, right: Right) {
        Self { left, right }
    }

    fn left(&self) -> &Left {
        self.left
    }

    fn right(&self) -> &Right {
        self.right
    }

    fn left_mut(&mut self) -> &mut Left {
        self.left
    }

    fn right_mut(&mut self) -> &mut Right {
        self.right
    }
}

impl<Left, Right> Component for Horizontal<Left, Right>
where
    Left: Component,
    Right: Component,
{
    type Claim = (Left::Claim, Right::Claim);
    type Witness = (Left::Witness, Right::Witness);

    fn constraints(
        &self,
        claim: &Self::Claim,
    ) -> (
        (usize, usize),
        Vec<RationalExpression>,
        HashMap<String, (usize, RationalExpression)>,
    ) {
        unimplemented!()
    }

    fn trace(&self, claim: &Self::Claim, witness: &Self::Witness) -> TraceTable {
        unimplemented!()
    }
}
