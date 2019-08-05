use primefield::FieldElement;
use std::ops::{Index, IndexMut};

pub struct TraceTable {
    trace_length: usize,
    num_columns:  usize,
    values:       Vec<FieldElement>,
}

impl TraceTable {
    pub fn new(trace_length: usize, num_columns: usize) -> TraceTable {
        TraceTable {
            trace_length,
            num_columns,
            values: vec![FieldElement::ZERO; trace_length * num_columns],
        }
    }
}

impl Index<usize> for TraceTable {
    type Output = [FieldElement];

    fn index(&self, i: usize) -> &[FieldElement] {
        &self.values[i * self.num_columns..(i + 1) * self.num_columns]
    }
}

impl Index<(usize, usize)> for TraceTable {
    type Output = FieldElement;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.values[i * self.num_columns + j]
    }
}

impl IndexMut<usize> for TraceTable {
    fn index_mut(&mut self, i: usize) -> &mut [FieldElement] {
        &mut self.values[i * self.num_columns..(i + 1) * self.num_columns]
    }
}

impl IndexMut<(usize, usize)> for TraceTable {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.values[i * self.num_columns + j]
    }
}
