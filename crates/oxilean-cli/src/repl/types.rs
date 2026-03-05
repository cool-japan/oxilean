//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_elab::ElabContext;
use oxilean_kernel::{print_expr, Declaration, Environment, Name, Reducer};
use oxilean_parse::{Lexer, Parser};
use std::collections::{HashMap, VecDeque};
use std::env;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::io::Write as IoWrite;
use std::path::PathBuf;
use std::time::Instant;

/// Options that control REPL behavior.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReplOptions {
    /// Pretty-print all (show implicit args, universe levels, etc.)
    pub pp_all: bool,
    /// Show implicit arguments.
    pub pp_implicit: bool,
    /// Show universe levels.
    pub pp_universes: bool,
    /// Use unicode output.
    pub pp_unicode: bool,
    /// Maximum line width for pretty printing.
    pub pp_width: usize,
    /// Show timing information.
    pub show_timing: bool,
    /// Auto-complete brackets.
    pub auto_complete: bool,
}
/// Full session state for the REPL.
#[allow(dead_code)]
pub struct ReplState {
    /// The kernel environment.
    pub env: Environment,
    /// Current REPL mode.
    pub mode: ReplMode,
    /// Configuration options.
    pub options: ReplOptions,
    /// Undo stack.
    pub undo_stack: Vec<UndoEntry>,
    /// List of loaded file paths.
    pub loaded_files: Vec<PathBuf>,
    /// Last loaded file (for :reload).
    pub last_loaded: Option<PathBuf>,
    /// Custom settings (key -> value).
    pub settings: HashMap<String, String>,
    /// Whether sorry was used (tracks proof incompleteness).
    pub used_sorry: bool,
    /// Axioms that have been added.
    pub axiom_names: Vec<Name>,
    /// Active proof state: list of (hypotheses, target) for each open goal.
    /// Each element is (Vec<(name, type_str)>, target_str).
    pub active_proof_state: Vec<(Vec<(String, String)>, String)>,
}
#[allow(dead_code)]
impl ReplState {
    /// Create a new session state.
    pub fn new() -> Self {
        Self {
            env: Environment::new(),
            mode: ReplMode::Normal,
            options: ReplOptions::default(),
            undo_stack: Vec::new(),
            loaded_files: Vec::new(),
            last_loaded: None,
            settings: HashMap::new(),
            used_sorry: false,
            axiom_names: Vec::new(),
            active_proof_state: Vec::new(),
        }
    }
    /// Push an undo entry.
    pub fn push_undo(&mut self, name: Name, description: String) {
        self.undo_stack.push(UndoEntry {
            name,
            old_env: self.env.clone(),
            description,
        });
    }
    /// Pop the most recent undo entry and restore state.
    pub fn pop_undo(&mut self) -> Option<UndoEntry> {
        if let Some(entry) = self.undo_stack.pop() {
            self.env = entry.old_env.clone();
            Some(entry)
        } else {
            None
        }
    }
    /// Reset the entire session state.
    pub fn reset_state(&mut self) {
        self.env = Environment::new();
        self.mode = ReplMode::Normal;
        self.undo_stack.clear();
        self.loaded_files.clear();
        self.last_loaded = None;
        self.used_sorry = false;
        self.axiom_names.clear();
        self.active_proof_state.clear();
    }
    /// Get an option value as a string.
    pub fn get_option(&self, key: &str) -> Option<String> {
        match key {
            "pp.all" => Some(self.options.pp_all.to_string()),
            "pp.implicit" => Some(self.options.pp_implicit.to_string()),
            "pp.universes" => Some(self.options.pp_universes.to_string()),
            "pp.unicode" => Some(self.options.pp_unicode.to_string()),
            "pp.width" => Some(self.options.pp_width.to_string()),
            "show_timing" => Some(self.options.show_timing.to_string()),
            _ => self.settings.get(key).cloned(),
        }
    }
    /// Set an option value.
    pub fn set_option(&mut self, key: &str, value: &str) -> Result<(), String> {
        match key {
            "pp.all" => {
                self.options.pp_all = parse_bool(value)?;
                if self.options.pp_all {
                    self.options.pp_implicit = true;
                    self.options.pp_universes = true;
                }
            }
            "pp.implicit" => self.options.pp_implicit = parse_bool(value)?,
            "pp.universes" => self.options.pp_universes = parse_bool(value)?,
            "pp.unicode" => self.options.pp_unicode = parse_bool(value)?,
            "pp.width" => {
                self.options.pp_width = value
                    .parse()
                    .map_err(|_| format!("invalid width: {}", value))?;
            }
            "show_timing" => self.options.show_timing = parse_bool(value)?,
            _ => {
                self.settings.insert(key.to_string(), value.to_string());
            }
        }
        Ok(())
    }
    /// Unset (reset to default) an option.
    pub fn unset_option(&mut self, key: &str) {
        let defaults = ReplOptions::default();
        match key {
            "pp.all" => self.options.pp_all = defaults.pp_all,
            "pp.implicit" => self.options.pp_implicit = defaults.pp_implicit,
            "pp.universes" => self.options.pp_universes = defaults.pp_universes,
            "pp.unicode" => self.options.pp_unicode = defaults.pp_unicode,
            "pp.width" => self.options.pp_width = defaults.pp_width,
            "show_timing" => self.options.show_timing = defaults.show_timing,
            _ => {
                self.settings.remove(key);
            }
        }
    }
}
/// Completion result with display information.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Completion {
    /// The text to insert.
    pub text: String,
    /// Short description.
    pub description: String,
}
/// Session file format.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SessionSnapshot {
    /// Metadata about the session.
    pub metadata: SessionMetadata,
    /// Command history (non-command entries).
    pub history_entries: Vec<String>,
    /// Settings stored during the session.
    pub settings: HashMap<String, String>,
}
/// Simple syntax highlighter for terminal output.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SyntaxHighlighter {
    /// Whether to use ANSI color codes.
    pub(crate) use_colors: bool,
}
impl SyntaxHighlighter {
    /// Create a new syntax highlighter.
    pub fn new(use_colors: bool) -> Self {
        Self { use_colors }
    }
    /// Highlight a keyword.
    #[allow(dead_code)]
    pub fn highlight_keyword(&self, kw: &str) -> String {
        if self.use_colors {
            format!("\x1b[35m{}\x1b[0m", kw)
        } else {
            kw.to_string()
        }
    }
    /// Highlight a type.
    #[allow(dead_code)]
    pub fn highlight_type(&self, ty: &str) -> String {
        if self.use_colors {
            format!("\x1b[33m{}\x1b[0m", ty)
        } else {
            ty.to_string()
        }
    }
    /// Highlight a comment.
    #[allow(dead_code)]
    pub fn highlight_comment(&self, comment: &str) -> String {
        if self.use_colors {
            format!("\x1b[90m{}\x1b[0m", comment)
        } else {
            comment.to_string()
        }
    }
    /// Highlight a string.
    #[allow(dead_code)]
    pub fn highlight_string(&self, s: &str) -> String {
        if self.use_colors {
            format!("\x1b[32m{}\x1b[0m", s)
        } else {
            s.to_string()
        }
    }
    /// Highlight an error.
    #[allow(dead_code)]
    pub fn highlight_error(&self, err: &str) -> String {
        if self.use_colors {
            format!("\x1b[91m{}\x1b[0m", err)
        } else {
            err.to_string()
        }
    }
    /// Highlight success.
    #[allow(dead_code)]
    pub fn highlight_success(&self, msg: &str) -> String {
        if self.use_colors {
            format!("\x1b[92m{}\x1b[0m", msg)
        } else {
            msg.to_string()
        }
    }
}
/// REPL state.
pub struct Repl {
    /// Session state.
    state: ReplState,
    /// Line number counter.
    pub(crate) line_number: usize,
    /// Command history.
    pub(crate) history: History,
    /// Multi-line input buffer.
    pub(crate) input_buffer: InputBuffer,
    /// Tab completer.
    #[allow(dead_code)]
    completer: Completer,
    /// Syntax highlighter.
    #[allow(dead_code)]
    highlighter: SyntaxHighlighter,
    /// Error recovery context.
    #[allow(dead_code)]
    pub(crate) error_recovery: ErrorRecovery,
    /// Browse history (for :browse navigation).
    #[allow(dead_code)]
    pub(crate) browse_history: VecDeque<String>,
}
impl Repl {
    /// Create a new REPL.
    pub fn new() -> Self {
        Self {
            state: ReplState::new(),
            line_number: 1,
            history: History::new(1000),
            input_buffer: InputBuffer::new(),
            completer: Completer::new(),
            highlighter: SyntaxHighlighter::default(),
            error_recovery: ErrorRecovery::new(),
            browse_history: VecDeque::new(),
        }
    }
    /// Load the REPL history from a file.
    fn load_history(&mut self) -> io::Result<()> {
        let history_file = env::var("HOME")
            .map(|h| PathBuf::from(h).join(".oxilean_history"))
            .or_else(|_| env::var("USERPROFILE").map(|h| PathBuf::from(h).join(".oxilean_history")))
            .ok();
        if let Some(file) = history_file {
            if file.exists() {
                let _ = self.history.load_from_file(&file);
            }
        }
        Ok(())
    }
    /// Save the REPL history to a file.
    fn save_history(&self) -> io::Result<()> {
        let history_file = env::var("HOME")
            .map(|h| PathBuf::from(h).join(".oxilean_history"))
            .or_else(|_| env::var("USERPROFILE").map(|h| PathBuf::from(h).join(".oxilean_history")))
            .ok();
        if let Some(file) = history_file {
            let _ = self.history.save_to_file(&file);
        }
        Ok(())
    }
    /// Run the REPL.
    pub fn run(&mut self) -> io::Result<()> {
        let _ = self.load_history();
        println!("OxiLean REPL v{}", env!("CARGO_PKG_VERSION"));
        println!("Type :help for help, :quit to exit");
        println!();
        loop {
            self.print_prompt()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            if input.is_empty() {
                if self.input_buffer.is_continuation() {
                    let full = self.input_buffer.get_complete_input();
                    self.input_buffer.reset();
                    if !full.trim().is_empty() {
                        self.history.add_entry(full.clone());
                        self.eval_expr(&full);
                        self.line_number += 1;
                    }
                }
                continue;
            }
            if self.input_buffer.is_continuation() {
                self.input_buffer.add_line(input);
                if self.input_buffer.is_complete() {
                    let full = self.input_buffer.get_complete_input();
                    self.input_buffer.reset();
                    self.history.add_entry(full.clone());
                    self.eval_expr(&full);
                    self.line_number += 1;
                }
                continue;
            }
            if input.starts_with(':') {
                self.history.add_entry(input.to_string());
                if !self.handle_command(input) {
                    break;
                }
            } else {
                self.input_buffer.add_line(input);
                if self.input_buffer.is_complete() {
                    let full = self.input_buffer.get_complete_input();
                    self.input_buffer.reset();
                    self.history.add_entry(full.clone());
                    self.eval_expr(&full);
                    self.line_number += 1;
                }
            }
        }
        let _ = self.save_history();
        Ok(())
    }
    /// Print the appropriate prompt based on mode and continuation state.
    fn print_prompt(&self) -> io::Result<()> {
        if self.input_buffer.is_continuation() {
            print!("... ");
        } else {
            match self.state.mode {
                ReplMode::Normal => print!("oxilean[{}]> ", self.line_number),
                ReplMode::Proof => print!("proof[{}]> ", self.line_number),
                ReplMode::Debug => print!("debug[{}]> ", self.line_number),
            }
        }
        io::stdout().flush()
    }
    fn handle_command(&mut self, cmd: &str) -> bool {
        let parsed = parse_command(cmd);
        match parsed {
            ReplCmd::Quit => {
                println!("Goodbye!");
                return false;
            }
            ReplCmd::Help => self.print_help(),
            ReplCmd::ShowEnv => self.print_env(),
            ReplCmd::Clear => {
                self.state.reset_state();
                self.line_number = 1;
                println!("Environment cleared");
            }
            ReplCmd::Type(expr) => self.show_type(&expr),
            ReplCmd::Check(expr) => self.check_expr(&expr),
            ReplCmd::Load(path) => self.load_file(&path),
            ReplCmd::Reload => self.reload_file(),
            ReplCmd::Print(name) => self.print_definition(&name),
            ReplCmd::Search(pattern) => self.search_definitions(&pattern),
            ReplCmd::Info(name) => self.show_info(&name),
            ReplCmd::Browse(module) => self.browse_declarations(module),
            ReplCmd::Set(key, value) => self.set_option(&key, &value),
            ReplCmd::Unset(key) => self.unset_option(&key),
            ReplCmd::Time(expr) => self.time_expr(&expr),
            ReplCmd::Trace(expr) => self.trace_expr(&expr),
            ReplCmd::Undo => self.undo(),
            ReplCmd::Save(path) => self.save_session(&path),
            ReplCmd::LoadSession(path) => self.load_session(&path),
            ReplCmd::Axioms => self.list_axioms(),
            ReplCmd::ShowHistory => self.show_history(),
            ReplCmd::HistorySearch(query) => {
                if let Some(q) = query {
                    self.search_history(&q);
                } else {
                    self.show_history();
                }
            }
            ReplCmd::ShowProofState => self.show_proof_state(),
            ReplCmd::ListTactics => self.list_tactics(),
            ReplCmd::Eval(expr) => self.eval_to_normal_form(&expr),
            ReplCmd::Unknown(msg) => {
                if msg.starts_with("Unknown command:") {
                    println!("{}", msg);
                } else if msg.starts_with(':') {
                    println!("Unknown command: {}", msg);
                } else {
                    println!("Error: {}", msg);
                }
                println!("Type :help for available commands");
            }
        }
        true
    }
    fn eval_expr(&mut self, input: &str) {
        let start = Instant::now();
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        match parser.parse_expr() {
            Ok(surface_expr) => {
                let mut ctx = ElabContext::new(&self.state.env);
                match oxilean_elab::elaborate_expr(&mut ctx, &surface_expr) {
                    Ok(kernel_expr) => {
                        println!("  => {}", print_expr(&kernel_expr));
                        if self.state.options.show_timing {
                            let elapsed = start.elapsed();
                            println!("  ({}ms)", elapsed.as_millis());
                        }
                    }
                    Err(e) => {
                        println!("Elaboration error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                self.try_eval_declaration(input, start);
                if false {
                    println!("{}", e);
                }
            }
        }
    }
    fn try_eval_declaration(&mut self, input: &str, start: Instant) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        match parser.parse_decl() {
            Ok(_decl) => {
                println!("  Declaration parsed successfully.");
                if self.state.options.show_timing {
                    let elapsed = start.elapsed();
                    println!("  ({}ms)", elapsed.as_millis());
                }
            }
            Err(e) => {
                println!("Parse error: {}", e);
            }
        }
    }
    /// Evaluate a #check command.
    #[allow(dead_code)]
    fn eval_command_check(&self, input: &str) {
        let trimmed = input.trim_start_matches("#check").trim();
        self.check_expr(trimmed);
    }
    /// Evaluate a #eval command.
    #[allow(dead_code)]
    fn eval_command_eval(&self, input: &str) {
        let trimmed = input.trim_start_matches("#eval").trim();
        let mut lexer = Lexer::new(trimmed);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        match parser.parse_expr() {
            Ok(surface_expr) => {
                let mut ctx = ElabContext::new(&self.state.env);
                match oxilean_elab::elaborate_expr(&mut ctx, &surface_expr) {
                    Ok(kernel_expr) => {
                        let mut reducer = Reducer::new();
                        let reduced = reducer.whnf(&kernel_expr);
                        println!("  {}", print_expr(&reduced));
                    }
                    Err(e) => {
                        println!("Elaboration error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("Parse error: {}", e);
            }
        }
    }
    fn show_type(&self, expr: &str) {
        let mut lexer = Lexer::new(expr);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        match parser.parse_expr() {
            Ok(surface_expr) => {
                let mut ctx = ElabContext::new(&self.state.env);
                match oxilean_elab::elaborate_expr(&mut ctx, &surface_expr) {
                    Ok(kernel_expr) => {
                        let mut tc = oxilean_kernel::TypeChecker::new(&self.state.env);
                        match tc.infer_type(&kernel_expr) {
                            Ok(ty) => {
                                println!("  : {}", print_expr(&ty));
                            }
                            Err(e) => {
                                println!("Type error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Elaboration error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("Parse error: {}", e);
            }
        }
    }
    fn check_expr(&self, expr: &str) {
        let mut lexer = Lexer::new(expr);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        match parser.parse_expr() {
            Ok(surface_expr) => {
                let mut ctx = ElabContext::new(&self.state.env);
                match oxilean_elab::elaborate_expr(&mut ctx, &surface_expr) {
                    Ok(kernel_expr) => {
                        let mut tc = oxilean_kernel::TypeChecker::new(&self.state.env);
                        match tc.infer_type(&kernel_expr) {
                            Ok(ty) => {
                                println!("[ok] Expression is well-typed");
                                println!("  Type: {}", print_expr(&ty));
                            }
                            Err(e) => {
                                println!("[error] Type error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("[error] Elaboration error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("[error] Parse error: {}", e);
            }
        }
    }
    fn load_file(&mut self, path: &str) {
        let path_buf = PathBuf::from(path);
        match std::fs::read_to_string(&path_buf) {
            Ok(contents) => {
                println!("Loading {}...", path);
                let mut lexer = Lexer::new(&contents);
                let tokens = lexer.tokenize();
                let mut parser = Parser::new(tokens);
                let mut decl_count = 0;
                loop {
                    match parser.parse_decl() {
                        Ok(_decl) => {
                            decl_count += 1;
                        }
                        Err(e) => {
                            if e.to_string().contains("end of file")
                                || e.to_string().contains("Eof")
                            {
                                break;
                            }
                            println!("Parse error in {}: {}", path, e);
                            return;
                        }
                    }
                }
                self.state.loaded_files.push(path_buf.clone());
                self.state.last_loaded = Some(path_buf);
                println!("  Loaded {} declaration(s) from {}", decl_count, path);
            }
            Err(e) => {
                println!("Failed to read file {}: {}", path, e);
            }
        }
    }
    fn reload_file(&mut self) {
        match self.state.last_loaded.clone() {
            Some(path) => {
                let path_str = path.to_string_lossy().to_string();
                self.load_file(&path_str);
            }
            None => {
                println!("No file has been loaded yet");
            }
        }
    }
    fn print_definition(&self, name: &str) {
        let kernel_name = Name::str(name);
        match self.state.env.get(&kernel_name) {
            Some(decl) => match decl {
                Declaration::Axiom { name, ty, .. } => {
                    println!("axiom {} : {}", name, print_expr(ty));
                }
                Declaration::Definition { name, ty, val, .. } => {
                    println!(
                        "def {} : {} :=\n  {}",
                        name,
                        print_expr(ty),
                        print_expr(val)
                    );
                }
                Declaration::Theorem { name, ty, val, .. } => {
                    println!(
                        "theorem {} : {} :=\n  {}",
                        name,
                        print_expr(ty),
                        print_expr(val)
                    );
                }
                Declaration::Opaque { name, ty, .. } => {
                    println!("opaque {} : {}", name, print_expr(ty));
                }
            },
            None => match self.state.env.find(&kernel_name) {
                Some(ci) => {
                    println!("{} : {}", ci.name(), print_expr(ci.ty()));
                }
                None => {
                    println!("Declaration '{}' not found", name);
                }
            },
        }
    }
    fn search_definitions(&self, pattern: &str) {
        let lower_pattern = pattern.to_lowercase();
        let mut found = 0;
        for name in self.state.env.constant_names() {
            let name_str = name.to_string().to_lowercase();
            if name_str.contains(&lower_pattern) {
                if let Some(ci) = self.state.env.find(name) {
                    println!("  {} : {}", name, print_expr(ci.ty()));
                    found += 1;
                }
            }
        }
        if found == 0 {
            println!("No declarations matching '{}' found", pattern);
        } else {
            println!("  ({} result(s))", found);
        }
    }
    fn show_info(&self, name: &str) {
        let kernel_name = Name::str(name);
        match self.state.env.find(&kernel_name) {
            Some(ci) => {
                println!("Name:   {}", ci.name());
                println!("Type:   {}", print_expr(ci.ty()));
                let kind = if ci.is_axiom() {
                    "axiom"
                } else if ci.is_definition() {
                    "definition"
                } else if ci.is_theorem() {
                    "theorem"
                } else if ci.is_inductive() {
                    "inductive"
                } else if ci.is_constructor() {
                    "constructor"
                } else if ci.is_recursor() {
                    "recursor"
                } else {
                    "unknown"
                };
                println!("Kind:   {}", kind);
                let params = ci.level_params();
                if !params.is_empty() {
                    let param_strs: Vec<String> = params.iter().map(|p| p.to_string()).collect();
                    println!("Univs:  {}", param_strs.join(", "));
                }
                if let Some(val) = ci.value() {
                    println!("Value:  {}", print_expr(val));
                }
            }
            None => {
                println!("Declaration '{}' not found", name);
            }
        }
    }
    fn set_option(&mut self, key: &str, value: &str) {
        match self.state.set_option(key, value) {
            Ok(()) => {
                println!("Set {} = {}", key, value);
            }
            Err(e) => {
                println!("Error setting option: {}", e);
            }
        }
    }
    fn unset_option(&mut self, key: &str) {
        self.state.unset_option(key);
        println!("Unset {}", key);
    }
    fn time_expr(&self, expr: &str) {
        let start = Instant::now();
        let mut lexer = Lexer::new(expr);
        let tokens = lexer.tokenize();
        let parse_time = start.elapsed();
        let mut parser = Parser::new(tokens);
        match parser.parse_expr() {
            Ok(surface_expr) => {
                let elab_start = Instant::now();
                let mut ctx = ElabContext::new(&self.state.env);
                match oxilean_elab::elaborate_expr(&mut ctx, &surface_expr) {
                    Ok(kernel_expr) => {
                        let elab_time = elab_start.elapsed();
                        let check_start = Instant::now();
                        let mut tc = oxilean_kernel::TypeChecker::new(&self.state.env);
                        match tc.infer_type(&kernel_expr) {
                            Ok(ty) => {
                                let check_time = check_start.elapsed();
                                let total = start.elapsed();
                                println!("  : {}", print_expr(&ty));
                                println!("  Parse:       {}us", parse_time.as_micros());
                                println!("  Elaborate:   {}us", elab_time.as_micros());
                                println!("  Type check:  {}us", check_time.as_micros());
                                println!("  Total:       {}us", total.as_micros());
                            }
                            Err(e) => {
                                println!("Type error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("Elaboration error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("Parse error: {}", e);
            }
        }
    }
    fn trace_expr(&self, expr: &str) {
        println!("  [trace] Parsing: {}", expr);
        let mut lexer = Lexer::new(expr);
        let tokens = lexer.tokenize();
        println!("  [trace] Tokenized: {} tokens", tokens.len());
        let mut parser = Parser::new(tokens);
        match parser.parse_expr() {
            Ok(surface_expr) => {
                println!("  [trace] Parsed: {:?}", surface_expr.value);
                let mut ctx = ElabContext::new(&self.state.env);
                match oxilean_elab::elaborate_expr(&mut ctx, &surface_expr) {
                    Ok(kernel_expr) => {
                        println!("  [trace] Elaborated: {}", print_expr(&kernel_expr));
                        let mut tc = oxilean_kernel::TypeChecker::new(&self.state.env);
                        match tc.infer_type(&kernel_expr) {
                            Ok(ty) => {
                                println!("  [trace] Inferred type: {}", print_expr(&ty));
                            }
                            Err(e) => {
                                println!("  [trace] Type error: {}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("  [trace] Elaboration error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("  [trace] Parse error: {}", e);
            }
        }
    }
    fn undo(&mut self) {
        match self.state.pop_undo() {
            Some(entry) => {
                println!("Undone: {} ({})", entry.name, entry.description);
            }
            None => {
                println!("Nothing to undo");
            }
        }
    }
    fn save_session(&self, path: &str) {
        let mut content = String::new();
        content.push_str("-- OxiLean REPL session\n");
        content.push_str(&format!("-- {} declarations\n\n", self.state.env.len()));
        for entry in self.history.entries() {
            if !entry.text.starts_with(':') {
                content.push_str(&entry.text);
                content.push('\n');
            }
        }
        match std::fs::write(path, content) {
            Ok(()) => println!("Session saved to {}", path),
            Err(e) => println!("Failed to save session: {}", e),
        }
    }
    /// Show current proof state: goals and their hypotheses.
    fn show_proof_state(&self) {
        if self.state.active_proof_state.is_empty() {
            println!("No active proof. Start a proof with 'theorem <name> : <type> := by'");
            return;
        }
        let goals = &self.state.active_proof_state;
        println!("{} goal(s):", goals.len());
        for (i, (hyps, target)) in goals.iter().enumerate() {
            println!();
            println!("  Goal {}:", i + 1);
            if hyps.is_empty() {
                println!("    (no hypotheses)");
            } else {
                for (name, ty) in hyps {
                    println!("    {} : {}", name, ty);
                }
            }
            println!("    ⊢ {}", target);
        }
    }
    /// List all available tactics with their descriptions.
    fn list_tactics(&self) {
        println!("Available Tactics:");
        println!();
        println!("  Introduction:");
        println!("    intro [name]          Introduce a Pi-bound variable as hypothesis");
        println!("    intros [names..]      Introduce multiple variables");
        println!("    revert <name>         Move hypothesis back into goal");
        println!();
        println!("  Closing Goals:");
        println!("    exact <term>          Close goal with an exact proof term");
        println!("    assumption            Close goal using a matching hypothesis");
        println!("    refl                  Close a reflexivity goal (a = a)");
        println!("    trivial               Try obvious closures (refl, assumption)");
        println!("    sorry                 Admit the goal without proof");
        println!();
        println!("  Applying Lemmas:");
        println!("    apply <term>          Apply a lemma, creating sub-goals for args");
        println!("    specialize <h> <args> Specialize hypothesis h with arguments");
        println!();
        println!("  Logical Structure:");
        println!("    constructor           Split a conjunction or intro an Iff");
        println!("    left                  Choose left disjunct");
        println!("    right                 Choose right disjunct");
        println!("    cases <h>             Case split on a hypothesis");
        println!("    induction <h>         Induct on a natural number hypothesis");
        println!("    split                 Split an Iff goal into two directions");
        println!("    obtain <pat> := <h>   Destructure a hypothesis");
        println!();
        println!("  Hypothesis Management:");
        println!("    have <h> : <T>        Introduce a new hypothesis sub-goal");
        println!("    show <T>              Change the goal to a definitionally equal type");
        println!("    clear <h>             Remove a hypothesis");
        println!("    rename <h> <h'>       Rename a hypothesis");
        println!();
        println!("  Negation / Contradiction:");
        println!("    exfalso               Change goal to False");
        println!("    contradiction         Close goal if hypotheses are contradictory");
        println!("    by_contra [h]         Introduce negation of goal as hypothesis h");
        println!("    contrapose            Swap and negate goal and hypotheses");
        println!("    push_neg              Push negation inward through quantifiers");
        println!();
        println!("  Rewriting:");
        println!("    rw [h]                Rewrite goal using equality hypothesis h");
        println!("    rw [<- h]             Rewrite in the reverse direction");
        println!("    rw [h] at hyp         Rewrite inside hypothesis hyp");
        println!();
        println!("  Automation:");
        println!("    simp [lemmas..]       Simplify using built-in + given lemmas");
        println!("    simp only [lemmas..]  Simplify using only given lemmas");
        println!("    simp_all              Simplify using all hypotheses and lemmas");
        println!("    ring                  Prove ring equalities");
        println!("    linarith              Prove linear arithmetic goals");
        println!("    omega                 Prove linear integer/nat arithmetic");
        println!("    norm_num              Prove numeric normalization goals");
        println!("    decide                Decide decidable propositions");
        println!("    field_simp            Simplify field expressions");
        println!();
        println!("  Existentials:");
        println!("    exists <witness>      Provide a witness for an Exists goal");
        println!("    use <witness>         Alias for exists");
        println!();
        println!("  Combinators:");
        println!("    repeat <tac>          Repeat tactic until failure");
        println!("    try <tac>             Run tactic, ignore failure");
        println!("    first | t1 | t2       Try t1, then t2 on failure");
        println!("    all_goals <tac>       Apply tactic to all goals");
        println!();
        println!("  Other:");
        println!("    done                  Assert no goals remain");
        println!("    ext / funext          Extensionality");
        println!("    congr                 Congruence closure");
        println!("    calc                  Begin a calculational proof chain");
        println!("    norm_cast             Normalize numeric cast goals");
        println!("    push_cast             Push casts inward");
    }
    /// Evaluate an expression and print it in normal form.
    fn eval_to_normal_form(&self, expr: &str) {
        let mut lexer = oxilean_parse::Lexer::new(expr);
        let tokens = lexer.tokenize();
        let mut parser = oxilean_parse::Parser::new(tokens);
        match parser.parse_expr() {
            Ok(surface_expr) => {
                let mut ctx = ElabContext::new(&self.state.env);
                match oxilean_elab::elaborate_expr(&mut ctx, &surface_expr) {
                    Ok(kernel_expr) => {
                        let mut reducer = Reducer::new();
                        let nf = reducer.whnf(&kernel_expr);
                        println!("  {}", oxilean_kernel::print_expr(&nf));
                    }
                    Err(e) => {
                        println!("Elaboration error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("Parse error: {}", e);
            }
        }
    }
    fn list_axioms(&self) {
        if self.state.axiom_names.is_empty() {
            let mut found = false;
            for (name, ci) in self.state.env.constant_infos() {
                if ci.is_axiom() {
                    println!("  axiom {} : {}", name, print_expr(ci.ty()));
                    found = true;
                }
            }
            if !found {
                println!("No axioms in the current environment");
            }
        } else {
            for name in &self.state.axiom_names {
                if let Some(ci) = self.state.env.find(name) {
                    println!("  axiom {} : {}", name, print_expr(ci.ty()));
                }
            }
        }
        if self.state.used_sorry {
            println!("  [warning] sorry was used in this session");
        }
    }
    fn show_history(&self) {
        if self.history.is_empty() {
            println!("No history");
            return;
        }
        for (i, entry) in self.history.entries().iter().enumerate() {
            println!("  {}: {}", i + 1, entry.text);
        }
    }
    fn browse_declarations(&mut self, module: Option<String>) {
        match module {
            Some(mod_name) => {
                let lower_mod = mod_name.to_lowercase();
                let mut found = 0;
                for name in self.state.env.constant_names() {
                    let name_str = name.to_string();
                    if name_str.to_lowercase().starts_with(&lower_mod) {
                        if let Some(ci) = self.state.env.find(name) {
                            println!("  {} : {}", name, print_expr(ci.ty()));
                            found += 1;
                        }
                    }
                }
                if found == 0 {
                    println!("No declarations found in module '{}'", mod_name);
                } else {
                    println!("  ({} declaration(s))", found);
                }
                self.browse_history.push_front(mod_name);
                if self.browse_history.len() > 20 {
                    self.browse_history.pop_back();
                }
            }
            None => {
                let mut all_decls: Vec<String> = self
                    .state
                    .env
                    .constant_names()
                    .map(|n| n.to_string())
                    .collect();
                all_decls.sort();
                for name_str in all_decls {
                    let name = Name::str(&name_str);
                    if let Some(ci) = self.state.env.find(&name) {
                        let kind = if ci.is_axiom() {
                            "axiom"
                        } else if ci.is_definition() {
                            "def"
                        } else if ci.is_theorem() {
                            "theorem"
                        } else {
                            "decl"
                        };
                        println!("  [{}] {} : {}", kind, name_str, print_expr(ci.ty()));
                    }
                }
            }
        }
    }
    fn search_history(&self, query: &str) {
        let results = self.history.search(query);
        if results.is_empty() {
            println!("No history entries matching '{}'", query);
        } else {
            println!("History search results for '{}':", query);
            for (i, entry) in results.iter().enumerate() {
                println!("  {} : {}", i + 1, entry);
            }
        }
    }
    fn load_session(&mut self, path: &str) {
        match fs::read_to_string(path) {
            Ok(contents) => {
                let lines: Vec<&str> = contents.lines().collect();
                let mut loaded_count = 0;
                for line in lines {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with("--") {
                        continue;
                    }
                    self.input_buffer.add_line(trimmed);
                    if self.input_buffer.is_complete() {
                        let full = self.input_buffer.get_complete_input();
                        self.input_buffer.reset();
                        self.eval_expr(&full);
                        loaded_count += 1;
                        self.line_number += 1;
                    }
                }
                if loaded_count > 0 {
                    println!("Loaded {} statement(s) from session file", loaded_count);
                }
            }
            Err(e) => {
                println!("Failed to load session file: {}", e);
            }
        }
    }
    fn print_help(&self) {
        println!("OxiLean REPL Commands:");
        println!("  :quit, :q              Exit REPL");
        println!("  :help, :h              Show this help");
        println!("  :type <expr>           Show type of expression");
        println!("  :check <expr>          Check expression");
        println!("  :env                   Show environment");
        println!("  :clear                 Clear environment");
        println!("  :load <file>           Load a source file");
        println!("  :reload, :r            Reload last loaded file");
        println!("  :print <name>          Print definition");
        println!("  :search <pattern>      Search definitions by name");
        println!("  :info <name>           Show detailed info about a declaration");
        println!("  :browse [module]       Browse declarations (optionally in a module)");
        println!("  :set <opt> <val>       Set REPL option (e.g., :set pp.all true)");
        println!("  :unset <opt>           Reset option to default");
        println!("  :time <expr>           Time elaboration/checking");
        println!("  :trace <expr>          Show elaboration trace");
        println!("  :undo                  Undo last declaration");
        println!("  :save <file>           Save session to file");
        println!("  :load-session <file>   Load a session from file");
        println!("  :axioms                List all axioms used");
        println!("  :proof, :goals         Show current proof state (goals and hypotheses)");
        println!("  :tactics               List all available tactics with descriptions");
        println!("  :eval <expr>           Evaluate expression to normal form");
        println!("  :history [query]       Show command history (optionally search)");
        println!();
        println!("Options:");
        println!("  pp.all             Show all (implicit args, universes)");
        println!("  pp.implicit        Show implicit arguments");
        println!("  pp.universes       Show universe levels");
        println!("  pp.unicode         Use unicode output");
        println!("  pp.width           Maximum line width");
        println!("  show_timing        Show timing information");
        println!();
        println!("Examples:");
        println!("  Type                -- The type of types");
        println!("  42                  -- Natural number literal");
        println!("  :type Type          -- Show the type of Type");
        println!("  :set pp.all true    -- Show all details");
    }
    fn print_env(&self) {
        if self.state.env.is_empty() {
            println!("Environment is empty");
            return;
        }
        println!(
            "Current environment ({} declarations):",
            self.state.env.len()
        );
        for (name, ci) in self.state.env.constant_infos() {
            let kind = if ci.is_axiom() {
                "axiom"
            } else if ci.is_definition() {
                "def"
            } else if ci.is_theorem() {
                "theorem"
            } else if ci.is_inductive() {
                "inductive"
            } else if ci.is_constructor() {
                "constructor"
            } else if ci.is_recursor() {
                "recursor"
            } else {
                "decl"
            };
            println!("  {} {} : {}", kind, name, print_expr(ci.ty()));
        }
    }
}
/// Buffer for accumulating multi-line input.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InputBuffer {
    /// Lines accumulated so far.
    lines: Vec<String>,
    /// Whether the input is in a continuation state.
    in_continuation: bool,
}
#[allow(dead_code)]
impl InputBuffer {
    /// Create a new empty input buffer.
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            in_continuation: false,
        }
    }
    /// Add a line to the buffer.
    pub fn add_line(&mut self, line: &str) {
        self.lines.push(line.to_string());
        self.in_continuation = !self.is_complete();
    }
    /// Check if the accumulated input is complete (no open delimiters, etc.).
    pub fn is_complete(&self) -> bool {
        let combined = self.lines.join("\n");
        let trimmed = combined.trim();
        if trimmed.is_empty() {
            return true;
        }
        let mut paren_depth: i32 = 0;
        let mut brace_depth: i32 = 0;
        let mut bracket_depth: i32 = 0;
        let mut in_string = false;
        let mut in_line_comment = false;
        let mut prev_char = '\0';
        for ch in trimmed.chars() {
            if in_line_comment {
                if ch == '\n' {
                    in_line_comment = false;
                }
                prev_char = ch;
                continue;
            }
            if in_string {
                if ch == '"' && prev_char != '\\' {
                    in_string = false;
                }
                prev_char = ch;
                continue;
            }
            match ch {
                '"' => in_string = true,
                '-' if prev_char == '-' => {
                    in_line_comment = true;
                }
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                '{' => brace_depth += 1,
                '}' => brace_depth -= 1,
                '[' => bracket_depth += 1,
                ']' => bracket_depth -= 1,
                _ => {}
            }
            prev_char = ch;
        }
        if paren_depth > 0 || brace_depth > 0 || bracket_depth > 0 || in_string {
            return false;
        }
        if trimmed.ends_with("by") || trimmed.ends_with("where") || trimmed.ends_with("do") {
            return false;
        }
        if trimmed.ends_with('\\') {
            return false;
        }
        let begin_count = trimmed.matches("begin").count();
        let end_count = trimmed.matches("end").count();
        if begin_count > end_count {
            return false;
        }
        true
    }
    /// Get the complete input as a single string.
    pub fn get_complete_input(&self) -> String {
        self.lines.join("\n")
    }
    /// Reset the buffer for a new input.
    pub fn reset(&mut self) {
        self.lines.clear();
        self.in_continuation = false;
    }
    /// Check if the buffer is in continuation mode.
    pub fn is_continuation(&self) -> bool {
        self.in_continuation
    }
    /// Check if the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    /// Get the number of lines in the buffer.
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}
/// Entry in the command history.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HistoryEntry {
    /// The command text.
    pub text: String,
    /// Timestamp (seconds since epoch, if available).
    pub timestamp: u64,
}
/// The current mode of the REPL.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ReplMode {
    /// Normal expression/declaration mode.
    Normal,
    /// Proof mode (interacting with tactic state).
    Proof,
    /// Debug mode (step through elaboration).
    Debug,
}
/// Session metadata for serialization.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SessionMetadata {
    /// Session creation timestamp.
    pub created_at: u64,
    /// Last modified timestamp.
    pub modified_at: u64,
    /// Number of declarations in the session.
    pub declaration_count: usize,
    /// Session name/description.
    pub name: String,
}
/// Error recovery context for maintaining REPL state.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ErrorRecovery {
    /// Last known good state (for rollback).
    pub(crate) last_good_env: Option<Environment>,
    /// Number of consecutive errors.
    pub(crate) consecutive_errors: usize,
    /// Maximum consecutive errors before auto-reset.
    max_consecutive_errors: usize,
}
impl ErrorRecovery {
    /// Create a new error recovery context.
    pub fn new() -> Self {
        Self {
            last_good_env: None,
            consecutive_errors: 0,
            max_consecutive_errors: 5,
        }
    }
    /// Record a successful operation (reset error counter).
    #[allow(dead_code)]
    pub fn record_success(&mut self, env: Environment) {
        self.last_good_env = Some(env);
        self.consecutive_errors = 0;
    }
    /// Record an error and check if recovery is needed.
    #[allow(dead_code)]
    pub fn record_error(&mut self) -> bool {
        self.consecutive_errors += 1;
        self.consecutive_errors >= self.max_consecutive_errors
    }
    /// Get the last good environment (for rollback).
    #[allow(dead_code)]
    pub fn get_last_good(&self) -> Option<&Environment> {
        self.last_good_env.as_ref()
    }
    /// Reset error counter.
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.consecutive_errors = 0;
    }
}
/// A parsed REPL command.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum ReplCmd {
    /// Quit the REPL.
    Quit,
    /// Show help.
    Help,
    /// Show the environment.
    ShowEnv,
    /// Clear the environment.
    Clear,
    /// Show type of expression.
    Type(String),
    /// Check an expression.
    Check(String),
    /// Load a file.
    Load(String),
    /// Reload last loaded file.
    Reload,
    /// Print a definition.
    Print(String),
    /// Search definitions by pattern.
    Search(String),
    /// Show info about a declaration.
    Info(String),
    /// Browse declarations (optionally in a module).
    Browse(Option<String>),
    /// Set an option.
    Set(String, String),
    /// Unset an option.
    Unset(String),
    /// Time an expression.
    Time(String),
    /// Trace elaboration of an expression.
    Trace(String),
    /// Undo last declaration.
    Undo,
    /// Save session to file.
    Save(String),
    /// Load a session from file.
    LoadSession(String),
    /// List axioms used.
    Axioms,
    /// Show history.
    ShowHistory,
    /// Show or manipulate history search.
    HistorySearch(Option<String>),
    /// Show the current proof state (goals, hypotheses).
    ShowProofState,
    /// List all available tactics with descriptions.
    ListTactics,
    /// Evaluate an expression and print its normal form.
    Eval(String),
    /// Unknown command.
    Unknown(String),
}
/// Command history with navigation and search.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct History {
    /// All history entries.
    entries: Vec<HistoryEntry>,
    /// Current navigation position (-1 means not navigating).
    position: Option<usize>,
    /// Maximum number of entries to keep.
    max_size: usize,
    /// Search query buffer.
    search_query: String,
}
#[allow(dead_code)]
impl History {
    /// Create a new history with the given max size.
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: Vec::new(),
            position: None,
            max_size,
            search_query: String::new(),
        }
    }
    /// Add an entry to the history, deduplicating consecutive identical entries.
    pub fn add_entry(&mut self, text: String) {
        let trimmed = text.trim().to_string();
        if trimmed.is_empty() {
            return;
        }
        if let Some(last) = self.entries.last() {
            if last.text == trimmed {
                self.position = None;
                return;
            }
        }
        self.entries.push(HistoryEntry {
            text: trimmed,
            timestamp: 0,
        });
        while self.entries.len() > self.max_size {
            self.entries.remove(0);
        }
        self.position = None;
    }
    /// Navigate to the previous entry.
    pub fn previous(&mut self) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }
        let new_pos = match self.position {
            None => self.entries.len().saturating_sub(1),
            Some(0) => 0,
            Some(pos) => pos.saturating_sub(1),
        };
        self.position = Some(new_pos);
        self.entries.get(new_pos).map(|e| e.text.as_str())
    }
    /// Navigate to the next entry.
    pub fn next(&mut self) -> Option<&str> {
        match self.position {
            None => None,
            Some(pos) => {
                if pos + 1 >= self.entries.len() {
                    self.position = None;
                    None
                } else {
                    let new_pos = pos + 1;
                    self.position = Some(new_pos);
                    self.entries.get(new_pos).map(|e| e.text.as_str())
                }
            }
        }
    }
    /// Search history entries for a pattern (most recent first).
    pub fn search(&self, pattern: &str) -> Vec<&str> {
        let lower_pat = pattern.to_lowercase();
        self.entries
            .iter()
            .rev()
            .filter(|e| e.text.to_lowercase().contains(&lower_pat))
            .map(|e| e.text.as_str())
            .collect()
    }
    /// Save history to a file.
    pub fn save_to_file(&self, path: &PathBuf) -> io::Result<()> {
        let mut content = String::new();
        for entry in &self.entries {
            content.push_str(&entry.text);
            content.push('\n');
        }
        std::fs::write(path, content)
    }
    /// Load history from a file.
    pub fn load_from_file(&mut self, path: &PathBuf) -> io::Result<()> {
        let content = std::fs::read_to_string(path)?;
        for line in content.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                self.entries.push(HistoryEntry {
                    text: trimmed.to_string(),
                    timestamp: 0,
                });
            }
        }
        while self.entries.len() > self.max_size {
            self.entries.remove(0);
        }
        Ok(())
    }
    /// Get the number of entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Check if the history is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Clear the history.
    pub fn clear(&mut self) {
        self.entries.clear();
        self.position = None;
    }
    /// Get all entries.
    pub fn entries(&self) -> &[HistoryEntry] {
        &self.entries
    }
    /// Reset navigation position.
    pub fn reset_position(&mut self) {
        self.position = None;
    }
    /// Perform a reverse search (like Ctrl+R in bash).
    pub fn search_reverse(&mut self, query: &str) -> Option<&str> {
        self.search_query = query.to_string();
        let results = self.search(query);
        if !results.is_empty() {
            for (i, entry) in self.entries.iter().enumerate().rev() {
                if entry.text.contains(query) {
                    self.position = Some(i);
                    return Some(&entry.text);
                }
            }
        }
        None
    }
    /// Get the current search query.
    pub fn current_search_query(&self) -> &str {
        &self.search_query
    }
}
/// An undo entry capturing a snapshot of the environment.
#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct UndoEntry {
    /// Name of the declaration that was added.
    pub name: Name,
    /// Snapshot of the environment before the declaration.
    pub old_env: Environment,
    /// Description of the action.
    pub description: String,
}
/// Tab completion engine for the REPL.
#[allow(dead_code)]
pub struct Completer {
    /// Known language keywords.
    pub(crate) keywords: Vec<String>,
    /// Known tactic names.
    pub(crate) tactics: Vec<String>,
    /// Known REPL commands.
    pub(crate) commands: Vec<String>,
}
#[allow(dead_code)]
impl Completer {
    /// Create a new completer with the built-in word lists.
    pub fn new() -> Self {
        Self {
            keywords: vec![
                "def",
                "theorem",
                "lemma",
                "axiom",
                "constant",
                "inductive",
                "structure",
                "class",
                "instance",
                "where",
                "let",
                "in",
                "if",
                "then",
                "else",
                "match",
                "with",
                "do",
                "return",
                "fun",
                "forall",
                "import",
                "open",
                "namespace",
                "section",
                "end",
                "variable",
                "universe",
                "noncomputable",
                "partial",
                "unsafe",
                "private",
                "protected",
                "mutual",
                "attribute",
                "deriving",
                "Type",
                "Prop",
                "Sort",
                "Nat",
                "Bool",
                "String",
                "List",
                "true",
                "false",
                "sorry",
                "by",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            tactics: vec![
                "intro",
                "intros",
                "apply",
                "exact",
                "assumption",
                "refl",
                "trivial",
                "simp",
                "ring",
                "omega",
                "decide",
                "norm_num",
                "constructor",
                "cases",
                "induction",
                "rewrite",
                "rw",
                "have",
                "suffices",
                "show",
                "calc",
                "left",
                "right",
                "exfalso",
                "contradiction",
                "sorry",
                "done",
                "clear",
                "rename",
                "revert",
                "specialize",
                "generalize",
                "obtain",
                "rcases",
                "ext",
                "funext",
                "congr",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
            commands: vec![
                ":quit",
                ":q",
                ":help",
                ":h",
                ":env",
                ":clear",
                ":type",
                ":check",
                ":load",
                ":reload",
                ":r",
                ":print",
                ":p",
                ":search",
                ":info",
                ":i",
                ":browse",
                ":b",
                ":set",
                ":unset",
                ":time",
                ":trace",
                ":undo",
                ":save",
                ":axioms",
                ":history",
                ":load-session",
                ":proof",
                ":goals",
                ":tactics",
                ":tactic-list",
                ":eval",
                ":e",
            ]
            .into_iter()
            .map(String::from)
            .collect(),
        }
    }
    /// Complete the given input prefix.
    pub fn complete(&self, input: &str, env: &Environment) -> Vec<Completion> {
        let trimmed = input.trim();
        if trimmed.starts_with(':') {
            return self.complete_command(trimmed);
        }
        let last_word = trimmed.split_whitespace().last().unwrap_or("");
        if last_word.is_empty() {
            return Vec::new();
        }
        let mut results = Vec::new();
        results.extend(self.complete_keyword(last_word));
        results.extend(self.complete_tactic(last_word));
        results.extend(self.complete_name(last_word, env));
        let mut seen = std::collections::HashSet::new();
        results.retain(|c| seen.insert(c.text.clone()));
        results
    }
    /// Complete a keyword.
    pub fn complete_keyword(&self, prefix: &str) -> Vec<Completion> {
        self.keywords
            .iter()
            .filter(|kw| kw.starts_with(prefix))
            .map(|kw| Completion {
                text: kw.clone(),
                description: "keyword".to_string(),
            })
            .collect()
    }
    /// Complete a name from the environment.
    pub fn complete_name(&self, prefix: &str, env: &Environment) -> Vec<Completion> {
        let mut results = Vec::new();
        for name in env.constant_names() {
            let name_str = name.to_string();
            if name_str.starts_with(prefix) {
                results.push(Completion {
                    text: name_str,
                    description: "declaration".to_string(),
                });
            }
        }
        results
    }
    /// Complete a tactic name.
    pub fn complete_tactic(&self, prefix: &str) -> Vec<Completion> {
        self.tactics
            .iter()
            .filter(|t| t.starts_with(prefix))
            .map(|t| Completion {
                text: t.clone(),
                description: "tactic".to_string(),
            })
            .collect()
    }
    /// Complete a REPL command.
    pub fn complete_command(&self, prefix: &str) -> Vec<Completion> {
        self.commands
            .iter()
            .filter(|c| c.starts_with(prefix))
            .map(|c| Completion {
                text: c.clone(),
                description: "command".to_string(),
            })
            .collect()
    }
    /// Find the common prefix among a list of completions.
    pub fn get_common_prefix(completions: &[Completion]) -> String {
        if completions.is_empty() {
            return String::new();
        }
        if completions.len() == 1 {
            return completions[0].text.clone();
        }
        let first = &completions[0].text;
        let mut prefix_len = first.len();
        for completion in &completions[1..] {
            let common = first
                .chars()
                .zip(completion.text.chars())
                .take_while(|(a, b)| a == b)
                .count();
            prefix_len = prefix_len.min(common);
        }
        first[..prefix_len].to_string()
    }
    /// Get context-aware completions (for more intelligent suggestions).
    pub fn complete_with_context(
        &self,
        input: &str,
        env: &Environment,
        context: &str,
    ) -> Vec<Completion> {
        let mut results = self.complete(input, env);
        if context.contains("proof") || context.contains("by") {
            results.sort_by_key(|c| {
                if self.tactics.iter().any(|t| t == &c.text) {
                    0
                } else {
                    1
                }
            });
        } else if context.contains("def") || context.contains("let") {
            results.sort_by_key(|c| {
                if self.keywords.iter().any(|k| k == &c.text) {
                    0
                } else {
                    1
                }
            });
        }
        results
    }
    /// Get short help for a completion.
    pub fn get_help(&self, text: &str) -> String {
        match text {
            "intro" => "intro [name] - Introduce a hypothesis from a Pi type".to_string(),
            "apply" => "apply [h] - Apply a lemma or hypothesis".to_string(),
            "exact" => "exact [e] - Provide an exact proof term".to_string(),
            "simp" => "simp [lemmas] - Simplify the goal".to_string(),
            "refl" => "refl - Prove reflexivity goals (a = a)".to_string(),
            "assumption" => "assumption - Search context for matching hypothesis".to_string(),
            "rw" | "rewrite" => "rw [eq] - Rewrite the goal using an equation".to_string(),
            "cases" => "cases [h] - Case split on an inductive hypothesis".to_string(),
            "constructor" => "constructor - Apply the constructor of the target type".to_string(),
            "sorry" => "sorry - Admit the goal (WARNING: proof incomplete)".to_string(),
            ":type" => ":type [expr] - Show the type of an expression".to_string(),
            ":info" => ":info [name] - Show information about a declaration".to_string(),
            ":browse" => ":browse [module] - Browse declarations in a module".to_string(),
            _ => format!("Help for '{}' not available", text),
        }
    }
}
