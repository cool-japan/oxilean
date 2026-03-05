//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{BTreeSet, HashSet};

use super::functions::*;

/// A semiring-weighted NFA where weights are generic (here: f64 for generality).
/// Computes: for each word w, sum over all accepting runs of product of weights.
#[derive(Debug, Clone)]
pub struct WeightedAutomaton {
    /// Number of states.
    pub num_states: usize,
    /// Alphabet.
    pub alphabet: Vec<char>,
    /// Weighted transitions: wtrans[q][a] = list of (successor, weight).
    pub wtrans: Vec<Vec<Vec<(usize, f64)>>>,
    /// Initial weight for each state (1.0 for initial, 0.0 otherwise).
    pub init_vec: Vec<f64>,
    /// Final weight for each state.
    pub final_vec: Vec<f64>,
}
impl WeightedAutomaton {
    /// Create a new weighted automaton with zero initial and final weights.
    pub fn new(n: usize, alphabet: Vec<char>) -> Self {
        let k = alphabet.len();
        Self {
            num_states: n,
            wtrans: vec![vec![vec![]; k]; n],
            init_vec: vec![0.0; n],
            final_vec: vec![0.0; n],
            alphabet,
        }
    }
    /// Add a weighted transition q --a/w--> r.
    pub fn add_transition(&mut self, q: usize, a: char, r: usize, w: f64) {
        let idx = self
            .alphabet
            .iter()
            .position(|&c| c == a)
            .expect("bad symbol");
        self.wtrans[q][idx].push((r, w));
    }
    /// Compute the total weight of a word (sum over all paths, product of weights).
    /// This implements the standard (sum, product) semiring evaluation.
    pub fn evaluate(&self, word: &str) -> f64 {
        let mut current = self.init_vec.clone();
        for c in word.chars() {
            let idx = match self.alphabet.iter().position(|&x| x == c) {
                Some(i) => i,
                None => return 0.0,
            };
            let mut next = vec![0.0; self.num_states];
            for q in 0..self.num_states {
                if current[q] == 0.0 {
                    continue;
                }
                for &(r, w) in &self.wtrans[q][idx] {
                    next[r] += current[q] * w;
                }
            }
            current = next;
        }
        current
            .iter()
            .zip(self.final_vec.iter())
            .map(|(c, f)| c * f)
            .sum()
    }
    /// Boolean membership: a word is accepted iff its weight > 0.
    pub fn accepts(&self, word: &str) -> bool {
        self.evaluate(word) > 0.0
    }
}
/// A parity game node.
#[derive(Debug, Clone)]
pub struct ParityGameNode {
    /// Priority of this node.
    pub priority: u32,
    /// Owner: 0 = Even player, 1 = Odd player.
    pub owner: u8,
    /// Successors.
    pub successors: Vec<usize>,
}
/// A simple 1-clock timed automaton edge with guard and reset.
#[derive(Debug, Clone)]
pub struct TimedEdge {
    /// Source state.
    pub from: usize,
    /// Target state.
    pub to: usize,
    /// Symbol label.
    pub symbol: char,
    /// Guard: clock value must satisfy lo <= x <= hi.
    pub guard_lo: f64,
    pub guard_hi: f64,
    /// If true, reset clock to 0 after taking this edge.
    pub reset: bool,
}
/// A clock region for a single clock x with bound c_max.
/// The regions are: x = 0, 0 < x < 1, x = 1, …, c_max-1 < x < c_max, x ≥ c_max.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClockRegion {
    /// x = k
    Exact(u32),
    /// k < x < k+1
    Open(u32),
    /// x ≥ c_max
    Above,
}
impl ClockRegion {
    /// Compute the region for a value given c_max.
    pub fn of(val: f64, c_max: u32) -> Self {
        if val >= c_max as f64 {
            return ClockRegion::Above;
        }
        let floor = val.floor() as u32;
        if (val - floor as f64).abs() < 1e-12 {
            ClockRegion::Exact(floor)
        } else {
            ClockRegion::Open(floor)
        }
    }
    /// Advance time by epsilon (time successor).
    pub fn time_succ(self) -> Self {
        match self {
            ClockRegion::Exact(k) => ClockRegion::Open(k),
            ClockRegion::Open(k) => ClockRegion::Exact(k + 1),
            ClockRegion::Above => ClockRegion::Above,
        }
    }
    /// Reset clock to 0.
    pub fn reset(self) -> Self {
        ClockRegion::Exact(0)
    }
}
/// A nondeterministic Büchi automaton over a character alphabet.
#[derive(Debug, Clone)]
pub struct BuchiNba {
    /// Number of states.
    pub num_states: usize,
    /// Alphabet symbols.
    pub alphabet: Vec<char>,
    /// Transition relation: delta[q][a] = set of successor states.
    pub delta: Vec<Vec<BTreeSet<usize>>>,
    /// Initial state (single for simplicity; generalizes easily).
    pub init: usize,
    /// Set of accepting (Büchi) states.
    pub accepting: BTreeSet<usize>,
}
impl BuchiNba {
    /// Create a new NBA with `n` states over the given alphabet.
    pub fn new(n: usize, alphabet: Vec<char>) -> Self {
        let k = alphabet.len();
        Self {
            num_states: n,
            delta: vec![vec![BTreeSet::new(); k]; n],
            init: 0,
            accepting: BTreeSet::new(),
            alphabet,
        }
    }
    /// Add a transition q -a-> r.
    pub fn add_transition(&mut self, q: usize, a: char, r: usize) {
        let idx = self
            .alphabet
            .iter()
            .position(|&c| c == a)
            .expect("symbol not in alphabet");
        self.delta[q][idx].insert(r);
    }
    /// Mark a state as accepting.
    pub fn set_accepting(&mut self, q: usize) {
        self.accepting.insert(q);
    }
    /// Check if the automaton accepts a finite lasso word (prefix, cycle).
    /// The lasso u·vω is accepted iff there's a run over u·v that visits
    /// an accepting state somewhere in v and returns to the same state.
    pub fn accepts_lasso(&self, prefix: &str, cycle: &str) -> bool {
        if cycle.is_empty() {
            return false;
        }
        let mut current: BTreeSet<usize> = BTreeSet::new();
        current.insert(self.init);
        for c in prefix.chars() {
            let idx = match self.alphabet.iter().position(|&x| x == c) {
                Some(i) => i,
                None => return false,
            };
            let mut next = BTreeSet::new();
            for &q in &current {
                for &r in &self.delta[q][idx] {
                    next.insert(r);
                }
            }
            current = next;
        }
        let mut cycle_starts = current.clone();
        {
            let mut after_one = current.clone();
            for c in cycle.chars() {
                let idx = match self.alphabet.iter().position(|&x| x == c) {
                    Some(i) => i,
                    None => {
                        after_one.clear();
                        break;
                    }
                };
                let mut next = BTreeSet::new();
                for &q in &after_one {
                    for &r in &self.delta[q][idx] {
                        next.insert(r);
                    }
                }
                after_one = next;
            }
            cycle_starts.extend(after_one.iter());
        }
        for &q0 in &cycle_starts {
            let mut states = BTreeSet::new();
            states.insert(q0);
            let mut saw_accept = false;
            for c in cycle.chars() {
                let idx = match self.alphabet.iter().position(|&x| x == c) {
                    Some(i) => i,
                    None => break,
                };
                let mut next = BTreeSet::new();
                for &q in &states {
                    for &r in &self.delta[q][idx] {
                        next.insert(r);
                        if self.accepting.contains(&r) {
                            saw_accept = true;
                        }
                    }
                }
                states = next;
            }
            if saw_accept && states.contains(&q0) {
                return true;
            }
        }
        false
    }
    /// Decide emptiness via nested DFS (de-facto on finite prefix/cycle pairs).
    /// Returns true iff the language is empty (no accepting lasso reachable).
    pub fn is_empty(&self) -> bool {
        let mut visited = vec![false; self.num_states];
        let mut stack = vec![self.init];
        visited[self.init] = true;
        let mut found_accept = false;
        while let Some(q) = stack.pop() {
            if self.accepting.contains(&q) {
                found_accept = true;
                if self.can_reach(q, q) {
                    return false;
                }
            }
            for idx in 0..self.alphabet.len() {
                for &r in &self.delta[q][idx] {
                    if !visited[r] {
                        visited[r] = true;
                        stack.push(r);
                    }
                }
            }
        }
        if !found_accept {
            return true;
        }
        for &acc in &self.accepting {
            if visited[acc] && self.can_reach(acc, acc) {
                return false;
            }
        }
        true
    }
    /// Check if state `target` is reachable from state `src`.
    fn can_reach(&self, src: usize, target: usize) -> bool {
        let mut visited = vec![false; self.num_states];
        let mut stack = vec![src];
        visited[src] = true;
        while let Some(q) = stack.pop() {
            for idx in 0..self.alphabet.len() {
                for &r in &self.delta[q][idx] {
                    if r == target {
                        return true;
                    }
                    if !visited[r] {
                        visited[r] = true;
                        stack.push(r);
                    }
                }
            }
        }
        false
    }
}
/// Priority: each state has an integer priority (even = accepting).
#[derive(Debug, Clone)]
pub struct ParityAut {
    /// Number of states.
    pub num_states: usize,
    /// Alphabet.
    pub alphabet: Vec<char>,
    /// Transition function (deterministic for simplicity).
    pub delta: Vec<Vec<usize>>,
    /// Initial state.
    pub init: usize,
    /// Priority of each state.
    pub priority: Vec<u32>,
}
impl ParityAut {
    /// Create a new deterministic parity automaton.
    pub fn new(n: usize, alphabet: Vec<char>, priority: Vec<u32>) -> Self {
        let k = alphabet.len();
        Self {
            num_states: n,
            delta: vec![vec![0; k]; n],
            init: 0,
            priority,
            alphabet,
        }
    }
    /// Set a transition q -a-> r.
    pub fn set_transition(&mut self, q: usize, a: char, r: usize) {
        let idx = self
            .alphabet
            .iter()
            .position(|&c| c == a)
            .expect("symbol not in alphabet");
        self.delta[q][idx] = r;
    }
    /// Check acceptance on a lasso (prefix + cycle^ω).
    /// Accepted iff min priority in cycle is even.
    pub fn accepts_lasso(&self, prefix: &str, cycle: &str) -> bool {
        if cycle.is_empty() {
            return false;
        }
        let mut q = self.init;
        for c in prefix.chars() {
            let idx = self
                .alphabet
                .iter()
                .position(|&x| x == c)
                .expect("bad symbol");
            q = self.delta[q][idx];
        }
        let mut min_prio = u32::MAX;
        for c in cycle.chars() {
            let idx = self
                .alphabet
                .iter()
                .position(|&x| x == c)
                .expect("bad symbol");
            q = self.delta[q][idx];
            let p = self.priority[q];
            if p < min_prio {
                min_prio = p;
            }
        }
        min_prio % 2 == 0
    }
}
/// Propositional LTL formula over named atomic propositions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LtlFormula {
    /// Atomic proposition.
    Atom(String),
    /// Logical true.
    True,
    /// Logical false.
    False,
    /// Negation.
    Not(Box<LtlFormula>),
    /// Conjunction.
    And(Box<LtlFormula>, Box<LtlFormula>),
    /// Disjunction.
    Or(Box<LtlFormula>, Box<LtlFormula>),
    /// Implication.
    Impl(Box<LtlFormula>, Box<LtlFormula>),
    /// Next.
    Next(Box<LtlFormula>),
    /// Globally (always).
    Globally(Box<LtlFormula>),
    /// Eventually.
    Finally(Box<LtlFormula>),
    /// Until.
    Until(Box<LtlFormula>, Box<LtlFormula>),
    /// Release (dual of Until).
    Release(Box<LtlFormula>, Box<LtlFormula>),
}
impl LtlFormula {
    /// Push negation inward (negation normal form).
    pub fn nnf(self) -> Self {
        match self {
            LtlFormula::Not(box_f) => match *box_f {
                LtlFormula::Not(inner) => inner.nnf(),
                LtlFormula::And(a, b) => LtlFormula::Or(
                    Box::new(LtlFormula::Not(a).nnf()),
                    Box::new(LtlFormula::Not(b).nnf()),
                ),
                LtlFormula::Or(a, b) => LtlFormula::And(
                    Box::new(LtlFormula::Not(a).nnf()),
                    Box::new(LtlFormula::Not(b).nnf()),
                ),
                LtlFormula::Globally(f) => LtlFormula::Finally(Box::new(LtlFormula::Not(f).nnf())),
                LtlFormula::Finally(f) => LtlFormula::Globally(Box::new(LtlFormula::Not(f).nnf())),
                LtlFormula::Until(a, b) => LtlFormula::Release(
                    Box::new(LtlFormula::Not(a).nnf()),
                    Box::new(LtlFormula::Not(b).nnf()),
                ),
                LtlFormula::Release(a, b) => LtlFormula::Until(
                    Box::new(LtlFormula::Not(a).nnf()),
                    Box::new(LtlFormula::Not(b).nnf()),
                ),
                other => LtlFormula::Not(Box::new(other.nnf())),
            },
            LtlFormula::And(a, b) => LtlFormula::And(Box::new(a.nnf()), Box::new(b.nnf())),
            LtlFormula::Or(a, b) => LtlFormula::Or(Box::new(a.nnf()), Box::new(b.nnf())),
            LtlFormula::Impl(a, b) => {
                LtlFormula::Or(Box::new(LtlFormula::Not(a).nnf()), Box::new(b.nnf()))
            }
            LtlFormula::Next(f) => LtlFormula::Next(Box::new(f.nnf())),
            LtlFormula::Globally(f) => LtlFormula::Globally(Box::new(f.nnf())),
            LtlFormula::Finally(f) => LtlFormula::Finally(Box::new(f.nnf())),
            LtlFormula::Until(a, b) => LtlFormula::Until(Box::new(a.nnf()), Box::new(b.nnf())),
            LtlFormula::Release(a, b) => LtlFormula::Release(Box::new(a.nnf()), Box::new(b.nnf())),
            other => other,
        }
    }
    /// Evaluate on a finite trace (returns value at position 0).
    /// Only handles finite segments; infinite properties approximated.
    pub fn eval_finite(&self, trace: &[HashSet<String>], pos: usize) -> bool {
        match self {
            LtlFormula::True => true,
            LtlFormula::False => false,
            LtlFormula::Atom(p) => trace
                .get(pos)
                .map(|s| s.contains(p.as_str()))
                .unwrap_or(false),
            LtlFormula::Not(f) => !f.eval_finite(trace, pos),
            LtlFormula::And(a, b) => a.eval_finite(trace, pos) && b.eval_finite(trace, pos),
            LtlFormula::Or(a, b) => a.eval_finite(trace, pos) || b.eval_finite(trace, pos),
            LtlFormula::Impl(a, b) => !a.eval_finite(trace, pos) || b.eval_finite(trace, pos),
            LtlFormula::Next(f) => f.eval_finite(trace, pos + 1),
            LtlFormula::Finally(f) => (pos..trace.len()).any(|i| f.eval_finite(trace, i)),
            LtlFormula::Globally(f) => (pos..trace.len()).all(|i| f.eval_finite(trace, i)),
            LtlFormula::Until(a, b) => (pos..trace.len())
                .any(|j| b.eval_finite(trace, j) && (pos..j).all(|i| a.eval_finite(trace, i))),
            LtlFormula::Release(a, b) => {
                (pos..trace.len()).all(|i| b.eval_finite(trace, i))
                    || (pos..trace.len()).any(|j| {
                        a.eval_finite(trace, j) && (pos..=j).all(|i| b.eval_finite(trace, i))
                    })
            }
        }
    }
}
/// A 1-clock timed automaton checker.
/// Checks reachability of a target state using region-based abstraction.
#[derive(Debug, Clone)]
pub struct TimedAutomatonChecker {
    /// Number of states.
    pub num_states: usize,
    /// Edges.
    pub edges: Vec<TimedEdge>,
    /// Initial state.
    pub init: usize,
    /// Maximum clock constant (for region construction).
    pub c_max: u32,
}
impl TimedAutomatonChecker {
    /// Create a new timed automaton checker.
    pub fn new(num_states: usize, init: usize, c_max: u32) -> Self {
        Self {
            num_states,
            edges: vec![],
            init,
            c_max,
        }
    }
    /// Add a timed edge.
    pub fn add_edge(
        &mut self,
        from: usize,
        to: usize,
        symbol: char,
        guard_lo: f64,
        guard_hi: f64,
        reset: bool,
    ) {
        self.edges.push(TimedEdge {
            from,
            to,
            symbol,
            guard_lo,
            guard_hi,
            reset,
        });
    }
    /// Check if target state is reachable using symbolic region exploration.
    /// Returns true if target is reachable.
    pub fn is_reachable(&self, target: usize) -> bool {
        let mut visited: HashSet<(usize, ClockRegion)> = HashSet::new();
        let mut worklist = vec![(self.init, ClockRegion::Exact(0))];
        visited.insert((self.init, ClockRegion::Exact(0)));
        while let Some((state, region)) = worklist.pop() {
            if state == target {
                return true;
            }
            let succ_region = region.time_succ();
            if visited.insert((state, succ_region)) {
                worklist.push((state, succ_region));
            }
            for edge in &self.edges {
                if edge.from != state {
                    continue;
                }
                let satisfies = match region {
                    ClockRegion::Exact(k) => {
                        let v = k as f64;
                        v >= edge.guard_lo && v <= edge.guard_hi
                    }
                    ClockRegion::Open(k) => {
                        let mid = k as f64 + 0.5;
                        mid >= edge.guard_lo && mid <= edge.guard_hi
                    }
                    ClockRegion::Above => edge.guard_lo <= self.c_max as f64,
                };
                if !satisfies {
                    continue;
                }
                let new_region = if edge.reset {
                    ClockRegion::Exact(0)
                } else {
                    region
                };
                if visited.insert((edge.to, new_region)) {
                    worklist.push((edge.to, new_region));
                }
            }
        }
        false
    }
}
/// A weighted automaton over the (max, +) tropical semiring.
/// Weight of a word w = max over accepting runs of sum of transition weights.
#[derive(Debug, Clone)]
pub struct WeightedAut {
    /// Number of states.
    pub num_states: usize,
    /// Alphabet.
    pub alphabet: Vec<char>,
    /// Transition weights: weights[q][a] = Vec<(successor, weight)>.
    pub weights: Vec<Vec<Vec<(usize, i64)>>>,
    /// Initial weight vector.
    pub init_weights: Vec<i64>,
    /// Final weight vector.
    pub final_weights: Vec<i64>,
}
impl WeightedAut {
    /// Create a new weighted automaton (all weights -∞ initially).
    pub fn new(n: usize, alphabet: Vec<char>) -> Self {
        let k = alphabet.len();
        let neg_inf = i64::MIN / 2;
        Self {
            num_states: n,
            weights: vec![vec![vec![]; k]; n],
            init_weights: vec![neg_inf; n],
            final_weights: vec![neg_inf; n],
            alphabet,
        }
    }
    /// Add a weighted transition.
    pub fn add_transition(&mut self, q: usize, a: char, r: usize, w: i64) {
        let idx = self
            .alphabet
            .iter()
            .position(|&c| c == a)
            .expect("bad symbol");
        self.weights[q][idx].push((r, w));
    }
    /// Compute the (max, +) weight of a word.
    pub fn run_weight(&self, word: &str) -> i64 {
        let neg_inf = i64::MIN / 2;
        let mut current = self.init_weights.clone();
        for c in word.chars() {
            let idx = self
                .alphabet
                .iter()
                .position(|&x| x == c)
                .unwrap_or(usize::MAX);
            if idx == usize::MAX {
                return neg_inf;
            }
            let mut next = vec![neg_inf; self.num_states];
            for q in 0..self.num_states {
                if current[q] <= neg_inf {
                    continue;
                }
                for &(r, w) in &self.weights[q][idx] {
                    let v = current[q].saturating_add(w);
                    if v > next[r] {
                        next[r] = v;
                    }
                }
            }
            current = next;
        }
        (0..self.num_states)
            .map(|q| {
                if current[q] <= neg_inf || self.final_weights[q] <= neg_inf {
                    neg_inf
                } else {
                    current[q].saturating_add(self.final_weights[q])
                }
            })
            .max()
            .unwrap_or(neg_inf)
    }
}
/// Simulates a Büchi automaton on a lasso (prefix + repeating cycle).
/// Tracks accepting states visited in the cycle.
#[derive(Debug, Clone)]
pub struct BuchiAutomatonSimulator {
    /// Number of states.
    pub num_states: usize,
    /// Alphabet.
    pub alphabet: Vec<char>,
    /// Transition relation: trans[q][a] = set of successor states.
    pub trans: Vec<Vec<BTreeSet<usize>>>,
    /// Set of initial states.
    pub initial_states: BTreeSet<usize>,
    /// Accepting state set (Büchi condition: visit infinitely often).
    pub accepting_states: BTreeSet<usize>,
}
impl BuchiAutomatonSimulator {
    /// Create a new Büchi automaton simulator with a single initial state 0.
    pub fn new(n: usize, alphabet: Vec<char>) -> Self {
        let k = alphabet.len();
        let mut initial_states = BTreeSet::new();
        initial_states.insert(0);
        Self {
            num_states: n,
            trans: vec![vec![BTreeSet::new(); k]; n],
            initial_states,
            accepting_states: BTreeSet::new(),
            alphabet,
        }
    }
    /// Add a transition q --a--> r.
    pub fn add_transition(&mut self, q: usize, a: char, r: usize) {
        let idx = self
            .alphabet
            .iter()
            .position(|&c| c == a)
            .expect("bad symbol");
        self.trans[q][idx].insert(r);
    }
    /// Mark a state as accepting.
    pub fn mark_accepting(&mut self, q: usize) {
        self.accepting_states.insert(q);
    }
    /// Run the automaton over a sequence of symbols, returning all reachable states.
    fn run_segment(&self, start_states: &BTreeSet<usize>, symbols: &str) -> BTreeSet<usize> {
        let mut current = start_states.clone();
        for c in symbols.chars() {
            let idx = match self.alphabet.iter().position(|&x| x == c) {
                Some(i) => i,
                None => return BTreeSet::new(),
            };
            let mut next = BTreeSet::new();
            for &q in &current {
                for &r in &self.trans[q][idx] {
                    next.insert(r);
                }
            }
            current = next;
        }
        current
    }
    /// Check Büchi acceptance on a lasso: prefix · cycle^ω.
    /// Accepted iff there exists a run that visits an accepting state in the cycle.
    pub fn accepts_lasso(&self, prefix: &str, cycle: &str) -> bool {
        if cycle.is_empty() {
            return false;
        }
        let after_prefix = self.run_segment(&self.initial_states, prefix);
        if after_prefix.is_empty() {
            return false;
        }
        for &q0 in &after_prefix {
            let mut start = BTreeSet::new();
            start.insert(q0);
            let mut visiting = start.clone();
            let mut saw_accept = false;
            for c in cycle.chars() {
                let idx = match self.alphabet.iter().position(|&x| x == c) {
                    Some(i) => i,
                    None => {
                        visiting.clear();
                        break;
                    }
                };
                let mut next = BTreeSet::new();
                for &q in &visiting {
                    for &r in &self.trans[q][idx] {
                        next.insert(r);
                        if self.accepting_states.contains(&r) {
                            saw_accept = true;
                        }
                    }
                }
                visiting = next;
            }
            if saw_accept && visiting.contains(&q0) {
                return true;
            }
        }
        false
    }
    /// Compute the set of states reachable from initial states.
    pub fn reachable_states(&self) -> BTreeSet<usize> {
        let mut visited = BTreeSet::new();
        let mut stack: Vec<usize> = self.initial_states.iter().cloned().collect();
        for &q in &self.initial_states {
            visited.insert(q);
        }
        while let Some(q) = stack.pop() {
            for idx in 0..self.alphabet.len() {
                for &r in &self.trans[q][idx] {
                    if visited.insert(r) {
                        stack.push(r);
                    }
                }
            }
        }
        visited
    }
    /// Check emptiness: no accepting lasso is reachable.
    pub fn is_empty(&self) -> bool {
        let reachable = self.reachable_states();
        for &acc in &self.accepting_states {
            if !reachable.contains(&acc) {
                continue;
            }
            let mut visited = BTreeSet::new();
            let mut stack = vec![acc];
            visited.insert(acc);
            let mut found_cycle = false;
            while let Some(q) = stack.pop() {
                for idx in 0..self.alphabet.len() {
                    for &r in &self.trans[q][idx] {
                        if r == acc {
                            found_cycle = true;
                            break;
                        }
                        if visited.insert(r) {
                            stack.push(r);
                        }
                    }
                    if found_cycle {
                        break;
                    }
                }
                if found_cycle {
                    break;
                }
            }
            if found_cycle {
                return false;
            }
        }
        true
    }
}
/// A 1D elementary cellular automaton using a Wolfram rule number (0–255).
#[derive(Debug, Clone)]
pub struct CellularAutomataRule {
    /// Wolfram rule number (0–255).
    pub rule: u8,
}
impl CellularAutomataRule {
    /// Create a new elementary CA rule.
    pub fn new(rule: u8) -> Self {
        Self { rule }
    }
    /// Apply one step of the CA to a row of cells (periodic boundary conditions).
    pub fn step(&self, cells: &[bool]) -> Vec<bool> {
        let n = cells.len();
        if n == 0 {
            return vec![];
        }
        (0..n)
            .map(|i| {
                let left = cells[(i + n - 1) % n];
                let center = cells[i];
                let right = cells[(i + 1) % n];
                let pattern = ((left as u8) << 2) | ((center as u8) << 1) | (right as u8);
                (self.rule >> pattern) & 1 == 1
            })
            .collect()
    }
    /// Run `steps` iterations of the CA from initial configuration.
    pub fn run(&self, initial: &[bool], steps: usize) -> Vec<Vec<bool>> {
        let mut history = Vec::with_capacity(steps + 1);
        let mut current = initial.to_vec();
        history.push(current.clone());
        for _ in 0..steps {
            current = self.step(&current);
            history.push(current.clone());
        }
        history
    }
    /// Check if Rule 110 supports a specific output after k steps (used for universality test).
    pub fn rule_number(&self) -> u8 {
        self.rule
    }
}
/// A simple parity game solver using attractor computation.
/// Returns the winning set for the Even player (player 0).
#[derive(Debug, Clone)]
pub struct ParityGameSolver {
    /// Nodes of the game graph.
    pub nodes: Vec<ParityGameNode>,
}
impl ParityGameSolver {
    /// Create a new parity game with n nodes.
    pub fn new(n: usize) -> Self {
        Self {
            nodes: (0..n)
                .map(|_| ParityGameNode {
                    priority: 0,
                    owner: 0,
                    successors: vec![],
                })
                .collect(),
        }
    }
    /// Set node properties.
    pub fn set_node(&mut self, id: usize, priority: u32, owner: u8, successors: Vec<usize>) {
        self.nodes[id].priority = priority;
        self.nodes[id].owner = owner;
        self.nodes[id].successors = successors;
    }
    /// Compute the attractor for player `player` toward the set `target`.
    /// Returns all nodes from which `player` can force reaching `target`.
    pub fn attractor(&self, player: u8, target: &BTreeSet<usize>) -> BTreeSet<usize> {
        let mut attr = target.clone();
        let mut changed = true;
        while changed {
            changed = false;
            for (i, node) in self.nodes.iter().enumerate() {
                if attr.contains(&i) {
                    continue;
                }
                let in_attr = if node.owner == player {
                    node.successors.iter().any(|s| attr.contains(s))
                } else {
                    !node.successors.is_empty() && node.successors.iter().all(|s| attr.contains(s))
                };
                if in_attr {
                    attr.insert(i);
                    changed = true;
                }
            }
        }
        attr
    }
    /// Solve the parity game using the small progress measures (simplified: Zielonka recursive).
    /// Returns (W0, W1) where W0 is the winning set for Even player.
    pub fn solve(&self) -> (BTreeSet<usize>, BTreeSet<usize>) {
        let all_nodes: BTreeSet<usize> = (0..self.nodes.len()).collect();
        self.zielonka_solve(&all_nodes)
    }
    fn zielonka_solve(&self, domain: &BTreeSet<usize>) -> (BTreeSet<usize>, BTreeSet<usize>) {
        if domain.is_empty() {
            return (BTreeSet::new(), BTreeSet::new());
        }
        let max_prio = domain
            .iter()
            .map(|&i| self.nodes[i].priority)
            .max()
            .unwrap_or(0);
        let player = (max_prio % 2) as u8;
        let opponent = 1 - player;
        let u: BTreeSet<usize> = domain
            .iter()
            .filter(|&&i| self.nodes[i].priority == max_prio)
            .cloned()
            .collect();
        let attr_u = self.attractor_in(player, &u, domain);
        let sub_domain: BTreeSet<usize> = domain.difference(&attr_u).cloned().collect();
        let (mut w0_sub, mut w1_sub) = self.zielonka_solve(&sub_domain);
        let w_opp = if opponent == 0 { &w0_sub } else { &w1_sub };
        if w_opp.is_empty() {
            let w_player_full: BTreeSet<usize> = domain.iter().cloned().collect();
            if player == 0 {
                (w_player_full, BTreeSet::new())
            } else {
                (BTreeSet::new(), w_player_full)
            }
        } else {
            let attr_opp = self.attractor_in(opponent, w_opp, domain);
            let sub_domain2: BTreeSet<usize> = domain.difference(&attr_opp).cloned().collect();
            let (w0_sub2, w1_sub2) = self.zielonka_solve(&sub_domain2);
            if player == 0 {
                w1_sub = w1_sub2;
                w1_sub.extend(attr_opp.iter());
                (w0_sub2, w1_sub)
            } else {
                w0_sub = w0_sub2;
                w0_sub.extend(attr_opp.iter());
                (w0_sub, w1_sub2)
            }
        }
    }
    /// Restricted attractor computation within a subdomain.
    fn attractor_in(
        &self,
        player: u8,
        target: &BTreeSet<usize>,
        domain: &BTreeSet<usize>,
    ) -> BTreeSet<usize> {
        let mut attr: BTreeSet<usize> = target.intersection(domain).cloned().collect();
        let mut changed = true;
        while changed {
            changed = false;
            for &i in domain {
                if attr.contains(&i) {
                    continue;
                }
                let node = &self.nodes[i];
                let dom_succs: Vec<usize> = node
                    .successors
                    .iter()
                    .filter(|s| domain.contains(s))
                    .cloned()
                    .collect();
                let in_attr = if node.owner == player {
                    dom_succs.iter().any(|s| attr.contains(s))
                } else {
                    !dom_succs.is_empty() && dom_succs.iter().all(|s| attr.contains(s))
                };
                if in_attr {
                    attr.insert(i);
                    changed = true;
                }
            }
        }
        attr
    }
}
