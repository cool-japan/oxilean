//! Module system functions: path parsing, resolution, graph analysis.

use super::types::{
    CycleError, ImportDecl, ModuleGraph, ModuleInfo, ModulePath, ModuleRegistry,
    ModuleResolutionResult,
};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::PathBuf;
use std::str::FromStr;

// ─────────────────────────────────────────────────────────────────────────────
// ModulePath — FromStr + helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Error returned when a module path string cannot be parsed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulePathParseError(pub String);

impl std::fmt::Display for ModulePathParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid module path: {:?}", self.0)
    }
}

impl std::error::Error for ModulePathParseError {}

impl FromStr for ModulePath {
    type Err = ModulePathParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ModulePathParseError(s.to_owned()));
        }
        let components: Vec<String> = s.split('.').map(|c| c.to_owned()).collect();
        if components.iter().any(|c| c.is_empty()) {
            return Err(ModulePathParseError(s.to_owned()));
        }
        Ok(Self { components })
    }
}

impl ModulePath {
    /// Parse a dot-separated string into a [`ModulePath`], returning `None` on
    /// failure.  Convenience wrapper around [`FromStr`].
    pub fn parse(s: &str) -> Option<Self> {
        s.parse().ok()
    }

    /// Convert to a relative file-system path with a `.lean` extension.
    /// The returned path is *relative* — callers must prefix with a root.
    ///
    /// `Mathlib.Algebra.Ring` → `Mathlib/Algebra/Ring.lean`
    pub fn to_file_path(&self) -> PathBuf {
        let mut p: PathBuf = self.components.iter().collect();
        p.set_extension("lean");
        p
    }

    /// Same as [`Self::to_file_path`] but with the `.oxilean` extension.
    pub fn to_oxilean_file_path(&self) -> PathBuf {
        let mut p: PathBuf = self.components.iter().collect();
        p.set_extension("oxilean");
        p
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// ModuleRegistry methods
// ─────────────────────────────────────────────────────────────────────────────

impl ModuleRegistry {
    /// Create a new registry with the given search roots.
    pub fn new(roots: Vec<PathBuf>) -> Self {
        Self {
            roots,
            cache: HashMap::new(),
        }
    }

    /// Look up `path` in the cache; if absent, search the root directories for
    /// a matching `.lean` or `.oxilean` file.  Populates the cache on success.
    pub fn resolve(&mut self, path: &ModulePath) -> ModuleResolutionResult {
        if let Some(info) = self.cache.get(path) {
            return ModuleResolutionResult::Found(info.clone());
        }

        let lean_rel = path.to_file_path();
        let oxilean_rel = path.to_oxilean_file_path();

        for root in &self.roots {
            let lean_abs = root.join(&lean_rel);
            let oxilean_abs = root.join(&oxilean_rel);

            let found = if lean_abs.exists() {
                Some(lean_abs)
            } else if oxilean_abs.exists() {
                Some(oxilean_abs)
            } else {
                None
            };

            if let Some(file_path) = found {
                let info = ModuleInfo {
                    path: file_path,
                    exports: Vec::new(),
                    dependencies: Vec::new(),
                };
                self.cache.insert(path.clone(), info.clone());
                return ModuleResolutionResult::Found(info);
            }
        }

        ModuleResolutionResult::NotFound(path.clone())
    }

    /// Manually register a module (bypasses file-system search).
    pub fn register(&mut self, path: ModulePath, info: ModuleInfo) {
        self.cache.insert(path, info);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Import parsing
// ─────────────────────────────────────────────────────────────────────────────

/// Parse a single `import` line.
///
/// Supported syntaxes:
/// - `import Mathlib.Algebra.Ring`
/// - `import Mathlib.Algebra.Ring as MAR`
/// - `import Mathlib.Algebra.Ring (add, mul)`
///
/// Returns `None` if the line does not start with `import` or the module path
/// is malformed.
pub fn parse_import_decl(input: &str) -> Option<ImportDecl> {
    let trimmed = input.trim();
    let rest = trimmed.strip_prefix("import")?.trim_start();
    if rest.is_empty() {
        return None;
    }

    // Check for selective import: `Foo.Bar (f, g)`
    if let Some(paren_pos) = rest.find('(') {
        let module_str = rest[..paren_pos].trim();
        let module = ModulePath::parse(module_str)?;

        let after_open = rest[paren_pos + 1..].trim_end_matches(|c: char| c.is_whitespace());
        let inner = after_open.strip_suffix(')')?;
        let selective: Vec<String> = inner
            .split(',')
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect();

        return Some(ImportDecl {
            module,
            alias: None,
            selective,
        });
    }

    // Check for aliased import: `Foo.Bar as X`
    //   We split on whitespace and look for "as" keyword.
    let tokens: Vec<&str> = rest.split_whitespace().collect();
    match tokens.as_slice() {
        [module_str] => {
            let module = ModulePath::parse(module_str)?;
            Some(ImportDecl {
                module,
                alias: None,
                selective: Vec::new(),
            })
        }
        [module_str, "as", alias] => {
            let module = ModulePath::parse(module_str)?;
            Some(ImportDecl {
                module,
                alias: Some((*alias).to_owned()),
                selective: Vec::new(),
            })
        }
        _ => None,
    }
}

/// Extract all `import` declarations from a source file.
///
/// Lines that do not parse as valid import declarations are silently ignored.
pub fn parse_imports(source: &str) -> Vec<ImportDecl> {
    source
        .lines()
        .filter_map(|line| parse_import_decl(line.trim()))
        .collect()
}

// ─────────────────────────────────────────────────────────────────────────────
// Graph construction
// ─────────────────────────────────────────────────────────────────────────────

/// Build a [`ModuleGraph`] from all modules currently registered in `registry`.
pub fn build_module_graph(registry: &ModuleRegistry) -> ModuleGraph {
    let mut graph = ModuleGraph::default();

    for (module_path, info) in &registry.cache {
        graph.nodes.insert(module_path.clone(), info.clone());
        for dep in &info.dependencies {
            graph.edges.push((module_path.clone(), dep.clone()));
        }
    }

    graph
}

// ─────────────────────────────────────────────────────────────────────────────
// Graph queries
// ─────────────────────────────────────────────────────────────────────────────

/// Return the direct dependencies of `module` in `graph`.
pub fn direct_deps_of<'a>(graph: &'a ModuleGraph, module: &ModulePath) -> Vec<&'a ModulePath> {
    graph
        .edges
        .iter()
        .filter_map(|(from, to)| if from == module { Some(to) } else { None })
        .collect()
}

/// Return all modules transitively reachable from `module` (excluding itself).
///
/// Uses BFS over the dependency edges.
pub fn transitive_deps(graph: &ModuleGraph, module: &ModulePath) -> Vec<ModulePath> {
    let mut visited: HashSet<ModulePath> = HashSet::new();
    let mut queue: VecDeque<ModulePath> = VecDeque::new();

    for dep in direct_deps_of(graph, module) {
        if !visited.contains(dep) {
            visited.insert(dep.clone());
            queue.push_back(dep.clone());
        }
    }

    while let Some(current) = queue.pop_front() {
        for dep in direct_deps_of(graph, &current) {
            if !visited.contains(dep) {
                visited.insert(dep.clone());
                queue.push_back(dep.clone());
            }
        }
    }

    let mut result: Vec<ModulePath> = visited.into_iter().collect();
    result.sort();
    result
}

// ─────────────────────────────────────────────────────────────────────────────
// Topological sort (Kahn's algorithm)
// ─────────────────────────────────────────────────────────────────────────────

/// Produce a topological ordering of modules in `graph` so that every module
/// appears after all its dependencies.
///
/// Returns `Err(CycleError)` if a cycle exists (reports the first cycle
/// detected via DFS back-edge).
pub fn topological_sort(graph: &ModuleGraph) -> Result<Vec<ModulePath>, CycleError> {
    // Build adjacency and in-degree maps over the nodes present in the graph.
    let mut in_degree: HashMap<&ModulePath, usize> = HashMap::new();
    let mut adjacency: HashMap<&ModulePath, Vec<&ModulePath>> = HashMap::new();

    for node in graph.nodes.keys() {
        in_degree.entry(node).or_insert(0);
        adjacency.entry(node).or_default();
    }

    for (from, to) in &graph.edges {
        *in_degree.entry(to).or_insert(0) += 1;
        adjacency.entry(from).or_default().push(to);
        // Ensure `to` has an in-degree entry even if it has no outgoing edges.
        adjacency.entry(to).or_default();
    }

    // Kahn's algorithm.
    let mut queue: VecDeque<&ModulePath> = in_degree
        .iter()
        .filter_map(|(k, &v)| if v == 0 { Some(*k) } else { None })
        .collect();

    // Deterministic order.
    let mut queue_vec: Vec<&ModulePath> = queue.drain(..).collect();
    queue_vec.sort();
    let mut queue: VecDeque<&ModulePath> = queue_vec.into_iter().collect();

    let mut sorted: Vec<ModulePath> = Vec::new();

    while let Some(node) = queue.pop_front() {
        sorted.push(node.clone());
        if let Some(neighbors) = adjacency.get(node) {
            let mut next_batch: Vec<&ModulePath> = Vec::new();
            for &neighbor in neighbors {
                let deg = in_degree.entry(neighbor).or_insert(0);
                if *deg > 0 {
                    *deg -= 1;
                    if *deg == 0 {
                        next_batch.push(neighbor);
                    }
                }
            }
            next_batch.sort();
            for n in next_batch {
                queue.push_back(n);
            }
        }
    }

    if sorted.len() != graph.nodes.len() {
        // There are nodes not reachable by Kahn's — they are part of a cycle.
        let in_cycle: Vec<ModulePath> = graph
            .nodes
            .keys()
            .filter(|n| !sorted.contains(n))
            .cloned()
            .collect();

        // Find an actual cycle path using DFS.
        let cycle = find_one_cycle(graph, &in_cycle);
        return Err(CycleError { cycle });
    }

    Ok(sorted)
}

/// Find a single cycle among `candidates` using DFS.
fn find_one_cycle(graph: &ModuleGraph, candidates: &[ModulePath]) -> Vec<ModulePath> {
    let candidate_set: HashSet<&ModulePath> = candidates.iter().collect();
    let mut visited: HashSet<&ModulePath> = HashSet::new();
    let mut stack: Vec<&ModulePath> = Vec::new();

    for start in candidates {
        if !visited.contains(start) {
            if let Some(cycle) = dfs_cycle(graph, start, &candidate_set, &mut visited, &mut stack) {
                return cycle;
            }
        }
    }

    // Fallback — return candidates as-is (should not happen for a cyclic graph).
    candidates.to_vec()
}

fn dfs_cycle<'a>(
    graph: &'a ModuleGraph,
    node: &'a ModulePath,
    candidates: &HashSet<&'a ModulePath>,
    visited: &mut HashSet<&'a ModulePath>,
    stack: &mut Vec<&'a ModulePath>,
) -> Option<Vec<ModulePath>> {
    visited.insert(node);
    stack.push(node);

    for (from, to) in &graph.edges {
        if from != node {
            continue;
        }
        if !candidates.contains(to) {
            continue;
        }
        if let Some(pos) = stack.iter().position(|&s| s == to) {
            // Found back edge — extract the cycle.
            let cycle: Vec<ModulePath> = stack[pos..].iter().map(|&p| p.clone()).collect();
            return Some(cycle);
        }
        if !visited.contains(to) {
            if let Some(cycle) = dfs_cycle(graph, to, candidates, visited, stack) {
                return Some(cycle);
            }
        }
    }

    stack.pop();
    None
}

// ─────────────────────────────────────────────────────────────────────────────
// Cycle detection (all cycles)
// ─────────────────────────────────────────────────────────────────────────────

/// Detect *all* import cycles in `graph`.
///
/// Uses Johnson's algorithm skeleton: for each SCC of size > 1 (or with a
/// self-loop), reports a cycle.  We use Tarjan's SCC here and then extract
/// one representative cycle path per SCC.
pub fn detect_cycles(graph: &ModuleGraph) -> Vec<CycleError> {
    let sccs = tarjan_sccs(graph);
    let mut errors: Vec<CycleError> = Vec::new();

    for scc in sccs {
        if scc.len() == 1 {
            // Self-loop?
            let node = &scc[0];
            let self_loop = graph
                .edges
                .iter()
                .any(|(from, to)| from == node && to == node);
            if !self_loop {
                continue;
            }
        }
        // Extract representative cycle path within this SCC.
        let cycle = find_one_cycle(graph, &scc);
        errors.push(CycleError { cycle });
    }

    errors
}

/// Tarjan's strongly connected components algorithm.
fn tarjan_sccs(graph: &ModuleGraph) -> Vec<Vec<ModulePath>> {
    struct State<'a> {
        index_counter: usize,
        stack: Vec<&'a ModulePath>,
        on_stack: HashSet<&'a ModulePath>,
        index: HashMap<&'a ModulePath, usize>,
        lowlink: HashMap<&'a ModulePath, usize>,
        sccs: Vec<Vec<ModulePath>>,
    }

    fn strongconnect<'a>(v: &'a ModulePath, graph: &'a ModuleGraph, state: &mut State<'a>) {
        let idx = state.index_counter;
        state.index.insert(v, idx);
        state.lowlink.insert(v, idx);
        state.index_counter += 1;
        state.stack.push(v);
        state.on_stack.insert(v);

        for (from, to) in &graph.edges {
            if from != v {
                continue;
            }
            if !graph.nodes.contains_key(to) {
                continue;
            }
            if !state.index.contains_key(to) {
                strongconnect(to, graph, state);
                let ll_to = *state.lowlink.get(to).unwrap_or(&usize::MAX);
                let ll_v = state.lowlink.get(v).copied().unwrap_or(usize::MAX);
                state.lowlink.insert(v, ll_v.min(ll_to));
            } else if state.on_stack.contains(to) {
                let idx_to = *state.index.get(to).unwrap_or(&usize::MAX);
                let ll_v = state.lowlink.get(v).copied().unwrap_or(usize::MAX);
                state.lowlink.insert(v, ll_v.min(idx_to));
            }
        }

        // If v is a root of an SCC, pop the stack.
        let ll_v = *state.lowlink.get(v).unwrap_or(&usize::MAX);
        let idx_v = *state.index.get(v).unwrap_or(&usize::MAX);
        if ll_v == idx_v {
            let mut scc: Vec<ModulePath> = Vec::new();
            while let Some(w) = state.stack.pop() {
                state.on_stack.remove(w);
                scc.push(w.clone());
                if w == v {
                    break;
                }
            }
            state.sccs.push(scc);
        }
    }

    let mut state = State {
        index_counter: 0,
        stack: Vec::new(),
        on_stack: HashSet::new(),
        index: HashMap::new(),
        lowlink: HashMap::new(),
        sccs: Vec::new(),
    };

    let mut nodes: Vec<&ModulePath> = graph.nodes.keys().collect();
    nodes.sort();

    for node in nodes {
        if !state.index.contains_key(node) {
            strongconnect(node, graph, &mut state);
        }
    }

    state.sccs
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    // ── helpers ──────────────────────────────────────────────────────────────

    fn mp(s: &str) -> ModulePath {
        ModulePath::parse(s).expect("valid module path")
    }

    fn info_with_deps(deps: &[&str]) -> ModuleInfo {
        ModuleInfo {
            path: PathBuf::from("dummy.lean"),
            exports: Vec::new(),
            dependencies: deps.iter().map(|s| mp(s)).collect(),
        }
    }

    fn graph_from_edges(pairs: &[(&str, &str)]) -> ModuleGraph {
        let mut g = ModuleGraph::default();
        for &(from, to) in pairs {
            let f = mp(from);
            let t = mp(to);
            g.nodes
                .entry(f.clone())
                .or_insert_with(|| info_with_deps(&[]));
            g.nodes
                .entry(t.clone())
                .or_insert_with(|| info_with_deps(&[]));
            g.edges.push((f, t));
        }
        g
    }

    // ── ModulePath::parse ─────────────────────────────────────────────────

    #[test]
    fn test_module_path_from_str_single() {
        let p = ModulePath::parse("Std").expect("valid");
        assert_eq!(p.components, vec!["Std"]);
    }

    #[test]
    fn test_module_path_from_str_multi() {
        let p = mp("Mathlib.Algebra.Ring");
        assert_eq!(p.components, vec!["Mathlib", "Algebra", "Ring"]);
    }

    #[test]
    fn test_module_path_from_str_empty_is_none() {
        assert!(ModulePath::parse("").is_none());
    }

    #[test]
    fn test_module_path_from_str_double_dot_is_none() {
        assert!(ModulePath::parse("A..B").is_none());
    }

    #[test]
    fn test_module_path_from_str_trailing_dot_is_none() {
        assert!(ModulePath::parse("A.B.").is_none());
    }

    // ── ModulePath::to_file_path ─────────────────────────────────────────────

    #[test]
    fn test_to_file_path_single() {
        let p = mp("Std");
        assert_eq!(p.to_file_path(), PathBuf::from("Std.lean"));
    }

    #[test]
    fn test_to_file_path_multi() {
        let p = mp("Mathlib.Algebra.Ring");
        assert_eq!(p.to_file_path(), PathBuf::from("Mathlib/Algebra/Ring.lean"));
    }

    #[test]
    fn test_to_oxilean_file_path() {
        let p = mp("Foo.Bar");
        assert_eq!(p.to_oxilean_file_path(), PathBuf::from("Foo/Bar.oxilean"));
    }

    // ── ModulePath::to_string ────────────────────────────────────────────────

    #[test]
    fn test_to_string() {
        let p = mp("A.B.C");
        assert_eq!(p.to_string(), "A.B.C");
    }

    #[test]
    fn test_display() {
        let p = mp("X.Y");
        assert_eq!(format!("{p}"), "X.Y");
    }

    // ── parse_import_decl ────────────────────────────────────────────────────

    #[test]
    fn test_parse_import_bare() {
        let decl = parse_import_decl("import Foo.Bar").expect("valid");
        assert_eq!(decl.module, mp("Foo.Bar"));
        assert!(decl.alias.is_none());
        assert!(decl.selective.is_empty());
    }

    #[test]
    fn test_parse_import_alias() {
        let decl = parse_import_decl("import Foo.Bar as FB").expect("valid");
        assert_eq!(decl.module, mp("Foo.Bar"));
        assert_eq!(decl.alias.as_deref(), Some("FB"));
        assert!(decl.selective.is_empty());
    }

    #[test]
    fn test_parse_import_selective() {
        let decl = parse_import_decl("import Foo.Bar (add, mul)").expect("valid");
        assert_eq!(decl.module, mp("Foo.Bar"));
        assert!(decl.alias.is_none());
        assert_eq!(decl.selective, vec!["add", "mul"]);
    }

    #[test]
    fn test_parse_import_leading_whitespace() {
        let decl = parse_import_decl("  import Std").expect("valid");
        assert_eq!(decl.module, mp("Std"));
    }

    #[test]
    fn test_parse_import_not_import() {
        assert!(parse_import_decl("def foo := 1").is_none());
    }

    #[test]
    fn test_parse_import_empty_line() {
        assert!(parse_import_decl("").is_none());
    }

    // ── parse_imports ────────────────────────────────────────────────────────

    #[test]
    fn test_parse_imports_multiple() {
        let src = r#"
import Std
import Mathlib.Algebra.Ring as MAR
def foo := 1
import Foo (bar, baz)
"#;
        let imports = parse_imports(src);
        assert_eq!(imports.len(), 3);
        assert_eq!(imports[0].module, mp("Std"));
        assert_eq!(imports[1].alias.as_deref(), Some("MAR"));
        assert_eq!(imports[2].selective, vec!["bar", "baz"]);
    }

    #[test]
    fn test_parse_imports_empty_source() {
        assert!(parse_imports("").is_empty());
    }

    // ── ModuleRegistry ───────────────────────────────────────────────────────

    #[test]
    fn test_registry_new_empty() {
        let reg = ModuleRegistry::new(vec![]);
        assert!(reg.cache.is_empty());
    }

    #[test]
    fn test_registry_register_and_resolve() {
        let mut reg = ModuleRegistry::new(vec![]);
        let path = mp("Foo.Bar");
        let info = ModuleInfo {
            path: PathBuf::from("Foo/Bar.lean"),
            exports: vec!["baz".to_owned()],
            dependencies: vec![],
        };
        reg.register(path.clone(), info.clone());
        match reg.resolve(&path) {
            ModuleResolutionResult::Found(i) => assert_eq!(i.path, PathBuf::from("Foo/Bar.lean")),
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn test_registry_not_found() {
        let mut reg = ModuleRegistry::new(vec![]);
        let path = mp("Nonexistent");
        assert_eq!(reg.resolve(&path), ModuleResolutionResult::NotFound(path));
    }

    #[test]
    fn test_registry_resolve_from_filesystem() {
        let tmp = env::temp_dir().join("oxilean_test_registry_resolve");
        fs::create_dir_all(&tmp).expect("create temp dir");
        let module_dir = tmp.join("Foo");
        fs::create_dir_all(&module_dir).expect("create Foo dir");
        let lean_file = module_dir.join("Bar.lean");
        fs::write(&lean_file, "-- empty").expect("write lean file");

        let mut reg = ModuleRegistry::new(vec![tmp.clone()]);
        let path = mp("Foo.Bar");
        match reg.resolve(&path) {
            ModuleResolutionResult::Found(info) => {
                assert!(info.path.ends_with("Foo/Bar.lean"));
            }
            other => panic!("expected Found, got {other:?}"),
        }

        // Clean up.
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_registry_resolve_oxilean_extension() {
        let tmp = env::temp_dir().join("oxilean_test_registry_oxilean_ext");
        fs::create_dir_all(&tmp).expect("create temp dir");
        let module_dir = tmp.join("Ox");
        fs::create_dir_all(&module_dir).expect("create dir");
        let file = module_dir.join("Mod.oxilean");
        fs::write(&file, "-- empty").expect("write file");

        let mut reg = ModuleRegistry::new(vec![tmp.clone()]);
        let path = mp("Ox.Mod");
        match reg.resolve(&path) {
            ModuleResolutionResult::Found(info) => {
                assert!(info.path.ends_with("Ox/Mod.oxilean"));
            }
            other => panic!("expected Found, got {other:?}"),
        }

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_registry_cache_hit() {
        let tmp = env::temp_dir().join("oxilean_test_cache_hit");
        fs::create_dir_all(&tmp).expect("create temp dir");
        let file = tmp.join("Cache.lean");
        fs::write(&file, "").expect("write");

        let mut reg = ModuleRegistry::new(vec![tmp.clone()]);
        let path = mp("Cache");

        // First resolution populates cache.
        let r1 = reg.resolve(&path);
        // Delete the file — second resolution must still succeed from cache.
        let _ = fs::remove_file(&file);
        let r2 = reg.resolve(&path);

        assert_eq!(r1, r2);
        let _ = fs::remove_dir_all(&tmp);
    }

    // ── build_module_graph ───────────────────────────────────────────────────

    #[test]
    fn test_build_module_graph_empty() {
        let reg = ModuleRegistry::new(vec![]);
        let g = build_module_graph(&reg);
        assert!(g.nodes.is_empty());
        assert!(g.edges.is_empty());
    }

    #[test]
    fn test_build_module_graph_nodes_and_edges() {
        let mut reg = ModuleRegistry::new(vec![]);
        reg.register(
            mp("A"),
            ModuleInfo {
                path: PathBuf::from("A.lean"),
                exports: vec![],
                dependencies: vec![mp("B")],
            },
        );
        reg.register(
            mp("B"),
            ModuleInfo {
                path: PathBuf::from("B.lean"),
                exports: vec![],
                dependencies: vec![],
            },
        );
        let g = build_module_graph(&reg);
        assert_eq!(g.nodes.len(), 2);
        assert!(g.edges.contains(&(mp("A"), mp("B"))));
    }

    // ── direct_deps_of ───────────────────────────────────────────────────────

    #[test]
    fn test_direct_deps_of_empty() {
        let g = graph_from_edges(&[]);
        let deps = direct_deps_of(&g, &mp("A"));
        assert!(deps.is_empty());
    }

    #[test]
    fn test_direct_deps_of_one() {
        let g = graph_from_edges(&[("A", "B"), ("A", "C"), ("B", "C")]);
        let deps = direct_deps_of(&g, &mp("A"));
        let mut names: Vec<String> = deps.iter().map(|p| p.to_string()).collect();
        names.sort();
        assert_eq!(names, vec!["B", "C"]);
    }

    // ── transitive_deps ──────────────────────────────────────────────────────

    #[test]
    fn test_transitive_deps_chain() {
        // A → B → C
        let g = graph_from_edges(&[("A", "B"), ("B", "C")]);
        let mut deps = transitive_deps(&g, &mp("A"));
        deps.sort();
        assert!(deps.contains(&mp("B")));
        assert!(deps.contains(&mp("C")));
        assert!(!deps.contains(&mp("A")));
    }

    #[test]
    fn test_transitive_deps_diamond() {
        // A → B, A → C, B → D, C → D
        let g = graph_from_edges(&[("A", "B"), ("A", "C"), ("B", "D"), ("C", "D")]);
        let deps = transitive_deps(&g, &mp("A"));
        assert!(deps.contains(&mp("B")));
        assert!(deps.contains(&mp("C")));
        assert!(deps.contains(&mp("D")));
        assert_eq!(deps.len(), 3);
    }

    // ── topological_sort ─────────────────────────────────────────────────────

    #[test]
    fn test_topo_sort_empty() {
        let g = ModuleGraph::default();
        let sorted = topological_sort(&g).expect("no cycle");
        assert!(sorted.is_empty());
    }

    #[test]
    fn test_topo_sort_chain() {
        // A → B → C  (A imports B, B imports C)
        // Kahn's algorithm (importer-first): nodes with no incoming edges come
        // first.  A has no incoming edges → A first, then B, then C.
        let g = graph_from_edges(&[("A", "B"), ("B", "C")]);
        let sorted = topological_sort(&g).expect("no cycle");
        let pos_a = sorted
            .iter()
            .position(|p| p == &mp("A"))
            .expect("A present");
        let pos_b = sorted
            .iter()
            .position(|p| p == &mp("B"))
            .expect("B present");
        let pos_c = sorted
            .iter()
            .position(|p| p == &mp("C"))
            .expect("C present");
        // A comes before B, and B comes before C (importer precedes importee).
        assert!(pos_a < pos_b, "A should precede B; sorted={sorted:?}");
        assert!(pos_b < pos_c, "B should precede C; sorted={sorted:?}");
    }

    #[test]
    fn test_topo_sort_cycle_returns_err() {
        // A → B → A
        let g = graph_from_edges(&[("A", "B"), ("B", "A")]);
        assert!(topological_sort(&g).is_err());
    }

    // ── detect_cycles ────────────────────────────────────────────────────────

    #[test]
    fn test_detect_cycles_none() {
        let g = graph_from_edges(&[("A", "B"), ("B", "C")]);
        assert!(detect_cycles(&g).is_empty());
    }

    #[test]
    fn test_detect_cycles_simple() {
        // A → B → A
        let g = graph_from_edges(&[("A", "B"), ("B", "A")]);
        let cycles = detect_cycles(&g);
        assert!(!cycles.is_empty());
        // The cycle should contain both A and B.
        let all_nodes: Vec<ModulePath> = cycles.into_iter().flat_map(|c| c.cycle).collect();
        assert!(all_nodes.contains(&mp("A")) || all_nodes.contains(&mp("B")));
    }

    #[test]
    fn test_detect_cycles_self_loop() {
        let mut g = ModuleGraph::default();
        g.nodes.insert(mp("A"), info_with_deps(&[]));
        g.edges.push((mp("A"), mp("A")));
        let cycles = detect_cycles(&g);
        assert!(!cycles.is_empty());
    }

    #[test]
    fn test_detect_cycles_two_distinct() {
        // A → B → A  and  C → D → C
        let g = graph_from_edges(&[("A", "B"), ("B", "A"), ("C", "D"), ("D", "C")]);
        let cycles = detect_cycles(&g);
        // At least two SCCs with cycles.
        assert!(cycles.len() >= 2);
    }

    // ── CycleError display ───────────────────────────────────────────────────

    #[test]
    fn test_cycle_error_display() {
        let err = CycleError {
            cycle: vec![mp("A"), mp("B"), mp("C")],
        };
        let s = format!("{err}");
        assert!(s.contains("A"));
        assert!(s.contains("B"));
        assert!(s.contains("C"));
    }
}
