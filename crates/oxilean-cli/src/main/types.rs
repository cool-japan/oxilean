//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::repl::Repl;

/// Diagnostic severity.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagSeverity {
    Error,
    Warning,
    Note,
    Hint,
}
/// Reporter for command-line diagnostic output.
#[allow(dead_code)]
pub struct CliDiagnosticsReporter {
    pub use_color: bool,
    pub compact: bool,
}
impl CliDiagnosticsReporter {
    /// Create a new reporter.
    #[allow(dead_code)]
    pub fn new(use_color: bool) -> Self {
        Self {
            use_color,
            compact: false,
        }
    }
    /// Format a count summary.
    #[allow(dead_code)]
    pub fn format_summary(&self, errors: usize, warnings: usize) -> String {
        if errors == 0 && warnings == 0 {
            "No errors or warnings.".to_string()
        } else {
            format!("{} error(s), {} warning(s)", errors, warnings)
        }
    }
}
/// Supported CLI commands (parsed from argv).
#[allow(dead_code)]
pub enum CliCommand {
    /// Start the interactive REPL.
    Repl,
    /// Check a single `.oxilean` file.
    Check(String),
    /// Show version information.
    Version,
    /// Show help message.
    Help,
    /// Build a project from a manifest file.
    Build { manifest: String },
    /// Run benchmarks.
    Bench { file: String },
    /// Generate documentation.
    Doc { output: String },
    /// Format a source file in-place.
    Fmt { file: String },
    /// Start the LSP language server.
    Lsp,
    /// Export declarations to JSON.
    Export { file: String, output: String },
    /// Unknown command with its name.
    Unknown(String),
}
#[allow(dead_code)]
impl CliCommand {
    /// Parse a `CliCommand` from a slice of argument strings.
    pub fn parse(args: &[String]) -> Self {
        let cmd = args.first().map(|s| s.as_str()).unwrap_or("");
        match cmd {
            "repl" | "r" => CliCommand::Repl,
            "check" | "c" => {
                let file = args.get(1).cloned().unwrap_or_default();
                CliCommand::Check(file)
            }
            "version" | "v" | "--version" | "-V" => CliCommand::Version,
            "help" | "h" | "--help" | "-h" => CliCommand::Help,
            "build" | "b" => {
                let manifest = args
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| "Oxilean.toml".to_string());
                CliCommand::Build { manifest }
            }
            "bench" => {
                let file = args.get(1).cloned().unwrap_or_default();
                CliCommand::Bench { file }
            }
            "doc" => {
                let output = args.get(1).cloned().unwrap_or_else(|| "docs".to_string());
                CliCommand::Doc { output }
            }
            "fmt" | "format" => {
                let file = args.get(1).cloned().unwrap_or_default();
                CliCommand::Fmt { file }
            }
            "lsp" => CliCommand::Lsp,
            "export" => {
                let file = args.get(1).cloned().unwrap_or_default();
                let output = args
                    .get(2)
                    .cloned()
                    .unwrap_or_else(|| "output.json".to_string());
                CliCommand::Export { file, output }
            }
            other => CliCommand::Unknown(other.to_string()),
        }
    }
    /// Return a one-line description of this command.
    pub fn description(&self) -> &'static str {
        match self {
            CliCommand::Repl => "Start the interactive REPL",
            CliCommand::Check(_) => "Type-check a .oxilean source file",
            CliCommand::Version => "Print version information",
            CliCommand::Help => "Print this help message",
            CliCommand::Build { .. } => "Build a project from its manifest",
            CliCommand::Bench { .. } => "Run benchmarks on a file",
            CliCommand::Doc { .. } => "Generate HTML documentation",
            CliCommand::Fmt { .. } => "Format a source file in-place",
            CliCommand::Lsp => "Start the LSP language server",
            CliCommand::Export { .. } => "Export declarations to JSON",
            CliCommand::Unknown(_) => "Unknown command",
        }
    }
    /// Return true if this command requires at least one file argument.
    pub fn requires_file(&self) -> bool {
        matches!(
            self,
            CliCommand::Check(_)
                | CliCommand::Bench { .. }
                | CliCommand::Fmt { .. }
                | CliCommand::Export { .. }
        )
    }
}
/// Parsed global CLI arguments.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct GlobalArgs {
    pub verbose: bool,
    pub quiet: bool,
    pub color: ColorChoice,
    pub log_level: String,
    pub config_path: Option<String>,
    pub no_config: bool,
}
/// Shell variants for completion generation.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}
#[allow(dead_code)]
impl Shell {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "bash" => Some(Shell::Bash),
            "zsh" => Some(Shell::Zsh),
            "fish" => Some(Shell::Fish),
            "powershell" | "ps" => Some(Shell::PowerShell),
            "elvish" => Some(Shell::Elvish),
            _ => None,
        }
    }
}
/// A CLI diagnostic message.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CliDiagnostic {
    pub severity: DiagSeverity,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub snippet: Option<String>,
    pub suggestion: Option<String>,
}
#[allow(dead_code)]
impl CliDiagnostic {
    pub fn error(message: impl Into<String>) -> Self {
        Self {
            severity: DiagSeverity::Error,
            message: message.into(),
            file: None,
            line: None,
            column: None,
            snippet: None,
            suggestion: None,
        }
    }
    pub fn warning(message: impl Into<String>) -> Self {
        Self {
            severity: DiagSeverity::Warning,
            message: message.into(),
            file: None,
            line: None,
            column: None,
            snippet: None,
            suggestion: None,
        }
    }
    pub fn at_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }
    pub fn at_location(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }
    pub fn with_snippet(mut self, snippet: impl Into<String>) -> Self {
        self.snippet = Some(snippet.into());
        self
    }
    pub fn with_suggestion(mut self, s: impl Into<String>) -> Self {
        self.suggestion = Some(s.into());
        self
    }
    pub fn format(&self, _color: bool) -> String {
        let location = match (&self.file, self.line, self.column) {
            (Some(f), Some(l), Some(c)) => format!("{}:{}:{}: ", f, l, c),
            (Some(f), Some(l), None) => format!("{}:{}: ", f, l),
            (Some(f), None, _) => format!("{}: ", f),
            _ => String::new(),
        };
        let mut lines = vec![format!("{}{}: {}", location, self.severity, self.message)];
        if let Some(ref s) = self.snippet {
            lines.push(format!("  | {}", s));
        }
        if let Some(ref s) = self.suggestion {
            lines.push(format!("  suggestion: {}", s));
        }
        lines.join("\n")
    }
}
/// The result of a CLI execution.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CliExecutionResult {
    pub exit_code: i32,
    pub output: String,
    pub duration_ms: u64,
}
impl CliExecutionResult {
    /// Create a successful result.
    #[allow(dead_code)]
    pub fn ok(output: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            exit_code: 0,
            output: output.into(),
            duration_ms,
        }
    }
    /// Create a failure result.
    #[allow(dead_code)]
    pub fn err(exit_code: i32, output: impl Into<String>, duration_ms: u64) -> Self {
        Self {
            exit_code,
            output: output.into(),
            duration_ms,
        }
    }
    /// Return whether the execution was successful.
    #[allow(dead_code)]
    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }
}
/// Color output choice.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ColorChoice {
    #[default]
    Auto,
    Always,
    Never,
}
impl ColorChoice {
    /// Parse from a string.
    #[allow(dead_code)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "auto" => Some(Self::Auto),
            "always" | "yes" => Some(Self::Always),
            "never" | "no" => Some(Self::Never),
            _ => None,
        }
    }
    /// Return true if color output should be enabled.
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Auto => atty_check(),
            Self::Always => true,
            Self::Never => false,
        }
    }
}
/// CLI build information.
#[allow(dead_code)]
pub struct CliBuildInfo {
    pub version: &'static str,
    pub git_commit: Option<&'static str>,
    pub build_date: &'static str,
    pub target_triple: &'static str,
    pub rustc_version: &'static str,
    pub profile: &'static str,
}
impl CliBuildInfo {
    /// Return the default build info.
    #[allow(dead_code)]
    pub fn current() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION"),
            git_commit: option_env!("GIT_COMMIT"),
            build_date: "2025-02-28",
            target_triple: std::env::consts::ARCH,
            rustc_version: "stable",
            profile: if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            },
        }
    }
    /// Format as a version string.
    #[allow(dead_code)]
    pub fn version_string(&self) -> String {
        match self.git_commit {
            Some(commit) => {
                format!("oxilean {} ({}) [{}]", self.version, commit, self.profile)
            }
            None => format!("oxilean {} [{}]", self.version, self.profile),
        }
    }
}
/// Detailed version information.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VersionInfo {
    pub version: String,
    pub git_hash: Option<String>,
    pub build_date: String,
    pub rustc_version: String,
    pub target: String,
}
#[allow(dead_code)]
impl VersionInfo {
    pub fn current() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            git_hash: option_env!("GIT_HASH").map(|s| s.to_string()),
            build_date: option_env!("BUILD_DATE").unwrap_or("unknown").to_string(),
            rustc_version: option_env!("RUSTC_VERSION")
                .unwrap_or("unknown")
                .to_string(),
            target: option_env!("TARGET")
                .unwrap_or(std::env::consts::ARCH)
                .to_string(),
        }
    }
    pub fn short_version(&self) -> String {
        format!("oxilean {}", self.version)
    }
    pub fn long_version(&self) -> String {
        let mut lines = vec![format!("oxilean {}", self.version)];
        if let Some(ref h) = self.git_hash {
            lines.push(format!("commit: {}", h));
        }
        lines.push(format!("build date: {}", self.build_date));
        lines.push(format!("target: {}", self.target));
        lines.join("\n")
    }
}
/// Global flags.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GlobalFlags {
    pub verbose: bool,
    pub quiet: bool,
    pub no_color: bool,
    pub jobs: usize,
    pub log_level: String,
    pub work_dir: Option<String>,
    pub telemetry: bool,
    pub check_updates: bool,
}
#[allow(dead_code)]
impl GlobalFlags {
    pub fn default_flags() -> Self {
        Self {
            verbose: false,
            quiet: false,
            no_color: false,
            jobs: num_cpus(),
            log_level: "warn".to_string(),
            work_dir: None,
            telemetry: false,
            check_updates: true,
        }
    }
    pub fn parse(args: &mut Vec<String>) -> Self {
        let mut flags = Self::default_flags();
        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--verbose" | "-v" => {
                    flags.verbose = true;
                    args.remove(i);
                }
                "--quiet" | "-q" => {
                    flags.quiet = true;
                    args.remove(i);
                }
                "--no-color" => {
                    flags.no_color = true;
                    args.remove(i);
                }
                "--telemetry" => {
                    flags.telemetry = true;
                    args.remove(i);
                }
                "--no-telemetry" => {
                    flags.telemetry = false;
                    args.remove(i);
                }
                "--no-update-check" => {
                    flags.check_updates = false;
                    args.remove(i);
                }
                arg if arg.starts_with("--jobs=") => {
                    if let Ok(n) = arg["--jobs=".len()..].parse::<usize>() {
                        flags.jobs = n;
                    }
                    args.remove(i);
                }
                arg if arg.starts_with("--log=") => {
                    flags.log_level = arg["--log=".len()..].to_string();
                    args.remove(i);
                }
                _ => {
                    i += 1;
                }
            }
        }
        flags
    }
    pub fn apply(&self) {
        if self.no_color {
            std::env::set_var("NO_COLOR", "1");
        }
        if self.verbose {
            std::env::set_var("OXILEAN_LOG", "debug");
        } else {
            std::env::set_var("OXILEAN_LOG", &self.log_level);
        }
    }
}
/// CLI environment configuration.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CliEnvironment {
    pub oxilean_home: Option<String>,
    pub oxilean_stdlib: Option<String>,
    pub oxilean_cache: Option<String>,
    pub editor: Option<String>,
    pub pager: Option<String>,
    pub no_color: bool,
    pub ci_mode: bool,
}
impl CliEnvironment {
    /// Detect environment from system variables.
    #[allow(dead_code)]
    pub fn detect() -> Self {
        Self {
            oxilean_home: std::env::var("OXILEAN_HOME").ok(),
            oxilean_stdlib: std::env::var("OXILEAN_STDLIB").ok(),
            oxilean_cache: std::env::var("OXILEAN_CACHE").ok(),
            editor: std::env::var("EDITOR").ok(),
            pager: std::env::var("PAGER").ok(),
            no_color: std::env::var("NO_COLOR").is_ok(),
            ci_mode: std::env::var("CI").is_ok(),
        }
    }
    /// Return whether interactive mode is appropriate.
    #[allow(dead_code)]
    pub fn is_interactive(&self) -> bool {
        !self.ci_mode
    }
}
/// A simple progress indicator for multi-step operations.
#[allow(dead_code)]
pub struct ProgressBar {
    total: usize,
    pub(crate) current: usize,
    label: String,
}
#[allow(dead_code)]
impl ProgressBar {
    /// Create a new progress bar with the given total.
    pub fn new(total: usize, label: impl Into<String>) -> Self {
        Self {
            total,
            current: 0,
            label: label.into(),
        }
    }
    /// Advance by one step.
    pub fn tick(&mut self) {
        self.current = (self.current + 1).min(self.total);
    }
    /// Advance by `n` steps.
    pub fn advance(&mut self, n: usize) {
        self.current = (self.current + n).min(self.total);
    }
    /// Return the current completion percentage (0–100).
    pub fn percent(&self) -> u32 {
        if self.total == 0 {
            return 100;
        }
        (self.current * 100 / self.total) as u32
    }
    /// Return true if complete.
    pub fn is_complete(&self) -> bool {
        self.current >= self.total
    }
    /// Format as a simple ASCII bar of given width.
    pub fn render(&self, bar_width: usize) -> String {
        let filled = if self.total == 0 {
            bar_width
        } else {
            self.current * bar_width / self.total
        };
        let empty = bar_width - filled;
        format!(
            "[{}{}] {}/{} {}",
            "#".repeat(filled),
            "-".repeat(empty),
            self.current,
            self.total,
            self.label
        )
    }
}
/// Global CLI configuration (flags and settings).
#[allow(dead_code)]
pub struct CliConfig {
    /// Whether verbose output is enabled (`--verbose` / `-v`).
    pub verbose: bool,
    /// Whether to suppress all output (`--quiet` / `-q`).
    pub quiet: bool,
    /// Maximum number of errors to report before stopping.
    pub max_errors: usize,
    /// Whether to use colored terminal output.
    pub color: bool,
    /// Output width for pretty-printing (0 = auto-detect).
    pub width: usize,
}
#[allow(dead_code)]
impl CliConfig {
    /// Create a default `CliConfig`.
    pub fn default_config() -> Self {
        CliConfig {
            verbose: false,
            quiet: false,
            max_errors: 100,
            color: true,
            width: 0,
        }
    }
    /// Parse flags from an argv slice (mutates the slice by removing recognized flags).
    pub fn parse_flags(args: &mut Vec<String>) -> Self {
        let mut cfg = Self::default_config();
        args.retain(|arg| match arg.as_str() {
            "--verbose" | "-v" => {
                cfg.verbose = true;
                false
            }
            "--quiet" | "-q" => {
                cfg.quiet = true;
                false
            }
            "--no-color" => {
                cfg.color = false;
                false
            }
            _ => true,
        });
        cfg
    }
    /// Print a verbose message if verbose mode is enabled.
    pub fn vlog(&self, msg: &str) {
        if self.verbose {
            eprintln!("[verbose] {msg}");
        }
    }
}
/// Extended subcommand set with more options.
#[allow(dead_code)]
pub enum ExtCommand {
    Init {
        name: String,
    },
    Test {
        filter: Option<String>,
    },
    Clean {
        profile: String,
    },
    Update,
    Deps,
    Search {
        pattern: String,
    },
    Proof {
        file: String,
    },
    Lint {
        paths: Vec<String>,
    },
    Report {
        output: String,
    },
    Watch {
        dirs: Vec<String>,
    },
    Benchmark {
        file: String,
        filter: Option<String>,
    },
    Diff {
        old: String,
        new: String,
    },
    Migrate {
        version: String,
    },
}
#[allow(dead_code)]
impl ExtCommand {
    pub fn parse(args: &[String]) -> Option<Self> {
        let cmd = args.first()?.as_str();
        match cmd {
            "init" => Some(ExtCommand::Init {
                name: args
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| "my-project".to_string()),
            }),
            "test" => Some(ExtCommand::Test {
                filter: args.get(1).cloned(),
            }),
            "clean" => Some(ExtCommand::Clean {
                profile: args.get(1).cloned().unwrap_or_else(|| "debug".to_string()),
            }),
            "update" => Some(ExtCommand::Update),
            "deps" => Some(ExtCommand::Deps),
            "search" => Some(ExtCommand::Search {
                pattern: args.get(1).cloned().unwrap_or_default(),
            }),
            "proof" => Some(ExtCommand::Proof {
                file: args.get(1).cloned().unwrap_or_default(),
            }),
            "lint" => Some(ExtCommand::Lint {
                paths: args[1..].to_vec(),
            }),
            "report" => Some(ExtCommand::Report {
                output: args
                    .get(1)
                    .cloned()
                    .unwrap_or_else(|| "report.html".to_string()),
            }),
            "watch" => Some(ExtCommand::Watch {
                dirs: if args.len() > 1 {
                    args[1..].to_vec()
                } else {
                    vec![".".to_string()]
                },
            }),
            "benchmark" => Some(ExtCommand::Benchmark {
                file: args.get(1).cloned().unwrap_or_default(),
                filter: args.get(2).cloned(),
            }),
            "diff" => Some(ExtCommand::Diff {
                old: args.get(1).cloned().unwrap_or_default(),
                new: args.get(2).cloned().unwrap_or_default(),
            }),
            "migrate" => Some(ExtCommand::Migrate {
                version: args.get(1).cloned().unwrap_or_default(),
            }),
            _ => None,
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            ExtCommand::Init { .. } => "Initialize a new project",
            ExtCommand::Test { .. } => "Run project tests",
            ExtCommand::Clean { .. } => "Clean build artifacts",
            ExtCommand::Update => "Upgrade dependencies",
            ExtCommand::Deps => "Show dependency tree",
            ExtCommand::Search { .. } => "Search declarations",
            ExtCommand::Proof { .. } => "Interactive proof session",
            ExtCommand::Lint { .. } => "Lint source files",
            ExtCommand::Report { .. } => "Generate project report",
            ExtCommand::Watch { .. } => "Watch and auto-rebuild",
            ExtCommand::Benchmark { .. } => "Run benchmarks",
            ExtCommand::Diff { .. } => "Diff two versions",
            ExtCommand::Migrate { .. } => "Migrate to new syntax",
        }
    }
    pub fn is_mutating(&self) -> bool {
        matches!(
            self,
            ExtCommand::Init { .. }
                | ExtCommand::Clean { .. }
                | ExtCommand::Update
                | ExtCommand::Migrate { .. }
        )
    }
}
/// Telemetry configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    pub enabled: bool,
    pub session_id: String,
    pub endpoint: String,
}
#[allow(dead_code)]
impl TelemetryConfig {
    pub fn default_config() -> Self {
        Self {
            enabled: false,
            session_id: generate_session_id(),
            endpoint: String::new(),
        }
    }
    pub fn opt_in(mut self) -> Self {
        self.enabled = true;
        self
    }
    pub fn opt_out(mut self) -> Self {
        self.enabled = false;
        self
    }
    pub fn effective_endpoint(&self) -> &str {
        if self.enabled {
            &self.endpoint
        } else {
            ""
        }
    }
}
/// A subcommand name with aliases.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SubcommandEntry {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub description: &'static str,
}
/// The result of a CLI operation.
#[allow(dead_code)]
pub enum CliResult {
    /// Success (exit code 0).
    Ok,
    /// Failure with a message (exit code 1).
    Err(String),
    /// Failure with a specific exit code.
    Exit(i32),
}
#[allow(dead_code)]
impl CliResult {
    /// Convert to a process exit code.
    pub fn exit_code(&self) -> i32 {
        match self {
            CliResult::Ok => 0,
            CliResult::Err(_) => 1,
            CliResult::Exit(code) => *code,
        }
    }
    /// Return true if the result is a success.
    pub fn is_ok(&self) -> bool {
        matches!(self, CliResult::Ok)
    }
}
