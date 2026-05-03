//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::impls1::*;
use super::impls2::*;
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
    pub(crate) entries: std::collections::HashMap<String, WCCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
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
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
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
    pub(crate) msgs: Vec<WasmCompExtDiagMsg>,
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
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
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
    pub(crate) next_id: u32,
    /// Canonical options for default lift/lower operations
    pub canonical_opts: CanonicalOptions,
    /// Generated type aliases
    pub(crate) type_aliases: HashMap<String, WasmComponentType>,
    /// Component-level function definitions
    pub(crate) component_funcs: Vec<ComponentFunc>,
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
    pub(crate) next: u32,
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
    pub(crate) entries: std::collections::HashMap<String, String>,
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
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
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
