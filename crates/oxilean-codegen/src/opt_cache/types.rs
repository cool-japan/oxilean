//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfExpr, LcnfFunDecl, LcnfLetValue};

use std::collections::{HashMap, HashSet, VecDeque};

/// Summary of data locality characteristics for a function or loop body.
#[derive(Debug, Clone)]
pub struct DataLocalityInfo {
    /// All memory accesses captured for this scope.
    pub accesses: Vec<MemoryAccess>,
    /// Estimated working set size in bytes.
    pub working_set_bytes: u64,
    /// The cache level that best accommodates the working set.
    pub best_cache_level: CacheLevel,
    /// Estimated average reuse distance (in number of intervening accesses).
    pub reuse_distance: f64,
}
impl DataLocalityInfo {
    /// Returns `true` if the working set fits in L1 cache.
    pub fn fits_in_l1(&self) -> bool {
        self.working_set_bytes <= CacheLevel::L1.capacity_bytes()
    }
    /// Returns `true` if the working set fits in L2 cache.
    pub fn fits_in_l2(&self) -> bool {
        self.working_set_bytes <= CacheLevel::L2.capacity_bytes()
    }
    /// Returns the fraction of accesses that are cache-friendly.
    pub fn cache_friendly_fraction(&self) -> f64 {
        if self.accesses.is_empty() {
            return 1.0;
        }
        let friendly = self
            .accesses
            .iter()
            .filter(|a| a.is_cache_friendly())
            .count();
        friendly as f64 / self.accesses.len() as f64
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, OCCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl OCAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        OCAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&OCCacheEntry> {
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
            OCCacheEntry {
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
/// Pass execution phase for OCacheExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OCacheExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl OCacheExtPassPhase {
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
/// Top-level configuration for the cache optimization pass.
#[derive(Debug, Clone)]
pub struct CacheOptConfig {
    /// Cache line size in bytes (typically 64 on x86-64).
    pub cache_line_size: u64,
    /// How many iterations ahead to prefetch (in units of loop iterations).
    pub prefetch_distance: u64,
    /// Whether to insert software prefetch hints.
    pub enable_prefetch: bool,
    /// Loop tiling sub-configuration.
    pub tiling: LoopTilingConfig,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl OCWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        OCWorklist {
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
/// Pass execution phase for OCacheX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OCacheX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl OCacheX2PassPhase {
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
/// A summary report produced after the cache optimization pass completes.
#[derive(Debug, Clone, Default)]
pub struct CacheOptReport {
    /// Number of loops that had tiling applied.
    pub num_loops_tiled: usize,
    /// Number of prefetch hints inserted.
    pub num_prefetches_inserted: usize,
    /// Estimated reduction in cache misses as a fraction in [0, 1].
    pub estimated_cache_miss_reduction: f64,
}
impl CacheOptReport {
    /// Returns a human-readable one-line summary of the report.
    pub fn summary(&self) -> String {
        format!(
            "CacheOpt: {} loops tiled, {} prefetches inserted, \
             estimated {:.1}% cache-miss reduction",
            self.num_loops_tiled,
            self.num_prefetches_inserted,
            self.estimated_cache_miss_reduction * 100.0,
        )
    }
}
/// Liveness analysis for OCacheExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OCacheExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl OCacheExtLiveness {
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
/// Pass registry for OCacheExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct OCacheExtPassRegistry {
    pub(super) configs: Vec<OCacheExtPassConfig>,
    pub(super) stats: Vec<OCacheExtPassStats>,
}
impl OCacheExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: OCacheExtPassConfig) {
        self.stats.push(OCacheExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&OCacheExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&OCacheExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&OCacheExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &OCacheExtPassPhase) -> Vec<&OCacheExtPassConfig> {
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
/// Represents a level of the memory hierarchy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CacheLevel {
    /// L1 data cache: typically ~32 KB, ~4 cycle latency.
    L1,
    /// L2 unified cache: typically ~256 KB, ~12 cycle latency.
    L2,
    /// L3 last-level cache: typically ~8 MB, ~40 cycle latency.
    L3,
    /// Main memory (RAM): 100+ cycle latency.
    Ram,
}
impl CacheLevel {
    /// Returns the typical capacity in bytes for this cache level.
    pub fn capacity_bytes(&self) -> u64 {
        match self {
            CacheLevel::L1 => 32 * 1024,
            CacheLevel::L2 => 256 * 1024,
            CacheLevel::L3 => 8 * 1024 * 1024,
            CacheLevel::Ram => u64::MAX,
        }
    }
    /// Returns the typical access latency in cycles for this cache level.
    pub fn latency_cycles(&self) -> u32 {
        match self {
            CacheLevel::L1 => 4,
            CacheLevel::L2 => 12,
            CacheLevel::L3 => 40,
            CacheLevel::Ram => 200,
        }
    }
}
/// Constant folding helper for OCacheExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OCacheExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl OCacheExtConstFolder {
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
/// Analysis cache for OCacheExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct OCacheExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl OCacheExtCache {
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
/// Configuration for OCacheX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCacheX2PassConfig {
    pub name: String,
    pub phase: OCacheX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl OCacheX2PassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: OCacheX2PassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: OCacheX2PassPhase) -> Self {
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
#[derive(Debug, Clone)]
pub struct OCPassConfig {
    pub phase: OCPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl OCPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: OCPassPhase) -> Self {
        OCPassConfig {
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
/// Configuration parameters for loop tiling (cache blocking).
#[derive(Debug, Clone)]
pub struct LoopTilingConfig {
    /// Tile size targeting L1 cache reuse, in elements.
    pub tile_size_l1: u64,
    /// Tile size targeting L2 cache reuse, in elements.
    pub tile_size_l2: u64,
    /// Whether to apply L1-level tiling.
    pub enable_l1_tiling: bool,
    /// Whether to apply L2-level tiling.
    pub enable_l2_tiling: bool,
}
/// Describes the memory layout of a struct, used for field reordering analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct StructLayout {
    /// The name of the struct.
    pub struct_name: String,
    /// Ordered list of (field_name, field_size_bytes) pairs.
    pub fields: Vec<(String, u64)>,
    /// Total size of the struct in bytes (including padding).
    pub total_size: u64,
    /// Required alignment of the struct in bytes.
    pub alignment: u64,
}
impl StructLayout {
    /// Returns `true` if the struct's total size is a multiple of the cache line size (64 bytes).
    pub fn is_cache_aligned(&self) -> bool {
        const CACHE_LINE: u64 = 64;
        self.total_size.is_multiple_of(CACHE_LINE) && self.alignment >= CACHE_LINE
    }
    /// Returns the number of padding bytes in the struct.
    pub fn padding_bytes(&self) -> u64 {
        let fields_total: u64 = self.fields.iter().map(|(_, sz)| sz).sum();
        self.total_size.saturating_sub(fields_total)
    }
    /// Returns the number of cache lines the struct spans.
    pub fn cache_lines_used(&self) -> u64 {
        const CACHE_LINE: u64 = 64;
        self.total_size.div_ceil(CACHE_LINE)
    }
}
/// Configuration for OCacheExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCacheExtPassConfig {
    pub name: String,
    pub phase: OCacheExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl OCacheExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: OCacheExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: OCacheExtPassPhase) -> Self {
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
/// Analyzes struct/constructor layouts in an LCNF function and produces
/// `StructLayout` records that describe each distinct constructor shape.
pub struct FieldReorderingAnalysis;
impl FieldReorderingAnalysis {
    /// Inspect `decl` and collect layout information for every constructor
    /// referenced in the body.  The heuristic assigns 8 bytes per field
    /// (pointer-sized) and rounds the total up to the nearest 8-byte boundary.
    pub fn analyze(decl: &LcnfFunDecl) -> Vec<StructLayout> {
        let mut layouts: Vec<StructLayout> = Vec::new();
        Self::collect_layouts(&decl.body, &mut layouts);
        layouts
    }
    pub(super) fn collect_layouts(expr: &LcnfExpr, out: &mut Vec<StructLayout>) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                if let LcnfLetValue::Ctor(name, _tag, args) = value {
                    if !out.iter().any(|l| &l.struct_name == name) {
                        let field_count = args.len() as u64;
                        let fields: Vec<(String, u64)> = (0..field_count)
                            .map(|i| (format!("field_{}", i), 8u64))
                            .collect();
                        let fields_total = field_count * 8;
                        let total_size = if fields_total == 0 {
                            8
                        } else {
                            fields_total.next_multiple_of(8)
                        };
                        out.push(StructLayout {
                            struct_name: name.clone(),
                            fields,
                            total_size,
                            alignment: 8,
                        });
                    }
                }
                Self::collect_layouts(body, out);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    Self::collect_layouts(&alt.body, out);
                }
                if let Some(def) = default {
                    Self::collect_layouts(def, out);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl OCDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        OCDominatorTree {
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
pub struct OCConstantFoldingHelper;
impl OCConstantFoldingHelper {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Constant folding helper for OCacheX2.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OCacheX2ConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl OCacheX2ConstFolder {
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
/// Dependency graph for OCacheExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCacheExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl OCacheExtDepGraph {
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
/// Statistics for OCacheX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OCacheX2PassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl OCacheX2PassStats {
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
    pub fn merge(&mut self, o: &OCacheX2PassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl OCLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        OCLivenessInfo {
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
/// Worklist for OCacheExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCacheExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl OCacheExtWorklist {
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
/// Dependency graph for OCacheX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCacheX2DepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl OCacheX2DepGraph {
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
/// Pass registry for OCacheX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct OCacheX2PassRegistry {
    pub(super) configs: Vec<OCacheX2PassConfig>,
    pub(super) stats: Vec<OCacheX2PassStats>,
}
impl OCacheX2PassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: OCacheX2PassConfig) {
        self.stats.push(OCacheX2PassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&OCacheX2PassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&OCacheX2PassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&OCacheX2PassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &OCacheX2PassPhase) -> Vec<&OCacheX2PassConfig> {
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
/// Describes how a single loop variable should be tiled.
#[derive(Debug, Clone, PartialEq)]
pub struct LoopTile {
    /// The original (un-tiled) loop variable name.
    pub original_var: String,
    /// The outer (tile-index) variable name generated by tiling.
    pub tile_var: String,
    /// The inner (intra-tile) variable name generated by tiling.
    pub intra_var: String,
    /// The tile size in elements.
    pub tile_size: u64,
}
impl LoopTile {
    /// Creates a new `LoopTile` with auto-derived variable names.
    pub fn new(original_var: impl Into<String>, tile_size: u64) -> Self {
        let original = original_var.into();
        let tile_var = format!("{}_tile", original);
        let intra_var = format!("{}_intra", original);
        LoopTile {
            original_var: original,
            tile_var,
            intra_var,
            tile_size,
        }
    }
}
/// A single memory access event captured during analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct MemoryAccess {
    /// The variable being accessed.
    pub var_name: String,
    /// Byte offset from the base of the variable (0 = base address).
    pub offset: i64,
    /// The access pattern observed or inferred.
    pub pattern: AccessPattern,
    /// For strided accesses, the stride in bytes.
    pub stride: Option<i64>,
    /// Estimated number of times this access occurs.
    pub count: u64,
}
impl MemoryAccess {
    /// Creates a new `MemoryAccess` with the given parameters.
    pub fn new(
        var_name: impl Into<String>,
        offset: i64,
        pattern: AccessPattern,
        stride: Option<i64>,
        count: u64,
    ) -> Self {
        MemoryAccess {
            var_name: var_name.into(),
            offset,
            pattern,
            stride,
            count,
        }
    }
    /// Returns `true` if this access is likely to produce cache hits.
    pub fn is_cache_friendly(&self) -> bool {
        self.pattern.is_cache_friendly()
    }
}
/// The kind of memory access the prefetch prepares for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrefetchType {
    /// Prefetch for a future read.
    Read,
    /// Prefetch for a future write (exclusive ownership hint).
    Write,
    /// Non-temporal prefetch: bypass caches (for streaming data not reused).
    NonTemporal,
}
/// Statistics for OCacheExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct OCacheExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl OCacheExtPassStats {
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
    pub fn merge(&mut self, o: &OCacheExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Dominator tree for OCacheX2.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OCacheX2DomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
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
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
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
    pub(super) configs: Vec<OCPassConfig>,
    pub(super) stats: std::collections::HashMap<String, OCPassStats>,
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
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
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
    pub(super) fn collect_accesses(expr: &LcnfExpr, out: &mut Vec<MemoryAccess>) {
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
    pub(super) fn estimate_working_set(&self, accesses: &[MemoryAccess]) -> u64 {
        use std::collections::HashSet;
        let distinct: HashSet<&str> = accesses.iter().map(|a| a.var_name.as_str()).collect();
        (distinct.len() as u64) * self.config.cache_line_size
    }
    /// Classifies the working set into the most appropriate cache level.
    pub(super) fn classify_cache_level(&self, working_set_bytes: u64) -> CacheLevel {
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
    pub(super) fn compute_reuse_distance(&self, accesses: &[MemoryAccess]) -> f64 {
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
    pub(super) fn propose_tiles(
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
    pub(super) fn annotate_tiling(expr: &mut LcnfExpr, tiles: &[LoopTile]) {
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
    pub(super) fn generate_prefetch_hints(&self, info: &DataLocalityInfo) -> Vec<PrefetchHint> {
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
    pub(super) fn insert_prefetch_hints(&self, decl: &mut LcnfFunDecl, hints: &[PrefetchHint]) {
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
    pub(super) fn estimate_miss_reduction(&self, decls: &[LcnfFunDecl]) -> f64 {
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
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
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
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
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
