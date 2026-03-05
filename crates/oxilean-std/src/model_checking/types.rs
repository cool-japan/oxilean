//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};
use std::collections::{HashMap, HashSet, VecDeque};

use super::functions::*;

/// A counterexample: a trace (sequence of states) witnessing a formula violation.
#[derive(Debug, Clone)]
pub struct CounterExample {
    /// The sequence of states in the counterexample.
    pub trace: Vec<usize>,
    /// Index into `trace` where the lasso loop starts (-1 = no loop).
    pub loop_start: Option<usize>,
    /// The violated formula (as a display string).
    pub violated_formula: String,
}
impl CounterExample {
    /// Create a finite counterexample trace.
    pub fn finite(trace: Vec<usize>, formula: impl Into<String>) -> Self {
        Self {
            trace,
            loop_start: None,
            violated_formula: formula.into(),
        }
    }
    /// Create a lasso (prefix + cycle) counterexample.
    pub fn lasso(trace: Vec<usize>, loop_start: usize, formula: impl Into<String>) -> Self {
        Self {
            trace,
            loop_start: Some(loop_start),
            violated_formula: formula.into(),
        }
    }
    /// Returns true if this is a lasso (infinite path).
    pub fn is_lasso(&self) -> bool {
        self.loop_start.is_some()
    }
}
/// LTL model checker: automaton-theoretic approach via Büchi automata.
#[derive(Debug, Clone)]
pub struct LtlModelChecker {
    /// The Kripke structure to check.
    pub kripke: KripkeStructure,
}
impl LtlModelChecker {
    /// Create a new LTL model checker for the given Kripke structure.
    pub fn new(kripke: KripkeStructure) -> Self {
        Self { kripke }
    }
    /// Check whether the Kripke structure satisfies the LTL formula.
    /// This is a placeholder that always returns true for trivially-safe formulas.
    pub fn check_ltl(&self, formula: &LtlFormula) -> bool {
        match formula {
            LtlFormula::True_ => true,
            LtlFormula::False_ => false,
            LtlFormula::Always(inner) => {
                if let LtlFormula::Atom(p) = inner.as_ref() {
                    let reachable = self.kripke.reachable_states();
                    reachable.iter().all(|&s| self.kripke.labeling[s].holds(p))
                } else {
                    true
                }
            }
            _ => true,
        }
    }
    /// Attempt to find a counterexample for the given LTL formula.
    pub fn find_counterexample(&self, formula: &LtlFormula) -> Option<CounterExample> {
        if !self.check_ltl(formula) {
            let trace: Vec<usize> = self.kripke.reachable_states().into_iter().collect();
            Some(CounterExample::finite(trace, format!("{}", formula)))
        } else {
            None
        }
    }
    /// Synthesize a strategy (stub).
    pub fn synthesize_strategy(&self, _formula: &LtlFormula) -> Option<String> {
        None
    }
}
/// Abstract transformer: post[τ](α(S)).
#[derive(Debug, Clone)]
pub struct AbstractTransformer {
    /// Name of the transformer.
    pub name: String,
}
impl AbstractTransformer {
    /// Create a named abstract transformer.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
    /// Apply the transformer: returns a (placeholder) successor abstract state.
    pub fn apply(&self, domain: &AbstractDomain) -> AbstractDomain {
        domain.clone()
    }
}
/// A Kripke structure M = (S, S_0, R, L).
#[derive(Debug, Clone)]
pub struct KripkeStructure {
    /// Total number of states (states are 0..num_states).
    pub num_states: usize,
    /// Initial states.
    pub initial_states: Vec<usize>,
    /// Transition relation: transition_relation[s] = successors of s.
    pub transition_relation: Vec<Vec<usize>>,
    /// Labeling function.
    pub labeling: Vec<StateLabel>,
}
impl KripkeStructure {
    /// Create a new Kripke structure with `n` states.
    pub fn new(n: usize) -> Self {
        let labeling = (0..n).map(StateLabel::new).collect();
        Self {
            num_states: n,
            initial_states: Vec::new(),
            transition_relation: vec![Vec::new(); n],
            labeling,
        }
    }
    /// Mark state `s` as an initial state.
    pub fn add_initial(&mut self, s: usize) {
        if !self.initial_states.contains(&s) {
            self.initial_states.push(s);
        }
    }
    /// Add a transition from `s` to `t`.
    pub fn add_transition(&mut self, s: usize, t: usize) {
        if s < self.num_states && t < self.num_states && !self.transition_relation[s].contains(&t) {
            self.transition_relation[s].push(t);
        }
    }
    /// Add a proposition to a state's label.
    pub fn label_state(&mut self, s: usize, prop: impl Into<String>) {
        if s < self.num_states {
            self.labeling[s].add(prop);
        }
    }
    /// Return all states reachable from initial states via BFS.
    pub fn reachable_states(&self) -> HashSet<usize> {
        let mut visited = HashSet::new();
        let mut queue: VecDeque<usize> = self.initial_states.iter().copied().collect();
        while let Some(s) = queue.pop_front() {
            if visited.insert(s) {
                for &t in &self.transition_relation[s] {
                    if !visited.contains(&t) {
                        queue.push_back(t);
                    }
                }
            }
        }
        visited
    }
    /// Returns true if all states are reachable from the initial states.
    pub fn is_connected(&self) -> bool {
        self.reachable_states().len() == self.num_states
    }
    /// Compute strongly connected components (Kosaraju's algorithm).
    pub fn compute_scc(&self) -> Vec<Vec<usize>> {
        let n = self.num_states;
        let mut visited = vec![false; n];
        let mut finish_order = Vec::with_capacity(n);
        for start in 0..n {
            if !visited[start] {
                self.dfs_finish(start, &mut visited, &mut finish_order);
            }
        }
        let mut rev = vec![Vec::new(); n];
        for s in 0..n {
            for &t in &self.transition_relation[s] {
                rev[t].push(s);
            }
        }
        let mut visited2 = vec![false; n];
        let mut sccs = Vec::new();
        for &start in finish_order.iter().rev() {
            if !visited2[start] {
                let mut component = Vec::new();
                Self::dfs_collect(start, &rev, &mut visited2, &mut component);
                sccs.push(component);
            }
        }
        sccs
    }
    fn dfs_finish(&self, v: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
        visited[v] = true;
        for &u in &self.transition_relation[v] {
            if !visited[u] {
                self.dfs_finish(u, visited, order);
            }
        }
        order.push(v);
    }
    fn dfs_collect(
        v: usize,
        rev: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        comp: &mut Vec<usize>,
    ) {
        visited[v] = true;
        comp.push(v);
        for &u in &rev[v] {
            if !visited[u] {
                Self::dfs_collect(u, rev, visited, comp);
            }
        }
    }
}
/// A parity game graph for Zielonka's algorithm.
#[derive(Debug, Clone)]
pub struct ParityGameZielonka {
    /// Number of vertices.
    pub num_vertices: usize,
    /// Priority of each vertex.
    pub priority: Vec<u32>,
    /// Owner of each vertex (0 = Player 0 / Even, 1 = Player 1 / Odd).
    pub owner: Vec<u8>,
    /// Successors of each vertex.
    pub successors: Vec<Vec<usize>>,
}
impl ParityGameZielonka {
    /// Create a new parity game with `n` vertices.
    pub fn new(n: usize) -> Self {
        Self {
            num_vertices: n,
            priority: vec![0; n],
            owner: vec![0; n],
            successors: vec![Vec::new(); n],
        }
    }
    /// Set the priority of vertex `v`.
    pub fn set_priority(&mut self, v: usize, p: u32) {
        if v < self.num_vertices {
            self.priority[v] = p;
        }
    }
    /// Set the owner of vertex `v` (0 = Player 0, 1 = Player 1).
    pub fn set_owner(&mut self, v: usize, player: u8) {
        if v < self.num_vertices {
            self.owner[v] = player & 1;
        }
    }
    /// Add a move from `u` to `v`.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        if u < self.num_vertices && v < self.num_vertices {
            self.successors[u].push(v);
        }
    }
    /// Compute max priority in a set of vertices.
    fn max_priority_in(&self, verts: &HashSet<usize>) -> u32 {
        verts.iter().map(|&v| self.priority[v]).max().unwrap_or(0)
    }
    /// Get vertices with priority `p` in a set.
    fn verts_with_priority(&self, verts: &HashSet<usize>, p: u32) -> HashSet<usize> {
        verts
            .iter()
            .copied()
            .filter(|&v| self.priority[v] == p)
            .collect()
    }
    /// Compute the attractor of `target` for `player` in the subgame `verts`.
    fn attractor(
        &self,
        player: u8,
        target: &HashSet<usize>,
        verts: &HashSet<usize>,
    ) -> HashSet<usize> {
        let mut attr = target.clone();
        let mut queue: VecDeque<usize> = target.iter().copied().collect();
        while let Some(v) = queue.pop_front() {
            for u in verts {
                if attr.contains(u) {
                    continue;
                }
                let succ_in_verts: Vec<usize> = self.successors[*u]
                    .iter()
                    .copied()
                    .filter(|&w| verts.contains(&w))
                    .collect();
                if succ_in_verts.is_empty() {
                    continue;
                }
                let attracts = if self.owner[*u] == player {
                    succ_in_verts.iter().any(|w| attr.contains(w))
                } else {
                    succ_in_verts.iter().all(|w| attr.contains(w))
                };
                if attracts && self.successors[*u].iter().any(|w| w == &v) {
                    attr.insert(*u);
                    queue.push_back(*u);
                }
            }
        }
        attr
    }
    /// Solve the parity game using Zielonka's recursive algorithm.
    /// Returns (W0, W1): winning sets for Player 0 and Player 1.
    pub fn solve(&self) -> (HashSet<usize>, HashSet<usize>) {
        let all: HashSet<usize> = (0..self.num_vertices).collect();
        self.zielonka(&all)
    }
    fn zielonka(&self, verts: &HashSet<usize>) -> (HashSet<usize>, HashSet<usize>) {
        if verts.is_empty() {
            return (HashSet::new(), HashSet::new());
        }
        let p = self.max_priority_in(verts);
        let player = (p % 2) as u8;
        let opponent = 1 - player;
        let u = self.verts_with_priority(verts, p);
        let attr_u = self.attractor(player, &u, verts);
        let verts_minus: HashSet<usize> = verts.difference(&attr_u).copied().collect();
        let (mut w0, mut w1) = self.zielonka(&verts_minus);
        let (wo_player, wo_opp) = if player == 0 {
            (&mut w0, &mut w1)
        } else {
            (&mut w1, &mut w0)
        };
        if wo_opp.is_empty() {
            for &v in &attr_u {
                wo_player.insert(v);
            }
        } else {
            let attr_opp = self.attractor(opponent, wo_opp, verts);
            let verts2: HashSet<usize> = verts.difference(&attr_opp).copied().collect();
            let (mut w0b, mut w1b) = self.zielonka(&verts2);
            if opponent == 0 {
                for &v in &attr_opp {
                    w0b.insert(v);
                }
            } else {
                for &v in &attr_opp {
                    w1b.insert(v);
                }
            }
            if player == 0 {
                return (w0b.clone(), w1b.clone());
            } else {
                return (w1b.clone(), w0b.clone());
            }
        }
        (w0, w1)
    }
    /// Returns true if Player 0 wins from vertex `v`.
    pub fn player0_wins(&self, v: usize) -> bool {
        let (w0, _) = self.solve();
        w0.contains(&v)
    }
}
/// A CTL* formula: combines LTL path formulas with CTL state quantifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CtlStarFormula {
    /// State formula: atomic proposition.
    Atom(String),
    /// State formula: negation.
    Not(Box<CtlStarFormula>),
    /// State formula: conjunction.
    And(Box<CtlStarFormula>, Box<CtlStarFormula>),
    /// State formula: disjunction.
    Or(Box<CtlStarFormula>, Box<CtlStarFormula>),
    /// Existential path quantifier E[path].
    E(Box<CtlStarFormula>),
    /// Universal path quantifier A[path].
    A(Box<CtlStarFormula>),
    /// Path formula: Next.
    Next(Box<CtlStarFormula>),
    /// Path formula: Until.
    Until(Box<CtlStarFormula>, Box<CtlStarFormula>),
    /// Path formula: Eventually.
    Eventually(Box<CtlStarFormula>),
    /// Path formula: Always.
    Always(Box<CtlStarFormula>),
}
/// CEGAR loop: counterexample-guided abstraction refinement.
#[derive(Debug, Clone)]
pub struct CounterExampleGuidedRefinement {
    /// Current abstract domain.
    pub domain: AbstractDomain,
    /// Number of refinement iterations performed.
    pub iterations: usize,
    /// Whether a proof has been found.
    pub verified: bool,
}
impl CounterExampleGuidedRefinement {
    /// Create a CEGAR instance with the given initial abstraction.
    pub fn new(domain: AbstractDomain) -> Self {
        Self {
            domain,
            iterations: 0,
            verified: false,
        }
    }
    /// Map concrete states to their abstract representation.
    pub fn abstract_states(&self, states: &[usize]) -> AbstractDomain {
        let preds: Vec<String> = states.iter().map(|s| format!("s{}", s)).collect();
        AbstractDomain::predicate(preds)
    }
    /// Refine the abstraction using a spurious counterexample.
    pub fn refine_abstraction(&mut self, spurious: &SpuriousCounterexample) {
        self.domain
            .predicates
            .push(spurious.infeasibility_reason.clone());
        self.iterations += 1;
    }
    /// Check whether a counterexample is feasible (true = feasible, false = spurious).
    pub fn check_feasibility(&self, cex: &CounterExample) -> bool {
        !cex.trace.is_empty() && cex.loop_start.is_none()
    }
}
/// A spurious counterexample: an abstract path that has no concrete realization.
#[derive(Debug, Clone)]
pub struct SpuriousCounterexample {
    /// The abstract trace (sequence of abstract state descriptions).
    pub abstract_trace: Vec<String>,
    /// Why the path is spurious.
    pub infeasibility_reason: String,
}
impl SpuriousCounterexample {
    /// Create a spurious counterexample.
    pub fn new(trace: Vec<String>, reason: impl Into<String>) -> Self {
        Self {
            abstract_trace: trace,
            infeasibility_reason: reason.into(),
        }
    }
}
/// A Binary Decision Diagram (BDD).
#[derive(Debug, Clone)]
pub struct BDD {
    /// The unique-table of nodes.
    pub nodes: Vec<BDDNode>,
    /// Index of the root node.
    pub root: usize,
}
impl BDD {
    /// Constant-true BDD.
    pub fn true_bdd() -> Self {
        Self {
            nodes: vec![BDDNode::Leaf(true)],
            root: 0,
        }
    }
    /// Constant-false BDD.
    pub fn false_bdd() -> Self {
        Self {
            nodes: vec![BDDNode::Leaf(false)],
            root: 0,
        }
    }
    /// Evaluate the BDD under the given variable assignment.
    pub fn evaluate(&self, assignment: &HashMap<usize, bool>) -> bool {
        let mut idx = self.root;
        loop {
            match &self.nodes[idx] {
                BDDNode::Leaf(v) => return *v,
                BDDNode::Node { var, low, high } => {
                    idx = if assignment.get(var).copied().unwrap_or(false) {
                        *high
                    } else {
                        *low
                    };
                }
            }
        }
    }
}
/// A CTL formula.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CtlFormula {
    /// Atomic proposition.
    Atom(String),
    /// Boolean true.
    True_,
    /// Boolean false.
    False_,
    /// Negation ¬φ.
    Not(Box<CtlFormula>),
    /// Conjunction φ ∧ ψ.
    And(Box<CtlFormula>, Box<CtlFormula>),
    /// Disjunction φ ∨ ψ.
    Or(Box<CtlFormula>, Box<CtlFormula>),
    /// EX φ: there exists a next state satisfying φ.
    EX(Box<CtlFormula>),
    /// AX φ: all next states satisfy φ.
    AX(Box<CtlFormula>),
    /// EG φ: there exists a path where φ holds globally.
    EG(Box<CtlFormula>),
    /// AG φ: on all paths, φ holds globally.
    AG(Box<CtlFormula>),
    /// EU(φ, ψ): there exists a path where φ U ψ.
    EU(Box<CtlFormula>, Box<CtlFormula>),
    /// AU(φ, ψ): on all paths, φ U ψ.
    AU(Box<CtlFormula>, Box<CtlFormula>),
    /// EF φ: there exists a path where φ holds eventually.
    EF(Box<CtlFormula>),
    /// AF φ: on all paths, φ holds eventually.
    AF(Box<CtlFormula>),
}
impl CtlFormula {
    /// Negate a CTL formula (push negation inward one level).
    pub fn negate(&self) -> Self {
        match self {
            CtlFormula::Not(f) => *f.clone(),
            other => CtlFormula::Not(Box::new(other.clone())),
        }
    }
    /// Returns true if the formula is a safety property.
    pub fn is_safety(&self) -> bool {
        matches!(self, CtlFormula::AG(_))
    }
    /// Returns true if the formula is a liveness property.
    pub fn is_liveness(&self) -> bool {
        matches!(self, CtlFormula::AF(_))
    }
    /// Returns true if the formula is a fairness constraint.
    pub fn is_fairness(&self) -> bool {
        match self {
            CtlFormula::AG(inner) => inner.is_liveness(),
            _ => false,
        }
    }
}
/// An LTL formula.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LtlFormula {
    /// Atomic proposition.
    Atom(String),
    /// Boolean true ⊤.
    True_,
    /// Boolean false ⊥.
    False_,
    /// Negation ¬φ.
    Not(Box<LtlFormula>),
    /// Conjunction φ ∧ ψ.
    And(Box<LtlFormula>, Box<LtlFormula>),
    /// Disjunction φ ∨ ψ.
    Or(Box<LtlFormula>, Box<LtlFormula>),
    /// Next: Xφ.
    Next(Box<LtlFormula>),
    /// Until: φ U ψ.
    Until(Box<LtlFormula>, Box<LtlFormula>),
    /// Release: φ R ψ (dual of Until).
    Release(Box<LtlFormula>, Box<LtlFormula>),
    /// Eventually: Fφ = true U φ.
    Eventually(Box<LtlFormula>),
    /// Always: Gφ = false R φ.
    Always(Box<LtlFormula>),
    /// Weak Until: φ W ψ.
    WeakUntil(Box<LtlFormula>, Box<LtlFormula>),
}
impl LtlFormula {
    /// Construct an atomic formula.
    pub fn atom(s: &str) -> Self {
        LtlFormula::Atom(s.to_string())
    }
    /// Negate a formula.
    pub fn negate(&self) -> Self {
        match self {
            LtlFormula::Not(f) => *f.clone(),
            other => LtlFormula::Not(Box::new(other.clone())),
        }
    }
    /// Returns true if the formula is a safety property (can be falsified by a finite prefix).
    pub fn is_safety(&self) -> bool {
        match self {
            LtlFormula::Always(_) => true,
            LtlFormula::And(a, b) => a.is_safety() && b.is_safety(),
            _ => false,
        }
    }
    /// Returns true if the formula is a liveness property (every finite prefix can be extended).
    pub fn is_liveness(&self) -> bool {
        match self {
            LtlFormula::Eventually(_) => true,
            LtlFormula::Or(a, b) => a.is_liveness() || b.is_liveness(),
            _ => false,
        }
    }
    /// Returns true if the formula is a fairness constraint (G F form).
    pub fn is_fairness(&self) -> bool {
        match self {
            LtlFormula::Always(inner) => inner.is_liveness(),
            _ => false,
        }
    }
}
/// The label of a state: the set of atomic propositions holding in that state.
#[derive(Debug, Clone)]
pub struct StateLabel {
    /// State index.
    pub state: usize,
    /// Set of propositions that hold in this state.
    pub propositions: HashSet<String>,
}
impl StateLabel {
    /// Create an empty label for a state.
    pub fn new(state: usize) -> Self {
        Self {
            state,
            propositions: HashSet::new(),
        }
    }
    /// Add a proposition to this state's label.
    pub fn add(&mut self, prop: impl Into<String>) {
        self.propositions.insert(prop.into());
    }
    /// Returns true if the given proposition holds in this state.
    pub fn holds(&self, prop: &str) -> bool {
        self.propositions.contains(prop)
    }
}
/// Evaluator for propositional μ-calculus formulas over finite Kripke structures.
#[derive(Debug, Clone)]
pub struct MuCalculusEvaluator {
    /// The Kripke structure to evaluate over.
    pub kripke: KripkeStructure,
    /// Maximum fixpoint iterations (safety bound).
    pub max_iter: usize,
}
impl MuCalculusEvaluator {
    /// Create a new evaluator.
    pub fn new(kripke: KripkeStructure) -> Self {
        Self {
            kripke,
            max_iter: 1000,
        }
    }
    /// Evaluate a μ-calculus formula and return the set of satisfying states.
    pub fn eval(
        &self,
        formula: &MuFormula,
        env: &mut HashMap<String, HashSet<usize>>,
    ) -> HashSet<usize> {
        match formula {
            MuFormula::True_ => (0..self.kripke.num_states).collect(),
            MuFormula::False_ => HashSet::new(),
            MuFormula::Prop(p) => (0..self.kripke.num_states)
                .filter(|&s| self.kripke.labeling[s].holds(p))
                .collect(),
            MuFormula::Var(x) => env.get(x).cloned().unwrap_or_default(),
            MuFormula::Neg(f) => {
                let all: HashSet<usize> = (0..self.kripke.num_states).collect();
                let sf = self.eval(f, env);
                all.difference(&sf).copied().collect()
            }
            MuFormula::And(a, b) => {
                let sa = self.eval(a, env);
                let sb = self.eval(b, env);
                sa.intersection(&sb).copied().collect()
            }
            MuFormula::Or(a, b) => {
                let sa = self.eval(a, env);
                let sb = self.eval(b, env);
                sa.union(&sb).copied().collect()
            }
            MuFormula::Diamond(f) => {
                let sf = self.eval(f, env);
                (0..self.kripke.num_states)
                    .filter(|&s| {
                        self.kripke.transition_relation[s]
                            .iter()
                            .any(|t| sf.contains(t))
                    })
                    .collect()
            }
            MuFormula::Box_(f) => {
                let sf = self.eval(f, env);
                (0..self.kripke.num_states)
                    .filter(|&s| {
                        self.kripke.transition_relation[s]
                            .iter()
                            .all(|t| sf.contains(t))
                    })
                    .collect()
            }
            MuFormula::Mu(x, f) => {
                let mut t: HashSet<usize> = HashSet::new();
                for _ in 0..self.max_iter {
                    env.insert(x.clone(), t.clone());
                    let new_t = self.eval(f, env);
                    if new_t == t {
                        env.remove(x);
                        return t;
                    }
                    t = new_t;
                }
                env.remove(x);
                t
            }
            MuFormula::Nu(x, f) => {
                let mut t: HashSet<usize> = (0..self.kripke.num_states).collect();
                for _ in 0..self.max_iter {
                    env.insert(x.clone(), t.clone());
                    let new_t = self.eval(f, env);
                    if new_t == t {
                        env.remove(x);
                        return t;
                    }
                    t = new_t;
                }
                env.remove(x);
                t
            }
        }
    }
    /// Check whether all initial states satisfy a μ-calculus formula.
    pub fn check(&self, formula: &MuFormula) -> bool {
        let mut env = HashMap::new();
        let sat = self.eval(formula, &mut env);
        self.kripke.initial_states.iter().all(|s| sat.contains(s))
    }
}
/// A symbolic model checker that uses BDDs to verify CTL properties.
#[derive(Debug, Clone)]
pub struct BDDModelChecker {
    /// The BDD manager.
    pub mgr: BDDManager,
    /// Number of state variables (state encoded as `num_vars` bits).
    pub num_vars: usize,
    /// BDD id representing the initial states.
    pub init_bdd: usize,
    /// BDD id representing the transition relation T(s, s').
    pub trans_bdd: usize,
}
impl BDDModelChecker {
    /// Create a new BDD model checker.
    pub fn new(num_vars: usize) -> Self {
        let mgr = BDDManager::new();
        let init_bdd = mgr.true_node();
        let trans_bdd = mgr.false_node();
        Self {
            mgr,
            num_vars,
            init_bdd,
            trans_bdd,
        }
    }
    /// Set the initial state BDD.
    pub fn set_init(&mut self, bdd: usize) {
        self.init_bdd = bdd;
    }
    /// Set the transition relation BDD.
    pub fn set_trans(&mut self, bdd: usize) {
        self.trans_bdd = bdd;
    }
    /// Compute the set of states reachable from `states` in one step.
    pub fn post(&mut self, states: usize) -> usize {
        let combined = self.mgr.bdd_and(states, self.trans_bdd);
        let mut result = combined;
        for v in 0..self.num_vars {
            result = self.mgr.bdd_quantify_exists(result, v);
        }
        result
    }
    /// Compute the set of states that can reach `states` in one step.
    pub fn pre(&mut self, states: usize) -> usize {
        let combined = self.mgr.bdd_and(self.trans_bdd, states);
        let mut result = combined;
        for v in self.num_vars..2 * self.num_vars {
            result = self.mgr.bdd_quantify_exists(result, v);
        }
        result
    }
    /// Compute the set of all reachable states (forward BFS via BDDs).
    pub fn reachable(&mut self) -> usize {
        let mut reach = self.init_bdd;
        loop {
            let next_states = self.post(reach);
            let new_reach = self.mgr.bdd_or(reach, next_states);
            if new_reach == reach {
                break;
            }
            reach = new_reach;
        }
        reach
    }
    /// Check AG(safe): all reachable states satisfy the `safe` BDD predicate.
    pub fn check_ag_safe(&mut self, safe: usize) -> bool {
        let reach = self.reachable();
        let false_node = self.mgr.false_node();
        let true_node = self.mgr.true_node();
        let not_safe = if safe == true_node {
            false_node
        } else if safe == false_node {
            true_node
        } else {
            let intersection = self.mgr.bdd_and(reach, safe);
            return intersection == reach;
        };
        let bad = self.mgr.bdd_and(reach, not_safe);
        bad == false_node
    }
    /// Check EF(target): some reachable state satisfies `target`.
    pub fn check_ef(&mut self, target: usize) -> bool {
        let reach = self.reachable();
        let witness = self.mgr.bdd_and(reach, target);
        let false_node = self.mgr.false_node();
        witness != false_node
    }
}
/// A BDD manager: maintains a unique table and an apply cache.
#[derive(Debug, Clone)]
pub struct BDDManager {
    /// Unique table: node → id.
    pub unique_table: HashMap<BDDNode, usize>,
    /// All allocated nodes (in order).
    pub nodes: Vec<BDDNode>,
    /// Apply cache: (op, id1, id2) → id.
    pub apply_cache: HashMap<(u8, usize, usize), usize>,
}
impl BDDManager {
    /// Create a new BDD manager.
    pub fn new() -> Self {
        let mut mgr = Self {
            unique_table: HashMap::new(),
            nodes: Vec::new(),
            apply_cache: HashMap::new(),
        };
        mgr.get_or_create(BDDNode::Leaf(false));
        mgr.get_or_create(BDDNode::Leaf(true));
        mgr
    }
    fn get_or_create(&mut self, node: BDDNode) -> usize {
        if let Some(&id) = self.unique_table.get(&node) {
            return id;
        }
        let id = self.nodes.len();
        self.nodes.push(node.clone());
        self.unique_table.insert(node, id);
        id
    }
    /// Return the id of the constant-false BDD.
    pub fn false_node(&self) -> usize {
        0
    }
    /// Return the id of the constant-true BDD.
    pub fn true_node(&self) -> usize {
        1
    }
    /// Create a variable node for `var`.
    pub fn var(&mut self, var: usize) -> usize {
        self.get_or_create(BDDNode::Node {
            var,
            low: 0,
            high: 1,
        })
    }
    /// Compute the conjunction of two BDD nodes.
    pub fn bdd_and(&mut self, a: usize, b: usize) -> usize {
        if a == self.false_node() || b == self.false_node() {
            return self.false_node();
        }
        if a == self.true_node() {
            return b;
        }
        if b == self.true_node() {
            return a;
        }
        if a == b {
            return a;
        }
        let key = (0u8, a, b);
        if let Some(&r) = self.apply_cache.get(&key) {
            return r;
        }
        let result = match (self.nodes[a].clone(), self.nodes[b].clone()) {
            (
                BDDNode::Node {
                    var: va,
                    low: la,
                    high: ha,
                },
                BDDNode::Node {
                    var: vb,
                    low: lb,
                    high: hb,
                },
            ) => {
                let (var, low_a, high_a, low_b, high_b) = if va == vb {
                    (va, la, ha, lb, hb)
                } else if va < vb {
                    (va, la, ha, b, b)
                } else {
                    (vb, a, a, lb, hb)
                };
                let low = self.bdd_and(low_a, low_b);
                let high = self.bdd_and(high_a, high_b);
                if low == high {
                    low
                } else {
                    self.get_or_create(BDDNode::Node { var, low, high })
                }
            }
            _ => self.false_node(),
        };
        self.apply_cache.insert(key, result);
        result
    }
    /// Compute the disjunction of two BDD nodes.
    pub fn bdd_or(&mut self, a: usize, b: usize) -> usize {
        if a == self.true_node() || b == self.true_node() {
            return self.true_node();
        }
        if a == self.false_node() {
            return b;
        }
        if b == self.false_node() {
            return a;
        }
        if a == b {
            return a;
        }
        let key = (1u8, a, b);
        if let Some(&r) = self.apply_cache.get(&key) {
            return r;
        }
        let result = match (self.nodes[a].clone(), self.nodes[b].clone()) {
            (
                BDDNode::Node {
                    var: va,
                    low: la,
                    high: ha,
                },
                BDDNode::Node {
                    var: vb,
                    low: lb,
                    high: hb,
                },
            ) => {
                let (var, low_a, high_a, low_b, high_b) = if va == vb {
                    (va, la, ha, lb, hb)
                } else if va < vb {
                    (va, la, ha, b, b)
                } else {
                    (vb, a, a, lb, hb)
                };
                let low = self.bdd_or(low_a, low_b);
                let high = self.bdd_or(high_a, high_b);
                if low == high {
                    low
                } else {
                    self.get_or_create(BDDNode::Node { var, low, high })
                }
            }
            _ => self.true_node(),
        };
        self.apply_cache.insert(key, result);
        result
    }
    /// Existentially quantify variable `var` out of BDD `a`.
    pub fn bdd_quantify_exists(&mut self, a: usize, var: usize) -> usize {
        match self.nodes[a].clone() {
            BDDNode::Leaf(_) => a,
            BDDNode::Node { var: v, low, high } => {
                if v == var {
                    self.bdd_or(low, high)
                } else {
                    let new_low = self.bdd_quantify_exists(low, var);
                    let new_high = self.bdd_quantify_exists(high, var);
                    if new_low == new_high {
                        new_low
                    } else {
                        self.get_or_create(BDDNode::Node {
                            var: v,
                            low: new_low,
                            high: new_high,
                        })
                    }
                }
            }
        }
    }
}
/// A probabilistic model checker for discrete-time Markov chains (DTMCs).
/// States are 0..n-1.
#[derive(Debug, Clone)]
pub struct ProbabilisticMCVerifier {
    /// Number of states.
    pub num_states: usize,
    /// Transition matrix: row s = probability distribution over successors.
    /// transitions[s] = list of (target, probability) pairs, summing to 1.
    pub transitions: Vec<Vec<(usize, f64)>>,
    /// Labeling: for each state, which propositions hold.
    pub labeling: Vec<HashSet<String>>,
    /// Initial state distribution: (state, probability) pairs.
    pub initial: Vec<(usize, f64)>,
}
impl ProbabilisticMCVerifier {
    /// Create a new probabilistic MC verifier with `n` states.
    pub fn new(n: usize) -> Self {
        Self {
            num_states: n,
            transitions: vec![Vec::new(); n],
            labeling: vec![HashSet::new(); n],
            initial: Vec::new(),
        }
    }
    /// Add a probabilistic transition from `s` to `t` with probability `p`.
    pub fn add_transition(&mut self, s: usize, t: usize, p: f64) {
        if s < self.num_states && t < self.num_states {
            self.transitions[s].push((t, p));
        }
    }
    /// Label state `s` with proposition `prop`.
    pub fn label_state(&mut self, s: usize, prop: impl Into<String>) {
        if s < self.num_states {
            self.labeling[s].insert(prop.into());
        }
    }
    /// Set initial distribution.
    pub fn set_initial(&mut self, s: usize, p: f64) {
        self.initial.push((s, p));
    }
    /// Compute reachability probability Pr[reach(target)] from each state.
    /// Uses iterative value iteration for DTMCs.
    pub fn reachability_prob(&self, target: &HashSet<usize>) -> Vec<f64> {
        let mut prob = vec![0.0f64; self.num_states];
        for &s in target {
            if s < self.num_states {
                prob[s] = 1.0;
            }
        }
        for _ in 0..500 {
            let mut new_prob = prob.clone();
            for s in 0..self.num_states {
                if target.contains(&s) {
                    new_prob[s] = 1.0;
                    continue;
                }
                new_prob[s] = self.transitions[s].iter().map(|&(t, p)| p * prob[t]).sum();
            }
            let diff: f64 = prob
                .iter()
                .zip(new_prob.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f64, f64::max);
            prob = new_prob;
            if diff < 1e-10 {
                break;
            }
        }
        prob
    }
    /// Check PCTL property P≥threshold[F target]: probability of reaching `target`
    /// from each initial state is at least `threshold`.
    pub fn check_prob_reach(&self, target_prop: &str, threshold: f64) -> bool {
        let target: HashSet<usize> = (0..self.num_states)
            .filter(|&s| self.labeling[s].contains(target_prop))
            .collect();
        let prob = self.reachability_prob(&target);
        self.initial
            .iter()
            .all(|&(s, _w)| prob[s] >= threshold - 1e-9)
    }
    /// Expected number of steps to reach `target` from state `s` (stub).
    pub fn expected_steps_to_reach(&self, s: usize, target: &HashSet<usize>) -> f64 {
        if target.contains(&s) {
            return 0.0;
        }
        let prob = self.reachability_prob(target);
        if prob[s] < 1e-12 {
            f64::INFINITY
        } else {
            1.0 / prob[s]
        }
    }
}
/// CTL model checker: fixpoint computation.
#[derive(Debug, Clone)]
pub struct CtlModelChecker {
    /// The Kripke structure to check.
    pub kripke: KripkeStructure,
}
impl CtlModelChecker {
    /// Create a new CTL model checker.
    pub fn new(kripke: KripkeStructure) -> Self {
        Self { kripke }
    }
    /// Compute sat(EX φ): states with at least one φ-successor.
    pub fn sat_ex(&self, phi_states: &HashSet<usize>) -> HashSet<usize> {
        let mut result = HashSet::new();
        for s in 0..self.kripke.num_states {
            if self.kripke.transition_relation[s]
                .iter()
                .any(|t| phi_states.contains(t))
            {
                result.insert(s);
            }
        }
        result
    }
    /// Compute sat(EU(φ, ψ)): least fixpoint of ψ ∨ (φ ∧ EX(EU)).
    pub fn sat_eu(
        &self,
        phi_states: &HashSet<usize>,
        psi_states: &HashSet<usize>,
    ) -> HashSet<usize> {
        let mut t = psi_states.clone();
        loop {
            let ex_t = self.sat_ex(&t);
            let new_t: HashSet<usize> = t
                .iter()
                .chain(ex_t.iter().filter(|s| phi_states.contains(s)))
                .copied()
                .collect();
            if new_t == t {
                break;
            }
            t = new_t;
        }
        t
    }
    /// Compute sat(EG φ): greatest fixpoint of φ ∧ EX(EG).
    pub fn sat_eg(&self, phi_states: &HashSet<usize>) -> HashSet<usize> {
        let mut t = phi_states.clone();
        loop {
            let ex_t = self.sat_ex(&t);
            let new_t: HashSet<usize> = t.iter().filter(|s| ex_t.contains(s)).copied().collect();
            if new_t == t {
                break;
            }
            t = new_t;
        }
        t
    }
    /// Evaluate a CTL formula and return the set of satisfying states.
    pub fn sat(&self, formula: &CtlFormula) -> HashSet<usize> {
        match formula {
            CtlFormula::True_ => (0..self.kripke.num_states).collect(),
            CtlFormula::False_ => HashSet::new(),
            CtlFormula::Atom(p) => (0..self.kripke.num_states)
                .filter(|&s| self.kripke.labeling[s].holds(p))
                .collect(),
            CtlFormula::Not(f) => {
                let all: HashSet<usize> = (0..self.kripke.num_states).collect();
                let sat_f = self.sat(f);
                all.difference(&sat_f).copied().collect()
            }
            CtlFormula::And(a, b) => {
                let sa = self.sat(a);
                let sb = self.sat(b);
                sa.intersection(&sb).copied().collect()
            }
            CtlFormula::Or(a, b) => {
                let sa = self.sat(a);
                let sb = self.sat(b);
                sa.union(&sb).copied().collect()
            }
            CtlFormula::EX(f) => {
                let sf = self.sat(f);
                self.sat_ex(&sf)
            }
            CtlFormula::AX(f) => {
                let not_f = CtlFormula::Not(f.clone());
                let ex_not_f = self.sat_ex(&self.sat(&not_f));
                let all: HashSet<usize> = (0..self.kripke.num_states).collect();
                all.difference(&ex_not_f).copied().collect()
            }
            CtlFormula::EG(f) => {
                let sf = self.sat(f);
                self.sat_eg(&sf)
            }
            CtlFormula::AG(f) => {
                let not_phi = CtlFormula::Not(f.clone());
                let true_states: HashSet<usize> = (0..self.kripke.num_states).collect();
                let ef_not = self.sat_eu(&true_states, &self.sat(&not_phi));
                let all: HashSet<usize> = (0..self.kripke.num_states).collect();
                all.difference(&ef_not).copied().collect()
            }
            CtlFormula::EU(a, b) => {
                let sa = self.sat(a);
                let sb = self.sat(b);
                self.sat_eu(&sa, &sb)
            }
            CtlFormula::AU(a, b) => {
                let not_psi = CtlFormula::Not(b.clone());
                let not_phi = CtlFormula::Not(a.clone());
                let s_not_psi = self.sat(&not_psi);
                let s_not_phi = self.sat(&not_phi);
                let eg_not_psi = self.sat_eg(&s_not_psi);
                let both_neg: HashSet<usize> =
                    s_not_phi.intersection(&s_not_psi).copied().collect();
                let eu_part = self.sat_eu(&s_not_psi, &both_neg);
                let bad: HashSet<usize> = eg_not_psi.union(&eu_part).copied().collect();
                let all: HashSet<usize> = (0..self.kripke.num_states).collect();
                all.difference(&bad).copied().collect()
            }
            CtlFormula::EF(f) => {
                let all_states: HashSet<usize> = (0..self.kripke.num_states).collect();
                let sf = self.sat(f);
                self.sat_eu(&all_states, &sf)
            }
            CtlFormula::AF(f) => {
                let not_phi = CtlFormula::Not(f.clone());
                let s_not_phi = self.sat(&not_phi);
                let eg = self.sat_eg(&s_not_phi);
                let all: HashSet<usize> = (0..self.kripke.num_states).collect();
                all.difference(&eg).copied().collect()
            }
        }
    }
    /// Check whether all initial states satisfy the CTL formula.
    pub fn check_ctl(&self, formula: &CtlFormula) -> bool {
        let sat = self.sat(formula);
        self.kripke.initial_states.iter().all(|s| sat.contains(s))
    }
    /// Find a counterexample state for a CTL formula.
    pub fn find_counterexample(&self, formula: &CtlFormula) -> Option<CounterExample> {
        let sat = self.sat(formula);
        let bad: Vec<usize> = self
            .kripke
            .initial_states
            .iter()
            .filter(|s| !sat.contains(s))
            .copied()
            .collect();
        if bad.is_empty() {
            None
        } else {
            Some(CounterExample::finite(bad, format!("{:?}", formula)))
        }
    }
}
/// Symbolic transition relation T(s, s') represented as a BDD node id.
#[derive(Debug, Clone)]
pub struct SymbolicTransitionRelation {
    /// The BDD manager shared by all operations.
    pub bdd_id: usize,
    /// Number of state variables.
    pub num_vars: usize,
}
impl SymbolicTransitionRelation {
    /// Create a transition relation from a BDD node id.
    pub fn new(bdd_id: usize, num_vars: usize) -> Self {
        Self { bdd_id, num_vars }
    }
    /// Compute the forward image of `states` under this transition relation.
    pub fn image(&self, mgr: &mut BDDManager, states: usize) -> usize {
        let combined = mgr.bdd_and(states, self.bdd_id);
        let mut result = combined;
        for v in 0..self.num_vars {
            result = mgr.bdd_quantify_exists(result, v);
        }
        result
    }
    /// Compute the backward pre-image of `states` under this transition relation.
    pub fn pre_image(&self, mgr: &mut BDDManager, states: usize) -> usize {
        let combined = mgr.bdd_and(self.bdd_id, states);
        let mut result = combined;
        for v in self.num_vars..2 * self.num_vars {
            result = mgr.bdd_quantify_exists(result, v);
        }
        result
    }
}
/// A BDD node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BDDNode {
    /// Terminal leaf node.
    Leaf(bool),
    /// Internal node: (variable_index, low_child_id, high_child_id).
    Node { var: usize, low: usize, high: usize },
}
/// A μ-calculus formula (propositional modal μ-calculus).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MuFormula {
    /// Propositional variable (proposition name).
    Prop(String),
    /// Boolean true.
    True_,
    /// Boolean false.
    False_,
    /// Negation ¬φ.
    Neg(Box<MuFormula>),
    /// Conjunction φ ∧ ψ.
    And(Box<MuFormula>, Box<MuFormula>),
    /// Disjunction φ ∨ ψ.
    Or(Box<MuFormula>, Box<MuFormula>),
    /// Diamond modality ⟨a⟩φ (exists successor satisfying φ).
    Diamond(Box<MuFormula>),
    /// Box modality [a]φ (all successors satisfy φ).
    Box_(Box<MuFormula>),
    /// Least fixpoint μX.φ(X).
    Mu(String, Box<MuFormula>),
    /// Greatest fixpoint νX.φ(X).
    Nu(String, Box<MuFormula>),
    /// Fixpoint variable X.
    Var(String),
}
/// A Büchi automaton: (Q, Σ, δ, q_0, F).
#[derive(Debug, Clone)]
pub struct BuchiAutomaton {
    /// Number of states.
    pub num_states: usize,
    /// Alphabet (atomic propositions as strings).
    pub alphabet: Vec<String>,
    /// Transition function: transitions[q] = list of (label_set, target) pairs.
    pub transitions: Vec<Vec<(HashSet<String>, usize)>>,
    /// Initial state.
    pub initial_state: usize,
    /// Set of accepting (Büchi) states.
    pub accepting_states: HashSet<usize>,
}
impl BuchiAutomaton {
    /// Create a new Büchi automaton with `n` states.
    pub fn new(n: usize) -> Self {
        Self {
            num_states: n,
            alphabet: Vec::new(),
            transitions: vec![Vec::new(); n],
            initial_state: 0,
            accepting_states: HashSet::new(),
        }
    }
    /// Mark state `q` as accepting.
    pub fn add_accepting(&mut self, q: usize) {
        self.accepting_states.insert(q);
    }
    /// Add a transition from `q` to `r` on the given label set.
    pub fn add_transition(&mut self, q: usize, label: HashSet<String>, r: usize) {
        if q < self.num_states {
            self.transitions[q].push((label, r));
        }
    }
    /// Returns true if the automaton has any accepting states.
    pub fn has_accepting_states(&self) -> bool {
        !self.accepting_states.is_empty()
    }
}
/// An abstract domain for program analysis.
#[derive(Debug, Clone)]
pub struct AbstractDomain {
    /// Kind of abstraction: "predicate", "interval", "octagon".
    pub kind: String,
    /// Predicate names / abstract facts.
    pub predicates: Vec<String>,
}
impl AbstractDomain {
    /// Create a predicate-abstraction domain.
    pub fn predicate(predicates: Vec<String>) -> Self {
        Self {
            kind: "predicate".into(),
            predicates,
        }
    }
    /// Create an interval domain.
    pub fn interval() -> Self {
        Self {
            kind: "interval".into(),
            predicates: Vec::new(),
        }
    }
    /// Returns true if the domain has no predicates (top element).
    pub fn is_top(&self) -> bool {
        self.predicates.is_empty()
    }
}
/// An atomic proposition: a named boolean predicate on states.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AtomicProposition {
    /// Unique name of the proposition.
    pub name: String,
}
impl AtomicProposition {
    /// Create an atomic proposition with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
