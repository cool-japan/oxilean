//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::MVarId;
use oxilean_kernel::{Expr, Name};

/// A local proof step generated during conv mode.
#[derive(Clone, Debug)]
pub struct ConvLocalProof {
    /// The expression before the rewrite.
    pub before: Expr,
    /// The expression after the rewrite.
    pub after: Expr,
    /// The proof term for this step.
    pub proof: Expr,
    /// The path position where this rewrite occurred.
    pub path_depth: usize,
    /// The type `α` of the equality `@Eq α before after`, if known.
    pub ty: Option<Expr>,
}
/// A typed slot for TacticConvMode configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticConvModeConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticConvModeConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticConvModeConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticConvModeConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticConvModeConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticConvModeConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticConvModeConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticConvModeConfigValue::Bool(_) => "bool",
            TacticConvModeConfigValue::Int(_) => "int",
            TacticConvModeConfigValue::Float(_) => "float",
            TacticConvModeConfigValue::Str(_) => "str",
            TacticConvModeConfigValue::List(_) => "list",
        }
    }
}
/// A single step in the navigation path through an expression.
#[derive(Clone, Debug)]
pub struct ConvPathStep {
    /// The direction taken at this step.
    pub direction: ConvDirection,
    /// The full expression before navigating.
    pub context_expr: Expr,
    /// The position index within the expression (for arg navigation).
    pub position: usize,
}
impl ConvPathStep {
    /// Create a new path step.
    pub fn new(direction: ConvDirection, context_expr: Expr, position: usize) -> Self {
        Self {
            direction,
            context_expr,
            position,
        }
    }
}
/// Records all navigation steps from the root to the current position.
#[derive(Clone, Debug)]
pub struct ConvPath {
    /// The steps from root to current focused sub-expression.
    pub(super) steps: Vec<ConvPathStep>,
    /// The original goal before entering conv mode.
    pub(super) original_goal: Expr,
    /// Which side of the equality we entered from.
    pub(super) entry_side: ConvEntrySide,
}
impl ConvPath {
    /// Create a new empty path.
    pub fn new(original_goal: Expr, entry_side: ConvEntrySide) -> Self {
        Self {
            steps: Vec::new(),
            original_goal,
            entry_side,
        }
    }
    /// Push a step onto the path.
    pub fn push(&mut self, step: ConvPathStep) {
        self.steps.push(step);
    }
    /// Pop the last step from the path.
    pub fn pop(&mut self) -> Option<ConvPathStep> {
        self.steps.pop()
    }
    /// Check if the path is empty (at the root).
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Get the depth of navigation.
    pub fn depth(&self) -> usize {
        self.steps.len()
    }
    /// Get all steps as a slice.
    pub fn steps(&self) -> &[ConvPathStep] {
        &self.steps
    }
    /// Get the original goal.
    pub fn original_goal(&self) -> &Expr {
        &self.original_goal
    }
    /// Get the entry side.
    pub fn entry_side(&self) -> &ConvEntrySide {
        &self.entry_side
    }
}
/// The state of a conv session, tracking the focused sub-expression and path.
#[derive(Clone, Debug)]
pub struct ConvState {
    /// The currently focused sub-expression.
    pub focused: Expr,
    /// The original goal expression.
    pub original_goal: Expr,
    /// The path from root to current focus.
    pub path: ConvPath,
    /// The goal metavariable for the conv session.
    pub goal_mvar: MVarId,
    /// Local proofs accumulated during the session.
    pub local_proofs: Vec<ConvLocalProof>,
    /// Whether we are inside an ext context (under a binder).
    pub in_ext: bool,
    /// Names introduced by ext.
    pub ext_names: Vec<Name>,
    /// The type of the equality/relation.
    pub eq_type: Option<Expr>,
    /// Number of rewrites performed.
    pub rewrite_count: usize,
}
impl ConvState {
    /// Create a new conv state.
    pub fn new(
        focused: Expr,
        original_goal: Expr,
        goal_mvar: MVarId,
        entry_side: ConvEntrySide,
    ) -> Self {
        Self {
            focused: focused.clone(),
            original_goal: original_goal.clone(),
            path: ConvPath::new(original_goal, entry_side),
            goal_mvar,
            local_proofs: Vec::new(),
            in_ext: false,
            ext_names: Vec::new(),
            eq_type: None,
            rewrite_count: 0,
        }
    }
    /// Get the focused expression.
    pub fn focused(&self) -> &Expr {
        &self.focused
    }
    /// Get the navigation depth.
    pub fn depth(&self) -> usize {
        self.path.depth()
    }
    /// Check if the state is at the root.
    pub fn is_at_root(&self) -> bool {
        self.path.is_empty()
    }
    /// Record a local rewrite.
    pub fn record_rewrite(&mut self, before: Expr, after: Expr, proof: Expr, ty: Option<Expr>) {
        let depth = self.path.depth();
        self.local_proofs.push(ConvLocalProof {
            before,
            after,
            proof,
            path_depth: depth,
            ty,
        });
        self.rewrite_count += 1;
    }
}
/// Statistics for a conv session.
#[derive(Clone, Debug, Default)]
pub struct ConvStats {
    /// Number of navigation steps taken.
    pub nav_steps: usize,
    /// Maximum depth reached.
    pub max_depth_reached: usize,
    /// Number of simp calls.
    pub simp_calls: usize,
    /// Number of ring calls.
    pub ring_calls: usize,
    /// Number of norm_num calls.
    pub norm_num_calls: usize,
    /// Number of ext applications.
    pub ext_applications: usize,
    /// Number of failed rewrite attempts.
    pub failed_rewrites: usize,
}
/// Configuration for simp inside conv mode.
#[derive(Clone, Debug, Default)]
pub struct ConvSimpConfig {
    /// Simp lemma names to use.
    pub lemmas: Vec<Name>,
    /// Whether to use default simp lemmas.
    pub use_defaults: bool,
    /// Maximum number of simplification steps.
    pub max_steps: usize,
}
/// A diagnostic reporter for TacticConvMode.
#[allow(dead_code)]
pub struct TacticConvModeDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticConvModeDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticConvModeDiagnostics {
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
/// Which side of the equation was used to enter conv mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConvEntrySide {
    /// Entered from the left-hand side.
    Lhs,
    /// Entered from the right-hand side.
    Rhs,
    /// Entered from the whole expression (no side distinction).
    Whole,
}
/// Configuration for conv mode.
#[derive(Clone, Debug)]
pub struct ConvConfig {
    /// Maximum navigation depth.
    pub max_depth: usize,
    /// Whether to allow `simp` inside conv.
    pub allow_simp: bool,
    /// Whether to allow `ring` inside conv.
    pub allow_ring: bool,
    /// Whether to allow `norm_num` inside conv.
    pub allow_norm_num: bool,
    /// Maximum number of rewrites per session.
    pub max_rewrites: usize,
    /// Whether to automatically close trivial goals after rewrite.
    pub auto_close: bool,
}
impl ConvConfig {
    /// Create a config that only allows rewriting (no simp/ring/norm_num).
    pub fn rewrite_only() -> Self {
        Self {
            allow_simp: false,
            allow_ring: false,
            allow_norm_num: false,
            ..Self::default()
        }
    }
    /// Create a config with a specific max depth.
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
    /// Check if a given depth is within bounds.
    pub fn is_depth_ok(&self, depth: usize) -> bool {
        depth <= self.max_depth
    }
}
/// A result type for TacticConvMode analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticConvModeResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticConvModeResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticConvModeResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticConvModeResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticConvModeResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticConvModeResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticConvModeResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticConvModeResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticConvModeResult::Ok(_) => 1.0,
            TacticConvModeResult::Err(_) => 0.0,
            TacticConvModeResult::Skipped => 0.0,
            TacticConvModeResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// The result of a conv session.
#[derive(Clone, Debug)]
pub struct ConvResult {
    /// The modified expression after all rewrites.
    pub new_expr: Expr,
    /// The proof term that justifies the transformation.
    pub proof: Expr,
    /// Number of rewrites performed.
    pub num_rewrites: usize,
    /// Whether the conv session changed the expression.
    pub changed: bool,
    /// Statistics about the session.
    pub stats: ConvStats,
}
/// A pipeline of TacticConvMode analysis passes.
#[allow(dead_code)]
pub struct TacticConvModePipeline {
    pub passes: Vec<TacticConvModeAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticConvModePipeline {
    pub fn new(name: &str) -> Self {
        TacticConvModePipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticConvModeAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticConvModeResult> {
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
/// Navigation direction inside conv mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConvDirection {
    /// Navigate to the left side of an application `f a` -> focus on `f`.
    Left,
    /// Navigate to the right side of an application `f a` -> focus on `a`.
    Right,
    /// Navigate to the nth argument of a multi-arg application.
    Arg(usize),
    /// Navigate to the function head of an application.
    Fun,
    /// Apply extensionality: under a binder, focus on the body.
    Ext,
}
/// An operation that can be performed inside conv mode.
#[derive(Clone, Debug)]
pub enum ConvOperation {
    /// Focus on the lhs of an equality.
    Lhs,
    /// Focus on the rhs of an equality.
    Rhs,
    /// Focus on the nth argument.
    Arg(usize),
    /// Apply extensionality (enter binder body).
    Ext,
    /// Navigate up one level.
    Up,
    /// Rewrite with a lemma.
    Rw(Expr),
    /// Simplify with simp.
    Simp(ConvSimpConfig),
    /// Normalize with ring.
    Ring,
    /// Normalize with norm_num.
    NormNum,
}
/// A configuration store for TacticConvMode.
#[allow(dead_code)]
pub struct TacticConvModeConfig {
    pub values: std::collections::HashMap<String, TacticConvModeConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticConvModeConfig {
    pub fn new() -> Self {
        TacticConvModeConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticConvModeConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticConvModeConfigValue> {
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
        self.set(key, TacticConvModeConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticConvModeConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticConvModeConfigValue::Str(v.to_string()))
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
/// An analysis pass for TacticConvMode.
#[allow(dead_code)]
pub struct TacticConvModeAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticConvModeResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticConvModeAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticConvModeAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticConvModeResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticConvModeResult::Err("empty input".to_string())
        } else {
            TacticConvModeResult::Ok(format!("processed: {}", input))
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
/// Specifies which sub-expression to focus on when entering conv mode.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConvTarget {
    /// Focus on the left-hand side of an equality/relation.
    Lhs,
    /// Focus on the right-hand side of an equality/relation.
    Rhs,
    /// Focus on the nth argument of an application.
    Arg(usize),
    /// Focus on the function part of an application.
    Fun,
    /// Focus on a sub-expression matching a pattern.
    Pattern(Expr),
    /// Enter a sequence of navigation directions.
    Enter(Vec<ConvDirection>),
}
/// A diff for TacticConvMode analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticConvModeDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticConvModeDiff {
    pub fn new() -> Self {
        TacticConvModeDiff {
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
