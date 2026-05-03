//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::ast_impl::{
    AstNotationKind, AttributeKind, Binder, BinderKind, CalcStep as AstCalcStep, Constructor, Decl,
    DoAction, FieldDecl, Literal, Located, MatchArm, Pattern, SortKind, SurfaceExpr, TacticRef,
    WhereClause,
};
pub use crate::lexer::Lexer;
pub use crate::parser_impl::Parser;

use super::types::{
    Annotated, CompilationUnit, CompilePhaseExt, DeclTable, DeclTableExt2, ImportGraph,
    NameResolutionTableExt, NamespaceResolver, ParseContext, ParseContextExt, ParseDiagnostics,
    ParseFlagsExt, ParseStatsExt, ParseWarning, ParserConfig, PipelineTimingsExt,
    SimpleStringPoolExt, SourceFile, SourceFileRegistry, SourceSummary, SourceSummaryExt2,
    SyntacticHole, TokenBuffer, TokenFrequencyMapExt,
};
use crate::prettyprint::{print_decl, print_expr};

/// The current version of the OxiLean parser.
///
/// This follows semantic versioning: MAJOR.MINOR.PATCH.
#[allow(missing_docs)]
pub const PARSER_VERSION: &str = "0.1.1";
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
        assert!(s.len() > 3);
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
        assert!(d.is_ok());
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
