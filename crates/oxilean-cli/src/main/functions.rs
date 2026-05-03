//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::repl::Repl;

use super::types::{
    CliBuildInfo, CliCommand, CliConfig, CliDiagnostic, CliDiagnosticsReporter, CliEnvironment,
    CliExecutionResult, CliResult, ColorChoice, DiagSeverity, ExtCommand, GlobalArgs, GlobalFlags,
    ProgressBar, Shell, SubcommandEntry, TelemetryConfig, VersionInfo,
};
use std::env;
use std::fs;
use std::io;
use std::process;

pub fn cli_main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        run_repl();
    } else {
        match args[1].as_str() {
            "check" => {
                if args.len() < 3 {
                    eprintln!("Usage: oxilean check <file>");
                    process::exit(1);
                }
                check_file(&args[2]);
            }
            "repl" => run_repl(),
            "lsp" | "serve" => {
                if let Err(e) = crate::lsp::server::run_server_stdio() {
                    eprintln!("LSP server error: {}", e);
                    process::exit(1);
                }
            }
            "version" => print_version(),
            "help" => print_help(),
            _ => {
                check_file(&args[1]);
            }
        }
    }
}
fn run_repl() {
    let mut repl = Repl::new();
    if let Err(e) = repl.run() {
        eprintln!("REPL error: {}", e);
        process::exit(1);
    }
}
pub fn check_file(path: &str) {
    match fs::read_to_string(path) {
        Ok(contents) => match crate::commands::check_source(&contents) {
            Ok(()) => {
                println!("✓ File checked successfully");
            }
            Err(e) => {
                eprintln!("Error: {}", e.message);
                process::exit(e.code.as_u32() as i32);
            }
        },
        Err(e) => {
            eprintln!("Failed to read file {}: {}", path, e);
            process::exit(1);
        }
    }
}
fn print_version() {
    println!("OxiLean version {}", env!("CARGO_PKG_VERSION"));
    println!("Kernel SLOC: ~2,600");
    println!("Zero external dependencies in kernel");
}
fn print_help() {
    println!("OxiLean - Pure Rust Interactive Theorem Prover");
    println!();
    println!("Usage:");
    println!("  oxilean                 Start REPL");
    println!("  oxilean <file>          Check a file");
    println!("  oxilean check <file>    Check a file");
    println!("  oxilean repl            Start REPL");
    println!("  oxilean lsp             Start LSP server (stdio transport)");
    println!("  oxilean serve           Start LSP server (alias for lsp)");
    println!("  oxilean version         Show version");
    println!("  oxilean help            Show this help");
    println!();
    println!("REPL Commands:");
    println!("  :quit, :q               Exit REPL");
    println!("  :help, :h               Show help");
    println!("  :type <expr>            Show type of expression");
    println!("  :check <expr>           Check expression");
    println!("  :env                    Show environment");
    println!("  :clear                  Clear environment");
}
/// Format a duration in milliseconds as a human-readable string.
///
/// - `< 1000 ms` → `"42ms"`
/// - `< 60 000 ms` → `"3.14s"`
/// - `else` → `"2m 5s"`
#[allow(dead_code)]
pub fn format_duration_ms(ms: u64) -> String {
    if ms < 1_000 {
        format!("{ms}ms")
    } else if ms < 60_000 {
        let secs = ms as f64 / 1_000.0;
        format!("{secs:.2}s")
    } else {
        let total_secs = ms / 1_000;
        let minutes = total_secs / 60;
        let secs = total_secs % 60;
        format!("{minutes}m {secs}s")
    }
}
/// Format a file size in bytes as a human-readable string.
///
/// - `< 1024` → `"512 B"`
/// - `< 1024 * 1024` → `"3.1 KB"`
/// - `else` → `"1.2 MB"`
#[allow(dead_code)]
pub fn format_file_size(bytes: u64) -> String {
    if bytes < 1_024 {
        format!("{bytes} B")
    } else if bytes < 1_024 * 1_024 {
        let kb = bytes as f64 / 1_024.0;
        format!("{kb:.1} KB")
    } else {
        let mb = bytes as f64 / (1_024.0 * 1_024.0);
        format!("{mb:.1} MB")
    }
}
/// Check whether a file path looks like an OxiLean source file.
#[allow(dead_code)]
pub fn is_oxilean_file(path: &str) -> bool {
    path.ends_with(".oxilean") || path.ends_with(".lean")
}
/// Validate that a project name matches `[a-zA-Z][a-zA-Z0-9_-]*`.
#[allow(dead_code)]
pub fn is_valid_project_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}
/// Return the OxiLean data directory (`$OXILEAN_HOME` or `~/.oxilean`).
#[allow(dead_code)]
pub fn data_dir() -> String {
    std::env::var("OXILEAN_HOME").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        format!("{home}/.oxilean")
    })
}
/// Return a table of all supported CLI commands for `--help` output.
#[allow(dead_code)]
pub fn commands_table() -> Vec<(&'static str, &'static str)> {
    vec![
        ("check <file>", "Type-check a source file"),
        ("repl", "Start the interactive REPL"),
        ("build [manifest]", "Build a project"),
        ("bench <file>", "Run benchmarks"),
        ("doc [dir]", "Generate documentation"),
        ("fmt <file>", "Format a source file"),
        ("lsp", "Start the LSP server"),
        ("export <file>", "Export to JSON"),
        ("version", "Show version info"),
        ("help", "Show this message"),
    ]
}
/// Print the extended help message including all commands.
#[allow(dead_code)]
pub fn print_extended_help() {
    println!("OxiLean {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE:");
    println!("    oxilean [command] [options]");
    println!();
    println!("COMMANDS:");
    for (cmd, desc) in commands_table() {
        println!("    {cmd:<26} {desc}");
    }
    println!();
    println!("OPTIONS:");
    println!("    --verbose, -v    Enable verbose output");
    println!("    --quiet, -q      Suppress output");
    println!("    --no-color       Disable color output");
    println!("    --help, -h       Show this message");
    println!();
    println!("ENVIRONMENT:");
    println!("    OXILEAN_HOME     Override the data directory (default: ~/.oxilean)");
    println!("    OXILEAN_LOG      Log level: error, warn, info, debug, trace");
}
/// Count the number of `.oxilean` / `.lean` files under a directory (non-recursive).
#[allow(dead_code)]
pub fn count_source_files(dir: &str) -> usize {
    use std::fs;
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_str().map(is_oxilean_file).unwrap_or(false))
                .count()
        })
        .unwrap_or(0)
}
/// Estimate a rough "lines of code" count from a string.
#[allow(dead_code)]
pub fn count_lines(src: &str) -> usize {
    src.lines()
        .filter(|l| !l.trim().is_empty() && !l.trim().starts_with("--"))
        .count()
}
/// Strip ANSI color codes from a string (for plain-text output).
#[allow(dead_code)]
pub fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' && chars.peek() == Some(&'[') {
            for ch in chars.by_ref() {
                if ch == 'm' {
                    break;
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}
/// Right-pad a string to a given width (for table formatting).
#[allow(dead_code)]
pub fn pad_right(s: &str, width: usize) -> String {
    if s.len() >= width {
        s.to_string()
    } else {
        format!("{s}{}", " ".repeat(width - s.len()))
    }
}
#[cfg(test)]
mod cli_util_tests {
    use super::*;
    #[test]
    fn test_format_duration_ms() {
        assert_eq!(format_duration_ms(42), "42ms");
        assert_eq!(format_duration_ms(1000), "1.00s");
        assert_eq!(format_duration_ms(90_000), "1m 30s");
    }
    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(2048), "2.0 KB");
        assert_eq!(format_file_size(2 * 1024 * 1024), "2.0 MB");
    }
    #[test]
    fn test_is_oxilean_file() {
        assert!(is_oxilean_file("foo.oxilean"));
        assert!(is_oxilean_file("bar.lean"));
        assert!(!is_oxilean_file("main.rs"));
    }
    #[test]
    fn test_is_valid_project_name() {
        assert!(is_valid_project_name("MyProject"));
        assert!(is_valid_project_name("my-proj_1"));
        assert!(!is_valid_project_name("1bad"));
        assert!(!is_valid_project_name(""));
        assert!(!is_valid_project_name("bad name"));
    }
    #[test]
    fn test_cli_command_parse_check() {
        let args: Vec<String> = vec!["check".to_string(), "foo.oxilean".to_string()];
        let cmd = CliCommand::parse(&args);
        assert!(matches!(cmd, CliCommand::Check(f) if f == "foo.oxilean"));
    }
    #[test]
    fn test_cli_command_parse_repl() {
        let args: Vec<String> = vec!["repl".to_string()];
        let cmd = CliCommand::parse(&args);
        assert!(matches!(cmd, CliCommand::Repl));
    }
    #[test]
    fn test_cli_command_parse_version() {
        let args: Vec<String> = vec!["version".to_string()];
        let cmd = CliCommand::parse(&args);
        assert!(matches!(cmd, CliCommand::Version));
    }
    #[test]
    fn test_cli_command_parse_unknown() {
        let args: Vec<String> = vec!["foobarbaz".to_string()];
        let cmd = CliCommand::parse(&args);
        assert!(matches!(cmd, CliCommand::Unknown(_)));
    }
    #[test]
    fn test_cli_config_default() {
        let cfg = CliConfig::default_config();
        assert!(!cfg.verbose);
        assert!(!cfg.quiet);
        assert!(cfg.color);
    }
    #[test]
    fn test_cli_result_exit_code() {
        assert_eq!(CliResult::Ok.exit_code(), 0);
        assert_eq!(CliResult::Err("oops".to_string()).exit_code(), 1);
        assert_eq!(CliResult::Exit(42).exit_code(), 42);
    }
    #[test]
    fn test_commands_table_non_empty() {
        assert!(!commands_table().is_empty());
    }
    #[test]
    fn test_count_lines() {
        let src = "-- comment\nfoo\n\nbar";
        assert_eq!(count_lines(src), 2);
    }
    #[test]
    fn test_strip_ansi() {
        let colored = "\x1b[31mhello\x1b[0m";
        assert_eq!(strip_ansi(colored), "hello");
    }
    #[test]
    fn test_pad_right() {
        assert_eq!(pad_right("hi", 5), "hi   ");
        assert_eq!(pad_right("toolong", 3), "toolong");
    }
    #[test]
    fn test_requires_file() {
        assert!(CliCommand::Check("f".to_string()).requires_file());
        assert!(!CliCommand::Repl.requires_file());
        assert!(!CliCommand::Help.requires_file());
    }
}
/// Draw a horizontal rule of a given width.
#[allow(dead_code)]
pub fn horizontal_rule(width: usize) -> String {
    "─".repeat(width)
}
/// Format a key-value pair as a table row.
#[allow(dead_code)]
pub fn table_row(key: &str, value: &str, key_width: usize) -> String {
    format!("  {:<width$} {}", key, value, width = key_width)
}
/// Wrap text at a given column width.
#[allow(dead_code)]
pub fn word_wrap(text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for word in text.split_whitespace() {
        if !current.is_empty() && current.len() + 1 + word.len() > width {
            lines.push(current.clone());
            current.clear();
        }
        if !current.is_empty() {
            current.push(' ');
        }
        current.push_str(word);
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}
/// Truncate a string to `max_len` chars, adding `"..."` if truncated.
#[allow(dead_code)]
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    }
}
/// Center a string within a field of given width.
#[allow(dead_code)]
pub fn center(s: &str, width: usize) -> String {
    if s.len() >= width {
        return s.to_string();
    }
    let padding = width - s.len();
    let left = padding / 2;
    let right = padding - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}
/// Format a number with thousands separators.
#[allow(dead_code)]
pub fn format_number(n: u64) -> String {
    let s = n.to_string();
    let chars: Vec<char> = s.chars().collect();
    let mut result = String::new();
    for (i, c) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i) % 3 == 0 {
            result.push(',');
        }
        result.push(*c);
    }
    result
}
/// Check whether a string is a valid OxiLean identifier.
#[allow(dead_code)]
pub fn is_valid_identifier(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_alphanumeric() || c == '_' || c == '\'')
}
/// Parse a key=value pair from a string.
#[allow(dead_code)]
pub fn parse_key_value(s: &str) -> Option<(&str, &str)> {
    let pos = s.find('=')?;
    Some((&s[..pos], &s[pos + 1..]))
}
/// Exit status codes for the CLI.
#[allow(dead_code)]
pub mod exit_codes {
    /// Success.
    pub const SUCCESS: i32 = 0;
    /// General error.
    pub const ERROR: i32 = 1;
    /// Type-check failure.
    pub const TYPE_ERROR: i32 = 2;
    /// File not found.
    pub const FILE_NOT_FOUND: i32 = 3;
    /// Parse error.
    pub const PARSE_ERROR: i32 = 4;
    /// Internal error.
    pub const INTERNAL_ERROR: i32 = 5;
    /// Invalid argument.
    pub const INVALID_ARGUMENT: i32 = 6;
}
/// Signal handling utilities.
///
/// Uses an `AtomicBool` flag.  On Unix the flag is set from a raw signal
/// handler for `SIGINT`.  On other platforms a background thread polls
/// for a line containing only `"quit"` as a fallback, but the flag can
/// always be set manually via `interrupt()`.
pub mod signal_util {
    use std::sync::atomic::{AtomicBool, Ordering};
    /// Global interrupted flag.
    static INTERRUPTED: AtomicBool = AtomicBool::new(false);
    /// Install a Ctrl+C handler that sets the global interrupted flag.
    ///
    /// On Unix this registers a `SIGINT` handler.  The implementation is
    /// `async-signal-safe`: it only writes one byte to the atomic flag.
    pub fn install_ctrlc_handler() {
        #[cfg(unix)]
        {
            unsafe extern "C" fn handler(_: LibcSigNum) {
                INTERRUPTED.store(true, Ordering::SeqCst);
            }
            unsafe {
                libc_signal(SIGINT, handler as *const () as usize);
            }
        }
        #[cfg(not(unix))]
        {
            let _ = std::thread::Builder::new()
                .name("ctrlc-watcher".to_string())
                .spawn(|| {
                    let mut line = String::new();
                    while std::io::stdin().read_line(&mut line).is_ok() {
                        if line.trim() == "quit" || line.trim() == "exit" {
                            INTERRUPTED.store(true, Ordering::SeqCst);
                            break;
                        }
                        line.clear();
                    }
                });
        }
    }
    /// Set the interrupted flag programmatically (useful in tests or on
    /// platforms without proper signal support).
    #[allow(dead_code)]
    pub fn interrupt() {
        INTERRUPTED.store(true, Ordering::SeqCst);
    }
    /// Clear the interrupted flag (e.g. after handling the interrupt).
    #[allow(dead_code)]
    pub fn clear_interrupt() {
        INTERRUPTED.store(false, Ordering::SeqCst);
    }
    /// Return `true` if a Ctrl+C signal (or programmatic interrupt) was
    /// received since the last call to `clear_interrupt`.
    pub fn was_interrupted() -> bool {
        INTERRUPTED.load(Ordering::SeqCst)
    }
    #[cfg(unix)]
    type LibcSigNum = std::ffi::c_int;
    #[cfg(unix)]
    const SIGINT: LibcSigNum = 2;
    #[cfg(unix)]
    extern "C" {
        /// C standard `signal(2)`.
        fn signal(signum: LibcSigNum, handler: usize) -> usize;
    }
    #[cfg(unix)]
    unsafe fn libc_signal(signum: LibcSigNum, handler: usize) -> usize {
        signal(signum, handler)
    }
}
#[cfg(test)]
mod additional_cli_tests {
    use super::*;
    #[test]
    fn test_horizontal_rule() {
        let r = horizontal_rule(5);
        assert_eq!(r.chars().count(), 5);
    }
    #[test]
    fn test_table_row() {
        let row = table_row("key", "value", 10);
        assert!(row.contains("key"));
        assert!(row.contains("value"));
    }
    #[test]
    fn test_word_wrap_short() {
        let lines = word_wrap("hello world", 80);
        assert_eq!(lines.len(), 1);
    }
    #[test]
    fn test_word_wrap_long() {
        let text = "one two three four five six seven eight nine ten";
        let lines = word_wrap(text, 15);
        assert!(lines.len() > 1);
        for line in &lines {
            assert!(line.len() <= 15);
        }
    }
    #[test]
    fn test_truncate_short() {
        assert_eq!(truncate("hello", 10), "hello");
    }
    #[test]
    fn test_truncate_long() {
        let t = truncate("hello world foo bar", 10);
        assert!(t.len() <= 10);
        assert!(t.ends_with("..."));
    }
    #[test]
    fn test_center() {
        let c = center("hi", 10);
        assert_eq!(c.len(), 10);
    }
    #[test]
    fn test_format_number() {
        assert_eq!(format_number(1_000_000), "1,000,000");
        assert_eq!(format_number(42), "42");
        assert_eq!(format_number(1_234_567), "1,234,567");
    }
    #[test]
    fn test_is_valid_identifier() {
        assert!(is_valid_identifier("foo_bar"));
        assert!(is_valid_identifier("_x"));
        assert!(is_valid_identifier("f'"));
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("1bad"));
        assert!(!is_valid_identifier("has space"));
    }
    #[test]
    fn test_parse_key_value() {
        assert_eq!(parse_key_value("foo=bar"), Some(("foo", "bar")));
        assert_eq!(parse_key_value("noequalssign"), None);
        assert_eq!(parse_key_value("k="), Some(("k", "")));
    }
    #[test]
    fn test_progress_bar_percent() {
        let mut pb = ProgressBar::new(10, "test");
        assert_eq!(pb.percent(), 0);
        pb.advance(5);
        assert_eq!(pb.percent(), 50);
        pb.advance(5);
        assert_eq!(pb.percent(), 100);
        assert!(pb.is_complete());
    }
    #[test]
    fn test_progress_bar_render() {
        let mut pb = ProgressBar::new(10, "building");
        pb.advance(3);
        let r = pb.render(10);
        assert!(r.contains('['));
        assert!(r.contains(']'));
        assert!(r.contains("building"));
    }
    #[test]
    fn test_progress_bar_tick() {
        let mut pb = ProgressBar::new(5, "x");
        pb.tick();
        assert_eq!(pb.current, 1);
    }
    #[test]
    fn test_progress_bar_zero_total() {
        let pb = ProgressBar::new(0, "empty");
        assert_eq!(pb.percent(), 100);
        assert!(pb.is_complete());
    }
    #[test]
    fn test_exit_codes() {
        assert_eq!(exit_codes::SUCCESS, 0);
        assert_eq!(exit_codes::ERROR, 1);
        assert_ne!(exit_codes::TYPE_ERROR, exit_codes::PARSE_ERROR);
    }
    #[test]
    fn test_data_dir_default() {
        let d = data_dir();
        assert!(d.contains(".oxilean") || d.contains("/tmp") || d.contains("oxilean"));
    }
    #[test]
    fn test_count_lines_empty() {
        assert_eq!(count_lines(""), 0);
    }
}
#[allow(dead_code)]
pub fn num_cpus() -> usize {
    #[cfg(target_os = "linux")]
    if let Ok(s) = std::fs::read_to_string("/proc/cpuinfo") {
        let count = s.lines().filter(|l| l.starts_with("processor")).count();
        if count > 0 {
            return count;
        }
    }
    4
}
#[allow(dead_code)]
fn all_command_names() -> Vec<&'static str> {
    vec![
        "check",
        "repl",
        "build",
        "test",
        "bench",
        "doc",
        "fmt",
        "lsp",
        "export",
        "init",
        "clean",
        "update",
        "deps",
        "search",
        "proof",
        "lint",
        "report",
        "watch",
        "benchmark",
        "diff",
        "migrate",
        "version",
        "help",
    ]
}
#[allow(dead_code)]
fn all_global_flags() -> Vec<&'static str> {
    vec![
        "--verbose",
        "-v",
        "--quiet",
        "-q",
        "--no-color",
        "--jobs=",
        "--log=",
        "--telemetry",
        "--no-telemetry",
        "--no-update-check",
        "--help",
        "-h",
        "--version",
    ]
}
/// Generate shell completions.
#[allow(dead_code)]
pub fn generate_completions(shell: Shell) -> String {
    let commands = all_command_names();
    let flags = all_global_flags();
    match shell {
        Shell::Bash => {
            let cmds = commands.join(" ");
            let flgs = flags.join(" ");
            format!(
                "# Bash completions for oxilean\n_oxilean_completions() {{\n  local cur=\"${{COMP_WORDS[COMP_CWORD]}}\"\n  COMPREPLY=( $(compgen -W \"{cmds} {flgs}\" -- \"$cur\") )\n}}\ncomplete -F _oxilean_completions oxilean\n",
                cmds = cmds, flgs = flgs
            )
        }
        Shell::Zsh => {
            format!(
                "# Zsh completions for oxilean\n#compdef oxilean\n_oxilean() {{\n  _arguments '1: :->cmd' '*: :->args'\n}}\n_oxilean\n"
            )
        }
        Shell::Fish => commands
            .iter()
            .map(|c| {
                format!(
                    "complete -c oxilean -f -n '__fish_use_subcommand' -a {} -d ''",
                    c
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        Shell::PowerShell => {
            format!(
                "# PowerShell completions for oxilean\nRegister-ArgumentCompleter -Native -CommandName oxilean -ScriptBlock {{\n    param($word, $ast, $cursor)\n    @('{}') | Where-Object {{ $_ -like \"$word*\" }}\n}}\n",
                commands.join("', '")
            )
        }
        Shell::Elvish => {
            format!(
                "# Elvish completions\nset edit:completion:arg-completer[oxilean] = {{ |@args| put {} }}\n",
                commands.join(" ")
            )
        }
    }
}
#[allow(dead_code)]
pub fn check_for_update(_current_version: &str) -> Option<String> {
    None
}
#[allow(dead_code)]
pub fn generate_session_id() -> String {
    let pid = std::process::id();
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("{:x}-{:x}", pid, t & 0xffff_ffff)
}
/// Find project root by searching for Oxilean.toml.
#[allow(dead_code)]
pub fn find_project_root() -> Option<std::path::PathBuf> {
    let start = std::env::current_dir().ok()?;
    let mut current = start.as_path();
    loop {
        if current.join("Oxilean.toml").exists() || current.join(".oxilean").exists() {
            return Some(current.to_path_buf());
        }
        current = current.parent()?;
    }
}
/// Detect the working directory.
#[allow(dead_code)]
pub fn detect_working_dir() -> std::path::PathBuf {
    find_project_root()
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| std::path::PathBuf::from("."))
}
/// Config search paths.
#[allow(dead_code)]
pub fn config_search_paths() -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
    if let Some(root) = find_project_root() {
        paths.push(root.join("Oxilean.toml"));
    }
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| "/tmp".to_string());
    paths.push(
        std::path::PathBuf::from(&home)
            .join(".oxilean")
            .join("config.toml"),
    );
    paths.push(std::path::PathBuf::from("/etc/oxilean/config.toml"));
    paths
}
/// Return the first config file that exists.
#[allow(dead_code)]
pub fn discover_config_file() -> Option<std::path::PathBuf> {
    config_search_paths().into_iter().find(|p| p.exists())
}
#[cfg(test)]
mod main_new_tests {
    use super::*;
    #[test]
    fn test_ext_command_parse_init() {
        let args: Vec<String> = vec!["init".to_string(), "my-lib".to_string()];
        let cmd = ExtCommand::parse(&args).expect("parsing should succeed");
        assert!(matches!(cmd, ExtCommand::Init { name } if name == "my-lib"));
    }
    #[test]
    fn test_ext_command_parse_test() {
        let args: Vec<String> = vec!["test".to_string()];
        let cmd = ExtCommand::parse(&args).expect("parsing should succeed");
        assert!(matches!(cmd, ExtCommand::Test { filter: None }));
    }
    #[test]
    fn test_ext_command_parse_unknown() {
        let args: Vec<String> = vec!["zzz".to_string()];
        assert!(ExtCommand::parse(&args).is_none());
    }
    #[test]
    fn test_ext_command_is_mutating() {
        let init = ExtCommand::Init {
            name: "x".to_string(),
        };
        assert!(init.is_mutating());
        let test_cmd = ExtCommand::Test { filter: None };
        assert!(!test_cmd.is_mutating());
    }
    #[test]
    fn test_global_flags_default() {
        let flags = GlobalFlags::default_flags();
        assert!(!flags.verbose);
        assert!(flags.jobs > 0);
    }
    #[test]
    fn test_global_flags_parse_verbose() {
        let mut args: Vec<String> = vec!["--verbose".to_string(), "check".to_string()];
        let flags = GlobalFlags::parse(&mut args);
        assert!(flags.verbose);
        assert_eq!(args, vec!["check"]);
    }
    #[test]
    fn test_global_flags_parse_jobs() {
        let mut args: Vec<String> = vec!["--jobs=8".to_string(), "build".to_string()];
        let flags = GlobalFlags::parse(&mut args);
        assert_eq!(flags.jobs, 8);
    }
    #[test]
    fn test_shell_from_str() {
        assert_eq!(Shell::from_str("bash"), Some(Shell::Bash));
        assert_eq!(Shell::from_str("ZSH"), Some(Shell::Zsh));
        assert_eq!(Shell::from_str("unknown"), None);
    }
    #[test]
    fn test_shell_display() {
        assert_eq!(Shell::Bash.to_string(), "bash");
        assert_eq!(Shell::Fish.to_string(), "fish");
    }
    #[test]
    fn test_generate_completions_bash() {
        let out = generate_completions(Shell::Bash);
        assert!(out.contains("_oxilean_completions"));
        assert!(out.contains("check"));
    }
    #[test]
    fn test_generate_completions_zsh() {
        let out = generate_completions(Shell::Zsh);
        assert!(out.contains("_oxilean"));
    }
    #[test]
    fn test_generate_completions_fish() {
        let out = generate_completions(Shell::Fish);
        assert!(out.contains("oxilean"));
    }
    #[test]
    fn test_version_info_current() {
        let vi = VersionInfo::current();
        assert!(!vi.version.is_empty());
        assert!(!vi.short_version().is_empty());
        assert!(vi.long_version().contains(&vi.version));
    }
    #[test]
    fn test_check_for_update_stub() {
        assert!(check_for_update("0.1.1").is_none());
    }
    #[test]
    fn test_telemetry_default_disabled() {
        let cfg = TelemetryConfig::default_config();
        assert!(!cfg.enabled);
        assert!(cfg.effective_endpoint().is_empty());
    }
    #[test]
    fn test_telemetry_opt_in_out() {
        let cfg = TelemetryConfig::default_config().opt_in();
        assert!(cfg.enabled);
        let cfg2 = cfg.opt_out();
        assert!(!cfg2.enabled);
    }
    #[test]
    fn test_cli_diagnostic_error_format() {
        let diag = CliDiagnostic::error("type mismatch")
            .at_file("Main.lean")
            .at_location(10, 5)
            .with_snippet("let x = 1")
            .with_suggestion("use Nat");
        let text = diag.format(false);
        assert!(text.contains("error"));
        assert!(text.contains("Main.lean"));
        assert!(text.contains("suggestion"));
    }
    #[test]
    fn test_diag_severity_display() {
        assert_eq!(DiagSeverity::Error.to_string(), "error");
        assert_eq!(DiagSeverity::Hint.to_string(), "hint");
    }
    #[test]
    fn test_config_search_paths_not_empty() {
        assert!(!config_search_paths().is_empty());
    }
    #[test]
    fn test_detect_working_dir() {
        let dir = detect_working_dir();
        assert!(!dir.to_string_lossy().is_empty());
    }
    #[test]
    fn test_num_cpus_positive() {
        assert!(num_cpus() > 0);
    }
    #[test]
    fn test_generate_session_id() {
        let id = generate_session_id();
        assert!(!id.is_empty());
        assert!(id.contains('-'));
    }
    #[test]
    fn test_all_command_names_has_check() {
        assert!(all_command_names().contains(&"check"));
    }
}
/// Simple TTY check.
#[allow(dead_code)]
pub fn atty_check() -> bool {
    std::env::var("NO_COLOR").is_err()
}
/// Return the list of built-in subcommands.
#[allow(dead_code)]
pub fn builtin_subcommands() -> Vec<SubcommandEntry> {
    vec![
        SubcommandEntry {
            name: "check",
            aliases: &["c"],
            description: "Type-check a source file",
        },
        SubcommandEntry {
            name: "build",
            aliases: &["b"],
            description: "Build the project",
        },
        SubcommandEntry {
            name: "repl",
            aliases: &["r"],
            description: "Start the interactive REPL",
        },
        SubcommandEntry {
            name: "format",
            aliases: &["fmt"],
            description: "Format source files",
        },
        SubcommandEntry {
            name: "doc",
            aliases: &["docs"],
            description: "Generate documentation",
        },
        SubcommandEntry {
            name: "lint",
            aliases: &[],
            description: "Run lint checks",
        },
        SubcommandEntry {
            name: "serve",
            aliases: &["lsp"],
            description: "Start the LSP server",
        },
        SubcommandEntry {
            name: "clean",
            aliases: &[],
            description: "Clean build artifacts",
        },
        SubcommandEntry {
            name: "test",
            aliases: &["t"],
            description: "Run the test suite",
        },
        SubcommandEntry {
            name: "completions",
            aliases: &[],
            description: "Generate shell completions",
        },
        SubcommandEntry {
            name: "help",
            aliases: &["h", "-h", "--help"],
            description: "Show help information",
        },
        SubcommandEntry {
            name: "version",
            aliases: &["-V", "--version"],
            description: "Show version information",
        },
    ]
}
/// Resolve a command name (handling aliases).
#[allow(dead_code)]
pub fn resolve_subcommand(name: &str) -> Option<&'static str> {
    for entry in builtin_subcommands() {
        if entry.name == name || entry.aliases.contains(&name) {
            return Some(entry.name);
        }
    }
    None
}
/// Standard exit codes (extended set).
#[allow(dead_code)]
pub mod exit_codes_ext {
    pub const SUCCESS: i32 = 0;
    pub const GENERAL_ERROR: i32 = 1;
    pub const USAGE_ERROR: i32 = 2;
    pub const TYPE_ERROR: i32 = 3;
    pub const NOT_FOUND: i32 = 4;
    pub const PERMISSION_DENIED: i32 = 5;
    pub const INTERRUPTED: i32 = 130;
}
/// Generate a startup banner for the CLI.
#[allow(dead_code)]
pub fn cli_banner() -> String {
    let info = CliBuildInfo::current();
    format!(
        "OxiLean {} - The Lean 4 theorem prover in Rust\n",
        info.version
    )
}
/// Generate a help message summary.
#[allow(dead_code)]
pub fn cli_help_summary() -> String {
    let mut out = String::from("USAGE:\n    oxilean [OPTIONS] <SUBCOMMAND>\n\nSUBCOMMANDS:\n");
    for entry in builtin_subcommands() {
        out.push_str(&format!("    {:<15} {}\n", entry.name, entry.description));
    }
    out.push_str("\nOPTIONS:\n");
    out.push_str("    -v, --verbose     Enable verbose output\n");
    out.push_str("    -q, --quiet       Suppress all output\n");
    out.push_str("        --color       Color output (auto|always|never)\n");
    out.push_str("        --log-level   Log level (error|warn|info|debug|trace)\n");
    out.push_str("        --config      Path to configuration file\n");
    out.push_str("    -h, --help        Show this help message\n");
    out.push_str("    -V, --version     Show version information\n");
    out
}
/// Return the CLI version string.
#[allow(dead_code)]
pub fn cli_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
#[cfg(test)]
mod main_extra_tests {
    use super::*;
    #[test]
    fn test_cli_build_info() {
        let info = CliBuildInfo::current();
        assert!(!info.version.is_empty());
    }
    #[test]
    fn test_color_choice_parse() {
        assert_eq!(ColorChoice::from_str("auto"), Some(ColorChoice::Auto));
        assert_eq!(ColorChoice::from_str("always"), Some(ColorChoice::Always));
        assert_eq!(ColorChoice::from_str("never"), Some(ColorChoice::Never));
        assert_eq!(ColorChoice::from_str("yes"), Some(ColorChoice::Always));
        assert_eq!(ColorChoice::from_str("invalid"), None);
    }
    #[test]
    fn test_resolve_subcommand() {
        assert_eq!(resolve_subcommand("check"), Some("check"));
        assert_eq!(resolve_subcommand("c"), Some("check"));
        assert_eq!(resolve_subcommand("fmt"), Some("format"));
        assert_eq!(resolve_subcommand("nonexistent"), None);
    }
    #[test]
    fn test_builtin_subcommands() {
        let cmds = builtin_subcommands();
        assert!(!cmds.is_empty());
        assert!(cmds.iter().any(|c| c.name == "check"));
        assert!(cmds.iter().any(|c| c.name == "repl"));
    }
    #[test]
    fn test_cli_help_summary() {
        let help = cli_help_summary();
        assert!(help.contains("SUBCOMMANDS:"));
        assert!(help.contains("OPTIONS:"));
        assert!(help.contains("check"));
    }
    #[test]
    fn test_cli_banner() {
        let banner = cli_banner();
        assert!(banner.contains("OxiLean"));
    }
    #[test]
    fn test_cli_version() {
        assert!(!cli_version().is_empty());
    }
    #[test]
    fn test_exit_codes_ext() {
        assert_eq!(exit_codes_ext::SUCCESS, 0);
        assert_eq!(exit_codes_ext::GENERAL_ERROR, 1);
        assert_eq!(exit_codes_ext::USAGE_ERROR, 2);
    }
    #[test]
    fn test_cli_environment_detect() {
        let env = CliEnvironment::detect();
        let _ = env.is_interactive();
    }
    #[test]
    fn test_global_args_default() {
        let args = GlobalArgs::default();
        assert!(!args.verbose);
        assert!(!args.quiet);
        assert!(!args.no_config);
        assert_eq!(args.color, ColorChoice::Auto);
    }
}
/// Possible locations for the CLI configuration file (as strings).
#[allow(dead_code)]
pub fn config_search_paths_str() -> Vec<String> {
    let mut paths = vec![];
    if let Ok(home) = std::env::var("HOME") {
        paths.push(format!("{}/.config/oxilean/oxilean.toml", home));
        paths.push(format!("{}/.oxilean.toml", home));
    }
    if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
        paths.push(format!("{}/oxilean/oxilean.toml", xdg));
    }
    paths.push("oxilean.toml".to_string());
    paths.push(".oxilean.toml".to_string());
    paths
}
/// Find the first existing config file.
#[allow(dead_code)]
pub fn find_config_file() -> Option<String> {
    for path in config_search_paths_str() {
        if std::path::Path::new(&path).exists() {
            return Some(path);
        }
    }
    None
}
/// Set up signal handlers (stub).
#[allow(dead_code)]
pub fn setup_signal_handlers() {}
/// Return the main module feature list.
#[allow(dead_code)]
pub fn main_module_features() -> Vec<&'static str> {
    vec![
        "arg-parsing",
        "subcommand-dispatch",
        "color",
        "config-discovery",
        "build-info",
        "environment",
        "signal-handling",
        "exit-codes",
        "banner",
        "help",
    ]
}
#[cfg(test)]
mod main_config_tests {
    use super::*;
    #[test]
    fn test_config_search_paths() {
        let paths = config_search_paths();
        assert!(!paths.is_empty());
        assert!(paths.iter().any(|p| {
            let s = p.to_string_lossy();
            s.ends_with("Oxilean.toml") || s.ends_with("oxilean.toml") || s.ends_with("config.toml")
        }));
    }
    #[test]
    fn test_find_config_file() {
        let _ = find_config_file();
    }
    #[test]
    fn test_cli_execution_result() {
        let ok = CliExecutionResult::ok("done", 100);
        assert!(ok.is_success());
        assert_eq!(ok.exit_code, 0);
        let err = CliExecutionResult::err(1, "failed", 50);
        assert!(!err.is_success());
        assert_eq!(err.exit_code, 1);
    }
    #[test]
    fn test_main_module_features() {
        let features = main_module_features();
        assert!(features.contains(&"arg-parsing"));
        assert!(features.contains(&"color"));
    }
}
/// Return the CLI description string.
#[allow(dead_code)]
pub fn cli_description() -> &'static str {
    "OxiLean - A Lean 4 theorem prover implemented in Rust"
}
#[cfg(test)]
mod cli_reporter_tests {
    use super::*;
    #[test]
    fn test_reporter_format_summary() {
        let reporter = CliDiagnosticsReporter::new(false);
        let summary = reporter.format_summary(0, 0);
        assert!(summary.contains("No errors"));
        let summary2 = reporter.format_summary(3, 5);
        assert!(summary2.contains("3 error(s)"));
        assert!(summary2.contains("5 warning(s)"));
    }
    #[test]
    fn test_cli_description() {
        assert!(!cli_description().is_empty());
        assert!(cli_description().contains("OxiLean"));
    }
}
