//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::fmt;

/// Constant folding helper for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JuliaExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl JuliaExtConstFolder {
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
/// Julia statement.
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaStmt {
    /// Expression statement: `expr`
    Expr(JuliaExpr),
    /// Local assignment: `x = expr`
    Assign(JuliaExpr, JuliaExpr),
    /// Augmented assignment: `x += expr`
    AugAssign(JuliaExpr, String, JuliaExpr),
    /// Local variable declaration: `local x::T = expr`
    Local(String, Option<JuliaType>, Option<JuliaExpr>),
    /// Global variable declaration: `global x`
    Global(String),
    /// Const declaration: `const x = expr`
    Const(String, Option<JuliaType>, JuliaExpr),
    /// Return statement: `return expr`
    Return(Option<JuliaExpr>),
    /// Break statement: `break`
    Break,
    /// Continue statement: `continue`
    Continue,
    /// If/elseif/else statement
    If {
        cond: JuliaExpr,
        then_body: Vec<JuliaStmt>,
        elseif_branches: Vec<(JuliaExpr, Vec<JuliaStmt>)>,
        else_body: Option<Vec<JuliaStmt>>,
    },
    /// For loop: `for x in iter`
    For {
        vars: Vec<String>,
        iter: JuliaExpr,
        body: Vec<JuliaStmt>,
    },
    /// While loop: `while cond`
    While {
        cond: JuliaExpr,
        body: Vec<JuliaStmt>,
    },
    /// Try/catch/finally block
    TryCatch {
        try_body: Vec<JuliaStmt>,
        catch_var: Option<String>,
        catch_body: Vec<JuliaStmt>,
        finally_body: Option<Vec<JuliaStmt>>,
    },
    /// Function definition (see JuliaFunction)
    FunctionDef(JuliaFunction),
    /// Struct definition (see JuliaStruct)
    StructDef(JuliaStruct),
    /// Abstract type definition: `abstract type Foo <: Bar end`
    AbstractTypeDef {
        name: String,
        type_params: Vec<String>,
        supertype: Option<String>,
    },
    /// Primitive type definition: `primitive type Foo 64 end`
    PrimitiveTypeDef {
        name: String,
        bits: u32,
        supertype: Option<String>,
    },
    /// Module definition
    ModuleDef(JuliaModule),
    /// Using statement: `using Module`
    Using(Vec<String>),
    /// Import statement: `import Module: sym1, sym2`
    Import(String, Vec<String>),
    /// Export statement: `export sym1, sym2`
    Export(Vec<String>),
    /// Include statement: `include("file.jl")`
    Include(String),
    /// Macro definition: `macro name(args...) body end`
    MacroDef {
        name: String,
        params: Vec<String>,
        body: Vec<JuliaStmt>,
    },
    /// Line comment: `# comment`
    Comment(String),
    /// Empty line
    Blank,
}
/// A part of an interpolated string.
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaStringPart {
    /// Literal text segment
    Text(String),
    /// Interpolated expression: `$(expr)`
    Expr(JuliaExpr),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulPassConfig {
    pub phase: JulPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl JulPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: JulPassPhase) -> Self {
        JulPassConfig {
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
/// Dominator tree for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JuliaExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl JuliaExtDomTree {
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
/// A Julia struct (composite type) definition.
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaStruct {
    /// Struct name
    pub name: String,
    /// Type parameters: `{T, S}`
    pub type_params: Vec<String>,
    /// Supertype: `<: AbstractFoo`
    pub supertype: Option<String>,
    /// Whether this struct is mutable
    pub is_mutable: bool,
    /// Fields: (name, type, optional default)
    pub fields: Vec<(String, Option<JuliaType>, Option<JuliaExpr>)>,
    /// Inner constructors
    pub inner_constructors: Vec<JuliaFunction>,
    /// Doc string
    pub doc: Option<String>,
}
impl JuliaStruct {
    /// Create a new immutable struct.
    pub fn new(name: impl Into<String>) -> Self {
        JuliaStruct {
            name: name.into(),
            type_params: vec![],
            supertype: None,
            is_mutable: false,
            fields: vec![],
            inner_constructors: vec![],
            doc: None,
        }
    }
    /// Create a new mutable struct.
    pub fn mutable(name: impl Into<String>) -> Self {
        let mut s = JuliaStruct::new(name);
        s.is_mutable = true;
        s
    }
    /// Add a field.
    pub fn with_field(mut self, name: impl Into<String>, ty: JuliaType) -> Self {
        self.fields.push((name.into(), Some(ty), None));
        self
    }
    /// Set supertype.
    pub fn with_supertype(mut self, supertype: impl Into<String>) -> Self {
        self.supertype = Some(supertype.into());
        self
    }
    /// Add a type parameter.
    pub fn with_type_param(mut self, param: impl Into<String>) -> Self {
        self.type_params.push(param.into());
        self
    }
}
/// Julia parameter in function signatures.
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaParam {
    /// Parameter name
    pub name: String,
    /// Optional type annotation
    pub ty: Option<JuliaType>,
    /// Optional default value
    pub default: Option<JuliaExpr>,
    /// Whether this is a keyword parameter
    pub is_keyword: bool,
    /// Whether this is a splat parameter (`args...`)
    pub is_splat: bool,
}
impl JuliaParam {
    /// Create a simple positional parameter with no type annotation.
    pub fn simple(name: impl Into<String>) -> Self {
        JuliaParam {
            name: name.into(),
            ty: None,
            default: None,
            is_keyword: false,
            is_splat: false,
        }
    }
    /// Create a typed positional parameter.
    pub fn typed(name: impl Into<String>, ty: JuliaType) -> Self {
        JuliaParam {
            name: name.into(),
            ty: Some(ty),
            default: None,
            is_keyword: false,
            is_splat: false,
        }
    }
    /// Create a keyword parameter with a default value.
    pub fn keyword(name: impl Into<String>, default: JuliaExpr) -> Self {
        JuliaParam {
            name: name.into(),
            ty: None,
            default: Some(default),
            is_keyword: true,
            is_splat: false,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JulPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl JulPassStats {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, JulCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl JulAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        JulAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&JulCacheEntry> {
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
            JulCacheEntry {
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
/// Configuration for JuliaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JuliaExtPassConfig {
    pub name: String,
    pub phase: JuliaExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl JuliaExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: JuliaExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: JuliaExtPassPhase) -> Self {
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
/// Newtype wrapper for Display on JuliaExpr (avoids orphan impl).
pub struct JuliaExprDisplay<'a>(pub(super) &'a JuliaExpr);
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl JulDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        JulDominatorTree {
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
/// Liveness analysis for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JuliaExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl JuliaExtLiveness {
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
/// A dispatch table for multiple dispatch — groups method variants of a function.
#[derive(Debug, Clone)]
pub struct DispatchTable {
    /// Function name shared by all methods
    pub name: String,
    /// Method specializations, ordered by specificity (most specific first)
    pub methods: Vec<JuliaFunction>,
}
impl DispatchTable {
    /// Create a new dispatch table for a function name.
    pub fn new(name: impl Into<String>) -> Self {
        DispatchTable {
            name: name.into(),
            methods: vec![],
        }
    }
    /// Add a method specialization.
    pub fn add_method(&mut self, method: JuliaFunction) {
        self.methods.push(method);
    }
    /// Return the number of registered methods.
    pub fn num_methods(&self) -> usize {
        self.methods.len()
    }
    /// Find the most specific applicable method for given argument types.
    /// This is a simplified linear scan (real Julia does lattice-based dispatch).
    pub fn find_method(&self, arg_types: &[JuliaType]) -> Option<&JuliaFunction> {
        for method in &self.methods {
            if method.params.len() == arg_types.len() {
                let matches = method
                    .params
                    .iter()
                    .zip(arg_types.iter())
                    .all(|(p, t)| p.ty.as_ref().is_none_or(|pt| pt == t));
                if matches {
                    return Some(method);
                }
            }
        }
        self.methods
            .iter()
            .find(|m| m.params.iter().all(|p| p.ty.is_none()))
    }
}
/// Analysis cache for JuliaExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct JuliaExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl JuliaExtCache {
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
/// Julia code generation backend.
pub struct JuliaBackend {
    /// Indentation level
    pub(super) indent: usize,
    /// Output buffer
    pub(super) output: String,
    /// Registered dispatch tables (function name → dispatch table)
    pub(super) dispatch_tables: HashMap<String, DispatchTable>,
}
impl JuliaBackend {
    /// Create a new Julia backend.
    pub fn new() -> Self {
        JuliaBackend {
            indent: 0,
            output: String::new(),
            dispatch_tables: HashMap::new(),
        }
    }
    /// Return the current indentation string.
    pub(super) fn indent_str(&self) -> String {
        "    ".repeat(self.indent)
    }
    /// Push a line to the output.
    pub(super) fn push_line(&mut self, line: &str) {
        let indent = self.indent_str();
        self.output.push_str(&indent);
        self.output.push_str(line);
        self.output.push('\n');
    }
    /// Push an empty line.
    pub(super) fn push_blank(&mut self) {
        self.output.push('\n');
    }
    /// Register a method into the dispatch table for its function name.
    pub fn register_method(&mut self, func: JuliaFunction) {
        let table = self
            .dispatch_tables
            .entry(func.name.clone())
            .or_insert_with(|| DispatchTable::new(func.name.clone()));
        table.add_method(func);
    }
    /// Emit a Julia expression to a String.
    pub fn emit_expr(&self, expr: &JuliaExpr) -> String {
        let mut s = String::new();
        struct FmtStr<'a>(&'a mut String);
        impl<'a> fmt::Write for FmtStr<'a> {
            fn write_str(&mut self, s: &str) -> fmt::Result {
                self.0.push_str(s);
                Ok(())
            }
        }
        use std::fmt::Write as FmtWrite;
        let _ = write!(FmtStr(&mut s), "{}", JuliaExprDisplay(expr));
        s
    }
    /// Emit a Julia type to a String.
    pub fn emit_type(&self, ty: &JuliaType) -> String {
        ty.to_string()
    }
    /// Emit a Julia function definition to the output buffer.
    pub fn emit_function(&mut self, func: &JuliaFunction) {
        if let Some(ref doc) = func.doc {
            self.push_line("\"\"\"");
            for line in doc.lines() {
                self.push_line(line);
            }
            self.push_line("\"\"\"");
        }
        let sig = func.emit_signature();
        self.push_line(&sig);
        self.indent += 1;
        for stmt in &func.body {
            self.emit_stmt(stmt);
        }
        self.indent -= 1;
        self.push_line("end");
    }
    /// Emit a Julia struct definition to the output buffer.
    pub fn emit_struct(&mut self, s: &JuliaStruct) {
        if let Some(ref doc) = s.doc {
            self.push_line("\"\"\"");
            for line in doc.lines() {
                self.push_line(line);
            }
            self.push_line("\"\"\"");
        }
        let kw = if s.is_mutable {
            "mutable struct"
        } else {
            "struct"
        };
        let mut header = format!("{} {}", kw, s.name);
        if !s.type_params.is_empty() {
            header.push('{');
            header.push_str(&s.type_params.join(", "));
            header.push('}');
        }
        if let Some(ref sup) = s.supertype {
            header.push_str(&format!(" <: {}", sup));
        }
        self.push_line(&header);
        self.indent += 1;
        for (name, ty, default) in &s.fields {
            let mut field_str = name.clone();
            if let Some(ref t) = ty {
                field_str.push_str(&format!("::{}", t));
            }
            if let Some(ref d) = default {
                field_str.push_str(&format!(" = {}", self.emit_expr(d)));
            }
            self.push_line(&field_str);
        }
        for ctor in &s.inner_constructors {
            self.emit_function(ctor);
        }
        self.indent -= 1;
        self.push_line("end");
    }
    /// Emit a Julia module definition to the output buffer.
    pub fn emit_module(&mut self, m: &JuliaModule) {
        let kw = if m.is_bare { "baremodule" } else { "module" };
        self.push_line(&format!("{} {}", kw, m.name));
        self.push_blank();
        self.indent += 1;
        for mods in &m.usings {
            self.push_line(&format!("using {}", mods.join(", ")));
        }
        for (module, syms) in &m.imports {
            if syms.is_empty() {
                self.push_line(&format!("import {}", module));
            } else {
                self.push_line(&format!("import {}: {}", module, syms.join(", ")));
            }
        }
        if !m.exports.is_empty() {
            self.push_line(&format!("export {}", m.exports.join(", ")));
        }
        if !m.usings.is_empty() || !m.imports.is_empty() || !m.exports.is_empty() {
            self.push_blank();
        }
        for stmt in &m.body {
            self.emit_stmt(stmt);
        }
        self.indent -= 1;
        self.push_line("end");
    }
    /// Emit a Julia statement to the output buffer.
    pub fn emit_stmt(&mut self, stmt: &JuliaStmt) {
        match stmt {
            JuliaStmt::Expr(e) => {
                let s = self.emit_expr(e);
                self.push_line(&s);
            }
            JuliaStmt::Assign(lhs, rhs) => {
                let l = self.emit_expr(lhs);
                let r = self.emit_expr(rhs);
                self.push_line(&format!("{} = {}", l, r));
            }
            JuliaStmt::AugAssign(lhs, op, rhs) => {
                let l = self.emit_expr(lhs);
                let r = self.emit_expr(rhs);
                self.push_line(&format!("{} {}= {}", l, op, r));
            }
            JuliaStmt::Local(name, ty, init) => {
                let mut s = format!("local {}", name);
                if let Some(ref t) = ty {
                    s.push_str(&format!("::{}", t));
                }
                if let Some(ref e) = init {
                    s.push_str(&format!(" = {}", self.emit_expr(e)));
                }
                self.push_line(&s);
            }
            JuliaStmt::Global(name) => {
                self.push_line(&format!("global {}", name));
            }
            JuliaStmt::Const(name, ty, val) => {
                let mut s = format!("const {}", name);
                if let Some(ref t) = ty {
                    s.push_str(&format!("::{}", t));
                }
                s.push_str(&format!(" = {}", self.emit_expr(val)));
                self.push_line(&s);
            }
            JuliaStmt::Return(Some(e)) => {
                let s = self.emit_expr(e);
                self.push_line(&format!("return {}", s));
            }
            JuliaStmt::Return(None) => {
                self.push_line("return");
            }
            JuliaStmt::Break => self.push_line("break"),
            JuliaStmt::Continue => self.push_line("continue"),
            JuliaStmt::If {
                cond,
                then_body,
                elseif_branches,
                else_body,
            } => {
                let c = self.emit_expr(cond);
                self.push_line(&format!("if {}", c));
                self.indent += 1;
                for s in then_body {
                    self.emit_stmt(s);
                }
                self.indent -= 1;
                for (econd, ebody) in elseif_branches {
                    let ec = self.emit_expr(econd);
                    self.push_line(&format!("elseif {}", ec));
                    self.indent += 1;
                    for s in ebody {
                        self.emit_stmt(s);
                    }
                    self.indent -= 1;
                }
                if let Some(ref eb) = else_body {
                    self.push_line("else");
                    self.indent += 1;
                    for s in eb {
                        self.emit_stmt(s);
                    }
                    self.indent -= 1;
                }
                self.push_line("end");
            }
            JuliaStmt::For { vars, iter, body } => {
                let iter_s = self.emit_expr(iter);
                self.push_line(&format!("for {} in {}", vars.join(", "), iter_s));
                self.indent += 1;
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent -= 1;
                self.push_line("end");
            }
            JuliaStmt::While { cond, body } => {
                let c = self.emit_expr(cond);
                self.push_line(&format!("while {}", c));
                self.indent += 1;
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent -= 1;
                self.push_line("end");
            }
            JuliaStmt::TryCatch {
                try_body,
                catch_var,
                catch_body,
                finally_body,
            } => {
                self.push_line("try");
                self.indent += 1;
                for s in try_body {
                    self.emit_stmt(s);
                }
                self.indent -= 1;
                if let Some(ref cv) = catch_var {
                    self.push_line(&format!("catch {}", cv));
                } else {
                    self.push_line("catch");
                }
                self.indent += 1;
                for s in catch_body {
                    self.emit_stmt(s);
                }
                self.indent -= 1;
                if let Some(ref fb) = finally_body {
                    self.push_line("finally");
                    self.indent += 1;
                    for s in fb {
                        self.emit_stmt(s);
                    }
                    self.indent -= 1;
                }
                self.push_line("end");
            }
            JuliaStmt::FunctionDef(f) => {
                self.push_blank();
                self.emit_function(f);
                self.push_blank();
            }
            JuliaStmt::StructDef(s) => {
                self.push_blank();
                self.emit_struct(s);
                self.push_blank();
            }
            JuliaStmt::AbstractTypeDef {
                name,
                type_params,
                supertype,
            } => {
                let mut s = format!("abstract type {}", name);
                if !type_params.is_empty() {
                    s.push('{');
                    s.push_str(&type_params.join(", "));
                    s.push('}');
                }
                if let Some(ref sup) = supertype {
                    s.push_str(&format!(" <: {}", sup));
                }
                s.push_str(" end");
                self.push_line(&s);
            }
            JuliaStmt::PrimitiveTypeDef {
                name,
                bits,
                supertype,
            } => {
                let mut s = format!("primitive type {} {}", name, bits);
                if let Some(ref sup) = supertype {
                    s.push_str(&format!(" <: {}", sup));
                }
                s.push_str(" end");
                self.push_line(&s);
            }
            JuliaStmt::ModuleDef(m) => {
                self.push_blank();
                self.emit_module(m);
                self.push_blank();
            }
            JuliaStmt::Using(mods) => {
                self.push_line(&format!("using {}", mods.join(", ")));
            }
            JuliaStmt::Import(module, syms) => {
                if syms.is_empty() {
                    self.push_line(&format!("import {}", module));
                } else {
                    self.push_line(&format!("import {}: {}", module, syms.join(", ")));
                }
            }
            JuliaStmt::Export(syms) => {
                self.push_line(&format!("export {}", syms.join(", ")));
            }
            JuliaStmt::Include(path) => {
                self.push_line(&format!("include(\"{}\")", path));
            }
            JuliaStmt::MacroDef { name, params, body } => {
                self.push_line(&format!("macro {}({})", name, params.join(", ")));
                self.indent += 1;
                for s in body {
                    self.emit_stmt(s);
                }
                self.indent -= 1;
                self.push_line("end");
            }
            JuliaStmt::Comment(s) => {
                self.push_line(&format!("# {}", s));
            }
            JuliaStmt::Blank => {
                self.push_blank();
            }
        }
    }
    /// Emit all registered dispatch table methods (for multiple dispatch).
    pub fn emit_dispatch_tables(&mut self) {
        let names: Vec<String> = self.dispatch_tables.keys().cloned().collect();
        for name in names {
            let methods: Vec<JuliaFunction> = self.dispatch_tables[&name].methods.clone();
            self.push_line(&format!(
                "# Multiple dispatch: {} methods for '{}'",
                methods.len(),
                name
            ));
            for method in methods {
                self.emit_function(&method);
                self.push_blank();
            }
        }
    }
    /// Take the output buffer and return it.
    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.output)
    }
    /// Get a reference to the output buffer.
    pub fn output(&self) -> &str {
        &self.output
    }
}
/// A Julia module definition.
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaModule {
    /// Module name
    pub name: String,
    /// Whether this is a bare module (no automatic includes)
    pub is_bare: bool,
    /// Using statements
    pub usings: Vec<Vec<String>>,
    /// Import statements: (module, symbols)
    pub imports: Vec<(String, Vec<String>)>,
    /// Export list
    pub exports: Vec<String>,
    /// Module body (functions, structs, constants, etc.)
    pub body: Vec<JuliaStmt>,
}
impl JuliaModule {
    /// Create a new module.
    pub fn new(name: impl Into<String>) -> Self {
        JuliaModule {
            name: name.into(),
            is_bare: false,
            usings: vec![],
            imports: vec![],
            exports: vec![],
            body: vec![],
        }
    }
    /// Add an export symbol.
    pub fn export(mut self, sym: impl Into<String>) -> Self {
        self.exports.push(sym.into());
        self
    }
    /// Add a using statement.
    pub fn using(mut self, modules: Vec<String>) -> Self {
        self.usings.push(modules);
        self
    }
    /// Add a statement to the module body.
    pub fn push(mut self, stmt: JuliaStmt) -> Self {
        self.body.push(stmt);
        self
    }
}
/// Dependency graph for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JuliaExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl JuliaExtDepGraph {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl JulWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JulWorklist {
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
#[derive(Debug, Clone, PartialEq)]
pub enum JulPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl JulPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            JulPassPhase::Analysis => "analysis",
            JulPassPhase::Transformation => "transformation",
            JulPassPhase::Verification => "verification",
            JulPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, JulPassPhase::Transformation | JulPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl JulLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        JulLivenessInfo {
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
#[allow(dead_code)]
pub struct JulConstantFoldingHelper;
impl JulConstantFoldingHelper {
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
pub struct JuliaStmtDisplay<'a>(pub(super) &'a JuliaStmt);
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Pass registry for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct JuliaExtPassRegistry {
    pub(super) configs: Vec<JuliaExtPassConfig>,
    pub(super) stats: Vec<JuliaExtPassStats>,
}
impl JuliaExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: JuliaExtPassConfig) {
        self.stats.push(JuliaExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&JuliaExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&JuliaExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&JuliaExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &JuliaExtPassPhase) -> Vec<&JuliaExtPassConfig> {
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
#[allow(dead_code)]
pub struct JulPassRegistry {
    pub(super) configs: Vec<JulPassConfig>,
    pub(super) stats: std::collections::HashMap<String, JulPassStats>,
}
impl JulPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JulPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: JulPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), JulPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&JulPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&JulPassStats> {
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
/// Pass execution phase for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JuliaExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl JuliaExtPassPhase {
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
/// Julia type representation.
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaType {
    /// `Int8`
    Int8,
    /// `Int16`
    Int16,
    /// `Int32`
    Int32,
    /// `Int64`
    Int64,
    /// `Int128`
    Int128,
    /// `UInt8`
    UInt8,
    /// `UInt16`
    UInt16,
    /// `UInt32`
    UInt32,
    /// `UInt64`
    UInt64,
    /// `UInt128`
    UInt128,
    /// `Float32`
    Float32,
    /// `Float64`
    Float64,
    /// `Bool`
    Bool,
    /// `String`
    String,
    /// `Char`
    Char,
    /// `Nothing`
    Nothing,
    /// `Any`
    Any,
    /// `Vector{T}` — 1-D array
    Vector(Box<JuliaType>),
    /// `Matrix{T}` — 2-D array
    Matrix(Box<JuliaType>),
    /// `Array{T, N}` — N-dimensional array
    Array(Box<JuliaType>, u32),
    /// `Tuple{T1, T2, ...}`
    Tuple(Vec<JuliaType>),
    /// `NamedTuple{names, types}`
    NamedTuple(Vec<(String, JuliaType)>),
    /// `Union{T1, T2, ...}`
    Union(Vec<JuliaType>),
    /// Abstract type: `AbstractType`
    Abstract(String),
    /// Parametric type: `Type{T1, T2}`
    Parametric(String, Vec<JuliaType>),
    /// Type variable: `T` (used in parametric definitions)
    TypeVar(String),
    /// Function type (callable): `Function`
    Function,
    /// `Dict{K, V}`
    Dict(Box<JuliaType>, Box<JuliaType>),
    /// `Set{T}`
    Set(Box<JuliaType>),
    /// `Ref{T}` — mutable reference
    Ref(Box<JuliaType>),
    /// Named (user-defined) type: `MyStruct`
    Named(String),
}
/// A Julia function definition with multiple dispatch support.
#[derive(Debug, Clone, PartialEq)]
pub struct JuliaFunction {
    /// Function name
    pub name: String,
    /// Type parameters for parametric methods: `{T, S}`
    pub type_params: Vec<String>,
    /// Type parameter bounds: `T <: Number`
    pub type_param_bounds: Vec<(String, String)>,
    /// Positional parameters
    pub params: Vec<JuliaParam>,
    /// Keyword-only parameters (after `;`)
    pub kwargs: Vec<JuliaParam>,
    /// Return type annotation
    pub return_type: Option<JuliaType>,
    /// Function body
    pub body: Vec<JuliaStmt>,
    /// Whether this is an inner (anonymous) function
    pub is_inner: bool,
    /// Doc string
    pub doc: Option<String>,
}
impl JuliaFunction {
    /// Create a new named function.
    pub fn new(name: impl Into<String>) -> Self {
        JuliaFunction {
            name: name.into(),
            type_params: vec![],
            type_param_bounds: vec![],
            params: vec![],
            kwargs: vec![],
            return_type: None,
            body: vec![],
            is_inner: false,
            doc: None,
        }
    }
    /// Add a positional parameter.
    pub fn with_param(mut self, param: JuliaParam) -> Self {
        self.params.push(param);
        self
    }
    /// Set the return type.
    pub fn with_return_type(mut self, ty: JuliaType) -> Self {
        self.return_type = Some(ty);
        self
    }
    /// Add body statements.
    pub fn with_body(mut self, body: Vec<JuliaStmt>) -> Self {
        self.body = body;
        self
    }
    /// Add a type parameter (for multiple dispatch).
    pub fn with_type_param(mut self, param: impl Into<String>) -> Self {
        self.type_params.push(param.into());
        self
    }
    /// Add a type parameter with bound.
    pub fn with_type_param_bound(
        mut self,
        param: impl Into<String>,
        bound: impl Into<String>,
    ) -> Self {
        let p = param.into();
        self.type_params.push(p.clone());
        self.type_param_bounds.push((p, bound.into()));
        self
    }
    /// Emit function signature string.
    pub fn emit_signature(&self) -> String {
        let mut s = String::new();
        s.push_str("function ");
        s.push_str(&self.name);
        if !self.type_params.is_empty() {
            s.push('{');
            for (i, tp) in self.type_params.iter().enumerate() {
                if i > 0 {
                    s.push_str(", ");
                }
                let bound = self.type_param_bounds.iter().find(|(n, _)| n == tp);
                if let Some((_, b)) = bound {
                    s.push_str(&format!("{} <: {}", tp, b));
                } else {
                    s.push_str(tp);
                }
            }
            s.push('}');
        }
        s.push('(');
        for (i, p) in self.params.iter().enumerate() {
            if i > 0 {
                s.push_str(", ");
            }
            s.push_str(&p.to_string());
        }
        if !self.kwargs.is_empty() {
            if !self.params.is_empty() {
                s.push_str("; ");
            } else {
                s.push(';');
            }
            for (i, kw) in self.kwargs.iter().enumerate() {
                if i > 0 {
                    s.push_str(", ");
                }
                s.push_str(&kw.to_string());
            }
        }
        s.push(')');
        if let Some(ref rt) = self.return_type {
            s.push_str(&format!("::{}", rt));
        }
        s
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JulDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl JulDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        JulDepGraph {
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
/// Statistics for JuliaExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct JuliaExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl JuliaExtPassStats {
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
    pub fn merge(&mut self, o: &JuliaExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Worklist for JuliaExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JuliaExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl JuliaExtWorklist {
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
/// Julia expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum JuliaExpr {
    /// Integer literal: `42`
    IntLit(i64),
    /// Unsigned integer literal: `0x2a`
    UIntLit(u64),
    /// Float literal: `3.14`
    FloatLit(f64),
    /// Boolean literal: `true` / `false`
    BoolLit(bool),
    /// String literal: `"hello"`
    StringLit(String),
    /// Char literal: `'a'`
    CharLit(char),
    /// Nothing literal: `nothing`
    Nothing,
    /// Variable reference: `x`
    Var(String),
    /// Field access: `obj.field`
    Field(Box<JuliaExpr>, String),
    /// Index access: `arr[i]`
    Index(Box<JuliaExpr>, Vec<JuliaExpr>),
    /// Slice: `arr[begin:end]`
    Slice(
        Box<JuliaExpr>,
        Option<Box<JuliaExpr>>,
        Option<Box<JuliaExpr>>,
    ),
    /// Function call: `f(args...)`
    Call(Box<JuliaExpr>, Vec<JuliaExpr>),
    /// Keyword arguments call: `f(a; key=val, ...)`
    CallKw(Box<JuliaExpr>, Vec<JuliaExpr>, Vec<(String, JuliaExpr)>),
    /// Broadcasting call: `f.(args...)`
    Broadcast(Box<JuliaExpr>, Vec<JuliaExpr>),
    /// Binary operation: `a + b`
    BinOp(String, Box<JuliaExpr>, Box<JuliaExpr>),
    /// Unary operation: `-x`
    UnOp(String, Box<JuliaExpr>),
    /// Comparison chain: `a < b <= c`
    CompareChain(Vec<JuliaExpr>, Vec<String>),
    /// Array literal: `[1, 2, 3]`
    ArrayLit(Vec<JuliaExpr>),
    /// Matrix literal (rows separated by semicolons): `[1 2; 3 4]`
    MatrixLit(Vec<Vec<JuliaExpr>>),
    /// Range: `1:10` or `1:2:10`
    Range(Box<JuliaExpr>, Option<Box<JuliaExpr>>, Box<JuliaExpr>),
    /// Tuple: `(a, b, c)`
    TupleLit(Vec<JuliaExpr>),
    /// Array comprehension: `[f(x) for x in xs]`
    ArrayComp(
        Box<JuliaExpr>,
        Vec<(String, JuliaExpr)>,
        Option<Box<JuliaExpr>>,
    ),
    /// Generator expression: `(f(x) for x in xs)`
    Generator(
        Box<JuliaExpr>,
        Vec<(String, JuliaExpr)>,
        Option<Box<JuliaExpr>>,
    ),
    /// Dict comprehension: `Dict(k => v for (k,v) in pairs)`
    DictComp(Box<JuliaExpr>, Box<JuliaExpr>, Vec<(String, JuliaExpr)>),
    /// Anonymous function: `x -> x + 1`
    Lambda(Vec<JuliaParam>, Box<JuliaExpr>),
    /// Short anonymous function with `do` block is represented as Lambda
    DoBlock(Box<JuliaExpr>, Vec<String>, Vec<JuliaStmt>),
    /// Ternary: `cond ? then : else`
    Ternary(Box<JuliaExpr>, Box<JuliaExpr>, Box<JuliaExpr>),
    /// Type assertion: `x::T`
    TypeAssert(Box<JuliaExpr>, JuliaType),
    /// Type conversion: `convert(T, x)`
    Convert(JuliaType, Box<JuliaExpr>),
    /// `isa` check: `x isa T`
    IsA(Box<JuliaExpr>, JuliaType),
    /// `typeof` call: `typeof(x)`
    TypeOf(Box<JuliaExpr>),
    /// Macro call: `@macro args...`
    Macro(String, Vec<JuliaExpr>),
    /// Interpolated string: `"text $(expr) more"`
    Interpolated(Vec<JuliaStringPart>),
    /// Splat: `args...`
    Splat(Box<JuliaExpr>),
    /// Named argument pair: `key = value`
    NamedArg(String, Box<JuliaExpr>),
    /// Pair (for Dict): `k => v`
    Pair(Box<JuliaExpr>, Box<JuliaExpr>),
    /// Block expression: `begin ... end`
    Block(Vec<JuliaStmt>),
}
