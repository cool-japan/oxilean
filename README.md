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
- **1.2M+ lines of Rust** across 5,380 source files and 11 crates
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

| Crate | SLOC | Description |
|-------|-----:|-------------|
| `oxilean-kernel` | 113,179 | Trusted Computing Base -- type checking core (zero external deps) |
| `oxilean-meta` | 150,298 | Metavar-aware WHNF, unification, type class synthesis, tactics |
| `oxilean-parse` | 61,225 | Concrete syntax to abstract syntax parser |
| `oxilean-elab` | 91,008 | Surface syntax to kernel terms elaborator |
| `oxilean-cli` | 64,163 | Command-line interface with REPL |
| `oxilean-std` | 413,202 | Standard library (mathematics, logic, data structures) |
| `oxilean-codegen` | 240,840 | LCNF-based compilation and optimization code generator |
| `oxilean-runtime` | 31,115 | Memory management, closures, I/O, task scheduling |
| `oxilean-build` | 25,194 | Project compilation and dependency management |
| `oxilean-lint` | 17,061 | Static analysis and lint rules |
| `oxilean-wasm` | 381 | WebAssembly bindings (browser support) |
| **Total** | **1,221,710** | **11 crates, 5,380 Rust files** |

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
# All tests
cargo test --workspace

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

**v0.1.0** | Under active development
