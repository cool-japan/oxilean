# Verifying Lean 4 core (`Init`) with oxilean-verify — the first full-corpus number

*2026-07-15 · oxilean `9b5b435` (branch `0.1.3`) · one machine, one process, one pass*

## Headline

Over **all 57,277 declarations** of Lean 4 core's `Init` export (v4.32.0-rc1,
6,453,270 NDJSON lines, 345 MB):

```
35223 verified · 22054 unsupported · 0 rejected      (exit status 0)
```

| bucket | count | share |
|---|---:|---:|
| **verified** | 35,223 | 61.5% |
| **unsupported** (named buckets, below) | 22,054 | 38.5% |
| **rejected** | 0 | 0% |

Every declaration got exactly one verdict; nothing was skipped silently. The
unsupported bucket is dominated by a single root cause (one nested-inductive
type, `Lean.Syntax`) whose dependency cascade accounts for 98.9% of it.

## What "0 rejected" means (and what it does not)

`rejected` is an alarm verdict: *we checked this declaration against our
independent implementation of the Lean 4 kernel rules and it is not a proof.*
A wrong `rejected` on Lean core would mean our kernel disagrees with Lean's —
and every earlier disagreement of this kind (172 `noConfusion`-class
rejections, 20 lean4lean-diff disagreements, the UTF-8 string-literal class,
the out-of-context whnf class) turned out to be a completeness bug on our side
and was fixed (C15, C18–C21 in `TODO_VERIFY.md`). Zero rejections therefore
means: **nothing in Lean core was mis-flagged, and the checker found no
incorrect proof** — as expected, since `Init` is checked by Lean's own kernel
on every build.

It does **not** mean every declaration was verified: the checker refuses to
guess. Anything it cannot check with exact Lean semantics lands in a *named*
unsupported bucket — never in `verified`, never in `rejected`.

## The unsupported bucket: 178 roots, 21,876 followers

| named feature | roots | root cause | cascade reach |
|---|---:|---|---:|
| nested inductives | 1 (`Lean.Syntax`) | the kernel does not implement Lean's nested-to-mutual inductive elaboration yet | 21,625 followers (98.9%) |
| resource limit exceeded: per-declaration clone-fuel budget | 175 | legal-but-heavy kernel reduction: the 2^26-node clone-fuel budget (C16/C16b) cuts the declaration off before its multi-GiB transient | 7,526 followers (incl. joint) |
| declaration materializes too large (per-declaration budget) | 2 | C22 (below): the exported DAG expands to >2^25 tree nodes for one declaration | 16 followers (incl. joint) |
| dependency on an unsupported declaration | — | followers of the above | 21,876 |

Follower attribution (exact, computed by forward-propagating root reachability
over the corpus in dependency order):

| followers trace to | count |
|---|---:|
| nested inductives only | 14,347 |
| nested inductives + clone-fuel | 7,265 |
| clone-fuel only | 248 |
| all three | 13 |
| oversized only | 3 |
| **unattributed** | **0** |

So the cascade discipline holds: **one** missing kernel feature
(nested inductives — `Lean.Syntax` is declared at decl #1,373 and everything
parser/meta-flavoured in `Init` depends on it) explains two-thirds of the
whole unsupported bucket by itself, and joint Syntax+fuel paths most of the
rest. Implementing nested-to-mutual elaboration is the single highest-leverage
completeness item left.

### The resource-limit tail (175 declarations)

Dominated by fixed-width integer lemmas (UInt8/16/32/64, USize, Int8/16/64 —
2^64-scale modular arithmetic folded in the kernel), `Init.Data.Array.Extract`
let-tower private proofs (the C16b family), iterator/range lemmas, and
`WellFounded` unfolding lemmas. Spot-checks with a raised budget (`FUEL=2^30`,
memory-caged dependency-closure replay) confirm these are *legal but heavy*,
not wrong:

| declaration | verdict at 2^30 fuel | fuel used | check time |
|---|---|---:|---:|
| `Array.foldlM_toList.aux._unary` | verified | 506.8 Mnodes | 42.8 s |
| `Int64.ofBitVec_sdiv` | verified | 202.9 Mnodes | 19.0 s |
| `UInt64.toUInt32_mul` (C21 measurement) | verified | 100.2 Mnodes | — |

The corpus budget stays at 2^26 nodes deliberately: one charged node ≈ 60–100
bytes of peak term memory, so 2^30-node declarations mean tens-of-GiB
transients — exactly what a 14 GiB machine must refuse deterministically
rather than OOM on. The budget is per-declaration and deterministic; the same
input always exhausts at the same point.

### C22: the declaration that OOM-killed the first full run

The first full-corpus attempt (run7, at commit `8b57810`) stalled at decl
#42,086, `WellFounded.partialExtrinsicFix₃_eq_partialExtrinsicFix`, pinned
against the cage's 16 GiB swap ceiling with the kernel's clone-fuel latch
never firing. Root cause (confirmed by an isolated caged replay that OOM-ed a
12 GiB cage even with fuel at 2^23, and by gdb stack sampling): the blowup was
in the **export reader**, not the kernel — the reader's materialization budget
was *cumulative only*, and this single theorem's DAG-shared type+value legally
expand to >100 M tree nodes (≈7+ GiB) because the kernel `Expr` has no
structural sharing. The kernel never got to spend a single unit of fuel.

The fix (`9b5b435`): `Limits::decl_materialize_budget` — a per-declaration cap
checked in O(1) against the reader's memoized size table *before any expansion
work*. An over-budget declaration streams through as the named
`ExportDecl::Oversized` event (its names read from the interned table only; no
term is ever fabricated), the replayer defers those names, and dependents
cascade as ordinary followers. Corpus preset: 2^25 nodes (≈2–3.4 GiB per
declaration). A full-corpus calibration scan found exactly **two** declarations
over the cap — #18,697
(`…ForwardSliceSearcher.Invariants.isValidSearchFrom_toList`, which was
already an unsupported follower before the cap) and #42,086 itself — and the
largest fitting declaration at 29.67 Mnodes. No previously-verified
declaration was affected. A pleasant side effect: a read-only streaming pass
over the corpus dropped from ~8.4 GiB to **3.79 GiB** peak RSS — the two
monsters were most of the read peak.

## Memory story: a 345 MB file that wants ~16 GiB

The lean4export format is a shared DAG; our kernel `Expr` is an unshared
`Box` tree. Materialization therefore multiplies: the reader's index tables
plus the growing kernel environment reach ~10 GiB, and heavy declarations add
multi-GiB transients on top. On this 14 GiB machine every corpus-scale process
runs inside a systemd memory cage, and the official run used a swap-permitting
cage so cold pages (already-checked declaration bodies, cold table entries)
spill to swap while RSS stays hard-capped and the host keeps headroom:

```
systemd-run --user -p MemoryMax=12G -p MemoryHigh=10G -p MemorySwapMax=16G ...
```

Observed (20 s RSS sampling, `run9.rss`): RSS climbs to ~10.4 GiB by decl
~7,500 and plateaus there for the rest of the run — `MemoryHigh` reclaim holds
it — while process swap grows to a peak of **6.7 GiB** in the heavy tail
(total peak footprint ≈ 16.2 GiB). `/usr/bin/time -v`: peak RSS
**10,465,588 kB (9.98 GiB)**, 2.48 M major page faults, 96% CPU — the run
stayed compute-bound, not swap-thrashed.

History that led here: the previous full-run attempts died at decl 41,541
(run6: hard 12 GiB cage, no swap allowance — cage kill) and stalled at decl
42,086 (run7: C22, above). With C22 fixed the same cage completes the corpus.

**Tracked fix:** structural sharing in the kernel `Expr` (hash-consing/`Rc`)
removes both the memory wall and the dominant throughput cost. It is
deliberately **not** part of this milestone — it is surgery on the trusted
computing base and gets its own review cycle.

## Throughput — honest numbers

| metric | value |
|---|---|
| wall clock (whole run, incl. read) | **1 h 20 m 44 s** (4,844 s) |
| overall throughput | **11.8 decls/s** |
| CPU utilisation | 96% |
| peak RSS | 9.98 GiB (cage-held) |
| machine | Intel i7-1270P (16 threads), 14 GiB RAM + 50 GB swap, Linux 7.0.0-27-generic, rustc 1.95.0 |

The brief's budget — within 5× of the lean4lean baseline (~3,280 decls/s
measured by the diff harness, i.e. ≥ ~656 decls/s) — is **not met**: we are
~55× under budget (~278× slower than lean4lean). The gap is structural, not
incidental: with 96% CPU and a RAM-resident 2 k-decl slice benchmarking at
only ~41 decls/s, the cost is dominated by deep-clone substitution and
whole-tree traversals over the unshared `Box` `Expr` — the same root as the
memory wall, with the same tracked fix (structural sharing). We are not going
to chase constant-factor wins under a 14 GiB ceiling when the representation
change is both the correct fix and already on the roadmap; the number is
reported as-is.

## Reproduction

```sh
# pins
#   lean4export  3de59f10bc4b4a0f2de698597aeb1246caa0df0a
#   Lean         v4.32.0-rc1 (githash b4812ae53eea93439ad5dce5a5c26591c31cb697)
#   NDJSON       3.1.0
#   corpus       Init.ndjson  sha256 9e824677f2cb5c8504e4a8c2c2ee72eeca5ad874fd5f2f7f2138400770b2744a  (345,683,034 bytes)
#   oxilean      9b5b435 (branch 0.1.3), rustc 1.95.0

cargo build --release -p oxilean-verify
./target/release/oxilean-verify --version   # prints the pins above

# full corpus, memory-caged exactly as the official run
# (run as a transient unit; --scope in an interactive shell works too):
systemd-run --user --unit=init-verify \
  -p MemoryMax=12G -p MemoryHigh=10G -p MemorySwapMax=16G \
  -p StandardOutput=append:init-verify.log -p StandardError=append:init-verify.log \
  --working-directory="$PWD" -- \
  /usr/bin/time -v ./target/release/oxilean-verify --limits corpus \
  --no-color --json init-report.json ~/work/oxilean-corpus/Init.ndjson
```

Exit code 0 = ran to completion with zero rejections. The machine-readable
summary of the official run is committed as
[`2026-07-15-lean-core-init.json`](2026-07-15-lean-core-init.json).
