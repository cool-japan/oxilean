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
| `simple_add.ndjson` | 36864 | 643 | Export of `simple_add : 2 + 3 = 5 := rfl` (custom scratch module, Lean v4.32.0-rc1). Exercises Nat literals, OfNat instances, and `rfl` proof term. 23 declarations total. |
| `point_swap_swap.ndjson` | 21504 | 372 | Export of `point_swap_swap` theorem over a custom `Point` structure (custom scratch module, Lean v4.32.0-rc1). Exercises `structure` declarations, struct eta, field projections (`Expr.proj`), and `simp`-generated proof terms. 16 declarations total. |
| `Parity.isEven.ndjson` | 383000 | 7015 | Export of `Parity.isEven` defined via `Quotient.lift` over a custom `myRel` equivalence relation (custom scratch module, Lean v4.32.0-rc1). Exercises `quot` kernel primitive, `Quot.lift`, `Quot.ind`, and proof obligations. 181 declarations total. |
| `Tree_Forest.ndjson` | 9523 | 166 | Export of `Tree`/`Forest` mutual inductive (custom scratch module, Lean v4.32.0-rc1). One `inductive` NDJSON record bundles both types (the lean4export format for mutual inductives). Both types appear in each other's `all` field. 1 `inductive` bundle record. |
| `Tree_Forest_full.ndjson` | 71680 | 1265 | Export of `Tree`/`Forest` mutual inductive plus `Tree.size`/`Forest.size` mutual defs (custom scratch module, Lean v4.32.0-rc1). Exercises mutual inductives and mutual well-founded recursive definitions. |

## Verification

Each file was validated at download time: the first line of each `.ndjson` file was parsed as JSON using `python3 -c "import json,sys; json.loads(sys.stdin.read())"` and confirmed non-empty.

## Download URLs (pinned to commit)

```
https://raw.githubusercontent.com/leanprover/lean4export/3de59f10bc4b4a0f2de698597aeb1246caa0df0a/examples/Nat.add_succ.ndjson
```

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

## Notes

- These fixtures are sufficient for smoke-testing the `oxilean-export` reader and
  populating the initial cargo-fuzz corpus.
- For meaningful coverage of edge cases, the full lean4export corpus (Option A in the
  audit) should be generated once elan/Lean installation is approved.
