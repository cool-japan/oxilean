//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::functions::{find_best_coeff, gcd_i64, parse_linear_expr};

/// An analysis pass for TacticLinearCombination.
#[allow(dead_code)]
pub struct TacticLinearCombinationAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticLinearCombinationResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticLinearCombinationAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticLinearCombinationAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticLinearCombinationResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticLinearCombinationResult::Err("empty input".to_string())
        } else {
            TacticLinearCombinationResult::Ok(format!("processed: {}", input))
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
/// A utility type for LinearComb (index 12).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil12 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil12 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil12 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
#[allow(dead_code)]
pub struct LinearCombinationExtPass2900 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<LinearCombinationExtResult2900>,
}
impl LinearCombinationExtPass2900 {
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
    pub fn run(&mut self, input: &str) -> LinearCombinationExtResult2900 {
        if !self.enabled {
            return LinearCombinationExtResult2900::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            LinearCombinationExtResult2900::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            LinearCombinationExtResult2900::Ok(format!(
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
/// A priority queue for LinearComb items.
#[allow(dead_code)]
pub struct LinearCombPriorityQueue {
    pub items: Vec<(LinearCombUtil0, i64)>,
}
#[allow(dead_code)]
impl LinearCombPriorityQueue {
    pub fn new() -> Self {
        LinearCombPriorityQueue { items: Vec::new() }
    }
    pub fn push(&mut self, item: LinearCombUtil0, priority: i64) {
        self.items.push((item, priority));
        self.items.sort_by_key(|(_, p)| -p);
    }
    pub fn pop(&mut self) -> Option<(LinearCombUtil0, i64)> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
    pub fn peek(&self) -> Option<&(LinearCombUtil0, i64)> {
        self.items.first()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
#[allow(dead_code)]
pub struct LinearCombinationExtConfig2900 {
    pub(super) values: std::collections::HashMap<String, LinearCombinationExtConfigVal2900>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl LinearCombinationExtConfig2900 {
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
    pub fn set(&mut self, key: &str, value: LinearCombinationExtConfigVal2900) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&LinearCombinationExtConfigVal2900> {
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
        self.set(key, LinearCombinationExtConfigVal2900::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, LinearCombinationExtConfigVal2900::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, LinearCombinationExtConfigVal2900::Str(v.to_string()))
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
/// A utility type for LinearComb (index 14).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil14 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil14 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil14 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A rational number p/q in lowest terms.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rat {
    pub numer: i64,
    pub denom: i64,
}
#[allow(dead_code)]
impl Rat {
    pub fn new(numer: i64, denom: i64) -> Self {
        assert!(denom != 0, "denominator must be non-zero");
        let sign = if denom < 0 { -1 } else { 1 };
        let n = numer * sign;
        let d = denom * sign;
        let g = gcd_i64(n.abs(), d.abs()).max(1);
        Rat {
            numer: n / g,
            denom: d / g,
        }
    }
    pub fn zero() -> Self {
        Rat { numer: 0, denom: 1 }
    }
    pub fn one() -> Self {
        Rat { numer: 1, denom: 1 }
    }
    pub fn add(&self, other: &Self) -> Self {
        Rat::new(
            self.numer * other.denom + other.numer * self.denom,
            self.denom * other.denom,
        )
    }
    pub fn sub(&self, other: &Self) -> Self {
        Rat::new(
            self.numer * other.denom - other.numer * self.denom,
            self.denom * other.denom,
        )
    }
    pub fn mul(&self, other: &Self) -> Self {
        Rat::new(self.numer * other.numer, self.denom * other.denom)
    }
    pub fn div(&self, other: &Self) -> Self {
        Rat::new(self.numer * other.denom, self.denom * other.numer)
    }
    pub fn neg(&self) -> Self {
        Rat {
            numer: -self.numer,
            denom: self.denom,
        }
    }
    pub fn abs_val(&self) -> Self {
        Rat {
            numer: self.numer.abs(),
            denom: self.denom,
        }
    }
    pub fn is_zero(&self) -> bool {
        self.numer == 0
    }
    pub fn is_pos(&self) -> bool {
        self.numer > 0
    }
    pub fn is_neg(&self) -> bool {
        self.numer < 0
    }
}
/// A utility type for LinearComb (index 9).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil9 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil9 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil9 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
#[allow(dead_code)]
pub struct LinearCombinationExtDiag2900 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl LinearCombinationExtDiag2900 {
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
/// A utility type for LinearComb (index 8).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil8 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil8 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil8 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// The linear_combination tactic.
#[derive(Clone, Debug)]
pub struct LinearCombinationTactic {
    /// Maximum coefficient magnitude to try.
    pub(super) max_coeff: i64,
}
impl LinearCombinationTactic {
    /// Create a new `LinearCombinationTactic`.
    pub fn new() -> Self {
        LinearCombinationTactic { max_coeff: 100 }
    }
    /// Run the tactic on a `LinearCombination` problem.
    ///
    /// Returns `true` if a valid combination was found and verified.
    pub fn run(&self, combo: &LinearCombination) -> bool {
        match combo.find_combination() {
            Some(coeffs) => combo.verify(&coeffs),
            None => false,
        }
    }
    /// Run the tactic given string representations of hypotheses and goal.
    ///
    /// Parses each string as a linear expression of the form:
    /// `"c1 * x1 + c2 * x2 + ... + constant"`.
    pub fn run_with_expr(&self, hyps: &[(String, &str)], goal: &str) -> bool {
        let mut combo = LinearCombination::new();
        for (name, expr_str) in hyps {
            if let Some(expr) = parse_linear_expr(expr_str) {
                combo.add_hypothesis(name, expr);
            } else {
                return false;
            }
        }
        if let Some(goal_expr) = parse_linear_expr(goal) {
            combo.set_goal(goal_expr);
        } else {
            return false;
        }
        self.run(&combo)
    }
}
/// A pipeline of TacticLinearCombination analysis passes.
#[allow(dead_code)]
pub struct TacticLinearCombinationPipeline {
    pub passes: Vec<TacticLinearCombinationAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticLinearCombinationPipeline {
    pub fn new(name: &str) -> Self {
        TacticLinearCombinationPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticLinearCombinationAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticLinearCombinationResult> {
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
/// A diff for TacticLinearCombination analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticLinearCombinationDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticLinearCombinationDiff {
    pub fn new() -> Self {
        TacticLinearCombinationDiff {
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
/// A linear combination problem: given hypotheses prove the goal.
#[derive(Clone, Debug)]
pub struct LinearCombination {
    /// Named linear hypotheses.
    pub hypotheses: Vec<(String, LinCombExpr)>,
    /// The goal expression (we try to prove `goal = 0`).
    pub goal: LinCombExpr,
}
impl LinearCombination {
    /// Create a new empty `LinearCombination`.
    pub fn new() -> Self {
        LinearCombination {
            hypotheses: Vec::new(),
            goal: LinCombExpr::new(),
        }
    }
    /// Add a hypothesis with the given name.
    pub fn add_hypothesis(&mut self, name: &str, expr: LinCombExpr) {
        self.hypotheses.push((name.to_string(), expr));
    }
    /// Set the goal expression.
    pub fn set_goal(&mut self, goal: LinCombExpr) {
        self.goal = goal;
    }
    /// Try to find coefficients such that `Σ coeffᵢ * hypᵢ = goal`.
    ///
    /// Uses a simple greedy approach: tries each hypothesis with coefficient
    /// ±1, ±2, ... to cancel variables in the goal.
    ///
    /// Returns `Some(coefficients)` if successful, `None` otherwise.
    pub fn find_combination(&self) -> Option<Vec<(String, i64)>> {
        let mut residual = self.goal.simplify();
        let mut coeffs: Vec<(String, i64)> = Vec::new();
        for (name, hyp) in &self.hypotheses {
            let simplified_hyp = hyp.simplify();
            if simplified_hyp.is_zero() {
                coeffs.push((name.clone(), 0));
                continue;
            }
            let best_coeff = find_best_coeff(&residual, &simplified_hyp);
            if best_coeff != 0 {
                let contribution = simplified_hyp.scale(best_coeff);
                let neg_contribution = contribution.negate();
                residual = residual.add(&neg_contribution);
            }
            coeffs.push((name.clone(), best_coeff));
        }
        if residual.is_zero() {
            Some(coeffs)
        } else {
            None
        }
    }
    /// Verify that `coeffs` form a valid linear combination proving the goal.
    ///
    /// Checks that `Σ coeffᵢ * hypᵢ = goal`.
    pub fn verify(&self, coeffs: &[(String, i64)]) -> bool {
        let mut combination = LinCombExpr::new();
        for (name, coeff) in coeffs {
            if let Some((_, hyp)) = self.hypotheses.iter().find(|(n, _)| n == name) {
                let scaled = hyp.scale(*coeff);
                combination = combination.add(&scaled);
            }
        }
        let diff = combination.add(&self.goal.negate());
        diff.is_zero()
    }
}
/// A utility type for LinearComb (index 7).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil7 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil7 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil7 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A logger for LinearComb operations.
#[allow(dead_code)]
pub struct LinearCombLogger {
    pub entries: Vec<String>,
    pub max_entries: usize,
    pub verbose: bool,
}
#[allow(dead_code)]
impl LinearCombLogger {
    pub fn new(max_entries: usize) -> Self {
        LinearCombLogger {
            entries: Vec::new(),
            max_entries,
            verbose: false,
        }
    }
    pub fn log(&mut self, msg: &str) {
        if self.entries.len() < self.max_entries {
            self.entries.push(msg.to_string());
        }
    }
    pub fn verbose(&mut self, msg: &str) {
        if self.verbose {
            self.log(msg);
        }
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn last(&self) -> Option<&str> {
        self.entries.last().map(|s| s.as_str())
    }
    pub fn enable_verbose(&mut self) {
        self.verbose = true;
    }
    pub fn disable_verbose(&mut self) {
        self.verbose = false;
    }
}
/// Statistics for LinearComb operations.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombStats {
    pub total_ops: usize,
    pub successful_ops: usize,
    pub failed_ops: usize,
    pub total_time_ns: u64,
    pub max_time_ns: u64,
}
#[allow(dead_code)]
impl LinearCombStats {
    pub fn new() -> Self {
        LinearCombStats::default()
    }
    pub fn record_success(&mut self, time_ns: u64) {
        self.total_ops += 1;
        self.successful_ops += 1;
        self.total_time_ns += time_ns;
        if time_ns > self.max_time_ns {
            self.max_time_ns = time_ns;
        }
    }
    pub fn record_failure(&mut self) {
        self.total_ops += 1;
        self.failed_ops += 1;
    }
    pub fn success_rate(&self) -> f64 {
        if self.total_ops == 0 {
            0.0
        } else {
            self.successful_ops as f64 / self.total_ops as f64
        }
    }
    pub fn avg_time_ns(&self) -> f64 {
        if self.successful_ops == 0 {
            0.0
        } else {
            self.total_time_ns as f64 / self.successful_ops as f64
        }
    }
    pub fn merge(&mut self, other: &Self) {
        self.total_ops += other.total_ops;
        self.successful_ops += other.successful_ops;
        self.failed_ops += other.failed_ops;
        self.total_time_ns += other.total_time_ns;
        if other.max_time_ns > self.max_time_ns {
            self.max_time_ns = other.max_time_ns;
        }
    }
}
/// A registry for LinearComb utilities.
#[allow(dead_code)]
pub struct LinearCombRegistry {
    pub entries: Vec<LinearCombUtil0>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl LinearCombRegistry {
    pub fn new(capacity: usize) -> Self {
        LinearCombRegistry {
            entries: Vec::new(),
            capacity,
        }
    }
    pub fn register(&mut self, entry: LinearCombUtil0) -> bool {
        if self.entries.len() >= self.capacity {
            return false;
        }
        self.entries.push(entry);
        true
    }
    pub fn lookup(&self, id: usize) -> Option<&LinearCombUtil0> {
        self.entries.iter().find(|e| e.id == id)
    }
    pub fn remove(&mut self, id: usize) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < before
    }
    pub fn active_entries(&self) -> Vec<&LinearCombUtil0> {
        self.entries.iter().filter(|e| e.is_active()).collect()
    }
    pub fn total_score(&self) -> i64 {
        self.entries.iter().map(|e| e.score()).sum()
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn is_full(&self) -> bool {
        self.entries.len() >= self.capacity
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LinearCombinationExtResult2900 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl LinearCombinationExtResult2900 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, LinearCombinationExtResult2900::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, LinearCombinationExtResult2900::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, LinearCombinationExtResult2900::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, LinearCombinationExtResult2900::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let LinearCombinationExtResult2900::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let LinearCombinationExtResult2900::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            LinearCombinationExtResult2900::Ok(_) => 1.0,
            LinearCombinationExtResult2900::Err(_) => 0.0,
            LinearCombinationExtResult2900::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            LinearCombinationExtResult2900::Skipped => 0.5,
        }
    }
}
#[allow(dead_code)]
pub struct LinearCombinationExtDiff2900 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl LinearCombinationExtDiff2900 {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LinearCombinationExtConfigVal2900 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl LinearCombinationExtConfigVal2900 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let LinearCombinationExtConfigVal2900::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let LinearCombinationExtConfigVal2900::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let LinearCombinationExtConfigVal2900::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let LinearCombinationExtConfigVal2900::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let LinearCombinationExtConfigVal2900::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            LinearCombinationExtConfigVal2900::Bool(_) => "bool",
            LinearCombinationExtConfigVal2900::Int(_) => "int",
            LinearCombinationExtConfigVal2900::Float(_) => "float",
            LinearCombinationExtConfigVal2900::Str(_) => "str",
            LinearCombinationExtConfigVal2900::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct LinearCombinationExtPipeline2900 {
    pub name: String,
    pub passes: Vec<LinearCombinationExtPass2900>,
    pub run_count: usize,
}
impl LinearCombinationExtPipeline2900 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: LinearCombinationExtPass2900) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<LinearCombinationExtResult2900> {
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
/// A univariate polynomial with i64 coefficients.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UniPoly {
    pub coeffs: Vec<i64>,
}
#[allow(dead_code)]
impl UniPoly {
    pub fn zero() -> Self {
        UniPoly { coeffs: vec![] }
    }
    pub fn constant(c: i64) -> Self {
        UniPoly { coeffs: vec![c] }
    }
    pub fn monomial(degree: usize, c: i64) -> Self {
        let mut coeffs = vec![0i64; degree + 1];
        coeffs[degree] = c;
        UniPoly { coeffs }
    }
    pub fn degree(&self) -> usize {
        for i in (0..self.coeffs.len()).rev() {
            if self.coeffs[i] != 0 {
                return i;
            }
        }
        0
    }
    pub fn eval(&self, x: i64) -> i64 {
        let mut result = 0i64;
        let mut x_pow = 1i64;
        for &c in &self.coeffs {
            result = result.saturating_add(c.saturating_mul(x_pow));
            x_pow = x_pow.saturating_mul(x);
        }
        result
    }
    pub fn add(&self, other: &Self) -> Self {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut coeffs = vec![0i64; len];
        for (i, &c) in self.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }
        for (i, &c) in other.coeffs.iter().enumerate() {
            coeffs[i] += c;
        }
        while coeffs.last() == Some(&0) {
            coeffs.pop();
        }
        UniPoly { coeffs }
    }
    pub fn mul(&self, other: &Self) -> Self {
        if self.coeffs.is_empty() || other.coeffs.is_empty() {
            return Self::zero();
        }
        let mut coeffs = vec![0i64; self.coeffs.len() + other.coeffs.len() - 1];
        for (i, &c1) in self.coeffs.iter().enumerate() {
            for (j, &c2) in other.coeffs.iter().enumerate() {
                coeffs[i + j] = coeffs[i + j].saturating_add(c1.saturating_mul(c2));
            }
        }
        UniPoly { coeffs }
    }
    pub fn is_zero(&self) -> bool {
        self.coeffs.iter().all(|&c| c == 0)
    }
    pub fn leading_coeff(&self) -> i64 {
        for i in (0..self.coeffs.len()).rev() {
            if self.coeffs[i] != 0 {
                return self.coeffs[i];
            }
        }
        0
    }
}
/// A utility type for LinearComb (index 11).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil11 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil11 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil11 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A utility type for LinearComb (index 10).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil10 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil10 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil10 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A utility type for LinearComb (index 13).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil13 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil13 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil13 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A matrix with rational entries.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RatMatrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<Rat>>,
}
#[allow(dead_code)]
impl RatMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        RatMatrix {
            rows,
            cols,
            data: vec![vec![Rat::zero(); cols]; rows],
        }
    }
    pub fn set(&mut self, r: usize, c: usize, v: Rat) {
        self.data[r][c] = v;
    }
    pub fn get(&self, r: usize, c: usize) -> &Rat {
        &self.data[r][c]
    }
    /// Row reduce to RREF; returns rank.
    pub fn rref(&mut self) -> usize {
        let mut pivot_row = 0;
        let mut rank = 0;
        for col in 0..self.cols {
            let mut found = None;
            for row in pivot_row..self.rows {
                if !self.data[row][col].is_zero() {
                    found = Some(row);
                    break;
                }
            }
            if let Some(row) = found {
                self.data.swap(pivot_row, row);
                let piv = self.data[pivot_row][col].clone();
                let inv_piv = Rat::one().div(&piv);
                for c in 0..self.cols {
                    let v = self.data[pivot_row][c].mul(&inv_piv);
                    self.data[pivot_row][c] = v;
                }
                for row in 0..self.rows {
                    if row != pivot_row && !self.data[row][col].is_zero() {
                        let factor = self.data[row][col].clone();
                        for c in 0..self.cols {
                            let sub = factor.mul(&self.data[pivot_row][c]);
                            let new_v = self.data[row][c].sub(&sub);
                            self.data[row][c] = new_v;
                        }
                    }
                }
                pivot_row += 1;
                rank += 1;
            }
        }
        rank
    }
}
/// A Farkas certificate for infeasibility of a linear system.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FarkasCert {
    /// Non-negative multipliers for each inequality.
    pub multipliers: Vec<Rat>,
    /// The resulting contradiction value (should be < 0 for infeasibility).
    pub combined_rhs: Rat,
}
#[allow(dead_code)]
impl FarkasCert {
    pub fn new(multipliers: Vec<Rat>, combined_rhs: Rat) -> Self {
        FarkasCert {
            multipliers,
            combined_rhs,
        }
    }
    pub fn is_valid_refutation(&self) -> bool {
        self.multipliers.iter().all(|m| m.is_pos() || m.is_zero()) && self.combined_rhs.is_neg()
    }
    pub fn num_multipliers(&self) -> usize {
        self.multipliers.len()
    }
}
/// A configuration store for TacticLinearCombination.
#[allow(dead_code)]
pub struct TacticLinearCombinationConfig {
    pub values: std::collections::HashMap<String, TacticLinearCombinationConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticLinearCombinationConfig {
    pub fn new() -> Self {
        TacticLinearCombinationConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticLinearCombinationConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticLinearCombinationConfigValue> {
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
        self.set(key, TacticLinearCombinationConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticLinearCombinationConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticLinearCombinationConfigValue::Str(v.to_string()))
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
/// A single term in a linear expression: `coefficient * variable`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinCombTerm {
    /// The scalar coefficient.
    pub coefficient: i64,
    /// The variable name.
    pub variable: String,
}
impl LinCombTerm {
    /// Create a new `LinCombTerm`.
    pub fn new(coefficient: i64, variable: impl Into<String>) -> Self {
        LinCombTerm {
            coefficient,
            variable: variable.into(),
        }
    }
}
/// A utility type for LinearComb (index 2).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil2 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil2 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil2 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A utility type for LinearComb (index 0).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil0 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil0 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil0 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A simple 2-variable LP solver over integers (bounded domain).
#[allow(dead_code)]
pub struct SimpleLp {
    pub constraints: Vec<([i64; 2], i64)>,
    pub objective: [i64; 2],
    pub bounds: i64,
}
#[allow(dead_code)]
impl SimpleLp {
    pub fn new(obj: [i64; 2], bounds: i64) -> Self {
        SimpleLp {
            constraints: Vec::new(),
            objective: obj,
            bounds,
        }
    }
    pub fn add_constraint(&mut self, coeffs: [i64; 2], rhs: i64) {
        self.constraints.push((coeffs, rhs));
    }
    pub fn solve(&self) -> LpSolveResult {
        let mut best_obj = i64::MIN;
        let mut best_xy = None;
        let b = self.bounds;
        for x in -b..=b {
            for y in -b..=b {
                if self
                    .constraints
                    .iter()
                    .all(|([a, bb], rhs)| a * x + bb * y <= *rhs)
                {
                    let obj = self.objective[0] * x + self.objective[1] * y;
                    if obj > best_obj {
                        best_obj = obj;
                        best_xy = Some((x, y));
                    }
                }
            }
        }
        match best_xy {
            Some((x, y)) => LpSolveResult::Optimal(x, y),
            None => LpSolveResult::Infeasible,
        }
    }
}
/// Result of an LP solve.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LpSolveResult {
    Optimal(i64, i64),
    Infeasible,
    Unbounded,
}
/// A linear combination: map from variable → coefficient + constant term.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinCombMap {
    pub terms: std::collections::HashMap<String, i64>,
    pub constant: i64,
}
#[allow(dead_code)]
impl LinCombMap {
    pub fn new() -> Self {
        LinCombMap {
            terms: std::collections::HashMap::new(),
            constant: 0,
        }
    }
    pub fn add_term(&mut self, coeff: i64, var: &str) {
        *self.terms.entry(var.to_string()).or_insert(0) += coeff;
        if self.terms.get(var) == Some(&0) {
            self.terms.remove(var);
        }
    }
    pub fn add_constant(&mut self, c: i64) {
        self.constant += c;
    }
    pub fn scale(&self, factor: i64) -> Self {
        LinCombMap {
            terms: self
                .terms
                .iter()
                .map(|(k, &v)| (k.clone(), v * factor))
                .collect(),
            constant: self.constant * factor,
        }
    }
    pub fn add(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (var, &coeff) in &other.terms {
            result.add_term(coeff, var);
        }
        result.constant += other.constant;
        result
    }
    pub fn is_zero(&self) -> bool {
        self.constant == 0 && self.terms.values().all(|&v| v == 0)
    }
    pub fn eval(&self, env: &std::collections::HashMap<String, i64>) -> i64 {
        let mut sum = self.constant;
        for (var, &coeff) in &self.terms {
            sum += coeff * env.get(var).copied().unwrap_or(0);
        }
        sum
    }
    pub fn num_terms(&self) -> usize {
        self.terms.len()
    }
}
/// A utility type for LinearComb (index 4).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil4 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil4 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil4 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A utility type for LinearComb (index 5).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil5 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil5 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil5 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A utility type for LinearComb (index 3).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil3 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil3 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil3 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A utility type for LinearComb (index 6).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil6 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil6 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil6 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A linear expression: `constant + c₁·x₁ + c₂·x₂ + ...`
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LinCombExpr {
    /// The variable terms.
    pub terms: Vec<LinCombTerm>,
    /// The constant part.
    pub constant: i64,
}
impl LinCombExpr {
    /// Create a new empty `LinCombExpr` (zero expression).
    pub fn new() -> Self {
        LinCombExpr {
            terms: Vec::new(),
            constant: 0,
        }
    }
    /// Add a term `coefficient * var` to this expression.
    pub fn add_term(&mut self, coeff: i64, var: &str) {
        self.terms.push(LinCombTerm::new(coeff, var));
    }
    /// Set the constant part of this expression.
    pub fn set_constant(&mut self, c: i64) {
        self.constant = c;
    }
    /// Evaluate this expression given variable assignments.
    pub fn evaluate(&self, assignments: &HashMap<String, i64>) -> i64 {
        let mut result = self.constant;
        for term in &self.terms {
            let val = assignments.get(&term.variable).copied().unwrap_or(0);
            result += term.coefficient * val;
        }
        result
    }
    /// Simplify by collecting like terms (summing coefficients for the same variable).
    pub fn simplify(&self) -> LinCombExpr {
        let mut coeff_map: HashMap<String, i64> = HashMap::new();
        for term in &self.terms {
            *coeff_map.entry(term.variable.clone()).or_insert(0) += term.coefficient;
        }
        let mut terms: Vec<LinCombTerm> = coeff_map
            .into_iter()
            .filter(|(_, c)| *c != 0)
            .map(|(var, coeff)| LinCombTerm::new(coeff, var))
            .collect();
        terms.sort_by(|a, b| a.variable.cmp(&b.variable));
        LinCombExpr {
            terms,
            constant: self.constant,
        }
    }
    /// Negate this expression: `-(c + Σ cᵢxᵢ) = -c + Σ (-cᵢ)xᵢ`.
    pub fn negate(&self) -> LinCombExpr {
        LinCombExpr {
            terms: self
                .terms
                .iter()
                .map(|t| LinCombTerm::new(-t.coefficient, t.variable.clone()))
                .collect(),
            constant: -self.constant,
        }
    }
    /// Add another expression to this one.
    pub fn add(&self, other: &LinCombExpr) -> LinCombExpr {
        let mut terms = self.terms.clone();
        terms.extend(other.terms.iter().cloned());
        LinCombExpr {
            terms,
            constant: self.constant + other.constant,
        }
        .simplify()
    }
    /// Scale this expression by a constant factor.
    pub fn scale(&self, c: i64) -> LinCombExpr {
        LinCombExpr {
            terms: self
                .terms
                .iter()
                .map(|t| LinCombTerm::new(t.coefficient * c, t.variable.clone()))
                .collect(),
            constant: self.constant * c,
        }
    }
    /// Return `true` if this expression is identically zero.
    pub fn is_zero(&self) -> bool {
        let s = self.simplify();
        s.constant == 0 && s.terms.is_empty()
    }
}
/// A result type for TacticLinearCombination analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticLinearCombinationResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticLinearCombinationResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticLinearCombinationResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticLinearCombinationResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticLinearCombinationResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticLinearCombinationResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticLinearCombinationResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticLinearCombinationResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticLinearCombinationResult::Ok(_) => 1.0,
            TacticLinearCombinationResult::Err(_) => 0.0,
            TacticLinearCombinationResult::Skipped => 0.0,
            TacticLinearCombinationResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A simple cache for LinearComb computations.
#[allow(dead_code)]
pub struct LinearCombCache {
    pub data: std::collections::HashMap<String, i64>,
    pub hits: usize,
    pub misses: usize,
}
#[allow(dead_code)]
impl LinearCombCache {
    pub fn new() -> Self {
        LinearCombCache {
            data: std::collections::HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    pub fn get(&mut self, key: &str) -> Option<i64> {
        if let Some(&v) = self.data.get(key) {
            self.hits += 1;
            Some(v)
        } else {
            self.misses += 1;
            None
        }
    }
    pub fn insert(&mut self, key: &str, value: i64) {
        self.data.insert(key.to_string(), value);
    }
    pub fn hit_rate(&self) -> f64 {
        let t = self.hits + self.misses;
        if t == 0 {
            0.0
        } else {
            self.hits as f64 / t as f64
        }
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn clear(&mut self) {
        self.data.clear();
        self.hits = 0;
        self.misses = 0;
    }
}
/// A diagnostic reporter for TacticLinearCombination.
#[allow(dead_code)]
pub struct TacticLinearCombinationDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticLinearCombinationDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticLinearCombinationDiagnostics {
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
/// A utility type for LinearComb (index 1).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LinearCombUtil1 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl LinearCombUtil1 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        LinearCombUtil1 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A typed slot for TacticLinearCombination configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticLinearCombinationConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticLinearCombinationConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticLinearCombinationConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticLinearCombinationConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticLinearCombinationConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticLinearCombinationConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticLinearCombinationConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticLinearCombinationConfigValue::Bool(_) => "bool",
            TacticLinearCombinationConfigValue::Int(_) => "int",
            TacticLinearCombinationConfigValue::Float(_) => "float",
            TacticLinearCombinationConfigValue::Str(_) => "str",
            TacticLinearCombinationConfigValue::List(_) => "list",
        }
    }
}
