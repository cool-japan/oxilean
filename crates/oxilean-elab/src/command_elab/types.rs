//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use oxilean_kernel::{BinderInfo, Expr, Level, Name};
use std::collections::{HashMap, HashSet};

/// The result of a command elaboration.
#[derive(Debug, Clone)]
pub struct CommandResult {
    /// Output messages (for #check, #eval, #print).
    pub messages: Vec<String>,
    /// New declarations produced (if any).
    pub declarations: Vec<CommandDecl>,
    /// Whether the command modified the state.
    pub state_modified: bool,
}
impl CommandResult {
    /// Create an empty (no-op) result.
    pub fn empty() -> Self {
        Self {
            messages: Vec::new(),
            declarations: Vec::new(),
            state_modified: false,
        }
    }
    /// Create a result that modified state.
    pub fn state_change() -> Self {
        Self {
            messages: Vec::new(),
            declarations: Vec::new(),
            state_modified: true,
        }
    }
    /// Create a result with a message.
    pub fn with_message(msg: String) -> Self {
        Self {
            messages: vec![msg],
            declarations: Vec::new(),
            state_modified: false,
        }
    }
    /// Create a result with declarations.
    pub fn with_decls(decls: Vec<CommandDecl>) -> Self {
        Self {
            messages: Vec::new(),
            declarations: decls,
            state_modified: true,
        }
    }
    /// Add a message.
    pub fn add_message(&mut self, msg: String) {
        self.messages.push(msg);
    }
    /// Add a declaration.
    pub fn add_decl(&mut self, decl: CommandDecl) {
        self.declarations.push(decl);
    }
}
/// State for a single open section.
#[derive(Debug, Clone)]
pub struct SectionState {
    /// Section name.
    pub name: String,
    /// Variables declared in this section: `(name, type, binder_info)`.
    pub vars: Vec<(Name, Expr, BinderInfo)>,
    /// Names of declarations that should include section variables.
    pub includes: Vec<Name>,
    /// Universe level parameters declared in this section.
    pub level_params: Vec<Name>,
    /// Names declared within this section.
    pub decl_names: Vec<Name>,
}
impl SectionState {
    /// Create a new section state.
    pub fn new(name: String) -> Self {
        Self {
            name,
            vars: Vec::new(),
            includes: Vec::new(),
            level_params: Vec::new(),
            decl_names: Vec::new(),
        }
    }
    /// Add a variable to the section.
    pub fn add_variable(&mut self, name: Name, ty: Expr, binder_info: BinderInfo) {
        self.vars.push((name, ty, binder_info));
    }
    /// Add a universe level parameter.
    pub fn add_level_param(&mut self, name: Name) {
        self.level_params.push(name);
    }
    /// Record a declaration name.
    pub fn record_decl(&mut self, name: Name) {
        self.decl_names.push(name);
    }
    /// Get all variable names.
    pub fn var_names(&self) -> Vec<&Name> {
        self.vars.iter().map(|(n, _, _)| n).collect()
    }
    /// Look up a variable by name.
    pub fn lookup_var(&self, name: &Name) -> Option<&(Name, Expr, BinderInfo)> {
        self.vars.iter().find(|(n, _, _)| n == name)
    }
    /// Check if a variable exists.
    pub fn has_var(&self, name: &Name) -> bool {
        self.vars.iter().any(|(n, _, _)| n == name)
    }
    /// Number of variables.
    pub fn num_vars(&self) -> usize {
        self.vars.len()
    }
    /// Number of universe parameters.
    pub fn num_level_params(&self) -> usize {
        self.level_params.len()
    }
}
/// Central state for command elaboration.
///
/// Tracks the current namespace, open namespaces, section variables,
/// universe variables, and configuration options.
#[derive(Debug, Clone)]
pub struct CommandState {
    /// Current namespace (dot-separated).
    pub current_namespace: Vec<String>,
    /// Open namespaces (for unqualified name access).
    pub open_namespaces: Vec<Vec<String>>,
    /// Stack of open sections.
    pub section_stack: Vec<SectionState>,
    /// Global universe variables.
    pub universe_vars: Vec<Name>,
    /// Configuration options.
    pub options: HashMap<String, OptionValue>,
    /// Attributes applied to the next declaration.
    pub pending_attributes: Vec<String>,
    /// All declared names (for #print lookups).
    pub declared_names: HashMap<Name, DeclInfo>,
}
impl CommandState {
    /// Create a new command state.
    pub fn new() -> Self {
        Self {
            current_namespace: Vec::new(),
            open_namespaces: Vec::new(),
            section_stack: Vec::new(),
            universe_vars: Vec::new(),
            options: Self::default_options(),
            pending_attributes: Vec::new(),
            declared_names: HashMap::new(),
        }
    }
    /// Default configuration options.
    fn default_options() -> HashMap<String, OptionValue> {
        let mut opts = HashMap::new();
        opts.insert("pp.all".to_string(), OptionValue::Bool(false));
        opts.insert("pp.universes".to_string(), OptionValue::Bool(false));
        opts.insert("pp.implicit".to_string(), OptionValue::Bool(false));
        opts.insert("pp.notation".to_string(), OptionValue::Bool(true));
        opts.insert("pp.maxDepth".to_string(), OptionValue::Nat(100));
        opts.insert("maxHeartbeats".to_string(), OptionValue::Nat(200000));
        opts.insert("maxRecDepth".to_string(), OptionValue::Nat(1000));
        opts
    }
    /// Get the fully qualified current namespace as a dot-separated string.
    pub fn namespace_str(&self) -> String {
        self.current_namespace.join(".")
    }
    /// Qualify a name with the current namespace.
    pub fn qualify_name(&self, name: &str) -> Name {
        if self.current_namespace.is_empty() {
            Name::str(name)
        } else {
            Name::str(format!("{}.{}", self.namespace_str(), name))
        }
    }
    /// Check if there is an open section.
    pub fn has_open_section(&self) -> bool {
        !self.section_stack.is_empty()
    }
    /// Get the current section (innermost), if any.
    pub fn current_section(&self) -> Option<&SectionState> {
        self.section_stack.last()
    }
    /// Get the current section mutably.
    pub fn current_section_mut(&mut self) -> Option<&mut SectionState> {
        self.section_stack.last_mut()
    }
    /// Get all section variables in scope (from all nested sections).
    pub fn all_section_vars(&self) -> Vec<&(Name, Expr, BinderInfo)> {
        self.section_stack
            .iter()
            .flat_map(|s| s.vars.iter())
            .collect()
    }
    /// Check if a name is a section variable.
    pub fn is_section_var(&self, name: &Name) -> bool {
        self.section_stack.iter().any(|s| s.has_var(name))
    }
    /// Look up a section variable.
    pub fn lookup_section_var(&self, name: &Name) -> Option<&(Name, Expr, BinderInfo)> {
        for section in self.section_stack.iter().rev() {
            if let Some(v) = section.lookup_var(name) {
                return Some(v);
            }
        }
        None
    }
    /// Check if a namespace is currently open.
    pub fn is_namespace_open(&self, ns: &[String]) -> bool {
        self.open_namespaces.iter().any(|open| open == ns)
    }
    /// Get an option value.
    pub fn get_option(&self, name: &str) -> Option<&OptionValue> {
        self.options.get(name)
    }
    /// Register a declaration.
    pub fn register_decl(&mut self, name: Name, info: DeclInfo) {
        self.declared_names.insert(name.clone(), info);
        if let Some(section) = self.current_section_mut() {
            section.record_decl(name);
        }
    }
    /// Look up a declared name.
    pub fn lookup_decl(&self, name: &Name) -> Option<&DeclInfo> {
        self.declared_names.get(name)
    }
    /// Get the section nesting depth.
    pub fn section_depth(&self) -> usize {
        self.section_stack.len()
    }
    /// Get the namespace depth.
    pub fn namespace_depth(&self) -> usize {
        self.current_namespace.len()
    }
    /// Check if a universe variable is declared.
    pub fn is_universe_var(&self, name: &Name) -> bool {
        if self.universe_vars.contains(name) {
            return true;
        }
        self.section_stack
            .iter()
            .any(|s| s.level_params.contains(name))
    }
    /// Collect all universe variables in scope.
    pub fn all_universe_vars(&self) -> Vec<Name> {
        let mut result = self.universe_vars.clone();
        for section in &self.section_stack {
            for lp in &section.level_params {
                if !result.contains(lp) {
                    result.push(lp.clone());
                }
            }
        }
        result
    }
}
/// A history of commands processed in a session.
#[derive(Clone, Debug, Default)]
pub struct CommandHistory {
    entries: Vec<CommandHistoryEntry>,
}
impl CommandHistory {
    /// Create an empty history.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a command result.
    pub fn record(&mut self, entry: CommandHistoryEntry) {
        self.entries.push(entry);
    }
    /// Number of commands recorded.
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    /// Return `true` if no commands have been recorded.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    /// Number of successful commands.
    pub fn successes(&self) -> usize {
        self.entries.iter().filter(|e| e.success).count()
    }
    /// Number of failed commands.
    pub fn failures(&self) -> usize {
        self.entries.iter().filter(|e| !e.success).count()
    }
    /// Success rate in [0, 1].
    pub fn success_rate(&self) -> f64 {
        if self.entries.is_empty() {
            1.0
        } else {
            self.successes() as f64 / self.entries.len() as f64
        }
    }
    /// Return all failed entries.
    pub fn failed_entries(&self) -> Vec<&CommandHistoryEntry> {
        self.entries.iter().filter(|e| !e.success).collect()
    }
}
/// Stages of the command elaboration pipeline.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandStage {
    /// Parse → surface AST.
    Parse,
    /// Resolve names and check scope.
    Resolve,
    /// Elaborate surface AST to kernel term.
    Elaborate,
    /// Type-check kernel term.
    TypeCheck,
    /// Add to environment.
    AddToEnv,
    /// Post-processing (e.g. attribute dispatch).
    PostProcess,
    /// Done.
    Done,
}
/// A declaration produced by a command.
#[derive(Debug, Clone)]
pub struct CommandDecl {
    /// Name of the declaration.
    pub name: Name,
    /// Type of the declaration.
    pub ty: Expr,
    /// Value/body (if any).
    pub val: Option<Expr>,
    /// Whether this is a universe declaration.
    pub is_universe: bool,
}
/// Errors that can occur during command elaboration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandError {
    /// Section name mismatch on `end`.
    SectionMismatch {
        /// Expected section name.
        expected: String,
        /// Actual name given.
        got: String,
    },
    /// No section is currently open.
    NoOpenSection,
    /// Namespace not found.
    NamespaceNotFound(String),
    /// Duplicate variable declaration.
    DuplicateVariable(String),
    /// Duplicate universe variable.
    DuplicateUniverse(String),
    /// Unknown option.
    UnknownOption(String),
    /// Invalid option value.
    InvalidOptionValue {
        /// Option name.
        option: String,
        /// The invalid value.
        value: String,
    },
    /// Name not found (for #print).
    NameNotFound(String),
    /// General elaboration error.
    ElabError(String),
}
/// A log entry for tracking commands processed by a `CommandState`.
#[derive(Clone, Debug)]
pub struct CommandHistoryEntry {
    /// Name of the processed declaration.
    pub decl_name: Name,
    /// Stage the command completed at.
    pub final_stage: CommandStage,
    /// Whether processing succeeded.
    pub success: bool,
}
impl CommandHistoryEntry {
    /// Create an entry for a successful processing.
    pub fn success(decl_name: Name) -> Self {
        Self {
            decl_name,
            final_stage: CommandStage::Done,
            success: true,
        }
    }
    /// Create an entry for a failed processing.
    pub fn failure(decl_name: Name, stage: CommandStage) -> Self {
        Self {
            decl_name,
            final_stage: stage,
            success: false,
        }
    }
}
/// Represents a universe level inference constraint.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnivConstraint {
    /// `l1 <= l2`
    LeqLevel(Level, Level),
    /// `l1 == l2`
    EqLevel(Level, Level),
    /// `l is a parameter`
    IsParam(Level),
}
/// Tracks the progress of a single command through the elaboration pipeline.
#[derive(Debug, Clone)]
pub struct CommandPipeline {
    /// The command being processed.
    pub decl_name: Name,
    /// Current stage.
    pub stage: CommandStage,
    /// Error if the pipeline aborted.
    pub error: Option<CommandError>,
    /// Warnings accumulated.
    pub warnings: Vec<String>,
}
impl CommandPipeline {
    /// Create a new pipeline starting at `Parse`.
    pub fn new(decl_name: Name) -> Self {
        Self {
            decl_name,
            stage: CommandStage::Parse,
            error: None,
            warnings: Vec::new(),
        }
    }
    /// Advance to the next stage.
    pub fn advance(&mut self) {
        self.stage = match self.stage {
            CommandStage::Parse => CommandStage::Resolve,
            CommandStage::Resolve => CommandStage::Elaborate,
            CommandStage::Elaborate => CommandStage::TypeCheck,
            CommandStage::TypeCheck => CommandStage::AddToEnv,
            CommandStage::AddToEnv => CommandStage::PostProcess,
            CommandStage::PostProcess => CommandStage::Done,
            CommandStage::Done => CommandStage::Done,
        };
    }
    /// Abort the pipeline with the given error.
    pub fn abort(&mut self, err: CommandError) {
        self.error = Some(err);
        self.stage = CommandStage::Done;
    }
    /// Add a warning.
    pub fn warn(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }
    /// Return `true` if the pipeline has completed (successfully or with error).
    pub fn is_done(&self) -> bool {
        self.stage == CommandStage::Done
    }
    /// Return `true` if the pipeline completed successfully.
    pub fn is_success(&self) -> bool {
        self.is_done() && self.error.is_none()
    }
}
/// A configuration option value.
#[derive(Debug, Clone, PartialEq)]
pub enum OptionValue {
    /// Boolean option.
    Bool(bool),
    /// Numeric option.
    Nat(u64),
    /// String option.
    Str(String),
}
/// Information about a declared name (for #print).
#[derive(Debug, Clone)]
pub struct DeclInfo {
    /// Type of the declaration.
    pub ty: Expr,
    /// Value/body (if any).
    pub val: Option<Expr>,
    /// The namespace this was declared in.
    pub namespace: Vec<String>,
    /// Whether this is a theorem.
    pub is_theorem: bool,
    /// Universe parameters.
    pub univ_params: Vec<Name>,
}
/// Statistics about command elaboration throughput.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct CommandElabThroughput {
    /// Total commands processed.
    pub total: usize,
    /// Commands that succeeded.
    pub succeeded: usize,
    /// Commands that failed.
    pub failed: usize,
    /// Total time spent in microseconds.
    pub total_us: u64,
}
#[allow(dead_code)]
impl CommandElabThroughput {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a command result.
    pub fn record(&mut self, ok: bool, duration_us: u64) {
        self.total += 1;
        if ok {
            self.succeeded += 1;
        } else {
            self.failed += 1;
        }
        self.total_us += duration_us;
    }
    /// Return the success rate in [0, 1].
    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            return 1.0;
        }
        self.succeeded as f64 / self.total as f64
    }
    /// Return the average time per command in microseconds.
    pub fn avg_us(&self) -> f64 {
        if self.total == 0 {
            return 0.0;
        }
        self.total_us as f64 / self.total as f64
    }
    /// Merge another throughput record into this one.
    pub fn merge(&mut self, other: &CommandElabThroughput) {
        self.total += other.total;
        self.succeeded += other.succeeded;
        self.failed += other.failed;
        self.total_us += other.total_us;
    }
    /// Return a human-readable summary string.
    pub fn summary(&self) -> String {
        format!(
            "total={} ok={} fail={} success_rate={:.1}% avg_us={:.1}",
            self.total,
            self.succeeded,
            self.failed,
            self.success_rate() * 100.0,
            self.avg_us(),
        )
    }
}
/// The result of a declaration validation pass.
#[derive(Clone, Debug)]
pub struct ValidationResult {
    /// Name of the declaration validated.
    pub decl_name: Name,
    /// Errors found during validation.
    pub errors: Vec<CommandError>,
    /// Warnings found during validation.
    pub warnings: Vec<String>,
    /// Whether the declaration passed all checks.
    pub passed: bool,
}
impl ValidationResult {
    /// Create a passing result with no errors or warnings.
    pub fn ok(decl_name: Name) -> Self {
        Self {
            decl_name,
            errors: Vec::new(),
            warnings: Vec::new(),
            passed: true,
        }
    }
    /// Create a failing result with a single error.
    pub fn err(decl_name: Name, error: CommandError) -> Self {
        Self {
            decl_name,
            errors: vec![error],
            warnings: Vec::new(),
            passed: false,
        }
    }
    /// Add an error to the result and mark it as failed.
    pub fn add_error(&mut self, error: CommandError) {
        self.errors.push(error);
        self.passed = false;
    }
    /// Add a warning (does not affect `passed`).
    pub fn add_warning(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }
    /// Return `true` if there are no errors.
    pub fn is_ok(&self) -> bool {
        self.errors.is_empty()
    }
    /// Number of diagnostics (errors + warnings).
    pub fn num_diagnostics(&self) -> usize {
        self.errors.len() + self.warnings.len()
    }
}
/// A set of universe level constraints to be solved.
#[derive(Clone, Debug, Default)]
pub struct UnivConstraintSet {
    constraints: Vec<UnivConstraint>,
}
impl UnivConstraintSet {
    /// Create an empty constraint set.
    pub fn new() -> Self {
        Self::default()
    }
    /// Add a constraint.
    pub fn add(&mut self, c: UnivConstraint) {
        self.constraints.push(c);
    }
    /// Return the number of constraints.
    pub fn len(&self) -> usize {
        self.constraints.len()
    }
    /// Return `true` if no constraints have been added.
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
    /// Return all constraints of kind `IsParam`.
    pub fn params(&self) -> Vec<&Level> {
        self.constraints
            .iter()
            .filter_map(|c| {
                if let UnivConstraint::IsParam(l) = c {
                    Some(l)
                } else {
                    None
                }
            })
            .collect()
    }
    /// Remove duplicate constraints.
    pub fn deduplicate(&mut self) {
        self.constraints.dedup();
    }
    /// Return all constraints as a slice.
    pub fn as_slice(&self) -> &[UnivConstraint] {
        &self.constraints
    }
}
