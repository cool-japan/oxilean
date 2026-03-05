//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_elab::{Goal, TacticState};
use oxilean_kernel::{print_expr, Environment, Expr, Name};
use std::time::Instant;

/// Detailed information about a hypothesis.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HypothesisInfo {
    /// The hypothesis name.
    pub name: Name,
    /// Its type.
    pub ty: Expr,
    /// Optional value (if it's a let-binding).
    pub value: Option<Expr>,
    /// Whether it's a let-binding or a hypothesis.
    pub is_let: bool,
    /// How many times it's used in the goal.
    pub usage_count: usize,
}
/// A single step in a proof.
#[derive(Clone)]
#[allow(dead_code)]
pub struct ProofStep {
    /// The tactic that was applied.
    pub tactic: String,
    /// State before this step.
    pub state_before: TacticState,
    /// State after this step.
    pub state_after: TacticState,
}
/// A tactic suggestion with a reason.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TacticSuggestion {
    /// The suggested tactic text.
    pub tactic: String,
    /// Why this tactic is suggested.
    pub reason: SuggestionReason,
    /// Human-readable explanation.
    pub explanation: String,
}
/// Display configuration for goals.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct GoalDisplayConfig {
    /// Maximum width for type display.
    pub max_type_width: usize,
    /// Whether to show metavariable IDs.
    pub show_mvar_ids: bool,
    /// Whether to show goal tags.
    pub show_tags: bool,
    /// Whether to show let-bindings in context.
    pub show_let_values: bool,
}
/// A snapshot of the proof state at a particular point.
#[derive(Clone)]
#[allow(dead_code)]
pub struct ProofStateSnapshot {
    /// Step number.
    pub step: usize,
    /// The tactic applied to reach this state.
    pub tactic: String,
    /// The resulting tactic state.
    pub state: TacticState,
    /// Time it took to execute the tactic.
    pub execution_time: u128,
    /// Whether the proof is complete at this step.
    pub is_complete: bool,
}
/// Hints for proving a goal.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProofHint {
    /// The suggested tactic.
    pub tactic: String,
    /// Confidence level (0-100).
    pub confidence: u8,
    /// Explanation of why this might work.
    pub explanation: String,
    /// Estimated number of steps to close.
    pub estimated_steps: usize,
}
/// A proof tree built from proof steps.
#[allow(dead_code)]
pub struct ProofTree {
    /// Root node.
    root: ProofTreeNode,
}
#[allow(dead_code)]
impl ProofTree {
    /// Build a proof tree from proof steps.
    pub fn build_proof_tree(steps: &[ProofStep]) -> Self {
        if steps.is_empty() {
            return Self {
                root: ProofTreeNode {
                    tactic: "(empty)".to_string(),
                    goal_name: "root".to_string(),
                    children: Vec::new(),
                    is_complete: false,
                },
            };
        }
        let mut root = ProofTreeNode {
            tactic: steps[0].tactic.clone(),
            goal_name: "root".to_string(),
            children: Vec::new(),
            is_complete: false,
        };
        for step in steps.iter().skip(1) {
            let child = ProofTreeNode {
                tactic: step.tactic.clone(),
                goal_name: format!("goal_{}", root.children.len()),
                children: Vec::new(),
                is_complete: step.state_after.is_complete(),
            };
            root.children.push(child);
        }
        if let Some(last) = steps.last() {
            root.is_complete = last.state_after.is_complete();
        }
        Self { root }
    }
    /// Render the proof tree as ASCII art.
    pub fn render_tree(&self) -> String {
        let mut output = String::new();
        render_node(&self.root, &mut output, "", true);
        output
    }
    /// Render a summary of the proof.
    pub fn render_summary(&self) -> String {
        let total_steps = count_nodes(&self.root);
        let complete_branches = count_complete(&self.root);
        let mut output = String::new();
        output.push_str("Proof Summary:\n");
        output.push_str(&format!("  Total steps:       {}\n", total_steps));
        output.push_str(&format!("  Complete branches: {}\n", complete_branches));
        output.push_str(&format!(
            "  Status:            {}\n",
            if self.root.is_complete {
                "COMPLETE"
            } else {
                "INCOMPLETE"
            }
        ));
        output
    }
    /// Get the root node.
    pub fn root(&self) -> &ProofTreeNode {
        &self.root
    }
}
/// Proof state history for undo/redo.
#[allow(dead_code)]
pub struct ProofHistory {
    /// All snapshots in order.
    snapshots: Vec<ProofStateSnapshot>,
    /// Current position in the proof.
    current_pos: usize,
}
#[allow(dead_code)]
impl ProofHistory {
    /// Create a new proof history.
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
            current_pos: 0,
        }
    }
    /// Add a new state snapshot.
    pub fn add_snapshot(&mut self, snapshot: ProofStateSnapshot) {
        self.snapshots.truncate(self.current_pos);
        self.snapshots.push(snapshot);
        self.current_pos = self.snapshots.len();
    }
    /// Step forward in the proof history.
    pub fn step_forward(&mut self) -> Option<&ProofStateSnapshot> {
        if self.current_pos < self.snapshots.len() {
            let snapshot = &self.snapshots[self.current_pos];
            self.current_pos += 1;
            Some(snapshot)
        } else {
            None
        }
    }
    /// Step backward in the proof history.
    pub fn step_backward(&mut self) -> Option<&ProofStateSnapshot> {
        if self.current_pos > 0 {
            self.current_pos -= 1;
            Some(&self.snapshots[self.current_pos])
        } else {
            None
        }
    }
    /// Get current state without moving.
    pub fn current_state(&self) -> Option<&ProofStateSnapshot> {
        if self.current_pos > 0 && self.current_pos <= self.snapshots.len() {
            Some(&self.snapshots[self.current_pos - 1])
        } else {
            None
        }
    }
    /// Get the number of steps taken so far.
    pub fn steps_taken(&self) -> usize {
        self.current_pos
    }
    /// Get total number of snapshots.
    pub fn total_snapshots(&self) -> usize {
        self.snapshots.len()
    }
    /// Check if we can undo.
    pub fn can_undo(&self) -> bool {
        self.current_pos > 0
    }
    /// Check if we can redo.
    pub fn can_redo(&self) -> bool {
        self.current_pos < self.snapshots.len()
    }
    /// Get all snapshots for statistics.
    pub fn all_snapshots(&self) -> &[ProofStateSnapshot] {
        &self.snapshots
    }
}
/// Interactive proof session with extended capabilities.
#[allow(dead_code)]
pub struct InteractiveSession<'env> {
    /// The environment
    env: &'env Environment,
    /// Current tactic state
    pub(crate) state: TacticState,
    /// History of commands
    history: Vec<String>,
    /// Goal display utility
    display: GoalDisplay,
    /// Proof navigator
    navigator: ProofNavigator,
    /// Proof history with undo/redo support
    proof_history: ProofHistory,
    /// Proof search engine
    search_hints: ProofSearchHints,
    /// Hypothesis inspector
    hypothesis_inspector: HypothesisInspector,
    /// LSP integration
    lsp: LspIntegration,
    /// Statistics about the proof session
    stats: ProofSessionStats,
}
#[allow(dead_code)]
impl<'env> InteractiveSession<'env> {
    /// Create a new interactive session.
    pub fn new(env: &'env Environment) -> Self {
        let state = TacticState::new();
        let navigator = ProofNavigator::new(state.clone());
        Self {
            env,
            state,
            history: Vec::new(),
            display: GoalDisplay::new(),
            navigator,
            proof_history: ProofHistory::new(),
            search_hints: ProofSearchHints,
            hypothesis_inspector: HypothesisInspector,
            lsp: LspIntegration::new(false),
            stats: ProofSessionStats::default(),
        }
    }
    /// Start proving a theorem.
    pub fn start_proof(&mut self, name: Name, ty: Expr) {
        let goal = Goal::new(name, ty);
        self.state = TacticState::new();
        self.state.add_goal(goal);
        self.navigator = ProofNavigator::new(self.state.clone());
        self.history.clear();
    }
    /// Execute a tactic command.
    pub fn execute(&mut self, command: String) -> Result<String, String> {
        self.history.push(command.clone());
        let before = self.state.clone();
        match execute_tactic(&self.state, &command, self.env) {
            Ok(new_state) => {
                let step = ProofStep {
                    tactic: command,
                    state_before: before,
                    state_after: new_state.clone(),
                };
                self.navigator.add_step(step);
                self.state = new_state;
                if self.state.is_complete() {
                    Ok("Goals accomplished!".to_string())
                } else {
                    Ok(self.display.format_goals_panel(&self.state))
                }
            }
            Err(e) => Err(e),
        }
    }
    /// Get the current tactic state.
    pub fn state(&self) -> &TacticState {
        &self.state
    }
    /// Get the command history.
    pub fn history(&self) -> &[String] {
        &self.history
    }
    /// Show the current goals.
    pub fn show_goals(&self) -> String {
        self.display.format_goals_panel(&self.state)
    }
    /// Reset the session.
    pub fn reset(&mut self) {
        self.state = TacticState::new();
        self.history.clear();
        self.navigator = ProofNavigator::new(self.state.clone());
    }
    /// Get tactic suggestions for the current goal.
    pub fn suggest(&self) -> Vec<TacticSuggestion> {
        suggest_tactics(&self.state)
    }
    /// Get the proof navigator.
    pub fn navigator(&self) -> &ProofNavigator {
        &self.navigator
    }
    /// Get the goal display.
    pub fn display(&self) -> &GoalDisplay {
        &self.display
    }
    /// Get the proof script so far.
    pub fn proof_script(&self) -> String {
        self.navigator.get_proof_script()
    }
    /// Check if the proof is complete.
    pub fn is_complete(&self) -> bool {
        self.state.is_complete()
    }
    /// Get search hints for the current goal.
    pub fn get_hints(&self) -> Vec<ProofHint> {
        ProofSearchHints::generate_hints(&self.state)
    }
    /// Inspect a hypothesis.
    pub fn inspect_hypothesis(&self, name: &Name) -> Option<HypothesisInfo> {
        self.state
            .goals()
            .first()
            .and_then(|goal| HypothesisInspector::inspect(goal, name))
    }
    /// List all hypotheses in the current goal.
    pub fn list_hypotheses(&self) -> Vec<HypothesisInfo> {
        self.state
            .goals()
            .first()
            .map(HypothesisInspector::list_all)
            .unwrap_or_default()
    }
    /// Get formatted hypothesis information.
    pub fn format_hypothesis_info(&self, name: &Name) -> Option<String> {
        self.inspect_hypothesis(name)
            .map(|info| HypothesisInspector::format_info(&info))
    }
    /// Redo the last undone tactic.
    pub fn redo(&mut self) -> Result<String, String> {
        if let Some(step) = self.proof_history.step_forward() {
            self.state = step.state.clone();
            self.history.push(step.tactic.clone());
            Ok(format!("Redone: {}", step.tactic))
        } else {
            Err("Nothing to redo".to_string())
        }
    }
    /// Undo the last tactic.
    pub fn undo(&mut self) -> Result<String, String> {
        if let Some(step) = self.navigator.step_backward() {
            self.state = step.state_before.clone();
            self.history.pop();
            self.stats.undos += 1;
            Ok("Undone last tactic".to_string())
        } else {
            Err("Nothing to undo".to_string())
        }
    }
    /// Connect to an LSP server for IDE integration.
    pub fn connect_lsp(&mut self, uri: String) -> Result<(), String> {
        self.lsp.connect(uri)
    }
    /// Check if LSP is connected.
    pub fn is_lsp_connected(&self) -> bool {
        self.lsp.is_connected()
    }
    /// Get session statistics.
    pub fn get_stats(&self) -> &ProofSessionStats {
        &self.stats
    }
    /// Get a summary of the proof session.
    pub fn get_session_summary(&self) -> String {
        let elapsed = self.stats.start_time.elapsed().as_millis();
        let mut summary = String::new();
        summary.push_str("=== Proof Session Summary ===\n");
        summary.push_str(&format!(
            "Tactics applied: {}\n",
            self.stats.tactics_applied
        ));
        summary.push_str(&format!("Undo operations: {}\n", self.stats.undos));
        summary.push_str(&format!("Goals created: {}\n", self.stats.goals_created));
        summary.push_str(&format!("Goals solved: {}\n", self.stats.goals_solved));
        summary.push_str(&format!("Current goals: {}\n", self.state.num_goals()));
        summary.push_str(&format!("Total time: {}ms\n", elapsed));
        summary.push_str(&format!(
            "Proof complete: {}\n",
            if self.state.is_complete() {
                "Yes"
            } else {
                "No"
            }
        ));
        summary
    }
}
/// Proof search engine for generating hints.
#[allow(dead_code)]
pub struct ProofSearchHints;
#[allow(dead_code)]
impl ProofSearchHints {
    /// Generate hints for a goal state.
    pub fn generate_hints(state: &TacticState) -> Vec<ProofHint> {
        let mut hints = Vec::new();
        if let Some(goal) = state.goals().first() {
            if let Expr::Const(name, _) = goal.target() {
                if *name == Name::str("True") {
                    hints.push(ProofHint {
                        tactic: "constructor".to_string(),
                        confidence: 95,
                        explanation: "Goal is 'True', which has a trivial constructor".to_string(),
                        estimated_steps: 1,
                    });
                }
            }
            if Self::is_likely_refl(goal.target()) {
                hints.push(ProofHint {
                    tactic: "refl".to_string(),
                    confidence: 90,
                    explanation: "Goal appears to be a reflexivity equation".to_string(),
                    estimated_steps: 1,
                });
            }
            if goal.hypotheses().iter().any(|(_, ty)| ty == goal.target()) {
                hints.push(ProofHint {
                    tactic: "assumption".to_string(),
                    confidence: 100,
                    explanation: "A hypothesis exactly matches the goal".to_string(),
                    estimated_steps: 1,
                });
            }
            if matches!(goal.target(), Expr::Pi(_, _, _, _)) {
                hints.push(ProofHint {
                    tactic: "intro".to_string(),
                    confidence: 85,
                    explanation: "Goal is a function type; introduce the argument".to_string(),
                    estimated_steps: 2,
                });
            }
            hints.push(ProofHint {
                tactic: "simp".to_string(),
                confidence: 50,
                explanation: "Try simplification (may need lemmas)".to_string(),
                estimated_steps: 3,
            });
            hints.push(ProofHint {
                tactic: "sorry".to_string(),
                confidence: 100,
                explanation: "Admit the goal (marks proof as incomplete)".to_string(),
                estimated_steps: 1,
            });
        }
        hints
    }
    fn is_likely_refl(target: &Expr) -> bool {
        if let Expr::App(f1, rhs) = target {
            if let Expr::App(_f2, lhs) = f1.as_ref() {
                return lhs == rhs;
            }
        }
        false
    }
}
/// Navigator for stepping through a proof.
#[allow(dead_code)]
pub struct ProofNavigator {
    /// All proof steps.
    steps: Vec<ProofStep>,
    /// Current position in the proof.
    current_step: usize,
    /// The initial state (before any tactics).
    initial_state: TacticState,
}
#[allow(dead_code)]
impl ProofNavigator {
    /// Create a new proof navigator with the given initial state.
    pub fn new(initial_state: TacticState) -> Self {
        Self {
            steps: Vec::new(),
            current_step: 0,
            initial_state,
        }
    }
    /// Add a proof step.
    pub fn add_step(&mut self, step: ProofStep) {
        self.steps.truncate(self.current_step);
        self.steps.push(step);
        self.current_step = self.steps.len();
    }
    /// Step forward in the proof.
    pub fn step_forward(&mut self) -> Option<&ProofStep> {
        if self.current_step < self.steps.len() {
            let step = &self.steps[self.current_step];
            self.current_step += 1;
            Some(step)
        } else {
            None
        }
    }
    /// Step backward in the proof.
    pub fn step_backward(&mut self) -> Option<&ProofStep> {
        if self.current_step > 0 {
            self.current_step -= 1;
            Some(&self.steps[self.current_step])
        } else {
            None
        }
    }
    /// Go to a specific step.
    pub fn goto_step(&mut self, step: usize) -> Option<&TacticState> {
        if step == 0 {
            self.current_step = 0;
            return Some(&self.initial_state);
        }
        if step <= self.steps.len() {
            self.current_step = step;
            Some(&self.steps[step - 1].state_after)
        } else {
            None
        }
    }
    /// Get the tactic state at a given step.
    pub fn get_state_at(&self, step: usize) -> Option<&TacticState> {
        if step == 0 {
            Some(&self.initial_state)
        } else if step <= self.steps.len() {
            Some(&self.steps[step - 1].state_after)
        } else {
            None
        }
    }
    /// Get the current tactic state.
    pub fn current_state(&self) -> &TacticState {
        if self.current_step == 0 {
            &self.initial_state
        } else {
            &self.steps[self.current_step - 1].state_after
        }
    }
    /// Get the current step number.
    pub fn current_position(&self) -> usize {
        self.current_step
    }
    /// Get the total number of steps.
    pub fn total_steps(&self) -> usize {
        self.steps.len()
    }
    /// Extract the complete proof script.
    pub fn get_proof_script(&self) -> String {
        self.steps
            .iter()
            .map(|s| format!("  {}", s.tactic))
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Check if the proof is complete.
    pub fn is_complete(&self) -> bool {
        if self.steps.is_empty() {
            self.initial_state.is_complete()
        } else {
            self.steps
                .last()
                .expect("steps is non-empty: checked by is_empty guard")
                .state_after
                .is_complete()
        }
    }
}
/// Hypothesis inspector.
#[allow(dead_code)]
pub struct HypothesisInspector;
#[allow(dead_code)]
impl HypothesisInspector {
    /// Inspect a specific hypothesis.
    pub fn inspect(goal: &Goal, name: &Name) -> Option<HypothesisInfo> {
        for (n, ty) in goal.hypotheses() {
            if n == name {
                let usage_count = Self::count_usage_in_goal(&name.to_string(), goal.target());
                return Some(HypothesisInfo {
                    name: name.clone(),
                    ty: ty.clone(),
                    value: None,
                    is_let: false,
                    usage_count,
                });
            }
        }
        for (n, ty, val) in &goal.local_ctx {
            if n == name {
                let usage_count = Self::count_usage_in_goal(&name.to_string(), goal.target());
                return Some(HypothesisInfo {
                    name: name.clone(),
                    ty: ty.clone(),
                    value: val.clone(),
                    is_let: true,
                    usage_count,
                });
            }
        }
        None
    }
    /// List all hypotheses with their information.
    /// Uses the hypotheses list, enriching with values from local_ctx when available.
    pub fn list_all(goal: &Goal) -> Vec<HypothesisInfo> {
        let mut infos = Vec::new();
        for (n, ty) in goal.hypotheses() {
            let usage_count = Self::count_usage_in_goal(&n.to_string(), goal.target());
            let value = goal
                .local_ctx
                .iter()
                .find(|(ln, _, _)| ln == n)
                .and_then(|(_, _, v)| v.clone());
            let is_let = value.is_some();
            infos.push(HypothesisInfo {
                name: n.clone(),
                ty: ty.clone(),
                value,
                is_let,
                usage_count,
            });
        }
        infos
    }
    /// Format hypothesis info for display.
    pub fn format_info(info: &HypothesisInfo) -> String {
        let mut output = String::new();
        output.push_str(&format!("Name:       {}\n", info.name));
        output.push_str(&format!("Type:       {}\n", print_expr(&info.ty)));
        if let Some(val) = &info.value {
            output.push_str(&format!("Value:      {}\n", print_expr(val)));
        }
        output.push_str(&format!(
            "Kind:       {}\n",
            if info.is_let {
                "let-binding"
            } else {
                "hypothesis"
            }
        ));
        output.push_str(&format!("Uses in goal: {}\n", info.usage_count));
        output
    }
    /// Count how many times a name appears in an expression.
    fn count_usage_in_goal(name: &str, expr: &Expr) -> usize {
        print_expr(expr).matches(name).count()
    }
}
/// Reason for a tactic suggestion.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SuggestionReason {
    /// A hypothesis exactly matches the target.
    HypothesisMatches,
    /// The target is a function type, so intro is applicable.
    TargetIsPi,
    /// The target is an inductive type, so constructor/cases might work.
    TargetIsInductive,
    /// The target is a reflexivity goal.
    TargetIsRefl,
    /// The target is True, which is trivially provable.
    TargetIsTrue,
    /// A hypothesis has an inductive type that can be case-split.
    HypothesisInductive,
    /// General suggestion without specific reason.
    General,
}
/// Statistics about a proof session.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProofSessionStats {
    /// Number of tactics applied so far.
    pub tactics_applied: usize,
    /// Number of undo operations.
    pub undos: usize,
    /// Number of goals created during proof.
    pub goals_created: usize,
    /// Number of goals solved.
    pub goals_solved: usize,
    /// Total time spent on the proof (in milliseconds).
    pub total_time_ms: u128,
    /// Time of session start.
    pub start_time: Instant,
}
/// Goal display utility.
#[allow(dead_code)]
pub struct GoalDisplay {
    /// Display configuration.
    pub(crate) config: GoalDisplayConfig,
}
#[allow(dead_code)]
impl GoalDisplay {
    /// Create a new goal display with default config.
    pub fn new() -> Self {
        Self {
            config: GoalDisplayConfig::default(),
        }
    }
    /// Create a goal display with custom config.
    pub fn with_config(config: GoalDisplayConfig) -> Self {
        Self { config }
    }
    /// Format a single goal with its hypotheses.
    pub fn format_goal(&self, goal: &Goal) -> String {
        let mut output = String::new();
        if self.config.show_tags {
            if let Some(tag) = &goal.tag {
                output.push_str(&format!("case {}\n", tag));
            }
        }
        for (name, ty) in goal.hypotheses() {
            let ty_str = self.truncate_type(&print_expr(ty));
            output.push_str(&format!("{} : {}\n", name, ty_str));
        }
        if self.config.show_let_values {
            for (name, ty, val) in &goal.local_ctx {
                if let Some(v) = val {
                    let already_shown = goal.hypotheses().iter().any(|(n, _)| n == name);
                    if !already_shown {
                        output.push_str(&format!(
                            "{} : {} := {}\n",
                            name,
                            self.truncate_type(&print_expr(ty)),
                            self.truncate_type(&print_expr(v))
                        ));
                    }
                }
            }
        }
        let sep_len = 40;
        output.push_str(&"-".repeat(sep_len));
        output.push('\n');
        let target_str = print_expr(goal.target());
        output.push_str(&format!("{} {}", goal.name, target_str));
        if self.config.show_mvar_ids {
            output.push_str(&format!(" [mvar_id={}]", goal.mvar_id));
        }
        output
    }
    /// Format a single hypothesis.
    pub fn format_hypothesis(&self, name: &Name, ty: &Expr) -> String {
        let ty_str = self.truncate_type(&print_expr(ty));
        format!("{} : {}", name, ty_str)
    }
    /// Format a panel showing all current goals.
    pub fn format_goals_panel(&self, state: &TacticState) -> String {
        if state.is_complete() {
            return "Goals accomplished! No remaining goals.".to_string();
        }
        let goals = state.goals();
        let mut output = String::new();
        let total = goals.len();
        for (i, goal) in goals.iter().enumerate() {
            if i > 0 {
                output.push_str("\n\n");
            }
            output.push_str(&format!("Goal {}/{}\n", i + 1, total));
            output.push_str(&self.format_goal(goal));
        }
        output
    }
    /// Truncate a type string if it exceeds max width.
    pub fn truncate_type(&self, type_str: &str) -> String {
        if type_str.len() <= self.config.max_type_width {
            type_str.to_string()
        } else {
            let truncated = &type_str[..self.config.max_type_width.saturating_sub(3)];
            format!("{}...", truncated)
        }
    }
    /// Highlight differences between two goal states.
    pub fn highlight_differences(&self, before: &TacticState, after: &TacticState) -> String {
        let mut output = String::new();
        let before_count = before.num_goals();
        let after_count = after.num_goals();
        if after_count == 0 && before_count > 0 {
            output.push_str("All goals solved!\n");
            return output;
        }
        if after_count > before_count {
            output.push_str(&format!(
                "Created {} new goal(s) ({} -> {})\n",
                after_count - before_count,
                before_count,
                after_count
            ));
        } else if after_count < before_count {
            output.push_str(&format!(
                "Solved {} goal(s) ({} -> {})\n",
                before_count - after_count,
                before_count,
                after_count
            ));
        }
        if let Some(goal) = after.goals().first() {
            output.push_str("\nCurrent goal:\n");
            output.push_str(&self.format_goal(goal));
        }
        output
    }
}
/// LSP integration for IDE support.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LspIntegration {
    /// Whether LSP is enabled.
    pub(crate) enabled: bool,
    /// Server URI (if connected).
    server_uri: Option<String>,
}
#[allow(dead_code)]
impl LspIntegration {
    /// Create a new LSP integration.
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            server_uri: None,
        }
    }
    /// Connect to an LSP server.
    pub fn connect(&mut self, uri: String) -> Result<(), String> {
        if !self.enabled {
            return Err("LSP is disabled".to_string());
        }
        self.server_uri = Some(uri);
        Ok(())
    }
    /// Get hover information for a name.
    ///
    /// Looks up the name in a built-in documentation table and returns a
    /// short description.  Unknown names produce a generic fallback message.
    pub fn get_hover_info(&self, name: &str) -> Result<String, String> {
        if self.server_uri.is_none() {
            return Err("Not connected to LSP server".to_string());
        }
        let info = match name {
            "theorem" | "lemma" => {
                "**theorem** / **lemma** — Declare a theorem that must be proved."
            }
            "def" => "**def** — Define a term (function, constant, etc.).",
            "axiom" => "**axiom** — Postulate a proposition without proof.",
            "inductive" => "**inductive** — Declare an inductive type.",
            "structure" => "**structure** — Declare a record-like type.",
            "class" => "**class** — Declare a type-class.",
            "instance" => "**instance** — Provide a type-class instance.",
            "fun" => "**fun** — Lambda abstraction: `fun x -> body`.",
            "forall" | "∀" => "**forall** — Universal quantifier: `forall (x : T), P x`.",
            "exists" | "∃" => "**exists** — Existential quantifier: `exists (x : T), P x`.",
            "let" => "**let** — Local definition: `let x := e; body`.",
            "match" => "**match** — Pattern match: `match e with | pat -> body`.",
            "intro" | "intros" => {
                "**intro** / **intros** tactic — Introduce hypotheses into the context."
            }
            "exact" => "**exact** tactic — Close the goal with a given term.",
            "apply" => "**apply** tactic — Apply a lemma, unifying the conclusion with the goal.",
            "rewrite" | "rw" => "**rw** / **rewrite** tactic — Rewrite the goal using an equality.",
            "simp" => "**simp** tactic — Simplify using rewrite rules and beta reduction.",
            "assumption" => "**assumption** tactic — Close the goal using a matching hypothesis.",
            "ring" => "**ring** tactic — Prove ring equalities by normalisation.",
            "linarith" => "**linarith** tactic — Prove linear arithmetic goals.",
            "cases" => "**cases** tactic — Case split on an inductive hypothesis.",
            "induction" => "**induction** tactic — Apply structural induction.",
            "constructor" => "**constructor** tactic — Apply the first applicable constructor.",
            "refl" => "**refl** tactic — Close `a = a` goals by reflexivity.",
            "sorry" => "**sorry** tactic — Admit the goal (placeholder; not for production).",
            "Nat" => "**Nat** — The type of natural numbers (0, 1, 2, ...).",
            "Int" => "**Int** — The type of integers (..., -1, 0, 1, ...).",
            "Bool" => "**Bool** — The boolean type with constructors `true` and `false`.",
            "List" => "**List** — Homogeneous linked-list type.",
            "Option" => "**Option** — Optional value: `none` or `some x`.",
            "Prop" => "**Prop** — The sort of propositions.",
            "Type" => "**Type** — The sort of small types (`Type 0`, `Type 1`, ...).",
            _ => {
                return Ok(
                    format!(
                        "No documentation available for `{}`.                      Try `oxilean check` or consult the standard library.",
                        name
                    ),
                );
            }
        };
        Ok(info.to_string())
    }
    /// Get completion suggestions from LSP.
    ///
    /// Returns keywords, tactics, and built-in names that share the given
    /// prefix.  When connected to an LSP server the same list is returned
    /// (a live server connection is not required for static completions).
    pub fn get_completions(&self, prefix: &str) -> Result<Vec<String>, String> {
        if self.server_uri.is_none() {
            return Err("Not connected to LSP server".to_string());
        }
        const KEYWORDS: &[&str] = &[
            "theorem",
            "def",
            "axiom",
            "lemma",
            "example",
            "inductive",
            "structure",
            "class",
            "instance",
            "namespace",
            "end",
            "section",
            "variable",
            "open",
            "import",
            "export",
            "fun",
            "let",
            "in",
            "match",
            "with",
            "forall",
            "exists",
            "have",
            "show",
            "from",
            "if",
            "then",
            "else",
            "do",
            "return",
            "Type",
            "Prop",
            "Sort",
            "Bool",
            "Nat",
            "Int",
            "String",
            "List",
            "Option",
            "Result",
            "intro",
            "intros",
            "exact",
            "apply",
            "rw",
            "rewrite",
            "simp",
            "assumption",
            "constructor",
            "cases",
            "induction",
            "obtain",
            "have",
            "show",
            "use",
            "exists",
            "left",
            "right",
            "split",
            "refl",
            "trivial",
            "sorry",
            "ring",
            "linarith",
            "norm_num",
            "push_neg",
            "by_contra",
            "by_contradiction",
            "contrapose",
            "exfalso",
            "clear",
            "rename",
            "revert",
            "repeat",
            "first",
            "try",
            "all_goals",
            "field_simp",
            "simp_all",
            "Eq",
            "And",
            "Or",
            "Not",
            "True",
            "False",
            "Iff",
            "Nat.add",
            "Nat.mul",
            "Nat.sub",
            "Nat.zero",
            "Nat.succ",
            "List.nil",
            "List.cons",
            "List.append",
            "List.map",
            "List.length",
            "List.head",
            "List.tail",
            "Option.none",
            "Option.some",
        ];
        let lower_prefix = prefix.to_lowercase();
        let mut completions: Vec<String> = KEYWORDS
            .iter()
            .filter(|kw| kw.to_lowercase().starts_with(&lower_prefix))
            .map(|kw| kw.to_string())
            .collect();
        let mut seen = std::collections::HashSet::new();
        completions.retain(|c| seen.insert(c.clone()));
        completions.sort_by(|a, b| {
            let a_exact = a.starts_with(prefix);
            let b_exact = b.starts_with(prefix);
            match (a_exact, b_exact) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.cmp(b),
            }
        });
        Ok(completions)
    }
    /// Is LSP enabled and connected.
    pub fn is_connected(&self) -> bool {
        self.enabled && self.server_uri.is_some()
    }
}
/// A node in the proof tree.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProofTreeNode {
    /// The tactic applied at this node.
    pub tactic: String,
    /// The goal name.
    pub goal_name: String,
    /// Child nodes (sub-goals created).
    pub children: Vec<ProofTreeNode>,
    /// Whether this branch is complete.
    pub is_complete: bool,
}
