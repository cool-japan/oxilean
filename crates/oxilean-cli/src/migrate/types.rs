//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A deprecated API rewrite rule.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ApiRewrite {
    pub old: String,
    pub new: String,
    pub message: Option<String>,
}
impl ApiRewrite {
    pub fn new(old: impl Into<String>, new: impl Into<String>) -> Self {
        Self {
            old: old.into(),
            new: new.into(),
            message: None,
        }
    }
    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.message = Some(msg.into());
        self
    }
    pub fn apply_to_line(&self, line: &str) -> (String, usize) {
        if !line.contains(self.old.as_str()) {
            return (line.to_string(), 0);
        }
        let trimmed = line.trim_start();
        if trimmed.starts_with("--") {
            return (line.to_string(), 0);
        }
        let count = line.matches(self.old.as_str()).count();
        let result = line.replace(self.old.as_str(), &self.new);
        (result, count)
    }
    pub fn apply_to_source(&self, source: &str) -> (String, usize) {
        let mut total = 0;
        let lines: Vec<String> = source
            .lines()
            .map(|line| {
                let (new_line, count) = self.apply_to_line(line);
                total += count;
                new_line
            })
            .collect();
        let result = lines.join("\n");
        let result = if source.ends_with('\n') && !result.ends_with('\n') {
            result + "\n"
        } else {
            result
        };
        (result, total)
    }
}
/// A snapshot of a file's content before migration.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct MigrationSnapshot {
    pub path: PathBuf,
    pub original_content: String,
    pub migrated_content: String,
    pub applications: Vec<RuleApplication>,
}
impl MigrationSnapshot {
    pub fn new(
        path: PathBuf,
        original: String,
        migrated: String,
        applications: Vec<RuleApplication>,
    ) -> Self {
        Self {
            path,
            original_content: original,
            migrated_content: migrated,
            applications,
        }
    }
    pub fn has_changes(&self) -> bool {
        self.original_content != self.migrated_content
    }
    pub fn total_changes(&self) -> usize {
        self.applications.iter().map(|a| a.changes).sum()
    }
    pub fn rollback_content(&self) -> &str {
        &self.original_content
    }
}
/// Result of applying a single migration rule to a piece of text.
#[derive(Debug, Clone)]
pub struct RuleApplication {
    /// Name of the rule.
    pub rule_name: String,
    /// Number of changes made by this rule.
    pub changes: usize,
}
/// Configuration for a migration run.
#[derive(Debug, Clone)]
pub struct MigrationConfig {
    /// Directory containing the Lean 4 source files to migrate.
    pub source_dir: PathBuf,
    /// Directory where migrated OxiLean files will be written.
    pub target_dir: PathBuf,
    /// If true, report what would change without writing files.
    pub dry_run: bool,
    /// If true, emit detailed progress information.
    pub verbose: bool,
    /// File extensions to consider (default: `[".lean"]`).
    pub extensions: Vec<String>,
    /// Whether to overwrite existing files in the target directory.
    pub overwrite: bool,
}
impl MigrationConfig {
    /// Create a new config with sensible defaults.
    pub fn new(source_dir: impl Into<PathBuf>, target_dir: impl Into<PathBuf>) -> Self {
        Self {
            source_dir: source_dir.into(),
            target_dir: target_dir.into(),
            dry_run: false,
            verbose: false,
            extensions: vec![".lean".to_string()],
            overwrite: false,
        }
    }
    /// Enable dry-run mode (builder pattern).
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }
    /// Enable verbose mode (builder pattern).
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    /// Set overwrite mode (builder pattern).
    pub fn with_overwrite(mut self, overwrite: bool) -> Self {
        self.overwrite = overwrite;
        self
    }
    /// Check whether a filename should be migrated based on configured extensions.
    pub fn should_migrate(&self, path: &Path) -> bool {
        if let Some(ext_os) = path.extension() {
            let ext = format!(".{}", ext_os.to_string_lossy());
            self.extensions.iter().any(|e| e == &ext)
        } else {
            false
        }
    }
}
#[derive(Debug, Default)]
pub struct VersionMigrationChain {
    steps: Vec<VersionMigrationStep>,
}
impl VersionMigrationChain {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_step(&mut self, step: VersionMigrationStep) {
        self.steps.push(step);
    }
    pub fn apply(&self, source: &str) -> (String, usize) {
        let mut text = source.to_string();
        let mut total = 0;
        for step in &self.steps {
            let (new_text, count) = step.apply(&text);
            text = new_text;
            total += count;
        }
        (text, total)
    }
    pub fn len(&self) -> usize {
        self.steps.len()
    }
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
    pub fn summary(&self) -> String {
        let mut out = String::from("Version migration chain:\n");
        for step in &self.steps {
            out.push_str(&format!("  {} -> {}\n", step.from, step.to));
        }
        out
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractiveChoice {
    Accept,
    Reject,
    AcceptAll,
    RejectAll,
    Quit,
}
/// Source language for migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum SourceLanguage {
    /// Lean 4
    Lean4,
    /// Lean 3 (mathlib-era)
    Lean3,
    /// Coq / Rocq
    Coq,
    /// Agda
    Agda,
    /// OxiLean (version-to-version migration)
    OxiLean,
}
impl SourceLanguage {
    /// Return a human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            SourceLanguage::Lean4 => "Lean 4",
            SourceLanguage::Lean3 => "Lean 3",
            SourceLanguage::Coq => "Coq",
            SourceLanguage::Agda => "Agda",
            SourceLanguage::OxiLean => "OxiLean",
        }
    }
    /// Typical file extensions for this language.
    pub fn extensions(&self) -> &'static [&'static str] {
        match self {
            SourceLanguage::Lean4 | SourceLanguage::Lean3 => &[".lean"],
            SourceLanguage::Coq => &[".v"],
            SourceLanguage::Agda => &[".agda"],
            SourceLanguage::OxiLean => &[".oxilean"],
        }
    }
}
/// A migration session that tracks snapshots for rollback.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct MigrationSession {
    pub snapshots: Vec<MigrationSnapshot>,
    pub committed: bool,
}
impl MigrationSession {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn process(
        &mut self,
        path: PathBuf,
        source: String,
        registry: &RuleRegistry,
    ) -> &MigrationSnapshot {
        let (migrated, apps) = apply_all_rules(&source, registry);
        let snap = MigrationSnapshot::new(path, source, migrated, apps);
        self.snapshots.push(snap);
        self.snapshots
            .last()
            .expect("snapshot was just pushed to self.snapshots")
    }
    pub fn commit(&mut self) -> usize {
        let mut written = 0;
        for snap in &self.snapshots {
            if snap.has_changes() {
                if std::fs::write(&snap.path, &snap.migrated_content).is_ok() {
                    written += 1;
                }
            }
        }
        self.committed = true;
        written
    }
    pub fn rollback(&mut self) -> usize {
        let mut restored = 0;
        for snap in &self.snapshots {
            if snap.has_changes() {
                if std::fs::write(&snap.path, snap.rollback_content()).is_ok() {
                    restored += 1;
                }
            }
        }
        self.committed = false;
        restored
    }
    pub fn dry_run_report(&self) -> String {
        let mut out = String::from("Dry-run migration report:\n");
        for snap in &self.snapshots {
            out.push_str(&format!(
                "  {} : {} change(s)\n",
                snap.path.display(),
                snap.total_changes()
            ));
        }
        let total: usize = self.snapshots.iter().map(|s| s.total_changes()).sum();
        out.push_str(&format!(
            "Total: {} change(s) across {} file(s)\n",
            total,
            self.snapshots.len()
        ));
        out
    }
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ValidationMessage {
    pub severity: ValidationSeverity,
    pub text: String,
    pub line: Option<usize>,
}
impl ValidationMessage {
    pub fn error(text: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Error,
            text: text.into(),
            line: None,
        }
    }
    pub fn warning(text: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Warning,
            text: text.into(),
            line: None,
        }
    }
    pub fn note(text: impl Into<String>) -> Self {
        Self {
            severity: ValidationSeverity::Note,
            text: text.into(),
            line: None,
        }
    }
    pub fn at_line(mut self, line: usize) -> Self {
        self.line = Some(line);
        self
    }
}
/// Registry of migration rules, sorted by priority.
#[derive(Debug)]
pub struct RuleRegistry {
    rules: Vec<MigrationRule>,
}
impl RuleRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    /// Create a registry populated with all built-in rules.
    pub fn with_builtins() -> Self {
        let mut reg = Self::new();
        reg.add(MigrationRule::new(
            "fat_arrow",
            "Replace => with ->",
            RulePriority::Normal,
            transform_fat_arrow,
        ));
        reg.add(MigrationRule::new(
            "cons",
            "Replace :: with List.cons",
            RulePriority::Normal,
            transform_cons,
        ));
        reg.add(MigrationRule::new(
            "append",
            "Replace ++ with List.append",
            RulePriority::Normal,
            transform_append,
        ));
        reg.add(MigrationRule::new(
            "divides",
            "Replace divides (U+2223) with Nat.dvd",
            RulePriority::Low,
            transform_divides,
        ));
        reg.add(MigrationRule::new(
            "mapsto",
            "Replace mapsto (U+21A6) with ->",
            RulePriority::Low,
            transform_mapsto,
        ));
        reg.add(MigrationRule::new(
            "import_ext",
            "Rename .lean imports to .oxilean",
            RulePriority::High,
            transform_import_extension,
        ));
        reg.add(MigrationRule::new(
            "hash_commands",
            "Lowercase #Check / #Eval / #Print",
            RulePriority::Low,
            transform_hash_commands,
        ));
        reg
    }
    /// Add a rule.  The registry stays sorted by priority after insertion.
    pub fn add(&mut self, rule: MigrationRule) {
        self.rules.push(rule);
        self.rules.sort_by_key(|r| r.priority);
    }
    /// Number of registered rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Iterate over the rules in priority order.
    pub fn iter(&self) -> impl Iterator<Item = &MigrationRule> {
        self.rules.iter()
    }
    /// Look up a rule by name.
    pub fn get(&self, name: &str) -> Option<&MigrationRule> {
        self.rules.iter().find(|r| r.name == name)
    }
    /// Remove a rule by name.  Returns true if removed.
    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.rules.len();
        self.rules.retain(|r| r.name != name);
        self.rules.len() < before
    }
}
#[derive(Debug, Clone, Default)]
pub struct ExtendedMigrationReport {
    pub base: MigrationReport,
    pub file_details: Vec<FileMigrationDetail>,
}
impl ExtendedMigrationReport {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record_file_migration(
        &mut self,
        path: PathBuf,
        apps: &[RuleApplication],
        had_changes: bool,
        validation_passed: Option<bool>,
    ) {
        self.base.record_file(apps, had_changes);
        let rule_changes: Vec<(String, usize)> = apps
            .iter()
            .filter(|a| a.changes > 0)
            .map(|a| (a.rule_name.clone(), a.changes))
            .collect();
        let total_changes = rule_changes.iter().map(|(_, c)| c).sum();
        self.file_details.push(FileMigrationDetail {
            path,
            total_changes,
            rule_changes,
            validation: validation_passed,
        });
    }
    pub fn detailed_summary(&self) -> String {
        let mut out = self.base.summary();
        if !self.file_details.is_empty() {
            out.push_str("  File details:\n");
            for detail in &self.file_details {
                out.push_str(&format!(
                    "    {} : {} change(s)",
                    detail.path.display(),
                    detail.total_changes
                ));
                if let Some(val) = detail.validation {
                    out.push_str(if val {
                        " [validated OK]"
                    } else {
                        " [validation FAILED]"
                    });
                }
                out.push('\n');
                for (rule, count) in &detail.rule_changes {
                    out.push_str(&format!("      - {}: {}\n", rule, count));
                }
            }
        }
        out
    }
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ValidationResult {
    pub path: PathBuf,
    pub passed: bool,
    pub messages: Vec<ValidationMessage>,
}
impl ValidationResult {
    pub fn pass(path: PathBuf) -> Self {
        Self {
            path,
            passed: true,
            messages: Vec::new(),
        }
    }
    pub fn fail(path: PathBuf, msg: impl Into<String>) -> Self {
        Self {
            path,
            passed: false,
            messages: vec![ValidationMessage::error(msg)],
        }
    }
    pub fn add_message(&mut self, msg: ValidationMessage) {
        if msg.severity == ValidationSeverity::Error {
            self.passed = false;
        }
        self.messages.push(msg);
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    Error,
    Warning,
    Note,
}
/// Priority level for migration rules.  Lower values run first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RulePriority {
    /// Must run before all other rules (e.g. escaping).
    First = 0,
    /// High priority (e.g. block-level transforms).
    High = 10,
    /// Normal priority (most syntactic replacements).
    Normal = 50,
    /// Low priority (cosmetic / cleanup).
    Low = 90,
    /// Must run after all other rules.
    Last = 100,
}
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
impl Version {
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;
        Some(Self {
            major,
            minor,
            patch,
        })
    }
}
#[derive(Debug)]
#[allow(dead_code)]
pub struct VersionMigrationStep {
    pub from: Version,
    pub to: Version,
    pub rewrites: ApiRewriteSet,
    pub rules: Option<RuleRegistry>,
}
impl VersionMigrationStep {
    pub fn new(from: Version, to: Version, rewrites: ApiRewriteSet) -> Self {
        Self {
            from,
            to,
            rewrites,
            rules: None,
        }
    }
    pub fn with_rules(mut self, rules: RuleRegistry) -> Self {
        self.rules = Some(rules);
        self
    }
    pub fn apply(&self, source: &str) -> (String, usize) {
        let (text, count) = self.rewrites.apply_to_source(source);
        if let Some(ref rules) = self.rules {
            let (text2, apps) = apply_all_rules(&text, rules);
            let extra: usize = apps.iter().map(|a| a.changes).sum();
            return (text2, count + extra);
        }
        (text, count)
    }
}
/// Summary report for a migration run.
#[derive(Debug, Clone)]
pub struct MigrationReport {
    /// Total number of files that were scanned.
    pub files_processed: usize,
    /// Total number of textual changes made (across all files and rules).
    pub changes_made: usize,
    /// Number of files that had at least one change.
    pub files_changed: usize,
    /// Number of files that were skipped (e.g. not matching extensions).
    pub files_skipped: usize,
    /// Errors encountered during migration, keyed by file path.
    pub errors: Vec<(PathBuf, String)>,
    /// Per-rule change counts.
    pub rule_counts: HashMap<String, usize>,
}
impl MigrationReport {
    /// Create an empty report.
    pub fn new() -> Self {
        Self {
            files_processed: 0,
            changes_made: 0,
            files_changed: 0,
            files_skipped: 0,
            errors: Vec::new(),
            rule_counts: HashMap::new(),
        }
    }
    /// Record the results of processing a single file.
    pub fn record_file(&mut self, apps: &[RuleApplication], had_changes: bool) {
        self.files_processed += 1;
        if had_changes {
            self.files_changed += 1;
        }
        for app in apps {
            self.changes_made += app.changes;
            *self.rule_counts.entry(app.rule_name.clone()).or_insert(0) += app.changes;
        }
    }
    /// Record a skipped file.
    pub fn record_skip(&mut self) {
        self.files_skipped += 1;
    }
    /// Record an error for a file.
    pub fn record_error(&mut self, path: PathBuf, message: String) {
        self.errors.push((path, message));
    }
    /// Whether the migration completed without any errors.
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }
    /// Format the report as a human-readable summary.
    pub fn summary(&self) -> String {
        let mut out = String::new();
        out.push_str("Migration Report\n");
        out.push_str(&format!("  Files processed: {}\n", self.files_processed));
        out.push_str(&format!("  Files changed:   {}\n", self.files_changed));
        out.push_str(&format!("  Files skipped:   {}\n", self.files_skipped));
        out.push_str(&format!("  Total changes:   {}\n", self.changes_made));
        if !self.rule_counts.is_empty() {
            out.push_str("  Per-rule changes:\n");
            let mut sorted: Vec<_> = self.rule_counts.iter().collect();
            sorted.sort_by_key(|(name, _)| (*name).clone());
            for (name, count) in sorted {
                out.push_str(&format!("    {name}: {count}\n"));
            }
        }
        if !self.errors.is_empty() {
            out.push_str(&format!("  Errors ({}):\n", self.errors.len()));
            for (path, msg) in &self.errors {
                out.push_str(&format!("    {}: {}\n", path.display(), msg));
            }
        }
        out
    }
}
/// A collection of API rewrites.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct ApiRewriteSet {
    pub rewrites: Vec<ApiRewrite>,
    pub description: String,
}
impl ApiRewriteSet {
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            rewrites: Vec::new(),
            description: description.into(),
        }
    }
    pub fn add(&mut self, rewrite: ApiRewrite) {
        self.rewrites.push(rewrite);
    }
    pub fn apply_to_source(&self, source: &str) -> (String, usize) {
        let mut text = source.to_string();
        let mut total = 0;
        for rewrite in &self.rewrites {
            let (new_text, count) = rewrite.apply_to_source(&text);
            text = new_text;
            total += count;
        }
        (text, total)
    }
    pub fn describe(&self) -> String {
        let mut s = format!("API Rewrite Set: {}\n", self.description);
        for rw in &self.rewrites {
            s.push_str(&format!("  {} -> {}", rw.old, rw.new));
            if let Some(ref msg) = rw.message {
                s.push_str(&format!(" ({})", msg));
            }
            s.push('\n');
        }
        s
    }
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProposedChange {
    pub line: usize,
    pub rule_name: String,
    pub original: String,
    pub proposed: String,
}
impl ProposedChange {
    pub fn new(
        line: usize,
        rule_name: impl Into<String>,
        original: impl Into<String>,
        proposed: impl Into<String>,
    ) -> Self {
        Self {
            line,
            rule_name: rule_name.into(),
            original: original.into(),
            proposed: proposed.into(),
        }
    }
    pub fn display(&self) -> String {
        format!(
            "Line {}: [{rule}]\n  - {original}\n  + {proposed}",
            self.line,
            rule = self.rule_name,
            original = self.original,
            proposed = self.proposed
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationStatus {
    Pending,
    Applied,
    Skipped(String),
    Failed(String),
}
#[allow(dead_code)]
impl MigrationStatus {
    pub fn is_success(&self) -> bool {
        matches!(self, MigrationStatus::Applied | MigrationStatus::Skipped(_))
    }
    pub fn display(&self) -> String {
        match self {
            MigrationStatus::Pending => "pending".to_string(),
            MigrationStatus::Applied => "applied".to_string(),
            MigrationStatus::Skipped(reason) => format!("skipped ({})", reason),
            MigrationStatus::Failed(err) => format!("failed: {}", err),
        }
    }
}
/// A single migration rule that transforms source text.
#[derive(Clone)]
pub struct MigrationRule {
    /// Human-readable name for diagnostics.
    pub name: String,
    /// Short description of what the rule does.
    pub description: String,
    /// Execution priority (lower = earlier).
    pub priority: RulePriority,
    /// The transformation function: takes a line, returns the rewritten line
    /// plus a count of changes made.
    transform: fn(&str) -> (String, usize),
}
impl MigrationRule {
    /// Create a new rule.
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        priority: RulePriority,
        transform: fn(&str) -> (String, usize),
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            priority,
            transform,
        }
    }
    /// Apply this rule to a single line.
    pub fn apply(&self, line: &str) -> (String, usize) {
        (self.transform)(line)
    }
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FileMigrationDetail {
    pub path: PathBuf,
    pub total_changes: usize,
    pub rule_changes: Vec<(String, usize)>,
    pub validation: Option<bool>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MigrationRecord {
    pub version: u32,
    pub name: String,
    pub status: MigrationStatus,
    pub applied_at_ms: Option<u64>,
}
#[allow(dead_code)]
impl MigrationRecord {
    pub fn new(version: u32, name: &str) -> Self {
        Self {
            version,
            name: name.to_string(),
            status: MigrationStatus::Pending,
            applied_at_ms: None,
        }
    }
    pub fn mark_applied(&mut self, timestamp_ms: u64) {
        self.status = MigrationStatus::Applied;
        self.applied_at_ms = Some(timestamp_ms);
    }
    pub fn mark_failed(&mut self, error: &str) {
        self.status = MigrationStatus::Failed(error.to_string());
    }
    pub fn mark_skipped(&mut self, reason: &str) {
        self.status = MigrationStatus::Skipped(reason.to_string());
    }
}
