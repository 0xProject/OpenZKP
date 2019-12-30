use crate::square_root::square_root;
use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, BitAnd, Div, DivAssign, Mul, MulAssign, Neg, ShrAssign, Sub, SubAssign},
    prelude::v1::*,
};
use zkp_macros_decl::u256h;
use zkp_u256::{
    commutative_binop, noncommutative_binop, to_montgomery_const, DivRem, Montgomery,
    MontgomeryParameters, One, Pow, Zero, U256, Binary
};
// TODO: Implement Serde
#[cfg(feature = "std")]
use std::fmt;

/// Requirements for the base unsigned integer type
pub trait FieldUInt:
    PartialEq
    + Zero
    + One
    + Montgomery
    + for<'a> DivRem<&'a Self, Quotient = Self, Remainder = Self>
    + DivRem<u64, Quotient = Self, Remainder = u64>
    + ShrAssign<usize>
{
}

impl<T> FieldUInt for T where
    T: PartialEq
        + Zero
        + One
        + Montgomery
        + for<'a> DivRem<&'a Self, Quotient = Self, Remainder = Self>
        + DivRem<u64, Quotient = Self, Remainder = u64>
        + ShrAssign<usize>
{
}

/// Required constant parameters for the prime field
pub trait FieldParameters<UInt>: MontgomeryParameters<UInt>
where
    UInt: FieldUInt,
{
    const GENERATOR: UInt;
    const ORDER: UInt;
}

/// A finite field.
///
/// The order `Parameters::MODULUS` must be prime. Internally, values are
/// represented in Montgomery form for faster multiplications.
#[allow(clippy::module_name_repetitions)]
#[derive(PartialEq, Eq, Clone, Hash)]
pub struct Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    uint:        UInt,
    _parameters: PhantomData<Parameters>,
}

#[derive(PartialEq, Eq, Clone, Hash)]
struct StarkFieldParameters();

impl MontgomeryParameters<U256> for StarkFieldParameters {
    const M64: u64 = 0xffff_ffff_ffff_ffff;
    const MODULUS: U256 =
        u256h!("0800000000000011000000000000000000000000000000000000000000000001");
    const R1: U256 = u256h!("07fffffffffffdf0ffffffffffffffffffffffffffffffffffffffffffffffe1");
    const R2: U256 = u256h!("07ffd4ab5e008810ffffffffff6f800000000001330ffffffffffd737e000401");
    const R3: U256 = u256h!("038e5f79873c0a6df47d84f8363000187545706677ffcc06cc7177d1406df18e");
}

impl FieldParameters<U256> for StarkFieldParameters {
    /// 3, in montgomery form.
    const GENERATOR: U256 =
        u256h!("07fffffffffff9b0ffffffffffffffffffffffffffffffffffffffffffffffa1");
    ///
    const ORDER: U256 = u256h!("0800000000000011000000000000000000000000000000000000000000000000");
}

pub type FieldElement = Field<U256, StarkFieldParameters>;

impl<UInt, Parameters> Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    pub const GENERATOR: Self = Self::from_montgomery(Parameters::GENERATOR);
    pub const MODULUS: UInt = Parameters::MODULUS;
    pub const ONE: Self = Self::from_montgomery(Parameters::R1);
    pub const ZERO: Self = Self::from_montgomery(UInt::zero());

    /// Creates a constant value from a `Base` constant.
    ///
    /// It does compile-time conversion to Montgomery form.
    // TODO: Fix
    // pub const fn from_uint_const(uint: &UInt) -> Self {
    // Self::from_montgomery(to_montgomery_const(
    // uint,
    // &Parameters::MODULUS,
    // Parameters::M64,
    // &Parameters::R2,
    // ))
    // }

    #[inline(always)]
    pub const fn from_montgomery(uint: UInt) -> Self {
        // TODO: Uncomment assertion when support in `const fn` is enabled.
        // See https://github.com/rust-lang/rust/issues/57563
        // debug_assert!(n < Self::MODULUS);
        Self {
            uint,
            _parameters: PhantomData,
        }
    }

    // #[cfg(feature = "std")]
    // pub fn from_hex_str(s: &str) -> Self {
    // Self::from(UInt::from_hex_str(s))
    // }

    #[inline(always)]
    pub fn as_montgomery(&self) -> &UInt {
        &self.uint
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.uint == UInt::zero()
    }

    #[inline(always)]
    pub fn inv(&self) -> Option<Self> {
        self.uint
            .inv_redc::<Parameters>()
            .map(Self::from_montgomery)
    }

    #[inline(always)]
    pub fn double(&self) -> Self {
        // TODO: Optimize
        self.clone() + self
    }

    #[inline(always)]
    pub fn triple(&self) -> Self {
        // TODO: Optimize
        self.clone() + self + self
    }

    #[cfg_attr(feature = "inline", inline(always))]
    pub fn square(&self) -> Self {
        Self::from_montgomery(self.as_montgomery().square_redc_inline::<Parameters>())
    }

    // TODO
    // #[inline(always)]
    // pub fn square_root(&self) -> Option<Self> {
    // square_root(self)
    // }

    #[inline(always)]
    pub fn neg_assign(&mut self) {
        *self = self.neg()
    }

    // OPT: replace this with a constant array of roots of unity.
    // TODO: version with abstracted order
    pub fn root(order: usize) -> Option<Self> {
        // TODO: div_rem trait
        if let Some((q, rem)) = Parameters::ORDER.div_rem(order as u64) {
            if rem.is_zero() {
                Some(Self::GENERATOR.pow(&q))
            } else {
                None
            }
        } else {
            Some(Self::ONE)
        }
    }
}

impl<UInt, Parameters, Exponent> Pow<&Exponent> for &Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
    Exponent: Binary,
{
    type Output = Field<UInt, Parameters>;

    fn pow(self, exponent: &Exponent) -> Self::Output {
        if let Some(msb) = exponent.most_significant_bit() {
            let mut result = Self::Output::ONE;
            let mut square = self.clone();
            for i in (0..=msb) {
                if exponent.bit(i) {
                    result *= &square;
                }
                if !i.is_zero() {
                    square = &square.square();
                }
            }
            result
        } else {
            // exponent = 0
            Self::Output::ONE
        }
    }
}

impl<UInt, Parameters> Pow<usize> for &Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>
{
    type Output = Field<UInt, Parameters>;

    fn pow(self, exponent: usize) -> Self::Output {
        self.pow(&exponent)
    }
}

pub fn invert_batch_src_dst(source: &[FieldElement], destination: &mut [FieldElement]) {
    assert_eq!(source.len(), destination.len());
    let mut accumulator = FieldElement::ONE;
    for i in 0..source.len() {
        destination[i] = accumulator.clone();
        accumulator *= &source[i];
    }
    accumulator = accumulator.inv().unwrap();
    for i in (0..source.len()).rev() {
        destination[i] *= &accumulator;
        accumulator *= &source[i];
    }
}

pub fn invert_batch(to_be_inverted: &[FieldElement]) -> Vec<FieldElement> {
    if to_be_inverted.is_empty() {
        return Vec::new();
    }
    let n = to_be_inverted.len();
    let mut inverses = cumulative_product(to_be_inverted);

    // TODO: Enforce check to prevent uninvertable elements.
    let mut inverse = inverses[n - 1].inv().unwrap();
    for i in (1..n).rev() {
        inverses[i] = &inverses[i - 1] * &inverse;
        inverse *= &to_be_inverted[i];
    }
    inverses[0] = inverse;
    inverses
}

fn cumulative_product(elements: &[FieldElement]) -> Vec<FieldElement> {
    elements
        .iter()
        .scan(FieldElement::ONE, |running_product, x| {
            *running_product *= x;
            Some(running_product.clone())
        })
        .collect()
}

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

macro_rules! impl_from_uint {
    ($t:ty) => {
        impl From<$t> for FieldElement {
            #[inline(always)]
            fn from(n: $t) -> Self {
                U256::from(n).into()
            }
        }
    };
}

impl_from_uint!(u8);
impl_from_uint!(u16);
impl_from_uint!(u32);
impl_from_uint!(u64);
impl_from_uint!(u128);
impl_from_uint!(usize);

macro_rules! impl_from_int {
    ($t:ty) => {
        impl From<$t> for FieldElement {
            #[cfg_attr(feature = "inline", inline(always))]
            fn from(n: $t) -> Self {
                if n >= 0 {
                    U256::from(n).into()
                } else {
                    Self::from(U256::from(-n)).neg()
                }
            }
        }
    };
}

impl_from_int!(i8);
impl_from_int!(i16);
impl_from_int!(i32);
impl_from_int!(i64);
impl_from_int!(i128);
impl_from_int!(isize);

// The FieldElement versions are called `to_` and not `as_` like their
// U256 counterparts. This is because a `U256::from` is performed which
// does a non-trivial `from_montgomery` conversion.
macro_rules! to_uint {
    ($fname:ident, $uname:ident, $type:ty) => {
        #[inline(always)]
        pub fn $fname(&self) -> $type {
            U256::from(self).$uname()
        }
    };
}

macro_rules! to_int {
    ($fname:ident, $uname:ident, $type:ty) => {
        #[cfg_attr(feature = "inline", inline(always))]
        pub fn $fname(&self) -> $type {
            let n = U256::from(self);
            let half = Self::MODULUS >> 1;
            if n < half {
                n.$uname()
            } else {
                (n - Self::MODULUS).$uname()
            }
        }
    };
}

// We don't want newlines between the macro invocations.
#[rustfmt::skip]
impl FieldElement {
    to_uint!(to_u8, as_u8, u8);
    to_uint!(to_u16, as_u16, u16);
    to_uint!(to_u32, as_u32, u32);
    to_uint!(to_u64, as_u64, u64);
    to_uint!(to_u128, as_u128, u128);
    to_uint!(to_usize, as_usize, usize);
    to_int!(to_i8, as_i8, i8);
    to_int!(to_i16, as_i16, i16);
    to_int!(to_i32, as_i32, i32);
    to_int!(to_i64, as_i64, i64);
    to_int!(to_i128, as_i128, i128);
    to_int!(to_isize, as_isize, isize);
}

impl From<U256> for FieldElement {
    #[inline(always)]
    fn from(n: U256) -> Self {
        (&n).into()
    }
}

impl From<&U256> for FieldElement {
    #[inline(always)]
    fn from(n: &U256) -> Self {
        Self(n.to_montgomery::<Self>())
    }
}

impl From<FieldElement> for U256 {
    #[inline(always)]
    fn from(n: FieldElement) -> Self {
        (&n).into()
    }
}

impl From<&FieldElement> for U256 {
    #[inline(always)]
    fn from(n: &FieldElement) -> Self {
        n.0.from_montgomery::<FieldElement>()
    }
}

impl<UInt, Parameters> Neg for &Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Field<UInt, Parameters>;

    #[cfg_attr(feature = "inline", inline(always))]
    fn neg(self) -> Self::Output {
        Self::Output::from_montgomery(Parameters::MODULUS - self.as_montgomery())
    }
}

impl<UInt, Parameters> AddAssign<&Self> for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    #[cfg_attr(feature = "inline", inline(always))]
    fn add_assign(&mut self, rhs: &Self) {
        self.uint += &rhs.uint;
        if self.uint >= Self::MODULUS {
            self.uint -= &Self::MODULUS;
        }
    }
}

impl<UInt, Parameters> AddAssign<Self> for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.add_assign(&rhs);
    }
}

impl<UInt, Parameters> Add<&Self> for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: &Self) -> Self {
        self.add_assign(rhs);
        self
    }
}

impl<UInt, Parameters> Add<Self> for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        self.add(&rhs)
    }
}

impl<UInt, Parameters> Add<Self> for &Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Field<UInt, Parameters>;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        self.clone().add(rhs)
    }
}

impl<UInt, Parameters> Add<Field<UInt, Parameters>> for &Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Field<UInt, Parameters>;

    #[inline(always)]
    fn add(self, rhs: Field<UInt, Parameters>) -> Self::Output {
        rhs.add(self)
    }
}

impl SubAssign<&FieldElement> for FieldElement {
    #[cfg_attr(feature = "inline", inline(always))]
    fn sub_assign(&mut self, rhs: &Self) {
        if self.0 < rhs.0 {
            self.0 += &Self::MODULUS;
        }
        self.0 -= &rhs.0;
    }
}

impl MulAssign<&FieldElement> for FieldElement {
    #[cfg_attr(feature = "inline", inline(always))]
    fn mul_assign(&mut self, rhs: &Self) {
        self.0 = self.0.mul_redc_inline::<Self>(&rhs.0);
    }
}

impl DivAssign<&FieldElement> for FieldElement {
    fn div_assign(&mut self, rhs: &Self) {
        *self *= rhs.inv().unwrap();
    }
}

impl core::iter::Product for FieldElement {
    fn product<I: Iterator<Item = FieldElement>>(iter: I) -> Self {
        iter.fold(Self::ONE, Mul::mul)
    }
}

// TODO: Implement Sum, Successors, ... for FieldElement.

// commutative_binop!(FieldElement, Add, add, AddAssign, add_assign);
commutative_binop!(FieldElement, Mul, mul, MulAssign, mul_assign);
noncommutative_binop!(FieldElement, Sub, sub, SubAssign, sub_assign);
noncommutative_binop!(FieldElement, Div, div, DivAssign, div_assign);

#[cfg(any(test, feature = "quickcheck"))]
use quickcheck::{Arbitrary, Gen};

#[cfg(any(test, feature = "quickcheck"))]
impl Arbitrary for FieldElement {
    #[inline(always)]
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        // TODO: Generate 0, 1, p/2 and -1
        Self::from_montgomery(U256::arbitrary(g) % Self::MODULUS)
    }
}

#[allow(unused_macros)]
macro_rules! field_h {
    (- $e:expr) => {
        field_h!($e).neg()
    };
}

// Quickcheck needs pass by value
#[allow(clippy::needless_pass_by_value)]
// We allow these in tests for readability/ease of editing
#[allow(clippy::redundant_clone)]
#[cfg(test)]
mod tests {
    use super::*;
    use itertools::repeat_n;
    use quickcheck_macros::quickcheck;
    use zkp_macros_decl::field_element;

    #[test]
    fn test_literal() {
        const SMALL: FieldElement = field_element!("0F");
        const NUM: FieldElement =
            field_element!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c");
        assert_eq!(SMALL, FieldElement::from(15));
        assert_eq!(
            NUM,
            u256h!("0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c").into()
        );
    }

    #[test]
    fn negative_one_is_additive_inverse_of_one() {
        assert_eq!(
            FieldElement::ONE + FieldElement::NEGATIVE_ONE,
            FieldElement::ZERO
        );
    }

    #[test]
    fn minus_zero_equals_zero() {
        assert_eq!(-&FieldElement::ZERO, FieldElement::ZERO);
    }

    #[test]
    fn test_add() {
        let a = field_element!("06eabe184aa9caca2e17f6073bcc10bb9714c0e3866ff00e0d386f4396392852");
        let b = field_element!("0313000a764a9a5514efc99070de3f70586794f9bb0add62ac689763aadea7e8");
        let c = field_element!("01fdbe22c0f4650e4307bf97acaa502bef7c55dd417acd70b9a106a74117d039");
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_sub() {
        let a = FieldElement::from_montgomery(u256h!(
            "0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c"
        ));
        let b = FieldElement::from_montgomery(u256h!(
            "024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b"
        ));
        let c = FieldElement::from(u256h!(
            "03d7be0dd45f307519282c76caedd14b3ead2be9cb6512ab60cfd7dfeb5a806a"
        ));
        assert_eq!(a - b, c);
    }

    #[test]
    fn test_mul() {
        let a = FieldElement::from_montgomery(u256h!(
            "0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c"
        ));
        let b = FieldElement::from_montgomery(u256h!(
            "024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b"
        ));
        let c = FieldElement::from(u256h!(
            "0738900c5dcab24b419674df19d2cfeb9782eca6d1107be18577eb060390365b"
        ));
        assert_eq!(a * b, c);
    }

    #[test]
    fn test_div() {
        let a = FieldElement::from_montgomery(u256h!(
            "0548c135e26faa9c977fb2eda057b54b2e0baa9a77a0be7c80278f4f03462d4c"
        ));
        let b = FieldElement::from_montgomery(u256h!(
            "024385f6bebc1c496e09955db534ef4b1eaff9a78e27d4093cfa8f7c8f886f6b"
        ));
        let c = FieldElement::from(u256h!(
            "003a9a346e7103c74dfcddd0eeb4e16ca71d8887c2bed3d4ee718b62015e87b2"
        ));
        assert_eq!(a / b, c);
    }

    #[quickcheck]
    fn test_batch_inv(x: Vec<FieldElement>) -> bool {
        if x.iter().any(FieldElement::is_zero) {
            true
        } else {
            invert_batch(x.as_slice())
                .iter()
                .zip(x.iter())
                .all(|(a_inv, a)| *a_inv == a.inv().unwrap())
        }
    }

    #[quickcheck]
    fn from_as_isize(n: isize) -> bool {
        FieldElement::from(n).to_isize() == n
    }

    #[quickcheck]
    fn from_as_i128(n: i128) -> bool {
        FieldElement::from(n).to_i128() == n
    }

    #[quickcheck]
    fn add_identity(a: FieldElement) -> bool {
        &a + FieldElement::ZERO == a
    }

    #[quickcheck]
    fn mul_identity(a: FieldElement) -> bool {
        &a * FieldElement::ONE == a
    }

    #[quickcheck]
    fn commutative_add(a: FieldElement, b: FieldElement) -> bool {
        &a + &b == b + a
    }

    #[quickcheck]
    fn commutative_mul(a: FieldElement, b: FieldElement) -> bool {
        &a * &b == b * a
    }

    #[quickcheck]
    fn associative_add(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        &a + (&b + &c) == (a + b) + c
    }

    #[quickcheck]
    fn associative_mul(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        &a * (&b * &c) == (a * b) * c
    }

    #[quickcheck]
    fn inverse_add(a: FieldElement) -> bool {
        &a + a.neg() == FieldElement::ZERO
    }

    #[quickcheck]
    fn inverse_mul(a: FieldElement) -> bool {
        match a.inv() {
            None => a == FieldElement::ZERO,
            Some(ai) => a * ai == FieldElement::ONE,
        }
    }

    #[quickcheck]
    fn distributivity(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        &a * (&b + &c) == (&a * b) + (a * c)
    }

    #[quickcheck]
    fn pow_0(a: FieldElement) -> bool {
        a.pow(0) == FieldElement::ONE
    }

    #[quickcheck]
    fn pow_1(a: FieldElement) -> bool {
        a.pow(1) == a
    }

    #[quickcheck]
    fn pow_2(a: FieldElement) -> bool {
        a.pow(2) == a.square()
    }

    #[quickcheck]
    fn pow_n(a: FieldElement, n: usize) -> bool {
        a.pow(n) == repeat_n(a, n).product()
    }

    #[quickcheck]
    fn fermats_little_theorem(a: FieldElement) -> bool {
        a.pow(FieldElement::MODULUS) == a
    }

    #[test]
    fn zeroth_root_of_unity() {
        assert_eq!(FieldElement::root(0).unwrap(), FieldElement::ONE);
    }

    #[test]
    fn roots_of_unity_squared() {
        let powers_of_two = (0..193).map(|n| U256::ONE << n);
        let roots_of_unity: Vec<_> = powers_of_two
            .clone()
            .map(|n| FieldElement::root(n).unwrap())
            .collect();

        for (smaller_root, larger_root) in roots_of_unity[1..].iter().zip(roots_of_unity.as_slice())
        {
            assert_eq!(smaller_root.square(), *larger_root);
            assert!(!smaller_root.is_one());
        }
    }

    #[test]
    fn root_of_unity_definition() {
        let powers_of_two = (0..193).map(|n| U256::ONE << n);
        for n in powers_of_two {
            let root_of_unity = FieldElement::root(n.clone()).unwrap();
            assert_eq!(root_of_unity.pow(n), FieldElement::ONE);
        }
    }
}
