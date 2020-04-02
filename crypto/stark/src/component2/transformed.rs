use super::PolynomialBuilder;
use std::ops::{Index, IndexMut};

struct Transform {
    polynomial: usize,
    length:     usize,
    offset:     usize,
    stride:     usize,
}

impl Transform {
    fn map(&self, row: usize) -> (usize, usize) {
        assert!(row < self.length);
        (self.polynomial, self.offset + self.stride * row)
    }
}

pub struct Transformed<P: PolynomialBuilder> {
    inner:   P,
    columns: Vec<Transform>,
}

impl<P: PolynomialBuilder> Transformed<P> {
    fn map(&self, (row, column): (usize, usize)) -> (usize, usize) {
        assert!(column < self.columns.len());
        self.columns[column].map(row)
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
