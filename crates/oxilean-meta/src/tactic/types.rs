//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, VecDeque};

#[allow(dead_code)]
pub struct ModExtPass2700 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<ModExtResult2700>,
}
impl ModExtPass2700 {
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
    pub fn run(&mut self, input: &str) -> ModExtResult2700 {
        if !self.enabled {
            return ModExtResult2700::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            ModExtResult2700::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            ModExtResult2700::Ok(format!(
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
/// Summary of the current proof state for display.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct ProofStateSummary {
    /// Number of remaining goals.
    pub goal_count: usize,
    /// Descriptions of each goal's target type.
    pub goal_descriptions: Vec<String>,
    /// `true` if the proof is complete (no goals remain).
    pub is_complete: bool,
}
impl ProofStateSummary {
    /// Construct from raw fields.
    #[allow(dead_code)]
    pub fn new(goal_count: usize, goal_descriptions: Vec<String>) -> Self {
        let is_complete = goal_count == 0;
        Self {
            goal_count,
            goal_descriptions,
            is_complete,
        }
    }
}
/// A state machine controller for TacMod.
#[allow(dead_code)]
pub struct TacModStateMachine {
    pub state: TacModState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl TacModStateMachine {
    pub fn new() -> Self {
        TacModStateMachine {
            state: TacModState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: TacModState) -> bool {
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
        self.transition_to(TacModState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(TacModState::Paused)
    }

    pub fn complete(&mut self) -> bool {
        self.transition_to(TacModState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(TacModState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ModExtResult2700 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl ModExtResult2700 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, ModExtResult2700::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, ModExtResult2700::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, ModExtResult2700::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, ModExtResult2700::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let ModExtResult2700::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let ModExtResult2700::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            ModExtResult2700::Ok(_) => 1.0,
            ModExtResult2700::Err(_) => 0.0,
            ModExtResult2700::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            ModExtResult2700::Skipped => 0.5,
        }
    }
}
/// An extended map for TacMod keys to values.
#[allow(dead_code)]
pub struct TacModExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> TacModExtMap<V> {
    pub fn new() -> Self {
        TacModExtMap {
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
/// An extended utility type for TacMod.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacModExt2 {
    /// A numeric tag.
    pub tag: u32,
}
#[allow(dead_code)]
impl TacModExt2 {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { tag: 0 }
    }
}
/// An analysis pass for TacticMod.
#[allow(dead_code)]
pub struct TacticModAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticModResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticModAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticModAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticModResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticModResult::Err("empty input".to_string())
        } else {
            TacticModResult::Ok(format!("processed: {}", input))
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
#[allow(dead_code)]
pub struct ModExtDiff2700 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl ModExtDiff2700 {
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
/// A simple record describing when one tactic typically follows another.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TacticSequence {
    /// First tactic name.
    pub first: &'static str,
    /// Tactic that commonly follows.
    pub then: &'static str,
    /// Description of the common usage pattern.
    pub description: &'static str,
}
/// Analyse a tactic block and return statistics.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct TacticBlockStats {
    /// Total number of tactics.
    pub total: usize,
    /// Number of `sorry` tactics.
    pub sorry_count: usize,
    /// Number of `simp` / `simp only` tactics.
    pub simp_count: usize,
    /// Number of `rw` / `rfl` tactics.
    pub rw_count: usize,
    /// Number of `intro` / `intros` tactics.
    pub intro_count: usize,
    /// Number of `apply` tactics.
    pub apply_count: usize,
}
/// A builder pattern for TacMod.
#[allow(dead_code)]
pub struct TacModBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl TacModBuilder {
    pub fn new(name: &str) -> Self {
        TacModBuilder {
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
/// A configuration store for TacticMod.
#[allow(dead_code)]
pub struct TacticModConfig {
    pub values: std::collections::HashMap<String, TacticModConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticModConfig {
    pub fn new() -> Self {
        TacticModConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticModConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticModConfigValue> {
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
        self.set(key, TacticModConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticModConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticModConfigValue::Str(v.to_string()))
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
/// A typed slot for TacticMod configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticModConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticModConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticModConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticModConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticModConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticModConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticModConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticModConfigValue::Bool(_) => "bool",
            TacticModConfigValue::Int(_) => "int",
            TacticModConfigValue::Float(_) => "float",
            TacticModConfigValue::Str(_) => "str",
            TacticModConfigValue::List(_) => "list",
        }
    }
}
/// A work queue for TacMod items.
#[allow(dead_code)]
pub struct TacModWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl TacModWorkQueue {
    pub fn new(capacity: usize) -> Self {
        TacModWorkQueue {
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
/// A diagnostic reporter for TacticMod.
#[allow(dead_code)]
pub struct TacticModDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticModDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticModDiagnostics {
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
pub struct ModExtConfig2700 {
    pub(super) values: std::collections::HashMap<String, ModExtConfigVal2700>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl ModExtConfig2700 {
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
    pub fn set(&mut self, key: &str, value: ModExtConfigVal2700) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&ModExtConfigVal2700> {
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
        self.set(key, ModExtConfigVal2700::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, ModExtConfigVal2700::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, ModExtConfigVal2700::Str(v.to_string()))
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
/// A counter map for TacMod frequency analysis.
#[allow(dead_code)]
pub struct TacModCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl TacModCounterMap {
    pub fn new() -> Self {
        TacModCounterMap {
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
/// A timed record of a tactic invocation.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TacticTiming {
    /// Tactic name.
    pub name: String,
    /// Duration in microseconds.
    pub duration_us: u64,
    /// Whether the tactic succeeded.
    pub success: bool,
}
impl TacticTiming {
    /// Construct a timing record.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, duration_us: u64, success: bool) -> Self {
        Self {
            name: name.into(),
            duration_us,
            success,
        }
    }
    /// Duration in milliseconds.
    #[allow(dead_code)]
    pub fn duration_ms(&self) -> f64 {
        self.duration_us as f64 / 1000.0
    }
}
/// An extended utility type for TacMod.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacModExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl TacModExt {
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
/// Metadata entry for a tactic in the registry.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TacticEntry {
    /// Canonical name of the tactic (e.g. `"simp"`).
    pub name: &'static str,
    /// Short human-readable description.
    pub description: &'static str,
    /// Execution priority.
    pub priority: TacticPriority,
    /// `true` if the tactic may close the goal without leaving sub-goals.
    pub can_close: bool,
    /// `true` if the tactic may produce multiple sub-goals.
    pub can_split: bool,
}
impl TacticEntry {
    /// Construct a `TacticEntry`.
    #[allow(dead_code)]
    pub const fn new(
        name: &'static str,
        description: &'static str,
        priority: TacticPriority,
        can_close: bool,
        can_split: bool,
    ) -> Self {
        Self {
            name,
            description,
            priority,
            can_close,
            can_split,
        }
    }
}
/// Priority level for a registered tactic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum TacticPriority {
    /// Very cheap, safe tactics run first.
    Low = 0,
    /// Standard-priority tactics.
    Normal = 50,
    /// Expensive search tactics run last.
    High = 100,
}
/// A diff for TacticMod analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticModDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticModDiff {
    pub fn new() -> Self {
        TacticModDiff {
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
pub struct TacModExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl TacModExtUtil {
    pub fn new(key: &str) -> Self {
        TacModExtUtil {
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
#[allow(dead_code)]
pub struct ModExtPipeline2700 {
    pub name: String,
    pub passes: Vec<ModExtPass2700>,
    pub run_count: usize,
}
impl ModExtPipeline2700 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: ModExtPass2700) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<ModExtResult2700> {
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
/// A collection of tactic timing records.
#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct TacticProfile {
    pub(super) entries: Vec<TacticTiming>,
}
impl TacticProfile {
    /// Create an empty profile.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a tactic invocation.
    #[allow(dead_code)]
    pub fn record(&mut self, timing: TacticTiming) {
        self.entries.push(timing);
    }
    /// Total time spent in successful tactics.
    #[allow(dead_code)]
    pub fn total_success_us(&self) -> u64 {
        self.entries
            .iter()
            .filter(|e| e.success)
            .map(|e| e.duration_us)
            .sum()
    }
    /// Total time spent in failed tactics.
    #[allow(dead_code)]
    pub fn total_failure_us(&self) -> u64 {
        self.entries
            .iter()
            .filter(|e| !e.success)
            .map(|e| e.duration_us)
            .sum()
    }
    /// Number of recorded invocations.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    /// The slowest invocation.
    #[allow(dead_code)]
    pub fn slowest(&self) -> Option<&TacticTiming> {
        self.entries.iter().max_by_key(|e| e.duration_us)
    }
    /// Average duration in microseconds.
    #[allow(dead_code)]
    pub fn average_us(&self) -> f64 {
        if self.entries.is_empty() {
            0.0
        } else {
            self.entries.iter().map(|e| e.duration_us).sum::<u64>() as f64
                / self.entries.len() as f64
        }
    }
    /// Return timings for a specific tactic name.
    #[allow(dead_code)]
    pub fn for_tactic<'a>(&'a self, name: &str) -> Vec<&'a TacticTiming> {
        self.entries.iter().filter(|e| e.name == name).collect()
    }
}
/// A result type for TacticMod analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticModResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticModResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticModResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticModResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticModResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticModResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticModResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticModResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticModResult::Ok(_) => 1.0,
            TacticModResult::Err(_) => 0.0,
            TacticModResult::Skipped => 0.0,
            TacticModResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A state machine for TacMod.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacModState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl TacModState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, TacModState::Complete | TacModState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, TacModState::Initial | TacModState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, TacModState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            TacModState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
#[allow(dead_code)]
pub struct ModExtDiag2700 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl ModExtDiag2700 {
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
/// A sliding window accumulator for TacMod.
#[allow(dead_code)]
pub struct TacModWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl TacModWindow {
    pub fn new(capacity: usize) -> Self {
        TacModWindow {
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
#[derive(Debug, Clone)]
pub enum ModExtConfigVal2700 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl ModExtConfigVal2700 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let ModExtConfigVal2700::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let ModExtConfigVal2700::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let ModExtConfigVal2700::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let ModExtConfigVal2700::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let ModExtConfigVal2700::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            ModExtConfigVal2700::Bool(_) => "bool",
            ModExtConfigVal2700::Int(_) => "int",
            ModExtConfigVal2700::Float(_) => "float",
            ModExtConfigVal2700::Str(_) => "str",
            ModExtConfigVal2700::List(_) => "list",
        }
    }
}
/// A simple success/failure type for tactic combinators.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TacticOutcome {
    /// The tactic succeeded (goal was modified or closed).
    Success,
    /// The tactic failed (goal unchanged).
    Failure(String),
}
impl TacticOutcome {
    /// `true` if successful.
    #[allow(dead_code)]
    pub fn is_success(&self) -> bool {
        matches!(self, TacticOutcome::Success)
    }
    /// `true` if failed.
    #[allow(dead_code)]
    pub fn is_failure(&self) -> bool {
        matches!(self, TacticOutcome::Failure(_))
    }
    /// Return the failure message, or `None` if successful.
    #[allow(dead_code)]
    pub fn failure_msg(&self) -> Option<&str> {
        match self {
            TacticOutcome::Failure(msg) => Some(msg),
            _ => None,
        }
    }
    /// Chain: if `self` is `Failure`, try `other`.
    #[allow(dead_code)]
    pub fn or_else(self, other: TacticOutcome) -> TacticOutcome {
        match self {
            TacticOutcome::Failure(_) => other,
            _ => self,
        }
    }
}
/// A hint suggesting which tactic to try next.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct TacticHint {
    /// Suggested tactic name.
    pub tactic: &'static str,
    /// Explanation for why this is suggested.
    pub reason: String,
    /// Confidence score (0.0–1.0).
    pub confidence: f32,
}
impl TacticHint {
    /// Create a tactic hint.
    #[allow(dead_code)]
    pub fn new(tactic: &'static str, reason: impl Into<String>, confidence: f32) -> Self {
        Self {
            tactic,
            reason: reason.into(),
            confidence: confidence.clamp(0.0, 1.0),
        }
    }
}
/// A pipeline of TacticMod analysis passes.
#[allow(dead_code)]
pub struct TacticModPipeline {
    pub passes: Vec<TacticModAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticModPipeline {
    pub fn new(name: &str) -> Self {
        TacticModPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticModAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticModResult> {
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

// ── Default implementations ───────────────────────────────────────────────────

impl Default for TacModStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl<V: Clone + Default> Default for TacModExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ModExtDiff2700 {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TacticModConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ModExtConfig2700 {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TacModCounterMap {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TacticModDiff {
    fn default() -> Self {
        Self::new()
    }
}
