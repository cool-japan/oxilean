//! Tactic registry, metadata, helpers, validation, docs, profiling, and hints.

use std::collections::HashMap;

// ── TacticPriority / TacticEntry / TACTIC_REGISTRY ───────────────────────────

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

/// Static table of all built-in tactics.
#[allow(dead_code)]
pub static TACTIC_REGISTRY: &[TacticEntry] = &[
    TacticEntry::new("intro", "Introduce a hypothesis", TacticPriority::Low, false, false),
    TacticEntry::new("intros", "Introduce multiple hypotheses", TacticPriority::Low, false, false),
    TacticEntry::new("exact", "Close goal with exact term", TacticPriority::Normal, true, false),
    TacticEntry::new("apply", "Apply a lemma", TacticPriority::Normal, true, true),
    TacticEntry::new("assumption", "Close from hypothesis", TacticPriority::Low, true, false),
    TacticEntry::new("refl", "Close reflexive goal", TacticPriority::Low, true, false),
    TacticEntry::new("rw", "Rewrite using equality", TacticPriority::Normal, false, false),
    TacticEntry::new("simp", "Simplify the goal", TacticPriority::High, true, false),
    TacticEntry::new("cases", "Case-split on a term", TacticPriority::Normal, false, true),
    TacticEntry::new("induction", "Induction on a term", TacticPriority::Normal, false, true),
    TacticEntry::new("omega", "Linear arithmetic decision", TacticPriority::High, true, false),
    TacticEntry::new("ring", "Commutative ring equations", TacticPriority::High, true, false),
    TacticEntry::new("linarith", "Linear arithmetic", TacticPriority::High, true, false),
    TacticEntry::new("constructor", "Apply inductive constructor", TacticPriority::Normal, false, true),
    TacticEntry::new("left", "Choose left disjunct", TacticPriority::Normal, false, false),
    TacticEntry::new("right", "Choose right disjunct", TacticPriority::Normal, false, false),
    TacticEntry::new("trivial", "Trivial proof search", TacticPriority::Low, true, false),
    TacticEntry::new("sorry", "Placeholder proof", TacticPriority::High, true, false),
    TacticEntry::new("aesop", "Automated proof search", TacticPriority::High, true, false),
    TacticEntry::new("grind", "E-matching + congruence closure", TacticPriority::High, true, false),
];

/// Look up a tactic entry by name.
#[allow(dead_code)]
pub fn lookup_tactic(name: &str) -> Option<&'static TacticEntry> {
    TACTIC_REGISTRY.iter().find(|e| e.name == name)
}

/// Return all tactics that can close a goal.
#[allow(dead_code)]
pub fn closing_tactics() -> impl Iterator<Item = &'static TacticEntry> {
    TACTIC_REGISTRY.iter().filter(|e| e.can_close)
}

/// Return all tactics that may split a goal.
#[allow(dead_code)]
pub fn splitting_tactics() -> impl Iterator<Item = &'static TacticEntry> {
    TACTIC_REGISTRY.iter().filter(|e| e.can_split)
}

/// Return the number of registered tactics.
#[allow(dead_code)]
pub fn registered_tactic_count() -> usize {
    TACTIC_REGISTRY.len()
}

// ── Tactic name parsing helpers ───────────────────────────────────────────────

/// Parse the tactic name (first word) from a tactic string.
#[allow(dead_code)]
pub fn tactic_name(src: &str) -> &str {
    src.split_whitespace().next().unwrap_or("")
}

/// Return `true` if `name` is a recognised tactic.
#[allow(dead_code)]
pub fn is_known_tactic(name: &str) -> bool {
    lookup_tactic(name).is_some()
}

/// Return a list of tactic names that begin with `prefix`.
#[allow(dead_code)]
pub fn tactic_completions(prefix: &str) -> Vec<&'static str> {
    TACTIC_REGISTRY
        .iter()
        .filter(|e| e.name.starts_with(prefix))
        .map(|e| e.name)
        .collect()
}

// ── TacticOutcome ─────────────────────────────────────────────────────────────

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

// ── Tactic script helpers ─────────────────────────────────────────────────────

/// Split a tactic block string into individual tactic strings.
#[allow(dead_code)]
pub fn split_tactic_block(block: &str) -> Vec<String> {
    block
        .split(['\n', ';'])
        .map(str::trim)
        .filter(|s| !s.is_empty() && !s.starts_with("--"))
        .map(str::to_string)
        .collect()
}

/// Count the number of distinct tactic invocations in a block.
#[allow(dead_code)]
pub fn count_tactics_in_block(block: &str) -> usize {
    split_tactic_block(block).len()
}

/// Return `true` if `block` contains at least one `sorry`.
#[allow(dead_code)]
pub fn block_has_sorry(block: &str) -> bool {
    split_tactic_block(block)
        .iter()
        .any(|t| tactic_name(t) == "sorry")
}

// ── Tactic context information ────────────────────────────────────────────────

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

// ── Tactic validation ─────────────────────────────────────────────────────────

/// Validate that a tactic string has balanced brackets.
#[allow(dead_code)]
pub fn validate_tactic_brackets(tac: &str) -> Result<(), String> {
    let mut depth_paren: i32 = 0;
    let mut depth_bracket: i32 = 0;
    let mut depth_brace: i32 = 0;
    for ch in tac.chars() {
        match ch {
            '(' => depth_paren += 1,
            ')' => depth_paren -= 1,
            '[' => depth_bracket += 1,
            ']' => depth_bracket -= 1,
            '{' => depth_brace += 1,
            '}' => depth_brace -= 1,
            _ => {}
        }
        if depth_paren < 0 || depth_bracket < 0 || depth_brace < 0 {
            return Err(format!("unmatched closing bracket in tactic: `{}`", tac));
        }
    }
    if depth_paren != 0 {
        return Err(format!("unclosed `(` in tactic: `{}`", tac));
    }
    if depth_bracket != 0 {
        return Err(format!("unclosed `[` in tactic: `{}`", tac));
    }
    if depth_brace != 0 {
        return Err(format!("unclosed `{{` in tactic: `{}`", tac));
    }
    Ok(())
}

/// Validate all tactics in a block.
#[allow(dead_code)]
pub fn validate_tactic_block(block: &str) -> Vec<(String, String)> {
    split_tactic_block(block)
        .into_iter()
        .filter_map(|tac| validate_tactic_brackets(&tac).err().map(|e| (tac, e)))
        .collect()
}

// ── Tactic argument parsing helpers ──────────────────────────────────────────

/// Extract the argument string from a tactic like `"apply h"` → `"h"`.
#[allow(dead_code)]
pub fn tactic_argument(src: &str) -> Option<&str> {
    let name = tactic_name(src);
    if name.is_empty() {
        return None;
    }
    let rest = src[name.len()..].trim();
    if rest.is_empty() {
        None
    } else {
        Some(rest)
    }
}

/// Extract the list from a `simp only [h1, h2]` style tactic.
#[allow(dead_code)]
pub fn simp_only_lemmas(tac: &str) -> Option<Vec<String>> {
    let start = tac.find('[')?;
    let end = tac.rfind(']')?;
    if end <= start {
        return None;
    }
    let inner = &tac[start + 1..end];
    Some(
        inner
            .split(',')
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_string)
            .collect(),
    )
}

/// Return `true` if `tac` uses `← h` (backwards rewrite).
#[allow(dead_code)]
pub fn is_backward_rw(tac: &str) -> bool {
    tac.contains("← ") || tac.contains("<- ")
}

// ── Tactic sequence analysis ──────────────────────────────────────────────────

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

/// Compute statistics for a tactic block string.
#[allow(dead_code)]
pub fn tactic_block_stats(block: &str) -> TacticBlockStats {
    let tactics = split_tactic_block(block);
    let mut stats = TacticBlockStats {
        total: tactics.len(),
        ..Default::default()
    };
    for tac in &tactics {
        let name = tactic_name(tac);
        match name {
            "sorry" => stats.sorry_count += 1,
            "simp" => stats.simp_count += 1,
            "rw" | "rfl" | "refl" => stats.rw_count += 1,
            "intro" | "intros" => stats.intro_count += 1,
            "apply" => stats.apply_count += 1,
            _ => {}
        }
    }
    stats
}

// ── Tactic dependency graph ───────────────────────────────────────────────────

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

/// Common tactic sequences in OxiLean proofs.
#[allow(dead_code)]
pub static TACTIC_SEQUENCES: &[TacticSequence] = &[
    TacticSequence { first: "intro", then: "exact", description: "introduce then close" },
    TacticSequence { first: "intro", then: "apply", description: "introduce then apply" },
    TacticSequence { first: "induction", then: "intro", description: "induction then introduce IH" },
    TacticSequence { first: "cases", then: "exact", description: "case analysis then close" },
    TacticSequence { first: "rw", then: "exact", description: "rewrite then close" },
    TacticSequence { first: "simp", then: "exact", description: "simplify then close" },
    TacticSequence { first: "apply", then: "exact", description: "apply then fill subgoal" },
    TacticSequence { first: "constructor", then: "exact", description: "constructor then fill fields" },
    TacticSequence { first: "left", then: "exact", description: "choose disjunct then close" },
    TacticSequence { first: "right", then: "exact", description: "choose disjunct then close" },
];

/// Return tactics that commonly follow `name` in a proof.
#[allow(dead_code)]
pub fn common_next_tactics(name: &str) -> Vec<&'static str> {
    TACTIC_SEQUENCES
        .iter()
        .filter(|s| s.first == name)
        .map(|s| s.then)
        .collect()
}

// ── Proof skeleton generation ─────────────────────────────────────────────────

/// Generate a minimal proof skeleton for a tactic.
#[allow(dead_code)]
pub fn tactic_skeleton(name: &str) -> String {
    match name {
        "cases" | "induction" => {
            format!("{} h\n· sorry\n· sorry", name)
        }
        "constructor" => "constructor\n· sorry\n· sorry".to_string(),
        "split" | "apply And.intro" => "constructor\n· sorry\n· sorry".to_string(),
        _ => format!("{}\nsorry", name),
    }
}

// ── Tactic documentation ──────────────────────────────────────────────────────

/// Return the documentation string for a tactic.
#[allow(dead_code)]
pub fn tactic_docs(name: &str) -> &'static str {
    match name {
        "intro" | "intros" => "Introduce a hypothesis into the local context.",
        "exact" => "Close the goal with an exact proof term.",
        "apply" => "Apply a lemma, creating sub-goals for its hypotheses.",
        "assumption" => "Close the goal using a matching hypothesis.",
        "refl" => "Close a reflexivity goal (`a = a`).",
        "rw" => "Rewrite the goal using an equality hypothesis.",
        "simp" => "Simplify the goal using the simp lemma set.",
        "cases" => "Perform case analysis on a term or hypothesis.",
        "induction" => "Perform induction on a natural number or inductive type.",
        "omega" => "Decide linear arithmetic over integers/naturals.",
        "ring" => "Prove equalities in commutative rings.",
        "linarith" => "Prove linear arithmetic goals.",
        "constructor" => "Apply the appropriate inductive constructor.",
        "left" | "right" => "Choose a side of a disjunction.",
        "trivial" => "Try common closing tactics (refl, assumption, trivial simp).",
        "sorry" => "Admit the goal (placeholder for unfinished proofs).",
        "aesop" => "Automated proof search using a customisable rule set.",
        "grind" => "Congruence closure and equality saturation.",
        _ => "No documentation available.",
    }
}

/// Return `true` if `name` is a "finishing" tactic that should always close a goal.
#[allow(dead_code)]
pub fn is_finishing_tactic(name: &str) -> bool {
    matches!(
        name,
        "exact" | "assumption" | "refl" | "trivial" | "sorry" | "omega" | "ring"
    )
}

/// Return `true` if `name` is a "structural" tactic that modifies the goal structure.
#[allow(dead_code)]
pub fn is_structural_tactic(name: &str) -> bool {
    matches!(
        name,
        "intro" | "intros" | "revert" | "clear" | "rename" | "cases" | "induction"
    )
}

// ── Tactic performance profiling ──────────────────────────────────────────────

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

// ── Tactic hint system ────────────────────────────────────────────────────────

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

/// Suggest tactics based on simple heuristics about the goal shape.
#[allow(dead_code)]
pub fn suggest_tactics(goal_str: &str) -> Vec<TacticHint> {
    let mut hints = Vec::new();

    if goal_str.contains("= ") || goal_str.contains(" =") {
        hints.push(TacticHint::new("rfl", "Goal is an equality, try reflexivity", 0.9));
        hints.push(TacticHint::new("simp", "Goal is an equality, simp may close it", 0.7));
        hints.push(TacticHint::new("ring", "Equality in a ring, try ring", 0.6));
    }
    if goal_str.contains("∧") || goal_str.contains("And") {
        hints.push(TacticHint::new("constructor", "Goal is a conjunction, split it", 0.95));
    }
    if goal_str.contains("∨") || goal_str.contains("Or") {
        hints.push(TacticHint::new("left", "Goal is a disjunction, try left", 0.6));
        hints.push(TacticHint::new("right", "Goal is a disjunction, try right", 0.6));
    }
    if goal_str.contains("→") || goal_str.contains("->") {
        hints.push(TacticHint::new("intro", "Goal is an implication, introduce hypothesis", 0.95));
    }
    if goal_str.contains("∀") || goal_str.contains("forall") {
        hints.push(TacticHint::new("intro", "Goal has a universal quantifier, introduce", 0.95));
    }
    if goal_str.contains("Nat") || goal_str.contains("ℕ") {
        hints.push(TacticHint::new("omega", "Goal involves naturals, try omega", 0.7));
    }
    if hints.is_empty() {
        hints.push(TacticHint::new("assumption", "Try assumption first", 0.5));
        hints.push(TacticHint::new("trivial", "Try trivial", 0.3));
    }

    // Sort by confidence descending
    hints.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    hints
}

// ── Tactic invocation counts ──────────────────────────────────────────────────

/// Count how many times each tactic appears in a script.
#[allow(dead_code)]
pub fn tactic_invocation_counts(block: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for tac in split_tactic_block(block) {
        let name = tactic_name(&tac).to_string();
        if !name.is_empty() {
            *counts.entry(name).or_insert(0) += 1;
        }
    }
    counts
}

/// Return the most frequently used tactic in a script.
#[allow(dead_code)]
pub fn most_used_tactic(block: &str) -> Option<String> {
    let counts = tactic_invocation_counts(block);
    counts.into_iter().max_by_key(|(_, c)| *c).map(|(n, _)| n)
}

// ── Tactic precedence helpers ─────────────────────────────────────────────────

/// Return the default priority of a tactic (as an integer).
#[allow(dead_code)]
pub fn tactic_default_priority(name: &str) -> u32 {
    lookup_tactic(name)
        .map(|e| e.priority as u32)
        .unwrap_or(TacticPriority::Normal as u32)
}

/// Sort a list of tactic names by priority (lowest first = run first).
#[allow(dead_code)]
pub fn sort_by_priority(names: &mut [String]) {
    names.sort_by_key(|n| tactic_default_priority(n));
}
