//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::match_basic::MetaPattern;
use oxilean_kernel::Name;

/// A map tracking which constructors are covered.
#[derive(Debug, Clone, Default)]
pub struct CoverageMap {
    /// Covered constructors (name → arm indices that cover it).
    pub(super) covered: std::collections::HashMap<String, Vec<usize>>,
    /// Uncovered constructors.
    pub(super) uncovered: Vec<Name>,
}
impl CoverageMap {
    /// Create a new empty coverage map.
    pub fn new() -> Self {
        Self::default()
    }
    /// Initialize from a list of constructors (all uncovered).
    pub fn from_constructors(ctors: &[ConstructorSpec]) -> Self {
        let mut map = CoverageMap::new();
        for c in ctors {
            map.uncovered.push(c.name.clone());
        }
        map
    }
    /// Mark a constructor as covered by the given arm.
    pub fn mark_covered(&mut self, ctor: &Name, arm_idx: usize) {
        let key = format!("{}", ctor);
        self.covered.entry(key).or_default().push(arm_idx);
        self.uncovered.retain(|c| c != ctor);
    }
    /// Mark a constructor as covered by a catch-all pattern.
    pub fn mark_covered_by_catchall(&mut self, ctor: &Name) {
        self.mark_covered(ctor, usize::MAX);
    }
    /// Returns `true` if all constructors are covered.
    pub fn is_complete(&self) -> bool {
        self.uncovered.is_empty()
    }
    /// Get the list of uncovered constructors.
    pub fn uncovered_ctors(&self) -> &[Name] {
        &self.uncovered
    }
    /// Get the arms that cover a given constructor.
    pub fn covering_arms(&self, ctor: &Name) -> &[usize] {
        let key = format!("{}", ctor);
        self.covered.get(&key).map(|v| v.as_slice()).unwrap_or(&[])
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MatchExhaustExtConfigVal2000 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl MatchExhaustExtConfigVal2000 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let MatchExhaustExtConfigVal2000::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let MatchExhaustExtConfigVal2000::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let MatchExhaustExtConfigVal2000::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let MatchExhaustExtConfigVal2000::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let MatchExhaustExtConfigVal2000::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            MatchExhaustExtConfigVal2000::Bool(_) => "bool",
            MatchExhaustExtConfigVal2000::Int(_) => "int",
            MatchExhaustExtConfigVal2000::Float(_) => "float",
            MatchExhaustExtConfigVal2000::Str(_) => "str",
            MatchExhaustExtConfigVal2000::List(_) => "list",
        }
    }
}
/// Result of exhaustiveness checking.
#[derive(Clone, Debug)]
pub struct ExhaustivenessResult {
    /// Whether the match is exhaustive.
    pub is_exhaustive: bool,
    /// Missing constructor patterns (if not exhaustive).
    pub missing: Vec<MissingPattern>,
    /// Unreachable arms (indices).
    pub unreachable_arms: Vec<usize>,
}
impl ExhaustivenessResult {
    /// Returns `true` if the match is both exhaustive and has no redundant arms.
    pub fn is_perfect(&self) -> bool {
        self.is_exhaustive && self.unreachable_arms.is_empty()
    }
    /// Returns the number of missing cases.
    pub fn num_missing(&self) -> usize {
        self.missing.len()
    }
    /// Returns the number of unreachable arms.
    pub fn num_unreachable(&self) -> usize {
        self.unreachable_arms.len()
    }
    /// Format a human-readable summary of the result.
    pub fn summary(&self) -> String {
        if self.is_perfect() {
            return "perfect match: exhaustive with no redundant arms".to_string();
        }
        let mut parts = Vec::new();
        if !self.is_exhaustive {
            parts.push(format!("missing {} case(s)", self.missing.len()));
        }
        if !self.unreachable_arms.is_empty() {
            parts.push(format!(
                "{} unreachable arm(s)",
                self.unreachable_arms.len()
            ));
        }
        parts.join("; ")
    }
}
/// An extended utility type for MatchExhaust.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchExhaustExt2 {
    /// A numeric tag.
    pub tag: u32,
}
#[allow(dead_code)]
impl MatchExhaustExt2 {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { tag: 0 }
    }
}
/// Redundancy information for a single arm.
#[derive(Debug, Clone)]
pub struct ArmRedundancy {
    /// Arm index (0-based).
    pub arm_index: usize,
    /// Whether the arm is redundant.
    pub is_redundant: bool,
    /// Which earlier arm subsumes this one (if redundant).
    pub subsumed_by: Option<usize>,
}
impl ArmRedundancy {
    /// Returns `true` if the arm is useful (not redundant).
    pub fn is_useful(&self) -> bool {
        !self.is_redundant
    }
}
/// A result type for MatchExhaust analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MatchExhaustResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl MatchExhaustResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, MatchExhaustResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, MatchExhaustResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, MatchExhaustResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, MatchExhaustResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            MatchExhaustResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            MatchExhaustResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            MatchExhaustResult::Ok(_) => 1.0,
            MatchExhaustResult::Err(_) => 0.0,
            MatchExhaustResult::Skipped => 0.0,
            MatchExhaustResult::Partial { done, total } => {
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
pub struct MatchExhaustExtPipeline2000 {
    pub name: String,
    pub passes: Vec<MatchExhaustExtPass2000>,
    pub run_count: usize,
}
impl MatchExhaustExtPipeline2000 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: MatchExhaustExtPass2000) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<MatchExhaustExtResult2000> {
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
/// An analysis pass for MatchExhaust.
#[allow(dead_code)]
pub struct MatchExhaustAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<MatchExhaustResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl MatchExhaustAnalysisPass {
    pub fn new(name: &str) -> Self {
        MatchExhaustAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> MatchExhaustResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            MatchExhaustResult::Err("empty input".to_string())
        } else {
            MatchExhaustResult::Ok(format!("processed: {}", input))
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
/// A builder pattern for MatchExhaust.
#[allow(dead_code)]
pub struct MatchExhaustBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl MatchExhaustBuilder {
    pub fn new(name: &str) -> Self {
        MatchExhaustBuilder {
            name: name.to_string(),
            items: Vec::new(),
            config: std::collections::HashMap::new(),
        }
    }
    pub fn add_item(mut self, item: &str) -> Self {
        self.items.push(item.to_string());
        self
    }
    pub fn set_config(mut self, key: &str, value: &str) -> Self {
        self.config.insert(key.to_string(), value.to_string());
        self
    }
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    pub fn has_config(&self, key: &str) -> bool {
        self.config.contains_key(key)
    }
    pub fn get_config(&self, key: &str) -> Option<&str> {
        self.config.get(key).map(|s| s.as_str())
    }
    pub fn build_summary(&self) -> String {
        format!(
            "{}: {} items, {} config keys",
            self.name,
            self.items.len(),
            self.config.len()
        )
    }
}
/// Verify that a pattern list is complete (exhaustive AND irredundant).
///
/// Returns a `CompletenessReport` with detailed diagnostics.
#[derive(Debug, Clone)]
pub struct CompletenessReport {
    /// Whether the match is exhaustive.
    pub exhaustive: bool,
    /// Missing patterns.
    pub missing_patterns: Vec<MissingPattern>,
    /// Redundant arms with their reasons.
    pub redundant_arms: Vec<RedundantArm>,
    /// Total number of arms.
    pub total_arms: usize,
    /// Number of useful (non-redundant) arms.
    pub useful_arms: usize,
}
impl CompletenessReport {
    /// Returns `true` if the match is both exhaustive and irredundant.
    pub fn is_complete(&self) -> bool {
        self.exhaustive && self.redundant_arms.is_empty()
    }
    /// Format a human-readable report.
    pub fn format(&self) -> String {
        let mut lines = Vec::new();
        if self.exhaustive {
            lines.push("✓ Exhaustive".to_string());
        } else {
            lines.push(format!(
                "✗ Not exhaustive: {} missing case(s)",
                self.missing_patterns.len()
            ));
            for m in &self.missing_patterns {
                lines.push(format!("  - {}", m));
            }
        }
        if self.redundant_arms.is_empty() {
            lines.push("✓ No redundant arms".to_string());
        } else {
            lines.push(format!("⚠ {} redundant arm(s)", self.redundant_arms.len()));
            for r in &self.redundant_arms {
                lines.push(format!("  - arm #{}", r.arm_index));
            }
        }
        lines.join("\n")
    }
}
/// An extended map for MatchExhaust keys to values.
#[allow(dead_code)]
pub struct MatchExhaustExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> MatchExhaustExtMap<V> {
    pub fn new() -> Self {
        MatchExhaustExtMap {
            data: std::collections::HashMap::new(),
            default_key: None,
        }
    }
    pub fn insert(&mut self, key: &str, value: V) {
        self.data.insert(key.to_string(), value);
    }
    pub fn get(&self, key: &str) -> Option<&V> {
        self.data.get(key)
    }
    pub fn get_or_default(&self, key: &str) -> V {
        self.data.get(key).cloned().unwrap_or_default()
    }
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.data.remove(key)
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn set_default(&mut self, key: &str) {
        self.default_key = Some(key.to_string());
    }
    pub fn keys_sorted(&self) -> Vec<&String> {
        let mut keys: Vec<&String> = self.data.keys().collect();
        keys.sort();
        keys
    }
}
#[allow(dead_code)]
pub struct MatchExhaustExtConfig2000 {
    pub(super) values: std::collections::HashMap<String, MatchExhaustExtConfigVal2000>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl MatchExhaustExtConfig2000 {
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
    pub fn set(&mut self, key: &str, value: MatchExhaustExtConfigVal2000) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&MatchExhaustExtConfigVal2000> {
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
        self.set(key, MatchExhaustExtConfigVal2000::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, MatchExhaustExtConfigVal2000::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, MatchExhaustExtConfigVal2000::Str(v.to_string()))
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
/// Why an arm is redundant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedundancyReason {
    /// A previous arm has a wildcard that catches everything.
    PreviousCatchall {
        /// Index of the arm containing the catch-all.
        catchall_arm: usize,
    },
    /// A previous arm has the same constructor.
    DuplicateConstructor {
        /// Index of the previous arm with the same constructor.
        previous_arm: usize,
    },
    /// A previous arm's pattern subsumes this arm's pattern.
    SubsumedBy {
        /// Index of the arm that subsumes this one.
        subsuming_arm: usize,
    },
}
/// A state machine for MatchExhaust.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MatchExhaustState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl MatchExhaustState {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            MatchExhaustState::Complete | MatchExhaustState::Failed(_)
        )
    }
    pub fn can_run(&self) -> bool {
        matches!(self, MatchExhaustState::Initial | MatchExhaustState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, MatchExhaustState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            MatchExhaustState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// A configuration store for MatchExhaust.
#[allow(dead_code)]
pub struct MatchExhaustConfig {
    pub values: std::collections::HashMap<String, MatchExhaustConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl MatchExhaustConfig {
    pub fn new() -> Self {
        MatchExhaustConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: MatchExhaustConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&MatchExhaustConfigValue> {
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
        self.set(key, MatchExhaustConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, MatchExhaustConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, MatchExhaustConfigValue::Str(v.to_string()))
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
/// Signature of an inductive type for exhaustiveness purposes.
///
/// Describes the constructors of a type, used to determine which cases
/// need to be covered.
#[derive(Clone, Debug)]
pub struct InductiveSignature {
    /// Name of the inductive type.
    pub type_name: Name,
    /// Constructors of the type.
    pub constructors: Vec<ConstructorSpec>,
    /// Whether this type is a proposition (Prop).
    pub is_prop: bool,
    /// Whether this type has a finite number of inhabitants.
    pub is_finite: bool,
}
impl InductiveSignature {
    /// Create a signature for the Bool type.
    pub fn mk_bool() -> Self {
        InductiveSignature {
            type_name: Name::str("Bool"),
            constructors: vec![
                ConstructorSpec::new(Name::str("Bool.true"), 0, Name::str("Bool")),
                ConstructorSpec::new(Name::str("Bool.false"), 0, Name::str("Bool")),
            ],
            is_prop: false,
            is_finite: true,
        }
    }
    /// Create a signature for the Nat type.
    pub fn mk_nat() -> Self {
        InductiveSignature {
            type_name: Name::str("Nat"),
            constructors: vec![
                ConstructorSpec::new(Name::str("Nat.zero"), 0, Name::str("Nat")),
                ConstructorSpec::new(Name::str("Nat.succ"), 1, Name::str("Nat")),
            ],
            is_prop: false,
            is_finite: false,
        }
    }
    /// Create a signature for a simple Option-like type.
    pub fn mk_option(_inner_name: Name) -> Self {
        let type_name = Name::str("Option");
        InductiveSignature {
            type_name: type_name.clone(),
            constructors: vec![
                ConstructorSpec::new(Name::str("Option.none"), 0, type_name.clone()),
                ConstructorSpec::new(Name::str("Option.some"), 1, type_name),
            ],
            is_prop: false,
            is_finite: false,
        }
    }
    /// Get the constructor with the given name.
    pub fn get_constructor(&self, name: &Name) -> Option<&ConstructorSpec> {
        self.constructors.iter().find(|c| &c.name == name)
    }
    /// Returns `true` if the type has exactly one constructor.
    pub fn is_singleton(&self) -> bool {
        self.constructors.len() == 1
    }
    /// Returns the number of constructors.
    pub fn num_constructors(&self) -> usize {
        self.constructors.len()
    }
    /// Check exhaustiveness of a pattern list against this type.
    pub fn check_exhaustive(
        &self,
        patterns: &[Vec<MetaPattern>],
        col: usize,
    ) -> ExhaustivenessResult {
        check_exhaustive(&self.constructors, patterns, col)
    }
}
pub struct MatchExhaustExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl MatchExhaustExtUtil {
    pub fn new(key: &str) -> Self {
        MatchExhaustExtUtil {
            key: key.to_string(),
            data: Vec::new(),
            active: true,
            flags: 0,
        }
    }
    pub fn push(&mut self, v: i64) {
        self.data.push(v);
    }
    pub fn pop(&mut self) -> Option<i64> {
        self.data.pop()
    }
    pub fn sum(&self) -> i64 {
        self.data.iter().sum()
    }
    pub fn min_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::min)
    }
    pub fn max_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::max)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn set_flag(&mut self, bit: u32) {
        self.flags |= 1 << bit;
    }
    pub fn has_flag(&self, bit: u32) -> bool {
        self.flags & (1 << bit) != 0
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    pub fn activate(&mut self) {
        self.active = true;
    }
}
/// An extended utility type for MatchExhaust.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchExhaustExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl MatchExhaustExt {
    /// Creates a new default instance.
    pub fn new() -> Self {
        Self {
            tag: 0,
            description: None,
        }
    }
    /// Sets the tag.
    pub fn with_tag(mut self, tag: u32) -> Self {
        self.tag = tag;
        self
    }
    /// Sets the description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    /// Returns `true` if the description is set.
    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }
}
/// A counter map for MatchExhaust frequency analysis.
#[allow(dead_code)]
pub struct MatchExhaustCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl MatchExhaustCounterMap {
    pub fn new() -> Self {
        MatchExhaustCounterMap {
            counts: std::collections::HashMap::new(),
            total: 0,
        }
    }
    pub fn increment(&mut self, key: &str) {
        *self.counts.entry(key.to_string()).or_insert(0) += 1;
        self.total += 1;
    }
    pub fn count(&self, key: &str) -> usize {
        *self.counts.get(key).unwrap_or(&0)
    }
    pub fn frequency(&self, key: &str) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.count(key) as f64 / self.total as f64
        }
    }
    pub fn most_common(&self) -> Option<(&String, usize)> {
        self.counts
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(k, &v)| (k, v))
    }
    pub fn num_unique(&self) -> usize {
        self.counts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
}
/// A pipeline of MatchExhaust analysis passes.
#[allow(dead_code)]
pub struct MatchExhaustPipeline {
    pub passes: Vec<MatchExhaustAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl MatchExhaustPipeline {
    pub fn new(name: &str) -> Self {
        MatchExhaustPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: MatchExhaustAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<MatchExhaustResult> {
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
/// A diff for MatchExhaust analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatchExhaustDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl MatchExhaustDiff {
    pub fn new() -> Self {
        MatchExhaustDiff {
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
/// Information about a constructor used in exhaustiveness checking.
#[derive(Clone, Debug)]
pub struct ConstructorSpec {
    /// Constructor name.
    pub name: Name,
    /// Number of fields.
    pub num_fields: u32,
    /// Inductive type name.
    pub induct_name: Name,
}
impl ConstructorSpec {
    /// Create a new constructor specification.
    pub fn new(name: Name, num_fields: u32, induct_name: Name) -> Self {
        ConstructorSpec {
            name,
            num_fields,
            induct_name,
        }
    }
    /// Returns `true` if this constructor takes no arguments.
    pub fn is_nullary(&self) -> bool {
        self.num_fields == 0
    }
    /// Returns `true` if this constructor takes exactly one argument.
    pub fn is_unary(&self) -> bool {
        self.num_fields == 1
    }
}
/// A sliding window accumulator for MatchExhaust.
#[allow(dead_code)]
pub struct MatchExhaustWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl MatchExhaustWindow {
    pub fn new(capacity: usize) -> Self {
        MatchExhaustWindow {
            buffer: std::collections::VecDeque::new(),
            capacity,
            running_sum: 0.0,
        }
    }
    pub fn push(&mut self, v: f64) {
        if self.buffer.len() >= self.capacity {
            if let Some(old) = self.buffer.pop_front() {
                self.running_sum -= old;
            }
        }
        self.buffer.push_back(v);
        self.running_sum += v;
    }
    pub fn mean(&self) -> f64 {
        if self.buffer.is_empty() {
            0.0
        } else {
            self.running_sum / self.buffer.len() as f64
        }
    }
    pub fn variance(&self) -> f64 {
        if self.buffer.len() < 2 {
            return 0.0;
        }
        let m = self.mean();
        self.buffer.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / self.buffer.len() as f64
    }
    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
/// A typed slot for MatchExhaust configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MatchExhaustConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl MatchExhaustConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            MatchExhaustConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            MatchExhaustConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            MatchExhaustConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            MatchExhaustConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            MatchExhaustConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            MatchExhaustConfigValue::Bool(_) => "bool",
            MatchExhaustConfigValue::Int(_) => "int",
            MatchExhaustConfigValue::Float(_) => "float",
            MatchExhaustConfigValue::Str(_) => "str",
            MatchExhaustConfigValue::List(_) => "list",
        }
    }
}
/// A diagnostic reporter for MatchExhaust.
#[allow(dead_code)]
pub struct MatchExhaustDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl MatchExhaustDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        MatchExhaustDiagnostics {
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
#[derive(Debug, Clone)]
pub enum MatchExhaustExtResult2000 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl MatchExhaustExtResult2000 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, MatchExhaustExtResult2000::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, MatchExhaustExtResult2000::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, MatchExhaustExtResult2000::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, MatchExhaustExtResult2000::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let MatchExhaustExtResult2000::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let MatchExhaustExtResult2000::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            MatchExhaustExtResult2000::Ok(_) => 1.0,
            MatchExhaustExtResult2000::Err(_) => 0.0,
            MatchExhaustExtResult2000::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            MatchExhaustExtResult2000::Skipped => 0.5,
        }
    }
}
/// A pattern matrix for multi-column exhaustiveness checking.
///
/// Each row corresponds to one match arm, and each column corresponds to
/// one position being matched against.
#[derive(Clone, Debug, Default)]
pub struct PatternMatrix {
    /// Rows of the matrix. Each row is one arm's pattern list.
    pub(super) rows: Vec<Vec<MetaPattern>>,
    /// Number of columns (variables being scrutinized).
    pub(super) num_cols: usize,
}
impl PatternMatrix {
    /// Create a new empty pattern matrix with the given number of columns.
    pub fn new(num_cols: usize) -> Self {
        PatternMatrix {
            rows: Vec::new(),
            num_cols,
        }
    }
    /// Add a row (arm) to the matrix.
    ///
    /// The row must have exactly `num_cols` patterns.
    pub fn add_row(&mut self, patterns: Vec<MetaPattern>) -> bool {
        if patterns.len() != self.num_cols {
            return false;
        }
        self.rows.push(patterns);
        true
    }
    /// Get the number of rows (arms).
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }
    /// Get the number of columns.
    pub fn num_cols(&self) -> usize {
        self.num_cols
    }
    /// Returns `true` if the matrix is empty.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    /// Get the patterns for a specific column.
    pub fn column(&self, col: usize) -> Vec<&MetaPattern> {
        self.rows.iter().filter_map(|row| row.get(col)).collect()
    }
    /// Check if any row has a catch-all pattern in the given column.
    pub fn has_catchall_in_col(&self, col: usize) -> bool {
        self.rows
            .iter()
            .any(|row| row.get(col).map(|p| p.is_irrefutable()).unwrap_or(false))
    }
    /// Get the distinct constructor names appearing in a given column.
    pub fn constructors_in_col(&self, col: usize) -> Vec<Name> {
        let mut seen = Vec::new();
        for row in &self.rows {
            if let Some(MetaPattern::Constructor(name, _)) = row.get(col) {
                if !seen.contains(name) {
                    seen.push(name.clone());
                }
            }
        }
        seen
    }
    /// Specialize the matrix for a given constructor in the given column.
    ///
    /// Returns a new matrix with the given column specialized to the constructor's
    /// sub-patterns (arity many new columns).
    pub fn specialize(&self, col: usize, ctor: &Name, arity: usize) -> PatternMatrix {
        let new_num_cols = self.num_cols - 1 + arity;
        let mut result = PatternMatrix::new(new_num_cols);
        for row in &self.rows {
            match row.get(col) {
                Some(MetaPattern::Constructor(name, sub_pats)) if name == ctor => {
                    let mut new_row: Vec<MetaPattern> = row[..col].to_vec();
                    new_row.extend(sub_pats.iter().cloned());
                    new_row.extend(row[col + 1..].iter().cloned());
                    result.rows.push(new_row);
                }
                Some(pat) if pat.is_irrefutable() => {
                    let mut new_row: Vec<MetaPattern> = row[..col].to_vec();
                    for _ in 0..arity {
                        new_row.push(MetaPattern::Wildcard);
                    }
                    new_row.extend(row[col + 1..].iter().cloned());
                    result.rows.push(new_row);
                }
                _ => {}
            }
        }
        result
    }
    /// Compute the "default" matrix (rows that match anything in `col`).
    ///
    /// Used to handle the catch-all case during exhaustiveness checking.
    pub fn default_matrix(&self, col: usize) -> PatternMatrix {
        let new_num_cols = self.num_cols - 1;
        let mut result = PatternMatrix::new(new_num_cols);
        for row in &self.rows {
            if let Some(pat) = row.get(col) {
                if pat.is_irrefutable() {
                    let mut new_row: Vec<MetaPattern> = row[..col].to_vec();
                    new_row.extend(row[col + 1..].iter().cloned());
                    result.rows.push(new_row);
                }
            }
        }
        result
    }
}
#[allow(dead_code)]
pub struct MatchExhaustExtDiff2000 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl MatchExhaustExtDiff2000 {
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
/// Description of a missing pattern.
#[derive(Clone, Debug)]
pub struct MissingPattern {
    /// The constructor that is not covered.
    pub ctor_name: Name,
    /// Human-readable description.
    pub description: String,
    /// Sub-patterns that are also missing (for nested constructors).
    pub sub_missing: Vec<MissingPattern>,
}
impl MissingPattern {
    /// Create a simple missing pattern with no sub-patterns.
    pub fn simple(ctor_name: Name, description: String) -> Self {
        MissingPattern {
            ctor_name,
            description,
            sub_missing: Vec::new(),
        }
    }
    /// Returns a full description including sub-patterns.
    pub fn full_description(&self) -> String {
        if self.sub_missing.is_empty() {
            self.description.clone()
        } else {
            let subs: Vec<_> = self
                .sub_missing
                .iter()
                .map(|m| m.full_description())
                .collect();
            format!("{} (sub: {})", self.description, subs.join(", "))
        }
    }
}
/// A work queue for MatchExhaust items.
#[allow(dead_code)]
pub struct MatchExhaustWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl MatchExhaustWorkQueue {
    pub fn new(capacity: usize) -> Self {
        MatchExhaustWorkQueue {
            pending: std::collections::VecDeque::new(),
            processed: Vec::new(),
            capacity,
        }
    }
    pub fn enqueue(&mut self, item: String) -> bool {
        if self.pending.len() >= self.capacity {
            return false;
        }
        self.pending.push_back(item);
        true
    }
    pub fn dequeue(&mut self) -> Option<String> {
        let item = self.pending.pop_front()?;
        self.processed.push(item.clone());
        Some(item)
    }
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    pub fn processed_count(&self) -> usize {
        self.processed.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    pub fn is_full(&self) -> bool {
        self.pending.len() >= self.capacity
    }
    pub fn total_processed(&self) -> usize {
        self.processed.len()
    }
}
#[allow(dead_code)]
pub struct MatchExhaustExtDiag2000 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl MatchExhaustExtDiag2000 {
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
/// Description of a redundant arm.
#[derive(Debug, Clone)]
pub struct RedundantArm {
    /// Index of the redundant arm.
    pub arm_index: usize,
    /// Reason for redundancy.
    pub reason: RedundancyReason,
}
/// A state machine controller for MatchExhaust.
#[allow(dead_code)]
pub struct MatchExhaustStateMachine {
    pub state: MatchExhaustState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl MatchExhaustStateMachine {
    pub fn new() -> Self {
        MatchExhaustStateMachine {
            state: MatchExhaustState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: MatchExhaustState) -> bool {
        if self.state.is_terminal() {
            return false;
        }
        let desc = format!("{:?} -> {:?}", self.state, new_state);
        self.state = new_state;
        self.transitions += 1;
        self.history.push(desc);
        true
    }
    pub fn start(&mut self) -> bool {
        self.transition_to(MatchExhaustState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(MatchExhaustState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(MatchExhaustState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(MatchExhaustState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
#[allow(dead_code)]
pub struct MatchExhaustExtPass2000 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<MatchExhaustExtResult2000>,
}
impl MatchExhaustExtPass2000 {
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
    pub fn run(&mut self, input: &str) -> MatchExhaustExtResult2000 {
        if !self.enabled {
            return MatchExhaustExtResult2000::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            MatchExhaustExtResult2000::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            MatchExhaustExtResult2000::Ok(format!(
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
