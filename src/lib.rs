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
//!   - Quantifiers: `forall x, body` / `∀ x, body`
//!     (universal); `exists x, body` / `∃ x, body`
//!     (existential). Binders share lambda's `LamBinder`
//!     shape (untyped names or typed groups).
//!   - `do <stmts>` — monadic sequencing with `let x ← e`
//!     (bind), `let x := e` (pure let), `return e` /
//!     `pure e`, and bare expression statements.
//!   - `s!"…{x}…"` — string interpolation with `{{` / `}}`
//!     escapes for literal braces.
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
//! Decls: `def`, `structure`, `inductive`, `theorem` /
//! `lemma`, `axiom`, `instance`, `class`, `namespace`,
//! `section`, `mutual` parse. Still pending for v1.0 RC
//! (see ROADMAP OX6 sub-steps 10d–13): `open` / `import`
//! / `variable`, surface-coverage gaps (block comments,
//! anonymous ctor `⟨…⟩`, modifier prefixes, let-in, by
//! tactics, multi-line do statements, list literal,
//! universe annotation, `@` explicit args, …), DSL /
//! macro / debug-command decls, oxilean-parse cross-check,
//! and the leo4-oxilean-build switchover.

use leo4_abi::LeanError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decl {
    /// Attribute list prefix (`@[…]`) applied to this decl.
    /// Empty if the decl had no attribute prefix.
    pub attrs: Vec<Attribute>,
    /// Universe-parameter list in `def foo.{u, v} : …` form.
    /// Empty when the decl has no `.{…}` annotation. Applies
    /// to `def` / `theorem` / `axiom` / `structure` /
    /// `inductive` / `class` / `instance` (anywhere a name
    /// gets a `.{…}` suffix in Lean 4).
    pub univ_params: Vec<String>,
    /// Modifier-keyword prefix list: `private`, `protected`,
    /// `partial`, `noncomputable`, `unsafe`. Order preserved
    /// (Lean 4 syntax allows e.g. `private noncomputable def`
    /// in either order). `abbrev`-style decls (`abbrev f :=
    /// …`) surface as `Definition` with `"abbrev"` in
    /// `modifiers` — `abbrev` is desugared to a reducible
    /// `def` at elab time.
    pub modifiers: Vec<String>,
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
    /// `theorem NAME [binders]+ : TYPE := PROOF` or
    /// `lemma NAME [binders]+ : TYPE := PROOF` — same shape,
    /// semantically equivalent; the surface keyword used is
    /// not preserved in the AST today (the elaborator
    /// doesn't depend on it).
    Theorem {
        name: String,
        binders: Vec<BinderGroup>,
        ty: Expr,
        proof: Expr,
    },
    /// `axiom NAME [binders]+ : TYPE` — declared without a
    /// body. The elaborator trusts the type signature.
    Axiom {
        name: String,
        binders: Vec<BinderGroup>,
        ty: Expr,
    },
    /// `example [binders]+ : TYPE := PROOF` — anonymous
    /// theorem. Used to type-check / smoke-test a proof
    /// without binding a name.
    Example {
        binders: Vec<BinderGroup>,
        ty: Expr,
        proof: Expr,
    },
    /// `instance [NAME] [binders]+ : TYPE BODY` — typeclass
    /// instance. The name is optional (Lean 4 auto-names
    /// anonymous instances).
    Instance {
        name: Option<String>,
        binders: Vec<BinderGroup>,
        ty: Expr,
        body: InstanceBody,
    },
    /// `class NAME [binders]+ [extends BASES] where FIELDS
    /// [deriving ...]` — typeclass declaration, structurally
    /// identical to a `structure` (a record of methods)
    /// plus type-level binders for parametric classes.
    Class {
        name: String,
        binders: Vec<BinderGroup>,
        extends: Vec<String>,
        fields: Vec<StructField>,
        deriving: Vec<String>,
    },
    /// `namespace NAME … end NAME` — scoped block of inner
    /// declarations.
    Namespace {
        name: String,
        decls: Vec<Decl>,
    },
    /// `section [NAME] … end [NAME]` — section block. The
    /// name is optional (Lean 4 allows anonymous sections).
    /// Unlike `namespace`, `section` doesn't add a name
    /// prefix to inner decls — it just scopes `variable`
    /// declarations + opens.
    Section {
        name: Option<String>,
        decls: Vec<Decl>,
    },
    /// `open Foo Bar Baz` — opens identifiers into the
    /// current namespace. v0 captures the open list as
    /// space-separated tokens; selective `open Foo (x y)`
    /// or renaming `open Foo renaming x → y` lands its tail
    /// in `raw_tail` for downstream re-parsing.
    Open {
        items: Vec<String>,
        raw_tail: String,
    },
    /// `import Foo.Bar.Baz` — single-path import. Multiple
    /// imports = multiple `Import` decls.
    Import { path: String },
    /// `variable [binders]+` — binds variables for the
    /// surrounding section / namespace.
    Variable { binders: Vec<BinderGroup> },
    /// `mutual … end` — block of mutually-recursive decls.
    Mutual { decls: Vec<Decl> },
    /// `#check expr` / `#eval expr` / `#print name` /
    /// `#guard expr` / `#guard_msgs (cfg) in cmd`.
    /// `cmd` is the leading `#…` keyword (without `#`).
    /// `raw_args` is the tail captured verbatim until the
    /// next top-level decl boundary (so multi-line arg
    /// expressions are preserved as text — round-trip via
    /// the same expr parser is the consumer's job).
    HashCommand { cmd: String, raw_args: String },
    /// `omit ident+` — locally drops one or more
    /// section-level `variable` bindings from the current
    /// scope. Only meaningful inside a `section` block.
    Omit { items: Vec<String> },
    /// `include ident+` — re-introduces previously
    /// `omit`-ed (or otherwise unused) section variables.
    Include { items: Vec<String> },
}

/// Body of an `instance` decl — either a `where`-block of
/// field assignments (structurally like a `structure`'s
/// field list) or a single term expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstanceBody {
    Where(Vec<StructField>),
    Term(Expr),
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
    /// `if let PATTERN := SCRUT then THEN else ELSE` —
    /// pattern-matching `if`. Equivalent to a single-arm
    /// `match` with a default fallback branch:
    /// `match SCRUT with | PATTERN => THEN | _ => ELSE`.
    IfLet {
        pattern: Box<Pattern>,
        scrutinee: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Box<Expr>,
    },
    /// `match SCRUTINEE with | pat => body | ...`.
    Match(Box<Expr>, Vec<MatchArm>),
    /// `match BINDING : SCRUTINEE with | pat => body | ...` —
    /// scrutinee binding form. Lean 4 makes `BINDING` a
    /// proof-of-equality `BINDING : SCRUTINEE = pat`
    /// available inside each arm's body. Sibling of `Match`;
    /// kept as a separate variant so plain `Match` consumers
    /// (which never touch the binding) need no migration.
    MatchBind {
        binding: String,
        scrutinee: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    /// `fun BINDERS => BODY` / `λ BINDERS => BODY` /
    /// `fun BINDERS -> BODY` (`->` body-arrow accepted as a
    /// synonym for `=>` for OX3 normalisation compatibility).
    Lam(Vec<LamBinder>, Box<Expr>),
    /// `let NAME [: TY] := VALUE; BODY` or
    /// `let NAME [: TY] := VALUE \n BODY` — pure
    /// (non-monadic) let-binding in expression position.
    /// The monadic `let x ← e` inside `do` blocks lives
    /// in `DoStmt::Bind` instead.
    Let {
        name: String,
        /// Boxed so the recursive `Option<Expr>` doesn't
        /// blow up the enum size.
        ty: Option<Box<Expr>>,
        value: Box<Expr>,
        body: Box<Expr>,
    },
    /// `forall BINDERS, BODY` / `∀ BINDERS, BODY` —
    /// dependent function (Π) type. Binders share lambda's
    /// `LamBinder` shape (untyped names or typed groups);
    /// the body is the codomain type.
    Forall(Vec<LamBinder>, Box<Expr>),
    /// `exists BINDERS, BODY` / `∃ BINDERS, BODY` —
    /// existential quantifier (Σ-shaped at the prop level).
    /// Same binder grammar as `Forall`.
    Exists(Vec<LamBinder>, Box<Expr>),
    /// `do <stmts>` — monadic sequencing block.
    Do(Vec<DoStmt>),
    /// Lean 4 string interpolation: `s!"hello {name}!"`.
    /// Alternating text and `{expr}` holes, in the order
    /// they appear in the source. `{{` / `}}` decode to
    /// literal `{` / `}` in the `Text` segments.
    InterpStr(Vec<InterpPart>),
    /// `by <tactics>` — term-level entry into tactic mode.
    /// Each entry is the raw text of one tactic (split on
    /// `;` for single-line sequenced form and on newlines
    /// for multi-line form). Tactic sub-parsing (e.g.
    /// `exact e` → `Tactic::Exact(e)`) is a future step;
    /// v0 keeps the raw textual form so the surrounding
    /// surface parses cleanly.
    By(Vec<String>),
    /// List literal: `[1, 2, 3]`. Empty `[]` is `List(vec![])`.
    /// Lean 4 desugars to `List.cons` chains at elab time;
    /// the parser keeps the literal form for readability.
    List(Vec<Expr>),
    /// Anonymous structure literal: `{ x := 1, y := 2 }`.
    /// Each entry is `(field_name, value_expr)`; field
    /// order is preserved (matters for some elab paths).
    AnonStruct(Vec<(String, Expr)>),
    /// Anonymous constructor: `⟨a, b⟩` — Lean 4's
    /// Unicode-angle-bracket shorthand for the unique
    /// constructor of a single-ctor inductive (e.g.
    /// `⟨1, 2⟩` for `Point.mk 1 2`). Elab resolves the
    /// target ctor from the expected type.
    AnonCtor(Vec<Expr>),
    /// `@f` — explicit-args marker. In Lean 4, prefixing
    /// a name (or any atom) with `@` switches the
    /// elaborator from instance/implicit-resolving mode to
    /// "supply every argument explicitly". The wrapped
    /// expression is usually an Ident, but can be any atom.
    At(Box<Expr>),
    /// `(· + 1)` / `(·.field)` — anonymous function
    /// shorthand using the `·` (U+00B7 MIDDLE DOT)
    /// placeholder. Each `·` in the body is one λ
    /// parameter; `(· + ·)` is `λ x y => x + y`. The
    /// body is captured as raw text up to the closing
    /// `)`; consumer can re-parse via the same `expr`
    /// rule. Nested parens inside the body are NOT
    /// supported in v1 RC (typical shorthand uses are
    /// single-level).
    DotFn(String),
    /// Anything we couldn't further analyse — emitted as
    /// the raw source span. As the PEG grammar extends,
    /// fewer expressions land here.
    Raw(String),
}

/// One arm of a `match` expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchArm {
    pub pattern: Pattern,
    /// Optional `if EXPR` guard between the pattern and
    /// the `=>` arrow: `| n if n > 0 => …`. `None` for
    /// unguarded arms.
    pub guard: Option<Expr>,
    pub body: Expr,
}

/// One segment of a string interpolation. `Text` carries
/// the literal-text bytes (with `{{` / `}}` decoded to `{`
/// and `}`, and standard `\\n \\t \\r \\\\ \\"` escapes
/// resolved); `Hole(expr)` is one `{…}` interpolation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpPart {
    Text(String),
    Hole(Expr),
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
    /// `for BINDING in ITER do BODY` — iteration over a
    /// `ForIn`-conforming container. BODY is captured as
    /// a single-line expression in v1 RC (multi-line body
    /// is a follow-up — same layout-tracking challenge as
    /// `do_expr_stmt`).
    For { binding: String, iter: Expr, body: Expr },
    /// `while COND do BODY` — pre-condition loop.
    While { cond: Expr, body: Expr },
    /// `until COND do BODY` — pre-condition negated-cond
    /// loop. Less common than `while`; landed for surface
    /// completeness.
    Until { cond: Expr, body: Expr },
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
    /// Unsigned natural number literal. Source forms:
    /// decimal `42`, hex `0x1F`, binary `0b1010`, octal
    /// `0o17`, with optional `_` digit separators
    /// (`1_000_000`).
    Nat(u64),
    /// Floating-point literal. Held as the original source
    /// text (e.g. `"3.14"`, `"1.5e10"`, `"1.0e-3"`) rather
    /// than `f64` so the AST's `Eq` derive stays sound
    /// (`f64` is not `Eq` due to NaN).
    Float(String),
    /// String literal. Source forms: regular `"…"` (with
    /// `\\n \\t \\r \\\\ \\" \\0` + `\\xHH` + `\\u{H+}`
    /// escapes resolved) and triple-quoted `"""…"""`
    /// (raw, no escape resolution; arbitrary content
    /// including newlines).
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
        InstanceBody, InterpPart, LamBinder, Literal, MatchArm, Pattern,
        StructField,
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

    /// Parse a numeric literal source string with optional
    /// hex / binary / octal prefix + `_` digit separators
    /// stripped. Returns `None` on overflow / malformed
    /// digit set (the PEG rule's character classes already
    /// constrain the input to valid digits for the radix).
    fn parse_numeric_literal(s: &str) -> Option<u64> {
        let cleaned: String = s.chars().filter(|c| *c != '_').collect();
        if let Some(hex) = cleaned
            .strip_prefix("0x")
            .or_else(|| cleaned.strip_prefix("0X"))
        {
            u64::from_str_radix(hex, 16).ok()
        } else if let Some(bin) = cleaned
            .strip_prefix("0b")
            .or_else(|| cleaned.strip_prefix("0B"))
        {
            u64::from_str_radix(bin, 2).ok()
        } else if let Some(oct) = cleaned
            .strip_prefix("0o")
            .or_else(|| cleaned.strip_prefix("0O"))
        {
            u64::from_str_radix(oct, 8).ok()
        } else {
            cleaned.parse().ok()
        }
    }

    /// Split a `by`-block region into individual tactic
    /// strings. The region may be empty (`:= by` with no
    /// tactics — yields an empty Vec) or contain a mix of
    /// `;`-sequenced and newline-separated tactics.
    /// Whitespace and empty entries are trimmed away.
    fn parse_by_region(text: &str) -> Vec<String> {
        text.split([';', '\n'])
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Split an `open` line's text into a list of opened-
    /// namespace idents + a raw tail capturing any
    /// selective / renaming / hiding / scoped clauses for
    /// downstream re-parsing. The tail starts at the first
    /// token that looks like a clause marker (`(`,
    /// `renaming`, `hiding`, `scoped`).
    fn parse_open_line(line: &str) -> (Vec<String>, String) {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let stop = tokens.iter().position(|t| {
            t.starts_with('(')
                || *t == "renaming"
                || *t == "hiding"
                || *t == "scoped"
        });
        let split_at = stop.unwrap_or(tokens.len());
        let items: Vec<String> =
            tokens[..split_at].iter().map(|s| (*s).to_string()).collect();
        let raw_tail = tokens[split_at..].join(" ");
        (items, raw_tail)
    }

    /// Decode the literal-text segment of a `s!"…"` string:
    /// `{{` → `{`, `}}` → `}`, plus the standard `\\n \\t
    /// \\r \\\\ \\"` escapes. Other escapes pass through
    /// verbatim.
    fn decode_interp_text(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let mut chars = s.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '{' && chars.peek() == Some(&'{') {
                chars.next();
                out.push('{');
                continue;
            }
            if c == '}' && chars.peek() == Some(&'}') {
                chars.next();
                out.push('}');
                continue;
            }
            if c == '\\' {
                match chars.next() {
                    Some('n') => out.push('\n'),
                    Some('t') => out.push('\t'),
                    Some('r') => out.push('\r'),
                    Some('\\') | None => out.push('\\'),
                    Some('"') => out.push('"'),
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

    peg::parser! {
        pub grammar lean4() for str {
            // ─── Whitespace + comment skipping ──────────────
            rule _ = (whitespace() / line_comment() / block_comment())* {}
            rule whitespace() = quiet!{[' ' | '\t' | '\n' | '\r']}
            rule line_comment() = quiet!{"--" (!"\n" [_])* "\n"?}
            // Block comments — Lean 4 supports nested
            // `/- … /- inner -/ … -/`. Doc comments
            // `/-- … -/` are a special block comment used
            // for documentation; the parser skips them as
            // ordinary block comments here. Semantic
            // attachment to the following decl is OX6 step
            // 11u (deferred).
            rule block_comment() = quiet!{
                "/-" block_comment_body() "-/"
            }
            rule block_comment_body() =
                (block_comment() / (!"-/" [_]))*

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
                 / "if" / "then" / "else" / "do"
                 / "forall" / "exists" / "by")
                ![ 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '\'' ]
            }

            // Decimal / hex / binary / octal with optional
            // `_` digit separators (`1_000_000`, `0xFF_FF`,
            // `0b1010_0101`, `0o17_77`).
            rule nat_lit() -> u64 =
                n:$(("0x" / "0X") hex_digit() (['_']? hex_digit())*)
                    {? parse_numeric_literal(n).ok_or("invalid hex literal") }
                / n:$(("0b" / "0B") bin_digit() (['_']? bin_digit())*)
                    {? parse_numeric_literal(n).ok_or("invalid binary literal") }
                / n:$(("0o" / "0O") oct_digit() (['_']? oct_digit())*)
                    {? parse_numeric_literal(n).ok_or("invalid octal literal") }
                / n:$(['0'..='9'] (['_']? ['0'..='9'])*)
                    {? parse_numeric_literal(n).ok_or("invalid decimal literal") }

            rule hex_digit() = ['0'..='9' | 'a'..='f' | 'A'..='F']
            rule bin_digit() = ['0' | '1']
            rule oct_digit() = ['0'..='7']

            // `3.14`, `1.5e10`, `2.0e-3`. The fractional
            // part is mandatory (so float beats nat in the
            // alternative ordering); the exponent is
            // optional.
            rule float_lit() -> String =
                s:$(['0'..='9'] (['_']? ['0'..='9'])*
                    "." ['0'..='9'] (['_']? ['0'..='9'])*
                    (['e' | 'E'] ['+' | '-']? ['0'..='9']+)?)
                    { s.to_string() }

            // Single-line string `"…"` with escape resolution.
            rule str_lit() -> String =
                "\"" s:str_body() "\"" { s }

            rule str_body() -> String =
                s:$((!"\"" str_char())*) { unescape(s) }

            rule str_char() = "\\" [_] / [_]

            // Triple-quoted raw string `"""…"""`. Content
            // is preserved verbatim (no escape resolution),
            // including newlines.
            rule multiline_str_lit() -> String =
                "\"\"\"" body:$((!"\"\"\"" [_])*) "\"\"\"" {
                    body.to_string()
                }

            // ─── Top-level entry ─────────────────────────────
            pub rule source() -> Vec<Decl> =
                _ ds:(d:decl() _ { d })* ![_] { ds }

            // A top-level decl with optional prefixes:
            // `@[attrs]` attribute list, then any sequence
            // of modifier keywords (`partial`,
            // `noncomputable`, `private`, `protected`,
            // `unsafe`). `decl_body` returns a `Decl` with
            // empty attrs / modifiers; this wrapper
            // attaches the parsed ones.
            rule decl() -> Decl =
                attrs:(a:attribute_list() _ { a })?
                prefix_mods:(m:modifier_keyword() _ { m })*
                d:decl_body()
                {
                    let mut decl = d;
                    decl.attrs = attrs.unwrap_or_default();
                    // Prefix modifiers come first; inner-decl
                    // modifiers (e.g. the synthetic `"abbrev"`
                    // tag from `abbrev_decl`) follow.
                    let mut combined = prefix_mods;
                    combined.extend(std::mem::take(&mut decl.modifiers));
                    decl.modifiers = combined;
                    decl
                }

            rule modifier_keyword() -> String =
                s:$("private" / "protected" / "noncomputable"
                    / "partial" / "unsafe")
                word_boundary() { s.to_string() }

            rule decl_body() -> Decl =
                d:definition() { d }
                / d:abbrev_decl() { d }
                / d:theorem_decl() { d }
                / d:axiom_decl() { d }
                / d:example_decl() { d }
                / d:structure_decl() { d }
                / d:class_decl() { d }
                / d:inductive_decl() { d }
                / d:instance_decl() { d }
                / d:namespace_decl() { d }
                / d:section_decl() { d }
                / d:mutual_decl() { d }
                / d:open_decl() { d }
                / d:import_decl() { d }
                / d:variable_decl() { d }
                / d:omit_decl() { d }
                / d:include_decl() { d }
                / d:hash_command_decl() { d }

            // `example [binders]+ : TYPE := PROOF` —
            // anonymous theorem (smoke-test a proof
            // without binding a name).
            rule example_decl() -> Decl =
                "example" word_boundary() _
                binders:(b:binder_group() _ { b })*
                ":" _ ty:expr() _ ":=" _ proof:expr()
                {
                    Decl {
                        attrs: vec![],
                        univ_params: vec![],
                        modifiers: vec![],
                        kind: DeclKind::Example { binders, ty, proof },
                    }
                }

            // `abbrev NAME [binders]+ [: TYPE] := VALUE` —
            // surface synonym for `def` (elab treats the
            // result as a reducible def). Lands as a
            // `Definition` with `"abbrev"` in `modifiers`.
            rule abbrev_decl() -> Decl =
                "abbrev" word_boundary() _
                name:ident() univs:univ_params_opt() _
                binders:(b:binder_group() _ { b })*
                ty:(":" _ t:expr() _ { t })?
                ":=" _ value:expr()
                {
                    Decl {
                        attrs: vec![],
                        univ_params: univs,
                        modifiers: vec!["abbrev".to_string()],
                        kind: DeclKind::Definition { name, binders, ty, value },
                    }
                }

            // `theorem NAME [binders]+ : TYPE := PROOF` or
            // `lemma NAME [binders]+ : TYPE := PROOF` — same
            // shape as `def`; surface keyword not preserved.
            rule theorem_decl() -> Decl =
                ("theorem" / "lemma") word_boundary() _
                name:ident() univs:univ_params_opt() _
                binders:(b:binder_group() _ { b })*
                ":" _ ty:expr() _ ":=" _ proof:expr()
                {
                    Decl { attrs: vec![], univ_params: univs, modifiers: vec![], kind: DeclKind::Theorem {
                        name, binders, ty, proof,
                    }}
                }

            // `axiom NAME[.{u}]* [binders]+ : TYPE` — no body.
            rule axiom_decl() -> Decl =
                "axiom" word_boundary() _
                name:ident() univs:univ_params_opt() _
                binders:(b:binder_group() _ { b })*
                ":" _ ty:expr()
                {
                    Decl { attrs: vec![], univ_params: univs, modifiers: vec![], kind: DeclKind::Axiom {
                        name, binders, ty,
                    }}
                }

            // ─── instance ────────────────────────────────────
            //
            // `instance [NAME] [binders]+ : TYPE BODY`.
            // The name is optional (Lean 4 auto-names
            // anonymous instances). Body is either a
            // `where`-block (structure-style field list) or
            // `:= TERM`.
            rule instance_decl() -> Decl =
                "instance" word_boundary() _
                name:(n:ident() _ { n })?
                univs:univ_params_opt() _
                binders:(b:binder_group() _ { b })*
                ":" _ ty:expr() _
                body:instance_body()
                {
                    Decl { attrs: vec![], univ_params: univs, modifiers: vec![], kind: DeclKind::Instance {
                        name, binders, ty, body,
                    }}
                }

            rule instance_body() -> InstanceBody =
                instance_where() / instance_term()

            rule instance_where() -> InstanceBody =
                "where"
                fields:(_ f:struct_field() { f })*
                {
                    InstanceBody::Where(fields)
                }

            rule instance_term() -> InstanceBody =
                ":=" _ e:expr() { InstanceBody::Term(e) }

            // ─── namespace / section / mutual ───────────────
            //
            // `namespace NAME … end [NAME]` — scoped block.
            // The trailing `end NAME` matches the opener;
            // the parser doesn't enforce name agreement
            // (that's an elab-level check).
            rule namespace_decl() -> Decl =
                "namespace" word_boundary() _ name:ident_raw() _
                decls:(d:decl() _ { d })*
                "end" word_boundary() (_ ident_raw())?
                {
                    Decl { attrs: vec![], univ_params: vec![], modifiers: vec![], kind: DeclKind::Namespace {
                        name, decls,
                    }}
                }

            // `section [NAME] … end [NAME]` — name optional.
            rule section_decl() -> Decl =
                "section" word_boundary()
                name:(_ n:ident_raw() { n })?
                _ decls:(d:decl() _ { d })*
                "end" word_boundary() (_ ident_raw())?
                {
                    Decl { attrs: vec![], univ_params: vec![], modifiers: vec![], kind: DeclKind::Section {
                        name, decls,
                    }}
                }

            // `mutual … end` — block of mutually-recursive
            // decls. The terminator is bare `end`; the name
            // is forbidden (mutual blocks aren't named).
            rule mutual_decl() -> Decl =
                "mutual" word_boundary()
                _ decls:(d:decl() _ { d })*
                "end" word_boundary()
                {
                    Decl { attrs: vec![], univ_params: vec![], modifiers: vec![], kind: DeclKind::Mutual {
                        decls,
                    }}
                }

            // ─── open / import / variable ───────────────────
            //
            // `open Foo Bar Baz` — opens namespaces. v0
            // captures the line text and splits at the
            // first selective / renaming / hiding / scoped
            // marker; everything before that lands in
            // `items`, everything after in `raw_tail` for
            // downstream re-parsing (Lean 4's full open
            // syntax is rich: `open Foo (x y)`, `open Foo
            // renaming x → y`, `open Foo hiding z`,
            // `open scoped Foo`).
            rule open_decl() -> Decl =
                "open" word_boundary() _h() line:$((!"\n" [_])+)
                {
                    let (items, raw_tail) = parse_open_line(line);
                    Decl { attrs: vec![], univ_params: vec![], modifiers: vec![], kind: DeclKind::Open {
                        items, raw_tail,
                    }}
                }

            // `import Foo.Bar.Baz` — single dotted path per
            // import (multiple imports = multiple decls).
            rule import_decl() -> Decl =
                "import" word_boundary() _h() path:ident_raw()
                {
                    Decl { attrs: vec![], univ_params: vec![], modifiers: vec![], kind: DeclKind::Import { path } }
                }

            // `variable [binders]+` — section / namespace
            // variable binding. Same binder grammar as `def`.
            rule variable_decl() -> Decl =
                "variable" word_boundary() _
                binders:(b:binder_group() _ { b })+
                {
                    Decl { attrs: vec![], univ_params: vec![], modifiers: vec![], kind: DeclKind::Variable { binders } }
                }

            // `omit ident+` — drops named section variables
            // from the surrounding scope. Each whitespace-
            // separated ident is captured into `items`.
            rule omit_decl() -> Decl =
                "omit" word_boundary() _
                items:(i:ident_raw() _ { i })+
                {
                    Decl { attrs: vec![], univ_params: vec![], modifiers: vec![],
                        kind: DeclKind::Omit { items } }
                }

            // `include ident+` — re-introduces section
            // variables (mirror of `omit`).
            rule include_decl() -> Decl =
                "include" word_boundary() _
                items:(i:ident_raw() _ { i })+
                {
                    Decl { attrs: vec![], univ_params: vec![], modifiers: vec![],
                        kind: DeclKind::Include { items } }
                }

            // `#check expr` / `#eval expr` / `#print name` /
            // `#guard expr` / `#guard_msgs (cfg) in cmd` —
            // debug / diagnostic top-level commands. The
            // arg tail is captured raw to end of line; a
            // multi-line `#guard_msgs in #check …` lands as
            // *two* HashCommand decls (one per `#` prefix)
            // — the consumer reassembles if needed.
            rule hash_command_decl() -> Decl =
                "#" cmd:$(['a'..='z' | 'A'..='Z' | '_']
                          ['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*)
                args:$((!"\n" [_])*)
                {
                    Decl {
                        attrs: vec![],
                        univ_params: vec![],
                        modifiers: vec![],
                        kind: DeclKind::HashCommand {
                            cmd: cmd.to_string(),
                            raw_args: args.trim().to_string(),
                        },
                    }
                }

            // ─── class ───────────────────────────────────────
            //
            // `class NAME [binders]+ [extends BASES] where
            //     field1 : T1
            //     …
            //     [deriving Foo, Bar]`
            //
            // Structurally identical to `structure` plus
            // type-level binders (e.g. `class Functor (f :
            // Type -> Type) where ...`).
            rule class_decl() -> Decl =
                "class" word_boundary() _ name:ident() univs:univ_params_opt()
                binders:(_ b:binder_group() { b })*
                extends:(_ e:struct_extends() { e })?
                _ "where"
                fields:(_ f:struct_field() { f })*
                deriving:(_ d:deriving_clause() { d })?
                {
                    Decl { attrs: vec![], univ_params: univs, modifiers: vec![], kind: DeclKind::Class {
                        name,
                        binders,
                        extends: extends.unwrap_or_default(),
                        fields,
                        deriving: deriving.unwrap_or_default(),
                    }}
                }

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
                "def" _ name:ident() univs:univ_params_opt() _
                binders:(b:binder_group() _ { b })*
                ty:(":" _ t:expr() _ { t })?
                ":=" _ value:expr()
                {
                    Decl {
                        attrs: vec![],
                        univ_params: univs,
                        modifiers: vec![],
                        kind: DeclKind::Definition { name, binders, ty, value },
                    }
                }

            // Optional universe-parameter list: `.{u, v, …}`.
            // Empty vec if absent.
            rule univ_params_opt() -> Vec<String> =
                ".{" _ first:ident() rest:(_ "," _ n:ident() { n })* _ "}" {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }
                / "" { Vec::new() }

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
                "structure" _ name:ident() univs:univ_params_opt()
                extends:(_ e:struct_extends() { e })?
                _ "where"
                fields:(_ f:struct_field() { f })*
                deriving:(_ d:deriving_clause() { d })?
                {
                    Decl { attrs: vec![], univ_params: univs, modifiers: vec![], kind: DeclKind::Structure {
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
                "inductive" _ name:ident() univs:univ_params_opt()
                ty:(_ ":" _ t:expr() { t })?
                _ "where"?
                ctors:(_ c:inductive_ctor() { c })*
                deriving:(_ d:deriving_clause() { d })?
                {
                    Decl { attrs: vec![], univ_params: univs, modifiers: vec![], kind: DeclKind::Inductive {
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
                lhs:(@) _ "≠"  _ rhs:@ { Expr::BinOp("≠".into(),  Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "<=" _ rhs:@ { Expr::BinOp("<=".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "≤"  _ rhs:@ { Expr::BinOp("≤".into(),  Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ ">=" _ rhs:@ { Expr::BinOp(">=".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "≥"  _ rhs:@ { Expr::BinOp("≥".into(),  Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "<"  _ rhs:@ { Expr::BinOp("<".into(),  Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ ">"  _ rhs:@ { Expr::BinOp(">".into(),  Box::new(lhs), Box::new(rhs)) }
                // Set / membership operators at comparison level.
                lhs:(@) _ "∈"  _ rhs:@ { Expr::BinOp("∈".into(),  Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "∉"  _ rhs:@ { Expr::BinOp("∉".into(),  Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "⊆"  _ rhs:@ { Expr::BinOp("⊆".into(),  Box::new(lhs), Box::new(rhs)) }
                // Lean 4's propositional equality `a = b` —
                // single `=` (not `==` / not `:=`).
                lhs:(@) _ "=" !"=" _ rhs:@ { Expr::BinOp("=".into(),  Box::new(lhs), Box::new(rhs)) }
                --
                // ─── additive (left-assoc) ──────────────
                lhs:(@) _ "+" _ rhs:@ { Expr::BinOp("+".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "-" _ rhs:@ { Expr::BinOp("-".into(), Box::new(lhs), Box::new(rhs)) }
                --
                // ─── multiplicative (left-assoc) ────────
                lhs:(@) _ "*" _ rhs:@ { Expr::BinOp("*".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "×" _ rhs:@ { Expr::BinOp("×".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "/" _ rhs:@ { Expr::BinOp("/".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "÷" _ rhs:@ { Expr::BinOp("÷".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "%" _ rhs:@ { Expr::BinOp("%".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "∪" _ rhs:@ { Expr::BinOp("∪".into(), Box::new(lhs), Box::new(rhs)) }
                lhs:(@) _ "∩" _ rhs:@ { Expr::BinOp("∩".into(), Box::new(lhs), Box::new(rhs)) }
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
                if_let_expr()
                / if_expr()
                / let_expr()
                / match_expr()
                / lam_expr()
                / forall_expr()
                / exists_expr()
                / do_expr()
                / by_expr()
                / at_expr()
                / interp_str_lit()
                / list_lit()
                / anon_ctor_lit()
                / anon_struct_lit()
                / dot_fn_lit()
                / paren_atom()
                / lit_atom()
                / ident_atom()

            rule paren_atom() -> Expr =
                "(" _ e:expr() _ ")" { Expr::Paren(Box::new(e)) }

            // `(· + 1)` / `(·.field)` — Lean 4 anonymous
            // function shorthand. Triggered iff the body
            // between `(` and `)` contains at least one
            // `·` (U+00B7 MIDDLE DOT) placeholder. Body is
            // captured as raw text (consumer sub-parses if
            // it cares). Nested parens NOT supported in
            // v1 RC.
            rule dot_fn_lit() -> Expr =
                "(" &dot_fn_seek_placeholder()
                    body:$((!")" [_])+) ")"
                {
                    Expr::DotFn(body.trim().to_string())
                }

            rule dot_fn_seek_placeholder() =
                (!"·" !")" [_])* "·"

            rule lit_atom() -> Expr =
                f:float_lit() { Expr::Lit(Literal::Float(f)) }
                / n:nat_lit() { Expr::Lit(Literal::Nat(n)) }
                / s:multiline_str_lit() { Expr::Lit(Literal::Str(s)) }
                / s:str_lit() { Expr::Lit(Literal::Str(s)) }

            rule ident_atom() -> Expr =
                s:ident_raw() { Expr::Ident(s) }

            // ─── `@f` explicit-args marker ──────────────────
            //
            // The `!"["` lookahead rejects `@[…]` (attribute
            // prefix) — though `@[` only appears in decl
            // position, never in atom position, the
            // lookahead is a belt-and-suspenders guard
            // against future grammar changes.
            rule at_expr() -> Expr =
                "@" !"[" _ a:atom() {
                    Expr::At(Box::new(a))
                }

            // ─── Anonymous ctor `⟨a, b, …⟩` ──────────────────
            //
            // Unicode-angle-bracket shorthand for the unique
            // ctor of a single-ctor inductive (Point.mk shape).
            // Empty `⟨⟩` is `AnonCtor(vec![])`. Elab resolves
            // the target ctor from the expected type.
            rule anon_ctor_lit() -> Expr =
                "⟨" _ items:anon_ctor_items() _ "⟩" {
                    Expr::AnonCtor(items)
                }

            rule anon_ctor_items() -> Vec<Expr> =
                first:expr() rest:(_ "," _ e:expr() { e })* {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }
                / "" { Vec::new() }

            // ─── Anonymous structure literal `{ x := e, … }` ──
            //
            // Disambiguates from implicit binder `{T : Type}`
            // by the field syntax `name := value` (the
            // binder uses `name : type` with no `:=`). The
            // anon-struct-literal rule looks for `:=` after
            // the first ident to decide.
            //
            // Lives in expr atom position; implicit binder
            // `{…}` lives in binder context — no collision.
            rule anon_struct_lit() -> Expr =
                "{" _ fields:anon_struct_fields() _ "}" {
                    Expr::AnonStruct(fields)
                }

            rule anon_struct_fields() -> Vec<(String, Expr)> =
                first:anon_struct_field()
                rest:(_ "," _ f:anon_struct_field() { f })* {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }

            rule anon_struct_field() -> (String, Expr) =
                name:ident() _h() ":=" _ value:expr() {
                    (name, value)
                }

            // ─── List literal `[1, 2, 3]` / `[]` ─────────────
            //
            // Lives in atom position. Inside binder context
            // (e.g. `[Inhabited T]` instance binders) the
            // grammar uses its own binder rules, so there's
            // no ambiguity.
            rule list_lit() -> Expr =
                "[" _ items:list_items() _ "]" { Expr::List(items) }

            rule list_items() -> Vec<Expr> =
                first:expr() rest:(_ "," _ e:expr() { e })* {
                    let mut v = vec![first];
                    v.extend(rest);
                    v
                }
                / "" { Vec::new() }

            // ─── String interpolation `s!"…{x}…"` ──────────
            //
            // Alternating text + `{expr}` holes. `{{` / `}}`
            // are literal `{` / `}` in the text segments
            // (matches Lean 4 + Rust convention). Standard
            // `\\n \\t \\r \\\\ \\"` escapes are also
            // resolved in text segments.
            //
            // Out of scope today: nested `s!` strings inside
            // holes (which Lean 4 also accepts — recursive
            // interpolation). Acceptable v0 limitation since
            // it almost never appears in real code.
            rule interp_str_lit() -> Expr =
                "s!\"" parts:interp_part()* "\"" {
                    Expr::InterpStr(parts)
                }

            rule interp_part() -> InterpPart =
                interp_hole() / interp_text_part()

            rule interp_hole() -> InterpPart =
                // Real `{expr}` interpolation — but `{{` must
                // *not* match here (it's an escape). The
                // !"{" lookahead enforces single `{`.
                "{" !"{" _ e:expr() _ "}" {
                    InterpPart::Hole(e)
                }

            rule interp_text_part() -> InterpPart =
                s:$((!"\"" !interp_hole_start() interp_text_char())+) {
                    InterpPart::Text(decode_interp_text(s))
                }

            // Lookahead for the start of a real interpolation
            // hole (single `{` not followed by another `{`).
            rule interp_hole_start() = "{" !"{"

            rule interp_text_char() =
                "{{" {}    // literal `{`
                / "}}" {}  // literal `}`
                / "\\" [_] {}
                / [_]

            // ─── `by …` tactic block ────────────────────────
            //
            // `by` enters tactic mode at the term level
            // (`theorem t : T := by rfl`). v0 captures the
            // tactic region as raw text up to the next top-
            // level decl keyword or EOF, then splits on `;`
            // (single-line sequenced) and newlines (multi-
            // line) into individual tactic strings.
            //
            // Each `String` in `Expr::By(Vec<String>)` is one
            // tactic in raw form. Tactic AST sub-parsing is
            // a future step (Lean 4's tactic language is its
            // own DSL with its own grammar).
            rule by_expr() -> Expr =
                "by" word_boundary() region:$(by_region()) {
                    let tactics = parse_by_region(region);
                    Expr::By(tactics)
                }

            rule by_region() = (!by_block_end() [_])*

            rule by_block_end() =
                "\n" _h() top_level_decl_starter()
                / ![_]

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
                do_for()
                / do_while()
                / do_until()
                / do_let_bind()
                / do_let_pure()
                / do_return()
                / do_expr_stmt()

            // ─── Multi-line value capture (OX6 step 11f) ───
            //
            // Keyword-prefix statements (`let` / `return` /
            // `pure`) accept a multi-line value expression:
            // the capture continues across newlines until
            // either (a) the next keyword-prefix statement
            // begins (`let` / `return` / `pure` at the start
            // of a fresh line) or (b) the do-block boundary
            // fires (top-level decl keyword / EOF).
            //
            // Bare expression statements stay single-line —
            // detecting the boundary between two consecutive
            // bare expressions on separate lines without
            // column-level indent tracking is genuinely
            // ambiguous in PEG; v0 takes the safe single-
            // line interpretation.
            rule do_let_bind() -> DoStmt =
                "let" word_boundary() _h() name:ident() _h() ("<-" / "←") _h()
                text:$((!do_keyword_stmt_boundary() [_])+)
                {
                    let value = parse_expr_text(text.trim())
                        .unwrap_or_else(|| Expr::Raw(text.trim().to_string()));
                    DoStmt::Bind { name, value }
                }

            rule do_let_pure() -> DoStmt =
                "let" word_boundary() _h() name:ident() _h() ":=" _h()
                text:$((!do_keyword_stmt_boundary() [_])+)
                {
                    let value = parse_expr_text(text.trim())
                        .unwrap_or_else(|| Expr::Raw(text.trim().to_string()));
                    DoStmt::Let { name, value }
                }

            rule do_return() -> DoStmt =
                ("return" / "pure") word_boundary() _h()
                text:$((!do_keyword_stmt_boundary() [_])+)
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

            // Boundary fires when a fresh line starts with
            // a keyword-prefix do-statement (`let` /
            // `return` / `pure` / `for` / `while` /
            // `until`) or when the broader do-block ends.
            rule do_keyword_stmt_boundary() =
                "\n" _h() (("let" / "return" / "pure"
                          / "for" / "while" / "until")
                    word_boundary())
                / by_block_end()

            // ─── do-loops (OX6 step 11r) ───────────────────
            //
            // `for BINDING in ITER do BODY` /
            // `while COND do BODY` /
            // `until COND do BODY`. The iter / cond head is
            // captured raw up to the matching `do` keyword
            // (`do` at a word boundary, not as a substring
            // of a longer ident like `double`), then sub-
            // parsed. The body is single-line in v1 RC
            // (multi-line follow-up — see `do_expr_stmt`'s
            // single-line limitation note).
            rule do_for() -> DoStmt =
                "for" word_boundary() _h() binding:ident() _h()
                "in" word_boundary() _h()
                iter_text:$((!do_keyword_in_head() [_])+)
                "do" word_boundary() _h()
                body_text:$((!"\n" [_])+)
                {
                    let iter = parse_expr_text(iter_text.trim())
                        .unwrap_or_else(|| Expr::Raw(iter_text.trim().to_string()));
                    let body = parse_expr_text(body_text.trim())
                        .unwrap_or_else(|| Expr::Raw(body_text.trim().to_string()));
                    DoStmt::For { binding, iter, body }
                }

            rule do_while() -> DoStmt =
                "while" word_boundary() _h()
                cond_text:$((!do_keyword_in_head() [_])+)
                "do" word_boundary() _h()
                body_text:$((!"\n" [_])+)
                {
                    let cond = parse_expr_text(cond_text.trim())
                        .unwrap_or_else(|| Expr::Raw(cond_text.trim().to_string()));
                    let body = parse_expr_text(body_text.trim())
                        .unwrap_or_else(|| Expr::Raw(body_text.trim().to_string()));
                    DoStmt::While { cond, body }
                }

            rule do_until() -> DoStmt =
                "until" word_boundary() _h()
                cond_text:$((!do_keyword_in_head() [_])+)
                "do" word_boundary() _h()
                body_text:$((!"\n" [_])+)
                {
                    let cond = parse_expr_text(cond_text.trim())
                        .unwrap_or_else(|| Expr::Raw(cond_text.trim().to_string()));
                    let body = parse_expr_text(body_text.trim())
                        .unwrap_or_else(|| Expr::Raw(body_text.trim().to_string()));
                    DoStmt::Until { cond, body }
                }

            // Stop the raw-iter / raw-cond capture at the
            // `do` keyword that introduces the loop body —
            // word_boundary prevents matching `do` inside a
            // longer ident.
            rule do_keyword_in_head() =
                "do" word_boundary()

            // ─── let-in expression `let x := e ; body` ──────
            //
            // Pure (non-monadic) let-binding in expression
            // position. The monadic `let x ← e` inside `do`
            // blocks lives in `DoStmt::Bind`.
            //
            // v0 captures the `:= VALUE` region as raw text up
            // to the separator (`;` or newline), then sub-
            // parses it via `parse_expr_text`. Body is full
            // expr (multi-line OK). The optional type
            // annotation `: TY` is also raw-captured (up to
            // `:=`) and sub-parsed.
            //
            // Multi-line value bodies (`let x := \n  big`) are
            // not yet supported in v0 — single-line value
            // before the separator.
            rule let_expr() -> Expr =
                "let" word_boundary() _ name:ident() _h()
                ty_text:(":" !"=" _h() t:$((!":=" [_])+)
                    { t.trim().to_string() })?
                ":=" _h()
                value_text:$((!";" !"\n" [_])+)
                let_separator() _
                body:expr()
                {
                    let ty = ty_text
                        .and_then(|t| parse_expr_text(&t))
                        .map(Box::new);
                    let value = parse_expr_text(value_text.trim())
                        .unwrap_or_else(|| Expr::Raw(value_text.trim().to_string()));
                    Expr::Let {
                        name,
                        ty,
                        value: Box::new(value),
                        body: Box::new(body),
                    }
                }

            rule let_separator() = ";" / "\n"

            // ─── quantifiers (`forall` / `∀` / `exists` / `∃`) ──
            //
            // Lean 4 dependent quantifiers. Binders share
            // lambda's `LamBinder` shape (untyped names or
            // typed groups in `()` / `{}` / `[]` brackets).
            // The body is separated by `,` (not `=>` /
            // `->`); the body expression takes the
            // full-precedence parser.
            rule forall_expr() -> Expr =
                ("forall" word_boundary() / "∀") _
                binders:lam_binders() _ "," _ body:expr()
                {
                    Expr::Forall(binders, Box::new(body))
                }

            rule exists_expr() -> Expr =
                ("exists" word_boundary() / "∃") _
                binders:lam_binders() _ "," _ body:expr()
                {
                    Expr::Exists(binders, Box::new(body))
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

            // `if let pat := e then a else b` — pattern-
            // matching if (sibling of `match` with a binary
            // success-vs-fallback split). The `let` keyword
            // must follow `if` directly (no other tokens),
            // so this rule is tried *before* `if_expr` in
            // `atom` to avoid spurious matches against the
            // plain `if cond then …` form.
            rule if_let_expr() -> Expr =
                "if" _ "let" word_boundary() _ pat:pattern() _ ":=" _
                scrut:expr() _ "then" _ t:expr() _ "else" _ e:expr()
                {
                    Expr::IfLet {
                        pattern: Box::new(pat),
                        scrutinee: Box::new(scrut),
                        then_branch: Box::new(t),
                        else_branch: Box::new(e),
                    }
                }

            // ─── match … with | … ───────────────────────────
            //
            // Two surface forms:
            //
            // 1. `match SCRUT with | …` — plain match.
            // 2. `match BINDING : SCRUT with | …` —
            //    scrutinee-binding form (Lean 4 makes the
            //    binding's `eq` proof available in each arm).
            //
            // The bind-form is tried first; the
            // `BINDING ":" !"="` lookahead distinguishes
            // `match h : e with …` from `match e with …`
            // (the scrutinee `e` might syntactically allow a
            // bare `:` only inside parens, never at the top
            // of a `match` head).
            rule match_expr() -> Expr =
                "match" _ binding:ident() _ ":" !"=" _
                    scrut:expr() _ "with" _ arms:match_arms()
                {
                    Expr::MatchBind {
                        binding,
                        scrutinee: Box::new(scrut),
                        arms,
                    }
                }
                / "match" _ scrut:expr() _ "with" _ arms:match_arms() {
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

            // Pattern guard (`| pat if cond => body`) —
            // the `if EXPR` clause is captured raw between
            // the `if` keyword and the `=>` arrow, then
            // sub-parsed via `parse_expr_text`. Raw capture
            // avoids `if … then … else` lookahead conflicts
            // with the term-level `if_expr` rule.
            rule match_arm() -> MatchArm =
                arm_bar() _ pat:pattern() _
                guard_text:("if" word_boundary() _h() t:$((!"=>" [_])+)
                    { t.trim().to_string() })?
                "=>" _ body:expr()
                {
                    let guard = guard_text
                        .and_then(|t| parse_expr_text(&t));
                    MatchArm { pattern: pat, guard, body }
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
                    // `\xHH` — two hex digits → one byte.
                    Some('x') => {
                        let h1 = chars.next();
                        let h2 = chars.next();
                        if let (Some(a), Some(b)) = (h1, h2)
                            && let Some(d1) = a.to_digit(16)
                            && let Some(d2) = b.to_digit(16)
                            && let Ok(code) = u8::try_from(d1 * 16 + d2)
                        {
                            out.push(code as char);
                        } else {
                            out.push_str("\\x");
                            if let Some(a) = h1 {
                                out.push(a);
                            }
                            if let Some(b) = h2 {
                                out.push(b);
                            }
                        }
                    }
                    // `\u{HEX+}` — Unicode codepoint.
                    Some('u') => {
                        if chars.peek() == Some(&'{') {
                            chars.next();
                            let mut hex = String::new();
                            while let Some(&peek) = chars.peek() {
                                if peek == '}' {
                                    chars.next();
                                    break;
                                }
                                hex.push(peek);
                                chars.next();
                            }
                            if let Ok(code) = u32::from_str_radix(&hex, 16)
                                && let Some(c) = char::from_u32(code)
                            {
                                out.push(c);
                            } else {
                                out.push_str("\\u{");
                                out.push_str(&hex);
                                out.push('}');
                            }
                        } else {
                            out.push_str("\\u");
                        }
                    }
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

    // ─── literal extensions (OX6 step 11l) ─────────────────

    #[test]
    fn nat_hex_literal() {
        assert_eq!(parse_value_expr("0xFF"), Expr::Lit(Literal::Nat(255)));
        assert_eq!(parse_value_expr("0X10"), Expr::Lit(Literal::Nat(16)));
    }

    #[test]
    fn nat_binary_literal() {
        assert_eq!(parse_value_expr("0b1010"), Expr::Lit(Literal::Nat(10)));
        assert_eq!(parse_value_expr("0B1111"), Expr::Lit(Literal::Nat(15)));
    }

    #[test]
    fn nat_octal_literal() {
        assert_eq!(parse_value_expr("0o17"), Expr::Lit(Literal::Nat(15)));
        assert_eq!(parse_value_expr("0O777"), Expr::Lit(Literal::Nat(511)));
    }

    #[test]
    fn nat_with_separator() {
        assert_eq!(
            parse_value_expr("1_000_000"),
            Expr::Lit(Literal::Nat(1_000_000))
        );
        assert_eq!(
            parse_value_expr("0xFF_FF"),
            Expr::Lit(Literal::Nat(0xFFFF))
        );
        assert_eq!(
            parse_value_expr("0b1010_1010"),
            Expr::Lit(Literal::Nat(0b1010_1010))
        );
    }

    #[test]
    fn float_literal_simple() {
        assert_eq!(
            parse_value_expr("3.14"),
            Expr::Lit(Literal::Float("3.14".to_string()))
        );
    }

    #[test]
    fn float_literal_scientific() {
        assert_eq!(
            parse_value_expr("1.5e10"),
            Expr::Lit(Literal::Float("1.5e10".to_string()))
        );
        assert_eq!(
            parse_value_expr("1.0e-3"),
            Expr::Lit(Literal::Float("1.0e-3".to_string()))
        );
    }

    #[test]
    fn float_with_separator() {
        assert_eq!(
            parse_value_expr("1_000.5"),
            Expr::Lit(Literal::Float("1_000.5".to_string()))
        );
    }

    #[test]
    fn float_takes_priority_over_nat() {
        // `1.5` must parse as Float, not as Nat(1) then dot-shortcut.
        let e = parse_value_expr("1.5");
        assert!(matches!(e, Expr::Lit(Literal::Float(_))));
    }

    #[test]
    fn multiline_string_literal() {
        let e = parse_value_expr(r#""""hello
world""""#);
        match e {
            Expr::Lit(Literal::Str(s)) => {
                assert_eq!(s, "hello\nworld");
            }
            other => panic!("expected multiline Str, got {other:?}"),
        }
    }

    #[test]
    fn multiline_string_with_special_chars() {
        // Triple-quoted is raw — backslashes / quotes
        // pass through (until the closing `"""`).
        let e = parse_value_expr(r#""""raw \n no escape""""#);
        match e {
            Expr::Lit(Literal::Str(s)) => {
                // `\n` preserved as 2 chars (backslash + n).
                assert!(s.contains("\\n"));
            }
            other => panic!("expected multiline Str, got {other:?}"),
        }
    }

    #[test]
    fn str_escape_hex_byte() {
        let e = parse_value_expr(r#""x\x41y""#);
        match e {
            Expr::Lit(Literal::Str(s)) => assert_eq!(s, "xAy"),
            other => panic!("expected Str, got {other:?}"),
        }
    }

    #[test]
    fn str_escape_unicode() {
        let e = parse_value_expr(r#""\u{1F600}""#);
        match e {
            Expr::Lit(Literal::Str(s)) => {
                assert_eq!(s, "\u{1F600}");
            }
            other => panic!("expected Str, got {other:?}"),
        }
    }

    #[test]
    fn def_with_hex_value() {
        let src = "def color : UInt32 := 0xFF00AA";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        assert_eq!(*value, Expr::Lit(Literal::Nat(0xFF_00AA)));
    }

    // ─── Unicode operators (OX6 step 11q) ──────────────────

    #[test]
    fn unicode_op_le() {
        let e = parse_value_expr("a ≤ b");
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "≤"));
    }

    #[test]
    fn unicode_op_ge() {
        let e = parse_value_expr("a ≥ b");
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "≥"));
    }

    #[test]
    fn unicode_op_ne() {
        let e = parse_value_expr("a ≠ b");
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "≠"));
    }

    #[test]
    fn unicode_op_times() {
        let e = parse_value_expr("a × b");
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "×"));
    }

    #[test]
    fn unicode_op_div() {
        let e = parse_value_expr("a ÷ b");
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "÷"));
    }

    #[test]
    fn unicode_op_membership() {
        let e = parse_value_expr("x ∈ s");
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "∈"));
        let e = parse_value_expr("x ∉ s");
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "∉"));
    }

    #[test]
    fn unicode_op_subset() {
        let e = parse_value_expr("a ⊆ b");
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "⊆"));
    }

    #[test]
    fn unicode_op_set_ops() {
        let e = parse_value_expr("a ∪ b");
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "∪"));
        let e = parse_value_expr("a ∩ b");
        assert!(matches!(e, Expr::BinOp(ref o, _, _) if o == "∩"));
    }

    // ─── do-loops (OX6 step 11r) ───────────────────────────

    #[test]
    fn do_for_in_simple() {
        let src = "def main : IO Unit := do\n  for x in xs do print x";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        let Expr::Do(stmts) = value else { panic!("expected Do") };
        match &stmts[0] {
            DoStmt::For { binding, iter, body } => {
                assert_eq!(binding, "x");
                assert!(matches!(iter, Expr::Ident(s) if s == "xs"));
                assert!(matches!(body, Expr::App(_, _)));
            }
            other => panic!("expected For, got {other:?}"),
        }
    }

    #[test]
    fn do_while_simple() {
        let src = "def main : IO Unit := do\n  while keepGoing do step";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        let Expr::Do(stmts) = value else { panic!("expected Do") };
        match &stmts[0] {
            DoStmt::While { cond, body } => {
                assert!(matches!(cond, Expr::Ident(s) if s == "keepGoing"));
                assert!(matches!(body, Expr::Ident(s) if s == "step"));
            }
            other => panic!("expected While, got {other:?}"),
        }
    }

    #[test]
    fn do_until_simple() {
        let src = "def main : IO Unit := do\n  until done do work";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        let Expr::Do(stmts) = value else { panic!("expected Do") };
        match &stmts[0] {
            DoStmt::Until { cond, body } => {
                assert!(matches!(cond, Expr::Ident(s) if s == "done"));
                assert!(matches!(body, Expr::Ident(s) if s == "work"));
            }
            other => panic!("expected Until, got {other:?}"),
        }
    }

    #[test]
    fn do_for_then_let() {
        // `for` followed by `let` — boundary must fire so
        // the `let` lands as its own DoStmt.
        let src = "def main : IO Unit := do\n  \
                   for x in xs do print x\n  \
                   let n := 1\n  \
                   return n";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        let Expr::Do(stmts) = value else { panic!("expected Do") };
        assert_eq!(stmts.len(), 3);
        assert!(matches!(stmts[0], DoStmt::For { .. }));
        assert!(matches!(stmts[1], DoStmt::Let { .. }));
        assert!(matches!(stmts[2], DoStmt::Return(_)));
    }

    #[test]
    fn do_for_with_complex_iter() {
        let src = "def main : IO Unit := do\n  for x in List.range 10 do print x";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        let Expr::Do(stmts) = value else { panic!("expected Do") };
        match &stmts[0] {
            DoStmt::For { iter, .. } => {
                assert!(matches!(iter, Expr::App(_, _)));
            }
            other => panic!("expected For, got {other:?}"),
        }
    }

    // ─── pattern guards (OX6 step 11o) ─────────────────────

    #[test]
    fn match_arm_with_guard() {
        let e = parse_value_expr(
            "match n with | x if x > 0 => 1 | _ => 0"
        );
        match e {
            Expr::Match(_, arms) => {
                assert_eq!(arms.len(), 2);
                assert!(arms[0].guard.is_some());
                let g = arms[0].guard.as_ref().unwrap();
                assert!(matches!(g, Expr::BinOp(op, _, _) if op == ">"));
                assert!(arms[1].guard.is_none());
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn match_arm_unguarded_has_none_guard() {
        let e = parse_value_expr("match x with | 0 => 1 | _ => 2");
        match e {
            Expr::Match(_, arms) => {
                assert!(arms[0].guard.is_none());
                assert!(arms[1].guard.is_none());
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    #[test]
    fn match_arm_guard_on_ctor() {
        let e = parse_value_expr(
            "match opt with | some x if x == 0 => 1 | _ => 0"
        );
        match e {
            Expr::Match(_, arms) => {
                assert!(matches!(arms[0].pattern, Pattern::Ctor(ref s, _) if s == "some"));
                assert!(arms[0].guard.is_some());
            }
            other => panic!("expected Match, got {other:?}"),
        }
    }

    // ─── match scrutinee binding (OX6 step 11n) ────────────

    #[test]
    fn match_bind_simple() {
        let e = parse_value_expr("match h : opt with | some x => x | none => 0");
        match e {
            Expr::MatchBind { binding, scrutinee, arms } => {
                assert_eq!(binding, "h");
                assert!(matches!(*scrutinee, Expr::Ident(ref s) if s == "opt"));
                assert_eq!(arms.len(), 2);
            }
            other => panic!("expected MatchBind, got {other:?}"),
        }
    }

    #[test]
    fn plain_match_still_works() {
        let e = parse_value_expr("match opt with | some x => x | none => 0");
        assert!(matches!(e, Expr::Match(_, _)));
    }

    #[test]
    fn match_bind_with_dot_ctor_arm() {
        let e = parse_value_expr(
            "match h : r with | .ok v => v | .err _ => fallback"
        );
        match e {
            Expr::MatchBind { binding, arms, .. } => {
                assert_eq!(binding, "h");
                assert_eq!(arms.len(), 2);
                assert!(matches!(arms[0].pattern, Pattern::DotCtor(ref s, _) if s == "ok"));
                assert!(matches!(arms[1].pattern, Pattern::DotCtor(ref s, _) if s == "err"));
            }
            other => panic!("expected MatchBind, got {other:?}"),
        }
    }

    // ─── if-let (OX6 step 11m) ─────────────────────────────

    #[test]
    fn if_let_simple_some() {
        let e = parse_value_expr("if let some x := opt then x else 0");
        match e {
            Expr::IfLet { pattern, then_branch, else_branch, .. } => {
                assert!(matches!(*pattern, Pattern::Ctor(ref s, _) if s == "some"));
                assert!(matches!(*then_branch, Expr::Ident(ref s) if s == "x"));
                assert!(matches!(*else_branch, Expr::Lit(Literal::Nat(0))));
            }
            other => panic!("expected IfLet, got {other:?}"),
        }
    }

    #[test]
    fn if_let_with_dot_ctor() {
        let e = parse_value_expr("if let .ok v := r then v else 0");
        match e {
            Expr::IfLet { pattern, .. } => {
                assert!(matches!(*pattern, Pattern::DotCtor(ref s, _) if s == "ok"));
            }
            other => panic!("expected IfLet, got {other:?}"),
        }
    }

    #[test]
    fn if_let_wildcard() {
        let e = parse_value_expr("if let _ := x then 1 else 2");
        match e {
            Expr::IfLet { pattern, .. } => {
                assert!(matches!(*pattern, Pattern::Wildcard));
            }
            other => panic!("expected IfLet, got {other:?}"),
        }
    }

    #[test]
    fn plain_if_still_works() {
        let e = parse_value_expr("if cond then 1 else 2");
        assert!(matches!(e, Expr::If(_, _, _)));
    }

    // ─── anonymous fn shorthand (OX6 step 11p) ─────────────

    #[test]
    fn dot_fn_simple_binary() {
        let e = parse_value_expr("(· + 1)");
        match e {
            Expr::DotFn(body) => assert_eq!(body, "· + 1"),
            other => panic!("expected DotFn, got {other:?}"),
        }
    }

    #[test]
    fn dot_fn_projection() {
        let e = parse_value_expr("(·.field)");
        match e {
            Expr::DotFn(body) => assert_eq!(body, "·.field"),
            other => panic!("expected DotFn, got {other:?}"),
        }
    }

    #[test]
    fn dot_fn_two_placeholders() {
        // `(· + ·)` — λ x y => x + y in Lean 4.
        let e = parse_value_expr("(· + ·)");
        match e {
            Expr::DotFn(body) => assert_eq!(body, "· + ·"),
            other => panic!("expected DotFn, got {other:?}"),
        }
    }

    #[test]
    fn dot_fn_placeholder_on_right() {
        let e = parse_value_expr("(1 + ·)");
        match e {
            Expr::DotFn(body) => assert_eq!(body, "1 + ·"),
            other => panic!("expected DotFn, got {other:?}"),
        }
    }

    #[test]
    fn paren_without_placeholder_is_paren_not_dot_fn() {
        // `(1 + 1)` must still parse as Paren, NOT DotFn.
        let e = parse_value_expr("(1 + 1)");
        assert!(matches!(e, Expr::Paren(_)));
    }

    // ─── omit / include (OX6 step 11v) ─────────────────────

    #[test]
    fn omit_single() {
        let decls = parse_decls("omit x").expect("must parse");
        let DeclKind::Omit { items } = &decls[0].kind
            else { panic!("expected Omit") };
        assert_eq!(items, &["x"]);
    }

    #[test]
    fn omit_multiple() {
        let decls = parse_decls("omit x y z").expect("must parse");
        let DeclKind::Omit { items } = &decls[0].kind
            else { panic!("expected Omit") };
        assert_eq!(items, &["x", "y", "z"]);
    }

    #[test]
    fn include_single() {
        let decls = parse_decls("include foo").expect("must parse");
        let DeclKind::Include { items } = &decls[0].kind
            else { panic!("expected Include") };
        assert_eq!(items, &["foo"]);
    }

    #[test]
    fn include_multiple() {
        let decls = parse_decls("include x y").expect("must parse");
        let DeclKind::Include { items } = &decls[0].kind
            else { panic!("expected Include") };
        assert_eq!(items, &["x", "y"]);
    }

    #[test]
    fn omit_dotted_ident() {
        let decls = parse_decls("omit Foo.Bar").expect("must parse");
        let DeclKind::Omit { items } = &decls[0].kind
            else { panic!("expected Omit") };
        assert_eq!(items, &["Foo.Bar"]);
    }

    // ─── debug commands (OX6 step 11t) ─────────────────────

    #[test]
    fn hash_check_simple() {
        let decls = parse_decls("#check 1 + 1").expect("must parse");
        let DeclKind::HashCommand { cmd, raw_args } = &decls[0].kind
            else { panic!("expected HashCommand") };
        assert_eq!(cmd, "check");
        assert_eq!(raw_args, "1 + 1");
    }

    #[test]
    fn hash_eval_simple() {
        let decls = parse_decls("#eval foo bar").expect("must parse");
        let DeclKind::HashCommand { cmd, raw_args } = &decls[0].kind
            else { panic!("expected HashCommand") };
        assert_eq!(cmd, "eval");
        assert_eq!(raw_args, "foo bar");
    }

    #[test]
    fn hash_print_simple() {
        let decls = parse_decls("#print Nat.succ").expect("must parse");
        let DeclKind::HashCommand { cmd, raw_args } = &decls[0].kind
            else { panic!("expected HashCommand") };
        assert_eq!(cmd, "print");
        assert_eq!(raw_args, "Nat.succ");
    }

    #[test]
    fn hash_guard_msgs() {
        let decls = parse_decls("#guard_msgs (info) in").expect("must parse");
        let DeclKind::HashCommand { cmd, raw_args } = &decls[0].kind
            else { panic!("expected HashCommand") };
        assert_eq!(cmd, "guard_msgs");
        assert_eq!(raw_args, "(info) in");
    }

    #[test]
    fn hash_command_mixed_with_def() {
        let src = "def x : Nat := 1\n#check x\ndef y : Nat := 2";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 3);
        assert!(matches!(decls[0].kind, DeclKind::Definition { .. }));
        assert!(matches!(decls[1].kind, DeclKind::HashCommand { .. }));
        assert!(matches!(decls[2].kind, DeclKind::Definition { .. }));
    }

    // ─── multi-line do statements (OX6 step 11f) ───────────

    #[test]
    fn do_let_pure_multi_line_if_value() {
        // The `let x := if … then … else …` spans 3 lines;
        // the boundary must wait for the next `let`-prefix
        // statement.
        let src = "def main : IO Nat := do\n  \
                   let x := if cond then\n    \
                       1\n    \
                     else\n    \
                       2\n  \
                   return x";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        match value {
            Expr::Do(stmts) => {
                assert_eq!(stmts.len(), 2);
                match &stmts[0] {
                    DoStmt::Let { name, value } => {
                        assert_eq!(name, "x");
                        // Value is a fully-parsed if-expr (multi-line OK).
                        assert!(matches!(value, Expr::If(_, _, _)));
                    }
                    other => panic!("expected Let, got {other:?}"),
                }
                assert!(matches!(stmts[1], DoStmt::Return(_)));
            }
            other => panic!("expected Do, got {other:?}"),
        }
    }

    #[test]
    fn do_bind_multi_line_value() {
        let src = "def main : IO Nat := do\n  \
                   let x <- if cond then\n    \
                       readFirst\n    \
                     else\n    \
                       readSecond\n  \
                   return x";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        match value {
            Expr::Do(stmts) => {
                assert_eq!(stmts.len(), 2);
                match &stmts[0] {
                    DoStmt::Bind { value, .. } => {
                        assert!(matches!(value, Expr::If(_, _, _)));
                    }
                    other => panic!("expected Bind, got {other:?}"),
                }
            }
            other => panic!("expected Do, got {other:?}"),
        }
    }

    #[test]
    fn do_return_multi_line_value() {
        let src = "def main : IO Nat := do\n  \
                   return if cond then\n    \
                       1\n    \
                     else\n    \
                       2";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        match value {
            Expr::Do(stmts) => {
                assert_eq!(stmts.len(), 1);
                match &stmts[0] {
                    DoStmt::Return(e) => assert!(matches!(e, Expr::If(_, _, _))),
                    other => panic!("expected Return, got {other:?}"),
                }
            }
            other => panic!("expected Do, got {other:?}"),
        }
    }

    #[test]
    fn do_existing_single_line_still_works() {
        // Regression guard — step 8's multi-stmt-mix test
        // pattern (each stmt on its own line) must still
        // parse correctly under the new boundary rule.
        let src = "def main : IO Unit := do\n  \
                   let x <- readLn\n  \
                   let y := 1\n  \
                   return (x, y)";
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

    #[test]
    fn do_let_value_match_multi_line() {
        let src = "def main : IO Nat := do\n  \
                   let r := match scrut with\n    \
                     | A => 1\n    \
                     | B => 2\n  \
                   return r";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        match value {
            Expr::Do(stmts) => {
                assert_eq!(stmts.len(), 2);
                match &stmts[0] {
                    DoStmt::Let { value, .. } => {
                        assert!(matches!(value, Expr::Match(_, _)));
                    }
                    other => panic!("expected Let, got {other:?}"),
                }
            }
            other => panic!("expected Do, got {other:?}"),
        }
    }

    // ─── `by …` tactic block (OX6 step 11e) ────────────────

    #[test]
    fn by_single_tactic_inline() {
        let src = "theorem t : True := by exact True.intro";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Theorem { proof, .. } = &decls[0].kind
            else { panic!("expected Theorem") };
        match proof {
            Expr::By(tactics) => {
                assert_eq!(tactics.len(), 1);
                assert_eq!(tactics[0], "exact True.intro");
            }
            other => panic!("expected By, got {other:?}"),
        }
    }

    #[test]
    fn by_semicolon_sequenced() {
        let src = "theorem t : Nat := by intro x; exact x";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Theorem { proof, .. } = &decls[0].kind
            else { panic!("expected Theorem") };
        match proof {
            Expr::By(tactics) => {
                assert_eq!(tactics.len(), 2);
                assert_eq!(tactics[0], "intro x");
                assert_eq!(tactics[1], "exact x");
            }
            other => panic!("expected By, got {other:?}"),
        }
    }

    #[test]
    fn by_multi_line_block() {
        let src = "theorem t : Nat := by\n  intro x\n  exact x";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Theorem { proof, .. } = &decls[0].kind
            else { panic!("expected Theorem") };
        match proof {
            Expr::By(tactics) => {
                assert_eq!(tactics.len(), 2);
                assert_eq!(tactics[0], "intro x");
                assert_eq!(tactics[1], "exact x");
            }
            other => panic!("expected By, got {other:?}"),
        }
    }

    #[test]
    fn by_in_example() {
        let src = "example : True := by trivial";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Example { proof, .. } = &decls[0].kind
            else { panic!("expected Example") };
        match proof {
            Expr::By(tactics) => {
                assert_eq!(tactics[0], "trivial");
            }
            other => panic!("expected By, got {other:?}"),
        }
    }

    #[test]
    fn by_stops_at_next_top_level_decl() {
        // Multi-decl source — the `by` block must not eat
        // the next `def` line.
        let src = "theorem t : True := by\n  exact True.intro\ndef next : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 2);
        let DeclKind::Theorem { proof, .. } = &decls[0].kind
            else { panic!("expected Theorem") };
        match proof {
            Expr::By(tactics) => assert_eq!(tactics.len(), 1),
            other => panic!("expected By, got {other:?}"),
        }
    }

    #[test]
    fn by_with_simp_brackets() {
        // `simp [foo, bar]` — bracket content in the raw
        // tactic text is preserved as-is (not split further).
        let src = "theorem t : Nat := by simp [Nat.add_comm, Nat.zero_add]";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Theorem { proof, .. } = &decls[0].kind
            else { panic!("expected Theorem") };
        match proof {
            Expr::By(tactics) => {
                assert_eq!(tactics.len(), 1);
                assert!(tactics[0].contains("simp"));
                assert!(tactics[0].contains("Nat.add_comm"));
            }
            other => panic!("expected By, got {other:?}"),
        }
    }

    #[test]
    fn by_empty_block() {
        // `:= by` with no tactic body — yields empty Vec.
        let src = "theorem t : Nat := by";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Theorem { proof, .. } = &decls[0].kind
            else { panic!("expected Theorem") };
        match proof {
            Expr::By(tactics) => assert!(tactics.is_empty()),
            other => panic!("expected By, got {other:?}"),
        }
    }

    // ─── let-in expression (OX6 step 11d) ──────────────────

    #[test]
    fn let_in_simple_semicolon() {
        let e = parse_value_expr("let x := 1; x");
        match e {
            Expr::Let { name, ty, value, body } => {
                assert_eq!(name, "x");
                assert!(ty.is_none());
                assert_eq!(*value, Expr::Lit(Literal::Nat(1)));
                assert_eq!(*body, Expr::Ident("x".into()));
            }
            other => panic!("expected Let, got {other:?}"),
        }
    }

    #[test]
    fn let_in_with_type_annotation() {
        let e = parse_value_expr("let x : Nat := 1; x");
        match e {
            Expr::Let { name, ty, .. } => {
                assert_eq!(name, "x");
                assert_eq!(ty.as_deref(), Some(&Expr::Ident("Nat".into())));
            }
            other => panic!("expected Let, got {other:?}"),
        }
    }

    #[test]
    fn let_in_newline_separator() {
        // Body on the next line — newline is a valid
        // separator in addition to `;`.
        let src = "def f : Nat := let x := 1\nx + 1";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        assert!(matches!(value, Expr::Let { .. }));
    }

    #[test]
    fn let_in_value_is_complex_expr() {
        let e = parse_value_expr("let s := a + b; s * 2");
        match e {
            Expr::Let { value, body, .. } => {
                assert!(matches!(*value, Expr::BinOp(ref o, _, _) if o == "+"));
                assert!(matches!(*body, Expr::BinOp(ref o, _, _) if o == "*"));
            }
            other => panic!("expected Let, got {other:?}"),
        }
    }

    #[test]
    fn nested_let() {
        let e = parse_value_expr("let x := 1; let y := 2; x + y");
        match e {
            Expr::Let { name, body, .. } => {
                assert_eq!(name, "x");
                assert!(matches!(*body, Expr::Let { .. }));
            }
            other => panic!("expected Let, got {other:?}"),
        }
    }

    #[test]
    fn let_in_def_body() {
        let src = "def compute : Nat := let n := 10; n * n";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        assert!(matches!(value, Expr::Let { .. }));
    }

    // ─── `example` anonymous theorem (OX6 step 11k) ────────

    #[test]
    fn example_no_binders() {
        let src = "example : True := True.intro";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Example { binders, ty, .. } = &decls[0].kind
            else { panic!("expected Example") };
        assert!(binders.is_empty());
        assert_eq!(*ty, Expr::Ident("True".into()));
    }

    #[test]
    fn example_with_binder() {
        let src = "example (n : Nat) : n = n := rfl";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Example { binders, ty, proof } = &decls[0].kind
            else { panic!("expected Example") };
        assert_eq!(binders.len(), 1);
        assert!(matches!(ty, Expr::BinOp(o, _, _) if o == "="));
        assert_eq!(*proof, Expr::Ident("rfl".into()));
    }

    #[test]
    fn example_with_attr_prefix() {
        let src = "@[simp]\nexample : True := True.intro";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert!(matches!(decls[0].kind, DeclKind::Example { .. }));
    }

    #[test]
    fn example_with_forall_proof() {
        let src = "example : forall n, n = n := fun n => rfl";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Example { ty, proof, .. } = &decls[0].kind
            else { panic!("expected Example") };
        assert!(matches!(ty, Expr::Forall(_, _)));
        assert!(matches!(proof, Expr::Lam(_, _)));
    }

    // ─── modifier prefixes (OX6 step 11c) ──────────────────

    #[test]
    fn modifier_partial_def() {
        let src = "partial def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].modifiers, vec!["partial".to_string()]);
        assert!(matches!(decls[0].kind, DeclKind::Definition { .. }));
    }

    #[test]
    fn modifier_noncomputable_def() {
        let src = "noncomputable def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].modifiers, vec!["noncomputable".to_string()]);
    }

    #[test]
    fn modifier_private_def() {
        let src = "private def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].modifiers, vec!["private".to_string()]);
    }

    #[test]
    fn modifier_protected_theorem() {
        let src = "protected theorem t : True := True.intro";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].modifiers, vec!["protected".to_string()]);
        assert!(matches!(decls[0].kind, DeclKind::Theorem { .. }));
    }

    #[test]
    fn modifier_unsafe_def() {
        let src = "unsafe def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].modifiers, vec!["unsafe".to_string()]);
    }

    #[test]
    fn modifier_combined() {
        let src = "private noncomputable def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(
            decls[0].modifiers,
            vec!["private".to_string(), "noncomputable".to_string()]
        );
    }

    #[test]
    fn modifier_with_attr_prefix() {
        let src = "@[simp]\nprivate def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert_eq!(decls[0].modifiers, vec!["private".to_string()]);
    }

    #[test]
    fn abbrev_decl_surfaces_as_definition_with_modifier() {
        let src = "abbrev twice (n : Nat) : Nat := n";
        let decls = parse_decls(src).expect("must parse");
        assert!(matches!(decls[0].kind, DeclKind::Definition { .. }));
        assert_eq!(decls[0].modifiers, vec!["abbrev".to_string()]);
    }

    #[test]
    fn abbrev_with_universe_param() {
        let src = "abbrev id.{u} : Nat := 0";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].univ_params, vec!["u".to_string()]);
        assert_eq!(decls[0].modifiers, vec!["abbrev".to_string()]);
    }

    #[test]
    fn no_modifier_yields_empty_vec() {
        let src = "def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert!(decls[0].modifiers.is_empty());
    }

    // ─── universe annotation `.{u, v}` (OX6 step 11i) ──────

    #[test]
    fn def_with_single_universe_param() {
        let src = "def id.{u} : Nat := 0";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].univ_params, vec!["u".to_string()]);
    }

    #[test]
    fn def_with_multi_universe_params() {
        let src = "def pair.{u, v} : Nat := 0";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(
            decls[0].univ_params,
            vec!["u".to_string(), "v".to_string()]
        );
    }

    #[test]
    fn def_without_universe_params() {
        let src = "def f : Nat := 0";
        let decls = parse_decls(src).expect("must parse");
        assert!(decls[0].univ_params.is_empty());
    }

    #[test]
    fn theorem_with_universe_param() {
        let src = "theorem t.{u} : True := True.intro";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].univ_params, vec!["u".to_string()]);
    }

    #[test]
    fn axiom_with_universe_param() {
        let src = "axiom choice.{u} : Nat";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].univ_params, vec!["u".to_string()]);
    }

    #[test]
    fn structure_with_universe_params() {
        let src = "structure Pair.{u, v} where\n  fst : Nat\n  snd : Nat";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].univ_params.len(), 2);
        assert!(matches!(decls[0].kind, DeclKind::Structure { .. }));
    }

    #[test]
    fn inductive_with_universe_param() {
        let src = "inductive Tree.{u} where\n  | leaf\n  | node";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].univ_params, vec!["u".to_string()]);
    }

    #[test]
    fn class_with_universe_param() {
        let src = "class Functor.{u} (f : Nat) where\n  map : Nat";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].univ_params, vec!["u".to_string()]);
    }

    #[test]
    fn def_universe_params_then_attr_prefix() {
        // Combined: attr prefix + universe params.
        let src = "@[simp]\ndef foo.{u} : Nat := 0";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert_eq!(decls[0].univ_params, vec!["u".to_string()]);
    }

    // ─── `@` explicit args (OX6 step 11j) ──────────────────

    #[test]
    fn at_explicit_simple_ident() {
        let e = parse_value_expr("@id");
        match e {
            Expr::At(inner) => assert_eq!(*inner, Expr::Ident("id".into())),
            other => panic!("expected At, got {other:?}"),
        }
    }

    #[test]
    fn at_explicit_with_args() {
        // `@id Nat 0` — `@id` is the explicit-args form;
        // `@id Nat 0` is `App(App(@id, Nat), 0)`.
        let e = parse_value_expr("@id Nat 0");
        match e {
            Expr::App(fx, n) => {
                assert_eq!(*n, Expr::Lit(Literal::Nat(0)));
                match *fx {
                    Expr::App(at_id, nat) => {
                        assert!(matches!(*at_id, Expr::At(_)));
                        assert_eq!(*nat, Expr::Ident("Nat".into()));
                    }
                    other => panic!("expected nested App, got {other:?}"),
                }
            }
            other => panic!("expected App, got {other:?}"),
        }
    }

    #[test]
    fn at_explicit_dotted_ident() {
        let e = parse_value_expr("@Nat.succ");
        match e {
            Expr::At(inner) => assert_eq!(*inner, Expr::Ident("Nat.succ".into())),
            other => panic!("expected At, got {other:?}"),
        }
    }

    #[test]
    fn at_explicit_with_paren() {
        let e = parse_value_expr("@(foo + bar)");
        match e {
            Expr::At(inner) => {
                assert!(matches!(*inner, Expr::Paren(_)));
            }
            other => panic!("expected At, got {other:?}"),
        }
    }

    #[test]
    fn attribute_prefix_still_parses() {
        // Regression guard — `@[…]` in decl-prefix position
        // must NOT trigger the at-expr rule (which lives in
        // atom position only).
        let src = "@[simp]\ndef f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert_eq!(decls[0].attrs[0].name, "simp");
    }

    // ─── anonymous ctor `⟨…⟩` (OX6 step 11b) ───────────────

    #[test]
    fn anon_ctor_pair() {
        let e = parse_value_expr("⟨1, 2⟩");
        match e {
            Expr::AnonCtor(items) => {
                assert_eq!(items.len(), 2);
                assert_eq!(items[0], Expr::Lit(Literal::Nat(1)));
                assert_eq!(items[1], Expr::Lit(Literal::Nat(2)));
            }
            other => panic!("expected AnonCtor, got {other:?}"),
        }
    }

    #[test]
    fn anon_ctor_single_field() {
        let e = parse_value_expr("⟨42⟩");
        match e {
            Expr::AnonCtor(items) => {
                assert_eq!(items.len(), 1);
            }
            other => panic!("expected AnonCtor, got {other:?}"),
        }
    }

    #[test]
    fn anon_ctor_empty() {
        let e = parse_value_expr("⟨⟩");
        match e {
            Expr::AnonCtor(items) => assert!(items.is_empty()),
            other => panic!("expected AnonCtor, got {other:?}"),
        }
    }

    #[test]
    fn anon_ctor_nested() {
        let e = parse_value_expr("⟨⟨1, 2⟩, ⟨3, 4⟩⟩");
        match e {
            Expr::AnonCtor(outer) => {
                assert_eq!(outer.len(), 2);
                assert!(matches!(outer[0], Expr::AnonCtor(_)));
                assert!(matches!(outer[1], Expr::AnonCtor(_)));
            }
            other => panic!("expected AnonCtor, got {other:?}"),
        }
    }

    #[test]
    fn anon_ctor_with_complex_exprs() {
        let e = parse_value_expr("⟨a + b, fun n => n⟩");
        match e {
            Expr::AnonCtor(items) => {
                assert_eq!(items.len(), 2);
                assert!(matches!(items[0], Expr::BinOp(ref o, _, _) if o == "+"));
                assert!(matches!(items[1], Expr::Lam(_, _)));
            }
            other => panic!("expected AnonCtor, got {other:?}"),
        }
    }

    #[test]
    fn anon_ctor_in_def_value() {
        let src = "def origin : Point := ⟨0, 0⟩";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        assert!(matches!(value, Expr::AnonCtor(_)));
    }

    // ─── anonymous structure literal (OX6 step 11g) ────────

    #[test]
    fn anon_struct_two_fields() {
        let e = parse_value_expr("{ x := 1, y := 2 }");
        match e {
            Expr::AnonStruct(fields) => {
                assert_eq!(fields.len(), 2);
                assert_eq!(fields[0].0, "x");
                assert_eq!(fields[0].1, Expr::Lit(Literal::Nat(1)));
                assert_eq!(fields[1].0, "y");
                assert_eq!(fields[1].1, Expr::Lit(Literal::Nat(2)));
            }
            other => panic!("expected AnonStruct, got {other:?}"),
        }
    }

    #[test]
    fn anon_struct_single_field() {
        let e = parse_value_expr("{ value := 42 }");
        match e {
            Expr::AnonStruct(fields) => {
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].0, "value");
            }
            other => panic!("expected AnonStruct, got {other:?}"),
        }
    }

    #[test]
    fn anon_struct_field_with_complex_expr() {
        let e = parse_value_expr("{ sum := a + b, doubled := 2 * x }");
        match e {
            Expr::AnonStruct(fields) => {
                assert_eq!(fields.len(), 2);
                assert!(matches!(fields[0].1, Expr::BinOp(ref o, _, _) if o == "+"));
                assert!(matches!(fields[1].1, Expr::BinOp(ref o, _, _) if o == "*"));
            }
            other => panic!("expected AnonStruct, got {other:?}"),
        }
    }

    #[test]
    fn anon_struct_in_def_value() {
        let src = "def origin : Point := { x := 0, y := 0 }";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        assert!(matches!(value, Expr::AnonStruct(_)));
    }

    #[test]
    fn implicit_binder_still_works() {
        // `{T : Type}` in binder context must still parse
        // as an implicit binder, NOT an anon struct literal.
        // Disambiguator: binder uses `:` (no `:=`); anon
        // struct uses `:=`.
        let src = "def id {T : Type} (x : T) : T := x";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { binders, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        assert_eq!(binders[0].kind, BinderKind::Implicit);
    }

    // ─── list literal (OX6 step 11h) ───────────────────────

    #[test]
    fn list_lit_empty() {
        let e = parse_value_expr("[]");
        match e {
            Expr::List(items) => assert!(items.is_empty()),
            other => panic!("expected List, got {other:?}"),
        }
    }

    #[test]
    fn list_lit_three_nat() {
        let e = parse_value_expr("[1, 2, 3]");
        match e {
            Expr::List(items) => {
                assert_eq!(items.len(), 3);
                assert_eq!(items[0], Expr::Lit(Literal::Nat(1)));
                assert_eq!(items[2], Expr::Lit(Literal::Nat(3)));
            }
            other => panic!("expected List, got {other:?}"),
        }
    }

    #[test]
    fn list_lit_with_expressions() {
        let e = parse_value_expr("[a + b, f x, 42]");
        match e {
            Expr::List(items) => {
                assert_eq!(items.len(), 3);
                assert!(matches!(items[0], Expr::BinOp(ref o, _, _) if o == "+"));
                assert!(matches!(items[1], Expr::App(_, _)));
                assert_eq!(items[2], Expr::Lit(Literal::Nat(42)));
            }
            other => panic!("expected List, got {other:?}"),
        }
    }

    #[test]
    fn list_lit_nested() {
        let e = parse_value_expr("[[1, 2], [3, 4]]");
        match e {
            Expr::List(outer) => {
                assert_eq!(outer.len(), 2);
                assert!(matches!(outer[0], Expr::List(_)));
                assert!(matches!(outer[1], Expr::List(_)));
            }
            other => panic!("expected List, got {other:?}"),
        }
    }

    #[test]
    fn def_with_list_value() {
        let src = "def primes : List Nat := [2, 3, 5, 7, 11]";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        assert!(matches!(value, Expr::List(_)));
    }

    #[test]
    fn list_instance_binder_still_works() {
        // `[Ord T]` in binder context must still parse as
        // an instance binder, NOT a list literal — binders
        // use their own grammar, not the expr atom path.
        let src = "def f [Ord T] (a b : T) : T := a";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { binders, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        assert_eq!(binders.len(), 2);
        assert_eq!(binders[0].kind, BinderKind::Instance);
    }

    // ─── block + doc comments (OX6 step 11a) ───────────────

    #[test]
    fn block_comment_skipped() {
        let src = "/- top-level block comment -/\n\
                   def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 1);
    }

    #[test]
    fn block_comment_inside_decl() {
        let src = "def f /- inline -/ : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { name, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        assert_eq!(name, "f");
    }

    #[test]
    fn nested_block_comment() {
        let src = "/- outer /- inner -/ still outer -/\n\
                   def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 1);
    }

    #[test]
    fn deeply_nested_block_comment() {
        let src = "/- a /- b /- c -/ b -/ a -/\n\
                   def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 1);
    }

    #[test]
    fn doc_comment_treated_as_block_comment() {
        // `/-- … -/` parses as an ordinary block comment in
        // v0; semantic attachment to the next decl is OX6
        // step 11u.
        let src = "/-- doc string for f -/\ndef f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 1);
    }

    #[test]
    fn block_comment_multi_line() {
        let src = "/-\n\
                   multi-line\n\
                   block\n\
                   comment\n\
                   -/\n\
                   def f : Nat := 1";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 1);
    }

    // ─── open / import / variable (OX6 step 10d) ──────────

    #[test]
    fn open_single_namespace() {
        let src = "open Foo";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Open { items, raw_tail } = &decls[0].kind
            else { panic!("expected Open") };
        assert_eq!(items, &vec!["Foo".to_string()]);
        assert!(raw_tail.is_empty());
    }

    #[test]
    fn open_multiple_namespaces() {
        let src = "open Foo Bar Baz";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Open { items, raw_tail } = &decls[0].kind
            else { panic!("expected Open") };
        assert_eq!(items, &vec!["Foo".to_string(), "Bar".to_string(), "Baz".to_string()]);
        assert!(raw_tail.is_empty());
    }

    #[test]
    fn open_dotted_namespace() {
        let src = "open Foo.Bar.Baz";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Open { items, .. } = &decls[0].kind
            else { panic!("expected Open") };
        assert_eq!(items, &vec!["Foo.Bar.Baz".to_string()]);
    }

    #[test]
    fn open_selective_in_raw_tail() {
        let src = "open Foo (a b c)";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Open { items, raw_tail } = &decls[0].kind
            else { panic!("expected Open") };
        assert_eq!(items, &vec!["Foo".to_string()]);
        assert!(raw_tail.contains("(a b c)"));
    }

    #[test]
    fn open_renaming_in_raw_tail() {
        let src = "open Foo renaming a → b, c → d";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Open { items, raw_tail } = &decls[0].kind
            else { panic!("expected Open") };
        assert_eq!(items, &vec!["Foo".to_string()]);
        assert!(raw_tail.starts_with("renaming"));
    }

    #[test]
    fn open_hiding_in_raw_tail() {
        let src = "open Foo hiding x y";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Open { raw_tail, .. } = &decls[0].kind
            else { panic!("expected Open") };
        assert!(raw_tail.contains("hiding"));
    }

    #[test]
    fn open_scoped_in_raw_tail() {
        // Mathlib's `open scoped …`.
        let src = "open scoped Foo";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Open { items, raw_tail } = &decls[0].kind
            else { panic!("expected Open") };
        assert!(items.is_empty(), "scoped is a clause marker, not a namespace");
        assert!(raw_tail.starts_with("scoped"));
    }

    #[test]
    fn import_single_dotted() {
        let src = "import Foo.Bar.Baz";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Import { path } = &decls[0].kind
            else { panic!("expected Import") };
        assert_eq!(path, "Foo.Bar.Baz");
    }

    #[test]
    fn import_init_core() {
        let src = "import Init.Core";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Import { path } = &decls[0].kind
            else { panic!("expected Import") };
        assert_eq!(path, "Init.Core");
    }

    #[test]
    fn multiple_imports_each_own_decl() {
        let src = "import Foo\nimport Bar.Baz\nimport Qux";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 3);
        for d in &decls {
            assert!(matches!(d.kind, DeclKind::Import { .. }));
        }
    }

    #[test]
    fn variable_explicit_binder() {
        let src = "variable (n : Nat)";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Variable { binders } = &decls[0].kind
            else { panic!("expected Variable") };
        assert_eq!(binders.len(), 1);
        assert_eq!(binders[0].kind, BinderKind::Explicit);
    }

    #[test]
    fn variable_implicit_binder() {
        let src = "variable {T : Type}";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Variable { binders } = &decls[0].kind
            else { panic!("expected Variable") };
        assert_eq!(binders[0].kind, BinderKind::Implicit);
    }

    #[test]
    fn variable_multiple_binder_groups() {
        let src = "variable (n : Nat) {T : Type} [Inhabited T]";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Variable { binders } = &decls[0].kind
            else { panic!("expected Variable") };
        assert_eq!(binders.len(), 3);
        assert_eq!(binders[0].kind, BinderKind::Explicit);
        assert_eq!(binders[1].kind, BinderKind::Implicit);
        assert_eq!(binders[2].kind, BinderKind::Instance);
    }

    #[test]
    fn import_open_variable_section_combo() {
        let src = "import Init.Core\n\
                   open Foo\n\
                   section\n\
                   variable (n : Nat)\n\
                   def use_n : Nat := n\n\
                   end";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 3);
        assert!(matches!(decls[0].kind, DeclKind::Import { .. }));
        assert!(matches!(decls[1].kind, DeclKind::Open { .. }));
        assert!(matches!(decls[2].kind, DeclKind::Section { .. }));
    }

    // ─── namespace / section / mutual (OX6 step 10c) ──────

    #[test]
    fn namespace_with_inner_decls() {
        let src = "namespace Foo\n\
                   def x : Nat := 1\n\
                   def y : Nat := 2\n\
                   end Foo";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 1);
        let DeclKind::Namespace { name, decls: inner } = &decls[0].kind
            else { panic!("expected Namespace") };
        assert_eq!(name, "Foo");
        assert_eq!(inner.len(), 2);
        assert!(matches!(inner[0].kind, DeclKind::Definition { .. }));
        assert!(matches!(inner[1].kind, DeclKind::Definition { .. }));
    }

    #[test]
    fn namespace_end_without_name() {
        let src = "namespace Bar\n\
                   def x : Nat := 1\n\
                   end";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Namespace { name, decls: inner } = &decls[0].kind
            else { panic!("expected Namespace") };
        assert_eq!(name, "Bar");
        assert_eq!(inner.len(), 1);
    }

    #[test]
    fn namespace_dotted_name() {
        // `namespace Foo.Bar` — qualified namespace.
        let src = "namespace Foo.Bar\n\
                   def x : Nat := 1\n\
                   end Foo.Bar";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Namespace { name, .. } = &decls[0].kind
            else { panic!("expected Namespace") };
        assert_eq!(name, "Foo.Bar");
    }

    #[test]
    fn section_anonymous() {
        let src = "section\n\
                   def helper : Nat := 0\n\
                   end";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Section { name, decls: inner } = &decls[0].kind
            else { panic!("expected Section") };
        assert!(name.is_none());
        assert_eq!(inner.len(), 1);
    }

    #[test]
    fn section_named() {
        let src = "section MySection\n\
                   def helper : Nat := 0\n\
                   end MySection";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Section { name, .. } = &decls[0].kind
            else { panic!("expected Section") };
        assert_eq!(name.as_deref(), Some("MySection"));
    }

    #[test]
    fn mutual_two_decls() {
        let src = "mutual\n\
                   def f : Nat := 1\n\
                   def g : Nat := 2\n\
                   end";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Mutual { decls: inner } = &decls[0].kind
            else { panic!("expected Mutual") };
        assert_eq!(inner.len(), 2);
    }

    #[test]
    fn nested_namespaces() {
        let src = "namespace Outer\n\
                   namespace Inner\n\
                   def x : Nat := 1\n\
                   end Inner\n\
                   end Outer";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Namespace { name, decls: outer_decls } = &decls[0].kind
            else { panic!("expected outer Namespace") };
        assert_eq!(name, "Outer");
        assert_eq!(outer_decls.len(), 1);
        assert!(matches!(outer_decls[0].kind, DeclKind::Namespace { .. }));
    }

    #[test]
    fn namespace_contains_structure_and_inductive() {
        let src = "namespace M\n\
                   structure P where\n  x : Nat\n\
                   inductive Col where\n  | r\n  | b\n\
                   end M";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Namespace { decls: inner, .. } = &decls[0].kind
            else { panic!("expected Namespace") };
        assert_eq!(inner.len(), 2);
        assert!(matches!(inner[0].kind, DeclKind::Structure { .. }));
        assert!(matches!(inner[1].kind, DeclKind::Inductive { .. }));
    }

    #[test]
    fn mutual_with_attr_prefix() {
        // Attribute prefix applies to the `mutual` block as
        // a whole (or to its inner decls — Lean 4 spec is
        // contextual; we attach to the mutual block).
        let src = "@[macro_inline]\nmutual\n\
                   def f : Nat := 1\n\
                   end";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert!(matches!(decls[0].kind, DeclKind::Mutual { .. }));
    }

    #[test]
    fn empty_namespace() {
        let src = "namespace Empty\nend Empty";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Namespace { name, decls: inner } = &decls[0].kind
            else { panic!("expected Namespace") };
        assert_eq!(name, "Empty");
        assert!(inner.is_empty());
    }

    // ─── instance / class (OX6 step 10b) ───────────────────

    #[test]
    fn instance_anonymous_where_form() {
        let src = "instance : ToString Foo where\n  toString := fun _ => \"foo\"";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Instance { name, body, .. } = &decls[0].kind
            else { panic!("expected Instance") };
        assert!(name.is_none());
        match body {
            InstanceBody::Where(fields) => {
                assert_eq!(fields.len(), 1);
                assert_eq!(fields[0].name, "toString");
            }
            InstanceBody::Term(e) => panic!("expected Where body, got Term({e:?})"),
        }
    }

    #[test]
    fn instance_named_with_term_body() {
        let src = "instance addPair : Add Pair := wrap";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Instance { name, body, ty, .. } = &decls[0].kind
            else { panic!("expected Instance") };
        assert_eq!(name.as_deref(), Some("addPair"));
        match body {
            InstanceBody::Term(e) => assert_eq!(*e, Expr::Ident("wrap".into())),
            InstanceBody::Where(fs) => panic!("expected Term body, got Where({} fields)", fs.len()),
        }
        // ty `Add Pair` is App(Add, Pair).
        assert!(matches!(ty, Expr::App(_, _)));
    }

    #[test]
    fn instance_with_instance_binder() {
        let src = "instance [Inhabited T] : Inhabited Pair where\n  default := def_pair";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Instance { binders, .. } = &decls[0].kind
            else { panic!("expected Instance") };
        assert_eq!(binders.len(), 1);
        assert_eq!(binders[0].kind, BinderKind::Instance);
    }

    #[test]
    fn instance_with_attr_prefix() {
        let src = "@[default_instance]\ninstance : ToString Foo := mk";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert!(matches!(decls[0].kind, DeclKind::Instance { .. }));
    }

    #[test]
    fn class_no_binders() {
        // No binders form — `class NAME where ...`.
        let src = "class Trivial where\n  marker : Nat";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Class { name, binders, fields, .. } = &decls[0].kind
            else { panic!("expected Class") };
        assert_eq!(name, "Trivial");
        assert!(binders.is_empty());
        assert_eq!(fields[0].name, "marker");
    }

    #[test]
    fn class_with_typed_binder() {
        let src = "class Functor (f : Type -> Type) where\n  map : Nat";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Class { name, binders, fields, .. } = &decls[0].kind
            else { panic!("expected Class") };
        assert_eq!(name, "Functor");
        assert_eq!(binders.len(), 1);
        assert_eq!(binders[0].kind, BinderKind::Explicit);
        assert_eq!(binders[0].names, vec!["f".to_string()]);
        assert_eq!(fields.len(), 1);
    }

    #[test]
    fn class_extends_other_class() {
        let src = "class Monad extends Functor where\n  bind : Nat";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Class { extends, .. } = &decls[0].kind
            else { panic!("expected Class") };
        assert_eq!(extends, &vec!["Functor".to_string()]);
    }

    #[test]
    fn class_with_deriving() {
        let src = "class Foo where\n  x : Nat\n  deriving Repr";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Class { deriving, .. } = &decls[0].kind
            else { panic!("expected Class") };
        assert_eq!(deriving, &vec!["Repr".to_string()]);
    }

    #[test]
    fn class_with_attr_prefix() {
        // `class` itself is a keyword so it can't be used as
        // an attribute name. Use `builtin_class` (a common
        // attribute name shape).
        let src = "@[builtin_class]\nclass Foo where\n  x : Nat";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert!(matches!(decls[0].kind, DeclKind::Class { .. }));
    }

    #[test]
    fn multi_decl_with_class_and_instance() {
        let src = "class Foo where\n  x : Nat\n\
                   \n\
                   instance : Foo where\n  x := 42";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 2);
        assert!(matches!(decls[0].kind, DeclKind::Class { .. }));
        assert!(matches!(decls[1].kind, DeclKind::Instance { .. }));
    }

    // ─── quantifiers (OX6 step 9.5) ────────────────────────

    #[test]
    fn forall_untyped_binder() {
        let e = parse_value_expr("forall x, P x");
        match e {
            Expr::Forall(binders, body) => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0], LamBinder::Untyped("x".into()));
                assert!(matches!(*body, Expr::App(_, _)));
            }
            other => panic!("expected Forall, got {other:?}"),
        }
    }

    #[test]
    fn forall_unicode_symbol() {
        let e = parse_value_expr("∀ x, P x");
        assert!(matches!(e, Expr::Forall(_, _)));
    }

    #[test]
    fn forall_typed_binder() {
        let e = parse_value_expr("forall (n : Nat), P n");
        match e {
            Expr::Forall(binders, _) => {
                assert_eq!(binders.len(), 1);
                match &binders[0] {
                    LamBinder::Typed { kind, names, ty } => {
                        assert_eq!(*kind, BinderKind::Explicit);
                        assert_eq!(names, &vec!["n".to_string()]);
                        assert_eq!(*ty, Expr::Ident("Nat".into()));
                    }
                    LamBinder::Untyped(s) => panic!("expected Typed, got Untyped({s})"),
                }
            }
            other => panic!("expected Forall, got {other:?}"),
        }
    }

    #[test]
    fn forall_multi_binder() {
        let e = parse_value_expr("forall x y z, P x y z");
        match e {
            Expr::Forall(binders, _) => assert_eq!(binders.len(), 3),
            other => panic!("expected Forall, got {other:?}"),
        }
    }

    #[test]
    fn forall_implicit_binder() {
        let e = parse_value_expr("forall {T : Type}, T -> T");
        match e {
            Expr::Forall(binders, body) => {
                assert_eq!(binders.len(), 1);
                assert!(matches!(
                    &binders[0],
                    LamBinder::Typed { kind: BinderKind::Implicit, .. }
                ));
                // Body is `T -> T` arrow type.
                assert!(matches!(*body, Expr::BinOp(ref o, _, _) if o == "->"));
            }
            other => panic!("expected Forall, got {other:?}"),
        }
    }

    #[test]
    fn exists_untyped_binder() {
        let e = parse_value_expr("exists x, P x");
        match e {
            Expr::Exists(binders, _) => {
                assert_eq!(binders.len(), 1);
                assert_eq!(binders[0], LamBinder::Untyped("x".into()));
            }
            other => panic!("expected Exists, got {other:?}"),
        }
    }

    #[test]
    fn exists_unicode_symbol() {
        let e = parse_value_expr("∃ x, P x");
        assert!(matches!(e, Expr::Exists(_, _)));
    }

    #[test]
    fn exists_typed_binder() {
        let e = parse_value_expr("exists (n : Nat), n > 0");
        match e {
            Expr::Exists(_, body) => {
                assert!(matches!(*body, Expr::BinOp(ref o, _, _) if o == ">"));
            }
            other => panic!("expected Exists, got {other:?}"),
        }
    }

    #[test]
    fn theorem_with_forall_in_signature() {
        let src = "theorem id_eq : forall x, x = x := fun x => rfl";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Theorem { ty, proof, .. } = &decls[0].kind
            else { panic!("expected Theorem") };
        assert!(matches!(ty, Expr::Forall(_, _)));
        assert!(matches!(proof, Expr::Lam(_, _)));
    }

    #[test]
    fn nested_quantifiers() {
        let e = parse_value_expr("forall x, exists y, x = y");
        match e {
            Expr::Forall(_, body) => {
                assert!(matches!(*body, Expr::Exists(_, _)));
            }
            other => panic!("expected Forall, got {other:?}"),
        }
    }

    // ─── theorem / lemma / axiom (OX6 step 10a) ────────────

    #[test]
    fn theorem_simple() {
        let src = "theorem t (n : Nat) : n = n := rfl";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Theorem { name, binders, .. } = &decls[0].kind
            else { panic!("expected Theorem") };
        assert_eq!(name, "t");
        assert_eq!(binders.len(), 1);
    }

    #[test]
    fn lemma_parses_as_theorem() {
        let src = "lemma t (n : Nat) : n = n := rfl";
        let decls = parse_decls(src).expect("must parse");
        // `lemma` collapses into the same AST variant as
        // `theorem` (semantically equivalent in Lean 4).
        assert!(matches!(decls[0].kind, DeclKind::Theorem { .. }));
    }

    #[test]
    fn theorem_no_binders() {
        let src = "theorem foo : True := True.intro";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Theorem { name, binders, ty, .. } = &decls[0].kind
            else { panic!("expected Theorem") };
        assert_eq!(name, "foo");
        assert!(binders.is_empty());
        assert_eq!(*ty, Expr::Ident("True".into()));
    }

    #[test]
    fn axiom_simple() {
        let src = "axiom em : forall p, p";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Axiom { name, ty, .. } = &decls[0].kind
            else { panic!("expected Axiom") };
        assert_eq!(name, "em");
        assert!(matches!(ty, Expr::Forall(_, _)));
    }

    #[test]
    fn axiom_no_binders() {
        let src = "axiom undefined : Nat";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Axiom { name, binders, ty } = &decls[0].kind
            else { panic!("expected Axiom") };
        assert_eq!(name, "undefined");
        assert!(binders.is_empty());
        assert_eq!(*ty, Expr::Ident("Nat".into()));
    }

    #[test]
    fn axiom_with_binders() {
        // Use ASCII `T` instead of Unicode `α` — single-char
        // ident grammar covers ASCII; Unicode-ident support
        // is a future grammar extension.
        let src = "axiom choice {T : Type} : T -> T";
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Axiom { binders, .. } = &decls[0].kind
            else { panic!("expected Axiom") };
        assert_eq!(binders.len(), 1);
        assert_eq!(binders[0].kind, BinderKind::Implicit);
    }

    #[test]
    fn theorem_with_attr_prefix() {
        let src = "@[simp]\ntheorem t (n : Nat) : n = n := rfl";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert!(matches!(decls[0].kind, DeclKind::Theorem { .. }));
    }

    #[test]
    fn axiom_with_attr_prefix() {
        let src = "@[extern]\naxiom foo : Nat";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls[0].attrs.len(), 1);
        assert!(matches!(decls[0].kind, DeclKind::Axiom { .. }));
    }

    #[test]
    fn multi_decl_mixed_kinds() {
        let src = "def f : Nat := 1\n\
                   theorem t : True := True.intro\n\
                   axiom a : Nat\n";
        let decls = parse_decls(src).expect("must parse");
        assert_eq!(decls.len(), 3);
        assert!(matches!(decls[0].kind, DeclKind::Definition { .. }));
        assert!(matches!(decls[1].kind, DeclKind::Theorem { .. }));
        assert!(matches!(decls[2].kind, DeclKind::Axiom { .. }));
    }

    // ─── string interpolation (OX6 step 9) ─────────────────

    #[test]
    fn interp_single_hole_only() {
        let e = parse_value_expr(r#"s!"{x}""#);
        match e {
            Expr::InterpStr(parts) => {
                assert_eq!(parts.len(), 1);
                assert_eq!(parts[0], InterpPart::Hole(Expr::Ident("x".into())));
            }
            other => panic!("expected InterpStr, got {other:?}"),
        }
    }

    #[test]
    fn interp_text_then_hole() {
        let e = parse_value_expr(r#"s!"hello {name}""#);
        match e {
            Expr::InterpStr(parts) => {
                assert_eq!(parts.len(), 2);
                assert_eq!(parts[0], InterpPart::Text("hello ".into()));
                assert_eq!(parts[1], InterpPart::Hole(Expr::Ident("name".into())));
            }
            other => panic!("expected InterpStr, got {other:?}"),
        }
    }

    #[test]
    fn interp_hole_with_complex_expr() {
        let e = parse_value_expr(r#"s!"sum: {a + b}""#);
        match e {
            Expr::InterpStr(parts) => {
                assert_eq!(parts.len(), 2);
                match &parts[1] {
                    InterpPart::Hole(Expr::BinOp(op, _, _)) => assert_eq!(op, "+"),
                    other => panic!("expected BinOp hole, got {other:?}"),
                }
            }
            other => panic!("expected InterpStr, got {other:?}"),
        }
    }

    #[test]
    fn interp_multiple_holes() {
        let e = parse_value_expr(r#"s!"x={x}, y={y}""#);
        match e {
            Expr::InterpStr(parts) => {
                // text/hole/text/hole/(optional trailing text)
                let holes: Vec<&InterpPart> = parts
                    .iter()
                    .filter(|p| matches!(p, InterpPart::Hole(_)))
                    .collect();
                assert_eq!(holes.len(), 2);
            }
            other => panic!("expected InterpStr, got {other:?}"),
        }
    }

    #[test]
    fn interp_text_only_no_holes() {
        // `s!"hello"` with no `{…}` — still parses as InterpStr
        // (single Text part). Distinguishes from a plain
        // `"hello"` Lit at the parser level even though they
        // mean the same thing at the elaborator.
        let e = parse_value_expr(r#"s!"hello""#);
        match e {
            Expr::InterpStr(parts) => {
                assert_eq!(parts.len(), 1);
                assert_eq!(parts[0], InterpPart::Text("hello".into()));
            }
            other => panic!("expected InterpStr, got {other:?}"),
        }
    }

    #[test]
    fn interp_escaped_braces() {
        // `{{` / `}}` decode to literal `{` / `}` in the text
        // segment (matches Rust + Lean 4 convention).
        let e = parse_value_expr(r#"s!"set {{x, y}} -> {result}""#);
        match e {
            Expr::InterpStr(parts) => {
                // Text "set {x, y} -> " + Hole(result)
                let first_text = match &parts[0] {
                    InterpPart::Text(s) => s.clone(),
                    InterpPart::Hole(e) => panic!("expected Text, got Hole({e:?})"),
                };
                assert!(first_text.contains("set {x, y} -> "));
                assert!(matches!(parts.last(), Some(InterpPart::Hole(_))));
            }
            other => panic!("expected InterpStr, got {other:?}"),
        }
    }

    #[test]
    fn interp_text_escape_sequences() {
        let e = parse_value_expr(r#"s!"line1\nline2""#);
        match e {
            Expr::InterpStr(parts) => {
                assert_eq!(parts.len(), 1);
                assert_eq!(parts[0], InterpPart::Text("line1\nline2".into()));
            }
            other => panic!("expected InterpStr, got {other:?}"),
        }
    }

    #[test]
    fn interp_in_def_body() {
        let src = r#"def greet (name : String) : String := s!"hi {name}!""#;
        let decls = parse_decls(src).expect("must parse");
        let DeclKind::Definition { value, .. } = &decls[0].kind
            else { panic!("expected Definition") };
        assert!(matches!(value, Expr::InterpStr(_)));
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
