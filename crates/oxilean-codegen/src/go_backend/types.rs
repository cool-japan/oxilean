//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::functions::*;
use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GoDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl GoDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        GoDominatorTree {
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
/// A named Go function.
#[derive(Debug, Clone)]
pub struct GoFunc {
    /// Function name.
    pub name: String,
    /// Receiver (for methods): `(recv_name recv_type)`.
    pub receiver: Option<(String, GoType)>,
    /// Parameter list: `[(name, type), ...]`.
    pub params: Vec<(String, GoType)>,
    /// Return type list.
    pub return_types: Vec<GoType>,
    /// Function body statements.
    pub body: Vec<GoStmt>,
    /// Whether exported (capitalised).
    pub exported: bool,
}
impl GoFunc {
    /// Create a new unexported function with no receiver.
    pub fn new(name: impl Into<String>) -> Self {
        GoFunc {
            name: name.into(),
            receiver: None,
            params: Vec::new(),
            return_types: Vec::new(),
            body: Vec::new(),
            exported: false,
        }
    }
    /// Add a parameter.
    pub fn add_param(&mut self, name: impl Into<String>, ty: GoType) {
        self.params.push((name.into(), ty));
    }
    /// Add a return type.
    pub fn add_return(&mut self, ty: GoType) {
        self.return_types.push(ty);
    }
    /// Emit Go source for this function.
    pub fn codegen(&self) -> String {
        let mut out = String::new();
        let recv_str = self
            .receiver
            .as_ref()
            .map(|(n, t)| format!("({} {}) ", n, t))
            .unwrap_or_default();
        let params_str = self
            .params
            .iter()
            .map(|(n, t)| format!("{} {}", n, t))
            .collect::<Vec<_>>()
            .join(", ");
        let ret_str = match self.return_types.len() {
            0 => String::new(),
            1 => format!(" {}", self.return_types[0]),
            _ => {
                let rs: Vec<String> = self.return_types.iter().map(|t| t.to_string()).collect();
                format!(" ({})", rs.join(", "))
            }
        };
        out.push_str(&format!(
            "func {}{}({}){} {{\n",
            recv_str, self.name, params_str, ret_str
        ));
        let body_str = format_stmts(&self.body, 1);
        if !body_str.is_empty() {
            out.push_str(&body_str);
            out.push('\n');
        }
        out.push('}');
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GoDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl GoDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GoDepGraph {
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
/// A complete Go source file / module.
#[derive(Debug, Clone)]
pub struct GoModule {
    /// Package name.
    pub package: String,
    /// Import paths.
    pub imports: Vec<String>,
    /// Type declarations.
    pub types: Vec<GoTypeDecl>,
    /// Function declarations.
    pub funcs: Vec<GoFunc>,
    /// Package-level variable declarations.
    pub vars: Vec<GoStmt>,
    /// Constant declarations.
    pub consts: Vec<(String, GoType, GoExpr)>,
}
impl GoModule {
    /// Create a new module with the given package name.
    pub fn new(package: impl Into<String>) -> Self {
        GoModule {
            package: package.into(),
            imports: Vec::new(),
            types: Vec::new(),
            funcs: Vec::new(),
            vars: Vec::new(),
            consts: Vec::new(),
        }
    }
    /// Add an import path (e.g. `"fmt"`, `"math/big"`).
    pub fn add_import(&mut self, path: impl Into<String>) {
        let p = path.into();
        if !self.imports.contains(&p) {
            self.imports.push(p);
        }
    }
    /// Emit a complete Go source file as a string.
    pub fn codegen(&self) -> String {
        let mut out = format!("package {}\n\n", self.package);
        if !self.imports.is_empty() {
            if self.imports.len() == 1 {
                out.push_str(&format!("import \"{}\"\n\n", self.imports[0]));
            } else {
                out.push_str("import (\n");
                for imp in &self.imports {
                    out.push_str(&format!("    \"{}\"\n", imp));
                }
                out.push_str(")\n\n");
            }
        }
        if !self.consts.is_empty() {
            out.push_str("const (\n");
            for (name, ty, val) in &self.consts {
                out.push_str(&format!("    {} {} = {}\n", name, ty, val));
            }
            out.push_str(")\n\n");
        }
        for ty_decl in &self.types {
            out.push_str(&ty_decl.codegen());
            out.push_str("\n\n");
        }
        if !self.vars.is_empty() {
            out.push_str("var (\n");
            for v in &self.vars {
                out.push_str(&format!("    {}\n", v));
            }
            out.push_str(")\n\n");
        }
        for func in &self.funcs {
            out.push_str(&func.codegen());
            out.push_str("\n\n");
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GoWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl GoWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GoWorklist {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GoAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, GoCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl GoAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        GoAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&GoCacheEntry> {
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
            GoCacheEntry {
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
#[allow(dead_code)]
pub struct GoConstantFoldingHelper;
impl GoConstantFoldingHelper {
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
/// Go expression for code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum GoExpr {
    /// A literal value: `42`, `"hello"`, `true`, `nil`
    Lit(GoLit),
    /// A variable identifier: `x`, `_t0`, `natAdd`
    Var(String),
    /// A function call: `f(a, b, c)`
    Call(Box<GoExpr>, Vec<GoExpr>),
    /// A binary operator expression: `a + b`, `a == b`
    BinOp(String, Box<GoExpr>, Box<GoExpr>),
    /// A unary operator expression: `!x`, `-n`
    Unary(String, Box<GoExpr>),
    /// Field access: `obj.Field`
    Field(Box<GoExpr>, String),
    /// Index expression: `arr[i]`
    Index(Box<GoExpr>, Box<GoExpr>),
    /// Type assertion: `x.(T)`
    TypeAssert(Box<GoExpr>, GoType),
    /// Composite literal: `MyStruct{Field: val, ...}`
    Composite(GoType, Vec<(String, GoExpr)>),
    /// Slice literal: `[]T{a, b, c}`
    SliceLit(GoType, Vec<GoExpr>),
    /// Address-of: `&expr`
    AddressOf(Box<GoExpr>),
    /// Dereference: `*expr`
    Deref(Box<GoExpr>),
    /// Anonymous function literal: `func(params) ret_ty { body }`
    FuncLit(Vec<(String, GoType)>, Vec<GoType>, Vec<GoStmt>),
    /// `make(T, args...)`
    Make(GoType, Vec<GoExpr>),
    /// `new(T)`
    New(GoType),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GoPassConfig {
    pub phase: GoPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl GoPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: GoPassPhase) -> Self {
        GoPassConfig {
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
/// The Go code generation backend.
///
/// Translates LCNF declarations and expressions into Go source code.
pub struct GoBackend {
    /// Counter for generating fresh temporary variable names.
    pub(super) var_counter: u64,
    /// Map from LCNF variable IDs to Go identifiers.
    pub(super) var_map: HashMap<LcnfVarId, String>,
    /// Cached keyword set.
    pub(super) keywords: HashSet<&'static str>,
    /// Cached built-in set.
    pub(super) builtins: HashSet<&'static str>,
}
impl GoBackend {
    /// Create a new `GoBackend`.
    pub fn new() -> Self {
        GoBackend {
            var_counter: 0,
            var_map: HashMap::new(),
            keywords: go_keywords(),
            builtins: go_builtins(),
        }
    }
    /// Generate a fresh temporary variable name.
    pub(super) fn fresh_var(&mut self) -> String {
        let n = self.var_counter;
        self.var_counter += 1;
        format!("_t{}", n)
    }
    /// Mangle a name so that it is a valid, non-reserved Go identifier.
    ///
    /// Rules:
    /// 1. Replace `.`, `-`, `/`, ` ` with `_`.
    /// 2. If empty → `ox_empty`.
    /// 3. If starts with a digit → prefix with `ox_`.
    /// 4. If a Go keyword or built-in → prefix with `ox_`.
    pub fn mangle_name(name: &str) -> String {
        let sanitised: String = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        if sanitised.is_empty() {
            return "ox_empty".to_string();
        }
        if sanitised.starts_with(|c: char| c.is_ascii_digit()) {
            return format!("ox_{}", sanitised);
        }
        let kw = go_keywords();
        let builtins = go_builtins();
        if kw.contains(sanitised.as_str()) || builtins.contains(sanitised.as_str()) {
            return format!("ox_{}", sanitised);
        }
        sanitised
    }
    /// Emit a Go module with the OxiLean runtime preamble and compiled declarations.
    pub fn compile_module(&mut self, decls: &[LcnfFunDecl]) -> GoModule {
        let mut module = GoModule::new("main");
        module.add_import("fmt");
        let runtime_funcs = self.build_runtime();
        for f in runtime_funcs {
            module.funcs.push(f);
        }
        let mut ctor_decl = GoTypeDecl::new("OxiCtor");
        ctor_decl.add_field("Tag", GoType::GoInt);
        ctor_decl.add_field("Fields", GoType::GoSlice(Box::new(GoType::GoInterface)));
        module.types.push(ctor_decl);
        for decl in decls {
            if let Some(func) = self.compile_decl(decl) {
                module.funcs.push(func);
            }
        }
        module
    }
    /// Compile a single LCNF declaration into a Go function.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) -> Option<GoFunc> {
        let go_name = Self::mangle_name(&decl.name.to_string());
        let mut func = GoFunc::new(go_name);
        for param in &decl.params {
            if !param.erased {
                let go_param_name = Self::mangle_name(&param.name);
                let go_ty = self.compile_type(&param.ty);
                func.add_param(go_param_name.clone(), go_ty.clone());
                self.var_map.insert(param.id, go_param_name);
            }
        }
        let go_ret = self.compile_type(&decl.ret_type);
        func.add_return(go_ret);
        let body_stmts = self.compile_expr(&decl.body);
        func.body = body_stmts;
        Some(func)
    }
    /// Compile an LCNF type to its Go equivalent.
    pub fn compile_type(&self, ty: &LcnfType) -> GoType {
        match ty {
            LcnfType::Nat => GoType::GoInt,
            LcnfType::LcnfString => GoType::GoString,
            LcnfType::Erased | LcnfType::Irrelevant | LcnfType::Unit => GoType::GoUnit,
            LcnfType::Object => GoType::GoInterface,
            LcnfType::Var(_) => GoType::GoInterface,
            LcnfType::Ctor(_, _) => {
                GoType::GoPtr(Box::new(GoType::GoStruct("OxiCtor".to_string())))
            }
            LcnfType::Fun(params, ret) => {
                let go_params: Vec<GoType> = params.iter().map(|p| self.compile_type(p)).collect();
                let go_ret = self.compile_type(ret);
                GoType::GoFunc(go_params, vec![go_ret])
            }
        }
    }
    /// Compile an LCNF expression to a sequence of Go statements.
    ///
    /// The last statement should be a `return` carrying the final value.
    pub fn compile_expr(&mut self, expr: &LcnfExpr) -> Vec<GoStmt> {
        match expr {
            LcnfExpr::Return(arg) => {
                let go_expr = self.compile_arg(arg);
                vec![GoStmt::Return(vec![go_expr])]
            }
            LcnfExpr::Unreachable => {
                vec![GoStmt::Panic(GoExpr::Lit(GoLit::Str(
                    "unreachable".to_string(),
                )))]
            }
            LcnfExpr::TailCall(func, args) => {
                let go_func = self.compile_arg(func);
                let go_args: Vec<GoExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                let call = GoExpr::Call(Box::new(go_func), go_args);
                vec![GoStmt::Return(vec![call])]
            }
            LcnfExpr::Let {
                id,
                name,
                ty: _,
                value,
                body,
            } => {
                let go_name = Self::mangle_name(name);
                let go_name = format!("{}_{}", go_name, id.0);
                self.var_map.insert(*id, go_name.clone());
                let mut stmts = Vec::new();
                let val_expr = self.compile_let_value(value);
                stmts.push(GoStmt::ShortDecl(go_name, val_expr));
                let cont = self.compile_expr(body);
                stmts.extend(cont);
                stmts
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => self.compile_case(*scrutinee, alts, default.as_deref()),
        }
    }
    /// Compile a `LcnfLetValue` into a Go expression.
    pub(super) fn compile_let_value(&mut self, value: &LcnfLetValue) -> GoExpr {
        match value {
            LcnfLetValue::Lit(lit) => self.compile_lit(lit),
            LcnfLetValue::Erased | LcnfLetValue::Reset(_) => GoExpr::Lit(GoLit::Nil),
            LcnfLetValue::FVar(id) => {
                let name = self
                    .var_map
                    .get(id)
                    .cloned()
                    .unwrap_or_else(|| format!("_x{}", id.0));
                GoExpr::Var(name)
            }
            LcnfLetValue::App(func, args) => {
                let go_func = self.compile_arg(func);
                let go_args: Vec<GoExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                GoExpr::Call(Box::new(go_func), go_args)
            }
            LcnfLetValue::Proj(_, idx, var) => {
                let var_name = self
                    .var_map
                    .get(var)
                    .cloned()
                    .unwrap_or_else(|| format!("_x{}", var.0));
                GoExpr::Index(
                    Box::new(GoExpr::Field(
                        Box::new(GoExpr::Var(var_name)),
                        "Fields".to_string(),
                    )),
                    Box::new(GoExpr::Lit(GoLit::Int(*idx as i64))),
                )
            }
            LcnfLetValue::Ctor(name, tag, args) => {
                let go_name = Self::mangle_name(name);
                let mut fields = vec![
                    ("Tag".to_string(), GoExpr::Lit(GoLit::Int(*tag as i64))),
                    ("_ctorName".to_string(), GoExpr::Lit(GoLit::Str(go_name))),
                ];
                if !args.is_empty() {
                    let go_args: Vec<GoExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                    fields.push((
                        "Fields".to_string(),
                        GoExpr::SliceLit(GoType::GoInterface, go_args),
                    ));
                } else {
                    fields.push((
                        "Fields".to_string(),
                        GoExpr::SliceLit(GoType::GoInterface, vec![]),
                    ));
                }
                GoExpr::AddressOf(Box::new(GoExpr::Composite(
                    GoType::GoStruct("OxiCtor".to_string()),
                    fields,
                )))
            }
            LcnfLetValue::Reuse(_slot, name, tag, args) => {
                self.compile_let_value(&LcnfLetValue::Ctor(name.clone(), *tag, args.clone()))
            }
        }
    }
    /// Compile an `LcnfArg` into a Go expression.
    pub fn compile_arg(&self, arg: &LcnfArg) -> GoExpr {
        match arg {
            LcnfArg::Var(id) => {
                let name = self
                    .var_map
                    .get(id)
                    .cloned()
                    .unwrap_or_else(|| format!("_x{}", id.0));
                GoExpr::Var(name)
            }
            LcnfArg::Lit(lit) => self.compile_lit(lit),
            LcnfArg::Erased | LcnfArg::Type(_) => GoExpr::Lit(GoLit::Nil),
        }
    }
    /// Compile an `LcnfLit` to a Go expression.
    pub(super) fn compile_lit(&self, lit: &LcnfLit) -> GoExpr {
        match lit {
            LcnfLit::Nat(n) => GoExpr::Lit(GoLit::Int(*n as i64)),
            LcnfLit::Str(s) => GoExpr::Lit(GoLit::Str(s.clone())),
        }
    }
    /// Compile a `Case` expression to Go statements (switch).
    pub(super) fn compile_case(
        &mut self,
        scrutinee: LcnfVarId,
        alts: &[LcnfAlt],
        default: Option<&LcnfExpr>,
    ) -> Vec<GoStmt> {
        let scr_name = self
            .var_map
            .get(&scrutinee)
            .cloned()
            .unwrap_or_else(|| format!("_x{}", scrutinee.0));
        let tag_expr = GoExpr::Field(Box::new(GoExpr::Var(scr_name.clone())), "Tag".to_string());
        let mut cases: Vec<GoCase> = Vec::new();
        for alt in alts {
            let mut alt_body: Vec<GoStmt> = Vec::new();
            for (idx, param) in alt.params.iter().enumerate() {
                if !param.erased {
                    let go_param = Self::mangle_name(&param.name);
                    let go_param = format!("{}_{}", go_param, param.id.0);
                    self.var_map.insert(param.id, go_param.clone());
                    let field_expr = GoExpr::Index(
                        Box::new(GoExpr::Field(
                            Box::new(GoExpr::Var(scr_name.clone())),
                            "Fields".to_string(),
                        )),
                        Box::new(GoExpr::Lit(GoLit::Int(idx as i64))),
                    );
                    alt_body.push(GoStmt::ShortDecl(go_param, field_expr));
                }
            }
            let cont = self.compile_expr(&alt.body);
            alt_body.extend(cont);
            cases.push(GoCase {
                pattern: Some(vec![GoExpr::Lit(GoLit::Int(alt.ctor_tag as i64))]),
                body: alt_body,
            });
        }
        if let Some(def_expr) = default {
            let def_stmts = self.compile_expr(def_expr);
            cases.push(GoCase {
                pattern: None,
                body: def_stmts,
            });
        }
        vec![GoStmt::Switch(Some(tag_expr), cases)]
    }
    /// Build the minimal OxiLean Go runtime as a list of Go functions.
    ///
    /// These mirror the Lean 4 / OxiLean built-in operations that compiled code may call.
    pub(super) fn build_runtime(&self) -> Vec<GoFunc> {
        let mut funcs = Vec::new();
        funcs.push(self.simple_binop_func("natAdd", "+"));
        {
            let mut f = GoFunc::new("natSub");
            f.add_param("a", GoType::GoInt);
            f.add_param("b", GoType::GoInt);
            f.add_return(GoType::GoInt);
            f.body = vec![GoStmt::If(
                GoExpr::BinOp(
                    ">=".to_string(),
                    Box::new(GoExpr::Var("a".to_string())),
                    Box::new(GoExpr::Var("b".to_string())),
                ),
                vec![GoStmt::Return(vec![GoExpr::BinOp(
                    "-".to_string(),
                    Box::new(GoExpr::Var("a".to_string())),
                    Box::new(GoExpr::Var("b".to_string())),
                )])],
                vec![GoStmt::Return(vec![GoExpr::Lit(GoLit::Int(0))])],
            )];
            funcs.push(f);
        }
        funcs.push(self.simple_binop_func("natMul", "*"));
        {
            let mut f = GoFunc::new("natDiv");
            f.add_param("a", GoType::GoInt);
            f.add_param("b", GoType::GoInt);
            f.add_return(GoType::GoInt);
            f.body = vec![GoStmt::If(
                GoExpr::BinOp(
                    "==".to_string(),
                    Box::new(GoExpr::Var("b".to_string())),
                    Box::new(GoExpr::Lit(GoLit::Int(0))),
                ),
                vec![GoStmt::Return(vec![GoExpr::Lit(GoLit::Int(0))])],
                vec![GoStmt::Return(vec![GoExpr::BinOp(
                    "/".to_string(),
                    Box::new(GoExpr::Var("a".to_string())),
                    Box::new(GoExpr::Var("b".to_string())),
                )])],
            )];
            funcs.push(f);
        }
        {
            let mut f = GoFunc::new("natMod");
            f.add_param("a", GoType::GoInt);
            f.add_param("b", GoType::GoInt);
            f.add_return(GoType::GoInt);
            f.body = vec![GoStmt::If(
                GoExpr::BinOp(
                    "==".to_string(),
                    Box::new(GoExpr::Var("b".to_string())),
                    Box::new(GoExpr::Lit(GoLit::Int(0))),
                ),
                vec![GoStmt::Return(vec![GoExpr::Lit(GoLit::Int(0))])],
                vec![GoStmt::Return(vec![GoExpr::BinOp(
                    "%".to_string(),
                    Box::new(GoExpr::Var("a".to_string())),
                    Box::new(GoExpr::Var("b".to_string())),
                )])],
            )];
            funcs.push(f);
        }
        funcs.push(self.simple_cmp_func("natEq", "=="));
        funcs.push(self.simple_cmp_func("natLt", "<"));
        funcs.push(self.simple_cmp_func("natLe", "<="));
        funcs.push(self.simple_cmp_func("natGt", ">"));
        funcs.push(self.simple_cmp_func("natGe", ">="));
        funcs.push(self.simple_bool_func("boolAnd", "&&"));
        funcs.push(self.simple_bool_func("boolOr", "||"));
        {
            let mut f = GoFunc::new("boolNot");
            f.add_param("a", GoType::GoBool);
            f.add_return(GoType::GoBool);
            f.body = vec![GoStmt::Return(vec![GoExpr::Unary(
                "!".to_string(),
                Box::new(GoExpr::Var("a".to_string())),
            )])];
            funcs.push(f);
        }
        {
            let mut f = GoFunc::new("strAppend");
            f.add_param("a", GoType::GoString);
            f.add_param("b", GoType::GoString);
            f.add_return(GoType::GoString);
            f.body = vec![GoStmt::Return(vec![GoExpr::BinOp(
                "+".to_string(),
                Box::new(GoExpr::Var("a".to_string())),
                Box::new(GoExpr::Var("b".to_string())),
            )])];
            funcs.push(f);
        }
        {
            let mut f = GoFunc::new("strEq");
            f.add_param("a", GoType::GoString);
            f.add_param("b", GoType::GoString);
            f.add_return(GoType::GoBool);
            f.body = vec![GoStmt::Return(vec![GoExpr::BinOp(
                "==".to_string(),
                Box::new(GoExpr::Var("a".to_string())),
                Box::new(GoExpr::Var("b".to_string())),
            )])];
            funcs.push(f);
        }
        {
            let mut f = GoFunc::new("strLen");
            f.add_param("s", GoType::GoString);
            f.add_return(GoType::GoInt);
            f.body = vec![GoStmt::Return(vec![GoExpr::Call(
                Box::new(GoExpr::Var("int64".to_string())),
                vec![GoExpr::Call(
                    Box::new(GoExpr::Var("len".to_string())),
                    vec![GoExpr::Var("s".to_string())],
                )],
            )])];
            funcs.push(f);
        }
        {
            let mut f = GoFunc::new("oxiPrint");
            f.add_param("s", GoType::GoString);
            f.body = vec![GoStmt::Expr(GoExpr::Call(
                Box::new(GoExpr::Field(
                    Box::new(GoExpr::Var("fmt".to_string())),
                    "Println".to_string(),
                )),
                vec![GoExpr::Var("s".to_string())],
            ))];
            funcs.push(f);
        }
        {
            let mut f = GoFunc::new("oxiPanic");
            f.add_param("msg", GoType::GoString);
            f.body = vec![GoStmt::Panic(GoExpr::Var("msg".to_string()))];
            funcs.push(f);
        }
        funcs
    }
    /// Helper: build a two-arg int64 → int64 binary-op function.
    pub(super) fn simple_binop_func(&self, name: &str, op: &str) -> GoFunc {
        let mut f = GoFunc::new(name);
        f.add_param("a", GoType::GoInt);
        f.add_param("b", GoType::GoInt);
        f.add_return(GoType::GoInt);
        f.body = vec![GoStmt::Return(vec![GoExpr::BinOp(
            op.to_string(),
            Box::new(GoExpr::Var("a".to_string())),
            Box::new(GoExpr::Var("b".to_string())),
        )])];
        f
    }
    /// Helper: build a two-arg int64 → bool comparison function.
    pub(super) fn simple_cmp_func(&self, name: &str, op: &str) -> GoFunc {
        let mut f = GoFunc::new(name);
        f.add_param("a", GoType::GoInt);
        f.add_param("b", GoType::GoInt);
        f.add_return(GoType::GoBool);
        f.body = vec![GoStmt::Return(vec![GoExpr::BinOp(
            op.to_string(),
            Box::new(GoExpr::Var("a".to_string())),
            Box::new(GoExpr::Var("b".to_string())),
        )])];
        f
    }
    /// Helper: build a two-arg bool → bool binary-op function.
    pub(super) fn simple_bool_func(&self, name: &str, op: &str) -> GoFunc {
        let mut f = GoFunc::new(name);
        f.add_param("a", GoType::GoBool);
        f.add_param("b", GoType::GoBool);
        f.add_return(GoType::GoBool);
        f.body = vec![GoStmt::Return(vec![GoExpr::BinOp(
            op.to_string(),
            Box::new(GoExpr::Var("a".to_string())),
            Box::new(GoExpr::Var("b".to_string())),
        )])];
        f
    }
    /// Emit a single `GoFunc` to a string.
    pub fn emit_func(&self, func: &GoFunc) -> String {
        func.codegen()
    }
    /// Emit a `GoTypeDecl` to a string.
    pub fn emit_type_decl(&self, decl: &GoTypeDecl) -> String {
        decl.codegen()
    }
    /// Emit a full `GoModule` to a string.
    pub fn emit_module(&self, module: &GoModule) -> String {
        module.codegen()
    }
}
/// A case in a switch statement.
#[derive(Debug, Clone, PartialEq)]
pub struct GoCase {
    /// `None` means `default:`
    pub pattern: Option<Vec<GoExpr>>,
    pub body: Vec<GoStmt>,
}
/// A Go struct type declaration.
#[derive(Debug, Clone)]
pub struct GoTypeDecl {
    /// Type name.
    pub name: String,
    /// Fields: `[(field_name, field_type), ...]`.
    pub fields: Vec<(String, GoType)>,
    /// Whether exported.
    pub exported: bool,
}
impl GoTypeDecl {
    /// Create a new struct type declaration.
    pub fn new(name: impl Into<String>) -> Self {
        GoTypeDecl {
            name: name.into(),
            fields: Vec::new(),
            exported: false,
        }
    }
    /// Add a field.
    pub fn add_field(&mut self, name: impl Into<String>, ty: GoType) {
        self.fields.push((name.into(), ty));
    }
    /// Emit Go source for this type declaration.
    pub fn codegen(&self) -> String {
        let mut out = format!("type {} struct {{\n", self.name);
        for (name, ty) in &self.fields {
            out.push_str(&format!("    {} {}\n", name, ty));
        }
        out.push('}');
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct GoPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl GoPassStats {
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
/// Go literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum GoLit {
    /// Integer literal: `0`, `42`, `-7`
    Int(i64),
    /// Float literal: `3.14`
    Float(f64),
    /// Boolean literal: `true` or `false`
    Bool(bool),
    /// String literal: `"hello"`
    Str(String),
    /// `nil`
    Nil,
}
/// Go type representation for type-directed code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GoType {
    /// `bool`
    GoBool,
    /// `int64`
    GoInt,
    /// `float64`
    GoFloat,
    /// `string`
    GoString,
    /// `[]T` — a Go slice
    GoSlice(Box<GoType>),
    /// `map[K]V` — a Go map
    GoMap(Box<GoType>, Box<GoType>),
    /// `func(params...) returns...`
    GoFunc(Vec<GoType>, Vec<GoType>),
    /// `interface{}` / `any`
    GoInterface,
    /// A named struct: `MyStruct`
    GoStruct(String),
    /// A pointer: `*T`
    GoPtr(Box<GoType>),
    /// Channel: `chan T`
    GoChan(Box<GoType>),
    /// `error` interface
    GoError,
    /// Unit (represented as an empty struct `struct{}`)
    GoUnit,
}
/// Go statement for code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum GoStmt {
    /// `const name type = value`
    Const(String, Option<GoType>, GoExpr),
    /// `var name type` or `var name type = value`
    Var(String, GoType, Option<GoExpr>),
    /// Short variable declaration: `name := value`
    ShortDecl(String, GoExpr),
    /// Assignment: `target = value`
    Assign(GoExpr, GoExpr),
    /// Return statement: `return exprs...`
    Return(Vec<GoExpr>),
    /// If statement with optional else
    If(GoExpr, Vec<GoStmt>, Vec<GoStmt>),
    /// Switch statement: `switch scrutinee { case ... }`
    Switch(Option<GoExpr>, Vec<GoCase>),
    /// For loop: `for init; cond; post { body }`
    For(
        Option<Box<GoStmt>>,
        Option<GoExpr>,
        Option<Box<GoStmt>>,
        Vec<GoStmt>,
    ),
    /// Range-based for: `for k, v := range expr { body }`
    ForRange(Option<String>, Option<String>, GoExpr, Vec<GoStmt>),
    /// A block of statements: `{ stmts }`
    Block(Vec<GoStmt>),
    /// A bare expression statement
    Expr(GoExpr),
    /// Break statement
    Break,
    /// Continue statement
    Continue,
    /// Goto label
    Goto(String),
    /// Label statement
    Label(String, Box<GoStmt>),
    /// Defer statement
    Defer(GoExpr),
    /// Go statement (goroutine)
    GoRoutine(GoExpr),
    /// Panic
    Panic(GoExpr),
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum GoPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl GoPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            GoPassPhase::Analysis => "analysis",
            GoPassPhase::Transformation => "transformation",
            GoPassPhase::Verification => "verification",
            GoPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, GoPassPhase::Transformation | GoPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
pub struct GoPassRegistry {
    pub(super) configs: Vec<GoPassConfig>,
    pub(super) stats: std::collections::HashMap<String, GoPassStats>,
}
impl GoPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        GoPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: GoPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), GoPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&GoPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&GoPassStats> {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GoCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GoLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl GoLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        GoLivenessInfo {
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
