use crate::{
    polynomial::DensePolynomial, rational_expression::RationalExpression, trace_table::TraceTable,
};
use log::info;
use std::{cmp::min, ops::Neg, prelude::v1::*};
use tiny_keccak::{Hasher, Keccak};
use zkp_macros_decl::field_element;
use zkp_primefield::{invert_batch_src_dst, FieldElement, Inv, One, Pow, Root, Zero};
use zkp_u256::U256;

/// Number of values to calculate at once.
///
/// A larger value means larger chunks for batch inversion and fewer iterations
/// of the dag. Larger values also mean less cache locality.
const CHUNK_SIZE: usize = 16;
// HACK: FieldElement does not implement Copy, so we need to explicitly
// instantiate
const CHUNK_INIT: [FieldElement; CHUNK_SIZE] = [
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
    field_element!("00"),
];

/// Maximum size of a periodic lookup table.
///
/// Sub-expressions that are discovered to be periodic get evaluated into a
/// lookup table when the period is equal to or less than this value.
const LOOKUP_SIZE: usize = 1024;

/// Evaluation graph for algebraic expressions over a coset.
#[derive(Clone, PartialEq)]
pub(crate) struct AlgebraicGraph {
    /// The cofactor of the evaluation domain.
    cofactor: FieldElement,

    /// The size of the evaluation domain.
    coset_size: usize,

    /// The blowup of the trace table
    trace_blowup: usize,

    /// Seed value for random evaluation.
    seed: FieldElement,

    /// Evaluation nodes in causal order.
    nodes: Vec<Node>,

    /// Current row
    row: usize,
}

/// Node in the evaluation graph.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Node {
    /// The operation represented by the node
    op: Operation,

    /// Node evaluated on a random value.
    ///
    /// It acts as an 'algebraic' hash allowing
    /// us to identify algebraically equivalent nodes.
    hash: FieldElement,

    /// Period after which node values repeat
    period: usize,

    /// Scratch space for the evaluators
    // TODO: Something cleaner
    note:   FieldElement,
    values: [FieldElement; CHUNK_SIZE],
}

/// Algebraic operations supported by the graph.
#[derive(Clone, Debug, PartialEq)]
enum Operation {
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
pub(crate) struct Index(usize);

#[derive(Clone, PartialEq)]
struct Table(Vec<FieldElement>);

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
                n.hash.as_montgomery().limb(0),
                n.period,
                n.op
            )?
        }
        Ok(())
    }
}

// TODO: This leaks the Node type outside of this module.
impl std::ops::Index<Index> for AlgebraicGraph {
    type Output = Node;

    fn index(&self, index: Index) -> &Self::Output {
        &self.nodes[index.0]
    }
}

fn from_entropy(keccak: Keccak) -> FieldElement {
    let mut result = [0; 32];
    keccak.finalize(&mut result);
    result[0] &= 0xF;
    let mut u256 = U256::from_bytes_be(&result);
    if u256 >= FieldElement::MODULUS {
        u256 -= FieldElement::MODULUS;
    }
    assert!(u256 < FieldElement::MODULUS);
    FieldElement::from_montgomery(u256)
}

impl AlgebraicGraph {
    pub(crate) fn new(cofactor: &FieldElement, coset_size: usize, trace_blowup: usize) -> Self {
        assert!(coset_size.is_power_of_two());
        // Create seed out of parameters
        let mut keccak = Keccak::v256();
        keccak.update(&cofactor.as_montgomery().to_bytes_be());
        keccak.update(&coset_size.to_be_bytes());
        Self {
            cofactor: cofactor.clone(),
            coset_size,
            trace_blowup,
            seed: from_entropy(keccak),
            nodes: vec![],
            row: 0,
        }
    }

    /// A random evaluation of the node
    ///
    /// The node is evaluated on a random set up inputs derived from the seed.
    /// If two nodes have the same random evaluation, it can be safely assumed
    /// that they are algebraically identical.
    fn hash(&self, operation: &Operation) -> FieldElement {
        use Operation::*;
        match operation {
            Constant(a) => a.clone(),
            Trace(i, o) => {
                // Value = hash(seed, i, o)
                let mut keccak = Keccak::v256();
                keccak.update(&self.seed.as_montgomery().to_bytes_be());
                keccak.update(&i.to_be_bytes());
                keccak.update(&o.to_be_bytes());
                from_entropy(keccak)
            }
            Add(a, b) => &self[*a].hash + &self[*b].hash,
            Neg(a) => -&self[*a].hash,
            Mul(a, b) => &self[*a].hash * &self[*b].hash,
            Inv(a) => self[*a].hash.inv().expect("Division by zero"),
            Exp(a, i) => self[*a].hash.pow(*i),
            Poly(p, a) => p.evaluate(&self[*a].hash),
            // We pretend that X comes from Coset(self.cofactor, self.coset_size)
            // and do the algebraic manipulations to transform it into the
            // target coset.
            Coset(c, s) => {
                assert_eq!(self.coset_size % s, 0);
                c * (&self.seed / &self.cofactor).pow(self.coset_size / s)
            }
            // This would need to be the same as the replaced operation
            Lookup(_) => panic!("hash(Lookup) not implemented."),
        }
    }

    // Note that the hash check already covers cases where the result is
    // zero, one or a subexpression. So we don't need to match for `a - a = 0`,
    // `0 * a = 0`, `a^1 = a`, `-(-a) = a` etc. What remains is mostly
    // constant and coset propagation.
    // NOTE: for evaluation purposes Coset(c, 1) == Constant(c), but this
    // is not covered by the hash, and can not be easily incorprated while
    // maintaining the above identities.
    fn simplify(&self, operation: Operation) -> Operation {
        use Operation::*;
        match operation {
            Add(a, b) => {
                match (&self[a].op, &self[b].op) {
                    // `0 + a = a` is covered by the hash check
                    (Constant(a), Constant(b)) => Constant(a + b),
                    (Coset(c1, s1), Coset(c2, s2)) if s1 == s2 => Coset(c1 + c2, *s1),
                    _ => Add(a, b),
                }
            }
            Neg(a) => {
                match &self[a].op {
                    Constant(a) => Constant(a.neg()),
                    Coset(b, o) => Coset(b.neg(), *o),
                    _ => Neg(a),
                }
            }
            Mul(a, b) => {
                match (&self[a].op, &self[b].op) {
                    (Constant(a), Constant(b)) => Constant(a * b),
                    (Coset(a, 1), Coset(b, s))
                    | (Coset(b, s), Coset(a, 1))
                    | (Constant(a), Coset(b, s))
                    | (Coset(b, s), Constant(a)) => Coset(a * b, *s),
                    (Coset(c1, s1), Coset(c2, s2)) if s1 == s2 => Coset(c1 * c2, *s1 / 2),
                    _ => Mul(a, b),
                }
            }
            Exp(a, e) => {
                match &self[a].op {
                    Constant(a) => Constant(a.pow(e)),
                    Coset(b, o) if o % e == 0 => Coset(b.pow(e), o / e),
                    // TODO: Complex situations that break hash
                    //  * Coset(a, 1)^6 = Coset(a^6, 1)
                    //  * Coset(a, 2)^6 = Coset(a^6, 1)
                    //  * Coset(a, 4)^6 = Coset(a^2, 2)^3
                    _ => Exp(a, e),
                }
            }
            Inv(a) => {
                match &self[a].op {
                    Constant(a) => Constant(a.inv().expect("Division by zero")),
                    Coset(a, 1) => Coset(a.inv().expect("Division by zero"), 1),
                    Coset(a, 2) => Coset(a.inv().expect("Division by zero"), 2),
                    // TODO: For larger sizes we need a way to represent a
                    // reverse order Coset.
                    _ => Inv(a),
                }
            }
            Poly(p, a) => {
                match &self[a].op {
                    Constant(a) => Constant(p.evaluate(a)),
                    Coset(a, 1) => Coset(p.evaluate(a), 1),
                    _ => Poly(p, a),
                }
            }
            n => n,
        }
    }

    fn period(&self, operation: &Operation) -> usize {
        use Operation::*;
        fn lcm(a: usize, b: usize) -> usize {
            // TODO: Compute it for real. For powers of two this works.
            std::cmp::max(a, b)
        }
        match operation {
            Constant(_) => 1,
            Coset(_, s) => *s,
            Trace(..) => self.coset_size,
            Add(a, b) | Mul(a, b) => lcm(self[*a].period, self[*b].period),
            Neg(a) | Inv(a) | Exp(a, _) | Poly(_, a) => self[*a].period,
            Lookup(v) => v.0.len(),
        }
    }

    /// Insert the operation and return it's node index
    ///
    /// If an algebraically identical node already exits, that index will be
    /// returned instead.
    fn op(&mut self, operation: Operation) -> Index {
        let hash = self.hash(&operation);
        self.nodes.iter().position(|n| n.hash == hash).map_or_else(
            || {
                // Recognize expressions evaluating to zero or one. Simplify other
                // expressions.
                // OPT: Add more constants? Maybe evaluate in two points to detect all
                // constants?
                let operation = match &hash {
                    h if h.is_zero() => Operation::Constant(FieldElement::zero()),
                    h if h.is_one() => Operation::Constant(FieldElement::one()),
                    _ => self.simplify(operation),
                };
                // Create new node
                let index = self.nodes.len();
                let period = self.period(&operation);
                self.nodes.push(Node {
                    op: operation,
                    hash,
                    period,
                    values: CHUNK_INIT,
                    note: FieldElement::zero(),
                });
                Index(index)
            },
            |index| {
                // Return existing node index
                Index(index)
            },
        )
    }

    /// Adds a rational expression to the graph and return the result node
    /// index.
    pub(crate) fn expression(&mut self, expr: RationalExpression) -> Index {
        use Operation as Op;
        use RationalExpression as RE;
        match expr {
            RE::X => self.op(Op::Coset(self.cofactor.clone(), self.coset_size)),
            RE::Constant(a) => self.op(Op::Constant(a)),
            RE::Trace(i, j) => self.op(Op::Trace(i, j)),
            RE::Polynomial(p, a) => {
                let a = self.expression(*a);
                self.op(Op::Poly(p, a))
            }
            RE::ClaimPolynomial(..) => panic!("ClaimPolynomial not supported"),
            RE::Add(a, b) => {
                let a = self.expression(*a);
                let b = self.expression(*b);
                self.op(Op::Add(a, b))
            }
            RE::Neg(a) => {
                let a = self.expression(*a);
                self.op(Op::Neg(a))
            }
            RE::Mul(a, b) => {
                let a = self.expression(*a);
                let b = self.expression(*b);
                self.op(Op::Mul(a, b))
            }
            RE::Inv(a) => {
                let a = self.expression(*a);
                self.op(Op::Inv(a))
            }
            RE::Exp(a, e) => {
                let a = self.expression(*a);
                self.op(Op::Exp(a, e))
            }
        }
    }

    fn make_lookup(&self, index: Index) -> Vec<FieldElement> {
        let node = &self[index];
        assert!(node.period <= 1024);
        let mut result = Vec::with_capacity(node.period);
        let mut subdag = self.clone();
        let _ = subdag.tree_shake(index);
        let fake_table = TraceTable::new(0, 0);
        subdag.init(0);
        for _ in 0..node.period {
            result.push(subdag.next(&fake_table));
        }
        result
    }

    pub(crate) fn lookup_tables(&mut self) {
        use Operation::*;
        info!("Computing DAG Lookup tables");
        // OPT: Don't create a bunch of lookup tables just to throw them away
        // later. Analyze which nodes will be needed.
        let treshold = min(LOOKUP_SIZE, self.coset_size / 2);
        for i in 0..self.nodes.len() {
            let node = &self.nodes[i];
            if node.period > treshold {
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
    pub(crate) fn tree_shake(&mut self, tip: Index) -> Index {
        use Operation::*;
        fn recurse(nodes: &[Node], used: &mut [bool], i: usize) {
            used[i] = true;
            match &nodes[i].op {
                Add(a, b) | Mul(a, b) => {
                    recurse(nodes, used, a.0);
                    recurse(nodes, used, b.0);
                }
                Neg(a) | Inv(a) | Exp(a, _) | Poly(_, a) => recurse(nodes, used, a.0),
                _ => {}
            }
        }

        // Find all used nodes
        let mut used = vec![false; self.nodes.len()];
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
        for node in &mut self.nodes {
            match &mut node.op {
                Add(a, b) | Mul(a, b) => {
                    *a = numbers[a.0];
                    *b = numbers[b.0];
                }
                Neg(a) | Inv(a) | Exp(a, _) | Poly(_, a) => *a = numbers[a.0],
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

    // We want to use `for i in 0..CHUNK_SIZE` for consistency
    #[allow(clippy::needless_range_loop)]
    pub(crate) fn init(&mut self, start: usize) {
        use Operation::*;
        assert_eq!(start % CHUNK_SIZE, 0);
        self.row = start;
        for i in 0..self.nodes.len() {
            let (_previous, current) = self.nodes.split_at_mut(i);
            let Node {
                op, values, note, ..
            } = &mut current[0];
            match op {
                Constant(a) => {
                    for i in 0..CHUNK_SIZE {
                        values[i] = a.clone();
                    }
                }
                Coset(c, s) => {
                    let root = FieldElement::root(*s).unwrap();
                    let mut acc = c.clone();
                    acc *= root.pow(self.row);
                    for i in 0..CHUNK_SIZE {
                        values[i] = acc.clone();
                        acc *= &root;
                    }
                    if *s > CHUNK_SIZE {
                        *note = root.pow(CHUNK_SIZE);
                        // OPT: Avoid this step
                        // This is to compensate for the first round of *= note.
                        let inv = note.inv().unwrap();
                        for i in 0..CHUNK_SIZE {
                            values[i] *= &inv;
                        }
                    }
                }
                Lookup(v) if v.0.len() <= CHUNK_SIZE => {
                    assert_eq!(CHUNK_SIZE % v.0.len(), 0);
                    for i in 0..CHUNK_SIZE {
                        values[i] = v.0[(self.row + i) % v.0.len()].clone();
                    }
                }
                _ => {}
            };
        }
    }

    // We want to use `for i in 0..CHUNK_SIZE` for consistency
    #[allow(clippy::needless_range_loop)]
    #[inline(never)]
    pub(crate) fn next(&mut self, trace_table: &TraceTable) -> FieldElement {
        use Operation::*;
        if self.row % CHUNK_SIZE > 0 {
            let result = self.nodes.last().unwrap().values[self.row % CHUNK_SIZE].clone();
            self.row += 1;
            return result;
        }
        for i in 0..self.nodes.len() {
            let (previous, current) = self.nodes.split_at_mut(i);
            let Node {
                op, values, note, ..
            } = &mut current[0];
            match op {
                Trace(c, o) => {
                    // TODO: Handle all the casting more elegantly
                    // Sizes are small enough
                    #[allow(clippy::cast_possible_wrap)]
                    let n = trace_table.num_rows() as isize;
                    for i in 0..CHUNK_SIZE {
                        // Sizes are small enough
                        #[allow(clippy::cast_possible_wrap)]
                        let trace_blowup = self.trace_blowup as isize;
                        // Sizes are small enough
                        #[allow(clippy::cast_possible_wrap)]
                        let row = (self.row + i) as isize;
                        let row = (n + row + trace_blowup * *o) % n;
                        // Sizes are small enough
                        #[allow(clippy::cast_sign_loss)]
                        let row = row as usize;
                        values[i] = trace_table[(row, *c)].clone();
                    }
                }
                Add(a, b) => {
                    let a = &previous[a.0].values;
                    let b = &previous[b.0].values;
                    for i in 0..CHUNK_SIZE {
                        values[i] = &a[i] + &b[i]
                    }
                }
                Neg(a) => {
                    let a = &previous[a.0].values;
                    for i in 0..CHUNK_SIZE {
                        values[i] = -&a[i]
                    }
                }
                Mul(a, b) => {
                    let a = &previous[a.0].values;
                    let b = &previous[b.0].values;
                    for i in 0..CHUNK_SIZE {
                        values[i] = &a[i] * &b[i]
                    }
                }
                Inv(a) => {
                    let a = &previous[a.0].values;
                    invert_batch_src_dst(a, values);
                }
                Exp(a, e) => {
                    let a = &previous[a.0].values;
                    for i in 0..CHUNK_SIZE {
                        values[i] = a[i].pow(*e)
                    }
                }
                Poly(p, a) => {
                    let a = &previous[a.0].values;
                    for i in 0..CHUNK_SIZE {
                        values[i] = p.evaluate(&a[i])
                    }
                }
                Coset(_, s) if *s > CHUNK_SIZE => {
                    for i in 0..CHUNK_SIZE {
                        values[i] *= &*note;
                    }
                }
                Lookup(v) if v.0.len() > CHUNK_SIZE => {
                    // OPT: Bulk copy
                    for i in 0..CHUNK_SIZE {
                        values[i] = v.0[(self.row + i) % v.0.len()].clone();
                    }
                }
                _ => {}
            };
        }
        self.row += 1;
        self.nodes.last().unwrap().values[0].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zkp_macros_decl::field_element;
    use Operation as Op;
    use RationalExpression as RE;

    fn coset_hash(cofactor: FieldElement, size: usize) -> FieldElement {
        let mut dag = AlgebraicGraph::new(&FieldElement::generator(), 1024, 2);
        let index = dag.op(Op::Coset(cofactor, size));
        dag[index].hash.clone()
    }

    #[test]
    fn test_hash_coset_zero() {
        // hash(Coset(0, _)) = 0
        assert_eq!(coset_hash(FieldElement::zero(), 1), FieldElement::zero());
        assert_eq!(coset_hash(FieldElement::zero(), 2), FieldElement::zero());
        assert_eq!(coset_hash(FieldElement::zero(), 512), FieldElement::zero());
        assert_eq!(coset_hash(FieldElement::zero(), 1024), FieldElement::zero());
    }

    #[test]
    fn test_hash_coset_constant() {
        // hash(Coset(c, 1)) = c * (seed / cofactor) ^ coset_size
        fn test(c: FieldElement) {
            let mut dag = AlgebraicGraph::new(&FieldElement::generator(), 1024, 2);
            let factor = (&dag.seed / &dag.cofactor).pow(dag.coset_size);
            let index = dag.op(Op::Coset(c.clone(), 1));
            assert_eq!(dag[index].hash, c * factor);
        }
        test(FieldElement::zero());
        test(FieldElement::one());
        test(FieldElement::generator());
        test(field_element!(
            "022550177068302c52659dbd983cf622984f1f2a7fb2277003a64c7ecf96edaf"
        ));
    }

    #[test]
    fn test_hash_x_is_seed() {
        let mut dag = AlgebraicGraph::new(&FieldElement::generator(), 1024, 2);
        let index = dag.expression(RE::X);
        assert_eq!(dag[index].hash, dag.seed);
    }
}
