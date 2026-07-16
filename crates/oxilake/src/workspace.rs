//! Workspace support for oxilake.
//!
//! A single `oxilake.toml` at the workspace root can declare multiple member
//! packages via a `[workspace]` section.  Each member still owns its own
//! `oxilake.toml` with a `[package]` section; the workspace manifest coordinates
//! them.
//!
//! # Workspace manifest format
//! ```toml
//! [workspace]
//! members = ["pkg_a", "pkg_b/sub"]
//! ```
//!
//! Member paths are relative to the workspace root directory.

use crate::manifest::PackageSection;
use serde::Deserialize;
use std::fmt;
use std::path::{Path, PathBuf};

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

/// Errors produced by workspace operations.
#[derive(Debug)]
pub enum WorkspaceError {
    /// An I/O error occurred while reading a file.
    Io(std::io::Error),
    /// A TOML parse error occurred.
    TomlParse(String),
    /// A referenced member path does not exist on disk.
    MissingMember { path: PathBuf },
    /// A member directory exists but its manifest is invalid.
    InvalidMember { path: PathBuf, reason: String },
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkspaceError::Io(e) => write!(f, "workspace I/O error: {e}"),
            WorkspaceError::TomlParse(s) => write!(f, "workspace TOML parse error: {s}"),
            WorkspaceError::MissingMember { path } => {
                write!(f, "workspace member not found: {}", path.display())
            }
            WorkspaceError::InvalidMember { path, reason } => {
                write!(
                    f,
                    "workspace member at '{}' has invalid manifest: {reason}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for WorkspaceError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WorkspaceError::Io(e) => Some(e),
            _ => None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Raw TOML deserialization helpers (internal)
// ─────────────────────────────────────────────────────────────────────────────

/// Raw, serde-driven representation of an `oxilake.toml` at the workspace root.
///
/// Both `workspace` and `package` sections are optional so we can detect
/// which kind of manifest this is (workspace-root, standalone package, or both).
#[derive(Debug, Deserialize)]
struct WorkspaceManifestRaw {
    workspace: Option<WorkspaceSection>,
    /// The root may also define its own package (a "root package + workspace" pattern).
    #[allow(dead_code)]
    package: Option<serde::de::IgnoredAny>,
}

/// The `[workspace]` table inside `oxilake.toml`.
#[derive(Debug, Deserialize)]
struct WorkspaceSection {
    /// Relative paths from the workspace root to each member package directory.
    members: Vec<String>,
}

/// Minimal raw structure for reading a member package's `oxilake.toml`.
#[derive(Debug, Deserialize)]
struct MemberManifestRaw {
    package: Option<PackageSection>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Public types
// ─────────────────────────────────────────────────────────────────────────────

/// A workspace — a collection of member packages coordinated by a root
/// `oxilake.toml` that carries a `[workspace]` section.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// Absolute root directory of the workspace.
    pub root: PathBuf,
    /// Resolved member packages: `(member_dir, package_manifest)`.
    ///
    /// `member_dir` is the absolute path to the member's directory (the directory
    /// that contains the member's own `oxilake.toml`). `package_manifest` is the
    /// parsed `[package]` section of that member's manifest.
    pub members: Vec<(PathBuf, PackageSection)>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Public API
// ─────────────────────────────────────────────────────────────────────────────

/// Detect whether a directory contains a workspace manifest.
///
/// Returns `true` if `dir/oxilake.toml` exists and contains a `[workspace]` section.
/// Returns `false` if the file is absent, unreadable, or contains no `[workspace]`
/// section (i.e. it is an ordinary standalone-package manifest).
pub fn is_workspace(dir: &Path) -> bool {
    let manifest_path = dir.join("oxilake.toml");
    let content = match std::fs::read_to_string(&manifest_path) {
        Ok(s) => s,
        Err(_) => return false,
    };
    match toml::from_str::<WorkspaceManifestRaw>(&content) {
        Ok(raw) => raw.workspace.is_some(),
        Err(_) => false,
    }
}

/// Load the workspace rooted at `dir`.
///
/// Reads `dir/oxilake.toml`, parses the `[workspace]` section, resolves each
/// `members` entry as a path relative to `dir`, reads each member's own
/// `oxilake.toml`, and returns the fully-resolved [`Workspace`].
///
/// # Errors
///
/// - [`WorkspaceError::Io`] — the root manifest or a member manifest could not
///   be read from disk.
/// - [`WorkspaceError::TomlParse`] — the root manifest is not valid TOML or is
///   missing a `[workspace]` section entirely.
/// - [`WorkspaceError::MissingMember`] — a member path does not exist on disk.
/// - [`WorkspaceError::InvalidMember`] — a member's `oxilake.toml` is present
///   but malformed or lacks a `[package]` section.
pub fn load_workspace(dir: &Path) -> Result<Workspace, WorkspaceError> {
    let manifest_path = dir.join("oxilake.toml");

    // ── Step 1: read and parse the root manifest ─────────────────────────────
    let content = std::fs::read_to_string(&manifest_path).map_err(WorkspaceError::Io)?;

    let raw: WorkspaceManifestRaw =
        toml::from_str(&content).map_err(|e| WorkspaceError::TomlParse(e.to_string()))?;

    let workspace_section = raw.workspace.ok_or_else(|| {
        WorkspaceError::TomlParse(format!(
            "'{}' does not contain a [workspace] section",
            manifest_path.display()
        ))
    })?;

    // ── Step 2: resolve each member ──────────────────────────────────────────
    let mut members: Vec<(PathBuf, PackageSection)> = Vec::new();

    for member_rel in &workspace_section.members {
        let member_dir = dir.join(member_rel);

        // The member directory itself must exist.
        if !member_dir.exists() {
            return Err(WorkspaceError::MissingMember { path: member_dir });
        }

        let member_manifest_path = member_dir.join("oxilake.toml");

        // The member's oxilake.toml must be present and readable.
        let member_content = std::fs::read_to_string(&member_manifest_path).map_err(|e| {
            WorkspaceError::InvalidMember {
                path: member_dir.clone(),
                reason: format!("could not read member manifest: {e}"),
            }
        })?;

        // Parse the member manifest; it must have a [package] section.
        let member_raw: MemberManifestRaw =
            toml::from_str(&member_content).map_err(|e| WorkspaceError::InvalidMember {
                path: member_dir.clone(),
                reason: format!("TOML parse error: {e}"),
            })?;

        let package = member_raw
            .package
            .ok_or_else(|| WorkspaceError::InvalidMember {
                path: member_dir.clone(),
                reason: "member manifest is missing a [package] section".to_string(),
            })?;

        members.push((member_dir, package));
    }

    Ok(Workspace {
        root: dir.to_path_buf(),
        members,
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    /// Write the workspace root manifest to `dir/oxilake.toml`.
    fn write_workspace_manifest(dir: &Path, members: &[&str]) {
        let member_list = members
            .iter()
            .map(|m| format!("\"{}\"", m))
            .collect::<Vec<_>>()
            .join(", ");
        fs::write(
            dir.join("oxilake.toml"),
            format!("[workspace]\nmembers = [{member_list}]\n"),
        )
        .expect("write workspace manifest");
    }

    /// Write a minimal member `oxilake.toml` to `dir/oxilake.toml`.
    fn write_member_manifest(dir: &Path, name: &str, version: &str) {
        fs::create_dir_all(dir).expect("create member dir");
        fs::write(
            dir.join("oxilake.toml"),
            format!(
                "[package]\nname = \"{name}\"\nversion = \"{version}\"\nlean_version = \"0.1\"\n"
            ),
        )
        .expect("write member manifest");
    }

    // ── 1. is_workspace returns true for a workspace manifest ────────────────

    #[test]
    fn test_is_workspace_true() {
        let base = env::temp_dir().join("oxilake_ws_test_is_true_001");
        fs::remove_dir_all(&base).ok();
        fs::create_dir_all(&base).expect("create base");

        fs::write(base.join("oxilake.toml"), "[workspace]\nmembers = []\n")
            .expect("write manifest");

        assert!(is_workspace(&base), "should detect a workspace manifest");

        fs::remove_dir_all(&base).ok();
    }

    // ── 2. is_workspace returns false for a standalone package manifest ──────

    #[test]
    fn test_is_workspace_false() {
        let base = env::temp_dir().join("oxilake_ws_test_is_false_002");
        fs::remove_dir_all(&base).ok();
        fs::create_dir_all(&base).expect("create base");

        fs::write(
            base.join("oxilake.toml"),
            "[package]\nname = \"my-pkg\"\nversion = \"0.1.0\"\nlean_version = \"0.1\"\n",
        )
        .expect("write manifest");

        assert!(
            !is_workspace(&base),
            "standalone package should not be a workspace"
        );

        fs::remove_dir_all(&base).ok();
    }

    // ── 3. load_workspace with empty members list ────────────────────────────

    #[test]
    fn test_load_workspace_empty_members() {
        let base = env::temp_dir().join("oxilake_ws_test_empty_003");
        fs::remove_dir_all(&base).ok();
        fs::create_dir_all(&base).expect("create base");

        write_workspace_manifest(&base, &[]);

        let ws = load_workspace(&base).expect("load workspace with empty members");
        assert_eq!(ws.root, base);
        assert!(ws.members.is_empty());

        fs::remove_dir_all(&base).ok();
    }

    // ── 4. load_workspace with two members ───────────────────────────────────

    #[test]
    fn test_load_workspace_with_members() {
        let base = env::temp_dir().join("oxilake_ws_test_two_members_004");
        fs::remove_dir_all(&base).ok();
        fs::create_dir_all(&base).expect("create base");

        let pkg_a_dir = base.join("pkg_a");
        let pkg_b_dir = base.join("pkg_b");

        write_member_manifest(&pkg_a_dir, "pkg-a", "0.1.0");
        write_member_manifest(&pkg_b_dir, "pkg-b", "0.2.0");
        write_workspace_manifest(&base, &["pkg_a", "pkg_b"]);

        let ws = load_workspace(&base).expect("load workspace with two members");
        assert_eq!(ws.root, base);
        assert_eq!(ws.members.len(), 2);

        let (dir_a, pkg_a) = &ws.members[0];
        assert_eq!(dir_a, &pkg_a_dir);
        assert_eq!(pkg_a.name, "pkg-a");
        assert_eq!(pkg_a.version, "0.1.0");

        let (dir_b, pkg_b) = &ws.members[1];
        assert_eq!(dir_b, &pkg_b_dir);
        assert_eq!(pkg_b.name, "pkg-b");
        assert_eq!(pkg_b.version, "0.2.0");

        fs::remove_dir_all(&base).ok();
    }

    // ── 5. load_workspace with missing member path ───────────────────────────

    #[test]
    fn test_load_workspace_missing_member() {
        let base = env::temp_dir().join("oxilake_ws_test_missing_005");
        fs::remove_dir_all(&base).ok();
        fs::create_dir_all(&base).expect("create base");

        // Reference a member directory that does not exist.
        write_workspace_manifest(&base, &["nonexistent_member"]);

        let result = load_workspace(&base);
        assert!(result.is_err(), "missing member should cause an error");

        match result.unwrap_err() {
            WorkspaceError::MissingMember { path } => {
                assert!(
                    path.ends_with("nonexistent_member"),
                    "error should name the missing path, got: {}",
                    path.display()
                );
            }
            other => panic!("expected MissingMember, got: {other}"),
        }

        fs::remove_dir_all(&base).ok();
    }

    // ── 6. load_workspace with invalid member manifest ───────────────────────

    #[test]
    fn test_load_workspace_invalid_member_manifest() {
        let base = env::temp_dir().join("oxilake_ws_test_invalid_006");
        fs::remove_dir_all(&base).ok();
        fs::create_dir_all(&base).expect("create base");

        let bad_member = base.join("bad_member");
        fs::create_dir_all(&bad_member).expect("create bad member dir");
        // Write a malformed TOML manifest.
        fs::write(
            bad_member.join("oxilake.toml"),
            "this is not valid toml = = =\n",
        )
        .expect("write bad manifest");

        write_workspace_manifest(&base, &["bad_member"]);

        let result = load_workspace(&base);
        assert!(
            result.is_err(),
            "invalid member manifest should cause an error"
        );

        match result.unwrap_err() {
            WorkspaceError::InvalidMember { path, reason } => {
                assert!(
                    path.ends_with("bad_member"),
                    "error path should point at bad_member, got: {}",
                    path.display()
                );
                assert!(!reason.is_empty(), "reason should be non-empty");
            }
            other => panic!("expected InvalidMember, got: {other}"),
        }

        fs::remove_dir_all(&base).ok();
    }
}
