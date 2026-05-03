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
pub mod meta_bridge;
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

pub mod core_types;
pub use core_types::*;

pub mod hole_inference;
pub mod synthesis;

/// Auto-bound implicit variable handling (Lean 4 auto-bound feature).
pub mod auto_bound;
/// Universe level management and polymorphism.
pub mod universe_management;
