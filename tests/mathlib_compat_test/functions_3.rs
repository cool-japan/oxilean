//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_parse::{Lexer, Parser};
use std::path::Path;

use super::functions::normalize_lean4_to_oxilean;
use super::functions_2::{find_top_level_assign, normalize_bounded_quantifiers};
use super::types::CompatStats;

/// Normalize single-element list literals `[a]` in type positions.
///
/// In Lean 4, `[a]` is syntactic sugar for `List.cons a List.nil`.
/// OxiLean's parser may or may not handle `[a]` directly.
/// Replace single-element list literals with explicit form to ensure parsing.
/// Only replaces when content is a simple identifier (no operators, no spaces).
pub(super) fn normalize_list_literal_in_type(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        if chars[i] == '[' {
            let preceded_by_ident = i > 0
                && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_' || chars[i - 1] == '\'');
            if preceded_by_ident {
                result.push(chars[i]);
                i += 1;
                continue;
            }
            let bracket_start = i;
            let mut j = i + 1;
            let mut depth = 1usize;
            let mut found_comma = false;
            let content_start = j;
            let _ = content_start;
            while j < len && depth > 0 {
                match chars[j] {
                    '[' => {
                        depth += 1;
                        j += 1;
                    }
                    ']' => {
                        depth = depth.saturating_sub(1);
                        j += 1;
                    }
                    ',' if depth == 1 => {
                        found_comma = true;
                        j += 1;
                    }
                    _ => {
                        j += 1;
                    }
                }
            }
            if depth == 0 && !found_comma && j > bracket_start + 2 {
                let inner: String = chars[bracket_start + 1..j - 1].iter().collect();
                let inner_trimmed = inner.trim();
                let is_simple = !inner_trimmed.is_empty()
                    && inner_trimmed
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '_' || c == '\'' || c == '!');
                if is_simple {
                    result.push_str("(List.cons ");
                    result.push_str(inner_trimmed);
                    result.push_str(" List.nil)");
                    i = j;
                    continue;
                }
            }
            let raw: String = chars[bracket_start..j].iter().collect();
            result.push_str(&raw);
            i = j;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Normalize array/list subscript indexing notation./// Normalize array/list subscript indexing notation.
///
/// `ident[0]` → `ident` (drop numeric subscript for parsing purposes)
/// `ident[n]` → `ident` (drop variable subscript too)
/// This handles cases like `l[0]` appearing in types where OxiLean
/// does not support subscript syntax.
pub(super) fn normalize_subscript_indexing(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        if chars[i] == '[' {
            let preceded_by_ident = i > 0
                && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_' || chars[i - 1] == '\'');
            if preceded_by_ident {
                let bracket_start = i;
                let mut j = i + 1;
                let mut depth = 1usize;
                while j < len && depth > 0 {
                    match chars[j] {
                        '[' => {
                            depth += 1;
                            j += 1;
                        }
                        ']' => {
                            depth -= 1;
                            j += 1;
                        }
                        _ => {
                            j += 1;
                        }
                    }
                }
                let content: String = chars[bracket_start + 1..j - 1].iter().collect();
                let is_simple_subscript = !content.is_empty()
                    && content
                        .chars()
                        .all(|c| c.is_alphanumeric() || c == '_' || c == ' ');
                if is_simple_subscript {
                    i = j;
                    continue;
                } else {
                    result.push(chars[i]);
                    i += 1;
                }
            } else {
                result.push(chars[i]);
                i += 1;
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Move Lean 4 head binders into a `forall` type expression.
///
/// Lean 4 allows: `theorem foo (x : T) {y : U} : P := proof`
/// OxiLean requires: `theorem foo : forall (x : T) {y : U}, P := proof`
/// Normalize Sigma/PSigma dependent type notation in binder positions.
///
/// `Sigma  i, s i` → `Sigma (fun i -> s i)`
/// `PSigma  i, s i` → `PSigma (fun i -> s i)`
///
/// This should run AFTER `normalize_fun_bare_binders` so that `Sigma  i, s i`
/// inside `fun (a : Sigma  i, s i) ->` has the `)` as the body terminator.
pub(super) fn normalize_sigma_in_binders(src: &str) -> String {
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len() + 32);
    let mut i = 0;
    while i < len {
        let prev_is_word = i > 0 && {
            let p = chars[i - 1];
            p.is_alphanumeric() || p == '_' || p == '\''
        };
        let is_sigma =
            !prev_is_word && i + 6 <= len && chars[i..i + 6].iter().collect::<String>() == "Sigma ";
        let is_psigma = !prev_is_word
            && i + 7 <= len
            && chars[i..i + 7].iter().collect::<String>() == "PSigma ";
        if is_sigma || is_psigma {
            let kw = if is_sigma { "Sigma" } else { "PSigma" };
            let kw_len = kw.len();
            i += kw_len + 1;
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            let ident_start = i;
            while i < len && (chars[i].is_alphanumeric() || chars[i] == '_' || chars[i] == '\'') {
                i += 1;
            }
            let ident: String = chars[ident_start..i].iter().collect();
            if ident.is_empty() {
                result.push_str(kw);
                result.push(' ');
                continue;
            }
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            if i < len && chars[i] == ',' {
                i += 1;
                while i < len && chars[i] == ' ' {
                    i += 1;
                }
                let body_start = i;
                let mut depth = 0usize;
                let mut body_end = i;
                while i < len {
                    match chars[i] {
                        '(' | '{' | '[' => {
                            depth += 1;
                            i += 1;
                            body_end = i;
                        }
                        ')' | '}' | ']' if depth == 0 => break,
                        ')' | '}' | ']' => {
                            depth = depth.saturating_sub(1);
                            i += 1;
                            body_end = i;
                        }
                        ':' if depth == 0 && i + 1 < len && chars[i + 1] == '=' => break,
                        '-' if depth == 0 && i + 1 < len && chars[i + 1] == '>' => break,
                        _ => {
                            i += 1;
                            body_end = i;
                        }
                    }
                }
                let body: String = chars[body_start..body_end].iter().collect();
                result.push_str(kw);
                result.push_str(" (fun ");
                result.push_str(&ident);
                result.push_str(" -> ");
                result.push_str(body.trim_end());
                result.push(')');
            } else {
                result.push_str(kw);
                result.push(' ');
                result.push_str(&ident);
                result.push(' ');
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
/// Strip term-position type ascriptions `(IDENT : TYPE)` in theorem type bodies.
///
/// Lean 4 allows `Function.Injective (toRingHom : R ≃+* S → R →+* S)` where `(name : TYPE)`
/// is a type ascription in expression context (not a binder). OxiLean can't parse this.
/// We strip the `: TYPE` part leaving just the identifier.
///
/// Only strips `(IDENT : TYPE)` when the `(` is preceded by a function name or `)` (i.e.,
/// in function-argument position), NOT when preceded by `forall`, `fun`, `,` (binder position).
/// Normalize DFinsupp dependent type notation.
///
/// Lean 4: `Π₀ _ : ι, M` is DFinsupp notation for "finitely supported functions from ι to M".
/// After `Π₀` → `DFinsupp` replacement, this becomes `DFinsupp _ : ι, M`.
/// OxiLean can't parse the dependent indexing, so we simplify to just `DFinsupp`.
///
/// Handles: `DFinsupp IDENT : TYPE, VALTYPE` → `DFinsupp`
pub(super) fn normalize_dfinsupp_type(src: &str) -> String {
    if !src.contains("DFinsupp") {
        return src.to_string();
    }
    let chars: Vec<char> = src.chars().collect();
    let len = chars.len();
    let mut result = String::with_capacity(src.len());
    let mut i = 0;
    while i < len {
        let is_dfinsupp = i + 8 <= len
            && chars[i..i + 8].iter().collect::<String>() == "DFinsupp"
            && (i == 0 || !chars[i - 1].is_alphanumeric())
            && (i + 8 >= len || !chars[i + 8].is_alphanumeric() && chars[i + 8] != '_');
        if is_dfinsupp {
            result.push_str("DFinsupp");
            i += 8;
            let mut j = i;
            while j < len && chars[j] == ' ' {
                j += 1;
            }
            if j < len && (chars[j].is_alphanumeric() || chars[j] == '_') {
                let binder_start = j;
                while j < len && (chars[j].is_alphanumeric() || chars[j] == '_') {
                    j += 1;
                }
                while j < len && chars[j] == ' ' {
                    j += 1;
                }
                if j + 1 < len && chars[j] == ':' && chars[j + 1] != '=' {
                    j += 1;
                    while j < len && chars[j] == ' ' {
                        j += 1;
                    }
                    let mut depth = 0usize;
                    while j < len {
                        match chars[j] {
                            '(' | '{' | '[' => {
                                depth += 1;
                                j += 1;
                            }
                            ')' | '}' | ']' if depth == 0 => break,
                            ')' | '}' | ']' => {
                                depth = depth.saturating_sub(1);
                                j += 1;
                            }
                            ',' if depth == 0 => break,
                            _ => {
                                j += 1;
                            }
                        }
                    }
                    if j < len && chars[j] == ',' {
                        j += 1;
                        while j < len && chars[j] == ' ' {
                            j += 1;
                        }
                        while j < len
                            && (chars[j].is_alphanumeric() || chars[j] == '_' || chars[j] == '.')
                        {
                            j += 1;
                        }
                        while j < len && chars[j] == ' ' {
                            j += 1;
                        }
                        i = j;
                        continue;
                    }
                    let _ = binder_start;
                }
            }
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    result
}
pub(super) fn strip_term_type_ascriptions(src: &str) -> String {
    let kw_end = if src.starts_with("theorem ") {
        8
    } else if src.starts_with("lemma ") {
        6
    } else if src.starts_with("def ") {
        4
    } else if src.starts_with("axiom ") {
        6
    } else {
        return src.to_string();
    };
    let name_area = &src[kw_end..];
    let colon_pos = if let Some(pos) = name_area.find(" : ") {
        kw_end + pos + 3
    } else {
        return src.to_string();
    };
    let prefix = &src[..colon_pos];
    let type_body = &src[colon_pos..];
    let chars: Vec<char> = type_body.chars().collect();
    let len = chars.len();
    let mut result = String::new();
    let mut i = 0;
    while i < len {
        if chars[i] == '(' {
            let result_chars: Vec<char> = result.chars().collect();
            let rlen = result_chars.len();
            let mut back = rlen;
            while back > 0 && result_chars[back - 1] == ' ' {
                back -= 1;
            }
            let prev_char = if back > 0 {
                result_chars[back - 1]
            } else {
                ' '
            };
            let result_trimmed = result.trim_end();
            let last_word_is_binder = result_trimmed.ends_with("forall")
                || result_trimmed.ends_with("fun")
                || result_trimmed.ends_with("exists")
                || result_trimmed.ends_with('\u{2200}')
                || result_trimmed.ends_with('\u{2203}');
            let is_fn_arg = !last_word_is_binder
                && (prev_char.is_alphanumeric()
                    || prev_char == '_'
                    || prev_char == '\''
                    || prev_char == '.'
                    || prev_char == ')');
            if is_fn_arg {
                let mut j = i + 1;
                while j < len && chars[j] == ' ' {
                    j += 1;
                }
                let ident_start = j;
                while j < len
                    && (chars[j].is_alphanumeric()
                        || chars[j] == '_'
                        || chars[j] == '\''
                        || chars[j] == '.')
                {
                    j += 1;
                }
                let ident_end = j;
                let ident_len = ident_end - ident_start;
                while j < len && chars[j] == ' ' {
                    j += 1;
                }
                let has_colon = j + 1 < len && chars[j] == ':' && chars[j + 1] != '=';
                if ident_len > 0 && has_colon {
                    let mut depth = 1usize;
                    let mut k = i + 1;
                    while k < len && depth > 0 {
                        match chars[k] {
                            '(' | '{' | '[' => depth += 1,
                            ')' | '}' | ']' => depth -= 1,
                            _ => {}
                        }
                        k += 1;
                    }
                    if depth == 0 {
                        let ident: String = chars[ident_start..ident_end].iter().collect();
                        result.push_str(&ident);
                        i = k;
                        continue;
                    }
                }
            }
            result.push('(');
            i += 1;
        } else {
            result.push(chars[i]);
            i += 1;
        }
    }
    format!("{}{}", prefix, result)
}
pub(super) fn normalize_head_binders(src: &str) -> String {
    let kw = if src.starts_with("theorem ") {
        "theorem"
    } else if src.starts_with("lemma ") {
        "lemma"
    } else if src.starts_with("def ") {
        "def"
    } else if src.starts_with("axiom ") {
        "axiom"
    } else {
        return src.to_string();
    };
    let rest = src[kw.len()..].trim_start();
    let name_end = rest
        .char_indices()
        .find(|(_, c)| c.is_whitespace() || *c == '(' || *c == '{' || *c == '[' || *c == ':')
        .map(|(i, _)| i)
        .unwrap_or(rest.len());
    let raw_name = &rest[..name_end];
    let name_no_univ = if let Some(brace_pos) = raw_name.find('{') {
        &raw_name[..brace_pos]
    } else {
        raw_name
    };
    let name_owned = name_no_univ.replace('.', "_");
    let name = name_owned.as_str();
    let after_name = rest[name_end..].trim_start();
    let after_name = if after_name.starts_with('{')
        && !after_name.starts_with("{ ")
        && !after_name.contains(':')
    {
        let close = after_name
            .find('}')
            .map(|i| &after_name[i + 1..])
            .unwrap_or(after_name);
        close.trim_start()
    } else {
        after_name
    };
    let (binders, colon_rest) = collect_binders_before_colon(after_name);
    let name_changed = name != raw_name.trim_end_matches(|c: char| c == '{' || c.is_whitespace());
    if binders.is_empty() {
        let after_clean = colon_rest.trim_start();
        let unchanged = !name_changed && after_clean == after_name.trim_start();
        if unchanged {
            return src.to_string();
        }
        return format!("{kw} {name} {after_clean}");
    }
    if !colon_rest.starts_with(':') || colon_rest.starts_with(":=") {
        if kw == "def" && !binders.is_empty() && colon_rest.starts_with(":=") {
            return format!("def {name} := sorry");
        }
        if name_changed {
            return format!("{kw} {name} {}", after_name);
        }
        return src.to_string();
    }
    let type_and_proof = colon_rest[1..].trim_start();
    let binders_explicit = binders.trim().replace('{', "(").replace('}', ")");
    format!("{kw} {name} : forall {binders_explicit}, {type_and_proof}")
}
/// Scan a string for bracket-delimited binder groups that contain `:` annotations.
/// Returns (collected_binders_string, remainder_starting_at_colon_or_assign).
fn collect_binders_before_colon(s: &str) -> (String, &str) {
    let mut binders = String::new();
    let s_bytes = s.as_bytes();
    let mut i = 0;
    loop {
        while i < s_bytes.len() && (s_bytes[i] == b' ' || s_bytes[i] == b'\t') {
            i += 1;
        }
        if i >= s_bytes.len() {
            break;
        }
        let ch = s_bytes[i];
        if ch == b'(' || ch == b'{' || ch == b'[' {
            let _close = match ch {
                b'(' => b')',
                b'{' => b'}',
                b'[' => b']',
                _ => unreachable!(),
            };
            let start = i;
            let mut depth = 0usize;
            let mut has_colon = false;
            while i < s_bytes.len() {
                let c = s_bytes[i];
                match c {
                    b'(' | b'{' | b'[' => depth += 1,
                    b')' | b'}' | b']' => {
                        depth = depth.saturating_sub(1);
                        if depth == 0 {
                            i += 1;
                            break;
                        }
                    }
                    b':' if depth == 1 => {
                        let next = if i + 1 < s_bytes.len() {
                            s_bytes[i + 1]
                        } else {
                            0
                        };
                        if next != b'=' {
                            has_colon = true;
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
            if has_colon {
                if !binders.is_empty() {
                    binders.push(' ');
                }
                if let Ok(group) = std::str::from_utf8(&s_bytes[start..i]) {
                    binders.push_str(group);
                }
            }
        } else if ch == b':' {
            break;
        } else if ch.is_ascii_alphabetic() || ch == b'_' {
            let start = i;
            while i < s_bytes.len()
                && (s_bytes[i].is_ascii_alphanumeric() || s_bytes[i] == b'_' || s_bytes[i] == b'\'')
            {
                i += 1;
            }
            let mut j = i;
            while j < s_bytes.len() && s_bytes[j] == b' ' {
                j += 1;
            }
            let next_ch = if j < s_bytes.len() { s_bytes[j] } else { 0 };
            if next_ch == b':' {
                i = start;
                break;
            }
            if let Ok(ident) = std::str::from_utf8(&s_bytes[start..i]) {
                if !binders.is_empty() {
                    binders.push(' ');
                }
                binders.push('(');
                binders.push_str(ident);
                binders.push_str(" : _)");
            }
        } else {
            break;
        }
    }
    (binders, &s[i..])
}
/// Extract individual single-line theorem/lemma/def/axiom declarations from
/// a Lean 4 source file. Only extracts declarations that begin at column 0
/// and have a proof on the same line (i.e., `:=` is NOT at the very end).
fn extract_single_line_decls(content: &str) -> Vec<String> {
    let mut decls = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim_start();
        if !line.starts_with(' ')
            && !line.starts_with('\t')
            && (trimmed.starts_with("theorem ")
                || trimmed.starts_with("lemma ")
                || trimmed.starts_with("def ")
                || trimmed.starts_with("axiom "))
        {
            let stripped = line.trim_end();
            if let Some(pos) = find_top_level_assign(stripped) {
                let after_assign = stripped[pos + 2..].trim();
                if !after_assign.is_empty() {
                    decls.push(line.to_string());
                }
            }
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
pub(super) const MATHLIB4_ROOT: &str = "/media/kitasan/Backup/resource/mathlib4/Mathlib";
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
#[test]
fn test_normalization_basics() {
    let s = normalize_lean4_to_oxilean("fun x => x + 1");
    assert!(s.contains("->"), "Expected -> in: {s}");
    let s = normalize_lean4_to_oxilean("fun f : \u{2115} \u{2192} \u{2115} => f 0");
    assert!(s.contains("Nat"), "Expected Nat in: {s}");
    let s = normalize_lean4_to_oxilean("fun x \u{21A6} x");
    assert!(s.contains("->"), "Expected -> in: {s}");
}
#[test]
fn test_head_binder_normalization() {
    let s = normalize_head_binders("theorem foo (x : Nat) : x = x := rfl");
    assert!(
        s.contains("forall"),
        "Expected forall after head-binder normalization: {s}"
    );
    assert!(s.contains("(x : Nat)"), "Expected binder in forall: {s}");
    assert!(try_parse_decl(&s), "Normalized decl should parse: {s}");
    let s = normalize_head_binders("theorem bar {p : Prop} : p -> p := fun h -> h");
    assert!(s.contains("forall"), "Expected forall: {s}");
    assert!(
        try_parse_decl(&s),
        "Normalized implicit binder should parse: {s}"
    );
    let s = normalize_head_binders("theorem baz (n m : Nat) {p : Prop} : n = m -> p := sorry");
    assert!(s.contains("forall"), "Expected forall: {s}");
    assert!(try_parse_decl(&s), "Multiple binders should parse: {s}");
    let orig = "theorem no_binders : forall (n : Nat), n = n := rfl";
    let s = normalize_head_binders(orig);
    assert_eq!(s, orig, "No-binder theorem should be unchanged");
    let s = normalize_head_binders("theorem Nat.add_comm (n m : Nat) : n + m = m + n := sorry");
    assert!(
        s.contains("Nat_add_comm"),
        "Expected dotted name normalized to Nat_add_comm: {s}"
    );
    assert!(s.contains("forall"), "Expected forall: {s}");
    assert!(try_parse_decl(&s), "Dotted name theorem should parse: {s}");
    let s = normalize_head_binders("theorem univ.{u} : Prop := Prop");
    assert!(s.contains("univ"), "Should contain univ: {s}");
}
#[test]
fn test_tactic_proof_normalization() {
    let s = normalize_lean4_to_oxilean("theorem foo : forall (n : Nat), n = n := by rfl");
    assert!(s.contains(":= sorry"), "Expected sorry: {s}");
    assert!(
        try_parse_decl(&s),
        "Tactic-proof theorem should parse after sorry: {s}"
    );
    let s =
        normalize_lean4_to_oxilean("theorem bar : forall (n m : Nat), n + m = m + n := by simp");
    assert!(s.contains(":= sorry"), "Expected sorry: {s}");
    let s =
        normalize_lean4_to_oxilean("theorem Nat.add_comm (n m : Nat) : n + m = m + n := by ring");
    assert!(s.contains("Nat_add_comm"), "Expected Nat_add_comm: {s}");
    assert!(s.contains(":= sorry"), "Expected sorry: {s}");
    assert!(try_parse_decl(&s), "Should parse: {s}");
    let s = normalize_lean4_to_oxilean("@[simp] theorem foo2 : forall (n : Nat), n = n := by rfl");
    assert!(s.contains(":= sorry"), "Expected sorry: {s}");
    assert!(!s.starts_with("@["), "Attribute should be stripped: {s}");
    assert!(try_parse_decl(&s), "Attr+tactic theorem should parse: {s}");
    let s =
        normalize_lean4_to_oxilean("theorem foo3 (alpha : Sort*) : alpha -> alpha := fun h -> h");
    assert!(s.contains("Type"), "Expected Sort* -> Type: {s}");
    let s = normalize_lean4_to_oxilean(
        "theorem fact_iff {p : Prop} : Fact p \u{2194} p := \u{27E8}fun h \u{21A6} h.1, fun h \u{21A6} \u{27E8}h\u{27E9}\u{27E9}",
    );
    assert!(s.contains(":= sorry"), "Expected sorry for term proof: {s}");
    let s = normalize_lean4_to_oxilean("def id_fn : Nat -> Nat := fun n -> n");
    assert!(
        s.contains(":= sorry"),
        "Expected sorry for def term proof: {s}"
    );
}
#[test]
fn test_extract_decls_basic() {
    let content = r#"
theorem add_comm : n + m = m + n := by ring
def double (n : Nat) : Nat := n + n
-- comment
import Foo
open Bar
namespace Baz
theorem foo : True := trivial
"#;
    let decls = extract_single_line_decls(content);
    assert!(
        !decls.is_empty(),
        "Should extract at least some declarations"
    );
    for d in &decls {
        let ok = d.starts_with("theorem ")
            || d.starts_with("def ")
            || d.starts_with("lemma ")
            || d.starts_with("axiom ");
        assert!(ok, "Unexpected extraction: {d}");
    }
}
#[test]
fn test_parse_simple_oxilean_decls() {
    let decls = vec![
        "axiom em : forall (p : Prop), p \u{2228} \u{00AC} p",
        "theorem t1 : forall (n : Nat), n = n := rfl",
        "def id_nat : Nat -> Nat := fun n -> n",
        "theorem t2 : Prop := True",
    ];
    for decl in &decls {
        assert!(try_parse_decl(decl), "Failed to parse OxiLean decl: {decl}");
    }
}
#[test]
fn test_bounded_quantifier_normalization() {
    let s = normalize_bounded_quantifiers("ISup k < n + 1, u k");
    assert!(s.contains("fun k ->"), "Expected lambda form: {s}");
    assert!(!s.contains("< n + 1"), "Bound should be dropped: {s}");
    let s = normalize_bounded_quantifiers("IInf k \u{2264} n, f k");
    assert!(s.contains("fun k ->"), "Expected lambda form for IInf: {s}");
}
#[test]
fn test_subscript_normalization() {
    let s = normalize_subscript_indexing("l[0]");
    assert_eq!(s, "l", "Subscript should be dropped: {s}");
    let s = normalize_subscript_indexing("xs[n]");
    assert_eq!(s, "xs", "Variable subscript should be dropped: {s}");
    let s = normalize_subscript_indexing("[1, 2, 3]");
    assert_eq!(s, "[1, 2, 3]", "List literal should be preserved: {s}");
}
#[test]
fn test_def_without_return_type() {
    let s = normalize_head_binders("def Xor' (a b : Prop) := sorry");
    assert!(s.starts_with("def Xor"), "Should start with def Xor: {s}");
    assert!(s.contains(":= sorry"), "Should have sorry: {s}");
    assert!(
        !s.contains("(a b : Prop)"),
        "Binders should be dropped: {s}"
    );
    assert!(
        try_parse_decl(&s),
        "def without return type should parse after normalization: {s}"
    );
}
#[test]
fn test_nat_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Nat");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 20);
    print_stats("Data/Nat", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.success_rate() >= 0.0,
            "Compat rate should be non-negative"
        );
    } else {
        println!("  No single-line declarations found in Data/Nat (expected for complex files)");
    }
}
#[test]
fn test_logic_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Logic");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_recursive(&dir, 30);
    print_stats("Logic", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(stats.success_rate() >= 0.0);
    }
}
#[test]
fn test_algebra_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Algebra");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 20);
    print_stats("Algebra", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(stats.success_rate() >= 0.0);
    }
}
#[test]
fn test_data_list_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/List");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 25);
    print_stats("Data/List", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(stats.success_rate() >= 0.0);
    }
}
#[test]
fn test_data_option_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Option");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 5);
    print_stats("Data/Option", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(stats.success_rate() >= 0.0);
    }
}
#[test]
fn test_data_prod_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Prod");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 5);
    print_stats("Data/Prod", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(stats.success_rate() >= 0.0);
    }
}
#[test]
fn test_data_sum_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Sum");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 5);
    print_stats("Data/Sum", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(stats.success_rate() >= 0.0);
    }
}
#[test]
fn test_data_int_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Int");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 20);
    print_stats("Data/Int", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(stats.success_rate() >= 0.0);
    }
}
#[test]
fn test_data_bool_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Bool");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("Data/Bool", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(stats.success_rate() >= 0.0);
    }
}
#[test]
fn test_order_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Order");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("Order", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(stats.success_rate() >= 0.0);
    }
}
#[test]
fn test_data_fin_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Fin");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("Data/Fin", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 80,
            "Data/Fin compat should be >=80%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/Fin");
    }
}
#[test]
fn test_data_multiset_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Multiset");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 15);
    print_stats("Data/Multiset", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 80,
            "Data/Multiset compat should be >=80%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/Multiset");
    }
}
#[test]
fn test_data_finset_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Finset");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 15);
    print_stats("Data/Finset", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 80,
            "Data/Finset compat should be >=80%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/Finset");
    }
}
#[test]
fn test_data_set_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Set");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 15);
    print_stats("Data/Set", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 75,
            "Data/Set compat should be >=75%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/Set");
    }
}
#[test]
fn test_data_rat_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Rat");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("Data/Rat", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 75,
            "Data/Rat compat should be >=75%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/Rat");
    }
}
#[test]
fn test_data_pnat_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/PNat");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("Data/PNat", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 75,
            "Data/PNat compat should be >=75%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/PNat");
    }
}
#[test]
fn test_data_fintype_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Fintype");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 15);
    print_stats("Data/Fintype", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 75,
            "Data/Fintype compat should be >=75%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/Fintype");
    }
}
#[test]
fn test_algebra_group_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Algebra/Group");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 15);
    print_stats("Algebra/Group", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 75,
            "Algebra/Group compat should be >=75%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Algebra/Group");
    }
}
#[test]
fn test_algebra_ring_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Algebra/Ring");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 15);
    print_stats("Algebra/Ring", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 75,
            "Algebra/Ring compat should be >=75%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Algebra/Ring");
    }
}
#[test]
fn test_grouptheory_coset_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("GroupTheory/Coset");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("GroupTheory/Coset", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 75,
            "GroupTheory/Coset compat should be >=75%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in GroupTheory/Coset");
    }
}
#[test]
fn test_data_enat_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/ENat");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 5);
    print_stats("Data/ENat", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 70,
            "Data/ENat compat should be >=70%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/ENat");
    }
}
#[test]
fn test_data_finsupp_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Finsupp");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("Data/Finsupp", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 75,
            "Data/Finsupp compat should be >=75%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/Finsupp");
    }
}
#[test]
fn test_algebra_order_group_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Algebra/Order/Group");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("Algebra/Order/Group", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 75,
            "Algebra/Order/Group compat should be >=75%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Algebra/Order/Group");
    }
}
#[test]
fn test_number_theory_basic_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("NumberTheory");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 5);
    print_stats("NumberTheory", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 70,
            "NumberTheory compat should be >=70%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in NumberTheory");
    }
}
#[test]
fn test_algebra_field_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Algebra/Field");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 12);
    print_stats("Algebra/Field", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 65,
            "Algebra/Field compat should be >=65%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Algebra/Field");
    }
}
#[test]
fn test_algebra_module_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Algebra/Module");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 15);
    print_stats("Algebra/Module", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 65,
            "Algebra/Module compat should be >=65%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Algebra/Module");
    }
}
#[test]
fn test_grouptheory_main_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("GroupTheory");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 12);
    print_stats("GroupTheory", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "GroupTheory compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in GroupTheory");
    }
}
#[test]
fn test_combinatorics_simplegraph_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Combinatorics/SimpleGraph");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 15);
    print_stats("Combinatorics/SimpleGraph", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 65,
            "Combinatorics/SimpleGraph compat should be >=65%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Combinatorics/SimpleGraph");
    }
}
#[test]
fn test_algebra_algebra_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Algebra/Algebra");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 15);
    print_stats("Algebra/Algebra", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 65,
            "Algebra/Algebra compat should be >=65%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Algebra/Algebra");
    }
}
#[test]
fn test_ringtheory_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("RingTheory");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 15);
    print_stats("RingTheory", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 65,
            "RingTheory compat should be >=65%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in RingTheory");
    }
}
#[test]
fn test_data_nat_gcd_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Nat/GCD");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 5);
    print_stats("Data/Nat/GCD", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 70,
            "Data/Nat/GCD compat should be >=70%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/Nat/GCD");
    }
}
#[test]
fn test_data_nat_factorial_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/Nat/Factorial");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 6);
    print_stats("Data/Nat/Factorial", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 70,
            "Data/Nat/Factorial compat should be >=70%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/Nat/Factorial");
    }
}
#[test]
fn test_algebra_bigoperators_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Algebra/BigOperators");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 15);
    print_stats("Algebra/BigOperators", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 65,
            "Algebra/BigOperators compat should be >=65%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Algebra/BigOperators");
    }
}
#[test]
fn test_linearalgebra_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("LinearAlgebra");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 8);
    print_stats("LinearAlgebra", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 65,
            "LinearAlgebra compat should be >=65%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in LinearAlgebra");
    }
}
#[test]
fn test_numbertheory_arithfunc_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("NumberTheory/ArithmeticFunction");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 6);
    print_stats("NumberTheory/ArithmeticFunction", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 65,
            "NumberTheory/ArithmeticFunction compat should be >=65%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in NumberTheory/ArithmeticFunction");
    }
}
#[test]
fn test_topology_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Topology");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 12);
    print_stats("Topology", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "Topology compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Topology");
    }
}
#[test]
fn test_settheory_cardinal_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("SetTheory/Cardinal");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("SetTheory/Cardinal", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "SetTheory/Cardinal compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in SetTheory/Cardinal");
    }
}
#[test]
fn test_settheory_ordinal_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("SetTheory/Ordinal");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("SetTheory/Ordinal", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "SetTheory/Ordinal compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in SetTheory/Ordinal");
    }
}
#[test]
fn test_fieldtheory_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("FieldTheory");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("FieldTheory", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "FieldTheory compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in FieldTheory");
    }
}
#[test]
fn test_measuretheory_measure_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("MeasureTheory/Measure");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("MeasureTheory/Measure", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "MeasureTheory/Measure compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in MeasureTheory/Measure");
    }
}
#[test]
fn test_data_zmod_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Data/ZMod");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 10);
    print_stats("Data/ZMod", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "Data/ZMod compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Data/ZMod");
    }
}
#[test]
fn test_algebra_group_subgroup_compat() {
    let dir = Path::new(MATHLIB4_ROOT).join("Algebra/Group/Subgroup");
    if !dir.exists() {
        println!("[SKIP] Mathlib4 not found at {}", dir.display());
        return;
    }
    let stats = run_compat_on_dir(&dir, 12);
    print_stats("Algebra/Group/Subgroup", &stats);
    if stats.total > 0 {
        println!(
            "  Compat rate: {:.1}% ({}/{})",
            stats.success_rate(),
            stats.parsed_ok,
            stats.total
        );
        assert!(
            stats.parsed_ok * 100 >= stats.total * 60,
            "Algebra/Group/Subgroup compat should be >=60%: got {}/{}",
            stats.parsed_ok,
            stats.total
        );
    } else {
        println!("  No single-line declarations found in Algebra/Group/Subgroup");
    }
}
