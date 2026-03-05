//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::ast_impl::{Decl, Located, SurfaceExpr};
pub use crate::error_impl::{ParseError, ParseErrorKind};
pub use crate::lexer::Lexer;
pub use crate::parser_impl::Parser;
pub use crate::tokens::{Span, Token, TokenKind};

use super::functions::*;

/// Represents a parse error with position and message.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ParseErrorSimple {
    pub pos: usize,
    pub message: String,
    pub recovered: bool,
}
impl ParseErrorSimple {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(pos: usize, msg: impl Into<String>) -> Self {
        Self {
            pos,
            message: msg.into(),
            recovered: false,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn recovered(mut self) -> Self {
        self.recovered = true;
        self
    }
}
/// A stack of parse checkpoints.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct CheckpointStack {
    pub(super) stack: Vec<ParseCheckpoint>,
}
impl CheckpointStack {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn push(&mut self, cp: ParseCheckpoint) {
        self.stack.push(cp);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn pop(&mut self) -> Option<ParseCheckpoint> {
        self.stack.pop()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn peek(&self) -> Option<&ParseCheckpoint> {
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
}
/// Represents a parse ambiguity between two alternatives.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ParseAmbiguity {
    pub position: usize,
    pub alternatives: Vec<String>,
    pub resolved_to: Option<String>,
}
impl ParseAmbiguity {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(pos: usize, alternatives: Vec<String>) -> Self {
        Self {
            position: pos,
            alternatives,
            resolved_to: None,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn resolve(&mut self, choice: impl Into<String>) {
        self.resolved_to = Some(choice.into());
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_resolved(&self) -> bool {
        self.resolved_to.is_some()
    }
}
/// A checkpoint for parser backtracking.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct ParseCheckpoint {
    pub position: usize,
    pub depth: usize,
    pub error_count: usize,
}
impl ParseCheckpoint {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn save(cursor: &TokenCursor, errors: usize) -> Self {
        Self {
            position: cursor.position,
            depth: cursor.depth,
            error_count: errors,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn restore(&self, cursor: &mut TokenCursor) {
        cursor.position = self.position;
        cursor.depth = self.depth;
    }
}
/// A cache key for memoizing parse results by source hash.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub struct ParseCacheKey {
    /// FNV-1a hash of the source text.
    pub hash: u64,
    /// Length of the source text in bytes.
    pub len: usize,
}
impl ParseCacheKey {
    /// Compute the cache key for a source string.
    #[allow(missing_docs)]
    pub fn from_src(src: &str) -> Self {
        let hash = fnv1a(src.as_bytes());
        Self {
            hash,
            len: src.len(),
        }
    }
}
/// A registry of parse ambiguities encountered.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct AmbiguityRegistry {
    ambiguities: Vec<ParseAmbiguity>,
}
impl AmbiguityRegistry {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            ambiguities: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn report(&mut self, amb: ParseAmbiguity) {
        self.ambiguities.push(amb);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.ambiguities.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn unresolved(&self) -> usize {
        self.ambiguities.iter().filter(|a| !a.is_resolved()).count()
    }
}
/// Configuration for a single parse run.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct ParseConfig {
    pub max_depth: usize,
    pub max_errors: usize,
    pub recover_from_errors: bool,
    pub strict_mode: bool,
    #[allow(missing_docs)]
    pub track_whitespace: bool,
    pub allow_holes: bool,
}
impl ParseConfig {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn default_config() -> Self {
        Self {
            max_depth: 1000,
            max_errors: 50,
            recover_from_errors: true,
            strict_mode: false,
            track_whitespace: false,
            allow_holes: true,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn strict() -> Self {
        Self {
            strict_mode: true,
            recover_from_errors: false,
            ..Self::default_config()
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lenient() -> Self {
        Self {
            strict_mode: false,
            recover_from_errors: true,
            max_errors: 200,
            ..Self::default_config()
        }
    }
}
/// Statistics gathered during a single parse run.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Default, Debug, Clone)]
pub struct ParseStatsExt {
    pub tokens_consumed: u64,
    pub nodes_created: u64,
    pub backtrack_count: u64,
    pub error_count: u64,
    #[allow(missing_docs)]
    pub max_depth_reached: usize,
    pub parse_time_us: u64,
}
impl ParseStatsExt {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn efficiency(&self) -> f64 {
        if self.tokens_consumed == 0 {
            return 0.0;
        }
        let useful = self.tokens_consumed.saturating_sub(self.backtrack_count);
        useful as f64 / self.tokens_consumed as f64
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn error_rate(&self) -> f64 {
        if self.nodes_created == 0 {
            return 0.0;
        }
        self.error_count as f64 / self.nodes_created as f64
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn summary(&self) -> String {
        format!(
            "tokens={} nodes={} backtracks={} errors={} depth={} time={}us efficiency={:.1}%",
            self.tokens_consumed,
            self.nodes_created,
            self.backtrack_count,
            self.error_count,
            self.max_depth_reached,
            self.parse_time_us,
            self.efficiency() * 100.0,
        )
    }
}
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct PackratEntry {
    pub end_pos: usize,
    pub success: bool,
    pub result_repr: String,
}
/// A "fuel" mechanism to prevent infinite loops in parsers.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ParseFuel {
    pub(super) remaining: usize,
}
impl ParseFuel {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(fuel: usize) -> Self {
        Self { remaining: fuel }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn consume(&mut self, amount: usize) -> bool {
        if self.remaining >= amount {
            self.remaining -= amount;
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_fuel(&self) -> bool {
        self.remaining > 0
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn remaining(&self) -> usize {
        self.remaining
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn refuel(&mut self, amount: usize) {
        self.remaining = self.remaining.saturating_add(amount);
    }
}
/// A batch of parse requests.
#[derive(Debug, Default)]
#[allow(missing_docs)]
pub struct ParseBatch {
    /// Source strings with associated names.
    pub entries: Vec<(String, String)>,
}
impl ParseBatch {
    /// Create an empty batch.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a named source entry.
    #[allow(missing_docs)]
    pub fn add(&mut self, name: &str, src: &str) {
        self.entries.push((name.to_string(), src.to_string()));
    }
    /// Number of entries in the batch.
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the batch is empty.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Execute the batch and return a `ParseSession`.
    #[allow(missing_docs)]
    pub fn execute(self) -> ParseSession {
        let mut session = ParseSession::new();
        for (name, src) in self.entries {
            session.parse_file(&name, &src);
        }
        session
    }
}
/// A token cursor for tracking position during parsing.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct TokenCursor {
    pub position: usize,
    pub end: usize,
    pub depth: usize,
}
impl TokenCursor {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(end: usize) -> Self {
        Self {
            position: 0,
            end,
            depth: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn advance(&mut self) {
        if self.position < self.end {
            self.position += 1;
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn retreat(&mut self) {
        if self.position > 0 {
            self.position -= 1;
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_at_end(&self) -> bool {
        self.position >= self.end
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn remaining(&self) -> usize {
        self.end.saturating_sub(self.position)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn enter_scope(&mut self) {
        self.depth += 1;
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn exit_scope(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_at_root(&self) -> bool {
        self.depth == 0
    }
}
/// Represents a parser combinator result.
#[allow(dead_code)]
#[allow(missing_docs)]
pub enum CombResult<T> {
    Ok(T, usize),
    Err(String, usize),
}
impl<T> CombResult<T> {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_ok(&self) -> bool {
        matches!(self, CombResult::Ok(_, _))
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_err(&self) -> bool {
        matches!(self, CombResult::Err(_, _))
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn position(&self) -> usize {
        match self {
            CombResult::Ok(_, p) | CombResult::Err(_, p) => *p,
        }
    }
}
/// Represents a parser lookahead result.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LookaheadResult {
    Matches(usize),
    NoMatch,
    Ambiguous,
}
/// A registry of operator fixities.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FixityRegistry {
    entries: std::collections::HashMap<String, Fixity>,
}
impl FixityRegistry {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let mut reg = Self {
            entries: std::collections::HashMap::new(),
        };
        reg.add("+", Fixity::InfixLeft(65));
        reg.add("-", Fixity::InfixLeft(65));
        reg.add("*", Fixity::InfixLeft(70));
        reg.add("/", Fixity::InfixLeft(70));
        reg.add("^", Fixity::InfixRight(75));
        reg.add("=", Fixity::InfixNone(50));
        reg.add("<", Fixity::InfixNone(50));
        reg.add(">", Fixity::InfixNone(50));
        reg.add("&&", Fixity::InfixRight(35));
        reg.add("||", Fixity::InfixRight(30));
        reg
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, op: impl Into<String>, fixity: Fixity) {
        self.entries.insert(op.into(), fixity);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&self, op: &str) -> Option<&Fixity> {
        self.entries.get(op)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}
/// A parser recovery decision.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RecoveryDecision {
    pub strategy: RecoveryStrategy,
    pub tokens_to_skip: usize,
    pub message: String,
}
impl RecoveryDecision {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn skip(n: usize, msg: impl Into<String>) -> Self {
        Self {
            strategy: RecoveryStrategy::Skip,
            tokens_to_skip: n,
            message: msg.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn sync(msg: impl Into<String>) -> Self {
        Self {
            strategy: RecoveryStrategy::SyncToKeyword,
            tokens_to_skip: 0,
            message: msg.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn abandon(msg: impl Into<String>) -> Self {
        Self {
            strategy: RecoveryStrategy::Abandon,
            tokens_to_skip: 0,
            message: msg.into(),
        }
    }
}
/// A parser result type with error accumulation.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ParseResultWithErrors<T> {
    pub value: Option<T>,
    pub errors: Vec<ParseErrorSimple>,
}
impl<T> ParseResultWithErrors<T> {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn ok(value: T) -> Self {
        Self {
            value: Some(value),
            errors: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn err(e: ParseErrorSimple) -> Self {
        Self {
            value: None,
            errors: vec![e],
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn ok_with_errors(value: T, errors: Vec<ParseErrorSimple>) -> Self {
        Self {
            value: Some(value),
            errors,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_ok(&self) -> bool {
        self.value.is_some()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
}
/// Result for a single file parsed within a session.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct ParseFileResult {
    /// File name (or `"<stdin>"`).
    pub filename: String,
    /// Successfully parsed declarations.
    pub decls: Vec<Located<Decl>>,
    /// Errors encountered.
    #[allow(missing_docs)]
    pub errors: Vec<ParseError>,
}
impl ParseFileResult {
    /// Whether the file parsed without errors.
    #[allow(missing_docs)]
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
    /// Number of declarations successfully parsed.
    #[allow(missing_docs)]
    pub fn decl_count(&self) -> usize {
        self.decls.len()
    }
}
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct TraceEvent {
    pub rule: String,
    pub start_pos: usize,
    pub end_pos: usize,
    pub success: bool,
}
/// A memoization table for parser results (Packrat parsing).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct PackratTable {
    entries: std::collections::HashMap<(usize, String), PackratEntry>,
}
impl PackratTable {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            entries: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn lookup(&self, pos: usize, rule: &str) -> Option<&PackratEntry> {
        self.entries.get(&(pos, rule.to_string()))
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn store(&mut self, pos: usize, rule: impl Into<String>, entry: PackratEntry) {
        self.entries.insert((pos, rule.into()), entry);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn hit_rate_estimate(&self) -> f64 {
        if self.entries.is_empty() {
            return 0.0;
        }
        let hits = self.entries.values().filter(|e| e.success).count();
        hits as f64 / self.entries.len() as f64
    }
}
/// Tracks all recovery events during parsing.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RecoveryLog {
    entries: Vec<(usize, RecoveryDecision)>,
}
impl RecoveryLog {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record(&mut self, pos: usize, decision: RecoveryDecision) {
        self.entries.push((pos, decision));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn strategies_used(&self) -> Vec<RecoveryStrategy> {
        self.entries.iter().map(|(_, d)| d.strategy).collect()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn abandon_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|(_, d)| d.strategy == RecoveryStrategy::Abandon)
            .count()
    }
}
/// Holds the state for a single parse "frame" (a call to a recursive production).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ParseFrame {
    pub rule: String,
    pub start_pos: usize,
    pub depth: usize,
    pub in_type: bool,
    #[allow(missing_docs)]
    pub in_pattern: bool,
}
impl ParseFrame {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(rule: impl Into<String>, pos: usize, depth: usize) -> Self {
        Self {
            rule: rule.into(),
            start_pos: pos,
            depth,
            in_type: false,
            in_pattern: false,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn for_type(mut self) -> Self {
        self.in_type = true;
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn for_pattern(mut self) -> Self {
        self.in_pattern = true;
        self
    }
}
/// Source position with file information.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourcePos {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}
impl SourcePos {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(file: impl Into<String>, line: usize, col: usize, offset: usize) -> Self {
        Self {
            file: file.into(),
            line,
            column: col,
            byte_offset: offset,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn unknown() -> Self {
        Self::new("<unknown>", 0, 0, 0)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn display(&self) -> String {
        format!("{}:{}:{}", self.file, self.line + 1, self.column + 1)
    }
}
/// Maps byte offsets in a source file to line and column numbers.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct SourceMap {
    /// Starting byte offset of each line.
    pub(super) line_starts: Vec<usize>,
    /// Total length of the source.
    source_len: usize,
}
impl SourceMap {
    /// Build a source map from source text.
    #[allow(missing_docs)]
    pub fn new(src: &str) -> Self {
        let mut line_starts = vec![0];
        for (i, b) in src.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push(i + 1);
            }
        }
        Self {
            line_starts,
            source_len: src.len(),
        }
    }
    /// Convert a byte offset to (line, col), both 1-based.
    #[allow(missing_docs)]
    pub fn offset_to_line_col(&self, offset: usize) -> (u32, u32) {
        let line = match self.line_starts.binary_search(&offset) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let col = offset - self.line_starts[line];
        ((line + 1) as u32, (col + 1) as u32)
    }
    /// Number of lines in the source.
    #[allow(missing_docs)]
    pub fn num_lines(&self) -> usize {
        self.line_starts.len()
    }
    /// Total source length in bytes.
    #[allow(missing_docs)]
    pub fn source_len(&self) -> usize {
        self.source_len
    }
}
/// A ring buffer of `Token`s used for lookahead parsing.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct ParseBuffer {
    tokens: std::collections::VecDeque<Token>,
    /// Maximum lookahead.
    max_lookahead: usize,
}
impl ParseBuffer {
    /// Create an empty buffer.
    #[allow(missing_docs)]
    pub fn new(max_lookahead: usize) -> Self {
        Self {
            tokens: std::collections::VecDeque::new(),
            max_lookahead,
        }
    }
    /// Push a token onto the back of the buffer.
    #[allow(missing_docs)]
    pub fn push(&mut self, tok: Token) {
        if self.tokens.len() >= self.max_lookahead {
            self.tokens.pop_front();
        }
        self.tokens.push_back(tok);
    }
    /// Peek at the front of the buffer.
    #[allow(missing_docs)]
    pub fn front(&self) -> Option<&Token> {
        self.tokens.front()
    }
    /// Pop the front token.
    #[allow(missing_docs)]
    pub fn pop(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }
    /// Number of buffered tokens.
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    /// Whether the buffer is empty.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
    /// Clear all buffered tokens.
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        self.tokens.clear();
    }
}
/// A sequence of token kinds expected at the current position.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ExpectedSet {
    expected: Vec<String>,
}
impl ExpectedSet {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            expected: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, what: impl Into<String>) {
        self.expected.push(what.into());
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        self.expected.clear();
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.expected.is_empty()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.expected.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn to_message(&self) -> String {
        match self.expected.len() {
            0 => "nothing expected".to_string(),
            1 => format!("expected {}", self.expected[0]),
            _ => {
                let last = &self.expected[self.expected.len() - 1];
                let rest = &self.expected[..self.expected.len() - 1];
                format!("expected {} or {}", rest.join(", "), last)
            }
        }
    }
}
/// Operator fixity information.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fixity {
    InfixLeft(u8),
    InfixRight(u8),
    InfixNone(u8),
    Prefix(u8),
    Postfix(u8),
}
impl Fixity {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn precedence(&self) -> u8 {
        match self {
            Fixity::InfixLeft(p)
            | Fixity::InfixRight(p)
            | Fixity::InfixNone(p)
            | Fixity::Prefix(p)
            | Fixity::Postfix(p) => *p,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_infix(&self) -> bool {
        matches!(
            self,
            Fixity::InfixLeft(_) | Fixity::InfixRight(_) | Fixity::InfixNone(_)
        )
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_right_assoc(&self) -> bool {
        matches!(self, Fixity::InfixRight(_))
    }
}
/// A multi-file parse session that tracks per-file results and statistics.
#[derive(Debug, Default)]
#[allow(missing_docs)]
pub struct ParseSession {
    /// File names in order of addition.
    pub file_names: Vec<String>,
    /// Parse results for each file.
    pub results: Vec<ParseFileResult>,
    /// Aggregate statistics.
    #[allow(missing_docs)]
    pub stats: ParseStats,
}
impl ParseSession {
    /// Create an empty session.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Parse a named file's source text and add it to the session.
    #[allow(missing_docs)]
    pub fn parse_file(&mut self, filename: &str, src: &str) {
        let mut errors = Vec::new();
        let tokens = Lexer::new(src).tokenize();
        let mut parser = Parser::new(tokens);
        let mut decls = Vec::new();
        loop {
            match parser.parse_decl() {
                Ok(d) => decls.push(d),
                Err(e) if e.is_eof() => break,
                Err(e) => {
                    errors.push(e);
                    break;
                }
            }
        }
        self.stats.files_parsed += 1;
        self.stats.decls_parsed += decls.len() as u64;
        self.stats.errors_total += errors.len() as u64;
        self.file_names.push(filename.to_string());
        self.results.push(ParseFileResult {
            filename: filename.to_string(),
            decls,
            errors,
        });
    }
    /// Whether all files in the session parsed without errors.
    #[allow(missing_docs)]
    pub fn all_ok(&self) -> bool {
        self.results.iter().all(|r| r.is_ok())
    }
    /// Collect all errors across files.
    #[allow(missing_docs)]
    pub fn all_errors(&self) -> Vec<&ParseError> {
        self.results.iter().flat_map(|r| r.errors.iter()).collect()
    }
    /// Total number of declarations across all files.
    #[allow(missing_docs)]
    pub fn total_decls(&self) -> usize {
        self.results.iter().map(|r| r.decl_count()).sum()
    }
    /// Number of files in the session.
    #[allow(missing_docs)]
    pub fn file_count(&self) -> usize {
        self.file_names.len()
    }
}
/// A lightweight wrapper around a `Vec<Token>` that provides
/// cursor-based traversal without requiring a mutable `Parser`.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct TokenStream {
    pub(super) tokens: Vec<Token>,
    pub(super) pos: usize,
}
impl TokenStream {
    /// Create a new token stream from a token vector.
    #[allow(missing_docs)]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    /// Lex `src` and wrap the result.
    #[allow(missing_docs)]
    pub fn from_src(src: &str) -> Self {
        Self::new(Lexer::new(src).tokenize())
    }
    /// Peek at the current token without consuming it.
    #[allow(missing_docs)]
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    /// Advance and return the current token.
    #[allow(clippy::should_implement_trait)]
    #[allow(missing_docs)]
    pub fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.pos)?;
        self.pos += 1;
        Some(tok)
    }
    /// Whether the stream is exhausted.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.pos >= self.tokens.len()
    }
    /// Number of remaining tokens.
    #[allow(missing_docs)]
    pub fn remaining(&self) -> usize {
        self.tokens.len().saturating_sub(self.pos)
    }
    /// Total number of tokens (including consumed ones).
    #[allow(missing_docs)]
    pub fn total_len(&self) -> usize {
        self.tokens.len()
    }
    /// Reset the cursor to the beginning.
    #[allow(missing_docs)]
    pub fn reset(&mut self) {
        self.pos = 0;
    }
    /// Collect remaining tokens into a `Vec`.
    #[allow(missing_docs)]
    pub fn collect_remaining(&self) -> Vec<&Token> {
        self.tokens[self.pos..].iter().collect()
    }
}
/// Aggregate statistics from one or more parse operations.
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct ParseStats {
    /// Number of files parsed.
    pub files_parsed: u64,
    /// Total declarations parsed (across all files).
    pub decls_parsed: u64,
    /// Total parse errors encountered.
    #[allow(missing_docs)]
    pub errors_total: u64,
    /// Total tokens lexed.
    pub tokens_lexed: u64,
    /// Total source bytes processed.
    pub bytes_processed: u64,
}
impl ParseStats {
    /// Create zero-initialized stats.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Average declarations per file.
    #[allow(missing_docs)]
    pub fn avg_decls_per_file(&self) -> f64 {
        if self.files_parsed == 0 {
            0.0
        } else {
            self.decls_parsed as f64 / self.files_parsed as f64
        }
    }
    /// Error rate: errors per declaration.
    #[allow(missing_docs)]
    pub fn error_rate(&self) -> f64 {
        if self.decls_parsed == 0 {
            0.0
        } else {
            self.errors_total as f64 / self.decls_parsed as f64
        }
    }
    /// Whether parsing was entirely error-free.
    #[allow(missing_docs)]
    pub fn is_clean(&self) -> bool {
        self.errors_total == 0
    }
}
/// The kind of a parse annotation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum AnnotationKind {
    /// Informational note.
    Info,
    /// Non-fatal deprecation warning.
    Deprecated,
    /// Suggestion for alternative syntax.
    Suggestion,
}
/// Error recovery strategies for the parser.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecoveryStrategy {
    Skip,
    InsertToken,
    SyncToKeyword,
    Abandon,
}
/// A quality rating for a parse result.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[allow(missing_docs)]
pub enum ParseQuality {
    /// Parse failed completely.
    Failed,
    /// Parse succeeded with recoverable errors.
    Partial,
    /// Parse succeeded with warnings only.
    WithWarnings,
    /// Parse succeeded cleanly.
    Clean,
}
impl ParseQuality {
    /// Compute a quality rating from error/warning counts.
    #[allow(missing_docs)]
    pub fn rate(errors: usize, warnings: usize) -> Self {
        if errors > 0 {
            ParseQuality::Failed
        } else if warnings > 0 {
            ParseQuality::WithWarnings
        } else {
            ParseQuality::Clean
        }
    }
    /// Whether the parse result is usable (no hard errors).
    #[allow(missing_docs)]
    pub fn is_usable(&self) -> bool {
        *self >= ParseQuality::Partial
    }
}
/// A multi-stage parse pipeline that applies a sequence of
/// transformations to the token stream before parsing.
#[derive(Debug, Default)]
#[allow(missing_docs)]
pub struct ParsePipeline {
    /// Stage names for diagnostics.
    pub stages: Vec<String>,
}
impl ParsePipeline {
    /// Create an empty pipeline.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a named stage.
    #[allow(missing_docs)]
    pub fn add_stage(&mut self, name: &str) {
        self.stages.push(name.to_string());
    }
    /// Number of stages.
    #[allow(missing_docs)]
    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }
    /// Execute the pipeline on a source string, returning a token stream.
    #[allow(missing_docs)]
    pub fn execute(&self, src: &str) -> TokenStream {
        TokenStream::from_src(src)
    }
}
/// A depth-limited recursive descent helper.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct DepthLimiter {
    current: usize,
    max: usize,
}
impl DepthLimiter {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(max: usize) -> Self {
        Self { current: 0, max }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn enter(&mut self) -> bool {
        if self.current >= self.max {
            return false;
        }
        self.current += 1;
        true
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn exit(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn depth(&self) -> usize {
        self.current
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_at_limit(&self) -> bool {
        self.current >= self.max
    }
}
/// An annotation attached to a parse result.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct ParseAnnotation {
    /// Annotation kind.
    pub kind: AnnotationKind,
    /// Source span.
    pub span: Span,
    /// Message.
    #[allow(missing_docs)]
    pub message: String,
}
impl ParseAnnotation {
    /// Create a new annotation.
    #[allow(missing_docs)]
    pub fn new(kind: AnnotationKind, span: Span, message: &str) -> Self {
        Self {
            kind,
            span,
            message: message.to_string(),
        }
    }
    /// Create an info annotation.
    #[allow(missing_docs)]
    pub fn info(span: Span, message: &str) -> Self {
        Self::new(AnnotationKind::Info, span, message)
    }
    /// Create a deprecation annotation.
    #[allow(missing_docs)]
    pub fn deprecated(span: Span, message: &str) -> Self {
        Self::new(AnnotationKind::Deprecated, span, message)
    }
}
/// A context for operator-precedence parsing (Pratt parsing).
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct PrattContext {
    pub min_prec: u8,
    pub depth: usize,
    pub max_depth: usize,
}
impl PrattContext {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(min_prec: u8) -> Self {
        Self {
            min_prec,
            depth: 0,
            max_depth: 200,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_min_prec(&self, p: u8) -> Self {
        Self {
            min_prec: p,
            depth: self.depth + 1,
            max_depth: self.max_depth,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_too_deep(&self) -> bool {
        self.depth >= self.max_depth
    }
}
/// Flags controlling parser behavior.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct ParseMode {
    /// Whether to allow tactics inside term mode.
    pub allow_tactics: bool,
    /// Whether to recover from errors (continue after first error).
    pub recover_on_error: bool,
    /// Whether to parse in lenient mode (accept partial expressions).
    #[allow(missing_docs)]
    pub lenient: bool,
}
impl ParseMode {
    /// Strict mode: no recovery, no tactics.
    #[allow(missing_docs)]
    pub fn strict() -> Self {
        Self {
            allow_tactics: false,
            recover_on_error: false,
            lenient: false,
        }
    }
    /// Lenient mode: allow partial results.
    #[allow(missing_docs)]
    pub fn lenient() -> Self {
        Self {
            allow_tactics: false,
            recover_on_error: true,
            lenient: true,
        }
    }
}
/// A compact bitset for up to 64 token kinds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct TokenKindSet {
    bits: u64,
}
impl TokenKindSet {
    /// Empty set.
    #[allow(missing_docs)]
    pub fn empty() -> Self {
        Self { bits: 0 }
    }
    /// Insert a kind by discriminant index.
    #[allow(missing_docs)]
    pub fn insert(&mut self, idx: u32) {
        if idx < 64 {
            self.bits |= 1 << idx;
        }
    }
    /// Test whether an index is present.
    #[allow(missing_docs)]
    pub fn contains(&self, idx: u32) -> bool {
        idx < 64 && (self.bits >> idx) & 1 != 0
    }
    /// Whether the set is empty.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }
    /// Union of two sets.
    #[allow(missing_docs)]
    pub fn union(self, other: Self) -> Self {
        Self {
            bits: self.bits | other.bits,
        }
    }
    /// Intersection of two sets.
    #[allow(missing_docs)]
    pub fn intersect(self, other: Self) -> Self {
        Self {
            bits: self.bits & other.bits,
        }
    }
}
/// Tracks which tokens were consumed by each parser rule.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ParseTrace {
    events: Vec<TraceEvent>,
    max_events: usize,
}
impl ParseTrace {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Vec::new(),
            max_events,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn enter(&mut self, rule: impl Into<String>, pos: usize) -> usize {
        let idx = self.events.len();
        if self.events.len() < self.max_events {
            self.events.push(TraceEvent {
                rule: rule.into(),
                start_pos: pos,
                end_pos: pos,
                success: false,
            });
        }
        idx
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn exit(&mut self, idx: usize, end_pos: usize, success: bool) {
        if let Some(e) = self.events.get_mut(idx) {
            e.end_pos = end_pos;
            e.success = success;
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn success_count(&self) -> usize {
        self.events.iter().filter(|e| e.success).count()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn fail_count(&self) -> usize {
        self.events.iter().filter(|e| !e.success).count()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total(&self) -> usize {
        self.events.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn most_failing_rule(&self) -> Option<&str> {
        let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for e in &self.events {
            if !e.success {
                *counts.entry(e.rule.as_str()).or_insert(0) += 1;
            }
        }
        counts.into_iter().max_by_key(|(_, c)| *c).map(|(r, _)| r)
    }
}
/// A stack of parse frames for debugging.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ParseStack {
    frames: Vec<ParseFrame>,
}
impl ParseStack {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn push(&mut self, frame: ParseFrame) {
        self.frames.push(frame);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn pop(&mut self) -> Option<ParseFrame> {
        self.frames.pop()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn current_rule(&self) -> Option<&str> {
        self.frames.last().map(|f| f.rule.as_str())
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn in_type(&self) -> bool {
        self.frames.iter().any(|f| f.in_type)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn in_pattern(&self) -> bool {
        self.frames.iter().any(|f| f.in_pattern)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn rules_string(&self) -> String {
        self.frames
            .iter()
            .map(|f| f.rule.as_str())
            .collect::<Vec<_>>()
            .join(" > ")
    }
}
/// A summary of parse errors across a session.
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct ParseErrorSummary {
    /// Total errors.
    pub total: usize,
    /// Errors by file.
    pub by_file: Vec<(String, usize)>,
}
impl ParseErrorSummary {
    /// Build a summary from a parse session.
    #[allow(missing_docs)]
    pub fn from_session(session: &ParseSession) -> Self {
        let mut by_file = Vec::new();
        let mut total = 0;
        for r in &session.results {
            let n = r.errors.len();
            if n > 0 {
                by_file.push((r.filename.clone(), n));
                total += n;
            }
        }
        Self { total, by_file }
    }
    /// Whether there are no errors.
    #[allow(missing_docs)]
    pub fn is_clean(&self) -> bool {
        self.total == 0
    }
    /// File with the most errors (if any).
    #[allow(missing_docs)]
    pub fn worst_file(&self) -> Option<&str> {
        self.by_file
            .iter()
            .max_by_key(|(_, n)| *n)
            .map(|(f, _)| f.as_str())
    }
}
