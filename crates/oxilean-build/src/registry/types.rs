//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::{
    compute_bytes_checksum, compute_package_checksum, parse_pkg_info_text, persist_pkg_info,
};
use crate::manifest::{Dependency, ManifestError, Version, VersionConstraint};
use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

/// Represents an owner (user or team) of a package.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageOwner {
    /// Owner login name.
    pub login: String,
    /// Display name.
    pub name: Option<String>,
    /// Email address.
    pub email: Option<String>,
}
impl PackageOwner {
    /// Create an owner with just a login.
    pub fn new(login: &str) -> Self {
        Self {
            login: login.to_string(),
            name: None,
            email: None,
        }
    }
    /// Set display name.
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }
    /// Set email.
    pub fn with_email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }
    /// Display name or login.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.login)
    }
}
/// Local index of available packages from a registry.
pub struct RegistryIndex {
    /// Index entries.
    entries: HashMap<String, RegistryIndexEntry>,
    /// Last sync timestamp.
    last_synced: u64,
    /// Registry name.
    registry_name: String,
}
impl RegistryIndex {
    /// Create a new empty index.
    pub fn new(registry_name: &str) -> Self {
        Self {
            entries: HashMap::new(),
            last_synced: 0,
            registry_name: registry_name.to_string(),
        }
    }
    /// Add or update an entry.
    pub fn upsert(&mut self, entry: RegistryIndexEntry) {
        self.entries.insert(entry.name.clone(), entry);
    }
    /// Look up a package.
    pub fn lookup(&self, name: &str) -> Option<&RegistryIndexEntry> {
        self.entries.get(name)
    }
    /// Get all available versions for a package.
    pub fn versions_of(&self, name: &str) -> Vec<&Version> {
        self.entries
            .get(name)
            .map(|e| e.versions.iter().collect())
            .unwrap_or_default()
    }
    /// Get the latest version of a package.
    pub fn latest_version(&self, name: &str) -> Option<&Version> {
        self.entries.get(name).and_then(|e| e.latest.as_ref())
    }
    /// Check if the index contains a package.
    pub fn contains(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }
    /// Get the number of indexed packages.
    pub fn package_count(&self) -> usize {
        self.entries.len()
    }
    /// Get the registry name.
    pub fn registry_name(&self) -> &str {
        &self.registry_name
    }
    /// Get the last sync timestamp.
    pub fn last_synced(&self) -> u64 {
        self.last_synced
    }
    /// Set the last sync timestamp.
    pub fn set_last_synced(&mut self, ts: u64) {
        self.last_synced = ts;
    }
    /// Search the index for packages matching a query.
    pub fn search(&self, query: &str) -> Vec<&RegistryIndexEntry> {
        let query_lower = query.to_lowercase();
        self.entries
            .values()
            .filter(|e| e.name.to_lowercase().contains(&query_lower))
            .collect()
    }
}
/// A client for interacting with a package registry.
pub struct RegistryClient {
    /// Registry configuration.
    config: RegistryConfig,
    /// Local package index cache.
    index_cache: HashMap<String, PackageInfo>,
    /// Downloaded package cache.
    download_cache: HashMap<String, DownloadResult>,
}
impl RegistryClient {
    /// Create a new registry client.
    pub fn new(config: RegistryConfig) -> Self {
        Self {
            config,
            index_cache: HashMap::new(),
            download_cache: HashMap::new(),
        }
    }
    /// Get the registry configuration.
    pub fn config(&self) -> &RegistryConfig {
        &self.config
    }
    /// Query package information from the registry.
    ///
    /// Looks up in order: (1) in-memory index cache, (2) disk cache in
    /// `config.cache_dir/{name}.pkg-info`, (3) returns `PackageNotFound`.
    pub fn get_package_info(&mut self, name: &str) -> Result<&PackageInfo, RegistryError> {
        if self.index_cache.contains_key(name) {
            return Ok(&self.index_cache[name]);
        }
        let cache_file = self.config.cache_dir.join(format!("{}.pkg-info", name));
        if cache_file.exists() {
            if let Ok(text) = std::fs::read_to_string(&cache_file) {
                if let Some(info) = parse_pkg_info_text(&text) {
                    self.index_cache.insert(name.to_string(), info);
                    return Ok(&self.index_cache[name]);
                }
            }
        }
        Err(RegistryError::PackageNotFound(name.to_string()))
    }
    /// List all available versions of a package.
    pub fn list_versions(&mut self, name: &str) -> Result<Vec<VersionInfo>, RegistryError> {
        let info = self.get_package_info(name)?;
        Ok(info.versions.clone())
    }
    /// Find the best matching version for a constraint.
    pub fn find_best_version(
        &mut self,
        name: &str,
        constraint: &VersionConstraint,
    ) -> Result<Version, RegistryError> {
        let info = self.get_package_info(name)?;
        let mut matching: Vec<&VersionInfo> = info
            .versions
            .iter()
            .filter(|v| !v.yanked && constraint.matches(&v.version))
            .collect();
        matching.sort_by(|a, b| b.version.cmp(&a.version));
        matching
            .first()
            .map(|v| v.version.clone())
            .ok_or(RegistryError::VersionNotFound {
                package: name.to_string(),
                version: format!("{}", constraint),
            })
    }
    /// Download a specific package version.
    pub fn download(
        &mut self,
        name: &str,
        version: &Version,
    ) -> Result<DownloadResult, RegistryError> {
        let cache_key = format!("{}-{}", name, version);
        if let Some(cached) = self.download_cache.get(&cache_key) {
            return Ok(cached.clone());
        }
        if self.config.auth_token.is_none() {}
        let checksum = compute_package_checksum(name, &version.to_string());
        let result = DownloadResult {
            name: name.to_string(),
            version: version.clone(),
            archive_path: self
                .config
                .cache_dir
                .join(format!("{}-{}.tar.gz", name, version)),
            extracted_path: self.config.cache_dir.join(format!("{}-{}", name, version)),
            checksum,
            size: 0,
        };
        self.download_cache.insert(cache_key, result.clone());
        Ok(result)
    }
    /// Upload a package to the registry.
    ///
    /// Updates the in-memory `index_cache` so that subsequent calls to
    /// `get_package_info` / `search` reflect the newly published version.
    pub fn upload(
        &mut self,
        name: &str,
        version: &Version,
        _archive_path: &Path,
    ) -> Result<UploadResult, RegistryError> {
        let token = self
            .config
            .auth_token
            .as_ref()
            .ok_or(RegistryError::AuthRequired)?;
        if !token.has_scope("publish") && !token.scopes.is_empty() {
            return Err(RegistryError::AuthFailed(
                "token does not have 'publish' scope".to_string(),
            ));
        }
        let checksum = compute_package_checksum(name, &version.to_string());
        let new_ver_info = VersionInfo {
            version: version.clone(),
            yanked: false,
            downloads: 0,
            published_at: "now".to_string(),
            checksum,
            dependencies: Vec::new(),
            min_oxilean_version: None,
            size: 0,
        };
        let entry = self
            .index_cache
            .entry(name.to_string())
            .or_insert_with(|| PackageInfo {
                name: name.to_string(),
                latest_version: version.clone(),
                versions: Vec::new(),
                description: None,
                license: None,
                repository: None,
                documentation: None,
                downloads: 0,
                authors: Vec::new(),
                keywords: Vec::new(),
                categories: Vec::new(),
                created_at: "now".to_string(),
                updated_at: "now".to_string(),
            });
        if entry.versions.iter().any(|v| v.version == *version) {
            return Err(RegistryError::VersionExists {
                package: name.to_string(),
                version: version.clone(),
            });
        }
        entry.versions.push(new_ver_info);
        if let Some(best) = entry
            .versions
            .iter()
            .filter(|v| !v.yanked)
            .map(|v| &v.version)
            .max()
        {
            entry.latest_version = best.clone();
        }
        entry.updated_at = "now".to_string();
        let cache_dir = self.config.cache_dir.clone();
        if let Some(info) = self.index_cache.get(name) {
            let _ = persist_pkg_info(&cache_dir, info);
        }
        Ok(UploadResult {
            name: name.to_string(),
            version: version.clone(),
            registry_url: format!("{}/crates/{}/{}", self.config.url, name, version),
            warnings: Vec::new(),
        })
    }
    /// Yank a specific version (prevent new downloads but don't delete).
    ///
    /// Marks the given version as yanked in the in-memory `index_cache` and
    /// recalculates `latest_version` to skip yanked entries.
    pub fn yank(&mut self, name: &str, version: &Version) -> Result<(), RegistryError> {
        let _token = self
            .config
            .auth_token
            .as_ref()
            .ok_or(RegistryError::AuthRequired)?;
        let info = self
            .index_cache
            .get_mut(name)
            .ok_or_else(|| RegistryError::PackageNotFound(name.to_string()))?;
        let ver_info = info
            .versions
            .iter_mut()
            .find(|v| v.version == *version)
            .ok_or_else(|| RegistryError::VersionNotFound {
                package: name.to_string(),
                version: version.to_string(),
            })?;
        ver_info.yanked = true;
        if let Some(best) = info
            .versions
            .iter()
            .filter(|v| !v.yanked)
            .map(|v| &v.version)
            .max()
        {
            info.latest_version = best.clone();
        }
        Ok(())
    }
    /// Un-yank a specific version.
    ///
    /// Clears the yanked flag for the given version in the in-memory
    /// `index_cache` and recalculates `latest_version`.
    pub fn unyank(&mut self, name: &str, version: &Version) -> Result<(), RegistryError> {
        let _token = self
            .config
            .auth_token
            .as_ref()
            .ok_or(RegistryError::AuthRequired)?;
        let info = self
            .index_cache
            .get_mut(name)
            .ok_or_else(|| RegistryError::PackageNotFound(name.to_string()))?;
        let ver_info = info
            .versions
            .iter_mut()
            .find(|v| v.version == *version)
            .ok_or_else(|| RegistryError::VersionNotFound {
                package: name.to_string(),
                version: version.to_string(),
            })?;
        ver_info.yanked = false;
        if let Some(best) = info
            .versions
            .iter()
            .filter(|v| !v.yanked)
            .map(|v| &v.version)
            .max()
        {
            info.latest_version = best.clone();
        }
        Ok(())
    }
    /// Search for packages by query string.
    ///
    /// Filters the in-memory `index_cache` returning packages whose name
    /// contains `query` (case-insensitive).  At most `limit` results are
    /// returned; pass `usize::MAX` for unlimited results.
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<PackageInfo>, RegistryError> {
        let query_lower = query.to_lowercase();
        let mut results: Vec<PackageInfo> = self
            .index_cache
            .values()
            .filter(|info| info.name.to_lowercase().contains(&query_lower))
            .cloned()
            .collect();
        results.sort_by(|a, b| a.name.cmp(&b.name));
        results.truncate(limit);
        Ok(results)
    }
    /// Add a package to the local index cache.
    pub fn cache_package_info(&mut self, info: PackageInfo) {
        self.index_cache.insert(info.name.clone(), info);
    }
    /// Clear the local caches.
    pub fn clear_cache(&mut self) {
        self.index_cache.clear();
        self.download_cache.clear();
    }
    /// Verify a downloaded package's checksum.
    pub fn verify_checksum(
        &self,
        result: &DownloadResult,
        expected: &str,
    ) -> Result<(), RegistryError> {
        if result.checksum != expected {
            return Err(RegistryError::ChecksumMismatch {
                expected: expected.to_string(),
                actual: result.checksum.clone(),
            });
        }
        Ok(())
    }
}
/// Ordered log of download records.
pub struct DownloadLog {
    records: Vec<DownloadRecord>,
}
impl DownloadLog {
    /// Create an empty log.
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }
    /// Append a record.
    pub fn push(&mut self, rec: DownloadRecord) {
        self.records.push(rec);
    }
    /// Number of records.
    pub fn len(&self) -> usize {
        self.records.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
    /// Number of successful downloads.
    pub fn success_count(&self) -> usize {
        self.records.iter().filter(|r| r.success).count()
    }
    /// Total bytes downloaded.
    pub fn total_bytes(&self) -> u64 {
        self.records
            .iter()
            .filter(|r| r.success)
            .map(|r| r.size_bytes)
            .sum()
    }
    /// Success rate.
    pub fn success_rate(&self) -> f64 {
        if self.records.is_empty() {
            1.0
        } else {
            self.success_count() as f64 / self.records.len() as f64
        }
    }
}
/// Manages multiple registries.
pub struct RegistryManager {
    /// Configured registries.
    registries: BTreeMap<String, RegistryConfig>,
    /// Default registry name.
    default_registry: String,
    /// Credentials store.
    credentials: HashMap<String, AuthToken>,
}
impl RegistryManager {
    /// Create a new registry manager with the default registry.
    pub fn new() -> Self {
        let default = RegistryConfig::default_registry();
        let default_name = default.name.clone();
        let mut registries = BTreeMap::new();
        registries.insert(default_name.clone(), default);
        Self {
            registries,
            default_registry: default_name,
            credentials: HashMap::new(),
        }
    }
    /// Add a registry.
    pub fn add_registry(&mut self, config: RegistryConfig) {
        self.registries.insert(config.name.clone(), config);
    }
    /// Remove a registry.
    pub fn remove_registry(&mut self, name: &str) -> Option<RegistryConfig> {
        self.registries.remove(name)
    }
    /// Get a registry by name.
    pub fn get_registry(&self, name: &str) -> Option<&RegistryConfig> {
        self.registries.get(name)
    }
    /// Get the default registry.
    pub fn default_registry(&self) -> Option<&RegistryConfig> {
        self.registries.get(&self.default_registry)
    }
    /// Set the default registry.
    pub fn set_default(&mut self, name: &str) -> bool {
        if self.registries.contains_key(name) {
            self.default_registry = name.to_string();
            true
        } else {
            false
        }
    }
    /// List all configured registries.
    pub fn list_registries(&self) -> Vec<&RegistryConfig> {
        self.registries.values().collect()
    }
    /// Store credentials for a registry.
    pub fn store_credentials(&mut self, registry_name: &str, token: AuthToken) {
        self.credentials.insert(registry_name.to_string(), token);
    }
    /// Get credentials for a registry.
    pub fn get_credentials(&self, registry_name: &str) -> Option<&AuthToken> {
        self.credentials.get(registry_name)
    }
    /// Remove credentials for a registry.
    pub fn remove_credentials(&mut self, registry_name: &str) -> Option<AuthToken> {
        self.credentials.remove(registry_name)
    }
    /// Create a client for a specific registry.
    pub fn create_client(&self, registry_name: &str) -> Option<RegistryClient> {
        let config = self.registries.get(registry_name)?;
        let mut client_config = config.clone();
        if let Some(token) = self.credentials.get(registry_name) {
            client_config.auth_token = Some(token.clone());
        }
        Some(RegistryClient::new(client_config))
    }
    /// Create a client for the default registry.
    pub fn default_client(&self) -> Option<RegistryClient> {
        self.create_client(&self.default_registry)
    }
}
/// A local mirror of registry packages for offline builds.
pub struct DependencyMirror {
    /// Mirror directory.
    mirror_dir: PathBuf,
    /// Mirrored packages.
    packages: HashMap<String, Vec<Version>>,
}
impl DependencyMirror {
    /// Create a new mirror.
    pub fn new(mirror_dir: &Path) -> Self {
        Self {
            mirror_dir: mirror_dir.to_path_buf(),
            packages: HashMap::new(),
        }
    }
    /// Add a package version to the mirror.
    pub fn add_package(&mut self, name: &str, version: Version) {
        self.packages
            .entry(name.to_string())
            .or_default()
            .push(version);
    }
    /// Check if a package version is mirrored.
    pub fn has_package(&self, name: &str, version: &Version) -> bool {
        self.packages
            .get(name)
            .map(|versions| versions.contains(version))
            .unwrap_or(false)
    }
    /// Get the path to a mirrored package.
    pub fn package_path(&self, name: &str, version: &Version) -> PathBuf {
        self.mirror_dir.join(format!("{}-{}", name, version))
    }
    /// List all mirrored packages.
    pub fn list_packages(&self) -> Vec<(&str, &[Version])> {
        self.packages
            .iter()
            .map(|(name, versions)| (name.as_str(), versions.as_slice()))
            .collect()
    }
    /// Get the total number of mirrored versions.
    pub fn total_versions(&self) -> usize {
        self.packages.values().map(|v| v.len()).sum()
    }
    /// Get the mirror directory.
    pub fn mirror_dir(&self) -> &Path {
        &self.mirror_dir
    }
}
/// Error type for registry operations.
#[derive(Clone, Debug)]
pub enum RegistryError {
    /// Package not found.
    PackageNotFound(String),
    /// Version not found.
    VersionNotFound {
        /// Package name.
        package: String,
        /// Requested version.
        version: String,
    },
    /// Authentication required.
    AuthRequired,
    /// Authentication failed.
    AuthFailed(String),
    /// Network error.
    NetworkError(String),
    /// Rate limited.
    RateLimited {
        /// Seconds to wait before retrying.
        retry_after: u64,
    },
    /// Invalid package format.
    InvalidPackage(String),
    /// Checksum mismatch.
    ChecksumMismatch {
        /// Expected checksum.
        expected: String,
        /// Actual checksum.
        actual: String,
    },
    /// IO error.
    IoError(String),
    /// Manifest error.
    ManifestError(ManifestError),
    /// Version already exists.
    VersionExists {
        /// Package name.
        package: String,
        /// Version that already exists.
        version: Version,
    },
    /// Package name is reserved.
    NameReserved(String),
}
/// An authentication token for registry operations.
#[derive(Clone, Debug)]
pub struct AuthToken {
    /// The token value.
    value: String,
    /// Token type (e.g., "Bearer").
    pub token_type: String,
    /// When the token expires (as a UNIX timestamp, 0 = never).
    pub expires_at: u64,
    /// Scopes granted by this token.
    pub scopes: Vec<String>,
}
impl AuthToken {
    /// Create a new bearer token.
    pub fn bearer(value: &str) -> Self {
        Self {
            value: value.to_string(),
            token_type: "Bearer".to_string(),
            expires_at: 0,
            scopes: Vec::new(),
        }
    }
    /// Create an API key token.
    pub fn api_key(value: &str) -> Self {
        Self {
            value: value.to_string(),
            token_type: "ApiKey".to_string(),
            expires_at: 0,
            scopes: Vec::new(),
        }
    }
    /// Get the authorization header value.
    pub fn auth_header(&self) -> String {
        format!("{} {}", self.token_type, self.value)
    }
    /// Check if the token is expired.
    pub fn is_expired(&self, current_time: u64) -> bool {
        if self.expires_at == 0 {
            false
        } else {
            current_time >= self.expires_at
        }
    }
    /// Check if the token has a specific scope.
    pub fn has_scope(&self, scope: &str) -> bool {
        self.scopes.iter().any(|s| s == scope)
    }
    /// Add a scope to the token.
    pub fn with_scope(mut self, scope: &str) -> Self {
        self.scopes.push(scope.to_string());
        self
    }
    /// Get the token value (redacted for display).
    pub fn redacted(&self) -> String {
        if self.value.len() <= 8 {
            "****".to_string()
        } else {
            format!(
                "{}...{}",
                &self.value[..4],
                &self.value[self.value.len() - 4..]
            )
        }
    }
}
/// Summary information about a package from the registry.
#[derive(Clone, Debug)]
pub struct PackageInfo {
    /// Package name.
    pub name: String,
    /// Latest version.
    pub latest_version: Version,
    /// All available versions.
    pub versions: Vec<VersionInfo>,
    /// Description.
    pub description: Option<String>,
    /// License.
    pub license: Option<String>,
    /// Repository URL.
    pub repository: Option<String>,
    /// Documentation URL.
    pub documentation: Option<String>,
    /// Download count.
    pub downloads: u64,
    /// Authors.
    pub authors: Vec<String>,
    /// Keywords.
    pub keywords: Vec<String>,
    /// Categories.
    pub categories: Vec<String>,
    /// Date of creation.
    pub created_at: String,
    /// Date of last update.
    pub updated_at: String,
}
/// Information about a specific version.
#[derive(Clone, Debug)]
pub struct VersionInfo {
    /// The version number.
    pub version: Version,
    /// Whether this version is yanked.
    pub yanked: bool,
    /// Download count.
    pub downloads: u64,
    /// Upload date.
    pub published_at: String,
    /// Checksum.
    pub checksum: String,
    /// Dependencies.
    pub dependencies: Vec<Dependency>,
    /// Required OxiLean version.
    pub min_oxilean_version: Option<String>,
    /// Package size in bytes.
    pub size: u64,
}
/// Validates a package before publishing.
pub struct PackageValidator {
    /// Validation errors found.
    errors: Vec<String>,
    /// Validation warnings found.
    warnings: Vec<String>,
}
impl PackageValidator {
    /// Create a new validator.
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    /// Validate a package for publishing.
    pub fn validate(
        &mut self,
        name: &str,
        version: &Version,
        description: Option<&str>,
        license: Option<&str>,
        authors: &[String],
    ) -> bool {
        self.errors.clear();
        self.warnings.clear();
        if name.is_empty() {
            self.errors.push("package name is empty".to_string());
        }
        if name.len() > 64 {
            self.errors
                .push("package name exceeds 64 characters".to_string());
        }
        if name.contains(' ') || name.contains('\t') {
            self.errors
                .push("package name contains whitespace".to_string());
        }
        let reserved = ["std", "core", "test", "oxilean", "system", "kernel"];
        if reserved.contains(&name) {
            self.errors
                .push(format!("package name '{}' is reserved", name));
        }
        if version.major == 0 && version.minor == 0 && version.patch == 0 {
            self.errors
                .push("version 0.0.0 is not publishable".to_string());
        }
        if description.is_none() || description == Some("") {
            self.warnings.push("missing description".to_string());
        }
        if license.is_none() || license == Some("") {
            self.warnings.push("missing license".to_string());
        }
        if authors.is_empty() {
            self.warnings.push("no authors specified".to_string());
        }
        self.errors.is_empty()
    }
    /// Get validation errors.
    pub fn errors(&self) -> &[String] {
        &self.errors
    }
    /// Get validation warnings.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
}
/// A request to download a package.
#[derive(Clone, Debug)]
pub struct DownloadRequest {
    /// Package name.
    pub name: String,
    /// Version to download.
    pub version: Version,
    /// Registry to download from.
    pub registry: String,
    /// Priority (lower = higher priority).
    pub priority: u32,
}
/// An in-memory index of all packages and versions in the registry.
pub struct PackageVersionIndex {
    /// package_name → sorted list of available versions.
    packages: std::collections::HashMap<String, Vec<crate::manifest::Version>>,
}
impl PackageVersionIndex {
    /// Create an empty index.
    pub fn new() -> Self {
        Self {
            packages: std::collections::HashMap::new(),
        }
    }
    /// Record a version for a package.
    pub fn add_version(&mut self, package: &str, version: crate::manifest::Version) {
        let entry = self.packages.entry(package.to_string()).or_default();
        if !entry.contains(&version) {
            entry.push(version);
        }
    }
    /// Number of packages in the index.
    pub fn package_count(&self) -> usize {
        self.packages.len()
    }
    /// Number of versions for a package.
    pub fn version_count(&self, package: &str) -> usize {
        self.packages.get(package).map(|v| v.len()).unwrap_or(0)
    }
    /// Whether a package is known.
    pub fn has_package(&self, package: &str) -> bool {
        self.packages.contains_key(package)
    }
    /// Whether a specific version is known.
    pub fn has_version(&self, package: &str, version: &crate::manifest::Version) -> bool {
        self.packages
            .get(package)
            .map(|v| v.contains(version))
            .unwrap_or(false)
    }
    /// All known package names.
    pub fn package_names(&self) -> Vec<&str> {
        self.packages.keys().map(|k| k.as_str()).collect()
    }
    /// Total number of package-version pairs.
    pub fn total_versions(&self) -> usize {
        self.packages.values().map(|v| v.len()).sum()
    }
}
/// An ordered list of registry mirrors.
pub struct MirrorList {
    mirrors: Vec<RegistryMirror>,
}
impl MirrorList {
    /// Create an empty list.
    pub fn new() -> Self {
        Self {
            mirrors: Vec::new(),
        }
    }
    /// Add a mirror.
    pub fn add(&mut self, mirror: RegistryMirror) {
        self.mirrors.push(mirror);
        self.mirrors.sort_by_key(|m| m.priority);
    }
    /// Active mirrors in priority order.
    pub fn active(&self) -> Vec<&RegistryMirror> {
        self.mirrors.iter().filter(|m| m.enabled).collect()
    }
    /// Number of mirrors.
    pub fn len(&self) -> usize {
        self.mirrors.len()
    }
    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.mirrors.is_empty()
    }
}
/// Additional metadata for a package in the registry.
#[derive(Clone, Debug, Default)]
pub struct PackageMetadata {
    /// Package description.
    pub description: Option<String>,
    /// Repository URL.
    pub repository: Option<String>,
    /// License identifier.
    pub license: Option<String>,
    /// List of keywords.
    pub keywords: Vec<String>,
    /// List of categories.
    pub categories: Vec<String>,
    /// Total download count.
    pub downloads: u64,
}
impl PackageMetadata {
    /// Create empty metadata.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set description.
    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
    /// Set license.
    pub fn with_license(mut self, license: &str) -> Self {
        self.license = Some(license.to_string());
        self
    }
    /// Add a keyword.
    pub fn with_keyword(mut self, kw: &str) -> Self {
        self.keywords.push(kw.to_string());
        self
    }
    /// Add a category.
    pub fn with_category(mut self, cat: &str) -> Self {
        self.categories.push(cat.to_string());
        self
    }
}
/// Persistent credential storage for registry authentication.
pub struct CredentialStore {
    /// Storage path.
    store_path: PathBuf,
    /// Credentials indexed by registry URL.
    credentials: HashMap<String, AuthToken>,
}
impl CredentialStore {
    /// Create a new credential store.
    pub fn new(store_path: &Path) -> Self {
        Self {
            store_path: store_path.to_path_buf(),
            credentials: HashMap::new(),
        }
    }
    /// Store a token for a registry.
    pub fn store(&mut self, registry_url: &str, token: AuthToken) {
        self.credentials.insert(registry_url.to_string(), token);
    }
    /// Retrieve a token for a registry.
    pub fn retrieve(&self, registry_url: &str) -> Option<&AuthToken> {
        self.credentials.get(registry_url)
    }
    /// Remove a token for a registry.
    pub fn remove(&mut self, registry_url: &str) -> Option<AuthToken> {
        self.credentials.remove(registry_url)
    }
    /// List all stored registry URLs.
    pub fn list_registries(&self) -> Vec<&str> {
        self.credentials.keys().map(|s| s.as_str()).collect()
    }
    /// Get the store path.
    pub fn store_path(&self) -> &Path {
        &self.store_path
    }
    /// Serialize credentials to a string in INI format.
    pub fn serialize(&self) -> String {
        let mut out = String::new();
        for (url, token) in &self.credentials {
            out.push_str(&format!(
                "[{}]\ntoken = {}\ntype = {}\n\n",
                url,
                token.redacted(),
                token.token_type
            ));
        }
        out
    }
    /// Get the number of stored credentials.
    pub fn count(&self) -> usize {
        self.credentials.len()
    }
    /// Clear all credentials.
    pub fn clear(&mut self) {
        self.credentials.clear();
    }
}
/// Configuration for a package registry.
#[derive(Clone, Debug)]
pub struct RegistryConfig {
    /// Registry name.
    pub name: String,
    /// Registry API URL.
    pub url: String,
    /// Download URL template.
    pub download_url: String,
    /// Whether this is the default registry.
    pub is_default: bool,
    /// Authentication token (if any).
    pub auth_token: Option<AuthToken>,
    /// Connection timeout in seconds.
    pub timeout_secs: u64,
    /// Maximum retries for network operations.
    pub max_retries: u32,
    /// Cache directory for downloaded packages.
    pub cache_dir: PathBuf,
}
impl RegistryConfig {
    /// Create a new registry config.
    pub fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.to_string(),
            url: url.to_string(),
            download_url: format!("{}/api/v1/crates", url),
            is_default: false,
            auth_token: None,
            timeout_secs: 30,
            max_retries: 3,
            cache_dir: PathBuf::from(".oxilean/registry-cache"),
        }
    }
    /// Create the default OxiLean registry config.
    pub fn default_registry() -> Self {
        let mut config = Self::new("oxilean", "https://registry.oxilean.dev");
        config.is_default = true;
        config
    }
    /// Set the authentication token.
    pub fn with_auth(mut self, token: AuthToken) -> Self {
        self.auth_token = Some(token);
        self
    }
    /// Set the cache directory.
    pub fn with_cache_dir(mut self, dir: &Path) -> Self {
        self.cache_dir = dir.to_path_buf();
        self
    }
    /// Check if authentication is configured.
    pub fn is_authenticated(&self) -> bool {
        self.auth_token.is_some()
    }
    /// Get the download URL for a specific package version.
    pub fn package_download_url(&self, name: &str, version: &Version) -> String {
        format!("{}/{}/{}/download", self.download_url, name, version)
    }
    /// Get the API URL for querying package info.
    pub fn package_info_url(&self, name: &str) -> String {
        format!("{}/api/v1/crates/{}", self.url, name)
    }
}
/// Packs a package directory into a distributable archive.
pub struct PackagePacker {
    /// Files to include in the archive.
    files: Vec<PathBuf>,
    /// Files to exclude.
    exclude_patterns: Vec<String>,
    /// Maximum archive size in bytes.
    max_size: u64,
}
impl PackagePacker {
    /// Create a new packer.
    pub fn new() -> Self {
        Self {
            files: Vec::new(),
            exclude_patterns: vec![
                ".git".to_string(),
                ".oxilean-cache".to_string(),
                "target".to_string(),
                "*.tmp".to_string(),
            ],
            max_size: 50 * 1024 * 1024,
        }
    }
    /// Add a file to the archive.
    pub fn add_file(&mut self, path: &Path) {
        self.files.push(path.to_path_buf());
    }
    /// Add an exclude pattern.
    pub fn add_exclude(&mut self, pattern: &str) {
        self.exclude_patterns.push(pattern.to_string());
    }
    /// Set the maximum archive size.
    pub fn set_max_size(&mut self, max_bytes: u64) {
        self.max_size = max_bytes;
    }
    /// Check if a file should be excluded.
    pub fn should_exclude(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        for pattern in &self.exclude_patterns {
            if let Some(suffix) = pattern.strip_prefix('*') {
                if path_str.ends_with(suffix) {
                    return true;
                }
            } else if path_str.contains(pattern.as_str()) {
                return true;
            }
        }
        false
    }
    /// Compute the total size of files to be packed.
    pub fn total_size(&self) -> u64 {
        self.files
            .iter()
            .filter_map(|f| std::fs::metadata(f).ok())
            .map(|m| m.len())
            .sum()
    }
    /// Check if the total size is within limits.
    pub fn check_size_limit(&self) -> Result<(), RegistryError> {
        let total = self.total_size();
        if total > self.max_size {
            return Err(RegistryError::InvalidPackage(format!(
                "package size {} bytes exceeds limit of {} bytes",
                total, self.max_size
            )));
        }
        Ok(())
    }
    /// Pack the included files and write a package manifest to `output_dir`.
    ///
    /// This creates a plain-text manifest file listing every included file
    /// together with its size and a checksum.  The manifest is named
    /// `<name>-<version>.manifest` and sits alongside the (not-yet-created)
    /// archive so downstream tooling can verify the package contents without
    /// extracting the full archive.
    ///
    /// A real implementation would additionally create a compressed tar archive
    /// (`<name>-<version>.tar.gz`), which requires an external crate such as
    /// `tar` + `flate2`.  The archive path is still returned so callers do not
    /// need to change when that feature is added.
    pub fn pack(
        &self,
        output_dir: &Path,
        name: &str,
        version: &Version,
    ) -> Result<PathBuf, RegistryError> {
        self.check_size_limit()?;
        let archive_name = format!("{}-{}.tar.gz", name, version);
        let archive_path = output_dir.join(&archive_name);
        let mut manifest_lines: Vec<String> = Vec::new();
        manifest_lines.push(format!("package: {}", name));
        manifest_lines.push(format!("version: {}", version));
        manifest_lines.push(format!("files: {}", self.files.len()));
        manifest_lines.push(String::new());
        for file in &self.files {
            if self.should_exclude(file) {
                continue;
            }
            let size = std::fs::metadata(file).map(|m| m.len()).unwrap_or(0);
            let checksum = std::fs::read(file)
                .map(|bytes| compute_bytes_checksum(&bytes))
                .unwrap_or_else(|_| "unavailable".to_string());
            manifest_lines.push(format!(
                "  {} ({} bytes, checksum: {})",
                file.display(),
                size,
                checksum
            ));
        }
        let manifest_path = output_dir.join(format!("{}-{}.manifest", name, version));
        std::fs::write(&manifest_path, manifest_lines.join("\n")).map_err(|e| {
            RegistryError::InvalidPackage(format!("failed to write manifest: {}", e))
        })?;
        Ok(archive_path)
    }
}
/// An entry in the registry index.
#[derive(Clone, Debug)]
pub struct RegistryIndexEntry {
    /// Package name.
    pub name: String,
    /// Available versions (sorted).
    pub versions: Vec<Version>,
    /// Latest non-yanked version.
    pub latest: Option<Version>,
    /// Yanked versions.
    pub yanked_versions: Vec<Version>,
}
/// The result of downloading a package.
#[derive(Clone, Debug)]
pub struct DownloadResult {
    /// Package name.
    pub name: String,
    /// Downloaded version.
    pub version: Version,
    /// Path to the downloaded archive.
    pub archive_path: PathBuf,
    /// Path to the extracted directory.
    pub extracted_path: PathBuf,
    /// Checksum of the downloaded file.
    pub checksum: String,
    /// Download size in bytes.
    pub size: u64,
}
/// The result of uploading a package.
#[derive(Clone, Debug)]
pub struct UploadResult {
    /// Package name.
    pub name: String,
    /// Uploaded version.
    pub version: Version,
    /// Registry URL for the published package.
    pub registry_url: String,
    /// Warnings from the upload.
    pub warnings: Vec<String>,
}
/// A queue of packages to download.
pub struct DownloadQueue {
    /// Pending downloads.
    pending: Vec<DownloadRequest>,
    /// Completed downloads.
    completed: Vec<DownloadResult>,
    /// Failed downloads.
    failed: Vec<(DownloadRequest, String)>,
    /// Maximum concurrent downloads.
    max_concurrent: usize,
}
impl DownloadQueue {
    /// Create a new download queue.
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            pending: Vec::new(),
            completed: Vec::new(),
            failed: Vec::new(),
            max_concurrent: max_concurrent.max(1),
        }
    }
    /// Enqueue a download request.
    pub fn enqueue(&mut self, request: DownloadRequest) {
        self.pending.push(request);
        self.pending.sort_by_key(|r| r.priority);
    }
    /// Get the next batch of downloads to process.
    pub fn next_batch(&mut self) -> Vec<DownloadRequest> {
        let count = self.max_concurrent.min(self.pending.len());
        self.pending.drain(..count).collect()
    }
    /// Mark a download as completed.
    pub fn mark_completed(&mut self, result: DownloadResult) {
        self.completed.push(result);
    }
    /// Mark a download as failed.
    pub fn mark_failed(&mut self, request: DownloadRequest, error: String) {
        self.failed.push((request, error));
    }
    /// Get the number of pending downloads.
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    /// Get the number of completed downloads.
    pub fn completed_count(&self) -> usize {
        self.completed.len()
    }
    /// Get the number of failed downloads.
    pub fn failed_count(&self) -> usize {
        self.failed.len()
    }
    /// Check if all downloads are complete.
    pub fn is_done(&self) -> bool {
        self.pending.is_empty()
    }
    /// Get all completed results.
    pub fn completed_results(&self) -> &[DownloadResult] {
        &self.completed
    }
}
/// Configuration for a registry mirror.
#[derive(Clone, Debug)]
pub struct RegistryMirror {
    /// Mirror URL.
    pub url: String,
    /// Priority (lower = higher priority).
    pub priority: u32,
    /// Whether the mirror is enabled.
    pub enabled: bool,
    /// Geographic region (informational).
    pub region: Option<String>,
}
impl RegistryMirror {
    /// Create a new mirror.
    pub fn new(url: &str, priority: u32) -> Self {
        Self {
            url: url.to_string(),
            priority,
            enabled: true,
            region: None,
        }
    }
    /// Set the region.
    pub fn with_region(mut self, region: &str) -> Self {
        self.region = Some(region.to_string());
        self
    }
    /// Disable this mirror.
    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
}
/// Events emitted by the registry client.
#[derive(Clone, Debug)]
pub enum RegistryNotification {
    /// A package was published.
    Published {
        package: String,
        version: crate::manifest::Version,
    },
    /// A version was yanked.
    Yanked {
        package: String,
        version: crate::manifest::Version,
    },
    /// A package was downloaded.
    Downloaded {
        package: String,
        version: crate::manifest::Version,
    },
    /// Registry connection failed.
    ConnectionFailed(String),
}
impl RegistryNotification {
    /// Short label.
    pub fn label(&self) -> &'static str {
        match self {
            RegistryNotification::Published { .. } => "published",
            RegistryNotification::Yanked { .. } => "yanked",
            RegistryNotification::Downloaded { .. } => "downloaded",
            RegistryNotification::ConnectionFailed(_) => "connection-failed",
        }
    }
}
/// Aggregate statistics for a registry session.
#[derive(Clone, Debug, Default)]
pub struct RegistryStats {
    /// Total packages available.
    pub total_packages: u64,
    /// Total versions available.
    pub total_versions: u64,
    /// Total downloads performed.
    pub total_downloads: u64,
    /// Total upload operations.
    pub total_uploads: u64,
    /// Cache hits on version queries.
    pub cache_hits: u64,
    /// Cache misses on version queries.
    pub cache_misses: u64,
}
impl RegistryStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Cache hit rate.
    pub fn hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
    /// Human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "packages={} versions={} downloads={} uploads={} hit_rate={:.1}%",
            self.total_packages,
            self.total_versions,
            self.total_downloads,
            self.total_uploads,
            self.hit_rate() * 100.0,
        )
    }
}
/// A record of a package download event.
#[derive(Clone, Debug)]
pub struct DownloadRecord {
    /// Package name.
    pub package: String,
    /// Version downloaded.
    pub version: crate::manifest::Version,
    /// Download timestamp (seconds).
    pub timestamp: u64,
    /// Size of the downloaded archive in bytes.
    pub size_bytes: u64,
    /// Whether the download succeeded.
    pub success: bool,
}
impl DownloadRecord {
    /// Create a successful download record.
    pub fn success(
        package: &str,
        version: crate::manifest::Version,
        timestamp: u64,
        size_bytes: u64,
    ) -> Self {
        Self {
            package: package.to_string(),
            version,
            timestamp,
            size_bytes,
            success: true,
        }
    }
    /// Create a failed download record.
    pub fn failure(package: &str, version: crate::manifest::Version, timestamp: u64) -> Self {
        Self {
            package: package.to_string(),
            version,
            timestamp,
            size_bytes: 0,
            success: false,
        }
    }
}
