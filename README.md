# OxiLean

> **Pure Rust Interactive Theorem Prover**
> An implementation of the Calculus of Inductive Constructions (CiC), inspired by Lean 4

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.89%2B-orange.svg)](https://www.rust-lang.org/)
[![Docs.rs](https://docs.rs/oxilean-kernel/badge.svg)](https://docs.rs/oxilean-kernel)
[![Crates.io](https://img.shields.io/crates/v/oxilean-kernel.svg)](https://crates.io/crates/oxilean-kernel)
[![npm](https://img.shields.io/npm/v/@cooljapan/oxilean)](https://www.npmjs.com/package/@cooljapan/oxilean)

## Overview

OxiLean is a **memory-safe, high-performance Interactive Theorem Prover (ITP)** written entirely in Rust with zero C/Fortran dependencies. Inspired by Lean 4, it brings formal verification to the Rust ecosystem with:

- **Zero-dependency kernel** -- Trusted Computing Base with zero external crate dependencies
- **1.36M+ lines of Rust** across 6,168 source files and 17 crates
- **33,733 tests passing** -- comprehensive test suite with zero warnings (746 ignored)
- **WASM support** -- Runs in the browser with no server required
- **Tactic framework** -- intro, apply, simp, ring, omega, and more. The decision procedures (omega, linarith, polyrith) rebuild a proof term from their certificate and have the kernel check it; a goal they cannot certify falls back to `sorry`
- **Interactive REPL** -- Theorem proving from the command line
- **oxilean-verify** -- Independent Lean 4 proof checker over lean4export files ("Kernel in a Tab")

## oxilean-verify — Independent Lean 4 Proof Checker

`oxilean-verify` is an **independent re-implementation of the Lean 4 kernel's
checking rules** that reads Lean's own export format (lean4export NDJSON) and
re-checks every declaration from scratch — no Lean binary, no olean loading,
no C++. Its Trusted Computing Base is exactly two crates, **`oxilean-kernel` +
`oxilean-export`**, both with **zero external runtime dependencies** and
**`#![forbid(unsafe_code)]`** (checked by `scripts/gate-zero-deps.sh`,
`gate-forbid-unsafe.sh` and `gate-allow-list.sh`, run locally as release gates).
The CLI crate `oxilean-verify` itself is in the same zero-dep, no-unsafe
regime — argument parsing and JSON reporting are hand-rolled.

### Three-bucket semantics (never a wrong verdict)

Every declaration gets exactly one verdict; nothing is skipped silently:

| Verdict | Meaning | Exit code |
|---|---|---|
| `verified` | checked against our independent implementation of the kernel rules and accepted | `0` (when nothing was rejected) |
| `unsupported` | a **named** missing feature or limit (e.g. a per-declaration resource limit); the checker declines rather than guess | `0` (counted + named in the report) |
| `rejected` | **the alarm**: we checked it and it is *not* a proof | `1` |
| — broken input file, bad flag, I/O error | "this file is broken" is a different conversation than "this proof is wrong" | `2` |

### Result on Lean core (`Init`) — re-measured 2026-11-11

Over **all 57,277 declarations** of Lean 4 core's `Init` export
(v4.32.0-rc1, 6,453,270 NDJSON lines, 345 MB), in one process, one pass,
`--limits corpus`, on the 0.1.4 development tree:

```
57277 verified · 0 unsupported · 0 rejected      (exit status 0)
```

| bucket | count | share |
|---|---:|---:|
| **verified** | 57,277 | 100% |
| **unsupported** (named buckets) | 0 | 0% |
| **rejected** | 0 | 0% |

Qualifiers that must travel with the number:

- **What it does not say: that every Lean file verifies.** On a
  28,832-declaration slice of Mathlib the same build reports
  **27,523 verified · 1,309 unsupported · 0 rejected**: four nested inductive
  types the kernel cannot un-nest yet, and the 1,305 declarations that depend
  on one of them. The checker still declines rather than guess.
- **The default limits are enough.** Both corpora get the same verdicts
  without `--limits corpus`. The heaviest declaration of `Init` uses
  40% of the default per-declaration budget.
- **What changed since 0.1.3.** Nested inductives are supported, and the
  per-declaration budget is charged for the work the kernel does, not for the
  size a shared term would have as a tree (see
  [What's New in v0.1.4](#whats-new-in-v014)). In 0.1.3, one
  nested-inductive type (`Lean.Syntax`) and its dependency cascade accounted
  for most of an unsupported bucket of 21,853.
- **What "0 rejected" means.** *Nothing in Lean core was mis-flagged, and the
  checker found no incorrect proof.* That it does reject what is wrong is
  measured separately: 59,710 proofs, each checked against the statement of
  another theorem, with Lean's own kernel as the reference (see
  [What's New in v0.1.4](#whats-new-in-v014)).

All four rows were measured on the same machine: Apple M3, 24 GB, macOS, one
verification thread. Load average was ≈4–5 during the first two runs, ≈3–4
during the third and ≈3–6 during the fourth.

| build | verified · unsupported · rejected | wall | peak RSS |
|---|---|---:|---:|
| 0.1.3 (crates.io) | 35,424 · 21,853 · 0 | 8 m 28 s | 3.5 GiB |
| 0.1.4 tree, 2026-11-04 (`86227a8`) | 47,119 · 10,158 · 0 | 33 m 05 s | 2.1 GiB |
| 0.1.4 tree, 2026-11-11, first performance pass (`0524489`) | 47,133 · 10,144 · 0 | 3 m 58 s | 2.5 GiB |
| 0.1.4 tree, 2026-11-11, second pass | 57,277 · 0 · 0 | **34 s** | 0.9 GiB |

The fourth row's wall time was measured on the build just before its last
change (the two stack limits described under
[Kernel in a Tab](#kernel-in-a-tab-wasm-165-kb-gzip)). The final build gives the
same report, declaration for declaration, and executes 0.3% more
instructions over the run (536.1 billion against 534.3); by the
time it was built the machine was busy with other work (load average
≈8–25), and its four runs took 36–52 s.

Most of the 33 minutes went into the keys of the kernel's caches, not into
checking. In a profile of a Mathlib replay, 55% of the run was hashing the
terms that key the reduction and def-eq caches — each hash walked its whole
term — and 27% was formatting terms as text to put a pair of them in a fixed
order. Terms now carry their hash (see
[What's New in v0.1.4](#whats-new-in-v014)). On a 1,016-declaration
export that every build verifies in full, 0.1.3 takes 3.01 s and 60 MB of peak
RSS; the 0.1.4 tree took 2.48 s and 35 MB before that change, 0.46 s and
30 MB after it, and takes 0.13 s and 16 MB after the second pass.

The second pass removed the 632 declarations that ran out of budget, and the
9,512 behind them. They were not hard. A proof term of a few thousand shared
nodes stands for millions of nodes as a tree, and the kernel walked the tree —
substituting, typing and comparing a shared subterm once per path to it — and
charged its budget for the tree as well. It now works on the term as it is
shared. The declarations the first pass already verified took 161 s of that
run and take 16 s of this one.

Peak RSS in the third row is higher than in the second although that build
needed less memory: RSS also counts pages the allocator keeps after they are
freed. Measured with a counting allocator, the peak live heap of the `Init`
run went from 2,723 MB to 2,458 MB across the first pass and to
995 MB across the second; what is live at the peak is now the
environment itself.

**Structural sharing (wave5, 2026-07-16; measured on the 0.1.3 kernel on a 14 GiB
Linux box).** The kernel `Expr` moved from a deep
`Box` tree to reference-counted, cached-header `Node` edges (`Box<Expr>` →
`Rc<Expr>` → a `Node` caching each subterm's `looseBVarRange` and rebuild-cost),
and the export reader now materialises the shared DAG **once** (Init's
908,550,041 tree nodes are 552,915 distinct exprs, a ~1,643× factor) instead of
re-expanding it per parent. Full-`Init` wall fell from **1 h 20 m 44 s to
11 m 28 s (≈7×, ~83 decls/s)** and peak RSS from **9.98 GiB + ~6.7 GiB of
swap-thrash to ≈6 GiB with no swap** — the verdicts are a strict superset of the
pre-sharing 35,223 (the +201 is exactly explained: with clones now O(1) refcount
bumps the deterministic per-declaration fuel stops over-charging shared
subterms, and the untouched-subtree substitution skip charges the *same* fuel a
rebuild would, so declarations that previously hit the resource limit now
verify — nothing regresses, `rejected` stays 0). Stage F (0.1.4) adds
`Name`/`Level` interning on top. It only affects memory and moved no verdict.
The lean4lean comparison below was re-measured on 2026-11-11. Full report of
the pre-sharing 0.1.3 run,
with the follower attribution table and the reproduction command:
[`docs/reports/2026-07-15-lean-core-init.md`](docs/reports/2026-07-15-lean-core-init.md).

### Kernel in a Tab (WASM, ~165 KB gzip)

The same checker compiles to a **~165 KB gzipped** WASM artifact
(`oxilean-verify-wasm`: kernel + export + verify + wasm-bindgen only):

- 0.1.4 tree, measured 2026-11-11: 168,775 B (`gzip -9`), 427,912 B raw.
  Built the way the demo is built (`wasm-pack --release`, then `wasm-opt -Oz`).
  It was 169,956 B before the performance work of that day and 161,375 B
  after its first pass.
- Committed demo build (`web/verify-demo/pkg/`, built 2026-11-12 for the
  0.1.4 release): 168,785 B (`gzip -9`), 427,914 B raw.
- 0.1.3, measured the same way: 147,336 B.

Run under node 26, this build replays all of Lean core `Init` and the Mathlib
slice with the verdicts of the native binary, declaration for declaration
(57,277 · 0 · 0 and 27,523 · 1,309 · 0), in 1.3× and 1.6× the native binary's
time measured beside it (54 s and 20 s, on a busy
machine).

A browser gives a module about a megabyte of stack, which the module can
neither enlarge nor measure, and the kernel is recursive. So this build sets
two limits the native tool does not have — 256 reductions nested inside one
another, terms 512 levels deep — and reports a declaration over either as
`unsupported`, by name, where it would otherwise overflow the stack. Neither
corpus has such a declaration; a proof that evaluates a structural recursion
more than about 250 steps deep, or a list literal of more than about 500
elements, is one. See
[`docs/VERIFY.md`](docs/VERIFY.md#a-small-stack-two-limits-the-command-line-tool-does-not-set).

The artifact runs entirely client-side: drop an `.ndjson` export on the page,
watch the verdicts stream in, and **0 bytes are uploaded**.

```bash
cd web/verify-demo
./build.sh            # optional: rebuild pkg/ (the committed pkg/ is the 0.1.4 build)
python3 -m http.server 8000
# open http://localhost:8000
```

No CDN, no framework, no server round-trips. The browser demo and the native
binary run the same `verify_stream` code.

### Differential testing vs lean4lean

`verify/differential/` runs oxilean-verify and
[lean4lean](https://github.com/digama0/lean4lean) over the same declarations
and joins per-declaration verdicts by name. On `Init.Prelude` the harness
first surfaced **20 real disagreements** (all oxilean false-rejections of
multi-constructor `*.noConfusion` — one systematic kernel bug, since fixed);
the post-fix re-run joins at **0 disagreements** across 1,987 declarations.
Caveats recorded in the harness README for those runs: lean4lean `master`
pinned Lean v4.29.0 vs our v4.32.0-rc1 corpus (declaration-set skew lands in
ONLY-ONE-SIDE, not DISAGREE), and lean4lean's default `replayFromImports` path
segfaulted on that toolchain, so the harness drives it with `--fresh`.

The differential was run again on 2026-11-11 against the 0.1.4 tree, after
each of the two performance passes, with lean4lean `779c51f` built on the
corpus's own toolchain (v4.32.0-rc1): **0 disagreements** across
1,976 joined names. 1,810 agree; the other 166 are
on one side only — 162 auxiliary constants the export does not carry, and the
four `Quot` constants, which lean4lean reports as one.

Throughput on the same `Init.Prelude` set, measured that day on one machine
(Apple M3) with that one toolchain. Wall clock is end to end, the median of
nine runs; start-up is the same command on a module with no declarations.

| checker | wall | start-up | checking |
|---|---:|---:|---:|
| Lean's own kernel (`leanchecker Init.Prelude`) | 254 ms | 200 ms | ~54 ms |
| lean4lean `779c51f` (`--fresh`; 1,975 constants) | 535 ms | 398 ms | ~137 ms |
| oxilean-verify, 0.1.4 tree before the performance work (1,814 declarations) | 598 ms | ~5 ms | ~593 ms |
| oxilean-verify, after the first performance pass | 208 ms | ~5 ms | ~203 ms |
| oxilean-verify, 0.1.4 tree | **159 ms** | ~5 ms | **~154 ms** |

The first four rows were measured together; the last was measured later the
same day with the other two checkers run again beside it (`leanchecker`
256 ms with 207 ms of start-up, lean4lean 553 ms with
417 ms). Net of start-up, oxilean-verify takes 1.1× lean4lean's time
on this set and 3.2× the C++ kernel's; before the performance work it was
4.3× and 11.0×. Its figure includes reading the 3.7 MB export. End to end it
is the first to finish, having no Lean runtime to load. That is inside the 5×
budget.

The numbers recorded during the 0.1.3 cycle, on another machine and before
structural sharing, were ~3,280 decls/s for lean4lean against ~376 (0.1.2) and
~398 (0.1.3) for oxilean-verify, about 8× slower:
[`RESULTS-2026-07-12.md`](verify/differential/RESULTS-2026-07-12.md).

### Pins

| what | pin |
|---|---|
| lean4export | `3de59f10bc4b4a0f2de698597aeb1246caa0df0a` |
| Lean | `v4.32.0-rc1` (githash `b4812ae53eea93439ad5dce5a5c26591c31cb697`) |
| NDJSON format | `3.1.0` |

`oxilean-verify --version` prints the pins. Full schema, exit codes, JSON
report format, and the unsupported-feature list: [`docs/VERIFY.md`](docs/VERIFY.md).

## Architecture

```
                         +-----------------------+
                         |     oxilean-wasm      |  Browser / JS bindings
                         +-----------+-----------+
                                     |
+----------------+  +----------------+  +----------------+  +----------------+
|  oxilean-cli   |  | oxilean-build  |  | oxilean-codegen|  |  oxilean-lint  |
|  (REPL, check) |  | (project mgmt) |  | (LCNF compile) |  | (static anal.) |
+-------+--------+  +-------+--------+  +-------+--------+  +-------+--------+
        |                    |                   |                    |
        +--------------------+-------------------+--------------------+
                             |
               +-------------+-------------+
               |        oxilean-std        |  Standard library
               +-------------+-------------+
                             |
               +-------------+-------------+
               |       oxilean-runtime     |  Memory, closures, I/O
               +-------------+-------------+
                             |
               +-------------+-------------+
               |        oxilean-elab       |  Elaborator
               +-------------+-------------+
                             |
               +-------------+-------------+
               |        oxilean-meta       |  Unification, tactics,
               |                           |  type class synthesis
               +-------------+-------------+
                             |
               +-------------+-------------+
               |       oxilean-parse       |  Lexer, parser, AST
               +-------------+-------------+
                             |
        +--------------------+--------------------+
        |          oxilean-kernel (TCB)           |
        |    Type checking core -- ZERO deps      |
        +-----------------------------------------+
```

**Layer summary**: kernel (TCB) -> meta -> parse -> elab -> cli / build / codegen / runtime / lint / std -> wasm

## Workspace Crates

| Crate | SLOC | Tests | Description |
|-------|-----:|------:|-------------|
| `oxilean-kernel` | 134,392 | 3,742 | Trusted Computing Base -- type checking core (zero external deps, `#![forbid(unsafe_code)]`) |
| `oxilean-export` | 5,022 | 99 | lean4export NDJSON v3.1.0 reader + replay (TCB, zero external deps) |
| `oxilean-verify` | 2,336 | 50 | Independent Lean 4 proof checker CLI (three-bucket verdicts) |
| `oxilean-verify-wasm` | 387 | 4 | "Kernel in a Tab" WASM bindings (~165 KB gzip) |
| `oxilean-meta` | 163,149 | 5,426 | Metavar-aware WHNF, unification, type class synthesis, tactics |
| `oxilean-parse` | 67,822 | 2,388 | Concrete syntax to abstract syntax parser |
| `oxilean-elab` | 103,291 | 3,511 | Surface syntax to kernel terms elaborator |
| `oxilean-cli` | 73,652 | 2,205 | Command-line interface with REPL |
| `oxilean-std` | 442,733 | 7,977 | Standard library (mathematics, logic, data structures) |
| `oxilean-codegen` | 248,329 | 4,755 | LCNF-based compilation and optimization code generator |
| `oxilean-runtime` | 35,716 | 1,187 | Memory management, closures, I/O, task scheduling |
| `oxilean-build` | 30,598 | 794 | Project compilation and dependency management |
| `oxilean-lint` | 22,179 | 428 | Static analysis and lint rules |
| `oxilean-doc` | 2,726 | 102 | Documentation generation |
| `oxilake` | 2,539 | 63 | Lake-style project/package tooling |
| `oxilean-wasm` | 1,793 | 59 | WebAssembly bindings (browser support) |
| `oxilean` (umbrella) | 22 | 0 | Facade crate re-exporting the workspace |
| workspace `tests/` | — | 1,689 | Root integration suites (incl. Mathlib compat harness) |
| **Total** | **1,360,584** | **34,479** | **17 crates, 6,168 Rust files** (33,733 run + 746 ignored) |

> Fun fact: The COCOMO cost estimate for this codebase is **$52M+**.

## Feature Status

### Type Theory

- Universe hierarchy: `Prop : Type 0 : Type 1 : ...`
- Dependent types: `Pi (x : A), B`
- Inductive types: `Nat`, `List`, `Eq`, etc.
- Proof irrelevance: two proofs of the same proposition are definitionally equal
- Universe polymorphism: definitions can be polymorphic over universe levels

### Tactics

`intro`, `apply`, `exact`, `simp`, `rfl`, `ring`, `omega`, `sorry`, `cases`, `induction`, `constructor`

### CLI

- **REPL mode**: interactive theorem proving shell
- **File checking**: `oxilean check <file>` for `.oxilean` and `.lean` files
- **REPL commands**: `:type`, `:check`, `:env`, `:clear`, `:help`, `:quit`

### Mathlib4 Compatibility

OxiLean includes a **syntax compatibility test suite** that parses real Lean 4 / Mathlib4 declarations after applying automated normalization. This measures how much of Mathlib4's surface syntax OxiLean can handle.

- **7,759 Mathlib4 source files** parsed across 280+ categories
- **181,890 declarations** tested -- **99.7% parse compatibility** (181,326 parsed OK)
- The 99.7% is a **parse** number (surface syntax accepted by the parser), **not** a verification number -- kernel-verification results live in the [oxilean-verify section](#oxilean-verify--independent-lean-4-proof-checker) above and are measured on lean4export corpora, not Mathlib source
- Categories span all 28+ top-level Mathlib directories: Algebra, Analysis, CategoryTheory, Combinatorics, Data, FieldTheory, Geometry, GroupTheory, LinearAlgebra, Logic, MeasureTheory, NumberTheory, Order, Probability, RingTheory, SetTheory, Topology, and more

**Track 1** (parser compat): Reads `.lean` files from a local Mathlib4 checkout, normalizes syntax (`=>` to `->`, Unicode shorthand, head binders to `forall`, 280+ Unicode operators, etc.), and parses with OxiLean. The normalization pipeline handles quantifier binders, set-builder notation, subscript indexing, proof replacement, and more.

**Track 2** (curated theorems): 320 hand-adapted Mathlib4 theorems verified through parse + elaboration + tactic execution.

To run these tests locally, create `.env.mathlib` in the project root:

```bash
# .env.mathlib
MATHLIB4_ROOT=/path/to/mathlib4/Mathlib
```

```bash
# Run mathlib compat tests (ignored by default in normal test runs)
cargo test --test mathlib_compat_test -- --ignored --nocapture
cargo test --test mathlib_theorems_test
```

### WASM / npm

Published as [`@cooljapan/oxilean`](https://www.npmjs.com/package/@cooljapan/oxilean) on npm.

```bash
npm install @cooljapan/oxilean
```

```typescript
import { WasmOxiLean, checkSource, getVersion } from '@cooljapan/oxilean';

const ox = new WasmOxiLean();
const result = ox.check('theorem foo : True := trivial');
console.log(result.success); // true
console.log(result.declarations); // [{ name: "decl_0", kind: "theorem", ty: "Prop" }]
ox.free();
```

Full API: `check`, `repl`, `completions`, `hoverInfo`, `format`, `sessionId`, `history`, `clearHistory`, `version`

Supports bundler (webpack/vite), web (browser), and Node.js targets. See [oxilean-wasm README](crates/oxilean-wasm/README.md) for details.

## What's New in v0.1.4

- **Lean core `Init` verifies in full** (2026-11-11, second performance pass):
  **57,277 verified · 0 unsupported · 0 rejected in 34 s**, where the
  first pass of that day left 10,144 unsupported and took 3 m 58 s. The
  Mathlib slice goes from 25,690 verified in 43 s to **27,523 in
  11 s**. Every declaration verified before is verified after;
  `rejected` stays 0 on both. Both corpora now get the same verdicts under the
  default limits.
  - **The budget is charged for work done.** A substitution that shared an
    untouched subterm was charged what rebuilding it would have cost — its
    size as a tree. A proof term of a few thousand shared nodes stands for
    millions of tree nodes, so 632 declarations of `Init` spent the corpus
    budget in a tenth of a second and were cut off. A unit is now one subterm
    a term builder reaches; every node the kernel allocates is still charged.
    Over `Init` a declaration takes a median of 2,254 units and at
    most 6,655,688; the budgets are unchanged (2^24 and 2^26).
  - **Terms are walked as they are shared.** Substitution, lifting,
    abstraction and universe instantiation rebuilt a shared subterm once per
    path to it; they now build it once per walk. The reduction cache kept a
    term only if it was small as a tree, and compared keys by walking them; it
    now keeps terms of any size. Type inference remembers what it has typed,
    as Lean's kernel does.
  - **`f a ≡ f b` from `a ≡ b`.** Two applications of one definition are
    compared argument by argument before the definition is unfolded — the
    first step of Lean's lazy delta reduction. Unfolding first does not end in
    reasonable time when `f` recurses on something stuck (`x % USize.size`).
  - **Memory.** Peak live heap on `Init` falls from 2,458 MB to
    995 MB, and on the Mathlib slice from 1,213 MB to 419 MB.
  - **Checked against Lean's own kernel on wrong proofs.** For every theorem
    of both corpora, its proof was checked against the statement of another
    theorem, by this kernel and by Lean's (`Environment.addDeclCore`). Of
    59,710 such mutants this kernel accepts 430 — statements that happen to be
    definitionally equal — and Lean's kernel accepts exactly those; it rejects
    59,118 and Lean's kernel rejects every one; for 162 it gives no verdict (a
    resource limit, or a dependency on a nested inductive) where Lean rejects.
    No disagreement on any verdict given.
  - **A declaration must be a closed term.** `check_constant_info` and the
    inductive-family entry points refuse a type, value or recursor rule that
    contains a free variable or a loose bound variable, as Lean's kernel does.
    No export can spell a free variable, so `oxilean-verify` was never
    exposed; a caller building declarations through the Rust API was (also in
    0.1.3: `fun (x : F) => FVar 1` was accepted as a proof of `F → F` and
    stored with the free variable in it).
  - **Two limits for a caller with a small stack**, both off by default:
    `fuel::set_reduction_nesting_limit` and `fuel::set_term_depth_limit` (the
    second needs the new `term-depth` feature to cover terms built during
    checking). What is over one is `unsupported`, by name — the new
    `resource limit exceeded: term depth` among them — never a verdict. The
    wasm build sets both; see [Kernel in a Tab](#kernel-in-a-tab-wasm-165-kb-gzip).
  - **Public API.** `expr::REBUILD_COST_MAX` (new): `rebuild_cost` saturates
    at 2^29 − 1, the word being shared with three flags (contains a free
    variable, a universe parameter, a universe metavariable).
    `oxilean_export::TERM_DEPTH_LIMIT` (new). The budget constants keep their
    values and change their unit.
- **Performance, first pass** (2026-11-11). Checking Lean core `Init` takes **3 m 58 s**
  where it took 31 m 51 s, and a 28,832-declaration slice of Mathlib **43 s**
  where it took 5 m 50 s. The Mathlib report is identical before and after; on
  `Init`, six more declarations verify and nothing else moves.
  - **Terms carry their hash.** `Name`, `Level` and every shared subterm store
    a hash of their structure, computed once, when they are built, from their
    children's. The reduction and def-eq caches are keyed by terms, and every
    lookup used to hash its key by walking it — as a tree, so a subterm shared
    by many parents was hashed once per path to it. That was 55% of a profiled
    Mathlib replay.
  - **Pairs of terms are ordered by hash.** The equivalence manager formatted
    both terms of every pair as text to put them in a fixed order: another 27%
    of the same profile.
  - **A local's type is dropped when its binder closes.** The reducer and the
    def-eq checker kept the type of every binder opened while checking a
    declaration — up to 16,745 of them for one declaration, none still in
    scope — and copied them all into the nested checker of each K-like or
    structure-eta step. This is what brings the six `Init` declarations under
    the per-declaration budget.
  - **Memory.** Peak live heap falls by a tenth — 2,723 MB to 2,458 MB on
    `Init`, 1,349 MB to 1,213 MB on the Mathlib slice — and the bytes
    allocated over a run by two thirds. `Expr` is 40 bytes where it was 64,
    and the wasm artifact is 5% smaller.
  - **Against other checkers**, on `Init.Prelude` with one toolchain on one
    machine: net of start-up, 1.5× lean4lean's time where it was 4.3×. See
    [Differential testing vs lean4lean](#differential-testing-vs-lean4lean).
  - **Public API change.** `Node::rc()` is replaced by `Node::as_ptr()` and
    `Node::ptr_eq()`: a `Node` no longer holds an `Rc<Expr>`, because the
    facts cached about a subterm (its loose-bound-variable range, its rebuild
    cost, and now its hash) moved from the edge into the shared allocation.
    What `Expr` writes to a `Hasher` also differs from 0.1.3; `Debug` output
    does not.
- **Three kernel fixes** (2026-11-11), found while checking an external report
  against Mathlib. All three are also in the published 0.1.3.
  - **A wrong accept.** K-like reduction checks its side condition in a nested
    checker. That checker opened binders with free-variable ids the surrounding
    checker had already handed out, so a bound variable could be identified with
    an outer local and the condition could pass when it should not. A theorem
    that Lean's kernel and `leanchecker` both reject was reported as verified.
    It is now rejected; the declaration is kept as the negative fixture
    `KGuardCollision`.
  - **A wrong reject on point-free quotient maps.** A head that reduced to a
    partial application lost its outer arguments, so `Quot.lift` (or a
    recursor) whose major premise is one of them looked stuck. Mathlib's
    `Quot.map` / `Quotient.map'` are written that way; `AddCon.addGroup`,
    `Con.group`, `FreeGroup.instGroup` and `Quot.congr_mk` were among the
    declarations rejected.
  - **A wrong reject on long reductions.** A head-reduction chain was cut off
    after 10,000 steps and the unfinished term compared as if it were a normal
    form, so a valid `decide` proof over a longer chain was rejected. A
    declaration that fails after the cap fired is now checked again with the
    cap lifted, under the per-declaration budget. A failure that follows any
    other truncated reduction is reported as unsupported (`resource limit
    exceeded: reduction depth or step limit`), never as rejected.
  - On a 28,832-declaration slice of Mathlib (`v4.32.0-rc1`; every declaration
    of the `Order`, `Data`, `Logic`, `Algebra`, `GroupTheory` and `Control`
    modules that `Mathlib.GroupTheory.Congruence.Defs` imports, with their
    dependencies), rejected goes from **27 to 0**. On Lean core `Init`, eight
    declarations move from unsupported to verified (47,119 → 47,127) and
    rejected stays 0.
- **Nested inductives**: a nested inductive (for example `Lean.Syntax`, nested
  through `Array`/`List`) is specialised into an auxiliary mutual family.
  Recursors are derived on that family and restored over the real container,
  and every exported recursor is re-verified by def-eq, never trusted. On Lean
  core `Init`, verified rises from **35,424 to 47,119 (82.3%)**, still with
  **0 rejected**.
- **Structural sharing, Stage F**: `Name`/`Level` Rc-newtype interning. It is
  memory-only and moved no verdict. Peak RSS on full `Init` falls from 3.5 GiB
  to 2.1 GiB (same machine; see the table above).
- **Hygiene**, as reported by `cargo formal check --layer hygiene`:
  - The kernel goes from 297 findings to 19, and the workspace from 346 to 68.
  - Three unsound paths reachable from safe code were closed:
    `oxilean-elab`'s `validate_locals`, and `oxilean-runtime`'s `ThunkVec::get`
    and `ArenaAllocator::alloc_bytes`.
  - The `# Safety` contract of `PoolAllocator::deallocate` was tightened.
  - `StackCalc::{add, sub, mul}` now return `Option<i64>` instead of panicking.
    This is a public signature change.
- **Dependencies**: OxiZ 0.3.4, plus current oxicode and oxiarc.
- **MSRV is now 1.89** (it was declared 1.70, which never built: the locked
  dependencies already required 1.89); the clippy suggestions it unlocks are applied.
- **`deny.toml`** with the COOLJAPAN banned-crate list at the workspace root;
  `cargo deny check bans` passes.
- **`oxilean --help`, `-h`, `--version` and `-V` work.** They were treated as
  file names (`Failed to read file --help`, exit 1); they now print the help or
  the version and exit 0 (`-v` stays `--verbose`). `oxilean version` no longer
  prints the stale `Kernel SLOC: ~2,600` line.
- **Demo rebuilt for 0.1.4**: the committed `web/verify-demo/pkg/` is the 0.1.4
  build (168,785 B gzip).

## What's New in v0.1.3

- **oxilean-verify** — independent Lean 4 proof checker: streaming three-bucket CLI (`verified` / `unsupported (named)` / `rejected` = alarm) over lean4export NDJSON v3.1.0; TCB = `oxilean-kernel` + `oxilean-export`, zero external runtime deps, `#![forbid(unsafe_code)]`
- **First full-corpus result** — Lean core `Init` (all 57,277 declarations): **35,424 verified · 21,853 unsupported (named) · 0 rejected**, exit 0 ([report](docs/reports/2026-07-15-lean-core-init.md))
- **Structural sharing (wave5)** — kernel `Expr` as reference-counted, cached-header `Node` edges + materialise-once export reader: full-`Init` wall **1 h 20 m → 11 m 28 s (≈7×)**, peak RSS **9.98 GiB + swap-thrash → ≈6 GiB, no swap**, verdicts a strict superset (0 rejected). Plus a per-declaration wall-clock deadline and a file-streaming CLI so a whole-corpus export (Mathlib, ~6 GB) verifies without loading it into RAM
- **Kernel in a Tab** — `oxilean-verify-wasm` at **144 KB gzip** + static browser demo (`web/verify-demo`), all checking client-side, 0 bytes uploaded
- **Differential harness vs lean4lean** — found 20 real disagreements (one systematic `noConfusion` kernel bug), fixed: post-fix re-run joins at **0 disagreements** ([results](verify/differential/RESULTS-2026-07-12.md))
- **Kernel soundness overhaul** — arbitrary-precision `BigNat` literals (u64-overflow hole closed), quotients re-checked against kernel-built canonical types, recursors re-derived (never trusted from the export), kernel-decided large elimination, complete universe-level def-eq
- **CI/TCB gates + fuzzing** — zero-deps / forbid-unsafe / allow-list gates, WASM export & size gates, three cargo-fuzz targets with a weekly CI workflow

## Quick Start

```bash
# Clone the repository
git clone https://github.com/cool-japan/oxilean
cd oxilean

# Build the project
cargo build --release

# Run the REPL
cargo run --bin oxilean

# Check a file
cargo run --bin oxilean -- check examples/hello.oxilean

# Run tests
cargo test --workspace
```

## Development

### Prerequisites

- Rust 1.89 or later
- No external (non-Rust) dependencies required

### Running Tests

```bash
# All tests (33,733 tests; 746 ignored)
cargo test --workspace

# With cargo-nextest (recommended; about 140 s on an Apple M3)
cargo nextest run --no-fail-fast

# Kernel tests only
cargo test -p oxilean-kernel

# With verbose output
cargo test --workspace -- --nocapture
```

### Code Policies

- **Pure Rust** -- zero C/Fortran dependencies
- **No unwrap()** -- all error handling is explicit
- **No warnings** -- `cargo clippy` must pass cleanly
- **No `unsafe`** in the kernel
- **Workspace-managed versions** -- version set once in root `Cargo.toml`
- `cargo fmt` mandatory before commits

## Documentation

- [TODO.md](TODO.md) -- Master task list
- [CHANGELOG.md](CHANGELOG.md) -- Version history and release notes
- [CONTRIBUTING.md](CONTRIBUTING.md) -- Contribution guidelines

## Contributing

Contributions are welcome! This project follows:

- **Commit discipline**: one logical change per commit
- **Testing**: all new functions must have tests
- **Documentation**: public APIs must be documented

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Sponsorship

OxiLean is developed and maintained by **COOLJAPAN OU (Team Kitasan)**.

If you find OxiLean useful, please consider sponsoring the project to support continued development of the Pure Rust ecosystem.

[![Sponsor](https://img.shields.io/badge/Sponsor-%E2%9D%A4-red?logo=github)](https://github.com/sponsors/cool-japan)

**[https://github.com/sponsors/cool-japan](https://github.com/sponsors/cool-japan)**

Your sponsorship helps us:
- Maintain and improve the COOLJAPAN ecosystem
- Keep the entire ecosystem (OxiBLAS, OxiFFT, SciRS2, etc.) 100% Pure Rust
- Provide long-term support and security updates

## License

Copyright (c) COOLJAPAN OU (Team Kitasan). Licensed under [Apache-2.0](LICENSE).

## Related Projects (COOLJAPAN Ecosystem)

- [OxiZ](https://github.com/cool-japan/oxiz) -- Pure Rust SMT solver
- [Legalis-RS](https://github.com/cool-japan/legalis-rs) -- Law-as-Code framework
- [SciRS2](https://github.com/cool-japan/scirs2) -- Scientific computing library
- [OxiBLAS](https://github.com/cool-japan/oxiblas) -- Pure Rust BLAS
- [OxiFFT](https://github.com/cool-japan/oxifft) -- Pure Rust FFT

## Acknowledgments

- Inspired by [Lean 4](https://github.com/leanprover/lean4)
- Built by [COOLJAPAN OU (Team Kitasan)](https://github.com/cool-japan)

---

**v0.1.4** (2026-11-12) | Under active development
