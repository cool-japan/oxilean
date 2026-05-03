//! Functions for dead-declaration lint analysis.

use super::types::{
    DeadDeclConfig, DeadDeclIssue, DeadDeclKind, DeadDeclReport, DeclRef, DeclUsage,
};

// ---------------------------------------------------------------------------
// Declaration-keyword helpers
// ---------------------------------------------------------------------------

/// Keywords that introduce a named declaration in OxiLean/Lean4 source.
static DECL_KEYWORDS: &[&str] = &[
    "theorem",
    "lemma",
    "def",
    "axiom",
    "noncomputable def",
    "abbrev",
    "instance",
    "class",
    "structure",
    "inductive",
];

/// Keywords indicating public visibility.
static PUBLIC_KEYWORDS: &[&str] = &["pub", "export"];

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Extract all declarations from `source`.
///
/// For each line that starts with a declaration keyword (optionally preceded
/// by visibility modifiers), a `DeclUsage` is produced.
pub fn extract_declarations(source: &str) -> Vec<DeclUsage> {
    let mut decls = Vec::new();
    for (line_idx, line) in source.lines().enumerate() {
        let line_num = (line_idx + 1) as u32;
        let trimmed = line.trim();
        // Strip visibility / attribute prefixes.
        let stripped = strip_attributes(trimmed);
        let is_exported = PUBLIC_KEYWORDS.iter().any(|kw| trimmed.starts_with(kw));
        let is_axiom = stripped.starts_with("axiom ");

        // Find the first matching declaration keyword.
        let mut matched_name: Option<String> = None;
        for kw in DECL_KEYWORDS {
            if stripped.starts_with(kw) {
                let rest = stripped[kw.len()..].trim_start();
                // Name ends at first whitespace, '(', '[', '{', ':'.
                let end = rest
                    .find(|c: char| c.is_whitespace() || matches!(c, '(' | '[' | '{' | ':'))
                    .unwrap_or(rest.len());
                let name = rest[..end].trim();
                if !name.is_empty() && is_valid_ident(name) {
                    matched_name = Some(name.to_owned());
                }
                break;
            }
        }

        if let Some(name) = matched_name {
            decls.push(DeclUsage::new(name, (line_num, 1), is_exported, is_axiom));
        }
    }
    decls
}

/// Find every name-reference in `source`.
///
/// Heuristic: every whitespace-delimited token that looks like a valid
/// identifier and is NOT a keyword is returned as a `DeclRef`.
pub fn find_references(source: &str) -> Vec<DeclRef> {
    let mut refs = Vec::new();
    for (line_idx, line) in source.lines().enumerate() {
        let line_num = (line_idx + 1) as u32;
        let mut col = 1u32;
        for token in line.split(|c: char| {
            c.is_whitespace()
                || matches!(
                    c,
                    '(' | ')'
                        | ':'
                        | ','
                        | ';'
                        | '.'
                        | '{'
                        | '}'
                        | '['
                        | ']'
                        | '"'
                        | '`'
                        | '\''
                        | '#'
                        | '@'
                )
        }) {
            if is_valid_ident(token) && !is_keyword(token) {
                refs.push(DeclRef::new(token, (line_num, col)));
            }
            col += token.len() as u32 + 1;
        }
    }
    refs
}

/// Link `refs` to their corresponding `decls`, filling in `used_at`.
///
/// Returns a new `Vec<DeclUsage>` with `used_at` populated.  The
/// declaration site itself is excluded from the reference set so that
/// a declaration is not counted as using itself.
pub fn compute_usage(decls: &[DeclUsage], refs: &[DeclRef]) -> Vec<DeclUsage> {
    let mut result: Vec<DeclUsage> = decls.to_vec();
    for r in refs {
        for decl in &mut result {
            if decl.name != r.name {
                continue;
            }
            // Skip any reference that is on the same line as the declaration
            // (the name token that appears in the declaration header itself).
            if decl.defined_at.0 == r.location.0 {
                continue;
            }
            decl.used_at.push(r.location);
        }
    }
    result
}

/// Run the full dead-declaration analysis on `source` and return a report.
pub fn check_dead_declarations(source: &str, cfg: &DeadDeclConfig) -> DeadDeclReport {
    let raw_decls = extract_declarations(source);
    let refs = find_references(source);
    let decls = compute_usage(&raw_decls, &refs);
    let total_decls = decls.len();

    let mut issues: Vec<DeadDeclIssue> = Vec::new();

    // Detect shadowed declarations (same name declared more than once).
    detect_shadowed(&decls, &mut issues, cfg);

    for decl in &decls {
        // Skip names matched by ignore patterns.
        if cfg.is_ignored(&decl.name) {
            continue;
        }

        // Exported declarations are considered alive.
        if decl.is_exported {
            continue;
        }

        // Redundant axiom: an axiom that is never referenced AND is not exported.
        if decl.is_axiom && !decl.is_used() {
            issues.push(DeadDeclIssue::new(
                decl.name.clone(),
                DeadDeclKind::RedundantAxiom,
                decl.defined_at,
            ));
            continue;
        }

        if !decl.is_used() {
            // Distinguish private-unused from completely unused.
            let kind = if cfg.warn_private {
                DeadDeclKind::PrivateUnused
            } else {
                DeadDeclKind::Unused
            };
            issues.push(DeadDeclIssue::new(decl.name.clone(), kind, decl.defined_at));
        } else if cfg.warn_tests_only {
            // Check whether all use sites are inside test blocks.
            let all_in_tests = decl
                .used_at
                .iter()
                .all(|loc| is_test_reference(*loc, source));
            if all_in_tests {
                issues.push(DeadDeclIssue::new(
                    decl.name.clone(),
                    DeadDeclKind::OnlyUsedInTests,
                    decl.defined_at,
                ));
            }
        }
    }

    let dead_count = issues.len();
    DeadDeclReport {
        issues,
        total_decls,
        dead_count,
    }
}

/// Return `true` when `location` lies inside a `#[test]` or `#[cfg(test)]`
/// annotated block in `source`.
///
/// Heuristic: scan upward from the given line number for a `#[test]` or
/// `#[cfg(test)]` attribute within the same brace depth.
pub fn is_test_reference(location: (u32, u32), source: &str) -> bool {
    let target_line = location.0 as usize;
    let lines: Vec<&str> = source.lines().collect();
    if target_line == 0 || target_line > lines.len() {
        return false;
    }
    // Walk upward from target_line, scanning at most 50 lines back.
    // Return true if we find `#[test]` or `#[cfg(test)]` before encountering
    // a clearly unrelated top-level item boundary.
    let scan_start = target_line.saturating_sub(50);
    for idx in (scan_start..target_line).rev() {
        let line = lines[idx].trim();
        if line.contains("#[test]") || line.contains("#[cfg(test)]") {
            return true;
        }
        // Stop scanning upward when we see a module-level declaration that
        // would indicate we have left the potential test block entirely.
        // These keywords at column 0 are clear item boundaries that are not
        // part of a test wrapper:
        let raw = lines[idx];
        let at_col0 = !raw.starts_with(' ') && !raw.starts_with('\t');
        if at_col0 {
            let is_boundary = line.starts_with("theorem ")
                || line.starts_with("lemma ")
                || line.starts_with("def ")
                || line.starts_with("axiom ")
                || line.starts_with("struct ")
                || line.starts_with("enum ")
                || line.starts_with("impl ")
                || line.starts_with("trait ")
                || line.starts_with("mod ");
            if is_boundary {
                break;
            }
        }
    }
    false
}

/// Format a `DeadDeclReport` as a human-readable string.
pub fn format_dead_report(report: &DeadDeclReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Dead Declaration Report: {}/{} declarations dead ({:.1}%)\n",
        report.dead_count,
        report.total_decls,
        report.dead_ratio() * 100.0,
    ));
    if report.issues.is_empty() {
        out.push_str("No dead declarations found.\n");
    } else {
        for issue in &report.issues {
            out.push_str(&format!("  - {}\n", issue));
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Strip Rust/Lean-style `#[...]` attribute lines and leading visibility
/// keywords so we can match declaration keywords cleanly.
fn strip_attributes(line: &str) -> &str {
    let stripped =
        line.trim_start_matches(|c: char| c.is_whitespace() || c == '#' || c == '[' || c == ']');
    // Remove common visibility prefixes.
    for prefix in &[
        "pub(crate) ",
        "pub(super) ",
        "pub ",
        "private ",
        "protected ",
    ] {
        if stripped.starts_with(prefix) {
            return &stripped[prefix.len()..];
        }
    }
    stripped
}

/// Return `true` when `s` is a plausible identifier (alphanumeric + `_`,
/// starts with a letter or `_`).
fn is_valid_ident(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap_or('\0');
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
}

/// Lean4/OxiLean/Rust keywords that should not be treated as name references.
fn is_keyword(s: &str) -> bool {
    matches!(
        s,
        "theorem"
            | "lemma"
            | "def"
            | "axiom"
            | "abbrev"
            | "instance"
            | "class"
            | "structure"
            | "inductive"
            | "where"
            | "with"
            | "do"
            | "let"
            | "have"
            | "show"
            | "by"
            | "fun"
            | "match"
            | "if"
            | "then"
            | "else"
            | "return"
            | "in"
            | "at"
            | "from"
            | "import"
            | "open"
            | "namespace"
            | "end"
            | "section"
            | "variable"
            | "noncomputable"
            | "private"
            | "protected"
            | "pub"
            | "use"
            | "mod"
            | "fn"
            | "impl"
            | "trait"
            | "for"
            | "type"
            | "mut"
            | "ref"
            | "self"
            | "Self"
            | "super"
            | "crate"
            | "extern"
            | "unsafe"
            | "async"
            | "await"
            | "move"
            | "static"
            | "const"
            | "true"
            | "false"
            | "True"
            | "False"
            | "Prop"
            | "Type"
            | "Sort"
            | "sorry"
            | "trivial"
            | "rfl"
            | "simp"
            | "ring"
            | "omega"
    )
}

/// Detect declarations that share a name (shadowing) and emit issues.
fn detect_shadowed(decls: &[DeclUsage], issues: &mut Vec<DeadDeclIssue>, cfg: &DeadDeclConfig) {
    use std::collections::HashMap;
    let mut name_map: HashMap<&str, Vec<&DeclUsage>> = HashMap::new();
    for d in decls {
        name_map.entry(d.name.as_str()).or_default().push(d);
    }
    for (name, group) in &name_map {
        if group.len() < 2 {
            continue;
        }
        if cfg.is_ignored(name) {
            continue;
        }
        // All but the last occurrence are shadowed.
        for earlier in &group[..group.len() - 1] {
            let shadower = group[group.len() - 1];
            issues.push(DeadDeclIssue::new(
                (*name).to_owned(),
                DeadDeclKind::Shadowed {
                    by: shadower.name.clone(),
                },
                earlier.defined_at,
            ));
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- extract_declarations ---

    #[test]
    fn test_extract_theorem() {
        let src = "theorem foo : True := trivial";
        let decls = extract_declarations(src);
        assert!(decls.iter().any(|d| d.name == "foo"));
    }

    #[test]
    fn test_extract_lemma() {
        let src = "lemma bar : False := sorry";
        let decls = extract_declarations(src);
        assert!(decls.iter().any(|d| d.name == "bar"));
    }

    #[test]
    fn test_extract_def() {
        let src = "def myVal : Nat := 42";
        let decls = extract_declarations(src);
        assert!(decls.iter().any(|d| d.name == "myVal"));
    }

    #[test]
    fn test_extract_axiom() {
        let src = "axiom myAxiom : Prop";
        let decls = extract_declarations(src);
        let axiom = decls.iter().find(|d| d.name == "myAxiom");
        assert!(axiom.is_some());
        assert!(axiom.unwrap().is_axiom);
    }

    #[test]
    fn test_extract_multiple_decls() {
        let src = "theorem a : True := trivial\nlemma b : True := trivial\ndef c := 1";
        let decls = extract_declarations(src);
        assert_eq!(decls.len(), 3);
    }

    #[test]
    fn test_extract_location() {
        let src = "theorem foo : True := trivial";
        let decls = extract_declarations(src);
        let d = decls.iter().find(|d| d.name == "foo").unwrap();
        assert_eq!(d.defined_at.0, 1);
    }

    // --- find_references ---

    #[test]
    fn test_find_references_basic() {
        let src = "theorem uses_foo : Prop := foo";
        let refs = find_references(src);
        assert!(refs.iter().any(|r| r.name == "foo"));
    }

    #[test]
    fn test_find_references_excludes_keywords() {
        let src = "theorem t : True := trivial";
        let refs = find_references(src);
        assert!(!refs.iter().any(|r| r.name == "theorem"));
        assert!(!refs.iter().any(|r| r.name == "trivial"));
    }

    #[test]
    fn test_find_references_location() {
        let src = "foo";
        let refs = find_references(src);
        assert!(refs.iter().any(|r| r.name == "foo" && r.location.0 == 1));
    }

    // --- compute_usage ---

    #[test]
    fn test_compute_usage_marks_used() {
        let decls = vec![DeclUsage::new("helper", (1, 1), false, false)];
        let refs = vec![DeclRef::new("helper", (3, 5))];
        let updated = compute_usage(&decls, &refs);
        assert!(updated[0].is_used());
        assert_eq!(updated[0].used_at[0], (3, 5));
    }

    #[test]
    fn test_compute_usage_skips_self_def() {
        let decls = vec![DeclUsage::new("foo", (1, 1), false, false)];
        let refs = vec![DeclRef::new("foo", (1, 1))]; // same location → self
        let updated = compute_usage(&decls, &refs);
        assert!(!updated[0].is_used());
    }

    #[test]
    fn test_compute_usage_multiple_refs() {
        let decls = vec![DeclUsage::new("x", (1, 1), false, false)];
        let refs = vec![DeclRef::new("x", (2, 1)), DeclRef::new("x", (3, 1))];
        let updated = compute_usage(&decls, &refs);
        assert_eq!(updated[0].used_at.len(), 2);
    }

    // --- check_dead_declarations ---

    #[test]
    fn test_dead_decl_unused() {
        let src = "theorem unused_lemma : True := trivial";
        let cfg = DeadDeclConfig::default();
        let report = check_dead_declarations(src, &cfg);
        assert!(report.issues.iter().any(|i| i.name == "unused_lemma"));
    }

    #[test]
    fn test_dead_decl_used_not_flagged() {
        let src = "theorem helper : True := trivial\ntheorem main : True := helper";
        let cfg = DeadDeclConfig::default();
        let report = check_dead_declarations(src, &cfg);
        // `helper` is referenced by `main`, so it should NOT be dead.
        assert!(!report.issues.iter().any(|i| i.name == "helper"
            && matches!(i.kind, DeadDeclKind::Unused | DeadDeclKind::PrivateUnused)));
    }

    #[test]
    fn test_dead_decl_axiom_unused() {
        let src = "axiom badAxiom : False";
        let cfg = DeadDeclConfig::default();
        let report = check_dead_declarations(src, &cfg);
        assert!(report
            .issues
            .iter()
            .any(|i| i.name == "badAxiom" && i.kind == DeadDeclKind::RedundantAxiom));
    }

    #[test]
    fn test_dead_decl_report_counts() {
        let src = "theorem a : True := trivial\ntheorem b : True := trivial";
        let cfg = DeadDeclConfig::default();
        let report = check_dead_declarations(src, &cfg);
        assert_eq!(report.total_decls, 2);
        assert!(report.dead_count > 0);
    }

    #[test]
    fn test_dead_decl_ignore_pattern() {
        let src = "theorem _unused : True := trivial";
        let cfg = DeadDeclConfig {
            ignore_patterns: vec!["_".to_owned()],
            ..DeadDeclConfig::default()
        };
        let report = check_dead_declarations(src, &cfg);
        // `_unused` starts with `_` → ignored.
        assert!(!report.issues.iter().any(|i| i.name == "_unused"));
    }

    #[test]
    fn test_dead_decl_warn_private_false() {
        let src = "theorem secret : True := trivial";
        let cfg = DeadDeclConfig {
            warn_private: false,
            ..DeadDeclConfig::default()
        };
        let report = check_dead_declarations(src, &cfg);
        // With warn_private false, kind is Unused not PrivateUnused.
        assert!(report
            .issues
            .iter()
            .any(|i| i.name == "secret" && i.kind == DeadDeclKind::Unused));
    }

    // --- is_test_reference ---

    #[test]
    fn test_is_test_reference_true() {
        let src = "#[test]\nfn my_test() {\n    helper()\n}";
        // Line 3 is inside the test.
        assert!(is_test_reference((3, 5), src));
    }

    #[test]
    fn test_is_test_reference_false() {
        let src = "theorem normal : True := trivial";
        assert!(!is_test_reference((1, 1), src));
    }

    #[test]
    fn test_is_test_reference_out_of_bounds() {
        let src = "theorem t : True := trivial";
        assert!(!is_test_reference((999, 1), src));
    }

    // --- format_dead_report ---

    #[test]
    fn test_format_dead_report_clean() {
        let report = DeadDeclReport {
            issues: vec![],
            total_decls: 5,
            dead_count: 0,
        };
        let s = format_dead_report(&report);
        assert!(s.contains("0/5"));
        assert!(s.contains("No dead"));
    }

    #[test]
    fn test_format_dead_report_with_issue() {
        let report = DeadDeclReport {
            issues: vec![DeadDeclIssue::new("orphan", DeadDeclKind::Unused, (2, 1))],
            total_decls: 3,
            dead_count: 1,
        };
        let s = format_dead_report(&report);
        assert!(s.contains("orphan"));
        assert!(s.contains("1/3"));
    }

    // --- DeadDeclConfig helpers ---

    #[test]
    fn test_config_is_ignored_exact() {
        let cfg = DeadDeclConfig {
            ignore_patterns: vec!["ignore_me".to_owned()],
            ..DeadDeclConfig::default()
        };
        assert!(cfg.is_ignored("ignore_me"));
        assert!(!cfg.is_ignored("other"));
    }

    #[test]
    fn test_config_is_ignored_wildcard() {
        let cfg = DeadDeclConfig {
            ignore_patterns: vec!["test_*".to_owned()],
            ..DeadDeclConfig::default()
        };
        assert!(cfg.is_ignored("test_foo"));
        assert!(!cfg.is_ignored("foo_test"));
    }

    // --- DeadDeclReport helpers ---

    #[test]
    fn test_report_is_clean() {
        let r = DeadDeclReport::default();
        assert!(r.is_clean());
    }

    #[test]
    fn test_report_dead_ratio() {
        let r = DeadDeclReport {
            total_decls: 10,
            dead_count: 3,
            issues: vec![],
        };
        assert!((r.dead_ratio() - 0.3).abs() < 1e-9);
    }

    #[test]
    fn test_report_dead_ratio_zero_total() {
        let r = DeadDeclReport::default();
        assert!((r.dead_ratio() - 0.0).abs() < 1e-9);
    }

    // --- DeclUsage ---

    #[test]
    fn test_decl_usage_is_used() {
        let mut d = DeclUsage::new("x", (1, 1), false, false);
        assert!(!d.is_used());
        d.used_at.push((2, 1));
        assert!(d.is_used());
    }

    // --- DeadDeclKind Display ---

    #[test]
    fn test_dead_decl_kind_display() {
        assert_eq!(DeadDeclKind::Unused.to_string(), "unused");
        assert_eq!(
            DeadDeclKind::Shadowed {
                by: "bar".to_owned()
            }
            .to_string(),
            "shadowed_by(bar)"
        );
        assert_eq!(DeadDeclKind::RedundantAxiom.to_string(), "redundant_axiom");
        assert_eq!(
            DeadDeclKind::OnlyUsedInTests.to_string(),
            "only_used_in_tests"
        );
        assert_eq!(DeadDeclKind::PrivateUnused.to_string(), "private_unused");
    }

    // --- DeadDeclIssue Display ---

    #[test]
    fn test_dead_decl_issue_display() {
        let issue = DeadDeclIssue::new("foo", DeadDeclKind::Unused, (5, 1));
        let s = issue.to_string();
        assert!(s.contains("foo"));
        assert!(s.contains("5:1"));
        assert!(s.contains("unused"));
    }

    // --- is_valid_ident / is_keyword (via extract / find_refs) ---

    #[test]
    fn test_no_numeric_refs() {
        let src = "def x := 42";
        let refs = find_references(src);
        // "42" should not appear as a reference.
        assert!(!refs.iter().any(|r| r.name == "42"));
    }

    #[test]
    fn test_shadowed_detection() {
        let src = "theorem dup : True := trivial\ntheorem dup : Prop := trivial";
        let cfg = DeadDeclConfig {
            ignore_patterns: vec![],
            ..DeadDeclConfig::default()
        };
        let report = check_dead_declarations(src, &cfg);
        assert!(report
            .issues
            .iter()
            .any(|i| matches!(&i.kind, DeadDeclKind::Shadowed { .. })));
    }
}
