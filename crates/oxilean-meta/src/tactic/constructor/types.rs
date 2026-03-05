//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::Name;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ConstructorExtResult3500 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl ConstructorExtResult3500 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, ConstructorExtResult3500::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, ConstructorExtResult3500::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, ConstructorExtResult3500::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, ConstructorExtResult3500::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let ConstructorExtResult3500::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let ConstructorExtResult3500::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            ConstructorExtResult3500::Ok(_) => 1.0,
            ConstructorExtResult3500::Err(_) => 0.0,
            ConstructorExtResult3500::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            ConstructorExtResult3500::Skipped => 0.5,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ConstructorExtConfigVal3500 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl ConstructorExtConfigVal3500 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let ConstructorExtConfigVal3500::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let ConstructorExtConfigVal3500::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let ConstructorExtConfigVal3500::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let ConstructorExtConfigVal3500::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let ConstructorExtConfigVal3500::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            ConstructorExtConfigVal3500::Bool(_) => "bool",
            ConstructorExtConfigVal3500::Int(_) => "int",
            ConstructorExtConfigVal3500::Float(_) => "float",
            ConstructorExtConfigVal3500::Str(_) => "str",
            ConstructorExtConfigVal3500::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct ConstructorExtDiag3500 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl ConstructorExtDiag3500 {
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
/// A static table of known inductive types and their constructors.
pub struct ConstructorTable {
    pub(super) entries: Vec<(String, Vec<ConstructorInfo>)>,
}
impl ConstructorTable {
    /// Create the built-in constructor table.
    pub fn builtin() -> Self {
        let mut t = Self {
            entries: Vec::new(),
        };
        t.register(
            "True",
            vec![ConstructorInfo::new(Name::str("True.intro"), 0).unique()],
        );
        t.register("False", vec![]);
        t.register(
            "And",
            vec![ConstructorInfo::new(Name::str("And.intro"), 2).unique()],
        );
        t.register(
            "Or",
            vec![
                ConstructorInfo::new(Name::str("Or.inl"), 1),
                ConstructorInfo::new(Name::str("Or.inr"), 1),
            ],
        );
        t.register(
            "Exists",
            vec![ConstructorInfo::new(Name::str("Exists.intro"), 2)
                .unique()
                .polymorphic()],
        );
        t.register(
            "Sigma",
            vec![ConstructorInfo::new(Name::str("Sigma.mk"), 2)
                .unique()
                .polymorphic()],
        );
        t.register(
            "Prod",
            vec![ConstructorInfo::new(Name::str("Prod.mk"), 2)
                .unique()
                .polymorphic()],
        );
        t.register(
            "Unit",
            vec![ConstructorInfo::new(Name::str("Unit.unit"), 0).unique()],
        );
        t.register(
            "Nat",
            vec![
                ConstructorInfo::new(Name::str("Nat.zero"), 0),
                ConstructorInfo::new(Name::str("Nat.succ"), 1),
            ],
        );
        t.register(
            "Bool",
            vec![
                ConstructorInfo::new(Name::str("Bool.true"), 0),
                ConstructorInfo::new(Name::str("Bool.false"), 0),
            ],
        );
        t.register(
            "List",
            vec![
                ConstructorInfo::new(Name::str("List.nil"), 0).polymorphic(),
                ConstructorInfo::new(Name::str("List.cons"), 2).polymorphic(),
            ],
        );
        t.register(
            "Option",
            vec![
                ConstructorInfo::new(Name::str("Option.none"), 0).polymorphic(),
                ConstructorInfo::new(Name::str("Option.some"), 1).polymorphic(),
            ],
        );
        t.register(
            "Sum",
            vec![
                ConstructorInfo::new(Name::str("Sum.inl"), 1).polymorphic(),
                ConstructorInfo::new(Name::str("Sum.inr"), 1).polymorphic(),
            ],
        );
        t.register(
            "Iff",
            vec![ConstructorInfo::new(Name::str("Iff.intro"), 2).unique()],
        );
        t
    }
    /// Register constructors for a type.
    pub fn register(&mut self, type_name: &str, ctors: Vec<ConstructorInfo>) {
        self.entries.push((type_name.to_string(), ctors));
    }
    /// Look up constructors for a type name.
    pub fn lookup(&self, type_name: &str) -> Option<&Vec<ConstructorInfo>> {
        self.entries
            .iter()
            .find(|(n, _)| n == type_name)
            .map(|(_, ctors)| ctors)
    }
    /// Number of registered types.
    pub fn num_types(&self) -> usize {
        self.entries.len()
    }
    /// Get the first constructor for a type.
    pub fn first_constructor(&self, type_name: &str) -> Option<&ConstructorInfo> {
        self.lookup(type_name)?.first()
    }
    /// Get the number of constructors for a type.
    pub fn num_constructors(&self, type_name: &str) -> usize {
        self.lookup(type_name).map_or(0, |v| v.len())
    }
    /// Check if a type has a unique constructor.
    pub fn has_unique_constructor(&self, type_name: &str) -> bool {
        self.lookup(type_name)
            .is_some_and(|ctors| ctors.len() == 1 && ctors[0].is_unique)
    }
}
/// An extended map for TacCtor keys to values.
#[allow(dead_code)]
pub struct TacCtorExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> TacCtorExtMap<V> {
    pub fn new() -> Self {
        TacCtorExtMap {
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
pub struct ConstructorExtConfig3500 {
    pub(super) values: std::collections::HashMap<String, ConstructorExtConfigVal3500>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl ConstructorExtConfig3500 {
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
    pub fn set(&mut self, key: &str, value: ConstructorExtConfigVal3500) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&ConstructorExtConfigVal3500> {
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
        self.set(key, ConstructorExtConfigVal3500::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, ConstructorExtConfigVal3500::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, ConstructorExtConfigVal3500::Str(v.to_string()))
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
/// Registry mapping type head names to their constructor lists.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ConstructorRegistry {
    pub(super) entries: std::collections::HashMap<String, Vec<Name>>,
}
impl ConstructorRegistry {
    /// Create an empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Register constructors for a type head.
    #[allow(dead_code)]
    pub fn register(&mut self, type_head: &str, ctors: Vec<Name>) {
        self.entries.insert(type_head.to_string(), ctors);
    }
    /// Look up constructors for a type head.
    #[allow(dead_code)]
    pub fn lookup(&self, type_head: &str) -> Option<&[Name]> {
        self.entries.get(type_head).map(|v| v.as_slice())
    }
    /// Get the first constructor for a type head.
    #[allow(dead_code)]
    pub fn first_ctor(&self, type_head: &str) -> Option<&Name> {
        self.entries.get(type_head)?.first()
    }
    /// Number of registered types.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Check if a type head is registered.
    #[allow(dead_code)]
    pub fn contains(&self, type_head: &str) -> bool {
        self.entries.contains_key(type_head)
    }
    /// Build a standard registry with the core OxiLean constructors.
    #[allow(dead_code)]
    pub fn standard() -> Self {
        let mut reg = Self::new();
        reg.register("And", vec![Name::str("And.intro")]);
        reg.register("Or", vec![Name::str("Or.inl"), Name::str("Or.inr")]);
        reg.register("Iff", vec![Name::str("Iff.intro")]);
        reg.register("Exists", vec![Name::str("Exists.intro")]);
        reg.register("Sigma", vec![Name::str("Sigma.mk")]);
        reg.register("Prod", vec![Name::str("Prod.mk")]);
        reg.register("Subtype", vec![Name::str("Subtype.mk")]);
        reg.register("True", vec![Name::str("True.intro")]);
        reg.register("List", vec![Name::str("List.nil"), Name::str("List.cons")]);
        reg.register("Nat", vec![Name::str("Nat.zero"), Name::str("Nat.succ")]);
        reg
    }
}
/// A sliding window accumulator for TacCtor.
#[allow(dead_code)]
pub struct TacCtorWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl TacCtorWindow {
    pub fn new(capacity: usize) -> Self {
        TacCtorWindow {
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
/// A result type for TacticConstructor analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticConstructorResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticConstructorResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticConstructorResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticConstructorResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticConstructorResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticConstructorResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticConstructorResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticConstructorResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticConstructorResult::Ok(_) => 1.0,
            TacticConstructorResult::Err(_) => 0.0,
            TacticConstructorResult::Skipped => 0.0,
            TacticConstructorResult::Partial { done, total } => {
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
pub struct ConstructorExtPass3500 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<ConstructorExtResult3500>,
}
impl ConstructorExtPass3500 {
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
    pub fn run(&mut self, input: &str) -> ConstructorExtResult3500 {
        if !self.enabled {
            return ConstructorExtResult3500::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            ConstructorExtResult3500::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            ConstructorExtResult3500::Ok(format!(
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
pub struct ConstructorExtPipeline3500 {
    pub name: String,
    pub passes: Vec<ConstructorExtPass3500>,
    pub run_count: usize,
}
impl ConstructorExtPipeline3500 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: ConstructorExtPass3500) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<ConstructorExtResult3500> {
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
/// A counter map for TacCtor frequency analysis.
#[allow(dead_code)]
pub struct TacCtorCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl TacCtorCounterMap {
    pub fn new() -> Self {
        TacCtorCounterMap {
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
/// Statistics about constructor tactic usage.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ConstructorTacStats {
    /// Total times `constructor` tactic was invoked.
    pub constructor_calls: u64,
    /// Times `left` was invoked.
    pub left_calls: u64,
    /// Times `right` was invoked.
    pub right_calls: u64,
    /// Times `existsi` was invoked.
    pub existsi_calls: u64,
    /// Times a constructor closed the goal.
    pub goals_closed: u64,
    /// Times a constructor failed.
    pub failures: u64,
}
impl ConstructorTacStats {
    /// Create zeroed stats.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Merge another stats object into this one.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &Self) {
        self.constructor_calls += other.constructor_calls;
        self.left_calls += other.left_calls;
        self.right_calls += other.right_calls;
        self.existsi_calls += other.existsi_calls;
        self.goals_closed += other.goals_closed;
        self.failures += other.failures;
    }
    /// Success rate (0.0–1.0).
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        let total =
            self.constructor_calls + self.left_calls + self.right_calls + self.existsi_calls;
        if total == 0 {
            1.0
        } else {
            (total - self.failures) as f64 / total as f64
        }
    }
}
pub struct TacCtorExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl TacCtorExtUtil {
    pub fn new(key: &str) -> Self {
        TacCtorExtUtil {
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
/// A state machine for TacCtor.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacCtorState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl TacCtorState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, TacCtorState::Complete | TacCtorState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, TacCtorState::Initial | TacCtorState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, TacCtorState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            TacCtorState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// A record of a constructor application attempt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConstructorAttempt {
    /// The constructor name tried.
    pub ctor_name: Name,
    /// Whether the attempt succeeded.
    pub succeeded: bool,
    /// Number of subgoals generated.
    pub subgoal_count: usize,
    /// Error message, if the attempt failed.
    pub error: Option<String>,
}
impl ConstructorAttempt {
    /// Create a successful attempt record.
    #[allow(dead_code)]
    pub fn success(name: Name, subgoals: usize) -> Self {
        Self {
            ctor_name: name,
            succeeded: true,
            subgoal_count: subgoals,
            error: None,
        }
    }
    /// Create a failed attempt record.
    #[allow(dead_code)]
    pub fn failure(name: Name, msg: impl Into<String>) -> Self {
        Self {
            ctor_name: name,
            succeeded: false,
            subgoal_count: 0,
            error: Some(msg.into()),
        }
    }
}
/// A pipeline of TacticConstructor analysis passes.
#[allow(dead_code)]
pub struct TacticConstructorPipeline {
    pub passes: Vec<TacticConstructorAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticConstructorPipeline {
    pub fn new(name: &str) -> Self {
        TacticConstructorPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticConstructorAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticConstructorResult> {
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
/// A diff for TacticConstructor analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticConstructorDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticConstructorDiff {
    pub fn new() -> Self {
        TacticConstructorDiff {
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
/// A configuration store for TacticConstructor.
#[allow(dead_code)]
pub struct TacticConstructorConfig {
    pub values: std::collections::HashMap<String, TacticConstructorConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticConstructorConfig {
    pub fn new() -> Self {
        TacticConstructorConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticConstructorConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticConstructorConfigValue> {
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
        self.set(key, TacticConstructorConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticConstructorConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticConstructorConfigValue::Str(v.to_string()))
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
#[allow(dead_code)]
pub struct ConstructorExtDiff3500 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl ConstructorExtDiff3500 {
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
/// An extended utility type for TacCtor.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TacCtorExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl TacCtorExt {
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
/// A typed slot for TacticConstructor configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticConstructorConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticConstructorConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticConstructorConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticConstructorConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticConstructorConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticConstructorConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticConstructorConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticConstructorConfigValue::Bool(_) => "bool",
            TacticConstructorConfigValue::Int(_) => "int",
            TacticConstructorConfigValue::Float(_) => "float",
            TacticConstructorConfigValue::Str(_) => "str",
            TacticConstructorConfigValue::List(_) => "list",
        }
    }
}
/// A state machine controller for TacCtor.
#[allow(dead_code)]
pub struct TacCtorStateMachine {
    pub state: TacCtorState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl TacCtorStateMachine {
    pub fn new() -> Self {
        TacCtorStateMachine {
            state: TacCtorState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: TacCtorState) -> bool {
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
        self.transition_to(TacCtorState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(TacCtorState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(TacCtorState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(TacCtorState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
/// A diagnostic reporter for TacticConstructor.
#[allow(dead_code)]
pub struct TacticConstructorDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticConstructorDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticConstructorDiagnostics {
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
/// A work queue for TacCtor items.
#[allow(dead_code)]
pub struct TacCtorWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl TacCtorWorkQueue {
    pub fn new(capacity: usize) -> Self {
        TacCtorWorkQueue {
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
/// Constructor metadata for an inductive type.
#[derive(Clone, Debug)]
pub struct ConstructorInfo {
    /// Constructor name.
    pub name: Name,
    /// Number of fields.
    pub num_fields: u32,
    /// Whether this is the only constructor.
    pub is_unique: bool,
    /// Whether this constructor takes type arguments.
    pub is_polymorphic: bool,
}
impl ConstructorInfo {
    /// Create a new constructor info.
    pub fn new(name: Name, num_fields: u32) -> Self {
        Self {
            name,
            num_fields,
            is_unique: false,
            is_polymorphic: false,
        }
    }
    /// Mark as the unique constructor.
    pub fn unique(mut self) -> Self {
        self.is_unique = true;
        self
    }
    /// Mark as polymorphic.
    pub fn polymorphic(mut self) -> Self {
        self.is_polymorphic = true;
        self
    }
}
/// A builder pattern for TacCtor.
#[allow(dead_code)]
pub struct TacCtorBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl TacCtorBuilder {
    pub fn new(name: &str) -> Self {
        TacCtorBuilder {
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
/// An analysis pass for TacticConstructor.
#[allow(dead_code)]
pub struct TacticConstructorAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticConstructorResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticConstructorAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticConstructorAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticConstructorResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticConstructorResult::Err("empty input".to_string())
        } else {
            TacticConstructorResult::Ok(format!("processed: {}", input))
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
