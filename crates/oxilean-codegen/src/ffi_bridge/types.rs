//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{BTreeMap, HashMap};

use super::functions::*;
use std::collections::{HashSet, VecDeque};

/// FFI platform target
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FfiPlatformTarget {
    Linux,
    Windows,
    MacOS,
    FreeBSD,
    Wasm32,
    Wasm64,
    Android,
    Ios,
    Universal,
}
/// FFI header builder
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FfiHeaderBuilder {
    pub guard: Option<String>,
    pub includes: Vec<String>,
    pub typedefs: Vec<FfiTypedef>,
    pub structs: Vec<FfiStructDef>,
    pub enums: Vec<FfiEnumDef>,
    pub functions: Vec<FfiFuncSignature>,
    pub constants: Vec<FfiConst>,
    pub callbacks: Vec<FfiCallbackType>,
}
#[allow(dead_code)]
impl FfiHeaderBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_guard(mut self, guard: &str) -> Self {
        self.guard = Some(guard.to_string());
        self
    }
    pub fn add_include(&mut self, inc: &str) {
        self.includes.push(inc.to_string());
    }
    pub fn add_typedef(&mut self, td: FfiTypedef) {
        self.typedefs.push(td);
    }
    pub fn add_struct(&mut self, s: FfiStructDef) {
        self.structs.push(s);
    }
    pub fn add_enum(&mut self, e: FfiEnumDef) {
        self.enums.push(e);
    }
    pub fn add_function(&mut self, f: FfiFuncSignature) {
        self.functions.push(f);
    }
    pub fn add_const(&mut self, c: FfiConst) {
        self.constants.push(c);
    }
    pub fn add_callback(&mut self, cb: FfiCallbackType) {
        self.callbacks.push(cb);
    }
    pub fn build(&self) -> String {
        let mut out = String::new();
        if let Some(guard) = &self.guard {
            out.push_str(&format!("#ifndef {}\n#define {}\n\n", guard, guard));
        }
        for inc in &self.includes {
            if inc.starts_with('<') {
                out.push_str(&format!("#include {}\n", inc));
            } else {
                out.push_str(&format!("#include \"{}\"\n", inc));
            }
        }
        if !self.includes.is_empty() {
            out.push('\n');
        }
        for td in &self.typedefs {
            out.push_str(&format!("{}\n", td));
        }
        for s in &self.structs {
            out.push_str(&format!("{}\n\n", s));
        }
        for e in &self.enums {
            out.push_str(&format!("{}\n\n", e));
        }
        for cb in &self.callbacks {
            out.push_str(&format!("{};\n", cb));
        }
        if !self.callbacks.is_empty() {
            out.push('\n');
        }
        for c in &self.constants {
            out.push_str(&format!("{}\n", c));
        }
        if !self.constants.is_empty() {
            out.push('\n');
        }
        for func in &self.functions {
            out.push_str(&format!("{};\n", func));
        }
        if let Some(guard) = &self.guard {
            out.push_str(&format!("\n#endif /* {} */\n", guard));
        }
        out
    }
}
/// A diagnostic message from a FfiExt pass.
#[derive(Debug, Clone)]
pub struct FfiExtDiagMsg {
    pub severity: FfiExtDiagSeverity,
    pub pass: String,
    pub message: String,
}
impl FfiExtDiagMsg {
    pub fn error(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        FfiExtDiagMsg {
            severity: FfiExtDiagSeverity::Error,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn warning(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        FfiExtDiagMsg {
            severity: FfiExtDiagSeverity::Warning,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn note(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        FfiExtDiagMsg {
            severity: FfiExtDiagSeverity::Note,
            pass: pass.into(),
            message: msg.into(),
        }
    }
}
/// A version tag for FfiExt output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FfiExtVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
impl FfiExtVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        FfiExtVersion {
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
    pub fn is_compatible_with(&self, other: &FfiExtVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
/// A monotonically increasing ID generator for FfiExt.
#[derive(Debug, Default)]
pub struct FfiExtIdGen {
    pub(super) next: u32,
}
impl FfiExtIdGen {
    pub fn new() -> Self {
        FfiExtIdGen::default()
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
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FFIPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl FFIPassStats {
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
/// FFI verification result
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiVerifyResult {
    pub ok: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}
/// An FFI function declaration.
#[derive(Debug, Clone)]
pub struct FfiDecl {
    /// The OxiLean/LCNF name of the function.
    pub name: String,
    /// The external (C) name of the function.
    pub extern_name: String,
    /// Parameter types.
    pub params: Vec<(String, FfiNativeType)>,
    /// Return type.
    pub ret_type: FfiNativeType,
    /// Calling convention.
    pub calling_conv: CallingConvention,
    /// Whether the function is unsafe.
    pub is_unsafe: bool,
}
/// FFI Python CFFI binding
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FfiPythonCffi {
    pub ffi_defs: Vec<String>,
    pub lib_name: String,
    pub header_file: String,
}
#[allow(dead_code)]
impl FfiPythonCffi {
    pub fn new(lib: &str, header: &str) -> Self {
        Self {
            ffi_defs: Vec::new(),
            lib_name: lib.to_string(),
            header_file: header.to_string(),
        }
    }
    pub fn add_def(&mut self, def: &str) {
        self.ffi_defs.push(def.to_string());
    }
    pub fn emit(&self) -> String {
        let defs = self.ffi_defs.join("\n");
        format!(
            "from cffi import FFI\nffi = FFI()\nffi.cdef(\"\"\"\n{}\n\"\"\")\nlib = ffi.dlopen(\"lib{}.so\")\n",
            defs, self.lib_name
        )
    }
}
#[allow(dead_code)]
pub struct FFIPassRegistry {
    pub(super) configs: Vec<FFIPassConfig>,
    pub(super) stats: std::collections::HashMap<String, FFIPassStats>,
}
impl FFIPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        FFIPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: FFIPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), FFIPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&FFIPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&FFIPassStats> {
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
/// FFI parameter attribute
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FfiParamAttr {
    In,
    Out,
    InOut,
    Const,
    Volatile,
    Restrict,
    NullTerminated,
    Nonnull,
    Nullable,
    Retain,
    Escaping,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FfiRustBindingKind {
    Function,
    Struct,
    Enum,
    Type,
    Const,
}
/// FFI Zig binding
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FfiZigBinding {
    pub pub_fns: Vec<String>,
    pub types: Vec<String>,
    pub lib_path: String,
}
#[allow(dead_code)]
impl FfiZigBinding {
    pub fn new(lib_path: &str) -> Self {
        Self {
            pub_fns: Vec::new(),
            types: Vec::new(),
            lib_path: lib_path.to_string(),
        }
    }
    pub fn add_fn(&mut self, f: &str) {
        self.pub_fns.push(f.to_string());
    }
    pub fn add_type(&mut self, t: &str) {
        self.types.push(t.to_string());
    }
    pub fn emit(&self) -> String {
        let mut out = format!("const c = @cImport(@cInclude(\"{}\"));\n\n", self.lib_path);
        for t in &self.types {
            out.push_str(&format!("pub const {} = c.{};\n", t, t));
        }
        for f in &self.pub_fns {
            out.push_str(&format!("pub const {} = c.{};\n", f, f));
        }
        out
    }
}
/// The generated FFI binding output.
#[derive(Debug, Clone)]
pub struct FfiOutput {
    /// Generated Rust `extern "C"` bindings.
    pub rust_bindings: String,
    /// Generated C header declarations.
    pub c_header: String,
    /// Generated C wrapper functions.
    pub wrapper_fns: Vec<String>,
}
impl FfiOutput {
    pub(super) fn new() -> Self {
        FfiOutput {
            rust_bindings: String::new(),
            c_header: String::new(),
            wrapper_fns: Vec::new(),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FFICacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A feature flag set for FfiExt capabilities.
#[derive(Debug, Clone, Default)]
pub struct FfiExtFeatures {
    pub(super) flags: std::collections::HashSet<String>,
}
impl FfiExtFeatures {
    pub fn new() -> Self {
        FfiExtFeatures::default()
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
    pub fn union(&self, other: &FfiExtFeatures) -> FfiExtFeatures {
        FfiExtFeatures {
            flags: self.flags.union(&other.flags).cloned().collect(),
        }
    }
    pub fn intersection(&self, other: &FfiExtFeatures) -> FfiExtFeatures {
        FfiExtFeatures {
            flags: self.flags.intersection(&other.flags).cloned().collect(),
        }
    }
}
/// Native C type for FFI boundaries.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FfiNativeType {
    /// `void`
    Void,
    /// `int8_t`
    I8,
    /// `int16_t`
    I16,
    /// `int32_t`
    I32,
    /// `int64_t`
    I64,
    /// `uint8_t`
    U8,
    /// `uint16_t`
    U16,
    /// `uint32_t`
    U32,
    /// `uint64_t`
    U64,
    /// `float`
    F32,
    /// `double`
    F64,
    /// `bool` (C99 _Bool)
    Bool,
    /// `size_t`
    SizeT,
    /// `char*` (C string)
    CStr,
    /// Generic pointer `T*`
    Ptr(Box<FfiNativeType>),
    /// Opaque pointer `void*`
    OpaquePtr,
    /// `lean_object*`
    LeanObject,
    /// Struct with named fields.
    Struct(String, Vec<(String, FfiNativeType)>),
    /// Function pointer.
    FnPtr(Vec<FfiNativeType>, Box<FfiNativeType>),
}
impl FfiNativeType {
    /// Convert to a Rust type string.
    pub(super) fn to_rust_type(&self) -> String {
        match self {
            FfiNativeType::Void => "()".to_string(),
            FfiNativeType::I8 => "i8".to_string(),
            FfiNativeType::I16 => "i16".to_string(),
            FfiNativeType::I32 => "i32".to_string(),
            FfiNativeType::I64 => "i64".to_string(),
            FfiNativeType::U8 => "u8".to_string(),
            FfiNativeType::U16 => "u16".to_string(),
            FfiNativeType::U32 => "u32".to_string(),
            FfiNativeType::U64 => "u64".to_string(),
            FfiNativeType::F32 => "f32".to_string(),
            FfiNativeType::F64 => "f64".to_string(),
            FfiNativeType::Bool => "bool".to_string(),
            FfiNativeType::SizeT => "usize".to_string(),
            FfiNativeType::CStr => "*const std::os::raw::c_char".to_string(),
            FfiNativeType::Ptr(inner) => format!("*mut {}", inner.to_rust_type()),
            FfiNativeType::OpaquePtr => "*mut std::os::raw::c_void".to_string(),
            FfiNativeType::LeanObject => "*mut LeanObject".to_string(),
            FfiNativeType::Struct(name, _) => name.clone(),
            FfiNativeType::FnPtr(params, ret) => {
                let param_strs: Vec<String> = params.iter().map(|p| p.to_rust_type()).collect();
                format!(
                    "Option<unsafe extern \"C\" fn({}) -> {}>",
                    param_strs.join(", "),
                    ret.to_rust_type()
                )
            }
        }
    }
    /// Size in bytes (approximate, for validation).
    pub(super) fn size_bytes(&self) -> usize {
        match self {
            FfiNativeType::Void => 0,
            FfiNativeType::I8 | FfiNativeType::U8 | FfiNativeType::Bool => 1,
            FfiNativeType::I16 | FfiNativeType::U16 => 2,
            FfiNativeType::I32 | FfiNativeType::U32 | FfiNativeType::F32 => 4,
            FfiNativeType::I64
            | FfiNativeType::U64
            | FfiNativeType::F64
            | FfiNativeType::SizeT
            | FfiNativeType::CStr
            | FfiNativeType::Ptr(_)
            | FfiNativeType::OpaquePtr
            | FfiNativeType::LeanObject
            | FfiNativeType::FnPtr(_, _) => 8,
            FfiNativeType::Struct(_, fields) => fields.iter().map(|(_, ty)| ty.size_bytes()).sum(),
        }
    }
}
/// Tracks declared names for FfiExt scope analysis.
#[derive(Debug, Default)]
pub struct FfiExtNameScope {
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<FfiExtNameScope>>,
}
impl FfiExtNameScope {
    pub fn new() -> Self {
        FfiExtNameScope::default()
    }
    pub fn declare(&mut self, name: impl Into<String>) -> bool {
        self.declared.insert(name.into())
    }
    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name)
    }
    pub fn push_scope(self) -> Self {
        FfiExtNameScope {
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
/// FFI bridge statistics v2
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct FfiBridgeStatsV2 {
    pub functions_bridged: usize,
    pub structs_bridged: usize,
    pub enums_bridged: usize,
    pub typedefs_emitted: usize,
    pub callbacks_emitted: usize,
    pub constants_emitted: usize,
    pub headers_generated: usize,
    pub bytes_total: usize,
}
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum FFIPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl FFIPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            FFIPassPhase::Analysis => "analysis",
            FFIPassPhase::Transformation => "transformation",
            FFIPassPhase::Verification => "verification",
            FFIPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, FFIPassPhase::Transformation | FFIPassPhase::Cleanup)
    }
}
/// FFI null termination policy
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FfiNullTermPolicy {
    Always,
    Never,
    WhenNonNull,
}
/// FFI buffer parameter (slice/buffer with size)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiBufferParam {
    pub data: FfiFuncParam,
    pub size: FfiSizeHint,
    pub null_term: FfiNullTermPolicy,
}
/// Information about how to marshal a type across the FFI boundary.
#[derive(Debug, Clone)]
pub struct FfiMarshalInfo {
    /// The native (C) type to use at the FFI boundary.
    pub native_type: FfiNativeType,
    /// Code to convert from LCNF/Lean object to native type.
    pub to_native: String,
    /// Code to convert from native type to LCNF/Lean object.
    pub from_native: String,
    /// Whether the conversion is trivial (no-op).
    pub is_trivial: bool,
    /// Whether the native type needs to be freed after use.
    pub needs_free: bool,
}
impl FfiMarshalInfo {
    /// Create a trivial (identity) marshalling.
    pub(super) fn trivial(native_type: FfiNativeType) -> Self {
        FfiMarshalInfo {
            native_type,
            to_native: String::new(),
            from_native: String::new(),
            is_trivial: true,
            needs_free: false,
        }
    }
    /// Create a marshalling with conversion functions.
    pub(super) fn with_conversion(
        native_type: FfiNativeType,
        to_native: &str,
        from_native: &str,
    ) -> Self {
        FfiMarshalInfo {
            native_type,
            to_native: to_native.to_string(),
            from_native: from_native.to_string(),
            is_trivial: false,
            needs_free: false,
        }
    }
}
/// FFI calling convention
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FfiCallingConv {
    C,
    StdCall,
    FastCall,
    ThisCall,
    VectorCall,
    Win64,
    SysV64,
    Swift,
    Rust,
    Custom(String),
}
/// FFI profiler v2
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FfiExtProfilerV2 {
    pub timings: Vec<(String, u64)>,
}
#[allow(dead_code)]
impl FfiExtProfilerV2 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record(&mut self, pass: &str, us: u64) {
        self.timings.push((pass.to_string(), us));
    }
    pub fn total_us(&self) -> u64 {
        self.timings.iter().map(|(_, t)| *t).sum()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FFIDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl FFIDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        FFIDominatorTree {
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
/// FFI error table
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FfiErrorTable {
    pub codes: Vec<FfiErrorCode>,
}
#[allow(dead_code)]
impl FfiErrorTable {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add(&mut self, code: FfiErrorCode) {
        self.codes.push(code);
    }
    pub fn lookup(&self, value: i32) -> Option<&FfiErrorCode> {
        self.codes.iter().find(|c| c.value == value)
    }
    pub fn emit_enum(&self, name: &str) -> String {
        let mut out = format!("enum {} {{\n", name);
        for c in &self.codes {
            out.push_str(&format!("    {} = {},\n", c.name, c.value));
        }
        out.push_str("};\n");
        out
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FFIPassConfig {
    pub phase: FFIPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl FFIPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: FFIPassPhase) -> Self {
        FFIPassConfig {
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
/// FFI enum definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiEnumDef {
    pub name: String,
    pub variants: Vec<(String, Option<i64>)>,
    pub underlying_type: Option<String>,
}
/// Collects FfiExt diagnostics.
#[derive(Debug, Default)]
pub struct FfiExtDiagCollector {
    pub(super) msgs: Vec<FfiExtDiagMsg>,
}
impl FfiExtDiagCollector {
    pub fn new() -> Self {
        FfiExtDiagCollector::default()
    }
    pub fn emit(&mut self, d: FfiExtDiagMsg) {
        self.msgs.push(d);
    }
    pub fn has_errors(&self) -> bool {
        self.msgs
            .iter()
            .any(|d| d.severity == FfiExtDiagSeverity::Error)
    }
    pub fn errors(&self) -> Vec<&FfiExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == FfiExtDiagSeverity::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&FfiExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == FfiExtDiagSeverity::Warning)
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
/// A fixed-capacity ring buffer of strings (for recent-event logging in FfiExt).
#[derive(Debug)]
pub struct FfiExtEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
}
impl FfiExtEventLog {
    pub fn new(capacity: usize) -> Self {
        FfiExtEventLog {
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
/// Pass-timing record for FfiExt profiler.
#[derive(Debug, Clone)]
pub struct FfiExtPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
impl FfiExtPassTiming {
    pub fn new(
        pass_name: impl Into<String>,
        elapsed_us: u64,
        items: usize,
        before: usize,
        after: usize,
    ) -> Self {
        FfiExtPassTiming {
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
/// Bidirectional mapping between LCNF types and C types.
#[derive(Debug, Clone, Default)]
pub struct FfiTypeMap {
    /// LCNF type name -> FFI native type.
    pub(super) lcnf_to_native: HashMap<String, FfiNativeType>,
    /// FFI native type name -> LCNF type.
    pub(super) native_to_lcnf: HashMap<String, LcnfType>,
    /// Custom marshalling rules.
    pub(super) custom_marshal: HashMap<String, FfiMarshalInfo>,
}
impl FfiTypeMap {
    /// Create a new type map with default entries.
    pub fn new() -> Self {
        let mut map = FfiTypeMap::default();
        map.register("Nat", FfiNativeType::U64, LcnfType::Nat);
        map.register("String", FfiNativeType::CStr, LcnfType::LcnfString);
        map.register("Unit", FfiNativeType::Void, LcnfType::Unit);
        map.register("UInt8", FfiNativeType::U8, LcnfType::Nat);
        map.register("UInt16", FfiNativeType::U16, LcnfType::Nat);
        map.register("UInt32", FfiNativeType::U32, LcnfType::Nat);
        map.register("UInt64", FfiNativeType::U64, LcnfType::Nat);
        map.register("Int8", FfiNativeType::I8, LcnfType::Nat);
        map.register("Int16", FfiNativeType::I16, LcnfType::Nat);
        map.register("Int32", FfiNativeType::I32, LcnfType::Nat);
        map.register("Int64", FfiNativeType::I64, LcnfType::Nat);
        map.register("Float", FfiNativeType::F64, LcnfType::Nat);
        map.register("Bool", FfiNativeType::Bool, LcnfType::Nat);
        map
    }
    /// Register a bidirectional type mapping.
    pub fn register(&mut self, name: &str, native: FfiNativeType, lcnf: LcnfType) {
        self.lcnf_to_native.insert(name.to_string(), native.clone());
        self.native_to_lcnf.insert(native.to_string(), lcnf);
    }
    /// Register a custom marshalling rule.
    pub fn register_marshal(&mut self, name: &str, info: FfiMarshalInfo) {
        self.custom_marshal.insert(name.to_string(), info);
    }
    /// Look up the native type for an LCNF type name.
    pub fn to_native(&self, name: &str) -> Option<&FfiNativeType> {
        self.lcnf_to_native.get(name)
    }
    /// Look up the LCNF type for a native type.
    pub fn to_lcnf(&self, native_name: &str) -> Option<&LcnfType> {
        self.native_to_lcnf.get(native_name)
    }
    /// Get custom marshalling for a type, if registered.
    pub fn get_marshal(&self, name: &str) -> Option<&FfiMarshalInfo> {
        self.custom_marshal.get(name)
    }
}
/// FFI callback type
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiCallbackType {
    pub name: String,
    pub ret_type: String,
    pub params: Vec<FfiFuncParam>,
    pub calling_conv: FfiCallingConv,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FFIWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl FFIWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        FFIWorklist {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FFIDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl FFIDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        FFIDepGraph {
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
/// FFI error code description
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiErrorCode {
    pub value: i32,
    pub name: String,
    pub description: String,
}
/// FFI code stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct FfiCodeStats {
    pub functions: usize,
    pub structs: usize,
    pub enums: usize,
    pub typedefs: usize,
    pub constants: usize,
    pub callbacks: usize,
    pub total_bytes: usize,
}
/// FFI rust bindings file
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FfiRustBindingsFile {
    pub preamble: String,
    pub extern_c: Vec<FfiRustBinding>,
    pub types: Vec<FfiRustBinding>,
    pub constants: Vec<FfiRustBinding>,
}
#[allow(dead_code)]
impl FfiRustBindingsFile {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_extern(&mut self, b: FfiRustBinding) {
        self.extern_c.push(b);
    }
    pub fn add_type(&mut self, b: FfiRustBinding) {
        self.types.push(b);
    }
    pub fn add_const(&mut self, b: FfiRustBinding) {
        self.constants.push(b);
    }
    pub fn emit(&self) -> String {
        let mut out = self.preamble.clone();
        if !self.types.is_empty() {
            out.push_str("\n// --- Types ---\n");
            for t in &self.types {
                out.push_str(&format!("{}\n", t.source));
            }
        }
        if !self.constants.is_empty() {
            out.push_str("\n// --- Constants ---\n");
            for c in &self.constants {
                out.push_str(&format!("{}\n", c.source));
            }
        }
        if !self.extern_c.is_empty() {
            out.push_str("\nextern \"C\" {\n");
            for e in &self.extern_c {
                out.push_str(&format!("    {}\n", e.source));
            }
            out.push_str("}\n");
        }
        out
    }
}
/// FFI typedef
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiTypedef {
    pub name: String,
    pub base_type: String,
}
/// FFI validation errors.
#[derive(Debug, Clone)]
pub enum FfiError {
    /// Unsupported type at FFI boundary.
    UnsupportedType(String),
    /// Invalid calling convention for the target.
    InvalidCallingConvention(CallingConvention, String),
    /// Parameter count mismatch.
    ParamCountMismatch { expected: usize, found: usize },
    /// Type mismatch at a parameter.
    TypeMismatch {
        param: String,
        expected: String,
        found: String,
    },
    /// Struct too large to pass by value.
    StructTooLarge { name: String, size: usize },
    /// Recursive type at FFI boundary.
    RecursiveType(String),
    /// Generic error message.
    Other(String),
}
/// The FFI bridge generator.
///
/// Produces Rust extern blocks, C headers, and wrapper functions
/// for interoperating between OxiLean and C code.
pub struct FfiBridge {
    /// Type mapping.
    pub(super) type_map: FfiTypeMap,
    /// Generated declarations.
    pub(super) declarations: Vec<FfiDecl>,
}
impl FfiBridge {
    /// Create a new FFI bridge with default type mappings.
    pub fn new() -> Self {
        FfiBridge {
            type_map: FfiTypeMap::new(),
            declarations: Vec::new(),
        }
    }
    /// Create an FFI bridge with a custom type map.
    pub fn with_type_map(type_map: FfiTypeMap) -> Self {
        FfiBridge {
            type_map,
            declarations: Vec::new(),
        }
    }
    /// Generate all FFI bindings for the given declarations.
    pub fn generate_bindings(&self, decls: &[FfiDecl]) -> FfiOutput {
        let mut output = FfiOutput::new();
        output.rust_bindings = self.generate_extern_block(decls);
        output.c_header = self.generate_c_header(decls);
        for decl in decls {
            let wrapper = self.generate_c_wrapper(decl);
            output.wrapper_fns.push(wrapper);
        }
        output
    }
    /// Generate a Rust `extern "C"` block for the given declarations.
    pub fn generate_extern_block(&self, decls: &[FfiDecl]) -> String {
        let mut code = String::new();
        code.push_str("// Auto-generated FFI bindings for OxiLean\n\n");
        let mut by_cc: BTreeMap<String, Vec<&FfiDecl>> = BTreeMap::new();
        for decl in decls {
            let cc = match decl.calling_conv {
                CallingConvention::C => "C",
                CallingConvention::Rust => "Rust",
                CallingConvention::System => "system",
                CallingConvention::Fastcall => "fastcall",
            };
            by_cc.entry(cc.to_string()).or_default().push(decl);
        }
        for (cc, cc_decls) in &by_cc {
            code.push_str(&format!("extern \"{}\" {{\n", cc));
            for decl in cc_decls {
                code.push_str("    ");
                if decl.is_unsafe {
                    code.push_str("// unsafe\n    ");
                }
                code.push_str(&format!("fn {}(", decl.extern_name));
                for (i, (pname, pty)) in decl.params.iter().enumerate() {
                    if i > 0 {
                        code.push_str(", ");
                    }
                    code.push_str(&format!("{}: {}", pname, pty.to_rust_type()));
                }
                let ret_str = decl.ret_type.to_rust_type();
                if ret_str == "()" {
                    code.push_str(");\n");
                } else {
                    code.push_str(&format!(") -> {};\n", ret_str));
                }
            }
            code.push_str("}\n\n");
        }
        code
    }
    /// Generate a C header with function prototypes.
    pub(super) fn generate_c_header(&self, decls: &[FfiDecl]) -> String {
        let mut code = String::new();
        code.push_str("/* Auto-generated C header for OxiLean FFI */\n\n");
        code.push_str("#ifndef OXILEAN_FFI_H\n");
        code.push_str("#define OXILEAN_FFI_H\n\n");
        code.push_str("#include <stdint.h>\n");
        code.push_str("#include <stddef.h>\n");
        code.push_str("#include <stdbool.h>\n");
        code.push_str("#include \"lean_runtime.h\"\n\n");
        code.push_str("#ifdef __cplusplus\n");
        code.push_str("extern \"C\" {\n");
        code.push_str("#endif\n\n");
        for decl in decls {
            code.push_str(&format!("/* {} */\n", decl.name));
            code.push_str(&format!("{} {}(", decl.ret_type, decl.extern_name));
            for (i, (pname, pty)) in decl.params.iter().enumerate() {
                if i > 0 {
                    code.push_str(", ");
                }
                code.push_str(&format!("{} {}", pty, pname));
            }
            if decl.params.is_empty() {
                code.push_str("void");
            }
            code.push_str(");\n\n");
        }
        code.push_str("#ifdef __cplusplus\n");
        code.push_str("}\n");
        code.push_str("#endif\n\n");
        code.push_str("#endif /* OXILEAN_FFI_H */\n");
        code
    }
    /// Generate a C wrapper function that calls an OxiLean function
    /// via the Lean runtime.
    pub fn generate_c_wrapper(&self, decl: &FfiDecl) -> String {
        let mut code = String::new();
        code.push_str(&format!("/* Wrapper for {} */\n", decl.name));
        code.push_str(&format!("{} {}_wrapper(", decl.ret_type, decl.extern_name));
        for (i, (pname, pty)) in decl.params.iter().enumerate() {
            if i > 0 {
                code.push_str(", ");
            }
            code.push_str(&format!("{} {}", pty, pname));
        }
        if decl.params.is_empty() {
            code.push_str("void");
        }
        code.push_str(") {\n");
        let mut lean_args = Vec::new();
        for (pname, pty) in &decl.params {
            let marshal = marshal_native_to_lean(pty, pname);
            if marshal.is_trivial {
                lean_args.push(pname.clone());
            } else {
                let lean_var = format!("_lean_{}", pname);
                code.push_str(&format!(
                    "  lean_object* {} = {};\n",
                    lean_var,
                    marshal.to_native.replace("${arg}", pname),
                ));
                lean_args.push(lean_var);
            }
        }
        let mangled_name = mangle_lean_name(&decl.name);
        if decl.ret_type == FfiNativeType::Void {
            code.push_str(&format!("  {}(", mangled_name));
            code.push_str(&lean_args.join(", "));
            code.push_str(");\n");
        } else {
            code.push_str(&format!("  lean_object* _result = {}(", mangled_name));
            code.push_str(&lean_args.join(", "));
            code.push_str(");\n");
            let ret_marshal = marshal_lean_to_native(&decl.ret_type, "_result");
            if ret_marshal.is_trivial {
                code.push_str("  return _result;\n");
            } else {
                code.push_str(&format!(
                    "  {} _native_result = {};\n",
                    decl.ret_type,
                    ret_marshal.from_native.replace("${result}", "_result"),
                ));
                code.push_str("  lean_dec_ref(_result);\n");
                code.push_str("  return _native_result;\n");
            }
        }
        code.push_str("}\n");
        code
    }
    /// Get the type map.
    pub fn type_map(&self) -> &FfiTypeMap {
        &self.type_map
    }
    /// Get a mutable reference to the type map.
    pub fn type_map_mut(&mut self) -> &mut FfiTypeMap {
        &mut self.type_map
    }
}
/// Pipeline profiler for FfiExt.
#[derive(Debug, Default)]
pub struct FfiExtProfiler {
    pub(super) timings: Vec<FfiExtPassTiming>,
}
impl FfiExtProfiler {
    pub fn new() -> Self {
        FfiExtProfiler::default()
    }
    pub fn record(&mut self, t: FfiExtPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&FfiExtPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&FfiExtPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
/// FFI size hint
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FfiSizeHint {
    Fixed(u64),
    Param(String),
    Dynamic,
}
/// FFI rust-side generated binding
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiRustBinding {
    pub name: String,
    pub kind: FfiRustBindingKind,
    pub source: String,
}
/// FFI pass summary
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct FfiPassSummary {
    pub pass_name: String,
    pub symbols_bridged: usize,
    pub headers_emitted: usize,
    pub rust_bindings_emitted: usize,
    pub duration_us: u64,
}
/// Calling convention for FFI functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallingConvention {
    /// Standard C calling convention.
    C,
    /// Rust calling convention (default for Rust functions).
    Rust,
    /// System calling convention (stdcall on Windows, C on other platforms).
    System,
    /// Fastcall (register-based for first N arguments).
    Fastcall,
}
/// FFI return value wrapper
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum FfiReturnWrap {
    Direct(String),
    ErrorCode(String, String),
    OutParam(String),
    Bool(String),
}
/// FFI object descriptor
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiObjectDesc {
    pub c_type: String,
    pub rust_type: String,
    pub lifetime: FfiLifetime,
    pub destroy_fn: Option<String>,
    pub clone_fn: Option<String>,
}
/// FFI extended config
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiExtConfigV2 {
    pub platform: FfiPlatformTarget,
    pub emit_c_header: bool,
    pub emit_rust_bindings: bool,
    pub emit_python_cffi: bool,
    pub emit_zig_bindings: bool,
    pub header_guard_prefix: String,
    pub lib_name: String,
    pub calling_conv_default: FfiCallingConv,
    pub enable_null_checks: bool,
    pub enable_bounds_checks: bool,
}
/// FFI feature flags
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct FfiFeatureFlags {
    pub support_varargs: bool,
    pub support_callbacks: bool,
    pub support_bitfields: bool,
    pub support_anonymous: bool,
    pub support_packed: bool,
    pub support_aligned: bool,
}
/// Heuristic freshness key for FfiExt incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FfiExtIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
impl FfiExtIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        FfiExtIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &FfiExtIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
/// A generic key-value configuration store for FfiExt.
#[derive(Debug, Clone, Default)]
pub struct FfiExtConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
}
impl FfiExtConfig {
    pub fn new() -> Self {
        FfiExtConfig::default()
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
/// FFI bridge summary v2
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct FfiBridgeSummaryV2 {
    pub lib_name: String,
    pub platform: String,
    pub funcs: usize,
    pub structs: usize,
    pub enums: usize,
    pub bytes: usize,
}
#[allow(dead_code)]
pub struct FFIConstantFoldingHelper;
impl FFIConstantFoldingHelper {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FFIAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, FFICacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl FFIAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        FFIAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&FFICacheEntry> {
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
            FFICacheEntry {
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
/// FFI struct field
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiStructField {
    pub name: String,
    pub field_type: String,
    pub bit_width: Option<u8>,
    pub offset_bytes: Option<u64>,
    pub attrs: Vec<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FFILivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl FFILivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        FFILivenessInfo {
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
/// FFI object lifetime kind
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FfiLifetime {
    Static,
    ScopedToCall,
    CallerManaged,
    LibraryManaged,
    RefCounted,
}
/// FFI struct definition
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiStructDef {
    pub name: String,
    pub fields: Vec<FfiStructField>,
    pub alignment: Option<u64>,
    pub is_packed: bool,
    pub is_union: bool,
}
/// Severity of a FfiExt diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FfiExtDiagSeverity {
    Note,
    Warning,
    Error,
}
/// FFI function signature
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiFuncSignature {
    pub name: String,
    pub params: Vec<FfiFuncParam>,
    pub ret_type: String,
    pub calling_conv: FfiCallingConv,
    pub is_async: bool,
    pub is_noexcept: bool,
    pub is_nothrow: bool,
}
/// A text buffer for building FfiExt output source code.
#[derive(Debug, Default)]
pub struct FfiExtSourceBuffer {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
impl FfiExtSourceBuffer {
    pub fn new() -> Self {
        FfiExtSourceBuffer {
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
/// FFI function parameter
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiFuncParam {
    pub name: String,
    pub ffi_type: String,
    pub attrs: Vec<FfiParamAttr>,
    pub is_vararg: bool,
}
/// Emission statistics for FfiExt.
#[derive(Debug, Clone, Default)]
pub struct FfiExtEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
impl FfiExtEmitStats {
    pub fn new() -> Self {
        FfiExtEmitStats::default()
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
/// FFI constant
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FfiConst {
    pub name: String,
    pub const_type: String,
    pub value: String,
}
