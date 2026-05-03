//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

pub use crate::coercion::{Coercion, CoercionRegistry};
pub use crate::equation::{DecisionTree, Equation, EquationCompiler, Pattern};
pub use crate::tactic::{
    eval_tactic_block, tactic_apply, tactic_by_contra, tactic_cases, tactic_contrapose,
    tactic_exact, tactic_induction, tactic_intro, tactic_push_neg, tactic_split, Goal, Tactic,
    TacticError, TacticRegistry, TacticResult, TacticState,
};
pub use crate::typeclass::{Instance, Method, TypeClass, TypeClassRegistry};
use oxilean_kernel::{Expr, Literal, Name};

use super::functions::ElabPass;

/// A registry for declaration attributes.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct AttributeRegistry {
    attrs: Vec<DeclAttribute>,
}
impl AttributeRegistry {
    /// Create a new empty attribute registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Register an attribute.
    #[allow(dead_code)]
    pub fn register(&mut self, attr: DeclAttribute) {
        self.attrs.push(attr);
    }
    /// Find all attributes for a declaration.
    #[allow(dead_code)]
    pub fn attrs_of(&self, decl: &oxilean_kernel::Name) -> Vec<&DeclAttribute> {
        self.attrs.iter().filter(|a| &a.decl == decl).collect()
    }
    /// Find all declarations with a given attribute name.
    #[allow(dead_code)]
    pub fn decls_with(&self, attr_name: &str) -> Vec<&oxilean_kernel::Name> {
        self.attrs
            .iter()
            .filter(|a| a.name == attr_name)
            .map(|a| &a.decl)
            .collect()
    }
    /// Return the total number of attributes.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.attrs.len()
    }
    /// Whether there are no attributes.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.attrs.is_empty()
    }
}
/// A registry of coercions.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct CoercionRegistryExt {
    coercions: Vec<CoercionExt>,
}
impl CoercionRegistryExt {
    /// Create an empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a coercion.
    #[allow(dead_code)]
    pub fn register(&mut self, coercion: CoercionExt) {
        self.coercions.push(coercion);
    }
    /// Find a coercion from one type to another.
    #[allow(dead_code)]
    pub fn find(
        &self,
        from: &oxilean_kernel::Name,
        to: &oxilean_kernel::Name,
    ) -> Option<&CoercionExt> {
        self.coercions
            .iter()
            .filter(|c| &c.from_type == from && &c.to_type == to)
            .max_by_key(|c| c.priority)
    }
    /// Return all coercions from a given type.
    #[allow(dead_code)]
    pub fn coercions_from(&self, from: &oxilean_kernel::Name) -> Vec<&CoercionExt> {
        self.coercions
            .iter()
            .filter(|c| &c.from_type == from)
            .collect()
    }
    /// Return the number of registered coercions.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.coercions.len()
    }
    /// Whether there are no coercions.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.coercions.is_empty()
    }
    /// Remove all coercions from type `from`.
    #[allow(dead_code)]
    pub fn remove_from(&mut self, from: &oxilean_kernel::Name) {
        self.coercions.retain(|c| &c.from_type != from);
    }
}
/// Global configuration for the elaboration pipeline.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ElabConfig {
    /// Maximum depth for elaboration recursion.
    pub max_depth: u32,
    /// Whether to use proof irrelevance when elaborating proofs.
    pub proof_irrelevance: bool,
    /// Whether to insert implicit arguments automatically.
    pub auto_implicit: bool,
    /// Whether to report incomplete instances as errors.
    pub strict_instances: bool,
    /// Maximum number of tactic steps per proof.
    pub max_tactic_steps: u32,
    /// Whether to enable tracing for debugging.
    pub trace_elaboration: bool,
    /// Whether to run the kernel type checker after elaboration.
    pub kernel_check: bool,
    /// Whether to allow sorry (placeholder proofs).
    pub allow_sorry: bool,
    /// Universe polymorphism level limit.
    pub max_universe_level: u32,
}
impl ElabConfig {
    /// Create a configuration suitable for interactive / IDE use.
    #[allow(dead_code)]
    pub fn interactive() -> Self {
        Self {
            allow_sorry: true,
            strict_instances: false,
            ..Self::default()
        }
    }
    /// Create a strict configuration for verified builds.
    #[allow(dead_code)]
    pub fn strict() -> Self {
        Self {
            allow_sorry: false,
            strict_instances: true,
            kernel_check: true,
            ..Self::default()
        }
    }
    /// Create a debug-tracing configuration.
    #[allow(dead_code)]
    pub fn debug() -> Self {
        Self {
            trace_elaboration: true,
            ..Self::default()
        }
    }
    /// Create a configuration for batch/compilation use.
    #[allow(dead_code)]
    pub fn batch() -> Self {
        Self {
            allow_sorry: false,
            strict_instances: true,
            kernel_check: true,
            trace_elaboration: false,
            ..Self::default()
        }
    }
}
/// The kind of a top-level declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DeclKind {
    /// A definition (`def`).
    Def,
    /// A theorem (`theorem`).
    Theorem,
    /// An axiom (`axiom`).
    Axiom,
    /// An inductive type declaration.
    Inductive,
    /// A structure declaration.
    Structure,
    /// A class declaration.
    Class,
    /// An instance declaration.
    Instance,
    /// A namespace declaration.
    Namespace,
    /// An abbreviation (`abbrev`).
    Abbrev,
    /// A noncomputable definition.
    Noncomputable,
    /// An opaque definition.
    Opaque,
}
impl DeclKind {
    /// Return the keyword for this declaration kind.
    #[allow(dead_code)]
    pub fn keyword(&self) -> &'static str {
        match self {
            DeclKind::Def => "def",
            DeclKind::Theorem => "theorem",
            DeclKind::Axiom => "axiom",
            DeclKind::Inductive => "inductive",
            DeclKind::Structure => "structure",
            DeclKind::Class => "class",
            DeclKind::Instance => "instance",
            DeclKind::Namespace => "namespace",
            DeclKind::Abbrev => "abbrev",
            DeclKind::Noncomputable => "noncomputable",
            DeclKind::Opaque => "opaque",
        }
    }
    /// Whether this declaration kind produces a term.
    #[allow(dead_code)]
    pub fn produces_term(&self) -> bool {
        matches!(
            self,
            DeclKind::Def
                | DeclKind::Theorem
                | DeclKind::Axiom
                | DeclKind::Abbrev
                | DeclKind::Noncomputable
                | DeclKind::Opaque
        )
    }
    /// Whether this declaration kind requires a proof.
    #[allow(dead_code)]
    pub fn requires_proof(&self) -> bool {
        matches!(self, DeclKind::Theorem)
    }
    /// Whether this declaration is computable.
    #[allow(dead_code)]
    pub fn is_computable(&self) -> bool {
        !matches!(self, DeclKind::Noncomputable | DeclKind::Axiom)
    }
}
/// Metrics collected during elaboration.
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct ElabMetrics {
    /// Total number of declarations elaborated.
    pub declarations_elaborated: u64,
    /// Total number of tactics executed.
    pub tactics_executed: u64,
    /// Number of sorry usages.
    pub sorry_count: u64,
    /// Number of unification steps.
    pub unification_steps: u64,
    /// Number of metavariables created.
    pub metavars_created: u64,
    /// Number of metavariables solved.
    pub metavars_solved: u64,
    /// Total inference steps.
    pub inference_steps: u64,
    /// Elaboration failures.
    pub failures: u64,
}
impl ElabMetrics {
    /// Create zeroed metrics.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record one declaration.
    #[allow(dead_code)]
    pub fn record_decl(&mut self) {
        self.declarations_elaborated += 1;
    }
    /// Record one tactic.
    #[allow(dead_code)]
    pub fn record_tactic(&mut self) {
        self.tactics_executed += 1;
    }
    /// Record a sorry usage.
    #[allow(dead_code)]
    pub fn record_sorry(&mut self) {
        self.sorry_count += 1;
    }
    /// Record a failure.
    #[allow(dead_code)]
    pub fn record_failure(&mut self) {
        self.failures += 1;
    }
    /// Return the solve rate (metavars_solved / metavars_created).
    #[allow(dead_code)]
    pub fn solve_rate(&self) -> f64 {
        if self.metavars_created == 0 {
            1.0
        } else {
            self.metavars_solved as f64 / self.metavars_created as f64
        }
    }
    /// Merge another metrics record into this one.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &ElabMetrics) {
        self.declarations_elaborated += other.declarations_elaborated;
        self.tactics_executed += other.tactics_executed;
        self.sorry_count += other.sorry_count;
        self.unification_steps += other.unification_steps;
        self.metavars_created += other.metavars_created;
        self.metavars_solved += other.metavars_solved;
        self.inference_steps += other.inference_steps;
        self.failures += other.failures;
    }
}
/// A namespace entry.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NamespaceEntry {
    /// Fully-qualified name of this namespace.
    pub name: oxilean_kernel::Name,
    /// Whether the namespace is currently open.
    pub is_open: bool,
    /// Parent namespace (None = root).
    pub parent: Option<oxilean_kernel::Name>,
}
impl NamespaceEntry {
    /// Create a new namespace entry.
    #[allow(dead_code)]
    pub fn new(name: oxilean_kernel::Name, parent: Option<oxilean_kernel::Name>) -> Self {
        Self {
            name,
            is_open: false,
            parent,
        }
    }
}
/// Pipeline configuration registry.
///
/// Allows registering custom elaboration passes to be run at specific stages.
#[allow(dead_code)]
#[derive(Default)]
pub struct ElabPipelineRegistry {
    /// Pre-processing passes run before type inference.
    pre_passes: Vec<String>,
    /// Post-processing passes run after type inference.
    post_passes: Vec<String>,
    /// Tactic preprocessing passes.
    tactic_passes: Vec<String>,
}
impl ElabPipelineRegistry {
    /// Create a new empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Register a pre-processing pass.
    #[allow(dead_code)]
    pub fn add_pre_pass(&mut self, pass_name: impl Into<String>) {
        self.pre_passes.push(pass_name.into());
    }
    /// Register a post-processing pass.
    #[allow(dead_code)]
    pub fn add_post_pass(&mut self, pass_name: impl Into<String>) {
        self.post_passes.push(pass_name.into());
    }
    /// Register a tactic preprocessing pass.
    #[allow(dead_code)]
    pub fn add_tactic_pass(&mut self, pass_name: impl Into<String>) {
        self.tactic_passes.push(pass_name.into());
    }
    /// Get number of registered pre-passes.
    #[allow(dead_code)]
    pub fn num_pre_passes(&self) -> usize {
        self.pre_passes.len()
    }
    /// Get number of registered post-passes.
    #[allow(dead_code)]
    pub fn num_post_passes(&self) -> usize {
        self.post_passes.len()
    }
    /// Get number of registered tactic passes.
    #[allow(dead_code)]
    pub fn num_tactic_passes(&self) -> usize {
        self.tactic_passes.len()
    }
    /// Get all pass names (pre + post + tactic).
    #[allow(dead_code)]
    pub fn all_passes(&self) -> Vec<&str> {
        self.pre_passes
            .iter()
            .chain(self.post_passes.iter())
            .chain(self.tactic_passes.iter())
            .map(|s| s.as_str())
            .collect()
    }
}
/// A pipeline of elaboration passes applied in sequence.
#[allow(dead_code)]
pub struct ElabPipeline {
    passes: Vec<Box<dyn ElabPass>>,
    enabled: Vec<bool>,
}
impl ElabPipeline {
    /// Create an empty pipeline.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
            enabled: Vec::new(),
        }
    }
    /// Add a pass to the pipeline.
    #[allow(dead_code)]
    pub fn add<P: ElabPass + 'static>(&mut self, pass: P) {
        let enabled = pass.enabled_by_default();
        self.passes.push(Box::new(pass));
        self.enabled.push(enabled);
    }
    /// Enable or disable a pass by index.
    #[allow(dead_code)]
    pub fn set_enabled(&mut self, idx: usize, enabled: bool) {
        if let Some(e) = self.enabled.get_mut(idx) {
            *e = enabled;
        }
    }
    /// Run all enabled passes in sequence.
    #[allow(dead_code)]
    pub fn run_all(&self, expr: oxilean_kernel::Expr) -> Result<oxilean_kernel::Expr, String> {
        let mut current = expr;
        for (pass, &enabled) in self.passes.iter().zip(self.enabled.iter()) {
            if enabled {
                current = pass.run(current)?;
            }
        }
        Ok(current)
    }
    /// Return the number of passes.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.passes.len()
    }
    /// Whether the pipeline is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.passes.is_empty()
    }
}
/// A history of proof state snapshots for undo support.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct ProofHistory {
    snapshots: Vec<ProofStateSnapshot>,
    current: usize,
}
impl ProofHistory {
    /// Create an empty history.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new snapshot.
    #[allow(dead_code)]
    pub fn push(&mut self, snap: ProofStateSnapshot) {
        self.snapshots.truncate(self.current);
        self.snapshots.push(snap);
        self.current = self.snapshots.len();
    }
    /// Undo to the previous snapshot.
    #[allow(dead_code)]
    pub fn undo(&mut self) -> Option<&ProofStateSnapshot> {
        if self.current > 1 {
            self.current -= 1;
            self.snapshots.get(self.current - 1)
        } else {
            None
        }
    }
    /// Redo to the next snapshot.
    #[allow(dead_code)]
    pub fn redo(&mut self) -> Option<&ProofStateSnapshot> {
        if self.current < self.snapshots.len() {
            self.current += 1;
            self.snapshots.get(self.current - 1)
        } else {
            None
        }
    }
    /// Return the current snapshot.
    #[allow(dead_code)]
    pub fn current(&self) -> Option<&ProofStateSnapshot> {
        self.snapshots.get(self.current.saturating_sub(1))
    }
    /// Return the total number of snapshots.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }
    /// Whether the history is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }
}
/// Statistics collected during an elaboration run.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ElabStats {
    /// Number of declarations elaborated.
    pub num_decls: usize,
    /// Number of metavariables created.
    pub num_mvars_created: usize,
    /// Number of metavariables solved.
    pub num_mvars_solved: usize,
    /// Number of unification constraints solved.
    pub num_unifications: usize,
    /// Number of tactic steps executed.
    pub num_tactic_steps: usize,
    /// Number of instance lookups performed.
    pub num_instance_lookups: usize,
    /// Number of sorry placeholders encountered.
    pub num_sorry: usize,
    /// Number of coercions inserted.
    pub num_coercions: usize,
    /// Maximum recursion depth reached.
    pub max_depth_reached: u32,
}
impl ElabStats {
    /// Create a fresh stats instance.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Merge another stats object into this one.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: &ElabStats) {
        self.num_decls += other.num_decls;
        self.num_mvars_created += other.num_mvars_created;
        self.num_mvars_solved += other.num_mvars_solved;
        self.num_unifications += other.num_unifications;
        self.num_tactic_steps += other.num_tactic_steps;
        self.num_instance_lookups += other.num_instance_lookups;
        self.num_sorry += other.num_sorry;
        self.num_coercions += other.num_coercions;
        self.max_depth_reached = self.max_depth_reached.max(other.max_depth_reached);
    }
    /// Return the mvar solve rate as a fraction in [0, 1].
    #[allow(dead_code)]
    pub fn mvar_solve_rate(&self) -> f64 {
        if self.num_mvars_created == 0 {
            1.0
        } else {
            self.num_mvars_solved as f64 / self.num_mvars_created as f64
        }
    }
    /// Check if all created metavariables were solved.
    #[allow(dead_code)]
    pub fn all_mvars_solved(&self) -> bool {
        self.num_mvars_created == self.num_mvars_solved
    }
}
/// A collection of elaboration notes for a single declaration.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ElabNoteSet {
    notes: Vec<ElabNote>,
}
impl ElabNoteSet {
    /// Create an empty note set.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a note.
    #[allow(dead_code)]
    pub fn add(&mut self, note: ElabNote) {
        self.notes.push(note);
    }
    /// Add a hint.
    #[allow(dead_code)]
    pub fn add_hint(&mut self, msg: impl Into<String>) {
        self.add(ElabNote::Hint(msg.into()));
    }
    /// Add a warning.
    #[allow(dead_code)]
    pub fn add_warning(&mut self, msg: impl Into<String>) {
        self.add(ElabNote::Warning(msg.into()));
    }
    /// Add an info.
    #[allow(dead_code)]
    pub fn add_info(&mut self, msg: impl Into<String>) {
        self.add(ElabNote::Info(msg.into()));
    }
    /// Record a sorry usage.
    #[allow(dead_code)]
    pub fn add_sorry(&mut self, decl: impl Into<String>) {
        self.add(ElabNote::SorryUsed {
            declaration: decl.into(),
        });
    }
    /// Count notes of all types.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.notes.len()
    }
    /// Check if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.notes.is_empty()
    }
    /// Check if there are any warning-like notes.
    #[allow(dead_code)]
    pub fn has_warnings(&self) -> bool {
        self.notes.iter().any(|n| n.is_warning_like())
    }
    /// Collect all warning-like notes.
    #[allow(dead_code)]
    pub fn warnings(&self) -> Vec<&ElabNote> {
        self.notes.iter().filter(|n| n.is_warning_like()).collect()
    }
    /// Iterate over all notes.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &ElabNote> {
        self.notes.iter()
    }
    /// Merge another note set into this one.
    #[allow(dead_code)]
    pub fn merge(&mut self, other: ElabNoteSet) {
        self.notes.extend(other.notes);
    }
    /// Clear all notes.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.notes.clear();
    }
}
/// A coercion rule: how to convert from type A to type B.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CoercionExt {
    /// Source type name.
    pub from_type: oxilean_kernel::Name,
    /// Target type name.
    pub to_type: oxilean_kernel::Name,
    /// The coercion function (constant name).
    pub coercion_fn: oxilean_kernel::Name,
    /// Priority (higher = preferred).
    pub priority: u32,
}
impl CoercionExt {
    /// Create a new coercion.
    #[allow(dead_code)]
    pub fn new(
        from_type: oxilean_kernel::Name,
        to_type: oxilean_kernel::Name,
        coercion_fn: oxilean_kernel::Name,
    ) -> Self {
        Self {
            from_type,
            to_type,
            coercion_fn,
            priority: 0,
        }
    }
    /// Set priority.
    #[allow(dead_code)]
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
    /// Apply this coercion to an expression.
    #[allow(dead_code)]
    pub fn apply(&self, expr: oxilean_kernel::Expr) -> oxilean_kernel::Expr {
        use oxilean_kernel::Expr;
        Expr::App(
            Box::new(Expr::Const(self.coercion_fn.clone(), vec![])),
            Box::new(expr),
        )
    }
}
/// Configuration for the elaborator.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ElabConfigExt {
    /// Maximum number of metavariables to create.
    pub max_metavars: usize,
    /// Maximum recursion depth for type inference.
    pub max_depth: u32,
    /// Whether to emit sorry warnings.
    pub warn_sorry: bool,
    /// Whether to check for unused hypotheses.
    pub check_unused_hyps: bool,
    /// Whether to allow sorry at all.
    pub allow_sorry: bool,
    /// Whether to run coercion resolution.
    pub resolve_coercions: bool,
    /// Whether bidirectional type checking is enabled.
    pub bidir_checking: bool,
    /// Universe checking mode.
    pub universe_checking: UniverseCheckMode,
}
impl ElabConfigExt {
    /// Create a new configuration with defaults.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Create a strict configuration (no sorry, full universe checking).
    #[allow(dead_code)]
    pub fn strict() -> Self {
        Self {
            allow_sorry: false,
            warn_sorry: true,
            check_unused_hyps: true,
            universe_checking: UniverseCheckMode::Full,
            ..Self::default()
        }
    }
    /// Create a permissive configuration for prototyping.
    #[allow(dead_code)]
    pub fn permissive() -> Self {
        Self {
            allow_sorry: true,
            warn_sorry: false,
            check_unused_hyps: false,
            universe_checking: UniverseCheckMode::Skip,
            ..Self::default()
        }
    }
    /// Check if sorry is both allowed and warned about.
    #[allow(dead_code)]
    pub fn sorry_warned(&self) -> bool {
        self.allow_sorry && self.warn_sorry
    }
}
/// Reducibility annotation for a definition.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Reducibility {
    /// Always unfold (e.g., `abbrev`, inline lets).
    Reducible = 0,
    /// Unfold on semi-transparent passes.
    #[default]
    SemiReducible = 1,
    /// Never unfold unless explicitly requested.
    Irreducible = 2,
}
impl Reducibility {
    /// Check if the definition is always unfolded.
    #[allow(dead_code)]
    pub fn is_reducible(&self) -> bool {
        *self == Reducibility::Reducible
    }
    /// Check if the definition is never unfolded.
    #[allow(dead_code)]
    pub fn is_irreducible(&self) -> bool {
        *self == Reducibility::Irreducible
    }
    /// The attribute name corresponding to this reducibility level.
    #[allow(dead_code)]
    pub fn attr_name(&self) -> &'static str {
        match self {
            Reducibility::Reducible => "reducible",
            Reducibility::SemiReducible => "semireducible",
            Reducibility::Irreducible => "irreducible",
        }
    }
}
/// A type class instance record.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ClassInstance {
    /// The class name.
    pub class: oxilean_kernel::Name,
    /// The instance name.
    pub instance: oxilean_kernel::Name,
    /// Type parameters that this instance applies to.
    pub type_params: Vec<oxilean_kernel::Expr>,
    /// Priority for instance selection.
    pub priority: u32,
    /// Whether this is a default instance.
    pub is_default: bool,
}
impl ClassInstance {
    /// Create a new class instance.
    #[allow(dead_code)]
    pub fn new(class: oxilean_kernel::Name, instance: oxilean_kernel::Name) -> Self {
        Self {
            class,
            instance,
            type_params: Vec::new(),
            priority: 100,
            is_default: false,
        }
    }
    /// Set as a default instance.
    #[allow(dead_code)]
    pub fn as_default(mut self) -> Self {
        self.is_default = true;
        self
    }
    /// Set priority.
    #[allow(dead_code)]
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
    /// Add a type parameter.
    #[allow(dead_code)]
    pub fn with_type_param(mut self, param: oxilean_kernel::Expr) -> Self {
        self.type_params.push(param);
        self
    }
}
/// A registry for type class instances.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct InstanceRegistry {
    instances: Vec<ClassInstance>,
}
impl InstanceRegistry {
    /// Create a new empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Register an instance.
    #[allow(dead_code)]
    pub fn register(&mut self, instance: ClassInstance) {
        self.instances.push(instance);
    }
    /// Find instances of a given class.
    #[allow(dead_code)]
    pub fn instances_of(&self, class: &oxilean_kernel::Name) -> Vec<&ClassInstance> {
        let mut results: Vec<&ClassInstance> = self
            .instances
            .iter()
            .filter(|i| &i.class == class)
            .collect();
        results.sort_by_key(|b| std::cmp::Reverse(b.priority));
        results
    }
    /// Find the default instance of a given class.
    #[allow(dead_code)]
    pub fn default_instance(&self, class: &oxilean_kernel::Name) -> Option<&ClassInstance> {
        self.instances_of(class).into_iter().find(|i| i.is_default)
    }
    /// Return the total number of instances.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.instances.len()
    }
    /// Whether there are no instances.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }
    /// Remove all instances of a given class.
    #[allow(dead_code)]
    pub fn remove_class(&mut self, class: &oxilean_kernel::Name) {
        self.instances.retain(|i| &i.class != class);
    }
}
/// Represents a named stage in the elaboration pipeline.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElabStage {
    /// Name resolution.
    NameResolution,
    /// Type inference.
    TypeInference,
    /// Implicit argument resolution.
    ImplicitArgs,
    /// Typeclass instance resolution.
    InstanceResolution,
    /// Higher-order unification.
    Unification,
    /// Coercion insertion.
    Coercions,
    /// Macro expansion.
    MacroExpansion,
    /// Tactic execution.
    TacticExecution,
    /// Kernel validation.
    KernelValidation,
}
impl ElabStage {
    /// Get all stages in pipeline order.
    #[allow(dead_code)]
    pub fn all_in_order() -> &'static [ElabStage] {
        &[
            ElabStage::NameResolution,
            ElabStage::TypeInference,
            ElabStage::ImplicitArgs,
            ElabStage::InstanceResolution,
            ElabStage::Unification,
            ElabStage::Coercions,
            ElabStage::MacroExpansion,
            ElabStage::TacticExecution,
            ElabStage::KernelValidation,
        ]
    }
    /// Get a short name for this stage.
    #[allow(dead_code)]
    pub fn name(&self) -> &'static str {
        match self {
            ElabStage::NameResolution => "name_resolution",
            ElabStage::TypeInference => "type_inference",
            ElabStage::ImplicitArgs => "implicit_args",
            ElabStage::InstanceResolution => "instance_resolution",
            ElabStage::Unification => "unification",
            ElabStage::Coercions => "coercions",
            ElabStage::MacroExpansion => "macro_expansion",
            ElabStage::TacticExecution => "tactic_execution",
            ElabStage::KernelValidation => "kernel_validation",
        }
    }
}
/// A snapshot of a tactic proof state (for undo/redo).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ProofStateSnapshot {
    /// Snapshot ID.
    pub id: u64,
    /// Description of the state.
    pub description: String,
    /// Number of remaining goals.
    pub goal_count: usize,
    /// Names of current hypotheses.
    pub hypothesis_names: Vec<oxilean_kernel::Name>,
}
impl ProofStateSnapshot {
    /// Create a new snapshot.
    #[allow(dead_code)]
    pub fn new(
        id: u64,
        description: impl Into<String>,
        goal_count: usize,
        hypothesis_names: Vec<oxilean_kernel::Name>,
    ) -> Self {
        Self {
            id,
            description: description.into(),
            goal_count,
            hypothesis_names,
        }
    }
    /// Whether the proof is complete (0 goals remaining).
    #[allow(dead_code)]
    pub fn is_complete(&self) -> bool {
        self.goal_count == 0
    }
}
/// An attribute that can be attached to declarations.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeclAttribute {
    /// Attribute name.
    pub name: String,
    /// Optional argument.
    pub arg: Option<String>,
    /// The declaration this attribute applies to.
    pub decl: oxilean_kernel::Name,
}
impl DeclAttribute {
    /// Create a new attribute.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, decl: oxilean_kernel::Name) -> Self {
        Self {
            name: name.into(),
            arg: None,
            decl,
        }
    }
    /// Attach an argument.
    #[allow(dead_code)]
    pub fn with_arg(mut self, arg: impl Into<String>) -> Self {
        self.arg = Some(arg.into());
        self
    }
}
/// A namespace manager tracking open namespaces.
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct NamespaceManager {
    namespaces: Vec<NamespaceEntry>,
    open_stack: Vec<oxilean_kernel::Name>,
}
impl NamespaceManager {
    /// Create a new namespace manager.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Open a namespace.
    #[allow(dead_code)]
    pub fn open(&mut self, name: oxilean_kernel::Name) {
        if !self.namespaces.iter().any(|ns| ns.name == name) {
            self.namespaces
                .push(NamespaceEntry::new(name.clone(), self.current_namespace()));
        }
        if let Some(ns) = self.namespaces.iter_mut().find(|ns| ns.name == name) {
            ns.is_open = true;
        }
        self.open_stack.push(name);
    }
    /// Close the current namespace.
    #[allow(dead_code)]
    pub fn close(&mut self) -> Option<oxilean_kernel::Name> {
        if let Some(name) = self.open_stack.pop() {
            if let Some(ns) = self.namespaces.iter_mut().find(|ns| ns.name == name) {
                ns.is_open = !self.open_stack.contains(&name);
            }
            Some(name)
        } else {
            None
        }
    }
    /// Return the current namespace (innermost open).
    #[allow(dead_code)]
    pub fn current_namespace(&self) -> Option<oxilean_kernel::Name> {
        self.open_stack.last().cloned()
    }
    /// Return all open namespaces.
    #[allow(dead_code)]
    pub fn open_namespaces(&self) -> &[oxilean_kernel::Name] {
        &self.open_stack
    }
    /// Qualify a name with the current namespace.
    #[allow(dead_code)]
    pub fn qualify(&self, name: &str) -> oxilean_kernel::Name {
        match self.current_namespace() {
            Some(ns) => oxilean_kernel::Name::str(format!("{}.{}", ns, name)),
            None => oxilean_kernel::Name::str(name),
        }
    }
    /// Return the depth of namespace nesting.
    #[allow(dead_code)]
    pub fn depth(&self) -> usize {
        self.open_stack.len()
    }
}
/// A snapshot of the elaboration environment at a point in time.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EnvSnapshot {
    /// Snapshot ID.
    pub id: u64,
    /// Number of declarations in the environment.
    pub decl_count: usize,
    /// Description.
    pub description: String,
}
impl EnvSnapshot {
    /// Create a new environment snapshot.
    #[allow(dead_code)]
    pub fn new(id: u64, decl_count: usize, description: impl Into<String>) -> Self {
        Self {
            id,
            decl_count,
            description: description.into(),
        }
    }
}
/// Structured error codes for elaboration failures.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElabErrorCode {
    /// A name was not found in scope.
    UnknownName,
    /// Type mismatch: expected vs. actual type differ.
    TypeMismatch,
    /// A metavariable could not be solved.
    UnsolvedMvar,
    /// Multiple typeclass instances match.
    AmbiguousInstance,
    /// No typeclass instance found.
    NoInstance,
    /// Unification failed.
    UnificationFailed,
    /// Expression is ill-typed.
    IllTyped,
    /// Tactic execution failed.
    TacticFailed,
    /// Pattern matching is not exhaustive.
    NonExhaustiveMatch,
    /// Syntax error (propagated from parser).
    SyntaxError,
    /// The kernel rejected the term.
    KernelRejected,
    /// Sorry was used but not allowed.
    SorryNotAllowed,
    /// Recursion limit exceeded.
    RecursionLimit,
    /// Mutual recursion cycle detected.
    MutualCycle,
    /// Other/unclassified error.
    Other,
}
/// Universe checking mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum UniverseCheckMode {
    /// Fully check universe polymorphism.
    Full,
    /// Only check that sorts are well-formed.
    Partial,
    /// Skip universe checking (unsafe).
    Skip,
}
/// A manager for environment snapshots (for module-level rollback).
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct EnvSnapshotManager {
    snapshots: Vec<EnvSnapshot>,
    next_id: u64,
}
impl EnvSnapshotManager {
    /// Create a new snapshot manager.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Take a new snapshot.
    #[allow(dead_code)]
    pub fn take(&mut self, decl_count: usize, description: impl Into<String>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.snapshots
            .push(EnvSnapshot::new(id, decl_count, description));
        id
    }
    /// Find a snapshot by ID.
    #[allow(dead_code)]
    pub fn get(&self, id: u64) -> Option<&EnvSnapshot> {
        self.snapshots.iter().find(|s| s.id == id)
    }
    /// Return all snapshots.
    #[allow(dead_code)]
    pub fn all(&self) -> &[EnvSnapshot] {
        &self.snapshots
    }
    /// Return the most recent snapshot.
    #[allow(dead_code)]
    pub fn latest(&self) -> Option<&EnvSnapshot> {
        self.snapshots.last()
    }
    /// Return the number of snapshots.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.snapshots.len()
    }
    /// Whether there are no snapshots.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.snapshots.is_empty()
    }
}
/// A structured note (hint, warning, or info) attached to an elaborated item.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElabNote {
    /// Hint about a potential improvement.
    Hint(String),
    /// Warning about a potential problem.
    Warning(String),
    /// Informational message.
    Info(String),
    /// A sorry was used.
    SorryUsed {
        /// The declaration that used sorry.
        declaration: String,
    },
    /// Implicit universe was introduced.
    ImplicitUniverse(String),
}
impl ElabNote {
    /// Return a short prefix for display.
    #[allow(dead_code)]
    pub fn prefix(&self) -> &'static str {
        match self {
            ElabNote::Hint(_) => "hint",
            ElabNote::Warning(_) => "warning",
            ElabNote::Info(_) => "info",
            ElabNote::SorryUsed { .. } => "sorry",
            ElabNote::ImplicitUniverse(_) => "universe",
        }
    }
    /// The message text.
    #[allow(dead_code)]
    pub fn message(&self) -> &str {
        match self {
            ElabNote::Hint(s)
            | ElabNote::Warning(s)
            | ElabNote::Info(s)
            | ElabNote::ImplicitUniverse(s) => s,
            ElabNote::SorryUsed { declaration } => declaration,
        }
    }
    /// Whether this note is a warning or sorry.
    #[allow(dead_code)]
    pub fn is_warning_like(&self) -> bool {
        matches!(self, ElabNote::Warning(_) | ElabNote::SorryUsed { .. })
    }
}
