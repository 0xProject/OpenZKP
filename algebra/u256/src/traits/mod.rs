mod binary;

pub trait SubFromAssign<Rhs = Self> {
    fn sub_from_assign(&mut self, rhs: Rhs);
}

// TODO: Mega-trait for binary rings like U256 that PrimeField can use

pub use binary::{Binary, BinaryAssignRef};

pub trait BinaryRing: Binary {}
