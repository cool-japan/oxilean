//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Expr, Name};
use std::collections::HashMap;

pub struct CongrThmsExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl CongrThmsExtUtil {
    pub fn new(key: &str) -> Self {
        CongrThmsExtUtil {
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
/// A diagnostic reporter for CongrTheorems.
#[allow(dead_code)]
pub struct CongrTheoremsDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl CongrTheoremsDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        CongrTheoremsDiagnostics {
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
/// An extended utility type for CongrThms.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CongrThmsExt2 {
    /// A numeric tag.
    pub tag: u32,
}
#[allow(dead_code)]
impl CongrThmsExt2 {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { tag: 0 }
    }
}
/// An analysis pass for CongrTheorems.
#[allow(dead_code)]
pub struct CongrTheoremsAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<CongrTheoremsResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl CongrTheoremsAnalysisPass {
    pub fn new(name: &str) -> Self {
        CongrTheoremsAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> CongrTheoremsResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            CongrTheoremsResult::Err("empty input".to_string())
        } else {
            CongrTheoremsResult::Ok(format!("processed: {}", input))
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
/// An extended utility type for CongrThms.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CongrThmsExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl CongrThmsExt {
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
/// Congruence lemma builder.
#[allow(missing_docs)]
pub struct CongrBuilder {
    pub(super) fn_name: Name,
    pub(super) arg_specs: Vec<CongrArgKind>,
    pub(super) arg_types: Vec<Expr>,
}
impl CongrBuilder {
    #[allow(missing_docs)]
    pub fn for_fn(fn_name: Name) -> Self {
        Self {
            fn_name,
            arg_specs: Vec::new(),
            arg_types: Vec::new(),
        }
    }
    pub fn eq_arg(mut self, ty: Expr) -> Self {
        self.arg_specs.push(CongrArgKind::Eq);
        self.arg_types.push(ty);
        self
    }
    pub fn fixed_arg(mut self, ty: Expr) -> Self {
        self.arg_specs.push(CongrArgKind::Fixed);
        self.arg_types.push(ty);
        self
    }
    pub fn heq_arg(mut self, ty: Expr) -> Self {
        self.arg_specs.push(CongrArgKind::HEq);
        self.arg_types.push(ty);
        self
    }
    pub fn cast_arg(mut self, ty: Expr) -> Self {
        self.arg_specs.push(CongrArgKind::Cast);
        self.arg_types.push(ty);
        self
    }
    pub fn subsingleton_arg(mut self, ty: Expr) -> Self {
        self.arg_specs.push(CongrArgKind::Subsingle);
        self.arg_types.push(ty);
        self
    }
    pub fn build(self) -> MetaCongrTheorem {
        mk_mixed_congr(
            &self.fn_name,
            &self
                .arg_specs
                .iter()
                .zip(self.arg_types.iter())
                .map(|(k, t)| (*k, t.clone()))
                .collect::<Vec<_>>(),
        )
    }
}
/// A congruence closure context.
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct CongrClosure {
    pub(super) equalities: Vec<(Expr, Expr)>,
    pub(super) applied: Vec<MetaCongrTheorem>,
}
impl CongrClosure {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_equality(&mut self, lhs: Expr, rhs: Expr) {
        self.equalities.push((lhs, rhs));
    }
    pub fn are_congr(&self, e1: &Expr, e2: &Expr) -> bool {
        if e1 == e2 {
            return true;
        }
        for (lhs, rhs) in &self.equalities {
            if (lhs == e1 && rhs == e2) || (lhs == e2 && rhs == e1) {
                return true;
            }
        }
        false
    }
    pub fn record_applied(&mut self, thm: MetaCongrTheorem) {
        self.applied.push(thm);
    }
    pub fn num_applied(&self) -> usize {
        self.applied.len()
    }
    pub fn num_equalities(&self) -> usize {
        self.equalities.len()
    }
    pub fn clear(&mut self) {
        self.equalities.clear();
        self.applied.clear();
    }
}
/// A state machine for CongrThms.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CongrThmsState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl CongrThmsState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, CongrThmsState::Complete | CongrThmsState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, CongrThmsState::Initial | CongrThmsState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, CongrThmsState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            CongrThmsState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
#[allow(dead_code)]
pub struct CongrTheoremsExtPipeline2500 {
    pub name: String,
    pub passes: Vec<CongrTheoremsExtPass2500>,
    pub run_count: usize,
}
impl CongrTheoremsExtPipeline2500 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: CongrTheoremsExtPass2500) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<CongrTheoremsExtResult2500> {
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
/// A sliding window accumulator for CongrThms.
#[allow(dead_code)]
pub struct CongrThmsWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl CongrThmsWindow {
    pub fn new(capacity: usize) -> Self {
        CongrThmsWindow {
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
/// A typed slot for CongrTheorems configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CongrTheoremsConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl CongrTheoremsConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            CongrTheoremsConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            CongrTheoremsConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            CongrTheoremsConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            CongrTheoremsConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            CongrTheoremsConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            CongrTheoremsConfigValue::Bool(_) => "bool",
            CongrTheoremsConfigValue::Int(_) => "int",
            CongrTheoremsConfigValue::Float(_) => "float",
            CongrTheoremsConfigValue::Str(_) => "str",
            CongrTheoremsConfigValue::List(_) => "list",
        }
    }
}
/// A state machine controller for CongrThms.
#[allow(dead_code)]
pub struct CongrThmsStateMachine {
    pub state: CongrThmsState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl CongrThmsStateMachine {
    pub fn new() -> Self {
        CongrThmsStateMachine {
            state: CongrThmsState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: CongrThmsState) -> bool {
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
        self.transition_to(CongrThmsState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(CongrThmsState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(CongrThmsState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(CongrThmsState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
/// Statistics about congruence theorem usage.
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct CongrStats {
    pub applications: u64,
    pub misses: u64,
    pub validations: u64,
}
impl CongrStats {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(missing_docs)]
    pub fn record_application(&mut self) {
        self.applications += 1;
    }
    pub fn record_miss(&mut self) {
        self.misses += 1;
    }
    pub fn record_validation(&mut self) {
        self.validations += 1;
    }
    pub fn hit_rate(&self) -> f64 {
        let total = self.applications + self.misses;
        if total == 0 {
            1.0
        } else {
            self.applications as f64 / total as f64
        }
    }
}
/// A result type for CongrTheorems analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CongrTheoremsResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl CongrTheoremsResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, CongrTheoremsResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, CongrTheoremsResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, CongrTheoremsResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, CongrTheoremsResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            CongrTheoremsResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            CongrTheoremsResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            CongrTheoremsResult::Ok(_) => 1.0,
            CongrTheoremsResult::Err(_) => 0.0,
            CongrTheoremsResult::Skipped => 0.0,
            CongrTheoremsResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CongrTheoremsExtConfigVal2500 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl CongrTheoremsExtConfigVal2500 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let CongrTheoremsExtConfigVal2500::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let CongrTheoremsExtConfigVal2500::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let CongrTheoremsExtConfigVal2500::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let CongrTheoremsExtConfigVal2500::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let CongrTheoremsExtConfigVal2500::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            CongrTheoremsExtConfigVal2500::Bool(_) => "bool",
            CongrTheoremsExtConfigVal2500::Int(_) => "int",
            CongrTheoremsExtConfigVal2500::Float(_) => "float",
            CongrTheoremsExtConfigVal2500::Str(_) => "str",
            CongrTheoremsExtConfigVal2500::List(_) => "list",
        }
    }
}
/// A diff for CongrTheorems analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CongrTheoremsDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl CongrTheoremsDiff {
    pub fn new() -> Self {
        CongrTheoremsDiff {
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
/// Registry of congruence theorems indexed by function name.
#[derive(Clone, Debug, Default)]
pub struct CongrTheoremRegistry {
    pub(super) theorems: HashMap<String, Vec<MetaCongrTheorem>>,
}
impl CongrTheoremRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            theorems: HashMap::new(),
        }
    }
    /// Register a congruence theorem.
    pub fn register(&mut self, thm: MetaCongrTheorem) {
        let key = thm.fn_name.to_string();
        self.theorems.entry(key).or_default().push(thm);
    }
    /// Look up theorems for a function name.
    pub fn lookup(&self, name: &Name) -> Option<&[MetaCongrTheorem]> {
        self.theorems.get(&name.to_string()).map(|v| v.as_slice())
    }
    /// Look up theorem by name and arity.
    pub fn lookup_arity(&self, name: &Name, arity: u32) -> Option<&MetaCongrTheorem> {
        let thms = self.theorems.get(&name.to_string())?;
        thms.iter().find(|t| t.num_args == arity)
    }
    /// Return the total number of theorems.
    pub fn len(&self) -> usize {
        self.theorems.values().map(|v| v.len()).sum()
    }
    /// Check if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.theorems.is_empty()
    }
    /// Iterate over all theorems.
    pub fn iter(&self) -> impl Iterator<Item = &MetaCongrTheorem> {
        self.theorems.values().flatten()
    }
}
#[allow(dead_code)]
pub struct CongrTheoremsExtDiff2500 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl CongrTheoremsExtDiff2500 {
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
/// A generated congruence theorem.
#[derive(Clone, Debug)]
pub struct MetaCongrTheorem {
    /// Name of the function.
    pub fn_name: Name,
    /// Number of arguments covered.
    pub num_args: u32,
    /// Kind of each argument.
    pub arg_kinds: Vec<CongrArgKind>,
    /// Type of the congruence lemma.
    pub ty: Expr,
    /// Proof term (if available).
    pub proof: Option<Expr>,
}
#[allow(dead_code)]
pub struct CongrTheoremsExtDiag2500 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl CongrTheoremsExtDiag2500 {
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
#[allow(dead_code)]
pub struct CongrTheoremsExtConfig2500 {
    pub(super) values: std::collections::HashMap<String, CongrTheoremsExtConfigVal2500>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl CongrTheoremsExtConfig2500 {
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
    pub fn set(&mut self, key: &str, value: CongrTheoremsExtConfigVal2500) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&CongrTheoremsExtConfigVal2500> {
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
        self.set(key, CongrTheoremsExtConfigVal2500::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, CongrTheoremsExtConfigVal2500::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, CongrTheoremsExtConfigVal2500::Str(v.to_string()))
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
/// A counter map for CongrThms frequency analysis.
#[allow(dead_code)]
pub struct CongrThmsCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl CongrThmsCounterMap {
    pub fn new() -> Self {
        CongrThmsCounterMap {
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
/// A pipeline of CongrTheorems analysis passes.
#[allow(dead_code)]
pub struct CongrTheoremsPipeline {
    pub passes: Vec<CongrTheoremsAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl CongrTheoremsPipeline {
    pub fn new(name: &str) -> Self {
        CongrTheoremsPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: CongrTheoremsAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<CongrTheoremsResult> {
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
/// A configuration store for CongrTheorems.
#[allow(dead_code)]
pub struct CongrTheoremsConfig {
    pub values: std::collections::HashMap<String, CongrTheoremsConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl CongrTheoremsConfig {
    pub fn new() -> Self {
        CongrTheoremsConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: CongrTheoremsConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&CongrTheoremsConfigValue> {
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
        self.set(key, CongrTheoremsConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, CongrTheoremsConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, CongrTheoremsConfigValue::Str(v.to_string()))
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
/// A work queue for CongrThms items.
#[allow(dead_code)]
pub struct CongrThmsWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl CongrThmsWorkQueue {
    pub fn new(capacity: usize) -> Self {
        CongrThmsWorkQueue {
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
/// A congruence application result.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct CongrApplication {
    pub theorem: MetaCongrTheorem,
    pub subgoals: Vec<Expr>,
    pub conclusion: Expr,
}
impl CongrApplication {
    #[allow(missing_docs)]
    pub fn new(theorem: MetaCongrTheorem, subgoals: Vec<Expr>, conclusion: Expr) -> Self {
        Self {
            theorem,
            subgoals,
            conclusion,
        }
    }
    pub fn is_trivial(&self) -> bool {
        self.subgoals.is_empty()
    }
    pub fn num_subgoals(&self) -> usize {
        self.subgoals.len()
    }
}
/// A builder pattern for CongrThms.
#[allow(dead_code)]
pub struct CongrThmsBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl CongrThmsBuilder {
    pub fn new(name: &str) -> Self {
        CongrThmsBuilder {
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
pub struct CongrTheoremsExtPass2500 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<CongrTheoremsExtResult2500>,
}
impl CongrTheoremsExtPass2500 {
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
    pub fn run(&mut self, input: &str) -> CongrTheoremsExtResult2500 {
        if !self.enabled {
            return CongrTheoremsExtResult2500::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            CongrTheoremsExtResult2500::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            CongrTheoremsExtResult2500::Ok(format!(
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
pub enum CongrTheoremsExtResult2500 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl CongrTheoremsExtResult2500 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, CongrTheoremsExtResult2500::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, CongrTheoremsExtResult2500::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, CongrTheoremsExtResult2500::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, CongrTheoremsExtResult2500::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let CongrTheoremsExtResult2500::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let CongrTheoremsExtResult2500::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            CongrTheoremsExtResult2500::Ok(_) => 1.0,
            CongrTheoremsExtResult2500::Err(_) => 0.0,
            CongrTheoremsExtResult2500::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            CongrTheoremsExtResult2500::Skipped => 0.5,
        }
    }
}
/// An extended map for CongrThms keys to values.
#[allow(dead_code)]
pub struct CongrThmsExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> CongrThmsExtMap<V> {
    pub fn new() -> Self {
        CongrThmsExtMap {
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
/// How an argument participates in a congruence lemma.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CongrArgKind {
    /// Fixed argument (must be the same on both sides).
    Fixed,
    /// Equality argument (generates `aᵢ = bᵢ` subgoal).
    Eq,
    /// Heterogeneous equality argument.
    HEq,
    /// Cast argument (type changes based on earlier arguments).
    Cast,
    /// Subsingleton argument (at most one element, so always equal).
    Subsingle,
    /// Argument can be anything (dependency case).
    FixedNoParam,
}
