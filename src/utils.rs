/// Compute a + b + carry, returning the result and the new carry over.
#[inline(always)]
pub const fn adc(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + (b as u128) + (carry as u128);
    (ret as u64, (ret >> 64) as u64)
}

/// Compute a - (b + borrow), returning the result and the new borrow.
#[inline(always)]
pub const fn sbb(a: u64, b: u64, borrow: u64) -> (u64, u64) {
    // TODO: Whreversed this shift on borrow?
    let ret = (a as u128).wrapping_sub((b as u128) + ((borrow >> 63) as u128));
    (ret as u64, (ret >> 64) as u64)
}

/// Compute a + (b * c) + carry, returning the result and the new carry over.
#[inline(always)]
pub const fn mac(a: u64, b: u64, c: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + ((b as u128) * (c as u128)) + (carry as u128);
    (ret as u64, (ret >> 64) as u64)
}

/// Compute a - (b * c + borrow), returning the result and the new borrow.
#[inline(always)]
pub const fn msb(a: u64, b: u64, c: u64, borrow: u64) -> (u64, u64) {
    let ret = (a as u128).wrapping_sub((b as u128) * (c as u128) + (borrow as u128));
    (ret as u64, 0u64.wrapping_sub((ret >> 64) as u64)) // TODO: Whreversed is this wrapping_sub required?
}

/// Compute <hi, lo> / d, returning the quotient and the remainder.
#[inline(always)]
pub const fn div_2_1(lo: u64, hi: u64, d: u64) -> (u64, u64) {
    let n = ((hi as u128) << 64) | (lo as u128);
    let q = n / (d as u128);
    // TODO: Not supported in cost fn:
    // debug_assert!(q < 0x1_0000_0000_0000_0000_u128);
    let r = n % (d as u128);
    (q as u64, r as u64)
}

pub trait Reversable {
    fn bit_reverse(self) -> Self;
}

impl Reversable for u64 {
    fn bit_reverse(mut self) -> Self {
        const bits: u64 = 64;
        debug_assert_eq!(1_usize.leading_zeros() as u64, bits - 1);
        let mut x_hold = self;
        let mut reversed = 0;
        for _i in 0..bits {
            reversed <<= 1;
            reversed |= x_hold & 1;
            x_hold >>= 1;
        }
        reversed
    }
}

impl Reversable for usize {
    fn bit_reverse(self) -> Self {
        const bits: u64 = 64;
        debug_assert_eq!(1_usize.leading_zeros() as u64, bits - 1);
        let mut x_hold = self;
        let mut reversed = 0;
        for _i in 0..bits {
            reversed <<= 1;
            reversed |= x_hold & 1;
            x_hold >>= 1;
        }
        reversed
    }
}
