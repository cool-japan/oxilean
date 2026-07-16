# Audit: Inductive types, recursor derivation, iota reduction, strict positivity in oxilean-kernel

Repo: oxilean (branch 0.1.3). Crate audited: `crates/oxilean-kernel`.
Date: 2026-07-12. Scope = brief hard-part (4): recursors for nested/mutual inductives, iota
reduction, re-deriving recursors, strict positivity re-check; plus adjacent K-reduction and
literal iota (brief item 3/5 overlap).

Context: the workspace has NO `oxilean-verify` and NO `oxilean-export` crate. The kernel's
`src/export/` module is an internal Environment (de)serializer, not a lean4export reader.
There is no `fuzz/` directory anywhere in the repo. Everything below is about the kernel
primitives the future verify/export crates would sit on.

The kernel modules named in the brief exist but are SplitRS-generated ("🤖 Generated with
SplitRS" headers) and heavily padded with unrelated utility types (StatSummary, SmallMap,
SimpleDag, TokenBucket, ...) that inflate SLoC; e.g. `inductive/types.rs` is 1311 lines of
which maybe 450 are inductive logic. Total kernel ≈146k lines but the doc claims "~3,500
SLOC TCB" (lib.rs:10).

---

## 1. Does the kernel RE-DERIVE recursors?

**Answer: there are two disjoint paths; the checking path does NOT re-derive, and the
derivation path that exists is unchecked, non-Lean-shaped, and buggy for anything beyond
0-param/0-index enums.**

### Path A — derivation (exists, partially correct)
`crates/oxilean-kernel/src/inductive/types.rs`

- `InductiveType` (types.rs:586-605): fields `name, univ_params, num_params, num_indices,
  ty, intro_rules, recursor (auto = name ++ "rec", :616), is_nested: bool, is_prop: bool`.
  `is_prop` is CALLER-SUPPLIED (default false), never computed from `ty`.
- `InductiveType::to_constant_infos()` (types.rs:671-676) returns
  `(ConstantInfo::Inductive, Vec<ConstantInfo::Constructor>, ConstantInfo::Recursor)`.
- `make_recursor_val` (types.rs:716-753):
  - builds `RecursorRule { ctor, nfields, rhs }` per constructor via `build_recursor_rhs`;
  - K flag: `let k = self.is_prop && self.intro_rules.len() <= 1;` (types.rs:732) —
    WRONG vs Lean: Lean requires exactly 1 ctor AND 0 fields (all args params). Here
    `Empty` (0 ctors, Prop) gets k=true, and a 1-ctor Prop with fields also gets k=true.
    (Flag is inert anyway; see §5.)
  - motive universe: prepends fresh `u_1` level param unless `is_prop`
    (types.rs:733-736); if `is_prop`, motive sort is `Level::zero()`
    (types.rs:790-796). **There is NO large-elimination / subsingleton-elimination
    decision**: Prop inductives never get large elimination (breaks `Eq.rec`, `False.rec`,
    `Acc.rec`, `And.rec` semantics), and since `is_prop` is caller-set, a caller can set
    `is_prop=false` on a Prop-sorted inductive (e.g. `Or`) and get an unrestricted motive
    → unsound large elimination. Nothing re-checks.
- `build_recursor_type` (types.rs:784-867): shape
  `Π params.. motive minors.. indices.. (t : T params indices), motive indices t`.
  - **BUG (off-by-one)**: `ind_applied_major` (types.rs:797-808) computes param BVars as
    `BVar(ni + nminors + 1 + np - k)` and index BVars as `BVar(ni - k)`. In the DOMAIN of
    the `t` binder the correct values are `BVar(ni + nm + np - k)` and `BVar(ni - 1 - k)`.
    Both are one too high. Concrete counterexample: np=1, ni=0, nm=1 → param_0 emitted as
    BVar(3), correct is BVar(2). So the generated recursor TYPE is wrong for any inductive
    with parameters or indices. Only np=0/ni=0 (Bool, Nat) is exercised by tests, which is
    why this passes CI.
  - `Expr::Const(self.name.clone(), vec![])` (types.rs:798) — the inductive is referenced
    with an EMPTY universe-level list. Broken for universe-polymorphic inductives
    (List.{u} etc.). Same for the ctor in `build_minor_type` (types.rs:959) and the
    recursor in `build_ih` (types.rs:1020).
  - Index binder domains (types.rs:824-836): `lift_expr_bvars(all_binders[np+k], 1+nm)`
    lifts ALL loose BVars, including references to earlier indices, which must stay
    unlifted (indices are contiguous in both telescopes). Breaks indexed families whose
    later index types depend on earlier indices.
- `build_motive_type` (types.rs:870-911): `Π indices.., T params indices → Sort v`.
  BVar arithmetic here is correct (checked by hand for np=1/ni=0 and general case).
- `build_minor_type` (types.rs:924-985):
  - **MAJOR GAP: no induction hypotheses.** The minor premise is a Pi chain over the raw
    field types ending in `motive [ret-indices] (ctor params fields)` — recursive fields
    get NO `motive field` IH binder. So the derived `Nat.rec` minor for `succ` has type
    `(y : Nat) → motive (Nat.succ y)` instead of Lean's
    `(n : Nat) → motive n → motive (Nat.succ n)`. This makes the derived recursor a
    "casesOn"-shaped TYPE while the RULE RHS (below) applies the minor to `(field, IH)` —
    i.e. the type and the computation rule are mutually inconsistent. Any real Lean
    export using `Nat.rec` would fail to typecheck against this derived type, and the
    derived rule application `minor field ih` is ill-typed w.r.t. the derived minor type.
  - `return_indices` extraction (types.rs:936-957): peels np+nf Pis from the ctor type,
    strips the ctor codomain app args, takes the last `ni` — reasonable — but then lifts
    each by `(1 + cidx)` with `lift_expr_bvars` (cutoff 0), which wrongly shifts
    references to constructor FIELDS (fields are at the same relative depth in the minor
    conclusion). E.g. `Vec.cons : … (n : Nat) … → Vec α (Nat.succ n)` — the `n` inside
    the return index would be shifted by 1+cidx. Broken for indexed families.
  - Field domain types (types.rs:974-983): same lift-cutoff bug — `lift_expr_bvars(
    field_ty, 1+cidx)` shifts references to EARLIER FIELDS (dependent fields such as
    `(x : A) (p : P x)`), which must not be shifted. Broken for dependent ctor fields.
- `build_recursor_rhs` (types.rs:1000-1016) + `build_ih` (types.rs:1019-1032):
  - RHS is an open term over the subst `[params.., motive, minors.., fields..]` consumed
    by `instantiate_rev` (convention documented at types.rs:986-999 and consistent with
    `src/instantiate/functions.rs:24-45` — verified).
  - `minor_cidx = BVar(nf+nm-1-cidx)`, `field_j = BVar(nf-1-j)`, params/motive/minors in
    `build_ih` all check out arithmetically. For each field with `is_rec` an IH
    `T.rec params motive minors field` is appended. This part is internally consistent
    with `instantiate_recursor_rhs` and validated by tests for Bool and Nat
    (inductive/functions.rs:698-755).
  - **Recursive-field detection** `head_is_inductive` (types.rs:773-779) only matches a
    field whose HEAD is `Const(self.name)`. Reflexive fields `(Nat → T)` return false →
    no IH and no lambda-wrapped IH (Lean produces `∀ n, motive (f n)`). So reflexive
    inductives get a wrong (cases-like) rule. `InductiveVal.is_reflexive` is always set
    `false` (types.rs:691).
- `collect_field_info` (types.rs:756-771): fields = Pi domains after skipping num_params
  binders. `count_pi_args` (functions.rs:14-19) counts all Pis. Fine for well-formed
  ctor types.

### Path B — checking (what a verifier would call): NO re-derivation
`crates/oxilean-kernel/src/check/functions.rs`

- `check_constant_info(env, ci)` (check/functions.rs:80-126) dispatches:
  - `Inductive` → `check_inductive_val` (:144-151): **only** `tc.ensure_sort(iv.common.ty)`.
    No positivity, no ctor/param/index consistency, no universe checks, no `all`/mutual
    validation, nothing.
  - `Constructor` → `check_constructor_val` (:155-169): `ensure_sort(ty)` + syntactic
    "codomain head is the parent inductive" (`constructor_returns_inductive`, :190-198).
    No param-prefix check, no "recursive occurrences applied to exactly the params", no
    universe-fit check.
  - `Recursor` → `check_recursor_val` (:173-179): **only `ensure_sort(rv.common.ty)`**.
    The recursor type, rule RHSes, k flag, num_* fields are all TRUSTED as given. The
    unit test `test_check_constant_info_recursor` (check/functions.rs:858-896) even
    demonstrates acceptance of a recursor whose type is literally `Type` and whose
    `rules` vec is empty.
- `Environment::add_constant` (env/types.rs:250-257) does duplicate-name check only; raw
  insertion path with zero validation is publicly available.

**Conclusion for brief item (4) "re-derive recursors ourselves rather than trusting the
exported ones": NOT satisfied.** The derivation machinery exists but (a) is not invoked
from any checking path, (b) produces non-Lean recursor types (missing IHs), (c) has
de-Bruijn and universe-polymorphism bugs for np>0/ni>0/poly, (d) makes no large-elimination
decision. The check path plainly trusts external recursors.

### Builtin recursors are placeholders (worse than derived ones)
`crates/oxilean-kernel/src/builtin/functions.rs` (`init_builtin_env`, :18)

- Every builtin recursor (`Bool.rec` :136, `Unit.rec` :200, `Nat.rec` :296, `Eq.rec`
  :650, `Prod.rec` :752, `List.rec` :858) has:
  - placeholder TYPE `Expr::Sort(Level::zero())` (e.g. :299-301 for Nat.rec) — the
    recursor's declared type is `Prop`;
  - placeholder RHS `Expr::BVar(0)` for every rule. Under the `instantiate_rev`
    convention this is simply wrong:
    - `Nat.rec m z s Nat.zero` → subst `[m,z,s]`, BVar(0)→`s` (the SUCC minor), not `z`.
    - `Nat.rec m z s (Nat.succ n)` → subst `[m,z,s,n]`, BVar(0)→`n` (the field), not
      `s n (Nat.rec m z s n)`.
    - `Eq.rec …` (k: true at :666) → reduces to the refl FIELD `a`, not the minor.
    - `List.rec`/`Prod.rec` analogous (reduce to last field / wrong minor).
  So any consumer using `init_builtin_env` gets systematically wrong iota for the
  builtins. Only the `InductiveType::to_constant_infos`-derived Bool/Nat recursors have
  correct RHSes (proven by inductive/functions.rs tests).

---

## 2. Strict positivity

Location: `crates/oxilean-kernel/src/inductive/functions.rs`
- `check_inductive(ind: &InductiveType)` (:76-89): per ctor, (a) `returns_type` (:90-97,
  syntactic codomain-head check), (b) `check_positivity` (:98-146).
- Algorithm: `check_positivity_rec` walks the ctor type; occurrences of the inductive
  name in a Pi DOMAIN go through `check_positivity_rec_domain`, which allows direct
  `T`/`T args` heads and recurses into nested Pi domains with `positive=false`, rejecting
  `(… T …) → X` in a domain-of-domain (true negative positions).

What it gets right:
- `T → T` accepted; `(T → Bool) → T` rejected (tested: functions.rs:200-220).
- Reflexive args `(Nat → T)` are ACCEPTED (correct per Lean strict positivity), but note
  §1: the recursor derivation then mishandles them (no IH).

Gaps (all confirmed by reading, none tested):
1. **Not wired into any checking pipeline.** `check_inductive` is a standalone pub fn
   (re-exported lib.rs:368); `check_constant_info`/`check_inductive_val` never call it;
   `InductiveEnv::register_in_env` (inductive/types.rs:424-440) adds the inductive +
   ctors + derived recursor to the Environment WITHOUT calling `check_inductive`. So
   positivity is opt-in.
2. **Purely syntactic, no whnf/delta.** `mk : F T → T` with `def F X := X → Bool`
   passes (the checker sees `App(Const F, Const T)`), i.e. negativity hidden behind a
   definition is not detected. Lean's kernel whnfs constructor argument types first.
3. **Nested occurrences are accepted blindly.** `mk : List T → T` passes with no check
   that `T` sits in a strictly-positive parameter position of `List`, and no
   nested→mutual compilation happens afterwards (see §4). Any type constructor —
   including contravariant ones — is accepted as a wrapper.
4. **Mutual-blind.** The check receives a single `ind_name`; sibling types of a mutual
   family are ordinary constants to it, so negative occurrences of a sibling
   (`Even` in `Odd`'s ctor domain-of-domain… actually any position) are unchecked.
5. No check that recursive occurrences are applied to EXACTLY the uniform parameters
   (Lean's `is_valid_ind_app`); `T (f x)` in a field is accepted.
6. No universe/level checks at all here (see §6).
7. `Expr::Let` in ctor types is not traversed (falls into `_ => Ok(())`), so a negative
   occurrence hidden under a `Let` binder passes.

`InductiveError::NonStrictlyPositive` (inductive/types.rs:83) exists but the checker
returns `KernelError::InvalidInductive(String)` instead; the enum variant is dead except
in a Display test.

---

## 3. Mutual inductives

**Struct-level support only; no semantics.**
- `InductiveFamily` (inductive/types.rs:100-150): a Vec of `InductiveType` + shared
  univ params with helpers (`singleton`, `len`, `type_names`, `all_constructor_names`,
  `find_type`, `total_constructors`). Nothing consumes it beyond its own unit tests
  (inductive/functions.rs:604-667).
- No family-level `check_*`, no positivity across siblings, no recursor generation with
  multiple motives: `make_recursor_val` hardcodes `num_motives: 1` and
  `all: vec![self.name]` (types.rs:744-747). `RecursorVal.num_motives`/`all` fields exist
  (declaration/types.rs:197-220) and `instantiate_recursor_rhs`
  (reduce/functions.rs:28-63) does loop over `num_motives` motives when building the
  subst, so ACCEPTED external mutual recursors would iota-reduce correctly if their rule
  RHSes use the right convention — but nothing in the kernel can create or validate them.
- `InductiveEnv::add`/`register_in_env` handle one type at a time; `Environment` has no
  add-family API. grep for "mutual" in kernel src finds only comments/summary flags
  (`InductiveTypeInfo.is_mutual` is a display-only flag, types.rs:1049).

**Verdict: mutual inductives are NOT supported end-to-end.** Declaration form: absent
(only the passive `InductiveFamily`). Recursor generation with multiple motives: absent.
Iota for mutual: works only for externally-supplied trusted recursors.

## 4. Nested inductives

- Only metadata: `InductiveType.is_nested: bool` (types.rs:602, settable via builder
  :1177) and `InductiveVal.num_nested: u32` (declaration/types.rs:1032, always 0 from
  `make_inductive_val`, types.rs:688).
- There is NO nested→mutual compilation, no auxiliary type generation, no un-nesting of
  `List T` occurrences, nothing resembling Lean's `mkNestedRecursors` /
  `elimNestedInductives`. grep across kernel for nested-related logic: none.
- Combined with §2 gap 3, a nested inductive would be accepted (positively unverified)
  and its derived recursor would treat the `List T` field as non-recursive
  (`head_is_inductive` = false), producing a cases-like rule with no IH.

**Verdict: nested inductives NOT supported; the flag is decorative.**

---

## 5. Iota reduction in whnf

Authoritative reducer: `crate::reduce::Reducer` (reduce/types.rs:934-1189), used by
`TypeChecker::whnf` (infer/types.rs:841-843) and `DefEqChecker::is_def_eq_core`
(def_eq/types.rs:1314-1316). The `whnf/`, `reduction/`, `normalize/` modules are
parallel/legacy implementations without recursor support (whnf/functions.rs:17-25 just
wraps `Reducer::whnf`, the env-free variant). `inductive::reduce_recursor`
(inductive/functions.rs:150-152) is a stub returning `None`.

`Reducer::whnf_core` (reduce/types.rs:1083-1153) order: Let→zeta; Const→delta (hint
gated); App→beta over collected spine, then `try_reduce_nat_app`, `try_reduce_int_app`,
`try_reduce_recursor`, `try_reduce_quot`; Proj→`try_reduce_proj`.

`try_reduce_recursor` (reduce/types.rs:1156-1184):
- Looks up `RecursorVal` by head name, computes `major_idx = num_params + num_motives +
  num_minors + num_indices` (declaration/types.rs:222-224 — matches Lean), requires
  `args.len() > major_idx`, whnfs the major, requires its head to be a `Const` that
  `env.is_constructor`, finds the rule by ctor name, instantiates.
- `instantiate_recursor_rhs` (reduce/functions.rs:28-63): subst = params ++ motives ++
  minors ++ ctor fields (skipping ctor params); levels instantiated via
  `instantiate_type_lparams` (with the empty-levels escape hatch). Convention checked ✔.

Sub-question results:
1. **Recursor applied to constructor app: YES** (baseline works; tested for Bool.false
   and Nat.zero via derived recursors, inductive/functions.rs:757-847).
   - **BUG: over-application drops trailing args.** Neither `try_reduce_recursor` nor
     `whnf_core` re-applies `args[major_idx+1..]` to the instantiated RHS
     (reduce/types.rs:1136-1139 returns whnf of the bare rhs). A recursor whose motive
     returns a function type, applied further, silently LOSES arguments → wrong results
     presented as reduced terms. Same bug in `try_reduce_quot` usage (extra args beyond
     the 6th are dropped, reduce/types.rs:1140-1142).
2. **NatLit as constructor app for Nat.rec: NO.** A `Lit(Nat(n))` major has no Const
   head → `try_reduce_recursor` returns None; there is no `Lit → Nat.succ/Nat.zero`
   expansion anywhere. The bridging that exists is one-directional:
   `try_reduce_nat_app` folds `Nat.succ (Lit n)` → `Lit (n+1)` and `Nat.zero`(applied!)
   → `Lit 0` (reduce/functions.rs:69-76). Note the `"Nat.zero" => Lit 0` arm is only
   reachable when `Nat.zero` appears as an App HEAD with ≥1 args, which never happens;
   bare `Const Nat.zero` never becomes `Lit 0`, so `is_def_eq(Lit 0, Nat.zero)` is
   FALSE (def_eq falls through (Lit, Const) → try_lazy_delta → None/None → false,
   def_eq/types.rs:1350,1356,1363-1397). Literal/ctor def-eq bridging: missing.
3. **String literals for String.rec: NO.** No `String` inductive, no `String.rec`, no
   `StrLit → String.mk (List.cons (Char.ofNat …) …)` expansion. Only
   `String.length/append/beq` literal folds (reduce/functions.rs:198-218).
4. **K-like reduction: NO.** `RecursorVal.k` (declaration/types.rs:214) is stored (set
   true for builtin Eq.rec, builtin/functions.rs:666) but grep shows it is never READ by
   Reducer, DefEqChecker, or TypeChecker. `Eq.rec m mr h` with `h` a stuck (non-refl)
   proof of `a = a` will not reduce. Proof irrelevance
   (`is_proof_irrelevant_eq`, def_eq/types.rs:1511-1542) recovers some but not all cases
   (it equates the PROOFS, not the recursor APPLICATION with its reduct), and its
   `quick_infer_type` (:1454-1504) bails on BVar/FVar so it fails inside open terms.

Additional reducer notes:
- Nat literal arithmetic is u64 with wrapping/saturating semantics: `m + n`, `m * n`
  (reduce/functions.rs:86,93 — overflow panics in debug, silently wraps in release),
  `Nat.pow` uses `m.pow(n as u32)` (:127 — panics/wraps on overflow),
  `shiftLeft` returns None for shift ≥64 (:183-186, leaves expr stuck). **This is not
  the brief's arbitrary-precision bignum** (brief item 3) and wrapping is a soundness
  hole (2^64 truncation could equate unequal Nats).
- `Reducer.cache: HashMap<Expr, Expr>` (reduce/types.rs:938) is shared between the
  env-free `whnf` and env-aware `whnf_env` paths (whnf_with_depth :1002 and
  whnf_env_depth :1076-1080 read/write the same map) → env-free results can shadow
  env-aware reduction (under-reduction, incompleteness rather than unsoundness).
- Depth cap `max_depth = 10000` silently returns the unreduced expr (:969-971) rather
  than reporting "unsupported/deep" — a verifier would misclassify.

Quotient (adjacent, brief item 1): `try_reduce_quot` (reduce/functions.rs:354-399)
implements `Quot.lift f h (Quot.mk r a) ↦ f a` and `Quot.ind … (Quot.mk r a) ↦ h a`
with hardcoded arities (6 rec args, mk arity 3 — matches Lean's argument counts), gated
on `QuotVal` lookups. Over-application dropped as noted.

---

## 6. Universe constraints on inductive declarations

**Essentially absent.**
- `check_inductive_val` (check/functions.rs:144-151): only `ensure_sort(iv.common.ty)`.
- `check_constructor_val` (:155-169): `ensure_sort(cv.common.ty)` (i.e. the ctor type is
  *some* Pi into *some* sort) + codomain-head check. It does NOT:
  - compute each field's universe and require `field_level ≤ inductive_level` (unless
    the inductive is in Prop) — the classic impredicativity/paradox gate;
  - handle the Prop special case (fields unrestricted when the inductive lands in Prop);
  - check that params of the ctor telescope match the inductive's param telescope
    (types AND count — `num_params` is trusted metadata);
  - check `num_indices` against the inductive's type telescope;
  - check level-param lists of ctor/rec equal the inductive's (`univ_params_compatible`
    exists at check/functions.rs:305-311 but is only used in tests).
- The derivation path's motive-universe handling is described in §1 (no large-elim
  decision, caller-controlled `is_prop`).
- `TypeChecker::infer_type` Pi case uses `Level::imax` correctly (infer/types.rs:917-924)
  so `ensure_sort` gives the right sort for the given types, but that does not implement
  the inductive-specific universe rule.

---

## 7. Public API an export reader would call

Three entry points today (all in `oxilean_kernel`):

A. Legacy/derivation API (re-exported lib.rs:368):
```rust
let ind = InductiveTypeBuilder::new()
    .name(n).univ_params(us).num_params(p).num_indices(i)
    .ty(sort_expr).intro_rule(cname, cty)... .is_prop(b).build()?;   // inductive/types.rs:1127-1200
check_inductive(&ind)?;                       // OPTIONAL, positivity — functions.rs:76
let mut ienv = InductiveEnv::new();
ienv.register_in_env(&ind, &mut env)?;        // derives ctors + rec, adds all; NO checks — types.rs:424
```
Derives the recursor automatically (with all §1 defects). Does NOT typecheck ctor types,
does not call `check_inductive` internally, single-type only, `is_prop` trusted.

B. Lean4-style checked API (re-exported lib.rs:355):
```rust
check_constant_info(&mut env, ConstantInfo::Inductive(InductiveVal{..}))?;   // sort check only
check_constant_info(&mut env, ConstantInfo::Constructor(ConstructorVal{..}))?; // sort + codomain head
check_constant_info(&mut env, ConstantInfo::Recursor(RecursorVal{..}))?;     // sort check only (TRUSTS rules)
```
This is the path shaped like a lean4export consumer (InductiveVal/ConstructorVal/
RecursorVal mirror Lean's kernel records incl. `all`, `num_motives`, `k`), but it
performs almost no validation and never re-derives.

C. Raw: `env.add_constant(ci)` (env/types.rs:250) — duplicate-name check only.

Environment queries used by reduction: `find`, `is_constructor`, `get_constructor_val`,
`get_recursor_val`, `get_inductive_val`, `get_quotient_val` (env/types.rs:263-333).

**No API takes a mutual family; none re-derives a recursor from an
InductiveVal+ConstructorVals.** For the brief, a new
`add_inductive_family(params, indices, ctors) -> derives rec, checks positivity+universes`
entry point must be written; today's pieces cannot be composed into it without fixing §1.

---

## 8. Test coverage and gaps

Present (all in-module unit tests):
- `inductive/functions.rs`: builder round-trips; positivity accept/reject for two toy
  cases (:200-220, :547-555); recursive detection; `to_constant_infos` shape for Nat
  (:249-291); `register_in_env` for Bool (:293-320); recursor RHS shape for
  Bool.false/true and Nat.zero/succ (:698-755); END-TO-END IOTA for exactly two cases:
  `Bool.rec … Bool.false` (:757-794) and `Nat.rec … Nat.zero` (:796-847). Both np=0,
  ni=0, non-poly, non-Prop.
- `reduce/functions.rs`: beta/zeta/whnf basics; Nat/Int/String literal folds; quotient
  none; `check/functions.rs`: declaration checks incl. the test that PROVES external
  recursors are accepted unvalidated (:858-896).
- `tests/prop_tests.rs` (proptest): whnf idempotence/fixed points, def_eq reflexivity,
  substitution round-trips — nothing touching recursors/inductives.

Missing tests (each corresponds to a live bug or absent feature above):
- iota through the RECURSIVE minor (Nat.succ case end-to-end) — would expose the
  minor-type/RHS inconsistency if typechecked.
- derived recursor for parameterized (List) or indexed (Vec/Eq) inductive — would
  expose the build_recursor_type off-by-ones and lift-cutoff bugs.
- universe-polymorphic inductive recursor — would expose empty `Const(_, vec![])`.
- builtin `Nat.rec`/`Eq.rec`/`List.rec` reduction — would expose the placeholder BVar(0)
  RHSes and `Sort 0` types immediately.
- mutual family (Even/Odd) anything; nested (`Tree | node : List Tree → Tree`) anything.
- K-reduction (`Eq.rec` on a stuck refl-typed proof); proof-irrelevance interplay.
- Nat literal iota (`Nat.rec … (Lit 5)`), `Lit 0 ≟ Nat.zero` def-eq, String.rec.
- recursor over-application (motive returning a function).
- positivity: nested-under-definition (`def F X := X → Bool`), sibling mutual
  negativity, Let-hidden occurrence, `T (f x)` non-uniform recursive application.
- large-elimination gating (Prop single-ctor vs multi-ctor).
- No fuzz targets anywhere in the repo (brief requires cargo-fuzz on the export reader;
  neither reader nor fuzz exists).

---

## Priority fix list for the verify effort (this area)

1. Implement a real `add_inductive_family` kernel entry point mirroring Lean's
   `Environment.addInductive`: telescope-check ctor types against the inductive
   signature (params prefix, indices, universe rule with the Prop special case),
   positivity with whnf and mutual/sibling awareness, and reject nested inductives
   explicitly as `unsupported("nested inductive")` initially.
2. Rewrite recursor derivation: add IH binders to minor premises (incl. Pi-wrapped IHs
   for reflexive fields), fix the `build_recursor_type` off-by-ones, thread universe
   levels (`Const(name, univ_params-as-levels)`), implement the large-elimination
   decision (Prop + ≤1 ctor + all-args-in-Prop-or-params ⇒ eliminate to any Sort, else
   Prop-motive), and compute `k` per Lean (1 ctor, 0 fields, Prop).
3. In `check_constant_info`, refuse `ConstantInfo::Recursor` from outside for the verify
   product (or re-derive + compare); at minimum typecheck each rule RHS against the
   recursor type instantiated at its ctor.
4. Fix iota: re-apply trailing args after rule instantiation (recursor AND quot);
   expand `Lit(Nat n)` to `Nat.succ (Lit (n-1))`/`Nat.zero` when a Nat recursor is
   stuck on a literal (and same for String via `String.mk`); implement K-like reduction
   gated on `rec_val.k` + def-eq of the major's type indices.
5. Replace u64 Literal::Nat with the brief's zero-dep bignum (Literal::Nat(BigUint));
   audit all `m + n`/`m * n`/`pow` folds.
6. Fix builtin env recursors or delete `init_builtin_env` from the verify TCB (an
   exported environment should define everything; builtins with wrong rules are a trap).
7. Add the missing test matrix from §8, starting with List/Vec derived-recursor
   round-trips typechecked against hand-written Lean recursor types.
