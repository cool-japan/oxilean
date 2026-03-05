//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::tactic::state::{TacticError, TacticResult};
use oxilean_kernel::{Expr, Level, Name};

/// A typed slot for TacticCalc configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticCalcConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticCalcConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticCalcConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticCalcConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticCalcConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticCalcConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticCalcConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticCalcConfigValue::Bool(_) => "bool",
            TacticCalcConfigValue::Int(_) => "int",
            TacticCalcConfigValue::Float(_) => "float",
            TacticCalcConfigValue::Str(_) => "str",
            TacticCalcConfigValue::List(_) => "list",
        }
    }
}
/// An extended map for TacCalc keys to values.
#[allow(dead_code)]
pub struct TacCalcExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> TacCalcExtMap<V> {
    pub fn new() -> Self {
        TacCalcExtMap {
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
#[allow(dead_code)]
pub struct CalcExtDiag1600 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl CalcExtDiag1600 {
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
/// A typed calculational proof chain.
#[derive(Clone, Debug)]
pub struct TypedCalcChain {
    /// Starting expression.
    pub start: Expr,
    /// Element type.
    pub ty: Expr,
    /// Steps.
    pub steps: Vec<TypedCalcStep>,
}
impl TypedCalcChain {
    /// Create a new typed calc chain.
    pub fn new(start: Expr, ty: Expr) -> Self {
        Self {
            start,
            ty,
            steps: Vec::new(),
        }
    }
    /// Add a step.
    pub fn step(mut self, kind: RelationKind, rhs: Expr, proof: Expr) -> Self {
        self.steps.push(TypedCalcStep {
            kind,
            rhs,
            proof,
            annotation: None,
        });
        self
    }
    /// Add an annotated step.
    pub fn step_ann(
        mut self,
        kind: RelationKind,
        rhs: Expr,
        proof: Expr,
        ann: impl Into<String>,
    ) -> Self {
        self.steps.push(TypedCalcStep {
            kind,
            rhs,
            proof,
            annotation: Some(ann.into()),
        });
        self
    }
    /// Get the current expression (after all steps).
    pub fn current(&self) -> &Expr {
        self.steps.last().map(|s| &s.rhs).unwrap_or(&self.start)
    }
    /// Get the overall relation kind.
    pub fn overall_relation(&self) -> Option<RelationKind> {
        if self.steps.is_empty() {
            return Some(RelationKind::Eq);
        }
        let mut weakest = self.steps[0].kind.clone();
        for step in &self.steps[1..] {
            if step.kind.strength() < weakest.strength() {
                weakest = step.kind.clone();
            }
        }
        Some(weakest)
    }
    /// Build the proof term.
    pub fn build(&self) -> crate::tactic::state::TacticResult<Expr> {
        if self.steps.is_empty() {
            return Err(crate::tactic::state::TacticError::Failed(
                "calc: empty chain".into(),
            ));
        }
        if self.steps.len() == 1 {
            return Ok(self.steps[0].proof.clone());
        }
        let mut proof = self.steps[0].proof.clone();
        for i in 1..self.steps.len() {
            let trans_name = self.steps[i - 1]
                .kind
                .trans_lemma(&self.steps[i].kind)
                .unwrap_or_else(|| Name::str("trans"));
            let trans = Expr::Const(trans_name, vec![Level::zero()]);
            proof = Expr::App(
                Box::new(Expr::App(Box::new(trans), Box::new(proof))),
                Box::new(self.steps[i].proof.clone()),
            );
        }
        Ok(proof)
    }
    /// Number of steps.
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Validate the chain.
    pub fn validate(&self) -> Result<(), String> {
        for i in 1..self.steps.len() {
            if self.steps[i - 1]
                .kind
                .trans_lemma(&self.steps[i].kind)
                .is_none()
            {
                return Err(format!(
                    "no transitivity for {:?} then {:?}",
                    self.steps[i - 1].kind,
                    self.steps[i].kind
                ));
            }
        }
        Ok(())
    }
}
/// Conv mode: focus on one side of an equation for rewriting.
#[derive(Clone, Debug)]
pub enum ConvSide {
    /// Focus on the left-hand side.
    Lhs,
    /// Focus on the right-hand side.
    Rhs,
}
/// A result type for TacticCalc analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticCalcResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticCalcResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticCalcResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticCalcResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticCalcResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticCalcResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticCalcResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticCalcResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticCalcResult::Ok(_) => 1.0,
            TacticCalcResult::Err(_) => 0.0,
            TacticCalcResult::Skipped => 0.0,
            TacticCalcResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// An extended utility type for TacCalc.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCalcExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl TacCalcExt {
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
/// A sliding window accumulator for TacCalc.
#[allow(dead_code)]
pub struct TacCalcWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl TacCalcWindow {
    pub fn new(capacity: usize) -> Self {
        TacCalcWindow {
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
#[allow(dead_code)]
pub struct CalcExtConfig1600 {
    pub(super) values: std::collections::HashMap<String, CalcExtConfigVal1600>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl CalcExtConfig1600 {
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
    pub fn set(&mut self, key: &str, value: CalcExtConfigVal1600) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&CalcExtConfigVal1600> {
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
        self.set(key, CalcExtConfigVal1600::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, CalcExtConfigVal1600::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, CalcExtConfigVal1600::Str(v.to_string()))
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
/// An analysis pass for TacticCalc.
#[allow(dead_code)]
pub struct TacticCalcAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticCalcResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticCalcAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticCalcAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticCalcResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticCalcResult::Err("empty input".to_string())
        } else {
            TacticCalcResult::Ok(format!("processed: {}", input))
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
pub struct TacCalcExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl TacCalcExtUtil {
    pub fn new(key: &str) -> Self {
        TacCalcExtUtil {
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
/// A single step in a calculational proof.
#[derive(Clone, Debug)]
pub struct CalcStep {
    /// The relation (e.g., `Eq`, `LE.le`, `LT.lt`).
    pub relation: Name,
    /// The right-hand side of this step.
    pub rhs: Expr,
    /// The proof of this step.
    pub proof: Expr,
}
/// A work queue for TacCalc items.
#[allow(dead_code)]
pub struct TacCalcWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl TacCalcWorkQueue {
    pub fn new(capacity: usize) -> Self {
        TacCalcWorkQueue {
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
/// A pipeline of TacticCalc analysis passes.
#[allow(dead_code)]
pub struct TacticCalcPipeline {
    pub passes: Vec<TacticCalcAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticCalcPipeline {
    pub fn new(name: &str) -> Self {
        TacticCalcPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticCalcAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticCalcResult> {
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
/// A counter map for TacCalc frequency analysis.
#[allow(dead_code)]
pub struct TacCalcCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl TacCalcCounterMap {
    pub fn new() -> Self {
        TacCalcCounterMap {
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
/// A diff for TacticCalc analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticCalcDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticCalcDiff {
    pub fn new() -> Self {
        TacticCalcDiff {
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
pub struct CalcExtPass1600 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<CalcExtResult1600>,
}
impl CalcExtPass1600 {
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
    pub fn run(&mut self, input: &str) -> CalcExtResult1600 {
        if !self.enabled {
            return CalcExtResult1600::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            CalcExtResult1600::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            CalcExtResult1600::Ok(format!(
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
/// A typed calc step with a RelationKind.
#[derive(Clone, Debug)]
pub struct TypedCalcStep {
    /// Relation kind for this step.
    pub kind: RelationKind,
    /// Right-hand side expression.
    pub rhs: Expr,
    /// Proof of this step.
    pub proof: Expr,
    /// Optional annotation.
    pub annotation: Option<String>,
}
#[allow(dead_code)]
pub struct CalcExtDiff1600 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl CalcExtDiff1600 {
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
/// A configuration store for TacticCalc.
#[allow(dead_code)]
pub struct TacticCalcConfig {
    pub values: std::collections::HashMap<String, TacticCalcConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticCalcConfig {
    pub fn new() -> Self {
        TacticCalcConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticCalcConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticCalcConfigValue> {
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
        self.set(key, TacticCalcConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticCalcConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticCalcConfigValue::Str(v.to_string()))
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
#[derive(Debug, Clone)]
pub enum CalcExtConfigVal1600 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl CalcExtConfigVal1600 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let CalcExtConfigVal1600::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let CalcExtConfigVal1600::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let CalcExtConfigVal1600::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let CalcExtConfigVal1600::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let CalcExtConfigVal1600::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            CalcExtConfigVal1600::Bool(_) => "bool",
            CalcExtConfigVal1600::Int(_) => "int",
            CalcExtConfigVal1600::Float(_) => "float",
            CalcExtConfigVal1600::Str(_) => "str",
            CalcExtConfigVal1600::List(_) => "list",
        }
    }
}
/// Relation kind: what relation connects two calc steps?
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelationKind {
    /// Propositional equality.
    Eq,
    /// Less-than-or-equal.
    Le,
    /// Less-than.
    Lt,
    /// Greater-than-or-equal.
    Ge,
    /// Greater-than.
    Gt,
    /// Iff.
    Iff,
    /// Custom relation.
    Custom(Name),
}
impl RelationKind {
    /// Parse a relation name into a RelationKind.
    pub fn from_name(name: &Name) -> Self {
        let s = format!("{}", name);
        match s.as_str() {
            "Eq" | "eq" => RelationKind::Eq,
            "LE.le" | "le" | "Le" => RelationKind::Le,
            "LT.lt" | "lt" | "Lt" => RelationKind::Lt,
            "GE.ge" | "ge" | "Ge" => RelationKind::Ge,
            "GT.gt" | "gt" | "Gt" => RelationKind::Gt,
            "Iff" | "iff" => RelationKind::Iff,
            _ => RelationKind::Custom(name.clone()),
        }
    }
    /// Get the transitivity lemma name for chaining this relation with another.
    pub fn trans_lemma(&self, other: &RelationKind) -> Option<Name> {
        match (self, other) {
            (RelationKind::Eq, RelationKind::Eq) => Some(Name::str("Eq.trans")),
            (RelationKind::Eq, RelationKind::Le) => Some(Name::str("eq_le_trans")),
            (RelationKind::Le, RelationKind::Eq) => Some(Name::str("le_eq_trans")),
            (RelationKind::Le, RelationKind::Le) => Some(Name::str("le_trans")),
            (RelationKind::Le, RelationKind::Lt) => Some(Name::str("le_lt_trans")),
            (RelationKind::Lt, RelationKind::Le) => Some(Name::str("lt_le_trans")),
            (RelationKind::Lt, RelationKind::Lt) => Some(Name::str("lt_trans")),
            (RelationKind::Eq, RelationKind::Lt) => Some(Name::str("eq_lt_trans")),
            (RelationKind::Lt, RelationKind::Eq) => Some(Name::str("lt_eq_trans")),
            (RelationKind::Iff, RelationKind::Iff) => Some(Name::str("Iff.trans")),
            _ => None,
        }
    }
    /// The strength of a relation.
    pub fn strength(&self) -> u8 {
        match self {
            RelationKind::Eq | RelationKind::Iff => 3,
            RelationKind::Le | RelationKind::Ge => 2,
            RelationKind::Lt | RelationKind::Gt => 1,
            RelationKind::Custom(_) => 0,
        }
    }
    /// Get the name of this relation as a Name.
    pub fn as_name(&self) -> Name {
        match self {
            RelationKind::Eq => Name::str("Eq"),
            RelationKind::Le => Name::str("LE.le"),
            RelationKind::Lt => Name::str("LT.lt"),
            RelationKind::Ge => Name::str("GE.ge"),
            RelationKind::Gt => Name::str("GT.gt"),
            RelationKind::Iff => Name::str("Iff"),
            RelationKind::Custom(n) => n.clone(),
        }
    }
}
impl RelationKind {
    /// Convert back to a `Name`.
    pub fn to_name(&self) -> Name {
        match self {
            RelationKind::Eq => Name::str("Eq"),
            RelationKind::Le => Name::str("LE.le"),
            RelationKind::Lt => Name::str("LT.lt"),
            RelationKind::Ge => Name::str("GE.ge"),
            RelationKind::Gt => Name::str("GT.gt"),
            RelationKind::Iff => Name::str("Iff"),
            RelationKind::Custom(n) => n.clone(),
        }
    }
    /// Check if this relation implies equality (is Eq).
    pub fn implies_eq(&self) -> bool {
        matches!(self, RelationKind::Eq)
    }
    /// Check if this is an ordering relation.
    pub fn is_ordering(&self) -> bool {
        matches!(
            self,
            RelationKind::Le | RelationKind::Lt | RelationKind::Ge | RelationKind::Gt
        )
    }
}
/// A state machine for TacCalc.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacCalcState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl TacCalcState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, TacCalcState::Complete | TacCalcState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, TacCalcState::Initial | TacCalcState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, TacCalcState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            TacCalcState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// A complete calculational proof.
#[derive(Clone, Debug)]
pub struct CalcProof {
    /// Starting expression.
    pub start: Expr,
    /// Steps in order.
    pub steps: Vec<CalcStep>,
    /// Type of the expressions.
    pub ty: Expr,
}
impl CalcProof {
    /// Create a new calculational proof starting from an expression.
    pub fn new(start: Expr, ty: Expr) -> Self {
        Self {
            start,
            steps: Vec::new(),
            ty,
        }
    }
    /// Add a step to the proof.
    pub fn add_step(&mut self, step: CalcStep) {
        self.steps.push(step);
    }
    /// Get the current expression (RHS of last step, or start).
    pub fn current(&self) -> &Expr {
        self.steps.last().map(|s| &s.rhs).unwrap_or(&self.start)
    }
    /// Get the number of steps.
    pub fn num_steps(&self) -> usize {
        self.steps.len()
    }
    /// Build the combined proof term.
    ///
    /// Chains equalities using Eq.trans, mixed relations using
    /// appropriate transitivity lemmas.
    pub fn build_proof(&self) -> TacticResult<Expr> {
        if self.steps.is_empty() {
            return Err(TacticError::Failed("calc: no steps".into()));
        }
        if self.steps.len() == 1 {
            return Ok(self.steps[0].proof.clone());
        }
        let mut combined = self.steps[0].proof.clone();
        let mut current_lhs = self.start.clone();
        for i in 1..self.steps.len() {
            let prev_rhs = self.steps[i - 1].rhs.clone();
            let step = &self.steps[i];
            combined = build_trans(
                &self.ty,
                &current_lhs,
                &prev_rhs,
                &step.rhs,
                combined,
                step.proof.clone(),
                &self.steps[i - 1].relation,
                &step.relation,
            );
            current_lhs = prev_rhs;
        }
        Ok(combined)
    }
}
#[allow(dead_code)]
pub struct CalcExtPipeline1600 {
    pub name: String,
    pub passes: Vec<CalcExtPass1600>,
    pub run_count: usize,
}
impl CalcExtPipeline1600 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: CalcExtPass1600) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<CalcExtResult1600> {
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
/// A diagnostic reporter for TacticCalc.
#[allow(dead_code)]
pub struct TacticCalcDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticCalcDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticCalcDiagnostics {
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
/// A state machine controller for TacCalc.
#[allow(dead_code)]
pub struct TacCalcStateMachine {
    pub state: TacCalcState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl TacCalcStateMachine {
    pub fn new() -> Self {
        TacCalcStateMachine {
            state: TacCalcState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: TacCalcState) -> bool {
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
        self.transition_to(TacCalcState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(TacCalcState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(TacCalcState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(TacCalcState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
/// A builder pattern for TacCalc.
#[allow(dead_code)]
pub struct TacCalcBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl TacCalcBuilder {
    pub fn new(name: &str) -> Self {
        TacCalcBuilder {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CalcExtResult1600 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl CalcExtResult1600 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, CalcExtResult1600::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, CalcExtResult1600::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, CalcExtResult1600::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, CalcExtResult1600::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let CalcExtResult1600::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let CalcExtResult1600::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            CalcExtResult1600::Ok(_) => 1.0,
            CalcExtResult1600::Err(_) => 0.0,
            CalcExtResult1600::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            CalcExtResult1600::Skipped => 0.5,
        }
    }
}
