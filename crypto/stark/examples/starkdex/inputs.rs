use super::pedersen::hash;
use std::{collections::BTreeMap, prelude::v1::*};
use zkp_elliptic_curve::Affine;
use zkp_hash::Hash;
use zkp_primefield::FieldElement;

const VAULTS_DEPTH: usize = 31;

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Claim {
    pub n_transactions:      usize,
    pub modifications:       Vec<Modification>,
    pub initial_vaults_root: Hash,
    pub final_vaults_root:   Hash,
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
    initial_amount: u32,
    final_amount:   u32,
    index:          usize,
    key:            FieldElement,
    token:          FieldElement,
    vault:          u32,
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
    use quickcheck_macros::quickcheck;
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
    fn hash_test() {
        let x = field_element!("05d702904f10e78036abca2229077576488062ef2a9b44eb16b9e12a07932764");
        assert_eq!(
            hash(&x, &x),
            field_element!("04eb5413fc27c3950f8e9d3bd9ba325b6fcc144f6f484e10ebb6b6fda9b642a9")
        );
    }

    #[test]
    fn test_is_settlement_0() {
        let oods_point =
            field_element!("0342143aa4e0522de24cf42b3746e170dee7c72ad1459340483fed8524a80adb");
        let x = field_element!("039ac85199efa890dd0f93be37fa97426d949638b5bb7e7a0e74252bbad9dcb6");
        assert_eq!(x, oods_point.pow(4) - FieldElement::ONE);
    }

    #[test]
    fn test_is_settlement_1() {
        // get these values from using three modifications and n_transcations = 3.
        // we round n_tractions up to the next power of two.
        let oods_point =
            field_element!("0342143aa4e0522de24cf42b3746e170dee7c72ad1459340483fed8524a80adb");

        let is_settlement_oods =
            field_element!("01671673ce82eb78357e14917a70da38a40e55817dc8b41a72a153ac18bb42bd");
        let is_modification_oods =
            field_element!("03c544b737bf7258e9601c1315ee8ec9759cdf34a4862b80cf94bfa8fc531e1e");
        assert_eq!(
            is_settlement_oods * is_modification_oods,
            oods_point.pow(4) - FieldElement::ONE
        );
    }

    #[test]
    fn test_is_settlement_2() {
        let oods_point =
            field_element!("0342143aa4e0522de24cf42b3746e170dee7c72ad1459340483fed8524a80adb");
        let is_settlement_oods =
            field_element!("01671673ce82eb78357e14917a70da38a40e55817dc8b41a72a153ac18bb42bd");
        let is_modification_oods =
            field_element!("03c544b737bf7258e9601c1315ee8ec9759cdf34a4862b80cf94bfa8fc531e1e");

        assert_eq!(
            is_settlement_oods * is_modification_oods,
            oods_point.pow(4) - FieldElement::ONE
        );
    }
}
