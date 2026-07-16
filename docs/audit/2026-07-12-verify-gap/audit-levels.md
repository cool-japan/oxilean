# Audit: Universe level normalisation and definitional equality in oxilean-kernel

Date: 2026-07-12. Repo: oxilean (branch 0.1.3, clean).
Scope: brief section 4, hard part (5) — universe level defeq (max/imax normalisation, isDefEq on levels).
Method: full source read of `crates/oxilean-kernel/src/{level,universe,def_eq,infer,check}` plus an
empirical differential harness (scratch crate at
`<scratchpad>/levelaudit/`,
path-dep on the kernel; repo untouched) comparing kernel results against a reference evaluator with
param assignments in 0..=3.

---

## 0. File map (level-relevant)

| File | Content | Notes |
|---|---|---|
| `crates/oxilean-kernel/src/level/types.rs` | `Level` enum (L1084-1101), `LevelMVarId` (L1313-1315), `LevelConstraint`/`ConstraintSet` (L93-117, L740-790) | ~85% of the file is unrelated SplitRS filler (DecisionNode, TokenBucket, MinHeap, Stopwatch, ...) |
| `crates/oxilean-kernel/src/level/functions.rs` | `to_offset` L18, `from_offset` L28, `push_max_args` L36, `is_norm_lt` L48, `normalize` L99-200, `is_equivalent` L204-206, `is_geq`/`is_geq_core` L211-247, `is_leq` L249-251, `instantiate_level` L256, mvar helpers L300-339, misc helpers L533-736 | the real TCB level code, ~340 SLoC of substance |
| `crates/oxilean-kernel/src/level/level_traits.rs` | `Display for Level` | prints `max(..)`, `imax(..)`, numerals |
| `crates/oxilean-kernel/src/universe/functions.rs` | `pi_type_level` L152, `level_to_nat` L156-176 (ground evaluator with correct imax semantics), `substitute_level_param` L241, `format_level` L257, `parse_level_str` L279, `collect_nf_comps` L298-313 | mostly convenience layer, NOT on the defeq path |
| `crates/oxilean-kernel/src/universe/types.rs` | `LevelNormalForm` L455-489, `UnivPolySignature` L491-546, `UnivConstraintSet` L971-1039, `UnivChecker` L1041-1203 | constraint machinery; **not wired into check_declaration** |
| `crates/oxilean-kernel/src/def_eq/types.rs` | `DefEqChecker::is_def_eq_core` L1314-1357 (Sort L1324, Const L1327-1334), proof irrelevance L1511-1541 | the def-eq actually used by `TypeChecker` |
| `crates/oxilean-kernel/src/def_eq/functions.rs` | `syntactic_eq` L130-159 (Sort L134, Const L135-142) | despite its name it also uses `is_equivalent` |
| `crates/oxilean-kernel/src/conversion/{types,functions}.rs` | third and fourth copies of Sort/Const level comparison (types.rs L431, L469; functions.rs L118, L553) | duplicate def-eq implementations; drift risk |
| `crates/oxilean-kernel/src/infer/types.rs` | `TypeChecker` — `ensure_sort` L848-855, `infer_type` L891-941 (Sort L893, Pi L917-924), `infer_const` L944-965, `is_level_eq` L1314-1317 (dead stub) | main checker used by `check/functions.rs` |
| `crates/oxilean-kernel/src/check/functions.rs` | `check_declaration` L25-57, `check_constant_info` L80-126 | no universe-param hygiene checks at all |
| `crates/oxilean-kernel/tests/prop_tests.rs` | proptest: `arb_level` L108-129, normalize idempotency L297-307, numeral fixed point L386 | no reference-evaluator comparison |

`lib.rs` L263: `#![forbid(unsafe_code)]` present. `Cargo.toml`: zero runtime deps (proptest/criterion dev-only). Good TCB hygiene on that axis.

---

## 1. Level representation and `normalize`

### Representation (`level/types.rs:1084-1101`)
```rust
pub enum Level {
    Zero,
    Succ(Box<Level>),
    Max(Box<Level>, Box<Level>),
    IMax(Box<Level>, Box<Level>),
    Param(Name),
    MVar(LevelMVarId),   // <-- TCB fat for a proof checker
}
```
- All six constructors, matching Lean plus `MVar`. **For the oxilean-verify product the `MVar`
  variant is dead weight in the TCB**: a kernel checking a lean4export file must never see a level
  metavariable. Lean's kernel *rejects* declarations containing mvars; oxilean silently threads
  them through (`is_not_zero` -> false, `instantiate_level` clones them, reference behaviour
  undefined). Recommend: verify path rejects `MVar` on ingestion (or feature-gate the variant out).
- `is_not_zero` (types.rs:1154-1162) is correct: `Max` -> either side, `IMax(_,l2)` -> `l2`.
- Boxed binary tree, no hash-consing/interning; `is_equivalent` re-normalizes both sides on every
  call from `DefEqChecker::is_def_eq_core` (no level cache). Perf risk on mathlib-scale exports.

### `normalize` (`level/functions.rs:99-200`)
Algorithm: `to_offset` peel; then by base kind:
- `Zero/Param/MVar/Succ` -> return input unchanged (already canonical).
- `IMax(l1,l2)` (L104-134): normalize children, then
  - `l2 == 0`  -> `succ^k(0)`   (imax(_,0)=0)  — correct.
  - `l2.is_not_zero()` -> `normalize(max(succ^k l1, succ^k l2))` — correct (imax->max when rhs
    provably nonzero), and **distributes the outer offset into the max args**.
  - `l1 == 0` -> `succ^k(l2)` re-normalized — correct (imax(0,v)=v).
  - otherwise -> `succ^k(imax(l1',l2'))` — **misses Lean's `l1 == l2 -> l1` rule** (see §3, D1).
- `Max` (L135-198): flatten via `push_max_args`, normalize each arg **with the outer offset k
  pushed inside** (`normalize(&from_offset(a, k))`, L143), re-flatten, sort by `is_norm_lt`,
  merge equal-base args keeping larger offset (L156-177), drop literal `Zero` args only when some
  arg `is_not_zero()` (L178-183), rebuild right-assoc max.

### Empirical verification (harness output)
- `normalize` is **idempotent** (0 violations over 583 enumerated levels) — matches the existing
  proptest (`tests/prop_tests.rs:304`).
- `normalize` is **semantics-preserving** (0 changes over the enumeration, assignments 0..=3).
- So `normalize` is SOUND. Its problem is **incompleteness relative to Lean's normalize**.

### Divergences from Lean's kernel normalize (src/kernel/level.cpp `normalize` + `mk_imax`)
1. **Missing `imax(u,u) -> u`** (Lean `mk_imax`: `else if (l1 == l2) return l1;`). oxilean keeps
   `IMax(u,u)`. Confirmed: `is_equivalent(imax(u,u), u) == false`.
2. **Missing explicit-numeral subsumption in max**: Lean drops an explicit numeral arg `k` when
   another arg has offset >= k (every level >= its offset). oxilean only drops the literal `Zero`
   and only when a sibling `is_not_zero()`. Confirmed:
   - `is_equivalent(max(0,u), u) == false` (norm keeps `Max(Zero, u)`);
   - `is_equivalent(max(1, succ u), succ u) == false` (norm keeps `Max(1, succ u)`).
3. **Offset placement differs from Lean**: Lean keeps the outer offset OUTSIDE the rebuilt max
   (`mk_succ(r, p.second)`); oxilean distributes it into each arg. Consequence:
   oxilean says `succ(max(u,v)) == max(succ u, succ v)` (both normalize to
   `Max(Succ u, Succ v)`), Lean's kernel `is_equiv` says NOT equal (different normal forms).
   This is the one place oxilean is **more permissive** than the Lean kernel. It is semantically
   sound (succ distributes over max), so not an alarm, but it is a native-vs-Lean determinism
   divergence to document: oxilean could "verify" a Sort equation the C++ kernel would reject.
   (In practice exports come from Lean-accepted terms, so only the too-strict direction bites.)

---

## 2. `is_leq` / `is_geq` — algorithm choice and gaps

`is_leq(l1,l2)` = `is_geq(l2,l1)` (functions.rs:249-251). `is_geq` = `is_geq_core(normalize(l1), normalize(l2))`.

`is_geq_core` (functions.rs:214-247) rules, tried in order (union, each early-returns true):
1. `l1 == l2` -> true
2. `l2.is_zero()` -> true
3. `Max(a,b)` on left: `a >= l2 || b >= l2`
4. `Max(b,c)` on right: `l1 >= b && l1 >= c`
5. `IMax(b,c)` on right: `l1 >= b && l1 >= c`
6. `IMax(_,b)` on left: `b >= l2` (sound: imax(a,b) >= b always)
7. offsets: `to_offset` both; if bases syntactically equal -> `k1 >= k2`; else false.

### Answer to the brief question: NO param case-split
This is **not** the standard complete "leq with case-split on Param" algorithm used by
trepplein / nanoda / lean4lean (substitute `p := 0` and `p := succ p'` when an imax with param
occurs, recurse with balance counter). Nothing in the crate performs a case split. What is
implemented is a clone of Lean 4's *Lean-side* `Level.geq` heuristic (Lean/Level.lean), minus two
of its cases:

- **Missing** Lean's final-case disjunct `v'.isZero`: Lean returns true when the right base is
  `zero` and `k1 >= k2` (any level >= its numeral offset). oxilean requires the bases to be
  *equal*. Confirmed: `is_geq(succ(u), 1) == false`, `is_geq(u+2, 2) == false` (semantically true).
- **Missing** Lean's `succ u, succ v -> go u v` strip-common-offset recursion (equivalently
  `p1.second == p2.second && p1.second > 0 -> is_geq(base1, base2)`). Confirmed:
  `is_geq(succ(imax(u,v)), succ(v)) == false` (semantically true; Lean geq: true).

Rules 3-6 do cover: `max l1 l2 <= l3 iff l1<=l3 && l2<=l3` (via geq rule 4) and the incomplete
`l <= max l1 l2 if l<=l1 or l<=l2` (via geq rule 3); imax on both sides is handled by rules 5/6
(and because all four rules are tried sequentially rather than committed via a match, oxilean is
marginally *more* complete than Lean's `Level.geq` match ordering for imax-vs-imax — but far less
complete than the case-split algorithm).

### Empirical sweep (depth<=2 levels over {0, u, v}, 339,889 ordered pairs, assignments 0..=3)
- `is_geq`: **0 unsound**, 14,168 incomplete (true-but-rejected) pairs.
- `is_equivalent`: **0 unsound**, 14,564 incomplete pairs (~4.3% of all pairs).
- Soundness holds — the checker never *accepts* a false level fact. All defects are in the
  reject-valid-proofs direction (maps to spurious `rejected` verdicts, which the brief treats as
  ALARM — see §6 impact).

### Where is_geq/is_leq are actually used
Only: `LevelConstraint::is_satisfied` (level/types.rs:106), `UnivChecker::{is_geq,is_gt,check,check_lt}`
(universe/types.rs:1096-1150), `UnivConstraintSet::check_all` (universe/types.rs:1013), `level_min`
(level/functions.rs:537). **None of these are on the `check_declaration` path.** Notably the
Lean-kernel use of `is_geq` — the inductive "universe level of arg is too big" check
(lean4 src/kernel/inductive.cpp) — is absent: `InductiveError::UniverseTooSmall`
(inductive/types.rs:81) is declared but **never constructed anywhere**. When the inductive
re-derivation work (hard part 4) lands, it will need `is_geq`, and today's `is_geq` would
spuriously reject e.g. `inductive Foo (n : Nat) : Type u` (needs `is_geq(succ u, 1)` = true;
oxilean says false). That is a fails-late trap.

---

## 3. Known divergence points — tricky-case catalogue (empirically confirmed)

| # | Case | oxilean | Lean 4 kernel | Semantic truth | Direction |
|---|---|---|---|---|---|
| D1 | `imax(u,u)` vs `u` | NOT equiv | equiv (`mk_imax` u==v rule) | equal | **too strict — CRITICAL** |
| D2 | `max(0,u)` vs `u` | NOT equiv | equiv (numeral subsumption) | equal | too strict |
| D3 | `max(1, succ u)` vs `succ u` | NOT equiv | equiv | equal | too strict |
| D4 | `succ(max(u,v))` vs `max(succ u, succ v)` | equiv | NOT equiv (offset kept outside) | equal | **too lenient vs Lean** (sound) |
| D5 | `imax(u, imax(v,w))` vs `max(imax(u,w), imax(v,w))` | NOT equiv | NOT equiv | equal | parity with Lean (both incomplete; complete checkers with case-split accept) |
| D6 | `imax(u, max(v,w))` vs `max(imax(u,v), imax(u,w))` | NOT equiv | NOT equiv | equal | parity with Lean |
| D7 | `imax(v, imax(v,u))` vs `imax(v,u)`; `imax(imax(u,v), v)` vs `imax(u,v)` | NOT equiv | NOT equiv (no such rule) | equal | parity |
| D8 | `is_geq(succ u, 1)`, `is_geq(u+2, 2)` | false | `Level.geq` true | true | too strict (matters for future inductive check) |
| D9 | `is_geq(succ(imax(u,v)), succ v)` | false | `Level.geq` true | true | too strict |
| D10 | `succ(imax(u,v))` normal form | `Succ(IMax(u,v))` (offset outside for imax; inside for max) | same for imax | — | internal inconsistency: offsets distributed for Max but not IMax; harmless because normalize is applied at comparison time, but makes normal forms non-uniform |

### D1 is reachable end-to-end and will fire on virtually every real export
Confirmed with the real `check_declaration` path (harness `src/bin/e2e.rs`):

```
def Endo.{u} : Sort u -> Sort u := fun (a : Sort u) => a -> a
REJECTED (spurious): type mismatch in checking (λ a : Type u, (#0 → #1)):
  expected (Type u → Type u), got (Type u → Type imax(u, u))
```

Mechanism: `TypeChecker::infer_type` for Pi (infer/types.rs:917-924) builds
`Expr::Sort(Level::imax(dom_sort, cod_sort))` with the RAW constructor — no smart-constructor
simplification (Lean's kernel uses `mk_imax`, and the elaborator already stored the simplified
`Sort u` in the export). Comparison then goes through `level::is_equivalent`, whose `normalize`
lacks the `u==v` rule, so `Sort (imax u u) != Sort u`. Every polymorphic definition whose body
mentions `a -> a`-shaped types (i.e., most of any real library) mass-fails. This single missing
line is the highest-impact bug in the level area.

### Other structural notes
- `is_norm_lt` (functions.rs:48-90) compares `Param` names via `to_string()` (allocation per
  comparison; perf). Potential comparator inconsistency: two structurally distinct hierarchical
  `Name`s that render to the same string (e.g. `Str(Str(_, "a"), "b")` vs `Str(_, "a.b")`) make
  `sort_by` in `normalize` (L147-155) see `Greater` in both directions — Rust's sort may panic
  ("comparison function does not correctly implement a total order") or produce a non-canonical
  order. Compare `Name` structurally instead.
- `to_offset` returns `u32` offset — fine (export files can't practically exceed 2^32 succs), but
  `from_offset` loops k times building boxes; deep offsets from hostile exports are an O(k) alloc
  chain; acceptable.
- `universe/types.rs:455-489 LevelNormalForm` + `universe/functions.rs:298-313 collect_nf_comps`
  is **semantically wrong**: it flattens `IMax(a,b)` exactly like `Max(a,b)` (drops the
  impredicative zero rule) and drops `MVar` args silently. It is currently used only by its own
  test (universe/functions.rs:409-413). It must never be used for defeq; recommend deleting it
  from the verify TCB or fixing the IMax case.
- `universe/functions.rs:156-176 level_to_nat` is a correct ground evaluator including imax
  semantics — a good seed for the reference-evaluator test harness (§5).

---

## 4. Is level defeq used correctly in expr def_eq?

**Yes — this part is right.** All four def-eq implementations compare levels semantically,
per-element, with length check:

- `DefEqChecker::is_def_eq_core` (def_eq/types.rs):
  - L1324 `(Sort l1, Sort l2) => level::is_equivalent(l1, l2)`
  - L1327-1334 `(Const(n1,ls1), Const(n2,ls2)) => n1==n2 && ls1.len()==ls2.len() && zip.all(is_equivalent)`
- `syntactic_eq` (def_eq/functions.rs:134, 135-142) — same pattern.
- `conversion/types.rs:431, 469` and `conversion/functions.rs:118, 553` — same pattern.

Wiring: `check/functions.rs::check_declaration` -> `TypeChecker` (infer/types.rs) ->
`self.def_eq_checker: DefEqChecker` (`is_def_eq`, infer/types.rs:868-870) -> the L1324/L1327 code.
So the *shape* required by the brief (per-element `is_equiv`, not syntactic list eq) is present;
the defect is inside `is_equivalent` itself (D1-D3), not in its call sites.

Caveats found nearby:
1. **`infer_const` universe-arity hole** (infer/types.rs:944-965): if a constant has universe
   params but is referenced with an EMPTY level list, the raw (uninstantiated) type is returned
   instead of erroring (`if params.is_empty() || levels.is_empty() { return Ok(ci.ty().clone()) }`).
   Lean's kernel requires exact arity. A malformed/hostile export can make a polymorphic
   constant's `Param(u)` leak into a context that declares its own `u` — unsoundness risk via
   capture, and definitely a "must reject" case for the verify product.
2. **No declaration-level universe hygiene** (`check/functions.rs:25-57`): no duplicate
   `univ_params` check, no check that every `Param` occurring in ty/val is declared, no `MVar`
   rejection. Lean's kernel does all three. A hostile export using an undeclared `Param` will be
   accepted here.
3. **Proof irrelevance Prop test is syntactic** (def_eq/types.rs:1524, 1536; also
   infer/types.rs:1073): `matches!(ty_ty_whnf, Expr::Sort(l) if l.is_zero())` uses the syntactic
   `Level::is_zero`, so a proof whose Sort level is e.g. `imax(u,0)` (normalizes to 0 but is not
   the literal `Zero`) will not get proof irrelevance — another too-strict path. Should be
   `is_equivalent(l, &Level::Zero)` or `normalize(l).is_zero()`.
4. `TypeChecker::is_level_eq` (infer/types.rs:1314-1317) is `l1 == l2 || (both zero)` — an
   essentially-syntactic stub. It is public API but has **zero internal callers**; it is a trap
   for future contributors. Delete or delegate to `level::is_equivalent`.

---

## 5. Tests: what exists, what's missing

Exists (`tests/prop_tests.rs`, proptest 4096-ish cases):
- `arb_level(max_depth)` generator over Zero/Param/Succ/Max/IMax (no MVar) — L108-129.
- `prop_level_normalize_idempotent` — L300-307: `normalize(normalize(l)) == normalize(l)`.
- `prop_numeric_level_normalize_fixed` — L386-392: numerals are normalize-fixed-points.
- Unit tests in level/functions.rs (L340-526, L583-658, L737-854): constructor sanity, a handful
  of normalize/geq cases (`imax(u,0)=0`, `imax(u, succ v)=max(u, succ v)`, max commutes, ...).
  Also universe/functions.rs unit tests. None cover D1-D9.

Missing (recommend building; the audit harness is a ready template at
`scratchpad/levelaudit/src/main.rs`):
1. **Reference-evaluator differential test** (the brief's suggestion): exhaustively enumerate
   levels of depth <= 2-3 over params {u,v(,w)}, evaluate under all assignments in 0..=3, assert
   - soundness: `is_equivalent => forall-assignment-equal`, `is_geq => forall-assignment->=`;
   - targeted completeness: a curated list of MUST-ACCEPT pairs (D1, D2, D3, D8, D9 plus every
     rewrite in Lean's `mk_imax`/`mk_max_core`) — these fail today and should become regression
     tests for the fixes.
   The evaluator is ~15 lines; `universe::level_to_nat` (functions.rs:156) already implements the
   ground semantics and can be reused with a param environment.
2. **Cross-check against Lean's normal forms**: golden tests asserting oxilean's `normalize`
   agrees with `Level.normalize` output on a corpus (or explicitly documenting D4 as an accepted
   divergence).
3. Property: `normalize` semantic preservation (0..=3 assignments) — currently only proven by my
   external harness, not encoded in the repo.
4. Property: `is_norm_lt`-induced ordering is a strict weak order (guards the sort_by panic risk).

---

## 6. Impact assessment for oxilean-verify's 3-bucket verdicts

- Soundness (the thing that must never break): **currently OK** in the level area — no case was
  found where a semantically-false level fact is accepted (340k-pair sweep + targeted probes).
- Verdict correctness: **broken in practice.** D1 alone makes the checker emit `rejected`
  (= alarm, non-zero exit) for `def Endo.{u} : Sort u → Sort u := fun a => a → a` — i.e., for
  essentially every real lean4export file. Spurious `rejected` verdicts are the worst failure
  mode the brief defines, because `rejected` is supposed to mean "proof is wrong / file tampered".
- D5/D6/D7 are parity-with-Lean incompletenesses; they cannot fire on exports produced by Lean
  (the elaborator never emits those shapes un-simplified) and can be left as-is, documented.
- D4 (more permissive than Lean) is sound; document it, or align by keeping offsets outside max
  during normalize if bit-for-bit Lean-kernel agreement is desired.

## 7. Ordered fix list (concrete)

1. `level/functions.rs` `normalize`, IMax branch (after the `l1_norm.is_zero()` check, ~L132):
   add `if l1_norm == l2_norm { return from_offset(l1_norm, k); }` — one line, kills D1.
   Better: implement Lean's `mk_imax`/`mk_max_core` smart constructors and route both `normalize`
   and `infer_type`'s Pi case (infer/types.rs:923) through them.
2. `normalize`, Max merge phase (~L178-183): implement numeral subsumption — drop any arg that is
   a pure numeral `n` when another arg has offset >= n; kills D2/D3. (After sorting, numerals sort
   first — kind_ord(Zero)=0 — so this is a single pass.)
3. `is_geq_core` (~L241-246): change the final case to Lean's
   `(b1 == b2 || b2.is_zero()) && k1 >= k2`, and add the equal-offset strip
   `if k1 == k2 && k1 > 0 { return is_geq_core(b1, b2); }` — kills D8/D9.
4. For the verify product decide: either (a) adopt the complete param-case-split leq
   (trepplein/lean4lean algorithm, ~60 lines, makes D5-D7 accepted too and gives a provably
   complete decision procedure) and use `leq(a,b) && leq(b,a)` for equivalence, or (b) stay
   normalize-based for strict Lean-kernel parity. Either is defensible; (a) is safer against
   "fails late" surprises, (b) maximizes determinism vs the C++ kernel. Do not ship the current
   halfway point.
5. `infer_const` (infer/types.rs:955): make level-list arity mismatch (including empty-vs-nonempty)
   a hard error.
6. `check_declaration` (check/functions.rs:25): add universe hygiene — duplicate `univ_params`
   rejection, all `Param`s in ty/val declared, no `MVar` anywhere (walk ty/val levels).
7. Proof irrelevance zero test (def_eq/types.rs:1524,1536; infer/types.rs:1073): use
   `normalize(l).is_zero()` / `is_equivalent(l, Zero)`.
8. Replace `is_norm_lt`'s `Name::to_string()` comparison with structural `Name` ordering
   (fixes comparator-inconsistency panic risk + allocations).
9. Delete or fix: `TypeChecker::is_level_eq` (dead, misleading), `LevelNormalForm`/`collect_nf_comps`
   (imax-unsound), and consider gating `Level::MVar` + `UnivChecker` mvar machinery out of the
   verify TCB.
10. Land the reference-evaluator differential test (harness in
    `scratchpad/levelaudit/src/main.rs` is directly portable to `tests/`), with D1-D9 as named
    regression cases.

## 8. Harness artifacts

- `scratchpad/levelaudit/src/main.rs` — differential sweep (targeted probes + 339,889-pair
  exhaustive check + idempotency + semantic preservation). Output summary:
  `is_equivalent: UNSOUND=0 INCOMPLETE=14564; is_geq: UNSOUND=0 INCOMPLETE=14168;`
  `normalize non-idempotent=0; normalize semantic-change=0`.
- `scratchpad/levelaudit/src/bin/e2e.rs` — real `check_declaration` rejection of
  `def Endo.{u}` (imax(u,u) case), output:
  `REJECTED (spurious): type mismatch ... expected (Type u → Type u), got (Type u → Type imax(u, u))`.
