pub use ff::Field;
use ff::*;
use serde::{Deserialize, Serialize};

#[derive(PrimeField)]
#[PrimeFieldModulus = "3618502788666131213697322783095070105623107215331596699973092056135872020481"]
#[PrimeFieldGenerator = "3"]
pub struct Fp(FpRepr);
