//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::collections::HashMap;
use std::ops::Range;

/// A red node: a green node viewed at a specific byte offset.
#[allow(missing_docs)]
pub struct RedNode<'a> {
    pub green: &'a GreenNode,
    pub offset: usize,
}
impl<'a> RedNode<'a> {
    #[allow(missing_docs)]
    pub fn new(green: &'a GreenNode, offset: usize) -> Self {
        Self { green, offset }
    }
    #[allow(missing_docs)]
    pub fn range(&self) -> Range<usize> {
        self.offset..self.offset + self.green.width
    }
    #[allow(missing_docs)]
    pub fn children(&self) -> Vec<RedNode<'_>> {
        let mut pos = self.offset;
        self.green
            .children
            .iter()
            .map(|child| {
                let node = RedNode::new(child, pos);
                pos += child.width;
                node
            })
            .collect()
    }
    #[allow(missing_docs)]
    pub fn kind(&self) -> &SyntaxKind {
        &self.green.kind
    }
}
/// A green node: an immutable, position-independent syntax tree node.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct GreenNode {
    pub kind: SyntaxKind,
    pub width: usize,
    pub children: Vec<GreenNode>,
    pub text: Option<String>,
}
impl GreenNode {
    #[allow(missing_docs)]
    pub fn leaf(kind: SyntaxKind, text: impl Into<String>) -> Self {
        let text = text.into();
        let width = text.len();
        GreenNode {
            kind,
            width,
            children: Vec::new(),
            text: Some(text),
        }
    }
    #[allow(missing_docs)]
    pub fn interior(kind: SyntaxKind, children: Vec<GreenNode>) -> Self {
        let width = children.iter().map(|c| c.width).sum();
        GreenNode {
            kind,
            width,
            children,
            text: None,
        }
    }
    #[allow(missing_docs)]
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
    #[allow(missing_docs)]
    pub fn to_text(&self) -> String {
        if let Some(t) = &self.text {
            return t.clone();
        }
        self.children.iter().map(|c| c.to_text()).collect()
    }
}
/// A simple incremental lexer that caches line-level token fingerprints.
#[allow(missing_docs)]
pub struct IncrementalLexer {
    line_fingerprints: Vec<Option<TokenFingerprint>>,
    line_tokens: Vec<Vec<String>>,
}
impl IncrementalLexer {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            line_fingerprints: Vec::new(),
            line_tokens: Vec::new(),
        }
    }
    #[allow(missing_docs)]
    pub fn lex(&mut self, source: &str, dirty_lines: &[usize]) -> Vec<String> {
        let lines: Vec<&str> = source.lines().collect();
        self.line_fingerprints.resize(lines.len(), None);
        self.line_tokens.resize(lines.len(), Vec::new());
        for (i, line) in lines.iter().enumerate() {
            let fp = TokenFingerprint::compute(&[line]);
            if dirty_lines.contains(&i) || self.line_fingerprints[i].as_ref() != Some(&fp) {
                let tokens = self.tokenize_line(line);
                self.line_fingerprints[i] = Some(fp);
                self.line_tokens[i] = tokens;
            }
        }
        self.line_tokens.iter().flatten().cloned().collect()
    }
    fn tokenize_line(&self, line: &str) -> Vec<String> {
        line.split_whitespace().map(String::from).collect()
    }
    #[allow(missing_docs)]
    pub fn invalidate_lines(&mut self, range: Range<usize>) {
        for i in range {
            if i < self.line_fingerprints.len() {
                self.line_fingerprints[i] = None;
            }
        }
    }
    #[allow(missing_docs)]
    pub fn reset(&mut self) {
        self.line_fingerprints.clear();
        self.line_tokens.clear();
    }
}
/// A cache mapping source ranges to AST node IDs for incremental updates.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NodeRangeCache {
    entries: std::collections::BTreeMap<(usize, usize), u32>,
}
impl NodeRangeCache {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            entries: std::collections::BTreeMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, start: usize, end: usize, node_id: u32) {
        self.entries.insert((start, end), node_id);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&self, start: usize, end: usize) -> Option<u32> {
        self.entries.get(&(start, end)).copied()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn invalidate_range(&mut self, inv: &InvalidatedRange) {
        let to_remove: Vec<_> = self
            .entries
            .keys()
            .filter(|(s, e)| *s < inv.end && *e > inv.start)
            .copied()
            .collect();
        for k in to_remove {
            self.entries.remove(&k);
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReparsePriority {
    Low,
    Normal,
    High,
    Urgent,
}
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeKind2 {
    Paren,
    Bracket,
    Brace,
    Do,
    Where,
    Let,
}
/// A "fiber" representing a partial parse continuation.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ParseFiber {
    pub id: u64,
    pub start_offset: usize,
    pub depth: usize,
    pub state_repr: String,
}
impl ParseFiber {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(id: u64, start: usize, depth: usize, state: impl Into<String>) -> Self {
        Self {
            id,
            start_offset: start,
            depth,
            state_repr: state.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_at_root(&self) -> bool {
        self.depth == 0
    }
}
/// An undo/redo stack for source text.
#[allow(missing_docs)]
pub struct UndoRedoStack {
    undo_stack: Vec<String>,
    redo_stack: Vec<String>,
    current: String,
}
impl UndoRedoStack {
    #[allow(missing_docs)]
    pub fn new(initial: impl Into<String>) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current: initial.into(),
        }
    }
    #[allow(missing_docs)]
    pub fn push(&mut self, new_source: impl Into<String>) {
        let new_source = new_source.into();
        self.undo_stack
            .push(std::mem::replace(&mut self.current, new_source));
        self.redo_stack.clear();
    }
    #[allow(missing_docs)]
    pub fn apply(&mut self, change: &TextChange) {
        let new_source = change.apply(&self.current);
        self.push(new_source);
    }
    #[allow(missing_docs)]
    pub fn undo(&mut self) -> Option<&str> {
        if let Some(prev) = self.undo_stack.pop() {
            let old_current = std::mem::replace(&mut self.current, prev);
            self.redo_stack.push(old_current);
            Some(&self.current)
        } else {
            None
        }
    }
    #[allow(missing_docs)]
    pub fn redo(&mut self) -> Option<&str> {
        if let Some(next) = self.redo_stack.pop() {
            let old_current = std::mem::replace(&mut self.current, next);
            self.undo_stack.push(old_current);
            Some(&self.current)
        } else {
            None
        }
    }
    #[allow(missing_docs)]
    pub fn current(&self) -> &str {
        &self.current
    }
    #[allow(missing_docs)]
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }
    #[allow(missing_docs)]
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
    #[allow(missing_docs)]
    pub fn undo_depth(&self) -> usize {
        self.undo_stack.len()
    }
    #[allow(missing_docs)]
    pub fn redo_depth(&self) -> usize {
        self.redo_stack.len()
    }
}
/// A version counter for incremental parsing state.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ParseVersion {
    version: u64,
    last_full_parse: u64,
}
impl ParseVersion {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            version: 0,
            last_full_parse: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn increment(&mut self) -> u64 {
        self.version += 1;
        self.version
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn mark_full_parse(&mut self) {
        self.last_full_parse = self.version;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn current(&self) -> u64 {
        self.version
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn edits_since_full_parse(&self) -> u64 {
        self.version - self.last_full_parse
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn needs_full_reparse(&self, threshold: u64) -> bool {
        self.edits_since_full_parse() >= threshold
    }
}
/// A text change applied to the source
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct TextChange {
    pub range: Range<usize>,
    pub new_text: String,
}
impl TextChange {
    #[allow(missing_docs)]
    pub fn new(start: usize, end: usize, new_text: impl Into<String>) -> Self {
        TextChange {
            range: start..end,
            new_text: new_text.into(),
        }
    }
    #[allow(missing_docs)]
    pub fn insertion(at: usize, text: impl Into<String>) -> Self {
        TextChange {
            range: at..at,
            new_text: text.into(),
        }
    }
    #[allow(missing_docs)]
    pub fn deletion(start: usize, end: usize) -> Self {
        TextChange {
            range: start..end,
            new_text: String::new(),
        }
    }
    #[allow(missing_docs)]
    pub fn replacement(start: usize, end: usize, text: impl Into<String>) -> Self {
        TextChange {
            range: start..end,
            new_text: text.into(),
        }
    }
    #[allow(missing_docs)]
    pub fn apply(&self, source: &str) -> String {
        let start = self.range.start.min(source.len());
        let end = self.range.end.min(source.len());
        let mut result = String::with_capacity(source.len() + self.new_text.len());
        result.push_str(&source[..start]);
        result.push_str(&self.new_text);
        result.push_str(&source[end..]);
        result
    }
    #[allow(missing_docs)]
    pub fn delta(&self) -> i64 {
        (self.new_text.len() as i64) - ((self.range.end - self.range.start) as i64)
    }
    #[allow(missing_docs)]
    pub fn is_insertion(&self) -> bool {
        self.range.start == self.range.end && !self.new_text.is_empty()
    }
    #[allow(missing_docs)]
    pub fn is_deletion(&self) -> bool {
        self.new_text.is_empty() && self.range.start < self.range.end
    }
    #[allow(missing_docs)]
    pub fn is_replacement(&self) -> bool {
        !self.new_text.is_empty() && self.range.start < self.range.end
    }
}
/// A rope-like structure for efficient incremental text editing.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SimpleRope {
    pub(crate) chunks: Vec<String>,
    pub(crate) len: usize,
}
impl SimpleRope {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(text: impl Into<String>) -> Self {
        let s = text.into();
        let len = s.len();
        Self {
            chunks: vec![s],
            len,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn as_string(&self) -> String {
        self.chunks.concat()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.len
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, pos: usize, text: &str) {
        let full = self.as_string();
        let pos = pos.min(full.len());
        let new_text = format!("{}{}{}", &full[..pos], text, &full[pos..]);
        self.len = new_text.len();
        self.chunks = vec![new_text];
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn delete(&mut self, start: usize, end: usize) {
        let full = self.as_string();
        let start = start.min(full.len());
        let end = end.min(full.len());
        let new_text = format!("{}{}", &full[..start], &full[end..]);
        self.len = new_text.len();
        self.chunks = vec![new_text];
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }
}
/// Tracks valid token ranges for incremental re-lexing.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TokenValidity {
    valid_ranges: Vec<(usize, usize)>,
}
impl TokenValidity {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            valid_ranges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn mark_valid(&mut self, start: usize, end: usize) {
        self.valid_ranges.push((start, end));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn invalidate(&mut self, range: &InvalidatedRange) {
        self.valid_ranges
            .retain(|(s, e)| *e <= range.start || *s >= range.end);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_valid_at(&self, pos: usize) -> bool {
        self.valid_ranges.iter().any(|(s, e)| pos >= *s && pos < *e)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn valid_count(&self) -> usize {
        self.valid_ranges.len()
    }
}
/// A priority queue for reparse requests.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ReparseQueue {
    requests: Vec<ReparseRequest>,
}
impl ReparseQueue {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn push(&mut self, req: ReparseRequest) {
        self.requests.push(req);
        self.requests.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn pop(&mut self) -> Option<ReparseRequest> {
        if self.requests.is_empty() {
            None
        } else {
            Some(self.requests.remove(0))
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.requests.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.requests.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_urgent(&self) -> bool {
        self.requests
            .iter()
            .any(|r| r.priority == ReparsePriority::Urgent)
    }
}
/// A rolling checksum for incremental validation.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct IncrementalChecksum {
    partial_sums: Vec<u64>,
}
impl IncrementalChecksum {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn build(source: &str) -> Self {
        let mut sums = vec![0u64; source.len() + 1];
        for (i, b) in source.bytes().enumerate() {
            sums[i + 1] = sums[i].wrapping_add(b as u64);
        }
        Self { partial_sums: sums }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn range_sum(&self, start: usize, end: usize) -> u64 {
        let end = end.min(self.partial_sums.len().saturating_sub(1));
        let start = start.min(end);
        self.partial_sums[end].wrapping_sub(self.partial_sums[start])
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total(&self) -> u64 {
        *self.partial_sums.last().unwrap_or(&0)
    }
}
/// Represents a bracket/indentation scope for incremental scope tracking.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct IncrScopeEntry {
    pub start: usize,
    pub kind: ScopeKind2,
    pub depth: usize,
}
impl IncrScopeEntry {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(start: usize, kind: ScopeKind2, depth: usize) -> Self {
        Self { start, kind, depth }
    }
}
/// A dependency graph for declarations.
#[allow(missing_docs)]
pub struct DependencyGraph {
    edges: HashMap<String, Vec<String>>,
    reverse: HashMap<String, Vec<String>>,
}
impl DependencyGraph {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            reverse: HashMap::new(),
        }
    }
    #[allow(missing_docs)]
    pub fn add_edge(&mut self, dependent: &str, dependency: &str) {
        self.edges
            .entry(dependent.to_string())
            .or_default()
            .push(dependency.to_string());
        self.reverse
            .entry(dependency.to_string())
            .or_default()
            .push(dependent.to_string());
    }
    #[allow(missing_docs)]
    pub fn dependents_of(&self, name: &str) -> Vec<String> {
        let mut visited = std::collections::HashSet::new();
        let mut queue = vec![name.to_string()];
        let mut result = Vec::new();
        while let Some(current) = queue.pop() {
            if let Some(deps) = self.reverse.get(&current) {
                for dep in deps {
                    if visited.insert(dep.clone()) {
                        result.push(dep.clone());
                        queue.push(dep.clone());
                    }
                }
            }
        }
        result
    }
    #[allow(missing_docs)]
    pub fn direct_dependencies(&self, name: &str) -> &[String] {
        self.edges.get(name).map(Vec::as_slice).unwrap_or(&[])
    }
    #[allow(missing_docs)]
    pub fn remove_node(&mut self, name: &str) {
        if let Some(deps) = self.edges.remove(name) {
            for dep in deps {
                if let Some(rev) = self.reverse.get_mut(&dep) {
                    rev.retain(|n| n != name);
                }
            }
        }
        if let Some(rev_deps) = self.reverse.remove(name) {
            for rev_dep in rev_deps {
                if let Some(fwd) = self.edges.get_mut(&rev_dep) {
                    fwd.retain(|n| n != name);
                }
            }
        }
    }
    #[allow(missing_docs)]
    pub fn node_count(&self) -> usize {
        let mut all: std::collections::HashSet<&str> = std::collections::HashSet::new();
        for k in self.edges.keys() {
            all.insert(k.as_str());
        }
        for k in self.reverse.keys() {
            all.insert(k.as_str());
        }
        all.len()
    }
}
/// Source version tracking for LSP
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct VersionedSource {
    pub uri: String,
    pub version: i32,
    pub content: String,
}
impl VersionedSource {
    #[allow(missing_docs)]
    pub fn new(uri: impl Into<String>, content: impl Into<String>) -> Self {
        VersionedSource {
            uri: uri.into(),
            version: 0,
            content: content.into(),
        }
    }
    #[allow(missing_docs)]
    pub fn apply_change(&mut self, change: TextChange) -> &mut Self {
        self.content = change.apply(&self.content);
        self
    }
    #[allow(missing_docs)]
    pub fn update(&mut self, new_content: impl Into<String>, new_version: i32) -> &mut Self {
        self.content = new_content.into();
        self.version = new_version;
        self
    }
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.content.len()
    }
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
    #[allow(missing_docs)]
    pub fn offset_to_position(&self, offset: usize) -> (usize, usize) {
        let offset = offset.min(self.content.len());
        let before = &self.content[..offset];
        let line = before.chars().filter(|&c| c == '\n').count();
        let col = before
            .rfind('\n')
            .map(|nl| offset - nl - 1)
            .unwrap_or(offset);
        (line, col)
    }
    #[allow(missing_docs)]
    pub fn position_to_offset(&self, line: usize, col: usize) -> Option<usize> {
        let mut current_line = 0usize;
        let mut line_start = 0usize;
        for (i, ch) in self.content.char_indices() {
            if current_line == line {
                let mut col_offset = 0usize;
                let mut offset = line_start;
                for c in self.content[line_start..].chars() {
                    if col_offset == col {
                        return Some(offset);
                    }
                    offset += c.len_utf8();
                    col_offset += 1;
                    if c == '\n' {
                        break;
                    }
                }
                if col_offset == col {
                    return Some(offset);
                }
                return None;
            }
            if ch == '\n' {
                current_line += 1;
                line_start = i + 1;
            }
        }
        if current_line == line {
            let line_len = self.content[line_start..].len();
            if col <= line_len {
                return Some(line_start + col);
            }
        }
        None
    }
}
/// A token range that becomes invalid after an edit.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidatedRange {
    pub start: usize,
    pub end: usize,
}
impl InvalidatedRange {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.end - self.start
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn contains(&self, pos: usize) -> bool {
        pos >= self.start && pos < self.end
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn overlaps(&self, other: &Self) -> bool {
        self.start < other.end && self.end > other.start
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}
/// Represents a "dirty" region in the source that needs re-parsing.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct DirtyRegion {
    pub start_line: usize,
    pub end_line: usize,
    pub start_byte: usize,
    pub end_byte: usize,
}
impl DirtyRegion {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(start_line: usize, end_line: usize, start_byte: usize, end_byte: usize) -> Self {
        Self {
            start_line,
            end_line,
            start_byte,
            end_byte,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn line_count(&self) -> usize {
        self.end_line.saturating_sub(self.start_line) + 1
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn byte_count(&self) -> usize {
        self.end_byte.saturating_sub(self.start_byte)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_single_line(&self) -> bool {
        self.start_line == self.end_line
    }
}
/// A simple persistent (copy-on-write) vector.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct PersistentVec<T: Clone> {
    data: std::rc::Rc<Vec<T>>,
}
impl<T: Clone> PersistentVec<T> {
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            data: std::rc::Rc::new(Vec::new()),
        }
    }
    #[allow(missing_docs)]
    pub fn push(&self, value: T) -> Self {
        let mut new_data = (*self.data).clone();
        new_data.push(value);
        Self {
            data: std::rc::Rc::new(new_data),
        }
    }
    #[allow(missing_docs)]
    pub fn set(&self, idx: usize, value: T) -> Option<Self> {
        if idx >= self.data.len() {
            return None;
        }
        let mut new_data = (*self.data).clone();
        new_data[idx] = value;
        Some(Self {
            data: std::rc::Rc::new(new_data),
        })
    }
    #[allow(missing_docs)]
    pub fn get(&self, idx: usize) -> Option<&T> {
        self.data.get(idx)
    }
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    #[allow(missing_docs)]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
}
/// Represents a single source edit (insert or delete).
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct SourceEdit {
    pub start: usize,
    pub end: usize,
    pub new_text: String,
}
impl SourceEdit {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(pos: usize, text: impl Into<String>) -> Self {
        Self {
            start: pos,
            end: pos,
            new_text: text.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn delete(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
            new_text: String::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn replace(start: usize, end: usize, text: impl Into<String>) -> Self {
        Self {
            start,
            end,
            new_text: text.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_insert(&self) -> bool {
        self.start == self.end && !self.new_text.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_delete(&self) -> bool {
        self.start < self.end && self.new_text.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_replace(&self) -> bool {
        self.start < self.end && !self.new_text.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn delta(&self) -> i64 {
        self.new_text.len() as i64 - (self.end - self.start) as i64
    }
}
/// A map from byte offset to token ID for incremental relexing.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct OffsetToTokenMap {
    map: std::collections::BTreeMap<usize, u32>,
}
impl OffsetToTokenMap {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            map: std::collections::BTreeMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn insert(&mut self, offset: usize, token_id: u32) {
        self.map.insert(offset, token_id);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn token_at(&self, offset: usize) -> Option<u32> {
        self.map.range(..=offset).next_back().map(|(_, &id)| id)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn invalidate_from(&mut self, offset: usize) {
        let to_remove: Vec<_> = self.map.range(offset..).map(|(&k, _)| k).collect();
        for k in to_remove {
            self.map.remove(&k);
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn shift(&mut self, from: usize, delta: i64) {
        let to_shift: Vec<_> = self.map.range(from..).map(|(&k, &v)| (k, v)).collect();
        for (k, _) in &to_shift {
            self.map.remove(k);
        }
        for (k, v) in to_shift {
            let new_k = (k as i64 + delta).max(0) as usize;
            self.map.insert(new_k, v);
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.map.len()
    }
}
/// A stack of scopes for incremental parsing.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct IncrScopeStack {
    stack: Vec<IncrScopeEntry>,
}
impl IncrScopeStack {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn push(&mut self, entry: IncrScopeEntry) {
        self.stack.push(entry);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn pop(&mut self) -> Option<IncrScopeEntry> {
        self.stack.pop()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn peek(&self) -> Option<&IncrScopeEntry> {
        self.stack.last()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn current_scope(&self) -> Option<ScopeKind2> {
        self.stack.last().map(|e| e.kind)
    }
}
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
        sorted.sort_by(|a, b| b.range.start.cmp(&a.range.start));
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
        changes.sort_by(|a, b| b.range.start.cmp(&a.range.start));
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
