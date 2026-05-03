use super::super::functions::*;
use crate::lcnf::*;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

/// Pass execution phase for M2RExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum M2RExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// Constant folding helper for M2RExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct M2RExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

/// Worklist for M2RX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RX2Worklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// Worklist for M2RExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// Summary of changes made by a single `Mem2Reg::run` invocation.
#[derive(Debug, Clone, Default)]
pub struct Mem2RegReport {
    /// Number of let-bindings successfully promoted to direct register uses.
    pub bindings_promoted: usize,
    /// Number of phi (join-point) nodes inserted during SSA construction.
    pub phi_nodes_inserted: usize,
}

/// Analysis cache for M2RX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct M2RX2Cache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

/// Configuration for the Mem2Reg promotion pass.
#[derive(Debug, Clone)]
pub struct Mem2RegConfig {
    /// Maximum number of phi nodes that may be inserted per function.
    /// Set to 0 to disable phi insertion entirely (only simple inlining).
    pub max_phi_nodes: usize,
    /// If `true`, only promote bindings that are provably single-use and
    /// never cross a case-branch boundary.  Safer but less aggressive.
    pub conservative: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct M2RPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RPassConfig {
    pub phase: M2RPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}

#[allow(dead_code)]
pub struct M2RPassRegistry {
    pub(crate) configs: Vec<M2RPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, M2RPassStats>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

/// Pass registry for M2RX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct M2RX2PassRegistry {
    pub(crate) configs: Vec<M2RX2PassConfig>,
    pub(crate) stats: Vec<M2RX2PassStats>,
}

/// Dominator tree for M2RX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RX2DomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

/// Statistics for M2RExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct M2RExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

/// Constant folding helper for M2RX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct M2RX2ConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

/// Pass registry for M2RExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct M2RExtPassRegistry {
    pub(crate) configs: Vec<M2RExtPassConfig>,
    pub(crate) stats: Vec<M2RExtPassStats>,
}

/// Configuration for M2RX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RX2PassConfig {
    pub name: String,
    pub phase: M2RX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

/// Analysis cache for M2RExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct M2RExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum M2RPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}

/// Information collected about a single let-binding during analysis.
#[derive(Debug, Clone)]
pub struct BindingInfo {
    /// Classification of the binding.
    pub(crate) kind: BindingKind,
    /// The value of the binding (for literal / FVar promotion).
    pub(crate) value: LcnfLetValue,
    /// Type of the binding.
    pub(crate) ty: LcnfType,
    /// Use count in the continuation.
    pub(crate) use_count: usize,
    /// Nesting depth at which this binding was introduced.
    pub(crate) depth: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, M2RCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

/// Memory-to-register promotion pass for LCNF.
pub struct Mem2Reg {
    pub(crate) config: Mem2RegConfig,
    pub(crate) report: Mem2RegReport,
}

/// Dominator tree for M2RExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

/// Dependency graph for M2RExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

/// A "dominance frontier" in the LCNF tree sense.
///
/// In LCNF, the IR is tree-structured.  A variable `x` defined in an
/// ancestor let-binding *dominates* all uses below it.  A dominance
/// frontier arises only at `Case` merge points, where different branches
/// may define different values for a variable.
///
/// We represent the frontier simply as the set of `LcnfVarId`s that are
/// defined in at least one branch of a case expression but used after the
/// case expression (i.e., in code that post-dominates the case).
#[derive(Debug, Default)]
pub struct DominanceFrontier {
    /// Variables that are live across at least one case-branch boundary.
    pub join_vars: HashSet<LcnfVarId>,
}

#[allow(dead_code)]
pub struct M2RConstantFoldingHelper;

/// Mutability status of a let-bound variable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingKind {
    /// Binding is immutable: assigned exactly once, never escape a case branch
    /// boundary.  Safe to promote unconditionally.
    Immutable,
    /// Binding crosses a case-branch boundary: requires phi-node insertion
    /// when promoted.
    MayJoin,
    /// Binding is the result of a Reset/Reuse operation and must NOT be
    /// promoted (memory semantics must be preserved).
    MemoryOp,
}

/// Liveness analysis for M2RExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct M2RExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}

/// Liveness analysis for M2RX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct M2RX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

/// Statistics for M2RX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct M2RX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

/// Dependency graph for M2RX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RX2DepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

/// Pass execution phase for M2RX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum M2RX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// Configuration for M2RExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct M2RExtPassConfig {
    pub name: String,
    pub phase: M2RExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
