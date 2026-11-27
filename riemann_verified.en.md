# Independent verification of the Lean proof of RH and GRH (compact edition) with OxiLean

**Verification report** · OxiLean `oxilean-verify` 0.1.6 · run of 2026-11-27T06:06:19Z – 2026-11-27T06:41:57Z (UTC) ·
日本語版: [riemann_verified.md](riemann_verified.md)

> **Result.** OxiLean's independent kernel checked all **204,583** declarations of the export of the
> compact edition of the Lean proof of the Riemann Hypothesis and of the Generalized Riemann
> Hypothesis for Dirichlet L-functions (github.com/lsernau/grh-lean, commit `72fead10`):
> **204,583 verified · 0 unsupported · 0 rejected**. Its comparator mode confirmed that the five
> theorems prove exactly the statements of the challenge file, from Lean's three standard axioms
> `propext`, `Classical.choice` and `Quot.sound` only: **`compare verdict: ESTABLISHED`**.

This adds a seventh kernel — an independent implementation in Rust — to the six that Andrew
Sutherland reports accepting the same export (Lean's default and paranoid kernels, nanoda,
lean4lean, con-leche and con-ron).

---

## 1. Summary

| Item | Result |
|---|---|
| Declarations checked | **204,583 verified · 0 unsupported · 0 rejected** (exit status 0) |
| The five theorems | 5 / 5 are theorems whose statements are identical to the challenge's (type and universe parameters; compared structurally, binder names included) and whose proofs the kernel verified |
| The rest of the challenge | 52,172 of the 52,173 challenge records occur identically in the solution export; the one left over is `sorryAx`, excluded by design; 0 differ, 0 missing |
| Dependency closure of the five theorems | 204,579 declaration records; 0 not verified, 0 `unsafe`/`partial`, 0 undeclared constants |
| Axioms the theorems depend on | `Classical.choice`, `Quot.sound`, `propext` — exactly the permitted ones |
| Axiom records in the whole export | `propext`, `Quot.sound`, `Classical.choice` (no others) |
| Wall-clock time / peak memory | 35 min 38 s on one thread / 13.2 GiB |

## 2. What was verified

| Item | Value |
|---|---|
| Proof | https://github.com/lsernau/grh-lean, commit `72fead102f663d5c7a407964a13dc0cf9d977207`, directory `grh-compact/` (794 files, about 76,000 lines) |
| Lean, Mathlib | Lean v4.35.0 (githash `eb6cafb7290e8bd5f2937522d8d804f58f02bafc`), Mathlib `56a005855ea17e0b0958f73ec801ae140b0a1679` |
| Solution export | `compact-solution.ndjson`, 5,172,274,528 bytes, SHA-256 `ab7b4223b9bd748a1aa68bdb63afb6d6264121fc834aadba7ba3d3796c32115b` |
| Challenge export | `challenge.ndjson`, 354,827,875 bytes, SHA-256 `20eb99a04b40f6afe1b44931e7ee82b15208542517dc588ace9634acffc41694` |
| Origin of the exports | https://math.mit.edu/~drew/shortgrh/ — made by `lake comparator` (Lean v4.35.0); Sutherland reports that exports made independently on two machines are byte-identical. The SHA-256 values above were recomputed after download and equal the published ones |
| Format | lean4export NDJSON, format version 3.1.0 (meta record: exporter lean4export 3.1.0, Lean 4.35.0) |

The five theorems (`src/Main.lean`). `RiemannHypothesis` is Mathlib's own definition; the other four
statements are defined in `src/Statements.lean`, which imports Mathlib only:

```lean
theorem riemannHypothesis : RiemannHypothesis                -- Mathlib's RiemannHypothesis
theorem riemannZeta_eq_zero_re : RiemannZetaZerosOnLine
theorem LFunction_ne_zero_of_half_lt_re : DirichletLNonvanishingRightOfHalf
theorem LFunction_eq_zero_re : DirichletLZerosOnLine
theorem LFunction_eq_zero_re_of_isPrimitive : PrimitiveDirichletLZerosOnLine
```

An export contains the five theorems and every declaration they depend on, transitively — the
project's own lemmas, Mathlib, Lean core — as fully elaborated kernel terms. A checker that reads it
re-checks all of them; nothing is taken from compiled `.olean` files.

## 3. The checker

| Item | Value |
|---|---|
| Tool | `oxilean-verify` 0.1.6, from https://github.com/cool-japan/oxilean |
| Source | commit `69d282f80d64890794a1ffae4e22c83f215de5e7` ("Riemann 2") of the `0.1.6` branch. The binary was built from `fb67fd5c` plus the change to `crates/oxilean-kernel/src/inductive/derive.rs` that `69d282f8` commits (recorded at build time: `fb67fd5c29292bdff874e17131cd30323f180c71 + working-tree.diff (sha256 398c87d47da0c4a0d983c558f5a9b4031db7ec2f5bf0e209b0399343d1141068)`); the Rust sources of the two trees are identical, byte for byte (checked by comparing the diffs) |
| Binary | SHA-256 `52aeea38a2494b9f1c7a1f0f175aff95ead660a83bf7f4c76a21dd8b06bb3592` (release profile, see §4) |
| Trusted computing base | `oxilean-kernel` (type checker) + `oxilean-export` (NDJSON reader): zero external dependencies, `#![forbid(unsafe_code)]`; the CLI adds no dependency either |
| Independence | A separate implementation in Rust that shares no code with Lean's C++ kernel, lean4lean, nanoda, con-leche or con-ron. It reads the export only — no Lean binary, no `.olean` |

What it checks:

- **Every declaration** is replayed into an environment that starts **empty**: definitions,
  theorems and opaque constants are type-checked; inductive types are checked and their recursors
  **derived by the kernel** (an exported recursor is compared with the derived one, never trusted);
  the quotient primitives are compared with the kernel's own; axioms are recorded. Each
  declaration gets exactly one verdict — `verified`, `unsupported` (a named missing feature or
  resource limit; never a guess) or `rejected` (it is not a proof).
- **`--compare <challenge>`** then checks, over the declarations verified in the same run, what
  `lake comparator` checks besides the kernel replay:
  1. each **target** — a theorem the challenge proves with `sorry` (found automatically; here the
     five above) — is a *theorem* of the solution whose type and universe parameters are
     identical to the challenge's, and was verified;
  2. every **other challenge declaration** (what the statements depend on: `riemannZeta`,
     `DirichletCharacter.LFunction`, `Complex`, `Real`, … — the 52,167 records of Mathlib, Lean core
     and `Statements.lean` left when the five targets and `sorryAx` are set aside) occurs in the solution under the same name and is **structurally identical**: kind,
     universe parameters, type, value, reducibility hints, inductive structure, binder names and
     binder annotations. Terms are compared exactly, with a structural equality that visits each
     pair of shared subterms once — never by hash alone — which is stricter than Lean's `==`;
  3. the **dependency closure** of the targets in the solution (through types, values,
     constructors and recursor rules) contains only permitted axioms, only verified declarations,
     no constant the export does not declare, and no `unsafe` or `partial` declaration.

## 4. Execution environment

| Item | Value |
|---|---|
| CPU | Intel Xeon Gold 5315Y @ 3.20 GHz, 8 vCPUs (Xen HVM guest), 1 thread per core |
| Memory | 44 GiB, no swap; `vm.overcommit_memory = 1` |
| OS | Ubuntu 22.04.3 LTS in a Docker container; Linux 6.8.0-138-generic x86_64 |
| Storage | inputs on the container's overlay filesystem; outputs on a network filesystem |
| Rust | rustc 1.100.0 (0ef1bc386 2026-11-09); the crate's MSRV is 1.89 |
| Build | `cargo build --release -p oxilean-verify`: `lto = true`, `codegen-units = 1`, `opt-level = 3` (workspace profile), with debug information kept (`CARGO_PROFILE_RELEASE_DEBUG=1`, `CARGO_PROFILE_RELEASE_STRIP=false`), which does not change the code generated |
| Threads | the check is single-threaded (one verification thread) |
| Other software | Python 3.11 with the `zstandard` package, only to decompress the downloaded `.zst` files |
| Concurrency | a second run (the default-limits run of §6.3) ran on the same machine at the same time |

## 5. Configuration

```bash
oxilean-verify --limits corpus --decl-fuel 2^30 --decl-timeout 1800 --stack-size 16384 \
  --no-color --json full.json --json-full \
  --compare challenge.ndjson compact-solution.ndjson > full.out
```

| Option | Meaning |
|---|---|
| `--limits corpus` | the preset for trusted whole-corpus exports (large reader budget) |
| `--decl-fuel 2^30` | per-declaration resource budget, in units of kernel work: 16 × the preset's 2^26 |
| `--decl-timeout 1800` | per-declaration wall-clock deadline: 30 minutes |
| `--stack-size 16384` | 16 GiB of stack reserved for the verification thread (only pages touched use memory) |
| `--json full.json --json-full` | machine-readable report with every declaration's verdict and time |
| `--compare challenge.ndjson` | the comparator checks of §3 |

The budget, the deadline and the stack are resources, not part of the logic: a declaration that
exhausts one is reported as **unsupported**, by name — never verified, never rejected. In this run
none was exhausted (0 unsupported).

## 6. Results

### 6.1 Verdicts and the comparison (end of `full.out`, verbatim)

```
204583 verified · 0 unsupported · 0 rejected
compare: challenge /tmp/grh/exports/challenge.ndjson
         sha256 20eb99a04b40f6afe1b44931e7ee82b15208542517dc588ace9634acffc41694, 52173 declaration records
  ✓  target riemannHypothesis                        statement identical to the challenge; proof verified
  ✓  target riemannZeta_eq_zero_re                   statement identical to the challenge; proof verified
  ✓  target LFunction_ne_zero_of_half_lt_re          statement identical to the challenge; proof verified
  ✓  target LFunction_eq_zero_re                     statement identical to the challenge; proof verified
  ✓  target LFunction_eq_zero_re_of_isPrimitive      statement identical to the challenge; proof verified
  statement closure: 52172 of 52173 challenge records identical in the solution (0 differ, 0 missing; sorryAx excluded)
  target closure: 204579 declaration records; 0 not verified; 0 unsafe/partial; 0 constants undeclared
  axioms used by the targets: Classical.choice, Quot.sound, propext (permitted: propext, Classical.choice, Quot.sound)
  axiom records in the solution file: propext, Quot.sound, Classical.choice
compare verdict: ESTABLISHED — the solution proves the 5 challenge statement(s) from the permitted axioms
```

### 6.2 Time and memory

| Item | Value |
|---|---|
| Start / end (UTC) | 2026-11-27T06:06:19Z / 2026-11-27T06:41:57Z |
| Wall-clock time | **2,138.1 s (35 min 38 s)** |
| Kernel time (sum over declarations) | 1,957.3 s |
| Everything else (computing the input's SHA-256, reading and parsing 5.17 GB, the comparison, the report) | 180.8 s |
| Peak resident memory | **13.2 GiB** (13,795,100 KiB) |
| Time per declaration | median 0.61 ms; 90th percentile 9.92 ms; 99th 177.9 ms; 99.9th 704.0 ms; maximum 46.1 s |
| Declarations over 100 ms / 1 s / 10 s | 3,344 / 112 / 7 (of 204,583) |

Kernel time by kind of declaration:

| Kind | Declarations | Kernel time (s) |
|---|---:|---:|
| `thm` | 165,589 | 1,477.0 |
| `def` | 36,489 | 86.8 |
| `inductive` | 2,419 | 393.5 |
| `axiom` | 3 | 0.0 |
| `opaque` | 79 | 0.0 |
| `quot` | 4 | 0.0 |

The 20 slowest declarations:

| # | Time | Kind | Declaration |
|---:|---:|---|---|
| 1 | 46.1 s | `thm` | `Filling.eq_one_of_reduceAt_eq_one._proof_1_4` |
| 2 | 28.6 s | `thm` | `IsCutBessel.norm_floorForm_le._proof_1_35` |
| 3 | 26.9 s | `thm` | `SplitPlaces.card_trivialised_of_isHigmanProjective` |
| 4 | 25.3 s | `thm` | `exists_analyticFamily_positiveBall` |
| 5 | 21.6 s | `thm` | `exists_isAbelianScheme` |
| 6 | 17.9 s | `thm` | `Filling.amplitudeAbs_lt_top_and_cut_le._proof_1_52` |
| 7 | 12.8 s | `thm` | `Filling.energy_sub_cut_le._proof_1_12` |
| 8 | 9.4 s | `thm` | `Filling.IsNear.le_classSize_and_cellMass_le._proof_1_15` |
| 9 | 6.1 s | `thm` | `hodgeTateMap_galois_twist` |
| 10 | 5.2 s | `thm` | `IsSplitPrimeToDisc.exists_place_above_ne._proof_1_19` |
| 11 | 5.2 s | `thm` | `hasBandLimits_of_isCutBessel._proof_1_55` |
| 12 | 5.1 s | `thm` | `ellAdicCohom.isCrystalline_and_linearFrob_rel` |
| 13 | 4.6 s | `thm` | `Filling.IsFillingLevel.exists_intermediateGroup_lines` |
| 14 | 4.5 s | `thm` | `Data.hasGap_gridParam._proof_1_6` |
| 15 | 3.8 s | `thm` | `CutsOutLocus.IsNoetherNormalization.pullback_injective` |
| 16 | 3.8 s | `thm` | `CMLattice.translatedWpDeriv_add_mem_multiPhiRing._proof_1_15` |
| 17 | 3.5 s | `thm` | `exists_isPrimeTriangle` |
| 18 | 3.5 s | `thm` | `IsSplitPrimeToDisc.exists_place_above_ne._proof_1_20` |
| 19 | 2.8 s | `thm` | `denominatorLe_rowBudget` |
| 20 | 2.8 s | `thm` | `exists_torusModel` |

The five theorems themselves (their proofs are lemmas checked earlier in the file):

| Theorem | Time |
|---|---:|
| `riemannHypothesis` | 0.08 ms |
| `riemannZeta_eq_zero_re` | 0.05 ms |
| `LFunction_ne_zero_of_half_lt_re` | 0.00 ms |
| `LFunction_eq_zero_re` | 0.00 ms |
| `LFunction_eq_zero_re_of_isPrimitive` | 0.08 ms |

### 6.3 The same binary under the default limits

Run beside the main run with the default `--limits corpus` (budget 2^26, deadline 30 s, stack
512 MiB) and `--compare`: **204,425 verified · 158 unsupported · 0 rejected**, exit
status 3, compare verdict NOT ESTABLISHED, 32 min 12 s, peak 13.1 GiB. The
unsupported ones are 8 declarations over the default budget — `IsCutBessel.norm_floorForm_le._proof_1_35`, `Filling.amplitudeAbs_lt_top_and_cut_le._proof_1_52`, `Filling.eq_one_of_reduceAt_eq_one._proof_1_4`, `Filling.energy_sub_cut_le._proof_1_12`, `Filling.IsNear.le_classSize_and_cellMass_le._proof_1_15`, `IsSplitPrimeToDisc.exists_place_above_ne._proof_1_19`, `exists_analyticFamily_positiveBall`, `SplitPlaces.card_trivialised_of_isHigmanProjective` — and
150 that depend on them. All of them are verified in the main run. **No run rejected
anything.** The conclusion rests on the main run (§5); that the default budget is not enough for
every declaration of this export is a performance limitation, not a soundness one.

### 6.4 Output files

| File | Size (bytes) | SHA-256 |
|---|---:|---|
| `full.out` (one line per declaration, then the summary and the compare section) | 13,816,044 | `b82e5a89cc18ad850e813394d46e9b7214bacb031ef096bebdfac5182d9078fb` |
| `full.json` (`--json-full` report; the per-declaration times make it differ from run to run) | 33,259,025 | `567c783d1bd1a569602b8a688a2549820bd64290e943255be3b206e33a13bf1c` |

The verdict lines of `full.out` carry per-declaration times, so its hash also differs between runs;
what a reproduction should match is `grep -c '^✓' full.out` (204,583) and the summary and compare
lines at its end (§6.1).

## 7. Comparison with the other checkers (same solution export)

The other checkers' figures are from Andrew Sutherland's logs (https://math.mit.edu/~drew/shortgrh/,
"machine B": a 192-core AWS r8a.48xlarge with 1.5 TB of memory). The machines differ, so the times
are indicative.

| Checker | Implementation | Declarations accepted | Wall clock | CPU time | Peak memory |
|---|---|---:|---:|---:|---:|
| Lean default kernel (`leanchecker --from-export`) | C++ | all | 33 min 05 s | 1,980 s | 7.0 GiB |
| lean4lean | Lean | 204,580 | 28 min 39 s | 1,716 s | 6.9 GiB |
| con-leche | — | 204,583 | 1 min 53 s (parallel) | 1,743 s | 13.7 GiB |
| con-ron | — | 204,583 | 1 min 16 s (parallel) | 470 s | 7.5 GiB |
| **OxiLean `oxilean-verify`** | **Rust** | **204,583** | **35 min 38 s (1 thread)** | **≈ 2,138.1 s** | **13.2 GiB** |

lean4lean's 204,580 counts the four built-in `Quot` constants as one (Sutherland's note).

## 8. How to reproduce

### 8.1 Requirements

- Linux or macOS on x86-64 or arm64; **16 GiB of memory at the very least, 32 GiB recommended**
  (peak 13.2 GiB in this run); about 12 GB of free disk for the inputs.
- Rust 1.89 or newer (`rustup` from https://rustup.rs).
- A `zstd` decompressor (`zstd -d`), or Python 3 with the `zstandard` package.
- About 40 minutes for the check.

### 8.2 Get the inputs and check them

```bash
mkdir grh && cd grh
curl -LO https://math.mit.edu/~drew/shortgrh/challenge.ndjson.zst
curl -LO https://math.mit.edu/~drew/shortgrh/compact-solution.ndjson.zst
zstd -d challenge.ndjson.zst compact-solution.ndjson.zst     # or: python3 -c "import zstandard ..." (see below)
sha256sum challenge.ndjson compact-solution.ndjson
# 20eb99a04b40f6afe1b44931e7ee82b15208542517dc588ace9634acffc41694  challenge.ndjson
# ab7b4223b9bd748a1aa68bdb63afb6d6264121fc834aadba7ba3d3796c32115b  compact-solution.ndjson
```

Without `zstd`:

```bash
python3 -m venv venv && venv/bin/pip install zstandard
for f in challenge compact-solution; do
  venv/bin/python -c "import zstandard; zstandard.ZstdDecompressor().copy_stream(open('$f.ndjson.zst','rb'), open('$f.ndjson','wb'))"
done
```

### 8.3 Get the checker

From crates.io:

```bash
cargo install oxilean-verify --version 0.1.6 --locked
```

or from source:

```bash
git clone https://github.com/cool-japan/oxilean && cd oxilean
git checkout v0.1.6        # the 0.1.6 release
cargo build --release -p oxilean-verify
# the binary is target/release/oxilean-verify
```

`oxilean-verify --version` prints the version and the lean4export format it reads.

### 8.4 Run

```bash
oxilean-verify --limits corpus --decl-fuel 2^30 --decl-timeout 1800 --stack-size 16384 \
  --no-color --json full.json --json-full \
  --compare challenge.ndjson compact-solution.ndjson > full.out
echo "exit status: $?"
```

Progress can be followed with `grep -c '^✓' full.out` (the export has 204,583 declarations).

### 8.5 Expected result

- exit status **0**;
- `grep -c '^✓' full.out` prints **204583**, and `grep -c '^[⊘✗]' full.out` prints **0**;
- the end of `full.out` is the block of §6.1: the summary line
  `204583 verified · 0 unsupported · 0 rejected`, five `✓  target …` lines, and
  `compare verdict: ESTABLISHED — the solution proves the 5 challenge statement(s) from the permitted axioms`;
- in `full.json`: `totals = {verified: 204583, unsupported: 0, rejected: 0}` and
  `compare.established = true`.

Exit status 1 would mean a rejected declaration, 2 a broken input or a usage error, 3 that nothing
was rejected but the challenge statements were not established.

### 8.6 Variants

- **Default limits**: drop `--decl-fuel`, `--decl-timeout` and `--stack-size`. Expect §6.3:
  204,425 · 158 · 0 and exit status 3.
- **Without the comparison**: drop `--compare challenge.ndjson`; the summary line is the same.
- **Only the challenge**: `oxilean-verify --limits corpus challenge.ndjson` checks the 52,173
  declaration records of the challenge (the statements and everything they depend on) (about 2 minutes; 52,173 verified · 0 · 0 here).

### 8.7 Making the exports yourself (optional)

The check above starts from Sutherland's exports, identified by their SHA-256. To start from the
Lean sources instead, build the project with Lean v4.35.0 and export the five theorems, as his page
describes:

```bash
git clone https://github.com/lsernau/grh-lean && cd grh-lean
git checkout 72fead102f663d5c7a407964a13dc0cf9d977207
cd grh-compact
lake exe cache get
lake build                                  # about 35 minutes on 16 cores
lake env leanexport Main -- riemannHypothesis riemannZeta_eq_zero_re \
  LFunction_ne_zero_of_half_lt_re LFunction_eq_zero_re LFunction_eq_zero_re_of_isPrimitive \
  > compact-solution.ndjson
```

(`lake comparator` also writes the challenge export.) The result should be byte-identical to the
published file.

## 9. What changed in OxiLean to make this verification possible

The verification found five gaps in OxiLean 0.1.5. All are fixed in 0.1.6; none of the fixes can turn
an invalid proof into a verified one.

1. **Definitional equality follows Lean's algorithm (kernel).** The kernel used to bring both sides
   of every comparison to full weak head normal form, unfolding every definition at the head, before
   comparing anything. On this export that evaluated `2000000 • x` in ℝ as two million nested
   `Real.add`s: the stack overflowed, the process ran out of memory at 33 GB, or the budget ran out.
   It now does what Lean's `is_def_eq_core` does: weak head normal form *without* unfolding the head
   constant, proof irrelevance, lazy δ reduction (unfold the side whose head has the greater
   definitional height, one step at a time; compare the arguments of two applications of the same
   definition before unfolding it), and only then the structural comparison. Proof irrelevance
   compares the two propositions as inferred. Unfolding is a valid reduction step and a `true` from
   comparing arguments is conclusive by congruence, so the new order can only find equalities that
   hold; and when neither head unfolds any more, the structural comparison sees the same terms as
   before.
   `exists_data._proof_1_66`: 70 s with a budget of 2^32 and a 16 GiB stack → 72 ms;
   `g₂_g₃_eq_eisenstein`: 17.4 s with a budget of 2^28 → 207 ms (times from the run of §6; both are
   within the default limits).
2. **Inductive declarations close their telescopes with the sharing-aware abstraction (kernel).** The
   inductive module abstracted variables with a walk that visited a subterm once per path to it;
   the large, heavily shared field types of a Mathlib-style structure made it exponential. It now
   uses the kernel's sharing-aware abstraction, which computes the same term. `IsPrimeTriangle`:
   116.5 s → 0.87 s; all 2,419 inductive declarations: 582 s → 393 s of kernel time.
3. **Term rendering is bounded (kernel).** An error message that rendered a heavily shared proof term
   as the tree it stands for ran one declaration to 24 GB before it could be reported. Rendering now
   stops after 2,048 subterms.
4. **`--compare` (verify).** Checking that every declaration is well typed does not say *what* was
   proved or *from what*. The comparator checks of §3 are new in 0.1.6 (exit status 3 when they fail).
5. **A safe declaration may use no `unsafe` constant and no `partial` definition (kernel),** Lean's
   rule in `infer_constant`. This export contains no `unsafe` or `partial` declaration, so it does not
   affect this result; it closes a soundness gap for other inputs.

Also new: `--decl-fuel` and `--decl-timeout`, and a diagnostic kernel feature `profile-unfold` (off
by default) that counts what the kernel unfolds per declaration — the tool that located items 1 and 2.

Regression checks of the release tree: the 4,055 tests of `oxilean-kernel`, `oxilean-export` and
`oxilean-verify` (including three new lazy-δ tests, two of which fail with the old algorithm) and
the 33,953 tests of the whole workspace pass (779 ignored); `cargo clippy` reports no warning; the
challenge export verifies in full (52,173 · 0 · 0) on 0.1.5 and on the new kernel alike.

## 10. How the verification went

All times UTC, 2026-11-27, on the machine of §4.

| Time | Event |
|---|---|
| 01:37 | First runs with OxiLean 0.1.5 + `--compare`: the challenge export verifies in full, 52,173 · 0 · 0 in 110 s |
| 01:48–02:05 | The solution run reaches declaration 116,627, `g₂_g₃_eq_eisenstein`, with everything before it verified, and grows to 24 GB; stopped. Stack samples, taken on an export of that declaration and its dependencies alone, show the time is spent rendering a term into an error message (gap 3) |
| 02:11 | With rendering bounded the declaration is reported unsupported (over budget) instead; with a budget of 2^28 it verifies in 17 s |
| 02:12–02:53 | Runs with budgets of 2^28, 2^30 and 2^32: no rejection, but 77 declarations exceed 2^28 in the first 137,788 (`exists_data._proof_1_66` the first of them); at 2^30 that declaration overflows the 512 MiB stack; at 2^32, with a 16 GiB stack, it passes, and the run is killed at 33 GB, out of memory, at declaration 122,375 |
| 04:31 | Profiling (`profile-unfold`): three million unary steps of `nsmulRec`/`Real.add` — `2000000 • x` evaluated in full. Root cause: definitional equality reduced both sides to full weak head normal form first (gap 1); fixed to Lean's lazy δ reduction |
| 04:47–05:26 | Runs with the new kernel: under a budget of 2^30, 204,583 · 0 · 0 and `ESTABLISHED`; under the default limits, 9 declarations over budget |
| 05:53–06:05 | `IsPrimeTriangle` (116 s) profiled; the inductive module's abstraction found to visit shared subterms once per path (gap 2); fixed: under 1 s |
| 2026-11-27T06:06:19Z – 2026-11-27T06:41:57Z | The run reported here, on the 0.1.6 release tree |

## 11. Caveats

- What is established is that **the five statements as Lean elaborates them** are proved, by the
  proof terms of the export, from the three permitted axioms. Whether those statements say what the
  Riemann Hypothesis and the Generalized Riemann Hypothesis for Dirichlet L-functions are understood
  to say is a matter of reading `Statements.lean` and Mathlib's definitions of `riemannZeta`,
  `DirichletCharacter.LFunction` and `RiemannHypothesis` (Sutherland's page shows the five
  statements fully elaborated, with `pp.all`).
- The exports were made by Sutherland; here they were identified by their SHA-256 and not
  regenerated (§8.7 shows how).
- OxiLean's reader names lean4export 3.1.0 / Lean v4.32.0-rc1 as the version it was written
  against; the file is lean4export format 3.1.0 written by Lean v4.35.0. The format is the same. A
  kernel rule introduced after v4.32 would have shown up as unsupported or rejected declarations;
  there were none.
- This is a machine check. It says nothing about whether the argument has been read or understood by
  a mathematician.

## 12. References

- Proof: https://github.com/lsernau/grh-lean (commit `72fead10`)
- Exports, logs of the other checkers: Andrew Sutherland, https://math.mit.edu/~drew/shortgrh/
- Michael Rubinstein's checks and explanation: https://www.math.uwaterloo.ca/~mrubinst/alpoge_claude_grh/
- OxiLean: https://github.com/cool-japan/oxilean — `oxilean-verify` documentation: [docs/VERIFY.md](docs/VERIFY.md)
- Changes in 0.1.6: [CHANGELOG.md](CHANGELOG.md)
