use crate::{
    polynomial::DensePolynomial, rational_expression::RationalExpression, trace_table::TraceTable,
};
use macros_decl::field_element;
use primefield::FieldElement;
use std::prelude::v1::*;
use tiny_keccak::Keccak;
use u256::U256;

#[derive(Clone, Debug, PartialEq)]
pub struct Index(usize);

#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    X,
    Constant(FieldElement),
    Trace(usize, isize),
    Add(Index, Index),
    Neg(Index),
    Mul(Index, Index),
    Inv(Index),
    Exp(Index, usize),
    Poly(DensePolynomial, Index),
}

use Node::*;

// The DAG is enforced by having each node only refer to prior nodes.
#[derive(Clone, Debug, PartialEq)]
pub struct AlgebraicGraph {
    nodes: Vec<Node>,

    // Random evaluation of nodes. Used to find common sub expressions.
    seed:   FieldElement,
    hashes: Vec<FieldElement>,

    // TODO: Move to evaluator
    values: Vec<FieldElement>,
}

impl AlgebraicGraph {
    pub fn new() -> Self {
        // TODO: Generate random seed
        Self {
            nodes:  vec![],
            seed:   field_element!(
                "06b44c52fda3e17432f8cb5dc9a17ee500c90caf7b426b8d2fe8572525e3e9ac"
            ),
            hashes: vec![],
            values: vec![],
        }
    }

    pub fn from_expression(expr: RationalExpression) -> Self {
        let mut result = Self::new();
        let Index(i) = result.insert_expression(expr);
        // TODO: Handle final result index not being last (can happen with CSE)
        assert_eq!(i, result.nodes.len() - 1);
        result
    }

    // Hash a node
    fn node_hash(&self, node: &Node) -> FieldElement {
        // TODO: Validate node indices
        match node {
            X => self.seed.clone(),
            Constant(value) => value.clone(),
            Trace(i, o) => {
                // Value = hash(seed, i, o)
                let mut result = [0; 32];
                let mut keccak = Keccak::new_keccak256();
                keccak.update(&self.seed.as_montgomery().to_bytes_be());
                keccak.update(&i.to_be_bytes());
                keccak.update(&o.to_be_bytes());
                keccak.finalize(&mut result);
                FieldElement::from_montgomery(U256::from_bytes_be(&result))
            }
            Add(a, b) => &self.hashes[a.0] + &self.hashes[b.0],
            Neg(a) => -&self.hashes[a.0],
            Mul(a, b) => &self.hashes[a.0] * &self.hashes[b.0],
            Inv(a) => {
                self.hashes[a.0]
                    .inv()
                    .expect("Division by zero while evaluating RationalExpression.")
            }
            Exp(a, i) => self.hashes[a.0].pow(*i),
            Poly(p, a) => p.evaluate(&self.hashes[a.0]),
        }
    }

    fn insert_node(&mut self, node: Node) -> Index {
        let hash = self.node_hash(&node);
        if let Some(index) = self.hashes.iter().position(|h| h == &hash) {
            // Return existing node index
            Index(index)
        } else {
            // Create new node
            let index = self.nodes.len();
            self.nodes.push(node);
            self.hashes.push(hash);
            self.values.push(FieldElement::ZERO);
            Index(index)
        }
    }

    fn insert_expression(&mut self, expr: RationalExpression) -> Index {
        use RationalExpression as RE;
        match expr {
            RE::X => self.insert_node(X),
            RE::Constant(a) => self.insert_node(Constant(a)),
            RE::Trace(i, j) => self.insert_node(Trace(i, j)),
            RE::Add(a, b) => {
                let a = self.insert_expression(*a);
                let b = self.insert_expression(*b);
                self.insert_node(Add(a, b))
            }
            RE::Neg(a) => {
                let a = self.insert_expression(*a);
                self.insert_node(Neg(a))
            }
            RE::Mul(a, b) => {
                let a = self.insert_expression(*a);
                let b = self.insert_expression(*b);
                self.insert_node(Mul(a, b))
            }
            RE::Inv(a) => {
                let a = self.insert_expression(*a);
                self.insert_node(Inv(a))
            }
            RE::Exp(a, e) => {
                let a = self.insert_expression(*a);
                self.insert_node(Exp(a, e))
            }
            RE::Poly(p, a) => {
                let a = self.insert_expression(*a);
                self.insert_node(Poly(p, a))
            }
            RE::Lookup(a, _) => self.insert_expression(*a),
        }
    }

    pub fn eval(
        &mut self,
        trace_table: &TraceTable,
        row: (usize, usize),
        x: &FieldElement,
    ) -> FieldElement {
        for i in 0..self.nodes.len() {
            let value = match &self.nodes[i] {
                X => x.clone(),
                Constant(a) => a.clone(),
                Trace(i, j) => {
                    let n = trace_table.num_rows() as isize;
                    let row = ((n + (row.1 as isize) + (row.0 as isize) * j) % n) as usize;
                    trace_table[(row, *i)].clone()
                }
                Add(a, b) => &self.values[a.0] + &self.values[b.0],
                Neg(a) => -&self.values[a.0],
                Mul(a, b) => &self.values[a.0] * &self.values[b.0],
                Inv(a) => self.values[a.0].inv().expect("Division by zero"),
                Exp(a, e) => self.values[a.0].pow(*e),
                Poly(p, a) => p.evaluate(&self.values[a.0]),
            };
            self.values[i] = value;
        }
        self.values.last().unwrap().clone()
    }
}
