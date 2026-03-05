//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;

/// The result of a decide procedure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DecideResult {
    /// The proposition is true.
    True,
    /// The proposition is false.
    False,
    /// The procedure could not determine the truth value.
    Unknown,
}
impl DecideResult {
    /// Return `true` if the result is `True`.
    pub fn is_true(&self) -> bool {
        matches!(self, DecideResult::True)
    }
    /// Return `true` if the result is `False`.
    pub fn is_false(&self) -> bool {
        matches!(self, DecideResult::False)
    }
    /// Return `true` if the result is `Unknown`.
    pub fn is_unknown(&self) -> bool {
        matches!(self, DecideResult::Unknown)
    }
}
/// A result type for TacticDecide analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticDecideResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticDecideResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticDecideResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticDecideResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticDecideResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticDecideResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticDecideResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticDecideResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticDecideResult::Ok(_) => 1.0,
            TacticDecideResult::Err(_) => 0.0,
            TacticDecideResult::Skipped => 0.0,
            TacticDecideResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// An analysis pass for TacticDecide.
#[allow(dead_code)]
pub struct TacticDecideAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticDecideResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticDecideAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticDecideAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticDecideResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticDecideResult::Err("empty input".to_string())
        } else {
            TacticDecideResult::Ok(format!("processed: {}", input))
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
/// A conjunction of decidable goals.
///
/// Evaluates a list of expressions and determines whether ALL are true.
#[allow(dead_code)]
pub struct DecideConjunction {
    pub(super) goals: Vec<String>,
    pub(super) tactic: DecideTactic,
}
impl DecideConjunction {
    /// Create a new conjunction with no goals.
    #[allow(dead_code)]
    pub fn new() -> Self {
        DecideConjunction {
            goals: Vec::new(),
            tactic: DecideTactic::new(),
        }
    }
    /// Add a goal string.
    #[allow(dead_code)]
    pub fn add_goal(&mut self, goal: &str) {
        self.goals.push(goal.to_string());
    }
    /// Try to prove all goals. Returns `True` iff every goal evaluates to `True`.
    #[allow(dead_code)]
    pub fn decide_all(&self) -> DecideResult {
        let mut result = DecideResult::True;
        for goal in &self.goals {
            let r = self.tactic.evaluate(goal);
            result = DecideTactic::decide_and(result, r.clone());
            if result == DecideResult::False {
                return DecideResult::False;
            }
        }
        result
    }
    /// Decide any: returns `True` if ANY goal evaluates to `True`.
    #[allow(dead_code)]
    pub fn decide_any(&self) -> DecideResult {
        let mut result = DecideResult::False;
        for goal in &self.goals {
            let r = self.tactic.evaluate(goal);
            result = DecideTactic::decide_or(result, r);
            if result == DecideResult::True {
                return DecideResult::True;
            }
        }
        result
    }
    /// Return the result for each goal individually.
    #[allow(dead_code)]
    pub fn decide_each(&self) -> Vec<DecideResult> {
        self.goals.iter().map(|g| self.tactic.evaluate(g)).collect()
    }
    /// Number of goals.
    #[allow(dead_code)]
    pub fn num_goals(&self) -> usize {
        self.goals.len()
    }
}
/// A typed slot for TacticDecide configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticDecideConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticDecideConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticDecideConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticDecideConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticDecideConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticDecideConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticDecideConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticDecideConfigValue::Bool(_) => "bool",
            TacticDecideConfigValue::Int(_) => "int",
            TacticDecideConfigValue::Float(_) => "float",
            TacticDecideConfigValue::Str(_) => "str",
            TacticDecideConfigValue::List(_) => "list",
        }
    }
}
/// Statistics for the decide tactic.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct DecideStats {
    /// Total number of goals attempted.
    pub total: usize,
    /// Number of goals decided True.
    pub decided_true: usize,
    /// Number of goals decided False.
    pub decided_false: usize,
    /// Number of goals that were Unknown.
    pub unknown: usize,
    /// Number of cache hits (if using CachedDecideTactic).
    pub cache_hits: usize,
}
impl DecideStats {
    /// Create empty stats.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a result.
    #[allow(dead_code)]
    pub fn record(&mut self, result: &DecideResult) {
        self.total += 1;
        match result {
            DecideResult::True => self.decided_true += 1,
            DecideResult::False => self.decided_false += 1,
            DecideResult::Unknown => self.unknown += 1,
        }
    }
    /// Success rate (True or False, i.e., not Unknown).
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        (self.decided_true + self.decided_false) as f64 / self.total as f64
    }
    /// Summary string.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "DecideStats {{ total: {}, true: {}, false: {}, unknown: {}, cache_hits: {} }}",
            self.total, self.decided_true, self.decided_false, self.unknown, self.cache_hits,
        )
    }
    /// Reset all counters.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        *self = Self::default();
    }
    /// Merge another stats record into this one.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &DecideStats) {
        self.total += other.total;
        self.decided_true += other.decided_true;
        self.decided_false += other.decided_false;
        self.unknown += other.unknown;
        self.cache_hits += other.cache_hits;
    }
}
/// A configuration store for TacticDecide.
#[allow(dead_code)]
pub struct TacticDecideConfig {
    pub values: std::collections::HashMap<String, TacticDecideConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticDecideConfig {
    pub fn new() -> Self {
        TacticDecideConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticDecideConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticDecideConfigValue> {
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
        self.set(key, TacticDecideConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticDecideConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticDecideConfigValue::Str(v.to_string()))
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
pub struct DecideExtPipeline900 {
    pub name: String,
    pub passes: Vec<DecideExtPass900>,
    pub run_count: usize,
}
impl DecideExtPipeline900 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: DecideExtPass900) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<DecideExtResult900> {
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
/// The decide tactic: evaluates decidable propositions.
#[derive(Clone, Debug)]
pub struct DecideTactic {
    pub(super) config: DecideConfig,
}
impl DecideTactic {
    /// Create a new `DecideTactic` with default configuration.
    pub fn new() -> Self {
        DecideTactic {
            config: DecideConfig::default(),
        }
    }
    /// Create a new `DecideTactic` with a custom configuration.
    pub fn with_config(config: DecideConfig) -> Self {
        DecideTactic { config }
    }
    /// Decide whether `a == b` for natural numbers.
    pub fn decide_nat_eq(a: u64, b: u64) -> DecideResult {
        if a == b {
            DecideResult::True
        } else {
            DecideResult::False
        }
    }
    /// Decide whether `a < b` for natural numbers.
    pub fn decide_nat_lt(a: u64, b: u64) -> DecideResult {
        if a < b {
            DecideResult::True
        } else {
            DecideResult::False
        }
    }
    /// Convert a boolean value into a `DecideResult`.
    pub fn decide_bool(b: bool) -> DecideResult {
        if b {
            DecideResult::True
        } else {
            DecideResult::False
        }
    }
    /// Decide the conjunction of two results.
    pub fn decide_and(r1: DecideResult, r2: DecideResult) -> DecideResult {
        match (r1, r2) {
            (DecideResult::True, DecideResult::True) => DecideResult::True,
            (DecideResult::False, _) | (_, DecideResult::False) => DecideResult::False,
            _ => DecideResult::Unknown,
        }
    }
    /// Decide the disjunction of two results.
    pub fn decide_or(r1: DecideResult, r2: DecideResult) -> DecideResult {
        match (r1, r2) {
            (DecideResult::True, _) | (_, DecideResult::True) => DecideResult::True,
            (DecideResult::False, DecideResult::False) => DecideResult::False,
            _ => DecideResult::Unknown,
        }
    }
    /// Decide the negation of a result.
    pub fn decide_not(r: DecideResult) -> DecideResult {
        match r {
            DecideResult::True => DecideResult::False,
            DecideResult::False => DecideResult::True,
            DecideResult::Unknown => DecideResult::Unknown,
        }
    }
    /// Evaluate a proposition given as a string representation.
    ///
    /// Handles simple patterns like "true", "false", "N = M", "N < M".
    pub fn evaluate(&self, expr: &str) -> DecideResult {
        let trimmed = expr.trim();
        if trimmed == "true" || trimmed == "True" {
            return DecideResult::True;
        }
        if trimmed == "false" || trimmed == "False" {
            return DecideResult::False;
        }
        if let Some(inner) = trimmed
            .strip_prefix("not ")
            .or_else(|| trimmed.strip_prefix('¬'))
        {
            return Self::decide_not(self.evaluate(inner));
        }
        if let Some(pos) = find_binary_op(trimmed, " and ") {
            let l = self.evaluate(&trimmed[..pos]);
            let r = self.evaluate(&trimmed[pos + 5..]);
            return Self::decide_and(l, r);
        }
        if let Some(pos) = find_binary_op(trimmed, " && ") {
            let l = self.evaluate(&trimmed[..pos]);
            let r = self.evaluate(&trimmed[pos + 4..]);
            return Self::decide_and(l, r);
        }
        if let Some(pos) = find_binary_op(trimmed, " or ") {
            let l = self.evaluate(&trimmed[..pos]);
            let r = self.evaluate(&trimmed[pos + 4..]);
            return Self::decide_or(l, r);
        }
        if let Some(pos) = find_binary_op(trimmed, " || ") {
            let l = self.evaluate(&trimmed[..pos]);
            let r = self.evaluate(&trimmed[pos + 4..]);
            return Self::decide_or(l, r);
        }
        if let Some(pos) = find_binary_op(trimmed, " = ") {
            if let (Ok(a), Ok(b)) = (
                trimmed[..pos].trim().parse::<u64>(),
                trimmed[pos + 3..].trim().parse::<u64>(),
            ) {
                return Self::decide_nat_eq(a, b);
            }
        }
        if let Some(pos) = find_binary_op(trimmed, " < ") {
            if let (Ok(a), Ok(b)) = (
                trimmed[..pos].trim().parse::<u64>(),
                trimmed[pos + 3..].trim().parse::<u64>(),
            ) {
                return Self::decide_nat_lt(a, b);
            }
        }
        if let Some(pos) = find_binary_op(trimmed, " <= ") {
            if let (Ok(a), Ok(b)) = (
                trimmed[..pos].trim().parse::<u64>(),
                trimmed[pos + 4..].trim().parse::<u64>(),
            ) {
                return Self::decide_bool(a <= b);
            }
        }
        DecideResult::Unknown
    }
}
/// The full decide engine combining all decision procedures.
#[allow(dead_code)]
pub struct DecideEngine {
    pub(super) tactic: DecideTactic,
    pub(super) stats: DecideStats,
    pub(super) cache: DecideCache,
}
impl DecideEngine {
    /// Create a new engine with default settings.
    #[allow(dead_code)]
    pub fn new() -> Self {
        DecideEngine {
            tactic: DecideTactic::new(),
            stats: DecideStats::new(),
            cache: DecideCache::new(256),
        }
    }
    /// Decide a propositional expression.
    #[allow(dead_code)]
    pub fn decide(&mut self, expr: &str) -> DecideResult {
        if let Some(r) = self.cache.get(expr) {
            self.stats.cache_hits += 1;
            self.stats.record(&r);
            return r;
        }
        let result = self.tactic.evaluate(expr);
        self.cache.store(expr.to_string(), result.clone());
        self.stats.record(&result);
        result
    }
    /// Decide a batch of goals (conjunction).
    #[allow(dead_code)]
    pub fn decide_all(&mut self, goals: &[&str]) -> DecideResult {
        let mut acc = DecideResult::True;
        for &g in goals {
            let r = self.decide(g);
            acc = DecideTactic::decide_and(acc, r.clone());
            if acc == DecideResult::False {
                return DecideResult::False;
            }
        }
        acc
    }
    /// Decide a batch of goals (disjunction).
    #[allow(dead_code)]
    pub fn decide_any(&mut self, goals: &[&str]) -> DecideResult {
        let mut acc = DecideResult::False;
        for &g in goals {
            let r = self.decide(g);
            acc = DecideTactic::decide_or(acc, r.clone());
            if acc == DecideResult::True {
                return DecideResult::True;
            }
        }
        acc
    }
    /// Access statistics.
    #[allow(dead_code)]
    pub fn stats(&self) -> &DecideStats {
        &self.stats
    }
    /// Reset statistics.
    #[allow(dead_code)]
    pub fn reset_stats(&mut self) {
        self.stats.reset();
    }
    /// Clear the cache.
    #[allow(dead_code)]
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
/// Extended integer decision procedures.
#[allow(dead_code)]
pub struct IntDecide;
impl IntDecide {
    /// Decide `a == b` for integers.
    #[allow(dead_code)]
    pub fn decide_int_eq(a: i64, b: i64) -> DecideResult {
        DecideTactic::decide_bool(a == b)
    }
    /// Decide `a < b` for integers.
    #[allow(dead_code)]
    pub fn decide_int_lt(a: i64, b: i64) -> DecideResult {
        DecideTactic::decide_bool(a < b)
    }
    /// Decide `a <= b` for integers.
    #[allow(dead_code)]
    pub fn decide_int_le(a: i64, b: i64) -> DecideResult {
        DecideTactic::decide_bool(a <= b)
    }
    /// Decide `a > b` for integers.
    #[allow(dead_code)]
    pub fn decide_int_gt(a: i64, b: i64) -> DecideResult {
        DecideTactic::decide_bool(a > b)
    }
    /// Decide `a >= b` for integers.
    #[allow(dead_code)]
    pub fn decide_int_ge(a: i64, b: i64) -> DecideResult {
        DecideTactic::decide_bool(a >= b)
    }
    /// Decide `a ≠ b` for integers.
    #[allow(dead_code)]
    pub fn decide_int_ne(a: i64, b: i64) -> DecideResult {
        DecideTactic::decide_bool(a != b)
    }
    /// Decide divisibility: `a ∣ b`.
    #[allow(dead_code)]
    pub fn decide_int_dvd(a: i64, b: i64) -> DecideResult {
        if a == 0 {
            DecideTactic::decide_bool(b == 0)
        } else {
            DecideTactic::decide_bool(b % a == 0)
        }
    }
    /// Decide coprimality: `gcd(a, b) = 1`.
    #[allow(dead_code)]
    pub fn decide_coprime(a: i64, b: i64) -> DecideResult {
        DecideTactic::decide_bool(gcd(a.unsigned_abs(), b.unsigned_abs()) == 1)
    }
    /// Decide whether `a` is even.
    #[allow(dead_code)]
    pub fn decide_even(a: i64) -> DecideResult {
        DecideTactic::decide_bool(a % 2 == 0)
    }
    /// Decide whether `a` is odd.
    #[allow(dead_code)]
    pub fn decide_odd(a: i64) -> DecideResult {
        DecideTactic::decide_bool(a % 2 != 0)
    }
    /// Decide whether `a` is prime (trial division, small primes only).
    #[allow(dead_code)]
    pub fn decide_prime(a: u64) -> DecideResult {
        DecideTactic::decide_bool(is_prime(a))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DecideExtResult900 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl DecideExtResult900 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, DecideExtResult900::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, DecideExtResult900::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, DecideExtResult900::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, DecideExtResult900::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let DecideExtResult900::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let DecideExtResult900::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            DecideExtResult900::Ok(_) => 1.0,
            DecideExtResult900::Err(_) => 0.0,
            DecideExtResult900::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            DecideExtResult900::Skipped => 0.5,
        }
    }
}
/// A diagnostic reporter for TacticDecide.
#[allow(dead_code)]
pub struct TacticDecideDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticDecideDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticDecideDiagnostics {
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
/// Decide properties of finite lists of integers.
#[allow(dead_code)]
pub struct ListDecide;
impl ListDecide {
    /// Decide whether a list contains a given element.
    #[allow(dead_code)]
    pub fn decide_mem(list: &[i64], elem: i64) -> DecideResult {
        DecideTactic::decide_bool(list.contains(&elem))
    }
    /// Decide whether all elements of a list satisfy a predicate.
    #[allow(dead_code)]
    pub fn decide_all_pos(list: &[i64]) -> DecideResult {
        DecideTactic::decide_bool(list.iter().all(|&x| x > 0))
    }
    /// Decide whether any element of a list is positive.
    #[allow(dead_code)]
    pub fn decide_any_pos(list: &[i64]) -> DecideResult {
        DecideTactic::decide_bool(list.iter().any(|&x| x > 0))
    }
    /// Decide whether a list is sorted in ascending order.
    #[allow(dead_code)]
    pub fn decide_sorted(list: &[i64]) -> DecideResult {
        DecideTactic::decide_bool(list.windows(2).all(|w| w[0] <= w[1]))
    }
    /// Decide whether a list is strictly sorted.
    #[allow(dead_code)]
    pub fn decide_strictly_sorted(list: &[i64]) -> DecideResult {
        DecideTactic::decide_bool(list.windows(2).all(|w| w[0] < w[1]))
    }
    /// Decide whether all elements are equal.
    #[allow(dead_code)]
    pub fn decide_all_equal(list: &[i64]) -> DecideResult {
        if list.is_empty() {
            return DecideResult::True;
        }
        let first = list[0];
        DecideTactic::decide_bool(list.iter().all(|&x| x == first))
    }
    /// Decide whether a list is a palindrome.
    #[allow(dead_code)]
    pub fn decide_palindrome(list: &[i64]) -> DecideResult {
        let n = list.len();
        DecideTactic::decide_bool((0..n / 2).all(|i| list[i] == list[n - 1 - i]))
    }
    /// Decide whether a list has all distinct elements.
    #[allow(dead_code)]
    pub fn decide_distinct(list: &[i64]) -> DecideResult {
        for i in 0..list.len() {
            for j in i + 1..list.len() {
                if list[i] == list[j] {
                    return DecideResult::False;
                }
            }
        }
        DecideResult::True
    }
}
#[allow(dead_code)]
pub struct DecideExtDiff900 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl DecideExtDiff900 {
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
/// Configuration for the decide tactic.
#[derive(Clone, Debug)]
pub struct DecideConfig {
    /// Maximum recursion depth for evaluation.
    pub max_depth: usize,
    /// Timeout in milliseconds (0 = no timeout).
    pub timeout_ms: u64,
}
/// Arithmetic decision procedures over integers and naturals.
#[allow(dead_code)]
pub struct ArithDecide;
impl ArithDecide {
    /// Decide `a + b = c`.
    #[allow(dead_code)]
    pub fn decide_sum_eq(a: i64, b: i64, c: i64) -> DecideResult {
        DecideTactic::decide_bool(a + b == c)
    }
    /// Decide `a * b = c`.
    #[allow(dead_code)]
    pub fn decide_product_eq(a: i64, b: i64, c: i64) -> DecideResult {
        DecideTactic::decide_bool(a * b == c)
    }
    /// Decide `a^n = c` for small n.
    #[allow(dead_code)]
    pub fn decide_power_eq(a: i64, n: u32, c: i64) -> DecideResult {
        DecideTactic::decide_bool(a.pow(n) == c)
    }
    /// Decide `a mod m = r`.
    #[allow(dead_code)]
    pub fn decide_mod_eq(a: i64, m: i64, r: i64) -> DecideResult {
        if m == 0 {
            return DecideResult::Unknown;
        }
        DecideTactic::decide_bool(a.rem_euclid(m) == r)
    }
    /// Decide `a ≡ b (mod m)`.
    #[allow(dead_code)]
    pub fn decide_congruent(a: i64, b: i64, m: i64) -> DecideResult {
        if m == 0 {
            return DecideResult::Unknown;
        }
        DecideTactic::decide_bool((a - b).rem_euclid(m) == 0)
    }
    /// Decide whether `n` is a perfect square.
    #[allow(dead_code)]
    pub fn decide_perfect_square(n: u64) -> DecideResult {
        let sq = (n as f64).sqrt() as u64;
        DecideTactic::decide_bool(sq * sq == n || (sq + 1) * (sq + 1) == n)
    }
    /// Decide whether `n` is a Fibonacci number.
    #[allow(dead_code)]
    pub fn decide_fibonacci(n: u64) -> DecideResult {
        let check = |k: u64| -> bool {
            let sq = (k as f64).sqrt() as u64;
            sq * sq == k || (sq + 1) * (sq + 1) == k
        };
        let five_n2 = 5u64.saturating_mul(n.saturating_mul(n));
        DecideTactic::decide_bool(
            check(five_n2.saturating_add(4)) || five_n2 >= 4 && check(five_n2 - 4),
        )
    }
    /// Decide whether `n` divides evenly into `a` and `b` (common divisor).
    #[allow(dead_code)]
    pub fn decide_common_divisor(n: u64, a: u64, b: u64) -> DecideResult {
        if n == 0 {
            return DecideTactic::decide_bool(a == 0 && b == 0);
        }
        DecideTactic::decide_bool(a % n == 0 && b % n == 0)
    }
}
/// Native decide: a faster path for evaluating decidable propositions.
///
/// In a full implementation this would compile to native code; here we
/// delegate to `DecideTactic::evaluate`.
#[derive(Clone, Debug)]
pub struct NativeDecide {
    pub(super) inner: DecideTactic,
}
impl NativeDecide {
    /// Create a new `NativeDecide`.
    pub fn new() -> Self {
        NativeDecide {
            inner: DecideTactic::new(),
        }
    }
    /// Run the native decide procedure on a string representation.
    pub fn run(&self, expr: &str) -> DecideResult {
        self.inner.evaluate(expr)
    }
}
/// A diff for TacticDecide analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticDecideDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticDecideDiff {
    pub fn new() -> Self {
        TacticDecideDiff {
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
#[allow(dead_code)]
pub struct DecideExtConfig900 {
    pub(super) values: std::collections::HashMap<String, DecideExtConfigVal900>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl DecideExtConfig900 {
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
    pub fn set(&mut self, key: &str, value: DecideExtConfigVal900) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&DecideExtConfigVal900> {
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
        self.set(key, DecideExtConfigVal900::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, DecideExtConfigVal900::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, DecideExtConfigVal900::Str(v.to_string()))
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
/// Decide set-theoretic properties over finite integer sets.
#[allow(dead_code)]
pub struct SetDecide;
impl SetDecide {
    /// Decide `x ∈ S`.
    #[allow(dead_code)]
    pub fn decide_mem(x: i64, set: &[i64]) -> DecideResult {
        DecideTactic::decide_bool(set.contains(&x))
    }
    /// Decide `S ⊆ T`.
    #[allow(dead_code)]
    pub fn decide_subset(s: &[i64], t: &[i64]) -> DecideResult {
        DecideTactic::decide_bool(s.iter().all(|x| t.contains(x)))
    }
    /// Decide `S = T` as sets.
    #[allow(dead_code)]
    pub fn decide_set_eq(s: &[i64], t: &[i64]) -> DecideResult {
        let sub_st = SetDecide::decide_subset(s, t);
        let sub_ts = SetDecide::decide_subset(t, s);
        DecideTactic::decide_and(sub_st, sub_ts)
    }
    /// Decide `S ∩ T = ∅` (disjointness).
    #[allow(dead_code)]
    pub fn decide_disjoint(s: &[i64], t: &[i64]) -> DecideResult {
        DecideTactic::decide_bool(!s.iter().any(|x| t.contains(x)))
    }
    /// Decide `|S| = n`.
    #[allow(dead_code)]
    pub fn decide_card_eq(s: &[i64], n: usize) -> DecideResult {
        let mut seen: Vec<i64> = Vec::new();
        for &x in s {
            if !seen.contains(&x) {
                seen.push(x);
            }
        }
        DecideTactic::decide_bool(seen.len() == n)
    }
    /// Decide whether the set is finite and non-empty.
    #[allow(dead_code)]
    pub fn decide_nonempty(s: &[i64]) -> DecideResult {
        DecideTactic::decide_bool(!s.is_empty())
    }
}
#[allow(dead_code)]
pub struct DecideExtPass900 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<DecideExtResult900>,
}
impl DecideExtPass900 {
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
    pub fn run(&mut self, input: &str) -> DecideExtResult900 {
        if !self.enabled {
            return DecideExtResult900::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            DecideExtResult900::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            DecideExtResult900::Ok(format!(
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
pub struct DecideExtDiag900 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl DecideExtDiag900 {
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
/// Extended logical connectives for decide.
#[allow(dead_code)]
pub struct DecideLogic;
impl DecideLogic {
    /// Exclusive-or: true iff exactly one of `r1`, `r2` is True.
    #[allow(dead_code)]
    pub fn decide_xor(r1: DecideResult, r2: DecideResult) -> DecideResult {
        match (&r1, &r2) {
            (DecideResult::True, DecideResult::False)
            | (DecideResult::False, DecideResult::True) => DecideResult::True,
            (DecideResult::True, DecideResult::True)
            | (DecideResult::False, DecideResult::False) => DecideResult::False,
            _ => DecideResult::Unknown,
        }
    }
    /// Implication: `r1 → r2` (False only if r1=True and r2=False).
    #[allow(dead_code)]
    pub fn decide_implies(r1: DecideResult, r2: DecideResult) -> DecideResult {
        match (&r1, &r2) {
            (DecideResult::True, DecideResult::False) => DecideResult::False,
            (DecideResult::False, _) => DecideResult::True,
            (_, DecideResult::True) => DecideResult::True,
            _ => DecideResult::Unknown,
        }
    }
    /// Biconditional (iff): `r1 ↔ r2`.
    #[allow(dead_code)]
    pub fn decide_iff(r1: DecideResult, r2: DecideResult) -> DecideResult {
        match (&r1, &r2) {
            (DecideResult::True, DecideResult::True)
            | (DecideResult::False, DecideResult::False) => DecideResult::True,
            (DecideResult::True, DecideResult::False)
            | (DecideResult::False, DecideResult::True) => DecideResult::False,
            _ => DecideResult::Unknown,
        }
    }
    /// Nand: `¬(r1 ∧ r2)`.
    #[allow(dead_code)]
    pub fn decide_nand(r1: DecideResult, r2: DecideResult) -> DecideResult {
        DecideTactic::decide_not(DecideTactic::decide_and(r1, r2))
    }
    /// Nor: `¬(r1 ∨ r2)`.
    #[allow(dead_code)]
    pub fn decide_nor(r1: DecideResult, r2: DecideResult) -> DecideResult {
        DecideTactic::decide_not(DecideTactic::decide_or(r1, r2))
    }
}
/// A memoization cache for decide results keyed by expression string.
#[allow(dead_code)]
pub struct DecideCache {
    pub(super) cache: HashMap<String, DecideResult>,
    pub(super) hit_count: u64,
    pub(super) miss_count: u64,
    pub(super) max_size: usize,
}
impl DecideCache {
    /// Create a new cache with the given maximum size.
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        DecideCache {
            cache: HashMap::new(),
            hit_count: 0,
            miss_count: 0,
            max_size,
        }
    }
    /// Look up an expression.
    #[allow(dead_code)]
    pub fn get(&mut self, expr: &str) -> Option<DecideResult> {
        if let Some(r) = self.cache.get(expr) {
            self.hit_count += 1;
            Some(r.clone())
        } else {
            self.miss_count += 1;
            None
        }
    }
    /// Store a result for an expression. Evicts oldest entries if over capacity.
    #[allow(dead_code)]
    pub fn store(&mut self, expr: String, result: DecideResult) {
        if self.cache.len() >= self.max_size {
            if let Some(k) = self.cache.keys().next().cloned() {
                self.cache.remove(&k);
            }
        }
        self.cache.insert(expr, result);
    }
    /// Clear the cache and reset counters.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hit_count = 0;
        self.miss_count = 0;
    }
    /// Hit rate (0.0 if no lookups).
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }
    /// Number of entries in the cache.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Whether the cache is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
/// A decide tactic that caches previously evaluated results.
#[allow(dead_code)]
pub struct CachedDecideTactic {
    pub(super) tactic: DecideTactic,
    pub(super) cache: DecideCache,
}
impl CachedDecideTactic {
    /// Create a new cached decide tactic.
    #[allow(dead_code)]
    pub fn new(cache_size: usize) -> Self {
        CachedDecideTactic {
            tactic: DecideTactic::new(),
            cache: DecideCache::new(cache_size),
        }
    }
    /// Evaluate an expression, using the cache for repeated lookups.
    #[allow(dead_code)]
    pub fn evaluate(&mut self, expr: &str) -> DecideResult {
        if let Some(r) = self.cache.get(expr) {
            return r;
        }
        let result = self.tactic.evaluate(expr);
        self.cache.store(expr.to_string(), result.clone());
        result
    }
    /// Access the underlying cache hit rate.
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        self.cache.hit_rate()
    }
}
/// A pipeline of TacticDecide analysis passes.
#[allow(dead_code)]
pub struct TacticDecidePipeline {
    pub passes: Vec<TacticDecideAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticDecidePipeline {
    pub fn new(name: &str) -> Self {
        TacticDecidePipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticDecideAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticDecideResult> {
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
pub enum DecideExtConfigVal900 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl DecideExtConfigVal900 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let DecideExtConfigVal900::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let DecideExtConfigVal900::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let DecideExtConfigVal900::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let DecideExtConfigVal900::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let DecideExtConfigVal900::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            DecideExtConfigVal900::Bool(_) => "bool",
            DecideExtConfigVal900::Int(_) => "int",
            DecideExtConfigVal900::Float(_) => "float",
            DecideExtConfigVal900::Str(_) => "str",
            DecideExtConfigVal900::List(_) => "list",
        }
    }
}
