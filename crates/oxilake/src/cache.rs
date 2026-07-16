//! Build artifact cache for oxilake.
//!
//! Stores compiled artifacts in `~/.oxilake/cache/<package-name>/<hash>/`.
//! Each artifact directory contains one or more files produced during a build
//! step (e.g. compiled `.leanc` objects, type-checked outputs, etc.).
//!
//! The hash parameter is a `u64` content hash computed externally (e.g. from
//! the source content + compiler flags), formatted as a 16-character lowercase
//! hex string for the directory name.
//!
//! # Cache layout
//! ```text
//! ~/.oxilake/cache/
//!   <package-name>/
//!     <hash-hex>/          ← one dir per (package, content-hash) pair
//!       output.leanc
//!       ...
//! ```

use std::fmt;
use std::path::PathBuf;

// ─────────────────────────────────────────────────────────────────────────────
// Error type
// ─────────────────────────────────────────────────────────────────────────────

/// Errors produced by cache operations.
#[derive(Debug)]
pub enum CacheError {
    /// An I/O error occurred.
    Io(std::io::Error),
    /// The home directory could not be determined (neither `HOME` nor
    /// `USERPROFILE` is set in the environment).
    HomeNotFound,
    /// The supplied package name contains characters that are not allowed
    /// (e.g. `/`, `..`, or a leading `.`).
    InvalidPackageName(String),
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::Io(e) => write!(f, "cache I/O error: {e}"),
            CacheError::HomeNotFound => write!(
                f,
                "cache: could not locate home directory \
                 (neither HOME nor USERPROFILE is set)"
            ),
            CacheError::InvalidPackageName(name) => {
                write!(f, "cache: invalid package name '{name}'")
            }
        }
    }
}

impl std::error::Error for CacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CacheError::Io(e) => Some(e),
            _ => None,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Home directory resolution (pure Rust, no `dirs` crate)
// ─────────────────────────────────────────────────────────────────────────────

/// Resolve the current user's home directory from the environment.
///
/// Checks `HOME` first (Unix/macOS), then `USERPROFILE` (Windows).
fn home_dir() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("USERPROFILE").map(PathBuf::from))
}

// ─────────────────────────────────────────────────────────────────────────────
// Package name validation
// ─────────────────────────────────────────────────────────────────────────────

/// Validate a package name for use as a filesystem path component.
///
/// Rejects names that:
/// - are empty
/// - start with `.`
/// - contain `/`
/// - contain `..` as a path segment (also caught by the `/` check in practice,
///   but checked explicitly for clarity)
fn validate_package_name(name: &str) -> Result<(), CacheError> {
    if name.is_empty() {
        return Err(CacheError::InvalidPackageName(name.to_string()));
    }
    if name.starts_with('.') {
        return Err(CacheError::InvalidPackageName(name.to_string()));
    }
    if name.contains('/') {
        return Err(CacheError::InvalidPackageName(name.to_string()));
    }
    // Reject explicit `..` or any component equal to `..` once split by `/`
    // (already handled by the `/` guard above, but kept for defence-in-depth).
    if name == ".." || name.contains("..") {
        return Err(CacheError::InvalidPackageName(name.to_string()));
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// CacheManager
// ─────────────────────────────────────────────────────────────────────────────

/// Cache manager for oxilake build artifacts.
///
/// Stores compiled artifacts in `<cache_dir>/<package-name>/<hash>/`.
/// By default the cache root is `~/.oxilake/cache/`; call
/// [`CacheManager::with_dir`] to use an explicit directory (e.g. in tests).
pub struct CacheManager {
    cache_dir: PathBuf,
}

impl CacheManager {
    /// Create a [`CacheManager`] rooted at the default location
    /// (`~/.oxilake/cache/`).
    ///
    /// Creates the directory if it does not exist.
    pub fn new() -> Result<Self, CacheError> {
        let home = home_dir().ok_or(CacheError::HomeNotFound)?;
        let cache_dir = home.join(".oxilake").join("cache");
        Self::with_dir(cache_dir)
    }

    /// Create a [`CacheManager`] with an explicit cache directory.
    ///
    /// The directory is created (including all parents) if it does not exist.
    /// Use this in tests to avoid touching the real `~/.oxilake/cache/`.
    pub fn with_dir(dir: PathBuf) -> Result<Self, CacheError> {
        std::fs::create_dir_all(&dir).map_err(CacheError::Io)?;
        Ok(Self { cache_dir: dir })
    }

    /// Return the path for a cached artifact directory given a package name and
    /// content hash.
    ///
    /// The path is `<cache_dir>/<package_name>/<hash_hex>/`.
    /// This function does **not** create the directory; call
    /// [`store_artifact`](CacheManager::store_artifact) for that.
    pub fn artifact_path(&self, package_name: &str, hash: u64) -> PathBuf {
        self.cache_dir
            .join(package_name)
            .join(format!("{:016x}", hash))
    }

    /// Check whether a cached artifact directory exists for `(package_name, hash)`.
    pub fn has_artifact(&self, package_name: &str, hash: u64) -> bool {
        self.artifact_path(package_name, hash).exists()
    }

    /// Store `data` as a cached artifact file.
    ///
    /// Creates `<cache_dir>/<package_name>/<hash>/` if necessary, then writes
    /// `data` to `<cache_dir>/<package_name>/<hash>/<filename>`.
    ///
    /// Returns the full path to the written file.
    ///
    /// # Errors
    /// - [`CacheError::InvalidPackageName`] — `package_name` is invalid.
    /// - [`CacheError::Io`] — directory creation or file write failed.
    pub fn store_artifact(
        &self,
        package_name: &str,
        hash: u64,
        data: &[u8],
        filename: &str,
    ) -> Result<PathBuf, CacheError> {
        validate_package_name(package_name)?;

        let artifact_dir = self.artifact_path(package_name, hash);
        std::fs::create_dir_all(&artifact_dir).map_err(CacheError::Io)?;

        let file_path = artifact_dir.join(filename);
        std::fs::write(&file_path, data).map_err(CacheError::Io)?;

        Ok(file_path)
    }

    /// Read a cached artifact file.
    ///
    /// Returns the file contents as a `Vec<u8>`.
    ///
    /// # Errors
    /// - [`CacheError::InvalidPackageName`] — `package_name` is invalid.
    /// - [`CacheError::Io`] — file does not exist or could not be read.
    pub fn read_artifact(
        &self,
        package_name: &str,
        hash: u64,
        filename: &str,
    ) -> Result<Vec<u8>, CacheError> {
        validate_package_name(package_name)?;
        let file_path = self.artifact_path(package_name, hash).join(filename);
        std::fs::read(&file_path).map_err(CacheError::Io)
    }

    /// Remove all cached artifact directories for `package_name`.
    ///
    /// If no cache exists for the package (i.e. the package directory is absent),
    /// this is a no-op (succeeds silently).
    ///
    /// # Errors
    /// - [`CacheError::InvalidPackageName`] — `package_name` is invalid.
    /// - [`CacheError::Io`] — removal failed for a reason other than "not found".
    pub fn clear_package(&self, package_name: &str) -> Result<(), CacheError> {
        validate_package_name(package_name)?;
        let pkg_dir = self.cache_dir.join(package_name);
        match std::fs::remove_dir_all(&pkg_dir) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(CacheError::Io(e)),
        }
    }

    /// Remove **all** cached artifacts (completely empties the cache directory).
    ///
    /// The cache directory itself is re-created after removal so subsequent
    /// operations can proceed without errors.
    ///
    /// # Errors
    /// - [`CacheError::Io`] — removal or re-creation failed.
    pub fn clear_all(&self) -> Result<(), CacheError> {
        // Remove each top-level entry (package directory) individually to avoid
        // accidentally deleting the cache root itself if a user configured an
        // unexpected path.
        let entries = match std::fs::read_dir(&self.cache_dir) {
            Ok(rd) => rd,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // Cache root doesn't exist — nothing to clear.
                return std::fs::create_dir_all(&self.cache_dir).map_err(CacheError::Io);
            }
            Err(e) => return Err(CacheError::Io(e)),
        };

        for entry in entries {
            let entry = entry.map_err(CacheError::Io)?;
            let path = entry.path();
            if path.is_dir() {
                std::fs::remove_dir_all(&path).map_err(CacheError::Io)?;
            } else {
                std::fs::remove_file(&path).map_err(CacheError::Io)?;
            }
        }

        Ok(())
    }

    /// Return the total number of artifact directories (hash-named subdirectories)
    /// across all packages in the cache.
    ///
    /// Each call to [`store_artifact`](CacheManager::store_artifact) with a
    /// previously unseen `(package_name, hash)` pair creates one new artifact
    /// directory; this function counts those directories.
    ///
    /// # Errors
    /// - [`CacheError::Io`] — the cache directory could not be read.
    pub fn artifact_count(&self) -> Result<usize, CacheError> {
        let pkg_entries = match std::fs::read_dir(&self.cache_dir) {
            Ok(rd) => rd,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
            Err(e) => return Err(CacheError::Io(e)),
        };

        let mut count = 0usize;

        for pkg_entry in pkg_entries {
            let pkg_entry = pkg_entry.map_err(CacheError::Io)?;
            let pkg_path = pkg_entry.path();
            if !pkg_path.is_dir() {
                continue;
            }

            // Count hash-named subdirectories inside this package directory.
            let hash_entries = match std::fs::read_dir(&pkg_path) {
                Ok(rd) => rd,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                Err(e) => return Err(CacheError::Io(e)),
            };

            for hash_entry in hash_entries {
                let hash_entry = hash_entry.map_err(CacheError::Io)?;
                if hash_entry.path().is_dir() {
                    count += 1;
                }
            }
        }

        Ok(count)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    /// Create an isolated `CacheManager` backed by a unique temp directory.
    fn make_cache(suffix: &str) -> CacheManager {
        let dir = env::temp_dir().join(format!("oxilake_cache_test_{suffix}"));
        fs::remove_dir_all(&dir).ok();
        CacheManager::with_dir(dir).expect("create CacheManager")
    }

    // ── 1. with_dir creates the directory ────────────────────────────────────

    #[test]
    fn test_new_creates_dir() {
        let dir = env::temp_dir().join("oxilake_cache_creates_dir_001");
        fs::remove_dir_all(&dir).ok();

        assert!(!dir.exists(), "pre-condition: dir should not exist yet");

        let _cm = CacheManager::with_dir(dir.clone()).expect("with_dir should succeed");
        assert!(dir.exists(), "cache dir should be created by with_dir");

        fs::remove_dir_all(&dir).ok();
    }

    // ── 2. artifact_path returns the expected path ────────────────────────────

    #[test]
    fn test_artifact_path() {
        let cm = make_cache("artifact_path_002");
        let path = cm.artifact_path("mypkg", 0xabc);
        // hash 0xabc = 2748 → "0000000000000abc"
        let expected = cm.cache_dir.join("mypkg").join("0000000000000abc");
        assert_eq!(path, expected);

        fs::remove_dir_all(&cm.cache_dir).ok();
    }

    // ── 3. has_artifact returns false when nothing is stored ──────────────────

    #[test]
    fn test_has_artifact_missing() {
        let cm = make_cache("has_missing_003");
        assert!(!cm.has_artifact("pkg", 123));
        fs::remove_dir_all(&cm.cache_dir).ok();
    }

    // ── 4. store_artifact then read_artifact returns the same bytes ───────────

    #[test]
    fn test_store_and_read() {
        let cm = make_cache("store_read_004");
        let data = b"hello, oxilake cache!";
        cm.store_artifact("mypkg", 0xdeadbeef, data, "out.leanc")
            .expect("store artifact");

        let read_back = cm
            .read_artifact("mypkg", 0xdeadbeef, "out.leanc")
            .expect("read artifact");
        assert_eq!(read_back, data);

        fs::remove_dir_all(&cm.cache_dir).ok();
    }

    // ── 5. has_artifact returns true after store ──────────────────────────────

    #[test]
    fn test_has_artifact_after_store() {
        let cm = make_cache("has_after_store_005");
        assert!(!cm.has_artifact("pkg", 42));

        cm.store_artifact("pkg", 42, b"data", "f.leanc")
            .expect("store");

        assert!(cm.has_artifact("pkg", 42));
        fs::remove_dir_all(&cm.cache_dir).ok();
    }

    // ── 6. clear_package removes all artifacts for that package ───────────────

    #[test]
    fn test_clear_package() {
        let cm = make_cache("clear_package_006");
        cm.store_artifact("pkg_a", 1, b"data1", "a.leanc")
            .expect("store");
        cm.store_artifact("pkg_a", 2, b"data2", "b.leanc")
            .expect("store");

        assert!(cm.has_artifact("pkg_a", 1));
        assert!(cm.has_artifact("pkg_a", 2));

        cm.clear_package("pkg_a").expect("clear");

        assert!(!cm.has_artifact("pkg_a", 1));
        assert!(!cm.has_artifact("pkg_a", 2));

        fs::remove_dir_all(&cm.cache_dir).ok();
    }

    // ── 7. clear_all removes all artifacts across all packages ────────────────

    #[test]
    fn test_clear_all() {
        let cm = make_cache("clear_all_007");
        cm.store_artifact("pkg_x", 10, b"x_data", "x.leanc")
            .expect("store x");
        cm.store_artifact("pkg_y", 20, b"y_data", "y.leanc")
            .expect("store y");

        assert_eq!(cm.artifact_count().expect("count before"), 2);

        cm.clear_all().expect("clear all");

        let count_after = cm.artifact_count().expect("count after");
        assert_eq!(count_after, 0, "cache should be empty after clear_all");

        fs::remove_dir_all(&cm.cache_dir).ok();
    }

    // ── 8. artifact_count returns the correct count ───────────────────────────

    #[test]
    fn test_artifact_count() {
        let cm = make_cache("artifact_count_008");

        // 2 artifacts for pkg_alpha, 1 for pkg_beta → total 3
        cm.store_artifact("pkg_alpha", 100, b"a1", "f1.leanc")
            .expect("store");
        cm.store_artifact("pkg_alpha", 200, b"a2", "f2.leanc")
            .expect("store");
        cm.store_artifact("pkg_beta", 300, b"b1", "f3.leanc")
            .expect("store");

        let count = cm.artifact_count().expect("count");
        assert_eq!(count, 3);

        fs::remove_dir_all(&cm.cache_dir).ok();
    }

    // ── 9. invalid package name is rejected ───────────────────────────────────

    #[test]
    fn test_invalid_package_name() {
        let cm = make_cache("invalid_name_009");

        let result = cm.store_artifact("../evil", 0xabc, b"data", "out.leanc");
        assert!(
            result.is_err(),
            "path traversal package name should be rejected"
        );
        match result.unwrap_err() {
            CacheError::InvalidPackageName(name) => {
                assert_eq!(name, "../evil");
            }
            other => panic!("expected InvalidPackageName, got: {other}"),
        }

        // Also test forward-slash
        let result2 = cm.store_artifact("foo/bar", 1, b"d", "f.leanc");
        assert!(result2.is_err());
        match result2.unwrap_err() {
            CacheError::InvalidPackageName(_) => {}
            other => panic!("expected InvalidPackageName for slash, got: {other}"),
        }

        fs::remove_dir_all(&cm.cache_dir).ok();
    }

    // ── 10. different hashes produce different artifact paths ─────────────────

    #[test]
    fn test_store_different_hashes() {
        let cm = make_cache("different_hashes_010");

        cm.store_artifact("pkg", 0xaaaa, b"version_a", "out.leanc")
            .expect("store hash_a");
        cm.store_artifact("pkg", 0xbbbb, b"version_b", "out.leanc")
            .expect("store hash_b");

        let path_a = cm.artifact_path("pkg", 0xaaaa);
        let path_b = cm.artifact_path("pkg", 0xbbbb);

        assert_ne!(
            path_a, path_b,
            "different hashes must yield different paths"
        );
        assert!(path_a.exists(), "hash_a artifact dir should exist");
        assert!(path_b.exists(), "hash_b artifact dir should exist");

        // The contents should be independent.
        let data_a = cm
            .read_artifact("pkg", 0xaaaa, "out.leanc")
            .expect("read_a");
        let data_b = cm
            .read_artifact("pkg", 0xbbbb, "out.leanc")
            .expect("read_b");
        assert_eq!(data_a, b"version_a");
        assert_eq!(data_b, b"version_b");

        fs::remove_dir_all(&cm.cache_dir).ok();
    }
}
