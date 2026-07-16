# oxilean-elab — TODO

> Task list for the elaborator crate.
> Last updated: 2026-05-29

## ✅ Completed

- [x] Module structure (466 files, ~92,415 SLOC total)
- [x] `MetaVarId` type (u64 wrapper with derive traits)
- [x] Re-exports in `lib.rs`

---

## ✅ Completed (Phase 3): Elaborator Core

### Meta-variable Infrastructure (`metavar.rs` — 166 lines)
- [x] `MetaContext` struct — map from `MetaVarId` to assignment status
- [x] `create_mvar(type)` — create a fresh metavariable with expected type
- [x] `assign(mvar, expr)` — assign a value to a metavariable
- [x] `is_assigned(mvar)` / `is_resolved(mvar)` — check assignment status
- [x] `get_unresolved()` — list unresolved metavariables
- [x] `count()` — total count
- [x] `zonk(expr)` — replace all assigned metavariables (recursive traversal)
- [x] Occurs check
- [x] Scope management for metavariables

### Unification (`unify.rs` — 148 lines + `solver.rs` — 181 lines)
- [x] `unify(lhs, rhs)` → `Result<(), UnifyError>` — structural equality
- [x] Structural comparison: Sort, BVar, FVar, Const, App, Lam, Pi, Let, Lit, Proj
- [x] First-order unification: `?m =? t` (metavar-aware) — assignment with occurs check
- [x] Priority-based constraint scheduler with postponement and retry
- [x] Definitional equality integration via kernel `is_def_eq`

### Expression Elaboration (`elaborate.rs` — 2,237 lines)
- [x] `ElabContext` struct with env, local context, meta context
- [x] `elab_expr(surface_expr)` → `Result<Expr, ElabError>`
- [x] Name resolution: local → global → overload resolution
- [x] Application elaboration with implicit argument insertion
- [x] Lambda elaboration (infer binder types from expected type)
- [x] Pi / Arrow elaboration
- [x] Let elaboration
- [x] Literal elaboration (Nat/String)
- [x] Hole `_` → create metavariable
- [x] Projection elaboration (`e.field`)
- [x] Match expression elaboration
- [x] `by` block → invoke tactic engine
- [x] If/then/else elaboration
- [x] Do-notation elaboration
- [x] Have / Suffices / Show expressions
- [x] Named arguments
- [x] Anonymous constructors, list literals, tuples
- [x] String interpolation, range expressions
- [x] Calc blocks
- [x] Type-directed elaboration with expected type propagation
- [x] Overload resolution

### Pattern Match Compilation (`pattern_match.rs` — 1,819 lines + `equation.rs` — 240 lines)
- [x] Surface patterns → decision tree
- [x] Scrutinee elaboration → pattern elaboration → exhaustiveness check → compile
- [x] Exhaustiveness checking (constructor set validation)
- [x] Redundancy checking (catch-all detection, subsumption)
- [x] Pattern elaboration: Wild, Var, Ctor, Lit, Or

### Declaration Elaboration (`elab_decl.rs` — 1,567 lines)
- [x] `elab_def(name, params, ret_ty, body)` — definition elaboration
- [x] `elab_theorem(name, params, ty, proof)` — theorem elaboration
- [x] `elab_axiom(name, params, ty)` — axiom elaboration
- [x] `elab_inductive(name, params, ty, ctors)` — inductive type elaboration
- [x] Universe parameter inference / checking
- [x] Mutual recursion support (forward declare → elaborate → assign)
- [x] Where clause elaboration (let-binding generation)
- [x] Opaque declarations
- [x] Structure / Class / Instance declarations
- [x] Namespace / Section / Variable / Open / Attribute / HashCmd
- [x] Attribute processing (simp/ext/instance/reducible/irreducible/inline etc.)

### Namespace & Import Resolution (`module_import.rs` — 1,983 lines)
- [x] Hierarchical namespace management
- [x] Module definition (name, imports, exports, visibility: Public/Protected/Private)
- [x] Import resolution with selective/hiding/renamed imports
- [x] Multi-module management (`ModuleManager`)
- [x] Dependency graph with cycle detection (DFS) and topological sort (Kahn)

### Command Elaboration (`command_elab.rs` — 1,850 lines)
- [x] Section / End section (with variable abstraction)
- [x] Namespace / End namespace
- [x] Variable / Universe declarations
- [x] Open / Set option
- [x] `#check`, `#eval`, `#print` commands

### Termination Checking (`mutual.rs` — 1,575 lines + `equation.rs`)
- [x] Structural recursion detection and checking
- [x] Recursive call collection and decreasing argument analysis
- [x] Well-founded recursion (WellFounded.fix term construction)
- [x] Mutual recursion support

---

## ✅ Completed (Phase 4, partial): Tactics

### Tactic Infrastructure (`tactic.rs` — 1,604 lines)
- [x] `TacticState` struct (goals, solved)
- [x] `Goal` struct (mvar_id, hypotheses, local_ctx, target, tag)
- [x] Tactic combinator framework (sequence execution)
- [x] Goal focusing
- [x] `TacticRegistry` — registration, lookup, execution (18 tactics registered)
- [x] Undo / backtrack support (snapshot/restore)

### Core Tactics (implemented)
- [x] `intro` / `intros` — introduce Pi binder as hypothesis, create new goal
- [x] `apply` — apply function/lemma (simplified)
- [x] `exact` — provide exact proof term
- [x] `assumption` — search local context for matching hypothesis
- [x] `rfl` — prove `a = a` by reflexivity (Eq pattern detection)
- [x] `trivial` — try refl → assumption → True.intro
- [x] `constructor` — apply constructor (True, And patterns)
- [x] `left` / `right` — for disjunction goals (Or)
- [x] `exists` — provide witness for existential (Exists pattern)
- [x] `exfalso` — change goal to False
- [x] `clear` / `rename` / `revert` — hypothesis management
- [x] `have` / `suffices` — introduce intermediate goals
- [x] `sorry` — admit proof

### Additional Tactics (IMPLEMENTED)
- [x] `cases` — And/Or/False/Nat/Exists case split
- [x] `induction` — Nat structural induction with IH
- [x] `rw [h]` / `rw [← h]` — rewrite goal; chain rewrites supported
- [x] `rw [h] at hyp` — rewrite inside a hypothesis
- [x] `simp` / `simp only [h1, h2]` — beta-reduce + built-in rules + rewrites
- [x] `push_neg`, `by_contra`, `contrapose`, `split` — logic tactics
- [x] `omega`, `ring`, `linarith`, `field_simp` — arithmetic tactics
- [x] `norm_cast`, `exact_mod_cast`, `push_cast` — coercion tactics

---

## ✅ Additional Features (beyond original TODO)

### Attribute System (`attribute.rs` — 1,348 lines)
- [x] 10+ attribute kinds: simp, ext, instance, reducible, irreducible, inline, noinline, specialize, priority, custom
- [x] Duplicate/incompatible attribute checking
- [x] `AttributeRegistry` with handler registration

### Binder Elaboration (`binder.rs` — 1,167 lines)
- [x] Binder type annotation elaboration + metavar generation
- [x] Auto-bound implicit variables
- [x] Instance synthesis (simplified environment-based)

### Coercion System (`coercion.rs` — 965 lines)
- [x] Coercion registration and chaining (BFS shortest chain search)
- [x] Built-in coercions: Nat→Int, Bool→Prop, Int→Rat, generic coe
- [x] Apply coercion chains

### Derive System (`derive.rs` — 1,672 lines + `derive_adv.rs` — 2,543 lines)
- [x] Derive handlers: BEq, DecidableEq, Hashable, Ord, Repr, Inhabited, Nonempty, ToString, Show, Default
- [x] Field comparison + AND chain generation
- [x] Constructor repr, tag hashing

### Structure Elaboration (`structure.rs` — 2,186 lines)
- [x] Structure/Class elaboration with parent field inheritance
- [x] Projection function generation
- [x] Constructor and recursor type generation
- [x] Circular inheritance detection

### Do-Notation Elaboration (`elaborate.rs` — includes Do desugaring)
- [x] DoBlock → bind/pure/map chain transformation
- [x] Bind, LetBind, Action, Return, For, If, Match, TryCatch, Unless elements

### Info Tree (`info_tree.rs` — 2,263 lines)
- [x] TermInfo, FieldInfo, TacticInfo, MacroExpansion, CommandInfo, CompletionInfo
- [x] Tree construction and query
- [x] Hover info, documentation display

### Macro Expansion (`macro_expand.rs` — 1,361 lines)
- [x] SyntaxMacro, CommandMacro, TacticMacro, TermMacro, NotationMacro
- [x] Depth-limited recursive expansion
- [x] Hygienic renaming

### Notation System (`notation.rs` — 1,351 lines)
- [x] Prefix/Infixl/Infixr/Postfix/Notation/Macro kinds
- [x] Scope management, registration, lookup
- [x] Do-notation and list literal expansion

### Parallel Elaboration (`parallel.rs` — 1,605 lines)
- [x] Task scheduling with dependency graph
- [x] Cycle detection (DFS) and topological sort (Kahn)
- [x] Level-parallel execution with max_parallelism
- [x] Progress tracking

### Error Messages (`error_msg.rs` — 877 lines)
- [x] 50+ error codes (E1000-E5010): syntax, type, name, universe, pattern errors
- [x] Levenshtein distance for "did you mean?" suggestions
- [x] Code snippet highlighting

### Implicit Resolution (`implicit.rs` — 106 lines)
- [x] Implicit argument insertion (Pi traversal + metavar generation)
- [x] `infer_implicit` — single-match local hypothesis lookup
- [x] `resolve_instance` — local + global environment search by class head

### Type Class System (`typeclass.rs` — 160 lines + `instance.rs` — 132 lines)
- [x] Class/Instance registration and lookup
- [x] `resolve_constraint` — uses `find_best_instance` from registry
- [x] Priority-based instance scoring

### Inference (`infer.rs` — 223 lines)
- [x] Type inference for basic Expr variants (Sort, BVar, FVar, Const, Lam, Pi, App, Let, Lit)
- [x] Proj inference (delegates to kernel `infer_proj`)
- [x] Universe level instantiation

### Quote/Unquote (`quote.rs` — 189 lines)
- [x] `quote_expr` / `unquote_expr` — full constructor-by-constructor transformation
- [x] Nested Name, Level, BinderInfo, Literal unquoting

### Context (`context.rs` — 174 lines)
- [x] `ElabLocalContext` with push/pop, lookup, metavar support

### Trace (`trace.rs` — 1,041 lines)
- [x] Log levels (Off/Error/Warn/Info/Debug/Trace)
- [x] Categories: Elaboration, TypeInference, Unification, InstanceSynthesis, etc.
- [x] Event recording, filtering, file output

---

## 🐛 Known Issues

None. All previously tracked issues have been resolved as of 2026-03-09.

---

## ⚪ Future Enhancements

- [x] Auto tactic (simple proof search) — `tactic_auto.rs` (AutoTactic, TautoTactic, eval_auto, 12 tests)
- [x] Metaprogramming (user-defined elaborators / tactics) — `metaprog.rs` (UserTactic, UserElab, MacroEngine registries, 7 tests)
- [x] Omega tactic (linear arithmetic) — stub in meta/tactic/omega.rs
- [x] Ring normalization tactic — partial in meta/tactic/ring.rs
- [x] Decision procedures (SAT/SMT via OxiZ integration) — stub in meta/tactic/smt.rs

## v0.1.3 Tactics

> Last updated: 2026-05-29

### Ring 0

- [x] Replace stub `OmegaMetaTactic` with Cooper's algorithm / Omega test for Presburger arithmetic
  - **Implemented:** `src/metaprog/omega_engine.rs` (new module, ~1350 lines) + updated `src/metaprog/omegametatactic_traits.rs`
  - **Algorithm:** Pugh's Omega test — dark shadow (cross-product elimination) + grey shadow (rounding-term recursion); UTF-8-safe parser for linear constraints; equality proof by split strategy (`a=b` ↔ `a≥b ∧ b≥a`); `i128` arithmetic to prevent overflow; MAX_VARS=20 guard
  - **Tests added (34 total):** `omega_trivial_true`, `omega_variable`, `omega_ground_false`, `omega_conjunction`, `omega_unsat_system`, plus 12 more in `omega_engine` module
  - **Result:** All 3441 oxilean-elab tests pass, zero clippy warnings

- [x] Replace stub `RingMetaTactic` with Fourier-Motzkin elimination for linear arithmetic over ordered fields
  - **Goal:** `ringmetatactic_traits.rs` currently returns `Solved` on any equality goal — replace with Fourier-Motzkin variable elimination to prove linear real arithmetic goals
  - **Design:** Negate the goal + collect linear constraints from hypotheses; represent each constraint as `(coefficients: Vec<(String, Rational)>, constant: Rational, op: CmpOp)`; implement FM elimination: for each variable, compute upper/lower bound pairs, add cross-products to residual system; detect empty feasible region = proof; implement `Rational = (i64, i64)` fraction arithmetic inline (no external crate); support `<`, `≤`, `>`, `≥`, `=`, `≠`
  - **Files:** `src/metaprog/ringmetatactic_traits.rs`, `src/metaprog/types.rs`
  - **Tests:** `¬(x > 0 ∧ x < 0)`, `a > b ∧ b > c → a > c`, `2x + 3y ≤ 6 ∧ x ≥ 0 ∧ y ≥ 0` feasibility, redundant constraint removal
  - **Risk:** FM with rational coefficients can blow up — add variable-count guard (refuse if >20 vars), return informative failure

### Ring 1

- [x] `nlinarith` — positivity heuristics + Positivstellensatz for nonlinear arithmetic (2026-05-29)
  - **Goal:** Split `nlinarith` into its own dispatcher arm with Positivstellensatz-lite preprocessing (squares + products of hyp atoms) before `has_farkas_certificate`.
  - **Files:** elab/src/tactic/functions_2.rs, elab/src/tactic/functions_3.rs
  - **Tests:** goals nlinarith can close that linarith cannot.
- [x] `polyrith` — Gröbner basis / Buchberger over ℚ for polynomial ring equalities (2026-05-30, proof recon cycle 1)
- [x] `cc` — congruence closure with E-graphs (Nieuwenhuis-Oliveras algorithm) (2026-05-29)
  - **Goal:** Wire `"cc"|"congruence"` to `tac_cc` (oxilean-meta) via `try_meta_tactic`. Add `ProofCertificate::Direct` handling in `elaborate_by_tactic`.
  - **Files:** elab/src/tactic/functions_2.rs, elab/src/elaborate/functions.rs
  - **Tests:** end-to-end `by cc` closes eq goals via kernel-verified proof.
- [x] `cc` e2e non-sorry tests + production cc_helper/polyrith_helper wiring (cycle 10) (2026-05-30)
  - **Goal:** Wire `register_cc_helper` and `register_polyrith_helper` into CLI production envs; update dispatcher_tests.rs to assert cc congruence proofs are non-sorry and kernel-verified.
  - **Files:** `src/tactic/dispatcher_tests.rs`, `crates/oxilean-cli/src/repl/types.rs`, `crates/oxilean-cli/src/commands/functions.rs`
  - **Tests:** 4 e2e non-sorry cc tests.
- [x] `simp` — wire real meta simp engine + add `tac_simp` (2026-05-29)
  - **Goal:** simp arm currently uses local `apply_simp_rules` only. Add `tac_simp` to meta and bridge before elab-local fallback.
  - **Files:** elab/src/tactic/functions_2.rs; meta/src/tactic/simp/main/functions.rs, tactic/mod.rs
  - **Tests:** `by simp` closes Nat.add_zero/True goals via meta engine.
- [x] Tactic error recovery — better diagnostic messages when tactics fail near the goal (2026-05-30)
  - **Goal:** Enrich `TacticError` with a `TypeMismatchDetailed` variant carrying `expected`/`actual`/`context` strings; add `format_tactic_failure(tactic_name, error, goal)` helper; implement `std::error::Error` for `TacticError`.
  - **Files:** `src/tactic/types.rs`, `src/tactic/tacticerror_traits.rs`, `src/tactic/functions_3_tests.rs`
  - **Tests:** 7 new tests covering `format_tactic_failure`, `TypeMismatchDetailed` display, `InvalidArg` display, `std::error::Error` impl.

### Ring 1 (in progress — this cycle)

- [x] omega tactic → real kernel-checkable proof terms via OmegaProof reconstruction (2026-05-29, extended 2026-05-29)
  - **Goal:** Replace `sorry` placeholder in `elaborate_by_tactic` with a genuine kernel-verified `Expr` proof term when omega succeeds, using the `OmegaProof` certificate produced by `tac_omega`.
  - **Design:** New module `src/tactic/proof_recon/` with `omega_proof_to_expr(proof: &OmegaProof, goal: &Expr, hyps: &[(Name, Expr)], env: &Environment) -> Option<Expr>` mapping goal/hyp patterns to `Expr::App`-chains over omega_helper `Const` references. `elaborate_by_tactic` (`elaborate/functions.rs:974`) passes `&hyps` through, calls reconstructor, then `TypeChecker::check_type` — accept iff `Ok(())`, else fallback to `Expr::Const("sorry")`. Hypothesis augmentation: each hyp is registered as an axiom in a cloned env so `Expr::Const(hyp_name)` references kernel-verify.
  - **Patterns implemented:** `le_refl` (kernel-verifies), `le_trans` (kernel-verifies), `lt_irrefl` (structure implemented; cannot kernel-verify because `Not` is an opaque axiom — documented limitation).
  - **Files:** `src/tactic/proof_recon/mod.rs`, `src/tactic/types.rs`, `src/elaborate/functions.rs`
  - **Prerequisites:** omega_helper lemma bundle in oxilean-std (registered before verification)
  - **Tests (7):** `test_le_refl_reconstructed`, `test_le_refl_distinct_sides_returns_none`, `test_sorry_fallback_for_complex_proofs`, `test_non_int_le_goal_returns_none`, `test_le_trans_reconstructed`, `test_le_trans_no_chain_returns_none`, `test_lt_irrefl_not_opaque_limitation` — all pass.
  - **Known limitations:** (1) `LE.le` vs `Int.le` gap (surface syntax uses opaque LE.le; bridge lemma needed). (2) `Not` opacity: `lt_irrefl` inferred type is `Not(Int.lt a a)` not `False`; kernel cannot reduce opaque `Not`.
  - **Risk:** Kernel gate is the correctness firewall — no risk of forged proofs.

- [x] polyrith: wire real Gröbner engine into live eval_tactic dispatch (2026-05-29)
  - **Goal:** Add `"polyrith"` arm to `eval_tactic` (`src/tactic/functions_2.rs`, currently falls to catch-all `UnknownTactic`) calling `tac_polyrith` via `meta_bridge::try_meta_tactic`.
  - **Design:** One new arm in the match block: `"polyrith" => { if let Ok(s) = crate::meta_bridge::try_meta_tactic(state, env, |bridge| oxilean_meta::tactic::tac_polyrith(&mut bridge.meta_state, &mut bridge.meta_ctx)) { ... } }`. The real Gröbner work is in oxilean-meta; this subagent only adds the dispatch arm.
  - **Files:** `src/tactic/functions_2.rs`
  - **Prerequisites:** oxilean-meta polyrith Gröbner reclaim (must complete first — separate subagent)
  - **Tests:** Integration test: `by polyrith` closes a goal the old UnknownTactic path would reject.
  - **Risk:** Minimal — mirror existing omega/grind arm pattern.

- [x] Wire `polyrith_cert_to_expr` into elaborator dispatch (cycle 11) (2026-05-30)
  - **Goal:** Fix `elaborate/functions.rs:1031` — change `Polyrith(_) => None` to call `polyrith_cert_to_expr`. Add 2 dispatcher tests for polyrith cert wiring.
  - **Files:** `src/elaborate/functions.rs`, `src/tactic/dispatcher_tests.rs`
  - **Tests:** `test_polyrith_1entry_cert_wired` (asserts Polyrith cert + goal closed), `test_polyrith_multi_cert_fallback_no_panic` (regression guard).

- [x] omega proof recon: FVar threading + LE.le bridge + lt_irrefl fix + production env wiring (2026-05-29)
  - **Goal:** Fix the three omega reconstruction blockers: (1) FVar blindness — extend `omega_proof_to_expr` to accept `&[(FVarId, Name, Expr)]` locals; populate `TypeChecker::push_local` with exact FVarIds from `elaborate_by_tactic`; (2) lt_irrefl via absurd — `@absurd (Int.lt a a) goal h (Int.lt_irrefl a)` now returns `Some` and kernel-verifies; (3) LE.le bridge — detect `LE.le Int inst a b`, reconstruct inner `Int.le` proof, wrap with `@le_of_int_le inst a b p`. Wire `elaborate_by_tactic` to dispatch Farkas proof builder (B2 task, from nlinarith).
  - **Refinement (2026-05-29):** Cycle 4 complete — FVar threading + LE.le bridge + lt_irrefl via absurd implemented.
  - **Design:** `omega_proof_to_expr(proof, goal, hyps, locals: &[(FVarId, Name, Expr)], env) -> Option<Expr>`; `verify_proof_term` builds TypeChecker with `push_local(LocalDecl{fvar, name, ty, val:None})` per local. LE.le detection: match `Expr::App(Expr::App(Expr::App(Const("LE.le"), _ty), inst), a)` and recursion.
  - **Files:** `src/tactic/proof_recon/mod.rs`, `src/elaborate/functions.rs`
  - **Tests (10 total):** `test_le_refl_reconstructed`, `test_le_refl_distinct_sides_returns_none`, `test_sorry_fallback_for_complex_proofs`, `test_non_int_le_goal_returns_none`, `test_le_trans_reconstructed`, `test_le_trans_no_chain_returns_none`, `test_lt_irrefl_with_absurd`, `test_le_refl_with_fvar_goal`, `test_le_trans_with_fvar_goal`, `test_le_trans_fvar_goal_with_locals_now_succeeds` — all pass.
  - **Key fixes:** `verify_proof_term` now has `locals: &[(FVarId, Name, Expr)]` arg; `elaborate_by_tactic` collects `ctx.locals()` and passes as `&locals`; `Linarith` cert arm returns `None` (farkas.rs skeleton); `Direct` cert arm also injects locals into checker. `hyp_proof_term` prefers `FVar(id)` over `Const(name)` for proof refs.

- [x] nlinarith: `find_farkas_certificate` + `proof_recon/farkas.rs` (2026-05-29)
  - **Goal:** Refactor `has_farkas_certificate` (functions_3.rs) → `find_farkas_certificate(cons) -> Option<FarkasCert>` retaining multipliers+source map; thin bool wrapper for backward compat. `nlinarith` arm stores `ProofCertificate::Linarith(cert)` on success. `proof_recon/farkas.rs`: `farkas_cert_to_expr(cert, goal, hyps, locals, env) -> Option<Expr>` — kernel-gated, safe fallback `None`.
  - **Files:** `src/tactic/functions_3.rs`, `src/tactic/proof_recon/farkas.rs`
  - **Implemented:** `find_farkas_certificate` returns `Option<FarkasCert>` with all 6 search cases retaining multipliers; `has_farkas_certificate` is thin `is_some()` wrapper; `try_nlinarith_with_positivstellensatz` stores `ProofCertificate::Linarith(cert)` on success; `farkas_cert_to_expr` guards (invalid cert, fractional multiplier, out-of-bounds, empty entries/sources) + proof builders (single/two/multi constraint via `Int.add_le_add`+`Int.le_trans`+`Int.absurd_le_zero`) with kernel gate.
  - **A3 additions (2026-05-29):** Provenance plumbing: `parse_hyps_with_sources` returns `(Vec<SymLinCon>, Vec<ConSource>)` in parallel; `try_nlinarith_with_positivstellensatz` builds augmented_sources (negated-goal + hyp sources + augmented placeholders) and attaches via `FarkasCert::with_sources`. `farkas_cert_to_expr` updated signature `(cert, goal, hyps, locals, env)`; FVar threading in `verify_proof_term`; builders implemented with 4 strategies; elaborate/functions.rs Linarith arm wired. 8 tests passing.
  - **Tests (8):** invalid cert → None; fractional multiplier → None; empty constraints → None; no sources → None; augmented source → None; out-of-bounds → None; single-entry (no axioms) → None (kernel gate); two-entry named sources → None or Some (kernel-verified if Some).

- [x] Farkas proof reconstruction: transitivity-cycle builder (strict-aware) + LE.le/LT.lt bridge + real verified tests (cycle 6) (2026-05-29)
  - **Goal:** Replace the broken `Int.absurd_le_zero`-based strategies in `proof_recon/farkas.rs` (which need literal Int arithmetic) with a strict-aware transitivity-cycle builder that produces kernel-verified non-`sorry` terms for the tractable opaque-Int class.
  - **Design:** `extract_int_rel` recognizes `Int.le`/`Int.lt`/`LE.le`/`LT.lt` (bridge LE.le/LT.lt hyp proofs via `int_le_of_le`). Resolve unit-multiplier cert entries → (lhs, rhs, strictness, proof); find a cycle with ≥1 strict edge; fold via `le_trans`/`lt_of_le_of_lt`/`lt_of_lt_of_le`/`lt_trans` → `Int.lt a a`; close via `Int.lt_irrefl a`. Kernel-gate (infer_type + is_def_eq) → else `sorry`. Also wire `add_omega_lemmas` into the CLI/REPL production env.
  - **Files:** `crates/oxilean-elab/src/tactic/proof_recon/farkas.rs`, `crates/oxilean-elab/src/elaborate/functions.rs`, `crates/oxilean-cli/src/commands/functions.rs`, `crates/oxilean-cli/src/repl/types.rs`.
  - **Prerequisites:** oxilean-std omega_helper lt-transitivity lemmas (cycle 6 A1).
  - **Tests:** REAL e2e (env with register_omega_helper) — `a ≤ b, b ≤ c, c < a ⊢ False` asserts Some + non-`sorry` + kernel-verifies; `a < b, b < a ⊢ False`; LE.le/LT.lt variant; negative → `sorry`.

- [x] Farkas proof reconstruction: Bool-reflection arithmetic chain close (cycle 8) (2026-05-30)
  - **Goal:** Extend `try_cycle_from` in `farkas.rs` to close chains `a ≤/< b` with ground `Literal::Int` endpoints where the arithmetic relation is false, via `Int.not_le_of_ble_false`/`Int.not_lt_of_blt_false` + `Eq.refl Bool Bool.false`. Closes `0 ≤ x, x ≤ -1 ⊢ False` with a fully kernel-verified proof term.
  - **Design:** After the syntactic cycle check in `try_cycle_from`, add Strategy 2: if `acc_lhs` and `acc_rhs` are both `Expr::Lit(Literal::Int(_))` and arithmetic contradiction holds, build `not_lemma acc_lhs acc_rhs (Eq.refl.{1} Bool Bool.false) acc_proof` and kernel-gate. Added `mk_app4`, `extract_int_lit`, `mk_bool_false_rfl` helpers. Also fixed a kernel naming inconsistency: `bool_result` in `reduce/functions.rs` now returns flat `Name::str("Bool.false")` (matching builtin env registration) instead of hierarchical `Name::str("Bool").append_str("false")`.
  - **Files:** `crates/oxilean-elab/src/tactic/proof_recon/farkas.rs`, `crates/oxilean-std/src/omega_helper/mod.rs`, `crates/oxilean-kernel/src/reduce/functions.rs`.
  - **Tests:** 3 new tests: Le chain (0 ≤ x ≤ -1 → contradiction), all-Le (2 ≤ x ≤ 1 → contradiction), Lt+Le (1 < x ≤ 0 → contradiction). All assert Some + non-sorry + kernel-verifies.
