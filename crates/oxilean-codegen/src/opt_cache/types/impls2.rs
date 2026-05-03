//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls1::*;
use crate::lcnf::{LcnfExpr, LcnfFunDecl, LcnfLetValue};

use std::collections::{HashMap, HashSet, VecDeque};

/// Summary of data locality characteristics for a function or loop body.
/// Dominator tree for OCacheX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCacheX2DomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}
impl OCacheX2DomTree {
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
pub struct OCDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}
impl OCDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OCDepGraph {
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
/// A software prefetch hint to be emitted before the actual access.
#[derive(Debug, Clone, PartialEq)]
pub struct PrefetchHint {
    /// Expression string representing the address to prefetch.
    pub address_expr: String,
    /// How many iterations ahead this prefetch is issued.
    pub distance: u64,
    /// The type of prefetch hint.
    pub hint_type: PrefetchType,
}
impl PrefetchHint {
    /// Creates a new `PrefetchHint`.
    pub fn new(address_expr: impl Into<String>, distance: u64, hint_type: PrefetchType) -> Self {
        PrefetchHint {
            address_expr: address_expr.into(),
            distance,
            hint_type,
        }
    }
}
#[allow(dead_code)]
pub struct OCPassRegistry {
    pub(crate) configs: Vec<OCPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, OCPassStats>,
}
impl OCPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OCPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: OCPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), OCPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&OCPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&OCPassStats> {
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
/// Analysis cache for OCacheX2.
#[allow(dead_code)]
#[derive(Debug)]
pub struct OCacheX2Cache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl OCacheX2Cache {
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
/// The top-level cache-aware / data-locality optimization pass.
///
/// Usage:
/// ```rust
/// use oxilean_codegen::opt_cache::{CacheOptConfig, CacheOptPass};
/// let mut pass = CacheOptPass::new(CacheOptConfig::default());
/// // pass.run(&mut decls);
/// ```
pub struct CacheOptPass {
    /// Configuration for this pass.
    pub config: CacheOptConfig,
    /// Accumulated report from the last `run` call.
    pub report: CacheOptReport,
}
impl CacheOptPass {
    /// Creates a new `CacheOptPass` with the given configuration.
    pub fn new(config: CacheOptConfig) -> Self {
        CacheOptPass {
            config,
            report: CacheOptReport::default(),
        }
    }
    /// Runs all cache optimizations over the provided function declarations.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        self.report = CacheOptReport::default();
        for decl in decls.iter_mut() {
            let info = self.analyze_locality(decl);
            if self.config.tiling.enable_l1_tiling && !info.fits_in_l1() {
                let tiles = self.propose_tiles(decl, &info);
                if !tiles.is_empty() {
                    let n = tiles.len();
                    self.apply_loop_tiling(decl, &tiles);
                    self.report.num_loops_tiled += n;
                }
            }
            if self.config.enable_prefetch && info.reuse_distance > 8.0 {
                let hints = self.generate_prefetch_hints(&info);
                let n = hints.len();
                self.insert_prefetch_hints(decl, &hints);
                self.report.num_prefetches_inserted += n;
            }
            self.reorder_data_structures(decl);
        }
        if !decls.is_empty() {
            self.report.estimated_cache_miss_reduction = self.estimate_miss_reduction(decls);
        }
    }
    /// Analyses data locality for a single function declaration.
    pub fn analyze_locality(&self, decl: &LcnfFunDecl) -> DataLocalityInfo {
        let mut accesses: Vec<MemoryAccess> = Vec::new();
        Self::collect_accesses(&decl.body, &mut accesses);
        let working_set_bytes = self.estimate_working_set(&accesses);
        let best_cache_level = self.classify_cache_level(working_set_bytes);
        let reuse_distance = self.compute_reuse_distance(&accesses);
        DataLocalityInfo {
            accesses,
            working_set_bytes,
            best_cache_level,
            reuse_distance,
        }
    }
    /// Recursively collects `MemoryAccess` records from an LCNF expression tree.
    pub(crate) fn collect_accesses(expr: &LcnfExpr, out: &mut Vec<MemoryAccess>) {
        match expr {
            LcnfExpr::Let {
                value, body, id, ..
            } => {
                match value {
                    LcnfLetValue::Proj(field, idx, src) => {
                        out.push(MemoryAccess::new(
                            format!("{}.{}", src, field),
                            (*idx as i64) * 8,
                            AccessPattern::Sequential,
                            Some(8),
                            1,
                        ));
                    }
                    LcnfLetValue::Ctor(name, _, args) => {
                        for (i, _arg) in args.iter().enumerate() {
                            out.push(MemoryAccess::new(
                                format!("{}#{}", name, id),
                                (i as i64) * 8,
                                AccessPattern::Sequential,
                                Some(8),
                                1,
                            ));
                        }
                    }
                    LcnfLetValue::App(_, args) => {
                        for (i, _arg) in args.iter().enumerate() {
                            out.push(MemoryAccess::new(
                                format!("arg_{}", i),
                                0,
                                AccessPattern::Random,
                                None,
                                1,
                            ));
                        }
                    }
                    _ => {}
                }
                Self::collect_accesses(body, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    Self::collect_accesses(&alt.body, out);
                }
                if let Some(def) = default {
                    Self::collect_accesses(def, out);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
        }
    }
    /// Estimates the working set size from the collected accesses.
    pub(crate) fn estimate_working_set(&self, accesses: &[MemoryAccess]) -> u64 {
        use std::collections::HashSet;
        let distinct: HashSet<&str> = accesses.iter().map(|a| a.var_name.as_str()).collect();
        (distinct.len() as u64) * self.config.cache_line_size
    }
    /// Classifies the working set into the most appropriate cache level.
    pub(crate) fn classify_cache_level(&self, working_set_bytes: u64) -> CacheLevel {
        if working_set_bytes <= CacheLevel::L1.capacity_bytes() {
            CacheLevel::L1
        } else if working_set_bytes <= CacheLevel::L2.capacity_bytes() {
            CacheLevel::L2
        } else if working_set_bytes <= CacheLevel::L3.capacity_bytes() {
            CacheLevel::L3
        } else {
            CacheLevel::Ram
        }
    }
    /// Computes a heuristic reuse distance from the access list.
    ///
    /// Reuse distance is approximated as the average number of distinct
    /// variables accessed between consecutive accesses to the same variable.
    pub(crate) fn compute_reuse_distance(&self, accesses: &[MemoryAccess]) -> f64 {
        if accesses.len() < 2 {
            return 0.0;
        }
        use std::collections::HashMap;
        let mut last_seen: HashMap<&str, usize> = HashMap::new();
        let mut total_distance: f64 = 0.0;
        let mut reuse_count: usize = 0;
        for (i, acc) in accesses.iter().enumerate() {
            if let Some(&prev) = last_seen.get(acc.var_name.as_str()) {
                total_distance += (i - prev) as f64;
                reuse_count += 1;
            }
            last_seen.insert(&acc.var_name, i);
        }
        if reuse_count == 0 {
            accesses.len() as f64
        } else {
            total_distance / reuse_count as f64
        }
    }
    /// Proposes a set of loop tiles based on locality analysis.
    pub(crate) fn propose_tiles(
        &self,
        decl: &LcnfFunDecl,
        info: &DataLocalityInfo,
    ) -> Vec<LoopTile> {
        let mut tiles = Vec::new();
        for param in &decl.params {
            let used_in_sequential = info.accesses.iter().any(|a| {
                a.var_name.contains(&param.name) && matches!(a.pattern, AccessPattern::Sequential)
            });
            if used_in_sequential {
                let tile_size = if info.fits_in_l1() {
                    self.config.tiling.tile_size_l1
                } else {
                    self.config.tiling.tile_size_l2
                };
                tiles.push(LoopTile::new(&param.name, tile_size));
            }
        }
        tiles
    }
    /// Applies loop tiling annotations to a function declaration.
    ///
    /// In the LCNF IR there are no explicit loop constructs, so this pass
    /// records the tiling decision metadata for downstream backends to act on.
    /// The body is traversed and `Proj` accesses on tiled variables are
    /// annotated via a comment in the surrounding let binding name hint.
    pub fn apply_loop_tiling(&self, decl: &mut LcnfFunDecl, tiles: &[LoopTile]) {
        Self::annotate_tiling(&mut decl.body, tiles);
    }
    pub(crate) fn annotate_tiling(expr: &mut LcnfExpr, tiles: &[LoopTile]) {
        match expr {
            LcnfExpr::Let {
                name, value, body, ..
            } => {
                if tiles.iter().any(|t| name.contains(&t.original_var)) && !name.ends_with("_tiled")
                {
                    *name = format!("{}_tiled", name);
                }
                if let LcnfLetValue::Proj(field, _idx, src) = value {
                    if tiles
                        .iter()
                        .any(|t| src.to_string().contains(&t.original_var))
                    {
                        *field = format!("{}_tile_cached", field);
                    }
                }
                Self::annotate_tiling(body, tiles);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    Self::annotate_tiling(&mut alt.body, tiles);
                }
                if let Some(def) = default {
                    Self::annotate_tiling(def, tiles);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
        }
    }
    /// Generates software prefetch hints based on locality info.
    pub(crate) fn generate_prefetch_hints(&self, info: &DataLocalityInfo) -> Vec<PrefetchHint> {
        let mut hints = Vec::new();
        for acc in &info.accesses {
            if !acc.is_cache_friendly() {
                hints.push(PrefetchHint::new(
                    format!("&{}[{}]", acc.var_name, self.config.prefetch_distance),
                    self.config.prefetch_distance,
                    PrefetchType::Read,
                ));
            } else if matches!(acc.pattern, AccessPattern::Sequential) && acc.count > 8 {
                hints.push(PrefetchHint::new(
                    format!("&{}[{}]", acc.var_name, self.config.prefetch_distance),
                    self.config.prefetch_distance,
                    PrefetchType::NonTemporal,
                ));
            }
        }
        hints
    }
    /// Records prefetch hint metadata in the function declaration name hint.
    pub(crate) fn insert_prefetch_hints(&self, decl: &mut LcnfFunDecl, hints: &[PrefetchHint]) {
        if hints.is_empty() {
            return;
        }
        let annotation = format!("__prefetch_{}", hints.len());
        if !decl.name.contains(&annotation) {
            decl.name = format!("{}{}", decl.name, annotation);
        }
    }
    /// Reorders struct fields for improved spatial locality.
    ///
    /// Uses `FieldReorderingAnalysis` to collect layout information, then
    /// records the reordering decision as metadata on the declaration.
    pub fn reorder_data_structures(&self, decl: &mut LcnfFunDecl) {
        let layouts = FieldReorderingAnalysis::analyze(decl);
        for layout in &layouts {
            if layout.padding_bytes() > 0 || !layout.is_cache_aligned() {
                let annotation = format!("__reorder_{}", layout.struct_name);
                if !decl.name.contains(&annotation) {
                    decl.name = format!("{}{}", decl.name, annotation);
                }
            }
        }
    }
    /// Estimates the overall cache miss reduction across all declarations.
    pub(crate) fn estimate_miss_reduction(&self, decls: &[LcnfFunDecl]) -> f64 {
        if decls.is_empty() {
            return 0.0;
        }
        let total_friendly: f64 = decls
            .iter()
            .map(|d| {
                let info = self.analyze_locality(d);
                info.cache_friendly_fraction()
            })
            .sum();
        let avg_friendly = total_friendly / decls.len() as f64;
        let tiling_factor = if self.config.tiling.enable_l1_tiling {
            0.4
        } else {
            0.2
        };
        (1.0 - avg_friendly) * tiling_factor
    }
}
/// Liveness analysis for OCacheX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OCacheX2Liveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl OCacheX2Liveness {
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
/// Dominator tree for OCacheExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCacheExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}
impl OCacheExtDomTree {
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
#[derive(Debug, Clone, Default)]
pub struct OCPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl OCPassStats {
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
/// Worklist for OCacheX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCacheX2Worklist {
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
}
impl OCacheX2Worklist {
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
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum OCPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl OCPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            OCPassPhase::Analysis => "analysis",
            OCPassPhase::Transformation => "transformation",
            OCPassPhase::Verification => "verification",
            OCPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, OCPassPhase::Transformation | OCPassPhase::Cleanup)
    }
}
/// Describes how memory is accessed in a loop or computation.
#[derive(Debug, Clone, PartialEq)]
pub enum AccessPattern {
    /// Sequential access: elements accessed one after another (a\[0\], a\[1\], ...).
    Sequential,
    /// Strided access: elements accessed with a fixed stride (a\[0\], a\[s\], a\[2s\], ...).
    Strided(i64),
    /// Random (irregular) access: no discernible pattern.
    Random,
    /// Broadcast: same address read many times.
    Broadcast,
}
impl AccessPattern {
    /// Returns `true` if the pattern is cache-friendly (sequential or small stride).
    pub fn is_cache_friendly(&self) -> bool {
        match self {
            AccessPattern::Sequential => true,
            AccessPattern::Broadcast => true,
            AccessPattern::Strided(s) => s.unsigned_abs() <= 64,
            AccessPattern::Random => false,
        }
    }
}
