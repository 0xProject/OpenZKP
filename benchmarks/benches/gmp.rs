use rug::Integer;
use criterion::{black_box, Bencher};

// Uses mpz_invert(n, modulus)
// See https://gmplib.org/repo/gmp/file/tip/mpz/invert.c
// which in turn uses mpz_gcdext(n. modulus);
// See https://gmplib.org/repo/gmp/file/tip/mpz/gcdext.c
// which in turn uses mpn_gcdext(n, modulus)
// See https://gmplib.org/repo/gmp/file/tip/mpn/generic/gcdext.c
// See https://gmplib.org/manual/Greatest-Common-Divisor-Algorithms.html#Greatest-Common-Divisor-Algorithms
// For 1 and 2 limbs, Binary GCD is used
// See https://gmplib.org/manual/Binary-GCD.html#Binary-GCD
// Between 3 and GCDEXT_DC_THRESHOLD Lehmer's GCD is used
// See https://gmplib.org/devel/thres/GCDEXT_DC_THRESHOLD.html
// See https://gmplib.org/manual/Lehmer_0027s-Algorithm.html#Lehmer_0027s-Algorithm
// See https://www.csie.nuk.edu.tw/~cychen/gcd/A%20double-digit%20Lehmer-Euclid%20algorithm%20for%20finding%20the%20GCD%20of%20long%20integers.pdf
// The GCDEXT_DC_THRESHOLD is > 100 limbs, so not relevant for us.
// In our case, we want Lehmer's algorithm.
// See also http://www.csd.uwo.ca/~moreno/CS424/Ressources/ComparingSeveralGCDAlgorithms.Jebelean.1993.pdf
// See also http://www.sdiwc.net/ijncaa/files/IJNCAA_Vol7No1.pdf
// See also https://pdfs.semanticscholar.org/a7e7/b01a3dd6ac0ec160b35e513c5efa38c2369e.pdf
// See also https://stackoverflow.com/questions/16989677/lehmers-extended-gcd-algorithm-implementation
pub fn gmp_field_inv(bench: &mut Bencher, _i: &()) {
    let m = Integer::from_str_radix("0800000000000011000000000000000000000000000000000000000000000001", 16).unwrap();
    let a = Integer::from_str_radix("03d937c035c878245caf64531a5756109c53068da139362728feb561405371cb", 16).unwrap();
    bench.iter(|| {
        black_box(black_box(&a).clone().invert(&m));
    })
}
