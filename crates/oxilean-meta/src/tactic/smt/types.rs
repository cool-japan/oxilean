//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{smt_and_many, smt_is_ground, smt_negate, SmtSnapshot};

#[allow(dead_code)]
pub struct SmtExtConfig3100 {
    pub(super) values: std::collections::HashMap<String, SmtExtConfigVal3100>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl SmtExtConfig3100 {
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
    pub fn set(&mut self, key: &str, value: SmtExtConfigVal3100) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&SmtExtConfigVal3100> {
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
        self.set(key, SmtExtConfigVal3100::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, SmtExtConfigVal3100::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, SmtExtConfigVal3100::Str(v.to_string()))
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
#[allow(dead_code)]
pub struct SmtExtDiff3100 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl SmtExtDiff3100 {
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
/// Statistics collected during an SMT query session.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct SmtStats {
    /// Number of variables declared.
    pub declarations: usize,
    /// Number of assertions added.
    pub assertions: usize,
    /// Number of push/pop operations.
    pub push_pops: usize,
    /// Total size of all emitted SMT-LIB2 (in bytes).
    pub emitted_bytes: usize,
    /// Number of named assertions.
    pub named_assertions: usize,
}
#[allow(dead_code)]
impl SmtStats {
    /// Create new empty stats.
    pub fn new() -> Self {
        SmtStats::default()
    }
    /// Merge another stats object into this one.
    pub fn merge(&mut self, other: &SmtStats) {
        self.declarations += other.declarations;
        self.assertions += other.assertions;
        self.push_pops += other.push_pops;
        self.emitted_bytes += other.emitted_bytes;
        self.named_assertions += other.named_assertions;
    }
    /// Return assertions per declaration ratio (0.0 if no declarations).
    pub fn assertion_ratio(&self) -> f64 {
        if self.declarations == 0 {
            0.0
        } else {
            self.assertions as f64 / self.declarations as f64
        }
    }
    /// Return average assertion size in bytes.
    pub fn avg_assertion_size(&self) -> f64 {
        if self.assertions == 0 {
            0.0
        } else {
            self.emitted_bytes as f64 / self.assertions as f64
        }
    }
}
/// An analysis pass for TacticSmt.
#[allow(dead_code)]
pub struct TacticSmtAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticSmtResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticSmtAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticSmtAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticSmtResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticSmtResult::Err("empty input".to_string())
        } else {
            TacticSmtResult::Ok(format!("processed: {}", input))
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
/// A configuration store for TacticSmt.
#[allow(dead_code)]
pub struct TacticSmtConfig {
    pub values: std::collections::HashMap<String, TacticSmtConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticSmtConfig {
    pub fn new() -> Self {
        TacticSmtConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticSmtConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticSmtConfigValue> {
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
        self.set(key, TacticSmtConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticSmtConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticSmtConfigValue::Str(v.to_string()))
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
/// A result type for TacticSmt analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticSmtResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticSmtResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticSmtResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticSmtResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticSmtResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticSmtResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticSmtResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticSmtResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticSmtResult::Ok(_) => 1.0,
            TacticSmtResult::Err(_) => 0.0,
            TacticSmtResult::Skipped => 0.0,
            TacticSmtResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// Result of an SMT query
#[derive(Debug, Clone, PartialEq)]
pub enum SmtResult {
    Sat,
    Unsat,
    Unknown,
    Error(String),
}
/// A pipeline of TacticSmt analysis passes.
#[allow(dead_code)]
pub struct TacticSmtPipeline {
    pub passes: Vec<TacticSmtAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticSmtPipeline {
    pub fn new(name: &str) -> Self {
        TacticSmtPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticSmtAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticSmtResult> {
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
/// SMT-LIB2 sort
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmtSort {
    Bool,
    Int,
    Real,
    BitVec(u32),
    Array(Box<SmtSort>, Box<SmtSort>),
    Named(String),
}
/// Supported SMT solver backends
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SmtSolver {
    Z3,
    Cvc5,
    Yices2,
    Bitwuzla,
}
/// A proof obligation to be discharged by an SMT solver.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SmtProofObligation {
    /// Human-readable description.
    pub description: String,
    /// Hypothesis terms (assumed true).
    pub hypotheses: Vec<SmtTerm>,
    /// The goal to prove.
    pub goal: SmtTerm,
    /// Variable declarations needed.
    pub declarations: Vec<(String, SmtSort)>,
}
#[allow(dead_code)]
impl SmtProofObligation {
    /// Create a new proof obligation.
    pub fn new(goal: SmtTerm) -> Self {
        SmtProofObligation {
            description: String::new(),
            hypotheses: Vec::new(),
            goal,
            declarations: Vec::new(),
        }
    }
    /// Set the description.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }
    /// Add a hypothesis.
    pub fn add_hypothesis(&mut self, hyp: SmtTerm) {
        self.hypotheses.push(hyp);
    }
    /// Add a variable declaration.
    pub fn add_decl(&mut self, name: &str, sort: SmtSort) {
        self.declarations.push((name.to_string(), sort));
    }
    /// Convert to an SMT-LIB2 script that checks validity (refutation style).
    pub fn to_smtlib2_refutation(&self, solver: SmtSolver) -> String {
        let mut ctx = SmtContext::new(solver);
        for (name, sort) in &self.declarations {
            ctx.declare_const(name, sort.clone());
        }
        for hyp in &self.hypotheses {
            ctx.assert(hyp.clone());
        }
        ctx.assert(smt_negate(self.goal.clone()));
        ctx.emit_smtlib2()
    }
    /// Return `true` if the obligation has no hypotheses.
    pub fn is_ground_goal(&self) -> bool {
        self.hypotheses.is_empty() && smt_is_ground(&self.goal)
    }
    /// Return the number of variables involved.
    pub fn variable_count(&self) -> usize {
        self.declarations.len()
    }
    /// Build the negated goal that the solver should prove UNSAT for.
    pub fn negated_goal(&self) -> SmtTerm {
        let mut conjunction = self.hypotheses.clone();
        conjunction.push(smt_negate(self.goal.clone()));
        smt_and_many(conjunction)
    }
}
/// An SMT model: a mapping from variable names to values.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct SmtModel {
    pub(super) values: std::collections::HashMap<String, ModelValue>,
}
#[allow(dead_code)]
impl SmtModel {
    /// Create an empty model.
    pub fn new() -> Self {
        SmtModel::default()
    }
    /// Insert a value into the model.
    pub fn insert(&mut self, name: &str, value: ModelValue) {
        self.values.insert(name.to_string(), value);
    }
    /// Look up a variable value.
    pub fn get(&self, name: &str) -> Option<&ModelValue> {
        self.values.get(name)
    }
    /// Return the number of bindings.
    pub fn len(&self) -> usize {
        self.values.len()
    }
    /// Return `true` if the model is empty.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    /// Return all variable names in the model.
    pub fn variables(&self) -> Vec<&str> {
        self.values.keys().map(|s| s.as_str()).collect()
    }
    /// Evaluate a simple arithmetic term under this model.
    pub fn eval_int(&self, term: &SmtTerm) -> Option<i64> {
        match term {
            SmtTerm::IntLit(n) => Some(*n),
            SmtTerm::Var(name, SmtSort::Int) => self.get(name).and_then(|v| v.as_int()),
            SmtTerm::Add(a, b) => Some(self.eval_int(a)? + self.eval_int(b)?),
            SmtTerm::Sub(a, b) => Some(self.eval_int(a)? - self.eval_int(b)?),
            SmtTerm::Mul(a, b) => Some(self.eval_int(a)? * self.eval_int(b)?),
            _ => None,
        }
    }
    /// Evaluate a simple Boolean term under this model.
    pub fn eval_bool(&self, term: &SmtTerm) -> Option<bool> {
        match term {
            SmtTerm::BoolLit(b) => Some(*b),
            SmtTerm::Var(name, SmtSort::Bool) => self.get(name).and_then(|v| v.as_bool()),
            SmtTerm::Not(t) => Some(!self.eval_bool(t)?),
            SmtTerm::And(terms) => {
                for t in terms {
                    if !self.eval_bool(t)? {
                        return Some(false);
                    }
                }
                Some(true)
            }
            SmtTerm::Or(terms) => {
                for t in terms {
                    if self.eval_bool(t)? {
                        return Some(true);
                    }
                }
                Some(false)
            }
            SmtTerm::Implies(a, b) => Some(!self.eval_bool(a)? || self.eval_bool(b)?),
            SmtTerm::Lt(a, b) => Some(self.eval_int(a)? < self.eval_int(b)?),
            SmtTerm::Le(a, b) => Some(self.eval_int(a)? <= self.eval_int(b)?),
            SmtTerm::Gt(a, b) => Some(self.eval_int(a)? > self.eval_int(b)?),
            SmtTerm::Ge(a, b) => Some(self.eval_int(a)? >= self.eval_int(b)?),
            SmtTerm::Eq(a, b) => {
                if let (Some(ia), Some(ib)) = (self.eval_int(a), self.eval_int(b)) {
                    Some(ia == ib)
                } else if let (Some(ba), Some(bb)) = (self.eval_bool(a), self.eval_bool(b)) {
                    Some(ba == bb)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
#[allow(dead_code)]
pub struct SmtExtPass3100 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<SmtExtResult3100>,
}
impl SmtExtPass3100 {
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
    pub fn run(&mut self, input: &str) -> SmtExtResult3100 {
        if !self.enabled {
            return SmtExtResult3100::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            SmtExtResult3100::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            SmtExtResult3100::Ok(format!(
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
#[derive(Debug, Clone)]
pub enum SmtExtConfigVal3100 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl SmtExtConfigVal3100 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let SmtExtConfigVal3100::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let SmtExtConfigVal3100::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let SmtExtConfigVal3100::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let SmtExtConfigVal3100::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let SmtExtConfigVal3100::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            SmtExtConfigVal3100::Bool(_) => "bool",
            SmtExtConfigVal3100::Int(_) => "int",
            SmtExtConfigVal3100::Float(_) => "float",
            SmtExtConfigVal3100::Str(_) => "str",
            SmtExtConfigVal3100::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct SmtExtDiag3100 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl SmtExtDiag3100 {
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
/// A diff for TacticSmt analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticSmtDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticSmtDiff {
    pub fn new() -> Self {
        TacticSmtDiff {
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
/// SMT-LIB2 term
#[derive(Debug, Clone, PartialEq)]
pub enum SmtTerm {
    BoolLit(bool),
    IntLit(i64),
    RealLit(f64),
    Var(String, SmtSort),
    App(String, Vec<SmtTerm>),
    And(Vec<SmtTerm>),
    Or(Vec<SmtTerm>),
    Not(Box<SmtTerm>),
    Implies(Box<SmtTerm>, Box<SmtTerm>),
    Iff(Box<SmtTerm>, Box<SmtTerm>),
    Eq(Box<SmtTerm>, Box<SmtTerm>),
    Ite(Box<SmtTerm>, Box<SmtTerm>, Box<SmtTerm>),
    Add(Box<SmtTerm>, Box<SmtTerm>),
    Sub(Box<SmtTerm>, Box<SmtTerm>),
    Mul(Box<SmtTerm>, Box<SmtTerm>),
    Div(Box<SmtTerm>, Box<SmtTerm>),
    Mod(Box<SmtTerm>, Box<SmtTerm>),
    Lt(Box<SmtTerm>, Box<SmtTerm>),
    Le(Box<SmtTerm>, Box<SmtTerm>),
    Gt(Box<SmtTerm>, Box<SmtTerm>),
    Ge(Box<SmtTerm>, Box<SmtTerm>),
    Forall(Vec<(String, SmtSort)>, Box<SmtTerm>),
    Exists(Vec<(String, SmtSort)>, Box<SmtTerm>),
    BvLit(u64, u32),
    BvAdd(Box<SmtTerm>, Box<SmtTerm>),
    BvAnd(Box<SmtTerm>, Box<SmtTerm>),
    BvNot(Box<SmtTerm>),
    BvUlt(Box<SmtTerm>, Box<SmtTerm>),
}
impl SmtTerm {
    /// Emit SMT-LIB2 string representation
    pub fn to_smtlib(&self) -> String {
        match self {
            SmtTerm::BoolLit(true) => "true".to_string(),
            SmtTerm::BoolLit(false) => "false".to_string(),
            SmtTerm::IntLit(n) => {
                if *n < 0 {
                    format!("(- {})", -n)
                } else {
                    n.to_string()
                }
            }
            SmtTerm::RealLit(r) => format!("{}", r),
            SmtTerm::Var(name, _) => name.clone(),
            SmtTerm::App(func, args) => {
                if args.is_empty() {
                    func.clone()
                } else {
                    let arg_strs: Vec<String> = args.iter().map(|a| a.to_smtlib()).collect();
                    format!("({} {})", func, arg_strs.join(" "))
                }
            }
            SmtTerm::And(terms) => {
                if terms.is_empty() {
                    "true".to_string()
                } else if terms.len() == 1 {
                    terms[0].to_smtlib()
                } else {
                    let ts: Vec<String> = terms.iter().map(|t| t.to_smtlib()).collect();
                    format!("(and {})", ts.join(" "))
                }
            }
            SmtTerm::Or(terms) => {
                if terms.is_empty() {
                    "false".to_string()
                } else if terms.len() == 1 {
                    terms[0].to_smtlib()
                } else {
                    let ts: Vec<String> = terms.iter().map(|t| t.to_smtlib()).collect();
                    format!("(or {})", ts.join(" "))
                }
            }
            SmtTerm::Not(t) => format!("(not {})", t.to_smtlib()),
            SmtTerm::Implies(a, b) => format!("(=> {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Iff(a, b) => format!("(= {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Eq(a, b) => format!("(= {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Ite(c, t, e) => {
                format!(
                    "(ite {} {} {})",
                    c.to_smtlib(),
                    t.to_smtlib(),
                    e.to_smtlib()
                )
            }
            SmtTerm::Add(a, b) => format!("(+ {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Sub(a, b) => format!("(- {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Mul(a, b) => format!("(* {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Div(a, b) => format!("(div {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Mod(a, b) => format!("(mod {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Lt(a, b) => format!("(< {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Le(a, b) => format!("(<= {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Gt(a, b) => format!("(> {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Ge(a, b) => format!("(>= {} {})", a.to_smtlib(), b.to_smtlib()),
            SmtTerm::Forall(vars, body) => {
                let var_strs: Vec<String> =
                    vars.iter().map(|(n, s)| format!("({} {})", n, s)).collect();
                format!("(forall ({}) {})", var_strs.join(" "), body.to_smtlib())
            }
            SmtTerm::Exists(vars, body) => {
                let var_strs: Vec<String> =
                    vars.iter().map(|(n, s)| format!("({} {})", n, s)).collect();
                format!("(exists ({}) {})", var_strs.join(" "), body.to_smtlib())
            }
            SmtTerm::BvLit(val, width) => format!("(_ bv{} {})", val, width),
            SmtTerm::BvAdd(a, b) => {
                format!("(bvadd {} {})", a.to_smtlib(), b.to_smtlib())
            }
            SmtTerm::BvAnd(a, b) => {
                format!("(bvand {} {})", a.to_smtlib(), b.to_smtlib())
            }
            SmtTerm::BvNot(t) => format!("(bvnot {})", t.to_smtlib()),
            SmtTerm::BvUlt(a, b) => {
                format!("(bvult {} {})", a.to_smtlib(), b.to_smtlib())
            }
        }
    }
    /// Is this a literal (no free variables)?
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            SmtTerm::BoolLit(_) | SmtTerm::IntLit(_) | SmtTerm::RealLit(_) | SmtTerm::BvLit(..)
        )
    }
}
/// A typed slot for TacticSmt configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticSmtConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticSmtConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticSmtConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticSmtConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticSmtConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticSmtConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticSmtConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticSmtConfigValue::Bool(_) => "bool",
            TacticSmtConfigValue::Int(_) => "int",
            TacticSmtConfigValue::Float(_) => "float",
            TacticSmtConfigValue::Str(_) => "str",
            TacticSmtConfigValue::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct SmtExtPipeline3100 {
    pub name: String,
    pub passes: Vec<SmtExtPass3100>,
    pub run_count: usize,
}
impl SmtExtPipeline3100 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: SmtExtPass3100) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<SmtExtResult3100> {
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
/// A higher-level builder for constructing SMT-LIB2 queries.
#[allow(dead_code)]
pub struct SmtQueryBuilder {
    pub(super) context: SmtContext,
    pub(super) logic: String,
    pub(super) named_assertions: Vec<(String, SmtTerm)>,
}
#[allow(dead_code)]
impl SmtQueryBuilder {
    /// Create a new builder with the given solver and logic.
    pub fn new(solver: SmtSolver, logic: &str) -> Self {
        SmtQueryBuilder {
            context: SmtContext::new(solver),
            logic: logic.to_string(),
            named_assertions: Vec::new(),
        }
    }
    /// Add a constant declaration.
    pub fn declare(&mut self, name: &str, sort: SmtSort) -> &mut Self {
        self.context.declare_const(name, sort);
        self
    }
    /// Add an unnamed assertion.
    pub fn assert(&mut self, term: SmtTerm) -> &mut Self {
        self.context.assert(term);
        self
    }
    /// Add a named assertion (for unsat-core tracking).
    pub fn assert_named(&mut self, label: &str, term: SmtTerm) -> &mut Self {
        self.named_assertions
            .push((label.to_string(), term.clone()));
        self.context.assert(term);
        self
    }
    /// Declare multiple constants at once.
    pub fn declare_many(&mut self, decls: Vec<(&str, SmtSort)>) -> &mut Self {
        for (name, sort) in decls {
            self.context.declare_const(name, sort);
        }
        self
    }
    /// Set the logic string.
    pub fn with_logic(mut self, logic: &str) -> Self {
        self.logic = logic.to_string();
        self
    }
    /// Emit the full SMT-LIB2 script.
    pub fn build(&self) -> String {
        let mut out = format!("(set-logic {})\n", self.logic);
        out.push_str(&self.context.emit_smtlib2());
        out
    }
    /// Return the solver kind.
    pub fn solver(&self) -> &SmtSolver {
        &self.context.solver
    }
    /// Return the number of named assertions.
    pub fn named_assertion_count(&self) -> usize {
        self.named_assertions.len()
    }
    /// Return all labels for named assertions.
    pub fn assertion_labels(&self) -> Vec<&str> {
        self.named_assertions
            .iter()
            .map(|(l, _)| l.as_str())
            .collect()
    }
}
/// Configuration for the SMT tactic.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SmtTacticConfig {
    /// Which solver to prefer.
    pub solver: SmtSolver,
    /// Timeout in milliseconds (stub — not actually used).
    pub timeout_ms: u64,
    /// Whether to use quantified reasoning.
    pub allow_quantifiers: bool,
    /// Whether to use bit-vector theory.
    pub allow_bitvec: bool,
    /// Maximum size of generated SMT term.
    pub max_term_size: usize,
    /// Whether to emit debug output.
    pub debug: bool,
}
#[allow(dead_code)]
impl SmtTacticConfig {
    /// Default configuration.
    pub fn default_config() -> Self {
        SmtTacticConfig {
            solver: SmtSolver::Z3,
            timeout_ms: 5000,
            allow_quantifiers: true,
            allow_bitvec: true,
            max_term_size: 10_000,
            debug: false,
        }
    }
    /// Minimal configuration (no quantifiers, no bitvec).
    pub fn minimal() -> Self {
        SmtTacticConfig {
            solver: SmtSolver::Z3,
            timeout_ms: 1000,
            allow_quantifiers: false,
            allow_bitvec: false,
            max_term_size: 1000,
            debug: false,
        }
    }
    /// Return a copy with debug enabled.
    pub fn with_debug(mut self) -> Self {
        self.debug = true;
        self
    }
    /// Return a copy with a specific solver.
    pub fn with_solver(mut self, solver: SmtSolver) -> Self {
        self.solver = solver;
        self
    }
    /// Return a copy with a specific timeout.
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }
    /// Return `true` if quantifier-free linear arithmetic suffices.
    pub fn is_qflia(&self) -> bool {
        !self.allow_quantifiers && !self.allow_bitvec
    }
}
/// The outcome of an SMT tactic discharge attempt.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SmtTacticResult {
    /// The goal was proved (solver returned UNSAT on negation).
    Proved,
    /// Solver returned SAT with a counterexample model.
    CounterExample(SmtModel),
    /// Solver returned Unknown.
    Unknown(String),
    /// An error occurred during translation or emission.
    Error(String),
    /// The term was too large to translate.
    TooLarge,
}
#[allow(dead_code)]
impl SmtTacticResult {
    /// Return `true` if the goal was proved.
    pub fn is_proved(&self) -> bool {
        matches!(self, SmtTacticResult::Proved)
    }
    /// Return `true` if a counterexample was found.
    pub fn has_counterexample(&self) -> bool {
        matches!(self, SmtTacticResult::CounterExample(_))
    }
    /// Extract the counterexample model, if any.
    pub fn counterexample(&self) -> Option<&SmtModel> {
        match self {
            SmtTacticResult::CounterExample(m) => Some(m),
            _ => None,
        }
    }
    /// Return a human-readable description of the result.
    pub fn description(&self) -> &'static str {
        match self {
            SmtTacticResult::Proved => "proved",
            SmtTacticResult::CounterExample(_) => "counterexample found",
            SmtTacticResult::Unknown(_) => "unknown",
            SmtTacticResult::Error(_) => "error",
            SmtTacticResult::TooLarge => "term too large",
        }
    }
}
/// SMT query context — collects declarations and assertions
pub struct SmtContext {
    /// The solver backend preference (advisory — OxiZ is always used as backend).
    pub solver: SmtSolver,
    pub(super) declarations: Vec<(String, SmtSort)>,
    /// Function declarations: (name, arg_sorts, return_sort).
    pub(super) fun_declarations: Vec<(String, Vec<SmtSort>, SmtSort)>,
    pub(super) assertions: Vec<SmtTerm>,
    pub(super) options: Vec<(String, String)>,
    pub(super) stack: Vec<SmtSnapshot>,
}
impl SmtContext {
    pub fn new(solver: SmtSolver) -> Self {
        SmtContext {
            solver,
            declarations: Vec::new(),
            fun_declarations: Vec::new(),
            assertions: Vec::new(),
            options: Vec::new(),
            stack: Vec::new(),
        }
    }
    pub fn declare_const(&mut self, name: &str, sort: SmtSort) -> &mut Self {
        self.declarations.push((name.to_string(), sort));
        self
    }
    /// Declare an uninterpreted function.
    ///
    /// Use this for functions with one or more arguments. For nullary
    /// functions (constants), use `declare_const` instead.
    pub fn declare_fun(
        &mut self,
        name: &str,
        arg_sorts: Vec<SmtSort>,
        ret_sort: SmtSort,
    ) -> &mut Self {
        self.fun_declarations
            .push((name.to_string(), arg_sorts, ret_sort));
        self
    }
    pub fn assert(&mut self, term: SmtTerm) -> &mut Self {
        self.assertions.push(term);
        self
    }
    pub fn set_option(&mut self, key: &str, value: &str) -> &mut Self {
        self.options.push((key.to_string(), value.to_string()));
        self
    }
    /// Emit full SMT-LIB2 script
    pub fn emit_smtlib2(&self) -> String {
        let mut out = String::new();
        for (k, v) in &self.options {
            out.push_str(&format!("(set-option :{} {})\n", k, v));
        }
        out.push_str("(set-logic ALL)\n");
        for (name, sort) in &self.declarations {
            out.push_str(&format!("(declare-const {} {})\n", name, sort));
        }
        for (name, arg_sorts, ret_sort) in &self.fun_declarations {
            let args: Vec<String> = arg_sorts.iter().map(|s| s.to_string()).collect();
            out.push_str(&format!(
                "(declare-fun {} ({}) {})\n",
                name,
                args.join(" "),
                ret_sort
            ));
        }
        for term in &self.assertions {
            out.push_str(&format!("(assert {})\n", term.to_smtlib()));
        }
        out.push_str("(check-sat)\n");
        out
    }
    /// Invoke the OxiZ SMT solver on the current assertions and return
    /// `Sat`, `Unsat`, or `Unknown`.
    ///
    /// The `solver` field on this context is advisory; OxiZ is always used
    /// as the solving backend.  Any script-level parse error from OxiZ is
    /// returned as `SmtResult::Error(...)` rather than silently mapped to
    /// `Unknown`.
    pub fn check_sat(&self) -> SmtResult {
        let script = self.emit_smtlib2();
        let mut ctx = oxiz_solver::Context::new();
        match ctx.execute_script(&script) {
            Err(e) => SmtResult::Error(format!("OxiZ parse error: {}", e)),
            Ok(output) => {
                // Find the last non-empty line that is a recognised verdict.
                let verdict = output.iter().rev().find_map(|line| {
                    let trimmed = line.trim();
                    match trimmed {
                        "sat" | "unsat" | "unknown" => Some(trimmed),
                        _ => None,
                    }
                });
                match verdict {
                    Some("sat") => SmtResult::Sat,
                    Some("unsat") => SmtResult::Unsat,
                    Some("unknown") => SmtResult::Unknown,
                    _ => SmtResult::Unknown,
                }
            }
        }
    }
    /// Returns true if check_sat is Unsat
    pub fn check_unsat(&self) -> bool {
        self.check_sat() == SmtResult::Unsat
    }
    pub fn reset(&mut self) {
        self.declarations.clear();
        self.fun_declarations.clear();
        self.assertions.clear();
        self.options.clear();
        self.stack.clear();
    }
    pub fn assertion_count(&self) -> usize {
        self.assertions.len()
    }
    pub fn decl_count(&self) -> usize {
        self.declarations.len()
    }
    /// Number of declared uninterpreted functions.
    pub fn fun_decl_count(&self) -> usize {
        self.fun_declarations.len()
    }
    /// Push assertion stack (saves current declarations and assertions)
    pub fn push(&mut self) {
        self.stack
            .push((self.declarations.clone(), self.assertions.clone()));
    }
    /// Pop assertion stack (restores saved declarations and assertions)
    pub fn pop(&mut self) {
        if let Some((decls, asserts)) = self.stack.pop() {
            self.declarations = decls;
            self.assertions = asserts;
        }
    }
}
/// A model value returned by an SMT solver.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum ModelValue {
    Bool(bool),
    Int(i64),
    Real(f64),
    BitVec(u64, u32),
    Unknown,
}
#[allow(dead_code)]
impl ModelValue {
    /// Return `true` if this is a Boolean `true`.
    pub fn is_true(&self) -> bool {
        matches!(self, ModelValue::Bool(true))
    }
    /// Return `true` if this is a Boolean `false`.
    pub fn is_false(&self) -> bool {
        matches!(self, ModelValue::Bool(false))
    }
    /// Try to extract as integer.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            ModelValue::Int(n) => Some(*n),
            _ => None,
        }
    }
    /// Try to extract as bool.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ModelValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    /// Return `true` if the value is unknown.
    pub fn is_unknown(&self) -> bool {
        matches!(self, ModelValue::Unknown)
    }
}
/// A diagnostic reporter for TacticSmt.
#[allow(dead_code)]
pub struct TacticSmtDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticSmtDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticSmtDiagnostics {
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
#[derive(Debug, Clone)]
pub enum SmtExtResult3100 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl SmtExtResult3100 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, SmtExtResult3100::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, SmtExtResult3100::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, SmtExtResult3100::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, SmtExtResult3100::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let SmtExtResult3100::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let SmtExtResult3100::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            SmtExtResult3100::Ok(_) => 1.0,
            SmtExtResult3100::Err(_) => 0.0,
            SmtExtResult3100::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            SmtExtResult3100::Skipped => 0.5,
        }
    }
}
