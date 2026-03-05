//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::framework::{
    LintCategory, LintContext, LintDiagnostic, LintId, LintRule, Severity, SourceRange,
};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use super::functions::LintPlugin;

/// Lightweight statistics collected for a single plugin during a lint run.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PluginRunStats {
    /// Total number of diagnostics emitted by this plugin.
    pub diagnostics_emitted: usize,
    /// Total number of files visited.
    pub files_visited: usize,
    /// Whether the plugin encountered a fatal error during the run.
    pub had_fatal_error: bool,
    /// Accumulated milliseconds of CPU time (approximate).
    pub cpu_ms: u64,
}
#[allow(dead_code)]
impl PluginRunStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Merge another stats record into this one.
    pub fn merge(&mut self, other: &PluginRunStats) {
        self.diagnostics_emitted += other.diagnostics_emitted;
        self.files_visited += other.files_visited;
        self.had_fatal_error |= other.had_fatal_error;
        self.cpu_ms += other.cpu_ms;
    }
    /// Return the average diagnostics per file, or 0.0 if no files visited.
    pub fn avg_diagnostics_per_file(&self) -> f64 {
        if self.files_visited == 0 {
            0.0
        } else {
            self.diagnostics_emitted as f64 / self.files_visited as f64
        }
    }
}
/// A simple semantic version triple (major, minor, patch).
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PluginVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}
#[allow(dead_code)]
impl PluginVersion {
    /// Create a new plugin version.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
    /// Parse a version string of the form `"major.minor.patch"`.
    ///
    /// Returns `None` if the string is malformed.
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;
        Some(Self {
            major,
            minor,
            patch,
        })
    }
    /// Return `true` if this version is API-compatible with `required`.
    ///
    /// Compatibility means same major version and `self >= required`.
    pub fn is_compatible_with(&self, required: &PluginVersion) -> bool {
        self.major == required.major && self >= required
    }
    /// Format the version as a `"major.minor.patch"` string.
    pub fn to_string_repr(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
/// A catalog of known plugins with their manifests.
#[derive(Default)]
pub struct PluginCatalog {
    entries: HashMap<String, PluginManifest>,
}
impl PluginCatalog {
    /// Create an empty catalog.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a manifest to the catalog.
    pub fn add(&mut self, manifest: PluginManifest) {
        self.entries.insert(manifest.name.clone(), manifest);
    }
    /// Look up a manifest by plugin name.
    pub fn get(&self, name: &str) -> Option<&PluginManifest> {
        self.entries.get(name)
    }
    /// Remove a manifest from the catalog.
    pub fn remove(&mut self, name: &str) -> Option<PluginManifest> {
        self.entries.remove(name)
    }
    /// Return all catalog entries.
    pub fn all(&self) -> Vec<&PluginManifest> {
        self.entries.values().collect()
    }
    /// Return the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return `true` if the catalog is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Search for plugins whose name contains `query`.
    pub fn search(&self, query: &str) -> Vec<&PluginManifest> {
        self.entries
            .values()
            .filter(|m| m.name.contains(query) || m.description.contains(query))
            .collect()
    }
}
/// Records pairwise compatibility between plugins.
///
/// When two plugins declare incompatibility, the registry can warn or refuse
/// to load both simultaneously.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PluginCompatibilityMatrix {
    /// Set of incompatible plugin name pairs (stored in sorted order).
    incompatible: HashSet<(String, String)>,
    /// Set of plugin pairs that require each other.
    required_pairs: HashSet<(String, String)>,
}
#[allow(dead_code)]
impl PluginCompatibilityMatrix {
    /// Create a new, empty compatibility matrix.
    pub fn new() -> Self {
        Self::default()
    }
    fn sorted_pair(a: &str, b: &str) -> (String, String) {
        if a <= b {
            (a.to_string(), b.to_string())
        } else {
            (b.to_string(), a.to_string())
        }
    }
    /// Declare that plugins `a` and `b` are incompatible.
    pub fn declare_incompatible(&mut self, a: &str, b: &str) {
        self.incompatible.insert(Self::sorted_pair(a, b));
    }
    /// Declare that plugin `a` requires plugin `b` to be present.
    pub fn declare_requires(&mut self, a: &str, b: &str) {
        self.required_pairs.insert(Self::sorted_pair(a, b));
    }
    /// Check whether two plugins are incompatible with each other.
    pub fn are_incompatible(&self, a: &str, b: &str) -> bool {
        self.incompatible.contains(&Self::sorted_pair(a, b))
    }
    /// Check whether the pair `(a, b)` is a declared requirement.
    pub fn are_required_together(&self, a: &str, b: &str) -> bool {
        self.required_pairs.contains(&Self::sorted_pair(a, b))
    }
    /// Validate a set of active plugin names against this matrix.
    ///
    /// Returns a list of conflict descriptions.
    pub fn validate(&self, active: &[&str]) -> Vec<String> {
        let mut conflicts = Vec::new();
        for i in 0..active.len() {
            for j in (i + 1)..active.len() {
                if self.are_incompatible(active[i], active[j]) {
                    conflicts.push(format!(
                        "plugins '{}' and '{}' are incompatible",
                        active[i], active[j]
                    ));
                }
            }
        }
        conflicts
    }
}
/// Configuration for discovering plugins in the filesystem.
#[derive(Clone, Debug)]
pub struct PluginDiscoveryConfig {
    /// Directories to search for plugin manifests.
    pub search_dirs: Vec<PathBuf>,
    /// File extensions to look for (e.g., `"oxplugin"`).
    pub manifest_extensions: Vec<String>,
    /// Whether to recurse into subdirectories.
    pub recursive: bool,
    /// Maximum recursion depth (0 = unlimited).
    pub max_depth: usize,
}
impl PluginDiscoveryConfig {
    /// Create a default config that searches the current directory non-recursively.
    pub fn new() -> Self {
        Self {
            search_dirs: vec![PathBuf::from(".")],
            manifest_extensions: vec!["oxplugin".to_string(), "toml".to_string()],
            recursive: false,
            max_depth: 3,
        }
    }
    /// Add a search directory.
    pub fn add_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.search_dirs.push(dir.into());
        self
    }
    /// Enable recursive directory scanning.
    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }
}
/// Monitors the health of loaded plugins and records failures.
#[allow(dead_code)]
pub struct PluginHealthMonitor {
    failure_counts: std::collections::HashMap<String, u32>,
    pub max_failures: u32,
}
impl PluginHealthMonitor {
    #[allow(dead_code)]
    pub fn new(max_failures: u32) -> Self {
        Self {
            failure_counts: std::collections::HashMap::new(),
            max_failures,
        }
    }
    /// Record a failure for `plugin`. Returns `true` if the plugin should be disabled.
    #[allow(dead_code)]
    pub fn record_failure(&mut self, plugin: &str) -> bool {
        let count = self.failure_counts.entry(plugin.to_string()).or_insert(0);
        *count += 1;
        *count >= self.max_failures
    }
    /// Reset the failure count for `plugin`.
    #[allow(dead_code)]
    pub fn reset(&mut self, plugin: &str) {
        self.failure_counts.remove(plugin);
    }
    /// Return the failure count for `plugin`.
    #[allow(dead_code)]
    pub fn failure_count(&self, plugin: &str) -> u32 {
        self.failure_counts.get(plugin).copied().unwrap_or(0)
    }
    /// Check if a plugin is healthy (failure count below threshold).
    #[allow(dead_code)]
    pub fn is_healthy(&self, plugin: &str) -> bool {
        self.failure_count(plugin) < self.max_failures
    }
}
/// A plugin paired with its execution priority.
pub struct PrioritizedPlugin {
    /// The priority.
    pub priority: PluginPriority,
    /// The plugin.
    pub plugin: Box<dyn LintPlugin>,
}
impl PrioritizedPlugin {
    /// Create a new prioritized plugin.
    pub fn new(priority: PluginPriority, plugin: Box<dyn LintPlugin>) -> Self {
        Self { priority, plugin }
    }
}
/// A set of named capabilities that a plugin declares it supports.
///
/// Capabilities allow the host engine to query whether a plugin can handle
/// optional features (e.g., incremental analysis, cross-module linting).
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PluginCapabilitySet {
    capabilities: HashSet<String>,
}
#[allow(dead_code)]
impl PluginCapabilitySet {
    /// Create an empty capability set.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a capability by name.
    pub fn add(&mut self, cap: impl Into<String>) -> &mut Self {
        self.capabilities.insert(cap.into());
        self
    }
    /// Check whether a named capability is present.
    pub fn has(&self, cap: &str) -> bool {
        self.capabilities.contains(cap)
    }
    /// Remove a capability by name.
    pub fn remove(&mut self, cap: &str) -> bool {
        self.capabilities.remove(cap)
    }
    /// Return an iterator over all capability names.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.capabilities.iter().map(String::as_str)
    }
    /// Return the number of declared capabilities.
    pub fn len(&self) -> usize {
        self.capabilities.len()
    }
    /// Return `true` if no capabilities are declared.
    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }
    /// Merge another capability set into this one.
    pub fn merge(&mut self, other: &PluginCapabilitySet) {
        for cap in &other.capabilities {
            self.capabilities.insert(cap.clone());
        }
    }
    /// Return the intersection of two capability sets.
    pub fn intersection(&self, other: &PluginCapabilitySet) -> PluginCapabilitySet {
        let caps = self
            .capabilities
            .intersection(&other.capabilities)
            .cloned()
            .collect();
        PluginCapabilitySet { capabilities: caps }
    }
    /// Return the union of two capability sets.
    pub fn union(&self, other: &PluginCapabilitySet) -> PluginCapabilitySet {
        let caps = self
            .capabilities
            .union(&other.capabilities)
            .cloned()
            .collect();
        PluginCapabilitySet { capabilities: caps }
    }
}
/// Per-plugin configuration key-value store.
#[derive(Clone, Debug, Default)]
pub struct PluginConfig {
    values: HashMap<String, String>,
}
impl PluginConfig {
    /// Create an empty config.
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    /// Set a configuration key.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }
    /// Get a configuration value by key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }
    /// Get a value as a boolean (`"true"` / `"false"`).
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        match self.get(key) {
            Some("true") | Some("1") | Some("yes") => Some(true),
            Some("false") | Some("0") | Some("no") => Some(false),
            _ => None,
        }
    }
    /// Get a value as a `usize`.
    pub fn get_usize(&self, key: &str) -> Option<usize> {
        self.get(key)?.parse().ok()
    }
    /// Remove a key and return its value.
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.values.remove(key)
    }
    /// Iterate over all key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.values.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
    /// Return the number of configuration entries.
    pub fn len(&self) -> usize {
        self.values.len()
    }
    /// Return `true` if no entries are set.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}
/// Discovers plugin manifests on the filesystem.
pub struct PluginDiscovery {
    config: PluginDiscoveryConfig,
}
impl PluginDiscovery {
    /// Create a discoverer with the given config.
    pub fn new(config: PluginDiscoveryConfig) -> Self {
        Self { config }
    }
    /// Scan the configured directories and return discovered manifest paths.
    ///
    /// This is a simulation — in a real system it would walk the filesystem.
    pub fn discover(&self) -> Vec<PathBuf> {
        let mut found = Vec::new();
        for dir in &self.config.search_dirs {
            self.scan_dir(dir, 0, &mut found);
        }
        found
    }
    fn scan_dir(&self, dir: &Path, depth: usize, found: &mut Vec<PathBuf>) {
        if self.config.max_depth > 0 && depth > self.config.max_depth {
            return;
        }
        for ext in &self.config.manifest_extensions {
            let synthetic = dir.join(format!("plugin.{}", ext));
            found.push(synthetic);
        }
        if self.config.recursive && depth < self.config.max_depth.max(1) {
            let subdir = dir.join("plugins");
            if depth + 1 <= self.config.max_depth {
                self.scan_dir(&subdir, depth + 1, found);
            }
        }
    }
    /// Filter discovered paths by extension.
    pub fn filter_by_extension<'a>(paths: &'a [PathBuf], ext: &str) -> Vec<&'a PathBuf> {
        paths
            .iter()
            .filter(|p| {
                p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e == ext)
                    .unwrap_or(false)
            })
            .collect()
    }
}
/// Priority level for plugin execution ordering.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PluginPriority {
    /// Executed first — critical checks.
    High,
    /// Normal execution order.
    Normal,
    /// Executed last — cosmetic/style checks.
    Low,
}
/// Rate-limits plugin invocations to prevent runaway execution.
#[allow(dead_code)]
pub struct PluginRateLimiter {
    pub max_calls_per_second: u32,
    call_count: u32,
    window_start: u64,
}
impl PluginRateLimiter {
    #[allow(dead_code)]
    pub fn new(max_calls_per_second: u32) -> Self {
        Self {
            max_calls_per_second,
            call_count: 0,
            window_start: 0,
        }
    }
    /// Record a call and return `true` if within rate limit.
    #[allow(dead_code)]
    pub fn allow(&mut self, current_tick: u64) -> bool {
        if current_tick != self.window_start {
            self.window_start = current_tick;
            self.call_count = 0;
        }
        self.call_count += 1;
        self.call_count <= self.max_calls_per_second
    }
    /// Reset the call counter.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.call_count = 0;
    }
}
/// Registry holding all registered lint plugins.
pub struct PluginRegistry {
    plugins: Vec<Box<dyn LintPlugin>>,
}
impl PluginRegistry {
    /// Create an empty plugin registry.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    /// Register a plugin.
    pub fn register(&mut self, plugin: Box<dyn LintPlugin>) {
        self.plugins.push(plugin);
    }
    /// Unregister a plugin by name.
    ///
    /// Returns `true` if a plugin with that name was found and removed.
    pub fn unregister(&mut self, name: &str) -> bool {
        let before = self.plugins.len();
        self.plugins.retain(|p| p.name() != name);
        self.plugins.len() < before
    }
    /// Check whether a plugin with the given name is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.plugins.iter().any(|p| p.name() == name)
    }
    /// Return the number of registered plugins.
    pub fn len(&self) -> usize {
        self.plugins.len()
    }
    /// Return `true` if no plugins are registered.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
    /// Aggregate all rules from all registered plugins.
    pub fn get_all_rules(&self) -> Vec<Box<dyn LintRule>> {
        self.plugins.iter().flat_map(|p| p.rules()).collect()
    }
    /// Return names of all registered plugins.
    pub fn plugin_names(&self) -> Vec<&str> {
        self.plugins.iter().map(|p| p.name()).collect()
    }
}
/// Result of a single plugin test assertion.
#[derive(Clone, Debug)]
pub struct PluginTestResult {
    /// Name of the test.
    pub test_name: String,
    /// Whether the test passed.
    pub passed: bool,
    /// Failure message if the test failed.
    pub failure_msg: Option<String>,
}
impl PluginTestResult {
    /// Create a passing test result.
    pub fn pass(test_name: &str) -> Self {
        Self {
            test_name: test_name.to_string(),
            passed: true,
            failure_msg: None,
        }
    }
    /// Create a failing test result.
    pub fn fail(test_name: &str, msg: &str) -> Self {
        Self {
            test_name: test_name.to_string(),
            passed: false,
            failure_msg: Some(msg.to_string()),
        }
    }
}
/// Runtime resource usage tracked for a plugin invocation.
#[derive(Clone, Debug, Default)]
pub struct PluginResourceUsage {
    /// Approximate CPU time consumed in milliseconds.
    pub cpu_ms: u64,
    /// Peak memory usage in kilobytes.
    pub peak_memory_kb: u64,
    /// Number of diagnostics emitted.
    pub diagnostics_emitted: usize,
    /// Number of files processed.
    pub files_processed: usize,
}
impl PluginResourceUsage {
    /// Create a zeroed usage record.
    pub fn new() -> Self {
        Self::default()
    }
    /// Merge another usage record into this one (accumulate).
    pub fn merge(&mut self, other: &PluginResourceUsage) {
        self.cpu_ms += other.cpu_ms;
        self.peak_memory_kb = self.peak_memory_kb.max(other.peak_memory_kb);
        self.diagnostics_emitted += other.diagnostics_emitted;
        self.files_processed += other.files_processed;
    }
    /// Check whether usage exceeds the given sandbox limits.
    pub fn exceeds_limits(&self, sandbox: &PluginSandboxConfig) -> bool {
        self.cpu_ms > sandbox.max_cpu_ms
            || self.peak_memory_kb > sandbox.max_memory_mb * 1024
            || self.diagnostics_emitted > sandbox.max_diagnostics_per_file
    }
}
/// Manages hot-reload lifecycle for the plugin registry.
pub struct HotReloadManager {
    state: HotReloadState,
    /// Reload callbacks (plugin name → reload count).
    reload_counts: HashMap<String, u32>,
}
impl HotReloadManager {
    /// Create a manager with hot-reload enabled.
    pub fn new(enabled: bool) -> Self {
        Self {
            state: HotReloadState::new(enabled),
            reload_counts: HashMap::new(),
        }
    }
    /// Check which plugins are stale and record a reload for them.
    pub fn check_and_reload(&mut self, current_stamps: &HashMap<String, u64>) -> Vec<String> {
        if !self.state.enabled {
            return Vec::new();
        }
        let stale: Vec<String> = self
            .state
            .stale_plugins(current_stamps)
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        for name in &stale {
            *self.reload_counts.entry(name.clone()).or_insert(0) += 1;
        }
        self.state.commit(current_stamps);
        stale
    }
    /// Return the total number of reload events for a plugin.
    pub fn reload_count(&self, name: &str) -> u32 {
        self.reload_counts.get(name).copied().unwrap_or(0)
    }
    /// Enable or disable hot-reload.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.state.enabled = enabled;
    }
}
/// A lint rule that flags `sorry` usage in proofs.
pub struct NoSorryRule;
/// The built-in plugin providing OxiLean's standard lint rules.
pub struct BuiltinPlugin;
impl BuiltinPlugin {
    /// Create a new instance of the built-in plugin.
    pub fn new() -> Self {
        Self
    }
}
/// Harness for testing lint plugins in isolation.
pub struct PluginTestHarness {
    results: Vec<PluginTestResult>,
}
impl PluginTestHarness {
    /// Create a new test harness.
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }
    /// Assert that a plugin provides a rule with the given id.
    pub fn assert_has_rule(&mut self, plugin: &dyn LintPlugin, rule_id: &str) {
        let rules = plugin.rules();
        let found = rules.iter().any(|r| r.id().as_str() == rule_id);
        if found {
            self.results
                .push(PluginTestResult::pass(&format!("has_rule:{}", rule_id)));
        } else {
            self.results.push(PluginTestResult::fail(
                &format!("has_rule:{}", rule_id),
                &format!(
                    "Plugin '{}' does not provide rule '{}'",
                    plugin.name(),
                    rule_id
                ),
            ));
        }
    }
    /// Assert that a plugin's version is parseable as SemVer.
    pub fn assert_version_parseable(&mut self, plugin: &dyn LintPlugin) {
        let version = plugin.version();
        if SemVer::parse(version).is_some() {
            self.results.push(PluginTestResult::pass(&format!(
                "version_parseable:{}",
                plugin.name()
            )));
        } else {
            self.results.push(PluginTestResult::fail(
                &format!("version_parseable:{}", plugin.name()),
                &format!("Version '{}' is not valid SemVer", version),
            ));
        }
    }
    /// Assert that a plugin's name is non-empty.
    pub fn assert_name_non_empty(&mut self, plugin: &dyn LintPlugin) {
        let name = plugin.name();
        if !name.is_empty() {
            self.results.push(PluginTestResult::pass("name_non_empty"));
        } else {
            self.results.push(PluginTestResult::fail(
                "name_non_empty",
                "Plugin name must not be empty",
            ));
        }
    }
    /// Assert that the plugin provides at least one rule.
    pub fn assert_has_rules(&mut self, plugin: &dyn LintPlugin) {
        let rules = plugin.rules();
        if !rules.is_empty() {
            self.results.push(PluginTestResult::pass(&format!(
                "has_rules:{}",
                plugin.name()
            )));
        } else {
            self.results.push(PluginTestResult::fail(
                &format!("has_rules:{}", plugin.name()),
                "Plugin provides no rules",
            ));
        }
    }
    /// Return all results.
    pub fn results(&self) -> &[PluginTestResult] {
        &self.results
    }
    /// Return `true` if all assertions passed.
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }
    /// Return the number of failures.
    pub fn failure_count(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }
    /// Print a summary to a string.
    pub fn summary(&self) -> String {
        let total = self.results.len();
        let passed = self.results.iter().filter(|r| r.passed).count();
        let failed = total - passed;
        let mut out = format!("Plugin tests: {}/{} passed", passed, total);
        if failed > 0 {
            out.push('\n');
            for r in self.results.iter().filter(|r| !r.passed) {
                out.push_str(&format!(
                    "  FAIL {}: {}\n",
                    r.test_name,
                    r.failure_msg.as_deref().unwrap_or("unknown")
                ));
            }
        }
        out
    }
}
/// Log of plugin lifecycle events.
#[derive(Default)]
pub struct PluginEventLog {
    events: Vec<PluginEvent>,
}
impl PluginEventLog {
    /// Create an empty event log.
    pub fn new() -> Self {
        Self::default()
    }
    /// Append an event.
    pub fn push(&mut self, event: PluginEvent) {
        self.events.push(event);
    }
    /// Return all events.
    pub fn events(&self) -> &[PluginEvent] {
        &self.events
    }
    /// Return events matching a predicate.
    pub fn filter<F>(&self, pred: F) -> Vec<&PluginEvent>
    where
        F: Fn(&PluginEvent) -> bool,
    {
        self.events.iter().filter(|e| pred(e)).collect()
    }
    /// Count events of a specific kind using a predicate.
    pub fn count_where<F>(&self, pred: F) -> usize
    where
        F: Fn(&PluginEvent) -> bool,
    {
        self.events.iter().filter(|e| pred(e)).count()
    }
    /// Clear all recorded events.
    pub fn clear(&mut self) {
        self.events.clear();
    }
}
/// State tracking for hot-reload of plugins.
#[derive(Clone, Debug)]
pub struct HotReloadState {
    /// Last-modified stamps (plugin name → timestamp-like u64).
    stamps: HashMap<String, u64>,
    /// Whether hot-reload is enabled.
    pub enabled: bool,
}
impl HotReloadState {
    /// Create a new hot-reload state.
    pub fn new(enabled: bool) -> Self {
        Self {
            stamps: HashMap::new(),
            enabled,
        }
    }
    /// Record a stamp for the named plugin.
    pub fn record(&mut self, name: &str, stamp: u64) {
        self.stamps.insert(name.to_string(), stamp);
    }
    /// Check whether the plugin's stamp has changed.
    pub fn has_changed(&self, name: &str, current_stamp: u64) -> bool {
        match self.stamps.get(name) {
            None => true,
            Some(&prev) => prev != current_stamp,
        }
    }
    /// Return plugins that need to be reloaded.
    pub fn stale_plugins<'a>(&self, current: &'a HashMap<String, u64>) -> Vec<&'a str> {
        current
            .iter()
            .filter(|(name, &stamp)| self.has_changed(name, stamp))
            .map(|(name, _)| name.as_str())
            .collect()
    }
    /// Update all stamps from the current snapshot.
    pub fn commit(&mut self, current: &HashMap<String, u64>) {
        for (k, v) in current {
            self.stamps.insert(k.clone(), *v);
        }
    }
}
/// Executes plugins respecting sandbox limits and policies.
pub struct PluginRunner {
    policy: ProjectPluginPolicy,
}
impl PluginRunner {
    /// Create a runner with the given policy.
    pub fn new(policy: ProjectPluginPolicy) -> Self {
        Self { policy }
    }
    /// Run a plugin and return its result.
    ///
    /// In a real implementation this would enforce CPU/memory limits.
    pub fn run_plugin(&self, plugin: &dyn LintPlugin) -> PluginRunResult {
        let name = plugin.name();
        if !self.policy.is_allowed(name) {
            return PluginRunResult::aborted(name, "disabled by policy");
        }
        let sandbox = self.policy.effective_sandbox(name);
        let rules = plugin.rules();
        if sandbox.exceeds_rule_limit(rules.len()) {
            return PluginRunResult::aborted(
                name,
                &format!(
                    "rule count {} exceeds sandbox limit {}",
                    rules.len(),
                    sandbox.max_rules
                ),
            );
        }
        PluginRunResult::success(name, 0)
    }
    /// Run all plugins in a registry and aggregate results.
    pub fn run_all(&self, registry: &PluginRegistry) -> Vec<PluginRunResult> {
        registry
            .plugins
            .iter()
            .map(|p| self.run_plugin(p.as_ref()))
            .collect()
    }
}
/// Tags associated with a plugin for categorization.
#[derive(Clone, Debug, Default)]
pub struct PluginTags {
    tags: HashSet<String>,
}
impl PluginTags {
    /// Create empty tags.
    pub fn new() -> Self {
        Self::default()
    }
    /// Create from an iterator of tag strings.
    pub fn from_iter<I: IntoIterator<Item = S>, S: Into<String>>(iter: I) -> Self {
        Self {
            tags: iter.into_iter().map(|s| s.into()).collect(),
        }
    }
    /// Add a tag.
    pub fn add(&mut self, tag: impl Into<String>) {
        self.tags.insert(tag.into());
    }
    /// Remove a tag.
    pub fn remove(&mut self, tag: &str) {
        self.tags.remove(tag);
    }
    /// Check whether a tag is present.
    pub fn has(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }
    /// Return all tags.
    pub fn all(&self) -> Vec<&str> {
        self.tags.iter().map(|s| s.as_str()).collect()
    }
    /// Return the number of tags.
    pub fn len(&self) -> usize {
        self.tags.len()
    }
    /// Return `true` if there are no tags.
    pub fn is_empty(&self) -> bool {
        self.tags.is_empty()
    }
    /// Check whether any tag in `self` is also in `other`.
    pub fn overlaps(&self, other: &PluginTags) -> bool {
        self.tags.iter().any(|t| other.has(t))
    }
}
/// Example custom plugin demonstrating the plugin API.
///
/// This plugin provides two rules: `no_sorry` and `prefer_omega`.
pub struct ExampleCustomPlugin;
impl ExampleCustomPlugin {
    /// Create a new instance of the example custom plugin.
    pub fn new() -> Self {
        Self
    }
}
/// Tracks plugin dependencies (plugin A requires plugin B).
#[allow(dead_code)]
pub struct PluginDependencyGraph {
    /// Map from plugin name to its list of required plugins.
    edges: std::collections::HashMap<String, Vec<String>>,
}
impl PluginDependencyGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            edges: std::collections::HashMap::new(),
        }
    }
    /// Declare that `plugin` depends on `dependency`.
    #[allow(dead_code)]
    pub fn add_dependency(&mut self, plugin: &str, dependency: &str) {
        self.edges
            .entry(plugin.to_string())
            .or_default()
            .push(dependency.to_string());
    }
    /// Return the direct dependencies of `plugin`.
    #[allow(dead_code)]
    pub fn dependencies_of(&self, plugin: &str) -> Vec<&str> {
        self.edges
            .get(plugin)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
    /// Detect whether there is a cycle involving `plugin`.
    #[allow(dead_code)]
    pub fn has_cycle(&self, start: &str) -> bool {
        let mut visited = std::collections::HashSet::new();
        self.dfs_cycle(start, start, &mut visited)
    }
    fn dfs_cycle(
        &self,
        current: &str,
        target: &str,
        visited: &mut std::collections::HashSet<String>,
    ) -> bool {
        if let Some(deps) = self.edges.get(current) {
            for dep in deps {
                if dep == target {
                    return true;
                }
                if visited.insert(dep.clone()) && self.dfs_cycle(dep, target, visited) {
                    return true;
                }
            }
        }
        false
    }
}
/// A single plugin audit event.
#[allow(dead_code)]
pub struct PluginAuditEntry {
    pub id: u64,
    pub plugin_name: String,
    pub event: String,
    pub success: bool,
}
/// A simple semantic version (major.minor.patch).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SemVer {
    /// Major version number.
    pub major: u32,
    /// Minor version number.
    pub minor: u32,
    /// Patch version number.
    pub patch: u32,
}
impl SemVer {
    /// Create a new semantic version.
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }
    /// Parse a version string like `"1.2.3"`.
    ///
    /// Returns `None` if parsing fails.
    pub fn parse(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return None;
        }
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        let patch = parts[2].parse().ok()?;
        Some(Self {
            major,
            minor,
            patch,
        })
    }
    /// Return the string representation.
    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
    /// Check whether `self` is compatible with `required`.
    ///
    /// Compatible means same major version and `self >= required`.
    pub fn is_compatible_with(&self, required: &SemVer) -> bool {
        self.major == required.major && *self >= *required
    }
    /// Check whether this version is a pre-release (major == 0).
    pub fn is_pre_release(&self) -> bool {
        self.major == 0
    }
    /// Increment the patch component and return a new version.
    pub fn bump_patch(&self) -> Self {
        Self::new(self.major, self.minor, self.patch + 1)
    }
    /// Increment the minor component, reset patch, and return a new version.
    pub fn bump_minor(&self) -> Self {
        Self::new(self.major, self.minor + 1, 0)
    }
    /// Increment the major component, reset minor and patch, and return a new version.
    pub fn bump_major(&self) -> Self {
        Self::new(self.major + 1, 0, 0)
    }
}
/// Per-project policy controlling which plugins are active.
#[derive(Clone, Debug, Default)]
pub struct ProjectPluginPolicy {
    /// Explicitly enabled plugin names (empty = all enabled by default).
    pub enabled: HashSet<String>,
    /// Explicitly disabled plugin names.
    pub disabled: HashSet<String>,
    /// Per-plugin configuration overrides.
    pub configs: HashMap<String, PluginConfig>,
    /// Per-plugin sandbox overrides.
    pub sandboxes: HashMap<String, PluginSandboxConfig>,
}
impl ProjectPluginPolicy {
    /// Create a policy that enables all plugins.
    pub fn all_enabled() -> Self {
        Self::default()
    }
    /// Enable a named plugin.
    pub fn enable(&mut self, name: impl Into<String>) {
        let n = name.into();
        self.disabled.remove(&n);
        self.enabled.insert(n);
    }
    /// Disable a named plugin.
    pub fn disable(&mut self, name: impl Into<String>) {
        let n = name.into();
        self.enabled.remove(&n);
        self.disabled.insert(n);
    }
    /// Check whether the named plugin is allowed to run.
    pub fn is_allowed(&self, name: &str) -> bool {
        if self.disabled.contains(name) {
            return false;
        }
        if !self.enabled.is_empty() {
            return self.enabled.contains(name);
        }
        true
    }
    /// Set per-plugin configuration.
    pub fn set_config(&mut self, name: impl Into<String>, config: PluginConfig) {
        self.configs.insert(name.into(), config);
    }
    /// Get per-plugin configuration.
    pub fn get_config(&self, name: &str) -> Option<&PluginConfig> {
        self.configs.get(name)
    }
    /// Set per-plugin sandbox config.
    pub fn set_sandbox(&mut self, name: impl Into<String>, sandbox: PluginSandboxConfig) {
        self.sandboxes.insert(name.into(), sandbox);
    }
    /// Get the effective sandbox for a plugin (plugin-specific or default).
    pub fn effective_sandbox(&self, name: &str) -> PluginSandboxConfig {
        self.sandboxes
            .get(name)
            .cloned()
            .unwrap_or_else(PluginSandboxConfig::default)
    }
}
/// Maintains an immutable audit log of plugin events.
#[allow(dead_code)]
pub struct PluginAuditLog {
    entries: Vec<PluginAuditEntry>,
    counter: u64,
}
impl PluginAuditLog {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            counter: 0,
        }
    }
    #[allow(dead_code)]
    pub fn log(&mut self, plugin_name: &str, event: &str, success: bool) -> u64 {
        self.counter += 1;
        let id = self.counter;
        self.entries.push(PluginAuditEntry {
            id,
            plugin_name: plugin_name.to_string(),
            event: event.to_string(),
            success,
        });
        id
    }
    #[allow(dead_code)]
    pub fn total(&self) -> usize {
        self.entries.len()
    }
    #[allow(dead_code)]
    pub fn failures(&self) -> Vec<&PluginAuditEntry> {
        self.entries.iter().filter(|e| !e.success).collect()
    }
}
/// A dependency declaration between plugins.
#[derive(Clone, Debug)]
pub struct PluginDependency {
    /// The name of the required plugin.
    pub plugin_name: String,
    /// The minimum version required.
    pub min_version: SemVer,
    /// Whether the dependency is optional.
    pub optional: bool,
}
impl PluginDependency {
    /// Declare a mandatory dependency.
    pub fn required(plugin_name: impl Into<String>, min_version: SemVer) -> Self {
        Self {
            plugin_name: plugin_name.into(),
            min_version,
            optional: false,
        }
    }
    /// Declare an optional dependency.
    pub fn optional(plugin_name: impl Into<String>, min_version: SemVer) -> Self {
        Self {
            plugin_name: plugin_name.into(),
            min_version,
            optional: true,
        }
    }
}
/// The outcome of running a plugin on a file.
#[derive(Clone, Debug)]
pub struct PluginRunResult {
    /// Name of the plugin.
    pub plugin_name: String,
    /// Number of diagnostics emitted.
    pub diagnostic_count: usize,
    /// Whether execution was aborted due to resource limits.
    pub aborted: bool,
    /// Human-readable status message.
    pub status: String,
    /// Resource usage recorded during this run.
    pub usage: PluginResourceUsage,
}
impl PluginRunResult {
    /// Create a successful result.
    pub fn success(plugin_name: &str, diag_count: usize) -> Self {
        Self {
            plugin_name: plugin_name.to_string(),
            diagnostic_count: diag_count,
            aborted: false,
            status: "ok".to_string(),
            usage: PluginResourceUsage::new(),
        }
    }
    /// Create an aborted result.
    pub fn aborted(plugin_name: &str, reason: &str) -> Self {
        Self {
            plugin_name: plugin_name.to_string(),
            diagnostic_count: 0,
            aborted: true,
            status: format!("aborted: {}", reason),
            usage: PluginResourceUsage::new(),
        }
    }
    /// Attach resource usage.
    pub fn with_usage(mut self, usage: PluginResourceUsage) -> Self {
        self.usage = usage;
        self
    }
}
/// Serializable manifest describing a lint plugin.
#[derive(Clone, Debug)]
pub struct PluginManifest {
    /// Unique plugin name.
    pub name: String,
    /// Semantic version string.
    pub version: String,
    /// Human-readable description.
    pub description: String,
    /// Names of rules provided by the plugin.
    pub rules: Vec<String>,
}
impl PluginManifest {
    /// Create a new plugin manifest.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        description: impl Into<String>,
        rules: Vec<String>,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: description.into(),
            rules,
        }
    }
    /// Serialize the manifest to a simple TOML-like text format.
    pub fn to_text(&self) -> String {
        let mut out = format!(
            "name = \"{}\"\nversion = \"{}\"\ndescription = \"{}\"\nrules = [",
            self.name, self.version, self.description
        );
        let rule_strs: Vec<String> = self.rules.iter().map(|r| format!("\"{}\"", r)).collect();
        out.push_str(&rule_strs.join(", "));
        out.push(']');
        out
    }
}
/// Checks plugin version compatibility against engine requirements.
pub struct CompatibilityChecker {
    /// Minimum engine API version required.
    pub min_engine_version: SemVer,
    /// Maximum engine API version supported.
    pub max_engine_version: Option<SemVer>,
}
impl CompatibilityChecker {
    /// Create a checker requiring exactly the given major version.
    pub fn new(min: SemVer) -> Self {
        Self {
            min_engine_version: min,
            max_engine_version: None,
        }
    }
    /// Set an upper bound on the engine version.
    pub fn with_max(mut self, max: SemVer) -> Self {
        self.max_engine_version = Some(max);
        self
    }
    /// Check whether the given engine version satisfies the requirements.
    pub fn is_engine_compatible(&self, engine: &SemVer) -> bool {
        if !engine.is_compatible_with(&self.min_engine_version) {
            return false;
        }
        if let Some(ref max) = self.max_engine_version {
            if engine > max {
                return false;
            }
        }
        true
    }
    /// Produce a human-readable description of the requirement.
    pub fn requirement_description(&self) -> String {
        match &self.max_engine_version {
            Some(max) => format!(">={} and <={}", self.min_engine_version, max),
            None => format!(">={}", self.min_engine_version),
        }
    }
}
/// Events emitted during plugin lifecycle.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PluginEvent {
    /// A plugin was registered.
    Registered(String),
    /// A plugin was unregistered.
    Unregistered(String),
    /// A plugin was reloaded.
    Reloaded(String),
    /// A plugin was disabled.
    Disabled(String),
    /// A plugin was enabled.
    Enabled(String),
    /// A plugin run was aborted.
    RunAborted { plugin: String, reason: String },
}
/// Resolves dependencies among a set of registered plugins.
pub struct DependencyResolver;
impl DependencyResolver {
    /// Resolve `deps` against the given map of available plugins and their versions.
    pub fn resolve(
        deps: &[PluginDependency],
        available: &HashMap<String, SemVer>,
    ) -> Vec<ResolveResult> {
        let mut results = Vec::new();
        let mut missing_mandatory = Vec::new();
        let mut missing_optional = Vec::new();
        for dep in deps {
            match available.get(&dep.plugin_name) {
                None => {
                    if dep.optional {
                        missing_optional.push(dep.plugin_name.clone());
                    } else {
                        missing_mandatory.push(dep.plugin_name.clone());
                    }
                }
                Some(found_ver) => {
                    if !found_ver.is_compatible_with(&dep.min_version) {
                        results.push(ResolveResult::IncompatibleVersion {
                            dep_name: dep.plugin_name.clone(),
                            required: dep.min_version.to_string(),
                            found: found_ver.to_string(),
                        });
                    }
                }
            }
        }
        if !missing_mandatory.is_empty() {
            results.push(ResolveResult::MissingMandatory(missing_mandatory));
        }
        if !missing_optional.is_empty() {
            results.push(ResolveResult::MissingOptional(missing_optional));
        }
        if results.is_empty() {
            results.push(ResolveResult::Ok);
        }
        results
    }
    /// Returns `true` when resolution produced no hard errors.
    pub fn is_ok(results: &[ResolveResult]) -> bool {
        !results.iter().any(|r| {
            matches!(
                r,
                ResolveResult::MissingMandatory(_) | ResolveResult::IncompatibleVersion { .. }
            )
        })
    }
}
/// Result of dependency resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResolveResult {
    /// All dependencies satisfied.
    Ok,
    /// One or more mandatory dependencies are missing.
    MissingMandatory(Vec<String>),
    /// One or more optional dependencies are missing.
    MissingOptional(Vec<String>),
    /// A dependency has an incompatible version.
    IncompatibleVersion {
        /// The dependency whose version is wrong.
        dep_name: String,
        /// The required minimum version.
        required: String,
        /// The found version.
        found: String,
    },
}
/// A point-in-time snapshot of the plugin registry.
#[derive(Clone, Debug)]
pub struct PluginRegistrySnapshot {
    /// Plugin names at snapshot time.
    pub plugin_names: Vec<String>,
    /// Plugin versions at snapshot time.
    pub plugin_versions: Vec<String>,
    /// Total rule count at snapshot time.
    pub total_rule_count: usize,
    /// Snapshot timestamp (arbitrary u64 counter for testing).
    pub timestamp: u64,
}
impl PluginRegistrySnapshot {
    /// Take a snapshot from a registry.
    pub fn take(registry: &PluginRegistry, timestamp: u64) -> Self {
        let plugin_names: Vec<String> = registry
            .plugins
            .iter()
            .map(|p| p.name().to_string())
            .collect();
        let plugin_versions: Vec<String> = registry
            .plugins
            .iter()
            .map(|p| p.version().to_string())
            .collect();
        let total_rule_count: usize = registry.plugins.iter().map(|p| p.rules().len()).sum();
        Self {
            plugin_names,
            plugin_versions,
            total_rule_count,
            timestamp,
        }
    }
    /// Check whether the snapshot differs from another.
    pub fn differs_from(&self, other: &PluginRegistrySnapshot) -> bool {
        self.plugin_names != other.plugin_names
            || self.plugin_versions != other.plugin_versions
            || self.total_rule_count != other.total_rule_count
    }
}
/// Resource limits for plugin sandboxing.
#[derive(Clone, Debug)]
pub struct PluginSandboxConfig {
    /// Maximum CPU time in milliseconds per lint pass.
    pub max_cpu_ms: u64,
    /// Maximum memory in megabytes.
    pub max_memory_mb: u64,
    /// Maximum number of diagnostics a plugin may emit per file.
    pub max_diagnostics_per_file: usize,
    /// Whether the plugin may read files from the filesystem.
    pub allow_fs_read: bool,
    /// Whether the plugin may write files to the filesystem.
    pub allow_fs_write: bool,
    /// Whether the plugin may spawn subprocesses.
    pub allow_subprocesses: bool,
    /// Maximum number of lint rules a plugin may contribute.
    pub max_rules: usize,
}
impl PluginSandboxConfig {
    /// Create a permissive sandbox (suitable for trusted built-in plugins).
    pub fn permissive() -> Self {
        Self {
            max_cpu_ms: 30_000,
            max_memory_mb: 512,
            max_diagnostics_per_file: 10_000,
            allow_fs_read: true,
            allow_fs_write: false,
            allow_subprocesses: false,
            max_rules: 1000,
        }
    }
    /// Create a strict sandbox (suitable for untrusted third-party plugins).
    pub fn strict() -> Self {
        Self {
            max_cpu_ms: 5_000,
            max_memory_mb: 64,
            max_diagnostics_per_file: 500,
            allow_fs_read: false,
            allow_fs_write: false,
            allow_subprocesses: false,
            max_rules: 50,
        }
    }
    /// Check whether a given number of emitted diagnostics exceeds the limit.
    pub fn exceeds_diag_limit(&self, count: usize) -> bool {
        count > self.max_diagnostics_per_file
    }
    /// Check whether a given number of rules exceeds the limit.
    pub fn exceeds_rule_limit(&self, count: usize) -> bool {
        count > self.max_rules
    }
}
/// A lint rule that suggests using `omega` for arithmetic goals.
pub struct PreferOmegaRule;
/// Registry that runs plugins in priority order.
pub struct PrioritizedPluginRegistry {
    plugins: Vec<PrioritizedPlugin>,
}
impl PrioritizedPluginRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    /// Add a plugin with a priority.
    pub fn add(&mut self, priority: PluginPriority, plugin: Box<dyn LintPlugin>) {
        self.plugins.push(PrioritizedPlugin::new(priority, plugin));
    }
    /// Sort plugins by priority (High first, Low last).
    pub fn sort(&mut self) {
        self.plugins.sort_by_key(|p| p.priority);
    }
    /// Return rules from all plugins in priority order.
    pub fn all_rules_ordered(&mut self) -> Vec<Box<dyn LintRule>> {
        self.sort();
        self.plugins.iter().flat_map(|p| p.plugin.rules()).collect()
    }
    /// Return plugin names in priority order.
    pub fn names_ordered(&mut self) -> Vec<String> {
        self.sort();
        self.plugins
            .iter()
            .map(|p| p.plugin.name().to_string())
            .collect()
    }
    /// Return the number of registered plugins.
    pub fn len(&self) -> usize {
        self.plugins.len()
    }
    /// Return `true` if no plugins are registered.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }
}
