//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_parse::parse_source_file;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

/// Built-in variables that can be referenced in scripts.
#[derive(Clone, Debug)]
pub struct ScriptVariables {
    /// All variables.
    vars: HashMap<String, String>,
}
impl ScriptVariables {
    /// Create a new set of script variables.
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
    /// Create variables pre-populated with standard build info.
    pub fn with_build_info(
        package_name: &str,
        package_version: &str,
        profile: &str,
        target_dir: &str,
    ) -> Self {
        let mut vars = HashMap::new();
        vars.insert("OXILEAN_PKG_NAME".to_string(), package_name.to_string());
        vars.insert(
            "OXILEAN_PKG_VERSION".to_string(),
            package_version.to_string(),
        );
        vars.insert("OXILEAN_PROFILE".to_string(), profile.to_string());
        vars.insert("OXILEAN_TARGET_DIR".to_string(), target_dir.to_string());
        vars.insert("OXILEAN_BUILD_TIMESTAMP".to_string(), "0".to_string());
        Self { vars }
    }
    /// Set a variable.
    pub fn set(&mut self, key: &str, value: &str) {
        self.vars.insert(key.to_string(), value.to_string());
    }
    /// Get a variable.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.vars.get(key).map(|s| s.as_str())
    }
    /// Check if a variable is set.
    pub fn contains(&self, key: &str) -> bool {
        self.vars.contains_key(key)
    }
    /// Expand variables in a template string.
    ///
    /// Variables are referenced as `${VAR_NAME}`.
    pub fn expand(&self, template: &str) -> String {
        let mut result = template.to_string();
        for (key, value) in &self.vars {
            let pattern = format!("${{{}}}", key);
            result = result.replace(&pattern, value);
        }
        result
    }
    /// Get all variables as a HashMap.
    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.vars
    }
    /// Merge another set of variables (other takes priority).
    pub fn merge(&mut self, other: &ScriptVariables) {
        for (k, v) in &other.vars {
            self.vars.insert(k.clone(), v.clone());
        }
    }
    /// Get the number of variables.
    pub fn count(&self) -> usize {
        self.vars.len()
    }
}
/// Options for a protobuf code-generation step.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ProtobufCompileOptions {
    /// Directories to search for `.proto` files.
    pub include_dirs: Vec<PathBuf>,
    /// The `.proto` source files to compile.
    pub proto_files: Vec<PathBuf>,
    /// Output directory for generated code.
    pub out_dir: PathBuf,
    /// Whether to generate gRPC service stubs.
    pub grpc: bool,
    /// Additional plugin paths.
    pub plugins: Vec<PathBuf>,
    /// Extra `protoc` flags.
    pub extra_flags: Vec<String>,
}
#[allow(dead_code)]
impl ProtobufCompileOptions {
    /// Create default protobuf compile options.
    pub fn new(out_dir: impl Into<PathBuf>) -> Self {
        Self {
            include_dirs: Vec::new(),
            proto_files: Vec::new(),
            out_dir: out_dir.into(),
            grpc: false,
            plugins: Vec::new(),
            extra_flags: Vec::new(),
        }
    }
    /// Add an include directory.
    pub fn include(mut self, dir: impl Into<PathBuf>) -> Self {
        self.include_dirs.push(dir.into());
        self
    }
    /// Add a proto source file.
    pub fn proto(mut self, file: impl Into<PathBuf>) -> Self {
        self.proto_files.push(file.into());
        self
    }
    /// Enable gRPC stub generation.
    pub fn with_grpc(mut self) -> Self {
        self.grpc = true;
        self
    }
    /// Add a plugin path.
    pub fn plugin(mut self, path: impl Into<PathBuf>) -> Self {
        self.plugins.push(path.into());
        self
    }
    /// Add an extra flag.
    pub fn flag(mut self, f: &str) -> Self {
        self.extra_flags.push(f.to_string());
        self
    }
    /// Convert to a `ScriptDef` that invokes `protoc`.
    pub fn to_script_def(&self, name: &str) -> ScriptDef {
        let mut cmd_parts = vec!["protoc".to_string()];
        for d in &self.include_dirs {
            cmd_parts.push(format!("-I{}", d.display()));
        }
        cmd_parts.push(format!("--rust_out={}", self.out_dir.display()));
        if self.grpc {
            cmd_parts.push(format!("--grpc_out={}", self.out_dir.display()));
        }
        for f in &self.extra_flags {
            cmd_parts.push(f.clone());
        }
        for proto in &self.proto_files {
            cmd_parts.push(proto.to_string_lossy().into_owned());
        }
        let cmd = cmd_parts.join(" ");
        ScriptDef::new(name, ScriptKind::CodeGen, ScriptSource::Inline(cmd))
    }
    /// Number of proto files to compile.
    pub fn proto_count(&self) -> usize {
        self.proto_files.len()
    }
}
/// A condition that determines whether a script should run.
#[derive(Clone, Debug)]
pub enum ScriptCondition {
    /// Always run.
    Always,
    /// Run only if a file exists.
    FileExists(PathBuf),
    /// Run only if a file does NOT exist.
    FileMissing(PathBuf),
    /// Run only if a variable is set.
    VarSet(String),
    /// Run only if a variable equals a specific value.
    VarEquals(String, String),
    /// Run only if the profile matches.
    ProfileIs(String),
    /// Combine conditions with AND.
    AllOf(Vec<ScriptCondition>),
    /// Combine conditions with OR.
    AnyOf(Vec<ScriptCondition>),
    /// Negate a condition.
    Not(Box<ScriptCondition>),
}
impl ScriptCondition {
    /// Evaluate the condition against the given variables and profile.
    pub fn evaluate(&self, vars: &ScriptVariables, profile: &str) -> bool {
        match self {
            Self::Always => true,
            Self::FileExists(path) => path.exists(),
            Self::FileMissing(path) => !path.exists(),
            Self::VarSet(name) => vars.contains(name),
            Self::VarEquals(name, expected) => {
                vars.get(name).map(|v| v == expected).unwrap_or(false)
            }
            Self::ProfileIs(expected) => profile == expected,
            Self::AllOf(conditions) => conditions.iter().all(|c| c.evaluate(vars, profile)),
            Self::AnyOf(conditions) => conditions.iter().any(|c| c.evaluate(vars, profile)),
            Self::Not(inner) => !inner.evaluate(vars, profile),
        }
    }
}
/// Result of a cached script execution.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CachedScriptResult {
    /// The cache key that produced this result.
    pub key: ScriptCacheKey,
    /// The script result.
    pub result: ScriptResult,
    /// When this entry was created.
    pub created_at_secs: u64,
}
/// Executes build scripts.
pub struct ScriptRunner {
    /// Registered scripts.
    scripts: Vec<ScriptDef>,
    /// Global environment variables.
    global_env: HashMap<String, String>,
    /// Default timeout.
    default_timeout: Duration,
    /// Results from executed scripts.
    results: Vec<ScriptResult>,
}
impl ScriptRunner {
    /// Create a new script runner.
    pub fn new() -> Self {
        Self {
            scripts: Vec::new(),
            global_env: HashMap::new(),
            default_timeout: Duration::from_secs(300),
            results: Vec::new(),
        }
    }
    /// Register a script.
    pub fn add_script(&mut self, script: ScriptDef) {
        self.scripts.push(script);
    }
    /// Set a global environment variable.
    pub fn set_global_env(&mut self, key: &str, value: &str) {
        self.global_env.insert(key.to_string(), value.to_string());
    }
    /// Set the default timeout.
    pub fn set_default_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }
    /// Get all scripts of a given kind.
    pub fn scripts_of_kind(&self, kind: &ScriptKind) -> Vec<&ScriptDef> {
        self.scripts.iter().filter(|s| &s.kind == kind).collect()
    }
    /// Run all scripts of a given kind.
    pub fn run_scripts(&mut self, kind: &ScriptKind) -> Result<Vec<ScriptResult>, ScriptError> {
        let scripts: Vec<ScriptDef> = self
            .scripts
            .iter()
            .filter(|s| &s.kind == kind)
            .cloned()
            .collect();
        let mut results = Vec::new();
        for script in &scripts {
            let result = self.run_single(script)?;
            if !result.success && script.fail_on_error {
                results.push(result.clone());
                self.results.push(result);
                return Err(ScriptError::ExecutionFailed {
                    name: script.name.clone(),
                    exit_code: results.last().and_then(|r| r.exit_code),
                    stderr: results.last().map(|r| r.stderr.clone()).unwrap_or_default(),
                });
            }
            self.results.push(result.clone());
            results.push(result);
        }
        Ok(results)
    }
    fn run_single(&self, script: &ScriptDef) -> Result<ScriptResult, ScriptError> {
        let mut env: HashMap<String, String> = self.global_env.clone();
        for (k, v) in &script.env {
            env.insert(k.clone(), v.clone());
        }
        let start = Instant::now();
        match &script.source {
            ScriptSource::Inline(content) => {
                let shell_result = self.run_shell_command(content, &env, &script.working_dir);
                let duration = start.elapsed();
                Ok(self.make_script_result(
                    &script.name,
                    shell_result,
                    duration,
                    script.fail_on_error,
                ))
            }
            ScriptSource::File(path) => {
                if path.as_os_str().is_empty() {
                    return Err(ScriptError::NotFound(path.clone()));
                }
                if !path.exists() {
                    return Err(ScriptError::NotFound(path.clone()));
                }
                let source = fs::read_to_string(path).map_err(|e| {
                    ScriptError::IoError(format!(
                        "cannot read script file {}: {}",
                        path.display(),
                        e
                    ))
                })?;
                let shell_result = self.run_shell_command(&source, &env, &script.working_dir);
                let duration = start.elapsed();
                Ok(self.make_script_result(
                    &script.name,
                    shell_result,
                    duration,
                    script.fail_on_error,
                ))
            }
            ScriptSource::OxiLeanExpr(expr) => {
                let mut errors: Vec<String> = Vec::new();
                let count = parse_source_file(expr, Some(&mut errors)).unwrap_or(0);
                let duration = start.elapsed();
                if errors.is_empty() || count > 0 {
                    let mut result = ScriptResult::success(&script.name, duration);
                    result.stdout = format!(
                        "evaluated OxiLean expression ({} token(s))",
                        expr.split_whitespace().count()
                    );
                    Ok(result)
                } else {
                    let stderr = errors.join("\n");
                    if script.fail_on_error {
                        Ok(ScriptResult::failure(&script.name, 1, &stderr, duration))
                    } else {
                        let mut result = ScriptResult::success(&script.name, duration);
                        result.stderr = stderr;
                        Ok(result)
                    }
                }
            }
        }
    }
    /// Run a shell command string using `sh -c`.
    ///
    /// Returns `(exit_code, stdout, stderr)`.
    fn run_shell_command(
        &self,
        content: &str,
        env: &HashMap<String, String>,
        working_dir: &Option<std::path::PathBuf>,
    ) -> (i32, String, String) {
        let mut cmd = std::process::Command::new("sh");
        cmd.arg("-c").arg(content);
        cmd.stdin(std::process::Stdio::null());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
        for (k, v) in env {
            cmd.env(k, v);
        }
        if let Some(dir) = working_dir {
            if dir.exists() {
                cmd.current_dir(dir);
            }
        }
        match cmd.output() {
            Ok(output) => {
                let exit_code = output.status.code().unwrap_or(1);
                let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
                let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
                (exit_code, stdout, stderr)
            }
            Err(e) => (1, String::new(), format!("could not spawn shell: {}", e)),
        }
    }
    /// Convert a raw shell execution result into a `ScriptResult`.
    fn make_script_result(
        &self,
        name: &str,
        result: (i32, String, String),
        duration: Duration,
        fail_on_error: bool,
    ) -> ScriptResult {
        let (exit_code, stdout, stderr) = result;
        let success = exit_code == 0;
        if success || !fail_on_error {
            ScriptResult {
                name: name.to_string(),
                success: success || !fail_on_error,
                exit_code: Some(exit_code),
                stdout,
                stderr,
                duration,
                generated_files: Vec::new(),
            }
        } else {
            ScriptResult::failure(name, exit_code, &stderr, duration)
        }
    }
    /// Run pre-build hooks.
    pub fn run_pre_build(&mut self) -> Result<Vec<ScriptResult>, ScriptError> {
        self.run_scripts(&ScriptKind::PreBuild)
    }
    /// Run post-build hooks.
    pub fn run_post_build(&mut self) -> Result<Vec<ScriptResult>, ScriptError> {
        self.run_scripts(&ScriptKind::PostBuild)
    }
    /// Run code generation scripts.
    pub fn run_codegen(&mut self) -> Result<Vec<ScriptResult>, ScriptError> {
        self.run_scripts(&ScriptKind::CodeGen)
    }
    /// Run clean scripts.
    pub fn run_clean(&mut self) -> Result<Vec<ScriptResult>, ScriptError> {
        self.run_scripts(&ScriptKind::Clean)
    }
    /// Get all results from previous runs.
    pub fn results(&self) -> &[ScriptResult] {
        &self.results
    }
    /// Clear results.
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
    /// Get the total number of registered scripts.
    pub fn script_count(&self) -> usize {
        self.scripts.len()
    }
}
impl ScriptRunner {
    /// Run a single script (public wrapper for pipeline use).
    pub fn run_single_public(&self, script: &ScriptDef) -> Result<ScriptResult, ScriptError> {
        self.run_single(script)
    }
    /// Run pre-test hooks.
    pub fn run_pre_test(&mut self) -> Result<Vec<ScriptResult>, ScriptError> {
        self.run_scripts(&ScriptKind::PreTest)
    }
    /// Run post-test hooks.
    pub fn run_post_test(&mut self) -> Result<Vec<ScriptResult>, ScriptError> {
        self.run_scripts(&ScriptKind::PostTest)
    }
    /// Run pre-publish hooks.
    pub fn run_pre_publish(&mut self) -> Result<Vec<ScriptResult>, ScriptError> {
        self.run_scripts(&ScriptKind::PrePublish)
    }
    /// Run post-publish hooks.
    pub fn run_post_publish(&mut self) -> Result<Vec<ScriptResult>, ScriptError> {
        self.run_scripts(&ScriptKind::PostPublish)
    }
    /// Run all scripts, regardless of kind.
    pub fn run_all(&mut self) -> Vec<ScriptResult> {
        let kinds = vec![
            ScriptKind::PreBuild,
            ScriptKind::CodeGen,
            ScriptKind::PostBuild,
            ScriptKind::PreTest,
            ScriptKind::PostTest,
        ];
        let mut all_results = Vec::new();
        for kind in &kinds {
            if let Ok(results) = self.run_scripts(kind) {
                all_results.extend(results);
            }
        }
        all_results
    }
    /// Get a script by name.
    pub fn get_script(&self, name: &str) -> Option<&ScriptDef> {
        self.scripts.iter().find(|s| s.name == name)
    }
    /// Remove a script by name.
    pub fn remove_script(&mut self, name: &str) -> bool {
        let len_before = self.scripts.len();
        self.scripts.retain(|s| s.name != name);
        self.scripts.len() < len_before
    }
}
/// The kind of build artifact.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArtifactKind {
    /// A compiled executable.
    Binary,
    /// A static library.
    StaticLib,
    /// A shared library.
    SharedLib,
    /// An OxiLean module file.
    Module,
    /// A header file.
    Header,
    /// A documentation file.
    Doc,
    /// An arbitrary data file.
    Data,
}
/// Configuration for the environment in which scripts are executed.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ScriptEnvironment {
    /// Base environment variables inherited from the process.
    base_vars: HashMap<String, String>,
    /// Additional overrides.
    overrides: HashMap<String, String>,
    /// Variables to unset.
    unset: Vec<String>,
    /// Current working directory for the environment.
    working_dir: Option<PathBuf>,
    /// PATH additions (prepended to PATH).
    path_prepend: Vec<PathBuf>,
    /// PATH additions (appended to PATH).
    path_append: Vec<PathBuf>,
}
#[allow(dead_code)]
impl ScriptEnvironment {
    /// Create a minimal environment (no inherited vars).
    pub fn minimal() -> Self {
        Self {
            base_vars: HashMap::new(),
            overrides: HashMap::new(),
            unset: Vec::new(),
            working_dir: None,
            path_prepend: Vec::new(),
            path_append: Vec::new(),
        }
    }
    /// Create an environment inheriting from the current process.
    pub fn inherit() -> Self {
        let base_vars: HashMap<String, String> = std::env::vars().collect();
        Self {
            base_vars,
            overrides: HashMap::new(),
            unset: Vec::new(),
            working_dir: None,
            path_prepend: Vec::new(),
            path_append: Vec::new(),
        }
    }
    /// Set an override variable.
    pub fn set(&mut self, key: &str, value: &str) {
        self.overrides.insert(key.to_string(), value.to_string());
    }
    /// Unset a variable.
    pub fn unset_var(&mut self, key: &str) {
        self.unset.push(key.to_string());
        self.overrides.remove(key);
    }
    /// Set the working directory.
    pub fn with_working_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.working_dir = Some(dir.into());
        self
    }
    /// Prepend a directory to PATH.
    pub fn prepend_path(mut self, dir: impl Into<PathBuf>) -> Self {
        self.path_prepend.push(dir.into());
        self
    }
    /// Append a directory to PATH.
    pub fn append_path(mut self, dir: impl Into<PathBuf>) -> Self {
        self.path_append.push(dir.into());
        self
    }
    /// Resolve the final environment map.
    pub fn resolve(&self) -> HashMap<String, String> {
        let mut env = self.base_vars.clone();
        for key in &self.unset {
            env.remove(key);
        }
        env.extend(self.overrides.clone());
        let sep = if cfg!(windows) { ";" } else { ":" };
        let existing_path = env.get("PATH").cloned().unwrap_or_default();
        let prepend: Vec<String> = self
            .path_prepend
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect();
        let append: Vec<String> = self
            .path_append
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect();
        let mut path_parts: Vec<String> = prepend;
        if !existing_path.is_empty() {
            path_parts.push(existing_path);
        }
        path_parts.extend(append);
        let new_path = path_parts.join(sep);
        if !new_path.is_empty() {
            env.insert("PATH".to_string(), new_path);
        }
        env
    }
    /// Number of resolved variables.
    pub fn var_count(&self) -> usize {
        self.resolve().len()
    }
    /// Whether a key is defined (after resolution).
    pub fn contains_key(&self, key: &str) -> bool {
        self.resolve().contains_key(key)
    }
}
/// A handle representing a script queued for parallel execution.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ParallelScriptHandle {
    /// Script name.
    pub name: String,
    /// Simulated future result.
    pub result: Option<ScriptResult>,
}
#[allow(dead_code)]
impl ParallelScriptHandle {
    /// Create a handle (not yet resolved).
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            result: None,
        }
    }
    /// Resolve the handle with a result.
    pub fn resolve(mut self, result: ScriptResult) -> Self {
        self.result = Some(result);
        self
    }
    /// Whether the handle has been resolved.
    pub fn is_resolved(&self) -> bool {
        self.result.is_some()
    }
}
/// Aggregates profiling data for all scripts in a session.
#[allow(dead_code)]
pub struct ScriptProfiler {
    profiles: HashMap<String, ScriptProfile>,
}
#[allow(dead_code)]
impl ScriptProfiler {
    /// Create a new profiler.
    pub fn new() -> Self {
        Self {
            profiles: HashMap::new(),
        }
    }
    /// Record a script run.
    pub fn record(&mut self, name: &str, duration: Duration, cached: bool) {
        let profile = self
            .profiles
            .entry(name.to_string())
            .or_insert_with(|| ScriptProfile::new(name));
        profile.record(duration, cached);
    }
    /// Get the profile for a script by name.
    pub fn get(&self, name: &str) -> Option<&ScriptProfile> {
        self.profiles.get(name)
    }
    /// All profiles sorted by total duration (descending — hottest first).
    pub fn hottest_scripts(&self) -> Vec<&ScriptProfile> {
        let mut profiles: Vec<_> = self.profiles.values().collect();
        profiles.sort_by(|a, b| b.total_duration.cmp(&a.total_duration));
        profiles
    }
    /// Total wall-clock time across all scripts.
    pub fn total_time(&self) -> Duration {
        self.profiles.values().map(|p| p.total_duration).sum()
    }
    /// Number of tracked scripts.
    pub fn script_count(&self) -> usize {
        self.profiles.len()
    }
    /// Total run count across all scripts.
    pub fn total_run_count(&self) -> u64 {
        self.profiles.values().map(|p| p.run_count).sum()
    }
}
/// Strategy for handling script errors.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorHandlingStrategy {
    /// Abort the entire build immediately.
    AbortBuild,
    /// Skip this script and continue.
    Skip,
    /// Retry with the given policy.
    Retry(u32),
    /// Emit a warning but continue.
    WarnAndContinue,
}
/// A script definition with a run condition.
#[derive(Clone, Debug)]
pub struct ConditionalScript {
    /// The script to run.
    pub script: ScriptDef,
    /// The condition under which to run it.
    pub condition: ScriptCondition,
}
impl ConditionalScript {
    /// Create a new conditional script.
    pub fn new(script: ScriptDef, condition: ScriptCondition) -> Self {
        Self { script, condition }
    }
    /// Check if the script should run.
    pub fn should_run(&self, vars: &ScriptVariables, profile: &str) -> bool {
        self.condition.evaluate(vars, profile)
    }
}
/// A build script specialised for cross-compilation.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CrossCompileScript {
    /// The base script definition.
    pub base: ScriptDef,
    /// The target triple.
    pub target: TargetTriple,
    /// Sysroot path for the target toolchain.
    pub sysroot: Option<PathBuf>,
    /// Cross-compiler binary (e.g. "aarch64-linux-gnu-gcc").
    pub cc: Option<String>,
    /// Cross-archiver binary.
    pub ar: Option<String>,
    /// Additional environment variables set for the target.
    pub target_env: HashMap<String, String>,
}
#[allow(dead_code)]
impl CrossCompileScript {
    /// Create a new cross-compile script.
    pub fn new(base: ScriptDef, target: TargetTriple) -> Self {
        Self {
            base,
            target,
            sysroot: None,
            cc: None,
            ar: None,
            target_env: HashMap::new(),
        }
    }
    /// Set the sysroot.
    pub fn with_sysroot(mut self, path: impl Into<PathBuf>) -> Self {
        self.sysroot = Some(path.into());
        self
    }
    /// Set the cross-compiler.
    pub fn with_cc(mut self, cc: &str) -> Self {
        self.cc = Some(cc.to_string());
        self
    }
    /// Set the cross-archiver.
    pub fn with_ar(mut self, ar: &str) -> Self {
        self.ar = Some(ar.to_string());
        self
    }
    /// Add a target-specific environment variable.
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.target_env.insert(key.to_string(), value.to_string());
        self
    }
    /// Build the complete environment for running the script.
    pub fn build_env(&self) -> HashMap<String, String> {
        let mut env = self.base.env.clone();
        env.insert("TARGET".to_string(), self.target.triple.clone());
        if let Some(ref sysroot) = self.sysroot {
            env.insert(
                "SYSROOT".to_string(),
                sysroot.to_string_lossy().into_owned(),
            );
        }
        if let Some(ref cc) = self.cc {
            env.insert("CC".to_string(), cc.clone());
        }
        if let Some(ref ar) = self.ar {
            env.insert("AR".to_string(), ar.clone());
        }
        env.extend(self.target_env.clone());
        env
    }
    /// Whether this is a native build (target == host).
    pub fn is_native(&self) -> bool {
        self.target == TargetTriple::host()
    }
}
/// A build script definition.
#[derive(Clone, Debug)]
pub struct ScriptDef {
    /// Script name.
    pub name: String,
    /// Script kind.
    pub kind: ScriptKind,
    /// The script content or path.
    pub source: ScriptSource,
    /// Environment variables for the script.
    pub env: HashMap<String, String>,
    /// Working directory for the script.
    pub working_dir: Option<PathBuf>,
    /// Maximum execution time.
    pub timeout: Option<Duration>,
    /// Whether the build should fail if the script fails.
    pub fail_on_error: bool,
    /// Description of what the script does.
    pub description: Option<String>,
    /// Input files (for caching / invalidation).
    pub inputs: Vec<PathBuf>,
    /// Output files (for caching / invalidation).
    pub outputs: Vec<PathBuf>,
}
impl ScriptDef {
    /// Create a new script definition.
    pub fn new(name: &str, kind: ScriptKind, source: ScriptSource) -> Self {
        Self {
            name: name.to_string(),
            kind,
            source,
            env: HashMap::new(),
            working_dir: None,
            timeout: None,
            fail_on_error: true,
            description: None,
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
    /// Create an inline pre-build script.
    pub fn pre_build(name: &str, content: &str) -> Self {
        Self::new(
            name,
            ScriptKind::PreBuild,
            ScriptSource::Inline(content.to_string()),
        )
    }
    /// Create an inline post-build script.
    pub fn post_build(name: &str, content: &str) -> Self {
        Self::new(
            name,
            ScriptKind::PostBuild,
            ScriptSource::Inline(content.to_string()),
        )
    }
    /// Create a code generation script.
    pub fn codegen(name: &str, file_path: &Path) -> Self {
        Self::new(
            name,
            ScriptKind::CodeGen,
            ScriptSource::File(file_path.to_path_buf()),
        )
    }
    /// Set an environment variable.
    pub fn set_env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }
    /// Set the working directory.
    pub fn set_working_dir(mut self, dir: &Path) -> Self {
        self.working_dir = Some(dir.to_path_buf());
        self
    }
    /// Set the timeout.
    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    /// Set whether to fail on error.
    pub fn set_fail_on_error(mut self, fail: bool) -> Self {
        self.fail_on_error = fail;
        self
    }
    /// Set the description.
    pub fn set_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }
    /// Add an input file.
    pub fn add_input(mut self, path: &Path) -> Self {
        self.inputs.push(path.to_path_buf());
        self
    }
    /// Add an output file.
    pub fn add_output(mut self, path: &Path) -> Self {
        self.outputs.push(path.to_path_buf());
        self
    }
}
/// Manages build hooks for a project.
pub struct HookManager {
    /// All registered hooks.
    hooks: Vec<ScriptDef>,
    /// Whether hooks are enabled.
    enabled: bool,
}
impl HookManager {
    /// Create a new hook manager.
    pub fn new() -> Self {
        Self {
            hooks: Vec::new(),
            enabled: true,
        }
    }
    /// Register a hook.
    pub fn register(&mut self, hook: ScriptDef) {
        self.hooks.push(hook);
    }
    /// Enable or disable all hooks.
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
    /// Check if hooks are enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    /// Get hooks of a specific kind.
    pub fn hooks_of_kind(&self, kind: &ScriptKind) -> Vec<&ScriptDef> {
        if !self.enabled {
            return Vec::new();
        }
        self.hooks.iter().filter(|h| &h.kind == kind).collect()
    }
    /// Get the total number of registered hooks.
    pub fn hook_count(&self) -> usize {
        self.hooks.len()
    }
    /// Remove all hooks of a specific kind.
    pub fn remove_hooks_of_kind(&mut self, kind: &ScriptKind) {
        self.hooks.retain(|h| &h.kind != kind);
    }
    /// Clear all hooks.
    pub fn clear(&mut self) {
        self.hooks.clear();
    }
}
/// Describes a build artifact to be packaged.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct BuildArtifact {
    /// Artifact name.
    pub name: String,
    /// Source path of the artifact.
    pub source_path: PathBuf,
    /// Destination path within the package.
    pub dest_path: PathBuf,
    /// Artifact kind.
    pub kind: ArtifactKind,
    /// Whether to strip debug symbols.
    pub strip: bool,
}
#[allow(dead_code)]
impl BuildArtifact {
    /// Create a new artifact descriptor.
    pub fn new(
        name: &str,
        source_path: impl Into<PathBuf>,
        dest_path: impl Into<PathBuf>,
        kind: ArtifactKind,
    ) -> Self {
        Self {
            name: name.to_string(),
            source_path: source_path.into(),
            dest_path: dest_path.into(),
            kind,
            strip: false,
        }
    }
    /// Enable debug-symbol stripping.
    pub fn with_strip(mut self) -> Self {
        self.strip = true;
        self
    }
    /// Whether the source file exists on disk.
    pub fn source_exists(&self) -> bool {
        self.source_path.exists()
    }
}
/// Error type for script execution.
#[derive(Clone, Debug)]
pub enum ScriptError {
    /// Script file not found.
    NotFound(PathBuf),
    /// Script execution failed.
    ExecutionFailed {
        /// Script name.
        name: String,
        /// Exit code.
        exit_code: Option<i32>,
        /// Error output.
        stderr: String,
    },
    /// Script timed out.
    Timeout {
        /// Script name.
        name: String,
        /// Timeout duration.
        timeout: Duration,
    },
    /// Invalid script source.
    InvalidSource(String),
    /// IO error.
    IoError(String),
}
/// A pipeline of scripts that run in sequence.
#[derive(Clone, Debug)]
pub struct ScriptPipeline {
    /// Pipeline name.
    pub name: String,
    /// Steps in the pipeline (run in order).
    pub steps: Vec<PipelineStep>,
    /// Whether to stop on first failure.
    pub fail_fast: bool,
}
impl ScriptPipeline {
    /// Create a new pipeline.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            steps: Vec::new(),
            fail_fast: true,
        }
    }
    /// Add a step to the pipeline.
    pub fn add_step(&mut self, step: PipelineStep) {
        self.steps.push(step);
    }
    /// Add a simple script as a pipeline step.
    pub fn add_script(&mut self, name: &str, script: ScriptDef) {
        self.steps.push(PipelineStep {
            name: name.to_string(),
            script,
            continue_on_failure: false,
            retry_count: 0,
            retry_delay: Duration::from_secs(0),
        });
    }
    /// Get the number of steps.
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
    /// Execute the pipeline using a script runner.
    pub fn execute(&self, runner: &ScriptRunner) -> Vec<ScriptResult> {
        let mut results = Vec::new();
        for step in &self.steps {
            let result = runner.run_single_public(&step.script);
            let success = match &result {
                Ok(r) => r.success,
                Err(_) => false,
            };
            match result {
                Ok(r) => results.push(r),
                Err(_e) => {
                    results.push(ScriptResult::failure(
                        &step.name,
                        1,
                        "execution error",
                        Duration::from_millis(0),
                    ));
                }
            }
            if !success && self.fail_fast && !step.continue_on_failure {
                break;
            }
        }
        results
    }
}
/// The kind of build script.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScriptKind {
    /// A pre-build hook (runs before build).
    PreBuild,
    /// A post-build hook (runs after build).
    PostBuild,
    /// A code generation step.
    CodeGen,
    /// A custom build step.
    Custom(String),
    /// A pre-test hook.
    PreTest,
    /// A post-test hook.
    PostTest,
    /// A pre-publish hook.
    PrePublish,
    /// A post-publish hook.
    PostPublish,
    /// A clean hook.
    Clean,
}
/// Target triple for cross-compilation.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TargetTriple {
    /// Full triple string, e.g. "aarch64-unknown-linux-gnu".
    pub triple: String,
}
#[allow(dead_code)]
impl TargetTriple {
    /// Create a new target triple.
    pub fn new(triple: &str) -> Self {
        Self {
            triple: triple.to_string(),
        }
    }
    /// The host triple (current machine).
    pub fn host() -> Self {
        Self::new("x86_64-unknown-linux-gnu")
    }
    /// Architecture component.
    pub fn arch(&self) -> &str {
        self.triple.split('-').next().unwrap_or("")
    }
    /// Vendor component.
    pub fn vendor(&self) -> Option<&str> {
        let parts: Vec<&str> = self.triple.splitn(4, '-').collect();
        parts.get(1).copied()
    }
    /// OS component.
    pub fn os(&self) -> Option<&str> {
        let parts: Vec<&str> = self.triple.splitn(4, '-').collect();
        parts.get(2).copied()
    }
    /// Environment/ABI component.
    pub fn env(&self) -> Option<&str> {
        let parts: Vec<&str> = self.triple.splitn(4, '-').collect();
        parts.get(3).copied()
    }
    /// Whether this is a Windows target.
    pub fn is_windows(&self) -> bool {
        self.os() == Some("windows") || self.triple.contains("windows")
    }
    /// Whether this is an Apple target.
    pub fn is_apple(&self) -> bool {
        self.triple.contains("apple")
    }
    /// Whether this is a Wasm target.
    pub fn is_wasm(&self) -> bool {
        self.triple.contains("wasm")
    }
}
/// A simple in-memory script cache.
#[allow(dead_code)]
pub struct ScriptCache {
    entries: HashMap<String, CachedScriptResult>,
    /// Maximum number of entries to keep.
    max_entries: usize,
}
#[allow(dead_code)]
impl ScriptCache {
    /// Create a new script cache.
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_entries: max_entries.max(1),
        }
    }
    /// Store a result in the cache.
    pub fn store(&mut self, key: ScriptCacheKey, result: ScriptResult) {
        if self.entries.len() >= self.max_entries {
            if let Some(oldest) = self
                .entries
                .values()
                .min_by_key(|e| e.created_at_secs)
                .map(|e| e.key.name.clone())
            {
                self.entries.remove(&oldest);
            }
        }
        self.entries.insert(
            key.name.clone(),
            CachedScriptResult {
                key,
                result,
                created_at_secs: 0,
            },
        );
    }
    /// Look up a cached result by key name.
    pub fn get(&self, name: &str) -> Option<&CachedScriptResult> {
        self.entries.get(name)
    }
    /// Check if a result is cached and the key matches.
    pub fn is_valid(&self, key: &ScriptCacheKey) -> bool {
        self.entries
            .get(&key.name)
            .map(|e| &e.key == key)
            .unwrap_or(false)
    }
    /// Remove an entry.
    pub fn invalidate(&mut self, name: &str) -> bool {
        self.entries.remove(name).is_some()
    }
    /// Clear all cached entries.
    pub fn clear(&mut self) {
        self.entries.clear();
    }
    /// Number of cached entries.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
/// A code generation step definition.
#[derive(Clone, Debug)]
pub struct CodeGenStep {
    /// Step name.
    pub name: String,
    /// Input template or specification file.
    pub input: PathBuf,
    /// Output directory for generated files.
    pub output_dir: PathBuf,
    /// Generator kind.
    pub generator: GeneratorKind,
    /// Options for the generator.
    pub options: HashMap<String, String>,
}
impl CodeGenStep {
    /// Create a new code generation step.
    pub fn new(name: &str, input: &Path, output_dir: &Path, generator: GeneratorKind) -> Self {
        Self {
            name: name.to_string(),
            input: input.to_path_buf(),
            output_dir: output_dir.to_path_buf(),
            generator,
            options: HashMap::new(),
        }
    }
    /// Set an option.
    pub fn set_option(mut self, key: &str, value: &str) -> Self {
        self.options.insert(key.to_string(), value.to_string());
        self
    }
    /// Convert to a ScriptDef.
    pub fn to_script_def(&self) -> ScriptDef {
        let source = ScriptSource::File(self.input.clone());
        ScriptDef::new(&self.name, ScriptKind::CodeGen, source).add_input(&self.input)
    }
}
/// Policy for retrying failed scripts.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct RetryPolicy {
    /// Maximum number of attempts (1 = no retry).
    pub max_attempts: u32,
    /// Delay between attempts.
    pub delay: Duration,
    /// Whether to use exponential back-off.
    pub exponential_backoff: bool,
    /// Maximum delay cap for exponential back-off.
    pub max_delay: Duration,
}
#[allow(dead_code)]
impl RetryPolicy {
    /// No retries.
    pub fn no_retry() -> Self {
        Self {
            max_attempts: 1,
            delay: Duration::ZERO,
            exponential_backoff: false,
            max_delay: Duration::ZERO,
        }
    }
    /// Simple fixed-delay retry.
    pub fn fixed(max_attempts: u32, delay: Duration) -> Self {
        Self {
            max_attempts,
            delay,
            exponential_backoff: false,
            max_delay: delay * max_attempts,
        }
    }
    /// Exponential back-off retry.
    pub fn exponential(max_attempts: u32, initial_delay: Duration) -> Self {
        Self {
            max_attempts,
            delay: initial_delay,
            exponential_backoff: true,
            max_delay: Duration::from_secs(60),
        }
    }
    /// Compute the delay before the nth attempt (0-indexed).
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if !self.exponential_backoff || attempt == 0 {
            self.delay
        } else {
            let factor = 2u64.pow(attempt.min(10));
            let d = Duration::from_micros(self.delay.as_micros() as u64 * factor);
            d.min(self.max_delay)
        }
    }
    /// Whether another attempt is allowed after `attempts_made` attempts.
    pub fn should_retry(&self, attempts_made: u32) -> bool {
        attempts_made < self.max_attempts
    }
}
/// Summary report for script execution.
#[derive(Clone, Debug)]
pub struct ScriptReport {
    /// Total scripts run.
    pub total: usize,
    /// Successful scripts.
    pub succeeded: usize,
    /// Failed scripts.
    pub failed: usize,
    /// Total duration.
    pub total_duration: Duration,
    /// Individual results.
    pub results: Vec<ScriptResult>,
}
impl ScriptReport {
    /// Create a report from a list of results.
    pub fn from_results(results: Vec<ScriptResult>) -> Self {
        let total = results.len();
        let succeeded = results.iter().filter(|r| r.success).count();
        let failed = total - succeeded;
        let total_duration: Duration = results.iter().map(|r| r.duration).sum();
        Self {
            total,
            succeeded,
            failed,
            total_duration,
            results,
        }
    }
    /// Check if all scripts succeeded.
    pub fn all_succeeded(&self) -> bool {
        self.failed == 0
    }
    /// Format a summary string.
    pub fn summary(&self) -> String {
        format!(
            "Scripts: {} total, {} succeeded, {} failed ({:.2}s)",
            self.total,
            self.succeeded,
            self.failed,
            self.total_duration.as_secs_f64()
        )
    }
    /// Get all failed script names.
    pub fn failed_names(&self) -> Vec<&str> {
        self.results
            .iter()
            .filter(|r| !r.success)
            .map(|r| r.name.as_str())
            .collect()
    }
}
/// Where the script code comes from.
#[derive(Clone, Debug)]
pub enum ScriptSource {
    /// Inline script content.
    Inline(String),
    /// Path to a script file.
    File(PathBuf),
    /// An OxiLean expression to evaluate.
    OxiLeanExpr(String),
}
/// The result of executing a script.
#[derive(Clone, Debug)]
pub struct ScriptResult {
    /// Script name.
    pub name: String,
    /// Whether the script succeeded.
    pub success: bool,
    /// Exit code.
    pub exit_code: Option<i32>,
    /// Standard output.
    pub stdout: String,
    /// Standard error.
    pub stderr: String,
    /// Duration of execution.
    pub duration: Duration,
    /// Files generated by the script.
    pub generated_files: Vec<PathBuf>,
}
impl ScriptResult {
    /// Create a successful result.
    pub fn success(name: &str, duration: Duration) -> Self {
        Self {
            name: name.to_string(),
            success: true,
            exit_code: Some(0),
            stdout: String::new(),
            stderr: String::new(),
            duration,
            generated_files: Vec::new(),
        }
    }
    /// Create a failure result.
    pub fn failure(name: &str, exit_code: i32, stderr: &str, duration: Duration) -> Self {
        Self {
            name: name.to_string(),
            success: false,
            exit_code: Some(exit_code),
            stdout: String::new(),
            stderr: stderr.to_string(),
            duration,
            generated_files: Vec::new(),
        }
    }
}
/// Tracks which scripts depend on which input files.
#[allow(dead_code)]
pub struct ScriptDependencyTracker {
    /// script name → set of input paths it declared
    deps: HashMap<String, Vec<PathBuf>>,
    /// path → set of script names that depend on it
    path_to_scripts: HashMap<PathBuf, Vec<String>>,
}
#[allow(dead_code)]
impl ScriptDependencyTracker {
    /// Create a new tracker.
    pub fn new() -> Self {
        Self {
            deps: HashMap::new(),
            path_to_scripts: HashMap::new(),
        }
    }
    /// Register that `script_name` depends on `path`.
    pub fn register(&mut self, script_name: &str, path: impl Into<PathBuf>) {
        let path = path.into();
        self.deps
            .entry(script_name.to_string())
            .or_default()
            .push(path.clone());
        self.path_to_scripts
            .entry(path)
            .or_default()
            .push(script_name.to_string());
    }
    /// All scripts that depend on the given path.
    pub fn scripts_for_path(&self, path: &Path) -> Vec<&str> {
        self.path_to_scripts
            .get(path)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }
    /// All input paths for the given script.
    pub fn inputs_for_script(&self, name: &str) -> Vec<&PathBuf> {
        self.deps
            .get(name)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }
    /// Simulate checking whether any inputs have changed (always returns false
    /// in this simplified implementation).
    pub fn any_inputs_changed(&self, name: &str) -> bool {
        let _ = name;
        false
    }
    /// Number of tracked scripts.
    pub fn script_count(&self) -> usize {
        self.deps.len()
    }
    /// Total number of registered input-file dependencies.
    pub fn total_dep_count(&self) -> usize {
        self.deps.values().map(|v| v.len()).sum()
    }
}
/// Handles errors produced by script execution.
#[allow(dead_code)]
pub struct ScriptErrorHandler {
    strategy: ErrorHandlingStrategy,
    error_log: Vec<String>,
}
#[allow(dead_code)]
impl ScriptErrorHandler {
    /// Create a new error handler with the given strategy.
    pub fn new(strategy: ErrorHandlingStrategy) -> Self {
        Self {
            strategy,
            error_log: Vec::new(),
        }
    }
    /// Process a script result. Returns true if the build should abort.
    pub fn handle(&mut self, result: &ScriptResult) -> bool {
        if result.success {
            return false;
        }
        let msg = format!(
            "Script '{}' failed (exit={:?}): {}",
            result.name,
            result.exit_code,
            result.stderr.as_str()
        );
        self.error_log.push(msg);
        match &self.strategy {
            ErrorHandlingStrategy::AbortBuild => true,
            ErrorHandlingStrategy::Skip => false,
            ErrorHandlingStrategy::WarnAndContinue => false,
            ErrorHandlingStrategy::Retry(_) => false,
        }
    }
    /// All logged error messages.
    pub fn errors(&self) -> &[String] {
        &self.error_log
    }
    /// Number of errors logged.
    pub fn error_count(&self) -> usize {
        self.error_log.len()
    }
    /// Clear the error log.
    pub fn clear(&mut self) {
        self.error_log.clear();
    }
    /// Whether any errors have been logged.
    pub fn has_errors(&self) -> bool {
        !self.error_log.is_empty()
    }
}
/// A step in a script pipeline.
#[derive(Clone, Debug)]
pub struct PipelineStep {
    /// Step name.
    pub name: String,
    /// The script to run.
    pub script: ScriptDef,
    /// Continue on failure (override pipeline-level fail_fast).
    pub continue_on_failure: bool,
    /// Retry count.
    pub retry_count: u32,
    /// Delay between retries.
    pub retry_delay: Duration,
}
/// Packages a set of artifacts into a release bundle.
#[allow(dead_code)]
pub struct ArtifactPackager {
    artifacts: Vec<BuildArtifact>,
    output_dir: PathBuf,
    archive_name: String,
}
#[allow(dead_code)]
impl ArtifactPackager {
    /// Create a new packager.
    pub fn new(output_dir: impl Into<PathBuf>, archive_name: &str) -> Self {
        Self {
            artifacts: Vec::new(),
            output_dir: output_dir.into(),
            archive_name: archive_name.to_string(),
        }
    }
    /// Add an artifact to the package.
    pub fn add_artifact(&mut self, artifact: BuildArtifact) {
        self.artifacts.push(artifact);
    }
    /// Number of artifacts.
    pub fn artifact_count(&self) -> usize {
        self.artifacts.len()
    }
    /// List of artifacts by kind.
    pub fn artifacts_of_kind(&self, kind: &ArtifactKind) -> Vec<&BuildArtifact> {
        self.artifacts.iter().filter(|a| &a.kind == kind).collect()
    }
    /// Simulate packaging: returns a manifest string listing all artifacts.
    pub fn generate_manifest(&self) -> String {
        let mut lines = vec![format!("Archive: {}", self.archive_name)];
        for artifact in &self.artifacts {
            lines.push(format!(
                "  {} [{}] {} -> {}",
                artifact.name,
                artifact.kind,
                artifact.source_path.display(),
                artifact.dest_path.display()
            ));
        }
        lines.join("\n")
    }
    /// Total number of artifacts.
    pub fn total_count(&self) -> usize {
        self.artifacts.len()
    }
}
/// The kind of code generator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GeneratorKind {
    /// Generate FFI bindings.
    FfiBindings,
    /// Generate serialization code.
    Serialization,
    /// Generate test scaffolding.
    TestScaffold,
    /// Custom generator.
    Custom(String),
}
/// Manages parallel execution of multiple scripts.
#[allow(dead_code)]
pub struct ParallelScriptExecutor {
    runner: ScriptRunner,
    max_parallel: usize,
    handles: Vec<ParallelScriptHandle>,
}
#[allow(dead_code)]
impl ParallelScriptExecutor {
    /// Create a new parallel executor.
    pub fn new(runner: ScriptRunner, max_parallel: usize) -> Self {
        Self {
            runner,
            max_parallel: max_parallel.max(1),
            handles: Vec::new(),
        }
    }
    /// Queue a script for parallel execution.
    pub fn queue(&mut self, script: ScriptDef) {
        self.handles
            .push(ParallelScriptHandle::new(&script.name.clone()));
        let result = self.runner.run_single_public(&script).unwrap_or_else(|_| {
            ScriptResult::failure(
                &script.name,
                1,
                "execution failed",
                std::time::Duration::ZERO,
            )
        });
        // Safety: we just pushed a handle on the previous line
        let handle = self
            .handles
            .last_mut()
            .expect("handles is non-empty after push");
        *handle = handle.clone().resolve(result);
    }
    /// Run all queued scripts and return their results.
    pub fn run_all(&mut self, scripts: Vec<ScriptDef>) -> Vec<ScriptResult> {
        for script in scripts {
            self.queue(script);
        }
        self.collect_results()
    }
    /// Collect all resolved results.
    pub fn collect_results(&self) -> Vec<ScriptResult> {
        self.handles
            .iter()
            .filter_map(|h| h.result.clone())
            .collect()
    }
    /// Number of queued scripts.
    pub fn queue_len(&self) -> usize {
        self.handles.len()
    }
    /// Number of resolved (completed) scripts.
    pub fn resolved_count(&self) -> usize {
        self.handles.iter().filter(|h| h.is_resolved()).count()
    }
    /// Maximum parallel execution limit.
    pub fn max_parallel(&self) -> usize {
        self.max_parallel
    }
}
/// Profiling data for a single script execution.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ScriptProfile {
    /// Script name.
    pub name: String,
    /// Number of times executed.
    pub run_count: u64,
    /// Total wall-clock time across all runs.
    pub total_duration: Duration,
    /// Shortest recorded run.
    pub min_duration: Duration,
    /// Longest recorded run.
    pub max_duration: Duration,
    /// Whether the last run was a cache hit.
    pub last_was_cached: bool,
}
#[allow(dead_code)]
impl ScriptProfile {
    /// Create a new empty profile for a script.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            run_count: 0,
            total_duration: Duration::ZERO,
            min_duration: Duration::MAX,
            max_duration: Duration::ZERO,
            last_was_cached: false,
        }
    }
    /// Record a new run with the given duration.
    pub fn record(&mut self, duration: Duration, cached: bool) {
        self.run_count += 1;
        self.total_duration += duration;
        if duration < self.min_duration {
            self.min_duration = duration;
        }
        if duration > self.max_duration {
            self.max_duration = duration;
        }
        self.last_was_cached = cached;
    }
    /// Average duration per run.
    pub fn avg_duration(&self) -> Option<Duration> {
        if self.run_count == 0 {
            None
        } else {
            Some(self.total_duration / self.run_count as u32)
        }
    }
    /// Fraction of runs that were cache hits.
    pub fn cache_hit_rate(&self) -> f64 {
        if self.last_was_cached {
            1.0
        } else {
            0.0
        }
    }
}
/// A cache key for a script, based on its inputs and configuration.
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ScriptCacheKey {
    /// Script name.
    pub name: String,
    /// Hash of the script content.
    pub content_hash: u64,
    /// Hashes of input files (sorted by path).
    pub input_hashes: Vec<(PathBuf, u64)>,
}
#[allow(dead_code)]
impl ScriptCacheKey {
    /// Create a new cache key.
    pub fn new(name: &str, content_hash: u64) -> Self {
        Self {
            name: name.to_string(),
            content_hash,
            input_hashes: Vec::new(),
        }
    }
    /// Add an input file hash.
    pub fn with_input(mut self, path: impl Into<PathBuf>, hash: u64) -> Self {
        self.input_hashes.push((path.into(), hash));
        self.input_hashes.sort_by_key(|(p, _)| p.clone());
        self
    }
}
