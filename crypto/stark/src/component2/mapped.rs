use super::PolyWriter;
use itertools::Itertools;
use std::ops::{Index, IndexMut};
use zkp_primefield::FieldElement;

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

impl<'a, P: PolyWriter, F: Fn(usize, usize) -> (usize, usize)> PolyWriter for Mapped<'a, P, F> {
    fn dimensions(&self) -> (usize, usize) {
        self.dimensions
    }

    fn write(&mut self, polynomial: usize, location: usize, value: &FieldElement) {
        // debug_assert!(polynomial < self.dimensions.0);
        // debug_assert!(location < self.dimensions.1);
        let (polynomial, location) = (self.map)(polynomial, location);
        self.inner.write(polynomial, location, value)
    }
}
