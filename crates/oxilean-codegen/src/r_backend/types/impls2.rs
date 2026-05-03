use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::fmt::Write as FmtWrite;

use super::defs::*;

impl RBackend {
    /// Create a new R backend with default settings.
    pub fn new() -> Self {
        RBackend {
            output: String::new(),
            indent: 0,
            indent_str: "  ".to_string(),
            s4_classes: HashMap::new(),
            s3_generics: Vec::new(),
            vectorized_ops: HashMap::new(),
        }
    }
    /// Take the accumulated output, resetting the buffer.
    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.output)
    }
    /// Register a vectorized operation.
    pub fn register_vectorized(&mut self, name: &str, op: VectorizedOp) {
        self.vectorized_ops.insert(name.to_string(), op);
    }
    pub(crate) fn current_indent(&self) -> String {
        self.indent_str.repeat(self.indent)
    }
    pub(crate) fn emit_line(&mut self, line: &str) {
        let indent = self.current_indent();
        let _ = writeln!(self.output, "{}{}", indent, line);
    }
    pub(crate) fn emit_raw(&mut self, s: &str) {
        self.output.push_str(s);
    }
    pub(crate) fn indent_up(&mut self) {
        self.indent += 1;
    }
    pub(crate) fn indent_down(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }
    /// Emit a complete R file.
    pub fn emit_file(&mut self, file: &RFile) {
        if let Some(shebang) = &file.shebang {
            self.emit_line(shebang);
        }
        if let Some(header) = &file.header_comment {
            for line in header.lines() {
                self.emit_line(&format!("# {}", line));
            }
            self.emit_line("");
        }
        for pkg in &file.imports {
            self.emit_line(&format!("library({})", pkg));
        }
        if !file.imports.is_empty() {
            self.emit_line("");
        }
        for obj in &file.data_objects {
            self.emit_data_object(obj);
        }
        if !file.data_objects.is_empty() {
            self.emit_line("");
        }
        for fun in &file.functions {
            self.emit_function(fun);
            self.emit_line("");
        }
        for stmt in &file.scripts {
            self.emit_stmt(stmt);
        }
    }
    pub(crate) fn emit_data_object(&mut self, obj: &RDataObject) {
        if let Some(comment) = &obj.comment {
            self.emit_line(&format!("# {}", comment));
        }
        let value_str = self.emit_expr(&obj.value);
        self.emit_line(&format!("{} <- {}", obj.name, value_str));
    }
    /// Emit an R function definition.
    pub fn emit_function(&mut self, fun: &RFunction) {
        if let Some(doc) = &fun.doc {
            for line in doc.lines() {
                self.emit_line(&format!("#' {}", line));
            }
        }
        let formals_str = self.emit_formals(&fun.formals);
        self.emit_line(&format!("{} <- function({}) {{", fun.name, formals_str));
        self.indent_up();
        if fun.is_generic {
            self.emit_line(&format!("UseMethod(\"{}\")", fun.name));
        } else {
            for stmt in &fun.body {
                self.emit_stmt(stmt);
            }
        }
        self.indent_down();
        self.emit_line("}");
        for (class, method) in &fun.s3_methods {
            let method_name = format!("{}.{}", fun.name, class);
            let formals_str2 = self.emit_formals(&method.formals);
            self.emit_line(&format!("{} <- function({}) {{", method_name, formals_str2));
            self.indent_up();
            for stmt in &method.body {
                self.emit_stmt(stmt);
            }
            self.indent_down();
            self.emit_line("}");
        }
    }
    pub(crate) fn emit_formals(&self, formals: &[RFormal]) -> String {
        let mut parts = Vec::new();
        for formal in formals {
            if let Some(default) = &formal.default {
                let default_str = self.emit_expr_pure(default);
                parts.push(format!("{} = {}", formal.name, default_str));
            } else {
                parts.push(formal.name.clone());
            }
        }
        parts.join(", ")
    }
    /// Emit a single R statement.
    pub fn emit_stmt(&mut self, stmt: &RStmt) {
        match stmt {
            RStmt::Assign(op, lhs, rhs) => {
                let rhs_str = self.emit_expr(rhs);
                self.emit_line(&format!("{} {} {}", lhs, op, rhs_str));
            }
            RStmt::AssignLhs(op, lhs, rhs) => {
                let lhs_str = self.emit_expr(lhs);
                let rhs_str = self.emit_expr(rhs);
                self.emit_line(&format!("{} {} {}", lhs_str, op, rhs_str));
            }
            RStmt::ForLoop { var, seq, body } => {
                let seq_str = self.emit_expr(seq);
                self.emit_line(&format!("for ({} in {}) {{", var, seq_str));
                self.indent_up();
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                self.emit_line("}");
            }
            RStmt::WhileLoop { cond, body } => {
                let cond_str = self.emit_expr(cond);
                self.emit_line(&format!("while ({}) {{", cond_str));
                self.indent_up();
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                self.emit_line("}");
            }
            RStmt::Repeat(body) => {
                self.emit_line("repeat {");
                self.indent_up();
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                self.emit_line("}");
            }
            RStmt::IfElse {
                cond,
                then_body,
                else_if_branches,
                else_body,
            } => {
                let cond_str = self.emit_expr(cond);
                self.emit_line(&format!("if ({}) {{", cond_str));
                self.indent_up();
                for s in then_body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                for (elif_cond, elif_body) in else_if_branches {
                    let elif_cond_str = self.emit_expr(elif_cond);
                    self.emit_line(&format!("}} else if ({}) {{", elif_cond_str));
                    self.indent_up();
                    for s in elif_body {
                        self.emit_stmt(s);
                    }
                    self.indent_down();
                }
                if let Some(else_stmts) = else_body {
                    self.emit_line("} else {");
                    self.indent_up();
                    for s in else_stmts {
                        self.emit_stmt(s);
                    }
                    self.indent_down();
                }
                self.emit_line("}");
            }
            RStmt::Return(expr) => {
                if let Some(e) = expr {
                    let e_str = self.emit_expr(e);
                    self.emit_line(&format!("return({})", e_str));
                } else {
                    self.emit_line("return(invisible(NULL))");
                }
            }
            RStmt::Next => {
                self.emit_line("next");
            }
            RStmt::Break => {
                self.emit_line("break");
            }
            RStmt::FunctionDef(fun) => {
                self.emit_function(fun);
            }
            RStmt::Library { pkg, use_require } => {
                if *use_require {
                    self.emit_line(&format!("require({}, quietly = TRUE)", pkg));
                } else {
                    self.emit_line(&format!("library({})", pkg));
                }
            }
            RStmt::Source(path) => {
                self.emit_line(&format!("source(\"{}\")", path));
            }
            RStmt::Expr(expr) => {
                let e_str = self.emit_expr(expr);
                self.emit_line(&e_str);
            }
            RStmt::Comment(text) => {
                for line in text.lines() {
                    self.emit_line(&format!("# {}", line));
                }
            }
            RStmt::Stopifnot(conds) => {
                let conds_str: Vec<String> = conds.iter().map(|c| self.emit_expr_pure(c)).collect();
                self.emit_line(&format!("stopifnot({})", conds_str.join(", ")));
            }
            RStmt::TryCatch {
                body,
                handlers,
                finally,
            } => {
                self.emit_line("tryCatch({");
                self.indent_up();
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                self.emit_raw(&self.current_indent());
                self.emit_raw("}");
                for (condition, formal, handler_body) in handlers {
                    self.emit_raw(&format!(
                        ",\n{}{} = function({}) {{\n",
                        self.current_indent(),
                        condition,
                        formal.name
                    ));
                    self.indent_up();
                    let handler_stmts: Vec<String> = handler_body
                        .iter()
                        .map(|s| {
                            let mut tmp = RBackend::new();
                            tmp.indent = self.indent;
                            tmp.emit_stmt(s);
                            tmp.take_output()
                        })
                        .collect();
                    for h in handler_stmts {
                        self.emit_raw(&h);
                    }
                    self.indent_down();
                    self.emit_raw(&format!("{}}}", self.current_indent()));
                }
                if let Some(fin) = finally {
                    self.emit_raw(",\nfinally = {\n");
                    self.indent_up();
                    for s in fin {
                        self.emit_stmt(s);
                    }
                    self.indent_down();
                    self.emit_raw(&format!("{}}}", self.current_indent()));
                }
                self.emit_raw(")\n");
            }
            RStmt::SetMethod {
                generic,
                signature,
                fun,
            } => {
                let sig_str: Vec<String> = signature.iter().map(|s| format!("\"{}\"", s)).collect();
                let formals_str = self.emit_formals(&fun.formals);
                self.emit_line(&format!(
                    "setMethod(\"{}\", signature({}), function({}) {{",
                    generic,
                    sig_str.join(", "),
                    formals_str
                ));
                self.indent_up();
                for s in &fun.body {
                    self.emit_stmt(s);
                }
                self.indent_down();
                self.emit_line("})");
            }
            RStmt::SetClass {
                class,
                contains,
                slots,
            } => {
                let slots_str: Vec<String> = slots
                    .iter()
                    .map(|(name, ty)| format!("{} = \"{}\"", name, ty))
                    .collect();
                let contains_str = if let Some(parent) = contains {
                    format!(", contains = \"{}\"", parent)
                } else {
                    String::new()
                };
                self.emit_line(&format!(
                    "setClass(\"{}\", slots = c({}){})  ",
                    class,
                    slots_str.join(", "),
                    contains_str
                ));
            }
        }
    }
    /// Emit an R expression to a string (may use output buffer for nested stmts).
    pub fn emit_expr(&mut self, expr: &RExpr) -> String {
        self.emit_expr_pure(expr)
    }
    /// Emit an R expression to a string (pure, no side-effects on buffer).
    pub fn emit_expr_pure(&self, expr: &RExpr) -> String {
        match expr {
            RExpr::Lit(lit) => self.emit_literal(lit),
            RExpr::Var(name) => name.clone(),
            RExpr::Call(func, args) => {
                let func_str = self.emit_expr_pure(func);
                let args_str: Vec<String> = args.iter().map(|a| self.emit_arg(a)).collect();
                format!("{}({})", func_str, args_str.join(", "))
            }
            RExpr::InfixOp(op, lhs, rhs) => {
                let lhs_str = self.emit_expr_pure(lhs);
                let rhs_str = self.emit_expr_pure(rhs);
                format!("{} {} {}", lhs_str, op, rhs_str)
            }
            RExpr::UnaryOp(op, operand) => {
                let operand_str = self.emit_expr_pure(operand);
                if op == "!" || op == "-" || op == "+" {
                    format!("{}{}", op, operand_str)
                } else {
                    format!("{}({})", op, operand_str)
                }
            }
            RExpr::IndexSingle(obj, indices) => {
                let obj_str = self.emit_expr_pure(obj);
                let idx_str: Vec<String> = indices.iter().map(|i| self.emit_expr_pure(i)).collect();
                format!("{}[{}]", obj_str, idx_str.join(", "))
            }
            RExpr::IndexDouble(obj, index) => {
                let obj_str = self.emit_expr_pure(obj);
                let idx_str = self.emit_expr_pure(index);
                format!("{}[[{}]]", obj_str, idx_str)
            }
            RExpr::DollarAccess(obj, field) => {
                let obj_str = self.emit_expr_pure(obj);
                format!("{}${}", obj_str, field)
            }
            RExpr::AtAccess(obj, slot) => {
                let obj_str = self.emit_expr_pure(obj);
                format!("{}@{}", obj_str, slot)
            }
            RExpr::Formula(lhs, rhs) => {
                let rhs_str = self.emit_expr_pure(rhs);
                if let Some(lhs_expr) = lhs {
                    let lhs_str = self.emit_expr_pure(lhs_expr);
                    format!("{} ~ {}", lhs_str, rhs_str)
                } else {
                    format!("~ {}", rhs_str)
                }
            }
            RExpr::IfElse(cond, then_expr, else_expr) => {
                let cond_str = self.emit_expr_pure(cond);
                let then_str = self.emit_expr_pure(then_expr);
                if let Some(else_e) = else_expr {
                    let else_str = self.emit_expr_pure(else_e);
                    format!("if ({}) {} else {}", cond_str, then_str, else_str)
                } else {
                    format!("if ({}) {}", cond_str, then_str)
                }
            }
            RExpr::Lambda(formals, body) => {
                let formals_str = self.emit_formals(formals);
                let body_str = self.emit_expr_pure(body);
                format!("function({}) {}", formals_str, body_str)
            }
            RExpr::Pipe(lhs, rhs) => {
                let lhs_str = self.emit_expr_pure(lhs);
                let rhs_str = self.emit_expr_pure(rhs);
                format!("{} |> {}", lhs_str, rhs_str)
            }
            RExpr::MagrittrPipe(lhs, rhs) => {
                let lhs_str = self.emit_expr_pure(lhs);
                let rhs_str = self.emit_expr_pure(rhs);
                format!("{} %>% {}", lhs_str, rhs_str)
            }
            RExpr::Seq(start, end) => {
                let start_str = self.emit_expr_pure(start);
                let end_str = self.emit_expr_pure(end);
                format!("{}:{}", start_str, end_str)
            }
            RExpr::CVec(elems) => {
                let elems_str: Vec<String> = elems.iter().map(|e| self.emit_expr_pure(e)).collect();
                format!("c({})", elems_str.join(", "))
            }
            RExpr::ListExpr(args) => {
                let args_str: Vec<String> = args.iter().map(|a| self.emit_arg(a)).collect();
                format!("list({})", args_str.join(", "))
            }
            RExpr::Block(stmts) => {
                if stmts.is_empty() {
                    return "{}".to_string();
                }
                let mut inner = RBackend::new();
                inner.indent = self.indent + 1;
                for s in stmts {
                    inner.emit_stmt(s);
                }
                let inner_out = inner.take_output();
                format!("{{\n{}{}}}", inner_out, self.current_indent())
            }
            RExpr::Namespace(pkg, func) => format!("{}::{}", pkg, func),
            RExpr::NamespaceInternal(pkg, func) => format!("{}:::{}", pkg, func),
        }
    }
    pub(crate) fn emit_literal(&self, lit: &RLiteral) -> String {
        match lit {
            RLiteral::Integer(n) => format!("{}L", n),
            RLiteral::Numeric(f) => {
                if f.fract() == 0.0 && f.abs() < 1e15 {
                    format!("{:.1}", f)
                } else {
                    format!("{}", f)
                }
            }
            RLiteral::Logical(b) => {
                if *b {
                    "TRUE".to_string()
                } else {
                    "FALSE".to_string()
                }
            }
            RLiteral::Character(s) => format!("\"{}\"", s.replace('"', "\\\"")),
            RLiteral::Complex(re, im) => {
                if *im >= 0.0 {
                    format!("{}+{}i", re, im)
                } else {
                    format!("{}{}i", re, im)
                }
            }
            RLiteral::Null => "NULL".to_string(),
            RLiteral::Na => "NA".to_string(),
            RLiteral::NaInteger => "NA_integer_".to_string(),
            RLiteral::NaReal => "NA_real_".to_string(),
            RLiteral::NaCharacter => "NA_character_".to_string(),
            RLiteral::NaComplex => "NA_complex_".to_string(),
            RLiteral::Inf => "Inf".to_string(),
            RLiteral::NaN => "NaN".to_string(),
        }
    }
    pub(crate) fn emit_arg(&self, arg: &RArg) -> String {
        let val_str = self.emit_expr_pure(&arg.value);
        if let Some(name) = &arg.name {
            format!("{} = {}", name, val_str)
        } else {
            val_str
        }
    }
    /// Emit a vectorized operation over a vector expression.
    pub fn emit_vectorized(
        &self,
        op: &VectorizedOp,
        vec_expr: &RExpr,
        extra_args: &[RArg],
    ) -> String {
        let vec_str = self.emit_expr_pure(vec_expr);
        if let Some(apply_fn) = &op.use_apply_family {
            let func_expr = RExpr::Var(op.op.clone());
            let func_str = self.emit_expr_pure(&func_expr);
            let mut args_str = format!("{}, {}", vec_str, func_str);
            for a in extra_args {
                args_str.push_str(", ");
                args_str.push_str(&self.emit_arg(a));
            }
            format!("{}({})", apply_fn, args_str)
        } else if op.needs_vectorize {
            let func_str = op.op.clone();
            let extra: Vec<String> = extra_args.iter().map(|a| self.emit_arg(a)).collect();
            if extra.is_empty() {
                format!("Vectorize({})({})", func_str, vec_str)
            } else {
                format!("Vectorize({})({}{})", func_str, vec_str, extra.join(", "))
            }
        } else {
            let mut args_str = vec_str;
            for a in extra_args {
                args_str.push_str(", ");
                args_str.push_str(&self.emit_arg(a));
            }
            format!("{}({})", op.op, args_str)
        }
    }
}

impl RLangLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        RLangLivenessInfo {
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

impl RLangConstantFoldingHelper {
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

impl RLangExtPassPhase {
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

impl RLangExtLiveness {
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

impl RLangExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: RLangExtPassConfig) {
        self.stats.push(RLangExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&RLangExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&RLangExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&RLangExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &RLangExtPassPhase) -> Vec<&RLangExtPassConfig> {
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

impl RLangAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        RLangAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&RLangCacheEntry> {
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
            RLangCacheEntry {
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

impl RLangExtCache {
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

impl RLangExtPassStats {
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
    pub fn merge(&mut self, o: &RLangExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}

impl RLangExtWorklist {
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

impl RLangExtConstFolder {
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

impl RLangPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            RLangPassPhase::Analysis => "analysis",
            RLangPassPhase::Transformation => "transformation",
            RLangPassPhase::Verification => "verification",
            RLangPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(
            self,
            RLangPassPhase::Transformation | RLangPassPhase::Cleanup
        )
    }
}

impl RLangExtDepGraph {
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
