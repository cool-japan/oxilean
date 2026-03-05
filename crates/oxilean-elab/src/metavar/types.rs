//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::Expr;
use std::collections::{HashMap, HashSet};

use super::metavarcontext_type::MetaVarContext;

/// A group of metavariables that are related (e.g., from the same `have` block).
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct MetaVarGroup {
    /// Group label.
    pub label: String,
    /// IDs of metavariables in this group.
    pub members: Vec<u64>,
    /// Whether this group is "closed" (no new members can be added).
    pub closed: bool,
}
#[allow(dead_code)]
impl MetaVarGroup {
    /// Create a new open group.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            members: Vec::new(),
            closed: false,
        }
    }
    /// Add a member.
    pub fn add(&mut self, id: u64) {
        if !self.closed {
            self.members.push(id);
        }
    }
    /// Close the group.
    pub fn close(&mut self) {
        self.closed = true;
    }
    /// Number of members.
    pub fn len(&self) -> usize {
        self.members.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
    /// Check if a given ID is in this group.
    pub fn contains(&self, id: u64) -> bool {
        self.members.contains(&id)
    }
    /// Check how many members are solved.
    pub fn solved_count(&self, ctx: &MetaVarContext) -> usize {
        self.members.iter().filter(|&&id| ctx.is_solved(id)).count()
    }
    /// Check if all members are solved.
    pub fn all_solved(&self, ctx: &MetaVarContext) -> bool {
        self.members.iter().all(|&id| ctx.is_solved(id))
    }
}
/// A queue of delayed assignments.
#[derive(Clone, Debug, Default)]
pub struct DelayedAssignmentQueue {
    queue: Vec<DelayedAssignment>,
}
impl DelayedAssignmentQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self::default()
    }
    /// Enqueue a delayed assignment.
    pub fn enqueue(&mut self, da: DelayedAssignment) {
        self.queue.push(da);
    }
    /// Drain all ready assignments, returning them.
    pub fn drain_ready(&mut self, ctx: &MetaVarContext) -> Vec<DelayedAssignment> {
        let mut ready = Vec::new();
        let mut remaining = Vec::new();
        for da in self.queue.drain(..) {
            if da.is_ready(ctx) {
                ready.push(da);
            } else {
                remaining.push(da);
            }
        }
        self.queue = remaining;
        ready
    }
    /// Number of pending assignments.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
/// Aggregated statistics about a `MetaVarContext`.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct MetaVarStats {
    /// Total metavariables created.
    pub total: usize,
    /// Solved metavariables.
    pub solved: usize,
    /// Unsolved metavariables.
    pub unsolved: usize,
    /// Frozen metavariables.
    pub frozen: usize,
    /// Natural metavariables.
    pub natural: usize,
    /// Synthetic metavariables.
    pub synthetic: usize,
    /// Synthetic-opaque metavariables.
    pub synthetic_opaque: usize,
    /// Pending constraints.
    pub constraints: usize,
}
#[allow(dead_code)]
impl MetaVarStats {
    /// Collect stats from a `MetaVarContext`.
    pub fn from_ctx(ctx: &MetaVarContext) -> Self {
        let total = ctx.count();
        let solved = ctx.solved_count();
        let unsolved = ctx.unsolved_count();
        let frozen = ctx.frozen.len();
        let mut natural = 0;
        let mut synthetic = 0;
        let mut synthetic_opaque = 0;
        for m in ctx.metas.values() {
            match m.kind {
                MetaVarKind::Natural => natural += 1,
                MetaVarKind::Synthetic => synthetic += 1,
                MetaVarKind::SyntheticOpaque => synthetic_opaque += 1,
            }
        }
        Self {
            total,
            solved,
            unsolved,
            frozen,
            natural,
            synthetic,
            synthetic_opaque,
            constraints: ctx.constraint_count(),
        }
    }
    /// Solve ratio (0.0 to 1.0).
    pub fn solve_ratio(&self) -> f64 {
        if self.total == 0 {
            1.0
        } else {
            self.solved as f64 / self.total as f64
        }
    }
    /// Format a human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "total: {}, solved: {}, unsolved: {}, frozen: {}, constraints: {} (ratio: {:.1}%)",
            self.total,
            self.solved,
            self.unsolved,
            self.frozen,
            self.constraints,
            self.solve_ratio() * 100.0,
        )
    }
    /// Check if all metavariables are solved.
    pub fn is_fully_solved(&self) -> bool {
        self.unsolved == 0
    }
}
/// An event in the metavariable log.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum MetaVarEvent {
    /// A new metavariable was created.
    Created {
        id: u64,
        kind: MetaVarKind,
        depth: u32,
    },
    /// A metavariable was assigned.
    Assigned { id: u64 },
    /// A metavariable was frozen.
    Frozen { id: u64 },
    /// A metavariable was unfrozen.
    Unfrozen { id: u64 },
    /// A constraint was added.
    ConstraintAdded { origin: String },
    /// A snapshot was taken.
    SnapshotTaken { next_id: u64 },
    /// A restore was performed.
    Restored { next_id: u64 },
}
/// A log of metavariable operations for debugging.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct MetaVarLog {
    events: Vec<MetaVarEvent>,
    enabled: bool,
}
#[allow(dead_code)]
impl MetaVarLog {
    /// Create a disabled log (no-op).
    pub fn new() -> Self {
        Self::default()
    }
    /// Create an enabled log.
    pub fn enabled() -> Self {
        Self {
            events: Vec::new(),
            enabled: true,
        }
    }
    /// Enable logging.
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Disable logging.
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    /// Record an event.
    pub fn record(&mut self, event: MetaVarEvent) {
        if self.enabled {
            self.events.push(event);
        }
    }
    /// Get all events.
    pub fn events(&self) -> &[MetaVarEvent] {
        &self.events
    }
    /// Count events by type.
    pub fn count_created(&self) -> usize {
        self.events
            .iter()
            .filter(|e| matches!(e, MetaVarEvent::Created { .. }))
            .count()
    }
    /// Count assignment events.
    pub fn count_assigned(&self) -> usize {
        self.events
            .iter()
            .filter(|e| matches!(e, MetaVarEvent::Assigned { .. }))
            .count()
    }
    /// Clear the log.
    pub fn clear(&mut self) {
        self.events.clear();
    }
    /// Check if logging is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
/// A handle for a checkpoint in the meta context (for error recovery).
#[derive(Debug, Clone)]
pub struct MetaCheckpoint {
    /// Number of metas at checkpoint time.
    pub meta_count: u64,
    /// Depth at checkpoint time.
    pub depth: u32,
}
/// A queue of constraints ordered for efficient solving.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ConstraintQueue {
    /// Simple (equality) constraints solved first.
    simple: Vec<MetaConstraint>,
    /// Complex (higher-order) constraints.
    complex: Vec<MetaConstraint>,
}
#[allow(dead_code)]
impl ConstraintQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a constraint, classified by complexity.
    pub fn push(&mut self, constraint: MetaConstraint) {
        let is_simple = matches!(
            &constraint.lhs,
            Expr::FVar(_) | Expr::Const(_, _) | Expr::BVar(_)
        );
        if is_simple {
            self.simple.push(constraint);
        } else {
            self.complex.push(constraint);
        }
    }
    /// Pop the next constraint to solve (simple first, then complex).
    pub fn pop(&mut self) -> Option<MetaConstraint> {
        if !self.simple.is_empty() {
            Some(self.simple.remove(0))
        } else if !self.complex.is_empty() {
            Some(self.complex.remove(0))
        } else {
            None
        }
    }
    /// Number of simple constraints.
    pub fn simple_count(&self) -> usize {
        self.simple.len()
    }
    /// Number of complex constraints.
    pub fn complex_count(&self) -> usize {
        self.complex.len()
    }
    /// Total constraints.
    pub fn len(&self) -> usize {
        self.simple.len() + self.complex.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.simple.is_empty() && self.complex.is_empty()
    }
    /// Clear all constraints.
    pub fn clear(&mut self) {
        self.simple.clear();
        self.complex.clear();
    }
    /// Get all constraints as a flat list.
    pub fn all_constraints(&self) -> Vec<&MetaConstraint> {
        self.simple.iter().chain(self.complex.iter()).collect()
    }
}
/// A dependency graph among metavariables.
///
/// Edge `(a, b)` means "metavariable `a` appears in the type or body of `b`".
#[derive(Clone, Debug, Default)]
pub struct MetaVarGraph {
    /// Adjacency list: for each meta ID, the set of meta IDs that depend on it.
    edges: HashMap<u64, HashSet<u64>>,
}
impl MetaVarGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a dependency edge: `from` appears in `to`.
    pub fn add_edge(&mut self, from: u64, to: u64) {
        self.edges.entry(from).or_default().insert(to);
    }
    /// Get all metavariables that depend on `id`.
    pub fn dependents_of(&self, id: u64) -> HashSet<u64> {
        self.edges.get(&id).cloned().unwrap_or_default()
    }
    /// Whether `id` has any dependents.
    pub fn has_dependents(&self, id: u64) -> bool {
        self.edges.get(&id).is_some_and(|s| !s.is_empty())
    }
    /// Number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.edges.len()
    }
    /// Total number of edges.
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|s| s.len()).sum()
    }
    /// Remove all edges for a node (when a meta is solved).
    pub fn remove_node(&mut self, id: u64) {
        self.edges.remove(&id);
        for deps in self.edges.values_mut() {
            deps.remove(&id);
        }
    }
    /// Compute the transitive closure of dependents for `id`.
    pub fn transitive_dependents(&self, id: u64) -> HashSet<u64> {
        let mut visited = HashSet::new();
        let mut queue = vec![id];
        while let Some(cur) = queue.pop() {
            for &dep in self.dependents_of(cur).iter() {
                if visited.insert(dep) {
                    queue.push(dep);
                }
            }
        }
        visited
    }
}
/// A single entry in the assignment history.
#[derive(Clone, Debug)]
pub struct AssignmentHistoryEntry {
    /// The step at which this assignment was made.
    pub step: u64,
    /// The metavariable ID assigned.
    pub id: u64,
    /// The assigned value (stringified).
    pub value_debug: String,
}
/// A delayed metavariable assignment.
///
/// The assignment is not immediately applied; instead, it waits for a
/// condition (e.g., another meta to be resolved) before being committed.
#[derive(Clone, Debug)]
pub struct DelayedAssignment {
    /// The metavariable to assign.
    pub target_id: u64,
    /// The value to assign.
    pub value: Expr,
    /// IDs of metavariables this assignment waits for.
    pub waiting_for: HashSet<u64>,
}
impl DelayedAssignment {
    /// Create a new delayed assignment.
    pub fn new(target_id: u64, value: Expr, waiting_for: HashSet<u64>) -> Self {
        Self {
            target_id,
            value,
            waiting_for,
        }
    }
    /// Whether all blocking metas have been resolved.
    pub fn is_ready(&self, ctx: &MetaVarContext) -> bool {
        self.waiting_for.iter().all(|&id| ctx.is_solved(id))
    }
    /// Remove newly-resolved metas from the wait set.
    pub fn update_wait_set(&mut self, ctx: &MetaVarContext) {
        self.waiting_for.retain(|&id| !ctx.is_solved(id));
    }
}
/// A named pool manager for metavariables.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct MetaVarPool {
    groups: HashMap<String, MetaVarGroup>,
}
#[allow(dead_code)]
impl MetaVarPool {
    /// Create an empty pool.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a new group.
    pub fn create_group(&mut self, label: impl Into<String>) {
        let label = label.into();
        self.groups.insert(label.clone(), MetaVarGroup::new(label));
    }
    /// Add a metavariable to a group.
    pub fn add_to_group(&mut self, group: &str, id: u64) {
        if let Some(g) = self.groups.get_mut(group) {
            g.add(id);
        }
    }
    /// Get a group by name.
    pub fn get_group(&self, label: &str) -> Option<&MetaVarGroup> {
        self.groups.get(label)
    }
    /// Close a group.
    pub fn close_group(&mut self, label: &str) {
        if let Some(g) = self.groups.get_mut(label) {
            g.close();
        }
    }
    /// Number of groups.
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
    /// Get all group labels.
    pub fn group_labels(&self) -> Vec<&str> {
        self.groups.keys().map(|s| s.as_str()).collect()
    }
    /// Check if all metavariables in all groups are solved.
    pub fn all_groups_solved(&self, ctx: &MetaVarContext) -> bool {
        self.groups.values().all(|g| g.all_solved(ctx))
    }
}
/// Priority level for solving a metavariable.
///
/// Higher priority metavariables are solved first during constraint processing.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MetaVarPriority {
    /// Lowest priority — solved last.
    Low = 0,
    /// Normal priority.
    Normal = 1,
    /// High priority — solved early.
    High = 2,
    /// Immediately solvable (e.g., from a local hypothesis).
    Immediate = 3,
}
/// The outcome of a unification attempt.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum UnificationResult {
    /// Unification succeeded; may have produced new assignments.
    Success(MetaSubstitution),
    /// Unification failed.
    Failure(String),
    /// Unification is delayed (deferred constraint).
    Delayed(MetaConstraint),
}
#[allow(dead_code)]
impl UnificationResult {
    /// Check if unification succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, UnificationResult::Success(_))
    }
    /// Check if unification failed.
    pub fn is_failure(&self) -> bool {
        matches!(self, UnificationResult::Failure(_))
    }
    /// Check if unification was delayed.
    pub fn is_delayed(&self) -> bool {
        matches!(self, UnificationResult::Delayed(_))
    }
    /// Get the substitution if successful.
    pub fn substitution(self) -> Option<MetaSubstitution> {
        match self {
            UnificationResult::Success(s) => Some(s),
            _ => None,
        }
    }
    /// Get the error message if failed.
    pub fn error_message(&self) -> Option<&str> {
        match self {
            UnificationResult::Failure(msg) => Some(msg.as_str()),
            _ => None,
        }
    }
    /// Create a trivial success (no assignments).
    pub fn trivial() -> Self {
        UnificationResult::Success(MetaSubstitution::new())
    }
    /// Create a failure with a message.
    pub fn fail(msg: impl Into<String>) -> Self {
        UnificationResult::Failure(msg.into())
    }
}
/// A pending constraint between two metavariable expressions.
///
/// Unification of metavariables produces `Unification` constraints; type
/// checking produces `Typing` constraints.
#[derive(Clone, Debug)]
pub enum PendingConstraint {
    /// `lhs` must be definitionally equal to `rhs`.
    Unification {
        /// Left-hand side expression.
        lhs: Expr,
        /// Right-hand side expression.
        rhs: Expr,
    },
    /// `expr` must have type `ty`.
    Typing {
        /// The expression to type-check.
        expr: Expr,
        /// The expected type.
        ty: Expr,
    },
    /// Metavariable `id` must be instantiated before processing `expr`.
    Delayed {
        /// The metavariable ID.
        id: u64,
        /// The expression to process after instantiation.
        expr: Expr,
    },
}
impl PendingConstraint {
    /// Return `true` if this is a unification constraint.
    pub fn is_unification(&self) -> bool {
        matches!(self, PendingConstraint::Unification { .. })
    }
    /// Return `true` if this is a typing constraint.
    pub fn is_typing(&self) -> bool {
        matches!(self, PendingConstraint::Typing { .. })
    }
    /// Return `true` if this is a delayed constraint.
    pub fn is_delayed(&self) -> bool {
        matches!(self, PendingConstraint::Delayed { .. })
    }
}
/// A log entry for metavariable assignment tracing.
#[derive(Clone, Debug)]
pub struct MetaAssignLog {
    /// Metavariable ID that was assigned.
    pub id: u64,
    /// The value assigned.
    pub value: Expr,
    /// Nesting depth at time of assignment.
    pub depth: u32,
}
/// An ordered log of metavariable assignments with timestamps.
#[derive(Clone, Debug, Default)]
pub struct MetaAssignmentHistory {
    /// Entries in chronological order.
    entries: Vec<AssignmentHistoryEntry>,
    /// Monotonic step counter.
    step: u64,
}
impl MetaAssignmentHistory {
    /// Create an empty history.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record an assignment.
    pub fn record(&mut self, id: u64, value: &Expr) {
        self.entries.push(AssignmentHistoryEntry {
            step: self.step,
            id,
            value_debug: format!("{:?}", value),
        });
        self.step += 1;
    }
    /// Number of assignments recorded.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the history is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Find all assignments for a given meta ID.
    pub fn find(&self, id: u64) -> Vec<&AssignmentHistoryEntry> {
        self.entries.iter().filter(|e| e.id == id).collect()
    }
    /// Clear all history.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.step = 0;
    }
    /// Most recent step counter.
    pub fn current_step(&self) -> u64 {
        self.step
    }
}
/// Snapshot for backtracking.
#[derive(Clone, Debug)]
pub struct MetaSnapshot {
    /// Metavariable count at snapshot time.
    pub meta_count: usize,
    /// Assignments at snapshot time.
    pub assignments: HashMap<u64, Expr>,
    /// Next ID at snapshot time.
    pub next_id: u64,
}
/// An explicit map from metavariable IDs to expressions.
///
/// This is useful for representing partial solutions separate from
/// the main `MetaVarContext`.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct MetaSubstitution {
    map: HashMap<u64, Expr>,
}
#[allow(dead_code)]
impl MetaSubstitution {
    /// Create an empty substitution.
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a mapping.
    pub fn insert(&mut self, id: u64, expr: Expr) {
        self.map.insert(id, expr);
    }
    /// Look up a mapping.
    pub fn get(&self, id: u64) -> Option<&Expr> {
        self.map.get(&id)
    }
    /// Check if the substitution is defined for `id`.
    pub fn contains(&self, id: u64) -> bool {
        self.map.contains_key(&id)
    }
    /// Remove a mapping.
    pub fn remove(&mut self, id: u64) -> Option<Expr> {
        self.map.remove(&id)
    }
    /// Number of mappings.
    pub fn len(&self) -> usize {
        self.map.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    /// Merge another substitution into this one.
    /// If a key is present in both, the `other` value takes precedence.
    pub fn merge(&mut self, other: MetaSubstitution) {
        for (id, expr) in other.map {
            self.map.insert(id, expr);
        }
    }
    /// Apply this substitution to a `MetaVarContext`.
    pub fn apply_to_ctx(&self, ctx: &mut MetaVarContext) -> usize {
        let mut applied = 0;
        for (&id, expr) in &self.map {
            if ctx.assign(id, expr.clone()) {
                applied += 1;
            }
        }
        applied
    }
    /// Compose two substitutions: `self` then `other`.
    pub fn compose(mut self, other: MetaSubstitution) -> Self {
        self.merge(other);
        self
    }
    /// Get all (id, expr) pairs.
    pub fn entries(&self) -> impl Iterator<Item = (&u64, &Expr)> {
        self.map.iter()
    }
}
/// A rich metavariable context with constraint management and assignment logging.
pub struct RichMetaContext {
    pub(crate) inner: MetaVarContext,
    pending: Vec<PendingConstraint>,
    log: Vec<MetaAssignLog>,
    log_enabled: bool,
}
impl RichMetaContext {
    /// Create an empty rich context with logging disabled.
    pub fn new() -> Self {
        Self {
            inner: MetaVarContext::new(),
            pending: Vec::new(),
            log: Vec::new(),
            log_enabled: false,
        }
    }
    /// Enable assignment logging.
    pub fn enable_logging(&mut self) {
        self.log_enabled = true;
    }
    /// Disable assignment logging.
    pub fn disable_logging(&mut self) {
        self.log_enabled = false;
    }
    /// Create a fresh metavariable.
    pub fn fresh(&mut self, ty: Expr) -> u64 {
        self.inner.fresh(ty)
    }
    /// Assign a metavariable, recording to the log if enabled.
    pub fn assign(&mut self, id: u64, value: Expr) -> bool {
        let depth = self.inner.depth;
        let ok = self.inner.assign(id, value.clone());
        if ok && self.log_enabled {
            self.log.push(MetaAssignLog { id, value, depth });
        }
        ok
    }
    /// Look up an assignment.
    pub fn get(&self, id: u64) -> Option<&Expr> {
        self.inner.get_assignment(id)
    }
    /// Add a pending constraint.
    pub fn push_constraint(&mut self, c: PendingConstraint) {
        self.pending.push(c);
    }
    /// Take all pending constraints.
    pub fn take_constraints(&mut self) -> Vec<PendingConstraint> {
        std::mem::take(&mut self.pending)
    }
    /// Count pending constraints.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    /// Count unification constraints.
    pub fn unification_count(&self) -> usize {
        self.pending.iter().filter(|c| c.is_unification()).count()
    }
    /// Get the assignment log.
    pub fn log(&self) -> &[MetaAssignLog] {
        &self.log
    }
    /// Clear the assignment log.
    pub fn clear_log(&mut self) {
        self.log.clear();
    }
    /// Take a snapshot of the inner context.
    pub fn snapshot(&self) -> MetaSnapshot {
        self.inner.snapshot()
    }
    /// Restore from a snapshot.
    pub fn restore(&mut self, snap: MetaSnapshot) {
        self.inner.restore(&snap);
        let limit = self.inner.next_id;
        self.log.retain(|e| e.id < limit);
    }
    /// Push elaboration depth.
    pub fn push_depth(&mut self) {
        self.inner.push_depth();
    }
    /// Pop elaboration depth.
    pub fn pop_depth(&mut self) {
        self.inner.pop_depth();
    }
    /// Get current depth.
    pub fn depth(&self) -> u32 {
        self.inner.depth
    }
    /// Check if all metas are assigned.
    pub fn is_fully_assigned(&self) -> bool {
        self.inner.metas.values().all(|m| m.is_assigned())
    }
    /// Return unassigned metavariable IDs.
    pub fn unassigned_ids(&self) -> Vec<u64> {
        self.inner
            .metas
            .iter()
            .filter(|(_, m)| !m.is_assigned())
            .map(|(id, _)| *id)
            .collect()
    }
}
/// Kind of metavariable.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MetaVarKind {
    /// Natural: assigned freely by unification.
    Natural,
    /// Synthetic: created by the elaborator.
    Synthetic,
    /// Synthetic-opaque: solved only by tactics.
    SyntheticOpaque,
}
/// A unification constraint.
#[derive(Clone, Debug)]
pub struct MetaConstraint {
    /// Left-hand side.
    pub lhs: Expr,
    /// Right-hand side.
    pub rhs: Expr,
    /// Origin description.
    pub origin: String,
}
impl MetaConstraint {
    /// Create an equality constraint.
    pub fn new_eq(lhs: Expr, rhs: Expr, origin: impl Into<String>) -> Self {
        Self {
            lhs,
            rhs,
            origin: origin.into(),
        }
    }
}
/// A metavariable with its type and optional assignment.
#[derive(Clone, Debug)]
pub struct MetaVar {
    /// Metavariable ID.
    pub id: u64,
    /// Expected type.
    pub ty: Expr,
    /// Current assignment.
    pub assignment: Option<Expr>,
    /// Kind.
    pub kind: MetaVarKind,
    /// User-facing name.
    pub user_name: Option<String>,
    /// Creation depth.
    pub depth: u32,
    /// Local variable FVar IDs that were in scope when this metavariable was created.
    /// Used to enforce scope safety during assignment.
    pub scope_vars: Vec<u64>,
}
impl MetaVar {
    /// Create a natural metavariable.
    pub fn new(id: u64, ty: Expr) -> Self {
        Self {
            id,
            ty,
            assignment: None,
            kind: MetaVarKind::Natural,
            user_name: None,
            depth: 0,
            scope_vars: Vec::new(),
        }
    }
    /// Create with a specific kind.
    pub fn with_kind(id: u64, ty: Expr, kind: MetaVarKind) -> Self {
        Self {
            id,
            ty,
            assignment: None,
            kind,
            user_name: None,
            depth: 0,
            scope_vars: Vec::new(),
        }
    }
    /// Set user name.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.user_name = Some(name.into());
        self
    }
    /// Set depth.
    pub fn with_depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }
    /// Set the scope variables (FVar IDs in scope at creation time).
    pub fn with_scope(mut self, scope_vars: Vec<u64>) -> Self {
        self.scope_vars = scope_vars;
        self
    }
    /// Check whether a FVar ID is in scope for this metavariable.
    pub fn fvar_in_scope(&self, fvar_id: u64) -> bool {
        if fvar_id >= 1_000_000 {
            return true;
        }
        self.scope_vars.contains(&fvar_id)
    }
    /// Check if solved.
    pub fn is_solved(&self) -> bool {
        self.assignment.is_some()
    }
    /// Check if assigned (alias for is_solved).
    pub fn is_assigned(&self) -> bool {
        self.assignment.is_some()
    }
    /// Check if natural.
    pub fn is_natural(&self) -> bool {
        self.kind == MetaVarKind::Natural
    }
    /// Check if synthetic.
    pub fn is_synthetic(&self) -> bool {
        self.kind == MetaVarKind::Synthetic
    }
    /// Check if synthetic-opaque.
    pub fn is_synthetic_opaque(&self) -> bool {
        self.kind == MetaVarKind::SyntheticOpaque
    }
    /// Assign a value.
    pub fn assign(&mut self, expr: Expr) {
        self.assignment = Some(expr);
    }
    /// Force-reassign.
    pub fn reassign(&mut self, expr: Expr) {
        self.assignment = Some(expr);
    }
    /// Clear assignment.
    pub fn clear_assignment(&mut self) {
        self.assignment = None;
    }
    /// Display name.
    pub fn display_name(&self) -> String {
        self.user_name
            .clone()
            .unwrap_or_else(|| format!("?m_{}", self.id))
    }
}
/// Union-Find structure for metavariable equivalence classes.
///
/// When two metavariables are unified, they are merged into the same class.
#[derive(Clone, Debug, Default)]
pub struct MetaEqClass {
    parent: HashMap<u64, u64>,
    rank: HashMap<u64, u32>,
}
impl MetaEqClass {
    /// Create an empty structure.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a metavariable as its own class.
    pub fn add(&mut self, id: u64) {
        self.parent.entry(id).or_insert(id);
        self.rank.entry(id).or_insert(0);
    }
    /// Find the representative of a meta's class (with path compression).
    pub fn find(&mut self, id: u64) -> u64 {
        self.add(id);
        // Safety: self.add(id) guarantees the key exists in self.parent
        let parent = *self
            .parent
            .get(&id)
            .expect("id was just added via self.add");
        if parent == id {
            id
        } else {
            let root = self.find(parent);
            // Safety: self.add(id) guarantees the key exists
            *self.parent.get_mut(&id).expect("id was added via self.add") = root;
            root
        }
    }
    /// Union two metas into the same class.
    pub fn union(&mut self, a: u64, b: u64) {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb {
            return;
        }
        let rank_a = *self.rank.get(&ra).unwrap_or(&0);
        let rank_b = *self.rank.get(&rb).unwrap_or(&0);
        // Safety: find() calls add() which guarantees keys exist in self.parent
        if rank_a < rank_b {
            *self.parent.get_mut(&ra).expect("ra was added via find") = rb;
        } else if rank_a > rank_b {
            *self.parent.get_mut(&rb).expect("rb was added via find") = ra;
        } else {
            *self.parent.get_mut(&rb).expect("rb was added via find") = ra;
            *self.rank.get_mut(&ra).expect("ra was added via find") += 1;
        }
    }
    /// Whether two metas are in the same equivalence class.
    pub fn same_class(&mut self, a: u64, b: u64) -> bool {
        self.find(a) == self.find(b)
    }
    /// Number of distinct classes.
    pub fn class_count(&mut self) -> usize {
        let keys: Vec<u64> = self.parent.keys().copied().collect();
        let mut roots: HashSet<u64> = HashSet::new();
        for k in keys {
            roots.insert(self.find(k));
        }
        roots.len()
    }
}
