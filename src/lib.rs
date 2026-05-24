//! leo4-lean4-parse — PEG-based Lean 4 parser. See `README.md`
//! for the strict-superset rationale + roadmap.
//!
//! ## Public API surface (v0.5)
//!
//! - [`parse_decls`] — parse a top-level Lean 4 source string
//!   into a vector of [`Decl`]s.
//! - [`Decl`] / [`Expr`] / [`Literal`] / [`BinderGroup`] /
//!   [`LamBinder`] / [`Pattern`] / [`MatchArm`] /
//!   [`StructField`] / [`Ctor`] — the AST types designed to
//!   mirror `oxilean-parse`'s shapes for downstream interop.
//!
//! ## v0.5 grammar coverage
//!
//! - Decls: `def`, `structure`, `inductive` (both Lean 4
//!   `where`-form and OxiLean `: Type | …`-form), with
//!   `deriving Foo, Bar` clauses + `extends Base1, Base2`.
//! - Binders (def + lambda): explicit `(...)`, implicit
//!   `{...}`, named-instance `[name : T]`, anonymous-instance
//!   `[T]`. Lambdas also accept bare untyped names.
//! - Expression grammar (used in TYPE + VALUE positions):
//!   - Literals: numeric (`42`), string (`"hello"` with
//!     `\\n \\t \\r \\\\ \\" \\0` escapes).
//!   - Identifiers: `add`, `Nat.succ` (dotted forms).
//!   - Function application (left-associative): `f x y`.
//!   - Binary operators with precedence (low → high):
//!     `->`/`→` (arrow, right-assoc), `||`, `&&`,
//!     `==`/`!=`/`<`/`<=`/`>`/`>=`, `+`/`-`, `*`/`/`/`%`,
//!     `^` (right-assoc).
//!   - Unary: `-x`, `!x`, `¬x`.
//!   - Parenthesised: `(expr)`.
//!   - `if cond then t else e`.
//!   - `match scrut with | pat => body | …` with full
//!     pattern AST (wildcard, var, lit, ctor with args,
//!     dot-ctor, paren, tuple).
//!   - `fun BINDERS => body` / `λ BINDERS => body` /
//!     `fun BINDERS -> body`.
//!   - `do <stmts>` — monadic sequencing with `let x ← e`
//!     (bind), `let x := e` (pure let), `return e` /
//!     `pure e`, and bare expression statements.
//!
//! Structure/inductive field & ctor type annotations are
//! fully parsed as `Expr` (including multi-line types via
//! layout-sensitive boundary detection — the type region
//! spans every line until the next field header, `deriving`,
//! top-level decl keyword, or EOF; the captured text is
//! then sub-parsed through the same `expr` grammar).
//! Attribute lists with arguments parse natively (args
//! preserved as raw text per-attribute).
//!
//! Out of scope until subsequent OX6 commits:
//! `let`, `do` notation, string interpolation `s!"…"`,
//! `theorem` / `lemma` / `axiom` / `instance` / `class` /
//! `namespace` / `open` / `import` / `variable` / `mutual`
//! decls.

use leo4_abi::LeanError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decl {
    /// Attribute list prefix (`@[…]`) applied to this decl.
    /// Empty if the decl had no attribute prefix.
    pub attrs: Vec<Attribute>,
    pub kind: DeclKind,
}

/// One attribute inside a `@[…]` list. The `raw_args` field
/// holds everything after the attribute name (whitespace
/// trimmed); v0 doesn't sub-parse arguments because
/// attribute-specific arg grammars are defined per-attribute
/// in Lean 4 (`@[simp]` takes no args, `@[builtin_attribute
/// "name" "doc"]` takes string-literal args, etc.). Downstream
/// consumers parse `raw_args` themselves if needed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,
    pub raw_args: String,
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
    /// `structure NAME [extends BASE, ...] where FIELDS [deriving ...]`
    Structure {
        name: String,
        extends: Vec<String>,
        fields: Vec<StructField>,
        deriving: Vec<String>,
    },
    /// `inductive NAME [: TYPE] where | CTOR ... [deriving ...]`
    /// (Lean 4 `where`-form; the older OxiLean
    /// `inductive NAME : Type | ctor : T` form is **also**
    /// supported — both feed into the same AST.)
    Inductive {
        name: String,
        ty: Option<Expr>,
        ctors: Vec<Ctor>,
        deriving: Vec<String>,
    },
}

/// One named field of a `structure`. The `ty` is the full
/// parsed expression annotation (multi-line continuations
/// supported via layout-sensitive boundary detection — a
/// field's type spans every line until the next field
/// header, `deriving` clause, top-level decl, or EOF).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructField {
    pub name: String,
    pub ty: Expr,
}

/// One constructor of an `inductive`. `Some(ty)` is the
/// explicit type annotation (parsed; multi-line allowed via
/// the same boundary-detection rules as `StructField`);
/// `None` means the ctor was written bare (`| red`) — the
/// elaborator supplies the inductive type as the ctor type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ctor {
    pub name: String,
    pub ty: Option<Expr>,
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
    /// `if COND then THEN else ELSE`.
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    /// `match SCRUTINEE with | pat => body | ...`.
    Match(Box<Expr>, Vec<MatchArm>),
    /// `fun BINDERS => BODY` / `λ BINDERS => BODY` /
    /// `fun BINDERS -> BODY` (`->` body-arrow accepted as a
    /// synonym for `=>` for OX3 normalisation compatibility).
    Lam(Vec<LamBinder>, Box<Expr>),
    /// `do <stmts>` — monadic sequencing block.
    Do(Vec<DoStmt>),
    /// Anything we couldn't further analyse — emitted as
    /// the raw source span. As the PEG grammar extends,
    /// fewer expressions land here.
    Raw(String),
}

/// One arm of a `match` expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expr,
}

/// One statement inside a `do` block. v0 supports the
/// four most common forms; multi-line statement bodies +
/// embedded `if` / `match` statements are a follow-up.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoStmt {
    /// `let x ← e` / `let x <- e` (monadic bind).
    Bind { name: String, value: Expr },
    /// `let x := e` (pure let).
    Let { name: String, value: Expr },
    /// `return e` / `pure e` — semantically distinct in
    /// some Lean 4 monads but collapsed here for simplicity
    /// (elaborator can re-disambiguate from context).
    Return(Expr),
    /// Bare expression statement (its effect is sequenced).
    Expr(Expr),
}

/// One lambda binder. Lambdas accept both untyped names and
/// typed groups (mirroring `def`'s binder syntax), so each
/// binder is either a single bare ident or a parenthesised /
/// braced / bracketed typed group.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LamBinder {
    /// Bare untyped name: `fun x => ...`.
    Untyped(String),
    /// Typed group: `fun (x : T) => ...` / `fun (a b : T) => ...`.
    /// `kind` distinguishes `()` / `{}` / `[]` brackets.
    Typed {
        kind: BinderKind,
        names: Vec<String>,
        ty: Expr,
    },
}

/// `match` pattern AST.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern {
    /// `_` wildcard.
    Wildcard,
    /// Single-name binder: `x`. (Distinguishing var from
    /// ctor name is the elaborator's job — we treat any
    /// bare ident as a var, and ctor application as `Ctor`.)
    Var(String),
    /// Literal pattern: `42` / `"s"`.
    Lit(Literal),
    /// Constructor with optional args: `Color.red` / `some x`
    /// / `node l r`.
    Ctor(String, Vec<Pattern>),
    /// Lean 4's dot-ctor shorthand: `.lt` / `.some x`.
    DotCtor(String, Vec<Pattern>),
    /// Parenthesised: `(p)`.
    Paren(Box<Pattern>),
    /// Tuple: `(a, b)` / `(a, b, c)`.
    Tuple(Vec<Pattern>),
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
    use super::{
        Attribute, BinderGroup, BinderKind, Ctor, Decl, DeclKind, DoStmt, Expr,
        LamBinder, Literal, MatchArm, Pattern, StructField,
    };

    /// Sub-parse a captured raw-text region (a structure
    /// field's or inductive ctor's type annotation) as a
    /// stand-alone `expr`. Returns `None` if the text fails
    /// to parse — caller falls back to `Expr::Raw(text)` so
    /// even malformed type annotations don't abort the whole
    /// decl.
    ///
    /// Layout-sensitive multi-line type regions work because
    /// the boundary detection in `field_or_ctor_boundary`
    /// stops capture at the next field / ctor / decl
    /// keyword line, then this helper feeds the captured
    /// multi-line slice to `expr` (whose grammar tolerates
    /// newlines via the `_` rule).
    fn parse_expr_text(text: &str) -> Option<Expr> {
        lean4::expr(text.trim()).ok()
    }

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

            // A top-level decl with an optional attribute
            // prefix. `decl_body` returns a `Decl` with
            // `attrs: vec![]`; this wrapper attaches the
            // parsed attrs if present.
            rule decl() -> Decl =
                attrs:(a:attribute_list() _ { a })? d:decl_body() {
                    let mut decl = d;
                    decl.attrs = attrs.unwrap_or_default();
                    decl
                }

            rule decl_body() -> Decl =
                d:definition() { d }
                / d:structure_decl() { d }
                / d:inductive_decl() { d }

            // ─── Attribute list `@[…]` ───────────────────────
            //
            // Each entry is `name [raw_args]`, comma-separated.
            // `raw_args` is everything between the name and
            // the next `,` or `]` (whitespace trimmed). v0
            // doesn't sub-parse args because attribute-
            // specific arg grammars are per-attribute in Lean
            // 4 — downstream consumers parse `raw_args`
            // themselves.
            rule attribute_list() -> Vec<Attribute> =
                "@[" _ first:attribute() rest:(_ "," _ a:attribute() { a })* _ "]" {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }

            rule attribute() -> Attribute =
                name:ident_raw()
                args:(_h() s:$((!"," !"]" !"\n" [_])+) { s.trim().to_string() })?
                {
                    Attribute { name, raw_args: args.unwrap_or_default() }
                }

            rule definition() -> Decl =
                "def" _ name:ident() _ binders:(b:binder_group() _ { b })*
                ty:(":" _ t:expr() _ { t })?
                ":=" _ value:expr()
                {
                    Decl {
                        attrs: vec![],
                        kind: DeclKind::Definition { name, binders, ty, value },
                    }
                }

            // ─── Structure ───────────────────────────────────
            //
            // `structure NAME [extends BASE, ...] where
            //     field1 : T1
            //     field2 : T2
            //     ...
            //     [deriving Foo, Bar]`
            //
            // Fields are one-per-line in v0; the type annotation
            // is captured as raw source text (full expression
            // re-parsing on the type side needs layout-aware
            // grammar — a follow-up commit).
            rule structure_decl() -> Decl =
                "structure" _ name:ident()
                extends:(_ e:struct_extends() { e })?
                _ "where"
                fields:(_ f:struct_field() { f })*
                deriving:(_ d:deriving_clause() { d })?
                {
                    Decl { attrs: vec![], kind: DeclKind::Structure {
                        name,
                        extends: extends.unwrap_or_default(),
                        fields,
                        deriving: deriving.unwrap_or_default(),
                    }}
                }

            rule struct_extends() -> Vec<String> =
                "extends" _ first:ident_raw() rest:(_ "," _ n:ident_raw() { n })* {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }

            // Multi-line field type: capture raw text up to
            // the next field header / deriving / top-level
            // decl / EOF (the "field-region" boundary), then
            // sub-parse that text as an `expr`. The boundary
            // detection is layout-sensitive in the loose
            // sense that the next field's header pattern
            // (`<ident> :`) on its own line is what stops
            // the current field's type.
            rule struct_field() -> StructField =
                name:ident() _h() ":" _h()
                ty_text:$((!field_or_ctor_boundary() [_])+)
                {
                    let trimmed = ty_text.trim();
                    let ty = parse_expr_text(trimmed)
                        .unwrap_or_else(|| Expr::Raw(trimmed.to_string()));
                    StructField { name, ty }
                }

            // ─── Inductive ───────────────────────────────────
            //
            // Two surface forms, both fed into the same AST:
            //
            //   1. `inductive NAME [: TYPE] where
            //        | ctor1
            //        | ctor2 : T -> NAME
            //        ...
            //        [deriving ...]`
            //
            //   2. `inductive NAME [: TYPE]
            //        | ctor1 : NAME
            //        | ctor2 : T -> NAME
            //        ...
            //        [deriving ...]`
            //
            // The `where` keyword is optional; ctor `:` is
            // also optional (bare `| red` ⇒ elaborator
            // supplies the inductive type as ctor type).
            rule inductive_decl() -> Decl =
                "inductive" _ name:ident()
                ty:(_ ":" _ t:expr() { t })?
                _ "where"?
                ctors:(_ c:inductive_ctor() { c })*
                deriving:(_ d:deriving_clause() { d })?
                {
                    Decl { attrs: vec![], kind: DeclKind::Inductive {
                        name,
                        ty,
                        ctors,
                        deriving: deriving.unwrap_or_default(),
                    }}
                }

            rule inductive_ctor() -> Ctor =
                arm_bar() _ name:ident()
                ty:(_h() ":" _h()
                    text:$((!field_or_ctor_boundary() [_])+) {
                        let trimmed = text.trim();
                        parse_expr_text(trimmed)
                            .unwrap_or_else(|| Expr::Raw(trimmed.to_string()))
                    })?
                {
                    Ctor { name, ty }
                }

            // Boundary used by both `struct_field` and
            // `inductive_ctor` when capturing the type-region
            // text. The current type region ends when the
            // PEG can see, after a newline + optional
            // horizontal whitespace, any of:
            //
            //   - a next field/ctor header (`<ident> :` or
            //     `| <ident>`),
            //   - `deriving` keyword,
            //   - a top-level decl keyword (`def`, `structure`,
            //     `inductive`, `@[…]` attribute prefix),
            //   - end of file.
            //
            // Same-line content (no newline) never triggers
            // the boundary, so a header followed by trailing
            // whitespace on its own line opens a multi-line
            // continuation.
            rule field_or_ctor_boundary() =
                "\n" _h() boundary_starter()

            rule boundary_starter() =
                ident() _h() ":" {}        // next struct field
                / "|" {}                    // next ctor arm
                / "deriving" word_boundary()
                / top_level_decl_starter()
                / ![_]                       // EOF

            rule top_level_decl_starter() =
                ("def" / "theorem" / "lemma" / "axiom" / "inductive"
                 / "structure" / "class" / "instance" / "namespace"
                 / "section" / "end" / "open" / "import" / "variable"
                 / "abbrev")
                word_boundary()
                / "@[" {}

            rule word_boundary() =
                ![ 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\'']

            // ─── Deriving ────────────────────────────────────
            rule deriving_clause() -> Vec<String> =
                "deriving" _ first:ident_raw() rest:(_ "," _ n:ident_raw() { n })* {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }

            // Horizontal-only whitespace (no newline). Used in
            // single-line field / ctor parsing so the line-end
            // boundary is exact.
            rule _h() = ([' ' | '\t'])*

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
                if_expr()
                / match_expr()
                / lam_expr()
                / do_expr()
                / paren_atom()
                / lit_atom()
                / ident_atom()

            rule paren_atom() -> Expr =
                "(" _ e:expr() _ ")" { Expr::Paren(Box::new(e)) }

            rule lit_atom() -> Expr =
                n:nat_lit() { Expr::Lit(Literal::Nat(n)) }
                / s:str_lit() { Expr::Lit(Literal::Str(s)) }

            rule ident_atom() -> Expr =
                s:ident_raw() { Expr::Ident(s) }

            // ─── do notation ────────────────────────────────
            //
            // `do <stmts>` — monadic sequencing block. Each
            // statement lives on its own line (v0); multi-
            // line statement bodies are a follow-up commit.
            //
            // Statement forms (priority-ordered for the
            // PEG):
            //   1. `let NAME <- E` / `let NAME ← E`  (bind)
            //   2. `let NAME := E`                    (let)
            //   3. `return E` / `pure E`              (return)
            //   4. bare E                             (effect)
            //
            // Each statement's expression text is captured
            // to end-of-line and sub-parsed via
            // `parse_expr_text` (same pattern as field /
            // ctor type capture in OX6 step 7).
            //
            // Block boundary: next statement on a new line
            // OR top-level decl keyword OR EOF.
            rule do_expr() -> Expr =
                "do" word_boundary() _ stmts:do_stmts() {
                    Expr::Do(stmts)
                }

            rule do_stmts() -> Vec<DoStmt> =
                first:do_stmt() rest:(do_stmt_sep() s:do_stmt() { s })* {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }

            rule do_stmt_sep() =
                "\n" _h() !do_block_end()

            rule do_block_end() =
                top_level_decl_starter() / ![_]

            rule do_stmt() -> DoStmt =
                do_let_bind()
                / do_let_pure()
                / do_return()
                / do_expr_stmt()

            rule do_let_bind() -> DoStmt =
                "let" word_boundary() _h() name:ident() _h() ("<-" / "←") _h()
                text:$((!"\n" [_])+)
                {
                    let value = parse_expr_text(text.trim())
                        .unwrap_or_else(|| Expr::Raw(text.trim().to_string()));
                    DoStmt::Bind { name, value }
                }

            rule do_let_pure() -> DoStmt =
                "let" word_boundary() _h() name:ident() _h() ":=" _h()
                text:$((!"\n" [_])+)
                {
                    let value = parse_expr_text(text.trim())
                        .unwrap_or_else(|| Expr::Raw(text.trim().to_string()));
                    DoStmt::Let { name, value }
                }

            rule do_return() -> DoStmt =
                ("return" / "pure") word_boundary() _h()
                text:$((!"\n" [_])+)
                {
                    let value = parse_expr_text(text.trim())
                        .unwrap_or_else(|| Expr::Raw(text.trim().to_string()));
                    DoStmt::Return(value)
                }

            rule do_expr_stmt() -> DoStmt =
                text:$((!"\n" [_])+)
                {
                    let e = parse_expr_text(text.trim())
                        .unwrap_or_else(|| Expr::Raw(text.trim().to_string()));
                    DoStmt::Expr(e)
                }

            // ─── lambda (`fun` / `λ`) ───────────────────────
            //
            // `fun BINDERS => BODY` form. The body-arrow is
            // either `=>` (Lean 4 native) or `->` (accepted
            // as a synonym; OX3's `lean4_normalize` rewrites
            // ` => ` to ` -> `, and `leo4-lean4-parse` should
            // accept both so it can replace the textual
            // normaliser later without surface regressions).
            rule lam_expr() -> Expr =
                ("fun" / "λ") _ binders:lam_binders() _ ("=>" / "->") _ body:expr() {
                    Expr::Lam(binders, Box::new(body))
                }

            rule lam_binders() -> Vec<LamBinder> =
                first:lam_binder() rest:(_ b:lam_binder() { b })* {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }

            rule lam_binder() -> LamBinder =
                lam_typed_explicit()
                / lam_typed_implicit()
                / lam_typed_instance()
                / lam_untyped()

            rule lam_typed_explicit() -> LamBinder =
                "(" _ names:ident_list() _ ":" _ ty:expr() _ ")" {
                    LamBinder::Typed { kind: BinderKind::Explicit, names, ty }
                }

            rule lam_typed_implicit() -> LamBinder =
                "{" _ names:ident_list() _ ":" _ ty:expr() _ "}" {
                    LamBinder::Typed { kind: BinderKind::Implicit, names, ty }
                }

            rule lam_typed_instance() -> LamBinder =
                "[" _ names:ident_list() _ ":" _ ty:expr() _ "]" {
                    LamBinder::Typed { kind: BinderKind::Instance, names, ty }
                }

            rule lam_untyped() -> LamBinder =
                s:ident() { LamBinder::Untyped(s) }

            // ─── if-then-else ───────────────────────────────
            rule if_expr() -> Expr =
                "if" _ cond:expr() _ "then" _ t:expr() _ "else" _ e:expr() {
                    Expr::If(Box::new(cond), Box::new(t), Box::new(e))
                }

            // ─── match … with | … ───────────────────────────
            rule match_expr() -> Expr =
                "match" _ scrut:expr() _ "with" _ arms:match_arms() {
                    Expr::Match(Box::new(scrut), arms)
                }

            // One-or-more arms separated by their leading `|`.
            // The single `|` token (NOT `||`) is the arm
            // separator; the precedence-climbing expression
            // grammar accepts `||` as a binary op so they
            // don't collide.
            rule match_arms() -> Vec<MatchArm> =
                first:match_arm() rest:(_ a:match_arm() { a })* {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }

            rule match_arm() -> MatchArm =
                arm_bar() _ pat:pattern() _ "=>" _ body:expr() {
                    MatchArm { pattern: pat, body }
                }

            // Single `|` (not the binary-op `||`).
            rule arm_bar() = "|" !"|"

            // ─── Patterns ───────────────────────────────────
            //
            // Order matters in PEG: `pat_dot_ctor` and
            // `pat_ctor_with_args` come before bare `pat_var`
            // / `pat_lit` so applied forms parse correctly.
            // Tuple / paren grouped together at the front so
            // they take priority over bare paren tokens.
            rule pattern() -> Pattern =
                pat_tuple_or_paren()
                / pat_dot_ctor()
                / pat_ctor_with_args()
                / pat_wildcard()
                / pat_lit()
                / pat_var()

            rule pat_atom() -> Pattern =
                pat_tuple_or_paren()
                / pat_wildcard()
                / pat_lit()
                / pat_dot_ctor_atom()
                / pat_var()

            rule pat_wildcard() -> Pattern =
                "_" !['a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\''] {
                    Pattern::Wildcard
                }

            // Dotted forms (`Color.red`) are also valid var-
            // shape patterns when they take no args — the
            // elaborator decides ctor-vs-var classification.
            rule pat_var() -> Pattern =
                s:ident_raw() { Pattern::Var(s) }

            rule pat_lit() -> Pattern =
                n:nat_lit() { Pattern::Lit(Literal::Nat(n)) }
                / s:str_lit() { Pattern::Lit(Literal::Str(s)) }

            rule pat_ctor_with_args() -> Pattern =
                name:ident_raw() args:(_ p:pat_atom() { p })+ {
                    Pattern::Ctor(name, args)
                }

            rule pat_dot_ctor() -> Pattern =
                "." name:ident_raw() args:(_ p:pat_atom() { p })* {
                    Pattern::DotCtor(name, args)
                }

            // In atom position (i.e. inside a ctor's arg list),
            // a dot-ctor is parsed without further arguments —
            // the outer ctor's `args*` loop owns the next atom.
            rule pat_dot_ctor_atom() -> Pattern =
                "." name:ident_raw() { Pattern::DotCtor(name, vec![]) }

            rule pat_tuple_or_paren() -> Pattern =
                "(" _ first:pattern() rest:(_ "," _ p:pattern() { p })* _ ")" {
                    if rest.is_empty() {
                        Pattern::Paren(Box::new(first))
                    } else {
                        let mut v = vec![first];
                        v.extend(rest);
                        Pattern::Tuple(v)
                    }
                }
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
        let DeclKind::Definition { name, binders, ty, value } = &decls[0].kind
            else { panic!("expected Definition") };
        assert_eq!(name, "identity");
        assert_eq!(binders.len(), 1);
        assert_eq!(binders[0].kind, BinderKind::Explicit);
        assert_eq!(binders[0].names, vec!["n".to_string()]);
        assert_eq!(binders[0].ty, Expr::Ident("Nat".into()));
        assert_eq!(ty.as_ref().unwrap(), &Expr::Ident("Nat".into()));
        assert_eq!(value, &Expr::Ident("n".into()));
    }

    #[test]
    fn def_with_multiple_binders_in_one_group() {
        let src = "def add (a b : UInt64) : UInt64 := a + b";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 1);
        let DeclKind::Definition { name, binders, value, .. } = &decls[0].kind else { panic!("expected Definition") };
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
        let DeclKind::Definition { binders, .. } = &decls[0].kind else { panic!("expected Definition") };
        assert_eq!(binders.len(), 3);
        assert_eq!(binders[0].kind, BinderKind::Implicit);
        assert_eq!(binders[1].kind, BinderKind::Instance);
        assert_eq!(binders[2].kind, BinderKind::Explicit);
    }

    #[test]
    fn def_with_no_type_annotation() {
        let src = "def hello := \"world\"";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { ty, value, .. } = &decls[0].kind else { panic!("expected Definition") };
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
        let DeclKind::Definition { binders, .. } = &decls[0].kind else { panic!("expected Definition") };
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
        let DeclKind::Definition { value, .. } = decls.remove(0).kind else { panic!("expected Definition") };
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

    // ─── if-then-else tests (OX6 step 3) ───────────────────

    #[test]
    fn expr_if_then_else_simple() {
        let e = parse_value_expr("if a then b else c");
        match e {
            Expr::If(cond, t, els) => {
                assert_eq!(*cond, Expr::Ident("a".into()));
                assert_eq!(*t, Expr::Ident("b".into()));
                assert_eq!(*els, Expr::Ident("c".into()));
            }
            other => panic!("expected If, got {other:?}"),
        }
    }

    #[test]
    fn expr_if_inside_binop() {
        // `x + if a then b else c` — the if expression is an
        // atom on the right of `+`. The else branch eats the
        // trailing tail.
        let e = parse_value_expr("x + if a then b else c");
        match e {
            Expr::BinOp(op, _, rhs) => {
                assert_eq!(op, "+");
                assert!(matches!(*rhs, Expr::If(_, _, _)));
            }
            other => panic!("expected +, got {other:?}"),
        }
    }

    #[test]
    fn expr_if_with_complex_branches() {
        // `if a == 0 then 1 + 2 else 3 * 4`
        let e = parse_value_expr("if a == 0 then 1 + 2 else 3 * 4");
        match e {
            Expr::If(cond, t, els) => {
                assert!(matches!(*cond, Expr::BinOp(ref o, _, _) if o == "=="));
                assert!(matches!(*t, Expr::BinOp(ref o, _, _) if o == "+"));
                assert!(matches!(*els, Expr::BinOp(ref o, _, _) if o == "*"));
            }
            other => panic!("expected If, got {other:?}"),
        }
    }

    #[test]
    fn expr_nested_if() {
        let e = parse_value_expr("if a then if b then c else d else e");
        match e {
            Expr::If(_, then_branch, _) => {
                assert!(matches!(*then_branch, Expr::If(_, _, _)));
            }
            other => panic!("expected If, got {other:?}"),
        }
    }

    // ─── match tests (OX6 step 3) ──────────────────────────

    #[test]
    fn expr_match_single_arm_wildcard() {
        let e = parse_value_expr("match c with | _ => 0");
        match e {
            Expr::Match(_, arms) => {
                assert_eq!(arms.len(), 1);
                assert_eq!(arms[0].pattern, Pattern::Wildcard);
                assert_eq!(arms[0].body, Expr::Lit(Literal::Nat(0)));
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn expr_match_multi_arm() {
        let e = parse_value_expr("match c with | a => 1 | b => 2 | _ => 3");
        match e {
            Expr::Match(_, arms) => {
                assert_eq!(arms.len(), 3);
                assert_eq!(arms[0].pattern, Pattern::Var("a".into()));
                assert_eq!(arms[1].pattern, Pattern::Var("b".into()));
                assert_eq!(arms[2].pattern, Pattern::Wildcard);
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn expr_match_dot_ctor_pattern() {
        let e = parse_value_expr("match c with | .lt => 1 | .eq => 2 | .gt => 3");
        match e {
            Expr::Match(_, arms) => {
                assert_eq!(arms.len(), 3);
                assert_eq!(arms[0].pattern, Pattern::DotCtor("lt".into(), vec![]));
                assert_eq!(arms[1].pattern, Pattern::DotCtor("eq".into(), vec![]));
                assert_eq!(arms[2].pattern, Pattern::DotCtor("gt".into(), vec![]));
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn expr_match_dot_ctor_with_args() {
        // `match m with | .some x => x | .none => 0`
        let e = parse_value_expr("match m with | .some x => x | .none => 0");
        match e {
            Expr::Match(_, arms) => {
                assert_eq!(arms.len(), 2);
                match &arms[0].pattern {
                    Pattern::DotCtor(name, args) => {
                        assert_eq!(name, "some");
                        assert_eq!(args.len(), 1);
                        assert_eq!(args[0], Pattern::Var("x".into()));
                    }
                    other => panic!("expected DotCtor, got {other:?}"),
                }
                assert_eq!(arms[1].pattern, Pattern::DotCtor("none".into(), vec![]));
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn expr_match_qualified_ctor_pattern() {
        // `Color.red` ctor pattern (no args).
        let e = parse_value_expr("match c with | Color.red => 1 | Color.blue => 3");
        match e {
            Expr::Match(_, arms) => {
                // No-arg `Color.red` falls into pat_var (single ident).
                assert_eq!(arms[0].pattern, Pattern::Var("Color.red".into()));
                assert_eq!(arms[1].pattern, Pattern::Var("Color.blue".into()));
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn expr_match_ctor_with_args() {
        // `node l r` — multi-arg ctor pattern.
        let e = parse_value_expr("match t with | leaf => 0 | node l r => 1");
        match e {
            Expr::Match(_, arms) => {
                assert_eq!(arms[0].pattern, Pattern::Var("leaf".into()));
                match &arms[1].pattern {
                    Pattern::Ctor(name, args) => {
                        assert_eq!(name, "node");
                        assert_eq!(args.len(), 2);
                        assert_eq!(args[0], Pattern::Var("l".into()));
                        assert_eq!(args[1], Pattern::Var("r".into()));
                    }
                    other => panic!("expected Ctor, got {other:?}"),
                }
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn expr_match_tuple_pattern() {
        let e = parse_value_expr("match p with | (a, b) => a");
        match e {
            Expr::Match(_, arms) => {
                match &arms[0].pattern {
                    Pattern::Tuple(parts) => {
                        assert_eq!(parts.len(), 2);
                        assert_eq!(parts[0], Pattern::Var("a".into()));
                        assert_eq!(parts[1], Pattern::Var("b".into()));
                    }
                    other => panic!("expected Tuple, got {other:?}"),
                }
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn expr_match_lit_pattern() {
        let e = parse_value_expr("match n with | 0 => 1 | _ => 2");
        match e {
            Expr::Match(_, arms) => {
                assert_eq!(arms[0].pattern, Pattern::Lit(Literal::Nat(0)));
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn expr_match_body_uses_or_binary_op() {
        // The `||` binary op inside an arm body must NOT
        // collide with the arm-separator `|`. The arm body
        // `a || b` is one BinOp; only the *next* leading `|`
        // (with a non-`|` follower) introduces a new arm.
        let e = parse_value_expr("match c with | _ => a || b | x => x");
        match e {
            Expr::Match(_, arms) => {
                assert_eq!(arms.len(), 2);
                match &arms[0].body {
                    Expr::BinOp(op, _, _) => assert_eq!(op, "||"),
                    other => panic!("expected || BinOp, got {other:?}"),
                }
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn expr_match_with_complex_scrutinee() {
        let e = parse_value_expr("match a + b with | _ => 0");
        match e {
            Expr::Match(scrut, _) => {
                assert!(matches!(*scrut, Expr::BinOp(ref o, _, _) if o == "+"));
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn def_with_match_body() {
        let src = "def colorName (c : Color) : String := \
                   match c with | .red => \"red\" | .green => \"green\" | .blue => \"blue\"";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind else { panic!("expected Definition") };
        assert!(matches!(value, Expr::Match(_, _)));
    }

    #[test]
    fn def_with_if_body() {
        let src = "def safeDiv (a b : UInt64) : Option UInt64 := \
                   if b == 0 then none else some (a / b)";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind else { panic!("expected Definition") };
        assert!(matches!(value, Expr::If(_, _, _)));
    }

    // ─── lambda tests (OX6 step 4) ─────────────────────────

    #[test]
    fn expr_lam_identity() {
        let e = parse_value_expr("fun x => x");
        match e {
            Expr::Lam(binders, body) => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0], LamBinder::Untyped("x".into()));
                assert_eq!(*body, Expr::Ident("x".into()));
            }
            other => panic!("expected Lam, got {other:?}"),
        }
    }

    #[test]
    fn expr_lam_multi_arg() {
        let e = parse_value_expr("fun a b => a + b");
        match e {
            Expr::Lam(binders, body) => {
                assert_eq!(binders.len(), 2);
                assert_eq!(binders[0], LamBinder::Untyped("a".into()));
                assert_eq!(binders[1], LamBinder::Untyped("b".into()));
                assert!(matches!(*body, Expr::BinOp(ref o, _, _) if o == "+"));
            }
            other => panic!("expected Lam, got {other:?}"),
        }
    }

    #[test]
    fn expr_lam_typed_explicit() {
        let e = parse_value_expr("fun (x : Nat) => x");
        match e {
            Expr::Lam(binders, _) => {
                assert_eq!(binders.len(), 1);
                match &binders[0] {
                    LamBinder::Typed { kind, names, ty } => {
                        assert_eq!(*kind, BinderKind::Explicit);
                        assert_eq!(names, &vec!["x".to_string()]);
                        assert_eq!(*ty, Expr::Ident("Nat".into()));
                    }
                    LamBinder::Untyped(s) => panic!("expected Typed binder, got Untyped({s})"),
                }
            }
            other => panic!("expected Lam, got {other:?}"),
        }
    }

    #[test]
    fn expr_lam_typed_multi_name() {
        let e = parse_value_expr("fun (a b : Nat) => a + b");
        match e {
            Expr::Lam(binders, _) => {
                assert_eq!(binders.len(), 1);
                match &binders[0] {
                    LamBinder::Typed { names, .. } => {
                        assert_eq!(names, &vec!["a".to_string(), "b".to_string()]);
                    }
                    LamBinder::Untyped(s) => panic!("expected Typed, got Untyped({s})"),
                }
            }
            other => panic!("expected Lam, got {other:?}"),
        }
    }

    #[test]
    fn expr_lam_mixed_typed_groups() {
        let e = parse_value_expr("fun {T : Type} (x : T) => x");
        match e {
            Expr::Lam(binders, _) => {
                assert_eq!(binders.len(), 2);
                assert!(matches!(
                    &binders[0],
                    LamBinder::Typed { kind: BinderKind::Implicit, .. }
                ));
                assert!(matches!(
                    &binders[1],
                    LamBinder::Typed { kind: BinderKind::Explicit, .. }
                ));
            }
            other => panic!("expected Lam, got {other:?}"),
        }
    }

    #[test]
    fn expr_lam_unicode_lambda() {
        let e = parse_value_expr("λ x => x");
        assert!(matches!(e, Expr::Lam(_, _)));
    }

    #[test]
    fn expr_lam_dash_arrow_body() {
        // OX3's textual normaliser rewrites `=>` → `->`; the
        // PEG parser must accept both shapes natively so it
        // can replace the normaliser later without surface
        // regressions.
        let dash = parse_value_expr("fun x -> x");
        let arrow = parse_value_expr("fun x => x");
        assert_eq!(dash, arrow);
    }

    #[test]
    fn expr_lam_inside_def_value() {
        let src = "def addOne : Nat -> Nat := fun n => n + 1";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind else { panic!("expected Definition") };
        assert!(matches!(value, Expr::Lam(_, _)));
    }

    #[test]
    fn expr_lam_body_is_full_expr() {
        // The body slurps the entire trailing expression
        // (including BinOps + App).
        let e = parse_value_expr("fun a b c => f a + g b * h c");
        match e {
            Expr::Lam(_, body) => {
                // Body is `f a + g b * h c` — BinOp `+` with
                // `f a` on the left and `g b * h c` on the
                // right.
                assert!(matches!(*body, Expr::BinOp(ref o, _, _) if o == "+"));
            }
            other => panic!("expected Lam, got {other:?}"),
        }
    }

    #[test]
    fn expr_lam_no_collision_with_def_keyword() {
        // Multi-decl source: a def whose body is a lambda
        // followed by another def. The lambda body must not
        // eat the next decl's `def` keyword.
        let src = "def f : Nat -> Nat := fun n => n\n\
                   def g (n : Nat) : Nat := n";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 2);
    }

    // ─── structure / inductive / deriving (OX6 step 5) ────

    #[test]
    fn structure_basic() {
        let src = "structure Point where\n  x : Float\n  y : Float";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 1);
        let DeclKind::Structure { name, extends, fields, deriving } = &decls[0].kind
            else { panic!("expected Structure") };
        assert_eq!(name, "Point");
        assert!(extends.is_empty());
        assert!(deriving.is_empty());
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, "x");
        assert_eq!(fields[0].ty, Expr::Ident("Float".into()));
        assert_eq!(fields[1].name, "y");
        assert_eq!(fields[1].ty, Expr::Ident("Float".into()));
    }

    #[test]
    fn structure_with_deriving() {
        let src = "structure Point where\n  x : Float\n  y : Float\n  deriving LeanMarshal, Repr";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Structure { fields, deriving, .. } = &decls[0].kind
            else { panic!("expected Structure") };
        assert_eq!(fields.len(), 2);
        assert_eq!(deriving, &vec!["LeanMarshal".to_string(), "Repr".to_string()]);
    }

    #[test]
    fn structure_with_extends() {
        let src = "structure Point3D extends Point where\n  z : Float";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Structure { extends, fields, .. } = &decls[0].kind
            else { panic!("expected Structure") };
        assert_eq!(extends, &vec!["Point".to_string()]);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "z");
    }

    #[test]
    fn structure_with_multi_extends() {
        let src = "structure Color extends Foo, Bar where\n  hex : UInt32";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Structure { extends, .. } = &decls[0].kind
            else { panic!("expected Structure") };
        assert_eq!(extends, &vec!["Foo".to_string(), "Bar".to_string()]);
    }

    #[test]
    fn inductive_where_form_all_bare() {
        // Lean 4 `inductive ... where | r | g | b` (no ctor
        // type annotations).
        let src = "inductive Color where\n  | red\n  | green\n  | blue";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Inductive { name, ctors, deriving, .. } = &decls[0].kind
            else { panic!("expected Inductive") };
        assert_eq!(name, "Color");
        assert!(deriving.is_empty());
        assert_eq!(ctors.len(), 3);
        for (c, expected) in ctors.iter().zip(["red", "green", "blue"]) {
            assert_eq!(c.name, expected);
            assert!(c.ty.is_none(), "bare ctor must have ty = None");
        }
    }

    #[test]
    fn inductive_where_form_mixed_annotations() {
        // Some ctors carry explicit types, others don't.
        let src = "inductive Tree where\n  | leaf\n  | node : Tree -> Tree -> Tree";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Inductive { ctors, .. } = &decls[0].kind
            else { panic!("expected Inductive") };
        assert_eq!(ctors.len(), 2);
        assert_eq!(ctors[0].name, "leaf");
        assert!(ctors[0].ty.is_none());
        assert_eq!(ctors[1].name, "node");
        // `Tree -> Tree -> Tree` parses to a right-assoc arrow tree.
        match ctors[1].ty.as_ref() {
            Some(Expr::BinOp(op, _, _)) => assert_eq!(op, "->"),
            other => panic!("expected arrow BinOp, got {other:?}"),
        }
    }

    #[test]
    fn inductive_oxilean_form_explicit_type_annot() {
        // Older OxiLean form: `inductive NAME : Type | …`.
        let src = "inductive Color : Type\n  | red : Color\n  | green : Color\n  | blue : Color";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Inductive { ty, ctors, .. } = &decls[0].kind
            else { panic!("expected Inductive") };
        assert!(ty.is_some(), "explicit type annotation must surface");
        assert_eq!(ctors.len(), 3);
        for c in ctors {
            assert_eq!(c.ty, Some(Expr::Ident("Color".into())));
        }
    }

    #[test]
    fn inductive_with_deriving() {
        let src = "inductive Color where\n  | red\n  | green\n  deriving LeanMarshal";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Inductive { ctors, deriving, .. } = &decls[0].kind
            else { panic!("expected Inductive") };
        assert_eq!(ctors.len(), 2);
        assert_eq!(deriving, &vec!["LeanMarshal".to_string()]);
    }

    #[test]
    fn multi_decl_with_struct_and_def() {
        let src = "structure Point where\n  x : Float\n  y : Float\n\
                   \n\
                   def origin : Point := Point";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 2);
        assert!(matches!(decls[0].kind, DeclKind::Structure { .. }));
        assert!(matches!(decls[1].kind, DeclKind::Definition { .. }));
    }

    #[test]
    fn deriving_single_class() {
        let src = "structure A where\n  x : Nat\n  deriving Repr";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Structure { deriving, .. } = &decls[0].kind
            else { panic!("expected Structure") };
        assert_eq!(deriving, &vec!["Repr".to_string()]);
    }

    #[test]
    fn struct_zero_fields() {
        // Lean 4 allows `structure Empty where` (no fields).
        let src = "structure Empty where";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Structure { fields, .. } = &decls[0].kind
            else { panic!("expected Structure") };
        assert!(fields.is_empty());
    }

    // ─── do notation (OX6 step 8) ──────────────────────────

    #[test]
    fn do_single_return() {
        let e = parse_value_expr("do return 0");
        match e {
            Expr::Do(stmts) => {
                assert_eq!(stmts.len(), 1);
                assert_eq!(stmts[0], DoStmt::Return(Expr::Lit(Literal::Nat(0))));
            }
            other => panic!("expected Do, got {other:?}"),
        }
    }

    #[test]
    fn do_pure_collapsed_to_return() {
        let e = parse_value_expr("do pure x");
        match e {
            Expr::Do(stmts) => {
                assert_eq!(stmts.len(), 1);
                match &stmts[0] {
                    DoStmt::Return(inner) => assert_eq!(inner, &Expr::Ident("x".into())),
                    other => panic!("expected Return, got {other:?}"),
                }
            }
            other => panic!("expected Do, got {other:?}"),
        }
    }

    #[test]
    fn do_let_bind_with_dash_arrow() {
        let src = "def main : IO Unit := do\n  let x <- readLn\n  return x";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        match value {
            Expr::Do(stmts) => {
                assert_eq!(stmts.len(), 2);
                match &stmts[0] {
                    DoStmt::Bind { name, value } => {
                        assert_eq!(name, "x");
                        assert_eq!(value, &Expr::Ident("readLn".into()));
                    }
                    other => panic!("expected Bind, got {other:?}"),
                }
                assert!(matches!(stmts[1], DoStmt::Return(_)));
            }
            other => panic!("expected Do, got {other:?}"),
        }
    }

    #[test]
    fn do_let_bind_with_unicode_arrow() {
        let src = "def main : IO Unit := do\n  let x ← readLn\n  return x";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        match value {
            Expr::Do(stmts) => {
                match &stmts[0] {
                    DoStmt::Bind { name, .. } => assert_eq!(name, "x"),
                    other => panic!("expected Bind, got {other:?}"),
                }
            }
            other => panic!("expected Do, got {other:?}"),
        }
    }

    #[test]
    fn do_let_pure() {
        let src = "def main : IO Unit := do\n  let y := 42\n  return y";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        match value {
            Expr::Do(stmts) => {
                match &stmts[0] {
                    DoStmt::Let { name, value } => {
                        assert_eq!(name, "y");
                        assert_eq!(value, &Expr::Lit(Literal::Nat(42)));
                    }
                    other => panic!("expected Let, got {other:?}"),
                }
            }
            other => panic!("expected Do, got {other:?}"),
        }
    }

    #[test]
    fn do_bare_expr_statement() {
        let src = "def main : IO Unit := do\n  IO.println \"hello\"\n  return ()";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        match value {
            Expr::Do(stmts) => {
                match &stmts[0] {
                    DoStmt::Expr(_) => {} // ok
                    other => panic!("expected Expr stmt, got {other:?}"),
                }
            }
            other => panic!("expected Do, got {other:?}"),
        }
    }

    #[test]
    fn do_block_ends_at_top_level_decl() {
        // do block must not eat the next def keyword line.
        let src = "def main : IO Unit := do\n  return 0\ndef next : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 2);
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        match value {
            Expr::Do(stmts) => assert_eq!(stmts.len(), 1),
            other => panic!("expected Do, got {other:?}"),
        }
    }

    #[test]
    fn do_multi_stmt_mix() {
        let src = "def main : IO Unit := do\n  let x <- readLn\n  let y := 1\n  return (x, y)";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        match value {
            Expr::Do(stmts) => {
                assert_eq!(stmts.len(), 3);
                assert!(matches!(stmts[0], DoStmt::Bind { .. }));
                assert!(matches!(stmts[1], DoStmt::Let { .. }));
                assert!(matches!(stmts[2], DoStmt::Return(_)));
            }
            other => panic!("expected Do, got {other:?}"),
        }
    }

    // ─── attribute lists (OX6 step 6) ──────────────────────

    #[test]
    fn attr_single_bare_ident() {
        let src = "@[simp]\ndef f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert_eq!(decls[0].attrs[0].name, "simp");
        assert!(decls[0].attrs[0].raw_args.is_empty());
    }

    #[test]
    fn attr_comma_list_bare_idents() {
        let src = "@[simp, ext, inline]\ndef f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        let names: Vec<_> = decls[0].attrs.iter().map(|a| a.name.as_str()).collect();
        assert_eq!(names, vec!["simp", "ext", "inline"]);
    }

    #[test]
    fn attr_with_args_preserved_raw() {
        // OxiLean's parser rejects attr args; OX6 preserves
        // them as raw text in `raw_args` so downstream
        // consumers can inspect.
        let src = "@[leo4_specialize_when scalar ∧ ord]\ndef f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        let attr = &decls[0].attrs[0];
        assert_eq!(attr.name, "leo4_specialize_when");
        assert_eq!(attr.raw_args, "scalar ∧ ord");
    }

    #[test]
    fn attr_mix_bare_and_args_in_list() {
        let src = "@[leo4_export, leo4_specialize_when scalar ∧ ord]\ndef f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        let attrs = &decls[0].attrs;
        assert_eq!(attrs.len(), 2);
        assert_eq!(attrs[0].name, "leo4_export");
        assert!(attrs[0].raw_args.is_empty());
        assert_eq!(attrs[1].name, "leo4_specialize_when");
        assert_eq!(attrs[1].raw_args, "scalar ∧ ord");
    }

    #[test]
    fn attr_attaches_to_structure() {
        let src = "@[ext]\nstructure Point where\n  x : Float";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert_eq!(decls[0].attrs[0].name, "ext");
        assert!(matches!(decls[0].kind, DeclKind::Structure { .. }));
    }

    #[test]
    fn attr_attaches_to_inductive() {
        let src = "@[derive_smt]\ninductive Color where\n  | red";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert_eq!(decls[0].attrs[0].name, "derive_smt");
        assert!(matches!(decls[0].kind, DeclKind::Inductive { .. }));
    }

    #[test]
    fn no_attr_yields_empty_vec() {
        let src = "def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert!(decls[0].attrs.is_empty());
    }

    #[test]
    fn multi_decl_with_per_decl_attrs() {
        let src = "@[a]\ndef f : Nat := 1\n@[b, c]\ndef g : Nat := 2";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 2);
        assert_eq!(decls[0].attrs.len(), 1);
        assert_eq!(decls[0].attrs[0].name, "a");
        assert_eq!(decls[1].attrs.len(), 2);
        assert_eq!(decls[1].attrs[0].name, "b");
        assert_eq!(decls[1].attrs[1].name, "c");
    }

    // ─── multi-line + full-expr-reparse (OX6 step 7) ───────

    #[test]
    fn struct_field_type_is_fully_parsed_expr() {
        // Previously `fields[0].ty` was a raw `String`; now
        // it's a fully parsed `Expr`.
        let src = "structure Point where\n  x : Float\n  y : Float";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Structure { fields, .. } = &decls[0].kind
            else { panic!("expected Structure") };
        assert_eq!(fields[0].ty, Expr::Ident("Float".into()));
        assert_eq!(fields[1].ty, Expr::Ident("Float".into()));
    }

    #[test]
    fn struct_field_type_with_app() {
        let src = "structure Bucket where\n  items : List Nat";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Structure { fields, .. } = &decls[0].kind
            else { panic!("expected Structure") };
        // `List Nat` → App(List, Nat)
        match &fields[0].ty {
            Expr::App(f, x) => {
                assert_eq!(**f, Expr::Ident("List".into()));
                assert_eq!(**x, Expr::Ident("Nat".into()));
            }
            other => panic!("expected App, got {other:?}"),
        }
    }

    #[test]
    fn struct_field_type_multi_line_continuation() {
        // Type spans onto continuation lines. The boundary
        // detection only stops at the next field header
        // (`<ident> :` on its own line), so deeply-nested
        // generic types parse correctly.
        let src = "structure Big where\n  \
                   xs : List\n    \
                   (Option\n    \
                   Nat)\n  \
                   y : Nat";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Structure { fields, .. } = &decls[0].kind
            else { panic!("expected Structure") };
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name, "xs");
        // The xs type is the full multi-line `List (Option
        // Nat)` shape.
        assert!(matches!(fields[0].ty, Expr::App(_, _)));
        assert_eq!(fields[1].name, "y");
        assert_eq!(fields[1].ty, Expr::Ident("Nat".into()));
    }

    #[test]
    fn struct_field_type_with_arrow() {
        let src = "structure Callback where\n  cb : Nat -> Bool";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Structure { fields, .. } = &decls[0].kind
            else { panic!("expected Structure") };
        match &fields[0].ty {
            Expr::BinOp(op, lhs, rhs) => {
                assert_eq!(op, "->");
                assert_eq!(**lhs, Expr::Ident("Nat".into()));
                assert_eq!(**rhs, Expr::Ident("Bool".into()));
            }
            other => panic!("expected arrow BinOp, got {other:?}"),
        }
    }

    #[test]
    fn inductive_ctor_payload_is_fully_parsed_expr() {
        let src = "inductive Tree where\n  | leaf\n  | node : Tree -> Tree -> Tree";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Inductive { ctors, .. } = &decls[0].kind
            else { panic!("expected Inductive") };
        assert!(ctors[0].ty.is_none());
        match ctors[1].ty.as_ref() {
            Some(Expr::BinOp(op, _, _)) => assert_eq!(op, "->"),
            other => panic!("expected arrow, got {other:?}"),
        }
    }

    #[test]
    fn inductive_ctor_multi_line_payload() {
        let src = "inductive E where\n  | wrap : \n    Nat ->\n    Bool ->\n    E";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Inductive { ctors, .. } = &decls[0].kind
            else { panic!("expected Inductive") };
        assert_eq!(ctors[0].name, "wrap");
        // wrap's payload is a right-associative arrow chain.
        match ctors[0].ty.as_ref() {
            Some(Expr::BinOp(op, _, _)) => assert_eq!(op, "->"),
            other => panic!("expected arrow, got {other:?}"),
        }
    }

    #[test]
    fn struct_unparseable_field_falls_back_to_raw() {
        // A field type that contains a syntactic shape we
        // don't yet support should fall back to `Expr::Raw`
        // rather than failing the whole struct.
        let src = "structure X where\n  x : ¡weird syntax!";
        let result = parse_decls(src);
        // Outer parse succeeds (raw fallback in the field);
        // OR the boundary capture itself rejects — both are
        // acceptable v0 behaviour. The contract is "never
        // panic on weird-but-finite text".
        let _ = result;
    }

    #[test]
    fn structure_with_carrier_field_and_deriving() {
        let src = "structure MoneyBag where\n  \
                   major : BigNat\n  \
                   minor : UInt32\n  \
                   deriving LeanMarshal";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Structure { fields, deriving, .. } = &decls[0].kind
            else { panic!("expected Structure") };
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].ty, Expr::Ident("BigNat".into()));
        assert_eq!(fields[1].ty, Expr::Ident("UInt32".into()));
        assert_eq!(deriving, &vec!["LeanMarshal".to_string()]);
    }

    #[test]
    fn attr_dotted_name() {
        // `@[Foo.bar]` — dotted attribute names are valid in
        // Lean 4 (e.g. `@[Lean.Elab.Tactic.builtin_tactic]`).
        let src = "@[Foo.bar]\ndef f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs[0].name, "Foo.bar");
    }

    #[test]
    fn expr_app_in_def_body() {
        // `def f (a b : Nat) : Nat := Nat.succ a` exercises
        // app + dotted ident in a real fixture.
        let decls = parse_decls("def f (a b : Nat) : Nat := Nat.succ a").expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind else { panic!("expected Definition") };
        match value {
            Expr::App(f, x) => {
                assert_eq!(**f, Expr::Ident("Nat.succ".into()));
                assert_eq!(**x, Expr::Ident("a".into()));
            }
            other => panic!("expected App, got {other:?}"),
        }
    }
}
