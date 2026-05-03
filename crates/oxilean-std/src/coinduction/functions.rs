//! Coinductive proofs and greatest fixed points — implementations and tests.

use super::types::{
    BisimulationRelation, CoalgebraMap, Codata, CoinductiveProof, GreibachNormalForm, LazyStream,
    StreamApprox, StreamNode,
};

// ── LazyStream ────────────────────────────────────────────────────────────────

impl<T: Clone> LazyStream<T> {
    /// Create a stream with an explicit prefix and repeating cycle.
    ///
    /// If `cycle` is empty the stream is treated as a finite prefix followed
    /// by an infinite repetition of the last prefix element; if the prefix is
    /// also empty the stream returns the default value if T: Default, but
    /// callers should guarantee `cycle` is non-empty for infinite streams.
    pub fn new(prefix: Vec<T>, cycle: Vec<T>) -> Self {
        LazyStream { prefix, cycle }
    }

    /// Create the constant stream `[val, val, val, ...]`.
    pub fn constant(val: T) -> Self {
        LazyStream {
            prefix: Vec::new(),
            cycle: vec![val],
        }
    }

    /// Create a stream that repeats `vals` forever: `vals\[0\], vals\[1\], ..., vals[n-1], vals\[0\], ...`.
    ///
    /// Panics if `vals` is empty.
    pub fn cycle(vals: Vec<T>) -> Self {
        assert!(!vals.is_empty(), "cycle must be non-empty");
        LazyStream {
            prefix: Vec::new(),
            cycle: vals,
        }
    }

    /// Return the first `n` elements of the stream.
    pub fn take(&self, n: usize) -> Vec<T> {
        (0..n).map(|i| self.nth(i)).collect()
    }

    /// Return the `n`-th element (0-indexed).
    pub fn nth(&self, n: usize) -> T {
        if n < self.prefix.len() {
            self.prefix[n].clone()
        } else if self.cycle.is_empty() {
            // Fallback: repeat last prefix element (should not happen in normal use)
            self.prefix
                .last()
                .cloned()
                .expect("LazyStream: both prefix and cycle are empty")
        } else {
            let cycle_idx = (n - self.prefix.len()) % self.cycle.len();
            self.cycle[cycle_idx].clone()
        }
    }

    /// Apply `f` element-wise, producing a new `LazyStream<U>`.
    pub fn map<U: Clone, F: Fn(&T) -> U>(&self, f: F) -> LazyStream<U> {
        LazyStream {
            prefix: self.prefix.iter().map(|x| f(x)).collect(),
            cycle: self.cycle.iter().map(|x| f(x)).collect(),
        }
    }

    /// Point-wise combine two streams with a binary function.
    ///
    /// The resulting stream has:
    /// - prefix length = max of the two prefixes, extended by cycling the shorter cycle
    /// - cycle length = lcm of the two cycle lengths
    pub fn zip<U: Clone, V: Clone, F: Fn(&T, &U) -> V>(
        &self,
        other: &LazyStream<U>,
        f: F,
    ) -> LazyStream<V> {
        // Compute LCM of cycle lengths for the combined cycle
        let lcm_len = if self.cycle.is_empty() || other.cycle.is_empty() {
            0
        } else {
            lcm(self.cycle.len(), other.cycle.len())
        };

        // Unified prefix length: max of both prefix lengths
        let prefix_len = self.prefix.len().max(other.prefix.len());
        let combined_prefix: Vec<V> = (0..prefix_len)
            .map(|i| f(&self.nth(i), &other.nth(i)))
            .collect();

        let combined_cycle: Vec<V> = (0..lcm_len)
            .map(|i| f(&self.nth(prefix_len + i), &other.nth(prefix_len + i)))
            .collect();

        LazyStream {
            prefix: combined_prefix,
            cycle: combined_cycle,
        }
    }

    /// Convert the stream to a finite unrolled approximation of depth `n`.
    pub fn to_approx(&self, n: usize) -> StreamApprox<T> {
        self.to_approx_at(0, n)
    }

    fn to_approx_at(&self, offset: usize, depth: usize) -> StreamApprox<T> {
        if depth == 0 {
            StreamApprox::Nil
        } else {
            let head = self.nth(offset);
            StreamApprox::Cons(Box::new(StreamNode {
                head,
                tail: Box::new(self.to_approx_at(offset + 1, depth - 1)),
            }))
        }
    }
}

// ── greatest common divisor / least common multiple ──────────────────────────

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        a / gcd(a, b) * b
    }
}

// ── CoalgebraMap ──────────────────────────────────────────────────────────────

impl<S: Clone + Eq, O: Clone + Eq> CoalgebraMap<S, O> {
    /// Create an empty coalgebra map.
    pub fn new() -> Self {
        CoalgebraMap {
            transitions: Vec::new(),
        }
    }

    /// Add a transition `src --obs--> dst`.
    pub fn add_transition(&mut self, src: S, obs: O, dst: S) {
        self.transitions.push((src, obs, dst));
    }

    /// Return all transitions out of `state`.
    pub fn transitions_from(&self, state: &S) -> Vec<(&O, &S)> {
        self.transitions
            .iter()
            .filter(|(s, _, _)| s == state)
            .map(|(_, o, d)| (o, d))
            .collect()
    }
}

// ── BisimulationRelation ──────────────────────────────────────────────────────

impl<S: Clone + Eq> BisimulationRelation<S> {
    /// Create an empty bisimulation relation.
    pub fn new() -> Self {
        BisimulationRelation { pairs: Vec::new() }
    }

    /// Add a pair `(s, t)` to the candidate relation.
    pub fn add_pair(&mut self, s: S, t: S) {
        if !self.pairs.iter().any(|(a, b)| *a == s && *b == t) {
            self.pairs.push((s, t));
        }
    }

    /// Return true if `(s, t)` is in the relation.
    pub fn contains(&self, s: &S, t: &S) -> bool {
        self.pairs.iter().any(|(a, b)| a == s && b == t)
    }

    /// Number of pairs.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }

    /// True if no pairs.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }

    /// Verify that `self` is a valid bisimulation w.r.t. `coalgebra`.
    ///
    /// A relation R is a bisimulation if, for every (s, t) ∈ R:
    /// - for every transition s --o--> s', there exists t --o--> t' with (s', t') ∈ R
    /// - for every transition t --o--> t', there exists s --o--> s' with (s', t') ∈ R
    pub fn check<O: Clone + Eq>(&self, coalgebra: &CoalgebraMap<S, O>) -> bool {
        for (s, t) in &self.pairs {
            // Check forward simulation: s ⊆ t
            for (obs, s_next) in coalgebra.transitions_from(s) {
                let matched = coalgebra
                    .transitions_from(t)
                    .into_iter()
                    .any(|(o2, t_next)| o2 == obs && self.contains(s_next, t_next));
                if !matched {
                    return false;
                }
            }
            // Check backward simulation: t ⊆ s
            for (obs, t_next) in coalgebra.transitions_from(t) {
                let matched = coalgebra
                    .transitions_from(s)
                    .into_iter()
                    .any(|(o2, s_next)| o2 == obs && self.contains(s_next, t_next));
                if !matched {
                    return false;
                }
            }
        }
        true
    }
}

// ── Greatest fixed-point approximation ───────────────────────────────────────

/// Compute an approximation of the greatest bisimulation on `coalgebra` by
/// iterative subset refinement.
///
/// Starts with the total relation (all pairs) and removes pairs that violate
/// the bisimulation condition until a fixed point is reached, up to
/// `max_iters` refinement steps.
pub fn greatest_fixed_point_approx<S: Clone + Eq>(
    coalgebra: &CoalgebraMap<S, String>,
    max_iters: usize,
) -> BisimulationRelation<S> {
    // Collect all states
    let mut states: Vec<S> = Vec::new();
    for (s, _, d) in &coalgebra.transitions {
        if !states.contains(s) {
            states.push(s.clone());
        }
        if !states.contains(d) {
            states.push(d.clone());
        }
    }

    // Start with the total relation
    let mut relation = BisimulationRelation::new();
    for s in &states {
        for t in &states {
            relation.add_pair(s.clone(), t.clone());
        }
    }

    for _ in 0..max_iters {
        let mut next = BisimulationRelation::new();
        for (s, t) in &relation.pairs {
            // Check if (s, t) satisfies the bisimulation condition w.r.t. `relation`
            let s_ok = coalgebra.transitions_from(s).iter().all(|(obs, s_next)| {
                coalgebra
                    .transitions_from(t)
                    .iter()
                    .any(|(o2, t_next)| *o2 == *obs && relation.contains(s_next, t_next))
            });
            let t_ok = coalgebra.transitions_from(t).iter().all(|(obs, t_next)| {
                coalgebra
                    .transitions_from(s)
                    .iter()
                    .any(|(o2, s_next)| *o2 == *obs && relation.contains(s_next, t_next))
            });
            if s_ok && t_ok {
                next.add_pair(s.clone(), t.clone());
            }
        }
        if next.pairs.len() == relation.pairs.len() {
            return next; // fixed point reached
        }
        relation = next;
    }
    relation
}

/// Try to prove that states `s1` and `s2` are bisimilar w.r.t. `coalgebra`.
///
/// Returns a `CoinductiveProof` if a valid bisimulation containing `(s1, s2)`
/// is found, or `None` if the iterative approximation fails to find one.
pub fn prove_bisimilar<S: Clone + Eq + std::fmt::Debug>(
    s1: S,
    s2: S,
    coalgebra: &CoalgebraMap<S, String>,
) -> Option<CoinductiveProof<S>> {
    let max_iters = 100;
    let gfp = greatest_fixed_point_approx(coalgebra, max_iters);
    let mut progress = Vec::new();
    progress.push(format!(
        "Computed GFP with {} pairs after up to {} iterations",
        gfp.pairs.len(),
        max_iters
    ));
    if gfp.contains(&s1, &s2) {
        progress.push(format!(
            "Found ({:?}, {:?}) in the greatest bisimulation",
            s1, s2
        ));
        Some(CoinductiveProof {
            relation: gfp,
            progress,
        })
    } else {
        None
    }
}

// ── GreibachNormalForm ────────────────────────────────────────────────────────

impl GreibachNormalForm {
    /// Create a GNF grammar from a list of rules `(lhs, terminal, rhs)`.
    pub fn new(rules: Vec<(String, char, Vec<String>)>) -> Self {
        GreibachNormalForm { rules }
    }

    /// Return all rules for non-terminal `nt`.
    pub fn rules_for(&self, nt: &str) -> Vec<(char, &[String])> {
        self.rules
            .iter()
            .filter(|(lhs, _, _)| lhs == nt)
            .map(|(_, terminal, rhs)| (*terminal, rhs.as_slice()))
            .collect()
    }

    /// Check whether the string `input` is derivable from non-terminal `start`.
    ///
    /// Uses a simple top-down recursive descent (exponential in general, but
    /// fine for small grammars in tests).
    pub fn derives(&self, start: &str, input: &[char]) -> bool {
        self.derives_stack(vec![start.to_string()], input)
    }

    fn derives_stack(&self, stack: Vec<String>, input: &[char]) -> bool {
        if stack.is_empty() {
            return input.is_empty();
        }
        if input.is_empty() {
            return false;
        }
        let top = &stack[0];
        let rest_stack = stack[1..].to_vec();
        for (terminal, rhs) in self.rules_for(top) {
            if terminal == input[0] {
                let mut new_stack: Vec<String> = rhs.to_vec();
                new_stack.extend_from_slice(&rest_stack);
                if self.derives_stack(new_stack, &input[1..]) {
                    return true;
                }
            }
        }
        false
    }
}

// ── Codata ────────────────────────────────────────────────────────────────────

impl<F: 'static> Codata<F> {
    /// Construct a `Codata` from an unfolding thunk.
    pub fn new(unfold: impl Fn() -> F + 'static) -> Self {
        Codata {
            unfold: Box::new(unfold),
        }
    }

    /// Unfold one layer of the codata.
    pub fn unfold(&self) -> F {
        (self.unfold)()
    }
}

// ── fibonacci_stream ──────────────────────────────────────────────────────────

/// Produce the Fibonacci stream `0, 1, 1, 2, 3, 5, 8, ...` as a `LazyStream<u64>`.
///
/// The stream is computed eagerly for `n` elements and stored as a prefix with
/// a zero-length cycle; callers that need truly infinite access should call
/// `fibonacci_stream_n` with a sufficient count.
pub fn fibonacci_stream() -> LazyStream<u64> {
    // Compute enough Fibonacci numbers to populate both prefix and cycle.
    // We store 94 values (the full u64-range Fibonacci sequence without overflow).
    let mut fibs = Vec::with_capacity(94);
    let (mut a, mut b) = (0u64, 1u64);
    while let Some(next) = a.checked_add(b) {
        fibs.push(a);
        a = b;
        b = next;
        if fibs.len() >= 94 {
            break;
        }
    }
    fibs.push(a); // last non-overflowing value
                  // Represent as prefix + empty cycle (finite model of infinite stream)
                  // For `nth` purposes, out-of-range accesses will repeat the last element.
    LazyStream::new(fibs, Vec::new())
}

/// Fibonacci stream with a specific `n`-element prefix; requests beyond `n`
/// elements repeat the last value.
pub fn fibonacci_stream_n(n: usize) -> Vec<u64> {
    fibonacci_stream().take(n)
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── LazyStream tests ──────────────────────────────────────────────────────

    #[test]
    fn constant_stream_take() {
        let s = LazyStream::constant(42u32);
        let v = s.take(5);
        assert_eq!(v, vec![42, 42, 42, 42, 42]);
    }

    #[test]
    fn cycle_stream() {
        let s = LazyStream::cycle(vec![1u32, 2, 3]);
        assert_eq!(s.take(7), vec![1, 2, 3, 1, 2, 3, 1]);
    }

    #[test]
    fn nth_within_prefix() {
        let s = LazyStream::new(vec![10u32, 20, 30], vec![1, 2]);
        assert_eq!(s.nth(0), 10);
        assert_eq!(s.nth(1), 20);
        assert_eq!(s.nth(2), 30);
    }

    #[test]
    fn nth_into_cycle() {
        let s = LazyStream::new(vec![10u32], vec![1, 2, 3]);
        assert_eq!(s.nth(1), 1);
        assert_eq!(s.nth(2), 2);
        assert_eq!(s.nth(3), 3);
        assert_eq!(s.nth(4), 1); // wraps
    }

    #[test]
    fn take_empty() {
        let s = LazyStream::constant(0u32);
        assert_eq!(s.take(0), Vec::<u32>::new());
    }

    #[test]
    fn map_stream() {
        let s = LazyStream::cycle(vec![1u32, 2, 3]);
        let doubled = s.map(|x| x * 2);
        assert_eq!(doubled.take(6), vec![2, 4, 6, 2, 4, 6]);
    }

    #[test]
    fn zip_streams_same_cycle() {
        let s1 = LazyStream::cycle(vec![1u32, 2, 3]);
        let s2 = LazyStream::cycle(vec![10u32, 20, 30]);
        let sums = s1.zip(&s2, |a, b| a + b);
        assert_eq!(sums.take(6), vec![11, 22, 33, 11, 22, 33]);
    }

    #[test]
    fn zip_streams_different_cycles() {
        let s1 = LazyStream::cycle(vec![1u32, 2]);
        let s2 = LazyStream::cycle(vec![10u32, 20, 30]);
        let sums = s1.zip(&s2, |a, b| a + b);
        // LCM(2,3) = 6, so cycle has 6 elements
        let v = sums.take(6);
        assert_eq!(v[0], 11); // 1+10
        assert_eq!(v[1], 22); // 2+20
        assert_eq!(v[2], 31); // 1+30
        assert_eq!(v[3], 12); // 2+10
        assert_eq!(v[4], 21); // 1+20
        assert_eq!(v[5], 32); // 2+30
    }

    #[test]
    fn stream_with_prefix_and_cycle() {
        let s = LazyStream::new(vec![0u32, 1], vec![2, 3]);
        assert_eq!(s.take(8), vec![0, 1, 2, 3, 2, 3, 2, 3]);
    }

    #[test]
    fn to_approx_depth_0() {
        let s = LazyStream::constant(5u32);
        let a = s.to_approx(0);
        assert!(matches!(a, StreamApprox::Nil));
    }

    #[test]
    fn to_approx_depth_3() {
        let s = LazyStream::cycle(vec![1u32, 2, 3]);
        let a = s.to_approx(3);
        if let StreamApprox::Cons(node) = a {
            assert_eq!(node.head, 1);
            if let StreamApprox::Cons(n2) = *node.tail {
                assert_eq!(n2.head, 2);
            } else {
                panic!("expected Cons at depth 2");
            }
        } else {
            panic!("expected Cons at depth 1");
        }
    }

    // ── Fibonacci stream tests ────────────────────────────────────────────────

    #[test]
    fn fibonacci_first_10() {
        let fibs = fibonacci_stream_n(10);
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn fibonacci_0th_element() {
        let s = fibonacci_stream();
        assert_eq!(s.nth(0), 0);
    }

    #[test]
    fn fibonacci_1st_element() {
        let s = fibonacci_stream();
        assert_eq!(s.nth(1), 1);
    }

    #[test]
    fn fibonacci_map() {
        let s = fibonacci_stream();
        let doubled: Vec<u64> = s.map(|x| x * 2).take(5);
        assert_eq!(doubled, vec![0, 2, 2, 4, 6]);
    }

    // ── BisimulationRelation tests ────────────────────────────────────────────

    #[test]
    fn bisim_check_trivial_identity() {
        // Two identical deterministic processes are bisimilar
        let mut coalgebra: CoalgebraMap<u32, String> = CoalgebraMap::new();
        coalgebra.add_transition(0, "a".to_string(), 0);
        coalgebra.add_transition(1, "a".to_string(), 1);

        let mut rel: BisimulationRelation<u32> = BisimulationRelation::new();
        rel.add_pair(0, 1);
        rel.add_pair(0, 0);
        rel.add_pair(1, 1);

        assert!(rel.check(&coalgebra));
    }

    #[test]
    fn bisim_check_invalid() {
        // States with different observations are NOT bisimilar
        let mut coalgebra: CoalgebraMap<u32, String> = CoalgebraMap::new();
        coalgebra.add_transition(0, "a".to_string(), 0);
        coalgebra.add_transition(1, "b".to_string(), 1);

        let mut rel: BisimulationRelation<u32> = BisimulationRelation::new();
        rel.add_pair(0, 1);

        assert!(!rel.check(&coalgebra));
    }

    #[test]
    fn bisim_relation_contains() {
        let mut rel: BisimulationRelation<i32> = BisimulationRelation::new();
        rel.add_pair(1, 2);
        assert!(rel.contains(&1, &2));
        assert!(!rel.contains(&2, &1));
    }

    // ── Greatest fixed-point tests ────────────────────────────────────────────

    #[test]
    fn gfp_simple_two_state() {
        // Two states both looping on "a" should be in the GFP
        let mut coalgebra: CoalgebraMap<u32, String> = CoalgebraMap::new();
        coalgebra.add_transition(0, "a".to_string(), 0);
        coalgebra.add_transition(1, "a".to_string(), 1);

        let gfp = greatest_fixed_point_approx(&coalgebra, 20);
        assert!(gfp.contains(&0, &1));
        assert!(gfp.contains(&1, &0));
    }

    #[test]
    fn gfp_distinguishable_states_not_bisimilar() {
        let mut coalgebra: CoalgebraMap<u32, String> = CoalgebraMap::new();
        coalgebra.add_transition(0, "a".to_string(), 0);
        coalgebra.add_transition(1, "b".to_string(), 1);

        let gfp = greatest_fixed_point_approx(&coalgebra, 20);
        assert!(!gfp.contains(&0, &1));
    }

    #[test]
    fn prove_bisimilar_success() {
        let mut coalgebra: CoalgebraMap<&str, String> = CoalgebraMap::new();
        coalgebra.add_transition("p", "tick".to_string(), "p");
        coalgebra.add_transition("q", "tick".to_string(), "q");

        let proof = prove_bisimilar("p", "q", &coalgebra);
        assert!(proof.is_some());
        let proof = proof.expect("should be Some");
        assert!(!proof.progress.is_empty());
        assert!(proof.relation.contains(&"p", &"q"));
    }

    #[test]
    fn prove_bisimilar_failure() {
        let mut coalgebra: CoalgebraMap<&str, String> = CoalgebraMap::new();
        coalgebra.add_transition("p", "a".to_string(), "p");
        coalgebra.add_transition("q", "b".to_string(), "q");

        let proof = prove_bisimilar("p", "q", &coalgebra);
        assert!(proof.is_none());
    }

    // ── GreibachNormalForm tests ──────────────────────────────────────────────

    #[test]
    fn gnf_simple_grammar() {
        // S → a S | a
        // Represented in GNF: S → a S (rhs=[S]), S → a (rhs=[])
        let gnf = GreibachNormalForm::new(vec![
            ("S".to_string(), 'a', vec!["S".to_string()]),
            ("S".to_string(), 'a', vec![]),
        ]);
        // "aaa" should be derivable
        assert!(gnf.derives("S", &['a', 'a', 'a']));
        // "b" should not be derivable
        assert!(!gnf.derives("S", &['b']));
        // empty string should not be derivable (GNF always consumes at least one char)
        assert!(!gnf.derives("S", &[]));
    }

    #[test]
    fn gnf_rules_for() {
        let gnf = GreibachNormalForm::new(vec![
            ("A".to_string(), 'x', vec!["B".to_string()]),
            ("A".to_string(), 'y', vec![]),
            ("B".to_string(), 'z', vec![]),
        ]);
        let a_rules = gnf.rules_for("A");
        assert_eq!(a_rules.len(), 2);
    }

    // ── Codata tests ──────────────────────────────────────────────────────────

    #[test]
    fn codata_unfold() {
        let c = Codata::new(|| 42u32);
        assert_eq!(c.unfold(), 42);
    }

    #[test]
    fn codata_unfold_returns_constant() {
        // Codata<u32> with a constant unfolding thunk
        let c: Codata<u32> = Codata::new(|| 99u32);
        // Calling unfold multiple times always gives the same value
        assert_eq!(c.unfold(), 99);
        assert_eq!(c.unfold(), 99);
    }

    // ── Additional edge-case tests ────────────────────────────────────────────

    #[test]
    fn lcm_helper() {
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(5, 5), 5);
        assert_eq!(lcm(0, 5), 0);
    }

    #[test]
    fn gcd_helper() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 3), 1);
        assert_eq!(gcd(0, 5), 5);
    }

    #[test]
    fn bisim_empty_relation_is_valid() {
        let coalgebra: CoalgebraMap<u32, String> = CoalgebraMap::new();
        let rel: BisimulationRelation<u32> = BisimulationRelation::new();
        // Empty relation vacuously satisfies bisimulation
        assert!(rel.check(&coalgebra));
    }

    #[test]
    fn coalgebra_transitions_from() {
        let mut c: CoalgebraMap<u32, String> = CoalgebraMap::new();
        c.add_transition(0, "a".to_string(), 1);
        c.add_transition(0, "b".to_string(), 2);
        c.add_transition(1, "c".to_string(), 0);
        let from_0 = c.transitions_from(&0);
        assert_eq!(from_0.len(), 2);
    }
}
