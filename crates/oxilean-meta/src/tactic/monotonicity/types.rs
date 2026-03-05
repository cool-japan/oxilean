//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::MVarId;
use oxilean_kernel::{Expr, Name};
use std::collections::{HashMap, HashSet};

use super::functions::{combine_relations, register_default_rules, MAX_MONO_RULES};

/// Configuration for the monotonicity tactic.
#[derive(Clone, Debug)]
pub struct MonoConfig {
    /// Maximum recursion depth.
    pub max_depth: usize,
    /// Whether to use default built-in rules.
    pub use_defaults: bool,
    /// Custom rules to use (in addition to or instead of defaults).
    pub custom_rules: Vec<MonoRule>,
    /// Whether to try reflexivity on remaining sub-goals.
    pub try_refl: bool,
    /// Whether to try `assumption` on remaining sub-goals.
    pub try_assumption: bool,
    /// Whether to generate trace output.
    pub trace: bool,
}
impl MonoConfig {
    /// Create a config using only custom rules (no defaults).
    pub fn custom_only(rules: Vec<MonoRule>) -> Self {
        Self {
            use_defaults: false,
            custom_rules: rules,
            ..Self::default()
        }
    }
    /// Create a config with a specific max depth.
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
}
/// A diff for TacticMonotonicity analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticMonotonicityDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticMonotonicityDiff {
    pub fn new() -> Self {
        TacticMonotonicityDiff {
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
/// The result of applying the monotonicity tactic.
#[derive(Clone, Debug)]
pub struct MonoResult {
    /// Whether the tactic made progress.
    pub success: bool,
    /// Sub-goals remaining after mono application.
    pub remaining_goals: Vec<MVarId>,
    /// Goals closed by reflexivity or assumption.
    pub closed_goals: usize,
    /// The rule that was applied.
    pub applied_rule: Option<Name>,
    /// Statistics.
    pub stats: MonoStats,
}
/// Statistics for the monotonicity tactic.
#[derive(Clone, Debug, Default)]
pub struct MonoStats {
    /// Number of rules considered.
    pub rules_considered: usize,
    /// Number of rules tried.
    pub rules_tried: usize,
    /// Number of successful rule applications.
    pub rules_applied: usize,
    /// Number of sub-goals generated.
    pub subgoals_generated: usize,
    /// Number of sub-goals closed by refl.
    pub closed_by_refl: usize,
    /// Number of sub-goals closed by assumption.
    pub closed_by_assumption: usize,
    /// Maximum recursion depth reached.
    pub max_depth_reached: usize,
    /// Total time in microseconds (approximate).
    pub time_us: u64,
}
/// An analysis pass for TacticMonotonicity.
#[allow(dead_code)]
pub struct TacticMonotonicityAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticMonotonicityResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticMonotonicityAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticMonotonicityAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticMonotonicityResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticMonotonicityResult::Err("empty input".to_string())
        } else {
            TacticMonotonicityResult::Ok(format!("processed: {}", input))
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
/// A typed slot for TacticMonotonicity configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticMonotonicityConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticMonotonicityConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticMonotonicityConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticMonotonicityConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticMonotonicityConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticMonotonicityConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticMonotonicityConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticMonotonicityConfigValue::Bool(_) => "bool",
            TacticMonotonicityConfigValue::Int(_) => "int",
            TacticMonotonicityConfigValue::Float(_) => "float",
            TacticMonotonicityConfigValue::Str(_) => "str",
            TacticMonotonicityConfigValue::List(_) => "list",
        }
    }
}
/// A single monotonicity rule: conclusion follows from premises.
///
/// For example, `add_le_add` states that `a + b ≤ c + d` if `a ≤ c` and `b ≤ d`.
#[derive(Clone, Debug)]
pub struct MonoRule {
    /// The name of this rule.
    pub name: Name,
    /// The conclusion pattern: `(function_name, relation)`.
    pub conclusion: MonoConclusion,
    /// The premises required by this rule.
    pub premises: Vec<MonoPremise>,
    /// The proof term (constant) for this rule.
    pub proof: Expr,
    /// Priority (lower = tried first).
    pub priority: u32,
    /// Number of type/instance arguments before the main arguments.
    pub num_implicit_args: usize,
    /// Tags for categorization.
    pub tags: HashSet<String>,
}
impl MonoRule {
    /// Create a new monotonicity rule.
    pub fn new(
        name: Name,
        conclusion: MonoConclusion,
        premises: Vec<MonoPremise>,
        proof: Expr,
        priority: u32,
    ) -> Self {
        Self {
            name,
            conclusion,
            premises,
            proof,
            priority,
            num_implicit_args: 0,
            tags: HashSet::new(),
        }
    }
    /// Set the number of implicit arguments.
    pub fn with_implicit_args(mut self, n: usize) -> Self {
        self.num_implicit_args = n;
        self
    }
    /// Add a tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.insert(tag.into());
        self
    }
    /// Check if this rule matches a goal with the given function and relation.
    pub fn matches(&self, function: &Name, relation: &MonoRelation) -> bool {
        &self.conclusion.function == function && self.conclusion.relation.is_compatible(relation)
    }
    /// Get the number of sub-goals this rule generates.
    pub fn num_subgoals(&self) -> usize {
        self.premises.iter().filter(|p| !p.optional).count()
    }
}
/// A premise of a monotonicity rule.
#[derive(Clone, Debug)]
pub struct MonoPremise {
    /// The relation required in this premise.
    pub relation: MonoRelation,
    /// Which argument position (0-indexed) this premise constrains on the LHS.
    pub lhs_arg_index: usize,
    /// Which argument position (0-indexed) this premise constrains on the RHS.
    pub rhs_arg_index: usize,
    /// Whether this premise is optional (can be discharged by reflexivity).
    pub optional: bool,
}
/// An indexed collection of monotonicity rules.
///
/// Rules are indexed by `(function_name, relation)` for fast lookup.
#[derive(Clone, Debug)]
pub struct MonoRuleSet {
    /// Rules indexed by function name.
    pub(super) by_function: HashMap<Name, Vec<MonoRule>>,
    /// Rules indexed by relation.
    pub(super) by_relation: HashMap<MonoRelation, Vec<usize>>,
    /// All rules in insertion order.
    pub(super) all_rules: Vec<MonoRule>,
    /// Number of rules.
    pub(super) count: usize,
}
impl MonoRuleSet {
    /// Create a new empty rule set.
    pub fn new() -> Self {
        Self {
            by_function: HashMap::new(),
            by_relation: HashMap::new(),
            all_rules: Vec::new(),
            count: 0,
        }
    }
    /// Create a rule set with the default built-in rules.
    pub fn with_defaults() -> Self {
        let mut set = Self::new();
        register_default_rules(&mut set);
        set
    }
    /// Add a rule to the set.
    pub fn add_rule(&mut self, rule: MonoRule) {
        if self.count >= MAX_MONO_RULES {
            return;
        }
        let idx = self.all_rules.len();
        let func_name = rule.conclusion.function.clone();
        let relation = rule.conclusion.relation.clone();
        self.by_function
            .entry(func_name)
            .or_default()
            .push(rule.clone());
        self.by_relation.entry(relation).or_default().push(idx);
        self.all_rules.push(rule);
        self.count += 1;
    }
    /// Query for rules matching a specific function and relation.
    ///
    /// Returns rules sorted by priority (lowest first).
    pub fn query(&self, relation: &MonoRelation, function: &Name) -> Vec<&MonoRule> {
        let mut candidates: Vec<&MonoRule> = Vec::new();
        if let Some(rules) = self.by_function.get(function) {
            for rule in rules {
                if rule.conclusion.relation.is_compatible(relation) {
                    candidates.push(rule);
                }
            }
        }
        candidates.sort_by_key(|r| r.priority);
        candidates
    }
    /// Get all rules for a given function (any relation).
    pub fn rules_for_function(&self, function: &Name) -> Vec<&MonoRule> {
        self.by_function
            .get(function)
            .map(|rules| rules.iter().collect())
            .unwrap_or_default()
    }
    /// Get all rules for a given relation (any function).
    pub fn rules_for_relation(&self, relation: &MonoRelation) -> Vec<&MonoRule> {
        self.by_relation
            .get(relation)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&i| self.all_rules.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Get the total number of rules.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Check if the rule set is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Get all rules.
    pub fn all_rules(&self) -> &[MonoRule] {
        &self.all_rules
    }
    /// Remove all rules for a specific function.
    pub fn remove_function(&mut self, function: &Name) {
        if let Some(rules) = self.by_function.remove(function) {
            self.count -= rules.len();
        }
        self.all_rules
            .retain(|r| &r.conclusion.function != function);
    }
    /// Merge another rule set into this one.
    pub fn merge(&mut self, other: &MonoRuleSet) {
        for rule in &other.all_rules {
            self.add_rule(rule.clone());
        }
    }
}
/// A configuration store for TacticMonotonicity.
#[allow(dead_code)]
pub struct TacticMonotonicityConfig {
    pub values: std::collections::HashMap<String, TacticMonotonicityConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticMonotonicityConfig {
    pub fn new() -> Self {
        TacticMonotonicityConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticMonotonicityConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticMonotonicityConfigValue> {
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
        self.set(key, TacticMonotonicityConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticMonotonicityConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticMonotonicityConfigValue::Str(v.to_string()))
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
/// A result type for TacticMonotonicity analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticMonotonicityResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticMonotonicityResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticMonotonicityResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticMonotonicityResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticMonotonicityResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticMonotonicityResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticMonotonicityResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticMonotonicityResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticMonotonicityResult::Ok(_) => 1.0,
            TacticMonotonicityResult::Err(_) => 0.0,
            TacticMonotonicityResult::Skipped => 0.0,
            TacticMonotonicityResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A sub-goal generated by a monotonicity rule.
#[derive(Clone, Debug)]
pub struct MonoSubGoal {
    /// The metavariable ID for this sub-goal.
    pub mvar_id: MVarId,
    /// The target type to prove.
    pub target: Expr,
    /// Which premise of the rule generated this sub-goal.
    pub premise_index: usize,
    /// The relation in this sub-goal.
    pub relation: MonoRelation,
}
/// State for tracking the decomposition of a monotonicity goal.
#[derive(Clone, Debug)]
pub struct MonoState {
    /// The original goal expression.
    pub original_goal: Expr,
    /// Decomposed: lhs of the relation.
    pub lhs: Expr,
    /// Decomposed: rhs of the relation.
    pub rhs: Expr,
    /// The relation.
    pub relation: MonoRelation,
    /// The head function on the lhs.
    pub lhs_function: Option<Name>,
    /// Arguments to the lhs function.
    pub lhs_args: Vec<Expr>,
    /// The head function on the rhs.
    pub rhs_function: Option<Name>,
    /// Arguments to the rhs function.
    pub rhs_args: Vec<Expr>,
    /// The type of the expressions.
    pub expr_type: Option<Expr>,
    /// Sub-goals generated so far.
    pub sub_goals: Vec<MonoSubGoal>,
    /// Depth of recursive mono applications.
    pub depth: usize,
}
/// The ordering/relation used in a monotonicity goal.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum MonoRelation {
    /// Less than or equal: `≤`
    Le,
    /// Strict less than: `<`
    Lt,
    /// Greater than or equal: `≥`
    Ge,
    /// Strict greater than: `>`
    Gt,
    /// Divisibility: `∣`
    Dvd,
    /// Subset: `⊆`
    Subset,
    /// Custom relation identified by name.
    Custom(Name),
}
impl MonoRelation {
    /// Get the name of the relation as used in Lean 4.
    pub fn lean_name(&self) -> Name {
        match self {
            MonoRelation::Le => Name::str("LE.le"),
            MonoRelation::Lt => Name::str("LT.lt"),
            MonoRelation::Ge => Name::str("GE.ge"),
            MonoRelation::Gt => Name::str("GT.gt"),
            MonoRelation::Dvd => Name::str("Dvd.dvd"),
            MonoRelation::Subset => Name::str("HasSubset.subset"),
            MonoRelation::Custom(name) => name.clone(),
        }
    }
    /// Try to parse a relation from a constant name.
    pub fn from_name(name: &Name) -> Option<MonoRelation> {
        let s = name.to_string();
        if s.contains("LE.le") || s.contains("le") {
            Some(MonoRelation::Le)
        } else if s.contains("LT.lt") || s.contains("lt") {
            Some(MonoRelation::Lt)
        } else if s.contains("GE.ge") || s.contains("ge") {
            Some(MonoRelation::Ge)
        } else if s.contains("GT.gt") || s.contains("gt") {
            Some(MonoRelation::Gt)
        } else if s.contains("Dvd.dvd") || s.contains("dvd") {
            Some(MonoRelation::Dvd)
        } else if s.contains("HasSubset.subset") || s.contains("subset") {
            Some(MonoRelation::Subset)
        } else {
            Some(MonoRelation::Custom(name.clone()))
        }
    }
    /// Check if this relation is an inequality (Le or Lt).
    pub fn is_inequality(&self) -> bool {
        matches!(
            self,
            MonoRelation::Le | MonoRelation::Lt | MonoRelation::Ge | MonoRelation::Gt
        )
    }
    /// Flip the relation direction (Le -> Ge, Lt -> Gt, etc.).
    pub fn flip(&self) -> MonoRelation {
        match self {
            MonoRelation::Le => MonoRelation::Ge,
            MonoRelation::Lt => MonoRelation::Gt,
            MonoRelation::Ge => MonoRelation::Le,
            MonoRelation::Gt => MonoRelation::Lt,
            MonoRelation::Dvd => MonoRelation::Dvd,
            MonoRelation::Subset => MonoRelation::Subset,
            MonoRelation::Custom(name) => MonoRelation::Custom(name.clone()),
        }
    }
    /// Check if this relation is compatible with another for transitivity.
    pub fn is_compatible(&self, other: &MonoRelation) -> bool {
        match (self, other) {
            (MonoRelation::Le, MonoRelation::Le) => true,
            (MonoRelation::Le, MonoRelation::Lt) => true,
            (MonoRelation::Lt, MonoRelation::Le) => true,
            (MonoRelation::Lt, MonoRelation::Lt) => true,
            (MonoRelation::Ge, MonoRelation::Ge) => true,
            (MonoRelation::Ge, MonoRelation::Gt) => true,
            (MonoRelation::Gt, MonoRelation::Ge) => true,
            (MonoRelation::Gt, MonoRelation::Gt) => true,
            (MonoRelation::Dvd, MonoRelation::Dvd) => true,
            (MonoRelation::Subset, MonoRelation::Subset) => true,
            (MonoRelation::Custom(a), MonoRelation::Custom(b)) => a == b,
            _ => false,
        }
    }
}
/// A chain of monotonicity rule applications.
///
/// When `f` is monotone and `g` is monotone, we can conclude that
/// `f ∘ g` is monotone by composing the rules.
#[derive(Clone, Debug)]
pub struct MonoChain {
    /// The rules applied in order (outermost first).
    pub rules: Vec<MonoRule>,
    /// The combined relation.
    pub relation: MonoRelation,
}
impl MonoChain {
    /// Create a new chain with a single rule.
    pub fn singleton(rule: MonoRule) -> Self {
        let relation = rule.conclusion.relation.clone();
        MonoChain {
            rules: vec![rule],
            relation,
        }
    }
    /// Extend the chain with another rule (must use compatible relations).
    pub fn extend(&self, rule: MonoRule) -> Option<MonoChain> {
        let combined = combine_relations(&self.relation, &rule.conclusion.relation)?;
        let mut new_chain = self.clone();
        new_chain.rules.push(rule);
        new_chain.relation = combined;
        Some(new_chain)
    }
    /// Length of the chain.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Check if the chain is empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
/// A pipeline of TacticMonotonicity analysis passes.
#[allow(dead_code)]
pub struct TacticMonotonicityPipeline {
    pub passes: Vec<TacticMonotonicityAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticMonotonicityPipeline {
    pub fn new(name: &str) -> Self {
        TacticMonotonicityPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticMonotonicityAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticMonotonicityResult> {
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
/// A diagnostic reporter for TacticMonotonicity.
#[allow(dead_code)]
pub struct TacticMonotonicityDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticMonotonicityDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticMonotonicityDiagnostics {
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
/// The conclusion of a monotonicity rule.
#[derive(Clone, Debug)]
pub struct MonoConclusion {
    /// The function applied to arguments (e.g., `HAdd.hAdd`).
    pub function: Name,
    /// The relation in the conclusion (e.g., Le).
    pub relation: MonoRelation,
    /// Number of main arguments to the function.
    pub num_args: usize,
}
