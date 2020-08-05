use crate::{
    algorithms::{mul_redc_inline, redc_inline, square_redc_inline},
    arch::{gcd, gcd_extended, reduce_1},
    InvMod, Montgomery, MontgomeryParameters, GCD, U256,
};

impl GCD for U256 {
    #[inline(always)]
    fn gcd(a: &Self, b: &Self) -> Self {
        gcd(a.clone(), b.clone())
    }

    #[inline(always)]
    fn gcd_extended(a: &Self, b: &Self) -> (Self, Self, Self, bool) {
        gcd_extended(a.clone(), b.clone())
    }
}

// TODO: Provide methods to compute parameters from Modulus
// tricks from <https://medium.com/wicketh/mathemagic-512-bit-division-in-solidity-afa55870a65>
// can help here. Extra credit: make it a `const fn`.

impl Montgomery for U256 {
    #[inline(always)]
    fn reduce_1_inline<M: MontgomeryParameters<UInt = U256>>(&self) -> Self {
        reduce_1(self, &M::MODULUS)
    }

    #[inline(always)]
    fn redc_inline<M: MontgomeryParameters<UInt = U256>>(lo: &Self, hi: &Self) -> Self {
        redc_inline::<M>(lo, hi)
    }

    #[inline(always)]
    fn square_redc_inline<M: MontgomeryParameters<UInt = U256>>(&self) -> Self {
        square_redc_inline::<M>(self)
    }

    #[inline(always)]
    fn mul_redc_inline<M: MontgomeryParameters<UInt = U256>>(&self, rhs: &Self) -> Self {
        mul_redc_inline::<M>(self, rhs)
    }

    // Inline to reduce to `inv_mod` + `mul_redc`
    #[inline(always)]
    fn inv_redc<M: MontgomeryParameters<UInt = U256>>(&self) -> Option<Self> {
        self.inv_mod(&M::MODULUS).map(|ni| ni.mul_redc::<M>(&M::R3))
    }
}
