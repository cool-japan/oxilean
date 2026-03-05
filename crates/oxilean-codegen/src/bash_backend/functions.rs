//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::types::{
    BashAnalysisCache, BashArgParser, BashBackend, BashCliOption, BashCondition,
    BashConstantFoldingHelper, BashDepGraph, BashDominatorTree, BashExpr, BashFunction,
    BashHereDoc, BashHeredoc, BashJobManager, BashLivenessInfo, BashLogLevel, BashLogger,
    BashPassConfig, BashPassPhase, BashPassRegistry, BashPassStats, BashScript, BashTemplate,
    BashTrap, BashVar, BashWorklist,
};

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn backend() -> BashBackend {
        BashBackend::new()
    }
    #[test]
    pub(super) fn test_bash_var_display_local() {
        let v = BashVar::Local("count".to_string());
        assert_eq!(format!("{}", v), "${count}");
    }
    #[test]
    pub(super) fn test_bash_var_display_env() {
        let v = BashVar::Env("HOME".to_string());
        assert_eq!(format!("{}", v), "${HOME}");
    }
    #[test]
    pub(super) fn test_bash_var_name() {
        let v = BashVar::AssocArray("my_map".to_string());
        assert_eq!(v.name(), "my_map");
    }
    #[test]
    pub(super) fn test_bash_var_is_exported() {
        assert!(BashVar::Env("X".to_string()).is_exported());
        assert!(!BashVar::Local("X".to_string()).is_exported());
        assert!(!BashVar::Global("X".to_string()).is_exported());
    }
    #[test]
    pub(super) fn test_bash_var_is_readonly() {
        assert!(BashVar::Readonly("X".to_string()).is_readonly());
        assert!(!BashVar::Local("X".to_string()).is_readonly());
    }
    #[test]
    pub(super) fn test_bash_var_decl_local() {
        let b = backend();
        let v = BashVar::Local("x".to_string());
        assert_eq!(b.emit_var_decl(&v, Some("0")), "local x=0");
        assert_eq!(b.emit_var_decl(&v, None), "local x");
    }
    #[test]
    pub(super) fn test_bash_var_decl_export() {
        let b = backend();
        let v = BashVar::Env("PATH".to_string());
        let result = b.emit_var_decl(&v, Some("/usr/bin"));
        assert_eq!(result, "export PATH=/usr/bin");
    }
    #[test]
    pub(super) fn test_bash_var_decl_integer() {
        let b = backend();
        let v = BashVar::Integer("n".to_string());
        assert_eq!(b.emit_var_decl(&v, Some("42")), "declare -i n=42");
    }
    #[test]
    pub(super) fn test_bash_var_decl_assoc_array() {
        let b = backend();
        let v = BashVar::AssocArray("opts".to_string());
        assert_eq!(b.emit_var_decl(&v, None), "declare -A opts");
    }
    #[test]
    pub(super) fn test_bash_var_decl_nameref() {
        let b = backend();
        let v = BashVar::NameRef("ref".to_string());
        assert_eq!(b.emit_var_decl(&v, Some("target")), "declare -n ref=target");
    }
    #[test]
    pub(super) fn test_bash_expr_var() {
        let b = backend();
        let expr = BashExpr::Var(BashVar::Local("x".to_string()));
        assert_eq!(b.emit_expr(&expr), "${x}");
    }
    #[test]
    pub(super) fn test_bash_expr_lit() {
        let b = backend();
        let expr = BashExpr::Lit("hello world".to_string());
        assert_eq!(b.emit_expr(&expr), "'hello world'");
    }
    #[test]
    pub(super) fn test_bash_expr_dquoted() {
        let b = backend();
        let expr = BashExpr::DQuoted("hello $name".to_string());
        assert_eq!(b.emit_expr(&expr), "\"hello $name\"");
    }
    #[test]
    pub(super) fn test_bash_expr_cmd_subst() {
        let b = backend();
        let expr = BashExpr::CmdSubst("date +%s".to_string());
        assert_eq!(b.emit_expr(&expr), "$(date +%s)");
    }
    #[test]
    pub(super) fn test_bash_expr_arith() {
        let b = backend();
        let expr = BashExpr::ArithExpr("x + y * 2".to_string());
        assert_eq!(b.emit_expr(&expr), "$((x + y * 2))");
    }
    #[test]
    pub(super) fn test_bash_expr_array_elem() {
        let b = backend();
        let expr = BashExpr::ArrayElem("arr".to_string(), Box::new(BashExpr::Lit("0".to_string())));
        assert_eq!(b.emit_expr(&expr), "${arr['0']}");
    }
    #[test]
    pub(super) fn test_bash_expr_array_len() {
        let b = backend();
        let expr = BashExpr::ArrayLen("items".to_string());
        assert_eq!(b.emit_expr(&expr), "${#items[@]}");
    }
    #[test]
    pub(super) fn test_bash_expr_array_all() {
        let b = backend();
        let expr = BashExpr::ArrayAll("items".to_string());
        assert_eq!(b.emit_expr(&expr), "\"${items[@]}\"");
    }
    #[test]
    pub(super) fn test_bash_expr_default() {
        let b = backend();
        let expr = BashExpr::Default(
            "NAME".to_string(),
            Box::new(BashExpr::Lit("world".to_string())),
        );
        assert_eq!(b.emit_expr(&expr), "${NAME:-'world'}");
    }
    #[test]
    pub(super) fn test_bash_expr_string_len() {
        let b = backend();
        let expr = BashExpr::StringLen("msg".to_string());
        assert_eq!(b.emit_expr(&expr), "${#msg}");
    }
    #[test]
    pub(super) fn test_bash_expr_strip_prefix() {
        let b = backend();
        let expr = BashExpr::StripPrefix("path".to_string(), "*/".to_string());
        assert_eq!(b.emit_expr(&expr), "${path#*/}");
    }
    #[test]
    pub(super) fn test_bash_expr_strip_suffix() {
        let b = backend();
        let expr = BashExpr::StripSuffix("file".to_string(), ".txt".to_string());
        assert_eq!(b.emit_expr(&expr), "${file%.txt}");
    }
    #[test]
    pub(super) fn test_bash_expr_uppercase() {
        let b = backend();
        let expr = BashExpr::UpperCase("word".to_string());
        assert_eq!(b.emit_expr(&expr), "${word^^}");
    }
    #[test]
    pub(super) fn test_bash_expr_lowercase() {
        let b = backend();
        let expr = BashExpr::LowerCase("word".to_string());
        assert_eq!(b.emit_expr(&expr), "${word,,}");
    }
    #[test]
    pub(super) fn test_bash_expr_special_vars() {
        let b = backend();
        assert_eq!(b.emit_expr(&BashExpr::LastStatus), "$?");
        assert_eq!(b.emit_expr(&BashExpr::ShellPid), "$$");
        assert_eq!(b.emit_expr(&BashExpr::ScriptName), "$0");
        assert_eq!(b.emit_expr(&BashExpr::AllArgs), "\"$@\"");
        assert_eq!(b.emit_expr(&BashExpr::ArgCount), "$#");
        assert_eq!(b.emit_expr(&BashExpr::Positional(1)), "$1");
    }
    #[test]
    pub(super) fn test_bash_expr_substring() {
        let b = backend();
        let e1 = BashExpr::Substring("s".to_string(), 2, None);
        let e2 = BashExpr::Substring("s".to_string(), 2, Some(5));
        assert_eq!(b.emit_expr(&e1), "${s:2}");
        assert_eq!(b.emit_expr(&e2), "${s:2:5}");
    }
    #[test]
    pub(super) fn test_bash_expr_concat() {
        let b = backend();
        let expr = BashExpr::Concat(
            Box::new(BashExpr::DQuoted("hello ".to_string())),
            Box::new(BashExpr::Var(BashVar::Local("name".to_string()))),
        );
        assert_eq!(b.emit_expr(&expr), "\"hello \"${name}");
    }
    #[test]
    pub(super) fn test_bash_condition_file_exists() {
        let b = backend();
        let cond = BashCondition::FileExists(BashExpr::DQuoted("/etc/passwd".to_string()));
        assert_eq!(b.emit_condition(&cond), "[[ -e \"/etc/passwd\" ]]");
    }
    #[test]
    pub(super) fn test_bash_condition_str_eq() {
        let b = backend();
        let cond = BashCondition::StrEq(
            BashExpr::Var(BashVar::Local("a".to_string())),
            BashExpr::Lit("yes".to_string()),
        );
        assert_eq!(b.emit_condition(&cond), "[[ ${a} == 'yes' ]]");
    }
    #[test]
    pub(super) fn test_bash_condition_arith_lt() {
        let b = backend();
        let cond = BashCondition::ArithLt("count".to_string(), "10".to_string());
        assert_eq!(b.emit_condition(&cond), "(( count < 10 ))");
    }
    #[test]
    pub(super) fn test_bash_condition_not() {
        let b = backend();
        let inner = BashCondition::IsFile(BashExpr::Lit("config.txt".to_string()));
        let cond = BashCondition::Not(Box::new(inner));
        assert_eq!(b.emit_condition(&cond), "! [[ -f 'config.txt' ]]");
    }
    #[test]
    pub(super) fn test_mangle_simple() {
        let b = backend();
        assert_eq!(b.mangle_name("my_func"), "my_func");
        assert_eq!(b.mangle_name("MyFunc"), "MyFunc");
    }
    #[test]
    pub(super) fn test_mangle_dot_separator() {
        let b = backend();
        assert_eq!(b.mangle_name("Nat.add"), "Nat__add");
    }
    #[test]
    pub(super) fn test_mangle_colon_separator() {
        let b = backend();
        assert_eq!(b.mangle_name("List::map"), "List__map");
    }
    #[test]
    pub(super) fn test_mangle_leading_digit() {
        let b = backend();
        let result = b.mangle_name("3foo");
        assert!(
            result.starts_with('_'),
            "expected _ prefix, got: {}",
            result
        );
        assert!(result.contains("foo"));
    }
    #[test]
    pub(super) fn test_mangle_reserved_builtin() {
        let b = backend();
        assert_eq!(b.mangle_name("echo"), "echo__ox");
        assert_eq!(b.mangle_name("read"), "read__ox");
        assert_eq!(b.mangle_name("local"), "local__ox");
    }
    #[test]
    pub(super) fn test_emit_simple_function() {
        let b = backend();
        let func = BashFunction::new("greet", vec!["echo \"Hello, $1!\"".to_string()]);
        let code = b.emit_function(&func);
        assert!(code.contains("greet() {"), "got: {}", code);
        assert!(code.contains("echo \"Hello, $1!\""), "got: {}", code);
        assert!(code.contains("}\n"), "got: {}", code);
    }
    #[test]
    pub(super) fn test_emit_function_with_locals() {
        let b = backend();
        let func = BashFunction::with_locals(
            "add",
            vec!["a=$1".to_string(), "b=$2".to_string()],
            vec!["echo $(( a + b ))".to_string()],
        );
        let code = b.emit_function(&func);
        assert!(code.contains("add() {"));
        assert!(code.contains("local a=$1"));
        assert!(code.contains("local b=$2"));
        assert!(code.contains("echo $(( a + b ))"));
    }
    #[test]
    pub(super) fn test_emit_function_with_description() {
        let b = backend();
        let mut func = BashFunction::new("helper", vec!["true".to_string()]);
        func.description = Some("This is a helper function".to_string());
        let code = b.emit_function(&func);
        assert!(code.contains("# This is a helper function"));
    }
    #[test]
    pub(super) fn test_emit_heredoc() {
        let b = backend();
        let hd = BashHereDoc::new("EOF", vec!["line1".to_string(), "line2".to_string()]);
        let code = b.emit_heredoc(&hd);
        assert!(code.contains("<<EOF"), "got: {}", code);
        assert!(code.contains("line1"));
        assert!(code.contains("line2"));
        assert!(code.contains("EOF"));
    }
    #[test]
    pub(super) fn test_emit_heredoc_no_expand() {
        let hd = BashHereDoc {
            delimiter: "EOF".to_string(),
            strip_tabs: false,
            no_expand: true,
            content: vec!["$literal".to_string()],
        };
        let b = backend();
        let code = b.emit_heredoc(&hd);
        assert!(code.contains("<<'EOF'"), "got: {}", code);
    }
    #[test]
    pub(super) fn test_emit_script_shebang() {
        let b = backend();
        let script = BashScript::new();
        let code = b.emit_script(&script);
        assert!(code.starts_with("#!/usr/bin/env bash\n"), "got: {}", code);
    }
    #[test]
    pub(super) fn test_emit_script_set_flags() {
        let b = backend();
        let script = BashScript::new();
        let code = b.emit_script(&script);
        assert!(code.contains("set -euo pipefail"), "got: {}", code);
    }
    #[test]
    pub(super) fn test_emit_script_with_globals() {
        let b = backend();
        let mut script = BashScript::new();
        script
            .globals
            .push(("VERSION".to_string(), "1.0.0".to_string()));
        let code = b.emit_script(&script);
        assert!(code.contains("readonly VERSION=1.0.0"), "got: {}", code);
    }
    #[test]
    pub(super) fn test_emit_script_with_functions() {
        let b = backend();
        let mut script = BashScript::new();
        script.functions.push(BashFunction::new(
            "main",
            vec!["echo 'running'".to_string()],
        ));
        let code = b.emit_script(&script);
        assert!(code.contains("main() {"));
        assert!(code.contains("echo 'running'"));
    }
    #[test]
    pub(super) fn test_emit_script_with_trap() {
        let b = backend();
        let mut script = BashScript::new();
        script
            .traps
            .push(("EXIT".to_string(), "cleanup".to_string()));
        let code = b.emit_script(&script);
        assert!(code.contains("trap 'cleanup' EXIT"), "got: {}", code);
    }
    #[test]
    pub(super) fn test_emit_script_full() {
        let b = backend();
        let mut script = BashScript::new();
        script
            .globals
            .push(("PROG".to_string(), "oxilean".to_string()));
        script.functions.push(BashFunction::new(
            "usage",
            vec!["echo \"Usage: $PROG [options]\"".to_string()],
        ));
        script.main.push("usage".to_string());
        let code = b.emit_script(&script);
        assert!(code.contains("#!/usr/bin/env bash"));
        assert!(code.contains("readonly PROG=oxilean"));
        assert!(code.contains("usage() {"));
        assert!(code.contains("usage"));
    }
    #[test]
    pub(super) fn test_emit_array_assign() {
        let b = backend();
        let elems = vec![
            BashExpr::Lit("a".to_string()),
            BashExpr::Lit("b".to_string()),
            BashExpr::Lit("c".to_string()),
        ];
        let code = b.emit_array_assign("items", &elems);
        assert_eq!(code, "items=('a' 'b' 'c')");
    }
    #[test]
    pub(super) fn test_emit_assoc_array_assign() {
        let b = backend();
        let pairs = vec![
            ("key1".to_string(), "val1".to_string()),
            ("key2".to_string(), "val2".to_string()),
        ];
        let code = b.emit_assoc_array_assign("opts", &pairs);
        assert!(code.contains("declare -A opts"), "got: {}", code);
        assert!(code.contains("opts[key1]=val1"), "got: {}", code);
        assert!(code.contains("opts[key2]=val2"), "got: {}", code);
    }
    #[test]
    pub(super) fn test_emit_if() {
        let b = backend();
        let cond = BashCondition::StrEq(
            BashExpr::Var(BashVar::Local("x".to_string())),
            BashExpr::Lit("yes".to_string()),
        );
        let code = b.emit_if(&cond, &["echo 'yes'"], Some(&["echo 'no'"]));
        assert!(
            code.contains("if [[ ${x} == 'yes' ]]; then"),
            "got: {}",
            code
        );
        assert!(code.contains("echo 'yes'"), "got: {}", code);
        assert!(code.contains("else"), "got: {}", code);
        assert!(code.contains("echo 'no'"), "got: {}", code);
        assert!(code.contains("fi"), "got: {}", code);
    }
    #[test]
    pub(super) fn test_emit_for_in() {
        let b = backend();
        let items = vec![
            BashExpr::Lit("a".to_string()),
            BashExpr::Lit("b".to_string()),
        ];
        let code = b.emit_for_in("item", &items, &["echo $item"]);
        assert!(code.contains("for item in 'a' 'b'; do"), "got: {}", code);
        assert!(code.contains("echo $item"), "got: {}", code);
        assert!(code.contains("done"), "got: {}", code);
    }
    #[test]
    pub(super) fn test_emit_while() {
        let b = backend();
        let cond = BashCondition::ArithLt("i".to_string(), "10".to_string());
        let code = b.emit_while(&cond, &["echo $i", "(( i++ ))"]);
        assert!(code.contains("while (( i < 10 )); do"), "got: {}", code);
        assert!(code.contains("echo $i"), "got: {}", code);
        assert!(code.contains("done"), "got: {}", code);
    }
    #[test]
    pub(super) fn test_emit_case() {
        let b = backend();
        let expr = BashExpr::Var(BashVar::Local("cmd".to_string()));
        let arms = vec![
            ("start", vec!["do_start"]),
            ("stop", vec!["do_stop"]),
            ("*", vec!["echo 'unknown'"]),
        ];
        let code = b.emit_case(&expr, &arms);
        assert!(code.contains("case ${cmd} in"), "got: {}", code);
        assert!(code.contains("start)"), "got: {}", code);
        assert!(code.contains("do_start"), "got: {}", code);
        assert!(code.contains("esac"), "got: {}", code);
    }
    #[test]
    pub(super) fn test_lenient_script_no_strict() {
        let b = backend();
        let script = BashScript::lenient();
        let code = b.emit_script(&script);
        assert!(code.starts_with("#!/usr/bin/env bash"));
        assert!(
            !code.contains("set -euo pipefail"),
            "lenient should not have strict mode"
        );
    }
    #[test]
    pub(super) fn test_compact_backend_indent() {
        let b = BashBackend::compact();
        let func = BashFunction::new("f", vec!["echo hi".to_string()]);
        let code = b.emit_function(&func);
        assert!(code.contains("  echo hi"), "got: {}", code);
    }
}
/// ANSI terminal color constants for bash scripts.
#[allow(dead_code)]
pub mod bash_colors {
    pub const RESET: &str = "\\033[0m";
    pub const BOLD: &str = "\\033[1m";
    pub const DIM: &str = "\\033[2m";
    pub const ITALIC: &str = "\\033[3m";
    pub const UNDERLINE: &str = "\\033[4m";
    pub const BLINK: &str = "\\033[5m";
    pub const REVERSE: &str = "\\033[7m";
    pub const HIDDEN: &str = "\\033[8m";
    pub const STRIKE: &str = "\\033[9m";
    pub const BLACK: &str = "\\033[30m";
    pub const RED: &str = "\\033[31m";
    pub const GREEN: &str = "\\033[32m";
    pub const YELLOW: &str = "\\033[33m";
    pub const BLUE: &str = "\\033[34m";
    pub const MAGENTA: &str = "\\033[35m";
    pub const CYAN: &str = "\\033[36m";
    pub const WHITE: &str = "\\033[37m";
    pub const BRIGHT_BLACK: &str = "\\033[90m";
    pub const BRIGHT_RED: &str = "\\033[91m";
    pub const BRIGHT_GREEN: &str = "\\033[92m";
    pub const BRIGHT_YELLOW: &str = "\\033[93m";
    pub const BRIGHT_BLUE: &str = "\\033[94m";
    pub const BRIGHT_MAGENTA: &str = "\\033[95m";
    pub const BRIGHT_CYAN: &str = "\\033[96m";
    pub const BRIGHT_WHITE: &str = "\\033[97m";
    pub const BG_RED: &str = "\\033[41m";
    pub const BG_GREEN: &str = "\\033[42m";
    pub const BG_YELLOW: &str = "\\033[43m";
    pub const BG_BLUE: &str = "\\033[44m";
    pub const BG_MAGENTA: &str = "\\033[45m";
    pub const BG_CYAN: &str = "\\033[46m";
}
/// Generate bash code to split a string by a delimiter.
#[allow(dead_code)]
pub fn emit_bash_split(str_var: &str, delim: &str, arr_var: &str) -> std::string::String {
    format!(
        "IFS='{}' read -r -a {} <<< \"${{{}}}\"\n",
        delim, arr_var, str_var
    )
}
/// Generate bash code to join an array with a delimiter.
#[allow(dead_code)]
pub fn emit_bash_join(arr_var: &str, delim: &str, result_var: &str) -> std::string::String {
    format!(
        "local IFS='{}'; {}=\"${{{}[*]}}\"\n",
        delim, result_var, arr_var
    )
}
/// Generate bash code to trim whitespace from a variable.
#[allow(dead_code)]
pub fn emit_bash_trim(var: &str, result_var: &str) -> std::string::String {
    format!(
        "{var}=\"${{${{{var}}}##*( )}}\"  # trim leading\n{result_var}=\"${{${{{var}}}%%*( )}}\"  # trim trailing\n",
        var = var, result_var = result_var
    )
}
/// Generate bash code to URL-encode a string.
#[allow(dead_code)]
pub fn emit_bash_url_encode(var: &str, result_var: &str) -> std::string::String {
    format!(
        "{result}=$(printf '%s' \"${{{var}}}\" | jq -Rr @uri 2>/dev/null || python3 -c \"import sys,urllib.parse; print(urllib.parse.quote(sys.stdin.read().rstrip()))\" <<< \"${{{var}}}\")\n",
        var = var, result = result_var
    )
}
/// Generate bash code to check if a command exists.
#[allow(dead_code)]
pub fn emit_bash_require_cmd(cmd: &str) -> std::string::String {
    format!(
        "command -v {} &>/dev/null || {{ echo \"Error: '{}' not found in PATH\" >&2; exit 1; }}\n",
        cmd, cmd
    )
}
/// Emits bash code to source a .env or .conf file.
#[allow(dead_code)]
pub fn emit_bash_source_env(file_path: &str, required: bool) -> std::string::String {
    if required {
        format!(
            "if [[ -f \"{path}\" ]]; then\n  # shellcheck source=/dev/null\n  source \"{path}\"\nelse\n  echo \"Error: config file not found: {path}\" >&2\n  exit 1\nfi\n",
            path = file_path
        )
    } else {
        format!(
            "if [[ -f \"{path}\" ]]; then\n  # shellcheck source=/dev/null\n  source \"{path}\"\nfi\n",
            path = file_path
        )
    }
}
/// Emits bash code to read a .env file safely (without sourcing).
#[allow(dead_code)]
pub fn emit_bash_read_env(file_path: &str) -> std::string::String {
    format!(
        "while IFS='=' read -r _key _val || [[ -n \"$_key\" ]]; do\n  [[ \"$_key\" =~ ^#.*$ || -z \"$_key\" ]] && continue\n  export \"$_key\"=\"$_val\"\ndone < \"{}\"\n",
        file_path
    )
}
/// Emits bash code to acquire an exclusive lock file.
#[allow(dead_code)]
pub fn emit_bash_lock(lock_file: &str, lock_fd_var: &str) -> std::string::String {
    format!(
        "exec {fd}<>\"{file}\"\nflock -n ${fd} || {{ echo \"Another instance is running\" >&2; exit 1; }}\n",
        fd = lock_fd_var, file = lock_file
    )
}
/// Emits bash code to release a lock file.
#[allow(dead_code)]
pub fn emit_bash_unlock(lock_fd_var: &str) -> std::string::String {
    format!("flock -u ${}\n", lock_fd_var)
}
/// Emits a bash retry wrapper function.
#[allow(dead_code)]
pub fn emit_bash_retry_fn(max_attempts: u8, delay_secs: u8) -> std::string::String {
    format!(
        "retry() {{\n  local _attempt=1\n  until \"$@\"; do\n    _attempt=$(( _attempt + 1 ))\n    if [[ $_attempt -gt {max} ]]; then\n      echo \"Command failed after {max} attempts\" >&2\n      return 1\n    fi\n    echo \"Attempt $_attempt of {max} failed, retrying in {delay}s...\" >&2\n    sleep {delay}\n  done\n}}\n",
        max = max_attempts, delay = delay_secs
    )
}
/// Emits a bash progress bar function.
#[allow(dead_code)]
pub fn emit_bash_progress_bar(width: usize) -> std::string::String {
    format!(
        "progress_bar() {{\n  local current=$1 total=$2 width={w}\n  local pct=$(( current * 100 / total ))\n  local filled=$(( pct * width / 100 ))\n  local bar; bar=$(printf '%0.s#' $(seq 1 $filled))\n  local empty; empty=$(printf '%0.s-' $(seq 1 $(( width - filled ))))\n  printf \"\\r[%s%s] %d%%\" \"$bar\" \"$empty\" \"$pct\"\n  [[ $current -eq $total ]] && echo\n}}\n",
        w = width
    )
}
/// Emits bash code to make an HTTP GET request.
#[allow(dead_code)]
pub fn emit_bash_http_get(
    url_var: &str,
    result_var: &str,
    _timeout_secs: u8,
) -> std::string::String {
    format!(
        "{result}=$(curl -fsSL --max-time {timeout} \"${{{url}}}\" 2>/dev/null)\nif [[ $? -ne 0 ]]; then\n  echo \"HTTP GET failed for ${{url}}\" >&2\n  exit 1\nfi\n",
        result = result_var, timeout = _timeout_secs, url = url_var
    )
}
/// Emits bash code to make an HTTP POST request with JSON.
#[allow(dead_code)]
pub fn emit_bash_http_post_json(
    url_var: &str,
    body_var: &str,
    result_var: &str,
    _timeout_secs: u8,
) -> std::string::String {
    format!(
        "{result}=$(curl -fsSL --max-time {timeout} -X POST -H 'Content-Type: application/json' -d \"${{{body}}}\" \"${{{url}}}\" 2>/dev/null)\n",
        result = result_var, timeout = _timeout_secs, body = body_var, url = url_var
    )
}
/// Emit a C-style for loop.
#[allow(dead_code)]
pub fn emit_bash_for_arith(
    init: &str,
    cond: &str,
    incr: &str,
    body: &[&str],
    indent: &str,
) -> std::string::String {
    let mut out = format!("for (( {}; {}; {} )); do\n", init, cond, incr);
    for stmt in body {
        out.push_str(&format!("{}{}\n", indent, stmt));
    }
    out.push_str("done\n");
    out
}
/// Emit a bash until loop.
#[allow(dead_code)]
pub fn emit_bash_until(cond: &BashCondition, body: &[&str], indent: &str) -> std::string::String {
    let mut out = format!("until {}; do\n", cond);
    for stmt in body {
        out.push_str(&format!("{}{}\n", indent, stmt));
    }
    out.push_str("done\n");
    out
}
/// Emit a select menu.
#[allow(dead_code)]
pub fn emit_bash_select_menu(
    var: &str,
    choices: &[&str],
    body: &[&str],
    indent: &str,
) -> std::string::String {
    let choices_str: Vec<std::string::String> =
        choices.iter().map(|c| format!("\"{}\"", c)).collect();
    let mut out = format!(
        "PS3=\"Select: \"\nselect {} in {}; do\n",
        var,
        choices_str.join(" ")
    );
    for stmt in body {
        out.push_str(&format!("{}{}\n", indent, stmt));
    }
    out.push_str("done\n");
    out
}
#[cfg(test)]
mod bash_extended_tests {
    use super::*;
    #[test]
    pub(super) fn test_heredoc_emit() {
        let h = BashHeredoc::new("EOF")
            .line("Hello, World!")
            .line("Second line");
        let out = h.emit();
        assert!(out.contains("<<EOF"), "missing heredoc tag: {}", out);
        assert!(out.contains("Hello"), "missing content: {}", out);
    }
    #[test]
    pub(super) fn test_heredoc_quoted() {
        let h = BashHeredoc::new("SCRIPT").quoted().line("$var");
        let out = h.emit();
        assert!(out.contains("<<'SCRIPT'"), "missing quoted tag: {}", out);
    }
    #[test]
    pub(super) fn test_trap_emit() {
        let t = BashTrap::on_exit("cleanup");
        assert_eq!(t.emit(), "trap 'cleanup' EXIT");
        let t2 = BashTrap::on_err("handle_error");
        assert_eq!(t2.emit(), "trap 'handle_error' ERR");
        let t3 = BashTrap::reset("INT");
        assert!(t3.emit().contains("INT"), "missing signal: {}", t3.emit());
    }
    #[test]
    pub(super) fn test_log_level_ordering() {
        assert!(BashLogLevel::Debug < BashLogLevel::Info);
        assert!(BashLogLevel::Info < BashLogLevel::Warn);
        assert!(BashLogLevel::Warn < BashLogLevel::Error);
        assert!(BashLogLevel::Error < BashLogLevel::Fatal);
        assert!(BashLogLevel::Error.is_stderr());
        assert!(BashLogLevel::Fatal.is_stderr());
        assert!(!BashLogLevel::Info.is_stderr());
    }
    #[test]
    pub(super) fn test_logger_emit_framework() {
        let logger = BashLogger::new()
            .with_timestamps()
            .with_color()
            .with_min_level(BashLogLevel::Debug)
            .with_log_file("/var/log/app.log");
        let code = logger.emit_framework();
        assert!(code.contains("_log()"), "missing _log: {}", code);
        assert!(code.contains("log_debug"), "missing log_debug: {}", code);
        assert!(code.contains("log_fatal"), "missing log_fatal: {}", code);
        assert!(
            code.contains("/var/log/app.log"),
            "missing log file: {}",
            code
        );
    }
    #[test]
    pub(super) fn test_cli_option_flag() {
        let opt = BashCliOption::flag("verbose", Some('v'), "verbose", "Enable verbose output");
        assert!(!opt.has_arg);
        assert_eq!(opt.default, Some("false".to_string()));
        assert!(!opt.required);
    }
    #[test]
    pub(super) fn test_arg_parser_emit() {
        let parser = BashArgParser::new("myapp", "A test application")
            .add_option(BashCliOption::flag(
                "verbose",
                Some('v'),
                "verbose",
                "Verbose mode",
            ))
            .add_option(
                BashCliOption::arg(
                    "output",
                    Some('o'),
                    "output",
                    Some("/tmp/out"),
                    "Output file",
                )
                .required(),
            )
            .add_positional("input_file");
        let usage = parser.emit_usage();
        assert!(usage.contains("usage()"), "missing usage fn: {}", usage);
        assert!(usage.contains("--verbose"), "missing verbose: {}", usage);
        assert!(usage.contains("--output"), "missing output: {}", usage);
        let parse = parser.emit_parse_block();
        assert!(
            parse.contains("VERBOSE=false"),
            "missing default: {}",
            parse
        );
        assert!(
            parse.contains("--verbose"),
            "missing verbose case: {}",
            parse
        );
        assert!(
            parse.contains("OUTPUT is required"),
            "missing required check: {}",
            parse
        );
    }
    #[test]
    pub(super) fn test_job_manager_emit() {
        let jm = BashJobManager::new(4).with_pids_var("JOB_PIDS");
        let code = jm.emit_framework();
        assert!(code.contains("_MAX_JOBS=4"), "missing max_jobs: {}", code);
        assert!(code.contains("JOB_PIDS"), "missing pids var: {}", code);
        assert!(code.contains("run_job()"), "missing run_job: {}", code);
        assert!(code.contains("_wait_jobs()"), "missing wait_jobs: {}", code);
    }
    #[test]
    pub(super) fn test_template_render() {
        let tmpl = BashTemplate::new("Hello, {{NAME}}! You are {{AGE}} years old.")
            .set("NAME", "Alice")
            .set("AGE", "30");
        let rendered = tmpl.render();
        assert_eq!(rendered, "Hello, Alice! You are 30 years old.");
    }
    #[test]
    pub(super) fn test_split_join_emit() {
        let split = emit_bash_split("INPUT", ":", "PARTS");
        assert!(split.contains("IFS=':'"), "missing IFS: {}", split);
        assert!(split.contains("PARTS"), "missing arr: {}", split);
        let join = emit_bash_join("PARTS", ",", "RESULT");
        assert!(join.contains("IFS=','"), "missing IFS: {}", join);
    }
    #[test]
    pub(super) fn test_retry_fn_emit() {
        let code = emit_bash_retry_fn(3, 5);
        assert!(code.contains("retry()"), "missing fn: {}", code);
        assert!(code.contains("3"), "missing max: {}", code);
        assert!(code.contains("sleep 5"), "missing sleep: {}", code);
    }
    #[test]
    pub(super) fn test_progress_bar_emit() {
        let code = emit_bash_progress_bar(50);
        assert!(code.contains("progress_bar()"), "missing fn: {}", code);
        assert!(code.contains("50"), "missing width: {}", code);
    }
    #[test]
    pub(super) fn test_source_env_emit() {
        let required = emit_bash_source_env("/etc/app/config.conf", true);
        assert!(required.contains("source"), "missing source: {}", required);
        assert!(required.contains("exit 1"), "missing exit: {}", required);
        let optional = emit_bash_source_env("/etc/app/config.conf", false);
        assert!(optional.contains("source"), "missing source: {}", optional);
        assert!(
            !optional.contains("exit 1"),
            "should not exit: {}",
            optional
        );
    }
    #[test]
    pub(super) fn test_lock_unlock_emit() {
        let lock = emit_bash_lock("/var/run/app.lock", "9");
        assert!(lock.contains("flock"), "missing flock: {}", lock);
        let unlock = emit_bash_unlock("9");
        assert!(unlock.contains("flock -u"), "missing unlock: {}", unlock);
    }
    #[test]
    pub(super) fn test_for_arith_emit() {
        let code = emit_bash_for_arith("i=0", "i<10", "i++", &["echo $i"], "  ");
        assert!(
            code.contains("for (( i=0; i<10; i++ ))"),
            "missing for arith: {}",
            code
        );
        assert!(code.contains("echo $i"), "missing body: {}", code);
        assert!(code.contains("done"), "missing done: {}", code);
    }
    #[test]
    pub(super) fn test_bash_colors_module() {
        assert!(bash_colors::RED.contains("31m"));
        assert!(bash_colors::GREEN.contains("32m"));
        assert!(bash_colors::BLUE.contains("34m"));
        assert!(bash_colors::RESET.contains("0m"));
        assert!(bash_colors::BOLD.contains("1m"));
    }
    #[test]
    pub(super) fn test_require_cmd_emit() {
        let code = emit_bash_require_cmd("jq");
        assert!(
            code.contains("command -v jq"),
            "missing command check: {}",
            code
        );
        assert!(code.contains("exit 1"), "missing exit: {}", code);
    }
    #[test]
    pub(super) fn test_http_get_emit() {
        let code = emit_bash_http_get("URL", "RESPONSE", 30);
        assert!(code.contains("curl"), "missing curl: {}", code);
        assert!(code.contains("RESPONSE"), "missing result var: {}", code);
        assert!(code.contains("30"), "missing timeout: {}", code);
    }
}
#[cfg(test)]
mod Bash_infra_tests {
    use super::*;
    #[test]
    pub(super) fn test_pass_config() {
        let config = BashPassConfig::new("test_pass", BashPassPhase::Transformation);
        assert!(config.enabled);
        assert!(config.phase.is_modifying());
        assert_eq!(config.phase.name(), "transformation");
    }
    #[test]
    pub(super) fn test_pass_stats() {
        let mut stats = BashPassStats::new();
        stats.record_run(10, 100, 3);
        stats.record_run(20, 200, 5);
        assert_eq!(stats.total_runs, 2);
        assert!((stats.average_changes_per_run() - 15.0).abs() < 0.01);
        assert!((stats.success_rate() - 1.0).abs() < 0.01);
        let s = stats.format_summary();
        assert!(s.contains("Runs: 2/2"));
    }
    #[test]
    pub(super) fn test_pass_registry() {
        let mut reg = BashPassRegistry::new();
        reg.register(BashPassConfig::new("pass_a", BashPassPhase::Analysis));
        reg.register(BashPassConfig::new("pass_b", BashPassPhase::Transformation).disabled());
        assert_eq!(reg.total_passes(), 2);
        assert_eq!(reg.enabled_count(), 1);
        reg.update_stats("pass_a", 5, 50, 2);
        let stats = reg.get_stats("pass_a").expect("stats should exist");
        assert_eq!(stats.total_changes, 5);
    }
    #[test]
    pub(super) fn test_analysis_cache() {
        let mut cache = BashAnalysisCache::new(10);
        cache.insert("key1".to_string(), vec![1, 2, 3]);
        assert!(cache.get("key1").is_some());
        assert!(cache.get("key2").is_none());
        assert!((cache.hit_rate() - 0.5).abs() < 0.01);
        cache.invalidate("key1");
        assert!(!cache.entries["key1"].valid);
        assert_eq!(cache.size(), 1);
    }
    #[test]
    pub(super) fn test_worklist() {
        let mut wl = BashWorklist::new();
        assert!(wl.push(1));
        assert!(wl.push(2));
        assert!(!wl.push(1));
        assert_eq!(wl.len(), 2);
        assert_eq!(wl.pop(), Some(1));
        assert!(!wl.contains(1));
        assert!(wl.contains(2));
    }
    #[test]
    pub(super) fn test_dominator_tree() {
        let mut dt = BashDominatorTree::new(5);
        dt.set_idom(1, 0);
        dt.set_idom(2, 0);
        dt.set_idom(3, 1);
        assert!(dt.dominates(0, 3));
        assert!(dt.dominates(1, 3));
        assert!(!dt.dominates(2, 3));
        assert!(dt.dominates(3, 3));
    }
    #[test]
    pub(super) fn test_liveness() {
        let mut liveness = BashLivenessInfo::new(3);
        liveness.add_def(0, 1);
        liveness.add_use(1, 1);
        assert!(liveness.defs[0].contains(&1));
        assert!(liveness.uses[1].contains(&1));
    }
    #[test]
    pub(super) fn test_constant_folding() {
        assert_eq!(BashConstantFoldingHelper::fold_add_i64(3, 4), Some(7));
        assert_eq!(BashConstantFoldingHelper::fold_div_i64(10, 0), None);
        assert_eq!(BashConstantFoldingHelper::fold_div_i64(10, 2), Some(5));
        assert_eq!(
            BashConstantFoldingHelper::fold_bitand_i64(0b1100, 0b1010),
            0b1000
        );
        assert_eq!(BashConstantFoldingHelper::fold_bitnot_i64(0), -1);
    }
    #[test]
    pub(super) fn test_dep_graph() {
        let mut g = BashDepGraph::new();
        g.add_dep(1, 2);
        g.add_dep(2, 3);
        g.add_dep(1, 3);
        assert_eq!(g.dependencies_of(2), vec![1]);
        let topo = g.topological_sort();
        assert_eq!(topo.len(), 3);
        assert!(!g.has_cycle());
        let pos: std::collections::HashMap<u32, usize> =
            topo.iter().enumerate().map(|(i, &n)| (n, i)).collect();
        assert!(pos[&1] < pos[&2]);
        assert!(pos[&1] < pos[&3]);
        assert!(pos[&2] < pos[&3]);
    }
}
