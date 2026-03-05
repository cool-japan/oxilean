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

// ---------------------------------------------------------------------------
// Section 2: Meta layer facade, convenience API, and registry utilities
// ---------------------------------------------------------------------------

/// Version of the meta layer.
pub const META_VERSION: &str = "0.9.0-alpha";

/// Return the meta layer version string.
pub fn meta_version_str() -> &'static str {
    META_VERSION
}

/// Helper to create a minimal `MetaContext` for testing.
pub fn mk_test_ctx() -> MetaContext {
    MetaContext::new(oxilean_kernel::Environment::new())
}

/// A named tactic registry.
#[derive(Debug, Default)]
pub struct TacticRegistry {
    pub entries: std::collections::HashMap<String, usize>,
    pub names: Vec<String>,
}

impl TacticRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a tactic name.
    pub fn register(&mut self, name: impl Into<String>) -> usize {
        let name = name.into();
        if let Some(&idx) = self.entries.get(&name) {
            return idx;
        }
        let idx = self.names.len();
        self.entries.insert(name.clone(), idx);
        self.names.push(name);
        idx
    }

    /// Look up a tactic index by name.
    pub fn lookup(&self, name: &str) -> Option<usize> {
        self.entries.get(name).copied()
    }

    /// Get the name for an index.
    pub fn name_of(&self, idx: usize) -> Option<&str> {
        self.names.get(idx).map(String::as_str)
    }

    /// Number of registered tactics.
    pub fn len(&self) -> usize {
        self.names.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Get all registered names.
    pub fn all_names(&self) -> &[String] {
        &self.names
    }
}

/// Build a default tactic registry.
pub fn default_tactic_registry() -> TacticRegistry {
    let mut reg = TacticRegistry::new();
    for tac in [
        "intro",
        "intros",
        "exact",
        "assumption",
        "refl",
        "trivial",
        "sorry",
        "apply",
        "cases",
        "induction",
        "rw",
        "rewrite",
        "simp",
        "simp_only",
        "have",
        "show",
        "obtain",
        "use",
        "exists",
        "constructor",
        "left",
        "right",
        "split",
        "exfalso",
        "clear",
        "revert",
        "subst",
        "rename",
        "ring",
        "linarith",
        "omega",
        "norm_num",
        "push_neg",
        "by_contra",
        "by_contradiction",
        "contrapose",
        "field_simp",
        "simp_all",
        "rfl",
        "all_goals",
        "first",
        "repeat",
        "try",
    ] {
        reg.register(tac);
    }
    reg
}

/// Check if a tactic name is a core tactic.
pub fn is_core_tactic(name: &str) -> bool {
    matches!(
        name,
        "intro"
            | "intros"
            | "exact"
            | "assumption"
            | "refl"
            | "trivial"
            | "sorry"
            | "apply"
            | "cases"
            | "induction"
            | "rw"
            | "rewrite"
            | "simp"
            | "have"
            | "show"
            | "obtain"
            | "use"
            | "constructor"
            | "left"
            | "right"
            | "split"
            | "exfalso"
            | "clear"
            | "revert"
            | "subst"
            | "rename"
            | "ring"
            | "linarith"
            | "omega"
            | "push_neg"
            | "by_contra"
    )
}

/// Check if a tactic is an automation tactic.
pub fn is_automation_tactic(name: &str) -> bool {
    matches!(
        name,
        "simp"
            | "simp_all"
            | "omega"
            | "linarith"
            | "ring"
            | "norm_num"
            | "decide"
            | "trivial"
            | "tauto"
            | "aesop"
            | "field_simp"
    )
}

/// Describe a tactic's purpose.
pub fn tactic_description(name: &str) -> Option<&'static str> {
    match name {
        "intro" | "intros" => Some("Introduce binders from the goal"),
        "exact" => Some("Close the goal with a proof term"),
        "apply" => Some("Apply a lemma to the goal"),
        "assumption" => Some("Close goal using a hypothesis"),
        "refl" | "rfl" => Some("Close a reflexivity goal"),
        "cases" => Some("Case-split on an inductive type"),
        "induction" => Some("Induct on an inductive type"),
        "rw" | "rewrite" => Some("Rewrite the goal using an equation"),
        "simp" => Some("Simplify using simp lemmas"),
        "have" => Some("Introduce a local lemma"),
        "split" => Some("Split a conjunction or iff goal"),
        "sorry" => Some("Close goal with sorry (unsound)"),
        _ => None,
    }
}

/// A simple cache for memoizing meta computations.
#[derive(Debug)]
pub struct MetaCache<K, V> {
    pub entries: std::collections::HashMap<K, V>,
    pub capacity: usize,
    pub hits: u64,
    pub misses: u64,
}

impl<K: std::hash::Hash + Eq + Clone, V> MetaCache<K, V> {
    /// Create a cache with a given capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            entries: std::collections::HashMap::with_capacity(capacity),
            capacity,
            hits: 0,
            misses: 0,
        }
    }

    /// Insert a value.
    pub fn insert(&mut self, key: K, value: V) {
        if self.entries.len() >= self.capacity {
            let len = self.entries.len();
            if len > 0 {
                let to_remove = len / 2;
                let keys: Vec<K> = self.entries.keys().take(to_remove).cloned().collect();
                for k in keys {
                    self.entries.remove(&k);
                }
            }
        }
        self.entries.insert(key, value);
    }

    /// Look up a value.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.entries.contains_key(key) {
            self.hits += 1;
            self.entries.get(key)
        } else {
            self.misses += 1;
            None
        }
    }

    /// Cache hit rate.
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    /// Number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Clear the cache.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.hits = 0;
        self.misses = 0;
    }
}

/// Summary statistics about a `MetaContext`.
#[derive(Debug, Clone, Default)]
pub struct MetaStats {
    /// Number of expression metavariables.
    pub num_expr_mvars: usize,
    /// Number of assigned expression metavariables.
    pub num_assigned_expr: usize,
    /// Number of level metavariables.
    pub num_level_mvars: usize,
    /// Number of assigned level metavariables.
    pub num_assigned_levels: usize,
    /// Number of postponed constraints.
    pub num_postponed: usize,
}

/// Collect statistics from a `MetaContext`.
pub fn collect_meta_stats(ctx: &MetaContext) -> MetaStats {
    MetaStats {
        num_expr_mvars: ctx.num_mvars(),
        num_assigned_expr: 0, // private field; use num_mvars() as approximation
        num_level_mvars: 0,   // private field; not exposed via public API
        num_assigned_levels: 0, // private field; not exposed via public API
        num_postponed: ctx.num_postponed(),
    }
}

/// Report of proof state completeness.
#[derive(Debug, Clone)]
pub struct ProofStateReport {
    /// Number of open goals.
    pub open_goals: usize,
    /// Whether the proof is complete.
    pub is_complete: bool,
}

impl ProofStateReport {
    /// Create from a tactic state.
    pub fn from_state(state: &TacticState) -> Self {
        ProofStateReport {
            open_goals: state.num_goals(),
            is_complete: state.is_done(),
        }
    }
}

/// A scored candidate.
#[derive(Debug, Clone)]
pub struct ScoredCandidate<T> {
    /// The candidate.
    pub candidate: T,
    /// Score.
    pub score: i64,
}

impl<T> ScoredCandidate<T> {
    /// Create a new scored candidate.
    pub fn new(candidate: T, score: i64) -> Self {
        Self { candidate, score }
    }
}

/// Sort candidates by descending score.
pub fn sort_candidates<T: Clone>(candidates: &mut [ScoredCandidate<T>]) {
    candidates.sort_by(|a, b| b.score.cmp(&a.score));
}

#[cfg(test)]
mod meta_lib_tests {
    use super::*;

    #[test]
    fn test_meta_version_str() {
        assert!(!meta_version_str().is_empty());
    }

    #[test]
    fn test_tactic_registry_register() {
        let mut reg = TacticRegistry::new();
        let idx = reg.register("intro");
        assert_eq!(reg.lookup("intro"), Some(idx));
        assert_eq!(reg.lookup("nonexistent"), None);
    }

    #[test]
    fn test_tactic_registry_idempotent() {
        let mut reg = TacticRegistry::new();
        assert_eq!(reg.register("intro"), reg.register("intro"));
    }

    #[test]
    fn test_tactic_registry_name_of() {
        let mut reg = TacticRegistry::new();
        let idx = reg.register("apply");
        assert_eq!(reg.name_of(idx), Some("apply"));
        assert_eq!(reg.name_of(999), None);
    }

    #[test]
    fn test_default_tactic_registry() {
        let reg = default_tactic_registry();
        assert!(reg.len() > 10);
        assert!(reg.lookup("intro").is_some());
    }

    #[test]
    fn test_is_core_tactic() {
        assert!(is_core_tactic("intro"));
        assert!(!is_core_tactic("nonexistent"));
    }

    #[test]
    fn test_is_automation_tactic() {
        assert!(is_automation_tactic("simp"));
        assert!(!is_automation_tactic("intro"));
    }

    #[test]
    fn test_tactic_description() {
        assert!(tactic_description("intro").is_some());
        assert_eq!(tactic_description("nonexistent_xyz"), None);
    }

    #[test]
    fn test_meta_cache_basic() {
        let mut cache: MetaCache<String, i32> = MetaCache::with_capacity(10);
        cache.insert("key".into(), 42);
        assert_eq!(cache.get(&"key".to_string()), Some(&42));
        assert_eq!(cache.get(&"missing".to_string()), None);
    }

    #[test]
    fn test_meta_cache_hit_rate() {
        let mut cache: MetaCache<String, i32> = MetaCache::with_capacity(10);
        cache.insert("key".into(), 1);
        let _ = cache.get(&"key".to_string());
        let _ = cache.get(&"miss".to_string());
        assert!((cache.hit_rate() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn test_meta_cache_clear() {
        let mut cache: MetaCache<String, i32> = MetaCache::with_capacity(10);
        cache.insert("a".into(), 1);
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_scored_candidate() {
        let c = ScoredCandidate::new("lemma", 100i64);
        assert_eq!(c.candidate, "lemma");
    }

    #[test]
    fn test_sort_candidates() {
        let mut v = vec![
            ScoredCandidate::new("a", 1i64),
            ScoredCandidate::new("b", 3i64),
            ScoredCandidate::new("c", 2i64),
        ];
        sort_candidates(&mut v);
        assert_eq!(v[0].candidate, "b");
    }

    #[test]
    fn test_collect_meta_stats() {
        let ctx = mk_test_ctx();
        let stats = collect_meta_stats(&ctx);
        assert_eq!(stats.num_expr_mvars, 0);
    }

    #[test]
    fn test_proof_state_report() {
        let mut ctx = mk_test_ctx();
        let goal_ty = oxilean_kernel::Expr::Const(oxilean_kernel::Name::str("P"), vec![]);
        let (mvar_id, _) = ctx.mk_fresh_expr_mvar(goal_ty, crate::basic::MetavarKind::Natural);
        let state = TacticState::single(mvar_id);
        let report = ProofStateReport::from_state(&state);
        assert_eq!(report.open_goals, 1);
        assert!(!report.is_complete);
    }

    #[test]
    fn test_tactic_registry_all_names() {
        let mut reg = TacticRegistry::new();
        reg.register("a");
        reg.register("b");
        assert_eq!(reg.all_names().len(), 2);
    }

    #[test]
    fn test_mk_test_ctx() {
        let ctx = mk_test_ctx();
        assert_eq!(ctx.num_mvars(), 0);
    }
}

// ============================================================
// Additional meta-layer utilities
// ============================================================

/// Simple accumulator for meta-layer performance statistics.
#[allow(dead_code)]
pub struct PerfStats {
    /// Total number of elaboration attempts.
    pub elab_attempts: u64,
    /// Number of successful elaborations.
    pub elab_successes: u64,
    /// Total unification attempts.
    pub unif_attempts: u64,
    /// Number of successful unifications.
    pub unif_successes: u64,
    /// Total elapsed time in microseconds.
    pub elapsed_us: u64,
}

#[allow(dead_code)]
impl PerfStats {
    /// Create an empty stats record.
    pub fn new() -> Self {
        PerfStats {
            elab_attempts: 0,
            elab_successes: 0,
            unif_attempts: 0,
            unif_successes: 0,
            elapsed_us: 0,
        }
    }

    /// Return the elaboration success rate as a fraction in [0, 1].
    pub fn elab_success_rate(&self) -> f64 {
        if self.elab_attempts == 0 {
            return 0.0;
        }
        self.elab_successes as f64 / self.elab_attempts as f64
    }

    /// Return the unification success rate as a fraction in [0, 1].
    pub fn unif_success_rate(&self) -> f64 {
        if self.unif_attempts == 0 {
            return 0.0;
        }
        self.unif_successes as f64 / self.unif_attempts as f64
    }

    /// Merge another `PerfStats` into this one.
    pub fn merge(&mut self, other: &PerfStats) {
        self.elab_attempts += other.elab_attempts;
        self.elab_successes += other.elab_successes;
        self.unif_attempts += other.unif_attempts;
        self.unif_successes += other.unif_successes;
        self.elapsed_us += other.elapsed_us;
    }
}

impl Default for PerfStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod perf_stats_tests {
    use super::*;

    #[test]
    fn test_perf_stats_empty() {
        let s = PerfStats::new();
        assert_eq!(s.elab_attempts, 0);
        assert!((s.elab_success_rate() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_perf_stats_success_rate() {
        let mut s = PerfStats::new();
        s.elab_attempts = 10;
        s.elab_successes = 7;
        assert!((s.elab_success_rate() - 0.7).abs() < 1e-9);
    }

    #[test]
    fn test_perf_stats_merge() {
        let mut a = PerfStats::new();
        a.elab_attempts = 5;
        let b = PerfStats::new();
        a.merge(&b);
        assert_eq!(a.elab_attempts, 5);
    }

    #[test]
    fn test_perf_stats_unif_success_rate_no_attempts() {
        let s = PerfStats::new();
        assert!((s.unif_success_rate() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_perf_stats_merge_sums() {
        let mut a = PerfStats::new();
        a.elab_attempts = 5;
        a.elab_successes = 3;
        let mut b = PerfStats::new();
        b.elab_attempts = 3;
        b.elab_successes = 2;
        a.merge(&b);
        assert_eq!(a.elab_attempts, 8);
        assert_eq!(a.elab_successes, 5);
    }

    #[test]
    fn test_perf_stats_elapsed() {
        let mut s = PerfStats::new();
        s.elapsed_us = 1000;
        assert_eq!(s.elapsed_us, 1000);
    }

    #[test]
    fn test_perf_stats_default() {
        let s = PerfStats::default();
        assert_eq!(s.elab_attempts, 0);
    }

    #[test]
    fn test_perf_stats_unif() {
        let mut s = PerfStats::new();
        s.unif_attempts = 4;
        s.unif_successes = 3;
        assert!((s.unif_success_rate() - 0.75).abs() < 1e-9);
    }
}

// ============================================================
// MetaLayer configuration and feature flags
// ============================================================

/// Feature flags for the meta layer.
#[derive(Clone, Debug)]
pub struct MetaFeatures {
    /// Enable discrimination tree indexing for simp lemmas.
    pub discr_tree: bool,
    /// Enable memoization of WHNF results.
    pub whnf_cache: bool,
    /// Enable proof term recording.
    pub proof_recording: bool,
    /// Enable instance synthesis.
    pub instance_synth: bool,
    /// Enable congr-lemma automation.
    pub congr_lemmas: bool,
}

impl Default for MetaFeatures {
    fn default() -> Self {
        Self {
            discr_tree: true,
            whnf_cache: true,
            proof_recording: false,
            instance_synth: true,
            congr_lemmas: true,
        }
    }
}

impl MetaFeatures {
    /// All features enabled.
    pub fn all_enabled() -> Self {
        Self {
            discr_tree: true,
            whnf_cache: true,
            proof_recording: true,
            instance_synth: true,
            congr_lemmas: true,
        }
    }

    /// Minimal features (fast, less complete).
    pub fn minimal() -> Self {
        Self {
            discr_tree: false,
            whnf_cache: false,
            proof_recording: false,
            instance_synth: false,
            congr_lemmas: false,
        }
    }

    /// Whether at least one caching feature is enabled.
    pub fn any_caching(&self) -> bool {
        self.whnf_cache || self.proof_recording
    }
}

/// A named group of related tactics.
#[derive(Clone, Debug)]
pub struct TacticGroup {
    /// Group name.
    pub name: String,
    /// Tactic names in this group.
    pub members: Vec<String>,
    /// Short description of the group.
    pub description: String,
}

impl TacticGroup {
    /// Create a tactic group.
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            members: Vec::new(),
            description: description.to_string(),
        }
    }

    /// Add a member tactic.
    #[allow(clippy::should_implement_trait)]
    pub fn add(mut self, tactic: &str) -> Self {
        self.members.push(tactic.to_string());
        self
    }

    /// Whether a tactic is in this group.
    pub fn contains(&self, tactic: &str) -> bool {
        self.members.iter().any(|m| m == tactic)
    }
}

/// Return the standard tactic groups.
pub fn standard_tactic_groups() -> Vec<TacticGroup> {
    vec![
        TacticGroup::new("introduction", "Tactics that introduce hypotheses")
            .add("intro")
            .add("intros"),
        TacticGroup::new("closing", "Tactics that close goals")
            .add("exact")
            .add("assumption")
            .add("refl")
            .add("rfl")
            .add("trivial")
            .add("sorry"),
        TacticGroup::new("rewriting", "Tactics that rewrite the goal")
            .add("rw")
            .add("rewrite")
            .add("simp")
            .add("simp_all"),
        TacticGroup::new("structural", "Tactics that split/analyze goals")
            .add("cases")
            .add("induction")
            .add("split")
            .add("constructor")
            .add("left")
            .add("right"),
        TacticGroup::new("automation", "Automated solving tactics")
            .add("omega")
            .add("linarith")
            .add("ring")
            .add("norm_num")
            .add("decide"),
    ]
}

/// Find the group that a tactic belongs to.
pub fn tactic_group_for(tactic: &str) -> Option<&'static str> {
    match tactic {
        "intro" | "intros" => Some("introduction"),
        "exact" | "assumption" | "refl" | "rfl" | "trivial" | "sorry" => Some("closing"),
        "rw" | "rewrite" | "simp" | "simp_all" => Some("rewriting"),
        "cases" | "induction" | "split" | "constructor" | "left" | "right" => Some("structural"),
        "omega" | "linarith" | "ring" | "norm_num" | "decide" => Some("automation"),
        _ => None,
    }
}

#[cfg(test)]
mod meta_features_tests {
    use super::*;

    #[test]
    fn test_meta_features_default() {
        let f = MetaFeatures::default();
        assert!(f.discr_tree);
        assert!(f.instance_synth);
        assert!(!f.proof_recording);
    }

    #[test]
    fn test_meta_features_all_enabled() {
        let f = MetaFeatures::all_enabled();
        assert!(f.proof_recording);
        assert!(f.whnf_cache);
    }

    #[test]
    fn test_meta_features_minimal() {
        let f = MetaFeatures::minimal();
        assert!(!f.discr_tree);
        assert!(!f.instance_synth);
    }

    #[test]
    fn test_meta_features_any_caching_default() {
        let f = MetaFeatures::default();
        assert!(f.any_caching());
    }

    #[test]
    fn test_meta_features_any_caching_minimal() {
        let f = MetaFeatures::minimal();
        assert!(!f.any_caching());
    }

    #[test]
    fn test_tactic_group_contains() {
        let g = TacticGroup::new("test", "desc").add("intro").add("intros");
        assert!(g.contains("intro"));
        assert!(!g.contains("exact"));
    }

    #[test]
    fn test_standard_tactic_groups_nonempty() {
        let groups = standard_tactic_groups();
        assert!(!groups.is_empty());
    }

    #[test]
    fn test_tactic_group_for_intro() {
        assert_eq!(tactic_group_for("intro"), Some("introduction"));
    }

    #[test]
    fn test_tactic_group_for_exact() {
        assert_eq!(tactic_group_for("exact"), Some("closing"));
    }

    #[test]
    fn test_tactic_group_for_unknown() {
        assert_eq!(tactic_group_for("foobar_nonexistent"), None);
    }

    #[test]
    fn test_tactic_group_for_omega() {
        assert_eq!(tactic_group_for("omega"), Some("automation"));
    }
}

// ============================================================
// Extended: MetaLib Utilities (Part 2)
// ============================================================

/// An extended utility type for MetaLib.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct MetaLibExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}

#[allow(dead_code)]
impl MetaLibExtUtil {
    pub fn new(key: &str) -> Self {
        MetaLibExtUtil {
            key: key.to_string(),
            data: Vec::new(),
            active: true,
            flags: 0,
        }
    }

    pub fn push(&mut self, v: i64) {
        self.data.push(v);
    }
    pub fn pop(&mut self) -> Option<i64> {
        self.data.pop()
    }
    pub fn sum(&self) -> i64 {
        self.data.iter().sum()
    }
    pub fn min_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::min)
    }
    pub fn max_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::max)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn set_flag(&mut self, bit: u32) {
        self.flags |= 1 << bit;
    }
    pub fn has_flag(&self, bit: u32) -> bool {
        self.flags & (1 << bit) != 0
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    pub fn activate(&mut self) {
        self.active = true;
    }
}

/// An extended map for MetaLib keys to values.
#[allow(dead_code)]
pub struct MetaLibExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}

#[allow(dead_code)]
impl<V: Clone + Default> MetaLibExtMap<V> {
    pub fn new() -> Self {
        MetaLibExtMap {
            data: std::collections::HashMap::new(),
            default_key: None,
        }
    }

    pub fn insert(&mut self, key: &str, value: V) {
        self.data.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&V> {
        self.data.get(key)
    }

    pub fn get_or_default(&self, key: &str) -> V {
        self.data.get(key).cloned().unwrap_or_default()
    }

    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.data.remove(key)
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn set_default(&mut self, key: &str) {
        self.default_key = Some(key.to_string());
    }

    pub fn keys_sorted(&self) -> Vec<&String> {
        let mut keys: Vec<&String> = self.data.keys().collect();
        keys.sort();
        keys
    }
}

impl<V: Clone + Default> Default for MetaLibExtMap<V> {
    fn default() -> Self {
        Self::new()
    }
}

/// A sliding window accumulator for MetaLib.
#[allow(dead_code)]
pub struct MetaLibWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}

#[allow(dead_code)]
impl MetaLibWindow {
    pub fn new(capacity: usize) -> Self {
        MetaLibWindow {
            buffer: std::collections::VecDeque::new(),
            capacity,
            running_sum: 0.0,
        }
    }

    pub fn push(&mut self, v: f64) {
        if self.buffer.len() >= self.capacity {
            if let Some(old) = self.buffer.pop_front() {
                self.running_sum -= old;
            }
        }
        self.buffer.push_back(v);
        self.running_sum += v;
    }

    pub fn mean(&self) -> f64 {
        if self.buffer.is_empty() {
            0.0
        } else {
            self.running_sum / self.buffer.len() as f64
        }
    }

    pub fn variance(&self) -> f64 {
        if self.buffer.len() < 2 {
            return 0.0;
        }
        let m = self.mean();
        self.buffer.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / self.buffer.len() as f64
    }

    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

/// A builder pattern for MetaLib.
#[allow(dead_code)]
pub struct MetaLibBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}

#[allow(dead_code)]
impl MetaLibBuilder {
    pub fn new(name: &str) -> Self {
        MetaLibBuilder {
            name: name.to_string(),
            items: Vec::new(),
            config: std::collections::HashMap::new(),
        }
    }

    pub fn add_item(mut self, item: &str) -> Self {
        self.items.push(item.to_string());
        self
    }

    pub fn set_config(mut self, key: &str, value: &str) -> Self {
        self.config.insert(key.to_string(), value.to_string());
        self
    }

    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    pub fn has_config(&self, key: &str) -> bool {
        self.config.contains_key(key)
    }
    pub fn get_config(&self, key: &str) -> Option<&str> {
        self.config.get(key).map(|s| s.as_str())
    }

    pub fn build_summary(&self) -> String {
        format!(
            "{}: {} items, {} config keys",
            self.name,
            self.items.len(),
            self.config.len()
        )
    }
}

/// A state machine for MetaLib.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum MetaLibState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}

#[allow(dead_code)]
impl MetaLibState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, MetaLibState::Complete | MetaLibState::Failed(_))
    }

    pub fn can_run(&self) -> bool {
        matches!(self, MetaLibState::Initial | MetaLibState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, MetaLibState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            MetaLibState::Failed(s) => Some(s),
            _ => None,
        }
    }
}

/// A state machine controller for MetaLib.
#[allow(dead_code)]
pub struct MetaLibStateMachine {
    pub state: MetaLibState,
    pub transitions: usize,
    pub history: Vec<String>,
}

#[allow(dead_code)]
impl MetaLibStateMachine {
    pub fn new() -> Self {
        MetaLibStateMachine {
            state: MetaLibState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }

    pub fn transition_to(&mut self, new_state: MetaLibState) -> bool {
        if self.state.is_terminal() {
            return false;
        }
        let desc = format!("{:?} -> {:?}", self.state, new_state);
        self.state = new_state;
        self.transitions += 1;
        self.history.push(desc);
        true
    }

    pub fn start(&mut self) -> bool {
        self.transition_to(MetaLibState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(MetaLibState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(MetaLibState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(MetaLibState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}

impl Default for MetaLibStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

/// A work queue for MetaLib items.
#[allow(dead_code)]
pub struct MetaLibWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}

#[allow(dead_code)]
impl MetaLibWorkQueue {
    pub fn new(capacity: usize) -> Self {
        MetaLibWorkQueue {
            pending: std::collections::VecDeque::new(),
            processed: Vec::new(),
            capacity,
        }
    }

    pub fn enqueue(&mut self, item: String) -> bool {
        if self.pending.len() >= self.capacity {
            return false;
        }
        self.pending.push_back(item);
        true
    }

    pub fn dequeue(&mut self) -> Option<String> {
        let item = self.pending.pop_front()?;
        self.processed.push(item.clone());
        Some(item)
    }

    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    pub fn processed_count(&self) -> usize {
        self.processed.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    pub fn is_full(&self) -> bool {
        self.pending.len() >= self.capacity
    }
    pub fn total_processed(&self) -> usize {
        self.processed.len()
    }
}

/// A counter map for MetaLib frequency analysis.
#[allow(dead_code)]
pub struct MetaLibCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}

#[allow(dead_code)]
impl MetaLibCounterMap {
    pub fn new() -> Self {
        MetaLibCounterMap {
            counts: std::collections::HashMap::new(),
            total: 0,
        }
    }

    pub fn increment(&mut self, key: &str) {
        *self.counts.entry(key.to_string()).or_insert(0) += 1;
        self.total += 1;
    }

    pub fn count(&self, key: &str) -> usize {
        *self.counts.get(key).unwrap_or(&0)
    }

    pub fn frequency(&self, key: &str) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.count(key) as f64 / self.total as f64
        }
    }

    pub fn most_common(&self) -> Option<(&String, usize)> {
        self.counts
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(k, &v)| (k, v))
    }

    pub fn num_unique(&self) -> usize {
        self.counts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
}

impl Default for MetaLibCounterMap {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod metalib_ext2_tests {
    use super::*;

    #[test]
    fn test_metalib_ext_util_basic() {
        let mut u = MetaLibExtUtil::new("test");
        u.push(10);
        u.push(20);
        assert_eq!(u.sum(), 30);
        assert_eq!(u.len(), 2);
    }

    #[test]
    fn test_metalib_ext_util_min_max() {
        let mut u = MetaLibExtUtil::new("mm");
        u.push(5);
        u.push(1);
        u.push(9);
        assert_eq!(u.min_val(), Some(1));
        assert_eq!(u.max_val(), Some(9));
    }

    #[test]
    fn test_metalib_ext_util_flags() {
        let mut u = MetaLibExtUtil::new("flags");
        u.set_flag(3);
        assert!(u.has_flag(3));
        assert!(!u.has_flag(2));
    }

    #[test]
    fn test_metalib_ext_util_pop() {
        let mut u = MetaLibExtUtil::new("pop");
        u.push(42);
        assert_eq!(u.pop(), Some(42));
        assert!(u.is_empty());
    }

    #[test]
    fn test_metalib_ext_map_basic() {
        let mut m: MetaLibExtMap<i32> = MetaLibExtMap::new();
        m.insert("key", 42);
        assert_eq!(m.get("key"), Some(&42));
        assert!(m.contains("key"));
        assert!(!m.contains("other"));
    }

    #[test]
    fn test_metalib_ext_map_get_or_default() {
        let mut m: MetaLibExtMap<i32> = MetaLibExtMap::new();
        m.insert("k", 5);
        assert_eq!(m.get_or_default("k"), 5);
        assert_eq!(m.get_or_default("missing"), 0);
    }

    #[test]
    fn test_metalib_ext_map_keys_sorted() {
        let mut m: MetaLibExtMap<i32> = MetaLibExtMap::new();
        m.insert("z", 1);
        m.insert("a", 2);
        m.insert("m", 3);
        let keys = m.keys_sorted();
        assert_eq!(keys[0].as_str(), "a");
        assert_eq!(keys[2].as_str(), "z");
    }

    #[test]
    fn test_metalib_window_mean() {
        let mut w = MetaLibWindow::new(3);
        w.push(1.0);
        w.push(2.0);
        w.push(3.0);
        assert!((w.mean() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_metalib_window_evict() {
        let mut w = MetaLibWindow::new(2);
        w.push(10.0);
        w.push(20.0);
        w.push(30.0); // evicts 10.0
        assert_eq!(w.len(), 2);
        assert!((w.mean() - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_metalib_window_std_dev() {
        let mut w = MetaLibWindow::new(10);
        for i in 0..10 {
            w.push(i as f64);
        }
        assert!(w.std_dev() > 0.0);
    }

    #[test]
    fn test_metalib_builder_basic() {
        let b = MetaLibBuilder::new("test")
            .add_item("a")
            .add_item("b")
            .set_config("key", "val");
        assert_eq!(b.item_count(), 2);
        assert!(b.has_config("key"));
        assert_eq!(b.get_config("key"), Some("val"));
    }

    #[test]
    fn test_metalib_builder_summary() {
        let b = MetaLibBuilder::new("suite").add_item("x");
        let s = b.build_summary();
        assert!(s.contains("suite"));
    }

    #[test]
    fn test_metalib_state_machine_start() {
        let mut sm = MetaLibStateMachine::new();
        assert!(sm.start());
        assert!(sm.state.is_running());
    }

    #[test]
    fn test_metalib_state_machine_complete() {
        let mut sm = MetaLibStateMachine::new();
        sm.start();
        sm.complete();
        assert!(sm.state.is_terminal());
    }

    #[test]
    fn test_metalib_state_machine_fail() {
        let mut sm = MetaLibStateMachine::new();
        sm.fail("oops");
        assert!(sm.state.is_terminal());
        assert_eq!(sm.state.error_msg(), Some("oops"));
    }

    #[test]
    fn test_metalib_state_machine_no_transition_after_terminal() {
        let mut sm = MetaLibStateMachine::new();
        sm.complete();
        assert!(!sm.start()); // Already terminal
    }

    #[test]
    fn test_metalib_work_queue_basic() {
        let mut wq = MetaLibWorkQueue::new(10);
        wq.enqueue("task1".to_string());
        wq.enqueue("task2".to_string());
        assert_eq!(wq.pending_count(), 2);
        let t = wq.dequeue();
        assert_eq!(t, Some("task1".to_string()));
        assert_eq!(wq.processed_count(), 1);
    }

    #[test]
    fn test_metalib_work_queue_capacity() {
        let mut wq = MetaLibWorkQueue::new(2);
        wq.enqueue("a".to_string());
        wq.enqueue("b".to_string());
        assert!(wq.is_full());
        assert!(!wq.enqueue("c".to_string()));
    }

    #[test]
    fn test_metalib_counter_map_basic() {
        let mut cm = MetaLibCounterMap::new();
        cm.increment("apple");
        cm.increment("apple");
        cm.increment("banana");
        assert_eq!(cm.count("apple"), 2);
        assert_eq!(cm.count("banana"), 1);
        assert_eq!(cm.num_unique(), 2);
    }

    #[test]
    fn test_metalib_counter_map_frequency() {
        let mut cm = MetaLibCounterMap::new();
        cm.increment("a");
        cm.increment("a");
        cm.increment("b");
        assert!((cm.frequency("a") - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_metalib_counter_map_most_common() {
        let mut cm = MetaLibCounterMap::new();
        cm.increment("x");
        cm.increment("y");
        cm.increment("x");
        let (k, v) = cm
            .most_common()
            .expect("most_common should return a value after increments");
        assert_eq!(k.as_str(), "x");
        assert_eq!(v, 2);
    }
}

// ============================================================
// Extended: Lib Analysis Infrastructure
// ============================================================

/// A result type for Lib analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum LibResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}

#[allow(dead_code)]
impl LibResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, LibResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, LibResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, LibResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, LibResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            LibResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            LibResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            LibResult::Ok(_) => 1.0,
            LibResult::Err(_) => 0.0,
            LibResult::Skipped => 0.0,
            LibResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}

/// An analysis pass for Lib.
#[allow(dead_code)]
pub struct LibAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<LibResult>,
    pub total_runs: usize,
}

#[allow(dead_code)]
impl LibAnalysisPass {
    pub fn new(name: &str) -> Self {
        LibAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }

    pub fn run(&mut self, input: &str) -> LibResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            LibResult::Err("empty input".to_string())
        } else {
            LibResult::Ok(format!("processed: {}", input))
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

/// A pipeline of Lib analysis passes.
#[allow(dead_code)]
pub struct LibPipeline {
    pub passes: Vec<LibAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}

#[allow(dead_code)]
impl LibPipeline {
    pub fn new(name: &str) -> Self {
        LibPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }

    pub fn add_pass(&mut self, pass: LibAnalysisPass) {
        self.passes.push(pass);
    }

    pub fn run_all(&mut self, input: &str) -> Vec<LibResult> {
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

/// A diff for Lib analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LibDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

#[allow(dead_code)]
impl LibDiff {
    pub fn new() -> Self {
        LibDiff {
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

impl Default for LibDiff {
    fn default() -> Self {
        Self::new()
    }
}

/// A typed slot for Lib configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LibConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

#[allow(dead_code)]
impl LibConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            LibConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            LibConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            LibConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            LibConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            LibConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            LibConfigValue::Bool(_) => "bool",
            LibConfigValue::Int(_) => "int",
            LibConfigValue::Float(_) => "float",
            LibConfigValue::Str(_) => "str",
            LibConfigValue::List(_) => "list",
        }
    }
}

/// A configuration store for Lib.
#[allow(dead_code)]
pub struct LibConfig {
    pub values: std::collections::HashMap<String, LibConfigValue>,
    pub read_only: bool,
}

#[allow(dead_code)]
impl LibConfig {
    pub fn new() -> Self {
        LibConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }

    pub fn set(&mut self, key: &str, value: LibConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }

    pub fn get(&self, key: &str) -> Option<&LibConfigValue> {
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
        self.set(key, LibConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, LibConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, LibConfigValue::Str(v.to_string()))
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

impl Default for LibConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// A diagnostic reporter for Lib.
#[allow(dead_code)]
pub struct LibDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

#[allow(dead_code)]
impl LibDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        LibDiagnostics {
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

#[cfg(test)]
mod lib_analysis_tests {
    use super::*;

    #[test]
    fn test_lib_result_ok() {
        let r = LibResult::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_lib_result_err() {
        let r = LibResult::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_lib_result_partial() {
        let r = LibResult::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_lib_result_skipped() {
        let r = LibResult::Skipped;
        assert!(r.is_skipped());
    }

    #[test]
    fn test_lib_analysis_pass_run() {
        let mut p = LibAnalysisPass::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }

    #[test]
    fn test_lib_analysis_pass_empty_input() {
        let mut p = LibAnalysisPass::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }

    #[test]
    fn test_lib_analysis_pass_success_rate() {
        let mut p = LibAnalysisPass::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_lib_analysis_pass_disable() {
        let mut p = LibAnalysisPass::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }

    #[test]
    fn test_lib_pipeline_basic() {
        let mut pipeline = LibPipeline::new("main_pipeline");
        pipeline.add_pass(LibAnalysisPass::new("pass1"));
        pipeline.add_pass(LibAnalysisPass::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_lib_pipeline_disabled_pass() {
        let mut pipeline = LibPipeline::new("partial");
        let mut p = LibAnalysisPass::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(LibAnalysisPass::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_lib_diff_basic() {
        let mut d = LibDiff::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }

    #[test]
    fn test_lib_diff_summary() {
        let mut d = LibDiff::new();
        d.add("x");
        d.add("y");
        d.remove("z");
        let s = d.summary();
        assert!(s.contains("+2"));
    }

    #[test]
    fn test_lib_config_set_get() {
        let mut cfg = LibConfig::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }

    #[test]
    fn test_lib_config_read_only() {
        let mut cfg = LibConfig::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false)); // should fail
        assert_eq!(cfg.get_bool("key"), Some(true)); // unchanged
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }

    #[test]
    fn test_lib_config_remove() {
        let mut cfg = LibConfig::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }

    #[test]
    fn test_lib_diagnostics_basic() {
        let mut diag = LibDiagnostics::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }

    #[test]
    fn test_lib_diagnostics_max_errors() {
        let mut diag = LibDiagnostics::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3"); // e3 dropped
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }

    #[test]
    fn test_lib_diagnostics_clear() {
        let mut diag = LibDiagnostics::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }

    #[test]
    fn test_lib_config_value_types() {
        let b = LibConfigValue::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);

        let i = LibConfigValue::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));

        let f = LibConfigValue::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("Float variant should return as_float") - 2.5).abs() < 1e-10);

        let s = LibConfigValue::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));

        let l = LibConfigValue::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}

// --- Extended analysis infrastructure for lib ---

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LibExtResult1300 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}

impl LibExtResult1300 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, LibExtResult1300::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, LibExtResult1300::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, LibExtResult1300::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, LibExtResult1300::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let LibExtResult1300::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let LibExtResult1300::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            LibExtResult1300::Ok(_) => 1.0,
            LibExtResult1300::Err(_) => 0.0,
            LibExtResult1300::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            LibExtResult1300::Skipped => 0.5,
        }
    }
}

#[allow(dead_code)]
pub struct LibExtPass1300 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<LibExtResult1300>,
}

impl LibExtPass1300 {
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
    pub fn run(&mut self, input: &str) -> LibExtResult1300 {
        if !self.enabled {
            return LibExtResult1300::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            LibExtResult1300::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            LibExtResult1300::Ok(format!(
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
pub struct LibExtPipeline1300 {
    pub name: String,
    pub passes: Vec<LibExtPass1300>,
    pub run_count: usize,
}

impl LibExtPipeline1300 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: LibExtPass1300) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<LibExtResult1300> {
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

#[allow(dead_code)]
pub struct LibExtDiff1300 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}

impl LibExtDiff1300 {
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

impl Default for LibExtDiff1300 {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum LibExtConfigVal1300 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}

impl LibExtConfigVal1300 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let LibExtConfigVal1300::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let LibExtConfigVal1300::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let LibExtConfigVal1300::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let LibExtConfigVal1300::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let LibExtConfigVal1300::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            LibExtConfigVal1300::Bool(_) => "bool",
            LibExtConfigVal1300::Int(_) => "int",
            LibExtConfigVal1300::Float(_) => "float",
            LibExtConfigVal1300::Str(_) => "str",
            LibExtConfigVal1300::List(_) => "list",
        }
    }
}

#[allow(dead_code)]
pub struct LibExtConfig1300 {
    pub values: std::collections::HashMap<String, LibExtConfigVal1300>,
    pub read_only: bool,
    pub name: String,
}

impl LibExtConfig1300 {
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
    pub fn set(&mut self, key: &str, value: LibExtConfigVal1300) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&LibExtConfigVal1300> {
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
        self.set(key, LibExtConfigVal1300::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, LibExtConfigVal1300::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, LibExtConfigVal1300::Str(v.to_string()))
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

impl Default for LibExtConfig1300 {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
pub struct LibExtDiag1300 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}

impl LibExtDiag1300 {
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

#[cfg(test)]
mod lib_ext_tests_1300 {
    use super::*;

    #[test]
    fn test_lib_ext_result_ok_1300() {
        let r = LibExtResult1300::Ok("success".to_string());
        assert!(r.is_ok());
        assert!(!r.is_err());
        assert_eq!(r.ok_msg(), Some("success"));
        assert!((r.progress() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_lib_ext_result_err_1300() {
        let r = LibExtResult1300::Err("failure".to_string());
        assert!(r.is_err());
        assert_eq!(r.err_msg(), Some("failure"));
        assert!((r.progress() - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_lib_ext_result_partial_1300() {
        let r = LibExtResult1300::Partial { done: 3, total: 10 };
        assert!(r.is_partial());
        assert!((r.progress() - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_lib_ext_result_skipped_1300() {
        let r = LibExtResult1300::Skipped;
        assert!(r.is_skipped());
    }

    #[test]
    fn test_lib_ext_pass_run_1300() {
        let mut p = LibExtPass1300::new("test_pass");
        let r = p.run("hello");
        assert!(r.is_ok());
        assert_eq!(p.total_runs, 1);
        assert_eq!(p.success_count(), 1);
    }

    #[test]
    fn test_lib_ext_pass_empty_1300() {
        let mut p = LibExtPass1300::new("empty_test");
        let r = p.run("");
        assert!(r.is_err());
        assert_eq!(p.error_count(), 1);
    }

    #[test]
    fn test_lib_ext_pass_rate_1300() {
        let mut p = LibExtPass1300::new("rate_test");
        p.run("a");
        p.run("b");
        p.run("");
        assert!((p.success_rate() - 2.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn test_lib_ext_pass_disable_1300() {
        let mut p = LibExtPass1300::new("disable_test");
        p.disable();
        assert!(!p.enabled);
        p.enable();
        assert!(p.enabled);
    }

    #[test]
    fn test_lib_ext_pipeline_basic_1300() {
        let mut pipeline = LibExtPipeline1300::new("main_pipeline");
        pipeline.add_pass(LibExtPass1300::new("pass1"));
        pipeline.add_pass(LibExtPass1300::new("pass2"));
        assert_eq!(pipeline.num_passes(), 2);
        let results = pipeline.run_all("test_input");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_lib_ext_pipeline_disabled_1300() {
        let mut pipeline = LibExtPipeline1300::new("partial");
        let mut p = LibExtPass1300::new("disabled");
        p.disable();
        pipeline.add_pass(p);
        pipeline.add_pass(LibExtPass1300::new("enabled"));
        assert_eq!(pipeline.num_enabled_passes(), 1);
        let results = pipeline.run_all("input");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_lib_ext_diff_basic_1300() {
        let mut d = LibExtDiff1300::new();
        d.add("new_item");
        d.remove("old_item");
        d.keep("same_item");
        assert!(!d.is_empty());
        assert_eq!(d.total_changes(), 2);
        assert_eq!(d.net_additions(), 0);
    }

    #[test]
    fn test_lib_ext_config_set_get_1300() {
        let mut cfg = LibExtConfig1300::new();
        cfg.set_bool("debug", true);
        cfg.set_int("max_iter", 100);
        cfg.set_str("name", "test");
        assert_eq!(cfg.get_bool("debug"), Some(true));
        assert_eq!(cfg.get_int("max_iter"), Some(100));
        assert_eq!(cfg.get_str("name"), Some("test"));
    }

    #[test]
    fn test_lib_ext_config_read_only_1300() {
        let mut cfg = LibExtConfig1300::new();
        cfg.set_bool("key", true);
        cfg.lock();
        assert!(!cfg.set_bool("key", false));
        assert_eq!(cfg.get_bool("key"), Some(true));
        cfg.unlock();
        assert!(cfg.set_bool("key", false));
    }

    #[test]
    fn test_lib_ext_config_remove_1300() {
        let mut cfg = LibExtConfig1300::new();
        cfg.set_int("x", 42);
        assert!(cfg.has("x"));
        cfg.remove("x");
        assert!(!cfg.has("x"));
    }

    #[test]
    fn test_lib_ext_diagnostics_basic_1300() {
        let mut diag = LibExtDiag1300::new(10);
        diag.error("something went wrong");
        diag.warning("maybe check this");
        diag.note("fyi");
        assert!(diag.has_errors());
        assert!(!diag.is_clean());
        assert_eq!(diag.num_errors(), 1);
        assert_eq!(diag.num_warnings(), 1);
    }

    #[test]
    fn test_lib_ext_diagnostics_max_errors_1300() {
        let mut diag = LibExtDiag1300::new(2);
        diag.error("e1");
        diag.error("e2");
        diag.error("e3");
        assert_eq!(diag.num_errors(), 2);
        assert!(diag.at_error_limit());
    }

    #[test]
    fn test_lib_ext_diagnostics_clear_1300() {
        let mut diag = LibExtDiag1300::new(10);
        diag.error("e1");
        diag.clear();
        assert!(diag.is_clean());
    }

    #[test]
    fn test_lib_ext_config_value_types_1300() {
        let b = LibExtConfigVal1300::Bool(true);
        assert_eq!(b.type_name(), "bool");
        assert_eq!(b.as_bool(), Some(true));
        assert_eq!(b.as_int(), None);

        let i = LibExtConfigVal1300::Int(42);
        assert_eq!(i.type_name(), "int");
        assert_eq!(i.as_int(), Some(42));

        let f = LibExtConfigVal1300::Float(2.5);
        assert_eq!(f.type_name(), "float");
        assert!((f.as_float().expect("Float variant should return as_float") - 2.5).abs() < 1e-10);

        let s = LibExtConfigVal1300::Str("hello".to_string());
        assert_eq!(s.type_name(), "str");
        assert_eq!(s.as_str(), Some("hello"));

        let l = LibExtConfigVal1300::List(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(l.type_name(), "list");
        assert_eq!(l.as_list().map(|v| v.len()), Some(2));
    }
}
