// False positive: attribute has a use
#[allow(clippy::useless_attribute)]
// False positive: Importing preludes is allowed
#[allow(clippy::wildcard_imports)]
use std::{fmt, prelude::v1::*};

use crate::{Root, SquareRoot, UInt as FieldUInt};
#[cfg(feature = "parity_codec")]
use parity_scale_codec::{Decode, Encode};
use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
    ops::Shr,
};
use zkp_u256::{
    AddInline, Binary, DivRem, Inv, Montgomery as _, MontgomeryParameters, MulInline, NegInline,
    One, Pow, SquareInline, SubInline, Zero, U256,
};

/// A finite field of prime order.
///
/// The order `Parameters::MODULUS` must be prime. Internally, values are
/// represented in Montgomery form for faster multiplications.
///
/// At a minimum `UInt` should implement [`Clone`], [`PartialEq`],
/// [`PartialOrd`], [`Zero`], [`One`], [`AddInline`]`<&Self>`,
/// [`SubInline`]`<&Self>` and [`Montgomery`].
///
/// For [`Root`] it should also implment [`Binary`] and [`DivRem`]. For
/// [`SquareRoot`] it requires [`Binary`]  and [`Shr`]`<usize>`. For rand
/// support it requires [`rand::distributions::uniform::SampleUniform`]. For
/// `proptest` support `Parameters` needs to be `'static + Send` (which it
/// really should anyway).
// Derive fails for Clone, PartialEq, Eq, Hash
#[cfg_attr(feature = "parity_codec", derive(Encode, Decode))]
pub struct PrimeField<P: Parameters> {
    // TODO: un-pub. They are pub so FieldElement can have const-fn constructors.
    pub uint:        P::UInt,
    pub _parameters: PhantomData<P>,
}

/// Required constant parameters for the prime field
// TODO: Fix naming
#[allow(clippy::module_name_repetitions)]
// UInt can not have interior mutability
#[allow(clippy::declare_interior_mutable_const)]
// HACK: Ideally we'd use MontgomeryParameters<UInt: FieldUInt>
// See <https://github.com/rust-lang/rust/issues/52662>
pub trait Parameters: 'static + Send + Sync + Sized {
    type UInt: FieldUInt;

    /// The modulus to implement in Montgomery form
    const MODULUS: Self::UInt;

    /// M64 = -MODULUS^(-1) mod 2^64
    const M64: u64;

    // R1 = 2^256 mod MODULUS
    const R1: Self::UInt;

    // R2 = 2^512 mod MODULUS
    const R2: Self::UInt;

    // R3 = 2^768 mod MODULUS
    const R3: Self::UInt;

    // Generator and quadratic non-residue
    const GENERATOR: Self::UInt;

    // Multiplicative order: Modulus - 1
    const ORDER: Self::UInt;
}

// Derive `MontgomeryParameters` from `Parameters` as `Montgomery<P:
// Parameters>`
struct Montgomery<P: Parameters>(PhantomData<P>);
impl<P: Parameters> MontgomeryParameters for Montgomery<P> {
    type UInt = P::UInt;

    const M64: u64 = P::M64;
    const MODULUS: Self::UInt = P::MODULUS;
    const R1: Self::UInt = P::R1;
    const R2: Self::UInt = P::R2;
    const R3: Self::UInt = P::R3;
}

impl<P: Parameters> PrimeField<P> {
    // UInt can not have interior mutability
    #[allow(clippy::declare_interior_mutable_const)]
    pub const MODULUS: P::UInt = P::MODULUS;

    #[inline(always)]
    pub fn modulus() -> P::UInt {
        P::MODULUS
    }

    /// The multiplicative order of the field.
    ///
    /// Equal to `modulus() - 1` for prime fields.
    #[inline(always)]
    pub fn order() -> P::UInt {
        P::ORDER
    }

    #[inline(always)]
    pub fn generator() -> Self {
        Self::from_montgomery(P::GENERATOR)
    }

    #[inline(always)]
    pub fn as_montgomery(&self) -> &P::UInt {
        debug_assert!(self.uint < Self::modulus());
        &self.uint
    }

    /// Construct from `UInt` in Montgomery form.
    ///
    /// This is a trivial function.
    // TODO: Make `const fn` after <https://github.com/rust-lang/rust/issues/57563>
    #[inline(always)]
    pub fn from_montgomery(uint: P::UInt) -> Self {
        debug_assert!(uint < Self::modulus());
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

    /// Convert to `UInt`.
    #[inline(always)] // Simple wrapper for `from_montgomery`
    pub fn to_uint(&self) -> P::UInt {
        debug_assert!(self.uint < Self::modulus());
        P::UInt::from_montgomery::<Montgomery<P>>(self.as_montgomery())
    }

    /// Construct from `UInt`
    ///
    /// It does the montgomery conversion.
    pub fn from_uint(uint: &P::UInt) -> Self {
        debug_assert!(uint < &Self::modulus());
        Self::from_montgomery(uint.to_montgomery::<Montgomery<P>>())
    }

    /// Reduce and construct from `UInt`
    pub fn from_uint_reduce(uint: &P::UInt) -> Self {
        let uint = P::UInt::redc_inline::<Montgomery<P>>(uint, &P::UInt::zero());
        // UInt should not have interior mutability
        #[allow(clippy::borrow_interior_mutable_const)]
        let uint = uint.mul_redc_inline::<Montgomery<P>>(&P::R3);
        Self::from_montgomery(uint)
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

impl<P: Parameters> Clone for PrimeField<P> {
    fn clone(&self) -> Self {
        Self::from_montgomery(self.as_montgomery().clone())
    }
}

impl<P: Parameters> PartialEq for PrimeField<P> {
    fn eq(&self, other: &Self) -> bool {
        self.as_montgomery() == other.as_montgomery()
    }
}

impl<P: Parameters> Eq for PrimeField<P> {}

/// Implements [`Hash`] when `UInt` does.
impl<U, P> Hash for PrimeField<P>
where
    U: FieldUInt + Hash,
    P: Parameters<UInt = U>,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_montgomery().hash::<H>(state)
    }
}

impl<U, P> fmt::Debug for PrimeField<P>
where
    U: FieldUInt + fmt::Debug,
    P: Parameters<UInt = U>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "field_element!(\"{:?}\")", self.to_uint())
    }
}

impl<P: Parameters> Zero for PrimeField<P> {
    #[inline(always)]
    fn zero() -> Self {
        Self::from_montgomery(P::UInt::zero())
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.as_montgomery().is_zero()
    }
}

impl<P: Parameters> One for PrimeField<P> {
    #[inline(always)]
    fn one() -> Self {
        Self::from_montgomery(P::R1)
    }

    // UInt should not have interior mutability
    #[allow(clippy::borrow_interior_mutable_const)]
    #[inline(always)]
    fn is_one(&self) -> bool {
        self.as_montgomery() == &P::R1
    }
}

impl<P: Parameters> AddInline<&Self> for PrimeField<P> {
    #[inline(always)]
    fn add_inline(&self, rhs: &Self) -> Self {
        let result = self.as_montgomery().add_inline(rhs.as_montgomery());
        let result = result.reduce_1_inline::<Montgomery<P>>();
        Self::from_montgomery(result)
    }
}

impl<P: Parameters> SubInline<&Self> for PrimeField<P> {
    #[inline(always)]
    fn sub_inline(&self, rhs: &Self) -> Self {
        let lhs = self.as_montgomery();
        let rhs = rhs.as_montgomery();
        let borrow = rhs > lhs;
        let mut result = lhs.sub_inline(rhs);
        if borrow {
            result.add_assign_inline(&Self::modulus());
        }
        Self::from_montgomery(result)
    }
}

impl<P: Parameters> NegInline for PrimeField<P> {
    #[inline(always)]
    fn neg_inline(&self) -> Self {
        if self.is_zero() {
            Self::zero()
        } else {
            Self::from_montgomery(Self::modulus().sub_inline(self.as_montgomery()))
        }
    }
}

impl<P: Parameters> SquareInline for PrimeField<P> {
    #[inline(always)]
    fn square_inline(&self) -> Self {
        Self::from_montgomery(self.as_montgomery().square_redc_inline::<Montgomery<P>>())
    }
}

impl<P: Parameters> MulInline<&Self> for PrimeField<P> {
    #[inline(always)]
    fn mul_inline(&self, rhs: &Self) -> Self {
        Self::from_montgomery(
            self.as_montgomery()
                .mul_redc_inline::<Montgomery<P>>(rhs.as_montgomery()),
        )
    }
}

impl<P: Parameters> Inv for &PrimeField<P> {
    type Output = Option<PrimeField<P>>;

    #[inline(always)] // Simple wrapper
    fn inv(self) -> Self::Output {
        self.as_montgomery()
            .inv_redc::<Montgomery<P>>()
            .map(PrimeField::<P>::from_montgomery)
    }
}

impl<P: Parameters> Pow<usize> for &PrimeField<P> {
    type Output = PrimeField<P>;

    fn pow(self, exponent: usize) -> Self::Output {
        self.pow(&exponent)
    }
}

impl<P: Parameters> Pow<isize> for &PrimeField<P> {
    type Output = Option<PrimeField<P>>;

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

impl<P: Parameters, Exponent> Pow<&Exponent> for &PrimeField<P>
where
    Exponent: Binary,
{
    type Output = PrimeField<P>;

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

impl<U, P> Root<usize> for PrimeField<P>
where
    U: FieldUInt + Binary + DivRem<u64, Quotient = U, Remainder = u64>,
    P: Parameters<UInt = U>,
{
    // OPT: replace this with a constant array of roots of unity.
    fn root(order: usize) -> Option<Self> {
        let order = order as u64;
        if let Some((q, rem)) = Self::order().div_rem(order) {
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
impl<U, P> Root<&U> for PrimeField<P>
where
    U: FieldUInt + Binary + for<'a> DivRem<&'a U, Quotient = U, Remainder = U>,
    P: Parameters<UInt = U>,
{
    // OPT: replace this with a constant array of roots of unity.
    fn root(order: &P::UInt) -> Option<Self> {
        if let Some((q, rem)) = Self::order().div_rem(order) {
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

impl<U, P> SquareRoot for PrimeField<P>
where
    U: FieldUInt + Binary + Shr<usize, Output = U>,
    P: Parameters<UInt = U>,
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
        let trailing_zeros = Self::order().trailing_zeros();
        let signifcant = Self::order() >> trailing_zeros;
        // The starting value of c in the Tonelli Shanks algorithm. We are using the
        // prefered generator, as the quadratic nonresidue the algorithm requires.
        let c_start = Self::generator().pow(&signifcant);

        // This algorithm is still correct when the following assertion fails. However,
        // more efficient algorithms exist when MODULUS % 4 == 3 or MODULUS % 8 == 5
        // (3.36 and 3.37 in HAC).
        // debug_assert!(&FieldElement::MODULUS & 7_u64 == 1);

        // OPT: Raising a to a fixed power is a good candidate for an addition chain.
        let mut root = self.pow(&((signifcant + P::UInt::one()) >> 1));
        let mut c = c_start;
        let inverse = self.inv().unwrap(); // Zero case is handled above

        for i in 1..trailing_zeros {
            if (root.square() * &inverse).pow(&(P::UInt::one() << (trailing_zeros - i - 1)))
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

impl<P: Parameters> Default for PrimeField<P> {
    fn default() -> Self {
        Self::zero()
    }
}

// TODO: Find a way to create generic implementations of these
impl<P: Parameters<UInt = U256>> From<PrimeField<P>> for U256 {
    #[inline(always)]
    fn from(other: PrimeField<P>) -> Self {
        other.to_uint()
    }
}

impl<P: Parameters<UInt = U256>> From<&PrimeField<P>> for U256 {
    #[inline(always)]
    fn from(other: &PrimeField<P>) -> Self {
        other.to_uint()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FieldElement;
    use itertools::repeat_n;
    use num_traits::ToPrimitive;
    use proptest::prelude::*;
    use zkp_macros_decl::{field_element, u256h};
    use zkp_u256::U256;

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
        let c = field_element!("03d7be0dd45f307519282c76caedd14b3ead2be9cb6512ab60cfd7dfeb5a806a");
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
        let c = field_element!("0738900c5dcab24b419674df19d2cfeb9782eca6d1107be18577eb060390365b");
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
        let c = field_element!("003a9a346e7103c74dfcddd0eeb4e16ca71d8887c2bed3d4ee718b62015e87b2");
        assert_eq!(a / b, c);
    }

    proptest!(
        #[test]
        fn from_as_isize(n: isize) {
            prop_assert_eq!(FieldElement::from(n).to_isize().unwrap(), n)
        }

        #[test]
        fn from_as_i128(n: i128) {
            prop_assert_eq!(FieldElement::from(n).to_i128().unwrap(), n);
        }

        #[test]
        fn add_identity(a: FieldElement) {
            prop_assert_eq!(&a + FieldElement::zero(), a);
        }

        #[test]
        fn mul_identity(a: FieldElement) {
            prop_assert_eq!(&a * FieldElement::one(), a);
        }

        #[test]
        fn commutative_add(a: FieldElement, b: FieldElement) {
            prop_assert_eq!(&a + &b, b + a);
        }

        #[test]
        fn commutative_mul(a: FieldElement, b: FieldElement) {
            prop_assert_eq!(&a * &b, b * a);
        }

        #[test]
        fn associative_add(a: FieldElement, b: FieldElement, c: FieldElement) {
            prop_assert_eq!(&a + (&b + &c), (a + b) + c);
        }

        #[test]
        fn associative_mul(a: FieldElement, b: FieldElement, c: FieldElement) {
            prop_assert_eq!(&a * (&b * &c), (a * b) * c);
        }

        #[test]
        fn inverse_add(a: FieldElement) {
            prop_assert!((&a + a.neg()).is_zero());
        }

        #[test]
        fn inverse_mul(a: FieldElement) {
            let inverse = a.inv();
            match inverse {
                None => prop_assert!(a.is_zero()),
                Some(ai) => prop_assert!((a * ai).is_one()),
            }
        }

        #[test]
        fn distributivity(a: FieldElement, b: FieldElement, c: FieldElement) {
            prop_assert_eq!(&a * (&b + &c), (&a * b) + (a * c));
        }

        #[test]
        fn square(a: FieldElement) {
            prop_assert_eq!(a.square(), &a * &a);
        }

        #[test]
        fn pow_0(a: FieldElement) {
            prop_assert!(a.pow(0_usize).is_one());
        }

        #[test]
        fn pow_1(a: FieldElement) {
            prop_assert_eq!(a.pow(1_usize), a);
        }

        #[test]
        fn pow_2(a: FieldElement) {
            prop_assert_eq!(a.pow(2_usize), &a * &a);
        }

        #[test]
        fn pow_n(a: FieldElement, n: usize) {
            let exponent = n % 512;
            prop_assert_eq!(a.pow(exponent), repeat_n(a, exponent).product());
        }

        #[test]
        fn fermats_little_theorem(a: FieldElement) {
            prop_assert_eq!(a.pow(&FieldElement::MODULUS), a);
        }

        #[test]
        fn square_root(a: FieldElement) {
            let s = a.square();
            let r = s.square_root().unwrap();
            prop_assert!(r == a || r == -a);
        }
    );

    #[test]
    fn zeroth_root_of_unity() {
        assert_eq!(FieldElement::root(0).unwrap(), FieldElement::one());
    }

    #[test]
    fn roots_of_unity_squared() {
        let powers_of_two = (0..193).map(|n| U256::ONE << n);
        let roots_of_unity: Vec<_> = powers_of_two
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
