//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls2::*;
use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

/// Content of a core WebAssembly module.
/// A fixed-capacity ring buffer of strings (for recent-event logging in WasmCompExt).
#[derive(Debug)]
pub struct WasmCompExtEventLog {
    pub(crate) entries: std::collections::VecDeque<String>,
    pub(crate) capacity: usize,
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
    pub(crate) flags: std::collections::HashSet<String>,
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
    pub(crate) items: std::collections::VecDeque<usize>,
    pub(crate) present: Vec<bool>,
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
    pub(crate) timings: Vec<WasmCompExtPassTiming>,
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
    pub(crate) n: usize,
    pub(crate) adj: Vec<Vec<usize>>,
    pub(crate) rev: Vec<Vec<usize>>,
    pub(crate) edge_count: usize,
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
