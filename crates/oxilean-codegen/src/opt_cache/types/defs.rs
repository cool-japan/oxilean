//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::impls1::*;
use super::impls2::*;
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
    pub(crate) entries: std::collections::HashMap<String, OCCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
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
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
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
    pub(crate) configs: Vec<OCacheExtPassConfig>,
    pub(crate) stats: Vec<OCacheExtPassStats>,
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
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
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
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
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
    pub(crate) fn collect_layouts(expr: &LcnfExpr, out: &mut Vec<StructLayout>) {
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
