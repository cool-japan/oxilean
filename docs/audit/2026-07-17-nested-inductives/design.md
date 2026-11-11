# C10 — Nested-to-mutual inductive elaboration (design)

> Branch: `0.1.4`. Started 2026-07-17. Single highest-leverage completeness item
> for the verify product: `Lean.Syntax` alone gates **98.9 %** of Init's 22,054
> `unsupported` declarations.

## 0. Goal & non-goal

**Goal.** Teach the verify kernel to *compile* a nested inductive (an inductive
whose constructor fields mention the type under an already-declared container,
e.g. `node : Array Syntax → Syntax`) the way Lean's kernel does — by specializing
the container into an auxiliary mutual family, checking that mutual family with
the machinery we already have, and restoring recursors stated over the **real**
container so that the exported `Syntax.rec` re-verifies by def-eq.

**Non-goal.** We do *not* weaken `recursor_matches` (`derive.rs:1464`): every
exported recursor must still be re-derived and matched def-eq. C10 is a
**completeness** change; it must never turn a `rejected` into an accept without a
sound kernel re-derivation, and must never turn an accept into a `rejected`.

## 1. Ground truth (from the real Init export)

`~/work/oxilean-corpus/Init.ndjson` has **exactly one** nested inductive:
`Lean.Syntax` (name index `8577`, `numNested: 2`). Its exported `inductive`
object (line 78369) shows:

- **`types: [8577]` only** — *no auxiliary inductive types are exported.* Nesting
  is represented purely in the recursors. Restoration is therefore a
  recursor-level problem; the original type carries `numNested`.
- **`ctors`**: `8586..8589` — Syntax's 4 ctors (`node`, `atom`, `ident`, `missing`),
  `numFields` `0/3/2/4`, `numParams: 0`.
- **`recs`: three recursors, each `numMotives: 3`, `numMinors: 7`:**
  | rec name | rules fire on ctors | meaning |
  |----------|--------------------|---------|
  | `8606` (`Syntax.rec`) | `8586..8589` (Syntax's) | the user-facing recursor |
  | `8595` | `59` (nfields 0), `60` (nfields 2) | List-of-Syntax aux rec — `59/60 = List.nil/List.cons` |
  | `8607` | `68` (nfields 1) | Array-of-Syntax aux rec — `68 = Array.mk` |

  `numMinors: 7 = 4 (Syntax) + 2 (List) + 1 (Array)`; `numMotives: 3 =`
  {Syntax, List Syntax, Array Syntax}. So `numNested: 2` is **Array-then-List
  double nesting** (`Array Syntax`, and `Array` is built on `List`), and the
  restored recursors **iota-reduce on the real container ctors**
  (`List.nil/List.cons/Array.mk`), not on aux ctors.

**Consequence.** The exported `Syntax.rec` (`8606`) already carries the 3 motives
/ 7 minors of the *expanded* mutual family `{Syntax, ListSyntax_aux,
ArraySyntax_aux}` — but its rules are stated over the **real** `List`/`Array`
constructors. Our kernel must re-derive that same object: build the expanded
mutual family, derive its recursors, then substitute aux type-formers /
constructors / recursors back to the real container constants and drop the
abstracted parameters, producing recursors def-eq to `8595/8606/8607`.

## 2. Reference algorithm (lean4lean `ElimNestedInductive` + Lean C++ kernel)

Three phases — SPECIALIZE → CHECK → RESTORE — run as a fuel-bounded fixpoint
(reach-through: abstracting `Array Syntax` forces also specializing `List Syntax`).

1. **DETECT** (`isNestedInductiveApp?`). A field type WHNFs to `C Ds Is` where
   `C` is an already-declared inductive **not** in the family being defined, the
   parametric args `Ds` contain a family-being-defined type (and lack loose
   bvars), and no family type occurs in an **index** position of `C`. Reject
   (keep `UnsupportedNestedInductive`) if `C` is not a declared inductive, is
   undeclared, or a family type occurs in an index position.

2. **SPECIALIZE** (`ElimNestedInductive.run(fuel, nparams, types) → Result`).
   For each distinct nested container application, copy the entire mutual block
   `C.all` into fresh auxiliary inductives (`<decl>._nested.<k>.<Cmember>`). In
   each auxiliary, abstract the family-occurring parameter args `As` of `C` as
   **new leading parameters** prepended to that aux type's param telescope, and
   inside the copied ctor types replace every occurrence of `C`'s members by the
   corresponding aux member and every family-being-defined type by a direct
   mutual-sibling reference. `List Syntax` becomes aux `ListSyntax` (param
   `Syntax`) recursing directly into `Syntax` and `ListSyntax`.
   `Result { ngen, nparams, aux2nested: NameMap Expr, types }` — `aux2nested`
   maps each aux inductive name to the instantiated real container application.

3. **CHECK** (reuse existing path unchanged). Feed `Result.types` (originals ++
   auxiliaries — a genuine nesting-free mutual family) to
   `build_family_pre`/`check_ctor`/`derive_family_core`. Now `as_valid_ind_app`
   succeeds on the direct aux occurrences, `check_positivity` passes,
   `classify_rec_field` records the aux `type_idx`, and `derive_family_core`
   emits motives / minors / one recursor per member. The rule RHS recursive
   calls already use `Const(rec_names[type_idx])`, so aux recursors are invoked
   automatically — **no reducer change**.

4. **RESTORE** (`restoreNested(r, env', e, auxRec)`). Rewrite the derived
   recursors for the original types: substitute aux type-former → real container,
   aux ctor → real ctor, aux recursor → real/retained recursor, and **drop the
   abstracted `As` params** (`mkAppRange`), yielding a user-facing `<T>.rec`
   stated over real `List`/`Array`. Retain the aux recursors in the environment
   under stable names (Lean issue #11283: reducing the restored real `.rec` can
   still emit them). Set `InductiveVal.num_nested` on the original; keep `all` =
   original members only.

**Off-by-one invariant.** Auxiliaries get *precisely* the abstracted `As`
prepended as leading params; restoration removes *precisely* those. Universe /
level bookkeeping stays consistent across the whole family and matches the
kernel's existing recursor universe rules.

## 3. Where the code changes

Current machinery, all `crates/oxilean-kernel/src/inductive/derive.rs`:

- `check_and_derive_family` (`:1060`) — stage stubs, `build_family_pre` (`:430`),
  `check_ctor` (`:664`) loop, `derive_family_core` (`:800`); does not mutate env.
- `add_inductive_family` (`:1131`) — the above plus `env.add_constant`.
- **The single kernel reject site**: `check_positivity` (`:602`), leaf branch
  `:628-631` — foreign `Const c` head ⇒ `UnsupportedNestedInductive(decl_name)`.
- `as_valid_ind_app` strict mode (`:547-564`) demands recursive occurrences apply
  to exactly the family params — correct for the *expanded* family; unchanged.
- `is_large_eliminating` (`:746`) returns `false` for any `>1`-member family —
  must key the elim decision off the **original** type once expanded.
- `num_nested` hard-coded `0` in the real build (`:998-1000`) and stub (`:1098`).

Export side, `crates/oxilean-export/src/replay.rs`:

- `replay_inductive_core` (`:619`) short-circuits `num_nested > 0` ⇒
  `Unsupported(NESTED_INDUCTIVES)` (`:623`); `map_inductive_kernel_error`
  (`:701`) folds `UnsupportedNestedInductive` to the same bucket. Both change in
  lockstep: drop the short-circuit, route through the nested-aware derivation,
  keep the fold as the fallback for the genuinely-unsupported residue.

New module: `crates/oxilean-kernel/src/inductive/nested.rs`
(`NestedOcc`, `detect`, `specialize → ExpandedFamily`, `restore`).

## 4. Staged plan (each stage independently caged-gate-able)

- **S0 — fixtures + caged baseline.** Real minimal nested fixtures (kernel unit
  tests need none; replay tests do). Snapshot caged full-Init baseline
  (verified 35,223 / unsupported 22,054 / rejected 0) and the
  `NESTED_INDUCTIVES`-attributed follower set (per-decl `{name, outcome}` NDJSON).
- **S1 — detection + structural capture (still reject).** `nested::detect`
  returns `NestedOcc{container, levels, args, family_positions}`; behaviour
  unchanged. Unit-tested in isolation. *(This doc's first implemented stage.)*
- **S2 — specialize + kernel-only nested derivation (raw recursors).** HIGH risk.
  `nested::specialize` + `check_and_derive_family_nested` → `build_family_pre`/…
  on originals ++ auxiliaries, returning raw (un-restored) aux recursors. Fix
  `is_large_eliminating`. Gate: kernel-only, decoupled from export matching —
  prove iota round-trip on the raw aux family.
- **S3 — recursor restoration + num_nested.** HIGH risk / make-or-break.
  `nested::restore`; set correct `num_nested`; retain aux recursors. Gate:
  restored `<T>.rec` mentions the real container, iota still reduces, restored
  type re-checks.
- **S4 — wire into replay + match exported recursors.** Drop the `num_nested>0`
  short-circuit; route `replay_inductive_core` through the nested-aware
  derivation; exported recursors re-verified via `recursor_matches`. Land
  `NestedT`/`RoseTree`/`RoseTreeArray` fixtures end-to-end.
- **S5 — `Lean.Syntax` + full-Init acceptance.** Get `Lean.Syntax` to verify;
  flip the negative tests where the shape is now supported (keep genuine
  rejects); run the caged full-Init acceptance gate.

**Minimal first target.** `inductive NestedT | mk : List NestedT → NestedT`
(one type, no params, one self-ref ctor under builtin `List`). Make it pass at
the KERNEL level first (a `derive.rs` unit test beside `nested_inductive_rejected`
`:1562`): `check_and_derive_family_nested` returns `Ok`; expanded family
`{NestedT, ListNestedT_aux}`; after restore `NestedT.rec` mentions real
`List`/`List.nil`/`List.cons`; iota round-trip
`NestedT.rec motive minor_mk (NestedT.mk xs)` reduces to `minor_mk` applied to
`xs` plus the List-shaped IH, no dangling aux constant.

## 5. Acceptance gate

Every corpus-scale run is **caged**: `systemd-run --user --scope -p
MemoryMax=12G` on the 14 GiB box (per `machine-memory-discipline`) or the machine
freezes. Corpus-scale runs are the user's segment; kernel unit tests are cheap
and local.

Full-Init acceptance, three hard invariants against the S0 baseline
(35,223 / 22,054 / 0):

1. **`rejected` stays exactly 0** — C10 never turns a deferred/unsupported decl
   into a rejection.
2. **`verified` is a strict superset by NAME** — every previously-verified name
   still verifies (zero regressions), and the set is strictly larger. Compare
   name-sets, not counts.
3. **Expected delta (honest):** `verified += Lean.Syntax (1) + ~14,347
   nested-only followers ≈ +14,348` (verified ≈ 49,571); the `NESTED_INDUCTIVES`
   bucket drops to 0 as a root cause; unsupported drops to ≈ 7,706. The **7,265
   nested+clone-fuel-joint** + 13 triple-joint followers **do not clear** (still
   blocked on clone-fuel, out of C10 scope) — expected, not a failure.
   `num_nested` must be non-zero on `Lean.Syntax`'s `InductiveVal`.

Diff procedure: caged Init replay before/after → per-decl `{name, outcome}`
NDJSON → assert superset + `rejected == 0` + `NESTED_INDUCTIVES` root cleared. A
single regressed name or any `rejected > 0` fails the gate.

### Result (2026-07-17, caged full-Init, `--limits corpus`)

| | verified | unsupported | rejected |
|---|---|---|---|
| baseline (pre-C10) | 35,223 | 22,054 | 0 |
| **after C10** | **47,119** | **10,158** | **0** |
| delta | **+11,896** | −11,896 | ±0 |

- **`rejected == 0` held** — the soundness invariant. All 57,277 declarations
  got exactly one verdict (no silent skips).
- **`nested inductives` root cleared: 1 → 0** — `Lean.Syntax` verifies. Its
  cascade resolved: `dependency on an unsupported declaration` fell 21,876 →
  9,515 (−12,361).
- **`clone-fuel budget` rose 175 → 643** (+468) — expected and design-anticipated:
  with `Lean.Syntax` now available, more of its followers become *reachable* and
  a subset hits the (out-of-scope) clone-fuel budget rather than verifying. Net
  verified still +11,896.
- The +11,896 is below the pre-run ~14,348 estimate precisely because of that
  clone-fuel re-attribution (the "nested + clone-fuel joint" followers the plan
  flagged as not clearing under C10 alone). Wall ≈ 50 min — longer than the
  35k-verified baseline because ~12k more declarations are now genuinely checked
  rather than skipped as unsupported-dependency.

Fidelity was independently confirmed against real Lean output: a targeted
`lake env lean4export Init -- Lean.Syntax` export (563 KB, 269 decls) replays to
**269 verified / 0 unsupported / 0 rejected**, with `Lean.Syntax` (numNested=2)
and its recursors `Syntax.rec` / `.rec_1` (Array) / `.rec_2` (List) installed —
pinned as the repo fixture `tests/fixtures/lean4export/Syntax.ndjson` and the
`syntax_nested_inductive_all_checked` regression test.

## 6. Open questions (carried into implementation)

- Restoration by **substitution** (lean4lean, relies on aux ≡ real container up
  to renaming) vs **transport** (Lean3 explicit pack/unpack). Substitution is
  lighter but requires our derivation to produce an aux family structurally
  identical to the real container up to renaming — confirm, else fall back to
  transport.
- Exact motive-universe / injected-level bookkeeping for the elim-to-`Type`
  motive across the expanded family; must match `recursor_matches` def-eq.
- Genuinely-unsupported residue (family in an index position, non-inductive or
  undeclared container): keep `UnsupportedNestedInductive` (lands in the
  `NESTED_INDUCTIVES` bucket) — never silently accept.
