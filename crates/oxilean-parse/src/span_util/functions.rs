//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tokens::Span;

use super::types::{PrioritizedSpan, SourcePos};

/// Create a dummy span for testing (line 1, column 1, zero-length).
#[allow(missing_docs)]
pub fn dummy_span() -> Span {
    Span::new(0, 0, 1, 1)
}
/// Check if a span contains a byte position.
#[allow(missing_docs)]
pub fn span_contains(span: &Span, pos: usize) -> bool {
    pos >= span.start && pos < span.end
}
/// Merge multiple spans into one encompassing span.
///
/// Returns `None` if the slice is empty.
#[allow(missing_docs)]
pub fn merge_spans(spans: &[Span]) -> Option<Span> {
    if spans.is_empty() {
        return None;
    }
    let mut result = spans[0].clone();
    for span in &spans[1..] {
        result = result.merge(span);
    }
    Some(result)
}
/// Get the byte-length of a span.
#[allow(missing_docs)]
pub fn span_len(span: &Span) -> usize {
    span.end.saturating_sub(span.start)
}
/// Extract the `SourcePos` from a `Span` (start of span).
#[allow(missing_docs)]
pub fn span_start_pos(span: &Span) -> SourcePos {
    SourcePos::new(span.line, span.column)
}
/// Extract the start byte offset from a `Span`.
#[allow(missing_docs)]
pub fn span_start(span: &Span) -> usize {
    span.start
}
/// Extract the end byte offset from a `Span`.
#[allow(missing_docs)]
pub fn span_end(span: &Span) -> usize {
    span.end
}
/// Return a span that begins immediately after `span` ends (zero-length).
#[allow(missing_docs)]
pub fn span_after(span: &Span) -> Span {
    Span::new(span.end, span.end, span.line, span.column + span_len(span))
}
/// Return a span that begins one byte before `span` starts (zero-length).
///
/// Saturates at zero — will not produce negative offsets.
#[allow(missing_docs)]
pub fn span_before(span: &Span) -> Span {
    let start = span.start.saturating_sub(1);
    let col = span.column.saturating_sub(1).max(1);
    Span::new(start, start, span.line, col)
}
/// Extend `span` by `n` bytes to the right (clamped to `source_len`).
#[allow(missing_docs)]
pub fn span_extend(span: &Span, n: usize, source_len: usize) -> Span {
    let new_end = (span.end + n).min(source_len);
    Span::new(span.start, new_end, span.line, span.column)
}
/// Shrink a span from the left by `n` bytes.
#[allow(missing_docs)]
pub fn span_shrink_left(span: &Span, n: usize) -> Span {
    let new_start = (span.start + n).min(span.end);
    Span::new(new_start, span.end, span.line, span.column + n)
}
/// Shrink a span from the right by `n` bytes.
#[allow(missing_docs)]
pub fn span_shrink_right(span: &Span, n: usize) -> Span {
    let new_end = span.end.saturating_sub(n).max(span.start);
    Span::new(span.start, new_end, span.line, span.column)
}
/// Return `true` if `a` and `b` overlap (share at least one byte).
#[allow(missing_docs)]
pub fn spans_overlap(a: &Span, b: &Span) -> bool {
    a.start < b.end && b.start < a.end
}
/// Return `true` if `outer` completely contains `inner`.
#[allow(missing_docs)]
pub fn span_contains_span(outer: &Span, inner: &Span) -> bool {
    outer.start <= inner.start && inner.end <= outer.end
}
/// Return the intersection of two spans, or `None` if they do not overlap.
#[allow(missing_docs)]
pub fn span_intersection(a: &Span, b: &Span) -> Option<Span> {
    let start = a.start.max(b.start);
    let end = a.end.min(b.end);
    if start < end {
        Some(Span::new(start, end, a.line, a.column))
    } else {
        None
    }
}
/// Extract the source text covered by `span` from a full source `&str`.
///
/// Returns `""` if the span is out of bounds.
#[allow(missing_docs)]
pub fn extract_span<'a>(source: &'a str, span: &Span) -> &'a str {
    source.get(span.start..span.end).unwrap_or("")
}
/// Return the entire line that contains `span` from `source`.
///
/// Scans backwards from `span.start` to find the line start.
#[allow(missing_docs)]
pub fn extract_line<'a>(source: &'a str, span: &Span) -> &'a str {
    let bytes = source.as_bytes();
    let line_start = bytes[..span.start]
        .iter()
        .rposition(|&b| b == b'\n')
        .map(|i| i + 1)
        .unwrap_or(0);
    let line_end = bytes[span.start..]
        .iter()
        .position(|&b| b == b'\n')
        .map(|i| span.start + i)
        .unwrap_or(source.len());
    &source[line_start..line_end]
}
/// Count the number of lines in a source string.
#[allow(missing_docs)]
pub fn count_lines(source: &str) -> usize {
    source.bytes().filter(|&b| b == b'\n').count() + 1
}
/// Compute a `Span` from a (line, col) pair by scanning the source string.
///
/// Line and column are 1-indexed.  Returns `None` if out of range.
#[allow(missing_docs)]
pub fn pos_to_span(source: &str, line: usize, col: usize) -> Option<Span> {
    let mut cur_line = 1usize;
    let mut cur_col = 1usize;
    for (i, ch) in source.char_indices() {
        if cur_line == line && cur_col == col {
            let end = i + ch.len_utf8();
            return Some(Span::new(i, end, line, col));
        }
        if ch == '\n' {
            cur_line += 1;
            cur_col = 1;
        } else {
            cur_col += 1;
        }
    }
    None
}
/// Format a span as `line:col` for short diagnostic messages.
#[allow(missing_docs)]
pub fn span_short(span: &Span) -> String {
    format!("{}:{}", span.line, span.column)
}
/// Format a span as `line:col-end` showing the end column too.
#[allow(missing_docs)]
pub fn span_range(span: &Span) -> String {
    let end_col = span.column + span_len(span);
    format!("{}:{}-{}", span.line, span.column, end_col)
}
/// Build a caret string (`^~~~~`) pointing at a span within its line.
///
/// `col` is 1-indexed; `len` is the number of `~` characters after the first `^`.
#[allow(missing_docs)]
pub fn span_caret(col: usize, len: usize) -> String {
    let spaces = " ".repeat(col.saturating_sub(1));
    let carets = if len == 0 {
        "^".to_string()
    } else {
        let mut s = "^".to_string();
        s.push_str(&"~".repeat(len.saturating_sub(1)));
        s
    };
    format!("{}{}", spaces, carets)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::span_util::*;
    #[test]
    fn test_dummy_span() {
        let span = dummy_span();
        assert_eq!(span.line, 1);
        assert_eq!(span.column, 1);
    }
    #[test]
    fn test_span_contains() {
        let span = Span::new(10, 20, 1, 1);
        assert!(span_contains(&span, 10));
        assert!(span_contains(&span, 15));
        assert!(!span_contains(&span, 20));
        assert!(!span_contains(&span, 5));
    }
    #[test]
    fn test_merge_spans() {
        let s1 = Span::new(0, 5, 1, 1);
        let s2 = Span::new(10, 15, 1, 11);
        let s3 = Span::new(20, 25, 2, 1);
        let merged = merge_spans(&[s1, s2, s3]).expect("span should be present");
        assert_eq!(merged.start, 0);
        assert_eq!(merged.end, 25);
    }
    #[test]
    fn test_span_len() {
        let span = Span::new(10, 25, 1, 1);
        assert_eq!(span_len(&span), 15);
    }
    #[test]
    fn test_source_pos_display() {
        let pos = SourcePos::new(3, 7);
        assert_eq!(pos.to_string(), "3:7");
    }
    #[test]
    fn test_source_pos_advance() {
        let pos = SourcePos::start();
        let next = pos.advance_col();
        assert_eq!(next.col, 2);
        let nl = pos.advance_line();
        assert_eq!(nl.line, 2);
        assert_eq!(nl.col, 1);
    }
    #[test]
    fn test_spans_overlap() {
        let a = Span::new(0, 10, 1, 1);
        let b = Span::new(5, 15, 1, 6);
        let c = Span::new(10, 20, 1, 11);
        assert!(spans_overlap(&a, &b));
        assert!(!spans_overlap(&a, &c));
    }
    #[test]
    fn test_span_contains_span() {
        let outer = Span::new(0, 20, 1, 1);
        let inner = Span::new(5, 15, 1, 6);
        let too_big = Span::new(5, 25, 1, 6);
        assert!(span_contains_span(&outer, &inner));
        assert!(!span_contains_span(&outer, &too_big));
    }
    #[test]
    fn test_span_intersection() {
        let a = Span::new(0, 10, 1, 1);
        let b = Span::new(5, 15, 1, 6);
        let isect = span_intersection(&a, &b).expect("span should be present");
        assert_eq!(isect.start, 5);
        assert_eq!(isect.end, 10);
        let c = Span::new(20, 30, 2, 1);
        assert!(span_intersection(&a, &c).is_none());
    }
    #[test]
    fn test_extract_span() {
        let source = "hello world";
        let span = Span::new(6, 11, 1, 7);
        assert_eq!(extract_span(source, &span), "world");
    }
    #[test]
    fn test_extract_line() {
        let source = "line one\nline two\nline three";
        let span = Span::new(9, 13, 2, 1);
        assert_eq!(extract_line(source, &span), "line two");
    }
    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines("a\nb\nc"), 3);
        assert_eq!(count_lines("no newlines"), 1);
    }
    #[test]
    fn test_span_caret() {
        let c = span_caret(5, 3);
        assert!(c.starts_with("    ^"));
    }
    #[test]
    fn test_span_builder() {
        let builder = SpanBuilder::new(10, 2, 5);
        let span = builder.finish(20);
        assert_eq!(span.start, 10);
        assert_eq!(span.end, 20);
        assert_eq!(span.line, 2);
    }
    #[test]
    fn test_spanned_map() {
        let s = Spanned::new(42u32, dummy_span());
        let s2 = s.map(|v| v * 2);
        assert_eq!(s2.value, 84);
    }
    #[test]
    fn test_labeled_span() {
        let span = Span::new(0, 5, 1, 1);
        let ls = LabeledSpan::new("here", span);
        assert_eq!(ls.label, "here");
        assert_eq!(ls.len(), 5);
        assert!(!ls.is_empty());
    }
    #[test]
    fn test_span_short_and_range() {
        let span = Span::new(10, 15, 3, 7);
        assert_eq!(span_short(&span), "3:7");
        let r = span_range(&span);
        assert!(r.contains("3:7"));
    }
    #[test]
    fn test_span_extend_shrink() {
        let span = Span::new(5, 10, 1, 6);
        let extended = span_extend(&span, 3, 100);
        assert_eq!(extended.end, 13);
        let shrunk = span_shrink_right(&span, 2);
        assert_eq!(shrunk.end, 8);
        let shrunk_l = span_shrink_left(&span, 2);
        assert_eq!(shrunk_l.start, 7);
    }
    #[test]
    fn test_merge_empty() {
        assert!(merge_spans(&[]).is_none());
    }
    #[test]
    fn test_span_before_after() {
        let span = Span::new(10, 15, 1, 11);
        let after = span_after(&span);
        assert_eq!(after.start, 15);
        let before = span_before(&span);
        assert_eq!(before.start, 9);
    }
}
/// Return `true` if `a` starts before `b` (by byte offset).
#[allow(missing_docs)]
pub fn span_before_other(a: &Span, b: &Span) -> bool {
    a.start < b.start
}
/// Sort a slice of spans by start offset.
#[allow(missing_docs)]
pub fn sort_spans(spans: &mut [Span]) {
    spans.sort_by_key(|s| s.start);
}
/// Merge overlapping/adjacent spans in a sorted list.
#[allow(missing_docs)]
pub fn coalesce_spans(spans: &[Span]) -> Vec<Span> {
    if spans.is_empty() {
        return vec![];
    }
    let mut result: Vec<Span> = Vec::new();
    let mut current = spans[0].clone();
    for span in &spans[1..] {
        if span.start <= current.end {
            current = current.merge(span);
        } else {
            result.push(current.clone());
            current = span.clone();
        }
    }
    result.push(current);
    result
}
/// Compute the gap (non-overlapping region) between two non-overlapping spans.
///
/// Returns `None` if the spans overlap.
#[allow(missing_docs)]
pub fn span_gap(a: &Span, b: &Span) -> Option<Span> {
    let (first, second) = if a.end <= b.start { (a, b) } else { (b, a) };
    if first.end < second.start {
        Some(Span::new(
            first.end,
            second.start,
            first.line,
            first.column + span_len(first),
        ))
    } else {
        None
    }
}
/// Build a line-start index from a source string.
///
/// `line_starts[i]` is the byte offset of line `i` (0-indexed).
#[allow(missing_docs)]
pub fn build_line_index(source: &str) -> Vec<usize> {
    let mut starts = vec![0usize];
    for (i, &b) in source.as_bytes().iter().enumerate() {
        if b == b'\n' {
            starts.push(i + 1);
        }
    }
    starts
}
/// Convert a byte offset to a `(line, col)` pair using a pre-built line index.
///
/// Both values are 1-indexed.
#[allow(missing_docs)]
pub fn offset_to_line_col(line_index: &[usize], offset: usize) -> (usize, usize) {
    let line = match line_index.binary_search(&offset) {
        Ok(i) => i,
        Err(i) => i.saturating_sub(1),
    };
    let col = offset - line_index.get(line).copied().unwrap_or(0);
    (line + 1, col + 1)
}
#[cfg(test)]
mod span_extra_tests {
    use super::*;
    use crate::span_util::*;
    #[test]
    fn test_source_cursor_basic() {
        let src = "hello";
        let mut cursor = SourceCursor::new(src);
        assert_eq!(cursor.peek(), Some('h'));
        assert_eq!(cursor.advance(), Some('h'));
        assert_eq!(cursor.pos(), 1);
        assert_eq!(cursor.peek(), Some('e'));
    }
    #[test]
    fn test_source_cursor_newline() {
        let src = "a\nb";
        let mut cursor = SourceCursor::new(src);
        cursor.advance();
        cursor.advance();
        assert_eq!(cursor.source_pos().line, 2);
        assert_eq!(cursor.source_pos().col, 1);
    }
    #[test]
    fn test_source_cursor_advance_while() {
        let src = "   hello";
        let mut cursor = SourceCursor::new(src);
        let spaces = cursor.advance_while(|c| c == ' ');
        assert_eq!(spaces, "   ");
        assert_eq!(cursor.peek(), Some('h'));
    }
    #[test]
    fn test_source_cursor_eof() {
        let mut cursor = SourceCursor::new("ab");
        cursor.advance();
        cursor.advance();
        assert!(cursor.is_eof());
        assert_eq!(cursor.advance(), None);
    }
    #[test]
    fn test_span_registry() {
        let mut reg = SpanRegistry::new();
        let span = Span::new(0, 5, 1, 1);
        reg.register("foo", span.clone());
        assert!(reg.contains("foo"));
        assert!(!reg.contains("bar"));
        assert_eq!(reg.get("foo").expect("key should exist").end, 5);
        assert_eq!(reg.len(), 1);
    }
    #[test]
    fn test_span_registry_merge() {
        let mut a = SpanRegistry::new();
        let mut b = SpanRegistry::new();
        a.register("x", Span::new(0, 1, 1, 1));
        b.register("y", Span::new(2, 3, 1, 3));
        a.merge(b);
        assert_eq!(a.len(), 2);
    }
    #[test]
    fn test_coalesce_spans() {
        let spans = vec![
            Span::new(0, 5, 1, 1),
            Span::new(3, 8, 1, 4),
            Span::new(20, 25, 2, 1),
        ];
        let coalesced = coalesce_spans(&spans);
        assert_eq!(coalesced.len(), 2);
        assert_eq!(coalesced[0].end, 8);
        assert_eq!(coalesced[1].start, 20);
    }
    #[test]
    fn test_span_gap() {
        let a = Span::new(0, 5, 1, 1);
        let b = Span::new(10, 15, 1, 11);
        let gap = span_gap(&a, &b).expect("span should be present");
        assert_eq!(gap.start, 5);
        assert_eq!(gap.end, 10);
    }
    #[test]
    fn test_span_gap_overlap() {
        let a = Span::new(0, 10, 1, 1);
        let b = Span::new(5, 15, 1, 6);
        assert!(span_gap(&a, &b).is_none());
    }
    #[test]
    fn test_build_line_index() {
        let src = "abc\ndef\nghi";
        let idx = build_line_index(src);
        assert_eq!(idx, vec![0, 4, 8]);
    }
    #[test]
    fn test_offset_to_line_col() {
        let src = "abc\ndef\nghi";
        let idx = build_line_index(src);
        assert_eq!(offset_to_line_col(&idx, 0), (1, 1));
        assert_eq!(offset_to_line_col(&idx, 4), (2, 1));
        assert_eq!(offset_to_line_col(&idx, 5), (2, 2));
    }
    #[test]
    fn test_sort_spans() {
        let mut spans = vec![
            Span::new(10, 15, 1, 11),
            Span::new(0, 5, 1, 1),
            Span::new(5, 10, 1, 6),
        ];
        sort_spans(&mut spans);
        assert_eq!(spans[0].start, 0);
        assert_eq!(spans[1].start, 5);
        assert_eq!(spans[2].start, 10);
    }
    #[test]
    fn test_span_before_other() {
        let a = Span::new(0, 5, 1, 1);
        let b = Span::new(10, 15, 1, 11);
        assert!(span_before_other(&a, &b));
        assert!(!span_before_other(&b, &a));
    }
}
/// Highlight a span in source text using a caret line.
///
/// Returns a two-line string: the source line, then the caret annotation.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn highlight_span(source: &str, span: &Span) -> String {
    let line = extract_line(source, span);
    let caret = span_caret(span.column, span_len(span));
    format!("{}\n{}", line, caret)
}
/// A zero-length span at byte offset `pos` with the given line/col.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn zero_span(pos: usize, line: usize, col: usize) -> Span {
    Span::new(pos, pos, line, col)
}
/// Check whether a byte offset is within the bounds of a source string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_valid_offset(source: &str, offset: usize) -> bool {
    offset <= source.len()
}
/// Count the lines covered by a span.
///
/// Returns 1 if the span is on a single line, more if it spans newlines.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_line_count(source: &str, span: &Span) -> usize {
    let text = extract_span(source, span);
    text.bytes().filter(|&b| b == b'\n').count() + 1
}
/// Return the column at which the span ends (exclusive).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_end_col(span: &Span) -> usize {
    span.column + span_len(span)
}
/// Build an `IndexVec` of all span starts from a list of spans.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_start_offsets(spans: &[Span]) -> Vec<usize> {
    spans.iter().map(|s| s.start).collect()
}
/// Find the span that contains byte `offset`, if any.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn find_span_at(spans: &[Span], offset: usize) -> Option<&Span> {
    spans.iter().find(|s| span_contains(s, offset))
}
/// Partition spans into two groups: those entirely within `region`, and those outside.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn partition_spans_by_region<'a>(
    spans: &'a [Span],
    region: &Span,
) -> (Vec<&'a Span>, Vec<&'a Span>) {
    let inside = spans
        .iter()
        .filter(|s| span_contains_span(region, s))
        .collect();
    let outside = spans
        .iter()
        .filter(|s| !span_contains_span(region, s))
        .collect();
    (inside, outside)
}
#[cfg(test)]
mod diag_span_tests {
    use super::*;
    use crate::span_util::*;
    #[test]
    fn test_diagnostic_span_error() {
        let span = dummy_span();
        let d = DiagnosticSpan::error(span, "test error");
        assert!(d.severity.is_error());
        assert!(d.format_short().contains("error"));
    }
    #[test]
    fn test_diagnostic_span_warning() {
        let span = dummy_span();
        let d = DiagnosticSpan::warning(span, "test warning");
        assert_eq!(d.severity.label(), "warning");
    }
    #[test]
    fn test_diagnostic_set_count() {
        let mut set = DiagnosticSet::new();
        set.add_error(dummy_span(), "e1");
        set.add_error(dummy_span(), "e2");
        set.add_warning(dummy_span(), "w1");
        assert_eq!(set.count_severity(&SpanSeverity::Error), 2);
        assert_eq!(set.count_severity(&SpanSeverity::Warning), 1);
        assert_eq!(set.len(), 3);
        assert!(set.has_errors());
    }
    #[test]
    fn test_diagnostic_set_no_errors() {
        let mut set = DiagnosticSet::new();
        set.add_info(dummy_span(), "info");
        assert!(!set.has_errors());
    }
    #[test]
    fn test_diagnostic_set_merge() {
        let mut a = DiagnosticSet::new();
        a.add_error(dummy_span(), "e1");
        let mut b = DiagnosticSet::new();
        b.add_warning(dummy_span(), "w1");
        a.merge(b);
        assert_eq!(a.len(), 2);
    }
    #[test]
    fn test_diagnostic_set_clear() {
        let mut set = DiagnosticSet::new();
        set.add_error(dummy_span(), "e");
        set.clear();
        assert!(set.is_empty());
    }
    #[test]
    fn test_highlight_span() {
        let src = "hello world";
        let span = Span::new(6, 11, 1, 7);
        let h = highlight_span(src, &span);
        assert!(h.contains("hello world"));
        assert!(h.contains('^'));
    }
    #[test]
    fn test_zero_span() {
        let z = zero_span(10, 2, 3);
        assert_eq!(z.start, 10);
        assert_eq!(z.end, 10);
        assert!(span_len(&z) == 0);
    }
    #[test]
    fn test_span_line_count() {
        let src = "ab\ncd\nef";
        let span = Span::new(0, 8, 1, 1);
        assert_eq!(span_line_count(src, &span), 3);
    }
    #[test]
    fn test_span_diff() {
        let old = Span::new(0, 5, 1, 1);
        let new = Span::new(0, 8, 1, 1);
        let diff = SpanDiff::compute(old, new);
        assert!(diff.grew());
        assert!(!diff.shrank());
        assert!(!diff.unchanged());
    }
    #[test]
    fn test_span_map() {
        let mut m: SpanMap<&str> = SpanMap::new();
        m.insert(5, "hello");
        m.insert(10, "world");
        assert_eq!(m.get(5), Some(&"hello"));
        assert_eq!(m.get(99), None);
        assert_eq!(m.len(), 2);
    }
    #[test]
    fn test_find_span_at() {
        let spans = vec![Span::new(0, 5, 1, 1), Span::new(10, 15, 1, 11)];
        let found = find_span_at(&spans, 3);
        assert!(found.is_some());
        let not_found = find_span_at(&spans, 7);
        assert!(not_found.is_none());
    }
    #[test]
    fn test_span_end_col() {
        let span = Span::new(0, 5, 1, 1);
        assert_eq!(span_end_col(&span), 6);
    }
    #[test]
    fn test_is_valid_offset() {
        let src = "hello";
        assert!(is_valid_offset(src, 5));
        assert!(!is_valid_offset(src, 6));
    }
}
/// Convert a byte column offset within a line to a UTF-16 code-unit count.
///
/// `line_text` is the full text of the line; `byte_col` is 0-indexed from line start.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn byte_col_to_utf16(line_text: &str, byte_col: usize) -> usize {
    let prefix = line_text.get(..byte_col).unwrap_or(line_text);
    prefix.chars().map(|c| c.len_utf16()).sum()
}
/// Convert a UTF-16 code-unit count within a line to a byte offset.
///
/// Returns the byte offset at which the UTF-16 unit `utf16_col` falls.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn utf16_col_to_byte(line_text: &str, utf16_col: usize) -> usize {
    let mut remaining = utf16_col;
    let mut byte_offset = 0;
    for ch in line_text.chars() {
        if remaining == 0 {
            break;
        }
        let units = ch.len_utf16();
        if remaining < units {
            break;
        }
        remaining -= units;
        byte_offset += ch.len_utf8();
    }
    byte_offset
}
/// Compute the byte offset of a `(line, col)` position in `source`,
/// where both `line` and `col` are 1-indexed.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn line_col_to_offset(source: &str, line: usize, col: usize) -> Option<usize> {
    let mut cur_line = 1usize;
    let mut cur_col = 1usize;
    let mut pos = 0usize;
    for ch in source.chars() {
        if cur_line == line && cur_col == col {
            return Some(pos);
        }
        if ch == '\n' {
            cur_line += 1;
            cur_col = 1;
        } else {
            cur_col += 1;
        }
        pos += ch.len_utf8();
    }
    if cur_line == line && cur_col == col {
        Some(pos)
    } else {
        None
    }
}
/// Apply a byte delta to all spans in a list (e.g., after an edit).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn shift_spans(spans: &mut [Span], edit_start: usize, delta: i64) {
    for span in spans.iter_mut() {
        if span.start >= edit_start {
            span.start = (span.start as i64 + delta).max(0) as usize;
            span.end = (span.end as i64 + delta).max(span.start as i64) as usize;
        } else if span.end > edit_start {
            span.end = edit_start;
        }
    }
}
/// Count the number of Unicode scalar values in a span of source.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_chars(source: &str, span: &Span) -> usize {
    extract_span(source, span).chars().count()
}
/// Split a span at a byte offset within it, returning two adjacent spans.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn split_span(span: &Span, split_at: usize) -> (Span, Span) {
    assert!(
        split_at >= span.start && split_at <= span.end,
        "split_at out of range"
    );
    let left = Span::new(span.start, split_at, span.line, span.column);
    let right_col = span.column + (split_at - span.start);
    let right = Span::new(split_at, span.end, span.line, right_col);
    (left, right)
}
/// Format a `Span` as `"file:line:col"` using an optional file name.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_to_diagnostic_string(file: &str, span: &Span) -> String {
    format!("{}:{}:{}", file, span.line, span.column)
}
/// Format a multi-line diagnostic snippet.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_diagnostic(source: &str, span: &Span, label: &str) -> String {
    let line = extract_line(source, span);
    let caret = span_caret(span.column, span_len(span));
    format!("{}\n{}\n{}", line, caret, label)
}
/// Convert a `Span` to a human-readable summary string for debugging.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_debug_str(span: &Span) -> String {
    format!(
        "Span {{ start: {}, end: {}, line: {}, col: {} }}",
        span.start, span.end, span.line, span.column
    )
}
/// Return `true` if two spans are exactly adjacent.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn spans_adjacent(a: &Span, b: &Span) -> bool {
    a.end == b.start || b.end == a.start
}
/// Compute the center byte offset of a span.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_center(span: &Span) -> usize {
    span.start + span_len(span) / 2
}
/// Check whether a span is a strict subspan of another.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_strictly_contains(outer: &Span, inner: &Span) -> bool {
    outer.start <= inner.start
        && inner.end <= outer.end
        && !(outer.start == inner.start && outer.end == inner.end)
}
/// Sort spans by their end offset (ascending).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn sort_spans_by_end(spans: &mut [Span]) {
    spans.sort_by(|a, b| a.end.cmp(&b.end).then(a.start.cmp(&b.start)));
}
/// Sort spans in reverse order (largest start offset first).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn sort_spans_reverse(spans: &mut [Span]) {
    spans.sort_by(|a, b| b.start.cmp(&a.start));
}
/// Return `true` if `spans` are sorted by start offset.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn spans_are_sorted(spans: &[Span]) -> bool {
    spans.windows(2).all(|w| w[0].start <= w[1].start)
}
/// Deduplicate a sorted list of spans (remove exact duplicates).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn dedup_spans(spans: &mut Vec<Span>) {
    spans.dedup_by(|a, b| a.start == b.start && a.end == b.end);
}
/// Return all spans that do NOT overlap with `exclude`.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn filter_non_overlapping<'a>(spans: &'a [Span], exclude: &Span) -> Vec<&'a Span> {
    spans
        .iter()
        .filter(|s| !spans_overlap(s, exclude))
        .collect()
}
/// Total ordering comparison for spans: by start, then by end.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_cmp(a: &Span, b: &Span) -> std::cmp::Ordering {
    a.start.cmp(&b.start).then(a.end.cmp(&b.end))
}
/// `true` if `a` ends at or before `b` starts (non-overlapping).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_precedes(a: &Span, b: &Span) -> bool {
    a.end <= b.start
}
/// `true` if `a` and `b` are the same byte range.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_eq_range(a: &Span, b: &Span) -> bool {
    a.start == b.start && a.end == b.end
}
/// Compute the minimum distance (in bytes) between two non-overlapping spans.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_distance(a: &Span, b: &Span) -> usize {
    if spans_overlap(a, b) {
        0
    } else if a.end <= b.start {
        b.start - a.end
    } else {
        a.start - b.end
    }
}
/// Choose the span that starts earlier.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_earlier<'a>(a: &'a Span, b: &'a Span) -> &'a Span {
    if a.start <= b.start {
        a
    } else {
        b
    }
}
/// Choose the span that ends later.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn span_later<'a>(a: &'a Span, b: &'a Span) -> &'a Span {
    if a.end >= b.end {
        a
    } else {
        b
    }
}
/// Extract a "window" of lines around a span for error reporting.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn extract_context_window(
    source: &str,
    span: &Span,
    context_lines: usize,
) -> Vec<(usize, String)> {
    let lines: Vec<&str> = source.split('\n').collect();
    let span_line = span.line.saturating_sub(1);
    let first = span_line.saturating_sub(context_lines);
    let last = (span_line + context_lines + 1).min(lines.len());
    lines[first..last]
        .iter()
        .enumerate()
        .map(|(i, &l)| (first + i + 1, l.to_string()))
        .collect()
}
/// Format a context window as a human-readable string with line numbers.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_context_window(window: &[(usize, String)], highlight_line: usize) -> String {
    let mut out = String::new();
    for (ln, text) in window {
        let marker = if *ln == highlight_line { ">" } else { " " };
        out.push_str(&format!("{} {:4} | {}\n", marker, ln, text));
    }
    out
}
#[cfg(test)]
mod extended_span_tests {
    use super::*;
    use crate::span_util::*;
    #[test]
    fn test_file_id_basic() {
        let id = FileId::new(42);
        assert_eq!(id.raw(), 42);
        assert_eq!(id.to_string(), "file#42");
    }
    #[test]
    fn test_file_registry_register() {
        let mut reg = FileRegistry::new();
        let id = reg.register("foo.lean", "def x := 1");
        assert_eq!(reg.source(id), Some("def x := 1"));
        assert_eq!(reg.path(id), Some("foo.lean"));
    }
    #[test]
    fn test_file_registry_extract() {
        let mut reg = FileRegistry::new();
        let id = reg.register("a.lean", "hello world");
        let span = Span::new(6, 11, 1, 7);
        let fspan = FileSpan::new(id, span);
        assert_eq!(reg.extract(&fspan), "world");
    }
    #[test]
    fn test_file_span_merge() {
        let id = FileId::new(1);
        let a = FileSpan::new(id, Span::new(0, 5, 1, 1));
        let b = FileSpan::new(id, Span::new(3, 8, 1, 4));
        let merged = a.merge_with(&b);
        assert_eq!(merged.span.start, 0);
        assert_eq!(merged.span.end, 8);
    }
    #[test]
    fn test_line_col_to_offset_basic() {
        let src = "abc\ndef\nghi";
        assert_eq!(line_col_to_offset(src, 1, 1), Some(0));
        assert_eq!(line_col_to_offset(src, 2, 1), Some(4));
    }
    #[test]
    fn test_shift_spans() {
        let mut spans = vec![Span::new(0, 5, 1, 1), Span::new(10, 15, 1, 11)];
        shift_spans(&mut spans, 8, 3);
        assert_eq!(spans[0].start, 0);
        assert_eq!(spans[1].start, 13);
    }
    #[test]
    fn test_split_span_basic() {
        let span = Span::new(0, 10, 1, 1);
        let (left, right) = split_span(&span, 5);
        assert_eq!(left.end, 5);
        assert_eq!(right.start, 5);
    }
    #[test]
    fn test_span_strictly_contains() {
        let outer = Span::new(0, 10, 1, 1);
        let inner = Span::new(2, 8, 1, 3);
        assert!(span_strictly_contains(&outer, &inner));
    }
    #[test]
    fn test_spans_adjacent() {
        let a = Span::new(0, 5, 1, 1);
        let b = Span::new(5, 10, 1, 6);
        assert!(spans_adjacent(&a, &b));
    }
    #[test]
    fn test_annotated_span() {
        let span = Span::new(0, 5, 1, 1);
        let a = AnnotatedSpan::new(span, "test");
        assert_eq!(a.annotation, "test");
        assert_eq!(a.len(), 5);
    }
    #[test]
    fn test_span_annotations_at_offset() {
        let mut anns = SpanAnnotations::new();
        anns.annotate(Span::new(0, 10, 1, 1), "first");
        anns.annotate(Span::new(5, 15, 1, 6), "second");
        let at7 = anns.at_offset(7);
        assert_eq!(at7.len(), 2);
    }
    #[test]
    fn test_incremental_tracker() {
        let mut tracker = IncrementalSpanTracker::new();
        tracker.track(Span::new(0, 5, 1, 1));
        tracker.track(Span::new(10, 15, 1, 11));
        tracker.apply_edit(8, 3);
        assert_eq!(tracker.spans()[1].start, 13);
        assert_eq!(tracker.edit_count(), 1);
    }
    #[test]
    fn test_span_origin() {
        assert_eq!(SpanOrigin::UserSource.kind_str(), "user");
        assert_eq!(SpanOrigin::Synthetic.kind_str(), "synthetic");
    }
    #[test]
    fn test_span_stats() {
        let spans = vec![Span::new(0, 5, 1, 1), Span::new(10, 20, 2, 1)];
        let stats = SpanStats::compute(&spans);
        assert_eq!(stats.count, 2);
        assert_eq!(stats.total_len, 15);
        assert_eq!(stats.avg_len(), 7);
    }
    #[test]
    fn test_span_chain() {
        let mut chain = SpanChain::new();
        chain.push(Span::new(0, 5, 1, 1));
        chain.push(Span::new(6, 10, 1, 7));
        let total = chain.total_span().expect("span should be present");
        assert_eq!(total.start, 0);
        assert_eq!(total.end, 10);
    }
    #[test]
    fn test_span_cmp() {
        let a = Span::new(0, 5, 1, 1);
        let b = Span::new(5, 10, 1, 6);
        assert_eq!(span_cmp(&a, &b), std::cmp::Ordering::Less);
    }
    #[test]
    fn test_span_distance_nonoverlap() {
        let a = Span::new(0, 5, 1, 1);
        let b = Span::new(10, 15, 1, 11);
        assert_eq!(span_distance(&a, &b), 5);
    }
    #[test]
    fn test_span_earlier_later() {
        let a = Span::new(0, 5, 1, 1);
        let b = Span::new(3, 12, 1, 4);
        assert_eq!(span_earlier(&a, &b).start, 0);
        assert_eq!(span_later(&a, &b).end, 12);
    }
    #[test]
    fn test_count_chars_unicode() {
        let src = "αβγδ";
        let span = Span::new(0, src.len(), 1, 1);
        assert_eq!(count_chars(src, &span), 4);
    }
    #[test]
    fn test_extract_context_window() {
        let src = "line1\nline2\nline3\nline4\nline5";
        let span = Span::new(12, 17, 3, 1);
        let window = extract_context_window(src, &span, 1);
        assert_eq!(window.len(), 3);
        assert_eq!(window[1].0, 3);
    }
    #[test]
    fn test_dedup_spans() {
        let mut spans = vec![
            Span::new(0, 5, 1, 1),
            Span::new(0, 5, 1, 1),
            Span::new(6, 10, 1, 7),
        ];
        dedup_spans(&mut spans);
        assert_eq!(spans.len(), 2);
    }
    #[test]
    fn test_sort_spans_reverse() {
        let mut spans = vec![Span::new(0, 5, 1, 1), Span::new(10, 15, 1, 11)];
        sort_spans_reverse(&mut spans);
        assert_eq!(spans[0].start, 10);
    }
    #[test]
    fn test_byte_col_to_utf16_ascii() {
        let line = "hello world";
        assert_eq!(byte_col_to_utf16(line, 5), 5);
    }
    #[test]
    fn test_span_debug_str() {
        let span = Span::new(1, 5, 2, 3);
        let s = span_debug_str(&span);
        assert!(s.contains("start: 1"));
    }
    #[test]
    fn test_span_center() {
        let span = Span::new(2, 8, 1, 3);
        assert_eq!(span_center(&span), 5);
    }
    #[test]
    fn test_sort_spans_by_end() {
        let mut spans = vec![Span::new(5, 15, 1, 6), Span::new(0, 8, 1, 1)];
        sort_spans_by_end(&mut spans);
        assert_eq!(spans[0].end, 8);
    }
}
/// Check if spans are sorted by start offset.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn spans_sorted_check(spans: &[Span]) -> bool {
    spans.windows(2).all(|w| w[0].start <= w[1].start)
}
/// Return the total byte coverage of a set of spans (counting overlaps once).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn total_coverage(spans: &[Span]) -> usize {
    if spans.is_empty() {
        return 0;
    }
    let mut sorted = spans.to_vec();
    sort_spans(&mut sorted);
    let coalesced = coalesce_spans(&sorted);
    coalesced.iter().map(span_len).sum()
}
/// Given a list of prioritized spans, return the one with the highest priority at .
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn highest_priority_at(spans: &[PrioritizedSpan], offset: usize) -> Option<&PrioritizedSpan> {
    spans
        .iter()
        .filter(|ps| span_contains(&ps.span, offset))
        .max_by_key(|ps| ps.priority)
}
#[cfg(test)]
mod coverage_extra_tests {
    use super::*;
    use crate::span_util::*;
    #[test]
    fn test_total_coverage_no_overlap() {
        let spans = vec![Span::new(0, 5, 1, 1), Span::new(10, 15, 1, 11)];
        assert_eq!(total_coverage(&spans), 10);
    }
    #[test]
    fn test_total_coverage_overlap() {
        let spans = vec![Span::new(0, 10, 1, 1), Span::new(5, 15, 1, 6)];
        assert_eq!(total_coverage(&spans), 15);
    }
    #[test]
    fn test_total_coverage_empty() {
        assert_eq!(total_coverage(&[]), 0);
    }
    #[test]
    fn test_spans_sorted_check_true() {
        let sorted = vec![Span::new(0, 5, 1, 1), Span::new(6, 10, 1, 7)];
        assert!(spans_sorted_check(&sorted));
    }
    #[test]
    fn test_spans_sorted_check_false() {
        let unsorted = vec![Span::new(6, 10, 1, 7), Span::new(0, 5, 1, 1)];
        assert!(!spans_sorted_check(&unsorted));
    }
    #[test]
    fn test_highest_priority_at() {
        let spans = vec![
            PrioritizedSpan::new(Span::new(0, 10, 1, 1), 1),
            PrioritizedSpan::new(Span::new(3, 7, 1, 4), 5),
        ];
        let best = highest_priority_at(&spans, 5).expect("span should be present");
        assert_eq!(best.priority, 5);
    }
    #[test]
    fn test_highest_priority_at_none() {
        let spans = vec![PrioritizedSpan::new(Span::new(0, 5, 1, 1), 1)];
        assert!(highest_priority_at(&spans, 10).is_none());
    }
    #[test]
    fn test_span_range() {
        let r = SpanRange::new(2, 7);
        assert_eq!(r.len(), 5);
        assert!(r.contains(4));
        assert!(!r.contains(7));
    }
    #[test]
    fn test_padded_span() {
        let inner = Span::new(5, 10, 1, 6);
        let padded = PaddedSpan::new(inner, 3, 2);
        let expanded = padded.expanded(100);
        assert_eq!(expanded.start, 2);
        assert_eq!(expanded.end, 12);
    }
    #[test]
    fn test_padded_span_clamp() {
        let inner = Span::new(0, 5, 1, 1);
        let padded = PaddedSpan::new(inner, 10, 100);
        let expanded = padded.expanded(8);
        assert_eq!(expanded.start, 0);
        assert_eq!(expanded.end, 8);
    }
}
