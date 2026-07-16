# oxilean-meta — TODO

> Task list for the metaprogramming crate.
> Last updated: 2026-05-29

## ✅ Completed

**Status**: COMPLETE — ~152,716 SLOC implemented across 648 source files

### Core Metaprogramming Infrastructure
- [x] Expression manipulation and analysis
- [x] AST transformation utilities
- [x] Tactic metaprogramming support
- [x] Meta-level computation
- [x] Reflection primitives
- [x] Quotation and antiquotation
- [x] Syntax tree manipulation
- [x] Code generation helpers

### Meta-level Operations
- [x] Expression construction
- [x] Pattern matching on AST
- [x] Type-level computation
- [x] Compile-time evaluation

---

## 🐛 Known Issues

None reported. All tests passing.

---

## ✅ Completed: Extended Meta Features

- [x] Additional convenience macros for common patterns — `convenience.rs` (mk_const, mk_app, mk_pi, mk_lam, mk_arrow, mk_eq, etc., 10 tests)
- [x] Enhanced debugging support for metaprograms — `meta_debug.rs` (ExprPrinter, TraceLogger, PrettyCtx, 6 tests)
- [x] SMT solver integration — `tactic/smt.rs` (SmtSolver, SmtGoal, z3/cvc5/yices2 backends, 8 tests)
- [x] Property-based testing — `prop_test.rs` (QuickCheck-like framework, ExprGen, PropTest, 8 tests)
- [x] Performance optimizations for large AST transformations

## v0.1.3 Tactic Meta-Hooks

> Last updated: 2026-05-29

### Ring 0

- [x] Thread OmegaProof certificate from tac_omega through MetaBridge to elab TacticState (2026-05-29)
  - **Goal:** `tac_omega` currently builds `OmegaProof` at `omega/functions.rs:88` but drops it at `:95` (`build_omega_proof` returns a placeholder). Thread the certificate out so `elaborate_by_tactic` can attempt kernel reconstruction.
  - **Design:** (1) Add `pub enum ProofCertificate { Omega(OmegaProof), Linarith(FarkasCert) }` to `src/tactic/mod.rs` (or new `src/tactic/certificate.rs`) + re-export; (2) add `pub last_certificate: Option<ProofCertificate>` to `MetaContext` (`src/basic/metacontext_type.rs:14`); (3) in `tac_omega` (`tactic/omega/functions.rs:234`), store the `OmegaResult::Unsatisfiable(proof)` into `ctx.last_certificate = Some(ProofCertificate::Omega(proof))` before `close_goal`; (4) OmegaProof already fully defined at `tactic/omega/types.rs:977` — no changes there.
  - **Files:** `src/tactic/mod.rs` (or new `src/tactic/certificate.rs`), `src/basic/metacontext_type.rs`, `src/tactic/omega/functions.rs`
  - **Prerequisites:** None — OmegaProof type already exists
  - **Tests:** After `tac_omega` runs successfully on a test constraint, `ctx.last_certificate` is `Some(ProofCertificate::Omega(_))` with non-empty steps.
  - **Risk:** `MetaContext` field addition must not break existing code that constructs MetaContext — add `last_certificate: None` to all construction sites. Use grep to find them.

### Ring 1

- [x] PolyrithCert + ProofCertificate::Polyrith + MetaContext.last_polyrith_cert (2026-05-30)
- [x] SMT bridge for polyrith — OxiZ-math Gröbner validation (cycle 11) (2026-05-30)
  - **Goal:** Add `oxiz-math` dependency; implement polynomial conversion bridge (in-house Polynomial ↔ oxiz_math::Polynomial); use `oxiz_math::grobner_basis` for real Buchberger basis; validate certificates via `oxiz_math::ideal_membership`; add `validated: bool` to `PolyrithCert`.
  - **Files:** `Cargo.toml` (root + oxilean-meta), `src/tactic/polyrith/functions.rs`, `src/tactic/certificate.rs`
  - **Implemented:** `build_var_map_for_oxiz`, `to_oxiz_poly`, `oxiz_validate_ideal_membership` in `functions.rs`; `validated: bool` field on `PolyrithCert`; `tac_polyrith` now calls OxiZ-math Buchberger and stores result in cert.
  - **Tests:** 7 new OxiZ bridge tests (conversion round-trip, zero poly, membership true/false, validated field, linear combination, multivariate quadratic) — all passing. 124 total polyrith tests pass. Zero clippy warnings.
- [x] FarkasCert enrichment for nlinarith proof reconstruction (2026-05-29)
  - **Goal:** Enrich `FarkasCert` (`src/tactic/linear_combination/types/defs.rs`) to carry per-source multiplier + constraint index/orientation needed for proof-term reconstruction; drop `#[allow(dead_code)]`. Add `FarkasCertEntry { constraint_index: usize, multiplier: Rat, orientation: ConOrient }` field to `FarkasCert`.
  - **Design:** `pub struct FarkasCert { pub entries: Vec<FarkasCertEntry>, pub combined_rhs: Rat }` where `ConOrient` marks whether the source constraint is ≥0 or ≤0. Provide a `from_search(pairs: &[(usize, Rat, ConOrient)]) -> Self` constructor.
  - **Files:** `src/tactic/linear_combination/types/defs.rs`, `src/tactic/linear_combination/types/impls/` (if split)
  - **Prerequisites:** None
  - **Tests:** Constructor + `is_valid_refutation` still holds; field access unit tests.
  - **Risk:** `#[allow(dead_code)]` removal causes clippy -D warnings if FarkasCert is used nowhere else — verify usage, or add minimal usage in elab (B2 task).
- [x] Meta-level proof term export for cc (congruence closure certificates) (2026-05-29)
  - **Goal:** Add `tac_cc` to oxilean-meta (using grind CongruenceClosure with existing proof-producing `explain_equality`/`merge_log`/`MergeReason`). Add `ProofCertificate::Direct(Expr)` variant to `tactic/certificate.rs`.
  - **Files:** meta/src/tactic/grind/functions.rs (tac_cc), meta/src/tactic/certificate.rs (Direct variant), meta/src/tactic/mod.rs (re-export)
  - **Tests:** CC proof terms kernel-verified via TypeChecker gate.
- [x] `cc` proof reconstruction — NO proof forest + kernel-correct congruence terms (cycle 10) (2026-05-30)
  - **Goal:** Implement the Nieuwenhuis-Oliveras explain operation in `grind/cc_proof.rs`: proof-forest on ENodeIds (separate from union-find), LCA-based explain, recursive congruence argument explain, kernel-correct proof-term builder supplying all implicit args explicitly.
  - **Files:** `src/tactic/grind/types.rs` (ProofLabel enum, proof_parent field, reroot_proof_tree, flip_label, updated merge_with_reason), `src/tactic/grind/cc_proof.rs` (new — 1241 lines: explain, ExplainStep, build_eq_proof_with_locals, mk_eq_refl_full/symm_full/trans_full/congr_full), `src/tactic/grind/functions.rs` (tac_cc updated to use cc_proof::explain + build_eq_proof_with_locals), `src/tactic/grind/mod.rs` (cc_proof module added).
  - **Also fixed:** `oxilean-std/cc_helper/mod.rs` — corrected erroneous De Bruijn indices in `ty_congr()` (g's type and hf's (α→β) encoding were wrong).
  - **Tests:** 14 tests passing — 5 kernel-verified proof-term tests (refl, hyp, symm, trans, single-arg congruence, multi-arg congruence, nested congruence) + explain unit tests + decompose spine tests. All 5403 oxilean-meta tests pass.
