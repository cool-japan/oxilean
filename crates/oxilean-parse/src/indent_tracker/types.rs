//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

#[derive(Debug, Clone, Default)]
pub struct IndentValidator {
    pub expect_spaces: bool,
    pub violations: Vec<(usize, char)>,
}
impl IndentValidator {
    pub fn expect_spaces() -> Self {
        Self {
            expect_spaces: true,
            violations: Vec::new(),
        }
    }
    pub fn expect_tabs() -> Self {
        Self {
            expect_spaces: false,
            violations: Vec::new(),
        }
    }
    pub fn validate(&mut self, src: &str) {
        for (i, line) in src.lines().enumerate() {
            for ch in line.chars().take_while(|c| c.is_whitespace()) {
                let violation = if self.expect_spaces {
                    ch == '\t'
                } else {
                    ch == ' '
                };
                if violation {
                    self.violations.push((i + 1, ch));
                    break;
                }
            }
        }
    }
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }
}
/// A contiguous region of source lines that share the same base indentation.
#[derive(Debug, Clone)]
pub struct IndentRegion {
    /// 1-based start line.
    pub start_line: usize,
    /// 1-based end line (inclusive).
    pub end_line: usize,
    /// Base indentation width for this region.
    pub indent_width: usize,
}
impl IndentRegion {
    /// Create a new region.
    pub fn new(start_line: usize, end_line: usize, indent_width: usize) -> Self {
        Self {
            start_line,
            end_line,
            indent_width,
        }
    }
    /// Number of lines in this region.
    pub fn line_count(&self) -> usize {
        self.end_line.saturating_sub(self.start_line) + 1
    }
    /// True if this region contains `line`.
    pub fn contains_line(&self, line: usize) -> bool {
        line >= self.start_line && line <= self.end_line
    }
}
#[derive(Debug, Default)]
pub struct SourceSplitter {
    pub tab_width: usize,
}
impl SourceSplitter {
    pub fn new(tab_width: usize) -> Self {
        Self { tab_width }
    }
    pub fn split<'a>(&self, src: &'a str) -> Vec<(usize, &'a str)> {
        let lines: Vec<&str> = src.lines().collect();
        let mut result = Vec::new();
        let mut block_start: Option<usize> = None;
        let mut block_start_byte: usize = 0;
        let mut byte_offset = 0usize;
        for (i, &line) in lines.iter().enumerate() {
            let class = LineClass::classify(line);
            if class.starts_decl() {
                if let Some(start_line) = block_start {
                    result.push((start_line, &src[block_start_byte..byte_offset]));
                }
                block_start = Some(i);
                block_start_byte = byte_offset;
            }
            byte_offset += line.len();
            if i + 1 < lines.len() {
                byte_offset += 1;
            }
        }
        if let Some(start_line) = block_start {
            result.push((start_line, &src[block_start_byte..]));
        }
        result
    }
}
#[derive(Debug, Clone)]
pub struct DoBlockTracker {
    block_columns: Vec<usize>,
    pub tab_width: usize,
}
impl DoBlockTracker {
    pub fn new(tab_width: usize) -> Self {
        Self {
            block_columns: Vec::new(),
            tab_width,
        }
    }
    pub fn enter(&mut self, col: usize) {
        self.block_columns.push(col);
    }
    pub fn exit(&mut self) -> Option<usize> {
        self.block_columns.pop()
    }
    pub fn current_col(&self) -> Option<usize> {
        self.block_columns.last().copied()
    }
    pub fn is_statement(&self, line: &str) -> bool {
        let col = LayoutContext::indent_col(line, self.tab_width);
        self.current_col() == Some(col)
    }
    pub fn is_deeper(&self, line: &str) -> bool {
        let col = LayoutContext::indent_col(line, self.tab_width);
        self.current_col().is_some_and(|e| col > e)
    }
    pub fn is_closer(&self, line: &str) -> bool {
        let col = LayoutContext::indent_col(line, self.tab_width);
        self.current_col().is_some_and(|e| col < e)
    }
    pub fn depth(&self) -> usize {
        self.block_columns.len()
    }
}
/// A layout rule describes how indentation affects block structure in OxiLean.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutRule {
    /// Lines at exactly this column continue the same block.
    SameBlock(usize),
    /// Lines strictly to the right of this column are continuations.
    Continuation(usize),
    /// Lines strictly to the left of this column close the block.
    CloseBlock(usize),
}
impl LayoutRule {
    /// Test whether a given column satisfies this layout rule.
    pub fn matches(&self, col: usize) -> bool {
        match self {
            LayoutRule::SameBlock(c) => col == *c,
            LayoutRule::Continuation(c) => col > *c,
            LayoutRule::CloseBlock(c) => col < *c,
        }
    }
    /// The pivot column for this rule.
    pub fn pivot(&self) -> usize {
        match self {
            LayoutRule::SameBlock(c) | LayoutRule::Continuation(c) | LayoutRule::CloseBlock(c) => {
                *c
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentMismatchError {
    pub line_number: usize,
    pub expected: usize,
    pub actual: usize,
    pub context: String,
}
impl IndentMismatchError {
    pub fn new(
        line_number: usize,
        expected: usize,
        actual: usize,
        context: impl Into<String>,
    ) -> Self {
        Self {
            line_number,
            expected,
            actual,
            context: context.into(),
        }
    }
    pub fn message(&self) -> String {
        format!(
            "indentation error at line {}: expected {} spaces, found {} ({})",
            self.line_number, self.expected, self.actual, self.context
        )
    }
}
/// Records a history of indent levels for undo/redo-style editing support.
#[derive(Debug, Clone, Default)]
pub struct IndentLevelHistory {
    past: Vec<Vec<IndentLevel>>,
    present: Vec<IndentLevel>,
    future: Vec<Vec<IndentLevel>>,
}
impl IndentLevelHistory {
    /// Create an empty history.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record the current state before making a change.
    pub fn snapshot(&mut self) {
        self.past.push(self.present.clone());
        self.future.clear();
    }
    /// Set the current indent levels.
    pub fn set_levels(&mut self, levels: Vec<IndentLevel>) {
        self.present = levels;
    }
    /// Current indent levels.
    pub fn current(&self) -> &[IndentLevel] {
        &self.present
    }
    /// Undo: restore the previous state.
    pub fn undo(&mut self) -> bool {
        if let Some(prev) = self.past.pop() {
            self.future.push(self.present.clone());
            self.present = prev;
            true
        } else {
            false
        }
    }
    /// Redo: reapply the previously undone change.
    pub fn redo(&mut self) -> bool {
        if let Some(next) = self.future.pop() {
            self.past.push(self.present.clone());
            self.present = next;
            true
        } else {
            false
        }
    }
    /// Number of undoable steps.
    pub fn undo_depth(&self) -> usize {
        self.past.len()
    }
    /// Number of redoable steps.
    pub fn redo_depth(&self) -> usize {
        self.future.len()
    }
}
#[derive(Debug, Clone)]
pub struct Scope {
    pub kind: String,
    pub base_indent: IndentLevel,
    pub bindings: Vec<String>,
}
impl Scope {
    pub fn new(kind: impl Into<String>, base_indent: IndentLevel) -> Self {
        Self {
            kind: kind.into(),
            base_indent,
            bindings: Vec::new(),
        }
    }
    pub fn add_binding(&mut self, name: impl Into<String>) {
        self.bindings.push(name.into());
    }
    pub fn is_bound(&self, name: &str) -> bool {
        self.bindings.iter().any(|b| b == name)
    }
}
/// Context passed around during layout-sensitive parsing.
#[derive(Debug, Clone)]
pub struct LayoutContext {
    rules: Vec<LayoutRule>,
    pub tab_width: usize,
}
impl LayoutContext {
    pub fn new(tab_width: usize) -> Self {
        Self {
            rules: Vec::new(),
            tab_width,
        }
    }
    pub fn push_rule(&mut self, rule: LayoutRule) {
        self.rules.push(rule);
    }
    pub fn pop_rule(&mut self) -> Option<LayoutRule> {
        self.rules.pop()
    }
    pub fn current_rule(&self) -> Option<&LayoutRule> {
        self.rules.last()
    }
    pub fn indent_col(line: &str, tab_width: usize) -> usize {
        let mut col = 0usize;
        for ch in line.chars() {
            match ch {
                ' ' => col += 1,
                '\t' => {
                    col = (col / tab_width + 1) * tab_width;
                }
                _ => break,
            }
        }
        col
    }
    pub fn line_continues_block(&self, line: &str) -> bool {
        let col = Self::indent_col(line, self.tab_width);
        match self.current_rule() {
            Some(rule) => rule.matches(col),
            None => true,
        }
    }
    pub fn depth(&self) -> usize {
        self.rules.len()
    }
    pub fn clear(&mut self) {
        self.rules.clear();
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndentDiff {
    Indent,
    Same,
    Dedent,
}
/// A stack of indentation levels used during parsing.
pub struct IndentStack {
    pub stack: Vec<IndentLevel>,
    pub tab_width: usize,
}
impl IndentStack {
    pub fn new(tab_width: usize) -> Self {
        Self {
            stack: Vec::new(),
            tab_width,
        }
    }
    pub fn push(&mut self, level: IndentLevel) {
        self.stack.push(level);
    }
    pub fn pop(&mut self) -> Option<IndentLevel> {
        self.stack.pop()
    }
    /// Return a reference to the current (top) indentation level.
    pub fn current(&self) -> Option<&IndentLevel> {
        self.stack.last()
    }
    /// Pop levels from the stack until the top is at or below `level`.
    /// Returns the number of levels popped.
    pub fn dedent_to(&mut self, level: &IndentLevel) -> usize {
        let tw = self.tab_width;
        let mut count = 0;
        while let Some(top) = self.stack.last() {
            if top.total_width(tw) > level.total_width(tw) {
                self.stack.pop();
                count += 1;
            } else {
                break;
            }
        }
        count
    }
}
/// Represents an indentation level as a mix of spaces and tabs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentLevel {
    pub spaces: usize,
    pub tabs: usize,
}
impl IndentLevel {
    pub fn new(spaces: usize, tabs: usize) -> Self {
        Self { spaces, tabs }
    }
    /// Total width in columns, using the given tab width.
    pub fn total_width(&self, tab_width: usize) -> usize {
        self.tabs * tab_width + self.spaces
    }
    /// Returns true if this level is strictly deeper than `other`.
    pub fn is_deeper_than(&self, other: &IndentLevel, tab_width: usize) -> bool {
        self.total_width(tab_width) > other.total_width(tab_width)
    }
}
/// An indent "fence" that marks the minimum indentation for a scope.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct IndentFence {
    pub min_indent: usize,
    pub active: bool,
}
impl IndentFence {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(min_indent: usize) -> Self {
        Self {
            min_indent,
            active: true,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn allows(&self, indent: usize) -> bool {
        !self.active || indent >= self.min_indent
    }
}
#[derive(Debug, Clone)]
pub struct IndentRewriter {
    pub from_step: usize,
    pub to_step: usize,
}
impl IndentRewriter {
    pub fn new(from_step: usize, to_step: usize) -> Self {
        Self { from_step, to_step }
    }
    pub fn rewrite_line(&self, line: &str) -> String {
        let spaces = line.chars().take_while(|&c| c == ' ').count();
        if spaces == 0 || self.from_step == 0 {
            return line.to_string();
        }
        let levels = spaces / self.from_step;
        let remainder = spaces % self.from_step;
        " ".repeat(levels * self.to_step + remainder) + &line[spaces..]
    }
    pub fn rewrite(&self, src: &str) -> String {
        src.lines()
            .map(|l| self.rewrite_line(l))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
#[derive(Debug, Clone, Default)]
pub struct MultilineStringTracker {
    pub in_string: bool,
    pub open_col: usize,
    pub line_count: usize,
}
impl MultilineStringTracker {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn open(&mut self, col: usize) {
        self.in_string = true;
        self.open_col = col;
        self.line_count = 0;
    }
    pub fn close(&mut self) {
        self.in_string = false;
        self.open_col = 0;
        self.line_count = 0;
    }
    pub fn next_line(&mut self) {
        if self.in_string {
            self.line_count += 1;
        }
    }
    pub fn has_unescaped_quote(line: &str) -> bool {
        let mut escaped = false;
        for ch in line.chars() {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                return true;
            }
        }
        false
    }
}
#[derive(Debug, Clone, Default)]
pub struct IndentConsistencyReport {
    pub step: usize,
    pub uses_tabs: bool,
    pub uses_spaces: bool,
    pub is_mixed: bool,
    pub total_indented: usize,
}
impl IndentConsistencyReport {
    pub fn from_source(src: &str) -> Self {
        let stats = IndentStats::analyse(src);
        let deltas = IndentDelta::compute_all(src, 4);
        let steps: Vec<usize> = deltas
            .iter()
            .filter(|d| d.is_increase())
            .map(|d| d.after - d.before)
            .collect();
        let step = steps.iter().copied().fold(0usize, gcd);
        IndentConsistencyReport {
            step,
            uses_tabs: stats.tab_lines > 0 || stats.mixed_lines > 0,
            uses_spaces: stats.space_lines > 0 || stats.mixed_lines > 0,
            is_mixed: stats.mixed_lines > 0,
            total_indented: stats.code_lines(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhitespaceKind {
    None,
    Space,
    MultiSpace(usize),
    Newline,
    MultiNewline(usize),
    Mixed,
}
impl WhitespaceKind {
    pub fn classify(src: &str, start: usize, end: usize) -> WhitespaceKind {
        let slice = &src[start..end];
        if slice.is_empty() {
            return WhitespaceKind::None;
        }
        let newline_count = slice.chars().filter(|&c| c == '\n').count();
        let space_count = slice.chars().filter(|&c| c == ' ').count();
        if newline_count == 0 {
            if space_count == 1 {
                WhitespaceKind::Space
            } else {
                WhitespaceKind::MultiSpace(space_count)
            }
        } else if space_count == 0 {
            if newline_count == 1 {
                WhitespaceKind::Newline
            } else {
                WhitespaceKind::MultiNewline(newline_count)
            }
        } else {
            WhitespaceKind::Mixed
        }
    }
    pub fn contains_newline(self) -> bool {
        matches!(
            self,
            WhitespaceKind::Newline | WhitespaceKind::MultiNewline(_) | WhitespaceKind::Mixed
        )
    }
}
#[derive(Debug)]
pub struct BlockParser {
    pub tab_width: usize,
    block_stack: Vec<usize>,
}
impl BlockParser {
    pub fn new(tab_width: usize) -> Self {
        Self {
            tab_width,
            block_stack: Vec::new(),
        }
    }
    pub fn parse_blocks<'a>(&mut self, lines: &[&'a str]) -> Vec<Vec<&'a str>> {
        let mut result: Vec<Vec<&'a str>> = Vec::new();
        let mut current: Vec<&'a str> = Vec::new();
        for &line in lines {
            if line.trim().is_empty() {
                current.push(line);
                continue;
            }
            let col = LayoutContext::indent_col(line, self.tab_width);
            match self.block_stack.last().copied() {
                None => {
                    self.block_stack.push(col);
                    current.push(line);
                }
                Some(base) => {
                    if col >= base {
                        current.push(line);
                    } else {
                        if !current.is_empty() {
                            result.push(current.clone());
                            current.clear();
                        }
                        self.block_stack.pop();
                        self.block_stack.push(col);
                        current.push(line);
                    }
                }
            }
        }
        if !current.is_empty() {
            result.push(current);
        }
        result
    }
    pub fn reset(&mut self) {
        self.block_stack.clear();
    }
    pub fn depth(&self) -> usize {
        self.block_stack.len()
    }
}
#[derive(Debug, Clone, Default)]
pub struct LetBindingTracker {
    bindings: Vec<LetBinding>,
}
impl LetBindingTracker {
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }
    pub fn push(&mut self, binding: LetBinding) {
        self.bindings.push(binding);
    }
    pub fn pop(&mut self) -> Option<LetBinding> {
        self.bindings.pop()
    }
    pub fn resolve(&self, name: &str) -> Option<&LetBinding> {
        self.bindings.iter().rev().find(|b| b.name == name)
    }
    pub fn all(&self) -> &[LetBinding] {
        &self.bindings
    }
    pub fn clear(&mut self) {
        self.bindings.clear();
    }
    pub fn len(&self) -> usize {
        self.bindings.len()
    }
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
}
#[derive(Debug, Clone, Default)]
pub struct OpenBraceTracker {
    stack: Vec<(char, usize, usize)>,
}
impl OpenBraceTracker {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, ch: char, line: usize, col: usize) {
        self.stack.push((ch, line, col));
    }
    pub fn pop(
        &mut self,
        ch: char,
        line: usize,
        col: usize,
    ) -> Result<(), (char, char, usize, usize)> {
        let expected = Self::matching_open(ch);
        match self.stack.last() {
            Some(&(top, _, _)) if top == expected => {
                self.stack.pop();
                Ok(())
            }
            Some(&(top, open_line, open_col)) => Err((top, ch, open_line, open_col)),
            None => Err(('?', ch, line, col)),
        }
    }
    pub fn matching_open(close: char) -> char {
        match close {
            ')' => '(',
            ']' => '[',
            '}' => '{',
            _ => '\0',
        }
    }
    pub fn is_balanced(&self) -> bool {
        self.stack.is_empty()
    }
    pub fn unmatched(&self) -> &[(char, usize, usize)] {
        &self.stack
    }
    pub fn clear(&mut self) {
        self.stack.clear();
    }
}
/// Tracks `where` blocks and their item indentation.
pub struct WhereBlockTracker {
    pub indent_stack: IndentStack,
    pub in_where_block: bool,
    /// Each entry is (item_name, indent_width).
    pub where_items: Vec<(String, usize)>,
}
impl WhereBlockTracker {
    pub fn new() -> Self {
        Self {
            indent_stack: IndentStack::new(4),
            in_where_block: false,
            where_items: Vec::new(),
        }
    }
    /// Enter a `where` block at the given base indentation.
    pub fn enter_where(&mut self, base_indent: IndentLevel) {
        self.in_where_block = true;
        self.indent_stack.push(base_indent);
    }
    /// Exit the current `where` block.
    pub fn exit_where(&mut self) {
        self.in_where_block = false;
        self.indent_stack.pop();
        self.where_items.clear();
    }
    /// Register a named item inside the where block at the given indent.
    pub fn add_where_item(&mut self, name: &str, indent: IndentLevel) {
        let tw = self.indent_stack.tab_width;
        self.where_items
            .push((name.to_string(), indent.total_width(tw)));
    }
    /// Check whether `line` at `indent` is a valid where-block item.
    pub fn is_where_item(&self, _line: &str, indent: IndentLevel) -> bool {
        if !self.in_where_block {
            return false;
        }
        let tw = self.indent_stack.tab_width;
        match self.indent_stack.current() {
            Some(base) => indent.is_deeper_than(base, tw),
            None => false,
        }
    }
    /// Parse the leading whitespace of a line into an `IndentLevel`.
    pub fn parse_leading_whitespace(line: &str) -> IndentLevel {
        let mut spaces = 0usize;
        let mut tabs = 0usize;
        for ch in line.chars() {
            match ch {
                ' ' => spaces += 1,
                '\t' => tabs += 1,
                _ => break,
            }
        }
        IndentLevel::new(spaces, tabs)
    }
}
/// Classifies sequences of tokens based on their indentation patterns.
/// Used to detect common OxiLean syntax patterns like `where` clauses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenSequenceClass {
    /// A top-level definition with a `where` clause.
    DefWithWhere,
    /// A standalone definition without `where`.
    PlainDef,
    /// A `let` expression block.
    LetBlock,
    /// A `do`-notation block.
    DoBlock,
    /// Unknown/unclassified sequence.
    Unknown,
}
/// Indentation mode: spaces vs tabs.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndentMode {
    Spaces(usize),
    Tabs(usize),
}
impl IndentMode {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn unit_width(&self) -> usize {
        match self {
            Self::Spaces(n) | Self::Tabs(n) => *n,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn indent_str(&self, level: usize) -> String {
        match self {
            Self::Spaces(n) => " ".repeat(n * level),
            Self::Tabs(_) => "\t".repeat(level),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct IndentHistory {
    pub entries: Vec<(usize, usize)>,
    pub tab_width: usize,
}
impl IndentHistory {
    pub fn new(tab_width: usize) -> Self {
        Self {
            entries: Vec::new(),
            tab_width,
        }
    }
    pub fn record(&mut self, line_number: usize, line: &str) {
        let col = LayoutContext::indent_col(line, self.tab_width);
        self.entries.push((line_number, col));
    }
    pub fn from_source(src: &str, tab_width: usize) -> Self {
        let mut hist = Self::new(tab_width);
        for (i, line) in src.lines().enumerate() {
            if !line.trim().is_empty() {
                hist.record(i + 1, line);
            }
        }
        hist
    }
    pub fn max_indent(&self) -> usize {
        self.entries.iter().map(|&(_, w)| w).max().unwrap_or(0)
    }
    pub fn min_nonzero_indent(&self) -> Option<usize> {
        self.entries
            .iter()
            .map(|&(_, w)| w)
            .filter(|&w| w > 0)
            .min()
    }
}
/// Represents a "virtual" column position in a layout engine.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct VirtualColumn {
    pub column: usize,
    pub tab_width: usize,
}
impl VirtualColumn {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(column: usize, tab_width: usize) -> Self {
        Self { column, tab_width }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn advance_by(&self, n: usize) -> Self {
        Self {
            column: self.column + n,
            ..*self
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn advance_tab(&self) -> Self {
        let next = ((self.column / self.tab_width) + 1) * self.tab_width;
        Self {
            column: next,
            ..*self
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_aligned_to(&self, n: usize) -> bool {
        n > 0 && self.column % n == 0
    }
}
#[derive(Debug, Clone, Default)]
pub struct CommentTracker {
    pub block_depth: usize,
    pub in_line_comment: bool,
}
impl CommentTracker {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn process_pair(&mut self, ch: char, next: char) {
        if self.in_line_comment {
            return;
        }
        if self.block_depth == 0 && ch == '-' && next == '-' {
            self.in_line_comment = true;
        } else if ch == '/' && next == '-' {
            self.block_depth += 1;
        } else if self.block_depth > 0 && ch == '-' && next == '/' {
            self.block_depth -= 1;
        }
    }
    pub fn end_of_line(&mut self) {
        self.in_line_comment = false;
    }
    pub fn in_comment(&self) -> bool {
        self.in_line_comment || self.block_depth > 0
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineClass {
    Blank,
    Comment,
    DeclOpener,
    WhereKeyword,
    LetBinding,
    Continuation,
    Other,
}
impl LineClass {
    pub fn classify(line: &str) -> LineClass {
        let trimmed = line.trim_start();
        if trimmed.is_empty() {
            return LineClass::Blank;
        }
        if trimmed.starts_with("--") {
            return LineClass::Comment;
        }
        let decl_openers = [
            "def ",
            "theorem ",
            "lemma ",
            "axiom ",
            "structure ",
            "class ",
            "instance ",
            "definition ",
            "noncomputable ",
            "private ",
            "protected ",
            "abbrev ",
        ];
        for opener in &decl_openers {
            if trimmed.starts_with(opener) {
                return LineClass::DeclOpener;
            }
        }
        if let Some(after) = trimmed.strip_prefix("where") {
            if after.is_empty()
                || !after
                    .chars()
                    .next()
                    .is_some_and(|c| c.is_alphanumeric() || c == '_')
            {
                return LineClass::WhereKeyword;
            }
        }
        if trimmed.starts_with("let ") {
            return LineClass::LetBinding;
        }
        LineClass::Other
    }
    pub fn is_ignorable(&self) -> bool {
        matches!(self, LineClass::Blank | LineClass::Comment)
    }
    pub fn starts_decl(&self) -> bool {
        matches!(self, LineClass::DeclOpener)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSpan {
    pub start: usize,
    pub end: usize,
}
impl LineSpan {
    pub fn new(start: usize, end: usize) -> Self {
        assert!(start <= end);
        Self { start, end }
    }
    pub fn single(line: usize) -> Self {
        Self {
            start: line,
            end: line,
        }
    }
    pub fn len(&self) -> usize {
        self.end - self.start + 1
    }
    /// Whether this span covers no lines.
    pub fn is_empty(&self) -> bool {
        self.end < self.start
    }
    pub fn is_single_line(&self) -> bool {
        self.start == self.end
    }
    pub fn contains(&self, line: usize) -> bool {
        line >= self.start && line <= self.end
    }
    pub fn merge(self, other: LineSpan) -> LineSpan {
        LineSpan::new(self.start.min(other.start), self.end.max(other.end))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentGuide {
    pub column: usize,
    pub is_active: bool,
}
impl IndentGuide {
    pub fn new(column: usize, is_active: bool) -> Self {
        Self { column, is_active }
    }
}
#[derive(Debug, Clone)]
pub struct IndentFixer {
    pub target_step: usize,
    pub tab_width: usize,
}
impl IndentFixer {
    pub fn new(target_step: usize, tab_width: usize) -> Self {
        Self {
            target_step,
            tab_width,
        }
    }
    pub fn fix(&self, src: &str) -> String {
        let report = IndentConsistencyReport::from_source(src);
        let from_step = if report.step > 0 {
            report.step
        } else {
            self.target_step
        };
        let norm = IndentNormaliser::new(self.tab_width);
        let rw = IndentRewriter::new(from_step, self.target_step);
        rw.rewrite(&norm.normalise(src))
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HangingIndent {
    pub first_col: usize,
    pub cont_col: usize,
}
impl HangingIndent {
    pub fn new(first_col: usize, cont_col: usize) -> Self {
        Self {
            first_col,
            cont_col,
        }
    }
    pub fn is_continuation(&self, col: usize) -> bool {
        col == self.cont_col
    }
    pub fn is_first(&self, col: usize) -> bool {
        col == self.first_col
    }
    pub fn overhang(&self) -> usize {
        self.cont_col.saturating_sub(self.first_col)
    }
}
#[derive(Debug, Clone)]
pub struct IndentNormaliser {
    pub tab_width: usize,
}
impl IndentNormaliser {
    pub fn new(tab_width: usize) -> Self {
        Self { tab_width }
    }
    pub fn normalise_line(&self, line: &str) -> String {
        let mut col = 0usize;
        let mut rest_start = 0;
        for (byte_idx, ch) in line.char_indices() {
            match ch {
                ' ' => {
                    col += 1;
                    rest_start = byte_idx + 1;
                }
                '\t' => {
                    let next_stop = (col / self.tab_width + 1) * self.tab_width;
                    col = next_stop;
                    rest_start = byte_idx + 1;
                }
                _ => break,
            }
        }
        " ".repeat(col) + &line[rest_start..]
    }
    pub fn normalise(&self, src: &str) -> String {
        src.lines()
            .map(|l| self.normalise_line(l))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// A registry of construct indent rules.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ConstructRuleRegistry {
    rules: std::collections::HashMap<String, ConstructIndentRule>,
}
impl ConstructRuleRegistry {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let mut reg = Self {
            rules: std::collections::HashMap::new(),
        };
        reg.add(ConstructIndentRule::standard("def"));
        reg.add(ConstructIndentRule::standard("theorem"));
        reg.add(ConstructIndentRule::standard("match"));
        reg.add(ConstructIndentRule::standard("do"));
        reg.add(ConstructIndentRule::standard("where"));
        reg
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, rule: ConstructIndentRule) {
        self.rules.insert(rule.construct_name.clone(), rule);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&self, name: &str) -> Option<&ConstructIndentRule> {
        self.rules.get(name)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn body_indent(&self, name: &str) -> usize {
        self.rules.get(name).map_or(2, |r| r.body_indent)
    }
}
/// A "column oracle" that can predict alignment positions.
#[allow(dead_code)]
pub struct ColumnOracle {
    tab_width: usize,
    reference_columns: Vec<usize>,
}
impl ColumnOracle {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(tab_width: usize) -> Self {
        Self {
            tab_width,
            reference_columns: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_reference(&mut self, col: usize) {
        self.reference_columns.push(col);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn next_alignment(&self, current: usize) -> usize {
        self.reference_columns
            .iter()
            .copied()
            .filter(|&c| c > current)
            .min()
            .unwrap_or(((current / self.tab_width) + 1) * self.tab_width)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_at_reference(&self, col: usize) -> bool {
        self.reference_columns.contains(&col)
    }
}
#[derive(Debug, Clone)]
pub struct LetBinding {
    pub name: String,
    pub col: usize,
    pub has_type: bool,
}
impl LetBinding {
    pub fn new(name: impl Into<String>, col: usize, has_type: bool) -> Self {
        Self {
            name: name.into(),
            col,
            has_type,
        }
    }
}
#[derive(Debug, Clone)]
pub struct TabStopIterator {
    pub tab_width: usize,
    pub(super) current: usize,
}
impl TabStopIterator {
    pub fn new(tab_width: usize) -> Self {
        Self {
            tab_width,
            current: 0,
        }
    }
    pub fn from_col(tab_width: usize, start_col: usize) -> Self {
        let next_stop = if start_col % tab_width == 0 {
            start_col
        } else {
            (start_col / tab_width + 1) * tab_width
        };
        Self {
            tab_width,
            current: next_stop,
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct ColumnAligner {
    pub target_col: usize,
}
impl ColumnAligner {
    pub fn new(target_col: usize) -> Self {
        Self { target_col }
    }
    pub fn pad(&self, s: &str, current_col: usize) -> String {
        if current_col >= self.target_col {
            format!("{} ", s)
        } else {
            format!("{}{}", s, " ".repeat(self.target_col - current_col))
        }
    }
    pub fn align_all(items: &[String]) -> Vec<String> {
        let max_len = items.iter().map(|s| s.len()).max().unwrap_or(0);
        items
            .iter()
            .map(|s| format!("{}{}", s, " ".repeat(max_len - s.len())))
            .collect()
    }
}
/// An "indent block" that groups lines by their indent level.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct IndentBlock {
    pub level: usize,
    pub lines: Vec<String>,
}
impl IndentBlock {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(level: usize) -> Self {
        Self {
            level,
            lines: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_line(&mut self, line: impl Into<String>) {
        self.lines.push(line.into());
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn render(&self) -> String {
        self.lines.join("\n")
    }
}
#[derive(Debug, Clone, Default)]
pub struct AlignedPrinter {
    entries: Vec<(usize, String, String)>,
    pub separator: String,
}
impl AlignedPrinter {
    pub fn new(separator: impl Into<String>) -> Self {
        Self {
            entries: Vec::new(),
            separator: separator.into(),
        }
    }
    pub fn add(&mut self, indent: usize, lhs: impl Into<String>, rhs: impl Into<String>) {
        self.entries.push((indent, lhs.into(), rhs.into()));
    }
    pub fn format(&self) -> String {
        let max_lhs = self
            .entries
            .iter()
            .map(|(_, lhs, _)| lhs.len())
            .max()
            .unwrap_or(0);
        let mut lines = Vec::new();
        for (indent, lhs, rhs) in &self.entries {
            let padding = " ".repeat(max_lhs - lhs.len());
            let indent_str = " ".repeat(*indent);
            lines.push(format!(
                "{}{}{} {} {}",
                indent_str, lhs, padding, self.separator, rhs
            ));
        }
        lines.join("\n")
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A position tracker that records all indent-change events.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct IndentChangeLog {
    events: Vec<(usize, usize, usize)>,
}
impl IndentChangeLog {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record(&mut self, line: usize, old: usize, new: usize) {
        if old != new {
            self.events.push((line, old, new));
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.events.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn increases(&self) -> usize {
        self.events.iter().filter(|(_, o, n)| n > o).count()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn decreases(&self) -> usize {
        self.events.iter().filter(|(_, o, n)| n < o).count()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentDelta {
    pub line: usize,
    pub before: usize,
    pub after: usize,
}
impl IndentDelta {
    pub fn compute_all(src: &str, tab_width: usize) -> Vec<IndentDelta> {
        let mut deltas = Vec::new();
        let mut prev: Option<usize> = None;
        for (i, line) in src.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let col = LayoutContext::indent_col(line, tab_width);
            if let Some(p) = prev {
                if col != p {
                    deltas.push(IndentDelta {
                        line: i + 1,
                        before: p,
                        after: col,
                    });
                }
            }
            prev = Some(col);
        }
        deltas
    }
    pub fn is_increase(&self) -> bool {
        self.after > self.before
    }
    pub fn is_decrease(&self) -> bool {
        self.after < self.before
    }
    pub fn change(&self) -> i64 {
        self.after as i64 - self.before as i64
    }
}
#[derive(Debug, Clone, Default)]
pub struct IndentStats {
    pub space_lines: usize,
    pub tab_lines: usize,
    pub mixed_lines: usize,
    pub blank_lines: usize,
    pub most_common_step: usize,
}
impl IndentStats {
    pub fn analyse(src: &str) -> Self {
        let mut stats = IndentStats::default();
        let mut step_counts: std::collections::HashMap<usize, usize> =
            std::collections::HashMap::new();
        let mut prev_width: Option<usize> = None;
        for line in src.lines() {
            let trimmed = line.trim_start();
            if trimmed.is_empty() {
                stats.blank_lines += 1;
                continue;
            }
            let leading: String = line.chars().take_while(|c| c.is_whitespace()).collect();
            let has_tab = leading.contains('\t');
            let has_space = leading.contains(' ');
            if has_tab && has_space {
                stats.mixed_lines += 1;
            } else if has_tab {
                stats.tab_lines += 1;
            } else if has_space {
                stats.space_lines += 1;
            }
            let width: usize = leading.chars().map(|c| if c == '\t' { 4 } else { 1 }).sum();
            if let Some(pw) = prev_width {
                if width > pw {
                    *step_counts.entry(width - pw).or_insert(0) += 1;
                }
            }
            prev_width = Some(width);
        }
        if let Some((&step, _)) = step_counts.iter().max_by_key(|(_, &c)| c) {
            stats.most_common_step = step;
        }
        stats
    }
    pub fn code_lines(&self) -> usize {
        self.space_lines + self.tab_lines + self.mixed_lines
    }
    pub fn is_spaces_only(&self) -> bool {
        self.tab_lines == 0 && self.mixed_lines == 0
    }
    pub fn is_tabs_only(&self) -> bool {
        self.space_lines == 0 && self.mixed_lines == 0
    }
}
/// A "hanging indent" state: tracks when a line is a continuation of the previous.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct HangingIndentState {
    pub base_indent: usize,
    pub hanging: bool,
    pub hanging_indent: usize,
}
impl HangingIndentState {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(base: usize) -> Self {
        Self {
            base_indent: base,
            hanging: false,
            hanging_indent: base + 4,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn enter_hanging(&mut self) {
        self.hanging = true;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn exit_hanging(&mut self) {
        self.hanging = false;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn current_indent(&self) -> usize {
        if self.hanging {
            self.hanging_indent
        } else {
            self.base_indent
        }
    }
}
#[derive(Debug, Default)]
pub struct IndentationChecker {
    pub tab_width: usize,
    pub errors: Vec<IndentMismatchError>,
}
impl IndentationChecker {
    pub fn new(tab_width: usize) -> Self {
        Self {
            tab_width,
            errors: Vec::new(),
        }
    }
    pub fn check_transition(
        &mut self,
        line_number: usize,
        prev: &IndentLevel,
        next: &IndentLevel,
        context: &str,
    ) {
        let tw = self.tab_width;
        let pw = prev.total_width(tw);
        let nw = next.total_width(tw);
        if nw > pw && (nw - pw) % tw != 0 && tw > 0 {
            self.errors
                .push(IndentMismatchError::new(line_number, pw + tw, nw, context));
        }
    }
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty()
    }
    pub fn clear(&mut self) {
        self.errors.clear();
    }
}
/// An indent "zipper": allows navigating up and down the indent hierarchy.
#[allow(dead_code)]
pub struct IndentZipper {
    above: Vec<(usize, String)>,
    focus: (usize, String),
    below: Vec<(usize, String)>,
}
impl IndentZipper {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn from_source(source: &str) -> Option<Self> {
        let lines: Vec<_> = source
            .lines()
            .enumerate()
            .filter(|(_, l)| !l.trim().is_empty())
            .map(|(_, l)| {
                let indent = l.len() - l.trim_start().len();
                (indent, l.to_string())
            })
            .collect();
        if lines.is_empty() {
            return None;
        }
        let mut above = lines;
        let focus = above.remove(0);
        Some(Self {
            above: Vec::new(),
            focus,
            below: above,
        })
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn move_down(&mut self) -> bool {
        if self.below.is_empty() {
            return false;
        }
        let next = self.below.remove(0);
        self.above.push(std::mem::replace(&mut self.focus, next));
        true
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn move_up(&mut self) -> bool {
        if self.above.is_empty() {
            return false;
        }
        let prev = self
            .above
            .pop()
            .expect("above is non-empty per is_empty check above");
        self.below
            .insert(0, std::mem::replace(&mut self.focus, prev));
        true
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn current_indent(&self) -> usize {
        self.focus.0
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn current_content(&self) -> &str {
        &self.focus.1
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn position(&self) -> usize {
        self.above.len()
    }
}
/// Indent rule for a specific construct.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct ConstructIndentRule {
    pub construct_name: String,
    pub body_indent: usize,
    pub continuation_indent: usize,
    pub hanging_indent: usize,
}
impl ConstructIndentRule {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(name: impl Into<String>, body: usize, cont: usize, hang: usize) -> Self {
        Self {
            construct_name: name.into(),
            body_indent: body,
            continuation_indent: cont,
            hanging_indent: hang,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn standard(name: impl Into<String>) -> Self {
        Self::new(name, 2, 4, 4)
    }
}
#[derive(Debug, Clone, Default)]
pub struct ScopeTracker {
    scopes: Vec<Scope>,
}
impl ScopeTracker {
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }
    pub fn enter(&mut self, kind: impl Into<String>, indent: IndentLevel) {
        self.scopes.push(Scope::new(kind, indent));
    }
    pub fn exit(&mut self) -> Option<Scope> {
        self.scopes.pop()
    }
    pub fn current(&self) -> Option<&Scope> {
        self.scopes.last()
    }
    pub fn current_mut(&mut self) -> Option<&mut Scope> {
        self.scopes.last_mut()
    }
    pub fn resolve(&self, name: &str) -> Option<&Scope> {
        self.scopes.iter().rev().find(|s| s.is_bound(name))
    }
    pub fn depth(&self) -> usize {
        self.scopes.len()
    }
    pub fn clear(&mut self) {
        self.scopes.clear();
    }
    pub fn bind(&mut self, name: impl Into<String>) {
        if let Some(scope) = self.current_mut() {
            scope.add_binding(name);
        }
    }
}
