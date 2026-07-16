# Audit: Definitional eta for structures in oxilean-kernel

Date: 2026-07-12. Repo: oxilean (branch 0.1.3, clean).
Area: brief section 4 hard-part (2) — definitional eta for structures — plus the surrounding
def_eq / whnf / infer machinery it must be wired into.

**Headline verdict: struct eta is a dead, unwired, partially-buggy utility module. It is NOT
in the definitional-equality path at all. TODO.md claims it is complete. This is exactly the
"partial that fails late and confusingly" the brief warns about — every real lean4export file
containing a structure eta obligation will be REJECTED (false alarm) by the current kernel.**

Empirically confirmed with a probe binary (source at
`<scratchpad>/eta_probe/`):

```
STRUCT ETA  e == MyPair.mk e.0 e.1        : false   <- Lean requires true
STRUCT ETA  MyPair.mk e.0 e.1 == e (sym)  : false   <- Lean requires true
PROJ-CTOR   (MyPair.mk e e).0 == e        : true    <- ok (iota/proj reduction works)
FUN ETA     (fun x => f x) == f           : true    <- ok (single binder)
FUN ETA sym f == (fun x => f x)           : true    <- ok
FUN ETA 2   (fun x y => g x y) == g       : false   <- Lean requires true (multi-binder)
UNIT-LIKE   u1 == Unit.unit               : false   <- Lean requires true (isDefEqUnitLike)
TC PATH     e == MyPair.mk e.0 e.1        : false   <- same failure via TypeChecker
```

---

## 1. Call graph from the real is_def_eq entry point

The declaration-checking pipeline (what a verifier would drive):

```
check::functions::check_declaration            check/functions.rs:25
  -> TypeChecker::check_type                   infer/types.rs:873
    -> TypeChecker::is_def_eq                  infer/types.rs:868
      -> DefEqChecker::is_def_eq               def_eq/types.rs:1291
        - syntactic == fast path
        - EquivManager::is_equiv / is_failure  equiv_manager/types.rs:838/857 (union-find cache,
          also caches FAILURES per checker instance)
        - HashMap<(Expr,Expr),bool> cache
        -> is_def_eq_core                      def_eq/types.rs:1314
          - whnf both sides: reduce::Reducer::whnf_env  reduce/types.rs:1069
          - is_proof_irrelevant_eq             def_eq/types.rs:1511
          - structural match                   def_eq/types.rs:1323-1356
            (Sort/BVar/FVar/Const/App/Lam/Pi/Let/Lit/Proj-vs-Proj)
          - (Lam, non-Lam) -> try_eta_lhs      def_eq/types.rs:1354/1546
          - (non-Lam, Lam) -> try_eta_rhs      def_eq/types.rs:1355/1561
          - _ -> try_lazy_delta                def_eq/types.rs:1356/1363
```

**There is no struct-eta anywhere in this graph.** Grep evidence:

- `struct_eta` referenced outside its own directory only at `lib.rs:326` (`pub mod struct_eta;`).
  There is not even a `pub use struct_eta::...` re-export in lib.rs.
- `StructureEta`, `eta_expand_struct`, `SingletonKReducer`, `k_reduce` have ZERO callers
  outside `src/struct_eta/` (aside from that module's own unit tests). The only other repo
  hits are in `oxilean-elab/src/structure/{types,functions}.rs` (elaborator, outside the TCB
  and outside the def-eq path).
- The (Proj, non-Proj) / (non-ctor, ctor-app) mismatch cases in `is_def_eq_core` fall through
  to `try_lazy_delta` (def_eq/types.rs:1363), which only unfolds definitions with
  reducibility hints; constructors report `ReducibilityHint::Opaque`
  (declaration/types.rs:709-717), so `get_hint` returns None for both sides -> `false`.
  This is the exact mechanism by which `e == S.mk e.0 e.1` fails.

Two more def-eq implementations exist, neither with any eta:

- `ConversionChecker::is_convertible_in_env` (conversion/types.rs:348-501): WHNF + pure
  structural comparison. No function eta, no struct eta, no proof irrelevance. Exported at
  lib.rs:361 — a second public "def eq" that disagrees with `DefEqChecker` on eta pairs.
- `def_eq::syntactic_eq` (def_eq/functions.rs:130): explicitly syntactic, fine.

Verdict Q1: **struct_eta/ is a standalone normalization utility that def_eq never calls.**

## 2. Ordinary function eta — partially implemented

Location: `DefEqChecker::try_eta_lhs` / `try_eta_rhs`, def_eq/types.rs:1543-1574, dispatched
from is_def_eq_core:1354-1355 only when, after whnf, exactly one side is a `Lam`.

Mechanism: **eta-contraction**, not Lean's eta-expansion. For `Lam(_, _, _, body)` with
`body = App(f, BVar(0))` and `!has_loose_bvar(f, 0)` (expr_util/functions.rs:103 — correct,
binder-depth-aware), it computes `instantiate(f, FVar(u64::MAX))` (which merely shifts loose
bvars down by one, since BVar 0 does not occur) and recurses on `f' =?= s`.

Works (verified): `(fun x => f x) == f` in both directions; also when `f` is itself an
application head (`fun x => Nat.add 1 x` style).

Fails (verified): `(fun x y => g x y) == g` — the outer lambda's body is a `Lam`, not an
`App`, so contraction bails and returns false. Lean's kernel handles this because it
eta-EXPANDS the non-lambda side (`etaExpand`: build `fun x => s x` from the Pi domain of s's
type) and recurses under the binder. Any curried multi-argument eta obligation — extremely
common in mathlib exports — is rejected.

Also note: `DefEqConfig { eta: bool, ... }` (def_eq/types.rs:765-776) is decorative.
`DefEqChecker` has fields `{env, reducer, cache, equiv_manager, proof_irrelevance}`
(def_eq/types.rs:1262-1268) and never reads a `DefEqConfig`. The `eta` flag is only touched
by its own default-value tests (def_eq/functions.rs:245-255).

Verdict Q2: **present but contraction-only and single-binder-only; multi-binder eta fails.**

## 3. "Is a structure" decision — predicate correct, applied in the wrong (or no) places

- `ConstantInfo::is_structure_like` (declaration/types.rs:731-736):
  `ctors.len() == 1 && !v.is_rec && v.num_indices == 0` — **matches Lean's
  `InductiveVal.isStructureLike` exactly** (1 ctor, no indices, not recursive).
- `Environment::is_structure_like` (env/types.rs:309-312) delegates to it.
- BUT the only consumers are the dead `struct_eta` module and `declaration` unit tests.
- `TypeChecker::infer_proj` (infer/types.rs:984-1016) does NOT use it: it only checks
  `ind_val.ctors.len() == 1` and `idx < ctor_val.num_fields`. It does **not** check
  `num_indices == 0` nor `!is_rec`. Lean's kernel `inferProj` rejects projections on indexed
  inductives; oxilean will happily type a `Proj` on a single-ctor indexed family, which is a
  soundness laxity in the TCB (accepting terms Lean's kernel rejects).
- `SingletonKReducer::is_singleton_type` (struct_eta/types.rs:140-157) checks
  `ctors.len() == 1 && num_fields == 0` but does NOT check `num_indices == 0` / `!is_rec`
  — diverges from Lean's `isDefEqUnitLike` gating (which requires structure-like). It would
  treat e.g. a single-ctor 0-field indexed Prop family (like `Eq` seen through the right
  lens: `Eq` has indices) as "singleton". Currently harmless only because it is dead code.
- `StructureEta::is_structure_type` (struct_eta/types.rs:873-884) correctly delegates to
  `env.is_structure_like` on the head const.

Verdict Q3: **the predicate itself is correct vs Lean, but it is not enforced where it
matters (infer_proj) and only referenced from dead code.**

## 4. Unit-like eta (Lean's isDefEqUnitLike) — absent from def_eq; dead stub exists

- No `unit_like`/`UnitLike` handling anywhere in src (grep: zero hits).
- `SingletonKReducer` (struct_eta/types.rs:130-193) is the closest artifact and is dead code.
  Its `k_reduce` (types.rs:177-184) returns `Expr::Const(ctor_name, vec![])`:
  - drops universe levels (empty `vec![]`),
  - never applies inductive parameters (wrong for any parameterized unit-like type, e.g.
    a `PLift`-like structure) ,
  - ignores the `_expr` argument entirely.
- `SingletonKReducer::apply_k_reduction` (types.rs:190-193) is a **meaningless stub**: it
  returns `Some(App(motive, proof))` unconditionally — not a K-reduction of anything.
- Empirically `u1 == Unit.unit` (u1 an axiom of type Unit, 1 ctor / 0 fields) is `false`
  under `DefEqChecker`; Lean's kernel proves it via isDefEqUnitLike (types def-eq + ctor has
  no fields). Note proof irrelevance does not rescue this because Unit : Type, not Prop.

Verdict Q4: **missing.**

## 5. Projection handling in whnf / infer

WHNF (the one that matters — `reduce::Reducer::whnf_env`, used by DefEqChecker and
TypeChecker):

- `whnf_core` Proj case (reduce/types.rs:1146-1152): whnf the struct expr, then
  `try_reduce_proj` (reduce/functions.rs:328-352): if the whnf'd operand is an application of
  a `Const` that is a constructor of `struct_name`, return arg at `num_params + field_idx`.
  Correct, including parameter skipping. Verified: `(MyPair.mk e e).0 == e` is true.
- Proj over an **eta-expanded form** `Proj(S, i, S.mk e.0 ... e.(n-1))` reduces fine (the
  eta-expanded form *is* a ctor app), collapsing to `Proj(S, i, e)` per field. So the
  ctor-app direction works; only the bare-`e`-vs-expansion direction (the actual eta rule)
  is missing.
- Gap: no String-literal handling. Lean's kernel converts `Expr.lit (.strVal s)` to
  `String.mk (List Char ...)` before proj reduction (`reduce_proj` in the C++ kernel and
  lean4lean). `try_reduce_proj` bails on `Lit`, so `("ab" : String).data`-style obligations
  get stuck. (Nat literals: Lean also converts big Nat literals to ctor form for iota;
  separate area, but same family of gaps.)
- Cosmetic-but-real: the other two Reducer variants `whnf` (reduce/types.rs:997-1000) and
  `whnf_delta` (1050-1053) never reduce Proj at all — they only whnf the operand and rebuild.
  Only `whnf_env` reduces projections. The crate-level re-export `whnf::whnf`
  (whnf/functions.rs:17) therefore does not reduce Proj-of-ctor.

Type inference:

- `Expr::Proj` -> `TypeChecker::infer_proj` (infer/types.rs:937, 984-1016) ->
  `infer_proj_field_type` (1025-1068): instantiates universe params from the head of the
  whnf'd struct type, feeds inductive params from the struct type's arguments, substitutes
  earlier fields with `Proj(S, j, struct_expr)` (the right dependent-field treatment), and
  returns the idx-th Pi domain. Structurally faithful to Lean.
- Bugs/laxities in infer_proj_field_type:
  - On telescope mismatch it silently `return struct_ty.clone()` (lines 1048, 1061, 1066)
    instead of erroring — a wrong type is fabricated and checking continues; fails far away.
  - Missing struct args are papered over with `Expr::BVar(0)` (line 1045).
  - As noted in §3, no `num_indices == 0` / structure-like enforcement.

Verdict Q5: **Proj-of-ctor works in whnf_env and infer_proj is mostly right; missing
String-literal expansion, missing index/rec checks, and it fails soft instead of hard.**

## 6. Test coverage for struct eta *in def_eq* — none

- def_eq module tests (def_eq/functions.rs:23-127, 178-435): reflexivity, beta, delta,
  lazy-delta, level equivalence, app/pi congruence, proof irrelevance. **Zero eta tests of
  any kind** — not even for the function eta that IS implemented.
- Integration tests: `crates/oxilean-kernel/tests/prop_tests.rs` — reflexivity/symmetry
  properties for `is_def_eq_simple`; no eta, no environment-backed structures.
- struct_eta module has 42 `#[test]`s in functions.rs, but they only exercise the standalone
  helpers: `is_structure_type`, `collect_field_types`, `make_proj_chain`,
  `eta_expand_struct` shape (asserts it's an `App`, nothing about def-eq!), `is_singleton_type`,
  `k_reduce`. The majority test SplitRS filler types (StructureRegistry, EtaStats,
  EtaStateMachine, EtaGraph, KReductionTable...) that model nothing in the kernel.
- eta/ module has 73 tests, all against its own standalone `eta_contract`/`eta_normalize`
  helpers (eta/functions.rs:39-117) — also never called by def_eq.
- `crates/oxilean-kernel/TODO.md` "Future Optimizations" marks
  `[x] η-expansion for structures` and `[x] K-like reduction for singleton types` as DONE.
  Both are dead code. The tracking is actively misleading.

Test gaps to add (all at the DefEqChecker/TypeChecker level, not module level):
1. `e == S.mk e.0 e.1` and symmetric, for a 0-param structure (currently false).
2. Same with inductive params (`Prod α β`) and universe polymorphism (`Prod.{u,v}`) —
   would also catch the eta_expand_struct level/param bugs (see §7).
3. Dependent structure (`Sigma`): field 1's type mentions field 0 — exercises
   `infer_proj_field_type`'s Proj substitution together with eta.
4. Nested eta: `p == Prod.mk (Prod.mk p.0.0 p.0.1) p.1` under one recursion.
5. Unit-like: `u1 == u2` for two axioms of a 1-ctor/0-field type; and parameterized
   unit-like.
6. Negative: eta must NOT apply to 2-ctor types, recursive single-ctor types (e.g. a
   `Stream`-like), or indexed families — guards the is_structure_like gate.
7. Multi-binder function eta `(fun x y => g x y) == g` (currently false).
8. Proj over eta-expansion round trip and Proj over String literal (if String support in scope).

## 7. The dead struct_eta utility itself has bugs (must be fixed or discarded before wiring)

`StructureEta::eta_expand_struct` (struct_eta/types.rs:941-960):
- `let ctor_levels: Vec<Level> = vec![];` — universe levels of the structure are dropped;
  the built ctor `Expr::Const(ctor_name, [])` is level-incorrect for any polymorphic
  structure (i.e., nearly all real ones: Prod, Sigma, Subtype...).
- Inductive **parameters are never applied**: for `S.mk : (α : ...) -> fields... -> S α`,
  the expansion produced is `S.mk e.0 ... e.(n-1)` with the params missing, which is
  ill-typed. Correct form is `S.mk p_1 ... p_k e.0 ... e.(n-1)` with params read off the
  whnf'd type of `e`.
- It never verifies `expr`'s type actually IS the structure type — the caller passes `ty`
  in, fine for a utility, but the def-eq integration needs infer+whnf of the *other* side.

`collect_field_types` (types.rs:894-928) returns Pi domains that still contain loose BVars
referring to params/earlier fields (un-instantiated) — usable only for arity counting.

Also note the module (like def_eq, eta, whnf, equiv_manager, conversion, infer) carries a
large volume of SplitRS "padding" (StringTrie, TokenBucket, Levenshtein, Stopwatch,
DecisionNode, duplicated per module) inside the supposed minimal-TCB kernel crate. For the
brief's TCB goal this is a real liability: ~146k SLoC in oxilean-kernel/src, of which the
def-eq-relevant logic is a few hundred lines.

## 8. How Lean does it (implementation target, cf. lean4lean / kernel C++)

`isDefEqCore` order: quick structural -> whnfCore -> proofIrrel -> lazy delta ->
constructor/level/etc. -> **then**:
- `tryEtaExpansionCore t s` both directions (function eta by EXPANSION: if t is Lam and s
  isn't, infer s's type, whnf to Pi, build `fun x => s x`, recurse),
- `tryEtaStructCore t s` both directions: if s (after whnf) is `S.mk ps fs`, with
  `env.isStructureLike S`, and `isDefEq (infer t) (infer s)`, then check
  `isDefEq (Proj S i t) (f_i)` for each field argument (params come from the type, checked
  via the type def-eq),
- `isDefEqUnitLike t s`: whnf(infer t) head is structure-like S whose single ctor has 0
  fields -> equal iff `isDefEq (infer t) (infer s)`.

Note Lean's tryEtaStruct compares `Proj S i t` against the ctor's field args — it does NOT
need to synthesize params for the ctor; oxilean should do the same rather than repairing
`eta_expand_struct`'s missing-params construction.

## 9. Concrete recommendations (ordered)

1. **Wire struct eta into DefEqChecker::is_def_eq_core** (def_eq/types.rs:1314): after the
   structural match and before `try_lazy_delta` fails, add `try_eta_struct(t, s)` +
   symmetric: if s's whnf is an application whose head Const is a constructor `c` with
   `env.get_constructor_val(c)` and `env.is_structure_like(cv.induct)` and
   `args.len() == cv.num_params + cv.num_fields`, then require the *types* def-eq (needs a
   type-infer hook — see 3) and check `is_def_eq(Proj(S, i, t), args[num_params + i])` for
   all i. This reuses the already-working Proj machinery and avoids the level/param bugs of
   eta_expand_struct.
2. **Add isDefEqUnitLike** in the same place for 1-ctor/0-field structure-like types (types
   must be def-eq; use structure-like gate, not the looser is_singleton_type).
3. **Break the def-eq/infer layering knot**: DefEqChecker cannot infer types today (its
   `quick_infer_type`, def_eq/types.rs:1454-1505, returns None for FVar/BVar and Proj!).
   Either move is_def_eq into TypeChecker (Lean's architecture: one type_checker owning
   whnf+infer+defeq) or pass an infer callback. quick_infer_type's Proj omission also means
   proof irrelevance never fires on projected proofs — same root cause.
4. **Replace contraction-only function eta with expansion-based eta** to fix multi-binder
   cases (def_eq/types.rs:1543-1574).
5. **Tighten infer_proj**: require `env.is_structure_like(struct_name)` (adds
   num_indices==0, !is_rec), and turn the silent `return struct_ty.clone()` fallbacks in
   infer_proj_field_type (infer/types.rs:1048,1061,1066) into KernelError.
6. **Delete or quarantine the dead modules** (`struct_eta/`, `eta/` standalone helpers,
   `ConversionChecker` if unused by the verify product) or at minimum fix TODO.md so
   "η-expansion for structures" is not marked complete. For the TCB budget, dead unsound
   stubs (apply_k_reduction) inside oxilean-kernel are audit poison.
7. **Add Proj-over-String-literal expansion** in try_reduce_proj if String verification is
   in scope for lean4export files.
8. **Add the def_eq-level test suite from §6**, plus a determinism check native-vs-WASM once
   wired (HashMap iteration is not currently observable in results, but keep it that way).

## 10. Key file/line index

| Item | Location |
|---|---|
| is_def_eq entry | def_eq/types.rs:1291 |
| is_def_eq_core dispatch (no struct eta) | def_eq/types.rs:1314-1358 |
| function eta (contraction only) | def_eq/types.rs:1543-1574 |
| lazy delta fallback | def_eq/types.rs:1363-1398 |
| quick_infer_type (no FVar/Proj) | def_eq/types.rs:1454-1505 |
| DefEqConfig.eta (decorative) | def_eq/types.rs:765-776 |
| TypeChecker::is_def_eq | infer/types.rs:868-870 |
| infer_proj / infer_proj_field_type | infer/types.rs:984-1068 |
| whnf_env Proj case | reduce/types.rs:1146-1152 |
| try_reduce_proj | reduce/functions.rs:328-352 |
| is_structure_like (correct predicate) | declaration/types.rs:731-736; env/types.rs:309-312 |
| StructureEta (dead) | struct_eta/types.rs:863-961 |
| eta_expand_struct level/param bugs | struct_eta/types.rs:950 (`vec![]` levels), 952-958 (params missing) |
| SingletonKReducer (dead) + stub | struct_eta/types.rs:130-193 (apply_k_reduction stub at 190-193) |
| ConversionChecker (no eta, 2nd def-eq) | conversion/types.rs:348-501 |
| eta/ standalone helpers (dead wrt def_eq) | eta/functions.rs:15-117 |
| False "done" claims | crates/oxilean-kernel/TODO.md ("η-expansion for structures", "K-like reduction for singleton types") |
| module-only tests, no def_eq eta tests | struct_eta/functions.rs:25-660; def_eq/functions.rs:23-127 |
| probe reproducer | scratchpad/eta_probe/src/main.rs |
