//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tactic::{Goal, TacticResult, TacticState};
use oxilean_kernel::{Expr, Name};

use super::functions::*;

use std::collections::{HashMap, VecDeque};

pub struct TacticAutoProfile {
    pub node_count: u64,
    pub backtrack_count: u64,
    pub max_depth_reached: u32,
    pub hint_database_size: usize,
    pub successful_hints: Vec<String>,
    pub failed_hints: Vec<String>,
}
impl TacticAutoProfile {
    pub fn new() -> Self {
        TacticAutoProfile {
            node_count: 0,
            backtrack_count: 0,
            max_depth_reached: 0,
            hint_database_size: 0,
            successful_hints: Vec::new(),
            failed_hints: Vec::new(),
        }
    }
    pub fn record_node(&mut self, depth: u32) {
        self.node_count += 1;
        if depth > self.max_depth_reached {
            self.max_depth_reached = depth;
        }
    }
    pub fn record_backtrack(&mut self) {
        self.backtrack_count += 1;
    }
    pub fn record_success(&mut self, hint: impl Into<String>) {
        self.successful_hints.push(hint.into());
    }
    pub fn record_failure(&mut self, hint: impl Into<String>) {
        self.failed_hints.push(hint.into());
    }
    pub fn success_rate(&self) -> f64 {
        let total = self.successful_hints.len() + self.failed_hints.len();
        if total == 0 {
            0.0
        } else {
            self.successful_hints.len() as f64 / total as f64
        }
    }
    pub fn summary(&self) -> String {
        format!(
            "AutoProfile: {} nodes, {} backtracks, max depth {}, {} successful hints",
            self.node_count,
            self.backtrack_count,
            self.max_depth_reached,
            self.successful_hints.len()
        )
    }
}
pub struct AutoHintFilterChain {
    filters: Vec<Box<dyn HintFilter>>,
}
impl AutoHintFilterChain {
    pub fn new() -> Self {
        AutoHintFilterChain {
            filters: Vec::new(),
        }
    }
    pub fn add<F: HintFilter + 'static>(mut self, filter: F) -> Self {
        self.filters.push(Box::new(filter));
        self
    }
    pub fn accept_all(&self, hint: &str, goal: &Goal) -> bool {
        self.filters.iter().all(|f| f.accept(hint, goal))
    }
    pub fn filter_hints(&self, hints: &[String], goal: &Goal) -> Vec<String> {
        hints
            .iter()
            .filter(|h| self.accept_all(h, goal))
            .cloned()
            .collect()
    }
    pub fn filter_count(&self) -> usize {
        self.filters.len()
    }
}
/// A priority queue entry for the auto tactic search.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SearchNode {
    /// Priority (higher = try sooner).
    pub priority: f64,
    /// Current depth in the search tree.
    pub depth: u32,
    /// The goal target string at this node.
    pub goal_str: String,
    /// Lemma or rule that produced this node.
    pub rule: String,
}
impl SearchNode {
    /// Create a new SearchNode.
    #[allow(dead_code)]
    pub fn new(
        priority: f64,
        depth: u32,
        goal_str: impl Into<String>,
        rule: impl Into<String>,
    ) -> Self {
        Self {
            priority,
            depth,
            goal_str: goal_str.into(),
            rule: rule.into(),
        }
    }
}
/// Configuration for the tautology checker.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TautoConfig {
    /// Maximum search depth.
    pub max_depth: u32,
    /// Whether to use disjunctive syllogism.
    pub use_disj_syllogism: bool,
    /// Whether to use hypothetical syllogism.
    pub use_hypo_syllogism: bool,
    /// Whether to use modus ponens from hypotheses.
    pub use_modus_ponens: bool,
}
/// A single step in a proof trace.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProofStep {
    /// The rule or tactic applied at this step.
    pub rule: String,
    /// The goal target before applying the rule.
    pub goal_before: String,
    /// The goal targets after (empty means solved).
    pub goals_after: Vec<String>,
    /// Search depth at this step.
    pub depth: u32,
}
impl ProofStep {
    /// Create a new proof step.
    #[allow(dead_code)]
    pub fn new(
        rule: impl Into<String>,
        goal_before: impl Into<String>,
        goals_after: Vec<String>,
        depth: u32,
    ) -> Self {
        Self {
            rule: rule.into(),
            goal_before: goal_before.into(),
            goals_after,
            depth,
        }
    }
    /// Return true if this step closed the goal.
    #[allow(dead_code)]
    pub fn is_closing(&self) -> bool {
        self.goals_after.is_empty()
    }
}
/// Configuration for the auto tactic.
#[derive(Debug, Clone)]
pub struct AutoConfig {
    /// Maximum search depth for iterative deepening.
    pub max_depth: u32,
    /// Maximum total steps before giving up.
    pub max_steps: u32,
    /// Whether to try hypothesis assumption at each step.
    pub use_assumptions: bool,
    /// Whether to apply simplification rules.
    pub use_simp: bool,
    /// Whether to try constructor rules (True, And, Or).
    pub use_constructor: bool,
    /// Whether to try applying lemma hints.
    pub use_apply: bool,
    /// Named lemma hints to try during search.
    pub lemma_hints: Vec<String>,
}
pub struct NamePrefixFilter {
    pub(super) prefix: String,
}
impl NamePrefixFilter {
    pub fn new(prefix: impl Into<String>) -> Self {
        NamePrefixFilter {
            prefix: prefix.into(),
        }
    }
}
/// A hint database for the auto tactic, grouping lemmas by category.
#[allow(dead_code)]
pub struct HintDatabase {
    entries: std::collections::HashMap<String, Vec<String>>,
}
impl HintDatabase {
    /// Create an empty hint database.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
    /// Add a lemma hint under the given category.
    #[allow(dead_code)]
    pub fn add(&mut self, category: impl Into<String>, lemma: impl Into<String>) {
        self.entries
            .entry(category.into())
            .or_default()
            .push(lemma.into());
    }
    /// Get all lemma names for a category.
    #[allow(dead_code)]
    pub fn get(&self, category: &str) -> &[String] {
        self.entries
            .get(category)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Get all lemma names across all categories (deduped).
    #[allow(dead_code)]
    pub fn all_lemmas(&self) -> Vec<&str> {
        let mut result: Vec<&str> = self
            .entries
            .values()
            .flat_map(|v| v.iter().map(|s| s.as_str()))
            .collect();
        result.sort_unstable();
        result.dedup();
        result
    }
    /// Number of categories in the database.
    #[allow(dead_code)]
    pub fn num_categories(&self) -> usize {
        self.entries.len()
    }
    /// Total number of lemma entries (with duplicates across categories).
    #[allow(dead_code)]
    pub fn total_entries(&self) -> usize {
        self.entries.values().map(|v| v.len()).sum()
    }
    /// Merge another database into this one.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: HintDatabase) {
        for (cat, lemmas) in other.entries {
            self.entries.entry(cat).or_default().extend(lemmas);
        }
    }
    /// Create a standard hint database with common Nat/Bool/Logic lemmas.
    #[allow(dead_code)]
    pub fn standard() -> Self {
        let mut db = Self::new();
        db.add("nat", "Nat.zero_add");
        db.add("nat", "Nat.add_zero");
        db.add("nat", "Nat.add_comm");
        db.add("nat", "Nat.add_assoc");
        db.add("nat", "Nat.mul_comm");
        db.add("nat", "Nat.mul_one");
        db.add("nat", "Nat.one_mul");
        db.add("logic", "And.intro");
        db.add("logic", "Or.inl");
        db.add("logic", "Or.inr");
        db.add("logic", "Iff.intro");
        db.add("logic", "not_not");
        db.add("bool", "Bool.true_and");
        db.add("bool", "Bool.and_true");
        db.add("bool", "Bool.false_or");
        db.add("bool", "Bool.or_false");
        db
    }
}
/// A trace of proof steps accumulated during auto search.
#[allow(dead_code)]
pub struct ProofTrace {
    steps: Vec<ProofStep>,
    enabled: bool,
}
impl ProofTrace {
    /// Create a new proof trace (disabled by default).
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            enabled: false,
        }
    }
    /// Enable tracing.
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Record a proof step if tracing is enabled.
    #[allow(dead_code)]
    pub fn record(&mut self, step: ProofStep) {
        if self.enabled {
            self.steps.push(step);
        }
    }
    /// Return the steps recorded.
    #[allow(dead_code)]
    pub fn steps(&self) -> &[ProofStep] {
        &self.steps
    }
    /// Count total steps recorded.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Return true if no steps have been recorded.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Format the trace as a human-readable string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let mut out = String::new();
        for (i, step) in self.steps.iter().enumerate() {
            let indent = "  ".repeat(step.depth as usize);
            out.push_str(&format!(
                "{}{}: [{}] {} -> {}\n",
                indent,
                i,
                step.depth,
                step.rule,
                if step.is_closing() {
                    "QED".to_string()
                } else {
                    step.goals_after.join(", ")
                }
            ));
        }
        out
    }
    /// Clear all recorded steps.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.steps.clear();
    }
}
/// Tracks the remaining search budget during proof search.
#[allow(dead_code)]
pub struct SearchBudget {
    max_steps: u32,
    consumed: u32,
    max_depth: u32,
}
impl SearchBudget {
    /// Create a new budget from an AutoConfig.
    #[allow(dead_code)]
    pub fn from_config(config: &AutoConfig) -> Self {
        Self {
            max_steps: config.max_steps,
            consumed: 0,
            max_depth: config.max_depth,
        }
    }
    /// Attempt to consume one step. Returns false if exhausted.
    #[allow(dead_code)]
    pub fn consume_step(&mut self) -> bool {
        if self.consumed < self.max_steps {
            self.consumed += 1;
            true
        } else {
            false
        }
    }
    /// Returns true if the given depth is within budget.
    #[allow(dead_code)]
    pub fn within_depth(&self, depth: u32) -> bool {
        depth <= self.max_depth
    }
    /// Remaining steps.
    #[allow(dead_code)]
    pub fn remaining(&self) -> u32 {
        self.max_steps.saturating_sub(self.consumed)
    }
    /// Whether the budget is exhausted.
    #[allow(dead_code)]
    pub fn is_exhausted(&self) -> bool {
        self.consumed >= self.max_steps
    }
    /// Reset consumed counter.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.consumed = 0;
    }
}
pub struct BestFirstSearchNode {
    pub goal: Goal,
    pub score: f64,
    pub depth: u32,
    pub parent_lemma: Option<Name>,
}
impl BestFirstSearchNode {
    pub fn new(goal: Goal, score: f64, depth: u32) -> Self {
        BestFirstSearchNode {
            goal,
            score,
            depth,
            parent_lemma: None,
        }
    }
    pub fn with_parent_lemma(mut self, lemma: Name) -> Self {
        self.parent_lemma = Some(lemma);
        self
    }
}
pub struct AutoLemmaScorer {
    scores: std::collections::HashMap<String, f64>,
}
impl AutoLemmaScorer {
    pub fn new() -> Self {
        AutoLemmaScorer {
            scores: std::collections::HashMap::new(),
        }
    }
    pub fn set_score(&mut self, lemma: impl Into<String>, score: f64) {
        self.scores.insert(lemma.into(), score);
    }
    pub fn score(&self, lemma: &str) -> f64 {
        self.scores.get(lemma).copied().unwrap_or(0.0)
    }
    pub fn top_lemmas(&self, n: usize) -> Vec<(&String, f64)> {
        let mut pairs: Vec<(&String, f64)> = self.scores.iter().map(|(k, &v)| (k, v)).collect();
        pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        pairs.truncate(n);
        pairs
    }
}
#[derive(Debug, Clone)]
pub enum AutoAnnotation {
    MaxDepth(u32),
    HintSet(Vec<Name>),
    Strategy(String),
    Timeout(u64),
    UseClassical,
}
impl AutoAnnotation {
    pub fn annotation_name(&self) -> &'static str {
        match self {
            AutoAnnotation::MaxDepth(_) => "max_depth",
            AutoAnnotation::HintSet(_) => "hint_set",
            AutoAnnotation::Strategy(_) => "strategy",
            AutoAnnotation::Timeout(_) => "timeout",
            AutoAnnotation::UseClassical => "classical",
        }
    }
    pub fn apply_to_config(&self, config: &mut AutoConfig) {
        match self {
            AutoAnnotation::MaxDepth(d) => config.max_depth = *d,
            AutoAnnotation::UseClassical => {}
            _ => {}
        }
    }
}
pub struct AutoSearchBudgetTracker {
    pub node_limit: u64,
    pub time_limit_ms: u64,
    pub nodes_used: u64,
    pub steps_used: u64,
}
impl AutoSearchBudgetTracker {
    pub fn new(node_limit: u64, time_limit_ms: u64) -> Self {
        AutoSearchBudgetTracker {
            node_limit,
            time_limit_ms,
            nodes_used: 0,
            steps_used: 0,
        }
    }
    pub fn consume_node(&mut self) -> bool {
        self.nodes_used += 1;
        self.nodes_used <= self.node_limit
    }
    pub fn consume_step(&mut self) -> bool {
        self.steps_used += 1;
        true
    }
    pub fn is_exhausted(&self) -> bool {
        self.nodes_used >= self.node_limit
    }
    pub fn utilization(&self) -> f64 {
        self.nodes_used as f64 / self.node_limit as f64
    }
    pub fn reset(&mut self) {
        self.nodes_used = 0;
        self.steps_used = 0;
    }
}
pub struct AutoTacticRegistry {
    tactics: std::collections::HashMap<String, AutoConfig>,
}
impl AutoTacticRegistry {
    pub fn new() -> Self {
        AutoTacticRegistry {
            tactics: std::collections::HashMap::new(),
        }
    }
    pub fn register(&mut self, name: impl Into<String>, config: AutoConfig) {
        self.tactics.insert(name.into(), config);
    }
    pub fn lookup(&self, name: &str) -> Option<&AutoConfig> {
        self.tactics.get(name)
    }
    pub fn names(&self) -> Vec<&String> {
        self.tactics.keys().collect()
    }
    pub fn count(&self) -> usize {
        self.tactics.len()
    }
    pub fn is_empty(&self) -> bool {
        self.tactics.is_empty()
    }
}
/// Iterative deepening depth-first proof search.
pub struct AutoTactic {
    pub(crate) config: AutoConfig,
    pub(crate) steps_taken: u32,
}
impl AutoTactic {
    /// Create a new AutoTactic with the given configuration.
    pub fn new(config: AutoConfig) -> Self {
        AutoTactic {
            config,
            steps_taken: 0,
        }
    }
    /// Create a new AutoTactic with default configuration.
    pub fn with_defaults() -> Self {
        AutoTactic::new(AutoConfig::default())
    }
    /// Search for a proof of all goals using iterative deepening DFS.
    pub fn search(&mut self, state: &TacticState) -> SearchResult {
        self.steps_taken = 0;
        let goals = state.goals();
        if goals.is_empty() {
            return SearchResult::Solved;
        }
        for depth in 1..=self.config.max_depth {
            let all_solved = goals.iter().all(|g| self.search_goal(g, depth));
            if all_solved {
                return SearchResult::Solved;
            }
            if self.steps_taken >= self.config.max_steps {
                break;
            }
        }
        let remaining = goals.len() as u32;
        if remaining > 0 {
            SearchResult::Partial(remaining)
        } else {
            SearchResult::Failed
        }
    }
    /// Try to prove a single goal up to the given depth.
    fn search_goal(&mut self, goal: &Goal, depth: u32) -> bool {
        if self.steps_taken >= self.config.max_steps {
            return false;
        }
        self.steps_taken += 1;
        if self.is_trivial(goal) {
            return true;
        }
        if depth == 0 {
            return false;
        }
        if self.config.use_assumptions && self.try_assumption(goal) {
            return true;
        }
        if self.config.use_constructor && self.try_constructor(goal) {
            return true;
        }
        if let Some(new_goal) = self.try_intro(goal) {
            if self.search_goal(&new_goal, depth - 1) {
                return true;
            }
        }
        if self.config.use_apply {
            for lemma in self.config.lemma_hints.clone() {
                if self.try_apply_lemma(goal, &lemma) {
                    return true;
                }
            }
        }
        false
    }
    /// Check if any hypothesis directly matches (alpha-equal to) the goal target.
    fn try_assumption(&self, goal: &Goal) -> bool {
        let target = &goal.target;
        goal.hypotheses().iter().any(|(_, ty)| ty == target)
    }
    /// Try constructor tactics: True, And, Or.
    fn try_constructor(&self, goal: &Goal) -> bool {
        match &goal.target {
            Expr::Const(name, _) if name == &Name::str("True") => true,
            Expr::App(f, _b) => {
                if let Expr::App(and_head, _a) = f.as_ref() {
                    if let Expr::Const(name, _) = and_head.as_ref() {
                        if name == &Name::str("And") {
                            return true;
                        }
                    }
                }
                false
            }
            _ => false,
        }
    }
    /// If the goal is a Pi (arrow), introduce the binder and return the new goal.
    fn try_intro(&self, goal: &Goal) -> Option<Goal> {
        if let Expr::Pi(_bi, binder_name, domain, _body) = &goal.target {
            let mut new_goal = goal.clone();
            new_goal.add_hypothesis(binder_name.clone(), *domain.clone());
            Some(new_goal)
        } else {
            None
        }
    }
    /// Try to apply a named lemma hint (by name string matching only).
    fn try_apply_lemma(&self, goal: &Goal, lemma: &str) -> bool {
        goal.hypotheses()
            .iter()
            .any(|(n, _)| n.to_string() == lemma)
    }
    /// Check if a goal is trivially provable without any search.
    pub fn is_trivial(&self, goal: &Goal) -> bool {
        if let Expr::Const(name, _) = &goal.target {
            if name == &Name::str("True") {
                return true;
            }
        }
        if is_refl_target(&goal.target) {
            return true;
        }
        if self.try_assumption(goal) {
            return true;
        }
        false
    }
    /// Return a short human-readable summary of a goal for debugging.
    pub fn goal_summary(&self, goal: &Goal) -> String {
        let hyps: Vec<String> = goal
            .hypotheses()
            .iter()
            .map(|(n, _)| n.to_string())
            .collect();
        format!(
            "Goal({}, hyps=[{}], target={})",
            goal.name,
            hyps.join(", "),
            expr_to_summary(&goal.target)
        )
    }
}
pub struct MinLengthFilter {
    pub(super) min_len: usize,
}
impl MinLengthFilter {
    pub fn new(min_len: usize) -> Self {
        MinLengthFilter { min_len }
    }
}
pub struct AutoTacticExtensionMarker;
impl AutoTacticExtensionMarker {
    pub fn new() -> Self {
        AutoTacticExtensionMarker
    }
    pub fn description() -> &'static str {
        "Auto tactic extensions: profile, filter chain, budget tracker, goal queue, lemma scorer."
    }
}
pub struct AutoGoalQueue {
    queue: std::collections::VecDeque<(Goal, f64)>,
    capacity: usize,
}
impl AutoGoalQueue {
    pub fn new(capacity: usize) -> Self {
        AutoGoalQueue {
            queue: std::collections::VecDeque::new(),
            capacity,
        }
    }
    pub fn enqueue(&mut self, goal: Goal, priority: f64) {
        if self.queue.len() >= self.capacity {
            self.queue.pop_back();
        }
        let pos = self
            .queue
            .iter()
            .position(|(_, p)| *p < priority)
            .unwrap_or(self.queue.len());
        self.queue.insert(pos, (goal, priority));
    }
    pub fn dequeue(&mut self) -> Option<(Goal, f64)> {
        self.queue.pop_front()
    }
    pub fn len(&self) -> usize {
        self.queue.len()
    }
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}
/// Propositional tautology checker.
pub struct TautoTactic;
impl TautoTactic {
    /// Create a new TautoTactic.
    pub fn new() -> Self {
        TautoTactic
    }
    /// Check whether `goal` is a propositional tautology.
    pub fn check(&self, goal: &Goal) -> bool {
        let hyps: Vec<(String, String)> = goal
            .hypotheses()
            .iter()
            .map(|(n, ty)| (n.to_string(), expr_to_summary(ty)))
            .collect();
        let target_str = expr_to_summary(&goal.target);
        self.check_with_hyps_str(&target_str, &hyps, 0)
    }
    /// Recursive checker using a goal with explicit hypotheses list.
    #[allow(dead_code)]
    fn check_with_hyps(&self, goal: &Goal, hyps: &[(String, String)], depth: u32) -> bool {
        let target_str = expr_to_summary(&goal.target);
        self.check_with_hyps_str(&target_str, hyps, depth)
    }
    fn check_with_hyps_str(&self, target: &str, hyps: &[(String, String)], depth: u32) -> bool {
        if depth > 30 {
            return false;
        }
        if hyps.iter().any(|(_, ty)| ty == target) {
            return true;
        }
        if target == "True" {
            return true;
        }
        if let Some((left, right)) = split_and(target) {
            return self.check_with_hyps_str(left, hyps, depth + 1)
                && self.check_with_hyps_str(right, hyps, depth + 1);
        }
        if let Some((left, _right)) = split_or(target) {
            if self.check_with_hyps_str(left, hyps, depth + 1) {
                return true;
            }
        }
        if let Some((_left, right)) = split_or(target) {
            if self.check_with_hyps_str(right, hyps, depth + 1) {
                return true;
            }
        }
        if target.starts_with("Pi(") {
            let hyp_name = format!("_intro_{depth}");
            let mut new_hyps = hyps.to_vec();
            new_hyps.push((hyp_name, "Hyp".to_string()));
            let _ = new_hyps;
            return false;
        }
        let not_target = format!("App(Not,{target})");
        if hyps.iter().any(|(_, ty)| ty == not_target.as_str()) {
            return true;
        }
        false
    }
}
/// Result of an auto proof search.
#[derive(Debug, PartialEq)]
pub enum SearchResult {
    /// All goals were solved.
    Solved,
    /// No proof was found within the search budget.
    Failed,
    /// Search ran out of depth; this many goals remain.
    Partial(u32),
}
pub struct HeuristicSearch {
    pub beam_width: usize,
}
impl HeuristicSearch {
    pub fn new(beam_width: usize) -> Self {
        HeuristicSearch { beam_width }
    }
}
pub struct TacticAutoReport {
    pub strategy: &'static str,
    pub stats: SearchStatistics,
    pub profile: TacticAutoProfile,
    pub proof_found: bool,
    pub proof_length: Option<usize>,
}
impl TacticAutoReport {
    pub fn new(strategy: &'static str) -> Self {
        TacticAutoReport {
            strategy,
            stats: SearchStatistics::new(),
            profile: TacticAutoProfile::new(),
            proof_found: false,
            proof_length: None,
        }
    }
    pub fn with_proof(mut self, length: usize) -> Self {
        self.proof_found = true;
        self.proof_length = Some(length);
        self
    }
    pub fn summary(&self) -> String {
        if self.proof_found {
            format!(
                "Auto success (strategy={}, {} steps, {} iters)",
                self.strategy,
                self.proof_length.unwrap_or(0),
                self.stats.iterations
            )
        } else {
            format!(
                "Auto failed (strategy={}, {} iters, {} depth limits)",
                self.strategy, self.stats.iterations, self.stats.depth_limit_hits
            )
        }
    }
}
/// Detailed result of running the auto tactic.
#[allow(dead_code)]
pub struct AutoResult {
    /// The high-level search outcome.
    pub outcome: SearchResult,
    /// Total steps consumed during search.
    pub steps_consumed: u32,
    /// Maximum depth reached during search.
    pub max_depth_reached: u32,
    /// Optional proof trace.
    pub trace: Option<ProofTrace>,
}
impl AutoResult {
    /// Create a successful result.
    #[allow(dead_code)]
    pub fn solved(steps: u32, max_depth: u32) -> Self {
        Self {
            outcome: SearchResult::Solved,
            steps_consumed: steps,
            max_depth_reached: max_depth,
            trace: None,
        }
    }
    /// Create a failed result.
    #[allow(dead_code)]
    pub fn failed(steps: u32, max_depth: u32) -> Self {
        Self {
            outcome: SearchResult::Failed,
            steps_consumed: steps,
            max_depth_reached: max_depth,
            trace: None,
        }
    }
    /// Attach a proof trace.
    #[allow(dead_code)]
    pub fn with_trace(mut self, trace: ProofTrace) -> Self {
        self.trace = Some(trace);
        self
    }
    /// Format a summary of the result.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        let outcome_str = match &self.outcome {
            SearchResult::Solved => "Solved".to_string(),
            SearchResult::Failed => "Failed".to_string(),
            SearchResult::Partial(n) => format!("Partial({n})"),
        };
        format!(
            "AutoResult: {} | steps={} | max_depth={}",
            outcome_str, self.steps_consumed, self.max_depth_reached
        )
    }
}
pub struct ExhaustiveSearch;
pub struct AutoTacticChain {
    tactics: Vec<AutoConfig>,
    stop_on_success: bool,
}
impl AutoTacticChain {
    pub fn new() -> Self {
        AutoTacticChain {
            tactics: Vec::new(),
            stop_on_success: true,
        }
    }
    pub fn add(mut self, config: AutoConfig) -> Self {
        self.tactics.push(config);
        self
    }
    pub fn try_all(mut self) -> Self {
        self.stop_on_success = false;
        self
    }
    pub fn run(&self, state: &TacticState) -> TacticResult {
        for config in &self.tactics {
            let result = eval_auto(state, config.clone());
            if result.is_ok() && self.stop_on_success {
                return result;
            }
        }
        if let Some(last) = self.tactics.last() {
            eval_auto(state, last.clone())
        } else {
            Err(crate::tactic::TacticError::InternalError(
                "AutoTacticChain: no tactics configured".to_string(),
            ))
        }
    }
    pub fn count(&self) -> usize {
        self.tactics.len()
    }
}
#[derive(Debug, Default)]
pub struct SearchStatistics {
    pub iterations: u64,
    pub depth_limit_hits: u64,
    pub lemma_applications: u64,
    pub success: bool,
}
impl SearchStatistics {
    pub fn new() -> Self {
        SearchStatistics::default()
    }
    pub fn record_iteration(&mut self) {
        self.iterations += 1;
    }
    pub fn record_depth_limit_hit(&mut self) {
        self.depth_limit_hits += 1;
    }
    pub fn record_lemma_application(&mut self) {
        self.lemma_applications += 1;
    }
    pub fn mark_success(&mut self) {
        self.success = true;
    }
    pub fn efficiency(&self) -> f64 {
        if self.iterations == 0 {
            0.0
        } else {
            self.lemma_applications as f64 / self.iterations as f64
        }
    }
}
pub struct AutoTacticBuilder {
    config: AutoConfig,
    hints: Vec<Name>,
    strategy: SearchStrategy,
}
impl AutoTacticBuilder {
    pub fn new() -> Self {
        AutoTacticBuilder {
            config: AutoConfig::default(),
            hints: Vec::new(),
            strategy: SearchStrategy::Exhaustive,
        }
    }
    pub fn max_depth(mut self, depth: u32) -> Self {
        self.config.max_depth = depth;
        self
    }
    pub fn hint(mut self, name: Name) -> Self {
        self.hints.push(name);
        self
    }
    pub fn strategy(mut self, strat: SearchStrategy) -> Self {
        self.strategy = strat;
        self
    }
    pub fn build(self) -> AutoTactic {
        let _hints = self.hints;
        AutoTactic::new(self.config)
    }
}
/// A simple priority-ordered search frontier using a sorted vector.
#[allow(dead_code)]
pub struct SearchFrontier {
    nodes: Vec<SearchNode>,
}
impl SearchFrontier {
    /// Create an empty frontier.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    /// Push a node onto the frontier, keeping it sorted by priority (desc).
    #[allow(dead_code)]
    pub fn push(&mut self, node: SearchNode) {
        let pos = self
            .nodes
            .binary_search_by(|n| {
                node.priority
                    .partial_cmp(&n.priority)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or_else(|i| i);
        self.nodes.insert(pos, node);
    }
    /// Pop the highest-priority node.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<SearchNode> {
        if self.nodes.is_empty() {
            None
        } else {
            Some(self.nodes.remove(0))
        }
    }
    /// Number of nodes in the frontier.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }
    /// Return true if the frontier is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    /// Clear the frontier.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.nodes.clear();
    }
}
pub enum SearchStrategy {
    Exhaustive,
    Heuristic(usize),
    BestFirst,
    Bidirectional,
}
impl SearchStrategy {
    pub fn name(&self) -> &'static str {
        match self {
            SearchStrategy::Exhaustive => "exhaustive",
            SearchStrategy::Heuristic(_) => "heuristic",
            SearchStrategy::BestFirst => "best-first",
            SearchStrategy::Bidirectional => "bidirectional",
        }
    }
    pub fn beam_width(&self) -> Option<usize> {
        if let SearchStrategy::Heuristic(w) = self {
            Some(*w)
        } else {
            None
        }
    }
}
pub struct AutoTacticSession {
    pub(crate) config: AutoConfig,
    pub(crate) profile: TacticAutoProfile,
    pub(crate) budget: AutoSearchBudgetTracker,
    pub(crate) scorer: AutoLemmaScorer,
    pub(crate) report: Option<TacticAutoReport>,
}
impl AutoTacticSession {
    pub fn new(config: AutoConfig) -> Self {
        let node_limit = config.max_steps as u64 * 10;
        AutoTacticSession {
            config,
            profile: TacticAutoProfile::new(),
            budget: AutoSearchBudgetTracker::new(node_limit, 5000),
            scorer: AutoLemmaScorer::new(),
            report: None,
        }
    }
    pub fn set_lemma_score(&mut self, lemma: impl Into<String>, score: f64) {
        self.scorer.set_score(lemma, score);
    }
    pub fn run(&mut self, state: &TacticState) -> TacticResult {
        let result = eval_auto(state, self.config.clone());
        let mut report = TacticAutoReport::new("session");
        if result.is_ok() {
            report = report.with_proof(1);
        }
        self.report = Some(report);
        result
    }
    pub fn report(&self) -> Option<&TacticAutoReport> {
        self.report.as_ref()
    }
    pub fn profile(&self) -> &TacticAutoProfile {
        &self.profile
    }
    pub fn is_budget_exhausted(&self) -> bool {
        self.budget.is_exhausted()
    }
}
