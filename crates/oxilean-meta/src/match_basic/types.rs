//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{Expr, Literal, Name};

/// A typed slot for MatchBasic configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MatchBasicConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl MatchBasicConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            MatchBasicConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            MatchBasicConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            MatchBasicConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            MatchBasicConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            MatchBasicConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            MatchBasicConfigValue::Bool(_) => "bool",
            MatchBasicConfigValue::Int(_) => "int",
            MatchBasicConfigValue::Float(_) => "float",
            MatchBasicConfigValue::Str(_) => "str",
            MatchBasicConfigValue::List(_) => "list",
        }
    }
}
/// A utility type for MatchBasic (index 12).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil12 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil12 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil12 {
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
pub struct MatchBasicExtPass4100 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<MatchBasicExtResult4100>,
}
impl MatchBasicExtPass4100 {
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
    pub fn run(&mut self, input: &str) -> MatchBasicExtResult4100 {
        if !self.enabled {
            return MatchBasicExtResult4100::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            MatchBasicExtResult4100::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            MatchBasicExtResult4100::Ok(format!(
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum MatchBasicExtConfigVal4100 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl MatchBasicExtConfigVal4100 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let MatchBasicExtConfigVal4100::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let MatchBasicExtConfigVal4100::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let MatchBasicExtConfigVal4100::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let MatchBasicExtConfigVal4100::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let MatchBasicExtConfigVal4100::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            MatchBasicExtConfigVal4100::Bool(_) => "bool",
            MatchBasicExtConfigVal4100::Int(_) => "int",
            MatchBasicExtConfigVal4100::Float(_) => "float",
            MatchBasicExtConfigVal4100::Str(_) => "str",
            MatchBasicExtConfigVal4100::List(_) => "list",
        }
    }
}
/// A diagnostic reporter for MatchBasic.
#[allow(dead_code)]
pub struct MatchBasicDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl MatchBasicDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        MatchBasicDiagnostics {
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
/// A configuration store for MatchBasic.
#[allow(dead_code)]
pub struct MatchBasicConfig {
    pub values: std::collections::HashMap<String, MatchBasicConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl MatchBasicConfig {
    pub fn new() -> Self {
        MatchBasicConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: MatchBasicConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&MatchBasicConfigValue> {
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
        self.set(key, MatchBasicConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, MatchBasicConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, MatchBasicConfigValue::Str(v.to_string()))
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
/// A utility type for MatchBasic (index 3).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil3 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil3 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil3 {
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
/// A pattern in a match expression.
#[derive(Clone, Debug, PartialEq)]
pub enum MetaPattern {
    /// Wildcard pattern: matches anything, binds nothing.
    Wildcard,
    /// Variable pattern: matches anything, binds the value.
    Var(Name),
    /// Constructor pattern: matches if head is this constructor.
    Constructor(Name, Vec<MetaPattern>),
    /// Literal pattern: matches exact literal value.
    Literal(Literal),
    /// As-pattern: `p as x` matches p and also binds x.
    As(Box<MetaPattern>, Name),
    /// Or-pattern: matches if either sub-pattern matches.
    Or(Box<MetaPattern>, Box<MetaPattern>),
    /// Inaccessible pattern (dot pattern): `.e`.
    Inaccessible(Expr),
}
impl MetaPattern {
    /// Check if this pattern is a wildcard or variable.
    pub fn is_irrefutable(&self) -> bool {
        matches!(self, MetaPattern::Wildcard | MetaPattern::Var(_))
    }
    /// Check if this pattern is a constructor pattern.
    pub fn is_constructor(&self) -> bool {
        matches!(self, MetaPattern::Constructor(_, _))
    }
    /// Check if this pattern is a literal.
    pub fn is_literal(&self) -> bool {
        matches!(self, MetaPattern::Literal(_))
    }
    /// Get the constructor name (if this is a constructor pattern).
    pub fn ctor_name(&self) -> Option<&Name> {
        match self {
            MetaPattern::Constructor(name, _) => Some(name),
            _ => None,
        }
    }
    /// Get the subpatterns (for constructor patterns).
    pub fn subpatterns(&self) -> &[MetaPattern] {
        match self {
            MetaPattern::Constructor(_, pats) => pats,
            _ => &[],
        }
    }
    /// Count the number of variables bound by this pattern.
    pub fn num_bindings(&self) -> usize {
        match self {
            MetaPattern::Wildcard | MetaPattern::Inaccessible(_) => 0,
            MetaPattern::Var(_) => 1,
            MetaPattern::Literal(_) => 0,
            MetaPattern::Constructor(_, pats) => pats.iter().map(|p| p.num_bindings()).sum(),
            MetaPattern::As(inner, _) => 1 + inner.num_bindings(),
            MetaPattern::Or(left, _right) => left.num_bindings(),
        }
    }
    /// Collect all variable names bound by this pattern.
    pub fn bound_vars(&self) -> Vec<Name> {
        let mut vars = Vec::new();
        self.collect_vars(&mut vars);
        vars
    }
    fn collect_vars(&self, vars: &mut Vec<Name>) {
        match self {
            MetaPattern::Var(name) => vars.push(name.clone()),
            MetaPattern::Constructor(_, pats) => {
                for p in pats {
                    p.collect_vars(vars);
                }
            }
            MetaPattern::As(inner, name) => {
                vars.push(name.clone());
                inner.collect_vars(vars);
            }
            MetaPattern::Or(left, _) => {
                left.collect_vars(vars);
            }
            _ => {}
        }
    }
    /// Get the depth of nesting.
    pub fn depth(&self) -> usize {
        match self {
            MetaPattern::Constructor(_, pats) => {
                1 + pats.iter().map(|p| p.depth()).max().unwrap_or(0)
            }
            MetaPattern::As(inner, _) => 1 + inner.depth(),
            MetaPattern::Or(left, right) => 1 + left.depth().max(right.depth()),
            _ => 0,
        }
    }
}
/// A pattern matrix: a list of rows.
#[derive(Clone, Debug, Default)]
pub struct PatternMatrix {
    /// Rows, one per match arm.
    pub rows: Vec<PatternRow>,
    /// Number of discriminants (columns).
    pub num_discriminants: usize,
}
impl PatternMatrix {
    /// Create an empty matrix.
    pub fn new(num_discriminants: usize) -> Self {
        Self {
            rows: Vec::new(),
            num_discriminants,
        }
    }
    /// Add a row.
    pub fn add_row(&mut self, row: PatternRow) {
        self.rows.push(row);
    }
    /// Return the number of rows (arms).
    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }
    /// Check if the matrix is empty.
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    /// Check if the first row is all irrefutable (catch-all).
    pub fn first_row_is_catchall(&self) -> bool {
        self.rows
            .first()
            .map(|r| r.is_all_irrefutable())
            .unwrap_or(false)
    }
    /// Specialize the matrix for a constructor.
    ///
    /// Keep only rows where the first column matches `ctor_name` (or is irrefutable).
    /// Expand constructor sub-patterns into the row.
    pub fn specialize(&self, col: usize, ctor_name: &Name, arity: usize) -> PatternMatrix {
        let mut result = PatternMatrix::new(self.num_discriminants - 1 + arity);
        for row in &self.rows {
            match &row.patterns[col] {
                MetaPattern::Constructor(name, sub) if name == ctor_name => {
                    let mut new_pats = row.patterns.clone();
                    new_pats.remove(col);
                    for (i, s) in sub.iter().enumerate().rev() {
                        new_pats.insert(col + arity - 1 - i, s.clone());
                    }
                    result.add_row(PatternRow {
                        patterns: new_pats,
                        ..row.clone()
                    });
                }
                MetaPattern::Wildcard | MetaPattern::Var(_) => {
                    let mut new_pats = row.patterns.clone();
                    new_pats.remove(col);
                    for _ in 0..arity {
                        new_pats.insert(col, MetaPattern::Wildcard);
                    }
                    result.add_row(PatternRow {
                        patterns: new_pats,
                        ..row.clone()
                    });
                }
                _ => {}
            }
        }
        result
    }
    /// Default matrix: keep only rows where column `col` is irrefutable.
    pub fn default_matrix(&self, col: usize) -> PatternMatrix {
        let mut result = PatternMatrix::new(self.num_discriminants - 1);
        for row in &self.rows {
            if row.patterns[col].is_irrefutable() {
                let mut new_pats = row.patterns.clone();
                new_pats.remove(col);
                result.add_row(PatternRow {
                    patterns: new_pats,
                    ..row.clone()
                });
            }
        }
        result
    }
}
/// A pipeline of MatchBasic analysis passes.
#[allow(dead_code)]
pub struct MatchBasicPipeline {
    pub passes: Vec<MatchBasicAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl MatchBasicPipeline {
    pub fn new(name: &str) -> Self {
        MatchBasicPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: MatchBasicAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<MatchBasicResult> {
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
/// The result of trying to match a pattern against an expression.
#[derive(Debug, Clone, PartialEq)]
pub enum MatchResult {
    /// The match definitely succeeds; bindings maps variable names to expressions.
    Success(Vec<(oxilean_kernel::Name, Expr)>),
    /// The match definitely fails.
    Failure,
    /// The match is undetermined (e.g., scrutinee not in WHNF yet).
    Undetermined,
}
impl MatchResult {
    /// Check if the match succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, MatchResult::Success(_))
    }
    /// Check if the match failed.
    pub fn is_failure(&self) -> bool {
        matches!(self, MatchResult::Failure)
    }
    /// Get the bindings produced by a successful match.
    pub fn bindings(&self) -> &[(oxilean_kernel::Name, Expr)] {
        match self {
            MatchResult::Success(b) => b,
            _ => &[],
        }
    }
}
#[allow(dead_code)]
pub struct MatchBasicExtDiff4100 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl MatchBasicExtDiff4100 {
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
/// A utility type for MatchBasic (index 9).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil9 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil9 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil9 {
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
/// A priority queue for MatchBasic items.
#[allow(dead_code)]
pub struct MatchBasicPriorityQueue {
    pub items: Vec<(MatchBasicUtil0, i64)>,
}
#[allow(dead_code)]
impl MatchBasicPriorityQueue {
    pub fn new() -> Self {
        MatchBasicPriorityQueue { items: Vec::new() }
    }
    pub fn push(&mut self, item: MatchBasicUtil0, priority: i64) {
        self.items.push((item, priority));
        self.items.sort_by_key(|(_, p)| -p);
    }
    pub fn pop(&mut self) -> Option<(MatchBasicUtil0, i64)> {
        if self.items.is_empty() {
            None
        } else {
            Some(self.items.remove(0))
        }
    }
    pub fn peek(&self) -> Option<&(MatchBasicUtil0, i64)> {
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
#[derive(Debug, Clone)]
pub enum MatchBasicExtResult4100 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl MatchBasicExtResult4100 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, MatchBasicExtResult4100::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, MatchBasicExtResult4100::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, MatchBasicExtResult4100::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, MatchBasicExtResult4100::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let MatchBasicExtResult4100::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let MatchBasicExtResult4100::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            MatchBasicExtResult4100::Ok(_) => 1.0,
            MatchBasicExtResult4100::Err(_) => 0.0,
            MatchBasicExtResult4100::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            MatchBasicExtResult4100::Skipped => 0.5,
        }
    }
}
#[allow(dead_code)]
pub struct MatchBasicExtDiag4100 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl MatchBasicExtDiag4100 {
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
/// A row in a pattern matrix.
#[derive(Clone, Debug)]
pub struct PatternRow {
    /// The patterns in this row (one per discriminant).
    pub patterns: Vec<MetaPattern>,
    /// Index of the match arm this row corresponds to.
    pub arm_index: usize,
    /// Optional guard.
    pub guard: Option<Expr>,
}
impl PatternRow {
    /// Create a new pattern row.
    pub fn new(patterns: Vec<MetaPattern>, arm_index: usize) -> Self {
        Self {
            patterns,
            arm_index,
            guard: None,
        }
    }
    /// Check if all patterns in this row are irrefutable.
    pub fn is_all_irrefutable(&self) -> bool {
        self.patterns.iter().all(|p| p.is_irrefutable())
    }
    /// Return the first non-wildcard pattern (if any).
    pub fn first_refutable(&self) -> Option<(usize, &MetaPattern)> {
        self.patterns
            .iter()
            .enumerate()
            .find(|(_, p)| !p.is_irrefutable())
    }
}
/// A utility type for MatchBasic (index 7).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil7 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil7 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil7 {
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
/// A utility type for MatchBasic (index 4).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil4 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil4 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil4 {
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
/// A simple cache for MatchBasic computations.
#[allow(dead_code)]
pub struct MatchBasicCache {
    pub data: std::collections::HashMap<String, i64>,
    pub hits: usize,
    pub misses: usize,
}
#[allow(dead_code)]
impl MatchBasicCache {
    pub fn new() -> Self {
        MatchBasicCache {
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
/// A diff for MatchBasic analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MatchBasicDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicDiff {
    pub fn new() -> Self {
        MatchBasicDiff {
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
/// A registry for MatchBasic utilities.
#[allow(dead_code)]
pub struct MatchBasicRegistry {
    pub entries: Vec<MatchBasicUtil0>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl MatchBasicRegistry {
    pub fn new(capacity: usize) -> Self {
        MatchBasicRegistry {
            entries: Vec::new(),
            capacity,
        }
    }
    pub fn register(&mut self, entry: MatchBasicUtil0) -> bool {
        if self.entries.len() >= self.capacity {
            return false;
        }
        self.entries.push(entry);
        true
    }
    pub fn lookup(&self, id: usize) -> Option<&MatchBasicUtil0> {
        self.entries.iter().find(|e| e.id == id)
    }
    pub fn remove(&mut self, id: usize) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| e.id != id);
        self.entries.len() < before
    }
    pub fn active_entries(&self) -> Vec<&MatchBasicUtil0> {
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
/// A utility type for MatchBasic (index 0).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil0 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil0 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil0 {
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
/// A utility type for MatchBasic (index 6).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil6 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil6 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil6 {
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
/// A utility type for MatchBasic (index 11).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil11 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil11 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil11 {
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
/// A result type for MatchBasic analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MatchBasicResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl MatchBasicResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, MatchBasicResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, MatchBasicResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, MatchBasicResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, MatchBasicResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            MatchBasicResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            MatchBasicResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            MatchBasicResult::Ok(_) => 1.0,
            MatchBasicResult::Err(_) => 0.0,
            MatchBasicResult::Skipped => 0.0,
            MatchBasicResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A utility type for MatchBasic (index 10).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil10 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil10 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil10 {
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
/// A match expression being elaborated.
#[derive(Clone, Debug)]
pub struct MetaMatchExpr {
    /// Discriminant expressions.
    pub discriminants: Vec<Expr>,
    /// Types of discriminants.
    pub discr_types: Vec<Expr>,
    /// Match arms.
    pub arms: Vec<MetaMatchArm>,
    /// Expected result type.
    pub expected_type: Option<Expr>,
}
impl MetaMatchExpr {
    /// Create a new match expression.
    pub fn new(discriminants: Vec<Expr>, discr_types: Vec<Expr>) -> Self {
        Self {
            discriminants,
            discr_types,
            arms: Vec::new(),
            expected_type: None,
        }
    }
    /// Add a match arm.
    pub fn add_arm(&mut self, arm: MetaMatchArm) {
        self.arms.push(arm);
    }
    /// Set the expected result type.
    pub fn set_expected_type(&mut self, ty: Expr) {
        self.expected_type = Some(ty);
    }
    /// Get the number of discriminants.
    pub fn num_discriminants(&self) -> usize {
        self.discriminants.len()
    }
    /// Get the number of arms.
    pub fn num_arms(&self) -> usize {
        self.arms.len()
    }
    /// Check if all arms have the correct number of patterns.
    pub fn validate_patterns(&self) -> Result<(), String> {
        let expected = self.num_discriminants();
        for (i, arm) in self.arms.iter().enumerate() {
            if arm.patterns.len() != expected {
                return Err(format!(
                    "Arm {} has {} patterns, expected {}",
                    i,
                    arm.patterns.len(),
                    expected
                ));
            }
        }
        Ok(())
    }
}
/// A node in a match decision tree.
///
/// Decision trees are a compiled form of pattern matching that avoid
/// repeatedly testing the same sub-expression.
#[derive(Debug, Clone)]
pub enum DecisionTree {
    /// Leaf: all patterns exhausted; execute this arm index.
    Leaf(usize),
    /// Failure: no arm matches.
    Fail,
    /// Switch on the constructor of a sub-expression.
    Switch {
        /// Index of the discriminant to examine (in the match clause).
        discr_idx: usize,
        /// Cases keyed by constructor name.
        cases: Vec<(oxilean_kernel::Name, Box<DecisionTree>)>,
        /// Default case (wildcard/variable patterns).
        default: Option<Box<DecisionTree>>,
    },
    /// Guard: check a boolean expression before proceeding.
    Guard {
        /// The guard expression to evaluate.
        condition: Expr,
        /// Sub-tree for when the guard holds.
        success: Box<DecisionTree>,
        /// Sub-tree for when the guard fails.
        failure: Box<DecisionTree>,
    },
}
impl DecisionTree {
    /// Check if this is a leaf node.
    pub fn is_leaf(&self) -> bool {
        matches!(self, DecisionTree::Leaf(_))
    }
    /// Check if this is a fail node.
    pub fn is_fail(&self) -> bool {
        matches!(self, DecisionTree::Fail)
    }
    /// Count the number of leaf nodes (reachable arms).
    pub fn num_reachable_arms(&self) -> usize {
        match self {
            DecisionTree::Leaf(_) => 1,
            DecisionTree::Fail => 0,
            DecisionTree::Switch { cases, default, .. } => {
                let case_count: usize = cases.iter().map(|(_, sub)| sub.num_reachable_arms()).sum();
                let default_count = default
                    .as_ref()
                    .map(|d| d.num_reachable_arms())
                    .unwrap_or(0);
                case_count + default_count
            }
            DecisionTree::Guard {
                success, failure, ..
            } => success.num_reachable_arms() + failure.num_reachable_arms(),
        }
    }
    /// Compute the maximum depth of this decision tree.
    pub fn depth(&self) -> usize {
        match self {
            DecisionTree::Leaf(_) | DecisionTree::Fail => 0,
            DecisionTree::Switch { cases, default, .. } => {
                let case_depth = cases.iter().map(|(_, sub)| sub.depth()).max().unwrap_or(0);
                let default_depth = default.as_ref().map(|d| d.depth()).unwrap_or(0);
                1 + case_depth.max(default_depth)
            }
            DecisionTree::Guard {
                success, failure, ..
            } => 1 + success.depth().max(failure.depth()),
        }
    }
}
/// A utility type for MatchBasic (index 13).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil13 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil13 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil13 {
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
/// An analysis pass for MatchBasic.
#[allow(dead_code)]
pub struct MatchBasicAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<MatchBasicResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl MatchBasicAnalysisPass {
    pub fn new(name: &str) -> Self {
        MatchBasicAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> MatchBasicResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            MatchBasicResult::Err("empty input".to_string())
        } else {
            MatchBasicResult::Ok(format!("processed: {}", input))
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
/// A match arm (clause) in a match expression.
#[derive(Clone, Debug)]
pub struct MetaMatchArm {
    /// Patterns for each discriminant.
    pub patterns: Vec<MetaPattern>,
    /// Optional guard expression.
    pub guard: Option<Expr>,
    /// Right-hand side (body).
    pub rhs: Expr,
}
/// A utility type for MatchBasic (index 1).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil1 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil1 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil1 {
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
/// Statistics for MatchBasic operations.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicStats {
    pub total_ops: usize,
    pub successful_ops: usize,
    pub failed_ops: usize,
    pub total_time_ns: u64,
    pub max_time_ns: u64,
}
#[allow(dead_code)]
impl MatchBasicStats {
    pub fn new() -> Self {
        MatchBasicStats::default()
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
#[allow(dead_code)]
pub struct MatchBasicExtConfig4100 {
    pub(super) values: std::collections::HashMap<String, MatchBasicExtConfigVal4100>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl MatchBasicExtConfig4100 {
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
    pub fn set(&mut self, key: &str, value: MatchBasicExtConfigVal4100) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&MatchBasicExtConfigVal4100> {
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
        self.set(key, MatchBasicExtConfigVal4100::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, MatchBasicExtConfigVal4100::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, MatchBasicExtConfigVal4100::Str(v.to_string()))
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
/// A utility type for MatchBasic (index 2).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil2 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil2 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil2 {
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
/// A utility type for MatchBasic (index 5).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil5 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil5 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil5 {
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
/// A logger for MatchBasic operations.
#[allow(dead_code)]
pub struct MatchBasicLogger {
    pub entries: Vec<String>,
    pub max_entries: usize,
    pub verbose: bool,
}
#[allow(dead_code)]
impl MatchBasicLogger {
    pub fn new(max_entries: usize) -> Self {
        MatchBasicLogger {
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
/// A utility type for MatchBasic (index 8).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MatchBasicUtil8 {
    pub id: usize,
    pub name: String,
    pub value: i64,
    pub enabled: bool,
    pub tags: Vec<String>,
}
#[allow(dead_code)]
impl MatchBasicUtil8 {
    pub fn new(id: usize, name: &str, value: i64) -> Self {
        MatchBasicUtil8 {
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
pub struct MatchBasicExtPipeline4100 {
    pub name: String,
    pub passes: Vec<MatchBasicExtPass4100>,
    pub run_count: usize,
}
impl MatchBasicExtPipeline4100 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: MatchBasicExtPass4100) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<MatchBasicExtResult4100> {
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
