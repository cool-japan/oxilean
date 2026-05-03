//! Mutation testing functions for oxilean-lint.
//!
//! This module implements the text-based mutation-generation engine and all
//! reporting helpers. No AST or external build-system calls are required;
//! every transformation is a pure string operation so the logic can be
//! exercised in unit tests without invoking a Rust compiler.

use std::collections::HashMap;
use std::fmt::Write as FmtWrite;

use super::types::{
    Mutation, MutationConfig, MutationFilter, MutationOperator, MutationReport, MutationResult,
    MutationScanContext,
};

// ============================================================
// Internal helpers
// ============================================================

/// Return the 1-based (line, col) for a byte `offset` inside `source`.
fn offset_to_line_col(source: &str, offset: usize) -> (u32, u32) {
    let before = &source[..offset.min(source.len())];
    let line = before.chars().filter(|&c| c == '\n').count() as u32 + 1;
    let col = before.rfind('\n').map(|p| offset - p - 1).unwrap_or(offset) as u32 + 1;
    (line, col)
}

/// Collect all byte offsets of non-overlapping occurrences of `needle` in `haystack`.
fn find_all_offsets(haystack: &str, needle: &str) -> Vec<usize> {
    let mut offsets = Vec::new();
    let mut start = 0usize;
    while let Some(pos) = haystack[start..].find(needle) {
        offsets.push(start + pos);
        start += pos + needle.len().max(1);
    }
    offsets
}

/// Return `true` when the character immediately before `offset` and immediately
/// after `offset + len` in `source` are both non-alphanumeric / non-underscore,
/// so that we avoid matching tokens that are substrings of identifiers.
///
/// Word-boundary check: boundary characters are whitespace, punctuation, or
/// beginning/end of string.
fn is_token_boundary(source: &str, offset: usize, len: usize) -> bool {
    let before_ok = if offset == 0 {
        true
    } else {
        let ch = source[..offset].chars().next_back().unwrap_or(' ');
        !ch.is_alphanumeric() && ch != '_'
    };
    let after_ok = {
        let end = offset + len;
        if end >= source.len() {
            true
        } else {
            let ch = source[end..].chars().next().unwrap_or(' ');
            !ch.is_alphanumeric() && ch != '_'
        }
    };
    before_ok && after_ok
}

// ============================================================
// Core mutation discovery
// ============================================================

/// Scan `source` text and return every mutation that any operator in
/// `operators` can generate.
///
/// The scan is purely textual — no AST or type information is used.
///
/// # Arguments
///
/// * `source`    – The Rust/Lean/any-language source text to scan.
/// * `operators` – The set of operators to apply. Duplicate entries are fine
///   (they are deduplicated internally).
/// * `file`      – Logical file name embedded in every returned [`Mutation`].
pub fn find_mutations(source: &str, operators: &[MutationOperator], file: &str) -> Vec<Mutation> {
    let mut mutations: Vec<Mutation> = Vec::new();

    // Deduplicate operators while preserving order.
    let mut seen_ops = std::collections::HashSet::new();
    let unique_ops: Vec<&MutationOperator> = operators
        .iter()
        .filter(|op| seen_ops.insert(op.to_string()))
        .collect();

    for op in unique_ops {
        match op {
            MutationOperator::ReplaceBoolLiteral => {
                collect_bool_mutations(source, file, &mut mutations);
            }
            MutationOperator::NegateCondition => {
                collect_negate_condition_mutations(source, file, &mut mutations);
            }
            MutationOperator::ReplaceArithmetic => {
                collect_arithmetic_mutations(source, file, &mut mutations);
            }
            MutationOperator::RemoveReturn => {
                collect_remove_return_mutations(source, file, &mut mutations);
            }
            MutationOperator::ReplaceComparison => {
                collect_comparison_mutations(source, file, &mut mutations);
            }
            MutationOperator::ReplaceLogical => {
                collect_logical_mutations(source, file, &mut mutations);
            }
            MutationOperator::IncrementLiteral => {
                collect_increment_mutations(source, file, &mut mutations);
            }
            MutationOperator::DecrementLiteral => {
                collect_decrement_mutations(source, file, &mut mutations);
            }
        }
    }

    mutations
}

// ---- per-operator collectors ----

fn collect_bool_mutations(source: &str, file: &str, out: &mut Vec<Mutation>) {
    for (needle, replacement) in &[("true", "false"), ("false", "true")] {
        for offset in find_all_offsets(source, needle) {
            if !is_token_boundary(source, offset, needle.len()) {
                continue;
            }
            let (line, col) = offset_to_line_col(source, offset);
            out.push(Mutation::new(
                MutationOperator::ReplaceBoolLiteral,
                file,
                line,
                col,
                *needle,
                *replacement,
            ));
        }
    }
}

fn collect_negate_condition_mutations(source: &str, file: &str, out: &mut Vec<Mutation>) {
    // Look for `if ` patterns and wrap the condition token.
    // Strategy: find `if ` followed by an expression, emit `if !(<expr>)`.
    // We use a heuristic: scan lines for `if ` (not `else if ` separately)
    // and record a mutation that prepends `!` to the condition.
    for (line_idx, line) in source.lines().enumerate() {
        // Detect `if <condition>` at various indentation levels.
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("if ") {
            // Determine column of the `if` keyword.
            let indent = line.len() - trimmed.len();
            let col = indent as u32 + 1;
            let condition = rest.trim_end_matches('{').trim();
            if condition.is_empty() {
                continue;
            }
            // Original: `if <condition>`, mutated: `if !(<condition>)`
            let original = format!("if {}", rest.trim_end_matches('{').trim());
            let mutated = format!("if !({})", condition);
            out.push(Mutation::new(
                MutationOperator::NegateCondition,
                file,
                (line_idx + 1) as u32,
                col,
                original,
                mutated,
            ));
        }
    }
}

fn collect_arithmetic_mutations(source: &str, file: &str, out: &mut Vec<Mutation>) {
    // `+` → `-`  and  `*` → `/`
    // We deliberately skip `+=`, `-=`, `++` etc. by checking surroundings.
    for (offset, ch) in source.char_indices() {
        let byte_after = source
            .as_bytes()
            .get(offset + ch.len_utf8())
            .copied()
            .unwrap_or(b' ');

        match ch {
            '+' => {
                // Skip `+=`, `++`, `->` (not `+`, just in case)
                if byte_after == b'=' || byte_after == b'+' {
                    continue;
                }
                let (line, col) = offset_to_line_col(source, offset);
                out.push(Mutation::new(
                    MutationOperator::ReplaceArithmetic,
                    file,
                    line,
                    col,
                    "+",
                    "-",
                ));
            }
            '*' => {
                // Skip `*=`, `**`, `*/` (end of block comment), `/*` (start).
                if byte_after == b'=' || byte_after == b'*' || byte_after == b'/' {
                    continue;
                }
                // Skip pointer dereferences (unary `*`).
                // Binary multiplication is preceded by an alphanumeric char, `_`,
                // `)`, or `]` (possibly with surrounding spaces).
                // Unary dereference is preceded only by operators/punctuation.
                //
                // Strategy: look backwards past whitespace to find the non-space
                // predecessor. If it is alphanumeric, `_`, `)`, or `]` then this is
                // multiplication; otherwise it is a dereference.
                let prefix = &source[..offset];
                let effective_before = prefix
                    .chars()
                    .rev()
                    .find(|c| !c.is_whitespace())
                    .unwrap_or('(');
                let is_multiplication = effective_before.is_alphanumeric()
                    || effective_before == '_'
                    || effective_before == ')'
                    || effective_before == ']';
                if !is_multiplication {
                    continue;
                }
                let (line, col) = offset_to_line_col(source, offset);
                out.push(Mutation::new(
                    MutationOperator::ReplaceArithmetic,
                    file,
                    line,
                    col,
                    "*",
                    "/",
                ));
            }
            _ => {}
        }
    }
}

fn collect_remove_return_mutations(source: &str, file: &str, out: &mut Vec<Mutation>) {
    // Match `return <expr>` and replace <expr> with `Default::default()`.
    for (line_idx, line) in source.lines().enumerate() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("return ") {
            let expr = rest.trim_end_matches(';').trim();
            if expr.is_empty() || expr == "Default::default()" {
                continue;
            }
            let indent = line.len() - trimmed.len();
            let original = format!("return {}", expr);
            let mutated = "return Default::default()".to_string();
            out.push(Mutation::new(
                MutationOperator::RemoveReturn,
                file,
                (line_idx + 1) as u32,
                indent as u32 + 1,
                original,
                mutated,
            ));
        }
    }
}

fn collect_comparison_mutations(source: &str, file: &str, out: &mut Vec<Mutation>) {
    // Ordered pairs: (original, mutated)
    // We must try longer tokens first so `==` is not partially matched as `=`.
    let replacements: &[(&str, &str)] = &[
        ("==", "!="),
        ("!=", "=="),
        ("<=", "<"),
        (">=", ">"),
        ("<", "<="),
        (">", ">="),
    ];

    // Track which offsets have already been covered to avoid double-emitting.
    let mut covered: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for (needle, replacement) in replacements {
        for offset in find_all_offsets(source, needle) {
            if covered.contains(&offset) {
                continue;
            }
            // For single-char operators, make sure we didn't already match a
            // two-char operator at this position.
            let (line, col) = offset_to_line_col(source, offset);
            covered.insert(offset);
            out.push(Mutation::new(
                MutationOperator::ReplaceComparison,
                file,
                line,
                col,
                *needle,
                *replacement,
            ));
        }
    }
}

fn collect_logical_mutations(source: &str, file: &str, out: &mut Vec<Mutation>) {
    for (needle, replacement) in &[("&&", "||"), ("||", "&&")] {
        for offset in find_all_offsets(source, needle) {
            let (line, col) = offset_to_line_col(source, offset);
            out.push(Mutation::new(
                MutationOperator::ReplaceLogical,
                file,
                line,
                col,
                *needle,
                *replacement,
            ));
        }
    }
}

fn collect_increment_mutations(source: &str, file: &str, out: &mut Vec<Mutation>) {
    collect_literal_shift_mutations(source, file, MutationOperator::IncrementLiteral, 1, out);
}

fn collect_decrement_mutations(source: &str, file: &str, out: &mut Vec<Mutation>) {
    collect_literal_shift_mutations(source, file, MutationOperator::DecrementLiteral, -1, out);
}

/// Shared implementation for `IncrementLiteral` and `DecrementLiteral`.
///
/// Scans `source` for standalone decimal integer literals and emits a mutation
/// that replaces `n` with `n + delta`.
fn collect_literal_shift_mutations(
    source: &str,
    file: &str,
    operator: MutationOperator,
    delta: i64,
    out: &mut Vec<Mutation>,
) {
    let bytes = source.as_bytes();
    let len = bytes.len();
    let mut i = 0usize;
    while i < len {
        let ch = bytes[i] as char;
        if ch.is_ascii_digit() {
            // Check that this is not part of a float (no preceding `.`).
            let preceded_by_dot = i > 0 && bytes[i - 1] == b'.';
            // Find the end of the numeric literal.
            let start = i;
            while i < len && (bytes[i] as char).is_ascii_digit() {
                i += 1;
            }
            // Skip floats (followed by `.` or `e`/`E`).
            let followed_by_dot =
                i < len && (bytes[i] == b'.' || bytes[i] == b'e' || bytes[i] == b'E');
            if preceded_by_dot || followed_by_dot {
                continue;
            }
            // Ensure token boundaries.
            let before_ok = start == 0
                || !((bytes[start - 1] as char).is_alphanumeric() || bytes[start - 1] == b'_');
            let after_ok = i >= len || !((bytes[i] as char).is_alphanumeric() || bytes[i] == b'_');
            if !before_ok || !after_ok {
                continue;
            }

            let literal = &source[start..i];
            if let Ok(value) = literal.parse::<i64>() {
                let new_value = value + delta;
                if new_value >= 0 || delta > 0 {
                    let (line, col) = offset_to_line_col(source, start);
                    out.push(Mutation::new(
                        operator.clone(),
                        file,
                        line,
                        col,
                        literal,
                        new_value.to_string(),
                    ));
                }
            }
        } else {
            i += 1;
        }
    }
}

// ============================================================
// Mutation application
// ============================================================

/// Apply a single [`Mutation`] to `source` text by replacing the first
/// occurrence of `mutation.original` that appears on the correct line.
///
/// Returns the modified source string, or the original if the mutation
/// cannot be applied (e.g. the token is no longer present).
pub fn apply_mutation(source: &str, mutation: &Mutation) -> String {
    // Walk lines until we reach the target line, then replace the first
    // occurrence of `original` in that line.
    let target_line = mutation.line as usize;
    let mut result = String::with_capacity(source.len() + mutation.mutated.len());
    let mut replaced = false;

    for (idx, line) in source.lines().enumerate() {
        let line_no = idx + 1;
        if line_no == target_line && !replaced {
            if let Some(pos) = line.find(mutation.original.as_str()) {
                result.push_str(&line[..pos]);
                result.push_str(&mutation.mutated);
                result.push_str(&line[pos + mutation.original.len()..]);
                replaced = true;
                result.push('\n');
                continue;
            }
        }
        result.push_str(line);
        result.push('\n');
    }

    // Trim trailing newline if original source did not end with one.
    if !source.ends_with('\n') && result.ends_with('\n') {
        result.truncate(result.len() - 1);
    }

    result
}

// ============================================================
// File-level helpers
// ============================================================

/// Read the file at `file_path`, scan it for mutations using the operators
/// specified in `cfg`, and return the list of possible mutations.
///
/// Returns an empty `Vec` (not an error) when the file cannot be read, so
/// callers that aggregate many files continue running gracefully.
pub fn generate_mutations_for_file(file_path: &str, cfg: &MutationConfig) -> Vec<Mutation> {
    let source = match std::fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    let mut mutations = find_mutations(&source, &cfg.operators, file_path);

    // Apply per-file filter from target_files if set.
    if !cfg.target_files.is_empty() {
        let path_matches = cfg
            .target_files
            .iter()
            .any(|t| file_path.contains(t.as_str()));
        if !path_matches {
            return Vec::new();
        }
    }

    // Honour max_mutations cap.
    if cfg.max_mutations > 0 && mutations.len() > cfg.max_mutations {
        mutations.truncate(cfg.max_mutations);
    }

    mutations
}

/// Apply a [`MutationFilter`] to reduce a mutation list in-place.
pub fn filter_mutations(mutations: Vec<Mutation>, filter: &MutationFilter) -> Vec<Mutation> {
    mutations
        .into_iter()
        .filter(|m| filter.accepts(m))
        .collect()
}

// ============================================================
// Scoring and reporting
// ============================================================

/// Return the mutation score for `report`: killed / total, or 0.0 when empty.
pub fn score_mutation_report(report: &MutationReport) -> f64 {
    if report.total == 0 {
        return 0.0;
    }
    report.killed as f64 / report.total as f64
}

/// Format `report` as a human-readable table with columns:
/// `File | Line | Operator | Original | Mutated | Result`.
pub fn format_mutation_report(report: &MutationReport) -> String {
    if report.mutations.is_empty() {
        return "No mutations recorded.".to_string();
    }

    let mut out = String::new();

    // Header
    let _ = writeln!(
        out,
        "{:<40} {:>6} {:<22} {:<20} {:<20} {:<12}",
        "File", "Line", "Operator", "Original", "Mutated", "Result"
    );
    let sep = "-".repeat(126);
    let _ = writeln!(out, "{}", sep);

    for (mutation, result) in &report.mutations {
        let file_display = if mutation.file.len() > 38 {
            format!("…{}", &mutation.file[mutation.file.len() - 37..])
        } else {
            mutation.file.clone()
        };
        let orig_display = truncate_str(&mutation.original, 18);
        let mut_display = truncate_str(&mutation.mutated, 18);
        let _ = writeln!(
            out,
            "{:<40} {:>6} {:<22} {:<20} {:<20} {:<12}",
            file_display,
            mutation.line,
            mutation.operator.to_string(),
            orig_display,
            mut_display,
            result.to_string()
        );
    }

    let _ = writeln!(out, "{}", "-".repeat(126));
    let _ = writeln!(
        out,
        "Total: {}  Killed: {}  Survived: {}  Kill rate: {:.1}%",
        report.total,
        report.killed,
        report.survived,
        report.kill_rate * 100.0
    );
    out
}

/// Return a per-operator breakdown: operator name → `(killed, total)`.
///
/// Operators that produced zero mutations are not included in the map.
pub fn mutations_by_operator(report: &MutationReport) -> HashMap<String, (usize, usize)> {
    let mut map: HashMap<String, (usize, usize)> = HashMap::new();
    for (mutation, result) in &report.mutations {
        let key = mutation.operator.to_string();
        let entry = map.entry(key).or_insert((0, 0));
        entry.1 += 1;
        if *result == MutationResult::Killed {
            entry.0 += 1;
        }
    }
    map
}

/// Return references to every mutation that survived the test suite.
pub fn survived_mutations(report: &MutationReport) -> Vec<&Mutation> {
    report
        .mutations
        .iter()
        .filter_map(|(m, r)| {
            if *r == MutationResult::Survived {
                Some(m)
            } else {
                None
            }
        })
        .collect()
}

/// Return references to every mutation that was killed by the test suite.
pub fn killed_mutations(report: &MutationReport) -> Vec<&Mutation> {
    report
        .mutations
        .iter()
        .filter_map(|(m, r)| {
            if *r == MutationResult::Killed {
                Some(m)
            } else {
                None
            }
        })
        .collect()
}

/// Return a summary string suitable for CI output lines.
pub fn summarize_mutation_report(report: &MutationReport) -> String {
    format!(
        "Mutation testing: {}/{} killed ({:.1}% score) | {} survived | {} timeout | {} compile-error",
        report.killed,
        report.total,
        score_mutation_report(report) * 100.0,
        report.survived,
        report
            .mutations
            .iter()
            .filter(|(_, r)| *r == MutationResult::Timeout)
            .count(),
        report
            .mutations
            .iter()
            .filter(|(_, r)| *r == MutationResult::CompileError)
            .count(),
    )
}

// ============================================================
// Utility
// ============================================================

/// Truncate `s` to at most `max_chars` characters, appending `…` if truncated.
fn truncate_str(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_chars {
        s.to_string()
    } else {
        let mut result: String = chars[..max_chars - 1].iter().collect();
        result.push('…');
        result
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mutation_testing::types::{MutationConfig, MutationResult, MutationStats};

    // ---- find_mutations: ReplaceBoolLiteral ----

    #[test]
    fn test_bool_true_to_false() {
        let src = "let x = true;";
        let muts = find_mutations(src, &[MutationOperator::ReplaceBoolLiteral], "test.rs");
        assert_eq!(muts.len(), 1);
        assert_eq!(muts[0].original, "true");
        assert_eq!(muts[0].mutated, "false");
    }

    #[test]
    fn test_bool_false_to_true() {
        let src = "let x = false;";
        let muts = find_mutations(src, &[MutationOperator::ReplaceBoolLiteral], "test.rs");
        assert_eq!(muts.len(), 1);
        assert_eq!(muts[0].original, "false");
        assert_eq!(muts[0].mutated, "true");
    }

    #[test]
    fn test_bool_both_literals() {
        let src = "let a = true; let b = false;";
        let muts = find_mutations(src, &[MutationOperator::ReplaceBoolLiteral], "test.rs");
        assert_eq!(muts.len(), 2);
    }

    #[test]
    fn test_bool_not_in_identifier() {
        // `trueness` should NOT match `true`
        let src = "let trueness = 1;";
        let muts = find_mutations(src, &[MutationOperator::ReplaceBoolLiteral], "test.rs");
        assert!(muts.is_empty(), "expected no mutations, got {:?}", muts);
    }

    // ---- find_mutations: NegateCondition ----

    #[test]
    fn test_negate_if_condition() {
        let src = "if x > 0 {\n    foo();\n}";
        let muts = find_mutations(src, &[MutationOperator::NegateCondition], "test.rs");
        assert!(!muts.is_empty());
        let m = &muts[0];
        assert!(m.mutated.starts_with("if !("), "mutated: {}", m.mutated);
    }

    #[test]
    fn test_negate_condition_line_number() {
        let src = "fn f() {\n    if flag {\n        bar();\n    }\n}";
        let muts = find_mutations(src, &[MutationOperator::NegateCondition], "test.rs");
        assert!(!muts.is_empty());
        assert_eq!(muts[0].line, 2);
    }

    // ---- find_mutations: ReplaceArithmetic ----

    #[test]
    fn test_arithmetic_plus_to_minus() {
        let src = "let y = a + b;";
        let muts = find_mutations(src, &[MutationOperator::ReplaceArithmetic], "test.rs");
        let plus_muts: Vec<_> = muts.iter().filter(|m| m.original == "+").collect();
        assert!(!plus_muts.is_empty());
        assert_eq!(plus_muts[0].mutated, "-");
    }

    #[test]
    fn test_arithmetic_star_to_slash() {
        let src = "let z = a * b;";
        let muts = find_mutations(src, &[MutationOperator::ReplaceArithmetic], "test.rs");
        let star_muts: Vec<_> = muts.iter().filter(|m| m.original == "*").collect();
        assert!(!star_muts.is_empty());
        assert_eq!(star_muts[0].mutated, "/");
    }

    // ---- find_mutations: RemoveReturn ----

    #[test]
    fn test_remove_return_basic() {
        let src = "fn f() -> i32 {\n    return 42;\n}";
        let muts = find_mutations(src, &[MutationOperator::RemoveReturn], "test.rs");
        assert!(!muts.is_empty());
        assert_eq!(muts[0].mutated, "return Default::default()");
    }

    #[test]
    fn test_remove_return_does_not_mutate_default() {
        let src = "fn f() -> i32 {\n    return Default::default();\n}";
        let muts = find_mutations(src, &[MutationOperator::RemoveReturn], "test.rs");
        assert!(muts.is_empty());
    }

    // ---- find_mutations: ReplaceComparison ----

    #[test]
    fn test_comparison_eq_to_ne() {
        let src = "if a == b {";
        let muts = find_mutations(src, &[MutationOperator::ReplaceComparison], "test.rs");
        let eq_muts: Vec<_> = muts.iter().filter(|m| m.original == "==").collect();
        assert!(!eq_muts.is_empty());
        assert_eq!(eq_muts[0].mutated, "!=");
    }

    #[test]
    fn test_comparison_lt_to_lte() {
        let src = "if x < 10 {";
        let muts = find_mutations(src, &[MutationOperator::ReplaceComparison], "test.rs");
        let lt_muts: Vec<_> = muts.iter().filter(|m| m.original == "<").collect();
        assert!(!lt_muts.is_empty());
        assert_eq!(lt_muts[0].mutated, "<=");
    }

    // ---- find_mutations: ReplaceLogical ----

    #[test]
    fn test_logical_and_to_or() {
        let src = "if a && b {";
        let muts = find_mutations(src, &[MutationOperator::ReplaceLogical], "test.rs");
        let and_muts: Vec<_> = muts.iter().filter(|m| m.original == "&&").collect();
        assert!(!and_muts.is_empty());
        assert_eq!(and_muts[0].mutated, "||");
    }

    #[test]
    fn test_logical_or_to_and() {
        let src = "if a || b {";
        let muts = find_mutations(src, &[MutationOperator::ReplaceLogical], "test.rs");
        let or_muts: Vec<_> = muts.iter().filter(|m| m.original == "||").collect();
        assert!(!or_muts.is_empty());
        assert_eq!(or_muts[0].mutated, "&&");
    }

    // ---- find_mutations: IncrementLiteral ----

    #[test]
    fn test_increment_literal() {
        let src = "let n = 5;";
        let muts = find_mutations(src, &[MutationOperator::IncrementLiteral], "test.rs");
        assert!(!muts.is_empty());
        assert_eq!(muts[0].original, "5");
        assert_eq!(muts[0].mutated, "6");
    }

    #[test]
    fn test_increment_zero() {
        let src = "let n = 0;";
        let muts = find_mutations(src, &[MutationOperator::IncrementLiteral], "test.rs");
        assert!(!muts.is_empty());
        assert_eq!(muts[0].mutated, "1");
    }

    // ---- find_mutations: DecrementLiteral ----

    #[test]
    fn test_decrement_literal() {
        let src = "let n = 10;";
        let muts = find_mutations(src, &[MutationOperator::DecrementLiteral], "test.rs");
        assert!(!muts.is_empty());
        assert_eq!(muts[0].original, "10");
        assert_eq!(muts[0].mutated, "9");
    }

    #[test]
    fn test_decrement_one_to_zero() {
        let src = "let n = 1;";
        let muts = find_mutations(src, &[MutationOperator::DecrementLiteral], "test.rs");
        assert!(!muts.is_empty());
        assert_eq!(muts[0].mutated, "0");
    }

    // ---- apply_mutation ----

    #[test]
    fn test_apply_bool_mutation() {
        let src = "let x = true;\n";
        let muts = find_mutations(src, &[MutationOperator::ReplaceBoolLiteral], "f.rs");
        assert!(!muts.is_empty());
        let applied = apply_mutation(src, &muts[0]);
        assert!(applied.contains("false"), "expected false in: {}", applied);
    }

    #[test]
    fn test_apply_mutation_does_not_affect_other_lines() {
        let src = "let a = true;\nlet b = false;\n";
        let muts = find_mutations(src, &[MutationOperator::ReplaceBoolLiteral], "f.rs");
        // First mutation should be on line 1 (`true` → `false`)
        let first = muts
            .iter()
            .find(|m| m.line == 1)
            .expect("no line-1 mutation");
        let applied = apply_mutation(src, first);
        // Line 2 should still contain `false` (the original `false`)
        let lines: Vec<&str> = applied.lines().collect();
        assert!(lines[1].contains("false"), "line 2 unchanged: {}", lines[1]);
    }

    #[test]
    fn test_apply_arithmetic_mutation() {
        let src = "let v = x + y;\n";
        let muts = find_mutations(src, &[MutationOperator::ReplaceArithmetic], "f.rs");
        let plus_mut = muts.iter().find(|m| m.original == "+").expect("+");
        let applied = apply_mutation(src, plus_mut);
        assert!(
            applied.contains("x - y"),
            "expected subtraction: {}",
            applied
        );
    }

    // ---- MutationReport & scoring ----

    #[test]
    fn test_mutation_report_from_results_empty() {
        let report = MutationReport::from_results(vec![]);
        assert_eq!(report.total, 0);
        assert_eq!(report.killed, 0);
        assert_eq!(report.survived, 0);
        assert!((report.kill_rate - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_mutation_report_kill_rate() {
        let m1 = Mutation::new(
            MutationOperator::ReplaceBoolLiteral,
            "f.rs",
            1,
            1,
            "true",
            "false",
        );
        let m2 = Mutation::new(MutationOperator::ReplaceLogical, "f.rs", 2, 5, "&&", "||");
        let report = MutationReport::from_results(vec![
            (m1, MutationResult::Killed),
            (m2, MutationResult::Survived),
        ]);
        assert_eq!(report.total, 2);
        assert_eq!(report.killed, 1);
        assert_eq!(report.survived, 1);
        assert!((report.kill_rate - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_score_mutation_report() {
        let m = Mutation::new(MutationOperator::IncrementLiteral, "f.rs", 1, 1, "5", "6");
        let report = MutationReport::from_results(vec![(m, MutationResult::Killed)]);
        assert!((score_mutation_report(&report) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_score_empty_report() {
        let report = MutationReport::default();
        assert_eq!(score_mutation_report(&report), 0.0);
    }

    // ---- mutations_by_operator ----

    #[test]
    fn test_mutations_by_operator() {
        let m1 = Mutation::new(
            MutationOperator::ReplaceBoolLiteral,
            "f.rs",
            1,
            1,
            "true",
            "false",
        );
        let m2 = Mutation::new(
            MutationOperator::ReplaceBoolLiteral,
            "f.rs",
            2,
            1,
            "false",
            "true",
        );
        let m3 = Mutation::new(MutationOperator::ReplaceLogical, "f.rs", 3, 5, "&&", "||");
        let report = MutationReport::from_results(vec![
            (m1, MutationResult::Killed),
            (m2, MutationResult::Survived),
            (m3, MutationResult::Killed),
        ]);
        let by_op = mutations_by_operator(&report);
        let (bool_killed, bool_total) = by_op["ReplaceBoolLiteral"];
        assert_eq!(bool_total, 2);
        assert_eq!(bool_killed, 1);
        let (log_killed, log_total) = by_op["ReplaceLogical"];
        assert_eq!(log_total, 1);
        assert_eq!(log_killed, 1);
    }

    // ---- survived_mutations ----

    #[test]
    fn test_survived_mutations() {
        let m1 = Mutation::new(
            MutationOperator::ReplaceBoolLiteral,
            "f.rs",
            1,
            1,
            "true",
            "false",
        );
        let m2 = Mutation::new(MutationOperator::IncrementLiteral, "f.rs", 2, 1, "5", "6");
        let report = MutationReport::from_results(vec![
            (m1, MutationResult::Survived),
            (m2, MutationResult::Killed),
        ]);
        let survived = survived_mutations(&report);
        assert_eq!(survived.len(), 1);
        assert_eq!(survived[0].original, "true");
    }

    // ---- format_mutation_report ----

    #[test]
    fn test_format_mutation_report_contains_header() {
        let m = Mutation::new(
            MutationOperator::ReplaceComparison,
            "src/main.rs",
            10,
            5,
            "==",
            "!=",
        );
        let report = MutationReport::from_results(vec![(m, MutationResult::Killed)]);
        let formatted = format_mutation_report(&report);
        assert!(formatted.contains("File"), "header missing: {}", formatted);
        assert!(
            formatted.contains("Operator"),
            "header missing: {}",
            formatted
        );
        assert!(
            formatted.contains("Result"),
            "header missing: {}",
            formatted
        );
    }

    #[test]
    fn test_format_empty_report() {
        let report = MutationReport::default();
        let formatted = format_mutation_report(&report);
        assert!(formatted.contains("No mutations"), "{}", formatted);
    }

    // ---- MutationStats ----

    #[test]
    fn test_mutation_stats_kill_rate_zero_total() {
        let report = MutationReport::default();
        let stats = MutationStats::from_report(&report);
        assert_eq!(stats.kill_rate(), 0.0);
    }

    #[test]
    fn test_mutation_stats_is_perfect() {
        let m = Mutation::new(MutationOperator::DecrementLiteral, "f.rs", 1, 1, "3", "2");
        let report = MutationReport::from_results(vec![(m, MutationResult::Killed)]);
        let stats = MutationStats::from_report(&report);
        assert!(stats.is_perfect());
    }

    #[test]
    fn test_mutation_stats_not_perfect_when_survived() {
        let m = Mutation::new(MutationOperator::DecrementLiteral, "f.rs", 1, 1, "3", "2");
        let report = MutationReport::from_results(vec![(m, MutationResult::Survived)]);
        let stats = MutationStats::from_report(&report);
        assert!(!stats.is_perfect());
    }

    #[test]
    fn test_mutation_stats_meets_threshold() {
        let m = Mutation::new(
            MutationOperator::NegateCondition,
            "f.rs",
            5,
            1,
            "if x",
            "if !(x)",
        );
        let report = MutationReport::from_results(vec![(m, MutationResult::Killed)]);
        let stats = MutationStats::from_report(&report);
        assert!(stats.meets_threshold(0.9));
        assert!(!stats.meets_threshold(1.01));
    }

    // ---- MutationConfig builder ----

    #[test]
    fn test_config_builder_defaults_all_operators() {
        let cfg = MutationConfig::default();
        assert_eq!(cfg.operators.len(), 8);
    }

    #[test]
    fn test_config_builder_custom() {
        let cfg = MutationConfig::builder()
            .with_operator(MutationOperator::ReplaceBoolLiteral)
            .with_operator(MutationOperator::NegateCondition)
            .max_mutations(50)
            .timeout_ms(5_000)
            .target_file("src/lib.rs")
            .build();
        assert_eq!(cfg.operators.len(), 2);
        assert_eq!(cfg.max_mutations, 50);
        assert_eq!(cfg.timeout_ms, 5_000);
        assert_eq!(cfg.target_files, vec!["src/lib.rs"]);
    }

    // ---- generate_mutations_for_file ----

    #[test]
    fn test_generate_mutations_for_nonexistent_file() {
        let cfg = MutationConfig::default();
        let muts = generate_mutations_for_file("/nonexistent/path/file.rs", &cfg);
        assert!(muts.is_empty());
    }

    #[test]
    fn test_generate_mutations_for_temp_file() {
        use std::io::Write;
        let mut tmp = std::env::temp_dir();
        tmp.push("oxilean_mutation_test_input.rs");
        {
            let mut f = std::fs::File::create(&tmp).expect("create tmp");
            writeln!(f, "fn add(a: i32, b: i32) -> i32 {{").expect("write");
            writeln!(f, "    return a + b;").expect("write");
            writeln!(f, "}}").expect("write");
        }
        let cfg = MutationConfig::default();
        let muts = generate_mutations_for_file(tmp.to_str().expect("path"), &cfg);
        assert!(!muts.is_empty(), "expected mutations in temp file");
        let _ = std::fs::remove_file(&tmp);
    }

    // ---- filter_mutations ----

    #[test]
    fn test_filter_mutations_by_operator() {
        use crate::mutation_testing::types::MutationFilter;
        let src = "let x = true && false;";
        let ops = vec![
            MutationOperator::ReplaceBoolLiteral,
            MutationOperator::ReplaceLogical,
        ];
        let all = find_mutations(src, &ops, "f.rs");
        let filter = MutationFilter {
            excluded_operators: vec![MutationOperator::ReplaceLogical],
            ..MutationFilter::accept_all()
        };
        let filtered = filter_mutations(all, &filter);
        assert!(
            filtered
                .iter()
                .all(|m| m.operator != MutationOperator::ReplaceLogical),
            "logical mutations leaked through"
        );
    }

    // ---- summarize_mutation_report ----

    #[test]
    fn test_summarize_mutation_report() {
        let m1 = Mutation::new(
            MutationOperator::ReplaceBoolLiteral,
            "f.rs",
            1,
            1,
            "true",
            "false",
        );
        let m2 = Mutation::new(MutationOperator::ReplaceLogical, "f.rs", 2, 5, "&&", "||");
        let report = MutationReport::from_results(vec![
            (m1, MutationResult::Killed),
            (m2, MutationResult::Timeout),
        ]);
        let summary = summarize_mutation_report(&report);
        assert!(summary.contains("1/2"), "{}", summary);
        assert!(summary.contains("timeout"), "{}", summary);
    }

    // ---- offset_to_line_col helpers ----

    #[test]
    fn test_offset_line_col_first_line() {
        let src = "hello world";
        let (l, c) = offset_to_line_col(src, 6);
        assert_eq!(l, 1);
        assert_eq!(c, 7);
    }

    #[test]
    fn test_offset_line_col_second_line() {
        let src = "line1\nline2";
        let (l, c) = offset_to_line_col(src, 6); // `l` in `line2`
        assert_eq!(l, 2);
        assert_eq!(c, 1);
    }
}
