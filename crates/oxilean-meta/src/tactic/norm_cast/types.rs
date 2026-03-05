//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Name};
use std::collections::HashMap;

use super::functions::{register_default_cast_lemmas, MAX_CAST_LEMMAS};

/// An indexed collection of cast lemmas.
///
/// Lemmas are indexed by `(from_type, to_type)` pairs for fast lookup.
#[derive(Clone, Debug)]
pub struct CastLemmaSet {
    /// Lemmas indexed by (from_type, to_type).
    pub(super) by_type_pair: HashMap<(Name, Name), Vec<CastLemma>>,
    /// Lemmas indexed by direction.
    pub(super) by_direction: HashMap<CastDirection, Vec<usize>>,
    /// Lemmas indexed by operation.
    pub(super) by_operation: HashMap<Name, Vec<usize>>,
    /// All lemmas in insertion order.
    pub(super) all_lemmas: Vec<CastLemma>,
    /// Number of lemmas.
    pub(super) count: usize,
    /// Known type coercion paths: (from, to) -> intermediate types.
    pub(super) coercion_graph: HashMap<(Name, Name), Vec<Name>>,
}
impl CastLemmaSet {
    /// Create a new empty lemma set.
    pub fn new() -> Self {
        Self {
            by_type_pair: HashMap::new(),
            by_direction: HashMap::new(),
            by_operation: HashMap::new(),
            all_lemmas: Vec::new(),
            count: 0,
            coercion_graph: HashMap::new(),
        }
    }
    /// Create a lemma set with default built-in lemmas.
    pub fn with_defaults() -> Self {
        let mut set = Self::new();
        register_default_cast_lemmas(&mut set);
        set
    }
    /// Add a lemma to the set.
    pub fn add_lemma(&mut self, lemma: CastLemma) {
        if self.count >= MAX_CAST_LEMMAS {
            return;
        }
        let idx = self.all_lemmas.len();
        let type_pair = lemma.type_pair();
        let direction = lemma.direction.clone();
        self.by_type_pair
            .entry(type_pair)
            .or_default()
            .push(lemma.clone());
        self.by_direction.entry(direction).or_default().push(idx);
        if let Some(ref op) = lemma.operation {
            self.by_operation.entry(op.clone()).or_default().push(idx);
        }
        self.all_lemmas.push(lemma);
        self.count += 1;
    }
    /// Add a coercion path to the graph.
    pub fn add_coercion_path(&mut self, from: Name, to: Name, intermediates: Vec<Name>) {
        self.coercion_graph.insert((from, to), intermediates);
    }
    /// Query for lemmas by type pair and optional operation.
    pub fn query(&self, from: &Name, to: &Name, operation: Option<&Name>) -> Vec<&CastLemma> {
        let key = (from.clone(), to.clone());
        let mut candidates: Vec<&CastLemma> = Vec::new();
        if let Some(lemmas) = self.by_type_pair.get(&key) {
            for lemma in lemmas {
                if let Some(op) = operation {
                    if lemma.operation.as_ref() == Some(op) || lemma.operation.is_none() {
                        candidates.push(lemma);
                    }
                } else {
                    candidates.push(lemma);
                }
            }
        }
        candidates.sort_by_key(|l| l.priority);
        candidates
    }
    /// Query for lemmas in a specific direction.
    pub fn query_by_direction(&self, direction: &CastDirection) -> Vec<&CastLemma> {
        self.by_direction
            .get(direction)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&i| self.all_lemmas.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Query for lemmas involving a specific operation.
    pub fn query_by_operation(&self, operation: &Name) -> Vec<&CastLemma> {
        self.by_operation
            .get(operation)
            .map(|indices| {
                indices
                    .iter()
                    .filter_map(|&i| self.all_lemmas.get(i))
                    .collect()
            })
            .unwrap_or_default()
    }
    /// Get the total number of lemmas.
    pub fn len(&self) -> usize {
        self.count
    }
    /// Check if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    /// Get all lemmas.
    pub fn all_lemmas(&self) -> &[CastLemma] {
        &self.all_lemmas
    }
    /// Get the coercion graph.
    pub fn coercion_graph(&self) -> &HashMap<(Name, Name), Vec<Name>> {
        &self.coercion_graph
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NormCastExtConfigVal1400 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl NormCastExtConfigVal1400 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let NormCastExtConfigVal1400::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let NormCastExtConfigVal1400::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let NormCastExtConfigVal1400::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let NormCastExtConfigVal1400::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let NormCastExtConfigVal1400::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            NormCastExtConfigVal1400::Bool(_) => "bool",
            NormCastExtConfigVal1400::Int(_) => "int",
            NormCastExtConfigVal1400::Float(_) => "float",
            NormCastExtConfigVal1400::Str(_) => "str",
            NormCastExtConfigVal1400::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct NormCastExtConfig1400 {
    pub(super) values: std::collections::HashMap<String, NormCastExtConfigVal1400>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl NormCastExtConfig1400 {
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
    pub fn set(&mut self, key: &str, value: NormCastExtConfigVal1400) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&NormCastExtConfigVal1400> {
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
        self.set(key, NormCastExtConfigVal1400::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, NormCastExtConfigVal1400::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, NormCastExtConfigVal1400::Str(v.to_string()))
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
/// A typed slot for TacticNormCast configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticNormCastConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticNormCastConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticNormCastConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticNormCastConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticNormCastConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticNormCastConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticNormCastConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticNormCastConfigValue::Bool(_) => "bool",
            TacticNormCastConfigValue::Int(_) => "int",
            TacticNormCastConfigValue::Float(_) => "float",
            TacticNormCastConfigValue::Str(_) => "str",
            TacticNormCastConfigValue::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum NormCastExtResult1400 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl NormCastExtResult1400 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, NormCastExtResult1400::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, NormCastExtResult1400::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, NormCastExtResult1400::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, NormCastExtResult1400::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let NormCastExtResult1400::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let NormCastExtResult1400::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            NormCastExtResult1400::Ok(_) => 1.0,
            NormCastExtResult1400::Err(_) => 0.0,
            NormCastExtResult1400::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            NormCastExtResult1400::Skipped => 0.5,
        }
    }
}
/// A single cast normalization lemma.
///
/// Describes how a cast interacts with a specific operation.
/// For example: `Nat.cast_add : ↑(a + b) = ↑a + ↑b` for `Nat → Int`.
#[derive(Clone, Debug)]
pub struct CastLemma {
    /// Name of the lemma.
    pub name: Name,
    /// Source type of the cast (e.g., `Nat`).
    pub from_type: Name,
    /// Target type of the cast (e.g., `Int`).
    pub to_type: Name,
    /// The operation this lemma applies to (e.g., `HAdd.hAdd`).
    pub operation: Option<Name>,
    /// The direction this lemma normalizes in.
    pub direction: CastDirection,
    /// The lemma proof term.
    pub lemma_expr: Expr,
    /// Priority (lower = tried first).
    pub priority: u32,
    /// Number of implicit arguments the lemma expects.
    pub num_implicit_args: usize,
    /// Whether this is a built-in lemma.
    pub is_builtin: bool,
}
impl CastLemma {
    /// Create a new cast lemma.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Name,
        from_type: Name,
        to_type: Name,
        operation: Option<Name>,
        direction: CastDirection,
        lemma_expr: Expr,
        priority: u32,
    ) -> Self {
        Self {
            name,
            from_type,
            to_type,
            operation,
            direction,
            lemma_expr,
            priority,
            num_implicit_args: 0,
            is_builtin: false,
        }
    }
    /// Mark this lemma as built-in.
    pub fn builtin(mut self) -> Self {
        self.is_builtin = true;
        self
    }
    /// Set the number of implicit arguments.
    pub fn with_implicit_args(mut self, n: usize) -> Self {
        self.num_implicit_args = n;
        self
    }
    /// Check if this lemma applies to a given (from, to, operation) triple.
    pub fn applies_to(&self, from: &Name, to: &Name, op: Option<&Name>) -> bool {
        &self.from_type == from
            && &self.to_type == to
            && match (&self.operation, op) {
                (Some(lemma_op), Some(expr_op)) => lemma_op == expr_op,
                (None, _) => true,
                (Some(_), None) => false,
            }
    }
    /// Get the type pair as a tuple.
    pub fn type_pair(&self) -> (Name, Name) {
        (self.from_type.clone(), self.to_type.clone())
    }
}
/// A single step in a cast chain.
#[derive(Clone, Debug)]
pub struct CastStep {
    /// Source type.
    pub from: Name,
    /// Target type.
    pub to: Name,
    /// The coercion function used.
    pub coercion: Expr,
    /// The proof that this coercion is valid.
    pub proof: Expr,
}
/// A configuration store for TacticNormCast.
#[allow(dead_code)]
pub struct TacticNormCastConfig {
    pub values: std::collections::HashMap<String, TacticNormCastConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticNormCastConfig {
    pub fn new() -> Self {
        TacticNormCastConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticNormCastConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticNormCastConfigValue> {
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
        self.set(key, TacticNormCastConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticNormCastConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticNormCastConfigValue::Str(v.to_string()))
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
#[allow(dead_code)]
pub struct NormCastExtPipeline1400 {
    pub name: String,
    pub passes: Vec<NormCastExtPass1400>,
    pub run_count: usize,
}
impl NormCastExtPipeline1400 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: NormCastExtPass1400) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<NormCastExtResult1400> {
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
/// A result type for TacticNormCast analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticNormCastResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticNormCastResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticNormCastResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticNormCastResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticNormCastResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticNormCastResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticNormCastResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticNormCastResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticNormCastResult::Ok(_) => 1.0,
            TacticNormCastResult::Err(_) => 0.0,
            TacticNormCastResult::Skipped => 0.0,
            TacticNormCastResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// An analysis pass for TacticNormCast.
#[allow(dead_code)]
pub struct TacticNormCastAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticNormCastResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticNormCastAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticNormCastAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticNormCastResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticNormCastResult::Err("empty input".to_string())
        } else {
            TacticNormCastResult::Ok(format!("processed: {}", input))
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
/// Configuration for cast normalization tactics.
#[derive(Clone, Debug)]
pub struct CastConfig {
    /// Maximum number of normalization steps.
    pub max_steps: usize,
    /// Whether to use default lemmas.
    pub use_defaults: bool,
    /// Extra lemmas to use.
    pub extra_lemmas: Vec<CastLemma>,
    /// Whether to simplify after normalization.
    pub simp_after: bool,
    /// Whether to trace normalization steps.
    pub trace: bool,
    /// Maximum depth for cast chain search.
    pub max_chain_depth: usize,
}
impl CastConfig {
    /// Create a minimal config (no defaults, no simp).
    pub fn minimal() -> Self {
        Self {
            use_defaults: false,
            simp_after: false,
            ..Self::default()
        }
    }
    /// Create a config with extra lemmas.
    pub fn with_extra_lemmas(mut self, lemmas: Vec<CastLemma>) -> Self {
        self.extra_lemmas = lemmas;
        self
    }
}
/// A diff for TacticNormCast analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticNormCastDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticNormCastDiff {
    pub fn new() -> Self {
        TacticNormCastDiff {
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
/// The result of cast normalization.
#[derive(Clone, Debug)]
pub struct CastResult {
    /// Whether the tactic made progress.
    pub success: bool,
    /// The normalized expression.
    pub normalized: Expr,
    /// The proof of equivalence between original and normalized.
    pub proof: Expr,
    /// Number of normalization steps performed.
    pub num_steps: usize,
    /// Statistics.
    pub stats: CastStats,
}
#[allow(dead_code)]
pub struct NormCastExtDiag1400 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl NormCastExtDiag1400 {
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
/// A pipeline of TacticNormCast analysis passes.
#[allow(dead_code)]
pub struct TacticNormCastPipeline {
    pub passes: Vec<TacticNormCastAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticNormCastPipeline {
    pub fn new(name: &str) -> Self {
        TacticNormCastPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticNormCastAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticNormCastResult> {
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
pub struct NormCastExtPass1400 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<NormCastExtResult1400>,
}
impl NormCastExtPass1400 {
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
    pub fn run(&mut self, input: &str) -> NormCastExtResult1400 {
        if !self.enabled {
            return NormCastExtResult1400::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            NormCastExtResult1400::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            NormCastExtResult1400::Ok(format!(
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
pub struct NormCastExtDiff1400 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl NormCastExtDiff1400 {
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
/// A diagnostic reporter for TacticNormCast.
#[allow(dead_code)]
pub struct TacticNormCastDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticNormCastDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticNormCastDiagnostics {
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
/// The direction in which to normalize casts.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum CastDirection {
    /// Push casts toward leaves: `↑(a + b)` -> `↑a + ↑b`.
    Push,
    /// Pull casts toward root: `↑a + ↑b` -> `↑(a + b)`.
    Pull,
    /// Squash adjacent casts: `↑↑a` -> `↑a` (via transitivity of coercions).
    Squash,
}
/// Statistics for cast normalization.
#[derive(Clone, Debug, Default)]
pub struct CastStats {
    /// Number of push steps.
    pub push_steps: usize,
    /// Number of pull steps.
    pub pull_steps: usize,
    /// Number of squash steps.
    pub squash_steps: usize,
    /// Number of lemmas applied.
    pub lemmas_applied: usize,
    /// Number of cast chains discovered.
    pub chains_found: usize,
    /// Maximum chain length.
    pub max_chain_length: usize,
}
