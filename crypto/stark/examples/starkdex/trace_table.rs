use crate::pedersen_points::{PEDERSEN_POINTS, SHIFT_POINT};
use zkp_elliptic_curve::Affine;
use zkp_primefield::FieldElement;
use zkp_u256::U256;

fn get_coordinates(point: &Affine) -> (FieldElement, FieldElement) {
    match point {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x.clone(), y.clone()),
    }
}

fn get_slope(p_1: &Affine, p_2: &Affine) -> FieldElement {
    let (x_1, y_1) = get_coordinates(p_1);
    let (x_2, y_2) = get_coordinates(p_2);
    (y_1 - y_2) / (x_1 - x_2)
}

fn shift_right(x: &FieldElement) -> FieldElement {
    (U256::from(x) >> 1).into()
}

fn get_pedersen_hash_columns(
    left: &FieldElement,
    right: &FieldElement,
) -> (
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
) {
    let mut sources = vec![U256::from(left)];
    for i in 0..255 {
        sources.push(sources[i].clone() >> 1);
    }
    sources.push(U256::from(right));
    for i in 256..512 {
        sources.push(sources[i].clone() >> 1);
    }
    let mut sum = SHIFT_POINT;
    let mut x_coordinates = Vec::with_capacity(512);
    let mut y_coordinates = Vec::with_capacity(512);
    let mut slopes = vec![FieldElement::ZERO; 512];
    for (i, source) in sources.iter().enumerate() {
        let (x, y) = get_coordinates(&sum);
        x_coordinates.push(x);
        y_coordinates.push(y);

        if source.bit(0) {
            let point = PEDERSEN_POINTS[if i < 256 { i + 1 } else { i - 4 }].clone();
            slopes[i] = get_slope(&sum, &point);
            sum += point;
        }
    }

    (
        sources.iter().map(|x| FieldElement::from(x)).collect(),
        x_coordinates,
        y_coordinates,
        slopes,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constraints::constraints,
        inputs::{Claim, Modification, Parameters, SignatureParameters, Tree, Vault, Vaults},
        pedersen_points::SHIFT_POINT,
    };
    use itertools::izip;
    use zkp_macros_decl::field_element;
    use zkp_stark::{check_constraints, Constraints, TraceTable};

    #[test]
    fn test_trace() {
        let parameters = Parameters {
            signature:        SignatureParameters {
                shift_point: SHIFT_POINT,
                alpha:       FieldElement::ONE,
                beta:        field_element!(
                    "06f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89"
                ),
            },
            hash_shift_point: SHIFT_POINT,
            n_vaults:         30,
        };

        let mut vaults = Vaults::new();
        let initial_root = vaults.root();

        let update_index = 2341972323;
        let vault = Vault {
            key:    FieldElement::GENERATOR,
            token:  FieldElement::NEGATIVE_ONE,
            amount: 1000,
        };
        vaults.update(update_index, vault);
        let final_root = vaults.root();

        let path_index = 1234123123;
        let (vault, path) = vaults.path(path_index);

        let claim = Claim {
            n_transactions:      1,
            modifications:       vec![Modification {
                initial_amount: 123412,
                final_amount:   1000,
                index:          0,
                key:            field_element!(
                    "057d5d2e5da7409db60d64ae4e79443fedfd5eb925b5e54523eaf42cc1978169"
                ),
                token:          field_element!(
                    "03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d"
                ),
                vault:          1,
            }],
            initial_vaults_root: initial_root,
            final_vaults_root:   final_root,
        };

        let constraints = constraints(&claim, &parameters);
        let trace_length = claim.n_transactions * 65536;

        let system =
            Constraints::from_expressions((trace_length, 10), vec![], constraints).unwrap();

        let mut trace_table = TraceTable::new(trace_length, 10);
        let mut hash_pool_accumulator = SHIFT_POINT.clone();

        let dummy_4: FieldElement = 123412341.into();

        for transaction_index in 0..claim.n_transactions {
            let offset = transaction_index * 65536;
            for quarter in 0..4 {
                let offset = offset + 16384 * quarter;
                for hash_pool_index in 0..4 {
                    let offset = offset + hash_pool_index * 4096;

                    let dummy_1 = FieldElement::NEGATIVE_ONE / FieldElement::from(2);
                    let dummy_2 = FieldElement::NEGATIVE_ONE / FieldElement::from(2);
                    let dummy_3 = FieldElement::from(1 << 30); // This is the amount in the vault

                    let (sources, xs, ys, slopes) = get_pedersen_hash_columns(&dummy_1, &dummy_2);
                    for (i, (source, x, y, slope)) in izip!(&sources, &xs, &ys, &slopes).enumerate() {
                        trace_table[(offset + 4 * i + 0, 8)] = x.clone();
                        trace_table[(offset + 4 * i + 1, 8)] = slope.clone();
                        trace_table[(offset + 4 * i + 2, 8)] = y.clone();
                        trace_table[(offset + 4 * i + 3, 8)] = source.clone();
                    }

                    let offset = offset + 2048;
                    let (sources, xs, ys, slopes) = get_pedersen_hash_columns(&xs[512], &dummy_3);
                    for (i, (source, x, y, slope)) in izip!(&sources, &xs, &ys, &slopes).enumerate() {
                        trace_table[(offset + 4 * i + 0, 8)] = x.clone();
                        trace_table[(offset + 4 * i + 1, 8)] = slope.clone();
                        trace_table[(offset + 4 * i + 2, 8)] = y.clone();
                        trace_table[(offset + 4 * i + 3, 8)] = source.clone();
                    }
                }

                trace_table[(offset + 8196, 9)] = trace_table[(offset + 11267, 8)].clone();
                for i in 0..32 {
                    let offset = offset + 8196;
                    trace_table[(offset + 128 * i + 128, 9)] = shift_right(&trace_table[(offset + 128 * i, 9)]);
                }

                let mut digest = trace_table[(offset + 4092, 8)].clone();
                for opening_hash_index in 0..32 {
                    let offset = offset + 512 * opening_hash_index;

                    let (sources, xs, ys, slopes) = get_pedersen_hash_columns(&digest, &digest);
                    for (i, (source, x, y, slope)) in izip!(&sources, &xs, &ys, &slopes).enumerate() {
                        trace_table[(offset + i, 3)] = source.clone();
                        trace_table[(offset + i, 0)] = x.clone();
                        trace_table[(offset + i, 1)] = y.clone();
                        trace_table[(offset + i, 2)] = slope.clone();
                    }
                    digest = xs[512].clone();
                }

                let mut closing_digest = trace_table[(offset + 12284, 8)].clone();
                for closing_hash_index in 0..32 {
                    let offset = offset + 512 * closing_hash_index;

                    let (sources, xs, ys, slopes) = get_pedersen_hash_columns(&closing_digest, &closing_digest);
                    for (i, (source, x, y, slope)) in izip!(&sources, &xs, &ys, &slopes).enumerate() {
                        trace_table[(offset + i, 7)] = source.clone();
                        trace_table[(offset + i, 4)] = x.clone();
                        trace_table[(offset + i, 5)] = y.clone();
                        trace_table[(offset + i, 6)] = slope.clone();
                    }
                    closing_digest = xs[512].clone();
                }
            }
        }

        let result = check_constraints(&system, &trace_table);

        dbg!(result.clone());
        assert!(result.is_ok());
    }
}
