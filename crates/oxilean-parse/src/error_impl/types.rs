//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tokens::{Span, TokenKind};

/// An error chain that wraps a root cause with a context message.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ErrorChainExt {
    /// Context message
    pub context: String,
    /// Root cause message
    pub cause: String,
}
impl ErrorChainExt {
    /// Create a new error chain.
    #[allow(dead_code)]
    pub fn new(context: &str, cause: &str) -> Self {
        ErrorChainExt {
            context: context.to_string(),
            cause: cause.to_string(),
        }
    }
    /// Format as "context: cause".
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!("{}: {}", self.context, self.cause)
    }
}
/// An error sink that collects errors for later processing.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct ErrorSink {
    /// Collected errors
    pub errors: Vec<LocatedError>,
    /// Whether to stop at first error
    pub stop_at_first: bool,
}
impl ErrorSink {
    /// Create a new ErrorSink.
    #[allow(dead_code)]
    pub fn new() -> Self {
        ErrorSink {
            errors: Vec::new(),
            stop_at_first: false,
        }
    }
    /// Create an ErrorSink that stops at first error.
    #[allow(dead_code)]
    pub fn fail_fast() -> Self {
        ErrorSink {
            errors: Vec::new(),
            stop_at_first: true,
        }
    }
    /// Add an error to the sink.
    #[allow(dead_code)]
    pub fn push(&mut self, err: LocatedError) {
        self.errors.push(err);
    }
    /// Returns whether the sink has any errors.
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    /// Returns the number of errors.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.errors.len()
    }
    /// Returns true if the sink is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    /// Clear all errors.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.errors.clear();
    }
}
/// An annotated span in a source string.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct AnnotatedSpan {
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
    /// Annotation text
    pub label: String,
}
impl AnnotatedSpan {
    /// Create a new AnnotatedSpan.
    #[allow(dead_code)]
    pub fn new(start: usize, end: usize, label: &str) -> Self {
        AnnotatedSpan {
            start,
            end,
            label: label.to_string(),
        }
    }
}
/// An error recovery suggestion.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct RecoverySuggestion {
    /// Human-readable description of the fix
    pub description: String,
    /// The replacement text (if applicable)
    pub replacement: Option<String>,
    /// Confidence score [0.0, 1.0]
    pub confidence: f64,
}
impl RecoverySuggestion {
    /// Create a new suggestion.
    #[allow(dead_code)]
    pub fn new(description: &str, replacement: Option<&str>, confidence: f64) -> Self {
        RecoverySuggestion {
            description: description.to_string(),
            replacement: replacement.map(|s| s.to_string()),
            confidence,
        }
    }
}
/// An error batch processor that groups errors by code.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorBatch {
    /// Errors grouped by code
    pub by_code: std::collections::HashMap<u32, Vec<SpannedError>>,
}
impl ErrorBatch {
    /// Create a new empty batch.
    #[allow(dead_code)]
    pub fn new() -> Self {
        ErrorBatch {
            by_code: std::collections::HashMap::new(),
        }
    }
    /// Add an error.
    #[allow(dead_code)]
    pub fn add(&mut self, err: SpannedError) {
        self.by_code.entry(err.code).or_default().push(err);
    }
    /// Returns all errors for a given code.
    #[allow(dead_code)]
    pub fn get(&self, code: u32) -> &[SpannedError] {
        self.by_code.get(&code).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Total number of errors.
    #[allow(dead_code)]
    pub fn total(&self) -> usize {
        self.by_code.values().map(|v| v.len()).sum()
    }
}
/// An error code, combining a category and a numeric identifier.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ErrorCode {
    /// The category string (e.g. "E" for error, "W" for warning)
    pub category: String,
    /// The numeric part of the code
    pub number: u32,
}
impl ErrorCode {
    /// Create a new ErrorCode.
    #[allow(dead_code)]
    pub fn new(category: &str, number: u32) -> Self {
        ErrorCode {
            category: category.to_string(),
            number,
        }
    }
    /// Format as "E001" style string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!("{}{:04}", self.category, self.number)
    }
}
/// A lint-style warning with a code and suggestion.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct LintWarning {
    /// The lint code (e.g. "unused-variable")
    pub code: String,
    /// The warning message
    pub message: String,
    /// Optional fix suggestion
    pub suggestion: Option<String>,
    /// Byte start
    pub start: usize,
    /// Byte end
    pub end: usize,
}
impl LintWarning {
    /// Create a new lint warning.
    #[allow(dead_code)]
    pub fn new(code: &str, message: &str) -> Self {
        LintWarning {
            code: code.to_string(),
            message: message.to_string(),
            suggestion: None,
            start: 0,
            end: 0,
        }
    }
    /// Set the suggestion.
    #[allow(dead_code)]
    pub fn with_suggestion(mut self, s: &str) -> Self {
        self.suggestion = Some(s.to_string());
        self
    }
    /// Set the location.
    #[allow(dead_code)]
    pub fn at_range(mut self, start: usize, end: usize) -> Self {
        self.start = start;
        self.end = end;
        self
    }
}
/// An error window that limits the number of errors shown.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorWindow {
    /// Max errors to show
    pub max: usize,
    /// Whether the limit was exceeded
    pub truncated: bool,
    /// Shown errors
    pub shown: Vec<LocatedError>,
}
impl ErrorWindow {
    /// Create a new window.
    #[allow(dead_code)]
    pub fn new(max: usize) -> Self {
        ErrorWindow {
            max,
            truncated: false,
            shown: Vec::new(),
        }
    }
    /// Try to add an error.
    #[allow(dead_code)]
    pub fn push(&mut self, err: LocatedError) {
        if self.shown.len() < self.max {
            self.shown.push(err);
        } else {
            self.truncated = true;
        }
    }
    /// Returns a summary.
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        if self.truncated {
            format!("{} errors shown (more omitted)", self.shown.len())
        } else {
            format!("{} errors", self.shown.len())
        }
    }
}
/// A template for generating error messages.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorTemplate {
    /// Template text with {N} placeholders
    pub template: String,
}
impl ErrorTemplate {
    /// Create a new template.
    #[allow(dead_code)]
    pub fn new(template: &str) -> Self {
        ErrorTemplate {
            template: template.to_string(),
        }
    }
    /// Instantiate the template with arguments.
    #[allow(dead_code)]
    pub fn format(&self, args: &[&str]) -> String {
        let mut result = self.template.clone();
        for (i, arg) in args.iter().enumerate() {
            result = result.replace(&format!("{{{}}}", i), arg);
        }
        result
    }
}
/// A position range for error highlighting.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ErrorRange {
    /// Start byte offset
    pub start: usize,
    /// End byte offset (exclusive)
    pub end: usize,
}
impl ErrorRange {
    /// Create a new error range.
    #[allow(dead_code)]
    pub fn new(start: usize, end: usize) -> Self {
        ErrorRange { start, end }
    }
    /// Returns the length of the range.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
    /// Returns true if the range is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
    /// Returns true if this range overlaps with another.
    #[allow(dead_code)]
    pub fn overlaps(&self, other: &ErrorRange) -> bool {
        self.start < other.end && other.start < self.end
    }
}
/// An error with a substitution suggestion.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ErrorWithFix {
    /// The error message
    pub message: String,
    /// Offset to apply the fix at
    pub fix_start: usize,
    /// End offset of the region to replace
    pub fix_end: usize,
    /// The replacement text
    pub fix_text: String,
}
impl ErrorWithFix {
    /// Create a new error with fix.
    #[allow(dead_code)]
    pub fn new(message: &str, fix_start: usize, fix_end: usize, fix_text: &str) -> Self {
        ErrorWithFix {
            message: message.to_string(),
            fix_start,
            fix_end,
            fix_text: fix_text.to_string(),
        }
    }
    /// Apply the fix to a source string.
    #[allow(dead_code)]
    pub fn apply(&self, src: &str) -> String {
        let start = self.fix_start.min(src.len());
        let end = self.fix_end.min(src.len());
        format!("{}{}{}", &src[..start], self.fix_text, &src[end..])
    }
}
/// A collection of parse diagnostics accumulated during parsing.
#[derive(Clone, Debug, Default)]
pub struct ParseErrors {
    diagnostics: Vec<Diagnostic>,
}
impl ParseErrors {
    /// Create an empty error collection.
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }
    /// Add a fatal parse error.
    pub fn add_error(&mut self, err: ParseError) {
        self.diagnostics.push(Diagnostic::error(err));
    }
    /// Add a warning.
    pub fn add_warning(&mut self, err: ParseError) {
        self.diagnostics.push(Diagnostic::warning(err));
    }
    /// Add an arbitrary diagnostic.
    pub fn add(&mut self, diag: Diagnostic) {
        self.diagnostics.push(diag);
    }
    /// `true` if there are any error-severity diagnostics.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Error)
    }
    /// `true` if there are any warning-severity diagnostics.
    pub fn has_warnings(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| d.severity == Severity::Warning)
    }
    /// Return the total number of diagnostics.
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }
    /// `true` if there are no diagnostics.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
    /// Iterate over all diagnostics.
    pub fn iter(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics.iter()
    }
    /// Return only the errors.
    pub fn errors(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Error)
    }
    /// Return only the warnings.
    pub fn warnings(&self) -> impl Iterator<Item = &Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Warning)
    }
    /// Produce a formatted summary string.
    pub fn summary(&self) -> String {
        let err_count = self.errors().count();
        let warn_count = self.warnings().count();
        format!("{} error(s), {} warning(s)", err_count, warn_count)
    }
    /// Convert into the first `ParseError`, or `None` if there are none.
    pub fn first_error(&self) -> Option<&ParseError> {
        self.errors().next().map(|d| &d.error)
    }
}
/// Parse error.
#[derive(Clone, Debug, PartialEq)]
pub struct ParseError {
    /// Error kind
    pub kind: ParseErrorKind,
    /// Source span
    pub span: Span,
}
impl ParseError {
    /// Create a new parse error.
    pub fn new(kind: ParseErrorKind, span: Span) -> Self {
        Self { kind, span }
    }
    /// Create an unexpected token error.
    pub fn unexpected(expected: Vec<String>, got: TokenKind, span: Span) -> Self {
        Self::new(ParseErrorKind::UnexpectedToken { expected, got }, span)
    }
    /// Create an unexpected end of file error.
    pub fn unexpected_eof(expected: Vec<String>, span: Span) -> Self {
        Self::new(ParseErrorKind::UnexpectedEof { expected }, span)
    }
    /// Get the human-readable message for this error.
    pub fn message(&self) -> String {
        format!("{}", self.kind)
    }
    /// Get the line number (from span).
    pub fn line(&self) -> u32 {
        self.span.line as u32
    }
    /// Get the column number (from span).
    pub fn col(&self) -> u32 {
        self.span.column as u32
    }
    /// Check whether this is an "unexpected end of file" error.
    pub fn is_eof(&self) -> bool {
        matches!(self.kind, ParseErrorKind::UnexpectedEof { .. })
    }
    /// Create an error with message, line, and column.
    pub fn from_msg(msg: &str, line: u32, col: u32) -> Self {
        Self::new(
            ParseErrorKind::InvalidSyntax(msg.to_string()),
            crate::tokens::Span::new(0, 0, line as usize, col as usize),
        )
    }
}
/// A structured error with source location information.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct LocatedError {
    /// The error message
    pub message: String,
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
    /// Source line number (1-based)
    pub line: usize,
    /// Source column number (1-based)
    pub col: usize,
}
impl LocatedError {
    /// Create a new LocatedError.
    #[allow(dead_code)]
    pub fn new(message: &str, start: usize, end: usize, line: usize, col: usize) -> Self {
        LocatedError {
            message: message.to_string(),
            start,
            end,
            line,
            col,
        }
    }
    /// Format the error as "line:col: message".
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!("{}:{}: {}", self.line, self.col, self.message)
    }
}
/// A collection of lint warnings.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct LintReport {
    /// All warnings
    pub warnings: Vec<LintWarning>,
}
impl LintReport {
    /// Create a new empty lint report.
    #[allow(dead_code)]
    pub fn new() -> Self {
        LintReport {
            warnings: Vec::new(),
        }
    }
    /// Add a warning.
    #[allow(dead_code)]
    pub fn add(&mut self, w: LintWarning) {
        self.warnings.push(w);
    }
    /// Filter warnings by code.
    #[allow(dead_code)]
    pub fn by_code(&self, code: &str) -> Vec<&LintWarning> {
        self.warnings.iter().filter(|w| w.code == code).collect()
    }
    /// Returns the count of warnings.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.warnings.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.warnings.is_empty()
    }
    /// Format all warnings.
    #[allow(dead_code)]
    pub fn format_all(&self) -> String {
        self.warnings
            .iter()
            .map(|w| format!("[{}] {}", w.code, w.message))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// A recoverable parse error with suggestions.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct RecoverableError {
    /// The primary error message
    pub message: String,
    /// Recovery suggestions
    pub suggestions: Vec<String>,
    /// Whether recovery was attempted
    pub recovered: bool,
}
impl RecoverableError {
    /// Create a new recoverable error.
    #[allow(dead_code)]
    pub fn new(message: &str) -> Self {
        RecoverableError {
            message: message.to_string(),
            suggestions: Vec::new(),
            recovered: false,
        }
    }
    /// Add a suggestion.
    #[allow(dead_code)]
    pub fn suggest(mut self, s: &str) -> Self {
        self.suggestions.push(s.to_string());
        self
    }
    /// Mark as recovered.
    #[allow(dead_code)]
    pub fn mark_recovered(mut self) -> Self {
        self.recovered = true;
        self
    }
}
/// A collection of diagnostics.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct DiagnosticBag {
    /// All diagnostics in this bag
    pub items: Vec<FullDiagnostic>,
}
impl DiagnosticBag {
    /// Create a new empty bag.
    #[allow(dead_code)]
    pub fn new() -> Self {
        DiagnosticBag { items: Vec::new() }
    }
    /// Add a diagnostic.
    #[allow(dead_code)]
    pub fn add(&mut self, diag: FullDiagnostic) {
        self.items.push(diag);
    }
    /// Returns errors only.
    #[allow(dead_code)]
    pub fn errors(&self) -> Vec<&FullDiagnostic> {
        self.items
            .iter()
            .filter(|d| {
                d.severity == DiagnosticSeverity::Error || d.severity == DiagnosticSeverity::Fatal
            })
            .collect()
    }
    /// Returns warnings only.
    #[allow(dead_code)]
    pub fn warnings(&self) -> Vec<&FullDiagnostic> {
        self.items
            .iter()
            .filter(|d| d.severity == DiagnosticSeverity::Warning)
            .collect()
    }
    /// Returns true if there are any errors.
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        self.items
            .iter()
            .any(|d| d.severity >= DiagnosticSeverity::Error)
    }
    /// Returns the total count.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    /// Format all diagnostics as a string.
    #[allow(dead_code)]
    pub fn format_all(&self) -> String {
        self.items
            .iter()
            .map(|d| d.display())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// Builder for constructing a `ParseError` incrementally.
#[derive(Debug)]
pub struct ParseErrorBuilder {
    kind: Option<ParseErrorKind>,
    span: Option<Span>,
}
impl ParseErrorBuilder {
    /// Start a new builder.
    pub fn new() -> Self {
        Self {
            kind: None,
            span: None,
        }
    }
    /// Set the error kind.
    pub fn kind(mut self, kind: ParseErrorKind) -> Self {
        self.kind = Some(kind);
        self
    }
    /// Set the span.
    pub fn at(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }
    /// Produce the `ParseError`, using a dummy span if none was set.
    pub fn build(self) -> ParseError {
        let kind = self
            .kind
            .unwrap_or(ParseErrorKind::Other("unknown error".to_string()));
        let span = self.span.unwrap_or_else(|| Span::new(0, 0, 1, 1));
        ParseError::new(kind, span)
    }
}
/// Parse error kinds.
#[derive(Clone, Debug, PartialEq)]
pub enum ParseErrorKind {
    /// Unexpected token
    UnexpectedToken {
        /// Expected tokens
        expected: Vec<String>,
        /// Got token
        got: TokenKind,
    },
    /// Unexpected end of file
    UnexpectedEof {
        /// Expected tokens
        expected: Vec<String>,
    },
    /// Invalid syntax
    InvalidSyntax(String),
    /// Duplicate declaration
    DuplicateDeclaration(String),
    /// Invalid binder
    InvalidBinder(String),
    /// Invalid pattern
    InvalidPattern(String),
    /// Invalid universe level
    InvalidUniverse(String),
    /// Other error
    Other(String),
}
/// A contextual error that includes surrounding source lines.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ContextualError {
    /// The error message
    pub message: String,
    /// Lines of context (before, at, after)
    pub context_lines: Vec<String>,
    /// Index in context_lines of the error line
    pub error_line_idx: usize,
    /// Column of the error
    pub col: usize,
}
impl ContextualError {
    /// Format the contextual error with surrounding lines.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        let mut out = format!("error: {}\n", self.message);
        for (i, line) in self.context_lines.iter().enumerate() {
            if i == self.error_line_idx {
                out.push_str(&format!("  > {}\n", line));
                out.push_str(&format!(
                    "    {}{}\n",
                    " ".repeat(self.col.saturating_sub(1)),
                    "^"
                ));
            } else {
                out.push_str(&format!("    {}\n", line));
            }
        }
        out
    }
}
/// A simple error rate limiter that drops errors after a threshold.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorRateLimiter {
    /// Maximum errors allowed
    pub max_errors: usize,
    /// Current error count
    pub count: usize,
    /// Whether the limit has been exceeded
    pub exceeded: bool,
}
impl ErrorRateLimiter {
    /// Create a new rate limiter.
    #[allow(dead_code)]
    pub fn new(max_errors: usize) -> Self {
        ErrorRateLimiter {
            max_errors,
            count: 0,
            exceeded: false,
        }
    }
    /// Returns true if the error should be accepted.
    #[allow(dead_code)]
    pub fn accept(&mut self) -> bool {
        if self.count >= self.max_errors {
            self.exceeded = true;
            return false;
        }
        self.count += 1;
        true
    }
}
/// A diagnostic with severity, code, and location.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct FullDiagnostic {
    /// Severity of this diagnostic
    pub severity: DiagnosticSeverity,
    /// Optional error code
    pub code: Option<ErrorCode>,
    /// The primary message
    pub message: String,
    /// Source location
    pub location: Option<LocatedError>,
    /// Related notes
    pub notes: Vec<String>,
}
impl FullDiagnostic {
    /// Create a new error diagnostic.
    #[allow(dead_code)]
    pub fn error(message: &str) -> Self {
        FullDiagnostic {
            severity: DiagnosticSeverity::Error,
            code: None,
            message: message.to_string(),
            location: None,
            notes: Vec::new(),
        }
    }
    /// Create a new warning diagnostic.
    #[allow(dead_code)]
    pub fn warning(message: &str) -> Self {
        FullDiagnostic {
            severity: DiagnosticSeverity::Warning,
            code: None,
            message: message.to_string(),
            location: None,
            notes: Vec::new(),
        }
    }
    /// Add a note to this diagnostic.
    #[allow(dead_code)]
    pub fn with_note(mut self, note: &str) -> Self {
        self.notes.push(note.to_string());
        self
    }
    /// Attach a location.
    #[allow(dead_code)]
    pub fn at(mut self, loc: LocatedError) -> Self {
        self.location = Some(loc);
        self
    }
    /// Format this diagnostic for display.
    #[allow(dead_code)]
    pub fn display(&self) -> String {
        let mut out = format!("{}: {}", self.severity, self.message);
        if let Some(loc) = &self.location {
            out = format!(
                "{}:{}: {}: {}",
                loc.line, loc.col, self.severity, self.message
            );
        }
        for note in &self.notes {
            out.push_str(&format!("\n  note: {}", note));
        }
        out
    }
}
/// A structured hint for error recovery.
#[derive(Clone, Debug)]
pub struct RecoveryHint {
    /// Human-readable description of the hint.
    pub message: String,
    /// Optional text replacement suggestion.
    pub replacement: Option<String>,
}
impl RecoveryHint {
    /// Construct a hint.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            replacement: None,
        }
    }
    /// Attach a replacement suggestion.
    pub fn with_replacement(mut self, r: impl Into<String>) -> Self {
        self.replacement = Some(r.into());
        self
    }
}
/// Severity level for diagnostics.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticSeverity {
    /// Informational note
    Note,
    /// Warning that doesn't prevent compilation
    Warning,
    /// Error that prevents compilation
    Error,
    /// Fatal error that stops compilation immediately
    Fatal,
}
/// A filter that suppresses errors matching a given substring.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ErrorMessageFilter {
    /// Substrings to suppress
    pub suppressed_patterns: Vec<String>,
}
impl ErrorMessageFilter {
    /// Create a new filter.
    #[allow(dead_code)]
    pub fn new() -> Self {
        ErrorMessageFilter {
            suppressed_patterns: Vec::new(),
        }
    }
    /// Add a pattern to suppress.
    #[allow(dead_code)]
    pub fn suppress(mut self, pattern: &str) -> Self {
        self.suppressed_patterns.push(pattern.to_string());
        self
    }
    /// Returns true if the given error message should be shown.
    #[allow(dead_code)]
    pub fn should_show(&self, msg: &str) -> bool {
        !self
            .suppressed_patterns
            .iter()
            .any(|p| msg.contains(p.as_str()))
    }
    /// Filter a list of errors.
    #[allow(dead_code)]
    pub fn filter<'a>(&self, errors: &'a [LocatedError]) -> Vec<&'a LocatedError> {
        errors
            .iter()
            .filter(|e| self.should_show(&e.message))
            .collect()
    }
}
/// Severity level of a parse diagnostic.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Informational note.
    Note,
    /// Non-fatal warning.
    Warning,
    /// Fatal error that stops parsing.
    Error,
}
/// A span-based error with formatted message.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct SpannedError {
    /// The error code
    pub code: u32,
    /// The formatted message
    pub message: String,
    /// Start byte offset
    pub start: usize,
    /// End byte offset
    pub end: usize,
}
impl SpannedError {
    /// Create a new spanned error.
    #[allow(dead_code)]
    pub fn new(code: u32, message: &str, start: usize, end: usize) -> Self {
        SpannedError {
            code,
            message: message.to_string(),
            start,
            end,
        }
    }
    /// Check if this error overlaps with a byte range.
    #[allow(dead_code)]
    pub fn overlaps(&self, lo: usize, hi: usize) -> bool {
        self.start < hi && lo < self.end
    }
}
/// A sentinel error used in testing.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct SentinelError {
    /// Message
    pub message: String,
    /// A unique sentinel ID
    pub sentinel_id: u64,
}
impl SentinelError {
    /// Create a new sentinel error.
    #[allow(dead_code)]
    pub fn new(id: u64, message: &str) -> Self {
        SentinelError {
            message: message.to_string(),
            sentinel_id: id,
        }
    }
}
/// A rich diagnostic combining a `ParseError` with additional context.
#[derive(Clone, Debug)]
pub struct Diagnostic {
    /// Underlying parse error.
    pub error: ParseError,
    /// Severity.
    pub severity: Severity,
    /// Optional hint message.
    pub hint: Option<String>,
    /// Optional secondary label.
    pub secondary: Option<String>,
}
impl Diagnostic {
    /// Construct a new `Diagnostic` from a parse error.
    pub fn new(error: ParseError, severity: Severity) -> Self {
        Self {
            error,
            severity,
            hint: None,
            secondary: None,
        }
    }
    /// Construct an error-severity diagnostic.
    pub fn error(error: ParseError) -> Self {
        Self::new(error, Severity::Error)
    }
    /// Construct a warning-severity diagnostic.
    pub fn warning(error: ParseError) -> Self {
        Self::new(error, Severity::Warning)
    }
    /// Attach a hint message.
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
    /// Attach a secondary label.
    pub fn with_secondary(mut self, label: impl Into<String>) -> Self {
        self.secondary = Some(label.into());
        self
    }
    /// Format a multi-line diagnostic report using the source text.
    pub fn report(&self, source: &str) -> String {
        let err = &self.error;
        let span = &err.span;
        let line_text = source
            .lines()
            .nth(span.line.saturating_sub(1))
            .unwrap_or("");
        let col = span.column.saturating_sub(1);
        let underline_len = (span.end.saturating_sub(span.start)).max(1);
        let spaces = " ".repeat(col);
        let carets = "^".repeat(
            underline_len
                .min(line_text.len().saturating_sub(col))
                .max(1),
        );
        let mut out = format!(
            "{}: {} [{}:{}]\n  {}\n  {}{}\n",
            self.severity,
            err.message(),
            span.line,
            span.column,
            line_text,
            spaces,
            carets,
        );
        if let Some(hint) = &self.hint {
            out.push_str(&format!("  hint: {}\n", hint));
        }
        if let Some(sec) = &self.secondary {
            out.push_str(&format!("  note: {}\n", sec));
        }
        out
    }
}
/// A multi-file error accumulator.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct MultiFileErrors {
    /// Map from filename to errors
    pub per_file: std::collections::HashMap<String, Vec<LocatedError>>,
}
impl MultiFileErrors {
    /// Create a new multi-file accumulator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        MultiFileErrors {
            per_file: std::collections::HashMap::new(),
        }
    }
    /// Add an error for a file.
    #[allow(dead_code)]
    pub fn add(&mut self, file: &str, err: LocatedError) {
        self.per_file.entry(file.to_string()).or_default().push(err);
    }
    /// Get all errors for a file.
    #[allow(dead_code)]
    pub fn get(&self, file: &str) -> &[LocatedError] {
        self.per_file.get(file).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Total error count across all files.
    #[allow(dead_code)]
    pub fn total(&self) -> usize {
        self.per_file.values().map(|v| v.len()).sum()
    }
    /// Returns a list of file names that have errors.
    #[allow(dead_code)]
    pub fn files_with_errors(&self) -> Vec<&str> {
        self.per_file.keys().map(|s| s.as_str()).collect()
    }
}
