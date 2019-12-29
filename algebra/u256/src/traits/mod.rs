mod binary;

pub trait SubFromAssign<Rhs = Self> {
    fn sub_from_assign(&mut self, rhs: Rhs);
}

pub trait DivRem<Rhs> {
    type Quotient;
    type Remainder;

    fn div_rem(self, rhs: Rhs) -> Option<(Self::Quotient, Self::Remainder)>;
}

pub trait InvMod: Sized {
    fn inv_mod(&self, modulus: &Self) -> Option<Self>;
}

pub trait GCD: Sized {
    fn gcd(a: &Self, b: &Self) -> Self;

    fn gcd_extended(a: &Self, b: &Self) -> (Self, Self, Self, bool);

    // TODO: LCM
}

// TODO: Factorial, Totient, Carmichael, Jacobi, Legendre, Binomial, etc.
// See https://gmplib.org/manual/Number-Theoretic-Functions.html

// TODO: Mega-trait for binary rings like U256 that PrimeField can use

pub use binary::{Binary, BinaryAssignRef};

pub trait BinaryRing: Binary {}
