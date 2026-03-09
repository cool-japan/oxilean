//! Test infrastructure: parsing helpers, directory scanning, stats reporting.

use oxilean_parse::{Lexer, Parser};
use std::path::Path;

use super::normalize::normalize_lean4_to_oxilean;
use super::normalize_2::find_top_level_assign;
use super::types::CompatStats;

/// Find the position of a line comment (`--`) that is not inside a string or operator.
fn find_line_comment(line: &str) -> Option<usize> {
    let bytes = line.as_bytes();
    let len = bytes.len();
    let mut i = 0;
    while i + 1 < len {
        if bytes[i] == b'-' && bytes[i + 1] == b'-' {
            // Check it's not preceded by a non-space char that would make it an operator
            if i == 0 || bytes[i - 1] == b' ' || bytes[i - 1] == b'\t' {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

/// Check if a declaration string has a type annotation (`:` at depth 0, not `:=`).
fn has_type_annotation(decl: &str) -> bool {
    let bytes = decl.as_bytes();
    let len = bytes.len();
    let mut depth = 0usize;
    let mut i = 0;
    // Skip the keyword (theorem/lemma/def/axiom) and name
    while i < len && bytes[i] != b' ' {
        i += 1;
    }
    while i < len {
        match bytes[i] {
            b'(' | b'{' | b'[' => {
                depth += 1;
                i += 1;
            }
            b')' | b'}' | b']' => {
                depth = depth.saturating_sub(1);
                i += 1;
            }
            b':' if depth == 0 && i + 1 < len && bytes[i + 1] == b'=' => {
                // This is `:=`, skip it
                i += 2;
            }
            b':' if depth == 0 => {
                return true;
            }
            _ => {
                i += 1;
            }
        }
    }
    false
}

/// Check if a line starting with `|` is likely a match arm (equation compiler).
/// Match arms in Lean 4 always contain `=>` or `↦` as the arm separator.
fn is_match_arm(trimmed: &str) -> bool {
    // Must start with `|`
    if !trimmed.starts_with('|') {
        return false;
    }
    // Check for `=>` or `↦` at depth 0 (not inside brackets)
    let bytes = trimmed.as_bytes();
    let len = bytes.len();
    let mut depth = 0usize;
    let mut i = 1; // skip the leading `|`
    while i < len {
        match bytes[i] {
            b'(' | b'{' | b'[' => {
                depth += 1;
                i += 1;
            }
            b')' | b'}' | b']' => {
                depth = depth.saturating_sub(1);
                i += 1;
            }
            b'=' if depth == 0 && i + 1 < len && bytes[i + 1] == b'>' => {
                return true;
            }
            _ => {
                // Check for `↦` (U+21A6, 3 bytes in UTF-8: E2 86 A6)
                if depth == 0
                    && i + 2 < len
                    && bytes[i] == 0xE2
                    && bytes[i + 1] == 0x86
                    && bytes[i + 2] == 0xA6
                {
                    return true;
                }
                i += 1;
            }
        }
    }
    false
}

/// Check if a line starts a declaration (at column 0).
fn is_decl_start(line: &str) -> bool {
    if line.starts_with(' ') || line.starts_with('\t') {
        return false;
    }
    let trimmed = line.trim_start();
    trimmed.starts_with("theorem ")
        || trimmed.starts_with("lemma ")
        || trimmed.starts_with("def ")
        || trimmed.starts_with("axiom ")
}

/// Extract theorem/lemma/def/axiom declarations from a Lean 4 source file.
/// Handles both single-line and multi-line declarations by joining continuation
/// lines (indented lines following a declaration start at column 0).
pub(super) fn extract_single_line_decls(content: &str) -> Vec<String> {
    let mut decls = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let len = lines.len();
    let mut i = 0;
    while i < len {
        let line = lines[i];
        if is_decl_start(line) {
            // Check single-line case first (fast path)
            let stripped = line.trim_end();
            if let Some(pos) = find_top_level_assign(stripped) {
                let after_assign = stripped[pos + 2..].trim();
                if !after_assign.is_empty() {
                    decls.push(line.to_string());
                    i += 1;
                    continue;
                }
            }
            // Multi-line: join continuation lines (indented or starting with special chars)
            let mut joined = line.trim_end().to_string();
            let mut j = i + 1;
            let mut found_assign = false;
            while j < len {
                let next = lines[j];
                let next_trimmed = next.trim_start();
                // Stop if we hit a new top-level declaration, blank line, or non-indented non-continuation
                if next.is_empty()
                    || next_trimmed.is_empty()
                    || is_decl_start(next)
                    || (!next.starts_with(' ')
                        && !next.starts_with('\t')
                        && !next_trimmed.starts_with('|')
                        && !next_trimmed.starts_with("where"))
                {
                    break;
                }
                // If the continuation line starts with `|` and looks like a match arm
                // (contains `=>` which is the Lean 4 match arm separator),
                // and the declaration already has a type annotation, stop:
                // the `|` is a match arm that we'll replace with `:= sorry`.
                if next_trimmed.starts_with('|')
                    && has_type_annotation(&joined)
                    && is_match_arm(next_trimmed)
                {
                    break;
                }
                // If the continuation is a `where` block, stop and handle it
                if next_trimmed.starts_with("where") {
                    break;
                }
                // Strip line comments before joining
                let line_content = if let Some(cpos) = find_line_comment(next_trimmed) {
                    next_trimmed[..cpos].trim_end()
                } else {
                    next_trimmed
                };
                if line_content.is_empty() {
                    j += 1;
                    continue;
                }
                joined.push(' ');
                joined.push_str(line_content);
                // Check if we now have := with content after it
                if let Some(pos) = find_top_level_assign(&joined) {
                    let after_assign = joined[pos + 2..].trim();
                    if !after_assign.is_empty() {
                        found_assign = true;
                        break;
                    }
                }
                j += 1;
            }
            if found_assign {
                decls.push(joined);
            } else if has_type_annotation(&joined) {
                // No `:= content` found, but we have a type annotation.
                // This is likely a pattern-match definition — add `:= sorry`
                let trimmed = joined.trim_end();
                if let Some(stripped) = trimmed.strip_suffix(" where") {
                    joined = format!("{}:= sorry", stripped);
                } else if find_top_level_assign(&joined).is_none() {
                    joined.push_str(" := sorry");
                }
                decls.push(joined);
            }
            i = j.max(i + 1);
        } else {
            i += 1;
        }
    }
    decls
}
/// Try to parse a single normalized declaration string.
/// Returns true if parsing succeeds.
pub(super) fn try_parse_decl(src: &str) -> bool {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    let has_lex_error = tokens
        .iter()
        .any(|t| matches!(&t.kind, oxilean_parse::TokenKind::Error(_)));
    if has_lex_error {
        return false;
    }
    let mut parser = Parser::new(tokens);
    parser.parse_decl().is_ok()
}
/// Try to parse and return the error message on failure.
pub(super) fn try_parse_decl_err(src: &str) -> Result<(), String> {
    let mut lexer = Lexer::new(src);
    let tokens = lexer.tokenize();
    for t in &tokens {
        if let oxilean_parse::TokenKind::Error(ref e) = t.kind {
            return Err(format!("lex_error: {e}"));
        }
    }
    let mut parser = Parser::new(tokens);
    match parser.parse_decl() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{e}")),
    }
}
/// Run compatibility test on all .lean files in a directory (non-recursive).
pub(super) fn run_compat_on_dir(dir: &Path, max_files: usize) -> CompatStats {
    let mut stats = CompatStats::default();
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return stats,
    };
    let mut file_count = 0;
    for entry in entries.flatten() {
        if file_count >= max_files {
            break;
        }
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("lean") {
            continue;
        }
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        file_count += 1;
        stats.files_processed += 1;
        let decls = extract_single_line_decls(&content);
        for decl in &decls {
            let normalized = normalize_lean4_to_oxilean(decl);
            stats.total += 1;
            if try_parse_decl(&normalized) {
                stats.parsed_ok += 1;
            } else if stats.failures.len() < 5 {
                let reason = classify_failure_reason(decl);
                let snippet = {
                    let limit = decl
                        .char_indices()
                        .nth(80)
                        .map(|(i, _)| i)
                        .unwrap_or(decl.len());
                    if limit < decl.len() {
                        format!("{}...", &decl[..limit])
                    } else {
                        decl.clone()
                    }
                };
                stats.failures.push((snippet, reason));
            }
        }
    }
    stats
}
/// Run compatibility test on .lean files recursively in a directory.
pub(super) fn run_compat_recursive(dir: &Path, max_files: usize) -> CompatStats {
    let mut stats = CompatStats::default();
    run_compat_recursive_inner(dir, &mut stats, &mut 0, max_files);
    stats
}
fn run_compat_recursive_inner(
    dir: &Path,
    stats: &mut CompatStats,
    file_count: &mut usize,
    max_files: usize,
) {
    if *file_count >= max_files {
        return;
    }
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        if *file_count >= max_files {
            break;
        }
        let path = entry.path();
        if path.is_dir() {
            run_compat_recursive_inner(&path, stats, file_count, max_files);
        } else if path.extension().and_then(|e| e.to_str()) == Some("lean") {
            let content = match std::fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            *file_count += 1;
            stats.files_processed += 1;
            let decls = extract_single_line_decls(&content);
            for decl in &decls {
                let normalized = normalize_lean4_to_oxilean(decl);
                stats.total += 1;
                if try_parse_decl(&normalized) {
                    stats.parsed_ok += 1;
                } else if stats.failures.len() < 5 {
                    let reason = classify_failure_reason(decl);
                    let limit = decl
                        .char_indices()
                        .nth(80)
                        .map(|(i, _)| i)
                        .unwrap_or(decl.len());
                    let snippet = if limit < decl.len() {
                        format!("{}...", &decl[..limit])
                    } else {
                        decl.clone()
                    };
                    stats.failures.push((snippet, reason));
                }
            }
        }
    }
}
/// Classify why a Lean 4 declaration failed to parse in OxiLean.
fn classify_failure_reason(decl: &str) -> String {
    if decl.contains('(') && !decl.contains(":=") {
        return "missing_assign".to_string();
    }
    if decl.contains("where") {
        return "where_clause".to_string();
    }
    if decl.contains("by ") || decl.contains(":= by") {
        return "tactic_proof_remaining".to_string();
    }
    if decl.contains('\u{27E8}') || decl.contains('\u{27E9}') {
        return "anon_constructor".to_string();
    }
    if decl.contains("Sort*") || decl.contains("Type*") || decl.contains("Sort _") {
        return "universe_polymorphism".to_string();
    }
    if decl.contains("@[") {
        return "attribute_remaining".to_string();
    }
    if decl.contains('\u{2208}') || decl.contains('\u{2286}') || decl.contains('\u{2282}') {
        return "set_operators_remaining".to_string();
    }
    if decl.contains('\u{22C5}') || decl.contains('\u{00B7}') {
        return "dot_lambda".to_string();
    }
    let colon_pos = decl.find(" : ");
    let paren_pos = decl.find(" (");
    if let (Some(c), Some(p)) = (colon_pos, paren_pos) {
        if p < c {
            return "head_binders_remaining".to_string();
        }
    }
    if decl.contains('\u{2194}') && !decl.contains(":= sorry") {
        return "iff_in_type".to_string();
    }
    "other".to_string()
}
/// Mathlib4 root directory. Set via `MATHLIB4_ROOT` env var or `.env.mathlib` file.
pub(super) fn mathlib4_root() -> Option<String> {
    if let Ok(val) = std::env::var("MATHLIB4_ROOT") {
        if !val.is_empty() {
            return Some(val);
        }
    }
    let env_file = Path::new(env!("CARGO_MANIFEST_DIR")).join(".env.mathlib");
    if let Ok(content) = std::fs::read_to_string(&env_file) {
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if let Some(val) = line.strip_prefix("MATHLIB4_ROOT=") {
                let val = val.trim();
                if !val.is_empty() {
                    return Some(val.to_string());
                }
            }
        }
    }
    None
}
/// Print a compat stats report.
pub(super) fn print_stats(category: &str, stats: &CompatStats) {
    println!(
        "\n[CompatStats] Category: {category}\n\
         Files processed: {}\n\
         Total declarations: {}\n\
         Parsed OK: {} ({:.1}%)",
        stats.files_processed,
        stats.total,
        stats.parsed_ok,
        stats.success_rate()
    );
    if !stats.failures.is_empty() {
        println!("  Sample failures:");
        for (snippet, reason) in &stats.failures {
            println!("    [{reason}] {snippet}");
        }
    }
}
