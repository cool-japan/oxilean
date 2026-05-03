//! Utility types: SimpModExt, SimpModExtUtil, SimpModExtMap, SimpModWindow,
//! SimpModBuilder, SimpModState, SimpModStateMachine, SimpModWorkQueue,
//! SimpModCounterMap, TacticSimpMod* analysis infrastructure,
//! ModExt800 infrastructure.

#![allow(dead_code)]
#![allow(missing_docs)]

// ============================================================
// Extended: SimpMod Utilities (Part 2)
// ============================================================

/// An extended utility type for SimpMod.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SimpModExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}

#[allow(dead_code)]
impl SimpModExt {
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

pub struct SimpModExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}

#[allow(dead_code)]
impl SimpModExtUtil {
    pub fn new(key: &str) -> Self {
        SimpModExtUtil {
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

/// An extended map for SimpMod keys to values.
#[allow(dead_code)]
pub struct SimpModExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}

#[allow(dead_code)]
impl<V: Clone + Default> SimpModExtMap<V> {
    pub fn new() -> Self {
        SimpModExtMap {
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

impl<V: Clone + Default> Default for SimpModExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

/// A sliding window accumulator for SimpMod.
#[allow(dead_code)]
pub struct SimpModWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}

#[allow(dead_code)]
impl SimpModWindow {
    pub fn new(capacity: usize) -> Self {
        SimpModWindow {
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

/// A builder pattern for SimpMod.
#[allow(dead_code)]
pub struct SimpModBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}

#[allow(dead_code)]
impl SimpModBuilder {
    pub fn new(name: &str) -> Self {
        SimpModBuilder {
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

/// A state machine for SimpMod.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SimpModState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}

#[allow(dead_code)]
impl SimpModState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, SimpModState::Complete | SimpModState::Failed(_))
    }

    pub fn can_run(&self) -> bool {
        matches!(self, SimpModState::Initial | SimpModState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, SimpModState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            SimpModState::Failed(s) => Some(s),
            _ => None,
        }
    }
}

/// A state machine controller for SimpMod.
#[allow(dead_code)]
pub struct SimpModStateMachine {
    pub state: SimpModState,
    pub transitions: usize,
    pub history: Vec<String>,
}

#[allow(dead_code)]
impl SimpModStateMachine {
    pub fn new() -> Self {
        SimpModStateMachine {
            state: SimpModState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }

    pub fn transition_to(&mut self, new_state: SimpModState) -> bool {
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
        self.transition_to(SimpModState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(SimpModState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(SimpModState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(SimpModState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}

impl Default for SimpModStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// A work queue for SimpMod items.
#[allow(dead_code)]
pub struct SimpModWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}

#[allow(dead_code)]
impl SimpModWorkQueue {
    pub fn new(capacity: usize) -> Self {
        SimpModWorkQueue {
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

/// A counter map for SimpMod frequency analysis.
#[allow(dead_code)]
pub struct SimpModCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}

#[allow(dead_code)]
impl SimpModCounterMap {
    pub fn new() -> Self {
        SimpModCounterMap {
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

impl Default for SimpModCounterMap {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Extended: TacticSimpMod Analysis Infrastructure
// ============================================================

/// A result type for TacticSimpMod analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticSimpModResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}

#[allow(dead_code)]
impl TacticSimpModResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticSimpModResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticSimpModResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticSimpModResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticSimpModResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticSimpModResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticSimpModResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticSimpModResult::Ok(_) => 1.0,
            TacticSimpModResult::Err(_) => 0.0,
            TacticSimpModResult::Skipped => 0.0,
            TacticSimpModResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}

/// An analysis pass for TacticSimpMod.
#[allow(dead_code)]
pub struct TacticSimpModAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticSimpModResult>,
    pub total_runs: usize,
}

#[allow(dead_code)]
impl TacticSimpModAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticSimpModAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }

    pub fn run(&mut self, input: &str) -> TacticSimpModResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticSimpModResult::Err("empty input".to_string())
        } else {
            TacticSimpModResult::Ok(format!("processed: {}", input))
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

/// A pipeline of TacticSimpMod analysis passes.
#[allow(dead_code)]
pub struct TacticSimpModPipeline {
    pub passes: Vec<TacticSimpModAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}

#[allow(dead_code)]
impl TacticSimpModPipeline {
    pub fn new(name: &str) -> Self {
        TacticSimpModPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }

    pub fn add_pass(&mut self, pass: TacticSimpModAnalysisPass) {
        self.passes.push(pass);
    }

    pub fn run_all(&mut self, input: &str) -> Vec<TacticSimpModResult> {
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

/// A diff for TacticSimpMod analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticSimpModDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

#[allow(dead_code)]
impl TacticSimpModDiff {
    pub fn new() -> Self {
        TacticSimpModDiff {
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

impl Default for TacticSimpModDiff {
    fn default() -> Self {
        Self::new()
    }
}

/// A typed slot for TacticSimpMod configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticSimpModConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

#[allow(dead_code)]
impl TacticSimpModConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticSimpModConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticSimpModConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticSimpModConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticSimpModConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticSimpModConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticSimpModConfigValue::Bool(_) => "bool",
            TacticSimpModConfigValue::Int(_) => "int",
            TacticSimpModConfigValue::Float(_) => "float",
            TacticSimpModConfigValue::Str(_) => "str",
            TacticSimpModConfigValue::List(_) => "list",
        }
    }
}

/// A configuration store for TacticSimpMod.
#[allow(dead_code)]
pub struct TacticSimpModConfig {
    pub values: std::collections::HashMap<String, TacticSimpModConfigValue>,
    pub read_only: bool,
}

#[allow(dead_code)]
impl TacticSimpModConfig {
    pub fn new() -> Self {
        TacticSimpModConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }

    pub fn set(&mut self, key: &str, value: TacticSimpModConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }

    pub fn get(&self, key: &str) -> Option<&TacticSimpModConfigValue> {
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
        self.set(key, TacticSimpModConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticSimpModConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticSimpModConfigValue::Str(v.to_string()))
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

impl Default for TacticSimpModConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// A diagnostic reporter for TacticSimpMod.
#[allow(dead_code)]
pub struct TacticSimpModDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

#[allow(dead_code)]
impl TacticSimpModDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticSimpModDiagnostics {
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

// --- Extended analysis infrastructure for mod (800-series) ---

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ModExtResult800 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}

impl ModExtResult800 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, ModExtResult800::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, ModExtResult800::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, ModExtResult800::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, ModExtResult800::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let ModExtResult800::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let ModExtResult800::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            ModExtResult800::Ok(_) => 1.0,
            ModExtResult800::Err(_) => 0.0,
            ModExtResult800::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            ModExtResult800::Skipped => 0.5,
        }
    }
}

#[allow(dead_code)]
pub struct ModExtPass800 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<ModExtResult800>,
}

impl ModExtPass800 {
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
    pub fn run(&mut self, input: &str) -> ModExtResult800 {
        if !self.enabled {
            return ModExtResult800::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            ModExtResult800::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            ModExtResult800::Ok(format!(
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
pub struct ModExtPipeline800 {
    pub name: String,
    pub passes: Vec<ModExtPass800>,
    pub run_count: usize,
}

impl ModExtPipeline800 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: ModExtPass800) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<ModExtResult800> {
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
pub struct ModExtDiff800 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

impl ModExtDiff800 {
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

impl Default for ModExtDiff800 {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ModExtConfigVal800 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

impl ModExtConfigVal800 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let ModExtConfigVal800::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let ModExtConfigVal800::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let ModExtConfigVal800::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let ModExtConfigVal800::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let ModExtConfigVal800::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            ModExtConfigVal800::Bool(_) => "bool",
            ModExtConfigVal800::Int(_) => "int",
            ModExtConfigVal800::Float(_) => "float",
            ModExtConfigVal800::Str(_) => "str",
            ModExtConfigVal800::List(_) => "list",
        }
    }
}

#[allow(dead_code)]
pub struct ModExtConfig800 {
    pub(super) values: std::collections::HashMap<String, ModExtConfigVal800>,
    pub(super) read_only: bool,
    pub(super) name: String,
}

impl ModExtConfig800 {
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
    pub fn set(&mut self, key: &str, value: ModExtConfigVal800) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&ModExtConfigVal800> {
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
        self.set(key, ModExtConfigVal800::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, ModExtConfigVal800::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, ModExtConfigVal800::Str(v.to_string()))
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

impl Default for ModExtConfig800 {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
pub struct ModExtDiag800 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

impl ModExtDiag800 {
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

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simpmod_ext_util_basic() {
        let mut u = SimpModExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }

    #[test]
    fn test_simpmod_ext_util_min_max() {
        let mut u = SimpModExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }

    #[test]
    fn test_simpmod_ext_util_flags() {
        let mut u = SimpModExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }

    #[test]
    fn test_simpmod_ext_util_pop() {
        let mut u = SimpModExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }

    #[test]
    fn test_simpmod_ext_map_basic() {
        let mut m: SimpModExtMap<i32> = SimpModExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }

    #[test]
    fn test_simpmod_ext_map_get_or_default() {
        let mut m: SimpModExtMap<i32> = SimpModExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }

    #[test]
    fn test_simpmod_ext_map_keys_sorted() {
        let mut m: SimpModExtMap<i32> = SimpModExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }

    #[test]
    fn test_simpmod_window_mean() {
        let mut w = SimpModWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_simpmod_window_evict() {
        let mut w = SimpModWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0); // evicts 10.0
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_simpmod_window_std_dev() {
        let mut w = SimpModWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }

    #[test]
    fn test_simpmod_builder_basic() {
        let b = SimpModBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }

    #[test]
    fn test_simpmod_builder_summary() {
        let b = SimpModBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }

    #[test]
    fn test_simpmod_state_machine_start() {
        let mut sm = SimpModStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }

    #[test]
    fn test_simpmod_state_machine_complete() {
        let mut sm = SimpModStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }

    #[test]
    fn test_simpmod_state_machine_fail() {
        let mut sm = SimpModStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }

    #[test]
    fn test_simpmod_state_machine_no_transition_after_terminal() {
        let mut sm = SimpModStateMachine::new();
        sm.complete();
        assert!(!sm.start()); // Already terminal
    }

    #[test]
    fn test_simpmod_work_queue_basic() {
        let mut wq = SimpModWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }

    #[test]
    fn test_simpmod_work_queue_capacity() {
        let mut wq = SimpModWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }

    #[test]
    fn test_simpmod_counter_map_basic() {
        let mut cm = SimpModCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }

    #[test]
    fn test_simpmod_counter_map_frequency() {
        let mut cm = SimpModCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_simpmod_counter_map_most_common() {
        let mut cm = SimpModCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }

    #[test]
    fn test_tacticsimpmod_result_ok() {
        let r = TacticSimpModResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tacticsimpmod_result_err() {
        let r = TacticSimpModResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_tacticsimpmod_result_partial() {
        let r = TacticSimpModResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_tacticsimpmod_result_skipped() {
        let r = TacticSimpModResult::Skipped;
        assert!(r.is_skipped());
    }

    #[test]
    fn test_tacticsimpmod_analysis_pass_run() {
        let mut p = TacticSimpModAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }

    #[test]
    fn test_tacticsimpmod_analysis_pass_empty_input() {
        let mut p = TacticSimpModAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }

    #[test]
    fn test_tacticsimpmod_analysis_pass_success_rate() {
        let mut p = TacticSimpModAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_tacticsimpmod_analysis_pass_disable() {
        let mut p = TacticSimpModAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }

    #[test]
    fn test_tacticsimpmod_pipeline_basic() {
        let mut pipeline = TacticSimpModPipeline::new("main_pipeline");
        pipeline.add_pass(TacticSimpModAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticSimpModAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_tacticsimpmod_pipeline_disabled_pass() {
        let mut pipeline = TacticSimpModPipeline::new("partial");
        let mut p = TacticSimpModAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticSimpModAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_tacticsimpmod_diff_basic() {
        let mut d = TacticSimpModDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }

    #[test]
    fn test_tacticsimpmod_diff_summary() {
        let mut d = TacticSimpModDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }

    #[test]
    fn test_tacticsimpmod_config_set_get() {
        let mut cfg = TacticSimpModConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }

    #[test]
    fn test_tacticsimpmod_config_read_only() {
        let mut cfg = TacticSimpModConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false)); // should fail
        assert_eq!(cfg.get_bool("key"), Some(true)); // unchanged
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }

    #[test]
    fn test_tacticsimpmod_config_remove() {
        let mut cfg = TacticSimpModConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }

    #[test]
    fn test_tacticsimpmod_diagnostics_basic() {
        let mut diag = TacticSimpModDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }

    #[test]
    fn test_tacticsimpmod_diagnostics_max_errors() {
        let mut diag = TacticSimpModDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3"); // e3 dropped
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }

    #[test]
    fn test_tacticsimpmod_diagnostics_clear() {
        let mut diag = TacticSimpModDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }

    #[test]
    fn test_tacticsimpmod_config_value_types() {
        let b = TacticSimpModConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);

        let i = TacticSimpModConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));

        let f = TacticSimpModConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);

        let s = TacticSimpModConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));

        let l = TacticSimpModConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }

    #[test]
    fn test_mod_ext_result_ok_800() {
        let r = ModExtResult800::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_mod_ext_result_err_800() {
        let r = ModExtResult800::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_mod_ext_result_partial_800() {
        let r = ModExtResult800::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_mod_ext_result_skipped_800() {
        let r = ModExtResult800::Skipped;
        assert!(r.is_skipped());
    }

    #[test]
    fn test_mod_ext_pass_run_800() {
        let mut p = ModExtPass800::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }

    #[test]
    fn test_mod_ext_pass_empty_800() {
        let mut p = ModExtPass800::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }

    #[test]
    fn test_mod_ext_pass_rate_800() {
        let mut p = ModExtPass800::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_mod_ext_pass_disable_800() {
        let mut p = ModExtPass800::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }

    #[test]
    fn test_mod_ext_pipeline_basic_800() {
        let mut pipeline = ModExtPipeline800::new("main_pipeline");
        pipeline.add_pass(ModExtPass800::new("pass1"));
        pipeline.add_pass(ModExtPass800::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_mod_ext_pipeline_disabled_800() {
        let mut pipeline = ModExtPipeline800::new("partial");
        let mut p = ModExtPass800::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(ModExtPass800::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_mod_ext_diff_basic_800() {
        let mut d = ModExtDiff800::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }

    #[test]
    fn test_mod_ext_config_set_get_800() {
        let mut cfg = ModExtConfig800::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }

    #[test]
    fn test_mod_ext_config_read_only_800() {
        let mut cfg = ModExtConfig800::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }

    #[test]
    fn test_mod_ext_config_remove_800() {
        let mut cfg = ModExtConfig800::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }

    #[test]
    fn test_mod_ext_diagnostics_basic_800() {
        let mut diag = ModExtDiag800::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }

    #[test]
    fn test_mod_ext_diagnostics_max_errors_800() {
        let mut diag = ModExtDiag800::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }

    #[test]
    fn test_mod_ext_diagnostics_clear_800() {
        let mut diag = ModExtDiag800::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }

    #[test]
    fn test_mod_ext_config_value_types_800() {
        let b = ModExtConfigVal800::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);

        let i = ModExtConfigVal800::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));

        let f = ModExtConfigVal800::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);

        let s = ModExtConfigVal800::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));

        let l = ModExtConfigVal800::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
