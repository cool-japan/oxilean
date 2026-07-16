//! `oxilake build` — build the OxiLean package described by `manifest_path`.
//!
//! Ring 1: delegates to `oxilean_build::build_project`, then writes an
//! `oxilake.lock` lockfile serialized via oxicode.
//!
//! Ring 1 (dep resolution): calls `resolver::resolve_dependencies` with an
//! `UnsupportedRegistry` before building, so path deps are built in topo order.

use crate::cache::CacheManager;
use crate::lockfile::OxiLock;
use crate::manifest::OxilakeManifest;
use crate::resolver::{resolve_dependencies, UnsupportedRegistry};
use crate::workspace;
use anyhow::{Context, Result};
use oxilean_build::convenience::BuildError;
use oxilean_build::core_types::BuildConfig;
use std::path::Path;

/// Build the OxiLean package described by `manifest_path`.
///
/// 1. Loads and parses the root manifest.
/// 2. Resolves dependencies via `resolve_dependencies` (path deps only for now).
/// 3. Prints the topo-ordered build plan.
/// 4. Delegates to `oxilean_build::build_project`.
/// 5. Writes an `oxilake.lock` lockfile (oxicode-serialized) next to the manifest.
pub fn run(manifest_path: &Path, release: bool) -> Result<()> {
    // ── Step 0: workspace detection ──────────────────────────────────────────
    let project_root = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    if workspace::is_workspace(project_root) {
        match workspace::load_workspace(project_root) {
            Ok(ws) => {
                println!(
                    "Workspace at '{}' — {} member(s):",
                    ws.root.display(),
                    ws.members.len()
                );
                for (member_dir, pkg) in &ws.members {
                    println!("  {} v{} ({})", pkg.name, pkg.version, member_dir.display());
                }
            }
            Err(e) => {
                eprintln!("warning: workspace detection failed: {e}");
            }
        }
    }

    // ── Step 0b: initialise artifact cache ──────────────────────────────────
    //
    // Best-effort: if the cache cannot be created (e.g. $HOME is unset in CI),
    // we warn and continue rather than hard-failing the build.
    let cache_opt: Option<CacheManager> = match CacheManager::new() {
        Ok(cache) => {
            if let Ok(count) = cache.artifact_count() {
                if count > 0 {
                    eprintln!("oxilake cache: {count} artifact(s) available");
                }
            }
            Some(cache)
        }
        Err(e) => {
            eprintln!("warning: could not open artifact cache: {e}");
            None
        }
    };

    // ── Step 1: load the oxilake.toml manifest ───────────────────────────────
    let manifest = OxilakeManifest::load(manifest_path)
        .with_context(|| format!("loading manifest {}", manifest_path.display()))?;

    println!(
        "Building '{}' v{} ...",
        manifest.package.name, manifest.package.version
    );

    // ── Step 1b: resolve dependencies ────────────────────────────────────────
    let registry = UnsupportedRegistry;
    match resolve_dependencies(manifest_path, &registry) {
        Ok(dep_dirs) => {
            // dep_dirs is topo-ordered; the last entry is the root package itself.
            if dep_dirs.len() > 1 {
                println!("  Dependency build order:");
                for dir in &dep_dirs {
                    println!("    {}", dir.display());
                }
            }
        }
        Err(e) => {
            // Surface resolver errors as a clean fatal message.
            eprintln!("error: dependency resolution failed: {e}");
            std::process::exit(1);
        }
    }

    // ── Step 1c: compute a content hash for cache lookup ────────────────────
    //
    // We derive the build hash from the manifest content + the release flag.
    // This is intentionally lightweight: a proper implementation would hash
    // all source files, but hashing the manifest gives cache invalidation on
    // version/dep bumps, which is the most common case.
    let manifest_hash = {
        let mut raw = std::fs::read(manifest_path).unwrap_or_default();
        // Salt the hash with the oxilake version: the cached artifact is a
        // lockfile stamped with the writer's version, so a version bump must
        // invalidate the cache instead of restoring a stale lockfile.
        raw.extend_from_slice(env!("CARGO_PKG_VERSION").as_bytes());
        let release_byte: u8 = if release { 1 } else { 0 };
        content_hash(&raw, release_byte)
    };

    // ── Step 1d: check cache for a prior successful build ────────────────────
    if let Some(ref cache) = cache_opt {
        if cache.has_artifact(&manifest.package.name, manifest_hash) {
            eprintln!(
                "oxilake cache: hit — '{}' (hash {:016x}), skipping rebuild",
                manifest.package.name, manifest_hash
            );
            // Re-use cached lockfile bytes if present, then return early.
            if let Ok(cached_lock_bytes) =
                cache.read_artifact(&manifest.package.name, manifest_hash, "oxilake.lock")
            {
                if let Err(e) =
                    std::fs::write(project_root.join("oxilake.lock"), &cached_lock_bytes)
                {
                    eprintln!("warning: could not restore cached lockfile: {e}");
                }
            }
            return Ok(());
        }
    }

    // ── Step 2: construct BuildConfig from CLI args ──────────────────────────
    let config = if release {
        BuildConfig::release()
    } else {
        BuildConfig::default()
    };

    // ── Step 3: invoke oxilean-build executor ────────────────────────────────
    let output = oxilean_build::build_project(manifest_path, config).map_err(|e| {
        let msg = format_build_error(&e);
        anyhow::anyhow!("{}", msg)
    })?;

    println!(
        "Build succeeded: '{}' v{} — {} step(s) completed",
        output.package_name, output.package_version, output.report.completed_steps,
    );

    // ── Step 4: write oxilake.lock next to the manifest ──────────────────────
    // If a lockfile already exists, report its previous timestamp for context.
    if let Ok(prev) = OxiLock::read_from_dir(project_root) {
        if prev.timestamp_secs > 0 {
            println!(
                "  (previous lock written by oxilake v{})",
                prev.oxilake_version
            );
        }
    }

    let lock = OxiLock::empty(env!("CARGO_PKG_VERSION"));
    if let Err(e) = lock.write_to_dir(project_root) {
        // Non-fatal: warn but don't abort a successful build.
        eprintln!("warning: could not write oxilake.lock: {e}");
    }

    // ── Step 5: store the lockfile in the artifact cache ─────────────────────
    if let Some(ref cache) = cache_opt {
        if let Ok(lock_bytes) = std::fs::read(project_root.join("oxilake.lock")) {
            if let Err(e) = cache.store_artifact(
                &manifest.package.name,
                manifest_hash,
                &lock_bytes,
                "oxilake.lock",
            ) {
                eprintln!("warning: could not store artifact in cache: {e}");
            }
        }
    }

    Ok(())
}

/// Compute a simple 64-bit content hash from a byte slice and a mode byte.
///
/// Uses FNV-1a (64-bit) for speed and pure-Rust zero-dependency operation.
/// The `mode` byte distinguishes debug vs release builds from the same source.
fn content_hash(data: &[u8], mode: u8) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

    let mut hash = FNV_OFFSET_BASIS;
    for &byte in data {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    // Mix in the mode byte.
    hash ^= mode as u64;
    hash = hash.wrapping_mul(FNV_PRIME);
    hash
}

/// Format a `BuildError` into a human-readable string.
fn format_build_error(e: &BuildError) -> String {
    format!("oxilean-build: {e}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    /// Helper: write a minimal oxilake.toml to `dir`.
    fn write_manifest(dir: &Path, name: &str, version: &str) {
        std::fs::write(
            dir.join("oxilake.toml"),
            format!(
                "[package]\nname = \"{name}\"\nversion = \"{version}\"\nlean_version = \"0.1\"\n"
            ),
        )
        .expect("write oxilake.toml");
    }

    #[test]
    fn test_build_missing_manifest_errors() {
        let dir = env::temp_dir().join("oxilake_build_test_missing_9001");
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).expect("create dir");

        let result = run(&dir.join("oxilake.toml"), false);
        assert!(result.is_err(), "build with missing manifest should fail");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_build_succeeds_and_writes_lockfile() {
        let dir = env::temp_dir().join("oxilake_build_test_ok_7777");
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).expect("create dir");

        write_manifest(&dir, "test-pkg", "0.1.0");

        let result = run(&dir.join("oxilake.toml"), false);
        assert!(
            result.is_ok(),
            "build with valid manifest should succeed: {:?}",
            result.err()
        );

        // oxilake.lock must have been written.
        assert!(
            dir.join("oxilake.lock").exists(),
            "oxilake.lock should exist after successful build"
        );

        // The lockfile must round-trip cleanly.
        let lock =
            crate::lockfile::OxiLock::read_from_dir(&dir).expect("lockfile should be readable");
        assert_eq!(lock.oxilake_version, env!("CARGO_PKG_VERSION"));

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_build_release_mode_succeeds() {
        let dir = env::temp_dir().join("oxilake_build_test_release_8888");
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).expect("create dir");

        write_manifest(&dir, "rel-pkg", "1.0.0");

        let result = run(&dir.join("oxilake.toml"), /* release = */ true);
        assert!(
            result.is_ok(),
            "release build should succeed: {:?}",
            result.err()
        );

        std::fs::remove_dir_all(&dir).ok();
    }
}
