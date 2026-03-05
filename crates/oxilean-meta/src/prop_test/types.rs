//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

/// Histogram: map labels to counts.
pub struct Histogram {
    /// The bucket counts.
    pub buckets: std::collections::HashMap<String, usize>,
}
impl Histogram {
    /// Create an empty histogram.
    pub fn new() -> Self {
        Histogram {
            buckets: std::collections::HashMap::new(),
        }
    }
    /// Increment the count for `label`.
    pub fn record(&mut self, label: &str) {
        *self.buckets.entry(label.to_string()).or_insert(0) += 1;
    }
    /// Total number of recorded events.
    pub fn total(&self) -> usize {
        self.buckets.values().sum()
    }
    /// Fraction of events with this label.
    pub fn fraction(&self, label: &str) -> f64 {
        let total = self.total();
        if total == 0 {
            return 0.0;
        }
        *self.buckets.get(label).unwrap_or(&0) as f64 / total as f64
    }
}
#[allow(dead_code)]
pub struct PropTestExtDiff700 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl PropTestExtDiff700 {
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
/// A diagnostic reporter for PropTest.
#[allow(dead_code)]
pub struct PropTestDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl PropTestDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        PropTestDiagnostics {
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
pub struct PropTestExtDiag700 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl PropTestExtDiag700 {
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
/// A simple deterministic pseudo-random generator (LCG)
pub struct Rng {
    pub(super) state: u64,
}
impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }
    pub fn next_usize(&mut self, max: usize) -> usize {
        (self.next_u64() as usize) % max.max(1)
    }
    pub fn next_bool(&mut self) -> bool {
        (self.next_u64() >> 33) % 2 == 0
    }
    pub fn next_u32(&mut self, max: u32) -> u32 {
        (self.next_u64() as u32) % max.max(1)
    }
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() as f64) / (u64::MAX as f64)
    }
}
/// A labelled counterexample with structured failure information.
#[derive(Debug, Clone)]
pub struct LabelledFail {
    /// Trial number at which the failure occurred.
    pub trial: u32,
    /// Property name that failed.
    pub label: String,
    /// Human-readable description.
    pub description: String,
}
/// A simple regression test record.
#[allow(dead_code)]
pub struct RegressionTestExt<T: PartialEq + std::fmt::Debug> {
    pub name: String,
    pub input: String,
    pub expected: T,
    pub actual: Option<T>,
}
#[allow(dead_code)]
impl<T: PartialEq + std::fmt::Debug> RegressionTestExt<T> {
    pub fn new(name: &str, input: &str, expected: T) -> Self {
        RegressionTestExt {
            name: name.to_string(),
            input: input.to_string(),
            expected,
            actual: None,
        }
    }
    pub fn set_actual(&mut self, v: T) {
        self.actual = Some(v);
    }
    pub fn is_pass(&self) -> bool {
        self.actual.as_ref() == Some(&self.expected)
    }
}
#[allow(dead_code)]
pub struct PropTestExtPipeline700 {
    pub name: String,
    pub passes: Vec<PropTestExtPass700>,
    pub run_count: usize,
}
impl PropTestExtPipeline700 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: PropTestExtPass700) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<PropTestExtResult700> {
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
/// Statistics about a collection of generated expressions.
pub struct ExprStats {
    /// Number of expressions sampled.
    pub count: usize,
    /// Minimum node count.
    pub min_nodes: usize,
    /// Maximum node count.
    pub max_nodes: usize,
    /// Total node count.
    pub total_nodes: usize,
    /// Minimum depth.
    pub min_depth: usize,
    /// Maximum depth.
    pub max_depth: usize,
    /// Total depth.
    pub total_depth: usize,
    /// Number of Const nodes.
    pub const_count: usize,
    /// Number of BVar nodes.
    pub bvar_count: usize,
    /// Number of App nodes.
    pub app_count: usize,
    /// Number of Lam nodes.
    pub lam_count: usize,
    /// Number of Pi nodes.
    pub pi_count: usize,
    /// Number of Lit nodes.
    pub lit_count: usize,
}
impl ExprStats {
    /// Compute statistics from `n` randomly generated expressions.
    pub fn from_samples(rng: &mut Rng, n: usize, depth: usize) -> Self {
        let mut stats = ExprStats {
            count: 0,
            min_nodes: usize::MAX,
            max_nodes: 0,
            total_nodes: 0,
            min_depth: usize::MAX,
            max_depth: 0,
            total_depth: 0,
            const_count: 0,
            bvar_count: 0,
            app_count: 0,
            lam_count: 0,
            pi_count: 0,
            lit_count: 0,
        };
        for _ in 0..n {
            let e = arbitrary_expr(rng, depth);
            let nc = properties::node_count(&e);
            let nd = expr_depth(&e);
            stats.count += 1;
            stats.min_nodes = stats.min_nodes.min(nc);
            stats.max_nodes = stats.max_nodes.max(nc);
            stats.total_nodes += nc;
            stats.min_depth = stats.min_depth.min(nd);
            stats.max_depth = stats.max_depth.max(nd);
            stats.total_depth += nd;
            count_expr_constructors_stats(&e, &mut stats);
        }
        if stats.count == 0 {
            stats.min_nodes = 0;
            stats.min_depth = 0;
        }
        stats
    }
    /// Average node count per expression.
    pub fn avg_nodes(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.total_nodes as f64 / self.count as f64
        }
    }
    /// Average depth per expression.
    pub fn avg_depth(&self) -> f64 {
        if self.count == 0 {
            0.0
        } else {
            self.total_depth as f64 / self.count as f64
        }
    }
}
#[allow(dead_code)]
pub struct PropTestExtPass700 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<PropTestExtResult700>,
}
impl PropTestExtPass700 {
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
    pub fn run(&mut self, input: &str) -> PropTestExtResult700 {
        if !self.enabled {
            return PropTestExtResult700::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            PropTestExtResult700::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            PropTestExtResult700::Ok(format!(
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
/// A result type for PropTest analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum PropTestResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl PropTestResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, PropTestResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, PropTestResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, PropTestResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, PropTestResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            PropTestResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            PropTestResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            PropTestResult::Ok(_) => 1.0,
            PropTestResult::Err(_) => 0.0,
            PropTestResult::Skipped => 0.0,
            PropTestResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A registry of regression cases.
pub struct RegressionSuite {
    /// The cases in this suite.
    pub cases: Vec<RegressionCase>,
}
impl RegressionSuite {
    /// Create a new empty suite.
    pub fn new() -> Self {
        RegressionSuite { cases: Vec::new() }
    }
    /// Add a case to the suite.
    pub fn add_case(&mut self, case: RegressionCase) {
        self.cases.push(case);
    }
    /// Count how many cases pass the given property.
    pub fn count_passing<F: Fn(&mut Rng) -> Option<bool>>(&self, prop: F) -> usize {
        self.cases.iter().filter(|c| c.verify_fixed(&prop)).count()
    }
}
/// A typed slot for PropTest configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PropTestConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl PropTestConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            PropTestConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            PropTestConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            PropTestConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            PropTestConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            PropTestConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            PropTestConfigValue::Bool(_) => "bool",
            PropTestConfigValue::Int(_) => "int",
            PropTestConfigValue::Float(_) => "float",
            PropTestConfigValue::Str(_) => "str",
            PropTestConfigValue::List(_) => "list",
        }
    }
}
/// A named regression test case (seed + trial number).
pub struct RegressionCase {
    /// Name of the regression case.
    pub name: String,
    /// Seed used when the failure was first observed.
    pub seed: u64,
    /// Trial number at which the failure first occurred.
    pub trial: u32,
}
impl RegressionCase {
    /// Create a new regression case.
    pub fn new(name: &str, seed: u64, trial: u32) -> Self {
        RegressionCase {
            name: name.to_string(),
            seed,
            trial,
        }
    }
    /// Re-run the property at the specific seed/trial.
    pub fn verify_fixed<F: Fn(&mut Rng) -> Option<bool>>(&self, prop: F) -> bool {
        let mut rng = Rng::new(self.seed);
        for _ in 0..self.trial {
            let _ = rng.next_u64();
        }
        matches!(prop(&mut rng), Some(true))
    }
}
/// An analysis pass for PropTest.
#[allow(dead_code)]
pub struct PropTestAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<PropTestResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl PropTestAnalysisPass {
    pub fn new(name: &str) -> Self {
        PropTestAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> PropTestResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            PropTestResult::Err("empty input".to_string())
        } else {
            PropTestResult::Ok(format!("processed: {}", input))
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
/// A pipeline of PropTest analysis passes.
#[allow(dead_code)]
pub struct PropTestPipeline {
    pub passes: Vec<PropTestAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl PropTestPipeline {
    pub fn new(name: &str) -> Self {
        PropTestPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: PropTestAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<PropTestResult> {
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
pub enum PropTestExtConfigVal700 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl PropTestExtConfigVal700 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let PropTestExtConfigVal700::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let PropTestExtConfigVal700::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let PropTestExtConfigVal700::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let PropTestExtConfigVal700::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let PropTestExtConfigVal700::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            PropTestExtConfigVal700::Bool(_) => "bool",
            PropTestExtConfigVal700::Int(_) => "int",
            PropTestExtConfigVal700::Float(_) => "float",
            PropTestExtConfigVal700::Str(_) => "str",
            PropTestExtConfigVal700::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct PropTestExtConfig700 {
    pub(super) values: std::collections::HashMap<String, PropTestExtConfigVal700>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl PropTestExtConfig700 {
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
    pub fn set(&mut self, key: &str, value: PropTestExtConfigVal700) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&PropTestExtConfigVal700> {
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
        self.set(key, PropTestExtConfigVal700::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, PropTestExtConfigVal700::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, PropTestExtConfigVal700::Str(v.to_string()))
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
/// Tracks which property branches have been exercised.
pub struct CoverageTracker {
    /// The hit counts by label.
    pub hits: std::collections::HashMap<String, u64>,
}
impl CoverageTracker {
    /// Create a new empty tracker.
    pub fn new() -> Self {
        CoverageTracker {
            hits: std::collections::HashMap::new(),
        }
    }
    /// Record a hit for the given label.
    pub fn hit(&mut self, label: &str) {
        *self.hits.entry(label.to_string()).or_insert(0) += 1;
    }
    /// Return the hit count for a label.
    pub fn count(&self, label: &str) -> u64 {
        *self.hits.get(label).unwrap_or(&0)
    }
    /// Return `true` if all required labels have been hit at least once.
    pub fn covers_all(&self, required: &[&str]) -> bool {
        required.iter().all(|r| self.count(r) > 0)
    }
    /// Return labels that have never been hit from the required set.
    pub fn missing(&self, required: &[&str]) -> Vec<String> {
        required
            .iter()
            .filter(|r| self.count(r) == 0)
            .map(|r| r.to_string())
            .collect()
    }
}
/// A property test result
#[derive(Debug, Clone)]
pub enum PropResult {
    Pass { trials: u32 },
    Fail { trial: u32, counterexample: String },
    Vacuous,
}
impl PropResult {
    pub fn is_pass(&self) -> bool {
        matches!(self, PropResult::Pass { .. })
    }
    pub fn is_fail(&self) -> bool {
        matches!(self, PropResult::Fail { .. })
    }
}
/// A property result type for extended testing.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PropCheckResult {
    Passed,
    Failed { reason: String, example: String },
    Skipped { reason: String },
}
#[allow(dead_code)]
impl PropCheckResult {
    pub fn is_passed(&self) -> bool {
        matches!(self, PropCheckResult::Passed)
    }
    pub fn is_failed(&self) -> bool {
        matches!(self, PropCheckResult::Failed { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, PropCheckResult::Skipped { .. })
    }
}
/// A named property test suite.
#[allow(dead_code)]
pub struct PropTestSuiteExt {
    pub name: String,
    pub tests: Vec<(String, Box<dyn Fn() -> bool>)>,
}
#[allow(dead_code)]
impl PropTestSuiteExt {
    pub fn new(name: &str) -> Self {
        PropTestSuiteExt {
            name: name.to_string(),
            tests: Vec::new(),
        }
    }
    pub fn add_test<F: Fn() -> bool + 'static>(&mut self, name: &str, test: F) {
        self.tests.push((name.to_string(), Box::new(test)));
    }
    pub fn run_all(&self) -> (usize, usize) {
        let (mut pass, mut fail) = (0, 0);
        for (_, test) in &self.tests {
            if test() {
                pass += 1;
            } else {
                fail += 1;
            }
        }
        (pass, fail)
    }
    pub fn all_pass(&self) -> bool {
        let (_, fail) = self.run_all();
        fail == 0
    }
    pub fn num_tests(&self) -> usize {
        self.tests.len()
    }
}
/// A diff for PropTest analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PropTestDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl PropTestDiff {
    pub fn new() -> Self {
        PropTestDiff {
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
/// A configuration store for PropTest.
#[allow(dead_code)]
pub struct PropTestConfig {
    pub values: std::collections::HashMap<String, PropTestConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl PropTestConfig {
    pub fn new() -> Self {
        PropTestConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: PropTestConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&PropTestConfigValue> {
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
        self.set(key, PropTestConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, PropTestConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, PropTestConfigValue::Str(v.to_string()))
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
/// A batch of property tests.
pub struct PropertyBatch {
    pub(super) items: Vec<(String, u64, u32)>,
}
impl PropertyBatch {
    /// Create an empty batch.
    pub fn new() -> Self {
        PropertyBatch { items: Vec::new() }
    }
    /// Add a property to run.
    pub fn add(&mut self, name: &str, seed: u64, trials: u32) {
        self.items.push((name.to_string(), seed, trials));
    }
    /// Run all properties and return (passes, failures).
    pub fn run_all<F>(&self, prop: F) -> (usize, usize)
    where
        F: Fn(&mut Rng) -> Option<bool>,
    {
        let mut passes = 0usize;
        let mut failures = 0usize;
        for (name, seed, trials) in &self.items {
            let result = check_property(*trials, *seed, name, &prop);
            match result {
                PropResult::Pass { .. } => passes += 1,
                PropResult::Fail { .. } => failures += 1,
                PropResult::Vacuous => {}
            }
        }
        (passes, failures)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum PropTestExtResult700 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl PropTestExtResult700 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, PropTestExtResult700::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, PropTestExtResult700::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, PropTestExtResult700::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, PropTestExtResult700::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let PropTestExtResult700::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let PropTestExtResult700::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            PropTestExtResult700::Ok(_) => 1.0,
            PropTestExtResult700::Err(_) => 0.0,
            PropTestExtResult700::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            PropTestExtResult700::Skipped => 0.5,
        }
    }
}
/// Statistics gathered over a property test run.
#[derive(Clone, Debug, Default)]
pub struct PropStats {
    /// Total number of trials attempted.
    pub total: u32,
    /// Number of trials that passed.
    pub passed: u32,
    /// Number of trials that failed.
    pub failed: u32,
    /// Number of trials that were vacuous (filtered).
    pub vacuous: u32,
}
impl PropStats {
    /// Create empty stats.
    pub fn new() -> Self {
        PropStats::default()
    }
    /// Pass rate as a value between 0.0 and 1.0.
    pub fn pass_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.passed as f64 / self.total as f64
        }
    }
    /// Fail rate as a value between 0.0 and 1.0.
    pub fn fail_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.failed as f64 / self.total as f64
        }
    }
    /// Whether the test is considered passing (no failures).
    pub fn is_passing(&self) -> bool {
        self.failed == 0 && self.passed > 0
    }
}
