//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::declaration::{
    ConstantInfo, ConstantVal, ConstructorVal, InductiveVal, RecursorRule, RecursorVal,
};
use crate::{Expr, KernelError, Level, Name};
use std::collections::HashMap;

/// A generic counter that tracks min/max/sum for statistical summaries.
#[allow(dead_code)]
pub struct StatSummary {
    count: u64,
    sum: f64,
    min: f64,
    max: f64,
}
#[allow(dead_code)]
impl StatSummary {
    /// Creates an empty summary.
    pub fn new() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
    /// Records a sample.
    pub fn record(&mut self, val: f64) {
        self.count += 1;
        self.sum += val;
        if val < self.min {
            self.min = val;
        }
        if val > self.max {
            self.max = val;
        }
    }
    /// Returns the mean, or `None` if no samples.
    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }
    }
    /// Returns the minimum, or `None` if no samples.
    pub fn min(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.min)
        }
    }
    /// Returns the maximum, or `None` if no samples.
    pub fn max(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.max)
        }
    }
    /// Returns the count of recorded samples.
    pub fn count(&self) -> u64 {
        self.count
    }
}
/// Errors that can occur when checking an inductive type definition.
#[derive(Clone, Debug, PartialEq)]
pub enum InductiveError {
    /// The type name is already defined.
    AlreadyDefined(Name),
    /// A constructor references a type that is not in scope.
    UndefinedType(Name),
    /// A constructor has an invalid type (not a Pi ending in the inductive type).
    InvalidConstructorType(Name),
    /// The universe level is not large enough.
    UniverseTooSmall(String),
    /// A non-strictly positive occurrence of the type.
    NonStrictlyPositive(Name),
    /// General error.
    Other(String),
}
/// An introduction rule (constructor) for an inductive type.
#[derive(Clone, Debug, PartialEq)]
pub struct IntroRule {
    /// Constructor name
    pub name: Name,
    /// Constructor type (as a Pi-type)
    pub ty: Expr,
}
/// A mutually inductive family of types.
///
/// Lean 4 supports mutual induction (e.g., `Even` and `Odd` defined together).
/// This struct holds a group of inductive types that are checked together.
#[derive(Clone, Debug)]
pub struct InductiveFamily {
    /// The inductive types in the family.
    pub types: Vec<InductiveType>,
    /// Shared universe parameters.
    pub univ_params: Vec<Name>,
}
impl InductiveFamily {
    /// Create a single-type family.
    pub fn singleton(ty: InductiveType) -> Self {
        let univ_params = ty.univ_params.clone();
        Self {
            types: vec![ty],
            univ_params,
        }
    }
    /// Create a mutually inductive family.
    pub fn new(types: Vec<InductiveType>, univ_params: Vec<Name>) -> Self {
        Self { types, univ_params }
    }
    /// Number of types in the family.
    pub fn len(&self) -> usize {
        self.types.len()
    }
    /// Whether the family contains a single type.
    pub fn is_singleton(&self) -> bool {
        self.types.len() == 1
    }
    /// Whether the family is empty.
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }
    /// All type names in the family.
    pub fn type_names(&self) -> Vec<&Name> {
        self.types.iter().map(|t| &t.name).collect()
    }
    /// All constructor names across all types.
    pub fn all_constructor_names(&self) -> Vec<&Name> {
        self.types
            .iter()
            .flat_map(|t| t.intro_rules.iter().map(|r| &r.name))
            .collect()
    }
    /// Find a type by name.
    pub fn find_type(&self, name: &Name) -> Option<&InductiveType> {
        self.types.iter().find(|t| &t.name == name)
    }
    /// Total number of constructors across all types.
    pub fn total_constructors(&self) -> usize {
        self.types.iter().map(|t| t.intro_rules.len()).sum()
    }
}
/// A simple key-value store backed by a sorted Vec for small maps.
#[allow(dead_code)]
pub struct SmallMap<K: Ord + Clone, V: Clone> {
    entries: Vec<(K, V)>,
}
#[allow(dead_code)]
impl<K: Ord + Clone, V: Clone> SmallMap<K, V> {
    /// Creates a new empty small map.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    /// Inserts or replaces the value for `key`.
    pub fn insert(&mut self, key: K, val: V) {
        match self.entries.binary_search_by_key(&&key, |(k, _)| k) {
            Ok(i) => self.entries[i].1 = val,
            Err(i) => self.entries.insert(i, (key, val)),
        }
    }
    /// Returns the value for `key`, or `None`.
    pub fn get(&self, key: &K) -> Option<&V> {
        self.entries
            .binary_search_by_key(&key, |(k, _)| k)
            .ok()
            .map(|i| &self.entries[i].1)
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Returns all keys.
    pub fn keys(&self) -> Vec<&K> {
        self.entries.iter().map(|(k, _)| k).collect()
    }
    /// Returns all values.
    pub fn values(&self) -> Vec<&V> {
        self.entries.iter().map(|(_, v)| v).collect()
    }
}
/// A hierarchical configuration tree.
#[allow(dead_code)]
pub struct ConfigNode {
    key: String,
    value: Option<String>,
    children: Vec<ConfigNode>,
}
#[allow(dead_code)]
impl ConfigNode {
    /// Creates a leaf config node with a value.
    pub fn leaf(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: Some(value.into()),
            children: Vec::new(),
        }
    }
    /// Creates a section node with children.
    pub fn section(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            value: None,
            children: Vec::new(),
        }
    }
    /// Adds a child node.
    pub fn add_child(&mut self, child: ConfigNode) {
        self.children.push(child);
    }
    /// Returns the key.
    pub fn key(&self) -> &str {
        &self.key
    }
    /// Returns the value, or `None` for section nodes.
    pub fn value(&self) -> Option<&str> {
        self.value.as_deref()
    }
    /// Returns the number of children.
    pub fn num_children(&self) -> usize {
        self.children.len()
    }
    /// Looks up a dot-separated path.
    pub fn lookup(&self, path: &str) -> Option<&str> {
        let mut parts = path.splitn(2, '.');
        let head = parts.next()?;
        let tail = parts.next();
        if head != self.key {
            return None;
        }
        match tail {
            None => self.value.as_deref(),
            Some(rest) => self.children.iter().find_map(|c| c.lookup_relative(rest)),
        }
    }
    fn lookup_relative(&self, path: &str) -> Option<&str> {
        let mut parts = path.splitn(2, '.');
        let head = parts.next()?;
        let tail = parts.next();
        if head != self.key {
            return None;
        }
        match tail {
            None => self.value.as_deref(),
            Some(rest) => self.children.iter().find_map(|c| c.lookup_relative(rest)),
        }
    }
}
/// A simple directed acyclic graph.
#[allow(dead_code)]
pub struct SimpleDag {
    /// `edges[i]` is the list of direct successors of node `i`.
    edges: Vec<Vec<usize>>,
}
#[allow(dead_code)]
impl SimpleDag {
    /// Creates a DAG with `n` nodes and no edges.
    pub fn new(n: usize) -> Self {
        Self {
            edges: vec![Vec::new(); n],
        }
    }
    /// Adds an edge from `from` to `to`.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.edges.len() {
            self.edges[from].push(to);
        }
    }
    /// Returns the successors of `node`.
    pub fn successors(&self, node: usize) -> &[usize] {
        self.edges.get(node).map(|v| v.as_slice()).unwrap_or(&[])
    }
    /// Returns `true` if `from` can reach `to` via DFS.
    pub fn can_reach(&self, from: usize, to: usize) -> bool {
        let mut visited = vec![false; self.edges.len()];
        self.dfs(from, to, &mut visited)
    }
    fn dfs(&self, cur: usize, target: usize, visited: &mut Vec<bool>) -> bool {
        if cur == target {
            return true;
        }
        if cur >= visited.len() || visited[cur] {
            return false;
        }
        visited[cur] = true;
        for &next in self.successors(cur) {
            if self.dfs(next, target, visited) {
                return true;
            }
        }
        false
    }
    /// Returns the topological order of nodes, or `None` if a cycle is detected.
    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        let n = self.edges.len();
        let mut in_degree = vec![0usize; n];
        for succs in &self.edges {
            for &s in succs {
                if s < n {
                    in_degree[s] += 1;
                }
            }
        }
        let mut queue: std::collections::VecDeque<usize> =
            (0..n).filter(|&i| in_degree[i] == 0).collect();
        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &s in self.successors(node) {
                if s < n {
                    in_degree[s] -= 1;
                    if in_degree[s] == 0 {
                        queue.push_back(s);
                    }
                }
            }
        }
        if order.len() == n {
            Some(order)
        } else {
            None
        }
    }
    /// Returns the number of nodes.
    pub fn num_nodes(&self) -> usize {
        self.edges.len()
    }
}
/// A window iterator that yields overlapping windows of size `n`.
#[allow(dead_code)]
pub struct WindowIterator<'a, T> {
    pub(super) data: &'a [T],
    pub(super) pos: usize,
    pub(super) window: usize,
}
#[allow(dead_code)]
impl<'a, T> WindowIterator<'a, T> {
    /// Creates a new window iterator.
    pub fn new(data: &'a [T], window: usize) -> Self {
        Self {
            data,
            pos: 0,
            window,
        }
    }
}
/// Environment extension for inductive types.
pub struct InductiveEnv {
    /// Map from type name to inductive declaration
    inductives: HashMap<Name, InductiveType>,
    /// Map from constructor name to parent type
    constructors: HashMap<Name, Name>,
    /// Map from recursor name to parent type
    recursors: HashMap<Name, Name>,
}
impl InductiveEnv {
    /// Create a new empty inductive environment.
    pub fn new() -> Self {
        Self {
            inductives: HashMap::new(),
            constructors: HashMap::new(),
            recursors: HashMap::new(),
        }
    }
    /// Add an inductive type to the environment.
    #[allow(clippy::result_large_err)]
    pub fn add(&mut self, ind: InductiveType) -> Result<(), KernelError> {
        if self.inductives.contains_key(&ind.name) {
            return Err(KernelError::Other(format!(
                "inductive type already declared: {}",
                ind.name
            )));
        }
        for intro in &ind.intro_rules {
            if self.constructors.contains_key(&intro.name) {
                return Err(KernelError::Other(format!(
                    "constructor already declared: {}",
                    intro.name
                )));
            }
            self.constructors
                .insert(intro.name.clone(), ind.name.clone());
        }
        self.recursors
            .insert(ind.recursor.clone(), ind.name.clone());
        self.inductives.insert(ind.name.clone(), ind);
        Ok(())
    }
    /// Get an inductive type by name.
    pub fn get(&self, name: &Name) -> Option<&InductiveType> {
        self.inductives.get(name)
    }
    /// Check if a name is a constructor.
    pub fn is_constructor(&self, name: &Name) -> bool {
        self.constructors.contains_key(name)
    }
    /// Get the parent type of a constructor.
    pub fn get_constructor_parent(&self, name: &Name) -> Option<&Name> {
        self.constructors.get(name)
    }
    /// Check if a name is a recursor.
    pub fn is_recursor(&self, name: &Name) -> bool {
        self.recursors.contains_key(name)
    }
    /// Get the parent type of a recursor.
    pub fn get_recursor_parent(&self, name: &Name) -> Option<&Name> {
        self.recursors.get(name)
    }
    /// Register an InductiveType into an Environment via ConstantInfo.
    #[allow(clippy::result_large_err)]
    pub fn register_in_env(
        &mut self,
        ind: &InductiveType,
        env: &mut crate::Environment,
    ) -> Result<(), KernelError> {
        let (ind_ci, ctor_cis, rec_ci) = ind.to_constant_infos();
        env.add_constant(ind_ci)
            .map_err(|e| KernelError::Other(e.to_string()))?;
        for ctor_ci in ctor_cis {
            env.add_constant(ctor_ci)
                .map_err(|e| KernelError::Other(e.to_string()))?;
        }
        env.add_constant(rec_ci)
            .map_err(|e| KernelError::Other(e.to_string()))?;
        self.add(ind.clone())?;
        Ok(())
    }
}
/// A non-empty list (at least one element guaranteed).
#[allow(dead_code)]
pub struct NonEmptyVec<T> {
    head: T,
    tail: Vec<T>,
}
#[allow(dead_code)]
impl<T> NonEmptyVec<T> {
    /// Creates a non-empty vec with a single element.
    pub fn singleton(val: T) -> Self {
        Self {
            head: val,
            tail: Vec::new(),
        }
    }
    /// Pushes an element.
    pub fn push(&mut self, val: T) {
        self.tail.push(val);
    }
    /// Returns a reference to the first element.
    pub fn first(&self) -> &T {
        &self.head
    }
    /// Returns a reference to the last element.
    pub fn last(&self) -> &T {
        self.tail.last().unwrap_or(&self.head)
    }
    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        1 + self.tail.len()
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns all elements as a Vec.
    pub fn to_vec(&self) -> Vec<&T> {
        let mut v = vec![&self.head];
        v.extend(self.tail.iter());
        v
    }
}
/// A pair of `StatSummary` values tracking before/after a transformation.
#[allow(dead_code)]
pub struct TransformStat {
    before: StatSummary,
    after: StatSummary,
}
#[allow(dead_code)]
impl TransformStat {
    /// Creates a new transform stat recorder.
    pub fn new() -> Self {
        Self {
            before: StatSummary::new(),
            after: StatSummary::new(),
        }
    }
    /// Records a before value.
    pub fn record_before(&mut self, v: f64) {
        self.before.record(v);
    }
    /// Records an after value.
    pub fn record_after(&mut self, v: f64) {
        self.after.record(v);
    }
    /// Returns the mean reduction ratio (after/before).
    pub fn mean_ratio(&self) -> Option<f64> {
        let b = self.before.mean()?;
        let a = self.after.mean()?;
        if b.abs() < f64::EPSILON {
            return None;
        }
        Some(a / b)
    }
}
/// A label set for a graph node.
#[allow(dead_code)]
pub struct LabelSet {
    labels: Vec<String>,
}
#[allow(dead_code)]
impl LabelSet {
    /// Creates a new empty label set.
    pub fn new() -> Self {
        Self { labels: Vec::new() }
    }
    /// Adds a label (deduplicates).
    pub fn add(&mut self, label: impl Into<String>) {
        let s = label.into();
        if !self.labels.contains(&s) {
            self.labels.push(s);
        }
    }
    /// Returns `true` if `label` is present.
    pub fn has(&self, label: &str) -> bool {
        self.labels.iter().any(|l| l == label)
    }
    /// Returns the count of labels.
    pub fn count(&self) -> usize {
        self.labels.len()
    }
    /// Returns all labels.
    pub fn all(&self) -> &[String] {
        &self.labels
    }
}
/// A versioned record that stores a history of values.
#[allow(dead_code)]
pub struct VersionedRecord<T: Clone> {
    history: Vec<T>,
}
#[allow(dead_code)]
impl<T: Clone> VersionedRecord<T> {
    /// Creates a new record with an initial value.
    pub fn new(initial: T) -> Self {
        Self {
            history: vec![initial],
        }
    }
    /// Updates the record with a new version.
    pub fn update(&mut self, val: T) {
        self.history.push(val);
    }
    /// Returns the current (latest) value.
    pub fn current(&self) -> &T {
        self.history
            .last()
            .expect("VersionedRecord history is always non-empty after construction")
    }
    /// Returns the value at version `n` (0-indexed), or `None`.
    pub fn at_version(&self, n: usize) -> Option<&T> {
        self.history.get(n)
    }
    /// Returns the version number of the current value.
    pub fn version(&self) -> usize {
        self.history.len() - 1
    }
    /// Returns `true` if more than one version exists.
    pub fn has_history(&self) -> bool {
        self.history.len() > 1
    }
}
/// An inductive type declaration.
#[derive(Clone, Debug, PartialEq)]
pub struct InductiveType {
    /// Type name
    pub name: Name,
    /// Universe parameters
    pub univ_params: Vec<Name>,
    /// Number of parameters (non-varying arguments)
    pub num_params: u32,
    /// Number of indices (varying arguments)
    pub num_indices: u32,
    /// Type of the inductive type (Pi type over params and indices)
    pub ty: Expr,
    /// Introduction rules (constructors)
    pub intro_rules: Vec<IntroRule>,
    /// Recursor name (generated automatically)
    pub recursor: Name,
    /// Whether this is a nested inductive type
    pub is_nested: bool,
    /// Whether this type is in Prop (proof-irrelevant)
    pub is_prop: bool,
}
impl InductiveType {
    /// Create a new inductive type.
    pub fn new(
        name: Name,
        univ_params: Vec<Name>,
        num_params: u32,
        num_indices: u32,
        ty: Expr,
        intro_rules: Vec<IntroRule>,
    ) -> Self {
        let recursor = Name::Str(Box::new(name.clone()), "rec".to_string());
        Self {
            name,
            univ_params,
            num_params,
            num_indices,
            ty,
            intro_rules,
            recursor,
            is_nested: false,
            is_prop: false,
        }
    }
    /// Get the arity (total number of arguments).
    pub fn arity(&self) -> u32 {
        self.num_params + self.num_indices
    }
    /// Check if this inductive type is recursive.
    pub fn is_recursive(&self) -> bool {
        self.intro_rules
            .iter()
            .any(|rule| self.occurs_in_arguments(&self.name, &rule.ty))
    }
    /// Return a vector of references to constructor names.
    pub fn constructor_names(&self) -> Vec<&Name> {
        self.intro_rules.iter().map(|r| &r.name).collect()
    }
    /// Return the number of constructors.
    pub fn num_constructors(&self) -> usize {
        self.intro_rules.len()
    }
    fn occurs_in_arguments(&self, name: &Name, ty: &Expr) -> bool {
        match ty {
            Expr::Pi(_, _, dom, cod) => {
                self.occurs_in_type(name, dom) || self.occurs_in_arguments(name, cod)
            }
            _ => false,
        }
    }
    fn occurs_in_type(&self, name: &Name, ty: &Expr) -> bool {
        match ty {
            Expr::Const(n, _) => n == name,
            Expr::App(f, a) => self.occurs_in_type(name, f) || self.occurs_in_type(name, a),
            Expr::Pi(_, _, dom, cod) => {
                self.occurs_in_type(name, dom) || self.occurs_in_type(name, cod)
            }
            Expr::Lam(_, _, ty_inner, body) => {
                self.occurs_in_type(name, ty_inner) || self.occurs_in_type(name, body)
            }
            _ => false,
        }
    }
    /// Generate ConstantInfo declarations for this inductive type.
    ///
    /// Returns: (InductiveVal, `Vec<ConstructorVal>`, RecursorVal)
    pub fn to_constant_infos(&self) -> (ConstantInfo, Vec<ConstantInfo>, ConstantInfo) {
        let ind_val = self.make_inductive_val();
        let ctor_vals: Vec<ConstantInfo> = self.make_constructor_vals();
        let rec_val = self.make_recursor_val();
        (ind_val, ctor_vals, rec_val)
    }
    fn make_inductive_val(&self) -> ConstantInfo {
        ConstantInfo::Inductive(InductiveVal {
            common: ConstantVal {
                name: self.name.clone(),
                level_params: self.univ_params.clone(),
                ty: self.ty.clone(),
            },
            num_params: self.num_params,
            num_indices: self.num_indices,
            all: vec![self.name.clone()],
            ctors: self.intro_rules.iter().map(|r| r.name.clone()).collect(),
            num_nested: 0,
            is_rec: self.is_recursive(),
            is_unsafe: false,
            is_reflexive: false,
            is_prop: self.is_prop,
        })
    }
    fn make_constructor_vals(&self) -> Vec<ConstantInfo> {
        self.intro_rules
            .iter()
            .enumerate()
            .map(|(i, rule)| {
                let num_fields = count_pi_args(&rule.ty).saturating_sub(self.num_params);
                ConstantInfo::Constructor(ConstructorVal {
                    common: ConstantVal {
                        name: rule.name.clone(),
                        level_params: self.univ_params.clone(),
                        ty: rule.ty.clone(),
                    },
                    induct: self.name.clone(),
                    cidx: i as u32,
                    num_params: self.num_params,
                    num_fields,
                    is_unsafe: false,
                })
            })
            .collect()
    }
    fn make_recursor_val(&self) -> ConstantInfo {
        let num_minors = self.intro_rules.len() as u32;
        let rules: Vec<RecursorRule> = self
            .intro_rules
            .iter()
            .enumerate()
            .map(|(cidx, rule)| {
                let nfields = count_pi_args(&rule.ty).saturating_sub(self.num_params);
                let rhs = self.build_recursor_rhs(rule, cidx as u32, nfields, num_minors);
                RecursorRule {
                    ctor: rule.name.clone(),
                    nfields,
                    rhs,
                }
            })
            .collect();
        let k = self.is_prop && self.intro_rules.len() <= 1;
        let mut rec_level_params = self.univ_params.clone();
        if !self.is_prop {
            rec_level_params.insert(0, Name::str("u_1"));
        }
        let rec_ty = self.build_recursor_type(&rec_level_params);
        ConstantInfo::Recursor(RecursorVal {
            common: ConstantVal {
                name: self.recursor.clone(),
                level_params: rec_level_params,
                ty: rec_ty,
            },
            all: vec![self.name.clone()],
            num_params: self.num_params,
            num_indices: self.num_indices,
            num_motives: 1,
            num_minors,
            rules,
            k,
            is_unsafe: false,
        })
    }
    /// Collect the field types (domains) for a constructor, skipping num_params params.
    /// Returns a Vec<(field_domain_Expr, is_recursive)>.
    fn collect_field_info(&self, rule: &IntroRule) -> Vec<(Expr, bool)> {
        let mut current = &rule.ty;
        for _ in 0..self.num_params {
            match current {
                Expr::Pi(_, _, _, body) => current = body,
                _ => return vec![],
            }
        }
        let mut fields = Vec::new();
        while let Expr::Pi(_, _, dom, body) = current {
            let is_rec = self.head_is_inductive(dom);
            fields.push((dom.as_ref().clone(), is_rec));
            current = body;
        }
        fields
    }
    /// Check if the head of an expression is the inductive type constant.
    fn head_is_inductive(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Const(n, _) => n == &self.name,
            Expr::App(f, _) => self.head_is_inductive(f),
            _ => false,
        }
    }
    /// Build the recursor type Pi-expression.
    ///
    /// The recursor type has the form:
    ///   forall params... motive minor_0... minor_{m-1} indices... (major : T p i), motive i major
    fn build_recursor_type(&self, rec_level_params: &[Name]) -> Expr {
        use crate::BinderInfo;
        let np = self.num_params as usize;
        let ni = self.num_indices as usize;
        let nminors = self.intro_rules.len();
        let (all_binders, _result_sort) = peel_pi_binders(&self.ty);
        let motive_sort = if self.is_prop {
            Level::zero()
        } else if let Some(name) = rec_level_params.first() {
            Level::Param(name.clone())
        } else {
            Level::zero()
        };
        let ind_applied_major = {
            let mut e: Expr = Expr::Const(self.name.clone(), vec![]);
            for k in 0..np {
                let bvar = Expr::BVar((ni + nminors + 1 + np - k) as u32);
                e = Expr::App(Box::new(e), Box::new(bvar));
            }
            for k in 0..ni {
                let bvar = Expr::BVar((ni - k) as u32);
                e = Expr::App(Box::new(e), Box::new(bvar));
            }
            e
        };
        let conclusion = {
            let mut e: Expr = Expr::BVar((1 + ni + nminors) as u32);
            for k in 0..ni {
                let bvar = Expr::BVar((ni - k) as u32);
                e = Expr::App(Box::new(e), Box::new(bvar));
            }
            e = Expr::App(Box::new(e), Box::new(Expr::BVar(0)));
            e
        };
        let mut result = Expr::Pi(
            BinderInfo::Default,
            Name::str("t"),
            Box::new(ind_applied_major),
            Box::new(conclusion),
        );
        for k in (0..ni).rev() {
            let idx_ty = if np + k < all_binders.len() {
                lift_expr_bvars(&all_binders[np + k], (1 + nminors) as u32)
            } else {
                Expr::Sort(Level::zero())
            };
            result = Expr::Pi(
                BinderInfo::Default,
                Name::str("i"),
                Box::new(idx_ty),
                Box::new(result),
            );
        }
        for cidx in (0..nminors).rev() {
            let minor_ty = self.build_minor_type(cidx, nminors, np, ni, &all_binders);
            result = Expr::Pi(
                BinderInfo::Default,
                Name::str("minor"),
                Box::new(minor_ty),
                Box::new(result),
            );
        }
        let motive_ty = self.build_motive_type(np, ni, &all_binders, motive_sort);
        result = Expr::Pi(
            BinderInfo::Default,
            Name::str("motive"),
            Box::new(motive_ty),
            Box::new(result),
        );
        for k in (0..np).rev() {
            let param_ty = if k < all_binders.len() {
                all_binders[k].clone()
            } else {
                Expr::Sort(Level::succ(Level::zero()))
            };
            result = Expr::Pi(
                BinderInfo::Default,
                Name::str("param"),
                Box::new(param_ty),
                Box::new(result),
            );
        }
        result
    }
    /// Build the motive type at depth np (inside np param binders):
    ///   forall (i_0 : I_0)...(i_{ni-1} : I_{ni-1}), T p i -> Sort v
    fn build_motive_type(
        &self,
        np: usize,
        ni: usize,
        all_binders: &[Expr],
        motive_sort: Level,
    ) -> Expr {
        use crate::BinderInfo;
        let ind_applied = {
            let mut e: Expr = Expr::Const(self.name.clone(), vec![]);
            for k in 0..np {
                let bvar = Expr::BVar((np + ni - 1 - k) as u32);
                e = Expr::App(Box::new(e), Box::new(bvar));
            }
            for k in 0..ni {
                let bvar = Expr::BVar((ni - 1 - k) as u32);
                e = Expr::App(Box::new(e), Box::new(bvar));
            }
            e
        };
        let inner = Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ind_applied),
            Box::new(Expr::Sort(motive_sort)),
        );
        let mut result = inner;
        for k in (0..ni).rev() {
            let idx_ty = if np + k < all_binders.len() {
                all_binders[np + k].clone()
            } else {
                Expr::Sort(Level::zero())
            };
            result = Expr::Pi(
                BinderInfo::Default,
                Name::str("i"),
                Box::new(idx_ty),
                Box::new(result),
            );
        }
        result
    }
    /// Build the minor premise type for constructor at `cidx`.
    ///
    /// The minor type for constructor `cidx` (with `nf` fields) is built in the context:
    ///   np param binders + 1 motive binder + cidx outer minor binders (already wrapped)
    ///
    /// The resulting type is a Pi chain over the field types ending with:
    ///   motive [return_indices] (ctor p_0 ... p_{np-1} y_0 ... y_{nf-1})
    ///
    /// At the body (depth = nf inside the field Pi chain):
    ///   y_j     = BVar(nf - 1 - j)        for j = 0..nf-1
    ///   motive  = BVar(nf + cidx)
    ///   p_k     = BVar(nf + cidx + np - k) for k = 0..np-1
    fn build_minor_type(
        &self,
        cidx: usize,
        _nminors: usize,
        np: usize,
        ni: usize,
        _all_binders: &[Expr],
    ) -> Expr {
        use crate::BinderInfo;
        let rule = &self.intro_rules[cidx];
        let field_info = self.collect_field_info(rule);
        let nf = field_info.len();
        let return_indices: Vec<Expr> = if ni > 0 {
            let mut cod: &Expr = &rule.ty;
            for _ in 0..(np + nf) {
                match cod {
                    Expr::Pi(_, _, _, body) => cod = body,
                    _ => break,
                }
            }
            let mut args: Vec<Expr> = Vec::new();
            let mut cur = cod;
            while let Expr::App(f, a) = cur {
                args.push(a.as_ref().clone());
                cur = f;
            }
            args.reverse();
            let start = args.len().saturating_sub(ni);
            args[start..]
                .iter()
                .map(|e| lift_expr_bvars(e, (1 + cidx) as u32))
                .collect()
        } else {
            vec![]
        };
        let mut ctor_app: Expr = Expr::Const(rule.name.clone(), vec![]);
        for k in 0..np {
            let bvar = Expr::BVar((nf + cidx + np - k) as u32);
            ctor_app = Expr::App(Box::new(ctor_app), Box::new(bvar));
        }
        for j in 0..nf {
            let bvar = Expr::BVar((nf - 1 - j) as u32);
            ctor_app = Expr::App(Box::new(ctor_app), Box::new(bvar));
        }
        let mut conclusion: Expr = Expr::BVar((nf + cidx) as u32);
        for idx in &return_indices {
            conclusion = Expr::App(Box::new(conclusion), Box::new(idx.clone()));
        }
        conclusion = Expr::App(Box::new(conclusion), Box::new(ctor_app));
        let mut result = conclusion;
        for j in (0..nf).rev() {
            let (field_ty, _is_rec) = &field_info[j];
            let adjusted = lift_expr_bvars(field_ty, (1 + cidx) as u32);
            result = Expr::Pi(
                BinderInfo::Default,
                Name::str("y"),
                Box::new(adjusted),
                Box::new(result),
            );
        }
        result
    }
    /// Build the recursor rule RHS for a constructor.
    ///
    /// The `instantiate_recursor_rhs` function builds a substitution array:
    ///   subst = [p_0, ..., p_{np-1}, motive, minor_0, ..., minor_{nm-1}, f_0, ..., f_{nf-1}]
    ///   len = np + 1 + nm + nf
    ///
    /// `instantiate_rev` maps BVar(i) -> subst[len - 1 - i]:
    ///   f_j     -> BVar(nf - 1 - j)         (j = 0..nf-1)
    ///   minor_k -> BVar(nf + nm - 1 - k)    (k = 0..nm-1)
    ///   motive  -> BVar(nf + nm)
    ///   p_j     -> BVar(nf + nm + 1 + np - 1 - j) = BVar(nf + nm + np - j)
    ///
    /// The RHS is: minor_cidx f_0 [IH_0] f_1 [IH_1] ... f_{nf-1} [IH_{nf-1}]
    /// where IH_j = rec_name p_0...p_{np-1} motive minor_0...minor_{nm-1} f_j
    fn build_recursor_rhs(&self, rule: &IntroRule, cidx: u32, nfields: u32, nminors: u32) -> Expr {
        let np = self.num_params;
        let nf = nfields;
        let nm = nminors;
        let field_info = self.collect_field_info(rule);
        let mut result = Expr::BVar(nf + nm - 1 - cidx);
        for (j, (_field_ty, is_rec)) in field_info.iter().enumerate() {
            let j = j as u32;
            let field_bvar = Expr::BVar(nf - 1 - j);
            result = Expr::App(Box::new(result), Box::new(field_bvar.clone()));
            if *is_rec {
                let ih = self.build_ih(field_bvar, np, nm, nf);
                result = Expr::App(Box::new(result), Box::new(ih));
            }
        }
        result
    }
    /// Build the inductive hypothesis for a recursive field:
    ///   IH = rec_name p_0...p_{np-1} motive minor_0...minor_{nm-1} field_expr
    fn build_ih(&self, field_expr: Expr, np: u32, nm: u32, nf: u32) -> Expr {
        let mut ih = Expr::Const(self.recursor.clone(), vec![]);
        for j in 0..np {
            let bvar = Expr::BVar(nf + nm + np - j);
            ih = Expr::App(Box::new(ih), Box::new(bvar));
        }
        ih = Expr::App(Box::new(ih), Box::new(Expr::BVar(nf + nm)));
        for k in 0..nm {
            let bvar = Expr::BVar(nf + nm - 1 - k);
            ih = Expr::App(Box::new(ih), Box::new(bvar));
        }
        ih = Expr::App(Box::new(ih), Box::new(field_expr));
        ih
    }
}
/// Summary information about an inductive type (for display/LSP use).
#[derive(Clone, Debug)]
pub struct InductiveTypeInfo {
    /// Type name.
    pub name: Name,
    /// Number of parameters.
    pub num_params: u32,
    /// Number of indices.
    pub num_indices: u32,
    /// Number of constructors.
    pub num_constructors: usize,
    /// Whether the type is in Prop.
    pub is_prop: bool,
    /// Whether the type is recursive (has recursive constructors).
    pub is_recursive: bool,
    /// Whether the type is mutually inductive.
    pub is_mutual: bool,
    /// Constructor names.
    pub constructor_names: Vec<Name>,
}
impl InductiveTypeInfo {
    /// Build info from an `InductiveType`.
    pub fn from_type(ty: &InductiveType, is_mutual: bool) -> Self {
        Self {
            name: ty.name.clone(),
            num_params: ty.num_params,
            num_indices: ty.num_indices,
            num_constructors: ty.intro_rules.len(),
            is_prop: ty.is_prop,
            is_recursive: ty.is_recursive(),
            is_mutual,
            constructor_names: ty.intro_rules.iter().map(|r| r.name.clone()).collect(),
        }
    }
    /// Human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "{}: {} params, {} indices, {} ctors, prop={}, recursive={}, mutual={}",
            self.name,
            self.num_params,
            self.num_indices,
            self.num_constructors,
            self.is_prop,
            self.is_recursive,
            self.is_mutual
        )
    }
}
/// A mutable reference stack for tracking the current "focus" in a tree traversal.
#[allow(dead_code)]
pub struct FocusStack<T> {
    items: Vec<T>,
}
#[allow(dead_code)]
impl<T> FocusStack<T> {
    /// Creates an empty focus stack.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    /// Focuses on `item`.
    pub fn focus(&mut self, item: T) {
        self.items.push(item);
    }
    /// Blurs (pops) the current focus.
    pub fn blur(&mut self) -> Option<T> {
        self.items.pop()
    }
    /// Returns the current focus, or `None`.
    pub fn current(&self) -> Option<&T> {
        self.items.last()
    }
    /// Returns the focus depth.
    pub fn depth(&self) -> usize {
        self.items.len()
    }
    /// Returns `true` if there is no current focus.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
/// Builder for constructing `InductiveType` declarations incrementally.
#[allow(dead_code)]
pub struct InductiveTypeBuilder {
    name: Option<Name>,
    univ_params: Vec<Name>,
    num_params: u32,
    num_indices: u32,
    ty: Option<Expr>,
    intro_rules: Vec<IntroRule>,
    is_prop: bool,
    is_nested: bool,
}
#[allow(dead_code)]
impl InductiveTypeBuilder {
    /// Create an empty builder.
    pub fn new() -> Self {
        Self {
            name: None,
            univ_params: vec![],
            num_params: 0,
            num_indices: 0,
            ty: None,
            intro_rules: vec![],
            is_prop: false,
            is_nested: false,
        }
    }
    /// Set the inductive type name.
    pub fn name(mut self, name: Name) -> Self {
        self.name = Some(name);
        self
    }
    /// Set universe parameters.
    pub fn univ_params(mut self, params: Vec<Name>) -> Self {
        self.univ_params = params;
        self
    }
    /// Set number of type parameters.
    pub fn num_params(mut self, n: u32) -> Self {
        self.num_params = n;
        self
    }
    /// Set number of type indices.
    pub fn num_indices(mut self, n: u32) -> Self {
        self.num_indices = n;
        self
    }
    /// Set the sort of the inductive type.
    pub fn ty(mut self, ty: Expr) -> Self {
        self.ty = Some(ty);
        self
    }
    /// Add a constructor/introduction rule.
    pub fn intro_rule(mut self, name: Name, ty: Expr) -> Self {
        self.intro_rules.push(IntroRule { name, ty });
        self
    }
    /// Mark as a Prop-valued inductive type (proof-irrelevant).
    pub fn is_prop(mut self, v: bool) -> Self {
        self.is_prop = v;
        self
    }
    /// Mark as a nested inductive type.
    pub fn is_nested(mut self, v: bool) -> Self {
        self.is_nested = v;
        self
    }
    /// Build the `InductiveType`. Returns `Err` if name or ty is missing.
    pub fn build(self) -> Result<InductiveType, String> {
        let name = self
            .name
            .ok_or_else(|| "InductiveTypeBuilder: name not set".to_string())?;
        let ty = self
            .ty
            .ok_or_else(|| "InductiveTypeBuilder: ty not set".to_string())?;
        let mut ind = InductiveType::new(
            name,
            self.univ_params,
            self.num_params,
            self.num_indices,
            ty,
            self.intro_rules,
        );
        ind.is_prop = self.is_prop;
        ind.is_nested = self.is_nested;
        Ok(ind)
    }
}
/// A builder for constructing recursor definitions programmatically.
///
/// Recursors (eliminators) are the fundamental way to consume inductive types.
/// This builder provides a fluent API for constructing them.
#[derive(Clone, Debug)]
pub struct RecursorBuilder {
    /// Name of the recursor (typically `T.rec` or `T.recOn`).
    pub name: Name,
    /// Universe parameters.
    pub univ_params: Vec<Name>,
    /// Number of type parameters.
    pub num_params: u32,
    /// Number of indices.
    pub num_indices: u32,
    /// Number of motives (typically 1).
    pub num_motives: u32,
    /// Number of minor premises (one per constructor).
    pub num_minor_premises: u32,
    /// Recursor rules (one per constructor).
    pub rules: Vec<RecursorRule>,
    /// Whether the recursor targets `Prop`.
    pub is_prop: bool,
}
impl RecursorBuilder {
    /// Create a new builder.
    pub fn new(name: Name) -> Self {
        Self {
            name,
            univ_params: vec![],
            num_params: 0,
            num_indices: 0,
            num_motives: 1,
            num_minor_premises: 0,
            rules: vec![],
            is_prop: false,
        }
    }
    /// Set universe parameters.
    pub fn univ_params(mut self, params: Vec<Name>) -> Self {
        self.univ_params = params;
        self
    }
    /// Set the number of parameters.
    pub fn num_params(mut self, n: u32) -> Self {
        self.num_params = n;
        self
    }
    /// Set the number of indices.
    pub fn num_indices(mut self, n: u32) -> Self {
        self.num_indices = n;
        self
    }
    /// Set whether the recursor targets Prop.
    pub fn is_prop(mut self, b: bool) -> Self {
        self.is_prop = b;
        self
    }
    /// Add a recursor rule.
    pub fn add_rule(mut self, rule: RecursorRule) -> Self {
        self.rules.push(rule);
        self.num_minor_premises += 1;
        self
    }
    /// Validate the builder state.
    ///
    /// Returns `Ok(())` if the builder is consistent.
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_anonymous() {
            return Err("recursor name must not be anonymous".to_string());
        }
        if self.num_motives == 0 {
            return Err("recursor must have at least one motive".to_string());
        }
        Ok(())
    }
    /// Return the name of the recursor.
    pub fn build_name(&self) -> Name {
        self.name.clone()
    }
    /// Build a complete [`RecursorVal`] from the builder state.
    ///
    /// `ty` is the type of the recursor (must be supplied by the caller since
    /// this builder does not reconstruct the Pi telescope).
    /// `all` is the list of inductive type names in the mutual family.
    pub fn build(&self, ty: Expr, all: Vec<Name>) -> Result<RecursorVal, String> {
        self.validate()?;
        if self.rules.len() != self.num_minor_premises as usize {
            return Err(format!(
                "RecursorBuilder: num_minor_premises ({}) does not match rule count ({})",
                self.num_minor_premises,
                self.rules.len()
            ));
        }
        Ok(RecursorVal {
            common: ConstantVal {
                name: self.name.clone(),
                level_params: self.univ_params.clone(),
                ty,
            },
            all,
            num_params: self.num_params,
            num_indices: self.num_indices,
            num_motives: self.num_motives,
            num_minors: self.num_minor_premises,
            rules: self.rules.clone(),
            k: false,
            is_unsafe: false,
        })
    }
}
