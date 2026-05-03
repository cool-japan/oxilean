//! Types for the enhanced PrettyPrinter.
//!
//! This module provides [`PrettyPrinter`] — a fully configurable,
//! layout-aware pretty-printer for kernel `Expr`, universe `Level`,
//! and `ConstantInfo` declarations.
//!
//! Unlike the simple `prettyprint::ExprPrinter` it supports:
//! - Configurable line width with soft/hard wrapping
//! - Indentation control (spaces vs tabs, configurable depth)
//! - Unicode / ASCII symbol selection
//! - Optional universe-annotation visibility
//! - Declaration printing with full signatures
//! - A Wadler-Lindig–style `Doc` IR for layout-optimal rendering

use std::fmt;
use std::fmt::Write as FmtWrite;

use crate::{BinderInfo, ConstantInfo, Expr, Level, Name};

// ── Indentation style ─────────────────────────────────────────────────────────

/// How the printer should indent nested constructs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndentMode {
    /// Use `n` spaces per level.
    Spaces(usize),
    /// Use a single tab character per level.
    Tabs,
}

impl Default for IndentMode {
    fn default() -> Self {
        IndentMode::Spaces(2)
    }
}

impl IndentMode {
    /// Produce the indentation string for `depth` levels.
    pub fn render(&self, depth: usize) -> String {
        match self {
            IndentMode::Spaces(n) => " ".repeat(n * depth),
            IndentMode::Tabs => "\t".repeat(depth),
        }
    }
}

// ── PrettyConfig ─────────────────────────────────────────────────────────────

/// Configuration for [`PrettyPrinter`].
#[derive(Clone, Debug)]
pub struct PrettyConfig {
    /// Maximum line width (used for layout decisions).  `0` means unlimited.
    pub width: usize,
    /// Indentation mode and depth.
    pub indent: IndentMode,
    /// Use Unicode symbols (λ, ∀, →, ≡, …).  If `false`, use ASCII.
    pub unicode: bool,
    /// Show universe levels on constants (`Nat.{0}` vs `Nat`).
    pub show_universes: bool,
    /// Show implicit arguments (`@f a b` style).
    pub show_implicit: bool,
    /// Show binder kinds `(x : A)` vs `{x : A}` vs `[x : A]`.
    pub show_binder_kinds: bool,
    /// Show de Bruijn indices for bound variables.
    pub show_bvar_indices: bool,
    /// Show proof bodies in theorems.
    pub show_proof_bodies: bool,
    /// Maximum depth before truncating with `…`.  `0` means unlimited.
    pub max_depth: usize,
}

impl Default for PrettyConfig {
    fn default() -> Self {
        Self {
            width: 100,
            indent: IndentMode::default(),
            unicode: true,
            show_universes: false,
            show_implicit: false,
            show_binder_kinds: true,
            show_bvar_indices: false,
            show_proof_bodies: false,
            max_depth: 0,
        }
    }
}

impl PrettyConfig {
    /// Verbose config: show everything.
    pub fn verbose() -> Self {
        Self {
            show_universes: true,
            show_implicit: true,
            show_binder_kinds: true,
            show_bvar_indices: true,
            show_proof_bodies: true,
            ..Default::default()
        }
    }

    /// ASCII-only config.
    pub fn ascii() -> Self {
        Self {
            unicode: false,
            ..Default::default()
        }
    }

    /// Compact config with no depth limit and wide lines.
    pub fn compact() -> Self {
        Self {
            width: 0,
            max_depth: 0,
            ..Default::default()
        }
    }

    /// Returns the arrow symbol (→ or ->).
    pub fn arrow(&self) -> &'static str {
        if self.unicode {
            "→"
        } else {
            "->"
        }
    }

    /// Returns the lambda symbol (λ or fun).
    pub fn lambda(&self) -> &'static str {
        if self.unicode {
            "λ"
        } else {
            "fun"
        }
    }

    /// Returns the forall symbol (∀ or forall).
    pub fn forall(&self) -> &'static str {
        if self.unicode {
            "∀"
        } else {
            "forall"
        }
    }

    /// Returns the definition symbol (:= or :=).
    pub fn define(&self) -> &'static str {
        ":="
    }

    /// Returns the ellipsis symbol (… or ...).
    pub fn ellipsis(&self) -> &'static str {
        if self.unicode {
            "…"
        } else {
            "..."
        }
    }
}

// ── Precedence levels ─────────────────────────────────────────────────────────

/// Operator precedence levels used for parenthesization decisions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Prec(pub u32);

impl Prec {
    /// Lowest precedence — used at top level; never parenthesizes.
    pub const BOTTOM: Prec = Prec(0);
    /// Application-body precedence (λ/∀ body).
    pub const BODY: Prec = Prec(5);
    /// Arrow/Pi precedence.
    pub const ARROW: Prec = Prec(20);
    /// Application precedence.
    pub const APP: Prec = Prec(100);
    /// Atom precedence (literals, names).
    pub const ATOM: Prec = Prec(1000);
}

// ── PrettyPrinter ─────────────────────────────────────────────────────────────

/// A configurable pretty-printer for OxiLean kernel terms.
///
/// The printer renders `Expr`, `Level`, and `ConstantInfo` to `String`.
/// It tracks the current indentation depth and checks the `max_depth`
/// limit (if set) to avoid infinite recursion on pathological terms.
///
/// ```
/// use oxilean_kernel::pretty_print::{PrettyPrinter, PrettyConfig};
/// use oxilean_kernel::{Expr, Level, Literal};
///
/// let pp = PrettyPrinter::new();
/// let nat_expr = Expr::Lit(Literal::Nat(42));
/// assert_eq!(pp.pp_expr(&nat_expr), "42");
/// ```
pub struct PrettyPrinter {
    /// Active configuration.
    pub config: PrettyConfig,
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self::new()
    }
}

impl PrettyPrinter {
    /// Create a printer with default configuration.
    pub fn new() -> Self {
        Self {
            config: PrettyConfig::default(),
        }
    }

    /// Create a printer with the supplied configuration.
    pub fn with_config(config: PrettyConfig) -> Self {
        Self { config }
    }

    /// Enable/disable unicode symbols.
    pub fn with_unicode(mut self, unicode: bool) -> Self {
        self.config.unicode = unicode;
        self
    }

    /// Set the maximum line width.
    pub fn with_width(mut self, width: usize) -> Self {
        self.config.width = width;
        self
    }

    /// Set the indentation mode.
    pub fn with_indent(mut self, indent: IndentMode) -> Self {
        self.config.indent = indent;
        self
    }

    /// Show or hide universe levels.
    pub fn with_universes(mut self, show: bool) -> Self {
        self.config.show_universes = show;
        self
    }

    // ── Public printing API ───────────────────────────────────────────────

    /// Pretty-print an expression.
    pub fn pp_expr(&self, e: &Expr) -> String {
        let mut buf = String::new();
        self.write_expr(&mut buf, e, Prec::BOTTOM, 0)
            .unwrap_or_default();
        buf
    }

    /// Pretty-print an expression intended as a type annotation.
    ///
    /// Identical to [`pp_expr`] but communicates intent and may in the future
    /// apply type-specific formatting (e.g., colouring or de Bruijn name
    /// recovery).
    pub fn pp_type(&self, t: &Expr) -> String {
        self.pp_expr(t)
    }

    /// Pretty-print a `ConstantInfo` declaration with its full signature.
    pub fn pp_decl(&self, decl: &ConstantInfo) -> String {
        let mut buf = String::new();
        self.write_decl(&mut buf, decl, 0).unwrap_or_default();
        buf
    }

    /// Pretty-print a universe level.
    pub fn pp_level(&self, l: &Level) -> String {
        let mut buf = String::new();
        self.write_level(&mut buf, l).unwrap_or_default();
        buf
    }

    // ── Internal: write_expr ──────────────────────────────────────────────

    /// Write an expression into `buf` at the given precedence and indent depth.
    ///
    /// `prec` determines whether parentheses are inserted.
    /// `depth` is the current indentation depth (for multi-line forms).
    pub(super) fn write_expr(
        &self,
        buf: &mut String,
        expr: &Expr,
        prec: Prec,
        depth: usize,
    ) -> fmt::Result {
        // Depth limit guard
        if self.config.max_depth > 0 && depth > self.config.max_depth {
            return write!(buf, "{}", self.config.ellipsis());
        }

        match expr {
            Expr::Sort(level) => self.write_sort(buf, level),

            Expr::BVar(idx) => {
                if self.config.show_bvar_indices {
                    write!(buf, "#{}", idx)
                } else {
                    write!(buf, "_")
                }
            }

            Expr::FVar(id) => write!(buf, "${}", id.0),

            Expr::Const(name, levels) => {
                self.write_name(buf, name)?;
                if self.config.show_universes && !levels.is_empty() {
                    write!(buf, ".{{")?;
                    for (i, lv) in levels.iter().enumerate() {
                        if i > 0 {
                            write!(buf, ", ")?;
                        }
                        self.write_level(buf, lv)?;
                    }
                    write!(buf, "}}")?;
                }
                Ok(())
            }

            Expr::App(_, _) => self.write_app(buf, expr, prec, depth),

            Expr::Lam(bi, name, ty, body) => self.write_lam(buf, *bi, name, ty, body, prec, depth),

            Expr::Pi(bi, name, ty, body) => self.write_pi(buf, *bi, name, ty, body, prec, depth),

            Expr::Let(name, ty, val, body) => self.write_let(buf, name, ty, val, body, prec, depth),

            Expr::Lit(lit) => write!(buf, "{}", lit),

            Expr::Proj(name, idx, inner) => {
                // inner.idx (structural projection)
                let needs_paren = prec > Prec::APP;
                if needs_paren {
                    write!(buf, "(")?;
                }
                self.write_expr(buf, inner, Prec::ATOM, depth)?;
                write!(buf, ".{}.{}", name, idx)?;
                if needs_paren {
                    write!(buf, ")")?;
                }
                Ok(())
            }
        }
    }

    // ── Sort / universe ───────────────────────────────────────────────────

    fn write_sort(&self, buf: &mut String, level: &Level) -> fmt::Result {
        if level.is_zero() {
            write!(buf, "Prop")
        } else if let Some(n) = level_to_nat(level) {
            if n == 1 {
                write!(buf, "Type")
            } else {
                write!(buf, "Type {}", n - 1)
            }
        } else {
            write!(buf, "Sort ")?;
            self.write_level(buf, level)
        }
    }

    fn write_level(&self, buf: &mut String, level: &Level) -> fmt::Result {
        match level {
            Level::Zero => write!(buf, "0"),
            Level::Succ(inner) => {
                if let Some(n) = level_to_nat(level) {
                    write!(buf, "{}", n)
                } else {
                    write!(buf, "succ(")?;
                    self.write_level(buf, inner)?;
                    write!(buf, ")")
                }
            }
            Level::Max(a, b) => {
                write!(buf, "max(")?;
                self.write_level(buf, a)?;
                write!(buf, ", ")?;
                self.write_level(buf, b)?;
                write!(buf, ")")
            }
            Level::IMax(a, b) => {
                write!(buf, "imax(")?;
                self.write_level(buf, a)?;
                write!(buf, ", ")?;
                self.write_level(buf, b)?;
                write!(buf, ")")
            }
            Level::Param(name) => self.write_name(buf, name),
            Level::MVar(id) => write!(buf, "?u{}", id.0),
        }
    }

    // ── Name ─────────────────────────────────────────────────────────────

    fn write_name(&self, buf: &mut String, name: &Name) -> fmt::Result {
        write!(buf, "{}", name)
    }

    // ── Application ──────────────────────────────────────────────────────

    fn write_app(&self, buf: &mut String, expr: &Expr, prec: Prec, depth: usize) -> fmt::Result {
        let (head, args) = collect_app(expr);
        let needs_paren = prec > Prec::APP;
        if needs_paren {
            write!(buf, "(")?;
        }
        self.write_expr(buf, head, Prec::APP, depth)?;
        for arg in args {
            write!(buf, " ")?;
            self.write_expr(buf, arg, Prec(Prec::APP.0 + 1), depth)?;
        }
        if needs_paren {
            write!(buf, ")")?;
        }
        Ok(())
    }

    // ── Lambda ────────────────────────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    fn write_lam(
        &self,
        buf: &mut String,
        bi: BinderInfo,
        name: &Name,
        ty: &Expr,
        body: &Expr,
        prec: Prec,
        depth: usize,
    ) -> fmt::Result {
        let needs_paren = prec > Prec::BODY;
        if needs_paren {
            write!(buf, "(")?;
        }
        write!(buf, "{} ", self.config.lambda())?;
        self.write_binder(buf, bi, name, ty, depth)?;

        // Collect consecutive lambdas for compact rendering
        let mut cur_body = body;
        while let Expr::Lam(bi2, name2, ty2, body2) = cur_body {
            write!(buf, " ")?;
            self.write_binder(buf, *bi2, name2, ty2, depth)?;
            cur_body = body2;
        }
        write!(buf, ", ")?;
        self.write_expr(buf, cur_body, Prec::BOTTOM, depth + 1)?;
        if needs_paren {
            write!(buf, ")")?;
        }
        Ok(())
    }

    // ── Pi / forall ───────────────────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    fn write_pi(
        &self,
        buf: &mut String,
        bi: BinderInfo,
        name: &Name,
        ty: &Expr,
        body: &Expr,
        prec: Prec,
        depth: usize,
    ) -> fmt::Result {
        let needs_paren = prec > Prec::ARROW;
        if needs_paren {
            write!(buf, "(")?;
        }

        let is_nondep = name.is_anonymous() || name.to_string() == "_";
        if is_nondep {
            // Non-dependent arrow: A → B
            self.write_expr(buf, ty, Prec(Prec::ARROW.0 + 1), depth)?;
            write!(buf, " {} ", self.config.arrow())?;
            self.write_expr(buf, body, Prec::ARROW, depth)?;
        } else {
            write!(buf, "{} ", self.config.forall())?;
            self.write_binder(buf, bi, name, ty, depth)?;

            // Collect consecutive dependent Pi's
            let mut cur_body = body;
            while let Expr::Pi(bi2, name2, ty2, body2) = cur_body {
                let is_dep2 = !name2.is_anonymous() && name2.to_string() != "_";
                if is_dep2 {
                    write!(buf, " ")?;
                    self.write_binder(buf, *bi2, name2, ty2, depth)?;
                    cur_body = body2;
                } else {
                    break;
                }
            }
            write!(buf, ", ")?;
            self.write_expr(buf, cur_body, Prec::BOTTOM, depth + 1)?;
        }

        if needs_paren {
            write!(buf, ")")?;
        }
        Ok(())
    }

    // ── Let ───────────────────────────────────────────────────────────────

    #[allow(clippy::too_many_arguments)]
    fn write_let(
        &self,
        buf: &mut String,
        name: &Name,
        ty: &Expr,
        val: &Expr,
        body: &Expr,
        _prec: Prec,
        depth: usize,
    ) -> fmt::Result {
        let indent = self.config.indent.render(depth);
        write!(buf, "let ")?;
        self.write_name(buf, name)?;
        write!(buf, " : ")?;
        self.write_expr(buf, ty, Prec::BOTTOM, depth)?;
        write!(buf, " {} ", self.config.define())?;
        self.write_expr(buf, val, Prec::BOTTOM, depth)?;
        write!(buf, "\n{}", indent)?;
        self.write_expr(buf, body, Prec::BOTTOM, depth + 1)
    }

    // ── Binder ────────────────────────────────────────────────────────────

    fn write_binder(
        &self,
        buf: &mut String,
        bi: BinderInfo,
        name: &Name,
        ty: &Expr,
        depth: usize,
    ) -> fmt::Result {
        if self.config.show_binder_kinds {
            match bi {
                BinderInfo::Default => {
                    write!(buf, "(")?;
                    self.write_name(buf, name)?;
                    write!(buf, " : ")?;
                    self.write_expr(buf, ty, Prec::BOTTOM, depth)?;
                    write!(buf, ")")
                }
                BinderInfo::Implicit => {
                    write!(buf, "{{")?;
                    self.write_name(buf, name)?;
                    write!(buf, " : ")?;
                    self.write_expr(buf, ty, Prec::BOTTOM, depth)?;
                    write!(buf, "}}")
                }
                BinderInfo::StrictImplicit => {
                    write!(buf, "{{")?;
                    self.write_name(buf, name)?;
                    write!(buf, " : ")?;
                    self.write_expr(buf, ty, Prec::BOTTOM, depth)?;
                    write!(buf, "}}")
                }
                BinderInfo::InstImplicit => {
                    write!(buf, "[")?;
                    self.write_name(buf, name)?;
                    write!(buf, " : ")?;
                    self.write_expr(buf, ty, Prec::BOTTOM, depth)?;
                    write!(buf, "]")
                }
            }
        } else {
            // Minimal binder without kind brackets
            self.write_name(buf, name)?;
            write!(buf, " : ")?;
            self.write_expr(buf, ty, Prec::BOTTOM, depth)
        }
    }

    // ── Declaration printing ──────────────────────────────────────────────

    fn write_decl(&self, buf: &mut String, decl: &ConstantInfo, depth: usize) -> fmt::Result {
        match decl {
            ConstantInfo::Axiom(ax) => {
                write!(buf, "axiom ")?;
                self.write_name(buf, &ax.common.name)?;
                self.write_level_params(buf, &ax.common.level_params)?;
                write!(buf, " : ")?;
                self.write_expr(buf, &ax.common.ty, Prec::BOTTOM, depth)
            }

            ConstantInfo::Definition(def) => {
                write!(buf, "def ")?;
                self.write_name(buf, &def.common.name)?;
                self.write_level_params(buf, &def.common.level_params)?;
                write!(buf, " : ")?;
                self.write_expr(buf, &def.common.ty, Prec::BOTTOM, depth)?;
                write!(buf, " {} ", self.config.define())?;
                self.write_expr(buf, &def.value, Prec::BOTTOM, depth)
            }

            ConstantInfo::Theorem(thm) => {
                write!(buf, "theorem ")?;
                self.write_name(buf, &thm.common.name)?;
                self.write_level_params(buf, &thm.common.level_params)?;
                write!(buf, " : ")?;
                self.write_expr(buf, &thm.common.ty, Prec::BOTTOM, depth)?;
                if self.config.show_proof_bodies {
                    write!(buf, " {} ", self.config.define())?;
                    self.write_expr(buf, &thm.value, Prec::BOTTOM, depth)?;
                } else {
                    let proof_kw = if self.config.unicode {
                        " :=  ⊢ …"
                    } else {
                        " := <proof>"
                    };
                    write!(buf, "{}", proof_kw)?;
                }
                Ok(())
            }

            ConstantInfo::Opaque(op) => {
                write!(buf, "opaque ")?;
                self.write_name(buf, &op.common.name)?;
                self.write_level_params(buf, &op.common.level_params)?;
                write!(buf, " : ")?;
                self.write_expr(buf, &op.common.ty, Prec::BOTTOM, depth)
            }

            ConstantInfo::Inductive(ind) => {
                write!(buf, "inductive ")?;
                self.write_name(buf, &ind.common.name)?;
                self.write_level_params(buf, &ind.common.level_params)?;
                write!(buf, " : ")?;
                self.write_expr(buf, &ind.common.ty, Prec::BOTTOM, depth)?;
                write!(buf, " [{} ctors]", ind.ctors.len())
            }

            ConstantInfo::Constructor(ctor) => {
                write!(buf, "constructor ")?;
                self.write_name(buf, &ctor.common.name)?;
                write!(buf, " : ")?;
                self.write_expr(buf, &ctor.common.ty, Prec::BOTTOM, depth)
            }

            ConstantInfo::Recursor(rec) => {
                write!(buf, "recursor ")?;
                self.write_name(buf, &rec.common.name)?;
                write!(buf, " : ")?;
                self.write_expr(buf, &rec.common.ty, Prec::BOTTOM, depth)
            }

            ConstantInfo::Quotient(quot) => {
                write!(buf, "quotient ")?;
                self.write_name(buf, &quot.common.name)?;
                write!(buf, " : ")?;
                self.write_expr(buf, &quot.common.ty, Prec::BOTTOM, depth)
            }
        }
    }

    /// Write universe level parameters if any: `.{u, v, w}`.
    fn write_level_params(&self, buf: &mut String, params: &[Name]) -> fmt::Result {
        if params.is_empty() {
            return Ok(());
        }
        write!(buf, ".{{")?;
        for (i, p) in params.iter().enumerate() {
            if i > 0 {
                write!(buf, ", ")?;
            }
            self.write_name(buf, p)?;
        }
        write!(buf, "}}")
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Decompose nested applications into `(head, [arg1, arg2, ...])`.
pub(super) fn collect_app(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut e = expr;
    while let Expr::App(f, a) = e {
        args.push(a.as_ref());
        e = f;
    }
    args.reverse();
    (e, args)
}

/// Convert a universe level to a natural number, if it is a concrete numeral.
pub(super) fn level_to_nat(level: &Level) -> Option<u32> {
    match level {
        Level::Zero => Some(0),
        Level::Succ(inner) => level_to_nat(inner).map(|n| n + 1),
        _ => None,
    }
}
