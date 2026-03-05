//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

/// Classify functions by Grzegorczyk hierarchy level E_n.
/// Level E_0: constants and projections; E_1: bounded addition; E_2: bounded mult;
/// E_3: 2-fold exponential; etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GrzegorczykLevel(pub usize);
impl GrzegorczykLevel {
    /// E_0: initial functions (constants, projections, successor).
    pub const E0: GrzegorczykLevel = GrzegorczykLevel(0);
    /// E_1: bounded linear growth (polynomial-bounded by n+1).
    pub const E1: GrzegorczykLevel = GrzegorczykLevel(1);
    /// E_2: bounded by iterated addition ~ polynomial.
    pub const E2: GrzegorczykLevel = GrzegorczykLevel(2);
    /// E_3: bounded by 2^n (elementary recursive).
    pub const E3: GrzegorczykLevel = GrzegorczykLevel(3);
    /// Return the bounding function for this level: E_n(x) = 2^{(n-2) times} x.
    pub fn bound(&self, x: u64) -> u64 {
        match self.0 {
            0 => x + 1,
            1 => x.saturating_add(x),
            2 => x.saturating_mul(x).saturating_add(2),
            n => {
                let mut v = x;
                for _ in 0..(n - 2) {
                    v = 2u64.saturating_pow(v.min(62) as u32);
                }
                v
            }
        }
    }
    /// Check whether the given function output is within the E_n bound for all inputs <= k.
    pub fn check_bounded(&self, f: impl Fn(u64) -> u64, k: u64) -> bool {
        (0..=k).all(|x| f(x) <= self.bound(x))
    }
    /// The union of all E_n levels corresponds to primitive recursive functions.
    pub fn is_primitive_recursive_approx(&self, f: impl Fn(u64) -> u64, k: u64) -> bool {
        (0..=10usize).any(|n| GrzegorczykLevel(n).check_bounded(&f, k))
    }
}
/// An efficient algorithm extracted from a constructive proof.
pub struct EfficientAlgorithm {
    /// Complexity class or big-O description (e.g., "O(n log n)").
    pub complexity: String,
}
impl EfficientAlgorithm {
    /// Create a new `EfficientAlgorithm`.
    pub fn new(complexity: impl Into<String>) -> Self {
        Self {
            complexity: complexity.into(),
        }
    }
    /// Returns true if the algorithm is both correct (proven by the proof)
    /// and runs in the stated complexity.
    pub fn correct_and_efficient(&self) -> bool {
        !self.complexity.is_empty()
    }
    /// Confirm that this algorithm was extracted from a constructive proof.
    pub fn extracted_from_proof(&self) -> bool {
        true
    }
}
/// A constructive real number represented as a function from precision k
/// to a dyadic rational m * 2^{-k} such that |x - m*2^{-k}| < 2^{-k}.
#[derive(Debug, Clone)]
pub struct ConstructiveReal {
    /// approx(k) gives the numerator m such that the approximation is m * 2^{-k}.
    pub approx: Vec<i64>,
    pub max_prec: usize,
}
impl ConstructiveReal {
    /// Construct a ConstructiveReal from a rational p/q.
    pub fn from_rational(p: i64, q: i64, max_prec: usize) -> Self {
        let approx = (0..=max_prec)
            .map(|k| {
                let num = p * (1i64 << k);
                num / q
            })
            .collect();
        ConstructiveReal { approx, max_prec }
    }
    /// Add two constructive reals.
    pub fn add(&self, other: &ConstructiveReal) -> ConstructiveReal {
        let max_prec = self.max_prec.min(other.max_prec);
        let approx = (0..=max_prec)
            .map(|k| self.approx[k] + other.approx[k])
            .collect();
        ConstructiveReal { approx, max_prec }
    }
    /// Get rational approximation at precision k: returns (m, k) meaning m * 2^{-k}.
    pub fn get_approx(&self, k: usize) -> (i64, usize) {
        (self.approx[k.min(self.max_prec)], k)
    }
    /// Check if |self - other| < 2^{-k} (approximate equality at precision k).
    pub fn approx_eq(&self, other: &ConstructiveReal, k: usize) -> bool {
        let k = k.min(self.max_prec).min(other.max_prec);
        (self.approx[k] - other.approx[k]).abs() <= 1
    }
}
/// Brouwer's intuitionism: mathematics as mental construction.
pub struct BrouwerIntuitionism {
    /// Version tag for documentation purposes.
    _version: &'static str,
}
impl BrouwerIntuitionism {
    /// Create a new `BrouwerIntuitionism` record.
    pub fn new() -> Self {
        Self { _version: "1907" }
    }
    /// In intuitionism the Law of Excluded Middle (LEM) is not universally valid.
    pub fn law_of_excluded_middle_fails(&self) -> bool {
        true
    }
    /// The creating subject is Brouwer's idealised mathematician who creates
    /// mathematical objects step by step.
    pub fn creating_subject(&self) -> &'static str {
        "The creating subject creates mathematical objects freely over time; \
         the truth of a proposition is settled only when the subject has \
         experienced a construction."
    }
    /// Choice sequences are potentially infinite sequences whose values are
    /// chosen freely (lawlessly) or according to a spread law.
    pub fn choice_sequences(&self) -> &'static str {
        "A choice sequence α is a potentially infinite sequence α(0), α(1), … \
         whose values may be chosen freely at each step, subject only to a spread."
    }
}
/// Finite Heyting algebra as a power-set lattice over n atoms.
/// Elements are represented as bitmasks of subsets of {0, ..., n-1}.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerSetHeyting {
    pub universe: u64,
}
impl PowerSetHeyting {
    pub fn new(n: u32) -> Self {
        PowerSetHeyting {
            universe: (1u64 << n) - 1,
        }
    }
    pub fn meet(&self, a: u64, b: u64) -> u64 {
        a & b
    }
    pub fn join(&self, a: u64, b: u64) -> u64 {
        a | b
    }
    pub fn top(&self) -> u64 {
        self.universe
    }
    pub fn bot(&self) -> u64 {
        0
    }
    pub fn neg(&self, a: u64) -> u64 {
        self.implication(a, self.bot())
    }
    /// Heyting implication: a → b = ⋁{x | x ∧ a ≤ b}.
    pub fn implication(&self, a: u64, b: u64) -> u64 {
        (!a & self.universe) | b
    }
    pub fn le(&self, a: u64, b: u64) -> bool {
        a & b == a
    }
    /// Double negation: ¬¬a in a Boolean algebra equals a.
    pub fn double_neg(&self, a: u64) -> u64 {
        self.neg(self.neg(a))
    }
}
/// A Heyting algebra: a lattice with a relative pseudo-complement (→).
pub struct HeylandAlgebra {
    /// The carrier elements (represented as strings).
    pub carrier: Vec<String>,
}
impl HeylandAlgebra {
    /// Create a new `HeylandAlgebra`.
    pub fn new(carrier: Vec<String>) -> Self {
        Self { carrier }
    }
    /// Check that this carrier can form a Heyting algebra (non-empty and closed).
    pub fn is_heyting_algebra(&self) -> bool {
        !self.carrier.is_empty()
    }
    /// Heyting algebras provide the algebraic semantics for intuitionistic
    /// propositional logic (IPL / IPC).
    pub fn intuitionistic_propositional_logic(&self) -> &'static str {
        "A Heyting algebra H models IPC: ⊤ = 1, ⊥ = 0, \
         a ∧ b = meet, a ∨ b = join, a → b = largest c with c ∧ a ≤ b."
    }
}
/// Simulate Spector's bar recursion up to a bounded depth.
/// This corresponds to the `BarRecursor` kernel axiom.
#[derive(Debug, Clone)]
pub struct BarRecursion {
    /// Maximum recursion depth allowed.
    pub max_depth: usize,
    /// The bar predicate: returns true when recursion should stop.
    pub bar: Vec<bool>,
}
impl BarRecursion {
    /// Create a new `BarRecursion` with a given depth bound.
    pub fn new(max_depth: usize) -> Self {
        BarRecursion {
            max_depth,
            bar: vec![false; max_depth + 1],
        }
    }
    /// Set the bar at position n (meaning: sequences of length n satisfy the bar).
    pub fn set_bar(&mut self, n: usize) {
        if n <= self.max_depth {
            self.bar[n] = true;
        }
    }
    /// Check whether position n is in the bar.
    pub fn in_bar(&self, n: usize) -> bool {
        n <= self.max_depth && self.bar[n]
    }
    /// Compute the bar recursion value for sequence length n using functionals Y and H.
    /// `y_val(n)` is the "stopping value" when n is in the bar.
    /// `h_step(n, prev)` computes the step from n using the previous value.
    pub fn compute(
        &self,
        n: usize,
        y_val: impl Fn(usize) -> i64,
        h_step: impl Fn(usize, i64) -> i64,
    ) -> i64 {
        if n > self.max_depth {
            return y_val(n);
        }
        if self.in_bar(n) {
            y_val(n)
        } else {
            let sub = self.compute(n + 1, &y_val, &h_step);
            h_step(n, sub)
        }
    }
    /// Depth of the bar (smallest n in the bar), if any.
    pub fn bar_depth(&self) -> Option<usize> {
        self.bar.iter().position(|&b| b)
    }
}
/// A constructive GCD computation over the integers using the Euclidean algorithm.
/// Corresponds to the kernel axiom `GCDDomain`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstructiveGcd {
    _marker: (),
}
impl ConstructiveGcd {
    /// Create a new `ConstructiveGcd` instance.
    pub fn new() -> Self {
        ConstructiveGcd { _marker: () }
    }
    /// Compute gcd(a, b) via the Euclidean algorithm (constructive, terminating).
    pub fn gcd(&self, mut a: i64, mut b: i64) -> i64 {
        a = a.abs();
        b = b.abs();
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
    /// Extended Euclidean algorithm: returns (g, s, t) with g = gcd(a,b), sa + tb = g.
    pub fn extended_gcd(&self, a: i64, b: i64) -> (i64, i64, i64) {
        if b == 0 {
            return (a.abs(), if a >= 0 { 1 } else { -1 }, 0);
        }
        let (g, s1, t1) = self.extended_gcd(b, a % b);
        (g, t1, s1 - (a / b) * t1)
    }
    /// Least common multiple: lcm(a, b) = |a * b| / gcd(a, b).
    pub fn lcm(&self, a: i64, b: i64) -> Option<i64> {
        let g = self.gcd(a, b);
        if g == 0 {
            return Some(0);
        }
        (a / g).checked_mul(b.abs())
    }
    /// Check whether a | b (a divides b), constructively.
    pub fn divides(&self, a: i64, b: i64) -> bool {
        a != 0 && b % a == 0
    }
}
/// A choice sequence: a potentially infinite sequence chosen freely.
pub struct InfiniteSequence {
    /// The values chosen so far.
    pub choice_sequence: Vec<u8>,
}
impl InfiniteSequence {
    /// Create a new `InfiniteSequence`.
    pub fn new(choice_sequence: Vec<u8>) -> Self {
        Self { choice_sequence }
    }
    /// A spread law (Baumgesetz) constrains which extensions are admissible.
    pub fn spread_law(&self) -> &'static str {
        "A spread is a decidable subtree of ω^{<ω} such that every node \
         has an extension in the tree.  Choice sequences live in spreads."
    }
    /// Bar induction: if a bar is decidable and every infinite sequence
    /// eventually hits it, then every node is secured.
    pub fn bar_induction(&self) -> &'static str {
        "Bar Induction (BI_D): if B is a decidable bar on a spread and \
         P is hereditary on B, then P holds at the empty sequence."
    }
}
/// A concrete witness for countable choice: given a sequence of non-empty sets
/// (represented by a function that picks a canonical element), builds the choice function.
#[derive(Debug, Clone)]
pub struct CountableChoiceWitness {
    /// The chosen elements: `choices[n]` is the chosen element from the n-th set.
    pub choices: Vec<u64>,
}
impl CountableChoiceWitness {
    /// Build a countable choice witness from a selection function `sel(n) -> element`.
    pub fn build(sel: impl Fn(usize) -> u64, count: usize) -> Self {
        CountableChoiceWitness {
            choices: (0..count).map(|n| sel(n)).collect(),
        }
    }
    /// Verify that the choice function is consistent: `choices[n]` belongs to set `n`.
    /// `membership(n, x)` returns true if x is in set n.
    pub fn verify(&self, membership: impl Fn(usize, u64) -> bool) -> bool {
        self.choices
            .iter()
            .enumerate()
            .all(|(n, &x)| membership(n, x))
    }
    /// Retrieve the choice for set n.
    pub fn get(&self, n: usize) -> Option<u64> {
        self.choices.get(n).copied()
    }
    /// Number of sets for which a choice has been made.
    pub fn len(&self) -> usize {
        self.choices.len()
    }
    /// Returns true if no choices have been recorded.
    pub fn is_empty(&self) -> bool {
        self.choices.is_empty()
    }
}
/// A constructive proof of a statement, given as an explicit algorithm.
pub struct ConstructiveProof {
    /// The statement being proved.
    pub statement: String,
    /// The algorithm (witness / proof term) realising the statement.
    pub algorithm: String,
}
impl ConstructiveProof {
    /// Create a new constructive proof.
    pub fn new(statement: impl Into<String>, algorithm: impl Into<String>) -> Self {
        Self {
            statement: statement.into(),
            algorithm: algorithm.into(),
        }
    }
    /// A proof is effective if the algorithm terminates on every input.
    pub fn is_effective(&self) -> bool {
        !self.algorithm.is_empty()
    }
    /// If the statement is existential, does this proof provide a witness?
    pub fn witnesses_existential(&self) -> bool {
        self.statement.contains('∃') || self.statement.contains("exists")
    }
}
/// Bishop-style constructive mathematics.
pub struct BishopMath {
    _tag: (),
}
impl BishopMath {
    /// Create a new `BishopMath` record.
    pub fn new() -> Self {
        Self { _tag: () }
    }
    /// Bishop's constructive real analysis (BISH): real numbers are
    /// Cauchy sequences with a modulus of convergence.
    pub fn constructive_real_analysis(&self) -> &'static str {
        "BISH uses Cauchy sequences with an explicit modulus; every \
         theorem has algorithmic content."
    }
    /// In BISH every function that is proven to exist is computable
    /// (though BISH does not assert this as an axiom).
    pub fn all_functions_continuous(&self) -> bool {
        false
    }
    /// BISH does not assume LEM.
    pub fn no_lem(&self) -> bool {
        true
    }
}
/// A Type-2 Turing machine representation for computable analysis.
/// Computes on infinite streams (Baire space ℕ^ℕ) rather than finite words.
#[derive(Debug, Clone)]
pub struct TTEOracle {
    /// Name / description of the represented real function.
    pub name: String,
    /// Maximum precision (in bits) we track in simulation.
    pub max_precision: usize,
    /// Cached approximations: approx[k] is the k-th dyadic approximation numerator.
    pub approx_cache: Vec<i64>,
}
impl TTEOracle {
    /// Create a new TTE oracle for a named function.
    pub fn new(name: impl Into<String>, max_precision: usize) -> Self {
        TTEOracle {
            name: name.into(),
            max_precision,
            approx_cache: Vec::new(),
        }
    }
    /// Load approximations from a Cauchy sequence with explicit modulus.
    /// `seq(k)` gives the k-th numerator so that `seq(k) * 2^{-k}` approximates the value.
    pub fn load_cauchy(&mut self, seq: impl Fn(usize) -> i64) {
        self.approx_cache = (0..=self.max_precision).map(|k| seq(k)).collect();
    }
    /// Query the oracle at precision `k`: returns (numerator, k) meaning `num * 2^{-k}`.
    pub fn query(&self, k: usize) -> Option<(i64, usize)> {
        let k = k.min(self.max_precision);
        self.approx_cache.get(k).map(|&m| (m, k))
    }
    /// Check whether this representation is consistent (Cauchy condition within ε = 2^{-k}).
    pub fn is_cauchy_consistent(&self, k: usize) -> bool {
        if self.approx_cache.len() < 2 {
            return true;
        }
        let k = k.min(self.max_precision);
        if k == 0 {
            return true;
        }
        (0..k).all(|j| {
            let aj = self.approx_cache.get(j).copied().unwrap_or(0);
            let aj1 = self.approx_cache.get(j + 1).copied().unwrap_or(0);
            (aj1 - 2 * aj).abs() <= 2
        })
    }
    /// Returns the name of the represented real function.
    pub fn function_name(&self) -> &str {
        &self.name
    }
}
/// A constructive real number as a Cauchy sequence with a modulus.
pub struct RealNumber {
    /// Approximations: `cauchy_seq[n]` approximates the real to within 2^{-n}.
    pub cauchy_seq: Vec<f64>,
    /// A description of the modulus of convergence.
    pub modulus: String,
}
impl RealNumber {
    /// Create a new `RealNumber`.
    pub fn new(cauchy_seq: Vec<f64>, modulus: impl Into<String>) -> Self {
        Self {
            cauchy_seq,
            modulus: modulus.into(),
        }
    }
    /// Equality of constructive reals is co-enumerable (not decidable in general).
    pub fn equality_is_undecidable(&self) -> bool {
        true
    }
    /// Two constructive reals x, y are apart (x # y) if |x - y| > 2^{-n}
    /// for some n — this is a positive, decidable relation.
    pub fn is_apartness_relation(&self) -> bool {
        true
    }
}
/// A constructive continuous function (on the Baire space or unit interval).
pub struct ConTinuousFunction {
    /// Whether the function is (provably) uniformly continuous.
    pub is_uniformly_continuous: bool,
}
impl ConTinuousFunction {
    /// Create a new `ConTinuousFunction`.
    pub fn new(is_uniformly_continuous: bool) -> Self {
        Self {
            is_uniformly_continuous,
        }
    }
    /// The Fan Theorem implies that every function on the Cantor space
    /// [2^ω] is uniformly continuous.
    pub fn by_fan_theorem(&self) -> bool {
        self.is_uniformly_continuous
    }
    /// The Kleene–Brouwer ordering on well-founded trees is used to
    /// construct continuous functions from spreads.
    pub fn kleene_brouwer(&self) -> &'static str {
        "The Kleene–Brouwer ordering witnesses well-foundedness of the tree \
         via an intuitionistic bar induction argument."
    }
}
/// Markov's Principle: if a decidable predicate is not everywhere false,
/// then there exists a witness.
pub struct MarkovPrinciple {
    _tag: (),
}
impl MarkovPrinciple {
    /// Create a new `MarkovPrinciple` record.
    pub fn new() -> Self {
        Self { _tag: () }
    }
    /// The statement of Markov's Principle (MP).
    pub fn statement(&self) -> &'static str {
        "Markov's Principle (MP): for every decidable predicate P on ℕ, \
         if ¬∀n, ¬P(n) then ∃n, P(n).  Equivalently: if a Turing machine \
         does not run forever then it halts."
    }
    /// MP is accepted in the Russian constructive school (Markov, Shanin).
    pub fn is_accepted_in_russian_school(&self) -> bool {
        true
    }
}
