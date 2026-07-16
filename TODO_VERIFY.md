# oxilean-verify — Production Readiness TODO

> Master task list for shipping **oxilean-verify** (independent Lean 4 proof checker,
> "Kernel in a Tab") as a production-grade product, per the engineering brief
> (`docs/audit/2026-07-12-verify-gap/00-engineering-brief.md`).
>
> Derived from the 14-area gap audit of 2026-07-12
> (`docs/audit/2026-07-12-verify-gap/`). Baseline: workspace fully green
> (32,928 tests pass, clippy silent) — but green because the broken paths have
> zero coverage.
>
> Status legend: `[ ]` open · `[~]` in progress · `[x]` done · `[>]` deliberate follow-up (out of this pass)

---

## Campaign result (2026-07-15)

The campaign is **shipped** at 0.1.3 (branches `wave4/fuzz-ci`,
`wave4/diff-harness`, `wave4/release-polish` merged; final quality loop green).

**Headline numbers:**

| metric | value |
|---|---|
| Lean core `Init` (all 57,277 decls) | **35,424 verified · 21,853 unsupported (named) · 0 rejected**, exit 0 (post-wave5 structural sharing; +201 verified vs the pre-sharing 35,223, strict superset) |
| unsupported attribution | `Lean.Syntax` nested-inductive cascade dominates; 0 unattributed |
| differential vs lean4lean (`Init.Prelude`, 1,987 joined) | **0 disagreements** (was 20 — all one `noConfusion` kernel bug, fixed by C15) |
| throughput (full Init, corpus cage) | **≈83 decls/s / 11 m 28 s wall / peak RSS ≈6 GiB, no swap** (wave5 structural sharing; was 11.8 decls/s / 1 h 20 m 44 s / 9.98 GiB + ~6.7 GiB swap-thrash — ≈7× faster) |
| throughput (`Init.Prelude`, apples-to-apples) | ~398 decls/s vs lean4lean ~3,280 (~8.2× slower; pre-sharing — NOT re-measured after wave5, whose ≈7× came mostly from O(distinct) materialisation + eliminating swap thrash) |
| wasm artifact | raw 371,592 B · **gzip 147,132 B (~144 KB)** vs 400 KB budget; 39 exports; validate VALID |
| workspace tests | **33,231 passed, 746 ignored** (nextest); clippy `-D warnings` silent; fmt clean; TCB doc-tests pass |
| gates (all passing) | zero-deps · allow-list · forbid-unsafe (kernel/export/verify/verify-wasm) · wasm export-count · wasm size · native determinism · demo smoke |
| fuzzing | 3 targets (`fuzz_bignat`, `fuzz_ndjson_parse`, `fuzz_replay`) build + run clean; weekly CI cron; ~850K accumulated execs, zero findings |
| TCB closure | `cargo tree -p oxilean-verify` = kernel 0.1.3 + export 0.1.3 only; zero external crates |

**Still open, and why:**

- **Throughput budget not met** (11.8 decls/s corpus / ~398 decls/s Prelude vs
  ≥656 needed): structural, not incidental — deep-clone substitution over the
  unshared `Box` kernel `Expr`. Fix = structural sharing (hash-consing/`Rc`);
  deliberately out of scope: TCB surgery, own review cycle. Same fix removes
  the ~16 GiB corpus memory footprint.
- **Nested inductives** (nested→mutual elaboration, `Lean.Syntax`): the single
  highest-leverage completeness item — would unlock up to 14,347 Syntax-only
  followers of the 22,054 unsupported.
- **Resource-limit tail**: 175 legal-but-heavy roots stay in the NAMED
  resource-limit bucket at the 2^26 corpus fuel preset (spot-checked verified
  at 2^30 under a cage); raising the preset re-opens the C16b OOM, so it stays.
- **G7 wasm half**: no node/deno/headless browser in this environment; native
  half + structural wasm validation + shared engine path stand in.
- **lean4lean version skew**: no lean4lean tag for v4.32.0-rc1 (master =
  v4.29.0) — declaration-set skew documented, lands in ONLY-ONE-SIDE.
- **Corpus re-export scripting** (`scripts/regen-corpus.sh` committed; the
  elan/lean4export install steps remain manual) — V5 residual.
- **F1–F4** deliberate follow-ups (TCB slimming, MSRV, legacy export format,
  announcement) — unchanged.

---

## P0 — Soundness bugs (kernel accepts wrong things)

- [x] **S1. Nat literal u64 overflow** — `Literal::Nat(u64)` with unchecked `+ * pow`:
  release builds silently wrap (would accept `2^32 * 2^32 = 0`). Replace with
  hand-written zero-dep arbitrary-precision `BigNat` (schoolbook + Karatsuba),
  differential-tested vs `num-bigint` (dev-dep only), fuzzed.
  (`audit-literals-bignum.md`)
  *(Wave 1: `oxilean_kernel::bignat` landed — Karatsuba mul, Knuth-D div,
  MAX_RESULT_BITS guard leaves terms stuck instead of wrong; differential +
  proptest suites vs num-bigint dev-dep; cargo-fuzz targets deferred to G9.)*
- [x] **S2. Quot over-application drops args** — `try_reduce_quot` returns `f a`,
  discarding `args[mk_pos+1..]` instead of re-applying (unsound whnf feeding def_eq).
  Same bug in `try_reduce_recursor`. (`audit-quotients.md`, `audit-recursors.md`)
  *(Wave 2: quot iota now `Reducer::try_reduce_quot`, re-applies `args[mk_pos+1..]`;
  recursor iota (`reduce/iota.rs`) re-applies `args[major+1..]`; both pinned by
  over-application tests.)*
- [x] **S3. check_* trusts caller-supplied types** — `check_quotient_val` /
  `check_inductive_val` / recursor checking only `ensure_sort` what the (untrusted!)
  export file supplies. Kernel must construct/derive canonical quotient types and
  recursors itself. (`audit-quotients.md`, `audit-recursors.md`)
  *(Wave 2: `check_quotient_val` validates level params + def-eq vs the
  kernel-built canonical types; recursors are re-derived from the env's
  inductives/ctors and compared (metadata + def-eq of type and rule RHSes) —
  tampered types/rules/K flags rejected.)*
- [x] **S4. No large-elimination decision** — `is_prop` is caller-trusted; a Prop
  inductive declared `is_prop=false` gets unsound large elimination. Kernel must
  decide elimination level itself. (`audit-recursors.md`)
  *(Wave 2: `is_large_eliminating` computed by the kernel on NORMALIZED levels;
  the `is_prop` flag is validated against the declared sort and never drives
  the decision.)*
- [x] **S5. Recursor derivation bugs** — minor premises generated with NO induction
  hypotheses; de Bruijn off-by-one with params/indices; universe-polymorphic
  inductives get empty level lists. (`audit-recursors.md`)
  *(Wave 2: `inductive/derive.rs` rebuilds derivation on an FVar telescope —
  IHs (Pi-wrapped for reflexive fields), correct binders, level params
  propagated into every generated ConstantInfo; golden tests vs Lean's types.)*
- [x] **S6. Builtin placeholder recursors** — `init_builtin_env` recursors have
  Sort-0 types and `BVar(0)` rule RHSes (e.g. `Nat.rec m z s Nat.zero ⟶ s`). Replace
  with correctly derived ones. (`audit-recursors.md`)
  *(Wave 2: all builtin inductives now go through the CHECKED
  `add_inductive_family`; `Eq` reshaped Lean-exact (2 params/1 index/K);
  `String` a real inductive; rule RHSes are closed lambdas per Lean.)*
- [x] **S7. infer_const accepts empty level list** for universe-polymorphic constants
  (returns uninstantiated type). Enforce arity. (`audit-levels.md`)
  *(Wave 2: `infer_const` hard-errors on ANY level-arity mismatch incl.
  empty-vs-nonempty; legacy Declaration path also instantiates now.)*
- [x] **S8. Universe-param hygiene absent** — duplicate/undeclared params and MVars
  accepted in declarations. (`audit-levels.md`)
  *(Wave 2: `check_univ_param_hygiene` rejects dups/undeclared/MVars at the top
  of `check_declaration`. Wave 3b: shared core
  `check_univ_param_hygiene_parts` + `check_univ_param_hygiene_ci` now runs at
  the top of `check_constant_info` for all 8 variants; `Level::MVar` stays in
  the enum — rejected in decls, treated conservatively in leq.)*
- [x] **S9. Wrong builtin `Quot` type** — built as `{α : Sort u} → Prop → Sort u`
  (relation domain collapsed) and registered as Axiom. (`audit-quotients.md`)
  *(Wave 2: wrong axiom removed; `init_builtin_env` calls `add_quot`, which
  installs the four canonical kernel-built QuotVals.)*
- [x] **S10. `Literal::Int(i64)` wrapping arithmetic** — non-Lean extension inside the
  TCB; remove from kernel checking path. (`audit-literals-bignum.md`)
  *(Wave 1: variant deleted from `Literal`, `try_reduce_int_app` + i64 wrapping
  evaluators removed; serialization tag 2 is now a hard read error.)*
- [x] **S11. `infer_proj_field_type` fabricates a type on telescope mismatch** instead
  of erroring. (`audit-struct-eta.md`)
  *(Wave 2: returns `Result` with typed errors for head/param-count/telescope
  mismatches; `infer_proj` gated by the strict `is_structure_like` predicate.)*

## P0 — Completeness bugs (kernel rejects real proofs → false `rejected` alarms)

- [x] **C1. Struct eta NOT in def_eq** — `s ≡ ⟨s.1, s.2⟩` returns false; `struct_eta/`
  module is dead code (zero external callers) and itself buggy. Implement lazy
  struct-eta both directions in `DefEqChecker`, plus unit-like (`isDefEqUnitLike`)
  case, using the (already-correct) `is_structure_like` predicate. (`audit-struct-eta.md`)
  *(Wave 2: `try_eta_struct` both orientations + `is_def_eq_unit_like` wired
  into `is_def_eq_core` in Lean's order; dead `struct_eta/` module deleted.
  Known limit: does not fire on FVar/BVar-typed terms — def_eq has no local
  context yet (stuck, never wrong).)*
- [x] **C2. Function eta single-binder only** — `(fun x y => g x y) ≡ g` fails. (`audit-struct-eta.md`)
  *(Wave 2: one-step `eta_expand_one` (Lean `tryEtaExpansionCore`) added behind
  the contraction fast path; multi-binder telescopes hold.)*
- [x] **C3. `imax(u,u) → u` normalisation missing** — verified end-to-end that
  `def Endo.{u} : Sort u → Sort u := fun a => a → a` is spuriously REJECTED.
  Also: max numeral subsumption, geq zero-base / strip-common-succ cases, full
  param-case-split leq. Use smart constructors in `infer`. (`audit-levels.md`)
  *(Wave 2: `level/order.rs` — smart ctors + the COMPLETE param-case-split leq
  (trepplein/nanoda algorithm), exhaustively + proptest-verified vs a reference
  evaluator; Pi inference uses `mk_imax`; Endo accepted end-to-end.)*
- [x] **C4. Quot.ind iota off-by-one** — requires ≥6 args, reads `args[5]`/`args[4]`;
  Lean: 5 args, mk_pos=4, minor at 3. Fully-applied `Quot.ind` never reduces;
  6-arg forms mis-reduce. (`audit-quotients.md`)
  *(Wave 2: Ind fires at exact arity 5 (arg_pos 3 / mk_pos 4), Lift at 6
  (arg_pos 3 / mk_pos 5), element = `mk_args[2]`; pinned by arity tests.)*
- [x] **C5. Major premise not whnf'd** before Quot.mk / constructor matching (both
  quot and recursor iota). Nat-op args also not whnf'd, so nested literal
  arithmetic never folds. (`audit-quotients.md`, `audit-literals-bignum.md`)
  *(Wave 1: Nat-op args whnf'd, exact arity, no args dropped. Wave 2: both
  quot and recursor iota now whnf the major premise before matching.)*
- [x] **C6. NatLit ↔ Nat.zero/Nat.succ bridge missing** — `0` not defeq `Nat.zero`;
  `Nat.rec` stuck on any literal. Same for `String.rec` on StrLit. (`audit-literals-bignum.md`)
  *(Wave 2: iota-level expansion DONE — `Nat.rec`/`String.rec` compute on
  literals (`0 ↦ Nat.zero`, `n ↦ Nat.succ (n-1)`, chars via `Char.ofNat`).
  Wave 3b: the def_eq bridge landed — `try_lit_ext` (Lean `natLitExt?`/
  `strLitExt?`) decides `NatLit n =?= Nat.zero/Nat.succ e` and
  `StrLit s =?= String.mk l` before lazy delta, magnitude-bounded (peels the
  constructor side, never materialises a succ-tower; `10^9` vs a shallow
  tower fails fast); 11 dedicated tests in `lit_bridge_defeq.rs`.)*
- [x] **C7. K-like reduction unimplemented** — `RecursorVal.k` flag never read
  (Eq.rec etc. stuck without it). (`audit-recursors.md`)
  *(Wave 2: K flag computed per Lean (non-mutual Prop, 1 ctor, 0 fields);
  `to_ctor_when_k` fires on stuck defeq-refl proofs. Known limit: majors with
  free vars from an outer local context stay stuck (fresh checker) —
  incomplete, never wrong.)*
- [x] **C8. Literal op semantic deviations vs Lean** — `pow` exponent truncated to
  u32; `shiftLeft` drops high bits; `String.length` counts bytes not chars;
  duplicate evaluator returns `x % 0 = 0` instead of `x`. (`audit-literals-bignum.md`)
  *(Wave 1: all four fixed; single `eval_nat_binop`/`eval_nat_cmp` evaluator,
  duplicate u64 evaluators deleted; `Nat.log2` implemented; `Nat.beq/ble/blt`
  yield genuine `Bool.true`/`Bool.false`; all pinned by tests.)*
- [x] **C9. Mutual inductives** — `num_motives` hardcoded 1, no multi-motive recursor
  generation/iota. Implement full mutual support. (`audit-recursors.md`)
  *(Wave 2: multi-motive derivation + iota for mutual families
  (Tree/Forest pinned); sibling recursor names follow lean4export's
  `<T>.rec` convention.)*
- [>] **C10. Nested inductives** — decorative bool flag; no nested-to-mutual
  compilation. Implement, or emit named `unsupported` verdict (three-bucket rule). (`audit-recursors.md`)
  *(Wave 2: the second disjunct landed — positivity checking rejects nested
  occurrences with typed `KernelError::UnsupportedNestedInductive` for the
  verify CLI's `unsupported` bucket; nested→mutual compilation itself is
  deliberately NOT implemented. Campaign close: this is now the single
  highest-leverage completeness item — `Lean.Syntax` alone cascades to 98.9%
  of Init's 22,054 unsupported (up to 14,347 Syntax-only followers). Promoted
  to a deliberate follow-up with its own design pass.)*
- [x] **C11. Strict positivity purely syntactic and not wired into checking** —
  defeatable via definitions/Let; single-type only. Wire into `add_inductive`
  path and extend. (`audit-recursors.md`)
  *(Wave 2: positivity wired into the checked `add_inductive_family` path;
  definition-hidden negativity caught via whnf; mutual siblings covered.)*
- [x] **C12. Quotient env API missing** — no `Environment::add_quot()` (#QUOT
  handler): Eq-presence check, canonical 4 declarations, once-only flag. (`audit-quotients.md`)
  *(Wave 2: `env/quot.rs` — structural Eq-presence check, atomic install of
  the four kernel-built QuotVals, once-only flag; direct
  `ConstantInfo::Quotient` additions rejected. `Quot.sound` stays an Axiom
  for the export reader; toy `quotient/` module quarantined, unsound helpers
  deleted.)*

- [x] **C13. Literal acceleration bypassed by head delta-unfold** (found by the
  Wave-3b Init.ndjson smoke) — `whnf_core` unfolded a DEFINED `Nat.ble` before
  trying the literal extension, sending `Nat.ble 55297 4294967296` into ~55K
  layers of unary `Nat.rec`: stack overflow / multi-GiB `Nat.below` towers /
  stuck term → wrong REJECTION of Lean core's `isValidChar_UInt32`.
  *(Wave 3b: extension now tried on the original Const head BEFORE unfolding
  (Lean kernel order); Bool results re-spelled for hierarchical replay envs
  (`respell_bool_const_for_env` — flat `"Bool.true"` doesn't resolve there,
  leaving `Bool.rec` stuck → wrong REJECTION of `noConfusion_of_Nat`);
  pinned by `tests/corpus_init_regressions.rs`.)*
- [x] **C14. def_eq could not type FVars** — `quick_infer_type` had no `FVar`
  arm, so one-sided eta (`eta_expand_one`) failed on any Pi-bound function vs
  lambda comparison → wrong REJECTION of Lean core's `funext` (and 130+
  cascading: `Classical.em`, `WellFounded` fixpoints, …).
  *(Wave 3b: `DefEqChecker.fvar_types` mirrored from the TypeChecker's local
  context via `record_fvar_type` (fresh_fvar/fresh_fvar_let/push_local; cache
  invalidated on conflicting re-bind); `quick_infer_type` FVar arm added.
  Distinct-FVar soundness control pinned.)*
- [x] **C15. `Std.IterStep.noConfusion` family wrongly rejected** (P0, found by
  the Init smoke at decl #1975) — `TypeMismatch` on a Pi with FVar-typed
  binders; cascades to the `.inj`/`.injEq` lemmas of the type.
  *(Wave 4, `2dd4c78`: `is_def_eq_under` now OPENS binders with fresh typed
  locals (Lean's `isDefEqBinding`) so every subterm stays locally closed; the
  Reducer mirrors the local context and K-like iota types FVar-mentioning
  majors wherever the redex sits (`try_k_whnf` can't reach buried redexes).
  All 172 Init rejections of this class now verify; also confirmed against
  the Wave-4 differential harness's 20 lean4lean disagreements — all
  *.noConfusion on multi-constructor inductives, all now verified.)*
- [x] **C16. Single-decl memory blow-up** (P0, Init decl #3865, misattributed
  to `…extract_append_extract._proof_1_1` — actually `Int.add_mul_ediv_right`;
  the Array decl is a `Lean.Syntax` unsupported-cascade).
  *(Wave 4, `bc8638f`: SAFETY — deterministic per-decl clone-fuel budget
  (`oxilean_kernel::fuel`, charged in `Expr::Clone`); exhaustion degrades to
  stuck/syntactic-only (never a wrong accept) and replays as the NAMED
  unsupported `RESOURCE_LIMIT`, never rejected, never OOM. CURE — whnf
  head-chains became ownership-moving loops, spines stay borrowed, caches
  skip >4096-node entries: offender RSS 2.93→1.27 GiB, closure wall
  12.1→8.5 s.)*
- [x] **C17. Deep reduction chains overflow the default 8 MiB stack** (P1) —
  *(Wave 4, `bc8638f`: oxilean-verify checks on a dedicated thread built with
  `std::thread::Builder::stack_size` (default 512 MiB reserved), new
  `--stack-size <MiB>` flag; spawn/join failures exit 2, never a verdict. No
  ulimit needed.)*
- [x] **C18. `is_leq` incomplete for param-dependent `imax` under a right
  `max`** (P0-class completeness, found by `prop_is_leq_complete` during the
  M2 full-corpus pass; pre-existing since C3, never a wrong accept).
  *(Wave 4, `220f52f`: the disjunctive max-right rule now case-splits on a
  zero-ness-critical parameter of any blocked imax before committing to a
  disjunct; terminating lexicographic measure; two directed regressions +
  checked-in proptest seeds; 50k-case stress green.)*
- [x] **C19. Structure-eta of stuck recursor majors missing (Lean's
  `toCtorWhenStruct`)** (P0, Init corpus roots `Nat.Linear.Poly.denote_reverse`
  and `Nat.Linear.ExprCnstr.denote_toNormPoly` + 37 cascades) — nested pair
  patterns (`| (k, v) :: p => …`) compile to an inner `Prod.casesOn` whose
  major is a bound variable; iota stayed stuck, TypeMismatch, wrong REJECT.
  *(Wave 4, `024399e`: `reduce::iota::to_ctor_when_struct` mirrors lean4lean
  exactly — structure-like inductives only, never on ctor applications, never
  on Prop-sorted types, guards fail safe-stuck; wired after literal expansion
  in `try_reduce_recursor`; three pinned regressions incl. Prop + wrong-field
  soundness controls; both corpus roots re-verified via closure replay.)*
- [x] **C16b. Fuel did not bound substitution-driven allocation; `infer_type`
  never observed the latch** (P0 — the C16 decl was NOT `Int.add_mul_ediv_right`
  after all: Init decl #3864 `…extract_append_extract._proof_1_1` is a
  `let`-tower whose value duplicates at every level; substitution rebuilds
  spine nodes without `Expr::clone`, so it allocated past 12 GiB with the
  budget long exhausted — the OOM that killed every full-corpus pass at
  exactly 3,864 decls and froze the machine twice on 2026-07-12).
  *(M2, `2f551d5`: the substitution/lift builders charge one fuel unit per
  visited node and `infer_type` aborts with a typed error once the latch is
  set (→ NAMED resource limit, never a rejection, terms never truncated).
  The offender degrades in 34 s under an 8 GiB cage; pinned 60-level
  exponential let-tower regression.)*
- [x] **C20. String literals used the pre-UTF-8 `String.mk (List Char)` model**
  (P0 wrong-REJECT class: `String.toByteArray_empty`, `String.ofList_nil`,
  `String.push_induction`, utf8Decode/EncodeChar lemma families). Lean
  v4.32's `String` constructor is `ofByteArray (bytes) (validity)`; the
  kernel expands literals to the *function* `String.ofList l` and WHNFs at
  every use site (iota major, `reduce_proj_core`, `try_string_lit_expansion`).
  *(M2, `6af1a90`: `reduce::iota::str_lit_expansion` mirrors v4.32 exactly,
  with the old one-field-constructor model kept for old-model envs (builtin),
  env-gated so a name collision can never bridge; def-eq fires exactly on
  `String.ofList`-headed applications with a re-entry guard; Proj-on-literal
  expands before projecting. Pinned both-orientation + mismatch controls.)*
- [x] **C21. whnf reduced application heads OUT of context, bypassing the
  literal extension** (P0 wrong-REJECT class: `UInt64.toUInt32_mul`,
  `Int64.toInt_minValue`, SInt/UInt lemma families, Omega-constraint-heavy
  utf8 proofs) — `HMod.hMod → … → Nat.mod` delta-unfolded the bare `Nat.mod`
  head before the extension could see `Nat.mod lit lit`, diverging into the
  unary structural-recursion body (Nat.below towers).
  *(M2, `8b57810`: the App arm steps the head one reduction at a time IN
  CONTEXT, re-trying extension → iota/quot → delta each iteration — Lean's
  whnf/unfold_definition order; `2^64 % 2^32` folds at ~0 fuel. Known
  residue: `UInt64.toUInt32_mul` needs 100.2 Mnodes and
  `Array.foldlM_toList.aux._unary` 506.8 Mnodes — above the 2^26 corpus
  budget, so they degrade to the NAMED resource limit; raising the budget to
  2^27 would let the C16b decl OOM again, so the preset stays.)*
- [x] **C22. Reader materialization budget was CUMULATIVE only — one
  declaration could legally OOM the process before the kernel ever ran**
  (found by the first official full-Init run, run7: stalled at decl #42,086
  `WellFounded.partialExtrinsicFix₃_eq_partialExtrinsicFix` against the
  16 GiB cage swap ceiling; in isolation the decl OOM-killed a 12 GiB cage
  even with kernel fuel at 2^23, and gdb sampling put the blowup inside
  `reader::materialize_expr/build_expr` — the theorem's DAG-shared
  type+value expand to >100 Mnodes ≈ 7+ GiB as an unshared `Box` tree).
  *(M2, `9b5b435`: `Limits::decl_materialize_budget` checked FIRST in
  `materialize_expr` — O(1) against the memoized size table, no expansion
  work; breach recovered by `dispatch` into the new
  `ExportDecl::Oversized { names, feature }` (names read from the interned
  table only), which the `Replayer` defers so dependents cascade as
  DEFERRED_DEPENDENCY — named unsupported, never a rejection, never an OOM.
  Corpus preset cap 2^25 nodes (≈2–3.4 GiB): full-Init calibration scan
  shows exactly 2 decls over cap (#18697, already a follower in run7, and
  #42,086 itself), max fitting decl 29.67 Mnodes, and the read-only corpus
  peak RSS drops 8.4 GiB → 3.79 GiB. Untrusted default unchanged (per-decl
  == cumulative bound); sharing bombs now degrade to a cheap per-decl named
  skip instead of a whole-file error, cumulative breach still hard-errors.
  Pinned: bomb recovery, cumulative-still-errors control, and the
  oversized→cascade regression in `replay_smoke`.)*

## P1 — The product (new crates & artifacts)

- [x] **V1. `oxilean-export` crate** — lean4export **NDJSON v3.1.0** reader
  (spec: `docs/specs/lean4export-format.md`, pinned commit `3de59f10`,
  Lean `v4.32.0-rc1`). Zero deps (hand-rolled minimal JSON parser),
  `#![forbid(unsafe_code)]`, streaming, index tables, `natVal` decimal-string →
  BigNat, exported recursor rules treated as informational (re-derive in kernel),
  errors split malformed-input vs unsupported-construct.
  *(Wave 3: reader + fuzzing landed (61 tests, sharing-bomb defenses,
  three-bucket errors). Wave 3b: FULL replay — empty-env true checker,
  inductive families installed via `check_constant_info` with exported
  recursors verified against the kernel re-derivation, quot records validated
  against `add_quot` primitives, `Quot.sound` checked against the canonical
  kernel type, honest Unsupported/Rejected cascades, streaming
  `replay_streaming` + `ReplayLimits::corpus()`; 74 tests.)*
- [x] **V2. `oxilean-verify` crate (CLI)** — three buckets **verified / unsupported
  (named reason) / rejected**; streaming per-decl output with timing; summary;
  exit codes 0 / 1 (rejected>0) / 2 (usage/io/malformed); `--json` report
  (tool version + lean4export commit + toolchain pins). Dependency closure:
  kernel + export ONLY.
  *(Wave 3b: landed — UI-free `verify_stream` engine + thin CLI, hand-rolled
  JSON writer + SHA-256, `--limits corpus`, 45 tests incl. determinism and
  exit-code controls. Per-fixture three-bucket splits pinned through the REAL
  binary, agreeing with oxilean-export's replay pins — all exit 0:*

  | fixture | verified | unsupported | rejected |
  |---|---:|---:|---:|
  | simple_add | 23 | 0 | 0 |
  | Nat.add_succ (3.0.0) | 19 | 0 | 0 |
  | Tree_Forest | 1 | 0 | 0 |
  | Tree_Forest_full | 37 | 0 | 0 |
  | point_swap_swap | 16 | 0 | 0 |
  | Parity.isEven | 181 | 0 | 0 |

  *Corrupted-proof control: simple_add with a swapped proof term → 22/0/1,
  REJECTED with a typed mismatch, exit 1. Init.ndjson smoke (release,
  `--limits corpus`, 1 GiB stack): 3,864/57,277 decls in ~136 s (≈28
  decls/s) — 3,296 verified · 396 unsupported (root: `Lean.Syntax` nested
  inductives, honest cascade) · 172 rejected (root: C15) — before the C16
  memory blow-up aborted the run. Wave-4 items C15–C17 own the rest.)*
- [x] **V3. `oxilean-verify-wasm`** — new wasm crate (kernel + export +
  verify(lib) + wasm-bindgen + js-sys ONLY; existing `oxilean-wasm` pulls
  parse/elab/serde and cannot meet budget). `cdylib`-only,
  `#![forbid(unsafe_code)]`, wasm-bindgen NON-optional (no feature gating — the
  0.1.2 DCE regression class is structurally impossible). Streaming JS API:
  `VerifySession::new(LimitsPreset)` → `push_bytes`/`push_chunk` (file slices) →
  `finish(on_decl, path)` streams a `DeclVerdict` per declaration (name / kind /
  verdict / detail / micros) and returns a `VerifySummary` (three buckets +
  deterministic `pins_json`). Client-side `sha256` getter ("0 bytes uploaded").
  npm name `@cooljapan/oxilean-verify` (set by `web/verify-demo/build.sh`; not
  published). Registered in workspace + `verify/allowed-deps.txt` (wasm-bindgen
  family closure, separated from the 3-crate TCB closure) + ci.yml gates.
  *(Wave 4: **built with wasm-pack (release, target web); wasm-opt -Oz ran**.
  Artifact: raw 353,957 B · **gzip 140,960 B (~138 KB)** — well under the
  400 KB budget. Finalize rebuild on the merged 0.1.3 tree (kernel
  completeness fixes on board): raw 371,592 B · **gzip 147,132 B (~144 KB)**,
  still 39 exports, still VALID. 39 exports (≥10, DCE guard PASS). `wasm-tools validate` VALID.
  4 native boundary tests pass. **A latent DCE-class bug was found and fixed
  during this wave**: `std::time::Instant::now()` panics on
  `wasm32-unknown-unknown` ("time not implemented"); because the engine takes a
  timestamp first, that panic made the ENTIRE kernel path dead code and the
  first build was a 22 KB stub that verified nothing. Fixed with
  `oxilean_kernel::wall_clock::Instant` — a transparent newtype over
  `std::time::Instant` off wasm (zero native behavior change; all 3,470 kernel
  tests green) and a non-panicking monotonic `AtomicU64` counter on wasm; the
  135 `std::time::Instant` callsites across kernel/export/verify were
  redirected mechanically. Zero external deps preserved; forbid(unsafe) intact.)*
- [x] **V4. Demo "Kernel in a Tab"** — static page (`web/verify-demo/`:
  `index.html`, `main.js`, `style.css`, built `pkg/`, `sample/simple_add.ndjson`,
  `build.sh`). One drop zone; badge line
  `kernel: 144 KB wasm · 0 external dependencies · 0 unsafe · 0 bytes uploaded`
  (the KB is written at build/copy time by `build.sh` from the measured gzipped
  wasm — 144 KB after the finalize rebuild on the merged 0.1.3 tree). On drop of a `.ndjson` export: declarations stream in live one line each
  (✓ name ms / ⊘ name unsupported: feature / ✗ name reason), summary
  `N verified · U unsupported · R rejected` with the rejected count visibly
  standing out when non-zero (the alarm), then the closing line
  *"Nothing left your machine. This kernel has never seen Lean's source code."*
  Plain module script + `requestAnimationFrame` batching keeps the UI live; no
  editor/REPL/tactics (brief §6.1 non-goals honored). No CDN/framework/worker;
  runs under `python3 -m http.server` (no COOP/COEP/SharedArrayBuffer) — proven
  by `scripts/gate-demo-smoke.sh`.
- [x] **V5. Real corpus** — install elan + Lean v4.32.0-rc1 + lean4export; export
  Lean core; keep corpus out-of-repo (`~/work/oxilean-corpus/`); small fixture(s)
  committed under `tests/fixtures/`.
  *(Corpus present: `~/work/oxilean-corpus/Init.ndjson` (57,277 decls) + 9
  small exports; six fixtures committed under `tests/fixtures/lean4export/`.
  **M2 FIRST REAL NUMBER (2026-07-15, official full run at `9b5b435`):
  35,223 verified · 22,054 unsupported · 0 rejected over all 57,277 Init
  declarations** — wall 1:20:44, peak RSS 9.98 GiB inside the 12G/10G-high/
  16G-swap systemd cage on the 14 GiB machine, exit 0. Unsupported = 178
  named roots (1 nested-inductive `Lean.Syntax`, 175 clone-fuel, 2 C22
  oversized) + 21,876 followers, all attributed (Syntax cascade alone covers
  98.9%). Full report: `docs/reports/2026-07-15-lean-core-init.md` (+ JSON).
  Throughput 11.8 decls/s — ~55× under the brief's 5×-of-lean4lean budget;
  documented honestly, root cause = unshared Box `Expr` (structural sharing
  is the tracked fix, same as the memory wall). Remaining follow-up: script
  the re-export pipeline for reproducibility.)*
- [x] **V6. Differential harness** — corpus runner comparing our verdicts vs
  lean4lean; disagreement report. (Skeleton + docs first; real runs need corpus.)
  *(Wave 4 (`verify/differential/`): run_oxilean.sh / run_lean4lean.sh /
  diff_verdicts.py + committed 2,000-decl Init slice + smoke test. REAL RUNS
  DONE on `Init.Prelude`: the harness found **20 genuine disagreements** (all
  oxilean false-rejections of multi-constructor `*.noConfusion` — the C15
  root), listed in full, never reclassified. Post-C15 re-run (2026-07-15,
  0.1.3): **0 disagreements** across 1,987 joined decls; evidence committed
  under `artifacts/` (pre- and post-fix). Caveats documented: lean4lean
  master = v4.29.0 vs our v4.32.0-rc1 (set skew → ONLY-ONE-SIDE) and its
  default replay path segfaults, so the harness drives `--fresh`. See
  `verify/differential/RESULTS-2026-07-12.md`.)*
- [x] **V7. Throughput benchmark** — decls/sec on export corpora; measure lean4lean
  baseline before setting target (brief §8.2: within 5×).
  *(Wave 4: `scripts/bench-verify.sh` (3 runs, median, machine specs printed).
  Baseline MEASURED: lean4lean ~3,280 decls/s on `Init.Prelude` → budget line
  ~656 decls/s. Ours (0.1.3, post-iterative-whnf): **~398 decls/s** on the
  same `Init.Prelude` set (~8.2× slower, improved from ~8.7×/376 pre-fix),
  ~40 decls/s on the heavier init-2000 slice, 11.8 decls/s full-corpus.
  **Budget NOT met** — honest number, recorded next to the baseline in
  `RESULTS-2026-07-12.md` §post-fix; root cause = unshared `Box` `Expr`,
  fix (structural sharing) tracked, not chased with constant-factor hacks.)*

## P1 — Gates & CI (brief §8)

- [x] **G1. Re-enable CI** — `workflows.disabled/ci.yml` → active, modernized
  (check / nextest / clippy `-D warnings`).
  *(Wave 1: `.github/workflows/ci.yml` live — check / nextest / clippy `-D
  warnings` / fmt `--check` / TCB-gates jobs; bench.yml re-enabled too.)*
- [x] **G2. Zero-dep invariant gate** — `cargo tree -p oxilean-kernel` (and
  `-p oxilean-export`) shows zero external crates.
  *(Wave 1: `scripts/gate-zero-deps.sh oxilean-kernel` in CI and passing.
  Wave 3b: CI invocation extended to
  `oxilean-kernel oxilean-export oxilean-verify`; workspace-internal path
  deps allowed, any crates.io dep at any depth fails.)*
- [x] **G3. Dependency allow-list** — `verify/allowed-deps.txt` (kernel + export +
  wasm-bindgen); CI compares against `cargo tree` of verify product.
  *(Wave 3b: `scripts/gate-allow-list.sh` validates the FULL transitive
  runtime closure of kernel + export + verify (root crates included) against
  `verify/allowed-deps.txt` (now: the three crates + wasm-bindgen reserved
  for the V3 shim); wired into ci.yml's tcb-gates job. Re-run for
  oxilean-verify-wasm when V3 creates it.)*
- [x] **G4. forbid(unsafe_code) gate** — present in kernel ✓ / export ✓ /
  verify ✓ / verify-wasm ✓.
  *(Wave 1: parse `unsafe { ptr::read }` removed (safe retain-based eviction);
  forbid added to parse/build/codegen/lint, umbrella crate deny→forbid.
  Wave 3b: CI gate ran over kernel + export + verify. Wave 4: extended to
  `oxilean-verify-wasm` — `#![forbid(unsafe_code)]` present and gated in
  ci.yml `tcb-gates` over all four crates.)*
- [x] **G5. WASM export-count gate** — `wasm-tools`/`wasm-objdump` based; fails if
  exports collapse (the 0.1.2 regression). Root cause: `npm-publish.yml` builds
  without `--features wasm` → fix the workflow too.
  *(Wave 1: `web/scripts/export-gate.sh` (wasm-tools → wasm-objdump → python3
  fallback) gates all three wasm-pack targets in `npm-publish.yml`, which now
  builds with `--features wasm`; verified locally: 23 exports ≥ 10. Wave 4: the
  same gate now guards `oxilean-verify-wasm` in ci.yml `wasm-demo-gates` — 39
  exports ≥ 10; the DCE-stub failure mode was actually triggered and fixed this
  wave (see V3: the `std::time::Instant` wasm panic collapsed the kernel to a
  22 KB stub before the fix).)*
- [x] **G6. WASM size budget gate** — ≤ 400 KB gzip for verify wasm.
  *(Wave 4: `web/scripts/size-gate.sh <wasm> [budget-kb]` gzips the artifact and
  fails over budget. Current (finalize rebuild, merged 0.1.3 tree):
  **147,132 B gzip (~144 KB)** vs 400 KB budget → PASS with ~64% headroom.
  Wired into ci.yml `wasm-demo-gates`. Honest number, no functionality
  stripped to hit it — the crate links kernel+export+verify in full.)*
- [~] **G7. Determinism gate** — identical verdicts/report native vs WASM on
  fixture corpus.
  *(Wave 4: `scripts/gate-determinism.sh` — **native half DONE**: runs the real
  release binary on `simple_add.ndjson` twice with timing masked, asserts the
  two JSON reports are byte-identical AND the three-bucket totals match the
  pinned 23/0/0. Wired into ci.yml. **WASM half DEFERRED to Wave 4/future**: no
  node/deno/headless browser in this environment, so the wasm report cannot be
  produced and diffed headlessly here. Mitigations in place: the wasm crate
  drives the IDENTICAL `verify_stream` + `render_report` path as native (proven
  by shared code + 4 native boundary tests), and the module is validated
  structurally (`wasm-tools validate` + kernel-symbol presence). Honest gap.)*
- [~] **G8. Demo smoke test** — serve via `python3 -m http.server`, headless
  load check.
  *(Wave 4: `scripts/gate-demo-smoke.sh` — starts `python3 -m http.server` on a
  random port over `web/verify-demo`, curls index.html + main.js + style.css +
  pkg/oxilean_verify.js + the .wasm, asserts HTTP 200 + non-empty + byte size
  matches on-disk, kills the server. PASSES. Wired into ci.yml `wasm-demo-gates`
  (G8 partial). A true headless-browser drive of the JS (drop→verdicts→summary)
  is DEFERRED — no headless Chrome/node here; the engine behavior is validated
  natively (`gate-determinism.sh`, the fixture table) instead.)*
- [x] **G9. Fuzzing** — `cargo-fuzz` targets: export reader (untrusted input → TCB),
  BigNat (differential vs reference); CI job (nightly toolchain).
  *(Wave 4 (`wave4/fuzz-ci`, merged): three targets — `fuzz_ndjson_parse`
  (bytes → reader), `fuzz_bignat` (differential vs reference ops), and
  `fuzz_replay` (bytes → reader → KERNEL, the strongest) — with seed corpora;
  `.github/workflows/fuzz.yml` weekly cron + PR-paths + manual dispatch.
  ~850K accumulated local execs, zero findings. Finalize pass: targets updated
  for the C16/C22 limit fields (`per_decl_fuel`, `decl_materialize_budget` —
  tight 2^20 presets) and re-smoked clean.)*
- [x] **G10. Dead workspace tests** — `tests/cli_test.rs` (68) and `tests/perf_test.rs`
  (26) exist but are not registered as `[[test]]` in root Cargo.toml. Register & fix.
  *(Wave 1: both registered as `[[test]]`; 94/94 pass post-BigNat merge.)*

## P2 — Release & docs (0.1.3)

- [x] **R1. Version bump 0.1.3** — workspace Cargo.toml + internal dep pins + doc
  comments + wasm package.json.
  *(Wave 4 (`wave4/release-polish`, merged): workspace + all internal pins at
  0.1.3; wasm package.json name/version via build.sh; verified by the finalize
  quality loop (`cargo tree -p oxilean-verify` resolves 0.1.3 internally).)*
- [x] **R2. CHANGELOG** — `[Unreleased]` is stale (v0.1.0-era); write real 0.1.3 notes.
  *(Wave 4: full `[0.1.3]` section (Added/Fixed/Changed/Security). Finalize:
  corpus-numbers marker filled with the REAL Init numbers (35,223/22,054/0)
  and the 20→0 differential result; wasm size corrected to the rebuilt
  artifact (144 KB gzip); dated 2026-07-15.)*
- [x] **R3. publish.sh** — missing `oxilake`, `oxilean-doc`; add `oxilean-export`,
  `oxilean-verify` tiers.
  *(Wave 4: tiered publish order incl. export/verify/verify-wasm, oxilake,
  oxilean-doc; NEVER auto-published — script still requires explicit operator
  action.)*
- [x] **R4. README** — crate count stale (12 → 14+2); add verify product section with
  three-bucket language; keep "parse" welded to the 99.7% figure (verified OK today).
  *(Finalize 2026-07-15: full product section added — what it is, TCB =
  kernel+export, zero deps + forbid(unsafe), three-bucket/exit-code table,
  the REAL Init numbers with their qualifiers welded on (single-root cascade,
  what "0 rejected" does and does not mean, honest throughput), Kernel-in-a-Tab
  demo instructions, differential-harness pointer with lean4lean caveats, all
  pins. 99.7% figure re-verified as parse-only and now cross-references the
  verify numbers explicitly so the two cannot be conflated. Crate table
  refreshed: 17 crates, per-crate SLOC/tests, 33,231 passing.)*
- [x] **R5. TODO.md corrections** — it claims struct-eta and K-reduction complete
  (false), references non-existent `src/quot.rs`, wrong quotient primitive list.
  Fix false claims explicitly.
  *(Wave 4: false claims corrected in place; verify-campaign section added.
  Finalize: wasm size updated to the rebuilt 144 KB artifact.)*
- [x] **R6. docs/VERIFY.md** — three buckets, exit codes, JSON schema, pins
  (lean4export commit + Lean toolchain), published `unsupported` list w/ reasons.
  *(Wave 3b: landed with V2 (buckets, exit codes, schema, pins, transcript).
  The published `unsupported` list with reasons now exists as the corpus
  report's named-bucket table — 178 roots across 3 named features with root
  causes and exact follower attribution
  (`docs/reports/2026-07-15-lean-core-init.md`) plus the per-run
  `unsupported_features` array in every JSON report.)*
- [x] **R7. LICENSE name consistency** — "KitaSan" vs "Kitasan".
  *(Wave 4: unified to "COOLJAPAN OU (Team Kitasan)" across LICENSE and crate
  metadata.)*

## P3 — Deliberate follow-ups (out of this pass, tracked)

- [>] **F1. TCB slimming** — kernel is 146,570 raw lines; SplitRS filler types
  (TokenBucket, SimpleDag, StringPool…) duplicated across modules, and non-TCB
  modules (simp, typeclasses, match_compile, ffi, abstract_interp, unif_hints,
  congruence, termination) are publicly exposed. Quarantine toy `quotient/`
  re-exports now (part of C12); full de-bloat is a separate reviewed pass —
  the "auditable by eye" story depends on it.
- [>] **F2. MSRV verification** — `rust-version = "1.70"` never tested; add CI job or
  bump honestly.
- [>] **F3. Legacy text-line export format** — current upstream lean4export emits
  NDJSON v3; old space-delimited format (trepplein-era) optional for
  cross-checking against older tools.
- [>] **F4. Zulip announcement** — brief §10; only after M4-style numbers exist.

---

## Execution waves (this campaign)

| Wave | Contents | Mode |
|---|---|---|
| **1** | S1/S10/C5(args)/C8 (BigNat + literal overhaul) · G5 wasm fix · G4 parse unsafe · G1/G2/G10 CI basics | bignum on live tree; rest in worktrees |
| **2** | Quotients (S2/S3/S9/C4/C5/C12) · Struct eta (C1/C2/S11) · Recursors (S2–S6/C6/C7/C9/C10/C11) · Levels (C3/S7/S8) | 4 isolated worktrees → integration merge |
| **3** | V1 export reader → V2 CLI → V3 wasm · V4 demo · G9 fuzz · V6 harness | pipeline + parallel |
| **4** | Corpus verification loop (V5/V7) · remaining gates (G3/G6/G7/G8) · docs/release (R1–R7) | parallel + final quality loop |

*Last updated: 2026-07-15 (campaign close — see "Campaign result" at the top.
Wave-4 branches merged into 0.1.3: `wave4/fuzz-ci` (G9), `wave4/diff-harness`
(V6/V7), `wave4/release-polish` (R1/R2/R3/R5/R7). Full Init corpus verified:
**35,223 / 22,054 / 0 over 57,277 decls, exit 0**
(`docs/reports/2026-07-15-lean-core-init.md`). Differential re-run post-C15:
**20 → 0 disagreements** vs lean4lean on `Init.Prelude`. Post-fix throughput
recorded next to the baseline (~398 vs ~3,280 decls/s — 5× budget NOT met,
root cause tracked). Wasm rebuilt on the merged tree: 147,132 B gzip (~144 KB),
39 exports. README product section (R4) + CHANGELOG corpus numbers landed.
Final quality loop green: 33,231 tests passed / 746 ignored, clippy
`-D warnings` silent, fmt clean, TCB doc-tests pass, all gates pass, all three
fuzz targets build + smoke clean, `cargo tree -p oxilean-verify` = internal
0.1.3 pins only. Open: structural sharing (throughput/memory), nested
inductives (C10), G7 wasm half, F1–F4.) Owner: Team Kitasan.*

---

## wave5 — structural sharing DONE + M4 (Mathlib) scouting (2026-07-16)

- [x] **Structural sharing of the kernel `Expr`** — `Box<Expr>` → `Rc<Expr>` →
  cached-header `Node` edges (`Node { range, cost, rc }`, transparent `Deref`) +
  materialise-once export reader. Untouched-subtree substitution skip is
  **fuel-exact** (charges the same fuel a rebuild would), so verdicts are a
  strict superset of the pre-sharing run. **Full-`Init`: wall 1 h 20 m 44 s →
  11 m 28 s (≈7×), peak RSS 9.98 GiB + ~6.7 GiB swap → ≈6 GiB no swap,
  35,424 verified / 21,853 unsupported / 0 rejected.** (An earlier `HashMap`
  attempt — the reverted "Stage D" — failed the caged gate with 1 false
  rejection + a peak-RSS regression; the lesson, pinned by a differential fuel
  test, is that the skip MUST preserve fuel exactly.) Commits `bc0bb50`,
  `6b75000`, `4996a12`, `bb59431`.
- [~] **M4 Mathlib — FEASIBLE, ran to 183,624 / 682,271 (27%), two blockers
  remain:**
  - [x] CLI now streams the input file (`b92faf7`) — a ~6 GB export no longer
    sits in RAM alongside the environment.
  - [x] Per-declaration wall-clock deadline (`db509da`) + `is_leq_core` guard
    (`b904017`) — bounds reduction/def-eq loops the deterministic fuel can't see
    (e.g. exponential `imax` case-split); over-time decls become named
    resource-limits, never hangs.
  - [ ] **Stage F — Name/Level interning.** RSS climbed 5.5 G→8.9 G→11.7 G by
    183 k decls (4.09 M un-interned names + the growing env hit the 12 G cage);
    the full 682 k needs interning (or a bigger machine / chunked run).
  - [ ] **More deadline guards (hang whack-a-mole).** decl ~183,625 (another
    `CategoryTheory` decl) hangs in yet another non-fuel loop; needs the same
    `deadline::is_expired()` treatment as `is_leq_core`.
  - [ ] **Fix the 2 FreeGroup false rejections** (`FreeGroup.instGroup._proof_12`,
    `FreeGroup.induction_on`). DIAGNOSED as a def-eq **completeness** gap (NOT
    unsoundness): `@rfl (1⁻¹ * 1)` proves the group law `1⁻¹ * 1 = 1`, definitional
    in FreeGroup (a `Quot` of lists) but the kernel fails to reduce the nested
    `HMul`/`Inv` instance-unfolding + `Quot.lift` on the identity. Minimal
    reproducer (0.43 s / 11.7 MB): `~/work/oxilean-corpus/FreeGroup_rejections_repro.ndjson`.
    Fix = complete that reduction path; do NOT blanket-reclassify def-eq
    failures as unsupported (that would hide genuine corrupted-proof rejections).
