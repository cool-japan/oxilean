//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::functions::*;
#[allow(unused_imports)]
use super::impls::*;
use oxilean_kernel::Expr;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Aggregated statistics from multiple caches.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CacheStatsExt {
    pub total_queries: usize,
    pub total_hits: usize,
    pub total_misses: usize,
    pub total_inserts: usize,
    pub evictions: usize,
}
#[allow(dead_code)]
impl CacheStatsExt {
    pub fn new() -> Self {
        CacheStatsExt::default()
    }
    pub fn record_hit(&mut self) {
        self.total_queries += 1;
        self.total_hits += 1;
    }
    pub fn record_miss(&mut self) {
        self.total_queries += 1;
        self.total_misses += 1;
    }
    pub fn record_insert(&mut self) {
        self.total_inserts += 1;
    }
    pub fn record_eviction(&mut self) {
        self.evictions += 1;
    }
    pub fn hit_rate(&self) -> f64 {
        if self.total_queries == 0 {
            0.0
        } else {
            self.total_hits as f64 / self.total_queries as f64
        }
    }
    pub fn is_warm(&self) -> bool {
        self.hit_rate() > 0.5
    }
    pub fn merge(&mut self, other: &Self) {
        self.total_queries += other.total_queries;
        self.total_hits += other.total_hits;
        self.total_misses += other.total_misses;
        self.total_inserts += other.total_inserts;
        self.evictions += other.evictions;
    }
}
/// An analysis pass for AstCache.
#[allow(dead_code)]
pub struct AstCacheAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<AstCacheResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl AstCacheAnalysisPass {
    pub fn new(name: &str) -> Self {
        AstCacheAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> AstCacheResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            AstCacheResult::Err("empty input".to_string())
        } else {
            AstCacheResult::Ok(format!("processed: {}", input))
        };
        self.results.push(result.clone());
        result
    }
    pub fn success_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_ok()).count()
    }
    pub fn error_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_err()).count()
    }
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.success_count() as f64 / self.total_runs as f64
        }
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
/// Performs bulk expression transformations backed by an `AstTransformCache`.
pub struct BatchTransformer {
    pub(super) cache: AstTransformCache,
    pub(super) transform_queue: Vec<Expr>,
}
impl BatchTransformer {
    /// Create a new `BatchTransformer` with a default cache size of 1024.
    pub fn new() -> Self {
        BatchTransformer {
            cache: AstTransformCache::new(1024),
            transform_queue: Vec::new(),
        }
    }
    /// Add an expression to the transformation queue.
    pub fn enqueue(&mut self, expr: Expr) {
        self.transform_queue.push(expr);
    }
    /// Apply substitution (replacing `BVar(depth)` with `replacement`) to all
    /// queued expressions, returning the results and clearing the queue.
    pub fn batch_substitute(&mut self, depth: usize, replacement: &Expr) -> Vec<Expr> {
        let rep_hash = ExprHash::of(replacement);
        let exprs: Vec<Expr> = self.transform_queue.drain(..).collect();
        exprs
            .into_iter()
            .map(|e| {
                let key = ExprHash::of(&e);
                let combined = ExprHash(
                    key.0 ^ rep_hash.0 ^ (depth as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15),
                );
                self.cache
                    .get_or_compute(combined, || substitute_bvar(&e, depth as u32, replacement))
            })
            .collect()
    }
    /// Beta-reduce all queued expressions (top-level only), returning results
    /// and clearing the queue.
    pub fn batch_beta_reduce(&mut self) -> Vec<Expr> {
        let exprs: Vec<Expr> = self.transform_queue.drain(..).collect();
        exprs
            .into_iter()
            .map(|e| {
                let key = ExprHash::of(&e);
                let cache_key = ExprHash(key.0 ^ 0xcafe_dead_beef_cafe);
                self.cache.get_or_compute(cache_key, || beta_reduce_top(&e))
            })
            .collect()
    }
    /// Drain the queue, returning the expressions unchanged.
    pub fn flush(&mut self) -> Vec<Expr> {
        self.transform_queue.drain(..).collect()
    }
}
#[allow(dead_code)]
pub struct AstCacheExtDiff400 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl AstCacheExtDiff400 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    #[allow(dead_code)]
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    #[allow(dead_code)]
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
#[allow(dead_code)]
pub struct AstCacheExtPipeline401 {
    pub name: String,
    pub passes: Vec<AstCacheExtPass401>,
    pub run_count: usize,
}
impl AstCacheExtPipeline401 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: AstCacheExtPass401) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<AstCacheExtResult401> {
        self.run_count += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    #[allow(dead_code)]
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    #[allow(dead_code)]
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    #[allow(dead_code)]
    pub fn total_success_rate(&self) -> f64 {
        let total: usize = self.passes.iter().map(|p| p.total_runs).sum();
        let ok: usize = self.passes.iter().map(|p| p.successes).sum();
        if total == 0 {
            0.0
        } else {
            ok as f64 / total as f64
        }
    }
}
/// A cache where entries expire after a given number of accesses.
#[allow(dead_code)]
pub struct TtlCache<K: std::hash::Hash + Eq, V: Clone> {
    pub data: std::collections::HashMap<K, TtlEntry<V>>,
    pub access_count: usize,
    pub expired_count: usize,
    pub default_ttl: u32,
}
#[allow(dead_code)]
impl<K: std::hash::Hash + Eq, V: Clone> TtlCache<K, V> {
    pub fn new(default_ttl: u32) -> Self {
        TtlCache {
            data: std::collections::HashMap::new(),
            access_count: 0,
            expired_count: 0,
            default_ttl,
        }
    }
    pub fn insert(&mut self, key: K, value: V) {
        let entry = TtlEntry {
            value,
            ttl: self.default_ttl,
            created_at: self.access_count,
        };
        self.data.insert(key, entry);
    }
    pub fn get(&mut self, key: &K) -> Option<V> {
        self.access_count += 1;
        if let Some(entry) = self.data.get_mut(key) {
            if entry.ttl == 0 {
                None
            } else {
                entry.ttl -= 1;
                Some(entry.value.clone())
            }
        } else {
            None
        }
    }
    pub fn evict_expired(&mut self) {
        let _: Vec<()> = Vec::new();
        self.data.retain(|_, e| {
            if e.ttl == 0 {
                self.expired_count += 1;
                false
            } else {
                true
            }
        });
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
/// Memoize an expression transformation using a hash map.
#[allow(dead_code)]
pub struct MemoTransformExt {
    pub cache: HashMap<u64, Expr>,
    pub calls: usize,
    pub cache_hits: usize,
}
#[allow(dead_code)]
impl MemoTransformExt {
    pub fn new() -> Self {
        MemoTransformExt {
            cache: HashMap::new(),
            calls: 0,
            cache_hits: 0,
        }
    }
    pub fn apply<F: Fn(&Expr) -> Expr>(&mut self, e: &Expr, f: F) -> Expr {
        self.calls += 1;
        let key = hash_expr(e);
        if let Some(cached) = self.cache.get(&key).cloned() {
            self.cache_hits += 1;
            return cached;
        }
        let result = f(e);
        self.cache.insert(key, result.clone());
        result
    }
    pub fn hit_rate(&self) -> f64 {
        if self.calls == 0 {
            0.0
        } else {
            self.cache_hits as f64 / self.calls as f64
        }
    }
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
/// A cache with a configurable warming strategy.
#[allow(dead_code)]
pub struct WarmableCache {
    pub data: std::collections::HashMap<u64, Expr>,
    pub strategy: WarmingStrategy,
    pub warmed: bool,
    pub capacity: usize,
}
#[allow(dead_code)]
impl WarmableCache {
    pub fn new(capacity: usize, strategy: WarmingStrategy) -> Self {
        WarmableCache {
            data: std::collections::HashMap::new(),
            strategy,
            warmed: false,
            capacity,
        }
    }
    pub fn warm_with(&mut self, entries: Vec<(u64, Expr)>) {
        for (k, v) in entries {
            if self.data.len() < self.capacity {
                self.data.insert(k, v);
            }
        }
        self.warmed = true;
    }
    pub fn get(&self, key: u64) -> Option<&Expr> {
        self.data.get(&key)
    }
    pub fn put(&mut self, key: u64, value: Expr) {
        if self.data.len() < self.capacity {
            self.data.insert(key, value);
        }
    }
    pub fn is_warm(&self) -> bool {
        self.warmed
    }
    pub fn fill_ratio(&self) -> f64 {
        if self.capacity == 0 {
            0.0
        } else {
            self.data.len() as f64 / self.capacity as f64
        }
    }
}
/// An interning table that assigns unique IDs to expressions.
#[allow(dead_code)]
pub struct InternTable {
    pub map: std::collections::HashMap<u64, usize>,
    pub next_id: usize,
    pub collisions: usize,
}
#[allow(dead_code)]
impl InternTable {
    pub fn new() -> Self {
        InternTable {
            map: std::collections::HashMap::new(),
            next_id: 0,
            collisions: 0,
        }
    }
    pub fn intern(&mut self, hash: u64) -> usize {
        if let Some(&id) = self.map.get(&hash) {
            id
        } else {
            let id = self.next_id;
            self.next_id += 1;
            self.map.insert(hash, id);
            id
        }
    }
    pub fn lookup(&self, hash: u64) -> Option<usize> {
        self.map.get(&hash).copied()
    }
    pub fn size(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
#[allow(dead_code)]
pub struct AstCacheExtConfig400 {
    pub(super) values: std::collections::HashMap<String, AstCacheExtConfigVal400>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl AstCacheExtConfig400 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: String::new(),
        }
    }
    #[allow(dead_code)]
    pub fn named(name: &str) -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: name.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self, key: &str, value: AstCacheExtConfigVal400) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&AstCacheExtConfigVal400> {
        self.values.get(key)
    }
    #[allow(dead_code)]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    #[allow(dead_code)]
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    #[allow(dead_code)]
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    #[allow(dead_code)]
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, AstCacheExtConfigVal400::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, AstCacheExtConfigVal400::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, AstCacheExtConfigVal400::Str(v.to_string()))
    }
    #[allow(dead_code)]
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    #[allow(dead_code)]
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.values.len()
    }
    #[allow(dead_code)]
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
/// A diagnostic reporter for AstCache.
#[allow(dead_code)]
pub struct AstCacheDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl AstCacheDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        AstCacheDiagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
/// A shared expression cache using hash-consing.
#[allow(dead_code)]
pub struct HashConsCache {
    pub table: std::collections::HashMap<u64, Expr>,
    pub lookups: usize,
    pub insertions: usize,
    pub sharing_ratio: f64,
}
#[allow(dead_code)]
impl HashConsCache {
    pub fn new() -> Self {
        HashConsCache {
            table: std::collections::HashMap::new(),
            lookups: 0,
            insertions: 0,
            sharing_ratio: 0.0,
        }
    }
    pub fn intern(&mut self, expr: Expr) -> Expr {
        let key = hash_expr(&expr);
        self.lookups += 1;
        if let Some(existing) = self.table.get(&key) {
            self.sharing_ratio =
                (self.sharing_ratio * (self.lookups - 1) as f64 + 1.0) / self.lookups as f64;
            return existing.clone();
        }
        self.insertions += 1;
        self.sharing_ratio = (self.sharing_ratio * (self.lookups - 1) as f64) / self.lookups as f64;
        self.table.insert(key, expr.clone());
        expr
    }
    pub fn size(&self) -> usize {
        self.table.len()
    }
    pub fn hit_count(&self) -> usize {
        self.lookups.saturating_sub(self.insertions)
    }
}
/// A cache that stores lazy (unevaluated) expressions.
#[allow(dead_code)]
pub enum CachedValue {
    Ready(Expr),
    Pending,
    Failed(String),
}
#[allow(dead_code)]
impl CachedValue {
    pub fn is_ready(&self) -> bool {
        matches!(self, CachedValue::Ready(_))
    }
    pub fn is_pending(&self) -> bool {
        matches!(self, CachedValue::Pending)
    }
    pub fn is_failed(&self) -> bool {
        matches!(self, CachedValue::Failed(_))
    }
    pub fn get(&self) -> Option<&Expr> {
        match self {
            CachedValue::Ready(e) => Some(e),
            _ => None,
        }
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            CachedValue::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// A typed slot for AstCache configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AstCacheConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl AstCacheConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AstCacheConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            AstCacheConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            AstCacheConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AstCacheConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            AstCacheConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            AstCacheConfigValue::Bool(_) => "bool",
            AstCacheConfigValue::Int(_) => "int",
            AstCacheConfigValue::Float(_) => "float",
            AstCacheConfigValue::Str(_) => "str",
            AstCacheConfigValue::List(_) => "list",
        }
    }
}
/// Frequency-aware cache for WHNF reduction results.
pub struct WhnfCache {
    pub(super) entries: HashMap<ExprHash, Expr>,
    pub(super) access_count: HashMap<ExprHash, u32>,
    pub(super) capacity: usize,
}
impl WhnfCache {
    /// Create a new WHNF cache with the given capacity.
    pub fn new(capacity: usize) -> Self {
        WhnfCache {
            entries: HashMap::new(),
            access_count: HashMap::new(),
            capacity,
        }
    }
    /// Look up a WHNF result, incrementing its access count.
    pub fn get(&mut self, key: ExprHash) -> Option<&Expr> {
        if self.entries.contains_key(&key) {
            *self.access_count.entry(key).or_insert(0) += 1;
            self.entries.get(&key)
        } else {
            None
        }
    }
    /// Insert a WHNF result. Evicts cold entries if over capacity.
    pub fn insert(&mut self, key: ExprHash, val: Expr) {
        if self.entries.len() >= self.capacity {
            let threshold = if self.entries.values().any(|_| true) {
                let avg_count = self.average_access_count();
                if avg_count < 2.0 {
                    0
                } else {
                    1
                }
            } else {
                0
            };
            self.evict_cold(threshold);
        }
        self.entries.insert(key, val);
        self.access_count.entry(key).or_insert(0);
    }
    /// Remove all entries whose access count is at or below `threshold`.
    pub fn evict_cold(&mut self, threshold: u32) {
        let cold_keys: Vec<ExprHash> = self
            .access_count
            .iter()
            .filter(|(_, &cnt)| cnt <= threshold)
            .map(|(&k, _)| k)
            .collect();
        for key in cold_keys {
            self.entries.remove(&key);
            self.access_count.remove(&key);
        }
    }
    /// Return the average access count across all entries.
    fn average_access_count(&self) -> f64 {
        if self.access_count.is_empty() {
            return 0.0;
        }
        let total: u32 = self.access_count.values().sum();
        total as f64 / self.access_count.len() as f64
    }
    /// Return the keys of entries with access count above the average.
    pub fn hot_entries(&self) -> Vec<ExprHash> {
        let avg = self.average_access_count();
        self.access_count
            .iter()
            .filter(|(_, &cnt)| cnt as f64 > avg)
            .map(|(&k, _)| k)
            .collect()
    }
}
/// A bounded, LRU-evicting cache for memoizing expensive expression transformations.
pub struct AstTransformCache {
    pub(super) cache: HashMap<ExprHash, Expr>,
    /// Insertion-order tracking for LRU eviction.
    pub(super) insertion_order: Vec<ExprHash>,
    /// Number of cache hits.
    pub(crate) hit_count: u64,
    /// Number of cache misses.
    pub(crate) miss_count: u64,
    /// Maximum number of entries before eviction.
    pub(super) max_size: usize,
}
impl AstTransformCache {
    /// Create a new cache with the given maximum size.
    pub fn new(max_size: usize) -> Self {
        AstTransformCache {
            cache: HashMap::new(),
            insertion_order: Vec::new(),
            hit_count: 0,
            miss_count: 0,
            max_size,
        }
    }
    /// Return the cached value for `key`, or compute it, store it, and return it.
    pub fn get_or_compute<F: FnOnce() -> Expr>(&mut self, key: ExprHash, compute: F) -> Expr {
        if let Some(expr) = self.cache.get(&key) {
            self.hit_count += 1;
            return expr.clone();
        }
        self.miss_count += 1;
        let val = compute();
        self.insert(key, val.clone());
        val
    }
    /// Insert a value into the cache, evicting the oldest entry when over capacity.
    pub fn insert(&mut self, key: ExprHash, val: Expr) {
        if let std::collections::hash_map::Entry::Occupied(mut e) = self.cache.entry(key) {
            e.insert(val);
            return;
        }
        if self.cache.len() >= self.max_size {
            self.evict_lru();
        }
        self.cache.insert(key, val);
        self.insertion_order.push(key);
    }
    /// Look up a key without modifying counters.
    pub fn get(&self, key: ExprHash) -> Option<&Expr> {
        self.cache.get(&key)
    }
    /// Fraction of lookups that were hits. Returns 0.0 if no lookups have been made.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hit_count + self.miss_count;
        if total == 0 {
            0.0
        } else {
            self.hit_count as f64 / total as f64
        }
    }
    /// Evict the oldest entry from the cache.
    pub fn evict_lru(&mut self) {
        while let Some(oldest) = self.insertion_order.first().cloned() {
            self.insertion_order.remove(0);
            if self.cache.remove(&oldest).is_some() {
                return;
            }
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AstCacheExtConfigVal401 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl AstCacheExtConfigVal401 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let AstCacheExtConfigVal401::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let AstCacheExtConfigVal401::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let AstCacheExtConfigVal401::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let AstCacheExtConfigVal401::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let AstCacheExtConfigVal401::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            AstCacheExtConfigVal401::Bool(_) => "bool",
            AstCacheExtConfigVal401::Int(_) => "int",
            AstCacheExtConfigVal401::Float(_) => "float",
            AstCacheExtConfigVal401::Str(_) => "str",
            AstCacheExtConfigVal401::List(_) => "list",
        }
    }
}
/// A two-level cache: fast small L1 and larger L2.
#[allow(dead_code)]
pub struct TieredCache {
    pub l1: LruCacheExt<u64, Expr>,
    pub l2: LruCacheExt<u64, Expr>,
    pub l1_hits: usize,
    pub l2_hits: usize,
    pub misses: usize,
}
#[allow(dead_code)]
impl TieredCache {
    pub fn new(l1_cap: usize, l2_cap: usize) -> Self {
        TieredCache {
            l1: LruCacheExt::new(l1_cap),
            l2: LruCacheExt::new(l2_cap),
            l1_hits: 0,
            l2_hits: 0,
            misses: 0,
        }
    }
    pub fn get(&mut self, key: u64) -> Option<Expr> {
        if let Some(v) = self.l1.get(&key) {
            self.l1_hits += 1;
            return Some(v);
        }
        if let Some(v) = self.l2.get(&key) {
            self.l2_hits += 1;
            self.l1.put(key, v.clone());
            return Some(v);
        }
        self.misses += 1;
        None
    }
    pub fn put(&mut self, key: u64, value: Expr) {
        self.l1.put(key, value.clone());
        self.l2.put(key, value);
    }
    pub fn total_hits(&self) -> usize {
        self.l1_hits + self.l2_hits
    }
    pub fn hit_rate(&self) -> f64 {
        let total = self.total_hits() + self.misses;
        if total == 0 {
            0.0
        } else {
            self.total_hits() as f64 / total as f64
        }
    }
}
#[allow(dead_code)]
pub struct AstCacheExtDiag400 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl AstCacheExtDiag400 {
    #[allow(dead_code)]
    pub fn new(max_errors: usize) -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    #[allow(dead_code)]
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    #[allow(dead_code)]
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    #[allow(dead_code)]
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    #[allow(dead_code)]
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    #[allow(dead_code)]
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
