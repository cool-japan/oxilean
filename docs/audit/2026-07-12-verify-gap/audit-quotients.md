# Audit: Quotient types in oxilean-kernel

Repo: oxilean (branch 0.1.3, clean). Date: 2026-07-12.
Scope: brief section 4 hard-part (1): Quot/Quot.mk/Quot.lift/Quot.ind + computation
rule `Quot.lift f h (Quot.mk r a) ≡ f a`, TCB-quality implementation.

## TL;DR

There are TWO disjoint quotient implementations in the kernel:

1. **The real path** (`reduce/functions.rs::try_reduce_quot`, driven by
   `QuotVal`/`QuotKind` entries in the `Environment`) — wired into the whnf used by
   both the `TypeChecker` and the `DefEqChecker`. `Quot.lift` iota is positionally
   correct; `Quot.ind` iota is **off-by-one and can never fire correctly**; both
   rules **drop over-application arguments (unsound)** and **fail to whnf the major
   premise (incomplete)**. It has **zero tests**.
2. **A toy path** (`src/quotient/` module, ~2650 SLoC, mostly filler) — string-based
   `Name::str("Quot.lift")` matching with wrong arities (3-arg lift, 1-arg mk).
   Not referenced by the type checker at all, yet re-exported from `lib.rs` as the
   public quotient API. Its names are single-atom (`Str(Anonymous, "Quot.lift")`),
   which will never equal hierarchical names from a lean4export reader.

Nothing anywhere constructs the four canonical quotient declarations with correct
types; there is no `add_quot`/#QUOT entry point, no Eq-presence check, no
once-only guard beyond generic duplicate-name rejection. `Quot.sound` is never
modeled. Test coverage of the real reduction path is zero.

---

## 1. Which of the four #QUOT primitives exist?

### Data model — correct shape, matches Lean

`crates/oxilean-kernel/src/declaration/types.rs:119-130`:

```rust
pub enum QuotKind {
    /// `Quot` type itself.
    Type,
    /// `Quot.mk` constructor.
    Mk,
    /// `Quot.lift` eliminator.
    Lift,
    /// `Quot.ind` induction principle.
    Ind,
}
```

`declaration/types.rs:1300-1308`:

```rust
pub struct QuotVal {
    pub common: ConstantVal,   // name, level_params, ty
    pub kind: QuotKind,
}
```

`ConstantInfo::Quotient(QuotVal)` variant exists (declaration/types.rs:572-573),
with `is_quotient()` (:630) and `to_quotient_val()` (:662). This exactly mirrors
Lean's four #QUOT declarations, and correctly does NOT include `Quot.sound`
(which in Lean is an axiom added separately). **The enum model is right.**

### But: nothing constructs the four declarations

- `grep 'QuotVal {'` over the whole workspace finds only two **test** sites
  (check/functions.rs:901, declaration/functions.rs:236) — both with
  `kind: QuotKind::Type` and a dummy `ty: Prop`. **No production code ever builds
  a QuotVal.** No QuotVal with kind `Mk`, `Lift`, or `Ind` exists anywhere,
  including tests.
- `builtin/functions.rs::add_core_axioms` (:464-527) adds only **`Quot`** — and as
  a plain `ConstantInfo::Axiom`, not `ConstantInfo::Quotient`. `Quot.mk`,
  `Quot.lift`, `Quot.ind`, `Quot.sound` are never added by any builtin/bootstrap
  code.
- `oxilean-std/src/env_builder/functions/part2.rs:722-802 add_quotient()` adds the
  **Setoid-layer** constants `Setoid`, `Quotient`, `Quotient.mk`, `Quotient.lift`,
  `Quotient.sound` as axioms — these are the std-library wrappers, NOT the kernel
  primitives, and their types are approximations (see §2).
- TODO.md:78-81 claims "3 built-in declarations: Quot.mk, Quot.lift, Quot.sound"
  in a file `oxilean-kernel/src/quot.rs` — **that file does not exist**, the count
  is wrong vs both the code (QuotKind has 4) and Lean (#QUOT introduces 4 +
  Quot.sound as axiom). The TODO entry is stale/aspirational.

**Verdict:** the *classification enum* models #QUOT correctly (4 decls, sound
separate); the *declarations themselves* exist nowhere.

## 2. Are the types of the primitives constructed correctly?

### The only constructed `Quot` type is wrong

`builtin/functions.rs:486-505`:

```rust
let quot_ty = Expr::Pi(
    BinderInfo::Implicit,
    Name::str("α"),
    Box::new(Expr::Sort(Level::param(Name::str("u")))),
    Box::new(Expr::Pi(
        BinderInfo::Default,
        Name::str("r"),
        Box::new(type0.clone()),                      // <-- BUG: r : Prop
        Box::new(Expr::Sort(Level::param(Name::str("u")))),
    )),
);
env.add_constant(ConstantInfo::Axiom(AxiomVal {
    common: ConstantVal { name: Name::str("Quot"),
                          level_params: vec![Name::str("u")], ty: quot_ty },
    is_unsafe: false,
}))
```

This gives `Quot : {α : Sort u} → Prop → Sort u`. Correct Lean type is
`Quot : {α : Sort u} → (α → α → Prop) → Sort u`, i.e. the second binder must be
`Pi(Default, a, BVar(0)/*α*/, Pi(Default, b, BVar(1)/*α*/, Sort 0))`. The relation
binder's domain is collapsed to bare `Prop`. Also registered as `Axiom`, not
`Quotient(QuotVal{kind: Type})`, so `env.get_quotient_val("Quot")` returns None
for it and the reduction path would not recognize it (harmless here since Quot
itself has no reduction rule, but inconsistent with the QuotVal-driven design).

### No types at all for Quot.mk / Quot.lift / Quot.ind

Nowhere in the kernel are these Pi-types built. `check/functions.rs:183-187`:

```rust
pub(super) fn check_quotient_val(env: &mut Environment, qv: &QuotVal) -> Result<(), KernelError> {
    let mut tc = TypeChecker::new(env);
    tc.ensure_sort(&qv.common.ty)?;
    Ok(())
}
```

i.e. the kernel accepts **any well-sorted type the caller supplies** for a
quotient primitive. Lean's kernel (src/kernel/quot.cpp, `add_quot`) constructs
all four types itself and never trusts input. For the brief's TCB this is a
critical gap: a malicious/buggy export file could declare
`Quot.lift : True` with `kind: Lift` and the checker would accept it, then the
reduction rule would fire on it.

### oxilean-std Setoid-layer types are placeholders

`part2.rs:740-775` (`Quotient.lift`): the "respects the relation" hypothesis is
encoded with domain `prop()` instead of `a ≈ b` / `r a b`:

```rust
pi_implicit("a", bvar(3), pi_implicit("b", bvar(4),
    pi(prop(), app(app(var("Eq"), app(bvar(5), bvar(2))), app(bvar(5), bvar(1)))))),
```

Same for `Quotient.sound` (:776-801, hypothesis domain is `prop()`).
These are demonstrably not the real types (also `Setoid : Type → Type 1` with no
universe polymorphism). Not kernel-relevant but shows the general "approximate
types" pattern.

## 3. Quot.lift computation rule

### Real path — reduce/functions.rs:354-399

```rust
pub(super) fn try_reduce_quot(name: &Name, args: &[Expr], env: &Environment) -> Option<Expr> {
    let qv = env.get_quotient_val(name)?;
    match qv.kind {
        QuotKind::Lift => {
            if args.len() < 6 { return None; }
            let quot_val = &args[5];
            let quot_head = get_app_fn(quot_val);
            if let Expr::Const(mk_name, _) = quot_head {
                if let Some(mk_qv) = env.get_quotient_val(mk_name) {
                    if mk_qv.kind == QuotKind::Mk {
                        let mk_args: Vec<&Expr> = get_app_args(quot_val);
                        if mk_args.len() >= 3 {
                            let a = mk_args[2];
                            let f = &args[3];
                            return Some(Expr::App(Box::new(f.clone()), Box::new(a.clone())));
                        }
                    }
                }
            }
            None
        }
        ...
```

Hooked into the main whnf at `reduce/types.rs:1135-1143` (inside
`Reducer::whnf_core`, `Expr::App` arm):

```rust
if let Expr::Const(name, levels) = &head_whnf {
    if let Some(reduced) = self.try_reduce_recursor(name, levels, &args, env, depth) { ... }
    if let Some(reduced) = try_reduce_quot(name, &args, env) {
        return self.whnf_env_depth(&reduced, env, depth + 1);
    }
}
```

And this Reducer IS the one used by the trusted checker:
- `infer/types.rs:758` `TypeChecker { reducer: Reducer, ... }`; `:841-842`
  `pub fn whnf(...) { self.reducer.whnf_env(expr, self.env) }`.
- `def_eq/types.rs:1262-1264` `DefEqChecker { reducer: Reducer, ... }`; `:1314-1316`
  `is_def_eq_core` whnf's both sides via `reducer.whnf_env`.

So the lift rule **is on the definitional-equality path**. Positional analysis vs
Lean quot.cpp (`lift: mk_pos = 5, arg_pos = 3`; `Quot.mk` fully applied = 3 args,
element at index 2):
- args[5] = q ✔, args[3] = f ✔, mk_args[2] = a ✔. **Positions correct for Lift.**
- Recognition of `Quot.mk` is via env lookup + `QuotKind::Mk` — name-independent,
  robust. ✔

### Bugs in the real path (both Lift and Ind)

1. **Major premise not whnf'd.** Lean does `expr mk = whnf(args[mk_pos])`. Here
   `get_app_fn(&args[5])` inspects the raw argument. `Quot.lift f h (id (Quot.mk r a))`
   or any q that reduces to a Quot.mk will NOT fire. Incompleteness: valid proofs
   relying on this defeq will be rejected.
2. **Over-application args dropped — UNSOUND.** Lean re-applies
   `args[mk_pos+1 ..]` to the result. Here the caller (reduce/types.rs:1140)
   discards everything: for `Quot.lift (β:=A→B) f h (Quot.mk r a) x` (7 args),
   whnf returns `f a` instead of `f a x`. whnf producing a wrong term feeds
   directly into `is_def_eq` — this can equate non-equal terms or accept
   ill-typed proofs. (Recursor path `try_reduce_recursor` at reduce/types.rs:1156+
   appears to have the analogous issue via `instantiate_recursor_rhs`; out of
   scope here but same pattern.)
3. Universe levels of the head const are ignored — acceptable for this rule
   (Lean also doesn't need them for quot iota), noted for completeness.

### Toy path — quotient/functions.rs:77-92 (NOT used by the checker)

```rust
pub fn reduce_quot_lift(args: &[Expr]) -> Option<Expr> {
    if args.len() < 3 { return None; }
    match &args[2] {                       // lift modeled with 3 explicit args
        Expr::App(head, a) => {
            if let Expr::Const(name, _) = head.as_ref() {
                if *name == Name::str("Quot.mk") {   // mk modeled with 1 arg
                    return Some(Expr::App(Box::new(args[0].clone()), a.clone()));
                }
            } ...
```

- Assumes `Quot.lift f h q` (3 args, no α/r/β) and `Quot.mk a` (1 arg, no α/r).
  Real export terms are fully applied (6 / 3 args) → never matches real data.
- `Name::str("Quot.mk")` builds `Str(Anonymous, "Quot.mk")` — a SINGLE component
  (name/types.rs:722-724). A lean4export reader builds hierarchical
  `Str(Str(Anonymous,"Quot"),"mk")` (cf. `Name::from_str`, name/types.rs:742-760).
  These are unequal, so even the arity-3 pattern would not match.
- `QuotientKernel`, `QuotientNormalizer`, `QuotientReducer` (quotient/types.rs:182,
  224, 419) are referenced from nowhere outside the quotient module (grep
  confirmed). Dead weight in the TCB.
- Yet `lib.rs:376-378` re-exports the toy API as the public surface:
  `pub use quotient::{check_equivalence_relation, check_quot_usage,
  is_quot_type_expr, quot_eq, reduce_quot_lift, QuotUsageKind, QuotientType};`
  — misleading (e.g., `quot_eq` at quotient/functions.rs:193-209 "checks"
  `Quot.mk r a = Quot.mk r b` by testing whether `r a b` is *syntactically equal*
  to `r b a`, which is nonsense as a kernel judgment).

## 4. Quot.ind reduction — broken

`reduce/functions.rs:377-396`:

```rust
QuotKind::Ind => {
    if args.len() < 6 { return None; }     // BUG: Quot.ind has 5 args
    let quot_val = &args[5];               // BUG: q is at index 4
    ...
    let a = mk_args[2];
    let h = &args[4];                      // BUG: minor premise mk is at index 3
    return Some(Expr::App(Box::new(h.clone()), Box::new(a.clone())));
```

Lean: `Quot.ind : {α : Sort u} → {r : α → α → Prop} → {β : Quot r → Prop} →
((a : α) → β (Quot.mk r a)) → (q : Quot r) → β q` — 5 args; quot.cpp uses
`mk_pos = 4, arg_pos = 3`.

Consequences:
- A correctly fully-applied `Quot.ind α r β mk (Quot.mk r a)` (5 args) **never
  reduces** — the `< 6` guard bails. Completeness failure: any proof relying on
  ind iota (ubiquitous in mathlib exports) gets a spurious defeq failure.
- If the term is over-applied by one (6 args: `... mk q extra` — possible only if
  β's codomain were a function, which for Prop-valued β can happen via defeq
  tricks, or in malformed input), the code reads `extra` as the Quot.mk term and
  `q` as the minor premise, producing the garbage reduction `q a_extra`. Combined
  with the checker trusting supplied quotient types (§2), this is exploitable.
- No test exists that would catch this (see §7).

Toy path `reduce_quot_ind` (quotient/functions.rs:418-434) models ind with 2 args
(`args[1]` = mk-term) — also wrong, also unused.

## 5. Once-only + Eq-presence checks — absent

- **Once-only:** the only guard is generic duplicate-name rejection in
  `Environment::add_constant` (env/types.rs:250-257). There is no
  "quotients initialized" flag (Lean: `environment::quot_initialized` /
  `quot_init`), nothing enforcing that all four are added together, in order,
  with kernel-known types, exactly once. You can add `QuotVal{name: Foo,
  kind: Lift}` alone with an arbitrary type and the reducer will happily use it.
- **Eq check:** Lean's `add_quot` first runs `check_eq_type` (Eq must be an
  inductive of the exact expected form) because Quot.sound's statement needs Eq.
  No such check exists anywhere in oxilean-kernel. `is_eq_relation`
  (quotient/functions.rs:69-75) is only a syntactic name test on expressions,
  unrelated to environment validation. grep for `check_eq`/Eq-existence: no hits.

## 6. #QUOT-style public API for an export reader

What a lean4export reader would have to call today:

- `oxilean_kernel::check_constant_info(env, ConstantInfo::Quotient(QuotVal {
   common: ConstantVal { name, level_params, ty }, kind }))` — exported at
  lib.rs:355. Validation = `ensure_sort(ty)` only (§2).
- Or `Environment::add_constant` directly (no checking at all).
- The reader must itself: build all four canonical Pi-types (universe params u/v,
  BVar plumbing), map names→QuotKind, add `Quot.sound` separately as an Axiom,
  and enforce ordering/completeness. I.e., **the un-trusted reader would own the
  most security-critical construction**, inverting the brief's TCB design.
- The legacy `Declaration` enum (env/types.rs:398+) has NO Quotient variant
  (Axiom/Definition/Theorem/Opaque/...), so the legacy `env.add`/
  `check_declaration` path cannot express #QUOT at all.
- `oxilean-kernel/src/export/` is a proprietary binary module serializer
  (`export_environment`/`import_module`, MAGIC_NUMBER 0x4F584C4E) — NOT a
  lean4export text reader. **No `oxilean-export` crate exists in the workspace**
  (14 crates: oxilake, oxilean, -build, -cli, -codegen, -doc, -elab, -kernel,
  -lint, -meta, -parse, -runtime, -std, -wasm). Note: `import_module`
  (export/functions.rs:31-52) silently *skips* constants already present instead
  of erroring — anti-TCB behavior if ever reused for verification input.

## 7. Test coverage

Existing (all in `src/quotient/functions.rs` unless noted):
- `test_quot_lift_reduction` (:248) — toy 3-arg lift, 1-arg mk.
- `test_quot_lift_too_few_args` (:261), `test_is_quot_mk` (:265),
  `test_quot_mk_arg` (:271), `test_check_equivalence_relation` (:277),
  `test_is_eq_relation` (:293), `test_quotient_kernel_*` (:302-325),
  `test_quot_eq_*` (:327-349), `test_is_quot_type_expr_*` (:351-375),
  `test_check_quot_usage_*` (:377-404), `extended_tests::test_reduce_quot_ind`
  (:570, toy 2-arg model), `test_try_reduce_quot_full_lift` (:582),
  builder/collect/cache/validate tests (:592-695), normalizer tests (:934-1024).
- Large fractions are pure filler: `tests_padding_infra` (:1026),
  `tests_extra_iterators` (:1109), `tests_padding2` (:1130) test StatSummary,
  SmallMap, TokenBucket, SimpleDag, WindowIterator, StringPool… nothing quotient.
- `check/functions.rs:898-910 test_check_constant_info_quotient` — registers one
  QuotVal (kind **Type**, ty `Sort 0`) and asserts env membership. The single
  test touching `ConstantInfo::Quotient`.
- `tests/prop_tests.rs` — zero quotient content.

Missing (all high-value; several would immediately expose the §4 bug):
1. End-to-end: register 4 QuotVals with canonical types; assert
   `TypeChecker::is_def_eq(Quot.lift α r β f h (Quot.mk α r a), f a)`.
2. Same for `Quot.ind α r β mk (Quot.mk α r a) ≡ mk a` (currently FAILS —
   off-by-one, §4).
3. Over-application: `Quot.lift ... (Quot.mk ...) x` reduces to `f a x`
   (currently drops `x` — unsound, §3).
4. Major premise needing whnf: `Quot.lift f h ((fun q => q) (Quot.mk r a))`.
5. Negative: q not headed by Quot.mk → no reduction; partial application → no
   reduction.
6. Environment hygiene: second #QUOT rejected; QuotVal with non-canonical type
   rejected; #QUOT before Eq rejected.
7. Hierarchical-name round trip: decls named via `Name::from_str("Quot.mk")`
   (not `Name::str`) still reduce (the env-keyed path passes this; the toy path
   fails; a regression test pins the right behavior).
8. Determinism/property tests over quot reduction for the WASM parity gate.

## Cross-references / misc findings

- `axiom/types.rs:1280-1283`: a "safe axioms" set inserts
  `Name::str("Quot.mk"/"Quot.lift"/"Quot.ind"/"Quot.sound")` — single-atom names
  again; would not match export-derived hierarchical names.
- `builtin/functions.rs:1097`: `"propext" | "Quot" | "Classical.choice" =>
  BuiltinKind::Axiom` — Quot classified as an axiom (Lean models it as a distinct
  quot declaration kind); `propext`'s constructed type (:466-476) is also wrong
  (`{a b : Prop} → Prop` instead of `(a ↔ b) → a = b`) — same "approximate type"
  pattern as Quot.
- `proof/functions.rs:22` lists `"Quotient.sound"` among recognized axiom names
  (Setoid layer, cosmetic).
- `reduce/types.rs:403-404` `ReductionRule::Quot` enum variant exists for
  trace/stats; display in reduce/reductionrule_traits.rs:21 and
  trace/reductionrule_traits.rs:22.
- `quotient/` module size: 2649 lines total; genuinely quotient-relevant logic is
  under ~200 lines. For a <= minimal-TCB kernel this module should be removed or
  reduced to nothing (the real logic lives in reduce/ + declaration/ + env/).
- `check_quot_usage`'s "Quot.ind motive must be Prop" (quotient/functions.rs:155-168)
  encodes a rule the kernel doesn't actually need as a separate check (β's type
  already forces it); it is dead code w.r.t. the checker.

## Gap analysis vs brief (hard part 1)

| Brief requirement | Status |
|---|---|
| Quot type former modeled | Partial — QuotKind::Type exists; only construction site has wrong type and wrong ConstantInfo kind |
| Quot.mk | Partial — QuotKind::Mk recognized by reducer; never constructed, type never validated |
| Quot.lift + computation rule | Partial — iota positions correct, on def-eq path; no whnf of major, over-application dropped (unsound), zero tests |
| Quot.ind + computation rule | **Broken** — off-by-one arg positions; fully-applied ind never reduces; mis-fires on 6-arg form |
| Quot.sound as separate axiom | Missing — never added anywhere (only Setoid-level `Quotient.sound` in oxilean-std) |
| #QUOT add-once + Eq check | Missing entirely |
| Kernel-owned canonical types (TCB) | Missing — `check_quotient_val` trusts caller-supplied types |
| Export-reader entry point | Missing — no oxilean-export crate; kernel `export/` is an unrelated serializer |

## Recommended implementation order

1. **Fix `try_reduce_quot` Ind case** (reduce/functions.rs:377-396): `mk_pos=4`,
   minor premise at `args[3]`; guard `args.len() >= 5`.
2. **Fix both rules' generic behavior**: whnf `args[mk_pos]` before matching
   Quot.mk (needs the Reducer, so move the fn onto `Reducer` or pass a whnf
   closure), and re-apply `args[mk_pos+1..]` to the result (`mk_app`).
3. **Implement `Environment::add_quot()`** mirroring Lean quot.cpp: verify Eq
   exists with expected inductive form; construct the four canonical types
   in-kernel (universe-polymorphic u/v); add the 4 `ConstantInfo::Quotient`
   entries atomically; set a once-only `quot_initialized` flag consulted by
   `add_constant` for any further `Quotient` entries. Expose it as the #QUOT
   handler for the future oxilean-export reader; also add `Quot.sound` as an
   axiom with kernel-constructed type (or leave to the export file but validate
   its type against the canonical form).
4. **Harden `check_quotient_val`**: compare supplied `common.ty` against the
   kernel-derived canonical type (alpha/level-aware defeq), never trust input.
5. **Fix/remove the wrong `Quot` axiom** in `add_core_axioms`
   (builtin/functions.rs:486-505).
6. **Add the §7 missing tests**, incl. a def-eq level integration test
   (`Quot.lift f h (Quot.mk r a) ≡ f a`) using hierarchical names.
7. **Quarantine the toy `quotient/` module**: drop the lib.rs:376-378 re-exports,
   delete or move filler types out of the kernel crate; it inflates the TCB and
   its semantics (e.g. `quot_eq`) are wrong.

## Key files

- crates/oxilean-kernel/src/reduce/functions.rs:354-399 — real quot iota (Lift ok-ish, Ind broken)
- crates/oxilean-kernel/src/reduce/types.rs:1111-1144 — whnf_core hook (drops over-application)
- crates/oxilean-kernel/src/declaration/types.rs:119-130, 1300-1308 — QuotKind/QuotVal (correct model)
- crates/oxilean-kernel/src/check/functions.rs:114-116, 181-187 — check_quotient_val (trusts input types)
- crates/oxilean-kernel/src/builtin/functions.rs:486-505 — wrong `Quot` axiom type
- crates/oxilean-kernel/src/env/types.rs:250-257, 304-332, 398+ — add_constant, quot lookups, legacy Declaration (no Quot variant)
- crates/oxilean-kernel/src/infer/types.rs:758, 841-842, 868-869 — TypeChecker uses quot-aware Reducer
- crates/oxilean-kernel/src/def_eq/types.rs:1262-1316 — DefEqChecker whnf path
- crates/oxilean-kernel/src/quotient/{functions.rs,types.rs} — toy module (~2650 SLoC, mostly filler)
- crates/oxilean-kernel/src/lib.rs:355, 376-378 — public API exports
- crates/oxilean-kernel/src/name/types.rs:711-760 — hierarchical Name; `Name::str` single-atom pitfall
- crates/oxilean-std/src/env_builder/functions/part2.rs:722-802 — Setoid-layer Quotient axioms
- TODO.md:78-81 — stale claims (quot.rs does not exist; "3 built-ins" wrong)
