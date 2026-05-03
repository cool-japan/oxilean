# oxilean

[![Crates.io](https://img.shields.io/crates/v/oxilean.svg)](https://crates.io/crates/oxilean)
[![docs.rs](https://docs.rs/oxilean/badge.svg)](https://docs.rs/oxilean)
[![License](https://img.shields.io/crates/l/oxilean.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

**Unified facade crate for OxiLean** -- a Pure Rust interactive theorem prover implementing the Calculus of Inductive Constructions (CiC), inspired by Lean 4.

This crate re-exports all OxiLean library subcrates under a single API surface with feature-gated imports. Zero C/Fortran dependencies.

## Architecture

```text
┌──────────────────────────────────────────────────────────┐
│                     oxilean (facade)                     │
├──────────┬──────────┬──────────┬──────────┬──────────────┤
│  parse   │   elab   │   meta   │  codegen │   build-sys  │
├──────────┴──────────┴──────────┴──────────┴──────────────┤
│                      kernel (TCB)                        │
├──────────┬──────────┬──────────┬──────────┬──────────────┤
│ std-lib  │ runtime  │   lint   │   wasm   │  cli (bin)   │
└──────────┴──────────┴──────────┴──────────┴──────────────┘
```

| Crate | Role |
|-------|------|
| `oxilean-kernel` | Trusted computing base — type checker, expressions, declarations (zero external deps) |
| `oxilean-parse` | Lexer, parser, surface AST |
| `oxilean-meta` | Metavar WHNF, unification, tactics, type class synthesis, discrimination trees |
| `oxilean-elab` | Elaborator — surface syntax to kernel terms |
| `oxilean-std` | Standard library definitions (Nat, Bool, List, mathematics) |
| `oxilean-codegen` | LCNF compilation and optimization |
| `oxilean-runtime` | Memory management, closures, I/O, task scheduling |
| `oxilean-build` | Project compilation, incremental builds, dependency management |
| `oxilean-lint` | Static analysis and lint rules |
| `oxilean-wasm` | WebAssembly bindings |
| `oxilean-cli` | Command-line binary (not re-exported; use directly) |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
oxilean = "0.1.2"
```

Or with all library components:

```toml
[dependencies]
oxilean = { version = "0.1.2", features = ["full"] }
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `kernel` | **yes** | Trusted computing base for type checking |
| `parse` | **yes** | Concrete syntax → abstract syntax parser |
| `elab` | **yes** | Surface syntax → kernel terms elaborator |
| `meta` | **yes** | Metavar-aware WHNF, unification, type class synthesis, tactics |
| `codegen` | no | LCNF-based compilation and optimization |
| `runtime` | no | Runtime system with GC, closures, and bytecode interpretation |
| `std-lib` | no | Standard library (Nat, Bool, List, etc.) |
| `lint` | no | Static analysis and lint rules |
| `build-sys` | no | Build system with incremental compilation |
| `wasm` | no | WebAssembly bindings (excluded from `full`) |
| `full` | no | All components except `wasm` |

## Usage

### Default features (kernel + parse + elab + meta)

```rust
use oxilean::kernel::{Environment, TypeChecker, Expr, Name, Level, Declaration};
use oxilean::parse::{Parser, Lexer, Token};
use oxilean::elab::{ElabContext, elaborate_expr, elaborate_decl, TypeInferencer};
use oxilean::meta::{MetaContext, TacticState, InstanceSynthesizer, DiscrTree};
```

### Full feature set

```toml
# Cargo.toml
oxilean = { version = "0.1.2", features = ["full"] }
```

```rust
use oxilean::kernel;
use oxilean::parse;
use oxilean::elab;
use oxilean::meta;
use oxilean::codegen;
use oxilean::runtime;
use oxilean::std_lib;
use oxilean::lint;
use oxilean::build_sys;
```

### Selecting individual components

Include only what you need to minimize compile times:

```toml
# Cargo.toml -- kernel + standard library only
oxilean = { version = "0.1.2", default-features = false, features = ["kernel", "std-lib"] }
```

```rust
use oxilean::kernel::{Environment, Expr, Name};
use oxilean::std_lib;
```

### Working with the type checker

```rust
use oxilean::kernel::{Environment, TypeChecker};

fn check_definitions(env: &Environment) {
    let tc = TypeChecker::new(env);
    // Type-check expressions against the kernel's CiC rules
    // ...
}
```

### Parsing and elaboration pipeline

```rust
use oxilean::parse::Parser;
use oxilean::elab::ElabContext;

fn process_source(source: &str) {
    let parser = Parser::new(source);
    // Parse surface syntax, then elaborate into kernel terms
    // ...
}
```

## Module Reference

All modules are re-exported at the crate root via `pub use`:

| Module | Source Crate | Access |
|--------|-------------|--------|
| `oxilean::kernel` | `oxilean-kernel` | `use oxilean::kernel;` |
| `oxilean::parse` | `oxilean-parse` | `use oxilean::parse;` |
| `oxilean::elab` | `oxilean-elab` | `use oxilean::elab;` |
| `oxilean::meta` | `oxilean-meta` | `use oxilean::meta;` |
| `oxilean::codegen` | `oxilean-codegen` | `use oxilean::codegen;` |
| `oxilean::runtime` | `oxilean-runtime` | `use oxilean::runtime;` |
| `oxilean::std_lib` | `oxilean-std` | `use oxilean::std_lib;` |
| `oxilean::lint` | `oxilean-lint` | `use oxilean::lint;` |
| `oxilean::build_sys` | `oxilean-build` | `use oxilean::build_sys;` |
| `oxilean::wasm` | `oxilean-wasm` | `use oxilean::wasm;` |

## CLI Companion

The `oxilean-cli` crate provides a command-line binary for interactive use:

```sh
cargo install oxilean-cli
oxilean check MyProject.lean
```

`oxilean-cli` is a standalone binary and is **not** re-exported through this facade crate.

## Related Projects

OxiLean is part of the [COOLJAPAN](https://github.com/cool-japan) Pure Rust ecosystem:

- **OxiZ** — SMT solver (SAT/SMT reasoning)
- **Legalis-RS** — Legal technology library
- **SciRS2** — Scientific computing
- **OxiBLAS** — Pure Rust BLAS
- **OxiFFT** — Pure Rust FFT

## License

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
