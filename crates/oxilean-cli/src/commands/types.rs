//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::path::{Path, PathBuf};
use std::time::Instant;

use super::functions::*;

use std::collections::HashMap;

/// Logs command execution output.
#[allow(dead_code)]
pub struct CommandLogger {
    pub level: LogLevel,
    pub prefix: String,
}
impl CommandLogger {
    /// Create a new logger.
    #[allow(dead_code)]
    pub fn new(level: LogLevel) -> Self {
        Self {
            level,
            prefix: String::new(),
        }
    }
    /// Set a log prefix.
    #[allow(dead_code)]
    pub fn with_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = prefix.into();
        self
    }
    /// Log a message at the given level.
    #[allow(dead_code)]
    pub fn log(&self, level: &LogLevel, msg: &str) -> Option<String> {
        if level <= &self.level {
            let prefix = if self.prefix.is_empty() {
                String::new()
            } else {
                format!("[{}] ", self.prefix)
            };
            Some(format!("{}{}", prefix, msg))
        } else {
            None
        }
    }
    /// Log an error.
    #[allow(dead_code)]
    pub fn error(&self, msg: &str) -> Option<String> {
        self.log(&LogLevel::Error, msg)
    }
    /// Log info.
    #[allow(dead_code)]
    pub fn info(&self, msg: &str) -> Option<String> {
        self.log(&LogLevel::Info, msg)
    }
}
/// Value of a command argument.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum CommandArgValue {
    Bool(bool),
    String(String),
    Integer(i64),
    Float(f64),
    Strings(Vec<String>),
}
impl CommandArgValue {
    /// Return as a string if this is a string value.
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s.as_str()),
            _ => None,
        }
    }
    /// Return as a bool.
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }
    /// Return as an integer.
    #[allow(dead_code)]
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Integer(n) => Some(*n),
            _ => None,
        }
    }
}
/// Options for formatting.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FormatCommandOptions {
    /// Write changes in place
    pub in_place: bool,
    /// Check if formatting is needed without writing
    pub check: bool,
    /// Show diff for changes
    pub diff: bool,
    /// Include directories recursively
    pub recursive: bool,
}
/// Exit code for commands.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ExitCode {
    /// Success (0)
    Success = 0,
    /// General error (1)
    Error = 1,
    /// Usage/argument error (2)
    Usage = 2,
    /// File not found (3)
    NotFound = 3,
    /// Permission denied (4)
    PermissionDenied = 4,
}
impl ExitCode {
    /// Convert to u32.
    pub fn as_u32(self) -> u32 {
        self as u32
    }
}
/// Command dispatcher for routing CLI operations.
#[allow(dead_code)]
pub struct CommandDispatcher {
    pub(crate) config: CommandConfig,
}
#[allow(dead_code)]
impl CommandDispatcher {
    /// Create a new command dispatcher.
    pub fn new(config: CommandConfig) -> Self {
        Self { config }
    }
    /// Dispatch a command by name.
    pub fn dispatch(&self, command: &str, args: &[String]) -> CommandResult<String> {
        match command {
            "check" => self.dispatch_check(args),
            "build" => self.dispatch_build(args),
            "test" => self.dispatch_test(args),
            "run" => self.dispatch_run(args),
            "fmt" => self.dispatch_format(args),
            "doc" => self.dispatch_doc(args),
            "clean" => self.dispatch_clean(args),
            _ => Err(CommandError::usage(format!("Unknown command: {}", command))),
        }
    }
    fn dispatch_check(&self, args: &[String]) -> CommandResult<String> {
        if args.is_empty() {
            return Err(CommandError::usage("check requires a file path"));
        }
        let path = PathBuf::from(&args[0]);
        check_file(&path, self.config.verbose)?;
        Ok("File checked successfully".to_string())
    }
    fn dispatch_build(&self, args: &[String]) -> CommandResult<String> {
        let release = args.contains(&"--release".to_string());
        let project_dir = if args.is_empty() {
            PathBuf::from(".")
        } else {
            PathBuf::from(&args[0])
        };
        build_project(&project_dir, release, self.config.verbose)?;
        Ok("Build completed successfully".to_string())
    }
    fn dispatch_test(&self, args: &[String]) -> CommandResult<String> {
        let filter = args.first().map(|s| s.as_str());
        let project_dir = if args.len() > 1 {
            PathBuf::from(&args[1])
        } else {
            PathBuf::from(".")
        };
        let count = run_tests(&project_dir, filter, self.config.verbose)?;
        Ok(format!("{} test(s) passed", count))
    }
    fn dispatch_run(&self, args: &[String]) -> CommandResult<String> {
        if args.is_empty() {
            return Err(CommandError::usage("run requires a file path"));
        }
        let path = PathBuf::from(&args[0]);
        run_script(&path, self.config.verbose)?;
        Ok("Script executed successfully".to_string())
    }
    fn dispatch_format(&self, args: &[String]) -> CommandResult<String> {
        let in_place = args.contains(&"--in-place".to_string());
        let check_only = args.contains(&"--check".to_string());
        let paths: Vec<PathBuf> = args
            .iter()
            .filter(|s| !s.starts_with("--"))
            .map(PathBuf::from)
            .collect();
        let count = format_files(&paths, in_place, check_only, self.config.verbose)?;
        Ok(format!("{} file(s) formatted", count))
    }
    fn dispatch_doc(&self, args: &[String]) -> CommandResult<String> {
        let project_dir = if args.is_empty() {
            PathBuf::from(".")
        } else {
            PathBuf::from(&args[0])
        };
        let output_dir = args.get(1).map(PathBuf::from);
        generate_docs(&project_dir, output_dir.as_deref(), self.config.verbose)?;
        Ok("Documentation generated successfully".to_string())
    }
    fn dispatch_clean(&self, args: &[String]) -> CommandResult<String> {
        let project_dir = if args.is_empty() {
            PathBuf::from(".")
        } else {
            PathBuf::from(&args[0])
        };
        clean_project(&project_dir, self.config.verbose)?;
        Ok("Project cleaned successfully".to_string())
    }
}
/// Error type for commands.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CommandError {
    pub code: ExitCode,
    pub message: String,
}
impl CommandError {
    /// Create a new command error.
    pub fn new(code: ExitCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
    /// Create a general error.
    pub fn general(message: impl Into<String>) -> Self {
        Self::new(ExitCode::Error, message)
    }
    /// Create a usage error.
    pub fn usage(message: impl Into<String>) -> Self {
        Self::new(ExitCode::Usage, message)
    }
    /// Create a file not found error.
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(ExitCode::NotFound, message)
    }
    /// Create a permission denied error.
    pub fn permission_denied(message: impl Into<String>) -> Self {
        Self::new(ExitCode::PermissionDenied, message)
    }
}
/// Output from a command execution.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CommandOutput {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub duration_ms: u64,
}
impl CommandOutput {
    /// Create a successful output.
    #[allow(dead_code)]
    pub fn success(stdout: impl Into<String>) -> Self {
        Self {
            success: true,
            exit_code: 0,
            stdout: stdout.into(),
            stderr: String::new(),
            duration_ms: 0,
        }
    }
    /// Create a failure output.
    #[allow(dead_code)]
    pub fn failure(exit_code: i32, stderr: impl Into<String>) -> Self {
        Self {
            success: false,
            exit_code,
            stdout: String::new(),
            stderr: stderr.into(),
            duration_ms: 0,
        }
    }
    /// Return true if the command succeeded.
    #[allow(dead_code)]
    pub fn is_success(&self) -> bool {
        self.success
    }
}
/// Registry of all available commands.
#[allow(dead_code)]
pub struct CommandRegistry {
    commands: Vec<CommandMetadata>,
}
impl CommandRegistry {
    /// Create a new empty registry.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { commands: vec![] }
    }
    /// Create a registry with standard OxiLean commands.
    #[allow(dead_code)]
    pub fn standard() -> Self {
        let mut reg = Self::new();
        reg.register(CommandMetadata::check_command());
        reg.register(CommandMetadata::build_command());
        reg.register(CommandMetadata::format_command());
        reg
    }
    /// Register a command.
    #[allow(dead_code)]
    pub fn register(&mut self, meta: CommandMetadata) {
        self.commands.push(meta);
    }
    /// Find a command by name or alias.
    #[allow(dead_code)]
    pub fn find(&self, name: &str) -> Option<&CommandMetadata> {
        self.commands
            .iter()
            .find(|c| c.name == name || c.aliases.iter().any(|a| a == name))
    }
    /// Return all commands in a category.
    #[allow(dead_code)]
    pub fn by_category(&self, category: &CommandCategory) -> Vec<&CommandMetadata> {
        self.commands
            .iter()
            .filter(|c| &c.category == category)
            .collect()
    }
    /// Return all registered command names.
    #[allow(dead_code)]
    pub fn command_names(&self) -> Vec<&str> {
        self.commands.iter().map(|c| c.name.as_str()).collect()
    }
}
/// Metadata about a registered command.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CommandMetadata {
    pub name: String,
    pub aliases: Vec<String>,
    pub description: String,
    pub usage: String,
    pub examples: Vec<String>,
    pub category: CommandCategory,
    pub flags: Vec<CommandFlagMeta>,
}
impl CommandMetadata {
    /// Create metadata for the check command.
    #[allow(dead_code)]
    pub fn check_command() -> Self {
        Self {
            name: "check".to_string(),
            aliases: vec!["c".to_string()],
            description: "Type-check and verify an OxiLean source file".to_string(),
            usage: "oxilean check [OPTIONS] <FILE>".to_string(),
            examples: vec![
                "oxilean check src/main.lean".to_string(),
                "oxilean check --verbose src/proof.lean".to_string(),
            ],
            category: CommandCategory::Development,
            flags: vec![
                CommandFlagMeta::bool_flag("--verbose", Some("-v"), "Enable verbose output"),
                CommandFlagMeta::value_flag("--output", Some("-o"), "Output format (text|json)"),
                CommandFlagMeta::bool_flag("--no-deps", None, "Skip dependency checking"),
            ],
        }
    }
    /// Create metadata for the build command.
    #[allow(dead_code)]
    pub fn build_command() -> Self {
        Self {
            name: "build".to_string(),
            aliases: vec!["b".to_string()],
            description: "Build the OxiLean project".to_string(),
            usage: "oxilean build [OPTIONS]".to_string(),
            examples: vec![
                "oxilean build".to_string(),
                "oxilean build --release".to_string(),
            ],
            category: CommandCategory::Development,
            flags: vec![
                CommandFlagMeta::bool_flag("--release", None, "Build in release mode"),
                CommandFlagMeta::value_flag("--jobs", Some("-j"), "Number of parallel jobs"),
            ],
        }
    }
    /// Create metadata for the format command.
    #[allow(dead_code)]
    pub fn format_command() -> Self {
        Self {
            name: "format".to_string(),
            aliases: vec!["fmt".to_string()],
            description: "Format OxiLean source files".to_string(),
            usage: "oxilean format [OPTIONS] [FILE...]".to_string(),
            examples: vec![
                "oxilean format src/".to_string(),
                "oxilean format --check src/proof.lean".to_string(),
            ],
            category: CommandCategory::Tools,
            flags: vec![
                CommandFlagMeta::bool_flag("--check", None, "Check only, don't write"),
                CommandFlagMeta::bool_flag("--diff", None, "Show diff"),
            ],
        }
    }
}
/// Progress reporter for long-running operations.
#[allow(dead_code)]
pub struct ProgressReporter {
    pub(crate) message: String,
    start_time: Instant,
    pub(crate) verbose: bool,
}
#[allow(dead_code)]
impl ProgressReporter {
    /// Create a new progress reporter.
    pub fn new(message: impl Into<String>, verbose: bool) -> Self {
        let msg = message.into();
        if verbose {
            eprintln!("{}", msg);
        }
        Self {
            message: msg,
            start_time: Instant::now(),
            verbose,
        }
    }
    /// Report progress.
    pub fn progress(&self, detail: &str) {
        if self.verbose {
            eprintln!("  {}", detail);
        }
    }
    /// Report completion.
    pub fn complete(&self) {
        if self.verbose {
            let elapsed = self.start_time.elapsed();
            eprintln!("  Completed in {:.2}s", elapsed.as_secs_f64());
        }
    }
    /// Report error.
    pub fn error(&self, error: &str) {
        if self.verbose {
            eprintln!("  Error: {}", error);
        }
    }
}
/// Options for running tests.
#[derive(Debug, Clone)]
#[allow(dead_code)]
#[derive(Default)]
pub struct TestOptions {
    /// Test filter
    pub filter: Option<String>,
    /// Show test output
    pub show_output: bool,
    /// Run tests sequentially
    pub sequential: bool,
    /// Number of parallel test jobs
    pub jobs: Option<usize>,
}
/// Metadata for a command flag.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CommandFlagMeta {
    pub long: String,
    pub short: Option<String>,
    pub description: String,
    pub takes_value: bool,
    pub required: bool,
}
impl CommandFlagMeta {
    /// Create a simple boolean flag.
    #[allow(dead_code)]
    pub fn bool_flag(
        long: impl Into<String>,
        short: Option<&str>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            long: long.into(),
            short: short.map(String::from),
            description: description.into(),
            takes_value: false,
            required: false,
        }
    }
    /// Create a flag that takes a value.
    #[allow(dead_code)]
    pub fn value_flag(
        long: impl Into<String>,
        short: Option<&str>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            long: long.into(),
            short: short.map(String::from),
            description: description.into(),
            takes_value: true,
            required: false,
        }
    }
}
/// A parsed command argument.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CommandArg {
    pub name: String,
    pub value: CommandArgValue,
}
/// Context for maintaining state across command execution.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CommandContext {
    /// Current configuration
    config: CommandConfig,
    /// Accumulated errors
    errors: Vec<CommandError>,
    /// Accumulated warnings
    warnings: Vec<String>,
}
#[allow(dead_code)]
impl CommandContext {
    /// Create a new command context.
    pub fn new(config: CommandConfig) -> Self {
        Self {
            config,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    /// Add an error to the context.
    pub fn add_error(&mut self, error: CommandError) {
        if self.errors.len() < self.config.max_errors {
            self.errors.push(error);
        }
    }
    /// Add a warning to the context.
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
    /// Get accumulated errors.
    pub fn errors(&self) -> &[CommandError] {
        &self.errors
    }
    /// Get accumulated warnings.
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
    /// Check if there are errors.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    /// Check if there are warnings.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}
/// Formats help text for commands.
#[allow(dead_code)]
pub struct CommandHelpFormatter;
impl CommandHelpFormatter {
    /// Format help text for a command.
    #[allow(dead_code)]
    pub fn format(meta: &CommandMetadata) -> String {
        let mut out = format!("{}\n\nUSAGE:\n    {}\n", meta.description, meta.usage);
        if !meta.aliases.is_empty() {
            out.push_str(&format!("\nALIASES:\n    {}\n", meta.aliases.join(", ")));
        }
        if !meta.flags.is_empty() {
            out.push_str("\nFLAGS:\n");
            for flag in &meta.flags {
                let short_str = flag
                    .short
                    .as_deref()
                    .map(|s| format!("{}, ", s))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "    {}{:<25} {}\n",
                    short_str, flag.long, flag.description
                ));
            }
        }
        if !meta.examples.is_empty() {
            out.push_str("\nEXAMPLES:\n");
            for example in &meta.examples {
                out.push_str(&format!("    {}\n", example));
            }
        }
        out
    }
    /// Format a short summary for all commands.
    #[allow(dead_code)]
    pub fn format_summary(registry: &CommandRegistry) -> String {
        let mut out = String::from("Available commands:\n\n");
        for cmd in &registry.commands {
            out.push_str(&format!("  {:<15} {}\n", cmd.name, cmd.description));
        }
        out
    }
}
/// Statistics for command executions.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct CommandStats {
    pub total_invocations: u64,
    pub successful: u64,
    pub failed: u64,
    pub total_duration_ms: u64,
}
impl CommandStats {
    /// Record an invocation.
    #[allow(dead_code)]
    pub fn record(&mut self, success: bool, duration_ms: u64) {
        self.total_invocations += 1;
        if success {
            self.successful += 1;
        } else {
            self.failed += 1;
        }
        self.total_duration_ms += duration_ms;
    }
    /// Return average duration in milliseconds.
    #[allow(dead_code)]
    pub fn avg_duration_ms(&self) -> f64 {
        if self.total_invocations == 0 {
            0.0
        } else {
            self.total_duration_ms as f64 / self.total_invocations as f64
        }
    }
    /// Return success rate as a percentage.
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_invocations == 0 {
            0.0
        } else {
            100.0 * self.successful as f64 / self.total_invocations as f64
        }
    }
}
/// Build options for customizing the build process.
#[derive(Debug, Clone)]
#[allow(dead_code)]
#[derive(Default)]
pub struct BuildOptions {
    /// Release mode (optimized)
    pub release: bool,
    /// Number of parallel jobs
    pub jobs: Option<usize>,
    /// Target triple for cross-compilation
    pub target: Option<String>,
    /// Build with all tests
    pub tests: bool,
    /// Build documentation
    pub docs: bool,
    /// Keep artifacts for debugging
    pub keep_artifacts: bool,
}
/// Category for a command.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandCategory {
    Development,
    Analysis,
    Documentation,
    Tools,
    Server,
    Advanced,
}
/// Configuration for a command run.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct CommandConfig {
    /// Enable verbose output
    pub verbose: bool,
    /// Enable color output
    pub color: bool,
    /// Project directory
    pub project_dir: PathBuf,
    /// Maximum errors to display
    pub max_errors: usize,
}
/// A parsed command invocation.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ParsedCommand {
    pub name: String,
    pub args: Vec<CommandArg>,
    pub positional: Vec<String>,
    pub raw_args: Vec<String>,
}
impl ParsedCommand {
    /// Find an argument by name.
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<&CommandArgValue> {
        self.args.iter().find(|a| a.name == name).map(|a| &a.value)
    }
    /// Check if a boolean flag is set.
    #[allow(dead_code)]
    pub fn has_flag(&self, name: &str) -> bool {
        self.get(name).and_then(|v| v.as_bool()).unwrap_or(false)
    }
    /// Get a string argument.
    #[allow(dead_code)]
    pub fn get_str(&self, name: &str) -> Option<&str> {
        self.get(name)?.as_str()
    }
    /// Get the first positional argument.
    #[allow(dead_code)]
    pub fn first_positional(&self) -> Option<&str> {
        self.positional.first().map(|s| s.as_str())
    }
}
/// Simple command-line argument parser.
#[allow(dead_code)]
pub struct CommandArgParser;
impl CommandArgParser {
    /// Parse a list of raw arguments for a named command.
    #[allow(dead_code)]
    pub fn parse(command_name: impl Into<String>, raw: &[&str]) -> ParsedCommand {
        let mut args: Vec<CommandArg> = vec![];
        let mut positional: Vec<String> = vec![];
        let mut i = 0;
        while i < raw.len() {
            let arg = raw[i];
            if let Some(long) = arg.strip_prefix("--") {
                if let Some(next) = raw.get(i + 1) {
                    if !next.starts_with('-') {
                        args.push(CommandArg {
                            name: format!("--{}", long),
                            value: CommandArgValue::String(next.to_string()),
                        });
                        i += 2;
                        continue;
                    }
                }
                args.push(CommandArg {
                    name: format!("--{}", long),
                    value: CommandArgValue::Bool(true),
                });
            } else if let Some(short) = arg.strip_prefix('-').filter(|s| s.len() == 1) {
                if let Some(next) = raw.get(i + 1) {
                    if !next.starts_with('-') {
                        args.push(CommandArg {
                            name: format!("-{}", short),
                            value: CommandArgValue::String(next.to_string()),
                        });
                        i += 2;
                        continue;
                    }
                }
                args.push(CommandArg {
                    name: format!("-{}", short),
                    value: CommandArgValue::Bool(true),
                });
            } else {
                positional.push(arg.to_string());
            }
            i += 1;
        }
        ParsedCommand {
            name: command_name.into(),
            args,
            positional,
            raw_args: raw.iter().map(|s| s.to_string()).collect(),
        }
    }
}
/// A step in a command pipeline.
#[allow(dead_code)]
pub struct CommandPipelineStep {
    pub name: String,
    pub args: Vec<String>,
}
impl CommandPipelineStep {
    /// Create a new step.
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, args: Vec<String>) -> Self {
        Self {
            name: name.into(),
            args,
        }
    }
}
/// Log level for command output.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}
/// A pipeline that runs commands in sequence.
#[allow(dead_code)]
pub struct CommandPipeline {
    steps: Vec<CommandPipelineStep>,
    pub(crate) stop_on_failure: bool,
}
impl CommandPipeline {
    /// Create a new pipeline.
    #[allow(dead_code)]
    pub fn new(stop_on_failure: bool) -> Self {
        Self {
            steps: vec![],
            stop_on_failure,
        }
    }
    /// Add a step.
    #[allow(dead_code)]
    pub fn add_step(mut self, step: CommandPipelineStep) -> Self {
        self.steps.push(step);
        self
    }
    /// Return the step count.
    #[allow(dead_code)]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}
/// The environment in which commands execute.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CommandEnvironment {
    pub working_directory: String,
    pub variables: std::collections::HashMap<String, String>,
    pub verbose: bool,
    pub dry_run: bool,
    pub color: bool,
}
impl CommandEnvironment {
    /// Create a new environment.
    #[allow(dead_code)]
    pub fn new(working_directory: impl Into<String>) -> Self {
        Self {
            working_directory: working_directory.into(),
            variables: std::collections::HashMap::new(),
            verbose: false,
            dry_run: false,
            color: true,
        }
    }
    /// Set a variable.
    #[allow(dead_code)]
    pub fn set_var(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.variables.insert(key.into(), value.into());
    }
    /// Get a variable.
    #[allow(dead_code)]
    pub fn get_var(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(|s| s.as_str())
    }
}
