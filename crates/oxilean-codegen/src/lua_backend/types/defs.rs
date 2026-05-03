//! Type definitions

use super::super::functions::LUA_KEYWORDS;
use super::super::functions::*;
use crate::lcnf::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

/// A text buffer for building LuaExt output source code.
#[derive(Debug, Default)]
pub struct LuaExtSourceBuffer {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
/// Emission statistics for LuaExt.
#[derive(Debug, Clone, Default)]
pub struct LuaExtEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaPassConfig {
    pub phase: LuaPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
/// A version tag for LuaExt output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LuaExtVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
/// Lua type representation (runtime tags + structural hints).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaType {
    /// `nil`
    Nil,
    /// `boolean`
    Boolean,
    /// `number` — integer when `is_int` is true, float otherwise
    Number(bool),
    /// `string`
    String,
    /// `table`
    Table,
    /// `function`
    Function,
    /// `userdata`
    Userdata,
    /// `thread` (coroutine)
    Thread,
    /// Named/custom type
    Custom(std::string::String),
}
/// A named Lua function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct LuaFunction {
    /// Function name (None for anonymous)
    pub name: Option<std::string::String>,
    /// Parameter names
    pub params: Vec<std::string::String>,
    /// Whether the function accepts varargs (`...`)
    pub vararg: bool,
    /// Function body statements
    pub body: Vec<LuaStmt>,
    /// Whether the function is `local`
    pub is_local: bool,
    /// Whether the function is a method (uses `:` syntax)
    pub is_method: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LuaPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
/// Worklist for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
/// Statistics for LuaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LuaExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LuaPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
/// Lua statement AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum LuaStmt {
    /// Assignment: `targets = values`
    Assign {
        targets: Vec<LuaExpr>,
        values: Vec<LuaExpr>,
    },
    /// Local variable declaration: `local names = values`
    LocalAssign {
        names: Vec<std::string::String>,
        attribs: Vec<Option<std::string::String>>,
        values: Vec<LuaExpr>,
    },
    /// Do block: `do ... end`
    Do(Vec<LuaStmt>),
    /// While loop: `while cond do ... end`
    While { cond: LuaExpr, body: Vec<LuaStmt> },
    /// Repeat-until loop: `repeat ... until cond`
    Repeat { body: Vec<LuaStmt>, cond: LuaExpr },
    /// If-elseif-else: `if cond then ... [elseif ...] [else ...] end`
    If {
        cond: LuaExpr,
        then_body: Vec<LuaStmt>,
        elseif_clauses: Vec<(LuaExpr, Vec<LuaStmt>)>,
        else_body: Option<Vec<LuaStmt>>,
    },
    /// Numeric for: `for var = start, limit[, step] do ... end`
    For {
        var: std::string::String,
        start: LuaExpr,
        limit: LuaExpr,
        step: Option<LuaExpr>,
        body: Vec<LuaStmt>,
    },
    /// Generic for: `for names in exprs do ... end`
    ForIn {
        names: Vec<std::string::String>,
        exprs: Vec<LuaExpr>,
        body: Vec<LuaStmt>,
    },
    /// Named function definition: `function name(params) ... end`
    Function(LuaFunction),
    /// Local function definition: `local function name(params) ... end`
    Local(LuaFunction),
    /// Return statement: `return [exprs]`
    Return(Vec<LuaExpr>),
    /// Break statement: `break`
    Break,
    /// Goto statement: `goto label`
    Goto(std::string::String),
    /// Label statement: `::label::`
    Label(std::string::String),
    /// Expression statement (function call): `expr`
    Call(LuaExpr),
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in LuaExt).
#[derive(Debug)]
pub struct LuaExtEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
}
/// Liveness analysis for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LuaExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
/// Pipeline profiler for LuaExt.
#[derive(Debug, Default)]
pub struct LuaExtProfiler {
    pub(super) timings: Vec<LuaExtPassTiming>,
}
/// Collects LuaExt diagnostics.
#[derive(Debug, Default)]
pub struct LuaExtDiagCollector {
    pub(super) msgs: Vec<LuaExtDiagMsg>,
}
/// Heuristic freshness key for LuaExt incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LuaExtIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
/// Lua code generation backend.
pub struct LuaBackend {
    /// Counter for fresh variable names.
    pub(super) fresh_counter: u64,
    /// Name mangling cache.
    pub(super) name_cache: HashMap<std::string::String, std::string::String>,
}
/// A generic key-value configuration store for LuaExt.
#[derive(Debug, Clone, Default)]
pub struct LuaExtConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A complete Lua module / script.
#[derive(Debug, Clone, PartialEq)]
pub struct LuaModule {
    /// `require` imports: `local modname = require("modname")`
    pub requires: Vec<(std::string::String, std::string::String)>,
    /// Top-level local declarations (before functions)
    pub local_decls: Vec<LuaStmt>,
    /// Top-level function definitions
    pub functions: Vec<LuaFunction>,
    /// Main block statements (executed at top level)
    pub main_block: Vec<LuaStmt>,
}
/// Tracks declared names for LuaExt scope analysis.
#[derive(Debug, Default)]
pub struct LuaExtNameScope {
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<LuaExtNameScope>>,
}
/// A Lua class emulated via metatables.
#[derive(Debug, Clone, PartialEq)]
pub struct LuaClass {
    /// Class name
    pub name: std::string::String,
    /// Methods (not including `new`)
    pub methods: Vec<LuaFunction>,
    /// __index implementation (default: self-reference)
    pub index: Option<LuaExpr>,
    /// __newindex implementation
    pub newindex: Option<LuaExpr>,
    /// __tostring implementation body
    pub tostring_body: Option<Vec<LuaStmt>>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
/// Pass registry for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct LuaExtPassRegistry {
    pub(super) configs: Vec<LuaExtPassConfig>,
    pub(super) stats: Vec<LuaExtPassStats>,
}
/// A feature flag set for LuaExt capabilities.
#[derive(Debug, Clone, Default)]
pub struct LuaExtFeatures {
    pub(super) flags: std::collections::HashSet<String>,
}
/// Lua expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum LuaExpr {
    /// `nil`
    Nil,
    /// `true`
    True,
    /// `false`
    False,
    /// Integer literal: `42`
    Int(i64),
    /// Float literal: `3.14`
    Float(f64),
    /// String literal: `"hello"`
    Str(std::string::String),
    /// Variable reference: `x`
    Var(std::string::String),
    /// Binary operation: `lhs op rhs`
    BinOp {
        op: std::string::String,
        lhs: Box<LuaExpr>,
        rhs: Box<LuaExpr>,
    },
    /// Unary operation: `op operand`
    UnaryOp {
        op: std::string::String,
        operand: Box<LuaExpr>,
    },
    /// Function call: `func(args)`
    Call {
        func: Box<LuaExpr>,
        args: Vec<LuaExpr>,
    },
    /// Method call: `obj:method(args)`
    MethodCall {
        obj: Box<LuaExpr>,
        method: std::string::String,
        args: Vec<LuaExpr>,
    },
    /// Table constructor: `{field, ...}`
    TableConstructor(Vec<LuaTableField>),
    /// Index access: `table[key]`
    IndexAccess {
        table: Box<LuaExpr>,
        key: Box<LuaExpr>,
    },
    /// Field access: `table.field`
    FieldAccess {
        table: Box<LuaExpr>,
        field: std::string::String,
    },
    /// Lambda (anonymous function): `function(params) body end`
    Lambda {
        params: Vec<std::string::String>,
        vararg: bool,
        body: Vec<LuaStmt>,
    },
    /// Vararg expression: `...`
    Ellipsis,
}
/// Configuration for LuaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaExtPassConfig {
    pub name: String,
    pub phase: LuaExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
/// A monotonically increasing ID generator for LuaExt.
#[derive(Debug, Default)]
pub struct LuaExtIdGen {
    pub(super) next: u32,
}
#[allow(dead_code)]
pub struct LuaPassRegistry {
    pub(super) configs: Vec<LuaPassConfig>,
    pub(super) stats: std::collections::HashMap<String, LuaPassStats>,
}
/// Constant folding helper for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct LuaExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
/// Analysis cache for LuaExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct LuaExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
/// Pass-timing record for LuaExt profiler.
#[derive(Debug, Clone)]
pub struct LuaExtPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
/// Pass execution phase for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LuaExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
/// A single field in a Lua table constructor.
#[derive(Debug, Clone, PartialEq)]
pub enum LuaTableField {
    /// Positional element: `expr` (auto-indexed)
    ArrayItem(LuaExpr),
    /// Named field: `key = expr`
    NamedField(std::string::String, LuaExpr),
    /// Computed-key field: `[key_expr] = expr`
    IndexedField(LuaExpr, LuaExpr),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, LuaCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
#[allow(dead_code)]
pub struct LuaConstantFoldingHelper;
/// Severity of a LuaExt diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LuaExtDiagSeverity {
    Note,
    Warning,
    Error,
}
/// A diagnostic message from a LuaExt pass.
#[derive(Debug, Clone)]
pub struct LuaExtDiagMsg {
    pub severity: LuaExtDiagSeverity,
    pub pass: String,
    pub message: String,
}
/// Dominator tree for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
/// Dependency graph for LuaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LuaExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
