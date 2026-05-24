//! leo4-lean4-parse — PEG-based Lean 4 parser. See `README.md`
//! for the strict-superset rationale + roadmap.
//!
//! ## Public API surface (v0.2 — expression grammar)
//!
//! - [`parse_decls`] — parse a top-level Lean 4 source string
//!   into a vector of [`Decl`]s.
//! - [`Decl`] / [`Expr`] / [`Literal`] / [`BinderGroup`] —
//!   the AST types designed to mirror `oxilean-parse`'s
//!   shapes for downstream interop.
//!
//! ## v0.2 grammar coverage
//!
//! - `def NAME [binders]+ [: TYPE] := VALUE` form.
//! - Binders: explicit `(...)`, implicit `{...}`, named-
//!   instance `[name : T]`, anonymous-instance `[T]`.
//! - Expression grammar (used in TYPE + VALUE positions):
//!   - Literals: numeric (`42`), string (`"hello"`).
//!   - Identifiers: `add`, `Nat.succ`.
//!   - Function application (left-associative): `f x y`.
//!   - Binary operators with precedence (low → high):
//!     `->`/`→` (arrow, right-assoc), `||`, `&&`,
//!     `==`/`!=`/`<`/`<=`/`>`/`>=`, `+`/`-`, `*`/`/`/`%`,
//!     `^` (right-assoc).
//!   - Unary: `-x`, `!x`, `¬x`.
//!   - Parenthesised: `(expr)`.
//!
//! Out of scope until subsequent OX6 commits:
//! `if-then-else`, `match` arms, lambda / `fun`, `let`,
//! `do` notation, string interpolation `s!"…"`, attribute
//! lists, `structure`/`inductive` decls.

use leo4_abi::LeanError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decl {
    pub kind: DeclKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclKind {
    /// `def NAME [binders]+ [: TYPE] := VALUE`
    Definition {
        name: String,
        binders: Vec<BinderGroup>,
        ty: Option<Expr>,
        value: Expr,
    },
}

/// A group of binders sharing a common type annotation:
/// `(a b : Nat)` → `BinderGroup { kind: Explicit, names: ["a", "b"], ty: Nat }`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinderGroup {
    pub kind: BinderKind,
    pub names: Vec<String>,
    pub ty: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinderKind {
    /// `(...)` — passed by name at call site.
    Explicit,
    /// `{...}` — implicit, auto-bound by elab.
    Implicit,
    /// `[...]` — typeclass instance.
    Instance,
}

/// Expression AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Identifier reference: `Nat`, `add`, `foo.bar`.
    Ident(String),
    /// Literal value: numeric or string.
    Lit(Literal),
    /// Function application (left-associative).
    /// `f x y` parses as `App(App(f, x), y)`.
    App(Box<Expr>, Box<Expr>),
    /// Binary operator: `BinOp(op, lhs, rhs)`. `op` is the
    /// surface symbol (`"+"`, `"=="`, `"->"`, etc.).
    BinOp(String, Box<Expr>, Box<Expr>),
    /// Unary prefix operator: `UnaryOp(op, operand)`.
    UnaryOp(String, Box<Expr>),
    /// Parenthesised — preserved in the AST so source spans
    /// round-trip; downstream consumers can strip if desired.
    Paren(Box<Expr>),
    /// Anything we couldn't further analyse — emitted as
    /// the raw source span. As the PEG grammar extends,
    /// fewer expressions land here.
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    /// Unsigned natural number literal (decimal).
    Nat(u64),
    /// String literal (raw bytes between `"…"`, with `\\` /
    /// `\"` escapes resolved).
    Str(String),
}

impl Expr {
    /// True if this is a fully-analysed `Ident` (vs the
    /// catch-all `Raw` form).
    #[must_use]
    pub fn is_ident(&self) -> bool {
        matches!(self, Self::Ident(_))
    }
}

/// Parse a Lean 4 source string into a sequence of
/// top-level declarations.
///
/// # Errors
/// `LeanError(DECODE_ERROR)` on parse failure with the
/// underlying PEG diagnostic in the message.
pub fn parse_decls(src: &str) -> Result<Vec<Decl>, LeanError> {
    grammar::lean4::source(src.trim()).map_err(|e| {
        LeanError::new(
            leo4_abi::error::error_codes::DECODE_ERROR,
            format!("leo4-lean4-parse: {e}"),
        )
    })
}

mod grammar {
    use super::{BinderGroup, BinderKind, Decl, DeclKind, Expr, Literal};

    peg::parser! {
        pub grammar lean4() for str {
            // ─── Whitespace + comment skipping ──────────────
            rule _ = (whitespace() / line_comment())* {}
            rule whitespace() = quiet!{[' ' | '\t' | '\n' | '\r']}
            rule line_comment() = quiet!{"--" (!"\n" [_])* "\n"?}

            // ─── Lexical atoms ───────────────────────────────
            rule ident_raw() -> String =
                quiet!{
                    !keyword()
                    head:$(['a'..='z' | 'A'..='Z' | '_']
                        ['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\'']*)
                    rest:("." s:$(['a'..='z' | 'A'..='Z' | '_']
                                  ['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\'']*)
                          { s.to_string() })*
                    {
                        let mut s = head.to_string();
                        for r in rest {
                            s.push('.');
                            s.push_str(&r);
                        }
                        s
                    }
                } / expected!("identifier")

            // Simple (no-dot) ident; used in places where
            // dots aren't allowed (e.g. binder names).
            rule ident() -> String =
                quiet!{
                    !keyword()
                    s:$(['a'..='z' | 'A'..='Z' | '_']
                        ['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\'']*)
                    { s.to_string() }
                } / expected!("identifier")

            rule keyword() = quiet!{
                ("def" / "theorem" / "lemma" / "axiom" / "inductive"
                 / "structure" / "class" / "instance" / "namespace"
                 / "section" / "end" / "open" / "import" / "variable"
                 / "where" / "with" / "match" / "fun" / "let"
                 / "if" / "then" / "else" / "do")
                ![ 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\'' ]
            }

            rule nat_lit() -> u64 =
                n:$(['0'..='9']+) {?
                    n.parse().map_err(|_| "nat literal out of range")
                }

            rule str_lit() -> String =
                "\"" s:str_body() "\"" { s }

            rule str_body() -> String =
                s:$((!"\"" str_char())*) { unescape(s) }

            rule str_char() = "\\" [_] / [_]

            // ─── Top-level entry ─────────────────────────────
            pub rule source() -> Vec<Decl> =
                _ ds:(d:decl() _ { d })* ![_] { ds }

            rule decl() -> Decl = d:definition() { d }

            rule definition() -> Decl =
                "def" _ name:ident() _ binders:(b:binder_group() _ { b })*
                ty:(":" _ t:expr() _ { t })?
                ":=" _ value:expr()
                {
                    Decl {
                        kind: DeclKind::Definition { name, binders, ty, value },
                    }
                }

            // ─── Binders ────────────────────────────────────
            rule binder_group() -> BinderGroup =
                explicit_binder() / implicit_binder() / instance_binder()

            rule explicit_binder() -> BinderGroup =
                "(" _ names:ident_list() _ ":" _ ty:expr() _ ")"
                { BinderGroup { kind: BinderKind::Explicit, names, ty } }

            rule implicit_binder() -> BinderGroup =
                "{" _ names:ident_list() _ ":" _ ty:expr() _ "}"
                { BinderGroup { kind: BinderKind::Implicit, names, ty } }

            // Two forms: named (`[inst : Ord T]`) and
            // anonymous (`[Ord T]` — anonymous typeclass arg).
            rule instance_binder() -> BinderGroup =
                named_instance() / anonymous_instance()

            rule named_instance() -> BinderGroup =
                "[" _ names:ident_list() _ ":" _ ty:expr() _ "]"
                { BinderGroup { kind: BinderKind::Instance, names, ty } }

            rule anonymous_instance() -> BinderGroup =
                "[" _ ty:expr() _ "]"
                { BinderGroup { kind: BinderKind::Instance, names: vec![], ty } }

            rule ident_list() -> Vec<String> =
                first:ident() rest:(_ n:ident() { n })* {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }

            // ─── Expression grammar with precedence ─────────
            //
            // Lean 4 / Mathlib operator precedence (rough):
            //
            //   25  ->   (arrow, right-assoc)        (lowest)
            //   30  ||
            //   35  &&
            //   50  ==  !=  <  >  <=  >=
            //   65  +   -
            //   70  *   /   %
            //   75  ^   (right-assoc)
            //   90  unary -  !  ¬
            //  max  application (left-assoc)
            //  --   atoms                            (highest)
            //
            // `peg`'s `precedence!` macro: `--` separates
            // levels (lower→higher); `x:(@)` = recurse at
            // same level (left-assoc); `x:@` = recurse at
            // higher level (right-assoc when on the LHS).
            pub rule expr() -> Expr = precedence!{
                // ─── arrow, right-assoc (Pi/fn type) ────
                lhs:@ _ ("->" / "→") _ rhs:(@) {
                    Expr::BinOp("->".into(), Box::new(lhs), Box::new(rhs))
                }
                --
                // ─── ||  (left-assoc) ───────────────────
                lhs:(@) _ "||" _ rhs:@ {
                    Expr::BinOp("||".into(), Box::new(lhs), Box::new(rhs))
                }
                --
                // ─── &&  (left-assoc) ───────────────────
                lhs:(@) _ "&&" _ rhs:@ {
                    Expr::BinOp("&&".into(), Box::new(lhs), Box::new(rhs))
                }
                --
                // ─── comparisons (left-assoc) ───────────
                lhs:(@) _ "==" _ rhs:@ { Expr::BinOp("==".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "!=" _ rhs:@ { Expr::BinOp("!=".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "<=" _ rhs:@ { Expr::BinOp("<=".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ ">=" _ rhs:@ { Expr::BinOp(">=".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "<"  _ rhs:@ { Expr::BinOp("<".into(),  Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ ">"  _ rhs:@ { Expr::BinOp(">".into(),  Box::new(lhs), Box::new(rhs)) }
                --
                // ─── additive (left-assoc) ──────────────
                lhs:(@) _ "+" _ rhs:@ { Expr::BinOp("+".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "-" _ rhs:@ { Expr::BinOp("-".into(), Box::new(lhs), Box::new(rhs)) }
                --
                // ─── multiplicative (left-assoc) ────────
                lhs:(@) _ "*" _ rhs:@ { Expr::BinOp("*".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "/" _ rhs:@ { Expr::BinOp("/".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "%" _ rhs:@ { Expr::BinOp("%".into(), Box::new(lhs), Box::new(rhs)) }
                --
                // ─── power (right-assoc) ────────────────
                lhs:@ _ "^" _ rhs:(@) {
                    Expr::BinOp("^".into(), Box::new(lhs), Box::new(rhs))
                }
                --
                // ─── unary prefix ───────────────────────
                "-" _ x:@ { Expr::UnaryOp("-".into(), Box::new(x)) }
                "!" _ x:@ { Expr::UnaryOp("!".into(), Box::new(x)) }
                "¬" _ x:@ { Expr::UnaryOp("¬".into(), Box::new(x)) }
                --
                // ─── application (left-assoc) ───────────
                f:(@) _ x:atom() {
                    Expr::App(Box::new(f), Box::new(x))
                }
                --
                // ─── atoms ──────────────────────────────
                a:atom() { a }
            }

            rule atom() -> Expr =
                paren_atom()
                / lit_atom()
                / ident_atom()

            rule paren_atom() -> Expr =
                "(" _ e:expr() _ ")" { Expr::Paren(Box::new(e)) }

            rule lit_atom() -> Expr =
                n:nat_lit() { Expr::Lit(Literal::Nat(n)) }
                / s:str_lit() { Expr::Lit(Literal::Str(s)) }

            rule ident_atom() -> Expr =
                s:ident_raw() { Expr::Ident(s) }
        }
    }

    /// Resolve `\n`, `\t`, `\\`, `\"` escape sequences inside
    /// a string literal body. Other escapes pass through
    /// verbatim (caller can iterate later if needed).
    fn unescape(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '\\' {
                match chars.next() {
                    Some('n') => out.push('\n'),
                    Some('t') => out.push('\t'),
                    Some('r') => out.push('\r'),
                    Some('\\') | None => out.push('\\'),
                    Some('"') => out.push('"'),
                    Some('0') => out.push('\0'),
                    Some(other) => {
                        out.push('\\');
                        out.push(other);
                    }
                }
            } else {
                out.push(c);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_source_yields_no_decls() {
        assert_eq!(parse_decls("").unwrap(), vec![]);
    }

    #[test]
    fn whitespace_only_source_yields_no_decls() {
        assert_eq!(parse_decls("   \n  \t  ").unwrap(), vec![]);
    }

    #[test]
    fn single_def_with_one_binder_group() {
        let src = "def identity (n : Nat) : Nat := n";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 1);
        match &decls[0].kind {
            DeclKind::Definition { name, binders, ty, value } => {
                assert_eq!(name, "identity");
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0].kind, BinderKind::Explicit);
                assert_eq!(binders[0].names, vec!["n".to_string()]);
                assert_eq!(binders[0].ty, Expr::Ident("Nat".into()));
                assert_eq!(ty.as_ref().unwrap(), &Expr::Ident("Nat".into()));
                assert_eq!(value, &Expr::Ident("n".into()));
            }
        }
    }

    #[test]
    fn def_with_multiple_binders_in_one_group() {
        let src = "def add (a b : UInt64) : UInt64 := a + b";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 1);
        let DeclKind::Definition { name, binders, value, .. } = &decls[0].kind;
        assert_eq!(name, "add");
        assert_eq!(binders.len(), 1);
        assert_eq!(binders[0].names, vec!["a".to_string(), "b".to_string()]);
        // `a + b` now parses as a real BinOp tree.
        assert_eq!(
            value,
            &Expr::BinOp(
                "+".into(),
                Box::new(Expr::Ident("a".into())),
                Box::new(Expr::Ident("b".into())),
            )
        );
    }

    #[test]
    fn def_with_implicit_and_instance_binders() {
        // No `if-then-else` yet — keep the body within v0.2.
        let src = "def maxScalar {T : Type} [Ord T] (a b : T) : T := a";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { binders, .. } = &decls[0].kind;
        assert_eq!(binders.len(), 3);
        assert_eq!(binders[0].kind, BinderKind::Implicit);
        assert_eq!(binders[1].kind, BinderKind::Instance);
        assert_eq!(binders[2].kind, BinderKind::Explicit);
    }

    #[test]
    fn def_with_no_type_annotation() {
        let src = "def hello := \"world\"";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { ty, value, .. } = &decls[0].kind;
        assert!(ty.is_none());
        assert_eq!(value, &Expr::Lit(Literal::Str("world".into())));
    }

    #[test]
    fn multi_def_source() {
        let src = "def f (n : Nat) : Nat := n\n\
                   def g (m : Nat) : Nat := m";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 2);
    }

    #[test]
    fn nested_bracket_in_type() {
        // `List (Option Nat)` parses as App(List, Paren(App(Option, Nat))).
        let src = "def f (xs : List (Option Nat)) : Nat := 0";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { binders, .. } = &decls[0].kind;
        // The ty is an App(List, Paren(App(Option, Nat))).
        match &binders[0].ty {
            Expr::App(f, x) => {
                assert_eq!(**f, Expr::Ident("List".into()));
                assert!(matches!(**x, Expr::Paren(_)));
            }
            other => panic!("expected App for `List (Option Nat)`, got {other:?}"),
        }
    }

    #[test]
    fn line_comments_skipped() {
        let src = "-- some doc\n\
                   def f (n : Nat) : Nat := n  -- inline\n";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 1);
    }

    #[test]
    fn parse_error_surfaces_as_lean_error() {
        // Missing `:=` — should fail.
        let src = "def bad (n : Nat) : Nat";
        let err = parse_decls(src).expect_err("must fail");
        assert_eq!(err.code, leo4_abi::error::error_codes::DECODE_ERROR);
    }

    // ─── Expression grammar tests (OX6 step 2) ────────────

    fn parse_value_expr(value_src: &str) -> Expr {
        let src = format!("def __probe : Nat := {value_src}");
        let mut decls = parse_decls(&src).expect("probe must parse");
        let DeclKind::Definition { value, .. } = decls.remove(0).kind;
        value
    }

    #[test]
    fn expr_nat_literal() {
        assert_eq!(parse_value_expr("42"), Expr::Lit(Literal::Nat(42)));
    }

    #[test]
    fn expr_string_literal_with_escape() {
        assert_eq!(
            parse_value_expr(r#""hello\nworld""#),
            Expr::Lit(Literal::Str("hello\nworld".into()))
        );
    }

    #[test]
    fn expr_dotted_ident() {
        // `foo.bar.baz` parses as one Ident with embedded dots.
        assert_eq!(
            parse_value_expr("Nat.succ"),
            Expr::Ident("Nat.succ".into())
        );
    }

    #[test]
    fn expr_app_is_left_associative() {
        // `f x y` = App(App(f, x), y)
        let e = parse_value_expr("f x y");
        match e {
            Expr::App(fx, y) => {
                assert_eq!(*y, Expr::Ident("y".into()));
                match *fx {
                    Expr::App(f, x) => {
                        assert_eq!(*f, Expr::Ident("f".into()));
                        assert_eq!(*x, Expr::Ident("x".into()));
                    }
                    other => panic!("expected nested App, got {other:?}"),
                }
            }
            other => panic!("expected App, got {other:?}"),
        }
    }

    #[test]
    fn expr_add_left_associative() {
        // a + b + c = (a + b) + c
        let e = parse_value_expr("a + b + c");
        match e {
            Expr::BinOp(op, lhs, rhs) => {
                assert_eq!(op, "+");
                assert_eq!(*rhs, Expr::Ident("c".into()));
                assert!(matches!(*lhs, Expr::BinOp(ref o, _, _) if o == "+"));
            }
            other => panic!("expected BinOp tree, got {other:?}"),
        }
    }

    #[test]
    fn expr_mul_binds_tighter_than_add() {
        // a + b * c = a + (b * c)
        let e = parse_value_expr("a + b * c");
        match e {
            Expr::BinOp(op, _, rhs) => {
                assert_eq!(op, "+");
                assert!(matches!(*rhs, Expr::BinOp(ref o, _, _) if o == "*"));
            }
            other => panic!("expected BinOp, got {other:?}"),
        }
    }

    #[test]
    fn expr_pow_right_associative() {
        // a ^ b ^ c = a ^ (b ^ c)
        let e = parse_value_expr("a ^ b ^ c");
        match e {
            Expr::BinOp(op, lhs, rhs) => {
                assert_eq!(op, "^");
                assert_eq!(*lhs, Expr::Ident("a".into()));
                assert!(matches!(*rhs, Expr::BinOp(ref o, _, _) if o == "^"));
            }
            other => panic!("expected right-assoc BinOp, got {other:?}"),
        }
    }

    #[test]
    fn expr_cmp_below_add() {
        // a + b == c = (a + b) == c
        let e = parse_value_expr("a + b == c");
        match e {
            Expr::BinOp(op, lhs, _) => {
                assert_eq!(op, "==");
                assert!(matches!(*lhs, Expr::BinOp(ref o, _, _) if o == "+"));
            }
            other => panic!("expected BinOp, got {other:?}"),
        }
    }

    #[test]
    fn expr_arrow_right_associative() {
        // T -> U -> V = T -> (U -> V)
        let e = parse_value_expr("T -> U -> V");
        match e {
            Expr::BinOp(op, lhs, rhs) => {
                assert_eq!(op, "->");
                assert_eq!(*lhs, Expr::Ident("T".into()));
                assert!(matches!(*rhs, Expr::BinOp(ref o, _, _) if o == "->"));
            }
            other => panic!("expected right-assoc arrow, got {other:?}"),
        }
    }

    #[test]
    fn expr_arrow_lowest_precedence() {
        // T1 + T2 -> R = (T1 + T2) -> R (additive binds
        // tighter than arrow)
        let e = parse_value_expr("T1 + T2 -> R");
        match e {
            Expr::BinOp(op, lhs, rhs) => {
                assert_eq!(op, "->");
                assert!(matches!(*lhs, Expr::BinOp(ref o, _, _) if o == "+"));
                assert_eq!(*rhs, Expr::Ident("R".into()));
            }
            other => panic!("expected arrow, got {other:?}"),
        }
    }

    #[test]
    fn expr_unicode_arrow_equivalent() {
        let e_ascii = parse_value_expr("T -> U");
        let e_unicode = parse_value_expr("T → U");
        assert_eq!(e_ascii, e_unicode);
    }

    #[test]
    fn expr_paren_preserved_in_ast() {
        let e = parse_value_expr("(a + b)");
        assert!(matches!(e, Expr::Paren(_)));
    }

    #[test]
    fn expr_unary_minus() {
        let e = parse_value_expr("- x");
        assert_eq!(e, Expr::UnaryOp("-".into(), Box::new(Expr::Ident("x".into()))));
    }

    #[test]
    fn expr_logical_and_or_precedence() {
        // a || b && c = a || (b && c) — && binds tighter
        let e = parse_value_expr("a || b && c");
        match e {
            Expr::BinOp(op, _, rhs) => {
                assert_eq!(op, "||");
                assert!(matches!(*rhs, Expr::BinOp(ref o, _, _) if o == "&&"));
            }
            other => panic!("expected ||, got {other:?}"),
        }
    }

    #[test]
    fn expr_app_in_def_body() {
        // `def f (a b : Nat) : Nat := Nat.succ a` exercises
        // app + dotted ident in a real fixture.
        let decls = parse_decls("def f (a b : Nat) : Nat := Nat.succ a").expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind;
        match value {
            Expr::App(f, x) => {
                assert_eq!(**f, Expr::Ident("Nat.succ".into()));
                assert_eq!(**x, Expr::Ident("a".into()));
            }
            other => panic!("expected App, got {other:?}"),
        }
    }
}
