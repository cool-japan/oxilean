//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Environment, Expr, FVarId, Level, Name};
use oxilean_parse::{Binder, BinderKind, Located, Span, SurfaceExpr};
use std::collections::{HashMap, HashSet};

/// Detects abbreviations in kernel expressions.
///
/// For example, recognizes `Nat.succ (Nat.succ Nat.zero)` as `2`.
#[derive(Clone, Debug, Default)]
pub struct AbbreviationDetector {
    /// Known abbreviation patterns.
    #[allow(dead_code)]
    patterns: Vec<AbbreviationPattern>,
}
impl AbbreviationDetector {
    /// Create a new detector with standard patterns.
    pub fn standard() -> Self {
        Self {
            patterns: vec![
                AbbreviationPattern {
                    name: "nat_literal".to_string(),
                    kind: AbbreviationKind::NatLiteral,
                },
                AbbreviationPattern {
                    name: "list_literal".to_string(),
                    kind: AbbreviationKind::ListLiteral,
                },
                AbbreviationPattern {
                    name: "string_literal".to_string(),
                    kind: AbbreviationKind::StringLiteral,
                },
            ],
        }
    }
    /// Try to detect a natural number literal from a kernel expression.
    pub fn try_nat_literal(expr: &Expr) -> Option<u64> {
        match expr {
            Expr::Lit(oxilean_kernel::Literal::Nat(n)) => Some(*n),
            Expr::Const(name, _) if name_is(name, "Nat.zero") => Some(0),
            Expr::App(f, arg) => {
                if let Expr::Const(name, _) = f.as_ref() {
                    if name_is(name, "Nat.succ") {
                        return Self::try_nat_literal(arg).map(|n| n + 1);
                    }
                }
                None
            }
            _ => None,
        }
    }
    /// Try to detect a list literal from a kernel expression.
    pub fn try_list_literal(expr: &Expr) -> Option<Vec<Expr>> {
        match expr {
            Expr::Const(name, _) if name_is(name, "List.nil") => Some(Vec::new()),
            Expr::App(f, arg) => {
                if let Expr::App(f2, head) = f.as_ref() {
                    if let Expr::App(f3, _type_arg) = f2.as_ref() {
                        if let Expr::Const(name, _) = f3.as_ref() {
                            if name_is(name, "List.cons") {
                                let mut elems = Self::try_list_literal(arg).unwrap_or_default();
                                elems.insert(0, *head.clone());
                                return Some(elems);
                            }
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}
/// A notation entry for pretty-printing.
#[derive(Clone, Debug)]
pub struct NotationEntry {
    /// Kernel constant name (e.g., "HAdd.hAdd").
    pub const_name: String,
    /// Notation symbol (e.g., "+").
    pub symbol: String,
    /// Precedence level.
    pub precedence: u32,
    /// Whether this is left-associative.
    pub left_assoc: bool,
    /// Number of explicit arguments to display.
    pub arity: usize,
    /// Whether this is an infix operator.
    pub is_infix: bool,
    /// Whether this is a prefix operator.
    pub is_prefix: bool,
    /// Whether this is a postfix operator.
    pub is_postfix: bool,
}
/// Table of registered notations for delaboration.
#[derive(Clone, Debug, Default)]
pub struct NotationTable {
    /// Map from kernel constant name to notation entry.
    entries: HashMap<String, NotationEntry>,
}
impl NotationTable {
    /// Create a new empty notation table.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create with standard notations.
    pub fn standard() -> Self {
        let mut table = Self::new();
        table.add_infix("HAdd.hAdd", "+", 65, true, 6);
        table.add_infix("HSub.hSub", "-", 65, true, 6);
        table.add_infix("HMul.hMul", "*", 70, true, 6);
        table.add_infix("HDiv.hDiv", "/", 70, true, 6);
        table.add_infix("HMod.hMod", "%", 70, true, 6);
        table.add_infix("HPow.hPow", "^", 75, false, 6);
        table.add_infix("HAnd.hAnd", "&&", 35, false, 6);
        table.add_infix("HOr.hOr", "||", 30, false, 6);
        table.add_infix("BEq.beq", "==", 50, false, 4);
        table.add_infix("Eq", "=", 50, false, 3);
        table.add_infix("LT.lt", "<", 50, false, 4);
        table.add_infix("LE.le", "<=", 50, false, 4);
        table.add_infix("GT.gt", ">", 50, false, 4);
        table.add_infix("GE.ge", ">=", 50, false, 4);
        table.add_infix("And", "/\\", 35, false, 2);
        table.add_infix("Or", "\\/", 30, false, 2);
        table.add_prefix("Not", "!", 40, 1);
        table.add_prefix("Neg.neg", "-", 100, 2);
        table
    }
    /// Add an infix notation.
    fn add_infix(
        &mut self,
        const_name: &str,
        symbol: &str,
        precedence: u32,
        left_assoc: bool,
        arity: usize,
    ) {
        self.entries.insert(
            const_name.to_string(),
            NotationEntry {
                const_name: const_name.to_string(),
                symbol: symbol.to_string(),
                precedence,
                left_assoc,
                arity,
                is_infix: true,
                is_prefix: false,
                is_postfix: false,
            },
        );
    }
    /// Add a prefix notation.
    fn add_prefix(&mut self, const_name: &str, symbol: &str, precedence: u32, arity: usize) {
        self.entries.insert(
            const_name.to_string(),
            NotationEntry {
                const_name: const_name.to_string(),
                symbol: symbol.to_string(),
                precedence,
                left_assoc: false,
                arity,
                is_infix: false,
                is_prefix: true,
                is_postfix: false,
            },
        );
    }
    /// Look up a notation for a constant name.
    pub fn get(&self, name: &str) -> Option<&NotationEntry> {
        self.entries.get(name)
    }
    /// Register a custom notation.
    pub fn register(&mut self, entry: NotationEntry) {
        self.entries.insert(entry.const_name.clone(), entry);
    }
    /// Get all registered notations.
    pub fn all_entries(&self) -> impl Iterator<Item = &NotationEntry> {
        self.entries.values()
    }
}
/// Options for the `print_surface_expr` function.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PrintOptionsExt {
    /// Width to attempt to fit output within (for line-breaking).
    pub width: usize,
    /// Use compact (single-line) output.
    pub compact: bool,
    /// Indentation string.
    pub indent: String,
}
/// The main delaborator: converts kernel Expr to surface SurfaceExpr.
pub struct Delaborator;
impl Delaborator {
    /// Delaborate a kernel expression to a surface expression.
    pub fn delab(ctx: &mut DelabContext<'_>, expr: &Expr) -> Located<SurfaceExpr> {
        if !ctx.enter() {
            ctx.leave();
            return mk_located(SurfaceExpr::Var("...".to_string()));
        }
        let result = Self::delab_inner(ctx, expr);
        ctx.leave();
        result
    }
    fn delab_inner(ctx: &mut DelabContext<'_>, expr: &Expr) -> Located<SurfaceExpr> {
        if ctx.config.use_abbreviations {
            if let Some(n) = AbbreviationDetector::try_nat_literal(expr) {
                return mk_located(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(n)));
            }
            if let Some(elems) = AbbreviationDetector::try_list_literal(expr) {
                let surface_elems: Vec<Located<SurfaceExpr>> =
                    elems.iter().map(|e| Self::delab(ctx, e)).collect();
                return mk_located(SurfaceExpr::ListLit(surface_elems));
            }
        }
        match expr {
            Expr::Sort(level) => Self::delab_sort(ctx, level),
            Expr::BVar(idx) => mk_located(SurfaceExpr::Var(format!("#{}", idx))),
            Expr::FVar(fvar) => {
                let name = ctx.fvar_name(fvar);
                mk_located(SurfaceExpr::Var(name))
            }
            Expr::Const(name, levels) => Self::delab_const(ctx, name, levels),
            Expr::App(f, arg) => Self::delab_app(ctx, f, arg),
            Expr::Lam(bi, name, ty, body) => Self::delab_lam(ctx, *bi, name, ty, body),
            Expr::Pi(bi, name, ty, body) => Self::delab_pi(ctx, *bi, name, ty, body),
            Expr::Let(name, ty, val, body) => Self::delab_let(ctx, name, ty, val, body),
            Expr::Lit(lit) => Self::delab_lit(lit),
            Expr::Proj(struct_name, idx, struct_expr) => {
                Self::delab_proj(ctx, struct_name, *idx, struct_expr)
            }
        }
    }
    /// Delaborate a Sort expression.
    fn delab_sort(_ctx: &mut DelabContext<'_>, level: &Level) -> Located<SurfaceExpr> {
        if level.is_zero() {
            mk_located(SurfaceExpr::Sort(oxilean_parse::SortKind::Prop))
        } else {
            mk_located(SurfaceExpr::Sort(oxilean_parse::SortKind::Type))
        }
    }
    /// Delaborate a Const expression.
    fn delab_const(
        ctx: &mut DelabContext<'_>,
        name: &Name,
        levels: &[Level],
    ) -> Located<SurfaceExpr> {
        let display_name = format!("{}", name);
        let final_name = ctx
            .config
            .name_overrides
            .get(&display_name)
            .cloned()
            .unwrap_or(display_name);
        if ctx.config.show_universes && !levels.is_empty() {
            let level_strs: Vec<String> = levels.iter().map(|l| format!("{}", l)).collect();
            mk_located(SurfaceExpr::Var(format!(
                "{}.{{{}}}",
                final_name,
                level_strs.join(", ")
            )))
        } else {
            mk_located(SurfaceExpr::Var(final_name))
        }
    }
    /// Delaborate an application, potentially using notation.
    fn delab_app(ctx: &mut DelabContext<'_>, f: &Expr, arg: &Expr) -> Located<SurfaceExpr> {
        if ctx.config.use_notation {
            if let Some(result) = Self::try_notation(ctx, f, arg) {
                return result;
            }
        }
        if !ctx.config.show_implicit {
            if let Expr::App(inner_f, _implicit_arg) = f {
                if Self::is_implicit_app(ctx, inner_f) {
                    let delab_f = Self::delab(ctx, inner_f);
                    let delab_arg = Self::delab(ctx, arg);
                    return mk_located(SurfaceExpr::App(Box::new(delab_f), Box::new(delab_arg)));
                }
            }
        }
        let delab_f = Self::delab(ctx, f);
        let delab_arg = Self::delab(ctx, arg);
        mk_located(SurfaceExpr::App(Box::new(delab_f), Box::new(delab_arg)))
    }
    /// Check if an application involves an implicit argument.
    fn is_implicit_app(_ctx: &mut DelabContext<'_>, f: &Expr) -> bool {
        if let Expr::Lam(bi, _, _, _) = f {
            matches!(bi, BinderInfo::Implicit | BinderInfo::InstImplicit)
        } else {
            false
        }
    }
    /// Try to use a notation for an application.
    fn try_notation(
        ctx: &mut DelabContext<'_>,
        f: &Expr,
        arg: &Expr,
    ) -> Option<Located<SurfaceExpr>> {
        let (head, args) = collect_app_args(f, arg);
        if let Expr::Const(name, _) = head {
            let name_str = format!("{}", name);
            let notation_info = ctx
                .notations
                .get(&name_str)
                .map(|n| (n.symbol.clone(), n.is_infix, n.is_prefix));
            if let Some((symbol, is_infix, is_prefix)) = notation_info {
                if is_infix && args.len() >= 2 {
                    let lhs = &args[args.len() - 2];
                    let rhs = &args[args.len() - 1];
                    let delab_lhs = Self::delab(ctx, lhs);
                    let delab_rhs = Self::delab(ctx, rhs);
                    let op = mk_located(SurfaceExpr::Var(symbol));
                    let partial = mk_located(SurfaceExpr::App(Box::new(op), Box::new(delab_lhs)));
                    return Some(mk_located(SurfaceExpr::App(
                        Box::new(partial),
                        Box::new(delab_rhs),
                    )));
                }
                if is_prefix && !args.is_empty() {
                    let operand = &args[args.len() - 1];
                    let delab_operand = Self::delab(ctx, operand);
                    let op = mk_located(SurfaceExpr::Var(symbol));
                    return Some(mk_located(SurfaceExpr::App(
                        Box::new(op),
                        Box::new(delab_operand),
                    )));
                }
            }
        }
        None
    }
    /// Delaborate a lambda expression.
    fn delab_lam(
        ctx: &mut DelabContext<'_>,
        bi: BinderInfo,
        name: &Name,
        ty: &Expr,
        body: &Expr,
    ) -> Located<SurfaceExpr> {
        let name_str = ctx.fresh_name(&format!("{}", name));
        let binder_kind = binder_info_to_kind(bi);
        let delab_ty = Self::delab(ctx, ty);
        let delab_body = Self::delab(ctx, body);
        let binder = Binder {
            name: name_str,
            ty: Some(Box::new(delab_ty)),
            info: binder_kind,
        };
        let mut binders = vec![binder];
        let mut current_body = delab_body;
        while let SurfaceExpr::Lam(inner_binders, inner_body) = current_body.value {
            binders.extend(inner_binders);
            current_body = *inner_body;
        }
        mk_located(SurfaceExpr::Lam(binders, Box::new(current_body)))
    }
    /// Delaborate a Pi type.
    fn delab_pi(
        ctx: &mut DelabContext<'_>,
        bi: BinderInfo,
        name: &Name,
        ty: &Expr,
        body: &Expr,
    ) -> Located<SurfaceExpr> {
        let name_str = format!("{}", name);
        let binder_kind = binder_info_to_kind(bi);
        let delab_ty = Self::delab(ctx, ty);
        if !has_loose_bvar(body, 0) && !ctx.config.show_implicit {
            let delab_body = Self::delab(ctx, body);
            let arrow_name = "->";
            let arrow = mk_located(SurfaceExpr::Var(arrow_name.to_string()));
            let partial = mk_located(SurfaceExpr::App(Box::new(arrow), Box::new(delab_ty)));
            return mk_located(SurfaceExpr::App(Box::new(partial), Box::new(delab_body)));
        }
        let fresh = ctx.fresh_name(&name_str);
        let delab_body = Self::delab(ctx, body);
        let binder = Binder {
            name: fresh,
            ty: Some(Box::new(delab_ty)),
            info: binder_kind,
        };
        mk_located(SurfaceExpr::Pi(vec![binder], Box::new(delab_body)))
    }
    /// Delaborate a let expression.
    fn delab_let(
        ctx: &mut DelabContext<'_>,
        name: &Name,
        ty: &Expr,
        val: &Expr,
        body: &Expr,
    ) -> Located<SurfaceExpr> {
        let name_str = ctx.fresh_name(&format!("{}", name));
        let delab_ty = Self::delab(ctx, ty);
        let delab_val = Self::delab(ctx, val);
        let delab_body = Self::delab(ctx, body);
        mk_located(SurfaceExpr::Let(
            name_str,
            Some(Box::new(delab_ty)),
            Box::new(delab_val),
            Box::new(delab_body),
        ))
    }
    /// Delaborate a literal.
    fn delab_lit(lit: &oxilean_kernel::Literal) -> Located<SurfaceExpr> {
        match lit {
            oxilean_kernel::Literal::Nat(n) => {
                mk_located(SurfaceExpr::Lit(oxilean_parse::Literal::Nat(*n)))
            }
            oxilean_kernel::Literal::Str(s) => {
                mk_located(SurfaceExpr::Lit(oxilean_parse::Literal::String(s.clone())))
            }
        }
    }
    /// Delaborate a projection.
    fn delab_proj(
        ctx: &mut DelabContext<'_>,
        struct_name: &Name,
        idx: u32,
        struct_expr: &Expr,
    ) -> Located<SurfaceExpr> {
        let delab_struct = Self::delab(ctx, struct_expr);
        let field_name = format!("{}.{}", struct_name, idx);
        mk_located(SurfaceExpr::Proj(Box::new(delab_struct), field_name))
    }
}
/// A single notation entry: a kernel name pattern and its display form.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct NotationEntryExt {
    /// The kernel constant name this notation applies to.
    pub kernel_name: String,
    /// The number of arguments the notation consumes.
    pub arity: usize,
    /// The display template (e.g., `"{0} + {1}"` for `HAdd.hAdd`).
    pub template: String,
    /// Precedence for parenthesization decisions.
    pub precedence: i32,
    /// Whether this notation is infix.
    pub is_infix: bool,
}
/// Kind of abbreviation.
#[derive(Clone, Debug)]
pub enum AbbreviationKind {
    /// Natural number literal from Nat.succ chains.
    NatLiteral,
    /// List literal from List.cons chains.
    ListLiteral,
    /// String literal from String.mk chains.
    StringLiteral,
    /// Custom pattern.
    Custom(String),
}
/// Registry of notations for pretty-printing.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct NotationRegistry {
    entries: Vec<NotationEntryExt>,
    by_name: HashMap<String, usize>,
}
impl NotationRegistry {
    /// Create an empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            by_name: HashMap::new(),
        }
    }
    /// Create a registry pre-populated with standard arithmetic notations.
    #[allow(dead_code)]
    pub fn with_standard_notations() -> Self {
        let mut reg = Self::new();
        reg.register(NotationEntryExt {
            kernel_name: "HAdd.hAdd".to_owned(),
            arity: 2,
            template: "{0} + {1}".to_owned(),
            precedence: 65,
            is_infix: true,
        });
        reg.register(NotationEntryExt {
            kernel_name: "HSub.hSub".to_owned(),
            arity: 2,
            template: "{0} - {1}".to_owned(),
            precedence: 65,
            is_infix: true,
        });
        reg.register(NotationEntryExt {
            kernel_name: "HMul.hMul".to_owned(),
            arity: 2,
            template: "{0} * {1}".to_owned(),
            precedence: 70,
            is_infix: true,
        });
        reg.register(NotationEntryExt {
            kernel_name: "HDiv.hDiv".to_owned(),
            arity: 2,
            template: "{0} / {1}".to_owned(),
            precedence: 70,
            is_infix: true,
        });
        reg.register(NotationEntryExt {
            kernel_name: "Eq".to_owned(),
            arity: 2,
            template: "{0} = {1}".to_owned(),
            precedence: 50,
            is_infix: true,
        });
        reg.register(NotationEntryExt {
            kernel_name: "Ne".to_owned(),
            arity: 2,
            template: "{0} != {1}".to_owned(),
            precedence: 50,
            is_infix: true,
        });
        reg.register(NotationEntryExt {
            kernel_name: "And".to_owned(),
            arity: 2,
            template: "{0} && {1}".to_owned(),
            precedence: 35,
            is_infix: true,
        });
        reg.register(NotationEntryExt {
            kernel_name: "Or".to_owned(),
            arity: 2,
            template: "{0} || {1}".to_owned(),
            precedence: 30,
            is_infix: true,
        });
        reg
    }
    /// Register a new notation.
    #[allow(dead_code)]
    pub fn register(&mut self, entry: NotationEntryExt) {
        let idx = self.entries.len();
        self.by_name.insert(entry.kernel_name.clone(), idx);
        self.entries.push(entry);
    }
    /// Look up notation for a kernel name.
    #[allow(dead_code)]
    pub fn lookup(&self, kernel_name: &str) -> Option<&NotationEntryExt> {
        self.by_name.get(kernel_name).map(|&i| &self.entries[i])
    }
    /// Number of registered notations.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the registry is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Remove a notation by kernel name. Returns `true` if it existed.
    #[allow(dead_code)]
    pub fn unregister(&mut self, kernel_name: &str) -> bool {
        if let Some(&idx) = self.by_name.get(kernel_name) {
            self.by_name.remove(kernel_name);
            self.entries.remove(idx);
            self.by_name.clear();
            for (i, e) in self.entries.iter().enumerate() {
                self.by_name.insert(e.kernel_name.clone(), i);
            }
            true
        } else {
            false
        }
    }
    /// Iterate over all entries.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &NotationEntryExt> {
        self.entries.iter()
    }
}
/// Context for the delaborator.
pub struct DelabContext<'a> {
    /// Kernel environment.
    pub env: &'a Environment,
    /// Configuration.
    pub config: DelabConfig,
    /// Notation table.
    pub notations: NotationTable,
    /// Abbreviation detector.
    pub abbreviations: AbbreviationDetector,
    /// Free variable name mapping.
    fvar_names: HashMap<FVarId, String>,
    /// Name counter for generating fresh names.
    pub(super) name_counter: u64,
    /// Current recursion depth.
    depth: usize,
    /// Set of names used in the current scope (to avoid clashes).
    pub(super) used_names: HashSet<String>,
}
impl<'a> DelabContext<'a> {
    /// Create a new delaborator context.
    pub fn new(env: &'a Environment) -> Self {
        Self {
            env,
            config: DelabConfig::default(),
            notations: NotationTable::standard(),
            abbreviations: AbbreviationDetector::standard(),
            fvar_names: HashMap::new(),
            name_counter: 0,
            depth: 0,
            used_names: HashSet::new(),
        }
    }
    /// Create with custom configuration.
    pub fn with_config(env: &'a Environment, config: DelabConfig) -> Self {
        Self {
            env,
            config,
            notations: NotationTable::standard(),
            abbreviations: AbbreviationDetector::standard(),
            fvar_names: HashMap::new(),
            name_counter: 0,
            depth: 0,
            used_names: HashSet::new(),
        }
    }
    /// Register a free variable name.
    pub fn register_fvar(&mut self, fvar: FVarId, name: String) {
        self.fvar_names.insert(fvar, name);
    }
    /// Generate a fresh name.
    pub fn fresh_name(&mut self, base: &str) -> String {
        let base = if base.is_empty() || base == "_" {
            "x"
        } else {
            base
        };
        let name = base.to_string();
        if !self.used_names.contains(&name) {
            self.used_names.insert(name.clone());
            return name;
        }
        loop {
            self.name_counter += 1;
            let candidate = format!("{}{}", base, self.name_counter);
            if !self.used_names.contains(&candidate) {
                self.used_names.insert(candidate.clone());
                return candidate;
            }
        }
    }
    /// Get the display name for a free variable.
    pub fn fvar_name(&self, fvar: &FVarId) -> String {
        self.fvar_names
            .get(fvar)
            .cloned()
            .unwrap_or_else(|| format!("_fvar_{}", fvar.0))
    }
    /// Increment depth (returns false if max depth exceeded).
    fn enter(&mut self) -> bool {
        self.depth += 1;
        self.depth <= self.config.max_depth
    }
    /// Decrement depth.
    fn leave(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }
}
/// Surface expression printer.
pub struct SurfacePrinter {
    pub(super) output: String,
    opts: PrintOptions,
    indent: usize,
    col: usize,
}
impl SurfacePrinter {
    pub fn new(opts: PrintOptions) -> Self {
        Self {
            output: String::new(),
            opts,
            indent: 0,
            col: 0,
        }
    }
    pub fn write(&mut self, s: &str) {
        self.output.push_str(s);
        self.col += s.len();
    }
    pub fn newline(&mut self) {
        self.output.push('\n');
        for _ in 0..self.indent {
            self.output.push(' ');
        }
        self.col = self.indent;
    }
    pub fn indent(&mut self) {
        self.indent += self.opts.indent_size;
    }
    pub fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(self.opts.indent_size);
    }
    pub fn print_expr(&mut self, expr: &SurfaceExpr) {
        match expr {
            SurfaceExpr::Var(name) => self.write(name),
            SurfaceExpr::Lit(lit) => self.print_literal(lit),
            SurfaceExpr::Hole => self.write("_"),
            SurfaceExpr::Sort(kind) => self.print_sort(kind),
            SurfaceExpr::App(f, arg) => {
                self.write("(");
                self.print_expr(&f.value);
                self.write(" ");
                self.print_expr(&arg.value);
                self.write(")");
            }
            SurfaceExpr::Lam(binders, body) => {
                self.write("fun");
                for b in binders {
                    self.write(" ");
                    self.print_binder(b);
                }
                self.write(" => ");
                self.print_expr(&body.value);
            }
            SurfaceExpr::Pi(binders, body) => {
                for b in binders {
                    self.print_pi_binder(b);
                }
                self.write(" -> ");
                self.print_expr(&body.value);
            }
            SurfaceExpr::Let(name, ty, val, body) => {
                self.write("let ");
                self.write(name);
                if let Some(ty_expr) = ty {
                    self.write(" : ");
                    self.print_expr(&ty_expr.value);
                }
                self.write(" := ");
                self.print_expr(&val.value);
                self.newline();
                self.print_expr(&body.value);
            }
            SurfaceExpr::Ann(e, ty) => {
                self.write("(");
                self.print_expr(&e.value);
                self.write(" : ");
                self.print_expr(&ty.value);
                self.write(")");
            }
            SurfaceExpr::If(c, t, e) => {
                self.write("if ");
                self.print_expr(&c.value);
                self.write(" then ");
                self.indent();
                self.newline();
                self.print_expr(&t.value);
                self.dedent();
                self.newline();
                self.write("else ");
                self.indent();
                self.newline();
                self.print_expr(&e.value);
                self.dedent();
            }
            SurfaceExpr::Match(scrutinee, arms) => {
                self.write("match ");
                self.print_expr(&scrutinee.value);
                self.write(" with");
                self.indent();
                for arm in arms {
                    self.newline();
                    self.write("| ");
                    self.print_pattern(&arm.pattern.value);
                    self.write(" => ");
                    self.print_expr(&arm.rhs.value);
                }
                self.dedent();
            }
            SurfaceExpr::Proj(e, field) => {
                self.print_expr(&e.value);
                self.write(".");
                self.write(field);
            }
            SurfaceExpr::ListLit(elems) => {
                self.write("[");
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.print_expr(&elem.value);
                }
                self.write("]");
            }
            SurfaceExpr::Tuple(elems) => {
                self.write("(");
                for (i, elem) in elems.iter().enumerate() {
                    if i > 0 {
                        self.write(", ");
                    }
                    self.print_expr(&elem.value);
                }
                self.write(")");
            }
            SurfaceExpr::Return(e) => {
                self.write("return ");
                self.print_expr(&e.value);
            }
            _ => self.write("<?>"),
        }
    }
    fn print_literal(&mut self, lit: &oxilean_parse::Literal) {
        match lit {
            oxilean_parse::Literal::Nat(n) => self.write(&n.to_string()),
            oxilean_parse::Literal::Float(f) => self.write(&f.to_string()),
            oxilean_parse::Literal::String(s) => {
                self.write("\"");
                self.write(s);
                self.write("\"");
            }
            oxilean_parse::Literal::Char(c) => {
                self.write("'");
                self.write(&c.to_string());
                self.write("'");
            }
        }
    }
    fn print_sort(&mut self, kind: &oxilean_parse::SortKind) {
        match kind {
            oxilean_parse::SortKind::Prop => self.write("Prop"),
            oxilean_parse::SortKind::Type => self.write("Type"),
            oxilean_parse::SortKind::TypeU(u) => {
                self.write("Type ");
                self.write(u);
            }
            oxilean_parse::SortKind::SortU(u) => {
                self.write("Sort ");
                self.write(u);
            }
        }
    }
    fn print_binder(&mut self, binder: &Binder) {
        let (open, close) = match binder.info {
            BinderKind::Default => ("(", ")"),
            BinderKind::Implicit => ("{", "}"),
            BinderKind::StrictImplicit => ("{{", "}}"),
            BinderKind::Instance => ("[", "]"),
        };
        self.write(open);
        self.write(&binder.name);
        if let Some(ref ty) = binder.ty {
            self.write(" : ");
            self.print_expr(&ty.value);
        }
        self.write(close);
    }
    fn print_pi_binder(&mut self, binder: &Binder) {
        match binder.info {
            BinderKind::Default => {
                self.write("(");
                self.write(&binder.name);
                if let Some(ref ty) = binder.ty {
                    self.write(" : ");
                    self.print_expr(&ty.value);
                }
                self.write(")");
            }
            BinderKind::Implicit => {
                self.write("{");
                self.write(&binder.name);
                if let Some(ref ty) = binder.ty {
                    self.write(" : ");
                    self.print_expr(&ty.value);
                }
                self.write("}");
            }
            BinderKind::Instance => {
                self.write("[");
                self.write(&binder.name);
                if let Some(ref ty) = binder.ty {
                    self.write(" : ");
                    self.print_expr(&ty.value);
                }
                self.write("]");
            }
            BinderKind::StrictImplicit => {
                self.write("{{");
                self.write(&binder.name);
                if let Some(ref ty) = binder.ty {
                    self.write(" : ");
                    self.print_expr(&ty.value);
                }
                self.write("}}");
            }
        }
    }
    fn print_pattern(&mut self, pat: &oxilean_parse::Pattern) {
        match pat {
            oxilean_parse::Pattern::Wild => self.write("_"),
            oxilean_parse::Pattern::Var(name) => self.write(name),
            oxilean_parse::Pattern::Ctor(name, args) => {
                self.write(name);
                for arg in args {
                    self.write(" ");
                    self.print_pattern(&arg.value);
                }
            }
            oxilean_parse::Pattern::Lit(lit) => self.print_literal(lit),
            oxilean_parse::Pattern::Or(a, b) => {
                self.print_pattern(&a.value);
                self.write(" | ");
                self.print_pattern(&b.value);
            }
        }
    }
}
/// Delaborate a kernel declaration to a surface declaration.
pub struct DeclDelaborator;
impl DeclDelaborator {
    /// Delaborate a definition.
    pub fn delab_definition(
        env: &Environment,
        name: &Name,
        ty: &Expr,
        val: &Expr,
        univ_params: &[Name],
    ) -> Located<oxilean_parse::Decl> {
        let mut ctx = DelabContext::new(env);
        let delab_ty = Delaborator::delab(&mut ctx, ty);
        let delab_val = Delaborator::delab(&mut ctx, val);
        let up: Vec<String> = univ_params.iter().map(|n| format!("{}", n)).collect();
        mk_located_decl(oxilean_parse::Decl::Definition {
            name: format!("{}", name),
            univ_params: up,
            ty: Some(delab_ty),
            val: delab_val,
            where_clauses: Vec::new(),
            attrs: Vec::new(),
        })
    }
    /// Delaborate a theorem.
    pub fn delab_theorem(
        env: &Environment,
        name: &Name,
        ty: &Expr,
        proof: &Expr,
        univ_params: &[Name],
        hide_proof: bool,
    ) -> Located<oxilean_parse::Decl> {
        let mut ctx = DelabContext::new(env);
        let delab_ty = Delaborator::delab(&mut ctx, ty);
        let delab_proof = if hide_proof {
            mk_located(SurfaceExpr::Var("...".to_string()))
        } else {
            Delaborator::delab(&mut ctx, proof)
        };
        let up: Vec<String> = univ_params.iter().map(|n| format!("{}", n)).collect();
        mk_located_decl(oxilean_parse::Decl::Theorem {
            name: format!("{}", name),
            univ_params: up,
            ty: delab_ty,
            proof: delab_proof,
            where_clauses: Vec::new(),
            attrs: Vec::new(),
        })
    }
    /// Delaborate an axiom.
    pub fn delab_axiom(
        env: &Environment,
        name: &Name,
        ty: &Expr,
        univ_params: &[Name],
    ) -> Located<oxilean_parse::Decl> {
        let mut ctx = DelabContext::new(env);
        let delab_ty = Delaborator::delab(&mut ctx, ty);
        let up: Vec<String> = univ_params.iter().map(|n| format!("{}", n)).collect();
        mk_located_decl(oxilean_parse::Decl::Axiom {
            name: format!("{}", name),
            univ_params: up,
            ty: delab_ty,
            attrs: Vec::new(),
        })
    }
}
/// Print configuration for delaborated expressions.
#[derive(Clone, Debug)]
pub struct PrintOptions {
    /// Maximum line width.
    pub max_width: usize,
    /// Indentation size.
    pub indent_size: usize,
    /// Whether to use Unicode.
    pub use_unicode: bool,
}
/// Configuration for the delaborator.
#[derive(Clone, Debug)]
pub struct DelabConfig {
    /// Whether to show implicit arguments.
    pub show_implicit: bool,
    /// Whether to show universe levels.
    pub show_universes: bool,
    /// Whether to use registered notations for pretty display.
    pub use_notation: bool,
    /// Whether to attempt abbreviation detection (e.g., numeral from Nat.succ chains).
    pub use_abbreviations: bool,
    /// Whether to hide proofs (replace with `...`).
    pub hide_proofs: bool,
    /// Maximum depth for recursive delaboration.
    pub max_depth: usize,
    /// Whether to use Unicode output.
    pub use_unicode: bool,
    /// Whether to show binder info markers ({}, [], etc.).
    pub show_binder_info: bool,
    /// Whether to omit redundant type annotations.
    pub omit_redundant_types: bool,
    /// Custom name overrides (kernel name -> display name).
    pub name_overrides: HashMap<String, String>,
}
impl DelabConfig {
    /// Verbose configuration: show everything.
    pub fn verbose() -> Self {
        Self {
            show_implicit: true,
            show_universes: true,
            use_notation: false,
            use_abbreviations: false,
            hide_proofs: false,
            ..Default::default()
        }
    }
    /// Minimal configuration: hide as much as possible.
    pub fn minimal() -> Self {
        Self {
            show_implicit: false,
            show_universes: false,
            use_notation: true,
            use_abbreviations: true,
            hide_proofs: true,
            omit_redundant_types: true,
            ..Default::default()
        }
    }
}
impl DelabConfig {
    /// Create a config suitable for proof-state display (hide proofs, show types).
    #[allow(dead_code)]
    pub fn proof_state() -> Self {
        Self {
            show_implicit: false,
            show_universes: false,
            use_notation: true,
            use_abbreviations: true,
            hide_proofs: true,
            max_depth: 50,
            use_unicode: true,
            show_binder_info: true,
            omit_redundant_types: false,
            name_overrides: HashMap::new(),
        }
    }
    /// Create a config for term export (minimal notation, no abbreviations).
    #[allow(dead_code)]
    pub fn export() -> Self {
        Self {
            show_implicit: true,
            show_universes: true,
            use_notation: false,
            use_abbreviations: false,
            hide_proofs: false,
            max_depth: 200,
            use_unicode: false,
            show_binder_info: true,
            omit_redundant_types: false,
            name_overrides: HashMap::new(),
        }
    }
    /// Create a config for error messages (compact, user-friendly).
    #[allow(dead_code)]
    pub fn error_message() -> Self {
        Self {
            show_implicit: false,
            show_universes: false,
            use_notation: true,
            use_abbreviations: true,
            hide_proofs: true,
            max_depth: 20,
            use_unicode: true,
            show_binder_info: false,
            omit_redundant_types: true,
            name_overrides: HashMap::new(),
        }
    }
    /// Add a name override.
    #[allow(dead_code)]
    pub fn with_name_override(mut self, kernel_name: &str, display_name: &str) -> Self {
        self.name_overrides
            .insert(kernel_name.to_owned(), display_name.to_owned());
        self
    }
    /// Set max depth.
    #[allow(dead_code)]
    pub fn with_max_depth(mut self, depth: usize) -> Self {
        self.max_depth = depth;
        self
    }
}
/// Precedence category for an expression.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Prec {
    /// Lowest precedence: lambda, Pi, forall, let.
    Binder = 0,
    /// Arrow type (→).
    Arrow = 25,
    /// Or (||, ∨).
    Or = 30,
    /// And (&&, ∧).
    And = 35,
    /// Equality, inequality, ordering.
    Rel = 50,
    /// Addition, subtraction.
    Add = 65,
    /// Multiplication, division.
    Mul = 70,
    /// Unary prefix operators.
    Prefix = 80,
    /// Function application.
    App = 90,
    /// Atomic: variables, literals, parenthesized subexpressions.
    Atom = 100,
}
/// A binder collected from a Pi chain.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CollectedBinder {
    /// Binder name (or `_` for anonymous).
    pub name: String,
    /// Binder kind.
    pub kind: BinderKind,
    /// Binder type expression.
    pub ty: SurfaceExpr,
}
/// A stateful delaborator that uses a `DelabContextExt` and `DelabConfig`.
#[allow(dead_code)]
pub struct ContextualDelaborator<'env> {
    /// Reference to the global environment.
    pub env: &'env Environment,
    /// Configuration.
    pub config: DelabConfig,
    /// Current context.
    pub ctx: DelabContextExt,
    /// Notation registry.
    pub notations: NotationRegistry,
}
impl<'env> ContextualDelaborator<'env> {
    /// Create a new contextual delaborator.
    #[allow(dead_code)]
    pub fn new(env: &'env Environment, config: DelabConfig) -> Self {
        let max_depth = config.max_depth;
        Self {
            env,
            config,
            ctx: DelabContextExt::new(max_depth),
            notations: NotationRegistry::with_standard_notations(),
        }
    }
    /// Delaborate a kernel expression to a surface expression.
    #[allow(dead_code)]
    pub fn delab(&mut self, expr: &Expr) -> SurfaceExpr {
        if self.ctx.at_max_depth() {
            return SurfaceExpr::Hole;
        }
        match expr {
            Expr::BVar(i) => {
                let name = self.ctx.lookup_bvar(*i).unwrap_or("_").to_owned();
                SurfaceExpr::Var(name)
            }
            Expr::FVar(fid) => SurfaceExpr::Var(format!("fvar_{}", fid.0)),
            Expr::Const(name, _levels) => {
                let display = self
                    .config
                    .name_overrides
                    .get(&name.to_string())
                    .cloned()
                    .unwrap_or_else(|| name.to_string());
                SurfaceExpr::Var(display)
            }
            Expr::Lit(lit) => delab_literal(lit),
            Expr::Sort(level) => {
                let kind = if matches!(level, Level::Zero) {
                    oxilean_parse::SortKind::Prop
                } else {
                    oxilean_parse::SortKind::Type
                };
                SurfaceExpr::Sort(kind)
            }
            Expr::App(f, a) => {
                if self.config.use_abbreviations {
                    if let Some(n) = decode_nat_numeral(expr) {
                        return SurfaceExpr::Lit(oxilean_parse::Literal::Nat(n));
                    }
                }
                let fun = self.delab(f);
                let arg = self.delab(a);
                SurfaceExpr::App(
                    Box::new(Located::new(fun, Span::new(0, 0, 0, 0))),
                    Box::new(Located::new(arg, Span::new(0, 0, 0, 0))),
                )
            }
            Expr::Lam(bind_info, name, ty, body) => {
                let ty_delab = self.delab(ty);
                let name_str = if is_anonymous_name(name) {
                    self.ctx.fresh("x")
                } else {
                    name.to_string()
                };
                let info = binder_info_to_kind(*bind_info);
                self.ctx.push(name_str.clone());
                let body_delab = self.delab(body);
                self.ctx.pop();
                SurfaceExpr::Lam(
                    vec![Binder {
                        name: name_str,
                        ty: Some(Box::new(Located::new(ty_delab, Span::new(0, 0, 0, 0)))),
                        info,
                    }],
                    Box::new(Located::new(body_delab, Span::new(0, 0, 0, 0))),
                )
            }
            Expr::Pi(bind_info, name, ty, body) => {
                let ty_delab = self.delab(ty);
                let name_str = if is_anonymous_name(name) {
                    "_".to_owned()
                } else {
                    name.to_string()
                };
                let info = binder_info_to_kind(*bind_info);
                self.ctx.push(name_str.clone());
                let body_delab = self.delab(body);
                self.ctx.pop();
                SurfaceExpr::Pi(
                    vec![Binder {
                        name: name_str,
                        ty: Some(Box::new(Located::new(ty_delab, Span::new(0, 0, 0, 0)))),
                        info,
                    }],
                    Box::new(Located::new(body_delab, Span::new(0, 0, 0, 0))),
                )
            }
            Expr::Let(_name, ty, val, body) => {
                let ty_delab = self.delab(ty);
                let val_delab = self.delab(val);
                let name_str = "x".to_owned();
                self.ctx.push(name_str.clone());
                let body_delab = self.delab(body);
                self.ctx.pop();
                SurfaceExpr::Let(
                    name_str,
                    Some(Box::new(Located::new(ty_delab, Span::new(0, 0, 0, 0)))),
                    Box::new(Located::new(val_delab, Span::new(0, 0, 0, 0))),
                    Box::new(Located::new(body_delab, Span::new(0, 0, 0, 0))),
                )
            }
            Expr::Proj(_name, _idx, _inner) => self.delab(_inner),
        }
    }
    /// Delaborate to a string using the registered print options.
    #[allow(dead_code)]
    pub fn delab_to_string(&mut self, expr: &Expr) -> String {
        let surface = self.delab(expr);
        print_surface_expr(&surface, &PrintOptions::default())
    }
    /// Add a notation to the registry.
    #[allow(dead_code)]
    pub fn add_notation(&mut self, entry: NotationEntryExt) {
        self.notations.register(entry);
    }
}
/// A pattern for abbreviation detection.
#[derive(Clone, Debug)]
pub struct AbbreviationPattern {
    /// Pattern name (e.g., "nat_literal").
    pub name: String,
    /// The detector function is represented as a pattern kind.
    pub kind: AbbreviationKind,
}
/// A simple cache for delaborated expressions.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DelabCache {
    cache: HashMap<u64, SurfaceExpr>,
    hits: usize,
    misses: usize,
}
impl DelabCache {
    /// Create a new empty cache.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Hash an expression for use as a cache key (very simplified).
    #[allow(dead_code)]
    pub fn hash_expr(expr: &Expr) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        format!("{:?}", expr).hash(&mut hasher);
        hasher.finish()
    }
    /// Look up a cached delaboration.
    #[allow(dead_code)]
    pub fn get(&mut self, expr: &Expr) -> Option<&SurfaceExpr> {
        let key = Self::hash_expr(expr);
        if self.cache.contains_key(&key) {
            self.hits += 1;
            self.cache.get(&key)
        } else {
            self.misses += 1;
            None
        }
    }
    /// Insert a delaboration result.
    #[allow(dead_code)]
    pub fn insert(&mut self, expr: &Expr, result: SurfaceExpr) {
        let key = Self::hash_expr(expr);
        self.cache.insert(key, result);
    }
    /// Cache hit rate (0.0 to 1.0).
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    /// Clear the cache.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    /// Number of cached entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Whether the cache is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
/// Delaboration context: tracks local variable names and depth.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DelabContextExt {
    /// Local variable names in scope (most recent at end).
    pub locals: Vec<String>,
    /// Current recursion depth.
    pub depth: usize,
    /// Maximum allowed depth.
    pub max_depth: usize,
    /// Set of names currently in scope (for freshness checks).
    pub in_scope: HashSet<String>,
}
impl DelabContextExt {
    /// Create a new empty context.
    #[allow(dead_code)]
    pub fn new(max_depth: usize) -> Self {
        Self {
            locals: Vec::new(),
            depth: 0,
            max_depth,
            in_scope: HashSet::new(),
        }
    }
    /// Push a local variable onto the context.
    #[allow(dead_code)]
    pub fn push(&mut self, name: String) {
        self.in_scope.insert(name.clone());
        self.locals.push(name);
        self.depth += 1;
    }
    /// Pop the most recent local variable.
    #[allow(dead_code)]
    pub fn pop(&mut self) {
        if let Some(name) = self.locals.pop() {
            self.in_scope.remove(&name);
            if self.depth > 0 {
                self.depth -= 1;
            }
        }
    }
    /// Look up the name for bound variable index `i` (0 = innermost).
    #[allow(dead_code)]
    pub fn lookup_bvar(&self, i: u32) -> Option<&str> {
        let idx = self.locals.len().checked_sub(i as usize + 1)?;
        self.locals.get(idx).map(|s| s.as_str())
    }
    /// Check whether a name is in scope.
    #[allow(dead_code)]
    pub fn is_in_scope(&self, name: &str) -> bool {
        self.in_scope.contains(name)
    }
    /// Generate a fresh name not already in scope.
    #[allow(dead_code)]
    pub fn fresh(&self, hint: &str) -> String {
        fresh_name(hint, &self.in_scope)
    }
    /// Whether we have exceeded the max depth.
    #[allow(dead_code)]
    pub fn at_max_depth(&self) -> bool {
        self.depth >= self.max_depth
    }
}
