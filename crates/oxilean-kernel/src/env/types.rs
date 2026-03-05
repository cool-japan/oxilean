//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::declaration::{ConstantInfo, ConstantVal, DefinitionSafety, DefinitionVal};
use crate::reduce::ReducibilityHint;
use crate::{Expr, Level, Name};
use std::collections::HashMap;

/// Represents a rewrite rule `lhs → rhs`.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct RewriteRule {
    /// The name of the rule.
    pub name: String,
    /// A string representation of the LHS pattern.
    pub lhs: String,
    /// A string representation of the RHS.
    pub rhs: String,
    /// Whether this is a conditional rule (has side conditions).
    pub conditional: bool,
}
#[allow(dead_code)]
impl RewriteRule {
    /// Creates an unconditional rewrite rule.
    pub fn unconditional(
        name: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
            conditional: false,
        }
    }
    /// Creates a conditional rewrite rule.
    pub fn conditional(
        name: impl Into<String>,
        lhs: impl Into<String>,
        rhs: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            lhs: lhs.into(),
            rhs: rhs.into(),
            conditional: true,
        }
    }
    /// Returns a textual representation.
    pub fn display(&self) -> String {
        format!("{}: {} → {}", self.name, self.lhs, self.rhs)
    }
}
/// A set of rewrite rules.
#[allow(dead_code)]
pub struct RewriteRuleSet {
    rules: Vec<RewriteRule>,
}
#[allow(dead_code)]
impl RewriteRuleSet {
    /// Creates an empty rule set.
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }
    /// Adds a rule.
    pub fn add(&mut self, rule: RewriteRule) {
        self.rules.push(rule);
    }
    /// Returns the number of rules.
    pub fn len(&self) -> usize {
        self.rules.len()
    }
    /// Returns `true` if the set is empty.
    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
    /// Returns all conditional rules.
    pub fn conditional_rules(&self) -> Vec<&RewriteRule> {
        self.rules.iter().filter(|r| r.conditional).collect()
    }
    /// Returns all unconditional rules.
    pub fn unconditional_rules(&self) -> Vec<&RewriteRule> {
        self.rules.iter().filter(|r| !r.conditional).collect()
    }
    /// Looks up a rule by name.
    pub fn get(&self, name: &str) -> Option<&RewriteRule> {
        self.rules.iter().find(|r| r.name == name)
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
/// A sparse vector: stores only non-default elements.
#[allow(dead_code)]
pub struct SparseVec<T: Default + Clone + PartialEq> {
    entries: std::collections::HashMap<usize, T>,
    default_: T,
    logical_len: usize,
}
#[allow(dead_code)]
impl<T: Default + Clone + PartialEq> SparseVec<T> {
    /// Creates a new sparse vector with logical length `len`.
    pub fn new(len: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            default_: T::default(),
            logical_len: len,
        }
    }
    /// Sets element at `idx`.
    pub fn set(&mut self, idx: usize, val: T) {
        if val == self.default_ {
            self.entries.remove(&idx);
        } else {
            self.entries.insert(idx, val);
        }
    }
    /// Gets element at `idx`.
    pub fn get(&self, idx: usize) -> &T {
        self.entries.get(&idx).unwrap_or(&self.default_)
    }
    /// Returns the logical length.
    pub fn len(&self) -> usize {
        self.logical_len
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Returns the number of non-default elements.
    pub fn nnz(&self) -> usize {
        self.entries.len()
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
/// The global environment containing all checked declarations.
#[derive(Clone, Debug, Default)]
pub struct Environment {
    /// All declarations indexed by name (legacy format).
    declarations: HashMap<Name, Declaration>,
    /// All constants indexed by name (new LEAN 4-style format).
    constants: HashMap<Name, ConstantInfo>,
}
impl Environment {
    /// Create a new empty environment.
    pub fn new() -> Self {
        Self {
            declarations: HashMap::new(),
            constants: HashMap::new(),
        }
    }
    /// Add a legacy declaration to the environment.
    ///
    /// Returns an error if a declaration with the same name already exists.
    pub fn add(&mut self, decl: Declaration) -> Result<(), EnvError> {
        let name = decl.name().clone();
        if self.declarations.contains_key(&name) || self.constants.contains_key(&name) {
            return Err(EnvError::DuplicateDeclaration(name));
        }
        let ci = decl.to_constant_info();
        self.constants.insert(name.clone(), ci);
        self.declarations.insert(name, decl);
        Ok(())
    }
    /// Add a ConstantInfo to the environment.
    pub fn add_constant(&mut self, ci: ConstantInfo) -> Result<(), EnvError> {
        let name = ci.name().clone();
        if self.declarations.contains_key(&name) || self.constants.contains_key(&name) {
            return Err(EnvError::DuplicateDeclaration(name));
        }
        self.constants.insert(name, ci);
        Ok(())
    }
    /// Look up a legacy declaration by name.
    pub fn get(&self, name: &Name) -> Option<&Declaration> {
        self.declarations.get(name)
    }
    /// Look up a constant info by name.
    pub fn find(&self, name: &Name) -> Option<&ConstantInfo> {
        self.constants.get(name)
    }
    /// Check if a declaration exists.
    pub fn contains(&self, name: &Name) -> bool {
        self.declarations.contains_key(name) || self.constants.contains_key(name)
    }
    /// Get the number of declarations.
    pub fn len(&self) -> usize {
        self.constants.len().max(self.declarations.len())
    }
    /// Check if the environment is empty.
    pub fn is_empty(&self) -> bool {
        self.declarations.is_empty() && self.constants.is_empty()
    }
    /// Get a definition's value and hint (for reduction).
    pub fn get_defn(&self, name: &Name) -> Option<(Expr, ReducibilityHint)> {
        if let Some(ci) = self.constants.get(name) {
            if let Some(val) = ci.value() {
                return Some((val.clone(), ci.reducibility_hint()));
            }
        }
        self.declarations.get(name).and_then(|decl| {
            decl.value()
                .map(|val| (val.clone(), decl.reducibility_hint()))
        })
    }
    /// Check if a name refers to an inductive type.
    pub fn is_inductive(&self, name: &Name) -> bool {
        self.constants.get(name).is_some_and(|ci| ci.is_inductive())
    }
    /// Check if a name refers to a constructor.
    pub fn is_constructor(&self, name: &Name) -> bool {
        self.constants
            .get(name)
            .is_some_and(|ci| ci.is_constructor())
    }
    /// Check if a name refers to a recursor.
    pub fn is_recursor(&self, name: &Name) -> bool {
        self.constants.get(name).is_some_and(|ci| ci.is_recursor())
    }
    /// Check if a name refers to a quotient.
    pub fn is_quotient(&self, name: &Name) -> bool {
        self.constants.get(name).is_some_and(|ci| ci.is_quotient())
    }
    /// Check if a name refers to a structure-like inductive.
    pub fn is_structure_like(&self, name: &Name) -> bool {
        self.constants
            .get(name)
            .is_some_and(|ci| ci.is_structure_like())
    }
    /// Get the inductive type info for a name.
    pub fn get_inductive_val(&self, name: &Name) -> Option<&crate::declaration::InductiveVal> {
        self.constants
            .get(name)
            .and_then(|ci| ci.to_inductive_val())
    }
    /// Get the constructor info for a name.
    pub fn get_constructor_val(&self, name: &Name) -> Option<&crate::declaration::ConstructorVal> {
        self.constants
            .get(name)
            .and_then(|ci| ci.to_constructor_val())
    }
    /// Get the recursor info for a name.
    pub fn get_recursor_val(&self, name: &Name) -> Option<&crate::declaration::RecursorVal> {
        self.constants.get(name).and_then(|ci| ci.to_recursor_val())
    }
    /// Get the quotient info for a name.
    pub fn get_quotient_val(&self, name: &Name) -> Option<&crate::declaration::QuotVal> {
        self.constants.get(name).and_then(|ci| ci.to_quotient_val())
    }
    /// Get the type of a constant (from ConstantInfo).
    pub fn get_type(&self, name: &Name) -> Option<&Expr> {
        self.constants.get(name).map(|ci| ci.ty())
    }
    /// Get the universe level parameters for a constant.
    pub fn get_level_params(&self, name: &Name) -> Option<&[Name]> {
        self.constants.get(name).map(|ci| ci.level_params())
    }
    /// Instantiate a constant's type with universe levels.
    pub fn instantiate_const_type(&self, name: &Name, levels: &[Level]) -> Option<Expr> {
        let ci = self.constants.get(name)?;
        let params = ci.level_params();
        if params.is_empty() || levels.is_empty() {
            return Some(ci.ty().clone());
        }
        Some(crate::declaration::instantiate_level_params(
            ci.ty(),
            params,
            levels,
        ))
    }
    /// Iterate over all constant names.
    pub fn constant_names(&self) -> impl Iterator<Item = &Name> {
        self.constants.keys()
    }
    /// Iterate over all constant infos.
    pub fn constant_infos(&self) -> impl Iterator<Item = (&Name, &ConstantInfo)> {
        self.constants.iter()
    }
}
/// Counts of declaration kinds in an environment.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct EnvKindCounts {
    /// Number of axioms.
    pub axioms: usize,
    /// Number of inductive type definitions.
    pub inductives: usize,
    /// Number of constructors.
    pub constructors: usize,
    /// Number of recursors.
    pub recursors: usize,
    /// Number of definitions.
    pub definitions: usize,
    /// Number of theorems.
    pub theorems: usize,
    /// Other (quotients, etc.).
    pub other: usize,
}
impl EnvKindCounts {
    /// Total number of declarations.
    #[allow(dead_code)]
    pub fn total(&self) -> usize {
        self.axioms
            + self.inductives
            + self.constructors
            + self.recursors
            + self.definitions
            + self.theorems
            + self.other
    }
}
/// A declaration in the global environment (legacy format).
#[derive(Clone, Debug)]
pub enum Declaration {
    /// Axiom: postulated type without proof
    Axiom {
        /// Name of the axiom
        name: Name,
        /// Universe parameters
        univ_params: Vec<Name>,
        /// Type of the axiom
        ty: Expr,
    },
    /// Definition: has a computational value
    Definition {
        /// Name of the definition
        name: Name,
        /// Universe parameters
        univ_params: Vec<Name>,
        /// Type of the definition
        ty: Expr,
        /// Value/body of the definition
        val: Expr,
        /// Reducibility hint
        hint: ReducibilityHint,
    },
    /// Theorem: like definition but value is a proof (typically opaque)
    Theorem {
        /// Name of the theorem
        name: Name,
        /// Universe parameters
        univ_params: Vec<Name>,
        /// Type of the theorem (the proposition)
        ty: Expr,
        /// Proof of the theorem
        val: Expr,
    },
    /// Opaque: definition that should never be unfolded
    Opaque {
        /// Name of the opaque definition
        name: Name,
        /// Universe parameters
        univ_params: Vec<Name>,
        /// Type of the definition
        ty: Expr,
        /// Value of the definition (hidden)
        val: Expr,
    },
}
impl Declaration {
    /// Get the name of this declaration.
    pub fn name(&self) -> &Name {
        match self {
            Declaration::Axiom { name, .. }
            | Declaration::Definition { name, .. }
            | Declaration::Theorem { name, .. }
            | Declaration::Opaque { name, .. } => name,
        }
    }
    /// Get the type of this declaration.
    pub fn ty(&self) -> &Expr {
        match self {
            Declaration::Axiom { ty, .. }
            | Declaration::Definition { ty, .. }
            | Declaration::Theorem { ty, .. }
            | Declaration::Opaque { ty, .. } => ty,
        }
    }
    /// Get the universe parameters.
    pub fn univ_params(&self) -> &[Name] {
        match self {
            Declaration::Axiom { univ_params, .. }
            | Declaration::Definition { univ_params, .. }
            | Declaration::Theorem { univ_params, .. }
            | Declaration::Opaque { univ_params, .. } => univ_params,
        }
    }
    /// Get the value (if this is a definition/theorem/opaque).
    pub fn value(&self) -> Option<&Expr> {
        match self {
            Declaration::Axiom { .. } => None,
            Declaration::Definition { val, .. }
            | Declaration::Theorem { val, .. }
            | Declaration::Opaque { val, .. } => Some(val),
        }
    }
    /// Get the reducibility hint.
    pub fn reducibility_hint(&self) -> ReducibilityHint {
        match self {
            Declaration::Axiom { .. } => ReducibilityHint::Opaque,
            Declaration::Definition { hint, .. } => *hint,
            Declaration::Theorem { .. } | Declaration::Opaque { .. } => ReducibilityHint::Opaque,
        }
    }
    /// Convert a legacy Declaration to ConstantInfo.
    pub fn to_constant_info(&self) -> ConstantInfo {
        match self {
            Declaration::Axiom {
                name,
                univ_params,
                ty,
            } => ConstantInfo::Axiom(crate::declaration::AxiomVal {
                common: ConstantVal {
                    name: name.clone(),
                    level_params: univ_params.clone(),
                    ty: ty.clone(),
                },
                is_unsafe: false,
            }),
            Declaration::Definition {
                name,
                univ_params,
                ty,
                val,
                hint,
            } => ConstantInfo::Definition(DefinitionVal {
                common: ConstantVal {
                    name: name.clone(),
                    level_params: univ_params.clone(),
                    ty: ty.clone(),
                },
                value: val.clone(),
                hints: *hint,
                safety: DefinitionSafety::Safe,
                all: vec![name.clone()],
            }),
            Declaration::Theorem {
                name,
                univ_params,
                ty,
                val,
            } => ConstantInfo::Theorem(crate::declaration::TheoremVal {
                common: ConstantVal {
                    name: name.clone(),
                    level_params: univ_params.clone(),
                    ty: ty.clone(),
                },
                value: val.clone(),
                all: vec![name.clone()],
            }),
            Declaration::Opaque {
                name,
                univ_params,
                ty,
                val,
            } => ConstantInfo::Opaque(crate::declaration::OpaqueVal {
                common: ConstantVal {
                    name: name.clone(),
                    level_params: univ_params.clone(),
                    ty: ty.clone(),
                },
                value: val.clone(),
                is_unsafe: false,
                all: vec![name.clone()],
            }),
        }
    }
}
/// A type-erased function pointer with arity tracking.
#[allow(dead_code)]
pub struct RawFnPtr {
    /// The raw function pointer (stored as usize for type erasure).
    ptr: usize,
    arity: usize,
    name: String,
}
#[allow(dead_code)]
impl RawFnPtr {
    /// Creates a new raw function pointer descriptor.
    pub fn new(ptr: usize, arity: usize, name: impl Into<String>) -> Self {
        Self {
            ptr,
            arity,
            name: name.into(),
        }
    }
    /// Returns the arity.
    pub fn arity(&self) -> usize {
        self.arity
    }
    /// Returns the name.
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Returns the raw pointer value.
    pub fn raw(&self) -> usize {
        self.ptr
    }
}
/// A trie-based prefix counter.
#[allow(dead_code)]
pub struct PrefixCounter {
    children: std::collections::HashMap<char, PrefixCounter>,
    count: usize,
}
#[allow(dead_code)]
impl PrefixCounter {
    /// Creates an empty prefix counter.
    pub fn new() -> Self {
        Self {
            children: std::collections::HashMap::new(),
            count: 0,
        }
    }
    /// Records a string.
    pub fn record(&mut self, s: &str) {
        self.count += 1;
        let mut node = self;
        for c in s.chars() {
            node = node.children.entry(c).or_default();
            node.count += 1;
        }
    }
    /// Returns how many strings have been recorded that start with `prefix`.
    pub fn count_with_prefix(&self, prefix: &str) -> usize {
        let mut node = self;
        for c in prefix.chars() {
            match node.children.get(&c) {
                Some(n) => node = n,
                None => return 0,
            }
        }
        node.count
    }
}
/// A simple mutable key-value store for test fixtures.
#[allow(dead_code)]
pub struct Fixture {
    data: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl Fixture {
    /// Creates an empty fixture.
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }
    /// Sets a key.
    pub fn set(&mut self, key: impl Into<String>, val: impl Into<String>) {
        self.data.insert(key.into(), val.into());
    }
    /// Gets a value.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|s| s.as_str())
    }
    /// Returns the number of entries.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns whether the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
/// A read-only view of the environment with aggregated queries.
#[allow(dead_code)]
pub struct EnvironmentView<'a> {
    env: &'a Environment,
}
impl<'a> EnvironmentView<'a> {
    /// Create a view over an environment.
    #[allow(dead_code)]
    pub fn new(env: &'a Environment) -> Self {
        Self { env }
    }
    /// Get all axiom names.
    #[allow(dead_code)]
    pub fn axiom_names(&self) -> Vec<&Name> {
        self.env
            .constant_infos()
            .filter_map(|(name, ci)| if ci.is_axiom() { Some(name) } else { None })
            .collect()
    }
    /// Get all inductive type names.
    #[allow(dead_code)]
    pub fn inductive_names(&self) -> Vec<&Name> {
        self.env
            .constant_infos()
            .filter_map(|(name, ci)| if ci.is_inductive() { Some(name) } else { None })
            .collect()
    }
    /// Get all constructor names.
    #[allow(dead_code)]
    pub fn constructor_names(&self) -> Vec<&Name> {
        self.env
            .constant_infos()
            .filter_map(|(name, ci)| {
                if ci.is_constructor() {
                    Some(name)
                } else {
                    None
                }
            })
            .collect()
    }
    /// Get all definition names.
    #[allow(dead_code)]
    pub fn definition_names(&self) -> Vec<&Name> {
        self.env
            .constant_infos()
            .filter_map(|(name, ci)| if ci.is_definition() { Some(name) } else { None })
            .collect()
    }
    /// Count declarations by kind.
    #[allow(dead_code)]
    pub fn count_by_kind(&self) -> EnvKindCounts {
        let mut counts = EnvKindCounts::default();
        for (_, ci) in self.env.constant_infos() {
            if ci.is_axiom() {
                counts.axioms += 1;
            } else if ci.is_inductive() {
                counts.inductives += 1;
            } else if ci.is_constructor() {
                counts.constructors += 1;
            } else if ci.is_recursor() {
                counts.recursors += 1;
            } else if ci.is_definition() {
                counts.definitions += 1;
            } else if ci.is_theorem() {
                counts.theorems += 1;
            } else {
                counts.other += 1;
            }
        }
        counts
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
/// Environment statistics summary.
#[derive(Clone, Debug, Default)]
pub struct EnvStats {
    /// Total constants.
    pub total: usize,
    /// Axioms.
    pub axioms: usize,
    /// Definitions.
    pub definitions: usize,
    /// Theorems.
    pub theorems: usize,
    /// Inductive types.
    pub inductives: usize,
    /// Constructors.
    pub constructors: usize,
    /// Recursors.
    pub recursors: usize,
}
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
/// A counter that can measure elapsed time between snapshots.
#[allow(dead_code)]
pub struct Stopwatch {
    start: std::time::Instant,
    splits: Vec<f64>,
}
#[allow(dead_code)]
impl Stopwatch {
    /// Creates and starts a new stopwatch.
    pub fn start() -> Self {
        Self {
            start: std::time::Instant::now(),
            splits: Vec::new(),
        }
    }
    /// Records a split time (elapsed since start).
    pub fn split(&mut self) {
        self.splits.push(self.elapsed_ms());
    }
    /// Returns total elapsed milliseconds since start.
    pub fn elapsed_ms(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }
    /// Returns all recorded split times.
    pub fn splits(&self) -> &[f64] {
        &self.splits
    }
    /// Returns the number of splits.
    pub fn num_splits(&self) -> usize {
        self.splits.len()
    }
}
/// A token bucket rate limiter.
#[allow(dead_code)]
pub struct TokenBucket {
    capacity: u64,
    tokens: u64,
    refill_per_ms: u64,
    last_refill: std::time::Instant,
}
#[allow(dead_code)]
impl TokenBucket {
    /// Creates a new token bucket.
    pub fn new(capacity: u64, refill_per_ms: u64) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_per_ms,
            last_refill: std::time::Instant::now(),
        }
    }
    /// Attempts to consume `n` tokens.  Returns `true` on success.
    pub fn try_consume(&mut self, n: u64) -> bool {
        self.refill();
        if self.tokens >= n {
            self.tokens -= n;
            true
        } else {
            false
        }
    }
    fn refill(&mut self) {
        let now = std::time::Instant::now();
        let elapsed_ms = now.duration_since(self.last_refill).as_millis() as u64;
        if elapsed_ms > 0 {
            let new_tokens = elapsed_ms * self.refill_per_ms;
            self.tokens = (self.tokens + new_tokens).min(self.capacity);
            self.last_refill = now;
        }
    }
    /// Returns the number of currently available tokens.
    pub fn available(&self) -> u64 {
        self.tokens
    }
    /// Returns the bucket capacity.
    pub fn capacity(&self) -> u64 {
        self.capacity
    }
}
/// A builder for incrementally constructing an `Environment`.
#[allow(dead_code)]
#[derive(Default)]
pub struct EnvironmentBuilder {
    declarations: Vec<Declaration>,
    constants: Vec<crate::declaration::ConstantInfo>,
}
impl EnvironmentBuilder {
    /// Create a new empty builder.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Queue a legacy declaration for addition.
    #[allow(dead_code)]
    pub fn add_decl(mut self, decl: Declaration) -> Self {
        self.declarations.push(decl);
        self
    }
    /// Queue a ConstantInfo for addition.
    #[allow(dead_code)]
    pub fn add_constant(mut self, ci: crate::declaration::ConstantInfo) -> Self {
        self.constants.push(ci);
        self
    }
    /// Build the environment.
    #[allow(dead_code)]
    pub fn build(self) -> Result<Environment, EnvError> {
        let mut env = Environment::new();
        for decl in self.declarations {
            env.add(decl)?;
        }
        for ci in self.constants {
            env.add_constant(ci)?;
        }
        Ok(env)
    }
    /// Return the number of queued items.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.declarations.len() + self.constants.len()
    }
    /// Check if the builder has nothing queued.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.declarations.is_empty() && self.constants.is_empty()
    }
}
/// A simple decision tree node for rule dispatching.
#[allow(dead_code)]
#[allow(missing_docs)]
pub enum DecisionNode {
    /// A leaf with an action string.
    Leaf(String),
    /// An interior node: check `key` equals `val` → `yes_branch`, else `no_branch`.
    Branch {
        key: String,
        val: String,
        yes_branch: Box<DecisionNode>,
        no_branch: Box<DecisionNode>,
    },
}
#[allow(dead_code)]
impl DecisionNode {
    /// Evaluates the decision tree with the given context.
    pub fn evaluate(&self, ctx: &std::collections::HashMap<String, String>) -> &str {
        match self {
            DecisionNode::Leaf(action) => action.as_str(),
            DecisionNode::Branch {
                key,
                val,
                yes_branch,
                no_branch,
            } => {
                let actual = ctx.get(key).map(|s| s.as_str()).unwrap_or("");
                if actual == val.as_str() {
                    yes_branch.evaluate(ctx)
                } else {
                    no_branch.evaluate(ctx)
                }
            }
        }
    }
    /// Returns the depth of the decision tree.
    pub fn depth(&self) -> usize {
        match self {
            DecisionNode::Leaf(_) => 0,
            DecisionNode::Branch {
                yes_branch,
                no_branch,
                ..
            } => 1 + yes_branch.depth().max(no_branch.depth()),
        }
    }
}
/// A pool of reusable string buffers.
#[allow(dead_code)]
pub struct StringPool {
    free: Vec<String>,
}
#[allow(dead_code)]
impl StringPool {
    /// Creates a new empty string pool.
    pub fn new() -> Self {
        Self { free: Vec::new() }
    }
    /// Takes a string from the pool (may be empty).
    pub fn take(&mut self) -> String {
        self.free.pop().unwrap_or_default()
    }
    /// Returns a string to the pool.
    pub fn give(&mut self, mut s: String) {
        s.clear();
        self.free.push(s);
    }
    /// Returns the number of free strings in the pool.
    pub fn free_count(&self) -> usize {
        self.free.len()
    }
}
/// A tagged union for representing a simple two-case discriminated union.
#[allow(dead_code)]
pub enum Either2<A, B> {
    /// The first alternative.
    First(A),
    /// The second alternative.
    Second(B),
}
#[allow(dead_code)]
impl<A, B> Either2<A, B> {
    /// Returns `true` if this is the first alternative.
    pub fn is_first(&self) -> bool {
        matches!(self, Either2::First(_))
    }
    /// Returns `true` if this is the second alternative.
    pub fn is_second(&self) -> bool {
        matches!(self, Either2::Second(_))
    }
    /// Returns the first value if present.
    pub fn first(self) -> Option<A> {
        match self {
            Either2::First(a) => Some(a),
            _ => None,
        }
    }
    /// Returns the second value if present.
    pub fn second(self) -> Option<B> {
        match self {
            Either2::Second(b) => Some(b),
            _ => None,
        }
    }
    /// Maps over the first alternative.
    pub fn map_first<C, F: FnOnce(A) -> C>(self, f: F) -> Either2<C, B> {
        match self {
            Either2::First(a) => Either2::First(f(a)),
            Either2::Second(b) => Either2::Second(b),
        }
    }
}
/// A simple stack-based calculator for arithmetic expressions.
#[allow(dead_code)]
pub struct StackCalc {
    stack: Vec<i64>,
}
#[allow(dead_code)]
impl StackCalc {
    /// Creates a new empty calculator.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }
    /// Pushes an integer literal.
    pub fn push(&mut self, n: i64) {
        self.stack.push(n);
    }
    /// Adds the top two values.  Panics if fewer than two values.
    pub fn add(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for add");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for add");
        self.stack.push(a + b);
    }
    /// Subtracts top from second.
    pub fn sub(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for sub");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for sub");
        self.stack.push(a - b);
    }
    /// Multiplies the top two values.
    pub fn mul(&mut self) {
        let b = self
            .stack
            .pop()
            .expect("stack must have at least two values for mul");
        let a = self
            .stack
            .pop()
            .expect("stack must have at least two values for mul");
        self.stack.push(a * b);
    }
    /// Peeks the top value.
    pub fn peek(&self) -> Option<i64> {
        self.stack.last().copied()
    }
    /// Returns the stack depth.
    pub fn depth(&self) -> usize {
        self.stack.len()
    }
}
/// A fixed-size sliding window that computes a running sum.
#[allow(dead_code)]
pub struct SlidingSum {
    window: Vec<f64>,
    capacity: usize,
    pos: usize,
    sum: f64,
    count: usize,
}
#[allow(dead_code)]
impl SlidingSum {
    /// Creates a sliding sum with the given window size.
    pub fn new(capacity: usize) -> Self {
        Self {
            window: vec![0.0; capacity],
            capacity,
            pos: 0,
            sum: 0.0,
            count: 0,
        }
    }
    /// Adds a value to the window, removing the oldest if full.
    pub fn push(&mut self, val: f64) {
        let oldest = self.window[self.pos];
        self.sum -= oldest;
        self.sum += val;
        self.window[self.pos] = val;
        self.pos = (self.pos + 1) % self.capacity;
        if self.count < self.capacity {
            self.count += 1;
        }
    }
    /// Returns the current window sum.
    pub fn sum(&self) -> f64 {
        self.sum
    }
    /// Returns the window mean, or `None` if empty.
    pub fn mean(&self) -> Option<f64> {
        if self.count == 0 {
            None
        } else {
            Some(self.sum / self.count as f64)
        }
    }
    /// Returns the current window size (number of valid elements).
    pub fn count(&self) -> usize {
        self.count
    }
}
/// A reusable scratch buffer for path computations.
#[allow(dead_code)]
pub struct PathBuf {
    components: Vec<String>,
}
#[allow(dead_code)]
impl PathBuf {
    /// Creates a new empty path buffer.
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }
    /// Pushes a component.
    pub fn push(&mut self, comp: impl Into<String>) {
        self.components.push(comp.into());
    }
    /// Pops the last component.
    pub fn pop(&mut self) {
        self.components.pop();
    }
    /// Returns the current path as a `/`-separated string.
    pub fn as_str(&self) -> String {
        self.components.join("/")
    }
    /// Returns the depth of the path.
    pub fn depth(&self) -> usize {
        self.components.len()
    }
    /// Clears the path.
    pub fn clear(&mut self) {
        self.components.clear();
    }
}
/// A flat list of substitution pairs `(from, to)`.
#[allow(dead_code)]
pub struct FlatSubstitution {
    pairs: Vec<(String, String)>,
}
#[allow(dead_code)]
impl FlatSubstitution {
    /// Creates an empty substitution.
    pub fn new() -> Self {
        Self { pairs: Vec::new() }
    }
    /// Adds a pair.
    pub fn add(&mut self, from: impl Into<String>, to: impl Into<String>) {
        self.pairs.push((from.into(), to.into()));
    }
    /// Applies all substitutions to `s` (leftmost-first order).
    pub fn apply(&self, s: &str) -> String {
        let mut result = s.to_string();
        for (from, to) in &self.pairs {
            result = result.replace(from.as_str(), to.as_str());
        }
        result
    }
    /// Returns the number of pairs.
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.pairs.is_empty()
    }
}
/// Errors that can occur when working with the environment.
#[derive(Clone, Debug)]
pub enum EnvError {
    /// A declaration with this name already exists
    DuplicateDeclaration(Name),
    /// The declaration was not found
    NotFound(Name),
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
/// A dependency closure builder (transitive closure via BFS).
#[allow(dead_code)]
pub struct TransitiveClosure {
    adj: Vec<Vec<usize>>,
    n: usize,
}
#[allow(dead_code)]
impl TransitiveClosure {
    /// Creates a transitive closure builder for `n` nodes.
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![Vec::new(); n],
            n,
        }
    }
    /// Adds a direct edge.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n {
            self.adj[from].push(to);
        }
    }
    /// Computes all nodes reachable from `start` (including `start`).
    pub fn reachable_from(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            if node >= self.n || visited[node] {
                continue;
            }
            visited[node] = true;
            for &next in &self.adj[node] {
                queue.push_back(next);
            }
        }
        (0..self.n).filter(|&i| visited[i]).collect()
    }
    /// Returns `true` if `from` can transitively reach `to`.
    pub fn can_reach(&self, from: usize, to: usize) -> bool {
        self.reachable_from(from).contains(&to)
    }
}
/// A min-heap implemented as a binary heap.
#[allow(dead_code)]
pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}
#[allow(dead_code)]
impl<T: Ord> MinHeap<T> {
    /// Creates a new empty min-heap.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    /// Inserts an element.
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.sift_up(self.data.len() - 1);
    }
    /// Removes and returns the minimum element.
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let n = self.data.len();
        self.data.swap(0, n - 1);
        let min = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        min
    }
    /// Returns a reference to the minimum element.
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }
    /// Returns the number of elements.
    pub fn len(&self) -> usize {
        self.data.len()
    }
    /// Returns `true` if empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }
    fn sift_down(&mut self, mut i: usize) {
        let n = self.data.len();
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut smallest = i;
            if left < n && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < n && self.data[right] < self.data[smallest] {
                smallest = right;
            }
            if smallest == i {
                break;
            }
            self.data.swap(i, smallest);
            i = smallest;
        }
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
/// A snapshot of an `Environment` at a point in time.
///
/// Snapshots are useful for implementing undo/redo in interactive mode.
#[derive(Clone, Debug)]
pub struct EnvironmentSnapshot {
    /// Names present at snapshot time, in insertion order.
    pub names: Vec<Name>,
}
impl EnvironmentSnapshot {
    /// Create a snapshot from an environment.
    pub fn from_env(env: &Environment) -> Self {
        let names: Vec<Name> = env.constant_names().cloned().collect();
        Self { names }
    }
    /// Number of constants in the snapshot.
    pub fn len(&self) -> usize {
        self.names.len()
    }
    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }
    /// Return the names added in `new` that were not in `self`.
    pub fn diff<'a>(&self, new: &'a EnvironmentSnapshot) -> Vec<&'a Name> {
        new.names
            .iter()
            .filter(|n| !self.names.contains(n))
            .collect()
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
/// A write-once cell.
#[allow(dead_code)]
pub struct WriteOnce<T> {
    value: std::cell::Cell<Option<T>>,
}
#[allow(dead_code)]
impl<T: Copy> WriteOnce<T> {
    /// Creates an empty write-once cell.
    pub fn new() -> Self {
        Self {
            value: std::cell::Cell::new(None),
        }
    }
    /// Writes a value.  Returns `false` if already written.
    pub fn write(&self, val: T) -> bool {
        if self.value.get().is_some() {
            return false;
        }
        self.value.set(Some(val));
        true
    }
    /// Returns the value if written.
    pub fn read(&self) -> Option<T> {
        self.value.get()
    }
    /// Returns `true` if the value has been written.
    pub fn is_written(&self) -> bool {
        self.value.get().is_some()
    }
}
