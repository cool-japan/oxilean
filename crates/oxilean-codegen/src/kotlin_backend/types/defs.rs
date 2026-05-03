//! Type definitions

use super::super::functions::KOTLIN_KEYWORDS;
use super::super::functions::*;
use crate::lcnf::*;
use std::collections::HashSet;
use std::collections::{HashMap, VecDeque};

/// Analysis cache for KotlinExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct KotlinExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
/// A single branch in a `when` expression.
#[derive(Debug, Clone, PartialEq)]
pub struct KotlinWhenBranch {
    pub condition: KotlinExpr,
    pub body: KotlinExpr,
}
/// Configuration for KotlinExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KotlinExtPassConfig {
    pub name: String,
    pub phase: KotlinExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KtDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
/// Dominator tree for KotlinExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KotlinExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
/// Pass registry for KotlinExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct KotlinExtPassRegistry {
    pub(super) configs: Vec<KotlinExtPassConfig>,
    pub(super) stats: Vec<KotlinExtPassStats>,
}
/// Dependency graph for KotlinExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KotlinExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
/// Statistics for KotlinX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct KotlinX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
/// Kotlin statement AST.
#[derive(Debug, Clone, PartialEq)]
pub enum KotlinStmt {
    /// `val name: Type = expr`
    Val(String, KotlinType, KotlinExpr),
    /// `var name: Type = expr`
    Var(String, KotlinType, KotlinExpr),
    /// `name = expr`
    Assign(String, KotlinExpr),
    /// `return expr`
    Return(KotlinExpr),
    /// Expression statement
    Expr(KotlinExpr),
    /// `if (cond) { then } else { else_ }`
    If(KotlinExpr, Vec<KotlinStmt>, Vec<KotlinStmt>),
    /// `when (expr) { branches each as (cond, stmts) } else { default }`
    When(
        KotlinExpr,
        Vec<(KotlinExpr, Vec<KotlinStmt>)>,
        Vec<KotlinStmt>,
    ),
}
/// Constant folding helper for KotlinExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct KotlinExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
/// Worklist for KotlinX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KotlinX2Worklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct KtPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KtLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
/// Statistics for KotlinExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct KotlinExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
/// Liveness analysis for KotlinExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct KotlinExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KtPassConfig {
    pub phase: KtPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
/// Pass execution phase for KotlinExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KotlinExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
/// Kotlin expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum KotlinExpr {
    /// Variable reference: `foo`
    Var(String),
    /// Literal value
    Lit(KotlinLit),
    /// Function call: `f(a, b)`
    Call(Box<KotlinExpr>, Vec<KotlinExpr>),
    /// Binary operation: `a + b`
    BinOp(String, Box<KotlinExpr>, Box<KotlinExpr>),
    /// Member access: `obj.field`
    Member(Box<KotlinExpr>, String),
    /// Index access: `arr[i]`
    Index(Box<KotlinExpr>, Box<KotlinExpr>),
    /// Unary operation: `!x`
    Unary(String, Box<KotlinExpr>),
    /// Lambda: `{ x, y -> body }`
    Lambda(Vec<String>, Box<KotlinExpr>),
    /// When expression: `when (scrutinee) { branches... else -> default }`
    When(
        Box<KotlinExpr>,
        Vec<KotlinWhenBranch>,
        Option<Box<KotlinExpr>>,
    ),
    /// Elvis operator: `a ?: b`
    Elvis(Box<KotlinExpr>, Box<KotlinExpr>),
}
/// A Kotlin `data class` definition.
#[derive(Debug, Clone)]
pub struct KotlinDataClass {
    pub name: String,
    pub fields: Vec<(String, KotlinType)>,
}
/// Worklist for KotlinExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KotlinExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
/// A Kotlin function definition.
#[derive(Debug, Clone)]
pub struct KotlinFunc {
    pub name: String,
    pub params: Vec<(String, KotlinType)>,
    pub return_type: KotlinType,
    pub body: Vec<KotlinStmt>,
    pub is_tailrec: bool,
}
/// Kotlin literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum KotlinLit {
    Int(i64),
    Long(i64),
    Bool(bool),
    Str(String),
    Null,
}
/// Analysis cache for KotlinX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct KotlinX2Cache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
/// Kotlin type representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KotlinType {
    /// `Int`
    KtInt,
    /// `Long`
    KtLong,
    /// `Boolean`
    KtBool,
    /// `String`
    KtString,
    /// `Unit`
    KtUnit,
    /// `Any`
    KtAny,
    /// `List<T>`
    KtList(Box<KotlinType>),
    /// `Pair<A, B>`
    KtPair(Box<KotlinType>, Box<KotlinType>),
    /// `(P0, P1, ...) -> R`
    KtFunc(Vec<KotlinType>, Box<KotlinType>),
    /// `T?`
    KtNullable(Box<KotlinType>),
    /// Named class / data class
    KtObject(String),
}
/// Kotlin code generation backend.
pub struct KotlinBackend {
    /// Counter for fresh temporary variable names.
    pub(super) var_counter: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KtDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
/// Pass registry for KotlinX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct KotlinX2PassRegistry {
    pub(super) configs: Vec<KotlinX2PassConfig>,
    pub(super) stats: Vec<KotlinX2PassStats>,
}
/// Configuration for KotlinX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KotlinX2PassConfig {
    pub name: String,
    pub phase: KotlinX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
/// Dependency graph for KotlinX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KotlinX2DepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
/// Dominator tree for KotlinX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KotlinX2DomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
/// Liveness analysis for KotlinX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct KotlinX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum KtPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KtCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A complete Kotlin compilation unit (file).
#[derive(Debug, Clone)]
pub struct KotlinModule {
    pub package: String,
    pub imports: Vec<String>,
    pub data_classes: Vec<KotlinDataClass>,
    pub funs: Vec<KotlinFunc>,
}
/// Pass execution phase for KotlinX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KotlinX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KtWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
/// Constant folding helper for KotlinX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct KotlinX2ConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
#[allow(dead_code)]
pub struct KtConstantFoldingHelper;
#[allow(dead_code)]
pub struct KtPassRegistry {
    pub(super) configs: Vec<KtPassConfig>,
    pub(super) stats: std::collections::HashMap<String, KtPassStats>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct KtAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, KtCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
