//! Provenance pins — the fingerprints every report carries so a reviewer can
//! reproduce it exactly.
//!
//! Two classes of pin:
//!
//! * **Build-time constants** — the tool version, and the `lean4export` commit
//!   and Lean toolchain this checker's reader is pinned to (re-exported from
//!   `oxilean-export`). These are the same for every run of a given binary.
//! * **Per-file values** — the NDJSON `format_version` and the exact Lean
//!   githash/version *this particular file* was produced with, read from the
//!   file's `meta` record. These vary per input.
//!
//! Recording both is a deliberate requirement of the engineering brief (§2, §7):
//! *"Pin the exact lean4export commit and the exact Lean toolchain version, and
//! record both in every report."*

use oxilean_export::{Meta, LEAN4EXPORT_COMMIT, LEAN_TOOLCHAIN, NDJSON_FORMAT_VERSION};

/// The provenance pins recorded in every report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentPins {
    /// This tool's name (`"oxilean-verify"`).
    pub tool_name: &'static str,
    /// This tool's version (`CARGO_PKG_VERSION` of the verify crate).
    pub tool_version: &'static str,
    /// The `lean4export` git commit the reader targets.
    pub lean4export_commit: &'static str,
    /// The Lean toolchain version the reader targets.
    pub lean_toolchain: &'static str,
    /// The NDJSON format version the reader targets.
    pub reader_format_version: &'static str,
    /// The NDJSON `format` version string read from *this file's* header, if a
    /// header was seen (`None` when the file could not be read to its header,
    /// e.g. under fail-fast abort).
    pub file_format_version: Option<String>,
    /// The Lean toolchain version *this file* was produced with, from the
    /// header.
    pub file_lean_version: Option<String>,
    /// The Lean toolchain git hash *this file* was produced with, from the
    /// header.
    pub file_lean_githash: Option<String>,
}

/// The tool name recorded in every report.
pub const TOOL_NAME: &str = "oxilean-verify";

impl EnvironmentPins {
    /// Build pins from a file's parsed [`Meta`] header.
    #[must_use]
    pub fn from_meta(tool_version: &'static str, meta: &Meta) -> Self {
        Self {
            tool_name: TOOL_NAME,
            tool_version,
            lean4export_commit: LEAN4EXPORT_COMMIT,
            lean_toolchain: LEAN_TOOLCHAIN,
            reader_format_version: NDJSON_FORMAT_VERSION,
            file_format_version: Some(meta.format_version.clone()),
            file_lean_version: Some(meta.lean_version.clone()),
            file_lean_githash: Some(meta.lean_githash.clone()),
        }
    }

    /// Build pins when no file header was available (fail-fast abort). The
    /// build-time pins are still present; the per-file fields are `None`.
    #[must_use]
    pub fn without_meta(tool_version: &'static str) -> Self {
        Self {
            tool_name: TOOL_NAME,
            tool_version,
            lean4export_commit: LEAN4EXPORT_COMMIT,
            lean_toolchain: LEAN_TOOLCHAIN,
            reader_format_version: NDJSON_FORMAT_VERSION,
            file_format_version: None,
            file_lean_version: None,
            file_lean_githash: None,
        }
    }
}
