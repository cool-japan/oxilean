//! Core types and configurations for the elaboration pipeline.
//
//! This module contains ElabConfig, ElabStats, ElabErrorCode, ElabStage,
//! tactic names, and Reducibility.

use oxilean_kernel::{Expr, Literal, Name};

// ============================================================================
// Elaboration Configuration & Pipeline Settings
// ============================================================================

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

impl Default for ElabConfig {
    fn default() -> Self {
        Self {
            max_depth: 512,
            proof_irrelevance: true,
            auto_implicit: true,
            strict_instances: false,
            max_tactic_steps: 100_000,
            trace_elaboration: false,
            kernel_check: true,
            allow_sorry: false,
            max_universe_level: 100,
        }
    }
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

impl std::fmt::Display for ElabErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ElabErrorCode::UnknownName => "unknown name",
            ElabErrorCode::TypeMismatch => "type mismatch",
            ElabErrorCode::UnsolvedMvar => "unsolved metavariable",
            ElabErrorCode::AmbiguousInstance => "ambiguous instance",
            ElabErrorCode::NoInstance => "no instance found",
            ElabErrorCode::UnificationFailed => "unification failed",
            ElabErrorCode::IllTyped => "ill-typed expression",
            ElabErrorCode::TacticFailed => "tactic failed",
            ElabErrorCode::NonExhaustiveMatch => "non-exhaustive match",
            ElabErrorCode::SyntaxError => "syntax error",
            ElabErrorCode::KernelRejected => "kernel rejected term",
            ElabErrorCode::SorryNotAllowed => "sorry not allowed",
            ElabErrorCode::RecursionLimit => "recursion limit exceeded",
            ElabErrorCode::MutualCycle => "mutual recursion cycle",
            ElabErrorCode::Other => "elaboration error",
        };
        write!(f, "{}", s)
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

/// Well-known attribute names used in elaboration.
#[allow(dead_code)]
pub mod attr_names {
    /// `@[simp]` marks a lemma for use by simp.
    pub const SIMP: &str = "simp";
    /// `@[reducible]` marks a definition as always unfolded.
    pub const REDUCIBLE: &str = "reducible";
    /// `@[semireducible]` default reducibility.
    pub const SEMIREDUCIBLE: &str = "semireducible";
    /// `@[irreducible]` never unfolded.
    pub const IRREDUCIBLE: &str = "irreducible";
    /// `@[inline]` hint to inline during code generation.
    pub const INLINE: &str = "inline";
    /// `@[instance]` typeclass instance.
    pub const INSTANCE: &str = "instance";
    /// `@[class]` typeclass definition.
    pub const CLASS: &str = "class";
    /// `@[derive]` automatic instance derivation.
    pub const DERIVE: &str = "derive";
    /// `@[ext]` extensionality lemma.
    pub const EXT: &str = "ext";
    /// `@[norm_cast]` for norm_cast / push_cast tactics.
    pub const NORM_CAST: &str = "norm_cast";
    /// `@[protected]` name requires qualified access.
    pub const PROTECTED: &str = "protected";
    /// `@[macro]` macro definition.
    pub const MACRO: &str = "macro";
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn test_elab_config_default() {
        let cfg = ElabConfig::default();
        assert_eq!(cfg.max_depth, 512);
        assert!(cfg.kernel_check);
        assert!(!cfg.allow_sorry);
    }

    #[test]
    fn test_elab_config_interactive() {
        let cfg = ElabConfig::interactive();
        assert!(cfg.allow_sorry);
        assert!(!cfg.strict_instances);
    }

    #[test]
    fn test_elab_config_strict() {
        let cfg = ElabConfig::strict();
        assert!(!cfg.allow_sorry);
        assert!(cfg.strict_instances);
        assert!(cfg.kernel_check);
    }

    #[test]
    fn test_elab_config_batch() {
        let cfg = ElabConfig::batch();
        assert!(!cfg.allow_sorry);
        assert!(!cfg.trace_elaboration);
    }

    #[test]
    fn test_elab_config_debug() {
        let cfg = ElabConfig::debug();
        assert!(cfg.trace_elaboration);
    }

    #[test]
    fn test_elab_stats_default() {
        let s = ElabStats::new();
        assert_eq!(s.num_decls, 0);
        assert!(s.all_mvars_solved());
        assert_eq!(s.mvar_solve_rate(), 1.0);
    }

    #[test]
    fn test_elab_stats_merge() {
        let mut s1 = ElabStats {
            num_decls: 3,
            num_mvars_created: 5,
            num_mvars_solved: 5,
            ..Default::default()
        };
        let s2 = ElabStats {
            num_decls: 2,
            num_mvars_created: 3,
            num_mvars_solved: 2,
            max_depth_reached: 100,
            ..Default::default()
        };
        s1.merge(&s2);
        assert_eq!(s1.num_decls, 5);
        assert_eq!(s1.num_mvars_created, 8);
        assert_eq!(s1.max_depth_reached, 100);
    }

    #[test]
    fn test_elab_stats_mvar_rate() {
        let s = ElabStats {
            num_mvars_created: 10,
            num_mvars_solved: 8,
            ..Default::default()
        };
        let rate = s.mvar_solve_rate();
        assert!((rate - 0.8).abs() < 1e-10);
        assert!(!s.all_mvars_solved());
    }

    #[test]
    fn test_elab_error_codes_display() {
        assert_eq!(format!("{}", ElabErrorCode::TypeMismatch), "type mismatch");
        assert_eq!(format!("{}", ElabErrorCode::UnknownName), "unknown name");
        assert_eq!(format!("{}", ElabErrorCode::TacticFailed), "tactic failed");
    }

    #[test]
    fn test_elab_stage_order() {
        let stages = ElabStage::all_in_order();
        assert_eq!(stages.len(), 9);
        assert_eq!(stages[0], ElabStage::NameResolution);
        assert_eq!(stages[8], ElabStage::KernelValidation);
    }

    #[test]
    fn test_elab_stage_names() {
        assert_eq!(ElabStage::Unification.name(), "unification");
        assert_eq!(ElabStage::KernelValidation.name(), "kernel_validation");
    }

    #[test]
    fn test_attr_names() {
        assert_eq!(attr_names::SIMP, "simp");
        assert_eq!(attr_names::INSTANCE, "instance");
        assert_eq!(attr_names::DERIVE, "derive");
    }

    #[test]
    fn test_elab_error_other() {
        assert_eq!(format!("{}", ElabErrorCode::Other), "elaboration error");
    }

    #[test]
    fn test_all_error_variants_display() {
        let variants = [
            ElabErrorCode::UnknownName,
            ElabErrorCode::TypeMismatch,
            ElabErrorCode::UnsolvedMvar,
            ElabErrorCode::AmbiguousInstance,
            ElabErrorCode::NoInstance,
            ElabErrorCode::UnificationFailed,
            ElabErrorCode::IllTyped,
            ElabErrorCode::TacticFailed,
            ElabErrorCode::NonExhaustiveMatch,
            ElabErrorCode::SyntaxError,
            ElabErrorCode::KernelRejected,
            ElabErrorCode::SorryNotAllowed,
            ElabErrorCode::RecursionLimit,
            ElabErrorCode::MutualCycle,
            ElabErrorCode::Other,
        ];
        for v in &variants {
            assert!(!format!("{}", v).is_empty());
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

#[cfg(test)]
mod pipeline_tests {
    use super::*;

    #[test]
    fn test_pipeline_registry_empty() {
        let reg = ElabPipelineRegistry::new();
        assert_eq!(reg.num_pre_passes(), 0);
        assert_eq!(reg.num_post_passes(), 0);
        assert!(reg.all_passes().is_empty());
    }

    #[test]
    fn test_pipeline_registry_add_passes() {
        let mut reg = ElabPipelineRegistry::new();
        reg.add_pre_pass("normalize");
        reg.add_post_pass("kernel_check");
        reg.add_tactic_pass("simp_prep");
        assert_eq!(reg.num_pre_passes(), 1);
        assert_eq!(reg.num_post_passes(), 1);
        assert_eq!(reg.num_tactic_passes(), 1);
        assert_eq!(reg.all_passes().len(), 3);
    }
}

// ============================================================================
// ElabNote: structured notes attached to declarations
// ============================================================================

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

impl std::fmt::Display for ElabNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.prefix(), self.message())
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

// ============================================================================
// Well-known tactic names
// ============================================================================

/// Names of all well-known tactics supported by the elaborator.
#[allow(dead_code)]
pub mod tactic_names {
    /// Introduce a binder into the context.
    pub const INTRO: &str = "intro";
    /// Introduce multiple binders at once.
    pub const INTROS: &str = "intros";
    /// Apply a lemma to the goal.
    pub const APPLY: &str = "apply";
    /// Provide an exact proof term.
    pub const EXACT: &str = "exact";
    /// Close goal by reflexivity.
    pub const REFL: &str = "refl";
    /// Assumption — close by hypothesis.
    pub const ASSUMPTION: &str = "assumption";
    /// Trivially close a trivial goal.
    pub const TRIVIAL: &str = "trivial";
    /// Placeholder proof.
    pub const SORRY: &str = "sorry";
    /// Rewrite goal using equality.
    pub const RW: &str = "rw";
    /// Simplify using simp lemmas.
    pub const SIMP: &str = "simp";
    /// Simp using all hypotheses.
    pub const SIMP_ALL: &str = "simp_all";
    /// Case split.
    pub const CASES: &str = "cases";
    /// Induction.
    pub const INDUCTION: &str = "induction";
    /// Apply first constructor.
    pub const CONSTRUCTOR: &str = "constructor";
    /// Apply left constructor of Or.
    pub const LEFT: &str = "left";
    /// Apply right constructor of Or.
    pub const RIGHT: &str = "right";
    /// Provide existential witness.
    pub const EXISTSI: &str = "existsi";
    /// Use witness (alias for existsi).
    pub const USE: &str = "use";
    /// Push negation inward.
    pub const PUSH_NEG: &str = "push_neg";
    /// By contradiction.
    pub const BY_CONTRA: &str = "by_contra";
    /// Contrapositive.
    pub const CONTRAPOSE: &str = "contrapose";
    /// Split an iff/and goal.
    pub const SPLIT: &str = "split";
    /// Exfalso: change goal to False.
    pub const EXFALSO: &str = "exfalso";
    /// Linear arithmetic.
    pub const LINARITH: &str = "linarith";
    /// Ring simplification.
    pub const RING: &str = "ring";
    /// Norm_cast.
    pub const NORM_CAST: &str = "norm_cast";
    /// Clear a hypothesis.
    pub const CLEAR: &str = "clear";
    /// Have: introduce a new hypothesis with proof.
    pub const HAVE: &str = "have";
    /// Obtain: like cases but with pattern.
    pub const OBTAIN: &str = "obtain";
    /// Show: change the goal type.
    pub const SHOW: &str = "show";
    /// Revert: move hypotheses back to goal.
    pub const REVERT: &str = "revert";
    /// Specialize an applied hypothesis.
    pub const SPECIALIZE: &str = "specialize";
    /// Rename a hypothesis.
    pub const RENAME: &str = "rename";
}

/// Check whether a string is a known tactic name.
#[allow(dead_code)]
pub fn is_known_tactic(name: &str) -> bool {
    matches!(
        name,
        "intro"
            | "intros"
            | "apply"
            | "exact"
            | "refl"
            | "assumption"
            | "trivial"
            | "sorry"
            | "rw"
            | "simp"
            | "simp_all"
            | "cases"
            | "induction"
            | "constructor"
            | "left"
            | "right"
            | "existsi"
            | "use"
            | "push_neg"
            | "by_contra"
            | "by_contradiction"
            | "contrapose"
            | "split"
            | "exfalso"
            | "linarith"
            | "ring"
            | "norm_cast"
            | "clear"
            | "have"
            | "obtain"
            | "show"
            | "revert"
            | "specialize"
            | "rename"
            | "repeat"
            | "first"
            | "try"
            | "all_goals"
            | "any_goals"
            | "field_simp"
            | "push_cast"
            | "exact_mod_cast"
    )
}

/// Return the category of a tactic (proof-search, rewriting, etc.).
#[allow(dead_code)]
pub fn tactic_category(name: &str) -> &'static str {
    match name {
        "intro" | "intros" | "revert" | "clear" | "rename" | "obtain" | "have" | "show" => {
            "context"
        }
        "apply" | "exact" | "assumption" | "trivial" | "sorry" | "refl" => "proof-search",
        "rw" | "simp" | "simp_all" | "field_simp" | "ring" | "linarith" | "norm_cast"
        | "push_cast" | "exact_mod_cast" => "rewriting",
        "cases" | "induction" | "constructor" | "left" | "right" | "existsi" | "use" | "split"
        | "exfalso" => "structure",
        "push_neg" | "by_contra" | "by_contradiction" | "contrapose" => "logic",
        "repeat" | "first" | "try" | "all_goals" | "any_goals" => "combinator",
        "specialize" => "context",
        _ => "unknown",
    }
}

// ============================================================================
// Reducibility hints
// ============================================================================

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

impl std::fmt::Display for Reducibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.attr_name())
    }
}

#[cfg(test)]
mod elab_lib_extra_tests {
    use super::*;

    #[test]
    fn test_elab_note_hint() {
        let n = ElabNote::Hint("use norm_num".to_string());
        assert_eq!(n.prefix(), "hint");
        assert!(!n.is_warning_like());
    }

    #[test]
    fn test_elab_note_warning() {
        let n = ElabNote::Warning("unsupported construct".to_string());
        assert!(n.is_warning_like());
    }

    #[test]
    fn test_elab_note_sorry() {
        let n = ElabNote::SorryUsed {
            declaration: "myTheorem".to_string(),
        };
        assert!(n.is_warning_like());
        assert_eq!(n.message(), "myTheorem");
    }

    #[test]
    fn test_elab_note_display() {
        let n = ElabNote::Info("no issues".to_string());
        let s = format!("{}", n);
        assert!(s.contains("info"));
    }

    #[test]
    fn test_elab_note_set_add_warning() {
        let mut ns = ElabNoteSet::new();
        ns.add_warning("potential issue");
        assert!(ns.has_warnings());
        assert_eq!(ns.len(), 1);
    }

    #[test]
    fn test_elab_note_set_merge() {
        let mut a = ElabNoteSet::new();
        a.add_hint("hint 1");
        let mut b = ElabNoteSet::new();
        b.add_info("info 1");
        a.merge(b);
        assert_eq!(a.len(), 2);
    }

    #[test]
    fn test_elab_note_set_clear() {
        let mut ns = ElabNoteSet::new();
        ns.add_sorry("myThm");
        ns.clear();
        assert!(ns.is_empty());
    }

    #[test]
    fn test_is_known_tactic() {
        assert!(is_known_tactic("intro"));
        assert!(is_known_tactic("simp"));
        assert!(is_known_tactic("ring"));
        assert!(!is_known_tactic("unknownTac"));
    }

    #[test]
    fn test_tactic_category() {
        assert_eq!(tactic_category("intro"), "context");
        assert_eq!(tactic_category("simp"), "rewriting");
        assert_eq!(tactic_category("cases"), "structure");
        assert_eq!(tactic_category("push_neg"), "logic");
        assert_eq!(tactic_category("repeat"), "combinator");
    }

    #[test]
    fn test_reducibility_ordering() {
        assert!(Reducibility::Reducible < Reducibility::SemiReducible);
        assert!(Reducibility::SemiReducible < Reducibility::Irreducible);
    }

    #[test]
    fn test_reducibility_attr_names() {
        assert_eq!(Reducibility::Reducible.attr_name(), "reducible");
        assert_eq!(Reducibility::Irreducible.attr_name(), "irreducible");
    }

    #[test]
    fn test_reducibility_default() {
        assert_eq!(Reducibility::default(), Reducibility::SemiReducible);
    }

    #[test]
    fn test_tactic_names_intro() {
        assert_eq!(tactic_names::INTRO, "intro");
        assert_eq!(tactic_names::SORRY, "sorry");
    }

    #[test]
    fn test_elab_note_warnings_filter() {
        let mut ns = ElabNoteSet::new();
        ns.add_hint("h1");
        ns.add_warning("w1");
        ns.add_sorry("decl");
        let warns = ns.warnings();
        assert_eq!(warns.len(), 2);
    }
}
