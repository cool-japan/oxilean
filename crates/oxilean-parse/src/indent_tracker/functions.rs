//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    IndentBlock, IndentChangeLog, IndentDiff, IndentGuide, IndentLevel, IndentMode, IndentRegion,
    LayoutContext, TokenSequenceClass,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent_tracker::*;
    #[test]
    fn test_indent_level_total_width_spaces_only() {
        let level = IndentLevel::new(4, 0);
        assert_eq!(level.total_width(4), 4);
    }
    #[test]
    fn test_indent_level_total_width_tabs() {
        let level = IndentLevel::new(0, 2);
        assert_eq!(level.total_width(4), 8);
    }
    #[test]
    fn test_indent_level_is_deeper_than() {
        let shallow = IndentLevel::new(2, 0);
        let deep = IndentLevel::new(6, 0);
        assert!(deep.is_deeper_than(&shallow, 4));
        assert!(!shallow.is_deeper_than(&deep, 4));
        assert!(!shallow.is_deeper_than(&shallow, 4));
    }
    #[test]
    fn test_indent_stack_push_pop() {
        let mut stack = IndentStack::new(4);
        stack.push(IndentLevel::new(0, 0));
        stack.push(IndentLevel::new(4, 0));
        assert_eq!(
            stack
                .current()
                .expect("test operation should succeed")
                .spaces,
            4
        );
        let popped = stack.pop().expect("collection should not be empty");
        assert_eq!(popped.spaces, 4);
        assert_eq!(
            stack
                .current()
                .expect("test operation should succeed")
                .spaces,
            0
        );
    }
    #[test]
    fn test_indent_stack_dedent_to() {
        let mut stack = IndentStack::new(4);
        stack.push(IndentLevel::new(0, 0));
        stack.push(IndentLevel::new(4, 0));
        stack.push(IndentLevel::new(8, 0));
        let target = IndentLevel::new(4, 0);
        let popped = stack.dedent_to(&target);
        assert_eq!(popped, 1);
        assert_eq!(
            stack
                .current()
                .expect("test operation should succeed")
                .spaces,
            4
        );
    }
    #[test]
    fn test_parse_leading_whitespace_spaces() {
        let level = WhereBlockTracker::parse_leading_whitespace("    hello");
        assert_eq!(level.spaces, 4);
        assert_eq!(level.tabs, 0);
    }
    #[test]
    fn test_parse_leading_whitespace_tabs() {
        let level = WhereBlockTracker::parse_leading_whitespace("\t\thello");
        assert_eq!(level.spaces, 0);
        assert_eq!(level.tabs, 2);
    }
    #[test]
    fn test_where_block_tracker_enter_exit() {
        let mut tracker = WhereBlockTracker::new();
        assert!(!tracker.in_where_block);
        tracker.enter_where(IndentLevel::new(0, 0));
        assert!(tracker.in_where_block);
        tracker.add_where_item("foo", IndentLevel::new(2, 0));
        assert_eq!(tracker.where_items.len(), 1);
        tracker.exit_where();
        assert!(!tracker.in_where_block);
        assert!(tracker.where_items.is_empty());
    }
    #[test]
    fn test_where_block_tracker_is_where_item() {
        let mut tracker = WhereBlockTracker::new();
        tracker.enter_where(IndentLevel::new(0, 0));
        assert!(tracker.is_where_item("  foo : Nat", IndentLevel::new(2, 0)));
        assert!(!tracker.is_where_item("bar : Nat", IndentLevel::new(0, 0)));
    }
}
/// Classify a block of text lines as a token sequence.
#[allow(dead_code)]
pub fn classify_sequence(lines: &[&str]) -> TokenSequenceClass {
    let has_where = lines
        .iter()
        .any(|l| l.trim() == "where" || l.trim().starts_with("where "));
    let has_let = lines.iter().any(|l| l.trim().starts_with("let "));
    let has_do = lines
        .iter()
        .any(|l| l.trim().starts_with("do ") || l.trim() == "do");
    let has_def = lines.iter().any(|l| {
        let t = l.trim();
        t.starts_with("def ") || t.starts_with("theorem ") || t.starts_with("definition ")
    });
    if has_def && has_where {
        TokenSequenceClass::DefWithWhere
    } else if has_def {
        TokenSequenceClass::PlainDef
    } else if has_let {
        TokenSequenceClass::LetBlock
    } else if has_do {
        TokenSequenceClass::DoBlock
    } else {
        TokenSequenceClass::Unknown
    }
}
/// Split a source string into `IndentRegion`s based on consistent indentation.
#[allow(dead_code)]
pub fn split_into_regions(src: &str, tab_width: usize) -> Vec<IndentRegion> {
    let mut regions = Vec::new();
    let mut current_indent: Option<usize> = None;
    let mut region_start = 1usize;
    for (i, line) in src.lines().enumerate() {
        let lineno = i + 1;
        if line.trim().is_empty() {
            continue;
        }
        let indent = LayoutContext::indent_col(line, tab_width);
        match current_indent {
            None => {
                current_indent = Some(indent);
                region_start = lineno;
            }
            Some(cur) if cur == indent => {}
            Some(cur) => {
                regions.push(IndentRegion::new(region_start, lineno - 1, cur));
                current_indent = Some(indent);
                region_start = lineno;
            }
        }
    }
    if let Some(cur) = current_indent {
        let line_count = src.lines().count();
        regions.push(IndentRegion::new(region_start, line_count, cur));
    }
    regions
}
#[cfg(test)]
mod additional_tests_2 {
    use super::*;
    use crate::indent_tracker::*;
    #[test]
    fn test_classify_sequence_def_with_where() {
        let lines = vec!["def foo := body", "  where", "    helper := 1"];
        assert_eq!(classify_sequence(&lines), TokenSequenceClass::DefWithWhere);
    }
    #[test]
    fn test_classify_sequence_plain_def() {
        let lines = vec!["def foo := 1"];
        assert_eq!(classify_sequence(&lines), TokenSequenceClass::PlainDef);
    }
    #[test]
    fn test_classify_sequence_let_block() {
        let lines = vec!["let x := 1", "let y := 2"];
        assert_eq!(classify_sequence(&lines), TokenSequenceClass::LetBlock);
    }
    #[test]
    fn test_classify_sequence_do_block() {
        let lines = vec!["do", "  x <- foo"];
        assert_eq!(classify_sequence(&lines), TokenSequenceClass::DoBlock);
    }
    #[test]
    fn test_classify_sequence_unknown() {
        let lines = vec!["x + y", "z * w"];
        assert_eq!(classify_sequence(&lines), TokenSequenceClass::Unknown);
    }
    #[test]
    fn test_indent_level_history_undo() {
        let mut hist = IndentLevelHistory::new();
        hist.set_levels(vec![IndentLevel::new(0, 0)]);
        hist.snapshot();
        hist.set_levels(vec![IndentLevel::new(4, 0)]);
        assert!(hist.undo());
        assert_eq!(hist.current()[0].spaces, 0);
        assert_eq!(hist.undo_depth(), 0);
        assert!(!hist.undo());
    }
    #[test]
    fn test_indent_level_history_redo() {
        let mut hist = IndentLevelHistory::new();
        hist.set_levels(vec![IndentLevel::new(0, 0)]);
        hist.snapshot();
        hist.set_levels(vec![IndentLevel::new(4, 0)]);
        hist.undo();
        assert!(hist.redo());
        assert_eq!(hist.current()[0].spaces, 4);
    }
    #[test]
    fn test_indent_level_history_snapshot_clears_future() {
        let mut hist = IndentLevelHistory::new();
        hist.set_levels(vec![IndentLevel::new(0, 0)]);
        hist.snapshot();
        hist.set_levels(vec![IndentLevel::new(4, 0)]);
        hist.undo();
        hist.snapshot();
        hist.set_levels(vec![IndentLevel::new(8, 0)]);
        assert_eq!(hist.redo_depth(), 0);
    }
    #[test]
    fn test_indent_region_line_count() {
        let region = IndentRegion::new(3, 7, 4);
        assert_eq!(region.line_count(), 5);
    }
    #[test]
    fn test_indent_region_contains_line() {
        let region = IndentRegion::new(2, 5, 0);
        assert!(region.contains_line(2));
        assert!(region.contains_line(5));
        assert!(!region.contains_line(6));
    }
    #[test]
    fn test_split_into_regions_empty() {
        let regions = split_into_regions("", 4);
        assert!(regions.is_empty());
    }
    #[test]
    fn test_split_into_regions_uniform() {
        let src = "def foo := 1\ndef bar := 2\n";
        let regions = split_into_regions(src, 4);
        assert!(!regions.is_empty());
    }
    #[test]
    fn test_indent_level_history_empty_undo() {
        let mut hist = IndentLevelHistory::new();
        assert!(!hist.undo());
    }
    #[test]
    fn test_indent_region_single_line() {
        let region = IndentRegion::new(5, 5, 0);
        assert_eq!(region.line_count(), 1);
    }
    #[test]
    fn test_token_sequence_class_eq() {
        assert_eq!(TokenSequenceClass::PlainDef, TokenSequenceClass::PlainDef);
        assert_ne!(TokenSequenceClass::PlainDef, TokenSequenceClass::DoBlock);
    }
    #[test]
    fn test_layout_context_default_tab_width() {
        let ctx = LayoutContext::default();
        assert_eq!(ctx.tab_width, 4);
    }
    #[test]
    fn test_layout_context_push_pop() {
        let mut ctx = LayoutContext::new(4);
        ctx.push_rule(LayoutRule::SameBlock(4));
        assert_eq!(ctx.depth(), 1);
        let popped = ctx.pop_rule();
        assert!(matches!(popped, Some(LayoutRule::SameBlock(4))));
        assert_eq!(ctx.depth(), 0);
    }
    #[test]
    fn test_layout_rule_continuation() {
        let rule = LayoutRule::Continuation(4);
        assert!(rule.matches(5));
        assert!(!rule.matches(4));
        assert!(!rule.matches(3));
    }
    #[test]
    fn test_layout_rule_close_block() {
        let rule = LayoutRule::CloseBlock(4);
        assert!(rule.matches(2));
        assert!(!rule.matches(4));
        assert!(!rule.matches(6));
    }
    #[test]
    fn test_indent_mismatch_error_display() {
        let err = IndentMismatchError::new(10, 4, 3, "test");
        let s = format!("{}", err);
        assert!(s.contains("line 10"));
    }
    #[test]
    fn test_indentation_checker_mismatch() {
        let mut checker = IndentationChecker::new(4);
        let base = IndentLevel::new(0, 0);
        let odd = IndentLevel::new(3, 0);
        checker.check_transition(5, &base, &odd, "test");
        assert!(!checker.is_clean());
    }
    #[test]
    fn test_indentation_checker_clean() {
        let mut checker = IndentationChecker::new(4);
        let base = IndentLevel::new(0, 0);
        let deeper = IndentLevel::new(4, 0);
        checker.check_transition(1, &base, &deeper, "test");
        assert!(checker.is_clean());
    }
    #[test]
    fn test_indent_diff_same() {
        let a = IndentLevel::new(4, 0);
        let b = IndentLevel::new(4, 0);
        assert_eq!(compare_indent(&a, &b, 4), IndentDiff::Same);
    }
    #[test]
    fn test_tab_stop_from_col_on_boundary() {
        let stops: Vec<usize> = TabStopIterator::from_col(4, 8).take(2).collect();
        assert_eq!(stops, vec![8, 12]);
    }
    #[test]
    fn test_scope_is_bound() {
        let mut scope = Scope::new("where", IndentLevel::new(0, 0));
        scope.add_binding("alpha");
        assert!(scope.is_bound("alpha"));
        assert!(!scope.is_bound("beta"));
    }
    #[test]
    fn test_let_binding_has_type() {
        let b = LetBinding::new("x", 0, true);
        assert!(b.has_type);
        assert_eq!(b.col, 0);
    }
    #[test]
    fn test_indent_history_max_min() {
        let hist = IndentHistory {
            entries: vec![(1, 0), (2, 4), (3, 8)],
            tab_width: 4,
        };
        assert_eq!(hist.max_indent(), 8);
        assert_eq!(hist.min_nonzero_indent(), Some(4));
    }
    #[test]
    fn test_do_block_tracker_nested() {
        let mut t = DoBlockTracker::new(4);
        t.enter(4);
        t.enter(8);
        assert_eq!(t.depth(), 2);
        t.exit();
        assert_eq!(t.current_col(), Some(4));
    }
    #[test]
    fn test_hanging_indent_zero_overhang() {
        let hi = HangingIndent::new(4, 4);
        assert_eq!(hi.overhang(), 0);
    }
    #[test]
    fn test_multiline_string_has_unescaped_quote() {
        assert!(MultilineStringTracker::has_unescaped_quote(
            r#"hello "world""#
        ));
        assert!(!MultilineStringTracker::has_unescaped_quote("no quotes"));
    }
    #[test]
    fn test_comment_tracker_nested_block() {
        let mut ct = CommentTracker::new();
        ct.process_pair('/', '-');
        ct.process_pair('/', '-');
        assert_eq!(ct.block_depth, 2);
        ct.process_pair('-', '/');
        assert_eq!(ct.block_depth, 1);
        ct.process_pair('-', '/');
        assert!(!ct.in_comment());
    }
    #[test]
    fn test_indent_validator_tabs() {
        let mut v = IndentValidator::expect_tabs();
        v.validate("\thello\n");
        assert!(v.is_valid());
        v.validate("    bad\n");
        assert!(!v.is_valid());
    }
    #[test]
    fn test_column_aligner_pad() {
        let aligner = ColumnAligner::new(10);
        let padded = aligner.pad("foo", 3);
        assert_eq!(padded.len(), 10);
    }
    #[test]
    fn test_whitespace_kind_none() {
        let kind = WhitespaceKind::classify("ab", 1, 1);
        assert_eq!(kind, WhitespaceKind::None);
    }
    #[test]
    fn test_open_brace_tracker_mismatch() {
        let mut t = OpenBraceTracker::new();
        t.push('(', 1, 0);
        assert!(t.pop(']', 1, 5).is_err());
    }
    #[test]
    fn test_indent_rewriter_no_op() {
        let rw = IndentRewriter::new(4, 4);
        let input = "    hello";
        assert_eq!(rw.rewrite_line(input), input);
    }
    #[test]
    fn test_source_splitter_single_decl() {
        let src = "def foo := 1\n  body\n";
        let splitter = SourceSplitter::new(4);
        let blocks = splitter.split(src);
        assert_eq!(blocks.len(), 1);
    }
    #[test]
    fn test_indent_normaliser_all_spaces() {
        let norm = IndentNormaliser::new(4);
        assert_eq!(norm.normalise_line("  hello"), "  hello");
    }
    #[test]
    fn test_aligned_printer_empty() {
        let printer = AlignedPrinter::new(":=");
        assert!(printer.is_empty());
        assert_eq!(printer.format(), "");
    }
    #[test]
    fn test_gcd_large() {
        assert_eq!(gcd(100, 75), 25);
        assert_eq!(gcd(17, 13), 1);
    }
    #[test]
    fn test_indent_delta_change() {
        let d = IndentDelta {
            line: 2,
            before: 0,
            after: 4,
        };
        assert_eq!(d.change(), 4);
        let d2 = IndentDelta {
            line: 5,
            before: 8,
            after: 4,
        };
        assert_eq!(d2.change(), -4);
    }
    #[test]
    fn test_indent_consistency_report_step() {
        let src = "def foo := 1\n    body\n";
        let report = IndentConsistencyReport::from_source(src);
        assert!(report.step > 0 || report.total_indented > 0);
    }
    #[test]
    fn test_line_class_starts_decl() {
        assert!(LineClass::DeclOpener.starts_decl());
        assert!(!LineClass::Other.starts_decl());
        assert!(!LineClass::Blank.starts_decl());
    }
    #[test]
    fn test_line_class_is_ignorable() {
        assert!(LineClass::Blank.is_ignorable());
        assert!(LineClass::Comment.is_ignorable());
        assert!(!LineClass::DeclOpener.is_ignorable());
    }
}
pub fn compare_indent(prev: &IndentLevel, next: &IndentLevel, tab_width: usize) -> IndentDiff {
    let pw = prev.total_width(tab_width);
    let nw = next.total_width(tab_width);
    if nw > pw {
        IndentDiff::Indent
    } else if nw == pw {
        IndentDiff::Same
    } else {
        IndentDiff::Dedent
    }
}
#[allow(dead_code)]
pub fn compute_indent_guides(indent_levels: &[usize], line_col: usize) -> Vec<IndentGuide> {
    indent_levels
        .iter()
        .filter(|&&col| col < line_col)
        .map(|&col| IndentGuide::new(col, false))
        .collect()
}
#[allow(dead_code)]
pub(super) fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    use crate::indent_tracker::*;
    #[test]
    fn test_layout_rule_same_block() {
        let rule = LayoutRule::SameBlock(4);
        assert!(rule.matches(4));
        assert!(!rule.matches(0));
        assert_eq!(rule.pivot(), 4);
    }
    #[test]
    fn test_layout_context_indent_col_spaces() {
        assert_eq!(LayoutContext::indent_col("    hello", 4), 4);
    }
    #[test]
    fn test_layout_context_indent_col_tab() {
        assert_eq!(LayoutContext::indent_col("\thello", 4), 4);
    }
    #[test]
    fn test_compare_indent() {
        let a = IndentLevel::new(2, 0);
        let b = IndentLevel::new(4, 0);
        assert_eq!(compare_indent(&a, &b, 4), IndentDiff::Indent);
        assert_eq!(compare_indent(&b, &a, 4), IndentDiff::Dedent);
    }
    #[test]
    fn test_block_parser_simple() {
        let mut bp = BlockParser::new(4);
        let lines = vec!["def foo : Nat := 0", "    body", "def bar := 1"];
        let blocks = bp.parse_blocks(&lines);
        assert!(!blocks.is_empty());
    }
    #[test]
    fn test_indent_normaliser_tab_expansion() {
        let norm = IndentNormaliser::new(4);
        assert_eq!(norm.normalise_line("\thello"), "    hello");
    }
    #[test]
    fn test_scope_tracker_resolve() {
        let mut tracker = ScopeTracker::new();
        tracker.enter("where", IndentLevel::new(0, 0));
        tracker.bind("foo");
        assert!(tracker.resolve("foo").is_some());
        assert!(tracker.resolve("bar").is_none());
        tracker.exit();
    }
    #[test]
    fn test_line_classifier() {
        assert_eq!(LineClass::classify(""), LineClass::Blank);
        assert_eq!(LineClass::classify("-- comment"), LineClass::Comment);
        assert_eq!(LineClass::classify("def foo := 1"), LineClass::DeclOpener);
        assert_eq!(LineClass::classify("where"), LineClass::WhereKeyword);
        assert_eq!(LineClass::classify("let x := 5"), LineClass::LetBinding);
    }
    #[test]
    fn test_indent_stats_spaces_only() {
        let src = "    x\n        y\n    z\n";
        let stats = IndentStats::analyse(src);
        assert!(stats.is_spaces_only());
    }
    #[test]
    fn test_do_block_tracker() {
        let mut tracker = DoBlockTracker::new(4);
        tracker.enter(4);
        assert!(tracker.is_statement("    x <- foo"));
        tracker.exit();
    }
    #[test]
    fn test_let_binding_tracker() {
        let mut tracker = LetBindingTracker::new();
        tracker.push(LetBinding::new("x", 0, true));
        assert!(tracker.resolve("x").is_some());
    }
    #[test]
    fn test_indent_guide() {
        let levels = vec![0, 4, 8];
        let guides = compute_indent_guides(&levels, 12);
        assert_eq!(guides.len(), 3);
    }
    #[test]
    fn test_tab_stop_iterator() {
        let stops: Vec<usize> = TabStopIterator::new(4).take(5).collect();
        assert_eq!(stops, vec![0, 4, 8, 12, 16]);
    }
    #[test]
    fn test_indent_delta() {
        let src = "def foo := 1\n    body\ndef bar := 2\n";
        let deltas = IndentDelta::compute_all(src, 4);
        assert!(!deltas.is_empty());
    }
    #[test]
    fn test_hanging_indent() {
        let hi = HangingIndent::new(0, 4);
        assert!(hi.is_first(0));
        assert!(hi.is_continuation(4));
        assert_eq!(hi.overhang(), 4);
    }
    #[test]
    fn test_line_span() {
        let span = LineSpan::new(3, 7);
        assert_eq!(span.len(), 5);
        assert!(span.contains(5));
        let merged = span.merge(LineSpan::new(6, 10));
        assert_eq!(merged.end, 10);
    }
    #[test]
    fn test_indent_validator_spaces() {
        let mut v = IndentValidator::expect_spaces();
        v.validate("    hello\n");
        assert!(v.is_valid());
    }
    #[test]
    fn test_indent_fixer_no_op() {
        let fixer = IndentFixer::new(4, 4);
        let src = "def foo := 1\n    body\n";
        let fixed = fixer.fix(src);
        assert!(fixed.contains("    body"));
    }
    #[test]
    fn test_whitespace_kind() {
        let src = "foo   bar";
        let kind = WhitespaceKind::classify(src, 3, 6);
        assert!(matches!(kind, WhitespaceKind::MultiSpace(3)));
        assert!(!kind.contains_newline());
    }
    #[test]
    fn test_aligned_printer() {
        let mut printer = AlignedPrinter::new(":=");
        printer.add(0, "foo", "1");
        printer.add(0, "longname", "2");
        let output = printer.format();
        let lines: Vec<&str> = output.lines().collect();
        let col0 = lines[0].find(":=").expect("lookup should succeed");
        let col1 = lines[1].find(":=").expect("lookup should succeed");
        assert_eq!(col0, col1);
    }
    #[test]
    fn test_comment_tracker() {
        let mut ct = CommentTracker::new();
        ct.process_pair('-', '-');
        assert!(ct.in_comment());
        ct.end_of_line();
        assert!(!ct.in_comment());
    }
    #[test]
    fn test_open_brace_tracker() {
        let mut t = OpenBraceTracker::new();
        t.push('(', 1, 0);
        assert!(t.pop(')', 1, 5).is_ok());
        assert!(t.is_balanced());
    }
    #[test]
    fn test_indent_rewriter() {
        let rw = IndentRewriter::new(2, 4);
        assert_eq!(rw.rewrite_line("  hello"), "    hello");
    }
    #[test]
    fn test_indent_history() {
        let src = "def foo := 1\n    body\n";
        let hist = IndentHistory::from_source(src, 4);
        assert_eq!(hist.max_indent(), 4);
    }
    #[test]
    fn test_multiline_string_tracker() {
        let mut t = MultilineStringTracker::new();
        t.open(5);
        assert!(t.in_string);
        t.close();
        assert!(!t.in_string);
    }
    #[test]
    fn test_source_splitter() {
        let src = "def foo := 1\ndef bar := 2\n";
        let splitter = SourceSplitter::new(4);
        let blocks = splitter.split(src);
        assert_eq!(blocks.len(), 2);
    }
    #[test]
    fn test_column_aligner() {
        let items = vec!["a".to_string(), "bb".to_string(), "ccc".to_string()];
        let aligned = ColumnAligner::align_all(&items);
        assert!(aligned.iter().all(|s| s.len() == 3));
    }
    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 4), 4);
        assert_eq!(gcd(8, 4), 4);
        assert_eq!(gcd(6, 4), 2);
    }
    #[test]
    fn test_indent_consistency_report() {
        let src = "def foo := 1\n    body\ndef bar := 2\n";
        let report = IndentConsistencyReport::from_source(src);
        assert!(report.uses_spaces);
        assert!(!report.uses_tabs);
    }
}
/// Computes the visual width of a string (tabs expand to tab_width).
#[allow(dead_code)]
pub fn visual_width(s: &str, tab_width: usize) -> usize {
    let mut col = 0;
    for ch in s.chars() {
        if ch == '\t' {
            col = ((col / tab_width) + 1) * tab_width;
        } else {
            col += 1;
        }
    }
    col
}
/// Converts a string with tabs to spaces.
#[allow(dead_code)]
pub fn expand_tabs(s: &str, tab_width: usize) -> String {
    let mut out = String::new();
    let mut col = 0;
    for ch in s.chars() {
        if ch == '\t' {
            let next = ((col / tab_width) + 1) * tab_width;
            for _ in col..next {
                out.push(' ');
            }
            col = next;
        } else {
            out.push(ch);
            col += 1;
        }
    }
    out
}
/// Converts spaces to tabs where possible (unexpand).
#[allow(dead_code)]
pub fn compress_spaces_to_tabs(s: &str, tab_width: usize) -> String {
    let leading_spaces = s.len() - s.trim_start_matches(' ').len();
    let tabs = leading_spaces / tab_width;
    let remaining = leading_spaces % tab_width;
    let mut out = "\t".repeat(tabs);
    for _ in 0..remaining {
        out.push(' ');
    }
    out.push_str(s.trim_start_matches(' '));
    out
}
/// Checks if a source string has consistent indentation (all-spaces or all-tabs).
#[allow(dead_code)]
pub fn has_mixed_indentation(source: &str) -> bool {
    let has_tabs = source.lines().any(|l| l.starts_with('\t'));
    let has_spaces = source.lines().any(|l| l.starts_with(' '));
    has_tabs && has_spaces
}
/// Computes a "signature" of the indentation pattern of a source string.
#[allow(dead_code)]
pub fn indent_signature(source: &str) -> Vec<usize> {
    source
        .lines()
        .map(|l| l.len() - l.trim_start().len())
        .collect()
}
/// Checks if two sources have the same indentation structure.
#[allow(dead_code)]
pub fn same_indent_structure(a: &str, b: &str) -> bool {
    indent_signature(a) == indent_signature(b)
}
/// Normalises indentation to use exactly `unit` spaces per level.
#[allow(dead_code)]
pub fn normalise_indentation(source: &str, unit: usize) -> String {
    if unit == 0 {
        return source.to_string();
    }
    // Detect the source indent unit as the smallest non-zero leading whitespace.
    let source_unit = source
        .lines()
        .map(|line| line.len() - line.trim_start().len())
        .filter(|&n| n > 0)
        .min()
        .unwrap_or(unit);
    source
        .lines()
        .map(|line| {
            let leading = line.len() - line.trim_start().len();
            let level = leading / source_unit.max(1);
            format!("{}{}", " ".repeat(level * unit), line.trim_start())
        })
        .collect::<Vec<_>>()
        .join("\n")
}
/// Splits source into indent blocks.
#[allow(dead_code)]
pub fn split_into_indent_blocks(source: &str) -> Vec<IndentBlock> {
    let mut blocks: Vec<IndentBlock> = Vec::new();
    for line in source.lines() {
        let level = line.len() - line.trim_start().len();
        if let Some(last) = blocks.last_mut() {
            if last.level == level {
                last.add_line(line);
                continue;
            }
        }
        let mut block = IndentBlock::new(level);
        block.add_line(line);
        blocks.push(block);
    }
    blocks
}
/// Computes the longest common indent prefix of a set of lines.
#[allow(dead_code)]
pub fn common_indent(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.len() - l.trim_start().len())
        .min()
        .unwrap_or(0)
}
/// Dedents all lines in a source by `n` spaces.
#[allow(dead_code)]
pub fn dedent(source: &str, n: usize) -> String {
    source
        .lines()
        .map(|l| {
            if l.len() >= n && l[..n].chars().all(|c| c == ' ') {
                &l[n..]
            } else {
                l.trim_start()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
/// Applies a function to the indentation of each line.
#[allow(dead_code)]
pub fn map_indent<F: Fn(usize) -> usize>(source: &str, f: F) -> String {
    source
        .lines()
        .map(|l| {
            let indent = l.len() - l.trim_start().len();
            let new_indent = f(indent);
            format!("{}{}", " ".repeat(new_indent), l.trim_start())
        })
        .collect::<Vec<_>>()
        .join("\n")
}
/// Detects the dominant indentation mode of a source.
#[allow(dead_code)]
pub fn detect_indent_mode(source: &str) -> IndentMode {
    let tab_lines = source.lines().filter(|l| l.starts_with('\t')).count();
    let space_lines = source.lines().filter(|l| l.starts_with("  ")).count();
    if tab_lines > space_lines {
        IndentMode::Tabs(4)
    } else {
        IndentMode::Spaces(2)
    }
}
/// Converts a source from one indent mode to another.
#[allow(dead_code)]
pub fn convert_indent_mode(source: &str, from: IndentMode, to: IndentMode) -> String {
    let unit = from.unit_width();
    source
        .lines()
        .map(|l| {
            let stripped = l.trim_start();
            let leading = l.len() - stripped.len();
            let level = if unit == 0 {
                0
            } else {
                match from {
                    IndentMode::Tabs(_) => l.chars().take_while(|&c| c == '\t').count(),
                    IndentMode::Spaces(u) => leading / u,
                }
            };
            format!("{}{}", to.indent_str(level), stripped)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
#[cfg(test)]
mod extended_indent_final_tests {
    use super::*;
    use crate::indent_tracker::*;
    #[test]
    fn test_virtual_column() {
        let col = VirtualColumn::new(0, 4);
        let col2 = col.advance_by(3);
        assert_eq!(col2.column, 3);
        let col3 = col2.advance_tab();
        assert_eq!(col3.column, 4);
        assert!(col3.is_aligned_to(4));
    }
    #[test]
    fn test_visual_width() {
        assert_eq!(visual_width("hello", 4), 5);
        assert_eq!(visual_width("\thello", 4), 9);
    }
    #[test]
    fn test_expand_tabs() {
        let s = expand_tabs("\thello", 4);
        assert_eq!(s, "    hello");
    }
    #[test]
    fn test_has_mixed_indentation() {
        assert!(has_mixed_indentation("  space\n\ttab"));
        assert!(!has_mixed_indentation("  a\n  b"));
        assert!(!has_mixed_indentation("\ta\n\tb"));
    }
    #[test]
    fn test_column_oracle() {
        let mut oracle = ColumnOracle::new(4);
        oracle.add_reference(4);
        oracle.add_reference(8);
        assert_eq!(oracle.next_alignment(0), 4);
        assert_eq!(oracle.next_alignment(5), 8);
        assert!(oracle.is_at_reference(4));
        assert!(!oracle.is_at_reference(3));
    }
    #[test]
    fn test_construct_rule_registry() {
        let reg = ConstructRuleRegistry::new();
        assert!(reg.lookup("def").is_some());
        assert_eq!(reg.body_indent("def"), 2);
        assert_eq!(reg.body_indent("unknown"), 2);
    }
    #[test]
    fn test_indent_signature() {
        let sig = indent_signature("a\n  b\n    c");
        assert_eq!(sig, vec![0, 2, 4]);
    }
    #[test]
    fn test_same_indent_structure() {
        assert!(same_indent_structure("a\n  b", "x\n  y"));
        assert!(!same_indent_structure("a\n  b", "a\n    b"));
    }
    #[test]
    fn test_normalise_indentation() {
        let s = normalise_indentation("a\n    b\n        c", 2);
        assert_eq!(s, "a\n  b\n    c");
    }
    #[test]
    fn test_split_into_indent_blocks() {
        let src = "a\n  b\n  c\nd";
        let blocks = split_into_indent_blocks(src);
        assert!(blocks.len() >= 2);
    }
    #[test]
    fn test_indent_fence() {
        let mut fence = IndentFence::new(2);
        assert!(fence.allows(2));
        assert!(fence.allows(4));
        assert!(!fence.allows(0));
        fence.deactivate();
        assert!(fence.allows(0));
    }
    #[test]
    fn test_common_indent() {
        let lines = ["  a", "    b", "  c"];
        assert_eq!(common_indent(&lines), 2);
        let empty: Vec<&str> = vec![];
        assert_eq!(common_indent(&empty), 0);
    }
    #[test]
    fn test_dedent() {
        let s = dedent("  hello\n  world", 2);
        assert_eq!(s, "hello\nworld");
    }
    #[test]
    fn test_map_indent() {
        let s = map_indent("a\n  b\n    c", |n| n + 2);
        assert_eq!(s, "  a\n    b\n      c");
    }
    #[test]
    fn test_indent_mode() {
        let m = IndentMode::Spaces(2);
        assert_eq!(m.unit_width(), 2);
        assert_eq!(m.indent_str(3), "      ");
        let t = IndentMode::Tabs(4);
        assert_eq!(t.indent_str(2), "\t\t");
    }
    #[test]
    fn test_detect_indent_mode() {
        let with_tabs = "\thello\n\tworld";
        let m = detect_indent_mode(with_tabs);
        assert_eq!(m, IndentMode::Tabs(4));
        let with_spaces = "  hello\n  world";
        let m2 = detect_indent_mode(with_spaces);
        assert_eq!(m2, IndentMode::Spaces(2));
    }
    #[test]
    fn test_convert_indent_mode() {
        let src = "a\n  b\n    c";
        let converted = convert_indent_mode(src, IndentMode::Spaces(2), IndentMode::Spaces(4));
        assert_eq!(converted, "a\n    b\n        c");
    }
    #[test]
    fn test_compress_spaces_to_tabs() {
        let s = compress_spaces_to_tabs("    hello", 4);
        assert!(s.starts_with('\t'));
        assert!(s.contains("hello"));
    }
}
/// Scans a source and generates an indent change log.
#[allow(dead_code)]
pub fn compute_indent_changes(source: &str) -> IndentChangeLog {
    let mut log = IndentChangeLog::new();
    let mut prev = 0usize;
    for (i, line) in source.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let cur = line.len() - line.trim_start().len();
        log.record(i, prev, cur);
        prev = cur;
    }
    log
}
/// An indent-level mapper: maps each line to its logical block depth.
#[allow(dead_code)]
pub fn map_to_block_depths(source: &str, unit: usize) -> Vec<usize> {
    source
        .lines()
        .map(|l| {
            if unit == 0 {
                return 0;
            }
            let indent = l.len() - l.trim_start().len();
            indent / unit
        })
        .collect()
}
/// Checks whether a line is a continuation of the previous (hanging indent).
#[allow(dead_code)]
pub fn is_continuation_line(prev_indent: usize, cur_indent: usize, unit: usize) -> bool {
    unit > 0 && cur_indent > prev_indent && (cur_indent - prev_indent) % unit != 0
}
/// Produces a visual representation of the indent structure.
#[allow(dead_code)]
pub fn visualise_indent_structure(source: &str) -> String {
    source
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let indent = l.len() - l.trim_start().len();
            let bar = "|".repeat(indent / 2);
            format!("{:4} {}{}", i + 1, bar, l.trim_start())
        })
        .collect::<Vec<_>>()
        .join("\n")
}
/// A simple depth-first indent traversal.
#[allow(dead_code)]
pub fn traverse_indent_tree(source: &str, unit: usize) -> Vec<(usize, usize, String)> {
    source
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.trim().is_empty())
        .map(|(i, l)| {
            let indent = l.len() - l.trim_start().len();
            let depth = indent.checked_div(unit).unwrap_or(0);
            (depth, i, l.trim().to_string())
        })
        .collect()
}
/// Checks if a source is "well-formed" in terms of indentation.
/// Well-formed: every indent increase is exactly `unit` spaces.
#[allow(dead_code)]
pub fn is_well_formed_indentation(source: &str, unit: usize) -> bool {
    if unit == 0 {
        return true;
    }
    let mut prev = 0usize;
    for line in source.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let cur = line.len() - line.trim_start().len();
        if cur > prev && (cur - prev) % unit != 0 {
            return false;
        }
        if cur % unit != 0 {
            return false;
        }
        prev = cur;
    }
    true
}
/// Repair indentation: round all indents to the nearest multiple of `unit`.
#[allow(dead_code)]
pub fn repair_indentation(source: &str, unit: usize) -> String {
    if unit == 0 {
        return source.to_string();
    }
    source
        .lines()
        .map(|l| {
            let indent = l.len() - l.trim_start().len();
            let rounded = ((indent + unit / 2) / unit) * unit;
            format!("{}{}", " ".repeat(rounded), l.trim_start())
        })
        .collect::<Vec<_>>()
        .join("\n")
}
#[cfg(test)]
mod extended_indent_extra_tests {
    use super::*;
    use crate::indent_tracker::*;
    #[test]
    fn test_indent_change_log() {
        let src = "a\n  b\n    c\n  d\ne";
        let log = compute_indent_changes(src);
        assert!(log.increases() >= 2);
        assert!(log.decreases() >= 1);
    }
    #[test]
    fn test_map_to_block_depths() {
        let depths = map_to_block_depths("a\n  b\n    c", 2);
        assert_eq!(depths, vec![0, 1, 2]);
    }
    #[test]
    fn test_hanging_indent_state() {
        let mut state = HangingIndentState::new(0);
        assert_eq!(state.current_indent(), 0);
        state.enter_hanging();
        assert_eq!(state.current_indent(), 4);
        state.exit_hanging();
        assert_eq!(state.current_indent(), 0);
    }
    #[test]
    fn test_is_continuation_line() {
        assert!(is_continuation_line(0, 6, 4));
        assert!(!is_continuation_line(0, 4, 4));
    }
    #[test]
    fn test_visualise_indent_structure() {
        let src = "a\n  b\n    c";
        let vis = visualise_indent_structure(src);
        assert!(vis.contains("a"));
        assert!(vis.contains("|"));
    }
    #[test]
    fn test_traverse_indent_tree() {
        let src = "a\n  b\n    c\n  d";
        let tree = traverse_indent_tree(src, 2);
        assert_eq!(tree[0].0, 0);
        assert_eq!(tree[1].0, 1);
        assert_eq!(tree[2].0, 2);
    }
    #[test]
    fn test_is_well_formed_indentation() {
        assert!(is_well_formed_indentation("a\n  b\n    c", 2));
        assert!(!is_well_formed_indentation("a\n   b", 2));
    }
    #[test]
    fn test_repair_indentation() {
        let repaired = repair_indentation("a\n   b\n     c", 2);
        assert!(is_well_formed_indentation(&repaired, 2));
    }
    #[test]
    fn test_indent_zipper() {
        let src = "a\n  b\n    c";
        let mut z = IndentZipper::from_source(src).expect("test operation should succeed");
        assert_eq!(z.current_indent(), 0);
        assert!(z.move_down());
        assert_eq!(z.current_indent(), 2);
        assert!(z.move_down());
        assert_eq!(z.current_indent(), 4);
        assert!(!z.move_down());
        assert!(z.move_up());
        assert_eq!(z.current_indent(), 2);
    }
}
