//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use std::path::PathBuf;

use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigChangeKind {
    Created,
    Modified,
    Deleted,
}
/// Cache configuration controlling build caches.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CacheConfig {
    /// Whether caching is enabled.
    pub enabled: bool,
    /// Directory to store compiled object files.
    pub cache_dir: PathBuf,
    /// Maximum cache size in bytes (0 = unlimited).
    pub max_size: u64,
    /// Time-to-live for cache entries in seconds (0 = forever).
    pub ttl: u64,
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct ConfigStats {
    pub total_keys: usize,
    pub secret_keys: usize,
    pub sections: std::collections::HashSet<String>,
    pub longest_key: usize,
    pub longest_value: usize,
}
#[allow(dead_code)]
pub struct ConfigMigrator {
    pub steps: Vec<ConfigMigrationStep>,
}
#[allow(dead_code)]
impl ConfigMigrator {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }
    pub fn add_step(&mut self, step: ConfigMigrationStep) {
        self.steps.push(step);
    }
    pub fn migrate(
        &self,
        map: &mut std::collections::HashMap<String, String>,
        from: u32,
        to: u32,
    ) -> Vec<String> {
        let mut log = Vec::new();
        let mut current = from;
        while current < to {
            if let Some(step) = self.steps.iter().find(|s| s.from_version == current) {
                (step.apply)(map);
                log.push(format!(
                    "migrated v{} -> v{}: {}",
                    step.from_version, step.to_version, step.description
                ));
                current = step.to_version;
            } else {
                log.push(format!("no migration step from v{}", current));
                break;
            }
        }
        log
    }
}
#[allow(dead_code)]
pub struct ConfigPresetMap {
    pub name: String,
    pub description: String,
    pub values: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl ConfigPresetMap {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            values: std::collections::HashMap::new(),
        }
    }
    pub fn set(mut self, key: &str, val: &str) -> Self {
        self.values.insert(key.to_string(), val.to_string());
        self
    }
}
#[allow(dead_code)]
pub struct ConfigMigrationStep {
    pub from_version: u32,
    pub to_version: u32,
    pub description: String,
    pub apply: fn(&mut std::collections::HashMap<String, String>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConfigAnnotation {
    pub key: String,
    pub tags: Vec<String>,
    pub example: Option<String>,
    pub since_version: Option<String>,
}
#[allow(dead_code)]
impl ConfigAnnotation {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            tags: Vec::new(),
            example: None,
            since_version: None,
        }
    }
    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    pub fn example(mut self, example: &str) -> Self {
        self.example = Some(example.to_string());
        self
    }
    pub fn since(mut self, version: &str) -> Self {
        self.since_version = Some(version.to_string());
        self
    }
    pub fn to_doc_string(&self) -> String {
        let mut doc = format!("Key: {}", self.key);
        if !self.tags.is_empty() {
            doc.push_str(&format!(" [{}]", self.tags.join(", ")));
        }
        if let Some(ex) = &self.example {
            doc.push_str(&format!(" (example: {})", ex));
        }
        if let Some(ver) = &self.since_version {
            doc.push_str(&format!(" (since v{})", ver));
        }
        doc
    }
}
/// Proof checking strategy for the elaborator.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum ProofCheckMode {
    /// Full kernel-level proof checking (slowest, most reliable).
    #[default]
    Full,
    /// Trusted elaboration with spot checks.
    Trusted,
    /// Skip proof checking (for drafting; sorry allowed).
    Skip,
    /// Parallel proof checking.
    Parallel { threads: usize },
}
/// Builder for Config.
#[allow(dead_code)]
pub struct ConfigBuilder {
    config: Config,
}
impl ConfigBuilder {
    /// Create a new config builder.
    pub fn new() -> Self {
        Self {
            config: Config::new(),
        }
    }
    /// Add a library path.
    #[allow(dead_code)]
    pub fn library_path(mut self, path: PathBuf) -> Self {
        self.config.add_library_path(path);
        self
    }
    /// Set verbosity level.
    #[allow(dead_code)]
    pub fn verbosity(mut self, level: u8) -> Self {
        self.config.set_verbosity(level);
        self
    }
    /// Enable/disable color.
    #[allow(dead_code)]
    pub fn color(mut self, enabled: bool) -> Self {
        self.config.set_color(enabled);
        self
    }
    /// Enable/disable unicode.
    #[allow(dead_code)]
    pub fn unicode(mut self, enabled: bool) -> Self {
        self.config.set_unicode(enabled);
        self
    }
    /// Set max errors.
    #[allow(dead_code)]
    pub fn max_errors(mut self, max: usize) -> Self {
        self.config.set_max_errors(max);
        self
    }
    /// Set working directory.
    #[allow(dead_code)]
    pub fn working_dir(mut self, dir: PathBuf) -> Self {
        self.config.working_dir = dir;
        self
    }
    /// Enable experimental features.
    #[allow(dead_code)]
    pub fn experimental(mut self, enabled: bool) -> Self {
        self.config.experimental = enabled;
        self
    }
    /// Set timeout.
    #[allow(dead_code)]
    pub fn timeout(mut self, seconds: Option<u64>) -> Self {
        self.config.timeout = seconds;
        self
    }
    /// Build the configuration.
    #[allow(dead_code)]
    pub fn build(self) -> Config {
        self.config
    }
}
/// A profile preset combining multiple config settings.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ConfigPreset {
    /// Name of the preset.
    pub name: String,
    /// Description.
    pub description: String,
    /// The configuration values.
    pub config: Config,
}
impl ConfigPreset {
    /// The "debug" preset: verbose, full checks.
    pub fn debug() -> Self {
        let mut c = Config::new();
        c.set_verbosity(3);
        c.color = true;
        c.unicode = true;
        c.experimental = true;
        Self {
            name: "debug".to_string(),
            description: "Full debugging with verbose output".to_string(),
            config: c,
        }
    }
    /// The "ci" preset: quiet, no color.
    pub fn ci() -> Self {
        let mut c = Config::new();
        c.set_verbosity(0);
        c.color = false;
        c.unicode = false;
        Self {
            name: "ci".to_string(),
            description: "Continuous integration mode".to_string(),
            config: c,
        }
    }
    /// The "release" preset: balanced and performant.
    pub fn release() -> Self {
        let mut c = Config::new();
        c.set_verbosity(1);
        c.experimental = false;
        Self {
            name: "release".to_string(),
            description: "Release build settings".to_string(),
            config: c,
        }
    }
}
/// Server configuration for the language server protocol (LSP).
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct LspConfig {
    /// Port to listen on (default: stdio).
    pub port: Option<u16>,
    /// Enable incremental document syncing.
    pub incremental_sync: bool,
    /// Maximum number of completions to return.
    pub max_completions: usize,
    /// Enable hover documentation.
    pub hover: bool,
    /// Enable go-to-definition.
    pub definition: bool,
    /// Enable workspace symbol search.
    pub workspace_symbols: bool,
}
#[allow(dead_code)]
pub struct ConfigHistory {
    pub(crate) entries: Vec<ConfigHistoryEntry>,
    max_entries: usize,
}
#[allow(dead_code)]
impl ConfigHistory {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }
    pub fn record(&mut self, key: &str, old: Option<&str>, new: Option<&str>) {
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(ConfigHistoryEntry {
            key: key.to_string(),
            old_value: old.map(|s| s.to_string()),
            new_value: new.map(|s| s.to_string()),
            timestamp: std::time::Instant::now(),
        });
    }
    pub fn last_n(&self, n: usize) -> &[ConfigHistoryEntry] {
        let start = self.entries.len().saturating_sub(n);
        &self.entries[start..]
    }
    pub fn undo_last(&self) -> Option<(&str, Option<&str>)> {
        self.entries
            .last()
            .map(|e| (e.key.as_str(), e.old_value.as_deref()))
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
#[allow(dead_code)]
pub struct ConfigComparison {
    pub added: Vec<(String, String)>,
    pub removed: Vec<String>,
    pub changed: Vec<(String, String, String)>,
    pub unchanged: Vec<String>,
}
/// Performance profile for compilation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum PerfProfile {
    /// Optimize for fast compilation.
    Fast,
    /// Optimize for correctness and completeness.
    Thorough,
    /// Balanced compile/check speed.
    #[default]
    Balanced,
}
#[allow(dead_code)]
pub struct ConfigLock {
    lock_path: std::path::PathBuf,
}
#[allow(dead_code)]
impl ConfigLock {
    pub fn new(config_path: &std::path::Path) -> Self {
        let lock_path = config_path.with_extension("lock");
        Self { lock_path }
    }
    pub fn try_acquire(&self) -> bool {
        if self.lock_path.exists() {
            return false;
        }
        std::fs::write(&self.lock_path, std::process::id().to_string()).is_ok()
    }
    pub fn release(&self) {
        let _ = std::fs::remove_file(&self.lock_path);
    }
    pub fn is_locked(&self) -> bool {
        self.lock_path.exists()
    }
}
/// A session configuration — config that can be modified interactively.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Underlying config.
    pub config: Config,
    /// Session-local overrides.
    pub overrides: HashMap<String, String>,
    /// Whether the config was modified since last save.
    pub dirty: bool,
}
impl SessionConfig {
    /// Create a new session config from a base config.
    #[allow(dead_code)]
    pub fn new(config: Config) -> Self {
        Self {
            config,
            overrides: HashMap::new(),
            dirty: false,
        }
    }
    /// Set a session-local override.
    #[allow(dead_code)]
    pub fn set_override(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.overrides.insert(key.into(), value.into());
        self.dirty = true;
    }
    /// Get an override value.
    #[allow(dead_code)]
    pub fn get_override(&self, key: &str) -> Option<&String> {
        self.overrides.get(key)
    }
    /// Apply all overrides to the config (consuming overrides).
    #[allow(dead_code)]
    pub fn apply_overrides(&mut self) {
        let overrides = std::mem::take(&mut self.overrides);
        for (k, v) in &overrides {
            match k.as_str() {
                "verbosity" => {
                    if let Ok(n) = v.parse::<u8>() {
                        self.config.set_verbosity(n);
                    }
                }
                "color" => self.config.color = v == "true",
                "unicode" => self.config.unicode = v == "true",
                "max_errors" => {
                    if let Ok(n) = v.parse::<usize>() {
                        self.config.set_max_errors(n);
                    }
                }
                _ => {
                    self.config.set_custom(k.clone(), v.clone());
                }
            }
        }
        self.dirty = false;
    }
    /// Mark the config as saved (clear dirty flag).
    #[allow(dead_code)]
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }
    /// Number of active overrides.
    #[allow(dead_code)]
    pub fn num_overrides(&self) -> usize {
        self.overrides.len()
    }
}
#[allow(dead_code)]
pub struct ConfigAliases {
    aliases: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl ConfigAliases {
    pub fn new() -> Self {
        let mut aliases = std::collections::HashMap::new();
        aliases.insert("j".into(), "build.jobs".into());
        aliases.insert("out".into(), "build.output_dir".into());
        aliases.insert("backend".into(), "codegen.backend".into());
        Self { aliases }
    }
    pub fn resolve<'a>(&'a self, key: &'a str) -> &'a str {
        self.aliases.get(key).map(|s| s.as_str()).unwrap_or(key)
    }
    pub fn add(&mut self, alias: &str, canonical: &str) {
        self.aliases
            .insert(alias.to_string(), canonical.to_string());
    }
}
/// A watch list of changed configuration keys between two configs.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ConfigChanges {
    /// Fields that changed.
    pub changed_fields: Vec<String>,
}
impl ConfigChanges {
    /// Compute which fields changed between `old` and `new`.
    #[allow(dead_code)]
    pub fn diff(old: &Config, new: &Config) -> Self {
        let mut changed = Vec::new();
        if old.verbosity != new.verbosity {
            changed.push("verbosity".to_string());
        }
        if old.color != new.color {
            changed.push("color".to_string());
        }
        if old.unicode != new.unicode {
            changed.push("unicode".to_string());
        }
        if old.max_errors != new.max_errors {
            changed.push("max_errors".to_string());
        }
        if old.experimental != new.experimental {
            changed.push("experimental".to_string());
        }
        if old.timeout != new.timeout {
            changed.push("timeout".to_string());
        }
        Self {
            changed_fields: changed,
        }
    }
    /// Check whether any field changed.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.changed_fields.is_empty()
    }
    /// Number of changed fields.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.changed_fields.len()
    }
    /// Check if a specific field changed.
    #[allow(dead_code)]
    pub fn field_changed(&self, field: &str) -> bool {
        self.changed_fields.iter().any(|f| f == field)
    }
}
/// Lint severity levels for code checking.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
#[allow(dead_code)]
pub enum LintSeverity {
    /// Informational hints only.
    Hint,
    /// Warnings (default lint level).
    #[default]
    Warning,
    /// Errors that block compilation.
    Error,
    /// All lint checks suppressed.
    Off,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConfigSchemaEntry {
    pub key: String,
    pub value_type: ConfigValueType,
    pub required: bool,
    pub default: Option<String>,
    pub description: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigValueType {
    Bool,
    Integer,
    Float,
    String,
    StringList,
    Path,
}
/// Output format for OxiLean CLI results.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[allow(dead_code)]
pub enum OutputFormat {
    /// Human-readable text output.
    #[default]
    Text,
    /// JSON output for tooling integration.
    Json,
    /// S-expression output.
    Sexp,
    /// Compact machine-readable format.
    Machine,
}
/// Extended configuration for advanced use cases.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ExtendedConfig {
    /// Base configuration.
    pub base: Config,
    /// Output format.
    pub output_format: OutputFormat,
    /// Lint severity threshold.
    pub lint_severity: LintSeverity,
    /// Proof checking mode.
    pub proof_check: ProofCheckMode,
    /// Cache configuration.
    pub cache: CacheConfig,
    /// LSP configuration.
    pub lsp: LspConfig,
    /// Performance profile.
    pub perf_profile: PerfProfile,
    /// Import paths for import commands.
    pub import_paths: Vec<PathBuf>,
    /// Extra preprocessor defines passed via -D.
    pub defines: HashMap<String, String>,
    /// Whether to emit term-mode proof objects.
    pub emit_proofs: bool,
    /// Number of worker threads for parallel elaboration.
    pub num_threads: usize,
    /// Maximum recursion depth for the elaborator.
    pub max_recursion: usize,
    /// Maximum universe level for auto-bound implicit universes.
    pub max_universe: usize,
}
impl ExtendedConfig {
    /// Create a new extended configuration with defaults.
    pub fn new() -> Self {
        Self {
            base: Config::new(),
            output_format: OutputFormat::default(),
            lint_severity: LintSeverity::default(),
            proof_check: ProofCheckMode::default(),
            cache: CacheConfig::default(),
            lsp: LspConfig::default(),
            perf_profile: PerfProfile::default(),
            import_paths: Vec::new(),
            defines: HashMap::new(),
            emit_proofs: false,
            num_threads: num_cpus(),
            max_recursion: 4096,
            max_universe: 32,
        }
    }
    /// Enable or disable caching.
    pub fn with_cache(mut self, enabled: bool) -> Self {
        self.cache.enabled = enabled;
        self
    }
    /// Set the output format.
    pub fn with_output_format(mut self, fmt: OutputFormat) -> Self {
        self.output_format = fmt;
        self
    }
    /// Set the proof check mode.
    pub fn with_proof_check(mut self, mode: ProofCheckMode) -> Self {
        self.proof_check = mode;
        self
    }
    /// Set the number of worker threads.
    pub fn with_threads(mut self, n: usize) -> Self {
        self.num_threads = n.max(1);
        self
    }
    /// Set a preprocessor define.
    pub fn define(mut self, key: String, val: String) -> Self {
        self.defines.insert(key, val);
        self
    }
    /// Add an import path.
    pub fn add_import_path(&mut self, path: PathBuf) {
        if !self.import_paths.contains(&path) {
            self.import_paths.push(path);
        }
    }
    /// Check whether sorry is allowed.
    pub fn sorry_allowed(&self) -> bool {
        self.proof_check == ProofCheckMode::Skip
    }
    /// Get the effective number of threads, clamped to a sane value.
    pub fn effective_threads(&self) -> usize {
        self.num_threads.clamp(1, 256)
    }
    /// Merge another config on top of this one (other wins on conflicts).
    pub fn merge(&mut self, other: &ExtendedConfig) {
        for (k, v) in &other.defines {
            self.defines.insert(k.clone(), v.clone());
        }
        for p in &other.import_paths {
            self.add_import_path(p.clone());
        }
        if other.base.verbosity != 1 {
            self.base.verbosity = other.base.verbosity;
        }
        if !other.base.color {
            self.base.color = false;
        }
        if !other.base.unicode {
            self.base.unicode = false;
        }
    }
    /// Validate the configuration, returning a list of warnings.
    pub fn validate(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        if self.num_threads == 0 {
            warnings.push("num_threads is 0; effective threads will be clamped to 1".to_string());
        }
        if self.max_recursion < 128 {
            warnings.push(format!(
                "max_recursion {} is very small; may cause stack overflows",
                self.max_recursion
            ));
        }
        if self.sorry_allowed() {
            warnings.push("proof checking is disabled (sorry mode)".to_string());
        }
        warnings
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConfigHistoryEntry {
    pub key: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub timestamp: std::time::Instant,
}
#[allow(dead_code)]
pub struct ConfigWatcher {
    path: std::path::PathBuf,
    last_modified: Option<std::time::SystemTime>,
}
#[allow(dead_code)]
impl ConfigWatcher {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self {
            path,
            last_modified: None,
        }
    }
    pub fn poll(&mut self) -> Option<ConfigChangeEvent> {
        let meta = std::fs::metadata(&self.path);
        match meta {
            Ok(m) => {
                let mtime = m.modified().ok();
                if mtime != self.last_modified {
                    let kind = if self.last_modified.is_none() {
                        ConfigChangeKind::Created
                    } else {
                        ConfigChangeKind::Modified
                    };
                    self.last_modified = mtime;
                    Some(ConfigChangeEvent {
                        path: self.path.clone(),
                        kind,
                        detected_at: std::time::Instant::now(),
                    })
                } else {
                    None
                }
            }
            Err(_) => {
                if self.last_modified.is_some() {
                    self.last_modified = None;
                    Some(ConfigChangeEvent {
                        path: self.path.clone(),
                        kind: ConfigChangeKind::Deleted,
                        detected_at: std::time::Instant::now(),
                    })
                } else {
                    None
                }
            }
        }
    }
}
/// Configuration validator: checks for common issues.
#[allow(dead_code)]
pub struct ConfigValidator;
impl ConfigValidator {
    /// Validate a `Config` and return a list of warning messages.
    #[allow(dead_code)]
    pub fn validate(config: &Config) -> Vec<String> {
        let mut warnings = Vec::new();
        if config.max_errors == 0 {
            warnings.push("max_errors is 0: no errors will be displayed".to_string());
        }
        if config.library_path.is_empty() {
            warnings.push("library_path is empty: no libraries will be found".to_string());
        }
        if config.verbosity > 3 {
            warnings.push(format!(
                "verbosity {} is above maximum (3)",
                config.verbosity
            ));
        }
        if let Some(0) = config.timeout {
            warnings.push("timeout is 0 seconds: operations will time out immediately".to_string());
        }
        warnings
    }
    /// Validate an `ExtendedConfig`.
    #[allow(dead_code)]
    pub fn validate_extended(config: &ExtendedConfig) -> Vec<String> {
        let mut warnings = Self::validate(&config.base);
        warnings.extend(config.validate());
        warnings
    }
}
/// A config that supports layered merging (base, project, user, session).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LayeredConfig {
    /// Base (system-wide) config.
    pub base: Config,
    /// Project config.
    pub project: Option<Config>,
    /// User config.
    pub user: Option<Config>,
    /// Session config.
    pub session: Option<Config>,
}
impl LayeredConfig {
    /// Create a new layered config with only a base.
    #[allow(dead_code)]
    pub fn new(base: Config) -> Self {
        Self {
            base,
            project: None,
            user: None,
            session: None,
        }
    }
    /// Set the project config.
    #[allow(dead_code)]
    pub fn with_project(mut self, config: Config) -> Self {
        self.project = Some(config);
        self
    }
    /// Set the user config.
    #[allow(dead_code)]
    pub fn with_user(mut self, config: Config) -> Self {
        self.user = Some(config);
        self
    }
    /// Set the session config.
    #[allow(dead_code)]
    pub fn with_session(mut self, config: Config) -> Self {
        self.session = Some(config);
        self
    }
    /// Resolve all layers into a single merged config.
    ///
    /// Priority: session > user > project > base.
    #[allow(dead_code)]
    pub fn resolve(&self) -> Config {
        let mut configs = vec![self.base.clone()];
        if let Some(p) = &self.project {
            configs.push(p.clone());
        }
        if let Some(u) = &self.user {
            configs.push(u.clone());
        }
        if let Some(s) = &self.session {
            configs.push(s.clone());
        }
        merge_configs(&configs)
    }
    /// Count the number of active layers.
    #[allow(dead_code)]
    pub fn layer_count(&self) -> usize {
        1 + self.project.is_some() as usize
            + self.user.is_some() as usize
            + self.session.is_some() as usize
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ConfigChangeEvent {
    pub path: std::path::PathBuf,
    pub kind: ConfigChangeKind,
    pub detected_at: std::time::Instant,
}
#[allow(dead_code)]
pub struct ConfigSchema {
    pub entries: Vec<ConfigSchemaEntry>,
}
#[allow(dead_code)]
impl ConfigSchema {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    pub fn add_entry(&mut self, entry: ConfigSchemaEntry) {
        self.entries.push(entry);
    }
    pub fn required_keys(&self) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|e| e.required)
            .map(|e| e.key.as_str())
            .collect()
    }
    pub fn validate_map(&self, map: &std::collections::HashMap<String, String>) -> Vec<String> {
        let mut errors = Vec::new();
        for entry in &self.entries {
            if entry.required && !map.contains_key(&entry.key) {
                errors.push(format!("missing required key: {}", entry.key));
            }
        }
        for key in map.keys() {
            if !self.entries.iter().any(|e| &e.key == key) {
                errors.push(format!("unknown config key: {}", key));
            }
        }
        errors
    }
}
/// Configuration for the OxiLean CLI.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Config {
    /// Path to search for libraries
    pub library_path: Vec<PathBuf>,
    /// Verbosity level (0 = quiet, 1 = normal, 2 = verbose, 3 = debug)
    pub verbosity: u8,
    /// Enable/disable color output
    pub color: bool,
    /// Enable/disable unicode output
    pub unicode: bool,
    /// Maximum number of errors to display
    pub max_errors: usize,
    /// Custom settings
    pub custom: HashMap<String, String>,
    /// Working directory
    pub working_dir: PathBuf,
    /// Enable experimental features
    pub experimental: bool,
    /// Timeout for operations (in seconds)
    pub timeout: Option<u64>,
}
impl Config {
    /// Create a new default configuration.
    pub fn new() -> Self {
        Self {
            library_path: vec![PathBuf::from("./lib"), PathBuf::from("/usr/lib/oxilean")],
            verbosity: 1,
            color: true,
            unicode: true,
            max_errors: 10,
            custom: HashMap::new(),
            working_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            experimental: false,
            timeout: Some(300),
        }
    }
    /// Load configuration from a file.
    ///
    /// Reads a simple TOML-style config file (key = value pairs, one per line).
    /// Lines starting with `#` are treated as comments and ignored.
    /// Returns `Err` if the file cannot be read.
    #[allow(dead_code)]
    pub fn load_from_file(path: &PathBuf) -> Result<Self, String> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file {}: {}", path.display(), e))?;
        let mut cfg = Self::new();
        let mut library_paths_from_file: Vec<PathBuf> = Vec::new();
        let mut in_custom_section = false;
        for raw_line in text.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            if line.starts_with('[') && line.ends_with(']') {
                let section = &line[1..line.len() - 1];
                in_custom_section = section == "custom";
                continue;
            }
            let Some(eq_pos) = line.find('=') else {
                continue;
            };
            let key = line[..eq_pos].trim();
            let raw_val = line[eq_pos + 1..].trim();
            let value = if (raw_val.starts_with('"') && raw_val.ends_with('"'))
                || (raw_val.starts_with('\'') && raw_val.ends_with('\''))
            {
                &raw_val[1..raw_val.len() - 1]
            } else {
                raw_val
            };
            if in_custom_section {
                cfg.custom.insert(key.to_string(), value.to_string());
                continue;
            }
            match key {
                "verbosity" => {
                    if let Ok(v) = value.parse::<u8>() {
                        cfg.verbosity = v.min(3);
                    }
                }
                "color" => {
                    cfg.color = value == "true" || value == "1";
                }
                "unicode" => {
                    cfg.unicode = value == "true" || value == "1";
                }
                "max_errors" => {
                    if let Ok(v) = value.parse::<usize>() {
                        cfg.max_errors = v;
                    }
                }
                "working_dir" => {
                    cfg.working_dir = PathBuf::from(value);
                }
                "experimental" => {
                    cfg.experimental = value == "true" || value == "1";
                }
                "timeout" => {
                    if value == "none" || value == "null" || value.is_empty() {
                        cfg.timeout = None;
                    } else if let Ok(v) = value.parse::<u64>() {
                        cfg.timeout = Some(v);
                    }
                }
                "library_path" => {
                    for p in value.split(',') {
                        let trimmed = p.trim();
                        if !trimmed.is_empty() {
                            library_paths_from_file.push(PathBuf::from(trimmed));
                        }
                    }
                }
                _ => {}
            }
        }
        if !library_paths_from_file.is_empty() {
            cfg.library_path = library_paths_from_file;
        }
        Ok(cfg)
    }
    /// Save configuration to a file in simple TOML format.
    ///
    /// Returns `Err` if the file cannot be written.
    #[allow(dead_code)]
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), String> {
        let mut lines: Vec<String> = Vec::new();
        lines.push("# OxiLean configuration file".to_string());
        lines.push(String::new());
        let lib_paths: Vec<String> = self
            .library_path
            .iter()
            .map(|p| p.display().to_string())
            .collect();
        lines.push(format!("library_path = \"{}\"", lib_paths.join(",")));
        lines.push(format!("verbosity = {}", self.verbosity));
        lines.push(format!("color = {}", self.color));
        lines.push(format!("unicode = {}", self.unicode));
        lines.push(format!("max_errors = {}", self.max_errors));
        lines.push(format!("working_dir = \"{}\"", self.working_dir.display()));
        lines.push(format!("experimental = {}", self.experimental));
        match self.timeout {
            Some(t) => lines.push(format!("timeout = {t}")),
            None => lines.push("timeout = none".to_string()),
        }
        if !self.custom.is_empty() {
            lines.push(String::new());
            lines.push("[custom]".to_string());
            let mut pairs: Vec<(&String, &String)> = self.custom.iter().collect();
            pairs.sort_by_key(|(k, _)| k.as_str());
            for (k, v) in pairs {
                lines.push(format!("{k} = \"{v}\""));
            }
        }
        lines.push(String::new());
        let content = lines.join("\n");
        std::fs::write(path, content)
            .map_err(|e| format!("Failed to write config file {}: {}", path.display(), e))
    }
    /// Add a library path.
    pub fn add_library_path(&mut self, path: PathBuf) {
        if !self.library_path.contains(&path) {
            self.library_path.push(path);
        }
    }
    /// Set verbosity level.
    #[allow(dead_code)]
    pub fn set_verbosity(&mut self, level: u8) {
        self.verbosity = level.min(3);
    }
    /// Enable/disable color output.
    #[allow(dead_code)]
    pub fn set_color(&mut self, enabled: bool) {
        self.color = enabled;
    }
    /// Enable/disable unicode output.
    #[allow(dead_code)]
    pub fn set_unicode(&mut self, enabled: bool) {
        self.unicode = enabled;
    }
    /// Set maximum number of errors to display.
    #[allow(dead_code)]
    pub fn set_max_errors(&mut self, max: usize) {
        self.max_errors = max;
    }
    /// Get a custom setting.
    #[allow(dead_code)]
    pub fn get_custom(&self, key: &str) -> Option<&String> {
        self.custom.get(key)
    }
    /// Set a custom setting.
    #[allow(dead_code)]
    pub fn set_custom(&mut self, key: String, value: String) {
        self.custom.insert(key, value);
    }
    /// Check if verbose mode is enabled.
    #[allow(dead_code)]
    pub fn is_verbose(&self) -> bool {
        self.verbosity >= 2
    }
    /// Check if debug mode is enabled.
    #[allow(dead_code)]
    pub fn is_debug(&self) -> bool {
        self.verbosity >= 3
    }
    /// Check if quiet mode is enabled.
    #[allow(dead_code)]
    pub fn is_quiet(&self) -> bool {
        self.verbosity == 0
    }
}
