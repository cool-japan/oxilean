//! Functions for proof quality lint analysis.

use super::types::{
    ProofAnalysis, ProofQualityConfig, ProofQualityIssue, ProofQualityReport, ProofQualityRule,
};
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Lean/OxiLean tactic keywords used to detect tactic names.
static TACTIC_KEYWORDS: &[&str] = &[
    "simp",
    "rfl",
    "ring",
    "omega",
    "linarith",
    "nlinarith",
    "norm_num",
    "exact",
    "apply",
    "intro",
    "intros",
    "cases",
    "induction",
    "constructor",
    "trivial",
    "tauto",
    "aesop",
    "decide",
    "contradiction",
    "assumption",
    "rewrite",
    "rw",
    "have",
    "show",
    "specialize",
    "obtain",
    "rcases",
    "use",
    "refine",
    "congr",
    "ext",
    "field_simp",
    "push_neg",
    "norm_cast",
    "positivity",
    "gcongr",
];

/// Extract the portion of `source` that is the body of the proof named `name`.
/// Heuristic: look for `theorem <name>` or `lemma <name>`, find the `:=` and
/// collect everything up to a blank line at the top indentation level, or end
/// of source.
fn extract_proof_body<'a>(source: &'a str, name: &str) -> &'a str {
    // Find the line that introduces the declaration.
    let decl_patterns = [
        format!("theorem {}", name),
        format!("lemma {}", name),
        format!("def {}", name),
    ];
    let mut start_byte = None;
    for line in source.lines() {
        for pat in &decl_patterns {
            if line.contains(pat.as_str()) {
                // Byte offset of this line in source.
                start_byte = Some(source.find(line).unwrap_or(0));
                break;
            }
        }
        if start_byte.is_some() {
            break;
        }
    }
    let start = start_byte.unwrap_or(0);
    &source[start..]
}

/// Count nesting depth produced by `by` blocks and `{`/`}` pairs.
fn compute_nesting_depth(text: &str) -> usize {
    let mut depth = 0usize;
    let mut max_depth = 0usize;
    for ch in text.chars() {
        match ch {
            '{' => {
                depth += 1;
                if depth > max_depth {
                    max_depth = depth;
                }
            }
            '}' => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    // Also count `by` keyword nesting (crude but useful).
    let by_count = text.split_whitespace().filter(|w| *w == "by").count();
    max_depth.max(by_count)
}

/// Collect tactic names used in a text snippet.
fn collect_tactics(text: &str) -> Vec<String> {
    let mut found = Vec::new();
    for token in text.split_whitespace() {
        // Strip leading punctuation like `·`, `|`, etc.
        let t = token.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
        if TACTIC_KEYWORDS.contains(&t) {
            found.push(t.to_owned());
        }
    }
    found
}

/// Collect hypothesis names from `intro`/`intros`/`obtain` patterns.
fn collect_hypotheses(text: &str) -> Vec<String> {
    let mut hyps = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        // `intro h` / `intros h1 h2`
        if trimmed.starts_with("intro ") || trimmed.starts_with("intros ") {
            let rest = trimmed
                .trim_start_matches("intros")
                .trim_start_matches("intro")
                .trim();
            for word in rest.split_whitespace() {
                let name = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
                if !name.is_empty() && name != "⟩" {
                    hyps.push(name.to_owned());
                }
            }
        }
        // `(h : T)` binder patterns
        if let Some(idx) = trimmed.find("(h") {
            let slice = &trimmed[idx + 1..];
            if let Some(colon) = slice.find(':') {
                let name = slice[..colon].trim();
                if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
                    hyps.push(name.to_owned());
                }
            }
        }
    }
    hyps.sort();
    hyps.dedup();
    hyps
}

/// Determine which of the provided hypothesis names appear in the proof body
/// after the first line (introductions come first).
fn find_used_hypotheses(text: &str, hyps: &[String]) -> Vec<String> {
    // Skip lines that introduce hypotheses; look for occurrences afterwards.
    let mut used = Vec::new();
    let body_lines: Vec<&str> = text
        .lines()
        .filter(|l| {
            let t = l.trim();
            !t.starts_with("intro ") && !t.starts_with("intros ")
        })
        .collect();
    let body = body_lines.join("\n");
    for hyp in hyps {
        // Match whole word occurrences.
        if body.split_whitespace().any(|w| {
            let cleaned = w.trim_matches(|c: char| !c.is_alphanumeric() && c != '_');
            cleaned == hyp.as_str()
        }) {
            used.push(hyp.clone());
        }
    }
    used
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Build a `ProofAnalysis` for the declaration named `name` within `source`.
pub fn analyze_proof(source: &str, name: &str) -> ProofAnalysis {
    let body = extract_proof_body(source, name);
    let proof_lines = body.lines().count();
    let sorry_count = body.matches("sorry").count();
    let nesting_depth = compute_nesting_depth(body);
    let tactics_used = collect_tactics(body);
    let hypotheses = collect_hypotheses(body);
    let hypotheses_used = find_used_hypotheses(body, &hypotheses);

    ProofAnalysis {
        file: String::new(),
        name: name.to_owned(),
        proof_lines,
        sorry_count,
        nesting_depth,
        tactics_used,
        hypotheses,
        hypotheses_used,
    }
}

/// Emit issues when the proof contains one or more `sorry` occurrences.
pub fn check_sorry_usage(analysis: &ProofAnalysis) -> Vec<ProofQualityIssue> {
    if analysis.sorry_count == 0 {
        return Vec::new();
    }
    vec![ProofQualityIssue::new(
        ProofQualityRule::AvoidSorry,
        (1, 1),
        format!(
            "Proof `{}` contains {} sorry placeholder(s)",
            analysis.name, analysis.sorry_count
        ),
    )
    .with_suggestion("Replace `sorry` with a complete proof term.")]
}

/// Emit issues when nesting depth in `source` exceeds `max_depth`.
pub fn check_nesting_depth(source: &str, max_depth: usize) -> Vec<ProofQualityIssue> {
    let mut issues = Vec::new();
    let mut depth = 0usize;
    for (line_idx, line) in source.lines().enumerate() {
        let line_num = (line_idx + 1) as u32;
        for ch in line.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    if depth > max_depth {
                        issues.push(
                            ProofQualityIssue::new(
                                ProofQualityRule::DeepNesting { max_depth },
                                (line_num, 1),
                                format!("Nesting depth {} exceeds maximum {}", depth, max_depth),
                            )
                            .with_suggestion("Extract nested sub-proofs into helper lemmas."),
                        );
                    }
                }
                '}' => {
                    depth = depth.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
    issues
}

/// Emit an issue when the proof is longer than `max_lines`.
pub fn check_proof_length(analysis: &ProofAnalysis, max_lines: usize) -> Vec<ProofQualityIssue> {
    if analysis.proof_lines <= max_lines {
        return Vec::new();
    }
    vec![ProofQualityIssue::new(
        ProofQualityRule::LongProof { max_lines },
        (1, 1),
        format!(
            "Proof `{}` is {} lines long (max {})",
            analysis.name, analysis.proof_lines, max_lines
        ),
    )
    .with_suggestion("Break this proof into smaller helper lemmas to improve readability.")]
}

/// Emit issues for any tactic that appears at least `min_count` times.
pub fn check_repeated_tactics(
    analysis: &ProofAnalysis,
    min_count: usize,
) -> Vec<ProofQualityIssue> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for tactic in &analysis.tactics_used {
        *counts.entry(tactic.as_str()).or_insert(0) += 1;
    }
    let mut issues = Vec::new();
    for (tactic, count) in &counts {
        if *count >= min_count {
            issues.push(
                ProofQualityIssue::new(
                    ProofQualityRule::RepeatedTactic { min_count },
                    (1, 1),
                    format!(
                        "Tactic `{}` appears {} times in proof `{}`",
                        tactic, count, analysis.name
                    ),
                )
                .with_suggestion(format!(
                    "Consider a `simp_all` or `aesop` call instead of repeating `{}`.",
                    tactic
                )),
            );
        }
    }
    issues
}

/// Emit issues for hypotheses that were introduced but never referenced.
pub fn check_unused_hypotheses(analysis: &ProofAnalysis) -> Vec<ProofQualityIssue> {
    let mut issues = Vec::new();
    for hyp in &analysis.hypotheses {
        if !analysis.hypotheses_used.contains(hyp) {
            issues.push(
                ProofQualityIssue::new(
                    ProofQualityRule::UnusedHypothesis,
                    (1, 1),
                    format!(
                        "Hypothesis `{}` in proof `{}` is never used",
                        hyp, analysis.name
                    ),
                )
                .with_suggestion(format!(
                    "Remove or rename `{}` with an underscore prefix.",
                    hyp
                )),
            );
        }
    }
    issues
}

/// Run all enabled proof-quality checks for `source` and return a full report.
pub fn run_proof_quality_checks(source: &str, cfg: &ProofQualityConfig) -> ProofQualityReport {
    // Collect declaration names in source.
    let names = extract_decl_names(source);
    let mut all_issues: Vec<ProofQualityIssue> = Vec::new();

    for name in &names {
        let analysis = analyze_proof(source, name);

        for rule in &cfg.rules {
            match rule {
                ProofQualityRule::AvoidSorry => {
                    all_issues.extend(check_sorry_usage(&analysis));
                }
                ProofQualityRule::DeepNesting { max_depth } => {
                    all_issues.extend(check_nesting_depth(source, *max_depth));
                }
                ProofQualityRule::LongProof { max_lines } => {
                    all_issues.extend(check_proof_length(&analysis, *max_lines));
                }
                ProofQualityRule::RepeatedTactic { min_count } => {
                    all_issues.extend(check_repeated_tactics(&analysis, *min_count));
                }
                ProofQualityRule::UnusedHypothesis => {
                    all_issues.extend(check_unused_hypotheses(&analysis));
                }
                ProofQualityRule::TrivialLemma => {
                    all_issues.extend(check_trivial_lemma(source, name));
                }
                ProofQualityRule::MissingTypeAnnotation => {
                    all_issues.extend(check_missing_type_annotation(source, name));
                }
            }
        }
    }

    // Deduplicate identical messages that arise from multiple names.
    all_issues.dedup_by(|a, b| a.message == b.message);

    let score = compute_proof_score(&all_issues, &cfg.score_weights);
    let suggestions = derive_suggestions(&all_issues);

    ProofQualityReport {
        issues: all_issues,
        score,
        suggestions,
    }
}

/// Compute a quality score in `[0.0, 1.0]` from the issue list.
/// Each issue with a matching weight key reduces the score by that weight.
pub fn compute_proof_score(issues: &[ProofQualityIssue], weights: &HashMap<String, f64>) -> f64 {
    let mut penalty = 0.0f64;
    for issue in issues {
        let key = issue.rule.to_string();
        let w = weights.get(&key).copied().unwrap_or(0.05);
        penalty += w;
    }
    (1.0 - penalty).max(0.0)
}

/// Format a `ProofQualityReport` as a human-readable string.
pub fn format_proof_report(report: &ProofQualityReport) -> String {
    let mut out = String::new();
    out.push_str(&format!("Proof Quality Score: {:.2}\n", report.score));
    if report.issues.is_empty() {
        out.push_str("No issues found.\n");
    } else {
        out.push_str(&format!("{} issue(s):\n", report.issues.len()));
        for issue in &report.issues {
            out.push_str(&format!("  - {}\n", issue));
            if let Some(ref sug) = issue.suggestion {
                out.push_str(&format!("    Suggestion: {}\n", sug));
            }
        }
    }
    if !report.suggestions.is_empty() {
        out.push_str("Suggestions:\n");
        for s in &report.suggestions {
            out.push_str(&format!("  * {}\n", s));
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Internal helpers used by run_proof_quality_checks
// ---------------------------------------------------------------------------

/// Extract all theorem/lemma/def names from `source`.
pub(super) fn extract_decl_names(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        for prefix in &["theorem ", "lemma ", "def "] {
            if trimmed.starts_with(prefix) {
                let rest = trimmed[prefix.len()..].trim_start();
                // Name ends at first whitespace, '(', '[', '{', or ':'
                let end = rest
                    .find(|c: char| c.is_whitespace() || matches!(c, '(' | '[' | '{' | ':'))
                    .unwrap_or(rest.len());
                let name = &rest[..end];
                if !name.is_empty() {
                    names.push(name.to_owned());
                }
                break;
            }
        }
    }
    names
}

/// Emit an issue when a lemma/theorem body is trivially short (≤ 1 real token
/// after `:=` or `by`).
pub(super) fn check_trivial_lemma(source: &str, name: &str) -> Vec<ProofQualityIssue> {
    let mut issues = Vec::new();
    for (line_idx, line) in source.lines().enumerate() {
        if !line.contains(name) {
            continue;
        }
        if !line.contains("theorem") && !line.contains("lemma") {
            continue;
        }
        // Find the proof body (after `:=` or `by`).
        let body = if let Some(idx) = line.find(":=") {
            line[idx + 2..].trim()
        } else if let Some(idx) = line.find(" by ") {
            line[idx + 4..].trim()
        } else {
            ""
        };
        if !body.is_empty() && body.split_whitespace().count() <= 1 && body != "sorry" {
            issues.push(
                ProofQualityIssue::new(
                    ProofQualityRule::TrivialLemma,
                    ((line_idx + 1) as u32, 1),
                    format!(
                        "Lemma `{}` has a trivially short proof body: `{}`",
                        name, body
                    ),
                )
                .with_suggestion(
                    "Verify this lemma is non-trivial or merge it into the call site.",
                ),
            );
        }
    }
    issues
}

/// Emit an issue when a declaration lacks a `:` type annotation before `:=`.
pub(super) fn check_missing_type_annotation(source: &str, name: &str) -> Vec<ProofQualityIssue> {
    let mut issues = Vec::new();
    for (line_idx, line) in source.lines().enumerate() {
        if !line.contains(name) {
            continue;
        }
        let is_decl = line.contains("theorem ") || line.contains("lemma ") || line.contains("def ");
        if !is_decl {
            continue;
        }
        // Check: `:=` present but no `:` before it (excluding `:=` itself).
        if let Some(assign_idx) = line.find(":=") {
            let before = &line[..assign_idx];
            // There should be a `:` that is NOT part of `:=`.
            let has_type = before.contains(':');
            if !has_type {
                issues.push(
                    ProofQualityIssue::new(
                        ProofQualityRule::MissingTypeAnnotation,
                        ((line_idx + 1) as u32, 1),
                        format!("Declaration `{}` has no explicit type annotation", name),
                    )
                    .with_suggestion("Add a `: Type` annotation before `:=` for clarity."),
                );
            }
        }
    }
    issues
}

/// Derive top-level suggestion strings from the issue list.
fn derive_suggestions(issues: &[ProofQualityIssue]) -> Vec<String> {
    let mut suggestions = Vec::new();
    let sorry_count = issues
        .iter()
        .filter(|i| i.rule == ProofQualityRule::AvoidSorry)
        .count();
    if sorry_count > 0 {
        suggestions.push(format!(
            "Complete {} sorry placeholder(s) to produce a sound proof.",
            sorry_count
        ));
    }
    let unused_hyps = issues
        .iter()
        .filter(|i| i.rule == ProofQualityRule::UnusedHypothesis)
        .count();
    if unused_hyps > 0 {
        suggestions.push(format!(
            "Review {} unused hypothesis(/es) and remove them.",
            unused_hyps
        ));
    }
    let long_proofs = issues
        .iter()
        .filter(|i| matches!(i.rule, ProofQualityRule::LongProof { .. }))
        .count();
    if long_proofs > 0 {
        suggestions.push("Consider splitting long proofs into helper lemmas.".to_owned());
    }
    suggestions
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn default_cfg() -> ProofQualityConfig {
        ProofQualityConfig::default()
    }

    // --- analyze_proof ---

    #[test]
    fn test_analyze_proof_name() {
        let src = "theorem foo : True := trivial";
        let a = analyze_proof(src, "foo");
        assert_eq!(a.name, "foo");
    }

    #[test]
    fn test_analyze_proof_no_sorry() {
        let src = "theorem bar : True := trivial";
        let a = analyze_proof(src, "bar");
        assert_eq!(a.sorry_count, 0);
    }

    #[test]
    fn test_analyze_proof_with_sorry() {
        let src = "theorem baz : True := sorry";
        let a = analyze_proof(src, "baz");
        assert_eq!(a.sorry_count, 1);
    }

    #[test]
    fn test_analyze_proof_multiple_sorries() {
        let src = "theorem multi : 1 = 1 := by\n  sorry\n  sorry\n";
        let a = analyze_proof(src, "multi");
        assert!(a.sorry_count >= 2);
    }

    #[test]
    fn test_analyze_proof_line_count() {
        let src = "theorem long_one : True := by\n  trivial\n  rfl\n";
        let a = analyze_proof(src, "long_one");
        assert!(a.proof_lines >= 3);
    }

    #[test]
    fn test_analyze_proof_tactics() {
        let src = "theorem t : True := by\n  simp\n  rfl\n";
        let a = analyze_proof(src, "t");
        assert!(a.tactics_used.contains(&"simp".to_owned()));
        assert!(a.tactics_used.contains(&"rfl".to_owned()));
    }

    #[test]
    fn test_analyze_proof_hypotheses() {
        let src = "theorem h_test (h : True) : True := by\n  intro h1\n  exact h1\n";
        let a = analyze_proof(src, "h_test");
        assert!(a.hypotheses.contains(&"h1".to_owned()));
    }

    // --- check_sorry_usage ---

    #[test]
    fn test_check_sorry_none() {
        let a = ProofAnalysis {
            name: "ok".to_owned(),
            sorry_count: 0,
            ..ProofAnalysis::default()
        };
        assert!(check_sorry_usage(&a).is_empty());
    }

    #[test]
    fn test_check_sorry_found() {
        let a = ProofAnalysis {
            name: "bad".to_owned(),
            sorry_count: 2,
            ..ProofAnalysis::default()
        };
        let issues = check_sorry_usage(&a);
        assert_eq!(issues.len(), 1);
        assert_eq!(issues[0].rule, ProofQualityRule::AvoidSorry);
        assert!(issues[0].suggestion.is_some());
    }

    // --- check_nesting_depth ---

    #[test]
    fn test_nesting_depth_ok() {
        let src = "theorem t : True := { trivial }";
        let issues = check_nesting_depth(src, 5);
        assert!(issues.is_empty());
    }

    #[test]
    fn test_nesting_depth_exceeded() {
        let src = "theorem t : True := { { { { { { deep } } } } } }";
        let issues = check_nesting_depth(src, 2);
        assert!(!issues.is_empty());
        assert!(matches!(
            issues[0].rule,
            ProofQualityRule::DeepNesting { max_depth: 2 }
        ));
    }

    // --- check_proof_length ---

    #[test]
    fn test_proof_length_ok() {
        let a = ProofAnalysis {
            name: "short".to_owned(),
            proof_lines: 10,
            ..ProofAnalysis::default()
        };
        assert!(check_proof_length(&a, 50).is_empty());
    }

    #[test]
    fn test_proof_length_exceeded() {
        let a = ProofAnalysis {
            name: "giant".to_owned(),
            proof_lines: 200,
            ..ProofAnalysis::default()
        };
        let issues = check_proof_length(&a, 100);
        assert_eq!(issues.len(), 1);
        assert!(matches!(
            issues[0].rule,
            ProofQualityRule::LongProof { max_lines: 100 }
        ));
    }

    // --- check_repeated_tactics ---

    #[test]
    fn test_repeated_tactics_none() {
        let a = ProofAnalysis {
            name: "t".to_owned(),
            tactics_used: vec!["simp".to_owned(), "rfl".to_owned()],
            ..ProofAnalysis::default()
        };
        assert!(check_repeated_tactics(&a, 3).is_empty());
    }

    #[test]
    fn test_repeated_tactics_found() {
        let a = ProofAnalysis {
            name: "t".to_owned(),
            tactics_used: vec![
                "simp".to_owned(),
                "simp".to_owned(),
                "simp".to_owned(),
                "rfl".to_owned(),
            ],
            ..ProofAnalysis::default()
        };
        let issues = check_repeated_tactics(&a, 3);
        assert!(!issues.is_empty());
        assert!(issues.iter().any(|i| i.message.contains("simp")));
    }

    // --- check_unused_hypotheses ---

    #[test]
    fn test_unused_hyps_none() {
        let a = ProofAnalysis {
            name: "t".to_owned(),
            hypotheses: vec!["h".to_owned()],
            hypotheses_used: vec!["h".to_owned()],
            ..ProofAnalysis::default()
        };
        assert!(check_unused_hypotheses(&a).is_empty());
    }

    #[test]
    fn test_unused_hyps_found() {
        let a = ProofAnalysis {
            name: "t".to_owned(),
            hypotheses: vec!["h1".to_owned(), "h2".to_owned()],
            hypotheses_used: vec!["h1".to_owned()],
            ..ProofAnalysis::default()
        };
        let issues = check_unused_hypotheses(&a);
        assert_eq!(issues.len(), 1);
        assert!(issues[0].message.contains("h2"));
    }

    // --- compute_proof_score ---

    #[test]
    fn test_score_no_issues() {
        let score = compute_proof_score(&[], &HashMap::new());
        assert!((score - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_score_decreases_with_issues() {
        let mut weights = HashMap::new();
        weights.insert("avoid_sorry".to_owned(), 0.4);
        let issues = vec![ProofQualityIssue::new(
            ProofQualityRule::AvoidSorry,
            (1, 1),
            "sorry",
        )];
        let score = compute_proof_score(&issues, &weights);
        assert!((score - 0.6).abs() < 1e-9);
    }

    #[test]
    fn test_score_floor_zero() {
        let mut weights = HashMap::new();
        weights.insert("avoid_sorry".to_owned(), 2.0);
        let issues = vec![ProofQualityIssue::new(
            ProofQualityRule::AvoidSorry,
            (1, 1),
            "sorry",
        )];
        let score = compute_proof_score(&issues, &weights);
        assert!(score >= 0.0);
    }

    // --- format_proof_report ---

    #[test]
    fn test_format_report_clean() {
        use super::super::types::ProofQualityReport;
        let report = ProofQualityReport {
            issues: vec![],
            score: 1.0,
            suggestions: vec![],
        };
        let s = format_proof_report(&report);
        assert!(s.contains("1.00"));
        assert!(s.contains("No issues found"));
    }

    #[test]
    fn test_format_report_with_issues() {
        use super::super::types::ProofQualityReport;
        let report = ProofQualityReport {
            issues: vec![ProofQualityIssue::new(
                ProofQualityRule::AvoidSorry,
                (1, 1),
                "has sorry",
            )],
            score: 0.6,
            suggestions: vec!["Fix it".to_owned()],
        };
        let s = format_proof_report(&report);
        assert!(s.contains("0.60"));
        assert!(s.contains("sorry"));
        assert!(s.contains("Fix it"));
    }

    // --- run_proof_quality_checks ---

    #[test]
    fn test_run_checks_clean() {
        let src = "theorem easy : True := trivial";
        let cfg = ProofQualityConfig {
            rules: vec![ProofQualityRule::AvoidSorry],
            score_weights: HashMap::new(),
        };
        let report = run_proof_quality_checks(src, &cfg);
        assert!(report
            .issues
            .iter()
            .all(|i| i.rule != ProofQualityRule::AvoidSorry));
    }

    #[test]
    fn test_run_checks_sorry_detected() {
        let src = "theorem broken : True := sorry";
        let cfg = ProofQualityConfig {
            rules: vec![ProofQualityRule::AvoidSorry],
            score_weights: HashMap::new(),
        };
        let report = run_proof_quality_checks(src, &cfg);
        assert!(report
            .issues
            .iter()
            .any(|i| i.rule == ProofQualityRule::AvoidSorry));
    }

    #[test]
    fn test_run_checks_report_score_range() {
        let src = "theorem t : True := sorry";
        let cfg = default_cfg();
        let report = run_proof_quality_checks(src, &cfg);
        assert!((0.0..=1.0).contains(&report.score));
    }

    #[test]
    fn test_extract_decl_names() {
        let src = "theorem foo : True := trivial\nlemma bar : False := sorry\ndef baz := 42";
        let names = extract_decl_names(src);
        assert!(names.contains(&"foo".to_owned()));
        assert!(names.contains(&"bar".to_owned()));
        assert!(names.contains(&"baz".to_owned()));
    }

    #[test]
    fn test_trivial_lemma_detection() {
        let src = "lemma triv : True := trivial";
        let issues = check_trivial_lemma(src, "triv");
        // trivial is a single token — should flag it.
        assert!(!issues.is_empty());
    }

    #[test]
    fn test_missing_type_annotation_detected() {
        let src = "def no_type := 42";
        let issues = check_missing_type_annotation(src, "no_type");
        assert!(!issues.is_empty());
        assert!(issues
            .iter()
            .any(|i| i.rule == ProofQualityRule::MissingTypeAnnotation));
    }

    #[test]
    fn test_missing_type_annotation_ok() {
        let src = "def has_type : Nat := 42";
        let issues = check_missing_type_annotation(src, "has_type");
        assert!(issues.is_empty());
    }

    #[test]
    fn test_proof_quality_issue_display() {
        let issue = ProofQualityIssue::new(ProofQualityRule::AvoidSorry, (3, 1), "sorry found");
        let s = issue.to_string();
        assert!(s.contains("avoid_sorry"));
        assert!(s.contains("3:1"));
    }

    #[test]
    fn test_proof_quality_report_is_clean() {
        use super::super::types::ProofQualityReport;
        let r = ProofQualityReport::default();
        assert!(r.is_clean());
    }

    #[test]
    fn test_proof_quality_report_count_by_rule() {
        use super::super::types::ProofQualityReport;
        let r = ProofQualityReport {
            issues: vec![
                ProofQualityIssue::new(ProofQualityRule::AvoidSorry, (1, 1), "s"),
                ProofQualityIssue::new(ProofQualityRule::AvoidSorry, (2, 1), "s2"),
                ProofQualityIssue::new(ProofQualityRule::UnusedHypothesis, (3, 1), "u"),
            ],
            score: 0.5,
            suggestions: vec![],
        };
        assert_eq!(r.count_by_rule(&ProofQualityRule::AvoidSorry), 2);
        assert_eq!(r.count_by_rule(&ProofQualityRule::UnusedHypothesis), 1);
    }

    #[test]
    fn test_config_default_rules() {
        let cfg = ProofQualityConfig::default();
        assert!(!cfg.rules.is_empty());
        assert!(cfg.rules.contains(&ProofQualityRule::AvoidSorry));
    }

    #[test]
    fn test_collect_tactics_finds_simp() {
        let text = "simp rfl ring omega simp simp";
        let tactics = collect_tactics(text);
        assert_eq!(tactics.iter().filter(|t| t.as_str() == "simp").count(), 3);
    }

    #[test]
    fn test_compute_nesting_depth_empty() {
        assert_eq!(compute_nesting_depth("theorem x := trivial"), 0);
    }

    #[test]
    fn test_compute_nesting_depth_nested() {
        let text = "{{ { } }}";
        assert!(compute_nesting_depth(text) >= 3);
    }
}
