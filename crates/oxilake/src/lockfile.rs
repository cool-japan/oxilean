//! Oxilake lockfile (`oxilake.lock`) — records resolved deps and content hashes.
//!
//! The lockfile is serialized via oxicode, COOLJAPAN's pure-Rust binary
//! serialization library (the mandatory workspace replacement for bincode).
//! It uses `oxicode::encode_to_file` / `oxicode::decode_from_file` with the
//! standard configuration so the format is compact and self-describing.

use std::path::Path;

use oxicode::{Decode, Encode};

/// A resolved dependency recorded in the lockfile.
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct LockedDep {
    /// Package name.
    pub name: String,
    /// Resolved version string (e.g. `"0.1.3"`).
    pub version: String,
    /// SHA-256 content hash of the resolved source (lowercase hex string).
    pub content_hash: String,
}

/// The Oxilake lockfile contents.
///
/// Written to `<project_root>/oxilake.lock` after a successful build.
/// Serialized as a compact oxicode binary blob (not JSON or TOML).
#[derive(Debug, Clone, PartialEq, Encode, Decode)]
pub struct OxiLock {
    /// All resolved dependencies.
    pub resolved: Vec<LockedDep>,
    /// Unix timestamp of when this lock was last written (seconds since epoch).
    pub timestamp_secs: u64,
    /// Version of oxilake that wrote this file (e.g. `"0.1.3"`).
    pub oxilake_version: String,
}

/// Errors from lockfile operations.
#[derive(Debug)]
pub enum LockfileError {
    /// An I/O error occurred while reading or writing the lockfile.
    Io(std::io::Error),
    /// The lockfile bytes could not be decoded via oxicode.
    Decode(String),
}

impl std::fmt::Display for LockfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockfileError::Io(e) => write!(f, "lockfile I/O error: {e}"),
            LockfileError::Decode(s) => write!(f, "lockfile decode error: {s}"),
        }
    }
}

impl std::error::Error for LockfileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LockfileError::Io(e) => Some(e),
            LockfileError::Decode(_) => None,
        }
    }
}

impl OxiLock {
    /// Create a minimal lockfile with no resolved deps.
    ///
    /// `oxilake_version` should be the current crate version string,
    /// e.g. `"0.1.3"`. Callers are responsible for setting `timestamp_secs`
    /// before writing if they need an accurate timestamp.
    pub fn empty(oxilake_version: &str) -> Self {
        Self {
            resolved: vec![],
            timestamp_secs: 0,
            oxilake_version: oxilake_version.to_string(),
        }
    }

    /// Write this lockfile to `<project_root>/oxilake.lock`.
    ///
    /// Serializes via oxicode with the standard configuration. Overwrites any
    /// existing lockfile at that path.
    pub fn write_to_dir(&self, project_root: &Path) -> Result<(), LockfileError> {
        let path = project_root.join("oxilake.lock");
        oxicode::encode_to_file(self, &path).map_err(|e| {
            LockfileError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                e.to_string(),
            ))
        })
    }

    /// Read and decode a lockfile from `<project_root>/oxilake.lock`.
    ///
    /// Returns `LockfileError::Io` if the file is missing or unreadable, and
    /// `LockfileError::Decode` if the bytes are not a valid oxicode-encoded
    /// `OxiLock`.
    pub fn read_from_dir(project_root: &Path) -> Result<Self, LockfileError> {
        let path = project_root.join("oxilake.lock");
        oxicode::decode_from_file(&path).map_err(|e| {
            // Distinguish I/O errors (file missing) from decode errors.
            let msg = e.to_string();
            if msg.contains("No such file") || msg.contains("os error") {
                LockfileError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, msg))
            } else {
                LockfileError::Decode(msg)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oxilock_roundtrip() {
        let mut lock = OxiLock::empty("0.1.3");
        lock.resolved.push(LockedDep {
            name: "oxilean-kernel".to_string(),
            version: "0.1.3".to_string(),
            content_hash: "abc123def456".to_string(),
        });
        lock.timestamp_secs = 1_748_476_800;

        let dir = std::env::temp_dir().join("oxilake_test_lockfile_12345");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        lock.write_to_dir(&dir).expect("write lockfile");
        let loaded = OxiLock::read_from_dir(&dir).expect("read lockfile");

        assert_eq!(loaded.oxilake_version, "0.1.3");
        assert_eq!(loaded.timestamp_secs, 1_748_476_800);
        assert_eq!(loaded.resolved.len(), 1);
        assert_eq!(loaded.resolved[0].name, "oxilean-kernel");
        assert_eq!(loaded.resolved[0].version, "0.1.3");
        assert_eq!(loaded.resolved[0].content_hash, "abc123def456");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_oxilock_empty() {
        let lock = OxiLock::empty("0.1.3");
        assert_eq!(lock.oxilake_version, "0.1.3");
        assert_eq!(lock.timestamp_secs, 0);
        assert!(lock.resolved.is_empty());
    }

    #[test]
    fn test_oxilock_missing_file_errors() {
        let dir = std::env::temp_dir().join("oxilake_test_no_lockfile_99999");
        std::fs::remove_dir_all(&dir).ok();
        std::fs::create_dir_all(&dir).expect("create temp dir");

        let result = OxiLock::read_from_dir(&dir);
        assert!(result.is_err(), "reading missing lockfile should error");

        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn test_lockfile_error_display() {
        let io_err = LockfileError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "no such file",
        ));
        assert!(io_err.to_string().contains("I/O error"));

        let decode_err = LockfileError::Decode("bad bytes".to_string());
        assert!(decode_err.to_string().contains("decode error"));
    }

    #[test]
    fn test_oxilock_multiple_deps_roundtrip() {
        let lock = OxiLock {
            resolved: vec![
                LockedDep {
                    name: "pkg-a".to_string(),
                    version: "1.0.0".to_string(),
                    content_hash: "deadbeef".to_string(),
                },
                LockedDep {
                    name: "pkg-b".to_string(),
                    version: "2.3.4".to_string(),
                    content_hash: "cafebabe".to_string(),
                },
            ],
            timestamp_secs: 9_999_999,
            oxilake_version: "0.1.3".to_string(),
        };

        let dir = std::env::temp_dir().join("oxilake_test_multi_dep_54321");
        std::fs::create_dir_all(&dir).expect("create temp dir");

        lock.write_to_dir(&dir).expect("write lockfile");
        let loaded = OxiLock::read_from_dir(&dir).expect("read lockfile");

        assert_eq!(loaded, lock);

        std::fs::remove_dir_all(&dir).ok();
    }
}
