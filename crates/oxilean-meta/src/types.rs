//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tactic::TacticState;
use std::collections::{HashMap, VecDeque};

/// Report of proof state completeness.
#[derive(Debug, Clone)]
pub struct ProofStateReport {
    /// Number of open goals.
    pub open_goals: usize,
    /// Whether the proof is complete.
    pub is_complete: bool,
}
impl ProofStateReport {
    /// Create from a tactic state.
    pub fn from_state(state: &TacticState) -> Self {
        ProofStateReport {
            open_goals: state.num_goals(),
            is_complete: state.is_done(),
        }
    }
}
/// A scored candidate.
#[derive(Debug, Clone)]
pub struct ScoredCandidate<T> {
    /// The candidate.
    pub candidate: T,
    /// Score.
    pub score: i64,
}
impl<T> ScoredCandidate<T> {
    /// Create a new scored candidate.
    pub fn new(candidate: T, score: i64) -> Self {
        Self { candidate, score }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LibExtResult1300 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl LibExtResult1300 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, LibExtResult1300::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, LibExtResult1300::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, LibExtResult1300::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, LibExtResult1300::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let LibExtResult1300::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let LibExtResult1300::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            LibExtResult1300::Ok(_) => 1.0,
            LibExtResult1300::Err(_) => 0.0,
            LibExtResult1300::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            LibExtResult1300::Skipped => 0.5,
        }
    }
}
/// A result type for Lib analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LibResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl LibResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, LibResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, LibResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, LibResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, LibResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            LibResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            LibResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            LibResult::Ok(_) => 1.0,
            LibResult::Err(_) => 0.0,
            LibResult::Skipped => 0.0,
            LibResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A builder pattern for MetaLib.
#[allow(dead_code)]
pub struct MetaLibBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl MetaLibBuilder {
    pub fn new(name: &str) -> Self {
        MetaLibBuilder {
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
/// A simple cache for memoizing meta computations.
#[derive(Debug)]
pub struct MetaCache<K, V> {
    pub entries: std::collections::HashMap<K, V>,
    pub capacity: usize,
    pub hits: u64,
    pub misses: u64,
}
impl<K: std::hash::Hash + Eq + Clone, V> MetaCache<K, V> {
    /// Create a cache with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: std::collections::HashMap::with_capacity(capacity),
            capacity,
            hits: 0,
            misses: 0,
        }
    }
    /// Insert a value.
    pub fn insert(&mut self, key: K, value: V) {
        if self.entries.len() >= self.capacity {
            let len = self.entries.len();
            if len > 0 {
                let to_remove = len / 2;
                let keys: Vec<K> = self.entries.keys().take(to_remove).cloned().collect();
                for k in keys {
                    self.entries.remove(&k);
                }
            }
        }
        self.entries.insert(key, value);
    }
    /// Look up a value.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.entries.contains_key(key) {
            self.hits += 1;
            self.entries.get(key)
        } else {
            self.misses += 1;
            None
        }
    }
    /// Cache hit rate.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear the cache.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.hits = 0;
        self.misses = 0;
    }
}
/// A pipeline of Lib analysis passes.
#[allow(dead_code)]
pub struct LibPipeline {
    pub passes: Vec<LibAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl LibPipeline {
    pub fn new(name: &str) -> Self {
        LibPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: LibAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<LibResult> {
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
pub struct LibExtDiff1300 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl LibExtDiff1300 {
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
/// A named group of related tactics.
#[derive(Clone, Debug)]
pub struct TacticGroup {
    /// Group name.
    pub name: String,
    /// Tactic names in this group.
    pub members: Vec<String>,
    /// Short description of the group.
    pub description: String,
}
impl TacticGroup {
    /// Create a tactic group.
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            members: Vec::new(),
            description: description.to_string(),
        }
    }
    /// Add a member tactic.
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, tactic: &str) -> Self {
        self.members.push(tactic.to_string());
        self
    }
    /// Whether a tactic is in this group.
    pub fn contains(&self, tactic: &str) -> bool {
        self.members.iter().any(|m| m == tactic)
    }
}
/// An extended map for MetaLib keys to values.
#[allow(dead_code)]
pub struct MetaLibExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> MetaLibExtMap<V> {
    pub fn new() -> Self {
        MetaLibExtMap {
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
/// A counter map for MetaLib frequency analysis.
#[allow(dead_code)]
pub struct MetaLibCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl MetaLibCounterMap {
    pub fn new() -> Self {
        MetaLibCounterMap {
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
/// A sliding window accumulator for MetaLib.
#[allow(dead_code)]
pub struct MetaLibWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl MetaLibWindow {
    pub fn new(capacity: usize) -> Self {
        MetaLibWindow {
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
pub struct LibExtConfig1300 {
    pub values: std::collections::HashMap<String, LibExtConfigVal1300>,
    pub read_only: bool,
    pub name: String,
}
impl LibExtConfig1300 {
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
    pub fn set(&mut self, key: &str, value: LibExtConfigVal1300) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&LibExtConfigVal1300> {
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
        self.set(key, LibExtConfigVal1300::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, LibExtConfigVal1300::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, LibExtConfigVal1300::Str(v.to_string()))
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
/// A work queue for MetaLib items.
#[allow(dead_code)]
pub struct MetaLibWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl MetaLibWorkQueue {
    pub fn new(capacity: usize) -> Self {
        MetaLibWorkQueue {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LibExtConfigVal1300 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl LibExtConfigVal1300 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let LibExtConfigVal1300::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let LibExtConfigVal1300::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let LibExtConfigVal1300::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let LibExtConfigVal1300::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let LibExtConfigVal1300::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            LibExtConfigVal1300::Bool(_) => "bool",
            LibExtConfigVal1300::Int(_) => "int",
            LibExtConfigVal1300::Float(_) => "float",
            LibExtConfigVal1300::Str(_) => "str",
            LibExtConfigVal1300::List(_) => "list",
        }
    }
}
/// A typed slot for Lib configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LibConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl LibConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            LibConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            LibConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            LibConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            LibConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            LibConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            LibConfigValue::Bool(_) => "bool",
            LibConfigValue::Int(_) => "int",
            LibConfigValue::Float(_) => "float",
            LibConfigValue::Str(_) => "str",
            LibConfigValue::List(_) => "list",
        }
    }
}
/// An extended utility type for MetaLib.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaLibExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl MetaLibExtUtil {
    pub fn new(key: &str) -> Self {
        MetaLibExtUtil {
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
/// Summary statistics about a `MetaContext`.
#[derive(Debug, Clone, Default)]
pub struct MetaStats {
    /// Number of expression metavariables.
    pub num_expr_mvars: usize,
    /// Number of assigned expression metavariables.
    pub num_assigned_expr: usize,
    /// Number of level metavariables.
    pub num_level_mvars: usize,
    /// Number of assigned level metavariables.
    pub num_assigned_levels: usize,
    /// Number of postponed constraints.
    pub num_postponed: usize,
}
/// An analysis pass for Lib.
#[allow(dead_code)]
pub struct LibAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<LibResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl LibAnalysisPass {
    pub fn new(name: &str) -> Self {
        LibAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> LibResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            LibResult::Err("empty input".to_string())
        } else {
            LibResult::Ok(format!("processed: {}", input))
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
/// Simple accumulator for meta-layer performance statistics.
#[allow(dead_code)]
pub struct PerfStats {
    /// Total number of elaboration attempts.
    pub elab_attempts: u64,
    /// Number of successful elaborations.
    pub elab_successes: u64,
    /// Total unification attempts.
    pub unif_attempts: u64,
    /// Number of successful unifications.
    pub unif_successes: u64,
    /// Total elapsed time in microseconds.
    pub elapsed_us: u64,
}
#[allow(dead_code)]
impl PerfStats {
    /// Create an empty stats record.
    pub fn new() -> Self {
        PerfStats {
            elab_attempts: 0,
            elab_successes: 0,
            unif_attempts: 0,
            unif_successes: 0,
            elapsed_us: 0,
        }
    }
    /// Return the elaboration success rate as a fraction in [0, 1].
    pub fn elab_success_rate(&self) -> f64 {
        if self.elab_attempts == 0 {
            return 0.0;
        }
        self.elab_successes as f64 / self.elab_attempts as f64
    }
    /// Return the unification success rate as a fraction in [0, 1].
    pub fn unif_success_rate(&self) -> f64 {
        if self.unif_attempts == 0 {
            return 0.0;
        }
        self.unif_successes as f64 / self.unif_attempts as f64
    }
    /// Merge another `PerfStats` into this one.
    pub fn merge(&mut self, other: &PerfStats) {
        self.elab_attempts += other.elab_attempts;
        self.elab_successes += other.elab_successes;
        self.unif_attempts += other.unif_attempts;
        self.unif_successes += other.unif_successes;
        self.elapsed_us += other.elapsed_us;
    }
}
impl Default for PerfStats {
    fn default() -> Self {
        Self::new()
    }
}
/// A named tactic registry.
#[derive(Debug, Default)]
pub struct TacticRegistry {
    pub entries: std::collections::HashMap<String, usize>,
    pub names: Vec<String>,
}
impl TacticRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a tactic name.
    pub fn register(&mut self, name: impl Into<String>) -> usize {
        let name = name.into();
        if let Some(&idx) = self.entries.get(&name) {
            return idx;
        }
        let idx = self.names.len();
        self.entries.insert(name.clone(), idx);
        self.names.push(name);
        idx
    }
    /// Look up a tactic index by name.
    pub fn lookup(&self, name: &str) -> Option<usize> {
        self.entries.get(name).copied()
    }
    /// Get the name for an index.
    pub fn name_of(&self, idx: usize) -> Option<&str> {
        self.names.get(idx).map(String::as_str)
    }
    /// Number of registered tactics.
    pub fn len(&self) -> usize {
        self.names.len()
    }
    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
    /// Get all registered names.
    pub fn all_names(&self) -> &[String] {
        &self.names
    }
}
/// A state machine for MetaLib.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MetaLibState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl MetaLibState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, MetaLibState::Complete | MetaLibState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, MetaLibState::Initial | MetaLibState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, MetaLibState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            MetaLibState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// A diff for Lib analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LibDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl LibDiff {
    pub fn new() -> Self {
        LibDiff {
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
/// A configuration store for Lib.
#[allow(dead_code)]
pub struct LibConfig {
    pub values: std::collections::HashMap<String, LibConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl LibConfig {
    pub fn new() -> Self {
        LibConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: LibConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&LibConfigValue> {
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
        self.set(key, LibConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, LibConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, LibConfigValue::Str(v.to_string()))
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
pub struct LibExtPass1300 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<LibExtResult1300>,
}
impl LibExtPass1300 {
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
    pub fn run(&mut self, input: &str) -> LibExtResult1300 {
        if !self.enabled {
            return LibExtResult1300::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            LibExtResult1300::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            LibExtResult1300::Ok(format!(
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
pub struct LibExtPipeline1300 {
    pub name: String,
    pub passes: Vec<LibExtPass1300>,
    pub run_count: usize,
}
impl LibExtPipeline1300 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: LibExtPass1300) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<LibExtResult1300> {
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
/// Feature flags for the meta layer.
#[derive(Clone, Debug)]
pub struct MetaFeatures {
    /// Enable discrimination tree indexing for simp lemmas.
    pub discr_tree: bool,
    /// Enable memoization of WHNF results.
    pub whnf_cache: bool,
    /// Enable proof term recording.
    pub proof_recording: bool,
    /// Enable instance synthesis.
    pub instance_synth: bool,
    /// Enable congr-lemma automation.
    pub congr_lemmas: bool,
}
impl MetaFeatures {
    /// All features enabled.
    pub fn all_enabled() -> Self {
        Self {
            discr_tree: true,
            whnf_cache: true,
            proof_recording: true,
            instance_synth: true,
            congr_lemmas: true,
        }
    }
    /// Minimal features (fast, less complete).
    pub fn minimal() -> Self {
        Self {
            discr_tree: false,
            whnf_cache: false,
            proof_recording: false,
            instance_synth: false,
            congr_lemmas: false,
        }
    }
    /// Whether at least one caching feature is enabled.
    pub fn any_caching(&self) -> bool {
        self.whnf_cache || self.proof_recording
    }
}
impl Default for MetaFeatures {
    fn default() -> Self {
        Self {
            discr_tree: true,
            whnf_cache: true,
            proof_recording: false,
            instance_synth: true,
            congr_lemmas: true,
        }
    }
}
/// A diagnostic reporter for Lib.
#[allow(dead_code)]
pub struct LibDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl LibDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        LibDiagnostics {
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
pub struct LibExtDiag1300 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl LibExtDiag1300 {
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
/// A state machine controller for MetaLib.
#[allow(dead_code)]
pub struct MetaLibStateMachine {
    pub state: MetaLibState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl MetaLibStateMachine {
    pub fn new() -> Self {
        MetaLibStateMachine {
            state: MetaLibState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: MetaLibState) -> bool {
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
        self.transition_to(MetaLibState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(MetaLibState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(MetaLibState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(MetaLibState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}

// ── Default implementations ───────────────────────────────────────────────────

impl Default for LibExtDiff1300 {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Clone + Default> Default for MetaLibExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MetaLibCounterMap {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LibExtConfig1300 {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LibDiff {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for LibConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for MetaLibStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
