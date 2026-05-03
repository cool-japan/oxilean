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
/// Structural sharing (hash-consing) for `Expr` values.
pub mod hash_cons;
/// No-std compatibility layer for constrained and WASM environments.
pub mod no_std_compat;
/// Thread-safe string interning pool for `Name` construction.
pub mod string_intern;

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

pub mod core_types;
pub use core_types::*;

/// Proof Certificate System — compact, verifiable records of kernel-checked proofs.
pub mod proof_cert;
pub use proof_cert::{
    create_certificate, deserialize_cert, hash_declaration, hash_expr, serialize_cert,
    verify_certificate, CertCheckResult, CertificateStore, ProofCertId, ProofCertificate,
    ProofStep,
};

/// Definitional Equality Cache — memoised results for `is_def_eq` queries.
pub mod def_eq_cache;
pub use def_eq_cache::{
    with_cache, CacheEviction, DefEqCache, DefEqCacheStats, DefEqEntry, DefEqKey,
};

/// Indexed environment for fast declaration lookup.
pub mod env_index;
pub use env_index::{
    is_in_namespace, namespace_of, EnvIndex, IndexStats, LookupResult, ModuleIndex, NameIndex,
    TypeIndex,
};

/// Memoized WHNF reduction with environment-version-aware cache invalidation.
pub mod whnf_memo;
pub use whnf_memo::{hash_bytes, with_memo, MemoConfig, MemoStats, WhnfEntry, WhnfKey, WhnfMemo};
