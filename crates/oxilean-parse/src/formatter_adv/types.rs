//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::ast_impl::*;
use std::collections::HashMap;

/// Configuration for AST formatting.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct FormatConfig {
    /// Maximum line width.
    pub max_width: usize,
    /// Indentation size.
    pub indent_size: i32,
    /// Whether to use Unicode symbols.
    #[allow(missing_docs)]
    pub use_unicode: bool,
    /// Whether to add spaces around operators.
    pub spaces_around_ops: bool,
    /// Whether to add blank lines between top-level declarations.
    pub blank_between_decls: bool,
    /// Whether to preserve existing comments.
    #[allow(missing_docs)]
    pub preserve_comments: bool,
    /// Whether to normalize trailing whitespace.
    pub normalize_whitespace: bool,
    /// Whether to use explicit parentheses.
    pub explicit_parens: bool,
}
/// Format context tracking.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FormatContext {
    pub indent: usize,
    pub width: usize,
    pub in_type: bool,
    pub in_proof: bool,
    #[allow(missing_docs)]
    pub prec: u8,
}
impl FormatContext {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(width: usize) -> Self {
        Self {
            indent: 0,
            width,
            in_type: false,
            in_proof: false,
            prec: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn indented(&self, n: usize) -> Self {
        Self {
            indent: self.indent + n,
            ..*self
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_prec(&self, prec: u8) -> Self {
        Self { prec, ..*self }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn in_type_mode(&self) -> Self {
        Self {
            in_type: true,
            ..*self
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn remaining_width(&self) -> usize {
        self.width.saturating_sub(self.indent)
    }
}
/// The layout mode for the Wadler-Lindig algorithm.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mode {
    /// Flat mode: no line breaks in the current group.
    Flat,
    /// Break mode: line breaks are active.
    Break,
}
/// Formatter configuration struct for central settings.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    pub line_width: usize,
    pub indent_size: usize,
    pub use_tabs: bool,
    pub ribbon_fraction: f64,
    #[allow(missing_docs)]
    pub max_blank_lines: usize,
    pub trailing_newline: bool,
    pub align_let_bindings: bool,
}
impl FormatterConfig {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn default_config() -> Self {
        Self {
            line_width: 100,
            indent_size: 2,
            use_tabs: false,
            ribbon_fraction: 0.6,
            max_blank_lines: 2,
            trailing_newline: true,
            align_let_bindings: false,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn compact() -> Self {
        Self {
            line_width: 120,
            indent_size: 4,
            use_tabs: false,
            ribbon_fraction: 0.8,
            max_blank_lines: 1,
            trailing_newline: true,
            align_let_bindings: true,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn indent_str(&self) -> String {
        if self.use_tabs {
            "\t".to_string()
        } else {
            " ".repeat(self.indent_size)
        }
    }
}
/// Represents the width of a single doc layout.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub struct DocWidth {
    pub flat: usize,
    pub max_line: usize,
    pub last_line: usize,
}
impl DocWidth {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn atom(w: usize) -> Self {
        Self {
            flat: w,
            max_line: w,
            last_line: w,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn newline(indent: usize) -> Self {
        Self {
            flat: 0,
            max_line: 0,
            last_line: indent,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn concat(a: Self, b: Self) -> Self {
        Self {
            flat: a.flat + b.flat,
            max_line: a.max_line.max(b.max_line),
            last_line: b.last_line,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn fits_in(&self, width: usize) -> bool {
        self.max_line <= width
    }
}
/// Tracks all formatting decisions made during a single pass.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FormatDecisionLog {
    decisions: Vec<FormatDecision>,
}
impl FormatDecisionLog {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            decisions: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record(
        &mut self,
        location: impl Into<String>,
        flat_chosen: bool,
        flat_len: usize,
        avail: usize,
    ) {
        self.decisions.push(FormatDecision {
            location: location.into(),
            flat_chosen,
            flat_len,
            available_width: avail,
        });
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn flat_fraction(&self) -> f64 {
        if self.decisions.is_empty() {
            return 0.0;
        }
        let flat = self.decisions.iter().filter(|d| d.flat_chosen).count();
        flat as f64 / self.decisions.len() as f64
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn count(&self) -> usize {
        self.decisions.len()
    }
}
/// Annotation for syntax highlighting or metadata.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum Annotation {
    /// Keyword (def, theorem, fun, etc.)
    Keyword,
    /// Operator (+, -, *, etc.)
    Operator,
    /// Identifier
    Identifier,
    /// Type name
    TypeName,
    /// Literal value
    Literal,
    /// Comment
    Comment,
    /// String literal
    StringLit,
    /// Custom annotation
    Custom(String),
}
/// Priority for pretty-printing choices.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FormatPriority {
    Low = 0,
    Medium = 1,
    High = 2,
}
/// Output format for a declaration, with variants.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub enum DeclFormat {
    OneLiner(String),
    MultiLine(Vec<String>),
    Compact(String),
}
impl DeclFormat {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn render(&self) -> String {
        match self {
            DeclFormat::OneLiner(s) => s.clone(),
            DeclFormat::MultiLine(lines) => lines.join("\n"),
            DeclFormat::Compact(s) => s.clone(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn line_count(&self) -> usize {
        match self {
            DeclFormat::OneLiner(_) => 1,
            DeclFormat::MultiLine(lines) => lines.len(),
            DeclFormat::Compact(_) => 1,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_one_liner(&self) -> bool {
        matches!(self, DeclFormat::OneLiner(_))
    }
}
/// Render a tree-like structure for debugging.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TreeRenderer {
    indent_unit: usize,
    lines: Vec<(usize, String)>,
}
impl TreeRenderer {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(indent_unit: usize) -> Self {
        Self {
            indent_unit,
            lines: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, depth: usize, label: impl Into<String>) {
        self.lines.push((depth, label.into()));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn render(&self) -> String {
        self.lines
            .iter()
            .map(|(d, label)| {
                let indent = " ".repeat(d * self.indent_unit);
                format!("{}|- {}", indent, label)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn node_count(&self) -> usize {
        self.lines.len()
    }
}
/// Converts OxiLean AST nodes to document elements.
#[allow(missing_docs)]
pub struct AstFormatter {
    /// Formatter configuration.
    pub config: FormatConfig,
    /// Preserved comments (offset -> comment text).
    pub comments: HashMap<usize, String>,
}
impl AstFormatter {
    /// Create a new formatter with default configuration.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            config: FormatConfig::default(),
            comments: HashMap::new(),
        }
    }
    /// Create a formatter with custom configuration.
    #[allow(missing_docs)]
    pub fn with_config(config: FormatConfig) -> Self {
        Self {
            config,
            comments: HashMap::new(),
        }
    }
    /// Register comments to be preserved during formatting.
    #[allow(missing_docs)]
    pub fn register_comments(&mut self, comments: HashMap<usize, String>) {
        self.comments = comments;
    }
    /// Format a surface expression to a Doc.
    #[allow(missing_docs)]
    pub fn format_expr(&self, expr: &SurfaceExpr) -> Doc {
        match expr {
            SurfaceExpr::Var(name) => ident(name),
            SurfaceExpr::Lit(lit) => self.format_literal(lit),
            SurfaceExpr::Hole => Doc::text("_"),
            SurfaceExpr::Sort(kind) => self.format_sort(kind),
            SurfaceExpr::App(f, arg) => {
                let f_doc = self.format_expr(&f.value);
                let arg_doc = self.format_expr(&arg.value);
                f_doc.space(arg_doc).group()
            }
            SurfaceExpr::Lam(binders, body) => {
                let mut doc = keyword("fun");
                for b in binders {
                    doc = doc.space(self.format_binder(b));
                }
                doc = doc.space(operator("=>"));
                doc = doc
                    .cat(Doc::SoftLine)
                    .cat(self.format_expr(&body.value).nest(self.config.indent_size));
                doc.group()
            }
            SurfaceExpr::Pi(binders, body) => {
                let arrow = "->";
                let mut binder_docs = Vec::new();
                for b in binders {
                    binder_docs.push(self.format_binder(b));
                }
                let binders_doc = hsep(&binder_docs);
                let body_doc = self.format_expr(&body.value);
                binders_doc.space(operator(arrow)).space(body_doc).group()
            }
            SurfaceExpr::Let(name, ty, val, body) => {
                let mut doc = keyword("let").space(ident(name));
                if let Some(ty_expr) = ty {
                    doc = doc
                        .space(operator(":"))
                        .space(self.format_expr(&ty_expr.value));
                }
                doc = doc
                    .space(operator(":="))
                    .cat(Doc::SoftLine)
                    .cat(self.format_expr(&val.value).nest(self.config.indent_size));
                doc = doc.cat(Doc::HardLine).cat(self.format_expr(&body.value));
                doc
            }
            SurfaceExpr::Ann(e, ty) => parens(
                self.format_expr(&e.value)
                    .space(operator(":"))
                    .space(self.format_expr(&ty.value)),
            ),
            SurfaceExpr::If(cond, then_e, else_e) => {
                let cond_doc = self.format_expr(&cond.value);
                let then_doc = self.format_expr(&then_e.value);
                let else_doc = self.format_expr(&else_e.value);
                keyword("if")
                    .space(cond_doc)
                    .space(keyword("then"))
                    .cat(Doc::SoftLine)
                    .cat(then_doc.nest(self.config.indent_size))
                    .cat(Doc::SoftLine)
                    .cat(keyword("else"))
                    .cat(Doc::SoftLine)
                    .cat(else_doc.nest(self.config.indent_size))
                    .group()
            }
            SurfaceExpr::Match(scrutinee, arms) => {
                let mut doc = keyword("match")
                    .space(self.format_expr(&scrutinee.value))
                    .space(keyword("with"));
                for arm in arms {
                    doc = doc
                        .cat(Doc::HardLine)
                        .cat(self.format_match_arm(arm).nest(self.config.indent_size));
                }
                doc
            }
            SurfaceExpr::ListLit(elems) => {
                if elems.is_empty() {
                    return Doc::text("[]");
                }
                let elem_docs: Vec<Doc> =
                    elems.iter().map(|e| self.format_expr(&e.value)).collect();
                brackets(intersperse(&elem_docs, Doc::text(", ").cat(Doc::SoftBreak))).group()
            }
            SurfaceExpr::Tuple(elems) => {
                let elem_docs: Vec<Doc> =
                    elems.iter().map(|e| self.format_expr(&e.value)).collect();
                parens(intersperse(&elem_docs, Doc::text(", ").cat(Doc::SoftBreak))).group()
            }
            SurfaceExpr::Proj(e, field) => self
                .format_expr(&e.value)
                .cat(Doc::text("."))
                .cat(ident(field)),
            SurfaceExpr::Have(name, ty, proof, body) => keyword("have")
                .space(ident(name))
                .space(operator(":"))
                .space(self.format_expr(&ty.value))
                .space(operator(":="))
                .cat(Doc::SoftLine)
                .cat(self.format_expr(&proof.value).nest(self.config.indent_size))
                .cat(Doc::HardLine)
                .cat(self.format_expr(&body.value)),
            SurfaceExpr::Do(actions) => {
                let mut doc = keyword("do");
                for action in actions {
                    doc = doc
                        .cat(Doc::HardLine)
                        .cat(self.format_do_action(action).nest(self.config.indent_size));
                }
                doc
            }
            SurfaceExpr::Return(e) => keyword("return").space(self.format_expr(&e.value)),
            SurfaceExpr::NamedArg(f, name, e) => self.format_expr(&f.value).space(parens(
                ident(name)
                    .space(operator(":="))
                    .space(self.format_expr(&e.value)),
            )),
            SurfaceExpr::Suffices(name, ty, body) => keyword("suffices")
                .space(ident(name))
                .space(operator(":"))
                .space(self.format_expr(&ty.value))
                .cat(Doc::HardLine)
                .cat(self.format_expr(&body.value)),
            SurfaceExpr::Show(ty, e) => keyword("show")
                .space(self.format_expr(&ty.value))
                .space(keyword("from"))
                .space(self.format_expr(&e.value)),
            SurfaceExpr::AnonymousCtor(fields) => {
                let field_docs: Vec<Doc> =
                    fields.iter().map(|f| self.format_expr(&f.value)).collect();
                Doc::text("<")
                    .cat(intersperse(&field_docs, Doc::text(", ")))
                    .cat(Doc::text(">"))
            }
            SurfaceExpr::ByTactic(tactics) => {
                let tac_docs: Vec<Doc> =
                    tactics.iter().map(|t| Doc::text(t.value.clone())).collect();
                keyword("by").space(intersperse(&tac_docs, Doc::text("; ")))
            }
            SurfaceExpr::Calc(steps) => {
                let mut doc = keyword("calc");
                for step in steps {
                    doc = doc.cat(Doc::HardLine).cat(
                        self.format_expr(&step.lhs.value)
                            .space(Doc::text(&step.rel))
                            .space(self.format_expr(&step.rhs.value))
                            .space(operator(":="))
                            .space(self.format_expr(&step.proof.value))
                            .nest(self.config.indent_size),
                    );
                }
                doc
            }
            SurfaceExpr::StringInterp(_parts) => Doc::text("s!\"...\""),
            SurfaceExpr::Range(start, end) => {
                let mut doc = Doc::Nil;
                if let Some(s) = start {
                    doc = self.format_expr(&s.value);
                }
                doc = doc.cat(Doc::text(".."));
                if let Some(e) = end {
                    doc = doc.cat(self.format_expr(&e.value));
                }
                doc
            }
        }
    }
    /// Format a literal.
    fn format_literal(&self, lit: &Literal) -> Doc {
        match lit {
            Literal::Nat(n) => Doc::text(n.to_string()).annotate(Annotation::Literal),
            Literal::Float(f) => Doc::text(f.to_string()).annotate(Annotation::Literal),
            Literal::String(s) => Doc::text(format!("\"{}\"", s)).annotate(Annotation::StringLit),
            Literal::Char(c) => Doc::text(format!("'{}'", c)).annotate(Annotation::Literal),
        }
    }
    /// Format a sort kind.
    fn format_sort(&self, kind: &SortKind) -> Doc {
        match kind {
            SortKind::Prop => keyword("Prop"),
            SortKind::Type => keyword("Type"),
            SortKind::TypeU(u) => keyword("Type").space(ident(u)),
            SortKind::SortU(u) => keyword("Sort").space(ident(u)),
        }
    }
    /// Format a binder.
    fn format_binder(&self, binder: &Binder) -> Doc {
        let (open, close) = match binder.info {
            BinderKind::Default => ("(", ")"),
            BinderKind::Implicit => ("{", "}"),
            BinderKind::StrictImplicit => ("{{", "}}"),
            BinderKind::Instance => ("[", "]"),
        };
        let mut doc = Doc::text(open).cat(ident(&binder.name));
        if let Some(ref ty) = binder.ty {
            doc = doc.space(operator(":")).space(self.format_expr(&ty.value));
        }
        doc.cat(Doc::text(close))
    }
    /// Format a match arm.
    fn format_match_arm(&self, arm: &MatchArm) -> Doc {
        let pat_doc = self.format_pattern(&arm.pattern.value);
        let mut doc = Doc::text("| ").cat(pat_doc);
        if let Some(ref guard) = arm.guard {
            doc = doc
                .space(keyword("when"))
                .space(self.format_expr(&guard.value));
        }
        doc = doc.space(operator("=>")).cat(Doc::SoftLine).cat(
            self.format_expr(&arm.rhs.value)
                .nest(self.config.indent_size),
        );
        doc
    }
    /// Format a pattern.
    fn format_pattern(&self, pat: &Pattern) -> Doc {
        match pat {
            Pattern::Wild => Doc::text("_"),
            Pattern::Var(name) => ident(name),
            Pattern::Ctor(name, args) => {
                if args.is_empty() {
                    ident(name)
                } else {
                    let arg_docs: Vec<Doc> =
                        args.iter().map(|a| self.format_pattern(&a.value)).collect();
                    ident(name).space(hsep(&arg_docs))
                }
            }
            Pattern::Lit(lit) => self.format_literal(lit),
            Pattern::Or(a, b) => self
                .format_pattern(&a.value)
                .space(operator("|"))
                .space(self.format_pattern(&b.value)),
        }
    }
    /// Format a do-notation action.
    fn format_do_action(&self, action: &DoAction) -> Doc {
        match action {
            DoAction::Let(name, val) => keyword("let")
                .space(ident(name))
                .space(operator(":="))
                .space(self.format_expr(&val.value)),
            DoAction::LetTyped(name, ty, val) => keyword("let")
                .space(ident(name))
                .space(operator(":"))
                .space(self.format_expr(&ty.value))
                .space(operator(":="))
                .space(self.format_expr(&val.value)),
            DoAction::Bind(name, val) => ident(name)
                .space(operator("<-"))
                .space(self.format_expr(&val.value)),
            DoAction::Expr(expr) => self.format_expr(&expr.value),
            DoAction::Return(expr) => keyword("return").space(self.format_expr(&expr.value)),
        }
    }
    /// Format a top-level declaration.
    #[allow(missing_docs)]
    pub fn format_decl(&self, decl: &Decl) -> Doc {
        match decl {
            Decl::Definition {
                name,
                univ_params,
                ty,
                val,
                attrs,
                ..
            } => {
                let mut doc = Doc::Nil;
                if !attrs.is_empty() {
                    let attr_docs: Vec<Doc> =
                        attrs.iter().map(|a| Doc::text(format!("{}", a))).collect();
                    doc = doc
                        .cat(Doc::text("@["))
                        .cat(intersperse(&attr_docs, Doc::text(", ")))
                        .cat(Doc::text("]"))
                        .cat(Doc::HardLine);
                }
                doc = doc.cat(keyword("def")).space(ident(name));
                if !univ_params.is_empty() {
                    let up_docs: Vec<Doc> = univ_params.iter().map(|u| ident(u)).collect();
                    doc = doc
                        .cat(Doc::text(".{"))
                        .cat(intersperse(&up_docs, Doc::text(", ")))
                        .cat(Doc::text("}"));
                }
                if let Some(ty_expr) = ty {
                    doc = doc
                        .space(operator(":"))
                        .space(self.format_expr(&ty_expr.value));
                }
                doc = doc
                    .space(operator(":="))
                    .cat(Doc::SoftLine)
                    .cat(self.format_expr(&val.value).nest(self.config.indent_size));
                doc.group()
            }
            Decl::Theorem {
                name,
                univ_params,
                ty,
                proof,
                attrs,
                ..
            } => {
                let mut doc = Doc::Nil;
                if !attrs.is_empty() {
                    let attr_docs: Vec<Doc> =
                        attrs.iter().map(|a| Doc::text(format!("{}", a))).collect();
                    doc = doc
                        .cat(Doc::text("@["))
                        .cat(intersperse(&attr_docs, Doc::text(", ")))
                        .cat(Doc::text("]"))
                        .cat(Doc::HardLine);
                }
                doc = doc.cat(keyword("theorem")).space(ident(name));
                if !univ_params.is_empty() {
                    let up_docs: Vec<Doc> = univ_params.iter().map(|u| ident(u)).collect();
                    doc = doc
                        .cat(Doc::text(".{"))
                        .cat(intersperse(&up_docs, Doc::text(", ")))
                        .cat(Doc::text("}"));
                }
                doc = doc
                    .space(operator(":"))
                    .space(self.format_expr(&ty.value))
                    .space(operator(":="))
                    .cat(Doc::SoftLine)
                    .cat(self.format_expr(&proof.value).nest(self.config.indent_size));
                doc.group()
            }
            Decl::Axiom { name, ty, .. } => keyword("axiom")
                .space(ident(name))
                .space(operator(":"))
                .space(self.format_expr(&ty.value)),
            Decl::Inductive {
                name,
                params,
                ty,
                ctors,
                ..
            } => {
                let mut doc = keyword("inductive").space(ident(name));
                for p in params {
                    doc = doc.space(self.format_binder(p));
                }
                doc = doc.space(operator(":")).space(self.format_expr(&ty.value));
                doc = doc.space(keyword("where"));
                for ctor in ctors {
                    doc = doc.cat(Doc::HardLine).cat(
                        Doc::text("| ")
                            .cat(ident(&ctor.name))
                            .space(operator(":"))
                            .space(self.format_expr(&ctor.ty.value))
                            .nest(self.config.indent_size),
                    );
                }
                doc
            }
            Decl::Structure {
                name,
                extends,
                fields,
                ..
            } => {
                let mut doc = keyword("structure").space(ident(name));
                if !extends.is_empty() {
                    doc = doc.space(keyword("extends"));
                    let ext_docs: Vec<Doc> = extends.iter().map(|e| ident(e)).collect();
                    doc = doc.space(intersperse(&ext_docs, Doc::text(", ")));
                }
                doc = doc.space(keyword("where"));
                for field in fields {
                    doc = doc.cat(Doc::HardLine).cat(
                        ident(&field.name)
                            .space(operator(":"))
                            .space(self.format_expr(&field.ty.value))
                            .nest(self.config.indent_size),
                    );
                }
                doc
            }
            Decl::Import { path } => keyword("import").space(Doc::text(path.join("."))),
            Decl::Namespace { name, decls } => {
                let mut doc = keyword("namespace").space(ident(name));
                for d in decls {
                    doc = doc
                        .cat(Doc::HardLine)
                        .cat(Doc::HardLine)
                        .cat(self.format_decl(&d.value));
                }
                doc = doc
                    .cat(Doc::HardLine)
                    .cat(keyword("end"))
                    .space(ident(name));
                doc
            }
            Decl::Open { path, names } => {
                let mut doc = keyword("open").space(Doc::text(path.join(".")));
                if !names.is_empty() {
                    let name_docs: Vec<Doc> = names.iter().map(|n| ident(n)).collect();
                    doc = doc.space(parens(intersperse(&name_docs, Doc::text(", "))));
                }
                doc
            }
            _ => Doc::text("<unsupported decl>"),
        }
    }
    /// Format an entire module (list of declarations).
    #[allow(missing_docs)]
    pub fn format_module(&self, decls: &[Located<Decl>]) -> Doc {
        let decl_docs: Vec<Doc> = decls.iter().map(|d| self.format_decl(&d.value)).collect();
        if self.config.blank_between_decls {
            intersperse(&decl_docs, Doc::HardLine.cat(Doc::HardLine))
        } else {
            vcat(&decl_docs)
        }
    }
}
/// Counts the distribution of line lengths in a formatted output.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LineLengthDistribution {
    pub histogram: Vec<u32>,
    pub total_lines: usize,
}
impl LineLengthDistribution {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn compute(text: &str) -> Self {
        let mut max_len = 0usize;
        let mut lengths = Vec::new();
        for line in text.lines() {
            let l = line.len();
            lengths.push(l);
            if l > max_len {
                max_len = l;
            }
        }
        let mut histogram = vec![0u32; max_len + 1];
        let total = lengths.len();
        for l in lengths {
            histogram[l] += 1;
        }
        Self {
            histogram,
            total_lines: total,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn mean_length(&self) -> f64 {
        if self.total_lines == 0 {
            return 0.0;
        }
        let sum: usize = self
            .histogram
            .iter()
            .enumerate()
            .map(|(i, &c)| i * c as usize)
            .sum();
        sum as f64 / self.total_lines as f64
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn max_length(&self) -> usize {
        self.histogram
            .iter()
            .enumerate()
            .filter(|(_, &c)| c > 0)
            .map(|(i, _)| i)
            .max()
            .unwrap_or(0)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn over_limit(&self, limit: usize) -> usize {
        self.histogram
            .iter()
            .enumerate()
            .filter(|(i, &c)| *i > limit && c > 0)
            .map(|(_, &c)| c as usize)
            .sum()
    }
}
/// A token that carries formatting metadata.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct FormatToken {
    pub text: String,
    pub priority: FormatPriority,
    pub break_before: bool,
    pub break_after: bool,
    #[allow(missing_docs)]
    pub indent_delta: i32,
}
impl FormatToken {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn simple(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            priority: FormatPriority::Medium,
            break_before: false,
            break_after: false,
            indent_delta: 0,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_priority(mut self, p: FormatPriority) -> Self {
        self.priority = p;
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_break_after(mut self) -> Self {
        self.break_after = true;
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_indent(mut self, delta: i32) -> Self {
        self.indent_delta = delta;
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.text.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}
/// A stack-based indent tracker for the formatter.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FormatterIndentStack {
    stack: Vec<usize>,
}
impl FormatterIndentStack {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self { stack: vec![0] }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn push(&mut self, extra: usize) {
        let top = *self.stack.last().unwrap_or(&0);
        self.stack.push(top + extra);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn pop(&mut self) -> usize {
        if self.stack.len() > 1 {
            self.stack.pop().unwrap_or(0)
        } else {
            0
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn current(&self) -> usize {
        *self.stack.last().unwrap_or(&0)
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}
/// A "ribbon" formatter that tries to keep content within a fraction of the line width.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RibbonFormatter {
    pub line_width: usize,
    pub ribbon_frac: f64,
}
impl RibbonFormatter {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(line_width: usize, ribbon_frac: f64) -> Self {
        Self {
            line_width,
            ribbon_frac,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn ribbon_width(&self) -> usize {
        ((self.line_width as f64) * self.ribbon_frac) as usize
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn fits(&self, indent: usize, content_len: usize) -> bool {
        indent + content_len <= self.line_width && content_len <= self.ribbon_width()
    }
}
/// FlatOrBroken choice.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FlatOrBroken2 {
    pub flat: String,
    pub broken: String,
}
impl FlatOrBroken2 {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(flat: impl Into<String>, broken: impl Into<String>) -> Self {
        Self {
            flat: flat.into(),
            broken: broken.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn choose(&self, fits_in: usize, used: usize) -> &str {
        if used + self.flat.len() <= fits_in {
            &self.flat
        } else {
            &self.broken
        }
    }
}
/// Infix expression builder.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct InfixExprBuilder {
    parts: Vec<(String, String)>,
    first: String,
}
impl InfixExprBuilder {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(first: impl Into<String>) -> Self {
        Self {
            parts: Vec::new(),
            first: first.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(mut self, op: impl Into<String>, operand: impl Into<String>) -> Self {
        self.parts.push((op.into(), operand.into()));
        self
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn build_flat(&self) -> String {
        let mut out = self.first.clone();
        for (op, rhs) in &self.parts {
            out.push_str(&format!(" {} {}", op, rhs));
        }
        out
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn build_broken(&self, indent: usize) -> String {
        let mut out = self.first.clone();
        let prefix = " ".repeat(indent);
        for (op, rhs) in &self.parts {
            out.push_str(&format!("\n{}{} {}", prefix, op, rhs));
        }
        out
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn build_auto(&self, width: usize, indent: usize) -> String {
        let flat = self.build_flat();
        if flat.len() <= width {
            flat
        } else {
            self.build_broken(indent)
        }
    }
}
/// A formatter that emits source with syntax highlighting tokens.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct SyntaxHighlightFormatter {
    spans: Vec<HighlightSpan>,
}
impl SyntaxHighlightFormatter {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self { spans: Vec::new() }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_span(&mut self, start: usize, end: usize, kind: HighlightKind) {
        self.spans.push(HighlightSpan { start, end, kind });
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn spans_of_kind(&self, kind: HighlightKind) -> Vec<&HighlightSpan> {
        self.spans.iter().filter(|s| s.kind == kind).collect()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total_highlighted_chars(&self) -> usize {
        self.spans.iter().map(|s| s.end - s.start).sum()
    }
}
/// The Wadler-Lindig layout engine.
///
/// Takes a `Doc` and produces a formatted string, choosing optimal
/// line breaks to stay within the configured width.
#[allow(missing_docs)]
pub struct LayoutEngine {
    config: LayoutConfig,
}
impl LayoutEngine {
    /// Create a new layout engine.
    #[allow(missing_docs)]
    pub fn new(config: LayoutConfig) -> Self {
        Self { config }
    }
    /// Create with default configuration.
    #[allow(missing_docs)]
    pub fn default_engine() -> Self {
        Self {
            config: LayoutConfig::default(),
        }
    }
    /// Lay out a document to a string.
    #[allow(missing_docs)]
    pub fn layout(&self, doc: &Doc) -> String {
        let mut output = String::new();
        let mut col: usize = 0;
        let mut stack = vec![LayoutItem {
            indent: 0,
            mode: Mode::Break,
            doc: doc.clone(),
        }];
        while let Some(item) = stack.pop() {
            match item.doc {
                Doc::Nil => {}
                Doc::Text(ref s) => {
                    output.push_str(s);
                    col += s.len();
                }
                Doc::HardLine => {
                    output.push('\n');
                    let indent = item.indent.max(0) as usize;
                    for _ in 0..indent {
                        output.push(' ');
                    }
                    col = indent;
                }
                Doc::SoftLine => {
                    if item.mode == Mode::Flat {
                        output.push(' ');
                        col += 1;
                    } else {
                        output.push('\n');
                        let indent = item.indent.max(0) as usize;
                        for _ in 0..indent {
                            output.push(' ');
                        }
                        col = indent;
                    }
                }
                Doc::SoftBreak => {
                    if item.mode == Mode::Break {
                        output.push('\n');
                        let indent = item.indent.max(0) as usize;
                        for _ in 0..indent {
                            output.push(' ');
                        }
                        col = indent;
                    }
                }
                Doc::Cat(a, b) => {
                    stack.push(LayoutItem {
                        indent: item.indent,
                        mode: item.mode,
                        doc: *b,
                    });
                    stack.push(LayoutItem {
                        indent: item.indent,
                        mode: item.mode,
                        doc: *a,
                    });
                }
                Doc::Nest(n, inner) => {
                    stack.push(LayoutItem {
                        indent: item.indent + n,
                        mode: item.mode,
                        doc: *inner,
                    });
                }
                Doc::Group(inner) => {
                    let flat_width = self.measure_flat(&inner);
                    let fits = col + flat_width <= self.config.max_width;
                    let mode = if fits { Mode::Flat } else { Mode::Break };
                    stack.push(LayoutItem {
                        indent: item.indent,
                        mode,
                        doc: *inner,
                    });
                }
                Doc::Align(inner) => {
                    stack.push(LayoutItem {
                        indent: col as i32,
                        mode: item.mode,
                        doc: *inner,
                    });
                }
                Doc::Fill(docs) => {
                    for d in docs.into_iter().rev() {
                        let flat_width = self.measure_flat(&d);
                        let fits = col + flat_width <= self.config.max_width;
                        let mode = if fits { Mode::Flat } else { Mode::Break };
                        stack.push(LayoutItem {
                            indent: item.indent,
                            mode,
                            doc: d,
                        });
                    }
                }
                Doc::FlatAlt(flat, broken) => {
                    let chosen = if item.mode == Mode::Flat {
                        *flat
                    } else {
                        *broken
                    };
                    stack.push(LayoutItem {
                        indent: item.indent,
                        mode: item.mode,
                        doc: chosen,
                    });
                }
                Doc::Annotate(_ann, inner) => {
                    stack.push(LayoutItem {
                        indent: item.indent,
                        mode: item.mode,
                        doc: *inner,
                    });
                }
            }
        }
        output
    }
    /// Measure the width of a document when laid out flat (no line breaks).
    fn measure_flat(&self, doc: &Doc) -> usize {
        match doc {
            Doc::Nil => 0,
            Doc::Text(s) => s.len(),
            Doc::HardLine => usize::MAX / 2,
            Doc::SoftLine => 1,
            Doc::SoftBreak => 0,
            Doc::Cat(a, b) => {
                let wa = self.measure_flat(a);
                let wb = self.measure_flat(b);
                wa.saturating_add(wb)
            }
            Doc::Nest(_, inner) => self.measure_flat(inner),
            Doc::Group(inner) => self.measure_flat(inner),
            Doc::Align(inner) => self.measure_flat(inner),
            Doc::Fill(docs) => docs.iter().map(|d| self.measure_flat(d)).sum(),
            Doc::FlatAlt(flat, _) => self.measure_flat(flat),
            Doc::Annotate(_, inner) => self.measure_flat(inner),
        }
    }
}
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighlightKind {
    Keyword,
    Identifier,
    Operator,
    Literal,
    Comment,
    Type,
    Error,
}
/// Let binding aligner.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LetBindingAligner2 {
    bindings: Vec<(String, String)>,
}
impl LetBindingAligner2 {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.bindings.push((name.into(), value.into()));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn render_aligned(&self) -> String {
        let max_len = self
            .bindings
            .iter()
            .map(|(n, _)| n.len())
            .max()
            .unwrap_or(0);
        self.bindings
            .iter()
            .enumerate()
            .map(|(i, (n, v))| {
                let pad = max_len - n.len();
                let prefix = if i > 0 { "\n" } else { "" };
                format!("{}let {}{} := {}", prefix, n, " ".repeat(pad), v)
            })
            .collect()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn render_compact(&self) -> String {
        self.bindings
            .iter()
            .enumerate()
            .map(|(i, (n, v))| {
                let prefix = if i > 0 { "\n" } else { "" };
                format!("{}let {} := {}", prefix, n, v)
            })
            .collect()
    }
}
/// An item on the layout stack.
#[derive(Clone, Debug)]
struct LayoutItem {
    /// Current indentation level.
    indent: i32,
    /// Current mode (flat or break).
    mode: Mode,
    /// The document to lay out.
    doc: Doc,
}
/// Formats a sequence of tokens into lines respecting a width limit.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LineBreaker {
    pub width: usize,
    pub indent: usize,
    pub separator: String,
}
impl LineBreaker {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(width: usize, indent: usize, separator: impl Into<String>) -> Self {
        Self {
            width,
            indent,
            separator: separator.into(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn break_tokens(&self, tokens: &[&str]) -> String {
        let mut out = String::new();
        let mut line_len = self.indent;
        let pad = " ".repeat(self.indent);
        let first = true;
        let mut _first = first;
        for (i, tok) in tokens.iter().enumerate() {
            let sep_len = if i == 0 { 0 } else { self.separator.len() };
            if i == 0 {
                out.push_str(&pad);
                out.push_str(tok);
                line_len += tok.len();
                _first = false;
            } else if line_len + sep_len + tok.len() <= self.width {
                out.push_str(&self.separator);
                out.push_str(tok);
                line_len += sep_len + tok.len();
            } else {
                out.push('\n');
                out.push_str(&pad);
                out.push_str(tok);
                line_len = self.indent + tok.len();
            }
        }
        out
    }
}
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct FormatDecision {
    pub location: String,
    pub flat_chosen: bool,
    pub flat_len: usize,
    pub available_width: usize,
}
/// A document element in the Wadler-Lindig pretty-printing algebra.
///
/// Documents describe layouts abstractly. The printer then chooses
/// the best concrete layout for a given line width.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum Doc {
    /// Empty document (produces nothing).
    Nil,
    /// A literal text fragment.
    Text(String),
    /// A hard line break (always breaks).
    HardLine,
    /// A soft line break (breaks if the group doesn't fit on one line, otherwise space).
    SoftLine,
    /// A soft line break that becomes empty (not a space) when the group fits.
    SoftBreak,
    /// Concatenation of two documents.
    Cat(Box<Doc>, Box<Doc>),
    /// Nest (increase indentation by n for the inner doc).
    Nest(i32, Box<Doc>),
    /// A group that the printer may choose to flatten or break.
    Group(Box<Doc>),
    /// Explicit alignment (set indentation to current column).
    Align(Box<Doc>),
    /// Fill: like group but breaks at fill points.
    Fill(Vec<Doc>),
    /// Alternative layouts: first is flat, second is broken.
    FlatAlt(Box<Doc>, Box<Doc>),
    /// Annotated document (for syntax highlighting or metadata).
    Annotate(Annotation, Box<Doc>),
}
impl Doc {
    /// Create a text document.
    #[allow(missing_docs)]
    pub fn text(s: impl Into<String>) -> Doc {
        Doc::Text(s.into())
    }
    /// Create a hard line break.
    #[allow(missing_docs)]
    pub fn hardline() -> Doc {
        Doc::HardLine
    }
    /// Create a soft line (space when flat, newline when broken).
    #[allow(missing_docs)]
    pub fn softline() -> Doc {
        Doc::SoftLine
    }
    /// Create a soft break (empty when flat, newline when broken).
    #[allow(missing_docs)]
    pub fn softbreak() -> Doc {
        Doc::SoftBreak
    }
    /// Concatenate two documents.
    #[allow(missing_docs)]
    pub fn cat(self, other: Doc) -> Doc {
        Doc::Cat(Box::new(self), Box::new(other))
    }
    /// Nest (indent) the document by n spaces.
    #[allow(missing_docs)]
    pub fn nest(self, n: i32) -> Doc {
        Doc::Nest(n, Box::new(self))
    }
    /// Wrap in a group (printer chooses flat vs broken layout).
    #[allow(missing_docs)]
    pub fn group(self) -> Doc {
        Doc::Group(Box::new(self))
    }
    /// Align to current column.
    #[allow(missing_docs)]
    pub fn align(self) -> Doc {
        Doc::Align(Box::new(self))
    }
    /// Add an annotation.
    #[allow(missing_docs)]
    pub fn annotate(self, ann: Annotation) -> Doc {
        Doc::Annotate(ann, Box::new(self))
    }
    /// Concatenate with a space separator.
    #[allow(missing_docs)]
    pub fn space(self, other: Doc) -> Doc {
        self.cat(Doc::text(" ")).cat(other)
    }
    /// Concatenate with a soft line separator.
    #[allow(missing_docs)]
    pub fn line(self, other: Doc) -> Doc {
        self.cat(Doc::SoftLine).cat(other)
    }
    /// Check if the document is empty/nil.
    #[allow(missing_docs)]
    pub fn is_nil(&self) -> bool {
        matches!(self, Doc::Nil)
    }
}
/// Configuration for the layout engine.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub struct LayoutConfig {
    /// Maximum line width.
    pub max_width: usize,
    /// Indentation size (spaces).
    pub indent_size: i32,
    /// Whether to use tabs instead of spaces.
    #[allow(missing_docs)]
    pub use_tabs: bool,
    /// Tab width (for column computation).
    pub tab_width: usize,
}
/// Tracks output sections with labels for structured formatting.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LabeledSectionFormatter {
    sections: Vec<(String, String)>,
}
impl LabeledSectionFormatter {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add_section(&mut self, label: impl Into<String>, content: impl Into<String>) {
        self.sections.push((label.into(), content.into()));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn render(&self, width: usize) -> String {
        let sep = sep_line(width, '=');
        let mut out = String::new();
        for (i, (label, content)) in self.sections.iter().enumerate() {
            if i > 0 {
                out.push('\n');
            }
            out.push_str(&sep);
            out.push('\n');
            out.push_str(label);
            out.push('\n');
            out.push_str(&sep);
            out.push('\n');
            out.push_str(content);
            out.push('\n');
        }
        out
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }
}
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct HighlightSpan {
    pub start: usize,
    pub end: usize,
    pub kind: HighlightKind,
}
/// A queue of formatting tasks.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FormatQueue {
    tasks: std::collections::VecDeque<String>,
}
impl FormatQueue {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            tasks: std::collections::VecDeque::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn enqueue(&mut self, task: impl Into<String>) {
        self.tasks.push_back(task.into());
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn dequeue(&mut self) -> Option<String> {
        self.tasks.pop_front()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}
/// Annotated format output: maps character positions to AST node ids.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct AnnotatedOutput {
    pub text: String,
    pub annotations: Vec<(usize, usize, u32)>,
}
impl AnnotatedOutput {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            annotations: Vec::new(),
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn annotate(&mut self, start: usize, end: usize, node_id: u32) {
        self.annotations.push((start, end, node_id));
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn node_at(&self, pos: usize) -> Option<u32> {
        self.annotations
            .iter()
            .filter(|(s, e, _)| pos >= *s && pos < *e)
            .map(|(_, _, id)| *id)
            .next()
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn annotation_count(&self) -> usize {
        self.annotations.len()
    }
}
/// A flat sequence of format tokens that can be rendered to a string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct TokenStream2 {
    tokens: Vec<FormatToken>,
    line_width: usize,
}
impl TokenStream2 {
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(line_width: usize) -> Self {
        Self {
            tokens: Vec::new(),
            line_width,
        }
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn push(&mut self, tok: FormatToken) {
        self.tokens.push(tok);
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn render(&self) -> String {
        let mut out = String::new();
        let mut col = 0usize;
        for (i, tok) in self.tokens.iter().enumerate() {
            if i > 0 {
                if col + tok.len() + 1 > self.line_width {
                    out.push('\n');
                    col = 0;
                } else {
                    out.push(' ');
                    col += 1;
                }
            }
            out.push_str(&tok.text);
            col += tok.len();
        }
        out
    }
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }
}
