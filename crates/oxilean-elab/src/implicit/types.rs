//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::metavar::MetaVarContext;
use oxilean_kernel::{alpha_equiv, const_name, expr_head, BinderInfo, Expr, FVarId, Name};

use std::collections::HashMap;

/// Extended result of the full implicit pipeline, including statistics.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ImplicitPipelineResult {
    /// The resulting expression with all inserted implicits applied.
    pub expr: Expr,
    /// Remaining type after implicit binders are stripped.
    pub remaining_ty: Expr,
    /// Records of inserted implicits.
    pub inserted: Vec<InsertedImplicit>,
}
/// A simple table of type-class instances.
///
/// Maps a class name to a list of instance expressions.
#[derive(Clone, Debug, Default)]
pub struct InstanceTable {
    entries: std::collections::HashMap<String, Vec<Expr>>,
}
impl InstanceTable {
    /// Create an empty table.
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
    /// Register an instance for `class_name`.
    pub fn register(&mut self, class_name: impl Into<String>, instance: Expr) {
        self.entries
            .entry(class_name.into())
            .or_default()
            .push(instance);
    }
    /// Look up instances for `class_name`.
    pub fn lookup(&self, class_name: &str) -> &[Expr] {
        self.entries
            .get(class_name)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
    /// `true` if any instances are registered for `class_name`.
    pub fn has_instances(&self, class_name: &str) -> bool {
        !self.lookup(class_name).is_empty()
    }
    /// Total number of registered instances across all classes.
    pub fn total(&self) -> usize {
        self.entries.values().map(Vec::len).sum()
    }
    /// Return all class names for which instances are registered.
    pub fn class_names(&self) -> impl Iterator<Item = &String> {
        self.entries.keys()
    }
}
/// An auto-implicit scope tracks which free names have been auto-implicitly
/// bound in the current declaration.
#[derive(Clone, Debug, Default)]
pub struct AutoImplicitScope {
    bound: Vec<String>,
}
impl AutoImplicitScope {
    /// Create a new empty scope.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a name as auto-implicitly bound.
    pub fn bind(&mut self, name: impl Into<String>) {
        let n = name.into();
        if !self.bound.contains(&n) {
            self.bound.push(n);
        }
    }
    /// Whether a name is already bound.
    pub fn is_bound(&self, name: &str) -> bool {
        self.bound.contains(&name.to_string())
    }
    /// Number of auto-implicit variables.
    pub fn len(&self) -> usize {
        self.bound.len()
    }
    /// Whether the scope is empty.
    pub fn is_empty(&self) -> bool {
        self.bound.is_empty()
    }
    /// Return the ordered list of auto-implicit names.
    pub fn names(&self) -> &[String] {
        &self.bound
    }
}
/// A record of an implicit argument that was left unsolved (pending synthesis).
#[derive(Clone, Debug)]
pub struct PendingImplicit {
    /// The position (argument index) of this implicit.
    pub arg_index: usize,
    /// The expected type of the argument.
    pub expected_ty: Expr,
    /// The mode for resolution.
    pub mode: ImplicitMode,
    /// Whether this argument was already provided by the user.
    pub user_provided: bool,
}
impl PendingImplicit {
    /// Create a new pending implicit.
    pub fn new(arg_index: usize, expected_ty: Expr, mode: ImplicitMode) -> Self {
        Self {
            arg_index,
            expected_ty,
            mode,
            user_provided: false,
        }
    }
    /// Mark this argument as user-provided.
    pub fn mark_user_provided(mut self) -> Self {
        self.user_provided = true;
        self
    }
}
/// Result of elaborating a single implicit argument.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum ImplicitElabResult {
    /// Successfully inferred a concrete expression.
    Inferred(Expr),
    /// Left as a metavariable (not yet solved).
    Pending(u64),
    /// Solved by type-class synthesis.
    Synthesized(Expr),
}
impl ImplicitElabResult {
    /// Extract the expression, regardless of variant.
    #[allow(dead_code)]
    pub fn into_expr(self, metas: &MetaVarContext) -> Option<Expr> {
        match self {
            ImplicitElabResult::Inferred(e) => Some(e),
            ImplicitElabResult::Synthesized(e) => Some(e),
            ImplicitElabResult::Pending(id) => metas.get_assignment(id).cloned(),
        }
    }
    /// Whether the result is still pending.
    #[allow(dead_code)]
    pub fn is_pending(&self) -> bool {
        matches!(self, ImplicitElabResult::Pending(_))
    }
}
/// Information about a single implicit argument of a function.
#[derive(Clone, Debug)]
pub struct ImplicitArg {
    /// Argument name (for diagnostics).
    pub name: String,
    /// Expected type of the argument.
    pub ty: Expr,
    /// Whether this is a type-class instance argument.
    pub is_instance: bool,
}
impl ImplicitArg {
    /// Construct a regular implicit argument.
    pub fn implicit(name: impl Into<String>, ty: Expr) -> Self {
        Self {
            name: name.into(),
            ty,
            is_instance: false,
        }
    }
    /// Construct a type-class instance argument.
    pub fn instance(name: impl Into<String>, ty: Expr) -> Self {
        Self {
            name: name.into(),
            ty,
            is_instance: true,
        }
    }
}
/// Cumulative statistics for implicit argument insertion.
#[derive(Clone, Debug, Default)]
pub struct ImplicitInsertionStats {
    /// Total implicit arguments inserted.
    pub inserted: u64,
    /// Total expressions processed.
    pub exprs_processed: u64,
    /// Unification hints used.
    pub hints_used: u64,
    /// Synthesis failures.
    pub failures: u64,
}
impl ImplicitInsertionStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record one expression processing outcome.
    pub fn record(&mut self, inserted: u64, ok: bool) {
        self.exprs_processed += 1;
        self.inserted += inserted;
        if !ok {
            self.failures += 1;
        }
    }
    /// Average implicits inserted per expression.
    pub fn avg_inserted(&self) -> f64 {
        if self.exprs_processed == 0 {
            0.0
        } else {
            self.inserted as f64 / self.exprs_processed as f64
        }
    }
    /// Summary string.
    pub fn summary(&self) -> String {
        format!(
            "inserted={} exprs={} hints={} failures={}",
            self.inserted, self.exprs_processed, self.hints_used, self.failures
        )
    }
}
/// Direction of type-checking for implicit argument inference.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckDirection {
    /// Infer the type from the expression (synthesis mode).
    Infer,
    /// Check the expression against a given type (check mode).
    Check,
}
impl CheckDirection {
    /// Whether this is infer mode.
    #[allow(dead_code)]
    pub fn is_infer(self) -> bool {
        self == CheckDirection::Infer
    }
    /// Whether this is check mode.
    #[allow(dead_code)]
    pub fn is_check(self) -> bool {
        self == CheckDirection::Check
    }
}
/// Classify each binder in a Pi-type as explicit or implicit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArgClass {
    /// Ordinary explicit argument.
    Explicit,
    /// Implicit `{…}` argument.
    Implicit,
    /// Instance `[…]` argument.
    Instance,
    /// Strict `⦃…⦄` argument.
    Strict,
}
impl ArgClass {
    /// Construct from a `BinderInfo`.
    pub fn from_binder(bi: &BinderInfo) -> Self {
        match bi {
            BinderInfo::Default => ArgClass::Explicit,
            BinderInfo::Implicit => ArgClass::Implicit,
            BinderInfo::InstImplicit => ArgClass::Instance,
            BinderInfo::StrictImplicit => ArgClass::Strict,
        }
    }
    /// Whether this argument class is any form of implicit.
    pub fn is_implicit(&self) -> bool {
        !matches!(self, ArgClass::Explicit)
    }
}
/// Records the positions (0-based) of implicit arguments in a Pi-type.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ImplicitPositionIndex {
    positions: Vec<(usize, bool)>,
}
impl ImplicitPositionIndex {
    /// Build an index from a Pi-type.
    #[allow(dead_code)]
    pub fn from_type(ty: &Expr) -> Self {
        let mut positions = Vec::new();
        let mut current = ty;
        let mut pos = 0;
        while let Expr::Pi(bi, _, _, cod) = current {
            let is_inst = matches!(bi, BinderInfo::InstImplicit);
            if matches!(
                bi,
                BinderInfo::Implicit | BinderInfo::InstImplicit | BinderInfo::StrictImplicit
            ) {
                positions.push((pos, is_inst));
            }
            current = cod;
            pos += 1;
        }
        Self { positions }
    }
    /// Number of implicit positions.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.positions.len()
    }
    /// Whether any implicit positions exist.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.positions.is_empty()
    }
    /// Get the position indices of all implicit arguments.
    #[allow(dead_code)]
    pub fn implicit_positions(&self) -> Vec<usize> {
        self.positions.iter().map(|(p, _)| *p).collect()
    }
    /// Get the position indices of all instance arguments.
    #[allow(dead_code)]
    pub fn instance_positions(&self) -> Vec<usize> {
        self.positions
            .iter()
            .filter(|(_, is_inst)| *is_inst)
            .map(|(p, _)| *p)
            .collect()
    }
}
/// Result of attempting to insert all implicit arguments before an explicit
/// application site.
#[derive(Clone, Debug)]
pub struct ImplicitInsertResult {
    /// The function expression with all implicits applied.
    pub expr: Expr,
    /// The remaining type (after stripping leading implicit Pi-types).
    pub remaining_ty: Expr,
    /// Records of each implicit that was inserted.
    pub inserted: Vec<InsertedImplicit>,
}
/// Statistics collected during the implicit insertion pipeline.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ImplicitStats {
    /// Total number of implicit arguments inserted.
    pub total_insertions: usize,
    /// Number of type-class arguments inserted.
    pub tc_insertions: usize,
    /// Number of strict implicit arguments inserted.
    pub strict_insertions: usize,
    /// Number of times an instance was found locally.
    pub local_instance_hits: usize,
    /// Number of times an instance was found globally.
    pub global_instance_hits: usize,
    /// Number of cache hits.
    pub cache_hits: usize,
}
impl ImplicitStats {
    /// Create zeroed statistics.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a single implicit insertion.
    #[allow(dead_code)]
    pub fn record_insertion(&mut self, mode: ImplicitMode) {
        self.total_insertions += 1;
        match mode {
            ImplicitMode::TypeClass => self.tc_insertions += 1,
            ImplicitMode::Strict => self.strict_insertions += 1,
            ImplicitMode::Unification => {}
        }
    }
    /// Whether any insertions happened.
    #[allow(dead_code)]
    pub fn any_inserted(&self) -> bool {
        self.total_insertions > 0
    }
    /// Reset all counters.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}
/// A simple memoisation cache for implicit argument resolution.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ImplicitCache {
    entries: std::collections::HashMap<String, Expr>,
}
impl ImplicitCache {
    /// Create an empty cache.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a resolved result for a type.
    #[allow(dead_code)]
    pub fn insert(&mut self, ty_key: impl Into<String>, expr: Expr) {
        self.entries.insert(ty_key.into(), expr);
    }
    /// Look up a cached resolution.
    #[allow(dead_code)]
    pub fn get(&self, ty_key: &str) -> Option<&Expr> {
        self.entries.get(ty_key)
    }
    /// Number of cached entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the cache is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear all cached entries.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// A stack of auto-implicit scopes for nested declarations.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ImplicitScopeStack {
    stack: Vec<AutoImplicitScope>,
}
impl ImplicitScopeStack {
    /// Create a new empty scope stack.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new scope onto the stack.
    #[allow(dead_code)]
    pub fn push_scope(&mut self) {
        self.stack.push(AutoImplicitScope::new());
    }
    /// Pop the current scope from the stack.
    #[allow(dead_code)]
    pub fn pop_scope(&mut self) -> Option<AutoImplicitScope> {
        self.stack.pop()
    }
    /// Bind a name in the current (top) scope.
    #[allow(dead_code)]
    pub fn bind_current(&mut self, name: impl Into<String>) {
        if let Some(top) = self.stack.last_mut() {
            top.bind(name);
        }
    }
    /// Check if a name is bound in any scope on the stack.
    #[allow(dead_code)]
    pub fn is_bound_anywhere(&self, name: &str) -> bool {
        self.stack.iter().any(|s| s.is_bound(name))
    }
    /// Get the depth of the scope stack.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    /// Whether the stack is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}
/// A summary of the implicit argument structure of a Pi-type.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ImplicitArgSummary {
    /// Number of leading implicit ({...}) arguments.
    pub leading_implicit: usize,
    /// Number of leading instance ([...]) arguments.
    pub leading_instance: usize,
    /// Number of leading strict arguments.
    pub leading_strict: usize,
    /// Number of explicit arguments.
    pub explicit: usize,
    /// Total arity.
    pub total: usize,
}
impl ImplicitArgSummary {
    /// Summarise a Pi-type.
    #[allow(dead_code)]
    pub fn of(ty: &Expr) -> Self {
        let mut summary = Self::default();
        let mut current = ty;
        let mut in_leading = true;
        while let Expr::Pi(bi, _, _, cod) = current {
            summary.total += 1;
            match bi {
                BinderInfo::Implicit => {
                    if in_leading {
                        summary.leading_implicit += 1;
                    }
                }
                BinderInfo::InstImplicit => {
                    if in_leading {
                        summary.leading_instance += 1;
                    }
                }
                BinderInfo::StrictImplicit => {
                    if in_leading {
                        summary.leading_strict += 1;
                    }
                }
                BinderInfo::Default => {
                    in_leading = false;
                    summary.explicit += 1;
                }
            }
            current = cod;
        }
        summary
    }
    /// Total number of leading implicits (any kind).
    #[allow(dead_code)]
    pub fn total_leading(&self) -> usize {
        self.leading_implicit + self.leading_instance + self.leading_strict
    }
    /// Whether there are any leading implicits.
    #[allow(dead_code)]
    pub fn has_leading(&self) -> bool {
        self.total_leading() > 0
    }
}
/// A guard that tracks whether implicit argument insertion is currently enabled.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ImplicitGuard {
    enabled: bool,
    depth: usize,
}
impl ImplicitGuard {
    /// Create a new guard with insertion enabled.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            enabled: true,
            depth: 0,
        }
    }
    /// Temporarily disable implicit insertion.
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.depth += 1;
        self.enabled = false;
    }
    /// Re-enable implicit insertion.
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
        if self.depth == 0 {
            self.enabled = true;
        }
    }
    /// Whether insertion is currently enabled.
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
/// Errors that can occur during implicit argument insertion.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImplicitError {
    /// Could not infer the implicit argument.
    CannotInfer(String),
    /// Type-class instance not found.
    InstanceNotFound(String),
    /// Maximum implicit argument count exceeded.
    TooManyImplicits(usize),
    /// Circular dependency between implicit arguments.
    CircularDependency(usize, usize),
}
/// A queue of pending implicit arguments awaiting resolution.
#[derive(Clone, Debug, Default)]
pub struct PendingImplicitQueue {
    queue: Vec<PendingImplicit>,
}
impl PendingImplicitQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self::default()
    }
    /// Enqueue a pending implicit.
    pub fn push(&mut self, p: PendingImplicit) {
        self.queue.push(p);
    }
    /// Dequeue the next pending implicit.
    pub fn pop(&mut self) -> Option<PendingImplicit> {
        self.queue.pop()
    }
    /// Peek at the next pending implicit.
    pub fn peek(&self) -> Option<&PendingImplicit> {
        self.queue.last()
    }
    /// Number of pending implicits.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    /// Whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    /// Clear all pending implicits.
    pub fn clear(&mut self) {
        self.queue.clear();
    }
    /// Return how many are user-provided vs synthesised.
    pub fn user_provided_count(&self) -> usize {
        self.queue.iter().filter(|p| p.user_provided).count()
    }
    /// Return how many require type-class synthesis.
    pub fn tc_pending_count(&self) -> usize {
        self.queue
            .iter()
            .filter(|p| p.mode == ImplicitMode::TypeClass && !p.user_provided)
            .count()
    }
}
/// The mode in which an implicit argument should be filled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImplicitMode {
    /// Fill by unification (ordinary `{…}` implicit).
    Unification,
    /// Fill by type-class synthesis (`[…]` instance implicit).
    TypeClass,
    /// Fill by strict unification (`{{…}}` strict implicit).
    Strict,
}
impl ImplicitMode {
    /// Construct from a `BinderInfo`.
    pub fn from_binder(bi: &BinderInfo) -> Option<Self> {
        match bi {
            BinderInfo::Implicit => Some(ImplicitMode::Unification),
            BinderInfo::InstImplicit => Some(ImplicitMode::TypeClass),
            BinderInfo::StrictImplicit => Some(ImplicitMode::Strict),
            _ => None,
        }
    }
    /// `true` if this mode requires unification to solve.
    pub fn needs_unification(self) -> bool {
        matches!(self, ImplicitMode::Unification | ImplicitMode::Strict)
    }
    /// `true` if this mode requires type-class synthesis.
    pub fn needs_synthesis(self) -> bool {
        self == ImplicitMode::TypeClass
    }
}
/// Record of a single auto-inserted implicit argument together with the
/// metavariable that was created for it.
#[derive(Clone, Debug)]
pub struct InsertedImplicit {
    /// Argument name.
    pub name: String,
    /// The metavariable ID.
    pub meta_id: u64,
    /// The mode used for filling.
    pub mode: ImplicitMode,
}
impl InsertedImplicit {
    /// Construct an `InsertedImplicit`.
    pub fn new(name: impl Into<String>, meta_id: u64, mode: ImplicitMode) -> Self {
        Self {
            name: name.into(),
            meta_id,
            mode,
        }
    }
}
/// Configuration for the implicit insertion pipeline.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ImplicitPipelineConfig {
    /// Maximum number of implicit arguments to insert in one pass.
    pub max_implicit_args: usize,
    /// Whether to enable type-class synthesis.
    pub enable_tc_synthesis: bool,
    /// Whether to allow partial application (stop before all implicits are filled).
    pub allow_partial: bool,
    /// Whether to insert strict implicits automatically.
    pub insert_strict: bool,
}
impl ImplicitPipelineConfig {
    /// Create a configuration that inserts everything.
    #[allow(dead_code)]
    pub fn full() -> Self {
        Self {
            max_implicit_args: 256,
            enable_tc_synthesis: true,
            allow_partial: false,
            insert_strict: true,
        }
    }
    /// Create a minimal configuration (no TC synthesis, no strict).
    #[allow(dead_code)]
    pub fn minimal() -> Self {
        Self {
            max_implicit_args: 16,
            enable_tc_synthesis: false,
            allow_partial: true,
            insert_strict: false,
        }
    }
}
