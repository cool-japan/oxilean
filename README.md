# OxiLean

> **Pure Rust Interactive Theorem Prover**
> An implementation of the Calculus of Inductive Constructions (CiC), inspired by Lean 4

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Docs.rs](https://docs.rs/oxilean-kernel/badge.svg)](https://docs.rs/oxilean-kernel)
[![Crates.io](https://img.shields.io/crates/v/oxilean-kernel.svg)](https://crates.io/crates/oxilean-kernel)
[![npm](https://img.shields.io/npm/v/@cooljapan/oxilean)](https://www.npmjs.com/package/@cooljapan/oxilean)

## Overview

OxiLean is a **memory-safe, high-performance Interactive Theorem Prover (ITP)** written entirely in Rust with zero C/Fortran dependencies. Inspired by Lean 4, it brings formal verification to the Rust ecosystem with:

- **Zero-dependency kernel** -- Trusted Computing Base with zero external crate dependencies
- **1.35M+ lines of Rust** across 6,050 source files and 17 crates
- **33,238 tests passing** -- comprehensive test suite with zero warnings (746 ignored)
- **WASM support** -- Runs in the browser with no server required
- **Full tactic framework** -- intro, apply, simp, ring, omega, and more
- **Interactive REPL** -- Theorem proving from the command line
- **oxilean-verify** -- Independent Lean 4 proof checker over lean4export files ("Kernel in a Tab")

## oxilean-verify — Independent Lean 4 Proof Checker

`oxilean-verify` is an **independent re-implementation of the Lean 4 kernel's
checking rules** that reads Lean's own export format (lean4export NDJSON) and
re-checks every declaration from scratch — no Lean binary, no olean loading,
no C++. Its Trusted Computing Base is exactly two crates, **`oxilean-kernel` +
`oxilean-export`**, both with **zero external runtime dependencies** and
**`#![forbid(unsafe_code)]`** (enforced by CI gates on every push:
`scripts/gate-zero-deps.sh`, `gate-forbid-unsafe.sh`, `gate-allow-list.sh`).
The CLI crate `oxilean-verify` itself is in the same zero-dep, no-unsafe
regime — argument parsing and JSON reporting are hand-rolled.

### Three-bucket semantics (never a wrong verdict)

Every declaration gets exactly one verdict; nothing is skipped silently:

| Verdict | Meaning | Exit code |
|---|---|---|
| `verified` | checked against our independent implementation of the kernel rules and accepted | `0` (when nothing was rejected) |
| `unsupported` | a **named** missing feature (e.g. `nested inductives`); the checker declines rather than guess | `0` (counted + named in the report) |
| `rejected` | **the alarm**: we checked it and it is *not* a proof | `1` |
| — broken input file, bad flag, I/O error | "this file is broken" is a different conversation than "this proof is wrong" | `2` |

### Result on Lean core (`Init`) — 2026-07-15 (structural sharing: 2026-07-16)

Over **all 57,277 declarations** of Lean 4 core's `Init` export
(v4.32.0-rc1, 6,453,270 NDJSON lines, 345 MB), in one process, one pass:

```
35424 verified · 21853 unsupported · 0 rejected      (exit status 0)
```

| bucket | count | share |
|---|---:|---:|
| **verified** | 35,424 | 61.8% |
| **unsupported** (named buckets) | 21,853 | 38.2% |
| **rejected** | 0 | 0% |

Qualifiers that must travel with the number: the unsupported bucket is
dominated by a **single root cause** — one nested-inductive type
(`Lean.Syntax`) whose dependency cascade accounts for the overwhelming majority
of it; "0 rejected" means *nothing in Lean core was mis-flagged and the checker
found no incorrect proof* — it does **not** mean everything was verified,
because the checker refuses to guess.

**Structural sharing (wave5, 2026-07-16).** The kernel `Expr` moved from a deep
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
verify — nothing regresses, `rejected` stays 0). The lean4lean per-declaration
compute gap (below) predates this work and has not been re-measured on
`Init.Prelude`. Full report of the pre-sharing run with the follower attribution
table and reproduction command:
[`docs/reports/2026-07-15-lean-core-init.md`](docs/reports/2026-07-15-lean-core-init.md).

### Kernel in a Tab (WASM, 144 KB gzip)

The same checker compiles to a **144 KB gzipped** WASM artifact — 147,132 B
measured at 0.1.3 (`oxilean-verify-wasm`: kernel + export + verify +
wasm-bindgen only) — and runs
entirely client-side — drop an `.ndjson` export on the page, watch streaming
verdicts, **0 bytes uploaded**:

```bash
cd web/verify-demo
python3 -m http.server 8000
# open http://localhost:8000
```

No CDN, no framework, no server round-trips; the browser demo and the native
binary drive the identical `verify_stream` code path.

### Differential testing vs lean4lean

`verify/differential/` runs oxilean-verify and
[lean4lean](https://github.com/digama0/lean4lean) over the same declarations
and joins per-declaration verdicts by name. On `Init.Prelude` the harness
first surfaced **20 real disagreements** (all oxilean false-rejections of
multi-constructor `*.noConfusion` — one systematic kernel bug, since fixed);
the post-fix re-run joins at **0 disagreements** across 1,987 declarations.
Caveats recorded in the harness README: lean4lean `master` pins Lean v4.29.0
vs our v4.32.0-rc1 corpus (declaration-set skew lands in ONLY-ONE-SIDE, not
DISAGREE), and lean4lean's default `replayFromImports` path segfaults on that
toolchain, so the harness drives it with `--fresh`. Baseline throughput
measured there: lean4lean ~3,280 decls/s vs oxilean-verify ~398 decls/s on the
same `Init.Prelude` set (~8.2× slower — outside the 5× budget, tracked). See
[`verify/differential/RESULTS-2026-07-12.md`](verify/differential/RESULTS-2026-07-12.md).

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
| `oxilean-kernel` | 127,298 | 3,485 | Trusted Computing Base -- type checking core (zero external deps, `#![forbid(unsafe_code)]`) |
| `oxilean-export` | 4,100 | 77 | lean4export NDJSON v3.1.0 reader + replay (TCB, zero external deps) |
| `oxilean-verify` | 2,205 | 47 | Independent Lean 4 proof checker CLI (three-bucket verdicts) |
| `oxilean-verify-wasm` | 399 | 4 | "Kernel in a Tab" WASM bindings (144 KB gzip) |
| `oxilean-meta` | 162,760 | 5,410 | Metavar-aware WHNF, unification, type class synthesis, tactics |
| `oxilean-parse` | 67,245 | 2,341 | Concrete syntax to abstract syntax parser |
| `oxilean-elab` | 101,970 | 3,497 | Surface syntax to kernel terms elaborator |
| `oxilean-cli` | 72,892 | 2,191 | Command-line interface with REPL |
| `oxilean-std` | 440,384 | 7,934 | Standard library (mathematics, logic, data structures) |
| `oxilean-codegen` | 247,202 | 4,713 | LCNF-based compilation and optimization code generator |
| `oxilean-runtime` | 35,229 | 1,162 | Memory management, closures, I/O, task scheduling |
| `oxilean-build` | 30,782 | 783 | Project compilation and dependency management |
| `oxilean-lint` | 22,111 | 425 | Static analysis and lint rules |
| `oxilean-doc` | 2,685 | 97 | Documentation generation |
| `oxilake` | 2,543 | 63 | Lake-style project/package tooling |
| `oxilean-wasm` | 1,797 | 59 | WebAssembly bindings (browser support) |
| `oxilean` (umbrella) | 22 | 0 | Facade crate re-exporting the workspace |
| workspace `tests/` | — | 1,689 | Root integration suites (incl. Mathlib compat harness) |
| **Total** | **1,345,501** | **33,984** | **17 crates, 6,050 Rust files** (33,238 run + 746 ignored) |

> Fun fact: The COCOMO cost estimate for this codebase is **$47M+**.

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

- Rust 1.70 or later
- No external (non-Rust) dependencies required

### Running Tests

```bash
# All tests (33,238 tests, ~78s)
cargo test --workspace

# With cargo-nextest (recommended)
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

**v0.1.3** (2026-07-16) | Under active development
