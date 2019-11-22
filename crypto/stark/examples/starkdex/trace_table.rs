use crate::{
    inputs::{get_directions, root, Direction, Modification, SignatureParameters, Vault, Vaults},
    pedersen::hash,
    pedersen_points::{PEDERSEN_POINTS, SHIFT_POINT},
    periodic_columns::ECDSA_GENERATOR,
};
use zkp_elliptic_curve::Affine;
use zkp_elliptic_curve_crypto::sign;
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

fn is_odd(x: &FieldElement) -> bool {
    U256::from(x).is_odd()
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

fn sign_vault(private_key: &U256, vault: &Vault) -> (FieldElement, FieldElement) {
    let (r, w) = sign(&vault.hash().into(), private_key);
    (r.into(), w.into())
}

fn get_pedersen_hash_columns(
    left: &FieldElement,
    right: &FieldElement,
) -> Vec<(FieldElement, FieldElement, FieldElement, FieldElement)> {
    let mut sources = vec![left.clone()];
    for i in 0..255 {
        sources.push(shift_right(&sources[i]));
    }
    sources.push(right.clone());
    for i in 256..511 {
        sources.push(shift_right(&sources[i]));
    }
    assert_eq!(sources.len(), 512);

    let mut sum = SHIFT_POINT;
    let mut result = Vec::with_capacity(256);
    for (i, source) in sources.iter().enumerate() {
        let (x, y) = get_coordinates(&sum);
        let mut slope = FieldElement::ZERO;
        if is_odd(source) {
            let point = &PEDERSEN_POINTS[if i < 256 { i + 1 } else { i - 4 }].clone();
            slope = get_slope(&sum, &point);
            sum += point;
        }
        result.push((x, y, slope, source.clone()))
    }
    assert_eq!(hash(left, right), result[511].0);
    result
}

fn scalar_multiply(
    point: &Affine,
    scalar: &FieldElement,
    parameters: &SignatureParameters,
) -> Vec<(
    (FieldElement, FieldElement, FieldElement),
    (FieldElement, FieldElement, FieldElement, FieldElement),
)> {
    let mut point = point.clone();
    let mut scalar = scalar.clone();
    let mut sum = parameters.shift_point.clone();

    let mut result = Vec::with_capacity(256);
    for _ in 0..256 {
        let (x, y) = get_coordinates(&point);
        let doubling_values = (x, y, get_tangent_slope(&point, &parameters.alpha));

        let (x, y) = get_coordinates(&sum);
        let mut slope = FieldElement::ZERO;
        if is_odd(&scalar) {
            slope = get_slope(&sum, &point);
            sum += point.clone();
        };
        let sum_values = (x, y, slope, scalar.clone());

        result.push((doubling_values, sum_values));

        scalar = shift_right(&scalar);
        point += point.clone();
    }
    result
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
) -> Vec<(FieldElement, FieldElement, FieldElement, FieldElement)> {
    let (vault, digests) = vaults.path(index);
    let directions = get_directions(index);
    assert_eq!(root(&vault, &directions, &digests), vaults.root());

    let mut root = vault.hash();
    let mut result = Vec::with_capacity(512 * directions.len());
    for (direction, digest) in directions.iter().zip(&digests) {
        result.extend(match direction {
            Direction::LEFT => get_pedersen_hash_columns(digest, &root),
            Direction::RIGHT => get_pedersen_hash_columns(&root, digest),
        });
        root = result[result.len() - 1].0.clone();
    }
    assert_eq!(vaults.root(), result[result.len() - 1].0);
    result
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
    use zkp_elliptic_curve_crypto::private_to_public;
    use zkp_macros_decl::field_element;
    use zkp_stark::{check_constraints, Constraints, TraceTable};

    #[test]
    fn mason() {
        get_pedersen_hash_columns(&FieldElement::ZERO, &FieldElement::ZERO);
        get_pedersen_hash_columns(&FieldElement::ONE, &FieldElement::ZERO);
        get_pedersen_hash_columns(&FieldElement::ZERO, &FieldElement::ONE);
    }

    fn multiply(n: &FieldElement, p: &Affine) -> Affine {
        let values = scalar_multiply(p, n, &SignatureParameters {
            shift_point: SHIFT_POINT,
            alpha:       FieldElement::ONE,
            beta:        field_element!(
                "06f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89"
            ),
        });

        Affine::Point {
            x: (values[255].1).0.clone(),
            y: (values[255].1).1.clone(),
        }
    }

    #[test]
    fn test_sign_vault() {
        let vault = Vault {
            key:    field_element!(
                "06f21413efbe40de150e596d72f7a8c5609ad26c15c915c1f4cdfcb99cee9e89"
            ),
            token:  field_element!(
                "03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d"
            ),
            amount: 2,
        };
        let private_key = U256::from(2342);
        let public_key = private_to_public(&private_key);

        let (r, w) = sign_vault(&private_key, &vault);
        let z = vault.hash();

        let a = multiply(&z, &ECDSA_GENERATOR) - SHIFT_POINT;
        let b = multiply(&r, &public_key) - SHIFT_POINT;

        let result = multiply(&w, &(a + b)) - SHIFT_POINT;

        let claim = match result {
            Affine::Zero => panic!(),
            Affine::Point { x, y } => x,
        };

        assert_eq!(claim, r);
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

        let private_key = U256::from(12123);
        let public_key = private_to_public(&private_key);
        let (key, _) = get_coordinates(&public_key);

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

                let mut z = FieldElement::ZERO;
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

                    let key_token_hash_values = get_pedersen_hash_columns(&vault.key, &vault.token);
                    for (i, (x, y, slope, source)) in key_token_hash_values.iter().enumerate() {
                        trace_table[(offset + 4 * i + 0, 8)] = x.clone();
                        if !slope.is_zero() {
                            trace_table[(offset + 4 * i + 1, 8)] = slope.clone();
                        }
                        trace_table[(offset + 4 * i + 2, 8)] = y.clone();
                        trace_table[(offset + 4 * i + 3, 8)] = source.clone();
                    }
                    assert_eq!(key_token_hash_values.len(), 512);

                    let offset = offset + 2048;
                    let vault_hash_values = get_pedersen_hash_columns(
                        &key_token_hash_values[511].0,
                        &vault.amount.into(),
                    );
                    for (i, (x, y, slope, source)) in vault_hash_values.iter().enumerate() {
                        trace_table[(offset + 4 * i + 0, 8)] = x.clone();
                        if !slope.is_zero() {
                            // need to special case this to not overwrite trace_table[(offset +
                            // 27645, 8)].
                            trace_table[(offset + 4 * i + 1, 8)] = slope.clone();
                        }
                        trace_table[(offset + 4 * i + 2, 8)] = y.clone();
                        trace_table[(offset + 4 * i + 3, 8)] = source.clone();
                    }
                    assert_eq!(vault_hash_values.len(), 512);
                    assert_eq!(trace_table[(offset + 3075 - 2048, 8)], vault.amount.into());

                    z = vault_hash_values[511].0.clone();
                }

                if quarter % 2 == 0 {
                    let u_1 = z.clone();
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
                    trace_table[(offset + 11261, 8)] = z.inv().expect("z_nonzero");

                    trace_table[(offset + 27645, 8)] = modification.key.pow(2);
                }

                let u_2 = FieldElement::GENERATOR; // dummy value.
                                                   // on even ones, public key is fead in, on

                // on odd rounds, feed in r * G.
                for (i, (doubling_values, sum_values)) in
                    scalar_multiply(&public_key, &u_2, &parameters.signature)
                        .iter()
                        .enumerate()
                {
                    let stride = 64;

                    // assert!(offset + stride * i != 16384);
                    let (doubling_x, doubling_y, doubling_slope) = doubling_values;
                    trace_table[(offset + stride * i + 0, 9)] = doubling_x.clone();
                    trace_table[(offset + stride * i + 16, 9)] = doubling_slope.clone();
                    trace_table[(offset + stride * i + 32, 9)] = doubling_y.clone();

                    let (result_x, result_y, result_slope, source) = sum_values;
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

                if quarter % 2 == 1 {
                    let p_1 = Affine::Point {
                        x: trace_table[(offset + 32708 - 16384, 9)].clone(),
                        y: trace_table[(offset + 32676 - 16384, 9)].clone(),
                    };
                    let p_2 = Affine::Point {
                        x: trace_table[(offset + 16368 - 16384, 9)].clone(),
                        y: trace_table[(offset + 16328 - 16384, 9)].clone(),
                    };
                    let (x, y, slope) = add_points(&p_1, &p_2);
                    trace_table[(offset + 32724 - 16384, 9)] = slope;
                    // trace_table[(offset + 16384 - 16384, 9)] = -&x; // this is clashing, which
                    // means you need to have done something correctly here.
                    // This should be r.
                    // trace_table[(offset + 16416 - 16384, 9)] = y; // this won't fix the
                    // constraint until
                    // you get the r/x to
                    // line up above.

                    let mystery_point = Affine::Point {
                        // will this hack still work for settlements?
                        // this should be the hash of something....
                        x: trace_table[(offset + 32752 - 16384 - 16384, 9)].clone(),
                        y: trace_table[(offset + 32712 - 16384 - 16384, 9)].clone(),
                    }; // somehow the final one is being writtern in to this one.
                    trace_table[(3069 + offset - 16384, 8)] =
                        get_slope(&(Affine::ZERO - SHIFT_POINT), &mystery_point);
                    // need to subtract out the shift point, which exists so
                    // that we don't have intermediate
                    // values that are Affine::Zero.
                    // the resulting x value is the hash, which is fed into the

                    // Theory: (9, 24) is r * w mod n.
                    // (9, 20) is z?
                    //

                    // u_2 =
                }

                trace_table[(offset + 16336, 9)] = u_2.inv().expect("r_and_w_nonzero");

                trace_table[(offset + 8196, 9)] = trace_table[(offset + 11267, 8)].clone();
                for i in 0..32 {
                    let offset = offset + 8196;
                    trace_table[(offset + 128 * i + 128, 9)] =
                        shift_right(&trace_table[(offset + 128 * i, 9)]);
                }
                assert_eq!(trace_table[(0, 9)].pow(2), trace_table[(27645, 8)]);

                assert_eq!(trace_table[(offset + 4092, 8)], quarter_vaults[0].hash());

                let old_root_path_values = get_merkle_tree_columns(&vaults, modification.vault);
                for (i, (x, y, slope, source)) in old_root_path_values.iter().enumerate() {
                    trace_table[(offset + i, 3)] = source.clone();
                    trace_table[(offset + i, 0)] = x.clone();
                    trace_table[(offset + i, 1)] = y.clone();
                    trace_table[(offset + i, 2)] = slope.clone();
                }

                vaults.update(modification.vault, quarter_vaults[2].clone());

                let new_root_path_values = get_merkle_tree_columns(&vaults, modification.vault);
                for (i, (x, y, slope, source)) in new_root_path_values.iter().enumerate() {
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
