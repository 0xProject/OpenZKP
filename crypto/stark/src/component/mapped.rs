use super::PolynomialWriter;
use zkp_primefield::FieldElement;

#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
// False positive, lifetime needs to be explicit here
#[allow(single_use_lifetimes)]
pub struct Mapped<'a, P: PolynomialWriter, F: Fn(usize, usize) -> (usize, usize)> {
    inner:           &'a mut P,
    num_polynomials: usize,
    polynomial_size: usize,
    map:             F,
}

impl<'a, P: PolynomialWriter, F: Fn(usize, usize) -> (usize, usize)> Mapped<'a, P, F> {
    pub fn new(inner: &'a mut P, num_polynomials: usize, polynomial_size: usize, map: F) -> Self {
        Self {
            inner,
            num_polynomials,
            polynomial_size,
            map,
        }
    }
}

impl<P: PolynomialWriter, F: Fn(usize, usize) -> (usize, usize)> PolynomialWriter
    for Mapped<'_, P, F>
{
    fn write(&mut self, polynomial: usize, location: usize, value: FieldElement) {
        debug_assert!(polynomial < self.num_polynomials);
        debug_assert!(location < self.polynomial_size);
        let (polynomial, location) = (self.map)(polynomial, location);
        self.inner.write(polynomial, location, value)
    }

    fn num_polynomials(&self) -> usize {
        self.num_polynomials
    }

    fn polynomial_size(&self) -> usize {
        self.polynomial_size
    }
}
