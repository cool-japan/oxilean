use super::super::functions::LivenessInfo;
use super::super::functions::*;
use crate::lcnf::*;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};

/// Configuration for DSEExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEExtPassConfig {
    pub name: String,
    pub phase: DSEExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

/// Dominator tree for DSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

/// Worklist for DSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEX2Worklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// Bidirectional use-def / def-use chains for the IR.
///
/// - `defs[x]` = the let-value that defines `x` (there is exactly one in SSA).
/// - `uses[x]` = all variables that reference `x` in their definitions.
#[derive(Debug, Default)]
pub struct UseDefChain {
    /// Forward: variable → all variables that use it.
    pub uses: HashMap<LcnfVarId, Vec<LcnfVarId>>,
    /// Backward: variable → its defining expression.
    pub defs: HashMap<LcnfVarId, LcnfLetValue>,
}

/// Statistics produced by a single DSE run.
#[derive(Debug, Clone, Default)]
pub struct DSEReport {
    /// Total number of let-bindings analysed.
    pub stores_analyzed: usize,
    /// Number of dead let-bindings removed.
    pub dead_stores_eliminated: usize,
    /// Estimated bytes of IR saved (each removed binding ≈ 64 bytes).
    pub bytes_saved: usize,
}

/// Analysis cache for DSEExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct DSEExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

/// Statistics for DSEExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DSEExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

/// Pass execution phase for DSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DSEExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// Dependency graph for DSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

/// Configuration for DSEX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEX2PassConfig {
    pub name: String,
    pub phase: DSEX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

/// Liveness analysis for DSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DSEX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

/// Worklist for DSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

/// The main Dead Store Elimination pass.
pub struct DSEPass {
    pub(crate) config: DeadStoreConfig,
    pub(crate) report: DSEReport,
}

/// Configuration for the DSE pass.
#[derive(Debug, Clone, Default)]
pub struct DeadStoreConfig {
    /// Whether to perform alias analysis before marking stores dead.
    /// When `false`, any store to a variable that is live is kept.
    pub check_aliasing: bool,
    /// Aggressive mode: also eliminate stores whose values have no
    /// observable side effects (e.g. constructor allocations) even if
    /// aliasing is possible.
    pub aggressive: bool,
}

/// Metadata for a single let-binding ("store") in the IR.
#[derive(Debug, Clone)]
pub struct StoreInfo {
    /// The variable being written.
    pub var: LcnfVarId,
    /// The value being stored.
    pub value: LcnfLetValue,
    /// Whether this store is overwritten before any read.
    /// In SSA LCNF this is equivalent to the variable being dead.
    pub overwritten_before_read: bool,
}

/// Pass execution phase for DSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DSEX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}

/// Pass registry for DSEX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DSEX2PassRegistry {
    pub(crate) configs: Vec<DSEX2PassConfig>,
    pub(crate) stats: Vec<DSEX2PassStats>,
}

/// Pass registry for DSEExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DSEExtPassRegistry {
    pub(crate) configs: Vec<DSEExtPassConfig>,
    pub(crate) stats: Vec<DSEExtPassStats>,
}

/// Analysis cache for DSEX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct DSEX2Cache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

#[allow(dead_code)]
pub struct DSEConstantFoldingHelper;

/// Constant folding helper for DSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DSEX2ConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

/// Dominator tree for DSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEX2DomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSELivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, DSECacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSECacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEPassConfig {
    pub phase: DSEPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}

/// Dependency graph for DSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DSEX2DepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

/// Liveness analysis for DSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DSEExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

/// Constant folding helper for DSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DSEExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

/// Liveness information: the set of variables live at each let-binding
/// site (identified by the variable introduced at that site).
///
/// A variable `x` is *live* at a program point if there exists a path from
/// that point to a *use* of `x` that doesn't first re-define `x`.
#[derive(Debug, Default)]
pub struct LiveVariableInfo {
    /// `live_after[x]` = set of variables live *after* the binding of `x`.
    pub live_after: HashMap<LcnfVarId, HashSet<LcnfVarId>>,
    /// Variables live at the entry of the function.
    pub live_at_entry: HashSet<LcnfVarId>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DSEPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DSEPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

/// Statistics for DSEX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DSEX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

#[allow(dead_code)]
pub struct DSEPassRegistry {
    pub(crate) configs: Vec<DSEPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, DSEPassStats>,
}
