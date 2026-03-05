//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

/// Log level for bash script logging.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BashLogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}
#[allow(dead_code)]
impl BashLogLevel {
    /// Returns the level name string.
    pub fn name(&self) -> &'static str {
        match self {
            BashLogLevel::Debug => "DEBUG",
            BashLogLevel::Info => "INFO",
            BashLogLevel::Warn => "WARN",
            BashLogLevel::Error => "ERROR",
            BashLogLevel::Fatal => "FATAL",
        }
    }
    /// Returns the ANSI color code for terminal output.
    pub fn ansi_color(&self) -> &'static str {
        match self {
            BashLogLevel::Debug => "\\033[0;36m",
            BashLogLevel::Info => "\\033[0;32m",
            BashLogLevel::Warn => "\\033[0;33m",
            BashLogLevel::Error => "\\033[0;31m",
            BashLogLevel::Fatal => "\\033[1;31m",
        }
    }
    /// Whether this level goes to stderr.
    pub fn is_stderr(&self) -> bool {
        matches!(self, BashLogLevel::Error | BashLogLevel::Fatal)
    }
}
/// Bash logging framework generator.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashLogger {
    /// Whether to include timestamps.
    pub timestamps: bool,
    /// Whether to use ANSI color codes.
    pub color: bool,
    /// Minimum log level to emit.
    pub min_level: BashLogLevel,
    /// Log file path (if None, only stdout/stderr).
    pub log_file: Option<std::string::String>,
}
#[allow(dead_code)]
impl BashLogger {
    /// Create a default logger (info level, no color, no timestamps).
    pub fn new() -> Self {
        BashLogger {
            timestamps: false,
            color: false,
            min_level: BashLogLevel::Info,
            log_file: None,
        }
    }
    /// Enable timestamps.
    pub fn with_timestamps(mut self) -> Self {
        self.timestamps = true;
        self
    }
    /// Enable ANSI color output.
    pub fn with_color(mut self) -> Self {
        self.color = true;
        self
    }
    /// Set minimum log level.
    pub fn with_min_level(mut self, level: BashLogLevel) -> Self {
        self.min_level = level;
        self
    }
    /// Set log file path.
    pub fn with_log_file(mut self, path: &str) -> Self {
        self.log_file = Some(path.to_string());
        self
    }
    /// Emit the logging framework functions.
    pub fn emit_framework(&self) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str("# ---- Logging framework ----\n");
        out.push_str(&format!(
            "readonly _LOG_LEVEL_MIN={}\n",
            self.min_level as u8
        ));
        out.push_str("readonly _LOG_DEBUG=0 _LOG_INFO=1 _LOG_WARN=2 _LOG_ERROR=3 _LOG_FATAL=4\n");
        out.push_str("_log() {\n");
        out.push_str("  local level=$1 level_name=$2 message=$3\n");
        out.push_str("  [[ $level -lt $_LOG_LEVEL_MIN ]] && return 0\n");
        if self.timestamps {
            out.push_str("  local ts; ts=$(date '+%Y-%m-%dT%H:%M:%S')\n");
        }
        let msg_prefix = if self.timestamps {
            "\"[$ts][$level_name] $message\""
        } else {
            "\"[$level_name] $message\""
        };
        if self.color {
            out.push_str("  local color reset='\\033[0m'\n");
            out.push_str("  case $level_name in\n");
            out.push_str("    DEBUG) color='\\033[0;36m' ;;\n");
            out.push_str("    INFO)  color='\\033[0;32m' ;;\n");
            out.push_str("    WARN)  color='\\033[0;33m' ;;\n");
            out.push_str("    ERROR|FATAL) color='\\033[0;31m' ;;\n");
            out.push_str("    *) color='' ;;\n");
            out.push_str("  esac\n");
            if self.timestamps {
                out.push_str("  local formatted=\"${color}[$ts][$level_name]${reset} $message\"\n");
            } else {
                out.push_str("  local formatted=\"${color}[$level_name]${reset} $message\"\n");
            }
            out.push_str("  if [[ $level -ge $_LOG_ERROR ]]; then\n");
            out.push_str("    echo -e \"$formatted\" >&2\n");
            out.push_str("  else\n");
            out.push_str("    echo -e \"$formatted\"\n");
            out.push_str("  fi\n");
        } else {
            out.push_str(&format!("  if [[ $level -ge $_LOG_ERROR ]]; then\n"));
            out.push_str(&format!("    echo {} >&2\n", msg_prefix));
            out.push_str("  else\n");
            out.push_str(&format!("    echo {}\n", msg_prefix));
            out.push_str("  fi\n");
        }
        if let Some(log_file) = &self.log_file {
            if self.timestamps {
                out.push_str(&format!(
                    "  echo \"[$ts][$level_name] $message\" >> \"{}\"\n",
                    log_file
                ));
            } else {
                out.push_str(&format!(
                    "  echo \"[$level_name] $message\" >> \"{}\"\n",
                    log_file
                ));
            }
        }
        out.push_str("}\n\n");
        for (fn_name, level_const, level_name) in &[
            ("log_debug", "_LOG_DEBUG", "DEBUG"),
            ("log_info", "_LOG_INFO", "INFO"),
            ("log_warn", "_LOG_WARN", "WARN"),
            ("log_error", "_LOG_ERROR", "ERROR"),
            ("log_fatal", "_LOG_FATAL", "FATAL"),
        ] {
            out.push_str(&format!(
                "{fn_name}() {{ _log ${level_const} \"{level_name}\" \"$1\"; }}\n"
            ));
        }
        out.push('\n');
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, BashCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl BashAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        BashAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&BashCacheEntry> {
        if self.entries.contains_key(key) {
            self.hits += 1;
            self.entries.get(key)
        } else {
            self.misses += 1;
            None
        }
    }
    #[allow(dead_code)]
    pub fn insert(&mut self, key: String, data: Vec<u8>) {
        if self.entries.len() >= self.max_size {
            if let Some(oldest) = self.entries.keys().next().cloned() {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(
            key.clone(),
            BashCacheEntry {
                key,
                data,
                timestamp: 0,
                valid: true,
            },
        );
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self, key: &str) {
        if let Some(entry) = self.entries.get_mut(key) {
            entry.valid = false;
        }
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            return 0.0;
        }
        self.hits as f64 / total as f64
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.entries.len()
    }
}
#[allow(dead_code)]
pub struct BashConstantFoldingHelper;
impl BashConstantFoldingHelper {
    #[allow(dead_code)]
    pub fn fold_add_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn fold_sub_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn fold_mul_i64(a: i64, b: i64) -> Option<i64> {
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn fold_div_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_add_f64(a: f64, b: f64) -> f64 {
        a + b
    }
    #[allow(dead_code)]
    pub fn fold_mul_f64(a: f64, b: f64) -> f64 {
        a * b
    }
    #[allow(dead_code)]
    pub fn fold_neg_i64(a: i64) -> Option<i64> {
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn fold_not_bool(a: bool) -> bool {
        !a
    }
    #[allow(dead_code)]
    pub fn fold_and_bool(a: bool, b: bool) -> bool {
        a && b
    }
    #[allow(dead_code)]
    pub fn fold_or_bool(a: bool, b: bool) -> bool {
        a || b
    }
    #[allow(dead_code)]
    pub fn fold_shl_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shl(b)
    }
    #[allow(dead_code)]
    pub fn fold_shr_i64(a: i64, b: u32) -> Option<i64> {
        a.checked_shr(b)
    }
    #[allow(dead_code)]
    pub fn fold_rem_i64(a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            None
        } else {
            Some(a % b)
        }
    }
    #[allow(dead_code)]
    pub fn fold_bitand_i64(a: i64, b: i64) -> i64 {
        a & b
    }
    #[allow(dead_code)]
    pub fn fold_bitor_i64(a: i64, b: i64) -> i64 {
        a | b
    }
    #[allow(dead_code)]
    pub fn fold_bitxor_i64(a: i64, b: i64) -> i64 {
        a ^ b
    }
    #[allow(dead_code)]
    pub fn fold_bitnot_i64(a: i64) -> i64 {
        !a
    }
}
/// A Bash here-document.
#[derive(Debug, Clone, PartialEq)]
pub struct BashHereDoc {
    /// Delimiter (e.g. `EOF`)
    pub delimiter: std::string::String,
    /// Whether to suppress leading tab indentation (`<<-EOF`)
    pub strip_tabs: bool,
    /// Whether to prevent parameter expansion (`'EOF'`)
    pub no_expand: bool,
    /// Content lines
    pub content: Vec<std::string::String>,
}
impl BashHereDoc {
    /// Create a simple here-document.
    pub fn new(
        delimiter: impl Into<std::string::String>,
        content: Vec<std::string::String>,
    ) -> Self {
        BashHereDoc {
            delimiter: delimiter.into(),
            strip_tabs: false,
            no_expand: false,
            content,
        }
    }
}
/// Bash variable kinds, classified by scope / export semantics.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BashVar {
    /// Regular local (function-scoped) variable: `local name`
    Local(std::string::String),
    /// Script-global variable: plain assignment at top level
    Global(std::string::String),
    /// Exported environment variable: `export NAME=...`
    Env(std::string::String),
    /// Read-only variable: `readonly NAME=...`
    Readonly(std::string::String),
    /// Integer variable: `declare -i NAME`
    Integer(std::string::String),
    /// Indexed array: `declare -a NAME`
    Array(std::string::String),
    /// Associative array: `declare -A NAME`
    AssocArray(std::string::String),
    /// Name-ref variable (Bash 4.3+): `declare -n NAME`
    NameRef(std::string::String),
}
impl BashVar {
    /// Get the variable name (without sigil).
    pub fn name(&self) -> &str {
        match self {
            BashVar::Local(n)
            | BashVar::Global(n)
            | BashVar::Env(n)
            | BashVar::Readonly(n)
            | BashVar::Integer(n)
            | BashVar::Array(n)
            | BashVar::AssocArray(n)
            | BashVar::NameRef(n) => n.as_str(),
        }
    }
    /// Whether this variable is exported to the environment.
    pub fn is_exported(&self) -> bool {
        matches!(self, BashVar::Env(_))
    }
    /// Whether this variable is read-only.
    pub fn is_readonly(&self) -> bool {
        matches!(self, BashVar::Readonly(_))
    }
}
/// A Bash `[[ ... ]]` conditional test.
#[derive(Debug, Clone, PartialEq)]
pub enum BashCondition {
    /// `[[ -e file ]]`
    FileExists(BashExpr),
    /// `[[ -f file ]]`
    IsFile(BashExpr),
    /// `[[ -d file ]]`
    IsDir(BashExpr),
    /// `[[ -n str ]]`
    NonEmpty(BashExpr),
    /// `[[ -z str ]]`
    Empty(BashExpr),
    /// `[[ a == b ]]`
    StrEq(BashExpr, BashExpr),
    /// `[[ a != b ]]`
    StrNe(BashExpr, BashExpr),
    /// `[[ a < b ]]` (lexicographic)
    StrLt(BashExpr, BashExpr),
    /// `(( a < b ))`
    ArithLt(std::string::String, std::string::String),
    /// `(( a == b ))`
    ArithEq(std::string::String, std::string::String),
    /// `[[ cond1 && cond2 ]]`
    And(Box<BashCondition>, Box<BashCondition>),
    /// `[[ cond1 || cond2 ]]`
    Or(Box<BashCondition>, Box<BashCondition>),
    /// `! cond`
    Not(Box<BashCondition>),
    /// Raw condition string (fallback)
    Raw(std::string::String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl BashDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        BashDepGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add_node(&mut self, id: u32) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
        }
    }
    #[allow(dead_code)]
    pub fn add_dep(&mut self, dep: u32, dependent: u32) {
        self.add_node(dep);
        self.add_node(dependent);
        self.edges.push((dep, dependent));
    }
    #[allow(dead_code)]
    pub fn dependents_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(d, _)| *d == node)
            .map(|(_, dep)| *dep)
            .collect()
    }
    #[allow(dead_code)]
    pub fn dependencies_of(&self, node: u32) -> Vec<u32> {
        self.edges
            .iter()
            .filter(|(_, dep)| *dep == node)
            .map(|(d, _)| *d)
            .collect()
    }
    #[allow(dead_code)]
    pub fn topological_sort(&self) -> Vec<u32> {
        let mut in_degree: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
        for &n in &self.nodes {
            in_degree.insert(n, 0);
        }
        for (_, dep) in &self.edges {
            *in_degree.entry(*dep).or_insert(0) += 1;
        }
        let mut queue: std::collections::VecDeque<u32> = self
            .nodes
            .iter()
            .filter(|&&n| in_degree[&n] == 0)
            .copied()
            .collect();
        let mut result = Vec::new();
        while let Some(node) = queue.pop_front() {
            result.push(node);
            for dep in self.dependents_of(node) {
                let cnt = in_degree.entry(dep).or_insert(0);
                *cnt = cnt.saturating_sub(1);
                if *cnt == 0 {
                    queue.push_back(dep);
                }
            }
        }
        result
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topological_sort().len() < self.nodes.len()
    }
}
/// Background job tracker for bash scripts.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashJobManager {
    /// Maximum number of parallel jobs.
    pub max_jobs: usize,
    /// Whether to wait for all jobs on exit.
    pub wait_on_exit: bool,
    /// PID array variable name.
    pub pids_var: std::string::String,
}
#[allow(dead_code)]
impl BashJobManager {
    /// Create a new job manager.
    pub fn new(max_jobs: usize) -> Self {
        BashJobManager {
            max_jobs,
            wait_on_exit: true,
            pids_var: "PIDS".to_string(),
        }
    }
    /// Set the PID array variable name.
    pub fn with_pids_var(mut self, var: &str) -> Self {
        self.pids_var = var.to_string();
        self
    }
    /// Emit the job management framework.
    pub fn emit_framework(&self) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str("# ---- Job manager ----\n");
        out.push_str(&format!("declare -a {}\n", self.pids_var));
        out.push_str(&format!("readonly _MAX_JOBS={}\n\n", self.max_jobs));
        out.push_str("_wait_jobs() {\n");
        out.push_str(&format!("  local -n _pids={}\n", self.pids_var));
        out.push_str("  local _failed=0\n");
        out.push_str("  for _pid in \"${_pids[@]:-}\"; do\n");
        out.push_str("    wait \"$_pid\" || (( _failed++ ))\n");
        out.push_str("  done\n");
        out.push_str(&format!("  {}=()\n", self.pids_var));
        out.push_str("  return $_failed\n");
        out.push_str("}\n\n");
        out.push_str("run_job() {\n");
        out.push_str("  local _cmd=\"$@\"\n");
        out.push_str(&format!("  local -n _pids={}\n", self.pids_var));
        out.push_str("  while [[ ${#_pids[@]} -ge $_MAX_JOBS ]]; do\n");
        out.push_str("    _wait_jobs || true\n");
        out.push_str("  done\n");
        out.push_str("  eval \"$_cmd\" &\n");
        out.push_str("  _pids+=(\"$!\")\n");
        out.push_str("}\n\n");
        if self.wait_on_exit {
            out.push_str("trap '_wait_jobs' EXIT\n\n");
        }
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl BashWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        BashWorklist {
            items: std::collections::VecDeque::new(),
            in_worklist: std::collections::HashSet::new(),
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, item: u32) -> bool {
        if self.in_worklist.insert(item) {
            self.items.push_back(item);
            true
        } else {
            false
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<u32> {
        let item = self.items.pop_front()?;
        self.in_worklist.remove(&item);
        Some(item)
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.items.len()
    }
    #[allow(dead_code)]
    pub fn contains(&self, item: u32) -> bool {
        self.in_worklist.contains(&item)
    }
}
/// A simple bash template with variable substitution.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashTemplate {
    /// Template text with `{{VAR}}` placeholders.
    pub template: std::string::String,
    /// Variable substitutions.
    pub vars: HashMap<std::string::String, std::string::String>,
}
#[allow(dead_code)]
impl BashTemplate {
    /// Create a new template.
    pub fn new(template: &str) -> Self {
        BashTemplate {
            template: template.to_string(),
            vars: HashMap::new(),
        }
    }
    /// Set a variable substitution.
    pub fn set(mut self, key: &str, value: &str) -> Self {
        self.vars.insert(key.to_string(), value.to_string());
        self
    }
    /// Render the template.
    pub fn render(&self) -> std::string::String {
        let mut result = self.template.clone();
        for (key, val) in &self.vars {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, val);
        }
        result
    }
    /// Emit bash code that performs the substitution at runtime.
    pub fn emit_bash_render(&self) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str("_render_template() {\n");
        out.push_str("  local _template=\"$1\"\n");
        for (key, _val) in &self.vars {
            out.push_str(&format!(
                "  _template=\"${{_template//\\\"{{{{{}}}}}\\\"/${{{}}}}}\"\n",
                key,
                key.to_uppercase()
            ));
        }
        out.push_str("  echo \"$_template\"\n");
        out.push_str("}\n");
        out
    }
}
/// A Bash statement (line or block).
#[derive(Debug, Clone, PartialEq)]
pub enum BashStatement {
    /// Variable assignment: `name=value`
    Assign(std::string::String, BashExpr),
    /// Local variable declaration: `local name=value`
    Local(std::string::String, Option<BashExpr>),
    /// Exported variable: `export NAME=value`
    Export(std::string::String, BashExpr),
    /// Readonly variable: `readonly NAME=value`
    Readonly(std::string::String, BashExpr),
    /// Declare with flags: `declare -flags name=value`
    Declare(std::string::String, std::string::String, Option<BashExpr>),
    /// Command invocation (raw line)
    Cmd(std::string::String),
    /// If/elif/else block
    If {
        cond: BashCondition,
        then: Vec<BashStatement>,
        elifs: Vec<(BashCondition, Vec<BashStatement>)>,
        else_: Option<Vec<BashStatement>>,
    },
    /// While loop
    While {
        cond: BashCondition,
        body: Vec<BashStatement>,
    },
    /// For-in loop: `for var in list; do ... done`
    For {
        var: std::string::String,
        in_: Vec<BashExpr>,
        body: Vec<BashStatement>,
    },
    /// C-style for loop: `for (( init; cond; incr ))`
    ForArith {
        init: std::string::String,
        cond: std::string::String,
        incr: std::string::String,
        body: Vec<BashStatement>,
    },
    /// `case` statement
    Case {
        expr: BashExpr,
        arms: Vec<(std::string::String, Vec<BashStatement>)>,
    },
    /// Function call
    Call(std::string::String, Vec<BashExpr>),
    /// Return statement
    Return(Option<u8>),
    /// `break`
    Break,
    /// `continue`
    Continue,
    /// `echo` output
    Echo(BashExpr),
    /// `printf` formatted output
    Printf(std::string::String, Vec<BashExpr>),
    /// `read` input
    Read(Vec<std::string::String>),
    /// `exit` with code
    Exit(u8),
    /// Raw statement string (fallback)
    Raw(std::string::String),
    /// Pipe: cmd1 | cmd2
    Pipe(Vec<std::string::String>),
    /// Trap: `trap 'handler' SIGNAL`
    Trap(std::string::String, std::string::String),
    /// Source file: `. file` or `source file`
    Source(std::string::String),
}
/// A bash heredoc block.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct BashHeredoc {
    /// The delimiter tag (e.g. "EOF", "SCRIPT").
    pub tag: std::string::String,
    /// Whether to use `<<-` (strip leading tabs) instead of `<<`.
    pub strip_tabs: bool,
    /// Whether to quote the tag (prevents expansion in body).
    pub quoted: bool,
    /// The heredoc body lines.
    pub lines: Vec<std::string::String>,
    /// Target file descriptor (default 1 = stdout).
    pub fd: Option<u8>,
}
#[allow(dead_code)]
impl BashHeredoc {
    /// Create a new heredoc with a given delimiter tag.
    pub fn new(tag: &str) -> Self {
        BashHeredoc {
            tag: tag.to_string(),
            strip_tabs: false,
            quoted: false,
            lines: vec![],
            fd: None,
        }
    }
    /// Use `<<-` to strip leading tabs.
    pub fn strip_tabs(mut self) -> Self {
        self.strip_tabs = true;
        self
    }
    /// Quote the delimiter (prevents variable expansion in body).
    pub fn quoted(mut self) -> Self {
        self.quoted = true;
        self
    }
    /// Redirect to a specific file descriptor.
    pub fn redirect_fd(mut self, fd: u8) -> Self {
        self.fd = Some(fd);
        self
    }
    /// Add a line to the heredoc body.
    pub fn line(mut self, s: &str) -> Self {
        self.lines.push(s.to_string());
        self
    }
    /// Emit the heredoc as a string.
    pub fn emit(&self) -> std::string::String {
        let arrow = if self.strip_tabs { "<<-" } else { "<<" };
        let tag = if self.quoted {
            format!("'{}'", self.tag)
        } else {
            self.tag.clone()
        };
        let fd_str = self.fd.map(|fd| format!("{}&", fd)).unwrap_or_default();
        let mut out = format!("{}{}{}\\n", fd_str, arrow, tag);
        for l in &self.lines {
            out.push_str(l);
            out.push_str("\\n");
        }
        out.push_str(&self.tag);
        out
    }
}
/// A generated bash argument parser.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashArgParser {
    /// Program name for usage display.
    pub prog_name: std::string::String,
    /// Options to parse.
    pub options: Vec<BashCliOption>,
    /// Positional argument names.
    pub positionals: Vec<std::string::String>,
    /// Usage description.
    pub description: std::string::String,
}
#[allow(dead_code)]
impl BashArgParser {
    /// Create a new argument parser.
    pub fn new(prog_name: &str, description: &str) -> Self {
        BashArgParser {
            prog_name: prog_name.to_string(),
            options: vec![],
            positionals: vec![],
            description: description.to_string(),
        }
    }
    /// Add an option.
    pub fn add_option(mut self, opt: BashCliOption) -> Self {
        self.options.push(opt);
        self
    }
    /// Add a positional argument name.
    pub fn add_positional(mut self, name: &str) -> Self {
        self.positionals.push(name.to_string());
        self
    }
    /// Emit the usage function.
    pub fn emit_usage(&self) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str("usage() {\n");
        out.push_str(&format!("  echo \"Usage: {} [OPTIONS]", self.prog_name));
        for pos in &self.positionals {
            out.push_str(&format!(" <{}>", pos));
        }
        out.push_str("\"\n");
        out.push_str(&format!("  echo \"{}\"\n", self.description));
        out.push_str("  echo \"\"\n");
        out.push_str("  echo \"Options:\"\n");
        out.push_str("  echo \"  -h, --help      Show this help message\"\n");
        for opt in &self.options {
            let short_str = opt
                .short
                .map(|c| format!("-{}, ", c))
                .unwrap_or_else(|| "    ".to_string());
            let arg_meta = if opt.has_arg { " <VALUE>" } else { "" };
            let req_str = if opt.required { " (required)" } else { "" };
            let def_str = opt
                .default
                .as_ref()
                .map(|d| format!(" [default: {}]", d))
                .unwrap_or_default();
            out.push_str(&format!(
                "  {}--{}{:<12}{}{}{}\n",
                short_str, opt.long, arg_meta, opt.help, req_str, def_str
            ));
        }
        out.push_str("}\n");
        out
    }
    /// Emit the full argument parsing block.
    pub fn emit_parse_block(&self) -> std::string::String {
        let mut out = std::string::String::new();
        for opt in &self.options {
            if let Some(default) = &opt.default {
                out.push_str(&format!("{}={}\n", opt.var_name.to_uppercase(), default));
            }
        }
        out.push('\n');
        out.push_str("while [[ $# -gt 0 ]]; do\n");
        out.push_str("  case \"$1\" in\n");
        out.push_str("    -h|--help) usage; exit 0 ;;\n");
        for opt in &self.options {
            let short_pat = opt.short.map(|c| format!("-{}|", c)).unwrap_or_default();
            if opt.has_arg {
                out.push_str(&format!(
                    "    {}--{})\n      {}=\"${{2:-}}\"\n      shift 2\n      ;;\n",
                    short_pat,
                    opt.long,
                    opt.var_name.to_uppercase()
                ));
            } else {
                out.push_str(&format!(
                    "    {}--{}) {}=true; shift ;;\n",
                    short_pat,
                    opt.long,
                    opt.var_name.to_uppercase()
                ));
            }
        }
        out.push_str("    --) shift; break ;;\n");
        out.push_str("    -*) echo \"Unknown option: $1\" >&2; usage; exit 1 ;;\n");
        out.push_str("    *) break ;;\n");
        out.push_str("  esac\n");
        out.push_str("done\n\n");
        for opt in &self.options {
            if opt.required {
                out.push_str(
                    &format!(
                        "if [[ -z \"${{{}:-}}\" ]]; then\n  echo \"Error: {} is required\" >&2\n  usage\n  exit 1\nfi\n",
                        opt.var_name.to_uppercase(), opt.var_name.to_uppercase()
                    ),
                );
            }
        }
        for (i, pos) in self.positionals.iter().enumerate() {
            out.push_str(&format!(
                "readonly {}=\"${{{}:-}}\"\n",
                pos.to_uppercase(),
                i + 1
            ));
        }
        out
    }
}
/// A command-line option for the generated argument parser.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashCliOption {
    /// Long option name (e.g. "verbose").
    pub long: std::string::String,
    /// Short option character (e.g. 'v'), if any.
    pub short: Option<char>,
    /// Variable name to store the value in.
    pub var_name: std::string::String,
    /// Default value (empty string means no default).
    pub default: Option<std::string::String>,
    /// Whether the option takes an argument.
    pub has_arg: bool,
    /// Help text.
    pub help: std::string::String,
    /// Whether the option is required.
    pub required: bool,
}
#[allow(dead_code)]
impl BashCliOption {
    /// Create a boolean flag (no argument).
    pub fn flag(long: &str, short: Option<char>, var_name: &str, help: &str) -> Self {
        BashCliOption {
            long: long.to_string(),
            short,
            var_name: var_name.to_string(),
            default: Some("false".to_string()),
            has_arg: false,
            help: help.to_string(),
            required: false,
        }
    }
    /// Create an option that takes an argument.
    pub fn arg(
        long: &str,
        short: Option<char>,
        var_name: &str,
        default: Option<&str>,
        help: &str,
    ) -> Self {
        BashCliOption {
            long: long.to_string(),
            short,
            var_name: var_name.to_string(),
            default: default.map(|s| s.to_string()),
            has_arg: true,
            help: help.to_string(),
            required: false,
        }
    }
    /// Mark this option as required.
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct BashPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl BashPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record_run(&mut self, changes: u64, time_ms: u64, iterations: u32) {
        self.total_runs += 1;
        self.successful_runs += 1;
        self.total_changes += changes;
        self.time_ms += time_ms;
        self.iterations_used = iterations;
    }
    #[allow(dead_code)]
    pub fn average_changes_per_run(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.total_changes as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            return 0.0;
        }
        self.successful_runs as f64 / self.total_runs as f64
    }
    #[allow(dead_code)]
    pub fn format_summary(&self) -> String {
        format!(
            "Runs: {}/{}, Changes: {}, Time: {}ms",
            self.successful_runs, self.total_runs, self.total_changes, self.time_ms
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl BashDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        BashDominatorTree {
            idom: vec![None; size],
            dom_children: vec![Vec::new(); size],
            dom_depth: vec![0; size],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, idom: u32) {
        self.idom[node] = Some(idom);
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, b: usize) -> bool {
        if a == b {
            return true;
        }
        let mut cur = b;
        loop {
            match self.idom[cur] {
                Some(parent) if parent as usize == a => return true,
                Some(parent) if parent as usize == cur => return false,
                Some(parent) => cur = parent as usize,
                None => return false,
            }
        }
    }
    #[allow(dead_code)]
    pub fn depth(&self, node: usize) -> u32 {
        self.dom_depth.get(node).copied().unwrap_or(0)
    }
}
/// Bash expression / expansion node.
#[derive(Debug, Clone, PartialEq)]
pub enum BashExpr {
    /// Variable expansion: `${name}` or `$name`
    Var(BashVar),
    /// String literal (single-quoted, no expansion): `'hello'`
    Lit(std::string::String),
    /// Double-quoted string (allows expansions): `"hello $world"`
    DQuoted(std::string::String),
    /// Command substitution: `$(cmd)`
    CmdSubst(std::string::String),
    /// Arithmetic expansion: `$(( expr ))`
    ArithExpr(std::string::String),
    /// Process substitution: `<(cmd)` or `>(cmd)`
    ProcSubst {
        is_input: bool,
        cmd: std::string::String,
    },
    /// Array element: `${arr[idx]}`
    ArrayElem(std::string::String, Box<BashExpr>),
    /// Array length: `${#arr[@]}`
    ArrayLen(std::string::String),
    /// All array elements: `"${arr[@]}"`
    ArrayAll(std::string::String),
    /// Associative array element: `${map[$key]}`
    AssocElem(std::string::String, Box<BashExpr>),
    /// Parameter expansion with default: `${var:-default}`
    Default(std::string::String, Box<BashExpr>),
    /// Parameter expansion with assign default: `${var:=default}`
    AssignDefault(std::string::String, Box<BashExpr>),
    /// Substring: `${var:offset:length}`
    Substring(std::string::String, usize, Option<usize>),
    /// String length: `${#var}`
    StringLen(std::string::String),
    /// Pattern removal (prefix): `${var#pattern}`
    StripPrefix(std::string::String, std::string::String),
    /// Pattern removal (suffix): `${var%pattern}`
    StripSuffix(std::string::String, std::string::String),
    /// Case conversion (Bash 4+): `${var^^}` / `${var,,}`
    UpperCase(std::string::String),
    LowerCase(std::string::String),
    /// Exit code of last command: `$?`
    LastStatus,
    /// PID of current shell: `$$`
    ShellPid,
    /// Script name: `$0`
    ScriptName,
    /// Positional argument: `$N`
    Positional(usize),
    /// All positional arguments: `"$@"`
    AllArgs,
    /// Number of positional arguments: `$#`
    ArgCount,
    /// Concatenation of two expressions
    Concat(Box<BashExpr>, Box<BashExpr>),
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum BashPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl BashPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            BashPassPhase::Analysis => "analysis",
            BashPassPhase::Transformation => "transformation",
            BashPassPhase::Verification => "verification",
            BashPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, BashPassPhase::Transformation | BashPassPhase::Cleanup)
    }
}
/// A complete Bash script.
#[derive(Debug, Clone)]
pub struct BashScript {
    /// Shebang line (e.g. `#!/usr/bin/env bash`)
    pub shebang: std::string::String,
    /// Initial set flags (e.g. `set -euo pipefail`)
    pub set_flags: Vec<std::string::String>,
    /// Trap handlers: `(signal, handler)`
    pub traps: Vec<(std::string::String, std::string::String)>,
    /// Top-level variable declarations
    pub globals: Vec<(std::string::String, std::string::String)>,
    /// Helper functions
    pub functions: Vec<BashFunction>,
    /// Main body statements (raw lines)
    pub main: Vec<std::string::String>,
}
impl BashScript {
    /// Create a new Bash script with a standard shebang.
    pub fn new() -> Self {
        BashScript {
            shebang: "#!/usr/bin/env bash".to_string(),
            set_flags: vec!["-euo".to_string(), "pipefail".to_string()],
            traps: vec![],
            globals: vec![],
            functions: vec![],
            main: vec![],
        }
    }
    /// Create a script with strict mode disabled.
    pub fn lenient() -> Self {
        BashScript {
            shebang: "#!/usr/bin/env bash".to_string(),
            set_flags: vec![],
            traps: vec![],
            globals: vec![],
            functions: vec![],
            main: vec![],
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashPassConfig {
    pub phase: BashPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl BashPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: BashPassPhase) -> Self {
        BashPassConfig {
            phase,
            enabled: true,
            max_iterations: 10,
            debug_output: false,
            pass_name: name.into(),
        }
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self) -> Self {
        self.debug_output = true;
        self
    }
    #[allow(dead_code)]
    pub fn max_iter(mut self, n: u32) -> Self {
        self.max_iterations = n;
        self
    }
}
#[allow(dead_code)]
pub struct BashPassRegistry {
    pub(super) configs: Vec<BashPassConfig>,
    pub(super) stats: std::collections::HashMap<String, BashPassStats>,
}
impl BashPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        BashPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: BashPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), BashPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&BashPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&BashPassStats> {
        self.stats.get(name)
    }
    #[allow(dead_code)]
    pub fn total_passes(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn enabled_count(&self) -> usize {
        self.enabled_passes().len()
    }
    #[allow(dead_code)]
    pub fn update_stats(&mut self, name: &str, changes: u64, time_ms: u64, iter: u32) {
        if let Some(stats) = self.stats.get_mut(name) {
            stats.record_run(changes, time_ms, iter);
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A Bash shell function.
#[derive(Debug, Clone, PartialEq)]
pub struct BashFunction {
    /// Function name
    pub name: std::string::String,
    /// Local variable declarations (names only — values set in body)
    pub local_vars: Vec<std::string::String>,
    /// Body lines (raw Bash statements)
    pub body: Vec<std::string::String>,
    /// Optional description (emitted as a comment before the function)
    pub description: Option<std::string::String>,
}
impl BashFunction {
    /// Create a new function with a name and body lines.
    pub fn new(name: impl Into<std::string::String>, body: Vec<std::string::String>) -> Self {
        BashFunction {
            name: name.into(),
            local_vars: vec![],
            body,
            description: None,
        }
    }
    /// Create a function with local variables.
    pub fn with_locals(
        name: impl Into<std::string::String>,
        local_vars: Vec<std::string::String>,
        body: Vec<std::string::String>,
    ) -> Self {
        BashFunction {
            name: name.into(),
            local_vars,
            body,
            description: None,
        }
    }
}
/// Bash script code generation backend for OxiLean.
pub struct BashBackend {
    /// Indent string (default: 4 spaces)
    pub(super) indent: std::string::String,
    /// Name mangling cache
    pub(super) mangle_cache: HashMap<std::string::String, std::string::String>,
    /// Whether to add blank lines between functions
    pub(super) spacing: bool,
}
impl BashBackend {
    /// Create a new BashBackend with default settings.
    pub fn new() -> Self {
        BashBackend {
            indent: "    ".to_string(),
            mangle_cache: HashMap::new(),
            spacing: true,
        }
    }
    /// Create a BashBackend with 2-space indentation.
    pub fn compact() -> Self {
        BashBackend {
            indent: "  ".to_string(),
            mangle_cache: HashMap::new(),
            spacing: false,
        }
    }
    /// Emit a BashVar as its declaration line (for `declare` statements).
    pub fn emit_var_decl(&self, var: &BashVar, value: Option<&str>) -> std::string::String {
        let val_s = value.map(|v| format!("={}", v)).unwrap_or_default();
        match var {
            BashVar::Local(n) => format!("local {}{}", n, val_s),
            BashVar::Global(n) => format!("{}{}", n, val_s),
            BashVar::Env(n) => format!("export {}{}", n, val_s),
            BashVar::Readonly(n) => format!("readonly {}{}", n, val_s),
            BashVar::Integer(n) => format!("declare -i {}{}", n, val_s),
            BashVar::Array(n) => format!("declare -a {}{}", n, val_s),
            BashVar::AssocArray(n) => format!("declare -A {}{}", n, val_s),
            BashVar::NameRef(n) => format!("declare -n {}{}", n, val_s),
        }
    }
    /// Emit a BashVar as an expansion (`${name}` form).
    pub fn emit_var(&self, var: &BashVar) -> std::string::String {
        format!("{}", var)
    }
    /// Emit a BashExpr as a string.
    pub fn emit_expr(&self, expr: &BashExpr) -> std::string::String {
        format!("{}", expr)
    }
    /// Emit a BashCondition as a string.
    pub fn emit_condition(&self, cond: &BashCondition) -> std::string::String {
        format!("{}", cond)
    }
    /// Mangle an OxiLean name into a valid Bash identifier.
    ///
    /// Bash function/variable names match `[a-zA-Z_][a-zA-Z0-9_]*`.
    /// Namespace separators (`.`, `::`) become `__`.
    pub fn mangle_name(&self, name: &str) -> std::string::String {
        if let Some(cached) = self.mangle_cache.get(name) {
            return cached.clone();
        }
        let mut result = std::string::String::new();
        let mut prev_special = false;
        for (i, c) in name.chars().enumerate() {
            match c {
                'a'..='z' | 'A'..='Z' | '_' => {
                    result.push(c);
                    prev_special = false;
                }
                '0'..='9' => {
                    if i == 0 {
                        result.push('_');
                    }
                    result.push(c);
                    prev_special = false;
                }
                '.' | ':' => {
                    if !prev_special {
                        result.push_str("__");
                    }
                    prev_special = true;
                }
                '\'' | '-' => {
                    if !prev_special {
                        result.push('_');
                    }
                    prev_special = true;
                }
                _ => {
                    if !prev_special {
                        result.push_str(&format!("_u{:04X}_", c as u32));
                    }
                    prev_special = true;
                }
            }
        }
        if result.is_empty() {
            result.push('_');
        }
        let builtins = [
            "alias",
            "bg",
            "bind",
            "break",
            "builtin",
            "caller",
            "cd",
            "command",
            "compgen",
            "complete",
            "compopt",
            "continue",
            "declare",
            "dirs",
            "disown",
            "echo",
            "enable",
            "eval",
            "exec",
            "exit",
            "export",
            "false",
            "fc",
            "fg",
            "getopts",
            "hash",
            "help",
            "history",
            "if",
            "jobs",
            "kill",
            "let",
            "local",
            "logout",
            "mapfile",
            "popd",
            "printf",
            "pushd",
            "pwd",
            "read",
            "readarray",
            "readonly",
            "return",
            "select",
            "set",
            "shift",
            "shopt",
            "source",
            "suspend",
            "test",
            "time",
            "times",
            "trap",
            "true",
            "type",
            "typeset",
            "ulimit",
            "umask",
            "unalias",
            "unset",
            "until",
            "wait",
            "while",
        ];
        if builtins.contains(&result.as_str()) {
            result.push_str("__ox");
        }
        result
    }
    /// Emit a BashFunction as a shell function definition.
    pub fn emit_function(&self, func: &BashFunction) -> std::string::String {
        let mut out = std::string::String::new();
        if let Some(desc) = &func.description {
            out.push_str(&format!("# {}\n", desc));
        }
        out.push_str(&format!("{}() {{\n", func.name));
        for local in &func.local_vars {
            out.push_str(&format!("{}local {}\n", self.indent, local));
        }
        if !func.local_vars.is_empty() && !func.body.is_empty() {
            out.push('\n');
        }
        for line in &func.body {
            if line.is_empty() {
                out.push('\n');
            } else {
                out.push_str(&format!("{}{}\n", self.indent, line));
            }
        }
        out.push_str("}\n");
        out
    }
    /// Emit a BashHereDoc as a string.
    pub fn emit_heredoc(&self, heredoc: &BashHereDoc) -> std::string::String {
        format!("{}", heredoc)
    }
    /// Emit a complete BashScript as a string.
    pub fn emit_script(&self, script: &BashScript) -> std::string::String {
        let mut out = std::string::String::new();
        out.push_str(&format!("{}\n", script.shebang));
        if !script.set_flags.is_empty() {
            out.push_str(&format!("set {}\n", script.set_flags.join(" ")));
        }
        out.push('\n');
        for (signal, handler) in &script.traps {
            out.push_str(&format!("trap '{}' {}\n", handler, signal));
        }
        if !script.traps.is_empty() {
            out.push('\n');
        }
        for (name, value) in &script.globals {
            out.push_str(&format!("readonly {}={}\n", name, value));
        }
        if !script.globals.is_empty() {
            out.push('\n');
        }
        for func in &script.functions {
            out.push_str(&self.emit_function(func));
            if self.spacing {
                out.push('\n');
            }
        }
        if !script.main.is_empty() {
            if !script.functions.is_empty() {
                out.push_str("# --- main ---\n");
            }
            for line in &script.main {
                out.push_str(line);
                out.push('\n');
            }
        }
        out
    }
    /// Emit an array assignment: `name=(elem1 elem2 ...)`
    pub fn emit_array_assign(&self, name: &str, elems: &[BashExpr]) -> std::string::String {
        let elems_s: Vec<std::string::String> = elems.iter().map(|e| format!("{}", e)).collect();
        format!("{}=({})", name, elems_s.join(" "))
    }
    /// Emit an associative array assignment block.
    pub fn emit_assoc_array_assign(
        &self,
        name: &str,
        pairs: &[(std::string::String, std::string::String)],
    ) -> std::string::String {
        let mut out = format!("declare -A {}\n", name);
        for (k, v) in pairs {
            out.push_str(&format!("{}[{}]={}\n", name, k, v));
        }
        out
    }
    /// Emit an if statement.
    pub fn emit_if(
        &self,
        cond: &BashCondition,
        then: &[&str],
        else_: Option<&[&str]>,
    ) -> std::string::String {
        let mut out = format!("if {}; then\n", cond);
        for line in then {
            out.push_str(&format!("{}{}\n", self.indent, line));
        }
        if let Some(else_body) = else_ {
            out.push_str("else\n");
            for line in else_body {
                out.push_str(&format!("{}{}\n", self.indent, line));
            }
        }
        out.push_str("fi\n");
        out
    }
    /// Emit a for-in loop.
    pub fn emit_for_in(&self, var: &str, items: &[BashExpr], body: &[&str]) -> std::string::String {
        let items_s: Vec<std::string::String> = items.iter().map(|e| format!("{}", e)).collect();
        let mut out = format!("for {} in {}; do\n", var, items_s.join(" "));
        for line in body {
            out.push_str(&format!("{}{}\n", self.indent, line));
        }
        out.push_str("done\n");
        out
    }
    /// Emit a while loop.
    pub fn emit_while(&self, cond: &BashCondition, body: &[&str]) -> std::string::String {
        let mut out = format!("while {}; do\n", cond);
        for line in body {
            out.push_str(&format!("{}{}\n", self.indent, line));
        }
        out.push_str("done\n");
        out
    }
    /// Emit a case statement.
    pub fn emit_case(&self, expr: &BashExpr, arms: &[(&str, Vec<&str>)]) -> std::string::String {
        let mut out = format!("case {} in\n", expr);
        for (pattern, body) in arms {
            out.push_str(&format!("{}{})\n", self.indent, pattern));
            for line in body {
                out.push_str(&format!("{}{}{}\n", self.indent, self.indent, line));
            }
            out.push_str(&format!("{};;\n", self.indent));
        }
        out.push_str("esac\n");
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct BashLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl BashLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        BashLivenessInfo {
            live_in: vec![std::collections::HashSet::new(); block_count],
            live_out: vec![std::collections::HashSet::new(); block_count],
            defs: vec![std::collections::HashSet::new(); block_count],
            uses: vec![std::collections::HashSet::new(); block_count],
        }
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, block: usize, var: u32) {
        if block < self.defs.len() {
            self.defs[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, block: usize, var: u32) {
        if block < self.uses.len() {
            self.uses[block].insert(var);
        }
    }
    #[allow(dead_code)]
    pub fn is_live_in(&self, block: usize, var: u32) -> bool {
        self.live_in
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn is_live_out(&self, block: usize, var: u32) -> bool {
        self.live_out
            .get(block)
            .map(|s| s.contains(&var))
            .unwrap_or(false)
    }
}
/// A signal trap specification.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct BashTrap {
    /// Signal name (EXIT, ERR, INT, TERM, HUP, etc.).
    pub signal: std::string::String,
    /// Handler command or function name.
    pub handler: std::string::String,
    /// Whether to reset to default (use `-` as handler).
    pub reset: bool,
}
#[allow(dead_code)]
impl BashTrap {
    /// Create a new trap for a signal.
    pub fn new(signal: &str, handler: &str) -> Self {
        BashTrap {
            signal: signal.to_string(),
            handler: handler.to_string(),
            reset: false,
        }
    }
    /// Create an EXIT trap.
    pub fn on_exit(handler: &str) -> Self {
        BashTrap::new("EXIT", handler)
    }
    /// Create an ERR trap.
    pub fn on_err(handler: &str) -> Self {
        BashTrap::new("ERR", handler)
    }
    /// Create an INT trap (Ctrl-C).
    pub fn on_int(handler: &str) -> Self {
        BashTrap::new("INT", handler)
    }
    /// Create a TERM trap.
    pub fn on_term(handler: &str) -> Self {
        BashTrap::new("TERM", handler)
    }
    /// Create a reset trap (ignore the signal).
    pub fn reset(signal: &str) -> Self {
        BashTrap {
            signal: signal.to_string(),
            handler: "-".to_string(),
            reset: true,
        }
    }
    /// Emit the trap statement.
    pub fn emit(&self) -> std::string::String {
        format!("trap '{}' {}", self.handler, self.signal)
    }
}
