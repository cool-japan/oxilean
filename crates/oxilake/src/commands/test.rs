//! `oxilake test` — run all checks for the OxiLean package described by `manifest_path`.
//!
//! Delegates to `oxilean_build::build_project` (with a `Test` build profile), then
//! inspects the resulting `BuildReport` for step-level failures and reports them.

use crate::manifest::OxilakeManifest;
use anyhow::{Context, Result};
use oxilean_build::convenience::BuildError;
use oxilean_build::core_types::{BuildConfig, BuildProfileKind};
use std::path::Path;

/// Run all checks (type-check + test steps) for the package at `manifest_path`.
///
/// Uses the `Test` build profile so the executor runs test-tagged steps.
/// All step failures are collected and reported; the command returns an error
/// if any step failed.
pub fn run(manifest_path: &Path) -> Result<()> {
    // ── Step 1: load the manifest ────────────────────────────────────────────
    let manifest = OxilakeManifest::load(manifest_path)
        .with_context(|| format!("loading manifest {}", manifest_path.display()))?;

    println!(
        "Testing '{}' v{} ...",
        manifest.package.name, manifest.package.version
    );

    // ── Step 2: build with Test profile ──────────────────────────────────────
    let config = BuildConfig {
        profile: BuildProfileKind::Test,
        ..BuildConfig::default()
    };

    let output = oxilean_build::build_project(manifest_path, config)
        .map_err(|e| anyhow::anyhow!("{}", format_build_error(&e)))?;

    // ── Step 3: report results ───────────────────────────────────────────────
    let total = output.report.total_steps;
    let completed = output.report.completed_steps;
    let failures = collect_failures(&output.report);

    println!(
        "  Steps: {completed}/{total} completed, {} failed",
        failures.len()
    );

    if failures.is_empty() {
        println!("  All checks passed.");
        Ok(())
    } else {
        for f in &failures {
            eprintln!("  FAILED: {f}");
        }
        Err(anyhow::anyhow!(
            "{} check(s) failed for '{}'",
            failures.len(),
            manifest.package.name
        ))
    }
}

/// Collect failure messages from all step results.
fn collect_failures(report: &oxilean_build::executor::BuildReport) -> Vec<String> {
    report
        .step_results
        .iter()
        .filter(|r| !r.success)
        .flat_map(|r| {
            if r.errors.is_empty() {
                // Fall back to stderr if no structured errors.
                if r.stderr.is_empty() {
                    vec![format!("step {:?} failed (no details)", r.step_id)]
                } else {
                    vec![r.stderr.clone()]
                }
            } else {
                r.errors.clone()
            }
        })
        .collect()
}

/// Format a `BuildError` into a human-readable string.
fn format_build_error(e: &BuildError) -> String {
    format!("oxilean-build: {e}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    fn write_manifest(dir: &Path, name: &str, version: &str) {
        fs::write(
            dir.join("oxilake.toml"),
            format!(
                "[package]\nname = \"{name}\"\nversion = \"{version}\"\nlean_version = \"0.1\"\n"
            ),
        )
        .expect("write oxilake.toml");
    }

    #[test]
    fn test_test_missing_manifest_errors() {
        let dir = env::temp_dir().join("oxilake_test_cmd_missing_4321");
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(&dir).expect("create dir");

        let result = run(&dir.join("oxilake.toml"));
        assert!(result.is_err(), "test with missing manifest should fail");

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_test_succeeds_with_valid_manifest() {
        let dir = env::temp_dir().join("oxilake_test_cmd_ok_5678");
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(&dir).expect("create dir");

        write_manifest(&dir, "test-pkg", "0.1.0");

        let result = run(&dir.join("oxilake.toml"));
        assert!(
            result.is_ok(),
            "test with valid manifest should succeed: {:?}",
            result.err()
        );

        fs::remove_dir_all(&dir).ok();
    }
}
