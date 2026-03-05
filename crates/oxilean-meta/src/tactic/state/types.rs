//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::{MVarId, MetaContext};
use oxilean_kernel::{Expr, Name};

use super::functions::TacticResult;

/// A work queue for TacState items.
#[allow(dead_code)]
pub struct TacStateWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl TacStateWorkQueue {
    pub fn new(capacity: usize) -> Self {
        TacStateWorkQueue {
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
/// An extended utility type for TacState.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacStateExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl TacStateExt {
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
/// The state of an interactive proof.
///
/// Manages a list of goals (metavariables) and provides
/// operations for focusing, unfocusing, and querying goals.
pub struct TacticState {
    /// All goals in order.
    pub(super) goals: Vec<MVarId>,
    /// Index of the focused goal (0 = first).
    pub(super) focus_idx: usize,
    /// Goal tags for user display.
    pub(super) tags: Vec<Option<String>>,
    /// Saved states for backtracking.
    pub(super) saved: Vec<TacticStateSnapshot>,
}
impl TacticState {
    /// Create a new tactic state with initial goals.
    pub fn new(goals: Vec<MVarId>) -> Self {
        let len = goals.len();
        Self {
            goals,
            focus_idx: 0,
            tags: vec![None; len],
            saved: Vec::new(),
        }
    }
    /// Create a state with a single goal.
    pub fn single(goal: MVarId) -> Self {
        Self::new(vec![goal])
    }
    /// Check if all goals are solved.
    pub fn is_done(&self) -> bool {
        self.goals.is_empty()
    }
    /// Get the number of remaining goals.
    pub fn num_goals(&self) -> usize {
        self.goals.len()
    }
    /// Get the focused goal's MVarId.
    pub fn current_goal(&self) -> TacticResult<MVarId> {
        self.goals
            .get(self.focus_idx)
            .copied()
            .ok_or(TacticError::NoGoals)
    }
    /// Get all goal IDs.
    pub fn all_goals(&self) -> &[MVarId] {
        &self.goals
    }
    /// Get a view of the focused goal.
    pub fn goal_view(&self, ctx: &MetaContext) -> TacticResult<GoalView> {
        let mvar_id = self.current_goal()?;
        let target = ctx
            .get_mvar_type(mvar_id)
            .cloned()
            .ok_or_else(|| TacticError::Internal("goal mvar has no type".to_string()))?;
        let target = ctx.instantiate_mvars(&target);
        let hyps = ctx.get_local_hyps();
        let tag = self.tags.get(self.focus_idx).cloned().unwrap_or(None);
        Ok(GoalView {
            mvar_id,
            target,
            hyps,
            tag,
        })
    }
    /// Close the current goal by assigning the metavariable.
    pub fn close_goal(&mut self, proof: Expr, ctx: &mut MetaContext) -> TacticResult<()> {
        let mvar_id = self.current_goal()?;
        ctx.assign_mvar(mvar_id, proof);
        self.goals.remove(self.focus_idx);
        if !self.tags.is_empty() && self.focus_idx < self.tags.len() {
            self.tags.remove(self.focus_idx);
        }
        if self.focus_idx >= self.goals.len() && !self.goals.is_empty() {
            self.focus_idx = self.goals.len() - 1;
        }
        Ok(())
    }
    /// Replace the current goal with new sub-goals.
    pub fn replace_goal(&mut self, new_goals: Vec<MVarId>) {
        if self.goals.is_empty() {
            return;
        }
        let idx = self.focus_idx;
        self.goals.remove(idx);
        if idx < self.tags.len() {
            self.tags.remove(idx);
        }
        for (i, g) in new_goals.iter().enumerate() {
            self.goals.insert(idx + i, *g);
            self.tags.insert(idx + i, None);
        }
    }
    /// Focus on a specific goal by index.
    pub fn focus(&mut self, idx: usize) -> TacticResult<()> {
        if idx >= self.goals.len() {
            return Err(TacticError::Failed(format!(
                "goal index {} out of range (have {} goals)",
                idx,
                self.goals.len()
            )));
        }
        self.focus_idx = idx;
        Ok(())
    }
    /// Rotate goals: move the first goal to the end.
    pub fn rotate(&mut self) {
        if self.goals.len() > 1 {
            let g = self.goals.remove(0);
            self.goals.push(g);
            let t = self.tags.remove(0);
            self.tags.push(t);
        }
    }
    /// Set a tag on the current goal.
    pub fn tag_goal(&mut self, tag: String) -> TacticResult<()> {
        let _ = self.current_goal()?;
        if self.focus_idx < self.tags.len() {
            self.tags[self.focus_idx] = Some(tag);
        }
        Ok(())
    }
    /// Push new goals at the end.
    pub fn push_goals(&mut self, goals: Vec<MVarId>) {
        for g in goals {
            self.goals.push(g);
            self.tags.push(None);
        }
    }
    /// Save the current tactic state for backtracking.
    pub fn save(&mut self) {
        self.saved.push(TacticStateSnapshot {
            goals: self.goals.clone(),
            focus_idx: self.focus_idx,
            tags: self.tags.clone(),
        });
    }
    /// Restore the most recently saved state.
    pub fn restore(&mut self) -> TacticResult<()> {
        let snap = self
            .saved
            .pop()
            .ok_or_else(|| TacticError::Internal("no saved state".to_string()))?;
        self.goals = snap.goals;
        self.focus_idx = snap.focus_idx;
        self.tags = snap.tags;
        Ok(())
    }
    /// Get the focus index.
    pub fn focus_idx(&self) -> usize {
        self.focus_idx
    }
}
/// A typed slot for TacticState configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticStateConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticStateConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticStateConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticStateConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticStateConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticStateConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticStateConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticStateConfigValue::Bool(_) => "bool",
            TacticStateConfigValue::Int(_) => "int",
            TacticStateConfigValue::Float(_) => "float",
            TacticStateConfigValue::Str(_) => "str",
            TacticStateConfigValue::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct StateExtDiff1800 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl StateExtDiff1800 {
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
/// A named checkpoint in a tactic proof.
///
/// Allows the user to name specific states in a proof for later review
/// or rollback.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct TacticCheckpoint {
    /// Name of this checkpoint.
    pub name: String,
    /// The goal list at the time of the checkpoint.
    pub goals: Vec<MVarId>,
    /// Focus index at the time of the checkpoint.
    pub focus_idx: usize,
}
impl TacticCheckpoint {
    /// Create a new checkpoint from a tactic state.
    #[allow(dead_code)]
    pub fn from_state(name: &str, state: &TacticState) -> Self {
        Self {
            name: name.to_string(),
            goals: state.all_goals().to_vec(),
            focus_idx: state.focus_idx(),
        }
    }
    /// Number of goals at this checkpoint.
    #[allow(dead_code)]
    pub fn num_goals(&self) -> usize {
        self.goals.len()
    }
}
/// A pipeline of TacticState analysis passes.
#[allow(dead_code)]
pub struct TacticStatePipeline {
    pub passes: Vec<TacticStateAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticStatePipeline {
    pub fn new(name: &str) -> Self {
        TacticStatePipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticStateAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticStateResult> {
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
/// A diff for TacticState analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticStateDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticStateDiff {
    pub fn new() -> Self {
        TacticStateDiff {
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
/// A snapshot of tactic statistics collected during a proof attempt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacticStats {
    /// Number of tactics applied so far.
    pub tactic_count: usize,
    /// Number of goals that were closed.
    pub goals_closed: usize,
    /// Number of goals that were opened (e.g., by `apply`).
    pub goals_opened: usize,
    /// Number of times the state was saved.
    pub saves: usize,
    /// Number of times the state was restored.
    pub restores: usize,
}
impl TacticStats {
    /// Create a new zeroed stats record.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record that a tactic was applied.
    #[allow(dead_code)]
    pub fn record_tactic(&mut self) {
        self.tactic_count += 1;
    }
    /// Record that `n` goals were closed by the last tactic.
    #[allow(dead_code)]
    pub fn record_goals_closed(&mut self, n: usize) {
        self.goals_closed += n;
    }
    /// Record that `n` goals were opened by the last tactic.
    #[allow(dead_code)]
    pub fn record_goals_opened(&mut self, n: usize) {
        self.goals_opened += n;
    }
}
/// A view into a single goal (metavariable).
#[derive(Clone, Debug)]
pub struct GoalView {
    /// The metavariable ID for this goal.
    pub mvar_id: MVarId,
    /// The target type to prove.
    pub target: Expr,
    /// Names and types of local hypotheses.
    pub hyps: Vec<(Name, Expr)>,
    /// User-assigned tag for this goal.
    pub tag: Option<String>,
}
/// Error type for tactic failures.
#[derive(Clone, Debug)]
pub enum TacticError {
    /// No goals remain.
    NoGoals,
    /// The tactic failed with a message.
    Failed(String),
    /// Type mismatch.
    TypeMismatch {
        /// Expected type.
        expected: Expr,
        /// Got type.
        got: Expr,
    },
    /// Unknown hypothesis.
    UnknownHyp(Name),
    /// Goal is not of the expected form.
    GoalMismatch(String),
    /// Internal error.
    Internal(String),
}
/// Records the difference between two tactic states.
///
/// Useful for explaining what changed after a tactic was applied.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct GoalDiff {
    /// Goals that were present before but not after.
    pub closed: Vec<MVarId>,
    /// Goals that are present after but not before.
    pub opened: Vec<MVarId>,
}
impl GoalDiff {
    /// Compute the diff between two goal lists.
    #[allow(dead_code)]
    pub fn compute(before: &[MVarId], after: &[MVarId]) -> Self {
        let closed = before
            .iter()
            .filter(|g| !after.contains(g))
            .copied()
            .collect();
        let opened = after
            .iter()
            .filter(|g| !before.contains(g))
            .copied()
            .collect();
        GoalDiff { closed, opened }
    }
    /// Number of goals closed.
    #[allow(dead_code)]
    pub fn num_closed(&self) -> usize {
        self.closed.len()
    }
    /// Number of goals opened.
    #[allow(dead_code)]
    pub fn num_opened(&self) -> usize {
        self.opened.len()
    }
    /// Net change in goal count (positive = fewer goals).
    #[allow(dead_code)]
    pub fn net(&self) -> i64 {
        self.closed.len() as i64 - self.opened.len() as i64
    }
    /// Whether any change occurred.
    #[allow(dead_code)]
    pub fn has_change(&self) -> bool {
        !self.closed.is_empty() || !self.opened.is_empty()
    }
}
/// A configuration store for TacticState.
#[allow(dead_code)]
pub struct TacticStateConfig {
    pub values: std::collections::HashMap<String, TacticStateConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticStateConfig {
    pub fn new() -> Self {
        TacticStateConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticStateConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticStateConfigValue> {
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
        self.set(key, TacticStateConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticStateConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticStateConfigValue::Str(v.to_string()))
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
/// A checkpoint manager that stores named checkpoints.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct CheckpointManager {
    pub(super) checkpoints: Vec<TacticCheckpoint>,
}
impl CheckpointManager {
    /// Create an empty manager.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Save a checkpoint with the given name.
    #[allow(dead_code)]
    pub fn save(&mut self, name: &str, state: &TacticState) {
        self.checkpoints
            .push(TacticCheckpoint::from_state(name, state));
    }
    /// Load the most recently saved checkpoint with the given name.
    #[allow(dead_code)]
    pub fn load(&self, name: &str) -> Option<&TacticCheckpoint> {
        self.checkpoints.iter().rev().find(|c| c.name == name)
    }
    /// Number of stored checkpoints.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.checkpoints.len()
    }
    /// Whether any checkpoints have been saved.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.checkpoints.is_empty()
    }
    /// Clear all checkpoints.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.checkpoints.clear();
    }
    /// Return all checkpoint names.
    #[allow(dead_code)]
    pub fn names(&self) -> Vec<&str> {
        self.checkpoints.iter().map(|c| c.name.as_str()).collect()
    }
}
/// A counter map for TacState frequency analysis.
#[allow(dead_code)]
pub struct TacStateCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl TacStateCounterMap {
    pub fn new() -> Self {
        TacStateCounterMap {
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
/// A result type for TacticState analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticStateResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticStateResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticStateResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticStateResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticStateResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticStateResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticStateResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticStateResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticStateResult::Ok(_) => 1.0,
            TacticStateResult::Err(_) => 0.0,
            TacticStateResult::Skipped => 0.0,
            TacticStateResult::Partial { done, total } => {
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
pub struct StateExtPass1800 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<StateExtResult1800>,
}
impl StateExtPass1800 {
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
    pub fn run(&mut self, input: &str) -> StateExtResult1800 {
        if !self.enabled {
            return StateExtResult1800::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            StateExtResult1800::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            StateExtResult1800::Ok(format!(
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
/// A proof trace: the full history of tactic applications in a proof attempt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ProofTrace {
    /// The ordered list of tactic steps.
    pub steps: Vec<TacticStep>,
}
impl ProofTrace {
    /// Create a new empty proof trace.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Append a new step to the trace.
    #[allow(dead_code)]
    pub fn push(&mut self, step: TacticStep) {
        self.steps.push(step);
    }
    /// Total number of tactics applied.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Returns true if no tactics have been applied yet.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Returns the last tactic step, if any.
    #[allow(dead_code)]
    pub fn last(&self) -> Option<&TacticStep> {
        self.steps.last()
    }
    /// Returns the net change in goal count over the entire proof.
    #[allow(dead_code)]
    pub fn total_delta(&self) -> i64 {
        self.steps.iter().map(|s| s.delta()).sum()
    }
    /// Returns the names of all tactics used (in order).
    #[allow(dead_code)]
    pub fn tactic_names(&self) -> Vec<&str> {
        self.steps.iter().map(|s| s.tactic_name.as_str()).collect()
    }
}
/// Snapshot for tactic-level backtracking.
#[derive(Clone)]
struct TacticStateSnapshot {
    pub(super) goals: Vec<MVarId>,
    pub(super) focus_idx: usize,
    pub(super) tags: Vec<Option<String>>,
}
#[allow(dead_code)]
pub struct StateExtConfig1800 {
    pub(super) values: std::collections::HashMap<String, StateExtConfigVal1800>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl StateExtConfig1800 {
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
    pub fn set(&mut self, key: &str, value: StateExtConfigVal1800) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&StateExtConfigVal1800> {
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
        self.set(key, StateExtConfigVal1800::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, StateExtConfigVal1800::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, StateExtConfigVal1800::Str(v.to_string()))
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
/// An extended map for TacState keys to values.
#[allow(dead_code)]
pub struct TacStateExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> TacStateExtMap<V> {
    pub fn new() -> Self {
        TacStateExtMap {
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
pub struct StateExtPipeline1800 {
    pub name: String,
    pub passes: Vec<StateExtPass1800>,
    pub run_count: usize,
}
impl StateExtPipeline1800 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: StateExtPass1800) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<StateExtResult1800> {
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
/// A state machine controller for TacState.
#[allow(dead_code)]
pub struct TacStateStateMachine {
    pub state: TacStateState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl TacStateStateMachine {
    pub fn new() -> Self {
        TacStateStateMachine {
            state: TacStateState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: TacStateState) -> bool {
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
        self.transition_to(TacStateState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(TacStateState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(TacStateState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(TacStateState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
/// A sliding window accumulator for TacState.
#[allow(dead_code)]
pub struct TacStateWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl TacStateWindow {
    pub fn new(capacity: usize) -> Self {
        TacStateWindow {
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
/// A proof context that wraps the tactic state and tracks proof metadata.
#[allow(dead_code)]
pub struct ProofContext {
    /// The underlying tactic state.
    pub state: TacticState,
    /// Tactic performance statistics.
    pub stats: TacticStats,
    /// Trace of all tactic applications.
    pub trace: ProofTrace,
}
impl ProofContext {
    /// Create a new proof context from a tactic state.
    #[allow(dead_code)]
    pub fn new(state: TacticState) -> Self {
        ProofContext {
            state,
            stats: TacticStats::new(),
            trace: ProofTrace::new(),
        }
    }
    /// Run a tactic (by name) on the state, recording the step.
    ///
    /// The `apply_fn` callback receives a mutable reference to the state
    /// and returns `Ok(())` on success or an error string on failure.
    #[allow(dead_code)]
    pub fn run_tactic<F>(&mut self, name: &str, apply_fn: F) -> Result<(), String>
    where
        F: FnOnce(&mut TacticState) -> Result<(), String>,
    {
        let before = self.state.num_goals();
        self.stats.record_tactic();
        let result = apply_fn(&mut self.state);
        let after = self.state.num_goals();
        let step = TacticStep::new(name, before, after);
        self.trace.push(step);
        if after < before {
            self.stats.record_goals_closed(before - after);
        } else if after > before {
            self.stats.record_goals_opened(after - before);
        }
        result
    }
    /// Returns `true` if the proof is complete (no remaining goals).
    #[allow(dead_code)]
    pub fn is_complete(&self) -> bool {
        self.state.is_done()
    }
}
pub struct TacStateExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl TacStateExtUtil {
    pub fn new(key: &str) -> Self {
        TacStateExtUtil {
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
/// A state machine for TacState.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacStateState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl TacStateState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, TacStateState::Complete | TacStateState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, TacStateState::Initial | TacStateState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, TacStateState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            TacStateState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// A builder pattern for TacState.
#[allow(dead_code)]
pub struct TacStateBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl TacStateBuilder {
    pub fn new(name: &str) -> Self {
        TacStateBuilder {
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
pub struct StateExtDiag1800 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl StateExtDiag1800 {
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
/// A diagnostic reporter for TacticState.
#[allow(dead_code)]
pub struct TacticStateDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticStateDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticStateDiagnostics {
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
pub enum StateExtConfigVal1800 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl StateExtConfigVal1800 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let StateExtConfigVal1800::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let StateExtConfigVal1800::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let StateExtConfigVal1800::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let StateExtConfigVal1800::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let StateExtConfigVal1800::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            StateExtConfigVal1800::Bool(_) => "bool",
            StateExtConfigVal1800::Int(_) => "int",
            StateExtConfigVal1800::Float(_) => "float",
            StateExtConfigVal1800::Str(_) => "str",
            StateExtConfigVal1800::List(_) => "list",
        }
    }
}
/// An analysis pass for TacticState.
#[allow(dead_code)]
pub struct TacticStateAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticStateResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticStateAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticStateAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticStateResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticStateResult::Err("empty input".to_string())
        } else {
            TacticStateResult::Ok(format!("processed: {}", input))
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
#[derive(Debug, Clone)]
pub enum StateExtResult1800 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl StateExtResult1800 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, StateExtResult1800::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, StateExtResult1800::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, StateExtResult1800::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, StateExtResult1800::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let StateExtResult1800::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let StateExtResult1800::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            StateExtResult1800::Ok(_) => 1.0,
            StateExtResult1800::Err(_) => 0.0,
            StateExtResult1800::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            StateExtResult1800::Skipped => 0.5,
        }
    }
}
/// A single recorded tactic application for proof tracing.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticStep {
    /// The tactic name (e.g., "intro", "apply", "simp").
    pub tactic_name: String,
    /// Number of goals before this tactic was applied.
    pub goals_before: usize,
    /// Number of goals after this tactic was applied.
    pub goals_after: usize,
    /// Optional diagnostic message emitted by the tactic.
    pub message: Option<String>,
}
impl TacticStep {
    /// Create a new tactic step record.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, before: usize, after: usize) -> Self {
        TacticStep {
            tactic_name: name.into(),
            goals_before: before,
            goals_after: after,
            message: None,
        }
    }
    /// Attach a diagnostic message to this step.
    #[allow(dead_code)]
    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }
    /// Returns how many goals were closed by this step (negative = goals opened).
    #[allow(dead_code)]
    pub fn delta(&self) -> i64 {
        self.goals_before as i64 - self.goals_after as i64
    }
}
