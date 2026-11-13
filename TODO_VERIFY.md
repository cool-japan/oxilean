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

## Status on the 0.1.4 tree (re-measured 2026-11-04, `86227a8`)

Both builds were run on the same machine: Apple M3, 24 GB, macOS, one
verification thread, `--limits corpus`, load average ≈4–5.

| metric | 0.1.3 (crates.io) | 0.1.4 tree |
|---|---|---|
| Lean core `Init` (57,277 decls) | 35,424 verified · 21,853 unsupported · 0 rejected | **47,119 verified · 10,158 unsupported · 0 rejected** |
| unsupported attribution | 1 nested-inductive root (`Lean.Syntax`) + 97 clone-fuel roots + 21,755 dependents | **643 clone-fuel roots + 9,515 dependents**; nested-inductive root cleared (C10) |
| full-`Init` wall / peak RSS | 8 m 28 s / 3.5 GiB | 33 m 05 s / 2.1 GiB |
| same-set throughput (`prelude-probe-u32`, 1,016 decls, both fully verified) | 3.03 s / 57 MB | 2.94 s / 35 MB |
| `init-2000` slice | 1,905 · 95 · 0 in 8.4 s | 1,991 · 9 · 0 in 11.7 s |
| wasm artifact (`gzip -9`, wasm-pack + `wasm-opt -Oz`) | 147,336 B | 168,728 B (~165 KB; 439,567 B raw) vs 400 KB budget |

**Open on 0.1.4:**

- [x] **Decompose the full-`Init` wall-clock increase** (8 m 28 s → 33 m 05 s).
  On the 1,016-declaration probe, throughput is unchanged. That points at the
  work 0.1.4 adds: the 11,695 newly verified declarations, and the 643
  clone-fuel roots, each of which runs to its budget before it is degraded.
  The increase is not decomposed yet. Next step: a `--json-full`
  per-declaration timing run on both builds.
  *(2026-11-11: the time was in the keys of the kernel's caches — hashing
  terms, and formatting them to order a pair — on every cache lookup, in both
  builds. The declarations the 0.1.4 tree verifies took 1,321 s of its run
  and take 161 s now; the 1,016-declaration probe takes
  0.46 s / 30 MB where 0.1.3 takes 3.01 s / 60 MB. `Init` takes
  3 m 58 s. See "Performance work on the 0.1.4 tree" below.)*
- [x] **Resource-limit tail**: 643 roots now exceed the 2^26 corpus clone-fuel
  budget (97 on 0.1.3). Most of them were previously hidden behind the
  `Lean.Syntax` cascade. *(634 after the kernel fixes of 2026-11-11, 632 after
  the performance work, **0 after its second pass**: `Init` is
  57,277 verified · 0 unsupported · 0 rejected. See "Budget tail, sharing and
  def-eq work" below.)*
- [x] **Re-run the lean4lean differential** on the 0.1.4 kernel. It was last
  run during the 0.1.3 cycle, before C10. *(2026-11-11: 0 disagreements over
  1,976 joined names, against lean4lean `779c51f` built on v4.32.0-rc1 — the
  corpus's own toolchain, so without the earlier version skew.)*
- [ ] **Refresh the committed demo artifact** (`web/verify-demo/pkg` is still the
  0.1.3 build). *(2026-11-12: done for the 0.1.4 release — 168,785 B gzip, 427,914 B raw.)*

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
  the ~16 GiB corpus memory footprint. *(Met on the 0.1.4 tree, 2026-11-11:
  measured against lean4lean on one machine with one toolchain — see
  "Performance work on the 0.1.4 tree".)*
- **Nested inductives** (nested→mutual elaboration, `Lean.Syntax`): the single
  highest-leverage completeness item — would unlock up to 14,347 Syntax-only
  followers of the 22,054 unsupported. *(Done after the release: C10,
  `b3c17f8`/`b3adf3f`, on the 0.1.4 tree. See the status section above.)*
- **Resource-limit tail**: 175 legal-but-heavy roots stay in the NAMED
  resource-limit bucket at the 2^26 corpus fuel preset (spot-checked verified
  at 2^30 under a cage); raising the preset re-opens the C16b OOM, so it stays.
- **G7 wasm half**: no node/deno/headless browser in this environment; native
  half + structural wasm validation + shared engine path stand in. *(2026-11-12: node is available now, and the wasm build has been run under node over both corpora (see README, Kernel in a Tab); the gate script still does not implement the wasm half.)*
- **lean4lean version skew**: no lean4lean tag for v4.32.0-rc1 (master =
  v4.29.0) — declaration-set skew documented, lands in ONLY-ONE-SIDE.
  *(2026-11-11: lean4lean `779c51f`, which pins v4.32.2, builds on
  v4.32.0-rc1 when its `lean-toolchain` is overridden; the differential was
  re-run with it.)*
- **Corpus re-export scripting** (`scripts/regen-corpus.sh` committed; the
  elan/lean4export install steps remain manual) — V5 residual.
- **F1–F4** deliberate follow-ups (TCB slimming, MSRV, legacy export format,
  announcement) — unchanged.

---

## Kernel fixes on the 0.1.4 tree (2026-11-11, working copy on `eafe042`)

Found while checking an external report: a pinned 0.1.4 snapshot run against
Mathlib-based proof libraries, which named `AddCon.addGroup._proof_1`. S12, C23
and C24 are described in the P0 sections below. All three are also in the
published 0.1.3 (checked with the crates.io binary).

Same machine as above, `--limits corpus`; three counts are verified ·
unsupported · rejected.

| corpus | before (`eafe042`) | after |
|---|---|---|
| Lean core `Init` (57,277 decls) | 47,119 · 10,158 · 0 | **47,127 · 10,150 · 0** |
| Mathlib slice (28,832 decls) | 25,661 · 3,144 · **27** | **25,690 · 3,142 · 0** |
| every declaration of `Mathlib.GroupTheory.Congruence.Defs`, with dependencies (1,611) | 1,595 · 0 · 16 | 1,611 · 0 · 0 |
| `FreeGroup.instGroup` and `FreeGroup.induction_on`, with dependencies (419) | 416 · 0 · 3 | 419 · 0 · 0 |
| fixture `QuotPartialApp` (66) | 63 · 0 · 3 | 66 · 0 · 0 |
| fixture `ReductionChain` (36) | 34 · 0 · 2 | 36 · 0 · 0 |
| fixture `KGuardCollision` (4, one of which Lean's kernel rejects) | 4 · 0 · 0 | 3 · 0 · 1 |

The Mathlib slice is Mathlib `v4.32.0-rc1` (`360da6f`), exported with the pinned
lean4export: the 12,831 non-internal declarations of the `Order`, `Data`,
`Logic`, `Algebra`, `GroupTheory` and `Control` modules that
`Mathlib.GroupTheory.Congruence.Defs` imports, with their dependencies. Per
declaration, nothing moves except towards `verified`: on `Init`, 8 declarations
leave `unsupported`; on the slice, the 27 rejections (9 roots, 18 dependents)
and 2 unsupported ones.

**Measured on the Mathlib slice while checking the report** (a 40 s sample of
the slice run on the fixed kernel, 25,358 samples on the checking thread). The
first three were taken up the same day — see "Performance work" below.

One correction to the first version of this list. It put the `String`
formatting in the sample (about 9% of it) down to
`lit_op_arity(&name.to_string())`. That call is 4 samples of 25,358, and the
formatting of its argument 11. The formatting in the sample belongs to
`canonicalize_pair` in the equivalence manager, which
formatted both terms of every pair it was asked about with `Debug`, to put the
pair in a fixed order: 27% of the samples with the copying that goes with it.

- [x] **Expression hashing is over half the run time.** 54.6% of the samples
  are under the hashing of a map key (43% in SipHash itself, 28% hashing the
  names inside terms). The hash was structural and uncached, and one
  `is_def_eq` hashed both terms in full for the def-eq cache and for each of
  `EquivManager`'s three maps; `whnf` hashed its term for the cache lookup
  before it knew whether the term was small enough to be cached at all.
- [x] **Pair canonicalisation formats both terms**: 27.1% of the samples (see
  the correction above).
- [x] **Local types are never removed from the checker's maps.** On 25,002
  declarations of the slice, every binder opened while checking a declaration
  was still in three maps when the check ended (largest: 14,677 + 2,068 for one
  declaration; median 20), and each K-like or structure-eta attempt seeded a
  nested checker with all of them.
- [ ] **Budget tail on Mathlib.** 111 clone-fuel roots take 3,027 dependents
  with them (`sdiff_inf_sdiff` alone a large share). `loop 1000 0 = 1000` by
  `decide +kernel`, for a structurally recursive `loop` with an accumulator,
  exhausts the 2^26 budget; Lean's kernel checks it at once.

---

## Performance work on the 0.1.4 tree (2026-11-11, working copy on `0abe648`)

Four changes to the kernel. None of them changes what a term reduces to or
which terms are definitionally equal; PF1, PF2 and PF4 do not change what is
charged to the per-declaration budget either.

- [x] **PF1. A structural hash cached on every node.** `Name`, `Level` and every
  shared subterm of an `Expr` (`Node`) store a 64-bit hash of their structure,
  computed when the value is built from the stored hashes of its immediate
  children (`structural_hash.rs`). `Expr`'s `Hash` writes that one value, where
  the derived implementation walked the whole term — as a tree, a shared
  subterm once per path to it — on every cache lookup. `==` on nodes, names and
  levels uses the stored hash as a negative test and otherwise compares as
  before. The hash covers exactly the fields equality compares (binder names
  and annotations included), is the same in every process, and is not keyed.
- [x] **PF2. Pair canonicalisation without formatting.** `EquivManager` puts the
  two terms of a pair in the order of their hashes. Only two terms with equal
  hashes fall back to comparing their `Debug` text, which every call used to do.
- [x] **PF3. Local types leave the mirrors when their binder closes**
  (`local_types.rs`). `TypeChecker::pop_local` and the def-eq checker's binder
  opening now remove the variable from the three mirrors of the local context,
  so a nested K / structure-eta checker is seeded with the variables in scope
  and not with every binder ever opened. Two things are kept about a closed
  variable, in space bounded by the context depth: that its id was used (an id
  bound again drops the owner's caches — the old check only caught a different
  type for an id still in the map), and the largest id ever bound in each range
  (a nested checker mints past it, exactly as it did when nothing was removed —
  the S12 guarantee).
- [x] **PF4. `expr_size_within` reads the cached rebuild cost.** That cost counts
  each node once and each leaf once more, so `size <= cost <= 2 * size`: a cost
  at or under the cap, or over twice the cap, decides without counting.

**Layout.** A `Node` is now one pointer: `range`, `cost` and the hash sit in the
shared allocation next to the subterm (through 0.1.3 `range` and `cost` sat on
the edge). `Expr` is 40 bytes where it was 64, a node allocation 72 where it
was 80. **Public API change:** `Node::rc()`, which exposed the `Rc<Expr>` the
edge used to hold, is replaced by `Node::as_ptr()` (the address, for
memoising over shared subterms) and `Node::ptr_eq()`. `Reducer` and
`DefEqChecker` gain `forget_fvar_type`. What `Expr` and `Node` write to a
`Hasher` is a different value than in 0.1.3; `Name`, `Level` and every `Debug`
text are byte-for-byte what they were (pinned by tests against copies of the
old types).

**Verdicts** (`--limits corpus`, per declaration, against `0abe648`):

| corpus | `0abe648` | after |
|---|---|---|
| Lean core `Init` (57,277 decls) | 47,127 · 10,150 · 0 | **47,133 · 10,144 · 0** |
| Mathlib slice (28,832 decls) | 25,690 · 3,142 · 0 | 25,690 · 3,142 · 0 (identical report) |
| the ten committed fixtures, both presets | — | identical reports |
| `Init.Prelude` (1,814), `init-2000`, the three Mathlib closures above, both presets | — | identical reports |

"Identical report" is the full `--json-full` output with the timing fields
removed. On `Init` six declarations move from `unsupported` to `verified` and
nothing else moves: `Std.Iter.step_filterM` and `Std.Iter.toList_attachWith`,
which were over the clone-fuel budget, and the four declarations that were
unsupported only because they depend on one of them.
PF3 is the cause: a build with PF1 and PF2 alone gives an identical `Init`
report, and PF4 does not touch the budget. A nested checker no longer pays for
locals out of scope, which was enough to bring those two under it (they sit
at its edge: exported on their own, `toList_attachWith` is over the budget in
every build). Clone-fuel roots on `Init`: 634 → 632.

**Time and memory** (same machine as above; one verification thread; load
average ≈3–4 during the runs):

| measurement | `0abe648` | after | |
|---|---:|---:|---:|
| Lean core `Init`, wall | 31 m 51 s | **3 m 58 s** | 8.0× |
| Mathlib slice, wall | 5 m 50 s | **43 s** | 8.2× |
| `init-2000` slice (`scripts/bench-verify.sh`, median of 3) | 10.1 s (199 decls/s) | **1.3 s (1,518 decls/s)** | 7.6× |
| `Parity.isEven` fixture (same script) | 81 ms | 25 ms | 3.2× |
| 1,016-declaration probe (`prelude-probe-u32`), wall / peak RSS | 2.48 s / 35 MB | 0.46 s / 30 MB | 5.4× |
| instructions retired, `Init` | 32.0 × 10¹² | 4.0 × 10¹² | 8.0× |
| peak live heap, `Init` (counting allocator) | 2,723 MB | 2,458 MB | −10% |
| peak live heap, Mathlib slice | 1,349 MB | 1,213 MB | −10% |
| bytes allocated, `Init` / Mathlib slice | 1,482 GB / 247 GB | 480 GB / 88 GB | −68% / −64% |
| allocations, `Init` / Mathlib slice | 8.53 × 10⁹ / 1.50 × 10⁹ | 6.95 × 10⁹ / 1.23 × 10⁹ | −19% / −18% |
| peak RSS, `Init` (`/usr/bin/time -l`) | 2.31 GiB | 2.54 and 2.56 GiB | |
| wasm artifact (`gzip -9`; wasm-pack + `wasm-opt -Oz`) | 169,956 B | 161,375 B | −5% |

The published 0.1.3 takes 3.01 s / 60 MB on the 1,016-declaration probe.

Peak live heap is measured by linking the verify engine into a scratch binary
with a counting global allocator (requested bytes). In both builds the peak
is a declaration that runs into the clone-fuel budget, not the resident
environment (on `Init`, 1.6 GB of terms on top of a 0.8 GB environment, for
`WellFounded.partialExtrinsicFix₃_eq_partialExtrinsicFix`).

`/usr/bin/time -l` is a poor instrument for this workload. Its "peak memory
footprint" was 2.09 GiB in one run of the final build and 1.27 GiB in the
next, with identical reports. Its "maximum resident set size" is steady (2.54
and 2.56 GiB) but 0.2 GiB HIGHER than for `0abe648`, although the live heap is
lower: resident pages include what the allocator keeps after a free.

**Against Lean's kernel and lean4lean**, same machine, same toolchain
(v4.32.0-rc1), same module: `Init.Prelude`, end-to-end wall, median of 9.
"Start-up" is the same command on a module with no declarations.

| checker | wall | start-up | checking |
|---|---:|---:|---:|
| Lean's own kernel (`leanchecker Init.Prelude`) | 254 ms | 200 ms | ~54 ms |
| lean4lean `779c51f` (`--fresh`; 1,975 constants) | 535 ms | 398 ms | ~137 ms |
| oxilean-verify `0abe648` (1,814 declarations) | 598 ms | ~5 ms | ~593 ms |
| oxilean-verify, after | **208 ms** | ~5 ms | **~203 ms** |

Net of start-up, oxilean-verify takes 1.5× lean4lean's time and
3.8× the C++ kernel's on this set (4.3× and 11.0×
before); its figure includes reading the 3.7 MB export. **The 5× budget of V7
is met.** The earlier "~8× slower than lean4lean" was 0.1.3 on another machine,
before structural sharing.

The differential (V6) was re-run with that lean4lean on the same toolchain:
**0 disagreements** over 1,976 joined names (1,810 agree; 166 on one side
only — 162 auxiliary constants the export does not carry, and the four
`Quot` constants, which lean4lean reports as one).

The wasm build (`oxilean-verify-wasm`, run under node) gives the same
per-declaration verdicts as the native binary on nine inputs, including
`init-2000` with its budget-limited declarations, and went from 754 ms to
292 ms on `Init.Prelude`.

**Where the time goes now** (25 s sample of the slice run, 18,635 samples;
inclusive shares, which overlap): building terms. `instantiate` 33%
(allocation included), dropping terms 32%, level instantiation 10%,
`Expr::clone` 8%; computing the cached hash at construction is 5%, hashing
map keys 4%, `canonicalize_pair` 0.1%.

**Open after this work:**

- [x] **Substitution re-allocates the subtrees it skips.** `instantiate_at`
  shares the CHILDREN of an untouched subtree, but copies its root into a new
  allocation (`Node::new(expr.clone())`). Returning the existing `Node` would
  remove an allocation, a free and a clone per skipped subtree; the fuel
  charged for the skip would have to stay what it is. *(Done in the second
  pass, T2 below — and the fuel did not stay what it was: T1.)*
- [x] **`infer_type` has no cache**, so a shared subterm is inferred once per
  occurrence. *(T3 below.)*
- [x] **Budget tail**: 632 clone-fuel roots on `Init`, 111 on the Mathlib
  slice. Peak memory is theirs as well. *(0 and 0, below.)*
- [x] **`def_eq/types.rs` is 2,281 lines** (2,242 before this work; the limit
  is 2,000). *(1,959: the general-purpose types that opened the file moved to
  `def_eq/support_types.rs`, unchanged.)*
- [ ] The committed demo artifact (`web/verify-demo/pkg`) is still the 0.1.3
  build; `cargo formal`'s gates over `crates/oxilean-kernel/formal` were not
  run here (the tool is not installed on this machine; `BigNat` is untouched
  and the package's plain and randomized tests pass). *(2026-11-12: the demo artifact was rebuilt for the 0.1.4 release — 168,785 B gzip, 427,914 B raw. The `cargo formal` gates were still not run for this release.)*

---

## Budget tail, sharing and def-eq work on the 0.1.4 tree (2026-11-11, working copy on `0524489`)

The three items left open by the performance work above — substitution
re-allocating the subtrees it skips, `infer_type` without a cache, the budget
tail — are closed, and with them the tail itself: **no declaration of Lean
core `Init` or of the Mathlib slice runs out of budget any more.**

| corpus | `0524489` | after |
|---|---|---|
| Lean core `Init` (57,277 decls), `--limits corpus` | 47,133 · 10,144 · 0 | **57,277 · 0 · 0** |
| Mathlib slice (28,832 decls), `--limits corpus` | 25,690 · 3,142 · 0 | **27,523 · 1,309 · 0** |
| both, under the DEFAULT limits | — | the same verdicts, declaration for declaration (exit status 0) |

Every declaration verified before is verified after (per-declaration
comparison of the `--json-full` reports); `rejected` stays 0. What is left
`unsupported` on the slice is the four nested inductives the kernel cannot
un-nest (`Lean.PersistentHashMap.Node`, `Lean.PersistentArrayNode`,
`Lean.PrefixTreeNode`, `Lean.Meta.DiscrTree.Trie`) and the 1,305
declarations that depend on them.

**What was wrong.** The tail was not 632 hard declarations. It was three things
the kernel did to terms with shared subterms, and one it did not do:

- [x] **T1. The budget was charged for work that was not done.** A
  substitution that SHARED an untouched subterm charged what rebuilding it node
  by node would have cost — its size as a tree. That had been chosen (wave5) to
  keep verdicts as they were before terms were shared. A proof term of a few
  thousand shared nodes stands for millions of tree nodes (median over the 632
  roots: 1,637 allocations, 287,467 as a tree), so these declarations spent
  2^26 units in a tenth of a second and were cut off. A unit is now a subterm a
  term builder REACHES — rebuilt, shared or found already rebuilt — plus one
  per clone and per application the reducer allocates. Every allocated node is
  still charged. With this change alone (measured separately): `Init`
  47,133 → 53,354 verified, roots 632 → 67, nothing rejected — and 2.3× the
  run time, because the declarations now did their work, as trees.
- [x] **T2. The walks turned shared terms into trees.** `instantiate`,
  `lift_loose_bvars`, `abstract_expr` and `instantiate_type_lparams` rebuilt a
  shared subterm once per path to it. They now keep, per walk, what they
  built for each shared subterm, by address and binder depth
  (`node_memo.rs`; sound because the walk borrows the term for its whole
  life), return an untouched subterm as the allocation it is — the item
  "substitution re-allocates the subtrees it skips" — and put the substituted
  argument in once per depth. β-reduction substitutes all the arguments a
  head has in ONE walk (`instantiate_rev_nodes`), and the reducer passes
  arguments around as the allocations the spine holds, where it copied the
  root of each into a new one.
- [x] **T3. The caches missed exactly the shared terms.** They kept a term
  only if it was at most 4,096 nodes AS A TREE, and compared keys with `==`,
  which follows every edge. `term_cache.rs`: `same_term`, structural equality
  that visits a pair of shared subterms once; `TermCache` / `PairCache`,
  tables keyed by the structural hash with the key verified by `same_term`,
  for terms of any size. On them: the reduction cache (no size limit any
  more), a table of def-eq verdicts for the pairs too big for the equivalence
  manager, **the type-inference cache** (`TypeChecker::infer_type` — the item
  "`infer_type` has no cache"; Lean's `m_infer_type`), the def-eq checker's
  own quick inference, and the instantiated values of universe-polymorphic
  constants (one `omega` certificate unfolds the same constants at the same
  universes tens of thousands of times).
- [x] **T4. `f a ≡ f b` was decided by unfolding `f`.** The def-eq checker
  reduced both sides to weak head normal form first. When `f` recurses on an
  argument that is stuck (`x % USize.size`: the divisor is
  `2 ^ System.Platform.numBits`), that leaves a stuck eliminator with the next
  recursive call in a branch, and comparing two of those walks the recursion
  one unfolding at a time. It now compares the arguments of two applications
  of one unfoldable definition first — the `f a =?= f b` step of Lean's lazy
  delta reduction; `true` is conclusive by congruence, `false` decides
  nothing. This is what cleared the last roots (conversions between `USize`,
  `UInt64` and their signed counterparts), and it cut the work of the `omega`
  certificates as well.
- [x] **T5. The universe of a telescope.** The def-eq checker's quick
  inference built the sort of a `Pi` with the raw `imax`, so an n-binder
  telescope had an n-deep `imax`, and proof irrelevance asks of every pair
  whether that level is `0` — which distributes over every nesting. Two
  inductive types of Std (`Std.DTreeMap.Internal.Impl.WF`,
  `Std.DHashMap.Raw.WF`) had been waiting behind the budget tail; once their
  dependencies verified they took 18 s and more than 30 s (the deadline) to
  check. With `mk_imax`, as in `TypeChecker::infer_type`, 36 ms and 65 ms.
- [x] **T6. Facts on every node.** A `Node` stores three flags beside its
  hash: contains a free variable, a universe parameter, a universe
  metavariable (in the word of the tree size, whose saturation point moves
  from 2^32 − 1 to 2^29 − 1: `REBUILD_COST_MAX`). `has_any_fvar`,
  `has_level_param`, `has_level_mvar` and `has_loose_bvars` are O(1); universe
  instantiation and abstraction pass over what cannot change; the universe
  hygiene check of a declaration looks at each shared subterm once (it walked
  the value as a tree, twice, outside the budget).
- [x] **T7. A declaration must be a closed term** (soundness, Rust API only).
  `check_constant_info`, `check_declaration` and the inductive-family entry
  points now refuse a type, value, recursor rule or constructor type that
  contains a free variable or a loose bound variable, as Lean's kernel does
  (`check_no_metavar_no_fvar`). Free variables are the checker's own device,
  numbered from 0; a declaration that brings one along can only be naming one
  of those. The inference cache of T3 would have made that exploitable: in
  this working copy, before the check went in,
  `(fun (x : F) => x) (FVar 0)` checked as a proof of `F`, the argument being
  given the type remembered for the bound variable (no committed kernel has
  the cache without the check). Pinned by a test that fails with `Ok(())`
  when the check is removed. No export can spell a free
  variable (the format has no record for one), so `oxilean-verify` was never
  exposed; an embedder building `ConstantInfo`s by hand was — and, less
  directly, before this work too (a free variable under a binder of the same
  number passed for the bound one and went into the environment; the
  published 0.1.3 accepts `fun (x : F) => FVar 1 : F → F` and stores it).
- [x] **T8. A small stack: two limits, off by default.** With T1 a declaration
  gets as far as its work takes it, and the kernel is recursive: a reduction
  nested inside another, and every level of a term it walks or frees, is a
  frame of the native stack. The command-line tool has 512 MiB of it. The wasm
  build has the page's megabyte — and where it used to report a deep
  evaluation as over budget, after T1 it threw `RangeError: Maximum call stack
  size exceeded` (a `decide` proof recursing 2,000 deep; `Init` itself). Two
  limits a caller sets on its thread, neither set by default:
  - `fuel::set_reduction_nesting_limit`: a reduction that would nest deeper
    is not started — its term comes back unreduced, the event is recorded,
    and a failure that follows is `unsupported` (`resource limit exceeded:
    reduction depth or step limit`).
  - `fuel::set_term_depth_limit`: the reader, which knows the depth of every
    expression from its tables, does not materialize a declaration with a
    deeper term (the release of a term is a recursion too); and a kernel
    built with the new `term-depth` feature caches the depth of every shared
    subterm (four bytes each; off by default) and treats a node over the
    limit as the end of the declaration's budget. Reported as the new
    `resource limit exceeded: term depth`.
  - `oxilean-verify-wasm` sets 256 and 512 and turns the feature on for
    `wasm32` only. Measured under node 26 (V8: 984 KB of stack, as in Chrome):
    0.80 KB per nested reduction, 0.86 KB per level of a term being compared;
    the deepest combinations the limits allow ran in 312 KB; `Init` and the
    Mathlib slice replay with HALF the engine's stack and half the module's
    own linear-memory stack.
  - Cost to the native tool: one thread-local counter per reduction, 0.3% of
    the instructions of an `Init` run. The verdicts are those of the build
    without it, declaration for declaration.

**How the result was checked.**

- **Against Lean's own kernel, on wrong declarations.** For every theorem of
  both corpora, its proof was checked against the STATEMENT OF ANOTHER theorem
  (the nearest earlier one with a different statement) — by this kernel and,
  through `Environment.addDeclCore`, by Lean's. 59,710 such mutants:
  this kernel accepts 430 (statements that are definitionally
  equal — `symmDiff_left_involutive` is proved in Mathlib by
  `symmDiff_symmDiff_cancel_right _`) and Lean's kernel accepts exactly those;
  it rejects 59,118 and Lean's kernel rejects every one of them;
  on 162 it gives no verdict (a resource limit, or a dependency on a
  nested inductive) where Lean rejects. **No disagreement on any verdict
  given.** The previous build decided fewer (13,421 without a
  verdict) and agreed with the new one wherever it did.
- **lean4lean differential** (V6), re-run: **0 disagreements** over 1,976 joined names (1,810 agree; 166 on one side only), as before.
- **The same mutants under the limits of T8** (256 nested reductions, terms
  512 deep, depth tracked): 9 of the 59,710 move from
  rejected to no verdict, and nothing else moves — no mutant is accepted
  under the limits that is not accepted without them.
- Four new fixtures, each compiled by Lean and with a `leanchecker` run:
  `SharedSubterms` (a 60-level `let` tower, 2^60 additions as a tree:
  57 · 3 · 0 before, 60 · 0 · 0 after, under the default budget);
  `CongruenceFirst` (three `rfl` theorems that need T4, and a fourth that
  Lean's kernel REJECTS — the same function on unequal arguments:
  182 · 2 · 1 before, 184 · 0 · 1 after); `NestedReduction` (a reduction
  nested four hundred deep, and `count 400 0 = 401`, which Lean's kernel
  rejects: 64 · 0 · 1, and 62 · 3 · 0 under a nesting limit of 256);
  `DeepTerm` (a term three hundred levels deep, and a false equation between
  two such terms: 5 · 0 · 1, and 3 · 3 · 0 under a term-depth limit of 100).
  The tests replay the last two under every limit from zero up and require
  that the false theorem is never verified and no sound declaration is ever
  rejected.
- Tests: 33,725 pass in the workspace with every feature on
  (746 skipped), and the three checker crates' 3,888
  as they ship (63 added in this work). Each mechanism above was
  removed in turn — T1's unit, T2's memo, T3's inference cache, big-pair table
  and the reduction cache's size limit restored, T4, T5, T7, and of T8 the
  record of a refused reduction, the reader's refusal, the kernel's refusal
  and the spent budget — and each time a test named for it failed.
- wasm (node 26): the verdicts of the native binary, declaration for declaration, on all of `Init` (57,277) and the Mathlib slice (28,832), in 1.3× and 1.6× its time, and on twelve of the fourteen fixtures; on `NestedReduction` and `DeepTerm` it differs as T8 intends (62 · 3 · 0 and 5 · 1 · 0: the deep theorem and the two false ones are `unsupported`). No exception on any input. A `decide` proof 2,000 or 8,000 recursions deep threw after T1 and before T8; a term 2,000 or 8,000 levels deep threw in every earlier build, `0524489` included.

**Time and memory** (same machine; one verification thread; load average
≈3–6 during the runs):

| measurement | `0524489` | after | |
|---|---:|---:|---:|
| Lean core `Init`, wall | 3 m 58 s | **34 s** | 7.0× |
| Mathlib slice, wall | 43 s | **11 s** | 4.0× |
| `init-2000` slice (`scripts/bench-verify.sh`, median of 3) | 1.37 s (1,463 decls/s) | **381 ms (5,252 decls/s)** | 3.6× |
| `Parity.isEven` fixture (same script) | 26.8 ms | 21.2 ms | 1.3× |
| 1,016-declaration probe, wall / peak RSS | 0.49 s / 30 MB | 0.13 s / 16 MB | 3.6× |
| peak live heap, `Init` (counting allocator) | 2,458 MB | **995 MB** | −60% |
| peak live heap, Mathlib slice | 1,213 MB | **419 MB** | −65% |
| bytes allocated, `Init` / Mathlib slice | 480 GB / 88 GB | 72 GB / 25 GB | |
| peak RSS, `Init` (`/usr/bin/time -l`) | 2.54 GiB | 0.92 GiB | |
| slowest declaration, `Init` | — | 201 ms (`Array.extract_append._proof_1_1`) | |
| wasm artifact (`gzip -9`) | 161,375 B | 168,775 B | |
| wasm, `Init` / Mathlib slice under node 26 | — | 54 s / 20 s | |

The wall times, the benchmark rows and the table of checkers below were
measured on the build just before T8. The final build gives the same reports,
declaration for declaration, with the same peak live heap, and executes
0.3% more instructions over `Init` (536.1 billion against
534.3); by the time it was built the machine was busy with other work
(load average ≈8–25), and its four `Init` runs took 36–52 s.

The peak on `Init` is now the environment itself (995 MB with every declaration of `Init` in it); the largest
amount any one declaration adds to it is 50 MB (`Array.extract_append._proof_1_1`).

**The budget, in the new unit.** Over `Init`: median 2,254 units per
declaration, 99th percentile 249,301, maximum 6,655,688
(`Array.extract_append._proof_1_1`, an `omega` certificate, 201 ms).
The default budget (2^24) is 2.5× the maximum and the corpus
budget (2^26) 10×. The constants are unchanged; their
documentation and `docs/VERIFY.md` now say what a unit is.

**Against Lean's kernel and lean4lean**, same machine, same toolchain, same
module (`Init.Prelude`, end-to-end wall, median of 9):

| checker | wall | start-up | checking |
|---|---:|---:|---:|
| Lean's own kernel (`leanchecker Init.Prelude`) | 256 ms | 207 ms | ~49 ms |
| lean4lean `779c51f` (`--fresh`; 1,975 constants) | 553 ms | 417 ms | ~135 ms |
| oxilean-verify `0524489` (1,814 declarations) | 210 ms | ~5 ms | ~205 ms |
| oxilean-verify, after | **159 ms** | ~5 ms | **~154 ms** |

Net of start-up: 1.1× lean4lean's time and 3.2× the C++ kernel's
(1.5× and 3.8× before). On whole modules of `omega`-heavy proofs the C++
kernel is still well ahead: `leanchecker Init.Data.Array.Extract` takes 1.5 s
of CPU for the module, and its 61 private lemmas alone take this kernel
1.4 s.

**Public API.** `expr::REBUILD_COST_MAX` (new); `Expr::rebuild_cost` /
`Node::rebuild_cost` saturate there instead of at `u32::MAX`.
`check_constant_info` and `check_declaration` refuse terms that are not closed
(T7). The results of `instantiate`, `abstract_expr`, `lift_loose_bvars`,
`instantiate_type_lparams` and the O(1) predicates are what they were; what
they charge to the budget is not. `lift_loose_bvars` and `abstract_expr` no
longer wrap an index of `u32::MAX` round to a small one (it stays where it
is). New: `fuel::{set_reduction_nesting_limit, reduction_nesting_limit,
set_term_depth_limit, term_depth_limit, term_depth_limit_hit,
TERM_DEPTH_TRACKED}`, the feature `term-depth` on `oxilean-kernel`,
`oxilean-export` and `oxilean-verify`, `oxilean_export::TERM_DEPTH_LIMIT`
(a new feature string in reports: `resource limit exceeded: term depth`;
`ExportDecl::Oversized` can carry it), and in the wasm package
`reduction_nesting_limit()` / `term_depth_limit()`.

**Open after this work:**

- [ ] **Nesting depth.** The reducer's own guard stops at 10,000 NESTED
  reductions and reports the declaration as `unsupported` (reduction depth).
  `loop 20000 0 = 20000` and a `List.foldl` over 20,000 elements, both by
  `decide +kernel`, end there; Lean checks them. With the guard lifted in a
  scratch build both verify, on a stack of 32 MiB (not 16); the verification
  thread has 512 MiB by default. A caller can now LOWER the guard to its
  stack (T8); it cannot raise it. Neither corpus has such a declaration.
- [ ] **The recursion is still recursion.** T8 bounds the two things that
  grow with the work — nested reductions and the depth of terms. It does not
  bound every combination of them (a reduction nested 256 deep whose every
  level types a term 512 deep would need more than a megabyte; no input seen
  does this), nor universe levels and names, which are linked structures the
  kernel also walks by recursion. One counter for all of the kernel's
  recursion would. The wasm sizing was measured on V8 only.
- [ ] **Long evaluations.** `(List.range 200000).foldl (· + ·) 0 = …` by
  `decide +kernel` runs out of the corpus budget after 2.3 s (Lean's kernel
  takes about 5 s for it); peak live heap 0.5 GB.
- [ ] **Lazy delta reduction proper.** T4 covers two applications of the same
  definition. Two DIFFERENT heads that unfold to the same one are still
  reduced to weak head normal form first; Lean unfolds one step at a time,
  the taller definition first.
- [ ] **Rejections that end at a limit.** Of the mutants above, 154
  that Lean's kernel rejects end here at a resource limit (reported
  `unsupported`). Never a wrong verdict; a slower "no".
- [x] **`def_eq/types.rs`** was 2,281 lines before this work (the limit is
  2,000) and is 1,959: the general-purpose types that opened it
  moved, unchanged, to `def_eq/support_types.rs`. The other files this work
  touched are under the limit.
- [ ] The committed demo artifact (`web/verify-demo/pkg`) is still the 0.1.3
  build; `cargo formal`'s gates over `crates/oxilean-kernel/formal` were not
  run here (the tool is not installed on this machine; `BigNat` is untouched
  and the package's plain and randomized tests pass); MSRV 1.70 was not built
  (`cargo clippy --target wasm32-unknown-unknown` on the wasm crate reports
  `incompatible_msrv` for code `#[wasm_bindgen]` generates — 16 times before
  this work, 18 after it). *(Later the same day: the MSRV is 1.89 and built
  with 1.89.0, and those warnings are gone — see F2.)* *(2026-11-12: the demo artifact was rebuilt for the 0.1.4 release — 168,785 B gzip, 427,914 B raw. The `cargo formal` gates were still not run for this release.)*

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
  *(2026-11-12: that gate was stricter than Lean's `infer_proj` — a wrong
  REJECT of every projection out of a recursive or an indexed one-constructor
  inductive, so of every ordinary recursive `structure`; measured against
  Lean v4.32.0-rc1. `infer_proj` and the def-eq checker's
  `quick_proj_type` now ask for exactly one constructor (`infer::proj_target`);
  `is_structure_like` stays the condition of structure eta, of eta inside iota
  and of unit-like equality. Fixture `ProjRecIndexed`,
  `crates/oxilean-kernel/tests/proj_structure.rs`.)*
- [x] **S12. The nested K-guard checker reused the surrounding def-eq's
  free-variable ids** (P0 wrong-ACCEPT; found 2026-11-11 while checking an
  external report; present in the published 0.1.3) — K-like reduction types the
  recursor's major premise in a nested `TypeChecker::with_fvar_types` seeded
  with the surrounding locals and asks ITS def-eq whether the major's type is
  the canonical constructor's. That def-eq opened binders from
  `DEF_EQ_OPENED_FVAR_BASE` again, so a binder it opened could carry the id of a
  binder the surrounding def-eq had opened — a local the compared terms mention.
  Comparing `fun y1 y2 y3 => c` with `fun y1 y2 y3 => y3` opened `y3` as `c`,
  the guard passed, K fired on a VARIABLE `h : lhs = rhs` with `lhs ≢ rhs`, and
  `Eq.rec m h ≡ m` was accepted. End to end: `KGuardCollision.bad`, which Lean's
  kernel and `leanchecker` both reject, replayed as 4 verified / 0 rejected
  (0.1.3 and the 0.1.4 tree alike). The overwritten seed entry was never
  restored either, so the nested checker went on mistyping that local.
  *(`with_fvar_types` now keeps both id ranges of the nested checker clear of
  the seeded locals: its own locals start past the largest seeded own-range id,
  and its def-eq's opened ids past the largest seeded opened-range id
  (`DefEqChecker::reserve_opened_fvars_through`). Pinned at three levels: the
  kernel def-eq query (`k_guard_does_not_confuse_an_outer_local_with_a_nested_binder`),
  the seeding invariants (`seeded_checker_*` in `defeq_open_terms`), and the
  negative fixture `KGuardCollision` through `replay_smoke` and the CLI
  (3 verified / 0 unsupported / 1 rejected, exit 1). Real corpora were touched
  by the collision too: on Mathlib's `Con.liftOnUnits` family the reduction
  path changes with the fix (`Con.liftOnUnits_mk`: 2 K-guard evaluations before,
  32 after, about twice the time) — the verdicts are the same.)*
- [x] **S13. A rejected inductive bundle stayed half installed** (P0 wrong-ACCEPT
  from a hand-edited export; found 2026-11-12 by an adversarial pass on the
  projection fix; present in the published 0.1.3 and 0.1.4) — the export
  replay pre-checked a bundle against the kernel's derivation and then added
  the exported types, constructors and recursors ONE BY ONE through
  `check_constant_info`; a member refused late returned the error with the
  earlier members already in the environment, and the cascade to dependents
  only fired on `UnknownConstant`. A bundle whose type lists one of its two
  constructors (the second constructor's record still in the bundle: refused,
  "not listed") left a one-constructor type and that constructor behind. A
  projection out of it and structure eta on it then checked:
  `∀ t : Two, t = Two.mk t.0` by `rfl`, false of the other constructor, was
  reported verified, and so was any declaration naming only the type of a
  bundle refused at a constructor. `replay.rs` documented "a failing bundle
  installs nothing".
  *(A bundle is now added whole or not at all: `check_constant_infos_atomic`
  checks the group in order and takes the earlier members out on a failure;
  the kernel-derived family of a nested inductive goes through
  `add_derived_family` and `Environment::add_constants`, which adds nothing
  unless every name is free.
  Second line: every name of a failed record joins the rejected (or deferred)
  set, and `Replayer::blocked_by_an_earlier_failure` refuses a record that
  mentions such a name — as a constant or as the structure of a projection —
  before the kernel sees it, whether or not a constant of that name is
  present; a record that declares such a name again is rejected. Pinned by
  `group_atomic` (kernel), the bundle tests of `replay_bundles` (export), and the
  hand-edited negative fixture `CtorListLie` through `replay_bundles` and the CLI
  (3 verified / 0 unsupported / 7 rejected, exit 1; 5 / 0 / 5 on 0.1.3 and
  0.1.4).)*
- [x] **S14. The export replay took `isRec` (and more) from the file** (P0
  wrong-ACCEPT from a hand-edited export; found 2026-11-12; present in the
  published 0.1.3 and 0.1.4) — the exported `InductiveVal` was installed with
  the file's `isRec`, which `is_structure_like` reads: a recursive
  one-constructor type recorded as `isRec: false` got structure eta, eta
  inside iota and unit-like equality (two declarations Lean's kernel rejects
  verified). `isReflexive`, `numNested`, the `isUnsafe` flags and, on the
  nested path, `numIndices` and every constructor field were not compared
  with anything either (none of those changed a verdict by itself).
  *(Every field of a bundle that Lean's kernel computes is now compared with
  this kernel's derivation, `replay::compare_with_derivation`, and a
  difference rejects the bundle before anything is added. The derivation of
  `is_rec` / `is_reflexive` was brought to Lean's: for the family as a whole,
  from the constructor types as written — measured on Lean v4.32.0-rc1 with
  raw declarations, where the previous reading, after reduction, differed on
  a field of type `K T` with `K` dropping its argument, and on a field whose
  type is a function into the family only after unfolding. Pinned by
  `what_a_bundle_says_beyond_its_declaration_is_held_to_the_derivation`
  (`replay_bundles`, 21 edits of one field each) and the two flag tests of
  `group_atomic`. NOT closed here: the safety of a declaration. Lean's kernel
  refuses a safe declaration that uses an unsafe or a partial one; this
  kernel reads neither `isUnsafe` nor `safety`. lean4export writes no unsafe
  declaration, so it takes a hand-made file or a declaration added without
  Lean's kernel check.)*
- [x] **S15. A recursor record was not tied to its type** (P0 wrong-ACCEPT from
  a hand-edited export; found 2026-11-12; present in the published 0.1.3 and
  0.1.4) — `verify_recursor_val` accepted a recursor that matched ANY derived
  recursor of its family and the replay only counted the records: in a mutual
  block `B.rec` could be a renamed copy of `A.rec`, or the two names
  exchanged, and a declaration that applies `B.rec` to an `A` verified. On
  the nested path a record was paired with the derived recursor whose rules
  fire on the same constructors and gave it its NAME, so two recursors could
  be exchanged throughout a bundle. A recursor's rules were looked up by
  constructor name, in any order.
  *(A record is compared with the derived recursor of its own name — `T.rec`;
  on the nested path `<first type>.rec_<k>` for the `k`-th auxiliary type,
  `nested_recursor_name` — rule by rule in the constructors' order. The same
  change repairs a wrong reject of honest Lean: two auxiliary types with the
  same constructors (`A | mk : List B → A`, `B | mk : List A → B`;
  `T | mk : List (List T) → T`) could not be told apart by their rules.
  Pinned by `declaration_conditions` (kernel) and five tests of
  `replay_bundles` (export).)*
- [x] **S16. A theorem did not have to state a proposition** (P1 wrong-ACCEPT;
  found 2026-11-12; present in 0.1.3 and 0.1.4) — `theorem t : Nat := 5`
  verified; Lean: "type of theorem 't' is not a proposition". *(Both theorem
  branches of `check/functions.rs` go through `check_theorem_type`. Known
  difference kept: a sort that is zero only after normalisation and has no
  parameter, `Sort (max 0 0)`, is accepted; Lean compares it as written.)*
- [x] **S17. Quotients were installed over a misshapen `Eq`** (P0 wrong-ACCEPT
  from a hand-made export; found 2026-11-12; present in 0.1.3 and 0.1.4) —
  `add_quot` asked for an inductive `Eq` in `Prop` with one constructor; with
  `refl : (a b : α) → Eq a b`, a unary `Eq`, an `Eq` over `Type u` only, or
  two universe parameters, the quotient records verified. *(`check_eq_present`
  compares `Eq`'s type and its constructor's type with the ones the
  primitives assume, as Lean's `check_eq_type` does; measured against Lean on
  fourteen shapes, three of which Lean accepts and so does this kernel.)*
- [x] **S18. An inductive declaration was read more generously than Lean reads
  it** (P0 wrong-ACCEPT from a hand-made export; found 2026-11-12; present in
  0.1.3 and 0.1.4) — (a) an index of a constructor's result, or of a
  recursive field, could mention a type of the family:
  `inductive I : Prop → Prop | mk : I (I True → False)` derived, and with
  propositional extensionality that type is inconsistent; (b) parameters, a
  constructor's fields and its result were found by unfolding definitions,
  where Lean reads them off the type as written. *(`as_valid_ind_app`
  refuses an index that mentions the family; `build_family_pre`,
  `peel_against_params` and `check_ctor` read parameters, fields and result
  as written; a type's indices and sort may still be behind a definition, as
  in Lean.)*
- [x] **S19. A replayed file's `Nat.pred` was computed by the reducer's own
  rule** (P0 wrong-ACCEPT from a hand-made export or a non-core environment;
  found 2026-11-12; present in 0.1.3 and 0.1.4) — the literal rules for
  `Nat.pred`, `Nat.blt`, `Nat.log2`, `String.length`, `String.append`,
  `String.beq` belong to the builtin environment, which declares those
  operations without a definition; they fired on any constant of those
  names. In a file that defines `Nat.pred` as the identity, `Nat.pred 5 = 4`
  checked by `Eq.refl`. Lean's kernel computes none of the six by name.
  *(`lit_op_arity(name, primitives)`: the six fire only when
  `Environment::has_literal_primitives()`, which `init_builtin_env` sets.
  SHARED LIMIT, not closed: the operations of Lean's own extension —
  `Nat.add`, `sub`, `mul`, `div`, `mod`, `gcd`, `beq`, `ble`, `land`, `lor`,
  `xor`, `shiftLeft`, `shiftRight`, `pow`, `Nat.succ` on a literal, a string
  literal against `String.ofList` — are computed by name in every
  environment, here as in Lean's kernel (measured with each declared as an
  axiom): both believe an environment about them.)*
- [x] **S20. A second `quot` record verified** (P2; found 2026-11-12) — *(the
  replayer keeps the primitives a `quot` record has declared and rejects a
  second record of one.)*
- [ ] **S21. The safety of a declaration is not read** (difference from Lean
  in which declarations may be used where; found 2026-11-12) — a safe
  declaration that uses an unsafe or a partial one verifies; Lean refuses it.
  Nothing is relaxed for a flag (measured: an unsafe inductive with a
  non-positive occurrence, an unsafe and a partial definition that refer to
  themselves, ill-typed unsafe declarations are all rejected, and every
  verdict is the same with the flags cleared).

- [x] **S22. A recursor record was installed after a comparison by definitional
  equality alone** (P0 wrong-ACCEPT from a hand-edited export: a closed proof
  of `False` from no axiom; found 2026-11-12 by an adversarial pass; present
  in the published 0.1.3 and 0.1.4) — `verify_recursor_val` compared an
  exported recursor's type and rule right-hand sides with the derived ones by
  `is_def_eq`, `check_constant_info` then added the EXPORTED recursor, and
  iota ran on its rules; neither term was type-checked. Proof irrelevance
  reads the type of a proof off its head (`quick_infer_type` does not check
  an argument against its domain), so `mk b (B.rec (motive := fun x => P x)
  (P.mk B.f) (P.mk B.f) b)` compared equal to `mk b h`, and
  `T.rec .. (T.mk B.t (P.mk B.t))` reduced to `mk B.t (P.mk B.f)`. The same
  through the recursor's type in a family with a proof index. On the nested
  path the derived family was installed already, the exported recursors
  compared the same way (and in an environment without the family). The
  `quot` records' types and `Quot.sound`'s were compared by `is_def_eq`
  before (or without) a type check as well.
  *(The invariant: a term from outside is in the environment, or reaches
  definitional equality, WHNF or iota, only after the kernel type-checked it;
  where the kernel derives a term, the derived term is installed and the
  supplied one only held against it. `check_constant_info` adds the DERIVED
  recursor (`verify_recursor_val` returns it, under the supplied name and
  universe parameter names); `inductive::supplied` compares a supplied term
  as written (`same_up_to_binders`, the supplied universe parameter names
  taken first) and otherwise type-checks
  it — in a private copy of the environment that holds the kernel's family —
  before `is_def_eq`. A comparison as written alone would refuse honest
  Lean: this kernel's derivation keeps `γ : outParam (Type w)` where Lean's
  recursor has `γ : Type w` (98 of `Init`'s 610 recursors). Pinned by
  `derived_recursor` (kernel), `replay_installed` (export), the fixtures
  `RecRuleTruth` / `RecRuleJunk` / `RecRuleControl` and the CLI (7 verified /
  0 unsupported / 8 rejected on `RecRuleJunk`; 14 / 0 / 1 on 0.1.3 and
  0.1.4, `boom : False` among the verified).)*
- [x] **S23. The types of a block were checked with the block already
  declared** (P1 wrong-ACCEPT from a hand-made export; found 2026-11-12;
  present in 0.1.3 and 0.1.4) — `check_and_derive_family` stages the block's
  types into a working environment (the constructor types need them) and
  type-checked the types themselves there, so `mutual C : Type; A : C → Type`
  — an inductive-inductive definition — was accepted as an ordinary mutual
  family; Lean: "unknown constant 'C'". *(`check_types_do_not_mention_the_block`;
  a nested declaration whose auxiliary type would be `T → Type` is refused
  by the same check, as by Lean.)*
- [x] **S24. A nested constructor record could name the auxiliary type** (P1
  wrong-ACCEPT from a hand-made export; found 2026-11-12; present in 0.1.4)
  — `b : T._nested_0 → T` beside `a : List T → T` type-checked in the
  specialized family and was installed as `b : List T → T`; Lean: "unknown
  constant". *(`verify_nested_bundle` requires restore ∘ specialize to give
  back every declared type and constructor type, `declared_family_restored`.)*
- [x] **S25. A recursor record under a name Lean never gives** (P0
  wrong-ACCEPT from a hand-edited export: a closed proof of `False` from no
  axiom; found 2026-11-13 by an adversarial pass; present in the published
  0.1.3 and 0.1.4) — `verify_recursor_val` derives a family's recursors in
  the style of the record it is asked about: Lean's `A.rec` (the component
  `rec` under `A`) or the one flat component `"A.rec"` the kernel's own
  tests write. A mutual bundle with one record of each style was installed
  with rules that called recursors no one installed (`C.rec` calling the
  hierarchical `A.rec`); a later `def A.rec` answered the call, and
  `boom : False` verified (13 / 0 / 0, exit 0). *(The export replay accepts a
  recursor record only under Lean's names, `T.rec` or `<first type>.rec_<k>`
  (`check_recursor_record_names`); and the kernel guards the installs of
  the export replay's two bundle paths and of the quotient primitives
  (`inductive::installed`): each constant a derived term names resolves in
  the environment the install leaves, and a name the derivation gives a
  recursor or a type of the family is that constant — in
  `check_constant_infos_atomic` (the ordinary path), in
  `add_derived_family` (the nested path) and in `add_quot`.
  `add_inductive_family` is neither atomic nor guarded (S27). Pinned by `derived_references` (kernel, both paths),
  `replay_installed` (export), the fixtures `RecNameMixedStyle` /
  `RecNameMixedStyleTruth` / `RecNameFlat` and the CLI (8 / 0 / 5 and
  7 / 0 / 8, exit 1).)*
- [ ] **S26. A name Lean's kernel gives a constant of a refused bundle is
  free** (difference from Lean's replay, older than 0.1.5; found 2026-11-13)
  — a bundle refused for its records installs nothing, and the rejected set
  holds the names as the records spell them; a later record that declares
  the name Lean's kernel gives a constant of the declaration (`def A.rec`
  after a bundle whose record was `«A.rec»`) is checked as the declaration
  of a free name and verifies, where Lean's replay, which keeps the
  kernel's constants of the declaration, finds the name taken. Well typed;
  every record that names the bundle is refused. 0.1.3 and 0.1.4 alike.
- [ ] **S27. `add_inductive_family` is neither atomic nor guarded** (kernel
  API, older than 0.1.5; found 2026-11-13 by an adversarial pass; the same
  on the base) — it derives the family (nothing is added when that fails)
  and then adds the constants one by one with `Environment::add_constant`,
  without the guard of `inductive::installed`. With `axiom C.rec : B`
  declared, the mutual block `A | mk : (b : B) → P b → C → A ;
  C | base | step : A → C` returns `duplicate declaration: C.rec` and
  leaves `A`, `C`, `A.mk`, `C.base`, `C.step` and `A.rec` installed, and
  the installed `A.rec`'s rule for `A.mk` names `C.rec`, the axiom. Not a
  verdict of `oxilean-verify`: the export replay never calls it (it uses
  `check_constant_infos_atomic` and `add_derived_family`), so no file
  reaches it. Its callers: `init_builtin_env` (`builtin/functions.rs`,
  `add_builtin_inductive`), `InductiveEnv::register_in_env`
  (`inductive/types.rs`) and the kernel's tests. The one-line change:
  `add_derived_family(env, fam.into_constant_infos())` in place of the
  `add_constant` loop, which makes it atomic and guarded. Not made: it
  changes how the builtin environment is built (the guard would then run
  on every member of each builtin family, its types and constructors
  too), and that has not been measured — first `init_builtin_env` under
  the guard, then the whole workspace's suites. *(Reproducer: the
  declarations above, handed to `add_inductive_family` — `axiom C.rec : B`
  first, then the mutual block. The original probe is section 7 of
  `ISSUE-4.md` on the cargo-formal side, which is not in this
  repository.)*
## P0 — Completeness bugs (kernel rejects real proofs → false `rejected` alarms)

- [ ] **C25. A container nested twice gets two auxiliary types** (wrong REJECT
  of honest Lean; found 2026-11-12; present in 0.1.4) —
  `inductive T | mk : List T → Arr T → T` with `Arr α | mk : List α → Arr α`:
  Lean makes ONE auxiliary type for `List T`; `inductive::nested::specialize`
  makes a second one when it meets `List T` again inside the auxiliary copy
  of `Arr`, derives four recursors where the export has three, and the
  bundle is rejected.
- [ ] **C28. A universe-polymorphic nested inductive is rejected** (wrong
  REJECT of honest Lean; found 2026-11-13; present in 0.1.4) —
  `NU.{u} : Type u | mk : LU.{u} NU.{u} → NU.{u}`: `specialize` refers to
  the auxiliary type with no universe arguments while the staged auxiliary
  constant carries the family's universe parameters (`universe parameter
  count mismatch for NU._nested_0: expected 1, got 0`). The fix changes how
  the auxiliary constant takes universe parameters.
- [ ] **C29. The order of nested occurrences matters** (wrong REJECT of
  honest Lean; found 2026-11-13; present in 0.1.4) — `T | mk : List T →
  Option (List T) → T` is rejected, `Option (List T) → List T → T` and
  `Option (List T) → T` verify. With `List T` specialized first, the
  auxiliary type of `Option (List T)` is recorded with its argument already
  the first auxiliary type, and restoration does not give the declared
  constructor type back (by reading; the message, `'T.mk' is not declared
  with the type the kernel derives for it (a declaration must not name an
  auxiliary type of its own nesting)`, does not fit: the declaration names
  none). C25's neighbour.
- [ ] **C26. An inductive bundle gets no second attempt past the reducer's
  chain cap** (no verdict where Lean accepts; found 2026-11-12) — a
  constructor whose type needs a head-reduction chain longer than the cap is
  `unsupported: reduction depth or step limit` at both limit presets; a
  definition in the same situation is checked again with the cap lifted
  (`Replayer::check`), a bundle is not.
- [ ] **C27. A definition whose hints say `opaque` is not unfolded** (wrong
  REJECT; found 2026-11-12 with a hand-edited export) — Lean's kernel uses a
  definition's hints only to choose which side to unfold first.

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
- [x] **C10. Nested inductives** — decorative bool flag; no nested-to-mutual
  compilation. Implement, or emit named `unsupported` verdict (three-bucket rule). (`audit-recursors.md`)
  *(Wave 5, after the 0.1.3 release: implemented in `b3c17f8`/`b3adf3f`.
  Nested occurrences are specialised into an auxiliary mutual family, the
  recursors are derived and restored over the real container, and every
  exported recursor is re-verified by def-eq.
  `crates/oxilean-kernel/src/inductive/nested.rs` still answers
  `UnsupportedNestedInductive` for the shapes that stage does not handle.
  `Init` went from 35,223 to 47,119 verified with 0 rejected; re-measured on
  2026-11-04 at `86227a8`: 47,119 · 10,158 · 0.)*
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
- [x] **C23. A head that stepped to an application lost its outer arguments**
  (P0 wrong-REJECT class, reported externally 2026-11 on Mathlib's
  `AddCon.addGroup._proof_1`; also `AddCon.addGroup._proof_4`,
  `Con.group._proof_1` / `_proof_4`, `Quot.congr_mk`, `Quotient.congr_mk`,
  `OrderHom.antisymmetrization_apply_mk`,
  `Antisymmetrization.prodEquiv._proof_3`,
  `OrderIso.dualAntisymmetrization._proof_8`, and the two FreeGroup rejections
  of wave5; present in 0.1.3) — C21's in-context head loop stepped a head that
  had BECOME an application (a constant whose value is a partial application, a
  projected instance field, a `let` body) on its own. Mathlib writes its
  quotient maps point-free — `Quot.map f h := Quot.lift (fun x => ⟦f x⟧) _` —
  so the exposed `Quot.lift` is one argument short and its major premise is an
  OUTER argument: the bare head looked stuck and the application came back
  unreduced. The same happened to a partially applied recursor
  (`def pred : Nat → Nat := Nat.rec 0 (fun n _ => n)`), and a partially applied
  literal operation (`Nat.add 2` awaiting an outer `3`) was unfolded instead of
  folded.
  *(The head loop re-attaches the outer arguments and hands the application
  back to the chain loop, which re-flattens the spine — Lean's
  `whnf_core(mk_rev_app(f, args))`. Pinned in the kernel for each way a head
  becomes an application (delta, `let`, projection) and each operation
  (`Quot.lift`, `Quot.ind`, a recursor, a literal op), and end to end by the
  fixture `QuotPartialApp` (66 verified; 63 + 3 rejected before).)*
- [x] **C24. The reducer's step cap turned long reduction chains into
  rejections** (P0 wrong-REJECT; found 2026-11-11; present in 0.1.3) —
  `whnf_env_depth` stopped a head-reduction chain after `max_depth` (10,000)
  steps and returned the unfinished term as if it were a normal form, without a
  trace; the chain's steps were also charged against the NESTING depth, so the
  nested reductions of a long chain's later steps started out over the limit.
  `chain 20000 = true := by decide +kernel` — tens of thousands of cheap steps
  over a term that never grows — was REJECTED under both `--limits` presets
  although no budget had been exceeded, against the contract that a declaration
  over its resources is unsupported, never rejected.
  *(The cap stays — it is also what lets def-eq abandon a reduction that is
  going nowhere: lifting it outright under a budget cost 30 verified
  declarations on the Mathlib slice, 17 of them new clone-fuel roots — but it
  now latches `fuel::note_reduction_limit`, as does the nesting-depth guard.
  A `Replayer` with a budget that sees a declaration FAIL with the latch set
  repeats the check under a fresh budget with the cap lifted
  (`fuel::set_reduction_cap(false)`): the budget bounds the chain, steps are no
  longer charged against the nesting depth, and the second result is the
  verdict. Where the check cannot be repeated (no budget armed, the stateless
  entry points, the nesting-depth guard) the failure is the named unsupported
  `REDUCTION_LIMIT`, never a rejection. Pinned in the kernel (capped and
  recorded by default; runs to the end with the cap lifted under a budget; the
  cap cannot be lifted without one; nesting guard) and by the fixture
  `ReductionChain`: 36 verified under a budget and through the CLI; with no
  budget 34 checked / 2 unsupported / 0 rejected.)*

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
  *(2026-11-11, 0.1.4 tree: re-measured against lean4lean and against Lean's
  own kernel on one machine with one toolchain; **budget met**. Figures in
  "Performance work on the 0.1.4 tree" near the top.)*

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
  *(Wave 4: unified to "COOLJAPAN OU (Team KitaSan)" across LICENSE and crate
  metadata.)*

## P3 — Deliberate follow-ups (out of this pass, tracked)

- [>] **F1. TCB slimming** — kernel is 146,570 raw lines; SplitRS filler types
  (TokenBucket, SimpleDag, StringPool…) duplicated across modules, and non-TCB
  modules (simp, typeclasses, match_compile, ffi, abstract_interp, unif_hints,
  congruence, termination) are publicly exposed. Quarantine toy `quotient/`
  re-exports now (part of C12); full de-bloat is a separate reviewed pass —
  the "auditable by eye" story depends on it.
- [x] **F2. MSRV verification** — `rust-version = "1.70"` never tested; add CI job or
  bump honestly.
  *(2026-11-11: bumped honestly to `rust-version = "1.89"`, the floor the locked
  dependencies already set (`safe_arch` 1.2.0 and `wide` 1.7.1 declare 1.89) —
  1.70 never built. `cargo +1.89.0 check --workspace --all-targets
  --all-features` passes. Clippy 1.99.1 is clean at the new MSRV: the 202
  suggestions it unlocks (`is_multiple_of`, `div_ceil`, `repeat_n`,
  `is_none_or`, `inspect_err`, `as_chunks`, …) are applied, with the 21 that
  1.99's clippy raises at any MSRV; on wasm32, `oxilean-verify-wasm` and
  `oxilean-wasm` report no `incompatible_msrv` any more. `try_update` (1.95)
  is out of reach at this MSRV, so `AtomicRefCounter::dec` spells out the
  compare-and-swap loop instead of the deprecated `fetch_update`. No CI job
  yet — CI runs only `npm-publish.yml`.)*
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
inductives (C10), G7 wasm half, F1–F4.) Owner: Team KitaSan.*

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
  - [x] **Fix the 2 FreeGroup false rejections** (`FreeGroup.instGroup._proof_12`,
    `FreeGroup.induction_on`). DIAGNOSED as a def-eq **completeness** gap (NOT
    unsoundness): `@rfl (1⁻¹ * 1)` proves the group law `1⁻¹ * 1 = 1`, definitional
    in FreeGroup (a `Quot` of lists) but the kernel fails to reduce the nested
    `HMul`/`Inv` instance-unfolding + `Quot.lift` on the identity. Minimal
    reproducer (0.43 s / 11.7 MB): `~/work/oxilean-corpus/FreeGroup_rejections_repro.ndjson`.
    Fix = complete that reduction path; do NOT blanket-reclassify def-eq
    failures as unsupported (that would hide genuine corrupted-proof rejections).
    *(Done 2026-11-11 by C23: the instance field is a point-free `Quot.lift`,
    and its major premise was an outer argument. The closure of the two
    declarations, re-exported from Mathlib `v4.32.0-rc1`, goes from 416
    verified / 3 rejected to 419 verified / 0 rejected.)*
