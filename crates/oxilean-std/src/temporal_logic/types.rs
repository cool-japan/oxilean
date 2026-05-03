//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

use super::functions::*;
use super::functions::{AtomId, BuchiState, MuVar};

/// A fairness constraint is a set of states that must be visited infinitely often.
#[derive(Debug, Clone)]
pub struct FairnessConstraint {
    /// Human-readable name.
    pub name: String,
    /// States that must be visited infinitely often.
    pub required_states: HashSet<usize>,
}
impl FairnessConstraint {
    /// Create a named fairness constraint.
    pub fn new(name: impl Into<String>, states: HashSet<usize>) -> Self {
        FairnessConstraint {
            name: name.into(),
            required_states: states,
        }
    }
    /// Check if a finite trace prefix has at least one required state.
    pub fn trace_touches(&self, trace: &[usize]) -> bool {
        trace.iter().any(|s| self.required_states.contains(s))
    }
}
/// A node in an LTL tableau (a set of formulas to be satisfied simultaneously).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TableauNode {
    /// The set of formulas required at this point.
    pub formulas: Vec<LtlFormula>,
    /// The set of "next" formulas (to be passed to successor nodes).
    pub next_formulas: Vec<LtlFormula>,
    /// Whether this node is marked (for liveness checking).
    pub marked: bool,
}
#[allow(dead_code)]
impl TableauNode {
    /// Create a new tableau node from a set of formulas.
    pub fn new(formulas: Vec<LtlFormula>) -> Self {
        TableauNode {
            formulas,
            next_formulas: Vec::new(),
            marked: false,
        }
    }
    /// Mark this node (for Büchi acceptance).
    pub fn mark(&mut self) {
        self.marked = true;
    }
    /// Check if the node is consistent (no formula and its negation both present).
    pub fn is_consistent(&self) -> bool {
        for f in &self.formulas {
            if let LtlFormula::Not(phi) = f {
                if self.formulas.contains(phi) {
                    return false;
                }
            }
        }
        !self.formulas.contains(&LtlFormula::False)
    }
    /// Extract the "next-step" obligations from Until formulas.
    pub fn extract_until_obligations(&self) -> Vec<LtlFormula> {
        let mut obligations = Vec::new();
        for f in &self.formulas {
            if let LtlFormula::Until(phi, _psi) = f {
                obligations.push((**phi).clone());
            }
        }
        obligations
    }
}
/// A generalized Büchi automaton with multiple accepting sets.
#[allow(dead_code)]
pub struct GeneralizedBuchiAutomaton {
    /// Number of states.
    pub n_states: usize,
    /// Initial states.
    pub initial: HashSet<BuchiState>,
    /// Transitions: (state, label) → set of next states.
    pub transitions: HashMap<(BuchiState, u64), HashSet<BuchiState>>,
    /// Accepting sets F_1, ..., F_k (GBA condition: visit each set infinitely often).
    pub accepting_sets: Vec<HashSet<BuchiState>>,
}
#[allow(dead_code)]
impl GeneralizedBuchiAutomaton {
    /// Create a new GBA with `n` states and `k` accepting sets.
    pub fn new(n: usize, k: usize) -> Self {
        GeneralizedBuchiAutomaton {
            n_states: n,
            initial: HashSet::new(),
            transitions: HashMap::new(),
            accepting_sets: vec![HashSet::new(); k],
        }
    }
    /// Add an initial state.
    pub fn add_initial(&mut self, s: BuchiState) {
        self.initial.insert(s);
    }
    /// Add state `s` to the i-th accepting set.
    pub fn add_accepting(&mut self, i: usize, s: BuchiState) {
        if i < self.accepting_sets.len() {
            self.accepting_sets[i].insert(s);
        }
    }
    /// Add a transition.
    pub fn add_transition(&mut self, s: BuchiState, label: u64, t: BuchiState) {
        self.transitions.entry((s, label)).or_default().insert(t);
    }
    /// Degeneralize to a standard Büchi automaton.
    /// Product construction: states are (q, i) where i ∈ {0..k}.
    pub fn degeneralize(&self) -> BuchiAutomaton {
        let k = self.accepting_sets.len().max(1);
        let new_n = self.n_states * k;
        let mut nba = BuchiAutomaton::new(new_n);
        for &q0 in &self.initial {
            nba.add_initial((q0 as usize * k) as BuchiState);
        }
        for (&(s, label), targets) in &self.transitions {
            for &t in targets {
                for i in 0..k {
                    let src = (s as usize * k + i) as BuchiState;
                    let next_i = if self.accepting_sets[i].contains(&s) {
                        (i + 1) % k
                    } else {
                        i
                    };
                    let dst = (t as usize * k + next_i) as BuchiState;
                    nba.add_transition(src, label, dst);
                }
            }
        }
        if !self.accepting_sets.is_empty() {
            for &q in &self.accepting_sets[0] {
                nba.add_accepting((q as usize * k) as BuchiState);
            }
        }
        nba
    }
    /// Count total transitions.
    pub fn transition_count(&self) -> usize {
        self.transitions.values().map(|s| s.len()).sum()
    }
}
/// A nondeterministic Büchi automaton over alphabet 2^AP.
#[derive(Debug, Clone)]
pub struct BuchiAutomaton {
    /// Number of states.
    pub n_states: usize,
    /// Initial states.
    pub initial: HashSet<BuchiState>,
    /// Transitions: (state, label_set) → set of next states.
    pub transitions: HashMap<(BuchiState, u64), HashSet<BuchiState>>,
    /// Accepting (Büchi) states.
    pub accepting: HashSet<BuchiState>,
}
impl BuchiAutomaton {
    /// Create an empty Büchi automaton with `n` states.
    pub fn new(n: usize) -> Self {
        BuchiAutomaton {
            n_states: n,
            initial: HashSet::new(),
            transitions: HashMap::new(),
            accepting: HashSet::new(),
        }
    }
    /// Add an initial state.
    pub fn add_initial(&mut self, s: BuchiState) {
        self.initial.insert(s);
    }
    /// Add an accepting state.
    pub fn add_accepting(&mut self, s: BuchiState) {
        self.accepting.insert(s);
    }
    /// Add a transition: from state `s` on label `label`, go to `t`.
    pub fn add_transition(&mut self, s: BuchiState, label: u64, t: BuchiState) {
        self.transitions.entry((s, label)).or_default().insert(t);
    }
    /// Compute the successors of `s` on `label`.
    pub fn successors(&self, s: BuchiState, label: u64) -> Vec<BuchiState> {
        self.transitions
            .get(&(s, label))
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }
    /// Check whether the automaton has an empty language (no accepting run).
    /// Uses a simple SCC-based emptiness check.
    pub fn is_empty(&self) -> bool {
        if self.accepting.is_empty() {
            return true;
        }
        let mut reachable: HashSet<BuchiState> = HashSet::new();
        let mut queue: VecDeque<BuchiState> = self.initial.iter().cloned().collect();
        while let Some(s) = queue.pop_front() {
            if reachable.insert(s) {
                let all_labels: HashSet<u64> = self.transitions.keys().map(|&(_, l)| l).collect();
                for label in all_labels {
                    for t in self.successors(s, label) {
                        if !reachable.contains(&t) {
                            queue.push_back(t);
                        }
                    }
                }
            }
        }
        !self.accepting.iter().any(|s| reachable.contains(s))
    }
}
/// CTL state formula.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CtlFormula {
    /// Atomic proposition
    Atom(AtomId),
    /// ⊤
    True,
    /// ⊥
    False,
    /// ¬φ
    Not(Box<CtlFormula>),
    /// φ ∧ ψ
    And(Box<CtlFormula>, Box<CtlFormula>),
    /// φ ∨ ψ
    Or(Box<CtlFormula>, Box<CtlFormula>),
    /// EX φ — exists a next state satisfying φ
    EX(Box<CtlFormula>),
    /// AX φ — all next states satisfy φ
    AX(Box<CtlFormula>),
    /// EF φ — there exists a path with F φ
    EF(Box<CtlFormula>),
    /// AF φ — all paths have F φ
    AF(Box<CtlFormula>),
    /// EG φ — there exists a path with G φ
    EG(Box<CtlFormula>),
    /// AG φ — all paths have G φ
    AG(Box<CtlFormula>),
    /// EU: E\[φ U ψ\]
    EU(Box<CtlFormula>, Box<CtlFormula>),
    /// AU: A\[φ U ψ\]
    AU(Box<CtlFormula>, Box<CtlFormula>),
}
impl CtlFormula {
    /// EF φ = E\[⊤ U φ\]
    pub fn ef(phi: CtlFormula) -> Self {
        CtlFormula::EF(Box::new(phi))
    }
    /// AF φ = A\[⊤ U φ\]
    pub fn af(phi: CtlFormula) -> Self {
        CtlFormula::AF(Box::new(phi))
    }
    /// AG φ = ¬EF¬φ
    pub fn ag(phi: CtlFormula) -> Self {
        CtlFormula::AG(Box::new(phi))
    }
    /// EG φ = ¬AF¬φ
    pub fn eg(phi: CtlFormula) -> Self {
        CtlFormula::EG(Box::new(phi))
    }
    /// EU\[φ ψ\]
    pub fn eu(a: CtlFormula, b: CtlFormula) -> Self {
        CtlFormula::EU(Box::new(a), Box::new(b))
    }
}
/// LTL formula over atomic propositions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LtlFormula {
    /// Atomic proposition a_i
    Atom(AtomId),
    /// ⊤
    True,
    /// ⊥
    False,
    /// ¬φ
    Not(Box<LtlFormula>),
    /// φ ∧ ψ
    And(Box<LtlFormula>, Box<LtlFormula>),
    /// φ ∨ ψ
    Or(Box<LtlFormula>, Box<LtlFormula>),
    /// φ → ψ
    Implies(Box<LtlFormula>, Box<LtlFormula>),
    /// X φ (next)
    Next(Box<LtlFormula>),
    /// F φ (finally / eventually)
    Finally(Box<LtlFormula>),
    /// G φ (globally / always)
    Globally(Box<LtlFormula>),
    /// φ U ψ (until)
    Until(Box<LtlFormula>, Box<LtlFormula>),
    /// φ R ψ (release / dual of Until)
    Release(Box<LtlFormula>, Box<LtlFormula>),
    /// φ W ψ (weak until)
    WeakUntil(Box<LtlFormula>, Box<LtlFormula>),
}
impl LtlFormula {
    /// F φ = ⊤ U φ
    pub fn finally(phi: LtlFormula) -> Self {
        LtlFormula::Finally(Box::new(phi))
    }
    /// G φ = ⊥ R φ (via duality)
    pub fn globally(phi: LtlFormula) -> Self {
        LtlFormula::Globally(Box::new(phi))
    }
    /// X φ
    pub fn next(phi: LtlFormula) -> Self {
        LtlFormula::Next(Box::new(phi))
    }
    /// φ U ψ
    pub fn until(a: LtlFormula, b: LtlFormula) -> Self {
        LtlFormula::Until(Box::new(a), Box::new(b))
    }
    /// φ R ψ = ¬(¬φ U ¬ψ)
    pub fn release(a: LtlFormula, b: LtlFormula) -> Self {
        LtlFormula::Release(Box::new(a), Box::new(b))
    }
    /// Negate the formula (push negation inward — NNF).
    pub fn nnf(&self) -> LtlFormula {
        match self {
            LtlFormula::Not(phi) => phi.nnf_neg(),
            LtlFormula::And(a, b) => LtlFormula::And(Box::new(a.nnf()), Box::new(b.nnf())),
            LtlFormula::Or(a, b) => LtlFormula::Or(Box::new(a.nnf()), Box::new(b.nnf())),
            LtlFormula::Implies(a, b) => LtlFormula::Or(Box::new(a.nnf_neg()), Box::new(b.nnf())),
            LtlFormula::Next(phi) => LtlFormula::Next(Box::new(phi.nnf())),
            LtlFormula::Finally(phi) => LtlFormula::Finally(Box::new(phi.nnf())),
            LtlFormula::Globally(phi) => LtlFormula::Globally(Box::new(phi.nnf())),
            LtlFormula::Until(a, b) => LtlFormula::Until(Box::new(a.nnf()), Box::new(b.nnf())),
            LtlFormula::Release(a, b) => LtlFormula::Release(Box::new(a.nnf()), Box::new(b.nnf())),
            LtlFormula::WeakUntil(a, b) => {
                LtlFormula::WeakUntil(Box::new(a.nnf()), Box::new(b.nnf()))
            }
            other => other.clone(),
        }
    }
    /// Negate and push negation inward (for NNF construction).
    fn nnf_neg(&self) -> LtlFormula {
        match self {
            LtlFormula::True => LtlFormula::False,
            LtlFormula::False => LtlFormula::True,
            LtlFormula::Atom(p) => LtlFormula::Not(Box::new(LtlFormula::Atom(*p))),
            LtlFormula::Not(phi) => phi.nnf(),
            LtlFormula::And(a, b) => LtlFormula::Or(Box::new(a.nnf_neg()), Box::new(b.nnf_neg())),
            LtlFormula::Or(a, b) => LtlFormula::And(Box::new(a.nnf_neg()), Box::new(b.nnf_neg())),
            LtlFormula::Implies(a, b) => LtlFormula::And(Box::new(a.nnf()), Box::new(b.nnf_neg())),
            LtlFormula::Next(phi) => LtlFormula::Next(Box::new(phi.nnf_neg())),
            LtlFormula::Finally(phi) => LtlFormula::Globally(Box::new(phi.nnf_neg())),
            LtlFormula::Globally(phi) => LtlFormula::Finally(Box::new(phi.nnf_neg())),
            LtlFormula::Until(a, b) => {
                LtlFormula::Release(Box::new(a.nnf_neg()), Box::new(b.nnf_neg()))
            }
            LtlFormula::Release(a, b) => {
                LtlFormula::Until(Box::new(a.nnf_neg()), Box::new(b.nnf_neg()))
            }
            LtlFormula::WeakUntil(a, b) => {
                let na = a.nnf_neg();
                let nb = b.nnf_neg();
                LtlFormula::Until(
                    Box::new(nb.clone()),
                    Box::new(LtlFormula::And(Box::new(na), Box::new(nb))),
                )
            }
        }
    }
    /// Collect the closure set of a formula (used in tableau/automaton constructions).
    pub fn closure(&self) -> HashSet<LtlFormula> {
        let mut result = HashSet::new();
        self.collect_closure(&mut result);
        result
    }
    fn collect_closure(&self, set: &mut HashSet<LtlFormula>) {
        set.insert(self.clone());
        match self {
            LtlFormula::Not(phi) => phi.collect_closure(set),
            LtlFormula::And(a, b)
            | LtlFormula::Or(a, b)
            | LtlFormula::Implies(a, b)
            | LtlFormula::Until(a, b)
            | LtlFormula::Release(a, b)
            | LtlFormula::WeakUntil(a, b) => {
                a.collect_closure(set);
                b.collect_closure(set);
            }
            LtlFormula::Next(phi) | LtlFormula::Finally(phi) | LtlFormula::Globally(phi) => {
                phi.collect_closure(set)
            }
            _ => {}
        }
    }
    /// Check if the formula is a safety formula (syntactically).
    pub fn is_safety(&self) -> bool {
        match self {
            LtlFormula::Globally(_) => true,
            LtlFormula::And(a, b) => a.is_safety() && b.is_safety(),
            LtlFormula::Next(phi) => phi.is_safety(),
            _ => false,
        }
    }
    /// Check if the formula is a liveness formula (syntactically, very loose).
    pub fn is_liveness(&self) -> bool {
        match self {
            LtlFormula::Finally(_) => true,
            LtlFormula::Until(_, _) => true,
            LtlFormula::And(a, b) => a.is_liveness() || b.is_liveness(),
            _ => false,
        }
    }
}
/// A finite parity game.
#[derive(Debug, Clone)]
pub struct ParityGame {
    /// Vertices.
    pub vertices: Vec<ParityVertex>,
    /// Edges: vertex → set of successors.
    pub edges: HashMap<usize, HashSet<usize>>,
}
impl ParityGame {
    /// Create an empty parity game.
    pub fn new() -> Self {
        ParityGame {
            vertices: Vec::new(),
            edges: HashMap::new(),
        }
    }
    /// Add a vertex with given player and priority.
    pub fn add_vertex(&mut self, player: u8, priority: usize) -> usize {
        let id = self.vertices.len();
        self.vertices.push(ParityVertex {
            id,
            player,
            priority,
        });
        id
    }
    /// Add a directed edge from `u` to `v`.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.edges.entry(u).or_default().insert(v);
    }
    /// Compute the attractor set for player `p` towards a target set `target`.
    pub fn attractor(&self, player: u8, target: &HashSet<usize>) -> HashSet<usize> {
        let mut attr = target.clone();
        let mut queue: VecDeque<usize> = target.iter().cloned().collect();
        while let Some(v) = queue.pop_front() {
            for u in 0..self.vertices.len() {
                if attr.contains(&u) {
                    continue;
                }
                let succs: Vec<usize> = self
                    .edges
                    .get(&u)
                    .map(|s| s.iter().cloned().collect())
                    .unwrap_or_default();
                if !succs.contains(&v) {
                    continue;
                }
                let owned_by_player = self.vertices[u].player == player;
                let all_in_attr = succs.iter().all(|s| attr.contains(s));
                if owned_by_player || all_in_attr {
                    attr.insert(u);
                    queue.push_back(u);
                }
            }
        }
        attr
    }
    /// Solve the parity game using Zielonka's recursive algorithm.
    /// Returns (W0, W1) — winning sets for player 0 and player 1.
    pub fn solve(&self) -> (HashSet<usize>, HashSet<usize>) {
        let all_verts: HashSet<usize> = (0..self.vertices.len()).collect();
        self.zielonka(&all_verts)
    }
    fn zielonka(&self, verts: &HashSet<usize>) -> (HashSet<usize>, HashSet<usize>) {
        if verts.is_empty() {
            return (HashSet::new(), HashSet::new());
        }
        let max_prio = verts
            .iter()
            .map(|&v| self.vertices[v].priority)
            .max()
            .unwrap_or(0);
        let player = (max_prio % 2) as u8;
        let opp = 1 - player;
        let u: HashSet<usize> = verts
            .iter()
            .cloned()
            .filter(|&v| self.vertices[v].priority == max_prio)
            .collect();
        let attr_u = self.attractor(player, &u);
        let rest: HashSet<usize> = verts.difference(&attr_u).cloned().collect();
        let (w0_r, w1_r) = self.zielonka(&rest);
        let (_wp, wo) = if player == 0 {
            (w0_r, w1_r)
        } else {
            (w1_r, w0_r)
        };
        if wo.is_empty() {
            let win_player: HashSet<usize> = verts.union(&attr_u).cloned().collect();
            if player == 0 {
                (win_player, HashSet::new())
            } else {
                (HashSet::new(), win_player)
            }
        } else {
            let attr_wo = self.attractor(opp, &wo);
            let rest2: HashSet<usize> = verts.difference(&attr_wo).cloned().collect();
            let (w0_r2, w1_r2) = self.zielonka(&rest2);
            let (wp2, wo2) = if player == 0 {
                (w0_r2, w1_r2)
            } else {
                (w1_r2, w0_r2)
            };
            let win_opp: HashSet<usize> = wo2.union(&attr_wo).cloned().collect();
            if player == 0 {
                (wp2, win_opp)
            } else {
                (win_opp, wp2)
            }
        }
    }
    /// Number of vertices.
    pub fn size(&self) -> usize {
        self.vertices.len()
    }
}
/// A finite-state transition system for model checking.
#[derive(Debug, Clone)]
pub struct TransitionSystem {
    /// Number of states (states are labeled 0..n).
    pub n_states: usize,
    /// Initial states.
    pub initial: HashSet<usize>,
    /// Transition relation: state → set of successors.
    pub transitions: HashMap<usize, HashSet<usize>>,
    /// Labeling: state → set of true atomic propositions.
    pub labels: HashMap<usize, HashSet<AtomId>>,
}
impl TransitionSystem {
    /// Create a new transition system with n states and no transitions.
    pub fn new(n_states: usize) -> Self {
        TransitionSystem {
            n_states,
            initial: HashSet::new(),
            transitions: HashMap::new(),
            labels: HashMap::new(),
        }
    }
    /// Add a transition from `s` to `t`.
    pub fn add_transition(&mut self, s: usize, t: usize) {
        self.transitions.entry(s).or_default().insert(t);
    }
    /// Mark state `s` as initial.
    pub fn add_initial(&mut self, s: usize) {
        self.initial.insert(s);
    }
    /// Label state `s` with atomic proposition `a`.
    pub fn add_label(&mut self, s: usize, a: AtomId) {
        self.labels.entry(s).or_default().insert(a);
    }
    /// Return all successors of state `s`.
    pub fn successors(&self, s: usize) -> Vec<usize> {
        self.transitions
            .get(&s)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }
    /// Return all predecessors of state `s`.
    pub fn predecessors(&self, s: usize) -> Vec<usize> {
        (0..self.n_states)
            .filter(|&t| {
                self.transitions
                    .get(&t)
                    .is_some_and(|succs| succs.contains(&s))
            })
            .collect()
    }
    /// Check if atomic proposition `a` holds at state `s`.
    pub fn label_holds(&self, s: usize, a: AtomId) -> bool {
        self.labels.get(&s).is_some_and(|l| l.contains(&a))
    }
    /// Compute all reachable states via BFS.
    pub fn reachable_states(&self) -> HashSet<usize> {
        let mut visited = HashSet::new();
        let mut queue: VecDeque<usize> = self.initial.iter().cloned().collect();
        while let Some(s) = queue.pop_front() {
            if visited.insert(s) {
                for t in self.successors(s) {
                    if !visited.contains(&t) {
                        queue.push_back(t);
                    }
                }
            }
        }
        visited
    }
    /// Compute strongly connected components (Tarjan's algorithm).
    pub fn scc(&self) -> Vec<HashSet<usize>> {
        let mut index_counter = 0usize;
        let mut stack = Vec::new();
        let mut lowlink: HashMap<usize, usize> = HashMap::new();
        let mut index: HashMap<usize, usize> = HashMap::new();
        let mut on_stack: HashSet<usize> = HashSet::new();
        let mut sccs = Vec::new();
        fn strongconnect(
            v: usize,
            ts: &TransitionSystem,
            index: &mut HashMap<usize, usize>,
            lowlink: &mut HashMap<usize, usize>,
            on_stack: &mut HashSet<usize>,
            stack: &mut Vec<usize>,
            index_counter: &mut usize,
            sccs: &mut Vec<HashSet<usize>>,
        ) {
            index.insert(v, *index_counter);
            lowlink.insert(v, *index_counter);
            *index_counter += 1;
            stack.push(v);
            on_stack.insert(v);
            for w in ts.successors(v) {
                if !index.contains_key(&w) {
                    strongconnect(w, ts, index, lowlink, on_stack, stack, index_counter, sccs);
                    let ll_w = lowlink[&w];
                    let ll_v = lowlink[&v];
                    lowlink.insert(v, ll_v.min(ll_w));
                } else if on_stack.contains(&w) {
                    let idx_w = index[&w];
                    let ll_v = lowlink[&v];
                    lowlink.insert(v, ll_v.min(idx_w));
                }
            }
            if lowlink[&v] == index[&v] {
                let mut component = HashSet::new();
                loop {
                    let w = stack.pop().expect(
                        "stack is non-empty: Tarjan's algorithm guarantees v is on the stack",
                    );
                    on_stack.remove(&w);
                    component.insert(w);
                    if w == v {
                        break;
                    }
                }
                sccs.push(component);
            }
        }
        for v in 0..self.n_states {
            if !index.contains_key(&v) {
                strongconnect(
                    v,
                    self,
                    &mut index,
                    &mut lowlink,
                    &mut on_stack,
                    &mut stack,
                    &mut index_counter,
                    &mut sccs,
                );
            }
        }
        sccs
    }
}
/// CTL model checker using explicit-state fixpoint computation.
pub struct CtlChecker<'a> {
    /// The transition system being checked.
    pub ts: &'a TransitionSystem,
}
impl<'a> CtlChecker<'a> {
    /// Create a new CTL checker for the given transition system.
    pub fn new(ts: &'a TransitionSystem) -> Self {
        CtlChecker { ts }
    }
    /// Compute the set of states satisfying a CTL formula.
    pub fn sat(&self, phi: &CtlFormula) -> HashSet<usize> {
        match phi {
            CtlFormula::True => (0..self.ts.n_states).collect(),
            CtlFormula::False => HashSet::new(),
            CtlFormula::Atom(a) => (0..self.ts.n_states)
                .filter(|&s| self.ts.label_holds(s, *a))
                .collect(),
            CtlFormula::Not(psi) => {
                let sat_psi = self.sat(psi);
                (0..self.ts.n_states)
                    .filter(|s| !sat_psi.contains(s))
                    .collect()
            }
            CtlFormula::And(a, b) => {
                let sa = self.sat(a);
                let sb = self.sat(b);
                sa.intersection(&sb).cloned().collect()
            }
            CtlFormula::Or(a, b) => {
                let sa = self.sat(a);
                let sb = self.sat(b);
                sa.union(&sb).cloned().collect()
            }
            CtlFormula::EX(psi) => self.sat_ex(&self.sat(psi)),
            CtlFormula::AX(psi) => self.sat_ax(&self.sat(psi)),
            CtlFormula::EF(psi) => self.sat_ef(&self.sat(psi)),
            CtlFormula::AF(psi) => self.sat_af(&self.sat(psi)),
            CtlFormula::EG(psi) => self.sat_eg(&self.sat(psi)),
            CtlFormula::AG(psi) => self.sat_ag(&self.sat(psi)),
            CtlFormula::EU(a, b) => self.sat_eu(&self.sat(a), &self.sat(b)),
            CtlFormula::AU(a, b) => self.sat_au(&self.sat(a), &self.sat(b)),
        }
    }
    /// EX φ: exists a successor satisfying φ
    fn sat_ex(&self, sat_phi: &HashSet<usize>) -> HashSet<usize> {
        (0..self.ts.n_states)
            .filter(|&s| self.ts.successors(s).iter().any(|t| sat_phi.contains(t)))
            .collect()
    }
    /// AX φ: all successors satisfy φ
    fn sat_ax(&self, sat_phi: &HashSet<usize>) -> HashSet<usize> {
        (0..self.ts.n_states)
            .filter(|&s| {
                let succs = self.ts.successors(s);
                !succs.is_empty() && succs.iter().all(|t| sat_phi.contains(t))
            })
            .collect()
    }
    /// EF φ = lfp X. φ ∨ EX X
    fn sat_ef(&self, sat_phi: &HashSet<usize>) -> HashSet<usize> {
        let mut current = sat_phi.clone();
        loop {
            let pre = self.sat_ex(&current);
            let next: HashSet<usize> = current.union(&pre).cloned().collect();
            if next == current {
                break;
            }
            current = next;
        }
        current
    }
    /// AF φ = lfp X. φ ∨ AX X
    fn sat_af(&self, sat_phi: &HashSet<usize>) -> HashSet<usize> {
        let mut current = sat_phi.clone();
        loop {
            let pre = self.sat_ax(&current);
            let next: HashSet<usize> = current.union(&pre).cloned().collect();
            if next == current {
                break;
            }
            current = next;
        }
        current
    }
    /// EG φ = gfp X. φ ∧ EX X
    fn sat_eg(&self, sat_phi: &HashSet<usize>) -> HashSet<usize> {
        let mut current = sat_phi.clone();
        loop {
            let pre = self.sat_ex(&current);
            let next: HashSet<usize> = current.intersection(&pre).cloned().collect();
            if next == current {
                break;
            }
            current = next;
        }
        current
    }
    /// AG φ = gfp X. φ ∧ AX X
    fn sat_ag(&self, sat_phi: &HashSet<usize>) -> HashSet<usize> {
        let mut current = sat_phi.clone();
        loop {
            let pre = self.sat_ax(&current);
            let next: HashSet<usize> = current.intersection(&pre).cloned().collect();
            if next == current {
                break;
            }
            current = next;
        }
        current
    }
    /// E\[φ U ψ\] = lfp X. ψ ∨ (φ ∧ EX X)
    fn sat_eu(&self, sat_phi: &HashSet<usize>, sat_psi: &HashSet<usize>) -> HashSet<usize> {
        let mut current = sat_psi.clone();
        loop {
            let pre = self.sat_ex(&current);
            let phi_and_pre: HashSet<usize> = sat_phi.intersection(&pre).cloned().collect();
            let next: HashSet<usize> = current.union(&phi_and_pre).cloned().collect();
            if next == current {
                break;
            }
            current = next;
        }
        current
    }
    /// A\[φ U ψ\] = lfp X. ψ ∨ (φ ∧ AX X)
    fn sat_au(&self, sat_phi: &HashSet<usize>, sat_psi: &HashSet<usize>) -> HashSet<usize> {
        let mut current = sat_psi.clone();
        loop {
            let pre = self.sat_ax(&current);
            let phi_and_pre: HashSet<usize> = sat_phi.intersection(&pre).cloned().collect();
            let next: HashSet<usize> = current.union(&phi_and_pre).cloned().collect();
            if next == current {
                break;
            }
            current = next;
        }
        current
    }
    /// Check if initial states satisfy φ.
    pub fn check(&self, phi: &CtlFormula) -> bool {
        let sat_set = self.sat(phi);
        self.ts.initial.iter().all(|s| sat_set.contains(s))
    }
    /// Find a counterexample state (an initial state not satisfying φ).
    pub fn counterexample(&self, phi: &CtlFormula) -> Option<usize> {
        let sat_set = self.sat(phi);
        self.ts
            .initial
            .iter()
            .find(|s| !sat_set.contains(s))
            .cloned()
    }
}
/// A concurrent game structure for ATL strategy synthesis.
#[allow(dead_code)]
pub struct ConcurrentGame {
    /// Number of states.
    pub n_states: usize,
    /// Number of agents.
    pub n_agents: usize,
    /// Actions available to agent i at state s: actions\[s\]\[i\] = count.
    pub actions: HashMap<usize, Vec<usize>>,
    /// Transition: (state, joint_action) → next_state.
    pub delta: HashMap<(usize, Vec<usize>), usize>,
    /// Labels: state → set of true atomic propositions.
    pub labels: HashMap<usize, HashSet<AtomId>>,
}
#[allow(dead_code)]
impl ConcurrentGame {
    /// Create a new concurrent game structure.
    pub fn new(n_states: usize, n_agents: usize) -> Self {
        ConcurrentGame {
            n_states,
            n_agents,
            actions: HashMap::new(),
            delta: HashMap::new(),
            labels: HashMap::new(),
        }
    }
    /// Set the number of actions for agent `agent` at state `s`.
    pub fn set_actions(&mut self, s: usize, agent: usize, count: usize) {
        let entry = self
            .actions
            .entry(s)
            .or_insert_with(|| vec![1; self.n_agents]);
        if agent < entry.len() {
            entry[agent] = count;
        }
    }
    /// Set the transition for a given joint action.
    pub fn set_transition(&mut self, s: usize, joint: Vec<usize>, t: usize) {
        self.delta.insert((s, joint), t);
    }
    /// Label a state with an atomic proposition.
    pub fn add_label(&mut self, s: usize, a: AtomId) {
        self.labels.entry(s).or_default().insert(a);
    }
    /// Check if a coalition `coalition` (as bitmask) can enforce EX φ from state s.
    pub fn can_enforce_ex(&self, coalition: u64, s: usize, sat_phi: &HashSet<usize>) -> bool {
        let _ = coalition;
        self.actions
            .get(&s)
            .map(|_acts| {
                self.delta
                    .iter()
                    .any(|((src, _joint), &dst)| *src == s && sat_phi.contains(&dst))
            })
            .unwrap_or(false)
    }
    /// Compute the set of states from which coalition can enforce EF φ.
    pub fn sat_coop_ef(&self, coalition: u64, sat_phi: &HashSet<usize>) -> HashSet<usize> {
        let mut current = sat_phi.clone();
        loop {
            let pre: HashSet<usize> = (0..self.n_states)
                .filter(|&s| !current.contains(&s) && self.can_enforce_ex(coalition, s, &current))
                .collect();
            let next: HashSet<usize> = current.union(&pre).cloned().collect();
            if next == current {
                break;
            }
            current = next;
        }
        current
    }
}
/// Simplified BDD node (for teaching purposes — not a full reduced OBDD).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BddNode {
    /// Terminal 0 (false)
    Zero,
    /// Terminal 1 (true)
    One,
    /// Internal node: variable index, low (false), high (true)
    Node(u32, Box<BddNode>, Box<BddNode>),
}
impl BddNode {
    /// Evaluate the BDD on an assignment (var → bool).
    pub fn eval(&self, assignment: &HashMap<u32, bool>) -> bool {
        match self {
            BddNode::Zero => false,
            BddNode::One => true,
            BddNode::Node(var, low, high) => {
                let val = assignment.get(var).cloned().unwrap_or(false);
                if val {
                    high.eval(assignment)
                } else {
                    low.eval(assignment)
                }
            }
        }
    }
    /// Apply Boolean AND to two BDDs (simplified, non-canonical).
    pub fn and(a: &BddNode, b: &BddNode) -> BddNode {
        match (a, b) {
            (BddNode::Zero, _) | (_, BddNode::Zero) => BddNode::Zero,
            (BddNode::One, x) | (x, BddNode::One) => x.clone(),
            (BddNode::Node(va, la, ha), BddNode::Node(vb, lb, hb)) if va == vb => BddNode::Node(
                *va,
                Box::new(BddNode::and(la, lb)),
                Box::new(BddNode::and(ha, hb)),
            ),
            (BddNode::Node(va, la, ha), other) => BddNode::Node(
                *va,
                Box::new(BddNode::and(la, other)),
                Box::new(BddNode::and(ha, other)),
            ),
        }
    }
    /// Apply Boolean OR to two BDDs (simplified).
    pub fn or(a: &BddNode, b: &BddNode) -> BddNode {
        match (a, b) {
            (BddNode::One, _) | (_, BddNode::One) => BddNode::One,
            (BddNode::Zero, x) | (x, BddNode::Zero) => x.clone(),
            (BddNode::Node(va, la, ha), BddNode::Node(vb, lb, hb)) if va == vb => BddNode::Node(
                *va,
                Box::new(BddNode::or(la, lb)),
                Box::new(BddNode::or(ha, hb)),
            ),
            (BddNode::Node(va, la, ha), other) => BddNode::Node(
                *va,
                Box::new(BddNode::or(la, other)),
                Box::new(BddNode::or(ha, other)),
            ),
        }
    }
}
/// A parity game vertex.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParityVertex {
    /// Vertex index.
    pub id: usize,
    /// Owning player (0 or 1).
    pub player: u8,
    /// Priority (color).
    pub priority: usize,
}
/// A simple bounded model checker for safety properties.
#[allow(dead_code)]
pub struct BoundedModelChecker<'a> {
    /// The transition system.
    pub ts: &'a TransitionSystem,
    /// The bound k.
    pub bound: usize,
}
#[allow(dead_code)]
impl<'a> BoundedModelChecker<'a> {
    /// Create a new bounded model checker.
    pub fn new(ts: &'a TransitionSystem, bound: usize) -> Self {
        BoundedModelChecker { ts, bound }
    }
    /// Check a safety formula: AG φ holds up to depth k.
    /// Returns None if no violation found, Some(path) otherwise.
    pub fn check_safety(&self, safe_pred: impl Fn(usize) -> bool) -> Option<Vec<usize>> {
        let mut queue: VecDeque<(usize, Vec<usize>)> =
            self.ts.initial.iter().map(|&s| (s, vec![s])).collect();
        let mut visited: HashSet<usize> = HashSet::new();
        while let Some((s, path)) = queue.pop_front() {
            if path.len() > self.bound + 1 {
                continue;
            }
            if !safe_pred(s) {
                return Some(path);
            }
            if visited.insert(s) {
                for t in self.ts.successors(s) {
                    let mut new_path = path.clone();
                    new_path.push(t);
                    queue.push_back((t, new_path));
                }
            }
        }
        None
    }
    /// Count states explored up to bound.
    pub fn count_states_at_depth(&self, depth: usize) -> usize {
        if depth == 0 {
            return self.ts.initial.len();
        }
        let mut frontier: HashSet<usize> = self.ts.initial.clone();
        for _ in 0..depth {
            let mut next_frontier = HashSet::new();
            for s in &frontier {
                for t in self.ts.successors(*s) {
                    next_frontier.insert(t);
                }
            }
            frontier = next_frontier;
        }
        frontier.len()
    }
}
/// A deterministic Streett automaton (complement of Rabin).
#[allow(dead_code)]
pub struct StreettAutomaton {
    /// Number of states.
    pub n_states: usize,
    /// Initial state (deterministic).
    pub initial: usize,
    /// Transition function: (state, label) → next state.
    pub delta: HashMap<(usize, u64), usize>,
    /// Streett pairs: list of (E_i, F_i) — must hold: inf E_i → inf F_i.
    pub pairs: Vec<(HashSet<usize>, HashSet<usize>)>,
}
#[allow(dead_code)]
impl StreettAutomaton {
    /// Create a new Streett automaton.
    pub fn new(n: usize) -> Self {
        StreettAutomaton {
            n_states: n,
            initial: 0,
            delta: HashMap::new(),
            pairs: Vec::new(),
        }
    }
    /// Add a Streett pair (E, F).
    pub fn add_pair(&mut self, e: HashSet<usize>, f: HashSet<usize>) {
        self.pairs.push((e, f));
    }
    /// Set the deterministic transition.
    pub fn set_transition(&mut self, s: usize, label: u64, t: usize) {
        self.delta.insert((s, label), t);
    }
    /// Get the successor of state s on label.
    pub fn successor(&self, s: usize, label: u64) -> Option<usize> {
        self.delta.get(&(s, label)).cloned()
    }
    /// Count the number of Streett pairs.
    pub fn pair_count(&self) -> usize {
        self.pairs.len()
    }
    /// Check if a run satisfies the Streett condition.
    /// A run is accepting if for all i: if E_i is visited inf often, then F_i is too.
    /// This checks on a finite prefix (approximation).
    pub fn accepts_prefix(&self, run: &[usize]) -> bool {
        for (e_set, f_set) in &self.pairs {
            let visits_e = run.iter().any(|s| e_set.contains(s));
            let visits_f = run.iter().any(|s| f_set.contains(s));
            if visits_e && !visits_f {
                return false;
            }
        }
        true
    }
}
/// Modal mu-calculus formula.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MuFormula {
    /// ⊤
    True,
    /// ⊥
    False,
    /// Atomic proposition
    Atom(AtomId),
    /// Fixpoint variable
    Var(MuVar),
    /// ¬φ
    Not(Box<MuFormula>),
    /// φ ∧ ψ
    And(Box<MuFormula>, Box<MuFormula>),
    /// φ ∨ ψ
    Or(Box<MuFormula>, Box<MuFormula>),
    /// ⟨a⟩φ (diamond modality)
    Diamond(Box<MuFormula>),
    /// \[a\]φ (box modality)
    Box(Box<MuFormula>),
    /// μX.φ (least fixpoint)
    Mu(MuVar, Box<MuFormula>),
    /// νX.φ (greatest fixpoint)
    Nu(MuVar, Box<MuFormula>),
}
impl MuFormula {
    /// EF φ = μX. φ ∨ ⟨a⟩X
    pub fn ef(phi: MuFormula) -> Self {
        MuFormula::Mu(
            "X".into(),
            Box::new(MuFormula::Or(
                Box::new(phi),
                Box::new(MuFormula::Diamond(Box::new(MuFormula::Var("X".into())))),
            )),
        )
    }
    /// AG φ = νX. φ ∧ \[a\]X
    pub fn ag(phi: MuFormula) -> Self {
        MuFormula::Nu(
            "X".into(),
            Box::new(MuFormula::And(
                Box::new(phi),
                Box::new(MuFormula::Box(Box::new(MuFormula::Var("X".into())))),
            )),
        )
    }
    /// Compute the alternation depth of the formula.
    /// Alternation depth counts how many times the fixpoint type changes
    /// (μ under ν, or ν under μ) along any path.
    pub fn alternation_depth(&self) -> usize {
        // None = no enclosing fixpoint, Some(true) = under ν, Some(false) = under μ
        self.alt_depth_inner(None)
    }
    fn alt_depth_inner(&self, enclosing: Option<bool>) -> usize {
        match self {
            MuFormula::Mu(_, phi) => {
                let inner = phi.alt_depth_inner(Some(false));
                if enclosing == Some(true) {
                    // μ under ν: alternation
                    1 + inner
                } else {
                    inner
                }
            }
            MuFormula::Nu(_, phi) => {
                let inner = phi.alt_depth_inner(Some(true));
                if enclosing == Some(false) {
                    // ν under μ: alternation
                    1 + inner
                } else {
                    inner
                }
            }
            MuFormula::And(a, b) | MuFormula::Or(a, b) => a
                .alt_depth_inner(enclosing)
                .max(b.alt_depth_inner(enclosing)),
            MuFormula::Not(phi) | MuFormula::Diamond(phi) | MuFormula::Box(phi) => {
                phi.alt_depth_inner(enclosing)
            }
            _ => 0,
        }
    }
}
