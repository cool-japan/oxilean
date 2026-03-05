//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::MVarId;
use crate::discr_tree::DiscrTree;
use oxilean_kernel::{Expr, Level, Name};
use std::collections::{BTreeMap, HashMap};

use super::functions::{
    mk_eq_expr, mk_eq_pattern, register_ext_lemma, BUILTIN_EXT_PRIORITY, DEFAULT_EXT_PRIORITY,
};

/// Information about structural extensionality for a specific type.
#[derive(Clone, Debug)]
pub(super) struct StructExtInfo {
    /// Fully-qualified name of the structure type.
    pub(super) struct_name: Name,
    /// Field names in declaration order.
    pub(super) field_names: Vec<Name>,
    /// Field types (may reference earlier fields via BVar).
    pub(super) field_types: Vec<Expr>,
    /// Number of type parameters.
    pub(super) num_params: u32,
}
impl StructExtInfo {
    pub(super) fn new(
        struct_name: Name,
        field_names: Vec<Name>,
        field_types: Vec<Expr>,
        num_params: u32,
    ) -> Self {
        Self {
            struct_name,
            field_names,
            field_types,
            num_params,
        }
    }
    pub(super) fn num_fields(&self) -> usize {
        self.field_names.len()
    }
    /// Build equality goals for each field: `a.field_i = b.field_i`.
    pub(super) fn field_equalities(&self, lhs: &Expr, rhs: &Expr) -> Vec<Expr> {
        let mut eqs = Vec::new();
        for i in 0..self.field_names.len() {
            let lhs_proj = Expr::Proj(self.struct_name.clone(), i as u32, Box::new(lhs.clone()));
            let rhs_proj = Expr::Proj(self.struct_name.clone(), i as u32, Box::new(rhs.clone()));
            let field_ty = self
                .field_types
                .get(i)
                .cloned()
                .unwrap_or_else(|| Expr::Sort(Level::zero()));
            let eq = mk_eq_expr(field_ty, lhs_proj, rhs_proj);
            eqs.push(eq);
        }
        eqs
    }
}
/// A name generator that draws from a user-supplied list and then falls back
/// to auto-generated names.
#[derive(Clone, Debug)]
pub(super) struct NameGen {
    pub(super) user_names: Vec<Name>,
    pub(super) cursor: usize,
    pub(super) auto_counter: usize,
    pub(super) prefix: String,
}
impl NameGen {
    pub(super) fn new(user_names: Vec<Name>, prefix: &str) -> Self {
        Self {
            user_names,
            cursor: 0,
            auto_counter: 0,
            prefix: prefix.to_string(),
        }
    }
    pub(super) fn next(&mut self) -> Name {
        if self.cursor < self.user_names.len() {
            let name = self.user_names[self.cursor].clone();
            self.cursor += 1;
            name
        } else {
            let name = Name::str(format!("{}{}", self.prefix, self.auto_counter));
            self.auto_counter += 1;
            name
        }
    }
    pub(super) fn remaining_user_names(&self) -> usize {
        self.user_names.len().saturating_sub(self.cursor)
    }
}
/// A configuration store for TacticExt.
#[allow(dead_code)]
pub struct TacticExtConfig {
    pub values: std::collections::HashMap<String, TacticExtConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticExtConfig {
    pub fn new() -> Self {
        TacticExtConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticExtConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticExtConfigValue> {
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
        self.set(key, TacticExtConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticExtConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticExtConfigValue::Str(v.to_string()))
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
/// Configuration for the `ext` tactic.
#[derive(Clone, Debug)]
pub struct ExtConfig {
    /// Maximum recursion depth (0 = no recursion, 1 = single step).
    pub max_depth: usize,
    /// Whether to use the built-in default lemma set.
    pub use_default_lemmas: bool,
    /// Additional custom lemmas to try (on top of the registry).
    pub extra_lemmas: Vec<ExtLemma>,
    /// User-supplied names for introduced variables.
    pub with_names: Vec<Name>,
}
impl ExtConfig {
    /// Create a config that only tries function extensionality.
    pub fn funext_only() -> Self {
        Self {
            max_depth: 1,
            use_default_lemmas: false,
            extra_lemmas: vec![ExtLemma::builtin(Name::str("funext"), 1, "Pi")],
            with_names: Vec::new(),
        }
    }
    /// Create a config that only tries propositional extensionality.
    pub fn propext_only() -> Self {
        Self {
            max_depth: 1,
            use_default_lemmas: false,
            extra_lemmas: vec![ExtLemma::new(
                Name::str("propext"),
                Expr::Const(Name::str("propext"), vec![]),
                mk_eq_pattern("Prop"),
                BUILTIN_EXT_PRIORITY,
                1,
                Some(Name::str("Prop")),
            )],
            with_names: Vec::new(),
        }
    }
    /// Create a single-step (non-recursive) configuration.
    pub fn single_step() -> Self {
        Self {
            max_depth: 1,
            ..Self::default()
        }
    }
    /// Create a configuration with the given variable names.
    pub fn with_names(names: Vec<Name>) -> Self {
        Self {
            with_names: names,
            ..Self::default()
        }
    }
    /// Builder: set max depth.
    pub fn set_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
    /// Builder: set whether to use default lemmas.
    pub fn set_use_defaults(mut self, v: bool) -> Self {
        self.use_default_lemmas = v;
        self
    }
    /// Builder: add extra lemmas.
    pub fn add_extra_lemmas(mut self, lemmas: Vec<ExtLemma>) -> Self {
        self.extra_lemmas.extend(lemmas);
        self
    }
    /// Consume the next user-supplied name, if any.
    pub(super) fn next_name(&mut self) -> Option<Name> {
        if self.with_names.is_empty() {
            None
        } else {
            Some(self.with_names.remove(0))
        }
    }
    /// Return how many user-supplied names remain.
    pub fn remaining_names(&self) -> usize {
        self.with_names.len()
    }
}
/// A diff for TacticExt analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticExtDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticExtDiff {
    pub fn new() -> Self {
        TacticExtDiff {
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
/// A result type for TacticExt analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticExtResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticExtResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticExtResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticExtResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticExtResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticExtResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticExtResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticExtResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticExtResult::Ok(_) => 1.0,
            TacticExtResult::Err(_) => 0.0,
            TacticExtResult::Skipped => 0.0,
            TacticExtResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A single extensionality lemma registered via `@[ext]`.
///
/// Represents a theorem like `funext : (∀ x, f x = g x) → f = g` that can
/// decompose an equality goal into simpler subgoals.
#[derive(Clone, Debug)]
pub struct ExtLemma {
    /// Fully qualified name of the lemma declaration.
    pub name: Name,
    /// The lemma as a constant expression (possibly with universe parameters).
    pub lemma: Expr,
    /// The type of the lemma (its full Pi-telescope statement).
    pub lemma_type: Expr,
    /// Priority (lower = tried first).
    pub priority: u32,
    /// Number of explicit parameters the lemma expects.
    pub num_params: usize,
    /// The head constant of the target type, if known.
    /// For example, for `funext` targeting `@Eq (α → β) f g`, this is `Some("Pi")`.
    pub target_head: Option<Name>,
}
impl ExtLemma {
    /// Create a new extensionality lemma descriptor.
    pub fn new(
        name: Name,
        lemma: Expr,
        lemma_type: Expr,
        priority: u32,
        num_params: usize,
        target_head: Option<Name>,
    ) -> Self {
        Self {
            name,
            lemma,
            lemma_type,
            priority,
            num_params,
            target_head,
        }
    }
    /// Check whether this lemma has the default priority.
    pub fn is_default_priority(&self) -> bool {
        self.priority == DEFAULT_EXT_PRIORITY
    }
    /// Check whether this lemma has builtin priority.
    pub fn is_builtin_priority(&self) -> bool {
        self.priority <= BUILTIN_EXT_PRIORITY
    }
    /// Return the lemma as a constant expression (no universe params).
    pub fn to_const_expr(&self) -> Expr {
        Expr::Const(self.name.clone(), vec![])
    }
    /// Return the lemma as a constant expression with the given universe levels.
    pub fn to_const_expr_with_levels(&self, levels: Vec<Level>) -> Expr {
        Expr::Const(self.name.clone(), levels)
    }
    /// Check whether this lemma targets a specific head constant.
    pub fn targets_head(&self, head: &Name) -> bool {
        self.target_head.as_ref() == Some(head)
    }
    /// Create a simple ext lemma from a name and priority with placeholder type.
    pub fn simple(name: Name, num_params: usize, priority: u32) -> Self {
        let lemma_expr = Expr::Const(name.clone(), vec![]);
        let lemma_type = Expr::Sort(Level::zero());
        Self::new(name, lemma_expr, lemma_type, priority, num_params, None)
    }
    /// Create a builtin ext lemma targeting a specific type head.
    pub fn builtin(name: Name, num_params: usize, target_head: &str) -> Self {
        let lemma_expr = Expr::Const(name.clone(), vec![]);
        let lemma_type = Expr::Sort(Level::zero());
        Self::new(
            name,
            lemma_expr,
            lemma_type,
            BUILTIN_EXT_PRIORITY,
            num_params,
            Some(Name::str(target_head)),
        )
    }
}
/// A pipeline of TacticExt analysis passes.
#[allow(dead_code)]
pub struct TacticExtPipeline {
    pub passes: Vec<TacticExtAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticExtPipeline {
    pub fn new(name: &str) -> Self {
        TacticExtPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticExtAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticExtResult> {
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
/// Summary statistics for an `ExtLemmaRegistry`.
#[derive(Clone, Debug)]
pub struct RegistrySummary {
    /// Total number of registered lemmas.
    pub total_lemmas: usize,
    /// Number of distinct head constants.
    pub num_heads: usize,
    /// Distribution of lemmas by priority.
    pub by_priority: BTreeMap<u32, usize>,
}
/// Information extracted from an equality goal.
#[derive(Clone, Debug)]
pub(super) struct EqGoalInfo {
    /// The type `α` in `@Eq α lhs rhs`.
    pub(super) eq_type: Expr,
    /// The left-hand side.
    pub(super) lhs: Expr,
    /// The right-hand side.
    pub(super) rhs: Expr,
}
/// Registry of extensionality lemmas, indexed by a discrimination tree for fast lookup.
///
/// The registry stores lemmas in three structures for different access patterns:
/// - `lemmas`: DiscrTree for pattern-based lookup against goal types
/// - `by_name`: HashMap for direct name-based access
/// - `by_head`: HashMap grouping lemmas by their target type's head constant
#[derive(Clone, Debug)]
pub struct ExtLemmaRegistry {
    /// Lemmas indexed by their target equality pattern (DiscrTree).
    pub lemmas: DiscrTree<ExtLemma>,
    /// Lemmas indexed by their declaration name.
    pub by_name: HashMap<Name, ExtLemma>,
    /// Lemmas grouped by the head constant of the target type.
    pub by_head: HashMap<Name, Vec<ExtLemma>>,
}
impl ExtLemmaRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            lemmas: DiscrTree::new(),
            by_name: HashMap::new(),
            by_head: HashMap::new(),
        }
    }
    /// Return the number of registered lemmas.
    pub fn num_lemmas(&self) -> usize {
        self.by_name.len()
    }
    /// Check whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.by_name.is_empty()
    }
    /// Query the registry for lemmas whose pattern matches `expr`.
    /// Results are returned sorted by priority (lowest first).
    pub fn query(&self, expr: &Expr) -> Vec<&ExtLemma> {
        let mut candidates: Vec<&ExtLemma> = self.lemmas.find(expr);
        candidates.sort_by_key(|l| l.priority);
        candidates
    }
    /// Query the registry for lemmas matching a given head constant name.
    pub fn query_by_head(&self, head: &Name) -> Vec<&ExtLemma> {
        match self.by_head.get(head) {
            Some(lemmas) => {
                let mut sorted: Vec<&ExtLemma> = lemmas.iter().collect();
                sorted.sort_by_key(|l| l.priority);
                sorted
            }
            None => Vec::new(),
        }
    }
    /// Get a lemma by name.
    pub fn get(&self, name: &Name) -> Option<&ExtLemma> {
        self.by_name.get(name)
    }
    /// Check whether a lemma with the given name is registered.
    pub fn contains(&self, name: &Name) -> bool {
        self.by_name.contains_key(name)
    }
    /// Remove a lemma by name.
    pub fn remove(&mut self, name: &Name) {
        self.by_name.remove(name);
        for group in self.by_head.values_mut() {
            group.retain(|l| &l.name != name);
        }
    }
    /// Return all registered lemma names.
    pub fn all_names(&self) -> Vec<&Name> {
        self.by_name.keys().collect()
    }
    /// Return all registered lemmas.
    pub fn all_lemmas(&self) -> Vec<&ExtLemma> {
        self.by_name.values().collect()
    }
    /// Return all head constants that have registered lemmas.
    pub fn all_heads(&self) -> Vec<&Name> {
        self.by_head.keys().collect()
    }
    /// Clear the registry completely.
    pub fn clear(&mut self) {
        self.lemmas.clear();
        self.by_name.clear();
        self.by_head.clear();
    }
    /// Merge another registry into this one.
    pub fn merge(&mut self, other: &ExtLemmaRegistry) {
        for lemma in other.by_name.values() {
            register_ext_lemma(self, lemma.clone());
        }
    }
    /// Get the highest-priority lemma for a given head, if any.
    pub fn best_for_head(&self, head: &Name) -> Option<&ExtLemma> {
        self.query_by_head(head).into_iter().next()
    }
    /// Return a summary of the registry contents for diagnostics.
    pub fn summary(&self) -> RegistrySummary {
        let total = self.by_name.len();
        let num_heads = self.by_head.len();
        let mut by_priority: BTreeMap<u32, usize> = BTreeMap::new();
        for lemma in self.by_name.values() {
            *by_priority.entry(lemma.priority).or_insert(0) += 1;
        }
        RegistrySummary {
            total_lemmas: total,
            num_heads,
            by_priority,
        }
    }
}
/// Result of applying the `ext` tactic.
#[derive(Clone, Debug)]
pub struct ExtResult {
    /// The newly created goal IDs.
    pub new_goals: Vec<MVarId>,
    /// Names of the extensionality lemmas that were applied (in order).
    pub lemmas_applied: Vec<Name>,
    /// The recursion depth actually reached.
    pub depth_reached: usize,
}
impl ExtResult {
    /// Create a new result indicating no progress.
    pub fn no_progress() -> Self {
        Self {
            new_goals: Vec::new(),
            lemmas_applied: Vec::new(),
            depth_reached: 0,
        }
    }
    /// Did ext make progress?
    pub fn made_progress(&self) -> bool {
        !self.lemmas_applied.is_empty()
    }
    /// Number of new goals created.
    pub fn num_new_goals(&self) -> usize {
        self.new_goals.len()
    }
    /// Number of lemmas applied.
    pub fn num_lemmas_applied(&self) -> usize {
        self.lemmas_applied.len()
    }
}
/// An analysis pass for TacticExt.
#[allow(dead_code)]
pub struct TacticExtAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticExtResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticExtAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticExtAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticExtResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticExtResult::Err("empty input".to_string())
        } else {
            TacticExtResult::Ok(format!("processed: {}", input))
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
/// A typed slot for TacticExt configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticExtConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticExtConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticExtConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticExtConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticExtConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticExtConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticExtConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticExtConfigValue::Bool(_) => "bool",
            TacticExtConfigValue::Int(_) => "int",
            TacticExtConfigValue::Float(_) => "float",
            TacticExtConfigValue::Str(_) => "str",
            TacticExtConfigValue::List(_) => "list",
        }
    }
}
/// A diagnostic reporter for TacticExt.
#[allow(dead_code)]
pub struct TacticExtDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticExtDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticExtDiagnostics {
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
/// Internal classification of the equality type.
#[derive(Clone, Debug)]
pub(super) enum EqTypeClass {
    /// The equality type is a function type (Pi).
    Function,
    /// The equality type is Prop.
    Prop,
    /// The equality type is a set type.
    Set,
    /// The equality type is a known structure.
    Struct(Name),
    /// Unknown type.
    Unknown,
}
