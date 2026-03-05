//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::ast_impl::*;
use std::fmt::{self, Write};

/// Parenthesization mode.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum ParensMode {
    /// Only add parentheses when needed based on precedence.
    #[default]
    Minimal,
    /// Always add parentheses for compound expressions.
    Full,
}
/// A simple column layout calculator.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ColumnLayout {
    /// Column widths
    pub widths: Vec<usize>,
}
impl ColumnLayout {
    /// Create from a list of column widths.
    #[allow(dead_code)]
    pub fn new(widths: Vec<usize>) -> Self {
        ColumnLayout { widths }
    }
    /// Total width including column separators.
    #[allow(dead_code)]
    pub fn total_width(&self, sep_width: usize) -> usize {
        let sum: usize = self.widths.iter().sum();
        sum + sep_width * self.widths.len().saturating_sub(1)
    }
    /// Number of columns.
    #[allow(dead_code)]
    pub fn num_cols(&self) -> usize {
        self.widths.len()
    }
}
/// Configuration for the pretty printer.
#[derive(Clone, Debug)]
pub struct PrettyConfig {
    /// Maximum line width before breaking (default 100).
    pub max_width: usize,
    /// Number of spaces per indentation level (default 2).
    pub indent_size: usize,
    /// Whether to show implicit arguments.
    pub show_implicit: bool,
    /// Whether to show universe annotations.
    pub show_universes: bool,
    /// Whether to use unicode symbols (e.g. `->` vs `\u{2192}`).
    pub use_unicode: bool,
    /// Whether to try to use registered notations for App nodes.
    pub use_notation: bool,
    /// Parenthesization mode.
    pub parens_mode: ParensMode,
}
impl PrettyConfig {
    /// Create a new config with all defaults.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the maximum line width.
    #[allow(dead_code)]
    pub fn with_max_width(mut self, width: usize) -> Self {
        self.max_width = width;
        self
    }
    /// Set the indent size.
    #[allow(dead_code)]
    pub fn with_indent_size(mut self, size: usize) -> Self {
        self.indent_size = size;
        self
    }
    /// Enable or disable showing implicit arguments.
    #[allow(dead_code)]
    pub fn with_show_implicit(mut self, show: bool) -> Self {
        self.show_implicit = show;
        self
    }
    /// Enable or disable showing universe annotations.
    #[allow(dead_code)]
    pub fn with_show_universes(mut self, show: bool) -> Self {
        self.show_universes = show;
        self
    }
    /// Enable or disable unicode symbols.
    #[allow(dead_code)]
    pub fn with_unicode(mut self, unicode: bool) -> Self {
        self.use_unicode = unicode;
        self
    }
    /// Enable or disable notation-aware printing.
    #[allow(dead_code)]
    pub fn with_notation(mut self, notation: bool) -> Self {
        self.use_notation = notation;
        self
    }
    /// Set the parenthesization mode.
    #[allow(dead_code)]
    pub fn with_parens_mode(mut self, mode: ParensMode) -> Self {
        self.parens_mode = mode;
        self
    }
}
/// A registered notation for notation-aware printing.
#[derive(Clone, Debug)]
#[allow(dead_code)]
struct NotationEntry {
    /// The function name this notation applies to (e.g. "HAdd.hAdd").
    func_name: String,
    /// The operator symbol to display (e.g. "+").
    symbol: String,
    /// Precedence of this operator.
    precedence: u32,
    /// Whether this is a binary infix operator.
    is_infix: bool,
}
/// Pretty printer for surface expressions.
pub struct PrettyPrinter {
    /// Indentation level
    indent: usize,
    /// Output buffer
    buffer: String,
    /// Configuration
    pub(super) config: PrettyConfig,
    /// Registered notations for notation-aware printing
    #[allow(dead_code)]
    notations: Vec<NotationEntry>,
}
impl PrettyPrinter {
    /// Create a new pretty printer with default configuration.
    pub fn new() -> Self {
        Self {
            indent: 0,
            buffer: String::new(),
            config: PrettyConfig::default(),
            notations: Vec::new(),
        }
    }
    /// Create a pretty printer with the given configuration.
    #[allow(dead_code)]
    pub fn with_config(config: PrettyConfig) -> Self {
        Self {
            indent: 0,
            buffer: String::new(),
            config,
            notations: Vec::new(),
        }
    }
    /// Get the output buffer.
    pub fn output(self) -> String {
        self.buffer
    }
    /// Get the current length of the output buffer.
    #[allow(dead_code)]
    fn current_len(&self) -> usize {
        self.buffer.len()
    }
    /// Get the column position on the current line.
    #[allow(dead_code)]
    fn current_column(&self) -> usize {
        match self.buffer.rfind('\n') {
            Some(idx) => self.buffer.len() - idx - 1,
            None => self.buffer.len(),
        }
    }
    /// Register a notation for notation-aware printing.
    #[allow(dead_code)]
    pub fn register_notation(
        &mut self,
        func_name: &str,
        symbol: &str,
        precedence: u32,
        is_infix: bool,
    ) {
        self.notations.push(NotationEntry {
            func_name: func_name.to_string(),
            symbol: symbol.to_string(),
            precedence,
            is_infix,
        });
    }
    /// Return the arrow symbol based on configuration.
    fn arrow(&self) -> &str {
        if self.config.use_unicode {
            "\u{2192}"
        } else {
            "->"
        }
    }
    /// Return the forall symbol based on configuration.
    fn forall_sym(&self) -> &str {
        if self.config.use_unicode {
            "\u{2200}"
        } else {
            "forall"
        }
    }
    /// Return the lambda symbol based on configuration.
    fn lambda_sym(&self) -> &str {
        if self.config.use_unicode {
            "\u{03bb}"
        } else {
            "fun"
        }
    }
    /// Return the product symbol based on configuration.
    #[allow(dead_code)]
    fn prod_sym(&self) -> &str {
        if self.config.use_unicode {
            "\u{00d7}"
        } else {
            "Prod"
        }
    }
    /// Return the not symbol based on configuration.
    #[allow(dead_code)]
    fn not_sym(&self) -> &str {
        if self.config.use_unicode {
            "\u{00ac}"
        } else {
            "Not"
        }
    }
    /// Write indentation at the current level.
    fn write_indent(&mut self) {
        for _ in 0..(self.indent * self.config.indent_size) {
            self.buffer.push(' ');
        }
    }
    /// Increase indentation.
    fn increase_indent(&mut self) {
        self.indent += 1;
    }
    /// Decrease indentation.
    fn decrease_indent(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }
    /// Try to print an expression on a single line. Returns the string
    /// if it fits within `max_width` from the current column, otherwise `None`.
    #[allow(dead_code)]
    pub fn try_single_line(&mut self, expr: &SurfaceExpr) -> Option<String> {
        let mut sub = PrettyPrinter::with_config(self.config.clone());
        sub.notations = self.notations.clone();
        if sub.print_expr(expr).is_err() {
            return None;
        }
        let result = sub.output();
        let col = self.current_column();
        if col + result.len() <= self.config.max_width {
            Some(result)
        } else {
            None
        }
    }
    /// Try to fit an expression on a single line; if not, use multi-line
    /// formatting with the given indentation.
    #[allow(dead_code)]
    fn print_expr_auto(&mut self, expr: &SurfaceExpr) -> fmt::Result {
        if let Some(s) = self.try_single_line(expr) {
            write!(self.buffer, "{}", s)
        } else {
            self.print_expr_multiline(expr)
        }
    }
    /// Print an expression using multi-line formatting.
    #[allow(dead_code)]
    fn print_expr_multiline(&mut self, expr: &SurfaceExpr) -> fmt::Result {
        match expr {
            SurfaceExpr::Lam(binders, body) => {
                let sym = self.lambda_sym().to_string();
                write!(self.buffer, "{}", sym)?;
                for binder in binders {
                    self.print_binder(binder)?;
                }
                writeln!(self.buffer, " =>")?;
                self.increase_indent();
                self.write_indent();
                self.print_expr(&body.value)?;
                self.decrease_indent();
                Ok(())
            }
            SurfaceExpr::Let(name, ty, val, body) => {
                write!(self.buffer, "let {}", name)?;
                if let Some(ty) = ty {
                    write!(self.buffer, " : ")?;
                    self.print_expr(&ty.value)?;
                }
                write!(self.buffer, " := ")?;
                self.print_expr(&val.value)?;
                writeln!(self.buffer)?;
                self.write_indent();
                write!(self.buffer, "in ")?;
                self.print_expr(&body.value)
            }
            SurfaceExpr::If(c, t, e) => {
                write!(self.buffer, "if ")?;
                self.print_expr(&c.value)?;
                writeln!(self.buffer)?;
                self.increase_indent();
                self.write_indent();
                write!(self.buffer, "then ")?;
                self.print_expr(&t.value)?;
                writeln!(self.buffer)?;
                self.write_indent();
                write!(self.buffer, "else ")?;
                self.print_expr(&e.value)?;
                self.decrease_indent();
                Ok(())
            }
            _ => self.print_expr(expr),
        }
    }
    /// Print a single binder.
    pub(super) fn print_binder(&mut self, binder: &Binder) -> fmt::Result {
        let (open, close) = match binder.info {
            BinderKind::Default => ("(", ")"),
            BinderKind::Implicit => {
                if !self.config.show_implicit {
                    return Ok(());
                }
                ("{", "}")
            }
            BinderKind::Instance => ("[", "]"),
            BinderKind::StrictImplicit => ("{{", "}}"),
        };
        write!(self.buffer, " {}{}", open, binder.name)?;
        if let Some(ty) = &binder.ty {
            write!(self.buffer, " : ")?;
            self.print_expr(&ty.value)?;
        }
        write!(self.buffer, "{}", close)
    }
    /// Print a binder always, regardless of implicit settings.
    fn print_binder_always(&mut self, binder: &Binder) -> fmt::Result {
        let (open, close) = match binder.info {
            BinderKind::Default => ("(", ")"),
            BinderKind::Implicit => ("{", "}"),
            BinderKind::Instance => ("[", "]"),
            BinderKind::StrictImplicit => ("{{", "}}"),
        };
        write!(self.buffer, " {}{}", open, binder.name)?;
        if let Some(ty) = &binder.ty {
            write!(self.buffer, " : ")?;
            self.print_expr(&ty.value)?;
        }
        write!(self.buffer, "{}", close)
    }
    /// Get the precedence of an expression.
    #[allow(dead_code)]
    fn expr_prec(&self, expr: &SurfaceExpr) -> u32 {
        match expr {
            SurfaceExpr::Sort(_)
            | SurfaceExpr::Var(_)
            | SurfaceExpr::Lit(_)
            | SurfaceExpr::Hole
            | SurfaceExpr::ListLit(_)
            | SurfaceExpr::Tuple(_)
            | SurfaceExpr::AnonymousCtor(_) => prec::ATOM,
            SurfaceExpr::Proj(_, _) => prec::ATOM,
            SurfaceExpr::App(_, _) => prec::APP,
            SurfaceExpr::Ann(_, _) => prec::ATOM,
            SurfaceExpr::Pi(binders, _) => {
                if binders.len() == 1 && binders[0].name == "_" {
                    prec::ARROW
                } else {
                    prec::LAMBDA
                }
            }
            SurfaceExpr::Lam(_, _) => prec::LAMBDA,
            SurfaceExpr::Let(_, _, _, _) => prec::LET,
            SurfaceExpr::If(_, _, _) => prec::IF,
            SurfaceExpr::Match(_, _) => prec::LET,
            SurfaceExpr::Do(_) => prec::LET,
            SurfaceExpr::Have(_, _, _, _) => prec::LET,
            SurfaceExpr::Suffices(_, _, _) => prec::LET,
            SurfaceExpr::Show(_, _) => prec::LET,
            SurfaceExpr::NamedArg(_, _, _) => prec::APP,
            SurfaceExpr::Return(_) => prec::LET,
            SurfaceExpr::StringInterp(_) => prec::ATOM,
            SurfaceExpr::Range(_, _) => prec::LET,
            SurfaceExpr::ByTactic(_) => prec::LET,
            SurfaceExpr::Calc(_) => prec::LET,
        }
    }
    /// Print an expression with precedence-aware parenthesization.
    ///
    /// If the expression's own precedence is less than `outer_prec`,
    /// parentheses are added.
    #[allow(dead_code)]
    pub fn print_expr_prec(&mut self, expr: &SurfaceExpr, outer_prec: u32) -> fmt::Result {
        let inner_prec = self.expr_prec(expr);
        let needs_parens = match self.config.parens_mode {
            ParensMode::Full => !matches!(
                expr,
                SurfaceExpr::Sort(_)
                    | SurfaceExpr::Var(_)
                    | SurfaceExpr::Lit(_)
                    | SurfaceExpr::Hole
            ),
            ParensMode::Minimal => inner_prec < outer_prec,
        };
        if needs_parens {
            write!(self.buffer, "(")?;
            self.print_expr(expr)?;
            write!(self.buffer, ")")
        } else {
            self.print_expr(expr)
        }
    }
    /// Pretty print a pattern.
    pub fn print_pattern(&mut self, pat: &Pattern) -> fmt::Result {
        match pat {
            Pattern::Wild => write!(self.buffer, "_"),
            Pattern::Var(name) => write!(self.buffer, "{}", name),
            Pattern::Ctor(name, args) => {
                if args.is_empty() {
                    write!(self.buffer, "{}", name)
                } else {
                    write!(self.buffer, "{}", name)?;
                    for arg in args {
                        write!(self.buffer, " ")?;
                        self.print_pattern_atom(&arg.value)?;
                    }
                    Ok(())
                }
            }
            Pattern::Lit(lit) => write!(self.buffer, "{}", lit),
            Pattern::Or(left, right) => {
                self.print_pattern(&left.value)?;
                write!(self.buffer, " | ")?;
                self.print_pattern(&right.value)
            }
        }
    }
    /// Print an atomic pattern (parenthesize compound patterns).
    fn print_pattern_atom(&mut self, pat: &Pattern) -> fmt::Result {
        match pat {
            Pattern::Wild | Pattern::Var(_) | Pattern::Lit(_) => self.print_pattern(pat),
            Pattern::Ctor(_, args) if args.is_empty() => self.print_pattern(pat),
            _ => {
                write!(self.buffer, "(")?;
                self.print_pattern(pat)?;
                write!(self.buffer, ")")
            }
        }
    }
    /// Pretty print a match expression with aligned arms.
    #[allow(dead_code)]
    pub fn print_match(&mut self, scrutinee: &SurfaceExpr, arms: &[MatchArm]) -> fmt::Result {
        write!(self.buffer, "match ")?;
        self.print_expr(scrutinee)?;
        write!(self.buffer, " with")?;
        for arm in arms {
            writeln!(self.buffer)?;
            self.write_indent();
            write!(self.buffer, "| ")?;
            self.print_pattern(&arm.pattern.value)?;
            if let Some(guard) = &arm.guard {
                write!(self.buffer, " when ")?;
                self.print_expr(&guard.value)?;
            }
            write!(self.buffer, " => ")?;
            self.print_expr(&arm.rhs.value)?;
        }
        Ok(())
    }
    /// Pretty print a do block with indented actions.
    #[allow(dead_code)]
    pub fn print_do(&mut self, actions: &[DoAction]) -> fmt::Result {
        writeln!(self.buffer, "do")?;
        self.increase_indent();
        for action in actions {
            self.write_indent();
            match action {
                DoAction::Let(name, expr) => {
                    write!(self.buffer, "let {} := ", name)?;
                    self.print_expr(&expr.value)?;
                }
                DoAction::LetTyped(name, ty, expr) => {
                    write!(self.buffer, "let {} : ", name)?;
                    self.print_expr(&ty.value)?;
                    write!(self.buffer, " := ")?;
                    self.print_expr(&expr.value)?;
                }
                DoAction::Bind(name, expr) => {
                    let la = self.left_arrow().to_string();
                    write!(self.buffer, "let {} {} ", name, la)?;
                    self.print_expr(&expr.value)?;
                }
                DoAction::Expr(expr) => {
                    self.print_expr(&expr.value)?;
                }
                DoAction::Return(expr) => {
                    write!(self.buffer, "return ")?;
                    self.print_expr(&expr.value)?;
                }
            }
            writeln!(self.buffer)?;
        }
        self.decrease_indent();
        Ok(())
    }
    /// Return the left arrow symbol based on configuration.
    fn left_arrow(&self) -> &str {
        if self.config.use_unicode {
            "\u{2190}"
        } else {
            "<-"
        }
    }
    /// Pretty print an inductive type declaration.
    #[allow(dead_code)]
    pub fn print_inductive(
        &mut self,
        name: &str,
        univ_params: &[String],
        params: &[Binder],
        ty: &SurfaceExpr,
        ctors: &[Constructor],
    ) -> fmt::Result {
        write!(self.buffer, "inductive {}", name)?;
        if !univ_params.is_empty() {
            write!(self.buffer, ".{{{}}}", univ_params.join(", "))?;
        }
        for param in params {
            self.print_binder_always(param)?;
        }
        write!(self.buffer, " : ")?;
        self.print_expr(ty)?;
        write!(self.buffer, " where")?;
        for ctor in ctors {
            writeln!(self.buffer)?;
            self.write_indent();
            write!(self.buffer, "| {} : ", ctor.name)?;
            self.print_expr(&ctor.ty.value)?;
        }
        Ok(())
    }
    /// Pretty print a surface expression.
    pub fn print_expr(&mut self, expr: &SurfaceExpr) -> fmt::Result {
        match expr {
            SurfaceExpr::Sort(sort) => {
                if !self.config.show_universes {
                    match sort {
                        SortKind::Type | SortKind::Prop => {
                            write!(self.buffer, "{}", sort)
                        }
                        SortKind::TypeU(_) => write!(self.buffer, "Type"),
                        SortKind::SortU(_) => write!(self.buffer, "Sort"),
                    }
                } else {
                    write!(self.buffer, "{}", sort)
                }
            }
            SurfaceExpr::Var(name) => write!(self.buffer, "{}", name),
            SurfaceExpr::App(fun, arg) => {
                self.print_expr(&fun.value)?;
                write!(self.buffer, " ")?;
                self.print_atom(&arg.value)
            }
            SurfaceExpr::Lam(binders, body) => {
                let sym = self.lambda_sym().to_string();
                write!(self.buffer, "{}", sym)?;
                for binder in binders {
                    self.print_binder_always(binder)?;
                }
                write!(self.buffer, " => ")?;
                self.print_expr(&body.value)
            }
            SurfaceExpr::Pi(binders, body) => {
                if binders.len() == 1 && binders[0].name == "_" {
                    if let Some(ty) = &binders[0].ty {
                        self.print_atom(&ty.value)?;
                    }
                    let arr = self.arrow().to_string();
                    write!(self.buffer, " {} ", arr)?;
                    self.print_expr(&body.value)
                } else {
                    let sym = self.forall_sym().to_string();
                    write!(self.buffer, "{}", sym)?;
                    for binder in binders {
                        self.print_binder_always(binder)?;
                    }
                    write!(self.buffer, ", ")?;
                    self.print_expr(&body.value)
                }
            }
            SurfaceExpr::Let(name, ty, val, body) => {
                write!(self.buffer, "let {} ", name)?;
                if let Some(ty) = ty {
                    write!(self.buffer, ": ")?;
                    self.print_expr(&ty.value)?;
                    write!(self.buffer, " ")?;
                }
                write!(self.buffer, ":= ")?;
                self.print_expr(&val.value)?;
                write!(self.buffer, " in ")?;
                self.print_expr(&body.value)
            }
            SurfaceExpr::Lit(lit) => write!(self.buffer, "{}", lit),
            SurfaceExpr::Ann(expr, ty) => {
                write!(self.buffer, "(")?;
                self.print_expr(&expr.value)?;
                write!(self.buffer, " : ")?;
                self.print_expr(&ty.value)?;
                write!(self.buffer, ")")
            }
            SurfaceExpr::Hole => write!(self.buffer, "_"),
            SurfaceExpr::Proj(expr, field) => {
                self.print_atom(&expr.value)?;
                write!(self.buffer, ".{}", field)
            }
            SurfaceExpr::If(c, t, e) => {
                write!(self.buffer, "if ")?;
                self.print_expr(&c.value)?;
                write!(self.buffer, " then ")?;
                self.print_expr(&t.value)?;
                write!(self.buffer, " else ")?;
                self.print_expr(&e.value)
            }
            SurfaceExpr::Match(scrutinee, arms) => self.print_match(&scrutinee.value, arms),
            SurfaceExpr::Do(actions) => self.print_do(actions),
            SurfaceExpr::Have(name, ty, proof, body) => {
                write!(self.buffer, "have {} : ", name)?;
                self.print_expr(&ty.value)?;
                write!(self.buffer, " := ")?;
                self.print_expr(&proof.value)?;
                write!(self.buffer, "; ")?;
                self.print_expr(&body.value)
            }
            SurfaceExpr::Suffices(name, ty, body) => {
                write!(self.buffer, "suffices {} : ", name)?;
                self.print_expr(&ty.value)?;
                write!(self.buffer, " from ")?;
                self.print_expr(&body.value)
            }
            SurfaceExpr::Show(ty, proof) => {
                write!(self.buffer, "show ")?;
                self.print_expr(&ty.value)?;
                write!(self.buffer, " from ")?;
                self.print_expr(&proof.value)
            }
            SurfaceExpr::NamedArg(func, name, arg) => {
                self.print_expr(&func.value)?;
                write!(self.buffer, " ({} := ", name)?;
                self.print_expr(&arg.value)?;
                write!(self.buffer, ")")
            }
            SurfaceExpr::AnonymousCtor(fields) => {
                write!(self.buffer, "<")?;
                for (i, f) in fields.iter().enumerate() {
                    if i > 0 {
                        write!(self.buffer, ", ")?;
                    }
                    self.print_expr(&f.value)?;
                }
                write!(self.buffer, ">")
            }
            SurfaceExpr::ListLit(elems) => {
                write!(self.buffer, "[")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(self.buffer, ", ")?;
                    }
                    self.print_expr(&e.value)?;
                }
                write!(self.buffer, "]")
            }
            SurfaceExpr::Tuple(elems) => {
                write!(self.buffer, "(")?;
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        write!(self.buffer, ", ")?;
                    }
                    self.print_expr(&e.value)?;
                }
                write!(self.buffer, ")")
            }
            SurfaceExpr::Return(expr) => {
                write!(self.buffer, "return ")?;
                self.print_expr(&expr.value)
            }
            SurfaceExpr::StringInterp(_parts) => write!(self.buffer, "s!\"...\""),
            SurfaceExpr::Range(lo, hi) => {
                if let Some(lo) = lo {
                    self.print_expr(&lo.value)?;
                }
                write!(self.buffer, "..")?;
                if let Some(hi) = hi {
                    self.print_expr(&hi.value)?;
                }
                Ok(())
            }
            SurfaceExpr::ByTactic(tactics) => {
                write!(self.buffer, "by")?;
                for (i, t) in tactics.iter().enumerate() {
                    if i > 0 {
                        write!(self.buffer, ";")?;
                    }
                    write!(self.buffer, " {}", t.value)?;
                }
                Ok(())
            }
            SurfaceExpr::Calc(steps) => {
                write!(self.buffer, "calc")?;
                for step in steps {
                    write!(self.buffer, "\n  ")?;
                    self.print_expr(&step.lhs.value)?;
                    write!(self.buffer, " {} ", step.rel)?;
                    self.print_expr(&step.rhs.value)?;
                    write!(self.buffer, " := ")?;
                    self.print_expr(&step.proof.value)?;
                }
                Ok(())
            }
        }
    }
    /// Print an atomic expression (parenthesize if needed).
    fn print_atom(&mut self, expr: &SurfaceExpr) -> fmt::Result {
        match expr {
            SurfaceExpr::Sort(_)
            | SurfaceExpr::Var(_)
            | SurfaceExpr::Lit(_)
            | SurfaceExpr::Hole => self.print_expr(expr),
            _ => {
                write!(self.buffer, "(")?;
                self.print_expr(expr)?;
                write!(self.buffer, ")")
            }
        }
    }
    /// Pretty print a declaration.
    pub fn print_decl(&mut self, decl: &Decl) -> fmt::Result {
        match decl {
            Decl::Axiom {
                name,
                univ_params,
                ty,
                ..
            } => {
                write!(self.buffer, "axiom {}", name)?;
                if !univ_params.is_empty() {
                    write!(self.buffer, " {{{}}}", univ_params.join(", "))?;
                }
                write!(self.buffer, " : ")?;
                self.print_expr(&ty.value)
            }
            Decl::Definition {
                name,
                univ_params,
                ty,
                val,
                ..
            } => {
                write!(self.buffer, "definition {}", name)?;
                if !univ_params.is_empty() {
                    write!(self.buffer, " {{{}}}", univ_params.join(", "))?;
                }
                if let Some(ty) = ty {
                    write!(self.buffer, " : ")?;
                    self.print_expr(&ty.value)?;
                }
                write!(self.buffer, " := ")?;
                self.print_expr(&val.value)
            }
            Decl::Theorem {
                name,
                univ_params,
                ty,
                proof,
                ..
            } => {
                write!(self.buffer, "theorem {}", name)?;
                if !univ_params.is_empty() {
                    write!(self.buffer, " {{{}}}", univ_params.join(", "))?;
                }
                write!(self.buffer, " : ")?;
                self.print_expr(&ty.value)?;
                write!(self.buffer, " := ")?;
                self.print_expr(&proof.value)
            }
            Decl::Inductive {
                name,
                univ_params,
                params,
                ty,
                ctors,
                ..
            } => self.print_inductive(name, univ_params, params, &ty.value, ctors),
            Decl::Import { path } => write!(self.buffer, "import {}", path.join(".")),
            Decl::Namespace { name, .. } => write!(self.buffer, "namespace {} ...", name),
            Decl::Structure {
                name,
                univ_params,
                extends,
                fields,
            } => self.print_structure(name, univ_params, extends, fields),
            Decl::ClassDecl {
                name,
                univ_params,
                extends,
                fields,
            } => self.print_class(name, univ_params, extends, fields),
            Decl::InstanceDecl { class_name, .. } => {
                write!(self.buffer, "instance {} ...", class_name)
            }
            Decl::SectionDecl { name, .. } => write!(self.buffer, "section {} ...", name),
            Decl::Variable { binders } => {
                write!(self.buffer, "variable")?;
                for b in binders {
                    self.print_binder_always(b)?;
                }
                Ok(())
            }
            Decl::Open { path, names } => {
                write!(self.buffer, "open {}", path.join("."))?;
                if !names.is_empty() {
                    write!(self.buffer, " ({})", names.join(", "))?;
                }
                Ok(())
            }
            Decl::Attribute { attrs, decl } => {
                write!(self.buffer, "@[{}] ", attrs.join(", "))?;
                self.print_decl(&decl.value)
            }
            Decl::HashCmd { cmd, arg } => {
                write!(self.buffer, "#{} ", cmd)?;
                self.print_expr(&arg.value)
            }
            Decl::Mutual { decls } => {
                write!(self.buffer, "mutual")?;
                for d in decls {
                    writeln!(self.buffer)?;
                    self.print_decl(&d.value)?;
                }
                writeln!(self.buffer)?;
                write!(self.buffer, "end")
            }
            Decl::Derive {
                instances,
                type_name,
            } => {
                write!(
                    self.buffer,
                    "deriving {} for {}",
                    instances.join(", "),
                    type_name
                )
            }
            Decl::NotationDecl {
                kind,
                prec,
                name,
                notation,
            } => {
                write!(self.buffer, "{}", kind)?;
                if let Some(p) = prec {
                    write!(self.buffer, " {}", p)?;
                }
                write!(self.buffer, " {} := {}", name, notation)
            }
            Decl::Universe { names } => {
                write!(self.buffer, "universe {}", names.join(" "))
            }
        }
    }
    /// Pretty print a structure declaration.
    #[allow(dead_code)]
    fn print_structure(
        &mut self,
        name: &str,
        univ_params: &[String],
        extends: &[String],
        fields: &[FieldDecl],
    ) -> fmt::Result {
        write!(self.buffer, "structure {}", name)?;
        if !univ_params.is_empty() {
            write!(self.buffer, " {{{}}}", univ_params.join(", "))?;
        }
        if !extends.is_empty() {
            write!(self.buffer, " extends {}", extends.join(", "))?;
        }
        write!(self.buffer, " where")?;
        self.increase_indent();
        for field in fields {
            writeln!(self.buffer)?;
            self.write_indent();
            write!(self.buffer, "{} : ", field.name)?;
            self.print_expr(&field.ty.value)?;
            if let Some(default) = &field.default {
                write!(self.buffer, " := ")?;
                self.print_expr(&default.value)?;
            }
        }
        self.decrease_indent();
        Ok(())
    }
    /// Pretty print a class declaration.
    #[allow(dead_code)]
    fn print_class(
        &mut self,
        name: &str,
        univ_params: &[String],
        extends: &[String],
        fields: &[FieldDecl],
    ) -> fmt::Result {
        write!(self.buffer, "class {}", name)?;
        if !univ_params.is_empty() {
            write!(self.buffer, " {{{}}}", univ_params.join(", "))?;
        }
        if !extends.is_empty() {
            write!(self.buffer, " extends {}", extends.join(", "))?;
        }
        write!(self.buffer, " where")?;
        self.increase_indent();
        for field in fields {
            writeln!(self.buffer)?;
            self.write_indent();
            write!(self.buffer, "{} : ", field.name)?;
            self.print_expr(&field.ty.value)?;
            if let Some(default) = &field.default {
                write!(self.buffer, " := ")?;
                self.print_expr(&default.value)?;
            }
        }
        self.decrease_indent();
        Ok(())
    }
}
/// A simple indentation manager.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct IndentManager {
    /// Current indent level
    pub level: usize,
    /// Spaces per indent level
    pub tab_width: usize,
}
impl IndentManager {
    /// Create a new manager.
    #[allow(dead_code)]
    pub fn new(tab_width: usize) -> Self {
        IndentManager {
            level: 0,
            tab_width,
        }
    }
    /// Indent by one level.
    #[allow(dead_code)]
    pub fn indent(&mut self) {
        self.level += 1;
    }
    /// Dedent by one level.
    #[allow(dead_code)]
    pub fn dedent(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }
    /// Returns the current indent string.
    #[allow(dead_code)]
    pub fn current(&self) -> String {
        " ".repeat(self.level * self.tab_width)
    }
    /// Apply indentation to a multi-line string.
    #[allow(dead_code)]
    pub fn apply_to(&self, s: &str) -> String {
        let prefix = self.current();
        s.lines()
            .map(|line| format!("{}{}", prefix, line))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// A breadcrumb trail for pretty-printing context.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct BreadcrumbTrail {
    /// Breadcrumb labels
    pub crumbs: Vec<String>,
}
impl BreadcrumbTrail {
    /// Create a new empty trail.
    #[allow(dead_code)]
    pub fn new() -> Self {
        BreadcrumbTrail { crumbs: Vec::new() }
    }
    /// Push a crumb.
    #[allow(dead_code)]
    pub fn push(&mut self, label: &str) {
        self.crumbs.push(label.to_string());
    }
    /// Pop the last crumb.
    #[allow(dead_code)]
    pub fn pop(&mut self) {
        self.crumbs.pop();
    }
    /// Format as a path string.
    #[allow(dead_code)]
    pub fn format(&self) -> String {
        self.crumbs.join(" > ")
    }
}
/// A ribbon formatter with line-break decisions.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct DocFormatter {
    /// Line width limit
    pub width: usize,
    /// Current column
    pub col: usize,
    /// Output buffer
    pub out: String,
}
impl DocFormatter {
    /// Create a new formatter.
    #[allow(dead_code)]
    pub fn new(width: usize) -> Self {
        DocFormatter {
            width,
            col: 0,
            out: String::new(),
        }
    }
    /// Write text.
    #[allow(dead_code)]
    pub fn write_text(&mut self, s: &str) {
        self.out.push_str(s);
        self.col += s.chars().count();
    }
    /// Write a newline with indent.
    #[allow(dead_code)]
    pub fn newline(&mut self, indent: usize) {
        self.out.push('\n');
        self.out.push_str(&" ".repeat(indent));
        self.col = indent;
    }
    /// Returns the current output.
    #[allow(dead_code)]
    pub fn finish(self) -> String {
        self.out
    }
}
/// A column-aligned table formatter.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TableFormatter {
    /// Rows of the table
    pub rows: Vec<Vec<String>>,
    /// Column separator
    pub sep: String,
}
impl TableFormatter {
    /// Create a new table formatter.
    #[allow(dead_code)]
    pub fn new(sep: &str) -> Self {
        TableFormatter {
            rows: Vec::new(),
            sep: sep.to_string(),
        }
    }
    /// Add a row.
    #[allow(dead_code)]
    pub fn add_row(&mut self, row: Vec<&str>) {
        self.rows
            .push(row.into_iter().map(|s| s.to_string()).collect());
    }
    /// Compute column widths.
    #[allow(dead_code)]
    pub fn col_widths(&self) -> Vec<usize> {
        if self.rows.is_empty() {
            return Vec::new();
        }
        let ncols = self.rows.iter().map(|r| r.len()).max().unwrap_or(0);
        (0..ncols)
            .map(|c| {
                self.rows
                    .iter()
                    .filter_map(|r| r.get(c))
                    .map(|s| s.chars().count())
                    .max()
                    .unwrap_or(0)
            })
            .collect()
    }
    /// Render the table to a string.
    #[allow(dead_code)]
    pub fn render(&self) -> String {
        let widths = self.col_widths();
        let mut out = String::new();
        for row in &self.rows {
            let parts: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    let w = widths.get(i).copied().unwrap_or(0);
                    format!("{:width$}", cell, width = w)
                })
                .collect();
            out.push_str(&parts.join(&self.sep));
            out.push('\n');
        }
        out
    }
}
/// A syntax highlighter that wraps tokens in ANSI escape codes.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct AnsiHighlighter;
impl AnsiHighlighter {
    const RESET: &'static str = "\x1b[0m";
    const BOLD: &'static str = "\x1b[1m";
    const RED: &'static str = "\x1b[31m";
    const GREEN: &'static str = "\x1b[32m";
    #[allow(dead_code)]
    const YELLOW: &'static str = "\x1b[33m";
    const BLUE: &'static str = "\x1b[34m";
    /// Highlight a keyword.
    #[allow(dead_code)]
    pub fn keyword(s: &str) -> String {
        format!("{}{}{}", Self::BOLD, s, Self::RESET)
    }
    /// Highlight an error.
    #[allow(dead_code)]
    pub fn error(s: &str) -> String {
        format!("{}{}{}", Self::RED, s, Self::RESET)
    }
    /// Highlight a literal.
    #[allow(dead_code)]
    pub fn literal(s: &str) -> String {
        format!("{}{}{}", Self::GREEN, s, Self::RESET)
    }
    /// Highlight a type.
    #[allow(dead_code)]
    pub fn type_name(s: &str) -> String {
        format!("{}{}{}", Self::BLUE, s, Self::RESET)
    }
    /// Strip ANSI escape codes from a string.
    #[allow(dead_code)]
    pub fn strip_ansi(s: &str) -> String {
        let mut result = String::new();
        let mut in_escape = false;
        for c in s.chars() {
            if c == '\x1b' {
                in_escape = true;
            } else if in_escape {
                if c == 'm' {
                    in_escape = false;
                }
            } else {
                result.push(c);
            }
        }
        result
    }
}
/// A box model for aligned output.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct BoxModel {
    /// Width of this box
    pub width: usize,
    /// Lines of content
    pub lines: Vec<String>,
}
impl BoxModel {
    /// Create an empty box.
    #[allow(dead_code)]
    pub fn empty() -> Self {
        BoxModel {
            width: 0,
            lines: Vec::new(),
        }
    }
    /// Create a box from a single string.
    #[allow(dead_code)]
    pub fn of_str(s: &str) -> Self {
        let width = s.chars().count();
        BoxModel {
            width,
            lines: vec![s.to_string()],
        }
    }
    /// Stack two boxes vertically.
    #[allow(dead_code)]
    pub fn vstack(mut self, other: BoxModel) -> Self {
        self.width = self.width.max(other.width);
        self.lines.extend(other.lines);
        self
    }
    /// Place two boxes side by side.
    #[allow(dead_code)]
    pub fn hstack(self, other: BoxModel) -> Self {
        let h = self.lines.len().max(other.lines.len());
        let lw = self.width;
        let mut lines = Vec::new();
        for i in 0..h {
            let l = self.lines.get(i).cloned().unwrap_or_else(|| " ".repeat(lw));
            let r = other.lines.get(i).cloned().unwrap_or_default();
            let padded = format!("{:width$}{}", l, r, width = lw);
            lines.push(padded);
        }
        BoxModel {
            width: lw + other.width,
            lines,
        }
    }
    /// Render to a string.
    #[allow(dead_code)]
    pub fn render(&self) -> String {
        self.lines.join("\n")
    }
}
/// A document algebra node.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum DocNode {
    /// Empty document
    Empty,
    /// A text string
    Text(String),
    /// A newline followed by indentation
    Line(usize),
    /// Horizontal concatenation
    Cat(Box<DocNode>, Box<DocNode>),
    /// Nested indentation
    Indent(usize, Box<DocNode>),
    /// A group that prefers to be on one line
    Group(Box<DocNode>),
}
impl DocNode {
    /// Create a text node.
    #[allow(dead_code)]
    pub fn text(s: &str) -> Self {
        DocNode::Text(s.to_string())
    }
    /// Create a line break with indent level.
    #[allow(dead_code)]
    pub fn line(indent: usize) -> Self {
        DocNode::Line(indent)
    }
    /// Concatenate two docs.
    #[allow(dead_code)]
    pub fn cat(a: DocNode, b: DocNode) -> Self {
        DocNode::Cat(Box::new(a), Box::new(b))
    }
    /// Indent a document.
    #[allow(dead_code)]
    pub fn indent(n: usize, doc: DocNode) -> Self {
        DocNode::Indent(n, Box::new(doc))
    }
    /// Group a document.
    #[allow(dead_code)]
    pub fn group(doc: DocNode) -> Self {
        DocNode::Group(Box::new(doc))
    }
    /// Render the document to a string (simplified: ignores group optimization).
    #[allow(dead_code)]
    pub fn render(&self, width: usize) -> String {
        let mut out = String::new();
        render_doc(self, 0, width, &mut out);
        out
    }
}
