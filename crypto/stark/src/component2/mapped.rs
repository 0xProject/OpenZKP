use super::PolyWriter;
use zkp_primefield::FieldElement;

#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Debug))]
// False positive, lifetime needs to be explicit here
#[allow(single_use_lifetimes)]
pub struct Mapped<'a, P: PolyWriter, F: Fn(usize, usize) -> (usize, usize)> {
    inner:      &'a mut P,
    dimensions: (usize, usize),
    map:        F,
}

impl<'a, P: PolyWriter, F: Fn(usize, usize) -> (usize, usize)> Mapped<'a, P, F> {
    pub fn new(inner: &'a mut P, dimensions: (usize, usize), map: F) -> Self {
        Self {
            inner,
            dimensions,
            map,
        }
    }
}

impl<P: PolyWriter, F: Fn(usize, usize) -> (usize, usize)> PolyWriter for Mapped<'_, P, F> {
    fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    fn write(&mut self, polynomial: usize, location: usize, value: FieldElement) {
        debug_assert!(polynomial < self.dimensions.0);
        debug_assert!(location < self.dimensions.1);
        let (polynomial, location) = (self.map)(polynomial, location);
        self.inner.write(polynomial, location, value)
    }
}
