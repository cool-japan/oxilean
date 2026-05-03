//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::io::Write;

use super::functions::*;

use std::collections::HashMap;

/// The context in which completion is being performed.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CompletionContext {
    /// Full command line so far
    pub cmdline: String,
    /// The word being completed
    pub current_word: String,
    /// Position (word index) of the cursor
    pub cursor_position: usize,
    /// Parsed subcommand if any
    pub active_subcommand: Option<String>,
}
impl CompletionContext {
    /// Parse a completion context from a command line string.
    #[allow(dead_code)]
    pub fn from_cmdline(cmdline: &str, cursor_byte: usize) -> Self {
        let effective = &cmdline[..cursor_byte.min(cmdline.len())];
        let words: Vec<&str> = effective.split_whitespace().collect();
        let current_word = if effective.ends_with(' ') {
            String::new()
        } else {
            words.last().copied().unwrap_or("").to_string()
        };
        let cursor_position = if effective.ends_with(' ') {
            words.len()
        } else {
            words.len().saturating_sub(1)
        };
        let active_subcommand = words.get(1).map(|s| s.to_string());
        Self {
            cmdline: cmdline.to_string(),
            current_word,
            cursor_position,
            active_subcommand,
        }
    }
}
/// Tracks completion history to prioritize frequently-used completions.
#[allow(dead_code)]
pub struct CompletionHistory {
    entries: Vec<CompletionHistoryEntry>,
    max_entries: usize,
}
impl CompletionHistory {
    /// Create a new history tracker.
    #[allow(dead_code)]
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: vec![],
            max_entries,
        }
    }
    /// Record that the user selected `selected` when completing `input` in context `subcommand`.
    #[allow(dead_code)]
    pub fn record(&mut self, input: String, selected: String, subcommand: Option<String>) {
        if let Some(e) = self
            .entries
            .iter_mut()
            .find(|e| e.input == input && e.selected == selected)
        {
            e.count += 1;
            return;
        }
        if self.entries.len() >= self.max_entries {
            if let Some(pos) = self
                .entries
                .iter()
                .enumerate()
                .min_by_key(|(_, e)| e.count)
                .map(|(i, _)| i)
            {
                self.entries.remove(pos);
            }
        }
        self.entries.push(CompletionHistoryEntry {
            input,
            selected,
            subcommand,
            count: 1,
        });
    }
    /// Return the top N completions for a given input, ranked by frequency.
    #[allow(dead_code)]
    pub fn top_completions(&self, input: &str, n: usize) -> Vec<&CompletionHistoryEntry> {
        let mut matching: Vec<&CompletionHistoryEntry> = self
            .entries
            .iter()
            .filter(|e| e.input.starts_with(input))
            .collect();
        matching.sort_by_key(|b| std::cmp::Reverse(b.count));
        matching.truncate(n);
        matching
    }
    /// Clear all history.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// Persistent configuration for the completion system.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CompletionConfig {
    pub enabled: bool,
    pub cache_ttl_ms: u64,
    pub max_candidates: usize,
    pub fuzzy_matching: bool,
    pub history_enabled: bool,
    pub history_max_entries: usize,
    pub output_format: CompletionOutputFormat,
}
impl CompletionConfig {
    /// Create a default config.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Disable completions.
    #[allow(dead_code)]
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Self::default()
        }
    }
}
/// An LSP-style completion item.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LspCompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub documentation: Option<String>,
    pub insert_text: Option<String>,
    pub sort_text: Option<String>,
    pub filter_text: Option<String>,
    pub deprecated: bool,
}
impl LspCompletionItem {
    /// Create a new LSP completion item.
    #[allow(dead_code)]
    pub fn new(label: impl Into<String>, kind: CompletionItemKind) -> Self {
        Self {
            label: label.into(),
            kind,
            detail: None,
            documentation: None,
            insert_text: None,
            sort_text: None,
            filter_text: None,
            deprecated: false,
        }
    }
    /// Convert a CompletionCandidate to an LSP item.
    #[allow(dead_code)]
    pub fn from_candidate(candidate: &CompletionCandidate) -> Self {
        Self::new(candidate.text.clone(), CompletionItemKind::Keyword)
            .with_detail(candidate.description.clone())
    }
    /// Set detail.
    #[allow(dead_code)]
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
    /// Set documentation.
    #[allow(dead_code)]
    pub fn with_documentation(mut self, doc: impl Into<String>) -> Self {
        self.documentation = Some(doc.into());
        self
    }
    /// Set insert text (what actually gets inserted, vs the label).
    #[allow(dead_code)]
    pub fn with_insert_text(mut self, text: impl Into<String>) -> Self {
        self.insert_text = Some(text.into());
        self
    }
    /// Mark as deprecated.
    #[allow(dead_code)]
    pub fn deprecated(mut self) -> Self {
        self.deprecated = true;
        self
    }
}
/// The full completion specification for an application.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct AppCompletionSpec {
    /// Binary name
    pub binary_name: String,
    /// Global flags
    pub global_flags: Vec<CompletionSpec>,
    /// Subcommands
    pub subcommands: Vec<SubcommandSpec>,
}
impl AppCompletionSpec {
    /// Create a new application completion spec.
    #[allow(dead_code)]
    pub fn new(binary_name: impl Into<String>) -> Self {
        Self {
            binary_name: binary_name.into(),
            global_flags: vec![],
            subcommands: vec![],
        }
    }
    /// Add a global flag.
    #[allow(dead_code)]
    pub fn with_global_flag(mut self, flag: CompletionSpec) -> Self {
        self.global_flags.push(flag);
        self
    }
    /// Add a subcommand.
    #[allow(dead_code)]
    pub fn with_subcommand(mut self, subcmd: SubcommandSpec) -> Self {
        self.subcommands.push(subcmd);
        self
    }
    /// Find a subcommand by name or alias.
    #[allow(dead_code)]
    pub fn find_subcommand(&self, name: &str) -> Option<&SubcommandSpec> {
        self.subcommands
            .iter()
            .find(|s| s.name == name || s.aliases.iter().any(|a| a == name))
    }
    /// Build the full OxiLean completion spec.
    #[allow(dead_code)]
    pub fn oxilean_spec() -> Self {
        Self::new("oxilean")
            .with_global_flag(
                CompletionSpec::new("--verbose", "Enable verbose output").with_short("-v"),
            )
            .with_global_flag(CompletionSpec::new("--help", "Print help").with_short("-h"))
            .with_global_flag(CompletionSpec::new("--version", "Print version").with_short("-V"))
            .with_global_flag(
                CompletionSpec::new("--color", "Control color output")
                    .with_possible_values(vec!["auto", "always", "never"]),
            )
            .with_global_flag(
                CompletionSpec::new("--config", "Configuration file path").file_path(),
            )
            .with_global_flag(
                CompletionSpec::new("--log-level", "Log level")
                    .with_possible_values(vec!["error", "warn", "info", "debug", "trace"]),
            )
            .with_subcommand(
                SubcommandSpec::new("check", "Type-check a source file")
                    .with_alias("c")
                    .accepts_files()
                    .with_flag(CompletionSpec::new("--no-deps", "Skip dependency checks"))
                    .with_flag(
                        CompletionSpec::new("--output", "Output format")
                            .with_possible_values(vec!["text", "json", "compact"]),
                    ),
            )
            .with_subcommand(
                SubcommandSpec::new("build", "Build the project")
                    .with_alias("b")
                    .with_flag(CompletionSpec::new("--release", "Build in release mode"))
                    .with_flag(
                        CompletionSpec::new("--jobs", "Number of parallel jobs").takes_value(),
                    )
                    .with_flag(CompletionSpec::new("--target", "Build target").takes_value()),
            )
            .with_subcommand(
                SubcommandSpec::new("repl", "Start interactive REPL")
                    .with_alias("r")
                    .with_flag(CompletionSpec::new("--no-history", "Disable history")),
            )
            .with_subcommand(
                SubcommandSpec::new("format", "Format source files")
                    .with_alias("fmt")
                    .accepts_files()
                    .with_flag(CompletionSpec::new("--check", "Check without writing"))
                    .with_flag(CompletionSpec::new("--diff", "Show diff")),
            )
            .with_subcommand(
                SubcommandSpec::new("doc", "Generate documentation")
                    .with_alias("docs")
                    .with_flag(CompletionSpec::new("--output-dir", "Output directory").file_path())
                    .with_flag(
                        CompletionSpec::new("--format", "Output format")
                            .with_possible_values(vec!["html", "json", "markdown"]),
                    ),
            )
            .with_subcommand(
                SubcommandSpec::new("lint", "Run lint checks")
                    .accepts_files()
                    .with_flag(CompletionSpec::new("--fix", "Auto-fix lint issues"))
                    .with_flag(CompletionSpec::new("--rule", "Specific rule to run").takes_value()),
            )
            .with_subcommand(
                SubcommandSpec::new("serve", "Start LSP server")
                    .with_flag(CompletionSpec::new("--port", "Listen port").takes_value())
                    .with_flag(CompletionSpec::new("--stdio", "Use stdio transport")),
            )
            .with_subcommand(
                SubcommandSpec::new("clean", "Clean build artifacts")
                    .with_flag(CompletionSpec::new("--all", "Remove all caches")),
            )
            .with_subcommand(
                SubcommandSpec::new("test", "Run test suite")
                    .with_flag(CompletionSpec::new("--filter", "Filter test names").takes_value())
                    .with_flag(CompletionSpec::new("--jobs", "Parallel test jobs").takes_value())
                    .with_flag(CompletionSpec::new("--no-capture", "Show test output")),
            )
    }
}
/// A rich specification for a single completion entry.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CompletionSpec {
    /// Short form (e.g. "-v")
    pub short: Option<String>,
    /// Long form (e.g. "--verbose")
    pub long: String,
    /// Human-readable description
    pub description: String,
    /// Whether this takes an argument
    pub takes_value: bool,
    /// Possible values if restricted
    pub possible_values: Vec<String>,
    /// Whether the argument is a file path
    pub is_file_path: bool,
    /// Whether this flag can be repeated
    pub repeatable: bool,
}
impl CompletionSpec {
    /// Create a new flag spec with only a long form.
    #[allow(dead_code)]
    pub fn new(long: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            short: None,
            long: long.into(),
            description: description.into(),
            takes_value: false,
            possible_values: vec![],
            is_file_path: false,
            repeatable: false,
        }
    }
    /// Set the short form.
    #[allow(dead_code)]
    pub fn with_short(mut self, short: impl Into<String>) -> Self {
        self.short = Some(short.into());
        self
    }
    /// Mark this flag as taking a value.
    #[allow(dead_code)]
    pub fn takes_value(mut self) -> Self {
        self.takes_value = true;
        self
    }
    /// Set the possible values for this flag's argument.
    #[allow(dead_code)]
    pub fn with_possible_values(mut self, values: Vec<&str>) -> Self {
        self.possible_values = values.into_iter().map(String::from).collect();
        self.takes_value = true;
        self
    }
    /// Mark this flag as accepting a file path argument.
    #[allow(dead_code)]
    pub fn file_path(mut self) -> Self {
        self.is_file_path = true;
        self.takes_value = true;
        self
    }
    /// Mark this flag as repeatable.
    #[allow(dead_code)]
    pub fn repeatable(mut self) -> Self {
        self.repeatable = true;
        self
    }
}
/// Provides file system path completions.
#[allow(dead_code)]
pub struct FileSystemCompletionProvider {
    pub(super) argument_name: String,
    pub(super) filter_extensions: Vec<String>,
}
impl FileSystemCompletionProvider {
    /// Create a new FS provider for the given argument.
    #[allow(dead_code)]
    pub fn new(argument_name: impl Into<String>) -> Self {
        Self {
            argument_name: argument_name.into(),
            filter_extensions: vec![],
        }
    }
    /// Only complete paths with these extensions.
    #[allow(dead_code)]
    pub fn with_extensions(mut self, exts: Vec<&str>) -> Self {
        self.filter_extensions = exts.into_iter().map(String::from).collect();
        self
    }
}
/// An in-memory LRU cache for completion results.
#[allow(dead_code)]
pub struct CompletionCache {
    entries: std::collections::HashMap<String, CompletionCacheEntry>,
    max_size: usize,
}
impl CompletionCache {
    /// Create a new cache with the given maximum size.
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: std::collections::HashMap::new(),
            max_size,
        }
    }
    /// Insert an entry into the cache.
    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, candidates: Vec<CompletionCandidate>, ttl_ms: u64) {
        if self.entries.len() >= self.max_size {
            let oldest = self
                .entries
                .iter()
                .min_by_key(|(_, e)| e.timestamp)
                .map(|(k, _)| k.clone());
            if let Some(k) = oldest {
                self.entries.remove(&k);
            }
        }
        self.entries.insert(
            key.clone(),
            CompletionCacheEntry {
                key,
                candidates,
                timestamp: std::time::Instant::now(),
                ttl_ms,
            },
        );
    }
    /// Look up candidates for a key (returns None if missing or expired).
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&Vec<CompletionCandidate>> {
        self.entries
            .get(key)
            .filter(|e| e.is_valid())
            .map(|e| &e.candidates)
    }
    /// Evict all expired entries.
    #[allow(dead_code)]
    pub fn evict_expired(&mut self) {
        self.entries.retain(|_, e| e.is_valid());
    }
    /// Return the number of cached entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return whether the cache is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// Information about the detected shell environment.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ShellEnvironment {
    pub kind: Option<ShellKind>,
    pub version: Option<String>,
    pub config_dir: Option<String>,
    pub completion_dir: Option<String>,
}
impl ShellEnvironment {
    /// Detect the shell environment from system state.
    #[allow(dead_code)]
    pub fn detect() -> Self {
        let kind = detect_shell();
        let version = Self::detect_version(&kind);
        let config_dir = Self::detect_config_dir(&kind);
        let completion_dir = kind
            .as_ref()
            .and_then(CompletionInstallTarget::default_user_path);
        Self {
            kind,
            version,
            config_dir,
            completion_dir,
        }
    }
    fn detect_version(kind: &Option<ShellKind>) -> Option<String> {
        let shell_name = match kind {
            Some(ShellKind::Bash) => "bash",
            Some(ShellKind::Zsh) => "zsh",
            Some(ShellKind::Fish) => "fish",
            _ => return None,
        };
        let output = std::process::Command::new(shell_name)
            .arg("--version")
            .output()
            .ok()?;
        let s = String::from_utf8_lossy(&output.stdout);
        s.lines().next().map(|l| l.to_string())
    }
    fn detect_config_dir(kind: &Option<ShellKind>) -> Option<String> {
        let home = std::env::var("HOME").ok()?;
        match kind {
            Some(ShellKind::Bash) => Some(home),
            Some(ShellKind::Zsh) => Some(home),
            Some(ShellKind::Fish) => Some(format!("{}/.config/fish", home)),
            Some(ShellKind::PowerShell) => std::env::var("USERPROFILE").ok(),
            _ => None,
        }
    }
}
/// Output format for completion results.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompletionOutputFormat {
    /// One completion per line
    Lines,
    /// JSON array
    Json,
    /// Zsh array format
    ZshArray,
    /// Fish format (text\tdesc)
    Fish,
}
/// Statistics about completion performance.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct CompletionStats {
    pub total_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub avg_candidates: f64,
    pub avg_latency_us: f64,
}
impl CompletionStats {
    /// Create new zeroed stats.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a cache hit.
    #[allow(dead_code)]
    pub fn record_hit(&mut self) {
        self.total_requests += 1;
        self.cache_hits += 1;
    }
    /// Record a cache miss with the number of candidates returned and latency in microseconds.
    #[allow(dead_code)]
    pub fn record_miss(&mut self, candidate_count: usize, latency_us: u64) {
        self.total_requests += 1;
        self.cache_misses += 1;
        let n = self.cache_misses as f64;
        self.avg_candidates = (self.avg_candidates * (n - 1.0) + candidate_count as f64) / n;
        self.avg_latency_us = (self.avg_latency_us * (n - 1.0) + latency_us as f64) / n;
    }
    /// Return the cache hit rate as a percentage.
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            100.0 * self.cache_hits as f64 / self.total_requests as f64
        }
    }
}
/// Completion item kind (mirrors LSP CompletionItemKind).
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CompletionItemKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Unit,
    Value,
    Enum,
    Keyword,
    Snippet,
    Color,
    File,
    Reference,
    Folder,
    EnumMember,
    Constant,
    Struct,
    Event,
    Operator,
    TypeParameter,
}
/// Where to install a generated completion script.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum CompletionInstallTarget {
    /// A system-wide directory (e.g., /etc/bash_completion.d/)
    SystemDir(String),
    /// A user-specific directory (e.g., ~/.local/share/bash-completion/completions/)
    UserDir(String),
    /// Print to stdout
    Stdout,
    /// Write to a custom path
    CustomPath(String),
}
impl CompletionInstallTarget {
    /// Return the path string if this target has one.
    #[allow(dead_code)]
    pub fn path(&self) -> Option<&str> {
        match self {
            Self::SystemDir(p) | Self::UserDir(p) | Self::CustomPath(p) => Some(p.as_str()),
            Self::Stdout => None,
        }
    }
    /// Return the default system install path for a shell.
    #[allow(dead_code)]
    pub fn default_system_path(shell: &ShellKind) -> Option<&'static str> {
        match shell {
            ShellKind::Bash => Some("/etc/bash_completion.d/"),
            ShellKind::Zsh => Some("/usr/share/zsh/site-functions/"),
            ShellKind::Fish => Some("/usr/share/fish/completions/"),
            ShellKind::PowerShell => None,
            ShellKind::Elvish => None,
        }
    }
    /// Return the default user install path for a shell.
    #[allow(dead_code)]
    pub fn default_user_path(shell: &ShellKind) -> Option<String> {
        let home = std::env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));
        match shell {
            ShellKind::Bash => Some(format!(
                "{}/.local/share/bash-completion/completions/",
                home
            )),
            ShellKind::Zsh => Some(format!("{}/.zsh/completions/", home)),
            ShellKind::Fish => Some(format!("{}/.config/fish/completions/", home)),
            ShellKind::PowerShell => None,
            ShellKind::Elvish => Some(format!("{}/.elvish/completions/", home)),
        }
    }
}
/// Drives completions using a spec and dynamic registry.
#[allow(dead_code)]
pub struct CompletionEngine<'a> {
    spec: &'a AppCompletionSpec,
    registry: &'a DynamicCompletionRegistry,
}
impl<'a> CompletionEngine<'a> {
    /// Create a new completion engine.
    #[allow(dead_code)]
    pub fn new(spec: &'a AppCompletionSpec, registry: &'a DynamicCompletionRegistry) -> Self {
        Self { spec, registry }
    }
    /// Produce completion candidates for the given context.
    #[allow(dead_code)]
    pub fn complete(&self, ctx: &CompletionContext) -> Vec<CompletionCandidate> {
        if ctx.cursor_position <= 1 {
            let partial = &ctx.current_word;
            let mut cands: Vec<CompletionCandidate> = self
                .spec
                .subcommands
                .iter()
                .filter(|s| s.name.starts_with(partial.as_str()))
                .map(|s| CompletionCandidate::new(s.name.clone(), s.description.clone()))
                .collect();
            for f in &self.spec.global_flags {
                if f.long.starts_with(partial.as_str()) {
                    cands.push(CompletionCandidate::new(
                        f.long.clone(),
                        f.description.clone(),
                    ));
                }
            }
            return cands;
        }
        let Some(ref subcmd_name) = ctx.active_subcommand else {
            return vec![];
        };
        let partial = &ctx.current_word;
        let dynamic = self.registry.complete(partial, partial);
        if !dynamic.is_empty() {
            return dynamic;
        }
        if let Some(subcmd) = self.spec.find_subcommand(subcmd_name) {
            subcmd
                .flags
                .iter()
                .filter(|f| f.long.starts_with(partial.as_str()))
                .map(|f| CompletionCandidate::new(f.long.clone(), f.description.clone()))
                .collect()
        } else {
            vec![]
        }
    }
}
/// A cache entry for completion results.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CompletionCacheEntry {
    pub key: String,
    pub candidates: Vec<CompletionCandidate>,
    pub timestamp: std::time::Instant,
    pub ttl_ms: u64,
}
impl CompletionCacheEntry {
    /// Check whether this cache entry is still valid.
    #[allow(dead_code)]
    pub fn is_valid(&self) -> bool {
        let elapsed = self.timestamp.elapsed().as_millis() as u64;
        elapsed < self.ttl_ms
    }
}
/// A middleware layer that wraps a completion engine with caching, history, and stats.
#[allow(dead_code)]
pub struct CompletionMiddleware<'a> {
    engine: CompletionEngine<'a>,
    cache: CompletionCache,
    history: CompletionHistory,
    stats: CompletionStats,
}
impl<'a> CompletionMiddleware<'a> {
    /// Create a new middleware wrapper.
    #[allow(dead_code)]
    pub fn new(engine: CompletionEngine<'a>) -> Self {
        Self {
            engine,
            cache: CompletionCache::new(256),
            history: CompletionHistory::new(1000),
            stats: CompletionStats::new(),
        }
    }
    /// Complete with caching and stats tracking.
    #[allow(dead_code)]
    pub fn complete(&mut self, ctx: &CompletionContext) -> Vec<CompletionCandidate> {
        let key = format!("{}:{}", ctx.current_word, ctx.cursor_position);
        if let Some(cached) = self.cache.get(&key) {
            self.stats.record_hit();
            return cached.clone();
        }
        let start = std::time::Instant::now();
        let candidates = self.engine.complete(ctx);
        let latency_us = start.elapsed().as_micros() as u64;
        self.stats.record_miss(candidates.len(), latency_us);
        self.cache.insert(key, candidates.clone(), 5000);
        candidates
    }
    /// Record that a candidate was selected.
    #[allow(dead_code)]
    pub fn record_selection(
        &mut self,
        input: String,
        selected: String,
        subcommand: Option<String>,
    ) {
        self.history.record(input, selected, subcommand);
    }
    /// Get stats.
    #[allow(dead_code)]
    pub fn stats(&self) -> &CompletionStats {
        &self.stats
    }
}
/// A history entry recording a past completion selection.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CompletionHistoryEntry {
    pub input: String,
    pub selected: String,
    pub subcommand: Option<String>,
    pub count: u32,
}
/// A registry for dynamic (runtime) completion providers.
#[allow(dead_code)]
pub struct DynamicCompletionRegistry {
    providers: Vec<Box<dyn DynamicCompletionProvider>>,
}
impl DynamicCompletionRegistry {
    /// Create a new empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { providers: vec![] }
    }
    /// Register a completion provider.
    #[allow(dead_code)]
    pub fn register(&mut self, provider: Box<dyn DynamicCompletionProvider>) {
        self.providers.push(provider);
    }
    /// Get completions for an argument and partial input.
    #[allow(dead_code)]
    pub fn complete(&self, argument: &str, partial: &str) -> Vec<CompletionCandidate> {
        let mut results: Vec<CompletionCandidate> = self
            .providers
            .iter()
            .filter(|p| p.handles_argument() == argument)
            .flat_map(|p| p.candidates(partial))
            .collect();
        results.sort_by_key(|c| c.priority);
        results
    }
}
/// A rich specification for a subcommand.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SubcommandSpec {
    /// Subcommand name
    pub name: String,
    /// Aliases for this subcommand
    pub aliases: Vec<String>,
    /// Description
    pub description: String,
    /// Flags specific to this subcommand
    pub flags: Vec<CompletionSpec>,
    /// Whether this subcommand accepts file arguments
    pub accepts_files: bool,
}
impl SubcommandSpec {
    /// Create a new subcommand spec.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aliases: vec![],
            description: description.into(),
            flags: vec![],
            accepts_files: false,
        }
    }
    /// Add an alias for this subcommand.
    #[allow(dead_code)]
    pub fn with_alias(mut self, alias: impl Into<String>) -> Self {
        self.aliases.push(alias.into());
        self
    }
    /// Add a flag to this subcommand.
    #[allow(dead_code)]
    pub fn with_flag(mut self, flag: CompletionSpec) -> Self {
        self.flags.push(flag);
        self
    }
    /// Mark this subcommand as accepting file arguments.
    #[allow(dead_code)]
    pub fn accepts_files(mut self) -> Self {
        self.accepts_files = true;
        self
    }
}
/// Supported shell types for completion generation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ShellKind {
    /// Bash shell (Bourne Again Shell).
    Bash,
    /// Zsh shell (Z Shell).
    Zsh,
    /// Fish shell (Friendly Interactive Shell).
    Fish,
    /// PowerShell (Windows and cross-platform).
    PowerShell,
    /// Elvish shell.
    Elvish,
}
impl ShellKind {
    /// Return the canonical name of the shell.
    pub fn name(&self) -> &'static str {
        match self {
            ShellKind::Bash => "bash",
            ShellKind::Zsh => "zsh",
            ShellKind::Fish => "fish",
            ShellKind::PowerShell => "powershell",
            ShellKind::Elvish => "elvish",
        }
    }
    /// Return the typical completion file extension for this shell.
    pub fn file_extension(&self) -> &'static str {
        match self {
            ShellKind::Bash => "bash",
            ShellKind::Zsh => "zsh",
            ShellKind::Fish => "fish",
            ShellKind::PowerShell => "ps1",
            ShellKind::Elvish => "elv",
        }
    }
}
/// A single candidate in a dynamic completion list.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CompletionCandidate {
    /// The completion text
    pub text: String,
    /// Short description shown alongside the candidate
    pub description: String,
    /// Sort priority (lower = earlier)
    pub priority: u32,
}
impl CompletionCandidate {
    /// Create a new completion candidate.
    #[allow(dead_code)]
    pub fn new(text: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            description: description.into(),
            priority: 100,
        }
    }
    /// Set the priority for sorting.
    #[allow(dead_code)]
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.priority = priority;
        self
    }
}
