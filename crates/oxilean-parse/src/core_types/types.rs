//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::parser_impl::Parser;

use std::collections::HashMap;

/// A parse context: wraps a source file plus configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct ParseContext {
    /// The source file being parsed.
    pub file: SourceFile,
    /// Parser configuration.
    pub config: ParserConfig,
    /// Accumulated diagnostics during parsing.
    #[allow(missing_docs)]
    pub diagnostics: ParseDiagnostics,
}
impl ParseContext {
    /// Create a new parse context.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(file: SourceFile, config: ParserConfig) -> Self {
        Self {
            file,
            config,
            diagnostics: ParseDiagnostics::new(),
        }
    }
    /// Create a context from a source string with default config.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn from_source(source: impl Into<String>) -> Self {
        Self::new(SourceFile::virtual_(source), ParserConfig::default())
    }
    /// Check if any errors occurred during parsing.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_errors(&self) -> bool {
        !self.diagnostics.is_ok()
    }
    /// Emit an error diagnostic.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn emit_error(&mut self, msg: impl Into<String>) {
        self.diagnostics.error(msg);
    }
    /// Emit a warning diagnostic.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn emit_warning(&mut self, msg: impl Into<String>, start: usize, end: usize) {
        self.diagnostics.warn(msg, start, end);
    }
}
/// A simple import graph.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ImportGraph {
    /// Map from module name to list of imported modules
    pub imports: std::collections::HashMap<String, Vec<String>>,
}
impl ImportGraph {
    /// Create a new empty import graph.
    #[allow(dead_code)]
    pub fn new() -> Self {
        ImportGraph {
            imports: std::collections::HashMap::new(),
        }
    }
    /// Add an import edge.
    #[allow(dead_code)]
    pub fn add_import(&mut self, from: &str, to: &str) {
        self.imports
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
    }
    /// Returns all imports of a module.
    #[allow(dead_code)]
    pub fn imports_of(&self, module: &str) -> &[String] {
        self.imports
            .get(module)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
    /// Returns the total number of import edges.
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.imports.values().map(|v| v.len()).sum()
    }
}
/// A simple string pool that interns unique strings.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SimpleStringPoolExt {
    /// All interned strings
    pub pool: Vec<String>,
}
impl SimpleStringPoolExt {
    /// Create an empty pool.
    #[allow(dead_code)]
    pub fn new() -> Self {
        SimpleStringPoolExt { pool: Vec::new() }
    }
    /// Intern a string, returning its index.
    #[allow(dead_code)]
    pub fn intern(&mut self, s: &str) -> usize {
        if let Some(idx) = self.pool.iter().position(|x| x == s) {
            return idx;
        }
        let idx = self.pool.len();
        self.pool.push(s.to_string());
        idx
    }
    /// Retrieve an interned string by index.
    #[allow(dead_code)]
    pub fn get(&self, idx: usize) -> Option<&str> {
        self.pool.get(idx).map(|s| s.as_str())
    }
    /// Returns the number of interned strings.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.pool.len()
    }
    /// Returns true if the pool is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }
}
/// A source file summary.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct SourceSummary {
    /// Number of lines
    pub lines: usize,
    /// Number of words
    pub words: usize,
    /// Number of characters
    pub chars: usize,
    /// Number of blank lines
    pub blank_lines: usize,
}
impl SourceSummary {
    /// Compute a summary from source text.
    #[allow(dead_code)]
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(src: &str) -> Self {
        let mut lines = src.lines().count();
        if src.ends_with('\n') {
            lines += 1;
        }
        let blank_lines = src.lines().filter(|l| l.trim().is_empty()).count();
        let words = src.split_whitespace().count();
        let chars = src.chars().count();
        SourceSummary {
            lines,
            words,
            chars,
            blank_lines,
        }
    }
    /// Format as a string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!(
            "lines={} words={} chars={} blank={}",
            self.lines, self.words, self.chars, self.blank_lines
        )
    }
}
/// A parsed source file with its path and content.
///
/// Wraps the raw source text together with metadata that is useful
/// for diagnostics and IDE integration.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct SourceFile {
    /// Path to the source file (may be virtual, e.g., `<repl>`).
    pub path: String,
    /// The raw source text.
    pub source: String,
    /// Length of the source in bytes.
    #[allow(missing_docs)]
    pub byte_len: usize,
}
impl SourceFile {
    /// Create a new source file from a path and source string.
    #[allow(missing_docs)]
    pub fn new(path: impl Into<String>, source: impl Into<String>) -> Self {
        let source = source.into();
        let byte_len = source.len();
        SourceFile {
            path: path.into(),
            source,
            byte_len,
        }
    }
    /// Create a virtual source file (e.g., for REPL input).
    #[allow(missing_docs)]
    pub fn virtual_(source: impl Into<String>) -> Self {
        Self::new("<virtual>", source)
    }
    /// Returns the number of lines in the source file.
    #[allow(missing_docs)]
    pub fn line_count(&self) -> usize {
        self.source.lines().count()
    }
    /// Returns `true` if the source file is empty.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.source.is_empty()
    }
    /// Returns the source text of a specific line (0-indexed).
    ///
    /// Returns `None` if the line index is out of bounds.
    #[allow(missing_docs)]
    pub fn line(&self, idx: usize) -> Option<&str> {
        self.source.lines().nth(idx)
    }
}
/// Represents a syntactic "hole" in the AST: a position where a term is missing.
///
/// Holes arise from incomplete expressions like `_` or unresolved placeholders.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct SyntacticHole {
    /// Optional human hint about what should fill this hole.
    pub hint: Option<String>,
    /// Source byte offset.
    pub offset: usize,
}
impl SyntacticHole {
    /// Create a hole without a hint.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn anonymous(offset: usize) -> Self {
        Self { hint: None, offset }
    }
    /// Create a hole with a hint.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_hint(hint: impl Into<String>, offset: usize) -> Self {
        Self {
            hint: Some(hint.into()),
            offset,
        }
    }
    /// Returns `true` if this hole has a hint.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn has_hint(&self) -> bool {
        self.hint.is_some()
    }
}
/// A simple token buffer for look-ahead parsing.
///
/// Wraps a token stream and provides lookahead operations without
/// consuming the underlying iterator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct TokenBuffer {
    /// All tokens stored in the buffer.
    tokens: Vec<String>,
    /// Current read position.
    pos: usize,
}
impl TokenBuffer {
    /// Create a token buffer from a list of token strings.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(tokens: Vec<String>) -> Self {
        Self { tokens, pos: 0 }
    }
    /// Peek at the current token without consuming.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn peek(&self) -> Option<&str> {
        self.tokens.get(self.pos).map(String::as_str)
    }
    /// Peek `n` positions ahead (0 = current).
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn peek_at(&self, n: usize) -> Option<&str> {
        self.tokens.get(self.pos + n).map(String::as_str)
    }
    /// Advance and return the consumed token.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn advance(&mut self) -> Option<&str> {
        let tok = self.tokens.get(self.pos).map(String::as_str);
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }
    /// Consume the current token if it equals `expected`.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn eat(&mut self, expected: &str) -> bool {
        if self.peek() == Some(expected) {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    /// Return remaining token count.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn remaining(&self) -> usize {
        self.tokens.len().saturating_sub(self.pos)
    }
    /// Check if at end of token stream.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }
    /// Mark the current position (for backtracking).
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn mark(&self) -> usize {
        self.pos
    }
    /// Reset to a previously marked position.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn reset_to(&mut self, mark: usize) {
        self.pos = mark;
    }
}
/// A simple token frequency map.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TokenFrequencyMapExt {
    /// Map from token text to frequency
    pub freq: std::collections::HashMap<String, usize>,
}
impl TokenFrequencyMapExt {
    /// Create a new empty map.
    #[allow(dead_code)]
    pub fn new() -> Self {
        TokenFrequencyMapExt {
            freq: std::collections::HashMap::new(),
        }
    }
    /// Record a token.
    #[allow(dead_code)]
    pub fn record(&mut self, token: &str) {
        *self.freq.entry(token.to_string()).or_insert(0) += 1;
    }
    /// Most frequent token.
    #[allow(dead_code)]
    pub fn most_frequent(&self) -> Option<(&str, usize)> {
        self.freq
            .iter()
            .max_by_key(|(_, c)| *c)
            .map(|(k, &c)| (k.as_str(), c))
    }
    /// Returns the total number of recorded tokens.
    #[allow(dead_code)]
    pub fn total(&self) -> usize {
        self.freq.values().sum()
    }
}
/// A source file registry that tracks all loaded files.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SourceFileRegistry {
    /// Map from filename to source content
    pub files: std::collections::HashMap<String, String>,
    /// File IDs in order of loading
    pub file_order: Vec<String>,
}
impl SourceFileRegistry {
    /// Create a new empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        SourceFileRegistry {
            files: std::collections::HashMap::new(),
            file_order: Vec::new(),
        }
    }
    /// Add a source file.
    #[allow(dead_code)]
    pub fn add(&mut self, name: &str, content: &str) {
        self.files.insert(name.to_string(), content.to_string());
        if !self.file_order.contains(&name.to_string()) {
            self.file_order.push(name.to_string());
        }
    }
    /// Get the content of a file.
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&str> {
        self.files.get(name).map(|s| s.as_str())
    }
    /// Returns the number of files.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.files.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}
/// A parse context carrying source and flags.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ParseContextExt {
    /// The source text
    pub source: String,
    /// Parse flags
    pub flags: ParseFlagsExt,
    /// Source filename (for error messages)
    pub filename: Option<String>,
}
impl ParseContextExt {
    /// Create a new parse context.
    #[allow(dead_code)]
    pub fn new(source: &str) -> Self {
        ParseContextExt {
            source: source.to_string(),
            flags: ParseFlagsExt::default_flags(),
            filename: None,
        }
    }
    /// Set the filename.
    #[allow(dead_code)]
    pub fn with_filename(mut self, name: &str) -> Self {
        self.filename = Some(name.to_string());
        self
    }
    /// Set flags.
    #[allow(dead_code)]
    pub fn with_flags(mut self, flags: ParseFlagsExt) -> Self {
        self.flags = flags;
        self
    }
}
/// A collection of phase timers.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct PipelineTimingsExt {
    /// All recorded timings
    pub timings: Vec<PhaseTimerExt>,
}
impl PipelineTimingsExt {
    /// Create a new empty timings record.
    #[allow(dead_code)]
    pub fn new() -> Self {
        PipelineTimingsExt {
            timings: Vec::new(),
        }
    }
    /// Record a phase timing.
    #[allow(dead_code)]
    pub fn record(&mut self, phase: CompilePhaseExt, duration_us: u64) {
        self.timings.push(PhaseTimerExt { phase, duration_us });
    }
    /// Total duration in microseconds.
    #[allow(dead_code)]
    pub fn total_us(&self) -> u64 {
        self.timings.iter().map(|t| t.duration_us).sum()
    }
    /// Format all timings.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        self.timings
            .iter()
            .map(|t| format!("{}: {}us", t.phase, t.duration_us))
            .collect::<Vec<_>>()
            .join(", ")
    }
}
/// Configuration options for the parser.
///
/// These influence how the parser handles ambiguity, error recovery,
/// and diagnostic reporting.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct ParserConfig {
    /// Maximum expression nesting depth before the parser gives up.
    ///
    /// Default: 512.
    pub max_depth: usize,
    /// Whether to enable experimental Unicode operator support.
    ///
    /// Default: `true`.
    #[allow(missing_docs)]
    pub unicode_ops: bool,
    /// Whether to emit "did you mean?" suggestions on parse errors.
    ///
    /// Default: `true`.
    pub suggestions: bool,
    /// Whether to allow recovery from parse errors and continue parsing.
    ///
    /// Default: `false` (strict mode).
    #[allow(missing_docs)]
    pub error_recovery: bool,
    /// Whether `#check`, `#eval`, and similar commands are permitted.
    ///
    /// Default: `true`.
    pub allow_commands: bool,
}
impl ParserConfig {
    /// Create a new parser config with default settings.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the maximum nesting depth.
    #[allow(missing_docs)]
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
    /// Enable or disable unicode operator support.
    #[allow(missing_docs)]
    pub fn with_unicode_ops(mut self, enabled: bool) -> Self {
        self.unicode_ops = enabled;
        self
    }
    /// Enable error recovery mode.
    #[allow(missing_docs)]
    pub fn with_error_recovery(mut self, enabled: bool) -> Self {
        self.error_recovery = enabled;
        self
    }
    /// Disable all optional features for strict minimal parsing.
    #[allow(missing_docs)]
    pub fn strict() -> Self {
        ParserConfig {
            max_depth: 256,
            unicode_ops: false,
            suggestions: false,
            error_recovery: false,
            allow_commands: false,
        }
    }
}
/// A simple parse statistics record.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Default)]
pub struct ParseStatsExt {
    /// Number of tokens processed
    pub tokens_processed: usize,
    /// Number of declarations parsed
    pub decls_parsed: usize,
    /// Number of errors encountered
    pub errors: usize,
}
impl ParseStatsExt {
    /// Create a new empty record.
    #[allow(dead_code)]
    pub fn new() -> Self {
        ParseStatsExt::default()
    }
    /// Format the stats.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        format!(
            "tokens={} decls={} errors={}",
            self.tokens_processed, self.decls_parsed, self.errors
        )
    }
}
/// A parse warning — something suspicious that isn't a hard error.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct ParseWarning {
    /// Human-readable warning message.
    pub message: String,
    /// Start byte of the suspicious token.
    pub start: usize,
    /// End byte of the suspicious token.
    #[allow(missing_docs)]
    pub end: usize,
}
impl ParseWarning {
    /// Create a new parse warning.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(message: impl Into<String>, start: usize, end: usize) -> Self {
        ParseWarning {
            message: message.into(),
            start,
            end,
        }
    }
}
/// A span annotation: associates a value with its source location.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct Annotated<T> {
    /// The parsed value.
    pub value: T,
    /// Start byte offset in the source.
    pub start: usize,
    /// End byte offset (exclusive) in the source.
    #[allow(missing_docs)]
    pub end: usize,
}
impl<T> Annotated<T> {
    /// Wrap a value with its source span.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(value: T, start: usize, end: usize) -> Self {
        Annotated { value, start, end }
    }
    /// Map the inner value, preserving span information.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Annotated<U> {
        Annotated {
            value: f(self.value),
            start: self.start,
            end: self.end,
        }
    }
    /// Returns the length of this annotated span in bytes.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn span_len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}
/// A named phase in the compilation pipeline.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilePhaseExt {
    /// Lexing phase
    Lex,
    /// Parsing phase
    Parse,
    /// Elaboration phase
    Elaborate,
    /// Tactic evaluation phase
    Tactic,
    /// Code generation phase
    CodeGen,
}
/// A timing record for a pipeline phase.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct PhaseTimerExt {
    /// The phase
    pub phase: CompilePhaseExt,
    /// Duration in microseconds (mocked)
    pub duration_us: u64,
}
/// A collection of parse diagnostics (errors and warnings).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
#[allow(missing_docs)]
pub struct ParseDiagnostics {
    /// All warnings collected during parsing.
    pub warnings: Vec<ParseWarning>,
    /// All error messages collected during parsing.
    pub errors: Vec<String>,
}
impl ParseDiagnostics {
    /// Create an empty diagnostics collection.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a warning.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn warn(&mut self, msg: impl Into<String>, start: usize, end: usize) {
        self.warnings.push(ParseWarning::new(msg, start, end));
    }
    /// Add an error.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn error(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
    }
    /// Returns `true` if there are no errors (warnings are OK).
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
    /// Total number of diagnostics (warnings + errors).
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total(&self) -> usize {
        self.warnings.len() + self.errors.len()
    }
}
/// A simple name resolution table.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NameResolutionTableExt {
    /// Scoped name mappings
    scopes: Vec<std::collections::HashMap<String, String>>,
}
impl NameResolutionTableExt {
    /// Create a new table with one empty scope.
    #[allow(dead_code)]
    pub fn new() -> Self {
        NameResolutionTableExt {
            scopes: vec![std::collections::HashMap::new()],
        }
    }
    /// Push a new scope.
    #[allow(dead_code)]
    pub fn push_scope(&mut self) {
        self.scopes.push(std::collections::HashMap::new());
    }
    /// Pop the current scope.
    #[allow(dead_code)]
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    /// Define a name in the current scope.
    #[allow(dead_code)]
    pub fn define(&mut self, name: &str, resolved: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), resolved.to_string());
        }
    }
    /// Resolve a name (innermost scope first).
    #[allow(dead_code)]
    pub fn resolve(&self, name: &str) -> Option<&str> {
        for scope in self.scopes.iter().rev() {
            if let Some(v) = scope.get(name) {
                return Some(v.as_str());
            }
        }
        None
    }
}
/// A summary of a source file.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct SourceSummaryExt2 {
    /// Number of lines
    pub line_count: usize,
    /// Number of characters
    pub char_count: usize,
    /// Number of words (whitespace-separated)
    pub word_count: usize,
    /// File path or name
    pub name: String,
}
impl SourceSummaryExt2 {
    /// Build a summary from source text.
    #[allow(dead_code)]
    pub fn from_str(name: &str, src: &str) -> Self {
        SourceSummaryExt2 {
            name: name.to_string(),
            line_count: src.lines().count(),
            char_count: src.chars().count(),
            word_count: src.split_whitespace().count(),
        }
    }
    /// Format as a one-line string.
    #[allow(dead_code)]
    pub fn summary_line(&self) -> String {
        format!(
            "{}: {} lines, {} chars, {} words",
            self.name, self.line_count, self.char_count, self.word_count
        )
    }
}
/// A declaration table mapping names to types.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Default)]
pub struct DeclTableExt2 {
    /// Map from name to type string
    pub entries: std::collections::HashMap<String, String>,
}
impl DeclTableExt2 {
    /// Create an empty table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        DeclTableExt2 {
            entries: std::collections::HashMap::new(),
        }
    }
    /// Insert a declaration.
    #[allow(dead_code)]
    pub fn insert(&mut self, name: &str, ty: &str) {
        self.entries.insert(name.to_string(), ty.to_string());
    }
    /// Look up a declaration.
    #[allow(dead_code)]
    pub fn lookup(&self, name: &str) -> Option<&str> {
        self.entries.get(name).map(|s| s.as_str())
    }
    /// Returns the number of declarations.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A parse flag set for controlling parser behaviour.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, Default)]
pub struct ParseFlagsExt {
    /// Whether to allow sorry in proofs
    pub allow_sorry: bool,
    /// Whether to enable unicode operators
    pub unicode_ops: bool,
    /// Whether to recover from errors
    pub error_recovery: bool,
    /// Whether to emit warnings as errors
    pub warnings_as_errors: bool,
}
impl ParseFlagsExt {
    /// Create a default set of flags.
    #[allow(dead_code)]
    pub fn default_flags() -> Self {
        ParseFlagsExt {
            allow_sorry: true,
            unicode_ops: true,
            error_recovery: true,
            warnings_as_errors: false,
        }
    }
    /// Enable sorry.
    #[allow(dead_code)]
    pub fn with_sorry(mut self) -> Self {
        self.allow_sorry = true;
        self
    }
    /// Disable error recovery.
    #[allow(dead_code)]
    pub fn strict(mut self) -> Self {
        self.error_recovery = false;
        self
    }
}
/// A declaration table mapping names to their source locations.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct DeclTable {
    /// Map from name to (filename, line)
    pub entries: std::collections::HashMap<String, (String, usize)>,
}
impl DeclTable {
    /// Create a new empty declaration table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        DeclTable {
            entries: std::collections::HashMap::new(),
        }
    }
    /// Register a declaration.
    #[allow(dead_code)]
    pub fn register(&mut self, name: &str, file: &str, line: usize) {
        self.entries
            .insert(name.to_string(), (file.to_string(), line));
    }
    /// Look up a declaration's location.
    #[allow(dead_code)]
    pub fn lookup(&self, name: &str) -> Option<(&str, usize)> {
        self.entries.get(name).map(|(f, l)| (f.as_str(), *l))
    }
    /// Returns the number of declarations.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// A compilation unit consisting of source file, AST, and errors.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct CompilationUnit {
    /// The source filename
    pub filename: String,
    /// The source text
    pub source: String,
    /// Whether parsing succeeded
    pub parse_ok: bool,
    /// Number of errors
    pub error_count: usize,
}
impl CompilationUnit {
    /// Create a new compilation unit.
    #[allow(dead_code)]
    pub fn new(filename: &str, source: &str) -> Self {
        CompilationUnit {
            filename: filename.to_string(),
            source: source.to_string(),
            parse_ok: false,
            error_count: 0,
        }
    }
    /// Mark as successfully parsed.
    #[allow(dead_code)]
    pub fn mark_parsed(mut self) -> Self {
        self.parse_ok = true;
        self
    }
    /// Mark with errors.
    #[allow(dead_code)]
    pub fn with_errors(mut self, count: usize) -> Self {
        self.error_count = count;
        self
    }
}
/// A namespace resolver.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NamespaceResolver {
    /// Stack of open namespaces
    pub stack: Vec<String>,
}
impl NamespaceResolver {
    /// Create a new resolver.
    #[allow(dead_code)]
    pub fn new() -> Self {
        NamespaceResolver { stack: Vec::new() }
    }
    /// Open a namespace.
    #[allow(dead_code)]
    pub fn open(&mut self, ns: &str) {
        self.stack.push(ns.to_string());
    }
    /// Close the current namespace.
    #[allow(dead_code)]
    pub fn close(&mut self) {
        self.stack.pop();
    }
    /// Resolve a name against the current namespace.
    #[allow(dead_code)]
    pub fn resolve(&self, name: &str) -> String {
        if self.stack.is_empty() {
            return name.to_string();
        }
        format!("{}.{}", self.stack.join("."), name)
    }
}
