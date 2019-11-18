use crate::{
    inputs::{get_directions, root, Direction, Modification, SignatureParameters, Vault, Vaults},
    pedersen::hash,
    pedersen_points::{PEDERSEN_POINTS, SHIFT_POINT},
};
use zkp_elliptic_curve::Affine;
use zkp_primefield::FieldElement;
use zkp_u256::U256;

fn get_coordinates(point: &Affine) -> (FieldElement, FieldElement) {
    match point {
        Affine::Zero => panic!(),
        Affine::Point { x, y } => (x.clone(), y.clone()),
    }
}

fn get_point(x: &FieldElement, alpha: &FieldElement, beta: &FieldElement) -> Affine {
    let y = (x.pow(3) + alpha * x + beta)
        .square_root()
        .expect("x is not on curve");
    Affine::Point { x: x.clone(), y }
}

fn get_slope(p_1: &Affine, p_2: &Affine) -> FieldElement {
    let (x_1, y_1) = get_coordinates(p_1);
    let (x_2, y_2) = get_coordinates(p_2);
    (y_1 - y_2) / (x_1 - x_2)
}

fn get_tangent_slope(p: &Affine, alpha: &FieldElement) -> FieldElement {
    let (x, y) = get_coordinates(p);
    let numerator = x.pow(2) * FieldElement::from(3) + alpha;
    let denominator = y * FieldElement::from(2);
    numerator / denominator
}

fn shift_right(x: &FieldElement) -> FieldElement {
    (U256::from(x) >> 1).into()
}

fn get_quarter_vaults(modification: &Modification) -> Vec<Vault> {
    vec![
        Vault {
            key:    modification.key.clone(),
            token:  modification.token.clone(),
            amount: modification.initial_amount.clone(),
        },
        Vault {
            key:    FieldElement::ZERO,
            token:  FieldElement::ZERO,
            amount: 0,
        },
        Vault {
            key:    modification.key.clone(),
            token:  modification.token.clone(),
            amount: modification.final_amount.clone(),
        },
        Vault {
            key:    FieldElement::ZERO,
            token:  FieldElement::ZERO,
            amount: 0,
        },
    ]
}

fn modification_tetrad(modification: &Modification) -> Vec<Modification> {
    let mut noop_modification = modification.clone();
    noop_modification.initial_amount = modification.final_amount.clone();
    vec![
        modification.clone(),
        noop_modification.clone(),
        noop_modification.clone(),
        noop_modification,
    ]
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
    for i in 256..511 {
        sources.push(sources[i].clone() >> 1);
    }
    assert_eq!(sources.len(), 512);

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

    assert_eq!(hash(left, right), x_coordinates[511]);

    (
        sources.iter().map(|x| FieldElement::from(x)).collect(),
        x_coordinates,
        y_coordinates,
        slopes,
    )
}

fn verify_signatures(
    public_key: &Affine,
    parameters: &SignatureParameters,
    /* left: &FieldElement,
     * right: &FieldElement, */
) -> (Vec<FieldElement>, Vec<FieldElement>, Vec<FieldElement>) {
    //  + 16 is slope of line tangent to point being doubled
    // + 0 is x coordinate of point being doubled
    // + 32 is y coordinate of point being doubled
    // the point being doubled is the public key?

    let mut x_coordinates = Vec::with_capacity(256);
    let mut y_coordinates = Vec::with_capacity(256);
    let mut slopes = Vec::with_capacity(256);

    let mut p = public_key.clone();
    for _ in 0..256 {
        let (x, y) = get_coordinates(&p);
        x_coordinates.push(x);
        y_coordinates.push(y);
        slopes.push(get_tangent_slope(&p, &parameters.alpha));
        p = p.clone() + p;
    }
    (x_coordinates, y_coordinates, slopes)
}

fn get_merkle_tree_columns(
    vaults: &Vaults,
    index: u32,
) -> (
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
) {
    let (vault, digests) = vaults.path(index);
    let directions = get_directions(index);
    assert_eq!(root(&vault, &directions, &digests), vaults.root());

    let mut xs = vec![];
    let mut ys = vec![];
    let mut sources = vec![];
    let mut slopes = vec![];

    let mut root = vault.hash();
    for (direction, digest) in directions.iter().zip(&digests) {
        let columns = match direction {
            Direction::LEFT => get_pedersen_hash_columns(digest, &root),
            Direction::RIGHT => get_pedersen_hash_columns(&root, digest),
        };
        sources.extend(columns.0);
        xs.extend(columns.1);
        ys.extend(columns.2);
        slopes.extend(columns.3);

        root = xs[xs.len() - 1].clone();
    }
    assert_eq!(vaults.root(), xs[xs.len() - 1]);
    (sources, xs, ys, slopes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        constraints::constraints,
        inputs::{Claim, Parameters, SignatureParameters, Tree, Vaults},
        pedersen_points::SHIFT_POINT,
    };
    use itertools::izip;
    use zkp_macros_decl::field_element;
    use zkp_stark::{check_constraints, Constraints, TraceTable};

    #[test]
    fn mason() {
        get_pedersen_hash_columns(&FieldElement::ZERO, &FieldElement::ZERO);
        get_pedersen_hash_columns(&FieldElement::ONE, &FieldElement::ZERO);
        get_pedersen_hash_columns(&FieldElement::ZERO, &FieldElement::ONE);
    }

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

        let key =
            field_element!("057d5d2e5da7409db60d64ae4e79443fedfd5eb925b5e54523eaf42cc1978169");
        let token =
            field_element!("03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d");

        let update_index = 12;
        let vault = Vault {
            key:    key.clone(),
            token:  token.clone(),
            amount: 2,
        };
        vaults.update(update_index, vault);
        let final_root = vaults.root();

        let path_index = 1234123123;
        let (vault, path) = vaults.path(path_index);

        let claim = Claim {
            n_transactions:      2,
            modifications:       vec![
                Modification {
                    initial_amount: 2,
                    final_amount:   1000,
                    index:          0,
                    key:            key.clone(),
                    token:          token.clone(),
                    vault:          update_index,
                },
                Modification {
                    initial_amount: 1000,
                    final_amount:   1300,
                    index:          1,
                    key:            key.clone(),
                    token:          token.clone(),
                    vault:          update_index,
                },
            ],
            initial_vaults_root: initial_root,
            final_vaults_root:   final_root,
        };

        let constraints = constraints(&claim, &parameters);
        let trace_length = claim.n_transactions * 65536;

        let system =
            Constraints::from_expressions((trace_length, 10), vec![], constraints).unwrap();

        let mut trace_table = TraceTable::new(trace_length, 10);

        for (transaction_index, modification) in claim.modifications.iter().enumerate() {
            let offset = transaction_index * 65536;

            for (quarter, modification) in modification_tetrad(&modification).iter().enumerate() {
                let offset = offset + 16384 * quarter;
                let quarter_vaults = get_quarter_vaults(&modification);

                let public_key = get_point(
                    &modification.key,
                    &parameters.signature.alpha,
                    &parameters.signature.beta,
                );
                let (xs, ys, slopes) = verify_signatures(&public_key, &parameters.signature);
                for (i, (x, y, slope)) in izip!(&xs, &ys, &slopes).enumerate() {
                    let stride = 64;
                    trace_table[(offset + stride * i + 0, 9)] = x.clone();
                    trace_table[(offset + stride * i + 16, 9)] = slope.clone();
                    trace_table[(offset + stride * i + 32, 9)] = y.clone();
                }

                for (hash_pool_index, vault) in quarter_vaults.iter().enumerate() {
                    let offset = offset + hash_pool_index * 4096;

                    let (sources, xs, ys, slopes) =
                        get_pedersen_hash_columns(&vault.key, &vault.token);
                    for (i, (source, x, y, slope)) in izip!(&sources, &xs, &ys, &slopes).enumerate()
                    {
                        trace_table[(offset + 4 * i + 0, 8)] = x.clone();
                        trace_table[(offset + 4 * i + 1, 8)] = slope.clone();
                        trace_table[(offset + 4 * i + 2, 8)] = y.clone();
                        trace_table[(offset + 4 * i + 3, 8)] = source.clone();
                    }

                    let offset = offset + 2048;
                    let (sources, xs, ys, slopes) =
                        get_pedersen_hash_columns(&xs[511], &vault.amount.into());
                    for (i, (source, x, y, slope)) in izip!(&sources, &xs, &ys, &slopes).enumerate()
                    {
                        trace_table[(offset + 4 * i + 0, 8)] = x.clone();
                        trace_table[(offset + 4 * i + 1, 8)] = slope.clone();
                        trace_table[(offset + 4 * i + 2, 8)] = y.clone();
                        trace_table[(offset + 4 * i + 3, 8)] = source.clone();
                    }
                }

                trace_table[(offset + 8196, 9)] = trace_table[(offset + 11267, 8)].clone();
                for i in 0..32 {
                    let offset = offset + 8196;
                    trace_table[(offset + 128 * i + 128, 9)] =
                        shift_right(&trace_table[(offset + 128 * i, 9)]);
                }

                assert_eq!(trace_table[(offset + 4092, 8)], quarter_vaults[0].hash());
                let (sources, xs, ys, slopes) =
                    get_merkle_tree_columns(&vaults, modification.vault);
                assert_eq!(sources.len(), 16384);
                for (i, (x, y, source, slope)) in izip!(&xs, &ys, &sources, &slopes).enumerate() {
                    trace_table[(offset + i, 3)] = source.clone();
                    trace_table[(offset + i, 0)] = x.clone();
                    trace_table[(offset + i, 1)] = y.clone();
                    trace_table[(offset + i, 2)] = slope.clone();
                }

                vaults.update(modification.vault, quarter_vaults[2].clone());

                let (sources, xs, ys, slopes) =
                    get_merkle_tree_columns(&vaults, modification.vault);
                assert_eq!(sources.len(), 16384);
                for (i, (x, y, source, slope)) in izip!(&xs, &ys, &sources, &slopes).enumerate() {
                    trace_table[(offset + i, 7)] = source.clone();
                    trace_table[(offset + i, 4)] = x.clone();
                    trace_table[(offset + i, 5)] = y.clone();
                    trace_table[(offset + i, 6)] = slope.clone();
                }

                trace_table[(offset + 255, 6)] = modification.vault.into();
                for i in 0..31 {
                    trace_table[(offset + 255 + i * 512 + 512, 6)] =
                        shift_right(&trace_table[(offset + 255 + i * 512, 6)]);
                }
            }

            trace_table[(offset + 16376, 9)] = modification.key.clone();
            trace_table[(offset + 16360, 9)] = modification.token.clone();
            assert_eq!(trace_table[(offset + 16376, 9)], modification.key);
            assert_eq!(trace_table[(offset + 16360, 9)], modification.token);
            assert_eq!(
                trace_table[(offset + 3075, 8)],
                modification.initial_amount.into()
            );
            assert_eq!(
                trace_table[(offset + 11267, 8)],
                modification.final_amount.into()
            );
            assert_eq!(trace_table[(offset + 255, 6)], modification.vault.into());
        }

        dbg!(trace_table[(0, 9)].clone());
        dbg!(trace_table[(16, 9)].clone());
        dbg!(trace_table[(32, 9)].clone());

        let result = check_constraints(&system, &trace_table);

        dbg!(result.clone());
        assert!(result.is_ok());
    }
}
