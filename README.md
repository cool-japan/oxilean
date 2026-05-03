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
- **1.35M+ lines of Rust** across 5,978 source files and 12 crates
- **33,091 tests passing** -- comprehensive test suite with zero warnings
- **WASM support** -- Runs in the browser with no server required
- **Full tactic framework** -- intro, apply, simp, ring, omega, and more
- **Interactive REPL** -- Theorem proving from the command line

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
| `oxilean-kernel` | 115,444 | 3,444 | Trusted Computing Base -- type checking core (zero external deps) |
| `oxilean-meta` | 152,716 | 5,479 | Metavar-aware WHNF, unification, type class synthesis, tactics |
| `oxilean-parse` | 62,293 | 2,390 | Concrete syntax to abstract syntax parser |
| `oxilean-elab` | 92,415 | 3,448 | Surface syntax to kernel terms elaborator |
| `oxilean-cli` | 64,848 | 2,185 | Command-line interface with REPL |
| `oxilean-std` | 416,133 | 7,977 | Standard library (mathematics, logic, data structures) |
| `oxilean-codegen` | 243,915 | 4,706 | LCNF-based compilation and optimization code generator |
| `oxilean-runtime` | 31,676 | 1,162 | Memory management, closures, I/O, task scheduling |
| `oxilean-build` | 26,070 | 854 | Project compilation and dependency management |
| `oxilean-lint` | 17,600 | 685 | Static analysis and lint rules |
| `oxilean-wasm` | 510 | 47 | WebAssembly bindings (browser support) |
| **Total** | **1,347,647** | **33,091** | **12 crates, 5,978 Rust files** |

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

## What's New in v0.1.2

- **Real SMT solving** — `SmtContext::check_sat` and `run_smt_tactic` now call `oxiz-solver 0.2.1` (OxiZ) directly, returning live `Sat`/`Unsat`/`Unknown` results
- **WASM bytecode interpreter** — `WasmModule::call_function` wires all 157 `WasmInstruction` variants with full structured control flow, branch instructions, and frame-stack call dispatch
- **Keccak256-correct EVM/Solidity selectors** — `EvmBackend::compute_selector` uses real keccak256 (via `tiny-keccak`); 4-byte ABI selectors now interoperate with real EVM chains
- **Real Gröbner basis reduction** — `GroebnerBasis::reduce` implements multivariate polynomial division (Cox–Little–O'Shea §2.3); `polyrith` tactic ideal membership testing is now meaningful
- **Function inliner** — `InliningPass::run_all` / `run_with_context` in `opt_copy_prop` implements a fixed-point inliner with variable-ID freshening and configurable cost threshold
- **+3,260 tests** — test suite grown from 29,831 to 33,091 passing tests

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
# All tests (33,091 tests, ~78s)
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

**v0.1.2** (2026-05-03) | Under active development
