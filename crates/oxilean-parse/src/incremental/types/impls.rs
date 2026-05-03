//! Auto-generated module (split from types.rs)
//!
//! Second half of type definitions and impl blocks.

use super::super::functions::*;
use super::defs::*;
use std::collections::HashMap;
use std::ops::Range;

/// Represents the "reachability" of tokens from a parse entry point.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TokenReachability {
    reachable: std::collections::HashSet<usize>,
}
impl TokenReachability {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            reachable: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn mark_reachable(&mut self, offset: usize) {
        self.reachable.insert(offset);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_reachable(&self, offset: usize) -> bool {
        self.reachable.contains(&offset)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn reachable_count(&self) -> usize {
        self.reachable.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn coverage_fraction(&self, total_tokens: usize) -> f64 {
        if total_tokens == 0 {
            0.0
        } else {
            self.reachable.len() as f64 / total_tokens as f64
        }
    }
}
/// A pool of parse fibers for parallel/concurrent parsing.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FiberPool {
    fibers: Vec<ParseFiber>,
    next_id: u64,
}
impl FiberPool {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            fibers: Vec::new(),
            next_id: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn spawn(&mut self, start: usize, depth: usize, state: impl Into<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.fibers.push(ParseFiber::new(id, start, depth, state));
        id
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn get(&self, id: u64) -> Option<&ParseFiber> {
        self.fibers.iter().find(|f| f.id == id)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn remove(&mut self, id: u64) {
        self.fibers.retain(|f| f.id != id);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn active_count(&self) -> usize {
        self.fibers.len()
    }
}
/// Manages multiple snapshots with a limit.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SnapshotManager {
    snapshots: Vec<ParseSnapshot>,
    max_snapshots: usize,
}
impl SnapshotManager {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(max_snapshots: usize) -> Self {
        Self {
            snapshots: Vec::new(),
            max_snapshots,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn save(&mut self, snapshot: ParseSnapshot) {
        if self.snapshots.len() >= self.max_snapshots {
            self.snapshots.remove(0);
        }
        self.snapshots.push(snapshot);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn best(&self) -> Option<&ParseSnapshot> {
        self.snapshots.iter().min_by_key(|s| s.error_count)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn latest(&self) -> Option<&ParseSnapshot> {
        self.snapshots.last()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.snapshots.len()
    }
}
/// A simple edit buffer that accumulates edits before applying them.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct EditBuffer {
    pending: Vec<SourceEdit>,
    max_pending: usize,
}
impl EditBuffer {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(max_pending: usize) -> Self {
        Self {
            pending: Vec::new(),
            max_pending,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, edit: SourceEdit) -> bool {
        if self.pending.len() >= self.max_pending {
            return false;
        }
        self.pending.push(edit);
        true
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn flush(&mut self) -> Vec<SourceEdit> {
        std::mem::take(&mut self.pending)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_delta(&self) -> i64 {
        self.pending.iter().map(|e| e.delta()).sum()
    }
}
/// A session-level incremental parse manager.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct IncrementalSession {
    pub source: SimpleRope,
    pub version: ParseVersion,
    pub errors: IncrementalErrorMap,
    pub stats: IncrParseStats,
}
impl IncrementalSession {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: SimpleRope::new(source),
            version: ParseVersion::new(),
            errors: IncrementalErrorMap::new(),
            stats: IncrParseStats::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn apply_edit(&mut self, edit: SourceEdit) {
        let v = self.version.increment();
        let _ = v;
        let start = edit.start;
        let end = edit.end + edit.new_text.len();
        self.errors.clear_range(start, end);
        self.stats.total_edits += 1;
        let delta = edit.delta();
        let src = self.source.as_string();
        let new_src = apply_edits(&src, &[edit]);
        self.source = SimpleRope::new(new_src);
        let _ = delta;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn source_text(&self) -> String {
        self.source.as_string()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_errors(&self) -> bool {
        self.errors.total_error_count() > 0
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn current_version(&self) -> u64 {
        self.version.current()
    }
}
/// Represents the set of changed lines in a diff.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LineDiff {
    changed_lines: Vec<(usize, String, String)>,
}
impl LineDiff {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            changed_lines: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_change(&mut self, line: usize, old: impl Into<String>, new: impl Into<String>) {
        self.changed_lines.push((line, old.into(), new.into()));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.changed_lines.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn affected_lines(&self) -> Vec<usize> {
        self.changed_lines.iter().map(|(l, _, _)| *l).collect()
    }
}
/// A fingerprint computed from a slice of tokens.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct TokenFingerprint(u64);
impl TokenFingerprint {
    #[allow(missing_docs)]
    pub fn compute(tokens: &[&str]) -> Self {
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for tok in tokens {
            for byte in tok.bytes() {
                hash ^= byte as u64;
                hash = hash.wrapping_mul(0x0000_0100_0000_01B3);
            }
            hash ^= 0x1f;
        }
        TokenFingerprint(hash)
    }
    #[allow(missing_docs)]
    pub fn value(&self) -> u64 {
        self.0
    }
}
/// A transaction groups multiple `TextChange`s into an atomic unit.
#[allow(missing_docs)]
pub struct Transaction {
    changes: Vec<TextChange>,
    snapshot: Option<String>,
}
impl Transaction {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            changes: Vec::new(),
            snapshot: None,
        }
    }
    #[allow(missing_docs)]
    pub fn begin(source: &str) -> Self {
        Self {
            changes: Vec::new(),
            snapshot: Some(source.to_string()),
        }
    }
    #[allow(missing_docs)]
    pub fn add(&mut self, change: TextChange) {
        self.changes.push(change);
    }
    #[allow(missing_docs)]
    pub fn commit(&self, source: &str) -> String {
        let mut s = source.to_string();
        let mut sorted = self.changes.clone();
        sorted.sort_by_key(|b| std::cmp::Reverse(b.range.start));
        for change in &sorted {
            s = change.apply(&s);
        }
        s
    }
    #[allow(missing_docs)]
    pub fn rollback(&self) -> Option<&str> {
        self.snapshot.as_deref()
    }
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.changes.len()
    }
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }
}
/// A map of byte offset to error messages for incremental error tracking.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct IncrementalErrorMap {
    errors: std::collections::BTreeMap<usize, Vec<String>>,
}
impl IncrementalErrorMap {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            errors: std::collections::BTreeMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_error(&mut self, offset: usize, msg: impl Into<String>) {
        self.errors.entry(offset).or_default().push(msg.into());
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn clear_range(&mut self, start: usize, end: usize) {
        let keys: Vec<_> = self.errors.range(start..end).map(|(&k, _)| k).collect();
        for k in keys {
            self.errors.remove(&k);
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn errors_in_range(&self, start: usize, end: usize) -> Vec<&String> {
        self.errors
            .range(start..end)
            .flat_map(|(_, msgs)| msgs.iter())
            .collect()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_error_count(&self) -> usize {
        self.errors.values().map(|v| v.len()).sum()
    }
}
/// A cached parse result for one declaration
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct ParsedDecl {
    pub source_range: Range<usize>,
    pub name: Option<String>,
    pub decl_text: String,
    pub valid: bool,
}
/// The incremental parser state — tracks source + cache
#[allow(missing_docs)]
pub struct IncrementalParser {
    source: String,
    cache: HashMap<usize, ParsedDecl>,
    dirty_ranges: Vec<Range<usize>>,
    version: u32,
}
impl IncrementalParser {
    #[allow(missing_docs)]
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        let mut parser = IncrementalParser {
            source,
            cache: HashMap::new(),
            dirty_ranges: Vec::new(),
            version: 0,
        };
        parser.reparse_dirty();
        parser
    }
    #[allow(missing_docs)]
    pub fn apply_change(&mut self, change: TextChange) {
        let affected_start = change.range.start;
        let affected_end = change.range.start + change.new_text.len();
        self.source = change.apply(&self.source);
        self.version += 1;
        let dirty_end = affected_end.max(change.range.end);
        self.mark_dirty(affected_start..dirty_end);
    }
    #[allow(missing_docs)]
    pub fn apply_changes(&mut self, mut changes: Vec<TextChange>) {
        changes.sort_by_key(|b| std::cmp::Reverse(b.range.start));
        for change in changes {
            self.apply_change(change);
        }
    }
    #[allow(missing_docs)]
    pub fn source(&self) -> &str {
        &self.source
    }
    #[allow(missing_docs)]
    pub fn version(&self) -> u32 {
        self.version
    }
    #[allow(missing_docs)]
    pub fn split_declarations(source: &str) -> Vec<(usize, &str)> {
        let keywords = [
            "def ",
            "theorem ",
            "axiom ",
            "inductive ",
            "structure ",
            "class ",
        ];
        let mut result = Vec::new();
        let mut current_start: Option<usize> = None;
        let mut pos = 0usize;
        for line in source.split_inclusive('\n') {
            let is_decl_start = keywords.iter().any(|kw| line.starts_with(kw));
            if is_decl_start {
                if let Some(start) = current_start {
                    result.push((start, &source[start..pos]));
                }
                current_start = Some(pos);
            }
            pos += line.len();
        }
        if let Some(start) = current_start {
            result.push((start, &source[start..]));
        }
        result
    }
    #[allow(missing_docs)]
    pub fn reparse_dirty(&mut self) {
        let source = self.source.clone();
        let decls = Self::split_declarations(&source);
        for (start, text) in decls {
            let end = start + text.len();
            let range = start..end;
            if self.dirty_ranges.is_empty() || self.is_dirty(&range) {
                let name = Self::extract_decl_name(text);
                let entry = ParsedDecl {
                    source_range: range,
                    name,
                    decl_text: text.to_string(),
                    valid: true,
                };
                self.cache.insert(start, entry);
            }
        }
        self.clear_dirty();
    }
    #[allow(missing_docs)]
    pub fn declarations(&self) -> Vec<&ParsedDecl> {
        let mut decls: Vec<&ParsedDecl> = self.cache.values().collect();
        decls.sort_by_key(|d| d.source_range.start);
        decls
    }
    #[allow(missing_docs)]
    pub fn decl_at(&self, offset: usize) -> Option<&ParsedDecl> {
        self.cache
            .values()
            .find(|d| d.source_range.contains(&offset))
    }
    fn mark_dirty(&mut self, range: Range<usize>) {
        for decl in self.cache.values_mut() {
            if decl.source_range.start < range.end && decl.source_range.end > range.start {
                decl.valid = false;
            }
        }
        self.dirty_ranges.push(range);
    }
    fn is_dirty(&self, range: &Range<usize>) -> bool {
        self.dirty_ranges
            .iter()
            .any(|d| d.start < range.end && d.end > range.start)
    }
    fn clear_dirty(&mut self) {
        self.dirty_ranges.clear();
    }
    #[allow(missing_docs)]
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
    #[allow(missing_docs)]
    pub fn dirty_count(&self) -> usize {
        self.dirty_ranges.len()
    }
    fn extract_decl_name(text: &str) -> Option<String> {
        let keywords = [
            "def ",
            "theorem ",
            "axiom ",
            "inductive ",
            "structure ",
            "class ",
        ];
        for kw in &keywords {
            if let Some(rest) = text.strip_prefix(kw) {
                let name: String = rest
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '\'')
                    .collect();
                if !name.is_empty() {
                    return Some(name);
                }
            }
        }
        None
    }
    #[allow(missing_docs)]
    pub fn invalid_declarations(&self) -> Vec<&ParsedDecl> {
        let mut decls: Vec<&ParsedDecl> = self.cache.values().filter(|d| !d.valid).collect();
        decls.sort_by_key(|d| d.source_range.start);
        decls
    }
    #[allow(missing_docs)]
    pub fn invalidate_by_name(&mut self, name: &str) {
        for decl in self.cache.values_mut() {
            if decl.name.as_deref() == Some(name) {
                decl.valid = false;
            }
        }
    }
}
/// A reparse request indicating which region to re-parse.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ReparseRequest {
    pub start_byte: usize,
    pub end_byte: usize,
    pub source_version: u64,
    pub priority: ReparsePriority,
}
impl ReparseRequest {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(start: usize, end: usize, version: u64) -> Self {
        Self {
            start_byte: start,
            end_byte: end,
            source_version: version,
            priority: ReparsePriority::Normal,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_priority(mut self, p: ReparsePriority) -> Self {
        self.priority = p;
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn byte_span(&self) -> usize {
        self.end_byte.saturating_sub(self.start_byte)
    }
}
/// Incremental parse cache: maps dirty-region hashes to parse results.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct IncrementalParseCache {
    entries: std::collections::HashMap<u64, IncrParseEntry>,
    max_entries: usize,
    hits: u64,
    misses: u64,
}
impl IncrementalParseCache {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_entries,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&mut self, region_hash: u64) -> Option<&IncrParseEntry> {
        if self.entries.contains_key(&region_hash) {
            self.hits += 1;
            self.entries.get(&region_hash)
        } else {
            self.misses += 1;
            None
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn store(&mut self, entry: IncrParseEntry) {
        if self.entries.len() >= self.max_entries {
            if let Some(&k) = self.entries.keys().next() {
                self.entries.remove(&k);
            }
        }
        self.entries.insert(entry.region_hash, entry);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn stats(&self) -> (u64, u64) {
        (self.hits, self.misses)
    }
}
/// A "change detector" that tracks whether a portion of source has changed.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ChangeDetector {
    hashes: std::collections::HashMap<(usize, usize), u64>,
}
impl ChangeDetector {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            hashes: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record(&mut self, source: &str, start: usize, end: usize) {
        let end = end.min(source.len());
        let start = start.min(end);
        let h = {
            let data = &source.as_bytes()[start..end];
            let mut hash = 14695981039346656037u64;
            for &b in data {
                hash = hash.wrapping_mul(1099511628211u64) ^ b as u64;
            }
            hash
        };
        self.hashes.insert((start, end), h);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_changed(&self, source: &str, start: usize, end: usize) -> bool {
        let end = end.min(source.len());
        let start = start.min(end);
        let current = {
            let data = &source.as_bytes()[start..end];
            let mut hash = 14695981039346656037u64;
            for &b in data {
                hash = hash.wrapping_mul(1099511628211u64) ^ b as u64;
            }
            hash
        };
        self.hashes
            .get(&(start, end))
            .map_or(true, |&stored| stored != current)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn recorded_count(&self) -> usize {
        self.hashes.len()
    }
}
/// Represents a snapshot of incremental parse state for rollback.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ParseSnapshot {
    pub source: String,
    pub version: u64,
    pub node_count: usize,
    pub error_count: usize,
}
impl ParseSnapshot {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn capture(source: &str, version: u64, node_count: usize, error_count: usize) -> Self {
        Self {
            source: source.to_string(),
            version,
            node_count,
            error_count,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_cleaner_than(&self, other: &Self) -> bool {
        self.error_count < other.error_count
    }
}
/// Tracks which declarations are affected by an edit.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct DeclDependencyTracker {
    decl_ranges: Vec<(String, usize, usize)>,
}
impl DeclDependencyTracker {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            decl_ranges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn register_decl(&mut self, name: impl Into<String>, start: usize, end: usize) {
        self.decl_ranges.push((name.into(), start, end));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn affected_by_edit(&self, edit: &SourceEdit) -> Vec<&str> {
        self.decl_ranges
            .iter()
            .filter(|(_, s, e)| edit.start < *e && edit.end > *s)
            .map(|(n, _, _)| n.as_str())
            .collect()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn decl_count(&self) -> usize {
        self.decl_ranges.len()
    }
}
/// Statistics for an incremental parsing session.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Default, Debug)]
pub struct IncrParseStats {
    pub total_edits: u64,
    pub partial_reparses: u64,
    pub full_reparses: u64,
    pub tokens_reused: u64,
    #[allow(missing_docs)]
    pub tokens_relexed: u64,
    pub nodes_reused: u64,
    pub nodes_rebuilt: u64,
}
impl IncrParseStats {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn reuse_fraction_tokens(&self) -> f64 {
        let total = self.tokens_reused + self.tokens_relexed;
        if total == 0 {
            0.0
        } else {
            self.tokens_reused as f64 / total as f64
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn reuse_fraction_nodes(&self) -> f64 {
        let total = self.nodes_reused + self.nodes_rebuilt;
        if total == 0 {
            0.0
        } else {
            self.nodes_reused as f64 / total as f64
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn summary(&self) -> String {
        format!(
            "edits={} partial={} full={} token_reuse={:.1}% node_reuse={:.1}%",
            self.total_edits,
            self.partial_reparses,
            self.full_reparses,
            self.reuse_fraction_tokens() * 100.0,
            self.reuse_fraction_nodes() * 100.0,
        )
    }
}
/// Tracks a history of edits for undo/redo.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct EditHistory {
    history: Vec<SourceEdit>,
    undo_stack: Vec<SourceEdit>,
    max_history: usize,
}
impl EditHistory {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(max_history: usize) -> Self {
        Self {
            history: Vec::new(),
            undo_stack: Vec::new(),
            max_history,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn push(&mut self, edit: SourceEdit) {
        if self.history.len() >= self.max_history {
            self.history.remove(0);
        }
        self.history.push(edit);
        self.undo_stack.clear();
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn undo(&mut self) -> Option<SourceEdit> {
        let edit = self.history.pop()?;
        self.undo_stack.push(edit.clone());
        Some(edit)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn redo(&mut self) -> Option<SourceEdit> {
        let edit = self.undo_stack.pop()?;
        self.history.push(edit.clone());
        Some(edit)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn history_len(&self) -> usize {
        self.history.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }
}
/// Incremental lexer: re-lexes only the invalidated region.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct IncrementalLexerExt {
    source: String,
    validity: TokenValidity,
    version: u64,
}
impl IncrementalLexerExt {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            validity: TokenValidity::new(),
            version: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn apply_edit(&mut self, edit: SourceEdit) {
        let inv = compute_invalidated_range(&edit, 64);
        self.validity.invalidate(&inv);
        self.source = apply_edits(&self.source, &[edit]);
        self.version += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn source(&self) -> &str {
        &self.source
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn version(&self) -> u64 {
        self.version
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn valid_token_count(&self) -> usize {
        self.validity.valid_count()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn needs_relex(&self, pos: usize) -> bool {
        !self.validity.is_valid_at(pos)
    }
}
/// A concurrency-safe version counter for incremental state.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct AtomicVersion {
    inner: std::sync::atomic::AtomicU64,
}
impl AtomicVersion {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            inner: std::sync::atomic::AtomicU64::new(0),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn increment(&self) -> u64 {
        self.inner.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn load(&self) -> u64 {
        self.inner.load(std::sync::atomic::Ordering::SeqCst)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn reset(&self) {
        self.inner.store(0, std::sync::atomic::Ordering::SeqCst);
    }
}
/// The kind of a syntax node.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum SyntaxKind {
    Root,
    Def,
    Theorem,
    Axiom,
    Ident,
    Literal,
    Token(String),
    Error,
}
/// The result of an incremental parse attempt.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug)]
pub struct IncrementalParseResult {
    pub success: bool,
    pub reused_nodes: usize,
    pub rebuilt_nodes: usize,
    pub parse_time_us: u64,
    #[allow(missing_docs)]
    pub errors: Vec<String>,
}
impl IncrementalParseResult {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(success: bool, reused: usize, rebuilt: usize, time_us: u64) -> Self {
        Self {
            success,
            reused_nodes: reused,
            rebuilt_nodes: rebuilt,
            parse_time_us: time_us,
            errors: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_error(&mut self, e: impl Into<String>) {
        self.errors.push(e.into());
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn reuse_ratio(&self) -> f64 {
        let total = self.reused_nodes + self.rebuilt_nodes;
        if total == 0 {
            0.0
        } else {
            self.reused_nodes as f64 / total as f64
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone)]
pub struct IncrParseEntry {
    pub region_hash: u64,
    pub result_repr: String,
    pub version: u64,
}
