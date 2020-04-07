use zkp_elliptic_curve::ScalarFieldElement;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

// TODO (SECURITY): The signatures are malleable in w -> -w.
#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Signature {
    r: ScalarFieldElement,
    w: ScalarFieldElement,
}

impl Signature {
    pub fn new(r: ScalarFieldElement, w: ScalarFieldElement) -> Self {
        Self { r, w }
    }

    pub fn r(&self) -> &ScalarFieldElement {
        &self.r
    }

    pub fn w(&self) -> &ScalarFieldElement {
        &self.w
    }
}
