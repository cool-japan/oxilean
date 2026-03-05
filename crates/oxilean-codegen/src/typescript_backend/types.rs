//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet, VecDeque};

/// A segment of a template literal.
#[derive(Debug, Clone, PartialEq)]
pub enum TsTemplatePart {
    /// A raw string segment.
    Text(std::string::String),
    /// An interpolated expression: `${expr}`.
    Expr(TsExpr),
}
#[allow(dead_code)]
pub struct TSPassRegistry {
    pub(super) configs: Vec<TSPassConfig>,
    pub(super) stats: std::collections::HashMap<String, TSPassStats>,
}
impl TSPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TSPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: TSPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), TSPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&TSPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&TSPassStats> {
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
/// A TypeScript class declaration.
#[derive(Debug, Clone)]
pub struct TsClass {
    pub name: std::string::String,
    pub extends: Option<std::string::String>,
    pub implements: Vec<std::string::String>,
    pub fields: Vec<TsClassField>,
    pub methods: Vec<TsClassMethod>,
    pub type_params: Vec<std::string::String>,
    pub is_exported: bool,
}
/// A diagnostic message from a TsExt pass.
#[derive(Debug, Clone)]
pub struct TsExtDiagMsg {
    pub severity: TsExtDiagSeverity,
    pub pass: String,
    pub message: String,
}
impl TsExtDiagMsg {
    pub fn error(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        TsExtDiagMsg {
            severity: TsExtDiagSeverity::Error,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn warning(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        TsExtDiagMsg {
            severity: TsExtDiagSeverity::Warning,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn note(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        TsExtDiagMsg {
            severity: TsExtDiagSeverity::Note,
            pass: pass.into(),
            message: msg.into(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TSCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A feature flag set for TsExt capabilities.
#[derive(Debug, Clone, Default)]
pub struct TsExtFeatures {
    pub(super) flags: std::collections::HashSet<String>,
}
impl TsExtFeatures {
    pub fn new() -> Self {
        TsExtFeatures::default()
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
    pub fn union(&self, other: &TsExtFeatures) -> TsExtFeatures {
        TsExtFeatures {
            flags: self.flags.union(&other.flags).cloned().collect(),
        }
    }
    pub fn intersection(&self, other: &TsExtFeatures) -> TsExtFeatures {
        TsExtFeatures {
            flags: self.flags.intersection(&other.flags).cloned().collect(),
        }
    }
}
/// A TypeScript function parameter.
#[derive(Debug, Clone)]
pub struct TsParam {
    pub name: std::string::String,
    pub ty: TsType,
    pub optional: bool,
    pub rest: bool,
}
/// A field in a TypeScript class.
#[derive(Debug, Clone)]
pub struct TsClassField {
    pub name: std::string::String,
    pub ty: TsType,
    pub readonly: bool,
    pub optional: bool,
    pub is_private: bool,
    pub is_static: bool,
}
/// A TypeScript source module that can emit `.ts` or `.d.ts` source.
#[derive(Debug, Clone)]
pub struct TsModule {
    pub imports: Vec<TsImport>,
    pub type_imports: Vec<TsImport>,
    pub declarations: Vec<TsDeclaration>,
}
impl TsModule {
    /// Create an empty module.
    pub fn new() -> Self {
        TsModule {
            imports: Vec::new(),
            type_imports: Vec::new(),
            declarations: Vec::new(),
        }
    }
    /// Emit valid `.ts` source code.
    pub fn emit(&self) -> std::string::String {
        let mut out = std::string::String::new();
        for imp in &self.type_imports {
            out.push_str(&format!("{}\n", imp));
        }
        for imp in &self.imports {
            out.push_str(&format!("{}\n", imp));
        }
        if !self.imports.is_empty() || !self.type_imports.is_empty() {
            out.push('\n');
        }
        for decl in &self.declarations {
            out.push_str(&format!("{}\n\n", decl));
        }
        out
    }
    /// Emit a `.d.ts` declaration file (ambient declarations only).
    pub fn emit_d_ts(&self) -> std::string::String {
        let mut out = std::string::String::new();
        for imp in &self.type_imports {
            out.push_str(&format!("{}\n", imp));
        }
        for decl in &self.declarations {
            let dts = match decl {
                TsDeclaration::Interface(i) => format!("{}\n\n", i),
                TsDeclaration::TypeAlias(t) => format!("{}\n\n", t),
                TsDeclaration::Enum(e) => format!("{}\n\n", e),
                TsDeclaration::Function(f) => {
                    let mut sig = std::string::String::new();
                    if f.is_exported {
                        sig.push_str("export ");
                    }
                    sig.push_str(&format!("declare function {}", f.name));
                    if !f.type_params.is_empty() {
                        sig.push_str(&format!("<{}>", f.type_params.join(", ")));
                    }
                    sig.push('(');
                    for (i, p) in f.params.iter().enumerate() {
                        if i > 0 {
                            sig.push_str(", ");
                        }
                        sig.push_str(&format!("{}: {}", p.name, p.ty));
                    }
                    sig.push_str(&format!("): {};\n\n", f.return_type));
                    sig
                }
                TsDeclaration::Class(c) => format!("{}\n\n", c),
                TsDeclaration::Const(name, ty, _) => {
                    if let Some(t) = ty {
                        format!("export declare const {}: {};\n\n", name, t)
                    } else {
                        format!("export declare const {};\n\n", name)
                    }
                }
                TsDeclaration::Let(name, ty, _) => {
                    if let Some(t) = ty {
                        format!("export declare let {}: {};\n\n", name, t)
                    } else {
                        format!("export declare let {};\n\n", name)
                    }
                }
                TsDeclaration::ReExport(path) => {
                    format!("export * from \"{}\";\n\n", path)
                }
            };
            out.push_str(&dts);
        }
        out
    }
}
/// A text buffer for building TsExt output source code.
#[derive(Debug, Default)]
pub struct TsExtSourceBuffer {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
impl TsExtSourceBuffer {
    pub fn new() -> Self {
        TsExtSourceBuffer {
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
/// Severity of a TsExt diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TsExtDiagSeverity {
    Note,
    Warning,
    Error,
}
/// Heuristic freshness key for TsExt incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TsExtIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
impl TsExtIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        TsExtIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &TsExtIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
/// A version tag for TsExt output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TsExtVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
impl TsExtVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        TsExtVersion {
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
    pub fn is_compatible_with(&self, other: &TsExtVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
/// TypeScript literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum TsLit {
    Num(f64),
    Str(std::string::String),
    Bool(bool),
    Null,
    Undefined,
    BigInt(i64),
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TSPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl TSPassStats {
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
/// A top-level TypeScript declaration.
#[derive(Debug, Clone)]
pub enum TsDeclaration {
    Interface(TsInterface),
    TypeAlias(TsTypeAlias),
    Enum(TsEnum),
    Function(TsFunction),
    Class(TsClass),
    Const(std::string::String, Option<TsType>, TsExpr),
    Let(std::string::String, Option<TsType>, TsExpr),
    /// Re-export from another module.
    ReExport(std::string::String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TSDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl TSDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        TSDominatorTree {
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
#[derive(Debug, Clone)]
pub struct TSLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl TSLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        TSLivenessInfo {
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
/// A single enum member (name + optional value).
#[derive(Debug, Clone)]
pub struct TsEnumMember {
    pub name: std::string::String,
    pub value: Option<TsLit>,
}
/// Emission statistics for TsExt.
#[derive(Debug, Clone, Default)]
pub struct TsExtEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
impl TsExtEmitStats {
    pub fn new() -> Self {
        TsExtEmitStats::default()
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
#[allow(dead_code)]
pub struct TSConstantFoldingHelper;
impl TSConstantFoldingHelper {
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
/// A top-level TypeScript function declaration.
#[derive(Debug, Clone)]
pub struct TsFunction {
    pub name: std::string::String,
    pub params: Vec<TsParam>,
    pub return_type: TsType,
    pub body: Vec<TsStmt>,
    pub is_async: bool,
    pub type_params: Vec<std::string::String>,
    pub is_exported: bool,
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in TsExt).
#[derive(Debug)]
pub struct TsExtEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
}
impl TsExtEventLog {
    pub fn new(capacity: usize) -> Self {
        TsExtEventLog {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TSAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, TSCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl TSAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        TSAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&TSCacheEntry> {
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
            TSCacheEntry {
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
#[derive(Debug, Clone, PartialEq)]
pub enum TSPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl TSPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            TSPassPhase::Analysis => "analysis",
            TSPassPhase::Transformation => "transformation",
            TSPassPhase::Verification => "verification",
            TSPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, TSPassPhase::Transformation | TSPassPhase::Cleanup)
    }
}
/// TypeScript expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum TsExpr {
    /// A literal value.
    Lit(TsLit),
    /// A variable reference: `x`, `myVar`
    Var(std::string::String),
    /// A binary operation: `lhs + rhs`
    BinOp(std::string::String, Box<TsExpr>, Box<TsExpr>),
    /// A unary operation: `!x`, `-n`
    UnaryOp(std::string::String, Box<TsExpr>),
    /// A function/method call: `f(a, b)`
    Call(Box<TsExpr>, Vec<TsExpr>),
    /// A method call: `obj.method(a, b)`
    MethodCall(Box<TsExpr>, std::string::String, Vec<TsExpr>),
    /// Constructor call: `new Foo(a, b)`
    New(Box<TsExpr>, Vec<TsExpr>),
    /// Arrow function: `(x: T) => expr`
    Arrow(Vec<(std::string::String, Option<TsType>)>, Box<TsExpr>),
    /// Ternary expression: `cond ? then : else`
    Ternary(Box<TsExpr>, Box<TsExpr>, Box<TsExpr>),
    /// Type assertion: `expr as T`
    As(Box<TsExpr>, TsType),
    /// Satisfies expression: `expr satisfies T`
    Satisfies(Box<TsExpr>, TsType),
    /// Non-null assertion: `expr!`
    TypeAssert(Box<TsExpr>),
    /// Object literal: `{ key: val }`
    ObjectLit(Vec<(std::string::String, TsExpr)>),
    /// Array literal: `[a, b, c]`
    ArrayLit(Vec<TsExpr>),
    /// Template literal: `` `Hello ${name}!` ``
    Template(Vec<TsTemplatePart>),
    /// Await expression: `await p`
    Await(Box<TsExpr>),
    /// Nullish coalescing: `a ?? b`
    Nullish(Box<TsExpr>, Box<TsExpr>),
    /// Optional chaining: `obj?.prop`
    OptChain(Box<TsExpr>, std::string::String),
}
/// TypeScript statement AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum TsStmt {
    /// Expression statement: `expr;`
    Expr(TsExpr),
    /// `const x: T = expr;`
    Const(std::string::String, Option<TsType>, TsExpr),
    /// `let x: T = expr;`
    Let(std::string::String, Option<TsType>, TsExpr),
    /// `var x: T = expr;`
    Var(std::string::String, Option<TsType>, TsExpr),
    /// `if (cond) { then } else { else_ }`
    If(TsExpr, Vec<TsStmt>, Vec<TsStmt>),
    /// `switch (expr) { case x: ... default: ... }`
    Switch(TsExpr, Vec<(TsExpr, Vec<TsStmt>)>, Vec<TsStmt>),
    /// `for (let i = init; cond; step) { body }`
    For(Box<TsStmt>, TsExpr, TsExpr, Vec<TsStmt>),
    /// `for (const x of iter) { body }`
    ForOf(std::string::String, TsExpr, Vec<TsStmt>),
    /// `for (const k in obj) { body }`
    ForIn(std::string::String, TsExpr, Vec<TsStmt>),
    /// `while (cond) { body }`
    While(TsExpr, Vec<TsStmt>),
    /// `return expr;`
    Return(TsExpr),
    /// `throw expr;`
    Throw(TsExpr),
    /// `try { body } catch (e) { handler } finally { fin }`
    TryCatch(Vec<TsStmt>, std::string::String, Vec<TsStmt>, Vec<TsStmt>),
    /// `{ stmts }`
    Block(Vec<TsStmt>),
    /// `break;`
    Break,
    /// `continue;`
    Continue,
}
/// A method in a TypeScript class.
#[derive(Debug, Clone)]
pub struct TsClassMethod {
    pub name: std::string::String,
    pub params: Vec<TsParam>,
    pub return_type: TsType,
    pub body: Vec<TsStmt>,
    pub is_async: bool,
    pub is_static: bool,
    pub is_private: bool,
    pub is_getter: bool,
    pub is_setter: bool,
}
/// The TypeScript code generation backend.
///
/// Compiles OxiLean LCNF declarations into TypeScript source code.
pub struct TypeScriptBackend {
    pub(super) module: TsModule,
}
impl TypeScriptBackend {
    /// Create a new TypeScript backend.
    pub fn new() -> Self {
        TypeScriptBackend {
            module: TsModule::new(),
        }
    }
    /// Add a declaration to the module.
    pub fn add_declaration(&mut self, decl: TsDeclaration) {
        self.module.declarations.push(decl);
    }
    /// Add an import to the module.
    pub fn add_import(&mut self, imp: TsImport) {
        if imp.is_type {
            self.module.type_imports.push(imp);
        } else {
            self.module.imports.push(imp);
        }
    }
    /// Build a discriminated union type from a list of variant names and their fields.
    ///
    /// Example:
    /// ```text
    /// type Shape =
    ///   | { kind: 'circle'; radius: number }
    ///   | { kind: 'rect'; w: number; h: number }
    /// ```
    pub fn make_discriminated_union(
        &self,
        type_name: &str,
        variants: &[(&str, Vec<(&str, TsType)>)],
    ) -> TsTypeAlias {
        let union_types: Vec<TsType> = variants
            .iter()
            .map(|(variant_name, fields)| {
                let mut members: Vec<(std::string::String, TsType)> = vec![(
                    "kind".to_string(),
                    TsType::Custom(format!("'{}'", variant_name)),
                )];
                for (field_name, field_ty) in fields {
                    members.push((field_name.to_string(), field_ty.clone()));
                }
                TsType::Object(members)
            })
            .collect();
        TsTypeAlias {
            name: type_name.to_string(),
            type_params: Vec::new(),
            definition: TsType::Union(union_types),
        }
    }
    /// Emit the full TypeScript module source.
    pub fn emit_module(&self) -> std::string::String {
        self.module.emit()
    }
    /// Emit a `.d.ts` declaration file.
    pub fn emit_d_ts(&self) -> std::string::String {
        self.module.emit_d_ts()
    }
}
/// TypeScript type representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TsType {
    /// `number`
    Number,
    /// `string`
    String,
    /// `boolean`
    Boolean,
    /// `void`
    Void,
    /// `never`
    Never,
    /// `unknown`
    Unknown,
    /// `any`
    Any,
    /// `null`
    Null,
    /// `undefined`
    Undefined,
    /// `[T0, T1, ...]`
    Tuple(Vec<TsType>),
    /// `T[]`
    Array(Box<TsType>),
    /// `{ key: T; ... }`
    Object(Vec<(std::string::String, TsType)>),
    /// `T0 | T1 | ...`
    Union(Vec<TsType>),
    /// `T0 & T1 & ...`
    Intersection(Vec<TsType>),
    /// `(p0: T0, p1: T1, ...) => R`
    Function {
        params: Vec<TsType>,
        ret: Box<TsType>,
    },
    /// A named type: `MyClass`, `SomeInterface`
    Custom(std::string::String),
    /// A generic type application: `Promise<T>`, `Map<K, V>`
    Generic(std::string::String, Vec<TsType>),
    /// `readonly T`
    ReadOnly(Box<TsType>),
    /// `Readonly<T>` utility type alias shorthand (no arg stored)
    Readonly,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TSPassConfig {
    pub phase: TSPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl TSPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: TSPassPhase) -> Self {
        TSPassConfig {
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
/// Collects TsExt diagnostics.
#[derive(Debug, Default)]
pub struct TsExtDiagCollector {
    pub(super) msgs: Vec<TsExtDiagMsg>,
}
impl TsExtDiagCollector {
    pub fn new() -> Self {
        TsExtDiagCollector::default()
    }
    pub fn emit(&mut self, d: TsExtDiagMsg) {
        self.msgs.push(d);
    }
    pub fn has_errors(&self) -> bool {
        self.msgs
            .iter()
            .any(|d| d.severity == TsExtDiagSeverity::Error)
    }
    pub fn errors(&self) -> Vec<&TsExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == TsExtDiagSeverity::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&TsExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == TsExtDiagSeverity::Warning)
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
/// A TypeScript interface member.
#[derive(Debug, Clone, PartialEq)]
pub struct TsInterfaceMember {
    pub name: std::string::String,
    pub ty: TsType,
    pub optional: bool,
    pub readonly: bool,
}
/// A TypeScript `enum` or `const enum` declaration.
#[derive(Debug, Clone)]
pub struct TsEnum {
    pub name: std::string::String,
    pub is_const: bool,
    pub members: Vec<TsEnumMember>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TSDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl TSDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TSDepGraph {
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
/// Tracks declared names for TsExt scope analysis.
#[derive(Debug, Default)]
pub struct TsExtNameScope {
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<TsExtNameScope>>,
}
impl TsExtNameScope {
    pub fn new() -> Self {
        TsExtNameScope::default()
    }
    pub fn declare(&mut self, name: impl Into<String>) -> bool {
        self.declared.insert(name.into())
    }
    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name)
    }
    pub fn push_scope(self) -> Self {
        TsExtNameScope {
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
/// A TypeScript `type` alias.
#[derive(Debug, Clone)]
pub struct TsTypeAlias {
    pub name: std::string::String,
    pub type_params: Vec<std::string::String>,
    pub definition: TsType,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TSWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl TSWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TSWorklist {
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
/// Pipeline profiler for TsExt.
#[derive(Debug, Default)]
pub struct TsExtProfiler {
    pub(super) timings: Vec<TsExtPassTiming>,
}
impl TsExtProfiler {
    pub fn new() -> Self {
        TsExtProfiler::default()
    }
    pub fn record(&mut self, t: TsExtPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&TsExtPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&TsExtPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
/// Pass-timing record for TsExt profiler.
#[derive(Debug, Clone)]
pub struct TsExtPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
impl TsExtPassTiming {
    pub fn new(
        pass_name: impl Into<String>,
        elapsed_us: u64,
        items: usize,
        before: usize,
        after: usize,
    ) -> Self {
        TsExtPassTiming {
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
/// A generic key-value configuration store for TsExt.
#[derive(Debug, Clone, Default)]
pub struct TsExtConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
}
impl TsExtConfig {
    pub fn new() -> Self {
        TsExtConfig::default()
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
/// A TypeScript import statement.
#[derive(Debug, Clone)]
pub struct TsImport {
    pub names: Vec<std::string::String>,
    pub from: std::string::String,
    pub is_type: bool,
}
/// A monotonically increasing ID generator for TsExt.
#[derive(Debug, Default)]
pub struct TsExtIdGen {
    pub(super) next: u32,
}
impl TsExtIdGen {
    pub fn new() -> Self {
        TsExtIdGen::default()
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
/// A TypeScript `interface` declaration.
#[derive(Debug, Clone)]
pub struct TsInterface {
    pub name: std::string::String,
    /// Interfaces this interface extends.
    pub extends: Vec<std::string::String>,
    /// Members of the interface.
    pub members: Vec<TsInterfaceMember>,
    /// Generic type parameters: `<T, U extends string>`.
    pub type_params: Vec<std::string::String>,
}
