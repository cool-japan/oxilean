//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Expr, FVarId, Level, Literal, Name};

/// Description of an implicit argument to be inserted.
#[derive(Clone, Debug)]
pub struct ImplicitArg {
    /// Binder name.
    pub name: Name,
    /// Domain type (may contain metavariables).
    pub ty: Expr,
    /// Binder mode.
    pub info: BinderInfo,
}
impl ImplicitArg {
    /// Create a new implicit argument descriptor.
    pub fn new(name: Name, ty: Expr, info: BinderInfo) -> Self {
        Self { name, ty, info }
    }
    /// Whether this is a typeclass/instance argument.
    pub fn is_instance(&self) -> bool {
        self.info == BinderInfo::InstImplicit
    }
    /// Whether this is a strict implicit `{{...}}` argument.
    pub fn is_strict(&self) -> bool {
        self.info == BinderInfo::StrictImplicit
    }
}
/// A report about unresolved elaboration holes.
#[derive(Debug, Clone, Default)]
pub struct ElabHoleReport {
    /// Unresolved holes (name, type-string, depth).
    pub unresolved: Vec<(String, String, usize)>,
}
impl ElabHoleReport {
    /// Create an empty report.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an unresolved hole entry.
    pub fn add(&mut self, name: impl Into<String>, ty: impl Into<String>, depth: usize) {
        self.unresolved.push((name.into(), ty.into(), depth));
    }
    /// Return the number of unresolved holes.
    pub fn len(&self) -> usize {
        self.unresolved.len()
    }
    /// Return true if there are no unresolved holes.
    pub fn is_empty(&self) -> bool {
        self.unresolved.is_empty()
    }
    /// Format as a diagnostic string.
    pub fn diagnostic(&self) -> String {
        if self.is_empty() {
            return "No unresolved holes.".to_string();
        }
        let mut out = format!("{} unresolved hole(s):\n", self.len());
        for (name, ty, depth) in &self.unresolved {
            out.push_str(&format!("  ?{} : {} (depth {})\n", name, ty, depth));
        }
        out
    }
}
/// A pending coercion to be inserted.
#[derive(Clone, Debug)]
pub struct CoercionInsert {
    /// The expression being coerced.
    pub expr: Expr,
    /// Its current type.
    pub from_ty: Expr,
    /// The target type.
    pub to_ty: Expr,
    /// Kind of coercion.
    pub kind: CoercionKind,
}
impl CoercionInsert {
    /// Create a new coercion insertion record.
    pub fn new(expr: Expr, from_ty: Expr, to_ty: Expr, kind: CoercionKind) -> Self {
        Self {
            expr,
            from_ty,
            to_ty,
            kind,
        }
    }
    /// Whether this is a numeric coercion.
    pub fn is_numeric(&self) -> bool {
        matches!(self.kind, CoercionKind::NatToInt | CoercionKind::IntToFloat)
    }
}
/// Elaboration statistics collected during a single elaboration session.
#[derive(Debug, Clone, Default)]
pub struct ElabStats {
    /// Number of expressions elaborated.
    pub exprs_elaborated: usize,
    /// Number of holes allocated.
    pub holes_allocated: usize,
    /// Number of holes assigned.
    pub holes_assigned: usize,
    /// Number of implicit arguments inserted.
    pub implicits_inserted: usize,
    /// Number of coercions applied.
    pub coercions_applied: usize,
    /// Number of universe variables allocated.
    pub univs_allocated: usize,
    /// Maximum elaboration depth reached.
    pub max_depth: usize,
}
impl ElabStats {
    /// Create empty stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Return the hole fill rate.
    pub fn hole_fill_rate(&self) -> f64 {
        if self.holes_allocated == 0 {
            1.0
        } else {
            self.holes_assigned as f64 / self.holes_allocated as f64
        }
    }
    /// Return true if all allocated holes were assigned.
    pub fn all_holes_filled(&self) -> bool {
        self.holes_allocated == self.holes_assigned
    }
    /// Merge another stats record into this one.
    pub fn merge(&mut self, other: &ElabStats) {
        self.exprs_elaborated += other.exprs_elaborated;
        self.holes_allocated += other.holes_allocated;
        self.holes_assigned += other.holes_assigned;
        self.implicits_inserted += other.implicits_inserted;
        self.coercions_applied += other.coercions_applied;
        self.univs_allocated += other.univs_allocated;
        if other.max_depth > self.max_depth {
            self.max_depth = other.max_depth;
        }
    }
}
/// A lightweight elaboration context that tracks the current depth
/// and a stack of expected types for bidirectional type-checking.
#[derive(Debug, Default)]
pub struct ElabContext {
    /// Current elaboration depth.
    pub depth: usize,
    /// Stack of expected types (for checking mode).
    pub expected_ty_stack: Vec<Expr>,
    /// Stack of source positions for error reporting.
    pub position_stack: Vec<(u32, u32)>,
    /// Current elaboration mode.
    pub mode: ElabMode,
}
impl ElabContext {
    /// Create a new context in synthesis mode.
    pub fn new() -> Self {
        Self {
            mode: ElabMode::Synth,
            ..Self::default()
        }
    }
    /// Enter a new expected-type context (checking mode).
    pub fn push_expected_ty(&mut self, ty: Expr) {
        self.expected_ty_stack.push(ty);
        self.mode = ElabMode::Check;
        self.depth += 1;
    }
    /// Exit an expected-type context.
    pub fn pop_expected_ty(&mut self) -> Option<Expr> {
        let result = self.expected_ty_stack.pop();
        if self.expected_ty_stack.is_empty() {
            self.mode = ElabMode::Synth;
        }
        if self.depth > 0 {
            self.depth -= 1;
        }
        result
    }
    /// Push a source position.
    pub fn push_position(&mut self, line: u32, col: u32) {
        self.position_stack.push((line, col));
    }
    /// Pop a source position.
    pub fn pop_position(&mut self) -> Option<(u32, u32)> {
        self.position_stack.pop()
    }
    /// Return the current expected type, if in checking mode.
    pub fn current_expected_ty(&self) -> Option<&Expr> {
        self.expected_ty_stack.last()
    }
    /// Return true if we are in checking mode.
    pub fn in_check_mode(&self) -> bool {
        self.mode == ElabMode::Check
    }
}
/// The state of an elaboration hole.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoleState {
    /// The hole is unassigned.
    Unassigned,
    /// The hole has been assigned to an expression.
    Assigned,
    /// The hole was determined to be irrelevant.
    Irrelevant,
    /// The hole was rejected (type error).
    Rejected(String),
}
impl HoleState {
    /// Return true if the hole is unassigned.
    pub fn is_unassigned(&self) -> bool {
        matches!(self, HoleState::Unassigned)
    }
    /// Return true if the hole has been assigned.
    pub fn is_assigned(&self) -> bool {
        matches!(self, HoleState::Assigned)
    }
}
/// The result of elaborating a single expression.
#[derive(Clone, Debug)]
pub struct ElabResult {
    /// The elaborated kernel expression.
    pub expr: Expr,
    /// Its inferred/checked type.
    pub ty: Expr,
}
impl ElabResult {
    /// Construct a new elaboration result.
    pub fn new(expr: Expr, ty: Expr) -> Self {
        Self { expr, ty }
    }
    /// Decompose into (expr, type) pair.
    pub fn into_pair(self) -> (Expr, Expr) {
        (self.expr, self.ty)
    }
}
/// A metavariable ID counter (thread-local, for testing).
#[derive(Clone, Debug, Default)]
pub struct MetaIdGen {
    next: u64,
}
impl MetaIdGen {
    /// Create a new generator starting at 0.
    pub fn new() -> Self {
        Self::default()
    }
    /// Issue the next fresh ID.
    pub fn fresh(&mut self) -> u64 {
        let id = self.next;
        self.next += 1;
        id
    }
    /// Peek at the next ID without consuming it.
    pub fn peek(&self) -> u64 {
        self.next
    }
}
/// Kind of coercion to be applied.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoercionKind {
    /// Numeric: Nat → Int.
    NatToInt,
    /// Numeric: Int → Float.
    IntToFloat,
    /// Subtype coercion: `{x : A // P x}` → `A`.
    SubtypeToBase,
    /// Custom user-registered coercion.
    Custom,
}
/// A fresh universe variable (level).
#[derive(Debug, Clone)]
pub struct UniverseVar {
    /// Unique ID.
    pub id: u64,
    /// Display name.
    pub name: String,
}
/// The kind of an elaboration trace event.
#[derive(Debug, Clone)]
pub enum ElabTraceKind {
    /// Starting elaboration of an expression.
    Begin { expr_repr: String },
    /// Finished elaboration of an expression.
    End { ty_repr: String },
    /// A hole was allocated.
    HoleAlloc { hole_id: u64, name: String },
    /// A hole was assigned.
    HoleAssign { hole_id: u64 },
    /// A universe variable was allocated.
    UnivAlloc { univ_id: u64 },
    /// An implicit argument was inserted.
    ImplicitInsert { arg_name: String },
    /// A coercion was applied.
    CoercionApply { coerce_fn: String },
}
/// A single entry in the elaboration trace.
#[derive(Debug, Clone)]
pub struct ElabTraceEntry2 {
    /// Depth at which this event occurred.
    pub depth: usize,
    /// The kind of event.
    pub kind: ElabTraceKind,
}
/// A trace collector for elaboration.
#[derive(Debug, Default)]
pub struct ElabTrace2 {
    entries: Vec<ElabTraceEntry2>,
    enabled: bool,
    current_depth: usize,
}
impl ElabTrace2 {
    /// Create a disabled trace.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create an enabled trace.
    pub fn enabled() -> Self {
        Self {
            entries: Vec::new(),
            enabled: true,
            current_depth: 0,
        }
    }
    /// Enable the trace.
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Record an event.
    pub fn record(&mut self, kind: ElabTraceKind) {
        if !self.enabled {
            return;
        }
        self.entries.push(ElabTraceEntry2 {
            depth: self.current_depth,
            kind,
        });
    }
    /// Increase the depth.
    pub fn enter(&mut self) {
        self.current_depth += 1;
    }
    /// Decrease the depth.
    pub fn exit(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }
    /// Return the number of trace entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return true if the trace is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Return all entries.
    pub fn entries(&self) -> &[ElabTraceEntry2] {
        &self.entries
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.current_depth = 0;
    }
    /// Count entries of a given kind (by label).
    pub fn count_hole_allocs(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| matches!(e.kind, ElabTraceKind::HoleAlloc { .. }))
            .count()
    }
    /// Count implicit insertions.
    pub fn count_implicit_inserts(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| matches!(e.kind, ElabTraceKind::ImplicitInsert { .. }))
            .count()
    }
}
/// A pending implicit argument insertion.
#[derive(Debug, Clone)]
pub struct PendingImplicit {
    /// The binder info for this implicit.
    pub binder_info: BinderInfo,
    /// The expected type of the hole.
    pub expected_ty: Expr,
    /// Optional free variable ID if this is a named implicit.
    pub fvar_id: Option<FVarId>,
    /// The name of this implicit argument.
    pub name: Name,
}
impl PendingImplicit {
    /// Create a new pending implicit.
    pub fn new(name: Name, expected_ty: Expr, binder_info: BinderInfo) -> Self {
        Self {
            binder_info,
            expected_ty,
            fvar_id: None,
            name,
        }
    }
    /// Create a pending implicit with a free variable.
    pub fn with_fvar(mut self, fvar_id: FVarId) -> Self {
        self.fvar_id = Some(fvar_id);
        self
    }
    /// Return true if this is a strict implicit (double braces).
    pub fn is_strict(&self) -> bool {
        matches!(self.binder_info, BinderInfo::StrictImplicit)
    }
    /// Return true if this is an instance implicit.
    pub fn is_instance(&self) -> bool {
        matches!(self.binder_info, BinderInfo::InstImplicit)
    }
}
/// Configuration for the elaboration engine.
#[derive(Debug, Clone)]
pub struct ElabConfig {
    /// Maximum allowed elaboration depth.
    pub max_depth: usize,
    /// Whether to insert implicit arguments automatically.
    pub insert_implicits: bool,
    /// Whether to infer universe levels.
    pub infer_universes: bool,
    /// Whether to apply coercions automatically.
    pub auto_coercions: bool,
    /// Whether elaboration tracing is enabled.
    pub trace_enabled: bool,
    /// Whether to allow sorry proofs.
    pub allow_sorry: bool,
}
impl ElabConfig {
    /// Create a strict config (no sorry, no auto-coercions).
    pub fn strict() -> Self {
        ElabConfig {
            allow_sorry: false,
            auto_coercions: false,
            ..Self::default()
        }
    }
    /// Create a debug config (trace enabled).
    pub fn debug() -> Self {
        ElabConfig {
            trace_enabled: true,
            ..Self::default()
        }
    }
}
/// A registry of all elaboration holes in a single elaboration.
#[derive(Debug, Default)]
pub struct ElabHole {
    holes: Vec<HoleCtx>,
    next_id: u64,
}
impl ElabHole {
    /// Create a new hole environment.
    pub fn new() -> Self {
        Self::default()
    }
    /// Allocate a new hole.
    pub fn alloc(&mut self, name: Name, ty: Expr, depth: usize) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.holes.push(HoleCtx::new(id, name, ty, depth));
        id
    }
    /// Assign a hole by ID.
    pub fn assign(&mut self, id: u64) -> bool {
        if let Some(hole) = self.holes.iter_mut().find(|h| h.id == id) {
            hole.assign();
            true
        } else {
            false
        }
    }
    /// Return the number of unassigned holes.
    pub fn unassigned_count(&self) -> usize {
        self.holes.iter().filter(|h| h.is_pending()).count()
    }
    /// Return all unassigned holes.
    pub fn unassigned_holes(&self) -> Vec<&HoleCtx> {
        self.holes.iter().filter(|h| h.is_pending()).collect()
    }
    /// Return the total number of holes.
    pub fn len(&self) -> usize {
        self.holes.len()
    }
    /// Return true if there are no holes.
    pub fn is_empty(&self) -> bool {
        self.holes.is_empty()
    }
    /// Return true if all holes are assigned.
    pub fn all_assigned(&self) -> bool {
        self.holes
            .iter()
            .all(|h| h.state.is_assigned() || !h.state.is_unassigned())
    }
    /// Get a hole by ID.
    pub fn get(&self, id: u64) -> Option<&HoleCtx> {
        self.holes.iter().find(|h| h.id == id)
    }
    /// Get a mutable hole by ID.
    pub fn get_mut(&mut self, id: u64) -> Option<&mut HoleCtx> {
        self.holes.iter_mut().find(|h| h.id == id)
    }
}
/// A stack frame holding an expected type for bidirectional checking.
#[derive(Debug, Clone)]
pub struct ExpectedTypeFrame {
    /// The expected type at this frame.
    pub ty: Expr,
    /// The source location of the expression being checked.
    pub location: Option<(u32, u32)>,
    /// Whether this frame was pushed by implicit argument inference.
    pub from_implicit: bool,
}
impl ExpectedTypeFrame {
    /// Create a new expected type frame.
    pub fn new(ty: Expr) -> Self {
        Self {
            ty,
            location: None,
            from_implicit: false,
        }
    }
    /// Create an expected type frame from implicit inference.
    pub fn from_implicit_inference(ty: Expr) -> Self {
        Self {
            ty,
            location: None,
            from_implicit: true,
        }
    }
    /// Attach a source location.
    pub fn with_location(mut self, line: u32, col: u32) -> Self {
        self.location = Some((line, col));
        self
    }
}
/// A stack of expected type frames.
#[derive(Debug, Default)]
pub struct ExpectedTypeStack {
    frames: Vec<ExpectedTypeFrame>,
}
impl ExpectedTypeStack {
    /// Create an empty stack.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new expected type.
    pub fn push(&mut self, frame: ExpectedTypeFrame) {
        self.frames.push(frame);
    }
    /// Pop the top frame.
    pub fn pop(&mut self) -> Option<ExpectedTypeFrame> {
        self.frames.pop()
    }
    /// Peek at the top frame.
    pub fn top(&self) -> Option<&ExpectedTypeFrame> {
        self.frames.last()
    }
    /// Return the current depth.
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    /// Return true if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }
}
/// A single trace entry recording an elaboration step.
#[derive(Clone, Debug)]
pub struct ElabTraceEntry {
    /// The step label.
    pub label: String,
    /// The expression involved (as a debug string).
    pub expr_dbg: String,
    /// The mode (synth/check).
    pub mode: ElabMode,
}
impl ElabTraceEntry {
    /// Create a new trace entry.
    pub fn new(label: &str, expr: &Expr, mode: ElabMode) -> Self {
        Self {
            label: label.to_string(),
            expr_dbg: format!("{:?}", expr),
            mode,
        }
    }
}
/// A synthetic argument (implicit or instance) about to be inserted.
#[derive(Clone, Debug)]
pub struct ElabSyntheticArg {
    /// The argument binder name.
    pub name: Name,
    /// Its type expression.
    pub ty: Expr,
    /// Binder info determining insertion mode.
    pub info: BinderInfo,
    /// Source span (byte offset) for error reporting.
    pub span: Option<(usize, usize)>,
}
impl ElabSyntheticArg {
    /// Construct a synthetic arg without span info.
    pub fn new(name: Name, ty: Expr, info: BinderInfo) -> Self {
        Self {
            name,
            ty,
            info,
            span: None,
        }
    }
    /// Attach a source span.
    pub fn with_span(mut self, start: usize, end: usize) -> Self {
        self.span = Some((start, end));
        self
    }
}
/// Represents a built argument (either explicit or synthesised implicit).
#[derive(Debug, Clone)]
pub enum BuiltArg {
    /// An explicit argument supplied by the user.
    Explicit(Expr),
    /// A synthesised implicit argument (hole or instance).
    Implicit { hole_id: u64, ty: Expr },
}
impl BuiltArg {
    /// Return true if this is an explicit argument.
    pub fn is_explicit(&self) -> bool {
        matches!(self, BuiltArg::Explicit(_))
    }
    /// Return true if this is an implicit argument.
    pub fn is_implicit(&self) -> bool {
        matches!(self, BuiltArg::Implicit { .. })
    }
    /// Return the expression (for explicit) or a BVar(0) placeholder (for implicit).
    pub fn as_expr(&self) -> Expr {
        match self {
            BuiltArg::Explicit(e) => e.clone(),
            BuiltArg::Implicit { hole_id, .. } => Expr::BVar(*hole_id as u32),
        }
    }
}
/// A queue of pending implicit argument insertions.
#[derive(Debug, Default)]
pub struct ImplicitArgQueue {
    queue: Vec<PendingImplicit>,
}
impl ImplicitArgQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self::default()
    }
    /// Enqueue a pending implicit.
    pub fn push(&mut self, pending: PendingImplicit) {
        self.queue.push(pending);
    }
    /// Dequeue the next pending implicit.
    pub fn pop(&mut self) -> Option<PendingImplicit> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }
    /// Peek at the next pending implicit.
    pub fn peek(&self) -> Option<&PendingImplicit> {
        self.queue.first()
    }
    /// Return the number of pending implicits.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    /// Return true if there are no pending implicits.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    /// Drain all pending implicits.
    pub fn drain(&mut self) -> Vec<PendingImplicit> {
        std::mem::take(&mut self.queue)
    }
    /// Count pending implicits of a given kind.
    pub fn count_instances(&self) -> usize {
        self.queue.iter().filter(|p| p.is_instance()).count()
    }
    /// Count strict implicits.
    pub fn count_strict(&self) -> usize {
        self.queue.iter().filter(|p| p.is_strict()).count()
    }
}
/// An allocator for fresh universe variables.
#[derive(Debug, Default)]
pub struct UniverseVarAlloc {
    next_id: u64,
    vars: Vec<UniverseVar>,
}
impl UniverseVarAlloc {
    /// Create a new allocator.
    pub fn new() -> Self {
        Self::default()
    }
    /// Allocate a fresh universe variable.
    pub fn alloc(&mut self) -> &UniverseVar {
        let id = self.next_id;
        self.next_id += 1;
        let name = format!("u{}", id);
        self.vars.push(UniverseVar { id, name });
        self.vars.last().expect("vars is non-empty after push")
    }
    /// Return the total number of allocated variables.
    pub fn len(&self) -> usize {
        self.vars.len()
    }
    /// Return true if no variables have been allocated.
    pub fn is_empty(&self) -> bool {
        self.vars.is_empty()
    }
    /// Return all allocated variables.
    pub fn all_vars(&self) -> &[UniverseVar] {
        &self.vars
    }
    /// Look up a variable by ID.
    pub fn get(&self, id: u64) -> Option<&UniverseVar> {
        self.vars.iter().find(|v| v.id == id)
    }
}
/// A builder for creating a list of arguments (explicit + implicit).
#[derive(Debug, Default)]
pub struct SyntheticArgBuilder {
    args: Vec<BuiltArg>,
}
impl SyntheticArgBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an explicit argument.
    pub fn push_explicit(&mut self, expr: Expr) {
        self.args.push(BuiltArg::Explicit(expr));
    }
    /// Add a synthetic implicit argument (hole).
    pub fn push_implicit(&mut self, hole_id: u64, ty: Expr) {
        self.args.push(BuiltArg::Implicit { hole_id, ty });
    }
    /// Return the number of arguments.
    pub fn len(&self) -> usize {
        self.args.len()
    }
    /// Return true if no arguments have been added.
    pub fn is_empty(&self) -> bool {
        self.args.is_empty()
    }
    /// Return the count of explicit arguments.
    pub fn explicit_count(&self) -> usize {
        self.args.iter().filter(|a| a.is_explicit()).count()
    }
    /// Return the count of implicit arguments.
    pub fn implicit_count(&self) -> usize {
        self.args.iter().filter(|a| a.is_implicit()).count()
    }
    /// Build the final argument list.
    pub fn build(self) -> Vec<BuiltArg> {
        self.args
    }
}
/// A trace of elaboration steps for debugging.
#[derive(Clone, Debug, Default)]
pub struct ElabTrace {
    /// All recorded entries in order.
    pub entries: Vec<ElabTraceEntry>,
    /// Whether tracing is enabled.
    pub enabled: bool,
}
impl ElabTrace {
    /// Create a disabled trace.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create an enabled trace.
    pub fn enabled() -> Self {
        Self {
            entries: Vec::new(),
            enabled: true,
        }
    }
    /// Record a step if tracing is enabled.
    pub fn record(&mut self, label: &str, expr: &Expr, mode: ElabMode) {
        if self.enabled {
            self.entries.push(ElabTraceEntry::new(label, expr, mode));
        }
    }
    /// Clear all recorded entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Number of recorded steps.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the trace is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A context for a single elaboration hole (metavariable).
#[derive(Debug, Clone)]
pub struct HoleCtx {
    /// Unique ID for this hole.
    pub id: u64,
    /// The type of the hole.
    pub ty: Expr,
    /// The current state.
    pub state: HoleState,
    /// The name of the hole (for display purposes).
    pub name: Name,
    /// The depth at which this hole was created.
    pub depth: usize,
}
impl HoleCtx {
    /// Create a new unassigned hole.
    pub fn new(id: u64, name: Name, ty: Expr, depth: usize) -> Self {
        Self {
            id,
            ty,
            state: HoleState::Unassigned,
            name,
            depth,
        }
    }
    /// Assign the hole.
    pub fn assign(&mut self) {
        self.state = HoleState::Assigned;
    }
    /// Mark the hole as irrelevant.
    pub fn mark_irrelevant(&mut self) {
        self.state = HoleState::Irrelevant;
    }
    /// Reject the hole with an error message.
    pub fn reject(&mut self, reason: impl Into<String>) {
        self.state = HoleState::Rejected(reason.into());
    }
    /// Return true if the hole is still pending (unassigned or deferred).
    pub fn is_pending(&self) -> bool {
        self.state.is_unassigned()
    }
}
/// A summary of all elaboration activity in a session.
#[derive(Debug, Clone, Default)]
pub struct ElabSessionSummary {
    /// Stats accumulated during elaboration.
    pub stats: ElabStats,
    /// Total number of declarations elaborated.
    pub declarations: usize,
    /// Total number of type errors encountered.
    pub type_errors: usize,
    /// Total number of sorry usages.
    pub sorry_count: usize,
}
impl ElabSessionSummary {
    /// Create an empty session summary.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a type error.
    pub fn record_type_error(&mut self) {
        self.type_errors += 1;
    }
    /// Record a sorry usage.
    pub fn record_sorry(&mut self) {
        self.sorry_count += 1;
    }
    /// Record a declaration.
    pub fn record_declaration(&mut self) {
        self.declarations += 1;
    }
    /// Return true if the session had no errors.
    pub fn is_clean(&self) -> bool {
        self.type_errors == 0
    }
}
/// Bidirectional type-checking mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ElabMode {
    #[default]
    /// Synthesis (infer) mode: produce a type from the expression.
    Synth,
    /// Checking mode: verify the expression against a known type.
    Check,
}
impl ElabMode {
    /// Whether this is synthesis mode.
    pub fn is_synth(&self) -> bool {
        matches!(self, ElabMode::Synth)
    }
    /// Whether this is checking mode.
    pub fn is_check(&self) -> bool {
        matches!(self, ElabMode::Check)
    }
}
