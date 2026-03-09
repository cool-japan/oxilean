//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use super::types::{
    BuildArtifact, BuildCache, BuildPlan, BuildReport, BuildResult, BuildStages, Dependency,
    DependencySolver, DependencySource, DiscoveryOptions, LockEntry, LockFile, ModuleGraph,
    ModuleInfo, PackageEntry, PackageRegistry, Project, ProjectConfig, ProjectError,
    ProjectInitOptions, ResolvedDep, VersionConstraint,
};
use std::fs;
use std::process;

/// Parse `key = value` (values may be quoted).
pub fn parse_kv(line: &str) -> Option<(String, String)> {
    let idx = line.find('=')?;
    let key = line[..idx].trim().to_string();
    let raw_val = line[idx + 1..].trim();
    let value = unquote(raw_val);
    Some((key, value))
}
/// Remove surrounding quotes from a string value.
fn unquote(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}
/// Parse `["a", "b", "c"]` into `Vec<String>`.
pub fn parse_string_array(s: &str) -> Vec<String> {
    let s = s.trim();
    let inner = if s.starts_with('[') && s.ends_with(']') {
        &s[1..s.len() - 1]
    } else {
        s
    };
    inner
        .split(',')
        .map(|part| unquote(part.trim()))
        .filter(|part| !part.is_empty())
        .collect()
}
pub fn push_kv(out: &mut String, key: &str, value: &str) {
    out.push_str(&format!("{} = \"{}\"\n", key, value));
}
/// Minimal semver validation: MAJOR.MINOR.PATCH or *.
pub fn is_valid_semver(v: &str) -> bool {
    if v == "*" {
        return true;
    }
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    parts.iter().all(|p| p.parse::<u64>().is_ok())
}
/// Walk up directories from `start` looking for `oxilean.toml`.
pub fn find_project_file(start: &Path) -> Result<PathBuf, ProjectError> {
    let mut current = if start.is_file() {
        start.parent().unwrap_or(start).to_path_buf()
    } else {
        start.to_path_buf()
    };
    loop {
        let candidate = current.join("oxilean.toml");
        if candidate.exists() {
            return Ok(candidate);
        }
        if !current.pop() {
            break;
        }
    }
    Err(ProjectError::NotFound(
        "oxilean.toml not found in any parent directory".into(),
    ))
}
/// Recursively collect module files from a directory.
pub fn collect_modules(
    base: &Path,
    dir: &Path,
    out: &mut Vec<ModuleInfo>,
) -> Result<(), ProjectError> {
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };
    for entry in entries {
        let entry = entry.map_err(|e| ProjectError::IoError(e.to_string()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_modules(base, &path, out)?;
        } else if let Some(ext) = path.extension() {
            if ext == "lean" || ext == "ox" {
                let rel = path.strip_prefix(base).unwrap_or(&path).with_extension("");
                let mod_name = rel
                    .components()
                    .map(|c| c.as_os_str().to_string_lossy().to_string())
                    .collect::<Vec<_>>()
                    .join(".");
                let modified = path.metadata().ok().and_then(|m| m.modified().ok());
                out.push(ModuleInfo {
                    name: mod_name,
                    path: path.clone(),
                    dependencies: Vec::new(),
                    is_stale: true,
                    last_modified: modified,
                });
            }
        }
    }
    Ok(())
}
/// Validate project consistency.
pub fn validate_project(project: &Project) -> Result<(), Vec<ProjectError>> {
    let mut errors = Vec::new();
    if let Err(e) = project.config.validate() {
        errors.push(e);
    }
    let mut seen = HashSet::new();
    for m in &project.modules {
        if !seen.insert(&m.name) {
            errors.push(ProjectError::InvalidConfig(format!(
                "duplicate module: {}",
                m.name
            )));
        }
    }
    let cycles = find_cycles(&project.build_graph);
    if !cycles.is_empty() {
        for cycle in &cycles {
            errors.push(ProjectError::CyclicDependency(cycle.clone()));
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
/// Build a module graph from module info.
pub fn build_module_graph(modules: &[ModuleInfo]) -> ModuleGraph {
    let mut graph = ModuleGraph::new();
    for m in modules {
        graph.add_node(&m.name);
        for dep in &m.dependencies {
            graph.add_edge(&m.name, dep);
        }
    }
    graph
}
/// Topologically sort the module graph (returns build order).
/// Returns an error if the graph has cycles.
pub fn topological_sort(graph: &ModuleGraph) -> Result<Vec<String>, ProjectError> {
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for node in &graph.nodes {
        in_degree.entry(node.clone()).or_insert(0);
    }
    for deps in graph.edges.values() {
        for dep in deps {
            *in_degree.entry(dep.clone()).or_insert(0) += 1;
        }
    }
    let mut build_in_degree: HashMap<String, usize> = HashMap::new();
    for node in &graph.nodes {
        build_in_degree.entry(node.clone()).or_insert(0);
    }
    for (from, deps) in &graph.edges {
        let _ = from;
        for dep in deps {
            let _ = dep;
        }
    }
    for node in &graph.nodes {
        let dep_count = graph
            .dependencies_of(node)
            .iter()
            .filter(|d| graph.nodes.contains(d.as_str()))
            .count();
        build_in_degree.insert(node.clone(), dep_count);
    }
    let mut queue: VecDeque<String> = VecDeque::new();
    for (node, &deg) in &build_in_degree {
        if deg == 0 {
            queue.push_back(node.clone());
        }
    }
    let mut sorted_queue: Vec<String> = queue.into_iter().collect();
    sorted_queue.sort();
    let mut queue: VecDeque<String> = sorted_queue.into_iter().collect();
    let mut order = Vec::new();
    while let Some(node) = queue.pop_front() {
        order.push(node.clone());
        for dependent in graph.dependents_of(&node) {
            if let Some(deg) = build_in_degree.get_mut(dependent) {
                *deg = deg.saturating_sub(1);
                if *deg == 0 {
                    queue.push_back(dependent.clone());
                }
            }
        }
    }
    if order.len() != graph.nodes.len() {
        return Err(ProjectError::CyclicDependency(
            graph
                .nodes
                .difference(&order.iter().cloned().collect())
                .cloned()
                .collect::<Vec<_>>(),
        ));
    }
    Ok(order)
}
/// Detect cycles in the module graph.
/// Returns a list of cycles, each cycle being a list of module names.
pub fn find_cycles(graph: &ModuleGraph) -> Vec<Vec<String>> {
    let mut cycles = Vec::new();
    let mut visited = HashSet::new();
    let mut on_stack = HashSet::new();
    let mut stack = Vec::new();
    let mut sorted_nodes: Vec<&String> = graph.nodes.iter().collect();
    sorted_nodes.sort();
    for node in sorted_nodes {
        if !visited.contains(node.as_str()) {
            dfs_find_cycles(
                graph,
                node,
                &mut visited,
                &mut on_stack,
                &mut stack,
                &mut cycles,
            );
        }
    }
    cycles
}
fn dfs_find_cycles(
    graph: &ModuleGraph,
    node: &str,
    visited: &mut HashSet<String>,
    on_stack: &mut HashSet<String>,
    stack: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    visited.insert(node.to_string());
    on_stack.insert(node.to_string());
    stack.push(node.to_string());
    for dep in graph.dependencies_of(node) {
        if !visited.contains(dep.as_str()) {
            dfs_find_cycles(graph, dep, visited, on_stack, stack, cycles);
        } else if on_stack.contains(dep.as_str()) {
            let start = stack.iter().position(|s| s == dep).unwrap_or(0);
            let cycle: Vec<String> = stack[start..].to_vec();
            cycles.push(cycle);
        }
    }
    stack.pop();
    on_stack.remove(node);
}
/// Create a build plan from a project and its module graph.
pub fn create_build_plan(project: &Project) -> Result<BuildPlan, ProjectError> {
    let order = topological_sort(&project.build_graph)?;
    let stale_modules: HashSet<&str> = project
        .modules
        .iter()
        .filter(|m| m.is_stale)
        .map(|m| m.name.as_str())
        .collect();
    let mut to_rebuild: HashSet<String> = HashSet::new();
    for name in &order {
        if stale_modules.contains(name.as_str()) {
            to_rebuild.insert(name.clone());
        }
        for dep in project.build_graph.dependencies_of(name) {
            if to_rebuild.contains(dep) {
                to_rebuild.insert(name.clone());
                break;
            }
        }
    }
    let modules_to_build: Vec<String> = order
        .iter()
        .filter(|n| to_rebuild.contains(n.as_str()))
        .cloned()
        .collect();
    let parallelism = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1);
    Ok(BuildPlan {
        modules_to_build,
        order,
        parallelism,
    })
}
/// Execute one build step (compile a single module).
pub fn execute_build_step(
    project: &Project,
    module_name: &str,
    _env: &oxilean_kernel::Environment,
) -> BuildResult {
    let module_info = project.modules.iter().find(|m| m.name == module_name);
    let source_path = match module_info {
        Some(m) => m.path.clone(),
        None => {
            let rel_path = module_name.replace('.', "/") + ".lean";
            let found = project.config.source_dirs.iter().find_map(|dir| {
                let candidate = if dir.is_absolute() {
                    dir.join(&rel_path)
                } else {
                    project.root.join(dir).join(&rel_path)
                };
                if candidate.exists() {
                    Some(candidate)
                } else {
                    None
                }
            });
            match found {
                Some(p) => p,
                None => {
                    return BuildResult::Failure(format!("Module not found: {}", module_name));
                }
            }
        }
    };
    let source = match fs::read_to_string(&source_path) {
        Ok(s) => s,
        Err(e) => {
            return BuildResult::Failure(format!(
                "Failed to read {}: {}",
                source_path.display(),
                e
            ));
        }
    };
    match crate::commands::check_source(&source) {
        Ok(()) => BuildResult::Success,
        Err(e) => BuildResult::Failure(e.message),
    }
}
/// Run the full build pipeline.
pub fn build_project(project: &Project) -> Result<BuildReport, ProjectError> {
    let plan = create_build_plan(project)?;
    let start = std::time::Instant::now();
    let env = oxilean_kernel::Environment::new();
    let mut report = BuildReport::new();
    report.total = plan.order.len();
    for module_name in &plan.order {
        if plan.modules_to_build.contains(module_name) {
            let result = execute_build_step(project, module_name, &env);
            match &result {
                BuildResult::Success => report.succeeded += 1,
                BuildResult::Failure(_) => report.failed += 1,
                BuildResult::Cached => report.cached += 1,
            }
            report.results.insert(module_name.clone(), result);
        } else {
            report.cached += 1;
            report
                .results
                .insert(module_name.clone(), BuildResult::Cached);
        }
    }
    report.elapsed = start.elapsed();
    Ok(report)
}
/// Resolve all dependencies for a project.
pub fn resolve_dependencies(
    config: &ProjectConfig,
    registry: &PackageRegistry,
) -> Result<Vec<ResolvedDep>, ProjectError> {
    let mut resolved = Vec::new();
    for dep in &config.dependencies {
        match &dep.source {
            DependencySource::Path(p) => {
                resolved.push(ResolvedDep {
                    name: dep.name.clone(),
                    version: dep.version.clone(),
                    local_path: p.clone(),
                });
            }
            DependencySource::Git { url, rev } => {
                let local_path = PathBuf::from(format!(".oxilean/deps/{}", dep.name));
                resolved.push(ResolvedDep {
                    name: dep.name.clone(),
                    version: dep.version.clone(),
                    local_path: local_path.clone(),
                });
                let _ = (url, rev, local_path);
            }
            DependencySource::Registry { .. } => {
                let pkg = registry
                    .find(&dep.name)
                    .ok_or_else(|| ProjectError::DependencyNotFound(dep.name.clone()))?;
                let matched_version = pkg
                    .versions
                    .iter()
                    .find(|v| PackageRegistry::version_matches(&dep.version, v))
                    .ok_or_else(|| ProjectError::VersionNotFound {
                        name: dep.name.clone(),
                        version: dep.version.clone(),
                    })?;
                resolved.push(ResolvedDep {
                    name: dep.name.clone(),
                    version: matched_version.clone(),
                    local_path: PathBuf::from(format!(
                        ".oxilean/registry/{}/{}",
                        dep.name, matched_version
                    )),
                });
            }
        }
    }
    Ok(resolved)
}
/// Fetch (download/locate) a single dependency to disk.
///
/// - If the local path already exists, returns it immediately (cached).
/// - If the dep has an associated Git URL (stored in a `.git-src` marker),
///   runs `git clone` to populate the directory.
/// - Otherwise returns the path as-is.
pub fn fetch_dependency(dep: &ResolvedDep) -> Result<PathBuf, ProjectError> {
    if dep.local_path.exists() {
        return Ok(dep.local_path.clone());
    }
    let marker = dep.local_path.with_extension("git-src");
    if marker.exists() {
        if let Ok(url) = fs::read_to_string(&marker) {
            let url = url.trim().to_string();
            if !url.is_empty() {
                return git_clone_dep(&url, None, &dep.local_path);
            }
        }
    }
    if let Some(parent) = dep.local_path.parent() {
        fs::create_dir_all(parent).map_err(|e| ProjectError::IoError(e.to_string()))?;
    }
    Ok(dep.local_path.clone())
}
/// Clone a Git repository into `target_dir`, optionally checking out `rev`.
fn git_clone_dep(url: &str, rev: Option<&str>, target_dir: &Path) -> Result<PathBuf, ProjectError> {
    if let Some(parent) = target_dir.parent() {
        fs::create_dir_all(parent).map_err(|e| ProjectError::IoError(e.to_string()))?;
    }
    let status = std::process::Command::new("git")
        .args(["clone", "--depth", "1", url, &target_dir.to_string_lossy()])
        .status()
        .map_err(|e| ProjectError::IoError(format!("failed to run git: {}", e)))?;
    if !status.success() {
        return Err(ProjectError::IoError(format!(
            "git clone failed for {} (exit {})",
            url,
            status.code().unwrap_or(-1)
        )));
    }
    if let Some(rev) = rev {
        let status = std::process::Command::new("git")
            .args(["-C", &target_dir.to_string_lossy(), "checkout", rev])
            .status()
            .map_err(|e| ProjectError::IoError(format!("failed to run git checkout: {}", e)))?;
        if !status.success() {
            return Err(ProjectError::IoError(format!(
                "git checkout {} failed (exit {})",
                rev,
                status.code().unwrap_or(-1)
            )));
        }
    }
    Ok(target_dir.to_path_buf())
}
/// Fetch a Git dependency given explicit URL and optional rev.
pub fn fetch_git_dependency(
    url: &str,
    rev: Option<&str>,
    name: &str,
) -> Result<PathBuf, ProjectError> {
    let target = PathBuf::from(format!(".oxilean/deps/{}", name));
    if target.exists() {
        return Ok(target);
    }
    git_clone_dep(url, rev, &target)
}
/// Compare two semantic versions.
/// Returns -1 if v1 < v2, 0 if v1 == v2, 1 if v1 > v2.
pub fn compare_versions(v1: &str, v2: &str) -> i32 {
    let parts1: Vec<u64> = v1.split('.').filter_map(|p| p.parse().ok()).collect();
    let parts2: Vec<u64> = v2.split('.').filter_map(|p| p.parse().ok()).collect();
    for i in 0..parts1.len().max(parts2.len()) {
        let p1 = parts1.get(i).copied().unwrap_or(0);
        let p2 = parts2.get(i).copied().unwrap_or(0);
        if p1 < p2 {
            return -1;
        }
        if p1 > p2 {
            return 1;
        }
    }
    0
}
/// Initialize a new project at the given path.
pub fn init_project(path: &Path, options: ProjectInitOptions) -> Result<Project, ProjectError> {
    fs::create_dir_all(path).map_err(|e| ProjectError::IoError(e.to_string()))?;
    let src_dir = path.join("src");
    fs::create_dir_all(&src_dir).map_err(|e| ProjectError::IoError(e.to_string()))?;
    let lib_dir = path.join("lib");
    fs::create_dir_all(&lib_dir).map_err(|e| ProjectError::IoError(e.to_string()))?;
    let test_dir = path.join("test");
    fs::create_dir_all(&test_dir).map_err(|e| ProjectError::IoError(e.to_string()))?;
    let build_dir = path.join("build");
    fs::create_dir_all(&build_dir).map_err(|e| ProjectError::IoError(e.to_string()))?;
    let description = options.description.as_deref().unwrap_or("").to_string();
    let config = ProjectConfig {
        name: options.name.clone(),
        version: options.version.clone(),
        authors: options.authors.clone(),
        description,
        dependencies: Vec::new(),
        source_dirs: vec![PathBuf::from("src"), PathBuf::from("lib")],
        output_dir: PathBuf::from("build"),
        lean_version: "0.1.1".to_string(),
        extra_args: Vec::new(),
    };
    let config_path = path.join("oxilean.toml");
    let config_content = config.save();
    fs::write(&config_path, config_content).map_err(|e| ProjectError::IoError(e.to_string()))?;
    generate_gitignore(path)?;
    generate_readme(path, &options)?;
    generate_main_module(&src_dir)?;
    fs::write(src_dir.join(".gitkeep"), "").map_err(|e| ProjectError::IoError(e.to_string()))?;
    fs::write(test_dir.join(".gitkeep"), "").map_err(|e| ProjectError::IoError(e.to_string()))?;
    if options.with_git {
        let _ = init_git_repo(path);
    }
    Project::discover(path)
}
fn generate_gitignore(path: &Path) -> Result<(), ProjectError> {
    let gitignore_path = path.join(".gitignore");
    let content = r#"# OxiLean build artifacts
/build/
/*.olean
*.o
*.so
*.dylib

# Dependency cache
.oxilean/

# Editor artifacts
.vscode/
.idea/
*.swp
*.swo
*~

# OS artifacts
.DS_Store
Thumbs.db

# Lock file (optional - remove this line if you want to commit lockfile)
# Oxilean.lock
"#;
    fs::write(&gitignore_path, content).map_err(|e| ProjectError::IoError(e.to_string()))
}
fn generate_readme(path: &Path, options: &ProjectInitOptions) -> Result<(), ProjectError> {
    let readme_path = path.join("README.md");
    let content = format!(
        r#"# {}

{}

## Building

```bash
oxilean build
```

## Running

```bash
oxilean run
```

## Testing

```bash
oxilean test
```
"#,
        options.name,
        options
            .description
            .as_deref()
            .unwrap_or("A new OxiLean project")
    );
    fs::write(&readme_path, content).map_err(|e| ProjectError::IoError(e.to_string()))
}
fn generate_main_module(src_dir: &Path) -> Result<(), ProjectError> {
    let main_path = src_dir.join("Main.lean");
    let content = r#"-- Main module
-- Add your code here

namespace Main

def hello : String :=
  "Hello, OxiLean!"

end Main
"#;
    fs::write(&main_path, content).map_err(|e| ProjectError::IoError(e.to_string()))
}
fn init_git_repo(path: &Path) -> Result<(), ProjectError> {
    let git_dir = path.join(".git");
    if !git_dir.exists() {
        fs::create_dir(&git_dir).map_err(|e| ProjectError::IoError(e.to_string()))?;
    }
    Ok(())
}
/// Enhanced module discovery with namespace detection.
pub fn discover_modules(
    base: &Path,
    src_dir: &Path,
    options: &DiscoveryOptions,
) -> Result<Vec<ModuleInfo>, ProjectError> {
    let mut modules = Vec::new();
    collect_modules_advanced(base, src_dir, base, options, &mut modules)?;
    Ok(modules)
}
fn collect_modules_advanced(
    _base: &Path,
    dir: &Path,
    src_root: &Path,
    options: &DiscoveryOptions,
    out: &mut Vec<ModuleInfo>,
) -> Result<(), ProjectError> {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };
    for entry in entries {
        let entry = entry.map_err(|e| ProjectError::IoError(e.to_string()))?;
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();
        if path.is_dir() {
            if options.exclude_dirs.contains(file_name_str.as_ref()) {
                continue;
            }
            collect_modules_advanced(_base, &path, src_root, options, out)?;
        } else if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy();
            if options.extensions.contains(&ext_str.to_string()) {
                let rel = path.strip_prefix(src_root).unwrap_or(&path);
                let mod_name = if options.auto_namespace {
                    rel.components()
                        .map(|c| c.as_os_str().to_string_lossy().to_string())
                        .collect::<Vec<_>>()
                        .join(".")
                        .trim_end_matches(&format!(".{}", ext_str))
                        .to_string()
                } else {
                    path.file_stem()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string()
                };
                let modified = path.metadata().ok().and_then(|m| m.modified().ok());
                out.push(ModuleInfo {
                    name: mod_name,
                    path: path.clone(),
                    dependencies: Vec::new(),
                    is_stale: true,
                    last_modified: modified,
                });
            }
        }
    }
    Ok(())
}
/// Compute a simple hash of a file's content.
pub fn compute_source_hash(path: &Path) -> Result<String, ProjectError> {
    let content = fs::read(path).map_err(|e| ProjectError::IoError(e.to_string()))?;
    let len = content.len();
    let first = content.first().copied().unwrap_or(0);
    let last = content.last().copied().unwrap_or(0);
    Ok(format!("{:x}-{:x}-{:x}", len, first, last))
}
/// Extract import statements from a file.
pub fn extract_imports(path: &Path) -> Result<Vec<String>, ProjectError> {
    let content = fs::read_to_string(path).map_err(|e| ProjectError::IoError(e.to_string()))?;
    let mut imports = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("import ") {
            let rest = trimmed.strip_prefix("import ").unwrap_or("");
            let module_name = rest.split_whitespace().next().unwrap_or("").to_string();
            if !module_name.is_empty() {
                imports.push(module_name);
            }
        } else if trimmed.starts_with("open ") {
            let rest = trimmed.strip_prefix("open ").unwrap_or("");
            let module_name = rest.split_whitespace().next().unwrap_or("").to_string();
            if !module_name.is_empty() {
                imports.push(module_name);
            }
        }
    }
    Ok(imports)
}
/// Update module dependencies by parsing source files.
pub fn update_module_dependencies(modules: &mut [ModuleInfo]) -> Result<(), ProjectError> {
    for module in modules {
        module.dependencies = extract_imports(&module.path)?;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn sample_toml() -> &'static str {
        r#"
[package]
name = "my-project"
version = "0.2.0"
description = "A sample project"
authors = ["Alice", "Bob"]
lean_version = "0.1.1"

[[dependencies]]
name = "mathlib"
version = "3.0.0"
git = "https://github.com/oxilean/mathlib.git"
rev = "abc123"
"#
    }
    #[test]
    fn test_load_basic() {
        let config = ProjectConfig::load(sample_toml()).expect("config operation should succeed");
        assert_eq!(config.name, "my-project");
        assert_eq!(config.version, "0.2.0");
        assert_eq!(config.description, "A sample project");
        assert_eq!(config.authors, vec!["Alice", "Bob"]);
        assert_eq!(config.lean_version, "0.1.1");
    }
    #[test]
    fn test_load_dependencies() {
        let config = ProjectConfig::load(sample_toml()).expect("config operation should succeed");
        assert_eq!(config.dependencies.len(), 1);
        let dep = &config.dependencies[0];
        assert_eq!(dep.name, "mathlib");
        assert_eq!(dep.version, "3.0.0");
        match &dep.source {
            DependencySource::Git { url, rev } => {
                assert_eq!(url, "https://github.com/oxilean/mathlib.git");
                assert_eq!(rev.as_deref(), Some("abc123"));
            }
            _ => panic!("expected git source"),
        }
    }
    #[test]
    fn test_load_path_dependency() {
        let toml = r#"
[package]
name = "test"
version = "0.1.1"
lean_version = "0.1.1"

[[dependencies]]
name = "local-lib"
version = "0.1.1"
path = "../local-lib"
"#;
        let config = ProjectConfig::load(toml).expect("config operation should succeed");
        assert_eq!(config.dependencies.len(), 1);
        match &config.dependencies[0].source {
            DependencySource::Path(p) => assert_eq!(p, &PathBuf::from("../local-lib")),
            _ => panic!("expected path source"),
        }
    }
    #[test]
    fn test_save_roundtrip() {
        let original = ProjectConfig::load(sample_toml()).expect("test operation should succeed");
        let serialized = original.save();
        let reloaded = ProjectConfig::load(&serialized).expect("test operation should succeed");
        assert_eq!(reloaded.name, original.name);
        assert_eq!(reloaded.version, original.version);
        assert_eq!(reloaded.lean_version, original.lean_version);
        assert_eq!(reloaded.dependencies.len(), original.dependencies.len());
    }
    #[test]
    fn test_default_for() {
        let config = ProjectConfig::default_for("hello");
        assert_eq!(config.name, "hello");
        assert_eq!(config.version, "0.1.1");
        assert_eq!(config.source_dirs, vec![PathBuf::from("src")]);
        assert_eq!(config.output_dir, PathBuf::from("build"));
    }
    #[test]
    fn test_validate_empty_name() {
        let mut config = ProjectConfig::default_for("");
        config.name = String::new();
        assert!(config.validate().is_err());
    }
    #[test]
    fn test_validate_bad_version() {
        let mut config = ProjectConfig::default_for("test");
        config.version = "not-a-version".to_string();
        assert!(config.validate().is_err());
    }
    #[test]
    fn test_validate_duplicate_deps() {
        let mut config = ProjectConfig::default_for("test");
        config
            .dependencies
            .push(Dependency::path("a", "1.0.0", PathBuf::from("a")));
        config
            .dependencies
            .push(Dependency::path("a", "2.0.0", PathBuf::from("a2")));
        assert!(config.validate().is_err());
    }
    #[test]
    fn test_parse_error_unknown_section() {
        let toml = "[nonsense]\nkey = value\n";
        assert!(ProjectConfig::load(toml).is_err());
    }
    #[test]
    fn test_parse_error_bad_line() {
        let toml = "[package]\nthis is not a key value pair\n";
        assert!(ProjectConfig::load(toml).is_err());
    }
    fn make_graph() -> ModuleGraph {
        let mut g = ModuleGraph::new();
        g.add_edge("A", "B");
        g.add_edge("A", "C");
        g.add_edge("B", "C");
        g.add_edge("C", "D");
        g
    }
    #[test]
    fn test_graph_add_node() {
        let mut g = ModuleGraph::new();
        g.add_node("X");
        assert!(g.nodes.contains("X"));
        assert_eq!(g.len(), 1);
    }
    #[test]
    fn test_graph_add_edge() {
        let g = make_graph();
        assert_eq!(g.len(), 4);
        assert!(g.dependencies_of("A").contains(&"B".to_string()));
        assert!(g.dependencies_of("A").contains(&"C".to_string()));
        assert!(g.dependents_of("C").contains(&"A".to_string()));
        assert!(g.dependents_of("C").contains(&"B".to_string()));
    }
    #[test]
    fn test_transitive_deps() {
        let g = make_graph();
        let deps = g.transitive_deps("A");
        assert!(deps.contains("B"));
        assert!(deps.contains("C"));
        assert!(deps.contains("D"));
    }
    #[test]
    fn test_topological_sort_basic() {
        let g = make_graph();
        let order = topological_sort(&g).expect("test operation should succeed");
        let pos = |name: &str| {
            order
                .iter()
                .position(|n| n == name)
                .expect("test operation should succeed")
        };
        assert!(pos("D") < pos("C"));
        assert!(pos("C") < pos("B"));
        assert!(pos("B") < pos("A"));
    }
    #[test]
    fn test_topological_sort_single() {
        let mut g = ModuleGraph::new();
        g.add_node("Solo");
        let order = topological_sort(&g).expect("test operation should succeed");
        assert_eq!(order, vec!["Solo"]);
    }
    #[test]
    fn test_topological_sort_disjoint() {
        let mut g = ModuleGraph::new();
        g.add_node("X");
        g.add_node("Y");
        let order = topological_sort(&g).expect("test operation should succeed");
        assert_eq!(order.len(), 2);
    }
    #[test]
    fn test_find_cycles_none() {
        let g = make_graph();
        let cycles = find_cycles(&g);
        assert!(cycles.is_empty());
    }
    #[test]
    fn test_find_cycles_self_loop() {
        let mut g = ModuleGraph::new();
        g.add_edge("A", "A");
        let cycles = find_cycles(&g);
        assert!(!cycles.is_empty());
    }
    #[test]
    fn test_find_cycles_two_node() {
        let mut g = ModuleGraph::new();
        g.add_edge("A", "B");
        g.add_edge("B", "A");
        let cycles = find_cycles(&g);
        assert!(!cycles.is_empty());
    }
    #[test]
    fn test_topological_sort_cycle_error() {
        let mut g = ModuleGraph::new();
        g.add_edge("A", "B");
        g.add_edge("B", "A");
        assert!(topological_sort(&g).is_err());
    }
    fn make_test_project() -> Project {
        std::fs::create_dir_all("/tmp/test-proj/src").ok();
        std::fs::write("/tmp/test-proj/src/A.lean", "").ok();
        std::fs::write("/tmp/test-proj/src/B.lean", "").ok();
        let config = ProjectConfig::default_for("test-proj");
        let mut project = Project::from_config(PathBuf::from("/tmp/test-proj"), config);
        project.modules = vec![
            ModuleInfo {
                name: "A".into(),
                path: PathBuf::from("/tmp/test-proj/src/A.lean"),
                dependencies: vec!["B".into()],
                is_stale: true,
                last_modified: None,
            },
            ModuleInfo {
                name: "B".into(),
                path: PathBuf::from("/tmp/test-proj/src/B.lean"),
                dependencies: vec![],
                is_stale: true,
                last_modified: None,
            },
        ];
        project.build_dependency_graph();
        project
    }
    #[test]
    fn test_create_build_plan() {
        let project = make_test_project();
        let plan = create_build_plan(&project).expect("build operation should succeed");
        assert_eq!(plan.order.len(), 2);
        let pos_a = plan
            .order
            .iter()
            .position(|n| n == "A")
            .expect("test operation should succeed");
        let pos_b = plan
            .order
            .iter()
            .position(|n| n == "B")
            .expect("test operation should succeed");
        assert!(pos_b < pos_a);
        assert_eq!(plan.modules_to_build.len(), 2);
    }
    #[test]
    fn test_build_plan_cached() {
        let config = ProjectConfig::default_for("test");
        let mut project = Project::from_config(PathBuf::from("/tmp/test"), config);
        project.modules = vec![ModuleInfo {
            name: "X".into(),
            path: PathBuf::from("src/X.lean"),
            dependencies: vec![],
            is_stale: false,
            last_modified: None,
        }];
        project.build_dependency_graph();
        let plan = create_build_plan(&project).expect("build operation should succeed");
        assert!(plan.modules_to_build.is_empty());
    }
    #[test]
    fn test_build_project() {
        let project = make_test_project();
        let report = build_project(&project).expect("build operation should succeed");
        assert!(report.is_success());
        assert_eq!(report.total, 2);
        assert_eq!(report.succeeded, 2);
        assert_eq!(report.failed, 0);
    }
    #[test]
    fn test_build_report_summary() {
        let mut report = BuildReport::new();
        report.total = 5;
        report.succeeded = 3;
        report.cached = 1;
        report.failed = 1;
        report.elapsed = Duration::from_millis(1234);
        let s = report.summary();
        assert!(s.contains("5 total"));
        assert!(s.contains("3 succeeded"));
        assert!(s.contains("1 cached"));
        assert!(s.contains("1 failed"));
    }
    #[test]
    fn test_execute_build_step() {
        let project = make_test_project();
        let env = oxilean_kernel::Environment::new();
        let result = execute_build_step(&project, "A", &env);
        assert_eq!(result, BuildResult::Success);
    }
    #[test]
    fn test_package_registry() {
        let mut reg = PackageRegistry::new();
        reg.add(PackageEntry {
            name: "mathlib".into(),
            versions: vec!["3.0.0".into(), "2.0.0".into()],
            description: "Math library".into(),
            deps: vec![],
        });
        let pkg = reg.find("mathlib").expect("find should succeed");
        assert_eq!(pkg.versions.len(), 2);
    }
    #[test]
    fn test_version_matches() {
        assert!(PackageRegistry::version_matches("*", "1.2.3"));
        assert!(PackageRegistry::version_matches("1.2.3", "1.2.3"));
        assert!(!PackageRegistry::version_matches("1.2.3", "1.2.4"));
    }
    #[test]
    fn test_resolve_path_dep() {
        let mut config = ProjectConfig::default_for("test");
        config
            .dependencies
            .push(Dependency::path("foo", "1.0.0", PathBuf::from("./foo")));
        let reg = PackageRegistry::new();
        let resolved = resolve_dependencies(&config, &reg).expect("resolution should succeed");
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].name, "foo");
        assert_eq!(resolved[0].local_path, PathBuf::from("./foo"));
    }
    #[test]
    fn test_resolve_registry_dep() {
        let mut config = ProjectConfig::default_for("test");
        config.dependencies.push(Dependency::registry(
            "mathlib",
            "2.0.0",
            "https://pkg.oxilean.dev",
        ));
        let mut reg = PackageRegistry::new();
        reg.add(PackageEntry {
            name: "mathlib".into(),
            versions: vec!["3.0.0".into(), "2.0.0".into()],
            description: String::new(),
            deps: vec![],
        });
        let resolved = resolve_dependencies(&config, &reg).expect("resolution should succeed");
        assert_eq!(resolved[0].version, "2.0.0");
    }
    #[test]
    fn test_resolve_missing_dep() {
        let mut config = ProjectConfig::default_for("test");
        config.dependencies.push(Dependency::registry(
            "nonexistent",
            "1.0.0",
            "https://pkg.oxilean.dev",
        ));
        let reg = PackageRegistry::new();
        assert!(resolve_dependencies(&config, &reg).is_err());
    }
    #[test]
    fn test_lockfile_roundtrip() {
        let lock = LockFile {
            entries: vec![
                LockEntry {
                    name: "mathlib".into(),
                    version: "3.0.0".into(),
                    source: "git+https://...".into(),
                    checksum: Some("abc123".into()),
                },
                LockEntry {
                    name: "std".into(),
                    version: "0.1.1".into(),
                    source: "path+./std".into(),
                    checksum: None,
                },
            ],
        };
        let serialized = lock.serialize();
        let deserialized =
            LockFile::deserialize(&serialized).expect("test operation should succeed");
        assert_eq!(deserialized.entries.len(), 2);
        assert_eq!(deserialized.entries[0].name, "mathlib");
        assert_eq!(deserialized.entries[0].checksum.as_deref(), Some("abc123"));
        assert_eq!(deserialized.entries[1].name, "std");
    }
    #[test]
    fn test_lockfile_is_locked() {
        let lock = LockFile {
            entries: vec![LockEntry {
                name: "foo".into(),
                version: "1.0.0".into(),
                source: String::new(),
                checksum: None,
            }],
        };
        assert!(lock.is_locked("foo", "1.0.0"));
        assert!(!lock.is_locked("foo", "2.0.0"));
        assert!(!lock.is_locked("bar", "1.0.0"));
    }
    #[test]
    fn test_lockfile_from_resolved() {
        let resolved = vec![ResolvedDep {
            name: "math".into(),
            version: "1.0.0".into(),
            local_path: PathBuf::from("/tmp/math"),
        }];
        let lock = LockFile::from_resolved(&resolved);
        assert_eq!(lock.entries.len(), 1);
        assert_eq!(lock.entries[0].name, "math");
    }
    #[test]
    fn test_validate_project_ok() {
        let project = make_test_project();
        assert!(validate_project(&project).is_ok());
    }
    #[test]
    fn test_validate_project_cycle() {
        let config = ProjectConfig::default_for("test");
        let mut project = Project::from_config(PathBuf::from("/tmp/test"), config);
        project.modules = vec![
            ModuleInfo {
                name: "A".into(),
                path: PathBuf::from("A.lean"),
                dependencies: vec!["B".into()],
                is_stale: true,
                last_modified: None,
            },
            ModuleInfo {
                name: "B".into(),
                path: PathBuf::from("B.lean"),
                dependencies: vec!["A".into()],
                is_stale: true,
                last_modified: None,
            },
        ];
        project.build_dependency_graph();
        assert!(validate_project(&project).is_err());
    }
    #[test]
    fn test_semver_validation() {
        assert!(is_valid_semver("0.1.1"));
        assert!(is_valid_semver("1.2.3"));
        assert!(is_valid_semver("*"));
        assert!(!is_valid_semver("abc"));
        assert!(!is_valid_semver("1.2"));
        assert!(!is_valid_semver("1.2.3.4"));
    }
    #[test]
    fn test_unquote() {
        assert_eq!(unquote("\"hello\""), "hello");
        assert_eq!(unquote("'world'"), "world");
        assert_eq!(unquote("bare"), "bare");
    }
    #[test]
    fn test_parse_string_array() {
        let arr = parse_string_array("[\"a\", \"b\", \"c\"]");
        assert_eq!(arr, vec!["a", "b", "c"]);
    }
    #[test]
    fn test_parse_string_array_empty() {
        let arr = parse_string_array("[]");
        assert!(arr.is_empty());
    }
    #[test]
    fn test_dependency_display() {
        let dep_path = DependencySource::Path(PathBuf::from("./lib"));
        assert!(dep_path.to_string().contains("path"));
        let dep_git = DependencySource::Git {
            url: "https://github.com/test".into(),
            rev: Some("v1".into()),
        };
        assert!(dep_git.to_string().contains("git"));
        let dep_reg = DependencySource::Registry {
            registry: "https://reg.oxilean.dev".into(),
        };
        assert!(dep_reg.to_string().contains("registry"));
    }
    #[test]
    fn test_project_error_display() {
        let e = ProjectError::ParseError {
            line: 10,
            message: "bad key".into(),
        };
        let s = e.to_string();
        assert!(s.contains("line 10"));
        assert!(s.contains("bad key"));
    }
    #[test]
    fn test_build_module_graph_from_info() {
        let modules = vec![
            ModuleInfo {
                name: "Root".into(),
                path: PathBuf::from("Root.lean"),
                dependencies: vec!["Util".into()],
                is_stale: true,
                last_modified: None,
            },
            ModuleInfo {
                name: "Util".into(),
                path: PathBuf::from("Util.lean"),
                dependencies: vec![],
                is_stale: false,
                last_modified: None,
            },
        ];
        let graph = build_module_graph(&modules);
        assert_eq!(graph.len(), 2);
        assert!(graph.dependencies_of("Root").contains(&"Util".to_string()));
    }
    #[test]
    fn test_dependency_constructors() {
        let d1 = Dependency::path("a", "1.0.0", PathBuf::from("x"));
        assert_eq!(d1.name, "a");
        let d2 = Dependency::git("b", "2.0.0", "https://x", Some("main"));
        match &d2.source {
            DependencySource::Git { rev, .. } => assert_eq!(rev.as_deref(), Some("main")),
            _ => panic!("expected git"),
        }
        let d3 = Dependency::registry("c", "3.0.0", "https://reg");
        match &d3.source {
            DependencySource::Registry { registry } => {
                assert_eq!(registry, "https://reg")
            }
            _ => panic!("expected registry"),
        }
    }
    #[test]
    fn test_graph_empty() {
        let g = ModuleGraph::new();
        assert!(g.is_empty());
        assert_eq!(g.len(), 0);
    }
    #[test]
    fn test_topological_sort_empty() {
        let g = ModuleGraph::new();
        let order = topological_sort(&g).expect("test operation should succeed");
        assert!(order.is_empty());
    }
    #[test]
    fn test_fetch_dependency() {
        let dep = ResolvedDep {
            name: "test".into(),
            version: "1.0.0".into(),
            local_path: PathBuf::from("/tmp/dep"),
        };
        let path = fetch_dependency(&dep).expect("test operation should succeed");
        assert_eq!(path, PathBuf::from("/tmp/dep"));
    }
    #[test]
    fn test_config_save_with_extras() {
        let mut config = ProjectConfig::default_for("test");
        config.authors = vec!["Alice".into()];
        config.extra_args = vec!["--verbose".into()];
        config.source_dirs = vec![PathBuf::from("lib"), PathBuf::from("src")];
        config.output_dir = PathBuf::from("out");
        let saved = config.save();
        assert!(saved.contains("Alice"));
        assert!(saved.contains("--verbose"));
        assert!(saved.contains("lib"));
        assert!(saved.contains("out"));
    }
    #[test]
    fn test_resolve_git_dep() {
        let mut config = ProjectConfig::default_for("test");
        config.dependencies.push(Dependency::git(
            "lib",
            "1.0.0",
            "https://github.com/test/lib",
            None,
        ));
        let reg = PackageRegistry::new();
        let resolved = resolve_dependencies(&config, &reg).expect("resolution should succeed");
        assert_eq!(resolved.len(), 1);
        assert!(resolved[0].local_path.to_string_lossy().contains("lib"));
    }
    #[test]
    fn test_validate_no_source_dirs() {
        let mut config = ProjectConfig::default_for("test");
        config.source_dirs.clear();
        assert!(config.validate().is_err());
    }
    #[test]
    fn test_version_constraint_any() {
        let c = VersionConstraint::Any;
        assert!(c.matches("1.0.0"));
        assert!(c.matches("999.999.999"));
    }
    #[test]
    fn test_version_constraint_exact() {
        let c = VersionConstraint::Exact("1.2.3".to_string());
        assert!(c.matches("1.2.3"));
        assert!(!c.matches("1.2.4"));
    }
    #[test]
    fn test_version_constraint_at_least() {
        let c = VersionConstraint::AtLeast("1.0.0".to_string());
        assert!(c.matches("1.0.0"));
        assert!(c.matches("1.0.1"));
        assert!(c.matches("2.0.0"));
        assert!(!c.matches("0.9.9"));
    }
    #[test]
    fn test_version_constraint_compatible() {
        let c = VersionConstraint::Compatible("1.2.0".to_string());
        assert!(c.matches("1.2.0"));
        assert!(c.matches("1.2.5"));
    }
    #[test]
    fn test_version_constraint_parse_any() {
        assert_eq!(VersionConstraint::parse("*"), VersionConstraint::Any);
        assert_eq!(VersionConstraint::parse("latest"), VersionConstraint::Any);
    }
    #[test]
    fn test_version_constraint_parse_exact() {
        match VersionConstraint::parse("1.2.3") {
            VersionConstraint::Exact(v) => assert_eq!(v, "1.2.3"),
            _ => panic!("expected exact"),
        }
    }
    #[test]
    fn test_version_constraint_parse_at_least() {
        match VersionConstraint::parse(">=1.0.0") {
            VersionConstraint::AtLeast(v) => assert_eq!(v, "1.0.0"),
            _ => panic!("expected at least"),
        }
    }
    #[test]
    fn test_version_constraint_parse_compatible() {
        match VersionConstraint::parse("~1.2.0") {
            VersionConstraint::Compatible(v) => assert_eq!(v, "1.2.0"),
            _ => panic!("expected compatible"),
        }
    }
    #[test]
    fn test_compare_versions() {
        assert_eq!(compare_versions("1.0.0", "1.0.0"), 0);
        assert_eq!(compare_versions("2.0.0", "1.0.0"), 1);
        assert_eq!(compare_versions("1.0.0", "2.0.0"), -1);
        assert_eq!(compare_versions("1.0.0", "1.0.1"), -1);
    }
    #[test]
    fn test_dependency_solver_new() {
        let reg = PackageRegistry::new();
        let solver = DependencySolver::new(reg);
        assert!(solver.resolved.is_empty());
    }
    #[test]
    fn test_dependency_solver_resolve_path() {
        let reg = PackageRegistry::new();
        let mut solver = DependencySolver::new(reg);
        let dep = Dependency::path("lib", "1.0.0", PathBuf::from("./lib"));
        let deps = vec![dep];
        let resolved = solver
            .resolve_all(&deps)
            .expect("resolution should succeed");
        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].name, "lib");
    }
    #[test]
    fn test_dependency_solver_resolve_git() {
        let reg = PackageRegistry::new();
        let mut solver = DependencySolver::new(reg);
        let dep = Dependency::git("lib", "1.0.0", "https://github.com/test/lib", None);
        let deps = vec![dep];
        let resolved = solver
            .resolve_all(&deps)
            .expect("resolution should succeed");
        assert_eq!(resolved.len(), 1);
    }
    #[test]
    fn test_build_cache_new() {
        let cache = BuildCache::new();
        assert!(cache.artifacts.is_empty());
        assert!(cache.invalidated_at.is_none());
    }
    #[test]
    fn test_build_cache_insert() {
        let mut cache = BuildCache::new();
        let artifact = BuildArtifact {
            module_name: "Test".to_string(),
            artifact_path: PathBuf::from("test.olean"),
            source_hash: "abc123".to_string(),
            built_at: SystemTime::now(),
        };
        cache.insert(artifact);
        assert_eq!(cache.artifacts.len(), 1);
    }
    #[test]
    fn test_build_cache_is_valid() {
        let mut cache = BuildCache::new();
        let artifact = BuildArtifact {
            module_name: "Test".to_string(),
            artifact_path: PathBuf::from("test.olean"),
            source_hash: "abc123".to_string(),
            built_at: SystemTime::now(),
        };
        cache.insert(artifact);
        assert!(cache.is_valid("Test", "abc123"));
        assert!(!cache.is_valid("Test", "different"));
    }
    #[test]
    fn test_build_cache_invalidate() {
        let mut cache = BuildCache::new();
        cache.invalidate();
        assert!(cache.invalidated_at.is_some());
        assert!(!cache.is_valid("anything", "hash"));
    }
    #[test]
    fn test_build_cache_clear() {
        let mut cache = BuildCache::new();
        let artifact = BuildArtifact {
            module_name: "Test".to_string(),
            artifact_path: PathBuf::from("test.olean"),
            source_hash: "abc123".to_string(),
            built_at: SystemTime::now(),
        };
        cache.insert(artifact);
        cache.clear();
        assert!(cache.artifacts.is_empty());
    }
    #[test]
    fn test_build_stages_from_simple_graph() {
        let mut g = ModuleGraph::new();
        g.add_edge("A", "B");
        g.add_node("B");
        let stages = BuildStages::from_graph(&g).expect("test operation should succeed");
        assert_eq!(stages.stages.len(), 2);
        assert!(stages.stages[0].contains(&"B".to_string()));
        assert!(stages.stages[1].contains(&"A".to_string()));
    }
    #[test]
    fn test_build_stages_max_parallelism() {
        let mut g = ModuleGraph::new();
        g.add_node("A");
        g.add_node("B");
        g.add_node("C");
        g.add_edge("D", "A");
        g.add_edge("D", "B");
        g.add_edge("D", "C");
        let stages = BuildStages::from_graph(&g).expect("test operation should succeed");
        assert!(stages.max_parallelism() >= 2);
    }
    #[test]
    fn test_build_stages_detects_cycle() {
        let mut g = ModuleGraph::new();
        g.add_edge("A", "B");
        g.add_edge("B", "A");
        let result = BuildStages::from_graph(&g);
        assert!(result.is_err());
    }
    #[test]
    fn test_extract_imports_basic() {
        let path = PathBuf::from("/tmp/nonexistent.lean");
        let _ = extract_imports(&path);
    }
    #[test]
    fn test_project_init_options_new() {
        let opts = ProjectInitOptions::new("test-proj".to_string());
        assert_eq!(opts.name, "test-proj");
        assert_eq!(opts.version, "0.1.1");
        assert!(opts.with_git);
    }
    #[test]
    fn test_project_init_options_defaults() {
        let opts = ProjectInitOptions::new("proj".to_string());
        assert_eq!(opts.version, "0.1.1");
        assert!(opts.authors.is_empty());
        assert!(opts.description.is_none());
        assert!(!opts.with_examples);
    }
    #[test]
    fn test_discovery_options_default() {
        let opts = DiscoveryOptions::default();
        assert!(opts.extensions.contains(&"lean".to_string()));
        assert!(opts.extensions.contains(&"ox".to_string()));
        assert!(opts.exclude_dirs.contains("build"));
        assert!(opts.auto_namespace);
    }
    #[test]
    fn test_discovery_options_exclude_dirs() {
        let opts = DiscoveryOptions::default();
        assert!(opts.exclude_dirs.contains("build"));
        assert!(opts.exclude_dirs.contains(".git"));
        assert!(opts.exclude_dirs.contains("target"));
        assert!(opts.exclude_dirs.contains(".oxilean"));
    }
    #[test]
    fn test_topological_sort_complex() {
        let mut g = ModuleGraph::new();
        g.add_edge("Main", "Util");
        g.add_edge("Main", "Math");
        g.add_edge("Util", "Base");
        g.add_edge("Math", "Base");
        let order = topological_sort(&g).expect("test operation should succeed");
        let pos = |name: &str| {
            order
                .iter()
                .position(|n| n == name)
                .expect("test operation should succeed")
        };
        assert!(pos("Base") < pos("Util"));
        assert!(pos("Base") < pos("Math"));
        assert!(pos("Util") < pos("Main"));
        assert!(pos("Math") < pos("Main"));
    }
    #[test]
    fn test_transitive_deps_complex() {
        let mut g = ModuleGraph::new();
        g.add_edge("A", "B");
        g.add_edge("B", "C");
        g.add_edge("C", "D");
        let deps = g.transitive_deps("A");
        assert!(deps.contains("B"));
        assert!(deps.contains("C"));
        assert!(deps.contains("D"));
        assert!(!deps.contains("A"));
    }
    #[test]
    fn test_module_graph_independence() {
        let mut g = ModuleGraph::new();
        g.add_node("X");
        g.add_node("Y");
        g.add_node("Z");
        assert_eq!(g.dependencies_of("X").len(), 0);
        assert_eq!(g.dependents_of("X").len(), 0);
    }
    #[test]
    fn test_config_empty_sources() {
        let toml = "[package]\nname = \"test\"\nversion = \"0.1.1\"\nsource_dirs = []\n";
        let config = ProjectConfig::load(toml).expect("config operation should succeed");
        assert!(config.source_dirs.is_empty());
        assert!(config.validate().is_err());
    }
    #[test]
    fn test_config_multiple_source_dirs() {
        let toml = r#"
[package]
name = "test"
version = "0.1.1"
source_dirs = ["src", "lib", "examples"]
"#;
        let config = ProjectConfig::load(toml).expect("config operation should succeed");
        assert_eq!(config.source_dirs.len(), 3);
    }
    #[test]
    fn test_config_custom_output_dir() {
        let toml = r#"
[package]
name = "test"
version = "0.1.1"
output_dir = "target/olean"
"#;
        let config = ProjectConfig::load(toml).expect("config operation should succeed");
        assert_eq!(config.output_dir, PathBuf::from("target/olean"));
    }
    #[test]
    fn test_lockfile_deserialize_empty() {
        let content = "# Empty lockfile\n";
        let lock = LockFile::deserialize(content).expect("test operation should succeed");
        assert!(lock.entries.is_empty());
    }
    #[test]
    fn test_lockfile_with_checksums() {
        let lock = LockFile {
            entries: vec![
                LockEntry {
                    name: "a".into(),
                    version: "1.0.0".into(),
                    source: "path".into(),
                    checksum: Some("hash1".into()),
                },
                LockEntry {
                    name: "b".into(),
                    version: "2.0.0".into(),
                    source: "git".into(),
                    checksum: Some("hash2".into()),
                },
            ],
        };
        let serialized = lock.serialize();
        let deserialized =
            LockFile::deserialize(&serialized).expect("test operation should succeed");
        assert_eq!(deserialized.entries[0].checksum.as_deref(), Some("hash1"));
        assert_eq!(deserialized.entries[1].checksum.as_deref(), Some("hash2"));
    }
    #[test]
    fn test_resolve_dependencies_empty() {
        let config = ProjectConfig::default_for("test");
        let reg = PackageRegistry::new();
        let resolved = resolve_dependencies(&config, &reg).expect("resolution should succeed");
        assert!(resolved.is_empty());
    }
    #[test]
    fn test_resolve_multiple_deps() {
        let mut config = ProjectConfig::default_for("test");
        config
            .dependencies
            .push(Dependency::path("a", "1.0.0", PathBuf::from("./a")));
        config
            .dependencies
            .push(Dependency::path("b", "2.0.0", PathBuf::from("./b")));
        let reg = PackageRegistry::new();
        let resolved = resolve_dependencies(&config, &reg).expect("resolution should succeed");
        assert_eq!(resolved.len(), 2);
    }
    #[test]
    fn test_version_not_found() {
        let mut config = ProjectConfig::default_for("test");
        config
            .dependencies
            .push(Dependency::registry("math", "99.99.99", "https://reg"));
        let mut reg = PackageRegistry::new();
        reg.add(PackageEntry {
            name: "math".into(),
            versions: vec!["1.0.0".into()],
            description: String::new(),
            deps: vec![],
        });
        assert!(resolve_dependencies(&config, &reg).is_err());
    }
    #[test]
    fn test_build_plan_all_stale() {
        let config = ProjectConfig::default_for("test");
        let mut project = Project::from_config(PathBuf::from("/tmp"), config);
        project.modules = vec![
            ModuleInfo {
                name: "A".into(),
                path: PathBuf::from("A.lean"),
                dependencies: vec![],
                is_stale: true,
                last_modified: None,
            },
            ModuleInfo {
                name: "B".into(),
                path: PathBuf::from("B.lean"),
                dependencies: vec![],
                is_stale: true,
                last_modified: None,
            },
        ];
        project.build_dependency_graph();
        let plan = create_build_plan(&project).expect("build operation should succeed");
        assert_eq!(plan.modules_to_build.len(), 2);
    }
    #[test]
    fn test_build_plan_none_stale() {
        let config = ProjectConfig::default_for("test");
        let mut project = Project::from_config(PathBuf::from("/tmp"), config);
        project.modules = vec![ModuleInfo {
            name: "A".into(),
            path: PathBuf::from("A.lean"),
            dependencies: vec![],
            is_stale: false,
            last_modified: None,
        }];
        project.build_dependency_graph();
        let plan = create_build_plan(&project).expect("build operation should succeed");
        assert!(plan.modules_to_build.is_empty());
    }
    #[test]
    fn test_build_plan_mixed_staleness() {
        let config = ProjectConfig::default_for("test");
        let mut project = Project::from_config(PathBuf::from("/tmp"), config);
        project.modules = vec![
            ModuleInfo {
                name: "Base".into(),
                path: PathBuf::from("Base.lean"),
                dependencies: vec![],
                is_stale: false,
                last_modified: None,
            },
            ModuleInfo {
                name: "App".into(),
                path: PathBuf::from("App.lean"),
                dependencies: vec!["Base".into()],
                is_stale: true,
                last_modified: None,
            },
        ];
        project.build_dependency_graph();
        let plan = create_build_plan(&project).expect("build operation should succeed");
        assert_eq!(plan.modules_to_build.len(), 1);
        assert!(plan.modules_to_build.contains(&"App".to_string()));
    }
    #[test]
    fn test_build_plan_propagation() {
        let config = ProjectConfig::default_for("test");
        let mut project = Project::from_config(PathBuf::from("/tmp"), config);
        project.modules = vec![
            ModuleInfo {
                name: "Base".into(),
                path: PathBuf::from("Base.lean"),
                dependencies: vec![],
                is_stale: true,
                last_modified: None,
            },
            ModuleInfo {
                name: "Mid".into(),
                path: PathBuf::from("Mid.lean"),
                dependencies: vec!["Base".into()],
                is_stale: false,
                last_modified: None,
            },
            ModuleInfo {
                name: "App".into(),
                path: PathBuf::from("App.lean"),
                dependencies: vec!["Mid".into()],
                is_stale: false,
                last_modified: None,
            },
        ];
        project.build_dependency_graph();
        let plan = create_build_plan(&project).expect("build operation should succeed");
        assert!(plan.modules_to_build.contains(&"Base".to_string()));
    }
    #[test]
    fn test_cycles_three_node() {
        let mut g = ModuleGraph::new();
        g.add_edge("A", "B");
        g.add_edge("B", "C");
        g.add_edge("C", "A");
        let cycles = find_cycles(&g);
        assert!(!cycles.is_empty());
    }
    #[test]
    fn test_cycles_multiple_components() {
        let mut g = ModuleGraph::new();
        g.add_edge("A", "B");
        g.add_edge("B", "A");
        g.add_node("X");
        g.add_node("Y");
        let cycles = find_cycles(&g);
        assert!(!cycles.is_empty());
    }
    #[test]
    fn test_error_cyclic_dependency_display() {
        let e = ProjectError::CyclicDependency(vec!["A".into(), "B".into(), "C".into()]);
        let s = e.to_string();
        assert!(s.contains("cyclic dependency"));
        assert!(s.contains("A"));
    }
    #[test]
    fn test_error_version_not_found_display() {
        let e = ProjectError::VersionNotFound {
            name: "mathlib".into(),
            version: "99.0.0".into(),
        };
        let s = e.to_string();
        assert!(s.contains("99.0.0"));
        assert!(s.contains("mathlib"));
    }
    #[test]
    fn test_error_dependency_not_found_display() {
        let e = ProjectError::DependencyNotFound("nonexistent".into());
        let s = e.to_string();
        assert!(s.contains("nonexistent"));
        assert!(s.contains("not found"));
    }
    #[test]
    fn test_dependency_source_display_git_with_rev() {
        let src = DependencySource::Git {
            url: "https://github.com/test/repo".into(),
            rev: Some("v1.0".into()),
        };
        let s = src.to_string();
        assert!(s.contains("git"));
        assert!(s.contains("v1.0"));
    }
    #[test]
    fn test_dependency_source_display_git_no_rev() {
        let src = DependencySource::Git {
            url: "https://github.com/test/repo".into(),
            rev: None,
        };
        let s = src.to_string();
        assert!(s.contains("git"));
        assert!(!s.contains("rev"));
    }
    #[test]
    fn test_config_save_minimal() {
        let config = ProjectConfig::default_for("test");
        let saved = config.save();
        assert!(saved.contains("test"));
        assert!(saved.contains("0.1.1"));
    }
    #[test]
    fn test_config_serialization_preserves_structure() {
        let toml = r#"
[package]
name = "complex-proj"
version = "2.0.0"
description = "Complex project"
authors = ["Alice", "Bob"]

[[dependencies]]
name = "mathlib"
version = "3.0.0"
git = "https://github.com/test/mathlib.git"
rev = "main"

[[dependencies]]
name = "local"
version = "0.1.1"
path = "../local"
"#;
        let config = ProjectConfig::load(toml).expect("config operation should succeed");
        let saved = config.save();
        let reloaded = ProjectConfig::load(&saved).expect("test operation should succeed");
        assert_eq!(reloaded.name, "complex-proj");
        assert_eq!(reloaded.version, "2.0.0");
        assert_eq!(reloaded.authors.len(), 2);
        assert_eq!(reloaded.dependencies.len(), 2);
    }
    #[test]
    fn test_build_report_is_success() {
        let mut report = BuildReport::new();
        report.total = 5;
        report.succeeded = 5;
        report.failed = 0;
        assert!(report.is_success());
    }
    #[test]
    fn test_build_report_is_failure() {
        let mut report = BuildReport::new();
        report.total = 5;
        report.succeeded = 3;
        report.failed = 2;
        assert!(!report.is_success());
    }
    #[test]
    fn test_build_report_zero_elapsed() {
        let report = BuildReport::new();
        assert_eq!(report.elapsed, Duration::ZERO);
    }
    #[test]
    fn test_graph_deterministic_output() {
        let mut g = ModuleGraph::new();
        g.add_edge("Z", "A");
        g.add_edge("Y", "B");
        let order = topological_sort(&g).expect("test operation should succeed");
        assert_eq!(order.len(), 4);
    }
    #[test]
    fn test_module_info_creation() {
        let info = ModuleInfo {
            name: "Test.Module".into(),
            path: PathBuf::from("src/Test/Module.lean"),
            dependencies: vec!["Base".into()],
            is_stale: true,
            last_modified: None,
        };
        assert_eq!(info.name, "Test.Module");
        assert_eq!(info.dependencies.len(), 1);
    }
    #[test]
    fn test_package_registry_find_nonexistent() {
        let reg = PackageRegistry::new();
        assert!(reg.find("nonexistent").is_none());
    }
    #[test]
    fn test_package_entry_versions_order() {
        let entry = PackageEntry {
            name: "lib".into(),
            versions: vec!["1.0.0".into(), "2.0.0".into(), "0.5.0".into()],
            description: "A library".into(),
            deps: vec![],
        };
        assert_eq!(entry.versions.len(), 3);
    }
}
