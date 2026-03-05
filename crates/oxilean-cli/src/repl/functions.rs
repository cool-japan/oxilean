//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{print_expr, Declaration, Environment, Name, Reducer};
use std::collections::{HashMap, VecDeque};

use super::types::{
    Completer, Completion, ErrorRecovery, History, InputBuffer, Repl, ReplCmd, ReplMode,
    ReplOptions, ReplState, SessionMetadata, SessionSnapshot, SyntaxHighlighter, UndoEntry,
};

/// Parse a boolean value from a string.
pub fn parse_bool(s: &str) -> Result<bool, String> {
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => Ok(true),
        "false" | "0" | "no" | "off" => Ok(false),
        _ => Err(format!("invalid boolean value: {}", s)),
    }
}
/// Parse a colon-command string into a ReplCmd.
#[allow(dead_code)]
pub fn parse_command(input: &str) -> ReplCmd {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return ReplCmd::Unknown(String::new());
    }
    match parts[0] {
        ":quit" | ":q" => ReplCmd::Quit,
        ":help" | ":h" => ReplCmd::Help,
        ":env" => ReplCmd::ShowEnv,
        ":clear" => ReplCmd::Clear,
        ":type" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":type requires an expression".to_string())
            } else {
                ReplCmd::Type(parts[1..].join(" "))
            }
        }
        ":check" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":check requires an expression".to_string())
            } else {
                ReplCmd::Check(parts[1..].join(" "))
            }
        }
        ":load" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":load requires a file path".to_string())
            } else {
                ReplCmd::Load(parts[1..].join(" "))
            }
        }
        ":reload" | ":r" => ReplCmd::Reload,
        ":print" | ":p" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":print requires a name".to_string())
            } else {
                ReplCmd::Print(parts[1..].join(" "))
            }
        }
        ":search" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":search requires a pattern".to_string())
            } else {
                ReplCmd::Search(parts[1..].join(" "))
            }
        }
        ":info" | ":i" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":info requires a name".to_string())
            } else {
                ReplCmd::Info(parts[1..].join(" "))
            }
        }
        ":set" => {
            if parts.len() < 3 {
                ReplCmd::Unknown(":set requires <option> <value>".to_string())
            } else {
                ReplCmd::Set(parts[1].to_string(), parts[2..].join(" "))
            }
        }
        ":unset" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":unset requires an option name".to_string())
            } else {
                ReplCmd::Unset(parts[1].to_string())
            }
        }
        ":time" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":time requires an expression".to_string())
            } else {
                ReplCmd::Time(parts[1..].join(" "))
            }
        }
        ":trace" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":trace requires an expression".to_string())
            } else {
                ReplCmd::Trace(parts[1..].join(" "))
            }
        }
        ":undo" => ReplCmd::Undo,
        ":save" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":save requires a file path".to_string())
            } else {
                ReplCmd::Save(parts[1..].join(" "))
            }
        }
        ":axioms" => ReplCmd::Axioms,
        ":history" => {
            if parts.len() > 1 {
                ReplCmd::HistorySearch(Some(parts[1..].join(" ")))
            } else {
                ReplCmd::ShowHistory
            }
        }
        ":browse" | ":b" => {
            if parts.len() > 1 {
                ReplCmd::Browse(Some(parts[1].to_string()))
            } else {
                ReplCmd::Browse(None)
            }
        }
        ":load-session" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":load-session requires a file path".to_string())
            } else {
                ReplCmd::LoadSession(parts[1..].join(" "))
            }
        }
        ":proof" | ":goals" => ReplCmd::ShowProofState,
        ":tactics" | ":tactic-list" => ReplCmd::ListTactics,
        ":eval" | ":e" => {
            if parts.len() < 2 {
                ReplCmd::Unknown(":eval requires an expression".to_string())
            } else {
                ReplCmd::Eval(parts[1..].join(" "))
            }
        }
        _ => {
            let cmd = parts[0];
            let known = [
                ":quit",
                ":help",
                ":env",
                ":clear",
                ":type",
                ":check",
                ":load",
                ":reload",
                ":print",
                ":search",
                ":info",
                ":browse",
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
                ":tactics",
                ":eval",
            ];
            let suggestion = known.iter().find(|k| {
                let dist = levenshtein_distance(cmd, k);
                dist <= 2
            });
            if let Some(s) = suggestion {
                ReplCmd::Unknown(format!("Unknown command: {}. Did you mean {}?", cmd, s))
            } else {
                ReplCmd::Unknown(cmd.to_string())
            }
        }
    }
}
/// Compute the Levenshtein edit distance between two strings.
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let m = a.len();
    let n = b.len();
    let mut prev: Vec<usize> = (0..=n).collect();
    let mut curr: Vec<usize> = vec![0; n + 1];
    for i in 1..=m {
        curr[0] = i;
        for j in 1..=n {
            curr[j] = if a[i - 1] == b[j - 1] {
                prev[j - 1]
            } else {
                1 + prev[j].min(curr[j - 1]).min(prev[j - 1])
            };
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[n]
}
#[cfg(test)]
mod tests {
    use super::*;
    use oxilean_kernel::{Expr, Level};
    #[test]
    fn test_input_buffer_new() {
        let buf = InputBuffer::new();
        assert!(buf.is_empty());
        assert!(!buf.is_continuation());
        assert_eq!(buf.line_count(), 0);
    }
    #[test]
    fn test_input_buffer_complete_simple() {
        let mut buf = InputBuffer::new();
        buf.add_line("42");
        assert!(buf.is_complete());
        assert_eq!(buf.get_complete_input(), "42");
    }
    #[test]
    fn test_input_buffer_open_paren() {
        let mut buf = InputBuffer::new();
        buf.add_line("(1 + 2");
        assert!(!buf.is_complete());
        assert!(buf.is_continuation());
        buf.add_line(")");
        assert!(buf.is_complete());
    }
    #[test]
    fn test_input_buffer_open_brace() {
        let mut buf = InputBuffer::new();
        buf.add_line("{ x : Nat");
        assert!(!buf.is_complete());
        buf.add_line("}");
        assert!(buf.is_complete());
    }
    #[test]
    fn test_input_buffer_open_bracket() {
        let mut buf = InputBuffer::new();
        buf.add_line("[1, 2, 3");
        assert!(!buf.is_complete());
        buf.add_line("]");
        assert!(buf.is_complete());
    }
    #[test]
    fn test_input_buffer_unclosed_string() {
        let mut buf = InputBuffer::new();
        buf.add_line("\"hello");
        assert!(!buf.is_complete());
    }
    #[test]
    fn test_input_buffer_by_continuation() {
        let mut buf = InputBuffer::new();
        buf.add_line("theorem t : True := by");
        assert!(!buf.is_complete());
    }
    #[test]
    fn test_input_buffer_where_continuation() {
        let mut buf = InputBuffer::new();
        buf.add_line("def f := 1 where");
        assert!(!buf.is_complete());
    }
    #[test]
    fn test_input_buffer_trailing_backslash() {
        let mut buf = InputBuffer::new();
        buf.add_line("let x := \\");
        assert!(!buf.is_complete());
    }
    #[test]
    fn test_input_buffer_begin_end() {
        let mut buf = InputBuffer::new();
        buf.add_line("begin");
        assert!(!buf.is_complete());
        buf.add_line("  sorry");
        assert!(!buf.is_complete());
        buf.add_line("end");
        assert!(buf.is_complete());
    }
    #[test]
    fn test_input_buffer_reset() {
        let mut buf = InputBuffer::new();
        buf.add_line("(1 + 2");
        assert!(buf.is_continuation());
        buf.reset();
        assert!(buf.is_empty());
        assert!(!buf.is_continuation());
    }
    #[test]
    fn test_input_buffer_multiline_join() {
        let mut buf = InputBuffer::new();
        buf.add_line("let x := 1");
        buf.reset();
        buf.add_line("line1");
        buf.add_line("line2");
        assert_eq!(buf.get_complete_input(), "line1\nline2");
    }
    #[test]
    fn test_input_buffer_empty_is_complete() {
        let buf = InputBuffer::new();
        assert!(buf.is_complete());
    }
    #[test]
    fn test_input_buffer_comment_ignored() {
        let mut buf = InputBuffer::new();
        buf.add_line("42 -- this is a comment");
        assert!(buf.is_complete());
    }
    #[test]
    fn test_history_new() {
        let h = History::new(100);
        assert!(h.is_empty());
        assert_eq!(h.len(), 0);
    }
    #[test]
    fn test_history_add_entry() {
        let mut h = History::new(100);
        h.add_entry("hello".to_string());
        assert_eq!(h.len(), 1);
        assert!(!h.is_empty());
    }
    #[test]
    fn test_history_dedup_consecutive() {
        let mut h = History::new(100);
        h.add_entry("hello".to_string());
        h.add_entry("hello".to_string());
        assert_eq!(h.len(), 1);
    }
    #[test]
    fn test_history_no_dedup_nonconsecutive() {
        let mut h = History::new(100);
        h.add_entry("a".to_string());
        h.add_entry("b".to_string());
        h.add_entry("a".to_string());
        assert_eq!(h.len(), 3);
    }
    #[test]
    fn test_history_max_size() {
        let mut h = History::new(3);
        h.add_entry("a".to_string());
        h.add_entry("b".to_string());
        h.add_entry("c".to_string());
        h.add_entry("d".to_string());
        assert_eq!(h.len(), 3);
        assert_eq!(h.entries()[0].text, "b");
    }
    #[test]
    fn test_history_previous() {
        let mut h = History::new(100);
        h.add_entry("a".to_string());
        h.add_entry("b".to_string());
        h.add_entry("c".to_string());
        assert_eq!(h.previous(), Some("c"));
        assert_eq!(h.previous(), Some("b"));
        assert_eq!(h.previous(), Some("a"));
        assert_eq!(h.previous(), Some("a"));
    }
    #[test]
    fn test_history_next() {
        let mut h = History::new(100);
        h.add_entry("a".to_string());
        h.add_entry("b".to_string());
        assert_eq!(h.previous(), Some("b"));
        assert_eq!(h.previous(), Some("a"));
        assert_eq!(h.next(), Some("b"));
        assert_eq!(h.next(), None);
    }
    #[test]
    fn test_history_search() {
        let mut h = History::new(100);
        h.add_entry("def foo := 1".to_string());
        h.add_entry("theorem bar : Prop := sorry".to_string());
        h.add_entry("def baz := 2".to_string());
        let results = h.search("def");
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "def baz := 2");
    }
    #[test]
    fn test_history_search_case_insensitive() {
        let mut h = History::new(100);
        h.add_entry("DEF foo := 1".to_string());
        let results = h.search("def");
        assert_eq!(results.len(), 1);
    }
    #[test]
    fn test_history_clear() {
        let mut h = History::new(100);
        h.add_entry("a".to_string());
        h.add_entry("b".to_string());
        h.clear();
        assert!(h.is_empty());
    }
    #[test]
    fn test_history_empty_not_added() {
        let mut h = History::new(100);
        h.add_entry("".to_string());
        h.add_entry("   ".to_string());
        assert_eq!(h.len(), 0);
    }
    #[test]
    fn test_history_previous_empty() {
        let mut h = History::new(100);
        assert_eq!(h.previous(), None);
    }
    #[test]
    fn test_history_next_no_nav() {
        let mut h = History::new(100);
        h.add_entry("a".to_string());
        assert_eq!(h.next(), None);
    }
    #[test]
    fn test_history_reset_position() {
        let mut h = History::new(100);
        h.add_entry("a".to_string());
        h.add_entry("b".to_string());
        h.previous();
        h.reset_position();
        assert_eq!(h.previous(), Some("b"));
    }
    #[test]
    fn test_completer_new() {
        let c = Completer::new();
        assert!(!c.keywords.is_empty());
        assert!(!c.tactics.is_empty());
        assert!(!c.commands.is_empty());
    }
    #[test]
    fn test_complete_keyword() {
        let c = Completer::new();
        let results = c.complete_keyword("the");
        assert!(results.iter().any(|r| r.text == "theorem"));
        assert!(results.iter().any(|r| r.text == "then"));
    }
    #[test]
    fn test_complete_keyword_no_match() {
        let c = Completer::new();
        let results = c.complete_keyword("zzz");
        assert!(results.is_empty());
    }
    #[test]
    fn test_complete_tactic() {
        let c = Completer::new();
        let results = c.complete_tactic("in");
        assert!(results.iter().any(|r| r.text == "intro"));
        assert!(results.iter().any(|r| r.text == "intros"));
        assert!(results.iter().any(|r| r.text == "induction"));
    }
    #[test]
    fn test_complete_command() {
        let c = Completer::new();
        let results = c.complete_command(":q");
        assert!(results.iter().any(|r| r.text == ":quit"));
        assert!(results.iter().any(|r| r.text == ":q"));
    }
    #[test]
    fn test_complete_name_from_env() {
        let c = Completer::new();
        let mut env = Environment::new();
        let decl = Declaration::Axiom {
            name: Name::str("Nat.zero"),
            univ_params: vec![],
            ty: Expr::Sort(Level::zero()),
        };
        env.add(decl).expect("test operation should succeed");
        let results = c.complete_name("Nat", &env);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].text, "Nat.zero");
    }
    #[test]
    fn test_complete_combined() {
        let c = Completer::new();
        let env = Environment::new();
        let results = c.complete("de", &env);
        assert!(results.iter().any(|r| r.text == "def"));
    }
    #[test]
    fn test_complete_command_prefix() {
        let c = Completer::new();
        let env = Environment::new();
        let results = c.complete(":he", &env);
        assert!(results.iter().any(|r| r.text == ":help"));
    }
    #[test]
    fn test_common_prefix() {
        let completions = vec![
            Completion {
                text: "intro".to_string(),
                description: String::new(),
            },
            Completion {
                text: "intros".to_string(),
                description: String::new(),
            },
            Completion {
                text: "induction".to_string(),
                description: String::new(),
            },
        ];
        assert_eq!(Completer::get_common_prefix(&completions), "in");
    }
    #[test]
    fn test_common_prefix_empty() {
        let completions: Vec<Completion> = vec![];
        assert_eq!(Completer::get_common_prefix(&completions), "");
    }
    #[test]
    fn test_common_prefix_single() {
        let completions = vec![Completion {
            text: "exact".to_string(),
            description: String::new(),
        }];
        assert_eq!(Completer::get_common_prefix(&completions), "exact");
    }
    #[test]
    fn test_parse_command_quit() {
        assert!(matches!(parse_command(":quit"), ReplCmd::Quit));
        assert!(matches!(parse_command(":q"), ReplCmd::Quit));
    }
    #[test]
    fn test_parse_command_help() {
        assert!(matches!(parse_command(":help"), ReplCmd::Help));
        assert!(matches!(parse_command(":h"), ReplCmd::Help));
    }
    #[test]
    fn test_parse_command_env() {
        assert!(matches!(parse_command(":env"), ReplCmd::ShowEnv));
    }
    #[test]
    fn test_parse_command_clear() {
        assert!(matches!(parse_command(":clear"), ReplCmd::Clear));
    }
    #[test]
    fn test_parse_command_type() {
        if let ReplCmd::Type(expr) = parse_command(":type Nat") {
            assert_eq!(expr, "Nat");
        } else {
            panic!("expected Type command");
        }
    }
    #[test]
    fn test_parse_command_type_multi_word() {
        if let ReplCmd::Type(expr) = parse_command(":type Nat -> Nat") {
            assert_eq!(expr, "Nat -> Nat");
        } else {
            panic!("expected Type command");
        }
    }
    #[test]
    fn test_parse_command_check() {
        if let ReplCmd::Check(expr) = parse_command(":check 42") {
            assert_eq!(expr, "42");
        } else {
            panic!("expected Check command");
        }
    }
    #[test]
    fn test_parse_command_load() {
        if let ReplCmd::Load(path) = parse_command(":load test.lean") {
            assert_eq!(path, "test.lean");
        } else {
            panic!("expected Load command");
        }
    }
    #[test]
    fn test_parse_command_reload() {
        assert!(matches!(parse_command(":reload"), ReplCmd::Reload));
        assert!(matches!(parse_command(":r"), ReplCmd::Reload));
    }
    #[test]
    fn test_parse_command_print() {
        if let ReplCmd::Print(name) = parse_command(":print Nat.add") {
            assert_eq!(name, "Nat.add");
        } else {
            panic!("expected Print command");
        }
    }
    #[test]
    fn test_parse_command_search() {
        if let ReplCmd::Search(pattern) = parse_command(":search Nat") {
            assert_eq!(pattern, "Nat");
        } else {
            panic!("expected Search command");
        }
    }
    #[test]
    fn test_parse_command_info() {
        if let ReplCmd::Info(name) = parse_command(":info Nat.add") {
            assert_eq!(name, "Nat.add");
        } else {
            panic!("expected Info command");
        }
    }
    #[test]
    fn test_parse_command_set() {
        if let ReplCmd::Set(key, value) = parse_command(":set pp.all true") {
            assert_eq!(key, "pp.all");
            assert_eq!(value, "true");
        } else {
            panic!("expected Set command");
        }
    }
    #[test]
    fn test_parse_command_unset() {
        if let ReplCmd::Unset(key) = parse_command(":unset pp.all") {
            assert_eq!(key, "pp.all");
        } else {
            panic!("expected Unset command");
        }
    }
    #[test]
    fn test_parse_command_time() {
        if let ReplCmd::Time(expr) = parse_command(":time 1 + 2") {
            assert_eq!(expr, "1 + 2");
        } else {
            panic!("expected Time command");
        }
    }
    #[test]
    fn test_parse_command_trace() {
        if let ReplCmd::Trace(expr) = parse_command(":trace 42") {
            assert_eq!(expr, "42");
        } else {
            panic!("expected Trace command");
        }
    }
    #[test]
    fn test_parse_command_undo() {
        assert!(matches!(parse_command(":undo"), ReplCmd::Undo));
    }
    #[test]
    fn test_parse_command_save() {
        if let ReplCmd::Save(path) = parse_command(":save output.lean") {
            assert_eq!(path, "output.lean");
        } else {
            panic!("expected Save command");
        }
    }
    #[test]
    fn test_parse_command_axioms() {
        assert!(matches!(parse_command(":axioms"), ReplCmd::Axioms));
    }
    #[test]
    fn test_parse_command_unknown() {
        assert!(matches!(parse_command(":foobar"), ReplCmd::Unknown(_)));
    }
    #[test]
    fn test_parse_command_missing_args() {
        if let ReplCmd::Unknown(msg) = parse_command(":type") {
            assert!(msg.contains("requires"));
        } else {
            panic!("expected Unknown for missing args");
        }
    }
    #[test]
    fn test_repl_state_new() {
        let state = ReplState::new();
        assert!(state.env.is_empty());
        assert_eq!(state.mode, ReplMode::Normal);
        assert!(state.undo_stack.is_empty());
    }
    #[test]
    fn test_repl_state_push_pop_undo() {
        let mut state = ReplState::new();
        state
            .env
            .add(Declaration::Axiom {
                name: Name::str("foo"),
                univ_params: vec![],
                ty: Expr::Sort(Level::zero()),
            })
            .expect("test operation should succeed");
        state.push_undo(Name::str("foo"), "added axiom foo".to_string());
        state
            .env
            .add(Declaration::Axiom {
                name: Name::str("bar"),
                univ_params: vec![],
                ty: Expr::Sort(Level::zero()),
            })
            .expect("test operation should succeed");
        assert_eq!(state.env.len(), 2);
        let entry = state.pop_undo().expect("test operation should succeed");
        assert_eq!(entry.name, Name::str("foo"));
        assert_eq!(state.env.len(), 1);
    }
    #[test]
    fn test_repl_state_pop_undo_empty() {
        let mut state = ReplState::new();
        assert!(state.pop_undo().is_none());
    }
    #[test]
    fn test_repl_state_reset() {
        let mut state = ReplState::new();
        state
            .env
            .add(Declaration::Axiom {
                name: Name::str("foo"),
                univ_params: vec![],
                ty: Expr::Sort(Level::zero()),
            })
            .expect("test operation should succeed");
        state.mode = ReplMode::Proof;
        state.used_sorry = true;
        state.reset_state();
        assert!(state.env.is_empty());
        assert_eq!(state.mode, ReplMode::Normal);
        assert!(!state.used_sorry);
    }
    #[test]
    fn test_repl_state_set_option() {
        let mut state = ReplState::new();
        assert!(state.set_option("pp.all", "true").is_ok());
        assert!(state.options.pp_all);
        assert!(state.options.pp_implicit);
        assert!(state.options.pp_universes);
    }
    #[test]
    fn test_repl_state_set_option_width() {
        let mut state = ReplState::new();
        assert!(state.set_option("pp.width", "80").is_ok());
        assert_eq!(state.options.pp_width, 80);
    }
    #[test]
    fn test_repl_state_set_option_invalid() {
        let mut state = ReplState::new();
        assert!(state.set_option("pp.width", "abc").is_err());
    }
    #[test]
    fn test_repl_state_set_custom_option() {
        let mut state = ReplState::new();
        assert!(state.set_option("my.custom", "hello").is_ok());
        assert_eq!(state.get_option("my.custom"), Some("hello".to_string()));
    }
    #[test]
    fn test_repl_state_unset_option() {
        let mut state = ReplState::new();
        state
            .set_option("pp.all", "true")
            .expect("test operation should succeed");
        state.unset_option("pp.all");
        assert!(!state.options.pp_all);
    }
    #[test]
    fn test_repl_state_get_option() {
        let state = ReplState::new();
        assert_eq!(state.get_option("pp.all"), Some("false".to_string()));
        assert_eq!(state.get_option("pp.unicode"), Some("true".to_string()));
        assert_eq!(state.get_option("pp.width"), Some("100".to_string()));
        assert_eq!(state.get_option("nonexistent"), None);
    }
    #[test]
    fn test_repl_options_default() {
        let opts = ReplOptions::default();
        assert!(!opts.pp_all);
        assert!(!opts.pp_implicit);
        assert!(!opts.pp_universes);
        assert!(opts.pp_unicode);
        assert_eq!(opts.pp_width, 100);
        assert!(!opts.show_timing);
    }
    #[test]
    fn test_parse_bool_true() {
        assert!(parse_bool("true").expect("parsing should succeed"));
        assert!(parse_bool("1").expect("parsing should succeed"));
        assert!(parse_bool("yes").expect("parsing should succeed"));
        assert!(parse_bool("on").expect("parsing should succeed"));
        assert!(parse_bool("TRUE").expect("parsing should succeed"));
    }
    #[test]
    fn test_parse_bool_false() {
        assert!(!parse_bool("false").expect("parsing should succeed"));
        assert!(!parse_bool("0").expect("parsing should succeed"));
        assert!(!parse_bool("no").expect("parsing should succeed"));
        assert!(!parse_bool("off").expect("parsing should succeed"));
    }
    #[test]
    fn test_parse_bool_invalid() {
        assert!(parse_bool("maybe").is_err());
    }
    #[test]
    fn test_repl_mode_eq() {
        assert_eq!(ReplMode::Normal, ReplMode::Normal);
        assert_ne!(ReplMode::Normal, ReplMode::Proof);
        assert_ne!(ReplMode::Proof, ReplMode::Debug);
    }
    #[test]
    fn test_repl_new() {
        let repl = Repl::new();
        assert_eq!(repl.line_number, 1);
        assert!(repl.history.is_empty());
        assert!(repl.input_buffer.is_empty());
    }
    #[test]
    fn test_repl_default() {
        let repl = Repl::default();
        assert_eq!(repl.line_number, 1);
    }
    #[test]
    fn test_undo_entry() {
        let entry = UndoEntry {
            name: Name::str("test"),
            old_env: Environment::new(),
            description: "test undo".to_string(),
        };
        assert_eq!(entry.name, Name::str("test"));
        assert_eq!(entry.description, "test undo");
    }
    #[test]
    fn test_syntax_highlighter_new() {
        let sh = SyntaxHighlighter::new(true);
        assert!(sh.use_colors);
        let sh_no_color = SyntaxHighlighter::new(false);
        assert!(!sh_no_color.use_colors);
    }
    #[test]
    fn test_highlight_keyword_with_color() {
        let sh = SyntaxHighlighter::new(true);
        let highlighted = sh.highlight_keyword("def");
        assert!(highlighted.contains("def"));
        assert!(highlighted.contains("\x1b["));
    }
    #[test]
    fn test_highlight_keyword_no_color() {
        let sh = SyntaxHighlighter::new(false);
        let highlighted = sh.highlight_keyword("def");
        assert_eq!(highlighted, "def");
    }
    #[test]
    fn test_highlight_error() {
        let sh = SyntaxHighlighter::new(true);
        let highlighted = sh.highlight_error("error message");
        assert!(highlighted.contains("error message"));
    }
    #[test]
    fn test_highlight_success() {
        let sh = SyntaxHighlighter::new(true);
        let highlighted = sh.highlight_success("success");
        assert!(highlighted.contains("success"));
    }
    #[test]
    fn test_error_recovery_new() {
        let er = ErrorRecovery::new();
        assert_eq!(er.consecutive_errors, 0);
        assert!(er.last_good_env.is_none());
    }
    #[test]
    fn test_error_recovery_record_success() {
        let mut er = ErrorRecovery::new();
        let env = Environment::new();
        er.record_success(env.clone());
        assert_eq!(er.consecutive_errors, 0);
        assert!(er.get_last_good().is_some());
    }
    #[test]
    fn test_error_recovery_record_error() {
        let mut er = ErrorRecovery::new();
        assert!(!er.record_error());
        assert!(!er.record_error());
        assert!(!er.record_error());
        assert!(!er.record_error());
        assert!(er.record_error());
    }
    #[test]
    fn test_error_recovery_reset() {
        let mut er = ErrorRecovery::new();
        er.record_error();
        er.record_error();
        assert_eq!(er.consecutive_errors, 2);
        er.reset();
        assert_eq!(er.consecutive_errors, 0);
    }
    #[test]
    fn test_parse_command_browse() {
        if let ReplCmd::Browse(module) = parse_command(":browse") {
            assert!(module.is_none());
        } else {
            panic!("expected Browse command");
        }
        if let ReplCmd::Browse(Some(m)) = parse_command(":browse Nat") {
            assert_eq!(m, "Nat");
        } else {
            panic!("expected Browse with module");
        }
    }
    #[test]
    fn test_parse_command_history_search() {
        if let ReplCmd::HistorySearch(Some(q)) = parse_command(":history def") {
            assert_eq!(q, "def");
        } else {
            panic!("expected HistorySearch with query");
        }
    }
    #[test]
    fn test_parse_command_load_session() {
        if let ReplCmd::LoadSession(path) = parse_command(":load-session session.lean") {
            assert_eq!(path, "session.lean");
        } else {
            panic!("expected LoadSession command");
        }
    }
    #[test]
    fn test_history_search_reverse() {
        let mut h = History::new(100);
        h.add_entry("def foo := 1".to_string());
        h.add_entry("theorem bar : Prop := sorry".to_string());
        h.add_entry("def baz := 2".to_string());
        let result = h.search_reverse("def");
        assert_eq!(result, Some("def baz := 2"));
    }
    #[test]
    fn test_history_search_reverse_no_match() {
        let mut h = History::new(100);
        h.add_entry("theorem bar : Prop := sorry".to_string());
        let result = h.search_reverse("xyz");
        assert!(result.is_none());
    }
    #[test]
    fn test_complete_with_proof_context() {
        let c = Completer::new();
        let env = Environment::new();
        let results = c.complete_with_context("in", &env, "by intro");
        assert!(!results.is_empty());
    }
    #[test]
    fn test_completer_get_help() {
        let c = Completer::new();
        let help = c.get_help("intro");
        assert!(help.contains("intro"));
        assert!(help.contains("hypothesis"));
    }
    #[test]
    fn test_completer_get_help_command() {
        let c = Completer::new();
        let help = c.get_help(":type");
        assert!(help.contains(":type"));
    }
    #[test]
    fn test_completer_get_help_unknown() {
        let c = Completer::new();
        let help = c.get_help("nonexistent");
        assert!(help.contains("not available"));
    }
    #[test]
    fn test_session_metadata_creation() {
        let metadata = SessionMetadata {
            created_at: 0,
            modified_at: 100,
            declaration_count: 5,
            name: "test_session".to_string(),
        };
        assert_eq!(metadata.declaration_count, 5);
        assert_eq!(metadata.name, "test_session");
    }
    #[test]
    fn test_session_snapshot_creation() {
        let metadata = SessionMetadata {
            created_at: 0,
            modified_at: 100,
            declaration_count: 3,
            name: "test".to_string(),
        };
        let mut settings = HashMap::new();
        settings.insert("pp.all".to_string(), "true".to_string());
        let snapshot = SessionSnapshot {
            metadata,
            history_entries: vec!["entry1".to_string()],
            settings,
        };
        assert_eq!(snapshot.history_entries.len(), 1);
        assert_eq!(snapshot.settings.get("pp.all"), Some(&"true".to_string()));
    }
    #[test]
    fn test_repl_new_with_all_fields() {
        let repl = Repl::new();
        assert_eq!(repl.line_number, 1);
        assert!(repl.history.is_empty());
        assert!(repl.input_buffer.is_empty());
        assert!(repl.browse_history.is_empty());
        assert_eq!(repl.error_recovery.consecutive_errors, 0);
    }
    #[test]
    fn test_parse_command_all_new_variants() {
        assert!(matches!(parse_command(":browse"), ReplCmd::Browse(_)));
        assert!(matches!(
            parse_command(":browse Nat"),
            ReplCmd::Browse(Some(_))
        ));
        assert!(matches!(parse_command(":history"), ReplCmd::ShowHistory));
        assert!(matches!(
            parse_command(":history def"),
            ReplCmd::HistorySearch(_)
        ));
        assert!(matches!(
            parse_command(":load-session test.lean"),
            ReplCmd::LoadSession(_)
        ));
    }
}
