//! Functions for source map construction and querying.

use super::types::{LineColumn, Position, SourceId, SourceInfo, SourceKind, SourceMap, Span};

// ── SourceMap impl ───────────────────────────────────────────────────────────

impl SourceMap {
    /// Create an empty `SourceMap`.
    pub fn new() -> Self {
        Self {
            sources: Vec::new(),
            next_id: 0,
        }
    }

    /// Register a source file and return its [`SourceId`].
    pub fn add_file(&mut self, name: &str, content: String) -> SourceId {
        let id = SourceId(self.next_id);
        self.next_id += 1;
        self.sources.push(SourceInfo {
            id,
            name: name.to_owned(),
            content,
            kind: SourceKind::File,
        });
        id
    }

    /// Register a macro expansion and return its [`SourceId`].
    pub fn add_macro_expansion(
        &mut self,
        macro_name: &str,
        site: Span,
        content: String,
    ) -> SourceId {
        let id = SourceId(self.next_id);
        self.next_id += 1;
        self.sources.push(SourceInfo {
            id,
            name: format!("<macro:{}>", macro_name),
            content,
            kind: SourceKind::MacroExpansion {
                macro_name: macro_name.to_owned(),
                expansion_site: site,
            },
        });
        id
    }

    /// Look up a source by its identifier.
    pub fn get(&self, id: SourceId) -> Option<&SourceInfo> {
        self.sources.iter().find(|s| s.id == id)
    }

    /// Convert a byte-offset span to a [`LineColumn`].
    ///
    /// Returns the position of the *start* of the span.  Returns `None` if
    /// the source is unknown or the offset is out of range.
    pub fn span_to_position(&self, span: &Span) -> Option<LineColumn> {
        let info = self.get(span.source)?;
        let content = &info.content;
        if span.start > content.len() {
            return None;
        }
        let (line, col) = byte_offset_to_line_col(content, span.start);
        Some(LineColumn {
            source: span.source,
            position: Position::new(line, col),
        })
    }

    /// Convert a [`LineColumn`] back to a byte offset.
    ///
    /// Returns `None` if the source is unknown or the position is out of range.
    pub fn position_to_offset(&self, lc: &LineColumn) -> Option<usize> {
        let info = self.get(lc.source)?;
        line_col_to_byte_offset(&info.content, lc.position.line, lc.position.col)
    }

    /// Return the text slice covered by `span`.
    ///
    /// Returns `None` if the source is unknown or the span is out of range.
    pub fn span_text(&self, span: &Span) -> Option<&str> {
        let info = self.get(span.source)?;
        info.content.get(span.start..span.end)
    }

    /// Walk back through macro expansion sites and collect origin spans.
    ///
    /// If `span` belongs to a macro-expanded source, this function follows the
    /// expansion chain until it reaches a non-expanded source.  The returned
    /// vector goes from `span` (innermost) to the original user-written span
    /// (outermost).
    pub fn chain_origin(&self, span: &Span) -> Vec<Span> {
        let mut chain: Vec<Span> = vec![span.clone()];
        let mut current = span.clone();

        while let Some(info) = self.get(current.source) {
            match &info.kind {
                SourceKind::MacroExpansion { expansion_site, .. } => {
                    chain.push(expansion_site.clone());
                    current = expansion_site.clone();
                }
                _ => break,
            }
        }

        chain
    }
}

// ── Free span utilities ──────────────────────────────────────────────────────

/// Returns `true` if `inner` is entirely contained within `outer`.
///
/// Both spans must reference the same [`SourceId`] for this to return `true`.
pub fn span_contains(outer: &Span, inner: &Span) -> bool {
    outer.source == inner.source && outer.start <= inner.start && inner.end <= outer.end
}

/// Merge two spans into the smallest span that contains both.
///
/// Returns `None` if the spans belong to different sources.
pub fn merge_spans(a: &Span, b: &Span) -> Option<Span> {
    if a.source != b.source {
        return None;
    }
    Some(Span {
        source: a.source,
        start: a.start.min(b.start),
        end: a.end.max(b.end),
    })
}

// ── Internal helpers ─────────────────────────────────────────────────────────

/// Convert a byte offset in `content` to a 0-based `(line, col)` pair.
pub(super) fn byte_offset_to_line_col(content: &str, offset: usize) -> (u32, u32) {
    let safe = offset.min(content.len());
    let before = &content[..safe];
    let line = before.bytes().filter(|&b| b == b'\n').count() as u32;
    let col = before.rfind('\n').map(|nl| safe - nl - 1).unwrap_or(safe) as u32;
    (line, col)
}

/// Convert a 0-based `(line, col)` pair to a byte offset in `content`.
///
/// Returns `None` when the position is out of range.
pub(super) fn line_col_to_byte_offset(content: &str, line: u32, col: u32) -> Option<usize> {
    let mut current_line = 0u32;
    let mut line_start = 0usize;

    for (i, b) in content.bytes().enumerate() {
        if current_line == line {
            let col_offset = line_start + col as usize;
            if col_offset <= content.len() {
                return Some(col_offset);
            } else {
                return None;
            }
        }
        if b == b'\n' {
            current_line += 1;
            line_start = i + 1;
        }
    }

    // Handle last line (no trailing newline)
    if current_line == line {
        let col_offset = line_start + col as usize;
        if col_offset <= content.len() {
            return Some(col_offset);
        }
    }

    None
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_map::types::SourceKind;

    fn make_map_with_file(content: &str) -> (SourceMap, SourceId) {
        let mut sm = SourceMap::new();
        let id = sm.add_file("test.lean", content.to_owned());
        (sm, id)
    }

    // ── SourceMap::new ───────────────────────────────────────────────────────

    #[test]
    fn test_source_map_new_empty() {
        let sm = SourceMap::new();
        assert!(sm.sources.is_empty());
        assert_eq!(sm.next_id, 0);
    }

    // ── add_file ────────────────────────────────────────────────────────────

    #[test]
    fn test_add_file_assigns_ids_sequentially() {
        let mut sm = SourceMap::new();
        let a = sm.add_file("a.lean", "content a".into());
        let b = sm.add_file("b.lean", "content b".into());
        assert_eq!(a, SourceId(0));
        assert_eq!(b, SourceId(1));
    }

    #[test]
    fn test_add_file_stores_content() {
        let (sm, id) = make_map_with_file("hello world");
        let info = sm.get(id).expect("source not found");
        assert_eq!(info.content, "hello world");
        assert_eq!(info.name, "test.lean");
        assert!(matches!(info.kind, SourceKind::File));
    }

    // ── add_macro_expansion ─────────────────────────────────────────────────

    #[test]
    fn test_add_macro_expansion() {
        let mut sm = SourceMap::new();
        let file_id = sm.add_file("src.lean", "macro!(x)".into());
        let site = Span::new(file_id, 0, 9);
        let macro_id = sm.add_macro_expansion("macro", site, "expanded code".into());
        let info = sm.get(macro_id).expect("macro source not found");
        assert!(matches!(
            &info.kind,
            SourceKind::MacroExpansion { macro_name, .. } if macro_name == "macro"
        ));
    }

    // ── get ─────────────────────────────────────────────────────────────────

    #[test]
    fn test_get_unknown_id_returns_none() {
        let sm = SourceMap::new();
        assert!(sm.get(SourceId(999)).is_none());
    }

    // ── span_to_position ────────────────────────────────────────────────────

    #[test]
    fn test_span_to_position_first_char() {
        let (sm, id) = make_map_with_file("hello\nworld");
        let span = Span::new(id, 0, 1);
        let lc = sm.span_to_position(&span).expect("should resolve");
        assert_eq!(lc.position.line, 0);
        assert_eq!(lc.position.col, 0);
    }

    #[test]
    fn test_span_to_position_second_line() {
        let (sm, id) = make_map_with_file("hello\nworld");
        let span = Span::new(id, 6, 11); // 'world'
        let lc = sm.span_to_position(&span).expect("should resolve");
        assert_eq!(lc.position.line, 1);
        assert_eq!(lc.position.col, 0);
    }

    #[test]
    fn test_span_to_position_mid_line() {
        let (sm, id) = make_map_with_file("abcde\nfghij");
        let span = Span::new(id, 3, 4); // 'd'
        let lc = sm.span_to_position(&span).expect("should resolve");
        assert_eq!(lc.position.line, 0);
        assert_eq!(lc.position.col, 3);
    }

    #[test]
    fn test_span_to_position_out_of_range() {
        let (sm, id) = make_map_with_file("abc");
        let span = Span::new(id, 100, 110);
        assert!(sm.span_to_position(&span).is_none());
    }

    #[test]
    fn test_span_to_position_unknown_source() {
        let sm = SourceMap::new();
        let span = Span::new(SourceId(0), 0, 1);
        assert!(sm.span_to_position(&span).is_none());
    }

    // ── position_to_offset ──────────────────────────────────────────────────

    #[test]
    fn test_position_to_offset_roundtrip_single_line() {
        let (sm, id) = make_map_with_file("hello world");
        for col in 0_u32..=10 {
            let lc = LineColumn::new(id, 0, col);
            let off = sm.position_to_offset(&lc).expect("should resolve");
            let lc2 = sm
                .span_to_position(&Span::new(id, off, off + 1))
                .expect("roundtrip");
            assert_eq!(lc2.position.col, col, "col roundtrip failed for {}", col);
        }
    }

    #[test]
    fn test_position_to_offset_multi_line() {
        let content = "abc\ndef\nghi";
        let (sm, id) = make_map_with_file(content);
        // 'g' is at line 2, col 0, byte offset 8
        let lc = LineColumn::new(id, 2, 0);
        let off = sm.position_to_offset(&lc).expect("should resolve");
        assert_eq!(&content[off..off + 1], "g");
    }

    #[test]
    fn test_position_to_offset_out_of_range() {
        let (sm, id) = make_map_with_file("abc");
        let lc = LineColumn::new(id, 99, 0);
        assert!(sm.position_to_offset(&lc).is_none());
    }

    // ── span_text ───────────────────────────────────────────────────────────

    #[test]
    fn test_span_text_basic() {
        let (sm, id) = make_map_with_file("hello world");
        let span = Span::new(id, 6, 11);
        assert_eq!(sm.span_text(&span), Some("world"));
    }

    #[test]
    fn test_span_text_empty_span() {
        let (sm, id) = make_map_with_file("hello");
        let span = Span::new(id, 2, 2);
        assert_eq!(sm.span_text(&span), Some(""));
    }

    #[test]
    fn test_span_text_out_of_range() {
        let (sm, id) = make_map_with_file("hi");
        let span = Span::new(id, 1, 100);
        assert!(sm.span_text(&span).is_none());
    }

    #[test]
    fn test_span_text_unknown_source() {
        let sm = SourceMap::new();
        let span = Span::new(SourceId(0), 0, 1);
        assert!(sm.span_text(&span).is_none());
    }

    // ── chain_origin ────────────────────────────────────────────────────────

    #[test]
    fn test_chain_origin_file_only() {
        let (sm, id) = make_map_with_file("source");
        let span = Span::new(id, 0, 6);
        let chain = sm.chain_origin(&span);
        assert_eq!(chain.len(), 1);
        assert_eq!(chain[0], span);
    }

    #[test]
    fn test_chain_origin_one_expansion() {
        let mut sm = SourceMap::new();
        let file_id = sm.add_file("src.lean", "macro!(x)".into());
        let site = Span::new(file_id, 0, 9);
        let macro_id = sm.add_macro_expansion("macro", site.clone(), "expanded".into());
        let macro_span = Span::new(macro_id, 0, 8);
        let chain = sm.chain_origin(&macro_span);
        // Should have the macro span and the expansion site
        assert_eq!(chain.len(), 2);
        assert_eq!(chain[0], macro_span);
        assert_eq!(chain[1], site);
    }

    #[test]
    fn test_chain_origin_nested_expansions() {
        let mut sm = SourceMap::new();
        let file_id = sm.add_file("src.lean", "outer!(inner!(x))".into());
        let outer_site = Span::new(file_id, 0, 17);
        let mid_id = sm.add_macro_expansion("outer", outer_site.clone(), "mid content".into());
        let mid_site = Span::new(mid_id, 0, 11);
        let inner_id = sm.add_macro_expansion("inner", mid_site.clone(), "inner expanded".into());
        let inner_span = Span::new(inner_id, 0, 14);

        let chain = sm.chain_origin(&inner_span);
        assert_eq!(chain.len(), 3);
        assert_eq!(chain[0], inner_span);
        assert_eq!(chain[1], mid_site);
        assert_eq!(chain[2], outer_site);
    }

    // ── span_contains ───────────────────────────────────────────────────────

    #[test]
    fn test_span_contains_same_source() {
        let id = SourceId(0);
        let outer = Span::new(id, 0, 10);
        let inner = Span::new(id, 2, 8);
        assert!(span_contains(&outer, &inner));
    }

    #[test]
    fn test_span_contains_exact() {
        let id = SourceId(0);
        let span = Span::new(id, 5, 10);
        assert!(span_contains(&span, &span));
    }

    #[test]
    fn test_span_contains_not_contained() {
        let id = SourceId(0);
        let a = Span::new(id, 0, 5);
        let b = Span::new(id, 3, 8);
        assert!(!span_contains(&a, &b));
    }

    #[test]
    fn test_span_contains_different_sources() {
        let a_span = Span::new(SourceId(0), 0, 10);
        let b_span = Span::new(SourceId(1), 2, 8);
        assert!(!span_contains(&a_span, &b_span));
    }

    // ── merge_spans ─────────────────────────────────────────────────────────

    #[test]
    fn test_merge_spans_same_source() {
        let id = SourceId(0);
        let a = Span::new(id, 2, 5);
        let b = Span::new(id, 4, 9);
        let merged = merge_spans(&a, &b).expect("should merge");
        assert_eq!(merged.start, 2);
        assert_eq!(merged.end, 9);
    }

    #[test]
    fn test_merge_spans_disjoint() {
        let id = SourceId(0);
        let a = Span::new(id, 0, 3);
        let b = Span::new(id, 7, 10);
        let merged = merge_spans(&a, &b).expect("should merge");
        assert_eq!(merged.start, 0);
        assert_eq!(merged.end, 10);
    }

    #[test]
    fn test_merge_spans_different_sources() {
        let a = Span::new(SourceId(0), 0, 5);
        let b = Span::new(SourceId(1), 0, 5);
        assert!(merge_spans(&a, &b).is_none());
    }

    #[test]
    fn test_merge_spans_identical() {
        let id = SourceId(0);
        let span = Span::new(id, 3, 7);
        let merged = merge_spans(&span, &span).expect("merge");
        assert_eq!(merged, span);
    }

    // ── Span helpers ─────────────────────────────────────────────────────────

    #[test]
    fn test_span_len() {
        let span = Span::new(SourceId(0), 4, 9);
        assert_eq!(span.len(), 5);
    }

    #[test]
    fn test_span_is_empty() {
        let empty = Span::new(SourceId(0), 5, 5);
        let non_empty = Span::new(SourceId(0), 5, 6);
        assert!(empty.is_empty());
        assert!(!non_empty.is_empty());
    }

    // ── SpanChain helpers ────────────────────────────────────────────────────

    #[test]
    fn test_span_chain_depth() {
        let mut chain = crate::source_map::types::SpanChain::new();
        assert_eq!(chain.depth(), 0);
        chain.push(Span::new(SourceId(0), 0, 1));
        assert_eq!(chain.depth(), 1);
    }

    #[test]
    fn test_span_chain_outermost_innermost() {
        let mut chain = crate::source_map::types::SpanChain::new();
        let a = Span::new(SourceId(0), 0, 1);
        let b = Span::new(SourceId(1), 2, 3);
        chain.push(a.clone());
        chain.push(b.clone());
        assert_eq!(chain.outermost(), Some(&a));
        assert_eq!(chain.innermost(), Some(&b));
    }

    // ── byte_offset_to_line_col / line_col_to_byte_offset roundtrip ─────────

    #[test]
    fn test_offset_line_col_roundtrip() {
        let content = "abc\ndef\nghi";
        for off in 0..content.len() {
            let (line, col) = byte_offset_to_line_col(content, off);
            let back = line_col_to_byte_offset(content, line, col).expect("roundtrip");
            assert_eq!(back, off, "roundtrip failed for offset {}", off);
        }
    }

    // ── Position display ─────────────────────────────────────────────────────

    #[test]
    fn test_position_display_1_based() {
        let p = Position::new(0, 0);
        assert_eq!(format!("{}", p), "1:1");
        let p2 = Position::new(2, 4);
        assert_eq!(format!("{}", p2), "3:5");
    }
}
