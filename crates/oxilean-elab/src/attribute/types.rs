//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{Expr, Name};
use oxilean_parse::AttributeKind;
use std::collections::HashMap;

/// A rule for propagating attributes from one declaration to another.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AttrPropagationRule {
    /// The source attribute kind (trigger).
    pub source: AttributeKind,
    /// The target attribute kind (applied to the derived declaration).
    pub target: AttributeKind,
    /// Optional condition (e.g. only propagate if the decl has a certain name prefix).
    pub condition: Option<String>,
}
impl AttrPropagationRule {
    /// Create a simple propagation rule with no condition.
    #[allow(dead_code)]
    pub fn new(source: AttributeKind, target: AttributeKind) -> Self {
        Self {
            source,
            target,
            condition: None,
        }
    }
    /// Create a propagation rule with a name-prefix condition.
    #[allow(dead_code)]
    pub fn with_condition(
        source: AttributeKind,
        target: AttributeKind,
        condition: impl Into<String>,
    ) -> Self {
        Self {
            source,
            target,
            condition: Some(condition.into()),
        }
    }
    /// Check whether the rule applies to the given declaration name.
    #[allow(dead_code)]
    pub fn applies(&self, decl_name: &Name) -> bool {
        match &self.condition {
            None => true,
            Some(prefix) => decl_name.to_string().starts_with(prefix.as_str()),
        }
    }
}
/// Stages in the attribute processing pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AttrPipelineStage {
    /// Attribute has been parsed.
    Parsed,
    /// Attribute has been validated.
    Validated,
    /// Attribute has been applied to the declaration.
    Applied,
    /// Attribute has triggered post-processing.
    PostProcessed,
    /// Done.
    Done,
}
/// Statistics about the registered attributes.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct AttrStats {
    /// Total attributes registered.
    pub total: usize,
    /// Number of simp attributes.
    pub simp_count: usize,
    /// Number of instance attributes.
    pub instance_count: usize,
    /// Number of reducible attributes.
    pub reducible_count: usize,
    /// Number of irreducible attributes.
    pub irreducible_count: usize,
    /// Number of inline attributes.
    pub inline_count: usize,
    /// Number of custom attributes.
    pub custom_count: usize,
}
impl AttrStats {
    /// Collect statistics from an attribute manager.
    #[allow(dead_code)]
    pub fn collect(manager: &AttributeManager) -> Self {
        let mut stats = Self::default();
        for entries in manager.entries.values() {
            for entry in entries {
                stats.total += 1;
                match &entry.kind {
                    AttributeKind::Simp => stats.simp_count += 1,
                    AttributeKind::Instance => stats.instance_count += 1,
                    AttributeKind::Reducible => stats.reducible_count += 1,
                    AttributeKind::Irreducible => stats.irreducible_count += 1,
                    AttributeKind::Inline => stats.inline_count += 1,
                    AttributeKind::Custom(_) => stats.custom_count += 1,
                    _ => {}
                }
            }
        }
        stats
    }
}
/// Action to perform when an attribute is applied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttrAction {
    /// Add the declaration to the simp set
    AddToSimpSet,
    /// Add the declaration to the ext set
    AddToExtSet,
    /// Mark as a typeclass instance
    MarkAsInstance,
    /// Set reducibility (true = reducible, false = irreducible)
    SetReducibility(bool),
    /// Set inline hint (true = inline, false = noinline)
    SetInline(bool),
    /// A custom action identified by name
    Custom(String),
}
/// Handler for processing a specific attribute kind.
#[derive(Clone, Debug)]
pub struct AttrHandler {
    /// Name of the attribute this handler processes
    pub name: String,
    /// Documentation string for the attribute
    pub doc: String,
    /// The action to perform
    pub action: AttrAction,
}
impl AttrHandler {
    /// Create a new attribute handler.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, doc: impl Into<String>, action: AttrAction) -> Self {
        Self {
            name: name.into(),
            doc: doc.into(),
            action,
        }
    }
}
/// Registry of all registered derive handlers.
#[derive(Clone, Debug, Default)]
pub struct DeriveHandlerRegistry {
    handlers: HashMap<Name, DeriveHandler>,
}
impl DeriveHandlerRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a derive handler.
    pub fn register(&mut self, handler: DeriveHandler) {
        self.handlers.insert(handler.class_name.clone(), handler);
    }
    /// Look up a derive handler by class name.
    pub fn get(&self, class_name: &Name) -> Option<&DeriveHandler> {
        self.handlers.get(class_name)
    }
    /// Return `true` if a handler for `class_name` is registered.
    pub fn has(&self, class_name: &Name) -> bool {
        self.handlers.contains_key(class_name)
    }
    /// Number of registered handlers.
    pub fn len(&self) -> usize {
        self.handlers.len()
    }
    /// Return `true` if no handlers are registered.
    pub fn is_empty(&self) -> bool {
        self.handlers.is_empty()
    }
    /// Return all registered class names.
    pub fn class_names(&self) -> impl Iterator<Item = &Name> {
        self.handlers.keys()
    }
}
/// Result of processing a list of attributes on a declaration.
///
/// Provides boolean flags for commonly queried attribute states.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProcessedAttrs {
    /// Whether `@[simp]` is present
    pub is_simp: bool,
    /// Whether `@[ext]` is present
    pub is_ext: bool,
    /// Whether `@[instance]` is present
    pub is_instance: bool,
    /// Whether `@[reducible]` is present
    pub is_reducible: bool,
    /// Whether `@[irreducible]` is present
    pub is_irreducible: bool,
    /// Whether `@[inline]` is present
    pub is_inline: bool,
    /// Whether `@[noinline]` is present
    pub is_noinline: bool,
    /// Whether `@[specialize]` is present
    pub is_specialize: bool,
    /// Priority if specified
    pub priority: Option<u32>,
    /// Custom attributes with their arguments
    pub custom: Vec<(String, Vec<Expr>)>,
}
impl ProcessedAttrs {
    /// Create empty processed attributes.
    #[allow(dead_code)]
    pub fn empty() -> Self {
        Self::default()
    }
    /// Check if any attribute flag is set.
    #[allow(dead_code)]
    pub fn has_any(&self) -> bool {
        self.is_simp
            || self.is_ext
            || self.is_instance
            || self.is_reducible
            || self.is_irreducible
            || self.is_inline
            || self.is_noinline
            || self.is_specialize
            || !self.custom.is_empty()
    }
    /// Count how many attributes are set.
    #[allow(dead_code)]
    pub fn count(&self) -> usize {
        let mut n = 0;
        if self.is_simp {
            n += 1;
        }
        if self.is_ext {
            n += 1;
        }
        if self.is_instance {
            n += 1;
        }
        if self.is_reducible {
            n += 1;
        }
        if self.is_irreducible {
            n += 1;
        }
        if self.is_inline {
            n += 1;
        }
        if self.is_noinline {
            n += 1;
        }
        if self.is_specialize {
            n += 1;
        }
        n + self.custom.len()
    }
}
/// An ordered simp set, with entries sorted by decreasing priority.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct SimpSet {
    entries: Vec<SimpEntry>,
}
impl SimpSet {
    /// Create an empty simp set.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a simp entry, maintaining priority order.
    #[allow(dead_code)]
    pub fn insert(&mut self, entry: SimpEntry) {
        let pos = self
            .entries
            .partition_point(|e| e.priority >= entry.priority);
        self.entries.insert(pos, entry);
    }
    /// Remove an entry by name.  Returns true if removed.
    #[allow(dead_code)]
    pub fn remove(&mut self, name: &Name) -> bool {
        let before = self.entries.len();
        self.entries.retain(|e| &e.name != name);
        self.entries.len() < before
    }
    /// Get all entries.
    #[allow(dead_code)]
    pub fn entries(&self) -> &[SimpEntry] {
        &self.entries
    }
    /// Number of entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Whether the set is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Check whether a name is in the simp set.
    #[allow(dead_code)]
    pub fn contains(&self, name: &Name) -> bool {
        self.entries.iter().any(|e| &e.name == name)
    }
    /// Get entries with a specific tag.
    #[allow(dead_code)]
    pub fn by_tag(&self, tag: &str) -> Vec<&SimpEntry> {
        self.entries
            .iter()
            .filter(|e| e.tag.as_deref() == Some(tag))
            .collect()
    }
    /// Get forward (non-reverse) entries only.
    #[allow(dead_code)]
    pub fn forward_entries(&self) -> Vec<&SimpEntry> {
        self.entries.iter().filter(|e| !e.reverse).collect()
    }
    /// Get reverse entries only.
    #[allow(dead_code)]
    pub fn reverse_entries(&self) -> Vec<&SimpEntry> {
        self.entries.iter().filter(|e| e.reverse).collect()
    }
}
/// A macro attribute invocation: `@[macro_name args...]`.
#[derive(Clone, Debug)]
pub struct MacroAttr {
    /// The macro name.
    pub name: Name,
    /// Raw argument string.
    pub args: String,
    /// Source range of the attribute.
    pub range: (u32, u32),
}
impl MacroAttr {
    /// Create a macro attribute.
    pub fn new(name: Name, args: impl Into<String>, range: (u32, u32)) -> Self {
        Self {
            name,
            args: args.into(),
            range,
        }
    }
    /// Return `true` if this macro attribute has no arguments.
    pub fn has_no_args(&self) -> bool {
        self.args.trim().is_empty()
    }
}
/// An attribute entry with scope information.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ScopedAttrEntry {
    /// The underlying attribute entry.
    pub entry: AttrEntry,
    /// The scope of this attribute.
    pub scope: AttrScope,
}
impl ScopedAttrEntry {
    /// Create a global scoped attribute entry.
    #[allow(dead_code)]
    pub fn global(entry: AttrEntry) -> Self {
        Self {
            entry,
            scope: AttrScope::Global,
        }
    }
    /// Create a local scoped attribute entry.
    #[allow(dead_code)]
    pub fn local(entry: AttrEntry) -> Self {
        Self {
            entry,
            scope: AttrScope::Local,
        }
    }
    /// Create a namespace-scoped attribute entry.
    #[allow(dead_code)]
    pub fn in_namespace(entry: AttrEntry, ns: impl Into<String>) -> Self {
        Self {
            entry,
            scope: AttrScope::Namespace(ns.into()),
        }
    }
}
/// A filter for selecting attributes from the attribute manager.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct AttrFilter {
    /// Only return attributes of these kinds (empty = any kind).
    pub kinds: Vec<AttributeKind>,
    /// Only return attributes with priority >= this value.
    pub min_priority: u32,
    /// Only return attributes with priority <= this value.
    pub max_priority: u32,
}
impl AttrFilter {
    /// Create a filter that accepts any attribute.
    #[allow(dead_code)]
    pub fn any() -> Self {
        Self {
            kinds: Vec::new(),
            min_priority: 0,
            max_priority: u32::MAX,
        }
    }
    /// Create a filter for a single attribute kind.
    #[allow(dead_code)]
    pub fn for_kind(kind: AttributeKind) -> Self {
        Self {
            kinds: vec![kind],
            min_priority: 0,
            max_priority: u32::MAX,
        }
    }
    /// Create a filter for a priority range.
    #[allow(dead_code)]
    pub fn priority_range(min: u32, max: u32) -> Self {
        Self {
            kinds: Vec::new(),
            min_priority: min,
            max_priority: max,
        }
    }
    /// Check whether an attribute entry passes this filter.
    #[allow(dead_code)]
    pub fn matches(&self, entry: &AttrEntry) -> bool {
        let kind_ok = self.kinds.is_empty() || self.kinds.contains(&entry.kind);
        let priority_ok =
            entry.priority >= self.min_priority && entry.priority <= self.max_priority;
        kind_ok && priority_ok
    }
}
/// A single attribute application on a declaration.
#[derive(Clone, Debug)]
pub struct AttrEntry {
    /// The kind of attribute
    pub kind: AttributeKind,
    /// The declaration name this is applied to
    pub decl_name: Name,
    /// Arguments passed to the attribute
    pub args: Vec<Expr>,
    /// Priority (for instance resolution, simp ordering, etc.)
    pub priority: u32,
}
impl AttrEntry {
    /// Create a new attribute entry with default priority.
    #[allow(dead_code)]
    pub fn new(kind: AttributeKind, decl_name: Name) -> Self {
        Self {
            kind,
            decl_name,
            args: Vec::new(),
            priority: 1000,
        }
    }
    /// Create a new attribute entry with explicit priority.
    #[allow(dead_code)]
    pub fn with_priority(kind: AttributeKind, decl_name: Name, priority: u32) -> Self {
        Self {
            kind,
            decl_name,
            args: Vec::new(),
            priority,
        }
    }
    /// Create a new attribute entry with arguments.
    #[allow(dead_code)]
    pub fn with_args(kind: AttributeKind, decl_name: Name, args: Vec<Expr>) -> Self {
        Self {
            kind,
            decl_name,
            args,
            priority: 1000,
        }
    }
    /// Get the string name of the attribute kind.
    pub fn kind_name(&self) -> &str {
        self.kind.name()
    }
}
/// Tracks an attribute through its processing pipeline.
#[derive(Debug, Clone)]
pub struct AttrPipeline {
    /// The attribute entry being processed.
    pub entry: AttrEntry,
    /// Current stage.
    pub stage: AttrPipelineStage,
    /// Error if processing failed.
    pub error: Option<AttrError>,
}
impl AttrPipeline {
    /// Create a new pipeline starting at `Parsed`.
    pub fn new(entry: AttrEntry) -> Self {
        Self {
            entry,
            stage: AttrPipelineStage::Parsed,
            error: None,
        }
    }
    /// Advance to the next stage.
    pub fn advance(&mut self) {
        self.stage = match self.stage {
            AttrPipelineStage::Parsed => AttrPipelineStage::Validated,
            AttrPipelineStage::Validated => AttrPipelineStage::Applied,
            AttrPipelineStage::Applied => AttrPipelineStage::PostProcessed,
            AttrPipelineStage::PostProcessed => AttrPipelineStage::Done,
            AttrPipelineStage::Done => AttrPipelineStage::Done,
        };
    }
    /// Fail the pipeline with an error.
    pub fn fail(&mut self, err: AttrError) {
        self.error = Some(err);
        self.stage = AttrPipelineStage::Done;
    }
    /// Return `true` if processing has completed.
    pub fn is_done(&self) -> bool {
        self.stage == AttrPipelineStage::Done
    }
    /// Return `true` if processing succeeded (done, no error).
    pub fn is_success(&self) -> bool {
        self.is_done() && self.error.is_none()
    }
}
/// A derive handler produces elaboration actions for a `#[derive(...)]` attribute.
#[derive(Clone, Debug)]
pub struct DeriveHandler {
    /// The class name this handler implements (e.g. `Repr`, `DecidableEq`).
    pub class_name: Name,
    /// Human-readable description.
    pub description: String,
    /// Whether the handler requires the type to have a decidable equality.
    pub requires_dec_eq: bool,
    /// Whether the handler generates a typeclass instance.
    pub generates_instance: bool,
}
impl DeriveHandler {
    /// Create a new derive handler.
    pub fn new(class_name: Name, description: impl Into<String>) -> Self {
        Self {
            class_name,
            description: description.into(),
            requires_dec_eq: false,
            generates_instance: true,
        }
    }
    /// Mark as requiring decidable equality.
    pub fn with_dec_eq(mut self) -> Self {
        self.requires_dec_eq = true;
        self
    }
    /// Mark as not generating an instance (pure code generation).
    pub fn no_instance(mut self) -> Self {
        self.generates_instance = false;
        self
    }
}
/// A priority queue of type-class instances for resolution.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct InstancePriorityQueue {
    instances: Vec<(Name, u32)>,
}
impl InstancePriorityQueue {
    /// Create an empty queue.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert an instance with a priority.
    #[allow(dead_code)]
    pub fn insert(&mut self, name: Name, priority: u32) {
        let pos = self.instances.partition_point(|(_, p)| *p >= priority);
        self.instances.insert(pos, (name, priority));
    }
    /// Get the highest-priority instance.
    #[allow(dead_code)]
    pub fn peek(&self) -> Option<&Name> {
        self.instances.first().map(|(n, _)| n)
    }
    /// Pop the highest-priority instance.
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<(Name, u32)> {
        if self.instances.is_empty() {
            None
        } else {
            Some(self.instances.remove(0))
        }
    }
    /// Number of instances.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.instances.len()
    }
    /// Whether the queue is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }
    /// Get all instance names in priority order.
    #[allow(dead_code)]
    pub fn names_in_order(&self) -> Vec<&Name> {
        self.instances.iter().map(|(n, _)| n).collect()
    }
}
/// Result of expanding a macro attribute.
#[derive(Clone, Debug)]
pub struct MacroAttrExpansion {
    /// The original attribute.
    pub attr: MacroAttr,
    /// The attributes produced by expansion.
    pub produced_attrs: Vec<AttrEntry>,
    /// The declarations produced by expansion.
    pub produced_decls: Vec<Name>,
    /// Whether expansion succeeded.
    pub success: bool,
    /// Error message if expansion failed.
    pub error: Option<String>,
}
impl MacroAttrExpansion {
    /// Create a successful expansion.
    pub fn success(attr: MacroAttr, attrs: Vec<AttrEntry>, decls: Vec<Name>) -> Self {
        Self {
            attr,
            produced_attrs: attrs,
            produced_decls: decls,
            success: true,
            error: None,
        }
    }
    /// Create a failed expansion.
    pub fn failure(attr: MacroAttr, error: impl Into<String>) -> Self {
        Self {
            attr,
            produced_attrs: Vec::new(),
            produced_decls: Vec::new(),
            success: false,
            error: Some(error.into()),
        }
    }
}
/// Error from attribute processing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttrError {
    /// Unknown attribute name
    UnknownAttribute(String),
    /// Invalid arguments to an attribute
    InvalidArgs(String),
    /// Duplicate attribute on the same declaration
    DuplicateAttribute(String),
    /// Two attributes that cannot coexist
    IncompatibleAttributes(String, String),
    /// Other error
    Other(String),
}
/// A snapshot of all attributes at a point in time.
///
/// Can be used to roll back attribute registrations.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AttrSnapshot {
    entries: std::collections::HashMap<Name, Vec<AttrEntry>>,
    by_kind: std::collections::HashMap<String, Vec<Name>>,
}
impl AttrSnapshot {
    /// Take a snapshot from a manager.
    #[allow(dead_code)]
    pub fn take(manager: &AttributeManager) -> Self {
        Self {
            entries: manager.entries.clone(),
            by_kind: manager.by_kind.clone(),
        }
    }
    /// Restore a manager to a snapshot.
    #[allow(dead_code)]
    pub fn restore(self, manager: &mut AttributeManager) {
        manager.entries = self.entries;
        manager.by_kind = self.by_kind;
    }
    /// Number of entries in the snapshot.
    #[allow(dead_code)]
    pub fn total_entries(&self) -> usize {
        self.entries.values().map(Vec::len).sum()
    }
}
/// An attribute inheritance record: declarations that inherit attributes
/// from a base declaration.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct AttrInheritance {
    /// Map from base declaration to list of derived declarations.
    derives: std::collections::HashMap<Name, Vec<Name>>,
}
impl AttrInheritance {
    /// Create an empty inheritance record.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Register that `derived` inherits attributes from `base`.
    #[allow(dead_code)]
    pub fn register(&mut self, base: Name, derived: Name) {
        self.derives.entry(base).or_default().push(derived);
    }
    /// Get all declarations that inherit from `base`.
    #[allow(dead_code)]
    pub fn derived_from(&self, base: &Name) -> &[Name] {
        self.derives.get(base).map(Vec::as_slice).unwrap_or(&[])
    }
    /// Check whether `derived` inherits from `base`.
    #[allow(dead_code)]
    pub fn inherits_from(&self, base: &Name, derived: &Name) -> bool {
        self.derives
            .get(base)
            .map(|ds| ds.contains(derived))
            .unwrap_or(false)
    }
}
/// The scope of an attribute: whether it applies globally or locally.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttrScope {
    /// The attribute applies to the current declaration only.
    Local,
    /// The attribute is exported and applies globally.
    Global,
    /// The attribute applies within a named namespace.
    Namespace(String),
}
impl AttrScope {
    /// Whether this scope is global.
    #[allow(dead_code)]
    pub fn is_global(&self) -> bool {
        self == &AttrScope::Global
    }
    /// Whether this scope is local.
    #[allow(dead_code)]
    pub fn is_local(&self) -> bool {
        self == &AttrScope::Local
    }
    /// Get the namespace name if this is a namespace scope.
    #[allow(dead_code)]
    pub fn namespace(&self) -> Option<&str> {
        if let AttrScope::Namespace(ns) = self {
            Some(ns.as_str())
        } else {
            None
        }
    }
}
/// Manages attribute registrations for all declarations.
///
/// Provides both per-declaration lookups and reverse lookups
/// (e.g. "give me all simp lemmas").
pub struct AttributeManager {
    /// Attributes indexed by declaration name
    pub(super) entries: HashMap<Name, Vec<AttrEntry>>,
    /// Declaration names indexed by attribute kind string
    by_kind: HashMap<String, Vec<Name>>,
    /// Custom attribute handlers
    custom_handlers: HashMap<String, AttrHandler>,
}
impl AttributeManager {
    /// Create a new empty attribute manager.
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            by_kind: HashMap::new(),
            custom_handlers: HashMap::new(),
        }
    }
    /// Register an attribute entry for a declaration.
    pub fn register_attribute(&mut self, entry: AttrEntry) -> Result<(), AttrError> {
        let kind_name = entry.kind_name().to_string();
        let decl_name = entry.decl_name.clone();
        if let Some(existing) = self.entries.get(&decl_name) {
            for e in existing {
                if e.kind == entry.kind {
                    return Err(AttrError::DuplicateAttribute(format!(
                        "'{}' on '{:?}'",
                        kind_name, decl_name
                    )));
                }
            }
        }
        if let Some(existing) = self.entries.get(&decl_name) {
            for e in existing {
                if let Some(err) = check_incompatible(&e.kind, &entry.kind) {
                    return Err(err);
                }
            }
        }
        let kind_entries = self.by_kind.entry(kind_name).or_default();
        if !kind_entries.contains(&decl_name) {
            kind_entries.push(decl_name.clone());
        }
        self.entries.entry(decl_name).or_default().push(entry);
        Ok(())
    }
    /// Unregister a specific attribute from a declaration.
    ///
    /// Returns `true` if the attribute was found and removed.
    #[allow(dead_code)]
    pub fn unregister_attribute(&mut self, decl_name: &Name, kind: &AttributeKind) -> bool {
        let kind_name = kind.name();
        let removed = if let Some(entries) = self.entries.get_mut(decl_name) {
            let before = entries.len();
            entries.retain(|e| &e.kind != kind);
            let after = entries.len();
            if entries.is_empty() {
                self.entries.remove(decl_name);
            }
            before != after
        } else {
            false
        };
        if removed {
            if let Some(names) = self.by_kind.get_mut(kind_name) {
                names.retain(|n| n != decl_name);
                if names.is_empty() {
                    self.by_kind.remove(kind_name);
                }
            }
        }
        removed
    }
    /// Get all attributes for a declaration.
    #[allow(dead_code)]
    pub fn get_attributes(&self, decl_name: &Name) -> Vec<&AttrEntry> {
        self.entries
            .get(decl_name)
            .map(|entries| entries.iter().collect())
            .unwrap_or_default()
    }
    /// Check if a declaration has a specific attribute kind.
    pub fn has_attribute(&self, decl_name: &Name, kind: &AttributeKind) -> bool {
        self.entries
            .get(decl_name)
            .map(|entries| entries.iter().any(|e| &e.kind == kind))
            .unwrap_or(false)
    }
    /// Get all declarations with `@[simp]`.
    #[allow(dead_code)]
    pub fn get_simp_lemmas(&self) -> Vec<Name> {
        self.get_by_kind("simp")
    }
    /// Get all declarations with `@[ext]`.
    #[allow(dead_code)]
    pub fn get_ext_lemmas(&self) -> Vec<Name> {
        self.get_by_kind("ext")
    }
    /// Get all declarations with `@[instance]`.
    #[allow(dead_code)]
    pub fn get_instances(&self) -> Vec<Name> {
        self.get_by_kind("instance")
    }
    /// Get all declarations with `@[reducible]`.
    #[allow(dead_code)]
    pub fn get_reducible(&self) -> Vec<Name> {
        self.get_by_kind("reducible")
    }
    /// Get all declarations with `@[irreducible]`.
    #[allow(dead_code)]
    pub fn get_irreducible(&self) -> Vec<Name> {
        self.get_by_kind("irreducible")
    }
    /// Get all declarations with `@[inline]`.
    #[allow(dead_code)]
    pub fn get_inline(&self) -> Vec<Name> {
        self.get_by_kind("inline")
    }
    /// Get all declarations by attribute kind string.
    pub fn get_by_kind(&self, kind_str: &str) -> Vec<Name> {
        self.by_kind.get(kind_str).cloned().unwrap_or_default()
    }
    /// Check if a declaration is reducible.
    #[allow(dead_code)]
    pub fn is_reducible(&self, name: &Name) -> bool {
        self.has_attribute(name, &AttributeKind::Reducible)
    }
    /// Check if a declaration is irreducible.
    #[allow(dead_code)]
    pub fn is_irreducible(&self, name: &Name) -> bool {
        self.has_attribute(name, &AttributeKind::Irreducible)
    }
    /// Check if a declaration is inline.
    #[allow(dead_code)]
    pub fn is_inline(&self, name: &Name) -> bool {
        self.has_attribute(name, &AttributeKind::Inline)
    }
    /// Check if a declaration is an instance.
    #[allow(dead_code)]
    pub fn is_instance(&self, name: &Name) -> bool {
        self.has_attribute(name, &AttributeKind::Instance)
    }
    /// Check if a declaration is a simp lemma.
    #[allow(dead_code)]
    pub fn is_simp(&self, name: &Name) -> bool {
        self.has_attribute(name, &AttributeKind::Simp)
    }
    /// Validate a list of attributes for incompatible combinations.
    #[allow(dead_code)]
    pub fn validate_attributes(&self, attrs: &[AttributeKind]) -> Result<(), AttrError> {
        for i in 0..attrs.len() {
            for j in (i + 1)..attrs.len() {
                if let Some(err) = check_incompatible(&attrs[i], &attrs[j]) {
                    return Err(err);
                }
            }
            for j in (i + 1)..attrs.len() {
                if attrs[i] == attrs[j] {
                    return Err(AttrError::DuplicateAttribute(attrs[i].name().to_string()));
                }
            }
        }
        for attr in attrs {
            if let AttributeKind::Custom(name) = attr {
                if !self.custom_handlers.contains_key(name) {}
            }
        }
        Ok(())
    }
    /// Register a custom attribute handler.
    #[allow(dead_code)]
    pub fn register_custom_handler(&mut self, handler: AttrHandler) {
        self.custom_handlers.insert(handler.name.clone(), handler);
    }
    /// Get a custom handler by name.
    #[allow(dead_code)]
    pub fn get_handler(&self, name: &str) -> Option<&AttrHandler> {
        self.custom_handlers.get(name)
    }
    /// Get the total number of registered attributes across all declarations.
    #[allow(dead_code)]
    pub fn total_entries(&self) -> usize {
        self.entries.values().map(|v| v.len()).sum()
    }
    /// Get the number of declarations that have at least one attribute.
    #[allow(dead_code)]
    pub fn num_attributed_decls(&self) -> usize {
        self.entries.len()
    }
    /// Get all declaration names that have attributes.
    #[allow(dead_code)]
    pub fn all_attributed_names(&self) -> Vec<&Name> {
        self.entries.keys().collect()
    }
    /// Clear all registered attributes.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
        self.by_kind.clear();
    }
    /// Merge another attribute manager into this one.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &AttributeManager) -> Result<(), AttrError> {
        for entries in other.entries.values() {
            for entry in entries {
                self.register_attribute(entry.clone())?;
            }
        }
        Ok(())
    }
}
/// An entry in the simp set with an associated priority.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SimpEntry {
    /// The declaration name.
    pub name: Name,
    /// Priority (higher = applied first).
    pub priority: u32,
    /// Whether this is a reverse simp rule (rw [<- ...]).
    pub reverse: bool,
    /// Optional tag grouping this rule.
    pub tag: Option<String>,
}
impl SimpEntry {
    /// Create a basic simp entry.
    #[allow(dead_code)]
    pub fn new(name: Name, priority: u32) -> Self {
        Self {
            name,
            priority,
            reverse: false,
            tag: None,
        }
    }
    /// Create a reverse simp entry.
    #[allow(dead_code)]
    pub fn reverse(name: Name, priority: u32) -> Self {
        Self {
            name,
            priority,
            reverse: true,
            tag: None,
        }
    }
    /// Set the tag of this entry.
    #[allow(dead_code)]
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }
}
