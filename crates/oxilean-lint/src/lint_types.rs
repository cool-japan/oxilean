//! Core lint types: categories, metadata, stats, reports, profiles.

use std::fmt;

#[allow(unused_imports)]
use super::{LintConfig, LintDiagnostic, LintId, LintPass, Severity};

// ============================================================
// LintCategory: grouping lint rules
// ============================================================

/// Category of a lint rule.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum LintCategory {
    /// Correctness: potential bugs.
    Correctness,
    /// Style: cosmetic/formatting issues.
    Style,
    /// Performance: potential inefficiencies.
    Performance,
    /// Complexity: overly complex code.
    Complexity,
    /// Deprecation: use of deprecated APIs.
    Deprecation,
    /// Documentation: missing or incorrect docs.
    Documentation,
    /// Naming: naming convention violations.
    Naming,
    /// Redundancy: redundant or dead code.
    Redundancy,
    /// Security: potential security issues.
    Security,
    /// Custom: user-defined lint category.
    Custom(String),
}

impl fmt::Display for LintCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LintCategory::Correctness => write!(f, "correctness"),
            LintCategory::Style => write!(f, "style"),
            LintCategory::Performance => write!(f, "performance"),
            LintCategory::Complexity => write!(f, "complexity"),
            LintCategory::Deprecation => write!(f, "deprecation"),
            LintCategory::Documentation => write!(f, "documentation"),
            LintCategory::Naming => write!(f, "naming"),
            LintCategory::Redundancy => write!(f, "redundancy"),
            LintCategory::Security => write!(f, "security"),
            LintCategory::Custom(ref name) => write!(f, "custom:{}", name),
        }
    }
}

// ============================================================
// LintMetadata: rule metadata record
// ============================================================

/// Metadata about a lint rule.
#[derive(Clone, Debug)]
pub struct LintMetadata {
    /// Unique lint identifier.
    pub id: LintId,
    /// Category of the lint.
    pub category: LintCategory,
    /// Short one-line description.
    pub summary: String,
    /// Detailed explanation.
    pub explanation: String,
    /// Default severity.
    pub severity: Severity,
    /// Whether an auto-fix is available.
    pub fixable: bool,
    /// References (e.g., to Lean 4 documentation).
    pub references: Vec<String>,
}

impl LintMetadata {
    /// Create a new metadata record.
    pub fn new(id: &str, category: LintCategory, summary: &str, severity: Severity) -> Self {
        Self {
            id: LintId::new(id),
            category,
            summary: summary.to_string(),
            explanation: String::new(),
            severity,
            fixable: false,
            references: Vec::new(),
        }
    }

    /// Add an explanation.
    pub fn with_explanation(mut self, text: &str) -> Self {
        self.explanation = text.to_string();
        self
    }

    /// Mark as fixable.
    pub fn fixable(mut self) -> Self {
        self.fixable = true;
        self
    }

    /// Add a reference URL.
    pub fn with_reference(mut self, url: &str) -> Self {
        self.references.push(url.to_string());
        self
    }
}

// ============================================================
// LintStats: aggregate statistics
// ============================================================

/// Aggregate statistics from a lint run.
#[derive(Clone, Debug, Default)]
pub struct LintStats {
    /// Total diagnostics emitted.
    pub total_diagnostics: u64,
    /// Error-severity diagnostics.
    pub errors: u64,
    /// Warning-severity diagnostics.
    pub warnings: u64,
    /// Info-severity diagnostics.
    pub infos: u64,
    /// Hint-severity diagnostics.
    pub hints: u64,
    /// Number of declarations checked.
    pub decls_checked: u64,
    /// Number of auto-fixes applied.
    pub fixes_applied: u64,
    /// Number of suppressions honored.
    pub suppressions_honored: u64,
}

impl LintStats {
    /// Create zero-initialized stats.
    pub fn new() -> Self {
        Self::default()
    }

    /// Whether there are any error-severity diagnostics.
    pub fn has_errors(&self) -> bool {
        self.errors > 0
    }

    /// Whether the lint run was clean (no errors or warnings).
    pub fn is_clean(&self) -> bool {
        self.errors == 0 && self.warnings == 0
    }

    /// Add a diagnostic at the given severity.
    pub fn record(&mut self, sev: Severity) {
        self.total_diagnostics += 1;
        match sev {
            Severity::Error => self.errors += 1,
            Severity::Warning => self.warnings += 1,
            Severity::Info => self.infos += 1,
            Severity::Hint => self.hints += 1,
        }
    }
}

impl fmt::Display for LintStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LintStats {{ total: {}, errors: {}, warnings: {}, infos: {}, hints: {} }}",
            self.total_diagnostics, self.errors, self.warnings, self.infos, self.hints
        )
    }
}

// ============================================================
// LintSuppressAnnotation: #[allow(lint_id)]
// ============================================================

/// A lint suppression annotation parsed from source.
#[derive(Clone, Debug, PartialEq)]
pub struct LintSuppressAnnotation {
    /// IDs to suppress.
    pub ids: Vec<LintId>,
    /// Whether this annotation applies to the whole file.
    pub is_file_level: bool,
    /// Line number of the annotation (0-based).
    pub line: usize,
}

impl LintSuppressAnnotation {
    /// Create a single-lint suppression.
    pub fn single(id: &str, line: usize) -> Self {
        Self {
            ids: vec![LintId::new(id)],
            is_file_level: false,
            line,
        }
    }

    /// Create a file-level suppression.
    pub fn file_level(ids: Vec<&str>) -> Self {
        Self {
            ids: ids.into_iter().map(LintId::new).collect(),
            is_file_level: true,
            line: 0,
        }
    }

    /// Whether this suppression covers the given ID.
    pub fn suppresses(&self, id: &LintId) -> bool {
        self.ids.contains(id)
    }
}

// ============================================================
// LintReport: full result of a lint run
// ============================================================

/// The full result of running the lint engine on a source file.
#[derive(Clone, Debug)]
pub struct LintReport {
    /// File that was linted.
    pub filename: String,
    /// All diagnostics emitted.
    pub diagnostics: Vec<LintDiagnostic>,
    /// Aggregate statistics.
    pub stats: LintStats,
    /// Whether auto-fixes were applied.
    pub fixes_applied: bool,
}

impl LintReport {
    /// Create an empty report.
    pub fn empty(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
            diagnostics: Vec::new(),
            stats: LintStats::new(),
            fixes_applied: false,
        }
    }

    /// Add a diagnostic.
    pub fn add_diagnostic(&mut self, diag: LintDiagnostic) {
        self.stats.record(diag.severity);
        self.diagnostics.push(diag);
    }

    /// Diagnostics at or above a given severity.
    pub fn at_severity(&self, sev: Severity) -> Vec<&LintDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity <= sev)
            .collect()
    }

    /// Whether the lint run was clean.
    pub fn is_clean(&self) -> bool {
        self.stats.is_clean()
    }
}

impl fmt::Display for LintReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LintReport[{}] {{ {} diags, {} }}",
            self.filename,
            self.diagnostics.len(),
            self.stats
        )
    }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lint_pass_new() {
        let pass = LintPass::new("style");
        assert_eq!(pass.name, "style");
        assert!(pass.enabled);
        assert!(!pass.can_fix);
    }

    #[test]
    fn test_lint_pass_with_lint() {
        let pass = LintPass::new("unused").with_lint("unused_variable");
        assert_eq!(pass.lint_ids.len(), 1);
        assert_eq!(pass.lint_ids[0].as_str(), "unused_variable");
    }

    #[test]
    fn test_lint_pass_disabled() {
        let pass = LintPass::new("x").disabled();
        assert!(!pass.enabled);
    }

    #[test]
    fn test_lint_pass_with_fixes() {
        let pass = LintPass::new("x").with_fixes();
        assert!(pass.can_fix);
    }

    #[test]
    fn test_lint_category_display() {
        assert_eq!(format!("{}", LintCategory::Correctness), "correctness");
        assert_eq!(format!("{}", LintCategory::Style), "style");
        assert_eq!(format!("{}", LintCategory::Naming), "naming");
    }

    #[test]
    fn test_lint_metadata_new() {
        let meta = LintMetadata::new(
            "unused_variable",
            LintCategory::Redundancy,
            "Unused variable",
            Severity::Warning,
        );
        assert_eq!(meta.id.as_str(), "unused_variable");
        assert_eq!(meta.severity, Severity::Warning);
        assert!(!meta.fixable);
    }

    #[test]
    fn test_lint_metadata_fixable() {
        let meta = LintMetadata::new("x", LintCategory::Style, "X", Severity::Hint).fixable();
        assert!(meta.fixable);
    }

    #[test]
    fn test_lint_metadata_with_explanation() {
        let meta = LintMetadata::new("x", LintCategory::Style, "X", Severity::Hint)
            .with_explanation("More details here.");
        assert!(!meta.explanation.is_empty());
    }

    #[test]
    fn test_lint_stats_default() {
        let s = LintStats::new();
        assert!(!s.has_errors());
        assert!(s.is_clean());
        assert_eq!(s.total_diagnostics, 0);
    }

    #[test]
    fn test_lint_stats_record_error() {
        let mut s = LintStats::new();
        s.record(Severity::Error);
        assert!(s.has_errors());
        assert!(!s.is_clean());
        assert_eq!(s.errors, 1);
    }

    #[test]
    fn test_lint_stats_record_warning() {
        let mut s = LintStats::new();
        s.record(Severity::Warning);
        assert!(!s.has_errors());
        assert!(!s.is_clean());
        assert_eq!(s.warnings, 1);
    }

    #[test]
    fn test_lint_stats_display() {
        let mut s = LintStats::new();
        s.record(Severity::Error);
        s.record(Severity::Warning);
        let text = format!("{}", s);
        assert!(text.contains("total: 2"));
        assert!(text.contains("errors: 1"));
    }

    #[test]
    fn test_lint_suppress_annotation_single() {
        let ann = LintSuppressAnnotation::single("unused_variable", 5);
        assert_eq!(ann.ids.len(), 1);
        assert_eq!(ann.line, 5);
        assert!(!ann.is_file_level);
    }

    #[test]
    fn test_lint_suppress_annotation_file_level() {
        let ann = LintSuppressAnnotation::file_level(vec!["dead_code", "unused_import"]);
        assert_eq!(ann.ids.len(), 2);
        assert!(ann.is_file_level);
    }

    #[test]
    fn test_lint_suppress_annotation_suppresses() {
        let ann = LintSuppressAnnotation::single("unused_variable", 0);
        assert!(ann.suppresses(&LintId::new("unused_variable")));
        assert!(!ann.suppresses(&LintId::new("dead_code")));
    }

    #[test]
    fn test_lint_report_empty() {
        let r = LintReport::empty("foo.ox");
        assert_eq!(r.filename, "foo.ox");
        assert!(r.is_clean());
        assert!(r.diagnostics.is_empty());
    }

    #[test]
    fn test_lint_report_display() {
        let r = LintReport::empty("bar.ox");
        let s = format!("{}", r);
        assert!(s.contains("bar.ox"));
    }

    #[test]
    fn test_lint_id_matches_pattern_wildcard() {
        let id = LintId::new("unused_variable");
        assert!(id.matches_pattern("*"));
        assert!(id.matches_pattern("unused_*"));
        assert!(!id.matches_pattern("dead_*"));
    }

    #[test]
    fn test_lint_stats_hints_and_infos() {
        let mut s = LintStats::new();
        s.record(Severity::Info);
        s.record(Severity::Hint);
        assert_eq!(s.infos, 1);
        assert_eq!(s.hints, 1);
        assert!(s.is_clean());
    }

    #[test]
    fn test_lint_metadata_with_reference() {
        let meta = LintMetadata::new("x", LintCategory::Correctness, "x", Severity::Error)
            .with_reference("https://oxilean.org/lint/x");
        assert_eq!(meta.references.len(), 1);
    }
}

// ============================================================
// LintRuleSet: a named group of LintIds
// ============================================================

/// A named set of lint rules to apply together.
#[derive(Clone, Debug, Default)]
pub struct LintRuleSet {
    /// Set name.
    pub name: String,
    /// Rule IDs.
    pub ids: Vec<LintId>,
}

impl LintRuleSet {
    /// Create an empty set.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ids: Vec::new(),
        }
    }

    /// Add a rule.
    pub fn add(&mut self, id: &str) {
        self.ids.push(LintId::new(id));
    }

    /// Number of rules.
    pub fn len(&self) -> usize {
        self.ids.len()
    }

    /// Whether the set is empty.
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    /// Whether the set contains a given ID.
    pub fn contains(&self, id: &LintId) -> bool {
        self.ids.contains(id)
    }
}

impl std::fmt::Display for LintRuleSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LintRuleSet[{}]({} rules)", self.name, self.ids.len())
    }
}

// ============================================================
// LintLevel: configurable severity level
// ============================================================

/// Configurable level for a lint rule: deny, warn, allow.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LintLevel {
    /// Allow: suppress the lint.
    Allow,
    /// Warn: emit a warning.
    Warn,
    /// Deny: treat as error.
    Deny,
}

impl std::fmt::Display for LintLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LintLevel::Allow => write!(f, "allow"),
            LintLevel::Warn => write!(f, "warn"),
            LintLevel::Deny => write!(f, "deny"),
        }
    }
}

// ============================================================
// Additional tests
// ============================================================

#[cfg(test)]
mod extra_tests {
    use super::*;

    #[test]
    fn test_lint_rule_set_new() {
        let s = LintRuleSet::new("default");
        assert_eq!(s.name, "default");
        assert!(s.is_empty());
    }

    #[test]
    fn test_lint_rule_set_add() {
        let mut s = LintRuleSet::new("style");
        s.add("unused_variable");
        assert_eq!(s.len(), 1);
        assert!(s.contains(&LintId::new("unused_variable")));
    }

    #[test]
    fn test_lint_rule_set_contains_false() {
        let s = LintRuleSet::new("x");
        assert!(!s.contains(&LintId::new("nonexistent")));
    }

    #[test]
    fn test_lint_rule_set_display() {
        let mut s = LintRuleSet::new("perf");
        s.add("redundant_clone");
        let txt = format!("{}", s);
        assert!(txt.contains("perf"));
        assert!(txt.contains("1 rules"));
    }

    #[test]
    fn test_lint_level_ordering() {
        assert!(LintLevel::Deny > LintLevel::Warn);
        assert!(LintLevel::Warn > LintLevel::Allow);
    }

    #[test]
    fn test_lint_level_display() {
        assert_eq!(format!("{}", LintLevel::Allow), "allow");
        assert_eq!(format!("{}", LintLevel::Warn), "warn");
        assert_eq!(format!("{}", LintLevel::Deny), "deny");
    }

    #[test]
    fn test_lint_pass_multiple_lints() {
        let pass = LintPass::new("all")
            .with_lint("unused_variable")
            .with_lint("dead_code")
            .with_lint("unused_import");
        assert_eq!(pass.lint_ids.len(), 3);
    }

    #[test]
    fn test_lint_report_add_and_severity() {
        use crate::framework::Severity;
        let r = LintReport::empty("test.ox");
        // We can only call add_diagnostic, not construct LintDiagnostic directly here
        // So just verify the empty state
        assert!(r.is_clean());
        let _ = r.at_severity(Severity::Error);
    }

    #[test]
    fn test_lint_stats_multiple_records() {
        use crate::framework::Severity;
        let mut s = LintStats::new();
        s.record(Severity::Error);
        s.record(Severity::Warning);
        s.record(Severity::Info);
        s.record(Severity::Hint);
        assert_eq!(s.total_diagnostics, 4);
        assert_eq!(s.errors, 1);
        assert_eq!(s.warnings, 1);
        assert_eq!(s.infos, 1);
        assert_eq!(s.hints, 1);
    }
}

// ============================================================
// LintResult: result of running a lint rule on a declaration
// ============================================================

/// The result of running a single lint rule.
#[derive(Clone, Debug, Default)]
pub struct LintResult {
    /// Diagnostics emitted.
    pub diagnostics: Vec<LintDiagnostic>,
}

impl LintResult {
    /// Create an empty result.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a diagnostic.
    pub fn add(&mut self, diag: LintDiagnostic) {
        self.diagnostics.push(diag);
    }

    /// Whether any diagnostics were emitted.
    pub fn has_diagnostics(&self) -> bool {
        !self.diagnostics.is_empty()
    }

    /// Number of diagnostics.
    pub fn len(&self) -> usize {
        self.diagnostics.len()
    }

    /// Whether there are no diagnostics.
    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Whether the result is clean (no diagnostics).
    pub fn is_clean(&self) -> bool {
        self.diagnostics.is_empty()
    }

    /// Diagnostics at or above a severity.
    pub fn at_severity(&self, sev: Severity) -> Vec<&LintDiagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| d.severity <= sev)
            .collect()
    }

    /// Merge another result into this one.
    pub fn merge(&mut self, other: LintResult) {
        self.diagnostics.extend(other.diagnostics);
    }
}

impl std::fmt::Display for LintResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LintResult({} diagnostics)", self.diagnostics.len())
    }
}

// ============================================================
// LintConfigBuilder: builder for LintConfig
// ============================================================

/// Builder for `LintConfig`.
pub struct LintConfigBuilder {
    config: LintConfig,
}

impl LintConfigBuilder {
    /// Start with default config.
    pub fn new() -> Self {
        Self {
            config: LintConfig::default(),
        }
    }

    /// Allow a lint by ID.
    pub fn allow(mut self, id: &str) -> Self {
        self.config.enabled.insert(LintId::new(id));
        self
    }

    /// Deny a lint by ID.
    pub fn deny(mut self, id: &str) -> Self {
        self.config.disabled.insert(LintId::new(id));
        self
    }

    /// Build the final config.
    pub fn build(self) -> LintConfig {
        self.config
    }
}

impl Default for LintConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// Additional tests
// ============================================================

#[cfg(test)]
mod lint_result_tests {
    use super::*;
    use crate::framework::{LintId, Severity};

    fn mk_diag(sev: Severity) -> LintDiagnostic {
        use crate::framework::SourceRange;
        LintDiagnostic::new(
            LintId::new("test"),
            sev,
            "test message",
            SourceRange::default(),
        )
    }

    #[test]
    fn test_lint_result_empty() {
        let r = LintResult::new();
        assert!(r.is_clean());
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn test_lint_result_add() {
        let mut r = LintResult::new();
        r.add(mk_diag(Severity::Warning));
        assert!(r.has_diagnostics());
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn test_lint_result_at_severity() {
        let mut r = LintResult::new();
        r.add(mk_diag(Severity::Warning));
        r.add(mk_diag(Severity::Error));
        let errors = r.at_severity(Severity::Error);
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn test_lint_result_merge() {
        let mut r1 = LintResult::new();
        let mut r2 = LintResult::new();
        r1.add(mk_diag(Severity::Warning));
        r2.add(mk_diag(Severity::Error));
        r1.merge(r2);
        assert_eq!(r1.len(), 2);
    }

    #[test]
    fn test_lint_result_display() {
        let r = LintResult::new();
        let s = format!("{}", r);
        assert!(s.contains("LintResult"));
    }

    #[test]
    fn test_lint_config_builder() {
        let cfg = LintConfigBuilder::new()
            .allow("dead_code")
            .deny("unused_variable")
            .build();
        assert!(cfg.is_allowed(&LintId::new("dead_code")));
        assert!(cfg.is_denied(&LintId::new("unused_variable")));
    }

    #[test]
    fn test_lint_category_all_variants() {
        let cats = vec![
            LintCategory::Correctness,
            LintCategory::Style,
            LintCategory::Performance,
            LintCategory::Complexity,
            LintCategory::Deprecation,
            LintCategory::Documentation,
            LintCategory::Naming,
            LintCategory::Redundancy,
        ];
        for cat in cats {
            let s = format!("{}", cat);
            assert!(!s.is_empty());
        }
    }

    #[test]
    fn test_lint_suppress_annotation_suppresses_false() {
        let ann = LintSuppressAnnotation::single("unused_variable", 0);
        assert!(!ann.suppresses(&LintId::new("dead_code")));
    }

    #[test]
    fn test_lint_rule_set_add_multiple() {
        let mut s = LintRuleSet::new("default");
        for name in ["a", "b", "c", "d"] {
            s.add(name);
        }
        assert_eq!(s.len(), 4);
    }
}

// ── Lint filter ───────────────────────────────────────────────────────────────

/// A filter that decides whether a diagnostic should be reported.
#[derive(Clone, Debug, Default)]
pub struct LintFilter {
    /// Only report diagnostics whose id matches one of these patterns.
    include_patterns: Vec<String>,
    /// Suppress diagnostics whose id matches one of these patterns.
    exclude_patterns: Vec<String>,
    /// Minimum severity to report.
    min_severity: Option<Severity>,
}

impl LintFilter {
    /// Create an empty (pass-through) filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an include pattern (glob-style: `*` matches any suffix).
    pub fn include(mut self, pattern: &str) -> Self {
        self.include_patterns.push(pattern.to_string());
        self
    }

    /// Add an exclude pattern.
    pub fn exclude(mut self, pattern: &str) -> Self {
        self.exclude_patterns.push(pattern.to_string());
        self
    }

    /// Set a minimum severity.
    pub fn min_severity(mut self, sev: Severity) -> Self {
        self.min_severity = Some(sev);
        self
    }

    /// Return true if a diagnostic passes this filter.
    pub fn accepts(&self, diag: &LintDiagnostic) -> bool {
        // Minimum severity check
        if let Some(min) = &self.min_severity {
            if diag.severity > *min {
                return false;
            }
        }

        // Include patterns (if any specified, must match at least one)
        if !self.include_patterns.is_empty() {
            let matched = self
                .include_patterns
                .iter()
                .any(|p| diag.lint_id.matches_pattern(p));
            if !matched {
                return false;
            }
        }

        // Exclude patterns
        let excluded = self
            .exclude_patterns
            .iter()
            .any(|p| diag.lint_id.matches_pattern(p));
        !excluded
    }

    /// Apply the filter to a vec of diagnostics.
    pub fn apply<'a>(&self, diags: &'a [LintDiagnostic]) -> Vec<&'a LintDiagnostic> {
        diags.iter().filter(|d| self.accepts(d)).collect()
    }
}

// ── Lint formatter ────────────────────────────────────────────────────────────

/// Output format for lint diagnostics.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LintOutputFormat {
    /// Human-readable text (default).
    Text,
    /// JSON output.
    Json,
    /// GitHub Actions annotation format.
    GitHubActions,
    /// Count only (no per-diagnostic output).
    Count,
}

impl std::fmt::Display for LintOutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LintOutputFormat::Text => write!(f, "text"),
            LintOutputFormat::Json => write!(f, "json"),
            LintOutputFormat::GitHubActions => write!(f, "github-actions"),
            LintOutputFormat::Count => write!(f, "count"),
        }
    }
}

impl LintOutputFormat {
    /// Parse from a string, returning `None` if unrecognised.
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "text" => Some(LintOutputFormat::Text),
            "json" => Some(LintOutputFormat::Json),
            "github-actions" => Some(LintOutputFormat::GitHubActions),
            "count" => Some(LintOutputFormat::Count),
            _ => None,
        }
    }
}

// ── Lint profile ──────────────────────────────────────────────────────────────

/// A named configuration profile for the lint engine.
#[derive(Clone, Debug)]
pub struct LintProfile {
    /// Profile name (e.g., `"strict"`, `"pedantic"`, `"minimal"`).
    pub name: String,
    /// Rule sets activated by this profile.
    pub rule_sets: Vec<LintRuleSet>,
    /// Overridden levels for specific rules.
    pub overrides: Vec<(LintId, LintLevel)>,
}

impl LintProfile {
    /// Create a new empty profile.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            rule_sets: Vec::new(),
            overrides: Vec::new(),
        }
    }

    /// Add a rule set to this profile.
    pub fn with_rule_set(mut self, rs: LintRuleSet) -> Self {
        self.rule_sets.push(rs);
        self
    }

    /// Add a rule-level override.
    pub fn with_override(mut self, id: &str, level: LintLevel) -> Self {
        self.overrides.push((LintId::new(id), level));
        self
    }

    /// Return all lint IDs covered by this profile.
    pub fn all_ids(&self) -> Vec<&LintId> {
        self.rule_sets.iter().flat_map(|rs| rs.ids.iter()).collect()
    }

    /// Return the effective level for a given lint id.
    pub fn effective_level(&self, id: &LintId) -> Option<LintLevel> {
        self.overrides
            .iter()
            .rev()
            .find_map(|(k, v)| if k == id { Some(*v) } else { None })
    }
}

#[cfg(test)]
mod lint_profile_tests {
    use super::*;

    fn mk_diag_with_id(id: &str, sev: Severity) -> LintDiagnostic {
        use crate::framework::{LintId, SourceRange};
        LintDiagnostic::new(LintId::new(id), sev, "msg", SourceRange::default())
    }

    #[test]
    fn test_lint_filter_no_constraints() {
        let filter = LintFilter::new();
        let diag = mk_diag_with_id("unused_variable", Severity::Warning);
        assert!(filter.accepts(&diag));
    }

    #[test]
    fn test_lint_filter_min_severity_ok() {
        let filter = LintFilter::new().min_severity(Severity::Warning);
        let warn = mk_diag_with_id("x", Severity::Warning);
        let info = mk_diag_with_id("x", Severity::Info);
        assert!(filter.accepts(&warn));
        assert!(!filter.accepts(&info));
    }

    #[test]
    fn test_lint_filter_include_pattern() {
        let filter = LintFilter::new().include("unused_*");
        let pass = mk_diag_with_id("unused_variable", Severity::Warning);
        let fail = mk_diag_with_id("dead_code", Severity::Warning);
        assert!(filter.accepts(&pass));
        assert!(!filter.accepts(&fail));
    }

    #[test]
    fn test_lint_filter_exclude_pattern() {
        let filter = LintFilter::new().exclude("dead_*");
        let pass = mk_diag_with_id("unused_variable", Severity::Warning);
        let fail = mk_diag_with_id("dead_code", Severity::Warning);
        assert!(filter.accepts(&pass));
        assert!(!filter.accepts(&fail));
    }

    #[test]
    fn test_lint_filter_apply() {
        let filter = LintFilter::new().min_severity(Severity::Error);
        let diags = vec![
            mk_diag_with_id("a", Severity::Error),
            mk_diag_with_id("b", Severity::Warning),
            mk_diag_with_id("c", Severity::Info),
        ];
        let accepted = filter.apply(&diags);
        assert_eq!(accepted.len(), 1);
    }

    #[test]
    fn test_lint_output_format_display() {
        assert_eq!(format!("{}", LintOutputFormat::Text), "text");
        assert_eq!(format!("{}", LintOutputFormat::Json), "json");
        assert_eq!(format!("{}", LintOutputFormat::Count), "count");
    }

    #[test]
    fn test_lint_output_format_from_str() {
        assert_eq!(
            LintOutputFormat::parse("json"),
            Some(LintOutputFormat::Json)
        );
        assert_eq!(LintOutputFormat::parse("unknown"), None);
    }

    #[test]
    fn test_lint_profile_basic() {
        let profile = LintProfile::new("strict");
        assert_eq!(profile.name, "strict");
        assert!(profile.rule_sets.is_empty());
    }

    #[test]
    fn test_lint_profile_with_rule_set() {
        let mut rs = LintRuleSet::new("style");
        rs.add("unused_variable");
        rs.add("dead_code");
        let profile = LintProfile::new("standard").with_rule_set(rs);
        assert_eq!(profile.all_ids().len(), 2);
    }

    #[test]
    fn test_lint_profile_overrides() {
        let profile = LintProfile::new("strict").with_override("dead_code", LintLevel::Deny);
        let id = LintId::new("dead_code");
        assert_eq!(profile.effective_level(&id), Some(LintLevel::Deny));
        let id2 = LintId::new("nonexistent");
        assert_eq!(profile.effective_level(&id2), None);
    }

    #[test]
    fn test_lint_stats_is_clean_after_info_only() {
        let mut s = LintStats::new();
        s.record(Severity::Info);
        assert!(s.is_clean());
    }

    #[test]
    fn test_lint_filter_both_include_and_exclude() {
        // include wins for `*` but exclude removes specific
        let filter = LintFilter::new()
            .include("unused_*")
            .exclude("unused_import");
        let pass = mk_diag_with_id("unused_variable", Severity::Hint);
        let excluded = mk_diag_with_id("unused_import", Severity::Hint);
        assert!(filter.accepts(&pass));
        assert!(!filter.accepts(&excluded));
    }
}
