//! Type definitions

use super::super::functions::*;
use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfType, LcnfVarId};
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RADominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
/// Pass execution phase for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RAExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
/// Chaitin-Briggs graph coloring register allocator.
///
/// Phases:
/// 1. Build interference graph from live intervals.
/// 2. Simplify: repeatedly remove nodes with degree < k (available colors).
/// 3. Coalesce: merge copy-related nodes that don't interfere.
/// 4. Spill: if no simplifiable node, mark highest-cost node for potential spill.
/// 5. Select: pop nodes off the stack and assign colors.
#[derive(Debug)]
pub struct GraphColoringAllocator {
    /// Number of available colors (physical registers).
    pub num_colors: usize,
    /// Physical registers available.
    pub phys_regs: Vec<PhysReg>,
    /// Current interference graph (mutated during simplification).
    pub graph: InterferenceGraph,
    /// Stack of removed nodes (in simplification order).
    pub(super) simplify_stack: Vec<LcnfVarId>,
    /// Nodes marked as potential spills.
    pub(super) potential_spills: Vec<LcnfVarId>,
    /// Number of coalescing merges performed.
    pub coalesced_count: usize,
    /// Number of simplification iterations.
    pub iterations: usize,
}
/// A feature flag set for RegAlloc capabilities.
#[derive(Debug, Clone, Default)]
pub struct RegAllocFeatures {
    pub(super) flags: std::collections::HashSet<String>,
}
/// A candidate for spilling, ranked by cost.
#[derive(Debug, Clone)]
pub struct SpillCandidate {
    /// The virtual register to spill.
    pub vreg: LcnfVarId,
    /// Estimated spill cost (frequency * size).
    pub cost: f64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, RACacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
/// Summary statistics from register allocation.
#[derive(Debug, Clone, Default)]
pub struct RegAllocReport {
    /// Number of virtual registers successfully allocated to physical registers.
    pub vregs_allocated: usize,
    /// Number of virtual registers that were spilled to the stack.
    pub spills: usize,
    /// Number of copy pairs coalesced (eliminates move instructions).
    pub copies_coalesced: usize,
    /// Number of coloring / simplification iterations performed.
    pub coloring_iterations: usize,
    /// Total stack frame size used for spill slots (bytes).
    pub stack_frame_bytes: u32,
    /// Number of functions processed.
    pub functions_processed: usize,
}
/// Collects RegAlloc diagnostics.
#[derive(Debug, Default)]
pub struct RegAllocDiagCollector {
    pub(super) msgs: Vec<RegAllocDiagMsg>,
}
/// Register allocation pass.
///
/// Supports both linear scan and graph coloring algorithms.
#[derive(Debug)]
pub struct RegAllocPass {
    /// Physical register bank.
    pub phys_regs: Vec<PhysReg>,
    /// Whether to use graph coloring (Chaitin-Briggs) instead of linear scan.
    pub use_graph_coloring: bool,
    /// Accumulated report.
    pub(super) report: RegAllocReport,
    /// Allocation results per function (function name → allocation).
    pub allocations: HashMap<String, Allocation>,
}
/// The register class (hardware register file) of a physical register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegClass {
    /// General-purpose integer registers (e.g. rax, rbx, …).
    Integer,
    /// Floating-point / SSE / AVX registers (e.g. xmm0, …).
    Float,
    /// SIMD / vector registers (e.g. ymm0, zmm0, …).
    Vector,
    /// Predicate / condition registers (e.g. ARM p0, x86 k0, …).
    Predicate,
}
/// The result of register allocation.
#[derive(Debug, Clone, Default)]
pub struct Allocation {
    /// Maps virtual register IDs to physical registers.
    pub reg_map: HashMap<LcnfVarId, PhysReg>,
    /// Virtual registers that were spilled to the stack.
    pub spills: Vec<SpillSlot>,
    /// Next available stack offset.
    pub(super) next_stack_offset: u32,
}
/// A diagnostic message from a RegAlloc pass.
#[derive(Debug, Clone)]
pub struct RegAllocDiagMsg {
    pub severity: RegAllocDiagSeverity,
    pub pass: String,
    pub message: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RACacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Dependency graph for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
/// Pipeline profiler for RegAlloc.
#[derive(Debug, Default)]
pub struct RegAllocProfiler {
    pub(super) timings: Vec<RegAllocPassTiming>,
}
/// Emission statistics for RegAlloc.
#[derive(Debug, Clone, Default)]
pub struct RegAllocEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
/// An undirected interference graph for register allocation.
///
/// Nodes are virtual registers; an edge (u, v) means u and v have
/// overlapping live ranges and therefore cannot share a physical register.
#[derive(Debug, Clone, Default)]
pub struct InterferenceGraph {
    /// Adjacency lists.
    pub edges: HashMap<LcnfVarId, HashSet<LcnfVarId>>,
    /// All nodes (vregs) in the graph.
    pub nodes: HashSet<LcnfVarId>,
    /// Move-related pairs: pairs of vregs connected by copy instructions
    /// that are candidates for coalescing.
    pub move_pairs: Vec<(LcnfVarId, LcnfVarId)>,
}
/// Heuristic freshness key for RegAlloc incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegAllocIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RALivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
/// Liveness analysis for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RAExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
/// A generic key-value configuration store for RegAlloc.
#[derive(Debug, Clone, Default)]
pub struct RegAllocConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
}
/// Severity of a RegAlloc diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RegAllocDiagSeverity {
    Note,
    Warning,
    Error,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
/// A physical register on the target architecture.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysReg {
    /// Unique numeric ID of this register.
    pub id: u32,
    /// Human-readable name (e.g. "rax", "f0", "xmm3").
    pub name: String,
    /// The register class.
    pub class: RegClass,
}
/// A monotonically increasing ID generator for RegAlloc.
#[derive(Debug, Default)]
pub struct RegAllocIdGen {
    pub(super) next: u32,
}
/// A stack spill slot for a virtual register.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpillSlot {
    /// The spilled virtual register.
    pub vreg: LcnfVarId,
    /// Byte offset on the stack frame.
    pub offset: u32,
    /// Size of the slot in bytes.
    pub size: u32,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAPassConfig {
    pub phase: RAPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
/// A text buffer for building RegAlloc output source code.
#[derive(Debug, Default)]
pub struct RegAllocSourceBuffer {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
/// Tracks declared names for RegAlloc scope analysis.
#[derive(Debug, Default)]
pub struct RegAllocNameScope {
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<RegAllocNameScope>>,
}
/// Pass-timing record for RegAlloc profiler.
#[derive(Debug, Clone)]
pub struct RegAllocPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RAPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
#[allow(dead_code)]
pub struct RAConstantFoldingHelper;
/// A virtual register (pre-allocation).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VirtualReg {
    /// Unique ID matching the corresponding `LcnfVarId`.
    pub id: u32,
    /// The LCNF type of this register's value.
    pub ty: LcnfType,
    /// An optional hint towards a preferred physical register.
    pub hint: Option<PhysReg>,
}
/// The live interval [start, end) of a virtual register.
///
/// `start` is the instruction index where the vreg is first defined,
/// `end` is one past the last use.
#[derive(Debug, Clone, PartialEq)]
pub struct LiveInterval {
    /// The virtual register this interval belongs to.
    pub vreg: LcnfVarId,
    /// Inclusive start (def point).
    pub start: u32,
    /// Exclusive end (last use + 1).
    pub end: u32,
    /// All use points (instruction indices where this vreg is read).
    pub uses: Vec<u32>,
    /// All def points (instruction indices where this vreg is written).
    pub defs: Vec<u32>,
    /// Spill weight: higher = more costly to spill.
    pub spill_weight: f64,
}
#[allow(dead_code)]
pub struct RAPassRegistry {
    pub(super) configs: Vec<RAPassConfig>,
    pub(super) stats: std::collections::HashMap<String, RAPassStats>,
}
/// Configuration for RAExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAExtPassConfig {
    pub name: String,
    pub phase: RAExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
/// Worklist for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
/// A version tag for RegAlloc output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegAllocVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum RAPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
/// Statistics for RAExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RAExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
/// Pass registry for RAExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct RAExtPassRegistry {
    pub(super) configs: Vec<RAExtPassConfig>,
    pub(super) stats: Vec<RAExtPassStats>,
}
/// Analysis cache for RAExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct RAExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
/// Constant folding helper for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct RAExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
/// Linear scan register allocator.
///
/// Algorithm (Poletto & Sarkar, 1999):
/// 1. Sort live intervals by start point.
/// 2. For each interval:
///    a. Expire all intervals that end before this one starts → free their registers.
///    b. If a physical register is free, assign it.
///    c. Otherwise spill the interval with the furthest end point (or this interval).
#[derive(Debug)]
pub struct LinearScanAllocator {
    /// Available physical registers.
    pub phys_regs: Vec<PhysReg>,
    /// Number of spills performed.
    pub spill_count: usize,
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in RegAlloc).
#[derive(Debug)]
pub struct RegAllocEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RADepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
/// Dominator tree for RAExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RAExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
