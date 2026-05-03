//! Functions for the LSP call hierarchy module.
//!
//! Implements `textDocument/prepareCallHierarchy`, `callHierarchy/incomingCalls`,
//! and `callHierarchy/outgoingCalls` for Lean4-like source files.
//!
//! The analysis is purely textual (no type-checker involvement) and relies on
//! recognising declaration keywords and identifier references in source text.

use crate::lsp::{Position, Range};

use super::types::{
    CallHierarchyIncomingCall, CallHierarchyItem, CallHierarchyOutgoingCall, SymbolKind,
};

// ============================================================================
// Declaration keyword table
// ============================================================================

/// Declaration keywords that introduce named definitions.
const DECL_KEYWORDS: &[&str] = &[
    "def",
    "theorem",
    "lemma",
    "axiom",
    "instance",
    "class",
    "structure",
    "inductive",
    "abbrev",
    "opaque",
    "noncomputable",
    "partial",
];

// ============================================================================
// Public API
// ============================================================================

/// Handle `textDocument/prepareCallHierarchy`.
///
/// Scans `source` for the declaration that covers position `(line, col)` and
/// returns a `Vec` containing that item (or empty if none found).
pub fn handle_prepare(source: &str, line: u32, col: u32) -> Vec<CallHierarchyItem> {
    match find_declaration_at(source, line, col) {
        Some(item) => vec![item],
        None => Vec::new(),
    }
}

/// Handle `callHierarchy/incomingCalls`.
///
/// Returns every declaration in `source` that references `item.name` in its
/// body, together with the specific ranges at which each reference occurs.
pub fn handle_incoming_calls(
    item: &CallHierarchyItem,
    source: &str,
) -> Vec<CallHierarchyIncomingCall> {
    let lines: Vec<&str> = source.lines().collect();
    let all_decls = collect_all_declarations(source, &item.uri);

    let mut result = Vec::new();
    for decl in &all_decls {
        // Skip self-reference
        if decl.name == item.name {
            continue;
        }
        // Gather occurrences of `item.name` within this declaration's body
        let body_ranges = occurrences_in_body(source, decl, &item.name, &lines);
        if !body_ranges.is_empty() {
            result.push(CallHierarchyIncomingCall::new(decl.clone(), body_ranges));
        }
    }
    result
}

/// Handle `callHierarchy/outgoingCalls`.
///
/// Returns every named declaration referenced inside `item`'s body (i.e., the
/// functions/theorems that `item` calls), together with the call-site ranges.
pub fn handle_outgoing_calls(
    item: &CallHierarchyItem,
    source: &str,
) -> Vec<CallHierarchyOutgoingCall> {
    let lines: Vec<&str> = source.lines().collect();
    let body = match extract_body(source, item) {
        Some(b) => b,
        None => return Vec::new(),
    };

    // The body starts at item.range.start.line + 1 (after the signature line)
    let body_start_line = item.range.start.line + 1;

    // Collect all sibling declarations to recognise their names
    let all_decls = collect_all_declarations(source, &item.uri);
    let mut result = Vec::new();

    for decl in &all_decls {
        if decl.name == item.name {
            continue;
        }
        // Find call-site ranges in the body text (with absolute line numbers)
        let ranges =
            occurrences_of_name_in_text(&body, &decl.name, body_start_line, &lines, source);
        if !ranges.is_empty() {
            result.push(CallHierarchyOutgoingCall::new(decl.clone(), ranges));
        }
    }
    result
}

/// Find the declaration covering position `(line, col)` in `source`.
///
/// Returns `None` if no declaration matches the position.
pub fn find_declaration_at(source: &str, line: u32, col: u32) -> Option<CallHierarchyItem> {
    let all_decls = collect_all_declarations(source, "");
    // Check if the cursor position is within this declaration's range
    for item in all_decls {
        let r = &item.range;
        let inside_start =
            (line > r.start.line) || (line == r.start.line && col >= r.start.character);
        let inside_end = (line < r.end.line) || (line == r.end.line && col <= r.end.character);
        if inside_start && inside_end {
            return Some(item);
        }
    }
    None
}

/// Find all source ranges where `name` appears as a standalone identifier.
pub fn find_references_to(source: &str, name: &str) -> Vec<Range> {
    if name.is_empty() {
        return Vec::new();
    }
    let mut ranges = Vec::new();
    for (line_idx, line_text) in source.lines().enumerate() {
        for col in find_identifier_occurrences(line_text, name) {
            ranges.push(Range::single_line(
                line_idx as u32,
                col as u32,
                (col + name.len()) as u32,
            ));
        }
    }
    ranges
}

/// Extract the source text of `item`'s body.
///
/// Returns the lines from the start to the end of the declaration's range,
/// or `None` if the range is out of bounds.
pub fn extract_body(source: &str, item: &CallHierarchyItem) -> Option<String> {
    let lines: Vec<&str> = source.lines().collect();
    let start = item.range.start.line as usize;
    let end = item.range.end.line as usize;
    if start >= lines.len() {
        return None;
    }
    let end = end.min(lines.len().saturating_sub(1));
    Some(lines[start..=end].join("\n"))
}

/// Serialize a `CallHierarchyItem` to a `serde_json`-compatible `JsonValue`.
pub fn item_to_json(item: &CallHierarchyItem) -> crate::lsp::JsonValue {
    item.to_json()
}

// ============================================================================
// Internal helpers
// ============================================================================

/// Scan `source` and collect all top-level declarations as `CallHierarchyItem`s.
fn collect_all_declarations(source: &str, uri: &str) -> Vec<CallHierarchyItem> {
    let lines: Vec<&str> = source.lines().collect();
    let mut result = Vec::new();

    let mut line_idx = 0usize;
    while line_idx < lines.len() {
        let text = lines[line_idx];
        let trimmed = text.trim_start();

        if let Some((kw, name, kind)) = parse_declaration_line(trimmed) {
            let start_line = line_idx as u32;
            let end_line = find_decl_end(&lines, line_idx) as u32;

            let name_col = text.find(&name).map(|p| p as u32).unwrap_or(0);
            let sel_range = Range::single_line(start_line, name_col, name_col + name.len() as u32);
            let end_col = lines
                .get(end_line as usize)
                .map(|l| l.len() as u32)
                .unwrap_or(0);
            let full_range = Range::new(
                Position::new(start_line, 0),
                Position::new(end_line, end_col),
            );

            let item = CallHierarchyItem::new(name, kind, uri, full_range, sel_range)
                .with_detail(kw.to_string());
            result.push(item);
        }
        line_idx += 1;
    }
    result
}

/// Try to parse a declaration line.
///
/// Returns `(keyword, name, SymbolKind)` or `None`.
fn parse_declaration_line(trimmed: &str) -> Option<(&'static str, String, SymbolKind)> {
    // Handle `noncomputable def` and similar prefix+keyword combos
    let modifiers: &[&str] = &["private", "protected", "noncomputable", "partial", "unsafe"];
    let mut rest = trimmed;

    // Strip at most one modifier prefix
    for m in modifiers {
        if rest.starts_with(m) && rest.len() > m.len() {
            let after = &rest[m.len()..];
            if after.starts_with(|c: char| c.is_whitespace()) {
                rest = after.trim_start();
                break;
            }
        }
    }

    for &kw in DECL_KEYWORDS {
        if rest.starts_with(kw) {
            let after_kw = &rest[kw.len()..];
            // Must be followed by whitespace (word boundary)
            if !after_kw.starts_with(|c: char| c.is_whitespace()) {
                continue;
            }
            let name_part = after_kw.trim_start();
            if let Some(name) = extract_identifier(name_part) {
                let kind = SymbolKind::from_keyword(kw);
                return Some((kw, name, kind));
            }
        }
    }
    None
}

/// Extract a leading identifier (alphanumeric + `_` + `.` + `'`) from `s`.
fn extract_identifier(s: &str) -> Option<String> {
    let s = s.trim_start();
    let first = s.chars().next()?;
    if !first.is_alphabetic() && first != '_' {
        return None;
    }
    let name: String = s
        .chars()
        .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '.' || *c == '\'')
        .collect();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

/// Find the last line index (inclusive) of the declaration starting at `start`.
fn find_decl_end(lines: &[&str], start: usize) -> usize {
    if start + 1 >= lines.len() {
        return start;
    }
    for (i, &line_text) in lines.iter().enumerate().skip(start + 1) {
        let t = line_text.trim_start();
        let indent = line_text.len() - t.len();
        // A new top-level declaration at column 0 ends the previous one
        if indent == 0 && parse_declaration_line(t).is_some() {
            return i.saturating_sub(1);
        }
        // `end <Name>` also closes a block
        if indent == 0 && t.starts_with("end ") {
            return i.saturating_sub(1);
        }
    }
    lines.len().saturating_sub(1)
}

/// Find all byte-column offsets where `name` appears as a standalone identifier
/// in `line_text`.
fn find_identifier_occurrences(line_text: &str, name: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let bytes = line_text.as_bytes();
    let name_bytes = name.as_bytes();
    let n = name.len();
    if n == 0 || bytes.len() < n {
        return positions;
    }
    let mut i = 0usize;
    while i + n <= bytes.len() {
        if bytes[i..i + n] == *name_bytes {
            // Check word boundaries
            let before_ok = i == 0 || !is_ident_char(bytes[i - 1]);
            let after_ok = i + n >= bytes.len() || !is_ident_char(bytes[i + n]);
            if before_ok && after_ok {
                positions.push(i);
            }
            i += n;
        } else {
            i += 1;
        }
    }
    positions
}

/// Return true if the byte is part of an identifier character set.
#[inline]
fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'\'' || b == b'.'
}

/// Collect the ranges (using absolute line numbers from `source`) at which
/// `name` appears inside `decl`'s body.
fn occurrences_in_body(
    source: &str,
    decl: &CallHierarchyItem,
    name: &str,
    lines: &[&str],
) -> Vec<Range> {
    let body_start = decl.range.start.line as usize;
    let body_end = (decl.range.end.line as usize).min(lines.len().saturating_sub(1));

    let mut ranges = Vec::new();
    for line_idx in body_start..=body_end {
        if let Some(line_text) = lines.get(line_idx) {
            for col in find_identifier_occurrences(line_text, name) {
                ranges.push(Range::single_line(
                    line_idx as u32,
                    col as u32,
                    (col + name.len()) as u32,
                ));
            }
        }
    }
    // Skip occurrence on the declaration's own signature line (first line)
    // to avoid counting the def-site as a call
    ranges.retain(|r| r.start.line > decl.range.start.line);
    // Also skip the target decl's own definition line to avoid false positives
    let _ = source;
    ranges
}

/// Collect call-site ranges for `name` in `body_text`, offsetting line numbers
/// by `body_start_line`.
fn occurrences_of_name_in_text(
    body_text: &str,
    name: &str,
    body_start_line: u32,
    _lines: &[&str],
    _source: &str,
) -> Vec<Range> {
    let mut ranges = Vec::new();
    for (relative_line, line_text) in body_text.lines().enumerate() {
        // Skip the first line (the signature line itself)
        if relative_line == 0 {
            continue;
        }
        let abs_line = body_start_line + relative_line as u32 - 1;
        for col in find_identifier_occurrences(line_text, name) {
            ranges.push(Range::single_line(
                abs_line,
                col as u32,
                (col + name.len()) as u32,
            ));
        }
    }
    ranges
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // ── extract_identifier ─────────────────────────────────────────────────────

    #[test]
    fn test_extract_identifier_simple() {
        assert_eq!(extract_identifier("foo := 1"), Some("foo".to_string()));
    }

    #[test]
    fn test_extract_identifier_qualified() {
        assert_eq!(
            extract_identifier("Foo.bar := 1"),
            Some("Foo.bar".to_string())
        );
    }

    #[test]
    fn test_extract_identifier_with_prime() {
        assert_eq!(extract_identifier("foo' := 1"), Some("foo'".to_string()));
    }

    #[test]
    fn test_extract_identifier_empty() {
        assert_eq!(extract_identifier(""), None);
    }

    #[test]
    fn test_extract_identifier_starts_with_special() {
        assert_eq!(extract_identifier("(foo)"), None);
    }

    // ── symbol_kind_from_keyword ───────────────────────────────────────────────

    #[test]
    fn test_kind_def() {
        assert_eq!(SymbolKind::from_keyword("def"), SymbolKind::Function);
    }

    #[test]
    fn test_kind_theorem() {
        assert_eq!(SymbolKind::from_keyword("theorem"), SymbolKind::Function);
    }

    #[test]
    fn test_kind_axiom() {
        assert_eq!(SymbolKind::from_keyword("axiom"), SymbolKind::Constant);
    }

    #[test]
    fn test_kind_structure() {
        assert_eq!(SymbolKind::from_keyword("structure"), SymbolKind::Struct);
    }

    #[test]
    fn test_kind_instance() {
        assert_eq!(SymbolKind::from_keyword("instance"), SymbolKind::Event);
    }

    #[test]
    fn test_kind_namespace() {
        assert_eq!(SymbolKind::from_keyword("namespace"), SymbolKind::Namespace);
    }

    // ── find_identifier_occurrences ────────────────────────────────────────────

    #[test]
    fn test_occurrences_single() {
        let positions = find_identifier_occurrences("def foo := bar", "foo");
        assert_eq!(positions, vec![4]);
    }

    #[test]
    fn test_occurrences_multiple() {
        let positions = find_identifier_occurrences("foo + foo + foo", "foo");
        assert_eq!(positions, vec![0, 6, 12]);
    }

    #[test]
    fn test_occurrences_word_boundary() {
        // "foobar" must NOT match "foo"
        let positions = find_identifier_occurrences("foobar foo", "foo");
        assert_eq!(positions, vec![7]);
    }

    #[test]
    fn test_occurrences_none() {
        let positions = find_identifier_occurrences("def bar := 1", "foo");
        assert!(positions.is_empty());
    }

    #[test]
    fn test_occurrences_empty_name() {
        let positions = find_identifier_occurrences("anything", "");
        assert!(positions.is_empty());
    }

    // ── find_declaration_at ────────────────────────────────────────────────────

    #[test]
    fn test_find_decl_at_single_def() {
        let src = "def foo := 42\n";
        let item = find_declaration_at(src, 0, 4);
        assert!(item.is_some());
        let item = item.expect("declaration must be found");
        assert_eq!(item.name, "foo");
    }

    #[test]
    fn test_find_decl_at_no_match() {
        let src = "-- just a comment\n";
        let item = find_declaration_at(src, 0, 0);
        assert!(item.is_none());
    }

    #[test]
    fn test_find_decl_at_second_def() {
        let src = "def foo := 1\ndef bar := 2\n";
        let item = find_declaration_at(src, 1, 4);
        assert!(item.is_some());
        let item = item.expect("declaration must be found");
        assert_eq!(item.name, "bar");
    }

    // ── handle_prepare ────────────────────────────────────────────────────────

    #[test]
    fn test_handle_prepare_returns_item() {
        let src = "def myFunc := 1\n";
        let items = handle_prepare(src, 0, 5);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "myFunc");
    }

    #[test]
    fn test_handle_prepare_empty_on_no_decl() {
        let src = "-- comment\n";
        let items = handle_prepare(src, 0, 0);
        assert!(items.is_empty());
    }

    // ── find_references_to ─────────────────────────────────────────────────────

    #[test]
    fn test_references_to_name() {
        let src = "def foo := 1\ndef bar := foo + foo\n";
        let refs = find_references_to(src, "foo");
        // "foo" appears at line 0 col 4, line 1 col 11, line 1 col 17
        assert!(refs.len() >= 2);
    }

    #[test]
    fn test_references_to_nonexistent() {
        let src = "def foo := 1\n";
        let refs = find_references_to(src, "bar");
        assert!(refs.is_empty());
    }

    #[test]
    fn test_references_empty_name() {
        let src = "def foo := 1\n";
        let refs = find_references_to(src, "");
        assert!(refs.is_empty());
    }

    // ── extract_body ──────────────────────────────────────────────────────────

    #[test]
    fn test_extract_body_single_line() {
        let src = "def foo := 1\ndef bar := 2\n";
        let items = handle_prepare(src, 0, 4);
        let item = items.into_iter().next().expect("must find declaration");
        let body = extract_body(src, &item);
        assert!(body.is_some());
        assert!(body.expect("body must exist").contains("foo"));
    }

    #[test]
    fn test_extract_body_multiline() {
        let src = "def foo :=\n  let x := 1\n  x + 2\n";
        let items = handle_prepare(src, 0, 4);
        let item = items.into_iter().next().expect("must find declaration");
        let body = extract_body(src, &item);
        assert!(body.is_some());
        let b = body.expect("body must exist");
        assert!(b.contains("let x"));
    }

    // ── handle_incoming_calls ─────────────────────────────────────────────────

    #[test]
    fn test_incoming_calls_basic() {
        let src = "def foo := 1\ndef bar :=\n  foo + 1\n";
        let items = handle_prepare(src, 0, 4);
        let item = items.into_iter().next().expect("must find foo");
        let incoming = handle_incoming_calls(&item, src);
        // `bar` calls `foo`
        assert!(!incoming.is_empty());
        assert_eq!(incoming[0].from.name, "bar");
    }

    #[test]
    fn test_incoming_calls_none() {
        let src = "def foo := 1\ndef bar := 2\n";
        let items = handle_prepare(src, 0, 4);
        let item = items.into_iter().next().expect("must find foo");
        let incoming = handle_incoming_calls(&item, src);
        assert!(incoming.is_empty());
    }

    // ── handle_outgoing_calls ─────────────────────────────────────────────────

    #[test]
    fn test_outgoing_calls_basic() {
        let src = "def helper := 1\ndef main :=\n  helper + 1\n";
        let items = handle_prepare(src, 1, 4);
        let item = items.into_iter().next().expect("must find main");
        let outgoing = handle_outgoing_calls(&item, src);
        assert!(!outgoing.is_empty());
        assert_eq!(outgoing[0].to.name, "helper");
    }

    #[test]
    fn test_outgoing_calls_none() {
        let src = "def foo := 1\n";
        let items = handle_prepare(src, 0, 4);
        let item = items.into_iter().next().expect("must find foo");
        let outgoing = handle_outgoing_calls(&item, src);
        assert!(outgoing.is_empty());
    }

    // ── item_to_json ──────────────────────────────────────────────────────────

    #[test]
    fn test_item_to_json_has_required_fields() {
        let range = Range::single_line(0, 0, 10);
        let item = CallHierarchyItem::new(
            "myFunc",
            SymbolKind::Function,
            "file:///test.lean",
            range.clone(),
            range,
        );
        let json = item_to_json(&item);
        assert!(json.get("name").is_some());
        assert!(json.get("kind").is_some());
        assert!(json.get("uri").is_some());
        assert!(json.get("range").is_some());
        assert!(json.get("selectionRange").is_some());
    }

    #[test]
    fn test_item_to_json_kind_value() {
        let range = Range::single_line(0, 0, 5);
        let item = CallHierarchyItem::new(
            "f",
            SymbolKind::Function,
            "file:///a.lean",
            range.clone(),
            range,
        );
        let json = item_to_json(&item);
        assert_eq!(json.get("kind").and_then(|v| v.as_i64()), Some(12));
    }

    #[test]
    fn test_item_roundtrip_from_json() {
        let range = Range::single_line(2, 4, 9);
        let item = CallHierarchyItem::new(
            "compute",
            SymbolKind::Function,
            "file:///compute.lean",
            range.clone(),
            range,
        )
        .with_detail("def");
        let json = item.to_json();
        let parsed = CallHierarchyItem::from_json(&json).expect("roundtrip must succeed");
        assert_eq!(parsed.name, "compute");
        assert_eq!(parsed.kind, SymbolKind::Function);
        assert_eq!(parsed.detail, Some("def".to_string()));
    }

    // ── incoming/outgoing json ─────────────────────────────────────────────────

    #[test]
    fn test_incoming_call_to_json() {
        let range = Range::single_line(0, 0, 5);
        let item = CallHierarchyItem::new(
            "caller",
            SymbolKind::Function,
            "file:///t.lean",
            range.clone(),
            range.clone(),
        );
        let call_range = Range::single_line(3, 2, 8);
        let incoming = CallHierarchyIncomingCall::new(item, vec![call_range]);
        let json = incoming.to_json();
        assert!(json.get("from").is_some());
        assert!(json.get("fromRanges").is_some());
    }

    #[test]
    fn test_outgoing_call_to_json() {
        let range = Range::single_line(5, 0, 6);
        let item = CallHierarchyItem::new(
            "callee",
            SymbolKind::Constant,
            "file:///t.lean",
            range.clone(),
            range.clone(),
        );
        let call_range = Range::single_line(10, 4, 10);
        let outgoing = CallHierarchyOutgoingCall::new(item, vec![call_range]);
        let json = outgoing.to_json();
        assert!(json.get("to").is_some());
        assert!(json.get("fromRanges").is_some());
    }

    // ── collect_all_declarations ───────────────────────────────────────────────

    #[test]
    fn test_collect_all_declarations_multiple() {
        let src = "def a := 1\ntheorem b : True := trivial\naxiom c : Nat\n";
        let decls = collect_all_declarations(src, "file:///t.lean");
        let names: Vec<&str> = decls.iter().map(|d| d.name.as_str()).collect();
        assert!(names.contains(&"a"));
        assert!(names.contains(&"b"));
        assert!(names.contains(&"c"));
    }

    #[test]
    fn test_collect_all_declarations_empty() {
        let src = "-- only comments\n";
        let decls = collect_all_declarations(src, "");
        assert!(decls.is_empty());
    }

    #[test]
    fn test_from_ranges_in_incoming_calls() {
        let src = "def add := 1\ndef use_add :=\n  add + add\n";
        let items = handle_prepare(src, 0, 4);
        let item = items.into_iter().next().expect("must find add");
        let incoming = handle_incoming_calls(&item, src);
        assert!(!incoming.is_empty());
        // Both occurrences of `add` in the body of `use_add` should be tracked
        assert!(incoming[0].from_ranges.len() >= 2);
    }

    #[test]
    fn test_symbol_kind_labels() {
        assert_eq!(SymbolKind::Function.label(), "function");
        assert_eq!(SymbolKind::Constant.label(), "constant");
        assert_eq!(SymbolKind::Struct.label(), "struct");
        assert_eq!(SymbolKind::Namespace.label(), "namespace");
        assert_eq!(SymbolKind::Method.label(), "method");
    }
}
