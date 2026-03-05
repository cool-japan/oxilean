//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::Expr;

/// A utility type for MetaDbg (index 1).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil1 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil1 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil1 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A utility type for MetaDbg (index 9).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil9 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil9 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil9 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A utility type for MetaDbg (index 2).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil2 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil2 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil2 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A registry for MetaDbg utilities.
#[allow(dead_code)]
pub struct MetaDbgRegistry {
    pub entries: Vec<MetaDbgUtil0>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl MetaDbgRegistry {
    pub fn new(capacity: usize) -> Self {
        MetaDbgRegistry {
            entries: Vec::new(),
            capacity,
        }
    }
    pub fn register(&mut self, entry: MetaDbgUtil0) -> bool {
        if self.entries.len() >= self.capacity {
            return false;
        }
        self.entries.push(entry);
        true
    }
    pub fn lookup(&self, id: usize) -> Option<&MetaDbgUtil0> {
        self.entries.iter().find(|e| e.id == id)
    }
    pub fn remove(&mut self, id: usize) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < before
    }
    pub fn active_entries(&self) -> Vec<&MetaDbgUtil0> {
        self.entries.iter().filter(|e| e.is_active()).collect()
    }
    pub fn total_score(&self) -> i64 {
        self.entries.iter().map(|e| e.score()).sum()
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn is_full(&self) -> bool {
        self.entries.len() >= self.capacity
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// Collects traces during elaboration for post-hoc inspection.
pub struct MetaTracer {
    pub traces: Vec<MetaTrace>,
    pub enabled: bool,
    pub(super) depth: u32,
}
impl MetaTracer {
    /// Create a new disabled tracer.
    pub fn new() -> Self {
        MetaTracer {
            traces: Vec::new(),
            enabled: false,
            depth: 0,
        }
    }
    /// Enable tracing.
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    /// Disable tracing.
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    /// Record an entry event (increments depth).
    pub fn enter(&mut self, label: &str) {
        if self.enabled {
            self.traces.push(MetaTrace {
                label: format!(">> {}", label),
                expr: String::new(),
                depth: self.depth,
            });
        }
        self.depth = self.depth.saturating_add(1);
    }
    /// Record an exit event (decrements depth).
    pub fn exit(&mut self) {
        self.depth = self.depth.saturating_sub(1);
        if self.enabled {
            self.traces.push(MetaTrace {
                label: "<< exit".to_string(),
                expr: String::new(),
                depth: self.depth,
            });
        }
    }
    /// Record an expression with a label.
    pub fn record(&mut self, label: &str, expr: &Expr) {
        if self.enabled {
            self.traces.push(MetaTrace {
                label: label.to_string(),
                expr: expr_debug(expr),
                depth: self.depth,
            });
        }
    }
    /// Format all traces as a human-readable string.
    pub fn dump(&self) -> String {
        let mut out = String::new();
        for t in &self.traces {
            let pad = " ".repeat((t.depth as usize) * 2);
            if t.expr.is_empty() {
                out.push_str(&format!("{}{}\n", pad, t.label));
            } else {
                out.push_str(&format!("{}{}: {}\n", pad, t.label, t.expr));
            }
        }
        out
    }
    /// Clear all recorded traces and reset depth.
    pub fn clear(&mut self) {
        self.traces.clear();
        self.depth = 0;
    }
    /// Return the number of recorded traces.
    pub fn len(&self) -> usize {
        self.traces.len()
    }
    /// Return `true` if there are no recorded traces.
    pub fn is_empty(&self) -> bool {
        self.traces.is_empty()
    }
}
/// Detailed statistics about an expression.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ExprStats {
    pub depth: usize,
    pub node_count: usize,
    pub num_bvars: usize,
    pub num_fvars: usize,
    pub num_mvars: usize,
    pub num_apps: usize,
    pub num_lams: usize,
    pub num_pis: usize,
    pub num_consts: usize,
    pub num_sorts: usize,
    pub num_lits: usize,
}
#[allow(dead_code)]
impl ExprStats {
    pub fn compute(e: &Expr) -> Self {
        let mut stats = ExprStats::default();
        stats.collect(e, 0);
        stats
    }
    fn collect(&mut self, e: &Expr, depth: usize) {
        self.node_count += 1;
        self.depth = self.depth.max(depth);
        match e {
            Expr::BVar(_) => self.num_bvars += 1,
            Expr::FVar(_) => self.num_fvars += 1,
            Expr::Sort(_) => self.num_sorts += 1,
            Expr::Const(_, _) => self.num_consts += 1,
            Expr::Lit(_) => self.num_lits += 1,
            Expr::App(f, a) => {
                self.num_apps += 1;
                self.collect(f, depth + 1);
                self.collect(a, depth + 1);
            }
            Expr::Lam(_, _, t, b) => {
                self.num_lams += 1;
                self.collect(t, depth + 1);
                self.collect(b, depth + 1);
            }
            Expr::Pi(_, _, t, b) => {
                self.num_pis += 1;
                self.collect(t, depth + 1);
                self.collect(b, depth + 1);
            }
            Expr::Let(_, _, t, b) => {
                self.collect(t, depth + 1);
                self.collect(b, depth + 1);
            }
            Expr::Proj(_, _, e) => {
                self.collect(e, depth + 1);
            }
        }
    }
    pub fn is_closed(&self) -> bool {
        self.num_bvars == 0 && self.num_mvars == 0
    }
    pub fn is_ground(&self) -> bool {
        self.num_mvars == 0
    }
}
/// A simple cache for MetaDbg computations.
#[allow(dead_code)]
pub struct MetaDbgCache {
    pub data: std::collections::HashMap<String, i64>,
    pub hits: usize,
    pub misses: usize,
}
#[allow(dead_code)]
impl MetaDbgCache {
    pub fn new() -> Self {
        MetaDbgCache {
            data: std::collections::HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    pub fn get(&mut self, key: &str) -> Option<i64> {
        if let Some(&v) = self.data.get(key) {
            self.hits += 1;
            Some(v)
        } else {
            self.misses += 1;
            None
        }
    }
    pub fn insert(&mut self, key: &str, value: i64) {
        self.data.insert(key.to_string(), value);
    }
    pub fn hit_rate(&self) -> f64 {
        let t = self.hits + self.misses;
        if t == 0 {
            0.0
        } else {
            self.hits as f64 / t as f64
        }
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn clear(&mut self) {
        self.data.clear();
        self.hits = 0;
        self.misses = 0;
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MetaDebugExtConfigVal3300 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl MetaDebugExtConfigVal3300 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let MetaDebugExtConfigVal3300::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let MetaDebugExtConfigVal3300::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let MetaDebugExtConfigVal3300::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let MetaDebugExtConfigVal3300::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let MetaDebugExtConfigVal3300::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            MetaDebugExtConfigVal3300::Bool(_) => "bool",
            MetaDebugExtConfigVal3300::Int(_) => "int",
            MetaDebugExtConfigVal3300::Float(_) => "float",
            MetaDebugExtConfigVal3300::Str(_) => "str",
            MetaDebugExtConfigVal3300::List(_) => "list",
        }
    }
}
/// A diagnostic reporter for MetaDebug.
#[allow(dead_code)]
pub struct MetaDebugDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl MetaDebugDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        MetaDebugDiagnostics {
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
/// A utility type for MetaDbg (index 10).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil10 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil10 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil10 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A utility type for MetaDbg (index 13).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil13 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil13 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil13 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A trace record for metaprogram debugging.
#[derive(Debug, Clone)]
pub struct MetaTrace {
    pub label: String,
    pub expr: String,
    pub depth: u32,
}
/// A utility type for MetaDbg (index 3).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil3 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil3 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil3 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A utility type for MetaDbg (index 14).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil14 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil14 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil14 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A logger for MetaDbg operations.
#[allow(dead_code)]
pub struct MetaDbgLogger {
    pub entries: Vec<String>,
    pub max_entries: usize,
    pub verbose: bool,
}
#[allow(dead_code)]
impl MetaDbgLogger {
    pub fn new(max_entries: usize) -> Self {
        MetaDbgLogger {
            entries: Vec::new(),
            max_entries,
            verbose: false,
        }
    }
    pub fn log(&mut self, msg: &str) {
        if self.entries.len() < self.max_entries {
            self.entries.push(msg.to_string());
        }
    }
    pub fn verbose(&mut self, msg: &str) {
        if self.verbose {
            self.log(msg);
        }
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    pub fn count(&self) -> usize {
        self.entries.len()
    }
    pub fn last(&self) -> Option<&str> {
        self.entries.last().map(|s| s.as_str())
    }
    pub fn enable_verbose(&mut self) {
        self.verbose = true;
    }
    pub fn disable_verbose(&mut self) {
        self.verbose = false;
    }
}
/// A utility type for MetaDbg (index 12).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil12 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil12 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil12 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// Statistics for MetaDbg operations.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgStats {
    pub total_ops: usize,
    pub successful_ops: usize,
    pub failed_ops: usize,
    pub total_time_ns: u64,
    pub max_time_ns: u64,
}
#[allow(dead_code)]
impl MetaDbgStats {
    pub fn new() -> Self {
        MetaDbgStats::default()
    }
    pub fn record_success(&mut self, time_ns: u64) {
        self.total_ops += 1;
        self.successful_ops += 1;
        self.total_time_ns += time_ns;
        if time_ns > self.max_time_ns {
            self.max_time_ns = time_ns;
        }
    }
    pub fn record_failure(&mut self) {
        self.total_ops += 1;
        self.failed_ops += 1;
    }
    pub fn success_rate(&self) -> f64 {
        if self.total_ops == 0 {
            0.0
        } else {
            self.successful_ops as f64 / self.total_ops as f64
        }
    }
    pub fn avg_time_ns(&self) -> f64 {
        if self.successful_ops == 0 {
            0.0
        } else {
            self.total_time_ns as f64 / self.successful_ops as f64
        }
    }
    pub fn merge(&mut self, other: &Self) {
        self.total_ops += other.total_ops;
        self.successful_ops += other.successful_ops;
        self.failed_ops += other.failed_ops;
        self.total_time_ns += other.total_time_ns;
        if other.max_time_ns > self.max_time_ns {
            self.max_time_ns = other.max_time_ns;
        }
    }
}
/// A priority queue for MetaDbg items.
#[allow(dead_code)]
pub struct MetaDbgPriorityQueue {
    pub items: Vec<(MetaDbgUtil0, i64)>,
}
#[allow(dead_code)]
impl MetaDbgPriorityQueue {
    pub fn new() -> Self {
        MetaDbgPriorityQueue { items: Vec::new() }
    }
    pub fn push(&mut self, item: MetaDbgUtil0, priority: i64) {
        self.items.push((item, priority));
        self.items.sort_by_key(|(_, p)| -p);
    }
    pub fn pop(&mut self) -> Option<(MetaDbgUtil0, i64)> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
    pub fn peek(&self) -> Option<&(MetaDbgUtil0, i64)> {
        self.items.first()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
#[allow(dead_code)]
pub struct MetaDebugExtDiff3300 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl MetaDebugExtDiff3300 {
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
/// A utility type for MetaDbg (index 0).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil0 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil0 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil0 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A utility type for MetaDbg (index 8).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil8 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil8 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil8 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A diff for MetaDebug analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MetaDebugDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl MetaDebugDiff {
    pub fn new() -> Self {
        MetaDebugDiff {
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
/// A typed slot for MetaDebug configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MetaDebugConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl MetaDebugConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            MetaDebugConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            MetaDebugConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            MetaDebugConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            MetaDebugConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            MetaDebugConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            MetaDebugConfigValue::Bool(_) => "bool",
            MetaDebugConfigValue::Int(_) => "int",
            MetaDebugConfigValue::Float(_) => "float",
            MetaDebugConfigValue::Str(_) => "str",
            MetaDebugConfigValue::List(_) => "list",
        }
    }
}
/// A utility type for MetaDbg (index 5).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil5 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil5 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil5 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
#[allow(dead_code)]
pub struct MetaDebugExtDiag3300 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl MetaDebugExtDiag3300 {
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
/// A debug trace log.
#[allow(dead_code)]
pub struct TraceLog {
    pub entries: Vec<TraceEntry>,
    pub max_level: TraceLevel,
}
#[allow(dead_code)]
impl TraceLog {
    pub fn new(max_level: TraceLevel) -> Self {
        TraceLog {
            entries: Vec::new(),
            max_level,
        }
    }
    pub fn log(&mut self, entry: TraceEntry) {
        if entry.level <= self.max_level {
            self.entries.push(entry);
        }
    }
    pub fn error(&mut self, msg: &str, loc: &str) {
        self.log(TraceEntry::new(TraceLevel::Error, msg, loc));
    }
    pub fn info(&mut self, msg: &str, loc: &str) {
        self.log(TraceEntry::new(TraceLevel::Info, msg, loc));
    }
    pub fn debug(&mut self, msg: &str, loc: &str) {
        self.log(TraceEntry::new(TraceLevel::Debug, msg, loc));
    }
    pub fn num_entries(&self) -> usize {
        self.entries.len()
    }
    pub fn num_errors(&self) -> usize {
        self.entries.iter().filter(|e| e.is_error()).count()
    }
    pub fn has_errors(&self) -> bool {
        self.num_errors() > 0
    }
    pub fn filter_level(&self, level: &TraceLevel) -> Vec<&TraceEntry> {
        self.entries.iter().filter(|e| &e.level == level).collect()
    }
    pub fn summary(&self) -> String {
        format!(
            "{} entries, {} errors",
            self.num_entries(),
            self.num_errors()
        )
    }
}
/// Collect all BVar indices.
#[allow(dead_code)]
pub struct BVarCollector(pub Vec<u32>);
/// A utility type for MetaDbg (index 7).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil7 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil7 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil7 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
#[allow(dead_code)]
pub struct MetaDebugExtPipeline3300 {
    pub name: String,
    pub passes: Vec<MetaDebugExtPass3300>,
    pub run_count: usize,
}
impl MetaDebugExtPipeline3300 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: MetaDebugExtPass3300) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<MetaDebugExtResult3300> {
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
#[allow(dead_code)]
pub struct MetaDebugExtConfig3300 {
    pub(super) values: std::collections::HashMap<String, MetaDebugExtConfigVal3300>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl MetaDebugExtConfig3300 {
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
    pub fn set(&mut self, key: &str, value: MetaDebugExtConfigVal3300) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&MetaDebugExtConfigVal3300> {
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
        self.set(key, MetaDebugExtConfigVal3300::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, MetaDebugExtConfigVal3300::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, MetaDebugExtConfigVal3300::Str(v.to_string()))
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
/// A utility type for MetaDbg (index 11).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil11 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil11 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil11 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A pipeline of MetaDebug analysis passes.
#[allow(dead_code)]
pub struct MetaDebugPipeline {
    pub passes: Vec<MetaDebugAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl MetaDebugPipeline {
    pub fn new(name: &str) -> Self {
        MetaDebugPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: MetaDebugAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<MetaDebugResult> {
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
pub enum MetaDebugExtResult3300 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl MetaDebugExtResult3300 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, MetaDebugExtResult3300::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, MetaDebugExtResult3300::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, MetaDebugExtResult3300::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, MetaDebugExtResult3300::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let MetaDebugExtResult3300::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let MetaDebugExtResult3300::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            MetaDebugExtResult3300::Ok(_) => 1.0,
            MetaDebugExtResult3300::Err(_) => 0.0,
            MetaDebugExtResult3300::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            MetaDebugExtResult3300::Skipped => 0.5,
        }
    }
}
/// A utility type for MetaDbg (index 6).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil6 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil6 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil6 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// An analysis pass for MetaDebug.
#[allow(dead_code)]
pub struct MetaDebugAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<MetaDebugResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl MetaDebugAnalysisPass {
    pub fn new(name: &str) -> Self {
        MetaDebugAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> MetaDebugResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            MetaDebugResult::Err("empty input".to_string())
        } else {
            MetaDebugResult::Ok(format!("processed: {}", input))
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
/// A result type for MetaDebug analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MetaDebugResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl MetaDebugResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, MetaDebugResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, MetaDebugResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, MetaDebugResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, MetaDebugResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            MetaDebugResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            MetaDebugResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            MetaDebugResult::Ok(_) => 1.0,
            MetaDebugResult::Err(_) => 0.0,
            MetaDebugResult::Skipped => 0.0,
            MetaDebugResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// Count the number of App nodes.
#[allow(dead_code)]
pub struct AppCounter(pub usize);
/// A log entry in a debug trace.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TraceEntry {
    pub level: TraceLevel,
    pub message: String,
    pub location: String,
    pub context: Option<String>,
}
#[allow(dead_code)]
impl TraceEntry {
    pub fn new(level: TraceLevel, msg: &str, loc: &str) -> Self {
        TraceEntry {
            level,
            message: msg.to_string(),
            location: loc.to_string(),
            context: None,
        }
    }
    pub fn with_context(mut self, ctx: &str) -> Self {
        self.context = Some(ctx.to_string());
        self
    }
    pub fn is_error(&self) -> bool {
        self.level == TraceLevel::Error
    }
    pub fn is_debug_or_above(&self) -> bool {
        self.level >= TraceLevel::Debug
    }
}
#[allow(dead_code)]
pub struct MetaDebugExtPass3300 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<MetaDebugExtResult3300>,
}
impl MetaDebugExtPass3300 {
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
    pub fn run(&mut self, input: &str) -> MetaDebugExtResult3300 {
        if !self.enabled {
            return MetaDebugExtResult3300::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            MetaDebugExtResult3300::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            MetaDebugExtResult3300::Ok(format!(
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
/// Verbosity level for tracing.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceLevel {
    Error = 0,
    Warn = 1,
    Info = 2,
    Debug = 3,
    Trace = 4,
}
/// A utility type for MetaDbg (index 4).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaDbgUtil4 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MetaDbgUtil4 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MetaDbgUtil4 {
            id,
            name: name.to_string(),
            value,
            enabled: true,
            tags: Vec::new(),
        }
    }
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn is_active(&self) -> bool {
        self.enabled
    }
    pub fn score(&self) -> i64 {
        if self.enabled {
            self.value
        } else {
            0
        }
    }
    pub fn tag_count(&self) -> usize {
        self.tags.len()
    }
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}
/// A configuration store for MetaDebug.
#[allow(dead_code)]
pub struct MetaDebugConfig {
    pub values: std::collections::HashMap<String, MetaDebugConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl MetaDebugConfig {
    pub fn new() -> Self {
        MetaDebugConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: MetaDebugConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&MetaDebugConfigValue> {
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
        self.set(key, MetaDebugConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, MetaDebugConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, MetaDebugConfigValue::Str(v.to_string()))
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
