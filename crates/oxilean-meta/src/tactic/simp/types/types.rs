//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::discr_tree::DiscrTree;
use oxilean_kernel::{Expr, Name};

#[allow(dead_code)]
pub struct TypesExtConfig3200 {
    pub(super) values: std::collections::HashMap<String, TypesExtConfigVal3200>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl TypesExtConfig3200 {
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
    pub fn set(&mut self, key: &str, value: TypesExtConfigVal3200) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&TypesExtConfigVal3200> {
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
        self.set(key, TypesExtConfigVal3200::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TypesExtConfigVal3200::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TypesExtConfigVal3200::Str(v.to_string()))
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
/// A result type for TacticSimpTypes analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticSimpTypesResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticSimpTypesResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticSimpTypesResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticSimpTypesResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticSimpTypesResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticSimpTypesResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticSimpTypesResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticSimpTypesResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticSimpTypesResult::Ok(_) => 1.0,
            TacticSimpTypesResult::Err(_) => 0.0,
            TacticSimpTypesResult::Skipped => 0.0,
            TacticSimpTypesResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A simple ordered index mapping names to integer IDs.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct NameIndex {
    pub(super) names: Vec<String>,
    pub(super) index: std::collections::HashMap<String, usize>,
}
impl NameIndex {
    /// Create a new empty name index.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a name, returning its ID.
    /// If the name is already present, returns the existing ID.
    #[allow(dead_code)]
    pub fn insert(&mut self, name: impl Into<String>) -> usize {
        let name = name.into();
        if let Some(&id) = self.index.get(&name) {
            return id;
        }
        let id = self.names.len();
        self.index.insert(name.clone(), id);
        self.names.push(name);
        id
    }
    /// Get the ID for a name, if it exists.
    #[allow(dead_code)]
    pub fn get_id(&self, name: &str) -> Option<usize> {
        self.index.get(name).copied()
    }
    /// Get the name for an ID.
    #[allow(dead_code)]
    pub fn get_name(&self, id: usize) -> Option<&str> {
        self.names.get(id).map(|s| s.as_str())
    }
    /// Get the number of names in the index.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.names.len()
    }
    /// Check if the index is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
    /// Get all names in insertion order.
    #[allow(dead_code)]
    pub fn all_names(&self) -> &[String] {
        &self.names
    }
}
/// A diagnostic reporter for TacticSimpTypes.
#[allow(dead_code)]
pub struct TacticSimpTypesDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticSimpTypesDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticSimpTypesDiagnostics {
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
pub struct TypesExtDiag3200 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl TypesExtDiag3200 {
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
/// A configuration store for TacticSimpTypes.
#[allow(dead_code)]
pub struct TacticSimpTypesConfig {
    pub values: std::collections::HashMap<String, TacticSimpTypesConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticSimpTypesConfig {
    pub fn new() -> Self {
        TacticSimpTypesConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticSimpTypesConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticSimpTypesConfigValue> {
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
        self.set(key, TacticSimpTypesConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticSimpTypesConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticSimpTypesConfigValue::Str(v.to_string()))
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
/// A work queue for SimpTypes items.
#[allow(dead_code)]
pub struct SimpTypesWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl SimpTypesWorkQueue {
    pub fn new(capacity: usize) -> Self {
        SimpTypesWorkQueue {
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
/// A set of simp lemmas grouped by source.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct SimpLemmaSet {
    pub(super) inner: SimpTheorems,
    pub(super) label: String,
}
impl SimpLemmaSet {
    /// Create a new named lemma set.
    #[allow(dead_code)]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            inner: SimpTheorems::new(),
            label: label.into(),
        }
    }
    /// Get the label.
    #[allow(dead_code)]
    pub fn label(&self) -> &str {
        &self.label
    }
    /// Add a lemma to this set.
    #[allow(dead_code)]
    pub fn add(&mut self, lemma: SimpLemma) {
        self.inner.add_lemma(lemma);
    }
    /// Get the number of lemmas.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.inner.num_lemmas()
    }
    /// Check if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.inner.num_lemmas() == 0
    }
    /// Get all lemmas.
    #[allow(dead_code)]
    pub fn lemmas(&self) -> &[SimpLemma] {
        self.inner.all_lemmas()
    }
    /// Find matching lemmas for an expression.
    #[allow(dead_code)]
    pub fn find_lemmas(&self, expr: &Expr) -> Vec<&SimpLemma> {
        self.inner.find_lemmas(expr)
    }
    /// Merge another set's lemmas into this one.
    #[allow(dead_code)]
    pub fn merge_from(&mut self, other: &SimpLemmaSet) {
        self.inner.merge(&other.inner);
    }
}
/// An extended utility type for SimpTypes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SimpTypesExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl SimpTypesExt {
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
/// Simp trace entry - records what happened during simplification.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SimpTraceEntry {
    /// The expression before rewriting.
    pub before: Expr,
    /// The expression after rewriting.
    pub after: Expr,
    /// The lemma that was applied (or None for built-in reductions).
    pub lemma_used: Option<Name>,
    /// Number of steps taken to simplify.
    pub steps: u32,
}
impl SimpTraceEntry {
    /// Create a new trace entry.
    #[allow(dead_code)]
    pub fn new(before: Expr, after: Expr, lemma_used: Option<Name>, steps: u32) -> Self {
        Self {
            before,
            after,
            lemma_used,
            steps,
        }
    }
    /// Check if the expression actually changed.
    #[allow(dead_code)]
    pub fn did_change(&self) -> bool {
        self.before != self.after
    }
}
/// A state machine for SimpTypes.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SimpTypesState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl SimpTypesState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, SimpTypesState::Complete | SimpTypesState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, SimpTypesState::Initial | SimpTypesState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, SimpTypesState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            SimpTypesState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// A diff for TacticSimpTypes analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticSimpTypesDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticSimpTypesDiff {
    pub fn new() -> Self {
        TacticSimpTypesDiff {
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
/// A typed slot for TacticSimpTypes configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticSimpTypesConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticSimpTypesConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticSimpTypesConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticSimpTypesConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticSimpTypesConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticSimpTypesConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticSimpTypesConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticSimpTypesConfigValue::Bool(_) => "bool",
            TacticSimpTypesConfigValue::Int(_) => "int",
            TacticSimpTypesConfigValue::Float(_) => "float",
            TacticSimpTypesConfigValue::Str(_) => "str",
            TacticSimpTypesConfigValue::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TypesExtResult3200 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl TypesExtResult3200 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, TypesExtResult3200::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, TypesExtResult3200::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, TypesExtResult3200::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, TypesExtResult3200::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let TypesExtResult3200::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let TypesExtResult3200::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            TypesExtResult3200::Ok(_) => 1.0,
            TypesExtResult3200::Err(_) => 0.0,
            TypesExtResult3200::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            TypesExtResult3200::Skipped => 0.5,
        }
    }
}
/// Result of applying simp to an expression.
#[derive(Clone, Debug)]
pub enum SimpResult {
    /// Expression was simplified.
    Simplified {
        /// The simplified expression.
        new_expr: Expr,
        /// Proof that old = new.
        proof: Option<Expr>,
    },
    /// Expression was not changed.
    Unchanged,
    /// Simp proved the goal (expression simplified to `True`).
    Proved(Expr),
}
impl SimpResult {
    /// Check if any simplification occurred.
    pub fn is_simplified(&self) -> bool {
        !matches!(self, SimpResult::Unchanged)
    }
    /// Check if the goal was proved.
    pub fn is_proved(&self) -> bool {
        matches!(self, SimpResult::Proved(_))
    }
    /// Get the new expression, if simplified.
    pub fn new_expr(&self) -> Option<&Expr> {
        match self {
            SimpResult::Simplified { new_expr, .. } => Some(new_expr),
            _ => None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TypesExtConfigVal3200 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl TypesExtConfigVal3200 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let TypesExtConfigVal3200::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let TypesExtConfigVal3200::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let TypesExtConfigVal3200::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let TypesExtConfigVal3200::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let TypesExtConfigVal3200::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            TypesExtConfigVal3200::Bool(_) => "bool",
            TypesExtConfigVal3200::Int(_) => "int",
            TypesExtConfigVal3200::Float(_) => "float",
            TypesExtConfigVal3200::Str(_) => "str",
            TypesExtConfigVal3200::List(_) => "list",
        }
    }
}
/// Priority levels for simp lemmas.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpPriority(pub u32);
impl SimpPriority {
    /// Default priority for user-defined lemmas.
    pub const DEFAULT: SimpPriority = SimpPriority(1000);
    /// High priority (tried before defaults).
    pub const HIGH: SimpPriority = SimpPriority(100);
    /// Low priority (tried after defaults).
    pub const LOW: SimpPriority = SimpPriority(10000);
    /// Create a custom priority value.
    #[allow(dead_code)]
    pub fn custom(n: u32) -> Self {
        SimpPriority(n)
    }
}
/// Direction of a simp rewrite.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SimpDirection {
    /// Rewrite left-to-right (default): `lhs -> rhs`.
    #[default]
    Forward,
    /// Rewrite right-to-left: `rhs -> lhs`.
    Backward,
}
impl SimpDirection {
    /// Check if this is a forward (left-to-right) rewrite.
    #[allow(dead_code)]
    pub fn is_forward(self) -> bool {
        matches!(self, SimpDirection::Forward)
    }
    /// Return the opposite direction.
    #[allow(dead_code)]
    pub fn flip(self) -> Self {
        match self {
            SimpDirection::Forward => SimpDirection::Backward,
            SimpDirection::Backward => SimpDirection::Forward,
        }
    }
}
/// A counter map for SimpTypes frequency analysis.
#[allow(dead_code)]
pub struct SimpTypesCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl SimpTypesCounterMap {
    pub fn new() -> Self {
        SimpTypesCounterMap {
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
/// An analysis pass for TacticSimpTypes.
#[allow(dead_code)]
pub struct TacticSimpTypesAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticSimpTypesResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticSimpTypesAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticSimpTypesAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticSimpTypesResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticSimpTypesResult::Err("empty input".to_string())
        } else {
            TacticSimpTypesResult::Ok(format!("processed: {}", input))
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
/// A complete simp trace for debugging.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct SimpTrace {
    pub(super) entries: Vec<SimpTraceEntry>,
    pub(super) total_steps: u32,
}
impl SimpTrace {
    /// Create a new empty trace.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a trace entry.
    #[allow(dead_code)]
    pub fn push(&mut self, entry: SimpTraceEntry) {
        self.total_steps += entry.steps;
        self.entries.push(entry);
    }
    /// Get all trace entries.
    #[allow(dead_code)]
    pub fn entries(&self) -> &[SimpTraceEntry] {
        &self.entries
    }
    /// Get total number of simplification steps.
    #[allow(dead_code)]
    pub fn total_steps(&self) -> u32 {
        self.total_steps
    }
    /// Get the number of rewrites that actually changed the expression.
    #[allow(dead_code)]
    pub fn num_changes(&self) -> usize {
        self.entries.iter().filter(|e| e.did_change()).count()
    }
    /// Get the lemmas used (in order).
    #[allow(dead_code)]
    pub fn lemmas_used(&self) -> Vec<&Name> {
        self.entries
            .iter()
            .filter_map(|e| e.lemma_used.as_ref())
            .collect()
    }
}
/// Configuration for the `simp` tactic.
#[derive(Clone, Debug)]
pub struct SimpConfig {
    /// Maximum number of rewrite steps.
    pub max_steps: u32,
    /// Whether to use congruence lemmas.
    pub use_congr: bool,
    /// Whether to apply beta reduction.
    pub beta: bool,
    /// Whether to apply eta reduction.
    pub eta: bool,
    /// Whether to apply iota reduction (match/recursor).
    pub iota: bool,
    /// Whether to unfold definitions.
    pub zeta: bool,
    /// Whether to try `rfl` to close goals.
    pub try_rfl: bool,
    /// Whether to use the full simp lemma set or `simp only`.
    pub use_default_lemmas: bool,
    /// Whether to simplify hypotheses too (`simp_all`).
    pub simp_hyps: bool,
    /// Whether to discharge conditional rewrites.
    pub discharge_conditions: bool,
}
impl SimpConfig {
    /// Create a config for `simp only`.
    pub fn only() -> Self {
        Self {
            use_default_lemmas: false,
            ..Self::default()
        }
    }
}
/// A sliding window accumulator for SimpTypes.
#[allow(dead_code)]
pub struct SimpTypesWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl SimpTypesWindow {
    pub fn new(capacity: usize) -> Self {
        SimpTypesWindow {
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
/// An extended map for SimpTypes keys to values.
#[allow(dead_code)]
pub struct SimpTypesExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> SimpTypesExtMap<V> {
    pub fn new() -> Self {
        SimpTypesExtMap {
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
/// A pipeline of TacticSimpTypes analysis passes.
#[allow(dead_code)]
pub struct TacticSimpTypesPipeline {
    pub passes: Vec<TacticSimpTypesAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticSimpTypesPipeline {
    pub fn new(name: &str) -> Self {
        TacticSimpTypesPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticSimpTypesAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticSimpTypesResult> {
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
/// A builder pattern for SimpTypes.
#[allow(dead_code)]
pub struct SimpTypesBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl SimpTypesBuilder {
    pub fn new(name: &str) -> Self {
        SimpTypesBuilder {
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
pub struct SimpTypesExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl SimpTypesExtUtil {
    pub fn new(key: &str) -> Self {
        SimpTypesExtUtil {
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
/// Simple trie for efficient string prefix lookup.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct StringTrie {
    pub(super) children: std::collections::HashMap<char, StringTrie>,
    pub(super) is_end: bool,
    pub(super) value: Option<String>,
}
impl StringTrie {
    /// Create a new empty trie.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a string into the trie.
    #[allow(dead_code)]
    pub fn insert(&mut self, s: &str) {
        let mut node = self;
        for c in s.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_end = true;
        node.value = Some(s.to_string());
    }
    /// Check if a string is in the trie.
    #[allow(dead_code)]
    pub fn contains(&self, s: &str) -> bool {
        let mut node = self;
        for c in s.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return false,
            }
        }
        node.is_end
    }
    /// Find all strings with a given prefix.
    #[allow(dead_code)]
    pub fn starts_with(&self, prefix: &str) -> Vec<String> {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(next) => node = next,
                None => return vec![],
            }
        }
        let mut results = Vec::new();
        collect_strings(node, &mut results);
        results
    }
    /// Get the number of strings in the trie.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        let mut count = if self.is_end { 1 } else { 0 };
        for child in self.children.values() {
            count += child.len();
        }
        count
    }
    /// Check if the trie is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// A database of simp lemmas indexed by discrimination tree.
#[derive(Clone, Debug)]
pub struct SimpTheorems {
    /// Lemmas indexed by their LHS pattern.
    pub(super) lemma_index: DiscrTree<SimpLemma>,
    /// All lemmas in registration order.
    pub(super) lemmas: Vec<SimpLemma>,
    /// Lemmas to always exclude.
    pub(super) excluded: Vec<Name>,
}
impl SimpTheorems {
    /// Create an empty simp lemma database.
    pub fn new() -> Self {
        Self {
            lemma_index: DiscrTree::new(),
            lemmas: Vec::new(),
            excluded: Vec::new(),
        }
    }
    /// Add a simp lemma.
    pub fn add_lemma(&mut self, lemma: SimpLemma) {
        self.lemma_index.insert(&lemma.lhs, lemma.clone());
        self.lemmas.push(lemma);
    }
    /// Remove a simp lemma by name.
    pub fn remove_lemma(&mut self, name: &Name) {
        self.lemmas.retain(|l| &l.name != name);
        self.excluded.push(name.clone());
    }
    /// Find matching simp lemmas for a given expression.
    pub fn find_lemmas(&self, expr: &Expr) -> Vec<&SimpLemma> {
        let candidates = self.lemma_index.find(expr);
        candidates
            .into_iter()
            .filter(|l| !self.excluded.contains(&l.name))
            .collect()
    }
    /// Get the number of registered lemmas.
    pub fn num_lemmas(&self) -> usize {
        self.lemmas.len()
    }
    /// Get all lemmas.
    pub fn all_lemmas(&self) -> &[SimpLemma] {
        &self.lemmas
    }
    /// Merge another set of theorems into this one.
    pub fn merge(&mut self, other: &SimpTheorems) {
        for lemma in &other.lemmas {
            self.add_lemma(lemma.clone());
        }
    }
}
#[allow(dead_code)]
pub struct TypesExtDiff3200 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl TypesExtDiff3200 {
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
pub struct TypesExtPass3200 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<TypesExtResult3200>,
}
impl TypesExtPass3200 {
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
    pub fn run(&mut self, input: &str) -> TypesExtResult3200 {
        if !self.enabled {
            return TypesExtResult3200::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            TypesExtResult3200::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            TypesExtResult3200::Ok(format!(
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
/// A single simplification lemma.
#[derive(Clone, Debug)]
pub struct SimpLemma {
    /// Name of the lemma declaration.
    pub name: Name,
    /// Left-hand side of the rewrite (after matching).
    pub lhs: Expr,
    /// Right-hand side of the rewrite.
    pub rhs: Expr,
    /// Proof term.
    pub proof: Expr,
    /// Priority (lower = tried first).
    pub priority: u32,
    /// Whether this is a conditional rewrite (has hypotheses).
    pub is_conditional: bool,
    /// Whether rewriting is left-to-right.
    pub is_forward: bool,
}
/// A registry of named simp lemma sets.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct SimpRegistry {
    pub(super) sets: Vec<SimpLemmaSet>,
    pub(super) default_set: SimpTheorems,
}
impl SimpRegistry {
    /// Create an empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            sets: Vec::new(),
            default_set: SimpTheorems::new(),
        }
    }
    /// Register a named lemma set.
    #[allow(dead_code)]
    pub fn register_set(&mut self, set: SimpLemmaSet) {
        self.sets.push(set);
    }
    /// Add a lemma to the global default set.
    #[allow(dead_code)]
    pub fn add_default(&mut self, lemma: SimpLemma) {
        self.default_set.add_lemma(lemma);
    }
    /// Find a set by label.
    #[allow(dead_code)]
    pub fn find_set(&self, label: &str) -> Option<&SimpLemmaSet> {
        self.sets.iter().find(|s| s.label() == label)
    }
    /// Get all lemmas from all sets plus the default set.
    #[allow(dead_code)]
    pub fn all_lemmas(&self) -> Vec<&SimpLemma> {
        let mut result: Vec<&SimpLemma> = self.default_set.all_lemmas().iter().collect();
        for set in &self.sets {
            result.extend(set.lemmas());
        }
        result
    }
    /// Get the number of registered sets.
    #[allow(dead_code)]
    pub fn num_sets(&self) -> usize {
        self.sets.len()
    }
    /// Get total number of lemmas across all sets.
    #[allow(dead_code)]
    pub fn total_lemmas(&self) -> usize {
        self.sets.iter().map(|s| s.len()).sum::<usize>() + self.default_set.num_lemmas()
    }
}
#[allow(dead_code)]
pub struct TypesExtPipeline3200 {
    pub name: String,
    pub passes: Vec<TypesExtPass3200>,
    pub run_count: usize,
}
impl TypesExtPipeline3200 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: TypesExtPass3200) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<TypesExtResult3200> {
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
/// A state machine controller for SimpTypes.
#[allow(dead_code)]
pub struct SimpTypesStateMachine {
    pub state: SimpTypesState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl SimpTypesStateMachine {
    pub fn new() -> Self {
        SimpTypesStateMachine {
            state: SimpTypesState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: SimpTypesState) -> bool {
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
        self.transition_to(SimpTypesState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(SimpTypesState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(SimpTypesState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(SimpTypesState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
