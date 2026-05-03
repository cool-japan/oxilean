//! Type definitions

use super::super::functions::ELIXIR_RUNTIME;
use super::super::functions::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

/// An Elixir module (maps to a `defmodule` block).
#[derive(Debug, Clone)]
pub struct ElixirModule {
    /// Fully-qualified module name, e.g. `"MyApp.Math"`
    pub name: String,
    /// Functions defined in this module
    pub functions: Vec<ElixirFunction>,
    /// Modules listed in `use` directives
    pub use_modules: Vec<String>,
    /// `import` directives
    pub imports: Vec<String>,
    /// Module-level attributes, e.g. `@moduledoc`
    pub attributes: HashMap<String, String>,
}
/// Liveness analysis for ElixirX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ElixirX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
/// A single Elixir function definition.
///
/// Each function can have multiple clauses for pattern-matched dispatch.
#[derive(Debug, Clone)]
pub struct ElixirFunction {
    /// Unqualified function name
    pub name: String,
    /// Number of parameters (arity)
    pub arity: usize,
    /// Clauses: (patterns, optional guard, body)
    pub clauses: Vec<(Vec<ElixirExpr>, Option<ElixirExpr>, ElixirExpr)>,
    /// Whether this function is private (`defp`)
    pub is_private: bool,
    /// Inline documentation string
    pub doc: Option<String>,
}
/// Pass execution phase for ElixirExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElixirExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
/// Dominator tree for ElixirX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElixirX2DomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
/// Dependency graph for ElixirExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElixirExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
/// Constant folding helper for ElixirX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ElixirX2ConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ElxPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
/// Dominator tree for ElixirExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElixirExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
/// An Elixir expression node used for code generation.
#[derive(Debug, Clone)]
pub enum ElixirExpr {
    /// `:atom` literal
    Atom(String),
    /// Integer literal
    Integer(i64),
    /// Float literal
    Float(f64),
    /// Binary (string) literal: `"hello"`
    Binary(String),
    /// List literal: `[a, b, c]`
    List(Vec<ElixirExpr>),
    /// Tuple literal: `{a, b}`
    Tuple(Vec<ElixirExpr>),
    /// Map literal: `%{key => val}`
    Map(Vec<(ElixirExpr, ElixirExpr)>),
    /// Variable reference
    Var(String),
    /// Function call: `Module.func(args)` or `func(args)`
    FuncCall(String, Vec<ElixirExpr>),
    /// Match expression: `pattern = expr`
    Match(Box<ElixirExpr>, Box<ElixirExpr>),
    /// Case expression: `case expr do clauses end`
    Case(Box<ElixirExpr>, Vec<(ElixirExpr, ElixirExpr)>),
    /// Anonymous function: `fn params -> body end`
    Lambda(Vec<String>, Box<ElixirExpr>),
    /// Pipe expression: `lhs |> rhs`
    Pipe(Box<ElixirExpr>, Box<ElixirExpr>),
    /// Block of expressions (the last is the value)
    Block(Vec<ElixirExpr>),
    /// Conditional: `if cond, do: then_branch, else: else_branch`
    If(Box<ElixirExpr>, Box<ElixirExpr>, Box<ElixirExpr>),
    /// Binary operator: `lhs op rhs`
    BinOp(String, Box<ElixirExpr>, Box<ElixirExpr>),
    /// String interpolation: `"prefix#{expr}suffix"`
    Interpolation(Vec<ElixirStringPart>),
    /// `nil` literal
    Nil,
    /// Boolean literal
    Bool(bool),
    /// Struct literal: `%StructName{field: val}`
    Struct(String, Vec<(String, ElixirExpr)>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElxCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElxPassConfig {
    pub phase: ElxPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
/// Configuration for ElixirExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElixirExtPassConfig {
    pub name: String,
    pub phase: ElixirExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
/// Statistics for ElixirExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ElixirExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
/// Worklist for ElixirExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElixirExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
/// Worklist for ElixirX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElixirX2Worklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
/// A part of a string interpolation expression.
#[derive(Debug, Clone)]
pub enum ElixirStringPart {
    Literal(String),
    Expr(ElixirExpr),
}
/// Constant folding helper for ElixirExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ElixirExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
/// Pass execution phase for ElixirX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElixirX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElxLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
/// Statistics for ElixirX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ElixirX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
/// Elixir code generation backend.
pub struct ElixirBackend {
    /// Indentation string (default: two spaces)
    pub(super) indent_str: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ElxPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
/// Dependency graph for ElixirX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElixirX2DepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
#[allow(dead_code)]
pub struct ElxPassRegistry {
    pub(super) configs: Vec<ElxPassConfig>,
    pub(super) stats: std::collections::HashMap<String, ElxPassStats>,
}
/// Pass registry for ElixirX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ElixirX2PassRegistry {
    pub(super) configs: Vec<ElixirX2PassConfig>,
    pub(super) stats: Vec<ElixirX2PassStats>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElxDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
/// Liveness analysis for ElixirExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ElixirExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElxAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, ElxCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
/// Pass registry for ElixirExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ElixirExtPassRegistry {
    pub(super) configs: Vec<ElixirExtPassConfig>,
    pub(super) stats: Vec<ElixirExtPassStats>,
}
/// Analysis cache for ElixirExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ElixirExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElxWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
/// Analysis cache for ElixirX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ElixirX2Cache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElxDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
/// Configuration for ElixirX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElixirX2PassConfig {
    pub name: String,
    pub phase: ElixirX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
#[allow(dead_code)]
pub struct ElxConstantFoldingHelper;
