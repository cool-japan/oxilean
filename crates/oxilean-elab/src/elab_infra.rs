//! Elaboration infrastructure: passes, pipelines, metrics, declarations,
//! proof state, coercions, instances, attributes, namespaces, and expression helpers.

#[allow(unused_imports)]
use oxilean_kernel::{Expr, Literal, Name};

// Import types from elab_types module
use super::elab_types::*;

// ============================================================================
// Elaboration pass infrastructure
// ============================================================================

/// A named elaboration pass that transforms an expression.
#[allow(dead_code)]
pub trait ElabPass {
    /// Name of this pass.
    fn name(&self) -> &str;

    /// Run the pass on an expression, returning the (possibly transformed) result.
    fn run(&self, expr: oxilean_kernel::Expr) -> Result<oxilean_kernel::Expr, String>;

    /// Whether this pass is enabled by default.
    fn enabled_by_default(&self) -> bool {
        true
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

impl Default for ElabPipeline {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Elaboration configuration
// ============================================================================

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

impl Default for ElabConfigExt {
    fn default() -> Self {
        Self {
            max_metavars: 10_000,
            max_depth: 500,
            warn_sorry: true,
            check_unused_hyps: false,
            allow_sorry: true,
            resolve_coercions: true,
            bidir_checking: true,
            universe_checking: UniverseCheckMode::Partial,
        }
    }
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

// ============================================================================
// Elaboration metrics
// ============================================================================

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

// ============================================================================
// Declaration kind classification
// ============================================================================

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

impl std::fmt::Display for DeclKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.keyword())
    }
}

// ============================================================================
// Tactic proof state snapshot
// ============================================================================

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
        // Remove any forward history
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

// ============================================================================
// Coercion infrastructure
// ============================================================================

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

// ============================================================================
// Type class instance resolution
// ============================================================================

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
        results.sort_by_key(|r| std::cmp::Reverse(r.priority));
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

// ============================================================================
// Attribute registry
// ============================================================================

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

// ============================================================================
// Namespace management
// ============================================================================

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
        // Register if not already known
        if !self.namespaces.iter().any(|ns| ns.name == name) {
            self.namespaces
                .push(NamespaceEntry::new(name.clone(), self.current_namespace()));
        }
        // Mark as open
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

// ============================================================================
// Expression pretty-printing helpers
// ============================================================================

/// Format a kernel expression as a human-readable string.
#[allow(dead_code)]
pub fn pretty_expr(expr: &oxilean_kernel::Expr) -> String {
    match expr {
        Expr::Sort(l) => format!("Sort({:?})", l),
        Expr::BVar(i) => format!("#{}", i),
        Expr::FVar(fv) => format!("@{}", fv.0),
        Expr::Const(name, _) => name.to_string(),
        Expr::App(f, a) => format!("({} {})", pretty_expr(f), pretty_expr(a)),
        Expr::Lam(_, name, _ty, body) => {
            format!("(fun {} => {})", name, pretty_expr(body))
        }
        Expr::Pi(_, name, ty, body) => {
            format!(
                "(({} : {}) -> {})",
                name,
                pretty_expr(ty),
                pretty_expr(body)
            )
        }
        Expr::Let(name, _ty, val, body) => {
            format!(
                "(let {} := {} in {})",
                name,
                pretty_expr(val),
                pretty_expr(body)
            )
        }
        Expr::Lit(lit) => {
            use oxilean_kernel::Literal;
            match lit {
                Literal::Nat(n) => format!("{}", n),
                Literal::Str(s) => format!("{:?}", s),
            }
        }
        Expr::Proj(name, idx, inner) => {
            format!("{}.{} ({})", name, idx, pretty_expr(inner))
        }
    }
}

/// Format a list of expressions as a comma-separated string.
#[allow(dead_code)]
pub fn pretty_expr_list(exprs: &[oxilean_kernel::Expr]) -> String {
    exprs.iter().map(pretty_expr).collect::<Vec<_>>().join(", ")
}

// ============================================================================
// Well-foundedness checking helpers
// ============================================================================

/// Check if a declaration name looks like a recursive definition.
///
/// This is a heuristic check — actual recursion analysis happens in the kernel.
#[allow(dead_code)]
pub fn might_be_recursive(name: &oxilean_kernel::Name, body: &oxilean_kernel::Expr) -> bool {
    fn contains_name(expr: &Expr, target: &oxilean_kernel::Name) -> bool {
        match expr {
            Expr::Const(n, _) => n == target,
            Expr::App(f, a) => contains_name(f, target) || contains_name(a, target),
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
                contains_name(ty, target) || contains_name(body, target)
            }
            Expr::Let(_, ty, val, b) => {
                contains_name(ty, target) || contains_name(val, target) || contains_name(b, target)
            }
            Expr::Proj(_, _, inner) => contains_name(inner, target),
            _ => false,
        }
    }
    contains_name(body, name)
}

// ============================================================================
// Tactic name constants module (extended)
// ============================================================================

/// Extended tactic name constants.
#[allow(dead_code)]
pub mod tactic_names_ext {
    /// `norm_num` — numeric normalization.
    pub const NORM_NUM: &str = "norm_num";
    /// `omega` — linear arithmetic over integers.
    pub const OMEGA: &str = "omega";
    /// `decide` — decidable proposition checker.
    pub const DECIDE: &str = "decide";
    /// `native_decide` — faster decide using native code.
    pub const NATIVE_DECIDE: &str = "native_decide";
    /// `aesop` — automated proof search.
    pub const AESOP: &str = "aesop";
    /// `tauto` — propositional tautology prover.
    pub const TAUTO: &str = "tauto";
    /// `fin_cases` — case split on finite types.
    pub const FIN_CASES: &str = "fin_cases";
    /// `interval_cases` — case split on integer intervals.
    pub const INTERVAL_CASES: &str = "interval_cases";
    /// `gcongr` — generalized congruence.
    pub const GCONGR: &str = "gcongr";
    /// `positivity` — prove positivity of expressions.
    pub const POSITIVITY: &str = "positivity";
    /// `polyrith` — polynomial arithmetic.
    pub const POLYRITH: &str = "polyrith";
    /// `linear_combination` — linear combination proof.
    pub const LINEAR_COMBINATION: &str = "linear_combination";
    /// `ext` — extensionality.
    pub const EXT: &str = "ext";
    /// `funext` — function extensionality.
    pub const FUNEXT: &str = "funext";
    /// `congr` — congruence.
    pub const CONGR: &str = "congr";
    /// `unfold` — unfold a definition.
    pub const UNFOLD: &str = "unfold";
    /// `change` — change goal to definitionally equal form.
    pub const CHANGE: &str = "change";
    /// `subst` — substitute a hypothesis.
    pub const SUBST: &str = "subst";
    /// `symm` — symmetry of equality.
    pub const SYMM: &str = "symm";
    /// `trans` — transitivity.
    pub const TRANS: &str = "trans";
    /// `calc` — calculation proof.
    pub const CALC: &str = "calc";
    /// `rcases` — recursive case split.
    pub const RCASES: &str = "rcases";
    /// `rintro` — recursive intro.
    pub const RINTRO: &str = "rintro";
    /// `refine` — partial proof.
    pub const REFINE: &str = "refine";
    /// `ac_rfl` — AC-refl.
    pub const AC_RFL: &str = "ac_rfl";
}

/// Check if a tactic name is a Mathlib-style extended tactic.
#[allow(dead_code)]
pub fn is_mathlib_tactic(name: &str) -> bool {
    matches!(
        name,
        "norm_num"
            | "omega"
            | "decide"
            | "native_decide"
            | "aesop"
            | "tauto"
            | "fin_cases"
            | "interval_cases"
            | "gcongr"
            | "positivity"
            | "polyrith"
            | "linear_combination"
            | "ext"
            | "funext"
            | "congr"
            | "unfold"
            | "change"
            | "subst"
            | "symm"
            | "trans"
            | "calc"
            | "rcases"
            | "rintro"
            | "refine"
            | "ac_rfl"
    )
}

// ============================================================================
// Scoped environment snapshot
// ============================================================================

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

// ============================================================================
// Additional tests
// ============================================================================

#[cfg(test)]
mod lib_extended_tests {
    use super::*;
    use oxilean_kernel::Name;

    #[test]
    fn test_elab_config_defaults() {
        let cfg = ElabConfig::default();
        assert!(!cfg.allow_sorry);
        assert!(cfg.kernel_check);
        assert!(cfg.proof_irrelevance);
        assert!(cfg.auto_implicit);
    }

    #[test]
    fn test_elab_config_strict() {
        let cfg = ElabConfig::strict();
        assert!(!cfg.allow_sorry);
        assert!(cfg.strict_instances);
        assert!(cfg.kernel_check);
    }

    #[test]
    fn test_elab_config_interactive() {
        let cfg = ElabConfig::interactive();
        assert!(cfg.allow_sorry);
        assert!(!cfg.strict_instances);
    }

    #[test]
    fn test_elab_config_batch() {
        let cfg = ElabConfig::batch();
        assert!(!cfg.allow_sorry);
        assert!(cfg.strict_instances);
        assert!(!cfg.trace_elaboration);
    }

    #[test]
    fn test_elab_metrics_solve_rate() {
        let mut m = ElabMetrics::new();
        m.metavars_created = 10;
        m.metavars_solved = 8;
        let rate = m.solve_rate();
        assert!((rate - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_elab_metrics_solve_rate_zero() {
        let m = ElabMetrics::new();
        assert_eq!(m.solve_rate(), 1.0);
    }

    #[test]
    fn test_elab_metrics_merge() {
        let mut a = ElabMetrics::new();
        a.declarations_elaborated = 5;
        let mut b = ElabMetrics::new();
        b.declarations_elaborated = 3;
        a.merge(&b);
        assert_eq!(a.declarations_elaborated, 8);
    }

    #[test]
    fn test_decl_kind_keyword() {
        assert_eq!(DeclKind::Def.keyword(), "def");
        assert_eq!(DeclKind::Theorem.keyword(), "theorem");
        assert_eq!(DeclKind::Axiom.keyword(), "axiom");
    }

    #[test]
    fn test_decl_kind_produces_term() {
        assert!(DeclKind::Def.produces_term());
        assert!(DeclKind::Theorem.produces_term());
        assert!(!DeclKind::Inductive.produces_term());
        assert!(!DeclKind::Namespace.produces_term());
    }

    #[test]
    fn test_decl_kind_requires_proof() {
        assert!(DeclKind::Theorem.requires_proof());
        assert!(!DeclKind::Def.requires_proof());
    }

    #[test]
    fn test_decl_kind_is_computable() {
        assert!(DeclKind::Def.is_computable());
        assert!(!DeclKind::Noncomputable.is_computable());
        assert!(!DeclKind::Axiom.is_computable());
    }

    #[test]
    fn test_proof_history_undo_redo() {
        let mut h = ProofHistory::new();
        assert!(h.is_empty());
        h.push(ProofStateSnapshot::new(0, "start", 2, vec![]));
        h.push(ProofStateSnapshot::new(1, "step1", 1, vec![]));
        h.push(ProofStateSnapshot::new(2, "step2", 0, vec![]));
        assert_eq!(h.len(), 3);

        let prev = h.undo();
        assert!(prev.is_some());
        assert_eq!(prev.expect("test operation should succeed").id, 1);

        let next = h.redo();
        assert!(next.is_some());
        assert_eq!(next.expect("test operation should succeed").id, 2);
    }

    #[test]
    fn test_proof_history_current() {
        let mut h = ProofHistory::new();
        h.push(ProofStateSnapshot::new(0, "start", 1, vec![]));
        assert!(h.current().is_some());
        assert_eq!(h.current().expect("test operation should succeed").id, 0);
        assert!(!h
            .current()
            .expect("test operation should succeed")
            .is_complete());
    }

    #[test]
    fn test_coercion_registry_find() {
        let mut reg = CoercionRegistryExt::new();
        let c = CoercionExt::new(Name::str("Nat"), Name::str("Int"), Name::str("Int.ofNat"));
        reg.register(c);
        assert!(reg.find(&Name::str("Nat"), &Name::str("Int")).is_some());
        assert!(reg.find(&Name::str("Int"), &Name::str("Nat")).is_none());
    }

    #[test]
    fn test_coercion_apply() {
        let c = CoercionExt::new(Name::str("Nat"), Name::str("Int"), Name::str("Int.ofNat"));
        let nat_expr = Expr::Const(Name::str("zero"), vec![]);
        let coerced = c.apply(nat_expr);
        assert!(matches!(coerced, Expr::App(_, _)));
    }

    #[test]
    fn test_instance_registry() {
        let mut reg = InstanceRegistry::new();
        let inst = ClassInstance::new(Name::str("Add"), Name::str("instAddNat")).as_default();
        reg.register(inst);
        assert_eq!(reg.instances_of(&Name::str("Add")).len(), 1);
        assert!(reg.default_instance(&Name::str("Add")).is_some());
    }

    #[test]
    fn test_attribute_registry() {
        let mut reg = AttributeRegistry::new();
        let attr = DeclAttribute::new("simp", Name::str("myLemma")).with_arg("all");
        reg.register(attr);
        assert_eq!(reg.attrs_of(&Name::str("myLemma")).len(), 1);
        assert_eq!(reg.decls_with("simp").len(), 1);
    }

    #[test]
    fn test_namespace_manager() {
        let mut nm = NamespaceManager::new();
        assert_eq!(nm.depth(), 0);
        nm.open(Name::str("Nat"));
        assert_eq!(nm.depth(), 1);
        let q = nm.qualify("succ");
        assert!(q.to_string().contains("succ"));
        nm.close();
        assert_eq!(nm.depth(), 0);
    }

    #[test]
    fn test_pretty_expr() {
        let nat = Expr::Const(Name::str("Nat"), vec![]);
        let s = pretty_expr(&nat);
        assert_eq!(s, "Nat");

        let bvar = Expr::BVar(2);
        let s2 = pretty_expr(&bvar);
        assert!(s2.contains('2'));
    }

    #[test]
    fn test_pretty_expr_list() {
        let exprs = vec![
            Expr::Const(Name::str("a"), vec![]),
            Expr::Const(Name::str("b"), vec![]),
        ];
        let s = pretty_expr_list(&exprs);
        assert!(s.contains("a"));
        assert!(s.contains("b"));
        assert!(s.contains(','));
    }

    #[test]
    fn test_might_be_recursive_yes() {
        let name = Name::str("fib");
        let body = Expr::App(
            Box::new(Expr::Const(Name::str("fib"), vec![])),
            Box::new(Expr::BVar(0)),
        );
        assert!(might_be_recursive(&name, &body));
    }

    #[test]
    fn test_might_be_recursive_no() {
        let name = Name::str("fib");
        let body = Expr::Const(Name::str("Nat.succ"), vec![]);
        assert!(!might_be_recursive(&name, &body));
    }

    #[test]
    fn test_is_mathlib_tactic() {
        assert!(is_mathlib_tactic("omega"));
        assert!(is_mathlib_tactic("norm_num"));
        assert!(is_mathlib_tactic("aesop"));
        assert!(!is_mathlib_tactic("intro"));
        assert!(!is_mathlib_tactic("unknown"));
    }

    #[test]
    fn test_tactic_names_ext_constants() {
        assert_eq!(tactic_names_ext::OMEGA, "omega");
        assert_eq!(tactic_names_ext::NORM_NUM, "norm_num");
        assert_eq!(tactic_names_ext::EXT, "ext");
    }

    #[test]
    fn test_env_snapshot_manager() {
        let mut mgr = EnvSnapshotManager::new();
        assert!(mgr.is_empty());
        let id1 = mgr.take(10, "after module A");
        let _id2 = mgr.take(20, "after module B");
        assert_eq!(mgr.len(), 2);
        let snap = mgr.get(id1).expect("key should exist");
        assert_eq!(snap.decl_count, 10);
        let latest = mgr.latest().expect("test operation should succeed");
        assert_eq!(latest.decl_count, 20);
    }

    #[test]
    fn test_universe_check_mode_equality() {
        assert_eq!(UniverseCheckMode::Full, UniverseCheckMode::Full);
        assert_ne!(UniverseCheckMode::Full, UniverseCheckMode::Skip);
    }

    #[test]
    fn test_coercion_registry_remove_from() {
        let mut reg = CoercionRegistryExt::new();
        reg.register(CoercionExt::new(
            Name::str("Nat"),
            Name::str("Int"),
            Name::str("f"),
        ));
        reg.register(CoercionExt::new(
            Name::str("Nat"),
            Name::str("Real"),
            Name::str("g"),
        ));
        reg.register(CoercionExt::new(
            Name::str("Int"),
            Name::str("Real"),
            Name::str("h"),
        ));
        assert_eq!(reg.len(), 3);
        reg.remove_from(&Name::str("Nat"));
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn test_instance_registry_remove_class() {
        let mut reg = InstanceRegistry::new();
        reg.register(ClassInstance::new(Name::str("Add"), Name::str("addNat")));
        reg.register(ClassInstance::new(Name::str("Add"), Name::str("addInt")));
        reg.register(ClassInstance::new(Name::str("Mul"), Name::str("mulNat")));
        assert_eq!(reg.len(), 3);
        reg.remove_class(&Name::str("Add"));
        assert_eq!(reg.len(), 1);
    }

    #[test]
    fn test_decl_kind_display() {
        let s = format!("{}", DeclKind::Def);
        assert_eq!(s, "def");
    }
}
