//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::infer::{Constraint, MetaVarId};
use oxilean_kernel::{Expr, Level, Literal, Name};
use std::collections::HashMap;

use super::functions::*;

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SolverDiagLevel {
    Info,
    Warning,
    Error,
}
/// Statistics collected by a solver run.
#[derive(Debug, Clone, Default)]
pub struct SolverStats {
    /// Total constraints processed.
    pub total_processed: usize,
    /// Constraints solved on first attempt.
    pub solved_immediate: usize,
    /// Constraints that required retries.
    pub solved_with_retry: usize,
    /// Constraints that were deferred.
    pub deferred_total: usize,
    /// Constraints that ultimately failed.
    pub failed_total: usize,
    /// Number of metavariables assigned.
    pub mvars_assigned: usize,
    /// Number of solver passes.
    pub passes: usize,
}
impl SolverStats {
    /// Create empty stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Success rate (0.0 to 1.0).
    pub fn success_rate(&self) -> f64 {
        if self.total_processed == 0 {
            1.0
        } else {
            (self.solved_immediate + self.solved_with_retry) as f64 / self.total_processed as f64
        }
    }
    /// Check if all constraints were solved.
    pub fn all_solved(&self) -> bool {
        self.failed_total == 0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConstraintGraphNode {
    pub constraint_id: u64,
    pub depends_on: Vec<MetaVarId>,
    pub blocked_by: Vec<MetaVarId>,
}
/// A queue of postponed unification constraints.
///
/// When unification of two expressions cannot be resolved immediately (e.g.,
/// both sides involve unsolved metavariables), the constraint is *postponed*
/// rather than immediately failing.  Once new metavariable assignments become
/// available, `retry_postponed` is called to attempt those constraints again.
#[derive(Debug, Clone, Default)]
pub struct ConstraintQueue {
    /// Pending postponed constraints: `(lhs, rhs)` pairs.
    postponed: Vec<(Expr, Expr)>,
    /// Total number of constraints ever postponed.
    pub total_postponed: usize,
    /// Total number of constraints successfully retried.
    pub total_retried: usize,
}
impl ConstraintQueue {
    /// Create an empty constraint queue.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a constraint `lhs =?= rhs` to the postponed queue.
    ///
    /// This should be called when unification cannot make progress because
    /// one or both sides contain unsolved metavariables.
    pub fn postpone_constraint(&mut self, lhs: Expr, rhs: Expr) {
        self.postponed.push((lhs, rhs));
        self.total_postponed += 1;
    }
    /// Try to solve all postponed constraints using the current `assignments`.
    ///
    /// Constraints that can now be solved (because the required metavariables
    /// have been assigned) are removed from the queue; unsolvable constraints
    /// remain.  Returns the number of constraints successfully retired.
    ///
    /// A constraint is considered solved if `unify_basic` returns
    /// `UnifyResult::Solved`.  Newly-produced assignments are added to the
    /// `assignments` map so that subsequent retries can benefit from them.
    pub fn retry_postponed(&mut self, assignments: &mut HashMap<MetaVarId, Expr>) -> usize {
        let mut solved = 0;
        let old_postponed = std::mem::take(&mut self.postponed);
        for (lhs, rhs) in old_postponed {
            let lhs_applied = apply_assignments_impl(&lhs, assignments);
            let rhs_applied = apply_assignments_impl(&rhs, assignments);
            match unify_basic(&lhs_applied, &rhs_applied, assignments) {
                UnifyResult::Solved(new_assigns) => {
                    assignments.extend(new_assigns);
                    solved += 1;
                    self.total_retried += 1;
                }
                UnifyResult::Defer | UnifyResult::Fail(_) => {
                    self.postponed.push((lhs, rhs));
                }
            }
        }
        solved
    }
    /// Number of currently postponed constraints.
    pub fn len(&self) -> usize {
        self.postponed.len()
    }
    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.postponed.is_empty()
    }
    /// Clear all postponed constraints.
    pub fn clear(&mut self) {
        self.postponed.clear();
    }
    /// Iterate over postponed constraints (for inspection / debugging).
    pub fn iter(&self) -> impl Iterator<Item = &(Expr, Expr)> {
        self.postponed.iter()
    }
    /// Drain all remaining postponed constraints (e.g., to report errors).
    pub fn drain(&mut self) -> Vec<(Expr, Expr)> {
        std::mem::take(&mut self.postponed)
    }
    /// Repeatedly call `retry_postponed` until no more progress is made.
    ///
    /// Returns the total number of constraints solved across all passes.
    pub fn retry_until_stable(&mut self, assignments: &mut HashMap<MetaVarId, Expr>) -> usize {
        let mut total = 0;
        loop {
            let solved = self.retry_postponed(assignments);
            if solved == 0 {
                break;
            }
            total += solved;
        }
        total
    }
}
/// Priority of a constraint for scheduling.
///
/// Higher-priority constraints are solved first because they are more
/// likely to produce assignments that unblock lower-priority constraints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstraintPriority {
    /// Must be solved immediately (e.g., forced assignments).
    Urgent = 3,
    /// Normal elaboration constraint.
    Normal = 2,
    /// Deferred constraint; attempted last.
    Deferred = 1,
    /// Postponed until other constraints provide more information.
    Postponed = 0,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetaVarStatus {
    Unassigned,
    Assigned,
    Generalized,
    Failed,
}
/// A constraint annotated with scheduling metadata.
#[derive(Debug, Clone)]
pub struct ScheduledConstraint {
    /// The underlying constraint.
    pub constraint: Constraint,
    /// Priority for scheduling.
    pub priority: ConstraintPriority,
    /// Number of times this constraint has been retried.
    pub retry_count: u32,
    /// Maximum allowed retries before giving up.
    pub max_retries: u32,
}
impl ScheduledConstraint {
    /// Create a new scheduled constraint with normal priority.
    pub fn new(constraint: Constraint) -> Self {
        Self {
            constraint,
            priority: ConstraintPriority::Normal,
            retry_count: 0,
            max_retries: 10,
        }
    }
    /// Create with a specific priority.
    pub fn with_priority(constraint: Constraint, priority: ConstraintPriority) -> Self {
        Self {
            constraint,
            priority,
            retry_count: 0,
            max_retries: 10,
        }
    }
    /// Check if this constraint has exceeded its retry limit.
    pub fn is_exhausted(&self) -> bool {
        self.retry_count >= self.max_retries
    }
    /// Increment the retry counter.
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }
}
/// Constraint solver for type inference.
pub struct ConstraintSolver {
    /// Metavariable assignments
    assignments: HashMap<MetaVarId, Expr>,
    /// Pending constraints
    pending: Vec<Constraint>,
}
impl ConstraintSolver {
    /// Create a new constraint solver.
    pub fn new() -> Self {
        Self {
            assignments: HashMap::new(),
            pending: Vec::new(),
        }
    }
    /// Add a constraint to solve.
    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.pending.push(constraint);
    }
    /// Add multiple constraints.
    pub fn add_constraints(&mut self, constraints: Vec<Constraint>) {
        self.pending.extend(constraints);
    }
    /// Solve all pending constraints.
    pub fn solve(&mut self) -> Result<(), String> {
        while let Some(constraint) = self.pending.pop() {
            self.solve_constraint(constraint)?;
        }
        Ok(())
    }
    /// Solve a single constraint.
    fn solve_constraint(&mut self, constraint: Constraint) -> Result<(), String> {
        match constraint {
            Constraint::Equal(e1, e2) => self.unify(&e1, &e2),
            Constraint::HasType(_expr, _ty) => Ok(()),
            Constraint::Assign(meta, expr) => {
                self.assignments.insert(meta, expr);
                Ok(())
            }
        }
    }
    /// Unify two expressions.
    pub(super) fn unify(&mut self, e1: &Expr, e2: &Expr) -> Result<(), String> {
        if e1 == e2 {
            return Ok(());
        }
        match (e1, e2) {
            (Expr::BVar(i), Expr::BVar(j)) if i == j => Ok(()),
            (Expr::FVar(f1), Expr::FVar(f2)) if f1 == f2 => Ok(()),
            (Expr::Const(n1, _), Expr::Const(n2, _)) if n1 == n2 => Ok(()),
            (Expr::App(f1, a1), Expr::App(f2, a2)) => {
                self.unify(f1, f2)?;
                self.unify(a1, a2)
            }
            (Expr::Lam(_, _, ty1, body1), Expr::Lam(_, _, ty2, body2)) => {
                self.unify(ty1, ty2)?;
                self.unify(body1, body2)
            }
            (Expr::Pi(_, _, ty1, body1), Expr::Pi(_, _, ty2, body2)) => {
                self.unify(ty1, ty2)?;
                self.unify(body1, body2)
            }
            _ => Err(format!("Cannot unify {:?} with {:?}", e1, e2)),
        }
    }
    /// Get the assignment for a metavariable.
    pub fn get_assignment(&self, meta: MetaVarId) -> Option<&Expr> {
        self.assignments.get(&meta)
    }
    /// Get all assignments.
    pub fn assignments(&self) -> &HashMap<MetaVarId, Expr> {
        &self.assignments
    }
    /// Get pending constraints count.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    /// Clear all constraints and assignments.
    pub fn clear(&mut self) {
        self.assignments.clear();
        self.pending.clear();
    }
}
/// Describes a conflict detected during constraint solving.
#[derive(Debug, Clone)]
pub struct ConflictInfo {
    /// First expression in the conflict.
    pub lhs: Expr,
    /// Second expression in the conflict.
    pub rhs: Expr,
    /// Human-readable explanation.
    pub message: String,
}
impl ConflictInfo {
    /// Create a new conflict info.
    pub fn new(lhs: Expr, rhs: Expr, message: impl Into<String>) -> Self {
        Self {
            lhs,
            rhs,
            message: message.into(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetaVarInfo {
    pub id: MetaVarId,
    pub status: MetaVarStatus,
    pub value: Option<Expr>,
    pub depth: usize,
}
#[allow(dead_code)]
#[derive(Default)]
pub struct MetaVarContext {
    vars: HashMap<MetaVarId, MetaVarInfo>,
    next_id: MetaVarId,
}
#[allow(dead_code)]
impl MetaVarContext {
    pub fn new() -> Self {
        MetaVarContext {
            vars: HashMap::new(),
            next_id: 0,
        }
    }
    pub fn fresh(&mut self, depth: usize) -> MetaVarId {
        let id = self.next_id;
        self.next_id += 1;
        self.vars.insert(
            id,
            MetaVarInfo {
                id,
                status: MetaVarStatus::Unassigned,
                value: None,
                depth,
            },
        );
        id
    }
    pub fn assign(&mut self, id: MetaVarId, value: Expr) -> bool {
        if let Some(info) = self.vars.get_mut(&id) {
            if info.status == MetaVarStatus::Unassigned {
                info.status = MetaVarStatus::Assigned;
                info.value = Some(value);
                return true;
            }
        }
        false
    }
    pub fn get(&self, id: MetaVarId) -> Option<&MetaVarInfo> {
        self.vars.get(&id)
    }
    pub fn is_assigned(&self, id: MetaVarId) -> bool {
        self.vars
            .get(&id)
            .map(|v| v.status == MetaVarStatus::Assigned)
            .unwrap_or(false)
    }
    pub fn value_of(&self, id: MetaVarId) -> Option<&Expr> {
        self.vars.get(&id).and_then(|v| v.value.as_ref())
    }
    pub fn unassigned_count(&self) -> usize {
        self.vars
            .values()
            .filter(|v| v.status == MetaVarStatus::Unassigned)
            .count()
    }
    pub fn all_assigned(&self) -> bool {
        self.vars
            .values()
            .all(|v| v.status == MetaVarStatus::Assigned)
    }
    pub fn ids(&self) -> Vec<MetaVarId> {
        self.vars.keys().copied().collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SolverEventKind {
    ConstraintAdded,
    ConstraintSolved,
    ConstraintPostponed,
    ConstraintFailed,
    MetaAssigned { meta: MetaVarId },
    CheckpointSaved { id: u64 },
    CheckpointRestored { id: u64 },
    SolveStarted,
    SolveFinished { success: bool },
}
#[allow(dead_code)]
pub struct SimplificationPhase;
#[allow(dead_code)]
pub struct SolverEventLog {
    events: Vec<SolverEvent>,
    step: u64,
    enabled: bool,
}
#[allow(dead_code)]
impl SolverEventLog {
    pub fn new(enabled: bool) -> Self {
        SolverEventLog {
            events: Vec::new(),
            step: 0,
            enabled,
        }
    }
    pub fn emit(&mut self, kind: SolverEventKind) {
        if self.enabled {
            let step = self.step;
            self.step += 1;
            self.events.push(SolverEvent { kind, step });
        }
    }
    pub fn count(&self) -> usize {
        self.events.len()
    }
    pub fn all(&self) -> &[SolverEvent] {
        &self.events
    }
    pub fn clear(&mut self) {
        self.events.clear();
        self.step = 0;
    }
    pub fn assignments_emitted(&self) -> usize {
        self.events
            .iter()
            .filter(|e| matches!(e.kind, SolverEventKind::MetaAssigned { .. }))
            .count()
    }
}
#[allow(dead_code)]
#[derive(Default)]
pub struct ConstraintGraph {
    nodes: Vec<ConstraintGraphNode>,
    next_id: u64,
    meta_to_nodes: HashMap<MetaVarId, Vec<u64>>,
}
#[allow(dead_code)]
impl ConstraintGraph {
    pub fn new() -> Self {
        ConstraintGraph {
            nodes: Vec::new(),
            next_id: 0,
            meta_to_nodes: HashMap::new(),
        }
    }
    pub fn add_node(&mut self, depends_on: Vec<MetaVarId>, blocked_by: Vec<MetaVarId>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        for &m in &blocked_by {
            self.meta_to_nodes.entry(m).or_default().push(id);
        }
        self.nodes.push(ConstraintGraphNode {
            constraint_id: id,
            depends_on,
            blocked_by,
        });
        id
    }
    /// Return all constraint IDs that were blocked on `meta` (now unblocked)
    pub fn unblock(&mut self, meta: MetaVarId) -> Vec<u64> {
        let unblocked = self.meta_to_nodes.remove(&meta).unwrap_or_default();
        for node in &mut self.nodes {
            node.blocked_by.retain(|&m| m != meta);
        }
        unblocked
    }
    pub fn ready_nodes(&self) -> Vec<u64> {
        self.nodes
            .iter()
            .filter(|n| n.blocked_by.is_empty())
            .map(|n| n.constraint_id)
            .collect()
    }
    pub fn num_nodes(&self) -> usize {
        self.nodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Default)]
pub struct CheckpointManager {
    checkpoints: Vec<SolverCheckpoint>,
    next_id: u64,
}
#[allow(dead_code)]
impl CheckpointManager {
    pub fn new() -> Self {
        CheckpointManager {
            checkpoints: Vec::new(),
            next_id: 0,
        }
    }
    pub fn save(&mut self, assignments: &HashMap<MetaVarId, Expr>, pending_count: usize) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.checkpoints.push(SolverCheckpoint {
            assignments_snapshot: assignments.clone(),
            pending_count,
            checkpoint_id: id,
        });
        id
    }
    pub fn restore(&mut self, id: u64) -> Option<SolverCheckpoint> {
        let pos = self
            .checkpoints
            .iter()
            .position(|c| c.checkpoint_id == id)?;
        let checkpoint = self.checkpoints[pos].clone();
        self.checkpoints.truncate(pos);
        Some(checkpoint)
    }
    pub fn latest(&self) -> Option<&SolverCheckpoint> {
        self.checkpoints.last()
    }
    pub fn depth(&self) -> usize {
        self.checkpoints.len()
    }
    pub fn clear(&mut self) {
        self.checkpoints.clear();
    }
}
#[allow(dead_code)]
pub struct SolverExtensionMarker;
#[allow(dead_code)]
#[derive(Default)]
pub struct SolverPipeline {
    phases: Vec<Box<dyn SolverPhase>>,
}
#[allow(dead_code)]
impl SolverPipeline {
    pub fn new() -> Self {
        SolverPipeline { phases: Vec::new() }
    }
    pub fn add_phase(&mut self, phase: Box<dyn SolverPhase>) {
        self.phases.push(phase);
    }
    pub fn run(
        &self,
        mut constraints: Vec<Constraint>,
        assignments: &mut HashMap<MetaVarId, Expr>,
    ) -> (Vec<Constraint>, Vec<String>) {
        let mut all_errors = Vec::new();
        for phase in &self.phases {
            let (new_constraints, errors) = phase.run(constraints, assignments);
            constraints = new_constraints;
            all_errors.extend(errors);
        }
        (constraints, all_errors)
    }
    pub fn num_phases(&self) -> usize {
        self.phases.len()
    }
}
#[allow(dead_code)]
#[derive(Default)]
pub struct SolverDiagCollector {
    diagnostics: Vec<SolverDiagnostic>,
}
#[allow(dead_code)]
impl SolverDiagCollector {
    pub fn new() -> Self {
        SolverDiagCollector {
            diagnostics: Vec::new(),
        }
    }
    pub fn add(&mut self, d: SolverDiagnostic) {
        self.diagnostics.push(d);
    }
    pub fn add_info(&mut self, msg: impl Into<String>) {
        self.add(SolverDiagnostic::info(msg));
    }
    pub fn add_warning(&mut self, msg: impl Into<String>) {
        self.add(SolverDiagnostic::warning(msg));
    }
    pub fn add_error(&mut self, msg: impl Into<String>) {
        self.add(SolverDiagnostic::error(msg));
    }
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.is_error())
    }
    pub fn error_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.is_error()).count()
    }
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.level == SolverDiagLevel::Warning)
            .count()
    }
    pub fn all(&self) -> &[SolverDiagnostic] {
        &self.diagnostics
    }
    pub fn drain_errors(&mut self) -> Vec<SolverDiagnostic> {
        let errors: Vec<_> = self
            .diagnostics
            .iter()
            .filter(|d| d.is_error())
            .cloned()
            .collect();
        self.diagnostics.retain(|d| !d.is_error());
        errors
    }
    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }
}
#[allow(dead_code)]
pub struct OccursCheck;
#[allow(dead_code)]
impl OccursCheck {
    /// Returns true if `meta` appears free in `expr`.
    pub fn occurs(meta: MetaVarId, expr: &Expr, assignments: &HashMap<MetaVarId, Expr>) -> bool {
        Self::occurs_in(meta, expr, assignments, 0)
    }
    fn occurs_in(
        meta: MetaVarId,
        expr: &Expr,
        assignments: &HashMap<MetaVarId, Expr>,
        _depth: usize,
    ) -> bool {
        match expr {
            Expr::FVar(fv) if fv.0 >= MVAR_OFFSET => {
                let m = fv.0 - MVAR_OFFSET;
                if m == meta {
                    return true;
                }
                if let Some(val) = assignments.get(&m) {
                    return Self::occurs_in(meta, val, assignments, _depth);
                }
                false
            }
            Expr::App(f, a) => {
                Self::occurs_in(meta, f, assignments, _depth)
                    || Self::occurs_in(meta, a, assignments, _depth)
            }
            Expr::Pi(_n, _bi, dom, cod) => {
                Self::occurs_in(meta, dom, assignments, _depth)
                    || Self::occurs_in(meta, cod, assignments, _depth + 1)
            }
            Expr::Lam(_n, _bi, ty, body) => {
                Self::occurs_in(meta, ty, assignments, _depth)
                    || Self::occurs_in(meta, body, assignments, _depth + 1)
            }
            _ => false,
        }
    }
}
/// A priority-based constraint solver.
///
/// Processes urgent constraints before normal ones, and defers
/// constraints that cannot yet be solved.
pub struct PrioritySolver {
    /// Assignments made so far.
    assignments: HashMap<MetaVarId, Expr>,
    /// Urgent constraints (solved first).
    urgent: Vec<ScheduledConstraint>,
    /// Normal constraints.
    normal: Vec<ScheduledConstraint>,
    /// Deferred constraints (solved last).
    deferred: Vec<ScheduledConstraint>,
    /// Postponed constraints (stuck; may be retried later).
    postponed: Vec<ScheduledConstraint>,
    /// Statistics: number of constraints solved.
    pub num_solved: usize,
    /// Statistics: number of constraints deferred.
    pub num_deferred: usize,
    /// Statistics: number of constraints postponed.
    pub num_postponed: usize,
}
impl PrioritySolver {
    /// Create a new priority solver.
    pub fn new() -> Self {
        Self {
            assignments: HashMap::new(),
            urgent: Vec::new(),
            normal: Vec::new(),
            deferred: Vec::new(),
            postponed: Vec::new(),
            num_solved: 0,
            num_deferred: 0,
            num_postponed: 0,
        }
    }
    /// Add a constraint with its priority.
    pub fn add(&mut self, sc: ScheduledConstraint) {
        match sc.priority {
            ConstraintPriority::Urgent => self.urgent.push(sc),
            ConstraintPriority::Normal => self.normal.push(sc),
            ConstraintPriority::Deferred => self.deferred.push(sc),
            ConstraintPriority::Postponed => self.postponed.push(sc),
        }
    }
    /// Add a plain constraint at normal priority.
    pub fn add_normal(&mut self, c: Constraint) {
        self.add(ScheduledConstraint::new(c));
    }
    /// Add an urgent assignment.
    pub fn add_urgent_assign(&mut self, meta: MetaVarId, expr: Expr) {
        self.add(ScheduledConstraint::with_priority(
            Constraint::Assign(meta, expr),
            ConstraintPriority::Urgent,
        ));
    }
    /// Total number of pending constraints (across all queues).
    pub fn pending_count(&self) -> usize {
        self.urgent.len() + self.normal.len() + self.deferred.len() + self.postponed.len()
    }
    /// Check if any constraints remain.
    pub fn is_empty(&self) -> bool {
        self.pending_count() == 0
    }
    /// Pop the next constraint to process (highest priority first).
    fn pop_next(&mut self) -> Option<ScheduledConstraint> {
        if let Some(sc) = self.urgent.pop() {
            return Some(sc);
        }
        if let Some(sc) = self.normal.pop() {
            return Some(sc);
        }
        if let Some(sc) = self.deferred.pop() {
            return Some(sc);
        }
        if let Some(sc) = self.postponed.pop() {
            return Some(sc);
        }
        None
    }
    /// Apply metavariable assignments to an expression.
    fn apply_assignments(&self, expr: &Expr) -> Expr {
        apply_assignments_impl(expr, &self.assignments)
    }
    /// Solve all pending constraints, returning an error if any cannot be solved.
    pub fn solve(&mut self) -> Result<(), String> {
        let mut passes = 0;
        let max_passes = 100;
        while !self.is_empty() && passes < max_passes {
            passes += 1;
            let mut made_progress = false;
            let mut remaining = Vec::new();
            while let Some(mut sc) = self.pop_next() {
                let c = match &sc.constraint {
                    Constraint::Equal(e1, e2) => {
                        Constraint::Equal(self.apply_assignments(e1), self.apply_assignments(e2))
                    }
                    Constraint::HasType(e, ty) => {
                        Constraint::HasType(self.apply_assignments(e), self.apply_assignments(ty))
                    }
                    Constraint::Assign(m, e) => Constraint::Assign(*m, self.apply_assignments(e)),
                };
                match self.solve_one(c) {
                    Ok(true) => {
                        self.num_solved += 1;
                        made_progress = true;
                    }
                    Ok(false) => {
                        if sc.is_exhausted() {
                            return Err(format!(
                                "Constraint exhausted after {} retries: {:?}",
                                sc.retry_count, sc.constraint
                            ));
                        }
                        sc.increment_retry();
                        self.num_deferred += 1;
                        remaining.push(sc);
                    }
                    Err(e) => return Err(e),
                }
            }
            for sc in remaining {
                self.deferred.push(sc);
            }
            if !made_progress && !self.is_empty() {
                return Err(format!(
                    "Solver stuck: {} unsolved constraints remain",
                    self.pending_count()
                ));
            }
        }
        if !self.is_empty() {
            return Err(format!(
                "Solver did not converge after {} passes",
                max_passes
            ));
        }
        Ok(())
    }
    /// Attempt to solve a single constraint.
    ///
    /// Returns `Ok(true)` if solved, `Ok(false)` if deferred, `Err` on failure.
    fn solve_one(&mut self, constraint: Constraint) -> Result<bool, String> {
        match constraint {
            Constraint::Equal(e1, e2) => {
                if e1 == e2 {
                    return Ok(true);
                }
                match unify_basic(&e1, &e2, &self.assignments) {
                    UnifyResult::Solved(new_assigns) => {
                        self.assignments.extend(new_assigns);
                        Ok(true)
                    }
                    UnifyResult::Defer => Ok(false),
                    UnifyResult::Fail(msg) => Err(msg),
                }
            }
            Constraint::HasType(expr, ty) => Ok(check_has_type_basic(&expr, &ty).unwrap_or(true)),
            Constraint::Assign(meta, expr) => {
                if let Some(existing) = self.assignments.get(&meta) {
                    if existing != &expr {
                        return Err(format!(
                            "Conflicting assignments for metavar {}: {:?} vs {:?}",
                            meta, existing, expr
                        ));
                    }
                } else {
                    self.assignments.insert(meta, expr);
                }
                Ok(true)
            }
        }
    }
    /// Get an assignment for a metavariable.
    pub fn get_assignment(&self, meta: MetaVarId) -> Option<&Expr> {
        self.assignments.get(&meta)
    }
    /// Get all assignments.
    pub fn assignments(&self) -> &HashMap<MetaVarId, Expr> {
        &self.assignments
    }
    /// Clear all state.
    pub fn clear(&mut self) {
        self.assignments.clear();
        self.urgent.clear();
        self.normal.clear();
        self.deferred.clear();
        self.postponed.clear();
        self.num_solved = 0;
        self.num_deferred = 0;
        self.num_postponed = 0;
    }
}
/// Result of a basic unification attempt.
#[derive(Debug)]
pub enum UnifyResult {
    /// Successfully unified; the enclosed map contains new assignments.
    Solved(HashMap<MetaVarId, Expr>),
    /// Cannot determine outcome yet; defer the constraint.
    Defer,
    /// Unification failed with a message.
    Fail(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolverReport {
    pub success: bool,
    pub assignments_made: usize,
    pub constraints_solved: usize,
    pub constraints_postponed: usize,
    pub iterations: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
#[allow(dead_code)]
impl SolverReport {
    pub fn success(assignments: usize, solved: usize, iterations: usize) -> Self {
        SolverReport {
            success: true,
            assignments_made: assignments,
            constraints_solved: solved,
            constraints_postponed: 0,
            iterations,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    pub fn failure(errors: Vec<String>) -> Self {
        SolverReport {
            success: false,
            assignments_made: 0,
            constraints_solved: 0,
            constraints_postponed: 0,
            iterations: 0,
            errors,
            warnings: Vec::new(),
        }
    }
    pub fn add_warning(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolverDiagnostic {
    pub level: SolverDiagLevel,
    pub message: String,
    pub constraint_id: Option<u64>,
}
#[allow(dead_code)]
impl SolverDiagnostic {
    pub fn info(msg: impl Into<String>) -> Self {
        SolverDiagnostic {
            level: SolverDiagLevel::Info,
            message: msg.into(),
            constraint_id: None,
        }
    }
    pub fn warning(msg: impl Into<String>) -> Self {
        SolverDiagnostic {
            level: SolverDiagLevel::Warning,
            message: msg.into(),
            constraint_id: None,
        }
    }
    pub fn error(msg: impl Into<String>) -> Self {
        SolverDiagnostic {
            level: SolverDiagLevel::Error,
            message: msg.into(),
            constraint_id: None,
        }
    }
    pub fn with_constraint_id(mut self, id: u64) -> Self {
        self.constraint_id = Some(id);
        self
    }
    pub fn is_error(&self) -> bool {
        self.level == SolverDiagLevel::Error
    }
}
/// A snapshot of the solver state (assignments + pending constraints).
#[derive(Debug, Clone, Default)]
pub struct SolverState {
    /// Metavariable assignments.
    pub assignments: HashMap<MetaVarId, Expr>,
    /// Pending constraints.
    pub constraints: Vec<Constraint>,
}
impl SolverState {
    /// Create an empty state.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an assignment.
    pub fn assign(&mut self, meta: MetaVarId, expr: Expr) {
        self.assignments.insert(meta, expr);
    }
    /// Add a constraint.
    pub fn add_constraint(&mut self, c: Constraint) {
        self.constraints.push(c);
    }
    /// Check if all constraints are solved (no pending).
    pub fn is_solved(&self) -> bool {
        self.constraints.is_empty()
    }
    /// Number of pending constraints.
    pub fn pending(&self) -> usize {
        self.constraints.len()
    }
    /// Number of assignments.
    pub fn num_assignments(&self) -> usize {
        self.assignments.len()
    }
    /// Apply all assignments to all constraints (normalize).
    pub fn normalize(&mut self) {
        self.constraints = normalize_constraints(&self.constraints, &self.assignments);
        self.constraints = simplify_constraints(std::mem::take(&mut self.constraints));
    }
}
/// An incremental constraint solver that processes constraints one at a time.
///
/// Unlike `PrioritySolver`, this solver is designed to be called repeatedly
/// as new constraints become available during elaboration.
#[derive(Debug, Clone, Default)]
pub struct IncrementalSolver {
    /// Known assignments.
    assignments: HashMap<MetaVarId, Expr>,
    /// Unsolved constraints (may be retried as more info comes in).
    pending: Vec<Constraint>,
    /// Number of successful solves.
    pub num_solved: usize,
    /// Number of remaining unsolved constraints.
    pub num_pending: usize,
}
impl IncrementalSolver {
    /// Create a new incremental solver.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a new constraint (to be solved on the next `step` call).
    pub fn add(&mut self, c: Constraint) {
        self.pending.push(c);
        self.num_pending += 1;
    }
    /// Attempt to make progress by solving as many pending constraints as possible.
    ///
    /// Returns the number of constraints solved in this step.
    pub fn step(&mut self) -> usize {
        let mut solved = 0;
        let mut remaining = Vec::new();
        for c in std::mem::take(&mut self.pending) {
            let c_applied = apply_constraint_assignments(&c, &self.assignments);
            match try_solve_constraint(&c_applied, &mut self.assignments) {
                true => {
                    solved += 1;
                    self.num_solved += 1;
                    self.num_pending = self.num_pending.saturating_sub(1);
                }
                false => remaining.push(c),
            }
        }
        self.pending = remaining;
        solved
    }
    /// Run until no progress is made.
    pub fn solve_all(&mut self) {
        loop {
            let progress = self.step();
            if progress == 0 {
                break;
            }
        }
    }
    /// Get the assignment for a metavariable.
    pub fn get_assignment(&self, meta: MetaVarId) -> Option<&Expr> {
        self.assignments.get(&meta)
    }
    /// Check if all constraints are solved.
    pub fn is_complete(&self) -> bool {
        self.pending.is_empty()
    }
    /// Remaining unsolved constraint count.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    /// Clear all state.
    pub fn clear(&mut self) {
        self.assignments.clear();
        self.pending.clear();
        self.num_solved = 0;
        self.num_pending = 0;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolverConfig {
    pub max_iterations: usize,
    pub occurs_check: bool,
    pub enable_postponing: bool,
    pub max_postponed: usize,
    pub log_events: bool,
    pub strict_mode: bool,
}
#[allow(dead_code)]
impl SolverConfig {
    pub fn strict() -> Self {
        SolverConfig {
            strict_mode: true,
            occurs_check: true,
            ..Self::default()
        }
    }
    pub fn lenient() -> Self {
        SolverConfig {
            occurs_check: false,
            strict_mode: false,
            ..Self::default()
        }
    }
    pub fn with_max_iterations(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    pub fn with_logging(mut self) -> Self {
        self.log_events = true;
        self
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolverCheckpoint {
    pub assignments_snapshot: HashMap<MetaVarId, Expr>,
    pub pending_count: usize,
    pub checkpoint_id: u64,
}
#[allow(dead_code)]
pub struct ConstraintSimplifier;
#[allow(dead_code)]
impl ConstraintSimplifier {
    /// Simplify a list of constraints: remove trivially-true ones (Eq x x)
    pub fn simplify(constraints: Vec<Constraint>) -> Vec<Constraint> {
        constraints
            .into_iter()
            .filter(|c| !Self::is_trivial(c))
            .collect()
    }
    pub fn is_trivial(c: &Constraint) -> bool {
        match c {
            Constraint::Equal(a, b) => Self::syntactically_equal(a, b),
            _ => false,
        }
    }
    fn syntactically_equal(a: &Expr, b: &Expr) -> bool {
        match (a, b) {
            (Expr::BVar(i), Expr::BVar(j)) => i == j,
            (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
            (Expr::Const(n1, lvls1), Expr::Const(n2, lvls2)) => n1 == n2 && lvls1 == lvls2,
            (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
            (Expr::FVar(f1), Expr::FVar(f2)) => f1 == f2,
            (Expr::App(f1, a1), Expr::App(f2, a2)) => {
                Self::syntactically_equal(f1, f2) && Self::syntactically_equal(a1, a2)
            }
            _ => false,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SolverEvent {
    pub kind: SolverEventKind,
    pub step: u64,
}
