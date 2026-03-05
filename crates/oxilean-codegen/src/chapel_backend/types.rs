//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
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
    pub(super) buf: String,
    /// Current indentation level
    pub(super) indent: usize,
    /// Indentation string (spaces per level)
    pub(super) indent_str: String,
    /// Configuration
    pub(super) config: ChapelConfig,
    /// String interning for deduplication
    pub(super) _intern: HashMap<String, u32>,
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
    pub(super) fn push(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    pub(super) fn push_char(&mut self, c: char) {
        self.buf.push(c);
    }
    pub(super) fn newline(&mut self) {
        self.buf.push('\n');
    }
    pub(super) fn emit_indent(&mut self) {
        for _ in 0..self.indent {
            self.buf.push_str(&self.indent_str.clone());
        }
    }
    pub(super) fn emit_line(&mut self, s: &str) {
        self.emit_indent();
        self.push(s);
        self.newline();
    }
    pub(super) fn indent_in(&mut self) {
        self.indent += 1;
    }
    pub(super) fn indent_out(&mut self) {
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
    pub(super) fn emit_expr_paren(&mut self, expr: &ChapelExpr) {
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
    pub(super) fn emit_param(&mut self, param: &ChapelParam) {
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
/// Intent for procedure parameters.
#[derive(Debug, Clone, PartialEq)]
pub enum ChapelIntent {
    In,
    Out,
    InOut,
    Ref,
    Const,
    ConstRef,
    Param,
    Type,
}
#[allow(dead_code)]
pub struct ChplPassRegistry {
    pub(super) configs: Vec<ChplPassConfig>,
    pub(super) stats: std::collections::HashMap<String, ChplPassStats>,
}
impl ChplPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ChplPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: ChplPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), ChplPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ChplPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&ChplPassStats> {
        self.stats.get(name)
    }
    #[allow(dead_code)]
    pub fn total_passes(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn enabled_count(&self) -> usize {
        self.enabled_passes().len()
    }
    #[allow(dead_code)]
    pub fn update_stats(&mut self, name: &str, changes: u64, time_ms: u64, iter: u32) {
        if let Some(stats) = self.stats.get_mut(name) {
            stats.record_run(changes, time_ms, iter);
        }
    }
}
/// Constant folding helper for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ChapelExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl ChapelExtConstFolder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            folds: 0,
            failures: 0,
            enabled: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn sub_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn mul_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn div_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn rem_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_rem(b)
        }
    }
    #[allow(dead_code)]
    pub fn neg_i64(&mut self, a: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn shl_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shl(s)
        }
    }
    #[allow(dead_code)]
    pub fn shr_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shr(s)
        }
    }
    #[allow(dead_code)]
    pub fn and_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a & b
    }
    #[allow(dead_code)]
    pub fn or_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a | b
    }
    #[allow(dead_code)]
    pub fn xor_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a ^ b
    }
    #[allow(dead_code)]
    pub fn not_i64(&mut self, a: i64) -> i64 {
        self.folds += 1;
        !a
    }
    #[allow(dead_code)]
    pub fn fold_count(&self) -> usize {
        self.folds
    }
    #[allow(dead_code)]
    pub fn failure_count(&self) -> usize {
        self.failures
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplPassConfig {
    pub phase: ChplPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl ChplPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: ChplPassPhase) -> Self {
        ChplPassConfig {
            phase,
            enabled: true,
            max_iterations: 10,
            debug_output: false,
            pass_name: name.into(),
        }
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self) -> Self {
        self.debug_output = true;
        self
    }
    #[allow(dead_code)]
    pub fn max_iter(mut self, n: u32) -> Self {
        self.max_iterations = n;
        self
    }
}
/// Chapel type representation.
#[derive(Debug, Clone, PartialEq)]
pub enum ChapelType {
    /// `int(8)` / `int(16)` / `int(32)` / `int(64)` (default `int`)
    Int(Option<u32>),
    /// `uint(8)` / `uint(16)` / `uint(32)` / `uint(64)` (default `uint`)
    UInt(Option<u32>),
    /// `real(32)` / `real(64)` (default `real`)
    Real(Option<u32>),
    /// `imag(32)` / `imag(64)` (default `imag`)
    Imag(Option<u32>),
    /// `complex(64)` / `complex(128)` (default `complex`)
    Complex(Option<u32>),
    /// `bool`
    Bool,
    /// `string`
    String,
    /// `bytes`
    Bytes,
    /// `range(idxType)` — e.g. `1..n`
    Range(Box<ChapelType>),
    /// `domain(rank, idxType)` — multi-dimensional index set
    Domain(u32, Box<ChapelType>),
    /// `[D] eltType` — array over a domain
    Array(Box<ChapelType>, Box<ChapelType>),
    /// Record type: `record R { ... }`
    Record(String),
    /// Class type: `class C { ... }`
    Class(String),
    /// Union type: `union U { ... }`
    Union(String),
    /// Enum type: `enum E { ... }`
    EnumType(String),
    /// Procedure/function type: `proc(argTypes) : retType`
    ProcType(Vec<ChapelType>, Box<ChapelType>),
    /// Tuple type: `(t1, t2, ...)`
    Tuple(Vec<ChapelType>),
    /// Named / user-defined type
    Named(String),
    /// `void` (no return)
    Void,
    /// Type variable / generic
    TypeVar(String),
    /// `atomic T`
    Atomic(Box<ChapelType>),
    /// `sync T`
    Sync(Box<ChapelType>),
    /// `single T`
    Single(Box<ChapelType>),
    /// Pointer to owned object: `owned C`
    Owned(Box<ChapelType>),
    /// Shared: `shared C`
    Shared(Box<ChapelType>),
    /// Unmanaged: `unmanaged C`
    Unmanaged(Box<ChapelType>),
}
/// Pass execution phase for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChapelExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl ChapelExtPassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}
/// Configuration for the Chapel backend emitter.
#[derive(Debug, Clone)]
pub struct ChapelConfig {
    /// Spaces per indentation level
    pub indent_width: usize,
    /// Emit type annotations on var declarations when available
    pub annotate_vars: bool,
    /// Use `writeln` for print calls
    pub use_writeln: bool,
}
/// Chapel expression representation.
#[derive(Debug, Clone)]
pub enum ChapelExpr {
    /// Integer literal: `42`
    IntLit(i64),
    /// Real literal: `3.14`
    RealLit(f64),
    /// Bool literal: `true` / `false`
    BoolLit(bool),
    /// String literal: `"hello"`
    StrLit(String),
    /// Variable reference: `x`
    Var(String),
    /// Function/procedure application: `f(a, b, ...)`
    Apply(Box<ChapelExpr>, Vec<ChapelExpr>),
    /// Array index: `a[i]`
    Index(Box<ChapelExpr>, Box<ChapelExpr>),
    /// Field/member access: `r.field`
    FieldAccess(Box<ChapelExpr>, String),
    /// Binary operation: `lhs op rhs`
    BinOp(String, Box<ChapelExpr>, Box<ChapelExpr>),
    /// Unary operation: `op e`
    UnOp(String, Box<ChapelExpr>),
    /// Range: `lo..hi` or `lo..#n`
    RangeLit(Box<ChapelExpr>, Box<ChapelExpr>, bool),
    /// Reduce expression: `+ reduce arr`
    ReduceExpr(String, Box<ChapelExpr>),
    /// Forall expression: `[i in D] f(i)`
    ForallExpr(String, Box<ChapelExpr>, Box<ChapelExpr>),
    /// Coforall expression body reference
    CoforallExpr(String, Box<ChapelExpr>, Box<ChapelExpr>),
    /// Tuple literal: `(e1, e2, ...)`
    TupleLit(Vec<ChapelExpr>),
    /// Array literal: `[e1, e2, ...]`
    ArrayLit(Vec<ChapelExpr>),
    /// Cast: `e : t`
    Cast(Box<ChapelExpr>, ChapelType),
    /// Conditional (ternary): `if cond then t else e`
    IfExpr(Box<ChapelExpr>, Box<ChapelExpr>, Box<ChapelExpr>),
    /// New object: `new C(args...)`
    New(ChapelType, Vec<ChapelExpr>),
    /// `nil`
    Nil,
    /// `here` locale
    Here,
    /// `numLocales`
    NumLocales,
    /// `this` reference
    This,
    /// Type query: `e.type`
    TypeOf(Box<ChapelExpr>),
    /// Domain literal: `{e1, e2, ...}` or `{lo..hi}`
    DomainLit(Vec<ChapelExpr>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, ChplCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl ChplAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        ChplAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&ChplCacheEntry> {
        if self.entries.contains_key(key) {
            self.hits += 1;
            self.entries.get(key)
        } else {
            self.misses += 1;
            None
        }
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, data: Vec<u8>) {
        if self.entries.len() >= self.max_size {
            if let Some(oldest) = self.entries.keys().next().cloned() {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(
            key.clone(),
            ChplCacheEntry {
                key,
                data,
                timestamp: 0,
                valid: true,
            },
        );
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self, key: &str) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.valid = false;
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
/// Chapel statement representation.
#[derive(Debug, Clone)]
pub enum ChapelStmt {
    /// `var name: type = expr;`
    VarDecl(String, Option<ChapelType>, Option<ChapelExpr>),
    /// `const name: type = expr;`
    ConstDecl(String, Option<ChapelType>, ChapelExpr),
    /// `name = expr;`
    Assign(ChapelExpr, ChapelExpr),
    /// Compound assign: `name op= expr;`
    CompoundAssign(String, ChapelExpr, ChapelExpr),
    /// `if cond { ... } else { ... }`
    IfElse(ChapelExpr, Vec<ChapelStmt>, Option<Vec<ChapelStmt>>),
    /// `for idx in domain { ... }`
    ForLoop(String, ChapelExpr, Vec<ChapelStmt>),
    /// `forall idx in domain { ... }`
    ForallLoop(String, ChapelExpr, Vec<ChapelStmt>),
    /// `forall idx in domain with (op reduce acc) { ... }`
    ForallReduce(String, ChapelExpr, String, String, Vec<ChapelStmt>),
    /// `coforall idx in domain { ... }`
    CoforallLoop(String, ChapelExpr, Vec<ChapelStmt>),
    /// `while cond { ... }`
    WhileLoop(ChapelExpr, Vec<ChapelStmt>),
    /// `do { ... } while cond;`
    DoWhileLoop(Vec<ChapelStmt>, ChapelExpr),
    /// `return expr;`
    ReturnStmt(Option<ChapelExpr>),
    /// Procedure definition (nested or top-level)
    ProcDef(ChapelProc),
    /// Record definition
    RecordDef(ChapelRecord),
    /// Class definition
    ClassDef(ChapelClass),
    /// Expression statement: `expr;`
    ExprStmt(ChapelExpr),
    /// `writeln(args...);`
    Writeln(Vec<ChapelExpr>),
    /// `write(args...);`
    Write(Vec<ChapelExpr>),
    /// `break;`
    Break,
    /// `continue;`
    Continue,
    /// `halt(msg);`
    Halt(String),
    /// `on locale { ... }`
    On(ChapelExpr, Vec<ChapelStmt>),
    /// `begin { ... }` — async task
    Begin(Vec<ChapelStmt>),
    /// `sync { ... }` — synchronisation block
    SyncBlock(Vec<ChapelStmt>),
    /// Block comment
    Comment(String),
    /// Blank line separator
    Blank,
}
/// Analysis cache for ChapelExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ChapelExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl ChapelExtCache {
    #[allow(dead_code)]
    pub fn new(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            cap,
            total_hits: 0,
            total_misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: u64) -> Option<&[u8]> {
        for e in self.entries.iter_mut() {
            if e.0 == key && e.2 {
                e.3 += 1;
                self.total_hits += 1;
                return Some(&e.1);
            }
        }
        self.total_misses += 1;
        None
    }
    #[allow(dead_code)]
    pub fn put(&mut self, key: u64, data: Vec<u8>) {
        if self.entries.len() >= self.cap {
            self.entries.retain(|e| e.2);
            if self.entries.len() >= self.cap {
                self.entries.remove(0);
            }
        }
        self.entries.push((key, data, true, 0));
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        for e in self.entries.iter_mut() {
            e.2 = false;
        }
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let t = self.total_hits + self.total_misses;
        if t == 0 {
            0.0
        } else {
            self.total_hits as f64 / t as f64
        }
    }
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| e.2).count()
    }
}
/// A Chapel record definition.
#[derive(Debug, Clone)]
pub struct ChapelRecord {
    /// Record name
    pub name: String,
    /// Fields
    pub fields: Vec<ChapelField>,
    /// Methods
    pub methods: Vec<ChapelProc>,
    /// Optional generic type parameters
    pub type_params: Vec<String>,
}
impl ChapelRecord {
    /// Create an empty record.
    pub fn new(name: impl Into<String>) -> Self {
        ChapelRecord {
            name: name.into(),
            fields: vec![],
            methods: vec![],
            type_params: vec![],
        }
    }
    /// Add a field.
    pub fn add_field(&mut self, field: ChapelField) {
        self.fields.push(field);
    }
    /// Add a method.
    pub fn add_method(&mut self, method: ChapelProc) {
        self.methods.push(method);
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl ChplLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        ChplLivenessInfo {
            live_in: vec![std::collections::HashSet::new(); block_count],
            live_out: vec![std::collections::HashSet::new(); block_count],
            defs: vec![std::collections::HashSet::new(); block_count],
            uses: vec![std::collections::HashSet::new(); block_count],
        }
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, block: usize, var: u32) {
        if block < self.defs.len() {
            self.defs[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, block: usize, var: u32) {
        if block < self.uses.len() {
            self.uses[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn is_live_in(&self, block: usize, var: u32) -> bool {
        self.live_in
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn is_live_out(&self, block: usize, var: u32) -> bool {
        self.live_out
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl ChplDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        ChplDominatorTree {
            idom: vec![None; size],
            dom_children: vec![Vec::new(); size],
            dom_depth: vec![0; size],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, idom: u32) {
        self.idom[node] = Some(idom);
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, b: usize) -> bool {
        if a == b {
            return true;
        }
        let mut cur = b;
        loop {
            match self.idom[cur] {
                Some(parent) if parent as usize == a => return true,
                Some(parent) if parent as usize == cur => return false,
                Some(parent) => cur = parent as usize,
                None => return false,
            }
        }
    }
    #[allow(dead_code)]
    pub fn depth(&self, node: usize) -> u32 {
        self.dom_depth.get(node).copied().unwrap_or(0)
    }
}
/// A Chapel procedure parameter.
#[derive(Debug, Clone)]
pub struct ChapelParam {
    /// Parameter name
    pub name: String,
    /// Optional type annotation
    pub ty: Option<ChapelType>,
    /// Optional intent
    pub intent: Option<ChapelIntent>,
    /// Optional default value
    pub default: Option<ChapelExpr>,
}
impl ChapelParam {
    /// Simple parameter: `name: type`
    pub fn simple(name: impl Into<String>, ty: ChapelType) -> Self {
        ChapelParam {
            name: name.into(),
            ty: Some(ty),
            intent: None,
            default: None,
        }
    }
    /// Parameter with intent: `intent name: type`
    pub fn with_intent(name: impl Into<String>, ty: ChapelType, intent: ChapelIntent) -> Self {
        ChapelParam {
            name: name.into(),
            ty: Some(ty),
            intent: Some(intent),
            default: None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ChplPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl ChplPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record_run(&mut self, changes: u64, time_ms: u64, iterations: u32) {
        self.total_runs += 1;
        self.successful_runs += 1;
        self.total_changes += changes;
        self.time_ms += time_ms;
        self.iterations_used = iterations;
    }
    #[allow(dead_code)]
    pub fn average_changes_per_run(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.total_changes as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.successful_runs as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn format_summary(&self) -> String {
        format!(
            "Runs: {}/{}, Changes: {}, Time: {}ms",
            self.successful_runs, self.total_runs, self.total_changes, self.time_ms
        )
    }
}
/// Liveness analysis for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ChapelExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl ChapelExtLiveness {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            live_in: vec![Vec::new(); n],
            live_out: vec![Vec::new(); n],
            defs: vec![Vec::new(); n],
            uses: vec![Vec::new(); n],
        }
    }
    #[allow(dead_code)]
    pub fn live_in(&self, b: usize, v: usize) -> bool {
        self.live_in.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn live_out(&self, b: usize, v: usize) -> bool {
        self.live_out
            .get(b)
            .map(|s| s.contains(&v))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, b: usize, v: usize) {
        if let Some(s) = self.defs.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, b: usize, v: usize) {
        if let Some(s) = self.uses.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn var_is_used_in_block(&self, b: usize, v: usize) -> bool {
        self.uses.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn var_is_def_in_block(&self, b: usize, v: usize) -> bool {
        self.defs.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
}
/// A Chapel procedure (function).
#[derive(Debug, Clone)]
pub struct ChapelProc {
    /// Procedure name
    pub name: String,
    /// Parameters
    pub params: Vec<ChapelParam>,
    /// Return type (None = void)
    pub return_type: Option<ChapelType>,
    /// Body statements
    pub body: Vec<ChapelStmt>,
    /// Whether this is a parallel iterator (`iter` keyword)
    pub is_iter: bool,
    /// Whether the proc is `inline`
    pub is_inline: bool,
    /// Whether the proc is `override`
    pub is_override: bool,
    /// Whether the proc is `operator`
    pub is_operator: bool,
    /// `where` clause expression
    pub where_clause: Option<String>,
}
impl ChapelProc {
    /// Create a simple procedure.
    pub fn new(
        name: impl Into<String>,
        params: Vec<ChapelParam>,
        return_type: Option<ChapelType>,
        body: Vec<ChapelStmt>,
    ) -> Self {
        ChapelProc {
            name: name.into(),
            params,
            return_type,
            body,
            is_iter: false,
            is_inline: false,
            is_override: false,
            is_operator: false,
            where_clause: None,
        }
    }
    /// Mark as parallel iterator.
    pub fn as_iter(mut self) -> Self {
        self.is_iter = true;
        self
    }
    /// Mark as inline.
    pub fn as_inline(mut self) -> Self {
        self.is_inline = true;
        self
    }
    /// Add a `where` clause.
    pub fn with_where(mut self, clause: impl Into<String>) -> Self {
        self.where_clause = Some(clause.into());
        self
    }
}
/// A field/member of a record or class.
#[derive(Debug, Clone)]
pub struct ChapelField {
    /// Field name
    pub name: String,
    /// Field type
    pub ty: ChapelType,
    /// Whether the field is `var` or `const`
    pub is_const: bool,
    /// Optional default value
    pub default: Option<ChapelExpr>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Dominator tree for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChapelExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl ChapelExtDomTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            idom: vec![None; n],
            children: vec![Vec::new(); n],
            depth: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, dom: usize) {
        if node < self.idom.len() {
            self.idom[node] = Some(dom);
            if dom < self.children.len() {
                self.children[dom].push(node);
            }
            self.depth[node] = if dom < self.depth.len() {
                self.depth[dom] + 1
            } else {
                1
            };
        }
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        if a == b {
            return true;
        }
        let n = self.idom.len();
        for _ in 0..n {
            match self.idom.get(b).copied().flatten() {
                None => return false,
                Some(p) if p == a => return true,
                Some(p) if p == b => return false,
                Some(p) => b = p,
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn children_of(&self, n: usize) -> &[usize] {
        self.children.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn depth_of(&self, n: usize) -> usize {
        self.depth.get(n).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.idom.len();
        for _ in 0..(2 * n) {
            if a == b {
                return a;
            }
            if self.depth_of(a) > self.depth_of(b) {
                a = self.idom.get(a).and_then(|x| *x).unwrap_or(a);
            } else {
                b = self.idom.get(b).and_then(|x| *x).unwrap_or(b);
            }
        }
        0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl ChplWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ChplWorklist {
            items: std::collections::VecDeque::new(),
            in_worklist: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: u32) -> bool {
        if self.in_worklist.insert(item) {
            self.items.push_back(item);
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<u32> {
        let item = self.items.pop_front()?;
        self.in_worklist.remove(&item);
        Some(item)
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    pub fn contains(&self, item: u32) -> bool {
        self.in_worklist.contains(&item)
    }
}
/// Statistics for ChapelExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ChapelExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl ChapelExtPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn visit(&mut self) {
        self.nodes_visited += 1;
    }
    #[allow(dead_code)]
    pub fn modify(&mut self) {
        self.nodes_modified += 1;
        self.changed = true;
    }
    #[allow(dead_code)]
    pub fn iterate(&mut self) {
        self.iterations += 1;
    }
    #[allow(dead_code)]
    pub fn error(&mut self) {
        self.errors += 1;
    }
    #[allow(dead_code)]
    pub fn efficiency(&self) -> f64 {
        if self.nodes_visited == 0 {
            0.0
        } else {
            self.nodes_modified as f64 / self.nodes_visited as f64
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, o: &ChapelExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// A Chapel class definition.
#[derive(Debug, Clone)]
pub struct ChapelClass {
    /// Class name
    pub name: String,
    /// Optional parent class
    pub parent: Option<String>,
    /// Fields
    pub fields: Vec<ChapelField>,
    /// Methods
    pub methods: Vec<ChapelProc>,
    /// Optional generic type parameters
    pub type_params: Vec<String>,
}
impl ChapelClass {
    /// Create an empty class.
    pub fn new(name: impl Into<String>) -> Self {
        ChapelClass {
            name: name.into(),
            parent: None,
            fields: vec![],
            methods: vec![],
            type_params: vec![],
        }
    }
    /// Set the parent class.
    pub fn with_parent(mut self, parent: impl Into<String>) -> Self {
        self.parent = Some(parent.into());
        self
    }
    /// Add a field.
    pub fn add_field(&mut self, field: ChapelField) {
        self.fields.push(field);
    }
    /// Add a method.
    pub fn add_method(&mut self, method: ChapelProc) {
        self.methods.push(method);
    }
}
/// Configuration for ChapelExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChapelExtPassConfig {
    pub name: String,
    pub phase: ChapelExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl ChapelExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: ChapelExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: ChapelExtPassPhase) -> Self {
        self.phase = phase;
        self
    }
    #[allow(dead_code)]
    pub fn with_max_iter(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self, d: u32) -> Self {
        self.debug = d;
        self
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
    #[allow(dead_code)]
    pub fn is_debug_enabled(&self) -> bool {
        self.debug > 0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ChplPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl ChplPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            ChplPassPhase::Analysis => "analysis",
            ChplPassPhase::Transformation => "transformation",
            ChplPassPhase::Verification => "verification",
            ChplPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, ChplPassPhase::Transformation | ChplPassPhase::Cleanup)
    }
}
/// A Chapel source module (single `.chpl` file).
#[derive(Debug, Clone)]
pub struct ChapelModule {
    /// Module name (None = implicit top-level)
    pub name: Option<String>,
    /// `use` imports
    pub uses: Vec<String>,
    /// `require` directives
    pub requires: Vec<String>,
    /// Top-level constant declarations
    pub globals: Vec<(String, ChapelType, ChapelExpr)>,
    /// Top-level config variable declarations
    pub configs: Vec<(String, ChapelType, Option<ChapelExpr>)>,
    /// Top-level procedures
    pub procs: Vec<ChapelProc>,
    /// Top-level record definitions
    pub records: Vec<ChapelRecord>,
    /// Top-level class definitions
    pub classes: Vec<ChapelClass>,
    /// Sub-modules
    pub submodules: Vec<ChapelModule>,
    /// Module-level doc comment
    pub doc: Option<String>,
}
impl ChapelModule {
    /// Create an empty unnamed module.
    pub fn new() -> Self {
        ChapelModule {
            name: None,
            uses: vec![],
            requires: vec![],
            globals: vec![],
            configs: vec![],
            procs: vec![],
            records: vec![],
            classes: vec![],
            submodules: vec![],
            doc: None,
        }
    }
    /// Create a named module.
    pub fn named(name: impl Into<String>) -> Self {
        let mut m = ChapelModule::new();
        m.name = Some(name.into());
        m
    }
    /// Add a `use` directive.
    pub fn add_use(&mut self, name: impl Into<String>) {
        self.uses.push(name.into());
    }
    /// Add a `require` directive.
    pub fn add_require(&mut self, path: impl Into<String>) {
        self.requires.push(path.into());
    }
    /// Add a top-level constant.
    pub fn add_global(&mut self, name: impl Into<String>, ty: ChapelType, expr: ChapelExpr) {
        self.globals.push((name.into(), ty, expr));
    }
    /// Add a config variable.
    pub fn add_config(
        &mut self,
        name: impl Into<String>,
        ty: ChapelType,
        default: Option<ChapelExpr>,
    ) {
        self.configs.push((name.into(), ty, default));
    }
    /// Add a procedure.
    pub fn add_proc(&mut self, proc: ChapelProc) {
        self.procs.push(proc);
    }
    /// Add a record.
    pub fn add_record(&mut self, rec: ChapelRecord) {
        self.records.push(rec);
    }
    /// Add a class.
    pub fn add_class(&mut self, cls: ChapelClass) {
        self.classes.push(cls);
    }
    /// Set the doc comment.
    pub fn set_doc(&mut self, doc: impl Into<String>) {
        self.doc = Some(doc.into());
    }
}
/// Worklist for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChapelExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl ChapelExtWorklist {
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            items: std::collections::VecDeque::new(),
            present: vec![false; capacity],
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_back(id);
        }
    }
    #[allow(dead_code)]
    pub fn push_front(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_front(id);
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<usize> {
        let id = self.items.pop_front()?;
        if id < self.present.len() {
            self.present[id] = false;
        }
        Some(id)
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    pub fn contains(&self, id: usize) -> bool {
        id < self.present.len() && self.present[id]
    }
    #[allow(dead_code)]
    pub fn drain_all(&mut self) -> Vec<usize> {
        let v: Vec<usize> = self.items.drain(..).collect();
        for &id in &v {
            if id < self.present.len() {
                self.present[id] = false;
            }
        }
        v
    }
}
/// Dependency graph for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChapelExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl ChapelExtDepGraph {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
            rev: vec![Vec::new(); n],
            edge_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n && to < self.n {
            if !self.adj[from].contains(&to) {
                self.adj[from].push(to);
                self.rev[to].push(from);
                self.edge_count += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn succs(&self, n: usize) -> &[usize] {
        self.adj.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn preds(&self, n: usize) -> &[usize] {
        self.rev.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn topo_sort(&self) -> Option<Vec<usize>> {
        let mut deg: Vec<usize> = (0..self.n).map(|i| self.rev[i].len()).collect();
        let mut q: std::collections::VecDeque<usize> =
            (0..self.n).filter(|&i| deg[i] == 0).collect();
        let mut out = Vec::with_capacity(self.n);
        while let Some(u) = q.pop_front() {
            out.push(u);
            for &v in &self.adj[u] {
                deg[v] -= 1;
                if deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if out.len() == self.n {
            Some(out)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topo_sort().is_none()
    }
    #[allow(dead_code)]
    pub fn reachable(&self, start: usize) -> Vec<usize> {
        let mut vis = vec![false; self.n];
        let mut stk = vec![start];
        let mut out = Vec::new();
        while let Some(u) = stk.pop() {
            if u < self.n && !vis[u] {
                vis[u] = true;
                out.push(u);
                for &v in &self.adj[u] {
                    if !vis[v] {
                        stk.push(v);
                    }
                }
            }
        }
        out
    }
    #[allow(dead_code)]
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        for i in 0..self.n {
            if !visited[i] {
                let mut stk = vec![(i, 0usize)];
                while let Some((u, idx)) = stk.last_mut() {
                    if !visited[*u] {
                        visited[*u] = true;
                    }
                    if *idx < self.adj[*u].len() {
                        let v = self.adj[*u][*idx];
                        *idx += 1;
                        if !visited[v] {
                            stk.push((v, 0));
                        }
                    } else {
                        order.push(*u);
                        stk.pop();
                    }
                }
            }
        }
        let mut comp = vec![usize::MAX; self.n];
        let mut components: Vec<Vec<usize>> = Vec::new();
        for &start in order.iter().rev() {
            if comp[start] == usize::MAX {
                let cid = components.len();
                let mut component = Vec::new();
                let mut stk = vec![start];
                while let Some(u) = stk.pop() {
                    if comp[u] == usize::MAX {
                        comp[u] = cid;
                        component.push(u);
                        for &v in &self.rev[u] {
                            if comp[v] == usize::MAX {
                                stk.push(v);
                            }
                        }
                    }
                }
                components.push(component);
            }
        }
        components
    }
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.n
    }
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}
/// Pass registry for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ChapelExtPassRegistry {
    pub(super) configs: Vec<ChapelExtPassConfig>,
    pub(super) stats: Vec<ChapelExtPassStats>,
}
impl ChapelExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: ChapelExtPassConfig) {
        self.stats.push(ChapelExtPassStats::new());
        self.configs.push(c);
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }
    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> Option<&ChapelExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&ChapelExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ChapelExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &ChapelExtPassPhase) -> Vec<&ChapelExtPassConfig> {
        self.configs
            .iter()
            .filter(|c| c.enabled && &c.phase == ph)
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_nodes_visited(&self) -> usize {
        self.stats.iter().map(|s| s.nodes_visited).sum()
    }
    #[allow(dead_code)]
    pub fn any_changed(&self) -> bool {
        self.stats.iter().any(|s| s.changed)
    }
}
