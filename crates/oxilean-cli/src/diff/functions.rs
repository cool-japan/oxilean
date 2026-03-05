//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::path::Path;

use super::types::{
    AnnotatedDiffLine, BlameLine, BlameSource, ChangeClass, ChangeKind, CharDiffToken, DeclItem,
    DeclItemKind, DeclSummary, DiffCache, DiffConfig, DiffDisplayTarget, DiffHunk, DiffLine,
    DiffLineKind, DiffResult, DiffStatistics, DiffSummaryReport, DiffWindow, FilePatch,
    FilePatchHunk, FilePatchLine, FilePatchLineKind, KeywordFilter, LineChange, MultiDiff,
    OxiDiffToken, StructDiff, StructuralDiffResult, WordToken,
};

/// Compute the longest common subsequence edit script between two sequences
/// of lines.  Returns a list of `(ChangeKind, old_idx_or_none, new_idx_or_none)`.
fn compute_edit_script(
    old: &[&str],
    new: &[&str],
    ignore_ws: bool,
) -> Vec<(ChangeKind, usize, usize)> {
    let n = old.len();
    let m = new.len();
    let eq = |a: &str, b: &str| -> bool {
        if ignore_ws {
            normalize_whitespace(a) == normalize_whitespace(b)
        } else {
            a == b
        }
    };
    let mut lcs = vec![vec![0u32; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if eq(old[i - 1], new[j - 1]) {
                lcs[i][j] = lcs[i - 1][j - 1] + 1;
            } else {
                lcs[i][j] = lcs[i - 1][j].max(lcs[i][j - 1]);
            }
        }
    }
    let mut result = Vec::new();
    let mut i = n;
    let mut j = m;
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && eq(old[i - 1], new[j - 1]) {
            result.push((ChangeKind::Unchanged, i, j));
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || lcs[i][j - 1] >= lcs[i - 1][j]) {
            result.push((ChangeKind::Added, 0, j));
            j -= 1;
        } else {
            result.push((ChangeKind::Removed, i, 0));
            i -= 1;
        }
    }
    result.reverse();
    result
}
/// Normalize whitespace for comparison: collapse runs of whitespace to single
/// space and trim.
fn normalize_whitespace(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut prev_ws = true;
    for ch in s.chars() {
        if ch.is_whitespace() {
            if !prev_ws {
                result.push(' ');
            }
            prev_ws = true;
        } else {
            result.push(ch);
            prev_ws = false;
        }
    }
    if result.ends_with(' ') {
        result.pop();
    }
    result
}
/// Compute the diff between two strings, producing hunks with context.
pub fn line_diff(old_text: &str, new_text: &str, config: &DiffConfig) -> DiffResult {
    let old_lines: Vec<&str> = old_text.lines().collect();
    let new_lines: Vec<&str> = new_text.lines().collect();
    let edits = compute_edit_script(&old_lines, &new_lines, config.ignore_whitespace);
    let mut all_changes: Vec<LineChange> = Vec::new();
    for (kind, old_idx, new_idx) in &edits {
        let content = match kind {
            ChangeKind::Removed => old_lines[old_idx - 1].to_string(),
            ChangeKind::Added => new_lines[new_idx - 1].to_string(),
            ChangeKind::Unchanged => old_lines[old_idx - 1].to_string(),
        };
        all_changes.push(LineChange::new(kind.clone(), content, *old_idx, *new_idx));
    }
    let hunks = group_into_hunks(&all_changes, config.context_lines);
    let additions = all_changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Added)
        .count();
    let deletions = all_changes
        .iter()
        .filter(|c| c.kind == ChangeKind::Removed)
        .count();
    DiffResult {
        hunks,
        additions,
        deletions,
    }
}
/// Group a flat list of line changes into hunks, including context.
fn group_into_hunks(changes: &[LineChange], context: usize) -> Vec<DiffHunk> {
    if changes.is_empty() {
        return Vec::new();
    }
    let change_indices: Vec<usize> = changes
        .iter()
        .enumerate()
        .filter(|(_, c)| c.kind != ChangeKind::Unchanged)
        .map(|(i, _)| i)
        .collect();
    if change_indices.is_empty() {
        return Vec::new();
    }
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut current_group: Vec<usize> = vec![change_indices[0]];
    for &idx in &change_indices[1..] {
        let prev = *current_group
            .last()
            .expect("current_group is non-empty: initialized with one element");
        if idx <= prev + 2 * context + 1 {
            current_group.push(idx);
        } else {
            groups.push(current_group);
            current_group = vec![idx];
        }
    }
    groups.push(current_group);
    let mut hunks = Vec::new();
    for group in &groups {
        let first = *group
            .first()
            .expect("group is non-empty: produced from non-empty current_group");
        let last = *group
            .last()
            .expect("group is non-empty: produced from non-empty current_group");
        let start = first.saturating_sub(context);
        let end = (last + context + 1).min(changes.len());
        let hunk_lc: Vec<LineChange> = changes[start..end].to_vec();
        let old_start = hunk_lc
            .iter()
            .filter(|l| l.old_lineno > 0)
            .map(|l| l.old_lineno)
            .min()
            .unwrap_or(1);
        let new_start = hunk_lc
            .iter()
            .filter(|l| l.new_lineno > 0)
            .map(|l| l.new_lineno)
            .min()
            .unwrap_or(1);
        let old_count = hunk_lc
            .iter()
            .filter(|l| l.kind != ChangeKind::Added)
            .count();
        let new_count = hunk_lc
            .iter()
            .filter(|l| l.kind != ChangeKind::Removed)
            .count();
        let hunk_lines: Vec<DiffLine> = hunk_lc.iter().map(|l| l.to_diff_line()).collect();
        hunks.push(DiffHunk {
            header: DiffHunk::make_header(old_start, old_count, new_start, new_count),
            old_start,
            old_count,
            new_start,
            new_count,
            lines: hunk_lines,
        });
    }
    hunks
}
/// ANSI escape codes.
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";
/// Format a `DiffResult` as a human-readable unified diff string.
pub fn format_diff(diff: &DiffResult, config: &DiffConfig) -> String {
    if diff.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    if config.color_output {
        out.push_str(&format!("{RED}--- {}{RESET}\n", config.old_label));
        out.push_str(&format!("{GREEN}+++ {}{RESET}\n", config.new_label));
    } else {
        out.push_str(&format!("--- {}\n", config.old_label));
        out.push_str(&format!("+++ {}\n", config.new_label));
    }
    for hunk in &diff.hunks {
        if config.color_output {
            out.push_str(&format!("{CYAN}{}{RESET}\n", hunk.header));
        } else {
            out.push_str(&format!("{}\n", hunk.header));
        }
        for line in &hunk.lines {
            let prefix = line.prefix_char();
            let lineno_prefix = if config.show_line_numbers {
                match line.kind {
                    DiffLineKind::Removed => {
                        format!(
                            "{:>4} {:>4} ",
                            line.old_lineno.map(|n| n.to_string()).unwrap_or_default(),
                            ""
                        )
                    }
                    DiffLineKind::Added => {
                        format!(
                            "{:>4} {:>4} ",
                            "",
                            line.new_lineno.map(|n| n.to_string()).unwrap_or_default()
                        )
                    }
                    DiffLineKind::Context => {
                        format!(
                            "{:>4} {:>4} ",
                            line.old_lineno.map(|n| n.to_string()).unwrap_or_default(),
                            line.new_lineno.map(|n| n.to_string()).unwrap_or_default()
                        )
                    }
                }
            } else {
                String::new()
            };
            if config.color_output {
                let color = match line.kind {
                    DiffLineKind::Removed => RED,
                    DiffLineKind::Added => GREEN,
                    DiffLineKind::Context => "",
                };
                let reset = if color.is_empty() { "" } else { RESET };
                out.push_str(&format!(
                    "{lineno_prefix}{color}{prefix}{}{reset}\n",
                    line.content
                ));
            } else {
                out.push_str(&format!("{lineno_prefix}{prefix}{}\n", line.content));
            }
        }
    }
    out.push_str(&format!(
        "{} addition(s), {} deletion(s)\n",
        diff.additions, diff.deletions
    ));
    out
}
/// Compare two files at the given paths and return a `DiffResult`.
pub fn file_diff(
    old_path: &Path,
    new_path: &Path,
    config: &DiffConfig,
) -> Result<DiffResult, String> {
    let old_text = std::fs::read_to_string(old_path)
        .map_err(|e| format!("cannot read {}: {}", old_path.display(), e))?;
    let new_text = std::fs::read_to_string(new_path)
        .map_err(|e| format!("cannot read {}: {}", new_path.display(), e))?;
    Ok(line_diff(&old_text, &new_text, config))
}
/// Extract a simplified list of declaration summaries from OxiLean source text.
/// This is a heuristic parser that looks for `def`, `theorem`, `axiom`,
/// `lemma`, `instance`, `class`, `structure`, `inductive` keywords at the
/// start of a line.
pub fn extract_decl_summaries(source: &str) -> Vec<DeclSummary> {
    let keywords = [
        "def",
        "theorem",
        "axiom",
        "lemma",
        "instance",
        "class",
        "structure",
        "inductive",
        "abbrev",
        "noncomputable",
    ];
    let mut decls = Vec::new();
    for line in source.lines() {
        let trimmed = line.trim();
        for kw in &keywords {
            let prefix = format!("{kw} ");
            if trimmed.starts_with(&prefix) || trimmed == *kw {
                let rest = trimmed.strip_prefix(&prefix).unwrap_or("").trim();
                let (name, type_sig) = if let Some(colon_pos) = rest.find(':') {
                    let n = rest[..colon_pos].trim().to_string();
                    let t = rest[colon_pos + 1..].trim().to_string();
                    let t = if let Some(assign) = t.find(":=") {
                        t[..assign].trim().to_string()
                    } else {
                        t
                    };
                    (n, t)
                } else {
                    let name_end = rest
                        .find(|c: char| c.is_whitespace() || c == ':' || c == '(')
                        .unwrap_or(rest.len());
                    (rest[..name_end].to_string(), String::new())
                };
                if !name.is_empty() {
                    decls.push(DeclSummary {
                        kind: kw.to_string(),
                        name,
                        type_sig,
                    });
                }
                break;
            }
        }
    }
    decls
}
/// Compare two OxiLean sources at the declaration (structural) level.
pub fn structural_diff(old_source: &str, new_source: &str) -> StructuralDiffResult {
    let old_decls = extract_decl_summaries(old_source);
    let new_decls = extract_decl_summaries(new_source);
    let mut removed = Vec::new();
    let mut added = Vec::new();
    let mut modified = Vec::new();
    let mut unchanged = Vec::new();
    let mut new_map: std::collections::HashMap<&str, &DeclSummary> =
        std::collections::HashMap::new();
    for d in &new_decls {
        new_map.insert(&d.name, d);
    }
    let mut seen_in_new: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for old_d in &old_decls {
        if let Some(new_d) = new_map.get(old_d.name.as_str()) {
            seen_in_new.insert(&new_d.name);
            if old_d == *new_d {
                unchanged.push(old_d.clone());
            } else {
                modified.push((old_d.clone(), (*new_d).clone()));
            }
        } else {
            removed.push(old_d.clone());
        }
    }
    for new_d in &new_decls {
        if !seen_in_new.contains(new_d.name.as_str()) {
            let matched_old = old_decls.iter().any(|od| od.name == new_d.name);
            if !matched_old {
                added.push(new_d.clone());
            }
        }
    }
    StructuralDiffResult {
        removed,
        added,
        modified,
        unchanged,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_diff_identical() {
        let text = "line 1\nline 2\nline 3\n";
        let config = DiffConfig::new();
        let diff = line_diff(text, text, &config);
        assert!(diff.is_empty());
        assert_eq!(diff.additions, 0);
        assert_eq!(diff.deletions, 0);
    }
    #[test]
    fn test_diff_simple_addition() {
        let old = "line 1\nline 3\n";
        let new = "line 1\nline 2\nline 3\n";
        let config = DiffConfig::new();
        let diff = line_diff(old, new, &config);
        assert_eq!(diff.additions, 1);
        assert_eq!(diff.deletions, 0);
        assert!(!diff.is_empty());
    }
    #[test]
    fn test_diff_simple_deletion() {
        let old = "line 1\nline 2\nline 3\n";
        let new = "line 1\nline 3\n";
        let config = DiffConfig::new();
        let diff = line_diff(old, new, &config);
        assert_eq!(diff.additions, 0);
        assert_eq!(diff.deletions, 1);
    }
    #[test]
    fn test_diff_modification() {
        let old = "line 1\nold line\nline 3\n";
        let new = "line 1\nnew line\nline 3\n";
        let config = DiffConfig::new();
        let diff = line_diff(old, new, &config);
        assert_eq!(diff.additions, 1);
        assert_eq!(diff.deletions, 1);
        assert_eq!(diff.total_changes(), 2);
    }
    #[test]
    fn test_diff_ignore_whitespace() {
        let old = "  hello   world  \n";
        let new = "hello world\n";
        let config = DiffConfig::new().with_ignore_whitespace(true);
        let diff = line_diff(old, new, &config);
        assert!(diff.is_empty());
    }
    #[test]
    fn test_diff_whitespace_sensitive() {
        let old = "  hello   world  \n";
        let new = "hello world\n";
        let config = DiffConfig::new().with_ignore_whitespace(false);
        let diff = line_diff(old, new, &config);
        assert!(!diff.is_empty());
    }
    #[test]
    fn test_format_diff_no_color() {
        let old = "aaa\nbbb\n";
        let new = "aaa\nccc\n";
        let config = DiffConfig::new().with_color(false);
        let diff = line_diff(old, new, &config);
        let formatted = format_diff(&diff, &config);
        assert!(formatted.contains("---"));
        assert!(formatted.contains("+++"));
        assert!(formatted.contains("@@"));
        assert!(formatted.contains("-bbb"));
        assert!(formatted.contains("+ccc"));
    }
    #[test]
    fn test_format_diff_with_color() {
        let old = "aaa\nbbb\n";
        let new = "aaa\nccc\n";
        let config = DiffConfig::new().with_color(true);
        let diff = line_diff(old, new, &config);
        let formatted = format_diff(&diff, &config);
        assert!(formatted.contains("\x1b[31m"));
        assert!(formatted.contains("\x1b[32m"));
        assert!(formatted.contains("\x1b[0m"));
    }
    #[test]
    fn test_hunk_header() {
        let hunk = DiffHunk {
            old_start: 1,
            old_count: 3,
            new_start: 1,
            new_count: 4,
            lines: Vec::new(),
            header: DiffHunk::make_header(1, 3, 1, 4),
        };
        assert_eq!(hunk.header, "@@ -1,3 +1,4 @@");
    }
    #[test]
    fn test_line_change_prefix() {
        let removed = LineChange::new(ChangeKind::Removed, "x".to_string(), 1, 0);
        let added = LineChange::new(ChangeKind::Added, "y".to_string(), 0, 1);
        let unchanged = LineChange::new(ChangeKind::Unchanged, "z".to_string(), 1, 1);
        assert_eq!(removed.prefix_char(), '-');
        assert_eq!(added.prefix_char(), '+');
        assert_eq!(unchanged.prefix_char(), ' ');
    }
    #[test]
    fn test_extract_decl_summaries() {
        let source = "def foo : Nat := 42\ntheorem bar : True := sorry\naxiom baz : Prop\n-- comment\nlemma qux : False := sorry\n";
        let decls = extract_decl_summaries(source);
        assert_eq!(decls.len(), 4);
        assert_eq!(decls[0].kind, "def");
        assert_eq!(decls[0].name, "foo");
        assert_eq!(decls[0].type_sig, "Nat");
        assert_eq!(decls[1].kind, "theorem");
        assert_eq!(decls[1].name, "bar");
        assert_eq!(decls[2].kind, "axiom");
        assert_eq!(decls[2].name, "baz");
        assert_eq!(decls[3].kind, "lemma");
        assert_eq!(decls[3].name, "qux");
    }
    #[test]
    fn test_structural_diff_no_changes() {
        let source = "def foo : Nat := 42\ntheorem bar : True := sorry\n";
        let result = structural_diff(source, source);
        assert!(result.is_empty());
        assert_eq!(result.unchanged.len(), 2);
    }
    #[test]
    fn test_structural_diff_added_removed() {
        let old = "def foo : Nat := 42\n";
        let new = "def bar : Nat := 0\n";
        let result = structural_diff(old, new);
        assert_eq!(result.removed.len(), 1);
        assert_eq!(result.removed[0].name, "foo");
        assert_eq!(result.added.len(), 1);
        assert_eq!(result.added[0].name, "bar");
    }
    #[test]
    fn test_structural_diff_modified() {
        let old = "def foo : Nat := 42\n";
        let new = "def foo : Int := 42\n";
        let result = structural_diff(old, new);
        assert_eq!(result.modified.len(), 1);
        assert_eq!(result.modified[0].0.type_sig, "Nat");
        assert_eq!(result.modified[0].1.type_sig, "Int");
    }
    #[test]
    fn test_structural_diff_summary() {
        let old = "def foo : Nat := 0\ndef removed_fn : Bool := true\n";
        let new = "def foo : Int := 0\ndef added_fn : String := \"\"\n";
        let result = structural_diff(old, new);
        let summary = result.summary();
        assert!(summary.contains("Removed"));
        assert!(summary.contains("Added"));
        assert!(summary.contains("Modified"));
        assert!(summary.contains("removed_fn"));
        assert!(summary.contains("added_fn"));
        assert!(summary.contains("foo"));
    }
    #[test]
    fn test_diff_empty_inputs() {
        let config = DiffConfig::new();
        let diff = line_diff("", "", &config);
        assert!(diff.is_empty());
    }
    #[test]
    fn test_diff_one_empty() {
        let config = DiffConfig::new();
        let diff = line_diff("", "hello\n", &config);
        assert_eq!(diff.additions, 1);
        assert_eq!(diff.deletions, 0);
    }
    #[test]
    fn test_diff_config_builder() {
        let config = DiffConfig::new()
            .with_context(5)
            .with_ignore_whitespace(true)
            .with_color(true)
            .with_labels("old.oxilean", "new.oxilean");
        assert_eq!(config.context_lines, 5);
        assert!(config.ignore_whitespace);
        assert!(config.color_output);
        assert_eq!(config.old_label, "old.oxilean");
        assert_eq!(config.new_label, "new.oxilean");
    }
    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(normalize_whitespace("  a   b  c  "), "a b c");
        assert_eq!(normalize_whitespace("hello"), "hello");
        assert_eq!(normalize_whitespace("  "), "");
    }
    #[test]
    fn test_diff_result_display() {
        let old = "aaa\nbbb\n";
        let new = "aaa\nccc\n";
        let config = DiffConfig::new();
        let diff = line_diff(old, new, &config);
        let display = format!("{diff}");
        assert!(display.contains("-bbb"));
        assert!(display.contains("+ccc"));
    }
    #[test]
    fn test_large_diff() {
        let old: String = (0..100).map(|i| format!("line {i}\n")).collect();
        let new: String = (0..100)
            .map(|i| {
                if i == 50 {
                    "CHANGED\n".to_string()
                } else {
                    format!("line {i}\n")
                }
            })
            .collect();
        let config = DiffConfig::new().with_context(2);
        let diff = line_diff(&old, &new, &config);
        assert_eq!(diff.additions, 1);
        assert_eq!(diff.deletions, 1);
        assert!(diff.hunks.len() == 1);
        assert!(diff.hunks[0].lines.len() <= 7);
    }
}
#[allow(dead_code)]
pub fn split_into_words(s: &str) -> Vec<String> {
    let mut words = Vec::new();
    let mut current = String::new();
    for ch in s.chars() {
        if ch.is_whitespace() {
            if !current.is_empty() {
                words.push(current.clone());
                current.clear();
            }
            words.push(ch.to_string());
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        words.push(current);
    }
    words
}
#[allow(dead_code)]
pub fn lcs_of_words(a: &[String], b: &[String]) -> Vec<String> {
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    let mut lcs = Vec::new();
    let (mut i, mut j) = (m, n);
    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            lcs.push(a[i - 1].clone());
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    lcs.reverse();
    lcs
}
#[allow(dead_code)]
pub fn word_diff(old: &str, new: &str) -> Vec<WordToken> {
    let old_w = split_into_words(old);
    let new_w = split_into_words(new);
    let lcs = lcs_of_words(&old_w, &new_w);
    let mut result = Vec::new();
    let (mut li, mut oi, mut ni) = (0, 0, 0);
    while oi < old_w.len() || ni < new_w.len() {
        let lcs_word = lcs.get(li);
        match lcs_word {
            Some(lw) => {
                if oi < old_w.len() && &old_w[oi] == lw && ni < new_w.len() && &new_w[ni] == lw {
                    result.push(WordToken::Equal(lw.clone()));
                    li += 1;
                    oi += 1;
                    ni += 1;
                } else if oi < old_w.len() && &old_w[oi] != lw {
                    result.push(WordToken::Removed(old_w[oi].clone()));
                    oi += 1;
                } else if ni < new_w.len() {
                    result.push(WordToken::Added(new_w[ni].clone()));
                    ni += 1;
                }
            }
            None => {
                while oi < old_w.len() {
                    result.push(WordToken::Removed(old_w[oi].clone()));
                    oi += 1;
                }
                while ni < new_w.len() {
                    result.push(WordToken::Added(new_w[ni].clone()));
                    ni += 1;
                }
            }
        }
    }
    result
}
#[allow(dead_code)]
pub fn render_word_diff(tokens: &[WordToken]) -> String {
    let mut out = String::new();
    for tok in tokens {
        match tok {
            WordToken::Equal(s) => out.push_str(s),
            WordToken::Added(s) => out.push_str(&format!("[+{}]", s)),
            WordToken::Removed(s) => out.push_str(&format!("[-{}]", s)),
        }
    }
    out
}
#[allow(dead_code)]
pub fn char_diff(old: &str, new: &str) -> Vec<CharDiffToken> {
    let old_c: Vec<char> = old.chars().collect();
    let new_c: Vec<char> = new.chars().collect();
    let m = old_c.len();
    let n = new_c.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 1..=m {
        for j in 1..=n {
            if old_c[i - 1] == new_c[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }
    let mut result = Vec::new();
    let (mut i, mut j) = (m, n);
    while i > 0 || j > 0 {
        if i > 0 && j > 0 && old_c[i - 1] == new_c[j - 1] {
            result.push(CharDiffToken::Equal(old_c[i - 1]));
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || dp[i][j - 1] >= dp[i - 1][j]) {
            result.push(CharDiffToken::Added(new_c[j - 1]));
            j -= 1;
        } else {
            result.push(CharDiffToken::Removed(old_c[i - 1]));
            i -= 1;
        }
    }
    result.reverse();
    result
}
#[allow(dead_code)]
pub fn render_char_diff(tokens: &[CharDiffToken]) -> String {
    let mut out = String::new();
    for tok in tokens {
        match tok {
            CharDiffToken::Equal(c) => out.push(*c),
            CharDiffToken::Added(c) => {
                out.push('+');
                out.push(*c);
            }
            CharDiffToken::Removed(c) => {
                out.push('-');
                out.push(*c);
            }
        }
    }
    out
}
#[allow(dead_code)]
pub fn extract_decl_items(src: &str) -> Vec<DeclItem> {
    let mut items = Vec::new();
    for (line_idx, line) in src.lines().enumerate() {
        let trimmed = line.trim();
        let (kind, rest) = if let Some(r) = trimmed.strip_prefix("theorem ") {
            (DeclItemKind::Theorem, r)
        } else if let Some(r) = trimmed.strip_prefix("lemma ") {
            (DeclItemKind::Lemma, r)
        } else if let Some(r) = trimmed.strip_prefix("def ") {
            (DeclItemKind::Def, r)
        } else if let Some(r) = trimmed.strip_prefix("axiom ") {
            (DeclItemKind::Axiom, r)
        } else if let Some(r) = trimmed.strip_prefix("structure ") {
            (DeclItemKind::Structure, r)
        } else if let Some(r) = trimmed.strip_prefix("class ") {
            (DeclItemKind::Class, r)
        } else if let Some(r) = trimmed.strip_prefix("instance ") {
            (DeclItemKind::Instance, r)
        } else {
            continue;
        };
        let name = rest
            .split_whitespace()
            .next()
            .unwrap_or("")
            .split(':')
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        let body_hash = {
            let mut h: u64 = 14695981039346656037;
            for b in trimmed.as_bytes() {
                h = h.wrapping_mul(1099511628211);
                h ^= *b as u64;
            }
            h
        };
        items.push(DeclItem {
            name,
            kind,
            line: line_idx,
            body_hash,
        });
    }
    items
}
#[allow(dead_code)]
pub fn struct_diff_items(old_src: &str, new_src: &str) -> StructDiff {
    let old_items = extract_decl_items(old_src);
    let new_items = extract_decl_items(new_src);
    let mut added = Vec::new();
    let mut removed = Vec::new();
    let mut changed = Vec::new();
    let mut unchanged = Vec::new();
    for new_item in &new_items {
        if let Some(old_item) = old_items.iter().find(|i| i.name == new_item.name) {
            if old_item.body_hash == new_item.body_hash {
                unchanged.push(new_item.clone());
            } else {
                changed.push((old_item.clone(), new_item.clone()));
            }
        } else {
            added.push(new_item.clone());
        }
    }
    for old_item in &old_items {
        if !new_items.iter().any(|i| i.name == old_item.name) {
            removed.push(old_item.clone());
        }
    }
    StructDiff {
        added,
        removed,
        changed,
        unchanged,
    }
}
#[allow(dead_code)]
pub fn side_by_side_render(old_lines: &[&str], new_lines: &[&str], width: usize) -> String {
    let half = width / 2 - 3;
    let mut out = String::new();
    let max_lines = old_lines.len().max(new_lines.len());
    let header = format!("{:<width$} | {:<width$}\n", "OLD", "NEW", width = half);
    out.push_str(&header);
    out.push_str(&"-".repeat(width));
    out.push('\n');
    for i in 0..max_lines {
        let left = old_lines.get(i).copied().unwrap_or("");
        let right = new_lines.get(i).copied().unwrap_or("");
        let left_trunc = if left.len() > half {
            &left[..half]
        } else {
            left
        };
        let right_trunc = if right.len() > half {
            &right[..half]
        } else {
            right
        };
        out.push_str(&format!(
            "{:<width$} | {}\n",
            left_trunc,
            right_trunc,
            width = half
        ));
    }
    out
}
#[allow(dead_code)]
pub fn find_conflict_markers(src: &str) -> Vec<usize> {
    src.lines()
        .enumerate()
        .filter(|(_, l)| {
            l.starts_with("<<<<<<<") || l.starts_with("=======") || l.starts_with(">>>>>>>")
        })
        .map(|(i, _)| i)
        .collect()
}
#[allow(dead_code)]
pub fn has_conflicts(src: &str) -> bool {
    !find_conflict_markers(src).is_empty()
}
#[allow(dead_code)]
pub trait DiffFilter {
    fn filter_lines<'a>(&self, lines: Vec<&'a str>) -> Vec<&'a str>;
}
#[allow(dead_code)]
pub fn normalize_line_endings(s: &str) -> String {
    s.replace("\r\n", "\n").replace('\r', "\n")
}
#[allow(dead_code)]
pub fn normalized_diff(old: &str, new: &str) -> DiffResult {
    let old_n = normalize_line_endings(old);
    let new_n = normalize_line_endings(new);
    let config = DiffConfig::new();
    line_diff(&old_n, &new_n, &config)
}
#[allow(dead_code)]
pub fn reverse_hunk(hunk: &DiffHunk) -> DiffHunk {
    let mut lines = Vec::new();
    for line in &hunk.lines {
        let new_kind = match &line.kind {
            DiffLineKind::Added => DiffLineKind::Removed,
            DiffLineKind::Removed => DiffLineKind::Added,
            k => k.clone(),
        };
        lines.push(DiffLine {
            kind: new_kind,
            content: line.content.clone(),
            old_lineno: line.new_lineno,
            new_lineno: line.old_lineno,
        });
    }
    DiffHunk {
        old_start: hunk.new_start,
        old_count: hunk.new_count,
        new_start: hunk.old_start,
        new_count: hunk.old_count,
        lines,
        header: format!("reversed: {}", hunk.header),
    }
}
#[allow(dead_code)]
pub fn hunk_has_only_whitespace_changes(hunk: &DiffHunk) -> bool {
    hunk.lines
        .iter()
        .filter(|l| l.kind != DiffLineKind::Context)
        .all(|l| l.content.trim().is_empty())
}
#[allow(dead_code)]
pub fn count_changed_lines_in_hunk(hunk: &DiffHunk) -> (usize, usize) {
    let adds = hunk
        .lines
        .iter()
        .filter(|l| l.kind == DiffLineKind::Added)
        .count();
    let rems = hunk
        .lines
        .iter()
        .filter(|l| l.kind == DiffLineKind::Removed)
        .count();
    (adds, rems)
}
#[allow(dead_code)]
pub fn inline_char_diff_line(old_line: &str, new_line: &str) -> String {
    let tokens = char_diff(old_line, new_line);
    render_char_diff(&tokens)
}
#[allow(dead_code)]
pub fn oxi_tokenize_line(line: &str) -> Vec<OxiDiffToken> {
    let keywords = [
        "theorem", "def", "lemma", "axiom", "fun", "match", "forall", "exists", "let", "in", "by",
        "have", "show",
    ];
    let mut tokens = Vec::new();
    let mut rest = line;
    while !rest.is_empty() {
        if rest.starts_with(|c: char| c.is_whitespace()) {
            let end = rest
                .find(|c: char| !c.is_whitespace())
                .unwrap_or(rest.len());
            tokens.push(OxiDiffToken::Whitespace(rest[..end].to_string()));
            rest = &rest[end..];
        } else if rest.starts_with(|c: char| c.is_alphabetic() || c == '_') {
            let end = rest
                .find(|c: char| !c.is_alphanumeric() && c != '_')
                .unwrap_or(rest.len());
            let word = &rest[..end];
            if keywords.contains(&word) {
                tokens.push(OxiDiffToken::Keyword(word.to_string()));
            } else {
                tokens.push(OxiDiffToken::Ident(word.to_string()));
            }
            rest = &rest[end..];
        } else if rest.starts_with(|c: char| c.is_numeric()) {
            let end = rest.find(|c: char| !c.is_numeric()).unwrap_or(rest.len());
            tokens.push(OxiDiffToken::Number(rest[..end].to_string()));
            rest = &rest[end..];
        } else {
            let ch = rest
                .chars()
                .next()
                .expect("rest is non-empty: loop condition ensures !rest.is_empty()");
            let len = ch.len_utf8();
            tokens.push(OxiDiffToken::Punct(ch.to_string()));
            rest = &rest[len..];
        }
    }
    tokens
}
#[allow(dead_code)]
pub fn blame_view(src: &str, commit: &str, author: &str) -> BlameSource {
    let lines = src
        .lines()
        .enumerate()
        .map(|(i, l)| BlameLine {
            line_no: i + 1,
            content: l.to_string(),
            commit: commit.to_string(),
            author: author.to_string(),
        })
        .collect();
    BlameSource {
        file: "unknown".to_string(),
        lines,
    }
}
#[allow(dead_code)]
pub fn diff_to_csv(result: &DiffResult) -> String {
    let mut out = String::from("hunk,line_no,kind,content\n");
    for (hi, hunk) in result.hunks.iter().enumerate() {
        for line in &hunk.lines {
            let kind = match &line.kind {
                DiffLineKind::Added => "added",
                DiffLineKind::Removed => "removed",
                DiffLineKind::Context => "context",
            };
            let content = line.content.replace(',', ";");
            out.push_str(&format!(
                "{},{:?},{},{}\n",
                hi, line.old_lineno, kind, content
            ));
        }
    }
    out
}
#[allow(dead_code)]
pub fn line_similarity(a: &str, b: &str) -> f64 {
    if a == b {
        return 1.0;
    }
    if a.is_empty() && b.is_empty() {
        return 1.0;
    }
    let la = a.len();
    let lb = b.len();
    if la == 0 || lb == 0 {
        return 0.0;
    }
    let lcs_len = char_diff(a, b)
        .iter()
        .filter(|t| matches!(t, CharDiffToken::Equal(_)))
        .count();
    2.0 * lcs_len as f64 / (la + lb) as f64
}
#[allow(dead_code)]
pub mod diff_colors {
    pub const ADDED: &str = "\x1b[32m";
    pub const REMOVED: &str = "\x1b[31m";
    pub const CONTEXT: &str = "\x1b[0m";
    pub const HEADER: &str = "\x1b[36m";
    pub const RESET: &str = "\x1b[0m";
}
#[allow(dead_code)]
pub fn render_colored_diff(result: &DiffResult) -> String {
    let mut out = String::new();
    for hunk in &result.hunks {
        out.push_str(&format!(
            "{}@@ {} @@{}\n",
            diff_colors::HEADER,
            hunk.header,
            diff_colors::RESET
        ));
        for line in &hunk.lines {
            let (prefix, color) = match &line.kind {
                DiffLineKind::Added => ('+', diff_colors::ADDED),
                DiffLineKind::Removed => ('-', diff_colors::REMOVED),
                DiffLineKind::Context => (' ', diff_colors::CONTEXT),
            };
            out.push_str(&format!(
                "{}{}{}{}\n",
                color,
                prefix,
                line.content,
                diff_colors::RESET
            ));
        }
    }
    out
}
#[cfg(test)]
mod diff_extended_tests {
    use super::*;
    #[test]
    fn test_split_into_words() {
        let words = split_into_words("hello world");
        assert!(words.contains(&"hello".to_string()));
        assert!(words.contains(&"world".to_string()));
    }
    #[test]
    fn test_word_diff_equal() {
        let tokens = word_diff("hello world", "hello world");
        assert!(tokens.iter().all(|t| matches!(t, WordToken::Equal(_))));
    }
    #[test]
    fn test_word_diff_added() {
        let tokens = word_diff("hello", "hello world");
        assert!(tokens.iter().any(|t| matches!(t, WordToken::Added(_))));
    }
    #[test]
    fn test_char_diff_equal() {
        let tokens = char_diff("abc", "abc");
        assert!(tokens.iter().all(|t| matches!(t, CharDiffToken::Equal(_))));
    }
    #[test]
    fn test_char_diff_added() {
        let tokens = char_diff("ab", "abc");
        assert!(tokens.iter().any(|t| matches!(t, CharDiffToken::Added(_))));
    }
    #[test]
    fn test_extract_decl_items() {
        let src = "theorem foo : P := sorry\ndef bar : Nat := 0\n";
        let items = extract_decl_items(src);
        assert_eq!(items.len(), 2);
        assert!(items
            .iter()
            .any(|i| i.name == "foo" && i.kind == DeclItemKind::Theorem));
        assert!(items
            .iter()
            .any(|i| i.name == "bar" && i.kind == DeclItemKind::Def));
    }
    #[test]
    fn test_structural_diff_added() {
        let old = "theorem foo : P := sorry\n";
        let new = "theorem foo : P := sorry\ndef bar : Nat := 0\n";
        let sd = structural_diff(old, new);
        assert_eq!(sd.added.len(), 1);
        assert_eq!(sd.added[0].name, "bar");
    }
    #[test]
    fn test_file_patch_net_change() {
        let hunk = FilePatchHunk {
            old_start: 1,
            old_count: 2,
            new_start: 1,
            new_count: 3,
            lines: vec![
                FilePatchLine {
                    kind: FilePatchLineKind::Context,
                    content: "ctx".to_string(),
                },
                FilePatchLine {
                    kind: FilePatchLineKind::Removed,
                    content: "old".to_string(),
                },
                FilePatchLine {
                    kind: FilePatchLineKind::Added,
                    content: "new1".to_string(),
                },
                FilePatchLine {
                    kind: FilePatchLineKind::Added,
                    content: "new2".to_string(),
                },
            ],
        };
        let patch = FilePatch {
            old_file: "a.lean".to_string(),
            new_file: "b.lean".to_string(),
            hunks: vec![hunk],
        };
        assert_eq!(patch.net_change(), 1);
    }
    #[test]
    fn test_has_conflicts_true() {
        let src = "<<<<<<< HEAD\nfoo\n=======\nbar\n>>>>>>> branch\n";
        assert!(has_conflicts(src));
    }
    #[test]
    fn test_has_conflicts_false() {
        let src = "theorem foo : P := sorry\n";
        assert!(!has_conflicts(src));
    }
    #[test]
    fn test_side_by_side_render() {
        let old = vec!["line 1", "line 2"];
        let new = vec!["line 1", "modified"];
        let render = side_by_side_render(&old, &new, 80);
        assert!(render.contains("OLD"));
        assert!(render.contains("NEW"));
        assert!(render.contains("line 1"));
    }
    #[test]
    fn test_line_similarity_equal() {
        assert!((line_similarity("hello", "hello") - 1.0).abs() < 1e-6);
    }
    #[test]
    fn test_line_similarity_different() {
        let s = line_similarity("hello", "world");
        assert!(s < 1.0 && s >= 0.0);
    }
    #[test]
    fn test_diff_to_csv() {
        let config = DiffConfig::new();
        let result = line_diff("a\nb\n", "a\nc\n", &config);
        let csv = diff_to_csv(&result);
        assert!(csv.contains("hunk"));
    }
    #[test]
    fn test_normalize_line_endings() {
        let s = "a\r\nb\rc\n";
        let norm = normalize_line_endings(s);
        assert!(!norm.contains('\r'));
    }
    #[test]
    fn test_oxi_tokenize_keyword() {
        let tokens = oxi_tokenize_line("theorem foo");
        assert!(tokens
            .iter()
            .any(|t| matches!(t, OxiDiffToken::Keyword(k) if k ==
            "theorem")));
    }
    #[test]
    fn test_diff_statistics() {
        let stats = DiffStatistics {
            total_lines: 100,
            added_lines: 5,
            removed_lines: 5,
            context_lines: 90,
            hunks: 1,
            files_changed: 1,
        };
        let sim = stats.similarity_percent();
        assert!(sim > 0.0 && sim <= 100.0);
    }
    #[test]
    fn test_blame_view() {
        let src = "foo\nbar\n";
        let bv = blame_view(src, "abc123", "Author");
        assert_eq!(bv.lines.len(), 2);
        assert_eq!(bv.lines[0].author, "Author");
    }
    #[test]
    fn test_keyword_filter() {
        let f = KeywordFilter::new("theorem");
        let lines = vec!["theorem foo : P", "def bar : Nat", "theorem baz : Q"];
        let filtered = f.filter_lines(lines);
        assert_eq!(filtered.len(), 2);
    }
    #[test]
    fn test_render_word_diff() {
        let tokens = vec![
            WordToken::Equal("hello".to_string()),
            WordToken::Removed("world".to_string()),
            WordToken::Added("universe".to_string()),
        ];
        let rendered = render_word_diff(&tokens);
        assert!(rendered.contains("hello"));
        assert!(rendered.contains("[-world]"));
        assert!(rendered.contains("[+universe]"));
    }
    #[test]
    fn test_hunk_whitespace_only() {
        let hunk = DiffHunk {
            old_start: 1,
            old_count: 1,
            new_start: 1,
            new_count: 1,
            lines: vec![DiffLine {
                kind: DiffLineKind::Added,
                content: "   ".to_string(),
                old_lineno: None,
                new_lineno: Some(1),
            }],
            header: "".to_string(),
        };
        assert!(hunk_has_only_whitespace_changes(&hunk));
    }
    #[test]
    fn test_count_changed_lines_in_hunk() {
        let hunk = DiffHunk {
            old_start: 1,
            old_count: 2,
            new_start: 1,
            new_count: 2,
            lines: vec![
                DiffLine {
                    kind: DiffLineKind::Added,
                    content: "a".to_string(),
                    old_lineno: None,
                    new_lineno: Some(1),
                },
                DiffLine {
                    kind: DiffLineKind::Removed,
                    content: "b".to_string(),
                    old_lineno: Some(1),
                    new_lineno: None,
                },
                DiffLine {
                    kind: DiffLineKind::Removed,
                    content: "c".to_string(),
                    old_lineno: Some(2),
                    new_lineno: None,
                },
            ],
            header: "".to_string(),
        };
        let (adds, rems) = count_changed_lines_in_hunk(&hunk);
        assert_eq!(adds, 1);
        assert_eq!(rems, 2);
    }
}
#[allow(dead_code)]
pub fn annotate_all_lines(result: &DiffResult, src: &str) -> Vec<AnnotatedDiffLine> {
    let mut annotated = Vec::new();
    let src_lines: Vec<&str> = src.lines().collect();
    for hunk in &result.hunks {
        for line in &hunk.lines {
            let line_idx = line
                .old_lineno
                .or(line.new_lineno)
                .map(|n| n.saturating_sub(1))
                .unwrap_or(0);
            let func_ctx = src_lines.get(line_idx).and_then(|_| {
                src_lines[..line_idx]
                    .iter()
                    .rev()
                    .find(|l| {
                        l.trim_start().starts_with("theorem ") || l.trim_start().starts_with("def ")
                    })
                    .map(|l| l.trim().to_string())
            });
            let is_proof = line.content.contains("sorry")
                || line.content.contains("by")
                || line.content.contains("exact");
            let indent = line.content.len() - line.content.trim_start().len();
            annotated.push(AnnotatedDiffLine {
                line: line.clone(),
                function_context: func_ctx,
                is_in_proof: is_proof,
                indent_level: indent,
            });
        }
    }
    annotated
}
#[allow(dead_code)]
pub fn pair_changed_lines(hunk: &DiffHunk) -> Vec<(Option<&DiffLine>, Option<&DiffLine>)> {
    let removed: Vec<&DiffLine> = hunk
        .lines
        .iter()
        .filter(|l| l.kind == DiffLineKind::Removed)
        .collect();
    let added: Vec<&DiffLine> = hunk
        .lines
        .iter()
        .filter(|l| l.kind == DiffLineKind::Added)
        .collect();
    let max_len = removed.len().max(added.len());
    (0..max_len)
        .map(|i| (removed.get(i).copied(), added.get(i).copied()))
        .collect()
}
#[allow(dead_code)]
pub fn compact_summary(result: &DiffResult) -> String {
    format!(
        "+{} -{} lines in {} hunk(s)",
        result.additions,
        result.deletions,
        result.hunks.len()
    )
}
#[allow(dead_code)]
pub fn diff_to_html(result: &DiffResult) -> String {
    let mut out = String::from("<div class=\"diff\">\n");
    for hunk in &result.hunks {
        out.push_str("<div class=\"hunk\">\n");
        out.push_str(&format!(
            "  <div class=\"hunk-header\">{}</div>\n",
            hunk.header
        ));
        for line in &hunk.lines {
            let (cls, prefix) = match &line.kind {
                DiffLineKind::Added => ("added", "+"),
                DiffLineKind::Removed => ("removed", "-"),
                DiffLineKind::Context => ("context", " "),
            };
            let escaped = line.content.replace('<', "&lt;").replace('>', "&gt;");
            out.push_str(&format!(
                "  <div class=\"diff-line {}\"><span class=\"prefix\">{}</span>{}</div>\n",
                cls, prefix, escaped
            ));
        }
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n");
    out
}
#[cfg(test)]
mod diff_multi_tests {
    use super::*;
    #[test]
    fn test_multi_diff_summary() {
        let mut md = MultiDiff::new();
        md.add("a.lean", "foo\n", "foo\nbar\n");
        md.add("b.lean", "x\ny\n", "x\nz\n");
        let s = md.summary();
        assert!(s.contains("2 files"));
        assert_eq!(md.total_additions(), 2);
        assert_eq!(md.total_deletions(), 1);
    }
    #[test]
    fn test_multi_diff_changed_files() {
        let mut md = MultiDiff::new();
        md.add("unchanged.lean", "a\n", "a\n");
        md.add("changed.lean", "a\n", "b\n");
        let changed = md.changed_files();
        assert_eq!(changed.len(), 1);
        assert_eq!(changed[0], "changed.lean");
    }
    #[test]
    fn test_compact_summary() {
        let config = DiffConfig::new();
        let result = line_diff("a\nb\n", "a\nc\n", &config);
        let s = compact_summary(&result);
        assert!(s.contains("+1"));
        assert!(s.contains("-1"));
    }
    #[test]
    fn test_diff_to_html() {
        let config = DiffConfig::new();
        let result = line_diff("old\n", "new\n", &config);
        let html = diff_to_html(&result);
        assert!(html.contains("<div class=\"diff\">"));
        assert!(html.contains("removed") || html.contains("added"));
    }
    #[test]
    fn test_annotate_all_lines_runs() {
        let src = "theorem foo : P := sorry\n";
        let config = DiffConfig::new();
        let result = line_diff(src, src, &config);
        let annotated = annotate_all_lines(&result, src);
        assert!(annotated.len() <= result.hunks.iter().map(|h| h.lines.len()).sum::<usize>());
    }
    #[test]
    fn test_reverse_hunk_swaps_kinds() {
        let hunk = DiffHunk {
            old_start: 1,
            old_count: 1,
            new_start: 1,
            new_count: 1,
            lines: vec![DiffLine {
                kind: DiffLineKind::Added,
                content: "x".to_string(),
                old_lineno: None,
                new_lineno: Some(1),
            }],
            header: "".to_string(),
        };
        let rev = reverse_hunk(&hunk);
        assert_eq!(rev.lines[0].kind, DiffLineKind::Removed);
    }
    #[test]
    fn test_pair_changed_lines() {
        let hunk = DiffHunk {
            old_start: 1,
            old_count: 1,
            new_start: 1,
            new_count: 1,
            lines: vec![
                DiffLine {
                    kind: DiffLineKind::Removed,
                    content: "old".to_string(),
                    old_lineno: Some(1),
                    new_lineno: None,
                },
                DiffLine {
                    kind: DiffLineKind::Added,
                    content: "new".to_string(),
                    old_lineno: None,
                    new_lineno: Some(1),
                },
            ],
            header: "".to_string(),
        };
        let pairs = pair_changed_lines(&hunk);
        assert_eq!(pairs.len(), 1);
        assert!(pairs[0].0.is_some() && pairs[0].1.is_some());
    }
    #[test]
    fn test_word_diff_removed() {
        let tokens = word_diff("hello world", "hello");
        assert!(tokens.iter().any(|t| matches!(t, WordToken::Removed(_))));
    }
    #[test]
    fn test_lcs_of_words() {
        let a = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let b = vec!["a".to_string(), "c".to_string()];
        let lcs = lcs_of_words(&a, &b);
        assert_eq!(lcs, vec!["a".to_string(), "c".to_string()]);
    }
}
#[allow(dead_code)]
pub fn sort_hunks_by_impact(result: &mut DiffResult) {
    result.hunks.sort_by(|a, b| {
        let impact_a = a
            .lines
            .iter()
            .filter(|l| l.kind != DiffLineKind::Context)
            .count();
        let impact_b = b
            .lines
            .iter()
            .filter(|l| l.kind != DiffLineKind::Context)
            .count();
        impact_b.cmp(&impact_a)
    });
}
#[allow(dead_code)]
pub fn hash_str(s: &str) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for b in s.as_bytes() {
        h = h.wrapping_mul(1099511628211);
        h ^= *b as u64;
    }
    h
}
#[allow(dead_code)]
pub fn classify_change(old_line: &str, new_line: &str) -> ChangeClass {
    let old_t = old_line.trim();
    let new_t = new_line.trim();
    if old_t.trim() == new_t.trim() {
        return ChangeClass::WhitespaceChange;
    }
    if old_t.starts_with("--") || new_t.starts_with("--") {
        return ChangeClass::CommentChange;
    }
    if old_t.starts_with("import") || new_t.starts_with("import") {
        return ChangeClass::ImportChange;
    }
    if old_t.contains("sorry")
        || new_t.contains("sorry")
        || old_t.contains("exact")
        || new_t.contains("exact")
    {
        return ChangeClass::ProofChange;
    }
    if old_t.starts_with("theorem")
        || new_t.starts_with("theorem")
        || old_t.starts_with("def")
        || new_t.starts_with("def")
    {
        return ChangeClass::StructureChange;
    }
    if old_t.contains(":") || new_t.contains(":") {
        return ChangeClass::TypeChange;
    }
    ChangeClass::Other
}
#[allow(dead_code)]
pub fn diff_to_json(result: &DiffResult) -> String {
    let hunks: Vec<String> = result
        .hunks
        .iter()
        .map(|h| {
            let lines: Vec<String> = h
                .lines
                .iter()
                .map(|l| {
                    let kind = match &l.kind {
                        DiffLineKind::Added => "added",
                        DiffLineKind::Removed => "removed",
                        DiffLineKind::Context => "context",
                    };
                    format!(
                        "{{\"kind\":\"{}\",\"content\":\"{}\"}}",
                        kind,
                        l.content.replace('"', "'")
                    )
                })
                .collect();
            format!(
                "{{\"old_start\":{},\"new_start\":{},\"lines\":[{}]}}",
                h.old_start,
                h.new_start,
                lines.join(",")
            )
        })
        .collect();
    format!(
        "{{\"additions\":{},\"deletions\":{},\"hunks\":[{}]}}",
        result.additions,
        result.deletions,
        hunks.join(",")
    )
}
#[allow(dead_code)]
pub fn count_blank_line_changes(result: &DiffResult) -> (usize, usize) {
    let mut blank_adds = 0;
    let mut blank_rems = 0;
    for hunk in &result.hunks {
        for line in &hunk.lines {
            if line.content.trim().is_empty() {
                match &line.kind {
                    DiffLineKind::Added => blank_adds += 1,
                    DiffLineKind::Removed => blank_rems += 1,
                    _ => {}
                }
            }
        }
    }
    (blank_adds, blank_rems)
}
#[cfg(test)]
mod diff_final_tests {
    use super::*;
    #[test]
    fn test_diff_cache_get_or_compute() {
        let mut cache = DiffCache::new();
        let _r1 = cache.get_or_compute("a\n", "b\n");
        let _r2 = cache.get_or_compute("a\n", "b\n");
        assert_eq!(cache.size(), 1);
    }
    #[test]
    fn test_diff_cache_invalidate() {
        let mut cache = DiffCache::new();
        let _ = cache.get_or_compute("x\n", "y\n");
        cache.invalidate("x\n", "y\n");
        assert_eq!(cache.size(), 0);
    }
    #[test]
    fn test_classify_change_whitespace() {
        let c = classify_change("  foo", "    foo");
        assert_eq!(c, ChangeClass::WhitespaceChange);
    }
    #[test]
    fn test_classify_change_proof() {
        let c = classify_change("exact h", "sorry");
        assert_eq!(c, ChangeClass::ProofChange);
    }
    #[test]
    fn test_classify_change_import() {
        let c = classify_change("import Mathlib", "import Std");
        assert_eq!(c, ChangeClass::ImportChange);
    }
    #[test]
    fn test_diff_to_json() {
        let config = DiffConfig::new();
        let result = line_diff("old\n", "new\n", &config);
        let json = diff_to_json(&result);
        assert!(json.contains("additions"));
        assert!(json.contains("deletions"));
        assert!(json.contains("hunks"));
    }
    #[test]
    fn test_count_blank_line_changes() {
        let config = DiffConfig::new();
        let result = line_diff("a\n\nb\n", "a\nb\n", &config);
        let (adds, rems) = count_blank_line_changes(&result);
        assert!(adds + rems <= 2);
    }
    #[test]
    fn test_sort_hunks_by_impact() {
        let mut result = DiffResult {
            additions: 2,
            deletions: 1,
            hunks: vec![
                DiffHunk {
                    old_start: 1,
                    old_count: 1,
                    new_start: 1,
                    new_count: 1,
                    lines: vec![DiffLine {
                        kind: DiffLineKind::Added,
                        content: "a".to_string(),
                        old_lineno: None,
                        new_lineno: Some(1),
                    }],
                    header: "".to_string(),
                },
                DiffHunk {
                    old_start: 10,
                    old_count: 3,
                    new_start: 10,
                    new_count: 3,
                    lines: vec![
                        DiffLine {
                            kind: DiffLineKind::Added,
                            content: "b".to_string(),
                            old_lineno: None,
                            new_lineno: Some(10),
                        },
                        DiffLine {
                            kind: DiffLineKind::Added,
                            content: "c".to_string(),
                            old_lineno: None,
                            new_lineno: Some(11),
                        },
                        DiffLine {
                            kind: DiffLineKind::Removed,
                            content: "d".to_string(),
                            old_lineno: Some(10),
                            new_lineno: None,
                        },
                    ],
                    header: "".to_string(),
                },
            ],
        };
        sort_hunks_by_impact(&mut result);
        assert_eq!(result.hunks[0].lines.len(), 3);
    }
}
#[allow(dead_code)]
pub fn present_diff(result: &DiffResult, target: DiffDisplayTarget) -> String {
    match target {
        DiffDisplayTarget::Terminal => render_colored_diff(result),
        DiffDisplayTarget::Html => diff_to_html(result),
        DiffDisplayTarget::Json => diff_to_json(result),
        DiffDisplayTarget::Csv => diff_to_csv(result),
        DiffDisplayTarget::Compact => compact_summary(result),
    }
}
#[cfg(test)]
mod diff_window_tests {
    use super::*;
    #[test]
    fn test_diff_window_push_and_total() {
        let mut w = DiffWindow::new(5);
        let config = DiffConfig::new();
        w.push("a", line_diff("x\n", "y\n", &config));
        w.push("b", line_diff("p\n", "p\nq\n", &config));
        assert_eq!(w.len(), 2);
        assert_eq!(w.total_additions(), 2);
    }
    #[test]
    fn test_diff_window_max_size() {
        let mut w = DiffWindow::new(2);
        let config = DiffConfig::new();
        for i in 0..5 {
            w.push(&i.to_string(), line_diff("a\n", "b\n", &config));
        }
        assert_eq!(w.len(), 2);
    }
    #[test]
    fn test_diff_summary_report() {
        let mut md = MultiDiff::new();
        md.add("a.lean", "x\n", "y\n");
        md.add("b.lean", "p\n", "p\n");
        let report = DiffSummaryReport::from_multi_diff(&md);
        assert_eq!(report.files_checked, 2);
        assert_eq!(report.files_changed, 1);
        let s = report.to_string_report();
        assert!(s.contains("Files:"));
    }
    #[test]
    fn test_present_diff_compact() {
        let config = DiffConfig::new();
        let result = line_diff("old\n", "new\n", &config);
        let out = present_diff(&result, DiffDisplayTarget::Compact);
        assert!(out.contains("+") || out.contains("-"));
    }
    #[test]
    fn test_present_diff_json() {
        let config = DiffConfig::new();
        let result = line_diff("a\n", "b\n", &config);
        let json = present_diff(&result, DiffDisplayTarget::Json);
        assert!(json.starts_with("{"));
    }
}
#[allow(dead_code)]
pub fn count_line_types(result: &DiffResult) -> (usize, usize, usize) {
    let mut adds = 0;
    let mut rems = 0;
    let mut ctx = 0;
    for hunk in &result.hunks {
        for line in &hunk.lines {
            match line.kind {
                DiffLineKind::Added => adds += 1,
                DiffLineKind::Removed => rems += 1,
                DiffLineKind::Context => ctx += 1,
            }
        }
    }
    (adds, rems, ctx)
}
#[allow(dead_code)]
pub fn diff_is_empty(result: &DiffResult) -> bool {
    result.additions == 0 && result.deletions == 0
}
#[allow(dead_code)]
pub fn largest_hunk(result: &DiffResult) -> Option<&DiffHunk> {
    result.hunks.iter().max_by_key(|h| h.lines.len())
}
#[allow(dead_code)]
pub fn diff_total_lines(result: &DiffResult) -> usize {
    result.hunks.iter().map(|h| h.lines.len()).sum()
}
#[cfg(test)]
mod diff_counter_tests {
    use super::*;
    #[test]
    fn test_count_line_types() {
        let config = DiffConfig::new();
        let result = line_diff("a\nb\n", "a\nc\n", &config);
        let (adds, rems, _ctx) = count_line_types(&result);
        assert_eq!(adds, 1);
        assert_eq!(rems, 1);
    }
    #[test]
    fn test_diff_is_empty_true() {
        let config = DiffConfig::new();
        let result = line_diff("same\n", "same\n", &config);
        assert!(diff_is_empty(&result));
    }
    #[test]
    fn test_diff_is_empty_false() {
        let config = DiffConfig::new();
        let result = line_diff("a\n", "b\n", &config);
        assert!(!diff_is_empty(&result));
    }
    #[test]
    fn test_largest_hunk() {
        let config = DiffConfig::new();
        let result = line_diff("a\nb\nc\nd\n", "a\nx\ny\nz\nd\n", &config);
        let lh = largest_hunk(&result);
        assert!(lh.is_some());
    }
}
#[allow(dead_code)]
pub fn format_diff_with_line_numbers(result: &DiffResult) -> String {
    let mut out = String::new();
    for hunk in &result.hunks {
        out.push_str(&format!("@@ {} @@\n", hunk.header));
        for line in &hunk.lines {
            let old_n = line
                .old_lineno
                .map(|n| format!("{:4}", n))
                .unwrap_or_else(|| "    ".to_string());
            let new_n = line
                .new_lineno
                .map(|n| format!("{:4}", n))
                .unwrap_or_else(|| "    ".to_string());
            let prefix = match line.kind {
                DiffLineKind::Added => '+',
                DiffLineKind::Removed => '-',
                DiffLineKind::Context => ' ',
            };
            out.push_str(&format!(
                "{} {} {} {}\n",
                old_n, new_n, prefix, line.content
            ));
        }
    }
    out
}
#[cfg(test)]
mod diff_numbering_tests {
    use super::*;
    #[test]
    fn test_format_diff_with_line_numbers() {
        let config = DiffConfig::new();
        let result = line_diff("a\nb\n", "a\nc\n", &config);
        let formatted = format_diff_with_line_numbers(&result);
        assert!(formatted.contains("@@"));
    }
    #[test]
    fn test_diff_total_lines_count() {
        let config = DiffConfig::new();
        let result = line_diff("a\nb\n", "a\nc\n", &config);
        let total = diff_total_lines(&result);
        assert!(total >= 2);
    }
}
#[allow(dead_code)]
pub fn diff_version() -> &'static str {
    "1.0.0"
}
#[allow(dead_code)]
pub fn diff_author() -> &'static str {
    "oxilean"
}
#[allow(dead_code)]
pub fn diff_max_context() -> usize {
    1000
}
#[allow(dead_code)]
pub fn diff_default_context() -> usize {
    3
}
#[allow(dead_code)]
pub fn diff_supports_binary() -> bool {
    false
}
#[allow(dead_code)]
pub fn diff_supports_unicode() -> bool {
    true
}
#[allow(dead_code)]
pub fn diff_supports_crlf() -> bool {
    true
}
#[allow(dead_code)]
pub fn diff_supports_hunks() -> bool {
    true
}
#[allow(dead_code)]
pub fn diff_supports_word_diff() -> bool {
    true
}
#[allow(dead_code)]
pub fn diff_supports_char_diff() -> bool {
    true
}
#[allow(dead_code)]
pub fn diff_supports_structural_diff() -> bool {
    true
}
