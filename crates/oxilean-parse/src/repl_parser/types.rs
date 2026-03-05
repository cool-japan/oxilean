//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::ast_impl::*;
use crate::error_impl::ParseError;
use crate::lexer::Lexer;
use crate::parser_impl::Parser;

use super::functions::*;

/// Tally of REPL command types seen during a session.
#[allow(dead_code)]
#[derive(Default, Debug, Clone)]
#[allow(missing_docs)]
pub struct CommandTally {
    pub evals: u64,
    pub type_queries: u64,
    pub checks: u64,
    pub loads: u64,
    #[allow(missing_docs)]
    pub quits: u64,
    pub helps: u64,
    pub other: u64,
}
impl CommandTally {
    /// Create a zero-initialized tally.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a command.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn record(&mut self, cmd: &ReplCommand) {
        match cmd {
            ReplCommand::Eval(_) => self.evals += 1,
            ReplCommand::Type(_) => self.type_queries += 1,
            ReplCommand::Check(_) => self.checks += 1,
            ReplCommand::Load(_) => self.loads += 1,
            ReplCommand::Quit => self.quits += 1,
            ReplCommand::Help => self.helps += 1,
            _ => self.other += 1,
        }
    }
    /// Total commands.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn total(&self) -> u64 {
        self.evals
            + self.type_queries
            + self.checks
            + self.loads
            + self.quits
            + self.helps
            + self.other
    }
}
/// A REPL session history entry.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone)]
pub struct ReplHistoryEntry {
    /// The input line
    pub input: String,
    /// Whether parsing succeeded
    pub parse_ok: bool,
    /// Sequence number
    pub seq: usize,
}
impl ReplHistoryEntry {
    /// Create a new entry.
    #[allow(dead_code)]
    pub fn new(seq: usize, input: &str, parse_ok: bool) -> Self {
        ReplHistoryEntry {
            input: input.to_string(),
            parse_ok,
            seq,
        }
    }
}
/// An input splitter that breaks raw REPL input into logical lines,
/// handling multi-line continuation (`--` comments, `(`, `begin`, etc.).
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct InputSplitter {
    /// Lines accumulated so far.
    pub(super) buffer: Vec<String>,
    /// Current depth of open parentheses / brackets.
    pub(super) depth: i32,
}
impl InputSplitter {
    /// Create a new splitter.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a raw line of input.
    #[allow(missing_docs)]
    pub fn push(&mut self, line: &str) {
        self.buffer.push(line.to_string());
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' => self.depth += 1,
                ')' | ']' | '}' => self.depth -= 1,
                _ => {}
            }
        }
    }
    /// Check if the buffer is logically complete (balanced brackets).
    #[allow(missing_docs)]
    pub fn is_complete(&self) -> bool {
        self.depth <= 0 && !self.buffer.is_empty()
    }
    /// Flush the buffer and reset.
    #[allow(missing_docs)]
    pub fn flush(&mut self) -> String {
        let result = self.buffer.join("\n");
        self.buffer.clear();
        self.depth = 0;
        result
    }
    /// Number of lines buffered.
    #[allow(missing_docs)]
    pub fn line_count(&self) -> usize {
        self.buffer.len()
    }
    /// Current bracket depth.
    #[allow(missing_docs)]
    pub fn depth(&self) -> i32 {
        self.depth
    }
    /// Check if buffer is empty.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
/// A key-value option store for REPL session options.
#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct OptionStore {
    values: std::collections::HashMap<String, String>,
}
impl OptionStore {
    /// Create an empty option store.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set an option by name.
    #[allow(missing_docs)]
    pub fn set(&mut self, name: impl Into<String>, value: impl Into<String>) {
        self.values.insert(name.into(), value.into());
    }
    /// Get an option by name.
    #[allow(missing_docs)]
    pub fn get(&self, name: &str) -> Option<&str> {
        self.values.get(name).map(|s| s.as_str())
    }
    /// Check if an option is set.
    #[allow(missing_docs)]
    pub fn has(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
    /// Remove an option.
    #[allow(missing_docs)]
    pub fn remove(&mut self, name: &str) -> bool {
        self.values.remove(name).is_some()
    }
    /// Number of options set.
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.values.len()
    }
    /// Check if empty.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    /// Get a bool option, defaulting to `false`.
    #[allow(missing_docs)]
    pub fn get_bool(&self, name: &str) -> bool {
        self.get(name)
            .map(|v| matches!(v, "true" | "1" | "yes" | "on"))
            .unwrap_or(false)
    }
    /// Get a u64 option, defaulting to `default`.
    #[allow(missing_docs)]
    pub fn get_u64(&self, name: &str, default: u64) -> u64 {
        self.get(name)
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    }
    /// Get all option names.
    #[allow(missing_docs)]
    pub fn keys(&self) -> Vec<&str> {
        self.values.keys().map(|s| s.as_str()).collect()
    }
}
/// A REPL command alias: maps a short name to a full command string.
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct CommandAlias {
    pub from: String,
    pub to: String,
}
impl CommandAlias {
    /// Create a new alias.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}
/// A REPL mode that controls what kinds of input are accepted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum ReplMode {
    /// Standard mode: accept expressions and declarations.
    Normal,
    /// Tactic mode: accept tactic-block lines.
    Tactic,
    /// Search mode: accept search queries only.
    Search,
}
impl ReplMode {
    /// The mode name for display.
    #[allow(missing_docs)]
    pub fn name(&self) -> &'static str {
        match self {
            ReplMode::Normal => "normal",
            ReplMode::Tactic => "tactic",
            ReplMode::Search => "search",
        }
    }
    /// Parse a mode name.
    #[allow(missing_docs)]
    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "normal" => Some(ReplMode::Normal),
            "tactic" => Some(ReplMode::Tactic),
            "search" => Some(ReplMode::Search),
            _ => None,
        }
    }
}
/// REPL command type.
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
#[allow(missing_docs)]
pub enum ReplCommand {
    /// Evaluate an expression
    Eval(Located<SurfaceExpr>),
    /// Show type of an expression
    Type(Located<SurfaceExpr>),
    /// Check a declaration
    Check(Located<Decl>),
    /// Load a file
    Load(String),
    /// Show environment
    ShowEnv,
    /// Clear environment
    Clear,
    /// Show help
    Help,
    /// Quit REPL
    Quit,
}
/// A no-op REPL event listener.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct NoopListener;
/// A pipeline of input filters.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct FilterPipeline {
    filters: Vec<Box<dyn InputFilter>>,
}
impl FilterPipeline {
    /// Create an empty pipeline.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }
    /// Add a filter to the pipeline.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add<F: InputFilter + 'static>(&mut self, f: F) {
        self.filters.push(Box::new(f));
    }
    /// Apply all filters in sequence.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn apply(&self, input: &str) -> String {
        let mut result = input.to_string();
        for f in &self.filters {
            result = f.filter(&result);
        }
        result
    }
    /// Number of filters.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.filters.len()
    }
    /// Whether the pipeline is empty.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }
}
/// Statistics about REPL session.
#[derive(Debug, Clone, Default)]
#[allow(missing_docs)]
pub struct ReplStats {
    /// Number of commands evaluated.
    pub commands_run: u64,
    /// Number of successful elaborations.
    pub successes: u64,
    /// Number of errors encountered.
    #[allow(missing_docs)]
    pub errors: u64,
    /// Total characters typed.
    pub chars_typed: u64,
}
impl ReplStats {
    /// Create zeroed stats.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a successful command.
    #[allow(missing_docs)]
    pub fn record_success(&mut self) {
        self.commands_run += 1;
        self.successes += 1;
    }
    /// Record an error.
    #[allow(missing_docs)]
    pub fn record_error(&mut self) {
        self.commands_run += 1;
        self.errors += 1;
    }
    /// Record characters typed.
    #[allow(missing_docs)]
    pub fn record_chars(&mut self, n: u64) {
        self.chars_typed += n;
    }
    /// Return success rate (0.0 to 1.0).
    #[allow(missing_docs)]
    pub fn success_rate(&self) -> f64 {
        if self.commands_run == 0 {
            1.0
        } else {
            self.successes as f64 / self.commands_run as f64
        }
    }
}
/// REPL output formatter.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ReplFormatter {
    use_color: bool,
    max_width: usize,
}
impl ReplFormatter {
    /// Create a formatter.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(use_color: bool, max_width: usize) -> Self {
        Self {
            use_color,
            max_width,
        }
    }
    /// Format a success message.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn success(&self, msg: &str) -> String {
        if self.use_color {
            format!("\x1b[1;32m✓\x1b[0m {}", msg)
        } else {
            format!("OK: {}", msg)
        }
    }
    /// Format an error message.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn error(&self, msg: &str) -> String {
        if self.use_color {
            format!("\x1b[1;31m✗\x1b[0m {}", msg)
        } else {
            format!("Error: {}", msg)
        }
    }
    /// Format an info message.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn info(&self, msg: &str) -> String {
        if self.use_color {
            format!("\x1b[1;34mℹ\x1b[0m {}", msg)
        } else {
            format!("Info: {}", msg)
        }
    }
    /// Truncate a string to the max width.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn truncate(&self, s: &str) -> String {
        if s.len() <= self.max_width {
            s.to_string()
        } else {
            format!("{}..", &s[..self.max_width.saturating_sub(2)])
        }
    }
    /// Format a list of items.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn list(&self, items: &[String]) -> String {
        items
            .iter()
            .enumerate()
            .map(|(i, s)| format!("  {}: {}", i + 1, self.truncate(s)))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// REPL parser for parsing REPL-specific commands.
#[allow(missing_docs)]
pub struct ReplParser {
    /// Input line
    input: String,
}
impl ReplParser {
    /// Create a new REPL parser.
    #[allow(missing_docs)]
    pub fn new(input: String) -> Self {
        Self { input }
    }
    /// Parse a REPL command.
    #[allow(missing_docs)]
    pub fn parse(&self) -> Result<ReplCommand, ParseError> {
        let trimmed = self.input.trim();
        if let Some(cmd) = trimmed.strip_prefix(':') {
            return self.parse_meta_command(cmd);
        }
        self.parse_input()
    }
    /// Parse meta-commands (:quit, :help, etc.)
    fn parse_meta_command(&self, cmd: &str) -> Result<ReplCommand, ParseError> {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return Err(ParseError::unexpected(
                vec!["command".to_string()],
                crate::tokens::TokenKind::Eof,
                crate::span_util::dummy_span(),
            ));
        }
        match parts[0] {
            "quit" | "q" | "exit" => Ok(ReplCommand::Quit),
            "help" | "h" | "?" => Ok(ReplCommand::Help),
            "env" | "show" => Ok(ReplCommand::ShowEnv),
            "clear" | "reset" => Ok(ReplCommand::Clear),
            "type" | "t" => {
                if parts.len() < 2 {
                    return Err(ParseError::unexpected(
                        vec!["expression".to_string()],
                        crate::tokens::TokenKind::Eof,
                        crate::span_util::dummy_span(),
                    ));
                }
                let expr_str = parts[1..].join(" ");
                let expr = self.parse_expr_from_str(&expr_str)?;
                Ok(ReplCommand::Type(expr))
            }
            "load" | "l" => {
                if parts.len() < 2 {
                    return Err(ParseError::unexpected(
                        vec!["filename".to_string()],
                        crate::tokens::TokenKind::Eof,
                        crate::span_util::dummy_span(),
                    ));
                }
                Ok(ReplCommand::Load(parts[1].to_string()))
            }
            "check" | "c" => {
                if parts.len() < 2 {
                    return Err(ParseError::unexpected(
                        vec!["declaration".to_string()],
                        crate::tokens::TokenKind::Eof,
                        crate::span_util::dummy_span(),
                    ));
                }
                let decl_str = parts[1..].join(" ");
                let decl = self.parse_decl_from_str(&decl_str)?;
                Ok(ReplCommand::Check(decl))
            }
            _ => Err(ParseError::unexpected(
                vec!["known command".to_string()],
                crate::tokens::TokenKind::Eof,
                crate::span_util::dummy_span(),
            )),
        }
    }
    /// Parse input as expression or declaration.
    fn parse_input(&self) -> Result<ReplCommand, ParseError> {
        if let Ok(decl) = self.parse_decl_from_str(&self.input) {
            return Ok(ReplCommand::Check(decl));
        }
        let expr = self.parse_expr_from_str(&self.input)?;
        Ok(ReplCommand::Eval(expr))
    }
    /// Parse an expression from a string.
    fn parse_expr_from_str(&self, s: &str) -> Result<Located<SurfaceExpr>, ParseError> {
        let mut lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        parser.parse_expr()
    }
    /// Parse a declaration from a string.
    fn parse_decl_from_str(&self, s: &str) -> Result<Located<Decl>, ParseError> {
        let mut lexer = Lexer::new(s);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        parser.parse_decl()
    }
}
/// A REPL session history.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ReplHistory {
    /// All entries in order
    pub entries: Vec<ReplHistoryEntry>,
}
impl ReplHistory {
    /// Create a new empty history.
    #[allow(dead_code)]
    pub fn new() -> Self {
        ReplHistory {
            entries: Vec::new(),
        }
    }
    /// Add an entry.
    #[allow(dead_code)]
    pub fn push(&mut self, input: &str, parse_ok: bool) {
        let seq = self.entries.len();
        self.entries
            .push(ReplHistoryEntry::new(seq, input, parse_ok));
    }
    /// Returns the number of entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Returns true if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
/// Command history for the REPL.
#[derive(Debug, Clone, Default)]
#[allow(missing_docs)]
pub struct CommandHistory {
    entries: Vec<String>,
    max_entries: usize,
    position: usize,
}
impl CommandHistory {
    /// Create a new empty history with capacity.
    #[allow(missing_docs)]
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
            position: 0,
        }
    }
    /// Push a new entry to history.
    ///
    /// If the entry is identical to the last one, it is not duplicated.
    #[allow(missing_docs)]
    pub fn push(&mut self, entry: String) {
        if entry.trim().is_empty() {
            return;
        }
        if self.entries.last().map(|s| s.as_str()) == Some(&entry) {
            return;
        }
        if self.entries.len() >= self.max_entries {
            self.entries.remove(0);
        }
        self.entries.push(entry);
        self.position = self.entries.len();
    }
    /// Navigate backward in history (older entries).
    #[allow(missing_docs)]
    pub fn prev(&mut self) -> Option<&str> {
        if self.entries.is_empty() || self.position == 0 {
            return None;
        }
        self.position -= 1;
        self.entries.get(self.position).map(|s| s.as_str())
    }
    /// Navigate forward in history (newer entries).
    #[allow(clippy::should_implement_trait)]
    #[allow(missing_docs)]
    pub fn next(&mut self) -> Option<&str> {
        if self.position >= self.entries.len() {
            return None;
        }
        self.position += 1;
        self.entries.get(self.position).map(|s| s.as_str())
    }
    /// Get all history entries.
    #[allow(missing_docs)]
    pub fn entries(&self) -> &[String] {
        &self.entries
    }
    /// Clear the history.
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        self.entries.clear();
        self.position = 0;
    }
    /// Return the number of entries.
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return true if history is empty.
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Search history for entries containing the given query.
    #[allow(missing_docs)]
    pub fn search(&self, query: &str) -> Vec<&str> {
        self.entries
            .iter()
            .filter(|e| e.contains(query))
            .map(|s| s.as_str())
            .collect()
    }
}
/// Extended REPL command including option management and history.
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
#[allow(missing_docs)]
pub enum ExtendedReplCommand {
    /// Set an option: :set name value
    SetOption(String, String),
    /// Get an option: :get name
    GetOption(String),
    /// Show command history: :history
    History,
    /// Undo last declaration: :undo
    Undo,
    /// Print a term's normal form: :reduce expr
    Reduce(Located<SurfaceExpr>),
    /// Show statistics: :stats
    Stats,
    /// Search for a name: :search pattern
    Search(String),
    /// Print declaration info: :print name
    Print(String),
}
/// The kind of a completion item.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum CompletionKind {
    Command,
    Keyword,
    Identifier,
    Tactic,
}
/// Options for REPL behavior.
#[derive(Debug, Clone)]
#[allow(missing_docs)]
pub struct ReplOptions {
    /// Whether to show timing information.
    pub show_timing: bool,
    /// Whether to print types alongside values.
    pub print_types: bool,
    /// Maximum number of history entries.
    #[allow(missing_docs)]
    pub max_history: usize,
    /// Whether to enable color output.
    pub color: bool,
    /// Whether to enable verbose mode.
    pub verbose: bool,
}
impl ReplOptions {
    /// Create new options with defaults.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Set an option by name and value string.
    ///
    /// Returns `Ok(())` on success or an error string if the option
    /// name or value is unrecognized.
    #[allow(missing_docs)]
    pub fn set(&mut self, name: &str, value: &str) -> Result<(), String> {
        match name {
            "timing" => {
                self.show_timing = parse_bool(value)?;
                Ok(())
            }
            "types" => {
                self.print_types = parse_bool(value)?;
                Ok(())
            }
            "color" => {
                self.color = parse_bool(value)?;
                Ok(())
            }
            "verbose" => {
                self.verbose = parse_bool(value)?;
                Ok(())
            }
            "history" => {
                self.max_history = value
                    .parse::<usize>()
                    .map_err(|_| format!("Expected number, got '{}'", value))?;
                Ok(())
            }
            _ => Err(format!("Unknown option '{}'", name)),
        }
    }
    /// Get an option value as a string.
    #[allow(missing_docs)]
    pub fn get(&self, name: &str) -> Option<String> {
        match name {
            "timing" => Some(self.show_timing.to_string()),
            "types" => Some(self.print_types.to_string()),
            "color" => Some(self.color.to_string()),
            "verbose" => Some(self.verbose.to_string()),
            "history" => Some(self.max_history.to_string()),
            _ => None,
        }
    }
    /// List all known option names.
    #[allow(missing_docs)]
    pub fn known_options() -> &'static [&'static str] {
        &["timing", "types", "color", "verbose", "history"]
    }
}
/// State of multi-line input accumulation.
#[derive(Debug, Clone, Default)]
#[allow(missing_docs)]
pub struct MultilineState {
    /// Accumulated lines so far.
    pub(super) lines: Vec<String>,
    /// Brace/paren depth to track continuation.
    pub(super) depth: i32,
}
impl MultilineState {
    /// Create a new empty multi-line state.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a line to the accumulation.
    #[allow(missing_docs)]
    pub fn push_line(&mut self, line: &str) {
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' => self.depth += 1,
                ')' | ']' | '}' => self.depth -= 1,
                _ => {}
            }
        }
        self.lines.push(line.to_string());
    }
    /// Check if the accumulated input is complete.
    #[allow(missing_docs)]
    pub fn is_complete(&self) -> bool {
        self.depth <= 0 && !self.lines.is_empty()
    }
    /// Get the accumulated input as a single string.
    #[allow(missing_docs)]
    pub fn get_input(&self) -> String {
        self.lines.join("\n")
    }
    /// Reset the state.
    #[allow(missing_docs)]
    pub fn reset(&mut self) {
        self.lines.clear();
        self.depth = 0;
    }
    /// Return the current depth (positive means inside a group).
    #[allow(missing_docs)]
    pub fn depth(&self) -> i32 {
        self.depth
    }
    /// Return the number of accumulated lines.
    #[allow(missing_docs)]
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}
/// A REPL session that tracks history, options, and statistics.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct ReplSession {
    /// Command history.
    pub history: CommandHistory,
    /// REPL options.
    pub options: ReplOptions,
    /// Session statistics.
    #[allow(missing_docs)]
    pub stats: ReplStats,
    /// The underlying parser used by this session.
    _phantom: std::marker::PhantomData<()>,
}
impl ReplSession {
    /// Create a new REPL session with default settings.
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let options = ReplOptions::default();
        let max_history = options.max_history;
        Self {
            history: CommandHistory::new(max_history),
            options,
            stats: ReplStats::new(),
            _phantom: std::marker::PhantomData,
        }
    }
    /// Process a raw input line.
    ///
    /// Returns the parsed command or an error.
    #[allow(missing_docs)]
    pub fn process(&mut self, input: &str) -> Result<ReplCommand, String> {
        let input = input.trim().to_string();
        if input.is_empty() {
            return Err("Empty input".to_string());
        }
        self.history.push(input.clone());
        self.stats.record_chars(input.len() as u64);
        let parser = ReplParser::new(input);
        parser.parse().map_err(|e| e.to_string())
    }
    /// Get the help text.
    #[allow(missing_docs)]
    pub fn help(&self) -> &'static str {
        help_text()
    }
}
/// A REPL completion item.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(missing_docs)]
pub struct CompletionItem {
    pub text: String,
    pub description: String,
    pub kind: CompletionKind,
}
impl CompletionItem {
    /// Create a new completion item.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(text: &str, description: &str, kind: CompletionKind) -> Self {
        Self {
            text: text.to_string(),
            description: description.to_string(),
            kind,
        }
    }
}
/// A REPL session option that can be toggled.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(missing_docs)]
pub enum ReplOption {
    /// Print timing information after each command.
    Timing,
    /// Show the elaborated term after each check.
    ShowElaborated,
    /// Enable verbose mode.
    Verbose,
    /// Pretty-print expressions.
    PrettyPrint,
    /// Maximum output lines.
    MaxLines,
}
impl ReplOption {
    /// Parse a `ReplOption` from its name string.
    #[allow(missing_docs)]
    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "timing" => Some(ReplOption::Timing),
            "show_elaborated" | "showElaborated" => Some(ReplOption::ShowElaborated),
            "verbose" => Some(ReplOption::Verbose),
            "pretty_print" | "prettyPrint" => Some(ReplOption::PrettyPrint),
            "max_lines" | "maxLines" => Some(ReplOption::MaxLines),
            _ => None,
        }
    }
    /// Get the canonical name of this option.
    #[allow(missing_docs)]
    pub fn name(&self) -> &'static str {
        match self {
            ReplOption::Timing => "timing",
            ReplOption::ShowElaborated => "show_elaborated",
            ReplOption::Verbose => "verbose",
            ReplOption::PrettyPrint => "pretty_print",
            ReplOption::MaxLines => "max_lines",
        }
    }
}
/// Configuration for ReplParser.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
#[allow(missing_docs)]
pub struct ReplParserConfig {
    /// Whether to allow multi-line input.
    pub allow_multiline: bool,
    /// Whether to normalize whitespace before parsing.
    pub normalize_whitespace: bool,
    /// Whether to strip comments before parsing.
    #[allow(missing_docs)]
    pub strip_comments: bool,
}
impl ReplParserConfig {
    /// Create a default configuration.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Enable multi-line input.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn with_multiline(mut self) -> Self {
        self.allow_multiline = true;
        self
    }
}
/// A REPL event log that records all events.
#[allow(dead_code)]
#[derive(Default, Debug)]
#[allow(missing_docs)]
pub struct EventLog {
    pub(super) events: Vec<ReplEvent>,
}
impl EventLog {
    /// Create an empty event log.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Get all recorded events.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn events(&self) -> &[ReplEvent] {
        &self.events
    }
    /// Clear the log.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn clear(&mut self) {
        self.events.clear();
    }
    /// Number of events.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.events.len()
    }
    /// Whether the log is empty.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}
/// An input filter that lowercases command keywords.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct LowercaseCommandFilter;
/// A REPL input classifier.
#[allow(dead_code)]
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplInputKind {
    /// A term to evaluate
    Term,
    /// A definition
    Definition,
    /// A tactic proof
    Tactic,
    /// A command (e.g. #check, #eval)
    Command,
    /// An incomplete input
    Incomplete,
}
/// A REPL event emitted by the session.
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum ReplEvent {
    /// A command was successfully parsed.
    Parsed(String),
    /// A parse error occurred.
    Error(String),
    /// The session was reset.
    Reset,
    /// An option was changed.
    OptionChanged(String, String),
    /// The REPL is exiting.
    Exit,
}
/// A REPL completer that provides completions for a given prefix.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ReplCompleter {
    items: Vec<CompletionItem>,
}
impl ReplCompleter {
    /// Create a completer with built-in items.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        let mut items = Vec::new();
        for cmd in [
            ":quit", ":help", ":env", ":clear", ":type", ":load", ":check", ":set", ":history",
        ] {
            items.push(CompletionItem::new(
                cmd,
                "REPL command",
                CompletionKind::Command,
            ));
        }
        for kw in [
            "theorem",
            "def",
            "axiom",
            "inductive",
            "fun",
            "let",
            "match",
            "forall",
        ] {
            items.push(CompletionItem::new(kw, "keyword", CompletionKind::Keyword));
        }
        Self { items }
    }
    /// Complete a prefix.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn complete(&self, prefix: &str) -> Vec<&CompletionItem> {
        self.items
            .iter()
            .filter(|item| item.text.starts_with(prefix))
            .collect()
    }
    /// Add a custom completion item.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn add(&mut self, item: CompletionItem) {
        self.items.push(item);
    }
    /// Number of registered items.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    /// Whether there are any items.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}
/// A REPL alias registry.
#[allow(dead_code)]
#[derive(Default, Debug)]
#[allow(missing_docs)]
pub struct AliasRegistry {
    aliases: Vec<CommandAlias>,
}
impl AliasRegistry {
    /// Create an empty registry.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Register an alias.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn register(&mut self, from: &str, to: &str) {
        self.aliases.push(CommandAlias::new(from, to));
    }
    /// Expand an alias. Returns the expanded string or the original.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn expand<'a>(&'a self, input: &'a str) -> &'a str {
        for alias in &self.aliases {
            if input.trim_start() == alias.from {
                return &alias.to;
            }
        }
        input
    }
    /// Number of aliases.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn len(&self) -> usize {
        self.aliases.len()
    }
    /// Whether the registry is empty.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn is_empty(&self) -> bool {
        self.aliases.is_empty()
    }
    /// List all alias names.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn names(&self) -> Vec<&str> {
        self.aliases.iter().map(|a| a.from.as_str()).collect()
    }
}
/// An input filter that strips trailing semicolons.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct StripSemicolonFilter;
/// Configurable REPL parser.
#[allow(dead_code)]
#[allow(missing_docs)]
pub struct ConfigurableReplParser {
    config: ReplParserConfig,
}
impl ConfigurableReplParser {
    /// Create a new configurable REPL parser.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn new(config: ReplParserConfig) -> Self {
        Self { config }
    }
    /// Preprocess input according to config.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn preprocess(&self, input: &str) -> String {
        let mut s = input.to_string();
        if self.config.normalize_whitespace {
            s = s.split_whitespace().collect::<Vec<_>>().join(" ");
        }
        if self.config.strip_comments {
            s = s
                .lines()
                .map(|line| {
                    if let Some(idx) = line.find("--") {
                        &line[..idx]
                    } else {
                        line
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
        }
        s
    }
    /// Parse the preprocessed input.
    #[allow(dead_code)]
    #[allow(missing_docs)]
    pub fn parse(&self, input: &str) -> Result<ReplCommand, String> {
        let processed = self.preprocess(input);
        let p = ReplParser::new(processed);
        p.parse().map_err(|e| e.to_string())
    }
}
