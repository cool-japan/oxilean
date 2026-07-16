//! Convenience entry point for oxilean-build.
//!
//! This module exposes [`build_project`], a high-level function that external
//! callers (such as `oxilake`) can use without having to assemble the build
//! pipeline by hand.

use std::path::Path;

use crate::core_types::{BuildConfig, BuildSystemError};
use crate::executor::{BuildDag, BuildExecutor, BuildPlanBuilder, ExecutorConfig};
use crate::manifest::{Manifest, Version};

/// The output produced by a successful [`build_project`] invocation.
///
/// Carries the [`BuildReport`] from the executor together with the manifest
/// that was used to drive the build.  Callers can inspect `report` for
/// per-step timings, warning counts, and artifact paths.
///
/// [`BuildReport`]: crate::executor::BuildReport
#[derive(Clone, Debug)]
pub struct BuildProjectOutput {
    /// Summary report from the build executor (step counts, durations, errors).
    pub report: crate::executor::BuildReport,
    /// Name of the package that was built, as declared in the manifest.
    pub package_name: String,
    /// Version of the package that was built, as a `major.minor.patch` string.
    pub package_version: String,
}

/// Errors that can be returned by [`build_project`].
#[derive(Clone, Debug)]
pub enum BuildError {
    /// The manifest file could not be read from disk.
    ManifestReadError(String),
    /// The manifest content could not be parsed into a valid `Manifest`.
    ManifestParseError(String),
    /// The build executor encountered an error (e.g. cyclic dependency, step failure).
    ExecutorError(crate::executor::ExecutorError),
    /// A general build system error (e.g. invalid configuration).
    BuildSystemError(BuildSystemError),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::ManifestReadError(msg) => write!(f, "manifest read error: {}", msg),
            BuildError::ManifestParseError(msg) => write!(f, "manifest parse error: {}", msg),
            BuildError::ExecutorError(e) => write!(f, "executor error: {}", e),
            BuildError::BuildSystemError(e) => write!(f, "build system error: {}", e),
        }
    }
}

impl std::error::Error for BuildError {}

impl From<crate::executor::ExecutorError> for BuildError {
    fn from(e: crate::executor::ExecutorError) -> Self {
        BuildError::ExecutorError(e)
    }
}

impl From<BuildSystemError> for BuildError {
    fn from(e: BuildSystemError) -> Self {
        BuildError::BuildSystemError(e)
    }
}

/// Build an OxiLean project from the given manifest path.
///
/// This is the primary entry point for external callers (e.g., `oxilake`).
/// It reads the manifest at `manifest_path`, resolves the build graph from the
/// targets declared in the manifest, and runs the executor with the provided
/// `config`.
///
/// # Parameters
///
/// - `manifest_path`: Path to the project manifest file (typically `oxilean.toml`
///   or `lakefile.lean`).  The file must be readable; if it does not exist this
///   function returns [`BuildError::ManifestReadError`].
/// - `config`: High-level build configuration (profile, jobs, output directory,
///   etc.).  Use [`BuildConfig::default()`] for a sensible debug build or
///   [`BuildConfig::release()`] for an optimised build.
///
/// # Returns
///
/// On success, returns a [`BuildProjectOutput`] that wraps the executor's
/// [`BuildReport`] and package metadata.
///
/// # Errors
///
/// Returns [`BuildError`] if:
/// - the manifest file cannot be read (`ManifestReadError`);
/// - the manifest content is not a recognisable package description (`ManifestParseError`);
/// - the executor encounters a cyclic dependency, step failure, or invalid
///   configuration (`ExecutorError`).
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use oxilean_build::convenience::{build_project, BuildError};
/// use oxilean_build::core_types::BuildConfig;
///
/// let result = build_project(Path::new("oxilean.toml"), BuildConfig::default());
/// match result {
///     Ok(output) => println!("Build succeeded: {} steps", output.report.completed_steps),
///     Err(BuildError::ManifestReadError(msg)) => eprintln!("Cannot read manifest: {}", msg),
///     Err(e) => eprintln!("Build failed: {}", e),
/// }
/// ```
///
/// [`BuildReport`]: crate::executor::BuildReport
pub fn build_project(
    manifest_path: &Path,
    config: BuildConfig,
) -> Result<BuildProjectOutput, BuildError> {
    // ── Step 1: read the manifest ────────────────────────────────────────────
    let manifest_text = std::fs::read_to_string(manifest_path).map_err(|e| {
        BuildError::ManifestReadError(format!("{}: {}", manifest_path.display(), e))
    })?;

    // ── Step 2: parse the manifest into a Manifest struct ───────────────────
    let manifest = parse_manifest(&manifest_text, manifest_path)?;

    // ── Step 3: build the DAG from the manifest targets ─────────────────────
    let dag = build_dag_from_manifest(&manifest, &config);

    // ── Step 4: create the executor config from BuildConfig ─────────────────
    let executor_profile = match config.profile {
        crate::core_types::BuildProfileKind::Release => crate::manifest::BuildProfile::release(),
        crate::core_types::BuildProfileKind::Test => crate::manifest::BuildProfile::test(),
        _ => crate::manifest::BuildProfile::debug(),
    };
    let executor_config = ExecutorConfig {
        parallelism: config.jobs.max(1),
        fail_fast: false,
        profile: executor_profile,
        output_dir: config.out_dir.clone(),
        show_progress: config.verbose,
        verbose: config.verbose,
        step_timeout: None,
        package_version: manifest.version.clone(),
    };

    // ── Step 5: run the executor ─────────────────────────────────────────────
    let mut executor = BuildExecutor::new(dag, executor_config);
    let report = executor.execute()?;

    Ok(BuildProjectOutput {
        package_name: manifest.name.clone(),
        package_version: format!(
            "{}.{}.{}",
            manifest.version.major, manifest.version.minor, manifest.version.patch
        ),
        report,
    })
}

/// Parse a manifest from its TOML text.
///
/// Uses a lightweight TOML key extractor to find `name` and `version` fields
/// without introducing a TOML parser dependency.  If the manifest does not
/// contain these fields, sensible defaults are derived from the file path.
fn parse_manifest(text: &str, path: &Path) -> Result<Manifest, BuildError> {
    let name = extract_toml_string(text, "name").unwrap_or_else(|| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    });

    if name.is_empty() {
        return Err(BuildError::ManifestParseError(
            "manifest `name` field is empty".to_string(),
        ));
    }

    let version_str = extract_toml_string(text, "version").unwrap_or_else(|| "0.1.0".to_string());
    let version = Version::parse(&version_str).map_err(|e| {
        BuildError::ManifestParseError(format!("invalid version `{}`: {}", version_str, e))
    })?;

    let mut manifest = Manifest::new(&name, version);
    manifest.manifest_path = path.to_path_buf();

    // Add a library target if a standard source file exists.
    let package_root = path.parent().unwrap_or(Path::new("."));
    let lib_src = package_root.join("src").join("lib.lean");
    if lib_src.exists() || text.contains("[lib]") {
        manifest.add_target(crate::manifest::Target::library(
            &manifest.name.clone(),
            &lib_src,
        ));
    }

    Ok(manifest)
}

/// Build a [`BuildDag`] from the targets listed in the manifest.
///
/// Each manifest target becomes a three-step pipeline: parse → elaborate →
/// compile.  A final link step is appended that depends on all compile steps.
fn build_dag_from_manifest(manifest: &Manifest, _config: &BuildConfig) -> BuildDag {
    let mut builder = BuildPlanBuilder::new();

    for target in &manifest.targets {
        builder.add_module(&target.name, &target.src_path, &[]);
    }

    // If no targets were declared, insert a synthetic module so the executor
    // has at least one step and returns a coherent report.
    if manifest.targets.is_empty() {
        let package_root = manifest.manifest_path.parent().unwrap_or(Path::new("."));
        builder.add_module(
            &manifest.name,
            &package_root.join("src").join("Main.lean"),
            &[],
        );
    }

    builder.add_link_step(&manifest.name);
    builder.build()
}

/// Extract the value of a simple `key = "value"` entry from TOML text.
///
/// Returns `None` if the key is not present or if the value is not a
/// double-quoted string on the same line.
fn extract_toml_string(toml: &str, key: &str) -> Option<String> {
    for line in toml.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix(key) {
            let rest = rest.trim();
            if let Some(rest) = rest.strip_prefix('=') {
                let rest = rest.trim();
                if let Some(inner) = rest.strip_prefix('"') {
                    if let Some(value) = inner.split('"').next() {
                        return Some(value.to_string());
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `build_project` must return an error when the manifest path does not exist.
    #[test]
    fn test_build_project_missing_manifest() {
        let nonexistent =
            std::env::temp_dir().join("oxilean_test_nonexistent_convenience_12345.toml");
        let config = BuildConfig::default();
        let result = build_project(&nonexistent, config);
        assert!(
            result.is_err(),
            "build_project with missing manifest should fail"
        );
        match result.expect_err("should be an error") {
            BuildError::ManifestReadError(_) => {}
            other => panic!("expected ManifestReadError, got {:?}", other),
        }
    }

    /// `build_project` should succeed when given a minimal manifest in a temp dir.
    #[test]
    fn test_build_project_minimal_manifest() {
        let tmp = std::env::temp_dir().join("oxilean_convenience_test_minimal_9999");
        std::fs::create_dir_all(&tmp).expect("create tmp dir");
        let manifest_path = tmp.join("oxilean.toml");
        std::fs::write(
            &manifest_path,
            r#"
[package]
name = "test-pkg"
version = "0.1.0"
"#,
        )
        .expect("write manifest");

        let config = BuildConfig::default();
        let result = build_project(&manifest_path, config);

        // Clean up before asserting so we don't leave trash on disk.
        let _ = std::fs::remove_dir_all(&tmp);

        assert!(
            result.is_ok(),
            "build_project with valid manifest should succeed; got {:?}",
            result.err()
        );
        let output = result.expect("build should succeed");
        assert_eq!(output.package_name, "test-pkg");
        assert_eq!(output.package_version, "0.1.0");
        assert!(
            output.report.success,
            "build report should indicate success"
        );
    }

    #[test]
    fn test_extract_toml_string_found() {
        let toml = r#"
[package]
name = "my-pkg"
version = "1.2.3"
"#;
        assert_eq!(
            extract_toml_string(toml, "name"),
            Some("my-pkg".to_string())
        );
        assert_eq!(
            extract_toml_string(toml, "version"),
            Some("1.2.3".to_string())
        );
    }

    #[test]
    fn test_extract_toml_string_missing() {
        assert_eq!(extract_toml_string("", "name"), None);
    }

    #[test]
    fn test_build_error_display() {
        let e = BuildError::ManifestReadError("no such file".to_string());
        assert!(e.to_string().contains("manifest read error"));
        let e2 = BuildError::ManifestParseError("bad version".to_string());
        assert!(e2.to_string().contains("manifest parse error"));
    }
}
