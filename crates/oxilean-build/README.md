# oxilean-build

[![Crates.io](https://img.shields.io/crates/v/oxilean-build.svg)](https://crates.io/crates/oxilean-build)
[![Docs.rs](https://docs.rs/oxilean-build/badge.svg)](https://docs.rs/oxilean-build)

> **Build System and Package Manager for OxiLean**

`oxilean-build` implements the OxiLean build system and package manager. It handles package manifest parsing, dependency resolution (using a PubGrub-style algorithm), incremental compilation with content fingerprinting, DAG-based parallel build scheduling, remote caching, and package registry integration.

The design is intentionally analogous to Cargo: OxiLean projects declare their dependencies in a manifest file, and `oxilean-build` resolves, fetches, and compiles them in the correct order.

26,070 SLOC -- fully implemented build system (199 source files, 854 tests passing).

Part of the [OxiLean](https://github.com/cool-japan/oxilean) project -- a Lean-compatible theorem prover implemented in pure Rust.

## Overview

### Module Reference

| Module | Description |
|--------|-------------|
| `manifest` | Package manifest parsing and metadata validation |
| `resolver` | PubGrub-style version constraint solver |
| `incremental` | Incremental compilation with content-based fingerprinting |
| `opt_incremental` | Optimised incremental strategies (e.g. per-declaration caching) |
| `executor` | DAG-based parallel build job scheduler |
| `registry` | Package registry client (fetch, publish, search) |
| `remote_cache` | Remote build cache integration |
| `cache_eviction` | LRU / size-based local cache eviction policies |
| `scripts` | Custom build scripts and pre/post-build hooks |
| `distributed` | Distributed build coordination |
| `analytics` | Build telemetry and performance analytics |

### Build Configuration

```rust
use oxilean_build::BuildConfig;

// Debug build (default)
let config = BuildConfig::default();

// Release build with 8 parallel jobs
let config = BuildConfig::release().with_jobs(8).verbose();
```

### Build Profiles

| Profile | Description |
|---------|-------------|
| `Debug` | Fast compilation, full debug information, no optimisation |
| `Release` | Optimised output, stripped debug info |
| `Test` | Like debug but with test harness enabled |
| `Bench` | Like release but with benchmarking harness |

### Incremental Compilation

Content-based fingerprinting means only changed declarations are recompiled:

```text
File A.oxilean (unchanged hash) -> skip
File B.oxilean (hash changed)   -> recompile B and all dependents
```

Fingerprints are stored in the build cache directory alongside compiled `.olean`-equivalent artefacts.

### Dependency Resolution

The resolver takes a set of version constraints from manifests and finds a consistent assignment:

```text
my-project:
  depends: oxilean-std >= 0.1.1, < 0.2.0
           my-lib      >= 1.2.3

Resolution -> oxilean-std@0.1.1, my-lib@1.5.0
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
oxilean-build = "0.1.2"
```

### Running a Build

```rust,ignore
use oxilean_build::{BuildConfig, executor::BuildExecutor};

let config = BuildConfig::release().with_jobs(4);
let executor = BuildExecutor::new(config);
executor.build("path/to/project")?;
```

### Resolving Dependencies

```rust,ignore
use oxilean_build::resolver::Resolver;

let mut resolver = Resolver::new();
resolver.add_constraint("oxilean-std", ">=0.1.1, <0.2.0");
let solution = resolver.solve()?;
```

## Dependencies

- `oxilean-kernel` -- kernel types for compiled artefact representation
- `oxilean-parse` -- surface AST for dependency extraction from source files

## Testing

```bash
cargo test -p oxilean-build
cargo test -p oxilean-build -- --nocapture
```

## License

Copyright COOLJAPAN OU (Team Kitasan). Apache-2.0 -- See [LICENSE](../../LICENSE) for details.
