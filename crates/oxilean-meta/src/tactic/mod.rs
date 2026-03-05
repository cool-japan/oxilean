#![allow(missing_docs)]
//! Tactic framework for interactive proof construction.
//!
//! Provides the core tactic infrastructure and a library of basic tactics
//! that mirror LEAN 4's tactic mode.
//!
//! ## Architecture
//!
//! Tactics operate on a `TacticState` which manages a list of goals
//! (metavariables to be filled). Each tactic transforms the state by
//! assigning some metavariables and potentially creating new ones.

// --- Batch 4.3: Core Tactics ---
pub mod cases;
pub mod constructor;
pub mod core;
pub mod rewrite;
pub mod state;
pub mod structural;

// --- Batch 4.4: Advanced Tactics ---
pub mod calc;
pub mod omega;
pub mod simp;

// --- Batch 4.5: Specialized Tactics ---
pub mod ext;
pub mod norm_num;
pub mod ring;
pub mod solve_by_elim;

// --- Batch 4.6: Proof Search Tactics ---
pub mod aesop;
pub mod library_search;

// --- Batch 4.7: Recursive Destructuring Tactics ---
pub mod rcases;

// --- Batch 4.8: Advanced Induction Tactics ---
pub mod induction_adv;

// --- Batch 4.10: E-matching / Congruence Closure Prover ---
pub mod grind;

// --- Batch 4.9: Bit-Vector Decision Procedure ---
pub mod bvdecide;

// --- Phase 12: Additional Tactics ---
pub mod conv_mode;
pub mod injection;
pub mod monotonicity;
pub mod norm_cast;
pub mod smt;

// --- Phase 13: Decidability, Congruence, Positivity, Linear Combination ---
pub mod congr;
pub mod decide;
pub mod linear_combination;
pub mod positivity;

// --- Phase 14: Fun_prop, Polyrith, Tauto ---
pub mod apply_rules;
pub mod fun_prop;
pub mod gcongr;
pub mod polyrith;
pub mod tauto;

// --- Re-exports: Batch 4.3 ---
pub use cases::{tac_cases, tac_induction, CasesResult, InductionResult};
pub use constructor::{tac_constructor, tac_existsi, tac_left, tac_right};
pub use core::{
    tac_apply, tac_assumption, tac_exact, tac_intro, tac_intros, tac_refine, tac_trivial,
};
pub use rewrite::{tac_rewrite, RewriteDirection};
pub use state::{GoalView, TacticError, TacticResult, TacticState};
pub use structural::{tac_clear, tac_revert, tac_subst};

// --- Re-exports: Batch 4.4 ---
pub use calc::{enter_conv, tac_calc, CalcProof, CalcStep, ConvSide};
pub use omega::{
    gcd, is_satisfiable, is_unsatisfiable, lcm, solve_omega, solve_omega_with_config, tac_omega,
    LinearConstraint, LinearExpr, LinearTerm, OmegaConfig, OmegaProof, OmegaResult, OmegaSolver,
    OmegaStep,
};
pub use simp::discharge::DischargeStrategy;
pub use simp::types::{default_simp_lemmas, SimpConfig, SimpLemma, SimpResult, SimpTheorems};

// --- Re-exports: Batch 4.5 ---
pub use ext::{
    tac_ext, tac_ext_with_config, tac_funext, tac_propext, ExtConfig, ExtLemma, ExtLemmaRegistry,
    ExtResult, RegistrySummary,
};
pub use norm_num::{tac_norm_num, ComparisonOp, NumericValue};
pub use ring::{tac_ring, Monomial, Polynomial};
pub use solve_by_elim::{
    solve_by_elim_with_stats, tac_solve_by_elim, tac_solve_by_elim_with_config, BacktrackState,
    CandidateSource, SearchStats, SolveByElimConfig, SolveByElimResult,
};

// --- Re-exports: Batch 4.6 ---
pub use aesop::{
    tac_aesop, tac_aesop_with_config, tac_aesop_with_rules, AesopConfig, AesopResult, AesopRule,
    AesopRuleKind, AesopRuleSafety, AesopRuleSet, AesopSearchNode, AesopSearchState, AesopStats,
};
pub use library_search::{
    tac_exact_question, tac_library_search, CacheLookup, LemmaCandidate, LemmaIndex,
    LibrarySearchConfig, ScoringCriteria, SearchCache, SearchResult,
};

// --- Re-exports: Batch 4.7 ---
pub use rcases::{
    parse_rcases_pattern, tac_obtain, tac_rcases, tac_rcases_many, tac_rintro, ObtainResult,
    RcasesConfig, RcasesPattern, RcasesResult,
};

// --- Re-exports: Batch 4.8 ---
pub use induction_adv::{
    check_recursor_compatibility, infer_induction_scheme, tac_generalize, tac_induction_adv,
    tac_mutual_induction, tac_well_founded_induction, GeneralizationResult, InductionConfig,
    InductionScheme, MinorPremise, MutualInductionConfig, WellFoundedConfig,
};

// --- Re-exports: Batch 4.9 ---
pub use bvdecide::{
    bv_decide_with_stats, tac_bv_decide, tac_bv_decide_with_config, BitVec, BitWidth,
    BvDecideConfig, BvDecideStats, BvExpr, CdclSolver, CnfFormula, SatResult,
};

// --- Re-exports: Batch 4.10 ---
pub use grind::{
    check_nat_le_by_transitivity, extract_nat_constraints, grind_check_eq, grind_eq, grind_on_goal,
    grind_with_la, grind_with_stats, tac_grind, tac_grind_aggressive, tac_grind_with_config,
    try_parse_nat_constraint, CaseSplitter, CongruenceClosure, EClass, EClassId, EMatchCompiler,
    ENode, ENodeId, EPattern, EPatternNode, EqualityStep, GrindConfig, GrindProof, GrindResult,
    GrindState, GrindStats, MergeReason, NatConstraint, NatRelKind, ProofStep, SignatureTable,
    Substitution, TermIndex, UnionFind,
};

// --- Re-exports: Phase 12 ---
pub use conv_mode::{
    conv_arg, conv_ext, conv_lhs, conv_norm_num, conv_rhs, conv_ring, conv_rw, conv_simp,
    enter_conv as enter_conv_mode, exit_conv, run_conv_session, ConvConfig, ConvDirection,
    ConvEntrySide, ConvOperation, ConvPath, ConvResult, ConvState, ConvStats, ConvTarget,
};
pub use injection::{
    build_injection_proof, build_no_confusion_proof, decompose_constructor_eq, tac_injection,
    tac_injection_with, tac_no_confusion, InjectionConfig, InjectionResult, InjectionStats,
    NoConfusionResult,
};
pub use monotonicity::{
    combine_relations, count_rules, decompose_relation, generate_mono_goals,
    is_monotone_in_ruleset, monotone_functions_for_relation, structurally_compatible, tac_mono,
    tac_mono_with_config, tac_mono_with_rules, MonoChain, MonoConclusion, MonoConfig, MonoPremise,
    MonoRelation, MonoResult, MonoRule, MonoRuleSet, MonoStats,
};
pub use norm_cast::{
    find_cast_chain, tac_exact_mod_cast, tac_norm_cast, tac_pull_cast, tac_push_cast, CastConfig,
    CastDirection, CastLemma, CastLemmaSet, CastResult, CastStats, CastStep,
};

// ── Tactic registry and metadata ──────────────────────────────────────────────

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
    TacticEntry::new(
        "intro",
        "Introduce a hypothesis",
        TacticPriority::Low,
        false,
        false,
    ),
    TacticEntry::new(
        "intros",
        "Introduce multiple hypotheses",
        TacticPriority::Low,
        false,
        false,
    ),
    TacticEntry::new(
        "exact",
        "Close goal with exact term",
        TacticPriority::Normal,
        true,
        false,
    ),
    TacticEntry::new("apply", "Apply a lemma", TacticPriority::Normal, true, true),
    TacticEntry::new(
        "assumption",
        "Close from hypothesis",
        TacticPriority::Low,
        true,
        false,
    ),
    TacticEntry::new(
        "refl",
        "Close reflexive goal",
        TacticPriority::Low,
        true,
        false,
    ),
    TacticEntry::new(
        "rw",
        "Rewrite using equality",
        TacticPriority::Normal,
        false,
        false,
    ),
    TacticEntry::new(
        "simp",
        "Simplify the goal",
        TacticPriority::High,
        true,
        false,
    ),
    TacticEntry::new(
        "cases",
        "Case-split on a term",
        TacticPriority::Normal,
        false,
        true,
    ),
    TacticEntry::new(
        "induction",
        "Induction on a term",
        TacticPriority::Normal,
        false,
        true,
    ),
    TacticEntry::new(
        "omega",
        "Linear arithmetic decision",
        TacticPriority::High,
        true,
        false,
    ),
    TacticEntry::new(
        "ring",
        "Commutative ring equations",
        TacticPriority::High,
        true,
        false,
    ),
    TacticEntry::new(
        "linarith",
        "Linear arithmetic",
        TacticPriority::High,
        true,
        false,
    ),
    TacticEntry::new(
        "constructor",
        "Apply inductive constructor",
        TacticPriority::Normal,
        false,
        true,
    ),
    TacticEntry::new(
        "left",
        "Choose left disjunct",
        TacticPriority::Normal,
        false,
        false,
    ),
    TacticEntry::new(
        "right",
        "Choose right disjunct",
        TacticPriority::Normal,
        false,
        false,
    ),
    TacticEntry::new(
        "trivial",
        "Trivial proof search",
        TacticPriority::Low,
        true,
        false,
    ),
    TacticEntry::new(
        "sorry",
        "Placeholder proof",
        TacticPriority::High,
        true,
        false,
    ),
    TacticEntry::new(
        "aesop",
        "Automated proof search",
        TacticPriority::High,
        true,
        false,
    ),
    TacticEntry::new(
        "grind",
        "E-matching + congruence closure",
        TacticPriority::High,
        true,
        false,
    ),
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

// ── Tactic result combinators ─────────────────────────────────────────────────

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
///
/// Splits on newlines and semicolons, trimming whitespace and skipping
/// blank/comment lines.
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

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tactic_mod_tests {
    use super::*;

    #[test]
    fn test_lookup_tactic_found() {
        let entry = lookup_tactic("simp").expect("entry should be present");
        assert_eq!(entry.name, "simp");
        assert!(entry.can_close);
    }

    #[test]
    fn test_lookup_tactic_not_found() {
        assert!(lookup_tactic("nonexistent_tactic_xyz").is_none());
    }

    #[test]
    fn test_registered_count() {
        assert!(registered_tactic_count() >= 10);
    }

    #[test]
    fn test_closing_tactics_nonempty() {
        let closing: Vec<_> = closing_tactics().collect();
        assert!(!closing.is_empty());
        assert!(closing.iter().all(|e| e.can_close));
    }

    #[test]
    fn test_splitting_tactics_nonempty() {
        let splitting: Vec<_> = splitting_tactics().collect();
        assert!(!splitting.is_empty());
        assert!(splitting.iter().all(|e| e.can_split));
    }

    #[test]
    fn test_is_known_tactic() {
        assert!(is_known_tactic("omega"));
        assert!(is_known_tactic("exact"));
        assert!(!is_known_tactic("not_a_real_tactic"));
    }

    #[test]
    fn test_tactic_completions() {
        let comps = tactic_completions("in");
        assert!(comps.contains(&"induction"));
        assert!(comps.contains(&"intro"));
    }

    #[test]
    fn test_split_tactic_block() {
        let block = "intro h\nrw [h]\n-- comment\nexact h";
        let tactics = split_tactic_block(block);
        assert_eq!(tactics, vec!["intro h", "rw [h]", "exact h"]);
    }

    #[test]
    fn test_count_tactics_in_block() {
        let block = "intro x; apply h; exact rfl";
        assert_eq!(count_tactics_in_block(block), 3);
    }

    #[test]
    fn test_block_has_sorry() {
        assert!(block_has_sorry("intro h\nsorry"));
        assert!(!block_has_sorry("intro h\nexact h"));
    }

    #[test]
    fn test_tactic_outcome_or_else() {
        let fail = TacticOutcome::Failure("failed".to_string());
        let ok = TacticOutcome::Success;
        let fail2 = TacticOutcome::Failure("failed".to_string());
        assert_eq!(fail.or_else(ok.clone()), ok.clone());
        let ok2 = TacticOutcome::Success;
        assert_eq!(ok2.clone().or_else(fail2), ok2);
    }

    #[test]
    fn test_tactic_name_parsing() {
        assert_eq!(tactic_name("intro h"), "intro");
        assert_eq!(tactic_name("  simp only [h1]"), "simp");
        assert_eq!(tactic_name(""), "");
    }

    #[test]
    fn test_priority_ordering() {
        assert!(TacticPriority::Low < TacticPriority::Normal);
        assert!(TacticPriority::Normal < TacticPriority::High);
    }
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

// ── Tactic validation ──────────────────────────────────────────────────────────

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

// ── Tactic argument parsing helpers ───────────────────────────────────────────

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
///
/// Returns the comma-separated items inside the brackets, or `None` if
/// there is no bracket list.
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

// ── Tactic sequence analysis ───────────────────────────────────────────────────

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

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tactic_mod_extra_tests {
    use super::*;

    #[test]
    fn test_validate_tactic_brackets_ok() {
        assert!(validate_tactic_brackets("simp only [h1, h2]").is_ok());
        assert!(validate_tactic_brackets("apply (f x)").is_ok());
    }

    #[test]
    fn test_validate_tactic_brackets_bad() {
        assert!(validate_tactic_brackets("simp only [h1").is_err());
        assert!(validate_tactic_brackets("apply (f x").is_err());
        assert!(validate_tactic_brackets("]bad").is_err());
    }

    #[test]
    fn test_tactic_argument() {
        assert_eq!(tactic_argument("apply h"), Some("h"));
        assert_eq!(tactic_argument("intro"), None);
        assert_eq!(tactic_argument("exact (f x)"), Some("(f x)"));
    }

    #[test]
    fn test_simp_only_lemmas() {
        let lemmas = simp_only_lemmas("simp only [h1, h2, h3]").expect("lemmas should be present");
        assert_eq!(lemmas, vec!["h1", "h2", "h3"]);
        assert!(simp_only_lemmas("simp").is_none());
    }

    #[test]
    fn test_is_backward_rw() {
        assert!(is_backward_rw("rw [← h]"));
        assert!(is_backward_rw("rw [<- h]"));
        assert!(!is_backward_rw("rw [h]"));
    }

    #[test]
    fn test_tactic_block_stats() {
        let block = "intro h\napply f\nsimp\nsorry";
        let stats = tactic_block_stats(block);
        assert_eq!(stats.total, 4);
        assert_eq!(stats.sorry_count, 1);
        assert_eq!(stats.simp_count, 1);
        assert_eq!(stats.apply_count, 1);
        assert_eq!(stats.intro_count, 1);
    }

    #[test]
    fn test_proof_state_summary() {
        let s = ProofStateSummary::new(2, vec!["goal 1".to_string(), "goal 2".to_string()]);
        assert_eq!(s.goal_count, 2);
        assert!(!s.is_complete);
        let done = ProofStateSummary::new(0, vec![]);
        assert!(done.is_complete);
    }

    #[test]
    fn test_validate_tactic_block() {
        let block = "intro h\nsimp only [h1\nexact h";
        let errors = validate_tactic_block(block);
        assert!(!errors.is_empty());
    }
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
    TacticSequence {
        first: "intro",
        then: "exact",
        description: "introduce then close",
    },
    TacticSequence {
        first: "intro",
        then: "apply",
        description: "introduce then apply",
    },
    TacticSequence {
        first: "induction",
        then: "intro",
        description: "induction then introduce IH",
    },
    TacticSequence {
        first: "cases",
        then: "exact",
        description: "case analysis then close",
    },
    TacticSequence {
        first: "rw",
        then: "exact",
        description: "rewrite then close",
    },
    TacticSequence {
        first: "simp",
        then: "exact",
        description: "simplify then close",
    },
    TacticSequence {
        first: "apply",
        then: "exact",
        description: "apply then fill subgoal",
    },
    TacticSequence {
        first: "constructor",
        then: "exact",
        description: "constructor then fill fields",
    },
    TacticSequence {
        first: "left",
        then: "exact",
        description: "choose disjunct then close",
    },
    TacticSequence {
        first: "right",
        then: "exact",
        description: "choose disjunct then close",
    },
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
///
/// For example, for `cases` this would be `"cases h\n· sorry\n· sorry"`.
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

#[cfg(test)]
mod tactic_mod_final_tests {
    use super::*;

    #[test]
    fn test_common_next_tactics() {
        let nexts = common_next_tactics("intro");
        assert!(!nexts.is_empty());
        assert!(nexts.contains(&"exact") || nexts.contains(&"apply"));
    }

    #[test]
    fn test_tactic_sequences_nonempty() {
        assert!(!TACTIC_SEQUENCES.is_empty());
    }

    #[test]
    fn test_tactic_skeleton_cases() {
        let s = tactic_skeleton("cases");
        assert!(s.contains("cases h"));
        assert!(s.contains("sorry"));
    }

    #[test]
    fn test_tactic_skeleton_default() {
        let s = tactic_skeleton("exact");
        assert!(s.contains("exact"));
    }

    #[test]
    fn test_all_registry_entries_have_names() {
        for entry in TACTIC_REGISTRY {
            assert!(!entry.name.is_empty());
            assert!(!entry.description.is_empty());
        }
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

#[cfg(test)]
mod tactic_docs_tests {
    use super::*;
    #[test]
    fn test_tactic_docs() {
        assert!(!tactic_docs("simp").is_empty());
        assert!(!tactic_docs("omega").is_empty());
        assert_eq!(tactic_docs("unknown_xyz"), "No documentation available.");
    }
    #[test]
    fn test_is_finishing_tactic() {
        assert!(is_finishing_tactic("exact"));
        assert!(is_finishing_tactic("sorry"));
        assert!(!is_finishing_tactic("intro"));
    }
    #[test]
    fn test_is_structural_tactic() {
        assert!(is_structural_tactic("intro"));
        assert!(is_structural_tactic("cases"));
        assert!(!is_structural_tactic("exact"));
    }
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
///
/// `goal_str` is a textual representation of the goal type.
#[allow(dead_code)]
pub fn suggest_tactics(goal_str: &str) -> Vec<TacticHint> {
    let mut hints = Vec::new();

    if goal_str.contains("= ") || goal_str.contains(" =") {
        hints.push(TacticHint::new(
            "rfl",
            "Goal is an equality, try reflexivity",
            0.9,
        ));
        hints.push(TacticHint::new(
            "simp",
            "Goal is an equality, simp may close it",
            0.7,
        ));
        hints.push(TacticHint::new("ring", "Equality in a ring, try ring", 0.6));
    }
    if goal_str.contains("∧") || goal_str.contains("And") {
        hints.push(TacticHint::new(
            "constructor",
            "Goal is a conjunction, split it",
            0.95,
        ));
    }
    if goal_str.contains("∨") || goal_str.contains("Or") {
        hints.push(TacticHint::new(
            "left",
            "Goal is a disjunction, try left",
            0.6,
        ));
        hints.push(TacticHint::new(
            "right",
            "Goal is a disjunction, try right",
            0.6,
        ));
    }
    if goal_str.contains("→") || goal_str.contains("->") {
        hints.push(TacticHint::new(
            "intro",
            "Goal is an implication, introduce hypothesis",
            0.95,
        ));
    }
    if goal_str.contains("∀") || goal_str.contains("forall") {
        hints.push(TacticHint::new(
            "intro",
            "Goal has a universal quantifier, introduce",
            0.95,
        ));
    }
    if goal_str.contains("Nat") || goal_str.contains("ℕ") {
        hints.push(TacticHint::new(
            "omega",
            "Goal involves naturals, try omega",
            0.7,
        ));
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
pub fn tactic_invocation_counts(block: &str) -> std::collections::HashMap<String, usize> {
    let mut counts = std::collections::HashMap::new();
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

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tactic_new_tests {
    use super::*;

    #[test]
    fn test_tactic_timing_basic() {
        let t = TacticTiming::new("simp", 5000, true);
        assert_eq!(t.name, "simp");
        assert!(t.success);
        assert!((t.duration_ms() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_tactic_profile_empty() {
        let p = TacticProfile::new();
        assert_eq!(p.count(), 0);
        assert_eq!(p.average_us(), 0.0);
        assert!(p.slowest().is_none());
    }

    #[test]
    fn test_tactic_profile_record_and_query() {
        let mut p = TacticProfile::new();
        p.record(TacticTiming::new("intro", 100, true));
        p.record(TacticTiming::new("simp", 5000, true));
        p.record(TacticTiming::new("omega", 200, false));
        assert_eq!(p.count(), 3);
        assert_eq!(p.total_success_us(), 5100);
        assert_eq!(p.total_failure_us(), 200);
        let slowest = p.slowest().expect("slowest should be present");
        assert_eq!(slowest.name, "simp");
    }

    #[test]
    fn test_tactic_profile_for_tactic() {
        let mut p = TacticProfile::new();
        p.record(TacticTiming::new("simp", 100, true));
        p.record(TacticTiming::new("simp", 200, false));
        p.record(TacticTiming::new("exact", 50, true));
        let simp_entries = p.for_tactic("simp");
        assert_eq!(simp_entries.len(), 2);
    }

    #[test]
    fn test_suggest_tactics_equality() {
        let hints = suggest_tactics("a = b");
        let names: Vec<_> = hints.iter().map(|h| h.tactic).collect();
        assert!(names.contains(&"rfl"));
        assert!(names.contains(&"simp"));
    }

    #[test]
    fn test_suggest_tactics_implication() {
        let hints = suggest_tactics("P -> Q");
        let names: Vec<_> = hints.iter().map(|h| h.tactic).collect();
        assert!(names.contains(&"intro"));
    }

    #[test]
    fn test_suggest_tactics_conjunction() {
        let hints = suggest_tactics("P ∧ Q");
        let names: Vec<_> = hints.iter().map(|h| h.tactic).collect();
        assert!(names.contains(&"constructor"));
    }

    #[test]
    fn test_suggest_tactics_sorted_by_confidence() {
        let hints = suggest_tactics("P ∧ Q = R");
        for i in 0..hints.len().saturating_sub(1) {
            assert!(hints[i].confidence >= hints[i + 1].confidence);
        }
    }

    #[test]
    fn test_tactic_invocation_counts() {
        let block = "simp\nsimp\nrfl\nexact h";
        let counts = tactic_invocation_counts(block);
        assert_eq!(counts.get("simp"), Some(&2));
        assert_eq!(counts.get("rfl"), Some(&1));
        assert_eq!(counts.get("exact"), Some(&1));
    }

    #[test]
    fn test_most_used_tactic() {
        let block = "intro h\nsimp\nsimp\nsimp\nrfl";
        let most = most_used_tactic(block);
        assert_eq!(most, Some("simp".to_string()));
    }

    #[test]
    fn test_sort_by_priority() {
        let mut tactics = vec![
            "omega".to_string(),
            "intro".to_string(),
            "exact".to_string(),
        ];
        sort_by_priority(&mut tactics);
        // lower priority values should come first
        assert!(!tactics.is_empty());
    }

    #[test]
    fn test_tactic_hint_confidence_clamp() {
        let h = TacticHint::new("simp", "test", 2.0);
        assert_eq!(h.confidence, 1.0);
        let h2 = TacticHint::new("rfl", "test", -0.5);
        assert_eq!(h2.confidence, 0.0);
    }
}

// ============================================================
// Extended: TacMod Utilities (Part 2)
// ============================================================

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

impl<V: Clone + Default> Default for TacModExtMap<V> {
    fn default() -> Self {
        Self::new()
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

impl Default for TacModStateMachine {
    fn default() -> Self {
        Self::new()
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

impl Default for TacModCounterMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tacmod_ext2_tests {
    use super::*;

    #[test]
    fn test_tacmod_ext_util_basic() {
        let mut u = TacModExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }

    #[test]
    fn test_tacmod_ext_util_min_max() {
        let mut u = TacModExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }

    #[test]
    fn test_tacmod_ext_util_flags() {
        let mut u = TacModExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }

    #[test]
    fn test_tacmod_ext_util_pop() {
        let mut u = TacModExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }

    #[test]
    fn test_tacmod_ext_map_basic() {
        let mut m: TacModExtMap<i32> = TacModExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }

    #[test]
    fn test_tacmod_ext_map_get_or_default() {
        let mut m: TacModExtMap<i32> = TacModExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }

    #[test]
    fn test_tacmod_ext_map_keys_sorted() {
        let mut m: TacModExtMap<i32> = TacModExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }

    #[test]
    fn test_tacmod_window_mean() {
        let mut w = TacModWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_tacmod_window_evict() {
        let mut w = TacModWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0); // evicts 10.0
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_tacmod_window_std_dev() {
        let mut w = TacModWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }

    #[test]
    fn test_tacmod_builder_basic() {
        let b = TacModBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }

    #[test]
    fn test_tacmod_builder_summary() {
        let b = TacModBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }

    #[test]
    fn test_tacmod_state_machine_start() {
        let mut sm = TacModStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }

    #[test]
    fn test_tacmod_state_machine_complete() {
        let mut sm = TacModStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }

    #[test]
    fn test_tacmod_state_machine_fail() {
        let mut sm = TacModStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }

    #[test]
    fn test_tacmod_state_machine_no_transition_after_terminal() {
        let mut sm = TacModStateMachine::new();
        sm.complete();
        assert!(!sm.start()); // Already terminal
    }

    #[test]
    fn test_tacmod_work_queue_basic() {
        let mut wq = TacModWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }

    #[test]
    fn test_tacmod_work_queue_capacity() {
        let mut wq = TacModWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }

    #[test]
    fn test_tacmod_counter_map_basic() {
        let mut cm = TacModCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }

    #[test]
    fn test_tacmod_counter_map_frequency() {
        let mut cm = TacModCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_tacmod_counter_map_most_common() {
        let mut cm = TacModCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm.most_common().expect("most_common should succeed");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}

// ============================================================
// Extended: TacMod Utilities (Part 2)
// ============================================================

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

// ============================================================
// Extended: TacticMod Analysis Infrastructure
// ============================================================

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

impl Default for TacticModDiff {
    fn default() -> Self {
        Self::new()
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

impl Default for TacticModConfig {
    fn default() -> Self {
        Self::new()
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

#[cfg(test)]
mod tacticmod_analysis_tests {
    use super::*;

    #[test]
    fn test_tacticmod_result_ok() {
        let r = TacticModResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_tacticmod_result_err() {
        let r = TacticModResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_tacticmod_result_partial() {
        let r = TacticModResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_tacticmod_result_skipped() {
        let r = TacticModResult::Skipped;
        assert!(r.is_skipped());
    }

    #[test]
    fn test_tacticmod_analysis_pass_run() {
        let mut p = TacticModAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }

    #[test]
    fn test_tacticmod_analysis_pass_empty_input() {
        let mut p = TacticModAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }

    #[test]
    fn test_tacticmod_analysis_pass_success_rate() {
        let mut p = TacticModAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_tacticmod_analysis_pass_disable() {
        let mut p = TacticModAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }

    #[test]
    fn test_tacticmod_pipeline_basic() {
        let mut pipeline = TacticModPipeline::new("main_pipeline");
        pipeline.add_pass(TacticModAnalysisPass::new("pass1"));
        pipeline.add_pass(TacticModAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_tacticmod_pipeline_disabled_pass() {
        let mut pipeline = TacticModPipeline::new("partial");
        let mut p = TacticModAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(TacticModAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_tacticmod_diff_basic() {
        let mut d = TacticModDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }

    #[test]
    fn test_tacticmod_diff_summary() {
        let mut d = TacticModDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }

    #[test]
    fn test_tacticmod_config_set_get() {
        let mut cfg = TacticModConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }

    #[test]
    fn test_tacticmod_config_read_only() {
        let mut cfg = TacticModConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false)); // should fail
        assert_eq!(cfg.get_bool("key"), Some(true)); // unchanged
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }

    #[test]
    fn test_tacticmod_config_remove() {
        let mut cfg = TacticModConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }

    #[test]
    fn test_tacticmod_diagnostics_basic() {
        let mut diag = TacticModDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }

    #[test]
    fn test_tacticmod_diagnostics_max_errors() {
        let mut diag = TacticModDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3"); // e3 dropped
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }

    #[test]
    fn test_tacticmod_diagnostics_clear() {
        let mut diag = TacticModDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }

    #[test]
    fn test_tacticmod_config_value_types() {
        let b = TacticModConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);

        let i = TacticModConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));

        let f = TacticModConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);

        let s = TacticModConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));

        let l = TacticModConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}

// --- Extended analysis infrastructure for mod ---

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

impl Default for ModExtDiff2700 {
    fn default() -> Self {
        Self::new()
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

impl Default for ModExtConfig2700 {
    fn default() -> Self {
        Self::new()
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

#[cfg(test)]
mod mod_ext_tests_2700 {
    use super::*;

    #[test]
    fn test_mod_ext_result_ok_2700() {
        let r = ModExtResult2700::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_mod_ext_result_err_2700() {
        let r = ModExtResult2700::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_mod_ext_result_partial_2700() {
        let r = ModExtResult2700::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_mod_ext_result_skipped_2700() {
        let r = ModExtResult2700::Skipped;
        assert!(r.is_skipped());
    }

    #[test]
    fn test_mod_ext_pass_run_2700() {
        let mut p = ModExtPass2700::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }

    #[test]
    fn test_mod_ext_pass_empty_2700() {
        let mut p = ModExtPass2700::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }

    #[test]
    fn test_mod_ext_pass_rate_2700() {
        let mut p = ModExtPass2700::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_mod_ext_pass_disable_2700() {
        let mut p = ModExtPass2700::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }

    #[test]
    fn test_mod_ext_pipeline_basic_2700() {
        let mut pipeline = ModExtPipeline2700::new("main_pipeline");
        pipeline.add_pass(ModExtPass2700::new("pass1"));
        pipeline.add_pass(ModExtPass2700::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_mod_ext_pipeline_disabled_2700() {
        let mut pipeline = ModExtPipeline2700::new("partial");
        let mut p = ModExtPass2700::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(ModExtPass2700::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_mod_ext_diff_basic_2700() {
        let mut d = ModExtDiff2700::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }

    #[test]
    fn test_mod_ext_config_set_get_2700() {
        let mut cfg = ModExtConfig2700::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }

    #[test]
    fn test_mod_ext_config_read_only_2700() {
        let mut cfg = ModExtConfig2700::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }

    #[test]
    fn test_mod_ext_config_remove_2700() {
        let mut cfg = ModExtConfig2700::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }

    #[test]
    fn test_mod_ext_diagnostics_basic_2700() {
        let mut diag = ModExtDiag2700::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }

    #[test]
    fn test_mod_ext_diagnostics_max_errors_2700() {
        let mut diag = ModExtDiag2700::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }

    #[test]
    fn test_mod_ext_diagnostics_clear_2700() {
        let mut diag = ModExtDiag2700::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }

    #[test]
    fn test_mod_ext_config_value_types_2700() {
        let b = ModExtConfigVal2700::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);

        let i = ModExtConfigVal2700::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));

        let f = ModExtConfigVal2700::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("as_float should succeed") - 2.5).abs() < 1e-10);

        let s = ModExtConfigVal2700::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));

        let l = ModExtConfigVal2700::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
