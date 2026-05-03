use super::super::functions::SpecKey;
use super::super::functions::*;
use crate::lcnf::{
    LcnfAlt, LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfModule, LcnfParam, LcnfType,
    LcnfVarId,
};
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

/// Configuration for SpecTExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecTExtPassConfig {
    pub name: String,
    pub phase: SpecTExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

/// Worklist for SpecTExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecTExtWorklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

/// Dominator tree for SpecTExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecTExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

/// Dominator tree for SpecTX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecTX2DomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}

/// Type specialization pass over a whole `LcnfModule`.
pub struct TypeSpecializer {
    /// Configuration for this pass.
    pub config: TypeSpecConfig,
    /// Report from the most recent `run` call.
    pub report: TypeSpecReport,
}

/// Configuration for SpecTX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecTX2PassConfig {
    pub name: String,
    pub phase: SpecTX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}

/// Worklist for SpecTX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecTX2Worklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}

#[allow(dead_code)]
pub struct OSTPassRegistry {
    pub(crate) configs: Vec<OSTPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, OSTPassStats>,
}

/// Dependency graph for SpecTExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecTExtDepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

/// Analysis cache for SpecTExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct SpecTExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

/// Pass registry for SpecTX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SpecTX2PassRegistry {
    pub(crate) configs: Vec<SpecTX2PassConfig>,
    pub(crate) stats: Vec<SpecTX2PassStats>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OSTAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, OSTCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OSTCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}

/// Constant folding helper for SpecTX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SpecTX2ConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OSTLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum OSTPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}

/// Liveness analysis for SpecTX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SpecTX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

/// Dependency graph for SpecTX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecTX2DepGraph {
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OSTDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}

/// Statistics for SpecTX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SpecTX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

/// Configuration for the type-specialization pass.
#[derive(Debug, Clone)]
pub struct TypeSpecConfig {
    /// Maximum total number of specialized functions to generate per module.
    pub max_specializations: usize,
    /// Minimum number of call sites with the same type arguments before
    /// specialization is triggered.
    pub min_call_count: usize,
}

/// Analysis cache for SpecTX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct SpecTX2Cache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}

/// Pass execution phase for SpecTX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpecTX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// Constant folding helper for SpecTExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SpecTExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}

#[allow(dead_code)]
pub struct OSTConstantFoldingHelper;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OSTPassConfig {
    pub phase: OSTPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}

/// Statistics collected during a type-specialization pass.
#[derive(Debug, Clone, Default)]
pub struct TypeSpecReport {
    /// Number of new specialized function variants generated.
    pub functions_specialized: usize,
    /// Number of call sites rewritten to use a specialized variant.
    pub call_sites_updated: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OSTWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}

/// Pass registry for SpecTExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SpecTExtPassRegistry {
    pub(crate) configs: Vec<SpecTExtPassConfig>,
    pub(crate) stats: Vec<SpecTExtPassStats>,
}

/// Liveness analysis for SpecTExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SpecTExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OSTDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}

/// Pass execution phase for SpecTExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpecTExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}

/// Statistics for SpecTExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SpecTExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OSTPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
