use crate::{
    curve::Affine,
    field::FieldElement,
    pedersen::SHIFT_POINT,
    pedersen_merkle::input::{PrivateInput, PublicInput},
    pedersen_points::PEDERSEN_POINTS,
    u256::U256,
};
use std::default::Default;

pub fn get_trace_table(
    public_input: &PublicInput,
    private_input: &PrivateInput,
) -> Vec<FieldElement> {
    let mut state: Row = Default::default();
    state.right.point = Affine::Point {
        x: public_input.leaf.clone(),
        y: FieldElement::ZERO,
    };

    let mut trace_table: Vec<FieldElement> =
        Vec::with_capacity(public_input.path_length * 256 * 8 as usize);
    for path_index in 0..public_input.path_length {
        for bit_index in 0..256 {
            if bit_index % 256 == 0 {
                let other_hash = U256::from(&private_input.path[path_index]);
                let (x, _) = get_coordinates(&state.right.point);
                if !private_input.directions[path_index] {
                    state = initialize_hash(U256::from(x), other_hash);
                } else {
                    state = initialize_hash(other_hash, U256::from(x));
                }
            } else {
                state = hash_next_bit(&state, bit_index);
            }
            extend_trace_table_by_row(&mut trace_table, &state);
        }
    }
    trace_table
}

fn initialize_hash(left_source: U256, right_source: U256) -> Row {
    let mut state: Row = Default::default();
    state.left.source = left_source;
    state.right.source = right_source;
    state.right.point = SHIFT_POINT;
    state
}

fn hash_next_bit(state: &Row, bit_index: usize) -> Row {
    let mut next_state = Row {
        left:  Subrow {
            source: state.left.source.clone() >> 1,
            point: state.right.point.clone(),
            ..Default::default()
        },
        right: Subrow {
            source: state.right.source.clone() >> 1,
            ..Default::default()
        },
    };
    if state.left.source.bit(0) {
        let p = &PEDERSEN_POINTS[bit_index];
        next_state.left.slope = get_slope(&next_state.left.point, &p);
        next_state.left.point += p;
    }

    next_state.right.point = next_state.left.point.clone();
    if state.right.source.bit(0) {
        let p = &PEDERSEN_POINTS[bit_index + 252];
        next_state.right.slope = get_slope(&next_state.right.point, &p);
        next_state.right.point += p;
    }
    next_state
}

fn extend_trace_table_by_row(table: &mut Vec<FieldElement>, row: &Row) {
    extend_trace_table_by_subrow(table, &row.left);
    extend_trace_table_by_subrow(table, &row.right);
}

fn extend_trace_table_by_subrow(table: &mut Vec<FieldElement>, subrow: &Subrow) {
    let (x, y) = get_coordinates(&subrow.point);
    table.push(FieldElement(subrow.source.clone()));
    table.push(subrow.slope.clone());
    table.push(x.clone());
    table.push(y.clone());
}

#[derive(Default)]
struct Row {
    left:  Subrow,
    right: Subrow,
}

struct Subrow {
    source: U256,
    slope:  FieldElement,
    point:  Affine,
}

impl Default for Subrow {
    fn default() -> Self {
        Subrow {
            source: U256::ZERO,
            slope:  FieldElement::ZERO,
            point:  Affine::Point {
                x: FieldElement::ZERO,
                y: FieldElement::ZERO,
            },
        }
    }
}

fn get_slope(p_1: &Affine, p_2: &Affine) -> FieldElement {
    let (x_1, y_1) = get_coordinates(p_1);
    let (x_2, y_2) = get_coordinates(p_2);
    (y_1 - y_2) / (x_1 - x_2)
}

fn get_coordinates(p: &Affine) -> (&FieldElement, &FieldElement) {
    match p {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x, y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        pedersen::old_hash,
        pedersen_merkle::input::{get_private_input, get_public_input},
    };
    use itertools::Itertools;
    use std::iter;

    #[test]
    fn trace_table_is_correct() {
        let public_input = get_public_input();
        let trace_table = get_trace_table(&public_input, &get_private_input());
        assert_eq!(
            U256::from(&trace_table[trace_table.len() - 2]),
            U256::from(&public_input.root)
        );
    }

    #[test]
    fn root_hash_is_correct() {
        let public_input = get_public_input();
        let root = parallel_construction(&public_input, &get_private_input());
        assert_eq!(root, public_input.root);
    }

    fn generate_traces(left_root: &U256, right_root: &U256) -> Vec<FieldElement> {
        let left_sources = (0..256).map(|i| FieldElement(left_root.clone() >> i));
        let right_sources = (0..256).map(|i| FieldElement(right_root.clone() >> i));

        let left_bits = (0..256).map(|i| left_root.bit(i));
        let right_bits = (0..256).map(|i| right_root.bit(i));

        // Cycle these to pad out to over 256 elements. We don't need the final values.
        let left_pedersen_points = PEDERSEN_POINTS[1..1 + 252].iter().cycle();
        let right_pedersen_points = PEDERSEN_POINTS[253..253 + 252].iter().cycle();

        let slopes_and_points = left_bits
            .interleave(right_bits)
            .zip(left_pedersen_points.interleave(right_pedersen_points))
            .scan(SHIFT_POINT, |state, (bit, pedersen_point)| {
                let mut slope = FieldElement::ZERO;
                if bit {
                    slope = get_slope(state, pedersen_point);
                    *state += pedersen_point;
                }
                Some((slope, state.clone()))
            });

        left_sources
            .interleave(right_sources)
            .zip(slopes_and_points)
            .map(|(source, (slope, point))| {
                let (x, y) = get_coordinates(&point);
                iter::once(source)
                    .chain(iter::once(slope))
                    .chain(iter::once(x.clone()))
                    .chain(iter::once(y.clone()))
            })
            .flatten()
            .collect()
    }

    fn parallel_construction(
        public_input: &PublicInput,
        private_input: &PrivateInput,
    ) -> FieldElement {
        let mut root = U256::from(&public_input.leaf);
        for (f, direction) in private_input
            .path
            .iter()
            .zip(private_input.directions.clone())
        {
            let other_root = U256::from(f);
            if !direction {
                root = old_hash(&[root, other_root]);
            } else {
                root = old_hash(&[other_root, root]);
            }
        }
        FieldElement::from(root)
    }

    fn get_hashes(public_input: &PublicInput, private_input: &PrivateInput) -> Vec<(U256, U256)> {
        let mut result: Vec<(U256, U256)> = vec![];
        let mut root = U256::from(&public_input.leaf);
        for (f, direction) in private_input
            .path
            .iter()
            .zip(private_input.directions.clone())
        {
            let other_root = U256::from(f);
            if !direction {
                result.push((root.clone(), other_root.clone()));
                root = old_hash(&[root, other_root]);
            } else {
                result.push((other_root.clone(), root.clone()));
                root = old_hash(&[other_root, root]);
            }
        }
        result
    }
}
