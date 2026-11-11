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

## Notes

- These fixtures are sufficient for smoke-testing the `oxilean-export` reader and
  populating the initial cargo-fuzz corpus.
- For meaningful coverage of edge cases, the full lean4export corpus (Option A in the
  audit) should be generated once elan/Lean installation is approved.
