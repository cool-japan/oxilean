# oxilean-build — TODO

> Task list for the build system crate.
> Last updated: 2026-05-03

## ✅ Completed

**Status**: COMPLETE — ~26,070 SLOC implemented across 199 source files

### Build System Features
- [x] Multi-file compilation
- [x] Dependency resolution
- [x] Build orchestration
- [x] Incremental compilation
- [x] Parallel build support
- [x] Cache management
- [x] Build configuration

### Compilation Pipeline
- [x] Source file discovery
- [x] Dependency graph construction
- [x] Topological sort for build order
- [x] Cycle detection
- [x] Build artifact generation
- [x] Error aggregation and reporting

### Performance Features
- [x] Incremental builds (rebuild only changed files)
- [x] Build caching
- [x] Parallel compilation

---

## 🐛 Known Issues

None reported. All tests passing.

---

## ✅ Completed: Extended Build System

- [x] Distributed build support — `distributed.rs` (WorkerPool, Task distribution, BuildCluster)
- [x] Build analytics — `analytics.rs` (BuildTimings, AnalyticsReport, build performance tracking)
- [x] Remote caching — `remote_cache.rs` (RemoteCacheClient, LocalMirrorCache, CacheKey, 8 tests)
- [x] More aggressive incremental compilation — `opt_incremental.rs` (IncrementalGraph, FileFingerprint, ChangeBatch, 8 tests)

## v0.1.3 oxilake Integration

> Last updated: 2026-05-29

### Ring 0

- [x] Document and stabilize the `core_types` public API for oxilake integration
  - **Goal:** The new `oxilake` crate (package manager) needs a clean entry point into oxilean-build's executor; identify and stabilize the minimal public surface needed
  - **Design:** Read `src/core_types/` to find `BuildConfig`, `BuildGraph`, `BuildExecutor` types; add doc comments to any public item missing them; add a `pub fn build_project(manifest_path: &Path, config: BuildConfig) -> Result<BuildOutput, BuildError>` convenience function if one doesn't already exist
  - **Files:** `src/core_types/`, `src/executor/` (add convenience fn if needed)
  - **Prerequisites:** None
  - **Tests:** Existing tests must still pass; add one new unit test for the convenience function
  - **Risk:** The trait-heavy API may make it hard to add a simple `build_project()` — document the minimal call sequence instead if necessary

### Ring 1

- [x] Distributed build integration with oxilake workspace builds
- [x] oxilake.lock-aware cache invalidation
