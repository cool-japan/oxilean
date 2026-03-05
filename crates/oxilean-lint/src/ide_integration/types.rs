//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::functions::MAX_LINE_LEN;

/// Filters diagnostics for a specific severity threshold or code prefix.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct DiagnosticFilter {
    /// If `Some`, only include diagnostics at or above this severity code.
    pub min_severity_code: Option<u8>,
    /// If non-empty, only include diagnostics whose code starts with one of these prefixes.
    pub code_prefixes: Vec<String>,
}
#[allow(dead_code)]
impl DiagnosticFilter {
    /// Create a pass-through filter (no restrictions).
    pub fn new() -> Self {
        Self::default()
    }
    /// Only include diagnostics with severity <= `code` (LSP: 1=Error … 4=Hint).
    pub fn with_min_severity(mut self, code: u8) -> Self {
        self.min_severity_code = Some(code);
        self
    }
    /// Only include diagnostics whose code starts with `prefix`.
    pub fn with_code_prefix(mut self, prefix: &str) -> Self {
        self.code_prefixes.push(prefix.to_string());
        self
    }
    /// Return true when a diagnostic passes this filter.
    pub fn accepts(&self, diag: &LintDiagnostic) -> bool {
        if let Some(min) = self.min_severity_code {
            if diag.severity.to_lsp_code() > min {
                return false;
            }
        }
        if !self.code_prefixes.is_empty() {
            let matched = self
                .code_prefixes
                .iter()
                .any(|p| diag.code.starts_with(p.as_str()));
            if !matched {
                return false;
            }
        }
        true
    }
    /// Apply this filter to a slice of diagnostics.
    pub fn apply<'a>(&self, diags: &'a [LintDiagnostic]) -> Vec<&'a LintDiagnostic> {
        diags.iter().filter(|d| self.accepts(d)).collect()
    }
}
/// A lint annotation found in a comment.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LintAnnotation {
    pub kind: LintAnnotationKind,
    pub line: usize,
    pub byte_offset: usize,
}
#[allow(dead_code)]
impl LintAnnotation {
    /// Create a new annotation.
    pub fn new(kind: LintAnnotationKind, line: usize, byte_offset: usize) -> Self {
        Self {
            kind,
            line,
            byte_offset,
        }
    }
    /// Whether this annotation suppresses `id`.
    pub fn suppresses(&self, id: &str) -> bool {
        matches!(& self.kind, LintAnnotationKind::Suppress(s) if s == id)
    }
    /// Whether this annotation expects `id`.
    pub fn expects(&self, id: &str) -> bool {
        matches!(& self.kind, LintAnnotationKind::Expect(s) if s == id)
    }
}
/// Provides hover documentation for a cursor position inside a file.
#[allow(dead_code)]
pub struct HoverProvider {
    pub docs: LintRuleDocumentationRegistry,
}
#[allow(dead_code)]
impl HoverProvider {
    /// Create a provider with built-in documentation.
    pub fn new() -> Self {
        Self {
            docs: LintRuleDocumentationRegistry::with_builtins(),
        }
    }
    /// Return hover Markdown for any diagnostic that covers `cursor_offset`.
    pub fn hover_at(&self, diags: &[LintDiagnostic], cursor_offset: usize) -> Option<String> {
        let diag = diags
            .iter()
            .find(|d| d.range_start <= cursor_offset && cursor_offset <= d.range_end)?;
        if let Some(doc) = self.docs.get(&diag.code) {
            Some(doc.to_markdown())
        } else {
            Some(format!("**{}** — {}", diag.code, diag.message))
        }
    }
}
/// Persistent lint session state for an IDE workspace.
///
/// Keeps diagnostics, document versions, and configuration together
/// so that repeated lint calls can be debounced properly.
#[allow(dead_code)]
pub struct LintSession {
    /// The underlying server.
    pub server: IdeLintServer,
    /// Document version tracker.
    pub versions: DocumentVersion,
    /// The configured debounce delay in milliseconds.
    pub debounce_ms: u64,
    /// Custom max-line-length threshold (overrides `MAX_LINE_LEN`).
    pub max_line_len: usize,
    /// Whether to report TODO comments.
    pub report_todos: bool,
    /// Whether to report FIXME comments.
    pub report_fixme: bool,
    /// Whether to report trailing whitespace.
    pub report_trailing_ws: bool,
    /// Whether to report HACK comments.
    pub report_hack: bool,
}
#[allow(dead_code)]
impl LintSession {
    /// Create a new session with default settings.
    pub fn new() -> Self {
        Self {
            server: IdeLintServer::new(),
            versions: DocumentVersion::new(),
            debounce_ms: 200,
            max_line_len: MAX_LINE_LEN,
            report_todos: true,
            report_fixme: true,
            report_trailing_ws: true,
            report_hack: false,
        }
    }
    /// Update session settings and return `self` (builder pattern).
    pub fn with_debounce(mut self, ms: u64) -> Self {
        self.debounce_ms = ms;
        self
    }
    /// Configure max line length.
    pub fn with_max_line_len(mut self, len: usize) -> Self {
        self.max_line_len = len;
        self
    }
    /// Run a lint pass on a file and return diagnostics.
    pub fn lint(&mut self, path: &str, source: &str) -> Vec<LintDiagnostic> {
        let _version = self.versions.bump(path);
        let diags = self.run_pass(source);
        self.server.update_file(path, source, diags);
        self.server
            .get_diagnostics(path)
            .iter()
            .map(|d| LintDiagnostic {
                code: d.code.clone(),
                message: d.message.clone(),
                severity: d.severity.clone(),
                range_start: d.range_start,
                range_end: d.range_end,
                source: d.source.clone(),
            })
            .collect()
    }
    /// Internal: run all configured checks on `source`.
    fn run_pass(&self, source: &str) -> Vec<LintDiagnostic> {
        let mut diags = Vec::new();
        let mut byte_offset: usize = 0;
        for line in source.lines() {
            let line_len = line.len();
            if line_len > self.max_line_len {
                diags.push(LintDiagnostic::new(
                    "long_line",
                    &format!(
                        "Line exceeds {} characters ({})",
                        self.max_line_len, line_len
                    ),
                    LintSeverity::Warning,
                    byte_offset,
                    byte_offset + line_len,
                ));
            }
            if self.report_todos && line.contains("TODO") {
                diags.push(LintDiagnostic::new(
                    "todo_comment",
                    "TODO comment found — consider filing a ticket",
                    LintSeverity::Info,
                    byte_offset,
                    byte_offset + line_len,
                ));
            }
            if self.report_fixme && line.contains("FIXME") {
                diags.push(LintDiagnostic::new(
                    "fixme_comment",
                    "FIXME comment found — issue needs attention",
                    LintSeverity::Warning,
                    byte_offset,
                    byte_offset + line_len,
                ));
            }
            if self.report_hack && line.contains("HACK") {
                diags.push(LintDiagnostic::new(
                    "hack_comment",
                    "HACK comment found — revisit this workaround",
                    LintSeverity::Warning,
                    byte_offset,
                    byte_offset + line_len,
                ));
            }
            if self.report_trailing_ws && (line.ends_with(' ') || line.ends_with('\t')) {
                diags.push(LintDiagnostic::new(
                    "trailing_whitespace",
                    "Trailing whitespace",
                    LintSeverity::Hint,
                    byte_offset + line_len.saturating_sub(1),
                    byte_offset + line_len,
                ));
            }
            byte_offset += line_len + 1;
        }
        diags
    }
    /// Return all files currently tracked in this session.
    pub fn tracked_files(&self) -> Vec<&str> {
        self.versions.tracked_paths()
    }
    /// Clear all state for a file.
    pub fn close_file(&mut self, path: &str) {
        self.server.clear_file(path);
        self.versions.reset(path);
    }
    /// Total diagnostic count across all files.
    pub fn total_diagnostic_count(&self) -> usize {
        self.server.diagnostics.values().map(|v| v.len()).sum()
    }
}
#[allow(dead_code)]
impl LintSession {
    pub(crate) fn session_diagnostics_for(&self, path: &str) -> &[LintDiagnostic] {
        self.server.get_diagnostics(path)
    }
}
/// Tracks document version numbers for incremental re-linting.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct DocumentVersion {
    versions: HashMap<String, u64>,
}
#[allow(dead_code)]
impl DocumentVersion {
    /// Create a new empty version tracker.
    pub fn new() -> Self {
        Self::default()
    }
    /// Increment version for a file; return new version number.
    pub fn bump(&mut self, path: &str) -> u64 {
        let v = self.versions.entry(path.to_string()).or_insert(0);
        *v += 1;
        *v
    }
    /// Get the current version for a file (0 if never seen).
    pub fn get(&self, path: &str) -> u64 {
        *self.versions.get(path).unwrap_or(&0)
    }
    /// Reset version to 0 for a file.
    pub fn reset(&mut self, path: &str) {
        self.versions.remove(path);
    }
    /// Return all tracked paths sorted.
    pub fn tracked_paths(&self) -> Vec<&str> {
        let mut paths: Vec<&str> = self.versions.keys().map(|s| s.as_str()).collect();
        paths.sort();
        paths
    }
}
/// A parsed lint annotation found in source comments.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LintAnnotationKind {
    /// `-- @[nolint <id>]` — suppress a specific lint.
    Suppress(String),
    /// `-- @[lint_note <text>]` — informational note.
    Note(String),
    /// `-- @[expect_lint <id>]` — expect this lint to fire.
    Expect(String),
}
/// A single IDE completion item.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionKind,
    pub detail: Option<String>,
    pub insert_text: String,
}
#[allow(dead_code)]
impl CompletionItem {
    /// Create a new completion item.
    pub fn new(label: &str, kind: CompletionKind, insert_text: &str) -> Self {
        Self {
            label: label.to_string(),
            kind,
            detail: None,
            insert_text: insert_text.to_string(),
        }
    }
    /// Attach a detail string.
    pub fn with_detail(mut self, detail: &str) -> Self {
        self.detail = Some(detail.to_string());
        self
    }
    /// Serialize to a JSON-compatible string.
    pub fn to_json(&self) -> String {
        let kind_code = match self.kind {
            CompletionKind::Suppression => 14u8,
            CompletionKind::Tactic => 3u8,
            CompletionKind::Keyword => 14u8,
        };
        let safe_label = self.label.replace('"', "\\\"");
        let safe_insert = self.insert_text.replace('"', "\\\"");
        let detail_part = if let Some(d) = &self.detail {
            format!(",\"detail\":\"{}\"", d.replace('"', "\\\""))
        } else {
            String::new()
        };
        format!(
            "{{\"label\":\"{safe_label}\",\"kind\":{kind_code},\
             \"insertText\":\"{safe_insert}\"{detail_part}}}"
        )
    }
}
/// The outcome of checking expected lints against actual diagnostics.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct ExpectationOutcome {
    /// Expectations that were satisfied (lint fired as expected).
    pub satisfied: Vec<String>,
    /// Expectations that were NOT satisfied (lint did NOT fire).
    pub unsatisfied: Vec<String>,
    /// Diagnostics that fired without a matching expectation.
    pub unexpected: Vec<String>,
}
/// Stores per-file diagnostics for an IDE session.
pub struct IdeLintServer {
    pub diagnostics: HashMap<String, Vec<LintDiagnostic>>,
}
impl IdeLintServer {
    pub fn new() -> Self {
        Self {
            diagnostics: HashMap::new(),
        }
    }
    /// Replace all diagnostics for `path` after a fresh lint run.
    pub fn update_file(&mut self, path: &str, _source: &str, diags: Vec<LintDiagnostic>) {
        self.diagnostics.insert(path.to_string(), diags);
    }
    /// Return all diagnostics for `path` (empty slice if not found).
    pub fn get_diagnostics(&self, path: &str) -> &[LintDiagnostic] {
        self.diagnostics
            .get(path)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Remove all diagnostics for `path`.
    pub fn clear_file(&mut self, path: &str) {
        self.diagnostics.remove(path);
    }
    /// Iterate over all (path, diagnostics) pairs.
    pub fn all_diagnostics(&self) -> Vec<(&str, &[LintDiagnostic])> {
        let mut out: Vec<(&str, &[LintDiagnostic])> = self
            .diagnostics
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_slice()))
            .collect();
        out.sort_by_key(|(k, _)| *k);
        out
    }
    /// Count total errors across all files.
    pub fn error_count(&self) -> usize {
        self.diagnostics
            .values()
            .flat_map(|v| v.iter())
            .filter(|d| d.severity == LintSeverity::Error)
            .count()
    }
    /// Count total warnings across all files.
    pub fn warning_count(&self) -> usize {
        self.diagnostics
            .values()
            .flat_map(|v| v.iter())
            .filter(|d| d.severity == LintSeverity::Warning)
            .count()
    }
}
/// Counts diagnostics per severity level for quick aggregation.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct SeverityCounter {
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
    pub hints: usize,
}
#[allow(dead_code)]
impl SeverityCounter {
    /// Create a zero counter.
    pub fn new() -> Self {
        Self::default()
    }
    /// Tally a single diagnostic.
    pub fn record(&mut self, diag: &LintDiagnostic) {
        match diag.severity {
            LintSeverity::Error => self.errors += 1,
            LintSeverity::Warning => self.warnings += 1,
            LintSeverity::Info => self.infos += 1,
            LintSeverity::Hint => self.hints += 1,
        }
    }
    /// Tally an entire slice.
    pub fn record_all(&mut self, diags: &[LintDiagnostic]) {
        for d in diags {
            self.record(d);
        }
    }
    /// Total count across all severities.
    pub fn total(&self) -> usize {
        self.errors + self.warnings + self.infos + self.hints
    }
    /// Whether any errors were recorded.
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }
    /// Whether the run was clean (no errors or warnings).
    pub fn is_clean(&self) -> bool {
        self.errors == 0 && self.warnings == 0
    }
    /// Merge another counter into this one.
    pub fn merge(&mut self, other: &SeverityCounter) {
        self.errors += other.errors;
        self.warnings += other.warnings;
        self.infos += other.infos;
        self.hints += other.hints;
    }
}
/// Documentation for a single lint rule, shown on hover in the IDE.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LintRuleDocumentation {
    /// The lint code (e.g., `"todo_comment"`).
    pub code: String,
    /// One-line summary.
    pub summary: String,
    /// Detailed explanation (Markdown).
    pub explanation: String,
    /// Example of bad code.
    pub bad_example: Option<String>,
    /// Example of good code.
    pub good_example: Option<String>,
    /// Link to online documentation.
    pub docs_url: Option<String>,
}
#[allow(dead_code)]
impl LintRuleDocumentation {
    /// Create minimal documentation.
    pub fn new(code: &str, summary: &str) -> Self {
        Self {
            code: code.to_string(),
            summary: summary.to_string(),
            explanation: String::new(),
            bad_example: None,
            good_example: None,
            docs_url: None,
        }
    }
    /// Attach a detailed explanation.
    pub fn with_explanation(mut self, text: &str) -> Self {
        self.explanation = text.to_string();
        self
    }
    /// Attach bad/good example pair.
    pub fn with_examples(mut self, bad: &str, good: &str) -> Self {
        self.bad_example = Some(bad.to_string());
        self.good_example = Some(good.to_string());
        self
    }
    /// Attach a docs URL.
    pub fn with_docs_url(mut self, url: &str) -> Self {
        self.docs_url = Some(url.to_string());
        self
    }
    /// Render as a Markdown hover string.
    pub fn to_markdown(&self) -> String {
        let mut md = format!("**{}** — {}\n\n", self.code, self.summary);
        if !self.explanation.is_empty() {
            md.push_str(&self.explanation);
            md.push_str("\n\n");
        }
        if let Some(bad) = &self.bad_example {
            md.push_str("**Bad:**\n```lean\n");
            md.push_str(bad);
            md.push_str("\n```\n\n");
        }
        if let Some(good) = &self.good_example {
            md.push_str("**Good:**\n```lean\n");
            md.push_str(good);
            md.push_str("\n```\n\n");
        }
        if let Some(url) = &self.docs_url {
            md.push_str(&format!("[Documentation]({})\n", url));
        }
        md
    }
}
/// A single inlay hint.
#[allow(dead_code)]
pub struct InlayHint {
    pub position: usize,
    pub label: String,
    pub kind: InlayHintKind,
}
/// Registry of documentation for all known lint rules.
#[allow(dead_code)]
#[derive(Default)]
pub struct LintRuleDocumentationRegistry {
    docs: HashMap<String, LintRuleDocumentation>,
}
#[allow(dead_code)]
impl LintRuleDocumentationRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a registry pre-populated with built-in rule documentation.
    pub fn with_builtins() -> Self {
        let mut reg = Self::new();
        reg.register(
            LintRuleDocumentation::new("long_line", "Line exceeds maximum length")
                .with_explanation("Lines should be at most 100 characters to improve readability.")
                .with_examples(
                    "-- this line is very long ".repeat(5).trim(),
                    "-- short line",
                )
                .with_docs_url("https://oxilean.org/lint/long_line"),
        );
        reg.register(
            LintRuleDocumentation::new("todo_comment", "TODO comment found")
                .with_explanation("TODO comments indicate unfinished work. Track them as tickets.")
                .with_docs_url("https://oxilean.org/lint/todo_comment"),
        );
        reg.register(
            LintRuleDocumentation::new("fixme_comment", "FIXME comment found")
                .with_explanation("FIXME comments indicate broken code that needs attention.")
                .with_docs_url("https://oxilean.org/lint/fixme_comment"),
        );
        reg.register(
            LintRuleDocumentation::new("trailing_whitespace", "Trailing whitespace on line")
                .with_explanation(
                    "Trailing whitespace creates noisy diffs. Configure your editor to strip it.",
                )
                .with_docs_url("https://oxilean.org/lint/trailing_whitespace"),
        );
        reg.register(
            LintRuleDocumentation::new("hack_comment", "HACK comment found")
                .with_explanation("HACK comments indicate workarounds that should be revisited.")
                .with_docs_url("https://oxilean.org/lint/hack_comment"),
        );
        reg
    }
    /// Register documentation for a rule.
    pub fn register(&mut self, doc: LintRuleDocumentation) {
        self.docs.insert(doc.code.clone(), doc);
    }
    /// Look up documentation by lint code.
    pub fn get(&self, code: &str) -> Option<&LintRuleDocumentation> {
        self.docs.get(code)
    }
    /// All registered codes, sorted.
    pub fn all_codes(&self) -> Vec<&str> {
        let mut codes: Vec<&str> = self.docs.keys().map(|s| s.as_str()).collect();
        codes.sort();
        codes
    }
    /// Number of registered rules.
    pub fn len(&self) -> usize {
        self.docs.len()
    }
    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.docs.is_empty()
    }
}
/// An LSP Code Action: a user-visible command to apply a fix.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LspCodeAction {
    /// Action title shown in the IDE.
    pub title: String,
    /// The diagnostic code this action addresses.
    pub diagnostic_code: String,
    /// The text replacement suggestion (new_text).
    pub replacement: String,
    /// Start offset of the replacement.
    pub range_start: usize,
    /// End offset of the replacement.
    pub range_end: usize,
    /// Whether this is a preferred (quick-fix) action.
    pub is_preferred: bool,
}
#[allow(dead_code)]
impl LspCodeAction {
    /// Create a new code action.
    pub fn new(
        title: &str,
        diagnostic_code: &str,
        replacement: &str,
        range_start: usize,
        range_end: usize,
    ) -> Self {
        Self {
            title: title.to_string(),
            diagnostic_code: diagnostic_code.to_string(),
            replacement: replacement.to_string(),
            range_start,
            range_end,
            is_preferred: false,
        }
    }
    /// Mark this action as a preferred quick-fix.
    pub fn preferred(mut self) -> Self {
        self.is_preferred = true;
        self
    }
    /// Serialize to a minimal JSON representation.
    pub fn to_json(&self) -> String {
        let safe_title = self.title.replace('"', "\\\"");
        let safe_repl = self.replacement.replace('"', "\\\"").replace('\n', "\\n");
        let safe_code = self.diagnostic_code.replace('"', "\\\"");
        format!(
            "{{\"title\":\"{safe_title}\",\
             \"diagnosticCode\":\"{safe_code}\",\
             \"edit\":{{\"range\":{{\"start\":{},\"end\":{}}},\
             \"newText\":\"{safe_repl}\"}},\
             \"isPreferred\":{}}}",
            self.range_start, self.range_end, self.is_preferred
        )
    }
}
/// Maps byte offsets to (line, column) pairs for efficient LSP position translation.
#[allow(dead_code)]
pub struct LineIndexer {
    /// Start byte offset of each line (line 0 at index 0).
    line_starts: Vec<usize>,
}
#[allow(dead_code)]
impl LineIndexer {
    /// Build an indexer from source text.
    pub fn new(source: &str) -> Self {
        let mut line_starts = vec![0usize];
        for (i, ch) in source.char_indices() {
            if ch == '\n' {
                line_starts.push(i + 1);
            }
        }
        Self { line_starts }
    }
    /// Return the 0-based line number for a byte offset.
    pub fn line(&self, offset: usize) -> usize {
        match self.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        }
    }
    /// Return the 0-based column (UTF-8 byte column) for a byte offset.
    pub fn column(&self, offset: usize) -> usize {
        let ln = self.line(offset);
        offset.saturating_sub(self.line_starts[ln])
    }
    /// Convert (line, col) back to a byte offset.
    pub fn offset(&self, line: usize, col: usize) -> usize {
        self.line_starts
            .get(line)
            .copied()
            .unwrap_or(0)
            .saturating_add(col)
    }
    /// Number of lines in the indexed source.
    pub fn num_lines(&self) -> usize {
        self.line_starts.len()
    }
    /// Return (line, column) pair for a byte offset.
    pub fn line_col(&self, offset: usize) -> (usize, usize) {
        (self.line(offset), self.column(offset))
    }
}
/// Cache that remembers file content hashes to skip re-linting unchanged files.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct FileChangeCache {
    hashes: HashMap<String, u64>,
}
#[allow(dead_code)]
impl FileChangeCache {
    /// Create an empty cache.
    pub fn new() -> Self {
        Self::default()
    }
    /// Return `true` when the content is unchanged (same hash as last seen).
    pub fn is_unchanged(&mut self, path: &str, source: &str) -> bool {
        let h = FileHasher::hash(source);
        if let Some(prev) = self.hashes.get(path) {
            if *prev == h {
                return true;
            }
        }
        self.hashes.insert(path.to_string(), h);
        false
    }
    /// Invalidate a cached entry.
    pub fn invalidate(&mut self, path: &str) {
        self.hashes.remove(path);
    }
    /// Number of cached entries.
    pub fn len(&self) -> usize {
        self.hashes.len()
    }
    /// Whether the cache is empty.
    pub fn is_empty(&self) -> bool {
        self.hashes.is_empty()
    }
}
/// Renders a human-readable summary of a lint run.
#[allow(dead_code)]
pub struct LintSummaryFormatter;
#[allow(dead_code)]
impl LintSummaryFormatter {
    /// Format a brief summary line.
    pub fn format_brief(counter: &SeverityCounter) -> String {
        if counter.total() == 0 {
            return "No issues found.".to_string();
        }
        let mut parts = Vec::new();
        if counter.errors > 0 {
            parts.push(format!("{} error(s)", counter.errors));
        }
        if counter.warnings > 0 {
            parts.push(format!("{} warning(s)", counter.warnings));
        }
        if counter.infos > 0 {
            parts.push(format!("{} info(s)", counter.infos));
        }
        if counter.hints > 0 {
            parts.push(format!("{} hint(s)", counter.hints));
        }
        parts.join(", ") + " found."
    }
    /// Format diagnostics as a GitHub Actions annotation block.
    pub fn format_github_actions(path: &str, diags: &[LintDiagnostic]) -> String {
        let mut out = String::new();
        for d in diags {
            let level = match d.severity {
                LintSeverity::Error => "error",
                LintSeverity::Warning => "warning",
                LintSeverity::Info | LintSeverity::Hint => "notice",
            };
            let safe_msg = d.message.replace('\n', "%0A");
            out.push_str(&format!(
                "::{level} file={path},col={col}::{safe_msg}\n",
                col = d.range_start
            ));
        }
        out
    }
    /// Serialize all diagnostics to a JSON array string.
    pub fn format_json(diags: &[LintDiagnostic]) -> String {
        let items: Vec<String> = diags.iter().map(|d| d.to_json()).collect();
        format!("[{}]", items.join(","))
    }
}
/// Checks that `@[expect_lint ...]` annotations are satisfied.
#[allow(dead_code)]
pub struct ExpectationChecker;
#[allow(dead_code)]
impl ExpectationChecker {
    /// Check annotations against actual diagnostics.
    pub fn check(annotations: &[LintAnnotation], diags: &[LintDiagnostic]) -> ExpectationOutcome {
        let expected: Vec<String> = annotations
            .iter()
            .filter_map(|a| {
                if let LintAnnotationKind::Expect(id) = &a.kind {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect();
        let actual: Vec<String> = diags.iter().map(|d| d.code.clone()).collect();
        let mut satisfied = Vec::new();
        let mut unsatisfied = Vec::new();
        for exp in &expected {
            if actual.contains(exp) {
                satisfied.push(exp.clone());
            } else {
                unsatisfied.push(exp.clone());
            }
        }
        let mut unexpected = Vec::new();
        for act in &actual {
            if !expected.contains(act) {
                unexpected.push(act.clone());
            }
        }
        ExpectationOutcome {
            satisfied,
            unsatisfied,
            unexpected,
        }
    }
}
/// A real-time linter that maintains an `IdeLintServer` and runs style checks.
pub struct RealTimeLinter {
    pub server: IdeLintServer,
    pub debounce_ms: u64,
}
impl RealTimeLinter {
    pub fn new(debounce_ms: u64) -> Self {
        Self {
            server: IdeLintServer::new(),
            debounce_ms,
        }
    }
    /// Run basic style checks on `source`, store results, and return them.
    pub fn lint_source(&mut self, path: &str, source: &str) -> Vec<LintDiagnostic> {
        let diags = Self::run_lint_pass(source);
        self.server.update_file(path, source, diags);
        self.server
            .get_diagnostics(path)
            .iter()
            .map(|d| LintDiagnostic {
                code: d.code.clone(),
                message: d.message.clone(),
                severity: d.severity.clone(),
                range_start: d.range_start,
                range_end: d.range_end,
                source: d.source.clone(),
            })
            .collect()
    }
    /// Run a single lint pass over `source` and return diagnostics.
    ///
    /// Checks performed:
    /// - Lines exceeding `MAX_LINE_LEN` characters.
    /// - Lines containing `TODO` comments.
    /// - Lines containing `FIXME` comments.
    /// - Trailing whitespace.
    pub fn run_lint_pass(source: &str) -> Vec<LintDiagnostic> {
        let mut diags = Vec::new();
        let mut byte_offset: usize = 0;
        for line in source.lines() {
            let line_len = line.len();
            if line_len > MAX_LINE_LEN {
                diags.push(LintDiagnostic::new(
                    "long_line",
                    &format!("Line exceeds {MAX_LINE_LEN} characters ({line_len})"),
                    LintSeverity::Warning,
                    byte_offset,
                    byte_offset + line_len,
                ));
            }
            if line.contains("TODO") {
                diags.push(LintDiagnostic::new(
                    "todo_comment",
                    "TODO comment found — consider filing a ticket",
                    LintSeverity::Info,
                    byte_offset,
                    byte_offset + line_len,
                ));
            }
            if line.contains("FIXME") {
                diags.push(LintDiagnostic::new(
                    "fixme_comment",
                    "FIXME comment found — issue needs attention",
                    LintSeverity::Warning,
                    byte_offset,
                    byte_offset + line_len,
                ));
            }
            if line.ends_with(' ') || line.ends_with('\t') {
                diags.push(LintDiagnostic::new(
                    "trailing_whitespace",
                    "Trailing whitespace",
                    LintSeverity::Hint,
                    byte_offset + line_len.saturating_sub(1),
                    byte_offset + line_len,
                ));
            }
            byte_offset += line_len + 1;
        }
        diags
    }
}
/// Parser for `LintAnnotation` values from source text.
#[allow(dead_code)]
pub struct LintAnnotationParser;
#[allow(dead_code)]
impl LintAnnotationParser {
    /// Scan `source` and return all annotations.
    pub fn parse(source: &str) -> Vec<LintAnnotation> {
        let mut annotations = Vec::new();
        let mut byte_offset = 0usize;
        for (line_idx, line) in source.lines().enumerate() {
            let trimmed = line.trim();
            if let Some(after_dashes) = trimmed.strip_prefix("--") {
                let comment = after_dashes.trim();
                if let Some(inner) = comment
                    .strip_prefix("@[nolint ")
                    .and_then(|s| s.strip_suffix(']'))
                {
                    annotations.push(LintAnnotation::new(
                        LintAnnotationKind::Suppress(inner.trim().to_string()),
                        line_idx,
                        byte_offset,
                    ));
                } else if let Some(inner) = comment
                    .strip_prefix("@[lint_note ")
                    .and_then(|s| s.strip_suffix(']'))
                {
                    annotations.push(LintAnnotation::new(
                        LintAnnotationKind::Note(inner.trim().to_string()),
                        line_idx,
                        byte_offset,
                    ));
                } else if let Some(inner) = comment
                    .strip_prefix("@[expect_lint ")
                    .and_then(|s| s.strip_suffix(']'))
                {
                    annotations.push(LintAnnotation::new(
                        LintAnnotationKind::Expect(inner.trim().to_string()),
                        line_idx,
                        byte_offset,
                    ));
                }
            }
            byte_offset += line.len() + 1;
        }
        annotations
    }
}
/// Provides completion items for lint annotation directives.
#[allow(dead_code)]
pub struct LintCompletionProvider {
    registry: LintRuleDocumentationRegistry,
}
#[allow(dead_code)]
impl LintCompletionProvider {
    /// Create a provider with built-in completions.
    pub fn new() -> Self {
        Self {
            registry: LintRuleDocumentationRegistry::with_builtins(),
        }
    }
    /// Return completions relevant at a given cursor position.
    ///
    /// Currently returns all nolint suppressions.
    pub fn completions_at(&self, _cursor_offset: usize) -> Vec<CompletionItem> {
        self.registry
            .all_codes()
            .into_iter()
            .map(|code| {
                CompletionItem::new(
                    &format!("@[nolint {code}]"),
                    CompletionKind::Suppression,
                    &format!("@[nolint {code}]"),
                )
                .with_detail("Suppress lint rule")
            })
            .collect()
    }
}
/// A searchable index of lint rules indexed by their code.
#[allow(dead_code)]
#[derive(Default)]
pub struct LintRulesIndex {
    entries: Vec<(String, String)>,
}
#[allow(dead_code)]
impl LintRulesIndex {
    /// Create an empty index.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a rule to the index.
    pub fn add(&mut self, code: &str, summary: &str) {
        self.entries.push((code.to_string(), summary.to_string()));
    }
    /// Search for rules whose code or summary contains `query` (case-insensitive).
    pub fn search(&self, query: &str) -> Vec<(&str, &str)> {
        let q = query.to_lowercase();
        self.entries
            .iter()
            .filter(|(code, summary)| {
                code.to_lowercase().contains(&q) || summary.to_lowercase().contains(&q)
            })
            .map(|(c, s)| (c.as_str(), s.as_str()))
            .collect()
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the index is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// Sorts a list of `LintDiagnostic` values.
#[allow(dead_code)]
pub struct DiagnosticSorter;
#[allow(dead_code)]
impl DiagnosticSorter {
    /// Sort `diags` in place using the given key.
    pub fn sort(diags: &mut Vec<LintDiagnostic>, key: SortKey) {
        match key {
            SortKey::Offset => {
                diags.sort_by_key(|d| (d.range_start, d.severity.to_lsp_code()));
            }
            SortKey::Severity => {
                diags.sort_by_key(|d| (d.severity.to_lsp_code(), d.range_start));
            }
            SortKey::Code => {
                diags.sort_by(|a, b| a.code.cmp(&b.code).then(a.range_start.cmp(&b.range_start)));
            }
        }
    }
    /// Return a sorted copy.
    pub fn sorted(mut diags: Vec<LintDiagnostic>, key: SortKey) -> Vec<LintDiagnostic> {
        Self::sort(&mut diags, key);
        diags
    }
}
/// Formats lint rule documentation for hover tooltips in the IDE.
#[allow(dead_code)]
pub struct LintHoverDocFormatter;
impl LintHoverDocFormatter {
    /// Format a short one-line tooltip for a lint rule.
    #[allow(dead_code)]
    pub fn short_tooltip(rule_id: &str, description: &str) -> String {
        format!("**{}**: {}", rule_id, description)
    }
    /// Format a full markdown documentation block for a lint rule.
    #[allow(dead_code)]
    pub fn full_doc(
        rule_id: &str,
        description: &str,
        rationale: &str,
        example: Option<(&str, &str)>,
    ) -> String {
        let mut lines = vec![
            format!("## Lint Rule: `{}`", rule_id),
            String::new(),
            description.to_string(),
            String::new(),
            "### Rationale".to_string(),
            rationale.to_string(),
        ];
        if let Some((bad, good)) = example {
            lines.push(String::new());
            lines.push("### Example".to_string());
            lines.push("**Bad:**".to_string());
            lines.push(format!("```\n{}\n```", bad));
            lines.push("**Good:**".to_string());
            lines.push(format!("```\n{}\n```", good));
        }
        lines.join("\n")
    }
}
/// A single lint diagnostic emitted for an IDE client.
pub struct LintDiagnostic {
    pub code: String,
    pub message: String,
    pub severity: LintSeverity,
    pub range_start: usize,
    pub range_end: usize,
    pub source: String,
}
impl LintDiagnostic {
    pub fn new(code: &str, msg: &str, severity: LintSeverity, start: usize, end: usize) -> Self {
        Self {
            code: code.to_string(),
            message: msg.to_string(),
            severity,
            range_start: start,
            range_end: end,
            source: "oxilean-lint".to_string(),
        }
    }
    /// Serialize to a minimal JSON string (no external deps).
    pub fn to_json(&self) -> String {
        let sev_code = self.severity.to_lsp_code();
        let safe_msg = self.message.replace('"', "\\\"");
        let safe_code = self.code.replace('"', "\\\"");
        let safe_src = self.source.replace('"', "\\\"");
        format!(
            "{{\"code\":\"{safe_code}\",\"message\":\"{safe_msg}\",\
             \"severity\":{sev_code},\"range\":{{\"start\":{},\"end\":{}}},\
             \"source\":\"{safe_src}\"}}",
            self.range_start, self.range_end
        )
    }
}
/// An IDE lint server that skips re-linting files whose content has not changed.
#[allow(dead_code)]
pub struct IncrementalLintServer {
    pub inner: IdeLintServer,
    pub cache: FileChangeCache,
}
#[allow(dead_code)]
impl IncrementalLintServer {
    /// Create a new incremental server.
    pub fn new() -> Self {
        Self {
            inner: IdeLintServer::new(),
            cache: FileChangeCache::new(),
        }
    }
    /// Lint `source` for `path`, skipping if content is unchanged.
    ///
    /// Returns diagnostics from the stored result.
    pub fn lint_if_changed(&mut self, path: &str, source: &str) -> &[LintDiagnostic] {
        if !self.cache.is_unchanged(path, source) {
            let diags = RealTimeLinter::run_lint_pass(source);
            self.inner.update_file(path, source, diags);
        }
        self.inner.get_diagnostics(path)
    }
    /// Clear a file from both the cache and the server.
    pub fn close_file(&mut self, path: &str) {
        self.inner.clear_file(path);
        self.cache.invalidate(path);
    }
    /// Total errors across all tracked files.
    pub fn error_count(&self) -> usize {
        self.inner.error_count()
    }
    /// Total warnings across all tracked files.
    pub fn warning_count(&self) -> usize {
        self.inner.warning_count()
    }
}
/// Generates LSP code actions from `LintDiagnostic` entries.
#[allow(dead_code)]
pub struct LspCodeActionProvider;
#[allow(dead_code)]
impl LspCodeActionProvider {
    /// Create built-in code actions for a set of diagnostics.
    pub fn code_actions_for(diags: &[LintDiagnostic]) -> Vec<LspCodeAction> {
        let mut actions = Vec::new();
        for diag in diags {
            match diag.code.as_str() {
                "trailing_whitespace" => {
                    actions.push(
                        LspCodeAction::new(
                            "Remove trailing whitespace",
                            &diag.code,
                            "",
                            diag.range_start,
                            diag.range_end,
                        )
                        .preferred(),
                    );
                }
                "todo_comment" => {
                    actions.push(LspCodeAction::new(
                        "Acknowledge TODO (no-op)",
                        &diag.code,
                        "",
                        diag.range_start,
                        diag.range_start,
                    ));
                }
                "fixme_comment" => {
                    actions.push(LspCodeAction::new(
                        "Acknowledge FIXME (no-op)",
                        &diag.code,
                        "",
                        diag.range_start,
                        diag.range_start,
                    ));
                }
                _ => {}
            }
        }
        actions
    }
}
/// Sorting strategies for diagnostics.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortKey {
    /// Sort by offset (primary) then severity.
    Offset,
    /// Sort by severity (Error first) then offset.
    Severity,
    /// Sort by code alphabetically.
    Code,
}
/// Computes a simple 64-bit hash of a file's content for change detection.
///
/// Uses the FNV-1a algorithm for speed and simplicity.
#[allow(dead_code)]
pub struct FileHasher;
#[allow(dead_code)]
impl FileHasher {
    /// Compute FNV-1a hash over `data`.
    pub fn hash(data: &str) -> u64 {
        let mut h: u64 = 14695981039346656037;
        for byte in data.bytes() {
            h ^= byte as u64;
            h = h.wrapping_mul(1099511628211);
        }
        h
    }
}
/// A summary of a workspace-wide lint scan.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct WorkspaceScanResult {
    /// Number of files scanned.
    pub files_scanned: usize,
    /// Total diagnostics emitted.
    pub total_diagnostics: usize,
    /// Total errors.
    pub total_errors: usize,
    /// Total warnings.
    pub total_warnings: usize,
    /// Files with at least one error.
    pub files_with_errors: Vec<String>,
}
/// Kind of completion item.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompletionKind {
    /// A lint suppression directive.
    Suppression,
    /// A tactic name.
    Tactic,
    /// A keyword.
    Keyword,
}
/// The kind of inlay hint.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InlayHintKind {
    TypeAnnotation,
    ParameterName,
    LintWarning,
    AutoFixAvailable,
}
/// Provides inlay hints for lint-related information.
#[allow(dead_code)]
pub struct InlayHintProvider;
impl InlayHintProvider {
    /// Generate inlay hints indicating that auto-fixes are available at certain positions.
    #[allow(dead_code)]
    pub fn autofix_hints(fix_positions: &[usize]) -> Vec<InlayHint> {
        fix_positions
            .iter()
            .map(|&pos| InlayHint {
                position: pos,
                label: "💡 fix available".to_string(),
                kind: InlayHintKind::AutoFixAvailable,
            })
            .collect()
    }
    /// Filter hints to only those matching a specific kind.
    #[allow(dead_code)]
    pub fn filter_by_kind(hints: &[InlayHint], kind: InlayHintKind) -> Vec<&InlayHint> {
        hints.iter().filter(|h| h.kind == kind).collect()
    }
}
/// Removes duplicate diagnostics from a list (same code + range).
#[allow(dead_code)]
pub struct DiagnosticDeduplifier;
#[allow(dead_code)]
impl DiagnosticDeduplifier {
    /// Deduplicate, keeping the first occurrence of each (code, range_start, range_end).
    pub fn dedup(diags: Vec<LintDiagnostic>) -> Vec<LintDiagnostic> {
        let mut seen: std::collections::HashSet<(String, usize, usize)> =
            std::collections::HashSet::new();
        diags
            .into_iter()
            .filter(|d| seen.insert((d.code.clone(), d.range_start, d.range_end)))
            .collect()
    }
    /// Count how many duplicates would be removed.
    pub fn duplicate_count(diags: &[LintDiagnostic]) -> usize {
        let total = diags.len();
        let unique: std::collections::HashSet<(String, usize, usize)> = diags
            .iter()
            .map(|d| (d.code.clone(), d.range_start, d.range_end))
            .collect();
        total.saturating_sub(unique.len())
    }
}
/// Severity level for IDE diagnostics (maps to LSP DiagnosticSeverity).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LintSeverity {
    Error,
    Warning,
    Info,
    Hint,
}
impl LintSeverity {
    /// Return the LSP DiagnosticSeverity integer code.
    pub fn to_lsp_code(&self) -> u8 {
        match self {
            LintSeverity::Error => 1,
            LintSeverity::Warning => 2,
            LintSeverity::Info => 3,
            LintSeverity::Hint => 4,
        }
    }
}
/// A budget that caps the number of diagnostics reported per lint run.
#[allow(dead_code)]
pub struct LintBudget {
    /// Maximum number of diagnostics to accept.
    max: usize,
    /// Current count.
    count: usize,
}
#[allow(dead_code)]
impl LintBudget {
    /// Create a new budget.
    pub fn new(max: usize) -> Self {
        Self { max, count: 0 }
    }
    /// Try to consume one slot; return `true` if still within budget.
    pub fn consume(&mut self) -> bool {
        if self.count < self.max {
            self.count += 1;
            true
        } else {
            false
        }
    }
    /// Remaining slots.
    pub fn remaining(&self) -> usize {
        self.max.saturating_sub(self.count)
    }
    /// Whether the budget is exhausted.
    pub fn exhausted(&self) -> bool {
        self.count >= self.max
    }
    /// Reset the budget counter.
    pub fn reset(&mut self) {
        self.count = 0;
    }
    /// Apply the budget to a list of diagnostics, returning at most `max` entries.
    pub fn apply(&self, diags: Vec<LintDiagnostic>) -> Vec<LintDiagnostic> {
        diags.into_iter().take(self.max).collect()
    }
}
/// Runs real-time lint over a set of (path, source) pairs.
#[allow(dead_code)]
pub struct WorkspaceScanner {
    pub session: LintSession,
}
#[allow(dead_code)]
impl WorkspaceScanner {
    /// Create a new scanner.
    pub fn new() -> Self {
        Self {
            session: LintSession::new(),
        }
    }
    /// Scan all provided sources and return a result summary.
    pub fn scan_all(&mut self, files: &[(&str, &str)]) -> WorkspaceScanResult {
        let mut result = WorkspaceScanResult::default();
        for (path, source) in files {
            let diags = self.session.lint(path, source);
            let errors: Vec<_> = diags
                .iter()
                .filter(|d| d.severity == LintSeverity::Error)
                .collect();
            result.files_scanned += 1;
            result.total_diagnostics += diags.len();
            result.total_errors += errors.len();
            result.total_warnings += diags
                .iter()
                .filter(|d| d.severity == LintSeverity::Warning)
                .count();
            if !errors.is_empty() {
                result.files_with_errors.push(path.to_string());
            }
        }
        result
    }
    /// Return all diagnostics for a specific file.
    pub fn diagnostics_for(&self, path: &str) -> &[LintDiagnostic] {
        self.session.server.get_diagnostics(path)
    }
}
