//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::MVarId;
use crate::tactic::state::TacticState;
use oxilean_kernel::{Expr, Name};

/// A utility type for TacCases (index 6).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil6 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil6 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil6 {
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
/// A result type for TacticCases analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticCasesResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticCasesResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticCasesResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticCasesResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticCasesResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticCasesResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticCasesResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticCasesResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticCasesResult::Ok(_) => 1.0,
            TacticCasesResult::Err(_) => 0.0,
            TacticCasesResult::Skipped => 0.0,
            TacticCasesResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
#[allow(dead_code)]
pub struct CasesExtPipeline3900 {
    pub name: String,
    pub passes: Vec<CasesExtPass3900>,
    pub run_count: usize,
}
impl CasesExtPipeline3900 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: CasesExtPass3900) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<CasesExtResult3900> {
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
/// A utility type for TacCases (index 0).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil0 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil0 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil0 {
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
/// A utility type for TacCases (index 2).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil2 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil2 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil2 {
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
/// A utility type for TacCases (index 3).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil3 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil3 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil3 {
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
/// A utility type for TacCases (index 5).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil5 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil5 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil5 {
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
/// A utility type for TacCases (index 11).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil11 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil11 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil11 {
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
/// A utility type for TacCases (index 7).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil7 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil7 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil7 {
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
/// A utility type for TacCases (index 13).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil13 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil13 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil13 {
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
/// A priority queue for TacCases items.
#[allow(dead_code)]
pub struct TacCasesPriorityQueue {
    pub items: Vec<(TacCasesUtil0, i64)>,
}
#[allow(dead_code)]
impl TacCasesPriorityQueue {
    pub fn new() -> Self {
        TacCasesPriorityQueue { items: Vec::new() }
    }
    pub fn push(&mut self, item: TacCasesUtil0, priority: i64) {
        self.items.push((item, priority));
        self.items.sort_by_key(|(_, p)| -p);
    }
    pub fn pop(&mut self) -> Option<(TacCasesUtil0, i64)> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
    pub fn peek(&self) -> Option<&(TacCasesUtil0, i64)> {
        self.items.first()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
/// A pipeline of TacticCases analysis passes.
#[allow(dead_code)]
pub struct TacticCasesPipeline {
    pub passes: Vec<TacticCasesAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticCasesPipeline {
    pub fn new(name: &str) -> Self {
        TacticCasesPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticCasesAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticCasesResult> {
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
/// Statistics for TacCases operations.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesStats {
    pub total_ops: usize,
    pub successful_ops: usize,
    pub failed_ops: usize,
    pub total_time_ns: u64,
    pub max_time_ns: u64,
}
#[allow(dead_code)]
impl TacCasesStats {
    pub fn new() -> Self {
        TacCasesStats::default()
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
/// A diff for TacticCases analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticCasesDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticCasesDiff {
    pub fn new() -> Self {
        TacticCasesDiff {
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
/// A diagnostic reporter for TacticCases.
#[allow(dead_code)]
pub struct TacticCasesDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticCasesDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticCasesDiagnostics {
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
#[allow(dead_code)]
pub struct CasesExtConfig3900 {
    pub(super) values: std::collections::HashMap<String, CasesExtConfigVal3900>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl CasesExtConfig3900 {
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
    pub fn set(&mut self, key: &str, value: CasesExtConfigVal3900) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&CasesExtConfigVal3900> {
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
        self.set(key, CasesExtConfigVal3900::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, CasesExtConfigVal3900::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, CasesExtConfigVal3900::Str(v.to_string()))
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
/// A utility type for TacCases (index 8).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil8 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil8 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil8 {
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
pub struct CasesExtPass3900 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<CasesExtResult3900>,
}
impl CasesExtPass3900 {
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
    pub fn run(&mut self, input: &str) -> CasesExtResult3900 {
        if !self.enabled {
            return CasesExtResult3900::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            CasesExtResult3900::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            CasesExtResult3900::Ok(format!(
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
/// A utility type for TacCases (index 12).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil12 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil12 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil12 {
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
/// Result of an `induction` tactic application.
#[derive(Clone, Debug)]
pub struct InductionResult {
    /// New goals, one per constructor.
    pub goals: Vec<InductionGoal>,
}
/// A single goal produced by `induction`.
#[derive(Clone, Debug)]
pub struct InductionGoal {
    /// Goal metavariable.
    pub mvar_id: MVarId,
    /// Constructor name for this case.
    pub ctor_name: Name,
    /// Field names introduced.
    pub field_names: Vec<Name>,
    /// Induction hypothesis names (for recursive constructors).
    pub ih_names: Vec<Name>,
}
/// A complete case-split record (all branches).
#[allow(dead_code)]
pub struct CaseSplit {
    /// The inductive type name that was split on.
    pub type_name: Name,
    /// The individual constructor branches.
    pub branches: Vec<ConstructorBranch>,
}
#[allow(dead_code)]
impl CaseSplit {
    /// Create from a list of ConstructorBranch items.
    pub fn from_branches(type_name: Name, branches: Vec<ConstructorBranch>) -> Self {
        CaseSplit {
            type_name,
            branches,
        }
    }
    /// Return the number of branches.
    pub fn branch_count(&self) -> usize {
        self.branches.len()
    }
    /// Collect all branch states.
    pub fn branch_states(&self) -> Vec<&TacticState> {
        self.branches.iter().map(|b| &b.state).collect()
    }
}
/// A utility type for TacCases (index 1).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil1 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil1 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil1 {
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
/// A logger for TacCases operations.
#[allow(dead_code)]
pub struct TacCasesLogger {
    pub entries: Vec<String>,
    pub max_entries: usize,
    pub verbose: bool,
}
#[allow(dead_code)]
impl TacCasesLogger {
    pub fn new(max_entries: usize) -> Self {
        TacCasesLogger {
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
/// A typed slot for TacticCases configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticCasesConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticCasesConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticCasesConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticCasesConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticCasesConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticCasesConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticCasesConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticCasesConfigValue::Bool(_) => "bool",
            TacticCasesConfigValue::Int(_) => "int",
            TacticCasesConfigValue::Float(_) => "float",
            TacticCasesConfigValue::Str(_) => "str",
            TacticCasesConfigValue::List(_) => "list",
        }
    }
}
/// A configuration store for TacticCases.
#[allow(dead_code)]
pub struct TacticCasesConfig {
    pub values: std::collections::HashMap<String, TacticCasesConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticCasesConfig {
    pub fn new() -> Self {
        TacticCasesConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticCasesConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticCasesConfigValue> {
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
        self.set(key, TacticCasesConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticCasesConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticCasesConfigValue::Str(v.to_string()))
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
/// Result of a `cases` tactic application.
#[derive(Clone, Debug)]
pub struct CasesResult {
    /// New goals, one per constructor.
    pub goals: Vec<CasesGoal>,
}
#[allow(dead_code)]
pub struct CasesExtDiff3900 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl CasesExtDiff3900 {
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
#[derive(Debug, Clone)]
pub enum CasesExtResult3900 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl CasesExtResult3900 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, CasesExtResult3900::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, CasesExtResult3900::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, CasesExtResult3900::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, CasesExtResult3900::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let CasesExtResult3900::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let CasesExtResult3900::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            CasesExtResult3900::Ok(_) => 1.0,
            CasesExtResult3900::Err(_) => 0.0,
            CasesExtResult3900::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            CasesExtResult3900::Skipped => 0.5,
        }
    }
}
/// A utility type for TacCases (index 4).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil4 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil4 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil4 {
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
/// An analysis pass for TacticCases.
#[allow(dead_code)]
pub struct TacticCasesAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticCasesResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticCasesAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticCasesAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticCasesResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticCasesResult::Err("empty input".to_string())
        } else {
            TacticCasesResult::Ok(format!("processed: {}", input))
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CasesExtConfigVal3900 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl CasesExtConfigVal3900 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let CasesExtConfigVal3900::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let CasesExtConfigVal3900::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let CasesExtConfigVal3900::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let CasesExtConfigVal3900::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let CasesExtConfigVal3900::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            CasesExtConfigVal3900::Bool(_) => "bool",
            CasesExtConfigVal3900::Int(_) => "int",
            CasesExtConfigVal3900::Float(_) => "float",
            CasesExtConfigVal3900::Str(_) => "str",
            CasesExtConfigVal3900::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct CasesExtDiag3900 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl CasesExtDiag3900 {
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
/// Result of a structural case split.
#[allow(dead_code)]
pub struct ConstructorBranch {
    /// The constructor name for this branch.
    pub ctor_name: Name,
    /// New hypotheses introduced by this branch.
    pub new_hyps: Vec<(String, Expr)>,
    /// The updated tactic state for this branch.
    pub state: TacticState,
}
/// A single goal produced by `cases`.
#[derive(Clone, Debug)]
pub struct CasesGoal {
    /// Goal metavariable.
    pub mvar_id: MVarId,
    /// Constructor name for this case.
    pub ctor_name: Name,
    /// Field names introduced.
    pub field_names: Vec<Name>,
}
/// A registry for TacCases utilities.
#[allow(dead_code)]
pub struct TacCasesRegistry {
    pub entries: Vec<TacCasesUtil0>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl TacCasesRegistry {
    pub fn new(capacity: usize) -> Self {
        TacCasesRegistry {
            entries: Vec::new(),
            capacity,
        }
    }
    pub fn register(&mut self, entry: TacCasesUtil0) -> bool {
        if self.entries.len() >= self.capacity {
            return false;
        }
        self.entries.push(entry);
        true
    }
    pub fn lookup(&self, id: usize) -> Option<&TacCasesUtil0> {
        self.entries.iter().find(|e| e.id == id)
    }
    pub fn remove(&mut self, id: usize) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < before
    }
    pub fn active_entries(&self) -> Vec<&TacCasesUtil0> {
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
/// Classify an expression into a structural category for case-analysis purposes.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructuralClass {
    /// Inductive type with a known constructor set.
    KnownInductive,
    /// Proposition (Sort 0).
    Proposition,
    /// A function type (Pi).
    FunctionType,
    /// A sort (Type/Prop).
    Sort,
    /// Something else.
    Other,
}
/// A utility type for TacCases (index 9).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil9 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil9 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil9 {
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
/// A utility type for TacCases (index 10).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCasesUtil10 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl TacCasesUtil10 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        TacCasesUtil10 {
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
/// A simple cache for TacCases computations.
#[allow(dead_code)]
pub struct TacCasesCache {
    pub data: std::collections::HashMap<String, i64>,
    pub hits: usize,
    pub misses: usize,
}
#[allow(dead_code)]
impl TacCasesCache {
    pub fn new() -> Self {
        TacCasesCache {
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
