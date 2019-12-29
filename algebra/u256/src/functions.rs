use crate::{
    algorithms::{gcd, gcd_extended, montgomery},
    Montgomery, MontgomeryParameters, GCD, U256, SquareInline, InvMod
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

impl Montgomery for U256 {
    #[inline(always)]
    fn redc_inline<M: MontgomeryParameters<U256>>(lo: &Self, hi: &Self) -> Self {
        montgomery::redc_inline::<M>(lo, hi)
    }

    #[inline(always)]
    fn square_redc_inline<M: MontgomeryParameters<U256>>(&self) -> Self {
        let (lo, hi) = self.square_full_inline();
        Self::redc_inline::<M>(&lo, &hi)
    }

    #[inline(always)]
    fn mul_redc_inline<M: MontgomeryParameters<U256>>(&self, rhs: &Self) -> Self {
        montgomery::mul_redc_inline::<M>(self, rhs)
    }

    // Inline to reduce to `inv_mod` + `mul_redc`
    #[inline(always)]
    fn inv_redc<M: MontgomeryParameters<U256>>(&self) -> Option<Self> {
        self.inv_mod(&M::MODULUS).map(|ni| ni.mul_redc::<M>(&M::R3))
    }
}
