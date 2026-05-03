//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::impls1::*;
use super::impls2::*;
use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}
impl ChplDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ChplDepGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, id: u32) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }
    #[allow(dead_code)]
    pub fn add_dep(&mut self, dep: u32, dependent: u32) {
        self.add_node(dep);
        self.add_node(dependent);
        self.edges.push((dep, dependent));
    }
    #[allow(dead_code)]
    pub fn dependents_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(d, _)| *d == node)
            .map(|(_, dep)| *dep)
            .collect()
    }
    #[allow(dead_code)]
    pub fn dependencies_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(_, dep)| *dep == node)
            .map(|(d, _)| *d)
            .collect()
    }
    #[allow(dead_code)]
    pub fn topological_sort(&self) -> Vec<u32> {
        let mut in_degree: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for &n in &self.nodes {
            in_degree.insert(n, 0);
        }
        for (_, dep) in &self.edges {
            *in_degree.entry(*dep).or_insert(0) += 1;
        }
        let mut queue: std::collections::VecDeque<u32> = self
            .nodes
            .iter()
            .filter(|&&n| in_degree[&n] == 0)
            .copied()
            .collect();
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node);
            for dep in self.dependents_of(node) {
                let cnt = in_degree.entry(dep).or_insert(0);
                *cnt = cnt.saturating_sub(1);
                if *cnt == 0 {
                    queue.push_back(dep);
                }
            }
        }
        result
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topological_sort().len() < self.nodes.len()
    }
}
#[allow(dead_code)]
pub struct ChplConstantFoldingHelper;
impl ChplConstantFoldingHelper {
    #[allow(dead_code)]
    pub fn fold_add_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn fold_sub_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn fold_mul_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn fold_div_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_add_f64(a: f64, b: f64) -> f64 {
        a + b
    }
    #[allow(dead_code)]
    pub fn fold_mul_f64(a: f64, b: f64) -> f64 {
        a * b
    }
    #[allow(dead_code)]
    pub fn fold_neg_i64(a: i64) -> Option<i64> {
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn fold_not_bool(a: bool) -> bool {
        !a
    }
    #[allow(dead_code)]
    pub fn fold_and_bool(a: bool, b: bool) -> bool {
        a && b
    }
    #[allow(dead_code)]
    pub fn fold_or_bool(a: bool, b: bool) -> bool {
        a || b
    }
    #[allow(dead_code)]
    pub fn fold_shl_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shl(b)
    }
    #[allow(dead_code)]
    pub fn fold_shr_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shr(b)
    }
    #[allow(dead_code)]
    pub fn fold_rem_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            Some(a % b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_bitand_i64(a: i64, b: i64) -> i64 {
        a & b
    }
    #[allow(dead_code)]
    pub fn fold_bitor_i64(a: i64, b: i64) -> i64 {
        a | b
    }
    #[allow(dead_code)]
    pub fn fold_bitxor_i64(a: i64, b: i64) -> i64 {
        a ^ b
    }
    #[allow(dead_code)]
    pub fn fold_bitnot_i64(a: i64) -> i64 {
        !a
    }
}
/// Backend state for emitting Chapel source code.
pub struct ChapelBackend {
    /// Output buffer
    pub(crate) buf: String,
    /// Current indentation level
    pub(crate) indent: usize,
    /// Indentation string (spaces per level)
    pub(crate) indent_str: String,
    /// Configuration
    pub(crate) config: ChapelConfig,
    /// String interning for deduplication
    pub(crate) _intern: HashMap<String, u32>,
}
impl ChapelBackend {
    /// Create a new backend with default configuration.
    pub fn new() -> Self {
        ChapelBackend::with_config(ChapelConfig::default())
    }
    /// Create a new backend with custom configuration.
    pub fn with_config(config: ChapelConfig) -> Self {
        let indent_str = " ".repeat(config.indent_width);
        ChapelBackend {
            buf: String::new(),
            indent: 0,
            indent_str,
            config,
            _intern: HashMap::new(),
        }
    }
    pub(crate) fn push(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    pub(crate) fn push_char(&mut self, c: char) {
        self.buf.push(c);
    }
    pub(crate) fn newline(&mut self) {
        self.buf.push('\n');
    }
    pub(crate) fn emit_indent(&mut self) {
        for _ in 0..self.indent {
            self.buf.push_str(&self.indent_str.clone());
        }
    }
    pub(crate) fn emit_line(&mut self, s: &str) {
        self.emit_indent();
        self.push(s);
        self.newline();
    }
    pub(crate) fn indent_in(&mut self) {
        self.indent += 1;
    }
    pub(crate) fn indent_out(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }
    /// Emit a Chapel expression.
    pub fn emit_expr(&mut self, expr: &ChapelExpr) {
        match expr {
            ChapelExpr::IntLit(n) => self.push(&n.to_string()),
            ChapelExpr::RealLit(v) => self.push(&format!("{v}")),
            ChapelExpr::BoolLit(b) => self.push(if *b { "true" } else { "false" }),
            ChapelExpr::StrLit(s) => {
                self.push_char('"');
                self.push(s);
                self.push_char('"');
            }
            ChapelExpr::Var(name) => self.push(name),
            ChapelExpr::Apply(f, args) => {
                self.emit_expr(f);
                self.push("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.emit_expr(arg);
                }
                self.push(")");
            }
            ChapelExpr::Index(arr, idx) => {
                self.emit_expr(arr);
                self.push("[");
                self.emit_expr(idx);
                self.push("]");
            }
            ChapelExpr::FieldAccess(obj, field) => {
                self.emit_expr_paren(obj);
                self.push(".");
                self.push(field);
            }
            ChapelExpr::BinOp(op, lhs, rhs) => {
                self.emit_expr_paren(lhs);
                self.push(" ");
                self.push(op);
                self.push(" ");
                self.emit_expr_paren(rhs);
            }
            ChapelExpr::UnOp(op, e) => {
                self.push(op);
                self.emit_expr_paren(e);
            }
            ChapelExpr::RangeLit(lo, hi, count_based) => {
                self.emit_expr(lo);
                if *count_based {
                    self.push("..#");
                } else {
                    self.push("..");
                }
                self.emit_expr(hi);
            }
            ChapelExpr::ReduceExpr(op, arr) => {
                self.push(op);
                self.push(" reduce ");
                self.emit_expr_paren(arr);
            }
            ChapelExpr::ForallExpr(idx, domain, body) => {
                self.push("[");
                self.push(idx);
                self.push(" in ");
                self.emit_expr(domain);
                self.push("] ");
                self.emit_expr(body);
            }
            ChapelExpr::CoforallExpr(idx, domain, body) => {
                self.push("coforall ");
                self.push(idx);
                self.push(" in ");
                self.emit_expr(domain);
                self.push(" { ");
                self.emit_expr(body);
                self.push(" }");
            }
            ChapelExpr::TupleLit(elems) => {
                self.push("(");
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.emit_expr(e);
                }
                self.push(")");
            }
            ChapelExpr::ArrayLit(elems) => {
                self.push("[");
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.emit_expr(e);
                }
                self.push("]");
            }
            ChapelExpr::Cast(e, ty) => {
                self.emit_expr_paren(e);
                self.push(": ");
                self.push(&ty.to_string());
            }
            ChapelExpr::IfExpr(cond, t, e) => {
                self.push("if ");
                self.emit_expr(cond);
                self.push(" then ");
                self.emit_expr(t);
                self.push(" else ");
                self.emit_expr(e);
            }
            ChapelExpr::New(ty, args) => {
                self.push("new ");
                self.push(&ty.to_string());
                self.push("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.emit_expr(arg);
                }
                self.push(")");
            }
            ChapelExpr::Nil => self.push("nil"),
            ChapelExpr::Here => self.push("here"),
            ChapelExpr::NumLocales => self.push("numLocales"),
            ChapelExpr::This => self.push("this"),
            ChapelExpr::TypeOf(e) => {
                self.emit_expr(e);
                self.push(".type");
            }
            ChapelExpr::DomainLit(dims) => {
                self.push("{");
                for (i, d) in dims.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.emit_expr(d);
                }
                self.push("}");
            }
        }
    }
    /// Emit an expression, adding parentheses for complex forms.
    pub(crate) fn emit_expr_paren(&mut self, expr: &ChapelExpr) {
        let needs_paren = matches!(
            expr,
            ChapelExpr::BinOp(..)
                | ChapelExpr::IfExpr(..)
                | ChapelExpr::ForallExpr(..)
                | ChapelExpr::CoforallExpr(..)
                | ChapelExpr::Cast(..)
        );
        if needs_paren {
            self.push("(");
            self.emit_expr(expr);
            self.push(")");
        } else {
            self.emit_expr(expr);
        }
    }
    pub(crate) fn emit_param(&mut self, param: &ChapelParam) {
        if let Some(intent) = &param.intent {
            self.push(&intent.to_string());
            self.push(" ");
        }
        self.push(&param.name);
        if let Some(ty) = &param.ty {
            self.push(": ");
            self.push(&ty.to_string());
        }
        if let Some(def) = &param.default {
            self.push(" = ");
            let def = def.clone();
            self.emit_expr(&def);
        }
    }
    /// Emit a single Chapel statement.
    pub fn emit_stmt(&mut self, stmt: &ChapelStmt) {
        match stmt {
            ChapelStmt::VarDecl(name, ty, init) => {
                self.emit_indent();
                self.push("var ");
                self.push(name);
                if let Some(t) = ty {
                    if self.config.annotate_vars {
                        self.push(": ");
                        self.push(&t.to_string());
                    }
                }
                if let Some(e) = init {
                    self.push(" = ");
                    let e = e.clone();
                    self.emit_expr(&e);
                }
                self.push(";");
                self.newline();
            }
            ChapelStmt::ConstDecl(name, ty, expr) => {
                self.emit_indent();
                self.push("const ");
                self.push(name);
                if let Some(t) = ty {
                    if self.config.annotate_vars {
                        self.push(": ");
                        self.push(&t.to_string());
                    }
                }
                self.push(" = ");
                let expr = expr.clone();
                self.emit_expr(&expr);
                self.push(";");
                self.newline();
            }
            ChapelStmt::Assign(lhs, rhs) => {
                self.emit_indent();
                let lhs = lhs.clone();
                let rhs = rhs.clone();
                self.emit_expr(&lhs);
                self.push(" = ");
                self.emit_expr(&rhs);
                self.push(";");
                self.newline();
            }
            ChapelStmt::CompoundAssign(op, lhs, rhs) => {
                self.emit_indent();
                let lhs = lhs.clone();
                let rhs = rhs.clone();
                self.emit_expr(&lhs);
                self.push(" ");
                self.push(op);
                self.push("= ");
                self.emit_expr(&rhs);
                self.push(";");
                self.newline();
            }
            ChapelStmt::IfElse(cond, then_body, else_body) => {
                self.emit_indent();
                self.push("if ");
                let cond = cond.clone();
                self.emit_expr(&cond);
                self.push(" {");
                self.newline();
                self.indent_in();
                let then_body = then_body.clone();
                for s in &then_body {
                    self.emit_stmt(s);
                }
                self.indent_out();
                if let Some(eb) = else_body {
                    self.emit_indent();
                    self.push("} else {");
                    self.newline();
                    self.indent_in();
                    let eb = eb.clone();
                    for s in &eb {
                        self.emit_stmt(s);
                    }
                    self.indent_out();
                }
                self.emit_line("}");
            }
            ChapelStmt::ForLoop(idx, domain, body) => {
                self.emit_indent();
                self.push("for ");
                self.push(idx);
                self.push(" in ");
                let domain = domain.clone();
                self.emit_expr(&domain);
                self.push(" {");
                self.newline();
                self.indent_in();
                let body = body.clone();
                for s in &body {
                    self.emit_stmt(s);
                }
                self.indent_out();
                self.emit_line("}");
            }
            ChapelStmt::ForallLoop(idx, domain, body) => {
                self.emit_indent();
                self.push("forall ");
                self.push(idx);
                self.push(" in ");
                let domain = domain.clone();
                self.emit_expr(&domain);
                self.push(" {");
                self.newline();
                self.indent_in();
                let body = body.clone();
                for s in &body {
                    self.emit_stmt(s);
                }
                self.indent_out();
                self.emit_line("}");
            }
            ChapelStmt::ForallReduce(idx, domain, op, acc, body) => {
                self.emit_indent();
                self.push("forall ");
                self.push(idx);
                self.push(" in ");
                let domain = domain.clone();
                self.emit_expr(&domain);
                self.push(" with (");
                self.push(op);
                self.push(" reduce ");
                self.push(acc);
                self.push(") {");
                self.newline();
                self.indent_in();
                let body = body.clone();
                for s in &body {
                    self.emit_stmt(s);
                }
                self.indent_out();
                self.emit_line("}");
            }
            ChapelStmt::CoforallLoop(idx, domain, body) => {
                self.emit_indent();
                self.push("coforall ");
                self.push(idx);
                self.push(" in ");
                let domain = domain.clone();
                self.emit_expr(&domain);
                self.push(" {");
                self.newline();
                self.indent_in();
                let body = body.clone();
                for s in &body {
                    self.emit_stmt(s);
                }
                self.indent_out();
                self.emit_line("}");
            }
            ChapelStmt::WhileLoop(cond, body) => {
                self.emit_indent();
                self.push("while ");
                let cond = cond.clone();
                self.emit_expr(&cond);
                self.push(" {");
                self.newline();
                self.indent_in();
                let body = body.clone();
                for s in &body {
                    self.emit_stmt(s);
                }
                self.indent_out();
                self.emit_line("}");
            }
            ChapelStmt::DoWhileLoop(body, cond) => {
                self.emit_line("do {");
                self.indent_in();
                let body = body.clone();
                for s in &body {
                    self.emit_stmt(s);
                }
                self.indent_out();
                self.emit_indent();
                self.push("} while ");
                let cond = cond.clone();
                self.emit_expr(&cond);
                self.push(";");
                self.newline();
            }
            ChapelStmt::ReturnStmt(e) => {
                self.emit_indent();
                self.push("return");
                if let Some(expr) = e {
                    self.push(" ");
                    let expr = expr.clone();
                    self.emit_expr(&expr);
                }
                self.push(";");
                self.newline();
            }
            ChapelStmt::ProcDef(proc) => {
                self.emit_proc(proc);
            }
            ChapelStmt::RecordDef(rec) => {
                self.emit_record(rec);
            }
            ChapelStmt::ClassDef(cls) => {
                self.emit_class(cls);
            }
            ChapelStmt::ExprStmt(e) => {
                self.emit_indent();
                let e = e.clone();
                self.emit_expr(&e);
                self.push(";");
                self.newline();
            }
            ChapelStmt::Writeln(args) => {
                self.emit_indent();
                self.push("writeln(");
                let args = args.clone();
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.emit_expr(arg);
                }
                self.push(");");
                self.newline();
            }
            ChapelStmt::Write(args) => {
                self.emit_indent();
                self.push("write(");
                let args = args.clone();
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.emit_expr(arg);
                }
                self.push(");");
                self.newline();
            }
            ChapelStmt::Break => {
                self.emit_line("break;");
            }
            ChapelStmt::Continue => {
                self.emit_line("continue;");
            }
            ChapelStmt::Halt(msg) => {
                self.emit_indent();
                self.push("halt(\"");
                self.push(msg);
                self.push("\");");
                self.newline();
            }
            ChapelStmt::On(locale, body) => {
                self.emit_indent();
                self.push("on ");
                let locale = locale.clone();
                self.emit_expr(&locale);
                self.push(" {");
                self.newline();
                self.indent_in();
                let body = body.clone();
                for s in &body {
                    self.emit_stmt(s);
                }
                self.indent_out();
                self.emit_line("}");
            }
            ChapelStmt::Begin(body) => {
                self.emit_line("begin {");
                self.indent_in();
                let body = body.clone();
                for s in &body {
                    self.emit_stmt(s);
                }
                self.indent_out();
                self.emit_line("}");
            }
            ChapelStmt::SyncBlock(body) => {
                self.emit_line("sync {");
                self.indent_in();
                let body = body.clone();
                for s in &body {
                    self.emit_stmt(s);
                }
                self.indent_out();
                self.emit_line("}");
            }
            ChapelStmt::Comment(text) => {
                self.emit_indent();
                self.push("// ");
                self.push(text);
                self.newline();
            }
            ChapelStmt::Blank => {
                self.newline();
            }
        }
    }
    /// Emit a Chapel procedure definition.
    pub fn emit_proc(&mut self, proc: &ChapelProc) {
        self.emit_indent();
        if proc.is_inline {
            self.push("inline ");
        }
        if proc.is_override {
            self.push("override ");
        }
        if proc.is_iter {
            self.push("iter ");
        } else if proc.is_operator {
            self.push("operator ");
        } else {
            self.push("proc ");
        }
        self.push(&proc.name);
        self.push("(");
        let params = proc.params.clone();
        for (i, param) in params.iter().enumerate() {
            if i > 0 {
                self.push(", ");
            }
            self.emit_param(param);
        }
        self.push(")");
        if let Some(ret) = &proc.return_type {
            self.push(": ");
            self.push(&ret.to_string());
        }
        if let Some(wh) = &proc.where_clause {
            self.push(" where ");
            self.push(wh);
        }
        self.push(" {");
        self.newline();
        self.indent_in();
        let body = proc.body.clone();
        for stmt in &body {
            self.emit_stmt(stmt);
        }
        self.indent_out();
        self.emit_line("}");
        self.newline();
    }
    /// Emit a Chapel record definition.
    pub fn emit_record(&mut self, rec: &ChapelRecord) {
        self.emit_indent();
        self.push("record ");
        self.push(&rec.name);
        if !rec.type_params.is_empty() {
            self.push("(");
            for (i, tp) in rec.type_params.iter().enumerate() {
                if i > 0 {
                    self.push(", ");
                }
                self.push("type ");
                self.push(tp);
            }
            self.push(")");
        }
        self.push(" {");
        self.newline();
        self.indent_in();
        let fields = rec.fields.clone();
        for field in &fields {
            self.emit_indent();
            if field.is_const {
                self.push("const ");
            } else {
                self.push("var ");
            }
            self.push(&field.name);
            self.push(": ");
            self.push(&field.ty.to_string());
            if let Some(def) = &field.default {
                self.push(" = ");
                let def = def.clone();
                self.emit_expr(&def);
            }
            self.push(";");
            self.newline();
        }
        let methods = rec.methods.clone();
        for method in &methods {
            self.emit_proc(method);
        }
        self.indent_out();
        self.emit_line("}");
        self.newline();
    }
    /// Emit a Chapel class definition.
    pub fn emit_class(&mut self, cls: &ChapelClass) {
        self.emit_indent();
        self.push("class ");
        self.push(&cls.name);
        if !cls.type_params.is_empty() {
            self.push("(");
            for (i, tp) in cls.type_params.iter().enumerate() {
                if i > 0 {
                    self.push(", ");
                }
                self.push("type ");
                self.push(tp);
            }
            self.push(")");
        }
        if let Some(parent) = &cls.parent {
            self.push(" : ");
            self.push(parent);
        }
        self.push(" {");
        self.newline();
        self.indent_in();
        let fields = cls.fields.clone();
        for field in &fields {
            self.emit_indent();
            if field.is_const {
                self.push("const ");
            } else {
                self.push("var ");
            }
            self.push(&field.name);
            self.push(": ");
            self.push(&field.ty.to_string());
            if let Some(def) = &field.default {
                self.push(" = ");
                let def = def.clone();
                self.emit_expr(&def);
            }
            self.push(";");
            self.newline();
        }
        let methods = cls.methods.clone();
        for method in &methods {
            self.emit_proc(method);
        }
        self.indent_out();
        self.emit_line("}");
        self.newline();
    }
    /// Emit a complete Chapel module.
    pub fn emit_module(&mut self, module: &ChapelModule) {
        if let Some(doc) = &module.doc.clone() {
            for line in doc.lines() {
                self.push("// ");
                self.push(line);
                self.newline();
            }
            self.newline();
        }
        let has_name = module.name.is_some();
        if let Some(name) = &module.name.clone() {
            self.emit_line(&format!("module {name} {{"));
            self.indent_in();
        }
        for u in &module.uses.clone() {
            self.emit_line(&format!("use {u};"));
        }
        if !module.uses.is_empty() {
            self.newline();
        }
        for r in &module.requires.clone() {
            self.emit_line(&format!("require \"{r}\";"));
        }
        if !module.requires.is_empty() {
            self.newline();
        }
        for (name, ty, def) in &module.configs.clone() {
            self.emit_indent();
            self.push("config var ");
            self.push(name);
            self.push(": ");
            self.push(&ty.to_string());
            if let Some(d) = def {
                self.push(" = ");
                let d = d.clone();
                self.emit_expr(&d);
            }
            self.push(";");
            self.newline();
        }
        if !module.configs.is_empty() {
            self.newline();
        }
        for (name, ty, expr) in &module.globals.clone() {
            self.emit_indent();
            self.push("const ");
            self.push(name);
            self.push(": ");
            self.push(&ty.to_string());
            self.push(" = ");
            let expr = expr.clone();
            self.emit_expr(&expr);
            self.push(";");
            self.newline();
        }
        if !module.globals.is_empty() {
            self.newline();
        }
        for rec in &module.records.clone() {
            self.emit_record(rec);
        }
        for cls in &module.classes.clone() {
            self.emit_class(cls);
        }
        for proc in &module.procs.clone() {
            self.emit_proc(proc);
        }
        for sub in &module.submodules.clone() {
            self.emit_module(sub);
        }
        if has_name {
            self.indent_out();
            self.emit_line("}");
        }
    }
    /// Return the generated source and reset the buffer.
    pub fn finish(&mut self) -> String {
        std::mem::take(&mut self.buf)
    }
    /// Generate a complete `.chpl` file from a module.
    pub fn generate(module: &ChapelModule) -> String {
        let mut backend = ChapelBackend::new();
        backend.emit_module(module);
        backend.finish()
    }
    /// Generate with custom configuration.
    pub fn generate_with_config(module: &ChapelModule, config: ChapelConfig) -> String {
        let mut backend = ChapelBackend::with_config(config);
        backend.emit_module(module);
        backend.finish()
    }
}
