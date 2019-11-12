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

#[derive(Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct ClaimPolynomials {
    pub is_settlement:   RationalExpression,
    pub is_modification: RationalExpression,
    pub base:            RationalExpression,
    pub key:             RationalExpression,
    pub token:           RationalExpression,
    pub initial_amount:  RationalExpression,
    pub final_amount:    RationalExpression,
    pub vault:           RationalExpression,
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

impl From<&Claim> for ClaimPolynomials {
    fn from(claim: &Claim) -> Self {
        use RationalExpression::*;

        let root = Constant(FieldElement::root(claim.n_transactions).unwrap());
        let mut is_modification = Constant(FieldElement::ONE);
        for modification in &claim.modifications {
            is_modification = is_modification * (X - root.pow(modification.index));
        }
        let is_settlement = (X.pow(claim.n_transactions) - 1.into()) / is_modification.clone();

        let modifications_and_weights = claim.modifications.iter().map(|modification| {
            (
                modification.clone(),
                (X - root.pow(modification.index)).inv(),
            )
        });

        let gcd = modifications_and_weights
            .clone()
            .fold(Constant(1.into()), |product, (_modification, weight)| {
                product * weight
            })
            .inv();

        let get_weighted_field = |getter: &dyn Fn(&Modification) -> RationalExpression| {
            gcd.clone()
                * modifications_and_weights
                    .clone()
                    .fold(Constant(0.into()), |sum, (modification, weight)| {
                        sum + getter(&modification) * weight
                    })
        };

        ClaimPolynomials {
            is_settlement,
            is_modification,
            base: get_weighted_field(&|_modification| 1.into()),
            key: get_weighted_field(&|modification| Constant(modification.key.clone())),
            token: get_weighted_field(&|modification| Constant(modification.token.clone())),
            initial_amount: get_weighted_field(&|modification| {
                Constant(modification.initial_amount.into())
            }),
            final_amount: get_weighted_field(&|modification| {
                Constant(modification.final_amount.into())
            }),
            vault: get_weighted_field(&|modification| Constant(modification.vault.into())),
        }
    }
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
    use zkp_primefield::FieldElement;
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
    fn test_claim_polynomials() {
        let claim = Claim {
            n_transactions:      4,
            modifications:       vec![
                Modification {
                    initial_amount: 0,
                    final_amount:   1000,
                    index:          0,
                    key:            field_element!(
                        "057d5d2e5da7409db60d64ae4e79443fedfd5eb925b5e54523eaf42cc1978169"
                    ),
                    token:          field_element!(
                        "03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d"
                    ),
                    vault:          1,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   1000,
                    index:          1,
                    key:            field_element!(
                        "024dca9f8032c9c8d1a2aae85b49df5dded9bb8da46d32284e339f5a9b30e820"
                    ),
                    token:          field_element!(
                        "03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d"
                    ),
                    vault:          2,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   1000,
                    index:          2,
                    key:            field_element!(
                        "03be0fef73793139380d0d5c27a33d6b1a67c29eb3bbe24e5635bc13b3439542"
                    ),
                    token:          field_element!(
                        "03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d"
                    ),
                    vault:          3,
                },
                Modification {
                    initial_amount: 0,
                    final_amount:   1000,
                    index:          3,
                    key:            field_element!(
                        "03f0f302fdf6ba1a4669ce4fc9bd2b4ba17bdc088ae32984f40c26e7006d2f9b"
                    ),
                    token:          field_element!(
                        "03e7aa5d1a9b180d6a12d451d3ae6fb95e390f722280f1ea383bb49d11828d"
                    ),
                    vault:          4,
                },
            ],
            initial_vaults_root: field_element!(
                "00156823f988424670b3a750156e77068328aa496ff883106ccc78ff85ea1dc1"
            ),
            final_vaults_root:   field_element!(
                "0181ae03ea55029827c08a70034df9861bc6c86689205155d966f28bf2cfb20a"
            ),
        };

        let claim_polynomials = ClaimPolynomials::from(&claim);
        let oods_point =
            field_element!("0342143aa4e0522de24cf42b3746e170dee7c72ad1459340483fed8524a80adb");

        assert_eq!(
            claim_polynomials
                .is_modification
                .evaluate(&oods_point, &|_, _| FieldElement::ZERO),
            field_element!("039ac85199efa890dd0f93be37fa97426d949638b5bb7e7a0e74252bbad9dcb6")
        );
        assert_eq!(
            claim_polynomials
                .is_settlement
                .evaluate(&oods_point, &|_, _| FieldElement::ZERO),
            FieldElement::ONE
        );
        assert_eq!(
            claim_polynomials
                .base
                .evaluate(&oods_point, &|_, _| FieldElement::ZERO),
            field_element!("02b6835713475a8483f0a8a2db5ff732c40f774a37ff51af3b4b8a7f2dd36c78")
        );
        assert_eq!(
            claim_polynomials
                .key
                .evaluate(&oods_point, &|_, _| FieldElement::ZERO),
            field_element!("07116091279aaca9bd5ba2cbf348d5965babde6eddf84ba27fe7075f86fe939e")
        );
        assert_eq!(
            claim_polynomials
                .token
                .evaluate(&oods_point, &|_, _| FieldElement::ZERO),
            field_element!("030953503ac2a75aa07fe5635f3eeb53deb9f239cd0821ae239dc68169083d4d")
        );
        assert_eq!(
            claim_polynomials
                .initial_amount
                .evaluate(&oods_point, &|_, _| FieldElement::ZERO),
            FieldElement::ZERO,
        );
        assert_eq!(
            claim_polynomials
                .final_amount
                .evaluate(&oods_point, &|_, _| FieldElement::ZERO),
            field_element!("00f10c234eb97f206412bc28eedd9e4ddc69f9eabd57147f9f1500cb01dfb36d")
        );
        assert_eq!(
            claim_polynomials
                .vault
                .evaluate(&oods_point, &|_, _| FieldElement::ZERO),
            field_element!("04379eb23f17a18ebd075ed4ac1a402252a7897b9b12c85fe4d424a6c5bd34b4")
        );
    }
}
