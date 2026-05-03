//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::super::functions::*;
use oxilean_kernel::Expr;

use super::types::{MetaDebugExtPass3300, MetaDebugExtResult3300};

use std::collections::HashMap;

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
/// Collect all BVar indices.
#[allow(dead_code)]
pub struct BVarCollector(pub Vec<u32>);
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
