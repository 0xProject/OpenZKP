use crate::{FieldElement, Inv, One, Zero};
use std::fmt;
use zkp_u256::U256;

// TODO: Generalize all of these

#[cfg(feature = "std")]
impl fmt::Debug for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = U256::from(self);
        write!(
            f,
            "field_element!(\"{:016x}{:016x}{:016x}{:016x}\")",
            n.limb(3),
            n.limb(2),
            n.limb(1),
            n.limb(0)
        )
    }
}
