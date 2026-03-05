#![allow(unused_imports)]

//! # OxiLean Kernel — Trusted Computing Base
//!
//! The minimal, auditable kernel implementing the **Calculus of Inductive Constructions (CiC)**
//! with universe polymorphism. This crate is the **trusted core** of OxiLean.
//!
//! ## Philosophy
//!
//! - **Minimal TCB**: Only ~3,500 SLOC need to be trusted for soundness
//! - **Zero dependencies**: Uses only `std` (and is `no_std`-compatible where feasible)
//! - **No unsafe**: `#![forbid(unsafe_code)]` ensures memory safety via Rust's guarantees
//! - **Layered trust**: Bugs in the parser or elaborator cannot produce unsound proofs
//! - **Arena-based memory**: All expressions allocated in typed arenas for O(1) equality
//! - **WASM-first**: Compiles to `wasm32-unknown-unknown` with no system calls or FFI
//!
//! ## Quick Start
//!
//! ### Creating a Type
//!
//! ```ignore
//! use oxilean_kernel::{Expr, Environment, Level};
//!
//! let env = Environment::new();
//! // Nat : Type 0
//! let nat_type = Expr::Sort(Level::Zero);
//! ```
//!
//! ### Type Checking
//!
//! ```ignore
//! use oxilean_kernel::{TypeChecker, KernelError};
//!
//! let mut checker = TypeChecker::new(&env);
//! // Type-check an expression
//! let result = checker.infer(expr)?;
//! ```
//!
//! ## Architecture Overview
//!
//! The kernel is organized into three layers:
//!
//! ### Layer 1: Core Data Structures
//!
//! - **`arena`** — Typed arena allocator (`Arena<T>`, `Idx<T>`)
//! - **`name`** — Hierarchical identifiers (`Nat.add.comm`)
//! - **`level`** — Universe levels (`Zero`, `Succ`, `Max`, `IMax`, `Param`)
//! - **`expr`** — Core AST (11 node types: `BVar`, `FVar`, `Sort`, `Const`, `App`, `Lam`, `Pi`, `Let`, `Lit`, `Proj`, `Rec`)
//!
//! ### Layer 2: Environment & Declarations
//!
//! - **`env`** — Global environment holding axioms, definitions, inductives, etc.
//! - **`declaration`** — Declaration types: `Axiom`, `Def`, `Theorem`, `Inductive`, `Quotient`, `Recursor`, `Opaque`, `Constructor`
//! - **`context`** — Local variable contexts and context snapshots
//!
//! ### Layer 3: Type Theory Engine
//!
//! - **`infer`** — Type inference: `infer_type(expr) → Type`
//! - **`def_eq`** — Definitional equality: `is_def_eq(t1, t2) → bool`
//! - **`whnf`** — Weak head normal form: `whnf(expr) → Expr`
//! - **`check`** — Declaration checking and environment validation
//!
//! ### Layer 4: Reduction & Normalization
//!
//! - **`beta`** — β-reduction (function application)
//! - **`eta`** — η-reduction (function extensionality)
//! - **`simp`** — Simplification using equations
//! - **`reduce`** — Generic reducer trait with reducibility hints
//! - **`normalize`** — Full normal form computation
//!
//! ### Layer 5: Inductive Types & Recursion
//!
//! - **`inductive`** — Inductive type validation and recursor synthesis
//! - **`match_compile`** — Pattern matching compilation to decision trees
//! - **`quotient`** — Quotient type operations
//! - **`termination`** — Structural recursion validation
//!
//! ## Key Concepts & Terminology
//!
//! ### Calculus of Inductive Constructions (CiC)
//!
//! OxiLean implements a variant of CiC featuring:
//! - **Dependent types**: `Π (x : A), B x` (types can depend on values)
//! - **Universe polymorphism**: Definitions can abstract over type universe levels
//! - **Inductive types**: Type families with auto-generated recursion principles
//! - **Impredicativity**: `Prop : Prop` (proofs of propositions are proof-irrelevant)
//! - **Quotient types**: Extensional equality for abstract types\
//!
//! ### Trust Boundary
//!
//! Only the kernel is trusted. External code paths:
//! 1. **Parser** → produces AST (can contain errors/malice)
//! 2. **Elaborator** → converts AST to kernel terms (can produce invalid proofs)
//! 3. **Kernel** → validates every declaration independently (soundness gate)
//!
//! If parser or elaborator is compromised, users get proof failures, never unsoundness.
//!
//! ### Soundness Properties
//!
//! - Every type-checkable declaration is sound by construction
//! - No `unsafe` code means memory safety is Rust's responsibility
//! - Axiom tracking prevents unsound axioms from polluting proofs
//! - Universe polymorphism respects level constraints
//!
//! ### Memory Layout
//!
//! Expressions live in **typed arenas**:
//! ```text
//! Environment
//!   ├─ expr_arena: Arena<Expr>     // All Expr nodes
//!   ├─ level_arena: Arena<Level>   // All universe levels
//!   └─ name_arena: Arena<Name>     // All identifiers
//!
//! Idx<Expr> = u32 (not a pointer, just an index)
//! → O(1) equality via index comparison
//! → Cache-friendly dense layout
//! → Deterministic, no GC pauses
//! ```
//!
//! ### Reduction Strategy
//!
//! - **WHNF (Weak Head Normal Form)**: Reduces until top-level constructor
//!   - `λ x. t` stays as lambda
//!   - `App (λ x. t) a` reduces to `t[a/x]`
//!   - Inductive pattern match reduces to branch
//! - **Full Normal Form**: WHNF + reduce inside binders
//! - **Eta-expansion**: Insert `λ x. f x` for functions
//!
//! ### Locally Nameless Representation
//!
//! - **Bound variables** (inside binders): de Bruijn indices (`BVar(0)` = innermost binder)
//! - **Free variables** (global/local): unique IDs (`FVar(id)`)
//! - **Converting between**: `abstract` (close over binder), `instantiate` (substitute FVar)\
//!
//! ## Module Organization
//!
//! ### Foundational Modules\
//!
//! | Module | SLOC | Purpose |
//! |--------|------|---------|
//! | `arena` | ~150 | Typed arena allocator |
//! | `name` | ~200 | Hierarchical names |
//! | `level` | ~300 | Universe level computation |
//! | `expr` | ~400 | Expression AST definition |
//! | `subst` | ~150 | Substitution/instantiation |
//! | `context` | ~100 | Local context management |
//!
//! ### Type Checking Core
//!
//! | Module | SLOC | Purpose |
//! |--------|------|---------|
//! | `infer` | ~500 | Type inference |
//! | `def_eq` | ~400 | Definitional equality |
//! | `whnf` | ~300 | Weak head normal form |
//! | `check` | ~250 | Declaration validation |
//!
//! ### Reduction & Normalization
//!
//! | Module | SLOC | Purpose |
//! |--------|------|---------|
//! | `beta` | ~200 | Beta reduction |
//! | `eta` | ~150 | Eta reduction |
//! | `reduce` | ~200 | Generic reduction infrastructure |
//! | `simp` | ~300 | Simplification engine |
//! | `normalize` | ~150 | Full normalization |
//!
//! ### Type Families
//!
//! | Module | SLOC | Purpose |
//! |--------|------|---------|
//! | `inductive` | ~500 | Inductive types and recursors |
//! | `quotient` | ~300 | Quotient types |
//! | `match_compile` | ~400 | Pattern match compilation |
//! | `termination` | ~350 | Recursion validation |
//!
//! ### Utilities
//!
//! | Module | SLOC | Purpose |
//! |--------|------|---------|
//! | `alpha` | ~200 | Alpha equivalence |
//! | `axiom` | ~250 | Axiom tracking and safety |
//! | `congruence` | ~300 | Congruence closure |
//! | `export` | ~400 | Module serialization |
//! | `prettyprint` | ~350 | Expression printing |
//! | `trace` | ~200 | Debug tracing |
//!
//! ## Usage Examples
//!
//! ### Example 1: Check if Two Terms Are Definitionally Equal
//!
//! ```ignore
//! use oxilean_kernel::{DefEqChecker, Expr};
//!
//! let checker = DefEqChecker::new(&env);
//! let eq = checker.is_def_eq(expr1, expr2)?;
//! assert!(eq);
//! ```
//!
//! ### Example 2: Normalize an Expression
//!
//! ```ignore
//! use oxilean_kernel::normalize;
//!
//! let normal = normalize(&env, expr)?;
//! // normal is now in full normal form\
//! ```
//!
//! ### Example 3: Work with Inductive Types
//!
//! ```ignore
//! use oxilean_kernel::{InductiveType, check_inductive};
//!
//! let nat_ind = InductiveType::new(/* ... */);
//! check_inductive(&env, &nat_ind)?;
//! // nat_ind is validated and recursors are synthesized
//! ```
//!
//! ## Integration with Other Crates
//!
//! ### With `oxilean-meta`
//!
//! The meta layer (`oxilean-meta`) extends the kernel with:
//! - **Metavariable support**: Holes `?m` for unification
//! - **Meta WHNF**: WHNF aware of unsolved metavars
//! - **Meta DefEq**: Unification that assigns metavars
//! - **App builder**: Helpers for constructing proof terms
//!
//! ### With `oxilean-elab`
//!
//! The elaborator (`oxilean-elab`) uses the kernel for:
//! - **Type checking**: Validates elaborated proofs via `TypeChecker`
//! - **Definitional equality**: Checks `is_def_eq` during elaboration
//! - **Declaration checking**: Ensures all definitions are sound
//!
//! ### With `oxilean-parse`
//!
//! The parser provides surface syntax (`Expr` types) that the elaborator converts to kernel `Expr`.
//!
//! ## Soundness Guarantees
//!
//! OxiLean provides the following soundness invariants:
//!
//! 1. **Type Safety**: No expression can be assigned a type that doesn't logically follow
//! 2. **Axiom Tracking**: All uses of axioms are recorded; proofs can be reviewed for axiomatic assumptions
//! 3. **Termination**: Recursive definitions are proven terminating before acceptance
//! 4. **Universe Consistency**: No universe cycles or level violations
//! 5. **Proof Irrelevance**: All proofs of the same `Prop` are definitionally equal
//!
//! ## Performance Characteristics
//!
//! - **Expression comparison**: O(1) via arena indexing
//! - **WHNF reduction**: O(depth × reduction_steps) for typical programs
//! - **Environment lookup**: O(log n) (indexed environment)
//! - **Memory**: Dense arena layout, no pointer chasing
//! - **GC**: None needed (Rust ownership handles deallocation)\
//!
//! ## Further Reading
//!
//! - [ARCHITECTURE.md](../../ARCHITECTURE.md) — System architecture
//! - [BLUEPRINT.md](../../BLUEPRINT.md) — Formal CiC specification
//! - Module documentation for specific subcomponents

#![forbid(unsafe_code)]
#![allow(missing_docs)]
#![warn(clippy::all)]

pub mod arena;
pub mod expr;
pub mod level;
pub mod name;

// Re-exports for convenience
pub use arena::{Arena, Idx};
pub use expr::{BinderInfo, Expr, FVarId, Literal};
pub use level::{Level, LevelMVarId};
pub use name::Name;
pub mod subst;

pub use subst::{abstract_expr, instantiate};
pub mod env;
pub mod reduce;

pub use env::{Declaration, EnvError, Environment};
pub use reduce::{reduce_nat_op, Reducer, ReducibilityHint};
pub mod error;
pub mod infer;

// Phase 3.1: Kernel Foundation modules
pub mod r#abstract;
pub mod declaration;
pub mod equiv_manager;
pub mod expr_cache;
pub mod expr_util;
pub mod instantiate;

pub use declaration::{
    instantiate_level_params, AxiomVal, ConstantInfo, ConstantVal, ConstructorVal,
    DefinitionSafety, DefinitionVal, InductiveVal, OpaqueVal, QuotKind, QuotVal, RecursorRule,
    RecursorVal, TheoremVal,
};
pub use equiv_manager::EquivManager;

pub use error::KernelError;
pub use infer::{LocalDecl, TypeChecker};
pub mod abstract_interp;
pub mod alpha;
pub mod axiom;
pub mod beta;
pub mod builtin;
pub mod check;
pub mod congruence;
pub mod context;
pub mod conversion;
pub mod def_eq;
pub mod eta;
pub mod export;
pub mod inductive;
pub mod match_compile;
pub mod normalize;
pub mod prettyprint;
pub mod proof;
pub mod quotient;
pub mod reduction;
pub mod serial;
pub mod simp;
pub mod struct_eta;
pub mod substitution;
pub mod termination;
pub mod trace;
pub mod type_erasure;
pub mod typeclasses;
pub mod universe;
pub mod whnf;

/// Benchmarking support utilities for the OxiLean kernel.
pub mod bench_support;
/// Caching infrastructure for performance optimization.
pub mod cache;
/// Foreign function interface support.
pub mod ffi;
/// No-std compatibility layer for constrained and WASM environments.
pub mod no_std_compat;

pub use alpha::{alpha_equiv, canonicalize};
pub use axiom::{
    classify_axiom, extract_axioms, has_unsafe_dependencies, transitive_axiom_deps, AxiomSafety,
    AxiomValidator,
};
pub use beta::{beta_normalize, beta_step, beta_under_binder, is_beta_normal, mk_beta_redex};
pub use builtin::{init_builtin_env, is_nat_op, is_string_op};
pub use check::{check_constant_info, check_constant_infos, check_declaration, check_declarations};
pub use congruence::{
    mk_congr_theorem, mk_congr_theorem_with_fixed, CongrArgKind, CongruenceClosure,
    CongruenceTheorem,
};
pub use context::{ContextSnapshot, NameGenerator};
pub use conversion::ConversionChecker;
pub use def_eq::{is_def_eq_simple, DefEqChecker};
pub use eta::{eta_contract, eta_expand, is_eta_expandable};
pub use export::{
    deserialize_module_header, export_environment, import_module, serialize_module, ExportedModule,
    ModuleCache,
};
pub use inductive::{check_inductive, reduce_recursor, InductiveEnv, InductiveType, IntroRule};
pub use match_compile::{
    CompileResult, ConstructorInfo as MatchConstructorInfo, DecisionTree, MatchArm, MatchCompiler,
    Pattern,
};
pub use normalize::{alpha_eq_env, evaluate, is_normal_form, normalize_env, normalize_whnf};
pub use prettyprint::{print_expr, print_expr_ascii, ExprPrinter};
pub use proof::ProofTerm;
pub use quotient::{
    check_equivalence_relation, check_quot_usage, is_quot_type_expr, quot_eq, reduce_quot_lift,
    QuotUsageKind, QuotientType,
};
pub use simp::{alpha_eq, normalize, simplify};
pub use termination::{ParamInfo, RecCallInfo, TerminationChecker, TerminationResult};
pub use trace::{TraceEvent, TraceLevel, Tracer};
pub use typeclasses::{
    is_class_constraint, Instance as KernelInstance, Method as KernelMethod,
    TypeClass as KernelTypeClass, TypeClassRegistry as KernelTypeClassRegistry,
};
pub use universe::{UnivChecker, UnivConstraint};
pub use whnf::{is_whnf, whnf, whnf_is_lambda, whnf_is_pi, whnf_is_sort};

// ============================================================
// Kernel utility helpers
// ============================================================

/// Version string for the OxiLean kernel.
pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Return the kernel version as a `(major, minor, patch)` tuple.
#[allow(dead_code)]
pub fn kernel_version() -> (u32, u32, u32) {
    let v = KERNEL_VERSION;
    let parts: Vec<u32> = v.split('.').filter_map(|s| s.parse().ok()).collect();
    match parts.as_slice() {
        [major, minor, patch, ..] => (*major, *minor, *patch),
        [major, minor] => (*major, *minor, 0),
        [major] => (*major, 0, 0),
        [] => (0, 0, 0),
    }
}

/// Convenience: make a `Prop` expression (Sort 0).
#[allow(dead_code)]
pub fn mk_prop() -> Expr {
    Expr::Sort(Level::zero())
}

/// Convenience: make `Type 0` (Sort 1).
#[allow(dead_code)]
pub fn mk_type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}

/// Convenience: make `Type 1` (Sort 2).
#[allow(dead_code)]
pub fn mk_type1() -> Expr {
    Expr::Sort(Level::succ(Level::succ(Level::zero())))
}

/// Convenience: make `Sort u` for an arbitrary level.
#[allow(dead_code)]
pub fn mk_sort(level: Level) -> Expr {
    Expr::Sort(level)
}

/// Convenience: make a `Nat` literal expression.
#[allow(dead_code)]
pub fn mk_nat_lit(n: u64) -> Expr {
    Expr::Lit(Literal::Nat(n))
}

/// Convenience: make a `String` literal expression.
#[allow(dead_code)]
pub fn mk_string_lit(s: &str) -> Expr {
    Expr::Lit(Literal::Str(s.to_string()))
}

/// Convenience: make `App(f, a)`.
#[allow(dead_code)]
pub fn mk_app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}

/// Build `f a1 a2 ... an` from a head `f` and argument list.
#[allow(dead_code)]
pub fn mk_app_spine(f: Expr, args: Vec<Expr>) -> Expr {
    args.into_iter().fold(f, mk_app)
}

/// Convenience: make a Pi-type `(x : dom) -> cod`.
#[allow(dead_code)]
pub fn mk_pi(name: Name, dom: Expr, cod: Expr) -> Expr {
    Expr::Pi(BinderInfo::Default, name, Box::new(dom), Box::new(cod))
}

/// Build a chain of Pi-types from a list of `(name, type)` binders and a result type.
#[allow(dead_code)]
pub fn mk_pi_chain(binders: Vec<(Name, Expr)>, ret: Expr) -> Expr {
    binders
        .into_iter()
        .rev()
        .fold(ret, |acc, (n, ty)| mk_pi(n, ty, acc))
}

/// Convenience: make a lambda `fun x : dom => body`.
#[allow(dead_code)]
pub fn mk_lam(name: Name, dom: Expr, body: Expr) -> Expr {
    Expr::Lam(BinderInfo::Default, name, Box::new(dom), Box::new(body))
}

/// Build a chain of lambdas from a list of `(name, type)` binders and a body.
#[allow(dead_code)]
pub fn mk_lam_chain(binders: Vec<(Name, Expr)>, body: Expr) -> Expr {
    binders
        .into_iter()
        .rev()
        .fold(body, |acc, (n, ty)| mk_lam(n, ty, acc))
}

/// Unfold the head and arguments of an `App` spine.
///
/// `f a b c` → `(f, [a, b, c])`
#[allow(dead_code)]
pub fn unfold_app(expr: &Expr) -> (&Expr, Vec<&Expr>) {
    let mut args = Vec::new();
    let mut cur = expr;
    while let Expr::App(f, a) = cur {
        args.push(a.as_ref());
        cur = f;
    }
    args.reverse();
    (cur, args)
}

/// Count the number of arguments an expression is applied to.
#[allow(dead_code)]
pub fn app_arity(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, _) => 1 + app_arity(f),
        _ => 0,
    }
}

/// Count Pi binders in a Pi-chain.
#[allow(dead_code)]
pub fn count_pi_binders(expr: &Expr) -> usize {
    match expr {
        Expr::Pi(_, _, _, body) => 1 + count_pi_binders(body),
        _ => 0,
    }
}

/// Count Lam binders in a Lambda-chain.
#[allow(dead_code)]
pub fn count_lam_binders(expr: &Expr) -> usize {
    match expr {
        Expr::Lam(_, _, _, body) => 1 + count_lam_binders(body),
        _ => 0,
    }
}

/// Check whether an expression is closed (contains no `BVar` with index ≥ depth).
#[allow(dead_code)]
pub fn is_closed(expr: &Expr) -> bool {
    is_closed_at(expr, 0)
}

fn is_closed_at(expr: &Expr, depth: u32) -> bool {
    match expr {
        Expr::BVar(i) => *i < depth,
        Expr::FVar(_) | Expr::Sort(_) | Expr::Const(_, _) | Expr::Lit(_) => true,
        Expr::App(f, a) => is_closed_at(f, depth) && is_closed_at(a, depth),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            is_closed_at(dom, depth) && is_closed_at(body, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            is_closed_at(ty, depth) && is_closed_at(val, depth) && is_closed_at(body, depth + 1)
        }
        Expr::Proj(_, _, e) => is_closed_at(e, depth),
    }
}

/// Check whether an expression contains no `FVar`, `MVar`, or `BVar` nodes.
#[allow(dead_code)]
pub fn is_ground(expr: &Expr) -> bool {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) => false,
        Expr::Sort(_) | Expr::Lit(_) => true,
        Expr::Const(_, _) => true,
        Expr::App(f, a) => is_ground(f) && is_ground(a),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => is_ground(dom) && is_ground(body),
        Expr::Let(_, ty, val, body) => is_ground(ty) && is_ground(val) && is_ground(body),
        Expr::Proj(_, _, e) => is_ground(e),
    }
}

/// Compute an approximate "size" of an expression (node count).
#[allow(dead_code)]
pub fn expr_size(expr: &Expr) -> usize {
    match expr {
        Expr::BVar(_) | Expr::FVar(_) | Expr::Sort(_) | Expr::Lit(_) | Expr::Const(_, _) => 1,
        Expr::App(f, a) => 1 + expr_size(f) + expr_size(a),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            1 + expr_size(dom) + expr_size(body)
        }
        Expr::Let(_, ty, val, body) => 1 + expr_size(ty) + expr_size(val) + expr_size(body),
        Expr::Proj(_, _, e) => 1 + expr_size(e),
    }
}

/// Check whether an expression contains any metavariable (`MVar`).
#[allow(dead_code)]
pub fn has_metavars(expr: &Expr) -> bool {
    match expr {
        Expr::App(f, a) => has_metavars(f) || has_metavars(a),
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            has_metavars(dom) || has_metavars(body)
        }
        Expr::Let(_, ty, val, body) => has_metavars(ty) || has_metavars(val) || has_metavars(body),
        Expr::Proj(_, _, e) => has_metavars(e),
        _ => false,
    }
}

/// Collect all `Const` names referenced in an expression.
#[allow(dead_code)]
pub fn collect_const_names(expr: &Expr) -> Vec<Name> {
    let mut names = Vec::new();
    collect_const_names_rec(expr, &mut names);
    names
}

fn collect_const_names_rec(expr: &Expr, acc: &mut Vec<Name>) {
    match expr {
        Expr::Const(n, _) => acc.push(n.clone()),
        Expr::App(f, a) => {
            collect_const_names_rec(f, acc);
            collect_const_names_rec(a, acc);
        }
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            collect_const_names_rec(dom, acc);
            collect_const_names_rec(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_const_names_rec(ty, acc);
            collect_const_names_rec(val, acc);
            collect_const_names_rec(body, acc);
        }
        Expr::Proj(_, _, e) => collect_const_names_rec(e, acc),
        _ => {}
    }
}

/// Collect all `FVar` ids referenced in an expression.
#[allow(dead_code)]
pub fn collect_fvars(expr: &Expr) -> Vec<FVarId> {
    let mut fvars = Vec::new();
    collect_fvars_rec(expr, &mut fvars);
    fvars
}

fn collect_fvars_rec(expr: &Expr, acc: &mut Vec<FVarId>) {
    match expr {
        Expr::FVar(id) => acc.push(*id),
        Expr::App(f, a) => {
            collect_fvars_rec(f, acc);
            collect_fvars_rec(a, acc);
        }
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            collect_fvars_rec(dom, acc);
            collect_fvars_rec(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_fvars_rec(ty, acc);
            collect_fvars_rec(val, acc);
            collect_fvars_rec(body, acc);
        }
        Expr::Proj(_, _, e) => collect_fvars_rec(e, acc),
        _ => {}
    }
}

/// Return the "head" of an expression (strip App arguments).
#[allow(dead_code)]
pub fn expr_head(expr: &Expr) -> &Expr {
    match expr {
        Expr::App(f, _) => expr_head(f),
        _ => expr,
    }
}

/// Check whether an expression is an `App` whose head is `Const(name)`.
#[allow(dead_code)]
pub fn is_app_of(expr: &Expr, name: &Name) -> bool {
    matches!(expr_head(expr), Expr::Const(n, _) if n == name)
}

/// Return the maximum de Bruijn index that appears free (not under enough binders).
///
/// Returns `None` if no `BVar` nodes occur.
#[allow(dead_code)]
pub fn max_bvar_index(expr: &Expr) -> Option<u32> {
    max_bvar_index_at(expr, 0)
}

fn max_bvar_index_at(expr: &Expr, depth: u32) -> Option<u32> {
    match expr {
        Expr::BVar(i) => {
            if *i >= depth {
                Some(*i - depth)
            } else {
                None
            }
        }
        Expr::App(f, a) => {
            let l = max_bvar_index_at(f, depth);
            let r = max_bvar_index_at(a, depth);
            match (l, r) {
                (Some(a), Some(b)) => Some(a.max(b)),
                (x, None) | (None, x) => x,
            }
        }
        Expr::Lam(_, _, dom, body) | Expr::Pi(_, _, dom, body) => {
            let l = max_bvar_index_at(dom, depth);
            let r = max_bvar_index_at(body, depth + 1);
            match (l, r) {
                (Some(a), Some(b)) => Some(a.max(b)),
                (x, None) | (None, x) => x,
            }
        }
        Expr::Let(_, ty, val, body) => {
            let t = max_bvar_index_at(ty, depth);
            let v = max_bvar_index_at(val, depth);
            let b = max_bvar_index_at(body, depth + 1);
            [t, v, b].into_iter().flatten().reduce(|a, b| a.max(b))
        }
        Expr::Proj(_, _, e) => max_bvar_index_at(e, depth),
        _ => None,
    }
}

#[cfg(test)]
mod kernel_util_tests {
    use super::*;

    #[test]
    fn test_kernel_version_parses() {
        let (major, minor, patch) = kernel_version();
        let _ = (major, minor, patch); // just ensure it doesn't panic
    }

    #[test]
    fn test_mk_prop() {
        assert!(matches!(mk_prop(), Expr::Sort(l) if l == Level::zero()));
    }

    #[test]
    fn test_mk_type0() {
        assert!(matches!(mk_type0(), Expr::Sort(l) if l == Level::succ(Level::zero())));
    }

    #[test]
    fn test_mk_nat_lit() {
        assert!(matches!(mk_nat_lit(42), Expr::Lit(Literal::Nat(_))));
    }

    #[test]
    fn test_mk_string_lit() {
        assert!(matches!(mk_string_lit("hello"), Expr::Lit(Literal::Str(_))));
    }

    #[test]
    fn test_mk_app_spine_empty() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let result = mk_app_spine(f.clone(), vec![]);
        assert_eq!(result, f);
    }

    #[test]
    fn test_mk_app_spine() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let result = mk_app_spine(f, vec![a, b]);
        assert!(matches!(result, Expr::App(_, _)));
        assert_eq!(app_arity(&result), 2);
    }

    #[test]
    fn test_unfold_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let a = Expr::Const(Name::str("a"), vec![]);
        let b = Expr::Const(Name::str("b"), vec![]);
        let e = mk_app_spine(f, vec![a, b]);
        let (head, args) = unfold_app(&e);
        assert!(matches!(head, Expr::Const(n, _) if n == &Name::str("f")));
        assert_eq!(args.len(), 2);
    }

    #[test]
    fn test_is_closed_bvar_0() {
        // BVar(0) is closed at depth=1
        let e = Expr::BVar(0);
        assert!(!is_closed(&e)); // depth=0, so BVar(0) is NOT closed
    }

    #[test]
    fn test_is_closed_const() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(is_closed(&e));
    }

    #[test]
    fn test_is_ground_const() {
        let e = Expr::Const(Name::str("Nat"), vec![]);
        assert!(is_ground(&e));
    }

    #[test]
    fn test_is_ground_fvar() {
        let e = Expr::FVar(FVarId(0));
        assert!(!is_ground(&e));
    }

    #[test]
    fn test_expr_size_atom() {
        assert_eq!(expr_size(&Expr::BVar(0)), 1);
        assert_eq!(expr_size(&Expr::Sort(Level::zero())), 1);
    }

    #[test]
    fn test_expr_size_app() {
        let e = mk_app(Expr::BVar(0), Expr::BVar(1));
        assert_eq!(expr_size(&e), 3); // App(BVar, BVar) = 1 + 1 + 1
    }

    #[test]
    fn test_has_metavars_false() {
        let e = Expr::Const(Name::str("f"), vec![]);
        assert!(!has_metavars(&e));
    }

    #[test]
    fn test_has_metavars_true() {
        // Metavariables are represented as FVar in OxiLean kernel; has_metavars is always false for plain exprs
        let e = Expr::FVar(FVarId(0));
        assert!(!has_metavars(&e));
    }

    #[test]
    fn test_collect_const_names() {
        let e = mk_app(
            Expr::Const(Name::str("f"), vec![]),
            Expr::Const(Name::str("a"), vec![]),
        );
        let names = collect_const_names(&e);
        assert!(names.contains(&Name::str("f")));
        assert!(names.contains(&Name::str("a")));
    }

    #[test]
    fn test_is_app_of() {
        let e = mk_app(
            Expr::Const(Name::str("List"), vec![]),
            Expr::Const(Name::str("Nat"), vec![]),
        );
        assert!(is_app_of(&e, &Name::str("List")));
        assert!(!is_app_of(&e, &Name::str("Nat")));
    }

    #[test]
    fn test_count_pi_binders() {
        let p = mk_pi(Name::str("x"), mk_prop(), mk_prop());
        assert_eq!(count_pi_binders(&p), 1);
    }

    #[test]
    fn test_count_lam_binders() {
        let l = mk_lam(Name::str("x"), mk_prop(), Expr::BVar(0));
        assert_eq!(count_lam_binders(&l), 1);
    }
}

// ── Additional kernel utilities ───────────────────────────────────────────────

/// Returns true if an expression contains any `Let` binders.
#[allow(dead_code)]
pub fn has_let_binders(expr: &Expr) -> bool {
    match expr {
        Expr::Let(_, ty, val, body) => {
            let _ = (
                has_let_binders(ty),
                has_let_binders(val),
                has_let_binders(body),
            );
            true
        }
        Expr::App(f, a) => has_let_binders(f) || has_let_binders(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_let_binders(ty) || has_let_binders(body)
        }
        Expr::Proj(_, _, e) => has_let_binders(e),
        _ => false,
    }
}

/// Returns true if an expression contains any projections.
#[allow(dead_code)]
pub fn has_projections(expr: &Expr) -> bool {
    match expr {
        Expr::Proj(_, _, _) => true,
        Expr::App(f, a) => has_projections(f) || has_projections(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            has_projections(ty) || has_projections(body)
        }
        Expr::Let(_, ty, val, body) => {
            has_projections(ty) || has_projections(val) || has_projections(body)
        }
        _ => false,
    }
}

/// Count the number of `App` nodes in an expression.
#[allow(dead_code)]
pub fn count_apps(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, a) => 1 + count_apps(f) + count_apps(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => count_apps(ty) + count_apps(body),
        Expr::Let(_, ty, val, body) => count_apps(ty) + count_apps(val) + count_apps(body),
        Expr::Proj(_, _, e) => count_apps(e),
        _ => 0,
    }
}

/// Count all sort occurrences in an expression.
#[allow(dead_code)]
pub fn count_sorts(expr: &Expr) -> usize {
    match expr {
        Expr::Sort(_) => 1,
        Expr::App(f, a) => count_sorts(f) + count_sorts(a),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => count_sorts(ty) + count_sorts(body),
        Expr::Let(_, ty, val, body) => count_sorts(ty) + count_sorts(val) + count_sorts(body),
        Expr::Proj(_, _, e) => count_sorts(e),
        _ => 0,
    }
}

/// Check whether an expression is a literal.
#[allow(dead_code)]
pub fn is_literal(expr: &Expr) -> bool {
    matches!(expr, Expr::Lit(_))
}

/// Check whether an expression is a sort.
#[allow(dead_code)]
pub fn is_sort(expr: &Expr) -> bool {
    matches!(expr, Expr::Sort(_))
}

/// Check whether an expression is a Pi-type (possibly nested).
#[allow(dead_code)]
pub fn is_pi(expr: &Expr) -> bool {
    matches!(expr, Expr::Pi(_, _, _, _))
}

/// Check whether an expression is a lambda.
#[allow(dead_code)]
pub fn is_lam(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(_, _, _, _))
}

/// Check whether an expression is an application.
#[allow(dead_code)]
pub fn is_app(expr: &Expr) -> bool {
    matches!(expr, Expr::App(_, _))
}

/// Check whether an expression is a constant.
#[allow(dead_code)]
pub fn is_const(expr: &Expr) -> bool {
    matches!(expr, Expr::Const(_, _))
}

/// Get the name of a constant, or None if not a constant.
#[allow(dead_code)]
pub fn const_name(expr: &Expr) -> Option<&Name> {
    match expr {
        Expr::Const(n, _) => Some(n),
        _ => None,
    }
}

/// Strip outer Pi binders, collecting binder info.
///
/// Returns `(binders, inner_type)` where `binders` is a list of `(BinderInfo, Name, domain_type)`.
#[allow(dead_code)]
pub fn strip_pi_binders(expr: &Expr) -> (Vec<(BinderInfo, Name, Expr)>, &Expr) {
    let mut binders = Vec::new();
    let mut current = expr;
    while let Expr::Pi(bi, n, ty, body) = current {
        binders.push((*bi, n.clone(), ty.as_ref().clone()));
        current = body;
    }
    (binders, current)
}

/// Strip outer lambda binders, collecting binder info.
///
/// Returns `(binders, body)` where `binders` is a list of `(BinderInfo, Name, domain_type)`.
#[allow(dead_code)]
pub fn strip_lam_binders(expr: &Expr) -> (Vec<(BinderInfo, Name, Expr)>, &Expr) {
    let mut binders = Vec::new();
    let mut current = expr;
    while let Expr::Lam(bi, n, ty, body) = current {
        binders.push((*bi, n.clone(), ty.as_ref().clone()));
        current = body;
    }
    (binders, current)
}

/// Build a Pi type from a list of binders and an inner type.
#[allow(dead_code)]
pub fn build_pi_from_binders(binders: &[(BinderInfo, Name, Expr)], inner: Expr) -> Expr {
    binders.iter().rev().fold(inner, |acc, (bi, n, ty)| {
        Expr::Pi(*bi, n.clone(), Box::new(ty.clone()), Box::new(acc))
    })
}

/// Build a lambda from a list of binders and a body.
#[allow(dead_code)]
pub fn build_lam_from_binders(binders: &[(BinderInfo, Name, Expr)], body: Expr) -> Expr {
    binders.iter().rev().fold(body, |acc, (bi, n, ty)| {
        Expr::Lam(*bi, n.clone(), Box::new(ty.clone()), Box::new(acc))
    })
}

/// Replace all occurrences of a constant by another expression.
///
/// Traverses the expression and substitutes `replacement` for every
/// `Const(name, _)` node.
#[allow(dead_code)]
pub fn replace_const(expr: &Expr, name: &Name, replacement: &Expr) -> Expr {
    match expr {
        Expr::Const(n, _) if n == name => replacement.clone(),
        Expr::App(f, a) => Expr::App(
            Box::new(replace_const(f, name, replacement)),
            Box::new(replace_const(a, name, replacement)),
        ),
        Expr::Lam(bi, n, ty, body) => Expr::Lam(
            *bi,
            n.clone(),
            Box::new(replace_const(ty, name, replacement)),
            Box::new(replace_const(body, name, replacement)),
        ),
        Expr::Pi(bi, n, ty, body) => Expr::Pi(
            *bi,
            n.clone(),
            Box::new(replace_const(ty, name, replacement)),
            Box::new(replace_const(body, name, replacement)),
        ),
        Expr::Let(n, ty, val, body) => Expr::Let(
            n.clone(),
            Box::new(replace_const(ty, name, replacement)),
            Box::new(replace_const(val, name, replacement)),
            Box::new(replace_const(body, name, replacement)),
        ),
        Expr::Proj(n, i, s) => {
            Expr::Proj(n.clone(), *i, Box::new(replace_const(s, name, replacement)))
        }
        e => e.clone(),
    }
}

/// Check if an expression is eta-reducible at the top level.
///
/// An expression `λ x. f x` is eta-reducible to `f` when `x` does not occur free in `f`.
#[allow(dead_code)]
pub fn is_eta_reducible(expr: &Expr) -> bool {
    match expr {
        Expr::Lam(_, _, _, body) => {
            if let Expr::App(f, a) = body.as_ref() {
                if matches!(a.as_ref(), Expr::BVar(0)) {
                    // Check that BVar(0) doesn't appear free in f
                    return !contains_bvar(f, 0, 0);
                }
            }
            false
        }
        _ => false,
    }
}

/// Check if `BVar(idx + depth)` occurs in `expr` at the given depth.
#[allow(dead_code)]
pub fn contains_bvar(expr: &Expr, idx: u32, depth: u32) -> bool {
    match expr {
        Expr::BVar(i) => *i == idx + depth,
        Expr::App(f, a) => contains_bvar(f, idx, depth) || contains_bvar(a, idx, depth),
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            contains_bvar(ty, idx, depth) || contains_bvar(body, idx, depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            contains_bvar(ty, idx, depth)
                || contains_bvar(val, idx, depth)
                || contains_bvar(body, idx, depth + 1)
        }
        Expr::Proj(_, _, s) => contains_bvar(s, idx, depth),
        _ => false,
    }
}

/// Check if two expressions are syntactically equal (no alpha equivalence, just `==`).
#[allow(dead_code)]
pub fn syntactically_equal(e1: &Expr, e2: &Expr) -> bool {
    e1 == e2
}

/// Collect all literals occurring in an expression.
#[allow(dead_code)]
pub fn collect_literals(expr: &Expr) -> Vec<Literal> {
    let mut lits = Vec::new();
    collect_lits_rec(expr, &mut lits);
    lits
}

fn collect_lits_rec(expr: &Expr, acc: &mut Vec<Literal>) {
    match expr {
        Expr::Lit(l) => acc.push(l.clone()),
        Expr::App(f, a) => {
            collect_lits_rec(f, acc);
            collect_lits_rec(a, acc);
        }
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            collect_lits_rec(ty, acc);
            collect_lits_rec(body, acc);
        }
        Expr::Let(_, ty, val, body) => {
            collect_lits_rec(ty, acc);
            collect_lits_rec(val, acc);
            collect_lits_rec(body, acc);
        }
        Expr::Proj(_, _, e) => collect_lits_rec(e, acc),
        _ => {}
    }
}

/// Return the depth of the deepest nested binder.
#[allow(dead_code)]
pub fn max_binder_depth(expr: &Expr) -> u32 {
    max_binder_depth_impl(expr, 0)
}

fn max_binder_depth_impl(expr: &Expr, depth: u32) -> u32 {
    match expr {
        Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
            let ty_d = max_binder_depth_impl(ty, depth);
            let body_d = max_binder_depth_impl(body, depth + 1);
            ty_d.max(body_d).max(depth + 1)
        }
        Expr::Let(_, ty, val, body) => {
            let ty_d = max_binder_depth_impl(ty, depth);
            let val_d = max_binder_depth_impl(val, depth);
            let body_d = max_binder_depth_impl(body, depth + 1);
            ty_d.max(val_d).max(body_d)
        }
        Expr::App(f, a) => max_binder_depth_impl(f, depth).max(max_binder_depth_impl(a, depth)),
        Expr::Proj(_, _, e) => max_binder_depth_impl(e, depth),
        _ => depth,
    }
}

#[cfg(test)]
mod kernel_extra_tests {
    use super::*;

    fn nat() -> Expr {
        Expr::Const(Name::str("Nat"), vec![])
    }
    fn prop() -> Expr {
        Expr::Sort(Level::zero())
    }

    #[test]
    fn test_is_literal_true() {
        assert!(is_literal(&Expr::Lit(Literal::Nat(42))));
    }

    #[test]
    fn test_is_literal_false() {
        assert!(!is_literal(&nat()));
    }

    #[test]
    fn test_is_sort() {
        assert!(is_sort(&prop()));
        assert!(!is_sort(&nat()));
    }

    #[test]
    fn test_is_pi() {
        let p = mk_pi(Name::str("x"), prop(), prop());
        assert!(is_pi(&p));
        assert!(!is_pi(&nat()));
    }

    #[test]
    fn test_is_lam() {
        let l = mk_lam(Name::str("x"), prop(), Expr::BVar(0));
        assert!(is_lam(&l));
        assert!(!is_lam(&nat()));
    }

    #[test]
    fn test_is_app() {
        let e = mk_app(nat(), nat());
        assert!(is_app(&e));
        assert!(!is_app(&nat()));
    }

    #[test]
    fn test_is_const() {
        assert!(is_const(&nat()));
        assert!(!is_const(&Expr::BVar(0)));
    }

    #[test]
    fn test_const_name() {
        assert_eq!(const_name(&nat()), Some(&Name::str("Nat")));
        assert!(const_name(&Expr::BVar(0)).is_none());
    }

    #[test]
    fn test_strip_pi_binders_none() {
        let nat_expr = nat();
        let (binders, inner) = strip_pi_binders(&nat_expr);
        assert!(binders.is_empty());
        assert_eq!(*inner, nat());
    }

    #[test]
    fn test_strip_pi_binders_one() {
        let p = mk_pi(Name::str("x"), prop(), prop());
        let (binders, _inner) = strip_pi_binders(&p);
        assert_eq!(binders.len(), 1);
    }

    #[test]
    fn test_strip_lam_binders_one() {
        let l = mk_lam(Name::str("x"), prop(), Expr::BVar(0));
        let (binders, _body) = strip_lam_binders(&l);
        assert_eq!(binders.len(), 1);
    }

    #[test]
    fn test_build_pi_from_binders() {
        let binders = vec![(BinderInfo::Default, Name::str("x"), prop())];
        let ty = build_pi_from_binders(&binders, prop());
        assert!(is_pi(&ty));
    }

    #[test]
    fn test_build_lam_from_binders() {
        let binders = vec![(BinderInfo::Default, Name::str("x"), prop())];
        let l = build_lam_from_binders(&binders, Expr::BVar(0));
        assert!(is_lam(&l));
    }

    #[test]
    fn test_replace_const() {
        let e = nat();
        let result = replace_const(&e, &Name::str("Nat"), &prop());
        assert_eq!(result, prop());
    }

    #[test]
    fn test_replace_const_in_app() {
        let e = mk_app(nat(), nat());
        let result = replace_const(&e, &Name::str("Nat"), &prop());
        if let Expr::App(f, a) = &result {
            assert_eq!(**f, prop());
            assert_eq!(**a, prop());
        }
    }

    #[test]
    fn test_count_apps_zero() {
        assert_eq!(count_apps(&nat()), 0);
    }

    #[test]
    fn test_count_apps_one() {
        let e = mk_app(nat(), nat());
        assert_eq!(count_apps(&e), 1);
    }

    #[test]
    fn test_count_sorts_one() {
        assert_eq!(count_sorts(&prop()), 1);
    }

    #[test]
    fn test_count_sorts_zero() {
        assert_eq!(count_sorts(&nat()), 0);
    }

    #[test]
    fn test_contains_bvar_true() {
        assert!(contains_bvar(&Expr::BVar(0), 0, 0));
    }

    #[test]
    fn test_contains_bvar_false() {
        assert!(!contains_bvar(&Expr::BVar(1), 0, 0));
    }

    #[test]
    fn test_syntactically_equal() {
        assert!(syntactically_equal(&nat(), &nat()));
        assert!(!syntactically_equal(&nat(), &prop()));
    }

    #[test]
    fn test_collect_literals() {
        let e = mk_app(Expr::Lit(Literal::Nat(1)), Expr::Lit(Literal::Nat(2)));
        let lits = collect_literals(&e);
        assert_eq!(lits.len(), 2);
    }

    #[test]
    fn test_max_binder_depth_zero() {
        assert_eq!(max_binder_depth(&nat()), 0);
    }

    #[test]
    fn test_max_binder_depth_one() {
        let l = mk_lam(Name::str("x"), prop(), Expr::BVar(0));
        assert_eq!(max_binder_depth(&l), 1);
    }

    #[test]
    fn test_is_eta_reducible_false() {
        assert!(!is_eta_reducible(&nat()));
        // λ x. f x  where x = BVar(0)
        let not_eta = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(prop()),
            Box::new(Expr::App(
                Box::new(Expr::BVar(0)), // f = BVar(0) (x itself, not a function)
                Box::new(Expr::BVar(0)),
            )),
        );
        assert!(!is_eta_reducible(&not_eta));
    }

    #[test]
    fn test_has_let_binders_false() {
        assert!(!has_let_binders(&nat()));
        assert!(!has_let_binders(&mk_pi(Name::str("x"), prop(), prop())));
    }

    #[test]
    fn test_has_projections_false() {
        assert!(!has_projections(&nat()));
        assert!(!has_projections(&Expr::BVar(0)));
    }
}

// ─── Padding infrastructure ──────────────────────────────────────────────────

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

impl Default for StatSummary {
    fn default() -> Self {
        Self::new()
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

impl Default for TransformStat {
    fn default() -> Self {
        Self::new()
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

impl<K: Ord + Clone, V: Clone> Default for SmallMap<K, V> {
    fn default() -> Self {
        Self::new()
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

impl Default for LabelSet {
    fn default() -> Self {
        Self::new()
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

impl<T> Default for FocusStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests_padding_infra {
    use super::*;

    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }

    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }

    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        // Keys should be sorted
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }

    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo"); // duplicate
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }

    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }

    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }

    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}

/// A window iterator that yields overlapping windows of size `n`.
#[allow(dead_code)]
pub struct WindowIterator<'a, T> {
    data: &'a [T],
    pos: usize,
    window: usize,
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

impl<'a, T> Iterator for WindowIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos + self.window > self.data.len() {
            return None;
        }
        let slice = &self.data[self.pos..self.pos + self.window];
        self.pos += 1;
        Some(slice)
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

#[cfg(test)]
mod tests_extra_iterators {
    use super::*;

    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }

    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}

// ─── Second padding block ─────────────────────────────────────────────────────

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

impl Default for PathBuf {
    fn default() -> Self {
        Self::new()
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

impl Default for StringPool {
    fn default() -> Self {
        Self::new()
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

impl Default for RewriteRuleSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests_padding2 {
    use super::*;

    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0); // slides out 1.0
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }

    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }

    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty()); // cleared on give
        assert_eq!(pool.free_count(), 0);
    }

    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }

    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60)); // over remaining
        assert_eq!(tb.capacity(), 100);
    }

    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}

// ─── Third padding block ─────────────────────────────────────────────────────

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

impl Default for FlatSubstitution {
    fn default() -> Self {
        Self::new()
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

impl<T: Copy> Default for WriteOnce<T> {
    fn default() -> Self {
        Self::new()
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

impl Default for StackCalc {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests_padding3 {
    use super::*;

    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }

    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }

    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }

    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));

        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }

    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99)); // already written
        assert_eq!(wo.read(), Some(42));
    }

    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0); // default
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0); // reset to default
        assert_eq!(sv.nnz(), 1);
    }

    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
