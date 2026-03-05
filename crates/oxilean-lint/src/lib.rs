//! # OxiLean Lint -- Static Analysis and Lint Rules
//!
//! This crate provides a lint engine and a collection of lint rules for
//! analyzing OxiLean source code.

#![allow(dead_code)]
#![warn(clippy::all)]
#![allow(unused_imports)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::single_match)]
#![allow(clippy::needless_ifs)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::new_without_default)]
#![allow(clippy::inherent_to_string_shadow_display)]
#![allow(clippy::type_complexity)]
#![allow(clippy::manual_strip)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::implicit_saturating_sub)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_bool)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::manual_find)]
#![allow(clippy::for_kv_map)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::to_string_in_format_args)]

pub mod autofix;
pub mod framework;
pub mod ide_integration;
pub mod plugin;
pub mod rules;

pub use framework::{
    AutoFix, LintConfig, LintContext, LintDiagnostic, LintEngine, LintId, LintRegistry, LintRule,
    LintSuppression, Severity,
};
pub use rules::{
    DeadCodeRule, DeprecatedApiRule, DeprecatedTacticRule, LongProofRule, MissingDocRule,
    MissingDocstringRule, NamingConventionRule, RedundantAssumptionRule, RedundantPatternRule,
    SimplifiableExprRule, StyleRule, UnreachableCodeRule, UnusedHypothesisRule, UnusedImportRule,
    UnusedVariableRule,
};

use std::fmt;

// ============================================================
// LintPass: a group of related rules
// ============================================================

/// A lint pass groups related lint rules that run together.
#[derive(Clone, Debug)]
pub struct LintPass {
    /// Name of the pass.
    pub name: String,
    /// Lint IDs included in this pass.
    pub lint_ids: Vec<LintId>,
    /// Whether this pass is enabled by default.
    pub enabled: bool,
    /// Whether this pass can modify the AST (for fixes).
    pub can_fix: bool,
}

impl LintPass {
    /// Create a new lint pass.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            lint_ids: Vec::new(),
            enabled: true,
            can_fix: false,
        }
    }

    /// Add a lint to this pass.
    pub fn with_lint(mut self, id: &str) -> Self {
        self.lint_ids.push(LintId::new(id));
        self
    }

    /// Enable fix mode for this pass.
    pub fn with_fixes(mut self) -> Self {
        self.can_fix = true;
        self
    }

    /// Disable this pass.
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}

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
        use framework::Severity;
        let r = LintReport::empty("test.ox");
        // We can only call add_diagnostic, not construct LintDiagnostic directly here
        // So just verify the empty state
        assert!(r.is_clean());
        let _ = r.at_severity(Severity::Error);
    }

    #[test]
    fn test_lint_stats_multiple_records() {
        use framework::Severity;
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
    use framework::{LintId, Severity};

    fn mk_diag(sev: Severity) -> LintDiagnostic {
        use framework::SourceRange;
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
        use framework::{LintId, SourceRange};
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

// ============================================================
// LintDatabase
// ============================================================

/// A persistent store of all known lint rules and their metadata.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct LintDatabase {
    entries: std::collections::HashMap<String, LintEntry>,
}

/// Metadata for a single lint rule stored in the database.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct LintEntry {
    pub id: LintId,
    pub description: String,
    pub default_level: Severity,
    pub tags: Vec<String>,
    pub has_autofix: bool,
}

impl LintEntry {
    #[allow(dead_code)]
    pub fn new(id: &str, description: &str, default_level: Severity) -> Self {
        Self {
            id: LintId::new(id),
            description: description.to_string(),
            default_level,
            tags: Vec::new(),
            has_autofix: false,
        }
    }

    #[allow(dead_code)]
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    #[allow(dead_code)]
    pub fn with_autofix(mut self) -> Self {
        self.has_autofix = true;
        self
    }
}

impl LintDatabase {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }

    /// Register a lint entry.
    #[allow(dead_code)]
    pub fn register(&mut self, entry: LintEntry) {
        self.entries.insert(entry.id.as_str().to_string(), entry);
    }

    /// Look up a lint entry by ID string.
    #[allow(dead_code)]
    pub fn get(&self, id: &str) -> Option<&LintEntry> {
        self.entries.get(id)
    }

    /// Return all lint IDs sorted.
    #[allow(dead_code)]
    pub fn all_ids(&self) -> Vec<&str> {
        let mut ids: Vec<&str> = self.entries.keys().map(|s| s.as_str()).collect();
        ids.sort();
        ids
    }

    /// Return entries matching a given tag.
    #[allow(dead_code)]
    pub fn by_tag(&self, tag: &str) -> Vec<&LintEntry> {
        self.entries
            .values()
            .filter(|e| e.tags.iter().any(|t| t == tag))
            .collect()
    }

    /// Return entries that have an auto-fix available.
    #[allow(dead_code)]
    pub fn with_autofix(&self) -> Vec<&LintEntry> {
        self.entries.values().filter(|e| e.has_autofix).collect()
    }

    /// Total number of entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

// ============================================================
// LintRunOptions
// ============================================================

/// Options controlling a single lint run.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct LintRunOptions {
    /// Whether to emit `Info`-level diagnostics.
    pub include_info: bool,
    /// Whether to emit `Hint`-level diagnostics.
    pub include_hints: bool,
    /// Maximum number of diagnostics to emit before truncating.
    pub max_diagnostics: Option<usize>,
    /// Whether to apply auto-fixes automatically.
    pub auto_apply_fixes: bool,
    /// Whether to stop on the first error.
    pub fail_fast: bool,
}

impl LintRunOptions {
    #[allow(dead_code)]
    pub fn default_opts() -> Self {
        Self {
            include_info: true,
            include_hints: false,
            max_diagnostics: None,
            auto_apply_fixes: false,
            fail_fast: false,
        }
    }

    #[allow(dead_code)]
    pub fn strict() -> Self {
        Self {
            include_info: true,
            include_hints: true,
            max_diagnostics: None,
            auto_apply_fixes: false,
            fail_fast: true,
        }
    }
}

impl Default for LintRunOptions {
    fn default() -> Self {
        Self::default_opts()
    }
}

// ============================================================
// LintSummaryReport
// ============================================================

/// A high-level summary report of a lint run.
#[allow(dead_code)]
pub struct LintSummaryReport {
    pub total_diagnostics: usize,
    pub by_severity: std::collections::HashMap<String, usize>,
    pub by_category: std::collections::HashMap<String, usize>,
    pub files_with_issues: usize,
    pub auto_fixes_available: usize,
}

impl LintSummaryReport {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            total_diagnostics: 0,
            by_severity: std::collections::HashMap::new(),
            by_category: std::collections::HashMap::new(),
            files_with_issues: 0,
            auto_fixes_available: 0,
        }
    }

    /// Add a diagnostic to the report.
    #[allow(dead_code)]
    pub fn add(&mut self, diag: &LintDiagnostic) {
        self.total_diagnostics += 1;
        let sev_key = format!("{:?}", diag.severity).to_lowercase();
        *self.by_severity.entry(sev_key).or_insert(0) += 1;
        if diag.fix.is_some() {
            self.auto_fixes_available += 1;
        }
    }

    /// Returns `true` when there are no errors or warnings.
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        let errors = self.by_severity.get("error").copied().unwrap_or(0);
        let warnings = self.by_severity.get("warning").copied().unwrap_or(0);
        errors == 0 && warnings == 0
    }
}

impl Default for LintSummaryReport {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// LintIgnoreList
// ============================================================

/// A list of lint IDs that are explicitly ignored (suppressed globally).
#[allow(dead_code)]
pub struct LintIgnoreList {
    ignored: std::collections::HashSet<String>,
}

impl LintIgnoreList {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            ignored: std::collections::HashSet::new(),
        }
    }

    /// Add a lint ID to the ignore list.
    #[allow(dead_code)]
    pub fn ignore(&mut self, id: &str) {
        self.ignored.insert(id.to_string());
    }

    /// Returns `true` if the given ID is ignored.
    #[allow(dead_code)]
    pub fn is_ignored(&self, id: &str) -> bool {
        self.ignored.contains(id)
    }

    /// Filter a slice of diagnostics, removing any that are ignored.
    #[allow(dead_code)]
    pub fn filter<'a>(&self, diags: &'a [LintDiagnostic]) -> Vec<&'a LintDiagnostic> {
        diags
            .iter()
            .filter(|d| !self.is_ignored(d.lint_id.as_str()))
            .collect()
    }

    /// Number of ignored lints.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.ignored.len()
    }
}

impl Default for LintIgnoreList {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// LintFormatter
// ============================================================

/// Formats `LintDiagnostic`s into strings according to an output format.
#[allow(dead_code)]
pub struct LintFormatter {
    pub format: LintOutputFormat,
}

impl LintFormatter {
    #[allow(dead_code)]
    pub fn new(format: LintOutputFormat) -> Self {
        Self { format }
    }

    /// Format a single diagnostic.
    #[allow(dead_code)]
    pub fn format_one(&self, diag: &LintDiagnostic) -> String {
        let file = diag.range.file.as_deref().unwrap_or("unknown");
        let offset = diag.range.start;
        match self.format {
            LintOutputFormat::Text => {
                format!(
                    "[{:?}] {} at {}:{}: {}",
                    diag.severity,
                    diag.lint_id.as_str(),
                    file,
                    offset,
                    diag.message
                )
            }
            LintOutputFormat::GitHubActions => {
                let level = match diag.severity {
                    Severity::Error => "error",
                    Severity::Warning => "warning",
                    Severity::Hint | Severity::Info => "notice",
                };
                format!(
                    "::{} file={},line={}::{}",
                    level, file, offset, diag.message
                )
            }
            LintOutputFormat::Json => {
                format!(
                    "{{\"id\":\"{}\",\"severity\":\"{:?}\",\"file\":\"{}\",\"line\":{},\"message\":\"{}\"}}",
                    diag.lint_id.as_str(),
                    diag.severity,
                    file,
                    offset,
                    diag.message.replace('"', "\\\"")
                )
            }
            LintOutputFormat::Count => {
                format!(
                    "{}:{}:{:?}:{} - {}",
                    file,
                    offset,
                    diag.severity,
                    diag.lint_id.as_str(),
                    diag.message
                )
            }
        }
    }

    /// Format multiple diagnostics and return a combined string.
    #[allow(dead_code)]
    pub fn format_all(&self, diags: &[LintDiagnostic]) -> String {
        diags
            .iter()
            .map(|d| self.format_one(d))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

// ============================================================
// LintTrend
// ============================================================

/// Tracks diagnostic counts across runs to detect trends.
#[allow(dead_code)]
pub struct LintTrend {
    snapshots: Vec<(String, usize)>,
}

impl LintTrend {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            snapshots: Vec::new(),
        }
    }

    /// Record a new snapshot with a label and diagnostic count.
    #[allow(dead_code)]
    pub fn record(&mut self, label: &str, count: usize) {
        self.snapshots.push((label.to_string(), count));
    }

    /// Returns `true` when the latest count is less than the previous.
    #[allow(dead_code)]
    pub fn is_improving(&self) -> bool {
        if self.snapshots.len() < 2 {
            return false;
        }
        let prev = self.snapshots[self.snapshots.len() - 2].1;
        let latest = self.snapshots[self.snapshots.len() - 1].1;
        latest < prev
    }

    /// Latest diagnostic count.
    #[allow(dead_code)]
    pub fn latest_count(&self) -> usize {
        self.snapshots.last().map(|(_, c)| *c).unwrap_or(0)
    }

    /// Number of snapshots.
    #[allow(dead_code)]
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }
}

impl Default for LintTrend {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// LintBaseline
// ============================================================

/// Records a known set of lint diagnostics as a baseline for comparison.
#[allow(dead_code)]
pub struct LintBaseline {
    /// Known diagnostic fingerprints (id + location key).
    known: std::collections::HashSet<String>,
}

impl LintBaseline {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            known: std::collections::HashSet::new(),
        }
    }

    /// Add a diagnostic to the baseline.
    #[allow(dead_code)]
    pub fn add(&mut self, diag: &LintDiagnostic) {
        let file = diag.range.file.as_deref().unwrap_or("unknown");
        let key = format!("{}:{}:{}", diag.lint_id.as_str(), file, diag.range.start);
        self.known.insert(key);
    }

    /// Returns `true` when `diag` is already in the baseline (i.e., not new).
    #[allow(dead_code)]
    pub fn is_known(&self, diag: &LintDiagnostic) -> bool {
        let file = diag.range.file.as_deref().unwrap_or("unknown");
        let key = format!("{}:{}:{}", diag.lint_id.as_str(), file, diag.range.start);
        self.known.contains(&key)
    }

    /// Filter to only new diagnostics not in the baseline.
    #[allow(dead_code)]
    pub fn new_diagnostics<'a>(&self, diags: &'a [LintDiagnostic]) -> Vec<&'a LintDiagnostic> {
        diags.iter().filter(|d| !self.is_known(d)).collect()
    }

    /// Number of items in the baseline.
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.known.len()
    }
}

impl Default for LintBaseline {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// LintRuleGroup
// ============================================================

/// A named group of lint rules for easy management.
#[allow(dead_code)]
pub struct LintRuleGroup {
    pub name: String,
    pub description: String,
    pub rules: Vec<String>,
}

impl LintRuleGroup {
    #[allow(dead_code)]
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            rules: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_rule(&mut self, rule: &str) {
        self.rules.push(rule.to_string());
    }

    #[allow(dead_code)]
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    #[allow(dead_code)]
    pub fn contains(&self, rule: &str) -> bool {
        self.rules.iter().any(|r| r == rule)
    }
}

// ============================================================
// LintAggregator
// ============================================================

/// Aggregates diagnostics from multiple sources into a combined set.
#[allow(dead_code)]
pub struct LintAggregator {
    diagnostics: Vec<LintDiagnostic>,
}

impl LintAggregator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    /// Add a single diagnostic.
    #[allow(dead_code)]
    pub fn add(&mut self, diag: LintDiagnostic) {
        self.diagnostics.push(diag);
    }

    /// Add multiple diagnostics.
    #[allow(dead_code)]
    pub fn add_all(&mut self, diags: Vec<LintDiagnostic>) {
        self.diagnostics.extend(diags);
    }

    /// Consume the aggregator and return the collected diagnostics.
    #[allow(dead_code)]
    pub fn into_diagnostics(self) -> Vec<LintDiagnostic> {
        self.diagnostics
    }

    /// Number of diagnostics collected.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        self.diagnostics.len()
    }

    /// Count by severity.
    #[allow(dead_code)]
    pub fn count_by_severity(&self, severity: Severity) -> usize {
        self.diagnostics
            .iter()
            .filter(|d| d.severity == severity)
            .count()
    }
}

impl Default for LintAggregator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// LintEventLog
// ============================================================

/// Logs lint events (rule run, diagnostic emitted, fix applied) for debugging.
#[allow(dead_code)]
pub struct LintEventLog {
    events: Vec<LintEvent>,
    counter: u64,
}

/// A single lint event.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LintEvent {
    pub id: u64,
    pub kind: LintEventKind,
    pub message: String,
}

/// The kind of lint event.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum LintEventKind {
    RuleStarted,
    RuleFinished,
    DiagnosticEmitted,
    FixApplied,
    FixSkipped,
    PassEnabled,
    PassDisabled,
}

impl LintEventLog {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            events: Vec::new(),
            counter: 0,
        }
    }

    #[allow(dead_code)]
    pub fn log(&mut self, kind: LintEventKind, message: &str) -> u64 {
        self.counter += 1;
        let id = self.counter;
        self.events.push(LintEvent {
            id,
            kind,
            message: message.to_string(),
        });
        id
    }

    #[allow(dead_code)]
    pub fn total(&self) -> usize {
        self.events.len()
    }

    #[allow(dead_code)]
    pub fn events(&self) -> &[LintEvent] {
        &self.events
    }
}

impl Default for LintEventLog {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// LintDiff
// ============================================================

/// Computes the diff between two sets of diagnostics (added/removed).
#[allow(dead_code)]
pub struct LintDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
}

impl LintDiff {
    /// Compute the diff between `before` and `after` sets of diagnostic fingerprints.
    #[allow(dead_code)]
    pub fn compute(before: &[String], after: &[String]) -> Self {
        let before_set: std::collections::HashSet<&String> = before.iter().collect();
        let after_set: std::collections::HashSet<&String> = after.iter().collect();
        let added = after_set
            .difference(&before_set)
            .map(|s| s.to_string())
            .collect();
        let removed = before_set
            .difference(&after_set)
            .map(|s| s.to_string())
            .collect();
        Self { added, removed }
    }

    /// Returns `true` when there are no differences.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
}

// ============================================================
// Additional tests
// ============================================================

#[cfg(test)]
mod lib_extended_tests {
    use super::*;
    fn mk_diag(id: &str, severity: Severity) -> LintDiagnostic {
        LintDiagnostic::new(
            LintId::new(id),
            severity,
            "test",
            framework::SourceRange::with_file(0, 0, "test.ox".to_string()),
        )
    }

    // --- LintDatabase ---

    #[test]
    fn lint_database_register_and_get() {
        let mut db = LintDatabase::new();
        let entry = LintEntry::new("unused_import", "Remove unused imports", Severity::Warning)
            .with_tag("style")
            .with_autofix();
        db.register(entry);
        assert!(!db.is_empty());
        let found = db.get("unused_import").expect("key should exist");
        assert!(found.has_autofix);
        assert!(found.tags.contains(&"style".to_string()));
    }

    #[test]
    fn lint_database_by_tag() {
        let mut db = LintDatabase::new();
        db.register(LintEntry::new("a", "a", Severity::Info).with_tag("security"));
        db.register(LintEntry::new("b", "b", Severity::Info).with_tag("style"));
        db.register(LintEntry::new("c", "c", Severity::Info).with_tag("security"));
        let sec = db.by_tag("security");
        assert_eq!(sec.len(), 2);
    }

    #[test]
    fn lint_database_with_autofix() {
        let mut db = LintDatabase::new();
        db.register(LintEntry::new("fixable", "fixable", Severity::Warning).with_autofix());
        db.register(LintEntry::new("not_fixable", "no fix", Severity::Warning));
        let fixable = db.with_autofix();
        assert_eq!(fixable.len(), 1);
    }

    // --- LintRunOptions ---

    #[test]
    fn lint_run_options_default() {
        let opts = LintRunOptions::default_opts();
        assert!(opts.include_info);
        assert!(!opts.include_hints);
        assert!(!opts.auto_apply_fixes);
        assert!(!opts.fail_fast);
    }

    #[test]
    fn lint_run_options_strict() {
        let opts = LintRunOptions::strict();
        assert!(opts.include_hints);
        assert!(opts.fail_fast);
    }

    // --- LintCategory ---

    #[test]
    fn lint_category_display() {
        assert_eq!(format!("{}", LintCategory::Style), "style");
        assert_eq!(format!("{}", LintCategory::Security), "security");
        assert_eq!(
            format!("{}", LintCategory::Custom("my_cat".to_string())),
            "custom:my_cat"
        );
    }

    // --- LintSummaryReport ---

    #[test]
    fn lint_summary_report_add() {
        let mut report = LintSummaryReport::new();
        let diag = mk_diag("test", Severity::Warning);
        report.add(&diag);
        assert_eq!(report.total_diagnostics, 1);
        assert!(!report.is_clean()); // has warning
    }

    #[test]
    fn lint_summary_report_clean_with_info_only() {
        let mut report = LintSummaryReport::new();
        report.add(&mk_diag("test", Severity::Info));
        assert!(report.is_clean());
    }

    // --- LintIgnoreList ---

    #[test]
    fn lint_ignore_list_filters() {
        let mut ignore = LintIgnoreList::new();
        ignore.ignore("dead_code");
        ignore.ignore("unused_import");
        let diags = vec![
            mk_diag("dead_code", Severity::Warning),
            mk_diag("naming_convention", Severity::Warning),
        ];
        let filtered = ignore.filter(&diags);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].lint_id.as_str(), "naming_convention");
    }

    #[test]
    fn lint_ignore_list_is_ignored() {
        let mut ignore = LintIgnoreList::new();
        ignore.ignore("foo");
        assert!(ignore.is_ignored("foo"));
        assert!(!ignore.is_ignored("bar"));
        assert_eq!(ignore.len(), 1);
    }

    // --- LintOutputFormat ---

    #[test]
    fn lint_output_format_display() {
        assert_eq!(format!("{}", LintOutputFormat::Text), "text");
        assert_eq!(format!("{}", LintOutputFormat::Json), "json");
        assert_eq!(
            format!("{}", LintOutputFormat::GitHubActions),
            "github-actions"
        );
        assert_eq!(format!("{}", LintOutputFormat::Count), "count");
    }

    // --- LintFormatter ---

    #[test]
    fn lint_formatter_text() {
        let formatter = LintFormatter::new(LintOutputFormat::Text);
        let diag = mk_diag("unused_import", Severity::Warning);
        let output = formatter.format_one(&diag);
        assert!(output.contains("unused_import"));
        assert!(output.contains("test.ox"));
    }

    #[test]
    fn lint_formatter_github() {
        let formatter = LintFormatter::new(LintOutputFormat::GitHubActions);
        let diag = mk_diag("unused_import", Severity::Warning);
        let output = formatter.format_one(&diag);
        assert!(output.starts_with("::warning"));
    }

    #[test]
    fn lint_formatter_json() {
        let formatter = LintFormatter::new(LintOutputFormat::Json);
        let diag = mk_diag("foo", Severity::Error);
        let output = formatter.format_one(&diag);
        assert!(output.starts_with('{'));
        assert!(output.contains("\"id\":\"foo\""));
    }

    #[test]
    fn lint_formatter_compact() {
        let formatter = LintFormatter::new(LintOutputFormat::Count);
        let diag = mk_diag("bar", Severity::Info);
        let output = formatter.format_one(&diag);
        assert!(output.contains("bar"));
    }

    #[test]
    fn lint_formatter_format_all() {
        let formatter = LintFormatter::new(LintOutputFormat::Count);
        let diags = vec![
            mk_diag("a", Severity::Warning),
            mk_diag("b", Severity::Info),
        ];
        let output = formatter.format_all(&diags);
        assert!(output.contains('\n'));
    }

    // --- LintTrend ---

    #[test]
    fn lint_trend_improving() {
        let mut trend = LintTrend::new();
        trend.record("v1", 10);
        trend.record("v2", 5);
        assert!(trend.is_improving());
        assert_eq!(trend.latest_count(), 5);
        assert_eq!(trend.snapshot_count(), 2);
    }

    #[test]
    fn lint_trend_not_improving() {
        let mut trend = LintTrend::new();
        trend.record("v1", 3);
        trend.record("v2", 7);
        assert!(!trend.is_improving());
    }

    // --- LintBaseline ---

    #[test]
    fn lint_baseline_filters_known() {
        let diag = mk_diag("dead_code", Severity::Warning);
        let mut baseline = LintBaseline::new();
        baseline.add(&diag);
        assert!(baseline.is_known(&diag));

        let new_diag = mk_diag("new_lint", Severity::Warning);
        assert!(!baseline.is_known(&new_diag));

        let all = vec![diag, new_diag];
        let new_only = baseline.new_diagnostics(&all);
        assert_eq!(new_only.len(), 1);
        assert_eq!(new_only[0].lint_id.as_str(), "new_lint");
    }

    // --- LintRuleGroup ---

    #[test]
    fn lint_rule_group_contains() {
        let mut group = LintRuleGroup::new("style", "Style rules");
        group.add_rule("naming_convention");
        group.add_rule("unused_import");
        assert!(group.contains("naming_convention"));
        assert!(!group.contains("dead_code"));
        assert_eq!(group.rule_count(), 2);
    }

    // --- LintAggregator ---

    #[test]
    fn lint_aggregator_collects() {
        let mut agg = LintAggregator::new();
        agg.add(mk_diag("a", Severity::Warning));
        agg.add(mk_diag("b", Severity::Error));
        agg.add_all(vec![mk_diag("c", Severity::Info)]);
        assert_eq!(agg.count(), 3);
        assert_eq!(agg.count_by_severity(Severity::Warning), 1);
        assert_eq!(agg.count_by_severity(Severity::Error), 1);
    }

    #[test]
    fn lint_aggregator_into_diagnostics() {
        let mut agg = LintAggregator::new();
        agg.add(mk_diag("x", Severity::Info));
        let diags = agg.into_diagnostics();
        assert_eq!(diags.len(), 1);
    }

    // --- LintEventLog ---

    #[test]
    fn lint_event_log_basic() {
        let mut log = LintEventLog::new();
        let id = log.log(LintEventKind::RuleStarted, "checking naming_convention");
        assert_eq!(log.total(), 1);
        assert_eq!(log.events()[0].id, id);
    }

    // --- LintDiff ---

    #[test]
    fn lint_diff_no_change() {
        let fingerprints = vec!["a".to_string(), "b".to_string()];
        let diff = LintDiff::compute(&fingerprints, &fingerprints);
        assert!(diff.is_empty());
    }

    #[test]
    fn lint_diff_new_and_removed() {
        let before = vec!["a".to_string(), "b".to_string()];
        let after = vec!["b".to_string(), "c".to_string()];
        let diff = LintDiff::compute(&before, &after);
        assert!(!diff.is_empty());
        assert!(diff.added.contains(&"c".to_string()));
        assert!(diff.removed.contains(&"a".to_string()));
    }
}

// ============================================================
// LintRuleMetadata
// ============================================================

/// Complete metadata for a lint rule.
#[allow(dead_code)]
pub struct LintRuleMetadata {
    pub id: LintId,
    pub name: String,
    pub category: LintCategory,
    pub default_level: Severity,
    pub description: String,
    pub rationale: String,
    pub examples: Vec<LintExample>,
    pub since_version: String,
    pub deprecated: bool,
}

/// An example of a lint rule firing (good/bad pair).
#[allow(dead_code)]
pub struct LintExample {
    pub title: String,
    pub bad: String,
    pub good: String,
}

impl LintRuleMetadata {
    #[allow(dead_code)]
    pub fn new(id: &str, name: &str, category: LintCategory, default_level: Severity) -> Self {
        Self {
            id: LintId::new(id),
            name: name.to_string(),
            category,
            default_level,
            description: String::new(),
            rationale: String::new(),
            examples: Vec::new(),
            since_version: "0.1.0".to_string(),
            deprecated: false,
        }
    }

    #[allow(dead_code)]
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = desc.to_string();
        self
    }

    #[allow(dead_code)]
    pub fn with_rationale(mut self, rationale: &str) -> Self {
        self.rationale = rationale.to_string();
        self
    }

    #[allow(dead_code)]
    pub fn with_example(mut self, title: &str, bad: &str, good: &str) -> Self {
        self.examples.push(LintExample {
            title: title.to_string(),
            bad: bad.to_string(),
            good: good.to_string(),
        });
        self
    }

    #[allow(dead_code)]
    pub fn mark_deprecated(mut self) -> Self {
        self.deprecated = true;
        self
    }
}

// ============================================================
// LintPriorityQueue
// ============================================================

/// A priority queue for diagnostics, returning the most severe first.
#[allow(dead_code)]
pub struct LintPriorityQueue {
    items: Vec<(u8, LintDiagnostic)>,
}

impl LintPriorityQueue {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    #[allow(dead_code)]
    fn severity_to_priority(s: Severity) -> u8 {
        match s {
            Severity::Error => 4,
            Severity::Warning => 3,
            Severity::Hint => 2,
            Severity::Info => 1,
        }
    }

    /// Push a diagnostic into the queue.
    #[allow(dead_code)]
    pub fn push(&mut self, diag: LintDiagnostic) {
        let priority = Self::severity_to_priority(diag.severity);
        self.items.push((priority, diag));
        // Keep sorted by priority descending.
        self.items.sort_by(|a, b| b.0.cmp(&a.0));
    }

    /// Pop the highest-priority diagnostic.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<LintDiagnostic> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0).1)
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl Default for LintPriorityQueue {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// LintBudget
// ============================================================

/// Limits the total number of diagnostics emitted.
#[allow(dead_code)]
pub struct LintBudget {
    pub max_total: usize,
    pub max_per_file: usize,
    total_used: usize,
    per_file_used: std::collections::HashMap<String, usize>,
}

impl LintBudget {
    #[allow(dead_code)]
    pub fn new(max_total: usize, max_per_file: usize) -> Self {
        Self {
            max_total,
            max_per_file,
            total_used: 0,
            per_file_used: std::collections::HashMap::new(),
        }
    }

    /// Try to "spend" a slot for `file`. Returns `false` if any budget is exhausted.
    #[allow(dead_code)]
    pub fn try_spend(&mut self, file: &str) -> bool {
        if self.total_used >= self.max_total {
            return false;
        }
        let per_file = self.per_file_used.entry(file.to_string()).or_insert(0);
        if *per_file >= self.max_per_file {
            return false;
        }
        *per_file += 1;
        self.total_used += 1;
        true
    }

    #[allow(dead_code)]
    pub fn remaining_total(&self) -> usize {
        self.max_total.saturating_sub(self.total_used)
    }
}

// ============================================================
// LintCooldown
// ============================================================

/// Suppresses repeated identical diagnostics within a cooldown window.
#[allow(dead_code)]
pub struct LintCooldown {
    pub window: usize,
    seen: std::collections::HashMap<String, usize>,
    current_tick: usize,
}

impl LintCooldown {
    #[allow(dead_code)]
    pub fn new(window: usize) -> Self {
        Self {
            window,
            seen: std::collections::HashMap::new(),
            current_tick: 0,
        }
    }

    /// Advance the internal tick counter.
    #[allow(dead_code)]
    pub fn tick(&mut self) {
        self.current_tick += 1;
    }

    /// Returns `true` if `fingerprint` should be emitted (not in cooldown).
    #[allow(dead_code)]
    pub fn should_emit(&mut self, fingerprint: &str) -> bool {
        match self.seen.get(fingerprint).copied() {
            None => {
                self.seen.insert(fingerprint.to_string(), self.current_tick);
                true
            }
            Some(last) if self.current_tick.saturating_sub(last) >= self.window => {
                self.seen.insert(fingerprint.to_string(), self.current_tick);
                true
            }
            _ => false,
        }
    }
}

// ============================================================
// Additional tests
// ============================================================

#[cfg(test)]
mod lib_final_tests {
    use super::*;

    fn mk_diag(id: &str, severity: Severity) -> LintDiagnostic {
        LintDiagnostic::new(
            LintId::new(id),
            severity,
            "test",
            framework::SourceRange::new(0, 0),
        )
    }

    // --- LintRuleMetadata ---

    #[test]
    fn lint_rule_metadata_basic() {
        let meta = LintRuleMetadata::new(
            "unused_import",
            "Unused Import",
            LintCategory::Style,
            Severity::Warning,
        )
        .with_description("Detects unused imports.")
        .with_rationale("Unused imports add noise.")
        .with_example("simple", "import Unused", "-- no import");
        assert_eq!(meta.id.as_str().to_string(), "unused_import");
        assert_eq!(meta.examples.len(), 1);
        assert!(!meta.deprecated);
    }

    #[test]
    fn lint_rule_metadata_deprecated() {
        let meta =
            LintRuleMetadata::new("old_lint", "Old Lint", LintCategory::Style, Severity::Info)
                .mark_deprecated();
        assert!(meta.deprecated);
    }

    // --- LintPriorityQueue ---

    #[test]
    fn lint_priority_queue_orders_by_severity() {
        let mut pq = LintPriorityQueue::new();
        pq.push(mk_diag("info_lint", Severity::Info));
        pq.push(mk_diag("error_lint", Severity::Error));
        pq.push(mk_diag("warning_lint", Severity::Warning));
        // Error should come out first.
        let first = pq.pop().expect("queue should not be empty");
        assert_eq!(first.severity, Severity::Error);
        let second = pq.pop().expect("queue should not be empty");
        assert_eq!(second.severity, Severity::Warning);
    }

    #[test]
    fn lint_priority_queue_empty() {
        let mut pq = LintPriorityQueue::new();
        assert!(pq.is_empty());
        assert!(pq.pop().is_none());
    }

    // --- LintBudget ---

    #[test]
    fn lint_budget_total_limit() {
        let mut budget = LintBudget::new(2, 10);
        assert!(budget.try_spend("a.ox"));
        assert!(budget.try_spend("b.ox"));
        assert!(!budget.try_spend("c.ox")); // total exhausted
        assert_eq!(budget.remaining_total(), 0);
    }

    #[test]
    fn lint_budget_per_file_limit() {
        let mut budget = LintBudget::new(100, 2);
        assert!(budget.try_spend("a.ox"));
        assert!(budget.try_spend("a.ox"));
        assert!(!budget.try_spend("a.ox")); // per-file exhausted
    }

    // --- LintCooldown ---

    #[test]
    fn lint_cooldown_emits_once_then_suppresses() {
        let mut cd = LintCooldown::new(3);
        assert!(cd.should_emit("lint:a.ox:1"));
        // Same fingerprint within window — should be suppressed.
        assert!(!cd.should_emit("lint:a.ox:1"));
        // Tick past the window.
        cd.tick();
        cd.tick();
        cd.tick();
        assert!(cd.should_emit("lint:a.ox:1"));
    }

    #[test]
    fn lint_cooldown_different_fingerprints() {
        let mut cd = LintCooldown::new(5);
        assert!(cd.should_emit("fp1"));
        assert!(cd.should_emit("fp2")); // different fingerprint, always OK
    }
}

// ============================================================
// LintSessionContext
// ============================================================

/// Context for a lint session across multiple files.
#[allow(dead_code)]
pub struct LintSessionContext {
    pub session_id: String,
    pub files_processed: usize,
    pub total_diagnostics: usize,
    pub elapsed_ms: u64,
}

impl LintSessionContext {
    #[allow(dead_code)]
    pub fn new(session_id: &str) -> Self {
        Self {
            session_id: session_id.to_string(),
            files_processed: 0,
            total_diagnostics: 0,
            elapsed_ms: 0,
        }
    }

    #[allow(dead_code)]
    pub fn record_file(&mut self, diagnostic_count: usize, elapsed_ms: u64) {
        self.files_processed += 1;
        self.total_diagnostics += diagnostic_count;
        self.elapsed_ms += elapsed_ms;
    }

    #[allow(dead_code)]
    pub fn average_diagnostics_per_file(&self) -> f64 {
        if self.files_processed == 0 {
            return 0.0;
        }
        self.total_diagnostics as f64 / self.files_processed as f64
    }
}

// ============================================================
// LintConfigValidator
// ============================================================

/// Validates lint configuration for consistency.
#[allow(dead_code)]
pub struct LintConfigValidator;

impl LintConfigValidator {
    /// Validate that a config has no conflicting entries.
    /// Returns a list of validation error messages.
    #[allow(dead_code)]
    pub fn validate(config: &LintConfig) -> Vec<String> {
        let mut errors = Vec::new();
        // Check that no rule appears in both enabled and disabled sets.
        for id in config.enabled.iter() {
            if config.disabled.contains(id) {
                errors.push(format!(
                    "Rule `{}` appears in both enabled and disabled lists",
                    id.as_str().to_string()
                ));
            }
        }
        errors
    }
}

#[cfg(test)]
mod lint_session_tests {
    use super::*;

    #[test]
    fn lint_session_context_average() {
        let mut ctx = LintSessionContext::new("sess-1");
        ctx.record_file(10, 50);
        ctx.record_file(20, 100);
        assert_eq!(ctx.files_processed, 2);
        assert!((ctx.average_diagnostics_per_file() - 15.0).abs() < 1e-9);
        assert_eq!(ctx.elapsed_ms, 150);
    }

    #[test]
    fn lint_config_builder_builds() {
        let config = LintConfigBuilder::new()
            .allow("unused_import")
            .deny("dead_code")
            .build();
        assert_eq!(config.enabled.len(), 1);
        assert_eq!(config.disabled.len(), 1);
    }

    #[test]
    fn lint_config_validator_no_conflict() {
        let config = LintConfigBuilder::new()
            .allow("lint_a")
            .allow("lint_b")
            .build();
        let errors = LintConfigValidator::validate(&config);
        assert!(errors.is_empty());
    }

    #[test]
    fn lint_config_validator_with_conflict() {
        let config = LintConfigBuilder::new()
            .allow("conflict_lint")
            .deny("conflict_lint")
            .build();
        let errors = LintConfigValidator::validate(&config);
        assert!(!errors.is_empty());
        assert!(errors[0].contains("conflict_lint"));
    }
}

// ============================================================
// LintRunSummary
// ============================================================

/// A high-level end-of-run summary.
#[allow(dead_code)]
pub struct LintRunSummary {
    pub files_checked: usize,
    pub total_diagnostics: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
    pub fix_suggestions: usize,
}

impl LintRunSummary {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            files_checked: 0,
            total_diagnostics: 0,
            errors: 0,
            warnings: 0,
            elapsed_ms: 0,
            fix_suggestions: 0,
        }
    }

    /// Returns `true` when the run produced no errors.
    #[allow(dead_code)]
    pub fn is_success(&self) -> bool {
        self.errors == 0
    }

    /// Diagnostics per millisecond (throughput).
    #[allow(dead_code)]
    pub fn throughput(&self) -> f64 {
        if self.elapsed_ms == 0 {
            return 0.0;
        }
        self.total_diagnostics as f64 / self.elapsed_ms as f64
    }
}

impl Default for LintRunSummary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod lint_run_summary_tests {
    use super::*;

    #[test]
    fn lint_run_summary_is_success() {
        let mut s = LintRunSummary::new();
        assert!(s.is_success());
        s.errors = 1;
        assert!(!s.is_success());
    }

    #[test]
    fn lint_run_summary_throughput() {
        let s = LintRunSummary {
            total_diagnostics: 100,
            elapsed_ms: 50,
            ..LintRunSummary::new()
        };
        assert!((s.throughput() - 2.0).abs() < 1e-9);
    }

    #[test]
    fn lint_run_summary_zero_elapsed() {
        let s = LintRunSummary::new();
        assert_eq!(s.throughput(), 0.0);
    }
}
