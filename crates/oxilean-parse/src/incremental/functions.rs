//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    DirtyRegion, InvalidatedRange, LineDiff, ReparseRequest, SourceEdit, TextChange,
};

/// Compute the Levenshtein edit distance between two strings.
#[allow(missing_docs)]
pub fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for (i, row) in dp.iter_mut().enumerate().take(m + 1) {
        row[0] = i;
    }
    for (j, cell) in dp[0].iter_mut().enumerate().take(n + 1) {
        *cell = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i - 1] == b[j - 1] {
                dp[i - 1][j - 1]
            } else {
                1 + dp[i - 1][j - 1].min(dp[i - 1][j]).min(dp[i][j - 1])
            };
        }
    }
    dp[m][n]
}
/// Compute a minimal set of `TextChange` ops that transforms `old` into `new`.
#[allow(missing_docs)]
pub fn diff_lines(old: &str, new: &str) -> Vec<TextChange> {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();
    let mut changes = Vec::new();
    let min_len = old_lines.len().min(new_lines.len());
    let first_diff = (0..min_len).find(|&i| old_lines[i] != new_lines[i]);
    if let Some(idx) = first_diff {
        let old_start: usize = old_lines[..idx].iter().map(|l| l.len() + 1).sum();
        let old_end: usize = old_lines[..old_lines.len()]
            .iter()
            .map(|l| l.len() + 1)
            .sum();
        let new_text: String = new_lines[idx..].join("\n");
        changes.push(TextChange::replacement(old_start, old_end, new_text));
    } else if old_lines.len() > new_lines.len() {
        let del_start: usize = old_lines[..new_lines.len()]
            .iter()
            .map(|l| l.len() + 1)
            .sum();
        let del_end = old.len();
        changes.push(TextChange::deletion(del_start, del_end));
    } else if new_lines.len() > old_lines.len() {
        let ins_at = old.len();
        let extra: String = "\n".to_string() + &new_lines[old_lines.len()..].join("\n");
        changes.push(TextChange::insertion(ins_at, extra));
    }
    changes
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::incremental::*;
    #[test]
    fn test_text_change_apply_insertion() {
        let change = TextChange::insertion(5, " world");
        let result = change.apply("hello");
        assert_eq!(result, "hello world");
    }
    #[test]
    fn test_text_change_apply_deletion() {
        let change = TextChange::deletion(5, 11);
        let result = change.apply("hello world");
        assert_eq!(result, "hello");
    }
    #[test]
    fn test_text_change_apply_replacement() {
        let change = TextChange::replacement(6, 11, "Rust");
        let result = change.apply("hello world");
        assert_eq!(result, "hello Rust");
    }
    #[test]
    fn test_text_change_delta() {
        let ins = TextChange::insertion(0, "abc");
        assert_eq!(ins.delta(), 3);
        let del = TextChange::deletion(0, 5);
        assert_eq!(del.delta(), -5);
        let rep = TextChange::replacement(0, 5, "hi");
        assert_eq!(rep.delta(), -3);
    }
    #[test]
    fn test_text_change_is_insertion() {
        let ins = TextChange::insertion(0, "x");
        assert!(ins.is_insertion());
        assert!(!ins.is_deletion());
        assert!(!ins.is_replacement());
    }
    #[test]
    fn test_text_change_is_deletion() {
        let del = TextChange::deletion(0, 3);
        assert!(del.is_deletion());
    }
    #[test]
    fn test_text_change_is_replacement() {
        let rep = TextChange::replacement(0, 5, "hi");
        assert!(rep.is_replacement());
    }
    #[test]
    fn test_incremental_parser_new() {
        let src = "def foo : Nat := 0\ntheorem bar : True := trivial\n";
        let parser = IncrementalParser::new(src);
        assert_eq!(parser.version(), 0);
        assert!(parser.cache_size() > 0);
        assert_eq!(parser.dirty_count(), 0);
    }
    #[test]
    fn test_apply_change_marks_dirty() {
        let src = "def foo : Nat := 0\n";
        let mut parser = IncrementalParser::new(src);
        let initial_version = parser.version();
        parser.apply_change(TextChange::replacement(4, 7, "bar"));
        assert_eq!(parser.version(), initial_version + 1);
        assert!(parser.source().contains("bar"));
    }
    #[test]
    fn test_split_declarations() {
        let src = "def foo : Nat := 0\ntheorem bar : True := trivial\naxiom baz : False\n";
        let decls = IncrementalParser::split_declarations(src);
        assert_eq!(decls.len(), 3);
        assert!(decls[0].1.starts_with("def "));
        assert!(decls[1].1.starts_with("theorem "));
        assert!(decls[2].1.starts_with("axiom "));
    }
    #[test]
    fn test_versioned_source_new() {
        let vs = VersionedSource::new("file:///foo.oxilean", "def x := 1");
        assert_eq!(vs.uri, "file:///foo.oxilean");
        assert_eq!(vs.version, 0);
        assert_eq!(vs.content, "def x := 1");
        assert!(!vs.is_empty());
        assert_eq!(vs.len(), 10);
    }
    #[test]
    fn test_offset_to_position() {
        let vs = VersionedSource::new("u", "hello\nworld\n");
        assert_eq!(vs.offset_to_position(0), (0, 0));
        assert_eq!(vs.offset_to_position(5), (0, 5));
        assert_eq!(vs.offset_to_position(6), (1, 0));
        assert_eq!(vs.offset_to_position(11), (1, 5));
    }
    #[test]
    fn test_position_to_offset() {
        let vs = VersionedSource::new("u", "hello\nworld\n");
        assert_eq!(vs.position_to_offset(0, 0), Some(0));
        assert_eq!(vs.position_to_offset(0, 5), Some(5));
        assert_eq!(vs.position_to_offset(1, 0), Some(6));
        assert_eq!(vs.position_to_offset(1, 5), Some(11));
        assert_eq!(vs.position_to_offset(99, 0), None);
    }
    #[test]
    fn test_dependency_graph_add_and_dependents() {
        let mut g = DependencyGraph::new();
        g.add_edge("bar", "foo");
        g.add_edge("baz", "bar");
        let deps = g.dependents_of("foo");
        assert!(deps.contains(&"bar".to_string()));
        assert!(deps.contains(&"baz".to_string()));
    }
    #[test]
    fn test_dependency_graph_direct_dependencies() {
        let mut g = DependencyGraph::new();
        g.add_edge("bar", "foo");
        g.add_edge("bar", "qux");
        let direct = g.direct_dependencies("bar");
        assert!(direct.contains(&"foo".to_string()));
        assert!(direct.contains(&"qux".to_string()));
    }
    #[test]
    fn test_dependency_graph_remove_node() {
        let mut g = DependencyGraph::new();
        g.add_edge("bar", "foo");
        g.remove_node("foo");
        assert!(g.direct_dependencies("bar").is_empty());
    }
    #[test]
    fn test_edit_distance_same() {
        assert_eq!(edit_distance("hello", "hello"), 0);
    }
    #[test]
    fn test_edit_distance_empty() {
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("abc", ""), 3);
    }
    #[test]
    fn test_edit_distance_diff() {
        assert_eq!(edit_distance("kitten", "sitting"), 3);
    }
    #[test]
    fn test_diff_lines_insertion() {
        let old = "a\n";
        let new = "a\nb\n";
        let changes = diff_lines(old, new);
        assert!(!changes.is_empty());
    }
    #[test]
    fn test_diff_lines_no_change() {
        let src = "def foo := 0\n";
        let changes = diff_lines(src, src);
        assert!(changes.is_empty());
    }
    #[test]
    fn test_token_fingerprint_same() {
        let fp1 = TokenFingerprint::compute(&["def", "foo", ":=", "0"]);
        let fp2 = TokenFingerprint::compute(&["def", "foo", ":=", "0"]);
        assert_eq!(fp1, fp2);
    }
    #[test]
    fn test_token_fingerprint_diff() {
        let fp1 = TokenFingerprint::compute(&["def", "foo"]);
        let fp2 = TokenFingerprint::compute(&["def", "bar"]);
        assert_ne!(fp1, fp2);
    }
    #[test]
    fn test_green_node_leaf() {
        let node = GreenNode::leaf(SyntaxKind::Ident, "foo");
        assert!(node.is_leaf());
        assert_eq!(node.width, 3);
        assert_eq!(node.to_text(), "foo");
    }
    #[test]
    fn test_green_node_interior() {
        let a = GreenNode::leaf(SyntaxKind::Token("def".to_string()), "def");
        let b = GreenNode::leaf(SyntaxKind::Ident, "foo");
        let node = GreenNode::interior(SyntaxKind::Def, vec![a, b]);
        assert!(!node.is_leaf());
        assert_eq!(node.width, 6);
        assert_eq!(node.to_text(), "deffoo");
    }
    #[test]
    fn test_red_node_range() {
        let green = GreenNode::leaf(SyntaxKind::Ident, "hello");
        let red = RedNode::new(&green, 10);
        assert_eq!(red.range(), 10..15);
    }
    #[test]
    fn test_red_node_children() {
        let a = GreenNode::leaf(SyntaxKind::Ident, "ab");
        let b = GreenNode::leaf(SyntaxKind::Ident, "cd");
        let parent = GreenNode::interior(SyntaxKind::Root, vec![a, b]);
        let red = RedNode::new(&parent, 0);
        let children = red.children();
        assert_eq!(children.len(), 2);
        assert_eq!(children[0].range(), 0..2);
        assert_eq!(children[1].range(), 2..4);
    }
    #[test]
    fn test_persistent_vec_push() {
        let v0: PersistentVec<i32> = PersistentVec::new();
        let v1 = v0.push(1);
        let v2 = v1.push(2);
        assert_eq!(v0.len(), 0);
        assert_eq!(v1.len(), 1);
        assert_eq!(v2.len(), 2);
        assert_eq!(v2.get(0), Some(&1));
        assert_eq!(v2.get(1), Some(&2));
    }
    #[test]
    fn test_persistent_vec_set() {
        let v: PersistentVec<i32> = PersistentVec::new().push(10).push(20);
        let v2 = v.set(0, 99).expect("test operation should succeed");
        assert_eq!(v.get(0), Some(&10));
        assert_eq!(v2.get(0), Some(&99));
    }
    #[test]
    fn test_transaction_commit() {
        let src = "hello world";
        let mut tx = Transaction::begin(src);
        tx.add(TextChange::replacement(6, 11, "Rust"));
        let result = tx.commit(src);
        assert_eq!(result, "hello Rust");
    }
    #[test]
    fn test_transaction_rollback() {
        let src = "hello";
        let mut tx = Transaction::begin(src);
        tx.add(TextChange::insertion(5, " world"));
        let original = tx.rollback().expect("test operation should succeed");
        assert_eq!(original, "hello");
    }
    #[test]
    fn test_undo_redo_stack() {
        let mut stack = UndoRedoStack::new("v0");
        stack.push("v1");
        stack.push("v2");
        assert_eq!(stack.current(), "v2");
        assert!(stack.can_undo());
        stack.undo();
        assert_eq!(stack.current(), "v1");
        assert!(stack.can_redo());
        stack.redo();
        assert_eq!(stack.current(), "v2");
    }
    #[test]
    fn test_undo_redo_clears_redo_on_push() {
        let mut stack = UndoRedoStack::new("v0");
        stack.push("v1");
        stack.undo();
        assert!(stack.can_redo());
        stack.push("v2");
        assert!(!stack.can_redo());
    }
    #[test]
    fn test_incremental_lexer_basic() {
        let mut lexer = IncrementalLexer::new();
        let tokens = lexer.lex("def foo := 0\ndef bar := 1", &[0, 1]);
        assert!(tokens.contains(&"def".to_string()));
        assert!(tokens.contains(&"foo".to_string()));
    }
    #[test]
    fn test_incremental_lexer_caches() {
        let mut lexer = IncrementalLexer::new();
        let t1 = lexer.lex("def foo := 0\ndef bar := 1", &[0, 1]);
        let t2 = lexer.lex("def foo := 0\ndef bar := 1", &[]);
        assert_eq!(t1, t2);
    }
    #[test]
    fn test_invalidate_by_name() {
        let src = "def foo : Nat := 0\ndef bar : Nat := 1\n";
        let mut parser = IncrementalParser::new(src);
        parser.invalidate_by_name("foo");
        let invalid: Vec<_> = parser.invalid_declarations();
        assert!(invalid.iter().any(|d| d.name.as_deref() == Some("foo")));
    }
}
/// Applies a sequence of edits to a source string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn apply_edits(source: &str, edits: &[SourceEdit]) -> String {
    let mut result = source.to_string();
    let mut sorted = edits.to_vec();
    sorted.sort_by_key(|b| std::cmp::Reverse(b.start));
    for edit in sorted {
        let end = edit.end.min(result.len());
        let start = edit.start.min(end);
        result.replace_range(start..end, &edit.new_text);
    }
    result
}
/// Computes the invalidated range from an edit.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn compute_invalidated_range(edit: &SourceEdit, context_bytes: usize) -> InvalidatedRange {
    let start = edit.start.saturating_sub(context_bytes);
    let end = edit.end + edit.new_text.len() + context_bytes;
    InvalidatedRange::new(start, end)
}
/// Computes dirty region from a source edit and source content.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn compute_dirty_region(source: &str, edit: &SourceEdit) -> DirtyRegion {
    let start_line = source[..edit.start.min(source.len())]
        .lines()
        .count()
        .saturating_sub(1);
    let end_byte = (edit.end + edit.new_text.len()).min(source.len());
    let end_line = source[..end_byte].lines().count().saturating_sub(1);
    DirtyRegion::new(start_line, end_line, edit.start, end_byte)
}
#[cfg(test)]
mod extended_incremental_tests {
    use super::*;
    use crate::incremental::*;
    #[test]
    fn test_source_edit_kinds() {
        let ins = SourceEdit::insert(5, "hello");
        assert!(ins.is_insert());
        let del = SourceEdit::delete(2, 8);
        assert!(del.is_delete());
        let rep = SourceEdit::replace(0, 3, "xyz");
        assert!(rep.is_replace());
    }
    #[test]
    fn test_source_edit_delta() {
        let rep = SourceEdit::replace(0, 5, "abc");
        assert_eq!(rep.delta(), -2);
        let ins = SourceEdit::insert(3, "hello");
        assert_eq!(ins.delta(), 5);
    }
    #[test]
    fn test_apply_edits() {
        let source = "hello world";
        let edit = SourceEdit::replace(6, 11, "Rust");
        let result = apply_edits(source, &[edit]);
        assert_eq!(result, "hello Rust");
    }
    #[test]
    fn test_apply_edits_insert() {
        let source = "helo";
        let edit = SourceEdit::insert(3, "l");
        let result = apply_edits(source, &[edit]);
        assert_eq!(result, "hello");
    }
    #[test]
    fn test_edit_history_undo_redo() {
        let mut hist = EditHistory::new(10);
        let e = SourceEdit::insert(0, "x");
        hist.push(e.clone());
        assert_eq!(hist.history_len(), 1);
        let undone = hist.undo();
        assert!(undone.is_some());
        assert_eq!(hist.history_len(), 0);
        assert_eq!(hist.undo_count(), 1);
        let redone = hist.redo();
        assert!(redone.is_some());
        assert_eq!(hist.history_len(), 1);
    }
    #[test]
    fn test_invalidated_range() {
        let r = InvalidatedRange::new(10, 20);
        assert!(r.contains(15));
        assert!(!r.contains(5));
        assert_eq!(r.len(), 10);
        let r2 = InvalidatedRange::new(18, 30);
        assert!(r.overlaps(&r2));
        let merged = r.merge(&r2);
        assert_eq!(merged.start, 10);
        assert_eq!(merged.end, 30);
    }
    #[test]
    fn test_token_validity() {
        let mut tv = TokenValidity::new();
        tv.mark_valid(0, 10);
        tv.mark_valid(20, 30);
        assert!(tv.is_valid_at(5));
        assert!(!tv.is_valid_at(15));
        tv.invalidate(&InvalidatedRange::new(0, 10));
        assert!(!tv.is_valid_at(5));
        assert_eq!(tv.valid_count(), 1);
    }
    #[test]
    fn test_parse_version() {
        let mut v = ParseVersion::new();
        v.increment();
        v.increment();
        v.mark_full_parse();
        v.increment();
        assert_eq!(v.edits_since_full_parse(), 1);
        assert!(!v.needs_full_reparse(5));
    }
    #[test]
    fn test_node_range_cache() {
        let mut cache = NodeRangeCache::new();
        cache.insert(0, 10, 1);
        cache.insert(10, 20, 2);
        assert_eq!(cache.lookup(0, 10), Some(1));
        cache.invalidate_range(&InvalidatedRange::new(5, 15));
        assert_eq!(cache.lookup(0, 10), None);
        assert_eq!(cache.lookup(10, 20), None);
        assert_eq!(cache.size(), 0);
    }
    #[test]
    fn test_incremental_lexer() {
        let mut lex = IncrementalLexerExt::new("hello world");
        assert_eq!(lex.source(), "hello world");
        lex.apply_edit(SourceEdit::replace(6, 11, "Rust"));
        assert_eq!(lex.source(), "hello Rust");
        assert_eq!(lex.version(), 1);
    }
    #[test]
    fn test_simple_rope() {
        let mut rope = SimpleRope::new("hello");
        rope.insert(5, " world");
        assert_eq!(rope.to_string(), "hello world");
        rope.delete(5, 11);
        assert_eq!(rope.to_string(), "hello");
        assert_eq!(rope.len(), 5);
    }
    #[test]
    fn test_decl_dependency_tracker() {
        let mut tracker = DeclDependencyTracker::new();
        tracker.register_decl("foo", 0, 50);
        tracker.register_decl("bar", 50, 100);
        let edit = SourceEdit::replace(40, 60, "x");
        let affected = tracker.affected_by_edit(&edit);
        assert!(affected.contains(&"foo"));
        assert!(affected.contains(&"bar"));
        assert_eq!(tracker.decl_count(), 2);
    }
    #[test]
    fn test_incr_parse_stats() {
        let mut stats = IncrParseStats::new();
        stats.total_edits = 10;
        stats.tokens_reused = 80;
        stats.tokens_relexed = 20;
        stats.nodes_reused = 70;
        stats.nodes_rebuilt = 30;
        assert!((stats.reuse_fraction_tokens() - 0.8).abs() < 1e-9);
        assert!((stats.reuse_fraction_nodes() - 0.7).abs() < 1e-9);
        let s = stats.summary();
        assert!(s.contains("token_reuse=80.0%"));
    }
    #[test]
    fn test_incremental_parse_cache() {
        let mut cache = IncrementalParseCache::new(10);
        let entry = IncrParseEntry {
            region_hash: 42,
            result_repr: "expr".into(),
            version: 1,
        };
        cache.store(entry);
        assert!(cache.lookup(42).is_some());
        assert!(cache.lookup(99).is_none());
        assert_eq!(cache.stats(), (1, 1));
    }
    #[test]
    fn test_compute_dirty_region() {
        let source = "def foo := 1\ndef bar := 2\n";
        let edit = SourceEdit::replace(4, 7, "baz");
        let dirty = compute_dirty_region(source, &edit);
        assert!(dirty.byte_count() > 0);
        assert!(dirty.is_single_line());
    }
}
/// Computes a line diff between two versions of a source.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn line_diff_source(old: &str, new: &str) -> LineDiff {
    let old_lines: Vec<_> = old.lines().collect();
    let new_lines: Vec<_> = new.lines().collect();
    let mut diff = LineDiff::new();
    let max = old_lines.len().max(new_lines.len());
    for i in 0..max {
        match (old_lines.get(i), new_lines.get(i)) {
            (Some(o), Some(n)) if o != n => diff.add_change(i, *o, *n),
            (Some(o), None) => diff.add_change(i, *o, ""),
            (None, Some(n)) => diff.add_change(i, "", *n),
            _ => {}
        }
    }
    diff
}
/// Merge overlapping reparse requests.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn merge_reparse_requests(requests: &mut Vec<ReparseRequest>) {
    requests.sort_by_key(|r| r.start_byte);
    let mut merged: Vec<ReparseRequest> = Vec::new();
    for req in requests.drain(..) {
        if let Some(last) = merged.last_mut() {
            if req.start_byte <= last.end_byte {
                last.end_byte = last.end_byte.max(req.end_byte);
                if req.priority > last.priority {
                    last.priority = req.priority;
                }
                continue;
            }
        }
        merged.push(req);
    }
    *requests = merged;
}
#[cfg(test)]
mod extended_incremental_tests_2 {
    use super::*;
    use crate::incremental::*;
    #[test]
    fn test_line_diff_source() {
        let old = "a\nb\nc";
        let new = "a\nX\nc";
        let diff = line_diff_source(old, new);
        assert_eq!(diff.count(), 1);
        assert_eq!(diff.affected_lines(), vec![1]);
    }
    #[test]
    fn test_reparse_request() {
        let req = ReparseRequest::new(0, 100, 1).with_priority(ReparsePriority::High);
        assert_eq!(req.byte_span(), 100);
        assert_eq!(req.priority, ReparsePriority::High);
    }
    #[test]
    fn test_reparse_queue() {
        let mut q = ReparseQueue::new();
        q.push(ReparseRequest::new(0, 50, 1).with_priority(ReparsePriority::Low));
        q.push(ReparseRequest::new(0, 50, 1).with_priority(ReparsePriority::Urgent));
        assert!(q.has_urgent());
        let top = q.pop().expect("collection should not be empty");
        assert_eq!(top.priority, ReparsePriority::Urgent);
    }
    #[test]
    fn test_merge_reparse_requests() {
        let mut reqs = vec![
            ReparseRequest::new(0, 30, 1),
            ReparseRequest::new(20, 60, 1),
        ];
        merge_reparse_requests(&mut reqs);
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].end_byte, 60);
    }
    #[test]
    fn test_offset_to_token_map() {
        let mut map = OffsetToTokenMap::new();
        map.insert(0, 1);
        map.insert(10, 2);
        map.insert(20, 3);
        assert_eq!(map.token_at(5), Some(1));
        assert_eq!(map.token_at(15), Some(2));
        map.invalidate_from(10);
        assert_eq!(map.count(), 1);
    }
    #[test]
    fn test_offset_map_shift() {
        let mut map = OffsetToTokenMap::new();
        map.insert(10, 1);
        map.insert(20, 2);
        map.shift(10, 5);
        assert_eq!(map.token_at(15), Some(1));
        assert_eq!(map.token_at(25), Some(2));
    }
    #[test]
    fn test_incremental_parse_result() {
        let mut r = IncrementalParseResult::new(true, 80, 20, 500);
        assert!((r.reuse_ratio() - 0.8).abs() < 1e-9);
        assert!(!r.has_errors());
        r.add_error("parse error");
        assert!(r.has_errors());
    }
    #[test]
    fn test_change_detector() {
        let source = "hello world";
        let mut det = ChangeDetector::new();
        det.record(source, 0, 5);
        assert!(!det.has_changed(source, 0, 5));
        assert!(det.has_changed("HELLO world", 0, 5));
    }
    #[test]
    fn test_incremental_checksum() {
        let source = "abcdef";
        let cs = IncrementalChecksum::build(source);
        let total = cs.total();
        assert!(total > 0);
        let r1 = cs.range_sum(0, 3);
        let r2 = cs.range_sum(3, 6);
        assert_eq!(r1 + r2, total);
    }
    #[test]
    fn test_atomic_version() {
        let av = AtomicVersion::new();
        let v1 = av.increment();
        let v2 = av.increment();
        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
        av.reset();
        assert_eq!(av.load(), 0);
    }
    #[test]
    fn test_snapshot_manager() {
        let mut mgr = SnapshotManager::new(3);
        mgr.save(ParseSnapshot::capture("src1", 1, 10, 3));
        mgr.save(ParseSnapshot::capture("src2", 2, 10, 1));
        mgr.save(ParseSnapshot::capture("src3", 3, 10, 5));
        assert_eq!(mgr.count(), 3);
        let best = mgr.best().expect("test operation should succeed");
        assert_eq!(best.error_count, 1);
        let latest = mgr.latest().expect("test operation should succeed");
        assert_eq!(latest.version, 3);
    }
    #[test]
    fn test_parse_snapshot_cleaner_than() {
        let a = ParseSnapshot::capture("a", 1, 5, 2);
        let b = ParseSnapshot::capture("b", 2, 5, 5);
        assert!(a.is_cleaner_than(&b));
        assert!(!b.is_cleaner_than(&a));
    }
    #[test]
    fn test_line_diff_default() {
        let diff = LineDiff::default();
        assert_eq!(diff.count(), 0);
    }
}
#[cfg(test)]
mod extended_incremental_tests_3 {
    use super::*;
    use crate::incremental::*;
    #[test]
    fn test_incr_scope_stack() {
        let mut stack = IncrScopeStack::new();
        stack.push(IncrScopeEntry::new(0, ScopeKind2::Paren, 1));
        stack.push(IncrScopeEntry::new(5, ScopeKind2::Bracket, 2));
        assert_eq!(stack.depth(), 2);
        assert_eq!(stack.current_scope(), Some(ScopeKind2::Bracket));
        let popped = stack.pop().expect("collection should not be empty");
        assert_eq!(popped.kind, ScopeKind2::Bracket);
    }
    #[test]
    fn test_incremental_error_map() {
        let mut map = IncrementalErrorMap::new();
        map.add_error(10, "unexpected token");
        map.add_error(20, "expected ')'");
        map.add_error(15, "ambiguous");
        assert_eq!(map.total_error_count(), 3);
        let errs = map.errors_in_range(10, 20);
        assert_eq!(errs.len(), 2);
        map.clear_range(10, 20);
        assert_eq!(map.total_error_count(), 1);
    }
    #[test]
    fn test_edit_buffer() {
        let mut buf = EditBuffer::new(3);
        assert!(buf.add(SourceEdit::insert(0, "x")));
        assert!(buf.add(SourceEdit::delete(5, 10)));
        assert_eq!(buf.pending_count(), 2);
        let flushed = buf.flush();
        assert_eq!(flushed.len(), 2);
        assert!(buf.is_empty());
    }
    #[test]
    fn test_edit_buffer_overflow() {
        let mut buf = EditBuffer::new(2);
        buf.add(SourceEdit::insert(0, "a"));
        buf.add(SourceEdit::insert(1, "b"));
        assert!(!buf.add(SourceEdit::insert(2, "c")));
    }
    #[test]
    fn test_token_reachability() {
        let mut r = TokenReachability::new();
        r.mark_reachable(10);
        r.mark_reachable(20);
        r.mark_reachable(30);
        assert!(r.is_reachable(10));
        assert!(!r.is_reachable(15));
        assert_eq!(r.reachable_count(), 3);
        assert!((r.coverage_fraction(10) - 0.3).abs() < 1e-9);
    }
    #[test]
    fn test_fiber_pool() {
        let mut pool = FiberPool::new();
        let id1 = pool.spawn(0, 0, "start");
        let id2 = pool.spawn(10, 1, "mid");
        assert_eq!(pool.active_count(), 2);
        let f = pool.get(id1).expect("key should exist");
        assert!(f.is_at_root());
        pool.remove(id2);
        assert_eq!(pool.active_count(), 1);
    }
    #[test]
    fn test_incremental_session() {
        let mut sess = IncrementalSession::new("hello world");
        assert_eq!(sess.current_version(), 0);
        sess.apply_edit(SourceEdit::replace(6, 11, "Rust"));
        assert_eq!(sess.source_text(), "hello Rust");
        assert_eq!(sess.current_version(), 1);
        assert!(!sess.has_errors());
    }
    #[test]
    fn test_parse_fiber() {
        let f = ParseFiber::new(1, 0, 0, "state");
        assert!(f.is_at_root());
        let f2 = ParseFiber::new(2, 5, 3, "deeper");
        assert!(!f2.is_at_root());
    }
}
