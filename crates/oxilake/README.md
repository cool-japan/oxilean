# oxilake

Package manager for OxiLean projects.

## Purpose

`oxilake` is a Cargo-inspired package manager for `.lean`-style OxiLean
projects. It manages manifests, dependency resolution, and build orchestration
via `oxilean-build`.

## Subcommands

| Subcommand | Description |
|---|---|
| `oxilake new <name>` | Scaffold a new project with `oxilake.toml` + `Main.lean` |
| `oxilake build` | Resolve dependencies and build the project |
| `oxilake check` | Build without emitting code (type-check only) |
| `oxilake test` | Build and report per-declaration check status |
| `oxilake run` | Build then execute `Main` |
| `oxilake fmt` | Format all source files |

## Manifest format (`oxilake.toml`)

```toml
[package]
name    = "my-project"
version = "0.1.0"
lean-version = "4.32.0"

[dependencies]
my-lib = { path = "../my-lib" }
```

## Dependency resolution

- Semver-aware version resolution (pure Rust, no external crate)
- Local `path =` dependencies with cycle detection and topological build order
- Registry trait with `LocalDirRegistry` implementation; network registry is
  an explicit `Unsupported` variant (requires separate approval)
- Lockfile (`oxilake.lock`) serialized via `oxicode` for reproducibility

## Workspace support

Multi-package workspace manifests are supported. Each member is built and
tested independently, respecting the inter-package dependency graph.

## Crate details

```toml
[dependencies]
oxilake = "0.1.3"
```

- License: Apache-2.0
- Edition: 2021
- Rust version: 1.70+

Copyright 2026 COOLJAPAN OU (Team Kitasan). Apache-2.0.
