use crate::{Root, SquareRoot};
use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Shr,
    prelude::v1::*,
};
use zkp_macros_decl::u256h;
use zkp_u256::{
    to_montgomery_const, AddInline, Binary, DivRem, Inv, Montgomery, MontgomeryParameters,
    MulInline, NegInline, One, Pow, SquareInline, SubInline, Zero, U256,
};
// TODO: Implement Serde
#[cfg(feature = "std")]
use std::fmt;

/// Requirements for the base unsigned integer type
// TODO: Fix naming
#[allow(clippy::module_name_repetitions)]
// Lint has a false positive here
#[allow(single_use_lifetimes)]
pub trait FieldUInt:
    Clone
    + PartialEq
    + PartialOrd
    + Zero
    + One
    + for<'a> AddInline<&'a Self>
    + for<'a> SubInline<&'a Self>
    + Montgomery
{
}

// Lint has a false positive here
#[allow(single_use_lifetimes)]
impl<T> FieldUInt for T where
    T: Clone
        + PartialEq
        + PartialOrd
        + Zero
        + One
        + for<'a> AddInline<&'a T>
        + for<'a> SubInline<&'a T>
        + Montgomery
{
}

/// Required constant parameters for the prime field
// TODO: Make these and Tonelly-Shanks parameters optional and enable
// functionality when implemented.
// TODO: Fix naming
#[allow(clippy::module_name_repetitions)]
// UInt can not have interior mutability
#[allow(clippy::declare_interior_mutable_const)]
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
// Derive fails for Clone, PartialEq, Eq, Hash
pub struct Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    uint:        UInt,
    _parameters: PhantomData<Parameters>,
}

impl<UInt, Parameters> Clone for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    fn clone(&self) -> Self {
        Self::from_montgomery(self.as_montgomery().clone())
    }
}

impl<UInt, Parameters> PartialEq for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_montgomery() == other.as_montgomery()
    }
}

impl<UInt, Parameters> Eq for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
}

/// Implements [`Hash`] when `UInt` does.
impl<UInt, Parameters> Hash for Field<UInt, Parameters>
where
    UInt: FieldUInt + Hash,
    Parameters: FieldParameters<UInt>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_montgomery().hash::<H>(state)
    }
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub struct StarkFieldParameters();

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

// TODO: Fix naming
#[allow(clippy::module_name_repetitions)]
pub type FieldElement = Field<U256, StarkFieldParameters>;

impl FieldElement {
    /// Creates a constant value from a `U256` constant in Montgomery form.
    // TODO: Make member of `Field` after <https://github.com/rust-lang/rust/issues/57563>
    pub const fn from_montgomery_const(uint: U256) -> Self {
        Self {
            uint,
            _parameters: PhantomData,
        }
    }

    /// Creates a constant value from a `U256` constant.
    ///
    /// It does compile-time conversion to Montgomery form.
    // TODO: Make member of `Field` after <https://github.com/rust-lang/rust/issues/57563>
    pub const fn from_uint_const(n: &U256) -> Self {
        let uint = to_montgomery_const(
            n,
            &StarkFieldParameters::MODULUS,
            StarkFieldParameters::M64,
            &StarkFieldParameters::R2,
        );
        Self {
            uint,
            _parameters: PhantomData,
        }
    }
}

impl<UInt, Parameters> Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    pub const MODULUS: UInt = Parameters::MODULUS;

    #[inline(always)]
    pub fn generator() -> Self {
        Self::from_montgomery(Parameters::GENERATOR)
    }

    // TODO: Make `const fn` after <https://github.com/rust-lang/rust/issues/57563>
    #[inline(always)]
    pub fn from_montgomery(uint: UInt) -> Self {
        // TODO: Uncomment assertion when support in `const fn` is enabled.
        // See https://github.com/rust-lang/rust/issues/57563
        // debug_assert!(n < Self::MODULUS);
        Self {
            uint,
            _parameters: PhantomData,
        }
    }

    // TODO: from_radix_str
    // #[cfg(feature = "std")]
    // pub fn from_hex_str(s: &str) -> Self {
    // Self::from(UInt::from_hex_str(s))
    // }

    #[inline(always)]
    pub fn as_montgomery(&self) -> &UInt {
        &self.uint
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
}

impl<UInt, Parameters> Zero for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    #[inline(always)]
    fn zero() -> Self {
        Self::from_montgomery(UInt::zero())
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.as_montgomery().is_zero()
    }
}

impl<UInt, Parameters> One for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    #[inline(always)]
    fn one() -> Self {
        Self::from_montgomery(Parameters::R1)
    }

    #[inline(always)]
    fn is_one(&self) -> bool {
        self.as_montgomery() == &Parameters::R1
    }
}

impl<UInt, Parameters> AddInline<&Self> for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    #[inline(always)]
    fn add_inline(&self, rhs: &Self) -> Self {
        let mut result = self.as_montgomery().add_inline(rhs.as_montgomery());
        if result >= Self::MODULUS {
            result.sub_assign_inline(&Self::MODULUS);
        }
        Self::from_montgomery(result)
    }
}

impl<UInt, Parameters> SubInline<&Self> for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    #[inline(always)]
    fn sub_inline(&self, rhs: &Self) -> Self {
        let lhs = self.as_montgomery();
        let rhs = rhs.as_montgomery();
        let borrow = rhs > lhs;
        let mut result = lhs.sub_inline(rhs);
        if borrow {
            result.add_assign_inline(&Self::MODULUS);
        }
        Self::from_montgomery(result)
    }
}

impl<UInt, Parameters> NegInline for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    #[inline(always)]
    fn neg_inline(&self) -> Self {
        if self.is_zero() {
            Self::zero()
        } else {
            Self::from_montgomery(Self::MODULUS.sub_inline(self.as_montgomery()))
        }
    }
}

impl<UInt, Parameters> SquareInline for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    #[inline(always)]
    fn square_inline(&self) -> Self {
        Self::from_montgomery(self.as_montgomery().square_redc_inline::<Parameters>())
    }
}

impl<UInt, Parameters> MulInline<&Self> for Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    #[inline(always)]
    fn mul_inline(&self, rhs: &Self) -> Self {
        Self::from_montgomery(
            self.as_montgomery()
                .mul_redc_inline::<Parameters>(rhs.as_montgomery()),
        )
    }
}

impl<UInt, Parameters> Inv for &Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Option<Field<UInt, Parameters>>;

    #[inline(always)] // Simple wrapper
    fn inv(self) -> Self::Output {
        self.as_montgomery()
            .inv_redc::<Parameters>()
            .map(Field::<UInt, Parameters>::from_montgomery)
    }
}

impl<UInt, Parameters> Pow<usize> for &Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Field<UInt, Parameters>;

    fn pow(self, exponent: usize) -> Self::Output {
        self.pow(&exponent)
    }
}

impl<UInt, Parameters> Pow<isize> for &Field<UInt, Parameters>
where
    UInt: FieldUInt,
    Parameters: FieldParameters<UInt>,
{
    type Output = Option<Field<UInt, Parameters>>;

    fn pow(self, exponent: isize) -> Self::Output {
        let negative = exponent < 0;
        let abs = exponent.abs() as usize;
        if negative {
            self.inv().map(|n| n.pow(&abs))
        } else {
            Some(self.pow(&abs))
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
            let mut result = Self::Output::one();
            let mut square = self.clone();
            for i in 0..=msb {
                if exponent.bit(i) {
                    result *= &square;
                }
                if i < msb {
                    square.square_assign();
                }
            }
            result
        } else {
            // exponent = 0
            Self::Output::one()
        }
    }
}

impl<UInt, Parameters> Root<usize> for Field<UInt, Parameters>
where
    UInt: FieldUInt + Binary + DivRem<u64, Quotient = UInt, Remainder = u64>,
    Parameters: FieldParameters<UInt>,
{
    // OPT: replace this with a constant array of roots of unity.
    fn root(order: usize) -> Option<Self> {
        let order = order as u64;
        if let Some((q, rem)) = Parameters::ORDER.div_rem(order) {
            if rem.is_zero() {
                Some(Self::generator().pow(&q))
            } else {
                None
            }
        } else {
            Some(Self::one())
        }
    }
}

// TODO: Generalize over order type
// Lint has a false positive here
#[allow(single_use_lifetimes)]
impl<UInt, Parameters> Root<&UInt> for Field<UInt, Parameters>
where
    UInt: FieldUInt + Binary + for<'a> DivRem<&'a UInt, Quotient = UInt, Remainder = UInt>,
    Parameters: FieldParameters<UInt>,
{
    // OPT: replace this with a constant array of roots of unity.
    fn root(order: &UInt) -> Option<Self> {
        if let Some((q, rem)) = Parameters::ORDER.div_rem(order) {
            if rem.is_zero() {
                Some(Self::generator().pow(&q))
            } else {
                None
            }
        } else {
            Some(Self::one())
        }
    }
}

impl<UInt, Parameters> SquareRoot for Field<UInt, Parameters>
where
    UInt: FieldUInt + Binary + Shr<usize, Output = UInt>,
    Parameters: FieldParameters<UInt>,
{
    fn is_quadratic_residue(&self) -> bool {
        self.pow(&(Self::MODULUS >> 1_usize)) != -Self::one()
    }

    // Tonelli-Shanks square root algorithm for prime fields
    // See 'Handbook of Applied Cryptography' algorithm 3.34
    // OPT: Use algorithm 3.39 for Proth primes.
    fn square_root(&self) -> Option<Self> {
        if self.is_zero() {
            return Some(Self::zero());
        }
        if !self.is_quadratic_residue() {
            return None;
        }

        // TODO: Provide as a constant parameter?
        // Factor order as `signifcant` * 2 ^ `trailing_zeros`
        let trailing_zeros = Parameters::ORDER.trailing_zeros();
        let signifcant = Parameters::ORDER >> trailing_zeros;
        // The starting value of c in the Tonelli Shanks algorithm. We are using the
        // prefered generator, as the quadratic nonresidue the algorithm requires.
        let c_start = Self::generator().pow(&signifcant);

        // This algorithm is still correct when the following assertion fails. However,
        // more efficient algorithms exist when MODULUS % 4 == 3 or MODULUS % 8 == 5
        // (3.36 and 3.37 in HAC).
        // debug_assert!(&FieldElement::MODULUS & 7_u64 == 1);

        // OPT: Raising a to a fixed power is a good candidate for an addition chain.
        let mut root = self.pow(&((signifcant + UInt::one()) >> 1));
        let mut c = c_start;
        let inverse = self.inv().unwrap(); // Zero case is handled above

        for i in 1..trailing_zeros {
            if (root.square() * &inverse).pow(&(UInt::one() << (trailing_zeros - i - 1)))
                == -Self::one()
            {
                root *= &c;
            }
            // OPT: Create lookup table for squares of c.
            c.square_assign();
        }
        Some(root)
    }
}

pub fn invert_batch_src_dst(source: &[FieldElement], destination: &mut [FieldElement]) {
    assert_eq!(source.len(), destination.len());
    let mut accumulator = FieldElement::one();
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
        .scan(FieldElement::one(), |running_product, x| {
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
                    -Self::from(U256::from(-n))
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
        Self::from_montgomery(n.to_montgomery::<StarkFieldParameters>())
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
        n.as_montgomery().from_montgomery::<StarkFieldParameters>()
    }
}

impl core::iter::Product for FieldElement {
    fn product<I: Iterator<Item = FieldElement>>(iter: I) -> Self {
        use std::ops::Mul;
        iter.fold(Self::one(), Mul::mul)
    }
}

// TODO: Implement Sum, Successors, ... for FieldElement.

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
    fn minus_zero_equals_zero() {
        dbg!(field_element!("00").as_montgomery());
        assert!(FieldElement::zero().is_zero());
        assert!(field_element!("00").is_zero());
        assert_eq!(FieldElement::zero(), FieldElement::zero());
        assert_eq!(-FieldElement::zero(), FieldElement::zero());
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
        &a + FieldElement::zero() == a
    }

    #[quickcheck]
    fn mul_identity(a: FieldElement) -> bool {
        &a * FieldElement::one() == a
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
        &a + a.neg() == FieldElement::zero()
    }

    #[quickcheck]
    fn inverse_mul(a: FieldElement) -> bool {
        match a.inv() {
            None => a == FieldElement::zero(),
            Some(ai) => a * ai == FieldElement::one(),
        }
    }

    #[quickcheck]
    fn distributivity(a: FieldElement, b: FieldElement, c: FieldElement) -> bool {
        &a * (&b + &c) == (&a * b) + (a * c)
    }

    #[quickcheck]
    fn square(a: FieldElement) -> bool {
        a.square() == &a * &a
    }

    #[quickcheck]
    fn pow_0(a: FieldElement) -> bool {
        a.pow(0_usize) == FieldElement::one()
    }

    #[quickcheck]
    fn pow_1(a: FieldElement) -> bool {
        a.pow(1_usize) == a
    }

    #[quickcheck]
    fn pow_2(a: FieldElement) -> bool {
        a.pow(2_usize) == &a * &a
    }

    #[quickcheck]
    fn pow_n(a: FieldElement, n: usize) -> bool {
        a.pow(n) == repeat_n(a, n).product()
    }

    #[quickcheck]
    fn fermats_little_theorem(a: FieldElement) -> bool {
        a.pow(&FieldElement::MODULUS) == a
    }

    #[quickcheck]
    fn square_root(a: FieldElement) -> bool {
        let s = a.square();
        let r = s.square_root().unwrap();
        r == a || r == -a
    }

    #[test]
    fn zeroth_root_of_unity() {
        assert_eq!(FieldElement::root(0).unwrap(), FieldElement::one());
    }

    #[test]
    fn roots_of_unity_squared() {
        let powers_of_two = (0..193).map(|n| U256::ONE << n);
        let roots_of_unity: Vec<_> = powers_of_two
            .clone()
            .map(|n| FieldElement::root(&n).unwrap())
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
            let root_of_unity = FieldElement::root(&n).unwrap();
            assert_eq!(root_of_unity.pow(&n), FieldElement::one());
        }
    }
}
