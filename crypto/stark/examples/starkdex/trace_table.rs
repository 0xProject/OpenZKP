use crate::{
    inputs::{get_directions, root, Direction, Modification, SignatureParameters, Vault, Vaults},
    pedersen::hash,
    pedersen_points::{PEDERSEN_POINTS, SHIFT_POINT},
    periodic_columns::ECDSA_GENERATOR,
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

fn add_points(p_1: &Affine, p_2: &Affine) -> (FieldElement, FieldElement, FieldElement) {
    let slope = get_slope(&p_1, &p_2);
    let (x, y) = get_coordinates(&(p_1 + p_2));
    (x, y, slope)
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
    let empty_vault = Vault {
        key:    FieldElement::ZERO,
        token:  FieldElement::ZERO,
        amount: 0,
    };
    vec![
        if modification.initial_amount == 0 {
            empty_vault.clone()
        } else {
            Vault {
                key:    modification.key.clone(),
                token:  modification.token.clone(),
                amount: modification.initial_amount.clone(),
            }
        },
        if modification.initial_amount == 0 {
            empty_vault.clone()
        } else {
            Vault {
                key:    modification.key.clone(),
                token:  modification.token.clone(),
                amount: modification.initial_amount.clone(),
            }
        },
        if modification.final_amount == 0 {
            empty_vault.clone()
        } else {
            Vault {
                key:    modification.key.clone(),
                token:  modification.token.clone(),
                amount: modification.final_amount.clone(),
            }
        },
        if modification.final_amount == 0 {
            empty_vault.clone()
        } else {
            Vault {
                key:    modification.key.clone(),
                token:  modification.token.clone(),
                amount: modification.final_amount.clone(),
            }
        },
    ]
}

// fn translate_settlement(settlement: &Settlement) -> Vec<Vault> {}

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

fn exponentiate_key(
    u_2: &FieldElement, // from Wikipedia ECDSA name
    public_key: &Affine,
    parameters: &SignatureParameters,
) -> (
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
) {
    let mut doubling_xs = Vec::with_capacity(256);
    let mut doubling_ys = Vec::with_capacity(256);
    let mut doubling_slopes = Vec::with_capacity(256);

    let mut sources = vec![U256::from(u_2)];
    let mut result_slopes = vec![FieldElement::ZERO; 256];
    let mut result_xs = Vec::with_capacity(256);
    let mut result_ys = Vec::with_capacity(256);

    let mut doubling_key = public_key.clone();
    let mut result = parameters.shift_point.clone();
    for i in 0..256 {
        sources.push(sources[i].clone() >> 1);

        let (x, y) = get_coordinates(&doubling_key);
        doubling_xs.push(x);
        doubling_ys.push(y);
        doubling_slopes.push(get_tangent_slope(&doubling_key, &parameters.alpha));

        let (x, y) = get_coordinates(&result);
        result_xs.push(x);
        result_ys.push(y);

        if sources[i].bit(0) {
            result_slopes[i] = get_slope(&result, &doubling_key);
            result = result + doubling_key.clone();
        }
        doubling_key = doubling_key.clone() + doubling_key;
    }

    (
        doubling_xs,
        doubling_ys,
        doubling_slopes,
        sources.iter().map(|x| FieldElement::from(x)).collect(),
        result_xs,
        result_ys,
        result_slopes,
    )
}
fn exponentiate_generator(
    u_1: &FieldElement, // from Wikipedia ECDSA name
    parameters: &SignatureParameters,
) -> (
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
    Vec<FieldElement>,
) {
    let mut sources = vec![U256::from(u_1)];
    let mut slopes = vec![FieldElement::ZERO; 256];
    let mut xs = Vec::with_capacity(256);
    let mut ys = Vec::with_capacity(256);
    let mut delta_xs = Vec::with_capacity(256);

    let mut doubling_generator = ECDSA_GENERATOR;
    let mut result = Affine::ZERO - parameters.shift_point.clone();

    for i in 0..256 {
        sources.push(sources[i].clone() >> 1);

        let (x, y) = get_coordinates(&result);
        xs.push(x.clone());
        ys.push(y);

        let (doubling_x, _) = get_coordinates(&doubling_generator);
        delta_xs.push(x - doubling_x);

        if sources[i].bit(0) {
            slopes[i] = get_slope(&result, &doubling_generator);
            result = result + doubling_generator.clone();
        }

        if i < 250 {
            // ecdsa columns stop doubling after this index.
            doubling_generator = doubling_generator.clone() + doubling_generator;
        }
    }

    (
        sources.iter().map(|x| FieldElement::from(x)).collect(),
        xs,
        ys,
        slopes,
        delta_xs,
    )
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

        let key =
            field_element!("057d5d2e5da7409db60d64ae4e79443fedfd5eb925b5e54523eaf42cc1978169");
        let token =
            field_element!("03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d");
        let initial_vaults = vec![
            (0, Vault {
                key:    key.clone(),
                token:  token.clone(),
                amount: 2,
            }),
            (1, Vault {
                key:    key.clone(),
                token:  token.clone(),
                amount: 1000,
            }),
        ];

        let mut vaults = Vaults::new();
        for (index, vault) in initial_vaults.into_iter() {
            vaults.update(index, vault);
        }
        let initial_root = vaults.root();

        let modifications = vec![
            Modification {
                initial_amount: 2,
                final_amount:   1000,
                index:          0,
                key:            key.clone(),
                token:          token.clone(),
                vault:          0,
            },
            Modification {
                initial_amount: 1000,
                final_amount:   1300,
                index:          1,
                key:            key.clone(),
                token:          token.clone(),
                vault:          1,
            },
        ];

        let mut temp_vaults = vaults.clone();
        for modification in &modifications {
            // Some verification logic here?
            temp_vaults.update(modification.vault, Vault {
                key:    modification.key.clone(),
                token:  modification.token.clone(),
                amount: modification.final_amount,
            });
        }
        let final_root = temp_vaults.root();

        // let path_index = 1234123123;
        // let (vault, path) = vaults.path(path_index);

        let claim = Claim {
            n_transactions: 2,
            modifications,
            initial_vaults_root: initial_root,
            final_vaults_root: final_root,
        };

        let constraints = constraints(&claim, &parameters);
        let trace_length = claim.n_transactions * 65536;

        let system =
            Constraints::from_expressions((trace_length, 10), vec![], constraints).unwrap();

        let mut trace_table = TraceTable::new(trace_length, 10);

        for (transaction_index, modification) in claim.modifications.iter().enumerate() {
            let offset = transaction_index * 65536;

            for (quarter, modification) in modification_tetrad(&modification).iter().enumerate() {
                dbg!(quarter);
                let offset = offset + 16384 * quarter;
                trace_table[(offset + 16376, 9)] = modification.key.clone();
                trace_table[(offset + 16360, 9)] = modification.token.clone();

                let quarter_vaults = get_quarter_vaults(&modification);

                let public_key = get_point(
                    &modification.key,
                    &parameters.signature.alpha,
                    &parameters.signature.beta,
                );

                if quarter % 2 == 0 {
                    let u_1 = FieldElement::GENERATOR.pow(10); // dummy value
                    let (sources, xs, ys, slopes, delta_xs) =
                        exponentiate_generator(&u_1, &parameters.signature);
                    for (i, (source, x, y, slope, delta)) in
                        izip!(&sources, &xs, &ys, &slopes, delta_xs).enumerate()
                    {
                        let stride = 128;

                        trace_table[(offset + stride * i + 20, 9)] = source.clone();
                        trace_table[(offset + stride * i + 68, 9)] = x.clone();
                        trace_table[(offset + stride * i + 36, 9)] = y.clone();
                        trace_table[(offset + stride * i + 100, 9)] = slope.clone();
                        trace_table[(offset + stride * i + 84, 9)] =
                            delta.inv().expect("Why should never be 0?");
                    }
                    trace_table[(offset + 11261, 8)] = u_1.inv().expect("z_nonzero");
                }

                let u_2 = FieldElement::GENERATOR; // dummy value.
                let (
                    doubling_xs,
                    doubling_ys,
                    doubling_slopes,
                    sources,
                    result_xs,
                    result_ys,
                    result_slopes,
                ) = exponentiate_key(&u_2, &public_key, &parameters.signature);
                assert_eq!(doubling_xs.len(), 256);
                for (
                    i,
                    (
                        doubling_x,
                        doubling_y,
                        doubling_slope,
                        source,
                        result_x,
                        result_y,
                        result_slope,
                    ),
                ) in izip!(
                    &doubling_xs,
                    &doubling_ys,
                    &doubling_slopes,
                    &sources,
                    &result_xs,
                    &result_ys,
                    &result_slopes,
                )
                .enumerate()
                {
                    let stride = 64;
                    // assert!(offset + stride * i != 16384);
                    trace_table[(offset + stride * i + 0, 9)] = doubling_x.clone();
                    trace_table[(offset + stride * i + 16, 9)] = doubling_slope.clone();
                    trace_table[(offset + stride * i + 32, 9)] = doubling_y.clone();

                    trace_table[(offset + stride * i + 24, 9)] = source.clone();
                    trace_table[(offset + stride * i + 48, 9)] = result_x.clone();
                    trace_table[(offset + stride * i + 8, 9)] = result_y.clone();
                    if i == 255 {
                        continue;
                    }
                    trace_table[(offset + stride * i + 40, 9)] = result_slope.clone();
                    trace_table[(offset + stride * i + 56, 9)] = (result_x - doubling_x)
                        .inv()
                        .expect("Why should never be 0?");
                }
                trace_table[(offset + 16336, 9)] = u_2.inv().expect("r_and_w_nonzero");

                for (hash_pool_index, vault) in quarter_vaults.iter().enumerate() {
                    let offset = offset + hash_pool_index * 4096;
                    if hash_pool_index % 2 == 0 {
                        if vault.amount == 0 {
                            trace_table[(offset + 1021, 8)] = FieldElement::ONE;
                        } else {
                            trace_table[(offset + 1021, 8)] = FieldElement::ZERO;
                            trace_table[(offset + 5117, 8)] = FieldElement::from(vault.amount)
                                .inv()
                                .expect("Amount cannot be 0");
                        }
                    }

                    let (sources, xs, ys, slopes) =
                        get_pedersen_hash_columns(&vault.key, &vault.token);
                    for (i, (source, x, y, slope)) in izip!(&sources, &xs, &ys, &slopes).enumerate()
                    {
                        trace_table[(offset + 4 * i + 0, 8)] = x.clone();
                        if !slope.is_zero() {
                            trace_table[(offset + 4 * i + 1, 8)] = slope.clone();
                        }
                        trace_table[(offset + 4 * i + 2, 8)] = y.clone();
                        trace_table[(offset + 4 * i + 3, 8)] = source.clone();
                    }
                    assert_eq!(sources.len(), 512);

                    let offset = offset + 2048;
                    let (sources, xs, ys, slopes) =
                        get_pedersen_hash_columns(&xs[511], &vault.amount.into());
                    for (i, (source, x, y, slope)) in izip!(&sources, &xs, &ys, &slopes).enumerate()
                    {
                        trace_table[(offset + 4 * i + 0, 8)] = x.clone();
                        if !slope.is_zero() {
                            // need to special case this to not overwrite trace_table[(offset +
                            // 27645, 8)].
                            trace_table[(offset + 4 * i + 1, 8)] = slope.clone();
                        }
                        trace_table[(offset + 4 * i + 2, 8)] = y.clone();
                        trace_table[(offset + 4 * i + 3, 8)] = source.clone();
                    }
                    assert_eq!(sources.len(), 512);
                    assert_eq!(trace_table[(offset + 3075 - 2048, 8)], vault.amount.into());
                }

                if quarter % 2 == 0 {
                    trace_table[(offset + 27645, 8)] = modification.key.pow(2);

                    let p_1 = Affine::Point {
                        x: trace_table[(offset + 32708, 9)].clone(),
                        y: trace_table[(offset + 32676, 9)].clone(),
                    };
                    let p_2 = Affine::Point {
                        x: trace_table[(offset + 16368, 9)].clone(),
                        y: trace_table[(offset + 16328, 9)].clone(),
                    };
                    let (x, y, slope) = add_points(&p_1, &p_2);
                    trace_table[(offset + 32724, 9)] = slope;
                    dbg!(trace_table[(offset + 16384, 9)].clone());
                    dbg!(x.clone());
                    trace_table[(offset + 16384, 9)] = -&x; // this is clashing, which means you need to have done something correctly here.
                                                            // This should be r.
                    trace_table[(offset + 16416, 9)] = y; // this won't fix the
                                                          // constraint until
                                                          // you get the r/x to
                                                          // line up above.

                    let mystery_point = Affine::Point {
                        // will this hack still work for settlements?
                        // this should be the hash of something....
                        x: trace_table[(offset + 32752 - 16384, 9)].clone(),
                        y: trace_table[(offset + 32712 - 16384, 9)].clone(),
                    }; // somehow the final one is being writtern in to this one.
                    trace_table[(3069 + offset, 8)] =
                        get_slope(&(Affine::ZERO - SHIFT_POINT), &mystery_point);
                    // need to subtract out the shift point, which exists so
                    // that we don't have intermediate
                    // values that are Affine::Zero.
                    // the resulting x value is the hash, which is fed into the

                    // trace_table[(3069 + offset, 8)] =
                    // dbg!(trace_table[(32752, 9)].clone());
                    // dbg!(trace_table[(32712, 9)].clone());
                    // dbg!(trace_table[(3069, 8)].clone());
                }

                trace_table[(offset + 8196, 9)] = trace_table[(offset + 11267, 8)].clone();
                for i in 0..32 {
                    let offset = offset + 8196;
                    trace_table[(offset + 128 * i + 128, 9)] =
                        shift_right(&trace_table[(offset + 128 * i, 9)]);
                }
                assert_eq!(trace_table[(0, 9)].pow(2), trace_table[(27645, 8)]);

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
                dbg!(trace_table[(16384, 9)].clone());
            }

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
        assert_eq!(
            trace_table[(claim.n_transactions * 16384 * 4 - 1, 4)],
            claim.final_vaults_root
        );

        let result = check_constraints(&system, &trace_table);

        dbg!(result.clone());
        assert!(result.is_ok());
    }
}
