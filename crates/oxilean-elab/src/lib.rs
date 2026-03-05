#![allow(unused_imports)]

//! # OxiLean Elaborator — From Surface Syntax to Typed Kernel Terms
//!
//! The elaborator bridges the gap between user-written OxiLean code (surface syntax)
//! and kernel-verified terms. It performs type inference, implicit argument resolution,
//! unification, and tactic execution.
//!
//! ## Quick Start
//!
//! ### Elaborating an Expression
//!
//! ```ignore
//! use oxilean_elab::{ElabContext, elaborate_expr};
//! use oxilean_kernel::Environment;
//!
//! let env = Environment::new();
//! let ctx = ElabContext::new(&env);
//! let surface_expr = /* parsed from source */;
//! let (kernel_expr, ty) = elaborate_expr(&ctx, surface_expr)?;
//! ```
//!
//! ### Elaborating a Declaration
//!
//! ```ignore
//! use oxilean_elab::elaborate_decl;
//!
//! let decl = /* parsed surface declaration */;
//! let elaborator = DeclElaborator::new(&ctx);
//! elaborator.elaborate(decl)?;
//! ```
//!
//! ## Architecture Overview
//!
//! The elaborator is a multi-stage system:
//!
//! ```text
//! Surface Syntax AST (from parser)
//!     │
//!     ▼
//! ┌───────────────────────────┐
//! │  Name Resolution          │  → Resolve names to definitions
//! │  (context.rs, elab_decl)  │  → Build local contexts
//! └───────────────────────────┘
//!     │
//!     ▼
//! ┌───────────────────────────┐
//! │  Type Inference           │  → Synthesize types for unknowns
//! │  (infer.rs)               │  → Generate metavariables (?m)
//! └───────────────────────────┘
//!     │
//!     ▼
//! ┌───────────────────────────┐
//! │  Implicit Arg Resolution  │  → Resolve {x} and [x]
//! │  (implicit.rs)            │  → Unify with inferred types
//! └───────────────────────────┘
//!     │
//!     ▼
//! ┌───────────────────────────┐
//! │  Unification & Solving    │  → Solve metavariable constraints
//! │  (unify.rs, solver.rs)    │  → Higher-order unification
//! └───────────────────────────┘
//!     │
//!     ▼
//! ┌───────────────────────────┐
//! │  Elaboration Passes       │  → Coercions, macros, tactics
//! │  (elaborate.rs)           │  → Build kernel terms
//! └───────────────────────────┘
//!     │
//!     ▼
//! Kernel Expressions (fully elaborated)
//!     │
//!     └─→ Kernel Type Checker (kernel TCB validates)
//!     └─→ Environment (if type-check passes)
//! ```
//!
//! ## Key Concepts & Terminology
//!
//! ### Metavariables
//!
//! Metavariables (`?m`, `?t`, `?P`) represent unknown terms during elaboration:
//! - `?m : τ` = metavariable with type τ
//! - Initially unsolved
//! - Solved via unification constraints
//! - Must be fully solved before kernel validation
//!
//! Example:
//! ```text
//! User writes: let x := _ in x + 1
//! Elaborator creates: let x := ?m in x + 1
//! Constraint: ?m : Nat (inferred from context)
//! Solution: ?m = 5 (if inferrable from usage)
//! Final: let x := 5 in x + 1
//! ```
//!
//! ### Type Inference
//!
//! Bidirectional type checking:
//! - **Synthesis** (↑): Infer type of expression `expr ↑ τ`
//! - **Checking** (↓): Check expression against type `expr ↓ τ`
//!
//! Example:
//! ```text
//! Synth: f x ↑ ?τ
//!   ├─ Synth: f ↑ (α → β)
//!   ├─ Check: x ↓ α
//!   └─ Result: β
//!
//! Check: fun x => x + 1 ↓ Nat → Nat
//!   ├─ Expect x : Nat
//!   └─ Check: x + 1 ↓ Nat
//! ```
//!
//! ### Implicit Arguments
//!
//! Arguments can be:
//! - **Explicit** `(x : T)`: Must be provided by user
//! - **Implicit** `{x : T}`: Inferred from context
//! - **Instance** `[C x]`: Filled by typeclass resolution
//!
//! The elaborator inserts placeholder metavariables for implicit args,
//! then solves them via unification.
//!
//! ### Unification
//!
//! Solves constraints of the form `?m =?= expr`:
//! - **First-order**: `?m = f x`
//! - **Higher-order**: `?f = λ x. ?body`
//! - **Occurs check**: Prevents infinite types like `?m = f ?m`
//!
//! ### Elaboration Context
//!
//! Manages state during elaboration:
//! - **Local variables**: `x : T` in scope
//! - **Metavariables**: Mapping from `MetaVarId` to assignment
//! - **Configuration**: Options for tactics, coercions, etc.
//! - **Environment**: Global definitions and axioms
//!
//! ## Module Organization
//!
//! ### Core Elaboration Modules
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `context` | Elaboration context and local variable management |
//! | `metavar` | Metavariable creation and tracking |
//! | `infer` | Type synthesis and checking |
//! | `unify` | Higher-order unification algorithm |
//!
//! ### Expression & Declaration Elaboration
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `elaborate` | Main expression elaboration pipeline |
//! | `elab_decl` | Declaration (def, theorem, etc.) elaboration |
//! | `binder` | Binder (`fun`, `forall`) elaboration |
//!
//! ### Argument Resolution
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `implicit` | Implicit argument resolution and synthesis |
//! | `instance` | Typeclass instance resolution |
//!
//! ### Advanced Features
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `unify` | Higher-order unification |
//! | `solver` | Constraint solver |
//! | `coercion` | Type coercion insertion |
//! | `pattern_match` | Pattern matching elaboration |
//! | `mutual` | Mutual recursion validation |
//! | `equation` | Equation compiler (pattern matching) |
//!
//! ### Tactics
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `tactic` | Tactic engine and core tactics |
//! | `tactic::intro` | `intro` tactic (introduce binders) |
//! | `tactic::apply` | `apply` tactic (apply theorem) |
//! | `tactic::exact` | `exact` tactic (provide term directly) |
//! | `tactic::rw` | `rw` tactic (rewrite by equality) |
//!
//! ### Utilities
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `error_msg` | Error message formatting |
//! | `notation` | Notation expansion |
//! | `macro_expand` | Macro expansion |
//! | `attribute` | Attribute processing (e.g., `@[simp]`) |
//! | `derive` | Deriving instances (e.g., `Eq`, `Show`) |
//! | `trace` | Debug tracing |
//!
//! ## Elaboration Pipeline Details
//!
//! ### Phase 1: Name Resolution
//!
//! Converts surface names to qualified names:
//! - Local variables: `x` → local index
//! - Global constants: `Nat.add` → fully qualified path
//! - Scoped opens: Apply namespace resolution
//! - Shadowing: Inner scopes shadow outer
//!
//! ### Phase 2: Type Inference
//!
//! Generates type constraints:
//! - For each subexpression, create metavariable for unknown type
//! - Collect unification constraints
//! - Maintain bidirectional flow (synthesis ↑, checking ↓)
//!
//! ### Phase 3: Implicit Argument Synthesis
//!
//! Fill in implicit arguments:
//! - For each implicit parameter, create metavariable\
//! - Pass to unification solver
//! - Fails if multiple solutions or no solution\
//!
//! ### Phase 4: Unification & Solving
//!
//! Solve all metavariable constraints:
//! - Higher-order unification algorithm
//! - Occurs check to prevent infinite terms
//! - Backtracking for multiple solutions
//!
//! ### Phase 5: Term Construction
//!
//! Build final kernel `Expr`:
//! - Substitute metavars with solutions
//! - Insert coercions where needed
//! - Expand macros
//! - Validate termination
//!
//! ### Phase 6: Tactic Execution (for proofs)
//!
//! If proof is `by <tactic>`:
//! - Execute tactic to transform goal
//! - May generate new subgoals
//! - Recursively elaborate subgoals\
//!
//! ### Phase 7: Kernel Validation
//!
//! Pass to kernel type checker:
//! - Independent verification
//! - All metavars must be solved
//! - Type check must succeed
//!
//! ## Usage Examples
//!
//! ### Example 1: Simple Expression
//!
//! ```ignore
//! use oxilean_elab::{ElabContext, elaborate_expr};
//!
//! let ctx = ElabContext::new(&env);
//! let expr = SurfaceExpr::app(
//!     SurfaceExpr::const_("Nat.add"),
//!     vec![SurfaceExpr::lit(5), SurfaceExpr::lit(3)],
//! );
//! let (kernel_expr, ty) = elaborate_expr(&ctx, expr)?;
//! // kernel_expr is now fully elaborated and type-checked
//! ```
//!
//! ### Example 2: Function with Implicit Arguments
//!
//! ```ignore
//! // User writes: myFunc x (where myFunc has implicit args)
//! // Elaborator infers and fills implicit args from context
//! let elaborate = elaborate_expr(&ctx, surface_expr)?;
//! ```
//!
//! ### Example 3: Tactic-Based Proof
//!
//! ```text
//! // User writes: theorem my_thm : P := by intro h; exact h
//! // Elaborator:
//! //   1. Creates goal: ⊢ P
//! //   2. Runs tactic: intro h
//! //   3. New goal: h : ⊢ P (with h in context)
//! //   4. Runs tactic: exact h
//! //   5. Constructs proof term
//! ```
//!
//! ## Error Handling
//!
//! Elaboration errors include:
//! - **Type mismatch**: Expected type τ, got σ
//! - **Unification failure**: Cannot unify terms
//! - **Unsolved metavariables**: `?m` remains after elaboration
//! - **Unknown identifier**: Name not in scope
//! - **Ambiguous instance**: Multiple typeclass instances match
//! - **Tactic error**: Tactic failed or invalid in context\
//!
//! All errors carry source location and helpful context messages.
//!
//! ## Tactic System
//!
//! Tactics are proof-building procedures:
//! - **Interactive**: In REPL or IDE
//! - **Automated**: Called during elaboration
//!
//! Core tactics:
//! - `intro`: Introduce binders from goal type
//! - `apply`: Apply theorem to goal
//! - `exact`: Provide exact proof term
//! - `rw`: Rewrite using equality
//! - `simp`: Simplify using lemmas
//! - `cases`: Case analysis on inductive type
//! - `induction`: Inductive reasoning\
//!
//! ## Integration with Other Crates
//!
//! ### With oxilean-kernel
//!
//! - Uses kernel `Expr`, `Environment`, `TypeChecker`
//! - Passes elaborated terms to kernel for validation
//! - Kernel is **never** bypassed
//!
//! ### With oxilean-parse
//!
//! - Consumes `SurfaceExpr`, `SurfaceDecl` from parser
//! - Converts to kernel types
//!
//! ### With oxilean-meta
//!
//! - Meta layer provides metavariable-aware operations
//! - Extends kernel WHNF, DefEq, TypeInfer with metavar support
//!
//! ## Performance Considerations
//!
//! - **Metavar allocation**: O(1) arena insertion
//! - **Unification**: O(expr_size) per constraint
//! - **Memoization**: Avoid re-elaborating same subexpressions
//! - **Lazy evaluation**: Defer expensive operations (normalization)\
//!
//! ## Further Reading
//!
//! - [ARCHITECTURE.md](../../ARCHITECTURE.md) — System architecture
//! - Module documentation for specific subcomponents

#![allow(missing_docs)]
#![warn(clippy::all)]
#![allow(clippy::result_large_err)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::len_without_is_empty)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::type_complexity)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::single_match)]
#![allow(clippy::needless_ifs)]
#![allow(clippy::useless_format)]
#![allow(clippy::new_without_default)]
#![allow(clippy::manual_strip)]
#![allow(clippy::needless_borrows_for_generic_args)]
#![allow(clippy::manual_saturating_arithmetic)]
#![allow(clippy::manual_is_variant_and)]
#![allow(clippy::iter_kv_map)]
#![allow(clippy::if_same_then_else)]
#![allow(clippy::collapsible_str_replace)]
#![allow(clippy::bool_comparison)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::len_zero)]
#![allow(clippy::unnecessary_map_or)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::implicit_saturating_sub)]
#![allow(clippy::to_string_in_format_args)]
#![allow(clippy::incompatible_msrv)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_bool)]
#![allow(clippy::needless_else)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::inherent_to_string)]
#![allow(clippy::manual_find)]
#![allow(clippy::double_ended_iterator_last)]
#![allow(clippy::for_kv_map)]
#![allow(clippy::needless_splitn)]
#![allow(clippy::trim_split_whitespace)]
#![allow(clippy::useless_vec)]
#![allow(clippy::cloned_ref_to_slice_refs)]
#![allow(non_snake_case)]

use oxilean_kernel::{Expr, Literal, Name};
pub mod attribute;
pub mod bench_support;
pub mod binder;
pub mod coercion;
pub mod context;
pub mod derive;
/// Full do-notation elaboration (bind, pure, for, try-catch, return).
pub mod do_notation;
pub mod elab_decl;
pub mod elab_expr;
pub mod elaborate;
pub mod equation;
/// Error message formatting and reporting infrastructure.
pub mod error_msg;
pub mod implicit;
pub mod infer;
/// Info tree for IDE integration (hover, go-to-def, type-on-hover).
pub mod info_tree;
pub mod instance;
pub mod macro_expand;
pub mod metaprog;
pub mod metavar;
pub mod mutual;
pub mod notation;
pub mod parallel;
pub mod pattern_match;
/// Pre-definition analysis: well-foundedness, structural recursion, termination.
pub mod predef;
pub mod quote;
pub mod solver;
pub mod structure;
pub mod tactic;
pub mod tactic_auto;
pub mod trace;
pub mod typeclass;
pub mod unify;

pub mod command_elab;
pub mod completion_provider;
/// Delaborator: convert kernel Expr to surface syntax.
pub mod delaborator;
pub mod derive_adv;
/// Differential testing framework: compare OxiLean elaboration against expected outputs.
pub mod differential_test;
pub mod elaboration_profiler;
pub mod hover_info;
pub mod lean4_compat;
pub mod module_import;

pub use attribute::{
    apply_attributes, process_attributes, AttrEntry, AttrError, AttrHandler, AttributeManager,
    ProcessedAttrs,
};
pub use binder::{BinderElabResult, BinderTypeResult};
pub use coercion::{Coercion, CoercionRegistry};
pub use context::{ElabContext, LocalEntry};
pub use derive::{DerivableClass, DeriveRegistry, Deriver};
pub use elab_decl::{elaborate_decl, DeclElabError, DeclElaborator, PendingDecl};
pub use elaborate::elaborate_expr;
pub use equation::{DecisionTree, Equation, EquationCompiler, Pattern};
pub use implicit::{resolve_implicits, resolve_instance, ImplicitArg};
pub use infer::{Constraint, InferResult, TypeInferencer};
pub use instance::{InstanceDecl, InstanceResolver};
pub use macro_expand::{MacroDef, MacroExpander};
pub use metavar::{MetaVar, MetaVarContext};
pub use mutual::{
    CallGraph, MutualBlock, MutualChecker, MutualElabError, StructuralRecursion, TerminationKind,
    WellFoundedRecursion,
};
pub use notation::{expand_do_notation, expand_list_literal, Notation, NotationRegistry};
pub use pattern_match::{
    check_exhaustive, check_redundant, elaborate_match, ElabPattern, ExhaustivenessResult,
    MatchEquation, PatternCompiler,
};
pub use quote::{quote, unquote, QuoteContext};
pub use solver::{is_unifiable, ConstraintSolver};
pub use structure::{
    FieldInfo, ProjectionDecl, StructElabError, StructureElaborator, StructureInfo,
};
pub use tactic::{
    eval_tactic_block, tactic_apply, tactic_by_contra, tactic_cases, tactic_contrapose,
    tactic_exact, tactic_induction, tactic_intro, tactic_push_neg, tactic_split, Goal, Tactic,
    TacticError, TacticRegistry, TacticResult, TacticState,
};
pub use typeclass::{Instance, Method, TypeClass, TypeClassRegistry};
pub use unify::unify;

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
        results.sort_by(|a, b| b.priority.cmp(&a.priority));
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
