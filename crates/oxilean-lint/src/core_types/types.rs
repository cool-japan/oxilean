//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::framework::{
    AutoFix, LintConfig, LintContext, LintDiagnostic, LintEngine, LintId, LintRegistry, LintRule,
    LintSuppression, Severity,
};

use std::collections::{HashMap, HashSet};

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
                    diag.lint_id.as_str(), diag.severity, file, offset, diag.message
                    .replace('"', "\\\"")
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
/// A persistent store of all known lint rules and their metadata.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct LintDatabase {
    entries: std::collections::HashMap<String, LintEntry>,
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
/// Logs lint events (rule run, diagnostic emitted, fix applied) for debugging.
#[allow(dead_code)]
pub struct LintEventLog {
    events: Vec<LintEvent>,
    counter: u64,
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
        self.items.sort_by_key(|b| std::cmp::Reverse(b.0));
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
            since_version: "0.1.1".to_string(),
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
/// A single lint event.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LintEvent {
    pub id: u64,
    pub kind: LintEventKind,
    pub message: String,
}
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
/// An example of a lint rule firing (good/bad pair).
#[allow(dead_code)]
pub struct LintExample {
    pub title: String,
    pub bad: String,
    pub good: String,
}
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
        if let Some(min) = &self.min_severity {
            if diag.severity > *min {
                return false;
            }
        }
        if !self.include_patterns.is_empty() {
            let matched = self
                .include_patterns
                .iter()
                .any(|p| diag.lint_id.matches_pattern(p));
            if !matched {
                return false;
            }
        }
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
/// Validates lint configuration for consistency.
#[allow(dead_code)]
pub struct LintConfigValidator;
impl LintConfigValidator {
    /// Validate that a config has no conflicting entries.
    /// Returns a list of validation error messages.
    #[allow(dead_code)]
    pub fn validate(config: &LintConfig) -> Vec<String> {
        let mut errors = Vec::new();
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
