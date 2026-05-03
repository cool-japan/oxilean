#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(unused_imports)]
#![allow(private_interfaces)]
//! # OxiLean Meta Layer — Metavar-Aware Operations & Tactics
//!
//! The meta layer extends the kernel with metavariable support, providing all the infrastructure
//! for elaboration, unification, and interactive tactic proving. It mirrors LEAN 4's `Lean.Meta`
//! namespace and sits logically between the elaborator and the trusted kernel.
//!
//! ## Quick Start
//!
//! ### Creating a Meta Context
//!
//! ```ignore
//! use oxilean_meta::{MetaContext, MetaConfig};
//! use oxilean_kernel::Environment;
//!
//! let env = Environment::new();
//! let config = MetaConfig::default();
//! let mut meta_ctx = MetaContext::new(&env, config);
//! ```
//!
//! ### Creating and Solving Metavariables
//!
//! ```ignore
//! use oxilean_meta::MVarId;
//!
//! let m1 = meta_ctx.mk_metavar(ty)?;
//! // ... unification happens ...
//! let solution = meta_ctx.get_assignment(m1)?;
//! ```
//!
//! ### Running a Tactic
//!
//! ```ignore
//! use oxilean_meta::TacticState;
//!
//! let mut state = TacticState::new(&meta_ctx, goal)?;
//! // ... execute tactics ...
//! let proof = state.close()?;
//! ```
//!
//! ## Architecture Overview
//!
//! The meta layer is organized into three functional areas:
//!
//! ```text
//! ┌──────────────────────────────────────────────────────┐
//! │              Meta Layer (oxilean-meta)               │
//! ├────────────────────────────────────────────────────────┤
//! │                                                         │
//! │  ┌─────────────────┐  ┌──────────────────┐             │
//! │  │  Core Meta Ops  │  │  Advanced Features│             │
//! │  ├─────────────────┤  ├──────────────────┤             │
//! │  │- MetaContext    │  │- Instance Synth  │             │
//! │  │- MetaWhnf       │  │- Discrimination  │             │
//! │  │- MetaDefEq      │  │  Trees           │             │
//! │  │- MetaInferType  │  │- App Builder     │             │
//! │  │- Level DefEq    │  │- Congr Theorems  │             │
//! │  └─────────────────┘  └──────────────────┘             │
//! │                                                         │
//! │  ┌──────────────────────────────────────┐              │
//! │  │     Tactic System                    │              │
//! │  ├──────────────────────────────────────┤              │
//! │  │- intro, apply, exact, rw, simp, ... │              │
//! │  │- Goal & TacticState management      │              │
//! │  │- Calc proofs, Omega solver, etc.    │              │
//! │  └──────────────────────────────────────┘              │
//! │                                                         │
//! └──────────────────────────────────────────────────────────┘
//!                          │
//!                          │ uses (doesn't modify)
//!                          ▼
//! ┌──────────────────────────────────────────────────────┐
//! │              OxiLean Kernel (TCB)                    │
//! │         (Expression, Type Checking, WHNF, ...)      │
//! └──────────────────────────────────────────────────────┘
//! ```
//!
//! ## Key Concepts & Terminology
//!
//! ### MetaContext vs MetaState
//!
//! - **MetaContext**: Global state during a proof session
//!   - All metavariables and their assignments
//!   - Configuration (recursion depth, timeouts, etc.)
//!   - Doesn't change during individual tactics
//!
//! - **MetaState**: Local goal-solving state
//!   - Current goal and subgoals
//!   - Local context (variables and hypotheses)
//!   - Changes as tactics are applied
//!
//! ### Goal
//!
//! A proof goal represented as:
//! ```text
//! x : Nat
//! h : P x
//! ⊢ Q x
//! ```
//!
//! Components:
//! - **Local context**: Variables `x : Nat`, hypotheses `h : P x`
//! - **Type** (target): What to prove `Q x`
//!
//! Goals are solved when type is inhabited (proof term provided).
//!
//! ### Tactic
//!
//! A **tactic** transforms goals:
//! ```text
//! Tactic: Goal → (Proof ∪ NewGoals ∪ Error)
//! ```
//!
//! Examples:
//! - `intro`: Transform `⊢ P → Q` to `p : P ⊢ Q`
//! - `exact t`: If `t : P`, close goal `⊢ P`
//! - `rw [eq]`: Transform using equality
//!
//! ### Metavariable Assignment
//!
//! Unification solves `?m =?= expr` by assigning:
//! ```text
//! ?m := expr
//! ```
//!
//! Subsequent references to `?m` automatically get the value.
//!
//! ### Discrimination Tree
//!
//! Fast indexing structure for term-based lookup (e.g., for simp lemmas):
//! - Index by **discriminator**: First argument, head function, etc.
//! - Retrieve candidates in O(log n) instead of O(n)
//!
//! ### Instance Synthesis
//!
//! Automated search for typeclass instances:
//! - Given `[C x]`, find value of type `C x`
//! - Uses tabled resolution (memoization)
//! - Prevents infinite loops
//!
//! ## Module Organization
//!
//! ### Batch 4.1: Core Meta Infrastructure
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `basic` | `MetaContext`, `MetaState`, `MVarId`, metavariable creation |
//! | `whnf` | Weak head normal form with metavar support |
//! | `def_eq` | Definitional equality and unification |
//! | `infer_type` | Type synthesis and checking (metavar-aware) |
//! | `level_def_eq` | Universe level unification |
//! | `discr_tree` | Discrimination tree indexing for fast lookup |
//!
//! ### Batch 4.2: Advanced Features
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `app_builder` | Helpers for building common proof terms (eq, symm, etc.) |
//! | `congr_theorems` | Automatic congruence lemma generation |
//! | `match_basic` | Basic pattern matching representation |
//! | `match_dectree` | Decision tree compilation for patterns |
//! | `match_exhaust` | Exhaustiveness checking |
//! | `synth_instance` | Typeclass instance synthesis |
//!
//! ### Batch 4.3: Core Tactics
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `tactic` | Tactic engine and state management |
//! | `tactic::intro` | `intro` (introduce binders) |
//! | `tactic::apply` | `apply` (apply theorems) |
//! | `tactic::rewrite` | `rw` (rewrite by equality) |
//! | `tactic::simp` | `simp` (simplification with lemmas) |
//! | `tactic::omega` | `omega` (linear arithmetic) |
//! | `tactic::calc` | `calc` (calculational proofs) |
//!
//! ### Batch 4.4: Utilities
//!
//! | Module | Purpose |
//! |--------|---------|
//! | `util` | Utilities: `collect_mvars`, `collect_fvars`, `FunInfo` |
//! | `proof_replay` | Proof term caching and memoization |
//! | `simp_engine` | Simplification engine and simp lemma management |
//!
//! ## Meta Operations Pipeline
//!
//! ### Type Inference (Meta Version)
//!
//! ```text
//! MetaInferType::infer_synth(expr)
//!   │
//!   ├─ Recursively infer subexpressions
//!   ├─ Create metavars for unknown types
//!   ├─ Collect unification constraints
//!   └─ Return: (expr_with_metavars, type)
//! ```
//!
//! ### Unification Pipeline
//!
//! ```text
//! MetaDefEq::unify(goal_expr, expr)
//!   │
//!   ├─ Reduce both to WHNF (with metavar support)
//!   ├─ Check if structurally equal
//!   ├─ If not, try:
//!   │  ├─ Variable unification: ?m := expr
//!   │  ├─ Application: f a =? g b
//!   │  │  └─ Unify: f =? g, a =? b
//!   │  ├─ Lambda: λ x. t =? λ y. u
//!   │  │  └─ Unify: t[x/?a] =? u[y/?a] (fresh ?a)
//!   │  └─ Higher-order cases
//!   ├─ Occurs check: Prevent ?m := f ?m
//!   └─ Return: Success or failure
//! ```
//!
//! ### Tactic Execution
//!
//! ```text
//! TacticState::execute(tactic)
//!   │
//!   ├─ For each goal:
//!   │  ├─ Apply tactic to goal
//!   │  ├─ If success: Add new subgoals to queue
//!   │  ├─ If exact: Mark goal closed
//!   │  └─ If failure: Backtrack or error
//!   │
//!   ├─ Recursively solve subgoals
//!   │
//!   └─ When all goals closed: Construct proof term
//! ```
//!
//! ## Usage Examples
//!
//! ### Example 1: Type Inference with Metavars
//!
//! ```ignore
//! use oxilean_meta::MetaInferType;
//!
//! let mut infer = MetaInferType::new(&meta_ctx);
//! let (expr_with_metas, ty) = infer.infer_synth(surface_expr)?;
//! // expr_with_metas may contain unsolved metavars
//! ```
//!
//! ### Example 2: Unification
//!
//! ```ignore
//! use oxilean_meta::MetaDefEq;
//!
//! let mut def_eq = MetaDefEq::new(&meta_ctx);
//! def_eq.unify(goal, candidate)?;
//! // If successful, metavars are assigned
//! ```
//!
//! ### Example 3: Tactic Execution
//!
//! ```ignore
//! use oxilean_meta::TacticState;
//!
//! let mut state = TacticState::new(&meta_ctx, initial_goal)?;
//! state.intro()?;  // Run intro tactic
//! state.apply(theorem)?;  // Run apply tactic
//! let proof = state.close()?;  // Get proof term
//! ```
//!
//! ### Example 4: Instance Synthesis
//!
//! ```ignore
//! use oxilean_meta::InstanceSynthesizer;
//!
//! let synth = InstanceSynthesizer::new(&meta_ctx);
//! let instance = synth.synth_instance(class_type)?;
//! // Returns proof that satisfies typeclass constraint
//! ```
//!
//! ## Tactic Language
//!
//! Common tactics:
//! - **Proof construction**: `intro`, `apply`, `exact`, `refine`
//! - **Rewriting**: `rw [h]`, `simp`, `simp only`
//! - **Analysis**: `cases x`, `induction x on y`, `split`
//! - **Automation**: `omega` (linear arith), `decide` (decidable)
//! - **Control**: `;` (then), `<|>` (or), `repeat`, `try`
//!
//! ## Integration with Other Crates
//!
//! ### With oxilean-kernel
//!
//! - Uses kernel `Expr`, `Level`, `Environment`
//! - Reads kernel operations: WHNF, type checking
//! - **Does not** modify kernel (immutable reference)
//! - Proof terms eventually passed to kernel for validation
//!
//! ### With oxilean-elab
//!
//! - Elaborator uses MetaContext for managing elaboration state
//! - MetaDefEq/MetaWhnf for type checking during elaboration
//! - Instance synthesis for implicit arguments
//! - Tactic execution during proof elaboration
//!
//! ## Performance Optimizations
//!
//! - **Memoization**: Cache WHNF results for repeated reductions
//! - **Discrimination trees**: O(log n) lookup for simp lemmas
//! - **Proof replay**: Avoid re-executing identical tactics
//! - **Lazy normalization**: Only normalize when needed
//! - **Constraint postponement**: Defer hard constraints
//!
//! ## Further Reading
//!
//! - [ARCHITECTURE.md](../../ARCHITECTURE.md) — System architecture
//! - Module documentation for specific subcomponents

#![forbid(unsafe_code)]
#![warn(clippy::all)]
#![allow(clippy::result_large_err)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::single_match)]
#![allow(clippy::module_inception)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::needless_ifs)]

// --- Batch 4.1: Meta Core ---
pub mod basic;
pub mod def_eq;
pub mod discr_tree;
pub mod infer_type;
pub mod level_def_eq;
pub mod whnf;

// --- Batch 4.2: Meta Features ---
pub mod app_builder;
pub mod congr_theorems;
pub mod match_basic;
pub mod match_dectree;
pub mod match_exhaust;
pub mod synth_instance;

// --- Re-exports: Batch 4.1 ---
pub use basic::{
    MVarId, MetaConfig, MetaContext, MetaState, MetavarDecl, MetavarKind, PostponedConstraint,
};
pub use def_eq::{MetaDefEq, UnificationResult};
pub use discr_tree::{DiscrTree, DiscrTreeKey};
pub use infer_type::MetaInferType;
pub use level_def_eq::LevelDefEq;
pub use whnf::MetaWhnf;

// --- Re-exports: Batch 4.2 ---
pub use app_builder::{mk_eq, mk_eq_refl, mk_eq_symm, mk_eq_trans};
pub use congr_theorems::{CongrArgKind, MetaCongrTheorem};
pub use match_basic::{MetaMatchArm, MetaMatchExpr, MetaPattern};
pub use match_dectree::{DecisionBranch, DecisionTree, MatchEquation};
pub use match_exhaust::{ConstructorSpec, ExhaustivenessResult};
pub use synth_instance::{InstanceEntry, InstanceSynthesizer, SynthResult};

// --- Batch 4.3: Core Tactics ---
pub mod tactic;

// --- Batch 4.4: Meta Utilities ---
pub mod util;

// --- Phase 9.1: Performance modules ---
pub mod proof_replay;
pub mod prop_test;
pub mod simp_engine;

pub mod ast_cache;
pub mod convenience;
pub mod meta_debug;

// --- Extracted submodules (splitrs) ---
pub mod functions;
pub mod types;

// --- New modules ---
pub mod coercion_system;
pub mod elaboration_hooks;
pub mod tactic_reflection;
pub mod tc_synthesis;

// --- Re-exports: Batch 4.3 ---
pub use tactic::rewrite::RewriteDirection;
pub use tactic::{GoalView, TacticError, TacticResult, TacticState};

// --- Re-exports: Batch 4.4 ---
pub use tactic::calc::{CalcProof, CalcStep, ConvSide};
pub use tactic::omega::{LinearConstraint, LinearExpr, OmegaResult};
pub use tactic::simp::discharge::DischargeStrategy;
pub use tactic::simp::types::{
    default_simp_lemmas, SimpConfig, SimpLemma, SimpResult, SimpTheorems,
};
pub use util::{collect_fvars, collect_mvars, FunInfo};

// --- Re-exports: extracted submodules ---
pub use functions::{
    collect_meta_stats, default_tactic_registry, is_automation_tactic, is_core_tactic,
    meta_version_str, mk_test_ctx, sort_candidates, standard_tactic_groups, tactic_description,
    tactic_group_for, META_VERSION,
};
pub use types::{
    LibAnalysisPass, LibConfig, LibConfigValue, LibDiagnostics, LibDiff, LibExtConfig1300,
    LibExtConfigVal1300, LibExtDiag1300, LibExtDiff1300, LibExtPass1300, LibExtPipeline1300,
    LibExtResult1300, LibPipeline, LibResult, MetaCache, MetaFeatures, MetaLibBuilder,
    MetaLibCounterMap, MetaLibExtMap, MetaLibExtUtil, MetaLibState, MetaLibStateMachine,
    MetaLibWindow, MetaLibWorkQueue, MetaStats, PerfStats, ProofStateReport, ScoredCandidate,
    TacticGroup, TacticRegistry,
};
