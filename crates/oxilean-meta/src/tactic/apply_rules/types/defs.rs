//! Type definitions for apply_rules

use std::collections::HashMap;

/// A utility type for ApplyRules (index 1).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil1 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A typed slot for TacticApplyRules configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticApplyRulesConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

/// A registry for ApplyRules utilities.
#[allow(dead_code)]
pub struct ApplyRulesRegistry {
    pub entries: Vec<ApplyRulesUtil0>,
    pub capacity: usize,
}

/// A utility type for ApplyRules (index 5).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil5 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A diff for TacticApplyRules analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticApplyRulesDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

/// Statistics for ApplyRules operations.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesStats {
    pub total_ops: usize,
    pub successful_ops: usize,
    pub failed_ops: usize,
    pub total_time_ns: u64,
    pub max_time_ns: u64,
}

/// Configuration for the `apply_rules` tactic.
#[derive(Clone, Debug)]
pub struct ApplyRulesConfig {
    /// Maximum recursion depth for iterative application.
    pub max_depth: usize,
    /// The reasoning mode.
    pub mode: ReasoningMode,
    /// Whether to only use safe rules.
    pub safe_only: bool,
    /// Whether to continue after the first successful application.
    pub exhaustive: bool,
    /// Optional tag filter: only consider rules with these tags.
    pub tag_filter: Option<Vec<String>>,
    /// Whether to report a trace of applied rules.
    pub trace: bool,
}

/// A record of a single rule application.
#[derive(Clone, Debug)]
pub struct RuleApplication {
    /// The rule that was applied.
    pub rule_name: String,
    /// The goal before application.
    pub goal_before: String,
    /// The sub-goals produced after application.
    pub subgoals_after: Vec<String>,
    /// The depth at which this application occurred.
    pub depth: usize,
    /// Whether the application was in forward mode.
    pub forward: bool,
}

/// A prioritized collection of rules for `apply_rules`.
#[derive(Clone, Debug)]
pub struct RuleSet {
    /// Rules sorted by priority (lowest first).
    pub(crate) rules: Vec<RuleEntry>,
    /// Index by tag for fast filtering.
    pub(crate) by_tag: HashMap<String, Vec<usize>>,
}

/// The result of applying `apply_rules`.
#[derive(Clone, Debug)]
pub struct ApplyRulesResult {
    /// Whether any rules were applied successfully.
    pub success: bool,
    /// The remaining (unsolved) goals.
    pub remaining_goals: Vec<String>,
    /// The trace of rule applications.
    pub trace: Vec<RuleApplication>,
    /// Diagnostic message.
    pub message: String,
    /// Total number of rule applications.
    pub num_applications: usize,
}

/// A utility type for ApplyRules (index 11).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil11 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for ApplyRules (index 0).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil0 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// The `apply_rules` tactic implementation.
#[derive(Clone, Debug)]
pub struct ApplyRulesTactic {
    pub(crate) config: ApplyRulesConfig,
    pub(crate) rules: RuleSet,
}

/// A utility type for ApplyRules (index 6).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil6 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A result type for TacticApplyRules analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticApplyRulesResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}

/// A utility type for ApplyRules (index 8).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil8 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A pipeline of TacticApplyRules analysis passes.
#[allow(dead_code)]
pub struct TacticApplyRulesPipeline {
    pub passes: Vec<TacticApplyRulesAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}

/// A configuration store for TacticApplyRules.
#[allow(dead_code)]
pub struct TacticApplyRulesConfig {
    pub values: std::collections::HashMap<String, TacticApplyRulesConfigValue>,
    pub read_only: bool,
}

/// A diagnostic reporter for TacticApplyRules.
#[allow(dead_code)]
pub struct TacticApplyRulesDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

#[allow(dead_code)]
pub struct ApplyRulesExtPass3400 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<ApplyRulesExtResult3400>,
}

/// A single named rule (lemma) that `apply_rules` can use.
#[derive(Clone, Debug)]
pub struct RuleEntry {
    /// The name of the lemma (e.g. "Nat.le_trans", "And.intro").
    pub name: String,
    /// An optional tag for categorization (e.g. "algebra", "order").
    pub tag: Option<String>,
    /// Priority (lower = tried first).
    pub priority: u32,
    /// The shape of this rule.
    pub shape: RuleShape,
    /// The number of explicit parameters the rule expects.
    pub num_params: usize,
    /// Whether this rule is safe (will not produce unprovable sub-goals).
    pub safe: bool,
    /// The conclusion pattern as a string (for matching).
    pub conclusion_pattern: Option<String>,
    /// Hypothesis patterns as strings (for forward reasoning).
    pub hypothesis_patterns: Vec<String>,
}

/// A utility type for ApplyRules (index 2).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil2 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for ApplyRules (index 13).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil13 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

#[allow(dead_code)]
pub struct ApplyRulesExtDiff3400 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

/// A utility type for ApplyRules (index 4).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil4 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for ApplyRules (index 10).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil10 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A logger for ApplyRules operations.
#[allow(dead_code)]
pub struct ApplyRulesLogger {
    pub entries: Vec<String>,
    pub max_entries: usize,
    pub verbose: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ApplyRulesExtResult3400 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}

/// A priority queue for ApplyRules items.
#[allow(dead_code)]
pub struct ApplyRulesPriorityQueue {
    pub items: Vec<(ApplyRulesUtil0, i64)>,
}

#[allow(dead_code)]
pub struct ApplyRulesExtPipeline3400 {
    pub name: String,
    pub passes: Vec<ApplyRulesExtPass3400>,
    pub run_count: usize,
}

/// A simple cache for ApplyRules computations.
#[allow(dead_code)]
pub struct ApplyRulesCache {
    pub data: std::collections::HashMap<String, i64>,
    pub hits: usize,
    pub misses: usize,
}

/// A utility type for ApplyRules (index 3).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil3 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for ApplyRules (index 7).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil7 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// A utility type for ApplyRules (index 14).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil14 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

/// An analysis pass for TacticApplyRules.
#[allow(dead_code)]
pub struct TacticApplyRulesAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticApplyRulesResult>,
    pub total_runs: usize,
}

/// Internal state for tracking sub-goals during recursive application.
#[derive(Clone, Debug)]
pub(crate) struct SubgoalState {
    /// Current open goals.
    pub(crate) goals: Vec<String>,
    /// Hypotheses available for forward reasoning.
    pub(crate) hypotheses: Vec<String>,
    /// Current depth.
    pub(crate) depth: usize,
    /// Accumulated trace.
    pub(crate) trace: Vec<RuleApplication>,
}

/// A utility type for ApplyRules (index 12).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil12 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}

#[allow(dead_code)]
pub struct ApplyRulesExtConfig3400 {
    pub(crate) values: std::collections::HashMap<String, ApplyRulesExtConfigVal3400>,
    pub(crate) read_only: bool,
    pub(crate) name: String,
}

#[allow(dead_code)]
pub struct ApplyRulesExtDiag3400 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

/// The reasoning direction for `apply_rules`.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ReasoningMode {
    /// Backward reasoning: try to match the goal's target and reduce it.
    #[default]
    Backward,
    /// Forward reasoning: try to match hypotheses and derive new facts.
    Forward,
    /// Try both backward and forward reasoning.
    Both,
}

/// The shape of a rule: how many hypotheses and what kind of conclusion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RuleShape {
    /// A rule that closes the goal outright (e.g. `Nat.le_refl`).
    Closing,
    /// A rule that reduces the goal to one sub-goal.
    SingleSubgoal,
    /// A rule that reduces the goal to multiple sub-goals.
    MultiSubgoal(usize),
    /// Unknown shape.
    Unknown,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ApplyRulesExtConfigVal3400 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

/// A utility type for ApplyRules (index 9).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ApplyRulesUtil9 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
