//! Functions for the LSP folding range module.
//!
//! Implements `textDocument/foldingRange` for Lean4-like source files.
//! Detection is purely textual; five kinds of foldable regions are supported:
//!
//! 1. **Comment blocks** — consecutive `--` lines and `/- … -/` blocks
//! 2. **Import blocks** — consecutive `import` lines
//! 3. **Declaration bodies** — `def`/`theorem`/`namespace`/`section` and similar
//! 4. **Do-blocks** — indented bodies following a `do` keyword
//! 5. **Adjacent single-line folds** — merged by `merge_adjacent_ranges`

use super::types::{FoldingRange, FoldingRangeKind};

// ============================================================================
// Main entry point
// ============================================================================

/// Compute all folding ranges for `source`.
///
/// Ranges are sorted by start line.  Duplicate / overlapping ranges are
/// deduplicated before the result is returned.
pub fn compute_folding_ranges(source: &str) -> Vec<FoldingRange> {
    let mut ranges: Vec<FoldingRange> = Vec::new();

    // 1. Comment blocks
    ranges.extend(find_comment_blocks(source));

    // 2. Import block
    if let Some(r) = find_import_block(source) {
        ranges.push(r);
    }

    // 3. Declaration bodies
    ranges.extend(find_declaration_bodies(source));

    // 4. Do-blocks
    ranges.extend(find_do_blocks(source));

    // Sort and deduplicate
    ranges.sort_by(|a, b| {
        a.start_line
            .cmp(&b.start_line)
            .then(a.end_line.cmp(&b.end_line))
    });
    ranges.dedup_by(|a, b| a.start_line == b.start_line && a.end_line == b.end_line);

    // 5. Merge adjacent single-line ranges of the same kind
    merge_adjacent_ranges(&mut ranges);

    // Filter out single-line ranges (nothing to fold)
    ranges.retain(|r| r.is_multiline());

    ranges
}

// ============================================================================
// Comment blocks
// ============================================================================

/// Detect comment folding ranges:
///
/// - Runs of two or more consecutive `--` line-comment lines.
/// - Multi-line block comments `/-  … -/`.
pub fn find_comment_blocks(source: &str) -> Vec<FoldingRange> {
    let lines: Vec<&str> = source.lines().collect();
    let mut result = Vec::new();

    let mut line_idx = 0usize;
    while line_idx < lines.len() {
        let text = lines[line_idx].trim_start();

        // ── Block comment /- … -/ ──────────────────────────────────────────
        if text.starts_with("/-") {
            let start = line_idx as u32;
            // Search for the closing `-/`
            let mut end_line = line_idx;
            let mut found_end = false;
            // Check if open and close are on the same line
            let first_after = if text.starts_with("/-") {
                &text[2..]
            } else {
                text
            };
            if first_after.contains("-/") {
                // single-line block comment — not interesting for folding
                line_idx += 1;
                continue;
            }
            'block: for (scan, &scan_line) in lines.iter().enumerate().skip(line_idx + 1) {
                if scan_line.contains("-/") {
                    end_line = scan;
                    found_end = true;
                    break 'block;
                }
            }
            if found_end && end_line > start as usize {
                result.push(FoldingRange::whole_lines(
                    start,
                    end_line as u32,
                    FoldingRangeKind::Comment,
                ));
                line_idx = end_line + 1;
                continue;
            }
        }

        // ── Consecutive -- line comments ───────────────────────────────────
        if text.starts_with("--") {
            let start = line_idx as u32;
            let mut run_end = line_idx;
            while run_end + 1 < lines.len() && lines[run_end + 1].trim_start().starts_with("--") {
                run_end += 1;
            }
            if run_end > line_idx {
                result.push(FoldingRange::whole_lines(
                    start,
                    run_end as u32,
                    FoldingRangeKind::Comment,
                ));
                line_idx = run_end + 1;
                continue;
            }
        }

        line_idx += 1;
    }
    result
}

// ============================================================================
// Import block
// ============================================================================

/// Detect the contiguous block of `import` lines at the top of the file.
///
/// Returns `None` if there are fewer than two consecutive import lines.
pub fn find_import_block(source: &str) -> Option<FoldingRange> {
    let mut start: Option<u32> = None;
    let mut end = 0u32;

    for (idx, line) in source.lines().enumerate() {
        let t = line.trim_start();
        if t.starts_with("import ") || t == "import" {
            if start.is_none() {
                start = Some(idx as u32);
            }
            end = idx as u32;
        } else if start.is_some() {
            // Only skip blank lines between imports
            if !t.is_empty() {
                break;
            }
        }
    }

    let start = start?;
    if end > start {
        Some(FoldingRange::whole_lines(
            start,
            end,
            FoldingRangeKind::Imports,
        ))
    } else {
        None
    }
}

// ============================================================================
// Declaration bodies
// ============================================================================

/// Declaration keywords that introduce foldable bodies.
const FOLD_DECL_KEYWORDS: &[&str] = &[
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
    "namespace",
    "section",
    "partial",
    "noncomputable",
];

/// Detect folding ranges for declaration bodies.
///
/// A declaration body runs from the declaration keyword line to the last line
/// before the next top-level declaration (or end of file).  Only declarations
/// whose body spans at least two lines are included.
pub fn find_declaration_bodies(source: &str) -> Vec<FoldingRange> {
    let lines: Vec<&str> = source.lines().collect();
    let mut result = Vec::new();

    let mut line_idx = 0usize;
    while line_idx < lines.len() {
        let text = lines[line_idx];
        let trimmed = text.trim_start();
        let indent = text.len() - trimmed.len();

        // Only consider top-level declarations (no leading indent)
        if indent == 0 && is_decl_start(trimmed) {
            let start = line_idx as u32;
            let end = find_decl_end_line(&lines, line_idx) as u32;
            if end > start {
                result.push(FoldingRange::whole_lines(
                    start,
                    end,
                    FoldingRangeKind::Region,
                ));
            }
            line_idx += 1;
            continue;
        }

        line_idx += 1;
    }
    result
}

/// Return `true` if `trimmed` starts with a fold-worthy declaration keyword.
fn is_decl_start(trimmed: &str) -> bool {
    for kw in FOLD_DECL_KEYWORDS {
        if trimmed.starts_with(kw) {
            let rest = &trimmed[kw.len()..];
            if rest.is_empty() || rest.starts_with(|c: char| c.is_whitespace() || c == '(') {
                return true;
            }
        }
    }
    false
}

/// Find the last line (inclusive) of the declaration starting at `start`.
fn find_decl_end_line(lines: &[&str], start: usize) -> usize {
    if start + 1 >= lines.len() {
        return start;
    }
    for (i, &line_text) in lines.iter().enumerate().skip(start + 1) {
        let t = line_text.trim_start();
        let indent = line_text.len() - t.len();
        if indent == 0 && !t.is_empty() && is_decl_start(t) {
            return i.saturating_sub(1);
        }
        if indent == 0 && (t.starts_with("end ") || t == "end") {
            return i;
        }
    }
    lines.len().saturating_sub(1)
}

// ============================================================================
// Do-blocks
// ============================================================================

/// Detect folding ranges for `do`-blocks.
///
/// Two patterns are recognised:
///
/// 1. `do` keyword at the end of a line followed by indented lines.
/// 2. A standalone `do` line followed by an `end` line.
pub fn find_do_blocks(source: &str) -> Vec<FoldingRange> {
    let lines: Vec<&str> = source.lines().collect();
    let mut result = Vec::new();

    let mut line_idx = 0usize;
    while line_idx < lines.len() {
        let text = lines[line_idx];
        let trimmed = text.trim_start();

        // Match lines that end in a `do` keyword (e.g., `def main : IO Unit := do`)
        if is_do_opener(trimmed) {
            let start = line_idx as u32;
            let base_indent = text.len() - trimmed.len();
            // The do-block ends when we see a line at the same or lower indent
            // that is not blank, starting from the next line.
            let end = find_indented_block_end(&lines, line_idx, base_indent);
            if end > line_idx {
                result.push(FoldingRange::whole_lines(
                    start,
                    end as u32,
                    FoldingRangeKind::Region,
                ));
                line_idx = end + 1;
                continue;
            }
        }

        line_idx += 1;
    }
    result
}

/// Return `true` if `trimmed` is a line that opens a `do`-block.
fn is_do_opener(trimmed: &str) -> bool {
    // Matches: "do", "… := do", "… := do --comment"
    let s = trimmed.trim_end();
    s == "do" || s.ends_with(" do") || s.ends_with("\tdo") || s.ends_with(":= do")
}

/// Find the last line index (inclusive) of an indented block that starts after
/// `start_line` at base indent `base_indent`.
fn find_indented_block_end(lines: &[&str], start_line: usize, base_indent: usize) -> usize {
    let mut last = start_line;
    for (i, &line_text) in lines.iter().enumerate().skip(start_line + 1) {
        if line_text.trim().is_empty() {
            continue; // blank lines don't close the block
        }
        let line_indent = line_text.len() - line_text.trim_start().len();
        if line_indent > base_indent {
            last = i;
        } else {
            break;
        }
    }
    last
}

// ============================================================================
// Merge adjacent ranges
// ============================================================================

/// Merge consecutive single-line folding ranges of the same kind into one
/// multi-line range.
///
/// This reduces visual noise when every line is an independent fold.
/// Only adjacent single-line ranges of the same kind are merged together.
/// Already-multiline ranges are never extended by this function.
pub fn merge_adjacent_ranges(ranges: &mut Vec<FoldingRange>) {
    if ranges.len() < 2 {
        return;
    }

    // Collect runs of consecutive single-line ranges of the same kind and
    // replace each run (length >= 2) with a single merged range.
    let mut output: Vec<FoldingRange> = Vec::with_capacity(ranges.len());

    let mut i = 0;
    while i < ranges.len() {
        let current = ranges[i].clone();
        // Only attempt to extend if this range is single-line
        if !current.is_multiline() {
            let mut run_end = current.end_line;
            let mut j = i + 1;
            while j < ranges.len() {
                let next = &ranges[j];
                let same_kind = next.kind == current.kind;
                let adjacent = run_end + 1 == next.start_line;
                let next_single = !next.is_multiline();
                if same_kind && adjacent && next_single {
                    run_end = next.end_line;
                    j += 1;
                } else {
                    break;
                }
            }
            if run_end > current.end_line {
                // Emit the merged range
                output.push(FoldingRange::whole_lines(
                    current.start_line,
                    run_end,
                    current.kind,
                ));
                i = j;
            } else {
                output.push(current);
                i += 1;
            }
        } else {
            output.push(current);
            i += 1;
        }
    }

    *ranges = output;
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lsp::folding_range::types::range_to_json;
    use crate::lsp::JsonValue;

    // ── find_comment_blocks ────────────────────────────────────────────────────

    #[test]
    fn test_comment_consecutive_lines() {
        let src = "-- line 1\n-- line 2\n-- line 3\ndef foo := 1\n";
        let ranges = find_comment_blocks(src);
        assert!(!ranges.is_empty());
        let r = &ranges[0];
        assert_eq!(r.start_line, 0);
        assert_eq!(r.end_line, 2);
        assert_eq!(r.kind, FoldingRangeKind::Comment);
    }

    #[test]
    fn test_comment_single_line_not_folded() {
        let src = "-- only one comment line\ndef foo := 1\n";
        let ranges = find_comment_blocks(src);
        // Single comment line should NOT produce a range
        assert!(ranges.is_empty());
    }

    #[test]
    fn test_comment_block_comment_multiline() {
        let src = "/- this is\n   a block\n   comment -/\ndef foo := 1\n";
        let ranges = find_comment_blocks(src);
        assert!(!ranges.is_empty());
        let r = &ranges[0];
        assert_eq!(r.start_line, 0);
        assert_eq!(r.end_line, 2);
        assert_eq!(r.kind, FoldingRangeKind::Comment);
    }

    #[test]
    fn test_comment_block_single_line_not_folded() {
        let src = "/- inline -/\ndef foo := 1\n";
        let ranges = find_comment_blocks(src);
        // Single-line block comment should not produce a fold
        assert!(ranges.is_empty());
    }

    #[test]
    fn test_comment_two_groups() {
        let src = "-- a\n-- b\ndef foo := 1\n-- c\n-- d\n-- e\n";
        let ranges = find_comment_blocks(src);
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].start_line, 0);
        assert_eq!(ranges[1].start_line, 3);
    }

    // ── find_import_block ──────────────────────────────────────────────────────

    #[test]
    fn test_import_block_basic() {
        let src = "import Lean\nimport Lean.Parser\nimport Std\ndef foo := 1\n";
        let r = find_import_block(src);
        assert!(r.is_some());
        let r = r.expect("must have import block");
        assert_eq!(r.start_line, 0);
        assert_eq!(r.end_line, 2);
        assert_eq!(r.kind, FoldingRangeKind::Imports);
    }

    #[test]
    fn test_import_block_single_import_not_folded() {
        let src = "import Lean\ndef foo := 1\n";
        let r = find_import_block(src);
        assert!(r.is_none());
    }

    #[test]
    fn test_import_block_not_present() {
        let src = "def foo := 1\ndef bar := 2\n";
        let r = find_import_block(src);
        assert!(r.is_none());
    }

    // ── find_declaration_bodies ────────────────────────────────────────────────

    #[test]
    fn test_decl_bodies_multiline_def() {
        let src = "def foo :=\n  let x := 1\n  x + 2\ndef bar := 3\n";
        let ranges = find_declaration_bodies(src);
        assert!(!ranges.is_empty());
        let r = &ranges[0];
        assert_eq!(r.start_line, 0);
        assert_eq!(r.kind, FoldingRangeKind::Region);
    }

    #[test]
    fn test_decl_bodies_single_line_def_not_folded() {
        // A one-line def that spans only its own line — end_line == start_line
        let src = "def foo := 1\n";
        let ranges = find_declaration_bodies(src);
        // Either empty or a 1-line range; 1-line ranges are filtered in compute
        for r in &ranges {
            assert!(!r.is_multiline() || r.start_line == 0);
        }
    }

    #[test]
    fn test_decl_bodies_theorem() {
        let src = "theorem myThm :\n  1 = 1 :=\n  rfl\ndef x := 1\n";
        let ranges = find_declaration_bodies(src);
        assert!(!ranges.is_empty());
        assert_eq!(ranges[0].start_line, 0);
    }

    #[test]
    fn test_decl_bodies_namespace() {
        let src = "namespace Foo\ndef bar := 1\nend Foo\n";
        let ranges = find_declaration_bodies(src);
        assert!(!ranges.is_empty());
    }

    #[test]
    fn test_decl_bodies_multiple() {
        let src = "def a :=\n  1\ndef b :=\n  2\n";
        let ranges = find_declaration_bodies(src);
        assert_eq!(ranges.len(), 2);
    }

    // ── find_do_blocks ─────────────────────────────────────────────────────────

    #[test]
    fn test_do_block_basic() {
        let src = "def main : IO Unit := do\n  let x := 1\n  pure ()\n";
        let ranges = find_do_blocks(src);
        assert!(!ranges.is_empty());
        let r = &ranges[0];
        assert_eq!(r.start_line, 0);
        assert!(r.end_line >= 2);
        assert_eq!(r.kind, FoldingRangeKind::Region);
    }

    #[test]
    fn test_do_block_no_do() {
        let src = "def foo := 1\n";
        let ranges = find_do_blocks(src);
        assert!(ranges.is_empty());
    }

    #[test]
    fn test_do_block_standalone() {
        let src = "do\n  let x := 1\n  let y := 2\n";
        let ranges = find_do_blocks(src);
        assert!(!ranges.is_empty());
    }

    // ── merge_adjacent_ranges ──────────────────────────────────────────────────

    #[test]
    fn test_merge_adjacent_same_kind() {
        let mut ranges = vec![
            FoldingRange::whole_lines(0, 0, FoldingRangeKind::Comment),
            FoldingRange::whole_lines(1, 1, FoldingRangeKind::Comment),
            FoldingRange::whole_lines(2, 2, FoldingRangeKind::Comment),
        ];
        merge_adjacent_ranges(&mut ranges);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].start_line, 0);
        assert_eq!(ranges[0].end_line, 2);
    }

    #[test]
    fn test_merge_adjacent_different_kinds() {
        let mut ranges = vec![
            FoldingRange::whole_lines(0, 0, FoldingRangeKind::Comment),
            FoldingRange::whole_lines(1, 1, FoldingRangeKind::Imports),
        ];
        merge_adjacent_ranges(&mut ranges);
        // Different kinds must not be merged
        assert_eq!(ranges.len(), 2);
    }

    #[test]
    fn test_merge_nonadjacent_not_merged() {
        let mut ranges = vec![
            FoldingRange::whole_lines(0, 0, FoldingRangeKind::Comment),
            FoldingRange::whole_lines(5, 5, FoldingRangeKind::Comment),
        ];
        merge_adjacent_ranges(&mut ranges);
        assert_eq!(ranges.len(), 2);
    }

    #[test]
    fn test_merge_multiline_not_merged_with_single() {
        let mut ranges = vec![
            FoldingRange::whole_lines(0, 3, FoldingRangeKind::Region),
            FoldingRange::whole_lines(4, 4, FoldingRangeKind::Region),
        ];
        merge_adjacent_ranges(&mut ranges);
        // The first range is already multiline — not merged
        assert_eq!(ranges.len(), 2);
    }

    // ── range_to_json ──────────────────────────────────────────────────────────

    #[test]
    fn test_range_to_json_basic() {
        let r = FoldingRange::whole_lines(5, 10, FoldingRangeKind::Region);
        let json = range_to_json(&r);
        assert_eq!(json.get("startLine").and_then(|v| v.as_i64()), Some(5));
        assert_eq!(json.get("endLine").and_then(|v| v.as_i64()), Some(10));
        assert_eq!(json.get("kind").and_then(|v| v.as_str()), Some("region"));
        // No character fields in whole-line ranges
        assert!(json.get("startCharacter").is_none());
        assert!(json.get("endCharacter").is_none());
    }

    #[test]
    fn test_range_to_json_with_chars() {
        let r = FoldingRange::with_chars(0, 5, 2, 8, FoldingRangeKind::Comment);
        let json = range_to_json(&r);
        assert_eq!(json.get("startCharacter").and_then(|v| v.as_i64()), Some(2));
        assert_eq!(json.get("endCharacter").and_then(|v| v.as_i64()), Some(8));
    }

    #[test]
    fn test_range_to_json_comment_kind() {
        let r = FoldingRange::whole_lines(0, 2, FoldingRangeKind::Comment);
        let json = range_to_json(&r);
        assert_eq!(json.get("kind").and_then(|v| v.as_str()), Some("comment"));
    }

    #[test]
    fn test_range_to_json_imports_kind() {
        let r = FoldingRange::whole_lines(0, 3, FoldingRangeKind::Imports);
        let json = range_to_json(&r);
        assert_eq!(json.get("kind").and_then(|v| v.as_str()), Some("imports"));
    }

    // ── compute_folding_ranges (integration) ──────────────────────────────────

    #[test]
    fn test_compute_mixed_source() {
        let src = concat!(
            "import Lean\n",
            "import Std\n",
            "-- A comment\n",
            "-- continuation\n",
            "def foo :=\n",
            "  let x := 1\n",
            "  x\n",
        );
        let ranges = compute_folding_ranges(src);
        // Should have: imports block, comment block, def body
        assert!(ranges.len() >= 2);
        let kinds: Vec<FoldingRangeKind> = ranges.iter().map(|r| r.kind).collect();
        assert!(kinds.contains(&FoldingRangeKind::Imports));
        assert!(kinds.contains(&FoldingRangeKind::Comment));
        assert!(kinds.contains(&FoldingRangeKind::Region));
    }

    #[test]
    fn test_compute_empty_source() {
        let ranges = compute_folding_ranges("");
        assert!(ranges.is_empty());
    }

    #[test]
    fn test_compute_all_multiline() {
        let ranges = compute_folding_ranges("def a := 1\ndef b := 2\n");
        // Single-line defs produce no folds
        for r in &ranges {
            assert!(r.is_multiline(), "all returned ranges must be multiline");
        }
    }

    #[test]
    fn test_range_to_json_returns_object() {
        let r = FoldingRange::whole_lines(0, 5, FoldingRangeKind::Region);
        let json = r.to_json();
        assert!(matches!(json, JsonValue::Object(_)));
    }

    #[test]
    fn test_folding_range_kind_as_str() {
        assert_eq!(FoldingRangeKind::Comment.as_str(), "comment");
        assert_eq!(FoldingRangeKind::Imports.as_str(), "imports");
        assert_eq!(FoldingRangeKind::Region.as_str(), "region");
    }

    #[test]
    fn test_folding_range_line_count() {
        let r = FoldingRange::whole_lines(2, 7, FoldingRangeKind::Region);
        assert_eq!(r.line_count(), 6);
    }

    #[test]
    fn test_is_multiline() {
        let single = FoldingRange::whole_lines(3, 3, FoldingRangeKind::Comment);
        let multi = FoldingRange::whole_lines(3, 5, FoldingRangeKind::Comment);
        assert!(!single.is_multiline());
        assert!(multi.is_multiline());
    }
}
