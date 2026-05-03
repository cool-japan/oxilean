//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use super::types::{default_simp_lemmas, SimpConfig, SimpLemma, SimpResult, SimpTheorems};
use oxilean_kernel::{Expr, Name};

use std::collections::{HashMap, VecDeque};

/// A simple cache mapping name strings to lemma counts.
#[derive(Clone, Debug, Default)]
pub struct SimpLemmaCache {
    /// lookup_count: how many times each lemma was looked up
    pub lookups: std::collections::HashMap<String, u64>,
}
impl SimpLemmaCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a lemma lookup.
    pub fn record_lookup(&mut self, name: &oxilean_kernel::Name) {
        *self.lookups.entry(name.to_string()).or_insert(0) += 1;
    }
    /// Total number of lookups.
    pub fn total_lookups(&self) -> u64 {
        self.lookups.values().sum()
    }
    /// Most-looked-up lemma name.
    pub fn hottest_lemma(&self) -> Option<&str> {
        self.lookups
            .iter()
            .max_by_key(|(_, c)| *c)
            .map(|(k, _)| k.as_str())
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
/// Mutable state for a single simp invocation.
pub struct SimpContext<'a> {
    /// Configuration controlling the simp run.
    pub config: &'a SimpConfig,
    /// The active simp lemma database.
    pub theorems: &'a SimpTheorems,
    /// Accumulated statistics.
    pub stats: SimpStats,
    /// Remaining step budget.
    pub budget: u32,
    /// Additional locally-scoped lemmas.
    pub local_lemmas: Vec<SimpLemma>,
    /// Names of lemmas to exclude.
    pub excluded: Vec<Name>,
}
impl<'a> SimpContext<'a> {
    /// Create a new simp context.
    pub fn new(config: &'a SimpConfig, theorems: &'a SimpTheorems) -> Self {
        Self {
            budget: config.max_steps,
            config,
            theorems,
            stats: SimpStats::new(),
            local_lemmas: Vec::new(),
            excluded: Vec::new(),
        }
    }
    /// Add a local lemma for this invocation.
    pub fn add_local_lemma(&mut self, lemma: SimpLemma) {
        self.local_lemmas.push(lemma);
    }
    /// Exclude a lemma by name.
    pub fn exclude(&mut self, name: Name) {
        self.excluded.push(name);
    }
    /// Check whether the given lemma is excluded.
    pub fn is_excluded(&self, name: &Name) -> bool {
        self.excluded.contains(name)
    }
    /// Consume one step from the budget.
    ///
    /// Returns false when the budget is exhausted.
    pub fn consume_budget(&mut self) -> bool {
        if self.budget == 0 {
            self.stats.budget_exhausted = true;
            return false;
        }
        self.budget -= 1;
        true
    }
    /// Whether the budget is still active.
    pub fn has_budget(&self) -> bool {
        self.budget > 0
    }
}
/// A filter for selecting which simp lemmas to apply.
#[derive(Clone, Debug)]
pub struct SimpLemmaFilter {
    /// If Some, only apply lemmas whose names start with this prefix.
    pub name_prefix: Option<String>,
    /// If Some, only apply lemmas with priority >= this value.
    pub min_priority: Option<u32>,
    /// If true, exclude conditional lemmas.
    pub exclude_conditional: bool,
}
impl SimpLemmaFilter {
    /// Create a filter that passes all lemmas.
    pub fn all() -> Self {
        Self {
            name_prefix: None,
            min_priority: None,
            exclude_conditional: false,
        }
    }
    /// Create a filter for lemmas with a given name prefix.
    pub fn with_prefix(prefix: &str) -> Self {
        Self {
            name_prefix: Some(prefix.to_string()),
            ..Self::all()
        }
    }
    /// Test whether a lemma name passes the filter.
    pub fn passes(&self, name: &Name) -> bool {
        if let Some(prefix) = &self.name_prefix {
            if !name.to_string().starts_with(prefix.to_string().as_str()) {
                return false;
            }
        }
        true
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
/// Statistics collected during a single simp invocation.
#[derive(Clone, Debug, Default)]
pub struct SimpStats {
    /// Number of lemmas tried.
    pub lemmas_tried: u64,
    /// Number of successful rewrites.
    pub rewrites_applied: u64,
    /// Number of beta-reduction steps.
    pub beta_steps: u64,
    /// Number of eta-reduction steps.
    pub eta_steps: u64,
    /// Number of iota-reduction steps.
    pub iota_steps: u64,
    /// Number of zeta-reduction steps.
    pub zeta_steps: u64,
    /// Number of congruence closure applications.
    pub congr_steps: u64,
    /// Number of side goals generated and discharged.
    pub side_goals_discharged: u64,
    /// Number of side goals that failed to discharge.
    pub side_goals_failed: u64,
    /// Total subexpressions visited.
    pub exprs_visited: u64,
    /// Whether the budget was exhausted.
    pub budget_exhausted: bool,
}
impl SimpStats {
    /// Create zero-initialized stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Total reduction steps (not counting congruence/lemmas).
    pub fn total_reduction_steps(&self) -> u64 {
        self.beta_steps + self.eta_steps + self.iota_steps + self.zeta_steps
    }
    /// Whether any progress was made.
    pub fn any_progress(&self) -> bool {
        self.rewrites_applied > 0 || self.total_reduction_steps() > 0
    }
    /// Whether all side goals were discharged.
    pub fn all_side_goals_discharged(&self) -> bool {
        self.side_goals_failed == 0
    }
    /// Add another stats record into this one.
    pub fn merge(&mut self, other: &SimpStats) {
        self.lemmas_tried += other.lemmas_tried;
        self.rewrites_applied += other.rewrites_applied;
        self.beta_steps += other.beta_steps;
        self.eta_steps += other.eta_steps;
        self.iota_steps += other.iota_steps;
        self.zeta_steps += other.zeta_steps;
        self.congr_steps += other.congr_steps;
        self.side_goals_discharged += other.side_goals_discharged;
        self.side_goals_failed += other.side_goals_failed;
        self.exprs_visited += other.exprs_visited;
        self.budget_exhausted |= other.budget_exhausted;
    }
}
/// A named, versioned database of simp lemmas.
#[derive(Clone, Debug)]
pub struct SimpLemmaDatabase {
    /// Database label (e.g., "default", "ring").
    pub label: String,
    /// Version counter, incremented on each mutation.
    pub version: u64,
    /// Underlying theorem storage.
    pub theorems: SimpTheorems,
}
impl SimpLemmaDatabase {
    /// Create a new empty database with a label.
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            version: 0,
            theorems: SimpTheorems::new(),
        }
    }
    /// Create the default simp database.
    pub fn default_db() -> Self {
        Self {
            label: "default".to_string(),
            version: 1,
            theorems: default_simp_lemmas(),
        }
    }
    /// Add a lemma, bumping the version.
    pub fn add(&mut self, lemma: SimpLemma) {
        self.theorems.add_lemma(lemma);
        self.version += 1;
    }
    /// Remove a lemma by name, bumping the version.
    pub fn remove(&mut self, name: &Name) {
        self.theorems.remove_lemma(name);
        self.version += 1;
    }
    /// Number of lemmas in the database.
    pub fn len(&self) -> usize {
        self.theorems.num_lemmas()
    }
    /// Whether the database is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// The result of computing the simp normal form of an expression.
#[derive(Clone, Debug)]
pub struct SimpNormalForm {
    /// The normalized expression.
    pub expr: oxilean_kernel::Expr,
    /// Whether the expression was changed.
    pub changed: bool,
    /// Lemmas used.
    pub lemmas: Vec<Name>,
}
impl SimpNormalForm {
    /// Create an unchanged normal form.
    pub fn unchanged(expr: oxilean_kernel::Expr) -> Self {
        Self {
            expr,
            changed: false,
            lemmas: Vec::new(),
        }
    }
    /// Create a changed normal form.
    pub fn changed(expr: oxilean_kernel::Expr, lemmas: Vec<Name>) -> Self {
        Self {
            expr,
            changed: true,
            lemmas,
        }
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
/// Determines the order in which simp lemmas are tried.
#[derive(Clone, Debug, Default)]
pub struct SimpScheduler {
    /// Lemma names ordered by priority (highest first).
    pub(super) ordered: Vec<(u32, oxilean_kernel::Name)>,
}
impl SimpScheduler {
    /// Create an empty scheduler.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a lemma with its priority.
    pub fn register(&mut self, name: oxilean_kernel::Name, priority: u32) {
        let pos = self.ordered.partition_point(|(p, _)| *p >= priority);
        self.ordered.insert(pos, (priority, name));
    }
    /// Deregister a lemma.
    pub fn deregister(&mut self, name: &oxilean_kernel::Name) {
        self.ordered.retain(|(_, n)| n != name);
    }
    /// Iterate lemma names in priority order.
    pub fn iter_by_priority(&self) -> impl Iterator<Item = &oxilean_kernel::Name> {
        self.ordered.iter().map(|(_, n)| n)
    }
    /// Number of registered lemmas.
    pub fn len(&self) -> usize {
        self.ordered.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.ordered.is_empty()
    }
    /// Get the highest-priority lemma name.
    pub fn top(&self) -> Option<&oxilean_kernel::Name> {
        self.ordered.first().map(|(_, n)| n)
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
/// Tracks work budget for a simp invocation.
#[derive(Clone, Debug)]
pub struct SimpBudget {
    /// Total step budget.
    pub(super) total: u32,
    /// Remaining steps.
    pub(super) remaining: u32,
    /// Whether the budget was exhausted.
    pub(super) exhausted: bool,
}
impl SimpBudget {
    /// Create a budget with `total` steps.
    pub fn new(total: u32) -> Self {
        Self {
            total,
            remaining: total,
            exhausted: false,
        }
    }
    /// Consume `n` steps. Returns `false` if budget is exhausted.
    pub fn consume(&mut self, n: u32) -> bool {
        if self.remaining < n {
            self.remaining = 0;
            self.exhausted = true;
            false
        } else {
            self.remaining -= n;
            true
        }
    }
    /// Check if any budget remains.
    pub fn has_budget(&self) -> bool {
        self.remaining > 0
    }
    /// Whether budget was exhausted.
    pub fn is_exhausted(&self) -> bool {
        self.exhausted
    }
    /// Remaining steps.
    pub fn remaining(&self) -> u32 {
        self.remaining
    }
    /// Total steps.
    pub fn total(&self) -> u32 {
        self.total
    }
    /// Used steps.
    pub fn used(&self) -> u32 {
        self.total - self.remaining
    }
    /// Fraction used (0.0–1.0).
    pub fn fraction_used(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            self.used() as f32 / self.total as f32
        }
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
/// A report produced after a simp run.
#[derive(Clone, Debug)]
pub struct SimpReport {
    /// Whether the goal was fully proved.
    pub proved: bool,
    /// Whether the expression changed.
    pub simplified: bool,
    /// The resulting expression (after simplification).
    pub result: Option<Expr>,
    /// Statistics from the run.
    pub stats: SimpStats,
    /// Names of lemmas that fired.
    pub lemmas_used: Vec<Name>,
}
impl SimpReport {
    /// Create a report for an unchanged expression.
    pub fn unchanged(expr: Expr) -> Self {
        Self {
            proved: false,
            simplified: false,
            result: Some(expr),
            stats: SimpStats::new(),
            lemmas_used: Vec::new(),
        }
    }
    /// Create a report for a simplified expression.
    pub fn simplified(result: Expr, stats: SimpStats) -> Self {
        Self {
            proved: false,
            simplified: true,
            result: Some(result),
            stats,
            lemmas_used: Vec::new(),
        }
    }
    /// Create a report for a proved goal.
    pub fn proved(stats: SimpStats) -> Self {
        Self {
            proved: true,
            simplified: true,
            result: None,
            stats,
            lemmas_used: Vec::new(),
        }
    }
    /// Add a lemma to the "used" list.
    pub fn record_lemma(&mut self, name: Name) {
        if !self.lemmas_used.contains(&name) {
            self.lemmas_used.push(name);
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
/// Records the sequence of simp lemmas that fired during a simp run.
#[derive(Clone, Debug, Default)]
pub struct SimpTrace {
    /// Lemma names in the order they fired.
    pub fired: Vec<Name>,
    /// Whether tracing is enabled.
    pub enabled: bool,
}
impl SimpTrace {
    /// Create an enabled trace.
    pub fn enabled() -> Self {
        Self {
            fired: Vec::new(),
            enabled: true,
        }
    }
    /// Create a disabled trace.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a lemma firing.
    pub fn record(&mut self, name: Name) {
        if self.enabled {
            self.fired.push(name);
        }
    }
    /// Number of lemma firings recorded.
    pub fn len(&self) -> usize {
        self.fired.len()
    }
    /// Whether no lemmas were recorded.
    pub fn is_empty(&self) -> bool {
        self.fired.is_empty()
    }
    /// Clear the trace.
    pub fn clear(&mut self) {
        self.fired.clear();
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
