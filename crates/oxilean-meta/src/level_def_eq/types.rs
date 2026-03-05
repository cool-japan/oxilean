//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::basic::MetaContext;
use oxilean_kernel::{Level, LevelMVarId};
use std::collections::HashSet;

#[allow(dead_code)]
pub struct LevelDefEqExtConfig1200 {
    pub(super) values: std::collections::HashMap<String, LevelDefEqExtConfigVal1200>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl LevelDefEqExtConfig1200 {
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
    pub fn set(&mut self, key: &str, value: LevelDefEqExtConfigVal1200) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&LevelDefEqExtConfigVal1200> {
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
        self.set(key, LevelDefEqExtConfigVal1200::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, LevelDefEqExtConfigVal1200::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, LevelDefEqExtConfigVal1200::Str(v.to_string()))
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
/// Statistics for level unification.
#[derive(Debug, Default, Clone)]
pub struct LevelUnifStats {
    /// Attempts.
    pub attempts: u64,
    /// Successes.
    pub successes: u64,
    /// Failures.
    pub failures: u64,
}
impl LevelUnifStats {
    /// Create new.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record an attempt.
    pub fn record_attempt(&mut self, result: &LevelUnifResult) {
        self.attempts += 1;
        match result {
            LevelUnifResult::Success => self.successes += 1,
            LevelUnifResult::Failure => self.failures += 1,
            LevelUnifResult::Postponed => {}
        }
    }
    /// Success rate.
    pub fn success_rate(&self) -> f64 {
        if self.attempts == 0 {
            0.0
        } else {
            self.successes as f64 / self.attempts as f64
        }
    }
}
/// An extended utility type for LevelDefEq.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LevelDefEqExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl LevelDefEqExt {
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
/// A state machine controller for LevelDefEq.
#[allow(dead_code)]
pub struct LevelDefEqStateMachine {
    pub state: LevelDefEqState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl LevelDefEqStateMachine {
    pub fn new() -> Self {
        LevelDefEqStateMachine {
            state: LevelDefEqState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: LevelDefEqState) -> bool {
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
        self.transition_to(LevelDefEqState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(LevelDefEqState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(LevelDefEqState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(LevelDefEqState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
#[allow(dead_code)]
pub struct LevelDefEqExtPipeline1200 {
    pub name: String,
    pub passes: Vec<LevelDefEqExtPass1200>,
    pub run_count: usize,
}
impl LevelDefEqExtPipeline1200 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: LevelDefEqExtPass1200) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<LevelDefEqExtResult1200> {
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
/// An extended utility type for LevelDefEq.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LevelDefEqExt2 {
    /// A numeric tag.
    pub tag: u32,
}
#[allow(dead_code)]
impl LevelDefEqExt2 {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { tag: 0 }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LevelDefEqExtConfigVal1200 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl LevelDefEqExtConfigVal1200 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let LevelDefEqExtConfigVal1200::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let LevelDefEqExtConfigVal1200::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let LevelDefEqExtConfigVal1200::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let LevelDefEqExtConfigVal1200::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let LevelDefEqExtConfigVal1200::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            LevelDefEqExtConfigVal1200::Bool(_) => "bool",
            LevelDefEqExtConfigVal1200::Int(_) => "int",
            LevelDefEqExtConfigVal1200::Float(_) => "float",
            LevelDefEqExtConfigVal1200::Str(_) => "str",
            LevelDefEqExtConfigVal1200::List(_) => "list",
        }
    }
}
/// Level definitional equality checker with metavariable unification.
pub struct LevelDefEq {
    /// Maximum recursion depth.
    pub(super) max_depth: u32,
}
impl LevelDefEq {
    /// Create a new level def-eq checker.
    pub fn new() -> Self {
        Self { max_depth: 256 }
    }
    /// Check if two levels are definitionally equal, possibly assigning level mvars.
    pub fn is_level_def_eq(&mut self, l1: &Level, l2: &Level, ctx: &mut MetaContext) -> bool {
        self.is_level_def_eq_impl(l1, l2, ctx, 0)
    }
    /// Implementation with depth tracking.
    fn is_level_def_eq_impl(
        &mut self,
        l1: &Level,
        l2: &Level,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> bool {
        if depth > self.max_depth {
            return false;
        }
        let l1_inst = ctx.instantiate_level_mvars(l1);
        let l2_inst = ctx.instantiate_level_mvars(l2);
        if l1_inst == l2_inst {
            return true;
        }
        if oxilean_kernel::level::is_equivalent(&l1_inst, &l2_inst) {
            return true;
        }
        if self.try_level_mvar_assign(&l1_inst, &l2_inst, ctx) {
            return true;
        }
        let l1_norm = normalize_level(&l1_inst);
        let l2_norm = normalize_level(&l2_inst);
        if l1_norm == l2_norm {
            return true;
        }
        if oxilean_kernel::level::is_equivalent(&l1_norm, &l2_norm) {
            return true;
        }
        self.try_structural(&l1_norm, &l2_norm, ctx, depth)
    }
    /// Try to assign a level metavariable.
    fn try_level_mvar_assign(&mut self, l1: &Level, l2: &Level, ctx: &mut MetaContext) -> bool {
        if let Level::MVar(LevelMVarId(id)) = l1 {
            if ctx.get_level_assignment(*id).is_none() && !level_occurs_check(*id, l2) {
                ctx.assign_level_mvar(*id, l2.clone());
                return true;
            }
        }
        if let Level::MVar(LevelMVarId(id)) = l2 {
            if ctx.get_level_assignment(*id).is_none() && !level_occurs_check(*id, l1) {
                ctx.assign_level_mvar(*id, l1.clone());
                return true;
            }
        }
        if let (Level::Succ(inner1), Level::Succ(inner2)) = (l1, l2) {
            return self.is_level_def_eq_impl(inner1, inner2, ctx, 0);
        }
        if let Level::MVar(LevelMVarId(id)) = l1 {
            if ctx.get_level_assignment(*id).is_none() && !level_occurs_check(*id, l2) {
                ctx.assign_level_mvar(*id, l2.clone());
                return true;
            }
        }
        false
    }
    /// Try structural decomposition of levels.
    fn try_structural(
        &mut self,
        l1: &Level,
        l2: &Level,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> bool {
        match (l1, l2) {
            (Level::Succ(a), Level::Succ(b)) => self.is_level_def_eq_impl(a, b, ctx, depth + 1),
            (Level::Max(a1, b1), Level::Max(a2, b2)) => {
                self.is_level_def_eq_impl(a1, a2, ctx, depth + 1)
                    && self.is_level_def_eq_impl(b1, b2, ctx, depth + 1)
            }
            (Level::IMax(a1, b1), Level::IMax(a2, b2)) => {
                self.is_level_def_eq_impl(a1, a2, ctx, depth + 1)
                    && self.is_level_def_eq_impl(b1, b2, ctx, depth + 1)
            }
            _ => false,
        }
    }
    /// Check if level `l1` is ≤ level `l2`, possibly assigning level mvars.
    pub fn is_level_leq(&mut self, l1: &Level, l2: &Level, ctx: &mut MetaContext) -> bool {
        let max_level = Level::max(l1.clone(), l2.clone());
        self.is_level_def_eq(&max_level, l2, ctx)
    }
    /// Ensure a level is not zero (for universe constraints in Pi types).
    pub fn ensure_not_zero(&self, level: &Level, ctx: &MetaContext) -> bool {
        let inst = ctx.instantiate_level_mvars(level);
        let norm = normalize_level(&inst);
        !norm.is_zero()
    }
    /// Collect all unassigned level metavariables in a level.
    pub fn collect_level_mvars(&self, level: &Level, ctx: &MetaContext) -> HashSet<u64> {
        let mut result = HashSet::new();
        collect_level_mvars_impl(level, ctx, &mut result);
        result
    }
}
/// A counter map for LevelDefEq frequency analysis.
#[allow(dead_code)]
pub struct LevelDefEqCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl LevelDefEqCounterMap {
    pub fn new() -> Self {
        LevelDefEqCounterMap {
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
/// A pipeline of LevelDefEq analysis passes.
#[allow(dead_code)]
pub struct LevelDefEqPipeline {
    pub passes: Vec<LevelDefEqAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl LevelDefEqPipeline {
    pub fn new(name: &str) -> Self {
        LevelDefEqPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: LevelDefEqAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<LevelDefEqResult> {
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
/// A diff for LevelDefEq analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LevelDefEqDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl LevelDefEqDiff {
    pub fn new() -> Self {
        LevelDefEqDiff {
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
/// A typed slot for LevelDefEq configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LevelDefEqConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl LevelDefEqConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            LevelDefEqConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            LevelDefEqConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            LevelDefEqConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            LevelDefEqConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            LevelDefEqConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            LevelDefEqConfigValue::Bool(_) => "bool",
            LevelDefEqConfigValue::Int(_) => "int",
            LevelDefEqConfigValue::Float(_) => "float",
            LevelDefEqConfigValue::Str(_) => "str",
            LevelDefEqConfigValue::List(_) => "list",
        }
    }
}
/// A system of level constraints.
#[derive(Debug, Default, Clone)]
pub struct LevelConstraintSystem {
    pub(super) constraints: Vec<LevelConstraint>,
    pub(super) is_contradictory: bool,
}
impl LevelConstraintSystem {
    /// Create empty.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a constraint.
    pub fn add(&mut self, constraint: LevelConstraint) {
        if let LevelConstraint::Lt(l1, l2) = &constraint {
            if l1 == l2 {
                self.is_contradictory = true;
            }
        }
        self.constraints.push(constraint);
    }
    /// Add `l1 ≤ l2`.
    pub fn add_leq(&mut self, l1: Level, l2: Level) {
        self.add(LevelConstraint::Leq(l1, l2));
    }
    /// Add `l1 = l2`.
    pub fn add_eq(&mut self, l1: Level, l2: Level) {
        self.add(LevelConstraint::Eq(l1, l2));
    }
    /// Add `l1 < l2`.
    pub fn add_lt(&mut self, l1: Level, l2: Level) {
        self.add(LevelConstraint::Lt(l1, l2));
    }
    /// Check satisfiability.
    pub fn is_satisfiable(&self) -> bool {
        !self.is_contradictory
    }
    /// Get all constraints.
    pub fn constraints(&self) -> &[LevelConstraint] {
        &self.constraints
    }
    /// Number of constraints.
    pub fn len(&self) -> usize {
        self.constraints.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
    /// Clear.
    pub fn clear(&mut self) {
        self.constraints.clear();
        self.is_contradictory = false;
    }
}
/// A sliding window accumulator for LevelDefEq.
#[allow(dead_code)]
pub struct LevelDefEqWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl LevelDefEqWindow {
    pub fn new(capacity: usize) -> Self {
        LevelDefEqWindow {
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
/// A builder pattern for LevelDefEq.
#[allow(dead_code)]
pub struct LevelDefEqBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl LevelDefEqBuilder {
    pub fn new(name: &str) -> Self {
        LevelDefEqBuilder {
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
/// A result type for LevelDefEq analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LevelDefEqResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl LevelDefEqResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, LevelDefEqResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, LevelDefEqResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, LevelDefEqResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, LevelDefEqResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            LevelDefEqResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            LevelDefEqResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            LevelDefEqResult::Ok(_) => 1.0,
            LevelDefEqResult::Err(_) => 0.0,
            LevelDefEqResult::Skipped => 0.0,
            LevelDefEqResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A state machine for LevelDefEq.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LevelDefEqState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl LevelDefEqState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, LevelDefEqState::Complete | LevelDefEqState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, LevelDefEqState::Initial | LevelDefEqState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, LevelDefEqState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            LevelDefEqState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// Represent a level constraint system for incremental solving.
#[allow(dead_code)]
pub struct LevelConstraintSolver {
    /// Equality constraints.
    pub(super) equalities: Vec<(Level, Level)>,
    /// Less-or-equal constraints.
    pub(super) leqs: Vec<(Level, Level)>,
    /// Metavariable assignments found so far.
    pub assignments: std::collections::HashMap<LevelMVarId, Level>,
}
#[allow(dead_code)]
impl LevelConstraintSolver {
    /// Create an empty solver.
    pub fn new() -> Self {
        LevelConstraintSolver {
            equalities: Vec::new(),
            leqs: Vec::new(),
            assignments: std::collections::HashMap::new(),
        }
    }
    /// Add an equality constraint.
    pub fn add_eq(&mut self, l1: Level, l2: Level) {
        self.equalities.push((l1, l2));
    }
    /// Add a less-or-equal constraint.
    pub fn add_leq(&mut self, l1: Level, l2: Level) {
        self.leqs.push((l1, l2));
    }
    /// Attempt a simple propagation step and return `true` if progress was made.
    pub fn propagate_once(&mut self) -> bool {
        let mut changed = false;
        let eqs = self.equalities.clone();
        for (l, r) in eqs {
            let l_inst = self.instantiate(&l);
            let r_inst = self.instantiate(&r);
            if let Level::MVar(id) = &l_inst {
                if !r_inst.has_mvar() {
                    self.assignments.insert(*id, r_inst);
                    changed = true;
                    continue;
                }
            }
            if let Level::MVar(id) = &r_inst {
                if !l_inst.has_mvar() {
                    self.assignments.insert(*id, l_inst);
                    changed = true;
                }
            }
        }
        changed
    }
    /// Propagate until fixpoint.
    pub fn propagate(&mut self) {
        while self.propagate_once() {}
    }
    /// Instantiate metavariables in a level using current assignments.
    pub fn instantiate(&self, l: &Level) -> Level {
        oxilean_kernel::level::instantiate_level_mvars(l, &|id| self.assignments.get(&id).cloned())
    }
    /// Check whether all equality constraints are satisfied.
    pub fn check_equalities(&self) -> bool {
        self.equalities.iter().all(|(l, r)| {
            let l_inst = self.instantiate(l);
            let r_inst = self.instantiate(r);
            oxilean_kernel::level::is_equivalent(&l_inst, &r_inst)
        })
    }
    /// Check whether all leq constraints are satisfied.
    pub fn check_leqs(&self) -> bool {
        self.leqs.iter().all(|(l, r)| {
            let l_inst = self.instantiate(l);
            let r_inst = self.instantiate(r);
            oxilean_kernel::level::is_geq(&r_inst, &l_inst)
        })
    }
}
/// An analysis pass for LevelDefEq.
#[allow(dead_code)]
pub struct LevelDefEqAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<LevelDefEqResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl LevelDefEqAnalysisPass {
    pub fn new(name: &str) -> Self {
        LevelDefEqAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> LevelDefEqResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            LevelDefEqResult::Err("empty input".to_string())
        } else {
            LevelDefEqResult::Ok(format!("processed: {}", input))
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
/// A configuration store for LevelDefEq.
#[allow(dead_code)]
pub struct LevelDefEqConfig {
    pub values: std::collections::HashMap<String, LevelDefEqConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl LevelDefEqConfig {
    pub fn new() -> Self {
        LevelDefEqConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: LevelDefEqConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&LevelDefEqConfigValue> {
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
        self.set(key, LevelDefEqConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, LevelDefEqConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, LevelDefEqConfigValue::Str(v.to_string()))
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
pub struct LevelDefEqExtDiag1200 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl LevelDefEqExtDiag1200 {
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
/// Unification result for levels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LevelUnifResult {
    /// Success.
    Success,
    /// Failure.
    Failure,
    /// Postponed.
    Postponed,
}
/// An extended map for LevelDefEq keys to values.
#[allow(dead_code)]
pub struct LevelDefEqExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> LevelDefEqExtMap<V> {
    pub fn new() -> Self {
        LevelDefEqExtMap {
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
pub struct LevelDefEqExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl LevelDefEqExtUtil {
    pub fn new(key: &str) -> Self {
        LevelDefEqExtUtil {
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
/// A work queue for LevelDefEq items.
#[allow(dead_code)]
pub struct LevelDefEqWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl LevelDefEqWorkQueue {
    pub fn new(capacity: usize) -> Self {
        LevelDefEqWorkQueue {
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
/// A diagnostic reporter for LevelDefEq.
#[allow(dead_code)]
pub struct LevelDefEqDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl LevelDefEqDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        LevelDefEqDiagnostics {
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
pub struct LevelDefEqExtPass1200 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<LevelDefEqExtResult1200>,
}
impl LevelDefEqExtPass1200 {
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
    pub fn run(&mut self, input: &str) -> LevelDefEqExtResult1200 {
        if !self.enabled {
            return LevelDefEqExtResult1200::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            LevelDefEqExtResult1200::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            LevelDefEqExtResult1200::Ok(format!(
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
pub enum LevelDefEqExtResult1200 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl LevelDefEqExtResult1200 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, LevelDefEqExtResult1200::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, LevelDefEqExtResult1200::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, LevelDefEqExtResult1200::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, LevelDefEqExtResult1200::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let LevelDefEqExtResult1200::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let LevelDefEqExtResult1200::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            LevelDefEqExtResult1200::Ok(_) => 1.0,
            LevelDefEqExtResult1200::Err(_) => 0.0,
            LevelDefEqExtResult1200::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            LevelDefEqExtResult1200::Skipped => 0.5,
        }
    }
}
#[allow(dead_code)]
pub struct LevelDefEqExtDiff1200 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl LevelDefEqExtDiff1200 {
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
/// A level constraint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LevelConstraint {
    /// l1 ≤ l2
    Leq(Level, Level),
    /// l1 = l2
    Eq(Level, Level),
    /// l1 < l2
    Lt(Level, Level),
}
impl LevelConstraint {
    /// Get the two levels.
    pub fn levels(&self) -> (&Level, &Level) {
        match self {
            LevelConstraint::Leq(l1, l2)
            | LevelConstraint::Eq(l1, l2)
            | LevelConstraint::Lt(l1, l2) => (l1, l2),
        }
    }
}
