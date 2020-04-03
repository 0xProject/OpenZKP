use super::PolyWriter;
use itertools::Itertools;
use std::ops::{Index, IndexMut};
use zkp_primefield::FieldElement;

pub struct Transform {
    polynomial: usize,
    size:       usize,
    offset:     usize,
    stride:     usize,
}

impl Transform {
    fn size(&self) -> usize {
        self.size
    }

    fn map(&self, location: usize) -> (usize, usize) {
        assert!(location < self.size);
        (self.polynomial, self.offset + self.stride * location)
    }
}

pub struct Transformed<'a, P: PolyWriter> {
    inner:   &'a mut P,
    mapping: Vec<Transform>,
}

impl<'a, P: PolyWriter> Transformed<'a, P> {
    pub fn new(inner: &'a mut P, mapping: Vec<Transform>) -> Self {
        assert!(mapping.iter().map(|t| t.size()).all_equal());
        Self { inner, mapping }
    }

    pub fn map(&self, polynomial: usize, location: usize) -> (usize, usize) {
        assert!(polynomial < self.mapping.len());
        self.mapping[polynomial].map(location)
    }
}

impl<'a, P: PolyWriter> PolyWriter for Transformed<'a, P> {
    fn dimensions(&self) -> (usize, usize) {
        (
            self.mapping.len(),
            self.mapping.first().map_or(0, |t| t.size()),
        )
    }

    fn write(&mut self, polynomial: usize, location: usize, value: FieldElement) {
        let (polynomial, location) = self.map(polynomial, location);
        self.inner.write(polynomial, location, value)
    }
}
