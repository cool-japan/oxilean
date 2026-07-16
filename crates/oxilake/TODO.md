# oxilake TODO

> Status: v0.1.3 Ring 0 in progress
> Last updated: 2026-05-29

## v0.1.3 (Ring 0 — complete)

- [x] Crate scaffold: main.rs, manifest.rs, commands (new/build/check)
- [x] `oxilake.toml` manifest parsing (TOML + serde)
- [x] `oxilake new <name>` — scaffold package directory
- [x] `oxilake build` — invoke build pipeline (dry-run stub for Ring 0)
- [x] `oxilake check` — check without codegen (dry-run stub for Ring 0)

## Ring 1

- [x] Full oxilean-build executor wiring for `oxilake build` (done 2026-05-29)
  - **Goal:** Wire `oxilake build` to actually call `oxilean_build::build_project` (currently the dep is declared but never called).
  - **Design:** Import `oxilean_build::build_project` in the build command handler; construct a `BuildConfig` from the manifest and pass it to `build_project`; surface errors as `OxilakeError`.
  - **Files:** `src/commands/build.rs` (or equivalent), `Cargo.toml` (verify dep is wired)
  - **Prerequisites:** C1 (oxilean-build convenience API)
  - **Tests:** `oxilake build` on a minimal fixture project invokes the executor (use `std::env::temp_dir()` for fixture).
  - **Risk:** The `BuildConfig` shape may not match what the CLI provides — adapt or stub missing fields.
- [x] Dependency resolution (semver + local path + registry stub) (done 2026-05-29)
  - **Goal:** Real resolver in the oxilake crate: minimal pure-Rust semver (`Version`/`VersionReq` supporting `1.2.3`, `^`, `~`, `>=`, `*`), local `path = "../foo"` deps loaded from each dep's `oxilake.toml` with cycle detection and topological build order, and a `RegistrySource` trait with a `LocalDirRegistry` impl (network registry = explicit `Unsupported` stub). Wire into `oxilake build` (build deps in topo order before the root).
  - **Design:** New resolver module; extend manifest `[dependencies]` parsing additively; DFS topological sort with back-edge cycle detection.
  - **Files:** `crates/oxilake/src/` (resolver module, manifest dep table, commands/build.rs wiring).
  - **Tests (std::env::temp_dir()):** semver parse/compare/match matrix; 3-package path-dep graph → correct topo order; cycle detected and reported; missing dep errors cleanly.
  - **Risk:** manifest schema additive only.
- [x] `oxilake.lock` lockfile (oxicode-serialized) (done 2026-05-29)
  - **Goal:** Write a lockfile of resolved deps + content hashes after a successful build; read+validate on subsequent builds. Serialized via **oxicode** (NOT bincode/serde_json).
  - **Design:** `OxiLock { resolved: Vec<LockedDep>, timestamp: u64 }` where `LockedDep { name, version, content_hash }`. Serialize/deserialize via `oxicode`. Write to `<project_root>/oxilake.lock`.
  - **Files:** `src/lockfile.rs` (new), `src/commands/build.rs`
  - **Prerequisites:** oxicode in workspace deps
  - **Tests:** `OxiLock` round-trips through oxicode serialization/deserialization (use `std::env::temp_dir()`).
  - **Risk:** oxicode API — verify the encode/decode function names in ~/work/oxicode.
- [x] `oxilake test`, `oxilake run`, `oxilake fmt` subcommands (done 2026-05-29)
- [x] `oxilake check` real per-declaration type-check (done 2026-05-30)
  - **Goal:** Make `oxilake check` actually elaborate + kernel-check each `.lean` source file, reporting per-decl pass/fail (not the current "dry-run mode" stub). Model: `oxilean-cli/src/commands/functions.rs:26-65` (`check_source`).
  - **Files:** `crates/oxilake/Cargo.toml` (add oxilean-elab/kernel/std), `crates/oxilake/src/commands/check.rs`.
  - **Tests:** package with well-typed theorem passes; package with type error reported; empty package clean.
- [x] Workspace support (multi-package manifests) (2026-05-30)
- [x] Cache at `~/.oxilake/cache/` (2026-05-30)
