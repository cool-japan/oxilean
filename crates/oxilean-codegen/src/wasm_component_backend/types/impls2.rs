//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls1::*;
use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

/// Content of a core WebAssembly module.
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
    pub(crate) configs: Vec<WasmCExtPassConfig>,
    pub(crate) stats: Vec<WasmCExtPassStats>,
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
    pub(crate) buf: String,
    pub(crate) indent_level: usize,
    pub(crate) indent_str: String,
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
    pub(crate) declared: std::collections::HashSet<String>,
    pub(crate) depth: usize,
    pub(crate) parent: Option<Box<WasmCompExtNameScope>>,
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
    pub(crate) configs: Vec<WCPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, WCPassStats>,
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
    pub(crate) items: std::collections::VecDeque<u32>,
    pub(crate) in_worklist: std::collections::HashSet<u32>,
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
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
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
