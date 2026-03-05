//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::functions::*;
use std::collections::VecDeque;

/// Helper to build indented C source text.
pub struct CCodeWriter {
    pub(super) buffer: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
impl CCodeWriter {
    pub(super) fn new(indent_str: &str) -> Self {
        CCodeWriter {
            buffer: String::new(),
            indent_level: 0,
            indent_str: indent_str.to_string(),
        }
    }
    pub(super) fn indent(&mut self) {
        self.indent_level += 1;
    }
    pub(super) fn dedent(&mut self) {
        if self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
    pub(super) fn write_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.buffer.push_str(&self.indent_str);
        }
    }
    pub(super) fn writeln(&mut self, line: &str) {
        self.write_indent();
        self.buffer.push_str(line);
        self.buffer.push('\n');
    }
    pub(super) fn write_blank(&mut self) {
        self.buffer.push('\n');
    }
    pub(super) fn result(&self) -> String {
        self.buffer.clone()
    }
    pub(super) fn line_count(&self) -> usize {
        self.buffer.lines().count()
    }
}
/// Information about a closure struct.
#[derive(Debug, Clone)]
pub struct ClosureInfo {
    /// Name of the closure struct type.
    pub(super) struct_name: String,
    /// The function pointer field name.
    pub(super) fn_ptr_field: String,
    /// Captured environment field names and types.
    pub(super) env_fields: Vec<(String, CType)>,
    /// Arity of the closure (number of explicit parameters).
    pub(super) arity: usize,
}
/// Configuration for the C code emitter.
#[derive(Debug, Clone)]
pub struct CEmitConfig {
    /// Whether to emit comments in the generated code.
    pub emit_comments: bool,
    /// Whether to inline small functions.
    pub inline_small: bool,
    /// Whether to emit reference counting operations.
    pub use_rc: bool,
    /// Indentation string (e.g. two spaces or a tab).
    pub indent: String,
    /// Module name for the header guard.
    pub module_name: String,
    /// Whether to emit `static` qualifiers for internal functions.
    pub use_static: bool,
}
/// C expression for code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum CExpr {
    /// Variable reference.
    Var(String),
    /// Integer literal.
    IntLit(i64),
    /// Unsigned integer literal.
    UIntLit(u64),
    /// String literal.
    StringLit(String),
    /// Function call: `f(args...)`.
    Call(String, Vec<CExpr>),
    /// Binary operation: `lhs op rhs`.
    BinOp(CBinOp, Box<CExpr>, Box<CExpr>),
    /// Unary operation: `op expr`.
    UnaryOp(CUnaryOp, Box<CExpr>),
    /// Field access: `expr.field` or `expr->field`.
    FieldAccess(Box<CExpr>, String, bool),
    /// Array access: `expr[index]`.
    ArrayAccess(Box<CExpr>, Box<CExpr>),
    /// Cast: `(type)expr`.
    Cast(CType, Box<CExpr>),
    /// `sizeof(type)`.
    SizeOf(CType),
    /// `NULL`.
    Null,
    /// Ternary: `cond ? then : else`.
    Ternary(Box<CExpr>, Box<CExpr>, Box<CExpr>),
    /// Compound literal / initializer.
    Initializer(CType, Vec<(String, CExpr)>),
}
impl CExpr {
    /// Create a variable expression.
    pub(super) fn var(name: &str) -> Self {
        CExpr::Var(name.to_string())
    }
    /// Create a function call expression.
    pub(super) fn call(name: &str, args: Vec<CExpr>) -> Self {
        CExpr::Call(name.to_string(), args)
    }
    /// Create a field access (arrow style: `expr->field`).
    pub(super) fn arrow(self, field: &str) -> Self {
        CExpr::FieldAccess(Box::new(self), field.to_string(), true)
    }
    /// Create a field access (dot style: `expr.field`).
    pub(super) fn dot(self, field: &str) -> Self {
        CExpr::FieldAccess(Box::new(self), field.to_string(), false)
    }
    /// Create a binary operation.
    pub(super) fn binop(op: CBinOp, lhs: CExpr, rhs: CExpr) -> Self {
        CExpr::BinOp(op, Box::new(lhs), Box::new(rhs))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CBPassConfig {
    pub phase: CBPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl CBPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: CBPassPhase) -> Self {
        CBPassConfig {
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
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CBPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl CBPassStats {
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
/// Binary operator in C expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}
/// Analyze an LCNF expression to determine which variables need RC operations,
/// and return the expression annotated with inc/dec ref calls (as C statements).
struct RcInserter {
    /// Variables that are "live" (have been inc-refed and need dec-ref).
    pub(super) live_vars: HashMap<LcnfVarId, LcnfType>,
    /// Which variables have been consumed (ownership transferred).
    pub(super) consumed: HashSet<LcnfVarId>,
}
impl RcInserter {
    pub(super) fn new() -> Self {
        RcInserter {
            live_vars: HashMap::new(),
            consumed: HashSet::new(),
        }
    }
    /// Mark a variable as live (holding a reference).
    pub(super) fn mark_live(&mut self, id: LcnfVarId, ty: LcnfType) {
        if !is_scalar_type(&ty) {
            self.live_vars.insert(id, ty);
        }
    }
    /// Mark a variable as consumed (ownership transferred).
    pub(super) fn mark_consumed(&mut self, id: LcnfVarId) {
        self.consumed.insert(id);
    }
    /// Generate dec-ref statements for all live, unconsumed variables.
    pub(super) fn gen_dec_refs(&self) -> Vec<CStmt> {
        let mut stmts = Vec::new();
        for (id, ty) in &self.live_vars {
            if !self.consumed.contains(id) && !is_scalar_type(ty) {
                stmts.push(lean_dec_ref(&var_name(*id)));
            }
        }
        stmts
    }
    /// Generate inc-ref for a variable if it is used multiple times.
    pub(super) fn gen_inc_ref_if_shared(&self, id: LcnfVarId, use_count: usize) -> Vec<CStmt> {
        let mut stmts = Vec::new();
        if use_count > 1 {
            if let Some(ty) = self.live_vars.get(&id) {
                if !is_scalar_type(ty) {
                    for _ in 0..use_count - 1 {
                        stmts.push(lean_inc_ref(&var_name(id)));
                    }
                }
            }
        }
        stmts
    }
}
#[allow(dead_code)]
pub struct CBPassRegistry {
    pub(super) configs: Vec<CBPassConfig>,
    pub(super) stats: std::collections::HashMap<String, CBPassStats>,
}
impl CBPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CBPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: CBPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), CBPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&CBPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&CBPassStats> {
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
pub struct CBWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl CBWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CBWorklist {
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
pub struct CBLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl CBLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        CBLivenessInfo {
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
/// The output of C code generation.
#[derive(Debug, Clone)]
pub struct COutput {
    /// The `.h` header content.
    pub header: String,
    /// The `.c` source content.
    pub source: String,
    /// The collected declarations.
    pub declarations: Vec<CDecl>,
}
impl COutput {
    pub(super) fn new() -> Self {
        COutput {
            header: String::new(),
            source: String::new(),
            declarations: Vec::new(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CBPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl CBPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            CBPassPhase::Analysis => "analysis",
            CBPassPhase::Transformation => "transformation",
            CBPassPhase::Verification => "verification",
            CBPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, CBPassPhase::Transformation | CBPassPhase::Cleanup)
    }
}
/// Information about a generated struct's layout.
#[derive(Debug, Clone)]
pub struct StructLayout {
    /// Name of the struct.
    pub name: String,
    /// Fields with their offsets.
    pub fields: Vec<(String, CType, usize)>,
    /// Total size of the struct.
    pub total_size: usize,
    /// Required alignment.
    pub alignment: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CBDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl CBDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CBDepGraph {
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
/// A top-level C declaration.
#[derive(Debug, Clone, PartialEq)]
pub enum CDecl {
    /// Function definition.
    Function {
        ret_type: CType,
        name: String,
        params: Vec<(CType, String)>,
        body: Vec<CStmt>,
        is_static: bool,
    },
    /// Struct definition.
    Struct {
        name: String,
        fields: Vec<(CType, String)>,
    },
    /// Typedef.
    Typedef { original: CType, alias: String },
    /// Global variable.
    Global {
        ty: CType,
        name: String,
        init: Option<CExpr>,
        is_static: bool,
    },
    /// Forward declaration (function prototype).
    ForwardDecl {
        ret_type: CType,
        name: String,
        params: Vec<(CType, String)>,
    },
}
/// C type representation for code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CType {
    /// `void`
    Void,
    /// `int64_t`
    Int,
    /// `uint64_t`
    UInt,
    /// `uint8_t` (boolean)
    Bool,
    /// `char`
    Char,
    /// Pointer: `T*`
    Ptr(Box<CType>),
    /// Named struct: `struct Foo`
    Struct(String),
    /// Function pointer: `ret (*)(params...)`
    FnPtr(Vec<CType>, Box<CType>),
    /// Fixed-size array: `T[N]`
    Array(Box<CType>, usize),
    /// `size_t`
    SizeT,
    /// `uint8_t`
    U8,
    /// `lean_object*` — the universal OxiLean object type
    LeanObject,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CBAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, CBCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl CBAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        CBAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&CBCacheEntry> {
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
            CBCacheEntry {
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
/// Unary operator in C expressions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CUnaryOp {
    Neg,
    Not,
    BitNot,
    Deref,
    AddrOf,
}
/// The C code generation backend.
///
/// Translates an LCNF module into C99 source code with Lean-compatible
/// runtime support.
pub struct CBackend {
    pub(super) config: CEmitConfig,
    pub(super) stats: CEmitStats,
    /// Struct declarations generated so far (name -> fields).
    pub(super) struct_decls: HashMap<String, Vec<(CType, String)>>,
    /// Forward declarations needed.
    pub(super) forward_decls: Vec<CDecl>,
    /// Counter for generating unique temporary names.
    pub(super) temp_counter: usize,
    /// Mapping from LCNF function names to mangled C names.
    pub(super) name_map: HashMap<String, String>,
    /// Set of functions that are known to be used as closures.
    pub(super) closure_functions: HashSet<String>,
}
impl CBackend {
    /// Create a new C backend with the given configuration.
    pub fn new(config: CEmitConfig) -> Self {
        CBackend {
            config,
            stats: CEmitStats::default(),
            struct_decls: HashMap::new(),
            forward_decls: Vec::new(),
            temp_counter: 0,
            name_map: HashMap::new(),
            closure_functions: HashSet::new(),
        }
    }
    /// Create a C backend with default configuration.
    pub fn default_backend() -> Self {
        Self::new(CEmitConfig::default())
    }
    /// Get a fresh temporary variable name.
    pub(super) fn fresh_temp(&mut self) -> String {
        let name = format!("_t{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }
    /// Get the mangled C name for an LCNF name.
    pub(super) fn get_c_name(&mut self, lcnf_name: &str) -> String {
        if let Some(name) = self.name_map.get(lcnf_name) {
            return name.clone();
        }
        let mangled = mangle_name(lcnf_name);
        self.name_map.insert(lcnf_name.to_string(), mangled.clone());
        mangled
    }
    /// Emit a complete LCNF module to C code.
    pub fn emit_module(&mut self, module: &LcnfModule) -> COutput {
        let mut output = COutput::new();
        let mut decls: Vec<CDecl> = Vec::new();
        for fun in &module.fun_decls {
            let c_name = self.get_c_name(&fun.name);
            let c_params = self.emit_params(&fun.params);
            let c_ret = lcnf_type_to_ctype(&fun.ret_type);
            decls.push(CDecl::ForwardDecl {
                ret_type: c_ret,
                name: c_name,
                params: c_params,
            });
        }
        for ext in &module.extern_decls {
            let c_name = self.get_c_name(&ext.name);
            let c_params = self.emit_params(&ext.params);
            let c_ret = lcnf_type_to_ctype(&ext.ret_type);
            decls.push(CDecl::ForwardDecl {
                ret_type: c_ret,
                name: c_name,
                params: c_params,
            });
        }
        for fun in &module.fun_decls {
            let fun_decl = self.emit_fun_decl(fun);
            decls.push(fun_decl);
        }
        let mut header_writer = CCodeWriter::new(&self.config.indent);
        let guard = format!(
            "{}_H",
            self.config.module_name.to_uppercase().replace('.', "_")
        );
        header_writer.writeln(&format!("#ifndef {}", guard));
        header_writer.writeln(&format!("#define {}", guard));
        header_writer.write_blank();
        header_writer.writeln("#include <stdint.h>");
        header_writer.writeln("#include <stddef.h>");
        header_writer.writeln("#include \"lean_runtime.h\"");
        header_writer.write_blank();
        for name in self.struct_decls.keys() {
            header_writer.writeln(&format!("struct {};", name));
        }
        header_writer.write_blank();
        for (name, fields) in &self.struct_decls {
            let struct_decl = CDecl::Struct {
                name: name.clone(),
                fields: fields.clone(),
            };
            emit_decl(&mut header_writer, &struct_decl);
        }
        for d in &decls {
            if let CDecl::ForwardDecl { .. } = d {
                emit_decl(&mut header_writer, d);
            }
        }
        header_writer.write_blank();
        header_writer.writeln(&format!("#endif /* {} */", guard));
        output.header = header_writer.result();
        let mut source_writer = CCodeWriter::new(&self.config.indent);
        source_writer.writeln(&format!("#include \"{}.h\"", self.config.module_name));
        source_writer.write_blank();
        for d in &decls {
            if let CDecl::Function { .. } = d {
                emit_decl(&mut source_writer, d);
            }
        }
        self.stats.total_lines = source_writer.line_count() + header_writer.line_count();
        output.source = source_writer.result();
        output.declarations = decls;
        output
    }
    /// Emit an LCNF function declaration as a C function.
    pub fn emit_fun_decl(&mut self, decl: &LcnfFunDecl) -> CDecl {
        let c_name = self.get_c_name(&decl.name);
        let c_params = self.emit_params(&decl.params);
        let c_ret = lcnf_type_to_ctype(&decl.ret_type);
        let mut body_stmts = Vec::new();
        if self.config.emit_comments {
            body_stmts.push(CStmt::Comment(format!(
                "Function: {} (recursive={})",
                decl.name, decl.is_recursive
            )));
        }
        let expr_stmts = self.emit_expr(&decl.body, &decl.ret_type);
        body_stmts.extend(expr_stmts);
        self.stats.functions_emitted += 1;
        CDecl::Function {
            ret_type: c_ret,
            name: c_name,
            params: c_params,
            body: body_stmts,
            is_static: decl.is_lifted && self.config.use_static,
        }
    }
    /// Convert LCNF parameters to C function parameters.
    pub(super) fn emit_params(&self, params: &[LcnfParam]) -> Vec<(CType, String)> {
        params
            .iter()
            .filter(|p| !p.erased)
            .map(|p| (lcnf_type_to_ctype(&p.ty), var_name(p.id)))
            .collect()
    }
    /// Map an LCNF type to a C type (public interface).
    pub fn emit_type(&self, ty: &LcnfType) -> CType {
        lcnf_type_to_ctype(ty)
    }
    /// Emit C statements for an LCNF expression.
    pub fn emit_expr(&mut self, expr: &LcnfExpr, ret_ty: &LcnfType) -> Vec<CStmt> {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let mut stmts = Vec::new();
                let c_ty = lcnf_type_to_ctype(ty);
                let c_name = var_name(*id);
                if self.config.emit_comments && !name.is_empty() {
                    stmts.push(CStmt::Comment(format!("{} : {}", name, ty)));
                }
                let (init_stmts, init_expr) = self.emit_let_value(value, ty);
                stmts.extend(init_stmts);
                stmts.push(CStmt::VarDecl {
                    ty: c_ty,
                    name: c_name,
                    init: Some(init_expr),
                });
                let body_stmts = self.emit_expr(body, ret_ty);
                stmts.extend(body_stmts);
                stmts
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            } => {
                let mut stmts = Vec::new();
                let scrut_name = var_name(*scrutinee);
                if is_scalar_type(scrutinee_ty) {
                    stmts.extend(self.emit_scalar_case(&scrut_name, alts, default, ret_ty));
                } else {
                    let tag_var = self.fresh_temp();
                    stmts.push(CStmt::VarDecl {
                        ty: CType::U8,
                        name: tag_var.clone(),
                        init: Some(lean_obj_tag(&scrut_name)),
                    });
                    let mut cases: Vec<(u32, Vec<CStmt>)> = Vec::new();
                    for alt in alts {
                        let mut case_body = Vec::new();
                        if self.config.emit_comments {
                            case_body.push(CStmt::Comment(format!(
                                "Constructor: {}#{}",
                                alt.ctor_name, alt.ctor_tag
                            )));
                        }
                        for (i, param) in alt.params.iter().enumerate() {
                            if param.erased {
                                continue;
                            }
                            let field_ty = lcnf_type_to_ctype(&param.ty);
                            let field_name = var_name(param.id);
                            case_body.push(CStmt::VarDecl {
                                ty: field_ty.clone(),
                                name: field_name.clone(),
                                init: Some(lean_ctor_get(&scrut_name, i)),
                            });
                            if self.config.use_rc && needs_rc(&field_ty) {
                                case_body.push(lean_inc_ref(&field_name));
                                self.stats.rc_inc_calls += 1;
                            }
                        }
                        let branch_stmts = self.emit_expr(&alt.body, ret_ty);
                        case_body.extend(branch_stmts);
                        cases.push((alt.ctor_tag, case_body));
                    }
                    let default_body = if let Some(def) = default {
                        self.emit_expr(def, ret_ty)
                    } else {
                        vec![CStmt::Expr(CExpr::call(
                            "lean_internal_panic_unreachable",
                            vec![],
                        ))]
                    };
                    stmts.push(CStmt::Switch {
                        scrutinee: CExpr::var(&tag_var),
                        cases,
                        default: default_body,
                    });
                    self.stats.switches_emitted += 1;
                }
                if self.config.use_rc && !is_scalar_type(scrutinee_ty) {
                    stmts.push(lean_dec_ref(&scrut_name));
                    self.stats.rc_dec_calls += 1;
                }
                stmts
            }
            LcnfExpr::Return(arg) => {
                let c_expr = self.emit_arg(arg);
                vec![CStmt::Return(Some(c_expr))]
            }
            LcnfExpr::Unreachable => {
                vec![
                    CStmt::Expr(CExpr::call("lean_internal_panic_unreachable", vec![])),
                    CStmt::Return(Some(CExpr::Null)),
                ]
            }
            LcnfExpr::TailCall(func, args) => {
                let c_func = self.emit_arg(func);
                let c_args: Vec<CExpr> = args.iter().map(|a| self.emit_arg(a)).collect();
                let call_expr = match c_func {
                    CExpr::Var(name) => CExpr::Call(name, c_args),
                    _ => {
                        let tmp = self.fresh_temp();
                        return vec![
                            CStmt::Comment("Indirect tail call".to_string()),
                            CStmt::Return(Some(CExpr::Call(tmp, c_args))),
                        ];
                    }
                };
                vec![CStmt::Return(Some(call_expr))]
            }
        }
    }
    /// Emit a scalar case (if-else chain) for Nat / simple types.
    pub(super) fn emit_scalar_case(
        &mut self,
        scrut_name: &str,
        alts: &[LcnfAlt],
        default: &Option<Box<LcnfExpr>>,
        ret_ty: &LcnfType,
    ) -> Vec<CStmt> {
        let mut stmts = Vec::new();
        if alts.is_empty() {
            if let Some(def) = default {
                stmts.extend(self.emit_expr(def, ret_ty));
            } else {
                stmts.push(CStmt::Expr(CExpr::call(
                    "lean_internal_panic_unreachable",
                    vec![],
                )));
            }
            return stmts;
        }
        let mut remaining = alts.to_vec();
        remaining.reverse();
        let first = remaining
            .pop()
            .expect("alts is non-empty after reverse; guaranteed by caller");
        let mut result = {
            let cond = CExpr::binop(
                CBinOp::Eq,
                CExpr::var(scrut_name),
                CExpr::UIntLit(first.ctor_tag as u64),
            );
            let then_body = self.emit_expr(&first.body, ret_ty);
            let else_body = if remaining.is_empty() {
                if let Some(def) = default {
                    self.emit_expr(def, ret_ty)
                } else {
                    vec![CStmt::Expr(CExpr::call(
                        "lean_internal_panic_unreachable",
                        vec![],
                    ))]
                }
            } else {
                Vec::new()
            };
            CStmt::If {
                cond,
                then_body,
                else_body,
            }
        };
        while let Some(alt) = remaining.pop() {
            let cond = CExpr::binop(
                CBinOp::Eq,
                CExpr::var(scrut_name),
                CExpr::UIntLit(alt.ctor_tag as u64),
            );
            let then_body = self.emit_expr(&alt.body, ret_ty);
            let else_body = if remaining.is_empty() {
                if let Some(def) = default {
                    self.emit_expr(def, ret_ty)
                } else {
                    vec![result]
                }
            } else {
                vec![result]
            };
            result = CStmt::If {
                cond,
                then_body,
                else_body,
            };
        }
        stmts.push(result);
        stmts
    }
    /// Emit a let-value, returning any setup statements and the final expression.
    pub(super) fn emit_let_value(
        &mut self,
        value: &LcnfLetValue,
        _ty: &LcnfType,
    ) -> (Vec<CStmt>, CExpr) {
        match value {
            LcnfLetValue::App(func, args) => {
                let c_func = self.emit_arg(func);
                let c_args: Vec<CExpr> = args.iter().map(|a| self.emit_arg(a)).collect();
                let call = match c_func {
                    CExpr::Var(name) => CExpr::Call(name, c_args),
                    _ => {
                        let tmp = self.fresh_temp();
                        let stmts = gen_closure_apply("_closure", &c_args, &tmp);
                        return (stmts, CExpr::var(&tmp));
                    }
                };
                (Vec::new(), call)
            }
            LcnfLetValue::Proj(_name, idx, var) => {
                let obj = var_name(*var);
                (Vec::new(), lean_ctor_get(&obj, *idx as usize))
            }
            LcnfLetValue::Ctor(name, tag, args) => {
                let mut stmts = Vec::new();
                let num_objs = args.len();
                let ctor_var = self.fresh_temp();
                stmts.push(CStmt::VarDecl {
                    ty: CType::LeanObject,
                    name: ctor_var.clone(),
                    init: Some(lean_alloc_ctor(*tag, num_objs, 0)),
                });
                for (i, arg) in args.iter().enumerate() {
                    let c_arg = self.emit_arg(arg);
                    stmts.push(lean_ctor_set(&ctor_var, i, c_arg));
                }
                if self.config.emit_comments {
                    stmts.insert(0, CStmt::Comment(format!("Construct {}#{}", name, tag)));
                }
                (stmts, CExpr::var(&ctor_var))
            }
            LcnfLetValue::Lit(lit) => {
                let c_expr = match lit {
                    LcnfLit::Nat(n) => CExpr::UIntLit(*n),
                    LcnfLit::Str(s) => {
                        CExpr::call("lean_mk_string", vec![CExpr::StringLit(s.clone())])
                    }
                };
                (Vec::new(), c_expr)
            }
            LcnfLetValue::Erased => (Vec::new(), lean_box(CExpr::UIntLit(0))),
            LcnfLetValue::FVar(var) => (Vec::new(), CExpr::var(&var_name(*var))),
            LcnfLetValue::Reset(var) => (
                Vec::new(),
                CExpr::call("lean_obj_reset", vec![CExpr::var(&var_name(*var))]),
            ),
            LcnfLetValue::Reuse(slot, name, tag, args) => {
                let mut stmts = Vec::new();
                let num_objs = args.len();
                let ctor_var = self.fresh_temp();
                stmts.push(CStmt::VarDecl {
                    ty: CType::LeanObject,
                    name: ctor_var.clone(),
                    init: Some(CExpr::call(
                        "lean_alloc_ctor_using",
                        vec![
                            CExpr::var(&var_name(*slot)),
                            CExpr::UIntLit(*tag as u64),
                            CExpr::UIntLit(num_objs as u64),
                            CExpr::UIntLit(0),
                        ],
                    )),
                });
                for (i, arg) in args.iter().enumerate() {
                    let c_arg = self.emit_arg(arg);
                    stmts.push(lean_ctor_set(&ctor_var, i, c_arg));
                }
                if self.config.emit_comments {
                    stmts.insert(
                        0,
                        CStmt::Comment(format!("Reuse slot for {}#{}", name, tag)),
                    );
                }
                (stmts, CExpr::var(&ctor_var))
            }
        }
    }
    /// Emit a C expression for an LCNF argument.
    pub(super) fn emit_arg(&self, arg: &LcnfArg) -> CExpr {
        match arg {
            LcnfArg::Var(id) => CExpr::var(&var_name(*id)),
            LcnfArg::Lit(lit) => match lit {
                LcnfLit::Nat(n) => CExpr::UIntLit(*n),
                LcnfLit::Str(s) => CExpr::call("lean_mk_string", vec![CExpr::StringLit(s.clone())]),
            },
            LcnfArg::Erased => lean_box(CExpr::UIntLit(0)),
            LcnfArg::Type(_) => lean_box(CExpr::UIntLit(0)),
        }
    }
    /// Get the accumulated statistics.
    pub fn stats(&self) -> &CEmitStats {
        &self.stats
    }
}
/// C statement for code generation.
#[derive(Debug, Clone, PartialEq)]
pub enum CStmt {
    /// Variable declaration: `type name = init;`
    VarDecl {
        ty: CType,
        name: String,
        init: Option<CExpr>,
    },
    /// Assignment: `lhs = rhs;`
    Assign(CExpr, CExpr),
    /// If-else statement.
    If {
        cond: CExpr,
        then_body: Vec<CStmt>,
        else_body: Vec<CStmt>,
    },
    /// Switch statement.
    Switch {
        scrutinee: CExpr,
        cases: Vec<(u32, Vec<CStmt>)>,
        default: Vec<CStmt>,
    },
    /// While loop.
    While { cond: CExpr, body: Vec<CStmt> },
    /// Return statement.
    Return(Option<CExpr>),
    /// Block (compound statement).
    Block(Vec<CStmt>),
    /// Expression statement.
    Expr(CExpr),
    /// Comment.
    Comment(String),
    /// Blank line (for readability).
    Blank,
    /// Label for goto.
    Label(String),
    /// Goto statement.
    Goto(String),
    /// Break statement.
    Break,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CBDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl CBDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        CBDominatorTree {
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
/// Statistics collected during C code emission.
#[derive(Debug, Clone, Default)]
pub struct CEmitStats {
    /// Number of functions emitted.
    pub functions_emitted: usize,
    /// Number of structs emitted.
    pub structs_emitted: usize,
    /// Number of RC inc calls inserted.
    pub rc_inc_calls: usize,
    /// Number of RC dec calls inserted.
    pub rc_dec_calls: usize,
    /// Number of closures emitted.
    pub closures_emitted: usize,
    /// Number of switch statements emitted.
    pub switches_emitted: usize,
    /// Total lines of generated C code.
    pub total_lines: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CBCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
pub struct CBConstantFoldingHelper;
impl CBConstantFoldingHelper {
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
