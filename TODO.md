# OxiLean — TODO

> Master task list for the OxiLean project.
> Last updated: 2026-07-17
>
> **Note**: Phases 1-4 are COMPLETE. v0.1.2 released 2026-05-03. v0.1.3 released 2026-07-16 (see `TODO_VERIFY.md`). The project has 17 crates and 1.35M+ lines implemented.
>
> **Phases 1-5 Status**: The interactive-prover phases (1–5) are complete. The v0.1.3 campaign (Waves 1–4) delivered the independent verify product: oxilean-export, oxilean-verify, oxilean-verify-wasm, plus kernel soundness fixes (BigNat, quotients, struct eta, re-derived recursors, complete level defeq). See TODO_VERIFY.md for full status.

---

## ✅ Phase 1: Nano-Kernel — Type Checker (COMPLETE)

See `crates/oxilean-kernel/TODO.md` for detailed status (~113,158 lines implemented).

### Substitution Engine (`oxilean-kernel/src/subst.rs`)
- [x] `instantiate(body, arg)` — replace `BVar(0)` with `arg`, shift others down
- [x] `instantiate_rev(body, args)` — bulk instantiation for multiple binders
- [x] `abstract_expr(body, fvar)` — replace `FVar(fvar)` with `BVar(0)`, shift up
- [x] `lift_bvars(e, offset, shift)` — add `shift` to `BVar(i)` where `i >= offset`
- [x] `has_free_var(e, fvar)` — check if expression contains a free variable
- [x] `subst_levels(e, param_map)` — substitute universe parameters

### Level Operations (`oxilean-kernel/src/level.rs` — 783 lines)
- [x] `normalize(l)` — canonical form for universe levels
- [x] `level_leq(u, v)` — universe level comparison (`u ≤ v`)
- [x] `level_eq(u, v)` — bidirectional `leq`
- [x] `substitute_level_params(l, params)` — replace `Param(n)` with concrete levels
- [x] `imax_simplify(u, v)` — simplify `IMax` expressions

### WHNF Reduction (`oxilean-kernel/src/whnf.rs` — 924 lines)
- [x] β-reduction: `(λ x, body) arg → body[arg/x]`
- [x] δ-reduction: unfold definitions
- [x] ζ-reduction: `let x := v in body → body[v/x]`
- [x] ι-reduction: recursor application
- [x] Projection and Quotient reduction
- [x] WHNF caching (`HashMap<Idx<Expr>, Idx<Expr>>`)
- [x] Nat and String literal operations

### Type Inference (`oxilean-kernel/src/infer.rs` — 563 lines)
- [x] `TypeChecker` struct with environment and local context
- [x] `infer_type` dispatch for all `Expr` variants
- [x] `ensure_sort`, `ensure_pi` helpers
- [x] `infer_proj` — telescopes through constructor Pi-type to find field type

### Definitional Equality (`oxilean-kernel/src/def_eq.rs`)
- [x] Pointer equality fast path
- [x] Structural comparison on WHNF
- [x] App-App congruence
- [x] Lam-Lam, Pi-Pi with fresh FVars
- [x] Function η-expansion (single-binder + multi-binder via `eta_expand_one`) (Wave 2, C2)
- [x] Structural η for structures: `try_eta_struct` both orientations + `is_def_eq_unit_like` (Wave 2, C1)
  Note: K-reduction is NOT implemented in the kernel (it is a Lean metaprogramming feature,
  not part of the kernel's definitional equality in Lean 4).
- [x] Proof irrelevance — `is_proof_irrelevant_eq` infers types and checks Sort 0
- [x] NatLit/StrLit ↔ constructor bridge in DefEqChecker (Wave 3b, C6)

### Declaration Checking (`oxilean-kernel/src/check.rs`)
- [x] `check_and_add` for `Axiom`, `Definition`, `Theorem`, `Opaque`
- [x] Environment management (`env.rs` — 512 lines)
- [x] `check_inductive_val`, `check_constructor_val`, `check_recursor_val`, `check_quot_val`

---

## ✅ Phase 1b: Inductive Types (COMPLETE)

See `crates/oxilean-kernel/TODO.md` for detailed status.

### Inductive Declaration (`oxilean-kernel/src/inductive.rs` — 582 lines)
- [x] Type validation
- [x] Constructor type checking
- [x] Strict positivity check
- [x] Parameter handling
- [x] Empty type support (0 constructors — e.g. `Empty`)

### Recursor Generation
- [x] Recursor type generation (`T.rec`) via `inductive/derive.rs` (Wave 2)
- [x] Recursor computation rules — minor premises with induction hypotheses (Wave 2, S5)
- [x] ι-reduction in WHNF — with re-application of over-args (`reduce/iota.rs`) (Wave 2, S2)
- [x] Re-derived recursors: kernel derives canonical RecursorVals from inductives/ctors and validates
  exported ones by def-eq comparison — tampered types/rules rejected (Wave 2, S3/S4)
- [x] Strict positivity re-check on import of inductive families (Wave 2, S3)
- [x] Mutual inductives and nested inductives supported via `add_inductive_family` (Wave 2)
- [x] Builtin inductives (Nat, Bool, Unit, Empty, String, Eq) re-derived; `Eq` is
  Lean-exact (2 params/1 index/K flag); rule RHSes are closed lambdas (Wave 2, S6)

### Projection Reduction
- [x] `Proj(name, idx, struct_val)` reduction

### Quotient Types (`oxilean-kernel/src/quotient/`)
- [x] 4 built-in declarations: `Quot`, `Quot.mk`, `Quot.lift`, `Quot.sound`, `Quot.ind`
  (Note: `Quot.ind` was always implemented but not listed here; root of `src/quot.rs` was
  refactored to `src/quotient/` in Wave 2. `quot.rs` as a single file does not exist.)
- [x] `Quot.lift f h (Quot.mk a) → f a` reduction rule (iota, re-applies over-args)
- [x] `Quot.ind h (Quot.mk r a) ≡ h a` reduction rule (Wave 2, fixes S2/S9)
- [x] `check_quotient_val` validates level params + def-eq vs kernel-built canonical types (Wave 2, S3)
- [x] Wrong builtin `Quot` axiom removed; `add_quot` installs four canonical QuotVals (Wave 2, S9)

### Bootstrap (`oxilean-kernel/src/builtin.rs` — 866 lines)
- [x] `Bool`, `Unit`, `Empty`, `Nat`, `String` (inductive types)
- [x] Nat arithmetic and comparison operations
- [x] Core axioms (propext, Classical.choice)

---

## ✅ Phase 2: Parser (COMPLETE)

See `crates/oxilean-parse/TODO.md` for detailed status (~61,203 lines implemented).

### Lexer (`oxilean-parse/src/lexer_impl.rs` — 1,363 lines)
- [x] UTF-8 identifier support (α, β, Π, λ, →, ⊢, subscripts)
- [x] Line comments `--` and nested block comments `/- ... -/`
- [x] Number literals (decimal, hex, binary, octal with separators)
- [x] Float literals
- [x] String literals with escape sequences and interpolation
- [x] Character literals
- [x] `Span` annotation for error reporting
- [x] 30+ unit tests

### Token System (`oxilean-parse/src/token_impl.rs` — 498 lines)
- [x] 60+ token variants (keywords, symbols, literals)
- [x] Operator precedence handling
- [x] `TokenInfo` struct with span and trivia

### AST (`oxilean-parse/src/ast_impl.rs` — 1,551 lines)
- [x] `SurfaceExpr` enum — 27 variants (Var, App, Lam, Pi, Arrow, Let, Match, ByTactic, Lit, Hole, Proj, If, Do, Have, Suffices, Show, etc.)
- [x] `Command` enum — 16 variants (Def, Theorem, Axiom, Inductive, Structure, Class, Instance, Import, Namespace, Section, Open, Universe, Variable, Attribute, HashCmd, SetOption)
- [x] `Binder`, `MatchArm`, `Tactic`, `Pattern`, `Constructor` types
- [x] All types with `Spanned<T>` wrapper
- [x] `Display` for all types

### Parser (`oxilean-parse/src/parser_impl.rs` — 3,641 lines)
- [x] Pratt parser for expressions (operator precedence climbing)
- [x] Declaration parsing (17 declaration kinds)
- [x] Binder parsing (explicit/implicit/strict-implicit/inst-implicit)
- [x] Pattern matching / `match` expressions
- [x] Tactic block parsing (`by`)
- [x] Error recovery (synchronize on `def`, `theorem`, etc.)

### Additional Modules (ALL COMPLETE)
- [x] Tactic Parser (`tactic_parser.rs` — 2,657 lines) — 40+ tactic variants
- [x] Command Parser (`command_parser.rs` — 2,608 lines)
- [x] Pattern Compiler (`pattern_compiler.rs` — 2,013 lines) — exhaustiveness & redundancy
- [x] Macro System (`macro_parser.rs` — 1,419 lines) — hygiene & expansion
- [x] Notation System (`notation.rs` — 1,295 lines)
- [x] Module System (`module.rs` — 2,068 lines) — dependency graph with cycle detection
- [x] Pretty Printer (`pretty_printer.rs` — 1,695 lines) — Unicode/ASCII modes
- [x] Source Map (`source_map.rs` — 1,081 lines) — LSP-compatible semantic tokens
- [x] REPL Parser (`repl_parser.rs` — 197 lines)
- [x] Error Handling (`error_impl.rs` — 1,044 lines) — Rustc-style diagnostics

---

## ✅ Phase 3: Elaborator (COMPLETE)

See `crates/oxilean-elab/TODO.md` for detailed status (~90,982 lines implemented).

### Meta-variables (`oxilean-elab/src/metavar.rs` — 166 lines)
- [x] `MetaContext` — creation, assignment, status checking
- [x] `zonk(expr)` — replaces all assigned metavariables recursively
- [x] Occurs check and scope management

### Unification (`oxilean-elab/src/unify.rs` — 148 lines + `solver.rs` — 181 lines)
- [x] Structural equality for all Expr variants
- [x] Metavar-aware unification with assignment propagation
- [x] Priority-based constraint scheduler (`PrioritySolver`) with retry
- [x] Constraint postponement queue

### Expression Elaboration (`oxilean-elab/src/elaborate.rs` — 2,237 lines)
- [x] `ElabContext` with env, local context, meta context
- [x] `elab_expr(surface_expr)` → `Result<Expr, ElabError>`
- [x] Name resolution: local → global → overload resolution
- [x] Application elaboration with implicit argument insertion
- [x] Lambda/Pi/Arrow elaboration
- [x] Let elaboration
- [x] Literal elaboration (Nat/String)
- [x] Hole `_` → create metavariable
- [x] Projection elaboration (`e.field`)
- [x] Match expression elaboration
- [x] `by` block → invoke tactic engine
- [x] If/then/else, Do-notation, Have/Suffices/Show expressions
- [x] Named arguments, Anonymous constructors, List literals, Tuples
- [x] String interpolation, Range expressions, Calc blocks
- [x] Type-directed elaboration with expected type propagation
- [x] Overload resolution

### Pattern Match Compilation (`oxilean-elab/src/pattern_match.rs` — 1,819 lines + `equation.rs` — 240 lines)
- [x] Surface patterns → decision tree
- [x] Exhaustiveness checking
- [x] Redundancy checking

### Declaration Elaboration (`oxilean-elab/src/elab_decl.rs` — 1,567 lines)
- [x] Definition, Theorem, Axiom elaboration
- [x] Inductive type elaboration
- [x] Universe parameter inference/checking
- [x] Mutual recursion support
- [x] Where clause elaboration
- [x] Opaque declarations
- [x] Structure/Class/Instance declarations
- [x] Namespace/Section/Variable/Open/Attribute/HashCmd
- [x] Attribute processing (simp/ext/instance/reducible/irreducible/inline etc.)

### Additional Features (ALL COMPLETE)
- [x] Attribute System (`attribute.rs` — 1,348 lines) — 10+ attribute kinds
- [x] Binder Elaboration (`binder.rs` — 1,167 lines)
- [x] Coercion System (`coercion.rs` — 965 lines) — registration & chaining
- [x] Derive System (`derive.rs` — 1,672 lines + `derive_adv.rs` — 2,543 lines) — 10+ derive handlers
- [x] Structure Elaboration (`structure.rs` — 2,186 lines) — inheritance & projections
- [x] Do-Notation Elaboration (in `elaborate.rs`)
- [x] Info Tree (`info_tree.rs` — 2,263 lines) — hover info, completions
- [x] Macro Expansion (`macro_expand.rs` — 1,361 lines) — 5 macro kinds
- [x] Notation System (`notation.rs` — 1,351 lines)
- [x] Parallel Elaboration (`parallel.rs` — 1,605 lines) — task scheduling
- [x] Error Messages (`error_msg.rs` — 877 lines) — 50+ error codes
- [x] Module Import (`module_import.rs` — 1,983 lines) — hierarchical namespaces
- [x] Command Elaboration (`command_elab.rs` — 1,850 lines)
- [x] Termination Checking (`mutual.rs` — 1,575 lines) — structural & well-founded recursion
- [x] Trace System (`trace.rs` — 1,041 lines)

---

## ✅ Phase 4: Tactics (COMPLETE)

See `crates/oxilean-elab/TODO.md` for detailed status.

### Tactic Infrastructure (`oxilean-elab/src/tactic.rs` — 1,604 lines)
- [x] `TacticState` struct (goals, solved)
- [x] `Goal` struct (mvar_id, hypotheses, local_ctx, target, tag)
- [x] Tactic combinator framework (sequence execution)
- [x] Goal focusing
- [x] `TacticRegistry` — registration, lookup, execution (18 tactics registered)
- [x] Undo/backtrack support (snapshot/restore)

### Core Tactics (IMPLEMENTED)
- [x] `intro` / `intros` — introduce Pi binder as hypothesis
- [x] `exact` / `assumption` — exact proof / search context
- [x] `apply` — apply lemma (simplified)
- [x] `rfl` / `trivial` — reflexivity & simple proofs
- [x] `constructor` — apply constructor (True, And patterns)
- [x] `left` / `right` — for disjunction (Or)
- [x] `exists` — provide witness for existential
- [x] `exfalso` — change goal to False
- [x] `clear` / `rename` / `revert` — hypothesis management
- [x] `have` / `suffices` — introduce intermediate goals
- [x] `sorry` — admit proof

### Additional Tactics (IMPLEMENTED)
- [x] `cases` — case split: And/Or/False/Nat/Exists (tactic.rs)
- [x] `induction` — Nat induction: zero + succ with IH (tactic.rs)
- [x] `rw` / `rewrite` — rewrite goal using equality proof; supports `←` reverse
- [x] `simp` / `simp only` — beta-reduce + built-in rules + rewrite chain
- [x] `push_neg`, `by_contra`, `contrapose`, `split`, `omega`, `ring`, `linarith`

---

## ✅ Phase 5+: Advanced Features (COMPLETE)

### ✅ Completed Additional Crates

**oxilean-meta** (~152,716 lines) — Metaprogramming infrastructure
- [x] Expression manipulation and analysis
- [x] Tactic metaprogramming support
- [x] AST manipulation utilities
- [x] SMT solver integration (OxiZ backends)
- [x] Property-based testing framework

**oxilean-std** (~416,133 lines) — Standard library
- [x] Core data structures (Nat, Bool, List, Option, Result, Array, HashMap)
- [x] Mathematical definitions (linear algebra, graph theory, number theory)
- [x] Proof library foundations (logic, equality, order, algebra)
- [x] Extended mathematical library: 86 modules covering algebraic geometry,
  cryptography, topology, differential geometry, quantum computing, and more

**oxilean-cli** (~64,848 lines) — Command-line interface
- [x] REPL implementation with line editing
- [x] Multi-line input detection
- [x] Goal display formatting
- [x] Interactive proof mode
- [x] `#check`, `#eval`, `#print` commands
- [x] Error reporting with source spans
- [x] Colorized terminal output

**oxilean-codegen** (~243,915 lines) — Code generation
- [x] Rust code generation backend
- [x] Expression compilation and declaration code generation
- [x] WASM, LLVM IR, JavaScript, C, GLSL, WGSL, Zig backends
- [x] Profile-guided optimization

**oxilean-build** (~26,070 lines) — Build system
- [x] Multi-file compilation and dependency resolution
- [x] Incremental compilation with content-based fingerprinting
- [x] Distributed builds and remote caching

**oxilean-runtime** (~31,676 lines) — Runtime system
- [x] Runtime primitives and memory management
- [x] Reference-counted closures, lazy thunks, tail-call optimization
- [x] Work-stealing parallel task scheduler
- [x] Pluggable GC strategies and WASM runtime integration

**oxilean-lint** (~17,600 lines) — Linting system
- [x] Code quality checks and style enforcement
- [x] 15+ built-in lint rules across 8 categories
- [x] Custom lint plugin system and auto-fix suggestions

**oxilean-wasm** (~510 lines) — WebAssembly bindings
- [x] WASM bindings for browser/web integration
- [x] Full API: check, repl, completions, hoverInfo, format

### ✅ Advanced Features (All Implemented)

- [x] Rich error messages with source spans
- [x] Multi-file import system
- [x] Standard library (Init, Data, Math)
- [x] WASM bindings (`oxilean-wasm` crate)
- [x] Code generation (Rust / WASM / LLVM / JS / C backends)
- [x] Parallel proof checking
- [x] OxiZ integration for SMT-backed tactics
- [x] Mathlib4 compatibility: 99.7% parse rate (181,326/181,890 declarations)

---

## 🏗️ Infrastructure & Tooling

- [x] CI/CD pipeline (GitHub Actions)
- [x] Benchmark suite for performance regression detection
- [x] Property-based testing (random well-typed terms)
- [x] Integration tests with `.oxilean` golden files
- [x] `rustdoc` documentation for all public APIs
- [x] Tutorial / getting-started guide

---

## 📊 Progress Tracker

| Phase | Status | SLOC Target | Current |
|-------|--------|-------------|---------|
| Phase 0: Skeleton | ✅ Complete | ~800 | ~779 |
| Phase 1: Nano-Kernel | ✅ Complete | ~5,000 | ~115,444 |
| Phase 1b: Inductives | ✅ Complete | ~2,000 | (included above) |
| Phase 2: Parser | ✅ Complete | ~3,000 | ~62,293 |
| Phase 3: Elaborator | ✅ Complete | ~15,000 | ~92,415 |
| Phase 4: Tactics | ✅ Complete | ~5,000 | (included in elab) |
| Phase 5+: Advanced | ✅ Complete | ~120,000+ | ~956,000+ |

**Total Project Lines**: ~1,347,650 lines across 17 crates, 5,978 files

---

## Project Status: Phases 1-5 COMPLETE; v0.1.3 verify product shipped

**Phases 1-5 complete as of 2026-05-03. v0.1.3 shipped 2026-07-16.**
- 17 crates, 5,978+ files, 1,347,650+ lines implemented
- 33,238 tests passing (workspace)
- 0 warnings
- Mathlib4 compatibility: 99.7% parse rate (181,326/181,890 declarations)
- 320 curated theorem proofs: 100% pass rate
- oxilean-verify: independent Lean 4 proof checker, three-bucket verdicts (verified/unsupported/rejected)
- oxilean-verify-wasm: 144 KB gzip (147,132 B at 0.1.3), 0 external deps, 0 unsafe, 39 exports

## Verify Product (0.1.3)

See `TODO_VERIFY.md` for the complete task list and status of the oxilean-verify campaign.

**Summary of what shipped in v0.1.3** (all items tracked in `TODO_VERIFY.md`):
- P0 soundness fixes: BigNat (S1), Quot.ind over-args (S2), re-derived recursors (S3-S6), level arity enforcement (S7), universe-param hygiene (S8), Quot canonical types (S9), Literal::Int removed (S10), proj-field type errors (S11)
- P0 completeness fixes: struct eta (C1/C2), imax(u,u) level defeq (C3), NatLit/StrLit bridge (C6)
- Products: oxilean-export (V1), oxilean-verify CLI (V2), oxilean-verify-wasm 144 KB (V3), web/verify-demo (V4)
- Gates: CI (G1), zero-dep (G2), allow-list (G3), forbid-unsafe (G4), wasm export-count (G5), size-budget (G6), native determinism (G7 partial), demo smoke (G8 partial)
- Deferred to Wave 5+: V5 corpus reproducibility, V6 differential harness, V7 throughput benchmark, G7 wasm-half, G8 browser-drive, G9 cargo-fuzz

---

## v0.1.3 Development (Branch: 0.1.3)

> Status: In Progress — Ring 0 = foundation MVPs, Ring 1+ = full depth
> Last updated: 2026-05-29

### Theme 1: Proof Automation Tactics

#### Ring 0

- [x] Implement `omega` tactic with Cooper's algorithm / Omega test for Presburger arithmetic
  - **Goal:** Replace stub `OmegaMetaTactic.run()` (which returns Solved on any linear-arithmetic-looking goal) with a genuine integer linear arithmetic decision procedure
  - **Design:** Parse linear constraints from goal + tactic context hypotheses; implement Omega test (dark/grey shadow elimination, exact case); integrate with `UserTactic` trait; succeed only when goal is provable; output readable failure reason on refutation. Use `decide_enhanced/functions.rs` as implementation pattern.
  - **Files:** `crates/oxilean-elab/src/metaprog/omegametatactic_traits.rs`, `crates/oxilean-elab/src/metaprog/types.rs`
  - **Prerequisites:** None — existing `UserTactic` trait + tactic state API
  - **Tests:** `1 + 1 = 2`, `n ≥ 0 → n + 1 > 0`, unsatisfiable system, mixed system with solution
  - **Risk:** Goal AST string representation may need richer parsing; study types.rs (905 lines) for context shape
  - **Implemented:** `src/metaprog/omega_engine.rs` (~1350 lines) + `omegametatactic_traits.rs` updated (2026-05-29)

- [x] Implement `linarith` tactic with Fourier-Motzkin over ordered fields
  - **Goal:** Replace stub `RingMetaTactic.run()` with Fourier-Motzkin elimination for linear real/rational arithmetic; succeed only when negation of goal is unsatisfiable from hypotheses
  - **Design:** Extract linear constraints from hypotheses + negated goal; apply FM variable elimination iteratively; detect empty feasible set = proof; support rational coefficients (implement minimal Rational = (i64,i64) fraction arithmetic inline); output FM derivation trace on success
  - **Files:** `crates/oxilean-elab/src/metaprog/ringmetatactic_traits.rs`, `crates/oxilean-elab/src/metaprog/types.rs`
  - **Prerequisites:** None
  - **Tests:** Prove `¬(x > 0 ∧ x < 0)`, transitivity of `<`, bound propagation, redundant constraint elimination
  - **Risk:** Rational coefficient arithmetic — avoid external crates, implement minimal Rational type inline to stay Pure Rust
  - **Implemented:** Fourier-Motzkin with inline `Rational=(i64,i64)` arithmetic in `ringmetatactic_traits.rs` (2026-05-29)

#### Ring 1

- [x] `nlinarith` — Farkas proof reconstruction (cycle 4) (2026-05-29)
  - **Goal:** Refactor `has_farkas_certificate` → `find_farkas_certificate` returning the Farkas multipliers+source map; build a real kernel-checked proof term from the certificate via `Int.mul_le_mul_of_nonneg_left`+`Int.add_le_add` sum then `absurd`/contradiction; kernel-gate (fallback to sorry on failure).
  - **Files:** `oxilean-meta/src/tactic/linear_combination/types/defs.rs`, `oxilean-elab/src/tactic/functions_3.rs`, `oxilean-elab/src/tactic/functions_2.rs`, `oxilean-elab/src/tactic/proof_recon/farkas.rs`
  - **Implemented (2026-05-29 A3):** Provenance plumbing (`parse_hyps_with_sources`, `ConSource` per constraint), `try_nlinarith_with_positivstellensatz` attaches sources via `FarkasCert::with_sources`, `farkas_cert_to_expr` signature updated to `(cert, goal, hyps, locals, env)` with FVar threading, proof builders (single/two/multi constraint) implement `Int.add_le_add`+`Int.le_trans`+`Int.absurd_le_zero` strategies with kernel gate, elaborate/functions.rs wired to call `farkas_cert_to_expr`. Tests: 8 passing, `None` returned safely when kernel can't verify.
  - **Tests:** `0 ≤ x²`; `a²+b² ≥ 2ab`; integer-multiplier refutation → verified term; edge case → graceful placeholder.
- [x] nlinarith Farkas: REAL verified production proofs — transitivity-cycle class (cycle 6) (2026-05-29)
  - **Goal:** Make `farkas_cert_to_expr` actually emit kernel-verified (non-`sorry`) proof terms in production, not just architecture. Confirmed gaps: (1) omega lemmas never registered in any production env (`oxilean-cli/commands/functions.rs:33` builds bare `Environment::new()`); (2) builders close with `Int.absurd_le_zero k Int.zero_lt_one h` which needs literal Int arithmetic the opaque kernel can't compute, and never handle strict `<`.
  - **Design:** Implement the tractable opaque-Int class = Farkas refutation with unit multipliers forming a transitivity cycle with ≥1 strict edge → fold via `Int.le_trans`/`Int.lt_of_le_of_lt`/`Int.lt_of_lt_of_le`/`Int.lt_trans` to `Int.lt a a` → refute via `Int.lt_irrefl a`. Add LE.le/LT.lt→Int.le/Int.lt bridge (`int_le_of_le`). Wire `oxilean_std::add_omega_lemmas` into the CLI/REPL production env. Kernel-gate everything (infer_type + is_def_eq); else `sorry`.
  - **Files:** `oxilean-std/src/omega_helper/mod.rs` (3 new lt lemmas → 17 axioms), `oxilean-elab/src/tactic/proof_recon/farkas.rs` (strict-aware cycle builder + bridge), `oxilean-elab/src/elaborate/functions.rs` (call site), `oxilean-cli/src/commands/functions.rs` + `oxilean-cli/src/repl/types.rs` (env wiring).
  - **Tests:** `a ≤ b, b ≤ c, c < a ⊢ False` → verified non-`sorry` term; `a < b, b < a ⊢ False` via lt_trans; LE.le/LT.lt-typed variant bridged; negative (no strict edge) → `sorry`; soundness invariant retained.
  - **Deferred:** general Farkas (non-unit multipliers / linear summation) needs computational Int in the kernel — out of scope, documented.
- [x] oxilean-kernel: Int literal arithmetic (`Literal::Int` + reduction arms) (planned 2026-05-30)
  - **Goal:** Enable ground Int arithmetic in the kernel reducer, mirroring the Nat literal fast-path. After this cycle `Int.ble 0 (-1)` reduces to `Bool.false` for the first time, enabling Bool-reflection discharge for general Farkas/linarith in cycle 8.
  - **Design:** Add `Literal::Int(i64)` to `Literal` enum in `expr/types.rs`; propagate through all match arms; add `try_reduce_int_app` in `reduce/functions.rs` with arms for `Int.ofNat`/`Int.negSucc` bridges, `Int.add`/`mul`/`sub`/`neg`, and `Int.ble`/`Int.blt`/`Int.beq` (Bool-valued); dispatch at `reduce/types.rs`. All 18 omega_helper axioms stay axioms.
  - **Files:** `crates/oxilean-kernel/src/expr/types.rs`, `crates/oxilean-kernel/src/reduce/functions.rs`, `crates/oxilean-kernel/src/reduce/types.rs`.
  - **Tests:** Int.ble 0 (-1) → Bool.false; Int.add 2 3 → 5; Int.sub 1 3 → -2; bridge Int.negSucc 2 → -3; non-literal args stay stuck.
- [x] `polyrith` — Gröbner basis over ℚ (Buchberger's algorithm) for polynomial ring equalities (2026-05-30)
- [x] `cc` — kernel-verified congruence closure proof reconstruction (cycle 10) (planned 2026-05-30)
  - **Goal:** Implement sound, kernel-verified proof reconstruction for the existing E-graph CC engine in `grind/`. The decision procedure already works; what's broken is that `cc_build_single_step_proof` emits type-incorrect `congrArg` applications (passing `f a` where type `α` is expected) and `cc_build_proof` drops all implicit args from `Eq.trans`, so the kernel gate rejects every non-trivial proof and falls back to `sorry`.
  - **Design:** (A) Add `ProofLabel` enum + `proof_parent: Vec<Option<(ENodeId, ProofLabel)>>` proof-forest to `CongruenceClosure`; reroot before each merge (NO algorithm). (B) New `cc_proof.rs`: `explain(a,b)→Vec<ExplainStep>` via proof-forest LCA traversal with recursive congruence argument explain; `build_eq_proof(steps, env, hyp_fvars)→Option<Expr>` that supplies every implicit arg via kernel `TypeChecker::infer_type`. (C) Rewrite `tac_cc` to call new explain+build; fall back to refl placeholder on build failure. (D) Wire `register_cc_helper` + `register_polyrith_helper` into CLI/REPL production envs. (E) E2e dispatcher tests assert non-`sorry` + kernel-verified for congruence goals.
  - **Files:** `grind/types.rs`, `grind/cc_proof.rs` (new), `grind/functions.rs`, `grind/mod.rs`, `grind/functions_2.rs` (if needed); `oxilean-cli/src/repl/types.rs`, `oxilean-cli/src/commands/functions.rs`; `oxilean-elab/src/tactic/dispatcher_tests.rs`.
  - **Tests:** 5+ kernel-verified cc proof tests in cc_proof.rs (single/multi/nested congruence, transitivity, symmetry); 4 e2e dispatcher tests asserting non-sorry.
  - **Risk:** Meta `infer_type` threading for implicit arg computation; proof-forest rerooting correctness (tests 2-5 discriminate this).
- [x] Stdlib lemma bundles supporting omega/linarith — LE.le bridge axioms (planned 2026-05-29)
  - **Goal:** Add `le_of_int_le`+`int_le_of_le` bridge axioms to omega_helper; wire `register_omega_helper` into the production env builder; ensure `absurd`/`False.elim` present in production env.
  - **Files:** `oxilean-std/src/omega_helper/mod.rs`, `oxilean-std/src/env_builder/*`
  - **Tests:** Bridge axioms present+well-formed; production env contains all 12 lemmas + `absurd`.
- [x] Bool-reflection Farkas discharge — arithmetic chain close (cycle 8) (2026-05-30)
  - **Goal:** Extend farkas.rs cycle builder to close chains `a ≤/< b` with ground-literal endpoints where `b < a`/`b ≤ a` via `Int.not_le_of_ble_false`/`Int.not_lt_of_blt_false`; add 2 new axioms to omega_helper (18→20). Unlocks `0 ≤ x, x ≤ -1 ⊢ False` with kernel-verified proof.
  - **Files:** `crates/oxilean-std/src/omega_helper/mod.rs`, `crates/oxilean-elab/src/tactic/proof_recon/farkas.rs`, `crates/oxilean-kernel/src/reduce/functions.rs` (Bool name consistency fix)
  - **Tests:** 3+ new tests in farkas.rs asserting Some + non-sorry + kernel-verifies; omega_helper axiom count 20.

### Theme 2: LSP Integration Tests

#### Ring 0

- [x] Add LSP server integration test suite (in-process JSON-RPC round-trip)
  - **Goal:** Verify existing 109-file LSP implementation runs end-to-end: initialize → didOpen → publishDiagnostics on fixture with type error
  - **Design:** In-process test using a channel-pair mock transport (pipe stdin/stdout via `std::sync::mpsc`); drive `initialize`/`initialized`/`textDocument/didOpen` sequence; capture `textDocument/publishDiagnostics` notification; assert expected diagnostic location + message; add `shutdown`/`exit` cleanup
  - **Files:** `crates/oxilean-cli/src/lsp/` (new `#[cfg(test)]` module or `tests/lsp_integration.rs`)
  - **Prerequisites:** None — LSP infrastructure complete
  - **Tests:** round-trip JSON-RPC; diagnostics on error fixture; clean shutdown
  - **Risk:** Stdio-based server needs refactoring to accept generic Read/Write; may need to add a `run_with_transport(r, w)` entry point

#### Ring 1

- [x] VS Code extension skeleton under `crates/oxilean-cli/src/lsp/editor/vscode/`
- [x] Incremental `didChange` sync (range-based partial parse) (planned 2026-05-29)
  - **Goal:** Wire `oxilean_parse::parse_incremental_change` into the LSP `didChange` handler so the lex step is incremental (splice tokens) instead of full re-tokenize.
  - **Design:** Cache prior `Vec<Token>` on `Document`; in `apply_incremental_change` convert LSP line/col range → char-index `TextChange`, call `parse_incremental_change(old_tokens, new_content, &change)`, splice. Honest note: `parse_file` stays O(n); full incremental parse depends on Myers AST diff (oxilean-parse).
  - **Files:** `oxilean-cli/src/lsp/server/types.rs`, `oxilean-cli/src/lsp/document/mod.rs`, `oxilean-cli/src/lsp/analysis/mod.rs`.
  - **Tests:** incremental re-lex == full re-lex for single-token/single-line/multi-line edits incl. UTF-16/CJK boundary; didChange round-trip preserves doc state.
- [x] Semantic highlighting token type specification (planned 2026-05-29)
  - **Goal:** Define the canonical `SemanticTokenType` legend (keyword, function, type, variable, parameter, number, string, comment, operator, namespace, …) + modifiers, document the index mapping, ensure `semanticTokens/full` and `/range` emit against this fixed legend.
  - **Files:** `oxilean-cli/src/lsp/semantic_tokens/`, `oxilean-cli/src/lsp/server/`.
  - **Tests:** legend stable + documented; full vs range round-trip against the legend.
- [x] `oxilean playground` subcommand — static-file server for the wasm playground (planned 2026-05-30)
  - **Goal:** Add a `playground` subcommand to oxilean-cli that serves `crates/oxilean-wasm/playground/dist/` over a minimal `std::net::TcpListener` static-file server (pure std, no external HTTP crate). Best-effort browser open. `--port` flag.
  - **Files:** `crates/oxilean-cli/src/main/functions.rs`, `crates/oxilean-cli/src/commands/playground.rs` (new).
  - **Tests:** MIME mapping; path-traversal rejection; request-line parsing; fixture file round-trip.

### Theme 3: Browser Playground

#### Ring 0

- [x] Create `crates/oxilean-wasm/playground/` static-site proof assistant playground
  - **Goal:** Browser-deliverable: CodeMirror 6 editor + live check via existing WASM API + result panel; build.sh produces deployable `dist/`
  - **Design:** `index.html` (minimal, standards-compliant); `main.js` (ES modules, CodeMirror 6 from pinned ESM CDN, debounced parse-on-type calling `oxilean_wasm.check()` from wasm_api.rs); `style.css` (clean two-panel layout); `build.sh` (runs wasm-pack for bundler target, copies .wasm + .js + HTML/CSS to `dist/`)
  - **Files:** `crates/oxilean-wasm/playground/index.html`, `main.js`, `style.css`, `build.sh`
  - **Prerequisites:** `crates/oxilean-wasm/src/wasm_api.rs` (already exists)
  - **Tests:** `bash playground/build.sh` exits 0 and `dist/index.html` + `dist/*.wasm` exist
  - **Risk:** wasm-pack output naming varies by version; pin paths to `pkg-bundler/` output

#### Ring 1

- [x] Persistent storage via IndexedDB (planned 2026-05-29)
  - **Goal:** Save/restore editor content via browser IndexedDB from the playground JS.
  - **Files:** `oxilean-wasm/playground/index.html` (or equivalent JS)
  - **Tests:** Editor content survives page reload (manual + automated JS test).
- [x] Share-via-URL (URL fragment = OxiARC-compressed base64 source) (planned 2026-05-29)
  - **Goal:** wasm-bindgen exports `compress_share`/`decompress_share` using OxiARC deflate; playground JS encodes to base64url URL fragment; decodes on load.
  - **Files:** `oxilean-wasm/src/lib.rs` (or playground module), `oxilean-wasm/playground/index.html`, `oxilean-wasm/Cargo.toml` (oxiarc-* dep)
  - **Tests:** Round-trip compress→base64url→decompress equals original.
- [x] Bundled example library (10–15 proofs shipped in-page) (planned 2026-05-29)
  - **Goal:** In-page dropdown of 10-15 curated proofs loadable into the editor.
  - **Files:** `oxilean-wasm/playground/index.html`
  - **Tests:** Dropdown populates; selecting an example loads its code.
- [x] Playground: multi-file tab UI + per-file IndexedDB (planned 2026-05-29)
  - **Goal:** Tab bar + file tree; Map<filename, content> in JS; per-file IndexedDB persist; CodeMirror 6 switches content on tab click; add/rename/close tabs.
  - **Files:** `crates/oxilean-wasm/playground/index.html`, `crates/oxilean-wasm/playground/main.js`, `crates/oxilean-wasm/playground/style.css`
  - **Tests:** Tab create/switch/close; per-file persistence survives reload.
  - **Risk:** JS only — no Rust changes.
- [ ] GitHub Pages deploy (requires separate user approval for new workflow yaml)
- [x] Performance: incremental WASM check (diff-based partial reparse) (planned 2026-05-30)
  - **Goal:** Replace the line-based heuristic in `oxilean-wasm/src/incremental/functions.rs` with `diff_modules`-driven AST diffing (Myers decl-diff from oxilean-parse) for cache-key stability and correct cache hit/miss accounting.
  - **Files:** `crates/oxilean-wasm/src/incremental/functions.rs`, `crates/oxilean-wasm/src/wasm_api.rs`.
  - **Tests:** full check == incremental for a 3-decl fixture; cache_hit_count / recheck_count correct on middle-edit and append-decl edits.

### Theme 4: Package Manager + Doc-Gen

#### Ring 0

- [x] Create `crates/oxilake/` package manager binary crate
  - **Goal:** New `oxilake` binary with `new`, `build`, `check` subcommands; parses `oxilake.toml` manifest
  - **Design:** `oxilake.toml` manifest via `toml` + serde (Package table: name/version/lean-version/description; Dependencies table); `oxilake new <name>` scaffolds `<name>/oxilake.toml` + `<name>/Main.lean`; `oxilake build` invokes oxilean-build executor; `oxilake check` = build without codegen emit; CLI via `clap`; register crate in root `Cargo.toml`
  - **Files:** `crates/oxilake/Cargo.toml`, `crates/oxilake/src/main.rs`, `crates/oxilake/src/manifest.rs`, `crates/oxilake/src/commands/mod.rs`, `crates/oxilake/src/commands/build.rs`, `crates/oxilake/src/commands/new.rs`, `crates/oxilake/src/commands/check.rs`
  - **Prerequisites:** oxilean-build `core_types` API; read `crates/oxilean-build/src/core_types/` first
  - **Tests:** fixture manifest parse; `oxilake new` creates expected dir structure (temp_dir); `oxilake build` on fixture exits 0
  - **Risk:** oxilean-build API is trait-heavy — start with minimal executor path

- [x] Create `crates/oxilean-doc/` documentation generator binary crate
  - **Goal:** New `oxilean-doc <file.lean> [-o out.html]` binary that extracts docstrings + signatures from parsed AST and renders single-page HTML
  - **Design:** Use `oxilean-parse` to parse input `.lean` → walk `SurfaceDecl` nodes → extract `doc_comment` field + name + type string → render as `<section>` HTML blocks via `write!`/`format!`; no template engine; output to file or stdout; `clap` CLI
  - **Files:** `crates/oxilean-doc/Cargo.toml`, `crates/oxilean-doc/src/main.rs`, `crates/oxilean-doc/src/extractor.rs`, `crates/oxilean-doc/src/renderer.rs`
  - **Prerequisites:** `oxilean-parse` public API (parser, ast, module)
  - **Tests:** golden HTML test against 3-declaration fixture `.lean` file with docstrings
  - **Risk:** `doc_comment` field name in `SurfaceDecl` — verify exact field name before writing extractor

#### Ring 1

- [x] oxilake: dependency resolution (semver, registry stub, local path deps) (done 2026-05-29)
  - **Goal:** Real resolver: minimal pure-Rust semver (Version/VersionReq), local `path =` deps with cycle detection + topological build order, registry stub trait (LocalDirRegistry impl; network = explicit Unsupported). Wire into `oxilake build` (deps built in topo order).
  - **Files:** `crates/oxilake/src/` (new resolver module + manifest dep table + commands/build.rs wiring).
  - **Tests:** semver parse/compare/match matrix; 3-package path-dep graph → correct topo order; cycle detected; missing dep errors cleanly (std::env::temp_dir()).
- [x] oxilake: `oxilake.lock` lockfile (oxicode-serialized) (planned 2026-05-29)
  - **Goal:** Write/read a lockfile of resolved deps + content hashes, serialized via oxicode (NOT bincode/serde_json). Wire `oxilake build` to actually call `oxilean_build::build_project`.
  - **Files:** `oxilake/src/*`
  - **Tests:** Build invokes executor; lockfile round-trips via oxicode (`std::env::temp_dir()`).
- [x] oxilake: `test`, `run`, `fmt` subcommands (planned 2026-05-29)
  - **Goal:** `oxilake test` builds via oxilean_build::build_project then reports per-declaration check status; `oxilake run` builds then executes Main; `oxilake fmt` canonically formats sources (minimal formatter if no existing pretty-printer).
  - **Files:** `crates/oxilake/src/main.rs`, `crates/oxilake/src/commands/test.rs`, `crates/oxilake/src/commands/run.rs`, `crates/oxilake/src/commands/fmt.rs`
  - **Tests:** test/run/fmt over a scaffolded fixture project (`std::env::temp_dir()`); fmt idempotence.
  - **Risk:** executor may lack run-tests path — derive status from build_project output. fmt fallback to minimal formatter.
- [x] oxilake: workspace (multi-package manifests) (done 2026-05-30)
- [x] oxilake: cache directory at `~/.oxilake/cache/` (done 2026-05-30)
- [x] oxilean-doc: multi-file crate-wide doc generation with cross-references (planned 2026-05-29)
  - **Goal:** Walk all modules, build symbol→page/anchor index, resolve intra-crate refs in signatures/doc-comments to relative links; emit one page per module + index page.
  - **Files:** `oxilean-doc/src/*`
  - **Tests:** 2-module fixture cross-links symbols; relative links are hosting-portable.
- [x] oxilean-doc: client-side search index (JS + JSON) + markdown rendering in doc comments (planned 2026-05-29)
  - **Goal:** Generate `search-index.json` ({name, kind, page, anchor, signature}) from SymbolIndex; embed vanilla-JS search box in multi-file output; replace escape_html(doc) with pure-Rust mini-markdown renderer (bold, italic, inline code, fenced code, links, paragraphs).
  - **Files:** `crates/oxilean-doc/src/symbol_index.rs`, `crates/oxilean-doc/src/multifile.rs`, `crates/oxilean-doc/src/markdown.rs` (new)
  - **Tests:** 2-module fixture search index valid JSON with all symbols; markdown unit tests per element.
  - **Risk:** Pure-Rust only — no new external deps.
- [x] oxilean-doc: theming + hosting-ready relative-link output (planned 2026-05-29)
  - **Goal:** Light/dark theming via CSS custom properties (`:root` + `[data-theme="dark"]`, `prefers-color-scheme` default, localStorage JS toggle) across `renderer.rs` STYLE and `multifile.rs` MULTI_STYLE; keep relative links hosting-portable.
  - **Files:** `crates/oxilean-doc/src/renderer.rs`, `crates/oxilean-doc/src/multifile.rs`.
  - **Tests:** output contains light+dark variables + toggle element; relative links resolve.

---

### Ring 1 (in progress — this cycle)

- [x] `cc` tactic → real kernel-checkable proof terms via grind CongruenceClosure (2026-05-29)
  - **Goal:** Wire `cc`/`congruence` to the real grind CongruenceClosure engine (which already has proof-producing CC: `merge_log`, `explain_equality`, `MergeReason`). Produce kernel-verified `Eq` proof terms.
  - **Design:** New `tac_cc(state, ctx)` in oxilean-meta/tactic/grind/. Uses `state.goal_view(ctx)`, `decompose_eq` on goal, `add_term`/`merge_with_reason` for each eq hyp, `are_equal`/`explain_equality` for proof chain, convert EqualityStep vec → kernel Expr via fixed `cc_build_proof` (supplying full implicit args to Eq.refl/Eq.trans/congrArg). TypeChecker::infer_type + is_def_eq gate. ProofCertificate::Direct(Expr) variant added to certificate.rs. Elab elaborate_by_tactic uses Direct variant directly. `"cc"|"congruence"` dispatcher arm bridges to tac_cc.
  - **Files:** meta: tactic/grind/functions.rs, tactic/certificate.rs, tactic/mod.rs; elab: tactic/functions_2.rs, elaborate/functions.rs
  - **Tests:** a=a; h:a=b ⊢ b=a; h1:a=b,h2:b=c ⊢ a=c; h:a=b ⊢ f a=f b; each kernel-verified; negative (unrelated ⊢ a=b → fail).
  - **Risk:** build_single_step_proof uses approximate term builders; fixing to supply implicit type args is the hard part. TypeChecker gate is the soundness firewall.

- [x] `nlinarith` tactic → real nonlinear arithmetic (Positivstellensatz-lite) (2026-05-29)
  - **Goal:** Split `nlinarith` out of the `"linarith"|"nlinarith"` shared arm into its own arm with Positivstellensatz-lite preprocessing (squares + pairwise products of hypothesis atoms), then run `has_farkas_certificate` on the augmented constraint set. Placeholder proof term (real decision procedure, honest caveat).
  - **Design:** Route A (elab-side): augment `all_cons` with squares t²≥0 and products tᵢ·tⱼ≥0 as fresh SymLinCon atoms, then call `has_farkas_certificate`. Split `"nlinarith"` into its own dispatcher arm before `"linarith"`.
  - **Files:** elab: tactic/functions_2.rs, tactic/functions_3.rs (Positivstellensatz helpers)
  - **Tests:** `a²+b² ≥ 2ab`-style goal; `0 ≤ x²`; goal linear linarith cannot close but nlinarith can; negative.
  - **Risk:** SymLinCon must support fresh atoms; product generation must not blow up constraint count.

- [x] omega proof reconstruction: le_trans + lt_irrefl patterns (2026-05-29)
  - **Goal:** Extend `omega_proof_to_expr` with le_trans (goal `Int.le a c`, search hyps for `Int.le a b`+`Int.le b c`) and lt_irrefl (goal `False` + hyp `Int.lt a a`). Fix doc comment: LE.le IS genuinely opaque, not just a doc issue.
  - **Files:** elab: tactic/proof_recon/mod.rs, elaborate/functions.rs (thread &hyps)
  - **Tests:** h1:a≤b,h2:b≤c ⊢ a≤c; h:a<a ⊢ False; both kernel-verified.
  - **Risk:** Hypothesis FVar access requires threading hyps into omega_proof_to_expr.
  - **Refinement (2026-05-29):** Cycle 4: FVar threading + LE.le bridge + production env wiring (A1+A2 tracks).
  - **Refinement (2026-05-29):** Cycle 4 complete — FVar threading + LE.le bridge + lt_irrefl via absurd implemented.

- [x] `tac_simp` + `simp` dispatcher wiring (2026-05-29)
  - **Goal:** simp arm (functions_2.rs:126) uses local `apply_simp_rules` only. Add `tac_simp(state, ctx)` in meta (calls real `simp` driver with `default_simp_lemmas()`) and wire it into the dispatcher BEFORE the elab-local fallback.
  - **Files:** meta: tactic/simp/main/functions.rs (add tac_simp), tactic/mod.rs; elab: tactic/functions_2.rs (try meta first in simp arm)
  - **Tests:** `by simp` closes Nat.add_zero + True + Bool simplifications via the meta engine.
  - **Risk:** SimpResult::Proved(proof) must produce a well-typed kernel proof; fall back to elab-local on failure.

- [x] omega tactic → real kernel-checkable proof terms via OmegaProof certificate reconstruction (2026-05-29)
  - **Goal:** Replace the `sorry` placeholder that `elaborate_by_tactic` emits when omega succeeds with a genuinely kernel-checkable `Expr` proof term, gated by `TypeChecker::check_type`.
  - **Design:** (a) Add `omega_helper` lemma bundle to oxilean-std (axiom-backed Int.le_refl/trans/antisymm/lt_irrefl/add_le_add etc.); (b) thread `OmegaProof` certificate from `tac_omega` (meta) → `MetaBridge::to_elab_state` → elab `TacticState.certificate`; (c) new module `oxilean-elab/src/tactic/proof_recon/` mapping each `OmegaStep` to `Expr::app`-chains over omega_helper `Const` refs; (d) `elaborate_by_tactic` calls reconstructor, gates via `TypeChecker::check_type`, accepts iff `Ok(())`, otherwise keeps `sorry` placeholder.
  - **Files:** `crates/oxilean-std/src/omega_helper/`, `crates/oxilean-meta/src/{tactic/omega/functions.rs, basic/metacontext_type.rs}`, `crates/oxilean-elab/src/{meta_bridge.rs, tactic/types.rs, tactic/proof_recon/, elaborate/functions.rs}`
  - **Prerequisites:** omega_helper lemma bundle (oxilean-std)
  - **Tests:** Self-contained canary tests prove `a ≤ a`, `h:a≤b ⊢ a≤b`, `h:a≤b,h2:b≤c ⊢ a≤c`, `h:a<a ⊢ False` to non-placeholder, TypeChecker-verified terms.
  - **Risk:** Kernel API shape (TypeChecker::check_type, Expr constructors) verified via codebase exploration. Proof term correctness guaranteed by kernel gate — no false proofs possible; worst case is graceful fallback to sorry.
  - **Refinement (2026-05-29):** le_trans/lt_irrefl patterns being added; Int.le goals reconstructed; LE.le surface goals fall back to sorry (opaque axioms, no bridge lemma).
  - **Refinement (2026-05-29):** Cycle 4: FVar threading + LE.le bridge + production env wiring (A1+A2 tracks).
  - **Refinement (2026-05-29):** Cycle 4 complete — FVar threading + LE.le bridge + lt_irrefl via absurd implemented.

## Stubs to implement (added 2026-06-12 by /cooljapan-stub-check)

- [ ] `oxilean-build`: `crates/oxilean-build/src/executor/types.rs:961` — replace placeholder artifact files in `execute_compile` with real compilation output (currently creates empty stubs)
  - Priority: P2 | Scope: medium | Hint: none
- [ ] `oxilean-codegen`: `crates/oxilean-codegen/src/glsl_backend/types/impls1.rs:779` — replace `// TODO: compute work here` placeholder in GLSL compute shader template with parameterizable compute body
  - Priority: P2 | Scope: small | Hint: none

- [x] polyrith: reclaim real Gröbner engine into the live dispatch path (2026-05-29)
  - **Goal:** Replace the i64 brute-force stub in `PolyrithTactic::run` with the real `IdealMembershipChecker`/Buchberger algorithm already present in `oxilean-meta/src/tactic/polyrith/`, and wire `"polyrith"` into `eval_tactic` (currently falls to `UnknownTactic`).
  - **Design:** Parse goal `lhs = rhs` + hypotheses into multivariate ℚ polynomials; compute Gröbner basis of hypothesis ideal via existing `GroebnerBasis::reduce` + `s_polynomial`; test `IdealMembershipChecker::is_member(lhs-rhs, generators)`; emit cofactor certificate on success. Add `"polyrith"` arm to `eval_tactic` (`functions_2.rs:1351` catch-all) bridging to `tac_polyrith` via `meta_bridge::try_meta_tactic`. Proof term stays placeholder this cycle (ring/cofactor kernel reconstruction deferred).
  - **Files:** `crates/oxilean-meta/src/tactic/polyrith/{types/impls/functions_2.rs, functions.rs}`, `crates/oxilean-elab/src/tactic/functions_2.rs`
  - **Prerequisites:** None (engine already exists)
  - **Tests:** Goals the old i64 stub could not solve; negative (non-ideal-member → graceful fail); confirm Gröbner path taken (not brute-force).
  - **Risk:** Polynomial parsing must handle the full Expr AST into ℚ coefficient representation. IdealMembershipChecker confirmed present.

- [x] LSP: semanticTokens/range, codeAction↔lint, advertise incremental sync (2026-05-29)
  - **Goal:** Three completeness items: (3a) add `textDocument/semanticTokens/range` and advertise the provider; (3b) wire oxilean-lint diagnostics into `handle_code_action`; (3c) add edge-case tests for incremental sync then flip text_document_sync from 1 → 2.
  - **Files:** `crates/oxilean-cli/src/lsp/{semantic_tokens/, server/types.rs, lsp_server/mod.rs, lsp_types/mod.rs}`, `crates/oxilean-lint/src/`
  - **Tests:** semanticTokens/range round-trip; codeAction includes lint fix; incremental sync edge-case battery (UTF-16 offsets, CJK, multi-line insert/delete).
  - **Risk:** text_document_sync 1→2 is a client-facing behavior change — edge-case test battery must pass before flipping.

## Stubs to implement (added 2026-06-22 by /cooljapan-stub-check)

- [ ] **oxilean** `oxilean-codegen`: `crates/oxilean-codegen/src/glsl_backend/types/impls1.rs:779` — `TODO`: `compute work here` (the GLSL `compute_shader_template` emits a placeholder main body, unlike the fully-populated vertex/fragment templates in the same file)
  - **Priority:** P2  **Scope:** small  **Cross-project:** none
  - **Approach:** Give `compute_shader_template` a real parameterized body (e.g. a bounds-guarded SSBO read-modify-write over `gl_GlobalInvocationID`) so the generated compute shader is usable, mirroring how `*_shader_template` siblings emit concrete statements; add a codegen test asserting the emitted body + `layout(local_size_*)` line.
  - **Risk:** Low — string-template codegen only. Keep the emitted GLSL minimal/valid so it compiles under the advertised `GL_ARB_compute_shader` extension; this is not the lint-engine's TODO-detection string (those are intentional literals and out of scope).
