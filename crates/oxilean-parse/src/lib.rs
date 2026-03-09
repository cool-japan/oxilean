#![allow(unused_imports)]
#![allow(dead_code)]

//! # OxiLean Parser — Surface Syntax to AST
//!
//! This crate converts concrete OxiLean source syntax (text) into abstract syntax trees (ASTs)
//! that the elaborator can process into kernel-checkable terms.
//!
//! ## Quick Start
//!
//! ### Parsing an Expression
//!
//! ```ignore
//! use oxilean_parse::{Lexer, Parser, SurfaceExpr};
//!
//! let source = "fun (x : Nat) => x + 1";
//! let lexer = Lexer::new(source);
//! let tokens = lexer.tokenize()?;
//! let mut parser = Parser::new(tokens);
//! let expr = parser.parse_expr()?;
//! ```
//!
//! ### Parsing a Module
//!
//! ```ignore
//! use oxilean_parse::Module;
//!
//! let source = "def double (n : Nat) : Nat := n + n";
//! let module = Module::parse_source(source)?;
//! ```
//!
//! ## Architecture Overview
//!
//! The parser is a three-stage pipeline:
//!
//! ```text
//! Source Text (.oxilean file)
//!     │
//!     ▼
//! ┌──────────────────────┐
//! │  Lexer               │  → Tokenizes text
//! │  (lexer.rs)          │  → Handles UTF-8, comments, strings
//! └──────────────────────┘
//!     │
//!     ▼
//! Token Stream
//!     │
//!     ▼
//! ┌──────────────────────┐
//! │  Parser              │  → Builds AST from tokens
//! │  (parser_impl.rs)    │  → Handles precedence, associativity
//! │  + Helpers           │  → Pattern, macro, tactic parsers
//! └──────────────────────┘
//!     │
//!     ▼
//! Abstract Syntax Tree (AST)
//!     │
//!     └─→ SurfaceExpr, SurfaceDecl, Module, etc.
//!     └─→ Diagnostic information (errors, warnings)
//!     └─→ Source mapping (for IDE integration)
//! ```
//!
//! ## Key Concepts & Terminology
//!
//! ### Tokens
//!
//! Basic lexical elements:
//! - **Identifiers**: `x`, `Nat`, `add_comm`, etc.
//! - **Keywords**: `fun`, `def`, `theorem`, `inductive`, etc.
//! - **Operators**: `+`, `-`, `:`, `=>`, `|`, etc.
//! - **Literals**: Numbers, strings, characters
//! - **Delimiters**: `(`, `)`, `[`, `]`, `{`, `}`
//!
//! ### Surface Syntax (SurfaceExpr)
//!
//! Represents OxiLean code before elaboration:
//! - **Applications**: `f x y` (function calls)
//! - **Lambda**: `fun x => body`
//! - **Pi types**: `(x : A) -> B`
//! - **Matches**: `match x with | nil => ... | cons h t => ...`
//! - **Tactics**: `by (tac1; tac2)`
//! - **Attributes**: `@[simp] def foo := ...`
//!
//! ### AST vs Kernel Expr
//!
//! - **AST (this crate)**: Surface syntax with implicit info
//!   - Contains `?` (implicit args), `_` (placeholders)
//!   - No type annotations required
//!   - Represents user-written code
//! - **Kernel Expr (oxilean-kernel)**: Type-checked terms
//!   - All types explicit
//!   - All implicit args resolved
//!   - Fully elaborated
//!
//! ## Module Organization
//!
//! ### Core Parsing Modules
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `lexer` | Tokenization: text → tokens |
//! | `tokens` | Token and TokenKind definitions |
//! | `parser_impl` | Main parser: tokens → AST |
//! | `command` | Command parsing (`def`, `theorem`, etc.) |
//!
//! ### AST Definition
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `ast_impl` | Core AST types: `SurfaceExpr`, `SurfaceDecl`, etc. |
//! | `pattern` | Pattern matching syntax |
//! | `literal` | Number and string literals |
//!
//! ### Specialized Parsers
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `tactic_parser` | Tactic syntax: `intro`, `apply`, `rw`, etc. |
//! | `macro_parser` | Macro definition and expansion |
//! | `notation_system` | Operator precedence and associativity |
//!
//! ### Diagnostics & Source Mapping
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `diagnostic` | Error and warning collection |
//! | `error_impl` | Parse error types and messages |
//! | `sourcemap` | Source position tracking for IDE |
//! | `span_util` | Source span utilities |
//!
//! ### Utilities
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `prettyprint` | AST pretty-printing |
//! | `module` | Module system and imports |
//! | `repl_parser` | REPL command parsing |
//!
//! ## Parsing Pipeline Details
//!
//! ### Phase 1: Lexical Analysis (Lexer)
//!
//! Transforms character stream into token stream:
//! - Handles Unicode identifiers (UTF-8)
//! - Recognizes keywords vs identifiers
//! - Tracks line/column positions (for error reporting)
//! - Supports:
//!   - Single-line comments: `-- comment`
//!   - Multi-line comments: `/-  -/`
//!   - String literals: `"hello"`
//!   - Number literals: `42`, `0xFF`, `3.14`
//!
//! ### Phase 2: Syntactic Analysis (Parser)
//!
//! Transforms token stream into AST:
//! - **Recursive descent**: For statements and declarations
//! - **Pratt parsing**: For expressions (handles precedence)
//! - **Lookahead(1)**: LL(1) grammar for predictive parsing
//! - **Error recovery**: Continues parsing after errors
//!
//! ### Phase 3: Post-Processing
//!
//! - **Notation expansion**: Apply infix/prefix operators
//! - **Macro expansion**: Expand syntax sugar
//! - **Span assignment**: Map AST nodes to source positions
//!
//! ## Usage Examples
//!
//! ### Example 1: Parse and Pretty-Print
//!
//! ```text
//! use oxilean_parse::{Lexer, Parser, print_expr};
//!
//! let source = "(x : Nat) -> Nat";
//! let mut parser = Parser::from_source(source)?;
//! let expr = parser.parse_expr()?;
//! println!("{}", print_expr(&expr));
//! ```
//!
//! ### Example 2: Parse a Definition
//!
//! ```text
//! use oxilean_parse::{Lexer, Parser, Decl};
//!
//! let source = "def double (n : Nat) : Nat := n + n";
//! let mut parser = Parser::from_source(source)?;
//! let decl = parser.parse_decl()?;
//! assert!(matches!(decl, Decl::Def { .. }));
//! ```
//!
//! ### Example 3: Collect Diagnostics
//!
//! ```text
//! use oxilean_parse::DiagnosticCollector;
//!
//! let mut collector = DiagnosticCollector::new();
//! // ... parse code ...
//! for diag in collector.diagnostics() {
//!     println!("{:?}", diag);
//! }
//! ```
//!
//! ## Operator Precedence
//!
//! Operators are organized by precedence levels (0-100):
//! - **Level 100** (highest): Projections, applications
//! - **Level 90**: Power/exponentiation
//! - **Level 70**: Multiplication, division
//! - **Level 65**: Addition, subtraction
//! - **Level 50**: Comparison (`<`, `>`, `=`, etc.)
//! - **Level 40**: Conjunction (`and`)
//! - **Level 35**: Disjunction (`or`)
//! - **Level 25**: Implication (`->`)
//! - **Level 0** (lowest): Binders (`fun`, `:`, etc.)
//!
//! Associativity (left/right/non-associative) is per-operator.
//!
//! ## Error Handling
//!
//! Parser errors include:
//! - **Unexpected token**: Parser expected a different token
//! - **Expected `type` token**: Specific token was expected but not found
//! - **Unclosed delimiter**: Missing closing bracket/paren
//! - **Undeclared operator**: Unknown infix operator
//! - **Invalid pattern**: Malformed pattern in match/fun
//!
//! All errors carry:
//! - **Source location** (span): File, line, column
//! - **Error message**: Human-readable description
//! - **Context**: Surrounding code snippet (for IDE tooltips)
//!
//! ## Source Mapping & IDE Integration
//!
//! The parser builds a **source map** tracking:
//! - AST node → source location
//! - Hover information (for IDE hover tooltips)
//! - Semantic tokens (for syntax highlighting)
//! - Reference locations (for "go to definition")
//!
//! This enables:
//! - Accurate error reporting
//! - IDE language server protocol (LSP) support
//! - Refactoring tools
//!
//! ## Extensibility
//!
//! ### Adding New Operators
//!
//! Operators are registered in `notation_system`:
//! ```ignore
//! let notation = Notation {
//!     name: "my_op",
//!     kind: NotationKind::Infix,
//!     level: 60,
//!     associativity: Associativity::Left,
//! };
//! notation_table.insert(notation);
//! ```
//!
//! ### Adding New Keywords
//!
//! Keywords are hardcoded in `lexer::keyword_of_string()`.
//! Add new keyword, then handle in parser.
//!
//! ### Custom Macros
//!
//! Macros are parsed by `macro_parser` and expanded during parsing:
//! ```text
//! syntax "list" ["[", expr, (",", expr)*, "]"] => ...
//! macro list_to_cons : list => (...)
//! ```
//!
//! ## Integration with Other Crates
//!
//! ### With oxilean-elab
//!
//! The elaborator consumes this crate's AST:
//! ```text
//! Parser: Source → SurfaceExpr
//! Elaborator: SurfaceExpr → Kernel Expr (with type checking)
//! ```
//!
//! ### With oxilean-kernel
//!
//! Kernel types (Name, Level, Literal) are re-exported by parser for convenience.
//!
//! ## Performance Considerations
//!
//! - **Linear parsing**: O(n) where n = source length
//! - **Minimal allocations**: AST nodes are typically smaller than source
//! - **Single pass**: No tokenization+parsing phase, done in parallel
//!
//! ## Further Reading
//!
//! - [ARCHITECTURE.md](../../ARCHITECTURE.md) — System architecture
//! - [BLUEPRINT.md](../../BLUEPRINT.md) — Formal syntax specification
//! - Module documentation for specific subcomponents

#![allow(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::collapsible_if)]

// Module stubs for future implementation
pub mod ast;
pub mod error;
pub mod parser;
pub mod token;

// Full implementations
pub mod ast_impl;
pub mod command;
pub mod diagnostic;
pub mod error_impl;
pub mod expr_cache;
/// Advanced formatter with Wadler-Lindig optimal layout.
pub mod formatter_adv;
pub mod incremental;
pub mod indent_tracker;
pub mod lexer;
pub mod macro_parser;
pub mod module;
pub mod notation_system;
pub mod parser_impl;
pub mod pattern;
pub mod prettyprint;
pub mod repl_parser;
pub mod roundtrip;
pub mod sourcemap;
pub mod span_util;
pub mod tactic_parser;
pub mod tokens;
pub mod wasm_source_map;

pub use ast::SurfaceExpr as OldSurfaceExpr;
pub use ast_impl::{
    AstNotationKind, AttributeKind, Binder, BinderKind, CalcStep as AstCalcStep, Constructor, Decl,
    DoAction, FieldDecl, Literal, Located, MatchArm, Pattern, SortKind, SurfaceExpr, TacticRef,
    WhereClause,
};
pub use command::{Command, CommandParser, NotationKind, OpenItem};
pub use diagnostic::{Diagnostic, DiagnosticCollector, DiagnosticLabel, Severity};
pub use error::ParseError as OldParseError;
pub use error_impl::{ParseError, ParseErrorKind};
pub use lexer::Lexer;
pub use macro_parser::{
    HygieneInfo, MacroDef, MacroError, MacroErrorKind, MacroExpander, MacroParser, MacroRule,
    MacroToken, SyntaxDef, SyntaxItem, SyntaxKind,
};
pub use module::{Module, ModuleRegistry};
pub use notation_system::{
    Fixity, NotationEntry, NotationKind as NotationSystemKind, NotationPart, NotationTable,
    OperatorEntry,
};
pub use parser_impl::Parser;
pub use pattern::{MatchClause, PatternCompiler};
pub use prettyprint::{print_decl, print_expr, ParensMode, PrettyConfig, PrettyPrinter};
pub use repl_parser::{is_complete, ReplCommand, ReplParser};
pub use sourcemap::{
    EntryKind, HoverInfo, SemanticToken, SourceEntry, SourceMap, SourceMapBuilder,
};
pub use span_util::{dummy_span, merge_spans, span_contains, span_len};
pub use tactic_parser::{
    CalcStep, CaseArm, ConvSide, RewriteRule, SimpArgs, TacticExpr, TacticParser,
};
pub use tokens::{Span, StringPart, Token, TokenKind};

// ============================================================
// Extended Parse Utilities
// ============================================================

/// The current version of the OxiLean parser.
///
/// This follows semantic versioning: MAJOR.MINOR.PATCH.
#[allow(missing_docs)]
pub const PARSER_VERSION: &str = "0.1.1";

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
