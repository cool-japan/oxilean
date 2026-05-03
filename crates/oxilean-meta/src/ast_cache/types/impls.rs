//! Auto-generated module (split from types.rs)
//!
//! Second half of type definitions and impl blocks.

use super::super::functions::*;
use super::defs::*;
use oxilean_kernel::Expr;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[allow(dead_code)]
pub struct AstCacheExtDiff401 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl AstCacheExtDiff401 {
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
pub struct AstCacheExtConfig401 {
    pub(super) values: std::collections::HashMap<String, AstCacheExtConfigVal401>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl AstCacheExtConfig401 {
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
    pub fn set(&mut self, key: &str, value: AstCacheExtConfigVal401) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&AstCacheExtConfigVal401> {
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
        self.set(key, AstCacheExtConfigVal401::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, AstCacheExtConfigVal401::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, AstCacheExtConfigVal401::Str(v.to_string()))
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
#[allow(dead_code)]
pub struct AstCacheExtDiag401 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl AstCacheExtDiag401 {
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
/// A simple LRU cache with fixed capacity using timestamps.
#[allow(dead_code)]
pub struct LruCacheExt<K: std::hash::Hash + Eq + Clone, V: Clone> {
    pub(super) capacity: usize,
    pub(super) data: std::collections::HashMap<K, (V, usize)>,
    pub(super) clock: usize,
}
#[allow(dead_code)]
impl<K: std::hash::Hash + Eq + Clone, V: Clone> LruCacheExt<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        LruCacheExt {
            capacity,
            data: std::collections::HashMap::new(),
            clock: 0,
        }
    }
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some((v, ts)) = self.data.get_mut(key) {
            self.clock += 1;
            *ts = self.clock;
            Some(v.clone())
        } else {
            None
        }
    }
    pub fn put(&mut self, key: K, value: V) {
        if self.data.len() >= self.capacity && !self.data.contains_key(&key) {
            if let Some(oldest) = self
                .data
                .iter()
                .min_by_key(|(_, (_, ts))| ts)
                .map(|(k, _)| k.clone())
            {
                self.data.remove(&oldest);
            }
        }
        self.clock += 1;
        self.data.insert(key, (value, self.clock));
    }
    pub fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn clear_all(&mut self) {
        self.data.clear();
        self.clock = 0;
    }
}
#[allow(dead_code)]
pub struct AstCacheExtPipeline400 {
    pub name: String,
    pub passes: Vec<AstCacheExtPass400>,
    pub run_count: usize,
}
impl AstCacheExtPipeline400 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: AstCacheExtPass400) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<AstCacheExtResult400> {
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
/// A pipeline of AstCache analysis passes.
#[allow(dead_code)]
pub struct AstCachePipeline {
    pub passes: Vec<AstCacheAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl AstCachePipeline {
    pub fn new(name: &str) -> Self {
        AstCachePipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: AstCacheAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<AstCacheResult> {
        self.total_inputs_processed += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    pub fn total_success_rate(&self) -> f64 {
        if self.passes.is_empty() {
            0.0
        } else {
            let total_rate: f64 = self.passes.iter().map(|p| p.success_rate()).sum();
            total_rate / self.passes.len() as f64
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AstCacheExtResult400 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl AstCacheExtResult400 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, AstCacheExtResult400::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, AstCacheExtResult400::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, AstCacheExtResult400::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, AstCacheExtResult400::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let AstCacheExtResult400::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let AstCacheExtResult400::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            AstCacheExtResult400::Ok(_) => 1.0,
            AstCacheExtResult400::Err(_) => 0.0,
            AstCacheExtResult400::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            AstCacheExtResult400::Skipped => 0.5,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AstCacheExtResult401 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl AstCacheExtResult401 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, AstCacheExtResult401::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, AstCacheExtResult401::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, AstCacheExtResult401::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, AstCacheExtResult401::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let AstCacheExtResult401::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let AstCacheExtResult401::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            AstCacheExtResult401::Ok(_) => 1.0,
            AstCacheExtResult401::Err(_) => 0.0,
            AstCacheExtResult401::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            AstCacheExtResult401::Skipped => 0.5,
        }
    }
}
#[allow(dead_code)]
pub struct AstCacheExtPass400 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<AstCacheExtResult400>,
}
impl AstCacheExtPass400 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_runs: 0,
            successes: 0,
            errors: 0,
            enabled: true,
            results: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self, input: &str) -> AstCacheExtResult400 {
        if !self.enabled {
            return AstCacheExtResult400::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            AstCacheExtResult400::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            AstCacheExtResult400::Ok(format!(
                "processed {} chars in pass '{}'",
                input.len(),
                self.name
            ))
        };
        self.results.push(result.clone());
        result
    }
    #[allow(dead_code)]
    pub fn success_count(&self) -> usize {
        self.successes
    }
    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.errors
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.successes as f64 / self.total_runs as f64
        }
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
/// A cache of cached values indexed by expression hash.
#[allow(dead_code)]
pub struct LazyExprCache {
    pub data: HashMap<u64, CachedValue>,
}
#[allow(dead_code)]
impl LazyExprCache {
    pub fn new() -> Self {
        LazyExprCache {
            data: HashMap::new(),
        }
    }
    pub fn mark_pending(&mut self, key: u64) {
        self.data.insert(key, CachedValue::Pending);
    }
    pub fn store_result(&mut self, key: u64, result: Expr) {
        self.data.insert(key, CachedValue::Ready(result));
    }
    pub fn store_error(&mut self, key: u64, msg: String) {
        self.data.insert(key, CachedValue::Failed(msg));
    }
    pub fn get(&self, key: u64) -> Option<&CachedValue> {
        self.data.get(&key)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn num_ready(&self) -> usize {
        self.data.values().filter(|v| v.is_ready()).count()
    }
    pub fn num_failed(&self) -> usize {
        self.data.values().filter(|v| v.is_failed()).count()
    }
}
/// Opaque hash of an `Expr`, used as a cache key.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ExprHash(pub u64);
impl ExprHash {
    /// Compute the hash of an expression.
    pub fn of(expr: &Expr) -> Self {
        ExprHash(hash_expr(expr))
    }
}
/// A two-level cache with a fast in-memory layer and a slower persisted layer.
#[allow(dead_code)]
pub struct TwoLevelCache {
    pub hot: std::collections::HashMap<u64, Expr>,
    pub warm: std::collections::HashMap<u64, Expr>,
    pub hot_capacity: usize,
    pub warm_capacity: usize,
    pub promotions: usize,
    pub demotions: usize,
}
#[allow(dead_code)]
impl TwoLevelCache {
    pub fn new(hot_cap: usize, warm_cap: usize) -> Self {
        TwoLevelCache {
            hot: std::collections::HashMap::new(),
            warm: std::collections::HashMap::new(),
            hot_capacity: hot_cap,
            warm_capacity: warm_cap,
            promotions: 0,
            demotions: 0,
        }
    }
    pub fn lookup(&mut self, key: u64) -> Option<Expr> {
        if let Some(v) = self.hot.get(&key) {
            return Some(v.clone());
        }
        if let Some(v) = self.warm.remove(&key) {
            if self.hot.len() >= self.hot_capacity {
                if let Some((&k2, _)) = self.hot.iter().next() {
                    if let Some(v2) = self.hot.remove(&k2) {
                        if self.warm.len() < self.warm_capacity {
                            self.warm.insert(k2, v2);
                        }
                        self.demotions += 1;
                    }
                }
            }
            self.hot.insert(key, v.clone());
            self.promotions += 1;
            return Some(v);
        }
        None
    }
    pub fn insert(&mut self, key: u64, value: Expr) {
        if self.hot.len() < self.hot_capacity {
            self.hot.insert(key, value);
        } else if self.warm.len() < self.warm_capacity {
            self.warm.insert(key, value);
        }
    }
    pub fn total_size(&self) -> usize {
        self.hot.len() + self.warm.len()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum AstCacheExtConfigVal400 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl AstCacheExtConfigVal400 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let AstCacheExtConfigVal400::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let AstCacheExtConfigVal400::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let AstCacheExtConfigVal400::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let AstCacheExtConfigVal400::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let AstCacheExtConfigVal400::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            AstCacheExtConfigVal400::Bool(_) => "bool",
            AstCacheExtConfigVal400::Int(_) => "int",
            AstCacheExtConfigVal400::Float(_) => "float",
            AstCacheExtConfigVal400::Str(_) => "str",
            AstCacheExtConfigVal400::List(_) => "list",
        }
    }
}
/// Strategy for pre-warming a cache.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum WarmingStrategy {
    Eager,
    Lazy,
    Predictive,
    None,
}
#[allow(dead_code)]
pub struct AstCacheExtPass401 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<AstCacheExtResult401>,
}
impl AstCacheExtPass401 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_runs: 0,
            successes: 0,
            errors: 0,
            enabled: true,
            results: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self, input: &str) -> AstCacheExtResult401 {
        if !self.enabled {
            return AstCacheExtResult401::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            AstCacheExtResult401::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            AstCacheExtResult401::Ok(format!(
                "processed {} chars in pass '{}'",
                input.len(),
                self.name
            ))
        };
        self.results.push(result.clone());
        result
    }
    #[allow(dead_code)]
    pub fn success_count(&self) -> usize {
        self.successes
    }
    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.errors
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.successes as f64 / self.total_runs as f64
        }
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
/// A configuration store for AstCache.
#[allow(dead_code)]
pub struct AstCacheConfig {
    pub values: std::collections::HashMap<String, AstCacheConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl AstCacheConfig {
    pub fn new() -> Self {
        AstCacheConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: AstCacheConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&AstCacheConfigValue> {
        self.values.get(key)
    }
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, AstCacheConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, AstCacheConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, AstCacheConfigValue::Str(v.to_string()))
    }
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    pub fn size(&self) -> usize {
        self.values.len()
    }
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
/// A cache entry with a time-to-live counter.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TtlEntry<V: Clone> {
    pub value: V,
    pub ttl: u32,
    pub created_at: usize,
}
/// A result type for AstCache analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum AstCacheResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl AstCacheResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, AstCacheResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, AstCacheResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, AstCacheResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, AstCacheResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            AstCacheResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            AstCacheResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            AstCacheResult::Ok(_) => 1.0,
            AstCacheResult::Err(_) => 0.0,
            AstCacheResult::Skipped => 0.0,
            AstCacheResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A versioned cache that supports rollback.
#[allow(dead_code)]
pub struct VersionedCache<K: std::hash::Hash + Eq + Clone, V: Clone> {
    pub versions: Vec<std::collections::HashMap<K, V>>,
    pub current: usize,
}
#[allow(dead_code)]
impl<K: std::hash::Hash + Eq + Clone, V: Clone> VersionedCache<K, V> {
    pub fn new() -> Self {
        VersionedCache {
            versions: vec![std::collections::HashMap::new()],
            current: 0,
        }
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.versions[self.current].get(key)
    }
    pub fn insert(&mut self, key: K, value: V) {
        self.versions[self.current].insert(key, value);
    }
    pub fn checkpoint(&mut self) {
        let snapshot = self.versions[self.current].clone();
        self.versions.push(snapshot);
        self.current += 1;
    }
    pub fn rollback(&mut self) -> bool {
        if self.current == 0 {
            return false;
        }
        self.versions.pop();
        self.current -= 1;
        true
    }
    pub fn num_versions(&self) -> usize {
        self.versions.len()
    }
    pub fn current_size(&self) -> usize {
        self.versions[self.current].len()
    }
}
/// Cache for substitution results keyed by `(expr_hash, depth, replacement_hash)`.
pub struct SubstCache {
    pub(super) results: HashMap<(ExprHash, usize, ExprHash), Expr>,
}
impl SubstCache {
    /// Create an empty substitution cache.
    pub fn new() -> Self {
        SubstCache {
            results: HashMap::new(),
        }
    }
    /// Look up a prior substitution result.
    pub fn lookup(&self, expr: ExprHash, depth: usize, replacement: ExprHash) -> Option<&Expr> {
        self.results.get(&(expr, depth, replacement))
    }
    /// Store a substitution result.
    pub fn store(&mut self, expr: ExprHash, depth: usize, replacement: ExprHash, result: Expr) {
        self.results.insert((expr, depth, replacement), result);
    }
    /// Number of stored entries.
    pub fn size(&self) -> usize {
        self.results.len()
    }
    /// Remove all stored entries.
    pub fn clear(&mut self) {
        self.results.clear();
    }
}
/// A diff for AstCache analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AstCacheDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl AstCacheDiff {
    pub fn new() -> Self {
        AstCacheDiff {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
