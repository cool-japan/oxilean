//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::error_impl::{ParseError, ParseErrorKind};

use super::functions::*;

/// Represents a user-facing error explanation page.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorExplanation {
    pub code: String,
    pub title: String,
    pub description: String,
    pub example_bad: String,
    #[allow(missing_docs)]
    pub example_good: String,
}
impl ErrorExplanation {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(
        code: impl Into<String>,
        title: impl Into<String>,
        desc: impl Into<String>,
        bad: impl Into<String>,
        good: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            title: title.into(),
            description: desc.into(),
            example_bad: bad.into(),
            example_good: good.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn render(&self) -> String {
        format!(
            "[{}] {}\n\n{}\n\nBad:\n{}\n\nGood:\n{}",
            self.code, self.title, self.description, self.example_bad, self.example_good
        )
    }
}
/// Error location resolver: maps span to line/column.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorLocationResolver {
    source: String,
    line_starts: Vec<usize>,
}
impl ErrorLocationResolver {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(source: impl Into<String>) -> Self {
        let s = source.into();
        let mut starts = vec![0usize];
        for (i, c) in s.char_indices() {
            if c == '\n' {
                starts.push(i + 1);
            }
        }
        Self {
            source: s,
            line_starts: starts,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn resolve(&self, byte_offset: usize) -> (usize, usize) {
        let off = byte_offset.min(self.source.len());
        let line = self
            .line_starts
            .partition_point(|&s| s <= off)
            .saturating_sub(1);
        let col = off - self.line_starts[line];
        (line, col)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn line_text(&self, line: usize) -> &str {
        let start = *self.line_starts.get(line).unwrap_or(&self.source.len());
        let end = *self.line_starts.get(line + 1).unwrap_or(&self.source.len());
        let end = if end > start && self.source.as_bytes().get(end - 1) == Some(&b'\n') {
            end - 1
        } else {
            end
        };
        &self.source[start..end]
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn snippet(&self, byte_offset: usize, context_lines: usize) -> String {
        let (line, col) = self.resolve(byte_offset);
        let start_line = line.saturating_sub(context_lines);
        let end_line = (line + context_lines).min(self.line_count().saturating_sub(1));
        let mut out = String::new();
        for l in start_line..=end_line {
            let text = self.line_text(l);
            out.push_str(&format!("{:4} | {}\n", l + 1, text));
            if l == line {
                out.push_str(&format!("     | {}^\n", " ".repeat(col)));
            }
        }
        out
    }
}
/// An error sink that writes errors to a string buffer.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct StringErrorSink {
    buffer: String,
    count: usize,
}
impl StringErrorSink {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            count: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn emit(&mut self, e: &RichError) {
        if !self.buffer.is_empty() {
            self.buffer.push('\n');
        }
        self.buffer.push_str(&e.format());
        self.count += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn contents(&self) -> &str {
        &self.buffer
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.count
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        self.buffer.clear();
        self.count = 0;
    }
}
/// Attaches contextual information to a `ParseError`.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct ParseErrorContext {
    /// The underlying error.
    pub error: ParseError,
    /// The name of the declaration being parsed when the error occurred.
    pub decl_name: Option<String>,
    /// The parser phase in which the error occurred.
    #[allow(missing_docs)]
    pub phase: Option<String>,
}
impl ParseErrorContext {
    /// Create a context wrapping an error.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(error: ParseError) -> Self {
        Self {
            error,
            decl_name: None,
            phase: None,
        }
    }
    /// Attach a declaration name.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_decl(mut self, name: &str) -> Self {
        self.decl_name = Some(name.to_string());
        self
    }
    /// Attach a parser phase.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_phase(mut self, phase: &str) -> Self {
        self.phase = Some(phase.to_string());
        self
    }
}
/// Error severity enum for extended error handling.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverityLevel {
    Hint,
    Note,
    Warning,
    Error,
    Fatal,
}
/// The severity of a parse diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum ErrorSeverity {
    /// Informational note.
    Note,
    /// Non-fatal warning.
    Warning,
    /// Fatal error — parsing cannot produce a valid result.
    Error,
}
impl ErrorSeverity {
    /// Whether this severity represents a hard error.
    #[allow(missing_docs)]
    pub fn is_error(&self) -> bool {
        matches!(self, ErrorSeverity::Error)
    }
    /// Whether parsing can continue after this diagnostic.
    #[allow(missing_docs)]
    pub fn is_recoverable(&self) -> bool {
        !self.is_error()
    }
}
/// Accumulates multiple parse errors before reporting them all at once.
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct ParseErrorCollector {
    /// Collected errors in order.
    errors: Vec<ParseError>,
    /// Maximum number of errors to store (0 = unlimited).
    limit: usize,
}
impl ParseErrorCollector {
    /// Create an unlimited collector.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a collector with a maximum error count.
    #[allow(missing_docs)]
    pub fn with_limit(limit: usize) -> Self {
        Self {
            errors: Vec::new(),
            limit,
        }
    }
    /// Add an error.
    #[allow(missing_docs)]
    pub fn add(&mut self, error: ParseError) {
        if self.limit == 0 || self.errors.len() < self.limit {
            self.errors.push(error);
        }
    }
    /// Whether any errors have been collected.
    #[allow(missing_docs)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    /// Number of errors collected.
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.errors.len()
    }
    /// Whether the collector is empty.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    /// Consume the collector and return the error list.
    #[allow(missing_docs)]
    pub fn into_errors(self) -> Vec<ParseError> {
        self.errors
    }
    /// Return a reference to all errors.
    #[allow(missing_docs)]
    pub fn errors(&self) -> &[ParseError] {
        &self.errors
    }
    /// Take the first error, if any.
    #[allow(missing_docs)]
    pub fn first_error(&self) -> Option<&ParseError> {
        self.errors.first()
    }
    /// Clear all collected errors.
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        self.errors.clear();
    }
    /// Whether the error limit has been reached.
    #[allow(missing_docs)]
    pub fn is_full(&self) -> bool {
        self.limit > 0 && self.errors.len() >= self.limit
    }
    /// Merge another collector's errors into this one.
    #[allow(missing_docs)]
    pub fn merge(&mut self, other: ParseErrorCollector) {
        for e in other.errors {
            self.add(e);
        }
    }
}
/// Batch error report with summary statistics.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BatchErrorReport {
    pub errors: Vec<RichError>,
    pub source_file: String,
    pub parse_time_us: u64,
}
impl BatchErrorReport {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(source_file: impl Into<String>, errors: Vec<RichError>, parse_time_us: u64) -> Self {
        Self {
            errors,
            source_file: source_file.into(),
            parse_time_us,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn error_count(&self) -> usize {
        self.errors
            .iter()
            .filter(|e| e.severity >= ErrorSeverityLevel::Error)
            .count()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn warning_count(&self) -> usize {
        self.errors
            .iter()
            .filter(|e| e.severity == ErrorSeverityLevel::Warning)
            .count()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_success(&self) -> bool {
        self.error_count() == 0
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn summary_line(&self) -> String {
        format!(
            "{}: {} error(s), {} warning(s) in {}us",
            self.source_file,
            self.error_count(),
            self.warning_count(),
            self.parse_time_us
        )
    }
}
/// An error grouper: clusters errors by their error code.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorGrouper {
    groups: std::collections::HashMap<String, Vec<RichError>>,
}
impl ErrorGrouper {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            groups: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, e: RichError) {
        let key = e.code.clone().unwrap_or_else(|| "no-code".to_string());
        self.groups.entry(key).or_default().push(e);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn group_count(&self) -> usize {
        self.groups.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn errors_in_group(&self, code: &str) -> &[RichError] {
        self.groups.get(code).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn most_common_code(&self) -> Option<&str> {
        self.groups
            .iter()
            .max_by_key(|(_, v)| v.len())
            .map(|(k, _)| k.as_str())
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_error_count(&self) -> usize {
        self.groups.values().map(|v| v.len()).sum()
    }
}
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecoveryHintKind {
    InsertBefore,
    InsertAfter,
    Delete,
    Replace,
    Reorder,
}
/// Tracks error rates over time for adaptive error handling.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Default, Debug)]
pub struct ErrorRateTracker {
    window_size: usize,
    counts: std::collections::VecDeque<usize>,
    current: usize,
}
impl ErrorRateTracker {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(window_size: usize) -> Self {
        Self {
            window_size,
            counts: std::collections::VecDeque::new(),
            current: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record(&mut self, errors: usize) {
        self.current += errors;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn commit_window(&mut self) {
        self.counts.push_back(self.current);
        if self.counts.len() > self.window_size {
            self.counts.pop_front();
        }
        self.current = 0;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn average(&self) -> f64 {
        if self.counts.is_empty() {
            return 0.0;
        }
        self.counts.iter().sum::<usize>() as f64 / self.counts.len() as f64
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn trend(&self) -> f64 {
        if self.counts.len() < 2 {
            return 0.0;
        }
        let first = *self
            .counts
            .front()
            .expect("counts.len() >= 2 per check above") as f64;
        let last = *self
            .counts
            .back()
            .expect("counts.len() >= 2 per check above") as f64;
        last - first
    }
}
/// A non-fatal parse warning.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct ParseWarning {
    /// Warning message.
    pub message: String,
    /// Source line.
    pub line: u32,
    /// Source column.
    #[allow(missing_docs)]
    pub col: u32,
}
impl ParseWarning {
    /// Create a new warning.
    #[allow(missing_docs)]
    pub fn new(msg: &str, line: u32, col: u32) -> Self {
        Self {
            message: msg.to_string(),
            line,
            col,
        }
    }
}
/// A rich parse diagnostic combining severity, location, and message.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct ParseDiagnostic {
    /// Severity of this diagnostic.
    pub severity: ErrorSeverity,
    /// Source file name.
    pub filename: String,
    /// 1-based line number.
    #[allow(missing_docs)]
    pub line: u32,
    /// 1-based column number.
    pub col: u32,
    /// The diagnostic message.
    pub message: String,
    /// Optional hint for fixing the issue.
    #[allow(missing_docs)]
    pub hint: Option<String>,
    /// Optional code fragment illustrating the issue.
    pub code: Option<String>,
}
impl ParseDiagnostic {
    /// Create a new diagnostic.
    #[allow(missing_docs)]
    pub fn new(
        severity: ErrorSeverity,
        filename: &str,
        line: u32,
        col: u32,
        message: &str,
    ) -> Self {
        Self {
            severity,
            filename: filename.to_string(),
            line,
            col,
            message: message.to_string(),
            hint: None,
            code: None,
        }
    }
    /// Create an error diagnostic.
    #[allow(missing_docs)]
    pub fn error(filename: &str, line: u32, col: u32, msg: &str) -> Self {
        Self::new(ErrorSeverity::Error, filename, line, col, msg)
    }
    /// Create a warning diagnostic.
    #[allow(missing_docs)]
    pub fn warning(filename: &str, line: u32, col: u32, msg: &str) -> Self {
        Self::new(ErrorSeverity::Warning, filename, line, col, msg)
    }
    /// Attach a hint.
    #[allow(missing_docs)]
    pub fn with_hint(mut self, hint: &str) -> Self {
        self.hint = Some(hint.to_string());
        self
    }
    /// Attach a code snippet.
    #[allow(missing_docs)]
    pub fn with_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }
    /// Whether this is a hard error.
    #[allow(missing_docs)]
    pub fn is_error(&self) -> bool {
        self.severity.is_error()
    }
}
/// A collection of error explanations.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorExplanationBook {
    pages: std::collections::HashMap<String, ErrorExplanation>,
}
impl ErrorExplanationBook {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            pages: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, page: ErrorExplanation) {
        self.pages.insert(page.code.clone(), page);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&self, code: &str) -> Option<&ErrorExplanation> {
        self.pages.get(code)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.pages.len()
    }
}
/// A named group of related parse errors.
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct ParseErrorGroup {
    /// Group label.
    pub label: String,
    /// Errors in this group.
    pub errors: Vec<ParseError>,
}
impl ParseErrorGroup {
    /// Create a new group.
    #[allow(missing_docs)]
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            errors: Vec::new(),
        }
    }
    /// Add an error.
    #[allow(missing_docs)]
    pub fn add(&mut self, e: ParseError) {
        self.errors.push(e);
    }
    /// Number of errors.
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.errors.len()
    }
    /// Whether the group is empty.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
}
/// A recovery hint associated with an error.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct RecoveryHint {
    pub kind: RecoveryHintKind,
    pub text: String,
}
impl RecoveryHint {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert_before(text: impl Into<String>) -> Self {
        Self {
            kind: RecoveryHintKind::InsertBefore,
            text: text.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn delete(text: impl Into<String>) -> Self {
        Self {
            kind: RecoveryHintKind::Delete,
            text: text.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn replace(text: impl Into<String>) -> Self {
        Self {
            kind: RecoveryHintKind::Replace,
            text: text.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn description(&self) -> String {
        match self.kind {
            RecoveryHintKind::InsertBefore => {
                format!("insert '{}' before this token", self.text)
            }
            RecoveryHintKind::InsertAfter => {
                format!("insert '{}' after this token", self.text)
            }
            RecoveryHintKind::Delete => format!("delete '{}'", self.text),
            RecoveryHintKind::Replace => format!("replace with '{}'", self.text),
            RecoveryHintKind::Reorder => format!("reorder: {}", self.text),
        }
    }
}
/// Extended rich error with tags and recovery hints.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct TaggedError {
    pub inner: RichError,
    pub tags: Vec<ErrorTag>,
    pub hints: Vec<RecoveryHint>,
}
impl TaggedError {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(e: RichError) -> Self {
        Self {
            inner: e,
            tags: Vec::new(),
            hints: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_tag(mut self, tag: ErrorTag) -> Self {
        self.tags.push(tag);
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_hint(mut self, hint: RecoveryHint) -> Self {
        self.hints.push(hint);
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_tag(&self, tag: ErrorTag) -> bool {
        self.tags.contains(&tag)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn format_full(&self) -> String {
        let mut out = self.inner.format();
        for hint in &self.hints {
            out.push_str(&format!("\n  help: {}", hint.description()));
        }
        out
    }
}
/// Error tagging for categorisation.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ErrorTag {
    Syntax,
    Type,
    Name,
    Import,
    Layout,
    Unicode,
    Overflow,
    Internal,
}
/// An error filter: suppresses errors matching certain patterns.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorFilter {
    suppressed_codes: std::collections::HashSet<String>,
    min_severity: ErrorSeverityLevel,
}
impl ErrorFilter {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(min_severity: ErrorSeverityLevel) -> Self {
        Self {
            suppressed_codes: std::collections::HashSet::new(),
            min_severity,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn suppress_code(&mut self, code: impl Into<String>) {
        self.suppressed_codes.insert(code.into());
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn should_show(&self, e: &RichError) -> bool {
        if e.severity < self.min_severity {
            return false;
        }
        if let Some(code) = &e.code {
            if self.suppressed_codes.contains(code.as_str()) {
                return false;
            }
        }
        true
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn filter<'a>(&self, errors: &'a [RichError]) -> Vec<&'a RichError> {
        errors.iter().filter(|e| self.should_show(e)).collect()
    }
}
/// Controls how the parser recovers from errors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum RecoveryStrategy {
    /// Abort parsing immediately on the first error.
    Abort,
    /// Skip tokens until a synchronization point (e.g., `def`, `theorem`).
    SkipToSync,
    /// Insert a synthetic token and continue.
    InsertToken,
    /// Replace the offending token with a placeholder and continue.
    Replace,
}
impl RecoveryStrategy {
    /// Whether this strategy allows parsing to continue.
    #[allow(missing_docs)]
    pub fn continues(&self) -> bool {
        !matches!(self, RecoveryStrategy::Abort)
    }
}
/// Aggregate statistics about parse errors.
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct ParseErrorStats {
    /// Total number of errors.
    pub total: u64,
    /// Errors at line 0 (synthetic/EOF errors).
    pub eof_errors: u64,
    /// Errors with known location.
    #[allow(missing_docs)]
    pub located_errors: u64,
}
impl ParseErrorStats {
    /// Create zero-initialized stats.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record an error.
    #[allow(missing_docs)]
    pub fn record(&mut self, err: &ParseError) {
        self.total += 1;
        if err.span.line == 0 {
            self.eof_errors += 1;
        } else {
            self.located_errors += 1;
        }
    }
    /// Whether any errors were recorded.
    #[allow(missing_docs)]
    pub fn has_errors(&self) -> bool {
        self.total > 0
    }
}
/// An error budget: stop reporting after too many errors.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorBudget {
    budget: usize,
    spent: usize,
}
impl ErrorBudget {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(budget: usize) -> Self {
        Self { budget, spent: 0 }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn spend(&mut self) -> bool {
        if self.spent < self.budget {
            self.spent += 1;
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn remaining(&self) -> usize {
        self.budget.saturating_sub(self.spent)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_exhausted(&self) -> bool {
        self.spent >= self.budget
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn fraction_used(&self) -> f64 {
        if self.budget == 0 {
            1.0
        } else {
            self.spent as f64 / self.budget as f64
        }
    }
}
/// Accumulates errors and warnings over a parse session.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorAccumulator2 {
    errors: Vec<RichError>,
    max_errors: usize,
    fatal_encountered: bool,
}
impl ErrorAccumulator2 {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(max_errors: usize) -> Self {
        Self {
            errors: Vec::new(),
            max_errors,
            fatal_encountered: false,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, e: RichError) -> bool {
        if self.errors.len() >= self.max_errors {
            return false;
        }
        if e.is_fatal() {
            self.fatal_encountered = true;
        }
        self.errors.push(e);
        true
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn error_count(&self) -> usize {
        self.errors
            .iter()
            .filter(|e| e.severity >= ErrorSeverityLevel::Error)
            .count()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn warning_count(&self) -> usize {
        self.errors
            .iter()
            .filter(|e| e.severity == ErrorSeverityLevel::Warning)
            .count()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_fatal(&self) -> bool {
        self.fatal_encountered
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn all_errors(&self) -> &[RichError] {
        &self.errors
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn format_all(&self) -> String {
        self.errors
            .iter()
            .map(|e| e.format())
            .collect::<Vec<_>>()
            .join("\n")
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn sorted_by_severity(&self) -> Vec<&RichError> {
        let mut sorted: Vec<_> = self.errors.iter().collect();
        sorted.sort_by(|a, b| b.severity.cmp(&a.severity));
        sorted
    }
}
/// A rich error message with optional suggestions.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct RichError {
    pub message: String,
    pub severity: ErrorSeverityLevel,
    pub code: Option<String>,
    pub suggestions: Vec<String>,
    #[allow(missing_docs)]
    pub notes: Vec<String>,
    pub span_start: usize,
    pub span_end: usize,
}
impl RichError {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn error(msg: impl Into<String>, start: usize, end: usize) -> Self {
        Self {
            message: msg.into(),
            severity: ErrorSeverityLevel::Error,
            code: None,
            suggestions: Vec::new(),
            notes: Vec::new(),
            span_start: start,
            span_end: end,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn warning(msg: impl Into<String>, start: usize, end: usize) -> Self {
        Self {
            severity: ErrorSeverityLevel::Warning,
            ..Self::error(msg, start, end)
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_suggestion(mut self, s: impl Into<String>) -> Self {
        self.suggestions.push(s.into());
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_note(mut self, n: impl Into<String>) -> Self {
        self.notes.push(n.into());
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_fatal(&self) -> bool {
        self.severity >= ErrorSeverityLevel::Fatal
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn span_len(&self) -> usize {
        self.span_end.saturating_sub(self.span_start)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn format(&self) -> String {
        let mut out = format!("[{}] {}", self.severity, self.message);
        if let Some(code) = &self.code {
            out = format!("[{}][{}] {}", code, self.severity, self.message);
        }
        for sug in &self.suggestions {
            out.push_str(&format!("\n  suggestion: {}", sug));
        }
        for note in &self.notes {
            out.push_str(&format!("\n  note: {}", note));
        }
        out
    }
}
/// Renders parse errors with surrounding source context.
#[allow(missing_docs)]
pub struct ParseErrorFormatter<'a> {
    /// Source text.
    pub src: &'a str,
    /// Source file name.
    pub filename: &'a str,
    /// Number of context lines above/below the error.
    #[allow(missing_docs)]
    pub context_lines: usize,
}
impl<'a> ParseErrorFormatter<'a> {
    /// Create a formatter with default context (2 lines).
    #[allow(missing_docs)]
    pub fn new(src: &'a str, filename: &'a str) -> Self {
        Self {
            src,
            filename,
            context_lines: 2,
        }
    }
    /// Set the number of context lines.
    #[allow(missing_docs)]
    pub fn with_context(mut self, n: usize) -> Self {
        self.context_lines = n;
        self
    }
    /// Format a `ParseError` into a human-readable string.
    #[allow(missing_docs)]
    pub fn format(&self, err: &ParseError) -> String {
        let line = err.span.line;
        let col = err.span.column;
        let lines: Vec<&str> = self.src.lines().collect();
        let mut out = format!(
            "{}:{}:{}: error: {:?}\n",
            self.filename, line, col, err.kind
        );
        if line > 0 && line <= lines.len() {
            let start = line.saturating_sub(1 + self.context_lines);
            let end = (line + self.context_lines).min(lines.len());
            for (i, ln) in lines[start..end].iter().enumerate() {
                let lineno = start + i + 1;
                out.push_str(&format!("{:>4} | {}\n", lineno, ln));
                if lineno == line {
                    let spaces = " ".repeat(col.saturating_sub(1));
                    out.push_str(&format!("     | {}^\n", spaces));
                }
            }
        }
        out
    }
    /// Format all errors in a collector.
    #[allow(missing_docs)]
    pub fn format_all(&self, collector: &ParseErrorCollector) -> String {
        collector
            .errors()
            .iter()
            .map(|e| self.format(e))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// An aggregated error report combining errors, warnings, and notes.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, Default)]
pub struct ParseErrorReport {
    /// File name.
    pub filename: String,
    /// All diagnostics.
    pub diagnostics: Vec<ParseDiagnostic>,
}
impl ParseErrorReport {
    /// Create an empty report for a file.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            diagnostics: Vec::new(),
        }
    }
    /// Add a diagnostic.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, d: ParseDiagnostic) {
        self.diagnostics.push(d);
    }
    /// Number of hard errors.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn error_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.is_error()).count()
    }
    /// Number of warnings.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == ErrorSeverity::Warning)
            .count()
    }
    /// Whether the report contains no errors.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_clean(&self) -> bool {
        self.error_count() == 0
    }
    /// Return all errors.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn errors(&self) -> Vec<&ParseDiagnostic> {
        errors_only(&self.diagnostics)
    }
    /// Return all warnings.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn warnings(&self) -> Vec<&ParseDiagnostic> {
        warnings_only(&self.diagnostics)
    }
}
/// Error chain: a sequence of related errors.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorChain {
    chain: Vec<RichError>,
}
impl ErrorChain {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(root: RichError) -> Self {
        Self { chain: vec![root] }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn caused_by(mut self, e: RichError) -> Self {
        self.chain.push(e);
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn root(&self) -> &RichError {
        &self.chain[0]
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.chain.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn format_chain(&self) -> String {
        self.chain
            .iter()
            .enumerate()
            .map(|(i, e)| {
                if i == 0 {
                    e.format()
                } else {
                    format!("  caused by: {}", e.message)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// Tracks parse errors against a budget; aborts when exceeded.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct ParseErrorBudget {
    /// Total budget.
    pub budget: usize,
    /// Remaining budget.
    pub remaining: usize,
}
impl ParseErrorBudget {
    /// Create a budget.
    #[allow(missing_docs)]
    pub fn new(budget: usize) -> Self {
        Self {
            budget,
            remaining: budget,
        }
    }
    /// Consume one error token. Returns `true` if budget remains.
    #[allow(missing_docs)]
    pub fn consume(&mut self) -> bool {
        if self.remaining > 0 {
            self.remaining -= 1;
            true
        } else {
            false
        }
    }
    /// Whether the budget has been exhausted.
    #[allow(missing_docs)]
    pub fn is_exhausted(&self) -> bool {
        self.remaining == 0
    }
    /// Number of errors consumed.
    #[allow(missing_docs)]
    pub fn consumed(&self) -> usize {
        self.budget - self.remaining
    }
    /// Reset the budget to its initial value.
    #[allow(missing_docs)]
    pub fn reset(&mut self) {
        self.remaining = self.budget;
    }
}
/// Error code catalogue with descriptions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorCodeCatalogue {
    entries: std::collections::HashMap<String, String>,
}
impl ErrorCodeCatalogue {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let mut cat = Self {
            entries: std::collections::HashMap::new(),
        };
        cat.add("E0001", "unexpected token");
        cat.add("E0002", "missing closing bracket");
        cat.add("E0003", "expected identifier");
        cat.add("E0004", "invalid escape sequence");
        cat.add("E0005", "unterminated string literal");
        cat.add("E0006", "unexpected end of file");
        cat.add("E0007", "integer literal too large");
        cat.add("E0008", "invalid character");
        cat.add("E0009", "indentation error");
        cat.add("E0010", "ambiguous parse");
        cat
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, code: impl Into<String>, desc: impl Into<String>) {
        self.entries.insert(code.into(), desc.into());
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn description(&self, code: &str) -> Option<&str> {
        self.entries.get(code).map(|s| s.as_str())
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}
/// Maps error codes to their suggested quick-fix actions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct QuickFixRegistry {
    fixes: std::collections::HashMap<String, Vec<String>>,
}
impl QuickFixRegistry {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            fixes: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn register(&mut self, code: impl Into<String>, fix: impl Into<String>) {
        self.fixes.entry(code.into()).or_default().push(fix.into());
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn fixes_for(&self, code: &str) -> &[String] {
        self.fixes.get(code).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_fixes(&self, code: &str) -> bool {
        !self.fixes_for(code).is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_codes(&self) -> usize {
        self.fixes.len()
    }
}
/// Error deduplication: suppress repeated identical messages.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorDeduplicator {
    seen: std::collections::HashSet<String>,
    suppressed: usize,
}
impl ErrorDeduplicator {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            seen: std::collections::HashSet::new(),
            suppressed: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn should_emit(&mut self, key: &str) -> bool {
        if self.seen.contains(key) {
            self.suppressed += 1;
            false
        } else {
            self.seen.insert(key.to_string());
            true
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn suppressed_count(&self) -> usize {
        self.suppressed
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn unique_count(&self) -> usize {
        self.seen.len()
    }
}
/// An error with its source context pre-rendered.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ContextualRichError {
    pub error: RichError,
    pub context_snippet: String,
    pub file: String,
    pub line: usize,
    #[allow(missing_docs)]
    pub column: usize,
}
impl ContextualRichError {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(error: RichError, source: &str, file: impl Into<String>) -> Self {
        let resolver = ErrorLocationResolver::new(source);
        let (line, col) = resolver.resolve(error.span_start);
        let snippet = resolver.snippet(error.span_start, 1);
        Self {
            error,
            context_snippet: snippet,
            file: file.into(),
            line,
            column: col,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn format_full(&self) -> String {
        format!(
            "{}:{}:{}: {}\n{}",
            self.file,
            self.line + 1,
            self.column + 1,
            self.error.format(),
            self.context_snippet
        )
    }
}
