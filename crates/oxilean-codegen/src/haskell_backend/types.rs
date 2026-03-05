//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;

use super::functions::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// The various top-level declarations in a Haskell module.
#[derive(Debug, Clone, PartialEq)]
pub enum HaskellDecl {
    Data(HaskellDataDecl),
    Newtype(HaskellNewtype),
    TypeClass(HaskellTypeClass),
    Instance(HaskellInstance),
    Function(HaskellFunction),
    TypeSynonym(String, Vec<String>, HaskellType),
    Comment(String),
    RawLine(String),
}
/// A Haskell `data` declaration.
///
/// Example: `data Expr = Lit Int | Add Expr Expr deriving (Show, Eq)`
#[derive(Debug, Clone, PartialEq)]
pub struct HaskellDataDecl {
    /// Type name: `Expr`
    pub name: String,
    /// Type parameters: `a`, `b`, ...
    pub type_params: Vec<String>,
    /// Constructor list: (constructor_name, field_types)
    pub constructors: Vec<(String, Vec<HaskellType>)>,
    /// Deriving clauses: `Show`, `Eq`, `Ord`, ...
    pub deriving_clauses: Vec<String>,
}
/// A single alternative in a `case` expression.
#[derive(Debug, Clone, PartialEq)]
pub struct HaskellCaseAlt {
    pub pattern: HaskellPattern,
    /// Guards; if empty the alternative has a direct body.
    pub guards: Vec<HaskellGuard>,
    pub body: Option<HaskellExpr>,
}
/// Haskell literal values.
#[derive(Debug, Clone, PartialEq)]
pub enum HaskellLit {
    /// Integer literal: `42`, `-7`
    Int(i64),
    /// Floating-point literal: `3.14`
    Float(f64),
    /// Character literal: `'a'`
    Char(char),
    /// String literal: `"hello"`
    Str(String),
    /// Boolean literals `True` / `False`
    Bool(bool),
    /// Unit literal `()`
    Unit,
}
/// A statement inside a `do` block.
#[derive(Debug, Clone, PartialEq)]
pub enum HaskellDoStmt {
    /// `x <- action`
    Bind(String, HaskellExpr),
    /// `action` (discard result)
    Stmt(HaskellExpr),
    /// `let x = expr` inside do
    LetBind(String, HaskellExpr),
}
/// A Haskell `newtype` declaration.
///
/// Example: `newtype Name = Name { unName :: String } deriving (Show, Eq)`
#[derive(Debug, Clone, PartialEq)]
pub struct HaskellNewtype {
    /// Type name
    pub name: String,
    /// Single type parameter (or empty string)
    pub type_param: Option<String>,
    /// Constructor name
    pub constructor: String,
    /// Wrapped field name and type
    pub field: (String, HaskellType),
    /// Deriving clauses
    pub deriving_clauses: Vec<String>,
}
/// A complete Haskell source module.
#[derive(Debug, Clone, PartialEq)]
pub struct HaskellModule {
    /// Module name: `Main`, `Data.MyLib`
    pub name: String,
    /// Explicit export list (empty = export everything)
    pub exports: Vec<String>,
    /// Import declarations
    pub imports: Vec<HaskellImport>,
    /// Top-level declarations
    pub declarations: Vec<HaskellDecl>,
}
impl HaskellModule {
    /// Create a new empty module.
    pub fn new(name: impl Into<String>) -> Self {
        HaskellModule {
            name: name.into(),
            exports: Vec::new(),
            imports: Vec::new(),
            declarations: Vec::new(),
        }
    }
    /// Add an import.
    pub fn add_import(&mut self, imp: HaskellImport) {
        self.imports.push(imp);
    }
    /// Add a top-level declaration.
    pub fn add_decl(&mut self, decl: HaskellDecl) {
        self.declarations.push(decl);
    }
    /// Emit the complete Haskell source for this module.
    pub fn emit(&self) -> String {
        let mut out = String::new();
        if !self.exports.is_empty() {
            out.push_str(&format!("module {} (\n", self.name));
            for (i, exp) in self.exports.iter().enumerate() {
                if i > 0 {
                    out.push_str(",\n");
                }
                out.push_str(&format!("  {}", exp));
            }
            out.push_str("\n) where\n\n");
        } else {
            out.push_str(&format!("module {} where\n\n", self.name));
        }
        for imp in &self.imports {
            out.push_str(&format!("{}\n", imp));
        }
        if !self.imports.is_empty() {
            out.push('\n');
        }
        for decl in &self.declarations {
            out.push_str(&format!("{}\n", decl));
        }
        out
    }
}
/// Emission statistics for HsExt.
#[derive(Debug, Clone, Default)]
pub struct HsExtEmitStats {
    pub bytes_emitted: usize,
    pub items_emitted: usize,
    pub errors: usize,
    pub warnings: usize,
    pub elapsed_ms: u64,
}
impl HsExtEmitStats {
    pub fn new() -> Self {
        HsExtEmitStats::default()
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
#[derive(Debug, Clone)]
pub struct HskLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl HskLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        HskLivenessInfo {
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
pub struct HskDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl HskDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        HskDominatorTree {
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
pub struct HskWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl HskWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        HskWorklist {
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
/// A text buffer for building HsExt output source code.
#[derive(Debug, Default)]
pub struct HsExtSourceBuffer {
    pub(super) buf: String,
    pub(super) indent_level: usize,
    pub(super) indent_str: String,
}
impl HsExtSourceBuffer {
    pub fn new() -> Self {
        HsExtSourceBuffer {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HskPassConfig {
    pub phase: HskPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl HskPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: HskPassPhase) -> Self {
        HskPassConfig {
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
/// Heuristic freshness key for HsExt incremental compilation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HsExtIncrKey {
    pub content_hash: u64,
    pub config_hash: u64,
}
impl HsExtIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        HsExtIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &HsExtIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
/// A single guard in a function equation or case alternative.
#[derive(Debug, Clone, PartialEq)]
pub struct HaskellGuard {
    pub condition: HaskellExpr,
    pub body: HaskellExpr,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct HskPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl HskPassStats {
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
/// The Haskell code generation backend.
pub struct HaskellBackend {
    pub(super) module: HaskellModule,
}
impl HaskellBackend {
    /// Create a new backend targeting the named module.
    pub fn new(module_name: impl Into<String>) -> Self {
        let mut module = HaskellModule::new(module_name);
        module.add_import(HaskellImport {
            module: "Prelude".to_string(),
            qualified: false,
            alias: None,
            items: Vec::new(),
            hiding: Vec::new(),
        });
        HaskellBackend { module }
    }
    /// Compile a single LCNF function declaration into Haskell and add it to the module.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) {
        let hs_fn = self.compile_fun(decl);
        self.module.add_decl(HaskellDecl::Function(hs_fn));
    }
    /// Compile an LCNF function to a Haskell function.
    pub(super) fn compile_fun(&self, decl: &LcnfFunDecl) -> HaskellFunction {
        let params: Vec<HaskellPattern> = decl
            .params
            .iter()
            .map(|p| HaskellPattern::Var(p.name.clone()))
            .collect();
        let body = self.compile_expr(&decl.body);
        HaskellFunction {
            name: sanitize_hs_ident(&decl.name),
            type_annotation: None,
            equations: vec![HaskellEquation {
                patterns: params,
                guards: Vec::new(),
                body: Some(body),
                where_clause: Vec::new(),
            }],
        }
    }
    /// Compile an LCNF expression to a Haskell expression.
    pub(super) fn compile_expr(&self, expr: &LcnfExpr) -> HaskellExpr {
        match expr {
            LcnfExpr::Return(arg) => self.compile_arg(arg),
            LcnfExpr::Let {
                name, value, body, ..
            } => {
                let rhs_expr = self.compile_let_value(value);
                let cont_expr = self.compile_expr(body);
                HaskellExpr::Let(name.clone(), Box::new(rhs_expr), Box::new(cont_expr))
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrut = HaskellExpr::Var(format!("{}", scrutinee));
                let mut hs_alts: Vec<HaskellCaseAlt> =
                    alts.iter().map(|alt| self.compile_alt(alt)).collect();
                if let Some(def) = default {
                    let def_expr = self.compile_expr(def);
                    hs_alts.push(HaskellCaseAlt {
                        pattern: HaskellPattern::Wildcard,
                        guards: Vec::new(),
                        body: Some(def_expr),
                    });
                }
                HaskellExpr::Case(Box::new(scrut), hs_alts)
            }
            LcnfExpr::TailCall(func, args) => {
                let func_expr = self.compile_arg(func);
                if args.is_empty() {
                    func_expr
                } else {
                    let arg_exprs: Vec<HaskellExpr> =
                        args.iter().map(|a| self.compile_arg(a)).collect();
                    HaskellExpr::App(Box::new(func_expr), arg_exprs)
                }
            }
            LcnfExpr::Unreachable => HaskellExpr::Var("undefined".to_string()),
        }
    }
    /// Compile an LCNF let-value to a Haskell expression.
    pub(super) fn compile_let_value(&self, val: &LcnfLetValue) -> HaskellExpr {
        match val {
            LcnfLetValue::App(func, args) => {
                let func_expr = self.compile_arg(func);
                if args.is_empty() {
                    func_expr
                } else {
                    let arg_exprs: Vec<HaskellExpr> =
                        args.iter().map(|a| self.compile_arg(a)).collect();
                    HaskellExpr::App(Box::new(func_expr), arg_exprs)
                }
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let ctor_expr = HaskellExpr::Var(name.clone());
                if args.is_empty() {
                    ctor_expr
                } else {
                    let arg_exprs: Vec<HaskellExpr> =
                        args.iter().map(|a| self.compile_arg(a)).collect();
                    HaskellExpr::App(Box::new(ctor_expr), arg_exprs)
                }
            }
            LcnfLetValue::Proj(_name, idx, var) => {
                let accessor = match idx {
                    0 => "fst",
                    1 => "snd",
                    n => return HaskellExpr::Var(format!("_proj{}_{}", n, var)),
                };
                HaskellExpr::App(
                    Box::new(HaskellExpr::Var(accessor.to_string())),
                    vec![HaskellExpr::Var(format!("{}", var))],
                )
            }
            LcnfLetValue::Lit(lit) => match lit {
                LcnfLit::Nat(n) => HaskellExpr::Lit(HaskellLit::Int(*n as i64)),
                LcnfLit::Str(s) => HaskellExpr::Lit(HaskellLit::Str(s.clone())),
            },
            LcnfLetValue::Erased | LcnfLetValue::Reset(_) => HaskellExpr::Lit(HaskellLit::Unit),
            LcnfLetValue::FVar(v) => HaskellExpr::Var(format!("{}", v)),
            LcnfLetValue::Reuse(_, name, _tag, args) => {
                let ctor_expr = HaskellExpr::Var(name.clone());
                if args.is_empty() {
                    ctor_expr
                } else {
                    let arg_exprs: Vec<HaskellExpr> =
                        args.iter().map(|a| self.compile_arg(a)).collect();
                    HaskellExpr::App(Box::new(ctor_expr), arg_exprs)
                }
            }
        }
    }
    /// Compile an LCNF case alternative.
    pub(super) fn compile_alt(&self, alt: &LcnfAlt) -> HaskellCaseAlt {
        let body = self.compile_expr(&alt.body);
        let pat = HaskellPattern::Constructor(
            alt.ctor_name.clone(),
            alt.params
                .iter()
                .map(|p| HaskellPattern::Var(p.name.clone()))
                .collect(),
        );
        HaskellCaseAlt {
            pattern: pat,
            guards: Vec::new(),
            body: Some(body),
        }
    }
    /// Compile an LCNF argument to a Haskell expression.
    pub(super) fn compile_arg(&self, arg: &LcnfArg) -> HaskellExpr {
        match arg {
            LcnfArg::Var(v) => HaskellExpr::Var(format!("{}", v)),
            LcnfArg::Lit(lit) => match lit {
                LcnfLit::Nat(n) => HaskellExpr::Lit(HaskellLit::Int(*n as i64)),
                LcnfLit::Str(s) => HaskellExpr::Lit(HaskellLit::Str(s.clone())),
            },
            LcnfArg::Erased | LcnfArg::Type(_) => HaskellExpr::Lit(HaskellLit::Unit),
        }
    }
    /// Emit the complete Haskell module source.
    pub fn emit_module(&self) -> String {
        self.module.emit()
    }
}
/// A Haskell `instance` declaration.
///
/// Example:
/// ```text
/// instance Show Expr where
///   show (Lit n) = show n
///   show (Add l r) = show l ++ " + " ++ show r
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct HaskellInstance {
    /// Class being instantiated: `Show`
    pub class: String,
    /// Type being instantiated: `Maybe Int`
    pub instance_type: HaskellType,
    /// Context constraints: `Show a`
    pub context: Vec<HaskellType>,
    /// Method implementations as functions
    pub where_clause: Vec<HaskellFunction>,
}
/// A version tag for HsExt output artifacts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HsExtVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
    pub pre: Option<String>,
}
impl HsExtVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        HsExtVersion {
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
    pub fn is_compatible_with(&self, other: &HsExtVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
#[allow(dead_code)]
pub struct HskConstantFoldingHelper;
impl HskConstantFoldingHelper {
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
/// A feature flag set for HsExt capabilities.
#[derive(Debug, Clone, Default)]
pub struct HsExtFeatures {
    pub(super) flags: std::collections::HashSet<String>,
}
impl HsExtFeatures {
    pub fn new() -> Self {
        HsExtFeatures::default()
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
    pub fn union(&self, other: &HsExtFeatures) -> HsExtFeatures {
        HsExtFeatures {
            flags: self.flags.union(&other.flags).cloned().collect(),
        }
    }
    pub fn intersection(&self, other: &HsExtFeatures) -> HsExtFeatures {
        HsExtFeatures {
            flags: self.flags.intersection(&other.flags).cloned().collect(),
        }
    }
}
/// A qualifier in a list comprehension `[ e | q1, q2, ... ]`.
#[derive(Debug, Clone, PartialEq)]
pub enum HsListQual {
    /// Generator: `x <- xs`
    Generator(String, HaskellExpr),
    /// Guard: `x > 0`
    Guard(HaskellExpr),
    /// Let binding: `let y = f x`
    LetBind(String, HaskellExpr),
}
/// A fixed-capacity ring buffer of strings (for recent-event logging in HsExt).
#[derive(Debug)]
pub struct HsExtEventLog {
    pub(super) entries: std::collections::VecDeque<String>,
    pub(super) capacity: usize,
}
impl HsExtEventLog {
    pub fn new(capacity: usize) -> Self {
        HsExtEventLog {
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
pub struct HskPassRegistry {
    pub(super) configs: Vec<HskPassConfig>,
    pub(super) stats: std::collections::HashMap<String, HskPassStats>,
}
impl HskPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        HskPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: HskPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), HskPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&HskPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&HskPassStats> {
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
#[derive(Debug, Clone, PartialEq)]
pub enum HskPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl HskPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            HskPassPhase::Analysis => "analysis",
            HskPassPhase::Transformation => "transformation",
            HskPassPhase::Verification => "verification",
            HskPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, HskPassPhase::Transformation | HskPassPhase::Cleanup)
    }
}
/// A Haskell function definition (possibly with multiple equations).
///
/// Example:
/// ```text
/// factorial :: Int -> Int
/// factorial 0 = 1
/// factorial n = n * factorial (n - 1)
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct HaskellFunction {
    /// Function name
    pub name: String,
    /// Type signature annotation
    pub type_annotation: Option<HaskellType>,
    /// Equations (pattern-matched clauses)
    pub equations: Vec<HaskellEquation>,
}
/// Tracks declared names for HsExt scope analysis.
#[derive(Debug, Default)]
pub struct HsExtNameScope {
    pub(super) declared: std::collections::HashSet<String>,
    pub(super) depth: usize,
    pub(super) parent: Option<Box<HsExtNameScope>>,
}
impl HsExtNameScope {
    pub fn new() -> Self {
        HsExtNameScope::default()
    }
    pub fn declare(&mut self, name: impl Into<String>) -> bool {
        self.declared.insert(name.into())
    }
    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name)
    }
    pub fn push_scope(self) -> Self {
        HsExtNameScope {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct HskCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Haskell pattern AST used in `case` and function equations.
#[derive(Debug, Clone, PartialEq)]
pub enum HaskellPattern {
    /// `_` — wildcard
    Wildcard,
    /// Variable binding: `x`
    Var(String),
    /// Literal pattern: `42`, `'a'`, `True`
    Lit(HaskellLit),
    /// Tuple pattern: `(a, b, c)`
    Tuple(Vec<HaskellPattern>),
    /// List pattern: `[a, b, c]`
    List(Vec<HaskellPattern>),
    /// Cons pattern: `x : xs`
    Cons(Box<HaskellPattern>, Box<HaskellPattern>),
    /// Constructor pattern: `Just x`, `Left a`
    Constructor(String, Vec<HaskellPattern>),
    /// As pattern: `xs@(x:rest)`
    As(String, Box<HaskellPattern>),
    /// Lazy (irrefutable) pattern: `~pat`
    LazyPat(Box<HaskellPattern>),
}
/// Haskell expression AST.
#[derive(Debug, Clone, PartialEq)]
pub enum HaskellExpr {
    /// Literal value
    Lit(HaskellLit),
    /// Variable or constructor reference: `foo`, `Just`, `(:)`
    Var(String),
    /// Function application: `f x y`
    App(Box<HaskellExpr>, Vec<HaskellExpr>),
    /// Lambda: `\x y -> body`
    Lambda(Vec<HaskellPattern>, Box<HaskellExpr>),
    /// Let expression: `let x = e1 in e2`
    Let(String, Box<HaskellExpr>, Box<HaskellExpr>),
    /// Where clause as an expression wrapper
    Where(Box<HaskellExpr>, Vec<HaskellFunction>),
    /// If-then-else: `if c then t else e`
    If(Box<HaskellExpr>, Box<HaskellExpr>, Box<HaskellExpr>),
    /// Case expression
    Case(Box<HaskellExpr>, Vec<HaskellCaseAlt>),
    /// Do notation block
    Do(Vec<HaskellDoStmt>),
    /// List comprehension: `[ body | quals ]`
    ListComp(Box<HaskellExpr>, Vec<HsListQual>),
    /// Tuple: `(a, b, c)`
    Tuple(Vec<HaskellExpr>),
    /// List: `[a, b, c]`
    List(Vec<HaskellExpr>),
    /// Arithmetic negation: `negate x`
    Neg(Box<HaskellExpr>),
    /// Infix application: `x `op` y` or `x + y`
    InfixApp(Box<HaskellExpr>, String, Box<HaskellExpr>),
    /// Operator section / raw operator reference: `(+)`, `map`
    Operator(String),
    /// Type annotation: `expr :: Type`
    TypeAnnotation(Box<HaskellExpr>, HaskellType),
}
/// A Haskell `class` declaration.
///
/// Example:
/// ```text
/// class Functor f where
///   fmap :: (a -> b) -> f a -> f b
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct HaskellTypeClass {
    /// Class name: `Functor`
    pub name: String,
    /// Class type parameters: `f`
    pub type_params: Vec<String>,
    /// Superclass constraints: `Eq a`, `Ord a`
    pub superclasses: Vec<HaskellType>,
    /// Methods: (name, signature, optional default body)
    pub methods: Vec<(String, HaskellType, Option<HaskellExpr>)>,
}
/// Haskell type representation for type-directed code generation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HaskellType {
    /// `Int` — fixed-precision signed integer (at least 30-bit)
    Int,
    /// `Integer` — arbitrary-precision integer
    Integer,
    /// `Double` — 64-bit IEEE floating-point
    Double,
    /// `Float` — 32-bit IEEE floating-point
    Float,
    /// `Bool` — boolean
    Bool,
    /// `Char` — Unicode character
    Char,
    /// `String` — alias for `[Char]`
    HsString,
    /// `()` — unit type
    Unit,
    /// `IO a` — IO monad
    IO(Box<HaskellType>),
    /// `[a]` — list of a
    List(Box<HaskellType>),
    /// `Maybe a` — optional value
    Maybe(Box<HaskellType>),
    /// `Either a b` — sum type
    Either(Box<HaskellType>, Box<HaskellType>),
    /// `(a, b, ...)` — tuple
    Tuple(Vec<HaskellType>),
    /// `a -> b` — function type
    Fun(Box<HaskellType>, Box<HaskellType>),
    /// Named type (data, newtype, type synonym)
    Custom(String),
    /// Polymorphic type variable: `a`, `b`, `f`
    Polymorphic(String),
    /// Type class constraint: `Eq a`, `Functor f`
    Constraint(String, Vec<HaskellType>),
}
/// A single equation of a Haskell function definition.
///
/// Example: `factorial 0 = 1`
#[derive(Debug, Clone, PartialEq)]
pub struct HaskellEquation {
    /// Argument patterns for this equation
    pub patterns: Vec<HaskellPattern>,
    /// Optional guards; if empty, body is unconditional
    pub guards: Vec<HaskellGuard>,
    /// Right-hand side (if no guards)
    pub body: Option<HaskellExpr>,
    /// Where clause local bindings
    pub where_clause: Vec<HaskellFunction>,
}
/// A Haskell import declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct HaskellImport {
    /// Module name: `Data.List`
    pub module: String,
    /// `qualified` import
    pub qualified: bool,
    /// `as` alias
    pub alias: Option<String>,
    /// Explicit import list (empty = import everything)
    pub items: Vec<String>,
    /// `hiding` list
    pub hiding: Vec<String>,
}
/// A diagnostic message from a HsExt pass.
#[derive(Debug, Clone)]
pub struct HsExtDiagMsg {
    pub severity: HsExtDiagSeverity,
    pub pass: String,
    pub message: String,
}
impl HsExtDiagMsg {
    pub fn error(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        HsExtDiagMsg {
            severity: HsExtDiagSeverity::Error,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn warning(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        HsExtDiagMsg {
            severity: HsExtDiagSeverity::Warning,
            pass: pass.into(),
            message: msg.into(),
        }
    }
    pub fn note(pass: impl Into<String>, msg: impl Into<String>) -> Self {
        HsExtDiagMsg {
            severity: HsExtDiagSeverity::Note,
            pass: pass.into(),
            message: msg.into(),
        }
    }
}
/// Collects HsExt diagnostics.
#[derive(Debug, Default)]
pub struct HsExtDiagCollector {
    pub(super) msgs: Vec<HsExtDiagMsg>,
}
impl HsExtDiagCollector {
    pub fn new() -> Self {
        HsExtDiagCollector::default()
    }
    pub fn emit(&mut self, d: HsExtDiagMsg) {
        self.msgs.push(d);
    }
    pub fn has_errors(&self) -> bool {
        self.msgs
            .iter()
            .any(|d| d.severity == HsExtDiagSeverity::Error)
    }
    pub fn errors(&self) -> Vec<&HsExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == HsExtDiagSeverity::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&HsExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == HsExtDiagSeverity::Warning)
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
pub struct HskAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, HskCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl HskAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        HskAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&HskCacheEntry> {
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
            HskCacheEntry {
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
#[derive(Debug, Clone)]
pub struct HskDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl HskDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        HskDepGraph {
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
/// Pipeline profiler for HsExt.
#[derive(Debug, Default)]
pub struct HsExtProfiler {
    pub(super) timings: Vec<HsExtPassTiming>,
}
impl HsExtProfiler {
    pub fn new() -> Self {
        HsExtProfiler::default()
    }
    pub fn record(&mut self, t: HsExtPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&HsExtPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&HsExtPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
/// Pass-timing record for HsExt profiler.
#[derive(Debug, Clone)]
pub struct HsExtPassTiming {
    pub pass_name: String,
    pub elapsed_us: u64,
    pub items_processed: usize,
    pub bytes_before: usize,
    pub bytes_after: usize,
}
impl HsExtPassTiming {
    pub fn new(
        pass_name: impl Into<String>,
        elapsed_us: u64,
        items: usize,
        before: usize,
        after: usize,
    ) -> Self {
        HsExtPassTiming {
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
/// A monotonically increasing ID generator for HsExt.
#[derive(Debug, Default)]
pub struct HsExtIdGen {
    pub(super) next: u32,
}
impl HsExtIdGen {
    pub fn new() -> Self {
        HsExtIdGen::default()
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
/// Severity of a HsExt diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HsExtDiagSeverity {
    Note,
    Warning,
    Error,
}
/// A generic key-value configuration store for HsExt.
#[derive(Debug, Clone, Default)]
pub struct HsExtConfig {
    pub(super) entries: std::collections::HashMap<String, String>,
}
impl HsExtConfig {
    pub fn new() -> Self {
        HsExtConfig::default()
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
