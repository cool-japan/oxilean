//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Expr, Level, Name};
use std::collections::HashMap;

/// Configuration options for the unifier.
#[derive(Clone, Debug)]
pub struct UnifyConfig {
    /// Whether to use occurs check (cyclic detection).
    pub occurs_check: bool,
    /// Maximum recursion depth.
    pub max_depth: u32,
    /// Whether to treat sorts structurally (no universe polymorphism).
    pub structural_sorts: bool,
}
impl UnifyConfig {
    /// Create a config without occurs check (faster but unsound for cyclic terms).
    pub fn without_occurs_check() -> Self {
        Self {
            occurs_check: false,
            ..Self::default()
        }
    }
    /// Create a strict config for syntactic unification only.
    pub fn syntactic() -> Self {
        Self {
            occurs_check: true,
            max_depth: 64,
            structural_sorts: true,
        }
    }
}
/// A single unification constraint.
#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    /// Two expressions must be definitionally equal.
    ExprEq(Expr, Expr),
    /// Two levels must be equal.
    LevelEq(Level, Level),
    /// A level must be less-than-or-equal to another.
    LevelLe(Level, Level),
}
/// A priority for constraint scheduling.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstraintPriority {
    /// Rigid-rigid: can be decomposed immediately.
    RigidRigid = 0,
    /// Flex-rigid: try to assign the metavariable.
    FlexRigid = 1,
    /// Flex-flex: postpone until one side becomes rigid.
    FlexFlex = 2,
}
/// Full unification state, including substitution and pending constraints.
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct UnificationState {
    /// Current metavariable substitution.
    pub subst: Substitution,
    /// Pending constraints.
    pub pending: ConstraintSet,
    /// Constraints postponed for later (flex-flex).
    pub postponed: Vec<Constraint>,
    /// Number of constraints processed so far.
    pub steps: usize,
}
impl UnificationState {
    /// Create a fresh unification state.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an expression equality constraint.
    #[allow(dead_code)]
    pub fn add_eq(&mut self, lhs: Expr, rhs: Expr) {
        self.pending.eq_expr(lhs, rhs);
    }
    /// Add a level equality constraint.
    #[allow(dead_code)]
    pub fn add_level_eq(&mut self, l: Level, r: Level) {
        self.pending.eq_level(l, r);
    }
    /// Assign a metavariable.  Returns an error if it was already assigned
    /// to a different (non-equal) expression.
    #[allow(dead_code)]
    pub fn assign_mvar(&mut self, mvar: u32, expr: Expr) -> Result<(), UnifyError> {
        if let Some(existing) = self.subst.get(mvar) {
            let existing = existing.clone();
            if !structurally_equal(&existing, &expr, &self.subst) {
                return Err(UnifyError::TypeMismatch(existing, expr));
            }
            return Ok(());
        }
        self.subst.insert(mvar, expr);
        Ok(())
    }
    /// Run one step of constraint solving.
    ///
    /// Returns `Ok(true)` if a constraint was consumed, `Ok(false)` if done.
    #[allow(dead_code)]
    pub fn step(&mut self) -> Result<bool, UnifyError> {
        let Some(c) = self.pending.pop() else {
            return Ok(false);
        };
        self.steps += 1;
        self.solve_one(c)?;
        Ok(true)
    }
    /// Run all constraints to completion.
    #[allow(dead_code)]
    pub fn run(&mut self) -> Result<(), UnifyError> {
        while self.step()? {}
        Ok(())
    }
    fn solve_one(&mut self, c: Constraint) -> Result<(), UnifyError> {
        match c {
            Constraint::LevelEq(l1, l2) => {
                if !levels_equal(&normalize_level(&l1), &normalize_level(&l2)) {
                    return Err(UnifyError::LevelMismatch(l1, l2));
                }
                Ok(())
            }
            Constraint::LevelLe(_l1, _l2) => Ok(()),
            Constraint::ExprEq(lhs, rhs) => {
                let lhs = self.subst.apply_recursive(&lhs);
                let rhs = self.subst.apply_recursive(&rhs);
                if structurally_equal_nf(&lhs, &rhs) {
                    return Ok(());
                }
                self.decompose(&lhs, &rhs)
            }
        }
    }
    fn decompose(&mut self, lhs: &Expr, rhs: &Expr) -> Result<(), UnifyError> {
        match (lhs, rhs) {
            (Expr::App(f1, a1), Expr::App(f2, a2)) => {
                self.add_eq(*f1.clone(), *f2.clone());
                self.add_eq(*a1.clone(), *a2.clone());
                Ok(())
            }
            (Expr::Lam(_, _, ty1, b1), Expr::Lam(_, _, ty2, b2)) => {
                self.add_eq(*ty1.clone(), *ty2.clone());
                self.add_eq(*b1.clone(), *b2.clone());
                Ok(())
            }
            (Expr::Pi(_, _, ty1, b1), Expr::Pi(_, _, ty2, b2)) => {
                self.add_eq(*ty1.clone(), *ty2.clone());
                self.add_eq(*b1.clone(), *b2.clone());
                Ok(())
            }
            (Expr::Let(_, ty1, v1, b1), Expr::Let(_, ty2, v2, b2)) => {
                self.add_eq(*ty1.clone(), *ty2.clone());
                self.add_eq(*v1.clone(), *v2.clone());
                self.add_eq(*b1.clone(), *b2.clone());
                Ok(())
            }
            (Expr::Sort(l1), Expr::Sort(l2)) => {
                self.add_level_eq(l1.clone(), l2.clone());
                Ok(())
            }
            (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) if n1 == n2 => {
                for (a, b) in ls1.iter().zip(ls2.iter()) {
                    self.add_level_eq(a.clone(), b.clone());
                }
                Ok(())
            }
            _ => Err(UnifyError::TypeMismatch(lhs.clone(), rhs.clone())),
        }
    }
    /// Return the final substitution (if solving succeeded).
    #[allow(dead_code)]
    pub fn substitution(&self) -> &Substitution {
        &self.subst
    }
    /// Number of postponed flex-flex constraints.
    #[allow(dead_code)]
    pub fn postponed_count(&self) -> usize {
        self.postponed.len()
    }
}
/// Stateful unifier that accumulates a substitution.
///
/// Unlike `unify`, this can assign metavariables and collect deferred
/// constraints for later resolution.
pub struct Unifier {
    /// Accumulated state (substitution + pending constraints).
    pub state: UnifyState,
    /// Whether the unifier should be strict (no deferred constraints).
    pub strict: bool,
}
impl Unifier {
    /// Create a new strict unifier.
    pub fn new() -> Self {
        Self {
            state: UnifyState::new(),
            strict: true,
        }
    }
    /// Create a lenient unifier that defers constraints rather than failing.
    pub fn lenient() -> Self {
        Self {
            state: UnifyState::new(),
            strict: false,
        }
    }
    /// Unify two expressions, updating internal state.
    pub fn unify_exprs(&mut self, lhs: &Expr, rhs: &Expr) -> Result<(), UnifyError> {
        self.state.step()?;
        unify(lhs, rhs)
    }
    /// Defer an expression equality constraint for later resolution.
    pub fn defer(&mut self, lhs: Expr, rhs: Expr) {
        self.state.pending.eq_expr(lhs, rhs);
    }
    /// Solve all pending constraints.
    ///
    /// Returns the remaining unsolved constraints, or an error if
    /// strict mode is enabled and any constraint fails.
    pub fn solve_pending(&mut self) -> Result<Vec<Constraint>, UnifyError> {
        let mut unsolved = Vec::new();
        while let Some(c) = self.state.pending.pop() {
            match &c {
                Constraint::ExprEq(l, r) => match unify(l, r) {
                    Ok(()) => {}
                    Err(e) if self.strict => return Err(e),
                    Err(_) => unsolved.push(c),
                },
                Constraint::LevelEq(l, r) => {
                    if l != r {
                        if self.strict {
                            return Err(UnifyError::LevelMismatch(l.clone(), r.clone()));
                        }
                        unsolved.push(c);
                    }
                }
                Constraint::LevelLe(l, r) => {
                    let ln = oxilean_kernel::level::normalize(l);
                    let rn = oxilean_kernel::level::normalize(r);
                    if !oxilean_kernel::level::is_leq(&ln, &rn) {
                        if self.strict {
                            return Err(UnifyError::LevelMismatch(l.clone(), r.clone()));
                        }
                        unsolved.push(c);
                    }
                }
            }
        }
        Ok(unsolved)
    }
    /// Return the current substitution.
    pub fn substitution(&self) -> &Substitution {
        &self.state.subst
    }
    /// Return the number of steps taken.
    pub fn steps(&self) -> u64 {
        self.state.steps
    }
}
/// A set of pending unification constraints.
#[derive(Debug, Default, Clone)]
pub struct ConstraintSet {
    constraints: Vec<Constraint>,
}
impl ConstraintSet {
    /// Create an empty constraint set.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a constraint.
    pub fn add(&mut self, c: Constraint) {
        self.constraints.push(c);
    }
    /// Add an expression equality constraint.
    pub fn eq_expr(&mut self, lhs: Expr, rhs: Expr) {
        self.add(Constraint::ExprEq(lhs, rhs));
    }
    /// Add a level equality constraint.
    pub fn eq_level(&mut self, l: Level, r: Level) {
        self.add(Constraint::LevelEq(l, r));
    }
    /// Add a level `≤` constraint.
    pub fn le_level(&mut self, l: Level, r: Level) {
        self.add(Constraint::LevelLe(l, r));
    }
    /// Remove and return the next constraint, if any.
    pub fn pop(&mut self) -> Option<Constraint> {
        self.constraints.pop()
    }
    /// Return whether there are no pending constraints.
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
    /// Return the number of pending constraints.
    pub fn len(&self) -> usize {
        self.constraints.len()
    }
    /// Iterate over constraints without consuming them.
    pub fn iter(&self) -> impl Iterator<Item = &Constraint> {
        self.constraints.iter()
    }
    /// Clear all constraints.
    pub fn clear(&mut self) {
        self.constraints.clear();
    }
}
impl ConstraintSet {
    /// Drain all constraints into a Vec, leaving the set empty.
    #[allow(dead_code)]
    pub fn drain(&mut self) -> Vec<Constraint> {
        std::mem::take(&mut self.constraints)
    }
    /// Append all constraints from `other` into this set.
    #[allow(dead_code)]
    pub fn extend(&mut self, other: ConstraintSet) {
        self.constraints.extend(other.constraints);
    }
    /// Count expression-equality constraints.
    #[allow(dead_code)]
    pub fn count_expr_eq(&self) -> usize {
        self.constraints
            .iter()
            .filter(|c| matches!(c, Constraint::ExprEq(..)))
            .count()
    }
    /// Count level constraints.
    #[allow(dead_code)]
    pub fn count_level_constraints(&self) -> usize {
        self.constraints
            .iter()
            .filter(|c| matches!(c, Constraint::LevelEq(..) | Constraint::LevelLe(..)))
            .count()
    }
    /// Retain only constraints satisfying the predicate.
    #[allow(dead_code)]
    pub fn retain<F: FnMut(&Constraint) -> bool>(&mut self, f: F) {
        self.constraints.retain(f);
    }
    /// Peek at the top-most constraint without removing it.
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&Constraint> {
        self.constraints.last()
    }
    /// Insert a constraint at the front (high priority).
    #[allow(dead_code)]
    pub fn push_front(&mut self, c: Constraint) {
        self.constraints.insert(0, c);
    }
}
/// An annotated constraint with its priority.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PrioritizedConstraint {
    /// The underlying constraint.
    pub constraint: Constraint,
    /// Its scheduling priority.
    pub priority: ConstraintPriority,
}
impl PrioritizedConstraint {
    /// Create a prioritized constraint, classifying automatically.
    #[allow(dead_code)]
    pub fn new(constraint: Constraint, subst: &Substitution) -> Self {
        let priority = classify_constraint(&constraint, subst);
        Self {
            constraint,
            priority,
        }
    }
}
/// A tracer that records unification events.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct UnificationTracer {
    events: Vec<TraceEvent>,
    enabled: bool,
}
impl UnificationTracer {
    /// Create a new tracer (disabled by default).
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Enable tracing.
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Disable tracing.
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    /// Record an event (no-op if disabled).
    #[allow(dead_code)]
    pub fn record(&mut self, event: TraceEvent) {
        if self.enabled {
            self.events.push(event);
        }
    }
    /// Return all recorded events.
    #[allow(dead_code)]
    pub fn events(&self) -> &[TraceEvent] {
        &self.events
    }
    /// Count events of a specific kind.
    #[allow(dead_code)]
    pub fn count_assignments(&self) -> usize {
        self.events
            .iter()
            .filter(|e| matches!(e, TraceEvent::AssignMeta { .. }))
            .count()
    }
    /// Count constraint failures.
    #[allow(dead_code)]
    pub fn count_failures(&self) -> usize {
        self.events
            .iter()
            .filter(|e| matches!(e, TraceEvent::FailedConstraint { .. }))
            .count()
    }
    /// Clear all recorded events.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
/// A set of unification constraints to solve.
#[derive(Clone, Debug, Default)]
pub struct ConstraintSet2 {
    constraints: Vec<UnifyConstraint>,
}
impl ConstraintSet2 {
    /// Create an empty constraint set.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a constraint.
    pub fn add(&mut self, c: UnifyConstraint) {
        self.constraints.push(c);
    }
    /// Add a simple lhs ≡ rhs constraint.
    pub fn add_eq(&mut self, lhs: Expr, rhs: Expr) {
        self.add(UnifyConstraint::new(lhs, rhs));
    }
    /// Number of constraints.
    pub fn len(&self) -> usize {
        self.constraints.len()
    }
    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
    /// Remove and return the first constraint.
    pub fn pop(&mut self) -> Option<UnifyConstraint> {
        if self.constraints.is_empty() {
            None
        } else {
            Some(self.constraints.remove(0))
        }
    }
    /// Remove all trivially satisfied constraints.
    pub fn remove_trivial(&mut self) {
        self.constraints.retain(|c| !c.is_trivial());
    }
    /// Iterate over all constraints.
    pub fn iter(&self) -> impl Iterator<Item = &UnifyConstraint> {
        self.constraints.iter()
    }
}
/// Unification error.
#[derive(Clone, Debug, PartialEq)]
pub enum UnifyError {
    /// Types are incompatible
    TypeMismatch(Expr, Expr),
    /// Occurs check failed (cyclic substitution)
    OccursCheck,
    /// Universe level mismatch
    LevelMismatch(Level, Level),
    /// Unsolvable constraint
    Unsolvable(String),
    /// Other error
    Other(String),
}
/// The outcome of a unification attempt.
#[derive(Debug, Clone)]
pub enum UnifyResult {
    /// Unification succeeded with the given substitution.
    Ok(Substitution),
    /// Unification failed with the given error.
    Failed(UnifyError),
    /// Unification was deferred (more information needed).
    Deferred,
}
impl UnifyResult {
    /// Check whether the result is successful.
    pub fn is_ok(&self) -> bool {
        matches!(self, UnifyResult::Ok(_))
    }
    /// Check whether the result is a failure.
    pub fn is_failed(&self) -> bool {
        matches!(self, UnifyResult::Failed(_))
    }
    /// Unwrap the substitution, panicking if failed.
    pub fn unwrap_subst(self) -> Substitution {
        match self {
            UnifyResult::Ok(s) => s,
            _ => panic!("called unwrap_subst on non-Ok UnifyResult"),
        }
    }
}
/// Classify a term as flexible (headed by a metavariable) or rigid.
#[derive(Debug, Clone, PartialEq)]
pub enum Rigidity {
    /// Headed by a metavariable.
    Flex,
    /// Headed by a constant, constructor, or known term.
    Rigid,
    /// Unknown.
    Unknown,
}
/// An event recorded during unification.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TraceEvent {
    /// A constraint was added.
    AddConstraint(Constraint),
    /// A constraint was solved successfully.
    SolvedConstraint(Constraint),
    /// A metavariable was assigned.
    AssignMeta { mvar: u32, expr: Expr },
    /// A constraint failed.
    FailedConstraint {
        constraint: Constraint,
        reason: String,
    },
    /// An eta expansion was performed.
    EtaExpand { original: Expr, expanded: Expr },
}
/// A substitution mapping metavariable indices to expressions.
///
/// Metavariables are represented as `u32` keys.  The substitution is
/// built up incrementally during unification and used to instantiate
/// open terms.
#[derive(Debug, Clone, Default)]
pub struct Substitution {
    map: HashMap<u32, Expr>,
}
impl Substitution {
    /// Create an empty substitution.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a binding.  Returns the previous value if the key was present.
    pub fn insert(&mut self, mvar: u32, expr: Expr) -> Option<Expr> {
        self.map.insert(mvar, expr)
    }
    /// Look up a metavariable.
    pub fn get(&self, mvar: u32) -> Option<&Expr> {
        self.map.get(&mvar)
    }
    /// Follow a chain of metavariable assignments, returning the deepest
    /// non-metavariable expression (or the last metavariable).
    ///
    /// In a real elaborator `Expr::MVar(id)` would be a node; here we use
    /// `BVar(i)` as a stand-in for illustration.
    pub fn chase(&self, expr: &Expr) -> Expr {
        let mut cur = expr.clone();
        loop {
            if let Expr::BVar(i) = &cur {
                if let Some(next) = self.map.get(i) {
                    cur = next.clone();
                    continue;
                }
            }
            return cur;
        }
    }
    /// Apply this substitution to an expression shallowly (top-level only).
    ///
    /// A full implementation would recurse into sub-terms.
    pub fn apply_shallow(&self, expr: &Expr) -> Expr {
        if let Expr::BVar(i) = expr {
            if let Some(replacement) = self.map.get(i) {
                return replacement.clone();
            }
        }
        expr.clone()
    }
    /// Return the number of bindings.
    pub fn len(&self) -> usize {
        self.map.len()
    }
    /// Return whether the substitution is empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    /// Remove a binding.
    pub fn remove(&mut self, mvar: u32) -> Option<Expr> {
        self.map.remove(&mvar)
    }
    /// Merge another substitution into this one.
    ///
    /// Bindings in `other` take precedence on conflicts.
    pub fn merge(&mut self, other: Substitution) {
        self.map.extend(other.map);
    }
    /// Return all bound metavariable indices.
    pub fn domain(&self) -> Vec<u32> {
        self.map.keys().copied().collect()
    }
}
impl Substitution {
    /// Apply the substitution to an expression recursively.
    ///
    /// Unlike `apply_shallow`, this descends into all sub-expressions.
    #[allow(dead_code)]
    pub fn apply_recursive(&self, expr: &Expr) -> Expr {
        if self.map.is_empty() {
            return expr.clone();
        }
        match expr {
            Expr::BVar(i) => {
                if let Some(replacement) = self.map.get(i) {
                    self.apply_recursive(replacement)
                } else {
                    expr.clone()
                }
            }
            Expr::FVar(_) | Expr::Sort(_) | Expr::Lit(_) | Expr::Const(..) => expr.clone(),
            Expr::App(f, a) => Expr::App(
                Box::new(self.apply_recursive(f)),
                Box::new(self.apply_recursive(a)),
            ),
            Expr::Lam(bi, n, ty, body) => Expr::Lam(
                *bi,
                n.clone(),
                Box::new(self.apply_recursive(ty)),
                Box::new(self.apply_recursive(body)),
            ),
            Expr::Pi(bi, n, ty, body) => Expr::Pi(
                *bi,
                n.clone(),
                Box::new(self.apply_recursive(ty)),
                Box::new(self.apply_recursive(body)),
            ),
            Expr::Let(n, ty, val, body) => Expr::Let(
                n.clone(),
                Box::new(self.apply_recursive(ty)),
                Box::new(self.apply_recursive(val)),
                Box::new(self.apply_recursive(body)),
            ),
            Expr::Proj(n, i, inner) => {
                Expr::Proj(n.clone(), *i, Box::new(self.apply_recursive(inner)))
            }
        }
    }
    /// Compose two substitutions: `self ∘ other`.
    ///
    /// The result maps each key in `other` through `self`, and also includes all
    /// bindings in `self` that are not in `other`.
    #[allow(dead_code)]
    pub fn compose(mut self, other: &Substitution) -> Substitution {
        for (k, v) in &other.map {
            if !self.map.contains_key(k) {
                let applied = self.apply_recursive(v);
                self.map.insert(*k, applied);
            }
        }
        self
    }
    /// Restrict the substitution to the given set of keys.
    #[allow(dead_code)]
    pub fn restrict(&self, keys: &[u32]) -> Substitution {
        let map = keys
            .iter()
            .filter_map(|k| self.map.get(k).map(|v| (*k, v.clone())))
            .collect();
        Substitution { map }
    }
    /// Check whether a metavariable is in the domain of this substitution.
    #[allow(dead_code)]
    pub fn contains(&self, mvar: u32) -> bool {
        self.map.contains_key(&mvar)
    }
    /// Return a sorted list of (key, value) pairs (for deterministic output).
    #[allow(dead_code)]
    pub fn sorted_pairs(&self) -> Vec<(u32, &Expr)> {
        let mut pairs: Vec<_> = self.map.iter().map(|(k, v)| (*k, v)).collect();
        pairs.sort_by_key(|(k, _)| *k);
        pairs
    }
    /// Apply the substitution to a level (no-op in this encoding, but present for API symmetry).
    #[allow(dead_code)]
    pub fn apply_level(&self, level: &Level) -> Level {
        level.clone()
    }
}
/// Combined state for a unification session.
///
/// Accumulates metavariable assignments and pending constraints.
#[derive(Debug, Default, Clone)]
pub struct UnifyState {
    /// Current substitution (metavar → expr).
    pub subst: Substitution,
    /// Pending constraints not yet resolved.
    pub pending: ConstraintSet,
    /// Number of unification steps taken (for debugging / fuel limiting).
    pub steps: u64,
    /// Maximum allowed steps (0 = unlimited).
    pub max_steps: u64,
}
impl UnifyState {
    /// Create a fresh unification state with no limits.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a state with an explicit step limit.
    pub fn with_limit(max_steps: u64) -> Self {
        Self {
            max_steps,
            ..Self::default()
        }
    }
    /// Attempt to record a metavariable assignment.
    ///
    /// Fails with `OccursCheck` if `mvar == rhs` (trivially cyclic).
    pub fn assign(&mut self, mvar: u32, rhs: Expr) -> Result<(), UnifyError> {
        if let Expr::BVar(i) = &rhs {
            if *i == mvar {
                return Err(UnifyError::OccursCheck);
            }
        }
        self.subst.insert(mvar, rhs);
        Ok(())
    }
    /// Increment the step counter, returning `Err` if the limit is exceeded.
    pub fn step(&mut self) -> Result<(), UnifyError> {
        self.steps += 1;
        if self.max_steps > 0 && self.steps > self.max_steps {
            Err(UnifyError::Other(format!(
                "unification exceeded {} steps",
                self.max_steps
            )))
        } else {
            Ok(())
        }
    }
}
/// A single unification constraint: `lhs ≡ rhs`.
#[derive(Clone, Debug, PartialEq)]
pub struct UnifyConstraint {
    /// Left-hand side.
    pub lhs: Expr,
    /// Right-hand side.
    pub rhs: Expr,
    /// Source location for error reporting.
    pub source: Option<String>,
}
impl UnifyConstraint {
    /// Create a constraint without a source location.
    pub fn new(lhs: Expr, rhs: Expr) -> Self {
        Self {
            lhs,
            rhs,
            source: None,
        }
    }
    /// Create a constraint with a source location.
    pub fn with_source(lhs: Expr, rhs: Expr, src: impl Into<String>) -> Self {
        Self {
            lhs,
            rhs,
            source: Some(src.into()),
        }
    }
    /// Check if the constraint is trivially satisfied (lhs == rhs).
    pub fn is_trivial(&self) -> bool {
        self.lhs == self.rhs
    }
}
