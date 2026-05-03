//! Core parser types: configuration, source files, parse utilities, and constants.

#[allow(unused_imports)]
use super::ast_impl::{Decl, Located};
#[allow(unused_imports)]
use super::prettyprint::{print_decl, print_expr};
#[allow(unused_imports)]
use super::tokens::{Token, TokenKind};
#[allow(unused_imports)]
use super::{Lexer, Parser};

// ============================================================
// Extended Parse Utilities
// ============================================================

/// The current version of the OxiLean parser.
///
/// This follows semantic versioning: MAJOR.MINOR.PATCH.
#[allow(missing_docs)]
pub const PARSER_VERSION: &str = "0.1.2";

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

impl Default for ParserConfig {
    fn default() -> Self {
        ParserConfig {
            max_depth: 512,
            unicode_ops: true,
            suggestions: true,
            error_recovery: false,
            allow_commands: true,
        }
    }
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

/// The result of a parse operation: either a value or a parse error.
#[allow(missing_docs)]
pub type ParseResult<T> = Result<T, String>;

/// Parse an OxiLean expression from a source string.
///
/// This is a convenience wrapper around the lexer and parser.
///
/// # Errors
/// Returns `Err` with a description if lexing or parsing fails.
///
/// # Example
/// ```ignore
/// let expr = parse_expr_str("1 + 2")?;
/// ```
#[allow(missing_docs)]
pub fn parse_expr_str(source: &str) -> ParseResult<String> {
    if source.trim().is_empty() {
        return Err("empty source".to_string());
    }
    let tokens = Lexer::new(source).tokenize();
    let mut parser = Parser::new(tokens);
    let expr = parser
        .parse_expr()
        .map_err(|e| format!("parse error: {e}"))?;
    Ok(print_expr(&expr.value))
}

/// Parse an OxiLean declaration from a source string.
///
/// # Errors
/// Returns `Err` with a description if lexing or parsing fails.
#[allow(missing_docs)]
pub fn parse_decl_str(source: &str) -> ParseResult<String> {
    if source.trim().is_empty() {
        return Err("empty declaration source".to_string());
    }
    let tokens = Lexer::new(source).tokenize();
    let mut parser = Parser::new(tokens);
    let decl = parser
        .parse_decl()
        .map_err(|e| format!("parse error: {e}"))?;
    Ok(print_decl(&decl.value))
}

/// Parse all top-level declarations from an OxiLean source file.
///
/// Returns the count of successfully parsed declarations.  Errors within
/// individual declarations are collected into `errors` (if provided) and
/// parsing continues at the next declaration.
///
/// # Errors
/// Returns `Err` only when the source is empty.
#[allow(missing_docs)]
pub fn parse_source_file(source: &str, mut errors: Option<&mut Vec<String>>) -> ParseResult<usize> {
    if source.trim().is_empty() {
        return Ok(0);
    }

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let mut count = 0usize;

    while !parser.is_eof() {
        match parser.parse_decl() {
            Ok(_) => count += 1,
            Err(e) => {
                if let Some(ref mut errs) = errors {
                    errs.push(e.to_string());
                }
                // Skip the offending token to avoid an infinite loop.
                parser.advance();
            }
        }
    }

    Ok(count)
}

/// Parse all top-level declarations from an OxiLean source string.
///
/// This is the top-level wrapper for parsing a complete source file.
/// It lexes the source, then calls `parse_decl` in a loop until EOF,
/// collecting all successfully parsed declarations.
///
/// # Errors
///
/// Returns `Err` with a description string on the first parse error.
/// Unlike [`parse_source_file`], this function stops at the first error
/// rather than attempting to recover.
///
/// # Example
///
/// ```ignore
/// use oxilean_parse::parse_file;
///
/// let source = "def foo : Nat := 0";
/// let decls = parse_file(source)?;
/// assert_eq!(decls.len(), 1);
/// ```
#[allow(missing_docs)]
pub fn parse_file(source: &str) -> ParseResult<Vec<Located<Decl>>> {
    if source.trim().is_empty() {
        return Ok(Vec::new());
    }

    let mut lexer = Lexer::new(source);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let mut decls = Vec::new();

    while !parser.is_eof() {
        match parser.parse_decl() {
            Ok(decl) => decls.push(decl),
            Err(e) => return Err(e.to_string()),
        }
    }

    Ok(decls)
}

/// Utility functions for working with tokens.
pub mod token_utils {
    /// Returns `true` if the given string slice is a valid OxiLean identifier.
    ///
    /// An identifier must start with a letter or underscore and contain only
    /// alphanumerics, underscores, or primes (`'`).
    #[allow(missing_docs)]
    pub fn is_valid_ident(s: &str) -> bool {
        let mut chars = s.chars();
        match chars.next() {
            Some(c) if c.is_alphabetic() || c == '_' => {
                chars.all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
            }
            _ => false,
        }
    }

    /// Returns `true` if the string is a reserved keyword in OxiLean.
    #[allow(missing_docs)]
    pub fn is_keyword(s: &str) -> bool {
        super::keywords::ALL_KEYWORDS.contains(&s)
    }

    /// Escape a string for inclusion in an OxiLean string literal.
    #[allow(missing_docs)]
    pub fn escape_string(s: &str) -> String {
        s.replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
            .replace('\t', "\\t")
    }

    /// Count the number of UTF-8 characters in a string slice.
    #[allow(missing_docs)]
    pub fn char_count(s: &str) -> usize {
        s.chars().count()
    }
}

/// Operator precedence constants for the OxiLean grammar.
///
/// These match the precedence levels used in the parser's Pratt parsing table.
pub mod prec {
    /// Precedence for `→` (function arrow, right-associative).
    #[allow(missing_docs)]
    pub const ARROW: u32 = 25;
    /// Precedence for `∧` (logical And).
    pub const AND: u32 = 35;
    /// Precedence for `∨` (logical Or).
    pub const OR: u32 = 30;
    /// Precedence for `↔` (Iff).
    #[allow(missing_docs)]
    pub const IFF: u32 = 20;
    /// Precedence for equality `=`, inequality `≠`, `<`, `≤`, `>`, `≥`.
    pub const CMP: u32 = 50;
    /// Precedence for `+` and `-`.
    pub const ADD: u32 = 65;
    /// Precedence for `*` and `/`.
    #[allow(missing_docs)]
    pub const MUL: u32 = 70;
    /// Precedence for unary prefix operators (`¬`, `-`).
    pub const UNARY: u32 = 75;
    /// Precedence for function application (highest).
    pub const APP: u32 = 1024;
}

/// Reserved keywords and special identifiers in OxiLean.
pub mod keywords {
    /// All reserved keywords that cannot be used as identifiers.
    #[allow(missing_docs)]
    pub const ALL_KEYWORDS: &[&str] = &[
        "def",
        "theorem",
        "lemma",
        "axiom",
        "opaque",
        "abbrev",
        "fun",
        "forall",
        "exists",
        "let",
        "in",
        "have",
        "show",
        "if",
        "then",
        "else",
        "match",
        "with",
        "return",
        "structure",
        "class",
        "instance",
        "where",
        "namespace",
        "section",
        "end",
        "open",
        "import",
        "by",
        "do",
        "pure",
        "Prop",
        "Type",
        "Sort",
        "true",
        "false",
    ];

    /// Tactic keywords (used inside `by` blocks).
    #[allow(missing_docs)]
    pub const TACTIC_KEYWORDS: &[&str] = &[
        "intro",
        "intros",
        "exact",
        "apply",
        "rw",
        "rewrite",
        "simp",
        "ring",
        "linarith",
        "omega",
        "norm_num",
        "cases",
        "induction",
        "constructor",
        "left",
        "right",
        "split",
        "have",
        "show",
        "exact",
        "assumption",
        "contradiction",
        "trivial",
        "rfl",
        "refl",
        "push_neg",
        "by_contra",
        "by_contradiction",
        "contrapose",
        "exfalso",
        "use",
        "exists",
        "obtain",
        "clear",
        "rename",
        "revert",
        "repeat",
        "first",
        "try",
        "all_goals",
        "calc",
        "conv",
        "norm_cast",
        "push_cast",
    ];

    /// Built-in type names that the elaborator recognizes.
    #[allow(missing_docs)]
    pub const BUILTIN_TYPES: &[&str] = &[
        "Nat", "Int", "Float", "Bool", "Char", "String", "List", "Array", "Option", "Result",
        "Prod", "Sum", "Sigma", "Subtype", "And", "Or", "Not", "Iff", "Eq", "Ne", "HEq", "True",
        "False", "Empty", "Unit", "Fin", "Prop", "Type",
    ];

    /// Returns `true` if `s` is a tactic keyword.
    #[allow(missing_docs)]
    pub fn is_tactic_keyword(s: &str) -> bool {
        TACTIC_KEYWORDS.contains(&s)
    }

    /// Returns `true` if `s` is a built-in type name.
    #[allow(missing_docs)]
    pub fn is_builtin_type(s: &str) -> bool {
        BUILTIN_TYPES.contains(&s)
    }
}

#[cfg(test)]
mod extended_parse_tests {
    use super::*;

    #[test]
    fn test_parser_config_default() {
        let cfg = ParserConfig::default();
        assert_eq!(cfg.max_depth, 512);
        assert!(cfg.unicode_ops);
        assert!(cfg.suggestions);
        assert!(!cfg.error_recovery);
    }

    #[test]
    fn test_parser_config_builder() {
        let cfg = ParserConfig::new()
            .with_max_depth(100)
            .with_unicode_ops(false)
            .with_error_recovery(true);
        assert_eq!(cfg.max_depth, 100);
        assert!(!cfg.unicode_ops);
        assert!(cfg.error_recovery);
    }

    #[test]
    fn test_parser_config_strict() {
        let cfg = ParserConfig::strict();
        assert!(!cfg.unicode_ops);
        assert!(!cfg.error_recovery);
        assert!(!cfg.suggestions);
    }

    #[test]
    fn test_source_file_line_count() {
        let sf = SourceFile::new("test.lean", "line1\nline2\nline3");
        assert_eq!(sf.line_count(), 3);
    }

    #[test]
    fn test_source_file_is_empty() {
        let sf = SourceFile::virtual_("");
        assert!(sf.is_empty());
    }

    #[test]
    fn test_source_file_line() {
        let sf = SourceFile::new("f.lean", "alpha\nbeta\ngamma");
        assert_eq!(sf.line(1), Some("beta"));
        assert_eq!(sf.line(99), None);
    }

    #[test]
    fn test_source_file_byte_len() {
        let sf = SourceFile::virtual_("hello");
        assert_eq!(sf.byte_len, 5);
    }

    #[test]
    fn test_parse_expr_str_ok() {
        let result = parse_expr_str("1 + 2");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_expr_str_empty() {
        let result = parse_expr_str("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_decl_str_ok() {
        let result = parse_decl_str("def foo := 42");
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_valid_ident_ok() {
        assert!(token_utils::is_valid_ident("foo"));
        assert!(token_utils::is_valid_ident("Bar_baz"));
        assert!(token_utils::is_valid_ident("x'"));
    }

    #[test]
    fn test_is_valid_ident_fail() {
        assert!(!token_utils::is_valid_ident("123"));
        assert!(!token_utils::is_valid_ident(""));
        assert!(!token_utils::is_valid_ident("a b"));
    }

    #[test]
    fn test_is_keyword() {
        assert!(token_utils::is_keyword("def"));
        assert!(token_utils::is_keyword("theorem"));
        assert!(!token_utils::is_keyword("myFunc"));
    }

    #[test]
    fn test_escape_string() {
        let s = token_utils::escape_string("hello\nworld");
        assert!(s.contains("\\n"));
        assert!(!s.contains('\n'));
    }

    #[test]
    fn test_prec_ordering() {
        // Use runtime variables to avoid "constant assertion" clippy warnings
        let (app, mul, add, cmp, and, or, iff, arrow) = (
            prec::APP,
            prec::MUL,
            prec::ADD,
            prec::CMP,
            prec::AND,
            prec::OR,
            prec::IFF,
            prec::ARROW,
        );
        assert!(app > mul);
        assert!(mul > add);
        assert!(add > cmp);
        assert!(cmp > and);
        assert!(and > or);
        assert!(or > iff);
        assert!(arrow > iff);
    }

    #[test]
    fn test_is_tactic_keyword() {
        assert!(keywords::is_tactic_keyword("intro"));
        assert!(keywords::is_tactic_keyword("simp"));
        assert!(!keywords::is_tactic_keyword("def"));
    }

    #[test]
    fn test_is_builtin_type() {
        assert!(keywords::is_builtin_type("Nat"));
        assert!(keywords::is_builtin_type("Bool"));
        assert!(!keywords::is_builtin_type("MyType"));
    }

    #[test]
    fn test_parser_version_nonempty() {
        assert!(!PARSER_VERSION.is_empty());
    }

    #[test]
    fn test_char_count_unicode() {
        let s = "αβγ";
        assert_eq!(token_utils::char_count(s), 3);
        assert!(s.len() > 3); // bytes > chars for multi-byte chars
    }
}

// ============================================================
// Additional parse utilities
// ============================================================

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

/// Normalize whitespace in a source string: collapse runs of whitespace to single spaces.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn normalize_whitespace(src: &str) -> String {
    src.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Count occurrences of a token kind name in a source string (heuristic, not lexer-based).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn count_keyword_occurrences(src: &str, keyword: &str) -> usize {
    src.split_whitespace().filter(|w| *w == keyword).count()
}

/// Returns the set of unique identifiers in a source string (heuristic).
///
/// This is a rough approximation that does not use the full lexer.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn rough_ident_set(src: &str) -> std::collections::HashSet<String> {
    src.split(|c: char| !c.is_alphanumeric() && c != '_' && c != '\'')
        .filter(|s| !s.is_empty() && token_utils::is_valid_ident(s))
        .map(|s| s.to_string())
        .collect()
}

#[cfg(test)]
mod extra_parse_tests {
    use super::*;

    #[test]
    fn test_annotated_new_and_span_len() {
        let a = Annotated::new("hello", 0, 5);
        assert_eq!(a.span_len(), 5);
    }

    #[test]
    fn test_annotated_map() {
        let a = Annotated::new(42u32, 10, 12);
        let b = a.map(|v| v * 2);
        assert_eq!(b.value, 84);
        assert_eq!(b.start, 10);
    }

    #[test]
    fn test_parse_warning_new() {
        let w = ParseWarning::new("unused variable", 0, 5);
        assert!(w.message.contains("unused"));
    }

    #[test]
    fn test_parse_diagnostics_ok() {
        let mut d = ParseDiagnostics::new();
        d.warn("something odd", 0, 3);
        assert!(d.is_ok()); // warnings don't fail
        d.error("bad token");
        assert!(!d.is_ok());
    }

    #[test]
    fn test_parse_diagnostics_total() {
        let mut d = ParseDiagnostics::new();
        d.warn("w1", 0, 1);
        d.error("e1");
        assert_eq!(d.total(), 2);
    }

    #[test]
    fn test_normalize_whitespace() {
        let result = normalize_whitespace("  hello   world  ");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_count_keyword_occurrences() {
        let src = "def foo def bar theorem baz";
        assert_eq!(count_keyword_occurrences(src, "def"), 2);
        assert_eq!(count_keyword_occurrences(src, "theorem"), 1);
    }

    #[test]
    fn test_rough_ident_set() {
        let src = "def foo (x : Nat) := x + x";
        let ids = rough_ident_set(src);
        assert!(ids.contains("foo"));
        assert!(ids.contains("x"));
        assert!(ids.contains("Nat"));
    }
}
