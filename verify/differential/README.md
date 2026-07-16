# Differential testing: oxilean-verify vs. a second kernel

> *"A disagreement is not a bug report. It is a finding."* — engineering brief §5

This directory holds the **differential-testing harness** for the OxiLean verify
campaign. It runs two independent Lean 4 kernels over the same declarations and
joins their per-declaration verdicts, so that when they disagree we know
*instantly* which of two conversations we are in: a bug in *our* kernel, or
something worth reporting about the other one.

The second kernel is [`lean4lean`](https://github.com/digama0/lean4lean) — a Lean
4 kernel written (mostly) in Lean itself. It is not a fully independent
implementation (its README notes it is "derived directly from the C++ kernel"),
but it is a second checker, written by someone else, in a different language,
maintained separately — exactly the cross-check differential testing wants, and
the natural source of a throughput **baseline** (brief §8.2).

---

## The methodology

1. **Same declarations.** Both checkers must see the *same* mathematical
   content. oxilean-verify reads lean4export **NDJSON**; lean4lean reads Lean
   **oleans** off the search path. They never read the same file, so we join on
   fully-qualified **declaration name**, which is stable across both formats.

2. **Per-decl verdict, not a single pass/fail.** Each side maps every
   declaration to one verdict from a fixed vocabulary:

   | verdict       | meaning |
   |---------------|---------|
   | `verified`    | the checker accepted it as a proof |
   | `rejected`    | the checker checked it and says it is **not** a proof (an alarm) |
   | `unsupported` | the checker does not implement a feature it needs (named) |
   | `absent`      | the checker never produced a verdict for this decl |

   The two runner scripts normalize each checker's native output into a
   `.jsonl` stream of `{"name","verdict","detail"}` records. `diff_verdicts.py`
   joins the two streams by name.

3. **Three outcomes, all good (brief §5).** For each shared declaration the join
   classifies the pair:

   | class | pair | reading |
   |-------|------|---------|
   | **AGREE** | verified=verified, or rejected=rejected | the checkers concur |
   | **DISAGREE** | verified vs rejected | the **one true contradiction** — a *finding* |
   | **ONLY-ONE-SIDE** | one side `absent` | version/toolchain skew; not a contradiction |
   | **UNSUPPORTED-SKIPPED** | at least one side `unsupported` | one checker declines; declining ≠ contradicting |

   Only `verified`-vs-`rejected` is a real disagreement. A checker that *declines*
   to check a declaration (`unsupported`) has not contradicted the other; a
   declaration one toolchain simply does not contain (`absent`) is skew, not
   conflict. **`rejected` is never merged with `unsupported`** — the whole point
   of keeping the buckets separate is that a rejection is an alarm and an
   unsupported feature is a roadmap item.

4. **Disagreements are findings, listed in full.** `diff_verdicts.py` prints
   every `verified`-vs-`rejected` pair with both verdicts and both detail
   strings, and `--fail-on-disagree` makes the harness exit non-zero. We never
   hide, sample, or reclassify a disagreement. When it fires we go read the
   declaration and decide which kernel is right.

---

## Files

| file | role |
|------|------|
| `run_oxilean.sh` | drive `oxilean-verify --json-full` over an NDJSON file; emit normalized `.jsonl` verdicts |
| `run_lean4lean.sh` | drive `lean4lean --verbose --fresh` over a Lean module; map its native output into normalized `.jsonl` verdicts |
| `diff_verdicts.py` | join two verdict files by name; emit the agree/disagree table + summary (Python 3 stdlib only) |
| `test_harness.sh` | self-contained smoke test (mock adapter + oxilean-vs-oxilean); needs no lean4lean |
| `slices/make-slice.py` | cut a deterministic, self-contained N-declaration NDJSON slice from a corpus |
| `slices/init-2000.ndjson` | committed 2,000-declaration slice of Lean core `Init` (see below) |
| `RESULTS-2026-07-12.md` | the first differential comparison + throughput baseline |

### Typical run

```bash
# our side: verdicts over the committed 2,000-decl Init slice
LIMITS=corpus ./run_oxilean.sh slices/init-2000.ndjson /tmp/ox.jsonl

# their side: verdicts over Lean core Init.Prelude (built with lean4lean's toolchain)
LEAN4LEAN_DIR=~/work/lean4lean ./run_lean4lean.sh Init.Prelude /tmp/ll.jsonl

# join
python3 diff_verdicts.py \
    --a-name oxilean   --a /tmp/ox.jsonl \
    --b-name lean4lean --b /tmp/ll.jsonl \
    --out-md RESULTS.md --out-csv joined.csv --fail-on-disagree
```

---

## The 2,000-declaration slice

`slices/init-2000.ndjson` is a deterministic prefix of Lean core's
`Init.ndjson`, cut by `slices/make-slice.py` so that it ends exactly on the
2,000th declaration record. Because lean4export NDJSON is an append-only,
index-addressed graph in which a declaration only references indices emitted
before it, any prefix ending on a declaration record is a valid, self-contained
NDJSON file. It is committed (~8 MB) so the throughput bench and the
differential join are reproducible without the 330 MB `Init.ndjson`. Regenerate
it with:

```bash
python3 slices/make-slice.py ~/work/oxilean-corpus/Init.ndjson 2000 slices/init-2000.ndjson
```

---

## lean4lean status (version skew + a native crash)

- **Version skew.** lean4lean's `master` pins `leanprover/lean4:v4.29.0`; our
  corpus and reader are pinned to `v4.32.0-rc1`. There is no lean4lean branch or
  tag on `v4.32.0-rc1` (the newest are `arena-v4.27.0-rc1` / `v4.27.0-rc1`), so
  `master` (v4.29.0) is the nearest workable choice and is what we build. The
  skew means the two checkers' declaration *sets* only partially overlap; those
  differences land in **ONLY-ONE-SIDE**, not in DISAGREE. This is a documented
  caveat, not a blocker (brief §8.2).

- **A native crash in the default path.** With the v4.29.0 toolchain, lean4lean's
  default `replayFromImports` path **segfaults** (SIGSEGV) during its
  region-freeing cleanup: the fault is in `lean_dec_ref_cold` called from
  `lean4lean_replayFromImports` on a task-manager worker thread — a
  reference-count use-after-free in the region teardown. `run_lean4lean.sh`
  therefore defaults to **`--fresh`** (the `replayFromFresh` path), which does
  not free regions and runs cleanly to `checked N declarations`. Pass `FRESH=0`
  to reproduce the crash.

Both are recorded in full in `RESULTS-2026-07-12.md`.

---

*Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.*
