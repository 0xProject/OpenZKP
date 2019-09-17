use crate::{
    polynomial::DensePolynomial, rational_expression::RationalExpression, trace_table::TraceTable,
};
use log::info;
use macros_decl::field_element;
use primefield::FieldElement;
use std::{ops::Neg, prelude::v1::*};
use tiny_keccak::Keccak;
use u256::U256;

/// Evaluation graph for algebraic expressions over a coset.
#[derive(Clone, PartialEq)]
pub struct AlgebraicGraph {
    /// The cofactor of the evaluation domain.
    cofactor: FieldElement,

    /// The size of the evaluation domain.
    coset_size: usize,

    /// Seed value for random evaluation.
    seed: FieldElement,

    /// Evaluation nodes in causal order.
    nodes: Vec<Node>,
}

/// Node in the evaluation graph.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    /// The operation represented by the node
    op: Operation,

    /// Node evaluated on a random value.
    ///
    /// It acts as an 'algebraic' hash allowing
    /// us to identify algebraically equivalent nodes.
    hash: FieldElement,

    /// Period after which node values repeat
    period: usize,

    /// Current value
    // TODO: Remove
    value: FieldElement,
}

/// Algebraic operations supported by the graph.
#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Constant(FieldElement),
    Coset(FieldElement, usize),
    Trace(usize, isize),
    Add(Index, Index),
    Neg(Index),
    Mul(Index, Index),
    Inv(Index),
    Exp(Index, usize),
    Poly(DensePolynomial, Index),
    Lookup(Table),
}

/// Reference to a node in the graph.
#[derive(Clone, Copy, PartialEq)]
pub struct Index(usize);

#[derive(Clone, PartialEq)]
pub struct Table(Vec<FieldElement>);

use Operation::*;

impl std::fmt::Debug for Index {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{:>3}", self.0)
    }
}

impl std::fmt::Debug for Table {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Table(len = {:>3})", self.0.len())
    }
}

impl std::fmt::Debug for AlgebraicGraph {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "AlgebraicGraph:")?;
        for (i, n) in self.nodes.iter().enumerate() {
            writeln!(
                fmt,
                "{:?}: {:016x} {:>8} {:?}",
                Index(i),
                n.hash.as_montgomery().c0,
                n.period,
                n.op
            )?
        }
        Ok(())
    }
}

impl std::ops::Index<Index> for AlgebraicGraph {
    type Output = Node;

    fn index(&self, index: Index) -> &Self::Output {
        &self.nodes[index.0]
    }
}

impl AlgebraicGraph {
    pub fn new(cofactor: &FieldElement, coset_size: usize) -> Self {
        // Create seed out of parameters
        let mut seed = [0; 32];
        let mut keccak = Keccak::new_keccak256();
        keccak.update(&cofactor.as_montgomery().to_bytes_be());
        keccak.update(&coset_size.to_be_bytes());
        keccak.finalize(&mut seed);
        Self {
            cofactor: cofactor.clone(),
            coset_size,
            seed: FieldElement::from_montgomery(U256::from_bytes_be(&seed)),
            nodes: vec![],
        }
    }

    /// A random evaluation of the node
    ///
    /// The node is evaluated on a random set up inputs derived from the seed.
    /// If two nodes have the same random evaluation, it can be safely assumed
    /// that they are algebraically identical.
    fn hash(&self, operation: &Operation) -> FieldElement {
        // TODO: Validate indices
        match operation {
            Constant(value) => value.clone(),
            Trace(i, o) => {
                // Value = hash(seed, i, o)
                let mut result = [0; 32];
                let mut keccak = Keccak::new_keccak256();
                keccak.update(&self.seed.as_montgomery().to_bytes_be());
                keccak.update(&i.to_be_bytes());
                keccak.update(&o.to_be_bytes());
                keccak.finalize(&mut result);
                let result = FieldElement::from_montgomery(U256::from_bytes_be(&result));
                result
            }
            Add(a, b) => &self[*a].hash + &self[*b].hash,
            Neg(a) => -&self[*a].hash,
            Mul(a, b) => &self[*a].hash * &self[*b].hash,
            Inv(a) => {
                self[*a]
                    .hash
                    .inv()
                    .expect("Division by zero while evaluating RationalExpression.")
            }
            Exp(a, i) => self[*a].hash.pow(*i),
            Poly(p, a) => p.evaluate(&self[*a].hash),
            Coset(c, s) => {
                // Pretend that seed is a member of the evaluation domain and
                // convert it to the coset.
                assert_eq!(self.coset_size % s, 0);
                let exponent = self.coset_size / s;
                let mut t = self.seed.clone();
                t /= &self.cofactor;
                let mut t = t.pow(exponent);
                t *= c;
                t
            }
            // This would need to be the same as the replaced operation
            Lookup(_) => panic!("hash(Lookup) not implemented."),
        }
    }

    fn period(&self, operation: &Operation) -> usize {
        fn lcm(a: usize, b: usize) -> usize {
            // TODO: Compute it for real. For powers of two this works.
            std::cmp::max(a, b)
        }
        match operation {
            Coset(_, s) => *s,
            Constant(_) => 1,
            Trace(..) => self.coset_size,
            Add(a, b) => lcm(self[*a].period, self[*b].period),
            Neg(a) => self[*a].period,
            Mul(a, b) => lcm(self[*a].period, self[*b].period),
            Inv(a) => self[*a].period,
            Exp(a, e) => self[*a].period,
            Poly(_, a) => self[*a].period,
            Lookup(v) => v.0.len(),
        }
    }

    /// Insert the operation and return it's node index
    ///
    /// If an algebraically identical node already exits, that index will be
    /// returned instead.
    pub fn op(&mut self, operation: Operation) -> Index {
        let hash = self.hash(&operation);
        if let Some(index) = self.nodes.iter().position(|n| n.hash == hash) {
            // Return existing node index
            Index(index)
        } else {
            // Create new node
            let index = self.nodes.len();
            let period = self.period(&operation);
            self.nodes.push(Node {
                op: operation,
                hash,
                period,
                value: FieldElement::ZERO,
            });
            Index(index)
        }
    }

    /// Adds a rational expression to the graph and return the result node
    /// index.
    pub fn expression(&mut self, expr: RationalExpression) -> Index {
        use RationalExpression as RE;
        match expr {
            RE::X => self.op(Coset(self.cofactor.clone(), self.coset_size)),
            RE::Constant(a) => self.op(Constant(a)),
            RE::Trace(i, j) => self.op(Trace(i, j)),
            RE::Add(a, b) => {
                let a = self.expression(*a);
                let b = self.expression(*b);
                self.op(Add(a, b))
            }
            RE::Neg(a) => {
                let a = self.expression(*a);
                self.op(Neg(a))
            }
            RE::Mul(a, b) => {
                let a = self.expression(*a);
                let b = self.expression(*b);
                self.op(Mul(a, b))
            }
            RE::Inv(a) => {
                let a = self.expression(*a);
                self.op(Inv(a))
            }
            RE::Exp(a, e) => {
                let a = self.expression(*a);
                self.op(Exp(a, e))
            }
            RE::Poly(p, a) => {
                let a = self.expression(*a);
                self.op(Poly(p, a))
            }
            RE::Lookup(a, _) => self.expression(*a),
        }
    }

    pub fn optimize(&mut self) {
        for i in 0..self.nodes.len() {
            let op = match &self.nodes[i].op {
                // TODO: We can also do the constant propagation here. We can even
                // fold constant propagation in with lookup tables, as it is
                // equivalent to a repeating pattern of size one.
                Mul(a, b) => {
                    match (&self[*a].op, &self[*b].op) {
                        (Constant(a), Coset(c, s)) => Coset(a * c, *s),
                        (Coset(c, s), Constant(a)) => Coset(a * c, *s),
                        (Coset(c1, s1), Coset(c2, s2)) if s1 == s2 => Coset(c1 * c2, *s1 / 2),
                        _ => Mul(*a, *b),
                    }
                }
                Exp(a, e) => {
                    match &self[*a].op {
                        Coset(b, o) if o % *e == 0 => Coset(b.pow(*e), o / *e),
                        _ => Exp(*a, *e),
                    }
                }
                // TODO: Neg(a), Inv(a) also preserve some of the coset nature,
                // but change the ordering in a way that Coset currently can not
                // represent. We could re-introduce Geometric for this.
                n => n.clone(),
            };
            self.nodes[i].period = self.period(&op);
            self.nodes[i].op = op;
        }
    }

    fn make_lookup(&self, index: Index) -> Vec<FieldElement> {
        let node = &self[index];
        assert!(node.period <= 1024);
        let mut result = Vec::with_capacity(node.period);
        let mut subdag = self.clone();
        let index = subdag.tree_shake(index);
        let fake_table = TraceTable::new(0, 0);
        for i in 0..node.period {
            result.push(subdag.eval(&fake_table, (1, i), &FieldElement::ZERO));
        }
        result
    }

    pub fn lookup_tables(&mut self) {
        const TRESHOLD: usize = 1024;
        // TODO: Don't create a bunch of lookup tables just to throw them away
        // later.
        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];
            if node.period > TRESHOLD {
                continue;
            }
            if let Constant(_) = node.op {
                continue;
            }
            if let Coset(..) = node.op {
                continue;
            }
            let table = self.make_lookup(Index(i));
            self.nodes[i].op = Lookup(Table(table));
        }
    }

    /// Remove unnecessary nodes
    pub fn tree_shake(&mut self, tip: Index) -> Index {
        // Find all used nodes
        let mut used = vec![false; self.nodes.len()];
        fn recurse(nodes: &[Node], used: &mut [bool], i: usize) {
            used[i] = true;
            match &nodes[i].op {
                Add(a, b) => {
                    recurse(nodes, used, a.0);
                    recurse(nodes, used, b.0);
                }
                Neg(a) => recurse(nodes, used, a.0),
                Mul(a, b) => {
                    recurse(nodes, used, a.0);
                    recurse(nodes, used, b.0);
                }
                Inv(a) => recurse(nodes, used, a.0),
                Exp(a, e) => recurse(nodes, used, a.0),
                Poly(p, a) => recurse(nodes, used, a.0),
                _ => {}
            }
        }
        recurse(&self.nodes, &mut used, tip.0);

        // Renumber indices
        let mut numbers = vec![Index(0); self.nodes.len()];
        let mut counter = 0;
        for i in 0..self.nodes.len() {
            if used[i] {
                numbers[i] = Index(counter);
                counter += 1;
            }
        }
        for node in self.nodes.iter_mut() {
            match &mut node.op {
                Add(a, b) => {
                    *a = numbers[a.0];
                    *b = numbers[b.0];
                }
                Neg(a) => *a = numbers[a.0],
                Mul(a, b) => {
                    *a = numbers[a.0];
                    *b = numbers[b.0];
                }
                Inv(a) => *a = numbers[a.0],
                Exp(a, e) => *a = numbers[a.0],
                Poly(p, a) => *a = numbers[a.0],
                _ => {}
            }
        }
        let mut i = 0;
        self.nodes.retain(|_| {
            i += 1;
            used[i - 1]
        });

        numbers[tip.0]
    }

    // TODO: Batch invert: combine all space-like Inv nodes to a batch inversion
    // scheme.

    // TODO: next(&self, &TraceTable) -> (i, FieldElement)
    #[inline(never)]
    pub fn eval(
        &mut self,
        trace_table: &TraceTable,
        row: (usize, usize),
        x: &FieldElement,
    ) -> FieldElement {
        for i in 0..self.nodes.len() {
            let value = match &self.nodes[i].op {
                Constant(a) => a.clone(),
                Trace(i, j) => {
                    let n = trace_table.num_rows() as isize;
                    let row = ((n + (row.1 as isize) + (row.0 as isize) * j) % n) as usize;
                    trace_table[(row, *i)].clone()
                }
                Add(a, b) => &self[*a].value + &self[*b].value,
                Neg(a) => -&self[*a].value,
                Mul(a, b) => &self[*a].value * &self[*b].value,
                Inv(a) => self[*a].value.inv().expect("Division by zero"),
                Exp(a, e) => self[*a].value.pow(*e),
                Poly(p, a) => p.evaluate(&self[*a].value),
                Coset(c, s) => {
                    // TODO: We assume sequential rows here starting at 0
                    if row.1 == 0 {
                        c.clone()
                    } else {
                        if *s == 2 {
                            -&self[Index(i)].value
                        } else {
                            // TODO: Cache root
                            FieldElement::root(*s).unwrap() * &self[Index(i)].value
                        }
                    }
                }
                Lookup(v) => v.0[row.1 % v.0.len()].clone(),
            };
            self.nodes[i].value = value;
        }
        self.nodes.last().unwrap().value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use RationalExpression as RE;

    // #[test]
    // fn test_expr() {
    //     let expr = RE::Constant(5.into()) + RE::X.pow(5);
    //     let mut dag = AlgebraicGraph::from_expression(expr.clone());
    //     let trace_table = TraceTable::new(0, 0);
    //     let x =
    // field_element!("
    // 022550177068302c52659dbd983cf622984f1f2a7fb2277003a64c7ecf96edaf");

    //     let y1 = dag.eval(&trace_table, (0, 0), &x);
    //     let y2 = expr.eval(&trace_table, (0, 0), &x);
    //     assert_eq!(y1, y2);
    // }

    // #[test]
    // fn test_poly() {
    //     let p = DensePolynomial::from_vec(vec![1.into(), 2.into(), 5.into(),
    // 7.into()]);     let expr = RE::Poly(p, Box::new(RE::X.pow(5)));
    //     let mut dag = AlgebraicGraph::from_expression(expr.clone());
    //     let trace_table = TraceTable::new(0, 0);
    //     let x =
    // field_element!("
    // 022550177068302c52659dbd983cf622984f1f2a7fb2277003a64c7ecf96edaf");

    //     let y1 = dag.eval(&trace_table, (0, 0), &x);
    //     let y2 = expr.eval(&trace_table, (0, 0), &x);
    //     assert_eq!(y1, y2);
    // }
}
