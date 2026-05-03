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

pub mod core_types;
pub use core_types::*;

pub mod error_recovery;
pub mod hygienic_macro;
pub mod module_system;
pub mod notation_elaboration;
pub mod source_map;
