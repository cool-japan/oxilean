//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::tokens::Span;

/// Synchronization token for error recovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncToken {
    /// Semicolon `;`
    Semicolon,
    /// End keyword
    End,
    /// Declaration keyword (def, theorem, etc.)
    Declaration,
    /// Right brace `}`
    RightBrace,
    /// Right parenthesis `)`
    RightParen,
    /// End of file
    Eof,
}
/// Renders diagnostics to human-readable strings in various formats.
#[allow(dead_code)]
pub struct DiagnosticRenderer {
    /// Source text for context lines.
    pub source: String,
    /// Whether to include ANSI color codes.
    pub use_color: bool,
    /// Whether to include fix suggestions in the output.
    pub show_fixes: bool,
    /// Maximum number of context lines to show around each diagnostic.
    pub context_lines: usize,
}
#[allow(dead_code)]
impl DiagnosticRenderer {
    /// Create a renderer for the given source.
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            use_color: false,
            show_fixes: true,
            context_lines: 1,
        }
    }
    /// Enable or disable ANSI color output.
    pub fn with_color(mut self, v: bool) -> Self {
        self.use_color = v;
        self
    }
    /// Enable or disable fix suggestion output.
    pub fn with_show_fixes(mut self, v: bool) -> Self {
        self.show_fixes = v;
        self
    }
    /// Set the number of context lines to display.
    pub fn with_context_lines(mut self, n: usize) -> Self {
        self.context_lines = n;
        self
    }
    /// Render a single `Diagnostic` to a string.
    pub fn render(&self, diag: &Diagnostic) -> String {
        let mut out = String::new();
        let severity_tag = match diag.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Hint => "hint",
        };
        if let Some(code) = &diag.code {
            out.push_str(&format!("{}[{}]: {}\n", severity_tag, code, diag.message));
        } else {
            out.push_str(&format!("{}: {}\n", severity_tag, diag.message));
        }
        out.push_str(&format!(" --> {}:{}\n", diag.span.line, diag.span.column));
        let ctx = self.extract_context(diag.span.line);
        out.push_str(&ctx);
        let col = if diag.span.column > 0 {
            diag.span.column - 1
        } else {
            0
        };
        let len = (diag.span.end.saturating_sub(diag.span.start)).max(1);
        out.push_str(&format!("{}^\n", " ".repeat(col)));
        let _ = len;
        for label in &diag.labels {
            out.push_str(&format!("  note: {}\n", label.text));
        }
        if let Some(h) = &diag.help {
            out.push_str(&format!("  help: {}\n", h));
        }
        if self.show_fixes {
            for fix in &diag.fixes {
                out.push_str(&format!(
                    "  suggestion: {} → `{}`\n",
                    fix.message, fix.replacement
                ));
            }
        }
        out
    }
    /// Render a slice of diagnostics in order.
    pub fn render_all(&self, diags: &[Diagnostic]) -> String {
        diags
            .iter()
            .map(|d| self.render(d))
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Render only errors from a collector.
    pub fn render_errors(&self, collector: &DiagnosticCollector) -> String {
        let errors: Vec<&Diagnostic> = collector
            .diagnostics()
            .iter()
            .filter(|d| d.is_error())
            .collect();
        errors
            .iter()
            .map(|d| self.render(d))
            .collect::<Vec<_>>()
            .join("\n")
    }
    fn extract_context(&self, line: usize) -> String {
        if line == 0 {
            return String::new();
        }
        let start_line = line.saturating_sub(self.context_lines);
        let end_line = line + self.context_lines;
        let lines: Vec<&str> = self.source.lines().collect();
        let mut out = String::new();
        for (idx, l) in lines.iter().enumerate() {
            let lnum = idx + 1;
            if lnum >= start_line && lnum <= end_line {
                out.push_str(&format!("{:4} | {}\n", lnum, l));
            }
        }
        out
    }
}
/// Exports diagnostics to various text formats.
#[allow(dead_code)]
pub struct DiagnosticExporter;
#[allow(dead_code)]
impl DiagnosticExporter {
    /// Export a single `Diagnostic` as a JSON-like string.
    pub fn to_json(d: &Diagnostic) -> String {
        let severity = match d.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Hint => "hint",
        };
        let code = d
            .code
            .map(|c| format!("\"{}\"", c))
            .unwrap_or_else(|| "null".to_string());
        format!(
            r#"{{"severity":"{}","code":{},"message":"{}","line":{},"col":{}}}"#,
            severity,
            code,
            d.message.replace('"', "\\\""),
            d.span.line,
            d.span.column
        )
    }
    /// Export a `DiagnosticCollector` as a JSON array.
    pub fn collector_to_json(c: &DiagnosticCollector) -> String {
        let items: Vec<String> = c.diagnostics().iter().map(Self::to_json).collect();
        format!("[{}]", items.join(","))
    }
    /// Export a `Diagnostic` as a compact one-liner.
    pub fn to_oneliner(d: &Diagnostic) -> String {
        format!(
            "{}:{}: {}: {}",
            d.span.line,
            d.span.column,
            match d.severity {
                Severity::Error => "error",
                Severity::Warning => "warning",
                Severity::Info => "info",
                Severity::Hint => "hint",
            },
            d.message
        )
    }
    /// Export all diagnostics from a collector as one-liners, one per line.
    pub fn collector_to_oneliners(c: &DiagnosticCollector) -> String {
        c.diagnostics()
            .iter()
            .map(Self::to_oneliner)
            .collect::<Vec<_>>()
            .join("\n")
    }
    /// Export as a CSV line: `line,col,severity,message`.
    pub fn to_csv(d: &Diagnostic) -> String {
        let severity = match d.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Hint => "hint",
        };
        format!(
            "{},{},{},\"{}\"",
            d.span.line,
            d.span.column,
            severity,
            d.message.replace('"', "\"\"")
        )
    }
    /// Export all diagnostics from a collector as CSV rows (with header).
    pub fn collector_to_csv(c: &DiagnosticCollector) -> String {
        let mut out = "line,col,severity,message\n".to_string();
        for d in c.diagnostics() {
            out.push_str(&Self::to_csv(d));
            out.push('\n');
        }
        out
    }
}
/// Aggregated statistics over a `DiagnosticCollector`.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DiagnosticStats {
    /// Total errors.
    pub errors: usize,
    /// Total warnings.
    pub warnings: usize,
    /// Total infos.
    pub infos: usize,
    /// Total hints.
    pub hints: usize,
    /// Diagnostics with code.
    pub with_code: usize,
    /// Diagnostics with fix.
    pub with_fix: usize,
    /// Diagnostics with help.
    pub with_help: usize,
}
#[allow(dead_code)]
impl DiagnosticStats {
    /// Compute stats from a `DiagnosticCollector`.
    pub fn from_collector(c: &DiagnosticCollector) -> Self {
        let mut s = Self::default();
        for d in c.diagnostics() {
            match d.severity {
                Severity::Error => s.errors += 1,
                Severity::Warning => s.warnings += 1,
                Severity::Info => s.infos += 1,
                Severity::Hint => s.hints += 1,
            }
            if d.code.is_some() {
                s.with_code += 1;
            }
            if !d.fixes.is_empty() {
                s.with_fix += 1;
            }
            if d.help.is_some() {
                s.with_help += 1;
            }
        }
        s
    }
    /// Total diagnostics.
    pub fn total(&self) -> usize {
        self.errors + self.warnings + self.infos + self.hints
    }
    /// True if there are any errors.
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }
    /// Format a compact summary.
    pub fn summary(&self) -> String {
        format!(
            "{} errors, {} warnings, {} infos, {} hints",
            self.errors, self.warnings, self.infos, self.hints
        )
    }
}
/// Policy that controls how errors affect compilation flow.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiagnosticPolicy {
    /// Abort on first error.
    FailFast,
    /// Collect all errors before failing.
    CollectAll,
    /// Treat warnings as errors.
    WarningsAsErrors,
    /// Never fail (permissive mode, useful for IDEs).
    Permissive,
}
#[allow(dead_code)]
impl DiagnosticPolicy {
    /// Return `true` if the collector indicates a compilation failure under this policy.
    pub fn should_fail(&self, c: &DiagnosticCollector) -> bool {
        match self {
            DiagnosticPolicy::FailFast => c.has_errors(),
            DiagnosticPolicy::CollectAll => c.has_errors(),
            DiagnosticPolicy::WarningsAsErrors => c.has_errors() || c.warning_count() > 0,
            DiagnosticPolicy::Permissive => false,
        }
    }
    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            DiagnosticPolicy::FailFast => "fail-fast",
            DiagnosticPolicy::CollectAll => "collect-all",
            DiagnosticPolicy::WarningsAsErrors => "warnings-as-errors",
            DiagnosticPolicy::Permissive => "permissive",
        }
    }
}
/// A diagnostic event for logging.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct DiagnosticEvent {
    /// The message
    pub message: String,
    /// A unique event ID
    pub id: u64,
}
impl DiagnosticEvent {
    /// Create a new event.
    #[allow(dead_code)]
    pub fn new(id: u64, message: &str) -> Self {
        DiagnosticEvent {
            id,
            message: message.to_string(),
        }
    }
}
/// Diagnostic label for additional context.
#[derive(Debug, Clone)]
pub struct DiagnosticLabel {
    /// Label text
    pub text: String,
    /// Label span
    pub span: Span,
}
/// Metadata associated with a `SyncToken`.
#[allow(dead_code)]
pub struct SyncTokenInfo {
    /// The sync token kind.
    pub kind: SyncToken,
    /// Human-readable name.
    pub name: &'static str,
    /// Whether this sync token terminates a statement.
    pub is_statement_end: bool,
}
#[allow(dead_code)]
impl SyncTokenInfo {
    /// Return info for all sync token kinds.
    pub fn all() -> &'static [SyncTokenInfo] {
        &[
            SyncTokenInfo {
                kind: SyncToken::Semicolon,
                name: "semicolon",
                is_statement_end: true,
            },
            SyncTokenInfo {
                kind: SyncToken::End,
                name: "end",
                is_statement_end: true,
            },
            SyncTokenInfo {
                kind: SyncToken::Declaration,
                name: "declaration keyword",
                is_statement_end: true,
            },
            SyncTokenInfo {
                kind: SyncToken::RightBrace,
                name: "right brace",
                is_statement_end: false,
            },
            SyncTokenInfo {
                kind: SyncToken::RightParen,
                name: "right paren",
                is_statement_end: false,
            },
            SyncTokenInfo {
                kind: SyncToken::Eof,
                name: "end of file",
                is_statement_end: true,
            },
        ]
    }
}
/// Utilities for working with `Span` values in diagnostic contexts.
#[allow(dead_code)]
pub struct SpanUtils;
#[allow(dead_code)]
impl SpanUtils {
    /// Return `true` if `inner` is contained within `outer`.
    pub fn contains(outer: &Span, inner: &Span) -> bool {
        outer.start <= inner.start && inner.end <= outer.end
    }
    /// Return `true` if two spans overlap.
    pub fn overlaps(a: &Span, b: &Span) -> bool {
        a.start < b.end && b.start < a.end
    }
    /// Merge two spans into the smallest span that covers both.
    pub fn merge(a: &Span, b: &Span) -> Span {
        let start = a.start.min(b.start);
        let end = a.end.max(b.end);
        let line = a.line.min(b.line);
        let column = if a.line < b.line {
            a.column
        } else if b.line < a.line {
            b.column
        } else {
            a.column.min(b.column)
        };
        Span::new(start, end, line, column)
    }
    /// Length of a span in bytes.
    pub fn byte_len(span: &Span) -> usize {
        span.end.saturating_sub(span.start)
    }
    /// True if the span is empty (zero length).
    pub fn is_empty(span: &Span) -> bool {
        Self::byte_len(span) == 0
    }
    /// Expand a span by `n` bytes in both directions, clamped to `[0, max_end]`.
    pub fn expand(span: &Span, n: usize, max_end: usize) -> Span {
        let start = span.start.saturating_sub(n);
        let end = (span.end + n).min(max_end);
        Span::new(start, end, span.line, span.column)
    }
    /// Extract source text covered by the span.
    pub fn extract<'a>(span: &Span, source: &'a str) -> &'a str {
        source.get(span.start..span.end).unwrap_or("")
    }
    /// Build a span from a byte range and a source string (computing line/col).
    pub fn from_byte_range(start: usize, end: usize, source: &str) -> Span {
        let before = &source[..start.min(source.len())];
        let line = before.chars().filter(|&c| c == '\n').count() + 1;
        let col = before.rfind('\n').map(|p| start - p).unwrap_or(start + 1);
        Span::new(start, end, line, col)
    }
}
/// A diagnostic severity filter.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SeverityFilter {
    /// Minimum severity to include
    pub min_severity: u8,
}
impl SeverityFilter {
    /// Create a new filter that shows all.
    #[allow(dead_code)]
    pub fn all() -> Self {
        SeverityFilter { min_severity: 0 }
    }
    /// Create a filter that only shows errors.
    #[allow(dead_code)]
    pub fn errors_only() -> Self {
        SeverityFilter { min_severity: 2 }
    }
}
/// Simple plaintext diagnostic printer (no colour, no source context).
#[allow(dead_code)]
pub struct DiagnosticPrinter {
    policy: DiagnosticPolicy,
}
#[allow(dead_code)]
impl DiagnosticPrinter {
    /// Create a printer with the given policy.
    pub fn new(policy: DiagnosticPolicy) -> Self {
        Self { policy }
    }
    /// Print all diagnostics from a collector to a string.
    pub fn print(&self, c: &DiagnosticCollector) -> String {
        let mut out = String::new();
        for d in c.diagnostics() {
            out.push_str(&format!("{}\n", d));
        }
        out
    }
    /// Return `true` if compilation should be considered failed.
    pub fn should_fail(&self, c: &DiagnosticCollector) -> bool {
        self.policy.should_fail(c)
    }
}
/// Diagnostic severity level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Error - compilation cannot proceed
    Error,
    /// Warning - potential issue
    Warning,
    /// Info - informational message
    Info,
    /// Hint - suggestion for improvement
    Hint,
}
/// Diagnostic code for structured error categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    /// Unexpected token
    E0001,
    /// Unterminated string
    E0002,
    /// Unmatched bracket
    E0003,
    /// Missing semicolon
    E0004,
    /// Invalid number literal
    E0005,
    /// Type mismatch
    E0100,
    /// Undeclared variable
    E0101,
    /// Cannot infer type
    E0102,
    /// Too many arguments
    E0103,
    /// Too few arguments
    E0104,
    /// No goals to solve
    E0200,
    /// Tactic failed
    E0201,
    /// Unsolved goals
    E0202,
    /// Internal error
    E0900,
    /// Not implemented
    E0901,
}
/// Fluent builder for constructing `Diagnostic` values.
#[allow(dead_code)]
pub struct DiagnosticBuilder {
    severity: Severity,
    message: String,
    span: Span,
    labels: Vec<DiagnosticLabel>,
    help: Option<String>,
    code: Option<DiagnosticCode>,
    fixes: Vec<CodeFix>,
}
#[allow(dead_code)]
impl DiagnosticBuilder {
    /// Start building an error diagnostic.
    pub fn error(message: impl Into<String>, span: Span) -> Self {
        Self {
            severity: Severity::Error,
            message: message.into(),
            span,
            labels: Vec::new(),
            help: None,
            code: None,
            fixes: Vec::new(),
        }
    }
    /// Start building a warning diagnostic.
    pub fn warning(message: impl Into<String>, span: Span) -> Self {
        Self {
            severity: Severity::Warning,
            message: message.into(),
            span,
            labels: Vec::new(),
            help: None,
            code: None,
            fixes: Vec::new(),
        }
    }
    /// Start building an info diagnostic.
    pub fn info(message: impl Into<String>, span: Span) -> Self {
        Self {
            severity: Severity::Info,
            message: message.into(),
            span,
            labels: Vec::new(),
            help: None,
            code: None,
            fixes: Vec::new(),
        }
    }
    /// Start building a hint diagnostic.
    pub fn hint(message: impl Into<String>, span: Span) -> Self {
        Self {
            severity: Severity::Hint,
            message: message.into(),
            span,
            labels: Vec::new(),
            help: None,
            code: None,
            fixes: Vec::new(),
        }
    }
    /// Add a label.
    pub fn label(mut self, text: impl Into<String>, span: Span) -> Self {
        self.labels.push(DiagnosticLabel {
            text: text.into(),
            span,
        });
        self
    }
    /// Set help text.
    pub fn help(mut self, h: impl Into<String>) -> Self {
        self.help = Some(h.into());
        self
    }
    /// Set the diagnostic code.
    pub fn code(mut self, c: DiagnosticCode) -> Self {
        self.code = Some(c);
        self
    }
    /// Add a fix suggestion.
    pub fn fix(
        mut self,
        message: impl Into<String>,
        span: Span,
        replacement: impl Into<String>,
    ) -> Self {
        self.fixes.push(CodeFix {
            message: message.into(),
            span,
            replacement: replacement.into(),
        });
        self
    }
    /// Finalise and produce a `Diagnostic`.
    pub fn build(self) -> Diagnostic {
        Diagnostic {
            severity: self.severity,
            message: self.message,
            span: self.span,
            labels: self.labels,
            help: self.help,
            code: self.code,
            fixes: self.fixes,
        }
    }
}
/// Diagnostic message.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Severity level
    pub severity: Severity,
    /// Primary message
    pub message: String,
    /// Source location
    pub span: Span,
    /// Additional labels
    pub labels: Vec<DiagnosticLabel>,
    /// Help text
    pub help: Option<String>,
    /// Structured diagnostic code
    pub code: Option<DiagnosticCode>,
    /// Suggested code fixes
    pub fixes: Vec<CodeFix>,
}
impl Diagnostic {
    /// Create a new error diagnostic.
    pub fn error(message: String, span: Span) -> Self {
        Self {
            severity: Severity::Error,
            message,
            span,
            labels: Vec::new(),
            help: None,
            code: None,
            fixes: Vec::new(),
        }
    }
    /// Create a new warning diagnostic.
    pub fn warning(message: String, span: Span) -> Self {
        Self {
            severity: Severity::Warning,
            message,
            span,
            labels: Vec::new(),
            help: None,
            code: None,
            fixes: Vec::new(),
        }
    }
    /// Create a new info diagnostic.
    pub fn info(message: String, span: Span) -> Self {
        Self {
            severity: Severity::Info,
            message,
            span,
            labels: Vec::new(),
            help: None,
            code: None,
            fixes: Vec::new(),
        }
    }
    /// Create a note diagnostic (alias for info with specific semantics).
    #[allow(dead_code)]
    pub fn note(message: String, span: Span) -> Self {
        Self {
            severity: Severity::Info,
            message,
            span,
            labels: Vec::new(),
            help: None,
            code: None,
            fixes: Vec::new(),
        }
    }
    /// Add a label to this diagnostic.
    pub fn with_label(mut self, text: String, span: Span) -> Self {
        self.labels.push(DiagnosticLabel { text, span });
        self
    }
    /// Add help text to this diagnostic.
    pub fn with_help(mut self, help: String) -> Self {
        self.help = Some(help);
        self
    }
    /// Set the diagnostic code.
    #[allow(dead_code)]
    pub fn with_code(mut self, code: DiagnosticCode) -> Self {
        self.code = Some(code);
        self
    }
    /// Add a code fix suggestion.
    #[allow(dead_code)]
    pub fn with_fix(mut self, fix: CodeFix) -> Self {
        self.fixes.push(fix);
        self
    }
    /// Check if this is an error.
    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }
    /// Check if this is a warning.
    pub fn is_warning(&self) -> bool {
        self.severity == Severity::Warning
    }
    /// Format this diagnostic with rich source context.
    ///
    /// Produces output similar to rustc error messages, including
    /// the source line and an underline marker pointing to the error.
    #[allow(dead_code)]
    pub fn format_rich(&self, source: &str) -> String {
        let severity_str = match self.severity {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
            Severity::Hint => "hint",
        };
        let mut output = String::new();
        if let Some(code) = &self.code {
            output.push_str(&format!("{}[{}]: {}\n", severity_str, code, self.message));
        } else {
            output.push_str(&format!("{}: {}\n", severity_str, self.message));
        }
        output.push_str(&format!(" --> {}:{}\n", self.span.line, self.span.column));
        let highlight = Self::format_line_highlight(source, &self.span);
        if !highlight.is_empty() {
            output.push_str(&highlight);
        }
        for label in &self.labels {
            output.push_str(&format!("  = {}\n", label.text));
        }
        if let Some(help) = &self.help {
            output.push_str(&format!("  = help: {}\n", help));
        }
        for fix in &self.fixes {
            output.push_str(&format!(
                "  = fix: {} -> `{}`\n",
                fix.message, fix.replacement
            ));
        }
        output
    }
    /// Format a source line with an underline highlighting the span.
    ///
    /// Returns a string with the source line and a caret line pointing
    /// to the error location.
    #[allow(dead_code)]
    pub fn format_line_highlight(source: &str, span: &Span) -> String {
        let lines: Vec<&str> = source.lines().collect();
        if span.line == 0 || span.line > lines.len() {
            return String::new();
        }
        let line_content = lines[span.line - 1];
        let line_num = span.line;
        let line_num_width = format!("{}", line_num).len();
        let mut output = String::new();
        output.push_str(&format!("{} |\n", " ".repeat(line_num_width)));
        output.push_str(&format!("{} | {}\n", line_num, line_content));
        let col = if span.column > 0 { span.column - 1 } else { 0 };
        let underline_len = if span.end > span.start {
            span.end - span.start
        } else {
            1
        };
        let underline_len = underline_len.min(line_content.len().saturating_sub(col));
        let underline_len = if underline_len == 0 { 1 } else { underline_len };
        output.push_str(&format!(
            "{} | {}{}",
            " ".repeat(line_num_width),
            " ".repeat(col),
            "^".repeat(underline_len)
        ));
        output.push('\n');
        output
    }
}
/// Aggregates multiple `DiagnosticCollector`s into a unified view.
#[allow(dead_code)]
pub struct DiagnosticAggregator {
    collectors: Vec<DiagnosticCollector>,
    label: String,
}
#[allow(dead_code)]
impl DiagnosticAggregator {
    /// Create a new aggregator.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            collectors: Vec::new(),
            label: label.into(),
        }
    }
    /// Add a collector to the aggregator.
    pub fn add_collector(&mut self, c: DiagnosticCollector) {
        self.collectors.push(c);
    }
    /// Total error count across all collectors.
    pub fn total_errors(&self) -> usize {
        self.collectors.iter().map(|c| c.error_count()).sum()
    }
    /// Total warning count across all collectors.
    pub fn total_warnings(&self) -> usize {
        self.collectors.iter().map(|c| c.warning_count()).sum()
    }
    /// Total diagnostic count across all collectors.
    pub fn total_count(&self) -> usize {
        self.collectors.iter().map(|c| c.diagnostics().len()).sum()
    }
    /// Flatten all diagnostics into a single sorted (by position) list.
    pub fn flat_sorted(&self) -> Vec<Diagnostic> {
        let mut all: Vec<Diagnostic> = self
            .collectors
            .iter()
            .flat_map(|c| c.diagnostics().iter().cloned())
            .collect();
        all.sort_by(|a, b| {
            a.span
                .line
                .cmp(&b.span.line)
                .then(a.span.column.cmp(&b.span.column))
        });
        all
    }
    /// True if any collector has errors.
    pub fn has_errors(&self) -> bool {
        self.collectors.iter().any(|c| c.has_errors())
    }
    /// Summary line.
    pub fn summary(&self) -> String {
        format!(
            "DiagnosticAggregator [{}]: {} errors, {} warnings across {} collectors",
            self.label,
            self.total_errors(),
            self.total_warnings(),
            self.collectors.len()
        )
    }
    /// Number of collectors.
    pub fn collector_count(&self) -> usize {
        self.collectors.len()
    }
}
/// Suppresses diagnostics matching specific criteria before they are added.
#[allow(dead_code)]
pub struct DiagnosticSuppressor {
    suppressed_codes: Vec<DiagnosticCode>,
    suppress_warnings: bool,
    suppress_hints: bool,
}
#[allow(dead_code)]
impl DiagnosticSuppressor {
    /// Create a new suppressor with no rules.
    pub fn new() -> Self {
        Self {
            suppressed_codes: Vec::new(),
            suppress_warnings: false,
            suppress_hints: false,
        }
    }
    /// Suppress a specific diagnostic code.
    pub fn suppress_code(mut self, code: DiagnosticCode) -> Self {
        self.suppressed_codes.push(code);
        self
    }
    /// Suppress all warnings.
    pub fn suppress_all_warnings(mut self) -> Self {
        self.suppress_warnings = true;
        self
    }
    /// Suppress all hints.
    pub fn suppress_all_hints(mut self) -> Self {
        self.suppress_hints = true;
        self
    }
    /// Return `true` if the given diagnostic should be suppressed.
    pub fn should_suppress(&self, d: &Diagnostic) -> bool {
        if self.suppress_warnings && d.severity == Severity::Warning {
            return true;
        }
        if self.suppress_hints && d.severity == Severity::Hint {
            return true;
        }
        if let Some(code) = d.code {
            return self.suppressed_codes.contains(&code);
        }
        false
    }
    /// Filter a list of diagnostics, removing suppressed ones.
    pub fn filter(&self, diags: Vec<Diagnostic>) -> Vec<Diagnostic> {
        diags
            .into_iter()
            .filter(|d| !self.should_suppress(d))
            .collect()
    }
    /// Filter a collector, returning a new collector with unsuppressed diagnostics.
    pub fn filter_collector(&self, c: &DiagnosticCollector) -> DiagnosticCollector {
        let mut new_c = DiagnosticCollector::new();
        for d in c.diagnostics() {
            if !self.should_suppress(d) {
                new_c.add(d.clone());
            }
        }
        new_c
    }
}
/// Filters a `DiagnosticCollector` based on various criteria.
#[allow(dead_code)]
pub struct DiagnosticFilter<'a> {
    collector: &'a DiagnosticCollector,
}
#[allow(dead_code)]
impl<'a> DiagnosticFilter<'a> {
    /// Create a new filter wrapping the given collector.
    pub fn new(collector: &'a DiagnosticCollector) -> Self {
        Self { collector }
    }
    /// Diagnostics with the given code.
    pub fn with_code(&self, code: DiagnosticCode) -> Vec<&Diagnostic> {
        self.collector
            .diagnostics()
            .iter()
            .filter(|d| d.code == Some(code))
            .collect()
    }
    /// Diagnostics whose message contains the given substring.
    pub fn message_contains(&self, needle: &str) -> Vec<&Diagnostic> {
        self.collector
            .diagnostics()
            .iter()
            .filter(|d| d.message.contains(needle))
            .collect()
    }
    /// Diagnostics in a line range `[from_line, to_line]` (inclusive, 1-indexed).
    pub fn in_line_range(&self, from_line: usize, to_line: usize) -> Vec<&Diagnostic> {
        self.collector
            .diagnostics()
            .iter()
            .filter(|d| d.span.line >= from_line && d.span.line <= to_line)
            .collect()
    }
    /// Diagnostics that have at least one fix suggestion.
    pub fn with_fixes(&self) -> Vec<&Diagnostic> {
        self.collector
            .diagnostics()
            .iter()
            .filter(|d| !d.fixes.is_empty())
            .collect()
    }
    /// Diagnostics that have help text.
    pub fn with_help(&self) -> Vec<&Diagnostic> {
        self.collector
            .diagnostics()
            .iter()
            .filter(|d| d.help.is_some())
            .collect()
    }
    /// Diagnostics of severity Error.
    pub fn errors(&self) -> Vec<&Diagnostic> {
        self.collector.filter_severity(Severity::Error)
    }
    /// Diagnostics of severity Warning.
    pub fn warnings(&self) -> Vec<&Diagnostic> {
        self.collector.filter_severity(Severity::Warning)
    }
    /// Diagnostics of severity Info.
    pub fn infos(&self) -> Vec<&Diagnostic> {
        self.collector.filter_severity(Severity::Info)
    }
    /// Diagnostics of severity Hint.
    pub fn hints(&self) -> Vec<&Diagnostic> {
        self.collector.filter_severity(Severity::Hint)
    }
}
/// Diagnostic collector for gathering multiple diagnostics.
pub struct DiagnosticCollector {
    /// Collected diagnostics
    diagnostics: Vec<Diagnostic>,
    /// Error count
    error_count: usize,
    /// Warning count
    warning_count: usize,
}
impl DiagnosticCollector {
    /// Create a new diagnostic collector.
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
            error_count: 0,
            warning_count: 0,
        }
    }
    /// Add a diagnostic.
    pub fn add(&mut self, diagnostic: Diagnostic) {
        if diagnostic.is_error() {
            self.error_count += 1;
        } else if diagnostic.is_warning() {
            self.warning_count += 1;
        }
        self.diagnostics.push(diagnostic);
    }
    /// Get all diagnostics.
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
    /// Get error count.
    pub fn error_count(&self) -> usize {
        self.error_count
    }
    /// Get warning count.
    pub fn warning_count(&self) -> usize {
        self.warning_count
    }
    /// Check if there are any errors.
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }
    /// Clear all diagnostics.
    pub fn clear(&mut self) {
        self.diagnostics.clear();
        self.error_count = 0;
        self.warning_count = 0;
    }
    /// Get diagnostics at a specific line number.
    #[allow(dead_code)]
    pub fn diagnostics_at(&self, line: usize) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.span.line == line)
            .collect()
    }
    /// Count info-level diagnostics.
    #[allow(dead_code)]
    pub fn info_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == Severity::Info)
            .count()
    }
    /// Sort diagnostics by severity (errors first, then warnings, then info, then hints).
    #[allow(dead_code)]
    pub fn sort_by_severity(&mut self) {
        self.diagnostics.sort_by_key(|d| d.severity);
    }
    /// Sort diagnostics by source position (line, then column).
    #[allow(dead_code)]
    pub fn sort_by_position(&mut self) {
        self.diagnostics.sort_by(|a, b| {
            a.span
                .line
                .cmp(&b.span.line)
                .then(a.span.column.cmp(&b.span.column))
        });
    }
    /// Filter diagnostics by severity.
    #[allow(dead_code)]
    pub fn filter_severity(&self, severity: Severity) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == severity)
            .collect()
    }
    /// Merge another collector's diagnostics into this one.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &DiagnosticCollector) {
        for diag in &other.diagnostics {
            self.add(diag.clone());
        }
    }
    /// Generate a human-readable summary string.
    ///
    /// Example: "3 errors, 2 warnings"
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        let info = self.info_count();
        let mut parts = Vec::new();
        if self.error_count > 0 {
            parts.push(format!(
                "{} error{}",
                self.error_count,
                if self.error_count == 1 { "" } else { "s" }
            ));
        }
        if self.warning_count > 0 {
            parts.push(format!(
                "{} warning{}",
                self.warning_count,
                if self.warning_count == 1 { "" } else { "s" }
            ));
        }
        if info > 0 {
            parts.push(format!("{} info{}", info, if info == 1 { "" } else { "s" }));
        }
        if parts.is_empty() {
            "no diagnostics".to_string()
        } else {
            parts.join(", ")
        }
    }
}
/// A code fix suggestion.
#[derive(Debug, Clone)]
pub struct CodeFix {
    /// Human-readable description of the fix
    pub message: String,
    /// Span of code to replace
    pub span: Span,
    /// Replacement text
    pub replacement: String,
}
/// A named group of related diagnostics (e.g. from one file or one pass).
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct DiagnosticGroup {
    /// Group name or file path.
    pub name: String,
    /// Diagnostics in this group.
    pub diagnostics: Vec<Diagnostic>,
}
#[allow(dead_code)]
impl DiagnosticGroup {
    /// Create an empty group.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            diagnostics: Vec::new(),
        }
    }
    /// Add a diagnostic to the group.
    pub fn add(&mut self, d: Diagnostic) {
        self.diagnostics.push(d);
    }
    /// Error count.
    pub fn error_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.is_error()).count()
    }
    /// Warning count.
    pub fn warning_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.is_warning()).count()
    }
    /// True if there are any errors.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.is_error())
    }
    /// Total number of diagnostics.
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }
    /// True if there are no diagnostics.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
    /// Produce a summary string for this group.
    pub fn summary(&self) -> String {
        format!(
            "[{}]: {} errors, {} warnings",
            self.name,
            self.error_count(),
            self.warning_count()
        )
    }
    /// Sort the group's diagnostics by position.
    pub fn sort_by_position(&mut self) {
        self.diagnostics.sort_by(|a, b| {
            a.span
                .line
                .cmp(&b.span.line)
                .then(a.span.column.cmp(&b.span.column))
        });
    }
}
