//! Type definitions

use super::super::functions::*;
use crate::lcnf::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

/// Pass registry for CSEX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CSEX2PassRegistry {
    pub(super) configs: Vec<CSEX2PassConfig>,
    pub(super) stats: Vec<CSEX2PassStats>,
}
/// Configuration for CSEExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEExtPassConfig {
    pub name: String,
    pub phase: CSEExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
/// An expression that is currently available at a program point.
///
/// "Available" means the expression has already been computed and its
/// result is held in `var_id`.  Any subsequent identical computation
/// can be replaced by a reference to `var_id`.
#[derive(Clone, Debug)]
pub struct AvailableExpr {
    /// The canonical key for this expression.
    pub key: ExprKey,
    /// The variable that already holds the computed result.
    pub var_id: LcnfVarId,
    /// Human-readable name hint of the defining binding.
    pub name_hint: String,
    /// Depth (let-binding nesting level) at which this was introduced.
    pub depth: usize,
}
/// Configuration for the CSE pass.
#[derive(Clone, Debug)]
pub struct CseConfig {
    /// Maximum estimated "size" of an expression (in sub-nodes) for it to
    /// be considered a CSE candidate.  Larger expressions are skipped to
    /// avoid blowing up the available set.
    pub max_expr_size: usize,
    /// Whether to attempt CSE across function call sites (requires purity
    /// analysis).
    pub track_calls: bool,
    /// Maximum number of candidates in the available set at any one point.
    pub max_candidates: usize,
    /// Known-pure function names (in addition to built-in constructors /
    /// projections).
    pub pure_functions: Vec<String>,
}
/// Pass execution phase for CSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CSEX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
/// Dominator tree for CSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEX2DomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
/// Dependency graph for CSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
/// Common Subexpression Elimination pass.
///
/// Usage:
/// ```
/// use oxilean_codegen::opt_cse::{CSEPass, CseConfig};
/// let mut pass = CSEPass::new(CseConfig::default());
/// // pass.run(&mut decls);
/// ```
pub struct CSEPass {
    pub(super) config: CseConfig,
    pub(super) report: CseReport,
}
/// Worklist for CSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEX2Worklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
/// Pass execution phase for CSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CSEExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, CSECacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
/// Statistics for CSEExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CSEExtPassStats {
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
pub struct CSEPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSELivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSECacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Constant folding helper for CSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CSEExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
/// Summary statistics produced by the CSE pass.
#[derive(Clone, Default, Debug)]
pub struct CseReport {
    /// Number of redundant computations found.
    pub expressions_found: usize,
    /// Number of redundant computations eliminated (replaced by variable).
    pub expressions_eliminated: usize,
    /// Number of let-bindings hoisted / shared.
    pub lets_hoisted: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEPassConfig {
    pub phase: CSEPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
#[allow(dead_code)]
pub struct CSEConstantFoldingHelper;
/// Liveness analysis for CSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CSEX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
/// Dependency graph for CSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEX2DepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
/// Global Value Numbering (GVN) table.
///
/// GVN assigns a *value number* to every expression; two expressions with
/// the same value number are guaranteed to produce the same result.  In
/// our setting the "value number" is simply the canonical `LcnfVarId` of
/// the first binding that computed the expression.
#[derive(Clone, Debug, Default)]
pub struct GvnTable {
    /// Map from expression key to its canonical representative variable.
    pub table: HashMap<ExprKey, LcnfVarId>,
    /// Number of distinct value numbers assigned.
    pub num_classes: usize,
}
/// Dominator tree for CSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
/// Analysis cache for CSEX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct CSEX2Cache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CSEPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
#[allow(dead_code)]
pub struct CSEPassRegistry {
    pub(super) configs: Vec<CSEPassConfig>,
    pub(super) stats: std::collections::HashMap<String, CSEPassStats>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
/// Pass registry for CSEExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CSEExtPassRegistry {
    pub(super) configs: Vec<CSEExtPassConfig>,
    pub(super) stats: Vec<CSEExtPassStats>,
}
/// Worklist for CSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
/// Configuration for CSEX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CSEX2PassConfig {
    pub name: String,
    pub phase: CSEX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
/// Statistics for CSEX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CSEX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
/// Liveness analysis for CSEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CSEExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
/// Constant folding helper for CSEX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CSEX2ConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
/// A canonical, hashable key derived from an LCNF let-bound value.
///
/// `ExprKey` normalizes commutative operations so that `Add(a,b)` and
/// `Add(b,a)` map to the same key, enabling CSE across re-ordered
/// argument lists.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ExprKey {
    /// Literal value.
    Lit(LcnfLit),
    /// Copy / identity: `let x = y`.
    Var(LcnfVarId),
    /// Constructor: `Ctor(name, tag, args)`.
    Ctor(String, u32, Vec<LcnfArg>),
    /// Projection: `proj_name.idx(var)`.
    Proj(String, u32, LcnfVarId),
    /// Function application: `f(args)` — only for pure calls.
    App(LcnfArg, Vec<LcnfArg>),
    /// Normalized commutative application (sorted args).
    CommApp(String, Vec<LcnfArg>),
}
/// Simplified dominator information for CSE.
///
/// In a tree-shaped ANF expression (which LCNF is), dominator
/// relationships follow the let-binding nesting directly.  We track
/// only the depth (nesting level) to know whether one binding is
/// guaranteed to dominate another.
#[derive(Clone, Debug, Default)]
pub struct DominatorTree {
    /// Map from `LcnfVarId` to the nesting depth at which it was bound.
    pub depth_map: HashMap<LcnfVarId, usize>,
}
/// Analysis cache for CSEExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct CSEExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
/// The set of expressions available at a given program point.
///
/// This is essentially the "out" set from classical data-flow analysis,
/// maintained as a simple hash map from expression key to the variable
/// holding the result.
#[derive(Clone, Default, Debug)]
pub struct AvailableSet {
    /// Map from expression key to (variable, depth) where the expression
    /// was first computed.
    pub(super) inner: HashMap<ExprKey, AvailableExpr>,
}
