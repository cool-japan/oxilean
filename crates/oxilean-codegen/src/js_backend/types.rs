//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::functions::{JS_KEYWORDS, JS_RUNTIME};

use std::collections::{HashSet, VecDeque};

/// Simple peephole optimizations on `JsExpr`.
#[allow(dead_code)]
pub struct JsPeephole;
impl JsPeephole {
    /// Fold constant arithmetic on numeric literals.
    #[allow(dead_code)]
    pub fn fold_arith(expr: &JsExpr) -> JsExpr {
        if let JsExpr::BinOp(op, lhs, rhs) = expr {
            if let (JsExpr::Lit(JsLit::Num(a)), JsExpr::Lit(JsLit::Num(b))) =
                (lhs.as_ref(), rhs.as_ref())
            {
                let folded = match op.as_str() {
                    "+" => Some(a + b),
                    "-" => Some(a - b),
                    "*" => Some(a * b),
                    "/" if *b != 0.0 => Some(a / b),
                    _ => None,
                };
                if let Some(v) = folded {
                    return JsExpr::Lit(JsLit::Num(v));
                }
            }
        }
        expr.clone()
    }
    /// Simplify `x === x` → `true`.
    #[allow(dead_code)]
    pub fn simplify_identity(expr: &JsExpr) -> JsExpr {
        if let JsExpr::BinOp(op, lhs, rhs) = expr {
            if op == "===" && lhs == rhs {
                return JsExpr::Lit(JsLit::Bool(true));
            }
            if op == "!==" && lhs == rhs {
                return JsExpr::Lit(JsLit::Bool(false));
            }
        }
        expr.clone()
    }
    /// Simplify `!true` → `false`, `!false` → `true`.
    #[allow(dead_code)]
    pub fn simplify_not(expr: &JsExpr) -> JsExpr {
        if let JsExpr::UnOp(op, inner) = expr {
            if op == "!" {
                if let JsExpr::Lit(JsLit::Bool(b)) = inner.as_ref() {
                    return JsExpr::Lit(JsLit::Bool(!b));
                }
            }
        }
        expr.clone()
    }
    /// Apply all peephole optimizations.
    #[allow(dead_code)]
    pub fn optimize(expr: &JsExpr) -> JsExpr {
        let e = Self::fold_arith(expr);
        let e = Self::simplify_identity(&e);
        Self::simplify_not(&e)
    }
}
/// Estimates the uncompressed byte size of generated JS code.
#[allow(dead_code)]
pub struct JsSizeEstimator;
impl JsSizeEstimator {
    /// Estimate the byte size of a JS function.
    #[allow(dead_code)]
    pub fn estimate_function(func: &JsFunction) -> usize {
        func.to_string().len()
    }
    /// Estimate the byte size of a JS module.
    #[allow(dead_code)]
    pub fn estimate_module(module: &JsModule) -> usize {
        module.emit().len()
    }
    /// Estimate the byte size of a JS expression.
    #[allow(dead_code)]
    pub fn estimate_expr(expr: &JsExpr) -> usize {
        expr.to_string().len()
    }
    /// Estimate the byte size of a JS statement.
    #[allow(dead_code)]
    pub fn estimate_stmt(stmt: &JsStmt) -> usize {
        stmt.to_string().len()
    }
}
#[allow(dead_code)]
pub struct JSPassRegistry {
    pub(super) configs: Vec<JSPassConfig>,
    pub(super) stats: std::collections::HashMap<String, JSPassStats>,
}
impl JSPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JSPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: JSPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), JSPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&JSPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&JSPassStats> {
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
/// A very simple JavaScript minifier that removes comments and collapses
/// unnecessary whitespace.  Not a full minifier, but reduces output size.
#[allow(dead_code)]
pub struct JsMinifier;
impl JsMinifier {
    /// Minify a JS source string.
    ///
    /// Operations performed:
    /// - Remove `// ...` line comments
    /// - Collapse runs of whitespace to a single space
    /// - Remove leading/trailing whitespace per line
    #[allow(dead_code)]
    pub fn minify(source: &str) -> std::string::String {
        let mut out = std::string::String::new();
        for line in source.lines() {
            let line = if let Some(pos) = line.find("//") {
                &line[..pos]
            } else {
                line
            };
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let collapsed: std::string::String =
                    trimmed.split_whitespace().collect::<Vec<_>>().join(" ");
                out.push_str(&collapsed);
                out.push('\n');
            }
        }
        out
    }
    /// Strip block comments `/* ... */` from a JS string.
    #[allow(dead_code)]
    pub fn strip_block_comments(source: &str) -> std::string::String {
        let mut out = std::string::String::new();
        let mut in_comment = false;
        let bytes = source.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if !in_comment && i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'*' {
                in_comment = true;
                i += 2;
            } else if in_comment && i + 1 < bytes.len() && bytes[i] == b'*' && bytes[i + 1] == b'/'
            {
                in_comment = false;
                i += 2;
            } else if !in_comment {
                out.push(bytes[i] as char);
                i += 1;
            } else {
                i += 1;
            }
        }
        out
    }
}
/// A structural type checker for JS expressions.
///
/// Infers the most specific `JsType` for an expression based on its structure.
#[allow(dead_code)]
pub struct JsTypeChecker;
impl JsTypeChecker {
    /// Infer the JS type of a literal.
    #[allow(dead_code)]
    pub fn infer_lit(lit: &JsLit) -> JsType {
        match lit {
            JsLit::Num(_) => JsType::Number,
            JsLit::BigInt(_) => JsType::BigInt,
            JsLit::Str(_) => JsType::String,
            JsLit::Bool(_) => JsType::Boolean,
            JsLit::Null => JsType::Null,
            JsLit::Undefined => JsType::Undefined,
        }
    }
    /// Infer the JS type of an expression (best-effort).
    #[allow(dead_code)]
    pub fn infer_expr(expr: &JsExpr) -> JsType {
        match expr {
            JsExpr::Lit(lit) => Self::infer_lit(lit),
            JsExpr::BinOp(op, lhs, _) => match op.as_str() {
                "===" | "!==" | "==" | "!=" | "<" | ">" | "<=" | ">=" => JsType::Boolean,
                "+" => {
                    let lhs_ty = Self::infer_expr(lhs);
                    if lhs_ty == JsType::String {
                        JsType::String
                    } else if lhs_ty == JsType::BigInt {
                        JsType::BigInt
                    } else {
                        JsType::Number
                    }
                }
                "-" | "*" | "/" | "%" => {
                    if Self::infer_expr(lhs) == JsType::BigInt {
                        JsType::BigInt
                    } else {
                        JsType::Number
                    }
                }
                _ => JsType::Unknown,
            },
            JsExpr::UnOp(op, _) => match op.as_str() {
                "!" => JsType::Boolean,
                "-" | "+" => JsType::Number,
                "typeof" => JsType::String,
                _ => JsType::Unknown,
            },
            JsExpr::Object(_) => JsType::Object,
            JsExpr::Array(_) => JsType::Array,
            JsExpr::Arrow(_, _) => JsType::Function,
            JsExpr::New(_, _) => JsType::Object,
            _ => JsType::Unknown,
        }
    }
}
/// A JavaScript ES2020 module, containing functions and a preamble.
#[derive(Debug, Clone)]
pub struct JsModule {
    /// Top-level function declarations.
    pub functions: Vec<JsFunction>,
    /// Top-level `const`/`let` lines (preamble, runtime, etc.).
    pub preamble: Vec<std::string::String>,
    /// Names to include in the default export object.
    pub exports: Vec<std::string::String>,
}
impl JsModule {
    /// Create a new empty JS module.
    pub fn new() -> Self {
        JsModule {
            functions: Vec::new(),
            preamble: Vec::new(),
            exports: Vec::new(),
        }
    }
    /// Add a function to the module.
    pub fn add_function(&mut self, f: JsFunction) {
        self.functions.push(f);
    }
    /// Add a top-level preamble line (raw JS text).
    pub fn add_preamble(&mut self, line: std::string::String) {
        self.preamble.push(line);
    }
    /// Add a name to the export list.
    pub fn add_export(&mut self, name: std::string::String) {
        self.exports.push(name);
    }
    /// Emit the full JS module as a string.
    ///
    /// The output includes:
    /// 1. The OxiLean JS runtime preamble (`_OL`)
    /// 2. Any additional preamble lines
    /// 3. All function declarations
    /// 4. A named export object if `exports` is non-empty
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str(JS_RUNTIME);
        out.push('\n');
        for line in &self.preamble {
            out.push_str(line);
            out.push('\n');
        }
        if !self.preamble.is_empty() {
            out.push('\n');
        }
        for func in &self.functions {
            out.push_str(&func.to_string());
            out.push_str("\n\n");
        }
        if !self.exports.is_empty() {
            out.push_str("export { ");
            for (i, name) in self.exports.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                out.push_str(name);
            }
            out.push_str(" };\n");
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JSDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl JSDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        JSDominatorTree {
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
/// JavaScript literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum JsLit {
    /// Floating-point number literal: `1.0`, `42`, `-3.14`
    Num(f64),
    /// BigInt literal for Nat: `0n`, `42n`
    BigInt(i64),
    /// String literal: `"hello"`
    Str(std::string::String),
    /// Boolean literal: `true` or `false`
    Bool(bool),
    /// `null` literal
    Null,
    /// `undefined` literal
    Undefined,
}
/// A top-level JavaScript function declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct JsFunction {
    /// The name of the function.
    pub name: std::string::String,
    /// The parameter names.
    pub params: Vec<std::string::String>,
    /// The function body statements.
    pub body: Vec<JsStmt>,
    /// Whether this function is declared with `async`.
    pub is_async: bool,
    /// Whether this function is exported (for ES module export).
    pub is_export: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JSPassConfig {
    pub phase: JSPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl JSPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: JSPassPhase) -> Self {
        JSPassConfig {
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
/// JavaScript expression for code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum JsExpr {
    /// A literal value: `42n`, `"hello"`, `true`, etc.
    Lit(JsLit),
    /// A variable identifier: `x`, `_t0`, `Nat_add`
    Var(std::string::String),
    /// A function call: `f(a, b, c)`
    Call(Box<JsExpr>, Vec<JsExpr>),
    /// A method call: `obj.method(a, b)`
    Method(Box<JsExpr>, std::string::String, Vec<JsExpr>),
    /// Property access: `obj.field`
    Field(Box<JsExpr>, std::string::String),
    /// Array index: `arr[i]`
    Index(Box<JsExpr>, Box<JsExpr>),
    /// Arrow function: `(x, y) => { stmts }`
    Arrow(Vec<std::string::String>, Box<JsStmt>),
    /// Ternary expression: `cond ? then_expr : else_expr`
    Ternary(Box<JsExpr>, Box<JsExpr>, Box<JsExpr>),
    /// Binary operator: `lhs + rhs`, `a === b`, etc.
    BinOp(std::string::String, Box<JsExpr>, Box<JsExpr>),
    /// Unary operator: `!x`, `-n`, `typeof x`
    UnOp(std::string::String, Box<JsExpr>),
    /// Await expression: `await promise`
    Await(Box<JsExpr>),
    /// Constructor call: `new MyClass(a, b)`
    New(std::string::String, Vec<JsExpr>),
    /// Spread expression: `...arr`
    Spread(Box<JsExpr>),
    /// Object literal: `{ key: val, ... }`
    Object(Vec<(std::string::String, JsExpr)>),
    /// Array literal: `[a, b, c]`
    Array(Vec<JsExpr>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JSAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, JSCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl JSAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        JSAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&JSCacheEntry> {
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
            JSCacheEntry {
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
/// A simple source map associating generated positions with source functions.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JsSourceMap {
    pub(super) entries: Vec<SourceMapEntry>,
}
impl JsSourceMap {
    /// Create a new empty source map.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add an entry.
    #[allow(dead_code)]
    pub fn add(&mut self, entry: SourceMapEntry) {
        self.entries.push(entry);
    }
    /// Number of entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the source map is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Find entries for a given generated line.
    #[allow(dead_code)]
    pub fn entries_for_line(&self, gen_line: u32) -> Vec<&SourceMapEntry> {
        self.entries
            .iter()
            .filter(|e| e.gen_line == gen_line)
            .collect()
    }
}
/// Pretty-prints a `JsModule` with configurable indentation and line width.
#[allow(dead_code)]
pub struct JsPrettyPrinter {
    /// Indentation width (spaces per level).
    pub indent_width: usize,
    /// Target line width (for soft wrapping heuristics).
    pub line_width: usize,
    /// Whether to emit trailing commas in objects/arrays.
    pub trailing_commas: bool,
}
impl JsPrettyPrinter {
    /// Create a new pretty-printer with standard settings.
    #[allow(dead_code)]
    pub fn new() -> Self {
        JsPrettyPrinter {
            indent_width: 2,
            line_width: 80,
            trailing_commas: false,
        }
    }
    /// Pretty-print a `JsModule`.
    #[allow(dead_code)]
    pub fn print_module(&self, module: &JsModule) -> std::string::String {
        module.emit()
    }
    /// Pretty-print a single `JsFunction`.
    #[allow(dead_code)]
    pub fn print_function(&self, func: &JsFunction) -> std::string::String {
        func.to_string()
    }
    /// Pretty-print a `JsExpr` at the given indentation depth.
    #[allow(dead_code)]
    pub fn print_expr(&self, expr: &JsExpr, _depth: usize) -> std::string::String {
        expr.to_string()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JSCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JSWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl JSWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JSWorklist {
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
/// JavaScript code generation backend.
///
/// Compiles LCNF function declarations to a `JsModule` containing
/// ES2020+ JavaScript functions.
pub struct JsBackend {
    /// The module being built.
    pub module: JsModule,
    /// Mapping from LCNF names to mangled JS names.
    pub fn_map: HashMap<std::string::String, std::string::String>,
    /// Counter for generating fresh temporary variable names.
    pub fresh_counter: usize,
}
impl JsBackend {
    /// Create a new JS backend.
    pub fn new() -> Self {
        JsBackend {
            module: JsModule::new(),
            fn_map: HashMap::new(),
            fresh_counter: 0,
        }
    }
    /// Generate a fresh temporary variable name: `_t0`, `_t1`, etc.
    pub fn fresh_var(&mut self) -> std::string::String {
        let n = self.fresh_counter;
        self.fresh_counter += 1;
        format!("_t{}", n)
    }
    /// Mangle an LCNF name into a valid JavaScript identifier.
    ///
    /// Rules:
    /// - Replace `.` with `_`
    /// - Replace `'` (prime) with `_prime`
    /// - Prefix reserved words with `_`
    pub fn mangle_name(&self, name: &str) -> std::string::String {
        let mangled: std::string::String = name
            .chars()
            .map(|c| match c {
                '.' => '_',
                '\'' => '_',
                '-' => '_',
                c if c.is_alphanumeric() || c == '_' => c,
                _ => '_',
            })
            .collect();
        if JS_KEYWORDS.contains(&mangled.as_str())
            || mangled.starts_with(|c: char| c.is_ascii_digit())
        {
            format!("_{}", mangled)
        } else if mangled.is_empty() {
            "_anon".to_string()
        } else {
            mangled
        }
    }
    /// Top-level entry point: compile a slice of LCNF function declarations
    /// into a JavaScript module string.
    pub fn compile_module(
        decls: &[LcnfFunDecl],
    ) -> Result<std::string::String, std::string::String> {
        let mut backend = JsBackend::new();
        for decl in decls {
            let js_name = backend.mangle_name(&decl.name);
            backend.fn_map.insert(decl.name.clone(), js_name);
        }
        for decl in decls {
            let func = backend.compile_decl(decl)?;
            backend.module.add_function(func);
        }
        for decl in decls {
            if let Some(js_name) = backend.fn_map.get(&decl.name) {
                backend.module.add_export(js_name.clone());
            }
        }
        Ok(backend.module.emit())
    }
    /// Compile a single LCNF function declaration into a `JsFunction`.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) -> Result<JsFunction, std::string::String> {
        let js_name = self.mangle_name(&decl.name);
        self.fn_map.insert(decl.name.clone(), js_name.clone());
        let params: Vec<std::string::String> = decl
            .params
            .iter()
            .map(|p| {
                if p.erased {
                    format!("_{}", self.mangle_name(&p.name))
                } else {
                    self.mangle_name(&p.name)
                }
            })
            .collect();
        let mut body_stmts: Vec<JsStmt> = Vec::new();
        let result = self.compile_expr(&decl.body, &mut body_stmts)?;
        let last_is_return = body_stmts.last().is_some_and(|s| {
            matches!(s, JsStmt::Return(_) | JsStmt::ReturnVoid | JsStmt::Throw(_))
        });
        if !last_is_return {
            body_stmts.push(JsStmt::Return(result));
        }
        Ok(JsFunction {
            name: js_name,
            params,
            body: body_stmts,
            is_async: false,
            is_export: false,
        })
    }
    /// Compile an LCNF expression into a sequence of JS statements,
    /// returning the JS expression that holds the result.
    pub fn compile_expr(
        &mut self,
        expr: &LcnfExpr,
        stmts: &mut Vec<JsStmt>,
    ) -> Result<JsExpr, std::string::String> {
        match expr {
            LcnfExpr::Let {
                id: _,
                name,
                ty: _,
                value,
                body,
            } => {
                let js_val = self.compile_let_value(value, stmts)?;
                let js_name = self.mangle_name(name);
                stmts.push(JsStmt::Const(js_name.clone(), js_val));
                self.compile_expr(body, stmts)
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => {
                let scrutinee_name = format!("_x{}", scrutinee.0);
                let tag_expr = JsExpr::Field(
                    Box::new(JsExpr::Var(scrutinee_name.clone())),
                    "tag".to_string(),
                );
                if alts.len() == 1 && alts[0].ctor_name.is_empty() {
                    let alt = &alts[0];
                    for (i, param) in alt.params.iter().enumerate() {
                        let field_expr = JsExpr::Call(
                            Box::new(JsExpr::Field(
                                Box::new(JsExpr::Var("_OL".to_string())),
                                "proj".to_string(),
                            )),
                            vec![
                                JsExpr::Var(scrutinee_name.clone()),
                                JsExpr::Lit(JsLit::Num(i as f64)),
                            ],
                        );
                        let pname = self.mangle_name(&param.name);
                        stmts.push(JsStmt::Const(pname, field_expr));
                    }
                    return self.compile_expr(&alt.body, stmts);
                }
                let result_var = self.fresh_var();
                stmts.push(JsStmt::Let(
                    result_var.clone(),
                    JsExpr::Lit(JsLit::Undefined),
                ));
                let mut cases: Vec<(JsExpr, Vec<JsStmt>)> = Vec::new();
                for alt in alts {
                    let mut case_stmts: Vec<JsStmt> = Vec::new();
                    for (i, param) in alt.params.iter().enumerate() {
                        let field_expr = JsExpr::Call(
                            Box::new(JsExpr::Field(
                                Box::new(JsExpr::Var("_OL".to_string())),
                                "proj".to_string(),
                            )),
                            vec![
                                JsExpr::Var(scrutinee_name.clone()),
                                JsExpr::Lit(JsLit::Num(i as f64)),
                            ],
                        );
                        let pname = self.mangle_name(&param.name);
                        case_stmts.push(JsStmt::Const(pname, field_expr));
                    }
                    let branch_result = self.compile_expr(&alt.body, &mut case_stmts)?;
                    case_stmts.push(JsStmt::Expr(JsExpr::BinOp(
                        "=".to_string(),
                        Box::new(JsExpr::Var(result_var.clone())),
                        Box::new(branch_result),
                    )));
                    let tag_lit = JsExpr::Lit(JsLit::Str(alt.ctor_name.clone()));
                    cases.push((tag_lit, case_stmts));
                }
                let default_stmts = if let Some(def) = default {
                    let mut def_stmts: Vec<JsStmt> = Vec::new();
                    let def_result = self.compile_expr(def, &mut def_stmts)?;
                    def_stmts.push(JsStmt::Expr(JsExpr::BinOp(
                        "=".to_string(),
                        Box::new(JsExpr::Var(result_var.clone())),
                        Box::new(def_result),
                    )));
                    def_stmts
                } else {
                    vec![JsStmt::Throw(JsExpr::New(
                        "Error".to_string(),
                        vec![JsExpr::Lit(JsLit::Str("Unreachable case".to_string()))],
                    ))]
                };
                stmts.push(JsStmt::Switch(tag_expr, cases, default_stmts));
                Ok(JsExpr::Var(result_var))
            }
            LcnfExpr::Return(arg) => Ok(self.compile_arg(arg)),
            LcnfExpr::Unreachable => {
                stmts.push(JsStmt::Throw(JsExpr::New(
                    "Error".to_string(),
                    vec![JsExpr::Lit(JsLit::Str("Unreachable".to_string()))],
                )));
                Ok(JsExpr::Lit(JsLit::Undefined))
            }
            LcnfExpr::TailCall(func, args) => {
                let js_func = self.compile_arg(func);
                let js_args: Vec<JsExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(JsExpr::Call(Box::new(js_func), js_args))
            }
        }
    }
    /// Compile an LCNF let-value into a JS expression.
    pub(super) fn compile_let_value(
        &mut self,
        value: &LcnfLetValue,
        stmts: &mut Vec<JsStmt>,
    ) -> Result<JsExpr, std::string::String> {
        match value {
            LcnfLetValue::Lit(lit) => Ok(self.compile_lit(lit)),
            LcnfLetValue::App(func, args) => {
                let js_func = self.compile_arg(func);
                let js_args: Vec<JsExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(JsExpr::Call(Box::new(js_func), js_args))
            }
            LcnfLetValue::Ctor(name, tag, args) => {
                let mut ctor_args = vec![JsExpr::Lit(JsLit::Str(name.clone()))];
                for a in args {
                    ctor_args.push(self.compile_arg(a));
                }
                let _ = tag;
                Ok(JsExpr::Call(
                    Box::new(JsExpr::Field(
                        Box::new(JsExpr::Var("_OL".to_string())),
                        "ctor".to_string(),
                    )),
                    ctor_args,
                ))
            }
            LcnfLetValue::Proj(_, idx, var) => {
                let var_expr = JsExpr::Var(format!("_x{}", var.0));
                Ok(JsExpr::Call(
                    Box::new(JsExpr::Field(
                        Box::new(JsExpr::Var("_OL".to_string())),
                        "proj".to_string(),
                    )),
                    vec![var_expr, JsExpr::Lit(JsLit::Num(*idx as f64))],
                ))
            }
            LcnfLetValue::Erased => Ok(JsExpr::Lit(JsLit::Undefined)),
            LcnfLetValue::FVar(id) => Ok(JsExpr::Var(format!("_x{}", id.0))),
            LcnfLetValue::Reset(_var) => Ok(JsExpr::Lit(JsLit::Null)),
            LcnfLetValue::Reuse(_slot, name, _tag, args) => {
                let mut ctor_args = vec![JsExpr::Lit(JsLit::Str(name.clone()))];
                for a in args {
                    ctor_args.push(self.compile_arg(a));
                }
                let _ = stmts;
                Ok(JsExpr::Call(
                    Box::new(JsExpr::Field(
                        Box::new(JsExpr::Var("_OL".to_string())),
                        "ctor".to_string(),
                    )),
                    ctor_args,
                ))
            }
        }
    }
    /// Compile an LCNF argument (atomic value) into a JS expression.
    pub fn compile_arg(&self, arg: &LcnfArg) -> JsExpr {
        match arg {
            LcnfArg::Var(id) => JsExpr::Var(format!("_x{}", id.0)),
            LcnfArg::Lit(lit) => self.compile_lit(lit),
            LcnfArg::Erased => JsExpr::Lit(JsLit::Undefined),
            LcnfArg::Type(_) => JsExpr::Lit(JsLit::Undefined),
        }
    }
    /// Compile an LCNF literal into a JS expression.
    pub fn compile_lit(&self, lit: &LcnfLit) -> JsExpr {
        match lit {
            LcnfLit::Nat(n) => JsExpr::Lit(JsLit::BigInt(*n as i64)),
            LcnfLit::Str(s) => JsExpr::Lit(JsLit::Str(s.clone())),
        }
    }
}
#[allow(dead_code)]
pub struct JSConstantFoldingHelper;
impl JSConstantFoldingHelper {
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
/// Extended name mangler with support for module namespaces.
#[allow(dead_code)]
pub struct JsNameMangler {
    /// Namespace prefix (e.g., "Lean" for Lean library functions).
    pub namespace: std::string::String,
    /// Whether to preserve original capitalization.
    pub preserve_case: bool,
}
impl JsNameMangler {
    /// Create a new name mangler.
    #[allow(dead_code)]
    pub fn new(namespace: &str) -> Self {
        JsNameMangler {
            namespace: namespace.to_string(),
            preserve_case: true,
        }
    }
    /// Mangle a name with optional namespace prefix.
    #[allow(dead_code)]
    pub fn mangle(&self, name: &str) -> std::string::String {
        let backend = JsBackend::new();
        let mangled = backend.mangle_name(name);
        if self.namespace.is_empty() {
            mangled
        } else {
            format!("{}_{}", self.namespace, mangled)
        }
    }
    /// Mangle a qualified name like `Nat.add` → `Lean_Nat_add`.
    #[allow(dead_code)]
    pub fn mangle_qualified(&self, parts: &[&str]) -> std::string::String {
        let joined = parts.join(".");
        self.mangle(&joined)
    }
}
/// A single source-map entry mapping an output position to a source position.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SourceMapEntry {
    /// Generated (output) line number (0-based).
    pub gen_line: u32,
    /// Generated (output) column number (0-based).
    pub gen_col: u32,
    /// Original function name (if known).
    pub source_fn: std::string::String,
    /// Original source line (for display; may be 0 if unavailable).
    pub source_line: u32,
}
impl SourceMapEntry {
    /// Create a new source map entry.
    #[allow(dead_code)]
    pub fn new(gen_line: u32, gen_col: u32, source_fn: &str, source_line: u32) -> Self {
        SourceMapEntry {
            gen_line,
            gen_col,
            source_fn: source_fn.to_string(),
            source_line,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum JSPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl JSPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            JSPassPhase::Analysis => "analysis",
            JSPassPhase::Transformation => "transformation",
            JSPassPhase::Verification => "verification",
            JSPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, JSPassPhase::Transformation | JSPassPhase::Cleanup)
    }
}
/// A table of all identifiers used in a JS module, for rename-on-collision.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JsIdentTable {
    /// All identifiers currently in scope.
    pub(super) idents: std::collections::HashSet<std::string::String>,
    /// Collision counter per base name.
    pub(super) collisions: std::collections::HashMap<std::string::String, usize>,
}
impl JsIdentTable {
    /// Create a new empty ident table.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Register an identifier, returning a collision-free version.
    #[allow(dead_code)]
    pub fn register(&mut self, name: &str) -> std::string::String {
        if self.idents.contains(name) {
            let count = self.collisions.entry(name.to_string()).or_insert(0);
            *count += 1;
            let renamed = format!("{}_{}", name, count);
            self.idents.insert(renamed.clone());
            renamed
        } else {
            self.idents.insert(name.to_string());
            name.to_string()
        }
    }
    /// Whether a name is already taken.
    #[allow(dead_code)]
    pub fn is_taken(&self, name: &str) -> bool {
        self.idents.contains(name)
    }
    /// Number of registered identifiers.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.idents.len()
    }
    /// Whether the table is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.idents.is_empty()
    }
}
/// JavaScript statement for code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum JsStmt {
    /// Expression statement: `expr;`
    Expr(JsExpr),
    /// Let binding: `let x = expr;`
    Let(std::string::String, JsExpr),
    /// Const binding: `const x = expr;`
    Const(std::string::String, JsExpr),
    /// Return statement: `return expr;`
    Return(JsExpr),
    /// Void return: `return;`
    ReturnVoid,
    /// If-else: `if (cond) { then } else { else_ }`
    If(JsExpr, Vec<JsStmt>, Vec<JsStmt>),
    /// While loop: `while (cond) { body }`
    While(JsExpr, Vec<JsStmt>),
    /// For-of loop: `for (const x of iter) { body }`
    For(std::string::String, JsExpr, Vec<JsStmt>),
    /// Block: `{ stmts }`
    Block(Vec<JsStmt>),
    /// Throw statement: `throw expr;`
    Throw(JsExpr),
    /// Try-catch: `try { body } catch (e) { handler }`
    TryCatch(Vec<JsStmt>, std::string::String, Vec<JsStmt>),
    /// Switch statement: `switch (expr) { case ...: ... default: ... }`
    Switch(JsExpr, Vec<(JsExpr, Vec<JsStmt>)>, Vec<JsStmt>),
}
/// Links multiple `JsModule` objects into a single output.
#[allow(dead_code)]
pub struct JsModuleLinker {
    pub(super) modules: Vec<JsModule>,
}
impl JsModuleLinker {
    /// Create a new linker.
    #[allow(dead_code)]
    pub fn new() -> Self {
        JsModuleLinker {
            modules: Vec::new(),
        }
    }
    /// Add a module to link.
    #[allow(dead_code)]
    pub fn add_module(&mut self, module: JsModule) {
        self.modules.push(module);
    }
    /// Link all added modules into a single `JsModule`.
    ///
    /// Functions from all modules are merged; preamble lines are deduplicated;
    /// exports from all modules are combined.
    #[allow(dead_code)]
    pub fn link(&self) -> JsModule {
        let mut combined = JsModule::new();
        let mut seen_preamble: std::collections::HashSet<std::string::String> =
            std::collections::HashSet::new();
        for module in &self.modules {
            for line in &module.preamble {
                if seen_preamble.insert(line.clone()) {
                    combined.preamble.push(line.clone());
                }
            }
            for func in &module.functions {
                combined.functions.push(func.clone());
            }
            for name in &module.exports {
                if !combined.exports.contains(name) {
                    combined.exports.push(name.clone());
                }
            }
        }
        combined
    }
    /// Number of modules.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.modules.len()
    }
    /// Whether no modules have been added.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }
}
/// Configuration for the JS backend.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JsBackendConfig {
    /// Whether to use BigInt for Nat (default: true).
    pub use_bigint_for_nat: bool,
    /// Whether to emit strict mode (`"use strict";`).
    pub strict_mode: bool,
    /// Whether to include the runtime preamble.
    pub include_runtime: bool,
    /// Whether to emit JSDoc comments for functions.
    pub emit_jsdoc: bool,
    /// Module format: `es` (ES modules) or `cjs` (CommonJS).
    pub module_format: JsModuleFormat,
    /// Whether to minify the output.
    pub minify: bool,
}
/// JavaScript module output format.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsModuleFormat {
    /// ES2020 modules (`export`/`import`).
    Es,
    /// CommonJS (`module.exports`/`require`).
    Cjs,
    /// No module wrapper (IIFE or bare script).
    None,
}
/// JavaScript type representation for type-directed code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsType {
    /// `undefined`
    Undefined,
    /// `null`
    Null,
    /// `boolean`
    Boolean,
    /// `number` (64-bit float)
    Number,
    /// `bigint` (arbitrary-precision integer, used for Nat)
    BigInt,
    /// `string`
    String,
    /// `object` (heap-allocated record or constructor)
    Object,
    /// `Array` (JS Array)
    Array,
    /// `function`
    Function,
    /// Unknown or polymorphic type
    Unknown,
}
/// Context maintained during JS code emission.
#[allow(dead_code)]
pub struct JsEmitContext {
    /// Current indentation level.
    pub indent_level: usize,
    /// Indentation string per level.
    pub indent_str: std::string::String,
    /// Source map being built.
    pub source_map: JsSourceMap,
    /// Current output line number.
    pub current_line: u32,
    /// Current output column.
    pub current_col: u32,
    /// Whether we are inside an async function.
    pub in_async: bool,
}
impl JsEmitContext {
    /// Create a new emit context.
    #[allow(dead_code)]
    pub fn new(indent: &str) -> Self {
        JsEmitContext {
            indent_level: 0,
            indent_str: indent.to_string(),
            source_map: JsSourceMap::new(),
            current_line: 0,
            current_col: 0,
            in_async: false,
        }
    }
    /// Get the current indentation string.
    #[allow(dead_code)]
    pub fn indent(&self) -> std::string::String {
        self.indent_str.repeat(self.indent_level)
    }
    /// Increase indentation.
    #[allow(dead_code)]
    pub fn push_indent(&mut self) {
        self.indent_level += 1;
    }
    /// Decrease indentation.
    #[allow(dead_code)]
    pub fn pop_indent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
    /// Emit a newline, updating line/column counters.
    #[allow(dead_code)]
    pub fn newline(&mut self) {
        self.current_line += 1;
        self.current_col = 0;
    }
    /// Record a source map entry for the current position.
    #[allow(dead_code)]
    pub fn record_mapping(&mut self, fn_name: &str, source_line: u32) {
        self.source_map.add(SourceMapEntry::new(
            self.current_line,
            self.current_col,
            fn_name,
            source_line,
        ));
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JSPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl JSPassStats {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JSLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl JSLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        JSLivenessInfo {
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
pub struct JSDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl JSDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JSDepGraph {
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
