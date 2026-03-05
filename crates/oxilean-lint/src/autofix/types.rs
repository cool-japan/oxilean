//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::functions::AutofixRule;

/// Inserts a type annotation at a given position.
#[allow(dead_code)]
pub struct TypeAnnotationFix {
    pub type_name: String,
}
impl TypeAnnotationFix {
    #[allow(dead_code)]
    pub fn new(type_name: &str) -> Self {
        Self {
            type_name: type_name.to_string(),
        }
    }
}
/// Removes a block of dead code identified by a start and end byte offset.
#[allow(dead_code)]
pub struct RemoveDeadCodeFix;
/// Runs a sequence of `AutofixRule`s over a source string, threading the
/// result of each rule into the next.
#[allow(dead_code)]
pub struct FixPipeline {
    steps: Vec<(String, Box<dyn AutofixRule>)>,
}
impl FixPipeline {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }
    /// Add a named step to the pipeline.
    #[allow(dead_code)]
    pub fn add_step(&mut self, name: &str, rule: Box<dyn AutofixRule>) {
        self.steps.push((name.to_string(), rule));
    }
    /// Execute the pipeline.  Returns the final source and the names of
    /// steps that produced a non-trivial fix.
    #[allow(dead_code)]
    pub fn run(&self, source: &str) -> (String, Vec<String>) {
        let mut current = source.to_string();
        let mut applied_steps = Vec::new();
        for (name, rule) in &self.steps {
            if let Some(fix) = rule.suggest_fix(&current, 0, current.len()) {
                let new_source = fix.apply_all(&current);
                if new_source != current {
                    current = new_source;
                    applied_steps.push(name.clone());
                }
            }
        }
        (current, applied_steps)
    }
    /// Return the number of pipeline steps.
    #[allow(dead_code)]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}
/// A summary report of what fixes were applied and what was skipped.
#[allow(dead_code)]
pub struct FixReport {
    pub applied: Vec<String>,
    pub skipped: Vec<String>,
    pub final_source: String,
}
impl FixReport {
    #[allow(dead_code)]
    pub fn new(applied: Vec<String>, skipped: Vec<String>, final_source: String) -> Self {
        Self {
            applied,
            skipped,
            final_source,
        }
    }
    /// True when at least one fix was applied.
    #[allow(dead_code)]
    pub fn any_applied(&self) -> bool {
        !self.applied.is_empty()
    }
    /// Count how many fixes were applied.
    #[allow(dead_code)]
    pub fn applied_count(&self) -> usize {
        self.applied.len()
    }
    /// Count how many fixes were skipped.
    #[allow(dead_code)]
    pub fn skipped_count(&self) -> usize {
        self.skipped.len()
    }
}
/// Renames every occurrence of `old_name` to `new_name` in the source.
#[allow(dead_code)]
pub struct RenameIdentifierFix {
    pub old_name: String,
    pub new_name: String,
}
impl RenameIdentifierFix {
    #[allow(dead_code)]
    pub fn new(old_name: &str, new_name: &str) -> Self {
        Self {
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
        }
    }
}
/// Applies multiple `FixSuggestion`s to a source string in a conflict-aware way.
#[allow(dead_code)]
pub struct BatchFixApplicator {
    /// Whether to skip fixes that conflict with already-applied edits.
    pub skip_on_conflict: bool,
}
impl BatchFixApplicator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            skip_on_conflict: true,
        }
    }
    /// Check whether two `TextEdit`s overlap.
    #[allow(dead_code)]
    pub fn edits_overlap(a: &TextEdit, b: &TextEdit) -> bool {
        a.range_start < b.range_end && b.range_start < a.range_end
    }
    /// Apply a list of suggestions to `source`, skipping conflicting ones when
    /// `self.skip_on_conflict` is true.
    ///
    /// Returns the final source and the indices of applied suggestions.
    #[allow(dead_code)]
    pub fn apply_batch(&self, suggestions: &[FixSuggestion], source: &str) -> (String, Vec<usize>) {
        let mut result = source.to_string();
        let mut applied: Vec<usize> = Vec::new();
        let mut committed_edits: Vec<&TextEdit> = Vec::new();
        for (idx, suggestion) in suggestions.iter().enumerate() {
            if self.skip_on_conflict {
                let conflicts = suggestion.edits.iter().any(|new_edit| {
                    committed_edits
                        .iter()
                        .any(|old| Self::edits_overlap(old, new_edit))
                });
                if conflicts {
                    continue;
                }
            }
            result = suggestion.apply_all(&result);
            for e in &suggestion.edits {
                committed_edits.push(e);
            }
            applied.push(idx);
        }
        (result, applied)
    }
}
/// Filters a list of `AnnotatedFix`es according to configurable criteria.
#[allow(dead_code)]
pub struct FixFilter {
    pub min_confidence: FixConfidence,
    pub safe_only: bool,
}
impl FixFilter {
    #[allow(dead_code)]
    pub fn new(min_confidence: FixConfidence, safe_only: bool) -> Self {
        Self {
            min_confidence,
            safe_only,
        }
    }
    /// Apply the filter and return only matching fixes.
    #[allow(dead_code)]
    pub fn apply<'a>(&self, fixes: &'a [AnnotatedFix]) -> Vec<&'a AnnotatedFix> {
        fixes
            .iter()
            .filter(|af| {
                af.confidence >= self.min_confidence && (!self.safe_only || af.suggestion.is_safe)
            })
            .collect()
    }
}
/// Sorts import lines alphabetically within the leading import block.
#[allow(dead_code)]
pub struct SortImportsFix;
impl SortImportsFix {
    /// Extract the leading import block (consecutive lines starting with "import").
    #[allow(dead_code)]
    pub fn sort_imports(source: &str) -> String {
        let mut lines: Vec<&str> = source.lines().collect();
        let import_end = lines
            .iter()
            .position(|l| !l.trim().is_empty() && !l.trim().starts_with("import "))
            .unwrap_or(lines.len());
        let import_block = &mut lines[..import_end];
        import_block.sort_unstable();
        let mut parts: Vec<String> = lines.iter().map(|l| l.to_string()).collect();
        parts[..import_end].sort_unstable();
        parts.join("\n")
    }
}
/// Removes trailing whitespace from every line in the span.
#[allow(dead_code)]
pub struct WhitespaceFix;
/// Renders a unified-diff-style preview of a fix applied to source.
#[allow(dead_code)]
pub struct FixPreview;
impl FixPreview {
    /// Return a simplified before/after pair for the changed lines.
    ///
    /// Returns `(before_snippet, after_snippet)` where each snippet is
    /// a multiline string of the affected region.
    #[allow(dead_code)]
    pub fn preview(suggestion: &FixSuggestion, source: &str) -> (String, String) {
        let after = suggestion.apply_all(source);
        let before_lines: Vec<&str> = source.lines().collect();
        let after_lines: Vec<&str> = after.lines().collect();
        let mut before_diff = Vec::new();
        let mut after_diff = Vec::new();
        let max_len = before_lines.len().max(after_lines.len());
        for i in 0..max_len {
            let b = before_lines.get(i).copied().unwrap_or("");
            let a = after_lines.get(i).copied().unwrap_or("");
            if b != a {
                before_diff.push(format!("- {}", b));
                after_diff.push(format!("+ {}", a));
            }
        }
        (before_diff.join("\n"), after_diff.join("\n"))
    }
    /// Return a compact diff string combining before and after lines.
    #[allow(dead_code)]
    pub fn unified_diff(suggestion: &FixSuggestion, source: &str) -> String {
        let (before, after) = Self::preview(suggestion, source);
        format!("{}\n{}", before, after)
    }
}
/// Inserts a line of text immediately before the line containing `span_start`.
#[allow(dead_code)]
pub struct InsertLineBeforeFix {
    pub line_to_insert: String,
}
impl InsertLineBeforeFix {
    #[allow(dead_code)]
    pub fn new(line: &str) -> Self {
        Self {
            line_to_insert: line.to_string(),
        }
    }
}
/// Adds or removes a fixed amount of indentation from a span of lines.
#[allow(dead_code)]
pub struct IndentFix {
    /// Positive: add spaces; negative: remove spaces.
    pub delta: i32,
}
impl IndentFix {
    #[allow(dead_code)]
    pub fn new(delta: i32) -> Self {
        Self { delta }
    }
}
/// A fix suggestion paired with a confidence level and an optional explanation.
#[allow(dead_code)]
pub struct AnnotatedFix {
    pub suggestion: FixSuggestion,
    pub confidence: FixConfidence,
    pub explanation: Option<String>,
}
impl AnnotatedFix {
    #[allow(dead_code)]
    pub fn new(suggestion: FixSuggestion, confidence: FixConfidence) -> Self {
        Self {
            suggestion,
            confidence,
            explanation: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_explanation(mut self, expl: &str) -> Self {
        self.explanation = Some(expl.to_string());
        self
    }
    /// Returns true when confidence is Certain or High.
    #[allow(dead_code)]
    pub fn is_safe_to_apply(&self) -> bool {
        self.confidence >= FixConfidence::High
    }
}
/// A single text edit: replace [range_start, range_end) with new_text.
pub struct TextEdit {
    pub range_start: usize,
    pub range_end: usize,
    pub new_text: String,
}
impl TextEdit {
    pub fn new(start: usize, end: usize, text: &str) -> Self {
        Self {
            range_start: start,
            range_end: end,
            new_text: text.to_string(),
        }
    }
    /// Apply this edit to `source`, returning the modified string.
    pub fn apply(&self, source: &str) -> String {
        let start = self.range_start.min(source.len());
        let end = self.range_end.min(source.len());
        let mut result = String::new();
        result.push_str(&source[..start]);
        result.push_str(&self.new_text);
        result.push_str(&source[end..]);
        result
    }
    /// Returns true when new_text is empty (pure deletion).
    pub fn is_deletion(&self) -> bool {
        self.new_text.is_empty()
    }
    /// Returns true when range_start == range_end (pure insertion).
    pub fn is_insertion(&self) -> bool {
        self.range_start == self.range_end
    }
}
/// Removes `{- ... -}` wrapping from a block comment span.
#[allow(dead_code)]
pub struct UncommentFix;
/// Wraps a span in an OxiLean block comment `{- ... -}`.
#[allow(dead_code)]
pub struct CommentOutFix;
/// A fix suggestion composed of one or more text edits.
pub struct FixSuggestion {
    pub title: String,
    pub edits: Vec<TextEdit>,
    pub is_safe: bool,
}
impl FixSuggestion {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            edits: Vec::new(),
            is_safe: true,
        }
    }
    pub fn add_edit(&mut self, edit: TextEdit) {
        self.edits.push(edit);
    }
    /// Apply all edits in reverse order (to preserve byte offsets).
    pub fn apply_all(&self, source: &str) -> String {
        let mut sorted: Vec<&TextEdit> = self.edits.iter().collect();
        sorted.sort_by(|a, b| b.range_start.cmp(&a.range_start));
        let mut result = source.to_string();
        for edit in sorted {
            result = edit.apply(&result);
        }
        result
    }
}
/// Applies a sequence of pattern→replacement rules to source text.
#[allow(dead_code)]
pub struct SyntaxRewriter {
    rules: Vec<(String, String)>,
}
impl SyntaxRewriter {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    /// Register a rule: replace every occurrence of `pattern` with `replacement`.
    #[allow(dead_code)]
    pub fn add_rule(&mut self, pattern: &str, replacement: &str) {
        self.rules
            .push((pattern.to_string(), replacement.to_string()));
    }
    /// Apply all rules in registration order.
    #[allow(dead_code)]
    pub fn rewrite(&self, source: &str) -> String {
        let mut result = source.to_string();
        for (pat, rep) in &self.rules {
            result = result.replace(pat.as_str(), rep.as_str());
        }
        result
    }
    /// Return the number of registered rules.
    #[allow(dead_code)]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}
/// Confidence level that a generated fix is correct.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(dead_code)]
pub enum FixConfidence {
    /// The fix may be wrong; human review required.
    Low,
    /// The fix is a reasonable guess.
    Medium,
    /// The fix is very likely correct.
    High,
    /// The fix is provably correct.
    Certain,
}
/// Appends a semicolon to lines that are missing one (naively).
#[allow(dead_code)]
pub struct AddSemicolonFix;
/// A range of lines in a source file (both inclusive, 1-based).
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LineRange {
    pub start: usize,
    pub end: usize,
}
impl LineRange {
    #[allow(dead_code)]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    #[allow(dead_code)]
    pub fn single(line: usize) -> Self {
        Self {
            start: line,
            end: line,
        }
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start) + 1
    }
    #[allow(dead_code)]
    pub fn contains_line(&self, line: usize) -> bool {
        line >= self.start && line <= self.end
    }
    /// Convert line range to byte range in `source`.
    #[allow(dead_code)]
    pub fn to_byte_range(&self, source: &str) -> (usize, usize) {
        let mut start_byte = 0usize;
        let mut end_byte = source.len();
        let mut current_line = 1usize;
        let mut in_range = false;
        for (i, ch) in source.char_indices() {
            if current_line == self.start && !in_range {
                start_byte = i;
                in_range = true;
            }
            if ch == '\n' {
                current_line += 1;
                if in_range && current_line > self.end {
                    end_byte = i + 1;
                    break;
                }
            }
        }
        (start_byte, end_byte)
    }
}
/// Inserts a line of text immediately after the line containing `span_start`.
#[allow(dead_code)]
pub struct InsertLineAfterFix {
    pub line_to_insert: String,
}
impl InsertLineAfterFix {
    #[allow(dead_code)]
    pub fn new(line: &str) -> Self {
        Self {
            line_to_insert: line.to_string(),
        }
    }
}
/// Inserts a `/// TODO` doc-comment placeholder before the item.
pub struct MissingDocFix;
/// Converts the identifier in the span to snake_case.
pub struct NamingConventionFix;
impl NamingConventionFix {
    pub fn to_snake_case(s: &str) -> String {
        let mut result = String::new();
        let mut prev_upper = false;
        for (i, ch) in s.chars().enumerate() {
            if ch.is_uppercase() {
                if i > 0 && !prev_upper {
                    result.push('_');
                }
                result.push(ch.to_lowercase().next().unwrap_or(ch));
                prev_upper = true;
            } else {
                result.push(ch);
                prev_upper = false;
            }
        }
        result
    }
}
/// Replaces a commonly misspelled word in the span with the correct spelling.
#[allow(dead_code)]
pub struct SpellingFix {
    /// Map from misspelling to correct form.
    pub corrections: HashMap<String, String>,
}
impl SpellingFix {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut corrections = HashMap::new();
        corrections.insert("theroem".to_string(), "theorem".to_string());
        corrections.insert("lemam".to_string(), "lemma".to_string());
        corrections.insert("porof".to_string(), "proof".to_string());
        corrections.insert("corollry".to_string(), "corollary".to_string());
        corrections.insert("defnition".to_string(), "definition".to_string());
        Self { corrections }
    }
    #[allow(dead_code)]
    pub fn add_correction(&mut self, wrong: &str, right: &str) {
        self.corrections
            .insert(wrong.to_string(), right.to_string());
    }
}
/// Replaces Unicode operator symbols with ASCII equivalents.
#[allow(dead_code)]
pub struct AsciiOnlyFix;
impl AsciiOnlyFix {
    #[allow(dead_code)]
    pub(super) fn replacements() -> Vec<(&'static str, &'static str)> {
        vec![
            ("→", "->"),
            ("↔", "<->"),
            ("∀", "forall"),
            ("∃", "exists"),
            ("∧", "/\\"),
            ("∨", "\\/"),
            ("¬", "~"),
            ("≤", "<="),
            ("≥", ">="),
            ("≠", "<>"),
        ]
    }
}
/// Scores fix suggestions on a 0.0–1.0 scale based on heuristics.
#[allow(dead_code)]
pub struct FixScorer;
impl FixScorer {
    /// Score a fix based on:
    /// - Whether it is safe (`is_safe`)
    /// - How few edits it requires (fewer → higher score)
    /// - Total number of characters changed (fewer → higher score)
    #[allow(dead_code)]
    pub fn score(suggestion: &FixSuggestion) -> f64 {
        let safety_bonus = if suggestion.is_safe { 0.4 } else { 0.0 };
        let edit_penalty = (suggestion.edits.len() as f64 * 0.05).min(0.3);
        let char_change: usize = suggestion
            .edits
            .iter()
            .map(|e| e.new_text.len() + (e.range_end.saturating_sub(e.range_start)))
            .sum();
        let size_penalty = (char_change as f64 / 1000.0).min(0.3);
        (0.6 + safety_bonus - edit_penalty - size_penalty).clamp(0.0, 1.0)
    }
    /// Sort suggestions by descending score and return a new vec.
    #[allow(dead_code)]
    pub fn rank_suggestions(suggestions: Vec<AnnotatedFix>) -> Vec<AnnotatedFix> {
        let mut scored: Vec<(f64, AnnotatedFix)> = suggestions
            .into_iter()
            .map(|af| {
                let s = Self::score(&af.suggestion);
                (s, af)
            })
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().map(|(_, af)| af).collect()
    }
}
/// Detects conflicts between collections of `TextEdit`s.
#[allow(dead_code)]
pub struct ConflictDetector;
impl ConflictDetector {
    /// Return pairs of (i, j) indices that conflict among the given edits.
    #[allow(dead_code)]
    pub fn find_conflicts(edits: &[TextEdit]) -> Vec<(usize, usize)> {
        let mut conflicts = Vec::new();
        for i in 0..edits.len() {
            for j in (i + 1)..edits.len() {
                if BatchFixApplicator::edits_overlap(&edits[i], &edits[j]) {
                    conflicts.push((i, j));
                }
            }
        }
        conflicts
    }
    /// Returns `true` if the given edits are all non-conflicting.
    #[allow(dead_code)]
    pub fn is_conflict_free(edits: &[TextEdit]) -> bool {
        Self::find_conflicts(edits).is_empty()
    }
}
/// Removes the entire line containing the unused import.
pub struct UnusedImportFix;
/// A single entry in the fix history.
#[allow(dead_code)]
pub struct FixHistoryEntry {
    pub timestamp: u64,
    pub fix_title: String,
    pub source_before: String,
    pub source_after: String,
}
/// Aggregated metrics about fix suggestions.
#[allow(dead_code)]
pub struct FixMetrics {
    pub total_fixes: usize,
    pub safe_fixes: usize,
    pub unsafe_fixes: usize,
    pub total_edits: usize,
    pub total_chars_changed: usize,
}
impl FixMetrics {
    /// Compute metrics from a slice of suggestions.
    #[allow(dead_code)]
    pub fn compute(suggestions: &[FixSuggestion]) -> Self {
        let total_fixes = suggestions.len();
        let safe_fixes = suggestions.iter().filter(|s| s.is_safe).count();
        let unsafe_fixes = total_fixes - safe_fixes;
        let total_edits: usize = suggestions.iter().map(|s| s.edits.len()).sum();
        let total_chars_changed: usize = suggestions
            .iter()
            .flat_map(|s| s.edits.iter())
            .map(|e| e.new_text.len() + e.range_end.saturating_sub(e.range_start))
            .sum();
        Self {
            total_fixes,
            safe_fixes,
            unsafe_fixes,
            total_edits,
            total_chars_changed,
        }
    }
    /// True when all fixes are safe.
    #[allow(dead_code)]
    pub fn all_safe(&self) -> bool {
        self.unsafe_fixes == 0
    }
}
/// Logs every applied fix with a timestamp (simulated as a monotonically
/// increasing counter) for audit purposes.
#[allow(dead_code)]
pub struct FixHistory {
    entries: Vec<FixHistoryEntry>,
    counter: u64,
}
impl FixHistory {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            counter: 0,
        }
    }
    /// Record a fix application.
    #[allow(dead_code)]
    pub fn record(&mut self, fix: &FixSuggestion, before: &str) {
        let after = fix.apply_all(before);
        self.counter += 1;
        self.entries.push(FixHistoryEntry {
            timestamp: self.counter,
            fix_title: fix.title.clone(),
            source_before: before.to_string(),
            source_after: after,
        });
    }
    /// Retrieve all history entries.
    #[allow(dead_code)]
    pub fn entries(&self) -> &[FixHistoryEntry] {
        &self.entries
    }
    /// Count the total number of fixes recorded.
    #[allow(dead_code)]
    pub fn total_fixes(&self) -> usize {
        self.entries.len()
    }
    /// Find entries by fix title.
    #[allow(dead_code)]
    pub fn find_by_title(&self, title: &str) -> Vec<&FixHistoryEntry> {
        self.entries
            .iter()
            .filter(|e| e.fix_title == title)
            .collect()
    }
}
/// Replaces ASCII operator sequences with their Unicode equivalents.
#[allow(dead_code)]
pub struct UnicodeFix;
impl UnicodeFix {
    #[allow(dead_code)]
    pub(super) fn replacements() -> Vec<(&'static str, &'static str)> {
        vec![
            ("->", "→"),
            ("<->", "↔"),
            ("forall ", "∀ "),
            ("exists ", "∃ "),
            ("/\\", "∧"),
            ("\\/", "∨"),
            ("~", "¬"),
            ("<=", "≤"),
            (">=", "≥"),
            ("<>", "≠"),
        ]
    }
}
/// Removes duplicate import lines from source text.
#[allow(dead_code)]
pub struct DuplicateImportFix;
impl DuplicateImportFix {
    /// Return source with duplicate import lines removed (keeps first occurrence).
    #[allow(dead_code)]
    pub fn deduplicate(source: &str) -> String {
        let mut seen = std::collections::HashSet::new();
        let mut lines_out = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("import ") {
                if seen.insert(trimmed.to_string()) {
                    lines_out.push(line);
                }
            } else {
                lines_out.push(line);
            }
        }
        lines_out.join("\n")
    }
}
/// High-level engine that runs a set of named rules and produces a report.
#[allow(dead_code)]
pub struct FixEngine {
    registry: AutofixRegistry,
    /// Pairs of (lint_code, span_start, span_end).
    pending: Vec<(String, usize, usize)>,
}
impl FixEngine {
    #[allow(dead_code)]
    pub fn new(registry: AutofixRegistry) -> Self {
        Self {
            registry,
            pending: Vec::new(),
        }
    }
    /// Queue a lint occurrence for fixing.
    #[allow(dead_code)]
    pub fn queue(&mut self, lint_code: &str, start: usize, end: usize) {
        self.pending.push((lint_code.to_string(), start, end));
    }
    /// Apply all queued fixes and produce a report.
    #[allow(dead_code)]
    pub fn run(&mut self, source: &str) -> FixReport {
        let applicator = BatchFixApplicator::new();
        let mut suggestions: Vec<FixSuggestion> = Vec::new();
        let mut skipped_codes: Vec<String> = Vec::new();
        for (code, start, end) in self.pending.drain(..) {
            match self.registry.get_fix(&code, source, start, end) {
                Some(fix) => suggestions.push(fix),
                None => skipped_codes.push(code),
            }
        }
        let (final_source, applied_indices) = applicator.apply_batch(&suggestions, source);
        let applied_names: Vec<String> = applied_indices
            .into_iter()
            .map(|i| suggestions[i].title.clone())
            .collect();
        FixReport::new(applied_names, skipped_codes, final_source)
    }
}
pub struct AutofixRegistry {
    pub rules: HashMap<String, Box<dyn AutofixRule>>,
}
impl AutofixRegistry {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }
    pub fn register(&mut self, lint_code: &str, fix: Box<dyn AutofixRule>) {
        self.rules.insert(lint_code.to_string(), fix);
    }
    pub fn get_fix(
        &self,
        lint_code: &str,
        source: &str,
        start: usize,
        end: usize,
    ) -> Option<FixSuggestion> {
        self.rules
            .get(lint_code)
            .and_then(|rule| rule.suggest_fix(source, start, end))
    }
    pub fn available_fixes(&self) -> Vec<&str> {
        let mut keys: Vec<&str> = self.rules.keys().map(|s| s.as_str()).collect();
        keys.sort();
        keys
    }
}
/// A simple undo/redo stack for fix applications.
#[allow(dead_code)]
pub struct UndoStack {
    history: Vec<String>,
    redo_stack: Vec<String>,
}
impl UndoStack {
    #[allow(dead_code)]
    pub fn new(initial_source: &str) -> Self {
        Self {
            history: vec![initial_source.to_string()],
            redo_stack: Vec::new(),
        }
    }
    /// Push the current state before applying a fix, then apply it.
    #[allow(dead_code)]
    pub fn apply(&mut self, suggestion: &FixSuggestion) -> &str {
        let current = self.current().to_string();
        let new_source = suggestion.apply_all(&current);
        self.history.push(new_source);
        self.redo_stack.clear();
        self.current()
    }
    /// Return the current source.
    #[allow(dead_code)]
    pub fn current(&self) -> &str {
        self.history.last().map(|s| s.as_str()).unwrap_or("")
    }
    /// Undo the last fix application.  Returns `None` if nothing to undo.
    #[allow(dead_code)]
    pub fn undo(&mut self) -> Option<&str> {
        if self.history.len() <= 1 {
            return None;
        }
        let top = self
            .history
            .pop()
            .expect("history has at least 2 elements: checked by early return");
        self.redo_stack.push(top);
        Some(self.current())
    }
    /// Redo the last undone fix.  Returns `None` if nothing to redo.
    #[allow(dead_code)]
    pub fn redo(&mut self) -> Option<&str> {
        let top = self.redo_stack.pop()?;
        self.history.push(top);
        Some(self.current())
    }
    /// How many states are in the history (including the current one).
    #[allow(dead_code)]
    pub fn history_len(&self) -> usize {
        self.history.len()
    }
}
