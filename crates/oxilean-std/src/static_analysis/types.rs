//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

/// Eraser algorithm state for a single memory location.
#[derive(Debug, Clone)]
pub struct EraserState {
    /// Current candidate lock set C(v): intersection of all observed lock sets.
    pub candidate_set: Option<BTreeSet<usize>>,
    /// Whether a data race has been reported.
    pub race_reported: bool,
    /// Last writer thread.
    pub last_writer: Option<usize>,
    /// Last access was a write.
    pub has_write: bool,
}
impl EraserState {
    pub fn new() -> Self {
        Self {
            candidate_set: None,
            race_reported: false,
            last_writer: None,
            has_write: false,
        }
    }
    /// Record an access from `thread` holding `locks`.
    /// `is_write` is true if this is a write access.
    pub fn observe_access(&mut self, thread: usize, locks: &BTreeSet<usize>, is_write: bool) {
        self.candidate_set = Some(match &self.candidate_set {
            None => locks.clone(),
            Some(prev) => prev.intersection(locks).copied().collect(),
        });
        if let Some(ref cs) = self.candidate_set {
            if cs.is_empty() && (is_write || self.has_write) {
                if self.last_writer.is_some_and(|w| w != thread) || is_write {
                    self.race_reported = true;
                }
            }
        }
        if is_write {
            self.has_write = true;
            self.last_writer = Some(thread);
        }
    }
    /// Has a potential race been detected?
    pub fn has_race(&self) -> bool {
        self.race_reported
    }
}
/// A generic monotone framework fixpoint solver.
/// Each node in the program has an associated abstract value.
pub struct FixpointSolver<V: Clone + PartialEq> {
    /// Number of program points.
    pub num_nodes: usize,
    /// Current abstract values.
    pub values: Vec<V>,
    /// CFG edges: edges[n] = successors of n.
    pub edges: Vec<Vec<usize>>,
    /// Bottom element.
    pub bottom: V,
    /// Join function.
    pub join: fn(&V, &V) -> V,
    /// Transfer function: transfer(node, value_in) -> value_out.
    pub transfer: fn(usize, &V) -> V,
}
impl<V: Clone + PartialEq> FixpointSolver<V> {
    pub fn new(
        num_nodes: usize,
        edges: Vec<Vec<usize>>,
        bottom: V,
        join: fn(&V, &V) -> V,
        transfer: fn(usize, &V) -> V,
    ) -> Self {
        let values = vec![bottom.clone(); num_nodes];
        Self {
            num_nodes,
            values,
            edges,
            bottom,
            join,
            transfer,
        }
    }
    /// Run worklist fixpoint iteration.
    pub fn solve(&mut self) {
        let mut worklist: VecDeque<usize> = (0..self.num_nodes).collect();
        while let Some(n) = worklist.pop_front() {
            let out = (self.transfer)(n, &self.values[n]);
            for &succ in &self.edges[n].clone() {
                let new_val = (self.join)(&self.values[succ], &out);
                if new_val != self.values[succ] {
                    self.values[succ] = new_val;
                    worklist.push_back(succ);
                }
            }
        }
    }
}
/// Security label for two-point lattice: Low < High.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    Low,
    High,
}
impl SecurityLevel {
    /// Join in the security lattice.
    pub fn join(&self, other: &SecurityLevel) -> SecurityLevel {
        if *self == SecurityLevel::High || *other == SecurityLevel::High {
            SecurityLevel::High
        } else {
            SecurityLevel::Low
        }
    }
    /// Check if `self` can flow to `other` (self ⊑ other).
    pub fn can_flow_to(&self, other: &SecurityLevel) -> bool {
        *self <= *other
    }
}
/// Tracks information flow labels for a set of variables.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct IFCTracker {
    /// Security label for each variable.
    pub labels: std::collections::HashMap<String, SecurityLevel>,
    /// Recorded violations: (var, expected_label).
    pub violations: Vec<(String, SecurityLevel)>,
}
impl IFCTracker {
    /// Create a new tracker.
    pub fn new() -> Self {
        Self::default()
    }
    /// Assign a security label to a variable.
    pub fn assign(&mut self, var: impl Into<String>, level: SecurityLevel) {
        self.labels.insert(var.into(), level);
    }
    /// Get the label of a variable (defaults to Low if unknown).
    pub fn label_of(&self, var: &str) -> SecurityLevel {
        self.labels.get(var).cloned().unwrap_or(SecurityLevel::Low)
    }
    /// Propagate: `dst` gets the join of all source labels.
    pub fn propagate(&mut self, dst: impl Into<String>, srcs: &[&str]) {
        let joined = srcs
            .iter()
            .fold(SecurityLevel::Low, |acc, &s| acc.join(&self.label_of(s)));
        let dst = dst.into();
        self.labels.insert(dst, joined);
    }
    /// Check a flow: assert that `var`'s label ⊑ `required`.
    pub fn check_flow(&mut self, var: &str, required: &SecurityLevel) {
        let lbl = self.label_of(var);
        if !lbl.can_flow_to(required) {
            self.violations.push((var.to_string(), required.clone()));
        }
    }
    /// Returns true if any information flow violation was detected.
    pub fn has_violation(&self) -> bool {
        !self.violations.is_empty()
    }
}
/// A simple constant propagation state: maps variables to known constant values
/// or marks them as non-constant (⊤).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ConstPropState {
    /// None = unknown/non-constant (⊤); Some(v) = definitely v.
    pub values: std::collections::HashMap<String, Option<i64>>,
}
impl ConstPropState {
    /// Create a new state.
    pub fn new() -> Self {
        Self::default()
    }
    /// Define a variable as a known constant.
    pub fn set_const(&mut self, var: impl Into<String>, val: i64) {
        self.values.insert(var.into(), Some(val));
    }
    /// Mark a variable as non-constant.
    pub fn set_top(&mut self, var: impl Into<String>) {
        self.values.insert(var.into(), None);
    }
    /// Get the constant value of a variable, if known.
    pub fn get(&self, var: &str) -> Option<i64> {
        self.values.get(var).copied().flatten()
    }
    /// Join two states (meet of constant info: agree ⇒ keep, disagree ⇒ top).
    pub fn join(&self, other: &ConstPropState) -> ConstPropState {
        let mut result = ConstPropState::new();
        for (var, &val) in &self.values {
            let merged = match (val, other.values.get(var).copied().flatten()) {
                (Some(a), Some(b)) if a == b => Some(a),
                _ => None,
            };
            result.values.insert(var.clone(), merged);
        }
        for var in other.values.keys() {
            if !result.values.contains_key(var) {
                result.values.insert(var.clone(), None);
            }
        }
        result
    }
    /// Attempt constant folding of a binary add expression.
    pub fn fold_add(&self, lhs: &str, rhs: &str) -> Option<i64> {
        self.get(lhs)?.checked_add(self.get(rhs)?)
    }
}
/// An abstract interval element: either `[lo, hi]` or ⊥ (empty).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Interval {
    Bottom,
    Range(i64, i64),
}
impl Interval {
    /// Bottom element ⊥.
    pub fn bot() -> Self {
        Interval::Bottom
    }
    /// Top element ⊤ = [-∞, +∞] (represented with i64 extremes).
    pub fn top() -> Self {
        Interval::Range(i64::MIN, i64::MAX)
    }
    /// A single value.
    pub fn single(v: i64) -> Self {
        Interval::Range(v, v)
    }
    /// Partial order: `self ⊑ other`.
    pub fn leq(&self, other: &Self) -> bool {
        match (self, other) {
            (Interval::Bottom, _) => true,
            (_, Interval::Bottom) => false,
            (Interval::Range(a, b), Interval::Range(c, d)) => *a >= *c && *b <= *d,
        }
    }
    /// Join operator ⊔.
    pub fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bottom, x) | (x, Interval::Bottom) => x.clone(),
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                Interval::Range((*a).min(*c), (*b).max(*d))
            }
        }
    }
    /// Meet operator ⊓.
    pub fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bottom, _) | (_, Interval::Bottom) => Interval::Bottom,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let lo = (*a).max(*c);
                let hi = (*b).min(*d);
                if lo > hi {
                    Interval::Bottom
                } else {
                    Interval::Range(lo, hi)
                }
            }
        }
    }
    /// Widening operator ▽.
    pub fn widen(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bottom, x) => x.clone(),
            (x, Interval::Bottom) => x.clone(),
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let lo = if *c < *a { i64::MIN } else { *a };
                let hi = if *d > *b { i64::MAX } else { *b };
                Interval::Range(lo, hi)
            }
        }
    }
    /// Narrowing operator △.
    pub fn narrow(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bottom, _) => Interval::Bottom,
            (x, Interval::Bottom) => x.clone(),
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                let lo = if *a == i64::MIN { *c } else { *a };
                let hi = if *b == i64::MAX { *d } else { *b };
                if lo > hi {
                    Interval::Bottom
                } else {
                    Interval::Range(lo, hi)
                }
            }
        }
    }
    /// Arithmetic add abstraction.
    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bottom, _) | (_, Interval::Bottom) => Interval::Bottom,
            (Interval::Range(a, b), Interval::Range(c, d)) => {
                Interval::Range(a.saturating_add(*c), b.saturating_add(*d))
            }
        }
    }
    /// Contains a concrete value.
    pub fn contains(&self, v: i64) -> bool {
        match self {
            Interval::Bottom => false,
            Interval::Range(lo, hi) => *lo <= v && v <= *hi,
        }
    }
}
/// A simple variable-level taint tracking state.
#[derive(Debug, Clone, Default)]
pub struct TaintState {
    /// Tainted variables.
    pub tainted: HashSet<String>,
    /// Sanitized variables (explicitly cleaned).
    pub sanitized: HashSet<String>,
}
impl TaintState {
    pub fn new() -> Self {
        Self::default()
    }
    /// Mark a variable as a taint source.
    pub fn add_source(&mut self, var: impl Into<String>) {
        let v = var.into();
        self.sanitized.remove(&v);
        self.tainted.insert(v);
    }
    /// Mark a variable as sanitized.
    pub fn sanitize(&mut self, var: &str) {
        self.tainted.remove(var);
        self.sanitized.insert(var.to_string());
    }
    /// Propagate taint: `dst` is tainted if any of `srcs` is tainted.
    pub fn propagate(&mut self, dst: impl Into<String>, srcs: &[&str]) {
        let dst = dst.into();
        let tainted = srcs.iter().any(|&s| self.tainted.contains(s));
        if tainted {
            self.sanitized.remove(&dst);
            self.tainted.insert(dst);
        } else {
            self.tainted.remove(&dst);
        }
    }
    /// Check for a taint violation: `var` is tainted and reaches a sink.
    pub fn violates(&self, var: &str) -> bool {
        self.tainted.contains(var) && !self.sanitized.contains(var)
    }
    /// Join two taint states (union of tainted sets).
    pub fn join(&self, other: &TaintState) -> TaintState {
        TaintState {
            tainted: self.tainted.union(&other.tainted).cloned().collect(),
            sanitized: self
                .sanitized
                .intersection(&other.sanitized)
                .cloned()
                .collect(),
        }
    }
}
/// Represents a finite-state automaton for resource protocol checking.
/// States are `usize` indices; transitions are labeled by operation names.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TypestateAutomaton {
    /// Number of states.
    pub num_states: usize,
    /// Initial state.
    pub initial: usize,
    /// Accepting (final) states.
    pub accepting: BTreeSet<usize>,
    /// Transition table: transitions[state][op_name] = next_state.
    pub transitions: Vec<std::collections::BTreeMap<String, usize>>,
    /// Error states (invalid transitions recorded here).
    pub error_states: BTreeSet<usize>,
}
impl TypestateAutomaton {
    /// Construct an automaton with `n` states.
    pub fn new(num_states: usize, initial: usize) -> Self {
        Self {
            num_states,
            initial,
            accepting: BTreeSet::new(),
            transitions: vec![std::collections::BTreeMap::new(); num_states],
            error_states: BTreeSet::new(),
        }
    }
    /// Add a transition: from `from` via `op` to `to`.
    pub fn add_transition(&mut self, from: usize, op: &str, to: usize) {
        self.transitions[from].insert(op.to_string(), to);
    }
    /// Mark a state as accepting.
    pub fn set_accepting(&mut self, state: usize) {
        self.accepting.insert(state);
    }
    /// Simulate an operation sequence; return the final state or `None` on invalid transition.
    pub fn simulate(&self, ops: &[&str]) -> Option<usize> {
        let mut state = self.initial;
        for &op in ops {
            match self.transitions[state].get(op) {
                Some(&next) => state = next,
                None => return None,
            }
        }
        Some(state)
    }
    /// Check whether an operation sequence is accepted (ends in accepting state).
    pub fn accepts(&self, ops: &[&str]) -> bool {
        self.simulate(ops)
            .map_or(false, |s| self.accepting.contains(&s))
    }
    /// Check whether an operation sequence violates the protocol.
    pub fn violates(&self, ops: &[&str]) -> bool {
        self.simulate(ops).is_none()
    }
}
/// A simplified Andersen points-to analysis using a constraint worklist.
#[derive(Debug, Clone, Default)]
pub struct AndersenPTA {
    /// Number of variables.
    pub num_vars: usize,
    /// Points-to sets: pts[v] = set of allocation sites.
    pub pts: Vec<BTreeSet<usize>>,
    /// Copy constraints: copy_edges[a] = {b | pts(a) ⊆ pts(b)}.
    pub copy_edges: Vec<BTreeSet<usize>>,
    /// Store constraints: store[a] = set of (src, field).
    pub store: Vec<Vec<(usize, usize)>>,
    /// Load constraints: load[a] = set of (dst, field).
    pub load: Vec<Vec<(usize, usize)>>,
}
impl AndersenPTA {
    pub fn new(num_vars: usize) -> Self {
        Self {
            num_vars,
            pts: vec![BTreeSet::new(); num_vars],
            copy_edges: vec![BTreeSet::new(); num_vars],
            store: vec![vec![]; num_vars],
            load: vec![vec![]; num_vars],
        }
    }
    /// Add: `a = alloc()` — variable `a` points to allocation site `site`.
    pub fn add_alloc(&mut self, a: usize, site: usize) {
        self.pts[a].insert(site);
    }
    /// Add copy constraint: `b = a` (pts(a) ⊆ pts(b)).
    pub fn add_copy(&mut self, src: usize, dst: usize) {
        self.copy_edges[src].insert(dst);
    }
    /// Solve: propagate points-to sets to a fixpoint.
    pub fn solve(&mut self) {
        let mut worklist: VecDeque<usize> = (0..self.num_vars).collect();
        while let Some(v) = worklist.pop_front() {
            let pts_v: Vec<usize> = self.pts[v].iter().copied().collect();
            for &dst in &self.copy_edges[v].clone() {
                let added: Vec<usize> = pts_v
                    .iter()
                    .filter(|&&s| self.pts[dst].insert(s))
                    .copied()
                    .collect();
                if !added.is_empty() {
                    worklist.push_back(dst);
                }
            }
        }
    }
    /// Query: may `a` and `b` alias?
    pub fn may_alias(&self, a: usize, b: usize) -> bool {
        !self.pts[a].is_disjoint(&self.pts[b])
    }
}
/// A simplified program dependence graph over statement indices.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PDGraph {
    /// Number of statements.
    pub num_stmts: usize,
    /// Data dependence edges: data_edges[s] = set of statements s flows data to.
    pub data_edges: Vec<BTreeSet<usize>>,
    /// Control dependence edges: ctrl_edges[s] = statements s controls.
    pub ctrl_edges: Vec<BTreeSet<usize>>,
}
impl PDGraph {
    /// Create a PDG for `n` statements.
    pub fn new(num_stmts: usize) -> Self {
        Self {
            num_stmts,
            data_edges: vec![BTreeSet::new(); num_stmts],
            ctrl_edges: vec![BTreeSet::new(); num_stmts],
        }
    }
    /// Add a data dependence edge from `src` to `dst`.
    pub fn add_data_edge(&mut self, src: usize, dst: usize) {
        self.data_edges[src].insert(dst);
    }
    /// Add a control dependence edge from `src` to `dst`.
    pub fn add_ctrl_edge(&mut self, src: usize, dst: usize) {
        self.ctrl_edges[src].insert(dst);
    }
    /// Compute backward slice from a criterion statement.
    /// Returns the set of statement indices that can affect `criterion`.
    pub fn backward_slice(&self, criterion: usize) -> BTreeSet<usize> {
        let mut slice = BTreeSet::new();
        let mut worklist: VecDeque<usize> = VecDeque::new();
        worklist.push_back(criterion);
        while let Some(s) = worklist.pop_front() {
            if slice.insert(s) {
                for src in 0..self.num_stmts {
                    if self.data_edges[src].contains(&s) || self.ctrl_edges[src].contains(&s) {
                        if !slice.contains(&src) {
                            worklist.push_back(src);
                        }
                    }
                }
            }
        }
        slice
    }
    /// Compute forward slice from a criterion statement.
    pub fn forward_slice(&self, criterion: usize) -> BTreeSet<usize> {
        let mut slice = BTreeSet::new();
        let mut worklist: VecDeque<usize> = VecDeque::new();
        worklist.push_back(criterion);
        while let Some(s) = worklist.pop_front() {
            if slice.insert(s) {
                for &dst in &self.data_edges[s] {
                    if !slice.contains(&dst) {
                        worklist.push_back(dst);
                    }
                }
                for &dst in &self.ctrl_edges[s] {
                    if !slice.contains(&dst) {
                        worklist.push_back(dst);
                    }
                }
            }
        }
        slice
    }
}
/// Three-valued nullability: definitely null, definitely non-null, or unknown.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Nullability {
    /// Definitely null.
    Null,
    /// Definitely non-null.
    NonNull,
    /// Unknown / may be null.
    MaybeNull,
}
/// Tracks nullability for program variables.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct NullTracker {
    /// Nullability status per variable.
    pub status: std::collections::HashMap<String, Nullability>,
    /// Variables that caused a null dereference alarm.
    pub alarms: Vec<String>,
}
impl NullTracker {
    /// Create a new null tracker.
    pub fn new() -> Self {
        Self::default()
    }
    /// Declare a variable as definitely non-null (e.g. just allocated).
    pub fn declare_non_null(&mut self, var: impl Into<String>) {
        self.status.insert(var.into(), Nullability::NonNull);
    }
    /// Declare a variable as potentially null (e.g. result of nullable function).
    pub fn declare_maybe_null(&mut self, var: impl Into<String>) {
        self.status.insert(var.into(), Nullability::MaybeNull);
    }
    /// Declare a variable as definitely null.
    pub fn declare_null(&mut self, var: impl Into<String>) {
        self.status.insert(var.into(), Nullability::Null);
    }
    /// Get the nullability of a variable (defaults to MaybeNull if unknown).
    pub fn get(&self, var: &str) -> &Nullability {
        self.status.get(var).unwrap_or(&Nullability::MaybeNull)
    }
    /// Simulate a dereference: raises an alarm if var may be null.
    pub fn dereference(&mut self, var: &str) {
        match self.get(var) {
            Nullability::Null | Nullability::MaybeNull => self.alarms.push(var.to_string()),
            Nullability::NonNull => {}
        }
    }
    /// Join two null-tracker states at a merge point.
    pub fn join(&self, other: &NullTracker) -> NullTracker {
        let mut result = NullTracker::new();
        for (var, lhs) in &self.status {
            let rhs = other.get(var);
            let merged = match (lhs, rhs) {
                (Nullability::NonNull, Nullability::NonNull) => Nullability::NonNull,
                (Nullability::Null, Nullability::Null) => Nullability::Null,
                _ => Nullability::MaybeNull,
            };
            result.status.insert(var.clone(), merged);
        }
        for var in other.status.keys() {
            if !result.status.contains_key(var) {
                result.status.insert(var.clone(), Nullability::MaybeNull);
            }
        }
        result
    }
    /// Returns true if any null-dereference alarm was raised.
    pub fn has_alarm(&self) -> bool {
        !self.alarms.is_empty()
    }
}
/// Abstract sign: {⊥, Neg, Zero, Pos, ⊤}.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sign {
    Bottom,
    Neg,
    Zero,
    Pos,
    Top,
}
impl Sign {
    pub fn of(v: i64) -> Self {
        if v < 0 {
            Sign::Neg
        } else if v == 0 {
            Sign::Zero
        } else {
            Sign::Pos
        }
    }
    pub fn join(&self, other: &Self) -> Self {
        if self == other {
            return self.clone();
        }
        match (self, other) {
            (Sign::Bottom, x) | (x, Sign::Bottom) => x.clone(),
            _ => Sign::Top,
        }
    }
    pub fn leq(&self, other: &Self) -> bool {
        self == &Sign::Bottom || other == &Sign::Top || self == other
    }
    pub fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Sign::Bottom, _) | (_, Sign::Bottom) => Sign::Bottom,
            (Sign::Zero, x) | (x, Sign::Zero) => x.clone(),
            (Sign::Pos, Sign::Pos) => Sign::Pos,
            (Sign::Neg, Sign::Neg) => Sign::Neg,
            _ => Sign::Top,
        }
    }
    pub fn neg(&self) -> Self {
        match self {
            Sign::Pos => Sign::Neg,
            Sign::Neg => Sign::Pos,
            x => x.clone(),
        }
    }
}
