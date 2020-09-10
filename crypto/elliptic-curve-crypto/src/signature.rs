#[cfg(feature = "parity_codec")]
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use zkp_elliptic_curve::ScalarFieldElement;

// TODO (SECURITY): The signatures are malleable in w -> -w.
#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Default, Debug)]
#[cfg_attr(feature = "parity_codec", derive(Encode, Decode,))]
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
