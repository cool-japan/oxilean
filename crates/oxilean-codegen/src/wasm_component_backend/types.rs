//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

/// Content of a core WebAssembly module.
#[derive(Debug, Clone, PartialEq)]
pub enum CoreModuleContent {
    /// Inline WAT text
    Text(String),
    /// Reference to external binary file
    BinaryRef(String),
    /// Empty placeholder
    Empty,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WCAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, WCCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl WCAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        WCAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&WCCacheEntry> {
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
            WCCacheEntry {
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
/// Analysis cache for WasmCExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct WasmCExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl WasmCExtCache {
    #[allow(dead_code)]
    pub fn new(cap: usize) -> Self {
        Self {
            entries: Vec::new(),
            cap,
            total_hits: 0,
            total_misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: u64) -> Option<&[u8]> {
        for e in self.entries.iter_mut() {
            if e.0 == key && e.2 {
                e.3 += 1;
                self.total_hits += 1;
                return Some(&e.1);
            }
        }
        self.total_misses += 1;
        None
    }
    #[allow(dead_code)]
    pub fn put(&mut self, key: u64, data: Vec<u8>) {
        if self.entries.len() >= self.cap {
            self.entries.retain(|e| e.2);
            if self.entries.len() >= self.cap {
                self.entries.remove(0);
            }
        }
        self.entries.push((key, data, true, 0));
    }
    #[allow(dead_code)]
    pub fn invalidate(&mut self) {
        for e in self.entries.iter_mut() {
            e.2 = false;
        }
    }
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let t = self.total_hits + self.total_misses;
        if t == 0 {
            0.0
        } else {
            self.total_hits as f64 / t as f64
        }
    }
    #[allow(dead_code)]
    pub fn live_count(&self) -> usize {
        self.entries.iter().filter(|e| e.2).count()
    }
}
/// Severity of a WasmCompExt diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WasmCompExtDiagSeverity {
    Note,
    Warning,
    Error,
}
/// The kind of a component export.
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentExportKind {
    /// Export a function
    Func,
    /// Export a type definition
    Type,
    /// Export a nested component instance
    Instance,
    /// Export a core module
    Module,
    /// Export a value
    Value,
    /// Export a component
    Component,
}
/// Heuristic freshness key for WasmCompExt incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WasmCompExtIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
impl WasmCompExtIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        WasmCompExtIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &WasmCompExtIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
/// A version tag for WasmCompExt output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasmCompExtVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
impl WasmCompExtVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        WasmCompExtVersion {
            major,
            minor,
            patch,
            pre: None,
        }
    }
    pub fn with_pre(mut self, pre: impl Into<String>) -> Self {
        self.pre = Some(pre.into());
        self
    }
    pub fn is_stable(&self) -> bool {
        self.pre.is_none()
    }
    pub fn is_compatible_with(&self, other: &WasmCompExtVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WCPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl WCPassStats {
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
/// Collects WasmCompExt diagnostics.
#[derive(Debug, Default)]
pub struct WasmCompExtDiagCollector {
    pub(super) msgs: Vec<WasmCompExtDiagMsg>,
}
impl WasmCompExtDiagCollector {
    pub fn new() -> Self {
        WasmCompExtDiagCollector::default()
    }
    pub fn emit(&mut self, d: WasmCompExtDiagMsg) {
        self.msgs.push(d);
    }
    pub fn has_errors(&self) -> bool {
        self.msgs
            .iter()
            .any(|d| d.severity == WasmCompExtDiagSeverity::Error)
    }
    pub fn errors(&self) -> Vec<&WasmCompExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == WasmCompExtDiagSeverity::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&WasmCompExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == WasmCompExtDiagSeverity::Warning)
            .collect()
    }
    pub fn len(&self) -> usize {
        self.msgs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.msgs.is_empty()
    }
    pub fn clear(&mut self) {
        self.msgs.clear();
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WCCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A core WebAssembly module embedded within a component.
#[derive(Debug, Clone, PartialEq)]
pub struct CoreModule {
    /// Internal name for this module
    pub name: String,
    /// Module binary (bytes) or text (WAT)
    pub content: CoreModuleContent,
    /// Exports provided by this module to the component
    pub exports: Vec<String>,
}
impl CoreModule {
    /// Create a new core module with inline text.
    pub fn inline_text(name: impl Into<String>, wat: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: CoreModuleContent::Text(wat.into()),
            exports: Vec::new(),
        }
    }
    /// Add an export to this core module.
    pub fn with_export(mut self, export: impl Into<String>) -> Self {
        self.exports.push(export.into());
        self
    }
    /// Emit the core module declaration.
    pub fn emit(&self) -> String {
        match &self.content {
            CoreModuleContent::Text(wat) => {
                format!("(core module ${}  ;; inline WAT\n  {}\n)", self.name, wat)
            }
            CoreModuleContent::BinaryRef(path) => {
                format!("(core module ${} (from \"{}\"))", self.name, path)
            }
            CoreModuleContent::Empty => format!("(core module ${})", self.name),
        }
    }
}
/// A single import into a WebAssembly component.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentImport {
    /// Import namespace (e.g., "wasi:io/streams")
    pub namespace: String,
    /// Local name within the namespace
    pub name: String,
    /// Type of the imported item
    pub ty: WasmComponentType,
}
impl ComponentImport {
    /// Create a new function import.
    pub fn func(
        namespace: impl Into<String>,
        name: impl Into<String>,
        params: Vec<(String, WasmComponentType)>,
        results: Vec<(String, WasmComponentType)>,
    ) -> Self {
        Self {
            namespace: namespace.into(),
            name: name.into(),
            ty: WasmComponentType::Func(params, results),
        }
    }
    /// Emit the WAT import declaration.
    pub fn emit(&self) -> String {
        format!(
            "(import \"{}/{}\" (func (type {})))",
            self.namespace, self.name, self.ty
        )
    }
}
/// Dominator tree for WasmCExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmCExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl WasmCExtDomTree {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            idom: vec![None; n],
            children: vec![Vec::new(); n],
            depth: vec![0; n],
        }
    }
    #[allow(dead_code)]
    pub fn set_idom(&mut self, node: usize, dom: usize) {
        if node < self.idom.len() {
            self.idom[node] = Some(dom);
            if dom < self.children.len() {
                self.children[dom].push(node);
            }
            self.depth[node] = if dom < self.depth.len() {
                self.depth[dom] + 1
            } else {
                1
            };
        }
    }
    #[allow(dead_code)]
    pub fn dominates(&self, a: usize, mut b: usize) -> bool {
        if a == b {
            return true;
        }
        let n = self.idom.len();
        for _ in 0..n {
            match self.idom.get(b).copied().flatten() {
                None => return false,
                Some(p) if p == a => return true,
                Some(p) if p == b => return false,
                Some(p) => b = p,
            }
        }
        false
    }
    #[allow(dead_code)]
    pub fn children_of(&self, n: usize) -> &[usize] {
        self.children.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn depth_of(&self, n: usize) -> usize {
        self.depth.get(n).copied().unwrap_or(0)
    }
    #[allow(dead_code)]
    pub fn lca(&self, mut a: usize, mut b: usize) -> usize {
        let n = self.idom.len();
        for _ in 0..(2 * n) {
            if a == b {
                return a;
            }
            if self.depth_of(a) > self.depth_of(b) {
                a = self.idom.get(a).and_then(|x| *x).unwrap_or(a);
            } else {
                b = self.idom.get(b).and_then(|x| *x).unwrap_or(b);
            }
        }
        0
    }
}
/// Emission statistics for WasmCompExt.
#[derive(Debug, Clone, Default)]
pub struct WasmCompExtEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
impl WasmCompExtEmitStats {
    pub fn new() -> Self {
        WasmCompExtEmitStats::default()
    }
    pub fn throughput_bps(&self) -> f64 {
        if self.elapsed_ms == 0 {
            0.0
        } else {
            self.bytes_emitted as f64 / (self.elapsed_ms as f64 / 1000.0)
        }
    }
    pub fn is_clean(&self) -> bool {
        self.errors == 0
    }
}
/// The main WebAssembly Component Model code generation backend.
///
/// Translates OxiLean declarations into Component Model WIT definitions
/// and component binary layouts.
#[derive(Debug)]
pub struct WasmComponentBackend {
    /// Component instance being built
    pub component: ComponentInstance,
    /// WIT interfaces being generated
    pub interfaces: Vec<WitInterface>,
    /// Counter for generating unique IDs
    pub(super) next_id: u32,
    /// Canonical options for default lift/lower operations
    pub canonical_opts: CanonicalOptions,
    /// Generated type aliases
    pub(super) type_aliases: HashMap<String, WasmComponentType>,
    /// Component-level function definitions
    pub(super) component_funcs: Vec<ComponentFunc>,
}
impl WasmComponentBackend {
    /// Create a new Component Model backend.
    pub fn new(component_name: impl Into<String>) -> Self {
        Self {
            component: ComponentInstance::new(component_name),
            interfaces: Vec::new(),
            next_id: 0,
            canonical_opts: CanonicalOptions::default(),
            type_aliases: HashMap::new(),
            component_funcs: Vec::new(),
        }
    }
    /// Generate a fresh unique ID.
    pub fn fresh_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }
    /// Set the default canonical options (memory + realloc).
    pub fn set_canonical_opts(&mut self, memory: impl Into<String>, realloc: impl Into<String>) {
        self.canonical_opts = CanonicalOptions::with_memory_and_realloc(memory, realloc);
    }
    /// Define a named type alias.
    pub fn define_type(&mut self, name: impl Into<String>, ty: WasmComponentType) {
        let name = name.into();
        self.component.define_type(name.clone(), ty.clone());
        self.type_aliases.insert(name, ty);
    }
    /// Add a WIT interface.
    pub fn add_interface(&mut self, iface: WitInterface) {
        self.interfaces.push(iface);
    }
    /// Add a component function lifted from a core function.
    pub fn add_lifted_func(
        &mut self,
        name: impl Into<String>,
        core_func: impl Into<String>,
        params: Vec<(String, WasmComponentType)>,
        results: Vec<(String, WasmComponentType)>,
    ) {
        let opts = self.canonical_opts.clone();
        let func = ComponentFunc::lifted(name, core_func, params, results, opts);
        self.component_funcs.push(func);
    }
    /// Add a component import (lowered from a component import).
    pub fn add_lowered_import(
        &mut self,
        namespace: impl Into<String>,
        name: impl Into<String>,
        params: Vec<(String, WasmComponentType)>,
        results: Vec<(String, WasmComponentType)>,
    ) {
        let ns: String = namespace.into();
        let n: String = name.into();
        let import = ComponentImport::func(ns, n.clone(), params.clone(), results.clone());
        self.component.add_import(import);
        let func = ComponentFunc::lowered(n, params, results);
        self.component_funcs.push(func);
    }
    /// Add a core module to the component.
    pub fn add_core_module(&mut self, module: CoreModule) {
        self.component.add_core_module(module);
    }
    /// Add an export from this component.
    pub fn add_export(&mut self, export: ComponentExport) {
        self.component.add_export(export);
    }
    /// Emit the complete component in WAT text format.
    pub fn emit_component_wat(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("(component ${}", self.component.name));
        for import in &self.component.imports {
            lines.push(format!("  {}", import.emit()));
        }
        let mut sorted_types: Vec<(&String, &WasmComponentType)> =
            self.component.type_defs.iter().collect();
        sorted_types.sort_by_key(|(k, _)| k.as_str());
        for (name, ty) in sorted_types {
            lines.push(format!("  (type ${} {})", name, ty));
        }
        for module in &self.component.core_modules {
            for line in module.emit().lines() {
                lines.push(format!("  {}", line));
            }
        }
        for func in &self.component_funcs {
            lines.push(format!("  {}", func.emit_canon()));
        }
        for export in &self.component.exports {
            lines.push(format!("  {}", export.emit()));
        }
        lines.push(")".to_string());
        lines.join("\n")
    }
    /// Emit all WIT interface definitions.
    pub fn emit_wit(&self) -> String {
        let mut parts = Vec::new();
        for iface in &self.interfaces {
            parts.push(iface.emit_wit());
        }
        parts.join("\n\n")
    }
    /// Emit a complete WIT package definition.
    pub fn emit_wit_package(&self, package_name: &str, package_version: Option<&str>) -> String {
        let mut lines = Vec::new();
        let header = match package_version {
            Some(v) => format!("package {}@{};", package_name, v),
            None => format!("package {};", package_name),
        };
        lines.push(header);
        lines.push(String::new());
        for iface in &self.interfaces {
            lines.push(iface.emit_wit());
            lines.push(String::new());
        }
        lines.join("\n")
    }
    /// Report the number of component functions.
    pub fn func_count(&self) -> usize {
        self.component_funcs.len()
    }
    /// Report the number of interfaces.
    pub fn interface_count(&self) -> usize {
        self.interfaces.len()
    }
    /// Check whether a named type is defined.
    pub fn has_type(&self, name: &str) -> bool {
        self.type_aliases.contains_key(name)
    }
}
/// A monotonically increasing ID generator for WasmCompExt.
#[derive(Debug, Default)]
pub struct WasmCompExtIdGen {
    pub(super) next: u32,
}
impl WasmCompExtIdGen {
    pub fn new() -> Self {
        WasmCompExtIdGen::default()
    }
    pub fn next_id(&mut self) -> u32 {
        let id = self.next;
        self.next += 1;
        id
    }
    pub fn peek_next(&self) -> u32 {
        self.next
    }
    pub fn reset(&mut self) {
        self.next = 0;
    }
    pub fn skip(&mut self, n: u32) {
        self.next += n;
    }
}
/// A generic key-value configuration store for WasmCompExt.
#[derive(Debug, Clone, Default)]
pub struct WasmCompExtConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
}
impl WasmCompExtConfig {
    pub fn new() -> Self {
        WasmCompExtConfig::default()
    }
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.entries.insert(key.into(), value.into());
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.get(key).map(|s| s.as_str())
    }
    pub fn get_bool(&self, key: &str) -> bool {
        matches!(self.get(key), Some("true") | Some("1") | Some("yes"))
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.parse().ok()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum WCPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl WCPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            WCPassPhase::Analysis => "analysis",
            WCPassPhase::Transformation => "transformation",
            WCPassPhase::Verification => "verification",
            WCPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, WCPassPhase::Transformation | WCPassPhase::Cleanup)
    }
}
/// Pass execution phase for WasmCExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WasmCExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl WasmCExtPassPhase {
    #[allow(dead_code)]
    pub fn is_early(&self) -> bool {
        matches!(self, Self::Early)
    }
    #[allow(dead_code)]
    pub fn is_middle(&self) -> bool {
        matches!(self, Self::Middle)
    }
    #[allow(dead_code)]
    pub fn is_late(&self) -> bool {
        matches!(self, Self::Late)
    }
    #[allow(dead_code)]
    pub fn is_finalize(&self) -> bool {
        matches!(self, Self::Finalize)
    }
    #[allow(dead_code)]
    pub fn order(&self) -> u32 {
        match self {
            Self::Early => 0,
            Self::Middle => 1,
            Self::Late => 2,
            Self::Finalize => 3,
        }
    }
    #[allow(dead_code)]
    pub fn from_order(n: u32) -> Option<Self> {
        match n {
            0 => Some(Self::Early),
            1 => Some(Self::Middle),
            2 => Some(Self::Late),
            3 => Some(Self::Finalize),
            _ => None,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WCDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl WCDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        WCDepGraph {
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
/// A fixed-capacity ring buffer of strings (for recent-event logging in WasmCompExt).
#[derive(Debug)]
pub struct WasmCompExtEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
}
impl WasmCompExtEventLog {
    pub fn new(capacity: usize) -> Self {
        WasmCompExtEventLog {
            entries: std::collections::VecDeque::with_capacity(capacity),
            capacity,
        }
    }
    pub fn push(&mut self, event: impl Into<String>) {
        if self.entries.len() >= self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(event.into());
    }
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.entries.iter()
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
/// A diagnostic message from a WasmCompExt pass.
#[derive(Debug, Clone)]
pub struct WasmCompExtDiagMsg {
    pub severity: WasmCompExtDiagSeverity,
    pub pass: String,
    pub message: String,
}
impl WasmCompExtDiagMsg {
    pub fn error(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        WasmCompExtDiagMsg {
            severity: WasmCompExtDiagSeverity::Error,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn warning(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        WasmCompExtDiagMsg {
            severity: WasmCompExtDiagSeverity::Warning,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn note(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        WasmCompExtDiagMsg {
            severity: WasmCompExtDiagSeverity::Note,
            pass: pass.into(),
            message: msg.into(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WCLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl WCLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        WCLivenessInfo {
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
/// A feature flag set for WasmCompExt capabilities.
#[derive(Debug, Clone, Default)]
pub struct WasmCompExtFeatures {
    pub(super) flags: std::collections::HashSet<String>,
}
impl WasmCompExtFeatures {
    pub fn new() -> Self {
        WasmCompExtFeatures::default()
    }
    pub fn enable(&mut self, flag: impl Into<String>) {
        self.flags.insert(flag.into());
    }
    pub fn disable(&mut self, flag: &str) {
        self.flags.remove(flag);
    }
    pub fn is_enabled(&self, flag: &str) -> bool {
        self.flags.contains(flag)
    }
    pub fn len(&self) -> usize {
        self.flags.len()
    }
    pub fn is_empty(&self) -> bool {
        self.flags.is_empty()
    }
    pub fn union(&self, other: &WasmCompExtFeatures) -> WasmCompExtFeatures {
        WasmCompExtFeatures {
            flags: self.flags.union(&other.flags).cloned().collect(),
        }
    }
    pub fn intersection(&self, other: &WasmCompExtFeatures) -> WasmCompExtFeatures {
        WasmCompExtFeatures {
            flags: self.flags.intersection(&other.flags).cloned().collect(),
        }
    }
}
/// Worklist for WasmCExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmCExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl WasmCExtWorklist {
    #[allow(dead_code)]
    pub fn new(capacity: usize) -> Self {
        Self {
            items: std::collections::VecDeque::new(),
            present: vec![false; capacity],
        }
    }
    #[allow(dead_code)]
    pub fn push(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_back(id);
        }
    }
    #[allow(dead_code)]
    pub fn push_front(&mut self, id: usize) {
        if id < self.present.len() && !self.present[id] {
            self.present[id] = true;
            self.items.push_front(id);
        }
    }
    #[allow(dead_code)]
    pub fn pop(&mut self) -> Option<usize> {
        let id = self.items.pop_front()?;
        if id < self.present.len() {
            self.present[id] = false;
        }
        Some(id)
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
    pub fn contains(&self, id: usize) -> bool {
        id < self.present.len() && self.present[id]
    }
    #[allow(dead_code)]
    pub fn drain_all(&mut self) -> Vec<usize> {
        let v: Vec<usize> = self.items.drain(..).collect();
        for &id in &v {
            if id < self.present.len() {
                self.present[id] = false;
            }
        }
        v
    }
}
/// Pipeline profiler for WasmCompExt.
#[derive(Debug, Default)]
pub struct WasmCompExtProfiler {
    pub(super) timings: Vec<WasmCompExtPassTiming>,
}
impl WasmCompExtProfiler {
    pub fn new() -> Self {
        WasmCompExtProfiler::default()
    }
    pub fn record(&mut self, t: WasmCompExtPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&WasmCompExtPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&WasmCompExtPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
/// WIT (WebAssembly Interface Types) interface definition.
///
/// A WIT interface groups related types and functions that can be
/// imported or exported by a component.
#[derive(Debug, Clone)]
pub struct WitInterface {
    /// Name of the interface (e.g., "streams", "filesystem")
    pub name: String,
    /// Namespace (e.g., "wasi:io")
    pub namespace: String,
    /// Version string (e.g., "0.2.0")
    pub version: Option<String>,
    /// Type definitions in this interface
    pub types: Vec<(String, WasmComponentType)>,
    /// Function signatures exported by this interface
    pub functions: Vec<(String, WasmComponentType)>,
    /// Resource definitions
    pub resources: Vec<WitResource>,
    /// Whether this is a world (top-level) or interface
    pub is_world: bool,
}
impl WitInterface {
    /// Create a new WIT interface.
    pub fn new(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            namespace: namespace.into(),
            version: None,
            types: Vec::new(),
            functions: Vec::new(),
            resources: Vec::new(),
            is_world: false,
        }
    }
    /// Create a new WIT world.
    pub fn world(namespace: impl Into<String>, name: impl Into<String>) -> Self {
        let mut iface = Self::new(namespace, name);
        iface.is_world = true;
        iface
    }
    /// Set the version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
    /// Add a type definition.
    pub fn add_type(&mut self, name: impl Into<String>, ty: WasmComponentType) {
        self.types.push((name.into(), ty));
    }
    /// Add a function signature.
    pub fn add_function(&mut self, name: impl Into<String>, ty: WasmComponentType) {
        self.functions.push((name.into(), ty));
    }
    /// Add a resource definition.
    pub fn add_resource(&mut self, resource: WitResource) {
        self.resources.push(resource);
    }
    /// Get the fully-qualified interface ID.
    pub fn id(&self) -> String {
        match &self.version {
            Some(v) => format!("{}{}@{}", self.namespace, self.name, v),
            None => format!("{}{}", self.namespace, self.name),
        }
    }
    /// Emit the WIT textual representation.
    pub fn emit_wit(&self) -> String {
        let mut lines = Vec::new();
        let keyword = if self.is_world { "world" } else { "interface" };
        let header = match &self.version {
            Some(v) => format!("{} {} {{  // {}", keyword, self.name, v),
            None => format!("{} {} {{", keyword, self.name),
        };
        lines.push(header);
        for (name, ty) in &self.types {
            lines.push(format!("  type {} = {};", name, ty));
        }
        for resource in &self.resources {
            for line in resource.emit_wit().lines() {
                lines.push(format!("  {}", line));
            }
        }
        for (name, ty) in &self.functions {
            lines.push(format!("  {}: {};", name, ty));
        }
        lines.push("}".to_string());
        lines.join("\n")
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WCDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl WCDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        WCDominatorTree {
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
#[allow(dead_code)]
pub struct WCConstantFoldingHelper;
impl WCConstantFoldingHelper {
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
/// Dependency graph for WasmCExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmCExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl WasmCExtDepGraph {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); n],
            rev: vec![Vec::new(); n],
            edge_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_edge(&mut self, from: usize, to: usize) {
        if from < self.n && to < self.n {
            if !self.adj[from].contains(&to) {
                self.adj[from].push(to);
                self.rev[to].push(from);
                self.edge_count += 1;
            }
        }
    }
    #[allow(dead_code)]
    pub fn succs(&self, n: usize) -> &[usize] {
        self.adj.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn preds(&self, n: usize) -> &[usize] {
        self.rev.get(n).map(|v| v.as_slice()).unwrap_or(&[])
    }
    #[allow(dead_code)]
    pub fn topo_sort(&self) -> Option<Vec<usize>> {
        let mut deg: Vec<usize> = (0..self.n).map(|i| self.rev[i].len()).collect();
        let mut q: std::collections::VecDeque<usize> =
            (0..self.n).filter(|&i| deg[i] == 0).collect();
        let mut out = Vec::with_capacity(self.n);
        while let Some(u) = q.pop_front() {
            out.push(u);
            for &v in &self.adj[u] {
                deg[v] -= 1;
                if deg[v] == 0 {
                    q.push_back(v);
                }
            }
        }
        if out.len() == self.n {
            Some(out)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn has_cycle(&self) -> bool {
        self.topo_sort().is_none()
    }
    #[allow(dead_code)]
    pub fn reachable(&self, start: usize) -> Vec<usize> {
        let mut vis = vec![false; self.n];
        let mut stk = vec![start];
        let mut out = Vec::new();
        while let Some(u) = stk.pop() {
            if u < self.n && !vis[u] {
                vis[u] = true;
                out.push(u);
                for &v in &self.adj[u] {
                    if !vis[v] {
                        stk.push(v);
                    }
                }
            }
        }
        out
    }
    #[allow(dead_code)]
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.n];
        let mut order = Vec::new();
        for i in 0..self.n {
            if !visited[i] {
                let mut stk = vec![(i, 0usize)];
                while let Some((u, idx)) = stk.last_mut() {
                    if !visited[*u] {
                        visited[*u] = true;
                    }
                    if *idx < self.adj[*u].len() {
                        let v = self.adj[*u][*idx];
                        *idx += 1;
                        if !visited[v] {
                            stk.push((v, 0));
                        }
                    } else {
                        order.push(*u);
                        stk.pop();
                    }
                }
            }
        }
        let mut comp = vec![usize::MAX; self.n];
        let mut components: Vec<Vec<usize>> = Vec::new();
        for &start in order.iter().rev() {
            if comp[start] == usize::MAX {
                let cid = components.len();
                let mut component = Vec::new();
                let mut stk = vec![start];
                while let Some(u) = stk.pop() {
                    if comp[u] == usize::MAX {
                        comp[u] = cid;
                        component.push(u);
                        for &v in &self.rev[u] {
                            if comp[v] == usize::MAX {
                                stk.push(v);
                            }
                        }
                    }
                }
                components.push(component);
            }
        }
        components
    }
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.n
    }
    #[allow(dead_code)]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }
}
/// Expressions in the Component Model canonical ABI.
///
/// These represent the lift/lower operations and resource management
/// primitives that mediate between core WebAssembly and component-level types.
#[derive(Debug, Clone, PartialEq)]
pub enum WasmComponentExpr {
    /// Lift a core WebAssembly value to a component value
    Lift {
        /// Core module function being lifted
        func: String,
        /// Memory to use for string/list encoding
        memory: Option<String>,
        /// Realloc function for memory allocation
        realloc: Option<String>,
        /// String encoding (utf8, utf16, latin1+utf16)
        string_encoding: Option<String>,
        /// Result type after lifting
        result_type: WasmComponentType,
    },
    /// Lower a component value to core WebAssembly value
    Lower {
        /// Component function being lowered
        func: String,
        /// Memory to use for string/list decoding
        memory: Option<String>,
        /// Realloc function for memory allocation
        realloc: Option<String>,
        /// String encoding
        string_encoding: Option<String>,
    },
    /// Create a new resource handle
    ResourceNew {
        /// Resource type name
        resource: String,
        /// Core function implementing constructor
        core_func: String,
    },
    /// Drop a resource handle (destroy it)
    ResourceDrop {
        /// Resource type name
        resource: String,
    },
    /// Get the core representation of a resource handle
    ResourceRep {
        /// Resource type name
        resource: String,
    },
    /// Call a function in a component instance
    Call {
        /// Instance alias
        instance: String,
        /// Exported function name
        func: String,
        /// Arguments (as component-level expressions)
        args: Vec<WasmComponentExpr>,
    },
    /// Integer literal
    IntLit(i64),
    /// Float literal
    FloatLit(f64),
    /// String literal
    StringLit(String),
    /// Boolean literal
    BoolLit(bool),
    /// Variable reference
    Var(String),
    /// Record construction
    RecordNew(Vec<(String, WasmComponentExpr)>),
    /// Field access on a record
    FieldGet(Box<WasmComponentExpr>, String),
    /// Variant construction
    VariantNew(String, Box<std::option::Option<WasmComponentExpr>>),
    /// Option some value
    OptionSome(Box<WasmComponentExpr>),
    /// Option none
    OptionNone,
    /// Result ok
    ResultOk(Box<std::option::Option<WasmComponentExpr>>),
    /// Result error
    ResultErr(Box<std::option::Option<WasmComponentExpr>>),
    /// List literal
    ListNew(Vec<WasmComponentExpr>),
    /// Tuple construction
    TupleNew(Vec<WasmComponentExpr>),
}
/// Types in the WebAssembly Component Model type system.
///
/// The Component Model has a richer type system than core WebAssembly,
/// including structured types, algebraic types, and resource handles.
#[derive(Debug, Clone, PartialEq)]
pub enum WasmComponentType {
    /// Boolean type
    Bool,
    /// Signed 8-bit integer
    S8,
    /// Unsigned 8-bit integer
    U8,
    /// Signed 16-bit integer
    S16,
    /// Unsigned 16-bit integer
    U16,
    /// Signed 32-bit integer
    S32,
    /// Unsigned 32-bit integer
    U32,
    /// Signed 64-bit integer
    S64,
    /// Unsigned 64-bit integer
    U64,
    /// 32-bit float
    F32,
    /// 64-bit float
    F64,
    /// Unicode character (U+0000 to U+10FFFF)
    Char,
    /// UTF-8 string
    String,
    /// Record type: named fields with associated types
    Record(Vec<(String, WasmComponentType)>),
    /// Variant type: discriminated union with optional payloads
    Variant(Vec<(String, Option<WasmComponentType>)>),
    /// List type: homogeneous sequence
    List(Box<WasmComponentType>),
    /// Option type: value or none
    Option(Box<WasmComponentType>),
    /// Result type: success or error value
    Result(
        Box<std::option::Option<WasmComponentType>>,
        Box<std::option::Option<WasmComponentType>>,
    ),
    /// Tuple type: fixed-length heterogeneous sequence
    Tuple(Vec<WasmComponentType>),
    /// Enum type: unit variant
    Enum(Vec<String>),
    /// Flags type: set of named boolean bits
    Flags(Vec<String>),
    /// Resource type handle (owned)
    Resource(String),
    /// Borrowed resource handle
    Borrow(String),
    /// Named type alias reference
    TypeRef(String),
    /// Function type: parameters and results
    Func(
        Vec<(String, WasmComponentType)>,
        Vec<(String, WasmComponentType)>,
    ),
}
/// String encoding options in the Canonical ABI.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum StringEncoding {
    /// UTF-8 encoding (default)
    #[default]
    Utf8,
    /// UTF-16 encoding
    Utf16,
    /// Latin-1 with UTF-16 fallback
    Latin1Utf16,
}
/// A WebAssembly Component instance with imports, exports, and core modules.
///
/// A component instance represents a fully-linked component that can be
/// composed with other components via its import/export interface.
#[derive(Debug, Clone)]
pub struct ComponentInstance {
    /// Name of this component instance
    pub name: String,
    /// Imports required by this component
    pub imports: Vec<ComponentImport>,
    /// Exports provided by this component
    pub exports: Vec<ComponentExport>,
    /// Core WebAssembly modules embedded in this component
    pub core_modules: Vec<CoreModule>,
    /// Type definitions used within this component
    pub type_defs: HashMap<String, WasmComponentType>,
    /// Resource type definitions
    pub resource_defs: Vec<String>,
    /// Whether this component uses the WASI preview2 interfaces
    pub uses_wasi_preview2: bool,
}
impl ComponentInstance {
    /// Create a new empty component instance.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            imports: Vec::new(),
            exports: Vec::new(),
            core_modules: Vec::new(),
            type_defs: HashMap::new(),
            resource_defs: Vec::new(),
            uses_wasi_preview2: false,
        }
    }
    /// Add an import.
    pub fn add_import(&mut self, import: ComponentImport) {
        self.imports.push(import);
    }
    /// Add an export.
    pub fn add_export(&mut self, export: ComponentExport) {
        self.exports.push(export);
    }
    /// Add a core module.
    pub fn add_core_module(&mut self, module: CoreModule) {
        self.core_modules.push(module);
    }
    /// Define a named type.
    pub fn define_type(&mut self, name: impl Into<String>, ty: WasmComponentType) {
        self.type_defs.insert(name.into(), ty);
    }
    /// Add a resource type definition.
    pub fn add_resource(&mut self, name: impl Into<String>) {
        self.resource_defs.push(name.into());
    }
    /// Enable WASI preview2 interfaces.
    pub fn with_wasi_preview2(mut self) -> Self {
        self.uses_wasi_preview2 = true;
        self
    }
    /// Get the number of exports.
    pub fn export_count(&self) -> usize {
        self.exports.len()
    }
    /// Get the number of imports.
    pub fn import_count(&self) -> usize {
        self.imports.len()
    }
}
/// Liveness analysis for WasmCExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WasmCExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl WasmCExtLiveness {
    #[allow(dead_code)]
    pub fn new(n: usize) -> Self {
        Self {
            live_in: vec![Vec::new(); n],
            live_out: vec![Vec::new(); n],
            defs: vec![Vec::new(); n],
            uses: vec![Vec::new(); n],
        }
    }
    #[allow(dead_code)]
    pub fn live_in(&self, b: usize, v: usize) -> bool {
        self.live_in.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn live_out(&self, b: usize, v: usize) -> bool {
        self.live_out
            .get(b)
            .map(|s| s.contains(&v))
            .unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn add_def(&mut self, b: usize, v: usize) {
        if let Some(s) = self.defs.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn add_use(&mut self, b: usize, v: usize) {
        if let Some(s) = self.uses.get_mut(b) {
            if !s.contains(&v) {
                s.push(v);
            }
        }
    }
    #[allow(dead_code)]
    pub fn var_is_used_in_block(&self, b: usize, v: usize) -> bool {
        self.uses.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
    #[allow(dead_code)]
    pub fn var_is_def_in_block(&self, b: usize, v: usize) -> bool {
        self.defs.get(b).map(|s| s.contains(&v)).unwrap_or(false)
    }
}
/// Pass registry for WasmCExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct WasmCExtPassRegistry {
    pub(super) configs: Vec<WasmCExtPassConfig>,
    pub(super) stats: Vec<WasmCExtPassStats>,
}
impl WasmCExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: WasmCExtPassConfig) {
        self.stats.push(WasmCExtPassStats::new());
        self.configs.push(c);
    }
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.configs.len()
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.configs.is_empty()
    }
    #[allow(dead_code)]
    pub fn get(&self, i: usize) -> Option<&WasmCExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&WasmCExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&WasmCExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &WasmCExtPassPhase) -> Vec<&WasmCExtPassConfig> {
        self.configs
            .iter()
            .filter(|c| c.enabled && &c.phase == ph)
            .collect()
    }
    #[allow(dead_code)]
    pub fn total_nodes_visited(&self) -> usize {
        self.stats.iter().map(|s| s.nodes_visited).sum()
    }
    #[allow(dead_code)]
    pub fn any_changed(&self) -> bool {
        self.stats.iter().any(|s| s.changed)
    }
}
/// A single export from a WebAssembly component.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentExport {
    /// External name of the export (used in WIT)
    pub name: String,
    /// What kind of item is being exported
    pub kind: ComponentExportKind,
    /// Internal identifier or alias being exported
    pub item: String,
    /// Optional type annotation for the export
    pub type_annotation: Option<WasmComponentType>,
}
impl ComponentExport {
    /// Create a new function export.
    pub fn func(name: impl Into<String>, item: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: ComponentExportKind::Func,
            item: item.into(),
            type_annotation: None,
        }
    }
    /// Create a new type export.
    pub fn ty(name: impl Into<String>, item: impl Into<String>, ty: WasmComponentType) -> Self {
        Self {
            name: name.into(),
            kind: ComponentExportKind::Type,
            item: item.into(),
            type_annotation: Some(ty),
        }
    }
    /// Create a new instance export.
    pub fn instance(name: impl Into<String>, item: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            kind: ComponentExportKind::Instance,
            item: item.into(),
            type_annotation: None,
        }
    }
    /// Emit the WAT component-format export declaration.
    pub fn emit(&self) -> String {
        match &self.type_annotation {
            Some(ty) => {
                format!(
                    "(export \"{}\" ({} {}) (type {}))",
                    self.name, self.kind, self.item, ty
                )
            }
            None => format!("(export \"{}\" ({} {}))", self.name, self.kind, self.item),
        }
    }
}
/// A resource definition in a WIT interface.
#[derive(Debug, Clone)]
pub struct WitResource {
    /// Resource type name
    pub name: String,
    /// Constructor function signature (params)
    pub constructor: Option<Vec<(String, WasmComponentType)>>,
    /// Static methods
    pub static_methods: Vec<(String, WasmComponentType)>,
    /// Instance methods (first param is implicit self)
    pub methods: Vec<(String, WasmComponentType)>,
}
impl WitResource {
    /// Create a new resource definition.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            constructor: None,
            static_methods: Vec::new(),
            methods: Vec::new(),
        }
    }
    /// Set the constructor.
    pub fn with_constructor(mut self, params: Vec<(String, WasmComponentType)>) -> Self {
        self.constructor = Some(params);
        self
    }
    /// Add a method.
    pub fn with_method(mut self, name: impl Into<String>, ty: WasmComponentType) -> Self {
        self.methods.push((name.into(), ty));
        self
    }
    /// Emit the WIT resource block.
    pub fn emit_wit(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!("resource {} {{", self.name));
        if let Some(ctor_params) = &self.constructor {
            let params: Vec<String> = ctor_params
                .iter()
                .map(|(n, t)| format!("{}: {}", n, t))
                .collect();
            lines.push(format!("  constructor({});", params.join(", ")));
        }
        for (method_name, method_ty) in &self.methods {
            lines.push(format!("  {}: {};", method_name, method_ty));
        }
        for (method_name, method_ty) in &self.static_methods {
            lines.push(format!("  {}: static {};", method_name, method_ty));
        }
        lines.push("}".to_string());
        lines.join("\n")
    }
}
/// Configuration for WasmCExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WasmCExtPassConfig {
    pub name: String,
    pub phase: WasmCExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl WasmCExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: WasmCExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: WasmCExtPassPhase) -> Self {
        self.phase = phase;
        self
    }
    #[allow(dead_code)]
    pub fn with_max_iter(mut self, n: usize) -> Self {
        self.max_iterations = n;
        self
    }
    #[allow(dead_code)]
    pub fn with_debug(mut self, d: u32) -> Self {
        self.debug = d;
        self
    }
    #[allow(dead_code)]
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
    #[allow(dead_code)]
    pub fn with_timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }
    #[allow(dead_code)]
    pub fn is_debug_enabled(&self) -> bool {
        self.debug > 0
    }
}
/// A text buffer for building WasmCompExt output source code.
#[derive(Debug, Default)]
pub struct WasmCompExtSourceBuffer {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
impl WasmCompExtSourceBuffer {
    pub fn new() -> Self {
        WasmCompExtSourceBuffer {
            buf: String::new(),
            indent_level: 0,
            indent_str: "    ".to_string(),
        }
    }
    pub fn with_indent(mut self, indent: impl Into<String>) -> Self {
        self.indent_str = indent.into();
        self
    }
    pub fn push_line(&mut self, line: &str) {
        for _ in 0..self.indent_level {
            self.buf.push_str(&self.indent_str);
        }
        self.buf.push_str(line);
        self.buf.push('\n');
    }
    pub fn push_raw(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    pub fn indent(&mut self) {
        self.indent_level += 1;
    }
    pub fn dedent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }
    pub fn as_str(&self) -> &str {
        &self.buf
    }
    pub fn len(&self) -> usize {
        self.buf.len()
    }
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
    pub fn line_count(&self) -> usize {
        self.buf.lines().count()
    }
    pub fn into_string(self) -> String {
        self.buf
    }
    pub fn reset(&mut self) {
        self.buf.clear();
        self.indent_level = 0;
    }
}
/// Canonical ABI options for lift/lower operations.
#[derive(Debug, Clone, PartialEq)]
pub struct CanonicalOptions {
    /// Memory instance to use for passing data
    pub memory: Option<String>,
    /// Realloc function for dynamic allocation
    pub realloc: Option<String>,
    /// Post-return function for cleanup
    pub post_return: Option<String>,
    /// String encoding format
    pub string_encoding: StringEncoding,
}
impl CanonicalOptions {
    /// Create options with memory and realloc.
    pub fn with_memory_and_realloc(memory: impl Into<String>, realloc: impl Into<String>) -> Self {
        Self {
            memory: Some(memory.into()),
            realloc: Some(realloc.into()),
            post_return: None,
            string_encoding: StringEncoding::Utf8,
        }
    }
    /// Emit the canonical options as a WIT/WAT annotation string.
    pub fn emit(&self) -> String {
        let mut parts = Vec::new();
        if let Some(mem) = &self.memory {
            parts.push(format!("(memory {})", mem));
        }
        if let Some(ra) = &self.realloc {
            parts.push(format!("(realloc {})", ra));
        }
        if let Some(pr) = &self.post_return {
            parts.push(format!("(post-return {})", pr));
        }
        if self.string_encoding != StringEncoding::Utf8 {
            parts.push(format!("(string-encoding {})", self.string_encoding));
        }
        parts.join(" ")
    }
}
/// Tracks declared names for WasmCompExt scope analysis.
#[derive(Debug, Default)]
pub struct WasmCompExtNameScope {
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<WasmCompExtNameScope>>,
}
impl WasmCompExtNameScope {
    pub fn new() -> Self {
        WasmCompExtNameScope::default()
    }
    pub fn declare(&mut self, name: impl Into<String>) -> bool {
        self.declared.insert(name.into())
    }
    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name)
    }
    pub fn push_scope(self) -> Self {
        WasmCompExtNameScope {
            declared: std::collections::HashSet::new(),
            depth: self.depth + 1,
            parent: Some(Box::new(self)),
        }
    }
    pub fn pop_scope(self) -> Self {
        *self.parent.unwrap_or_default()
    }
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn len(&self) -> usize {
        self.declared.len()
    }
}
/// A function defined at the component level.
#[derive(Debug, Clone)]
pub struct ComponentFunc {
    /// Function name
    pub name: String,
    /// Parameter types (name, type)
    pub params: Vec<(String, WasmComponentType)>,
    /// Result types (name, type)
    pub results: Vec<(String, WasmComponentType)>,
    /// Whether this is a lifted core function or component function
    pub is_lifted: bool,
    /// Core function being lifted (if applicable)
    pub core_func: Option<String>,
    /// Canonical options for this function
    pub options: CanonicalOptions,
}
impl ComponentFunc {
    /// Create a new lifted component function.
    pub fn lifted(
        name: impl Into<String>,
        core_func: impl Into<String>,
        params: Vec<(String, WasmComponentType)>,
        results: Vec<(String, WasmComponentType)>,
        options: CanonicalOptions,
    ) -> Self {
        Self {
            name: name.into(),
            params,
            results,
            is_lifted: true,
            core_func: Some(core_func.into()),
            options,
        }
    }
    /// Create a new pure component function (lowered import).
    pub fn lowered(
        name: impl Into<String>,
        params: Vec<(String, WasmComponentType)>,
        results: Vec<(String, WasmComponentType)>,
    ) -> Self {
        Self {
            name: name.into(),
            params,
            results,
            is_lifted: false,
            core_func: None,
            options: CanonicalOptions::default(),
        }
    }
    /// Emit the function definition as a component-level canon instruction.
    pub fn emit_canon(&self) -> String {
        if self.is_lifted {
            let core = self.core_func.as_deref().unwrap_or("unknown");
            let opts = self.options.emit();
            if opts.is_empty() {
                format!("(canon lift (core func ${}) (func ${}))", core, self.name)
            } else {
                format!(
                    "(canon lift (core func ${}) {} (func ${}))",
                    core, opts, self.name
                )
            }
        } else {
            format!(
                "(canon lower (func ${}) (core func {}))",
                self.name, self.name
            )
        }
    }
    /// Emit the type declaration for this function.
    pub fn emit_type(&self) -> String {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|(n, t)| format!("(param \"{}\" {})", n, t))
            .collect();
        let results: Vec<String> = self
            .results
            .iter()
            .map(|(n, t)| format!("(result \"{}\" {})", n, t))
            .collect();
        format!("(func (type {} {}))", params.join(" "), results.join(" "))
    }
}
/// Pass-timing record for WasmCompExt profiler.
#[derive(Debug, Clone)]
pub struct WasmCompExtPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
impl WasmCompExtPassTiming {
    pub fn new(
        pass_name: impl Into<String>,
        elapsed_us: u64,
        items: usize,
        before: usize,
        after: usize,
    ) -> Self {
        WasmCompExtPassTiming {
            pass_name: pass_name.into(),
            elapsed_us,
            items_processed: items,
            bytes_before: before,
            bytes_after: after,
        }
    }
    pub fn throughput_mps(&self) -> f64 {
        if self.elapsed_us == 0 {
            0.0
        } else {
            self.items_processed as f64 / (self.elapsed_us as f64 / 1_000_000.0)
        }
    }
    pub fn size_ratio(&self) -> f64 {
        if self.bytes_before == 0 {
            1.0
        } else {
            self.bytes_after as f64 / self.bytes_before as f64
        }
    }
    pub fn is_profitable(&self) -> bool {
        self.size_ratio() <= 1.05
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WCPassConfig {
    pub phase: WCPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl WCPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: WCPassPhase) -> Self {
        WCPassConfig {
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
pub struct WCPassRegistry {
    pub(super) configs: Vec<WCPassConfig>,
    pub(super) stats: std::collections::HashMap<String, WCPassStats>,
}
impl WCPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        WCPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: WCPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), WCPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&WCPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&WCPassStats> {
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
pub struct WCWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl WCWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        WCWorklist {
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
/// Statistics for WasmCExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WasmCExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl WasmCExtPassStats {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn visit(&mut self) {
        self.nodes_visited += 1;
    }
    #[allow(dead_code)]
    pub fn modify(&mut self) {
        self.nodes_modified += 1;
        self.changed = true;
    }
    #[allow(dead_code)]
    pub fn iterate(&mut self) {
        self.iterations += 1;
    }
    #[allow(dead_code)]
    pub fn error(&mut self) {
        self.errors += 1;
    }
    #[allow(dead_code)]
    pub fn efficiency(&self) -> f64 {
        if self.nodes_visited == 0 {
            0.0
        } else {
            self.nodes_modified as f64 / self.nodes_visited as f64
        }
    }
    #[allow(dead_code)]
    pub fn merge(&mut self, o: &WasmCExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Constant folding helper for WasmCExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WasmCExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl WasmCExtConstFolder {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            folds: 0,
            failures: 0,
            enabled: true,
        }
    }
    #[allow(dead_code)]
    pub fn add_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_add(b)
    }
    #[allow(dead_code)]
    pub fn sub_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_sub(b)
    }
    #[allow(dead_code)]
    pub fn mul_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_mul(b)
    }
    #[allow(dead_code)]
    pub fn div_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_div(b)
        }
    }
    #[allow(dead_code)]
    pub fn rem_i64(&mut self, a: i64, b: i64) -> Option<i64> {
        if b == 0 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_rem(b)
        }
    }
    #[allow(dead_code)]
    pub fn neg_i64(&mut self, a: i64) -> Option<i64> {
        self.folds += 1;
        a.checked_neg()
    }
    #[allow(dead_code)]
    pub fn shl_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shl(s)
        }
    }
    #[allow(dead_code)]
    pub fn shr_i64(&mut self, a: i64, s: u32) -> Option<i64> {
        if s >= 64 {
            self.failures += 1;
            None
        } else {
            self.folds += 1;
            a.checked_shr(s)
        }
    }
    #[allow(dead_code)]
    pub fn and_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a & b
    }
    #[allow(dead_code)]
    pub fn or_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a | b
    }
    #[allow(dead_code)]
    pub fn xor_i64(&mut self, a: i64, b: i64) -> i64 {
        self.folds += 1;
        a ^ b
    }
    #[allow(dead_code)]
    pub fn not_i64(&mut self, a: i64) -> i64 {
        self.folds += 1;
        !a
    }
    #[allow(dead_code)]
    pub fn fold_count(&self) -> usize {
        self.folds
    }
    #[allow(dead_code)]
    pub fn failure_count(&self) -> usize {
        self.failures
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}
