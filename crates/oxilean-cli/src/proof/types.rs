//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{collect_const_names, Declaration, Environment, Expr, Name, TypeChecker};

use std::collections::HashMap;

/// A record associating a theorem name with its proof status.
#[derive(Clone, Debug)]
pub struct ProofRecord {
    /// Name of the theorem.
    pub name: Name,
    /// Verification status.
    pub status: ProofStatus,
    /// Optional extra detail.
    pub detail: Option<String>,
}
impl ProofRecord {
    /// Create a new record in the `Unchecked` state.
    pub fn new(name: Name) -> Self {
        Self {
            name,
            status: ProofStatus::Unchecked,
            detail: None,
        }
    }
    /// Mark as verified.
    pub fn mark_verified(mut self) -> Self {
        self.status = ProofStatus::Verified;
        self
    }
    /// Mark as partial.
    pub fn mark_partial(mut self) -> Self {
        self.status = ProofStatus::Partial;
        self
    }
    /// Mark as failed.
    pub fn mark_failed(mut self, msg: impl Into<String>) -> Self {
        self.status = ProofStatus::Failed(msg.into());
        self
    }
    /// Attach extra detail.
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}
/// Supported proof export formats.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExportFormat {
    /// Plain text.
    Text,
    /// JSON.
    Json,
    /// Lean 4 source file.
    Lean4,
    /// Markdown.
    Markdown,
}
impl ExportFormat {
    /// Infer format from file extension.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_ascii_lowercase().as_str() {
            "txt" => Some(ExportFormat::Text),
            "json" => Some(ExportFormat::Json),
            "lean" => Some(ExportFormat::Lean4),
            "md" => Some(ExportFormat::Markdown),
            _ => None,
        }
    }
    /// Default file extension.
    pub fn extension(&self) -> &'static str {
        match self {
            ExportFormat::Text => "txt",
            ExportFormat::Json => "json",
            ExportFormat::Lean4 => "lean",
            ExportFormat::Markdown => "md",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProofStatusV2 {
    Pending,
    InProgress,
    Complete,
    Failed(String),
    Abandoned,
}
#[allow(dead_code)]
impl ProofStatusV2 {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            ProofStatusV2::Complete | ProofStatusV2::Failed(_) | ProofStatusV2::Abandoned
        )
    }
    pub fn display(&self) -> &str {
        match self {
            ProofStatusV2::Pending => "pending",
            ProofStatusV2::InProgress => "in-progress",
            ProofStatusV2::Complete => "complete",
            ProofStatusV2::Failed(_) => "failed",
            ProofStatusV2::Abandoned => "abandoned",
        }
    }
}
/// Proof checker for verifying theorems and lemmas.
pub struct ProofChecker<'env> {
    /// The environment containing all declarations
    env: &'env Environment,
}
impl<'env> ProofChecker<'env> {
    /// Create a new proof checker.
    pub fn new(env: &'env Environment) -> Self {
        Self { env }
    }
    /// Check if a proof is valid for a given theorem.
    pub fn check_proof(&self, theorem: &Name, proof: &Expr) -> Result<(), String> {
        let decl = self
            .env
            .get(theorem)
            .ok_or_else(|| format!("Theorem {} not found", theorem))?;
        let ty = match decl {
            Declaration::Axiom { ty, .. } => ty,
            Declaration::Definition { ty, .. } => ty,
            Declaration::Theorem { ty, .. } => ty,
            Declaration::Opaque { ty, .. } => ty,
        };
        let mut tc = TypeChecker::new(self.env);
        let inferred_ty = tc.infer_type(proof).map_err(|e| e.to_string())?;
        if tc.is_def_eq(&inferred_ty, ty) {
            Ok(())
        } else {
            Err(format!(
                "Proof type mismatch: expected {:?}, got {:?}",
                ty, inferred_ty
            ))
        }
    }
    /// Verify all theorems in the environment.
    ///
    /// Iterates over every theorem, definition, and opaque declaration in the
    /// environment, type-checks its proof term against its stated type using
    /// the kernel type-checker, and collects the names of any declarations
    /// that fail verification.  Axioms are skipped because they have no proof
    /// term to verify.
    ///
    /// Returns `Ok(failed_names)` where `failed_names` is empty when every
    /// declaration type-checks successfully.
    pub fn verify_all(&self) -> Result<Vec<Name>, String> {
        let mut failed = Vec::new();
        let names: Vec<Name> = self.env.constant_names().cloned().collect();
        for name in &names {
            let decl = match self.env.get(name) {
                Some(d) => d,
                None => continue,
            };
            let (ty, val) = match decl {
                Declaration::Axiom { .. } => continue,
                Declaration::Theorem { ty, val, .. } => (ty, val),
                Declaration::Definition { ty, val, .. } => (ty, val),
                Declaration::Opaque { ty, val, .. } => (ty, val),
            };
            let has_sorry = collect_const_names(val)
                .iter()
                .any(|n| n.to_string().contains("sorry"));
            if has_sorry {
                failed.push(name.clone());
                continue;
            }
            let mut tc = TypeChecker::new(self.env);
            match tc.infer_type(val) {
                Err(_) => {
                    failed.push(name.clone());
                }
                Ok(inferred) => {
                    if !tc.is_def_eq(&inferred, ty) {
                        failed.push(name.clone());
                    }
                }
            }
        }
        Ok(failed)
    }
    /// Get the environment.
    pub fn env(&self) -> &Environment {
        self.env
    }
    /// Count the total number of declarations.
    pub fn declaration_count(&self) -> usize {
        self.env.constant_names().count()
    }
    /// Collect all theorem names.
    pub fn theorem_names(&self) -> Vec<Name> {
        self.env
            .constant_names()
            .filter(|name| matches!(self.env.get(name), Some(Declaration::Theorem { .. })))
            .cloned()
            .collect()
    }
    /// Collect all axiom names.
    pub fn axiom_names(&self) -> Vec<Name> {
        self.env
            .constant_names()
            .filter(|name| matches!(self.env.get(name), Some(Declaration::Axiom { .. })))
            .cloned()
            .collect()
    }
    /// Collect all definition names.
    pub fn definition_names(&self) -> Vec<Name> {
        self.env
            .constant_names()
            .filter(|name| matches!(self.env.get(name), Some(Declaration::Definition { .. })))
            .cloned()
            .collect()
    }
    /// Check whether a name is declared.
    pub fn is_declared(&self, name: &Name) -> bool {
        self.env.get(name).is_some()
    }
    /// Retrieve the type of a declared name.
    pub fn get_type(&self, name: &Name) -> Option<&Expr> {
        match self.env.get(name)? {
            Declaration::Axiom { ty, .. } => Some(ty),
            Declaration::Definition { ty, .. } => Some(ty),
            Declaration::Theorem { ty, .. } => Some(ty),
            Declaration::Opaque { ty, .. } => Some(ty),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProofAttemptLog {
    pub attempts: Vec<ProofAttempt>,
    pub next_id: u64,
}
#[allow(dead_code)]
impl ProofAttemptLog {
    pub fn new() -> Self {
        Self {
            attempts: Vec::new(),
            next_id: 1,
        }
    }
    pub fn start_attempt(&mut self, timestamp_ms: u64) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.attempts.push(ProofAttempt::new(id, timestamp_ms));
        id
    }
    pub fn get_attempt_mut(&mut self, id: u64) -> Option<&mut ProofAttempt> {
        self.attempts.iter_mut().find(|a| a.id == id)
    }
    pub fn successful_attempts(&self) -> Vec<&ProofAttempt> {
        self.attempts
            .iter()
            .filter(|a| a.status == ProofStatusV2::Complete)
            .collect()
    }
    pub fn failed_attempts(&self) -> Vec<&ProofAttempt> {
        self.attempts
            .iter()
            .filter(|a| matches!(a.status, ProofStatusV2::Failed(_)))
            .collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProofAnnotationV2 {
    pub step_id: usize,
    pub label: String,
    pub note: String,
}
#[allow(dead_code)]
impl ProofAnnotationV2 {
    pub fn new(step_id: usize, label: &str, note: &str) -> Self {
        Self {
            step_id,
            label: label.to_string(),
            note: note.to_string(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ProofComplexityMetrics {
    pub num_steps: usize,
    pub max_goal_depth: usize,
    pub branching_factor: f64,
    pub unique_tactics: usize,
    pub has_sorry: bool,
    pub proof_length_chars: usize,
}
/// A simplified proof goal for display.
#[derive(Clone, Debug)]
pub struct DisplayGoal {
    /// Goal index (1-based).
    pub index: usize,
    /// Hypotheses as (name, type) pairs.
    pub hypotheses: Vec<(String, String)>,
    /// The goal type.
    pub goal_type: String,
}
impl DisplayGoal {
    /// Create a new display goal.
    pub fn new(index: usize, goal_type: impl Into<String>) -> Self {
        Self {
            index,
            hypotheses: vec![],
            goal_type: goal_type.into(),
        }
    }
    /// Add a hypothesis.
    pub fn add_hyp(&mut self, name: impl Into<String>, ty: impl Into<String>) {
        self.hypotheses.push((name.into(), ty.into()));
    }
}
/// A single step in a proof trace.
#[derive(Clone, Debug)]
pub struct ProofStep {
    /// Step number.
    pub step: usize,
    /// Tactic applied.
    pub tactic: String,
    /// Goals before.
    pub goals_before: usize,
    /// Goals after.
    pub goals_after: usize,
    /// Whether successful.
    pub success: bool,
}
impl ProofStep {
    /// Create a proof step.
    pub fn new(
        step: usize,
        tactic: impl Into<String>,
        goals_before: usize,
        goals_after: usize,
        success: bool,
    ) -> Self {
        Self {
            step,
            tactic: tactic.into(),
            goals_before,
            goals_after,
            success,
        }
    }
    /// Did this step close all goals?
    pub fn closed_all(&self) -> bool {
        self.goals_after == 0
    }
    /// Goals eliminated.
    pub fn goals_eliminated(&self) -> isize {
        self.goals_before as isize - self.goals_after as isize
    }
}
/// An annotation attached to a proof step for documentation purposes.
#[derive(Clone, Debug)]
pub struct ProofAnnotation {
    /// Step index being annotated.
    pub step: usize,
    /// The annotation text.
    pub text: String,
    /// Kind of annotation.
    pub kind: AnnotationKind,
}
impl ProofAnnotation {
    /// Create a new annotation.
    pub fn new(step: usize, text: impl Into<String>, kind: AnnotationKind) -> Self {
        Self {
            step,
            text: text.into(),
            kind,
        }
    }
    /// Create a note annotation.
    pub fn note(step: usize, text: impl Into<String>) -> Self {
        Self::new(step, text, AnnotationKind::Note)
    }
    /// Create a warning annotation.
    pub fn warning(step: usize, text: impl Into<String>) -> Self {
        Self::new(step, text, AnnotationKind::Warning)
    }
}
/// A proof session that accumulates steps and records progress.
pub struct ProofSession {
    /// Theorem being proved.
    pub theorem_name: Name,
    /// Steps performed.
    pub steps: Vec<ProofStep>,
    /// Goal progress.
    pub progress: ProofProgress,
    /// Session status.
    pub status: ProofStatus,
}
impl ProofSession {
    /// Create a new proof session for a theorem.
    pub fn new(theorem_name: Name, initial_goals: usize) -> Self {
        let mut progress = ProofProgress::new();
        progress.record(initial_goals);
        Self {
            theorem_name,
            steps: Vec::new(),
            progress,
            status: ProofStatus::Unchecked,
        }
    }
    /// Apply a tactic step.
    pub fn apply_step(&mut self, tactic: impl Into<String>, goals_after: usize, success: bool) {
        let step_num = self.steps.len() + 1;
        let goals_before = self.progress.final_goals().unwrap_or(0);
        let step = ProofStep::new(step_num, tactic, goals_before, goals_after, success);
        self.steps.push(step);
        self.progress.record(goals_after);
        if goals_after == 0 && success {
            self.status = ProofStatus::Verified;
        }
    }
    /// Number of steps taken.
    pub fn num_steps(&self) -> usize {
        self.steps.len()
    }
    /// Check if the session is complete.
    pub fn is_complete(&self) -> bool {
        self.progress.is_complete()
    }
    /// Produce a summary record reflecting the current session status.
    pub fn to_record(&self) -> ProofRecord {
        let base = ProofRecord::new(self.theorem_name.clone())
            .with_detail(format!("{} steps", self.num_steps()));
        match &self.status {
            ProofStatus::Verified => base.mark_verified(),
            ProofStatus::Partial => base.mark_partial(),
            ProofStatus::Failed(msg) => base.mark_failed(msg.clone()),
            ProofStatus::Unchecked => base,
        }
    }
    /// Current goal count.
    pub fn current_goals(&self) -> usize {
        self.progress.final_goals().unwrap_or(0)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProofHint {
    pub tactic: String,
    pub rationale: String,
    pub confidence: f64,
}
#[allow(dead_code)]
impl ProofHint {
    pub fn new(tactic: &str, rationale: &str, confidence: f64) -> Self {
        Self {
            tactic: tactic.to_string(),
            rationale: rationale.to_string(),
            confidence: confidence.clamp(0.0, 1.0),
        }
    }
    pub fn display(&self) -> String {
        format!(
            "Try: {} ({}) [conf: {:.0}%]",
            self.tactic,
            self.rationale,
            self.confidence * 100.0
        )
    }
}
/// Kind of proof annotation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnnotationKind {
    /// Informal proof sketch note.
    Note,
    /// Mathematical motivation.
    Motivation,
    /// Reference to an external theorem.
    Reference,
    /// Warning about a subtle step.
    Warning,
}
#[allow(dead_code)]
pub struct ProofReplayResult {
    pub replayed_steps: usize,
    pub failed_at: Option<usize>,
    pub succeeded: bool,
}
/// Options for proof checking.
#[derive(Clone, Debug)]
pub struct ProofOptions {
    /// Treat `sorry` as an error.
    pub sorry_is_error: bool,
    /// Maximum proof term depth.
    pub max_depth: usize,
    /// Collect a full trace.
    pub trace: bool,
    /// Verbose output.
    pub verbose: bool,
}
impl ProofOptions {
    /// Strict options (sorry is error).
    pub fn strict() -> Self {
        Self {
            sorry_is_error: true,
            ..Default::default()
        }
    }
    /// Verbose options.
    pub fn verbose() -> Self {
        Self {
            verbose: true,
            ..Default::default()
        }
    }
}
#[allow(dead_code)]
pub struct ProofGoalHistory {
    snapshots: Vec<GoalSnapshot>,
}
#[allow(dead_code)]
impl ProofGoalHistory {
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }
    pub fn record(&mut self, step_id: usize, goal_count: usize, description: &str) {
        self.snapshots.push(GoalSnapshot {
            step_id,
            goal_count,
            description: description.to_string(),
        });
    }
    pub fn min_goals(&self) -> usize {
        self.snapshots
            .iter()
            .map(|s| s.goal_count)
            .min()
            .unwrap_or(0)
    }
    pub fn max_goals(&self) -> usize {
        self.snapshots
            .iter()
            .map(|s| s.goal_count)
            .max()
            .unwrap_or(0)
    }
    pub fn goal_at(&self, step_id: usize) -> Option<usize> {
        self.snapshots
            .iter()
            .filter(|s| s.step_id <= step_id)
            .last()
            .map(|s| s.goal_count)
    }
    pub fn trend(&self) -> Vec<(usize, usize)> {
        self.snapshots
            .iter()
            .map(|s| (s.step_id, s.goal_count))
            .collect()
    }
}
#[allow(dead_code)]
pub struct TacticSuggestion {
    pub tactic: String,
    pub reason: String,
    pub confidence: f64,
}
#[allow(dead_code)]
impl TacticSuggestion {
    pub fn new(tactic: &str, reason: &str, confidence: f64) -> Self {
        Self {
            tactic: tactic.to_string(),
            reason: reason.to_string(),
            confidence,
        }
    }
}
#[allow(dead_code)]
pub struct AnnotatedProof {
    pub steps: Vec<ProofStep>,
    pub annotations: Vec<ProofAnnotationV2>,
}
#[allow(dead_code)]
impl AnnotatedProof {
    pub fn new(steps: Vec<ProofStep>) -> Self {
        Self {
            steps,
            annotations: Vec::new(),
        }
    }
    pub fn annotate(&mut self, step_id: usize, label: &str, note: &str) {
        self.annotations
            .push(ProofAnnotationV2::new(step_id, label, note));
    }
    pub fn annotations_for(&self, step_id: usize) -> Vec<&ProofAnnotationV2> {
        self.annotations
            .iter()
            .filter(|a| a.step_id == step_id)
            .collect()
    }
    pub fn to_markdown(&self) -> String {
        let mut out = String::from("# Proof\n\n");
        for step in &self.steps {
            let annots: Vec<&ProofAnnotationV2> = self.annotations_for(step.step);
            out.push_str(&format!("**Step {}**: `{}`  \n", step.step, step.tactic));
            for ann in annots {
                out.push_str(&format!("> *{}*: {}  \n", ann.label, ann.note));
            }
            out.push('\n');
        }
        out
    }
}
#[allow(dead_code)]
pub struct ProofDepGraph {
    deps: std::collections::HashMap<usize, Vec<usize>>,
}
#[allow(dead_code)]
impl ProofDepGraph {
    pub fn new() -> Self {
        Self {
            deps: std::collections::HashMap::new(),
        }
    }
    pub fn add_dep(&mut self, step: usize, dep: usize) {
        self.deps.entry(step).or_default().push(dep);
    }
    pub fn deps_of(&self, step: usize) -> &[usize] {
        self.deps.get(&step).map(|v| v.as_slice()).unwrap_or(&[])
    }
    pub fn is_independent(&self, step: usize) -> bool {
        self.deps_of(step).is_empty()
    }
    pub fn topological_order(&self, steps: &[usize]) -> Vec<usize> {
        let mut in_degree: std::collections::HashMap<usize, usize> =
            steps.iter().map(|&s| (s, 0)).collect();
        for (_step, deps) in &self.deps {
            for &dep in deps {
                *in_degree.entry(dep).or_default() += 0;
            }
            *in_degree.entry(*_step).or_default() += deps.len();
        }
        let mut queue: std::collections::VecDeque<usize> = in_degree
            .iter()
            .filter(|(_, &d)| d == 0)
            .map(|(&s, _)| s)
            .collect();
        let mut order = Vec::new();
        while let Some(s) = queue.pop_front() {
            order.push(s);
        }
        order
    }
}
#[allow(dead_code)]
pub struct ProofSearchState {
    pub goals_remaining: usize,
    pub trace: ProofTrace,
    pub depth: usize,
}
#[allow(dead_code)]
impl ProofSearchState {
    pub fn initial(goals: usize) -> Self {
        Self {
            goals_remaining: goals,
            trace: ProofTrace::new(),
            depth: 0,
        }
    }
    pub fn apply_tactic(&self, tactic: &str, goals_after: usize) -> Self {
        let mut new_trace = ProofTrace::new();
        for step in self.trace.steps() {
            new_trace.push(step.clone());
        }
        new_trace.push(ProofStep::new(
            self.trace.steps().len(),
            tactic,
            self.goals_remaining,
            goals_after,
            true,
        ));
        Self {
            goals_remaining: goals_after,
            trace: new_trace,
            depth: self.depth + 1,
        }
    }
    pub fn is_complete(&self) -> bool {
        self.goals_remaining == 0
    }
}
/// Batch proof checker.
pub struct BatchChecker<'env> {
    checker: ProofChecker<'env>,
    options: ProofOptions,
}
impl<'env> BatchChecker<'env> {
    /// Create with default options.
    pub fn new(env: &'env Environment) -> Self {
        Self {
            checker: ProofChecker::new(env),
            options: ProofOptions::default(),
        }
    }
    /// Set options.
    pub fn with_options(mut self, options: ProofOptions) -> Self {
        self.options = options;
        self
    }
    /// Check a slice of names by verifying their proof terms against their types.
    ///
    /// For each name:
    /// - If not declared, the record is marked failed.
    /// - If declared as an axiom, the record is marked verified (no proof to check).
    /// - If the proof term references `sorry`, the record is marked partial.
    /// - Otherwise the proof term is type-checked; failure marks the record failed.
    pub fn check_names(&self, names: &[Name]) -> ProofSummary {
        let mut summary = ProofSummary::new();
        for name in names {
            let record = match self.checker.env().get(name) {
                None => ProofRecord::new(name.clone()).mark_failed("not declared in environment"),
                Some(Declaration::Axiom { .. }) => ProofRecord::new(name.clone()).mark_verified(),
                Some(decl) => {
                    let (ty, val) = match decl {
                        Declaration::Theorem { ty, val, .. } => (ty, val),
                        Declaration::Definition { ty, val, .. } => (ty, val),
                        Declaration::Opaque { ty, val, .. } => (ty, val),
                        Declaration::Axiom { .. } => unreachable!(),
                    };
                    let has_sorry = collect_const_names(val)
                        .iter()
                        .any(|n| n.to_string().contains("sorry"));
                    if has_sorry {
                        ProofRecord::new(name.clone()).mark_partial()
                    } else {
                        let mut tc = TypeChecker::new(self.checker.env());
                        match tc.infer_type(val) {
                            Err(e) => ProofRecord::new(name.clone())
                                .mark_failed(format!("type error: {}", e)),
                            Ok(inferred) => {
                                if tc.is_def_eq(&inferred, ty) {
                                    ProofRecord::new(name.clone()).mark_verified()
                                } else {
                                    ProofRecord::new(name.clone())
                                        .mark_failed("proof type does not match declared type")
                                }
                            }
                        }
                    }
                }
            };
            if self.options.verbose {
                let record = if record.status.is_partial() {
                    record.with_detail("contains sorry")
                } else {
                    record
                };
                summary.add(record);
            } else {
                summary.add(record);
            }
        }
        summary
    }
    /// Check all theorems in the environment.
    pub fn check_all_theorems(&self) -> ProofSummary {
        let names = self.checker.theorem_names();
        self.check_names(&names)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProofValidationError {
    pub step_id: usize,
    pub message: String,
}
/// The verification status of a single proof.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProofStatus {
    /// The proof was accepted.
    Verified,
    /// The proof contains `sorry`.
    Partial,
    /// The proof was rejected.
    Failed(String),
    /// The proof has not been checked.
    Unchecked,
}
impl ProofStatus {
    /// Return `true` if verified.
    pub fn is_verified(&self) -> bool {
        matches!(self, ProofStatus::Verified)
    }
    /// Return `true` if failed.
    pub fn is_failed(&self) -> bool {
        matches!(self, ProofStatus::Failed(_))
    }
    /// Return `true` if partial.
    pub fn is_partial(&self) -> bool {
        matches!(self, ProofStatus::Partial)
    }
    /// Return `true` if unchecked.
    pub fn is_unchecked(&self) -> bool {
        matches!(self, ProofStatus::Unchecked)
    }
}
/// Track proof progress over time.
#[derive(Clone, Debug, Default)]
pub struct ProofProgress {
    /// Snapshots of goal counts at each step.
    snapshots: Vec<usize>,
}
impl ProofProgress {
    /// Create a new progress tracker.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record the current goal count.
    pub fn record(&mut self, goals: usize) {
        self.snapshots.push(goals);
    }
    /// Return the initial goal count, if any.
    pub fn initial_goals(&self) -> Option<usize> {
        self.snapshots.first().copied()
    }
    /// Return the final goal count, if any.
    pub fn final_goals(&self) -> Option<usize> {
        self.snapshots.last().copied()
    }
    /// Return true if the proof is complete (ended at 0 goals).
    pub fn is_complete(&self) -> bool {
        self.final_goals() == Some(0)
    }
    /// Return the number of recorded snapshots.
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }
    /// Total goals eliminated (initial - final).
    pub fn goals_eliminated(&self) -> isize {
        match (self.initial_goals(), self.final_goals()) {
            (Some(init), Some(fin)) => init as isize - fin as isize,
            _ => 0,
        }
    }
    /// Return all snapshots as a slice.
    pub fn snapshots(&self) -> &[usize] {
        &self.snapshots
    }
}
#[allow(dead_code)]
pub struct ProofSessionV2 {
    pub theorem_name: String,
    pub trace: ProofTrace,
    pub goal_count: usize,
    pub session_start: std::time::Instant,
    pub session_complete: bool,
}
#[allow(dead_code)]
impl ProofSessionV2 {
    pub fn start(theorem_name: &str, initial_goals: usize) -> Self {
        Self {
            theorem_name: theorem_name.to_string(),
            trace: ProofTrace::new(),
            goal_count: initial_goals,
            session_start: std::time::Instant::now(),
            session_complete: false,
        }
    }
    pub fn apply(&mut self, tactic: &str, goals_after: usize) {
        let step_id = self.trace.steps().len();
        let goals_before = self.goal_count;
        let succeeded = true;
        self.trace.push(ProofStep::new(
            step_id,
            tactic,
            goals_before,
            goals_after,
            succeeded,
        ));
        self.goal_count = goals_after;
        if goals_after == 0 {
            self.session_complete = true;
        }
    }
    pub fn elapsed_secs(&self) -> f64 {
        self.session_start.elapsed().as_secs_f64()
    }
    pub fn summary(&self) -> String {
        format!(
            "Session: {} | Steps: {} | Goals: {} | Complete: {} | Elapsed: {:.2}s",
            self.theorem_name,
            self.trace.steps().len(),
            self.goal_count,
            self.session_complete,
            self.elapsed_secs()
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GoalSnapshot {
    pub step_id: usize,
    pub goal_count: usize,
    pub description: String,
}
/// A summary of a batch proof-checking run.
#[derive(Clone, Debug, Default)]
pub struct ProofSummary {
    pub(super) records: Vec<ProofRecord>,
}
impl ProofSummary {
    /// Create an empty summary.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a record.
    pub fn add(&mut self, record: ProofRecord) {
        self.records.push(record);
    }
    /// Total records.
    pub fn total(&self) -> usize {
        self.records.len()
    }
    /// Verified count.
    pub fn verified_count(&self) -> usize {
        self.records
            .iter()
            .filter(|r| r.status.is_verified())
            .count()
    }
    /// Failed count.
    pub fn failed_count(&self) -> usize {
        self.records.iter().filter(|r| r.status.is_failed()).count()
    }
    /// Partial count.
    pub fn partial_count(&self) -> usize {
        self.records
            .iter()
            .filter(|r| r.status.is_partial())
            .count()
    }
    /// Unchecked count.
    pub fn unchecked_count(&self) -> usize {
        self.records
            .iter()
            .filter(|r| r.status.is_unchecked())
            .count()
    }
    /// Are all verified?
    pub fn all_verified(&self) -> bool {
        !self.records.is_empty() && self.verified_count() == self.total()
    }
    /// Iterate over records.
    pub fn iter(&self) -> impl Iterator<Item = &ProofRecord> {
        self.records.iter()
    }
    /// Names of failed proofs.
    pub fn failed_names(&self) -> Vec<&Name> {
        self.records
            .iter()
            .filter(|r| r.status.is_failed())
            .map(|r| &r.name)
            .collect()
    }
}
/// A trace of all steps in a proof attempt.
#[derive(Clone, Debug, Default)]
pub struct ProofTrace {
    pub(super) steps: Vec<ProofStep>,
}
impl ProofTrace {
    /// Create an empty trace.
    pub fn new() -> Self {
        Self::default()
    }
    /// Append a step.
    pub fn push(&mut self, step: ProofStep) {
        self.steps.push(step);
    }
    /// Number of steps.
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    /// Is empty?
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    /// Iterate.
    pub fn iter(&self) -> impl Iterator<Item = &ProofStep> {
        self.steps.iter()
    }
    /// Was the proof completed?
    pub fn is_complete(&self) -> bool {
        self.steps.last().map(|s| s.closed_all()).unwrap_or(false)
    }
    /// Access the steps slice.
    pub fn steps(&self) -> &[ProofStep] {
        &self.steps
    }
    /// Successful steps.
    pub fn successful_steps(&self) -> usize {
        self.steps.iter().filter(|s| s.success).count()
    }
    /// Failed steps.
    pub fn failed_steps(&self) -> usize {
        self.steps.iter().filter(|s| !s.success).count()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ProofAttempt {
    pub id: u64,
    pub tactic_sequence: Vec<String>,
    pub status: ProofStatusV2,
    pub timestamp_ms: u64,
}
#[allow(dead_code)]
impl ProofAttempt {
    pub fn new(id: u64, timestamp_ms: u64) -> Self {
        Self {
            id,
            tactic_sequence: Vec::new(),
            status: ProofStatusV2::Pending,
            timestamp_ms,
        }
    }
    pub fn add_tactic(&mut self, tactic: &str) {
        self.tactic_sequence.push(tactic.to_string());
    }
    pub fn tactic_count(&self) -> usize {
        self.tactic_sequence.len()
    }
    pub fn render_script(&self) -> String {
        self.tactic_sequence.join(
            "
",
        )
    }
}
/// A proof trace augmented with annotations.
#[derive(Clone, Debug, Default)]
pub struct AnnotatedTrace {
    /// The underlying proof trace.
    pub trace: ProofTrace,
    /// Annotations keyed by step index.
    pub annotations: Vec<ProofAnnotation>,
}
impl AnnotatedTrace {
    /// Create an empty annotated trace.
    pub fn new() -> Self {
        Self::default()
    }
    /// Append a proof step.
    pub fn push(&mut self, step: ProofStep) {
        self.trace.push(step);
    }
    /// Add an annotation.
    pub fn annotate(&mut self, ann: ProofAnnotation) {
        self.annotations.push(ann);
    }
    /// Get all annotations for a given step.
    pub fn annotations_for(&self, step: usize) -> Vec<&ProofAnnotation> {
        self.annotations.iter().filter(|a| a.step == step).collect()
    }
    /// Return the total number of annotations.
    pub fn annotation_count(&self) -> usize {
        self.annotations.len()
    }
    /// Check if the proof is complete.
    pub fn is_complete(&self) -> bool {
        self.trace.is_complete()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum GoalShape {
    Equality,
    Implication,
    Conjunction,
    Disjunction,
    Universal,
    Existential,
    Negation,
    Atomic,
    Unknown,
}
