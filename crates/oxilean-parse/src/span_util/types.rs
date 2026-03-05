//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::tokens::Span;

/// A registry mapping `FileId` to file paths and source strings.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub struct FileRegistry {
    entries: Vec<(FileId, String, String)>,
    next_id: u32,
}
impl FileRegistry {
    /// Create an empty registry.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            next_id: 1,
        }
    }
    /// Register a new file, returning its `FileId`.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn register(&mut self, path: impl Into<String>, source: impl Into<String>) -> FileId {
        let id = FileId(self.next_id);
        self.next_id += 1;
        self.entries.push((id, path.into(), source.into()));
        id
    }
    /// Look up the source string for a file.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn source(&self, id: FileId) -> Option<&str> {
        self.entries
            .iter()
            .find(|(fid, _, _)| *fid == id)
            .map(|(_, _, src)| src.as_str())
    }
    /// Look up the path for a file.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn path(&self, id: FileId) -> Option<&str> {
        self.entries
            .iter()
            .find(|(fid, _, _)| *fid == id)
            .map(|(_, p, _)| p.as_str())
    }
    /// Number of registered files.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// `true` if no files are registered.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Extract text at a `FileSpan`.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn extract(&self, fspan: &FileSpan) -> &str {
        self.source(fspan.file)
            .and_then(|src| src.get(fspan.span.start..fspan.span.end))
            .unwrap_or("")
    }
}
/// A diagnostic span: a source span with severity and message.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct DiagnosticSpan {
    /// The source span.
    pub span: Span,
    /// Severity level.
    pub severity: SpanSeverity,
    /// Diagnostic message.
    #[allow(missing_docs)]
    pub message: String,
}
impl DiagnosticSpan {
    /// Create a new diagnostic span.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(span: Span, severity: SpanSeverity, message: impl Into<String>) -> Self {
        Self {
            span,
            severity,
            message: message.into(),
        }
    }
    /// Create an error diagnostic.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Self::new(span, SpanSeverity::Error, message)
    }
    /// Create a warning diagnostic.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn warning(span: Span, message: impl Into<String>) -> Self {
        Self::new(span, SpanSeverity::Warning, message)
    }
    /// Create an info diagnostic.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn info(span: Span, message: impl Into<String>) -> Self {
        Self::new(span, SpanSeverity::Info, message)
    }
    /// Format as a short string for display.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn format_short(&self) -> String {
        format!(
            "[{}] {}: {}",
            self.severity.label(),
            span_short(&self.span),
            self.message
        )
    }
}
/// A span paired with its provenance information.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct ProvenanceSpan {
    pub span: Span,
    pub origin: SpanOrigin,
}
impl ProvenanceSpan {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(span: Span, origin: SpanOrigin) -> Self {
        Self { span, origin }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn user(span: Span) -> Self {
        Self::new(span, SpanOrigin::UserSource)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn synthetic() -> Self {
        Self::new(dummy_span(), SpanOrigin::Synthetic)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn macro_expanded(span: Span, macro_name: impl Into<String>) -> Self {
        Self::new(
            span,
            SpanOrigin::MacroExpanded {
                macro_name: macro_name.into(),
            },
        )
    }
}
/// A collection of annotated spans.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub struct SpanAnnotations {
    items: Vec<AnnotatedSpan>,
}
impl SpanAnnotations {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, a: AnnotatedSpan) {
        self.items.push(a);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn annotate(&mut self, span: Span, text: impl Into<String>) {
        self.items.push(AnnotatedSpan::new(span, text));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn at_offset(&self, offset: usize) -> Vec<&AnnotatedSpan> {
        self.items
            .iter()
            .filter(|a| span_contains(&a.span, offset))
            .collect()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_tag<'a>(&'a self, tag: &str) -> Vec<&'a AnnotatedSpan> {
        self.items
            .iter()
            .filter(|a| a.tag.as_deref() == Some(tag))
            .collect()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn sort_by_start(&mut self) {
        self.items.sort_by_key(|a| a.span.start);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn iter(&self) -> impl Iterator<Item = &AnnotatedSpan> {
        self.items.iter()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        self.items.clear();
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn merge(&mut self, other: SpanAnnotations) {
        self.items.extend(other.items);
    }
}
/// Incrementally build a `Span` from individual character positions.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct SpanBuilder {
    start: usize,
    line: usize,
    column: usize,
}
impl SpanBuilder {
    /// Begin a new span at `start` (byte offset, 1-indexed line/col).
    #[allow(missing_docs)]
    pub fn new(start: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            line,
            column,
        }
    }
    /// Finish the span at `end` (byte offset).
    #[allow(missing_docs)]
    pub fn finish(&self, end: usize) -> Span {
        Span::new(self.start, end, self.line, self.column)
    }
    /// Return the start byte offset.
    #[allow(missing_docs)]
    pub fn start(&self) -> usize {
        self.start
    }
    /// Return the start `SourcePos`.
    #[allow(missing_docs)]
    pub fn pos(&self) -> SourcePos {
        SourcePos::new(self.line, self.column)
    }
}
/// Convert a UTF-8 byte span to a UTF-16 code-unit span.
///
/// UTF-16 spans are used by LSP (Language Server Protocol).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct Utf16Span {
    /// Start UTF-16 code-unit index on the line.
    pub start_utf16: usize,
    /// End UTF-16 code-unit index on the line.
    pub end_utf16: usize,
    /// Line number (0-indexed for LSP).
    #[allow(missing_docs)]
    pub line: usize,
}
/// Statistics about a collection of spans.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub struct SpanStats {
    pub count: usize,
    pub total_len: usize,
    pub min_len: usize,
    pub max_len: usize,
}
impl SpanStats {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn compute(spans: &[Span]) -> Self {
        if spans.is_empty() {
            return Self::default();
        }
        let count = spans.len();
        let lengths: Vec<usize> = spans.iter().map(span_len).collect();
        let total_len: usize = lengths.iter().sum();
        let min_len = *lengths.iter().min().unwrap_or(&0);
        let max_len = *lengths.iter().max().unwrap_or(&0);
        Self {
            count,
            total_len,
            min_len,
            max_len,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn avg_len(&self) -> usize {
        if self.count == 0 {
            0
        } else {
            self.total_len / self.count
        }
    }
}
/// Identifies a source file by an integer ID.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FileId(pub u32);
impl FileId {
    /// The "unknown" / dummy file ID.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub const UNKNOWN: FileId = FileId(0);
    /// Create a new file ID.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(id: u32) -> Self {
        FileId(id)
    }
    /// Returns the raw integer ID.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn raw(self) -> u32 {
        self.0
    }
}
/// A linear chain of spans.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub struct SpanChain {
    spans: Vec<Span>,
}
impl SpanChain {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn push(&mut self, span: Span) {
        self.spans.push(span);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_span(&self) -> Option<Span> {
        merge_spans(&self.spans)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.spans.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.spans.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn iter(&self) -> impl Iterator<Item = &Span> {
        self.spans.iter()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn first(&self) -> Option<&Span> {
        self.spans.first()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn last(&self) -> Option<&Span> {
        self.spans.last()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        self.spans.clear();
    }
}
/// A registry that maps string keys to `Span` values.
///
/// Useful for tracking where named items were defined in a source file.
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct SpanRegistry {
    entries: std::collections::HashMap<String, Span>,
}
impl SpanRegistry {
    /// Create an empty registry.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
    /// Register a span for `key`.
    #[allow(missing_docs)]
    pub fn register(&mut self, key: impl Into<String>, span: Span) {
        self.entries.insert(key.into(), span);
    }
    /// Look up the span for `key`.
    #[allow(missing_docs)]
    pub fn get(&self, key: &str) -> Option<&Span> {
        self.entries.get(key)
    }
    /// `true` if `key` is registered.
    #[allow(missing_docs)]
    pub fn contains(&self, key: &str) -> bool {
        self.entries.contains_key(key)
    }
    /// Number of registered entries.
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// `true` if empty.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Iterate over all `(key, span)` pairs.
    #[allow(missing_docs)]
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Span)> {
        self.entries.iter()
    }
    /// Remove an entry, returning its span.
    #[allow(missing_docs)]
    pub fn remove(&mut self, key: &str) -> Option<Span> {
        self.entries.remove(key)
    }
    /// Merge another registry into this one (other wins on conflict).
    #[allow(missing_docs)]
    pub fn merge(&mut self, other: SpanRegistry) {
        for (k, v) in other.entries {
            self.entries.insert(k, v);
        }
    }
}
/// A span together with a priority level.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct PrioritizedSpan {
    pub span: Span,
    pub priority: u32,
}
impl PrioritizedSpan {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(span: Span, priority: u32) -> Self {
        Self { span, priority }
    }
}
/// A named source range: a label plus a span.
///
/// Used in diagnostics to annotate specific regions of source text.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct LabeledSpan {
    /// Human-readable label for this region.
    pub label: String,
    /// The source span.
    pub span: Span,
}
impl LabeledSpan {
    /// Construct a new labeled span.
    #[allow(missing_docs)]
    pub fn new(label: impl Into<String>, span: Span) -> Self {
        Self {
            label: label.into(),
            span,
        }
    }
    /// Return the length of the underlying span.
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        span_len(&self.span)
    }
    /// `true` if the span is zero-length.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// A cursor that tracks the current byte position, line, and column while
/// scanning a source string.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct SourceCursor<'a> {
    source: &'a str,
    pos: usize,
    line: usize,
    col: usize,
}
impl<'a> SourceCursor<'a> {
    /// Create a new cursor at the beginning of `source`.
    #[allow(missing_docs)]
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            pos: 0,
            line: 1,
            col: 1,
        }
    }
    /// Return the current byte position.
    #[allow(missing_docs)]
    pub fn pos(&self) -> usize {
        self.pos
    }
    /// Return the current `SourcePos` (1-indexed).
    #[allow(missing_docs)]
    pub fn source_pos(&self) -> SourcePos {
        SourcePos::new(self.line, self.col)
    }
    /// Return the current `Span` (zero-length, at the current position).
    #[allow(missing_docs)]
    pub fn current_span(&self) -> Span {
        Span::new(self.pos, self.pos, self.line, self.col)
    }
    /// `true` if the cursor is at the end of the source.
    #[allow(missing_docs)]
    pub fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }
    /// Peek at the next character without advancing.
    #[allow(missing_docs)]
    pub fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }
    /// Peek at the character `n` UTF-8 code points ahead.
    #[allow(missing_docs)]
    pub fn peek_ahead(&self, n: usize) -> Option<char> {
        self.source[self.pos..].chars().nth(n)
    }
    /// Advance by one character and return it.
    #[allow(missing_docs)]
    pub fn advance(&mut self) -> Option<char> {
        let ch = self.source[self.pos..].chars().next()?;
        self.pos += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(ch)
    }
    /// Advance while the predicate is true, returning the consumed substring.
    #[allow(missing_docs)]
    pub fn advance_while<F: Fn(char) -> bool>(&mut self, pred: F) -> &'a str {
        let start = self.pos;
        while let Some(ch) = self.peek() {
            if pred(ch) {
                self.advance();
            } else {
                break;
            }
        }
        &self.source[start..self.pos]
    }
    /// Consume exactly `n` bytes (assumes ASCII for simplicity).
    #[allow(missing_docs)]
    pub fn advance_bytes(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }
    /// Create a `Span` starting at `start_pos` and ending at the current position.
    #[allow(missing_docs)]
    pub fn span_from(&self, start: usize, start_line: usize, start_col: usize) -> Span {
        Span::new(start, self.pos, start_line, start_col)
    }
    /// Return the remaining (unconsumed) source text.
    #[allow(missing_docs)]
    pub fn rest(&self) -> &'a str {
        &self.source[self.pos..]
    }
    /// Return the consumed source text.
    #[allow(missing_docs)]
    pub fn consumed(&self) -> &'a str {
        &self.source[..self.pos]
    }
}
/// A collection of diagnostics associated with a source file.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub struct DiagnosticSet {
    diagnostics: Vec<DiagnosticSpan>,
}
impl DiagnosticSet {
    /// Create an empty set.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a diagnostic.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, d: DiagnosticSpan) {
        self.diagnostics.push(d);
    }
    /// Add an error.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_error(&mut self, span: Span, msg: impl Into<String>) {
        self.add(DiagnosticSpan::error(span, msg));
    }
    /// Add a warning.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_warning(&mut self, span: Span, msg: impl Into<String>) {
        self.add(DiagnosticSpan::warning(span, msg));
    }
    /// Add an info.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_info(&mut self, span: Span, msg: impl Into<String>) {
        self.add(DiagnosticSpan::info(span, msg));
    }
    /// Count diagnostics of a given severity.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count_severity(&self, sev: &SpanSeverity) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| &d.severity == sev)
            .count()
    }
    /// Total number of diagnostics.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }
    /// Check if empty.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
    /// Check if there are any errors.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity.is_error())
    }
    /// Get all errors.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn errors(&self) -> Vec<&DiagnosticSpan> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity.is_error())
            .collect()
    }
    /// Get all warnings.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn warnings(&self) -> Vec<&DiagnosticSpan> {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, SpanSeverity::Warning))
            .collect()
    }
    /// Sort diagnostics by span start offset.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn sort_by_position(&mut self) {
        self.diagnostics.sort_by_key(|d| d.span.start);
    }
    /// Iterate over all diagnostics.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn iter(&self) -> impl Iterator<Item = &DiagnosticSpan> {
        self.diagnostics.iter()
    }
    /// Clear all diagnostics.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }
    /// Merge another set into this one.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn merge(&mut self, other: DiagnosticSet) {
        self.diagnostics.extend(other.diagnostics);
    }
}
/// A span diff: describes changes between two spans.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct SpanDiff {
    /// The old span.
    pub old: Span,
    /// The new span.
    pub new: Span,
    /// The byte delta (positive = grew, negative = shrank).
    #[allow(missing_docs)]
    pub byte_delta: i64,
}
impl SpanDiff {
    /// Compute the diff between two spans.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn compute(old: Span, new: Span) -> Self {
        let byte_delta = new.end as i64 - old.end as i64;
        Self {
            old,
            new,
            byte_delta,
        }
    }
    /// Whether the span grew.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn grew(&self) -> bool {
        self.byte_delta > 0
    }
    /// Whether the span shrank.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn shrank(&self) -> bool {
        self.byte_delta < 0
    }
    /// Whether the span is unchanged.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn unchanged(&self) -> bool {
        self.byte_delta == 0
    }
}
/// Tracks how a set of spans evolve as edits are applied.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub struct IncrementalSpanTracker {
    spans: Vec<Span>,
    edit_count: usize,
}
impl IncrementalSpanTracker {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn track(&mut self, span: Span) {
        self.spans.push(span);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn apply_edit(&mut self, edit_start: usize, delta: i64) {
        shift_spans(&mut self.spans, edit_start, delta);
        self.edit_count += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn spans(&self) -> &[Span] {
        &self.spans
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn edit_count(&self) -> usize {
        self.edit_count
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn reset(&mut self) {
        self.spans.clear();
        self.edit_count = 0;
    }
}
/// A value paired with its source `Span`.
#[derive(Clone, Debug, PartialEq)]
#[allow(missing_docs)]
pub struct Spanned<T> {
    /// The value.
    pub value: T,
    /// The source span.
    pub span: Span,
}
impl<T> Spanned<T> {
    /// Wrap `value` at `span`.
    #[allow(missing_docs)]
    pub fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }
    /// Map over the value, keeping the span.
    #[allow(missing_docs)]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Spanned<U> {
        Spanned {
            value: f(self.value),
            span: self.span,
        }
    }
    /// Borrow the inner value.
    #[allow(clippy::should_implement_trait)]
    #[allow(missing_docs)]
    pub fn as_ref(&self) -> &T {
        &self.value
    }
    /// Consume and return the value, discarding the span.
    #[allow(missing_docs)]
    pub fn into_value(self) -> T {
        self.value
    }
}
/// The origin of a span.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub enum SpanOrigin {
    UserSource,
    MacroExpanded { macro_name: String },
    Elaborated,
    Synthetic,
}
impl SpanOrigin {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_user_source(&self) -> bool {
        matches!(self, SpanOrigin::UserSource)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_synthetic(&self) -> bool {
        matches!(self, SpanOrigin::Synthetic)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn kind_str(&self) -> &'static str {
        match self {
            SpanOrigin::UserSource => "user",
            SpanOrigin::MacroExpanded { .. } => "macro",
            SpanOrigin::Elaborated => "elab",
            SpanOrigin::Synthetic => "synthetic",
        }
    }
}
/// A span paired with a severity level, for diagnostics.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub enum SpanSeverity {
    /// Informational annotation.
    Info,
    /// A warning annotation.
    Warning,
    /// An error annotation.
    Error,
}
impl SpanSeverity {
    /// Returns `true` if this is an error severity.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_error(&self) -> bool {
        matches!(self, SpanSeverity::Error)
    }
    /// Short label string for this severity.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn label(&self) -> &'static str {
        match self {
            SpanSeverity::Info => "info",
            SpanSeverity::Warning => "warning",
            SpanSeverity::Error => "error",
        }
    }
}
/// A span paired with annotation text and optional tag.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct AnnotatedSpan {
    pub span: Span,
    pub annotation: String,
    pub tag: Option<String>,
}
impl AnnotatedSpan {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(span: Span, annotation: impl Into<String>) -> Self {
        Self {
            span,
            annotation: annotation.into(),
            tag: None,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_tag(span: Span, annotation: impl Into<String>, tag: impl Into<String>) -> Self {
        Self {
            span,
            annotation: annotation.into(),
            tag: Some(tag.into()),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        span_len(&self.span)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// A half-open interval \[start, end) over span indices (for range operations).
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpanRange {
    pub start_idx: usize,
    pub end_idx: usize,
}
impl SpanRange {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(start_idx: usize, end_idx: usize) -> Self {
        Self { start_idx, end_idx }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.end_idx.saturating_sub(self.start_idx)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.start_idx >= self.end_idx
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn contains(&self, idx: usize) -> bool {
        idx >= self.start_idx && idx < self.end_idx
    }
}
/// A span with a byte-level padding on each side.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct PaddedSpan {
    pub inner: Span,
    pub left_pad: usize,
    pub right_pad: usize,
}
impl PaddedSpan {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(inner: Span, left_pad: usize, right_pad: usize) -> Self {
        Self {
            inner,
            left_pad,
            right_pad,
        }
    }
    /// Expand the inner span by the padding amounts, clamped to source bounds.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn expanded(&self, source_len: usize) -> Span {
        let start = self.inner.start.saturating_sub(self.left_pad);
        let end = (self.inner.end + self.right_pad).min(source_len);
        Span::new(start, end, self.inner.line, self.inner.column)
    }
}
/// A flat map from span start offsets to values.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub struct SpanMap<V> {
    entries: Vec<(usize, V)>,
}
impl<V> SpanMap<V> {
    /// Create an empty map.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Insert a value at `offset`.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, offset: usize, value: V) {
        self.entries.push((offset, value));
    }
    /// Look up the value at `offset`.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get(&self, offset: usize) -> Option<&V> {
        self.entries
            .iter()
            .find(|(o, _)| *o == offset)
            .map(|(_, v)| v)
    }
    /// Number of entries.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if empty.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Iterate over all entries.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn iter(&self) -> impl Iterator<Item = &(usize, V)> {
        self.entries.iter()
    }
}
/// A span that also carries a `FileId`, enabling multi-file diagnostics.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq)]
pub struct FileSpan {
    /// Which file this span belongs to.
    pub file: FileId,
    /// The span within that file.
    pub span: Span,
}
impl FileSpan {
    /// Create a new file span.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(file: FileId, span: Span) -> Self {
        Self { file, span }
    }
    /// Return the length in bytes.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        span_len(&self.span)
    }
    /// `true` if this is a zero-length span.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Merge two `FileSpan`s (panics if they come from different files).
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn merge_with(&self, other: &FileSpan) -> FileSpan {
        assert_eq!(
            self.file, other.file,
            "cannot merge spans from different files"
        );
        FileSpan {
            file: self.file,
            span: self.span.merge(&other.span),
        }
    }
}
/// A line/column position in source text (1-indexed).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(missing_docs)]
pub struct SourcePos {
    /// 1-indexed line number.
    pub line: usize,
    /// 1-indexed column number (byte offset within the line).
    pub col: usize,
}
impl SourcePos {
    /// Construct a new `SourcePos`.
    #[allow(missing_docs)]
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
    /// The "beginning of file" position.
    #[allow(missing_docs)]
    pub fn start() -> Self {
        Self { line: 1, col: 1 }
    }
    /// Advance by one ASCII character (no newline).
    #[allow(missing_docs)]
    pub fn advance_col(&self) -> Self {
        Self {
            line: self.line,
            col: self.col + 1,
        }
    }
    /// Advance to the next line (column resets to 1).
    #[allow(missing_docs)]
    pub fn advance_line(&self) -> Self {
        Self {
            line: self.line + 1,
            col: 1,
        }
    }
    /// `true` if `other` is on the same line as `self`.
    #[allow(missing_docs)]
    pub fn same_line(&self, other: &SourcePos) -> bool {
        self.line == other.line
    }
}
