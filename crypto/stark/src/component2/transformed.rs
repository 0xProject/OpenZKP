use super::PolynomialBuilder;
use std::ops::{Index, IndexMut};

struct Transform {
    polynomial: usize,
    size:       usize,
    offset:     usize,
    stride:     usize,
}

impl Transform {
    fn size(&self) -> usize {
        self.size
    }

    fn map(&self, index: usize) -> (usize, usize) {
        assert!(index < self.size);
        (self.polynomial, self.offset + self.stride * index)
    }
}

pub struct Transformed<P: PolynomialBuilder> {
    inner:       P,
    polynomials: Vec<Transform>,
}

impl<P: PolynomialBuilder> Transformed<P> {
    pub fn take(self) -> P {
        self.inner
    }
}

impl<P: PolynomialBuilder> Transformed<P> {
    fn map(&self, (polynomial, index): (usize, usize)) -> (usize, usize) {
        assert!(polynomial < self.polynomials.len());
        self.polynomials[polynomial].map(index)
    }
}

impl<P: PolynomialBuilder> Index<(usize, usize)> for Transformed<P> {
    type Output = <P as Index<(usize, usize)>>::Output;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        self.inner.index(self.map(index))
    }
}

impl<P: PolynomialBuilder> IndexMut<(usize, usize)> for Transformed<P> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        self.inner.index_mut(self.map(index))
    }
}

impl<P: PolynomialBuilder> PolynomialBuilder for Transformed<P> {
    fn count(&self) -> usize {
        self.polynomials.len()
    }

    fn size(&self, polynomial: usize) -> std::primitive::usize {
        assert!(polynomial < self.polynomials.len());
        self.polynomials[polynomial].size()
    }
}
