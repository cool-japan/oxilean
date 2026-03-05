//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;

#[allow(dead_code)]
pub struct FunPropExtDiff201 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl FunPropExtDiff201 {
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
/// The kind of function property that `fun_prop` tries to prove.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FunProperty {
    /// The function is continuous.
    Continuous,
    /// The function is measurable.
    Measurable,
    /// The function is (Fréchet) differentiable.
    Differentiable,
    /// The function is a bounded linear map.
    BoundedLinearMap,
    /// The function is monotone (order-preserving).
    Monotone,
    /// The function is antitone (order-reversing).
    Antitone,
    /// The function is strictly monotone.
    StrictMono,
    /// The function is injective.
    Injective,
    /// The function is surjective.
    Surjective,
    /// The function is bijective.
    Bijective,
}
/// Database of Lipschitz constants.
#[allow(dead_code)]
pub struct LipschitzDatabase {
    pub records: Vec<LipschitzInfo>,
}
#[allow(dead_code)]
impl LipschitzDatabase {
    pub fn new() -> Self {
        LipschitzDatabase {
            records: Vec::new(),
        }
    }
    pub fn register(&mut self, info: LipschitzInfo) {
        self.records.push(info);
    }
    pub fn lookup(&self, name: &str) -> Option<&LipschitzInfo> {
        self.records.iter().find(|r| r.name == name)
    }
    pub fn is_lipschitz(&self, name: &str) -> bool {
        self.lookup(name).is_some()
    }
}
#[allow(dead_code)]
pub struct FunPropExtDiag200 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl FunPropExtDiag200 {
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
#[allow(dead_code)]
pub struct FunPropExtPipeline201 {
    pub name: String,
    pub passes: Vec<FunPropExtPass201>,
    pub run_count: usize,
}
impl FunPropExtPipeline201 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: FunPropExtPass201) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<FunPropExtResult201> {
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
/// Information about Lipschitz continuity.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LipschitzInfo {
    pub name: String,
    pub constant: f64,
    pub is_contraction: bool,
}
#[allow(dead_code)]
impl LipschitzInfo {
    pub fn new(name: &str, constant: f64) -> Self {
        LipschitzInfo {
            name: name.to_string(),
            constant,
            is_contraction: constant < 1.0,
        }
    }
    pub fn lip_compose(&self, other: &Self) -> LipschitzInfo {
        LipschitzInfo::new(
            &format!("{}_circ_{}", self.name, other.name),
            self.constant * other.constant,
        )
    }
    pub fn is_non_expansive(&self) -> bool {
        self.constant <= 1.0
    }
}
/// A typed slot for TacticFunProp configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum TacticFunPropConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl TacticFunPropConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            TacticFunPropConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            TacticFunPropConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            TacticFunPropConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            TacticFunPropConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            TacticFunPropConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            TacticFunPropConfigValue::Bool(_) => "bool",
            TacticFunPropConfigValue::Int(_) => "int",
            TacticFunPropConfigValue::Float(_) => "float",
            TacticFunPropConfigValue::Str(_) => "str",
            TacticFunPropConfigValue::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct FunPropExtConfig201 {
    pub(super) values: std::collections::HashMap<String, FunPropExtConfigVal201>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl FunPropExtConfig201 {
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
    pub fn set(&mut self, key: &str, value: FunPropExtConfigVal201) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&FunPropExtConfigVal201> {
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
        self.set(key, FunPropExtConfigVal201::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, FunPropExtConfigVal201::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, FunPropExtConfigVal201::Str(v.to_string()))
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
/// Types of measurability.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MeasurabilityKind {
    BorelMeas,
    LebesgueMeas,
    UniversalMeas,
    CustomMeas(String),
}
/// Database of differentiability records.
#[allow(dead_code)]
pub struct DiffDatabase {
    pub records: Vec<DiffRecord>,
}
#[allow(dead_code)]
impl DiffDatabase {
    pub fn new() -> Self {
        DiffDatabase {
            records: Vec::new(),
        }
    }
    pub fn register(&mut self, rec: DiffRecord) {
        self.records.push(rec);
    }
    pub fn lookup(&self, name: &str) -> Option<&DiffRecord> {
        self.records.iter().find(|r| r.name == name)
    }
    pub fn diff_is_smooth(&self, name: &str) -> bool {
        self.lookup(name).map(|r| r.is_smooth).unwrap_or(false)
    }
    pub fn diff_is_analytic(&self, name: &str) -> bool {
        self.lookup(name).map(|r| r.is_analytic).unwrap_or(false)
    }
    pub fn num_records(&self) -> usize {
        self.records.len()
    }
}
/// A goal for the fun_prop tactic: prove that `func` satisfies `property`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunPropGoal {
    pub func_name: String,
    pub property: String,
    pub domain: String,
    pub codomain: String,
    pub assumptions: Vec<String>,
}
#[allow(dead_code)]
impl FunPropGoal {
    pub fn new(func_name: &str, property: &str) -> Self {
        FunPropGoal {
            func_name: func_name.to_string(),
            property: property.to_string(),
            domain: "ℝ".to_string(),
            codomain: "ℝ".to_string(),
            assumptions: Vec::new(),
        }
    }
    pub fn with_domain(mut self, domain: &str) -> Self {
        self.domain = domain.to_string();
        self
    }
    pub fn with_codomain(mut self, codomain: &str) -> Self {
        self.codomain = codomain.to_string();
        self
    }
    pub fn add_assumption(&mut self, assumption: &str) {
        self.assumptions.push(assumption.to_string());
    }
    pub fn description(&self) -> String {
        format!(
            "{} : {} → {} is {}",
            self.func_name, self.domain, self.codomain, self.property
        )
    }
}
/// Solver for fun_prop goals.
#[allow(dead_code)]
pub struct FunPropSolver {
    pub registry: FunPropRegistry,
    pub cache: FunPropCache,
    pub max_depth: usize,
}
#[allow(dead_code)]
impl FunPropSolver {
    pub fn new() -> Self {
        FunPropSolver {
            registry: FunPropRegistry::new(),
            cache: FunPropCache::new(),
            max_depth: 10,
        }
    }
    pub fn solve(&mut self, goal: &FunPropGoal) -> Option<FunPropStrength> {
        if let Some(cached) = self.cache.lookup(&goal.func_name, &goal.property) {
            if cached {
                return Some(FunPropStrength::Continuous);
            } else {
                return None;
            }
        }
        let strength = self.registry.strongest_property(&goal.func_name);
        let has_prop = self
            .registry
            .has_property(&goal.func_name, &FunPropStrength::Continuous);
        self.cache.insert(&goal.func_name, &goal.property, has_prop);
        if strength > FunPropStrength::Unknown {
            Some(strength)
        } else {
            None
        }
    }
    pub fn register_function(&mut self, func: &str, prop: FunPropStrength) {
        self.registry.register(func, prop);
    }
    pub fn cache_hit_rate(&self) -> f64 {
        self.cache.hit_rate()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FunPropExtConfigVal200 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl FunPropExtConfigVal200 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let FunPropExtConfigVal200::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let FunPropExtConfigVal200::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let FunPropExtConfigVal200::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let FunPropExtConfigVal200::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let FunPropExtConfigVal200::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            FunPropExtConfigVal200::Bool(_) => "bool",
            FunPropExtConfigVal200::Int(_) => "int",
            FunPropExtConfigVal200::Float(_) => "float",
            FunPropExtConfigVal200::Str(_) => "str",
            FunPropExtConfigVal200::List(_) => "list",
        }
    }
}
/// A configuration store for TacticFunProp.
#[allow(dead_code)]
pub struct TacticFunPropConfig {
    pub values: std::collections::HashMap<String, TacticFunPropConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl TacticFunPropConfig {
    pub fn new() -> Self {
        TacticFunPropConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: TacticFunPropConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&TacticFunPropConfigValue> {
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
        self.set(key, TacticFunPropConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, TacticFunPropConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, TacticFunPropConfigValue::Str(v.to_string()))
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
/// A function property goal with a typed property.
#[allow(dead_code)]
pub struct ExtFunPropGoal {
    pub func_name: String,
    pub property: FunPropStrength,
}
/// A tag annotating a function expression with a property and an optional space.
#[derive(Clone, Debug)]
pub struct FunPropTag {
    /// The property being asserted.
    pub property: FunProperty,
    /// Optional ambient space name (e.g. `"ℝ"`, `"NormedSpace"`).
    pub space: Option<String>,
}
impl FunPropTag {
    /// Create a new tag with no ambient space.
    pub fn new(property: FunProperty) -> Self {
        Self {
            property,
            space: None,
        }
    }
    /// Create a tag with an ambient space.
    pub fn with_space(property: FunProperty, space: impl Into<String>) -> Self {
        Self {
            property,
            space: Some(space.into()),
        }
    }
}
/// Cache for function property queries.
#[allow(dead_code)]
pub struct FunPropCache {
    /// Cache: (function_name, property_name) -> bool.
    pub cache: std::collections::HashMap<(String, String), bool>,
    pub hits: usize,
    pub misses: usize,
}
#[allow(dead_code)]
impl FunPropCache {
    pub fn new() -> Self {
        FunPropCache {
            cache: std::collections::HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }
    pub fn lookup(&mut self, func: &str, prop: &str) -> Option<bool> {
        let key = (func.to_string(), prop.to_string());
        match self.cache.get(&key).copied() {
            Some(v) => {
                self.hits += 1;
                Some(v)
            }
            None => {
                self.misses += 1;
                None
            }
        }
    }
    pub fn insert(&mut self, func: &str, prop: &str, value: bool) {
        self.cache
            .insert((func.to_string(), prop.to_string()), value);
    }
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    pub fn size(&self) -> usize {
        self.cache.len()
    }
    pub fn clear(&mut self) {
        self.cache.clear();
        self.hits = 0;
        self.misses = 0;
    }
}
/// A diff for TacticFunProp analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TacticFunPropDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl TacticFunPropDiff {
    pub fn new() -> Self {
        TacticFunPropDiff {
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
/// A record of differentiability for a function.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct DiffRecord {
    pub name: String,
    pub at_point: Option<f64>,
    pub derivative: Option<f64>,
    pub is_smooth: bool,
    pub is_analytic: bool,
}
#[allow(dead_code)]
impl DiffRecord {
    pub fn new(name: &str) -> Self {
        DiffRecord {
            name: name.to_string(),
            at_point: None,
            derivative: None,
            is_smooth: false,
            is_analytic: false,
        }
    }
    pub fn with_point(mut self, x: f64) -> Self {
        self.at_point = Some(x);
        self
    }
    pub fn with_derivative(mut self, d: f64) -> Self {
        self.derivative = Some(d);
        self
    }
    pub fn diff_smooth(mut self) -> Self {
        self.is_smooth = true;
        self
    }
    pub fn diff_analytic(mut self) -> Self {
        self.is_analytic = true;
        self.is_smooth = true;
        self
    }
}
/// A database of `fun_prop` rules.
#[derive(Clone, Debug, Default)]
pub struct FunPropDatabase {
    /// All rules registered in this database.
    pub rules: Vec<FunPropRule>,
}
impl FunPropDatabase {
    /// Create an empty database.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a database pre-populated with standard library rules for common
    /// function combinators: `id`, `const`, `comp`, `add`, `mul`.
    pub fn with_standard_rules() -> Self {
        let mut db = Self::new();
        for prop in [
            FunProperty::Continuous,
            FunProperty::Measurable,
            FunProperty::Monotone,
            FunProperty::StrictMono,
            FunProperty::Injective,
        ] {
            let mut rule = FunPropRule::new("id_rule", prop);
            rule.add_target("id");
            db.add_rule(rule);
        }
        for prop in [FunProperty::Continuous, FunProperty::Measurable] {
            let mut rule = FunPropRule::new("const_rule", prop);
            rule.add_target("const");
            db.add_rule(rule);
        }
        for prop in [
            FunProperty::Continuous,
            FunProperty::Measurable,
            FunProperty::Monotone,
        ] {
            let mut rule = FunPropRule::new("comp_rule", prop);
            rule.add_target("comp");
            rule.add_target("Function.comp");
            db.add_rule(rule);
        }
        for prop in [FunProperty::Continuous, FunProperty::Measurable] {
            let mut rule = FunPropRule::new("add_rule", prop);
            rule.add_target("add");
            rule.add_target("HAdd.hAdd");
            db.add_rule(rule);
        }
        for prop in [FunProperty::Continuous, FunProperty::Measurable] {
            let mut rule = FunPropRule::new("mul_rule", prop);
            rule.add_target("mul");
            rule.add_target("HMul.hMul");
            db.add_rule(rule);
        }
        db
    }
    /// Add a rule to the database.
    pub fn add_rule(&mut self, rule: FunPropRule) {
        self.rules.push(rule);
    }
    /// Return all rules that establish `property`.
    pub fn find_rules(&self, property: &FunProperty) -> Vec<&FunPropRule> {
        self.rules
            .iter()
            .filter(|r| &r.property == property)
            .collect()
    }
    /// Check whether the database has a rule that can prove `property` for
    /// the function named `fn_name`.
    pub fn can_prove(&self, property: &FunProperty, fn_name: &str) -> bool {
        self.rules
            .iter()
            .any(|r| &r.property == property && r.applies_to.iter().any(|t| t == fn_name))
    }
}
/// The `fun_prop` tactic engine.
pub struct FunPropTactic {
    pub(super) db: FunPropDatabase,
    pub(super) config: FunPropConfig,
}
impl FunPropTactic {
    /// Create a tactic with an empty database and default config.
    pub fn new() -> Self {
        Self {
            db: FunPropDatabase::new(),
            config: FunPropConfig::default(),
        }
    }
    /// Create a tactic with the given rule database.
    pub fn with_db(db: FunPropDatabase) -> Self {
        Self {
            db,
            config: FunPropConfig::default(),
        }
    }
    /// Override the configuration.
    pub fn with_config(mut self, config: FunPropConfig) -> Self {
        self.config = config;
        self
    }
    /// Try to prove `property` for `expr`.
    ///
    /// Returns a proof-term string on success, or `None` on failure.
    pub fn prove_property(&self, property: &FunProperty, expr: &str) -> Option<String> {
        let fn_name = expr.trim();
        if self.db.can_prove(property, fn_name) {
            for rule in self.db.find_rules(property) {
                if rule.applies_to.iter().any(|t| t == fn_name) {
                    return Some(format!("{}.{}", rule.name, fn_name));
                }
            }
        }
        let comp_proof = self.try_composition(property, fn_name)?;
        Some(comp_proof)
    }
    /// Try to prove `Continuous fn_expr`.
    pub fn prove_continuous(&self, fn_expr: &str) -> Option<String> {
        self.prove_property(&FunProperty::Continuous, fn_expr)
    }
    /// Try to prove `Measurable fn_expr`.
    pub fn prove_measurable(&self, fn_expr: &str) -> Option<String> {
        self.prove_property(&FunProperty::Measurable, fn_expr)
    }
    /// Try to prove `Monotone fn_expr`.
    pub fn prove_monotone(&self, fn_expr: &str) -> Option<String> {
        self.prove_property(&FunProperty::Monotone, fn_expr)
    }
    /// Given that `f` has property `f_prop` and `g` has property `g_prop`,
    /// derive the property of their composition `f ∘ g`, if any rule applies.
    pub fn apply_composition_rule(
        &self,
        f_prop: FunProperty,
        g_prop: FunProperty,
    ) -> Option<FunProperty> {
        if f_prop == FunProperty::Continuous && g_prop == FunProperty::Continuous {
            return Some(FunProperty::Continuous);
        }
        if f_prop == FunProperty::Measurable && g_prop == FunProperty::Measurable {
            return Some(FunProperty::Measurable);
        }
        if f_prop == FunProperty::Monotone && g_prop == FunProperty::Monotone {
            return Some(FunProperty::Monotone);
        }
        if f_prop == FunProperty::Antitone && g_prop == FunProperty::Antitone {
            return Some(FunProperty::Monotone);
        }
        if (f_prop == FunProperty::Monotone && g_prop == FunProperty::Antitone)
            || (f_prop == FunProperty::Antitone && g_prop == FunProperty::Monotone)
        {
            return Some(FunProperty::Antitone);
        }
        if f_prop == FunProperty::Injective && g_prop == FunProperty::Injective {
            return Some(FunProperty::Injective);
        }
        None
    }
    /// Attempt to decompose `fn_name` as a composition and prove the property
    /// by the composition rule.
    fn try_composition(&self, property: &FunProperty, fn_name: &str) -> Option<String> {
        let parts: Vec<&str> = fn_name.splitn(3, ' ').collect();
        if parts.len() == 3 && (parts[0] == "comp" || parts[0] == "Function.comp") {
            let f = parts[1];
            let g = parts[2];
            self.prove_property(property, f)?;
            self.prove_property(property, g)?;
            return Some(format!("comp_rule.{}.{}", f, g));
        }
        None
    }
}
/// Configuration options for the `fun_prop` tactic.
#[derive(Clone, Debug)]
pub struct FunPropConfig {
    /// Maximum search depth for recursive rule application.
    pub max_depth: usize,
    /// Whether to invoke `simp` as a sub-step when stuck.
    pub use_simp: bool,
    /// Print debug trace if `true`.
    pub verbose: bool,
}
/// The lattice of function properties (ordered by strength).
/// Continuous > Measurable > Unknown
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FunPropStrength {
    Unknown = 0,
    Measurable = 1,
    Integrable = 2,
    Continuous = 3,
    Differentiable = 4,
    SmoothInfinite = 5,
    Analytic = 6,
}
#[allow(dead_code)]
impl FunPropStrength {
    pub fn meets(&self, other: &Self) -> Self {
        use std::cmp::min;
        if min(self, other) == self {
            self.clone()
        } else {
            other.clone()
        }
    }
    pub fn joins(&self, other: &Self) -> Self {
        use std::cmp::max;
        if max(self, other) == self {
            self.clone()
        } else {
            other.clone()
        }
    }
    pub fn implies(&self, other: &Self) -> bool {
        self >= other
    }
    pub fn name(&self) -> &'static str {
        match self {
            FunPropStrength::Unknown => "unknown",
            FunPropStrength::Measurable => "measurable",
            FunPropStrength::Integrable => "integrable",
            FunPropStrength::Continuous => "continuous",
            FunPropStrength::Differentiable => "differentiable",
            FunPropStrength::SmoothInfinite => "smooth",
            FunPropStrength::Analytic => "analytic",
        }
    }
}
/// A registry of known function properties.
#[allow(dead_code)]
pub struct FunPropRegistry {
    /// Map from function name to list of known properties.
    pub entries: std::collections::HashMap<String, Vec<FunPropStrength>>,
}
#[allow(dead_code)]
impl FunPropRegistry {
    pub fn new() -> Self {
        let mut reg = FunPropRegistry {
            entries: std::collections::HashMap::new(),
        };
        reg.register("sin", FunPropStrength::Analytic);
        reg.register("cos", FunPropStrength::Analytic);
        reg.register("exp", FunPropStrength::Analytic);
        reg.register("log", FunPropStrength::Analytic);
        reg.register("sqrt", FunPropStrength::Continuous);
        reg.register("abs", FunPropStrength::Continuous);
        reg.register("floor", FunPropStrength::Measurable);
        reg.register("ceil", FunPropStrength::Measurable);
        reg
    }
    pub fn register(&mut self, func: &str, prop: FunPropStrength) {
        self.entries.entry(func.to_string()).or_default().push(prop);
    }
    pub fn strongest_property(&self, func: &str) -> FunPropStrength {
        self.entries
            .get(func)
            .and_then(|props| props.iter().max())
            .cloned()
            .unwrap_or(FunPropStrength::Unknown)
    }
    pub fn has_property(&self, func: &str, prop: &FunPropStrength) -> bool {
        self.strongest_property(func).implies(prop)
    }
    pub fn num_functions(&self) -> usize {
        self.entries.len()
    }
}
/// Database of measurability facts.
#[allow(dead_code)]
pub struct MeasDatabase {
    pub records: std::collections::HashMap<String, MeasRecord>,
}
#[allow(dead_code)]
impl MeasDatabase {
    pub fn new() -> Self {
        MeasDatabase {
            records: std::collections::HashMap::new(),
        }
    }
    pub fn register(&mut self, rec: MeasRecord) {
        self.records.insert(rec.name.clone(), rec);
    }
    pub fn is_measurable(&self, name: &str) -> bool {
        self.records.contains_key(name)
    }
    pub fn lookup(&self, name: &str) -> Option<&MeasRecord> {
        self.records.get(name)
    }
    pub fn num_records(&self) -> usize {
        self.records.len()
    }
}
/// Check continuity of a function at a point using epsilon-delta.
#[allow(dead_code)]
pub struct ContinuityChecker {
    pub epsilon: f64,
    pub delta: f64,
    pub checks_passed: usize,
    pub checks_failed: usize,
}
#[allow(dead_code)]
impl ContinuityChecker {
    pub fn new(epsilon: f64, delta: f64) -> Self {
        ContinuityChecker {
            epsilon,
            delta,
            checks_passed: 0,
            checks_failed: 0,
        }
    }
    pub fn check<F: Fn(f64) -> f64>(&mut self, f: F, x0: f64, test_points: &[f64]) -> bool {
        let fx0 = f(x0);
        let mut all_pass = true;
        for &x in test_points {
            if (x - x0).abs() < self.delta {
                let diff = (f(x) - fx0).abs();
                if diff < self.epsilon {
                    self.checks_passed += 1;
                } else {
                    self.checks_failed += 1;
                    all_pass = false;
                }
            }
        }
        all_pass
    }
    pub fn total_checks(&self) -> usize {
        self.checks_passed + self.checks_failed
    }
    pub fn success_rate(&self) -> f64 {
        let t = self.total_checks();
        if t == 0 {
            1.0
        } else {
            self.checks_passed as f64 / t as f64
        }
    }
}
/// A result type for TacticFunProp analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TacticFunPropResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl TacticFunPropResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, TacticFunPropResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, TacticFunPropResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, TacticFunPropResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, TacticFunPropResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            TacticFunPropResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            TacticFunPropResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            TacticFunPropResult::Ok(_) => 1.0,
            TacticFunPropResult::Err(_) => 0.0,
            TacticFunPropResult::Skipped => 0.0,
            TacticFunPropResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A lattice of function properties ordered by implication.
#[allow(dead_code)]
pub struct PropLattice {
    pub props: Vec<FunPropStrength>,
}
#[allow(dead_code)]
impl PropLattice {
    pub fn standard() -> Self {
        PropLattice {
            props: vec![
                FunPropStrength::SmoothInfinite,
                FunPropStrength::Differentiable,
                FunPropStrength::Continuous,
                FunPropStrength::Measurable,
            ],
        }
    }
    pub fn implied_by(&self, prop: &FunPropStrength) -> Vec<FunPropStrength> {
        self.props
            .iter()
            .filter(|p| is_stronger_than(prop, p))
            .cloned()
            .collect()
    }
    pub fn lattice_join(&self, props: &[FunPropStrength]) -> Option<FunPropStrength> {
        for candidate in &self.props {
            if props.iter().all(|p| is_stronger_than(candidate, p)) {
                return Some(candidate.clone());
            }
        }
        None
    }
    pub fn lattice_meet(&self, props: &[FunPropStrength]) -> Option<FunPropStrength> {
        for candidate in self.props.iter().rev() {
            if props.iter().all(|p| is_stronger_than(p, candidate)) {
                return Some(candidate.clone());
            }
        }
        None
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FunPropExtConfigVal201 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl FunPropExtConfigVal201 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let FunPropExtConfigVal201::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let FunPropExtConfigVal201::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let FunPropExtConfigVal201::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let FunPropExtConfigVal201::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let FunPropExtConfigVal201::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            FunPropExtConfigVal201::Bool(_) => "bool",
            FunPropExtConfigVal201::Int(_) => "int",
            FunPropExtConfigVal201::Float(_) => "float",
            FunPropExtConfigVal201::Str(_) => "str",
            FunPropExtConfigVal201::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FunPropExtResult201 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl FunPropExtResult201 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, FunPropExtResult201::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, FunPropExtResult201::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, FunPropExtResult201::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, FunPropExtResult201::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let FunPropExtResult201::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let FunPropExtResult201::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            FunPropExtResult201::Ok(_) => 1.0,
            FunPropExtResult201::Err(_) => 0.0,
            FunPropExtResult201::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            FunPropExtResult201::Skipped => 0.5,
        }
    }
}
/// A rule in the `fun_prop` database that says a named lemma proves a property
/// for certain function-expression patterns.
#[derive(Clone, Debug)]
pub struct FunPropRule {
    /// Canonical lemma / rule name.
    pub name: String,
    /// The property this rule can establish.
    pub property: FunProperty,
    /// Function names / expression shapes this rule applies to.
    pub applies_to: Vec<String>,
}
impl FunPropRule {
    /// Create a new rule with an empty `applies_to` list.
    pub fn new(name: &str, property: FunProperty) -> Self {
        Self {
            name: name.to_string(),
            property,
            applies_to: Vec::new(),
        }
    }
    /// Register a function-expression pattern that this rule handles.
    pub fn add_target(&mut self, target: &str) {
        self.applies_to.push(target.to_string());
    }
}
/// A pipeline of TacticFunProp analysis passes.
#[allow(dead_code)]
pub struct TacticFunPropPipeline {
    pub passes: Vec<TacticFunPropAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl TacticFunPropPipeline {
    pub fn new(name: &str) -> Self {
        TacticFunPropPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: TacticFunPropAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<TacticFunPropResult> {
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
pub struct FunPropExtDiag201 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl FunPropExtDiag201 {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FunPropExtResult200 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl FunPropExtResult200 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, FunPropExtResult200::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, FunPropExtResult200::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, FunPropExtResult200::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, FunPropExtResult200::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let FunPropExtResult200::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let FunPropExtResult200::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            FunPropExtResult200::Ok(_) => 1.0,
            FunPropExtResult200::Err(_) => 0.0,
            FunPropExtResult200::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            FunPropExtResult200::Skipped => 0.5,
        }
    }
}
/// A proof attempt for a function property goal.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FunPropProof {
    Direct(String),
    ByComposition { f: String, g: String, rule: String },
    BySum { fs: Vec<String> },
    ByProduct { fs: Vec<String> },
    ByLimit { sequence: String },
    Failed(String),
}
#[allow(dead_code)]
impl FunPropProof {
    pub fn is_success(&self) -> bool {
        !matches!(self, FunPropProof::Failed(_))
    }
    pub fn failure_reason(&self) -> Option<&str> {
        match self {
            FunPropProof::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// A trace of fun_prop tactic steps.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FunPropTrace {
    pub steps: Vec<String>,
}
#[allow(dead_code)]
impl FunPropTrace {
    pub fn new() -> Self {
        FunPropTrace { steps: Vec::new() }
    }
    pub fn log(&mut self, step: &str) {
        self.steps.push(step.to_string());
    }
    pub fn num_steps(&self) -> usize {
        self.steps.len()
    }
    pub fn summarize(&self) -> String {
        format!("{} steps: {}", self.steps.len(), self.steps.join("; "))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct MeasRecord {
    pub name: String,
    pub kind: MeasurabilityKind,
    pub is_strongly_measurable: bool,
}
#[allow(dead_code)]
impl MeasRecord {
    pub fn borel(name: &str) -> Self {
        MeasRecord {
            name: name.to_string(),
            kind: MeasurabilityKind::BorelMeas,
            is_strongly_measurable: false,
        }
    }
    pub fn strongly(name: &str) -> Self {
        MeasRecord {
            name: name.to_string(),
            kind: MeasurabilityKind::BorelMeas,
            is_strongly_measurable: true,
        }
    }
}
/// A diagnostic reporter for TacticFunProp.
#[allow(dead_code)]
pub struct TacticFunPropDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl TacticFunPropDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        TacticFunPropDiagnostics {
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
pub struct FunPropExtDiff200 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl FunPropExtDiff200 {
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
pub struct FunPropExtPass200 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<FunPropExtResult200>,
}
impl FunPropExtPass200 {
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
    pub fn run(&mut self, input: &str) -> FunPropExtResult200 {
        if !self.enabled {
            return FunPropExtResult200::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            FunPropExtResult200::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            FunPropExtResult200::Ok(format!(
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
pub struct FunPropExtPass201 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<FunPropExtResult201>,
}
impl FunPropExtPass201 {
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
    pub fn run(&mut self, input: &str) -> FunPropExtResult201 {
        if !self.enabled {
            return FunPropExtResult201::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            FunPropExtResult201::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            FunPropExtResult201::Ok(format!(
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
pub struct FunPropExtPipeline200 {
    pub name: String,
    pub passes: Vec<FunPropExtPass200>,
    pub run_count: usize,
}
impl FunPropExtPipeline200 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: FunPropExtPass200) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<FunPropExtResult200> {
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
/// An analysis pass for TacticFunProp.
#[allow(dead_code)]
pub struct TacticFunPropAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<TacticFunPropResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl TacticFunPropAnalysisPass {
    pub fn new(name: &str) -> Self {
        TacticFunPropAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> TacticFunPropResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            TacticFunPropResult::Err("empty input".to_string())
        } else {
            TacticFunPropResult::Ok(format!("processed: {}", input))
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
pub struct FunPropExtConfig200 {
    pub(super) values: std::collections::HashMap<String, FunPropExtConfigVal200>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl FunPropExtConfig200 {
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
    pub fn set(&mut self, key: &str, value: FunPropExtConfigVal200) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&FunPropExtConfigVal200> {
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
        self.set(key, FunPropExtConfigVal200::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, FunPropExtConfigVal200::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, FunPropExtConfigVal200::Str(v.to_string()))
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
