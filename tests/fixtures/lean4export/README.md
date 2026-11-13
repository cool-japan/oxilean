# lean4export Test Fixtures

## Source

- **Repository:** https://github.com/leanprover/lean4export
- **Pinned commit:** `3de59f10bc4b4a0f2de698597aeb1246caa0df0a`
- **Lean toolchain:** `leanprover/lean4:v4.32.0-rc1`
- **Format:** NDJSON v3.1.0 (see `format_ndjson.md` in the source repo)

## Download Date

2026-07-12

## File Inventory

| File | Size (bytes) | Lines | Description |
|------|-------------|-------|-------------|
| `Nat.add_succ.ndjson` | 32437 | 572 | Export of `Nat.add_succ` with full transitive dependency chain. Covers name interning, level interning, expr interning, axioms, theorems, inductives, and recursors needed to verify one concrete Lean theorem. Generated with Lean 4.27.0-rc1 (as recorded in the file's `meta` line). |
| `simple_add.ndjson` | 36629 | 643 | Export of `simple_add : 2 + 3 = 5 := rfl` (custom scratch module, Lean v4.32.0-rc1). Exercises Nat literals, OfNat instances, and `rfl` proof term. 23 declarations total. |
| `point_swap_swap.ndjson` | 21075 | 372 | Export of `point_swap_swap` theorem over a custom `Point` structure (custom scratch module, Lean v4.32.0-rc1). Exercises `structure` declarations, struct eta, field projections (`Expr.proj`), and `simp`-generated proof terms. 16 declarations total. |
| `Parity.isEven.ndjson` | 382606 | 7015 | Export of `Parity.isEven` defined via `Quotient.lift` over a custom `myRel` equivalence relation (custom scratch module, Lean v4.32.0-rc1). Exercises `quot` kernel primitive, `Quot.lift`, `Quot.ind`, and proof obligations. 181 declarations total. |
| `Tree_Forest.ndjson` | 9514 | 166 | Export of `Tree`/`Forest` mutual inductive (custom scratch module, Lean v4.32.0-rc1). One `inductive` NDJSON record bundles both types (the lean4export format for mutual inductives). Both types appear in each other's `all` field. 1 `inductive` bundle record. |
| `Tree_Forest_full.ndjson` | 71041 | 1265 | Export of `Tree`/`Forest` mutual inductive plus `Tree.size`/`Forest.size` mutual defs (custom scratch module, Lean v4.32.0-rc1). Exercises mutual inductives and mutual well-founded recursive definitions. |
| `Syntax.ndjson` | 563059 | 10040 | Export of Lean core's `Lean.Syntax` with its transitive dependencies (`lake env lean4export Init -- Lean.Syntax`, Lean v4.32.0-rc1). A nested inductive (`numNested = 2`: `Array`, then `List`) with the exported recursors `Lean.Syntax.rec` / `.rec_1` / `.rec_2`. 269 declarations total. |
| `QuotPartialApp.ndjson` | 108037 | 1930 | Export of five `rfl` theorems from `QuotPartialApp.lean` (in this directory, Lean v4.32.0-rc1). Each needs the kernel to reduce an application whose head becomes `Quot.lift` or a recursor only after a head step (a definition unfolding, a structure projection), with the major premise among the outer arguments — the point-free style of Mathlib's `Quot.map` / `Quotient.map'`. `succQ_mk`, `neg_mk` and `pred_succ` were rejected before the head-reduction fix of 2026-11-11 (the class of Mathlib's `AddCon.addGroup._proof_1`); `succQ'_mk` and `add_mk` are controls. 66 declarations total. |
| `KGuardCollision.ndjson` | 9031 | 165 | **Negative fixture**: export of `KGuardCollision.bad` from `KGuardCollision.lean` (in this directory, Lean v4.32.0-rc1), a declaration Lean's own kernel REJECTS — it was added with `debug.skipKernelTC` so that it could be exported. Its "proof" needs K-like reduction of `Eq.rec` on a variable hypothesis whose endpoints (`fun y1 y2 y3 => c` and `fun y1 y2 y3 => y3`) are not definitionally equal. Expected split: 3 verified · 0 unsupported · 1 rejected, exit code 1. Before the free-variable id fix of 2026-11-11 `oxilean-verify` reported it VERIFIED (a wrong accept, also present in the published 0.1.3). 4 declarations total. |
| `ReductionChain.ndjson` | 54355 | 992 | Export of `chain_1000` and `chain_20000` from `ReductionChain.lean` (in this directory, Lean v4.32.0-rc1): `chain n = true` by kernel evaluation (`decide +kernel`), where `chain n` is `Nat.rec` with a step that returns its induction hypothesis — one long head-reduction chain over a term that never grows. `chain_20000` needs tens of thousands of head steps; before the fix of 2026-11-11 the reducer stopped every chain after 10,000 steps and the theorem was rejected (also in the published 0.1.3). `chain_1000` is the control. 36 declarations total. |
| `SharedSubterms.ndjson` | 123360 | 2240 | Export of `tower` and `tower_eq` from `SharedSubterms.lean` (in this directory, Lean v4.32.0-rc1): sixty `let`s, each used twice by the next — a few hundred shared nodes that are 2^60 additions as a tree — and `tower = 2^60` by kernel evaluation (`decide +kernel`). Lean's kernel checks both at once. Before the accounting and sharing work of 2026-11-11 the per-declaration budget was charged for the tree: `tower` ran out of the `corpus` budget and the file replayed as 57 verified · 3 unsupported · 0 rejected. Both now check for a few thousand units of the default budget. 60 declarations total. |
| `CongruenceFirst.ndjson` | 382846 | 7018 | Export of four theorems from `CongruenceFirst.lean` (in this directory, Lean v4.32.0-rc1). Three are `rfl` proofs of `f a = f b` with `a` and `b` definitionally equal, for an `f` whose unfolding does not end in reasonable time: `same_function_equal_args` (`slow 1000000000 (k + 0) = slow 1000000000 k`, `k` opaque), `mod_stuck_divisor` (`(2^64 - 1) % USize.size = (2^64 - 1) % 2 ^ System.Platform.numBits`, the comparison inside Lean core's `UInt64.toUSize_ofNatClamp_of_le`) and the control `short_fuel`. Lean's kernel compares the arguments before it unfolds `f`; before 2026-11-11 `oxilean-verify` unfolded first and the first two ran out of budget. **The fourth is a negative control**: `CongruenceFirst.bad : slow 5 k = slow 6 k`, a declaration Lean's own kernel REJECTS (added with `debug.skipKernelTC`). Expected split: 184 verified · 0 unsupported · 1 rejected, exit code 1. The only fixture with `opaque` records. 185 declarations total. |
| `NestedReduction.ndjson` | 122374 | 2237 | Export of three theorems from `NestedReduction.lean` (in this directory, Lean v4.32.0-rc1) about `count n acc`, which counts `n` down by structural recursion: `deep` (`count 400 0 = 400`) and the control `shallow` (`count 40 0 = 40`), both by kernel evaluation (`decide +kernel`). Evaluating `count n 0` is a reduction nested about `n` deep, and each level holds a frame of the checker's native stack — so how deep it may go is a property of the stack, not of the proof. `oxilean-verify` checks all of it on its own thread (512 MiB of stack, no limit on the nesting). A checker given a limit on the nesting reports `deep` as `unsupported` (`resource limit exceeded: reduction depth or step limit`), never as rejected; the WebAssembly build sets 256 and also limits the depth of terms, and stops at whichever comes first (here the depth: the argument `count` hands on is 512 levels deep after 256 steps). **The third theorem is a negative control**: `NestedReduction.bad : count 400 0 = 401`, a declaration Lean's own kernel REJECTS (added with `debug.skipKernelTC`), which takes the same four hundred levels to find out; with the room it must be rejected, without it `unsupported`, and under no limit verified. Expected split: 64 verified · 0 unsupported · 1 rejected, exit code 1 (62 · 3 · 0 under a nesting limit of 256). 65 declarations total. |
| `DeepTerm.ndjson` | 28865 | 720 | Export of `tower`, `tower_eq` and `bad` from `DeepTerm.lean` (in this directory, Lean v4.32.0-rc1). `tower` is `Nat.succ` applied three hundred times to `Nat.zero`, written out as a term (the shape of a long list literal: every element one level further down), and `tower_eq` says the same tower over `z`, a definition that unfolds to zero, equals it. A kernel walks a term by recursion — to type it, to compare it, to release it — so the depth of a term is native stack as well. `oxilean-verify` sets no limit on it. A checker given one — the WebAssembly build sets 512 — does not read a declaration with a deeper term into memory and reports it as `unsupported` (`resource limit exceeded: term depth`); three hundred levels are within 512, and the tests use a lower limit to see the refusal. **The third declaration is a negative control**: `DeepTerm.bad`, the tower over `z` equated with a tower one level TALLER, which Lean's own kernel REJECTS (added with `debug.skipKernelTC`). Expected split: 5 verified · 0 unsupported · 1 rejected, exit code 1 (3 · 3 · 0 under a term-depth limit of 100). 6 declarations total; 633 of the 720 records are applications. |
| `ProjOutOfProp.ndjson` | 14083 | 241 | **Negative fixture**: export of `ProjOutOfProp.wit` and `ProjOutOfProp.false_of_proj` from `ProjOutOfProp.lean` (in this directory, Lean v4.32.0-rc1), two declarations Lean's own kernel REJECTS — they were added with `debug.skipKernelTC` so that they could be exported. Both rest on `Expr.proj Exists 0 h` for `h : ∃ n : Nat, True`: the witness read out of a proof. `wit` is that function as a definition; `false_of_proj` is a closed proof of `False` from it, with no axiom (any two proofs of the existential are definitionally equal, so their witnesses are, and the witnesses of `⟨0, _⟩` and `⟨1, _⟩` reduce to `0` and `1`). Lean's kernel refuses the projection itself: out of a proposition only a proposition may be projected. Expected split: 5 verified · 0 unsupported · 2 rejected, exit code 1. Before the kernel had that side condition `oxilean-verify` reported both VERIFIED — 7 verified · 0 unsupported · 0 rejected, exit 0 (a wrong accept of `False`, present in the published 0.1.3 and 0.1.4). 7 declarations total. |
| `ProjRecIndexed.ndjson` | 39606 | 666 | Export of `ProjRecIndexed.proj_mk` from `ProjRecIndexed.lean` (in this directory, Lean v4.32.0-rc1) — ordinary Lean, nothing added without kernel checking: every declaration is one Lean's own kernel ACCEPTED. `Tree` is a recursive `structure` whose recursion is nested through `List`, `Stream` a recursive `structure` whose recursion is direct, and `Tagged α : Nat → Type` a one-constructor family with a parameter and an index. The six `proj` records are projections out of them: `Tree.label`, `Tree.kids`, `Stream.head`, `Stream.tail` (generated by `structure`) and `Tagged.val`, `Tagged.tag` (the `inductive` command generates none; they are written `t.1` / `t.2`, which Lean elaborates to `Expr.proj`); `Tree.firstLabel` and the theorem `proj_mk` use them on variables and on constructor applications. Lean's kernel types a projection out of ANY inductive with exactly one constructor; structure eta and unit-like equality are narrower (no recursion, no indices). Expected split: 22 verified · 0 unsupported · 0 rejected, exit code 0. While the kernel asked the eta condition of a projection `oxilean-verify` reported 14 verified · 0 unsupported · 8 rejected, exit 1 — a wrong REJECT of every recursive `structure` (present in the published 0.1.3 and 0.1.4). 22 declarations total. |
| `CtorListLie.ndjson` | 16821 | 257 | **Negative fixture, edited by hand**: the export of four declarations from `CtorListLie.lean` (in this directory, Lean v4.32.0-rc1) with THREE RECORDS CHANGED after the export (the commands are below). `Two` (`mk (n : Nat)` / `nil`), `L2` (`cons` / `nil`, recursive) and `TI : Nat → Type` (`a` / `b`, indexed) each have two constructors; in each `inductive` record the type's `ctors` list was cut down to the first, the second constructor's own record left in the bundle. No Lean environment looks like that — the kernel builds the constructor list itself — so each bundle must be refused AS A WHOLE, and with it every declaration that names a part of it. The four declarations after them read a field with a raw projection, which Lean's own kernel REJECTS for a type with two constructors (added with `debug.skipKernelTC`): `two_proj`, `two_eta` (`∀ t : Two, t = Two.mk t.0` by `rfl` — false of `Two.nil`), `l2_proj`, `ti_proj`. Expected split: 3 verified · 0 unsupported · 7 rejected, exit code 1 (`Nat`, `Eq`, `Bool` verify). Through 0.1.4 a refused bundle stayed half installed — the type with one constructor, and that constructor — and `oxilean-verify` reported `two_proj` and `two_eta` VERIFIED against it: 5 verified · 0 unsupported · 5 rejected (a wrong accept, present in the published 0.1.3 and 0.1.4); with projections out of recursive and indexed one-constructor types admitted, `l2_proj` and `ti_proj` as well (7 · 0 · 3). 10 declarations total. |
| `RecRuleTruth.ndjson` | 16183 | 267 | **Written by hand** (not an export: see below). A family in which one recursor rule matters, every `inductive` record as Lean v4.32.0-rc1's kernel makes it: `B : Type` (`t`, `f`); `P : B → Prop` with `mk : (b : B) → P b`; `T : Type` with `mk : (b : B) → P b → T`; `Eq`, `False`, `True`; `get b h : B` (the index a proof of `P b` was built at, by large elimination of `P`); `val t` and `fst t`, which for `t = T.mk b h` are `get b h` and `b`; `get_eq`, `val_eq : ∀ t, val t = fst t`. Twelve declarations Lean's kernel accepts, among them `honest_val_is_t : val (T.mk B.t (P.mk B.t)) = B.t` by `rfl`, and three it REJECTS: `bad_val_is_f` (the same with `B.f`), `bad_f_eq_t : B.f = B.t := val_eq _` and `boom : False` from it. Expected split: 12 verified · 0 unsupported · 3 rejected, exit code 1. 15 declarations total. |
| `RecRuleJunk.ndjson` | 16818 | 279 | **Negative fixture, written by hand**: `RecRuleTruth` with ONE thing changed — the right-hand side of the only rule of `T.rec`, from `fun motive mk b h => mk b h` to `fun motive mk b h => mk b (B.rec (motive := fun x => P x) (P.mk B.f) (P.mk B.f) b)`. The inserted term is ILL TYPED (its first minor premise must have the type `P B.t` and has the type `P B.f`); its type read off its head is `P b`, so a comparison by definitional equality takes it for the proof `h`. No Lean environment looks like that — the kernel derives every recursor itself — so the bundle must be refused, and with it every declaration that names it or a dependent of it. Expected split: 7 verified · 0 unsupported · 8 rejected, exit code 1 (`B`, `P`, `Eq`, `False`, `True`, `get`, `get_eq` verify). Through 0.1.4 the record was accepted by definitional equality alone AND INSTALLED: `val (T.mk B.t (P.mk B.t))` reduced through the inserted term to `B.f`, and `oxilean-verify` reported `bad_val_is_f`, `bad_f_eq_t` and `boom : False` VERIFIED — a closed proof of `False` from no axiom — and the honest `honest_val_is_t` rejected: 14 verified · 0 unsupported · 1 rejected (a wrong accept, present in the published 0.1.3 and 0.1.4). 15 declarations total. |
| `RecRuleControl.ndjson` | 4817 | 77 | **Written by hand**: the control of `RecRuleJunk`. `B` and `P` as there, and an ill-typed term of the same making as the one put into the rule — `B.rec (motive := fun _ => P B.t) (P.mk B.f) (P.mk B.f) B.t`, whose first minor premise must have the type `P B.t` and has the type `P B.f` — as the value of a theorem (`ctrl_thm_junk_is_proof`) and of a definition (`ctrl_def_junk`), beside an honest proof (`ctrl_thm_real_proof`). Where a kernel infers the type of such a term it refuses it — Lean's: "application type mismatch" — and so did the published 0.1.3 and 0.1.4: 3 verified · 0 unsupported · 2 rejected, exit code 1. Such a term was only ever accepted where its type was not inferred: in a recursor record. 5 declarations total. |
| `RecNameMixedStyleTruth.ndjson` | 20892 | 341 | **Written by hand** (not an export: see below). The control of `RecNameMixedStyle`: `B`, `P`, `Eq`, `False`, `True`, `get`, `get_eq` as in `RecRuleTruth`, the mutual block of `A` (one constructor `A.mk : (b : B) → P b → C → A`) and `C` (`C.base : C`, `C.step : A → C`) as Lean v4.32.0-rc1's kernel makes it (recursors `A.rec` and `C.rec`), then the records of `RecNameMixedStyle`'s attack: `def A.rec` (a definition of another type under the name the kernel gave the recursor of `A`), `X : P B.t` by `C.rec`'s iota at `C.step`, `get_X_is_f`, `bad_f_eq_t : B.f = B.t` and `boom : False`. Lean's kernel accepts nine and REJECTS four: `A.rec` (the name is taken), `get_X_is_f`, `bad_f_eq_t`, `boom`. Expected split: 9 verified · 0 unsupported · 4 rejected, exit code 1. 13 declarations total. |
| `RecNameMixedStyle.ndjson` | 21870 | 359 | **Negative fixture, written by hand**: `RecNameMixedStyleTruth` with ONE record edited — the recursor record of `A` named with the ONE flat component `"A.rec"` (Lean writes it `«A.rec»`), where Lean's kernel names it `A.rec`, the component `rec` under `A`, and its two recursor constants with it. Lean's kernel never gives a recursor such a name, so the bundle must be refused, with the name expected, and with it every declaration that names it or a dependent of it. Expected split: 8 verified · 0 unsupported · 5 rejected, exit code 1 (`A`; `X`, `get_X_is_f`, `bad_f_eq_t`, `boom` as its dependents; `def A.rec` verifies: no constant of that name is installed). Through 0.1.4 — and in every build before the name rule — the kernel derived the family's recursors in the style of each record's name, so the installed `C.rec` called a hierarchical `A.rec` and the installed `«A.rec»` a flat `«C.rec»`, neither of them installed; the file's `def A.rec` then answered `C.rec`'s call, `X : P B.t` reduced to a proof of `P B.f`, and `oxilean-verify` reported every declaration VERIFIED, `boom : False` among them — a closed proof of `False` from no axiom: 13 verified · 0 unsupported · 0 rejected, exit code 0 (present in the published 0.1.3 and 0.1.4). 13 declarations total. |
| `RecNameFlat.ndjson` | 16239 | 268 | **Negative fixture, written by hand**: the family of `RecRuleTruth` (`T` with `T.mk : (b : B) → P b → T`; `val`, `fst`, `val_eq` and the four statements) with the recursor record of `T` named with the one flat component `"T.rec"`. Expected split: 7 verified · 0 unsupported · 8 rejected, exit code 1 (`T` for its recursor's name; `val`, `fst`, `val_eq`, `honest_val_is_t` and the three Lean rejects in any case as dependents). Through 0.1.4 the record was accepted and the recursor installed under the flat name, so `val` and `fst`, which name Lean's `T.rec`, were refused for an unknown constant, and `val_eq` and `honest_val_is_t` with them, where Lean's kernel accepts all four: 8 verified · 0 unsupported · 7 rejected, with `T` verified where Lean's kernel refuses the record. 15 declarations total. |

## Verification

Each file was validated at download time: the first line of each `.ndjson` file was parsed as JSON using `python3 -c "import json,sys; json.loads(sys.stdin.read())"` and confirmed non-empty.

## Download URLs (pinned to commit)

```
https://raw.githubusercontent.com/leanprover/lean4export/3de59f10bc4b4a0f2de698597aeb1246caa0df0a/examples/Nat.add_succ.ndjson
```

`Nat.add_succ.ndjson` is this download, byte for byte — never a local
re-export. Upstream's example still records format 3.0.0 and Lean 4.27.0-rc1,
which makes it the one fixture in the 3.0.0 record shapes (`{"def":[{...}]}`,
`{"thm":[{...}]}`, `inductiveVals`/`constructorVals`/`recursorVals`), and the
tests in `oxilean-export` and `oxilean-verify` pin it as such.
`scripts/regen-corpus.sh` exports `Nat.add_succ` with the pinned toolchain into
the corpus directory only: that export is format 3.1.0 with one more
declaration (`Nat.add._f`), and copying it here fails those tests. To restore
the fixture, `git checkout -- tests/fixtures/lean4export/Nat.add_succ.ndjson`
or download it again from the URL above.

## Format Notes

Each line in an `.ndjson` file is one JSON object. The file begins with a `meta` line specifying the format version, followed by a stream of interning records and declarations:

- **Name interning:** `{"in": <id>, "str": {...}}` or `{"in": <id>, "num": {...}}`
- **Level interning:** `{"il": <id>, "succ": ...}`, `{"il": <id>, "max": [...]}`, etc. (Level.zero is always index 0)
- **Expr interning:** `{"ie": <id>, "bvar": ...}`, `{"ie": <id>, "sort": ...}`, `{"ie": <id>, "const": {...}}`, `{"ie": <id>, "app": {...}}`, etc.
- **Declarations:** `{"axiom": {...}}`, `{"def": {...}}`, `{"thm": {...}}`, `{"opaque": {...}}`, `{"quot": {...}}`, `{"inductive": {...}}`

## Corpus Generation

The full Lean core corpus (`Init.ndjson`, 330 MB, 6.45M lines) was generated at
`~/work/oxilean-corpus/Init.ndjson`. It is NOT checked into this repo due to size.
To regenerate, see the scratchpad status report and the commands in the Corpus section below.

### Custom Scratch Fixtures

The four custom fixtures (`simple_add`, `point_swap_swap`, `Parity.isEven`, `Tree_Forest*`)
were generated from `~/work/lean4-scratch/Scratch.lean` using:

```sh
source ~/.elan/env
cd ~/work/lean4-scratch
lake env ~/work/lean4export/.lake/build/bin/lean4export Scratch -- <decl_names>
```

With lean4export pinned to commit `3de59f10bc4b4a0f2de698597aeb1246caa0df0a` and
Lean toolchain `leanprover/lean4:v4.32.0-rc1`.

### `QuotPartialApp` (source in this directory)

`QuotPartialApp.ndjson` is generated from `QuotPartialApp.lean`, which sits next
to it and imports nothing beyond the Lean prelude, so no scratch project is
needed. From the repository root, with the same pins:

```sh
TC="$HOME/.elan/toolchains/leanprover--lean4---v4.32.0-rc1"
work="$(mktemp -d)"
( cd tests/fixtures/lean4export && "$TC/bin/lean" -o "$work/QuotPartialApp.olean" QuotPartialApp.lean )
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" \
  ~/work/lean4export/.lake/build/bin/lean4export QuotPartialApp -- \
  QuotPartialApp.succQ_mk QuotPartialApp.neg_mk QuotPartialApp.pred_succ \
  "QuotPartialApp.succQ'_mk" QuotPartialApp.add_mk \
  > tests/fixtures/lean4export/QuotPartialApp.ndjson
```

The exporter runs `lean` to locate the toolchain, hence the `PATH` entry. `lean`
is run from inside the fixture directory because the names it generates for
anonymous binders (`fun _ => …`) embed the source path as given on the command
line: compiling `tests/fixtures/lean4export/QuotPartialApp.lean` from the root
yields an export with the same meaning but different bytes.

### `ReductionChain` (source in this directory)

```sh
TC="$HOME/.elan/toolchains/leanprover--lean4---v4.32.0-rc1"
work="$(mktemp -d)"
( cd tests/fixtures/lean4export && "$TC/bin/lean" -o "$work/ReductionChain.olean" ReductionChain.lean )
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" \
  ~/work/lean4export/.lake/build/bin/lean4export ReductionChain -- \
  ReductionChain.chain_1000 ReductionChain.chain_20000 \
  > tests/fixtures/lean4export/ReductionChain.ndjson
```

### `SharedSubterms` (source in this directory)

```sh
TC="$HOME/.elan/toolchains/leanprover--lean4---v4.32.0-rc1"
work="$(mktemp -d)"
( cd tests/fixtures/lean4export && "$TC/bin/lean" -o "$work/SharedSubterms.olean" SharedSubterms.lean )
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" \
  ~/work/lean4export/.lake/build/bin/lean4export SharedSubterms -- \
  SharedSubterms.tower SharedSubterms.tower_eq \
  > tests/fixtures/lean4export/SharedSubterms.ndjson
```

### `CongruenceFirst` (three sound theorems and a negative control, source in this directory)

`CongruenceFirst.lean` imports `Lean` for the small command that adds
`CongruenceFirst.bad` without kernel checking, as `KGuardCollision.lean` does:

```sh
TC="$HOME/.elan/toolchains/leanprover--lean4---v4.32.0-rc1"
work="$(mktemp -d)"
( cd tests/fixtures/lean4export && "$TC/bin/lean" -o "$work/CongruenceFirst.olean" CongruenceFirst.lean )
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" \
  ~/work/lean4export/.lake/build/bin/lean4export CongruenceFirst -- \
  CongruenceFirst.same_function_equal_args CongruenceFirst.mod_stuck_divisor \
  CongruenceFirst.short_fuel CongruenceFirst.bad \
  > tests/fixtures/lean4export/CongruenceFirst.ndjson
```

Compiling the source prints `LEAN KERNEL: REJECTED CongruenceFirst.bad`, and
`leanchecker CongruenceFirst` on the compiled module reports a declaration type
mismatch for it (`slow 5 k` against `slow 6 k`); the three sound theorems are
accepted by both.

### `NestedReduction` and `DeepTerm` (two sound declarations and a negative control each, sources in this directory)

Both import `Lean` for a small command, as `KGuardCollision.lean` does:
`NestedReduction.lean` to add `NestedReduction.bad` without kernel checking,
`DeepTerm.lean` to build its three terms (three hundred applications deep —
deeper than the elaborator will recurse) and to add `DeepTerm.bad` the same
way.

```sh
TC="$HOME/.elan/toolchains/leanprover--lean4---v4.32.0-rc1"
work="$(mktemp -d)"
( cd tests/fixtures/lean4export && "$TC/bin/lean" -o "$work/NestedReduction.olean" NestedReduction.lean \
    && "$TC/bin/lean" -o "$work/DeepTerm.olean" DeepTerm.lean )
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" \
  ~/work/lean4export/.lake/build/bin/lean4export NestedReduction -- \
  NestedReduction.deep NestedReduction.shallow NestedReduction.bad \
  > tests/fixtures/lean4export/NestedReduction.ndjson
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" \
  ~/work/lean4export/.lake/build/bin/lean4export DeepTerm -- \
  DeepTerm.tower DeepTerm.tower_eq DeepTerm.bad \
  > tests/fixtures/lean4export/DeepTerm.ndjson
```

Compiling the sources prints `LEAN KERNEL: REJECTED NestedReduction.bad` and
`LEAN KERNEL: REJECTED DeepTerm.bad`, and `leanchecker` on either compiled
module reports a declaration type mismatch for its `bad` (`count 400 0` against
`401`; the tower against the one a level taller). The sound declarations are
accepted by both.

### `KGuardCollision` (negative fixture, source in this directory)

`KGuardCollision.ndjson` is generated the same way from `KGuardCollision.lean`,
which imports `Lean` for the small command that adds the declaration without
kernel checking:

```sh
TC="$HOME/.elan/toolchains/leanprover--lean4---v4.32.0-rc1"
work="$(mktemp -d)"
( cd tests/fixtures/lean4export && "$TC/bin/lean" -o "$work/KGuardCollision.olean" KGuardCollision.lean )
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" \
  ~/work/lean4export/.lake/build/bin/lean4export KGuardCollision -- KGuardCollision.bad \
  > tests/fixtures/lean4export/KGuardCollision.ndjson
```

Compiling the source prints `LEAN KERNEL: REJECTED KGuardCollision.bad` — the
verdict of Lean's kernel on the declaration. Lean's environment re-checker gives
the same answer on the compiled module:

```sh
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" "$TC/bin/leanchecker" KGuardCollision
# leanchecker found a problem in KGuardCollision … (kernel) declaration type mismatch
```

### `ProjOutOfProp` (negative fixture, source in this directory)

`ProjOutOfProp.ndjson` is generated the same way from `ProjOutOfProp.lean`,
which imports `Lean` to build its terms with `Expr.proj` (the elaborator will
not write a projection out of an `Exists`) and to add the two declarations
without kernel checking:

```sh
TC="$HOME/.elan/toolchains/leanprover--lean4---v4.32.0-rc1"
work="$(mktemp -d)"
( cd tests/fixtures/lean4export && "$TC/bin/lean" -o "$work/ProjOutOfProp.olean" ProjOutOfProp.lean )
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" \
  ~/work/lean4export/.lake/build/bin/lean4export ProjOutOfProp -- \
  ProjOutOfProp.wit ProjOutOfProp.false_of_proj \
  > tests/fixtures/lean4export/ProjOutOfProp.ndjson
```

Compiling the source prints `LEAN KERNEL: REJECTED ProjOutOfProp.wit` and
`LEAN KERNEL: REJECTED ProjOutOfProp.false_of_proj` — the verdicts of Lean's
kernel on the two declarations. Lean's environment re-checker stops at the
first of them on the compiled module:

```sh
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" "$TC/bin/leanchecker" ProjOutOfProp
# leanchecker found a problem in ProjOutOfProp … while replaying declaration 'ProjOutOfProp.wit':
# (kernel) invalid projection
```

### `ProjRecIndexed` (positive fixture, source in this directory)

`ProjRecIndexed.ndjson` is generated the same way from `ProjRecIndexed.lean`,
which imports nothing beyond the Lean prelude and adds nothing without kernel
checking:

```sh
TC="$HOME/.elan/toolchains/leanprover--lean4---v4.32.0-rc1"
work="$(mktemp -d)"
( cd tests/fixtures/lean4export && "$TC/bin/lean" -o "$work/ProjRecIndexed.olean" ProjRecIndexed.lean )
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" \
  ~/work/lean4export/.lake/build/bin/lean4export ProjRecIndexed -- ProjRecIndexed.proj_mk \
  > tests/fixtures/lean4export/ProjRecIndexed.ndjson
```

Compiling the source prints nothing — Lean's kernel accepts every declaration —
and Lean's environment re-checker agrees on the compiled module:

```sh
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" "$TC/bin/leanchecker" ProjRecIndexed
# no output, exit code 0
```

`Tagged`'s index must stay an index for the fixture to make its point: Lean's
`inductive` command turns an index into a parameter when a constructor
argument is used as the index of every result type AND comes first among the
constructor's arguments (`mk (tag : Nat) (val : α) : Tagged α tag` would give
`Tagged : Type → Nat → Type` with TWO parameters and no index). `tag` follows
`val`, so it cannot be promoted; `oxilean-export`'s `replay_smoke` pins
`num_params = 1`, `num_indices = 1`.

### `CtorListLie` (negative fixture, source in this directory, three records edited by hand)

`CtorListLie.ndjson` is generated the same way from `CtorListLie.lean`, which
imports `Lean` to build its four terms with `Expr.proj` (the elaborator will
not write a projection out of a type with two constructors) and to add them
without kernel checking — and is then EDITED: the export goes through `sed`
before it is written.

```sh
TC="$HOME/.elan/toolchains/leanprover--lean4---v4.32.0-rc1"
work="$(mktemp -d)"
( cd tests/fixtures/lean4export && "$TC/bin/lean" -o "$work/CtorListLie.olean" CtorListLie.lean )
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" \
  ~/work/lean4export/.lake/build/bin/lean4export CtorListLie -- \
  CtorListLie.two_proj CtorListLie.two_eta CtorListLie.l2_proj CtorListLie.ti_proj \
  | sed -e 's/"types":\[{"all":\[13\],"ctors":\[14,15\]/"types":[{"all":[13],"ctors":[14]/' \
        -e 's/"types":\[{"all":\[32\],"ctors":\[33,34\]/"types":[{"all":[32],"ctors":[33]/' \
        -e 's/"types":\[{"all":\[48\],"ctors":\[49,50\]/"types":[{"all":[48],"ctors":[49]/' \
  > tests/fixtures/lean4export/CtorListLie.ndjson
```

The three expressions are the whole hand edit, one record each (257 records
before and after; three differ; 16,830 bytes before, 16,821 after). The
numbers are the export's name ids: 13 is `CtorListLie.Two`, 14 `Two.mk`,
15 `Two.nil`; 32 is `CtorListLie.L2`, 33 `L2.cons`, 34 `L2.nil`; 48 is
`CtorListLie.TI`, 49 `TI.a`, 50 `TI.b`. Each expression takes the second
constructor out of the `ctors` list of the TYPE (`"types":[{…,"ctors":[…]`);
the record of that constructor, further on in the same `inductive` record,
is not touched, and neither is the recursor.

Compiling the source prints `LEAN KERNEL: REJECTED` for each of
`CtorListLie.two_proj`, `CtorListLie.two_eta`, `CtorListLie.l2_proj` and
`CtorListLie.ti_proj` — the verdicts of Lean's kernel on the four declarations,
over the three types as they really are (the compiler also prints a panic of
its library-suggestions extension, which meets the ill-typed terms when the
module is written; the exit code is 0). Lean's environment re-checker stops at
the first of them on the compiled module:

```sh
PATH="$TC/bin:$PATH" LEAN_PATH="$work:$TC/lib/lean" "$TC/bin/leanchecker" CtorListLie
# leanchecker found a problem in CtorListLie … while replaying declaration 'CtorListLie.two_proj':
# (kernel) invalid projection
```

Lean cannot be shown the edited file — it reads no export — and needs not be:
its kernel computes the constructor list of an inductive type when the type
is declared, so the edited records say something no Lean environment says.
Without the edit the export replays as 6 verified · 0 unsupported ·
4 rejected: the three types are accepted and the four declarations refused
for their projections, as by Lean.

### `RecRuleTruth`, `RecRuleJunk`, `RecRuleControl` (a recursor rule that is not the kernel's; written by hand)

These three files have no Lean source, and lean4export did not write them. No
Lean session can export `RecRuleJunk`: Lean's kernel derives every recursor
when an inductive type is declared and takes none from anywhere, so a file
whose recursor record differs from the derivation has to be made by hand. Its
truthful twin and its control were made the same way, so that the three differ
in nothing but what they are about.

They were written on 2026-11-12 by a script outside this repository that
emits the records of format 3.1.0 directly (names, levels and expressions
interned as lean4export interns them). Every `inductive` record in them is
what Lean 4 v4.32.0-rc1's kernel makes of the declaration: the declaration —
each type's name and type, each constructor's name and type — was handed to
`Lean.Environment.addDeclCore` in an environment that held the file's earlier
declarations and nothing else, and the types, constructors and recursors the
kernel then holds were written back as the record. `RecRuleJunk` is that file
with one field of one record changed afterwards: the `rhs` of the one rule of
the recursor record `T.rec` (see the inventory row for the two terms). The
script numbers the records of each file afresh, so the two files do not
compare line by line; `fixtures_end_to_end` pins that they declare the same
fifteen names in the same order and that the lie is six `app`, five `lam` and
one `const` record more than the truth.

Lean cannot be shown a file as such — it reads no export — but its kernel can
be given every record of one: replayed record by record into an empty
environment through `Environment.addDeclCore` (an inductive record as the
declaration it states, the constants the kernel then holds compared with the
record), the three files get these verdicts from Lean 4 v4.32.0-rc1:

- `RecRuleTruth`: twelve declarations accepted; `bad_val_is_f` and
  `bad_f_eq_t` rejected ("declaration type mismatch"), `boom` rejected
  ("unknown constant 'bad_f_eq_t'").
- `RecRuleControl`: three accepted; `ctrl_thm_junk_is_proof` and
  `ctrl_def_junk` rejected ("application type mismatch … argument has type
  `P B.f` but function has type … `B.t` → …").
- `RecRuleJunk`: the kernel accepts the declaration of `T`, and the `T.rec`
  it derives is not the file's record (rule 0, `rhs`). Against the kernel's
  own recursor the declarations after it get the verdicts of `RecRuleTruth`:
  `honest_val_is_t` accepted; `bad_val_is_f`, `bad_f_eq_t`, `boom` rejected.

`oxilean-verify` refuses the bundle `T` of `RecRuleJunk` — a file whose
recursor record is not the derived recursor describes no Lean environment —
and therefore every declaration that names `T`, `T.mk`, `T.rec` or a
dependent of them: `val`, `fst`, `val_eq`, `honest_val_is_t` and the three
that Lean's kernel rejects in any case.

The files are restored with `git checkout`; their SHA-256:

```
eaf3c38c0638fe1b2d6b42b5a38eb5ebe9136dcc39fc07bfa0e33057db81d63a  RecRuleTruth.ndjson
9f88927a96df3c7b439b1d6441e199b0f9e1504a9d8d7a29dad468b0bd3cc8a9  RecRuleJunk.ndjson
1ab47077e7aa6400eaf11bf02cd8800ef770392bbd846e04a5298f219aa1d919  RecRuleControl.ndjson
```

### `RecNameMixedStyleTruth`, `RecNameMixedStyle`, `RecNameFlat` (a recursor record under a name Lean never gives; written by hand)

Like `RecRule*`, these files have no Lean source and lean4export did not write
them; they were made on 2026-11-12/13 by the same kind of script (the format
3.1.0 records emitted directly), every `inductive` record completed by Lean 4
v4.32.0-rc1's kernel as described for `RecRule*`, and then — in the two
negative files — ONE record edited: the name of a recursor record, from Lean's
hierarchical `A.rec` (the string `rec` under the name `A`) to the one flat
component `"A.rec"`, and in `RecNameMixedStyle` the recursor constants of that
record's own type and rules with it. A name of one component that contains a
dot prints as the hierarchical name does; Lean writes it `«A.rec»`.

The name matters because the kernel derives the recursors of a whole family
and, in a mutual (or nested) family, a recursor's rules call the recursors of
the sibling types by name. The kernel's own tests spell recursor names in the
flat style too, and a family is derived in the style of the record that is
checked, so a mutual bundle with one record of each style was installed with
rules that called recursors no one installed: here `C.rec`'s rule for
`C.step` called the hierarchical `A.rec`. A later record then defines
`A.rec`. `oxilean-verify` now refuses a recursor record under any name but
the one Lean's kernel gives it (`T.rec`, or `<first type>.rec_<k>` for an
auxiliary type of a nested declaration), and the kernel holds every name a
derived term mentions to the constants the install leaves in the environment.

Replayed record by record into an empty environment through
`Environment.addDeclCore`, Lean 4 v4.32.0-rc1 gives:

- `RecNameMixedStyleTruth`: nine accepted; `A.rec` rejected ("already
  declared"), `get_X_is_f` and `bad_f_eq_t` rejected ("declaration type
  mismatch"), `boom` rejected ("unknown constant").
- `RecNameMixedStyle`: the kernel accepts the declaration of `A` and `C`, and
  the recursors it makes are not the file's records (it made no recursor
  `«A.rec»`, and no record of the file is its `A.rec`). Against the kernel's
  own constants the records after it get the verdicts of the truth: `A.rec`,
  `get_X_is_f`, `bad_f_eq_t`, `boom` rejected.
- `RecNameFlat`: the record of `T` is not the kernel's (no recursor
  `«T.rec»`); the records after it get `RecRuleTruth`'s verdicts.

`oxilean-verify` installs nothing of a refused bundle. `RecNameMixedStyle`'s
`def A.rec` is then the definition of a free name and verifies, where Lean's
replay — which keeps its own constants for the declaration — finds the name
taken; every record that names the bundle is refused.

The files are restored with `git checkout`; their SHA-256:

```
1f36640514c59330a0f6bd7faab1f5c5fd078155bd809272555d9682a68a5af0  RecNameMixedStyleTruth.ndjson
0df8e3d31665a47128aa6da412df065936637a8ae4cafe623e37a3fb06d908cf  RecNameMixedStyle.ndjson
a7fcb06a69cb6616821d990d769c36b7e8f11a4489a1f68aca1698aac9893cc9  RecNameFlat.ndjson
```
## Notes

- These fixtures are sufficient for smoke-testing the `oxilean-export` reader and
  populating the initial cargo-fuzz corpus.
- For meaningful coverage of edge cases, the full lean4export corpus (Option A in the
  audit) should be generated once elan/Lean installation is approved.
