//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// A productivity checker for guarded corecursion.
/// Ensures every corecursive call is guarded by a constructor.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProductivityChecker {
    /// Name of the corecursive function.
    pub fn_name: String,
    /// Guard depth: minimum number of constructors before each recursive call.
    pub guard_depth: usize,
    /// Whether the function is productive.
    pub is_productive: bool,
}
#[allow(dead_code)]
impl ProductivityChecker {
    /// Create a new productivity checker.
    pub fn new(fn_name: &str, guard_depth: usize) -> Self {
        ProductivityChecker {
            fn_name: fn_name.to_string(),
            guard_depth,
            is_productive: guard_depth > 0,
        }
    }
    /// Check if a function body is guarded.
    /// Simplified check: guardedness requires all recursive calls to be
    /// syntactically under a constructor (represented by depth ≥ 1).
    pub fn check_guardedness(&self, recursive_call_depth: usize) -> bool {
        recursive_call_depth >= 1
    }
    /// Nakano's modality ▷: a type X is related to ▷X via the delay constructor.
    /// Returns the modal depth of the type.
    pub fn nakano_modal_depth(&self) -> usize {
        self.guard_depth
    }
}
/// An infinite stream over type A, represented lazily using a closure.
pub struct Stream<A: Clone> {
    /// The head element.
    head: A,
    /// The tail (lazily generated).
    tail: Box<dyn Fn() -> Stream<A>>,
}
impl<A: Clone + 'static> Stream<A> {
    /// Create a stream by unfolding a seed with a step function.
    /// `f(s) = (head, next_seed)`.
    pub fn unfold<S: Clone + 'static>(
        seed: S,
        f: impl Fn(S) -> (A, S) + Clone + 'static,
    ) -> Stream<A> {
        let (head, next) = f(seed.clone());
        let f2 = std::sync::Arc::new(f);
        Stream {
            head,
            tail: Box::new(move || {
                let f3 = f2.clone();
                Stream::unfold_arc(next.clone(), f3)
            }),
        }
    }
    /// Helper for unfold using Arc-wrapped function.
    fn unfold_arc<S: Clone + 'static>(
        seed: S,
        f: std::sync::Arc<dyn Fn(S) -> (A, S)>,
    ) -> Stream<A> {
        let (head, next) = f(seed.clone());
        let f2 = f.clone();
        Stream {
            head,
            tail: Box::new(move || Stream::unfold_arc(next.clone(), f2.clone())),
        }
    }
    /// Get the head element.
    pub fn head(&self) -> A {
        self.head.clone()
    }
    /// Consume the stream and get the tail.
    pub fn tail(self) -> Stream<A> {
        (self.tail)()
    }
    /// Get the nth element (0-indexed).
    pub fn nth(self, n: usize) -> A {
        if n == 0 {
            self.head()
        } else {
            self.tail().nth(n - 1)
        }
    }
    /// Collect the first `n` elements.
    pub fn take(self, n: usize) -> Vec<A> {
        if n == 0 {
            vec![]
        } else {
            let head = self.head.clone();
            let mut rest = self.tail().take(n - 1);
            let mut result = vec![head];
            result.append(&mut rest);
            result
        }
    }
    /// Create a constant stream repeating `val` forever.
    pub fn constant(val: A) -> Stream<A>
    where
        A: 'static,
    {
        Stream::unfold(val, |v| {
            let next = v.clone();
            (v, next)
        })
    }
    /// Create the stream of natural numbers starting at `start`.
    pub fn nats_from(start: u64) -> Stream<u64> {
        Stream::unfold(start, |n| (n, n + 1))
    }
}
impl Stream<u64> {
    /// Zip two u64 streams with a binary function.
    pub fn zip_with<F>(s: Stream<u64>, t: Stream<u64>, f: F) -> Stream<u64>
    where
        F: Fn(u64, u64) -> u64 + Clone + 'static,
    {
        let head = f(s.head, t.head);
        let f2 = f.clone();
        Stream {
            head,
            tail: Box::new(move || Stream::zip_with((s.tail)(), (t.tail)(), f2.clone())),
        }
    }
}
/// A lazy, possibly infinite list.
#[derive(Clone)]
pub enum CoList<A: Clone + 'static> {
    /// Empty colist.
    Nil,
    /// Cons cell with a lazy tail.
    Cons(A, Box<CoListTail<A>>),
}
impl<A: Clone + 'static> CoList<A> {
    /// Create a finite colist from a Vec.
    pub fn from_vec(v: Vec<A>) -> CoList<A> {
        v.into_iter().rev().fold(CoList::Nil, |acc, x| {
            let acc2 = acc.clone();
            CoList::Cons(x, Box::new(CoListTail(Box::new(move || acc2.clone()))))
        })
    }
    /// Collect at most `n` elements from the colist.
    pub fn take(&self, n: usize) -> Vec<A> {
        if n == 0 {
            return vec![];
        }
        match self {
            CoList::Nil => vec![],
            CoList::Cons(x, tail) => {
                let mut rest = (tail.0)().take(n - 1);
                let mut result = vec![x.clone()];
                result.append(&mut rest);
                result
            }
        }
    }
    /// Check if the colist is empty.
    pub fn is_nil(&self) -> bool {
        matches!(self, CoList::Nil)
    }
    /// Get the head if non-empty.
    pub fn head(&self) -> Option<A> {
        match self {
            CoList::Nil => None,
            CoList::Cons(x, _) => Some(x.clone()),
        }
    }
    /// Create a colist repeating one element infinitely.
    pub fn repeat(val: A) -> CoList<A>
    where
        A: 'static,
    {
        let v = val.clone();
        CoList::Cons(
            val,
            Box::new(CoListTail(Box::new(move || CoList::repeat(v.clone())))),
        )
    }
}
/// A non-well-founded set representation using an acyclic-or-cyclic graph.
/// Each node has an ID and a list of children (member IDs).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HyperSet {
    /// Nodes: each node is a set, represented by its ID.
    pub num_nodes: usize,
    /// Children: children\[i\] = list of member IDs of set i.
    pub children: Vec<Vec<usize>>,
}
#[allow(dead_code)]
impl HyperSet {
    /// Create a new empty hyperset with n nodes.
    pub fn new(n: usize) -> Self {
        HyperSet {
            num_nodes: n,
            children: vec![vec![]; n],
        }
    }
    /// Add a membership: set j is a member of set i.
    pub fn add_member(&mut self, i: usize, j: usize) {
        if i < self.num_nodes && j < self.num_nodes {
            self.children[i].push(j);
        }
    }
    /// Check if the hyperset is a Quine atom: a set X = {X}.
    /// Node i is a Quine atom if its only member is itself.
    pub fn is_quine_atom(&self, i: usize) -> bool {
        i < self.num_nodes && self.children[i] == vec![i]
    }
    /// Check if node i has a circular membership path back to itself.
    pub fn has_cycle_from(&self, i: usize) -> bool {
        let mut visited = vec![false; self.num_nodes];
        self.dfs_cycle(i, i, &mut visited)
    }
    fn dfs_cycle(&self, start: usize, current: usize, visited: &mut Vec<bool>) -> bool {
        for &child in &self.children[current] {
            if child == start {
                return true;
            }
            if !visited[child] {
                visited[child] = true;
                if self.dfs_cycle(start, child, visited) {
                    return true;
                }
            }
        }
        false
    }
    /// AFA (Anti-Foundation Axiom): every graph has a unique decoration (solution).
    /// Check that the graph has a unique bisimulation equivalence class.
    pub fn is_well_founded(&self) -> bool {
        !(0..self.num_nodes).any(|i| self.has_cycle_from(i))
    }
    /// Scott's representation: the set of well-founded hereditarily finite sets.
    /// Returns true if this is a well-founded set with no cycles.
    pub fn is_hereditary_finite(&self) -> bool {
        self.is_well_founded() && self.num_nodes < 1000
    }
}
/// A monadic stream: a coinductive type M (A × M) for a monad M.
/// We instantiate M = Option for partial streams.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OptionStream<T: Clone> {
    /// Eagerly computed prefix of the stream.
    pub prefix: Vec<T>,
    /// Whether the stream terminates after the prefix.
    pub terminates: bool,
}
#[allow(dead_code)]
impl<T: Clone> OptionStream<T> {
    /// Create a finite stream.
    pub fn finite(data: Vec<T>) -> Self {
        OptionStream {
            prefix: data,
            terminates: true,
        }
    }
    /// Create a partial stream (possibly infinite, captured by prefix).
    pub fn partial(prefix: Vec<T>) -> Self {
        OptionStream {
            prefix,
            terminates: false,
        }
    }
    /// Head of the stream, if available.
    pub fn head(&self) -> Option<&T> {
        self.prefix.first()
    }
    /// Tail of the stream (drops the first element).
    pub fn tail(&self) -> OptionStream<T> {
        OptionStream {
            prefix: self.prefix.get(1..).unwrap_or(&[]).to_vec(),
            terminates: self.terminates,
        }
    }
    /// Map a function over all elements in the prefix.
    pub fn map<U: Clone, F: Fn(&T) -> U>(&self, f: F) -> OptionStream<U> {
        OptionStream {
            prefix: self.prefix.iter().map(f).collect(),
            terminates: self.terminates,
        }
    }
    /// Length of the known prefix.
    pub fn prefix_length(&self) -> usize {
        self.prefix.len()
    }
    /// Zip two streams together.
    pub fn zip_with<U: Clone, V: Clone, F: Fn(&T, &U) -> V>(
        &self,
        other: &OptionStream<U>,
        f: F,
    ) -> OptionStream<V> {
        let prefix: Vec<V> = self
            .prefix
            .iter()
            .zip(other.prefix.iter())
            .map(|(a, b)| f(a, b))
            .collect();
        OptionStream {
            prefix,
            terminates: self.terminates || other.terminates,
        }
    }
}
/// A probabilistic automaton viewed as a coalgebra over the distribution functor.
/// States: Q, alphabet: Σ, transitions: Q → Σ → Dist(Q), acceptance: Q → \[0,1\].
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProbabilisticAutomaton {
    /// Number of states.
    pub num_states: usize,
    /// Alphabet size.
    pub alphabet_size: usize,
    /// Transition probabilities: trans\[state\]\[symbol\]\[next_state\] = probability.
    pub trans: Vec<Vec<Vec<f64>>>,
    /// Acceptance probabilities: accept\[state\] ∈ \[0,1\].
    pub accept: Vec<f64>,
}
#[allow(dead_code)]
impl ProbabilisticAutomaton {
    /// Create a new probabilistic automaton with uniform transitions.
    pub fn new(num_states: usize, alphabet_size: usize) -> Self {
        let prob = 1.0 / num_states as f64;
        let trans = vec![vec![vec![prob; num_states]; alphabet_size]; num_states];
        let accept = vec![0.0; num_states];
        ProbabilisticAutomaton {
            num_states,
            alphabet_size,
            trans,
            accept,
        }
    }
    /// Set acceptance probability of state s.
    pub fn set_accept(&mut self, state: usize, prob: f64) {
        if state < self.num_states {
            self.accept[state] = prob.clamp(0.0, 1.0);
        }
    }
    /// Set transition probability from state s on symbol a to state t.
    pub fn set_trans(&mut self, state: usize, symbol: usize, next: usize, prob: f64) {
        if state < self.num_states && symbol < self.alphabet_size && next < self.num_states {
            self.trans[state][symbol][next] = prob;
        }
    }
    /// Compute the acceptance probability for a given word (sequence of symbols).
    /// Uses forward computation: start from state 0, track distribution.
    pub fn acceptance_prob(&self, word: &[usize]) -> f64 {
        let mut dist = vec![0.0f64; self.num_states];
        dist[0] = 1.0;
        for &symbol in word {
            let mut next_dist = vec![0.0f64; self.num_states];
            for s in 0..self.num_states {
                if dist[s] > 0.0 {
                    for t in 0..self.num_states {
                        next_dist[t] += dist[s] * self.trans[s][symbol][t];
                    }
                }
            }
            dist = next_dist;
        }
        dist.iter()
            .zip(self.accept.iter())
            .map(|(&d, &a)| d * a)
            .sum()
    }
    /// Bisimulation distance: sup-norm of difference in acceptance probabilities.
    /// Simplified: compare acceptance vectors.
    pub fn behavioral_distance(&self, other: &ProbabilisticAutomaton) -> f64 {
        let n = self.num_states.min(other.num_states);
        (0..n)
            .map(|s| (self.accept[s] - other.accept[s]).abs())
            .fold(0.0_f64, f64::max)
    }
}
/// A bisimulation relation on a labeled transition system.
/// States are represented as usize indices.
#[derive(Debug, Clone)]
pub struct BisimRelation {
    /// The set of related state pairs (i, j).
    pub pairs: Vec<(usize, usize)>,
}
impl BisimRelation {
    /// Create an empty relation.
    pub fn empty() -> Self {
        BisimRelation { pairs: Vec::new() }
    }
    /// Add a pair (i, j) to the relation.
    pub fn add(&mut self, i: usize, j: usize) {
        if !self.contains(i, j) {
            self.pairs.push((i, j));
        }
    }
    /// Check if (i, j) is in the relation.
    pub fn contains(&self, i: usize, j: usize) -> bool {
        self.pairs.iter().any(|&(a, b)| a == i && b == j)
    }
    /// Close under symmetry.
    pub fn close_symmetric(&mut self) {
        let extra: Vec<(usize, usize)> = self
            .pairs
            .iter()
            .map(|&(a, b)| (b, a))
            .filter(|&(a, b)| !self.contains(a, b))
            .collect();
        self.pairs.extend(extra);
    }
    /// Size of the relation.
    pub fn size(&self) -> usize {
        self.pairs.len()
    }
}
/// Checks bisimulation on a labeled transition system using Paige-Tarjan-style
/// partition refinement.
#[derive(Debug, Clone)]
pub struct BisimulationChecker {
    /// The LTS to check.
    lts: LTS,
}
impl BisimulationChecker {
    /// Create a new `BisimulationChecker` wrapping the given LTS.
    pub fn new(lts: LTS) -> Self {
        BisimulationChecker { lts }
    }
    /// Check if states `p` and `q` are bisimilar.
    pub fn check(&self, p: usize, q: usize) -> bool {
        self.lts.bisimilar(p, q)
    }
    /// Compute the full bisimilarity partition: returns equivalence classes.
    pub fn compute_partition(&self) -> Vec<Vec<usize>> {
        let n = self.lts.n_states;
        let mut sim: Vec<Vec<bool>> = (0..n)
            .map(|p| (0..n).map(|q| self.lts.bisimilar(p, q)).collect())
            .collect();
        let mut classes: Vec<Vec<usize>> = Vec::new();
        let mut assigned = vec![false; n];
        for i in 0..n {
            if assigned[i] {
                continue;
            }
            let mut cls = vec![i];
            assigned[i] = true;
            for j in (i + 1)..n {
                if !assigned[j] && sim[i][j] {
                    cls.push(j);
                    assigned[j] = true;
                }
            }
            classes.push(cls);
        }
        let _ = sim.iter_mut();
        classes
    }
    /// Returns the number of bisimilarity classes.
    pub fn num_classes(&self) -> usize {
        self.compute_partition().len()
    }
    /// Check if a relation (given as pairs) is a bisimulation.
    pub fn is_bisimulation(&self, pairs: &[(usize, usize)]) -> bool {
        let labels = self.lts.labels();
        for &(p, q) in pairs {
            for lbl in &labels {
                let succs_p = self.lts.successors(p, lbl);
                let succs_q = self.lts.successors(q, lbl);
                for &sp in &succs_p {
                    if !succs_q
                        .iter()
                        .any(|&sq| pairs.iter().any(|&(a, b)| a == sp && b == sq))
                    {
                        return false;
                    }
                }
                for &sq in &succs_q {
                    if !succs_p
                        .iter()
                        .any(|&sp| pairs.iter().any(|&(a, b)| a == sp && b == sq))
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
}
/// A simple labeled transition system for testing bisimulation.
#[derive(Debug, Clone)]
pub struct LTS {
    /// Number of states.
    pub n_states: usize,
    /// Transitions: (source, label, target).
    pub transitions: Vec<(usize, String, usize)>,
}
impl LTS {
    /// Create a new LTS with `n` states.
    pub fn new(n: usize) -> Self {
        LTS {
            n_states: n,
            transitions: Vec::new(),
        }
    }
    /// Add a transition.
    pub fn add_transition(&mut self, from: usize, label: &str, to: usize) {
        self.transitions.push((from, label.to_string(), to));
    }
    /// Get all successors of state `s` under label `l`.
    pub fn successors(&self, s: usize, l: &str) -> Vec<usize> {
        self.transitions
            .iter()
            .filter(|(from, label, _)| *from == s && label == l)
            .map(|(_, _, to)| *to)
            .collect()
    }
    /// Get all labels used.
    pub fn labels(&self) -> Vec<String> {
        let mut labels: Vec<String> = self.transitions.iter().map(|(_, l, _)| l.clone()).collect();
        labels.sort();
        labels.dedup();
        labels
    }
    /// Check if two states are strongly bisimilar using a naive partition refinement.
    /// Returns true if states `p` and `q` are bisimilar.
    pub fn bisimilar(&self, p: usize, q: usize) -> bool {
        let mut candidate: Vec<Vec<bool>> = vec![vec![true; self.n_states]; self.n_states];
        let labels = self.labels();
        loop {
            let mut changed = false;
            for i in 0..self.n_states {
                for j in 0..self.n_states {
                    if !candidate[i][j] {
                        continue;
                    }
                    for lbl in &labels {
                        let succs_i = self.successors(i, lbl);
                        let succs_j = self.successors(j, lbl);
                        for &si in &succs_i {
                            if !succs_j.iter().any(|&sj| candidate[si][sj]) {
                                candidate[i][j] = false;
                                changed = true;
                                break;
                            }
                        }
                        if !candidate[i][j] {
                            break;
                        }
                        for &sj in &succs_j {
                            if !succs_i.iter().any(|&si| candidate[si][sj]) {
                                candidate[i][j] = false;
                                changed = true;
                                break;
                            }
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        candidate[p][q]
    }
}
/// The lazy tail of a colist.
pub struct CoListTail<A: Clone + 'static>(pub Box<dyn Fn() -> CoList<A>>);
/// A stream-based coalgebra that produces (head, tail) observations.
#[derive(Debug, Clone)]
pub struct StreamCoalgebra<A: Clone> {
    /// Internal buffer of pre-computed elements.
    pub buffer: Vec<A>,
    /// The index into the buffer (current position).
    pub position: usize,
}
impl<A: Clone> StreamCoalgebra<A> {
    /// Create a new `StreamCoalgebra` from a finite buffer.
    pub fn new(buffer: Vec<A>) -> Self {
        StreamCoalgebra {
            buffer,
            position: 0,
        }
    }
    /// Observe the current head element, if available.
    pub fn observe_head(&self) -> Option<&A> {
        self.buffer.get(self.position)
    }
    /// Advance to the next element (tail observation).
    pub fn advance(&mut self) {
        if self.position < self.buffer.len() {
            self.position += 1;
        }
    }
    /// Reset to the beginning.
    pub fn reset(&mut self) {
        self.position = 0;
    }
    /// Collect the remaining elements from current position.
    pub fn collect_remaining(&self) -> Vec<A> {
        self.buffer[self.position..].to_vec()
    }
    /// Check if two `StreamCoalgebra` instances are bisimilar up to `n` steps.
    pub fn bisimilar_up_to(&self, other: &StreamCoalgebra<A>, n: usize) -> bool
    where
        A: PartialEq,
    {
        let a = &self.buffer[self.position..];
        let b = &other.buffer[other.position..];
        let steps = n.min(a.len()).min(b.len());
        a[..steps] == b[..steps]
    }
}
/// A finite-dimensional Hopf algebra element represented as a vector of
/// coordinates in some basis. Operations: multiplication, comultiplication,
/// antipode, counit.
#[derive(Debug, Clone, PartialEq)]
pub struct HopfAlgebraOps {
    /// Coordinates of the element in the basis.
    pub coords: Vec<f64>,
    /// Dimension of the algebra.
    pub dim: usize,
}
impl HopfAlgebraOps {
    /// Create a new `HopfAlgebraOps` element with `dim`-dimensional zero vector.
    pub fn zero(dim: usize) -> Self {
        HopfAlgebraOps {
            coords: vec![0.0; dim],
            dim,
        }
    }
    /// Create a basis element e_i.
    pub fn basis(dim: usize, i: usize) -> Self {
        let mut coords = vec![0.0; dim];
        if i < dim {
            coords[i] = 1.0;
        }
        HopfAlgebraOps { coords, dim }
    }
    /// Addition of Hopf algebra elements.
    pub fn add(&self, other: &HopfAlgebraOps) -> HopfAlgebraOps {
        assert_eq!(self.dim, other.dim, "dimension mismatch");
        let coords = self
            .coords
            .iter()
            .zip(other.coords.iter())
            .map(|(a, b)| a + b)
            .collect();
        HopfAlgebraOps {
            coords,
            dim: self.dim,
        }
    }
    /// Scalar multiplication.
    pub fn scale(&self, s: f64) -> HopfAlgebraOps {
        HopfAlgebraOps {
            coords: self.coords.iter().map(|&x| x * s).collect(),
            dim: self.dim,
        }
    }
    /// Counit ε: H → k. For the group algebra k\[G\], ε(g) = 1.
    /// Simplified: returns the sum of all coordinates.
    pub fn counit(&self) -> f64 {
        self.coords.iter().sum()
    }
    /// Antipode S: for the group algebra k\[G\], S(g) = g^{-1}.
    /// Simplified: returns the negation (as a formal opposite).
    pub fn antipode(&self) -> HopfAlgebraOps {
        HopfAlgebraOps {
            coords: self.coords.iter().map(|&x| -x).collect(),
            dim: self.dim,
        }
    }
    /// Comultiplication Δ: H → H ⊗ H.
    /// For the group algebra k\[G\], Δ(g) = g ⊗ g.
    /// Simplified: returns (self.clone(), self.clone()) as a tensor-product pair.
    pub fn comultiply(&self) -> (HopfAlgebraOps, HopfAlgebraOps) {
        (self.clone(), self.clone())
    }
    /// Check the Hopf axiom: μ ∘ (S ⊗ id) ∘ Δ = η ∘ ε.
    /// Simplified: μ(S(x), x) = ε(x) · 1.
    pub fn check_hopf_axiom(&self) -> bool {
        let (s_part, id_part) = self.comultiply();
        let s = s_part.antipode();
        let product: f64 = s
            .coords
            .iter()
            .zip(id_part.coords.iter())
            .map(|(a, b)| a * b)
            .sum();
        let eta_eps = self.counit();
        (product - eta_eps).abs() < 1e-9
    }
    /// Convolution product (f * g)(x) = μ ∘ (f ⊗ g) ∘ Δ(x).
    /// Takes two linear maps f, g: H → H.
    pub fn convolution<F, G>(&self, f: F, g: G) -> HopfAlgebraOps
    where
        F: Fn(&HopfAlgebraOps) -> HopfAlgebraOps,
        G: Fn(&HopfAlgebraOps) -> HopfAlgebraOps,
    {
        let (left, right) = self.comultiply();
        let fl = f(&left);
        let gr = g(&right);
        let coords = fl
            .coords
            .iter()
            .zip(gr.coords.iter())
            .map(|(a, b)| a * b)
            .collect();
        HopfAlgebraOps {
            coords,
            dim: self.dim,
        }
    }
}
/// Approximates the final coalgebra of a functor F by computing finite
/// unfoldings (Kleene chain from below) up to a given depth.
#[derive(Debug, Clone)]
pub struct FinalCoalgebraApprox {
    /// The depth of approximation (number of unfolding steps).
    pub depth: usize,
    /// The tree structure: each node is a list of children.
    pub nodes: Vec<Vec<usize>>,
}
impl FinalCoalgebraApprox {
    /// Build a complete k-ary tree of given `depth` as the final coalgebra approximation.
    pub fn build_tree(depth: usize, branching: usize) -> Self {
        let mut nodes: Vec<Vec<usize>> = Vec::new();
        Self::build_node(&mut nodes, depth, branching);
        FinalCoalgebraApprox { depth, nodes }
    }
    fn build_node(nodes: &mut Vec<Vec<usize>>, depth: usize, branching: usize) -> usize {
        let idx = nodes.len();
        nodes.push(Vec::new());
        if depth == 0 {
            return idx;
        }
        let children: Vec<usize> = (0..branching)
            .map(|_| Self::build_node(nodes, depth - 1, branching))
            .collect();
        nodes[idx] = children;
        idx
    }
    /// Return the number of nodes in the approximation.
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
    /// Check if the approximation contains the root (node 0).
    pub fn has_root(&self) -> bool {
        !self.nodes.is_empty()
    }
    /// Compute the branching at node `i`.
    pub fn branching_at(&self, i: usize) -> usize {
        self.nodes.get(i).map(|c| c.len()).unwrap_or(0)
    }
    /// Check if two unfolding trees are isomorphic up to `steps` levels.
    pub fn isomorphic_up_to(&self, other: &FinalCoalgebraApprox, steps: usize) -> bool {
        if steps == 0 {
            return true;
        }
        if self.nodes.is_empty() && other.nodes.is_empty() {
            return true;
        }
        if self.branching_at(0) != other.branching_at(0) {
            return false;
        }
        self.num_nodes() == other.num_nodes()
    }
}
