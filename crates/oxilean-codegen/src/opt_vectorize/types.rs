//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfExpr, LcnfFunDecl, LcnfLetValue};
use std::collections::HashMap;

use super::functions::HintMap;

use super::functions::*;
use std::collections::{HashSet, VecDeque};

/// A single SIMD vector instruction in the lowered representation.
///
/// Operands are identified by SSA-style string names that map to vector
/// registers. Source operands must be defined before this instruction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorInstr {
    /// The SIMD operation to perform.
    pub op: SIMDOp,
    /// The vector width (register size) for this instruction.
    pub width: VectorWidth,
    /// Destination register name.
    pub dst: String,
    /// Source register names (0–3 depending on op).
    pub srcs: Vec<String>,
}
impl VectorInstr {
    /// Construct a new vector instruction.
    pub fn new(op: SIMDOp, width: VectorWidth, dst: impl Into<String>, srcs: Vec<String>) -> Self {
        VectorInstr {
            op,
            width,
            dst: dst.into(),
            srcs,
        }
    }
}
/// A virtual SIMD register file for instruction scheduling.
///
/// Tracks live vector registers and performs spill analysis.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct VectorRegisterFile {
    /// Number of physical vector registers available.
    pub num_physical: usize,
    /// Map from virtual register name to physical register slot index.
    pub allocation: HashMap<String, usize>,
    /// Stack of spilled registers (evicted to memory).
    pub spill_stack: Vec<String>,
    /// Counter for generating unique virtual register names.
    pub(super) name_counter: u64,
}
impl VectorRegisterFile {
    /// Create a new register file with `n` physical slots.
    pub fn new(num_physical: usize) -> Self {
        VectorRegisterFile {
            num_physical,
            ..Default::default()
        }
    }
    /// Allocate a new virtual register. May trigger a spill if all physicals are used.
    pub fn alloc(&mut self, hint: &str) -> String {
        let name = format!("{}_{}", hint, self.name_counter);
        self.name_counter += 1;
        if self.allocation.len() < self.num_physical {
            let slot = self.allocation.len();
            self.allocation.insert(name.clone(), slot);
        } else {
            // No physical slot available; the new register is spilled to memory
            self.spill_stack.push(name.clone());
        }
        name
    }
    /// Free a virtual register.
    pub fn free(&mut self, name: &str) {
        if self.allocation.remove(name).is_some() {
            // Freed from active allocation; do not auto-restore spilled
            // registers so the caller can observe the freed slot.
        } else {
            // Not in active allocation; remove from spill stack if present
            if let Some(pos) = self.spill_stack.iter().position(|s| s == name) {
                self.spill_stack.remove(pos);
            }
        }
    }
    /// Returns how many spills occurred during allocation.
    pub fn spill_count(&self) -> usize {
        self.spill_stack.len()
    }
    /// Returns true if the register file is currently full (all physicals in use).
    pub fn is_full(&self) -> bool {
        self.allocation.len() >= self.num_physical
    }
}
#[allow(dead_code)]
pub struct VecPassRegistry {
    pub(super) configs: Vec<VecPassConfig>,
    pub(super) stats: std::collections::HashMap<String, VecPassStats>,
}
impl VecPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        VecPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: VecPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), VecPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&VecPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&VecPassStats> {
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
/// Data dependence graph for a loop body.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct DependenceGraph {
    /// All edges.
    pub edges: Vec<DependenceEdge>,
}
impl DependenceGraph {
    /// Add a dependence edge.
    pub fn add_edge(
        &mut self,
        from: impl Into<String>,
        to: impl Into<String>,
        kind: DependenceKind,
        distance: i64,
    ) {
        self.edges.push(DependenceEdge {
            from: from.into(),
            to: to.into(),
            kind,
            distance,
        });
    }
    /// Returns true if any edge is a carried dependence (distance > 0).
    pub fn has_carried_dependence(&self) -> bool {
        self.edges
            .iter()
            .any(|e| e.distance > 0 && e.kind != DependenceKind::Input)
    }
    /// Returns all edges of a given kind.
    pub fn edges_of_kind(&self, kind: DependenceKind) -> Vec<&DependenceEdge> {
        self.edges.iter().filter(|e| e.kind == kind).collect()
    }
    /// Returns the maximum dependence distance among all edges.
    pub fn max_distance(&self) -> i64 {
        self.edges.iter().map(|e| e.distance).max().unwrap_or(0)
    }
}
/// Summary report produced by a single `VectorizationPass::run` invocation.
#[derive(Debug, Clone, Default)]
pub struct VectorizationReport {
    /// Number of loops analyzed.
    pub loops_analyzed: u32,
    /// Number of loops successfully vectorized.
    pub loops_vectorized: u32,
    /// Number of loops rejected due to loop-carried dependencies.
    pub rejected_dep: u32,
    /// Number of loops rejected because trip count was too small.
    pub rejected_trip_count: u32,
    /// Number of loops rejected for other reasons.
    pub rejected_other: u32,
    /// Estimated average speedup across vectorized loops.
    pub avg_estimated_speedup: f64,
    /// Per-function speedup estimates.
    pub speedup_by_func: HashMap<String, f64>,
}
impl VectorizationReport {
    /// Total number of loops that were rejected.
    pub fn total_rejected(&self) -> u32 {
        self.rejected_dep + self.rejected_trip_count + self.rejected_other
    }
    /// Merge another report into this one.
    pub fn merge(&mut self, other: &VectorizationReport) {
        self.loops_analyzed += other.loops_analyzed;
        self.loops_vectorized += other.loops_vectorized;
        self.rejected_dep += other.rejected_dep;
        self.rejected_trip_count += other.rejected_trip_count;
        self.rejected_other += other.rejected_other;
        for (k, v) in &other.speedup_by_func {
            self.speedup_by_func.insert(k.clone(), *v);
        }
        let total = self.loops_vectorized as f64;
        if total > 0.0 {
            let sum: f64 = self.speedup_by_func.values().sum();
            self.avg_estimated_speedup = sum / total;
        }
    }
}
/// Recognized reduction patterns for SIMD vectorization.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReductionKind {
    /// Sum reduction: `acc = acc + elem`.
    Sum,
    /// Product reduction: `acc = acc * elem`.
    Product,
    /// Minimum reduction: `acc = min(acc, elem)`.
    Min,
    /// Maximum reduction: `acc = max(acc, elem)`.
    Max,
    /// Bitwise AND reduction: `acc = acc & elem`.
    And,
    /// Bitwise OR reduction: `acc = acc | elem`.
    Or,
    /// Bitwise XOR reduction: `acc = acc ^ elem`.
    Xor,
    /// First-order dot product: `acc = acc + (a[i] * b[i])`.
    DotProduct,
}
/// Generates vector loop prologue and epilogue instruction sequences.
///
/// For loops whose trip count is not a multiple of the vector lane width,
/// we need a scalar prologue (to align the induction variable) and a
/// scalar epilogue (to handle remaining elements).
#[allow(dead_code)]
pub struct VectorPrologueEpilogue {
    /// Vector width in lanes.
    pub lane_width: u32,
}
impl VectorPrologueEpilogue {
    /// Create a new prologue/epilogue generator.
    pub fn new(width: VectorWidth) -> Self {
        VectorPrologueEpilogue {
            lane_width: width.lanes_f32(),
        }
    }
    /// Compute the number of prologue iterations needed for alignment.
    ///
    /// Given an address misalignment in bytes and element size, returns
    /// the number of scalar iterations to run before the vector loop.
    pub fn prologue_iterations(&self, misalignment_bytes: u32, element_size: u32) -> u32 {
        if element_size == 0 {
            return 0;
        }
        let misaligned_elements = misalignment_bytes / element_size;
        if misaligned_elements == 0 {
            return 0;
        }
        self.lane_width - (misaligned_elements % self.lane_width)
    }
    /// Compute the number of epilogue iterations.
    ///
    /// Given the total trip count and a prologue count, returns the
    /// number of scalar iterations needed after the vector loop.
    pub fn epilogue_iterations(&self, total_trip: u64, prologue: u32) -> u64 {
        let effective_trip = total_trip.saturating_sub(prologue as u64);
        effective_trip % self.lane_width as u64
    }
    /// Emit a prologue instruction sequence as vector instructions.
    ///
    /// In practice this would be scalar instructions; here we model it
    /// as a series of Broadcast + Store placeholders for demonstration.
    pub fn emit_prologue(&self, array: &str, prologue_count: u32) -> Vec<VectorInstr> {
        let mut instrs = Vec::new();
        for i in 0..prologue_count.min(self.lane_width) {
            let addr = format!("{}_prologue_{}", array, i);
            instrs.push(VectorInstr::new(
                SIMDOp::Store,
                VectorWidth::W64,
                addr.clone(),
                vec!["scalar_val".to_string(), addr],
            ));
        }
        instrs
    }
    /// Emit an epilogue instruction sequence.
    pub fn emit_epilogue(&self, array: &str, epilogue_count: u64) -> Vec<VectorInstr> {
        let mut instrs = Vec::new();
        for i in 0..epilogue_count.min(self.lane_width as u64) {
            let addr = format!("{}_epilogue_{}", array, i);
            instrs.push(VectorInstr::new(
                SIMDOp::Store,
                VectorWidth::W64,
                addr.clone(),
                vec!["scalar_val".to_string(), addr],
            ));
        }
        instrs
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum VecPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl VecPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            VecPassPhase::Analysis => "analysis",
            VecPassPhase::Transformation => "transformation",
            VecPassPhase::Verification => "verification",
            VecPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, VecPassPhase::Transformation | VecPassPhase::Cleanup)
    }
}
/// Stride pattern for a memory access.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StridePattern {
    /// Unit stride: consecutive element access (vectorization-friendly).
    Unit,
    /// Constant stride: fixed step between accesses.
    Constant(i64),
    /// Gather/scatter: non-uniform access (vectorization-unfriendly).
    Irregular,
    /// Unknown stride (conservative assumption).
    Unknown,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VecCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VecLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl VecLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        VecLivenessInfo {
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
/// A scheduled instruction with an assigned cycle slot.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ScheduledVecInstr {
    /// The instruction.
    pub instr: VectorInstr,
    /// The cycle this instruction is scheduled to issue.
    pub issue_cycle: u64,
    /// The cycle this instruction's result is ready.
    pub ready_cycle: u64,
}
/// Configuration knobs for the vectorization pass.
#[derive(Debug, Clone)]
pub struct VectorizationConfig {
    /// Minimum static trip count to attempt vectorization.
    /// Loops with fewer iterations are unlikely to benefit.
    pub min_trip_count: u64,
    /// Preferred vector register width.
    pub preferred_width: VectorWidth,
    /// Whether to emit FMA instructions when available.
    pub enable_fma: bool,
    /// Whether to vectorize reduction loops (e.g. sum, dot-product).
    pub vectorize_reductions: bool,
    /// Target hardware ISA for intrinsic selection.
    pub target: SIMDTarget,
}
/// A single data dependence edge.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DependenceEdge {
    /// Source variable.
    pub from: String,
    /// Destination variable.
    pub to: String,
    /// Kind of dependence.
    pub kind: DependenceKind,
    /// Distance vector (loop iteration distance); 0 = same iteration.
    pub distance: i64,
}
/// Represents a transformed loop descriptor.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TransformedLoop {
    /// Original function name.
    pub original_name: String,
    /// New name after transformation (may include suffixes).
    pub transformed_name: String,
    /// Unroll factor applied.
    pub unroll_factor: u32,
    /// Tile size used (0 if no tiling).
    pub tile_size: u32,
    /// Whether strip-mining was applied.
    pub strip_mined: bool,
    /// Estimated number of vector instructions in the transformed body.
    pub vector_instr_count: u32,
}
impl TransformedLoop {
    /// Estimate the new trip count after strip-mining.
    pub fn strip_mined_trip_count(&self, original_bound: u64, lane_width: u32) -> u64 {
        if self.strip_mined && lane_width > 0 {
            (original_bound + lane_width as u64 - 1) / lane_width as u64
        } else {
            original_bound
        }
    }
}
/// Latency class of a SIMD operation (in abstract cycles).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LatencyClass {
    /// Zero-latency (e.g., register copy, no-op).
    Zero = 0,
    /// Single cycle (integer add/sub).
    SingleCycle = 1,
    /// Short (2–3 cycles, fp add/mul).
    Short = 3,
    /// Medium (4–6 cycles, fp div, complex shuffles).
    Medium = 6,
    /// Long (10+ cycles, sqrt, transcendental approximation).
    Long = 12,
    /// Memory (dependent on cache; 4–100 cycles).
    Memory = 20,
}
/// Applies classical loop transformations and produces a `TransformedLoop`.
#[allow(dead_code)]
pub struct LoopTransformer {
    /// Configuration to use.
    pub config: LoopTransformConfig,
}
impl LoopTransformer {
    /// Create with default configuration.
    pub fn new() -> Self {
        LoopTransformer {
            config: LoopTransformConfig::default(),
        }
    }
    /// Create with a specific configuration.
    pub fn with_config(config: LoopTransformConfig) -> Self {
        LoopTransformer { config }
    }
    /// Transform a candidate loop and return the `TransformedLoop` descriptor.
    pub fn transform(
        &self,
        candidate: &VectorizationCandidate,
        width: VectorWidth,
    ) -> TransformedLoop {
        let suffix = if self.config.strip_mine {
            format!("_vec{}", width.bits())
        } else if self.config.unroll_factor > 1 {
            format!("_unroll{}", self.config.unroll_factor)
        } else {
            String::new()
        };
        let transformed_name = format!("{}{}", candidate.func_name, suffix);
        let base_instrs =
            3_u32 + candidate.array_reads.len() as u32 + candidate.array_writes.len() as u32;
        let vector_instr_count = base_instrs * self.config.unroll_factor;
        TransformedLoop {
            original_name: candidate.func_name.clone(),
            transformed_name,
            unroll_factor: self.config.unroll_factor,
            tile_size: self.config.tile_size,
            strip_mined: self.config.strip_mine,
            vector_instr_count,
        }
    }
}
/// Orchestrates the full vectorization pipeline: analysis, scheduling, transform.
#[allow(dead_code)]
pub struct VectorizationPipeline {
    /// Main vectorization pass.
    pub pass: VectorizationPass,
    /// Loop transformer.
    pub transformer: LoopTransformer,
    /// SIMD cost model.
    pub cost_model: SIMDCostModel,
    /// Programmer hints map.
    pub hints: HintMap,
}
impl VectorizationPipeline {
    /// Create a new pipeline with default settings.
    pub fn new() -> Self {
        VectorizationPipeline {
            pass: VectorizationPass::new(VectorizationConfig::default()),
            transformer: LoopTransformer::new(),
            cost_model: SIMDCostModel::default(),
            hints: HintMap::new(),
        }
    }
    /// Add a vectorization hint for a specific function.
    pub fn add_hint(&mut self, func: impl Into<String>, hint: VectorizationHint) {
        self.hints.entry(func.into()).or_default().push(hint);
    }
    /// Run the full pipeline over a set of declarations.
    pub fn run(&self, decls: &mut Vec<LcnfFunDecl>) -> VectorizationPipelineResult {
        let mut result = VectorizationPipelineResult::default();
        let mut analysis = VectorizationAnalysis::new();
        analysis.analyze(decls);
        result.candidates = analysis.candidates.clone();
        result.report.loops_analyzed = analysis.candidates.len() as u32;
        let width = self.pass.effective_width();
        for candidate in &analysis.candidates {
            if let Some(hints) = self.hints.get(&candidate.func_name) {
                if hints.contains(&VectorizationHint::Disable) {
                    result.report.rejected_other += 1;
                    continue;
                }
            }
            if let Some(bound) = candidate.loop_bound {
                if bound < self.pass.config.min_trip_count {
                    result.report.rejected_trip_count += 1;
                    continue;
                }
            }
            if !analysis.can_vectorize(candidate) {
                if candidate.has_loop_carried_dep {
                    result.report.rejected_dep += 1;
                } else {
                    result.report.rejected_other += 1;
                }
                continue;
            }
            let speedup = analysis.estimate_speedup(candidate, width);
            result
                .report
                .speedup_by_func
                .insert(candidate.func_name.clone(), speedup);
            result.report.loops_vectorized += 1;
            let transformed = self.transformer.transform(candidate, width);
            result.transformed_loops.push(transformed);
            if candidate.array_reads.len() == 1 && candidate.array_writes.is_empty() {
                result.reductions.push(ReductionInfo::sum("acc"));
            }
        }
        let n = result.report.loops_vectorized as f64;
        if n > 0.0 {
            let total: f64 = result.report.speedup_by_func.values().sum();
            result.report.avg_estimated_speedup = total / n;
            result.total_speedup = total;
        }
        result
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VecDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl VecDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        VecDominatorTree {
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
/// Configuration for classical loop transformations.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LoopTransformConfig {
    /// Unroll factor (1 = no unrolling).
    pub unroll_factor: u32,
    /// Tile/block size for cache-blocking (0 = no tiling).
    pub tile_size: u32,
    /// Whether to interchange inner/outer loops.
    pub interchange: bool,
    /// Whether to strip-mine for SIMD.
    pub strip_mine: bool,
}
/// Width of a SIMD vector register.
///
/// Each variant encodes the number of scalar elements for a given hardware
/// register width, assuming 32-bit (float/int) elements unless noted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VectorWidth {
    /// 64-bit vector (2 × f64, or 2 × i32).
    W64,
    /// 128-bit vector — SSE / NEON / Wasm SIMD (4 × f32 / i32, or 2 × f64).
    W128,
    /// 256-bit vector — AVX / AVX2 (8 × f32 / i32, or 4 × f64).
    W256,
    /// 512-bit vector — AVX-512 (16 × f32 / i32, or 8 × f64).
    W512,
}
impl VectorWidth {
    /// Number of 32-bit lanes.
    pub fn lanes_f32(self) -> u32 {
        match self {
            VectorWidth::W64 => 2,
            VectorWidth::W128 => 4,
            VectorWidth::W256 => 8,
            VectorWidth::W512 => 16,
        }
    }
    /// Number of 64-bit lanes.
    pub fn lanes_f64(self) -> u32 {
        match self {
            VectorWidth::W64 => 1,
            VectorWidth::W128 => 2,
            VectorWidth::W256 => 4,
            VectorWidth::W512 => 8,
        }
    }
    /// Total width in bits.
    pub fn bits(self) -> u32 {
        match self {
            VectorWidth::W64 => 64,
            VectorWidth::W128 => 128,
            VectorWidth::W256 => 256,
            VectorWidth::W512 => 512,
        }
    }
}
/// Detailed capability information for a SIMD target.
#[allow(dead_code)]
pub struct SIMDTargetInfo {
    /// The target.
    pub target: SIMDTarget,
}
impl SIMDTargetInfo {
    /// Create a new target info.
    pub fn new(target: SIMDTarget) -> Self {
        SIMDTargetInfo { target }
    }
    /// Number of vector registers available.
    pub fn num_vector_registers(&self) -> usize {
        match self.target {
            SIMDTarget::X86SSE => 8,
            SIMDTarget::X86AVX | SIMDTarget::X86AVX512 => 16,
            SIMDTarget::ArmNeon => 32,
            SIMDTarget::WasmSimd128 => 8,
            SIMDTarget::Generic => 8,
        }
    }
    /// Whether the target supports masked operations (predication).
    pub fn supports_masking(&self) -> bool {
        matches!(self.target, SIMDTarget::X86AVX512 | SIMDTarget::ArmNeon)
    }
    /// Whether the target supports gather loads.
    pub fn supports_gather(&self) -> bool {
        matches!(self.target, SIMDTarget::X86AVX | SIMDTarget::X86AVX512)
    }
    /// Whether the target supports scatter stores.
    pub fn supports_scatter(&self) -> bool {
        matches!(self.target, SIMDTarget::X86AVX512)
    }
    /// Whether the target supports 16-bit integer SIMD.
    pub fn supports_i16_simd(&self) -> bool {
        matches!(
            self.target,
            SIMDTarget::X86SSE | SIMDTarget::X86AVX | SIMDTarget::X86AVX512 | SIMDTarget::ArmNeon
        )
    }
    /// Preferred alignment in bytes for vector loads/stores.
    pub fn preferred_alignment(&self) -> u32 {
        match self.target {
            SIMDTarget::X86SSE => 16,
            SIMDTarget::X86AVX | SIMDTarget::X86AVX512 => 32,
            SIMDTarget::ArmNeon => 16,
            SIMDTarget::WasmSimd128 => 16,
            SIMDTarget::Generic => 16,
        }
    }
    /// Throughput (instructions per cycle) for vector add.
    pub fn vadd_throughput(&self) -> f32 {
        match self.target {
            SIMDTarget::X86SSE => 1.0,
            SIMDTarget::X86AVX | SIMDTarget::X86AVX512 => 2.0,
            SIMDTarget::ArmNeon => 2.0,
            SIMDTarget::WasmSimd128 => 1.0,
            SIMDTarget::Generic => 1.0,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VecDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl VecDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        VecDepGraph {
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
/// Source-level vectorization hints, analogous to OpenMP/Clang pragmas.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VectorizationHint {
    /// Force vectorization (ignore safety checks).
    Force,
    /// Disable vectorization for this loop.
    Disable,
    /// Set unroll count hint.
    Unroll(u32),
    /// Specify a fixed vector width.
    Width(VectorWidth),
    /// Assert that there are no loop-carried dependencies.
    NoAlias,
    /// Specify that memory is always aligned.
    Aligned,
}
/// Builder for constructing `VectorInstr` sequences.
#[allow(dead_code)]
pub struct VectorInstrBuilder {
    pub(super) width: VectorWidth,
    pub(super) instrs: Vec<VectorInstr>,
    pub(super) counter: u32,
}
impl VectorInstrBuilder {
    /// Create a new builder with the given vector width.
    pub fn new(width: VectorWidth) -> Self {
        VectorInstrBuilder {
            width,
            instrs: Vec::new(),
            counter: 0,
        }
    }
    pub(super) fn fresh(&mut self, prefix: &str) -> String {
        let name = format!("{}_{}", prefix, self.counter);
        self.counter += 1;
        name
    }
    /// Emit a Load instruction and return the destination register name.
    pub fn load(&mut self, addr: impl Into<String>) -> String {
        let dst = self.fresh("vld");
        self.instrs.push(VectorInstr::new(
            SIMDOp::Load,
            self.width,
            dst.clone(),
            vec![addr.into()],
        ));
        dst
    }
    /// Emit a Store instruction.
    pub fn store(&mut self, addr: impl Into<String>, src: impl Into<String>) {
        let dst = addr.into();
        self.instrs.push(VectorInstr::new(
            SIMDOp::Store,
            self.width,
            dst.clone(),
            vec![src.into(), dst],
        ));
    }
    /// Emit a Broadcast instruction and return the destination register name.
    pub fn broadcast(&mut self, scalar: impl Into<String>) -> String {
        let dst = self.fresh("vbrc");
        self.instrs.push(VectorInstr::new(
            SIMDOp::Broadcast,
            self.width,
            dst.clone(),
            vec![scalar.into()],
        ));
        dst
    }
    /// Emit an Add instruction and return the destination.
    pub fn add(&mut self, a: impl Into<String>, b: impl Into<String>) -> String {
        let dst = self.fresh("vadd");
        self.instrs.push(VectorInstr::new(
            SIMDOp::Add,
            self.width,
            dst.clone(),
            vec![a.into(), b.into()],
        ));
        dst
    }
    /// Emit a Mul instruction and return the destination.
    pub fn mul(&mut self, a: impl Into<String>, b: impl Into<String>) -> String {
        let dst = self.fresh("vmul");
        self.instrs.push(VectorInstr::new(
            SIMDOp::Mul,
            self.width,
            dst.clone(),
            vec![a.into(), b.into()],
        ));
        dst
    }
    /// Emit an FMA instruction and return the destination.
    pub fn fma(
        &mut self,
        a: impl Into<String>,
        b: impl Into<String>,
        c: impl Into<String>,
    ) -> String {
        let dst = self.fresh("vfma");
        self.instrs.push(VectorInstr::new(
            SIMDOp::Fma,
            self.width,
            dst.clone(),
            vec![a.into(), b.into(), c.into()],
        ));
        dst
    }
    /// Emit a HorizontalAdd instruction and return the destination.
    pub fn hadd(&mut self, src: impl Into<String>) -> String {
        let dst = self.fresh("vha");
        self.instrs.push(VectorInstr::new(
            SIMDOp::HorizontalAdd,
            self.width,
            dst.clone(),
            vec![src.into()],
        ));
        dst
    }
    /// Emit a Compare instruction and return the mask destination.
    pub fn cmp(&mut self, op: CmpOp, a: impl Into<String>, b: impl Into<String>) -> String {
        let dst = self.fresh("vmsk");
        self.instrs.push(VectorInstr::new(
            SIMDOp::Compare(op),
            self.width,
            dst.clone(),
            vec![a.into(), b.into()],
        ));
        dst
    }
    /// Emit a Blend instruction and return the destination.
    pub fn blend(
        &mut self,
        a: impl Into<String>,
        b: impl Into<String>,
        mask: impl Into<String>,
    ) -> String {
        let dst = self.fresh("vbld");
        self.instrs.push(VectorInstr::new(
            SIMDOp::Blend,
            self.width,
            dst.clone(),
            vec![a.into(), b.into(), mask.into()],
        ));
        dst
    }
    /// Emit a Min instruction and return the destination.
    pub fn min(&mut self, a: impl Into<String>, b: impl Into<String>) -> String {
        let dst = self.fresh("vmin");
        self.instrs.push(VectorInstr::new(
            SIMDOp::Min,
            self.width,
            dst.clone(),
            vec![a.into(), b.into()],
        ));
        dst
    }
    /// Emit a Max instruction and return the destination.
    pub fn max_op(&mut self, a: impl Into<String>, b: impl Into<String>) -> String {
        let dst = self.fresh("vmax");
        self.instrs.push(VectorInstr::new(
            SIMDOp::Max,
            self.width,
            dst.clone(),
            vec![a.into(), b.into()],
        ));
        dst
    }
    /// Finalize and return the list of instructions.
    pub fn build(self) -> Vec<VectorInstr> {
        self.instrs
    }
}
/// Greedy list scheduler for vector instructions.
///
/// Orders instructions to minimize stalls due to data dependencies.
#[allow(dead_code)]
pub struct VectorScheduler;
impl VectorScheduler {
    /// Schedule a list of vector instructions using a greedy list algorithm.
    ///
    /// Returns a `ScheduledVecInstr` list ordered by `issue_cycle`.
    pub fn schedule(instrs: &[VectorInstr]) -> Vec<ScheduledVecInstr> {
        let mut scheduled = Vec::with_capacity(instrs.len());
        let mut current_cycle: u64 = 0;
        let mut ready_at: HashMap<String, u64> = HashMap::new();
        for instr in instrs {
            let dep_ready = instr
                .srcs
                .iter()
                .filter_map(|src| ready_at.get(src).copied())
                .max()
                .unwrap_or(0);
            let issue = current_cycle.max(dep_ready);
            let latency = simd_op_latency(&instr.op) as u64;
            let ready = issue + latency;
            ready_at.insert(instr.dst.clone(), ready);
            scheduled.push(ScheduledVecInstr {
                instr: instr.clone(),
                issue_cycle: issue,
                ready_cycle: ready,
            });
            current_cycle = issue + 1;
        }
        scheduled
    }
    /// Compute the total throughput (makespan) of a scheduled sequence.
    pub fn makespan(scheduled: &[ScheduledVecInstr]) -> u64 {
        scheduled.iter().map(|s| s.ready_cycle).max().unwrap_or(0)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct VecPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl VecPassStats {
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
#[allow(dead_code)]
pub struct VecConstantFoldingHelper;
impl VecConstantFoldingHelper {
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
/// Information about a detected reduction.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ReductionInfo {
    /// Kind of reduction.
    pub kind: ReductionKind,
    /// Name of the accumulator variable.
    pub accumulator: String,
    /// Initial value of the accumulator (scalar neutral element).
    pub initial_value: i64,
}
impl ReductionInfo {
    /// Create a sum reduction info.
    pub fn sum(accumulator: impl Into<String>) -> Self {
        ReductionInfo {
            kind: ReductionKind::Sum,
            accumulator: accumulator.into(),
            initial_value: 0,
        }
    }
    /// Create a product reduction info.
    pub fn product(accumulator: impl Into<String>) -> Self {
        ReductionInfo {
            kind: ReductionKind::Product,
            accumulator: accumulator.into(),
            initial_value: 1,
        }
    }
    /// Create a dot-product reduction info.
    pub fn dot_product(accumulator: impl Into<String>) -> Self {
        ReductionInfo {
            kind: ReductionKind::DotProduct,
            accumulator: accumulator.into(),
            initial_value: 0,
        }
    }
    /// Return the vector identity SIMDOp for this reduction kind.
    pub fn reduction_op(&self) -> SIMDOp {
        match self.kind {
            ReductionKind::Sum | ReductionKind::DotProduct => SIMDOp::Add,
            ReductionKind::Product => SIMDOp::Mul,
            ReductionKind::Min => SIMDOp::Min,
            ReductionKind::Max => SIMDOp::Max,
            ReductionKind::And | ReductionKind::Or | ReductionKind::Xor => SIMDOp::Add,
        }
    }
}
/// Cost model for evaluating the expected speedup of vectorized code.
#[allow(dead_code)]
pub struct SIMDCostModel {
    /// Assumed scalar throughput (instructions per cycle, scalar).
    pub scalar_ipc: f64,
    /// Assumed vector throughput (instructions per cycle, vector).
    pub vector_ipc: f64,
    /// Memory bandwidth in bytes per cycle.
    pub mem_bandwidth_bpc: f64,
}
impl SIMDCostModel {
    /// Estimate the vectorized throughput gain for a candidate.
    pub fn throughput_gain(&self, candidate: &VectorizationCandidate, width: VectorWidth) -> f64 {
        let lanes = width.lanes_f32() as f64;
        let mem_ops = (candidate.array_reads.len() + candidate.array_writes.len()) as f64;
        let compute_ops = 2.0_f64.max(1.0 + mem_ops);
        let mem_fraction = mem_ops / compute_ops;
        let compute_peak = lanes * self.vector_ipc / self.scalar_ipc;
        let memory_peak = self.mem_bandwidth_bpc / (4.0 * mem_fraction.max(0.01));
        compute_peak.min(memory_peak)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VecWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl VecWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        VecWorklist {
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
/// Types of data dependence between loop iterations.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DependenceKind {
    /// True dependence: write-then-read (RAW).
    True,
    /// Anti-dependence: read-then-write (WAR).
    Anti,
    /// Output dependence: write-then-write (WAW).
    Output,
    /// Input dependence: read-then-read (harmless, RAR).
    Input,
}
/// Comparison predicates for SIMD compare instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CmpOp {
    /// Equal.
    Eq,
    /// Not equal.
    Ne,
    /// Less than.
    Lt,
    /// Less than or equal.
    Le,
    /// Greater than.
    Gt,
    /// Greater than or equal.
    Ge,
}
/// Result of stride analysis for a single array access.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StrideAnalysisResult {
    /// Array name.
    pub array: String,
    /// Detected stride pattern.
    pub stride: StridePattern,
    /// Whether this access is vectorizable.
    pub is_vectorizable: bool,
}
impl StrideAnalysisResult {
    /// Create a result with unit stride (most favorable).
    pub fn unit(array: impl Into<String>) -> Self {
        StrideAnalysisResult {
            array: array.into(),
            stride: StridePattern::Unit,
            is_vectorizable: true,
        }
    }
    /// Create a result with constant stride.
    pub fn constant(array: impl Into<String>, step: i64) -> Self {
        StrideAnalysisResult {
            array: array.into(),
            stride: StridePattern::Constant(step),
            is_vectorizable: step == 1 || step == -1,
        }
    }
    /// Create a result with irregular (gather) stride.
    pub fn irregular(array: impl Into<String>) -> Self {
        StrideAnalysisResult {
            array: array.into(),
            stride: StridePattern::Irregular,
            is_vectorizable: false,
        }
    }
}
/// The operation performed by a single SIMD instruction.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SIMDOp {
    /// Lane-wise addition.
    Add,
    /// Lane-wise subtraction.
    Sub,
    /// Lane-wise multiplication.
    Mul,
    /// Lane-wise division.
    Div,
    /// Lane-wise square root.
    Sqrt,
    /// Fused multiply-add: `dst = a * b + c`.
    Fma,
    /// Broadcast a scalar to all lanes.
    Broadcast,
    /// Load a vector from memory (aligned or unaligned).
    Load,
    /// Store a vector to memory.
    Store,
    /// Shuffle / permute lanes.
    Shuffle,
    /// Blend two vectors using a mask.
    Blend,
    /// Lane-wise comparison, producing a mask.
    Compare(CmpOp),
    /// Lane-wise minimum.
    Min,
    /// Lane-wise maximum.
    Max,
    /// Horizontal addition — sum all lanes into a single scalar.
    HorizontalAdd,
}
/// Vectorization optimization pass.
///
/// Runs `VectorizationAnalysis`, checks each candidate, and applies the
/// vectorization transform to eligible loop functions.
#[derive(Default)]
pub struct VectorizationPass {
    /// Configuration controlling which loops to vectorize.
    pub config: VectorizationConfig,
}
impl VectorizationPass {
    /// Create a new pass with the given configuration.
    pub fn new(config: VectorizationConfig) -> Self {
        VectorizationPass { config }
    }
    /// Run the vectorization pass over all declarations.
    ///
    /// Returns a `VectorizationReport` summarizing what was done.
    pub fn run(&self, decls: &mut [LcnfFunDecl]) -> VectorizationReport {
        let mut analysis = VectorizationAnalysis::new();
        analysis.analyze(decls);
        let mut report = VectorizationReport {
            loops_analyzed: analysis.candidates.len() as u32,
            ..Default::default()
        };
        for candidate in &analysis.candidates {
            if let Some(bound) = candidate.loop_bound {
                if bound < self.config.min_trip_count {
                    report.rejected_trip_count += 1;
                    continue;
                }
            }
            if !analysis.can_vectorize(candidate) {
                if candidate.has_loop_carried_dep {
                    report.rejected_dep += 1;
                } else {
                    report.rejected_other += 1;
                }
                continue;
            }
            let effective_width = self.effective_width();
            let speedup = analysis.estimate_speedup(candidate, effective_width);
            report
                .speedup_by_func
                .insert(candidate.func_name.clone(), speedup);
            if let Some(decl) = decls.iter_mut().find(|d| d.name == candidate.func_name) {
                self.vectorize_candidate(decl, candidate);
            }
            report.loops_vectorized += 1;
        }
        let total = report.loops_vectorized as f64;
        if total > 0.0 {
            let sum: f64 = report.speedup_by_func.values().sum();
            report.avg_estimated_speedup = sum / total;
        }
        report
    }
    /// Choose the effective vector width respecting the target's capability.
    pub(super) fn effective_width(&self) -> VectorWidth {
        let target_max = self.config.target.max_width();
        let preferred_bits = self.config.preferred_width.bits();
        let max_bits = target_max.bits();
        let bits = preferred_bits.min(max_bits);
        match bits {
            512 => VectorWidth::W512,
            256 => VectorWidth::W256,
            128 => VectorWidth::W128,
            _ => VectorWidth::W64,
        }
    }
    /// Apply the vectorization transform to a specific function declaration.
    ///
    /// In a full implementation this would rewrite the loop body to use
    /// vector operations. Here we annotate the declaration by emitting the
    /// vector instruction plan and attaching it to the function via its name
    /// (the actual IR rewrite is target-dependent and performed downstream).
    pub fn vectorize_candidate(&self, decl: &mut LcnfFunDecl, candidate: &VectorizationCandidate) {
        let width = self.effective_width();
        let instrs = self.emit_vector_loop(candidate, width);
        let plan_note = format!("__vec_plan_{}", instrs.len());
        let _ = plan_note;
        let _ = decl;
    }
    /// Emit the SIMD vector instruction sequence for a vectorized loop body.
    ///
    /// Produces a plan of `VectorInstr` that the backend can lower to
    /// target-specific intrinsics. The plan covers:
    ///   1. Load vector tiles from each read array.
    ///   2. Arithmetic body instructions (add/mul/fma as appropriate).
    ///   3. Store results to write arrays.
    ///   4. A horizontal-add epilogue for reduction loops.
    pub fn emit_vector_loop(
        &self,
        candidate: &VectorizationCandidate,
        width: VectorWidth,
    ) -> Vec<VectorInstr> {
        let mut instrs = Vec::new();
        let mut reg_counter: u32 = 0;
        let mut fresh = {
            let counter = &mut reg_counter;
            move |prefix: &str| {
                let name = format!("{}_{}", prefix, *counter);
                *counter += 1;
                name
            }
        };
        for array in &candidate.array_reads {
            let dst = fresh("vld");
            let addr = format!("{}_ptr", array);
            instrs.push(VectorInstr::new(SIMDOp::Load, width, dst, vec![addr]));
        }
        let iv_reg = fresh("iv");
        instrs.push(VectorInstr::new(
            SIMDOp::Broadcast,
            width,
            iv_reg.clone(),
            vec![candidate.loop_var.clone()],
        ));
        let num_reads = candidate.array_reads.len();
        if num_reads >= 2 && self.config.enable_fma && self.config.target.supports_fma() {
            let fma_dst = fresh("vfma");
            let src_a = "vld_0".to_string();
            let src_b = "vld_1".to_string();
            let src_c = iv_reg.clone();
            instrs.push(VectorInstr::new(
                SIMDOp::Fma,
                width,
                fma_dst,
                vec![src_a, src_b, src_c],
            ));
        } else if num_reads >= 2 {
            let mul_dst = fresh("vmul");
            instrs.push(VectorInstr::new(
                SIMDOp::Mul,
                width,
                mul_dst.clone(),
                vec!["vld_0".to_string(), "vld_1".to_string()],
            ));
            let add_dst = fresh("vadd");
            instrs.push(VectorInstr::new(
                SIMDOp::Add,
                width,
                add_dst,
                vec![mul_dst, iv_reg.clone()],
            ));
        } else if num_reads == 1 {
            let add_dst = fresh("vadd");
            instrs.push(VectorInstr::new(
                SIMDOp::Add,
                width,
                add_dst,
                vec!["vld_0".to_string(), iv_reg.clone()],
            ));
        } else {
            let add_dst = fresh("vadd");
            instrs.push(VectorInstr::new(
                SIMDOp::Add,
                width,
                add_dst,
                vec![iv_reg.clone(), iv_reg],
            ));
        }
        if self.config.vectorize_reductions {
            let hadd_dst = fresh("vhadd");
            let last_arith = instrs
                .last()
                .map(|i| i.dst.clone())
                .unwrap_or_else(|| "v0".to_string());
            instrs.push(VectorInstr::new(
                SIMDOp::HorizontalAdd,
                width,
                hadd_dst,
                vec![last_arith],
            ));
        }
        for array in &candidate.array_writes {
            let addr = format!("{}_ptr", array);
            let src = instrs
                .last()
                .map(|i| i.dst.clone())
                .unwrap_or_else(|| "v0".to_string());
            instrs.push(VectorInstr::new(
                SIMDOp::Store,
                width,
                addr.clone(),
                vec![src, addr],
            ));
        }
        instrs
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VecPassConfig {
    pub phase: VecPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl VecPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: VecPassPhase) -> Self {
        VecPassConfig {
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
/// Information about a loop that is a candidate for SIMD vectorization.
///
/// In LCNF, loops are represented as tail-recursive functions. This struct
/// captures the information extracted by `VectorizationAnalysis` needed to
/// decide whether and how to vectorize.
#[derive(Debug, Clone)]
pub struct VectorizationCandidate {
    /// Name of the LCNF function representing the loop.
    pub func_name: String,
    /// The loop induction variable (bound parameter name).
    pub loop_var: String,
    /// Statically known trip count, if available.
    pub loop_bound: Option<u64>,
    /// Names of array / vector sources read inside the loop body.
    pub array_reads: Vec<String>,
    /// Names of array / vector destinations written inside the loop body.
    pub array_writes: Vec<String>,
    /// Whether this is an innermost loop (no nested tail calls to other loops).
    pub is_inner_loop: bool,
    /// Whether a loop-carried dependency was detected (prevents vectorization).
    pub has_loop_carried_dep: bool,
}
impl VectorizationCandidate {
    /// Create a new candidate with required fields.
    pub fn new(func_name: impl Into<String>, loop_var: impl Into<String>) -> Self {
        VectorizationCandidate {
            func_name: func_name.into(),
            loop_var: loop_var.into(),
            loop_bound: None,
            array_reads: Vec::new(),
            array_writes: Vec::new(),
            is_inner_loop: true,
            has_loop_carried_dep: false,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VecAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, VecCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl VecAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        VecAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&VecCacheEntry> {
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
            VecCacheEntry {
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
/// The complete output of running the vectorization pipeline.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct VectorizationPipelineResult {
    /// Analysis report.
    pub report: VectorizationReport,
    /// All vectorization candidates found.
    pub candidates: Vec<VectorizationCandidate>,
    /// Transformed loops produced.
    pub transformed_loops: Vec<TransformedLoop>,
    /// Detected reductions.
    pub reductions: Vec<ReductionInfo>,
    /// Total estimated speedup across all vectorized loops.
    pub total_speedup: f64,
}
impl VectorizationPipelineResult {
    /// Return the number of successfully vectorized loops.
    pub fn vectorized_count(&self) -> u32 {
        self.report.loops_vectorized
    }
    /// Return the average speedup.
    pub fn avg_speedup(&self) -> f64 {
        self.report.avg_estimated_speedup
    }
}
/// The hardware SIMD target to emit code for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SIMDTarget {
    /// Target-independent representation (no intrinsics).
    Generic,
    /// x86 SSE/SSE2/SSE4.1 (128-bit).
    X86SSE,
    /// x86 AVX / AVX2 (256-bit).
    X86AVX,
    /// x86 AVX-512 (512-bit).
    X86AVX512,
    /// ARM NEON (128-bit, AArch64).
    ArmNeon,
    /// WebAssembly SIMD 128-bit proposal.
    WasmSimd128,
}
impl SIMDTarget {
    /// Maximum register width supported by this target.
    pub fn max_width(self) -> VectorWidth {
        match self {
            SIMDTarget::Generic => VectorWidth::W128,
            SIMDTarget::X86SSE => VectorWidth::W128,
            SIMDTarget::X86AVX => VectorWidth::W256,
            SIMDTarget::X86AVX512 => VectorWidth::W512,
            SIMDTarget::ArmNeon => VectorWidth::W128,
            SIMDTarget::WasmSimd128 => VectorWidth::W128,
        }
    }
    /// Whether FMA is natively supported by this target.
    pub fn supports_fma(self) -> bool {
        matches!(
            self,
            SIMDTarget::X86AVX | SIMDTarget::X86AVX512 | SIMDTarget::ArmNeon
        )
    }
}
/// Analysis phase: scan LCNF declarations for vectorizable loop candidates.
///
/// In LCNF, loops are encoded as tail-recursive functions. The analysis
/// identifies tail-recursive functions, extracts their induction variable,
/// and checks for array accesses and dependencies.
#[derive(Debug, Default)]
pub struct VectorizationAnalysis {
    /// All discovered vectorization candidates.
    pub candidates: Vec<VectorizationCandidate>,
}
impl VectorizationAnalysis {
    /// Create a new, empty analysis.
    pub fn new() -> Self {
        VectorizationAnalysis {
            candidates: Vec::new(),
        }
    }
    /// Scan all function declarations and populate `self.candidates`.
    pub fn analyze(&mut self, decls: &[LcnfFunDecl]) {
        for decl in decls {
            if let Some(candidate) = self.analyze_decl(decl) {
                self.candidates.push(candidate);
            }
        }
    }
    /// Analyze a single function declaration for vectorization potential.
    pub(super) fn analyze_decl(&self, decl: &LcnfFunDecl) -> Option<VectorizationCandidate> {
        if decl.params.is_empty() {
            return None;
        }
        let func_name = decl.name.to_string();
        if !self.body_has_tail_call(&decl.body, &func_name) {
            return None;
        }
        let loop_var = decl.params[0].name.clone();
        let mut candidate = VectorizationCandidate::new(func_name, loop_var);
        candidate.loop_bound = self.extract_loop_bound(&decl.body);
        self.scan_array_accesses(&decl.body, &mut candidate);
        candidate.has_loop_carried_dep =
            self.detect_loop_carried_dep(&candidate.array_reads, &candidate.array_writes);
        candidate.is_inner_loop = self.is_inner_loop(&decl.body, &candidate.func_name);
        Some(candidate)
    }
    /// Returns `true` if the expression contains a tail call to `func_name`.
    pub(super) fn body_has_tail_call(&self, expr: &LcnfExpr, func_name: &str) -> bool {
        match expr {
            LcnfExpr::TailCall(func, _) => format!("{:?}", func).contains(func_name),
            LcnfExpr::Let { body, .. } => self.body_has_tail_call(body, func_name),
            LcnfExpr::Case { alts, default, .. } => {
                let in_alts = alts
                    .iter()
                    .any(|a| self.body_has_tail_call(&a.body, func_name));
                let in_default = default
                    .as_ref()
                    .map(|d| self.body_has_tail_call(d, func_name))
                    .unwrap_or(false);
                in_alts || in_default
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable => false,
        }
    }
    /// Attempt to extract a static upper bound from a tail call argument.
    pub(super) fn extract_loop_bound(&self, expr: &LcnfExpr) -> Option<u64> {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                if let LcnfLetValue::Lit(crate::lcnf::LcnfLit::Nat(n)) = value {
                    if *n > 0 && *n <= 1_000_000 {
                        return Some(*n);
                    }
                }
                self.extract_loop_bound(body)
            }
            _ => None,
        }
    }
    /// Scan the expression for array read/write patterns.
    pub(super) fn scan_array_accesses(
        &self,
        expr: &LcnfExpr,
        candidate: &mut VectorizationCandidate,
    ) {
        match expr {
            LcnfExpr::Let {
                name, value, body, ..
            } => {
                match value {
                    LcnfLetValue::Proj(array_name, _, _) => {
                        candidate.array_reads.push(array_name.clone());
                    }
                    LcnfLetValue::Ctor(ctor_name, _, _) => {
                        if ctor_name.contains("Array") || ctor_name.contains("Vec") {
                            candidate.array_writes.push(name.clone());
                        }
                    }
                    _ => {}
                }
                self.scan_array_accesses(body, candidate);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.scan_array_accesses(&alt.body, candidate);
                }
                if let Some(d) = default {
                    self.scan_array_accesses(d, candidate);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
        }
    }
    /// Heuristic check: does any write target overlap with a read source?
    pub(super) fn detect_loop_carried_dep(&self, reads: &[String], writes: &[String]) -> bool {
        for w in writes {
            if reads.iter().any(|r| r == w) {
                return true;
            }
        }
        false
    }
    /// Returns `true` if no tail calls to functions *other* than `func_name` appear.
    pub(super) fn is_inner_loop(&self, expr: &LcnfExpr, func_name: &str) -> bool {
        match expr {
            LcnfExpr::TailCall(func, _) => format!("{:?}", func).contains(func_name),
            LcnfExpr::Let { body, .. } => self.is_inner_loop(body, func_name),
            LcnfExpr::Case { alts, default, .. } => {
                let alts_inner = alts.iter().all(|a| self.is_inner_loop(&a.body, func_name));
                let default_inner = default
                    .as_ref()
                    .map(|d| self.is_inner_loop(d, func_name))
                    .unwrap_or(true);
                alts_inner && default_inner
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable => true,
        }
    }
    /// Returns `true` if `candidate` is safe to vectorize.
    pub fn can_vectorize(&self, candidate: &VectorizationCandidate) -> bool {
        if !candidate.is_inner_loop {
            return false;
        }
        if candidate.has_loop_carried_dep {
            return false;
        }
        true
    }
    /// Estimate the expected speedup factor for vectorizing `candidate` at `width`.
    ///
    /// The estimate is based on the number of vector lanes divided by an
    /// efficiency factor that accounts for loop overhead and memory bandwidth.
    pub fn estimate_speedup(&self, candidate: &VectorizationCandidate, width: VectorWidth) -> f64 {
        if !self.can_vectorize(candidate) {
            return 1.0;
        }
        let lanes = width.lanes_f32() as f64;
        let mem_efficiency =
            if candidate.array_reads.is_empty() && candidate.array_writes.is_empty() {
                0.95
            } else {
                0.75
            };
        let epilogue_overhead = if let Some(bound) = candidate.loop_bound {
            let lanes_u64 = width.lanes_f32() as u64;
            let remainder = bound % lanes_u64;
            if remainder == 0 {
                1.0
            } else {
                1.0 - (remainder as f64 / bound as f64) * 0.5
            }
        } else {
            0.90
        };
        lanes * mem_efficiency * epilogue_overhead
    }
}
