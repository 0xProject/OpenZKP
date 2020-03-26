/// Compute a + b + carry, returning the result and the new carry over.
#[inline(always)]
pub const fn adc(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + (b as u128) + (carry as u128);
    // We want truncation here
    #[allow(clippy::cast_possible_truncation)]
    (ret as u64, (ret >> 64) as u64)
}

/// Compute a - (b + borrow), returning the result and the new borrow.
#[inline(always)]
pub const fn sbb(a: u64, b: u64, borrow: u64) -> (u64, u64) {
    let ret = (a as u128).wrapping_sub((b as u128) + (borrow as u128));
    // We want truncation here
    #[allow(clippy::cast_possible_truncation)]
    (ret as u64, (ret >> 127) as u64)
}

/// Compute a + (b * c) + carry, returning the result and the new carry over.
/// Both `a` and `carry` can be the full 64-bit range
#[inline(always)]
pub const fn mac(a: u64, b: u64, c: u64, carry: u64) -> (u64, u64) {
    let ret = (a as u128) + ((b as u128) * (c as u128)) + (carry as u128);
    // We want truncation here
    #[allow(clippy::cast_possible_truncation)]
    (ret as u64, (ret >> 64) as u64)
}

/// Compute a - (b * c + borrow), returning the result and the new borrow.
#[inline(always)]
pub const fn msb(a: u64, b: u64, c: u64, borrow: u64) -> (u64, u64) {
    let ret = (a as u128).wrapping_sub((b as u128) * (c as u128) + (borrow as u128));
    // TODO: Why is this wrapping_sub required?
    // We want truncation here
    #[allow(clippy::cast_possible_truncation)]
    (ret as u64, 0_u64.wrapping_sub((ret >> 64) as u64))
}

/// Compute <hi, lo> / d, returning the quotient and the remainder.
#[inline(always)]
pub const fn div_2_1(lo: u64, hi: u64, d: u64) -> (u64, u64) {
    let n = ((hi as u128) << 64) | (lo as u128);
    let q = n / (d as u128);
    // TODO: Not supported in cost fn:
    // debug_assert!(q < 0x1_0000_0000_0000_0000_u128);
    let r = n % (d as u128);
    // We want truncation here
    #[allow(clippy::cast_possible_truncation)]
    (q as u64, r as u64)
}
