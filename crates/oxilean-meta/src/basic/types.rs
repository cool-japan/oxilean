//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Expr, FVarId, Level, Name};
use std::collections::HashMap;

use super::metacontext_type::MetaContext;

/// Unique identifier for expression metavariables.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MVarId(pub u64);
impl MVarId {
    /// Create a new metavariable ID.
    pub fn new(id: u64) -> Self {
        MVarId(id)
    }
}
/// Declaration for a metavariable, recording its type and context.
#[derive(Clone, Debug)]
pub struct MetavarDecl {
    /// The type of the metavariable.
    pub ty: Expr,
    /// The local context in which this mvar was created.
    pub lctx_snapshot: Vec<FVarId>,
    /// Kind of metavariable.
    pub kind: MetavarKind,
    /// User-facing name (for error messages).
    pub user_name: Name,
    /// Number of scope arguments (for dependency tracking).
    pub num_scope_args: u32,
    /// Depth at which this mvar was created (for scoping).
    pub depth: u32,
}
/// A sliding window accumulator for MetaBasic.
#[allow(dead_code)]
pub struct MetaBasicWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl MetaBasicWindow {
    pub fn new(capacity: usize) -> Self {
        MetaBasicWindow {
            buffer: std::collections::VecDeque::new(),
            capacity,
            running_sum: 0.0,
        }
    }
    pub fn push(&mut self, v: f64) {
        if self.buffer.len() >= self.capacity {
            if let Some(old) = self.buffer.pop_front() {
                self.running_sum -= old;
            }
        }
        self.buffer.push_back(v);
        self.running_sum += v;
    }
    pub fn mean(&self) -> f64 {
        if self.buffer.is_empty() {
            0.0
        } else {
            self.running_sum / self.buffer.len() as f64
        }
    }
    pub fn variance(&self) -> f64 {
        if self.buffer.len() < 2 {
            return 0.0;
        }
        let m = self.mean();
        self.buffer.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / self.buffer.len() as f64
    }
    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
/// A trace of unification attempts (for debugging).
#[derive(Clone, Debug, Default)]
pub struct UnificationTrace {
    pub(super) entries: Vec<UnificationEntry>,
    pub(super) enabled: bool,
}
impl UnificationTrace {
    /// Create an enabled trace.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            enabled: true,
        }
    }
    /// Create a disabled (no-op) trace.
    pub fn disabled() -> Self {
        Self {
            entries: Vec::new(),
            enabled: false,
        }
    }
    /// Record a unification attempt.
    pub fn record(&mut self, lhs: &str, rhs: &str, success: bool, depth: u32) {
        if self.enabled {
            self.entries.push(UnificationEntry {
                lhs: lhs.to_string(),
                rhs: rhs.to_string(),
                success,
                depth,
            });
        }
    }
    /// Number of recorded entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the trace is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Number of successful unifications.
    pub fn success_count(&self) -> usize {
        self.entries.iter().filter(|e| e.success).count()
    }
    /// Number of failed unifications.
    pub fn failure_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.success).count()
    }
    /// Clear all entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Enable or disable tracing.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    /// Whether tracing is enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
/// A configuration store for Basic.
#[allow(dead_code)]
pub struct BasicConfig {
    pub values: std::collections::HashMap<String, BasicConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl BasicConfig {
    pub fn new() -> Self {
        BasicConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: BasicConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&BasicConfigValue> {
        self.values.get(key)
    }
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, BasicConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, BasicConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, BasicConfigValue::Str(v.to_string()))
    }
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    pub fn size(&self) -> usize {
        self.values.len()
    }
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
/// Statistics about the metavariable context.
#[derive(Clone, Debug, Default)]
pub struct MetaStatistics {
    /// Total number of metavariables created.
    pub total_created: usize,
    /// Number of metavariables assigned.
    pub total_assigned: usize,
    /// Number of metavariables still pending.
    pub total_pending: usize,
    /// Number of local declarations created.
    pub total_locals: usize,
    /// Number of postponed constraints created.
    pub total_postponed: usize,
}
impl MetaStatistics {
    /// Collect statistics from a `MetaContext`.
    pub fn from_ctx(ctx: &MetaContext) -> Self {
        let total_created = ctx.mvar_count();
        let pending = ctx.unassigned_mvars();
        let total_assigned = total_created - pending.len();
        Self {
            total_created,
            total_assigned,
            total_pending: pending.len(),
            total_locals: ctx.num_locals(),
            total_postponed: ctx.num_postponed(),
        }
    }
    /// Whether all metavariables are resolved.
    pub fn all_resolved(&self) -> bool {
        self.total_pending == 0
    }
    /// Summary string for debugging.
    pub fn summary(&self) -> String {
        format!(
            "MetaStatistics {{ created={}, assigned={}, pending={}, locals={}, postponed={} }}",
            self.total_created,
            self.total_assigned,
            self.total_pending,
            self.total_locals,
            self.total_postponed,
        )
    }
}
/// An extended map for MetaBasic keys to values.
#[allow(dead_code)]
pub struct MetaBasicExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> MetaBasicExtMap<V> {
    pub fn new() -> Self {
        MetaBasicExtMap {
            data: std::collections::HashMap::new(),
            default_key: None,
        }
    }
    pub fn insert(&mut self, key: &str, value: V) {
        self.data.insert(key.to_string(), value);
    }
    pub fn get(&self, key: &str) -> Option<&V> {
        self.data.get(key)
    }
    pub fn get_or_default(&self, key: &str) -> V {
        self.data.get(key).cloned().unwrap_or_default()
    }
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.data.remove(key)
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn set_default(&mut self, key: &str) {
        self.default_key = Some(key.to_string());
    }
    pub fn keys_sorted(&self) -> Vec<&String> {
        let mut keys: Vec<&String> = self.data.keys().collect();
        keys.sort();
        keys
    }
}
/// An extended utility type for MetaBasic.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaBasicExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl MetaBasicExt {
    /// Creates a new default instance.
    pub fn new() -> Self {
        Self {
            tag: 0,
            description: None,
        }
    }
    /// Sets the tag.
    pub fn with_tag(mut self, tag: u32) -> Self {
        self.tag = tag;
        self
    }
    /// Sets the description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    /// Returns `true` if the description is set.
    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }
}
/// Kind of metavariable, affecting how it can be assigned.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetavarKind {
    /// Natural metavariable: can be assigned by unification.
    Natural,
    /// Synthetic: created by the system, resolved by specific tactics.
    Synthetic,
    /// Synthetic opaque: cannot be assigned by unification, only by tactics.
    SyntheticOpaque,
}
#[allow(dead_code)]
pub struct BasicExtDiff3800 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl BasicExtDiff3800 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    #[allow(dead_code)]
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    #[allow(dead_code)]
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// A diagnostic reporter for Basic.
#[allow(dead_code)]
pub struct BasicDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl BasicDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        BasicDiagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
pub struct MetaBasicExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl MetaBasicExtUtil {
    pub fn new(key: &str) -> Self {
        MetaBasicExtUtil {
            key: key.to_string(),
            data: Vec::new(),
            active: true,
            flags: 0,
        }
    }
    pub fn push(&mut self, v: i64) {
        self.data.push(v);
    }
    pub fn pop(&mut self) -> Option<i64> {
        self.data.pop()
    }
    pub fn sum(&self) -> i64 {
        self.data.iter().sum()
    }
    pub fn min_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::min)
    }
    pub fn max_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::max)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn set_flag(&mut self, bit: u32) {
        self.flags |= 1 << bit;
    }
    pub fn has_flag(&self, bit: u32) -> bool {
        self.flags & (1 << bit) != 0
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    pub fn activate(&mut self) {
        self.active = true;
    }
}
/// An extended utility type for MetaBasic.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaBasicExt2 {
    /// A numeric tag.
    pub tag: u32,
}
#[allow(dead_code)]
impl MetaBasicExt2 {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { tag: 0 }
    }
}
/// A state machine controller for MetaBasic.
#[allow(dead_code)]
pub struct MetaBasicStateMachine {
    pub state: MetaBasicState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl MetaBasicStateMachine {
    pub fn new() -> Self {
        MetaBasicStateMachine {
            state: MetaBasicState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: MetaBasicState) -> bool {
        if self.state.is_terminal() {
            return false;
        }
        let desc = format!("{:?} -> {:?}", self.state, new_state);
        self.state = new_state;
        self.transitions += 1;
        self.history.push(desc);
        true
    }
    pub fn start(&mut self) -> bool {
        self.transition_to(MetaBasicState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(MetaBasicState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(MetaBasicState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(MetaBasicState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
/// A result type for Basic analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum BasicResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl BasicResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, BasicResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, BasicResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, BasicResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, BasicResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            BasicResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            BasicResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            BasicResult::Ok(_) => 1.0,
            BasicResult::Err(_) => 0.0,
            BasicResult::Skipped => 0.0,
            BasicResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// Local variable declaration in the meta context.
#[derive(Clone, Debug)]
pub struct LocalDecl {
    /// Free variable ID.
    pub fvar_id: FVarId,
    /// User-facing name.
    pub user_name: Name,
    /// Type of the variable.
    pub ty: Expr,
    /// Binder info.
    pub binder_info: BinderInfo,
    /// Optional value (for let-bindings).
    pub value: Option<Expr>,
    /// Index in context order.
    pub index: u32,
}
/// Configuration for meta operations.
#[derive(Clone, Debug)]
pub struct MetaConfig {
    /// Whether to use first-order approximation in unification.
    pub fo_approx: bool,
    /// Whether to use constant approximation in unification.
    pub const_approx: bool,
    /// Whether to use context approximation.
    pub ctx_approx: bool,
    /// Whether to track assignments for undo.
    pub track_assignments: bool,
    /// Maximum recursion depth for unification.
    pub max_recursion_depth: u32,
    /// Whether proof irrelevance is enabled.
    pub proof_irrelevance: bool,
    /// Whether eta expansion is used for structs.
    pub eta_struct: bool,
    /// Whether to unfold reducible definitions.
    pub unfold_reducible: bool,
}
/// Saved state for backtracking.
#[derive(Clone, Debug)]
pub struct MetaState {
    /// Number of metavariables at save point.
    pub num_mvars: u64,
    /// Number of local declarations at save point.
    pub num_locals: u32,
    /// Metavar assignments at save point.
    pub mvar_assignments: HashMap<MVarId, Expr>,
    /// Level mvar assignments at save point.
    pub level_assignments: HashMap<u64, Level>,
    /// Number of postponed constraints.
    pub num_postponed: usize,
}
/// A snapshot of the local hypothesis context, independent of `MetaContext`.
#[derive(Clone, Debug, Default)]
pub struct LocalContext {
    /// List of (name, type) pairs for local hypotheses.
    pub hyps: Vec<(Name, Expr)>,
}
impl LocalContext {
    /// Create an empty local context.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a hypothesis.
    pub fn add(&mut self, name: Name, ty: Expr) {
        self.hyps.push((name, ty));
    }
    /// Number of hypotheses.
    pub fn len(&self) -> usize {
        self.hyps.len()
    }
    /// Whether context is empty.
    pub fn is_empty(&self) -> bool {
        self.hyps.is_empty()
    }
    /// Find a hypothesis by name.
    pub fn get(&self, name: &Name) -> Option<&Expr> {
        self.hyps.iter().find(|(n, _)| n == name).map(|(_, ty)| ty)
    }
    /// Whether a hypothesis with the given name exists.
    pub fn contains(&self, name: &Name) -> bool {
        self.hyps.iter().any(|(n, _)| n == name)
    }
    /// Remove a hypothesis by name. Returns `true` if found.
    pub fn remove(&mut self, name: &Name) -> bool {
        if let Some(pos) = self.hyps.iter().position(|(n, _)| n == name) {
            self.hyps.remove(pos);
            true
        } else {
            false
        }
    }
    /// Names of all hypotheses.
    pub fn names(&self) -> Vec<&Name> {
        self.hyps.iter().map(|(n, _)| n).collect()
    }
}
/// A typed slot for Basic configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BasicConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl BasicConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            BasicConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            BasicConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            BasicConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            BasicConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            BasicConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            BasicConfigValue::Bool(_) => "bool",
            BasicConfigValue::Int(_) => "int",
            BasicConfigValue::Float(_) => "float",
            BasicConfigValue::Str(_) => "str",
            BasicConfigValue::List(_) => "list",
        }
    }
}
/// A postponed unification constraint.
#[derive(Clone, Debug)]
pub struct PostponedConstraint {
    /// Left-hand side.
    pub lhs: Expr,
    /// Right-hand side.
    pub rhs: Expr,
    /// Depth at which this constraint was created.
    pub depth: u32,
}
/// A single entry in a unification trace.
#[derive(Clone, Debug)]
pub struct UnificationEntry {
    /// Left-hand side expression (stringified).
    pub lhs: String,
    /// Right-hand side expression (stringified).
    pub rhs: String,
    /// Whether the unification succeeded.
    pub success: bool,
    /// Depth at which unification was attempted.
    pub depth: u32,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BasicExtConfigVal3800 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl BasicExtConfigVal3800 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let BasicExtConfigVal3800::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let BasicExtConfigVal3800::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let BasicExtConfigVal3800::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let BasicExtConfigVal3800::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let BasicExtConfigVal3800::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            BasicExtConfigVal3800::Bool(_) => "bool",
            BasicExtConfigVal3800::Int(_) => "int",
            BasicExtConfigVal3800::Float(_) => "float",
            BasicExtConfigVal3800::Str(_) => "str",
            BasicExtConfigVal3800::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct BasicExtDiag3800 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl BasicExtDiag3800 {
    #[allow(dead_code)]
    pub fn new(max_errors: usize) -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    #[allow(dead_code)]
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    #[allow(dead_code)]
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    #[allow(dead_code)]
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    #[allow(dead_code)]
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    #[allow(dead_code)]
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
/// A pool for efficiently generating metavariable IDs.
///
/// Batches pre-allocated IDs and issues them in sequence.
#[derive(Clone, Debug)]
pub struct MetaVarPool {
    pub(super) next_id: u64,
    pub(super) batch_size: usize,
}
impl MetaVarPool {
    /// Create a new pool starting at ID `start`.
    pub fn new(start: u64) -> Self {
        Self {
            next_id: start,
            batch_size: 64,
        }
    }
    /// Create with a custom batch size.
    pub fn with_batch_size(start: u64, batch_size: usize) -> Self {
        Self {
            next_id: start,
            batch_size,
        }
    }
    /// Get the next ID.
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    /// Reserve a batch of IDs, returning the start of the batch.
    pub fn reserve_batch(&mut self) -> (u64, usize) {
        let start = self.next_id;
        self.next_id += self.batch_size as u64;
        (start, self.batch_size)
    }
    /// Current counter (number of IDs issued so far from start).
    pub fn count_issued(&self) -> u64 {
        self.next_id
    }
}
/// A diff for Basic analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BasicDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl BasicDiff {
    pub fn new() -> Self {
        BasicDiff {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// A builder pattern for MetaBasic.
#[allow(dead_code)]
pub struct MetaBasicBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl MetaBasicBuilder {
    pub fn new(name: &str) -> Self {
        MetaBasicBuilder {
            name: name.to_string(),
            items: Vec::new(),
            config: std::collections::HashMap::new(),
        }
    }
    pub fn add_item(mut self, item: &str) -> Self {
        self.items.push(item.to_string());
        self
    }
    pub fn set_config(mut self, key: &str, value: &str) -> Self {
        self.config.insert(key.to_string(), value.to_string());
        self
    }
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    pub fn has_config(&self, key: &str) -> bool {
        self.config.contains_key(key)
    }
    pub fn get_config(&self, key: &str) -> Option<&str> {
        self.config.get(key).map(|s| s.as_str())
    }
    pub fn build_summary(&self) -> String {
        format!(
            "{}: {} items, {} config keys",
            self.name,
            self.items.len(),
            self.config.len()
        )
    }
}
/// A work queue for MetaBasic items.
#[allow(dead_code)]
pub struct MetaBasicWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl MetaBasicWorkQueue {
    pub fn new(capacity: usize) -> Self {
        MetaBasicWorkQueue {
            pending: std::collections::VecDeque::new(),
            processed: Vec::new(),
            capacity,
        }
    }
    pub fn enqueue(&mut self, item: String) -> bool {
        if self.pending.len() >= self.capacity {
            return false;
        }
        self.pending.push_back(item);
        true
    }
    pub fn dequeue(&mut self) -> Option<String> {
        let item = self.pending.pop_front()?;
        self.processed.push(item.clone());
        Some(item)
    }
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    pub fn processed_count(&self) -> usize {
        self.processed.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    pub fn is_full(&self) -> bool {
        self.pending.len() >= self.capacity
    }
    pub fn total_processed(&self) -> usize {
        self.processed.len()
    }
}
/// A state machine for MetaBasic.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MetaBasicState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl MetaBasicState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, MetaBasicState::Complete | MetaBasicState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, MetaBasicState::Initial | MetaBasicState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, MetaBasicState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            MetaBasicState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// An analysis pass for Basic.
#[allow(dead_code)]
pub struct BasicAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<BasicResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl BasicAnalysisPass {
    pub fn new(name: &str) -> Self {
        BasicAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> BasicResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            BasicResult::Err("empty input".to_string())
        } else {
            BasicResult::Ok(format!("processed: {}", input))
        };
        self.results.push(result.clone());
        result
    }
    pub fn success_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_ok()).count()
    }
    pub fn error_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_err()).count()
    }
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.success_count() as f64 / self.total_runs as f64
        }
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BasicExtResult3800 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl BasicExtResult3800 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, BasicExtResult3800::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, BasicExtResult3800::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, BasicExtResult3800::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, BasicExtResult3800::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let BasicExtResult3800::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let BasicExtResult3800::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            BasicExtResult3800::Ok(_) => 1.0,
            BasicExtResult3800::Err(_) => 0.0,
            BasicExtResult3800::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            BasicExtResult3800::Skipped => 0.5,
        }
    }
}
/// A counter map for MetaBasic frequency analysis.
#[allow(dead_code)]
pub struct MetaBasicCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl MetaBasicCounterMap {
    pub fn new() -> Self {
        MetaBasicCounterMap {
            counts: std::collections::HashMap::new(),
            total: 0,
        }
    }
    pub fn increment(&mut self, key: &str) {
        *self.counts.entry(key.to_string()).or_insert(0) += 1;
        self.total += 1;
    }
    pub fn count(&self, key: &str) -> usize {
        *self.counts.get(key).unwrap_or(&0)
    }
    pub fn frequency(&self, key: &str) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.count(key) as f64 / self.total as f64
        }
    }
    pub fn most_common(&self) -> Option<(&String, usize)> {
        self.counts
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(k, &v)| (k, v))
    }
    pub fn num_unique(&self) -> usize {
        self.counts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
}
/// A pipeline of Basic analysis passes.
#[allow(dead_code)]
pub struct BasicPipeline {
    pub passes: Vec<BasicAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl BasicPipeline {
    pub fn new(name: &str) -> Self {
        BasicPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: BasicAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<BasicResult> {
        self.total_inputs_processed += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    pub fn total_success_rate(&self) -> f64 {
        if self.passes.is_empty() {
            0.0
        } else {
            let total_rate: f64 = self.passes.iter().map(|p| p.success_rate()).sum();
            total_rate / self.passes.len() as f64
        }
    }
}
#[allow(dead_code)]
pub struct BasicExtPass3800 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<BasicExtResult3800>,
}
impl BasicExtPass3800 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_runs: 0,
            successes: 0,
            errors: 0,
            enabled: true,
            results: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self, input: &str) -> BasicExtResult3800 {
        if !self.enabled {
            return BasicExtResult3800::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            BasicExtResult3800::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            BasicExtResult3800::Ok(format!(
                "processed {} chars in pass '{}'",
                input.len(),
                self.name
            ))
        };
        self.results.push(result.clone());
        result
    }
    #[allow(dead_code)]
    pub fn success_count(&self) -> usize {
        self.successes
    }
    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.errors
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.successes as f64 / self.total_runs as f64
        }
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
#[allow(dead_code)]
pub struct BasicExtPipeline3800 {
    pub name: String,
    pub passes: Vec<BasicExtPass3800>,
    pub run_count: usize,
}
impl BasicExtPipeline3800 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: BasicExtPass3800) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<BasicExtResult3800> {
        self.run_count += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    #[allow(dead_code)]
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    #[allow(dead_code)]
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    #[allow(dead_code)]
    pub fn total_success_rate(&self) -> f64 {
        let total: usize = self.passes.iter().map(|p| p.total_runs).sum();
        let ok: usize = self.passes.iter().map(|p| p.successes).sum();
        if total == 0 {
            0.0
        } else {
            ok as f64 / total as f64
        }
    }
}
#[allow(dead_code)]
pub struct BasicExtConfig3800 {
    pub(super) values: std::collections::HashMap<String, BasicExtConfigVal3800>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl BasicExtConfig3800 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: String::new(),
        }
    }
    #[allow(dead_code)]
    pub fn named(name: &str) -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: name.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self, key: &str, value: BasicExtConfigVal3800) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&BasicExtConfigVal3800> {
        self.values.get(key)
    }
    #[allow(dead_code)]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    #[allow(dead_code)]
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    #[allow(dead_code)]
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    #[allow(dead_code)]
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, BasicExtConfigVal3800::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, BasicExtConfigVal3800::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, BasicExtConfigVal3800::Str(v.to_string()))
    }
    #[allow(dead_code)]
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    #[allow(dead_code)]
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.values.len()
    }
    #[allow(dead_code)]
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
