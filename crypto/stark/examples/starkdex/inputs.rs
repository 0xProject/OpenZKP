use super::pedersen::hash;
use std::{collections::BTreeMap, prelude::v1::*};
use zkp_elliptic_curve::Affine;
use zkp_primefield::FieldElement;
use zkp_stark::RationalExpression;

const VAULTS_DEPTH: usize = 31;

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Claim {
    pub n_transactions:      usize,
    pub modifications:       Vec<Modification>,
    pub initial_vaults_root: FieldElement,
    pub final_vaults_root:   FieldElement,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Witness {
    pub initial_vaults: Vaults,
    pub settlements:    Vec<Settlement>,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Parameters {
    pub signature:        SignatureParameters,
    pub hash_shift_point: Affine,
    pub n_vaults:         usize,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct SignatureParameters {
    pub shift_point: Affine,
    pub alpha:       FieldElement,
    pub beta:        FieldElement,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Settlement {
    maker: Modification,
    taker: Modification,
    index: usize,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Modification {
    pub initial_amount: u32,
    pub final_amount:   u32,
    pub index:          usize,
    pub key:            FieldElement,
    pub token:          FieldElement,
    pub vault:          u32,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Vault {
    pub key:    FieldElement,
    pub token:  FieldElement,
    pub amount: usize,
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Vaults {
    tree:   Tree,
    vaults: BTreeMap<u32, Vault>,
}

#[derive(PartialEq, Clone, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "std", derive(Debug))]
enum Direction {
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn other(&self) -> Self {
        match self {
            Self::LEFT => Self::RIGHT,
            Self::RIGHT => Self::LEFT,
        }
    }
}

// let boundary_base =
// let boundary_key = Polynomial(DensePolynom
// let boundary_token = Polynomial(DensePolynomial::new(
// let boundary_amount0 = Polynomial(DensePolynomial::n
// let boundary_amount1 = Polynomial(DensePolyn
// let boundary_vault_id = Polynomial(DensePolynomial:

pub fn get_is_settlement(claim: &Claim) -> RationalExpression {
    use RationalExpression::*;

    let root = FieldElement::root(claim.n_transactions).unwrap();
    let mut is_modification = Constant(FieldElement::ONE);
    for modification in &claim.modifications {
        is_modification = is_modification * (X - Constant(root.pow(modification.index)));
    }
    is_modification
}

pub fn get_is_modification(claim: &Claim) -> RationalExpression {
    (RationalExpression::X.pow(claim.n_transactions) - 1.into()) / get_is_settlement(claim)
}

pub fn get_boundary_base(claim: &Claim, x: &FieldElement) -> FieldElement {
    let root = FieldElement::root(claim.n_transactions).unwrap();

    let point_minus_x_values: Vec<_> = claim
        .modifications
        .iter()
        .map(|modification| x - root.pow(modification.index))
        .collect();

    let mut cumulative_products: Vec<FieldElement> = vec![];
    let mut cumulative_product = FieldElement::ONE;
    for term in &point_minus_x_values {
        cumulative_products.push(cumulative_product.clone());
        cumulative_product *= term;
    }

    let mut boundary_base = FieldElement::ZERO;
    let mut prod = FieldElement::ONE;
    for i in (0..claim.modifications.len()).rev() {
        let others_prod = &prod * &cumulative_products[i];
        boundary_base += others_prod;
        prod *= &point_minus_x_values[i];
    }
    boundary_base
}

pub fn get_boundary_base_2(claim: &Claim, x: &FieldElement) -> FieldElement {
    let root = FieldElement::root(claim.n_transactions).unwrap();

    let factors = claim
        .modifications
        .iter()
        .map(|modification| x - root.pow(modification.index));

    let product = factors
        .clone()
        .fold(FieldElement::ONE, |product, factor| &product * factor);
    let harmonic_sum = factors.fold(FieldElement::ZERO, |sum, factor| {
        &sum + factor.inv().unwrap()
    });

    product * harmonic_sum
}

pub fn get_key(claim: &Claim, x: &FieldElement) -> FieldElement {
    let root = FieldElement::root(claim.n_transactions).unwrap();

    let keys = claim
        .modifications
        .iter()
        .map(|modification| modification.key.clone());

    let factors = claim
        .modifications
        .iter()
        .map(|modification| x - root.pow(modification.index));

    let product = factors
        .clone()
        .fold(FieldElement::ONE, |product, factor| &product * factor);
    let weighted_sum = keys
        .zip(factors)
        .fold(FieldElement::ZERO, |sum, (key, factor)| &sum + key / factor);

    product * weighted_sum
}

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Tree {
    height:   usize,
    hash:     FieldElement,
    children: BTreeMap<Direction, Tree>,
}

impl Vault {
    pub fn hash(&self) -> FieldElement {
        hash(&self.key, &self.token)
    }
}

impl Default for Vault {
    fn default() -> Self {
        Self {
            key:    FieldElement::ZERO,
            token:  FieldElement::ZERO,
            amount: 0,
        }
    }
}

#[allow(dead_code)]
impl Vaults {
    pub fn new() -> Self {
        Self {
            tree:   Tree::new(VAULTS_DEPTH),
            vaults: BTreeMap::new(),
        }
    }

    pub fn root(&self) -> FieldElement {
        self.tree.hash.clone()
    }

    pub fn path(&self, index: u32) -> (Vault, Vec<FieldElement>) {
        (
            match self.vaults.get(&index) {
                None => Vault::default(),
                Some(vault) => vault.clone(),
            },
            self.tree.path(index),
        )
    }

    pub fn update(&mut self, index: u32, vault: Vault) {
        self.tree.update(index, &vault);
        *self.vaults.entry(index).or_insert_with(Vault::default) = vault;
    }
}

impl Tree {
    pub fn new(height: usize) -> Self {
        Self {
            height,
            hash: Self::empty_hash(height),
            children: BTreeMap::new(),
        }
    }

    pub fn path(&self, index: u32) -> Vec<FieldElement> {
        match self.height {
            0 => vec![],
            _ => {
                let direction = self.direction(index);
                let mut result = match self.children.get(&direction) {
                    None => self.empty_hashes(),
                    Some(subtree) => subtree.path(index),
                };
                let sibling = self.children.get(&direction.other());
                result.push(match sibling {
                    None => Self::empty_hash(self.height - 1),
                    Some(subtree) => subtree.hash.clone(),
                });
                result
            }
        }
    }

    pub fn update(&mut self, index: u32, vault: &Vault) {
        match self.height {
            0 => self.hash = vault.hash(),
            _ => {
                let height = self.height;
                self.children
                    .entry(self.direction(index))
                    .or_insert_with(|| Tree::new(height - 1))
                    .update(index, vault);

                let left_hash = match self.children.get(&Direction::LEFT) {
                    None => Self::empty_hash(height - 1),
                    Some(t) => t.hash.clone(),
                };
                let right_hash = match self.children.get(&Direction::RIGHT) {
                    None => Self::empty_hash(height - 1),
                    Some(t) => t.hash.clone(),
                };
                self.hash = hash(&left_hash, &right_hash);
            }
        };
    }

    fn direction(&self, index: u32) -> Direction {
        if index & (1 << (self.height - 1)) != 0 {
            Direction::LEFT
        } else {
            Direction::RIGHT
        }
    }

    fn empty_hash(height: usize) -> FieldElement {
        let mut result = FieldElement::ZERO;
        for _ in 0..=height {
            result = hash(&result, &result);
        }
        result
    }

    fn empty_hashes(&self) -> Vec<FieldElement> {
        (0..self.height - 1)
            .scan(FieldElement::ZERO, |digest, _| {
                *digest = hash(digest, digest);
                Some(digest.clone())
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::field_element;
    use zkp_primefield::{
        fft::{ifft_permuted, permute},
        FieldElement,
    };
    use zkp_stark::DensePolynomial;
    use zkp_u256::U256;

    fn get_directions(index: u32) -> Vec<Direction> {
        let mut index = index;
        let mut directions = vec![];
        for _ in 0..VAULTS_DEPTH {
            directions.push(if index % 2 == 0 {
                Direction::LEFT
            } else {
                Direction::RIGHT
            });
            index /= 2;
        }
        directions
    }

    fn root(leaf: &Vault, directions: &[Direction], path: &[FieldElement]) -> FieldElement {
        let mut root = leaf.hash();
        for (direction, sibling_hash) in directions.iter().zip(path) {
            root = match direction {
                Direction::LEFT => hash(&sibling_hash, &root),
                Direction::RIGHT => hash(&root, &sibling_hash),
            };
        }
        root
    }

    #[test]
    fn empty_path_correct() {
        let vaults = Vaults::new();
        let index = 1351234;
        let (vault, path) = vaults.path(index);

        assert_eq!(root(&vault, &get_directions(index), &path), vaults.root());
    }

    #[test]
    fn own_path_correct() {
        let mut vaults = Vaults::new();
        let index = 97234123;

        let vault = Vault {
            key:    FieldElement::GENERATOR,
            token:  FieldElement::NEGATIVE_ONE,
            amount: 1000,
        };
        vaults.update(index, vault);
        let (vault, path) = vaults.path(index);

        assert_eq!(root(&vault, &get_directions(index), &path), vaults.root());
    }

    #[test]
    fn other_path_correct() {
        let mut vaults = Vaults::new();

        let update_index = 2341972323;
        let vault = Vault {
            key:    FieldElement::GENERATOR,
            token:  FieldElement::NEGATIVE_ONE,
            amount: 1000,
        };
        vaults.update(update_index, vault);

        let path_index = 1234123123;
        let (vault, path) = vaults.path(path_index);

        assert_eq!(
            root(&vault, &get_directions(path_index), &path),
            vaults.root()
        );
    }

    #[test]
    fn test_is_modification() {
        let oods_point =
            field_element!("0342143aa4e0522de24cf42b3746e170dee7c72ad1459340483fed8524a80adb");
        let claim = Claim {
            n_transactions:      4,
            modifications:       vec![
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          0,
                    key:            FieldElement::ONE,
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          1,
                    key:            FieldElement::ONE,
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
            ],
            initial_vaults_root: FieldElement::ZERO,
            final_vaults_root:   FieldElement::ZERO,
        };

        let is_modification = get_is_modification(&claim);
        assert_eq!(
            is_modification.evaluate(&oods_point, &|_, _| FieldElement::ZERO),
            field_element!("02f87ef00f13bcc7631b24e2acde4b512bd8e6ab2ba4356b36bdc2357d5d44d7")
        );

        let is_settlement = get_is_settlement(&claim);
        assert_eq!(
            is_settlement.evaluate(&oods_point, &|_, _| FieldElement::ZERO),
            field_element!("021176ec9b276df960e146810ff872147f2559844b5a3e494ef179c1f9590b63")
        );
    }

    #[test]
    fn mason() {
        let mut x: Vec<FieldElement> = vec![1.into(), 2.into(), 4.into(), 8.into()];
        ifft_permuted(&mut x);
        permute(&mut x);

        let p = DensePolynomial::new(&x);
        // dbg!(p.evaluate(&(2.into())) / field_element!(
        //     "0590f95483183421066c9ecd368215a8b08c54fbfa769d7180dbcd8b23c65865"
        // ));
        assert_eq!(p.evaluate(&(2.into())), 1.into());
    }

    #[test]
    fn base_1() {
        let claim = Claim {
            n_transactions:      4,
            modifications:       vec![
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          0,
                    key:            FieldElement::ONE,
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          1,
                    key:            FieldElement::ONE,
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          2,
                    key:            FieldElement::ONE,
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          3,
                    key:            FieldElement::ONE,
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
            ],
            initial_vaults_root: FieldElement::ZERO,
            final_vaults_root:   FieldElement::ZERO,
        };
        assert_eq!(get_boundary_base(&claim, &FieldElement::ONE), 4.into());
        assert_eq!(
            get_boundary_base(&claim, &3.into()),
            get_boundary_base_2(&claim, &3.into())
        );
        assert_eq!(
            get_boundary_base(&claim, &5.into()),
            get_boundary_base_2(&claim, &5.into())
        );
        assert_eq!(
            get_boundary_base(&claim, &10.into()),
            get_boundary_base_2(&claim, &10.into())
        );
    }

    #[test]
    fn base_2() {
        let claim = Claim {
            n_transactions:      4,
            modifications:       vec![
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          0,
                    key:            FieldElement::ONE,
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          1,
                    key:            FieldElement::ONE,
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
            ],
            initial_vaults_root: FieldElement::ZERO,
            final_vaults_root:   FieldElement::ZERO,
        };
        assert_eq!(
            get_boundary_base(&claim, &2.into()),
            field_element!("01dafdc6d65d66b5accedf99bcd607383ad971a9537cdf25d59e99d90becc821")
        );
        assert_eq!(
            get_boundary_base(&claim, &3.into()),
            get_boundary_base_2(&claim, &3.into())
        );
        assert_eq!(
            get_boundary_base(&claim, &5.into()),
            get_boundary_base_2(&claim, &5.into())
        );
        assert_eq!(
            get_boundary_base(&claim, &10.into()),
            get_boundary_base_2(&claim, &10.into())
        );
    }

    #[test]
    fn base_3() {
        let claim = Claim {
            n_transactions:      4,
            modifications:       vec![
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          0,
                    key:            FieldElement::ONE,
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          3,
                    key:            FieldElement::ONE,
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
            ],
            initial_vaults_root: FieldElement::ZERO,
            final_vaults_root:   FieldElement::ZERO,
        };
        assert_eq!(
            get_boundary_base(&claim, &3.into()),
            field_element!("0625023929a2995b533120664329f8c7c5268e56ac8320da2a616626f41337e8")
        );
        assert_eq!(
            get_boundary_base(&claim, &3.into()),
            get_boundary_base_2(&claim, &3.into())
        );
        assert_eq!(
            get_boundary_base(&claim, &5.into()),
            get_boundary_base_2(&claim, &5.into())
        );
        assert_eq!(
            get_boundary_base(&claim, &10.into()),
            get_boundary_base_2(&claim, &10.into())
        );
    }

    #[test]
    fn key() {
        let claim = Claim {
            n_transactions:      4,
            modifications:       vec![
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          0,
                    key:            1.into(),
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   0,
                    index:          3,
                    key:            4.into(),
                    token:          FieldElement::ONE,
                    vault:          123412,
                },
            ],
            initial_vaults_root: FieldElement::ZERO,
            final_vaults_root:   FieldElement::ZERO,
        };
        assert_eq!(
            get_key(&claim, &3.into()),
            field_element!("0625023929a2995b533120664329f8c7c5268e56ac8320da2a616626f41337ee")
        );
    }

    // 0 -> 0 4       24    15
    // 1 -> 4     28      39    51
    // 2 -> 32    76    90     -3
    // 3 -> 108   148  87
    // 4 -> 256 244   120  24
    // 5 -> 500 364   144
    // 6 -> 864 508
    // 7 -> 1372
}
