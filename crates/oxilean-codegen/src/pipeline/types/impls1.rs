//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls2::*;
use crate::c_backend::{self, CEmitConfig, COutput};
use crate::closure_convert::{ClosureConvertConfig, ClosureConverter};
use crate::lcnf::*;
use crate::native_backend::{self, NativeEmitConfig, NativeModule};
use crate::opt_dce::{self, DceConfig};
use crate::to_lcnf::{self, ToLcnfConfig};
use crate::CodegenTarget;
use oxilean_kernel::expr::Expr;
use oxilean_kernel::Name;

use super::super::functions::LcnfDeclInput;

use super::super::functions::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PipePassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl PipePassStats {
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
/// Pass registry for PipeExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct PipeExtPassRegistry {
    pub(crate) configs: Vec<PipeExtPassConfig>,
    pub(crate) stats: Vec<PipeExtPassStats>,
}
impl PipeExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: PipeExtPassConfig) {
        self.stats.push(PipeExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&PipeExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&PipeExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&PipeExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &PipeExtPassPhase) -> Vec<&PipeExtPassConfig> {
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
/// Pass execution phase for PipeX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PipeX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl PipeX2PassPhase {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PipeWorklist {
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
}
impl PipeWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PipeWorklist {
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
#[allow(dead_code)]
pub struct PipeConstantFoldingHelper;
impl PipeConstantFoldingHelper {
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
/// The main compiler pipeline.
///
/// Orchestrates the transformation from kernel expressions through
/// LCNF optimization to backend code generation.
pub struct CompilerPipeline {
    pub(crate) config: PipelineConfig,
}
impl CompilerPipeline {
    /// Create a new pipeline with the given configuration.
    pub fn new(config: PipelineConfig) -> Self {
        CompilerPipeline { config }
    }
    /// Create a pipeline with default (O1) configuration.
    pub fn default_pipeline() -> Self {
        Self::new(PipelineConfig::default())
    }
    /// Run the complete pipeline on a set of declarations.
    ///
    /// Input: list of (name, type, value) triples from the kernel.
    /// Output: PipelineResult with generated code and statistics.
    pub fn run_pipeline(
        &self,
        input: Vec<(Name, Expr, Expr)>,
        config: &PipelineConfig,
    ) -> PipelineResult {
        let start_time = std::time::Instant::now();
        let mut stats = PipelineStats::default();
        let mut module = self.exprs_to_lcnf(&input);
        stats.input_decls = module.fun_decls.len();
        let passes = config.effective_passes();
        let max_iter = config.effective_max_iterations();
        if !passes.is_empty() {
            module = self.iterate_to_fixpoint(module, &passes, max_iter, &mut stats);
        }
        stats.output_decls = module.fun_decls.len();
        let mut result = PipelineResult {
            c_output: None,
            native_output: None,
            lcnf_module: module.clone(),
            stats: stats.clone(),
        };
        match config.target {
            CodegenTarget::C => {
                let c_config = CEmitConfig {
                    emit_comments: config.emit_comments,
                    ..CEmitConfig::default()
                };
                let c_output = c_backend::compile_to_c(&module, c_config);
                result.c_output = Some(c_output);
            }
            CodegenTarget::LlvmIr | CodegenTarget::Rust => {
                let native_config = NativeEmitConfig {
                    opt_level: config.opt_level.to_u8(),
                    debug_info: config.debug,
                    emit_comments: config.emit_comments,
                    ..NativeEmitConfig::default()
                };
                let mut backend = native_backend::NativeBackend::new(native_config);
                let native_module = backend.compile_module(&module);
                result.native_output = Some(native_module);
            }
            CodegenTarget::Interpreter => {}
        }
        result.stats.total_time_us = start_time.elapsed().as_micros() as u64;
        result
    }
    /// Convert kernel expressions to an LCNF module.
    ///
    /// Each triple is `(name, type, value)`.  We peel off leading `Lam`
    /// binders from `value` to recover explicit function parameters, then
    /// delegate to `to_lcnf::module_to_lcnf` for the actual conversion.
    pub(crate) fn exprs_to_lcnf(&self, input: &[(Name, Expr, Expr)]) -> LcnfModule {
        if input.is_empty() {
            return LcnfModule {
                fun_decls: Vec::new(),
                extern_decls: Vec::new(),
                name: "compiled_module".to_string(),
                metadata: LcnfModuleMetadata::default(),
            };
        }
        let config = ToLcnfConfig::default();
        let decls: Vec<LcnfDeclInput> = input
            .iter()
            .map(|(name, _ty, value)| {
                let (params, body) = peel_lam_params(value);
                (name.clone(), params, body)
            })
            .collect();
        match to_lcnf::module_to_lcnf(&decls, &config) {
            Ok(mut module) => {
                module.name = "compiled_module".to_string();
                module
            }
            Err(_err) => LcnfModule {
                fun_decls: Vec::new(),
                extern_decls: Vec::new(),
                name: "compiled_module".to_string(),
                metadata: LcnfModuleMetadata::default(),
            },
        }
    }
    /// Run optimization passes in a fixed-point loop until convergence.
    pub fn iterate_to_fixpoint(
        &self,
        mut module: LcnfModule,
        passes: &[PassId],
        max_iters: usize,
        stats: &mut PipelineStats,
    ) -> LcnfModule {
        for _iteration in 0..max_iters {
            stats.iterations += 1;
            let mut any_changed = false;
            for pass_id in passes {
                let result = self.run_pass(&module, pass_id);
                stats.per_pass.push((pass_id.clone(), result.stats));
                if result.changed {
                    any_changed = true;
                }
                module = result.module;
            }
            if !any_changed {
                break;
            }
        }
        module
    }
    /// Run a single optimization pass on the module.
    pub fn run_pass(&self, module: &LcnfModule, pass_id: &PassId) -> PassResult {
        let start = std::time::Instant::now();
        let before_count = count_module_lets(module);
        let (new_module, transformations) = match pass_id {
            PassId::Dce => {
                let dce_config = DceConfig {
                    max_iterations: 3,
                    ..DceConfig::default()
                };
                let (result, dce_stats) = opt_dce::optimize_dce(module, &dce_config);
                (result, dce_stats.total_changes())
            }
            PassId::JoinPoints => {
                let result = run_join_point_pass(module);
                let after_count = count_module_lets(&result);
                let changes = before_count.abs_diff(after_count);
                (result, changes)
            }
            PassId::Specialize => {
                let result = run_specialize_pass(module);
                let after_count = count_module_lets(&result);
                let changes = before_count.abs_diff(after_count);
                (result, changes)
            }
            PassId::Reuse => {
                let result = run_reuse_pass(module);
                let after_count = count_module_lets(&result);
                let changes = before_count.abs_diff(after_count);
                (result, changes)
            }
            PassId::ClosureConvert => {
                let mut result = module.clone();
                let cc_config = ClosureConvertConfig::default();
                let mut converter = ClosureConverter::new(cc_config);
                converter.convert_module(&mut result);
                let cc_stats = converter.stats();
                (result, cc_stats.closures_converted)
            }
            PassId::Custom(_name) => (module.clone(), 0),
        };
        let elapsed = start.elapsed().as_micros() as u64;
        let changed = transformations > 0;
        PassResult {
            module: new_module,
            stats: PassStats {
                decls_processed: module.fun_decls.len(),
                transformations,
                time_us: elapsed,
            },
            changed,
        }
    }
}
/// Statistics for PipeExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PipeExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl PipeExtPassStats {
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
    pub fn merge(&mut self, o: &PipeExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Analysis cache for PipeExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct PipeExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl PipeExtCache {
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
/// Optimization level, modeled after GCC/Clang conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptLevel {
    /// No optimization: emit LCNF as-is.
    O0,
    /// Basic optimization: DCE, constant propagation.
    O1,
    /// Full optimization: all passes.
    O2,
    /// Aggressive optimization: all passes with high iteration counts.
    O3,
}
impl OptLevel {
    /// Convert to a numeric level (0-3).
    pub fn to_u8(self) -> u8 {
        match self {
            OptLevel::O0 => 0,
            OptLevel::O1 => 1,
            OptLevel::O2 => 2,
            OptLevel::O3 => 3,
        }
    }
    /// Default passes for this optimization level.
    pub fn default_passes(self) -> Vec<PassId> {
        match self {
            OptLevel::O0 => vec![],
            OptLevel::O1 => vec![PassId::Dce],
            OptLevel::O2 => {
                vec![
                    PassId::Dce,
                    PassId::JoinPoints,
                    PassId::Specialize,
                    PassId::ClosureConvert,
                    PassId::Dce,
                ]
            }
            OptLevel::O3 => {
                vec![
                    PassId::Dce,
                    PassId::JoinPoints,
                    PassId::Specialize,
                    PassId::Reuse,
                    PassId::ClosureConvert,
                    PassId::Dce,
                    PassId::JoinPoints,
                    PassId::Dce,
                ]
            }
        }
    }
    /// Maximum fixed-point iterations for this level.
    pub fn max_iterations(self) -> usize {
        match self {
            OptLevel::O0 => 0,
            OptLevel::O1 => 3,
            OptLevel::O2 => 5,
            OptLevel::O3 => 10,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PipeDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}
impl PipeDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PipeDepGraph {
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
/// A summary of what the pipeline changed.
#[derive(Debug, Clone, Default)]
pub struct PipelineChangeSummary {
    /// Passes that made at least one change.
    pub active_passes: Vec<String>,
    /// Passes that made no changes (converged immediately).
    pub converged_passes: Vec<String>,
    /// Total let-bindings eliminated.
    pub lets_eliminated: usize,
}
impl PipelineChangeSummary {
    /// Create an empty summary.
    pub fn new() -> Self {
        Self::default()
    }
    /// Mark a pass as active (made changes).
    pub fn mark_active(&mut self, pass: &str) {
        self.active_passes.push(pass.to_string());
    }
    /// Mark a pass as converged (no changes).
    pub fn mark_converged(&mut self, pass: &str) {
        self.converged_passes.push(pass.to_string());
    }
    /// Whether any pass made changes.
    pub fn any_changed(&self) -> bool {
        !self.active_passes.is_empty()
    }
}
/// Configuration for PipeExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PipeExtPassConfig {
    pub name: String,
    pub phase: PipeExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl PipeExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: PipeExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: PipeExtPassPhase) -> Self {
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
