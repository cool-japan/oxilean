//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Expr, Name};

/// A named, versioned set of simp lemmas.
#[derive(Clone, Debug)]
pub struct SimpLemmaSet {
    /// Name of this set.
    pub name: String,
    /// Version counter.
    pub version: u64,
    /// Stored lemmas.
    pub lemmas: Vec<crate::tactic::simp::types::SimpLemma>,
}
impl SimpLemmaSet {
    /// Create an empty set.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: 0,
            lemmas: Vec::new(),
        }
    }
    /// Add a lemma.
    pub fn add(&mut self, lemma: crate::tactic::simp::types::SimpLemma) {
        self.lemmas.push(lemma);
        self.version += 1;
    }
    /// Remove a lemma by name.
    pub fn remove(&mut self, name: &Name) {
        self.lemmas.retain(|l| &l.name != name);
        self.version += 1;
    }
    /// Number of lemmas.
    pub fn len(&self) -> usize {
        self.lemmas.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.lemmas.is_empty()
    }
    /// Select lemmas matching a selector.
    pub fn select(&self, sel: &SimpLemmaSelector) -> Vec<&crate::tactic::simp::types::SimpLemma> {
        self.lemmas.iter().filter(|l| sel.accepts(l)).collect()
    }
    /// Merge another set into this one.
    pub fn merge(&mut self, other: &SimpLemmaSet) {
        for l in &other.lemmas {
            self.lemmas.push(l.clone());
        }
        self.version += 1;
    }
    /// Sort lemmas by priority (descending).
    pub fn sort_by_priority(&mut self) {
        self.lemmas.sort_by_key(|b| std::cmp::Reverse(b.priority));
    }
}
/// Selects which simp lemmas to apply based on a predicate.
#[derive(Clone, Debug)]
pub struct SimpLemmaSelector {
    /// If Some, only lemmas with name starting with this prefix.
    pub prefix: Option<String>,
    /// If Some, only lemmas with priority >= this.
    pub min_priority: Option<u32>,
    /// If true, exclude conditional lemmas.
    pub exclude_conditional: bool,
}
impl SimpLemmaSelector {
    /// Select all lemmas.
    pub fn all() -> Self {
        Self {
            prefix: None,
            min_priority: None,
            exclude_conditional: false,
        }
    }
    /// Select only unconditional lemmas.
    pub fn unconditional_only() -> Self {
        Self {
            prefix: None,
            min_priority: None,
            exclude_conditional: true,
        }
    }
    /// Select lemmas with priority at least `n`.
    pub fn with_min_priority(n: u32) -> Self {
        Self {
            min_priority: Some(n),
            ..Self::all()
        }
    }
    /// Test whether a lemma passes the selector.
    pub fn accepts(&self, lemma: &crate::tactic::simp::types::SimpLemma) -> bool {
        if let Some(ref pfx) = self.prefix {
            if !lemma.name.to_string().starts_with(pfx.as_str()) {
                return false;
            }
        }
        if let Some(min_p) = self.min_priority {
            if lemma.priority < min_p {
                return false;
            }
        }
        if self.exclude_conditional && lemma.is_conditional {
            return false;
        }
        true
    }
}
/// A configuration store for TacticSimpMain.
#[allow(dead_code)]
pub struct TacticSimpMainConfig {
    pub values: std::collections::HashMap<String, TacticSimpMainConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticSimpMainConfig {
    pub fn new() -> Self {
        TacticSimpMainConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticSimpMainConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticSimpMainConfigValue> {
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
        self.set(key, TacticSimpMainConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticSimpMainConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticSimpMainConfigValue::Str(v.to_string()))
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
/// A diff for TacticSimpMain analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticSimpMainDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticSimpMainDiff {
    pub fn new() -> Self {
        TacticSimpMainDiff {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MainExtResult1100 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl MainExtResult1100 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, MainExtResult1100::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, MainExtResult1100::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, MainExtResult1100::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, MainExtResult1100::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let MainExtResult1100::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let MainExtResult1100::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            MainExtResult1100::Ok(_) => 1.0,
            MainExtResult1100::Err(_) => 0.0,
            MainExtResult1100::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            MainExtResult1100::Skipped => 0.5,
        }
    }
}
#[allow(dead_code)]
pub struct MainExtDiff1100 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl MainExtDiff1100 {
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
/// Priority-ordered simp lemma collection.
#[allow(dead_code)]
pub struct PrioritizedSimpSet {
    pub(super) lemmas: Vec<crate::tactic::simp::types::SimpLemma>,
}
impl PrioritizedSimpSet {
    /// Create an empty prioritized set.
    pub fn new() -> Self {
        Self { lemmas: Vec::new() }
    }
    /// Add a lemma, maintaining priority order.
    pub fn insert(&mut self, lemma: crate::tactic::simp::types::SimpLemma) {
        let pos = self
            .lemmas
            .partition_point(|l| l.priority >= lemma.priority);
        self.lemmas.insert(pos, lemma);
    }
    /// Get lemmas in priority order.
    pub fn iter(&self) -> impl Iterator<Item = &crate::tactic::simp::types::SimpLemma> {
        self.lemmas.iter()
    }
    /// Number of lemmas.
    pub fn len(&self) -> usize {
        self.lemmas.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.lemmas.is_empty()
    }
    /// Apply the highest-priority matching lemma.
    pub fn apply_first(&self, expr: &Expr) -> Option<&crate::tactic::simp::types::SimpLemma> {
        self.lemmas.iter().find(|l| syntactic_match(expr, &l.lhs))
    }
}
/// A log of rewrite steps for debugging simp runs.
#[derive(Clone, Debug, Default)]
pub struct SimpRewriteLog {
    /// Entries: (lemma_name, before, after).
    pub entries: Vec<(Name, Expr, Expr)>,
    /// Maximum entries to keep (0 = unlimited).
    pub max_entries: usize,
}
impl SimpRewriteLog {
    /// Create an unlimited log.
    pub fn new() -> Self {
        Self {
            max_entries: 0,
            ..Self::default()
        }
    }
    /// Create a bounded log.
    pub fn bounded(max: usize) -> Self {
        Self {
            max_entries: max,
            ..Self::default()
        }
    }
    /// Record a rewrite step.
    pub fn record(&mut self, lemma: Name, before: Expr, after: Expr) {
        if self.max_entries > 0 && self.entries.len() >= self.max_entries {
            return;
        }
        self.entries.push((lemma, before, after));
    }
    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear the log.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Get all lemma names used.
    pub fn lemmas_used(&self) -> Vec<&Name> {
        self.entries.iter().map(|(n, _, _)| n).collect()
    }
    /// Get the before-after pair for the nth step.
    pub fn get_step(&self, idx: usize) -> Option<(&Name, &Expr, &Expr)> {
        self.entries.get(idx).map(|(n, b, a)| (n, b, a))
    }
    /// Count distinct lemmas used.
    pub fn distinct_lemma_count(&self) -> usize {
        let mut seen = std::collections::HashSet::new();
        for (n, _, _) in &self.entries {
            seen.insert(n.to_string());
        }
        seen.len()
    }
}
#[allow(dead_code)]
pub struct MainExtPass1100 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<MainExtResult1100>,
}
impl MainExtPass1100 {
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
    pub fn run(&mut self, input: &str) -> MainExtResult1100 {
        if !self.enabled {
            return MainExtResult1100::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            MainExtResult1100::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            MainExtResult1100::Ok(format!(
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
/// A diagnostic reporter for TacticSimpMain.
#[allow(dead_code)]
pub struct TacticSimpMainDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticSimpMainDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticSimpMainDiagnostics {
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
/// A typed slot for TacticSimpMain configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticSimpMainConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticSimpMainConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticSimpMainConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticSimpMainConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticSimpMainConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticSimpMainConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticSimpMainConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticSimpMainConfigValue::Bool(_) => "bool",
            TacticSimpMainConfigValue::Int(_) => "int",
            TacticSimpMainConfigValue::Float(_) => "float",
            TacticSimpMainConfigValue::Str(_) => "str",
            TacticSimpMainConfigValue::List(_) => "list",
        }
    }
}
/// Statistics collected during a simp run.
#[derive(Debug, Default, Clone)]
pub struct SimpStats {
    /// Number of lemmas tested.
    pub lemmas_tested: usize,
    /// Number of successful rewrites.
    pub rewrites_applied: usize,
    /// Number of beta reductions performed.
    pub beta_reductions: usize,
    /// Number of congruence steps.
    pub congruence_steps: usize,
    /// Total simp steps.
    pub total_steps: usize,
    /// Whether the simp loop hit the step limit.
    pub hit_limit: bool,
}
impl SimpStats {
    /// Create zeroed statistics.
    pub fn new() -> Self {
        Self::default()
    }
    /// Merge another stats record into this one.
    pub fn merge(&mut self, other: &SimpStats) {
        self.lemmas_tested += other.lemmas_tested;
        self.rewrites_applied += other.rewrites_applied;
        self.beta_reductions += other.beta_reductions;
        self.congruence_steps += other.congruence_steps;
        self.total_steps += other.total_steps;
        self.hit_limit |= other.hit_limit;
    }
}
/// A result type for TacticSimpMain analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticSimpMainResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticSimpMainResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticSimpMainResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticSimpMainResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticSimpMainResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticSimpMainResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticSimpMainResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticSimpMainResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticSimpMainResult::Ok(_) => 1.0,
            TacticSimpMainResult::Err(_) => 0.0,
            TacticSimpMainResult::Skipped => 0.0,
            TacticSimpMainResult::Partial { done, total } => {
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
pub struct MainExtPipeline1100 {
    pub name: String,
    pub passes: Vec<MainExtPass1100>,
    pub run_count: usize,
}
impl MainExtPipeline1100 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: MainExtPass1100) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<MainExtResult1100> {
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
#[derive(Debug, Clone)]
pub enum MainExtConfigVal1100 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl MainExtConfigVal1100 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let MainExtConfigVal1100::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let MainExtConfigVal1100::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let MainExtConfigVal1100::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let MainExtConfigVal1100::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let MainExtConfigVal1100::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            MainExtConfigVal1100::Bool(_) => "bool",
            MainExtConfigVal1100::Int(_) => "int",
            MainExtConfigVal1100::Float(_) => "float",
            MainExtConfigVal1100::Str(_) => "str",
            MainExtConfigVal1100::List(_) => "list",
        }
    }
}
/// A pipeline of TacticSimpMain analysis passes.
#[allow(dead_code)]
pub struct TacticSimpMainPipeline {
    pub passes: Vec<TacticSimpMainAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticSimpMainPipeline {
    pub fn new(name: &str) -> Self {
        TacticSimpMainPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticSimpMainAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticSimpMainResult> {
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
/// Simp lemma filter: determines which lemmas apply to an expression.
#[allow(dead_code)]
pub struct SimpLemmaFilter {
    /// Only allow lemmas matching these head symbols.
    pub(super) head_whitelist: Option<Vec<Name>>,
    /// Exclude lemmas with these head symbols.
    pub(super) head_blacklist: Vec<Name>,
    /// Priority threshold: only use lemmas with priority >= this.
    pub(super) min_priority: i32,
}
impl SimpLemmaFilter {
    /// Create a new filter that accepts all lemmas.
    pub fn new() -> Self {
        Self {
            head_whitelist: None,
            head_blacklist: Vec::new(),
            min_priority: 0,
        }
    }
    /// Restrict to lemmas matching specific head symbols.
    pub fn with_whitelist(mut self, heads: Vec<Name>) -> Self {
        self.head_whitelist = Some(heads);
        self
    }
    /// Exclude lemmas with these head symbols.
    pub fn with_blacklist(mut self, heads: Vec<Name>) -> Self {
        self.head_blacklist = heads;
        self
    }
    /// Only accept lemmas with priority at least min.
    pub fn min_priority(mut self, min: i32) -> Self {
        self.min_priority = min;
        self
    }
    /// Check if a lemma passes this filter.
    pub fn accepts(&self, lemma: &crate::tactic::simp::types::SimpLemma) -> bool {
        if (lemma.priority as i32) < self.min_priority {
            return false;
        }
        let head = expr_head_name(&lemma.lhs);
        if let Some(wl) = &self.head_whitelist {
            if !wl.iter().any(|n| head.as_ref() == Some(n)) {
                return false;
            }
        }
        if let Some(h) = &head {
            if self.head_blacklist.contains(h) {
                return false;
            }
        }
        true
    }
}
#[allow(dead_code)]
pub struct MainExtDiag1100 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl MainExtDiag1100 {
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
/// Aggregate simp statistics across multiple runs.
#[derive(Clone, Debug, Default)]
pub struct SimpRunSummary {
    /// Number of simp invocations.
    pub num_runs: u64,
    /// Total rewrites across all runs.
    pub total_rewrites: u64,
    /// Runs that made no progress.
    pub unchanged_runs: u64,
    /// Runs that fully proved the goal.
    pub proved_runs: u64,
}
impl SimpRunSummary {
    /// Create empty summary.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a single simp run result.
    pub fn record(&mut self, result: &crate::tactic::simp::types::SimpResult) {
        self.num_runs += 1;
        match result {
            crate::tactic::simp::types::SimpResult::Simplified { .. } => {
                self.total_rewrites += 1;
            }
            crate::tactic::simp::types::SimpResult::Proved(_) => {
                self.proved_runs += 1;
            }
            crate::tactic::simp::types::SimpResult::Unchanged => {
                self.unchanged_runs += 1;
            }
        }
    }
    /// Fraction of runs that made progress.
    pub fn progress_rate(&self) -> f32 {
        if self.num_runs == 0 {
            0.0
        } else {
            (self.num_runs - self.unchanged_runs) as f32 / self.num_runs as f32
        }
    }
    /// Total effective runs (simplified + proved).
    pub fn effective_runs(&self) -> u64 {
        self.num_runs - self.unchanged_runs
    }
}
/// An analysis pass for TacticSimpMain.
#[allow(dead_code)]
pub struct TacticSimpMainAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticSimpMainResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticSimpMainAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticSimpMainAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticSimpMainResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticSimpMainResult::Err("empty input".to_string())
        } else {
            TacticSimpMainResult::Ok(format!("processed: {}", input))
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
pub struct MainExtConfig1100 {
    pub(super) values: std::collections::HashMap<String, MainExtConfigVal1100>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl MainExtConfig1100 {
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
    pub fn set(&mut self, key: &str, value: MainExtConfigVal1100) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&MainExtConfigVal1100> {
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
        self.set(key, MainExtConfigVal1100::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, MainExtConfigVal1100::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, MainExtConfigVal1100::Str(v.to_string()))
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
