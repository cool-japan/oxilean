//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::lsp::{analyze_document, Diagnostic, DiagnosticSeverity, Document, Range, TextEdit};
use oxilean_kernel::{Environment, Name};
use oxilean_parse::{Lexer, TokenKind};

use std::collections::HashMap;

/// Extended rich diagnostic with priority and fix count.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ExtendedDiagnostic {
    /// The base rich diagnostic.
    pub rich: RichDiagnostic,
    /// Priority for ordering.
    pub priority: DiagnosticPriority,
    /// Number of available quick fixes.
    pub fix_count: usize,
    /// Whether this diagnostic can be auto-fixed.
    pub auto_fixable: bool,
    /// Tags (e.g. "unnecessary", "deprecated").
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl ExtendedDiagnostic {
    /// Create from a rich diagnostic with default priority.
    pub fn new(rich: RichDiagnostic) -> Self {
        let priority = match rich.diagnostic.severity {
            DiagnosticSeverity::Error => DiagnosticPriority::High,
            DiagnosticSeverity::Warning => DiagnosticPriority::Normal,
            _ => DiagnosticPriority::Low,
        };
        Self {
            rich,
            priority,
            fix_count: 0,
            auto_fixable: false,
            tags: Vec::new(),
        }
    }
    /// Set the number of available fixes.
    pub fn with_fix_count(mut self, count: usize) -> Self {
        self.fix_count = count;
        self.auto_fixable = count > 0;
        self
    }
    /// Add a tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    /// Set priority.
    pub fn with_priority(mut self, priority: DiagnosticPriority) -> Self {
        self.priority = priority;
        self
    }
    /// Check if this diagnostic has a given tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A subscription to diagnostic updates.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DiagnosticSubscription {
    pub uri_pattern: Option<String>,
    pub min_severity: DiagnosticSeverity,
}
impl DiagnosticSubscription {
    /// Subscribe to all diagnostics.
    #[allow(dead_code)]
    pub fn all() -> Self {
        Self {
            uri_pattern: None,
            min_severity: DiagnosticSeverity::Hint,
        }
    }
    /// Subscribe to errors and warnings for a specific URI.
    #[allow(dead_code)]
    pub fn errors_and_warnings(uri: impl Into<String>) -> Self {
        Self {
            uri_pattern: Some(uri.into()),
            min_severity: DiagnosticSeverity::Warning,
        }
    }
    /// Check if a diagnostic matches this subscription.
    #[allow(dead_code)]
    pub fn matches(&self, uri: &str, diag: &Diagnostic) -> bool {
        if let Some(ref pattern) = self.uri_pattern {
            if !uri.contains(pattern.as_str()) {
                return false;
            }
        }
        let rank = |s: &DiagnosticSeverity| match s {
            DiagnosticSeverity::Error => 0,
            DiagnosticSeverity::Warning => 1,
            DiagnosticSeverity::Information => 2,
            DiagnosticSeverity::Hint => 3,
        };
        rank(&diag.severity) <= rank(&self.min_severity)
    }
}
/// Cache for diagnostics keyed by (uri, version).
#[allow(dead_code)]
pub struct DiagnosticCache {
    entries: std::collections::HashMap<String, (String, Vec<Diagnostic>)>,
    max_size: usize,
}
impl DiagnosticCache {
    /// Create a new cache.
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_size,
        }
    }
    fn key(uri: &str, version: &str) -> String {
        format!("{}:{}", uri, version)
    }
    /// Store diagnostics.
    #[allow(dead_code)]
    pub fn store(&mut self, uri: String, version: String, diags: Vec<Diagnostic>) {
        if self.entries.len() >= self.max_size {
            let first = self.entries.keys().next().cloned();
            if let Some(k) = first {
                self.entries.remove(&k);
            }
        }
        let k = Self::key(&uri, &version);
        self.entries.insert(k, (uri, diags));
    }
    /// Look up cached diagnostics by URI and version.
    #[allow(dead_code)]
    pub fn get(&self, uri: &str, version: &str) -> Option<&Vec<Diagnostic>> {
        let k = Self::key(uri, version);
        self.entries.get(&k).map(|(_, d)| d)
    }
    /// Invalidate cache for a URI.
    #[allow(dead_code)]
    pub fn invalidate_uri(&mut self, uri: &str) {
        self.entries.retain(|_, (u, _)| u != uri);
    }
}
/// A quick fix suggestion for a diagnostic.
#[derive(Clone, Debug)]
pub struct QuickFix {
    /// Title shown to the user.
    pub title: String,
    /// Text edits to apply.
    pub edits: Vec<TextEdit>,
    /// The diagnostic this fix addresses.
    pub diagnostic: Diagnostic,
}
/// A filter that decides which diagnostics to include.
#[derive(Clone, Debug, Default)]
pub struct DiagnosticFilter {
    /// If set, only include diagnostics at or above this severity.
    pub min_severity: Option<DiagnosticSeverity>,
    /// Codes to suppress (ignore).
    pub suppressed_codes: Vec<DiagnosticCode>,
    /// Maximum number of diagnostics to emit.
    pub max_count: Option<usize>,
}
impl DiagnosticFilter {
    /// Create a filter that accepts all diagnostics.
    pub fn accept_all() -> Self {
        Self::default()
    }
    /// Create a filter that only accepts errors.
    pub fn errors_only() -> Self {
        Self {
            min_severity: Some(DiagnosticSeverity::Error),
            ..Default::default()
        }
    }
    /// Suppress a specific code.
    pub fn suppress(mut self, code: DiagnosticCode) -> Self {
        self.suppressed_codes.push(code);
        self
    }
    /// Set a maximum count.
    pub fn limit(mut self, n: usize) -> Self {
        self.max_count = Some(n);
        self
    }
    /// Decide whether a rich diagnostic passes the filter.
    pub fn accepts(&self, d: &RichDiagnostic) -> bool {
        if self.suppressed_codes.contains(&d.code) {
            return false;
        }
        if let Some(min) = &self.min_severity {
            if &d.diagnostic.severity > min {
                return false;
            }
        }
        true
    }
    /// Apply the filter to a list of rich diagnostics.
    pub fn apply<'a>(&self, diagnostics: &'a [RichDiagnostic]) -> Vec<&'a RichDiagnostic> {
        let filtered: Vec<&'a RichDiagnostic> =
            diagnostics.iter().filter(|d| self.accepts(d)).collect();
        match self.max_count {
            Some(n) => filtered.into_iter().take(n).collect(),
            None => filtered,
        }
    }
}
/// A batch of rich diagnostics with summary stats.
#[derive(Clone, Debug, Default)]
pub struct DiagnosticBatch {
    /// All diagnostics.
    pub items: Vec<RichDiagnostic>,
}
impl DiagnosticBatch {
    /// Create an empty batch.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a diagnostic.
    pub fn add(&mut self, d: RichDiagnostic) {
        self.items.push(d);
    }
    /// Number of error-level diagnostics.
    pub fn error_count(&self) -> usize {
        self.items
            .iter()
            .filter(|d| d.diagnostic.severity == DiagnosticSeverity::Error)
            .count()
    }
    /// Number of warning-level diagnostics.
    pub fn warning_count(&self) -> usize {
        self.items
            .iter()
            .filter(|d| d.diagnostic.severity == DiagnosticSeverity::Warning)
            .count()
    }
    /// Whether there are any errors.
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }
    /// Apply a filter.
    pub fn filter(&self, f: &DiagnosticFilter) -> Vec<&RichDiagnostic> {
        f.apply(&self.items)
    }
    /// Total number of items.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Whether the batch is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
/// Priority of a diagnostic for display ordering.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticPriority {
    /// Lowest priority.
    Low = 0,
    /// Normal priority.
    Normal = 1,
    /// High priority (shown first).
    High = 2,
    /// Critical (blocking) priority.
    Critical = 3,
}
/// Aggregates diagnostic statistics across multiple files.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct DiagnosticAggregator {
    /// Per-URI error counts.
    errors: std::collections::HashMap<String, usize>,
    /// Per-URI warning counts.
    warnings: std::collections::HashMap<String, usize>,
    /// Total declarations checked.
    pub total_decls: usize,
}
#[allow(dead_code)]
impl DiagnosticAggregator {
    /// Create a new aggregator.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record diagnostics for a URI.
    pub fn record(&mut self, uri: &str, diagnostics: &[RichDiagnostic]) {
        let errors = diagnostics
            .iter()
            .filter(|d| d.diagnostic.severity == DiagnosticSeverity::Error)
            .count();
        let warnings = diagnostics
            .iter()
            .filter(|d| d.diagnostic.severity == DiagnosticSeverity::Warning)
            .count();
        *self.errors.entry(uri.to_string()).or_insert(0) += errors;
        *self.warnings.entry(uri.to_string()).or_insert(0) += warnings;
    }
    /// Total errors across all files.
    pub fn total_errors(&self) -> usize {
        self.errors.values().sum()
    }
    /// Total warnings across all files.
    pub fn total_warnings(&self) -> usize {
        self.warnings.values().sum()
    }
    /// Files with errors.
    pub fn files_with_errors(&self) -> Vec<&str> {
        self.errors
            .iter()
            .filter(|(_, &c)| c > 0)
            .map(|(uri, _)| uri.as_str())
            .collect()
    }
    /// Return the URI with the most errors.
    pub fn worst_file(&self) -> Option<(&str, usize)> {
        self.errors
            .iter()
            .max_by_key(|(_, &c)| c)
            .map(|(uri, &c)| (uri.as_str(), c))
    }
    /// Summary string.
    pub fn summary(&self) -> String {
        format!(
            "{} file(s), {} error(s), {} warning(s), {} declaration(s) checked",
            self.errors.len(),
            self.total_errors(),
            self.total_warnings(),
            self.total_decls
        )
    }
}
/// An enriched diagnostic with a code and optional related information.
#[derive(Clone, Debug)]
pub struct RichDiagnostic {
    /// The base diagnostic (range, severity, message).
    pub diagnostic: Diagnostic,
    /// The diagnostic code category.
    pub code: DiagnosticCode,
    /// Related information (context messages).
    pub related: Vec<RelatedInfo>,
}
/// Threshold settings for promotion/demotion of diagnostic severity.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DiagnosticThreshold {
    pub promote_info_to_warning: bool,
    pub promote_warnings_to_errors: bool,
    pub demote_errors_to_warnings: bool,
}
impl DiagnosticThreshold {
    /// Apply threshold to a severity.
    #[allow(dead_code)]
    pub fn apply(&self, severity: DiagnosticSeverity) -> DiagnosticSeverity {
        match severity {
            DiagnosticSeverity::Information if self.promote_info_to_warning => {
                DiagnosticSeverity::Warning
            }
            DiagnosticSeverity::Warning if self.promote_warnings_to_errors => {
                DiagnosticSeverity::Error
            }
            DiagnosticSeverity::Error if self.demote_errors_to_warnings => {
                DiagnosticSeverity::Warning
            }
            other => other,
        }
    }
}
/// Tracks which diagnostics are new or resolved between runs.
#[allow(dead_code)]
pub struct DiagnosticDiffTracker {
    previous: Vec<String>,
}
impl DiagnosticDiffTracker {
    /// Create a new tracker.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { previous: vec![] }
    }
    fn key(diag: &Diagnostic) -> String {
        format!(
            "{}:{}:{}",
            diag.range.start.line, diag.range.start.character, diag.message
        )
    }
    /// Update with a new set of diagnostics, returning (new, resolved) counts.
    #[allow(dead_code)]
    pub fn update(&mut self, current: &[Diagnostic]) -> (usize, usize) {
        let current_keys: std::collections::HashSet<String> =
            current.iter().map(Self::key).collect();
        let previous_keys: std::collections::HashSet<String> =
            self.previous.iter().cloned().collect();
        let new_count = current_keys.difference(&previous_keys).count();
        let resolved_count = previous_keys.difference(&current_keys).count();
        self.previous = current_keys.into_iter().collect();
        (new_count, resolved_count)
    }
}
/// Output format for diagnostics.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DiagnosticOutputFormat {
    Text,
    Json,
    Compact,
    Annotated,
}
/// Aggregates diagnostics across multiple files in a workspace.
#[allow(dead_code)]
pub struct DiagnosticWorkspaceAggregator {
    per_file: std::collections::HashMap<String, Vec<Diagnostic>>,
}
impl DiagnosticWorkspaceAggregator {
    /// Create a new aggregator.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            per_file: std::collections::HashMap::new(),
        }
    }
    /// Set diagnostics for a file.
    #[allow(dead_code)]
    pub fn set_for_file(&mut self, uri: String, diags: Vec<Diagnostic>) {
        self.per_file.insert(uri, diags);
    }
    /// Clear diagnostics for a file.
    #[allow(dead_code)]
    pub fn clear_file(&mut self, uri: &str) {
        self.per_file.remove(uri);
    }
    /// Return total error count across all files.
    #[allow(dead_code)]
    pub fn total_errors(&self) -> usize {
        self.per_file
            .values()
            .flat_map(|diags| diags.iter())
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Error))
            .count()
    }
    /// Return the file with the most diagnostics.
    #[allow(dead_code)]
    pub fn worst_file(&self) -> Option<&str> {
        self.per_file
            .iter()
            .max_by_key(|(_, diags)| diags.len())
            .map(|(uri, _)| uri.as_str())
    }
}
/// Limits the number of diagnostics reported per file.
#[allow(dead_code)]
pub struct DiagnosticBudget {
    pub max_errors: usize,
    pub max_warnings: usize,
    pub max_total: usize,
}
impl DiagnosticBudget {
    /// Trim diagnostics to fit within budget.
    #[allow(dead_code)]
    pub fn apply(&self, diagnostics: Vec<Diagnostic>) -> (Vec<Diagnostic>, usize) {
        let truncated = diagnostics.len().saturating_sub(self.max_total);
        let trimmed: Vec<Diagnostic> = diagnostics.into_iter().take(self.max_total).collect();
        (trimmed, truncated)
    }
}
/// Categorized diagnostic codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DiagnosticCode {
    /// Lexer error: invalid token.
    LexError,
    /// Parser error: unexpected token or syntax error.
    ParseError,
    /// Type error: type mismatch or unresolved type.
    TypeError,
    /// An unused variable was detected.
    UnusedVariable,
    /// A name shadows a previous binding.
    Shadowing,
    /// A deprecated feature or API was used.
    Deprecation,
    /// An unresolved name reference.
    UnresolvedName,
    /// Missing import for a used name.
    MissingImport,
    /// Redundant import or open statement.
    RedundantImport,
    /// Style warning (e.g. naming convention).
    StyleWarning,
}
impl DiagnosticCode {
    /// Return a string representation of the code.
    pub fn as_str(self) -> &'static str {
        match self {
            DiagnosticCode::LexError => "E001",
            DiagnosticCode::ParseError => "E002",
            DiagnosticCode::TypeError => "E003",
            DiagnosticCode::UnusedVariable => "W001",
            DiagnosticCode::Shadowing => "W002",
            DiagnosticCode::Deprecation => "W003",
            DiagnosticCode::UnresolvedName => "E004",
            DiagnosticCode::MissingImport => "E005",
            DiagnosticCode::RedundantImport => "W004",
            DiagnosticCode::StyleWarning => "W005",
        }
    }
    /// Return a human-readable name for the code.
    pub fn description(self) -> &'static str {
        match self {
            DiagnosticCode::LexError => "lexer error",
            DiagnosticCode::ParseError => "parse error",
            DiagnosticCode::TypeError => "type error",
            DiagnosticCode::UnusedVariable => "unused variable",
            DiagnosticCode::Shadowing => "shadowing",
            DiagnosticCode::Deprecation => "deprecation",
            DiagnosticCode::UnresolvedName => "unresolved name",
            DiagnosticCode::MissingImport => "missing import",
            DiagnosticCode::RedundantImport => "redundant import",
            DiagnosticCode::StyleWarning => "style warning",
        }
    }
}
/// The kind of code action (maps to LSP CodeActionKind).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CodeActionKind {
    /// A quick fix for a diagnostic.
    QuickFix,
    /// A refactoring action.
    Refactor,
    /// A source-level action (e.g. organize imports).
    Source,
    /// Extract a sub-expression into a new declaration.
    RefactorExtract,
    /// Inline a definition.
    RefactorInline,
}
impl CodeActionKind {
    /// Return the LSP string representation.
    pub fn as_str(&self) -> &str {
        match self {
            CodeActionKind::QuickFix => "quickfix",
            CodeActionKind::Refactor => "refactor",
            CodeActionKind::Source => "source",
            CodeActionKind::RefactorExtract => "refactor.extract",
            CodeActionKind::RefactorInline => "refactor.inline",
        }
    }
}
/// An inline annotation to show at the end of a source line.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct InlineAnnotation {
    /// Line number (0-indexed).
    pub line: u32,
    /// Message to display.
    pub message: String,
    /// Severity level.
    pub severity: DiagnosticSeverity,
}
/// Tracks diagnostic emission rates to avoid flooding the client.
#[allow(dead_code)]
pub struct DiagnosticRateLimiter {
    /// Messages emitted per URI.
    counts: std::collections::HashMap<String, usize>,
    /// Per-URI limits.
    limit: usize,
}
#[allow(dead_code)]
impl DiagnosticRateLimiter {
    /// Create a new rate limiter.
    pub fn new(limit: usize) -> Self {
        Self {
            counts: std::collections::HashMap::new(),
            limit,
        }
    }
    /// Check whether a new diagnostic for `uri` is allowed.
    pub fn allow(&mut self, uri: &str) -> bool {
        let count = self.counts.entry(uri.to_string()).or_insert(0);
        if *count < self.limit {
            *count += 1;
            true
        } else {
            false
        }
    }
    /// Reset the count for a URI (call when a document is saved).
    pub fn reset(&mut self, uri: &str) {
        self.counts.remove(uri);
    }
    /// Reset all counts.
    pub fn reset_all(&mut self) {
        self.counts.clear();
    }
    /// Return the current count for a URI.
    pub fn count_for(&self, uri: &str) -> usize {
        self.counts.get(uri).copied().unwrap_or(0)
    }
}
/// Adds extra context to diagnostics (e.g., related messages).
#[allow(dead_code)]
pub struct DiagnosticEnricher {
    pub add_source_snippets: bool,
    pub add_fix_hints: bool,
}
impl DiagnosticEnricher {
    /// Create a default enricher.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            add_source_snippets: true,
            add_fix_hints: true,
        }
    }
    /// Enrich a diagnostic message with extra context.
    #[allow(dead_code)]
    pub fn enrich_message(&self, code: DiagnosticCode, message: &str) -> String {
        let hint = if self.add_fix_hints {
            match code {
                DiagnosticCode::UnusedVariable => " [hint: prefix with _ to suppress]",
                DiagnosticCode::TypeError => " [hint: check type annotations]",
                DiagnosticCode::UnresolvedName => " [hint: check imports or spelling]",
                _ => "",
            }
        } else {
            ""
        };
        format!("{}{}", message, hint)
    }
}
/// A complete diagnostic report for a file.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct DiagnosticReport {
    /// URI of the file.
    pub uri: String,
    /// All diagnostic groups.
    pub groups: Vec<DiagnosticGroup>,
    /// Total errors across all groups.
    pub total_errors: usize,
    /// Total warnings across all groups.
    pub total_warnings: usize,
}
#[allow(dead_code)]
impl DiagnosticReport {
    /// Create a new empty report.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            groups: Vec::new(),
            total_errors: 0,
            total_warnings: 0,
        }
    }
    /// Add a group.
    pub fn add_group(&mut self, group: DiagnosticGroup) {
        self.total_errors += group.error_count();
        self.total_warnings += group.warning_count();
        self.groups.push(group);
    }
    /// Return all diagnostics flattened.
    pub fn all_diagnostics(&self) -> Vec<&RichDiagnostic> {
        self.groups.iter().flat_map(|g| g.items.iter()).collect()
    }
    /// Whether the report is clean (no errors or warnings).
    pub fn is_clean(&self) -> bool {
        self.total_errors == 0 && self.total_warnings == 0
    }
    /// Format a summary line.
    pub fn summary(&self) -> String {
        if self.is_clean() {
            return format!("{}: no issues", self.uri);
        }
        format!(
            "{}: {} error(s), {} warning(s)",
            self.uri, self.total_errors, self.total_warnings
        )
    }
}
/// A group of related diagnostics (e.g., all errors in one declaration).
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct DiagnosticGroup {
    /// Group label (e.g., declaration name).
    pub label: String,
    /// The diagnostics in this group.
    pub items: Vec<RichDiagnostic>,
}
#[allow(dead_code)]
impl DiagnosticGroup {
    /// Create a new group.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            items: Vec::new(),
        }
    }
    /// Add a diagnostic to the group.
    pub fn add(&mut self, d: RichDiagnostic) {
        self.items.push(d);
    }
    /// Count errors in this group.
    pub fn error_count(&self) -> usize {
        self.items
            .iter()
            .filter(|d| d.diagnostic.severity == DiagnosticSeverity::Error)
            .count()
    }
    /// Count warnings in this group.
    pub fn warning_count(&self) -> usize {
        self.items
            .iter()
            .filter(|d| d.diagnostic.severity == DiagnosticSeverity::Warning)
            .count()
    }
    /// Whether this group has any errors.
    pub fn has_errors(&self) -> bool {
        self.error_count() > 0
    }
    /// Total count.
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
/// A diagnostic processing pipeline.
#[allow(dead_code)]
pub struct DiagnosticPipeline {
    stages: Vec<DiagnosticPipelineStage>,
}
impl DiagnosticPipeline {
    /// Create a default pipeline.
    #[allow(dead_code)]
    pub fn default_pipeline() -> Self {
        Self {
            stages: vec![
                DiagnosticPipelineStage::new("collect"),
                DiagnosticPipelineStage::new("deduplicate"),
                DiagnosticPipelineStage::new("sort"),
                DiagnosticPipelineStage::new("suppress"),
                DiagnosticPipelineStage::new("enrich"),
                DiagnosticPipelineStage::new("publish"),
            ],
        }
    }
    /// Return stage names.
    #[allow(dead_code)]
    pub fn stage_names(&self) -> Vec<&str> {
        self.stages.iter().map(|s| s.name.as_str()).collect()
    }
    /// Return enabled stages.
    #[allow(dead_code)]
    pub fn enabled_stages(&self) -> Vec<&DiagnosticPipelineStage> {
        self.stages.iter().filter(|s| s.enabled).collect()
    }
}
/// A code action returned to the client.
#[derive(Clone, Debug)]
pub struct CodeAction {
    /// Title shown to the user.
    pub title: String,
    /// The kind of code action.
    pub kind: CodeActionKind,
    /// The diagnostics this action addresses.
    pub diagnostics: Vec<Diagnostic>,
    /// The workspace edit to apply.
    pub edit: Option<Vec<TextEdit>>,
}
/// A point-in-time snapshot of diagnostics for a file.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DiagnosticSnapshot {
    /// URI.
    pub uri: String,
    /// Snapshot of diagnostics.
    pub diagnostics: Vec<(String, String, u32)>,
}
#[allow(dead_code)]
impl DiagnosticSnapshot {
    /// Create a snapshot from rich diagnostics.
    pub fn capture(uri: &str, diagnostics: &[RichDiagnostic]) -> Self {
        let entries = diagnostics
            .iter()
            .map(|d| {
                (
                    d.code.as_str().to_string(),
                    d.diagnostic.message.clone(),
                    d.diagnostic.range.start.line,
                )
            })
            .collect();
        Self {
            uri: uri.to_string(),
            diagnostics: entries,
        }
    }
    /// Compare to another snapshot.
    pub fn diff(&self, other: &Self) -> Vec<String> {
        let mut diffs = Vec::new();
        let self_set: std::collections::HashSet<_> = self.diagnostics.iter().collect();
        let other_set: std::collections::HashSet<_> = other.diagnostics.iter().collect();
        for item in self_set.difference(&other_set) {
            diffs.push(format!(
                "- removed: [{}] {} at line {}",
                item.0, item.1, item.2
            ));
        }
        for item in other_set.difference(&self_set) {
            diffs.push(format!(
                "+ added:   [{}] {} at line {}",
                item.0, item.1, item.2
            ));
        }
        diffs
    }
    /// Count diagnostics by code.
    pub fn count_by_code(&self, code: &str) -> usize {
        self.diagnostics
            .iter()
            .filter(|(c, _, _)| c == code)
            .count()
    }
}
/// Related information attached to a diagnostic.
#[derive(Clone, Debug)]
pub struct RelatedInfo {
    /// A message about this related location.
    pub message: String,
    /// URI of the related document.
    pub uri: String,
    /// Range of the related location.
    pub range: Range,
}
/// Collects diagnostics from various analysis phases.
pub struct DiagnosticCollector<'a> {
    /// The kernel environment for checking names.
    env: &'a Environment,
    /// Maximum number of diagnostics to collect.
    max_diagnostics: usize,
}
impl<'a> DiagnosticCollector<'a> {
    /// Create a new diagnostic collector.
    pub fn new(env: &'a Environment, max_diagnostics: usize) -> Self {
        Self {
            env,
            max_diagnostics,
        }
    }
    /// Collect all diagnostics from a document.
    pub fn collect_diagnostics(&self, doc: &Document) -> Vec<RichDiagnostic> {
        let mut diagnostics = Vec::new();
        diagnostics.extend(self.collect_lex_errors(&doc.content));
        diagnostics.extend(self.collect_parse_errors(&doc.content));
        diagnostics.extend(self.collect_type_errors(doc));
        diagnostics.extend(self.collect_warnings(doc));
        diagnostics.truncate(self.max_diagnostics);
        diagnostics
    }
    /// Collect lexer-level errors.
    pub fn collect_lex_errors(&self, content: &str) -> Vec<RichDiagnostic> {
        let mut diagnostics = Vec::new();
        let mut lexer = Lexer::new(content);
        let tokens = lexer.tokenize();
        for token in &tokens {
            if let TokenKind::Error(msg) = &token.kind {
                let line = if token.span.line > 0 {
                    token.span.line as u32 - 1
                } else {
                    0
                };
                let col = if token.span.column > 0 {
                    token.span.column as u32 - 1
                } else {
                    0
                };
                diagnostics.push(RichDiagnostic {
                    diagnostic: Diagnostic::error(
                        Range::single_line(line, col, col + 1),
                        format!("lexer error: {}", msg),
                    ),
                    code: DiagnosticCode::LexError,
                    related: Vec::new(),
                });
            }
        }
        diagnostics
    }
    /// Collect parse-level errors by running the parser.
    pub fn collect_parse_errors(&self, content: &str) -> Vec<RichDiagnostic> {
        let mut diagnostics = Vec::new();
        let mut lexer = Lexer::new(content);
        let tokens = lexer.tokenize();
        let mut paren_stack: Vec<(char, u32, u32)> = Vec::new();
        for token in &tokens {
            let line = if token.span.line > 0 {
                token.span.line as u32 - 1
            } else {
                0
            };
            let col = if token.span.column > 0 {
                token.span.column as u32 - 1
            } else {
                0
            };
            match &token.kind {
                TokenKind::LParen => paren_stack.push(('(', line, col)),
                TokenKind::LBracket => paren_stack.push(('[', line, col)),
                TokenKind::LBrace => paren_stack.push(('{', line, col)),
                TokenKind::RParen => {
                    if let Some((ch, _, _)) = paren_stack.last() {
                        if *ch == '(' {
                            paren_stack.pop();
                        } else {
                            diagnostics.push(RichDiagnostic {
                                diagnostic: Diagnostic::error(
                                    Range::single_line(line, col, col + 1),
                                    format!(
                                        "mismatched ')'; expected '{}' to close",
                                        closing_for(*ch)
                                    ),
                                ),
                                code: DiagnosticCode::ParseError,
                                related: Vec::new(),
                            });
                        }
                    } else {
                        diagnostics.push(RichDiagnostic {
                            diagnostic: Diagnostic::error(
                                Range::single_line(line, col, col + 1),
                                "unmatched ')'".to_string(),
                            ),
                            code: DiagnosticCode::ParseError,
                            related: Vec::new(),
                        });
                    }
                }
                TokenKind::RBracket => {
                    if let Some((ch, _, _)) = paren_stack.last() {
                        if *ch == '[' {
                            paren_stack.pop();
                        } else {
                            diagnostics.push(RichDiagnostic {
                                diagnostic: Diagnostic::error(
                                    Range::single_line(line, col, col + 1),
                                    format!(
                                        "mismatched ']'; expected '{}' to close",
                                        closing_for(*ch)
                                    ),
                                ),
                                code: DiagnosticCode::ParseError,
                                related: Vec::new(),
                            });
                        }
                    } else {
                        diagnostics.push(RichDiagnostic {
                            diagnostic: Diagnostic::error(
                                Range::single_line(line, col, col + 1),
                                "unmatched ']'".to_string(),
                            ),
                            code: DiagnosticCode::ParseError,
                            related: Vec::new(),
                        });
                    }
                }
                TokenKind::RBrace => {
                    if let Some((ch, _, _)) = paren_stack.last() {
                        if *ch == '{' {
                            paren_stack.pop();
                        } else {
                            diagnostics.push(RichDiagnostic {
                                diagnostic: Diagnostic::error(
                                    Range::single_line(line, col, col + 1),
                                    format!(
                                        "mismatched '}}'; expected '{}' to close",
                                        closing_for(*ch)
                                    ),
                                ),
                                code: DiagnosticCode::ParseError,
                                related: Vec::new(),
                            });
                        }
                    } else {
                        diagnostics.push(RichDiagnostic {
                            diagnostic: Diagnostic::error(
                                Range::single_line(line, col, col + 1),
                                "unmatched '}'".to_string(),
                            ),
                            code: DiagnosticCode::ParseError,
                            related: Vec::new(),
                        });
                    }
                }
                _ => {}
            }
        }
        for (ch, line, col) in &paren_stack {
            diagnostics.push(RichDiagnostic {
                diagnostic: Diagnostic::error(
                    Range::single_line(*line, *col, *col + 1),
                    format!("unclosed '{}'", ch),
                ),
                code: DiagnosticCode::ParseError,
                related: Vec::new(),
            });
        }
        diagnostics
    }
    /// Collect type-level errors from analysis.
    pub fn collect_type_errors(&self, doc: &Document) -> Vec<RichDiagnostic> {
        let mut diagnostics = Vec::new();
        let analysis = analyze_document(&doc.uri, &doc.content, self.env);
        for diag in &analysis.diagnostics {
            let code = if diag.message.contains("shadows") {
                DiagnosticCode::Shadowing
            } else if diag.message.contains("lexer") {
                DiagnosticCode::LexError
            } else {
                DiagnosticCode::TypeError
            };
            diagnostics.push(RichDiagnostic {
                diagnostic: diag.clone(),
                code,
                related: Vec::new(),
            });
        }
        diagnostics
    }
    /// Collect warnings: unused variables, shadowing, deprecation, etc.
    pub fn collect_warnings(&self, doc: &Document) -> Vec<RichDiagnostic> {
        let mut diagnostics = Vec::new();
        let analysis = analyze_document(&doc.uri, &doc.content, self.env);
        let mut defined_names: Vec<(String, Range)> = Vec::new();
        for def in &analysis.definitions {
            defined_names.push((def.name.clone(), def.range.clone()));
        }
        let mut lexer = Lexer::new(&doc.content);
        let tokens = lexer.tokenize();
        let mut usage_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        for token in &tokens {
            if let TokenKind::Ident(name) = &token.kind {
                *usage_counts.entry(name.clone()).or_insert(0) += 1;
            }
        }
        for (name, range) in &defined_names {
            if !name.starts_with('_') {
                if let Some(&count) = usage_counts.get(name) {
                    if count <= 1 {
                        diagnostics.push(RichDiagnostic {
                            diagnostic: Diagnostic::warning(
                                range.clone(),
                                format!("unused variable '{}'", name),
                            ),
                            code: DiagnosticCode::UnusedVariable,
                            related: Vec::new(),
                        });
                    }
                }
            }
        }
        for def in &analysis.definitions {
            let kernel_name = Name::str(&def.name);
            if self.env.contains(&kernel_name) {
                diagnostics.push(RichDiagnostic {
                    diagnostic: Diagnostic::warning(
                        def.range.clone(),
                        format!("'{}' shadows existing declaration in environment", def.name),
                    ),
                    code: DiagnosticCode::Shadowing,
                    related: Vec::new(),
                });
            }
        }
        diagnostics
    }
}
/// A single stage in the diagnostic pipeline.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct DiagnosticPipelineStage {
    pub name: String,
    pub enabled: bool,
}
impl DiagnosticPipelineStage {
    /// Create an enabled stage.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            enabled: true,
        }
    }
}
