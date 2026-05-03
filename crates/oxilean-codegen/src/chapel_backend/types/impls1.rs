//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls2::*;
use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

/// Intent for procedure parameters.
#[derive(Debug, Clone, PartialEq)]
pub enum ChapelIntent {
    In,
    Out,
    InOut,
    Ref,
    Const,
    ConstRef,
    Param,
    Type,
}
#[allow(dead_code)]
pub struct ChplPassRegistry {
    pub(crate) configs: Vec<ChplPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, ChplPassStats>,
}
impl ChplPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ChplPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: ChplPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), ChplPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ChplPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&ChplPassStats> {
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
/// Constant folding helper for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct ChapelExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}
impl ChapelExtConstFolder {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplPassConfig {
    pub phase: ChplPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl ChplPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: ChplPassPhase) -> Self {
        ChplPassConfig {
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
/// Chapel type representation.
#[derive(Debug, Clone, PartialEq)]
pub enum ChapelType {
    /// `int(8)` / `int(16)` / `int(32)` / `int(64)` (default `int`)
    Int(Option<u32>),
    /// `uint(8)` / `uint(16)` / `uint(32)` / `uint(64)` (default `uint`)
    UInt(Option<u32>),
    /// `real(32)` / `real(64)` (default `real`)
    Real(Option<u32>),
    /// `imag(32)` / `imag(64)` (default `imag`)
    Imag(Option<u32>),
    /// `complex(64)` / `complex(128)` (default `complex`)
    Complex(Option<u32>),
    /// `bool`
    Bool,
    /// `string`
    String,
    /// `bytes`
    Bytes,
    /// `range(idxType)` — e.g. `1..n`
    Range(Box<ChapelType>),
    /// `domain(rank, idxType)` — multi-dimensional index set
    Domain(u32, Box<ChapelType>),
    /// `[D] eltType` — array over a domain
    Array(Box<ChapelType>, Box<ChapelType>),
    /// Record type: `record R { ... }`
    Record(String),
    /// Class type: `class C { ... }`
    Class(String),
    /// Union type: `union U { ... }`
    Union(String),
    /// Enum type: `enum E { ... }`
    EnumType(String),
    /// Procedure/function type: `proc(argTypes) : retType`
    ProcType(Vec<ChapelType>, Box<ChapelType>),
    /// Tuple type: `(t1, t2, ...)`
    Tuple(Vec<ChapelType>),
    /// Named / user-defined type
    Named(String),
    /// `void` (no return)
    Void,
    /// Type variable / generic
    TypeVar(String),
    /// `atomic T`
    Atomic(Box<ChapelType>),
    /// `sync T`
    Sync(Box<ChapelType>),
    /// `single T`
    Single(Box<ChapelType>),
    /// Pointer to owned object: `owned C`
    Owned(Box<ChapelType>),
    /// Shared: `shared C`
    Shared(Box<ChapelType>),
    /// Unmanaged: `unmanaged C`
    Unmanaged(Box<ChapelType>),
}
/// Pass execution phase for ChapelExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChapelExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl ChapelExtPassPhase {
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
/// Configuration for the Chapel backend emitter.
#[derive(Debug, Clone)]
pub struct ChapelConfig {
    /// Spaces per indentation level
    pub indent_width: usize,
    /// Emit type annotations on var declarations when available
    pub annotate_vars: bool,
    /// Use `writeln` for print calls
    pub use_writeln: bool,
}
/// Chapel expression representation.
#[derive(Debug, Clone)]
pub enum ChapelExpr {
    /// Integer literal: `42`
    IntLit(i64),
    /// Real literal: `3.14`
    RealLit(f64),
    /// Bool literal: `true` / `false`
    BoolLit(bool),
    /// String literal: `"hello"`
    StrLit(String),
    /// Variable reference: `x`
    Var(String),
    /// Function/procedure application: `f(a, b, ...)`
    Apply(Box<ChapelExpr>, Vec<ChapelExpr>),
    /// Array index: `a[i]`
    Index(Box<ChapelExpr>, Box<ChapelExpr>),
    /// Field/member access: `r.field`
    FieldAccess(Box<ChapelExpr>, String),
    /// Binary operation: `lhs op rhs`
    BinOp(String, Box<ChapelExpr>, Box<ChapelExpr>),
    /// Unary operation: `op e`
    UnOp(String, Box<ChapelExpr>),
    /// Range: `lo..hi` or `lo..#n`
    RangeLit(Box<ChapelExpr>, Box<ChapelExpr>, bool),
    /// Reduce expression: `+ reduce arr`
    ReduceExpr(String, Box<ChapelExpr>),
    /// Forall expression: `[i in D] f(i)`
    ForallExpr(String, Box<ChapelExpr>, Box<ChapelExpr>),
    /// Coforall expression body reference
    CoforallExpr(String, Box<ChapelExpr>, Box<ChapelExpr>),
    /// Tuple literal: `(e1, e2, ...)`
    TupleLit(Vec<ChapelExpr>),
    /// Array literal: `[e1, e2, ...]`
    ArrayLit(Vec<ChapelExpr>),
    /// Cast: `e : t`
    Cast(Box<ChapelExpr>, ChapelType),
    /// Conditional (ternary): `if cond then t else e`
    IfExpr(Box<ChapelExpr>, Box<ChapelExpr>, Box<ChapelExpr>),
    /// New object: `new C(args...)`
    New(ChapelType, Vec<ChapelExpr>),
    /// `nil`
    Nil,
    /// `here` locale
    Here,
    /// `numLocales`
    NumLocales,
    /// `this` reference
    This,
    /// Type query: `e.type`
    TypeOf(Box<ChapelExpr>),
    /// Domain literal: `{e1, e2, ...}` or `{lo..hi}`
    DomainLit(Vec<ChapelExpr>),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplAnalysisCache {
    pub(crate) entries: std::collections::HashMap<String, ChplCacheEntry>,
    pub(crate) max_size: usize,
    pub(crate) hits: u64,
    pub(crate) misses: u64,
}
impl ChplAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        ChplAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&ChplCacheEntry> {
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
            ChplCacheEntry {
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
/// Chapel statement representation.
#[derive(Debug, Clone)]
pub enum ChapelStmt {
    /// `var name: type = expr;`
    VarDecl(String, Option<ChapelType>, Option<ChapelExpr>),
    /// `const name: type = expr;`
    ConstDecl(String, Option<ChapelType>, ChapelExpr),
    /// `name = expr;`
    Assign(ChapelExpr, ChapelExpr),
    /// Compound assign: `name op= expr;`
    CompoundAssign(String, ChapelExpr, ChapelExpr),
    /// `if cond { ... } else { ... }`
    IfElse(ChapelExpr, Vec<ChapelStmt>, Option<Vec<ChapelStmt>>),
    /// `for idx in domain { ... }`
    ForLoop(String, ChapelExpr, Vec<ChapelStmt>),
    /// `forall idx in domain { ... }`
    ForallLoop(String, ChapelExpr, Vec<ChapelStmt>),
    /// `forall idx in domain with (op reduce acc) { ... }`
    ForallReduce(String, ChapelExpr, String, String, Vec<ChapelStmt>),
    /// `coforall idx in domain { ... }`
    CoforallLoop(String, ChapelExpr, Vec<ChapelStmt>),
    /// `while cond { ... }`
    WhileLoop(ChapelExpr, Vec<ChapelStmt>),
    /// `do { ... } while cond;`
    DoWhileLoop(Vec<ChapelStmt>, ChapelExpr),
    /// `return expr;`
    ReturnStmt(Option<ChapelExpr>),
    /// Procedure definition (nested or top-level)
    ProcDef(ChapelProc),
    /// Record definition
    RecordDef(ChapelRecord),
    /// Class definition
    ClassDef(ChapelClass),
    /// Expression statement: `expr;`
    ExprStmt(ChapelExpr),
    /// `writeln(args...);`
    Writeln(Vec<ChapelExpr>),
    /// `write(args...);`
    Write(Vec<ChapelExpr>),
    /// `break;`
    Break,
    /// `continue;`
    Continue,
    /// `halt(msg);`
    Halt(String),
    /// `on locale { ... }`
    On(ChapelExpr, Vec<ChapelStmt>),
    /// `begin { ... }` — async task
    Begin(Vec<ChapelStmt>),
    /// `sync { ... }` — synchronisation block
    SyncBlock(Vec<ChapelStmt>),
    /// Block comment
    Comment(String),
    /// Blank line separator
    Blank,
}
/// Analysis cache for ChapelExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct ChapelExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl ChapelExtCache {
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
/// A Chapel record definition.
#[derive(Debug, Clone)]
pub struct ChapelRecord {
    /// Record name
    pub name: String,
    /// Fields
    pub fields: Vec<ChapelField>,
    /// Methods
    pub methods: Vec<ChapelProc>,
    /// Optional generic type parameters
    pub type_params: Vec<String>,
}
impl ChapelRecord {
    /// Create an empty record.
    pub fn new(name: impl Into<String>) -> Self {
        ChapelRecord {
            name: name.into(),
            fields: vec![],
            methods: vec![],
            type_params: vec![],
        }
    }
    /// Add a field.
    pub fn add_field(&mut self, field: ChapelField) {
        self.fields.push(field);
    }
    /// Add a method.
    pub fn add_method(&mut self, method: ChapelProc) {
        self.methods.push(method);
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ChplLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl ChplLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        ChplLivenessInfo {
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
#[derive(Debug, Clone)]
pub struct ChplDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl ChplDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        ChplDominatorTree {
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
/// A Chapel procedure parameter.
#[derive(Debug, Clone)]
pub struct ChapelParam {
    /// Parameter name
    pub name: String,
    /// Optional type annotation
    pub ty: Option<ChapelType>,
    /// Optional intent
    pub intent: Option<ChapelIntent>,
    /// Optional default value
    pub default: Option<ChapelExpr>,
}
