//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)
use super::functions::*;

/// An antichain: a set of pairwise incomparable elements.
pub struct Antichain {
    pub elements: Vec<usize>,
}
impl Antichain {
    /// Create an empty antichain.
    pub fn new() -> Self {
        Antichain {
            elements: Vec::new(),
        }
    }
    /// Add an element to the antichain.
    pub fn add(&mut self, e: usize) {
        self.elements.push(e);
    }
    /// Return the number of elements in the antichain.
    pub fn len(&self) -> usize {
        self.elements.len()
    }
    /// Return true if this is an empty antichain.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
    /// Check whether this is a valid antichain in the given lattice.
    ///
    /// No two distinct elements may be comparable.
    pub fn is_antichain_in(&self, lat: &FiniteLattice) -> bool {
        for i in 0..self.elements.len() {
            for j in (i + 1)..self.elements.len() {
                let a = self.elements[i];
                let b = self.elements[j];
                if lat.le(a, b) || lat.le(b, a) {
                    return false;
                }
            }
        }
        true
    }
}
/// A Galois connection between two posets, described by its adjoint pair.
pub struct GaloisConnection {
    pub adjoint_f: String,
    pub adjoint_g: String,
}
impl GaloisConnection {
    /// Create a new Galois connection from descriptions of f and g.
    pub fn new(f: &str, g: &str) -> Self {
        GaloisConnection {
            adjoint_f: f.to_string(),
            adjoint_g: g.to_string(),
        }
    }
    /// Describe the closure operator g ∘ f induced by the Galois connection.
    pub fn compose_fg(&self) -> String {
        format!("{} ∘ {} (closure operator)", self.adjoint_g, self.adjoint_f)
    }
}
/// A chain: a totally ordered subset of a lattice.
pub struct Chain {
    pub elements: Vec<usize>,
}
impl Chain {
    /// Create an empty chain.
    pub fn new() -> Self {
        Chain {
            elements: Vec::new(),
        }
    }
    /// Add an element to the chain.
    pub fn add(&mut self, e: usize) {
        self.elements.push(e);
    }
    /// Return the number of elements in the chain.
    pub fn len(&self) -> usize {
        self.elements.len()
    }
    /// Return true if this is an empty chain.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
    /// Check whether this is a valid chain in the given lattice.
    ///
    /// Every pair of elements must be comparable.
    pub fn is_chain_in(&self, lat: &FiniteLattice) -> bool {
        for i in 0..self.elements.len() {
            for j in (i + 1)..self.elements.len() {
                let a = self.elements[i];
                let b = self.elements[j];
                if !lat.le(a, b) && !lat.le(b, a) {
                    return false;
                }
            }
        }
        true
    }
}
/// An MV-algebra over a finite carrier (Lukasiewicz n-valued logic).
#[allow(dead_code)]
pub struct MVAlgebra {
    /// Number of truth values.
    pub num_values: usize,
}
#[allow(dead_code)]
impl MVAlgebra {
    /// Create the standard Lukasiewicz n-valued MV-algebra.
    pub fn lukasiewicz(n: usize) -> Self {
        assert!(n >= 1);
        Self { num_values: n + 1 }
    }
    /// Compute x ⊕ y: min(x + y, n).
    pub fn oplus(&self, x: usize, y: usize) -> usize {
        (x + y).min(self.num_values - 1)
    }
    /// Compute ¬x: n - x.
    pub fn neg(&self, x: usize) -> usize {
        (self.num_values - 1) - x
    }
    /// Compute x ⊙ y: max(0, x + y - n).
    pub fn otimes(&self, x: usize, y: usize) -> usize {
        let n = self.num_values - 1;
        if x + y > n {
            x + y - n
        } else {
            0
        }
    }
    /// Check x ≤ y (induced lattice order).
    pub fn le(&self, x: usize, y: usize) -> bool {
        x <= y
    }
    /// Top element (truth).
    pub fn top(&self) -> usize {
        self.num_values - 1
    }
    /// Bottom element (falsity).
    pub fn bottom(&self) -> usize {
        0
    }
}
/// A closure operator on a finite set, computed by fixed-point iteration.
///
/// A closure operator satisfies:
///   1. Extensive:   x ≤ c(x)
///   2. Monotone:    x ≤ y → c(x) ≤ c(y)
///   3. Idempotent:  c(c(x)) = c(x)
pub struct ClosureOperatorFinite {
    /// The underlying lattice.
    pub lat: FiniteLattice,
    /// The closure function, stored as a table.
    pub closure_table: Vec<usize>,
}
impl ClosureOperatorFinite {
    /// Build a closure operator from a monotone function `f` by iterating
    /// f until a fixed point is reached, taking joins to ensure extensiveness.
    ///
    /// The result is the smallest closure operator above `f`.
    pub fn from_fn<F>(lat: FiniteLattice, f: F) -> Self
    where
        F: Fn(usize) -> usize,
    {
        let n = lat.size;
        let mut table: Vec<usize> = (0..n).map(|x| lat.join(x, f(x))).collect();
        loop {
            let mut changed = false;
            let next: Vec<usize> = (0..n).map(|x| lat.join(x, table[table[x]])).collect();
            for i in 0..n {
                if next[i] != table[i] {
                    changed = true;
                }
            }
            table = next;
            if !changed {
                break;
            }
        }
        ClosureOperatorFinite {
            lat,
            closure_table: table,
        }
    }
    /// Apply the closure operator to an element.
    pub fn apply(&self, x: usize) -> usize {
        self.closure_table[x]
    }
    /// Return the set of fixed points (closed elements).
    pub fn fixed_points(&self) -> Vec<usize> {
        (0..self.lat.size)
            .filter(|&x| self.closure_table[x] == x)
            .collect()
    }
    /// Check that the operator is extensive: x ≤ c(x) for all x.
    pub fn is_extensive(&self) -> bool {
        (0..self.lat.size).all(|x| self.lat.le(x, self.closure_table[x]))
    }
    /// Check that the operator is idempotent: c(c(x)) = c(x) for all x.
    pub fn is_idempotent(&self) -> bool {
        (0..self.lat.size)
            .all(|x| self.closure_table[self.closure_table[x]] == self.closure_table[x])
    }
    /// Check that the operator is monotone: x ≤ y → c(x) ≤ c(y).
    pub fn is_monotone(&self) -> bool {
        for x in 0..self.lat.size {
            for y in 0..self.lat.size {
                if self.lat.le(x, y) && !self.lat.le(self.closure_table[x], self.closure_table[y]) {
                    return false;
                }
            }
        }
        true
    }
}
/// A lattice element represented by its index in the carrier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LatticeElement(pub usize);
/// A complete lattice over a finite carrier supporting arbitrary meets and joins.
pub struct CompleteLatticeFinite {
    inner: FiniteLattice,
}
impl CompleteLatticeFinite {
    /// Create a complete lattice from a `FiniteLattice`.
    pub fn new(lat: FiniteLattice) -> Self {
        CompleteLatticeFinite { inner: lat }
    }
    /// Compute the infimum (meet) of a subset, given as a slice of element indices.
    ///
    /// Returns the greatest lower bound of all elements in `subset`.
    /// An empty subset returns the top element.
    pub fn inf(&self, subset: &[usize]) -> usize {
        if subset.is_empty() {
            return self.inner.top();
        }
        let mut result = subset[0];
        for &x in &subset[1..] {
            result = self.inner.meet(result, x);
        }
        result
    }
    /// Compute the supremum (join) of a subset, given as a slice of element indices.
    ///
    /// Returns the least upper bound of all elements in `subset`.
    /// An empty subset returns the bottom element.
    pub fn sup(&self, subset: &[usize]) -> usize {
        if subset.is_empty() {
            return self.inner.bottom();
        }
        let mut result = subset[0];
        for &x in &subset[1..] {
            result = self.inner.join(result, x);
        }
        result
    }
    /// Return the top element.
    pub fn top(&self) -> usize {
        self.inner.top()
    }
    /// Return the bottom element.
    pub fn bottom(&self) -> usize {
        self.inner.bottom()
    }
    /// Test whether element `a` is below element `b`.
    pub fn le(&self, a: usize, b: usize) -> bool {
        self.inner.le(a, b)
    }
}
/// An orthomodular lattice finite model.
#[allow(dead_code)]
pub struct OrthoModularLattice {
    /// Underlying lattice.
    pub lat: FiniteLattice,
    /// Orthocomplement table: ortho\[i\] = i⊥.
    pub ortho: Vec<usize>,
}
#[allow(dead_code)]
impl OrthoModularLattice {
    /// Build from a lattice and orthocomplement table.
    pub fn new(lat: FiniteLattice, ortho: Vec<usize>) -> Self {
        Self { lat, ortho }
    }
    /// Check involution: (a⊥)⊥ = a for all a.
    pub fn check_involution(&self) -> bool {
        (0..self.lat.size).all(|i| self.ortho[self.ortho[i]] == i)
    }
    /// Check De Morgan law: (a ∨ b)⊥ = a⊥ ∧ b⊥.
    pub fn check_de_morgan_join(&self) -> bool {
        let n = self.lat.size;
        for i in 0..n {
            for j in 0..n {
                let join = self.lat.join(i, j);
                if self.ortho[join] != self.lat.meet(self.ortho[i], self.ortho[j]) {
                    return false;
                }
            }
        }
        true
    }
    /// Check orthomodular law: a ≤ b → a ∨ (a⊥ ∧ b) = b.
    pub fn check_orthomodular_law(&self) -> bool {
        let n = self.lat.size;
        for a in 0..n {
            for b in 0..n {
                if self.lat.le(a, b) {
                    let meet = self.lat.meet(self.ortho[a], b);
                    if self.lat.join(a, meet) != b {
                        return false;
                    }
                }
            }
        }
        true
    }
}
/// A Heyting algebra on the open sets of a finite topological space.
///
/// The carrier is a collection of subsets of {0, 1, ..., n-1}, encoded as u64 bitmasks,
/// closed under finite intersection and arbitrary union.
pub struct HeytingAlgebraFiniteTop {
    /// Number of points in the base space.
    pub n: u32,
    /// The open sets, stored as sorted bitmasks.
    pub opens: Vec<u64>,
}
impl HeytingAlgebraFiniteTop {
    /// Build the discrete topology: every subset is open.
    pub fn discrete(n: u32) -> Self {
        assert!(n <= 14, "n must be at most 14 for discrete topology");
        let count = 1u64 << n;
        let opens: Vec<u64> = (0..count).collect();
        HeytingAlgebraFiniteTop { n, opens }
    }
    /// Build the indiscrete topology: only ∅ and the whole space are open.
    pub fn indiscrete(n: u32) -> Self {
        let top = if n == 64 { u64::MAX } else { (1u64 << n) - 1 };
        HeytingAlgebraFiniteTop {
            n,
            opens: vec![0, top],
        }
    }
    /// Check whether a bitmask represents an open set.
    pub fn is_open(&self, s: u64) -> bool {
        self.opens.binary_search(&s).is_ok()
    }
    /// Interior: largest open set contained in s.
    pub fn interior(&self, s: u64) -> u64 {
        self.opens
            .iter()
            .filter(|&&o| o & s == o)
            .copied()
            .fold(0u64, |acc, o| acc | o)
    }
    /// Meet of two open sets (intersection).
    pub fn meet(&self, a: u64, b: u64) -> u64 {
        a & b
    }
    /// Join of two open sets (union).
    pub fn join(&self, a: u64, b: u64) -> u64 {
        a | b
    }
    /// Heyting implication: a ⇒ b = interior(¬a ∨ b).
    pub fn implies(&self, a: u64, b: u64) -> u64 {
        let top = if self.n == 64 {
            u64::MAX
        } else {
            (1u64 << self.n) - 1
        };
        self.interior((!a & top) | b)
    }
    /// Pseudo-complement (negation): ¬a = a ⇒ ⊥ = interior(¬a).
    pub fn pseudo_complement(&self, a: u64) -> u64 {
        let top = if self.n == 64 {
            u64::MAX
        } else {
            (1u64 << self.n) - 1
        };
        self.interior(!a & top)
    }
    /// Top element (whole space).
    pub fn top(&self) -> u64 {
        if self.n == 64 {
            u64::MAX
        } else {
            (1u64 << self.n) - 1
        }
    }
    /// Bottom element (empty set).
    pub fn bottom(&self) -> u64 {
        0
    }
}
/// A finite lattice represented by its order relation matrix.
pub struct FiniteLattice {
    pub size: usize,
    /// `order\[i\]\[j\]` is true iff element i ≤ element j.
    pub order: Vec<Vec<bool>>,
}
impl FiniteLattice {
    /// Create a new finite lattice of the given size.
    ///
    /// Initially only the reflexive pairs (i ≤ i) are set.
    pub fn new(size: usize) -> Self {
        let mut order = vec![vec![false; size]; size];
        for i in 0..size {
            order[i][i] = true;
        }
        FiniteLattice { size, order }
    }
    /// Assert that element i ≤ element j.
    pub fn set_order(&mut self, i: usize, j: usize) {
        if i < self.size && j < self.size {
            self.order[i][j] = true;
        }
    }
    /// Return true iff i ≤ j.
    pub fn le(&self, i: usize, j: usize) -> bool {
        i < self.size && j < self.size && self.order[i][j]
    }
    /// Return true iff i < j (i ≤ j and i ≠ j).
    pub fn lt(&self, i: usize, j: usize) -> bool {
        i != j && self.le(i, j)
    }
    /// Compute the meet (greatest lower bound) of i and j.
    ///
    /// Returns the largest k such that k ≤ i and k ≤ j.
    pub fn meet(&self, i: usize, j: usize) -> usize {
        let mut best = 0;
        let mut found = false;
        for k in 0..self.size {
            if self.le(k, i) && self.le(k, j) && (!found || self.lt(best, k)) {
                best = k;
                found = true;
            }
        }
        best
    }
    /// Compute the join (least upper bound) of i and j.
    ///
    /// Returns the smallest k such that i ≤ k and j ≤ k.
    pub fn join(&self, i: usize, j: usize) -> usize {
        let mut best = self.size - 1;
        let mut found = false;
        for k in 0..self.size {
            if self.le(i, k) && self.le(j, k) && (!found || self.lt(k, best)) {
                best = k;
                found = true;
            }
        }
        best
    }
    /// Return the top element (greatest element in the lattice).
    pub fn top(&self) -> usize {
        let mut top = 0;
        for k in 0..self.size {
            if self.le(top, k) {
                top = k;
            }
        }
        top
    }
    /// Return the bottom element (least element in the lattice).
    pub fn bottom(&self) -> usize {
        let mut bot = 0;
        for k in 0..self.size {
            if self.le(k, bot) {
                bot = k;
            }
        }
        bot
    }
    /// Check whether the distributive law holds for all triples.
    ///
    /// x ∧ (y ∨ z) = (x ∧ y) ∨ (x ∧ z) for all x, y, z.
    pub fn is_distributive(&self) -> bool {
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    let lhs = self.meet(x, self.join(y, z));
                    let rhs = self.join(self.meet(x, y), self.meet(x, z));
                    if lhs != rhs {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Check whether this lattice is a Boolean algebra.
    ///
    /// A lattice is Boolean iff it is distributive and every element has a complement.
    pub fn is_boolean(&self) -> bool {
        if !self.is_distributive() {
            return false;
        }
        let top = self.top();
        let bot = self.bottom();
        for x in 0..self.size {
            let has_complement =
                (0..self.size).any(|c| self.join(x, c) == top && self.meet(x, c) == bot);
            if !has_complement {
                return false;
            }
        }
        true
    }
}
/// A Boolean algebra on bit vectors of a fixed width.
///
/// Implements Stone's representation: an n-element set gives 2^n subsets
/// forming a Boolean algebra, encoded as u64 bitmasks.
pub struct BitVectorBooleanAlgebra {
    pub width: u32,
}
impl BitVectorBooleanAlgebra {
    /// Create a Boolean algebra on bit vectors of `width` bits.
    pub fn new(width: u32) -> Self {
        assert!(width <= 64, "width must be at most 64");
        BitVectorBooleanAlgebra { width }
    }
    /// The top element (all bits set).
    pub fn top(&self) -> u64 {
        if self.width == 64 {
            u64::MAX
        } else {
            (1u64 << self.width) - 1
        }
    }
    /// The bottom element (no bits set).
    pub fn bottom(&self) -> u64 {
        0
    }
    /// Meet (conjunction / bitwise AND).
    pub fn meet(&self, a: u64, b: u64) -> u64 {
        a & b
    }
    /// Join (disjunction / bitwise OR).
    pub fn join(&self, a: u64, b: u64) -> u64 {
        a | b
    }
    /// Complement (negation / bitwise NOT, masked to `width` bits).
    pub fn complement(&self, a: u64) -> u64 {
        (!a) & self.top()
    }
    /// Relative pseudo-complement (implication): a → b = ¬a ∨ b.
    pub fn implies(&self, a: u64, b: u64) -> u64 {
        self.join(self.complement(a), b)
    }
    /// Check the ordering: a ≤ b iff a ∧ b = a.
    pub fn le(&self, a: u64, b: u64) -> bool {
        self.meet(a, b) == a
    }
    /// Enumerate all atoms (elements with exactly one bit set).
    pub fn atoms(&self) -> Vec<u64> {
        (0..self.width).map(|i| 1u64 << i).collect()
    }
    /// Stone representation: return the element as the set of atoms below it.
    pub fn stone_set(&self, a: u64) -> Vec<u32> {
        (0..self.width).filter(|&i| a & (1u64 << i) != 0).collect()
    }
}
/// A residuated lattice over a finite carrier.
#[allow(dead_code)]
pub struct ResidLattice {
    /// Underlying finite lattice.
    pub lat: FiniteLattice,
    /// Product table: product_table\[i\]\[j\] = i ⊗ j.
    pub product_table: Vec<Vec<usize>>,
    /// Left residual table: left_resid\[i\]\[j\] = i \ j.
    pub left_resid: Vec<Vec<usize>>,
    /// Right residual table: right_resid\[i\]\[j\] = i / j.
    pub right_resid: Vec<Vec<usize>>,
}
#[allow(dead_code)]
impl ResidLattice {
    /// Create a residuated lattice from explicit tables.
    pub fn new(
        lat: FiniteLattice,
        product_table: Vec<Vec<usize>>,
        left_resid: Vec<Vec<usize>>,
        right_resid: Vec<Vec<usize>>,
    ) -> Self {
        Self {
            lat,
            product_table,
            left_resid,
            right_resid,
        }
    }
    /// Check left adjointness: i ⊗ j ≤ k ↔ j ≤ i \ k.
    pub fn check_left_adjoint(&self) -> bool {
        let n = self.lat.size;
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let prod = self.product_table[i][j];
                    let lhs = self.lat.le(prod, k);
                    let rhs = self.lat.le(j, self.left_resid[i][k]);
                    if lhs != rhs {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Number of elements.
    pub fn size(&self) -> usize {
        self.lat.size
    }
}
/// A Priestley space (ordered Stone space) dual to a distributive lattice.
#[allow(dead_code)]
pub struct PriestleySpace {
    /// Number of points.
    pub num_points: usize,
    /// Order relation: order\[i\]\[j\] = true iff i ≤ j.
    pub order: Vec<Vec<bool>>,
    /// Clopen upsets (each as a set of point indices).
    pub clopen_upsets: Vec<Vec<usize>>,
}
#[allow(dead_code)]
impl PriestleySpace {
    /// Create from order and clopen upsets.
    pub fn new(num_points: usize, order: Vec<Vec<bool>>, clopen_upsets: Vec<Vec<usize>>) -> Self {
        Self {
            num_points,
            order,
            clopen_upsets,
        }
    }
    /// Check Priestley separation: x ≰ y ⟹ ∃ clopen upset containing x but not y.
    pub fn check_separation(&self) -> bool {
        for i in 0..self.num_points {
            for j in 0..self.num_points {
                if !self.order[i][j] {
                    let found = self
                        .clopen_upsets
                        .iter()
                        .any(|u| u.contains(&i) && !u.contains(&j));
                    if !found {
                        return false;
                    }
                }
            }
        }
        true
    }
    /// Number of clopen upsets.
    pub fn num_clopens(&self) -> usize {
        self.clopen_upsets.len()
    }
    /// Check whether a set is an upset.
    pub fn is_upset(&self, pts: &[usize]) -> bool {
        for &p in pts {
            for q in 0..self.num_points {
                if self.order[p][q] && !pts.contains(&q) {
                    return false;
                }
            }
        }
        true
    }
}
/// A Galois connection between two finite posets represented as index sets.
///
/// A Galois connection is a pair of monotone functions f : A → B and g : B → A
/// such that f(a) ≤ b  ↔  a ≤ g(b).
pub struct GaloisConnectionFinite {
    /// f : A → B (left adjoint)
    pub f: Vec<usize>,
    /// g : B → A (right adjoint)
    pub g: Vec<usize>,
    pub poset_a: FiniteLattice,
    pub poset_b: FiniteLattice,
}
impl GaloisConnectionFinite {
    /// Create a Galois connection from explicit function tables.
    pub fn new(
        poset_a: FiniteLattice,
        poset_b: FiniteLattice,
        f: Vec<usize>,
        g: Vec<usize>,
    ) -> Self {
        GaloisConnectionFinite {
            f,
            g,
            poset_a,
            poset_b,
        }
    }
    /// Check the adjointness condition: f(a) ≤ b  ↔  a ≤ g(b) for all a, b.
    pub fn is_adjoint(&self) -> bool {
        for a in 0..self.poset_a.size {
            for b in 0..self.poset_b.size {
                let fa = self.f[a];
                let gb = self.g[b];
                let lhs = self.poset_b.le(fa, b);
                let rhs = self.poset_a.le(a, gb);
                if lhs != rhs {
                    return false;
                }
            }
        }
        true
    }
    /// Compute the closure operator g ∘ f on A.
    pub fn closure(&self, a: usize) -> usize {
        self.g[self.f[a]]
    }
    /// Compute the co-closure operator f ∘ g on B.
    pub fn co_closure(&self, b: usize) -> usize {
        self.f[self.g[b]]
    }
}
/// A formal context for Formal Concept Analysis.
#[allow(dead_code)]
pub struct FormalContext {
    /// Number of objects.
    pub num_objects: usize,
    /// Number of attributes.
    pub num_attributes: usize,
    /// Incidence relation: incidence\[g\]\[m\] = true iff object g has attribute m.
    pub incidence: Vec<Vec<bool>>,
}
#[allow(dead_code)]
impl FormalContext {
    /// Create a FormalContext.
    pub fn new(num_objects: usize, num_attributes: usize, incidence: Vec<Vec<bool>>) -> Self {
        Self {
            num_objects,
            num_attributes,
            incidence,
        }
    }
    /// Object derivation A' = {m : ∀g ∈ A, gIm}.
    pub fn object_derivation(&self, objects: &[usize]) -> Vec<usize> {
        (0..self.num_attributes)
            .filter(|&m| objects.iter().all(|&g| self.incidence[g][m]))
            .collect()
    }
    /// Attribute derivation B' = {g : ∀m ∈ B, gIm}.
    pub fn attribute_derivation(&self, attributes: &[usize]) -> Vec<usize> {
        (0..self.num_objects)
            .filter(|&g| attributes.iter().all(|&m| self.incidence[g][m]))
            .collect()
    }
    /// Check if (A, B) is a formal concept.
    pub fn is_concept(&self, objects: &[usize], attributes: &[usize]) -> bool {
        let b_from_a = self.object_derivation(objects);
        let a_from_b = self.attribute_derivation(attributes);
        let mut sa: Vec<usize> = objects.to_vec();
        sa.sort_unstable();
        let mut sb: Vec<usize> = attributes.to_vec();
        sb.sort_unstable();
        let mut bfa = b_from_a;
        bfa.sort_unstable();
        let mut afb = a_from_b;
        afb.sort_unstable();
        sa == afb && sb == bfa
    }
    /// Number of objects.
    pub fn size_g(&self) -> usize {
        self.num_objects
    }
    /// Number of attributes.
    pub fn size_m(&self) -> usize {
        self.num_attributes
    }
}
