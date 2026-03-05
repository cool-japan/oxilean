//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::{HashMap, HashSet};

/// Futhark reverse
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkReverse {
    pub array: String,
}
/// Futhark concat
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkConcat {
    pub arrays: Vec<String>,
    pub dim: Option<usize>,
}
/// Futhark pass timing
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct FutharkExtPassTiming {
    pub pass_name: String,
    pub duration_us: u64,
}
/// Futhark loop expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkLoop {
    pub kind: FutharkLoopKind,
    pub params: Vec<(String, FutharkType, String)>,
    pub body: String,
}
/// Futhark zip / unzip
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkZip {
    pub arrays: Vec<String>,
}
/// Futhark id generator
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FutharkExtIdGen {
    pub(super) counter: u64,
    pub(super) prefix: String,
}
#[allow(dead_code)]
impl FutharkExtIdGen {
    pub fn new(prefix: &str) -> Self {
        Self {
            counter: 0,
            prefix: prefix.to_string(),
        }
    }
    pub fn next(&mut self) -> String {
        let id = self.counter;
        self.counter += 1;
        format!("{}_{}", self.prefix, id)
    }
    pub fn reset(&mut self) {
        self.counter = 0;
    }
}
/// Futhark kernel launch params
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkKernelLaunch {
    pub kernel_name: String,
    pub global_size: Vec<u64>,
    pub local_size: Vec<u64>,
    pub shared_mem_bytes: u64,
}
/// Futhark name mangler for OxiLean → Futhark identifiers
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FutharkNameMangler {
    pub used: std::collections::HashSet<String>,
    pub map: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl FutharkNameMangler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn mangle(&mut self, name: &str) -> String {
        if let Some(m) = self.map.get(name) {
            return m.clone();
        }
        let mut mangled: String = name
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        let reserved = [
            "let", "in", "if", "then", "else", "loop", "for", "while", "do", "fun", "entry",
            "type", "module", "open", "import", "val", "include", "match", "case", "true", "false",
            "local",
        ];
        if reserved.contains(&mangled.as_str()) || mangled.starts_with(|c: char| c.is_ascii_digit())
        {
            mangled = format!("ox_{}", mangled);
        }
        let mut candidate = mangled.clone();
        let mut counter = 0;
        while self.used.contains(&candidate) {
            counter += 1;
            candidate = format!("{}_{}", mangled, counter);
        }
        self.used.insert(candidate.clone());
        self.map.insert(name.to_string(), candidate.clone());
        candidate
    }
    pub fn reset(&mut self) {
        self.used.clear();
        self.map.clear();
    }
}
/// Futhark map2 (map with 2 arrays)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkMap2Expr {
    pub func: String,
    pub arr1: String,
    pub arr2: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkDiag {
    pub level: FutharkDiagLevel,
    pub message: String,
    pub location: Option<String>,
}
/// A Futhark type alias: `type t = ...`
#[derive(Debug, Clone)]
pub struct FutharkTypeAlias {
    /// Alias name
    pub name: String,
    /// Type parameters
    pub params: Vec<String>,
    /// The aliased type
    pub ty: FutharkType,
    /// Whether the type is opaque (abstract)
    pub is_opaque: bool,
}
/// Futhark tuning parameter
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkTuningParam {
    pub name: String,
    pub default_value: i64,
    pub min_value: i64,
    pub max_value: i64,
    pub description: String,
}
/// Futhark statement (top-level body items inside functions).
#[derive(Debug, Clone)]
pub enum FutharkStmt {
    /// `let x = e`
    LetBinding(String, Option<FutharkType>, FutharkExpr),
    /// `let (x, y) = e`
    LetTupleBinding(Vec<String>, FutharkExpr),
    /// Loop binding: `loop (acc = init) for i < n do body`
    LoopBinding(String, FutharkExpr, String, FutharkExpr, Vec<FutharkStmt>),
    /// Return expression (final expression in block)
    ReturnExpr(FutharkExpr),
    /// Comment
    Comment(String),
}
/// Futhark source buffer (accumulates emitted code)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FutharkExtSourceBuffer {
    pub sections: Vec<(String, String)>,
    pub current: String,
}
#[allow(dead_code)]
impl FutharkExtSourceBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn write(&mut self, s: &str) {
        self.current.push_str(s);
    }
    pub fn writeln(&mut self, s: &str) {
        self.current.push_str(s);
        self.current.push('\n');
    }
    pub fn begin_section(&mut self, name: &str) {
        let done = std::mem::take(&mut self.current);
        if !done.is_empty() {
            self.sections.push(("anonymous".to_string(), done));
        }
        self.current = format!("-- === {} ===\n", name);
    }
    pub fn finish(self) -> String {
        let mut out = String::new();
        for (_, sec) in &self.sections {
            out.push_str(sec);
        }
        out.push_str(&self.current);
        out
    }
}
/// Futhark slice expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkSliceExpr {
    pub array: String,
    pub start: Option<String>,
    pub end: Option<String>,
    pub stride: Option<String>,
}
/// Futhark profiling result
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkProfileResult {
    pub kernel_name: String,
    pub runs: u64,
    pub total_us: u64,
    pub min_us: u64,
    pub max_us: u64,
    pub mean_us: f64,
}
/// Futhark let binding
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkLetBinding {
    pub pattern: String,
    pub type_ann: Option<FutharkType>,
    pub value: String,
    pub body: String,
}
/// Futhark flatten / unflatten
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkFlatten {
    pub array: String,
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FutharkExtProfiler {
    pub timings: Vec<FutharkExtPassTiming>,
}
#[allow(dead_code)]
impl FutharkExtProfiler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record(&mut self, pass: &str, us: u64) {
        self.timings.push(FutharkExtPassTiming {
            pass_name: pass.to_string(),
            duration_us: us,
        });
    }
    pub fn total_us(&self) -> u64 {
        self.timings.iter().map(|t| t.duration_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&FutharkExtPassTiming> {
        self.timings.iter().max_by_key(|t| t.duration_us)
    }
}
/// A Futhark source module (single `.fut` file).
#[derive(Debug, Clone)]
pub struct FutharkModule {
    /// Module-level `open` directives
    pub opens: Vec<String>,
    /// Type aliases
    pub types: Vec<FutharkTypeAlias>,
    /// Function definitions (including entry points)
    pub funs: Vec<FutharkFun>,
    /// Module-level constants: `let c = e`
    pub constants: Vec<(String, FutharkType, FutharkExpr)>,
    /// Module-level doc comment
    pub doc: Option<String>,
}
impl FutharkModule {
    /// Create an empty module.
    pub fn new() -> Self {
        FutharkModule {
            opens: vec![],
            types: vec![],
            funs: vec![],
            constants: vec![],
            doc: None,
        }
    }
    /// Add an `open` directive.
    pub fn add_open(&mut self, name: impl Into<String>) {
        self.opens.push(name.into());
    }
    /// Add a type alias.
    pub fn add_type(&mut self, alias: FutharkTypeAlias) {
        self.types.push(alias);
    }
    /// Add a function.
    pub fn add_fun(&mut self, fun: FutharkFun) {
        self.funs.push(fun);
    }
    /// Add a module-level constant.
    pub fn add_constant(&mut self, name: impl Into<String>, ty: FutharkType, expr: FutharkExpr) {
        self.constants.push((name.into(), ty, expr));
    }
    /// Set the module-level doc comment.
    pub fn set_doc(&mut self, doc: impl Into<String>) {
        self.doc = Some(doc.into());
    }
}
/// Diagnostics for Futhark code generation
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum FutharkDiagLevel {
    Info,
    Warning,
    Error,
}
/// Futhark program builder
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FutharkProgramBuilder {
    pub imports: Vec<String>,
    pub open_imports: Vec<String>,
    pub type_defs: Vec<String>,
    pub module_defs: Vec<String>,
    pub fun_defs: Vec<String>,
    pub entry_points: Vec<String>,
}
#[allow(dead_code)]
impl FutharkProgramBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_import(&mut self, path: &str) {
        self.imports.push(format!("import \"{}\"", path));
    }
    pub fn open_import(&mut self, path: &str) {
        self.open_imports.push(format!("open import \"{}\"", path));
    }
    pub fn add_type_alias(&mut self, name: &str, ty: &FutharkType) {
        self.type_defs.push(format!("type {} = {}", name, ty));
    }
    pub fn add_module_alias(&mut self, name: &str, module: &str) {
        self.module_defs
            .push(format!("module {} = {}", name, module));
    }
    pub fn add_fun(&mut self, fun: &str) {
        self.fun_defs.push(fun.to_string());
    }
    pub fn add_entry(&mut self, entry: &str) {
        self.entry_points.push(entry.to_string());
    }
    pub fn build(&self) -> String {
        let mut out = String::new();
        for imp in &self.imports {
            out.push_str(imp);
            out.push('\n');
        }
        for op in &self.open_imports {
            out.push_str(op);
            out.push('\n');
        }
        if !self.imports.is_empty() || !self.open_imports.is_empty() {
            out.push('\n');
        }
        for td in &self.type_defs {
            out.push_str(td);
            out.push('\n');
        }
        for md in &self.module_defs {
            out.push_str(md);
            out.push('\n');
        }
        if !self.type_defs.is_empty() || !self.module_defs.is_empty() {
            out.push('\n');
        }
        for fd in &self.fun_defs {
            out.push_str(fd);
            out.push('\n');
        }
        for ep in &self.entry_points {
            out.push_str(ep);
            out.push('\n');
        }
        out
    }
}
/// Extended Futhark emit stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct FutharkExtEmitStats {
    pub bytes_written: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
}
/// Futhark tuning file
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FutharkTuningFile {
    pub params: Vec<FutharkTuningParam>,
    pub program: String,
}
#[allow(dead_code)]
impl FutharkTuningFile {
    pub fn new(program: &str) -> Self {
        Self {
            params: Vec::new(),
            program: program.to_string(),
        }
    }
    pub fn add_param(&mut self, p: FutharkTuningParam) {
        self.params.push(p);
    }
    pub fn emit(&self) -> String {
        let mut out = String::new();
        for p in &self.params {
            out.push_str(&format!("{} = {}\n", p.name, p.default_value));
        }
        out
    }
}
/// Futhark primitive value constant
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum FutharkConst {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F16(u16),
    F32(f32),
    F64(f64),
    Bool(bool),
}
/// Futhark stream_red / stream_map
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FutharkStreamKind {
    Seq,
    Par,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkStreamRed {
    pub kind: FutharkStreamKind,
    pub op: String,
    pub neutral: String,
    pub func: String,
    pub array: String,
}
/// Futhark index expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkIndexExpr {
    pub array: String,
    pub index: String,
}
/// Futhark iota expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkIota {
    pub n: String,
    pub start: Option<String>,
    pub step: Option<String>,
}
/// Futhark map expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkMapExpr {
    pub func: String,
    pub arrays: Vec<String>,
}
/// Futhark copy
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkCopy {
    pub array: String,
}
/// Configuration for the Futhark backend emitter.
#[derive(Debug, Clone)]
pub struct FutharkConfig {
    /// Number of spaces per indentation level
    pub indent_width: usize,
    /// Emit type annotations on let-bindings when available
    pub annotate_lets: bool,
    /// Default integer type for literals without explicit type
    pub default_int: FutharkType,
    /// Default float type for literals without explicit type
    pub default_float: FutharkType,
}
/// Futhark unsafe coerce (for performance-critical code)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkUnsafeCoerce {
    pub value: String,
    pub from_type: FutharkType,
    pub to_type: FutharkType,
}
/// Futhark version target
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FutharkVersion {
    V020,
    V021,
    V022,
    V023,
    V024,
    V025,
    Latest,
}
/// Futhark scatter primitive
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkScatter {
    pub dest: String,
    pub indices: String,
    pub values: String,
}
/// Futhark code statistics
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct FutharkCodeStats {
    pub num_functions: usize,
    pub num_entries: usize,
    pub num_type_defs: usize,
    pub num_map_exprs: usize,
    pub num_reduce_exprs: usize,
    pub num_scan_exprs: usize,
    pub num_filter_exprs: usize,
    pub num_scatter_exprs: usize,
    pub num_loops: usize,
    pub num_unsafe: usize,
}
/// Futhark size expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkSizeExpr {
    pub dim: usize,
    pub array: String,
}
/// Futhark partition expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkPartitionExpr {
    pub k: usize,
    pub pred: String,
    pub array: String,
}
/// Futhark pass stats additional
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct FutharkPassStats {
    pub functions_processed: usize,
    pub maps_emitted: usize,
    pub reduces_emitted: usize,
    pub scans_emitted: usize,
    pub kernels_generated: usize,
    pub total_parallelism: u64,
}
/// Futhark loop form
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FutharkLoopKind {
    For {
        var: String,
        bound: String,
    },
    While {
        cond: String,
    },
    ForWhile {
        var: String,
        bound: String,
        cond: String,
    },
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkUnzip {
    pub array: String,
}
/// Attributes that can annotate a Futhark function.
#[derive(Debug, Clone, PartialEq)]
pub enum FutharkAttr {
    /// `#[inline]`
    Inline,
    /// `#[noinline]`
    NoInline,
    /// `#[nomap]` — prevent automatic parallelisation
    NoMap,
    /// `#[sequential]`
    Sequential,
    /// Custom attribute string
    Custom(String),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkMatchExpr {
    pub scrutinee: String,
    pub arms: Vec<FutharkMatchArm>,
}
/// Futhark rotate
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkRotate {
    pub dim: usize,
    pub amount: String,
    pub array: String,
}
/// Futhark memory block (for GPU memory management)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkMemBlock {
    pub block_id: u32,
    pub size_bytes: u64,
    pub device: String,
    pub is_pinned: bool,
}
/// Futhark feature flags
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FutharkFeatureFlags {
    pub enable_unsafe: bool,
    pub enable_in_place_updates: bool,
    pub enable_streaming: bool,
    pub enable_loop_fusion: bool,
    pub enable_double_buffering: bool,
}
/// Futhark match expression (introduced in Futhark 0.24)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkMatchArm {
    pub pattern: String,
    pub body: String,
}
/// A Futhark function (or entry point).
#[derive(Debug, Clone)]
pub struct FutharkFun {
    /// Function name
    pub name: String,
    /// Type parameters (e.g., `'t`)
    pub type_params: Vec<String>,
    /// Parameters: `(name : type)`
    pub params: Vec<(String, FutharkType)>,
    /// Return type
    pub return_type: FutharkType,
    /// Body statements
    pub body: Vec<FutharkStmt>,
    /// Whether this is an `entry` point
    pub is_entry: bool,
    /// Function attributes
    pub attrs: Vec<FutharkAttr>,
}
impl FutharkFun {
    /// Create a new regular function.
    pub fn new(
        name: impl Into<String>,
        params: Vec<(String, FutharkType)>,
        return_type: FutharkType,
        body: Vec<FutharkStmt>,
    ) -> Self {
        FutharkFun {
            name: name.into(),
            type_params: vec![],
            params,
            return_type,
            body,
            is_entry: false,
            attrs: vec![],
        }
    }
    /// Create a new entry point.
    pub fn entry(
        name: impl Into<String>,
        params: Vec<(String, FutharkType)>,
        return_type: FutharkType,
        body: Vec<FutharkStmt>,
    ) -> Self {
        FutharkFun {
            name: name.into(),
            type_params: vec![],
            params,
            return_type,
            body,
            is_entry: true,
            attrs: vec![],
        }
    }
    /// Add a type parameter.
    pub fn with_type_param(mut self, tp: impl Into<String>) -> Self {
        self.type_params.push(tp.into());
        self
    }
    /// Add an attribute.
    pub fn with_attr(mut self, attr: FutharkAttr) -> Self {
        self.attrs.push(attr);
        self
    }
}
/// Backend state for emitting Futhark source code.
pub struct FutharkBackend {
    /// Output buffer
    pub(super) buf: String,
    /// Current indentation level
    pub(super) indent: usize,
    /// Indentation string (spaces per level)
    pub(super) indent_str: String,
    /// Configuration options
    pub(super) config: FutharkConfig,
}
impl FutharkBackend {
    /// Create a new backend with default configuration.
    pub fn new() -> Self {
        FutharkBackend::with_config(FutharkConfig::default())
    }
    /// Create a new backend with custom configuration.
    pub fn with_config(config: FutharkConfig) -> Self {
        let indent_str = " ".repeat(config.indent_width);
        FutharkBackend {
            buf: String::new(),
            indent: 0,
            indent_str,
            config,
        }
    }
    pub(super) fn push(&mut self, s: &str) {
        self.buf.push_str(s);
    }
    pub(super) fn push_char(&mut self, c: char) {
        self.buf.push(c);
    }
    pub(super) fn newline(&mut self) {
        self.buf.push('\n');
    }
    pub(super) fn emit_indent(&mut self) {
        for _ in 0..self.indent {
            self.buf.push_str(&self.indent_str.clone());
        }
    }
    pub(super) fn emit_line(&mut self, s: &str) {
        self.emit_indent();
        self.push(s);
        self.newline();
    }
    pub(super) fn indent_in(&mut self) {
        self.indent += 1;
    }
    pub(super) fn indent_out(&mut self) {
        if self.indent > 0 {
            self.indent -= 1;
        }
    }
    /// Emit a Futhark type to the buffer.
    pub fn emit_type(&mut self, ty: &FutharkType) {
        let s = ty.to_string();
        self.push(&s);
    }
    /// Emit a Futhark expression to the buffer.
    pub fn emit_expr(&mut self, expr: &FutharkExpr) {
        match expr {
            FutharkExpr::IntLit(n, ty) => {
                self.push(&n.to_string());
                self.push(&ty.to_string());
            }
            FutharkExpr::FloatLit(v, ty) => {
                self.push(&format!("{v}"));
                self.push(&ty.to_string());
            }
            FutharkExpr::BoolLit(b) => {
                self.push(if *b { "true" } else { "false" });
            }
            FutharkExpr::Var(name) => {
                self.push(name);
            }
            FutharkExpr::ArrayLit(elems) => {
                self.push("[");
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.emit_expr(e);
                }
                self.push("]");
            }
            FutharkExpr::Index(arr, idx) => {
                self.emit_expr_paren(arr);
                self.push("[");
                self.emit_expr(idx);
                self.push("]");
            }
            FutharkExpr::Slice(arr, lo, hi) => {
                self.emit_expr_paren(arr);
                self.push("[");
                if let Some(l) = lo {
                    self.emit_expr(l);
                }
                self.push(":");
                if let Some(h) = hi {
                    self.emit_expr(h);
                }
                self.push("]");
            }
            FutharkExpr::Map(f, a) => {
                self.push("map ");
                self.emit_expr_paren(f);
                self.push(" ");
                self.emit_expr_paren(a);
            }
            FutharkExpr::Map2(f, a, b) => {
                self.push("map2 ");
                self.emit_expr_paren(f);
                self.push(" ");
                self.emit_expr_paren(a);
                self.push(" ");
                self.emit_expr_paren(b);
            }
            FutharkExpr::Reduce(op, ne, a) => {
                self.push("reduce ");
                self.emit_expr_paren(op);
                self.push(" ");
                self.emit_expr_paren(ne);
                self.push(" ");
                self.emit_expr_paren(a);
            }
            FutharkExpr::Scan(op, ne, a) => {
                self.push("scan ");
                self.emit_expr_paren(op);
                self.push(" ");
                self.emit_expr_paren(ne);
                self.push(" ");
                self.emit_expr_paren(a);
            }
            FutharkExpr::Filter(f, a) => {
                self.push("filter ");
                self.emit_expr_paren(f);
                self.push(" ");
                self.emit_expr_paren(a);
            }
            FutharkExpr::Zip(a, b) => {
                self.push("zip ");
                self.emit_expr_paren(a);
                self.push(" ");
                self.emit_expr_paren(b);
            }
            FutharkExpr::Unzip(a) => {
                self.push("unzip ");
                self.emit_expr_paren(a);
            }
            FutharkExpr::Iota(n) => {
                self.push("iota ");
                self.emit_expr_paren(n);
            }
            FutharkExpr::Replicate(n, x) => {
                self.push("replicate ");
                self.emit_expr_paren(n);
                self.push(" ");
                self.emit_expr_paren(x);
            }
            FutharkExpr::IfThenElse(cond, t, e) => {
                self.push("if ");
                self.emit_expr(cond);
                self.push(" then ");
                self.emit_expr(t);
                self.push(" else ");
                self.emit_expr(e);
            }
            FutharkExpr::Lambda(params, body) => {
                self.push("\\");
                for (i, (name, ty)) in params.iter().enumerate() {
                    if i > 0 {
                        self.push(" ");
                    }
                    self.push("(");
                    self.push(name);
                    self.push(": ");
                    self.emit_type(ty);
                    self.push(")");
                }
                self.push(" -> ");
                self.emit_expr(body);
            }
            FutharkExpr::LetIn(name, ty, val, body) => {
                self.push("let ");
                self.push(name);
                if let Some(t) = ty {
                    if self.config.annotate_lets {
                        self.push(": ");
                        self.emit_type(t);
                    }
                }
                self.push(" = ");
                self.emit_expr(val);
                self.push(" in ");
                self.emit_expr(body);
            }
            FutharkExpr::Loop(acc, init, var, bound, body) => {
                self.push("loop (");
                self.push(acc);
                self.push(" = ");
                self.emit_expr(init);
                self.push(") for ");
                self.push(var);
                self.push(" < ");
                self.emit_expr(bound);
                self.push(" do ");
                self.emit_expr(body);
            }
            FutharkExpr::BinOp(op, lhs, rhs) => {
                self.emit_expr_paren(lhs);
                self.push(" ");
                self.push(op);
                self.push(" ");
                self.emit_expr_paren(rhs);
            }
            FutharkExpr::UnOp(op, e) => {
                self.push(op);
                self.emit_expr_paren(e);
            }
            FutharkExpr::Apply(f, args) => {
                self.emit_expr_paren(f);
                for arg in args {
                    self.push(" ");
                    self.emit_expr_paren(arg);
                }
            }
            FutharkExpr::TupleLit(elems) => {
                self.push("(");
                for (i, e) in elems.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.emit_expr(e);
                }
                self.push(")");
            }
            FutharkExpr::RecordLit(fields) => {
                self.push("{");
                for (i, (name, e)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.push(name);
                    self.push(" = ");
                    self.emit_expr(e);
                }
                self.push("}");
            }
            FutharkExpr::FieldAccess(rec, field) => {
                self.emit_expr_paren(rec);
                self.push(".");
                self.push(field);
            }
            FutharkExpr::Ascribe(e, ty) => {
                self.push("(");
                self.emit_expr(e);
                self.push(" : ");
                self.emit_type(ty);
                self.push(")");
            }
            FutharkExpr::Scatter(a, is, vs) => {
                self.push("scatter ");
                self.emit_expr_paren(a);
                self.push(" ");
                self.emit_expr_paren(is);
                self.push(" ");
                self.emit_expr_paren(vs);
            }
            FutharkExpr::Rotate(n, a) => {
                self.push("rotate ");
                self.emit_expr_paren(n);
                self.push(" ");
                self.emit_expr_paren(a);
            }
            FutharkExpr::Transpose(a) => {
                self.push("transpose ");
                self.emit_expr_paren(a);
            }
            FutharkExpr::Flatten(a) => {
                self.push("flatten ");
                self.emit_expr_paren(a);
            }
            FutharkExpr::Unflatten(n, m, a) => {
                self.push("unflatten ");
                self.emit_expr_paren(n);
                self.push(" ");
                self.emit_expr_paren(m);
                self.push(" ");
                self.emit_expr_paren(a);
            }
            FutharkExpr::Size(dim, e) => {
                self.push(&format!("#{dim}("));
                self.emit_expr(e);
                self.push(")");
            }
            FutharkExpr::With(arr, idx, val) => {
                self.emit_expr_paren(arr);
                self.push(" with [");
                self.emit_expr(idx);
                self.push("] = ");
                self.emit_expr(val);
            }
        }
    }
    /// Emit an expression, parenthesised when it is complex.
    pub(super) fn emit_expr_paren(&mut self, expr: &FutharkExpr) {
        let needs_paren = matches!(
            expr,
            FutharkExpr::Lambda(..)
                | FutharkExpr::LetIn(..)
                | FutharkExpr::Loop(..)
                | FutharkExpr::IfThenElse(..)
                | FutharkExpr::BinOp(..)
                | FutharkExpr::Map(..)
                | FutharkExpr::Map2(..)
                | FutharkExpr::Reduce(..)
                | FutharkExpr::Scan(..)
                | FutharkExpr::Filter(..)
                | FutharkExpr::Apply(..)
                | FutharkExpr::Scatter(..)
                | FutharkExpr::Unflatten(..)
        );
        if needs_paren {
            self.push("(");
            self.emit_expr(expr);
            self.push(")");
        } else {
            self.emit_expr(expr);
        }
    }
    /// Emit a single Futhark statement.
    pub fn emit_stmt(&mut self, stmt: &FutharkStmt) {
        match stmt {
            FutharkStmt::LetBinding(name, ty, expr) => {
                self.emit_indent();
                self.push("let ");
                self.push(name);
                if let Some(t) = ty {
                    if self.config.annotate_lets {
                        self.push(": ");
                        self.emit_type(t);
                    }
                }
                self.push(" = ");
                self.emit_expr(expr);
                self.newline();
            }
            FutharkStmt::LetTupleBinding(names, expr) => {
                self.emit_indent();
                self.push("let (");
                for (i, n) in names.iter().enumerate() {
                    if i > 0 {
                        self.push(", ");
                    }
                    self.push(n);
                }
                self.push(") = ");
                self.emit_expr(expr);
                self.newline();
            }
            FutharkStmt::LoopBinding(acc, init, var, bound, body) => {
                self.emit_indent();
                self.push("let ");
                self.push(acc);
                self.push(" = loop (");
                self.push(acc);
                self.push(" = ");
                self.emit_expr(init);
                self.push(") for ");
                self.push(var);
                self.push(" < ");
                self.emit_expr(bound);
                self.push(" do");
                self.newline();
                self.indent_in();
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent_out();
            }
            FutharkStmt::ReturnExpr(expr) => {
                self.emit_indent();
                self.emit_expr(expr);
                self.newline();
            }
            FutharkStmt::Comment(text) => {
                self.emit_indent();
                self.push("-- ");
                self.push(text);
                self.newline();
            }
        }
    }
    /// Emit a Futhark function definition.
    pub fn emit_fun(&mut self, fun: &FutharkFun) {
        for attr in &fun.attrs {
            self.emit_line(&attr.to_string());
        }
        self.emit_indent();
        if fun.is_entry {
            self.push("entry ");
        } else {
            self.push("let ");
        }
        self.push(&fun.name);
        for tp in &fun.type_params {
            self.push(" '");
            self.push(tp);
        }
        for (pname, pty) in &fun.params {
            self.push(" (");
            self.push(pname);
            self.push(": ");
            self.emit_type(pty);
            self.push(")");
        }
        self.push(": ");
        self.emit_type(&fun.return_type.clone());
        self.push(" =");
        self.newline();
        self.indent_in();
        for stmt in &fun.body {
            self.emit_stmt(stmt);
        }
        self.indent_out();
        self.newline();
    }
    /// Emit a type alias definition.
    pub fn emit_type_alias(&mut self, alias: &FutharkTypeAlias) {
        self.emit_indent();
        if alias.is_opaque {
            self.push("type^ ");
        } else {
            self.push("type ");
        }
        self.push(&alias.name);
        for p in &alias.params {
            self.push(" '");
            self.push(p);
        }
        self.push(" = ");
        self.emit_type(&alias.ty.clone());
        self.newline();
    }
    /// Emit an entire Futhark module.
    pub fn emit_module(&mut self, module: &FutharkModule) {
        if let Some(doc) = &module.doc.clone() {
            for line in doc.lines() {
                self.push("-- | ");
                self.push(line);
                self.newline();
            }
            self.newline();
        }
        for open in &module.opens.clone() {
            self.emit_line(&format!("open {open}"));
        }
        if !module.opens.is_empty() {
            self.newline();
        }
        for alias in &module.types.clone() {
            self.emit_type_alias(alias);
        }
        if !module.types.is_empty() {
            self.newline();
        }
        for (name, ty, expr) in &module.constants.clone() {
            self.emit_indent();
            self.push("let ");
            self.push(name);
            self.push(": ");
            self.emit_type(ty);
            self.push(" = ");
            self.emit_expr(expr);
            self.newline();
        }
        if !module.constants.is_empty() {
            self.newline();
        }
        for fun in &module.funs.clone() {
            self.emit_fun(fun);
        }
    }
    /// Return the generated source and reset the buffer.
    pub fn finish(&mut self) -> String {
        std::mem::take(&mut self.buf)
    }
    /// Generate a complete `.fut` file from a module.
    pub fn generate(module: &FutharkModule) -> String {
        let mut backend = FutharkBackend::new();
        backend.emit_module(module);
        backend.finish()
    }
    /// Generate with custom configuration.
    pub fn generate_with_config(module: &FutharkModule, config: FutharkConfig) -> String {
        let mut backend = FutharkBackend::with_config(config);
        backend.emit_module(module);
        backend.finish()
    }
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FutharkDiagSink {
    pub diags: Vec<FutharkDiag>,
}
#[allow(dead_code)]
impl FutharkDiagSink {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn info(&mut self, msg: &str) {
        self.diags.push(FutharkDiag {
            level: FutharkDiagLevel::Info,
            message: msg.to_string(),
            location: None,
        });
    }
    pub fn warn(&mut self, msg: &str) {
        self.diags.push(FutharkDiag {
            level: FutharkDiagLevel::Warning,
            message: msg.to_string(),
            location: None,
        });
    }
    pub fn error(&mut self, msg: &str) {
        self.diags.push(FutharkDiag {
            level: FutharkDiagLevel::Error,
            message: msg.to_string(),
            location: None,
        });
    }
    pub fn has_errors(&self) -> bool {
        self.diags
            .iter()
            .any(|d| d.level == FutharkDiagLevel::Error)
    }
    pub fn error_count(&self) -> usize {
        self.diags
            .iter()
            .filter(|d| d.level == FutharkDiagLevel::Error)
            .count()
    }
    pub fn warning_count(&self) -> usize {
        self.diags
            .iter()
            .filter(|d| d.level == FutharkDiagLevel::Warning)
            .count()
    }
}
/// Futhark type representation.
#[derive(Debug, Clone, PartialEq)]
pub enum FutharkType {
    /// `i8`
    I8,
    /// `i16`
    I16,
    /// `i32`
    I32,
    /// `i64`
    I64,
    /// `u8`
    U8,
    /// `u16`
    U16,
    /// `u32`
    U32,
    /// `u64`
    U64,
    /// `f16`
    F16,
    /// `f32`
    F32,
    /// `f64`
    F64,
    /// `bool`
    Bool,
    /// Multi-dimensional array: `[n][m]...t`
    Array(Box<FutharkType>, Vec<Option<String>>),
    /// Tuple: `(t1, t2, ...)`
    Tuple(Vec<FutharkType>),
    /// Record: `{field: type, ...}`
    Record(Vec<(String, FutharkType)>),
    /// Opaque type (abstract/named): `#[opaque] type Foo`
    Opaque(String),
    /// Named (user-defined) type alias
    Named(String),
    /// Parametric type: `name 't`
    Parametric(String, Vec<String>),
}
/// Futhark replicate expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkReplicate {
    pub n: String,
    pub value: String,
}
/// Array literal in Futhark
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkArrayLiteral {
    pub elements: Vec<String>,
    pub element_type: FutharkType,
}
/// Futhark in-place update
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkInPlaceUpdate {
    pub array: String,
    pub index: String,
    pub value: String,
}
/// Futhark reduce_by_index (histogram)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkReduceByIndex {
    pub dest: String,
    pub op: String,
    pub neutral: String,
    pub indices: String,
    pub values: String,
}
/// Futhark GPU backend selector
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FutharkGpuBackend {
    OpenCL,
    CUDA,
    Hip,
    Sequential,
    Multicore,
    IsPC,
    WGpu,
}
/// Futhark reduce expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkReduceExpr {
    pub op: String,
    pub neutral: String,
    pub array: String,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkUnflatten {
    pub n: String,
    pub m: String,
    pub array: String,
}
/// Futhark expression representation.
#[derive(Debug, Clone)]
pub enum FutharkExpr {
    /// Integer literal: `42i32`
    IntLit(i64, FutharkType),
    /// Float literal: `3.14f64`
    FloatLit(f64, FutharkType),
    /// Bool literal: `true` / `false`
    BoolLit(bool),
    /// Variable reference: `x`
    Var(String),
    /// Array literal: `[e1, e2, ...]`
    ArrayLit(Vec<FutharkExpr>),
    /// Array index: `a[i]`
    Index(Box<FutharkExpr>, Box<FutharkExpr>),
    /// Array slice: `a[lo:hi]`
    Slice(
        Box<FutharkExpr>,
        Option<Box<FutharkExpr>>,
        Option<Box<FutharkExpr>>,
    ),
    /// `map f a`
    Map(Box<FutharkExpr>, Box<FutharkExpr>),
    /// `map2 f a b`
    Map2(Box<FutharkExpr>, Box<FutharkExpr>, Box<FutharkExpr>),
    /// `reduce op ne a`
    Reduce(Box<FutharkExpr>, Box<FutharkExpr>, Box<FutharkExpr>),
    /// `scan op ne a`
    Scan(Box<FutharkExpr>, Box<FutharkExpr>, Box<FutharkExpr>),
    /// `filter f a`
    Filter(Box<FutharkExpr>, Box<FutharkExpr>),
    /// `zip a b`
    Zip(Box<FutharkExpr>, Box<FutharkExpr>),
    /// `unzip a`
    Unzip(Box<FutharkExpr>),
    /// `iota n`
    Iota(Box<FutharkExpr>),
    /// `replicate n x`
    Replicate(Box<FutharkExpr>, Box<FutharkExpr>),
    /// `if cond then t else e`
    IfThenElse(Box<FutharkExpr>, Box<FutharkExpr>, Box<FutharkExpr>),
    /// Lambda: `\\ x -> body`
    Lambda(Vec<(String, FutharkType)>, Box<FutharkExpr>),
    /// Let-in: `let x = e in body`
    LetIn(
        String,
        Option<FutharkType>,
        Box<FutharkExpr>,
        Box<FutharkExpr>,
    ),
    /// Loop: `loop (acc = init) for i < n do body`
    Loop(
        String,
        Box<FutharkExpr>,
        String,
        Box<FutharkExpr>,
        Box<FutharkExpr>,
    ),
    /// Binary operation: `e1 op e2`
    BinOp(String, Box<FutharkExpr>, Box<FutharkExpr>),
    /// Unary operation: `op e`
    UnOp(String, Box<FutharkExpr>),
    /// Function application: `f a1 a2 ...`
    Apply(Box<FutharkExpr>, Vec<FutharkExpr>),
    /// Tuple construction: `(e1, e2, ...)`
    TupleLit(Vec<FutharkExpr>),
    /// Record construction: `{field = e, ...}`
    RecordLit(Vec<(String, FutharkExpr)>),
    /// Field access: `r.field`
    FieldAccess(Box<FutharkExpr>, String),
    /// Type ascription: `e : t`
    Ascribe(Box<FutharkExpr>, FutharkType),
    /// Scatter: `scatter a is vs`
    Scatter(Box<FutharkExpr>, Box<FutharkExpr>, Box<FutharkExpr>),
    /// `rotate n a`
    Rotate(Box<FutharkExpr>, Box<FutharkExpr>),
    /// `transpose a`
    Transpose(Box<FutharkExpr>),
    /// Flatten: `flatten a`
    Flatten(Box<FutharkExpr>),
    /// `unflatten n m a`
    Unflatten(Box<FutharkExpr>, Box<FutharkExpr>, Box<FutharkExpr>),
    /// Size expression: `#(e)`
    Size(usize, Box<FutharkExpr>),
    /// `with` update: `a with [i] = v`
    With(Box<FutharkExpr>, Box<FutharkExpr>, Box<FutharkExpr>),
}
/// Futhark version info
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkExtVersionInfo {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub git_rev: Option<String>,
}
/// Futhark filter expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkFilterExpr {
    pub pred: String,
    pub array: String,
}
/// Futhark module importer
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FutharkModuleImporter {
    pub imported: Vec<String>,
    pub opened: Vec<String>,
}
#[allow(dead_code)]
impl FutharkModuleImporter {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn import(&mut self, path: &str) {
        self.imported.push(path.to_string());
    }
    pub fn open(&mut self, path: &str) {
        self.opened.push(path.to_string());
    }
    pub fn emit(&self) -> String {
        let mut out = String::new();
        for p in &self.imported {
            out.push_str(&format!("import \"{}\"\n", p));
        }
        for p in &self.opened {
            out.push_str(&format!("open import \"{}\"\n", p));
        }
        out
    }
}
/// Futhark scan expression
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkScanExpr {
    pub op: String,
    pub neutral: String,
    pub array: String,
}
/// Futhark transpose
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkTranspose {
    pub array: String,
}
/// Extended Futhark backend config
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FutharkExtConfig {
    pub target_version: FutharkVersion,
    pub emit_safety_checks: bool,
    pub inline_threshold: usize,
    pub vectorize_threshold: usize,
    pub emit_comments: bool,
    pub mangle_names: bool,
    pub backend_target: String,
}
