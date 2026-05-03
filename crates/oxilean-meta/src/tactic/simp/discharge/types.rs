//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::basic::MetaContext;
use crate::tactic::simp::types::{SimpConfig, SimpTheorems};
use oxilean_kernel::Expr;

/// A discharge log accumulates `DischargeRecord`s for debugging.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct DischargeLog {
    pub(super) records: Vec<DischargeRecord>,
}
impl DischargeLog {
    /// Create an empty log.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Append a record.
    #[allow(dead_code)]
    pub fn push(&mut self, record: DischargeRecord) {
        self.records.push(record);
    }
    /// Number of records.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.records.len()
    }
    /// Whether the log is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
    /// Iterate over records.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &DischargeRecord> {
        self.records.iter()
    }
    /// Return the strategies used, in order.
    #[allow(dead_code)]
    pub fn strategies_used(&self) -> Vec<&str> {
        self.records.iter().map(|r| r.strategy.as_str()).collect()
    }
}
/// A priority queue of discharge strategies.
///
/// Tries strategies in priority order (highest priority number first).
#[derive(Clone, Debug, Default)]
pub struct PrioritizedDischarge {
    pub(super) strategies: Vec<(i32, DischargeStrategy)>,
}
impl PrioritizedDischarge {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
        }
    }
    /// Add a strategy with a given priority.
    pub fn add(&mut self, priority: i32, strategy: DischargeStrategy) {
        self.strategies.push((priority, strategy));
        self.strategies.sort_by_key(|b| std::cmp::Reverse(b.0));
    }
    /// Attempt discharge using each strategy in priority order.
    pub fn discharge(
        &self,
        goal: &Expr,
        theorems: &SimpTheorems,
        config: &SimpConfig,
        ctx: &mut MetaContext,
    ) -> Option<Expr> {
        for (_, strategy) in &self.strategies {
            if let Some(p) = discharge_side_goal(goal, strategy, theorems, config, ctx) {
                return Some(p);
            }
        }
        None
    }
    /// Number of strategies.
    pub fn len(&self) -> usize {
        self.strategies.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.strategies.is_empty()
    }
}
/// A diff for TacticSimpDischarge analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticSimpDischargeDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticSimpDischargeDiff {
    pub fn new() -> Self {
        TacticSimpDischargeDiff {
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
/// A configuration store for TacticSimpDischarge.
#[allow(dead_code)]
pub struct TacticSimpDischargeConfig {
    pub values: std::collections::HashMap<String, TacticSimpDischargeConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticSimpDischargeConfig {
    pub fn new() -> Self {
        TacticSimpDischargeConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticSimpDischargeConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticSimpDischargeConfigValue> {
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
        self.set(key, TacticSimpDischargeConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticSimpDischargeConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticSimpDischargeConfigValue::Str(v.to_string()))
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
/// Outcome of a discharge attempt.
#[derive(Debug, Clone)]
pub enum DischargeResult {
    /// Goal closed with this proof term.
    Proved(Expr),
    /// Goal could not be discharged.
    Failed,
    /// Goal was partially simplified.
    Simplified(Expr),
}
impl DischargeResult {
    /// Check if proved.
    pub fn is_proved(&self) -> bool {
        matches!(self, DischargeResult::Proved(_))
    }
    /// Extract proof term.
    pub fn proof(self) -> Option<Expr> {
        match self {
            DischargeResult::Proved(p) => Some(p),
            _ => None,
        }
    }
}
/// A discharge record: tracks which strategy closed which goal.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DischargeRecord {
    /// The goal expression that was discharged.
    pub goal: Expr,
    /// The strategy that succeeded.
    pub strategy: String,
    /// The proof term produced.
    pub proof: Expr,
}
impl DischargeRecord {
    /// Create a new record.
    #[allow(dead_code)]
    pub fn new(goal: Expr, strategy: &str, proof: Expr) -> Self {
        Self {
            goal,
            strategy: strategy.to_string(),
            proof,
        }
    }
}
/// Strategy for discharging side goals.
#[derive(Clone, Debug)]
pub enum DischargeStrategy {
    /// Search local context for a matching hypothesis.
    Assumption,
    /// Close obviously true goals.
    Trivial,
    /// Recursively apply simp.
    Simp,
    /// Use a specific proof term.
    Exact(Expr),
    /// Try Trivial, then Assumption, then Simp.
    Auto,
    /// Try a sequence of strategies in order.
    Sequence(Vec<DischargeStrategy>),
}
impl DischargeStrategy {
    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            DischargeStrategy::Assumption => "assumption",
            DischargeStrategy::Trivial => "trivial",
            DischargeStrategy::Simp => "simp",
            DischargeStrategy::Exact(_) => "exact",
            DischargeStrategy::Auto => "auto",
            DischargeStrategy::Sequence(_) => "sequence",
        }
    }
    /// Check if deterministic.
    pub fn is_deterministic(&self) -> bool {
        matches!(
            self,
            DischargeStrategy::Exact(_) | DischargeStrategy::Trivial
        )
    }
}
/// A single recorded discharge attempt.
#[derive(Debug, Clone)]
pub struct DischargeAttempt {
    /// The goal that was attempted.
    pub goal_repr: String,
    /// The strategy used.
    pub strategy_name: String,
    /// Whether the attempt succeeded.
    pub success: bool,
}
/// The context in which a discharge is attempted.
///
/// Carries the set of available local hypotheses and any simp lemmas
/// that may be used for auto-discharge.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct DischargeContext {
    /// Names of hypotheses available in the local context.
    pub local_hyps: Vec<oxilean_kernel::Name>,
    /// Maximum recursion depth for simp-based discharge.
    pub max_simp_depth: usize,
    /// Whether to use classical reasoning (e.g., `Classical.em`).
    pub allow_classical: bool,
}
impl DischargeContext {
    /// Create an empty discharge context.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            local_hyps: Vec::new(),
            max_simp_depth: 3,
            allow_classical: false,
        }
    }
    /// Add a local hypothesis name.
    #[allow(dead_code)]
    pub fn with_hyp(mut self, name: oxilean_kernel::Name) -> Self {
        self.local_hyps.push(name);
        self
    }
    /// Check if a hypothesis name is in scope.
    #[allow(dead_code)]
    pub fn has_hyp(&self, name: &oxilean_kernel::Name) -> bool {
        self.local_hyps.contains(name)
    }
    /// Set the maximum simp recursion depth.
    #[allow(dead_code)]
    pub fn with_simp_depth(mut self, depth: usize) -> Self {
        self.max_simp_depth = depth;
        self
    }
}
/// A pipeline of TacticSimpDischarge analysis passes.
#[allow(dead_code)]
pub struct TacticSimpDischargePipeline {
    pub passes: Vec<TacticSimpDischargeAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticSimpDischargePipeline {
    pub fn new(name: &str) -> Self {
        TacticSimpDischargePipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticSimpDischargeAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticSimpDischargeResult> {
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
#[derive(Debug, Clone)]
pub enum DischargeExtResult500 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl DischargeExtResult500 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, DischargeExtResult500::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, DischargeExtResult500::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, DischargeExtResult500::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, DischargeExtResult500::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let DischargeExtResult500::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let DischargeExtResult500::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            DischargeExtResult500::Ok(_) => 1.0,
            DischargeExtResult500::Err(_) => 0.0,
            DischargeExtResult500::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            DischargeExtResult500::Skipped => 0.5,
        }
    }
}
/// A memoizing discharge cache.
///
/// Caches successful discharges by goal expression to avoid re-proving the
/// same goal repeatedly during a simp run.
#[derive(Default, Debug, Clone)]
pub struct DischargeCache {
    pub(super) cache: std::collections::HashMap<String, Expr>,
    pub(super) hits: usize,
    pub(super) misses: usize,
}
impl DischargeCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    /// Look up a goal in the cache.
    pub fn get(&mut self, goal: &Expr) -> Option<&Expr> {
        let key = format!("{:?}", goal);
        if self.cache.contains_key(&key) {
            self.hits += 1;
            self.cache.get(&key)
        } else {
            self.misses += 1;
            None
        }
    }
    /// Store a proof in the cache.
    pub fn put(&mut self, goal: &Expr, proof: Expr) {
        self.cache.insert(format!("{:?}", goal), proof);
    }
    /// Clear the cache.
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
    /// Cache hit rate (0.0 if no lookups).
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    /// Number of cached entries.
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
#[allow(dead_code)]
pub struct DischargeExtPass500 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<DischargeExtResult500>,
}
impl DischargeExtPass500 {
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
    pub fn run(&mut self, input: &str) -> DischargeExtResult500 {
        if !self.enabled {
            return DischargeExtResult500::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            DischargeExtResult500::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            DischargeExtResult500::Ok(format!(
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
/// A diagnostic reporter for TacticSimpDischarge.
#[allow(dead_code)]
pub struct TacticSimpDischargeDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticSimpDischargeDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticSimpDischargeDiagnostics {
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
#[allow(dead_code)]
pub struct DischargeExtPipeline500 {
    pub name: String,
    pub passes: Vec<DischargeExtPass500>,
    pub run_count: usize,
}
impl DischargeExtPipeline500 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: DischargeExtPass500) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<DischargeExtResult500> {
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
pub struct DischargeExtDiff500 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl DischargeExtDiff500 {
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
/// A typed slot for TacticSimpDischarge configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticSimpDischargeConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticSimpDischargeConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticSimpDischargeConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticSimpDischargeConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticSimpDischargeConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticSimpDischargeConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticSimpDischargeConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticSimpDischargeConfigValue::Bool(_) => "bool",
            TacticSimpDischargeConfigValue::Int(_) => "int",
            TacticSimpDischargeConfigValue::Float(_) => "float",
            TacticSimpDischargeConfigValue::Str(_) => "str",
            TacticSimpDischargeConfigValue::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DischargeExtConfigVal500 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl DischargeExtConfigVal500 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let DischargeExtConfigVal500::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let DischargeExtConfigVal500::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let DischargeExtConfigVal500::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let DischargeExtConfigVal500::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let DischargeExtConfigVal500::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            DischargeExtConfigVal500::Bool(_) => "bool",
            DischargeExtConfigVal500::Int(_) => "int",
            DischargeExtConfigVal500::Float(_) => "float",
            DischargeExtConfigVal500::Str(_) => "str",
            DischargeExtConfigVal500::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct DischargeExtDiag500 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl DischargeExtDiag500 {
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
/// A result type for TacticSimpDischarge analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticSimpDischargeResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticSimpDischargeResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticSimpDischargeResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticSimpDischargeResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticSimpDischargeResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticSimpDischargeResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticSimpDischargeResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticSimpDischargeResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticSimpDischargeResult::Ok(_) => 1.0,
            TacticSimpDischargeResult::Err(_) => 0.0,
            TacticSimpDischargeResult::Skipped => 0.0,
            TacticSimpDischargeResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// Statistics gathered during a discharge attempt.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct DischargeRunStats {
    /// Number of assumption lookups.
    pub assumption_checks: u64,
    /// Number of trivial closures.
    pub trivial_closures: u64,
    /// Number of simp calls.
    pub simp_calls: u64,
    /// Number of successful discharges.
    pub successes: u64,
    /// Number of failed discharge attempts.
    pub failures: u64,
}
impl DischargeRunStats {
    /// Create zeroed statistics.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a successful discharge.
    #[allow(dead_code)]
    pub fn record_success(&mut self) {
        self.successes += 1;
    }
    /// Record a failed discharge.
    #[allow(dead_code)]
    pub fn record_failure(&mut self) {
        self.failures += 1;
    }
    /// Total discharge attempts.
    #[allow(dead_code)]
    pub fn total(&self) -> u64 {
        self.successes + self.failures
    }
    /// Success rate (0.0 to 1.0).
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        let t = self.total();
        if t == 0 {
            0.0
        } else {
            self.successes as f64 / t as f64
        }
    }
}
/// An analysis pass for TacticSimpDischarge.
#[allow(dead_code)]
pub struct TacticSimpDischargeAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticSimpDischargeResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticSimpDischargeAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticSimpDischargeAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticSimpDischargeResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticSimpDischargeResult::Err("empty input".to_string())
        } else {
            TacticSimpDischargeResult::Ok(format!("processed: {}", input))
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
/// Classify goals by discharge outcome.
pub struct DischargeClassification {
    /// Goals that were proved.
    pub proved: Vec<(Expr, Expr)>,
    /// Goals that failed.
    pub failed: Vec<Expr>,
}
impl DischargeClassification {
    /// Classify a list of goals.
    pub fn classify(
        goals: &[Expr],
        strategy: &DischargeStrategy,
        theorems: &SimpTheorems,
        config: &SimpConfig,
        ctx: &mut MetaContext,
    ) -> Self {
        let mut proved = Vec::new();
        let mut failed = Vec::new();
        for goal in goals {
            match discharge_side_goal(goal, strategy, theorems, config, ctx) {
                Some(p) => proved.push((goal.clone(), p)),
                None => failed.push(goal.clone()),
            }
        }
        Self { proved, failed }
    }
    /// Success rate as a percentage.
    pub fn success_rate(&self) -> f64 {
        let total = self.proved.len() + self.failed.len();
        if total == 0 {
            100.0
        } else {
            self.proved.len() as f64 / total as f64 * 100.0
        }
    }
    /// Check if all goals were proved.
    pub fn all_proved(&self) -> bool {
        self.failed.is_empty()
    }
}
/// A tracer for discharge attempts, recording each attempt and outcome.
#[derive(Default, Debug, Clone)]
pub struct DischargeTracer {
    pub(super) attempts: Vec<DischargeAttempt>,
}
impl DischargeTracer {
    /// Create an empty tracer.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a discharge attempt.
    pub fn record(&mut self, goal: &Expr, strategy: &DischargeStrategy, success: bool) {
        self.attempts.push(DischargeAttempt {
            goal_repr: format!("{:?}", goal),
            strategy_name: strategy.name().to_string(),
            success,
        });
    }
    /// Count successful attempts.
    pub fn success_count(&self) -> usize {
        self.attempts.iter().filter(|a| a.success).count()
    }
    /// Count failed attempts.
    pub fn failure_count(&self) -> usize {
        self.attempts.iter().filter(|a| !a.success).count()
    }
    /// Total attempts.
    pub fn total(&self) -> usize {
        self.attempts.len()
    }
    /// Check if all recorded attempts succeeded.
    pub fn all_succeeded(&self) -> bool {
        self.attempts.iter().all(|a| a.success)
    }
    /// Get attempts that used a specific strategy name.
    pub fn attempts_by_strategy(&self, strategy: &str) -> Vec<&DischargeAttempt> {
        self.attempts
            .iter()
            .filter(|a| a.strategy_name == strategy)
            .collect()
    }
    /// Clear the tracer.
    pub fn clear(&mut self) {
        self.attempts.clear();
    }
}
/// Statistics from a discharge run.
#[derive(Debug, Default, Clone)]
pub struct DischargeStats {
    /// Total goals attempted.
    pub attempted: usize,
    /// Goals discharged.
    pub discharged: usize,
    /// Goals failed.
    pub failed: usize,
    /// Trivial discharges.
    pub trivial_count: usize,
    /// Assumption discharges.
    pub assumption_count: usize,
    /// Simp discharges.
    pub simp_count: usize,
}
impl DischargeStats {
    /// Success rate as a percentage.
    pub fn success_rate(&self) -> f64 {
        if self.attempted == 0 {
            100.0
        } else {
            (self.discharged as f64 / self.attempted as f64) * 100.0
        }
    }
    /// Check if all discharged.
    pub fn all_discharged(&self) -> bool {
        self.attempted > 0 && self.failed == 0
    }
}
#[allow(dead_code)]
pub struct DischargeExtConfig500 {
    pub(super) values: std::collections::HashMap<String, DischargeExtConfigVal500>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl DischargeExtConfig500 {
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
    pub fn set(&mut self, key: &str, value: DischargeExtConfigVal500) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&DischargeExtConfigVal500> {
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
        self.set(key, DischargeExtConfigVal500::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, DischargeExtConfigVal500::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, DischargeExtConfigVal500::Str(v.to_string()))
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
