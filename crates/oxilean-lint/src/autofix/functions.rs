//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{
    AddSemicolonFix, AnnotatedFix, AsciiOnlyFix, AutofixRegistry, BatchFixApplicator,
    CommentOutFix, ConflictDetector, DuplicateImportFix, FixConfidence, FixEngine, FixFilter,
    FixHistory, FixMetrics, FixPipeline, FixPreview, FixReport, FixScorer, FixSuggestion,
    IndentFix, InsertLineAfterFix, InsertLineBeforeFix, LineRange, MissingDocFix,
    NamingConventionFix, RemoveDeadCodeFix, RenameIdentifierFix, SortImportsFix, SpellingFix,
    SyntaxRewriter, TextEdit, TypeAnnotationFix, UncommentFix, UndoStack, UnicodeFix,
    UnusedImportFix, WhitespaceFix,
};

pub trait AutofixRule {
    fn suggest_fix(
        &self,
        source: &str,
        span_start: usize,
        span_end: usize,
    ) -> Option<FixSuggestion>;
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn text_edit_apply_replace() {
        let edit = TextEdit::new(5, 10, " new ");
        assert_eq!(edit.apply("hello_____world"), "hello new world");
        let edit2 = TextEdit::new(0, 5, "Bye");
        assert_eq!(edit2.apply("Hello world"), "Bye world");
    }
    #[test]
    fn text_edit_is_deletion() {
        let edit = TextEdit::new(0, 5, "");
        assert!(edit.is_deletion());
        assert!(!edit.is_insertion());
    }
    #[test]
    fn text_edit_is_insertion() {
        let edit = TextEdit::new(3, 3, "XYZ");
        assert!(edit.is_insertion());
        assert!(!edit.is_deletion());
    }
    #[test]
    fn fix_suggestion_apply_all() {
        let mut fix = FixSuggestion::new("test");
        fix.add_edit(TextEdit::new(0, 3, "AAA"));
        fix.add_edit(TextEdit::new(4, 7, "BBB"));
        let result = fix.apply_all("foo bar");
        assert_eq!(result, "AAA BBB");
    }
    #[test]
    fn fix_suggestion_title() {
        let fix = FixSuggestion::new("My Fix");
        assert_eq!(fix.title, "My Fix");
        assert!(fix.is_safe);
    }
    #[test]
    fn unused_import_fix_removes_line() {
        let source = "line one\nimport Foo\nline three\n";
        let fix = UnusedImportFix;
        let result = fix
            .suggest_fix(source, 9, 19)
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert_eq!(applied, "line one\nline three\n");
    }
    #[test]
    fn missing_doc_fix_inserts_placeholder() {
        let source = "def foo := 1\n";
        let fix = MissingDocFix;
        let result = fix
            .suggest_fix(source, 0, 3)
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert!(applied.contains("/// TODO"));
        assert!(!result.is_safe);
    }
    #[test]
    fn naming_convention_to_snake_case() {
        assert_eq!(
            NamingConventionFix::to_snake_case("CamelCase"),
            "camel_case"
        );
        assert_eq!(NamingConventionFix::to_snake_case("myVar"), "my_var");
        assert_eq!(
            NamingConventionFix::to_snake_case("already_snake"),
            "already_snake"
        );
    }
    #[test]
    fn naming_convention_fix_applies() {
        let source = "let MyVar = 1";
        let fix = NamingConventionFix;
        let result = fix
            .suggest_fix(source, 4, 9)
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert_eq!(applied, "let my_var = 1");
    }
    #[test]
    fn autofix_registry_register_and_get() {
        let mut registry = AutofixRegistry::new();
        registry.register("unused_import", Box::new(UnusedImportFix));
        registry.register("naming_convention", Box::new(NamingConventionFix));
        let source = "line one\nimport Foo\nline three\n";
        let fix = registry.get_fix("unused_import", source, 9, 19);
        assert!(fix.is_some());
        let missing = registry.get_fix("nonexistent", source, 0, 5);
        assert!(missing.is_none());
    }
    #[test]
    fn autofix_registry_available_fixes_sorted() {
        let mut registry = AutofixRegistry::new();
        registry.register("zzz_fix", Box::new(UnusedImportFix));
        registry.register("aaa_fix", Box::new(MissingDocFix));
        let keys = registry.available_fixes();
        assert_eq!(keys, vec!["aaa_fix", "zzz_fix"]);
    }
}
/// Replaces all occurrences of `old_text` with `new_text` in the source.
#[allow(dead_code)]
pub fn replace_all_occurrences(source: &str, old_text: &str, new_text: &str) -> String {
    source.replace(old_text, new_text)
}
/// Removes trailing whitespace from every line.
#[allow(dead_code)]
pub fn strip_trailing_whitespace(source: &str) -> String {
    source
        .lines()
        .map(|l| l.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}
/// Normalises line endings to `\n`.
#[allow(dead_code)]
pub fn normalise_line_endings(source: &str) -> String {
    source.replace("\r\n", "\n").replace('\r', "\n")
}
/// Reindents a block of text by replacing the leading indentation of each line.
///
/// `old_indent` is the prefix to remove; `new_indent` is the prefix to add.
/// Lines that do not start with `old_indent` are left unchanged.
#[allow(dead_code)]
pub fn reindent(source: &str, old_indent: &str, new_indent: &str) -> String {
    source
        .lines()
        .map(|l| {
            if l.starts_with(old_indent) {
                format!("{}{}", new_indent, &l[old_indent.len()..])
            } else {
                l.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
/// Wraps lines that exceed `max_width` characters by inserting a newline
/// and continuation indent.
#[allow(dead_code)]
pub fn wrap_long_lines(source: &str, max_width: usize, cont_indent: &str) -> String {
    let mut result = Vec::new();
    for line in source.lines() {
        if line.len() <= max_width {
            result.push(line.to_string());
        } else {
            let mut remaining = line;
            let mut first = true;
            while remaining.len() > max_width {
                let split_at = max_width;
                if first {
                    result.push(remaining[..split_at].to_string());
                    first = false;
                } else {
                    result.push(format!("{}{}", cont_indent, &remaining[..split_at]));
                }
                remaining = &remaining[split_at..];
            }
            if !remaining.is_empty() {
                if first {
                    result.push(remaining.to_string());
                } else {
                    result.push(format!("{}{}", cont_indent, remaining));
                }
            }
        }
    }
    result.join("\n")
}
/// Returns a vector of line numbers (1-based) that differ between `a` and `b`.
#[allow(dead_code)]
pub fn diff_lines(a: &str, b: &str) -> Vec<usize> {
    let a_lines: Vec<&str> = a.lines().collect();
    let b_lines: Vec<&str> = b.lines().collect();
    let max_len = a_lines.len().max(b_lines.len());
    let mut changed = Vec::new();
    for i in 0..max_len {
        let al = a_lines.get(i).copied().unwrap_or("");
        let bl = b_lines.get(i).copied().unwrap_or("");
        if al != bl {
            changed.push(i + 1);
        }
    }
    changed
}
/// Count how many characters differ between two strings of the same length.
/// For strings of different lengths, the difference in length is also counted.
#[allow(dead_code)]
pub fn count_changed_chars(a: &str, b: &str) -> usize {
    let ac: Vec<char> = a.chars().collect();
    let bc: Vec<char> = b.chars().collect();
    let min_len = ac.len().min(bc.len());
    let mut count = ac.len().abs_diff(bc.len());
    for i in 0..min_len {
        if ac[i] != bc[i] {
            count += 1;
        }
    }
    count
}
/// Extract the identifier starting at byte `pos` in `source`.
/// Returns an empty string if `pos` is out of bounds or the character is not
/// an identifier start.
#[allow(dead_code)]
pub fn extract_identifier(source: &str, pos: usize) -> &str {
    if pos >= source.len() {
        return "";
    }
    let rest = &source[pos..];
    let end = rest
        .char_indices()
        .take_while(|(_, c)| c.is_alphanumeric() || *c == '_' || *c == '\'')
        .last()
        .map(|(i, c)| i + c.len_utf8())
        .unwrap_or(0);
    &rest[..end]
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn fix_confidence_ordering() {
        assert!(FixConfidence::Certain > FixConfidence::High);
        assert!(FixConfidence::High > FixConfidence::Medium);
        assert!(FixConfidence::Medium > FixConfidence::Low);
    }
    #[test]
    fn fix_confidence_display() {
        assert_eq!(format!("{}", FixConfidence::Certain), "certain");
        assert_eq!(format!("{}", FixConfidence::Low), "low");
    }
    #[test]
    fn annotated_fix_is_safe_to_apply() {
        let fix = FixSuggestion::new("test");
        let af = AnnotatedFix::new(fix, FixConfidence::High);
        assert!(af.is_safe_to_apply());
        let fix2 = FixSuggestion::new("test2");
        let af2 = AnnotatedFix::new(fix2, FixConfidence::Low);
        assert!(!af2.is_safe_to_apply());
    }
    #[test]
    fn annotated_fix_with_explanation() {
        let fix = FixSuggestion::new("test");
        let af = AnnotatedFix::new(fix, FixConfidence::Medium).with_explanation("because reasons");
        assert_eq!(af.explanation.as_deref(), Some("because reasons"));
    }
    #[test]
    fn fix_preview_shows_changed_lines() {
        let source = "line1\nfoo bar\nline3\n";
        let mut fix = FixSuggestion::new("test");
        fix.add_edit(TextEdit::new(6, 13, "replaced"));
        let (before, after) = FixPreview::preview(&fix, source);
        assert!(before.contains("- "));
        assert!(after.contains("+ "));
    }
    #[test]
    fn fix_preview_unified_diff() {
        let source = "aaa\nbbb\n";
        let mut fix = FixSuggestion::new("test");
        fix.add_edit(TextEdit::new(4, 7, "ccc"));
        let diff = FixPreview::unified_diff(&fix, source);
        assert!(diff.contains('-') || diff.contains('+'));
    }
    #[test]
    fn batch_fix_applicator_no_conflict() {
        let applicator = BatchFixApplicator::new();
        let mut fix1 = FixSuggestion::new("fix1");
        fix1.add_edit(TextEdit::new(0, 3, "AAA"));
        let mut fix2 = FixSuggestion::new("fix2");
        fix2.add_edit(TextEdit::new(4, 7, "BBB"));
        let (result, applied) = applicator.apply_batch(&[fix1, fix2], "foo bar");
        assert_eq!(result, "AAA BBB");
        assert_eq!(applied.len(), 2);
    }
    #[test]
    fn batch_fix_applicator_skip_conflict() {
        let applicator = BatchFixApplicator {
            skip_on_conflict: true,
        };
        let mut fix1 = FixSuggestion::new("fix1");
        fix1.add_edit(TextEdit::new(0, 5, "AAAAA"));
        let mut fix2 = FixSuggestion::new("fix2");
        fix2.add_edit(TextEdit::new(2, 4, "BB"));
        let (_, applied) = applicator.apply_batch(&[fix1, fix2], "hello world");
        assert_eq!(applied.len(), 1);
    }
    #[test]
    fn undo_stack_basic() {
        let mut stack = UndoStack::new("original");
        let mut fix = FixSuggestion::new("test");
        fix.add_edit(TextEdit::new(0, 8, "modified"));
        stack.apply(&fix);
        assert_eq!(stack.current(), "modified");
        let prev = stack.undo();
        assert_eq!(prev, Some("original"));
        assert_eq!(stack.current(), "original");
        let redone = stack.redo();
        assert_eq!(redone, Some("modified"));
        assert_eq!(stack.current(), "modified");
    }
    #[test]
    fn undo_stack_nothing_to_undo() {
        let mut stack = UndoStack::new("initial");
        assert!(stack.undo().is_none());
    }
    #[test]
    fn undo_stack_nothing_to_redo() {
        let mut stack = UndoStack::new("initial");
        assert!(stack.redo().is_none());
    }
    #[test]
    fn conflict_detector_finds_overlaps() {
        let edits = vec![
            TextEdit::new(0, 5, ""),
            TextEdit::new(3, 8, ""),
            TextEdit::new(10, 15, ""),
        ];
        let conflicts = ConflictDetector::find_conflicts(&edits);
        assert_eq!(conflicts.len(), 1);
        assert_eq!(conflicts[0], (0, 1));
    }
    #[test]
    fn conflict_detector_no_overlaps() {
        let edits = vec![
            TextEdit::new(0, 3, ""),
            TextEdit::new(5, 8, ""),
            TextEdit::new(10, 13, ""),
        ];
        assert!(ConflictDetector::is_conflict_free(&edits));
    }
    #[test]
    fn test_replace_all() {
        let result = replace_all_occurrences("foo foo foo", "foo", "bar");
        assert_eq!(result, "bar bar bar");
    }
    #[test]
    fn test_strip_trailing_whitespace() {
        let src = "hello   \nworld  \nok\n";
        let cleaned = strip_trailing_whitespace(src);
        assert!(!cleaned.contains("   "));
        assert!(!cleaned.contains("  "));
    }
    #[test]
    fn test_normalise_line_endings() {
        let src = "a\r\nb\rc\n";
        let result = normalise_line_endings(src);
        assert_eq!(result, "a\nb\nc\n");
    }
    #[test]
    fn test_reindent() {
        let src = "    foo\n    bar\nunindented";
        let result = reindent(src, "    ", "  ");
        assert_eq!(result, "  foo\n  bar\nunindented");
    }
    #[test]
    fn test_wrap_long_lines() {
        let src = "a".repeat(20);
        let result = wrap_long_lines(&src, 10, "  ");
        assert!(result.contains('\n'));
    }
    #[test]
    fn syntax_rewriter_applies_rules() {
        let mut rw = SyntaxRewriter::new();
        rw.add_rule("=>", "->");
        rw.add_rule("∀ ", "forall ");
        let result = rw.rewrite("∀ x, P x => Q x");
        assert_eq!(result, "forall x, P x -> Q x");
        assert_eq!(rw.rule_count(), 2);
    }
    #[test]
    fn whitespace_fix_removes_trailing_spaces() {
        let source = "hello   \nworld  \n";
        let fix = WhitespaceFix;
        let result = fix.suggest_fix(source, 0, source.len());
        assert!(result.is_some());
        let applied = result
            .expect("fix result should be present")
            .apply_all(source);
        assert!(!applied.contains("   "));
    }
    #[test]
    fn duplicate_import_fix_removes_duplicates() {
        let source = "import Foo\nimport Bar\nimport Foo\ntheorem t : True := trivial";
        let result = DuplicateImportFix::deduplicate(source);
        let import_foo_count = result.matches("import Foo").count();
        assert_eq!(import_foo_count, 1);
    }
    #[test]
    fn sort_imports_fix_sorts() {
        let source = "import Zzz\nimport Aaa\nimport Mmm\n\ntheorem t : True := trivial";
        let sorted = SortImportsFix::sort_imports(source);
        let lines: Vec<&str> = sorted.lines().collect();
        assert!(lines[0] <= lines[1]);
    }
    #[test]
    fn fix_scorer_safe_fix_scores_higher() {
        let mut safe_fix = FixSuggestion::new("safe");
        safe_fix.is_safe = true;
        safe_fix.add_edit(TextEdit::new(0, 1, "a"));
        let mut unsafe_fix = FixSuggestion::new("unsafe");
        unsafe_fix.is_safe = false;
        unsafe_fix.add_edit(TextEdit::new(0, 1, "a"));
        let safe_score = FixScorer::score(&safe_fix);
        let unsafe_score = FixScorer::score(&unsafe_fix);
        assert!(safe_score > unsafe_score);
    }
    #[test]
    fn fix_pipeline_runs_steps() {
        let mut pipeline = FixPipeline::new();
        pipeline.add_step("whitespace", Box::new(WhitespaceFix));
        pipeline.add_step("sort_imports", Box::new(SortImportsFix));
        assert_eq!(pipeline.step_count(), 2);
        let source = "import Zzz\nimport Aaa\nfoo   \n";
        let (result, _applied) = pipeline.run(source);
        assert_ne!(result, source);
    }
    #[test]
    fn diff_lines_finds_changed() {
        let a = "line1\nline2\nline3\n";
        let b = "line1\nXXXX\nline3\n";
        let changed = diff_lines(a, b);
        assert_eq!(changed, vec![2]);
    }
    #[test]
    fn diff_lines_no_change() {
        let a = "same\n";
        let b = "same\n";
        assert!(diff_lines(a, b).is_empty());
    }
    #[test]
    fn count_changed_chars_equal() {
        assert_eq!(count_changed_chars("abc", "abc"), 0);
    }
    #[test]
    fn count_changed_chars_different() {
        assert_eq!(count_changed_chars("abc", "axc"), 1);
    }
    #[test]
    fn count_changed_chars_different_length() {
        assert_eq!(count_changed_chars("abc", "abcde"), 2);
    }
    #[test]
    fn extract_identifier_basic() {
        let src = "foo_bar baz";
        let id = extract_identifier(src, 0);
        assert_eq!(id, "foo_bar");
    }
    #[test]
    fn extract_identifier_at_space() {
        let src = "foo bar";
        let id = extract_identifier(src, 4);
        assert_eq!(id, "bar");
    }
    #[test]
    fn extract_identifier_out_of_bounds() {
        let src = "foo";
        let id = extract_identifier(src, 100);
        assert_eq!(id, "");
    }
    #[test]
    fn fix_report_counts() {
        let report = FixReport::new(
            vec!["fix_a".to_string(), "fix_b".to_string()],
            vec!["skipped_c".to_string()],
            "final source".to_string(),
        );
        assert!(report.any_applied());
        assert_eq!(report.applied_count(), 2);
        assert_eq!(report.skipped_count(), 1);
    }
    #[test]
    fn fix_report_none_applied() {
        let report = FixReport::new(vec![], vec![], "source".to_string());
        assert!(!report.any_applied());
    }
    #[test]
    fn type_annotation_fix_inserts() {
        let source = "let x = 1";
        let fix = TypeAnnotationFix::new("Nat");
        let result = fix
            .suggest_fix(source, 5, 5)
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert!(applied.contains(": Nat"));
    }
    #[test]
    fn remove_dead_code_fix_deletes_span() {
        let source = "keep this [DEAD CODE] and keep this";
        let fix = RemoveDeadCodeFix;
        let result = fix
            .suggest_fix(source, 10, 21)
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert!(!applied.contains("[DEAD CODE]"));
        assert!(applied.contains("keep this"));
    }
    #[test]
    fn add_semicolon_fix_appends() {
        let source = "let x = 1";
        let fix = AddSemicolonFix;
        let result = fix
            .suggest_fix(source, 0, source.len())
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert!(applied.ends_with(';'));
    }
    #[test]
    fn add_semicolon_fix_noop_when_present() {
        let source = "let x = 1;";
        let fix = AddSemicolonFix;
        let result = fix.suggest_fix(source, 0, source.len());
        assert!(result.is_none());
    }
}
#[cfg(test)]
mod extended_tests {
    use super::*;
    #[test]
    fn line_range_contains() {
        let r = LineRange::new(3, 7);
        assert!(r.contains_line(3));
        assert!(r.contains_line(7));
        assert!(!r.contains_line(2));
        assert!(!r.contains_line(8));
    }
    #[test]
    fn line_range_len() {
        assert_eq!(LineRange::new(3, 7).len(), 5);
        assert_eq!(LineRange::single(5).len(), 1);
    }
    #[test]
    fn line_range_to_byte_range() {
        let source = "line1\nline2\nline3\n";
        let r = LineRange::new(2, 2);
        let (start, end) = r.to_byte_range(source);
        assert_eq!(&source[start..end], "line2\n");
    }
    #[test]
    fn comment_out_fix() {
        let source = "theorem foo : True := trivial";
        let fix = CommentOutFix;
        let result = fix
            .suggest_fix(source, 0, source.len())
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert!(applied.starts_with("{-"));
        assert!(applied.ends_with("-}"));
    }
    #[test]
    fn uncomment_fix() {
        let source = "{- theorem foo : True := trivial -}";
        let fix = UncommentFix;
        let result = fix
            .suggest_fix(source, 0, source.len())
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert!(!applied.contains("{-"));
        assert!(!applied.contains("-}"));
    }
    #[test]
    fn rename_identifier_fix() {
        let source = "def foo := foo + foo";
        let fix = RenameIdentifierFix::new("foo", "bar");
        let result = fix
            .suggest_fix(source, 0, source.len())
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert_eq!(applied, "def bar := bar + bar");
    }
    #[test]
    fn rename_identifier_fix_noop() {
        let source = "def baz := 1";
        let fix = RenameIdentifierFix::new("foo", "bar");
        assert!(fix.suggest_fix(source, 0, source.len()).is_none());
    }
    #[test]
    fn insert_line_after_fix() {
        let source = "line1\nline2\nline3";
        let fix = InsertLineAfterFix::new("-- inserted");
        let result = fix
            .suggest_fix(source, 0, 5)
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert!(applied.contains("-- inserted"));
        let lines: Vec<&str> = applied.lines().collect();
        assert_eq!(lines[1], "-- inserted");
    }
    #[test]
    fn insert_line_before_fix() {
        let source = "line1\nline2\nline3";
        let fix = InsertLineBeforeFix::new("-- before");
        let result = fix
            .suggest_fix(source, 6, 11)
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert!(applied.contains("-- before"));
    }
    #[test]
    fn indent_fix_add() {
        let source = "foo\nbar";
        let fix = IndentFix::new(2);
        let result = fix
            .suggest_fix(source, 0, source.len())
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        for line in applied.lines() {
            assert!(line.starts_with("  "));
        }
    }
    #[test]
    fn indent_fix_remove() {
        let source = "  foo\n  bar";
        let fix = IndentFix::new(-2);
        let result = fix
            .suggest_fix(source, 0, source.len())
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        for line in applied.lines() {
            assert!(!line.starts_with("  "));
        }
    }
    #[test]
    fn spelling_fix_corrects() {
        let source = "theroem foo : True := trivial";
        let fix = SpellingFix::new();
        let result = fix
            .suggest_fix(source, 0, source.len())
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert!(applied.contains("theorem"));
        assert!(!applied.contains("theroem"));
    }
    #[test]
    fn spelling_fix_noop_when_correct() {
        let source = "theorem foo : True := trivial";
        let fix = SpellingFix::new();
        assert!(fix.suggest_fix(source, 0, source.len()).is_none());
    }
    #[test]
    fn unicode_fix_converts_operators() {
        let source = "forall x, P x -> Q x";
        let fix = UnicodeFix;
        let result = fix
            .suggest_fix(source, 0, source.len())
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert!(applied.contains("→") || applied.contains("∀"));
    }
    #[test]
    fn ascii_only_fix_converts_unicode() {
        let source = "∀ x, P x → Q x";
        let fix = AsciiOnlyFix;
        let result = fix
            .suggest_fix(source, 0, source.len())
            .expect("suggest_fix should return a result");
        let applied = result.apply_all(source);
        assert!(applied.contains("forall") || applied.contains("->"));
    }
    #[test]
    fn fix_history_records() {
        let mut history = FixHistory::new();
        let mut fix = FixSuggestion::new("remove trailing ws");
        fix.add_edit(TextEdit::new(3, 6, ""));
        history.record(&fix, "foo   bar");
        assert_eq!(history.total_fixes(), 1);
        assert_eq!(history.entries()[0].fix_title, "remove trailing ws");
    }
    #[test]
    fn fix_history_find_by_title() {
        let mut history = FixHistory::new();
        let mut fix = FixSuggestion::new("my fix");
        fix.add_edit(TextEdit::new(0, 1, "X"));
        history.record(&fix, "hello");
        history.record(&fix, "world");
        let found = history.find_by_title("my fix");
        assert_eq!(found.len(), 2);
    }
    #[test]
    fn fix_metrics_compute() {
        let mut safe = FixSuggestion::new("safe");
        safe.is_safe = true;
        safe.add_edit(TextEdit::new(0, 3, "abc"));
        let mut unsafe_ = FixSuggestion::new("unsafe");
        unsafe_.is_safe = false;
        unsafe_.add_edit(TextEdit::new(5, 8, "xyz"));
        let metrics = FixMetrics::compute(&[safe, unsafe_]);
        assert_eq!(metrics.total_fixes, 2);
        assert_eq!(metrics.safe_fixes, 1);
        assert_eq!(metrics.unsafe_fixes, 1);
        assert!(!metrics.all_safe());
    }
    #[test]
    fn fix_filter_by_confidence() {
        let fix1 = AnnotatedFix::new(FixSuggestion::new("high"), FixConfidence::High);
        let fix2 = AnnotatedFix::new(FixSuggestion::new("low"), FixConfidence::Low);
        let fixes = vec![fix1, fix2];
        let filter = FixFilter::new(FixConfidence::High, false);
        let filtered = filter.apply(&fixes);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].suggestion.title, "high");
    }
    #[test]
    fn fix_filter_safe_only() {
        let mut unsafe_fix = FixSuggestion::new("unsafe");
        unsafe_fix.is_safe = false;
        let af_unsafe = AnnotatedFix::new(unsafe_fix, FixConfidence::High);
        let safe_fix = FixSuggestion::new("safe");
        let af_safe = AnnotatedFix::new(safe_fix, FixConfidence::High);
        let fixes = vec![af_unsafe, af_safe];
        let filter = FixFilter::new(FixConfidence::Low, true);
        let filtered = filter.apply(&fixes);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].suggestion.title, "safe");
    }
    #[test]
    fn fix_engine_run() {
        let mut registry = AutofixRegistry::new();
        registry.register("unused_import", Box::new(UnusedImportFix));
        let mut engine = FixEngine::new(registry);
        let source = "line one\nimport Foo\nline three\n";
        engine.queue("unused_import", 9, 19);
        let report = engine.run(source);
        assert!(report.any_applied());
    }
    #[test]
    fn fix_scorer_rank() {
        let af1 = AnnotatedFix::new(FixSuggestion::new("a"), FixConfidence::High);
        let af2 = AnnotatedFix::new(FixSuggestion::new("b"), FixConfidence::Low);
        let ranked = FixScorer::rank_suggestions(vec![af1, af2]);
        assert_eq!(ranked.len(), 2);
    }
    #[test]
    fn syntax_rewriter_empty_source() {
        let mut rw = SyntaxRewriter::new();
        rw.add_rule("foo", "bar");
        assert_eq!(rw.rewrite(""), "");
    }
    #[test]
    fn undo_stack_multiple_undo_redo() {
        let mut stack = UndoStack::new("v0");
        let mut fix1 = FixSuggestion::new("f1");
        fix1.add_edit(TextEdit::new(0, 2, "v1"));
        stack.apply(&fix1);
        let mut fix2 = FixSuggestion::new("f2");
        fix2.add_edit(TextEdit::new(0, 2, "v2"));
        stack.apply(&fix2);
        assert_eq!(stack.history_len(), 3);
        stack.undo();
        stack.undo();
        assert_eq!(stack.current(), "v0");
        stack.redo();
        assert_eq!(stack.history_len(), 2);
    }
}
/// Convert a 1-based (line, col) pair to a byte offset in `source`.
/// Returns `None` if the position is out of range.
#[allow(dead_code)]
pub fn line_col_to_byte_offset(source: &str, line: usize, col: usize) -> Option<usize> {
    let mut current_line = 1usize;
    let mut current_col = 1usize;
    for (i, ch) in source.char_indices() {
        if current_line == line && current_col == col {
            return Some(i);
        }
        if ch == '\n' {
            current_line += 1;
            current_col = 1;
        } else {
            current_col += 1;
        }
    }
    None
}
/// Convert a byte offset to a 1-based (line, col) pair.
#[allow(dead_code)]
pub fn byte_offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1usize;
    let mut col = 1usize;
    for (i, ch) in source.char_indices() {
        if i == offset {
            return (line, col);
        }
        if ch == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}
#[cfg(test)]
mod pos_tests {
    use super::*;
    #[test]
    fn round_trip_line_col() {
        let source = "hello\nworld\nfoo";
        let offset =
            line_col_to_byte_offset(source, 2, 1).expect("byte offset conversion should succeed");
        assert_eq!(offset, 6);
        let (l, c) = byte_offset_to_line_col(source, 6);
        assert_eq!((l, c), (2, 1));
    }
}
