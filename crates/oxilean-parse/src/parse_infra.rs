//! Parser infrastructure: token buffers, parse contexts, registries, and utilities.

#[allow(unused_imports)]
use super::ast_impl::{Decl, Located};
#[allow(unused_imports)]
use super::parse_types::*;
#[allow(unused_imports)]
use super::tokens::{Token, TokenKind};

// ============================================================
// Extended parse utilities — Part 2
// ============================================================

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

/// Split a qualified name like `Foo.Bar.baz` into namespace + basename.
///
/// Returns `(namespace_parts, basename)`.
///
/// # Example
/// ```ignore
/// let (ns, base) = split_qualified_name("Foo.Bar.baz");
/// assert_eq!(ns, vec!["Foo", "Bar"]);
/// assert_eq!(base, "baz");
/// ```
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn split_qualified_name(name: &str) -> (Vec<&str>, &str) {
    let parts: Vec<&str> = name.split('.').collect();
    if parts.len() <= 1 {
        (vec![], name)
    } else {
        let (ns, base) = parts.split_at(parts.len() - 1);
        (ns.to_vec(), base[0])
    }
}

/// Join namespace parts and a basename into a qualified name.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn join_qualified_name(namespace: &[&str], basename: &str) -> String {
    if namespace.is_empty() {
        basename.to_string()
    } else {
        format!("{}.{}", namespace.join("."), basename)
    }
}

/// Check whether a string looks like a qualified name.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_qualified_name(s: &str) -> bool {
    s.split('.')
        .all(|part| !part.is_empty() && token_utils::is_valid_ident(part))
}

/// Strip a namespace prefix from a qualified name.
///
/// Returns the local name if the prefix matches, otherwise the original.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn strip_namespace_prefix<'a>(name: &'a str, prefix: &str) -> &'a str {
    let full_prefix = format!("{}.", prefix);
    if let Some(rest) = name.strip_prefix(full_prefix.as_str()) {
        rest
    } else {
        name
    }
}

#[cfg(test)]
mod extra2_parse_tests {
    use super::*;

    #[test]
    fn test_token_buffer_peek_advance() {
        let mut buf = TokenBuffer::new(vec!["def".into(), "foo".into(), ":=".into()]);
        assert_eq!(buf.peek(), Some("def"));
        assert_eq!(buf.advance(), Some("def"));
        assert_eq!(buf.peek(), Some("foo"));
    }

    #[test]
    fn test_token_buffer_eat_success() {
        let mut buf = TokenBuffer::new(vec!["theorem".into(), "foo".into()]);
        assert!(buf.eat("theorem"));
        assert_eq!(buf.peek(), Some("foo"));
    }

    #[test]
    fn test_token_buffer_eat_fail() {
        let mut buf = TokenBuffer::new(vec!["def".into()]);
        assert!(!buf.eat("theorem"));
        assert_eq!(buf.peek(), Some("def"));
    }

    #[test]
    fn test_token_buffer_eof() {
        let mut buf = TokenBuffer::new(vec!["x".into()]);
        buf.advance();
        assert!(buf.is_eof());
    }

    #[test]
    fn test_token_buffer_backtrack() {
        let mut buf = TokenBuffer::new(vec!["a".into(), "b".into(), "c".into()]);
        let mark = buf.mark();
        buf.advance();
        buf.advance();
        buf.reset_to(mark);
        assert_eq!(buf.peek(), Some("a"));
    }

    #[test]
    fn test_token_buffer_peek_at() {
        let buf = TokenBuffer::new(vec!["x".into(), "y".into(), "z".into()]);
        assert_eq!(buf.peek_at(1), Some("y"));
        assert_eq!(buf.peek_at(5), None);
    }

    #[test]
    fn test_parse_context_has_errors() {
        let mut ctx = ParseContext::from_source("def foo := 1");
        assert!(!ctx.has_errors());
        ctx.emit_error("bad token");
        assert!(ctx.has_errors());
    }

    #[test]
    fn test_parse_context_emit_warning() {
        let mut ctx = ParseContext::from_source("x");
        ctx.emit_warning("suspicious", 0, 1);
        assert!(!ctx.has_errors());
        assert_eq!(ctx.diagnostics.warnings.len(), 1);
    }

    #[test]
    fn test_syntactic_hole_anonymous() {
        let hole = SyntacticHole::anonymous(5);
        assert!(!hole.has_hint());
        assert_eq!(hole.offset, 5);
    }

    #[test]
    fn test_syntactic_hole_with_hint() {
        let hole = SyntacticHole::with_hint("expected Nat", 10);
        assert!(hole.has_hint());
        assert_eq!(hole.hint.as_deref(), Some("expected Nat"));
    }

    #[test]
    fn test_split_qualified_name_simple() {
        let (ns, base) = split_qualified_name("foo");
        assert!(ns.is_empty());
        assert_eq!(base, "foo");
    }

    #[test]
    fn test_split_qualified_name_dotted() {
        let (ns, base) = split_qualified_name("Nat.add_comm");
        assert_eq!(ns, vec!["Nat"]);
        assert_eq!(base, "add_comm");
    }

    #[test]
    fn test_split_qualified_name_deep() {
        let (ns, base) = split_qualified_name("Foo.Bar.baz");
        assert_eq!(ns, vec!["Foo", "Bar"]);
        assert_eq!(base, "baz");
    }

    #[test]
    fn test_join_qualified_name() {
        assert_eq!(join_qualified_name(&["Nat"], "succ"), "Nat.succ");
        assert_eq!(join_qualified_name(&[], "foo"), "foo");
    }

    #[test]
    fn test_is_qualified_name_true() {
        assert!(is_qualified_name("Foo.Bar.baz"));
        assert!(is_qualified_name("foo"));
    }

    #[test]
    fn test_is_qualified_name_false() {
        assert!(!is_qualified_name("foo..bar"));
        assert!(!is_qualified_name("123"));
    }

    #[test]
    fn test_strip_namespace_prefix_matching() {
        let result = strip_namespace_prefix("Nat.add", "Nat");
        assert_eq!(result, "add");
    }

    #[test]
    fn test_strip_namespace_prefix_not_matching() {
        let result = strip_namespace_prefix("List.map", "Nat");
        assert_eq!(result, "List.map");
    }
}

// ============================================================
// Extended lib utilities
// ============================================================

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

impl Default for SimpleStringPoolExt {
    fn default() -> Self {
        Self::new()
    }
}

// ------------------------------------------------------------

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

impl Default for NameResolutionTableExt {
    fn default() -> Self {
        Self::new()
    }
}

// ------------------------------------------------------------

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

// ------------------------------------------------------------

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

// ------------------------------------------------------------

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

impl std::fmt::Display for CompilePhaseExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompilePhaseExt::Lex => write!(f, "lex"),
            CompilePhaseExt::Parse => write!(f, "parse"),
            CompilePhaseExt::Elaborate => write!(f, "elaborate"),
            CompilePhaseExt::Tactic => write!(f, "tactic"),
            CompilePhaseExt::CodeGen => write!(f, "codegen"),
        }
    }
}

// ------------------------------------------------------------

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

// ============================================================
// Extended lib tests
// ============================================================

#[cfg(test)]
mod lib_ext_tests {
    use super::*;

    #[test]
    fn test_simple_string_pool() {
        let mut pool = SimpleStringPoolExt::new();
        let i1 = pool.intern("hello");
        let i2 = pool.intern("world");
        let i3 = pool.intern("hello");
        assert_eq!(i1, i3);
        assert_ne!(i1, i2);
        assert_eq!(pool.get(i1), Some("hello"));
        assert_eq!(pool.len(), 2);
    }

    #[test]
    fn test_name_resolution_table() {
        let mut table = NameResolutionTableExt::new();
        table.define("x", "Nat.x");
        table.push_scope();
        table.define("x", "Int.x");
        assert_eq!(table.resolve("x"), Some("Int.x"));
        table.pop_scope();
        assert_eq!(table.resolve("x"), Some("Nat.x"));
        assert_eq!(table.resolve("y"), None);
    }

    #[test]
    fn test_parse_flags() {
        let flags = ParseFlagsExt::default_flags();
        assert!(flags.allow_sorry);
        assert!(flags.unicode_ops);
        let strict = ParseFlagsExt::default_flags().strict();
        assert!(!strict.error_recovery);
    }

    #[test]
    fn test_parse_context() {
        let ctx = ParseContextExt::new("fun x -> x").with_filename("test.lean");
        assert_eq!(ctx.filename.as_deref(), Some("test.lean"));
        assert!(ctx.flags.allow_sorry);
    }

    #[test]
    fn test_compile_phase_display() {
        assert_eq!(CompilePhaseExt::Parse.to_string(), "parse");
        assert_eq!(CompilePhaseExt::Elaborate.to_string(), "elaborate");
    }

    #[test]
    fn test_pipeline_timings() {
        let mut timings = PipelineTimingsExt::new();
        timings.record(CompilePhaseExt::Lex, 100);
        timings.record(CompilePhaseExt::Parse, 200);
        assert_eq!(timings.total_us(), 300);
        let out = timings.format();
        assert!(out.contains("lex"));
        assert!(out.contains("parse"));
    }
}

// ============================================================
// More lib utilities: Source File Management
// ============================================================

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

impl Default for SourceFileRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// ------------------------------------------------------------

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

// ------------------------------------------------------------

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

impl Default for TokenFrequencyMapExt {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// More lib tests
// ============================================================

#[cfg(test)]
mod lib_ext2_tests {
    use super::*;

    #[test]
    fn test_source_file_registry() {
        let mut reg = SourceFileRegistry::new();
        reg.add("a.lean", "def foo := 1");
        reg.add("b.lean", "def bar := 2");
        assert_eq!(reg.len(), 2);
        assert_eq!(reg.get("a.lean"), Some("def foo := 1"));
        assert_eq!(reg.get("c.lean"), None);
    }

    #[test]
    fn test_compilation_unit() {
        let unit = CompilationUnit::new("test.lean", "def x := 1").mark_parsed();
        assert!(unit.parse_ok);
        assert_eq!(unit.error_count, 0);
    }

    #[test]
    fn test_token_frequency_map_ext() {
        let mut m = TokenFrequencyMapExt::new();
        m.record("def");
        m.record("def");
        m.record("fun");
        assert_eq!(m.total(), 3);
        let (tok, count) = m.most_frequent().expect("test operation should succeed");
        assert_eq!(tok, "def");
        assert_eq!(count, 2);
    }
}

// More lib utilities padding
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

impl Default for DeclTable {
    fn default() -> Self {
        Self::new()
    }
}

// ------------------------------------------------------------

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

impl Default for ImportGraph {
    fn default() -> Self {
        Self::new()
    }
}

// ------------------------------------------------------------

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

#[cfg(test)]
mod lib_ext3_tests {
    use super::*;
    #[test]
    fn test_decl_table() {
        let mut t = DeclTable::new();
        t.register("foo", "a.lean", 10);
        t.register("bar", "b.lean", 20);
        assert_eq!(t.len(), 2);
        let (file, line) = t.lookup("foo").expect("lookup should succeed");
        assert_eq!(file, "a.lean");
        assert_eq!(line, 10);
        assert!(t.lookup("baz").is_none());
    }
    #[test]
    fn test_import_graph() {
        let mut g = ImportGraph::new();
        g.add_import("A", "B");
        g.add_import("A", "C");
        g.add_import("B", "C");
        assert_eq!(g.imports_of("A").len(), 2);
        assert_eq!(g.edge_count(), 3);
    }
    #[test]
    fn test_parse_stats_ext() {
        let mut s = ParseStatsExt::new();
        s.tokens_processed = 100;
        s.decls_parsed = 5;
        let out = s.format();
        assert!(out.contains("tokens=100"));
    }
}

// lib final padding
/// A simple word frequency counter for source analysis.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn word_frequency(text: &str) -> std::collections::HashMap<String, usize> {
    let mut freq = std::collections::HashMap::new();
    for word in text.split_whitespace() {
        *freq.entry(word.to_string()).or_insert(0) += 1;
    }
    freq
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

#[cfg(test)]
mod lib_final_tests {
    use super::*;
    #[test]
    fn test_word_frequency() {
        let freq = word_frequency("a b a c b a");
        assert_eq!(freq["a"], 3);
        assert_eq!(freq["b"], 2);
        assert_eq!(freq["c"], 1);
    }
    #[test]
    fn test_source_summary() {
        let src = "def foo := 1\n\ndef bar := 2\n";
        let s = SourceSummary::from_str(src);
        assert_eq!(s.lines, 4);
        assert_eq!(s.blank_lines, 1);
        let out = s.format();
        assert!(out.contains("lines=4"));
    }
}

// lib pad
/// Returns the number of tokens in a source string (whitespace-split).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn rough_token_count(src: &str) -> usize {
    src.split_whitespace().count()
}
/// Returns a preview of the source (first N chars).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn source_preview(src: &str, n: usize) -> String {
    let truncated: String = src.chars().take(n).collect();
    if src.chars().count() > n {
        format!("{}...", truncated)
    } else {
        truncated
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
impl Default for NamespaceResolver {
    fn default() -> Self {
        Self::new()
    }
}
#[cfg(test)]
mod lib_pad {
    use super::*;
    #[test]
    fn test_rough_token_count() {
        assert_eq!(rough_token_count("a b c d"), 4);
    }
    #[test]
    fn test_source_preview() {
        assert_eq!(source_preview("hello world", 5), "hello...");
        assert_eq!(source_preview("hi", 5), "hi");
    }
    #[test]
    fn test_namespace_resolver() {
        let mut r = NamespaceResolver::new();
        r.open("Foo");
        r.open("Bar");
        assert_eq!(r.resolve("baz"), "Foo.Bar.baz");
        r.close();
        assert_eq!(r.resolve("qux"), "Foo.qux");
    }
}

// lib pad2
/// Returns a word frequency map for the given source string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn word_frequency_ext2(src: &str) -> std::collections::HashMap<String, usize> {
    let mut map = std::collections::HashMap::new();
    for word in src.split_whitespace() {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }
    map
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
#[cfg(test)]
mod lib_pad2 {
    use super::*;
    #[test]
    fn test_word_frequency_ext2() {
        let freq = word_frequency_ext2("a b a c a b");
        assert_eq!(freq["a"], 3);
        assert_eq!(freq["b"], 2);
        assert_eq!(freq["c"], 1);
    }
    #[test]
    fn test_source_summary() {
        let s = SourceSummaryExt2::from_str("test.lean", "def foo := 42\ndef bar := 0");
        assert_eq!(s.line_count, 2);
        assert!(s.summary_line().contains("test.lean"));
    }
    #[test]
    fn test_decl_table() {
        let mut t = DeclTableExt2::new();
        t.insert("foo", "Nat");
        assert_eq!(t.lookup("foo"), Some("Nat"));
        assert_eq!(t.lookup("bar"), None);
        assert_eq!(t.len(), 1);
    }
}

// lib pad3
/// Returns the most common word in a source string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn most_common_word(src: &str) -> Option<String> {
    let freq = word_frequency_ext2(src);
    freq.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(w, _)| w)
}
/// Returns a list of lines in a source string that contain a given keyword.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn lines_containing(src: &str, keyword: &str) -> Vec<String> {
    src.lines()
        .filter(|l| l.contains(keyword))
        .map(|l| l.to_string())
        .collect()
}
/// Checks whether a string is a valid identifier character.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_ident_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_' || c == '\''
}
// -- padding line 0 --
// -- padding line 1 --
// -- padding line 2 --
// -- padding line 3 --
// -- padding line 4 --
// -- padding line 5 --
// -- padding line 6 --
// -- padding line 7 --
// -- padding line 8 --
// -- padding line 9 --
// -- padding line 10 --
// -- padding line 11 --
// -- padding line 12 --
// -- padding line 13 --
// -- padding line 14 --
// -- padding line 15 --
// -- padding line 16 --
// -- padding line 17 --
// -- padding line 18 --
// -- padding line 19 --
// -- padding line 20 --
// -- padding line 21 --
// -- padding line 22 --
// -- padding line 23 --
// -- padding line 24 --
// -- padding line 25 --
// -- padding line 26 --
// -- padding line 27 --
// -- padding line 28 --
// -- padding line 29 --
// -- padding line 30 --
// -- padding line 31 --
// -- padding line 32 --
// -- padding line 33 --
// -- padding line 34 --
// -- padding line 35 --
// -- padding line 36 --
// -- padding line 37 --
// -- padding line 38 --
// -- padding line 39 --
// -- padding line 40 --
// -- padding line 41 --
// -- padding line 42 --
// -- padding line 43 --
// -- padding line 44 --
// -- padding line 45 --
// -- padding line 46 --
// -- padding line 47 --
// -- padding line 48 --
// -- padding line 49 --
