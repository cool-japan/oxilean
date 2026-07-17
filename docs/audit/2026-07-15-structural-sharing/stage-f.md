# Stage F — Name / Level interning (wave5 structural sharing)

Status: in progress (0.1.4). Companion to [`design.md`](./design.md) §6 "Stage F".

## 1. Goal

`Const(Name, Vec<Level>)` deep-copied its `Name` and every `Level` on each clone.
At Mathlib scale (a claimed 4.09M distinct names, each referenced by many `Const`
nodes) that duplication is the difference between an ~8 GiB and an ~3 GiB
persistent floor — the M4 memory lever. Init already passes at Stage E, so Stage F
is **memory-only**: it must not move a single Init verdict.

The lever is **whole-value sharing**: the same name (e.g. `Nat.rec`) referenced by
thousands of `Const` nodes must be **one** heap allocation, and a name must share
its entire prefix with any longer name built on it.

## 2. Representation (chosen: full newtype, workspace-wide)

```rust
pub struct Name(Rc<NameKind>);
enum NameKind { Anonymous, Str(Name, String), Num(Name, u64) }   // private

pub struct Level(Rc<LevelKind>);
enum LevelKind { Zero, Succ(Level), Max(Level, Level), IMax(Level, Level),
                 Param(Name), MVar(LevelMVarId) }                 // private
```

* **O(1) clone** everywhere (refcount bump), so a `Const`/`Sort` clone no longer
  deep-copies its name/levels.
* **Prefix sharing** by construction: the parent link is itself a `Name`.
* **Node shrink**: `Const`'s name field 24 B → 8 B, each `Level` 24 B → 8 B
  (`Sort(Level)` likewise). This is the "interning shrinks the node" item.
* Matching goes through borrowed views obtained by `.view()`:
  `NameView<'a> { Anonymous, Str(&Name,&str), Num(&Name,u64) }` and the analogous
  `LevelView`. `match name { Name::Str(p,s) => .. }` becomes
  `match name.view() { NameView::Str(p,s) => .. }`.

### Why newtype over `NameId(u32)` handles

The design's rejected-alternatives lens weights **soundness surface** heaviest.
`PartialEq`/`Hash` are *derived* on the newtype, delegating to `Rc<…Kind>`: std's
`Rc<T: Eq>` short-circuits on pointer equality and then **falls back to structural
comparison**. So equality is tautologically a refinement of structural equality —
it does *not* depend on any interner being complete. An `Id`-eq scheme would make
id-equality the semantic `==`, and a single missing dedup field would silently
turn two distinct terms into a wrong def-eq accept. We keep ptr-eq as a fast
*path*, never as the *truth*.

### Interning pool (deferred)

Reader-boundary sharing (see §3) already captures the entire corpus lever, because
the export's name/level tables are file-global and dedup by id: each distinct name
has exactly one table id, so there is no cross-id duplication for a global pool to
collapse. A thread-local **index-only open-addressing** hash-cons pool (the
design's "never `HashMap<full-node,id>`" note) would only dedup names built
independently of the table (kernel-internal construction) and is a marginal,
determinism-sensitive add. It is therefore deferred behind the ON/OFF differential
gate; Stage F ships the newtype + reader sharing.

## 3. Reader interning (export boundary)

`reader.rs` keeps a file-global `names` table. Materializing name id `k =
Str(prefix_id, s)` builds `Name::mk_str(names[prefix_id].clone(), s)` — the prefix
is shared by an O(1) `Rc` clone, and `name_at(k)` hands every referencing `Const`
the *same* `Rc<NameKind>`. Levels are handled analogously through the level table.
No global pool is needed for the corpus win.

## 4. Determinism / fuel neutrality

`Name`/`Level` derive `Clone` (no `fuel::charge`), and `Expr::clone` still charges
exactly 1 per `Expr` node regardless of whether the inner `name.clone()` /
`levels.clone()` is a deep copy or an `Rc` bump. So **Stage F is fuel-neutral by
construction** — the ON/OFF differential determinism gate (identical `used()` fuel
AND identical per-decl verdicts) is satisfied trivially, since no fuel accounting
touches Name/Level clone depth.

## 5. Migration surface

Type change is workspace-wide (`Name`/`Level` live in `oxilean-kernel` and are used
by every crate). ~900 variant construction/pattern sites migrate to
`view()`/constructors:

| crate            | Name sites | Level sites |
|------------------|-----------:|------------:|
| oxilean-kernel   | 87         | 286         |
| oxilean-meta     | 14         | 215         |
| oxilean-elab     | 8          | 148         |
| oxilean-std      | 60         | 66          |
| others (cli/parse/codegen/export) | 4 | 23 |

Mechanical gotchas (all meaning-preserving): view `Str` binds `&str` not `&String`;
view `Num`/`MVar` bind by value not by reference; view `Succ`/`Max`/`IMax` bind
`&Level` not `&Box<Level>` (`(**x).clone()` → `x.clone()`); a `const … : Level`
must become a `fn` (`Level::zero()` is not `const`); walking a `Succ` chain in a
`while let` becomes a `loop { … break }` to avoid a `.view()` borrow conflict.

## 6. Acceptance gates

1. **rejected == 0** and **verified ≥ 35,223** (non-negotiable).
2. **Init verdicts identical** to the Stage E baseline: caged full-Init
   47,119 verified / 10,158 unsupported / 0 rejected, byte-for-byte the same set.
   Stage F is memory-only; any verdict shift is a bug.
3. **peak RSS ≤ baseline** on the caged full-Init run (`systemd-run MemoryMax=12G`).
4. All kernel + workspace tests green; `clippy -D warnings`; `cargo fmt`.
5. **wasm** `wasm32-unknown-unknown` still builds; gzip under the 400 KB budget.

## 7. Acceptance results (0.1.4, achieved)

Migration: `Name`/`Level` → `Rc<…Kind>` newtypes, ~900 variant sites across 8
crates rewritten to `view()`/constructors. All gates green:

| gate | result |
|------|--------|
| `cargo check --workspace --all-targets` | 0 errors |
| workspace test suite | **33,316 passed / 0 failed** |
| doctests | all pass |
| `clippy -D warnings` (workspace) | clean |
| `cargo fmt` | applied |
| verdict sanity (Syntax / Parity.isEven / …) | identical (Syntax 269/0/0) |
| wasm gzip (`oxilean-verify-wasm`) | **275 KB** < 400 KB |

**Caged full-Init** (`systemd-run MemoryMax=12G`, release, `--limits corpus`):

* **47,119 verified / 10,158 unsupported / 0 rejected** — byte-identical to the
  post-C10 baseline. Stage F moved **zero** verdicts, confirming memory-only.
* **Peak RSS 2.89 GiB (3,031,340 KB), zero swap** — well under the ≤ 5.5 GiB
  target; down from the pre-wave5 10.4 GiB RSS + 6.7 GiB swap. wall 41:18.

The Mathlib-scale RSS payoff (the ~8 GiB → ~3 GiB claim) is not measured here;
Init is the memory-only correctness gate. A Mathlib run remains future work.
