# oxilean-kernel — TODO

> Task list for the kernel crate (Trusted Computing Base).
> Last updated: 2026-05-03

## ✅ Completed (Phase 0): Core Types

- [x] `Arena<T>` with `Idx<T>` — typed arena allocator
- [x] `Name` enum — hierarchical names with `name!` macro
- [x] `Level` enum — universe levels (Zero, Succ, Max, IMax, Param)
- [x] `Expr` enum — 11 expression variants
- [x] `BinderInfo`, `Literal`, `FVarId` types
- [x] `Display` for all core types
- [x] Unit tests for arena, name, level, expr
- [x] `#![forbid(unsafe_code)]` enforced
- [x] Zero external dependencies

---

## ✅ Completed (Phase 1): Type Checker

### Substitution Engine (`subst.rs` + `abstract.rs` + `expr_util.rs`)
- [x] `instantiate(body, arg)` — replace `BVar(0)` with `arg`, shift rest
- [x] `instantiate_rev(body, args)` — bulk instantiation
- [x] `abstract_expr(body, fvar)` — replace `FVar(fvar)` with `BVar(0)`
- [x] `lift_bvars(e, offset, shift)` — shift de Bruijn indices
- [x] `has_free_var(e, fvar)` — occurrence check
- [x] `subst_levels(e, param_map)` — universe parameter substitution
- [x] Round-trip test: `abstract(instantiate(body, fvar), fvar) == body`

### Level Operations (`level.rs` — 783 lines)
- [x] `normalize(l)` — canonical form (flatten + sort + merge)
- [x] `level_leq(u, v)` — `u ≤ v` semantic comparison
- [x] `level_eq(u, v)` — bidirectional `leq`
- [x] `substitute_level_params(l, params)` — replace `Param(n)` with concrete levels
- [x] `imax_simplify(u, v)` — IMax simplification rules

### WHNF Reduction (`whnf.rs` — 924 lines)
- [x] β-reduction: `(λ x, body) arg → instantiate(body, arg)`
- [x] δ-reduction: unfold definitions from environment
- [x] ζ-reduction: `let x := v in body → instantiate(body, v)`
- [x] ι-reduction: recursor application (`reduce_recursor`)
- [x] Projection reduction (`reduce_proj`)
- [x] Quotient reduction (`reduce_quot`)
- [x] WHNF cache (`HashMap<Idx<Expr>, Idx<Expr>>`)
- [x] Nested application reduction
- [x] Already-WHNF detection (early return)
- [x] Nat literal operations (succ/add/mul/sub/div/mod/pow/beq/ble/blt/gcd/land/lor/xor/shift)
- [x] String literal operations (length/append/beq)

### Type Inference (`infer.rs` — 563 lines)
- [x] `TypeChecker` struct with `env`, `local_ctx`, Reducer, DefEqChecker, check_mode
- [x] Inference for: `Sort`, `BVar` (→ error), `FVar`, `Const` (with universe params)
- [x] Inference for: `App` (with domain checking via `is_def_eq`)
- [x] Inference for: `Lam`, `Pi`
- [x] Inference for: `Let`, `Lit`
- [x] `ensure_sort(e)` — WHNF + verify is Sort
- [x] `ensure_pi(e)` — WHNF + verify is Pi
- [x] `infer_proj(e)` — telescopes through InductiveVal constructor to find field type

### Definitional Equality (`def_eq.rs` — 428 lines)
- [x] Pointer/index equality fast path
- [x] Structural comparison after WHNF
- [x] `App(f₁, a₁) ≡ App(f₂, a₂)` congruence
- [x] `Lam`/`Pi` equality with fresh FVar binder opening
- [x] η-expansion: `f ≡ λx. f x` when `x ∉ FV(f)`
- [x] Proof irrelevance — `is_proof_irrelevant_eq` infers types and checks Sort 0
- [x] Lazy delta reduction (unfold by reducibility height)
- [x] Equiv manager integration (Union-Find + failure cache)

### Declaration Checking (`check.rs`)
- [x] `check_and_add(env, decl)` entry point
- [x] Axiom: verify type is well-formed (type infers to a Sort)
- [x] Definition: verify type well-formed + value has declared type
- [x] Theorem: same as Definition (but opaque)
- [x] Opaque declarations
- [x] `check_inductive_val`, `check_constructor_val`, `check_recursor_val`, `check_quot_val`

### Environment (`env.rs` — 512 lines)
- [x] `Environment` struct with dual store (HashMap + Vec)
- [x] `Declaration` enum (Axiom, Definition, Theorem, Opaque) + accessors
- [x] `ConstantInfo` enum (8 variants) integration
- [x] `add()` and `get()` operations
- [x] `ReducibilityHints` (opaque, abbrev, regular with height)
- [x] `get_recursor()`, `get_inductive()`, `get_constructor()`

---

## ✅ Phase 1b: Inductive Types (COMPLETE)

### Inductive Validation (`inductive.rs` — 582 lines)
- [x] Verify inductive type's own type
- [x] Check constructors return the inductive type
- [x] Strict positivity check
- [x] Parameter handling (shared constructor prefix)
- [x] Universe constraint checking (large elimination)
- [x] Register InductiveVal + ConstructorVals + RecursorVal to environment
- [x] Empty type support (0 constructors — e.g. `Empty`)

### Recursor Generation
- [x] Generate `T.rec` type (motive + minors + major → motive applied)
- [x] Recursor computation rules — `build_recursor_rhs` builds minor-premise application
- [x] Handle recursive constructor arguments detection

### ι-Reduction (in `whnf.rs`)
- [x] Detect `rec ... (ctor args)` pattern
- [x] Apply minor premise with fields and recursive results
- [x] Projection reduction: `Proj(S, i, ctor(a₁...aₙ)) → aᵢ`

### Quotient Types (`quot.rs`)
- [x] 3 built-in declarations: `Quot.mk`, `Quot.lift`, `Quot.sound`
- [x] Quot reduction: `Quot.lift f h (Quot.mk a) → f a`
- [x] `is_quot_type_expr` — checks for `Quot` applied to relation
- [x] `check_quot_usage` — validates Lift/Ind motive sort

### Bootstrap (`builtin.rs` — 866 lines)
- [x] `Bool` (inductive: `true`, `false` + recursor)
- [x] `Unit` (inductive)
- [x] `Empty` (inductive)
- [x] `Nat` (inductive: `zero`, `succ` + recursor)
- [x] `String` (built-in type)
- [x] Nat arithmetic (add/sub/mul/div/mod/pow/gcd)
- [x] Nat comparison (beq/ble/blt)
- [x] Nat bitwise (land/lor/xor/shiftLeft/shiftRight)
- [x] Axioms: `propext`, `Classical.choice`, `DecidableEq`
- [x] `Eq` (inductive: `refl`)
- [x] `Prod` (inductive: `mk`)
- [x] `List` (inductive: `nil`, `cons`)

---

## ✅ Additional Features (beyond original TODO)

### Alpha Equality (`alpha.rs` — 280 lines)
- [x] Context-tracking full alpha equality + canonical renaming

### Pattern Matching (`pattern.rs` — 713 lines)
- [x] Pattern types (Wildcard/Var/Constructor/Literal/As/Or/Inaccessible)
- [x] Decision tree compilation, exhaustiveness checking, redundancy detection

### Termination Checking (`termination.rs` — 485 lines)
- [x] Structural recursion checker, mutual recursion, transitive subterm relation

### Cache System (`cache.rs` — 972 lines)
- [x] Full LRU cache, def-eq result cache, unified cache management

### Congruence Closure (`congruence.rs` — 416 lines)
- [x] Union-Find with path compression + congruence propagation

### Local Context (`context.rs` — 515 lines)
- [x] `LocalContext` with push/pop, lookup, scope management (RAII)

### Pretty Printer (`pretty.rs` — 622 lines)
- [x] Full expression pretty printing, Unicode & ASCII modes

### FFI (`ffi.rs` — 921 lines)
- [x] FFI declarations (16 variants), built-in I/O, String, Arithmetic functions

### Export (`export.rs` — 429 lines)
- [x] Export format serialization/deserialization, dependency resolution

### Universe Constraints (`universe_constraint.rs` — 363 lines)
- [x] Constraint types (Lt/Le/Eq), first-order solver

### Expression Utilities (`expr_util.rs` — 688 lines)
- [x] App decomposition, variable checks, traversal, shift, analysis, construction

### Equivalence Manager (`equiv_manager.rs` — 280 lines)
- [x] Union-Find for def-eq result caching (path halving + rank union)

### Eta Expansion (`eta.rs` — 221 lines)
- [x] Full eta expansion and contraction with fuel limit

### Conversion (`conversion.rs` — 300 lines)
- [x] Reduction strategies (WHNF/NF/OneStep/CBV/CBN)
- [x] NF/CBV/CBN/OneStep reduction strategies — all implemented in `conversion.rs`

---

## 🐛 Known Bugs & Issues

None. All previously tracked issues have been resolved as of 2026-03-09.

---

## ✅ Completed: Serialization

- [x] `.oleanc` binary serialization — `serial.rs` (OleanWriter, OleanReader, 8 tests)

## ⚪ Future Optimizations

- [x] Migrate `Expr` from `Box` to arena-based `Idx<Expr>` (hash-consing)
- [x] Expression caching (hash → index dedup)
- [x] η-expansion for structures
- [x] K-like reduction for singleton types
- [x] `no_std` compatibility for WASM
