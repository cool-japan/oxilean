//! `oxilake run` — build and execute the `Main` declaration of the OxiLean package.
//!
//! Ring 1: builds the project via `oxilean_build::build_project`, then reports
//! execution status.  Full runtime execution of `Main` is deferred until the
//! oxilean-runtime VM is wired.

use crate::manifest::OxilakeManifest;
use anyhow::{Context, Result};
use oxilean_build::convenience::BuildError;
use oxilean_build::core_types::BuildConfig;
use std::path::Path;

/// Build and attempt to run the `Main` declaration of the package at `manifest_path`.
///
/// Currently performs a full debug build and, if the build succeeds, reports that
/// execution is not yet supported (the runtime VM is not wired to oxilake).
/// This keeps the command surface stable while the runtime matures.
pub fn run(manifest_path: &Path) -> Result<()> {
    // ── Step 1: load the manifest ────────────────────────────────────────────
    let manifest = OxilakeManifest::load(manifest_path)
        .with_context(|| format!("loading manifest {}", manifest_path.display()))?;

    println!(
        "Running '{}' v{} ...",
        manifest.package.name, manifest.package.version
    );

    // ── Step 2: build ────────────────────────────────────────────────────────
    let config = BuildConfig::default();

    let output = oxilean_build::build_project(manifest_path, config)
        .map_err(|e| anyhow::anyhow!("{}", format_build_error(&e)))?;

    println!(
        "  Build succeeded: {} step(s) completed",
        output.report.completed_steps
    );

    // ── Step 3: attempt execution ────────────────────────────────────────────
    //
    // The oxilean-runtime VM is not yet connected to oxilake.  Look for a
    // `Main.lean` source file so we can give a targeted message.
    let project_root = manifest_path.parent().unwrap_or_else(|| Path::new("."));

    let main_lean = project_root.join("src").join("Main.lean");
    if main_lean.exists() {
        println!(
            "  Found src/Main.lean — execution not yet supported (oxilean-runtime not wired)."
        );
    } else {
        println!("  No src/Main.lean found; nothing to run.");
    }

    println!("  Use `oxilake build` to produce build artifacts.");
    Ok(())
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
    fn test_run_missing_manifest_errors() {
        let dir = env::temp_dir().join("oxilake_run_cmd_missing_9753");
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(&dir).expect("create dir");

        let result = run(&dir.join("oxilake.toml"));
        assert!(result.is_err(), "run with missing manifest should fail");

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_run_succeeds_with_valid_manifest() {
        let dir = env::temp_dir().join("oxilake_run_cmd_ok_2468");
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(&dir).expect("create dir");

        write_manifest(&dir, "hello-pkg", "0.1.0");

        let result = run(&dir.join("oxilake.toml"));
        assert!(
            result.is_ok(),
            "run with valid manifest should succeed: {:?}",
            result.err()
        );

        fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_run_with_main_lean() {
        let dir = env::temp_dir().join("oxilake_run_cmd_main_1357");
        fs::remove_dir_all(&dir).ok();
        fs::create_dir_all(dir.join("src")).expect("create src dir");

        write_manifest(&dir, "main-pkg", "0.1.0");
        fs::write(
            dir.join("src").join("Main.lean"),
            "def main : IO Unit := IO.println \"hello\"\n",
        )
        .expect("write Main.lean");

        let result = run(&dir.join("oxilake.toml"));
        assert!(
            result.is_ok(),
            "run with Main.lean should succeed: {:?}",
            result.err()
        );

        fs::remove_dir_all(&dir).ok();
    }
}
