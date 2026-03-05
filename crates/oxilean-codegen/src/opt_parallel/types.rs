//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfVarId};
use std::collections::{HashMap, HashSet};

use super::functions::*;
use std::collections::VecDeque;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl ParLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        ParLivenessInfo {
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
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ParPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl ParPassStats {
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
#[derive(Debug, Clone)]
pub struct ParCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A code region that has been identified as amenable to parallel execution.
#[derive(Debug, Clone)]
pub struct ParallelRegion {
    /// Name of the LCNF function that represents this region.
    pub func_name: String,
    /// Index of the first basic block in the parallel body.
    pub start_block: usize,
    /// Index of the last basic block in the parallel body (inclusive).
    pub end_block: usize,
    /// Inferred parallel-execution model.
    pub kind: ParallelKind,
    /// Specific algorithmic pattern detected.
    pub pattern: ParallelPattern,
    /// Estimated parallel speed-up (ideal / Amdahl-limited).
    pub estimated_speedup: f64,
    /// Variables that are shared across parallel threads.
    pub shared_vars: Vec<String>,
    /// Variables that are private to each parallel thread / iteration.
    pub private_vars: Vec<String>,
    /// Trip count of the parallel loop (if statically known).
    pub trip_count: Option<u64>,
    /// Dependence analysis result for this region.
    pub dependence_info: DependenceInfo,
}
impl ParallelRegion {
    /// Create a minimal parallel region descriptor.
    pub fn new(func_name: impl Into<String>, kind: ParallelKind, pattern: ParallelPattern) -> Self {
        ParallelRegion {
            func_name: func_name.into(),
            start_block: 0,
            end_block: 0,
            kind,
            pattern,
            estimated_speedup: 1.0,
            shared_vars: Vec::new(),
            private_vars: Vec::new(),
            trip_count: None,
            dependence_info: DependenceInfo::default(),
        }
    }
    /// Whether this region is worth parallelising (speedup > threshold).
    pub fn is_profitable(&self, threshold: f64) -> bool {
        self.estimated_speedup > threshold && self.dependence_info.is_parallelizable()
    }
}
/// A diagnostic message from a OPar pass.
#[derive(Debug, Clone)]
pub struct OParDiagMsg {
    pub severity: OParDiagSeverity,
    pub pass: String,
    pub message: String,
}
impl OParDiagMsg {
    pub fn error(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        OParDiagMsg {
            severity: OParDiagSeverity::Error,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn warning(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        OParDiagMsg {
            severity: OParDiagSeverity::Warning,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn note(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        OParDiagMsg {
            severity: OParDiagSeverity::Note,
            pass: pass.into(),
            message: msg.into(),
        }
    }
}
/// Collects OPar diagnostics.
#[derive(Debug, Default)]
pub struct OParDiagCollector {
    pub(super) msgs: Vec<OParDiagMsg>,
}
impl OParDiagCollector {
    pub fn new() -> Self {
        OParDiagCollector::default()
    }
    pub fn emit(&mut self, d: OParDiagMsg) {
        self.msgs.push(d);
    }
    pub fn has_errors(&self) -> bool {
        self.msgs
            .iter()
            .any(|d| d.severity == OParDiagSeverity::Error)
    }
    pub fn errors(&self) -> Vec<&OParDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == OParDiagSeverity::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&OParDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == OParDiagSeverity::Warning)
            .collect()
    }
    pub fn len(&self) -> usize {
        self.msgs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.msgs.is_empty()
    }
    pub fn clear(&mut self) {
        self.msgs.clear();
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParPassConfig {
    pub phase: ParPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl ParPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: ParPassPhase) -> Self {
        ParPassConfig {
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
/// Severity of a OPar diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OParDiagSeverity {
    Note,
    Warning,
    Error,
}
/// Dependency edge between two LCNF variables (or array accesses).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DepEdge {
    /// Source variable or access.
    pub from: String,
    /// Destination variable or access.
    pub to: String,
    /// Distance vector (simplified to a scalar for 1-D loops).
    pub distance: i64,
}
/// Summary statistics produced by `ParallelPass::report()`.
#[derive(Debug, Clone, Default)]
pub struct ParallelReport {
    /// Number of parallel regions identified.
    pub regions_found: usize,
    /// Number of regions actually transformed.
    pub regions_transformed: usize,
    /// Estimated combined speed-up (product of individual speed-ups).
    pub estimated_total_speedup: f64,
    /// Number of data-race conditions detected (prevents transformation).
    pub race_conditions_detected: usize,
}
/// Loop-dependence information for a single function/loop nest.
///
/// We use a simplified version of the standard three-class model:
/// - **True dependence** (RAW): iteration J reads a value written by iteration I < J.
/// - **Anti dependence** (WAR): iteration J writes a location read by iteration I < J.
/// - **Output dependence** (WAW): two iterations write the same location.
/// - **Loop-carried**: the dependence crosses loop iterations (distance > 0).
#[derive(Debug, Clone, Default)]
pub struct DependenceInfo {
    /// Read-after-write (true) dependences.
    pub true_deps: Vec<DepEdge>,
    /// Write-after-read (anti) dependences.
    pub anti_deps: Vec<DepEdge>,
    /// Write-after-write (output) dependences.
    pub output_deps: Vec<DepEdge>,
    /// Subset of the above that are loop-carried (distance >= 1).
    pub loop_carried_deps: Vec<DepEdge>,
}
impl DependenceInfo {
    /// Returns `true` when no loop-carried dependences exist (Bernstein safe).
    pub fn is_parallelizable(&self) -> bool {
        self.loop_carried_deps.is_empty()
    }
    /// Total number of dependence edges.
    pub fn total_deps(&self) -> usize {
        self.true_deps.len() + self.anti_deps.len() + self.output_deps.len()
    }
}
/// Thread-safety analysis result for a parallel region.
#[derive(Debug, Clone)]
pub struct ThreadSafetyInfo {
    /// Whether the region is provably free of data races.
    pub is_thread_safe: bool,
    /// Pairs of (write, read/write) accesses that may race.
    pub race_conditions: Vec<(String, String)>,
    /// Variables that need atomic operations to be safe.
    pub atomic_ops_needed: Vec<String>,
}
impl ThreadSafetyInfo {
    /// Construct a trivially safe result.
    pub fn safe() -> Self {
        ThreadSafetyInfo {
            is_thread_safe: true,
            race_conditions: Vec::new(),
            atomic_ops_needed: Vec::new(),
        }
    }
    /// Construct an unsafe result with a single race.
    pub fn unsafe_race(write: impl Into<String>, read: impl Into<String>) -> Self {
        ThreadSafetyInfo {
            is_thread_safe: false,
            race_conditions: vec![(write.into(), read.into())],
            atomic_ops_needed: Vec::new(),
        }
    }
}
/// Configuration for the parallelism pass.
#[derive(Debug, Clone)]
pub struct ParallelConfig {
    /// Minimum estimated speedup below which a region is not transformed.
    pub min_speedup_threshold: f64,
    /// Maximum number of functions analysed (prevents runaway).
    pub max_functions: usize,
    /// Enable speculative parallelism (may be unsound).
    pub allow_speculative: bool,
    /// Assumed number of hardware threads for speed-up estimation.
    pub hardware_threads: u32,
}
/// Statistics for OParExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OParExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl OParExtPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn visit(&mut self) {
        self.nodes_visited += 1;
    }
    #[allow(dead_code)]
    pub fn modify(&mut self) {
        self.nodes_modified += 1;
        self.changed = true;
    }
    #[allow(dead_code)]
    pub fn iterate(&mut self) {
        self.iterations += 1;
    }
    #[allow(dead_code)]
    pub fn error(&mut self) {
        self.errors += 1;
    }
    #[allow(dead_code)]
    pub fn efficiency(&self) -> f64 {
        if self.nodes_visited == 0 {
            0.0
        } else {
            self.nodes_modified as f64 / self.nodes_visited as f64
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, o: &OParExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// The main parallelism optimisation pass.
pub struct ParallelPass {
    /// Configuration.
    pub config: ParallelConfig,
    /// Regions identified during analysis.
    pub regions: Vec<ParallelRegion>,
    /// Memoised thread-safety results.
    pub(super) safety_cache: HashMap<String, ThreadSafetyInfo>,
}
impl ParallelPass {
    /// Create a new pass with the given configuration.
    pub fn new(config: ParallelConfig) -> Self {
        ParallelPass {
            config,
            regions: Vec::new(),
            safety_cache: HashMap::new(),
        }
    }
    /// Create a pass with default configuration.
    pub fn default_pass() -> Self {
        Self::new(ParallelConfig::default())
    }
    /// Run analysis + transformation over all function declarations.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        self.regions.clear();
        self.safety_cache.clear();
        self.analyze_parallelism(decls);
        self.transform_to_parallel(decls);
    }
    /// Analyse all declarations and populate `self.regions`.
    pub fn analyze_parallelism(&mut self, decls: &[LcnfFunDecl]) {
        for decl in decls.iter().take(self.config.max_functions) {
            if let Some(region) = self.analyse_decl(decl) {
                self.regions.push(region);
            }
        }
    }
    /// Transform declarations that have profitable parallel regions.
    ///
    /// Currently adds an annotation to the function body (via `LcnfExpr::Let`
    /// with a sentinel `Ctor` value) so that downstream code generators can
    /// see the annotation.  A full production implementation would rewrite the
    /// loop body to use a parallel runtime API.
    pub fn transform_to_parallel(&mut self, decls: &mut [LcnfFunDecl]) {
        let profitable: HashSet<String> = self
            .regions
            .iter()
            .filter(|r| r.is_profitable(self.config.min_speedup_threshold))
            .map(|r| r.func_name.clone())
            .collect();
        for decl in decls.iter_mut() {
            if profitable.contains(&decl.name) {
                let old_body = std::mem::replace(&mut decl.body, LcnfExpr::Unreachable);
                decl.body = LcnfExpr::Let {
                    id: LcnfVarId(u64::MAX),
                    name: "__parallel_annotation__".to_string(),
                    ty: crate::lcnf::LcnfType::Unit,
                    value: LcnfLetValue::Ctor("parallel_region".to_string(), 0, vec![]),
                    body: Box::new(old_body),
                };
            }
        }
    }
    /// Produce a summary report of the pass results.
    pub fn report(&self) -> ParallelReport {
        let races: usize = self
            .safety_cache
            .values()
            .map(|s| s.race_conditions.len())
            .sum();
        let transformed = self
            .regions
            .iter()
            .filter(|r| r.is_profitable(self.config.min_speedup_threshold))
            .count();
        let total_speedup = if self.regions.is_empty() {
            1.0
        } else {
            self.regions
                .iter()
                .filter(|r| r.is_profitable(self.config.min_speedup_threshold))
                .map(|r| r.estimated_speedup)
                .fold(1.0_f64, f64::max)
        };
        ParallelReport {
            regions_found: self.regions.len(),
            regions_transformed: transformed,
            estimated_total_speedup: total_speedup,
            race_conditions_detected: races,
        }
    }
    pub(super) fn analyse_decl(&mut self, decl: &LcnfFunDecl) -> Option<ParallelRegion> {
        let pattern = PatternDetector::new(decl).detect()?;
        let dep_info = DependenceAnalyser::new(decl).analyse();
        if !dep_info.is_parallelizable() {
            return None;
        }
        let kind = self.infer_kind(pattern, decl);
        if kind == ParallelKind::SpeculativeParallel && !self.config.allow_speculative {
            return None;
        }
        let trip_count = self.estimate_trip_count(decl);
        let speedup = estimate_speedup_for_pattern(pattern, trip_count);
        let detector = RaceDetector::new();
        let safety = detector.analyse_decl(decl);
        self.safety_cache.insert(decl.name.clone(), safety.clone());
        if !safety.is_thread_safe && kind == ParallelKind::DataParallel {
            return None;
        }
        let shared_vars = self.collect_shared_vars(decl);
        let private_vars = self.collect_private_vars(decl);
        Some(ParallelRegion {
            func_name: decl.name.clone(),
            start_block: 0,
            end_block: decl.params.len().saturating_sub(1),
            kind,
            pattern,
            estimated_speedup: speedup,
            shared_vars,
            private_vars,
            trip_count,
            dependence_info: dep_info,
        })
    }
    pub(super) fn infer_kind(&self, pattern: ParallelPattern, _decl: &LcnfFunDecl) -> ParallelKind {
        match pattern {
            ParallelPattern::Map
            | ParallelPattern::Filter
            | ParallelPattern::Gather
            | ParallelPattern::Scatter
            | ParallelPattern::Stencil
            | ParallelPattern::ParallelFor => ParallelKind::DataParallel,
            ParallelPattern::Reduce => ParallelKind::DataParallel,
            ParallelPattern::Scan => ParallelKind::PipelineParallel,
        }
    }
    pub(super) fn estimate_trip_count(&self, decl: &LcnfFunDecl) -> Option<u64> {
        for param in &decl.params {
            if param.name == "n" || param.name == "len" || param.name == "size" {
                return Some(256);
            }
        }
        None
    }
    pub(super) fn collect_shared_vars(&self, decl: &LcnfFunDecl) -> Vec<String> {
        decl.params
            .iter()
            .filter(|p| p.name.starts_with("arr") || p.name.starts_with("buf"))
            .map(|p| p.name.clone())
            .collect()
    }
    pub(super) fn collect_private_vars(&self, decl: &LcnfFunDecl) -> Vec<String> {
        let mut privates = Vec::new();
        Self::collect_let_names(&decl.body, &mut privates);
        privates
    }
    pub(super) fn collect_let_names(expr: &LcnfExpr, out: &mut Vec<String>) {
        match expr {
            LcnfExpr::Let { name, body, .. } => {
                out.push(name.clone());
                Self::collect_let_names(body, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    Self::collect_let_names(&alt.body, out);
                }
                if let Some(d) = default {
                    Self::collect_let_names(d, out);
                }
            }
            _ => {}
        }
    }
}
/// Pass execution phase for OParExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OParExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl OParExtPassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}
/// Pass registry for OParExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct OParExtPassRegistry {
    pub(super) configs: Vec<OParExtPassConfig>,
    pub(super) stats: Vec<OParExtPassStats>,
}
impl OParExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: OParExtPassConfig) {
        self.stats.push(OParExtPassStats::new());
        self.configs.push(c);
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }
    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> Option<&OParExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&OParExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&OParExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &OParExtPassPhase) -> Vec<&OParExtPassConfig> {
        self.configs
            .iter()
            .filter(|c| c.enabled && &c.phase == ph)
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_nodes_visited(&self) -> usize {
        self.stats.iter().map(|s| s.nodes_visited).sum()
    }
    #[allow(dead_code)]
    pub fn any_changed(&self) -> bool {
        self.stats.iter().any(|s| s.changed)
    }
}
/// Worklist for OParExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OParExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl OParExtWorklist {
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            items: std::collections::VecDeque::new(),
            present: vec![false; capacity],
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_back(id);
        }
    }
    #[allow(dead_code)]
    pub fn push_front(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_front(id);
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<usize> {
        let id = self.items.pop_front()?;
        if id < self.present.len() {
            self.present[id] = false;
        }
        Some(id)
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
    pub fn contains(&self, id: usize) -> bool {
        id < self.present.len() && self.present[id]
    }
    #[allow(dead_code)]
    pub fn drain_all(&mut self) -> Vec<usize> {
        let v: Vec<usize> = self.items.drain(..).collect();
        for &id in &v {
            if id < self.present.len() {
                self.present[id] = false;
            }
        }
        v
    }
}
/// Dominator tree for OParExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OParExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl OParExtDomTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            idom: vec![None; n],
            children: vec![Vec::new(); n],
            depth: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, dom: usize) {
        if node < self.idom.len() {
            self.idom[node] = Some(dom);
            if dom < self.children.len() {
                self.children[dom].push(node);
            }
            self.depth[node] = if dom < self.depth.len() {
                self.depth[dom] + 1
            } else {
                1
            };
        }
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        if a == b {
            return true;
        }
        let n = self.idom.len();
        for _ in 0..n {
            match self.idom.get(b).copied().flatten() {
                None => return false,
                Some(p) if p == a => return true,
                Some(p) if p == b => return false,
                Some(p) => b = p,
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn children_of(&self, n: usize) -> &[usize] {
        self.children.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn depth_of(&self, n: usize) -> usize {
        self.depth.get(n).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.idom.len();
        for _ in 0..(2 * n) {
            if a == b {
                return a;
            }
            if self.depth_of(a) > self.depth_of(b) {
                a = self.idom.get(a).and_then(|x| *x).unwrap_or(a);
            } else {
                b = self.idom.get(b).and_then(|x| *x).unwrap_or(b);
            }
        }
        0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl ParDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ParDepGraph {
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
/// A text buffer for building OPar output source code.
#[derive(Debug, Default)]
pub struct OParSourceBuffer {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
impl OParSourceBuffer {
    pub fn new() -> Self {
        OParSourceBuffer {
            buf: String::new(),
            indent_level: 0,
            indent_str: "    ".to_string(),
        }
    }
    pub fn with_indent(mut self, indent: impl Into<String>) -> Self {
        self.indent_str = indent.into();
        self
    }
    pub fn push_line(&mut self, line: &str) {
        for _ in 0..self.indent_level {
            self.buf.push_str(&self.indent_str);
        }
        self.buf.push_str(line);
        self.buf.push('\n');
    }
    pub fn push_raw(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }
    pub fn dedent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }
    pub fn as_str(&self) -> &str {
        &self.buf
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    pub fn line_count(&self) -> usize {
        self.buf.lines().count()
    }
    pub fn into_string(self) -> String {
        self.buf
    }
    pub fn reset(&mut self) {
        self.buf.clear();
        self.indent_level = 0;
    }
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in OPar).
#[derive(Debug)]
pub struct OParEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
}
impl OParEventLog {
    pub fn new(capacity: usize) -> Self {
        OParEventLog {
            entries: std::collections::VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    pub fn push(&mut self, event: impl Into<String>) {
        if self.entries.len() >= self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(event.into());
    }
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.entries.iter()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// Analysis cache for OParExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct OParExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl OParExtCache {
    #[allow(dead_code)]
    pub fn new(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            cap,
            total_hits: 0,
            total_misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: u64) -> Option<&[u8]> {
        for e in self.entries.iter_mut() {
            if e.0 == key && e.2 {
                e.3 += 1;
                self.total_hits += 1;
                return Some(&e.1);
            }
        }
        self.total_misses += 1;
        None
    }
    #[allow(dead_code)]
    pub fn put(&mut self, key: u64, data: Vec<u8>) {
        if self.entries.len() >= self.cap {
            self.entries.retain(|e| e.2);
            if self.entries.len() >= self.cap {
                self.entries.remove(0);
            }
        }
        self.entries.push((key, data, true, 0));
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        for e in self.entries.iter_mut() {
            e.2 = false;
        }
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let t = self.total_hits + self.total_misses;
        if t == 0 {
            0.0
        } else {
            self.total_hits as f64 / t as f64
        }
    }
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| e.2).count()
    }
}
/// Analyses loop-carried dependences for a single LCNF function.
pub struct DependenceAnalyser<'a> {
    pub(super) decl: &'a LcnfFunDecl,
}
impl<'a> DependenceAnalyser<'a> {
    pub(super) fn new(decl: &'a LcnfFunDecl) -> Self {
        DependenceAnalyser { decl }
    }
    pub(super) fn analyse(&self) -> DependenceInfo {
        let accesses = self.collect_accesses(&self.decl.body);
        let mut info = DependenceInfo::default();
        for i in 0..accesses.len() {
            for j in (i + 1)..accesses.len() {
                let a = &accesses[i];
                let b = &accesses[j];
                if a.independent_from(b) {
                    continue;
                }
                let edge = DepEdge {
                    from: format!("{}{}", a.base, a.offset),
                    to: format!("{}{}", b.base, b.offset),
                    distance: (b.offset - a.offset).abs(),
                };
                let is_loop_carried = edge.distance > 0;
                if a.is_write && !b.is_write {
                    info.true_deps.push(edge.clone());
                } else if !a.is_write && b.is_write {
                    info.anti_deps.push(edge.clone());
                } else if a.is_write && b.is_write {
                    info.output_deps.push(edge.clone());
                }
                if is_loop_carried {
                    info.loop_carried_deps.push(edge);
                }
            }
        }
        info
    }
    pub(super) fn collect_accesses(&self, expr: &LcnfExpr) -> Vec<AffineAccess> {
        let mut out = Vec::new();
        self.collect_accesses_inner(expr, &mut out);
        out
    }
    pub(super) fn collect_accesses_inner(&self, expr: &LcnfExpr, out: &mut Vec<AffineAccess>) {
        match expr {
            LcnfExpr::Let {
                value, body, name, ..
            } => {
                match value {
                    LcnfLetValue::App(LcnfArg::Var(fid), args) => {
                        let coeff = if args.len() > 1 { 1 } else { 0 };
                        out.push(AffineAccess {
                            base: name.clone(),
                            coeff,
                            offset: fid.0 as i64,
                            is_write: false,
                        });
                    }
                    LcnfLetValue::Ctor(ctor_name, tag, _) => {
                        out.push(AffineAccess {
                            base: ctor_name.clone(),
                            coeff: 1,
                            offset: *tag as i64,
                            is_write: true,
                        });
                    }
                    _ => {}
                }
                self.collect_accesses_inner(body, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.collect_accesses_inner(&alt.body, out);
                }
                if let Some(d) = default {
                    self.collect_accesses_inner(d, out);
                }
            }
            _ => {}
        }
    }
}
/// Heuristic freshness key for OPar incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OParIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
impl OParIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        OParIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &OParIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
/// A feature flag set for OPar capabilities.
#[derive(Debug, Clone, Default)]
pub struct OParFeatures {
    pub(super) flags: std::collections::HashSet<String>,
}
impl OParFeatures {
    pub fn new() -> Self {
        OParFeatures::default()
    }
    pub fn enable(&mut self, flag: impl Into<String>) {
        self.flags.insert(flag.into());
    }
    pub fn disable(&mut self, flag: &str) {
        self.flags.remove(flag);
    }
    pub fn is_enabled(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }
    pub fn len(&self) -> usize {
        self.flags.len()
    }
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty()
    }
    pub fn union(&self, other: &OParFeatures) -> OParFeatures {
        OParFeatures {
            flags: self.flags.union(&other.flags).cloned().collect(),
        }
    }
    pub fn intersection(&self, other: &OParFeatures) -> OParFeatures {
        OParFeatures {
            flags: self.flags.intersection(&other.flags).cloned().collect(),
        }
    }
}
/// A monotonically increasing ID generator for OPar.
#[derive(Debug, Default)]
pub struct OParIdGen {
    pub(super) next: u32,
}
impl OParIdGen {
    pub fn new() -> Self {
        OParIdGen::default()
    }
    pub fn next_id(&mut self) -> u32 {
        let id = self.next;
        self.next += 1;
        id
    }
    pub fn peek_next(&self) -> u32 {
        self.next
    }
    pub fn reset(&mut self) {
        self.next = 0;
    }
    pub fn skip(&mut self, n: u32) {
        self.next += n;
    }
}
#[allow(dead_code)]
pub struct ParPassRegistry {
    pub(super) configs: Vec<ParPassConfig>,
    pub(super) stats: std::collections::HashMap<String, ParPassStats>,
}
impl ParPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ParPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: ParPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), ParPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ParPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&ParPassStats> {
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
pub struct ParDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl ParDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        ParDominatorTree {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl ParWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ParWorklist {
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
/// Configuration for OParExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OParExtPassConfig {
    pub name: String,
    pub phase: OParExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl OParExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: OParExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: OParExtPassPhase) -> Self {
        self.phase = phase;
        self
    }
    #[allow(dead_code)]
    pub fn with_max_iter(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self, d: u32) -> Self {
        self.debug = d;
        self
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
    #[allow(dead_code)]
    pub fn is_debug_enabled(&self) -> bool {
        self.debug > 0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ParPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl ParPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            ParPassPhase::Analysis => "analysis",
            ParPassPhase::Transformation => "transformation",
            ParPassPhase::Verification => "verification",
            ParPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, ParPassPhase::Transformation | ParPassPhase::Cleanup)
    }
}
/// A single affine access of the form `base + coeff * loop_var + offset`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AffineAccess {
    /// Base array / variable name.
    pub base: String,
    /// Coefficient of the loop induction variable.
    pub coeff: i64,
    /// Constant offset.
    pub offset: i64,
    /// Whether this is a write access.
    pub is_write: bool,
}
impl AffineAccess {
    /// Two accesses are independent under Bernstein's conditions when their
    /// access sets are provably disjoint, i.e., there is no integer `i`, `j`
    /// (with `i != j` for loop-carried deps) satisfying both access functions.
    ///
    /// For 1-D affine accesses `base + c1*i + o1` vs `base + c2*j + o2` we
    /// require `c1 == c2` (same stride) to get a GCD test: `gcd(c1,c2) | (o1-o2)`.
    pub fn independent_from(&self, other: &AffineAccess) -> bool {
        if self.base != other.base {
            return true;
        }
        if !self.is_write && !other.is_write {
            return true;
        }
        let g = gcd(self.coeff.unsigned_abs(), other.coeff.unsigned_abs()) as i64;
        if g == 0 {
            return self.offset != other.offset;
        }
        (self.offset - other.offset) % g != 0
    }
}
/// The concrete algorithmic pattern recognised in a parallel region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParallelPattern {
    /// `out[i] = f(in[i])` — embarrassingly parallel.
    Map,
    /// `out = [x for x in xs if p(x)]` — stream compaction.
    Filter,
    /// `acc = fold(f, init, xs)` — can use tree-reduction.
    Reduce,
    /// `out[i] = prefix_op(out[0..i])` — parallel-prefix.
    Scan,
    /// `out[i] = f(neighbours(in, i))` — finite-difference / convolution.
    Stencil,
    /// Generic counted loop with independent iterations.
    ParallelFor,
    /// Indexed scatter: `out[idx[i]] = val[i]`.
    Scatter,
    /// Indexed gather: `out[i] = in[idx[i]]`.
    Gather,
}
/// Constant folding helper for OParExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OParExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl OParExtConstFolder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            folds: 0,
            failures: 0,
            enabled: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn sub_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn mul_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn div_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn rem_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_rem(b)
        }
    }
    #[allow(dead_code)]
    pub fn neg_i64(&mut self, a: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn shl_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shl(s)
        }
    }
    #[allow(dead_code)]
    pub fn shr_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shr(s)
        }
    }
    #[allow(dead_code)]
    pub fn and_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a & b
    }
    #[allow(dead_code)]
    pub fn or_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a | b
    }
    #[allow(dead_code)]
    pub fn xor_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a ^ b
    }
    #[allow(dead_code)]
    pub fn not_i64(&mut self, a: i64) -> i64 {
        self.folds += 1;
        !a
    }
    #[allow(dead_code)]
    pub fn fold_count(&self) -> usize {
        self.folds
    }
    #[allow(dead_code)]
    pub fn failure_count(&self) -> usize {
        self.failures
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
/// Detects potential data races using a simplified Lamport happened-before model.
///
/// We conservatively flag any pair of accesses to the same variable where at
/// least one is a write and there is no ordering edge between them.
struct RaceDetector {
    /// Known ordering edges: `(a, b)` means `a` happens-before `b`.
    pub(super) happens_before: HashSet<(LcnfVarId, LcnfVarId)>,
}
impl RaceDetector {
    pub(super) fn new() -> Self {
        RaceDetector {
            happens_before: HashSet::new(),
        }
    }
    pub(super) fn add_ordering(&mut self, before: LcnfVarId, after: LcnfVarId) {
        self.happens_before.insert((before, after));
    }
    pub(super) fn may_race(
        &self,
        a: LcnfVarId,
        b: LcnfVarId,
        a_is_write: bool,
        b_is_write: bool,
    ) -> bool {
        if !a_is_write && !b_is_write {
            return false;
        }
        !self.happens_before.contains(&(a, b)) && !self.happens_before.contains(&(b, a))
    }
    pub(super) fn analyse_decl(&self, decl: &LcnfFunDecl) -> ThreadSafetyInfo {
        let mut races = Vec::new();
        let mut atomics = Vec::new();
        let accesses = Self::collect_var_accesses(&decl.body);
        let writes: Vec<_> = accesses
            .iter()
            .filter(|(_, is_write, _)| *is_write)
            .collect();
        let reads: Vec<_> = accesses
            .iter()
            .filter(|(_, is_write, _)| !*is_write)
            .collect();
        for (wid, _, wname) in &writes {
            for (rid, _, rname) in &reads {
                if wid != rid && self.may_race(*wid, *rid, true, false) {
                    races.push((wname.clone(), rname.clone()));
                }
            }
            for (wid2, _, wname2) in &writes {
                if wid != wid2 && self.may_race(*wid, *wid2, true, true) {
                    atomics.push(wname.clone());
                    atomics.push(wname2.clone());
                }
            }
        }
        atomics.sort();
        atomics.dedup();
        ThreadSafetyInfo {
            is_thread_safe: races.is_empty() && atomics.is_empty(),
            race_conditions: races,
            atomic_ops_needed: atomics,
        }
    }
    pub(super) fn collect_var_accesses(expr: &LcnfExpr) -> Vec<(LcnfVarId, bool, String)> {
        let mut out = Vec::new();
        Self::collect_inner(expr, &mut out);
        out
    }
    pub(super) fn collect_inner(expr: &LcnfExpr, out: &mut Vec<(LcnfVarId, bool, String)>) {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                value,
                body,
                ..
            } => {
                let is_write = matches!(
                    value, LcnfLetValue::Ctor(n, _, _) if n.contains("write") || n
                    .contains("store")
                );
                out.push((*id, is_write, name.clone()));
                Self::collect_inner(body, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    Self::collect_inner(&alt.body, out);
                }
                if let Some(d) = default {
                    Self::collect_inner(d, out);
                }
            }
            _ => {}
        }
    }
}
/// A generic key-value configuration store for OPar.
#[derive(Debug, Clone, Default)]
pub struct OParConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
}
impl OParConfig {
    pub fn new() -> Self {
        OParConfig::default()
    }
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.insert(key.into(), value.into());
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(|s| s.as_str())
    }
    pub fn get_bool(&self, key: &str) -> bool {
        matches!(self.get(key), Some("true") | Some("1") | Some("yes"))
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.parse().ok()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ParAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, ParCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl ParAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        ParAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&ParCacheEntry> {
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
            ParCacheEntry {
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
/// Dependency graph for OParExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OParExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl OParExtDepGraph {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
            rev: vec![Vec::new(); n],
            edge_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n && to < self.n {
            if !self.adj[from].contains(&to) {
                self.adj[from].push(to);
                self.rev[to].push(from);
                self.edge_count += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn succs(&self, n: usize) -> &[usize] {
        self.adj.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn preds(&self, n: usize) -> &[usize] {
        self.rev.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn topo_sort(&self) -> Option<Vec<usize>> {
        let mut deg: Vec<usize> = (0..self.n).map(|i| self.rev[i].len()).collect();
        let mut q: std::collections::VecDeque<usize> =
            (0..self.n).filter(|&i| deg[i] == 0).collect();
        let mut out = Vec::with_capacity(self.n);
        while let Some(u) = q.pop_front() {
            out.push(u);
            for &v in &self.adj[u] {
                deg[v] -= 1;
                if deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if out.len() == self.n {
            Some(out)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topo_sort().is_none()
    }
    #[allow(dead_code)]
    pub fn reachable(&self, start: usize) -> Vec<usize> {
        let mut vis = vec![false; self.n];
        let mut stk = vec![start];
        let mut out = Vec::new();
        while let Some(u) = stk.pop() {
            if u < self.n && !vis[u] {
                vis[u] = true;
                out.push(u);
                for &v in &self.adj[u] {
                    if !vis[v] {
                        stk.push(v);
                    }
                }
            }
        }
        out
    }
    #[allow(dead_code)]
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        for i in 0..self.n {
            if !visited[i] {
                let mut stk = vec![(i, 0usize)];
                while let Some((u, idx)) = stk.last_mut() {
                    if !visited[*u] {
                        visited[*u] = true;
                    }
                    if *idx < self.adj[*u].len() {
                        let v = self.adj[*u][*idx];
                        *idx += 1;
                        if !visited[v] {
                            stk.push((v, 0));
                        }
                    } else {
                        order.push(*u);
                        stk.pop();
                    }
                }
            }
        }
        let mut comp = vec![usize::MAX; self.n];
        let mut components: Vec<Vec<usize>> = Vec::new();
        for &start in order.iter().rev() {
            if comp[start] == usize::MAX {
                let cid = components.len();
                let mut component = Vec::new();
                let mut stk = vec![start];
                while let Some(u) = stk.pop() {
                    if comp[u] == usize::MAX {
                        comp[u] = cid;
                        component.push(u);
                        for &v in &self.rev[u] {
                            if comp[v] == usize::MAX {
                                stk.push(v);
                            }
                        }
                    }
                }
                components.push(component);
            }
        }
        components
    }
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.n
    }
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}
/// Emission statistics for OPar.
#[derive(Debug, Clone, Default)]
pub struct OParEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
impl OParEmitStats {
    pub fn new() -> Self {
        OParEmitStats::default()
    }
    pub fn throughput_bps(&self) -> f64 {
        if self.elapsed_ms == 0 {
            0.0
        } else {
            self.bytes_emitted as f64 / (self.elapsed_ms as f64 / 1000.0)
        }
    }
    pub fn is_clean(&self) -> bool {
        self.errors == 0
    }
}
/// A version tag for OPar output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OParVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
impl OParVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        OParVersion {
            major,
            minor,
            patch,
            pre: None,
        }
    }
    pub fn with_pre(mut self, pre: impl Into<String>) -> Self {
        self.pre = Some(pre.into());
        self
    }
    pub fn is_stable(&self) -> bool {
        self.pre.is_none()
    }
    pub fn is_compatible_with(&self, other: &OParVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
/// Tracks declared names for OPar scope analysis.
#[derive(Debug, Default)]
pub struct OParNameScope {
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<OParNameScope>>,
}
impl OParNameScope {
    pub fn new() -> Self {
        OParNameScope::default()
    }
    pub fn declare(&mut self, name: impl Into<String>) -> bool {
        self.declared.insert(name.into())
    }
    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name)
    }
    pub fn push_scope(self) -> Self {
        OParNameScope {
            declared: std::collections::HashSet::new(),
            depth: self.depth + 1,
            parent: Some(Box::new(self)),
        }
    }
    pub fn pop_scope(self) -> Self {
        *self.parent.unwrap_or_default()
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn len(&self) -> usize {
        self.declared.len()
    }
}
/// Pass-timing record for OPar profiler.
#[derive(Debug, Clone)]
pub struct OParPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
impl OParPassTiming {
    pub fn new(
        pass_name: impl Into<String>,
        elapsed_us: u64,
        items: usize,
        before: usize,
        after: usize,
    ) -> Self {
        OParPassTiming {
            pass_name: pass_name.into(),
            elapsed_us,
            items_processed: items,
            bytes_before: before,
            bytes_after: after,
        }
    }
    pub fn throughput_mps(&self) -> f64 {
        if self.elapsed_us == 0 {
            0.0
        } else {
            self.items_processed as f64 / (self.elapsed_us as f64 / 1_000_000.0)
        }
    }
    pub fn size_ratio(&self) -> f64 {
        if self.bytes_before == 0 {
            1.0
        } else {
            self.bytes_after as f64 / self.bytes_before as f64
        }
    }
    pub fn is_profitable(&self) -> bool {
        self.size_ratio() <= 1.05
    }
}
/// Pipeline profiler for OPar.
#[derive(Debug, Default)]
pub struct OParProfiler {
    pub(super) timings: Vec<OParPassTiming>,
}
impl OParProfiler {
    pub fn new() -> Self {
        OParProfiler::default()
    }
    pub fn record(&mut self, t: OParPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&OParPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&OParPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
/// High-level classification of a parallel execution model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParallelKind {
    /// Independent iterations can run simultaneously (SIMD / OpenMP parallel-for).
    DataParallel,
    /// Independent sub-computations (futures / async tasks).
    TaskParallel,
    /// Producer–consumer stages overlap in time.
    PipelineParallel,
    /// Evaluate multiple branches speculatively and discard losers.
    SpeculativeParallel,
}
/// Liveness analysis for OParExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OParExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl OParExtLiveness {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            live_in: vec![Vec::new(); n],
            live_out: vec![Vec::new(); n],
            defs: vec![Vec::new(); n],
            uses: vec![Vec::new(); n],
        }
    }
    #[allow(dead_code)]
    pub fn live_in(&self, b: usize, v: usize) -> bool {
        self.live_in.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn live_out(&self, b: usize, v: usize) -> bool {
        self.live_out
            .get(b)
            .map(|s| s.contains(&v))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, b: usize, v: usize) {
        if let Some(s) = self.defs.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, b: usize, v: usize) {
        if let Some(s) = self.uses.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn var_is_used_in_block(&self, b: usize, v: usize) -> bool {
        self.uses.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn var_is_def_in_block(&self, b: usize, v: usize) -> bool {
        self.defs.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
}
#[allow(dead_code)]
pub struct ParConstantFoldingHelper;
impl ParConstantFoldingHelper {
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
/// Heuristic detector for common parallel patterns over LCNF.
///
/// We walk the function body looking for structural clues:
/// - A tail-recursive function whose accumulator is updated unconditionally
///   is a `Reduce`.
/// - A tail-recursive function that writes an output array at index `i` and
///   reads only from the input at `i` is a `Map`.
/// - Otherwise, if the loop body is side-effect-free we classify as `ParallelFor`.
pub struct PatternDetector<'a> {
    pub(super) decl: &'a LcnfFunDecl,
}
impl<'a> PatternDetector<'a> {
    pub(super) fn new(decl: &'a LcnfFunDecl) -> Self {
        PatternDetector { decl }
    }
    pub(super) fn detect(&self) -> Option<ParallelPattern> {
        if !self.decl.is_recursive {
            return None;
        }
        let reads = self.collect_reads(&self.decl.body);
        let writes = self.collect_writes(&self.decl.body);
        let has_accumulator = self.has_accumulator_update(&self.decl.body);
        let has_index_write = self.has_index_write(&self.decl.body);
        let has_index_read = !reads.is_empty();
        if has_accumulator && !has_index_write {
            return Some(ParallelPattern::Reduce);
        }
        if has_index_write && has_index_read {
            let write_bases: HashSet<&str> = writes.iter().map(|s| s.as_str()).collect();
            let read_bases: HashSet<&str> = reads.iter().map(|s| s.as_str()).collect();
            if write_bases == read_bases {
                return Some(ParallelPattern::Stencil);
            }
            return Some(ParallelPattern::Map);
        }
        if has_index_write && !has_index_read {
            return Some(ParallelPattern::Scatter);
        }
        if !has_index_write && has_index_read {
            return Some(ParallelPattern::Gather);
        }
        if self.has_filter_pattern(&self.decl.body) {
            return Some(ParallelPattern::Filter);
        }
        if self.has_scan_pattern(&self.decl.body) {
            return Some(ParallelPattern::Scan);
        }
        Some(ParallelPattern::ParallelFor)
    }
    pub(super) fn collect_reads(&self, expr: &LcnfExpr) -> Vec<String> {
        let mut out = Vec::new();
        self.collect_reads_inner(expr, &mut out);
        out
    }
    pub(super) fn collect_reads_inner(&self, expr: &LcnfExpr, out: &mut Vec<String>) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                if let LcnfLetValue::App(LcnfArg::Var(id), _args) = value {
                    out.push(format!("read_{}", id.0));
                }
                self.collect_reads_inner(body, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.collect_reads_inner(&alt.body, out);
                }
                if let Some(d) = default {
                    self.collect_reads_inner(d, out);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {}
        }
    }
    pub(super) fn collect_writes(&self, expr: &LcnfExpr) -> Vec<String> {
        let mut out = Vec::new();
        self.collect_writes_inner(expr, &mut out);
        out
    }
    pub(super) fn collect_writes_inner(&self, expr: &LcnfExpr, out: &mut Vec<String>) {
        match expr {
            LcnfExpr::Let {
                value, body, name, ..
            } => {
                if let LcnfLetValue::Ctor(ctor_name, _, _) = value {
                    if ctor_name.contains("write") || ctor_name.contains("store") {
                        out.push(name.clone());
                    }
                }
                self.collect_writes_inner(body, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.collect_writes_inner(&alt.body, out);
                }
                if let Some(d) = default {
                    self.collect_writes_inner(d, out);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::TailCall(_, _) | LcnfExpr::Unreachable => {}
        }
    }
    pub(super) fn has_accumulator_update(&self, expr: &LcnfExpr) -> bool {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                if let LcnfLetValue::App(LcnfArg::Var(fid), args) = value {
                    let is_binop = args.len() == 2;
                    let is_arith = fid.0 < 16;
                    if is_binop && is_arith {
                        return true;
                    }
                }
                self.has_accumulator_update(body)
            }
            LcnfExpr::Case { alts, default, .. } => {
                alts.iter().any(|a| self.has_accumulator_update(&a.body))
                    || default
                        .as_ref()
                        .map(|d| self.has_accumulator_update(d))
                        .unwrap_or(false)
            }
            _ => false,
        }
    }
    pub(super) fn has_index_write(&self, expr: &LcnfExpr) -> bool {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                matches!(
                    value, LcnfLetValue::Ctor(n, _, _) if n.contains("write") || n
                    .contains("store")
                ) || self.has_index_write(body)
            }
            LcnfExpr::Case { alts, default, .. } => {
                alts.iter().any(|a| self.has_index_write(&a.body))
                    || default
                        .as_ref()
                        .map(|d| self.has_index_write(d))
                        .unwrap_or(false)
            }
            _ => false,
        }
    }
    pub(super) fn has_filter_pattern(&self, expr: &LcnfExpr) -> bool {
        matches!(expr, LcnfExpr::Case { alts, .. } if alts.len() == 2)
            || match expr {
                LcnfExpr::Let { body, .. } => self.has_filter_pattern(body),
                _ => false,
            }
    }
    pub(super) fn has_scan_pattern(&self, expr: &LcnfExpr) -> bool {
        self.has_accumulator_update(expr) && self.has_index_write(expr)
    }
}
