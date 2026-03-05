//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{BidiMapper, SortedSourceMap, SourceMapDiff};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sourcemap::*;
    use crate::tokens::Span;
    fn mk_span(start: usize, end: usize, line: usize, col: usize) -> Span {
        Span::new(start, end, line, col)
    }
    #[test]
    fn test_line_offsets_single_line() {
        let sm = SourceMap::new("hello world");
        assert_eq!(sm.line_count(), 1);
        assert_eq!(sm.line_content(1), "hello world");
    }
    #[test]
    fn test_line_offsets_multi_line() {
        let sm = SourceMap::new("line1\nline2\nline3");
        assert_eq!(sm.line_count(), 3);
        assert_eq!(sm.line_content(1), "line1");
        assert_eq!(sm.line_content(2), "line2");
        assert_eq!(sm.line_content(3), "line3");
    }
    #[test]
    fn test_line_content_out_of_bounds() {
        let sm = SourceMap::new("hello");
        assert_eq!(sm.line_content(0), "");
        assert_eq!(sm.line_content(99), "");
    }
    #[test]
    fn test_offset_to_position_first_line() {
        let sm = SourceMap::new("hello\nworld");
        assert_eq!(sm.offset_to_position(0), (1, 1));
        assert_eq!(sm.offset_to_position(4), (1, 5));
    }
    #[test]
    fn test_offset_to_position_second_line() {
        let sm = SourceMap::new("hello\nworld");
        assert_eq!(sm.offset_to_position(6), (2, 1));
        assert_eq!(sm.offset_to_position(10), (2, 5));
    }
    #[test]
    fn test_position_to_offset_roundtrip() {
        let sm = SourceMap::new("abc\ndef\nghi");
        for offset in 0..11 {
            let (line, col) = sm.offset_to_position(offset);
            let back = sm.position_to_offset(line, col);
            assert_eq!(back, offset, "roundtrip failed for offset {}", offset);
        }
    }
    #[test]
    fn test_position_to_offset_clamped() {
        let sm = SourceMap::new("abc");
        let off = sm.position_to_offset(1, 100);
        assert_eq!(off, 3);
    }
    #[test]
    fn test_builder_basic() {
        let mut builder = SourceMapBuilder::new("def foo := 42");
        builder.add_definition(mk_span(4, 7, 1, 5), "foo");
        builder.add_literal(mk_span(11, 13, 1, 12));
        let sm = builder.build();
        assert_eq!(sm.entries().len(), 2);
    }
    #[test]
    fn test_builder_add_reference() {
        let mut builder = SourceMapBuilder::new("foo + bar");
        builder.add_reference(mk_span(0, 3, 1, 1), "foo");
        builder.add_reference(mk_span(6, 9, 1, 7), "bar");
        let sm = builder.build();
        let refs = sm.references_to("foo");
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].span.start, 0);
    }
    #[test]
    fn test_builder_entry_count() {
        let mut builder = SourceMapBuilder::new("test");
        assert_eq!(builder.entry_count(), 0);
        builder.add_keyword(mk_span(0, 4, 1, 1));
        assert_eq!(builder.entry_count(), 1);
    }
    #[test]
    fn test_builder_sorted_output() {
        let mut builder = SourceMapBuilder::new("a b c");
        builder.add_entry(
            mk_span(4, 5, 1, 5),
            EntryKind::Reference,
            Some("c".to_string()),
        );
        builder.add_entry(
            mk_span(0, 1, 1, 1),
            EntryKind::Reference,
            Some("a".to_string()),
        );
        builder.add_entry(
            mk_span(2, 3, 1, 3),
            EntryKind::Reference,
            Some("b".to_string()),
        );
        let sm = builder.build();
        let entries = sm.entries();
        assert!(entries[0].span.start <= entries[1].span.start);
        assert!(entries[1].span.start <= entries[2].span.start);
    }
    #[test]
    fn test_entry_at() {
        let mut builder = SourceMapBuilder::new("def foo := 42");
        builder.add_keyword(mk_span(0, 3, 1, 1));
        builder.add_definition(mk_span(4, 7, 1, 5), "foo");
        builder.add_literal(mk_span(11, 13, 1, 12));
        let sm = builder.build();
        let kw = sm.entry_at(1, 2);
        assert!(kw.is_some());
        assert_eq!(
            kw.expect("test operation should succeed").kind,
            EntryKind::Keyword
        );
        let def = sm.entry_at(1, 5);
        assert!(def.is_some());
        assert_eq!(
            def.expect("type conversion should succeed").name.as_deref(),
            Some("foo")
        );
    }
    #[test]
    fn test_entry_at_empty() {
        let sm = SourceMap::new("hello");
        assert!(sm.entry_at(1, 1).is_none());
    }
    #[test]
    fn test_definitions_query() {
        let mut builder = SourceMapBuilder::new("def a def b");
        builder.add_definition(mk_span(4, 5, 1, 5), "a");
        builder.add_definition(mk_span(10, 11, 1, 11), "b");
        builder.add_reference(mk_span(0, 1, 1, 1), "a");
        let sm = builder.build();
        let defs = sm.definitions();
        assert_eq!(defs.len(), 2);
    }
    #[test]
    fn test_references_to() {
        let mut builder = SourceMapBuilder::new("foo foo bar foo");
        builder.add_reference(mk_span(0, 3, 1, 1), "foo");
        builder.add_reference(mk_span(4, 7, 1, 5), "foo");
        builder.add_reference(mk_span(8, 11, 1, 9), "bar");
        builder.add_reference(mk_span(12, 15, 1, 13), "foo");
        let sm = builder.build();
        assert_eq!(sm.references_to("foo").len(), 3);
        assert_eq!(sm.references_to("bar").len(), 1);
        assert_eq!(sm.references_to("baz").len(), 0);
    }
    #[test]
    fn test_entries_in_range() {
        let mut builder = SourceMapBuilder::new("abcdefghij");
        builder.add_entry(
            mk_span(0, 3, 1, 1),
            EntryKind::Reference,
            Some("a".to_string()),
        );
        builder.add_entry(
            mk_span(4, 7, 1, 5),
            EntryKind::Reference,
            Some("b".to_string()),
        );
        builder.add_entry(
            mk_span(8, 10, 1, 9),
            EntryKind::Reference,
            Some("c".to_string()),
        );
        let sm = builder.build();
        let range_start = mk_span(2, 2, 1, 3);
        let range_end = mk_span(6, 6, 1, 7);
        let in_range = sm.entries_in_range(&range_start, &range_end);
        assert_eq!(in_range.len(), 2);
    }
    #[test]
    fn test_hover_info_on_definition() {
        let mut builder = SourceMapBuilder::new("def foo := 42");
        builder.add_definition_with_type(mk_span(4, 7, 1, 5), "foo", "Nat");
        let sm = builder.build();
        let hover = sm.hover_info(1, 5);
        assert!(hover.is_some());
        let info = hover.expect("test operation should succeed");
        assert_eq!(info.name, "foo");
        assert_eq!(info.kind, EntryKind::Definition);
        assert_eq!(info.ty.as_deref(), Some("Nat"));
        assert!(info.definition_span.is_some());
    }
    #[test]
    fn test_hover_info_on_reference() {
        let mut builder = SourceMapBuilder::new("def foo := foo");
        builder.add_definition(mk_span(4, 7, 1, 5), "foo");
        builder.add_reference(mk_span(11, 14, 1, 12), "foo");
        let sm = builder.build();
        let hover = sm.hover_info(1, 12);
        assert!(hover.is_some());
        let info = hover.expect("test operation should succeed");
        assert_eq!(info.name, "foo");
        assert_eq!(info.kind, EntryKind::Reference);
        assert!(info.definition_span.is_some());
        assert_eq!(
            info.definition_span.expect("span should be present").start,
            4
        );
    }
    #[test]
    fn test_hover_info_no_entry() {
        let sm = SourceMap::new("hello");
        assert!(sm.hover_info(1, 1).is_none());
    }
    #[test]
    fn test_hover_info_with_doc_comment() {
        let source = "/-- A doc comment -/
def foo := 42";
        let mut builder = SourceMapBuilder::new(source);
        builder.add_doc_comment(mk_span(0, 20, 1, 1), "A doc comment");
        builder.add_definition(mk_span(25, 28, 2, 5), "foo");
        let sm = builder.build();
        let hover = sm.hover_info(2, 5);
        assert!(hover.is_some());
        let info = hover.expect("test operation should succeed");
        assert_eq!(info.name, "foo");
        assert_eq!(info.doc.as_deref(), Some("A doc comment"));
    }
    #[test]
    fn test_hover_info_doc_comment_separated_by_non_whitespace() {
        let source = "/-- doc -/ something def foo := 42";
        let mut builder = SourceMapBuilder::new(source);
        builder.add_doc_comment(mk_span(0, 10, 1, 1), "doc");
        builder.add_reference(mk_span(11, 20, 1, 12), "something");
        builder.add_definition(mk_span(25, 28, 1, 26), "foo");
        let sm = builder.build();
        let hover = sm.hover_info(1, 26);
        assert!(hover.is_some());
        let info = hover.expect("test operation should succeed");
        assert!(info.doc.is_none());
    }
    #[test]
    fn test_hover_info_doc_comment_via_reference() {
        let source = "/-- bar docs -/
def bar := 0
bar";
        let mut builder = SourceMapBuilder::new(source);
        builder.add_doc_comment(mk_span(0, 15, 1, 1), "bar docs");
        builder.add_definition(mk_span(20, 23, 2, 5), "bar");
        builder.add_reference(mk_span(29, 32, 3, 1), "bar");
        let sm = builder.build();
        let hover = sm.hover_info(3, 1);
        assert!(hover.is_some());
        let info = hover.expect("test operation should succeed");
        assert_eq!(info.name, "bar");
        assert_eq!(info.doc.as_deref(), Some("bar docs"));
    }
    #[test]
    fn test_semantic_tokens_from_entries() {
        let mut builder = SourceMapBuilder::new("def foo := 42");
        builder.add_keyword(mk_span(0, 3, 1, 1));
        builder.add_definition(mk_span(4, 7, 1, 5), "foo");
        builder.add_literal(mk_span(11, 13, 1, 12));
        let sm = builder.build();
        let tokens = sm.semantic_tokens();
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token_type, SemanticTokenType::Keyword);
        assert_eq!(tokens[1].token_type, SemanticTokenType::Function);
        assert!(tokens[1].modifiers.contains(&SemanticModifier::Definition));
        assert_eq!(tokens[2].token_type, SemanticTokenType::Number);
    }
    #[test]
    fn test_semantic_tokens_precomputed() {
        let mut builder = SourceMapBuilder::new("x");
        builder.add_semantic_token(SemanticToken::new(
            mk_span(0, 1, 1, 1),
            SemanticTokenType::Variable,
        ));
        let sm = builder.build();
        let tokens = sm.semantic_tokens();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].token_type, SemanticTokenType::Variable);
    }
    #[test]
    fn test_go_to_definition() {
        let mut builder = SourceMapBuilder::new("def foo := foo");
        builder.add_definition(mk_span(4, 7, 1, 5), "foo");
        builder.add_reference(mk_span(11, 14, 1, 12), "foo");
        let sm = builder.build();
        let def_span = sm.go_to_definition(1, 12);
        assert!(def_span.is_some());
        assert_eq!(def_span.expect("span should be present").start, 4);
    }
    #[test]
    fn test_find_all_occurrences() {
        let mut builder = SourceMapBuilder::new("def foo := foo + foo");
        builder.add_definition(mk_span(4, 7, 1, 5), "foo");
        builder.add_reference(mk_span(11, 14, 1, 12), "foo");
        builder.add_reference(mk_span(17, 20, 1, 18), "foo");
        let sm = builder.build();
        let all = sm.find_all_occurrences(1, 5);
        assert_eq!(all.len(), 3);
    }
    #[test]
    fn test_span_text() {
        let sm = SourceMap::new("hello world");
        let span = mk_span(6, 11, 1, 7);
        assert_eq!(sm.span_text(&span), "world");
    }
    #[test]
    fn test_source_entry_contains_offset() {
        let entry = SourceEntry::new(mk_span(5, 10, 1, 6), EntryKind::Reference);
        assert!(!entry.contains_offset(4));
        assert!(entry.contains_offset(5));
        assert!(entry.contains_offset(9));
        assert!(!entry.contains_offset(10));
    }
    #[test]
    fn test_source_entry_with_name() {
        let entry = SourceEntry::with_name(mk_span(0, 3, 1, 1), EntryKind::Definition, "foo");
        assert_eq!(entry.name.as_deref(), Some("foo"));
        assert!(entry.ty_info.is_none());
    }
    #[test]
    fn test_source_entry_with_name_and_type() {
        let entry = SourceEntry::with_name_and_type(
            mk_span(0, 3, 1, 1),
            EntryKind::Definition,
            "foo",
            "Nat",
        );
        assert_eq!(entry.name.as_deref(), Some("foo"));
        assert_eq!(entry.ty_info.as_deref(), Some("Nat"));
    }
    #[test]
    fn test_entries_of_kind() {
        let mut builder = SourceMapBuilder::new("test");
        builder.add_keyword(mk_span(0, 1, 1, 1));
        builder.add_keyword(mk_span(1, 2, 1, 2));
        builder.add_literal(mk_span(2, 3, 1, 3));
        let sm = builder.build();
        assert_eq!(sm.entries_of_kind(&EntryKind::Keyword).len(), 2);
        assert_eq!(sm.entries_of_kind(&EntryKind::Literal).len(), 1);
        assert_eq!(sm.entries_of_kind(&EntryKind::Definition).len(), 0);
    }
    #[test]
    fn test_builder_add_various_kinds() {
        let mut builder = SourceMapBuilder::new("test source");
        builder.add_binder(mk_span(0, 1, 1, 1), "x");
        builder.add_constructor(mk_span(1, 2, 1, 2), "Foo");
        builder.add_operator(mk_span(2, 3, 1, 3), "+");
        builder.add_comment(mk_span(3, 4, 1, 4));
        builder.add_doc_comment(mk_span(4, 5, 1, 5), "docs");
        builder.add_tactic(mk_span(5, 6, 1, 6), "simp");
        builder.add_pattern(mk_span(6, 7, 1, 7), Some("pat"));
        builder.add_type_annotation(mk_span(7, 8, 1, 8));
        let sm = builder.build();
        assert_eq!(sm.entries().len(), 8);
    }
    #[test]
    fn test_semantic_token_constructors() {
        let tok = SemanticToken::new(mk_span(0, 1, 1, 1), SemanticTokenType::Keyword);
        assert!(tok.modifiers.is_empty());
        let tok2 = SemanticToken::with_modifiers(
            mk_span(0, 1, 1, 1),
            SemanticTokenType::Function,
            vec![SemanticModifier::Definition, SemanticModifier::Declaration],
        );
        assert_eq!(tok2.modifiers.len(), 2);
    }
}
#[cfg(test)]
mod extended_sourcemap_tests {
    use super::*;
    use crate::sourcemap::*;
    use crate::tokens::Span;
    fn mk_span(start: usize, end: usize, line: usize, col: usize) -> Span {
        Span::new(start, end, line, col)
    }
    #[test]
    fn test_source_position_origin() {
        let pos = SourcePosition::origin();
        assert_eq!(pos.offset, 0);
        assert_eq!(pos.line, 1);
        assert_eq!(pos.column, 1);
    }
    #[test]
    fn test_source_position_advance_col() {
        let pos = SourcePosition::origin().advance_col(1);
        assert_eq!(pos.offset, 1);
        assert_eq!(pos.column, 2);
        assert_eq!(pos.line, 1);
    }
    #[test]
    fn test_source_position_advance_line() {
        let pos = SourcePosition::origin().advance_line(1);
        assert_eq!(pos.line, 2);
        assert_eq!(pos.column, 1);
    }
    #[test]
    fn test_source_position_ordering() {
        let a = SourcePosition::new(5, 1, 5);
        let b = SourcePosition::new(10, 2, 3);
        assert!(a.is_before(&b));
        assert!(b.is_after(&a));
    }
    #[test]
    fn test_source_region_contains() {
        let span = mk_span(10, 20, 1, 10);
        let region = SourceRegion::new("body", span);
        assert!(region.contains_offset(15));
        assert!(!region.contains_offset(25));
        assert_eq!(region.byte_len(), 10);
    }
    #[test]
    fn test_definition_index_basic() {
        let mut idx = DefinitionIndex::new();
        idx.register("foo", mk_span(0, 3, 1, 1));
        idx.register("foo", mk_span(10, 13, 2, 1));
        assert_eq!(idx.lookup("foo").len(), 2);
        assert!(idx.contains("foo"));
        assert!(!idx.contains("bar"));
    }
    #[test]
    fn test_definition_index_merge() {
        let mut a = DefinitionIndex::new();
        a.register("foo", mk_span(0, 3, 1, 1));
        let mut b = DefinitionIndex::new();
        b.register("bar", mk_span(5, 8, 2, 1));
        a.merge(b);
        assert_eq!(a.len(), 2);
        assert!(a.contains("foo"));
        assert!(a.contains("bar"));
    }
    #[test]
    fn test_reference_index_basic() {
        let mut idx = ReferenceIndex::new();
        idx.record("add", mk_span(0, 3, 1, 1));
        idx.record("add", mk_span(10, 13, 2, 1));
        idx.record("sub", mk_span(5, 8, 1, 5));
        assert_eq!(idx.uses_of("add").len(), 2);
        assert_eq!(idx.total_references(), 3);
    }
    #[test]
    fn test_document_symbol_flatten() {
        let mut sym = DocumentSymbol::new(
            "Foo".to_string(),
            SymbolKind::Structure,
            mk_span(0, 100, 1, 1),
            mk_span(0, 3, 1, 1),
        );
        let child = DocumentSymbol::new(
            "bar".to_string(),
            SymbolKind::Field,
            mk_span(10, 20, 2, 3),
            mk_span(10, 13, 2, 3),
        );
        sym.add_child(child);
        let flat = sym.flatten();
        assert_eq!(flat.len(), 2);
        assert_eq!(flat[0].name, "Foo");
        assert_eq!(flat[1].name, "bar");
    }
    #[test]
    fn test_document_symbol_find_by_name() {
        let mut sym = DocumentSymbol::new(
            "Root".to_string(),
            SymbolKind::Namespace,
            mk_span(0, 200, 1, 1),
            mk_span(0, 4, 1, 1),
        );
        let inner = DocumentSymbol::new(
            "inner_fn".to_string(),
            SymbolKind::Definition,
            mk_span(10, 50, 2, 1),
            mk_span(10, 18, 2, 1),
        );
        sym.add_child(inner);
        assert!(sym.find_by_name("inner_fn").is_some());
        assert!(sym.find_by_name("missing").is_none());
    }
    #[test]
    fn test_source_index_symbol_at_offset() {
        let mut idx = SourceIndex::new();
        let sym = DocumentSymbol::new(
            "myDef".to_string(),
            SymbolKind::Definition,
            mk_span(5, 50, 1, 5),
            mk_span(5, 10, 1, 5),
        );
        idx.add_symbol(sym);
        assert!(idx.symbol_at_offset(10).is_some());
        assert!(idx.symbol_at_offset(100).is_none());
    }
    #[test]
    fn test_goto_definition_result() {
        let spans = vec![mk_span(0, 5, 1, 1)];
        let result = GoToDefinitionResult::new("foo".to_string(), spans, true);
        assert!(result.found());
        assert!(result.is_local);
        assert_eq!(
            result.primary_span().expect("span should be present").start,
            0
        );
    }
    #[test]
    fn test_goto_definition_not_found() {
        let result = GoToDefinitionResult::new("unknown".to_string(), vec![], false);
        assert!(!result.found());
        assert!(result.primary_span().is_none());
    }
    #[test]
    fn test_diagnostic_severity_ordering() {
        assert!(DiagnosticSeverity::Hint < DiagnosticSeverity::Warning);
        assert!(DiagnosticSeverity::Warning < DiagnosticSeverity::Error);
    }
    #[test]
    fn test_source_diagnostic_constructors() {
        let span = mk_span(0, 5, 1, 1);
        let err = SourceDiagnostic::error(span.clone(), "bad token");
        assert!(err.is_error());
        assert!(!err.is_warning());
        let warn = SourceDiagnostic::warning(span, "maybe bad");
        assert!(warn.is_warning());
    }
    #[test]
    fn test_source_diagnostic_with_code() {
        let span = mk_span(0, 5, 1, 1);
        let diag = SourceDiagnostic::error(span, "msg").with_code("E001");
        assert_eq!(diag.code.as_deref(), Some("E001"));
    }
    #[test]
    fn test_source_map_stats() {
        let entries = vec![
            SourceEntry::new(mk_span(0, 1, 1, 1), EntryKind::Definition),
            SourceEntry::new(mk_span(1, 2, 1, 2), EntryKind::Reference),
            SourceEntry::new(mk_span(2, 3, 1, 3), EntryKind::Tactic),
            SourceEntry::new(mk_span(3, 4, 1, 4), EntryKind::Comment),
            SourceEntry::new(mk_span(4, 5, 1, 5), EntryKind::Operator),
        ];
        let stats = SourceMapStats::from_entries(&entries);
        assert_eq!(stats.total_entries, 5);
        assert_eq!(stats.definitions, 1);
        assert_eq!(stats.references, 1);
        assert_eq!(stats.tactics, 1);
        assert_eq!(stats.comments, 1);
        assert_eq!(stats.operators, 1);
    }
}
#[cfg(test)]
mod sourcemap_ext_tests {
    use super::*;
    use crate::sourcemap::*;
    use crate::tokens::Span;
    #[test]
    fn test_bidi_mapper() {
        let mut m = BidiMapper::new();
        m.add(0, 0);
        m.add(5, 10);
        m.add(10, 20);
        assert_eq!(m.to_gen(0), Some(0));
        assert_eq!(m.to_gen(7), Some(10));
        assert_eq!(m.to_orig(15), Some(5));
    }
    #[test]
    fn test_range_transform() {
        let rt = RangeTransform::new(0, 10, 0, 10);
        assert!(rt.is_length_preserving());
        let rt2 = RangeTransform::new(0, 10, 0, 20);
        assert!(!rt2.is_length_preserving());
    }
    #[test]
    fn test_map_chain() {
        let mut m1 = BidiMapper::new();
        m1.add(0, 0);
        m1.add(5, 5);
        let mut m2 = BidiMapper::new();
        m2.add(0, 0);
        m2.add(5, 10);
        let chain = MapChain::new(m1, m2);
        let c = chain.a_to_c(5);
        assert_eq!(c, Some(10));
    }
}
/// Compare two BidiMappers and return a diff.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn diff_mappers(old: &BidiMapper, new: &BidiMapper) -> SourceMapDiff {
    let old_set: std::collections::HashSet<(usize, usize)> = old.forward.iter().cloned().collect();
    let new_set: std::collections::HashSet<(usize, usize)> = new.forward.iter().cloned().collect();
    let added = new_set.difference(&old_set).count();
    let removed = old_set.difference(&new_set).count();
    SourceMapDiff {
        added,
        removed,
        modified: 0,
    }
}
#[cfg(test)]
mod sourcemap_ext2_tests {
    use super::*;
    use crate::sourcemap::*;
    use crate::tokens::Span;
    #[test]
    fn test_source_map_cache() {
        let mut cache = SourceMapCache::new();
        cache.insert(1, 5, 10, 3);
        assert_eq!(cache.lookup(1, 5), Some((10, 3)));
        assert_eq!(cache.lookup(2, 0), None);
    }
    #[test]
    fn test_diff_mappers() {
        let mut m1 = BidiMapper::new();
        m1.add(0, 0);
        m1.add(5, 10);
        let mut m2 = BidiMapper::new();
        m2.add(0, 0);
        m2.add(6, 12);
        let diff = diff_mappers(&m1, &m2);
        assert_eq!(diff.added, 1);
        assert_eq!(diff.removed, 1);
    }
}
#[cfg(test)]
mod sorted_sourcemap_tests {
    use super::*;
    use crate::sourcemap::*;
    use crate::tokens::Span;
    #[test]
    fn test_sorted_source_map() {
        let mut sm = SortedSourceMap::new();
        sm.add(0, 0);
        sm.add(10, 5);
        sm.add(20, 15);
        assert_eq!(sm.lookup(0), Some(0));
        assert_eq!(sm.lookup(15), Some(5));
        assert_eq!(sm.lookup(100), Some(15));
    }
}
/// A source map entry formatter (JSON-like).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn format_map_entry_json(
    orig_line: usize,
    orig_col: usize,
    gen_line: usize,
    gen_col: usize,
) -> String {
    format!(
        r#"{{"origLine":{},"origCol":{},"genLine":{},"genCol":{}}}"#,
        orig_line, orig_col, gen_line, gen_col
    )
}
#[cfg(test)]
mod sourcemap_batch_tests {
    use super::*;
    use crate::sourcemap::*;
    use crate::tokens::Span;
    #[test]
    fn test_format_map_entry_json() {
        let s = format_map_entry_json(1, 0, 2, 5);
        assert!(s.contains("origLine"));
        assert!(s.contains("genLine"));
    }
    #[test]
    fn test_source_map_batch() {
        let mut batch = SourceMapBatch::new();
        batch.add(RangeTransform::new(0, 10, 0, 10));
        batch.add(RangeTransform::new(10, 20, 10, 25));
        assert_eq!(batch.total_coverage(), 20);
        assert_eq!(batch.length_preserving_count(), 1);
    }
}
/// Merges two sorted source maps.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn merge_sorted_maps(a: SortedSourceMap, b: SortedSourceMap) -> SortedSourceMap {
    let mut result = SortedSourceMap::new();
    for (gen, orig) in a.pairs.into_iter().chain(b.pairs) {
        result.add(gen, orig);
    }
    result
}
/// Returns true if a sorted source map has no duplicate gen offsets.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_injective(map: &SortedSourceMap) -> bool {
    let gens: Vec<usize> = map.pairs.iter().map(|(g, _)| *g).collect();
    let mut seen = std::collections::HashSet::new();
    gens.iter().all(|g| seen.insert(*g))
}
#[cfg(test)]
mod sourcemap_pad {
    use super::*;
    use crate::sourcemap::*;
    use crate::tokens::Span;
    #[test]
    fn test_merge_sorted_maps() {
        let mut a = SortedSourceMap::new();
        a.add(0, 0);
        let mut b = SortedSourceMap::new();
        b.add(5, 10);
        let merged = merge_sorted_maps(a, b);
        assert_eq!(merged.len(), 2);
    }
    #[test]
    fn test_is_injective() {
        let mut m = SortedSourceMap::new();
        m.add(0, 0);
        m.add(5, 10);
        assert!(is_injective(&m));
    }
}
/// Returns the total number of mappings in a sorted source map.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn total_mappings(m: &SortedSourceMap) -> usize {
    m.len()
}
/// Returns the maximum generated offset in a source map, if any.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn max_gen_offset(m: &SortedSourceMap) -> Option<usize> {
    m.pairs.iter().map(|(g, _)| *g).max()
}
/// Returns the minimum generated offset in a source map, if any.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn min_gen_offset(m: &SortedSourceMap) -> Option<usize> {
    m.pairs.iter().map(|(g, _)| *g).min()
}
/// Returns true if a source map covers a generated offset.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn covers_gen_offset(m: &SortedSourceMap, gen: usize) -> bool {
    m.pairs.iter().any(|(g, _)| *g == gen)
}
/// Returns all original offsets in a source map.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn all_orig_offsets(m: &SortedSourceMap) -> Vec<usize> {
    m.pairs.iter().map(|(_, o)| *o).collect()
}
#[cfg(test)]
mod sourcemap_pad2 {
    use super::*;
    use crate::sourcemap::*;
    use crate::tokens::Span;
    #[test]
    fn test_max_min_gen_offset() {
        let mut m = SortedSourceMap::new();
        m.add(5, 10);
        m.add(2, 4);
        m.add(8, 20);
        assert_eq!(max_gen_offset(&m), Some(8));
        assert_eq!(min_gen_offset(&m), Some(2));
    }
    #[test]
    fn test_covers_gen_offset() {
        let mut m = SortedSourceMap::new();
        m.add(3, 6);
        assert!(covers_gen_offset(&m, 3));
        assert!(!covers_gen_offset(&m, 4));
    }
}
/// Returns true if all generated offsets in the map are in ascending order.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_sorted_ascending(m: &SortedSourceMap) -> bool {
    m.pairs.windows(2).all(|w| w[0].0 <= w[1].0)
}
/// Filters a source map to only entries where the generated offset is >= lo and < hi.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn filter_gen_range(m: &SortedSourceMap, lo: usize, hi: usize) -> SortedSourceMap {
    let mut result = SortedSourceMap::new();
    for &(gen, orig) in &m.pairs {
        if gen >= lo && gen < hi {
            result.add(gen, orig);
        }
    }
    result
}
/// Returns the number of mappings that map to a specific original offset.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_to_orig(m: &SortedSourceMap, orig: usize) -> usize {
    m.pairs.iter().filter(|(_, o)| *o == orig).count()
}
