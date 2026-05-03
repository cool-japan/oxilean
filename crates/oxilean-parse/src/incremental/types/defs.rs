//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::functions::*;
#[allow(unused_imports)]
use super::impls::*;
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
        self.requests.sort_by_key(|b| std::cmp::Reverse(b.priority));
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
