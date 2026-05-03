//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    ProofStateSummary, TacModBuilder, TacModCounterMap, TacModExtMap, TacModExtUtil,
    TacModStateMachine, TacModWindow, TacModWorkQueue, TacticBlockStats, TacticEntry, TacticHint,
    TacticOutcome, TacticPriority, TacticProfile, TacticSequence, TacticTiming,
};
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
/// Generate a minimal proof skeleton for a tactic.
///
/// For example, for `cases` this would be `"cases h\n· sorry\n· sorry"`.
#[allow(dead_code)]
pub fn tactic_skeleton(name: &str) -> String {
    match name {
        "cases" | "induction" => format!("{} h\n· sorry\n· sorry", name),
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
    hints.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    hints
}
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
        w.push(30.0);
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
        assert!(!sm.start());
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
