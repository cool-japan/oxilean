//! leo4-lean4-parse — PEG-based Lean 4 parser. See `README.md`
//! for the strict-superset rationale + roadmap.
//!
//! ## Public API surface (v0 scaffold)
//!
//! - [`parse_decls`] — parse a top-level Lean 4 source string
//!   into a vector of [`Decl`]s.
//! - [`Decl`] / [`Expr`] / [`Binder`] — the AST types
//!   designed to mirror `oxilean-parse`'s shapes for
//!   downstream interop.
//!
//! ## v0 grammar coverage
//!
//! Only `def NAME [binders]+ [: TYPE] := VALUE` form with
//! simple bracket-balanced type expressions and atomic /
//! bracket-balanced value expressions. Sufficient to land
//! the first sample-lean fixture; subsequent commits extend
//! the grammar.

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

/// Expression AST. v0 is intentionally narrow (atoms +
/// bracket-balanced raw text); future commits split this
/// into proper variants (App, Lam, Pi, BinOp, …).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Identifier reference: `Nat`, `add`, `foo.bar`.
    Ident(String),
    /// Anything we couldn't further analyse — emitted as
    /// the raw source span. Downstream consumers re-parse
    /// these as opaque text. As the PEG grammar extends,
    /// fewer expressions land here.
    Raw(String),
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
    use super::{BinderGroup, BinderKind, Decl, DeclKind, Expr};

    peg::parser! {
        pub grammar lean4() for str {
            // ─── Whitespace + comment skipping ──────────────
            rule _ = (whitespace() / line_comment())* {}
            rule whitespace() = quiet!{[' ' | '\t' | '\n' | '\r']}
            rule line_comment() = quiet!{"--" (!"\n" [_])* "\n"?}

            // ─── Lexical atoms ───────────────────────────────
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

            // ─── Top-level entry ─────────────────────────────
            pub rule source() -> Vec<Decl> =
                _ ds:(d:decl() _ { d })* ![_] { ds }

            rule decl() -> Decl = d:definition() { d }

            rule definition() -> Decl =
                "def" _ name:ident() _ binders:(b:binder_group() _ { b })*
                ty:(":" _ t:type_expr() _ { t })?
                ":=" _ value:value_expr()
                {
                    Decl {
                        kind: DeclKind::Definition { name, binders, ty, value },
                    }
                }

            // ─── Binders ────────────────────────────────────
            rule binder_group() -> BinderGroup =
                explicit_binder() / implicit_binder() / instance_binder()

            rule explicit_binder() -> BinderGroup =
                "(" _ names:ident_list() _ ":" _ ty:type_expr() _ ")"
                { BinderGroup { kind: BinderKind::Explicit, names, ty } }

            rule implicit_binder() -> BinderGroup =
                "{" _ names:ident_list() _ ":" _ ty:type_expr() _ "}"
                { BinderGroup { kind: BinderKind::Implicit, names, ty } }

            // Two forms: named (`[inst : Ord T]`) and
            // anonymous (`[Ord T]` — anonymous typeclass arg).
            rule instance_binder() -> BinderGroup =
                named_instance() / anonymous_instance()

            rule named_instance() -> BinderGroup =
                "[" _ names:ident_list() _ ":" _ ty:type_expr() _ "]"
                { BinderGroup { kind: BinderKind::Instance, names, ty } }

            rule anonymous_instance() -> BinderGroup =
                "[" _ ty:type_expr() _ "]"
                { BinderGroup { kind: BinderKind::Instance, names: vec![], ty } }

            rule ident_list() -> Vec<String> =
                first:ident() rest:(_ n:ident() { n })* {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }

            // ─── Type expressions ────────────────────────────
            //
            // v0: a type is either a bare identifier or a
            // bracket-balanced span (whose interior is treated
            // as opaque `Raw` text). Future commits will replace
            // this with a proper application / arrow / Pi
            // grammar — but the boundary char between a type
            // expr and the `:=` / `)` / closing bracket is
            // unambiguous, so this scaffold suffices.
            rule type_expr() -> Expr =
                raw:$(type_atom() (whitespace()+ type_atom())*)
                { Expr::Raw(raw.trim().to_string()) }

            rule type_atom() =
                ident() {} /
                "(" balanced_paren() ")" {} /
                "[" balanced_bracket() "]" {} /
                "{" balanced_brace() "}" {} /
                "->" {} / "→" {}

            // ─── Value expressions ──────────────────────────
            //
            // v0: catch-all raw text up to a newline followed
            // by a top-level keyword OR EOF.
            rule value_expr() -> Expr =
                raw:$(value_text())
                { Expr::Raw(raw.trim().to_string()) }

            rule value_text() =
                (!value_terminator() [_])+

            rule value_terminator() =
                // Newline followed by another top-level decl keyword.
                "\n" _ ("def" / "theorem" / "lemma" / "axiom" / "inductive"
                       / "structure" / "class" / "instance" / "namespace"
                       / "section" / "end" / "open" / "import" / "variable"
                       / "@[") {}

            // ─── Balanced bracket helpers ───────────────────
            rule balanced_paren() = (!")" balanced_char())*
            rule balanced_bracket() = (!"]" balanced_char())*
            rule balanced_brace() = (!"}" balanced_char())*

            rule balanced_char() =
                "(" balanced_paren() ")" {} /
                "[" balanced_bracket() "]" {} /
                "{" balanced_brace() "}" {} /
                [_]
        }
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
                assert_eq!(binders[0].ty, Expr::Raw("Nat".into()));
                assert_eq!(ty.as_ref().unwrap(), &Expr::Raw("Nat".into()));
                assert_eq!(value, &Expr::Raw("n".into()));
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
        assert_eq!(value, &Expr::Raw("a + b".into()));
    }

    #[test]
    fn def_with_implicit_and_instance_binders() {
        let src = "def maxScalar {T : Type} [Ord T] (a b : T) : T := \
                   if a > b then a else b";
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
        assert_eq!(value, &Expr::Raw("\"world\"".into()));
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
        let src = "def f (xs : List (Option Nat)) : Nat := xs.length";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { binders, .. } = &decls[0].kind;
        // The type lands as `Raw` text including nested brackets.
        match &binders[0].ty {
            Expr::Raw(s) => assert!(s.contains("List") && s.contains("Option")),
            Expr::Ident(s) => panic!("expected Raw type expr, got Ident({s})"),
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
}
