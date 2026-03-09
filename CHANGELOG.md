# Changelog

All notable changes to OxiLean will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Copyright (c) COOLJAPAN OU (Team Kitasan)

---

## [Unreleased]

### Planned
- Interactive proof mode improvements
- Language server protocol (LSP) support
- Package manager integration
- Extended standard library coverage

---

## [0.1.1] — 2026-03-09

Mathlib4 compatibility leap: from 4,530 to 181,890 declarations tested, achieving 99.7% parse compatibility across the entire Mathlib4 codebase.

### Added

#### Mathlib4 Compatibility Test Suite
- Expanded from 566 files / 47 categories to **7,759 files / 280+ categories**
- Expanded from 4,530 declarations to **181,890 declarations** (99.7% parse rate)
- Multi-line declaration extraction: joins indented continuation lines, strips line comments
- Summary test scanning 31 top-level recursive directories + Archive + Counterexamples
- Diagnostic test infrastructure for categorizing remaining parse failures
- 769 tests total (19 basic + 750 category/summary tests), zero warnings

#### Normalization Pipeline (normalize.rs, ~6,000 lines)
- 280+ Unicode operator replacements (category theory, analysis, algebra, set theory, etc.)
- `normalize_head_binders`: moves theorem head binders into `forall` type
- `normalize_exists_quantifier`: desugars multi-binder `exists` into nested `Exists(fun ...)`
- `normalize_psigma_binder`: wraps `PSigma x : T, body` into `PSigma (fun (x : T) -> body)`
- `normalize_bounded_quantifiers`: `ISup k < n, body` into `ISup (fun k -> body)`
- `normalize_big_prod_sum`: `BigProd` / `BigSum` with set membership
- `normalize_set_literals`, `normalize_set_builder_notation`, `normalize_singleton_sets`
- `normalize_subtype_sets`: `{ x : T // P }` into `Subtype T (fun x -> P)`
- `normalize_exists_unique`: `exists! x, P` into `ExistsUnique (fun x -> P)`
- `normalize_if_then_else_in_type`, `normalize_match_in_type`
- `normalize_dot_anonymous_fn`: `(. < .)` into `(fun x y -> x < y)`
- `normalize_star_type_suffix`: `beta*` into `beta_Star`
- `normalize_finsum_finprod`: handles `finprod`/`finsum` with `U+1DA0` marker
- `strip_universe_annotations`, `strip_attributes`, `strip_where_block`
- `replace_proof_with_sorry`: `by <tactics>` into `sorry`
- `parenthesize_dot_exprs`: `ident.field` into `(ident.field)`
- `parenthesize_bare_forall_binders`: `forall h:` into `forall (h:)`
- `strip_quantifier_binder_groups`, `strip_prop_condition_binders`
- `fix_trailing_operator_before_sorry`, `fix_forall_no_body_before_assign`
- `strip_orphan_close_brackets`, `strip_orphan_close_parens`
- `balance_parens_before_sorry`, `fix_truncated_decl`
- `^*` (fixed-points/pullback/dual) normalization

### Changed
- Test file structure reorganized: `normalize.rs`, `normalize_2.rs`, `normalize_3.rs`, `test_infra.rs`, `tests_basic.rs`, `tests_categories.rs`, `tests_summary.rs`, `types.rs`

---

## [0.1.0] — 2026-03-05

First release of OxiLean: a Lean4-inspired proof assistant kernel and toolchain
implemented in pure Rust. 1,221,710 SLOC across 11 crates and 5,380 files.

### Added

#### `oxilean-kernel` (113,179 SLOC)
- `Arena<T>` typed arena allocator with `Idx<T>` indexing
- `Name` hierarchical names (`Anonymous`, `Str`, `Num`) with `name!` macro
- `Level` universe levels (`Zero`, `Succ`, `Max`, `IMax`, `Param`)
- `Expr` core expression type with all variants (`BVar`, `FVar`, `Sort`, `Const`, `App`, `Lam`, `Pi`, `Let`, `Lit`, `Proj`)
- `BinderInfo` binder annotations (Default, Implicit, StrictImplicit, InstImplicit)
- `Literal` native literals (Nat, String)
- `FVarId` unique free variable identifiers
- Substitution engine: `instantiate`, `abstract`, `lift_bvars`
- WHNF reduction with full strategy support: beta, delta, zeta, iota, projection, and quotient reduction
- Type inference for all `Expr` variants
- Definitional equality checker with proof irrelevance
- Declaration checking for Axiom, Definition, Theorem, and Opaque declarations
- Inductive type declarations with strict positivity checking
- Recursor generation and iota-reduction rules
- Quotient types: `Quot.mk`, `Quot.lift`, `Quot.sound`
- Bootstrap types: Bool, Unit, Empty, Nat, String
- Zero external dependencies enforced
- `#![forbid(unsafe_code)]` in kernel

#### `oxilean-meta` (150,298 SLOC)
- Metavar-aware weak head normal form (WHNF) computation
- Higher-order unification engine
- Type class synthesis and instance resolution
- Tactic infrastructure and metaprogramming framework
- AST manipulation and transformation utilities

#### `oxilean-parse` (61,225 SLOC)
- UTF-8 lexer with full Unicode identifier support
- 60+ token types with precise source spans
- Pratt parser for operator precedence handling
- 27 `SurfaceExpr` variants covering the full surface syntax
- 16 `Command` variants for top-level declarations and directives
- Tactic parser supporting 40+ tactic forms
- Macro system with hygienic expansion
- Notation system for user-defined syntax extensions
- Module system with dependency graph resolution
- Pattern compiler for match expressions
- Pretty printer with configurable formatting
- Source map for diagnostic reporting
- Error recovery for resilient parsing

#### `oxilean-elab` (91,008 SLOC)
- `MetaContext` with metavar creation, assignment, and zonking
- Constraint-based unification solver
- Full expression elaboration: name resolution, implicit argument insertion, universe polymorphism
- Pattern match compilation with exhaustiveness and redundancy checking
- Declaration elaboration for def, theorem, inductive, structure, class, and instance
- Attribute system for declaration metadata
- Coercion system for automatic type conversions
- Derive system for automatic instance generation
- Parallel elaboration for independent declarations
- Termination checking for recursive definitions
- Tactic framework with core tactics: intro, apply, exact, simp, omega, ring, cases, induction, constructor, rewrite, have, let, suffices, show, assumption, contradiction, exfalso, trivial, decide, norm_num, linarith, field_simp, ring_nf, push_neg, by_contra, by_cases, ext, funext, congr, calc, rfl, symm, trans, and more

#### `oxilean-cli` (64,163 SLOC)
- Interactive REPL with line editing and history
- Multi-line input support with continuation detection
- Colorized output for types, terms, errors, and diagnostics
- File checking mode for `.oxilean` and `.lean` files
- `#check` command for type inference display
- `#eval` command for expression evaluation
- `#print` command for definition inspection

#### `oxilean-std` (413,202 SLOC)
- Core data structures: Nat, Bool, List, Option, Result
- Mathematical definitions and proof library
- Algebraic hierarchy: Semigroup, Monoid, Group, Ring, Field
- Type classes: Eq, Ord, Functor, Monad, Applicative, Decidable
- Decision procedures and certified algorithms

#### `oxilean-codegen` (240,840 SLOC)
- LCNF (lambda-lifted closure-free normal form) intermediate representation
- LCNF-based compilation pipeline with optimization passes
- Rust code generation backend

#### `oxilean-runtime` (31,115 SLOC)
- Runtime memory management and object representation
- Closure allocation and application
- I/O monad implementation
- Task scheduling and concurrency primitives

#### `oxilean-build` (25,194 SLOC)
- Multi-file project compilation
- Dependency resolution and topological ordering
- Incremental build support

#### `oxilean-lint` (17,061 SLOC)
- Static analysis passes for common errors
- Style enforcement and naming conventions
- Best practice recommendations and suggestions

#### `oxilean-wasm` (381 SLOC)
- WebAssembly bindings via `wasm-bindgen`
- `check` function for type checking expressions
- `repl` function for interactive evaluation
- `completions` function for editor integration
- `hover` function for type-on-hover information
- `format` function for source code formatting

#### Project Infrastructure
- Cargo workspace with 11 crates
- License: Apache-2.0
- Pure Rust with zero C/Fortran dependencies
- `cargo clippy` clean with zero warnings

---

[Unreleased]: https://github.com/cool-japan/oxilean/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/cool-japan/oxilean/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/cool-japan/oxilean/releases/tag/v0.1.0
