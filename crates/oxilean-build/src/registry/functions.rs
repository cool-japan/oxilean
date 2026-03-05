//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::manifest::{Dependency, ManifestError, Version, VersionConstraint};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

use super::types::{
    AuthToken, CredentialStore, DependencyMirror, DownloadLog, DownloadQueue, DownloadRecord,
    DownloadRequest, DownloadResult, MirrorList, PackageInfo, PackageMetadata, PackageOwner,
    PackagePacker, PackageValidator, PackageVersionIndex, RegistryClient, RegistryConfig,
    RegistryError, RegistryIndex, RegistryIndexEntry, RegistryManager, RegistryMirror,
    RegistryNotification, RegistryStats, VersionInfo,
};

/// Compute a deterministic hex checksum for a package given its name and version.
///
/// Uses [`DefaultHasher`] from the standard library so no external crate is
/// required.  The result is a 16-character lowercase hex string derived from a
/// 64-bit hash of `"<name>@<version>"`.
///
/// In a production registry client this would be replaced by the SHA-256
/// digest of the downloaded archive bytes, but this implementation already
/// produces a non-trivial, reproducible value suitable for cache keying and
/// basic integrity checks in tests.
pub(super) fn compute_package_checksum(name: &str, version: &str) -> String {
    let mut hasher = DefaultHasher::new();
    let key = format!("{}@{}", name, version);
    key.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
/// Compute a checksum for raw bytes (e.g., an archive read from disk).
///
/// Returns the same 16-char hex representation as [`compute_package_checksum`]
/// but operates on arbitrary byte slices so it can be used to verify a
/// downloaded archive.
pub(super) fn compute_bytes_checksum(bytes: &[u8]) -> String {
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}
/// Serialize a `PackageInfo` to a simple line-oriented text format and write
/// it to `cache_dir/{name}.pkg-info`.
pub(super) fn persist_pkg_info(cache_dir: &Path, info: &PackageInfo) -> std::io::Result<()> {
    std::fs::create_dir_all(cache_dir)?;
    let path = cache_dir.join(format!("{}.pkg-info", info.name));
    let mut text = String::new();
    text.push_str(&format!("name: {}\n", info.name));
    text.push_str(&format!("latest: {}\n", info.latest_version));
    text.push_str(&format!("downloads: {}\n", info.downloads));
    text.push_str(&format!("created_at: {}\n", info.created_at));
    text.push_str(&format!("updated_at: {}\n", info.updated_at));
    if let Some(ref d) = info.description {
        text.push_str(&format!("description: {}\n", d));
    }
    if let Some(ref l) = info.license {
        text.push_str(&format!("license: {}\n", l));
    }
    if let Some(ref r) = info.repository {
        text.push_str(&format!("repository: {}\n", r));
    }
    for author in &info.authors {
        text.push_str(&format!("author: {}\n", author));
    }
    for kw in &info.keywords {
        text.push_str(&format!("keyword: {}\n", kw));
    }
    for v in &info.versions {
        let yanked = if v.yanked { "yanked" } else { "ok" };
        text.push_str(&format!(
            "version: {} {} {} {}\n",
            v.version, yanked, v.downloads, v.published_at
        ));
    }
    std::fs::write(path, text)
}
/// Parse the simple text format written by `persist_pkg_info`.
pub(super) fn parse_pkg_info_text(text: &str) -> Option<PackageInfo> {
    let mut name = String::new();
    let mut latest_str = String::new();
    let mut downloads = 0u64;
    let mut created_at = String::new();
    let mut updated_at = String::new();
    let mut description: Option<String> = None;
    let mut license: Option<String> = None;
    let mut repository: Option<String> = None;
    let mut authors: Vec<String> = Vec::new();
    let mut keywords: Vec<String> = Vec::new();
    let mut versions: Vec<VersionInfo> = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if let Some(val) = line.strip_prefix("name: ") {
            name = val.to_string();
        } else if let Some(val) = line.strip_prefix("latest: ") {
            latest_str = val.to_string();
        } else if let Some(val) = line.strip_prefix("downloads: ") {
            downloads = val.parse().unwrap_or(0);
        } else if let Some(val) = line.strip_prefix("created_at: ") {
            created_at = val.to_string();
        } else if let Some(val) = line.strip_prefix("updated_at: ") {
            updated_at = val.to_string();
        } else if let Some(val) = line.strip_prefix("description: ") {
            description = Some(val.to_string());
        } else if let Some(val) = line.strip_prefix("license: ") {
            license = Some(val.to_string());
        } else if let Some(val) = line.strip_prefix("repository: ") {
            repository = Some(val.to_string());
        } else if let Some(val) = line.strip_prefix("author: ") {
            authors.push(val.to_string());
        } else if let Some(val) = line.strip_prefix("keyword: ") {
            keywords.push(val.to_string());
        } else if let Some(val) = line.strip_prefix("version: ") {
            let parts: Vec<&str> = val.splitn(4, ' ').collect();
            if !parts.is_empty() {
                let ver = crate::manifest::Version::parse(parts[0]).ok()?;
                let yanked = parts.get(1).is_some_and(|s| *s == "yanked");
                let dl: u64 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
                let pub_at = parts.get(3).copied().unwrap_or("").to_string();
                let checksum = compute_package_checksum(&name, parts[0]);
                versions.push(VersionInfo {
                    version: ver,
                    yanked,
                    downloads: dl,
                    published_at: pub_at,
                    checksum,
                    dependencies: Vec::new(),
                    min_oxilean_version: None,
                    size: 0,
                });
            }
        }
    }
    if name.is_empty() {
        return None;
    }
    let latest_version = crate::manifest::Version::parse(&latest_str).ok()?;
    Some(PackageInfo {
        name,
        latest_version,
        versions,
        description,
        license,
        repository,
        documentation: None,
        downloads,
        authors,
        keywords,
        categories: Vec::new(),
        created_at,
        updated_at,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_registry_config() {
        let config = RegistryConfig::default_registry();
        assert!(config.is_default);
        assert!(!config.is_authenticated());
    }
    #[test]
    fn test_auth_token() {
        let token = AuthToken::bearer("my-secret-token")
            .with_scope("publish")
            .with_scope("read");
        assert!(token.has_scope("publish"));
        assert!(token.has_scope("read"));
        assert!(!token.has_scope("admin"));
        assert!(!token.is_expired(0));
        assert_eq!(token.auth_header(), "Bearer my-secret-token");
    }
    #[test]
    fn test_auth_token_redacted() {
        let token = AuthToken::bearer("abcdefghijklmnop");
        let redacted = token.redacted();
        assert!(redacted.starts_with("abcd"));
        assert!(redacted.ends_with("mnop"));
        assert!(redacted.contains("..."));
    }
    #[test]
    fn test_registry_manager() {
        let mut manager = RegistryManager::new();
        assert!(manager.default_registry().is_some());
        let custom = RegistryConfig::new("custom", "https://custom.example.com");
        manager.add_registry(custom);
        assert!(manager.get_registry("custom").is_some());
    }
    #[test]
    fn test_package_validator() {
        let mut validator = PackageValidator::new();
        let valid = validator.validate(
            "my-pkg",
            &Version::new(1, 0, 0),
            Some("A test package"),
            Some("MIT"),
            &["Author".to_string()],
        );
        assert!(valid);
        assert!(validator.errors().is_empty());
        let valid = validator.validate("", &Version::new(1, 0, 0), Some("desc"), Some("MIT"), &[]);
        assert!(!valid);
    }
    #[test]
    fn test_package_validator_reserved() {
        let mut validator = PackageValidator::new();
        let valid = validator.validate(
            "std",
            &Version::new(1, 0, 0),
            Some("desc"),
            Some("MIT"),
            &["Author".to_string()],
        );
        assert!(!valid);
    }
    #[test]
    fn test_package_packer_exclude() {
        let packer = PackagePacker::new();
        assert!(packer.should_exclude(Path::new(".git/config")));
        assert!(packer.should_exclude(Path::new("target/debug/foo")));
        assert!(packer.should_exclude(Path::new("file.tmp")));
        assert!(!packer.should_exclude(Path::new("src/lib.rs")));
    }
    #[test]
    fn test_dependency_mirror() {
        let mut mirror = DependencyMirror::new(Path::new("/tmp/mirror"));
        mirror.add_package("foo", Version::new(1, 0, 0));
        mirror.add_package("foo", Version::new(1, 1, 0));
        mirror.add_package("bar", Version::new(2, 0, 0));
        assert!(mirror.has_package("foo", &Version::new(1, 0, 0)));
        assert!(!mirror.has_package("foo", &Version::new(2, 0, 0)));
        assert_eq!(mirror.total_versions(), 3);
    }
    #[test]
    fn test_registry_client_create() {
        let config = RegistryConfig::default_registry();
        let _client = RegistryClient::new(config);
    }
    #[test]
    fn test_registry_download_url() {
        let config = RegistryConfig::default_registry();
        let url = config.package_download_url("mathlib", &Version::new(4, 0, 0));
        assert!(url.contains("mathlib"));
        assert!(url.contains("4.0.0"));
    }
    #[test]
    fn test_credential_store() {
        let mut store = CredentialStore::new(Path::new("/tmp/creds"));
        store.store("https://registry.example.com", AuthToken::bearer("secret"));
        assert!(store.retrieve("https://registry.example.com").is_some());
        assert!(store.retrieve("https://other.com").is_none());
        assert_eq!(store.count(), 1);
        store.remove("https://registry.example.com");
        assert_eq!(store.count(), 0);
    }
    #[test]
    fn test_credential_store_list() {
        let mut store = CredentialStore::new(Path::new("/tmp/creds"));
        store.store("https://a.com", AuthToken::bearer("t1"));
        store.store("https://b.com", AuthToken::bearer("t2"));
        let registries = store.list_registries();
        assert_eq!(registries.len(), 2);
    }
    #[test]
    fn test_registry_index() {
        let mut index = RegistryIndex::new("test");
        index.upsert(RegistryIndexEntry {
            name: "foo".to_string(),
            versions: vec![Version::new(1, 0, 0), Version::new(1, 1, 0)],
            latest: Some(Version::new(1, 1, 0)),
            yanked_versions: Vec::new(),
        });
        assert!(index.contains("foo"));
        assert!(!index.contains("bar"));
        assert_eq!(index.package_count(), 1);
        assert_eq!(index.latest_version("foo"), Some(&Version::new(1, 1, 0)));
        assert_eq!(index.versions_of("foo").len(), 2);
    }
    #[test]
    fn test_registry_index_search() {
        let mut index = RegistryIndex::new("test");
        index.upsert(RegistryIndexEntry {
            name: "mathlib".to_string(),
            versions: vec![Version::new(4, 0, 0)],
            latest: Some(Version::new(4, 0, 0)),
            yanked_versions: Vec::new(),
        });
        index.upsert(RegistryIndexEntry {
            name: "math-extra".to_string(),
            versions: vec![Version::new(1, 0, 0)],
            latest: Some(Version::new(1, 0, 0)),
            yanked_versions: Vec::new(),
        });
        index.upsert(RegistryIndexEntry {
            name: "topology".to_string(),
            versions: vec![Version::new(1, 0, 0)],
            latest: Some(Version::new(1, 0, 0)),
            yanked_versions: Vec::new(),
        });
        let results = index.search("math");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_download_queue() {
        let mut queue = DownloadQueue::new(2);
        queue.enqueue(DownloadRequest {
            name: "foo".to_string(),
            version: Version::new(1, 0, 0),
            registry: "default".to_string(),
            priority: 0,
        });
        queue.enqueue(DownloadRequest {
            name: "bar".to_string(),
            version: Version::new(2, 0, 0),
            registry: "default".to_string(),
            priority: 1,
        });
        queue.enqueue(DownloadRequest {
            name: "baz".to_string(),
            version: Version::new(3, 0, 0),
            registry: "default".to_string(),
            priority: 2,
        });
        assert_eq!(queue.pending_count(), 3);
        let batch = queue.next_batch();
        assert_eq!(batch.len(), 2);
        assert_eq!(queue.pending_count(), 1);
        queue.mark_completed(DownloadResult {
            name: "foo".to_string(),
            version: Version::new(1, 0, 0),
            archive_path: PathBuf::from("/tmp/foo.tar.gz"),
            extracted_path: PathBuf::from("/tmp/foo"),
            checksum: "abc".to_string(),
            size: 1024,
        });
        assert_eq!(queue.completed_count(), 1);
    }
    #[test]
    fn test_download_queue_priority() {
        let mut queue = DownloadQueue::new(10);
        queue.enqueue(DownloadRequest {
            name: "low".to_string(),
            version: Version::new(1, 0, 0),
            registry: "default".to_string(),
            priority: 10,
        });
        queue.enqueue(DownloadRequest {
            name: "high".to_string(),
            version: Version::new(1, 0, 0),
            registry: "default".to_string(),
            priority: 0,
        });
        let batch = queue.next_batch();
        assert_eq!(batch[0].name, "high");
        assert_eq!(batch[1].name, "low");
    }
    #[test]
    fn test_registry_error_display() {
        let err = RegistryError::PackageNotFound("nonexistent".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("nonexistent"));
        let err2 = RegistryError::ChecksumMismatch {
            expected: "abc".to_string(),
            actual: "def".to_string(),
        };
        let msg2 = format!("{}", err2);
        assert!(msg2.contains("abc"));
        assert!(msg2.contains("def"));
    }
    #[test]
    fn test_registry_manager_credentials() {
        let mut manager = RegistryManager::new();
        manager.store_credentials("oxilean", AuthToken::bearer("secret"));
        assert!(manager.get_credentials("oxilean").is_some());
        assert!(manager.get_credentials("other").is_none());
    }
    #[test]
    fn test_registry_manager_default() {
        let mut manager = RegistryManager::new();
        let custom = RegistryConfig::new("custom", "https://custom.example.com");
        manager.add_registry(custom);
        assert!(manager.set_default("custom"));
        assert!(!manager.set_default("nonexistent"));
    }
    #[test]
    fn test_registry_client_upload() {
        let config = RegistryConfig::default_registry()
            .with_auth(AuthToken::bearer("secret").with_scope("publish"));
        let mut client = RegistryClient::new(config);
        let v1 = Version::new(1, 0, 0);
        let result = client.upload("my-pkg", &v1, Path::new("/tmp/my-pkg-1.0.0.tar.gz"));
        assert!(result.is_ok());
        let up = result.expect("test operation should succeed");
        assert_eq!(up.name, "my-pkg");
        assert_eq!(up.version, v1);
        let info = client.get_package_info("my-pkg").expect("should be cached");
        assert_eq!(info.name, "my-pkg");
        assert_eq!(info.versions.len(), 1);
        assert_eq!(info.latest_version, v1);
    }
    #[test]
    fn test_registry_client_upload_duplicate_version() {
        let config = RegistryConfig::default_registry()
            .with_auth(AuthToken::bearer("secret").with_scope("publish"));
        let mut client = RegistryClient::new(config);
        let v1 = Version::new(1, 0, 0);
        client
            .upload("my-pkg", &v1, Path::new("/tmp/x.tar.gz"))
            .expect("test operation should succeed");
        let err = client
            .upload("my-pkg", &v1, Path::new("/tmp/x.tar.gz"))
            .unwrap_err();
        assert!(matches!(err, RegistryError::VersionExists { .. }));
    }
    #[test]
    fn test_registry_client_upload_requires_auth() {
        let config = RegistryConfig::default_registry();
        let mut client = RegistryClient::new(config);
        let err = client
            .upload("pkg", &Version::new(1, 0, 0), Path::new("/tmp/x.tar.gz"))
            .unwrap_err();
        assert!(matches!(err, RegistryError::AuthRequired));
    }
    #[test]
    fn test_registry_client_yank_and_unyank() {
        let config = RegistryConfig::default_registry()
            .with_auth(AuthToken::bearer("secret").with_scope("publish"));
        let mut client = RegistryClient::new(config);
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(2, 0, 0);
        client
            .upload("foo", &v1, Path::new("/tmp/x.tar.gz"))
            .expect("test operation should succeed");
        client
            .upload("foo", &v2, Path::new("/tmp/x.tar.gz"))
            .expect("test operation should succeed");
        client
            .yank("foo", &v2)
            .expect("test operation should succeed");
        let info = client
            .get_package_info("foo")
            .expect("test operation should succeed");
        assert_eq!(info.latest_version, v1);
        let v2_info = info
            .versions
            .iter()
            .find(|v| v.version == v2)
            .expect("test operation should succeed");
        assert!(v2_info.yanked);
        client
            .unyank("foo", &v2)
            .expect("test operation should succeed");
        let info2 = client
            .get_package_info("foo")
            .expect("test operation should succeed");
        assert_eq!(info2.latest_version, v2);
        let v2_info2 = info2
            .versions
            .iter()
            .find(|v| v.version == v2)
            .expect("test operation should succeed");
        assert!(!v2_info2.yanked);
    }
    #[test]
    fn test_registry_client_yank_unknown_package() {
        let config = RegistryConfig::default_registry()
            .with_auth(AuthToken::bearer("secret").with_scope("publish"));
        let mut client = RegistryClient::new(config);
        let err = client
            .yank("nonexistent", &Version::new(1, 0, 0))
            .unwrap_err();
        assert!(matches!(err, RegistryError::PackageNotFound(_)));
    }
    #[test]
    fn test_registry_client_yank_requires_auth() {
        let config = RegistryConfig::default_registry();
        let mut client = RegistryClient::new(config);
        let err = client.yank("any", &Version::new(1, 0, 0)).unwrap_err();
        assert!(matches!(err, RegistryError::AuthRequired));
    }
    #[test]
    fn test_registry_client_search() {
        let config = RegistryConfig::default_registry()
            .with_auth(AuthToken::bearer("secret").with_scope("publish"));
        let mut client = RegistryClient::new(config);
        client
            .upload(
                "mathlib",
                &Version::new(4, 0, 0),
                Path::new("/tmp/x.tar.gz"),
            )
            .expect("test operation should succeed");
        client
            .upload(
                "math-extra",
                &Version::new(1, 0, 0),
                Path::new("/tmp/x.tar.gz"),
            )
            .expect("test operation should succeed");
        client
            .upload(
                "topology",
                &Version::new(1, 0, 0),
                Path::new("/tmp/x.tar.gz"),
            )
            .expect("test operation should succeed");
        let results = client
            .search("math", usize::MAX)
            .expect("test operation should succeed");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "math-extra");
        assert_eq!(results[1].name, "mathlib");
        let exact = client
            .search("topology", usize::MAX)
            .expect("test operation should succeed");
        assert_eq!(exact.len(), 1);
        assert_eq!(exact[0].name, "topology");
        let all = client
            .search("", usize::MAX)
            .expect("test operation should succeed");
        assert_eq!(all.len(), 3);
        let limited = client.search("", 2).expect("test operation should succeed");
        assert_eq!(limited.len(), 2);
        let upper = client
            .search("MATH", usize::MAX)
            .expect("test operation should succeed");
        assert_eq!(upper.len(), 2);
        let none = client
            .search("zzz", usize::MAX)
            .expect("test operation should succeed");
        assert!(none.is_empty());
    }
    #[test]
    fn test_registry_client_find_best_version_skips_yanked() {
        let config = RegistryConfig::default_registry()
            .with_auth(AuthToken::bearer("secret").with_scope("publish"));
        let mut client = RegistryClient::new(config);
        client
            .upload("pkg", &Version::new(1, 0, 0), Path::new("/tmp/x.tar.gz"))
            .expect("test operation should succeed");
        client
            .upload("pkg", &Version::new(2, 0, 0), Path::new("/tmp/x.tar.gz"))
            .expect("test operation should succeed");
        client
            .yank("pkg", &Version::new(2, 0, 0))
            .expect("test operation should succeed");
        let constraint = VersionConstraint::Any;
        let best = client
            .find_best_version("pkg", &constraint)
            .expect("test operation should succeed");
        assert_eq!(best, Version::new(1, 0, 0));
    }
}
#[cfg(test)]
mod registry_extra_tests {
    use super::*;
    use crate::manifest::Version;
    #[test]
    fn package_metadata_builder() {
        let meta = PackageMetadata::new()
            .with_description("A math library")
            .with_license("Apache-2.0")
            .with_keyword("math")
            .with_category("algebra");
        assert_eq!(meta.description.as_deref(), Some("A math library"));
        assert_eq!(meta.keywords.len(), 1);
        assert_eq!(meta.categories.len(), 1);
    }
    #[test]
    fn registry_index_add_and_count() {
        let mut idx = PackageVersionIndex::new();
        idx.add_version("pkg-a", Version::new(1, 0, 0));
        idx.add_version("pkg-a", Version::new(1, 1, 0));
        idx.add_version("pkg-b", Version::new(0, 1, 0));
        assert_eq!(idx.package_count(), 2);
        assert_eq!(idx.version_count("pkg-a"), 2);
        assert_eq!(idx.total_versions(), 3);
    }
    #[test]
    fn registry_index_has_version() {
        let mut idx = PackageVersionIndex::new();
        idx.add_version("pkg", Version::new(2, 0, 0));
        assert!(idx.has_version("pkg", &Version::new(2, 0, 0)));
        assert!(!idx.has_version("pkg", &Version::new(3, 0, 0)));
    }
    #[test]
    fn download_log_success_rate() {
        let mut log = DownloadLog::new();
        log.push(DownloadRecord::success(
            "pkg",
            Version::new(1, 0, 0),
            0,
            1024,
        ));
        log.push(DownloadRecord::failure("pkg", Version::new(2, 0, 0), 1));
        assert!((log.success_rate() - 0.5).abs() < 1e-9);
        assert_eq!(log.total_bytes(), 1024);
    }
    #[test]
    fn mirror_list_priority_order() {
        let mut list = MirrorList::new();
        list.add(RegistryMirror::new("https://mirror-b.example.com", 2));
        list.add(RegistryMirror::new("https://mirror-a.example.com", 1));
        let active = list.active();
        assert_eq!(active[0].priority, 1);
        assert_eq!(active[1].priority, 2);
    }
    #[test]
    fn mirror_list_active_excludes_disabled() {
        let mut list = MirrorList::new();
        list.add(RegistryMirror::new("https://active.example.com", 1));
        list.add(RegistryMirror::new("https://disabled.example.com", 2).disable());
        assert_eq!(list.active().len(), 1);
    }
}
/// Returns the registry client API version.
pub fn registry_api_version() -> &'static str {
    "v1"
}
#[cfg(test)]
mod registry_api_version_test {
    use super::*;
    #[test]
    fn api_version_nonempty() {
        assert!(!registry_api_version().is_empty());
    }
}
#[cfg(test)]
mod registry_stats_owner_tests {
    use super::*;
    #[test]
    fn registry_stats_hit_rate() {
        let mut s = RegistryStats::new();
        s.cache_hits = 7;
        s.cache_misses = 3;
        assert!((s.hit_rate() - 0.7).abs() < 1e-9);
    }
    #[test]
    fn registry_stats_summary() {
        let s = RegistryStats::new();
        assert!(!s.summary().is_empty());
    }
    #[test]
    fn package_owner_display_name() {
        let o = PackageOwner::new("jdoe").with_name("John Doe");
        assert_eq!(o.display_name(), "John Doe");
        let o2 = PackageOwner::new("alice");
        assert_eq!(o2.display_name(), "alice");
    }
    #[test]
    fn package_owner_display() {
        let o = PackageOwner::new("bob").with_name("Bob Smith");
        assert_eq!(format!("{}", o), "Bob Smith");
    }
}
#[cfg(test)]
mod notification_tests {
    use super::*;
    use crate::manifest::Version;
    #[test]
    fn notification_labels() {
        let n = RegistryNotification::Published {
            package: "pkg".into(),
            version: Version::new(1, 0, 0),
        };
        assert_eq!(n.label(), "published");
        let conn = RegistryNotification::ConnectionFailed("timeout".into());
        assert_eq!(conn.label(), "connection-failed");
    }
    #[test]
    fn notification_display() {
        let n = RegistryNotification::Downloaded {
            package: "pkg".into(),
            version: Version::new(2, 0, 0),
        };
        let s = format!("{}", n);
        assert!(s.contains("downloaded"));
    }
}
/// Returns the number of registry mirrors configured by default.
pub fn default_mirror_count() -> usize {
    0
}
#[cfg(test)]
mod mirror_count_test {
    use super::*;
    #[test]
    fn default_mirror_count_is_zero() {
        assert_eq!(default_mirror_count(), 0);
    }
}
