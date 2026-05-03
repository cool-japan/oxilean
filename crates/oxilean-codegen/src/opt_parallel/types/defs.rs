//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfVarId};
use std::collections::{HashMap, HashSet};

use super::super::functions::*;
use super::impls1::*;
use super::impls2::*;
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
    pub(crate) msgs: Vec<OParDiagMsg>,
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
    pub(crate) safety_cache: HashMap<String, ThreadSafetyInfo>,
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
    pub(crate) fn analyse_decl(&mut self, decl: &LcnfFunDecl) -> Option<ParallelRegion> {
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
    pub(crate) fn infer_kind(&self, pattern: ParallelPattern, _decl: &LcnfFunDecl) -> ParallelKind {
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
    pub(crate) fn estimate_trip_count(&self, decl: &LcnfFunDecl) -> Option<u64> {
        for param in &decl.params {
            if param.name == "n" || param.name == "len" || param.name == "size" {
                return Some(256);
            }
        }
        None
    }
    pub(crate) fn collect_shared_vars(&self, decl: &LcnfFunDecl) -> Vec<String> {
        decl.params
            .iter()
            .filter(|p| p.name.starts_with("arr") || p.name.starts_with("buf"))
            .map(|p| p.name.clone())
            .collect()
    }
    pub(crate) fn collect_private_vars(&self, decl: &LcnfFunDecl) -> Vec<String> {
        let mut privates = Vec::new();
        Self::collect_let_names(&decl.body, &mut privates);
        privates
    }
    pub(crate) fn collect_let_names(expr: &LcnfExpr, out: &mut Vec<String>) {
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
    pub(crate) configs: Vec<OParExtPassConfig>,
    pub(crate) stats: Vec<OParExtPassStats>,
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
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
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
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
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
