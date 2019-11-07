use super::pedersen::hash;
use std::{collections::BTreeMap, prelude::v1::*};
use zkp_elliptic_curve::Affine;
use zkp_hash::Hash;
use zkp_primefield::FieldElement;

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

#[derive(PartialEq, Clone)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Tree {
    height: usize,
    hash:   FieldElement,
    left:   Option<Box<Tree>>,
    right:  Option<Box<Tree>>,
}

impl Vault {
    pub fn hash(&self) -> FieldElement {
        FieldElement::ZERO
    }
}

#[allow(dead_code)]
impl Vaults {
    pub fn new() -> Self {
        Self {
            tree:   Tree::new(31),
            vaults: BTreeMap::new(),
        }
    }

    pub fn path(&self, index: u32) -> Vec<FieldElement> {
        self.tree.path(index)
    }
}

#[allow(dead_code)]
impl Tree {
    pub fn new(height: usize) -> Self {
        Self {
            height,
            hash: Self::empty_hash(height),
            left: None,
            right: None,
        }
    }

    pub fn path(&self, index: u32) -> Vec<FieldElement> {
        match self.height {
            0 => vec![self.hash.clone()],
            _ => {
                let mut result = vec![self.hash.clone()];
                match self.subtree(index) {
                    None => result.extend(self.empty_hashes()),
                    Some(t) => result.extend(t.path(index)),
                };
                result
            }
        }
    }

    pub fn update(&mut self, index: u32, vault: Vault) {
        match self.height {
            0 => self.hash = vault.hash(),
            _ => {
                let height = self.height;
                let next = self.mut_subtree(index);
                match next {
                    None => {
                        let mut t = Tree::new(height - 1);
                        t.update(index, vault);
                        *next = Some(Box::new(t));
                    }
                    Some(t) => t.update(index, vault),
                };

                let left_hash = match &self.left {
                    None => Self::empty_hash(height - 1),
                    Some(t) => t.hash.clone(),
                };
                let right_hash = match &self.right {
                    None => Self::empty_hash(height - 1),
                    Some(t) => t.hash.clone(),
                };
                self.hash = hash(&left_hash, &right_hash);
            }
        };
    }

    fn mut_subtree(&mut self, index: u32) -> &mut Option<Box<Tree>> {
        if index & (1 << self.height) != 0 {
            &mut self.left
        } else {
            &mut self.right
        }
    }

    fn subtree(&self, index: u32) -> &Option<Box<Tree>> {
        if index & (1 << self.height) != 0 {
            &self.left
        } else {
            &self.right
        }
    }

    fn empty_hash(height: usize) -> FieldElement {
        let mut result = FieldElement::ZERO;
        for _ in 0..height {
            result = hash(&result, &result);
        }
        result
    }

    fn empty_hashes(&self) -> Vec<FieldElement> {
        assert!(self.left.is_none() || self.right.is_none());
        (0..self.height).map(Self::empty_hash).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::field_element;
    use zkp_primefield::FieldElement;
    use zkp_u256::U256;

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
        let x = field_element!("039ac85199efa890dd0f93be37fa97426d949638b5bb7e7a0e74252bbad9dcb6");

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
