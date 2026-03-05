//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::{HashMap, VecDeque};

/// A single line change within a hunk.
#[derive(Debug, Clone)]
pub struct LineChange {
    /// The kind of change.
    pub kind: ChangeKind,
    /// The line content (without trailing newline).
    pub content: String,
    /// Line number in the old file (for Removed/Unchanged), or 0.
    pub old_lineno: usize,
    /// Line number in the new file (for Added/Unchanged), or 0.
    pub new_lineno: usize,
}
impl LineChange {
    /// Create a new line change.
    pub fn new(kind: ChangeKind, content: String, old_lineno: usize, new_lineno: usize) -> Self {
        Self {
            kind,
            content,
            old_lineno,
            new_lineno,
        }
    }
    /// Return the unified-diff prefix character for this change.
    pub fn prefix_char(&self) -> char {
        match self.kind {
            ChangeKind::Removed => '-',
            ChangeKind::Added => '+',
            ChangeKind::Unchanged => ' ',
        }
    }
    /// Convert to DiffLine.
    pub fn to_diff_line(&self) -> DiffLine {
        let (old, new) = match self.kind {
            ChangeKind::Removed => (Some(self.old_lineno), None),
            ChangeKind::Added => (None, Some(self.new_lineno)),
            ChangeKind::Unchanged => (Some(self.old_lineno), Some(self.new_lineno)),
        };
        DiffLine {
            kind: self.kind.clone().into(),
            content: self.content.clone(),
            old_lineno: old,
            new_lineno: new,
        }
    }
}
/// Extended diff line kind (compatible variant of ChangeKind).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLineKind {
    /// Line was added.
    Added,
    /// Line was removed.
    Removed,
    /// Unchanged context line.
    Context,
}
/// A simplified representation of a top-level declaration for structural comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclSummary {
    /// The kind of declaration (def, theorem, axiom, etc.).
    pub kind: String,
    /// The name of the declaration.
    pub name: String,
    /// The type signature (if visible).
    pub type_sig: String,
}
#[allow(dead_code)]
pub struct DiffWindow {
    diffs: std::collections::VecDeque<(String, DiffResult)>,
    max_size: usize,
}
#[allow(dead_code)]
impl DiffWindow {
    pub fn new(max_size: usize) -> Self {
        Self {
            diffs: std::collections::VecDeque::new(),
            max_size,
        }
    }
    pub fn push(&mut self, label: &str, result: DiffResult) {
        if self.diffs.len() >= self.max_size {
            self.diffs.pop_front();
        }
        self.diffs.push_back((label.to_string(), result));
    }
    pub fn total_additions(&self) -> usize {
        self.diffs.iter().map(|(_, r)| r.additions).sum()
    }
    pub fn total_deletions(&self) -> usize {
        self.diffs.iter().map(|(_, r)| r.deletions).sum()
    }
    pub fn len(&self) -> usize {
        self.diffs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.diffs.is_empty()
    }
    pub fn labels(&self) -> Vec<&str> {
        self.diffs.iter().map(|(l, _)| l.as_str()).collect()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FilePatchHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<FilePatchLine>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DeclItem {
    pub name: String,
    pub kind: DeclItemKind,
    pub line: usize,
    pub body_hash: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FilePatch {
    pub old_file: String,
    pub new_file: String,
    pub hunks: Vec<FilePatchHunk>,
}
#[allow(dead_code)]
impl FilePatch {
    pub fn to_unified_string(&self) -> String {
        let mut out = format!("--- {}\n+++ {}\n", self.old_file, self.new_file);
        for hunk in &self.hunks {
            out.push_str(&format!(
                "@@ -{},{} +{},{} @@\n",
                hunk.old_start, hunk.old_count, hunk.new_start, hunk.new_count
            ));
            for line in &hunk.lines {
                let prefix = match line.kind {
                    FilePatchLineKind::Context => ' ',
                    FilePatchLineKind::Added => '+',
                    FilePatchLineKind::Removed => '-',
                };
                out.push(prefix);
                out.push_str(&line.content);
                out.push('\n');
            }
        }
        out
    }
    pub fn net_change(&self) -> i64 {
        self.hunks
            .iter()
            .map(|h| {
                let adds = h
                    .lines
                    .iter()
                    .filter(|l| l.kind == FilePatchLineKind::Added)
                    .count() as i64;
                let rems = h
                    .lines
                    .iter()
                    .filter(|l| l.kind == FilePatchLineKind::Removed)
                    .count() as i64;
                adds - rems
            })
            .sum()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MultiDiffEntry {
    pub file_path: String,
    pub result: DiffResult,
}
#[allow(dead_code)]
pub struct KeywordFilter {
    pub(super) keyword: String,
}
#[allow(dead_code)]
impl KeywordFilter {
    pub fn new(keyword: &str) -> Self {
        Self {
            keyword: keyword.to_string(),
        }
    }
}
/// The type of change for a single line in a diff.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeKind {
    /// Line exists only in the old text (deletion).
    Removed,
    /// Line exists only in the new text (insertion).
    Added,
    /// Line is the same in both texts (context).
    Unchanged,
}
/// Extended diff line with optional line numbers.
#[derive(Debug, Clone)]
pub struct DiffLine {
    /// Kind of change.
    pub kind: DiffLineKind,
    /// Content of this line.
    pub content: String,
    /// Line number in the old file, if applicable.
    pub old_lineno: Option<usize>,
    /// Line number in the new file, if applicable.
    pub new_lineno: Option<usize>,
}
impl DiffLine {
    /// Create a new diff line.
    pub fn new(
        kind: DiffLineKind,
        content: String,
        old_lineno: Option<usize>,
        new_lineno: Option<usize>,
    ) -> Self {
        Self {
            kind,
            content,
            old_lineno,
            new_lineno,
        }
    }
    /// Return the unified-diff prefix character.
    pub fn prefix_char(&self) -> char {
        match self.kind {
            DiffLineKind::Removed => '-',
            DiffLineKind::Added => '+',
            DiffLineKind::Context => ' ',
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum OxiDiffToken {
    Keyword(String),
    Ident(String),
    Number(String),
    StringLit(String),
    Punct(String),
    Whitespace(String),
    Other(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum WordToken {
    Equal(String),
    Added(String),
    Removed(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BlameSource {
    pub file: String,
    pub lines: Vec<BlameLine>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BlameLine {
    pub line_no: usize,
    pub content: String,
    pub commit: String,
    pub author: String,
}
/// A contiguous group of changes with surrounding context.
#[derive(Debug, Clone)]
pub struct DiffHunk {
    /// Starting line number in the old text (1-based).
    pub old_start: usize,
    /// Number of lines from the old text in this hunk.
    pub old_count: usize,
    /// Starting line number in the new text (1-based).
    pub new_start: usize,
    /// Number of lines from the new text in this hunk.
    pub new_count: usize,
    /// The individual line changes in this hunk.
    pub lines: Vec<DiffLine>,
    /// Precomputed hunk header string.
    pub header: String,
}
impl DiffHunk {
    /// Build the hunk header in unified diff style: `@@ -old_start,old_count +new_start,new_count @@`
    pub fn make_header(
        old_start: usize,
        old_count: usize,
        new_start: usize,
        new_count: usize,
    ) -> String {
        format!(
            "@@ -{},{} +{},{} @@",
            old_start, old_count, new_start, new_count
        )
    }
    /// Return true if this hunk contains at least one actual change.
    pub fn has_changes(&self) -> bool {
        self.lines.iter().any(|l| l.kind != DiffLineKind::Context)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CharDiffToken {
    Equal(char),
    Added(char),
    Removed(char),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FilePatchLine {
    pub kind: FilePatchLineKind,
    pub content: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AnnotatedDiffLine {
    pub line: DiffLine,
    pub function_context: Option<String>,
    pub is_in_proof: bool,
    pub indent_level: usize,
}
#[allow(dead_code)]
pub struct MultiDiff {
    pub entries: Vec<MultiDiffEntry>,
}
#[allow(dead_code)]
impl MultiDiff {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn add(&mut self, path: &str, old: &str, new: &str) {
        let config = DiffConfig::new();
        let result = line_diff(old, new, &config);
        self.entries.push(MultiDiffEntry {
            file_path: path.to_string(),
            result,
        });
    }
    pub fn total_additions(&self) -> usize {
        self.entries.iter().map(|e| e.result.additions).sum()
    }
    pub fn total_deletions(&self) -> usize {
        self.entries.iter().map(|e| e.result.deletions).sum()
    }
    pub fn changed_files(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|e| e.result.additions > 0 || e.result.deletions > 0)
            .map(|e| e.file_path.as_str())
            .collect()
    }
    pub fn summary(&self) -> String {
        format!(
            "{} files: +{} -{} lines",
            self.entries.len(),
            self.total_additions(),
            self.total_deletions()
        )
    }
}
#[allow(dead_code)]
pub struct DiffSummaryReport {
    pub files_checked: usize,
    pub files_changed: usize,
    pub total_additions: usize,
    pub total_deletions: usize,
    pub largest_diff_file: Option<String>,
}
#[allow(dead_code)]
impl DiffSummaryReport {
    pub fn from_multi_diff(md: &MultiDiff) -> Self {
        let total_additions = md.total_additions();
        let total_deletions = md.total_deletions();
        let files_changed = md.changed_files().len();
        let largest = md
            .entries
            .iter()
            .max_by_key(|e| e.result.additions + e.result.deletions)
            .map(|e| e.file_path.clone());
        Self {
            files_checked: md.entries.len(),
            files_changed,
            total_additions,
            total_deletions,
            largest_diff_file: largest,
        }
    }
    pub fn to_string_report(&self) -> String {
        format!(
            "Files: {}/{} changed | Lines: +{} -{} | Largest: {}",
            self.files_changed,
            self.files_checked,
            self.total_additions,
            self.total_deletions,
            self.largest_diff_file.as_deref().unwrap_or("none")
        )
    }
}
/// Configuration for diff operations.
#[derive(Debug, Clone)]
pub struct DiffConfig {
    /// Number of context lines to show around each change.
    pub context_lines: usize,
    /// Whether to ignore differences in whitespace (leading/trailing/multiple).
    pub ignore_whitespace: bool,
    /// Whether to emit ANSI color codes in output.
    pub color_output: bool,
    /// Whether to show line numbers in the output.
    pub show_line_numbers: bool,
    /// Label for the "old" side of the diff.
    pub old_label: String,
    /// Label for the "new" side of the diff.
    pub new_label: String,
}
impl DiffConfig {
    /// Create a config with sensible defaults.
    pub fn new() -> Self {
        Self {
            context_lines: 3,
            ignore_whitespace: false,
            color_output: false,
            show_line_numbers: true,
            old_label: "a".to_string(),
            new_label: "b".to_string(),
        }
    }
    /// Set context lines (builder pattern).
    pub fn with_context(mut self, n: usize) -> Self {
        self.context_lines = n;
        self
    }
    /// Enable/disable whitespace ignoring (builder pattern).
    pub fn with_ignore_whitespace(mut self, ignore: bool) -> Self {
        self.ignore_whitespace = ignore;
        self
    }
    /// Enable/disable color output (builder pattern).
    pub fn with_color(mut self, color: bool) -> Self {
        self.color_output = color;
        self
    }
    /// Set labels for old and new sides (builder pattern).
    pub fn with_labels(mut self, old: impl Into<String>, new: impl Into<String>) -> Self {
        self.old_label = old.into();
        self.new_label = new.into();
        self
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ChangeClass {
    ProofChange,
    TypeChange,
    CommentChange,
    WhitespaceChange,
    StructureChange,
    ImportChange,
    Other,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DeclItemKind {
    Theorem,
    Def,
    Lemma,
    Axiom,
    Structure,
    Class,
    Instance,
    Other,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StructDiff {
    pub added: Vec<DeclItem>,
    pub removed: Vec<DeclItem>,
    pub changed: Vec<(DeclItem, DeclItem)>,
    pub unchanged: Vec<DeclItem>,
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DiffStatistics {
    pub total_lines: usize,
    pub added_lines: usize,
    pub removed_lines: usize,
    pub context_lines: usize,
    pub hunks: usize,
    pub files_changed: usize,
}
#[allow(dead_code)]
impl DiffStatistics {
    pub fn similarity_percent(&self) -> f64 {
        if self.total_lines == 0 {
            return 100.0;
        }
        let changed = self.added_lines + self.removed_lines;
        let sim = 1.0 - (changed as f64 / (2.0 * self.total_lines as f64));
        (sim * 100.0).max(0.0)
    }
}
#[allow(dead_code)]
pub enum DiffDisplayTarget {
    Terminal,
    Html,
    Json,
    Csv,
    Compact,
}
/// The complete result of diffing two texts.
#[derive(Debug, Clone)]
pub struct DiffResult {
    /// The hunks comprising the diff.
    pub hunks: Vec<DiffHunk>,
    /// Total number of lines added.
    pub additions: usize,
    /// Total number of lines removed.
    pub deletions: usize,
}
impl DiffResult {
    /// Return true if there are no differences.
    pub fn is_empty(&self) -> bool {
        self.hunks.is_empty()
    }
    /// Total number of changed lines (additions + deletions).
    pub fn total_changes(&self) -> usize {
        self.additions + self.deletions
    }
}
/// The result of structurally comparing two OxiLean sources.
#[derive(Debug, Clone)]
pub struct StructuralDiffResult {
    /// Declarations present only in the old source.
    pub removed: Vec<DeclSummary>,
    /// Declarations present only in the new source.
    pub added: Vec<DeclSummary>,
    /// Declarations present in both but with different type signatures.
    pub modified: Vec<(DeclSummary, DeclSummary)>,
    /// Declarations that are identical in both.
    pub unchanged: Vec<DeclSummary>,
}
impl StructuralDiffResult {
    /// True if there are no structural differences.
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.added.is_empty() && self.modified.is_empty()
    }
    /// Format as a human-readable summary.
    pub fn summary(&self) -> String {
        let mut out = String::new();
        if !self.removed.is_empty() {
            out.push_str(&format!("Removed ({}):\n", self.removed.len()));
            for d in &self.removed {
                out.push_str(&format!("  - {d}\n"));
            }
        }
        if !self.added.is_empty() {
            out.push_str(&format!("Added ({}):\n", self.added.len()));
            for d in &self.added {
                out.push_str(&format!("  + {d}\n"));
            }
        }
        if !self.modified.is_empty() {
            out.push_str(&format!("Modified ({}):\n", self.modified.len()));
            for (old, new) in &self.modified {
                out.push_str(&format!(
                    "  ~ {} : {} -> {}\n",
                    old.name, old.type_sig, new.type_sig
                ));
            }
        }
        if out.is_empty() {
            out.push_str("No structural differences.\n");
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum FilePatchLineKind {
    Context,
    Added,
    Removed,
}
#[allow(dead_code)]
pub struct DiffCache {
    cache: std::collections::HashMap<(u64, u64), DiffResult>,
}
#[allow(dead_code)]
impl DiffCache {
    pub fn new() -> Self {
        Self {
            cache: std::collections::HashMap::new(),
        }
    }
    pub fn get_or_compute(&mut self, old: &str, new: &str) -> &DiffResult {
        let key = (hash_str(old), hash_str(new));
        self.cache
            .entry(key)
            .or_insert_with(|| line_diff(old, new, &DiffConfig::new()))
    }
    pub fn invalidate(&mut self, old: &str, new: &str) {
        let key = (hash_str(old), hash_str(new));
        self.cache.remove(&key);
    }
    pub fn size(&self) -> usize {
        self.cache.len()
    }
}
