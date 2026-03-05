//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Environment, Expr, FVarId, Name};
use std::collections::HashMap;

use super::elabcontext_type::ElabContext;

/// Local variable entry in the elaboration context.
#[derive(Clone, Debug)]
pub struct LocalEntry {
    /// Free variable ID unique within this elaboration.
    pub fvar: FVarId,
    /// User-visible name.
    pub name: Name,
    /// Declared type.
    pub ty: Expr,
    /// Optional value (for let-bound variables).
    pub val: Option<Expr>,
    /// Kind of binding.
    pub kind: LocalKind,
    /// Elaboration depth at which this variable was introduced.
    pub depth: u32,
}
impl LocalEntry {
    /// Create a hypothesis entry.
    pub fn hypothesis(fvar: FVarId, name: Name, ty: Expr, depth: u32) -> Self {
        Self {
            fvar,
            name,
            ty,
            val: None,
            kind: LocalKind::Hypothesis,
            depth,
        }
    }
    /// Create a let-binding entry.
    pub fn let_binding(fvar: FVarId, name: Name, ty: Expr, val: Expr, depth: u32) -> Self {
        Self {
            fvar,
            name,
            ty,
            val: Some(val),
            kind: LocalKind::LetBinding,
            depth,
        }
    }
    /// Check whether this is a let-binding.
    pub fn is_let(&self) -> bool {
        self.kind == LocalKind::LetBinding
    }
    /// Check whether this is a hypothesis.
    pub fn is_hypothesis(&self) -> bool {
        self.kind == LocalKind::Hypothesis
    }
}
/// Configuration options for the elaborator.
#[derive(Clone, Debug)]
pub struct ElabOptions {
    /// Whether to allow sorry in proofs.
    pub allow_sorry: bool,
    /// Whether to auto-bound implicit variables.
    pub auto_bound_implicit: bool,
    /// Maximum universe level.
    pub max_universe: u32,
    /// Whether to emit warnings for unused hypotheses.
    pub warn_unused_hyps: bool,
    /// Whether definitional equality checking is strict.
    pub strict_def_eq: bool,
}
/// A lightweight summary of the current elaboration context for display.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ContextSummary {
    /// Depth of the context.
    pub depth: u32,
    /// Names of all local hypotheses.
    pub hypotheses: Vec<Name>,
    /// Names of all let-bound locals.
    pub let_bindings: Vec<Name>,
    /// Number of open goals.
    pub open_goals: usize,
    /// Number of unassigned metavariables.
    pub open_metas: usize,
}
impl ContextSummary {
    /// Compute a summary for the given context.
    #[allow(dead_code)]
    pub fn of(ctx: &ElabContext<'_>) -> Self {
        let hypotheses = ctx
            .local_view()
            .all()
            .iter()
            .filter(|e| e.is_hypothesis())
            .map(|e| e.name.clone())
            .collect();
        let let_bindings = ctx
            .local_view()
            .all()
            .iter()
            .filter(|e| e.is_let())
            .map(|e| e.name.clone())
            .collect();
        Self {
            depth: ctx.depth(),
            hypotheses,
            let_bindings,
            open_goals: ctx.goal_count(),
            open_metas: (ctx.meta_count() as usize).saturating_sub(ctx.assigned_meta_count()),
        }
    }
    /// Whether the context is "clean" (no open goals or metas).
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.open_goals == 0 && self.open_metas == 0
    }
}
/// A dependency record for a metavariable — which other metavars must be
/// solved before this one can be attempted.
#[derive(Debug, Clone)]
pub struct MetaDependency {
    /// The ID of the dependent metavariable.
    pub id: u64,
    /// IDs of metavariables that this one depends on.
    pub depends_on: Vec<u64>,
}
impl MetaDependency {
    /// Create a new dependency record.
    #[allow(dead_code)]
    pub fn new(id: u64, depends_on: Vec<u64>) -> Self {
        Self { id, depends_on }
    }
    /// Check if this metavar has no dependencies.
    #[allow(dead_code)]
    pub fn is_independent(&self) -> bool {
        self.depends_on.is_empty()
    }
    /// Check if this metavar depends on a specific other metavar.
    #[allow(dead_code)]
    pub fn depends_on_id(&self, other: u64) -> bool {
        self.depends_on.contains(&other)
    }
}
/// Statistics about an elaboration context.
#[derive(Debug, Default, Clone)]
pub struct ContextStats {
    /// Total locals.
    pub local_count: usize,
    /// Hypothesis locals.
    pub hypothesis_count: usize,
    /// Let-bound locals.
    pub let_count: usize,
    /// Metavariables created.
    pub meta_count: u64,
    /// Metavariables assigned.
    pub meta_assigned: usize,
    /// Pending goals.
    pub goal_count: usize,
    /// Current depth.
    pub depth: u32,
}
impl ContextStats {
    /// Compute statistics for an elaboration context.
    pub fn compute(ctx: &ElabContext<'_>) -> Self {
        Self {
            local_count: ctx.local_count(),
            hypothesis_count: ctx.hypothesis_count(),
            let_count: ctx.let_count(),
            meta_count: ctx.meta_count(),
            meta_assigned: ctx.assigned_meta_count(),
            goal_count: ctx.goal_count(),
            depth: ctx.depth(),
        }
    }
    /// Fraction of metas assigned (0.0 if none created).
    pub fn meta_completion(&self) -> f64 {
        if self.meta_count == 0 {
            1.0
        } else {
            self.meta_assigned as f64 / self.meta_count as f64
        }
    }
}
/// Priority of a proof goal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum GoalPriority {
    /// The goal should be solved first.
    High = 0,
    /// Normal priority goal.
    #[default]
    Normal = 1,
    /// Deferred goal (solve last).
    Low = 2,
}
/// A proof goal with associated priority and metadata.
#[derive(Debug, Clone)]
pub struct PrioritizedGoal {
    /// The goal expression.
    pub goal: oxilean_kernel::Expr,
    /// Priority.
    pub priority: GoalPriority,
    /// Optional label.
    pub label: Option<String>,
    /// Depth at which this goal was created.
    pub depth: u32,
}
impl PrioritizedGoal {
    /// Create a normal-priority goal.
    #[allow(dead_code)]
    pub fn normal(goal: oxilean_kernel::Expr, depth: u32) -> Self {
        Self {
            goal,
            priority: GoalPriority::Normal,
            label: None,
            depth,
        }
    }
    /// Create a high-priority goal.
    #[allow(dead_code)]
    pub fn high(goal: oxilean_kernel::Expr, depth: u32) -> Self {
        Self {
            goal,
            priority: GoalPriority::High,
            label: None,
            depth,
        }
    }
    /// Create a low-priority goal.
    #[allow(dead_code)]
    pub fn low(goal: oxilean_kernel::Expr, depth: u32) -> Self {
        Self {
            goal,
            priority: GoalPriority::Low,
            label: None,
            depth,
        }
    }
    /// Attach a label.
    #[allow(dead_code)]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}
/// A queue of proof goals sorted by priority.
#[derive(Debug, Default)]
pub struct GoalQueue {
    goals: Vec<PrioritizedGoal>,
}
impl GoalQueue {
    /// Create a new empty goal queue.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a goal (inserted in priority order).
    #[allow(dead_code)]
    pub fn push(&mut self, goal: PrioritizedGoal) {
        let pos = self.goals.partition_point(|g| g.priority <= goal.priority);
        self.goals.insert(pos, goal);
    }
    /// Pop the highest-priority goal.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<PrioritizedGoal> {
        if self.goals.is_empty() {
            None
        } else {
            Some(self.goals.remove(0))
        }
    }
    /// Peek at the highest-priority goal.
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&PrioritizedGoal> {
        self.goals.first()
    }
    /// Return the number of goals.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.goals.len()
    }
    /// Whether empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.goals.is_empty()
    }
    /// Clear all goals.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.goals.clear();
    }
    /// Count goals by priority.
    #[allow(dead_code)]
    pub fn count_by_priority(&self, priority: GoalPriority) -> usize {
        self.goals.iter().filter(|g| g.priority == priority).count()
    }
}
/// A diff between two local contexts (added and removed entries).
#[derive(Debug, Clone)]
pub struct ContextDiff {
    /// Entries added in the new context (relative to old).
    pub added: Vec<LocalEntry>,
    /// FVar IDs removed from the old context.
    pub removed: Vec<FVarId>,
}
impl ContextDiff {
    /// Compute the diff between an old and new list of local entries.
    #[allow(dead_code)]
    pub fn compute(old: &[LocalEntry], new: &[LocalEntry]) -> Self {
        let old_ids: std::collections::HashSet<u64> = old.iter().map(|e| e.fvar.0).collect();
        let new_ids: std::collections::HashSet<u64> = new.iter().map(|e| e.fvar.0).collect();
        let added = new
            .iter()
            .filter(|e| !old_ids.contains(&e.fvar.0))
            .cloned()
            .collect();
        let removed = old
            .iter()
            .filter(|e| !new_ids.contains(&e.fvar.0))
            .map(|e| e.fvar)
            .collect();
        Self { added, removed }
    }
    /// Whether there are no changes.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    /// Number of added entries.
    #[allow(dead_code)]
    pub fn added_count(&self) -> usize {
        self.added.len()
    }
    /// Number of removed entries.
    #[allow(dead_code)]
    pub fn removed_count(&self) -> usize {
        self.removed.len()
    }
}
/// A read-only view of the local context as a stack of hypotheses.
pub struct LocalView<'a> {
    entries: &'a [LocalEntry],
}
impl<'a> LocalView<'a> {
    /// Create a view from a slice of entries.
    pub fn new(entries: &'a [LocalEntry]) -> Self {
        Self { entries }
    }
    /// Get all entries.
    pub fn all(&self) -> &[LocalEntry] {
        self.entries
    }
    /// Find an entry by name.
    pub fn find(&self, name: &Name) -> Option<&LocalEntry> {
        self.entries.iter().rev().find(|e| &e.name == name)
    }
    /// Count entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Return only hypothesis entries.
    pub fn hypotheses(&self) -> Vec<&LocalEntry> {
        self.entries.iter().filter(|e| e.is_hypothesis()).collect()
    }
    /// Return only let-bound entries.
    pub fn let_entries(&self) -> Vec<&LocalEntry> {
        self.entries.iter().filter(|e| e.is_let()).collect()
    }
    /// Get all names in scope.
    pub fn names(&self) -> Vec<&Name> {
        self.entries.iter().map(|e| &e.name).collect()
    }
}
/// A map renaming local variables (for alpha-renaming during elaboration).
#[derive(Debug, Clone, Default)]
pub struct RenameMap {
    map: std::collections::HashMap<FVarId, Name>,
}
impl RenameMap {
    /// Create a new empty rename map.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a renaming.
    #[allow(dead_code)]
    pub fn add(&mut self, from: FVarId, to: Name) {
        self.map.insert(from, to);
    }
    /// Look up a renaming.
    #[allow(dead_code)]
    pub fn get(&self, fvar: FVarId) -> Option<&Name> {
        self.map.get(&fvar)
    }
    /// Apply renaming to a local entry.
    #[allow(dead_code)]
    pub fn apply_to_entry(&self, entry: &LocalEntry) -> LocalEntry {
        let mut result = entry.clone();
        if let Some(new_name) = self.map.get(&entry.fvar) {
            result.name = new_name.clone();
        }
        result
    }
    /// Return the number of renamings.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.map.len()
    }
    /// Whether empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    /// Merge another rename map into this one (other's entries take priority).
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &RenameMap) {
        for (&fvar, name) in &other.map {
            self.map.insert(fvar, name.clone());
        }
    }
}
/// Kind of local variable binding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocalKind {
    /// Regular function parameter / hypothesis.
    Hypothesis,
    /// Let-bound definition.
    LetBinding,
    /// Introduced by a `have` tactic step.
    Have,
    /// Auxiliary variable created by the elaborator.
    Auxiliary,
}
/// A saved snapshot of an `ElabContext` for backtracking.
pub struct ElabSnapshot {
    pub local_count: usize,
    pub meta_count: u64,
    pub depth: u32,
    pub goal_count: usize,
}
/// A builder for constructing local entries fluently.
#[derive(Debug, Clone)]
pub struct LocalEntryBuilder {
    fvar: FVarId,
    name: Name,
    ty: oxilean_kernel::Expr,
    val: Option<oxilean_kernel::Expr>,
    kind: LocalKind,
    depth: u32,
}
impl LocalEntryBuilder {
    /// Start building a hypothesis entry.
    #[allow(dead_code)]
    pub fn hypothesis(fvar: FVarId, name: Name, ty: oxilean_kernel::Expr) -> Self {
        Self {
            fvar,
            name,
            ty,
            val: None,
            kind: LocalKind::Hypothesis,
            depth: 0,
        }
    }
    /// Start building a let-binding entry.
    #[allow(dead_code)]
    pub fn let_binding(
        fvar: FVarId,
        name: Name,
        ty: oxilean_kernel::Expr,
        val: oxilean_kernel::Expr,
    ) -> Self {
        Self {
            fvar,
            name,
            ty,
            val: Some(val),
            kind: LocalKind::LetBinding,
            depth: 0,
        }
    }
    /// Set the depth.
    #[allow(dead_code)]
    pub fn with_depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }
    /// Set the kind.
    #[allow(dead_code)]
    pub fn with_kind(mut self, kind: LocalKind) -> Self {
        self.kind = kind;
        self
    }
    /// Set the value.
    #[allow(dead_code)]
    pub fn with_val(mut self, val: oxilean_kernel::Expr) -> Self {
        self.val = Some(val);
        self
    }
    /// Build the LocalEntry.
    #[allow(dead_code)]
    pub fn build(self) -> LocalEntry {
        LocalEntry {
            fvar: self.fvar,
            name: self.name,
            ty: self.ty,
            val: self.val,
            kind: self.kind,
            depth: self.depth,
        }
    }
}
/// An iterator adapter that yields only let-binding entries.
#[allow(dead_code)]
pub struct LetBindingIter<'a> {
    pub(super) inner: std::slice::Iter<'a, LocalEntry>,
}
impl<'a> LetBindingIter<'a> {
    /// Create a new let-binding iterator.
    #[allow(dead_code)]
    pub fn new(entries: &'a [LocalEntry]) -> Self {
        Self {
            inner: entries.iter(),
        }
    }
}
/// A depth guard: records depth on creation, restores on drop via an explicit
/// method call (no implicit Drop side effects — pure value type).
#[derive(Debug, Clone, Copy)]
pub struct DepthGuard {
    /// The depth recorded when this guard was created.
    pub saved_depth: u32,
}
impl DepthGuard {
    /// Save the current depth.
    #[allow(dead_code)]
    pub fn save(depth: u32) -> Self {
        Self { saved_depth: depth }
    }
    /// Return the saved depth.
    #[allow(dead_code)]
    pub fn depth(&self) -> u32 {
        self.saved_depth
    }
}
/// A builder for constructing an `ElabContext` with specific options.
pub struct ElabContextBuilder<'env> {
    env: &'env oxilean_kernel::Environment,
    options: ElabOptions,
}
impl<'env> ElabContextBuilder<'env> {
    /// Create a new builder for the given environment.
    pub fn new(env: &'env oxilean_kernel::Environment) -> Self {
        Self {
            env,
            options: ElabOptions::default(),
        }
    }
    /// Allow sorry in proofs.
    pub fn allow_sorry(mut self) -> Self {
        self.options.allow_sorry = true;
        self
    }
    /// Disable auto-bound implicits.
    pub fn no_auto_bound(mut self) -> Self {
        self.options.auto_bound_implicit = false;
        self
    }
    /// Warn on unused hypotheses.
    pub fn warn_unused_hyps(mut self) -> Self {
        self.options.warn_unused_hyps = true;
        self
    }
    /// Set strict definitional equality checking.
    pub fn strict(mut self) -> Self {
        self.options.strict_def_eq = true;
        self
    }
    /// Relax definitional equality checking.
    pub fn relaxed(mut self) -> Self {
        self.options.strict_def_eq = false;
        self
    }
    /// Build the context.
    pub fn build(self) -> ElabContext<'env> {
        ElabContext::with_options(self.env, self.options)
    }
}
/// A simple counter for generating fresh FVarIds.
#[derive(Debug, Clone, Default)]
pub struct FVarIdGen {
    counter: u64,
}
impl FVarIdGen {
    /// Create a new generator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Generate a fresh FVarId.
    #[allow(dead_code)]
    pub fn fresh(&mut self) -> FVarId {
        let id = self.counter;
        self.counter += 1;
        FVarId(id)
    }
    /// Peek at what the next ID will be (without consuming it).
    #[allow(dead_code)]
    pub fn peek_next(&self) -> u64 {
        self.counter
    }
    /// Reset the counter.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.counter = 0;
    }
}
/// A checkpoint of the context state that can be used to roll back.
///
/// Captures the lengths of the local variable stack, metavariable list,
/// and goal list so that partial elaboration can be undone.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ContextCheckpoint {
    /// Number of locals at checkpoint.
    pub local_len: usize,
    /// Number of metas at checkpoint.
    pub meta_len: u64,
    /// Number of goals at checkpoint.
    pub goal_len: usize,
    /// Elaboration depth at checkpoint.
    pub depth: u32,
}
impl ContextCheckpoint {
    /// Create a checkpoint from the current context state.
    #[allow(dead_code)]
    pub fn take(ctx: &ElabContext<'_>) -> Self {
        Self {
            local_len: ctx.local_count(),
            meta_len: ctx.meta_count(),
            goal_len: ctx.goal_count(),
            depth: ctx.depth(),
        }
    }
}
/// Detailed statistics about a local context.
#[derive(Debug, Clone, Default)]
pub struct LocalContextStats {
    /// Total number of local entries.
    pub total_locals: usize,
    /// Number of hypothesis entries.
    pub hypothesis_count: usize,
    /// Number of let-binding entries.
    pub let_binding_count: usize,
    /// Number of have-binding entries.
    pub have_count: usize,
    /// Number of auxiliary entries.
    pub aux_count: usize,
    /// Maximum depth of any local.
    pub max_depth: u32,
    /// Number of locals with values (let/have).
    pub valued_count: usize,
}
impl LocalContextStats {
    /// Compute statistics for a list of local entries.
    #[allow(dead_code)]
    pub fn of(entries: &[LocalEntry]) -> Self {
        let mut stats = Self::default();
        stats.total_locals = entries.len();
        for e in entries {
            match e.kind {
                LocalKind::Hypothesis => stats.hypothesis_count += 1,
                LocalKind::LetBinding => stats.let_binding_count += 1,
                LocalKind::Have => stats.have_count += 1,
                LocalKind::Auxiliary => stats.aux_count += 1,
            }
            if e.depth > stats.max_depth {
                stats.max_depth = e.depth;
            }
            if e.val.is_some() {
                stats.valued_count += 1;
            }
        }
        stats
    }
    /// Whether the context is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.total_locals == 0
    }
    /// Proportion of locals that have let-bindings.
    #[allow(dead_code)]
    pub fn let_fraction(&self) -> f64 {
        if self.total_locals == 0 {
            0.0
        } else {
            self.let_binding_count as f64 / self.total_locals as f64
        }
    }
}
/// A hypothesis group: a named collection of local variables sharing a common
/// source (e.g., "destructured from `h : A ∧ B`").
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HypGroup {
    /// Group label (e.g., the original hypothesis name).
    pub label: Name,
    /// FVar IDs of all locals in this group.
    pub members: Vec<FVarId>,
}
impl HypGroup {
    /// Create a new group with a given label and member list.
    #[allow(dead_code)]
    pub fn new(label: Name, members: Vec<FVarId>) -> Self {
        Self { label, members }
    }
    /// Check if the given fvar is a member of this group.
    #[allow(dead_code)]
    pub fn contains(&self, fvar: FVarId) -> bool {
        self.members.contains(&fvar)
    }
    /// Number of members in this group.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.members.len()
    }
    /// Whether the group has no members.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}
/// A snapshot of the types assigned to free variables at a point in time.
#[derive(Debug, Clone, Default)]
pub struct TypeSnapshot {
    /// Mapping from FVarId to type.
    pub types: std::collections::HashMap<FVarId, oxilean_kernel::Expr>,
    /// Snapshot depth.
    pub depth: u32,
}
impl TypeSnapshot {
    /// Create a new empty type snapshot.
    #[allow(dead_code)]
    pub fn new(depth: u32) -> Self {
        Self {
            types: std::collections::HashMap::new(),
            depth,
        }
    }
    /// Record the type of a free variable.
    #[allow(dead_code)]
    pub fn record(&mut self, fvar: FVarId, ty: oxilean_kernel::Expr) {
        self.types.insert(fvar, ty);
    }
    /// Look up a type.
    #[allow(dead_code)]
    pub fn lookup(&self, fvar: FVarId) -> Option<&oxilean_kernel::Expr> {
        self.types.get(&fvar)
    }
    /// Number of recorded types.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.types.len()
    }
    /// Whether empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }
    /// Build a snapshot from a list of local entries.
    #[allow(dead_code)]
    pub fn from_locals(entries: &[LocalEntry]) -> Self {
        let mut snap = Self::new(entries.last().map(|e| e.depth).unwrap_or(0));
        for entry in entries {
            snap.record(entry.fvar, entry.ty.clone());
        }
        snap
    }
}
/// An iterator adapter that yields only hypothesis entries.
#[allow(dead_code)]
pub struct HypothesisIter<'a> {
    pub(super) inner: std::slice::Iter<'a, LocalEntry>,
}
impl<'a> HypothesisIter<'a> {
    /// Create a new hypothesis iterator.
    #[allow(dead_code)]
    pub fn new(entries: &'a [LocalEntry]) -> Self {
        Self {
            inner: entries.iter(),
        }
    }
}
/// Result of validating a local context.
#[derive(Debug, Clone)]
pub struct ContextValidation {
    /// Whether the context is valid.
    pub is_valid: bool,
    /// Validation errors.
    pub errors: Vec<String>,
    /// Validation warnings.
    pub warnings: Vec<String>,
}
impl ContextValidation {
    /// Create an empty (valid) validation result.
    #[allow(dead_code)]
    pub fn ok() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    /// Add an error and mark as invalid.
    #[allow(dead_code)]
    pub fn add_error(&mut self, msg: impl Into<String>) {
        self.is_valid = false;
        self.errors.push(msg.into());
    }
    /// Add a warning.
    #[allow(dead_code)]
    pub fn add_warning(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }
    /// Whether there are no errors or warnings.
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.is_valid && self.warnings.is_empty()
    }
}
