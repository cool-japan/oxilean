//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::types::{ExtendedReplCommand, ReplCommand, ReplEvent, ReplInputKind};

/// Check if input is a complete statement.
///
/// Returns `true` when all brackets (`()`, `[]`, `{}`) are balanced and
/// no depths have gone negative (which would indicate a syntax error in
/// the input so far).
#[allow(missing_docs)]
pub fn is_complete(input: &str) -> bool {
    let mut paren_depth: i32 = 0;
    let mut bracket_depth: i32 = 0;
    let mut brace_depth: i32 = 0;
    for ch in input.chars() {
        match ch {
            '(' => paren_depth += 1,
            ')' => {
                paren_depth -= 1;
                if paren_depth < 0 {
                    return false;
                }
            }
            '[' => bracket_depth += 1,
            ']' => {
                bracket_depth -= 1;
                if bracket_depth < 0 {
                    return false;
                }
            }
            '{' => brace_depth += 1,
            '}' => {
                brace_depth -= 1;
                if brace_depth < 0 {
                    return false;
                }
            }
            _ => {}
        }
    }
    paren_depth == 0 && bracket_depth == 0 && brace_depth == 0
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::repl_parser::*;
    #[test]
    fn test_parse_quit() {
        let parser = ReplParser::new(":quit".to_string());
        let cmd = parser.parse().expect("parsing should succeed");
        assert_eq!(cmd, ReplCommand::Quit);
    }
    #[test]
    fn test_parse_quit_short() {
        let parser = ReplParser::new(":q".to_string());
        let cmd = parser.parse().expect("parsing should succeed");
        assert_eq!(cmd, ReplCommand::Quit);
    }
    #[test]
    fn test_parse_help() {
        let parser = ReplParser::new(":help".to_string());
        let cmd = parser.parse().expect("parsing should succeed");
        assert_eq!(cmd, ReplCommand::Help);
    }
    #[test]
    fn test_parse_show_env() {
        let parser = ReplParser::new(":env".to_string());
        let cmd = parser.parse().expect("parsing should succeed");
        assert_eq!(cmd, ReplCommand::ShowEnv);
    }
    #[test]
    fn test_parse_clear() {
        let parser = ReplParser::new(":clear".to_string());
        let cmd = parser.parse().expect("parsing should succeed");
        assert_eq!(cmd, ReplCommand::Clear);
    }
    #[test]
    fn test_parse_load() {
        let parser = ReplParser::new(":load test.lean".to_string());
        let cmd = parser.parse().expect("parsing should succeed");
        assert!(matches!(cmd, ReplCommand::Load(_)));
    }
    #[test]
    fn test_is_complete_balanced() {
        assert!(is_complete("(1 + 2)"));
        assert!(is_complete("{x : Nat}"));
        assert!(is_complete("[1, 2, 3]"));
        assert!(is_complete("simp only [h1, h2]"));
    }
    #[test]
    fn test_is_complete_unbalanced() {
        assert!(!is_complete("(1 + 2"));
        assert!(!is_complete("{x : Nat"));
        assert!(!is_complete("[1, 2, 3"));
    }
    #[test]
    fn test_is_complete_negative_depth() {
        assert!(!is_complete(")foo"));
        assert!(!is_complete("]foo"));
        assert!(!is_complete("}foo"));
    }
    #[test]
    fn test_parse_eval() {
        let parser = ReplParser::new("42".to_string());
        let cmd = parser.parse().expect("parsing should succeed");
        assert!(matches!(cmd, ReplCommand::Eval(_)));
    }
}
/// Parse an extended REPL command from a string slice.
///
/// Extended commands include `:set`, `:get`, `:history`, `:undo`,
/// `:reduce`, `:stats`, `:search`, and `:print`.
#[allow(missing_docs)]
pub fn parse_extended_command(
    cmd: &str,
    rest: &str,
) -> Result<Option<ExtendedReplCommand>, String> {
    match cmd {
        "set" => {
            let parts: Vec<&str> = rest.splitn(2, char::is_whitespace).collect();
            if parts.len() < 2 {
                return Err("Usage: :set <option> <value>".to_string());
            }
            Ok(Some(ExtendedReplCommand::SetOption(
                parts[0].trim().to_string(),
                parts[1].trim().to_string(),
            )))
        }
        "get" => {
            let name = rest.trim();
            if name.is_empty() {
                return Err("Usage: :get <option>".to_string());
            }
            Ok(Some(ExtendedReplCommand::GetOption(name.to_string())))
        }
        "history" | "hist" => Ok(Some(ExtendedReplCommand::History)),
        "undo" | "u" => Ok(Some(ExtendedReplCommand::Undo)),
        "stats" => Ok(Some(ExtendedReplCommand::Stats)),
        "search" | "s" => {
            let pattern = rest.trim();
            if pattern.is_empty() {
                return Err("Usage: :search <pattern>".to_string());
            }
            Ok(Some(ExtendedReplCommand::Search(pattern.to_string())))
        }
        "print" | "p" => {
            let name = rest.trim();
            if name.is_empty() {
                return Err("Usage: :print <name>".to_string());
            }
            Ok(Some(ExtendedReplCommand::Print(name.to_string())))
        }
        _ => Ok(None),
    }
}
/// Provide command completions for a given prefix.
///
/// Returns a list of possible completions sorted alphabetically.
#[allow(missing_docs)]
pub fn complete_command(prefix: &str) -> Vec<String> {
    let commands = [
        ":quit", ":q", ":exit", ":help", ":h", ":?", ":env", ":show", ":clear", ":reset", ":type",
        ":t", ":load", ":l", ":check", ":c", ":set", ":get", ":history", ":hist", ":undo", ":u",
        ":stats", ":search", ":s", ":print", ":p", ":reduce",
    ];
    let mut completions: Vec<String> = commands
        .iter()
        .filter(|cmd| cmd.starts_with(prefix))
        .map(|s| s.to_string())
        .collect();
    completions.sort();
    completions
}
/// Return the help text for the REPL.
#[allow(missing_docs)]
pub fn help_text() -> &'static str {
    "OxiLean REPL Commands:
  <expr>          Evaluate an expression
  theorem/def     Elaborate a declaration
  :type <expr>    Show type of expression        (:t)
  :check <decl>   Check a declaration            (:c)
  :load <file>    Load a file                    (:l)
  :env            Show environment               (:show)
  :clear          Clear environment              (:reset)
  :set <opt> <v>  Set a REPL option
  :get <opt>      Get a REPL option value
  :history        Show command history           (:hist)
  :undo           Undo last declaration          (:u)
  :stats          Show elaboration statistics
  :search <pat>   Search for declarations        (:s)
  :print <name>   Print declaration info         (:p)
  :reduce <expr>  Print normal form of expr
  :help           Show this help                 (:h, :?)
  :quit           Exit REPL                      (:q, :exit)"
}
/// Normalize input by trimming and collapsing internal whitespace.
#[allow(missing_docs)]
pub fn normalize_input(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}
/// Check if an input string is a meta-command (starts with `:`).
#[allow(missing_docs)]
pub fn is_meta_command(input: &str) -> bool {
    input.trim_start().starts_with(':')
}
/// Check if an input string is empty or only whitespace.
#[allow(missing_docs)]
pub fn is_empty_input(input: &str) -> bool {
    input.trim().is_empty()
}
/// Parse a boolean option value.
///
/// Accepts `"true"`, `"false"`, `"yes"`, `"no"`, `"1"`, `"0"`.
#[allow(missing_docs)]
pub fn parse_bool(s: &str) -> Result<bool, String> {
    match s.trim().to_lowercase().as_str() {
        "true" | "yes" | "1" | "on" => Ok(true),
        "false" | "no" | "0" | "off" => Ok(false),
        other => Err(format!("Expected boolean, got '{}'", other)),
    }
}
#[cfg(test)]
mod extended_repl_tests {
    use super::*;
    use crate::repl_parser::*;
    #[test]
    fn test_repl_options_default() {
        let opts = ReplOptions::default();
        assert!(opts.print_types);
        assert!(!opts.show_timing);
        assert_eq!(opts.max_history, 100);
    }
    #[test]
    fn test_repl_options_set_get() {
        let mut opts = ReplOptions::new();
        opts.set("timing", "true")
            .expect("test operation should succeed");
        assert_eq!(opts.get("timing"), Some("true".to_string()));
        opts.set("timing", "false")
            .expect("test operation should succeed");
        assert_eq!(opts.get("timing"), Some("false".to_string()));
    }
    #[test]
    fn test_repl_options_unknown_option() {
        let mut opts = ReplOptions::new();
        let result = opts.set("nonexistent", "value");
        assert!(result.is_err());
    }
    #[test]
    fn test_command_history_push() {
        let mut h = CommandHistory::new(10);
        h.push("foo".to_string());
        h.push("bar".to_string());
        assert_eq!(h.len(), 2);
    }
    #[test]
    fn test_command_history_no_duplicates() {
        let mut h = CommandHistory::new(10);
        h.push("foo".to_string());
        h.push("foo".to_string());
        assert_eq!(h.len(), 1);
    }
    #[test]
    fn test_command_history_search() {
        let mut h = CommandHistory::new(10);
        h.push("theorem foo : Nat".to_string());
        h.push("def bar : Type".to_string());
        h.push("theorem baz : Prop".to_string());
        let results = h.search("theorem");
        assert_eq!(results.len(), 2);
    }
    #[test]
    fn test_multiline_state_complete() {
        let mut m = MultilineState::new();
        m.push_line("(1 + 2)");
        assert!(m.is_complete());
    }
    #[test]
    fn test_multiline_state_incomplete() {
        let mut m = MultilineState::new();
        m.push_line("(1 + 2");
        assert!(!m.is_complete());
    }
    #[test]
    fn test_multiline_state_reset() {
        let mut m = MultilineState::new();
        m.push_line("(foo");
        m.reset();
        assert_eq!(m.depth(), 0);
        assert_eq!(m.line_count(), 0);
    }
    #[test]
    fn test_complete_command() {
        let completions = complete_command(":q");
        assert!(completions.contains(&":quit".to_string()));
        assert!(completions.contains(&":q".to_string()));
    }
    #[test]
    fn test_complete_command_help() {
        let completions = complete_command(":h");
        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.starts_with(":h")));
    }
    #[test]
    fn test_parse_bool_true_variants() {
        assert_eq!(parse_bool("true"), Ok(true));
        assert_eq!(parse_bool("yes"), Ok(true));
        assert_eq!(parse_bool("1"), Ok(true));
        assert_eq!(parse_bool("on"), Ok(true));
    }
    #[test]
    fn test_parse_bool_false_variants() {
        assert_eq!(parse_bool("false"), Ok(false));
        assert_eq!(parse_bool("no"), Ok(false));
        assert_eq!(parse_bool("0"), Ok(false));
        assert_eq!(parse_bool("off"), Ok(false));
    }
    #[test]
    fn test_parse_bool_invalid() {
        assert!(parse_bool("maybe").is_err());
        assert!(parse_bool("").is_err());
    }
    #[test]
    fn test_is_meta_command() {
        assert!(is_meta_command(":quit"));
        assert!(is_meta_command("  :help"));
        assert!(!is_meta_command("42"));
        assert!(!is_meta_command("def foo"));
    }
    #[test]
    fn test_is_empty_input() {
        assert!(is_empty_input(""));
        assert!(is_empty_input("   "));
        assert!(!is_empty_input("x"));
    }
    #[test]
    fn test_normalize_input() {
        let n = normalize_input("  foo   bar  baz ");
        assert_eq!(n, "foo bar baz");
    }
    #[test]
    fn test_help_text_nonempty() {
        assert!(!help_text().is_empty());
        assert!(help_text().contains(":quit"));
        assert!(help_text().contains(":help"));
    }
    #[test]
    fn test_repl_stats_success_rate() {
        let mut stats = ReplStats::new();
        stats.record_success();
        stats.record_success();
        stats.record_error();
        let rate = stats.success_rate();
        assert!((rate - 2.0 / 3.0).abs() < 1e-10);
    }
    #[test]
    fn test_repl_session_process_quit() {
        let mut session = ReplSession::new();
        let cmd = session
            .process(":quit")
            .expect("test operation should succeed");
        assert_eq!(cmd, ReplCommand::Quit);
    }
    #[test]
    fn test_repl_session_history_tracking() {
        let mut session = ReplSession::new();
        let _ = session.process(":help");
        let _ = session.process(":quit");
        assert_eq!(session.history.len(), 2);
    }
    #[test]
    fn test_parse_extended_set() {
        let result =
            parse_extended_command("set", "timing true").expect("test operation should succeed");
        assert!(matches!(result, Some(ExtendedReplCommand::SetOption(_, _))));
    }
    #[test]
    fn test_parse_extended_history() {
        let result = parse_extended_command("history", "").expect("test operation should succeed");
        assert!(matches!(result, Some(ExtendedReplCommand::History)));
    }
    #[test]
    fn test_parse_extended_search() {
        let result =
            parse_extended_command("search", "Nat").expect("test operation should succeed");
        assert!(matches!(result, Some(ExtendedReplCommand::Search(s)) if s == "Nat"));
    }
    #[test]
    fn test_parse_extended_unknown() {
        let result =
            parse_extended_command("nonexistent", "").expect("test operation should succeed");
        assert!(result.is_none());
    }
}
/// Format a REPL prompt based on state.
#[allow(missing_docs)]
pub fn format_prompt(depth: i32, session_num: u64) -> String {
    if depth > 0 {
        format!("  {}> ", "  ".repeat(depth as usize))
    } else {
        format!("oxilean[{}]> ", session_num)
    }
}
/// Syntax-highlight a short token for REPL display.
///
/// Returns the token with ANSI escape codes if `use_color` is `true`.
#[allow(missing_docs)]
pub fn highlight_keyword(token: &str, use_color: bool) -> String {
    if !use_color {
        return token.to_string();
    }
    let keywords = [
        "theorem",
        "def",
        "axiom",
        "inductive",
        "fun",
        "let",
        "in",
        "match",
        "with",
    ];
    if keywords.contains(&token) {
        format!("\x1b[1;34m{}\x1b[0m", token)
    } else {
        token.to_string()
    }
}
#[cfg(test)]
mod repl_extended_tests {
    use super::*;
    use crate::repl_parser::*;
    #[test]
    fn test_repl_option_from_name() {
        assert_eq!(ReplOption::from_name("timing"), Some(ReplOption::Timing));
        assert_eq!(ReplOption::from_name("verbose"), Some(ReplOption::Verbose));
        assert_eq!(ReplOption::from_name("unknown"), None);
    }
    #[test]
    fn test_repl_option_name() {
        assert_eq!(ReplOption::Timing.name(), "timing");
        assert_eq!(ReplOption::PrettyPrint.name(), "pretty_print");
    }
    #[test]
    fn test_option_store_set_get() {
        let mut store = OptionStore::new();
        store.set("timing", "true");
        assert_eq!(store.get("timing"), Some("true"));
        assert!(store.has("timing"));
    }
    #[test]
    fn test_option_store_get_bool() {
        let mut store = OptionStore::new();
        store.set("verbose", "yes");
        assert!(store.get_bool("verbose"));
        assert!(!store.get_bool("timing"));
    }
    #[test]
    fn test_option_store_get_u64() {
        let mut store = OptionStore::new();
        store.set("max_lines", "50");
        assert_eq!(store.get_u64("max_lines", 100), 50);
        assert_eq!(store.get_u64("other", 42), 42);
    }
    #[test]
    fn test_option_store_remove() {
        let mut store = OptionStore::new();
        store.set("k", "v");
        assert!(store.remove("k"));
        assert!(!store.has("k"));
    }
    #[test]
    fn test_option_store_len() {
        let mut store = OptionStore::new();
        store.set("a", "1");
        store.set("b", "2");
        assert_eq!(store.len(), 2);
    }
    #[test]
    fn test_input_splitter_complete() {
        let mut s = InputSplitter::new();
        s.push("(1 + 2)");
        assert!(s.is_complete());
    }
    #[test]
    fn test_input_splitter_incomplete() {
        let mut s = InputSplitter::new();
        s.push("(1 + 2");
        assert!(!s.is_complete());
    }
    #[test]
    fn test_input_splitter_flush() {
        let mut s = InputSplitter::new();
        s.push("hello");
        let out = s.flush();
        assert_eq!(out, "hello");
        assert!(s.is_empty());
    }
    #[test]
    fn test_input_splitter_multiline() {
        let mut s = InputSplitter::new();
        s.push("fun x =>");
        s.push("  x + 1");
        assert_eq!(s.line_count(), 2);
        let out = s.flush();
        assert!(out.contains("fun x =>"));
        assert!(out.contains("x + 1"));
    }
    #[test]
    fn test_format_prompt_toplevel() {
        let p = format_prompt(0, 1);
        assert!(p.contains("oxilean[1]"));
    }
    #[test]
    fn test_format_prompt_nested() {
        let p = format_prompt(2, 5);
        assert!(p.contains(">"));
        assert!(!p.contains("oxilean"));
    }
    #[test]
    fn test_highlight_keyword_no_color() {
        let s = highlight_keyword("theorem", false);
        assert_eq!(s, "theorem");
    }
    #[test]
    fn test_highlight_keyword_with_color() {
        let s = highlight_keyword("theorem", true);
        assert!(s.contains("theorem"));
        assert!(s.contains("\x1b["));
    }
    #[test]
    fn test_highlight_non_keyword() {
        let s = highlight_keyword("myFunc", true);
        assert_eq!(s, "myFunc");
    }
}
/// A REPL event listener trait.
#[allow(dead_code)]
#[allow(missing_docs)]
pub trait ReplEventListener {
    /// Called when a REPL event occurs.
    fn on_event(&mut self, event: &ReplEvent);
}
/// A REPL filter that preprocesses input before parsing.
#[allow(dead_code)]
#[allow(missing_docs)]
pub trait InputFilter {
    /// Filter/transform the input string.
    fn filter(&self, input: &str) -> String;
}
/// Count words in an input string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn word_count(input: &str) -> usize {
    input.split_whitespace().count()
}
/// Check if a line starts a tactic block (starts with `by`).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_tactic_block_start(input: &str) -> bool {
    let trimmed = input.trim();
    trimmed == "by" || trimmed.starts_with("by ") || trimmed.starts_with("by\t")
}
/// Extract command name and rest from a meta-command string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn split_meta_command(cmd: &str) -> (&str, &str) {
    let parts: Vec<&str> = cmd.splitn(2, char::is_whitespace).collect();
    let name = parts.first().copied().unwrap_or("");
    let rest = parts.get(1).copied().unwrap_or("").trim_start();
    (name, rest)
}
/// Determine if a REPL command is "safe" (non-destructive).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_safe_command(cmd: &ReplCommand) -> bool {
    matches!(
        cmd,
        ReplCommand::Eval(_) | ReplCommand::Type(_) | ReplCommand::Help | ReplCommand::ShowEnv
    )
}
/// Produce a one-line summary of a REPL command.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn command_summary(cmd: &ReplCommand) -> &'static str {
    match cmd {
        ReplCommand::Eval(_) => "evaluate expression",
        ReplCommand::Type(_) => "show type",
        ReplCommand::Check(_) => "check declaration",
        ReplCommand::Load(_) => "load file",
        ReplCommand::ShowEnv => "show environment",
        ReplCommand::Clear => "clear environment",
        ReplCommand::Help => "show help",
        ReplCommand::Quit => "quit REPL",
    }
}
/// Extended option names.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn all_option_names() -> Vec<&'static str> {
    vec![
        "timing",
        "types",
        "color",
        "verbose",
        "history",
        "max_lines",
        "pretty_print",
    ]
}
/// Suggest an option name for a misspelled option (Levenshtein heuristic).
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn suggest_option(input: &str) -> Option<&'static str> {
    let names = all_option_names();
    names
        .into_iter()
        .min_by_key(|name| simple_distance(input, name))
        .filter(|name| simple_distance(input, name) <= 3)
}
/// A simple character-level distance function.
#[allow(dead_code)]
pub(super) fn simple_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let m = a.len();
    let n = b.len();
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for (i, row) in dp.iter_mut().enumerate().take(m + 1) {
        row[0] = i;
    }
    for (j, cell) in dp[0].iter_mut().enumerate().take(n + 1) {
        *cell = j;
    }
    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
    }
    dp[m][n]
}
#[cfg(test)]
mod repl_extra_tests {
    use super::*;
    use crate::repl_parser::*;
    #[test]
    fn test_repl_mode_name() {
        assert_eq!(ReplMode::Normal.name(), "normal");
        assert_eq!(ReplMode::Tactic.name(), "tactic");
    }
    #[test]
    fn test_repl_mode_from_name() {
        assert_eq!(ReplMode::from_name("tactic"), Some(ReplMode::Tactic));
        assert_eq!(ReplMode::from_name("unknown"), None);
    }
    #[test]
    fn test_repl_mode_display() {
        assert_eq!(format!("{}", ReplMode::Search), "search");
    }
    #[test]
    fn test_repl_event_display() {
        let e = ReplEvent::Parsed("foo".to_string());
        let s = format!("{}", e);
        assert!(s.contains("foo"));
    }
    #[test]
    fn test_event_log_listener() {
        let mut log = EventLog::new();
        log.on_event(&ReplEvent::Reset);
        log.on_event(&ReplEvent::Exit);
        assert_eq!(log.len(), 2);
    }
    #[test]
    fn test_event_log_clear() {
        let mut log = EventLog::new();
        log.on_event(&ReplEvent::Reset);
        log.clear();
        assert!(log.is_empty());
    }
    #[test]
    fn test_command_alias_new() {
        let a = CommandAlias::new("q", ":quit");
        assert_eq!(a.from, "q");
        assert_eq!(a.to, ":quit");
    }
    #[test]
    fn test_alias_registry_expand() {
        let mut reg = AliasRegistry::new();
        reg.register("quit", ":quit");
        assert_eq!(reg.expand("quit"), ":quit");
        assert_eq!(reg.expand("help"), "help");
    }
    #[test]
    fn test_alias_registry_names() {
        let mut reg = AliasRegistry::new();
        reg.register("a", "b");
        reg.register("c", "d");
        let names = reg.names();
        assert!(names.contains(&"a"));
        assert!(names.contains(&"c"));
    }
    #[test]
    fn test_strip_semicolon_filter() {
        let f = StripSemicolonFilter;
        assert_eq!(f.filter("foo;"), "foo");
        assert_eq!(f.filter("foo"), "foo");
    }
    #[test]
    fn test_lowercase_command_filter() {
        let f = LowercaseCommandFilter;
        let out = f.filter(":QUIT");
        assert!(out.starts_with(":quit"));
    }
    #[test]
    fn test_filter_pipeline_apply() {
        let mut pipeline = FilterPipeline::new();
        pipeline.add(StripSemicolonFilter);
        let result = pipeline.apply("foo;");
        assert_eq!(result, "foo");
    }
    #[test]
    fn test_filter_pipeline_len() {
        let mut pipeline = FilterPipeline::new();
        assert_eq!(pipeline.len(), 0);
        pipeline.add(StripSemicolonFilter);
        assert_eq!(pipeline.len(), 1);
    }
    #[test]
    fn test_repl_completer_complete() {
        let completer = ReplCompleter::new();
        let results = completer.complete(":q");
        assert!(!results.is_empty());
        assert!(results.iter().any(|c| c.text == ":quit"));
    }
    #[test]
    fn test_repl_completer_add() {
        let mut completer = ReplCompleter::new();
        let initial = completer.len();
        completer.add(CompletionItem::new(
            "myCmd",
            "custom",
            CompletionKind::Command,
        ));
        assert_eq!(completer.len(), initial + 1);
    }
    #[test]
    fn test_repl_formatter_success() {
        let fmt = ReplFormatter::new(false, 80);
        let s = fmt.success("all good");
        assert!(s.contains("all good"));
    }
    #[test]
    fn test_repl_formatter_error() {
        let fmt = ReplFormatter::new(false, 80);
        let s = fmt.error("bad input");
        assert!(s.contains("bad input"));
    }
    #[test]
    fn test_repl_formatter_truncate() {
        let fmt = ReplFormatter::new(false, 5);
        let s = fmt.truncate("hello world");
        assert!(s.len() <= 6);
    }
    #[test]
    fn test_repl_formatter_list() {
        let fmt = ReplFormatter::new(false, 80);
        let items = vec!["foo".to_string(), "bar".to_string()];
        let s = fmt.list(&items);
        assert!(s.contains("foo"));
        assert!(s.contains("bar"));
    }
    #[test]
    fn test_configurable_repl_parser_normalize() {
        let config = ReplParserConfig::new();
        let p = ConfigurableReplParser::new(config);
        let out = p.preprocess("  foo   bar  ");
        assert_eq!(out, "  foo   bar  ");
    }
    #[test]
    fn test_configurable_repl_parser_with_normalize() {
        let config = ReplParserConfig {
            normalize_whitespace: true,
            ..Default::default()
        };
        let p = ConfigurableReplParser::new(config);
        let out = p.preprocess("  foo   bar  ");
        assert_eq!(out, "foo bar");
    }
    #[test]
    fn test_configurable_repl_parser_strip_comments() {
        let config = ReplParserConfig {
            strip_comments: true,
            ..Default::default()
        };
        let p = ConfigurableReplParser::new(config);
        let out = p.preprocess("def foo -- this is a comment");
        assert!(!out.contains("comment"));
    }
    #[test]
    fn test_command_tally_record() {
        let mut tally = CommandTally::new();
        tally.record(&ReplCommand::Help);
        tally.record(&ReplCommand::Quit);
        assert_eq!(tally.helps, 1);
        assert_eq!(tally.quits, 1);
        assert_eq!(tally.total(), 2);
    }
    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world foo"), 3);
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("  one  "), 1);
    }
    #[test]
    fn test_is_tactic_block_start() {
        assert!(is_tactic_block_start("by"));
        assert!(is_tactic_block_start("  by intro h"));
        assert!(!is_tactic_block_start("theorem foo"));
    }
    #[test]
    fn test_split_meta_command() {
        let (cmd, rest) = split_meta_command("set timing true");
        assert_eq!(cmd, "set");
        assert_eq!(rest, "timing true");
    }
    #[test]
    fn test_split_meta_command_no_rest() {
        let (cmd, rest) = split_meta_command("history");
        assert_eq!(cmd, "history");
        assert_eq!(rest, "");
    }
    #[test]
    fn test_is_safe_command() {
        assert!(is_safe_command(&ReplCommand::Help));
        assert!(is_safe_command(&ReplCommand::ShowEnv));
        assert!(!is_safe_command(&ReplCommand::Clear));
        assert!(!is_safe_command(&ReplCommand::Quit));
    }
    #[test]
    fn test_command_summary() {
        assert_eq!(command_summary(&ReplCommand::Help), "show help");
        assert_eq!(command_summary(&ReplCommand::Quit), "quit REPL");
    }
    #[test]
    fn test_suggest_option_close() {
        let s = suggest_option("timng");
        assert!(s.is_some());
        assert_eq!(s.expect("test operation should succeed"), "timing");
    }
    #[test]
    fn test_suggest_option_no_match() {
        let s = suggest_option("zzzzzzz");
        let _ = s;
    }
    #[test]
    fn test_all_option_names_nonempty() {
        let names = all_option_names();
        assert!(!names.is_empty());
        assert!(names.contains(&"timing"));
        assert!(names.contains(&"verbose"));
    }
    #[test]
    fn test_repl_session_default() {
        let session = ReplSession::default();
        assert!(session.history.is_empty());
        assert_eq!(session.stats.commands_run, 0);
    }
    #[test]
    fn test_option_store_keys() {
        let mut store = OptionStore::new();
        store.set("a", "1");
        store.set("b", "2");
        let keys = store.keys();
        assert_eq!(keys.len(), 2);
    }
    #[test]
    fn test_command_history_prev() {
        let mut h = CommandHistory::new(10);
        h.push("first".to_string());
        h.push("second".to_string());
        let prev = h.prev();
        assert_eq!(prev, Some("second"));
    }
    #[test]
    fn test_command_history_max() {
        let mut h = CommandHistory::new(2);
        h.push("a".to_string());
        h.push("b".to_string());
        h.push("c".to_string());
        assert_eq!(h.len(), 2);
        assert_eq!(h.entries()[0], "b");
    }
    #[test]
    fn test_simple_distance_equal() {
        assert_eq!(simple_distance("hello", "hello"), 0);
    }
    #[test]
    fn test_simple_distance_insert() {
        assert_eq!(simple_distance("", "a"), 1);
    }
    #[test]
    fn test_simple_distance_substitute() {
        assert_eq!(simple_distance("cat", "bat"), 1);
    }
}
#[cfg(test)]
mod repl_history_tests {
    use super::*;
    use crate::repl_parser::*;
    #[test]
    fn test_repl_history() {
        let mut h = ReplHistory::new();
        h.push("#check Nat", true);
        h.push("bad input!", false);
        assert_eq!(h.len(), 2);
        assert!(h.entries[0].parse_ok);
        assert!(!h.entries[1].parse_ok);
    }
}
/// Classify a REPL input string.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn classify_repl_input(input: &str) -> ReplInputKind {
    let trimmed = input.trim();
    if trimmed.starts_with("def ")
        || trimmed.starts_with("theorem ")
        || trimmed.starts_with("lemma ")
    {
        ReplInputKind::Definition
    } else if trimmed.starts_with('#') {
        ReplInputKind::Command
    } else if trimmed.starts_with("by ") || trimmed.ends_with(":= by") {
        ReplInputKind::Tactic
    } else if trimmed.starts_with("fun ")
        || trimmed.starts_with("let ")
        || trimmed.starts_with("match ")
    {
        ReplInputKind::Term
    } else if trimmed.ends_with('(') || trimmed.ends_with(',') || trimmed.ends_with("->") {
        ReplInputKind::Incomplete
    } else {
        ReplInputKind::Term
    }
}
#[cfg(test)]
mod repl_classify_tests {
    use super::*;
    use crate::repl_parser::*;
    #[test]
    fn test_classify_repl_input() {
        assert_eq!(
            classify_repl_input("def foo := 1"),
            ReplInputKind::Definition
        );
        assert_eq!(classify_repl_input("#check Nat"), ReplInputKind::Command);
        assert_eq!(classify_repl_input("1 + 2"), ReplInputKind::Term);
        assert_eq!(classify_repl_input("fun x ->"), ReplInputKind::Term);
    }
}
/// Returns true if a REPL input looks like an incomplete expression.
#[allow(dead_code)]
#[allow(missing_docs)]
pub fn is_incomplete_repl_input(s: &str) -> bool {
    let t = s.trim();
    t.is_empty() || t.ends_with('(') || t.ends_with(',') || t.ends_with("->") || t.ends_with(':')
}
#[cfg(test)]
mod repl_pad {
    use super::*;
    use crate::repl_parser::*;
    #[test]
    fn test_is_incomplete_repl_input() {
        assert!(is_incomplete_repl_input("fun x ->"));
        assert!(!is_incomplete_repl_input("fun x -> x"));
        assert!(is_incomplete_repl_input(""));
    }
}
