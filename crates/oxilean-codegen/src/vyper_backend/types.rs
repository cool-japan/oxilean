//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use std::collections::HashMap;

use super::functions::VYPER_RUNTIME;

use std::collections::{HashSet, VecDeque};

/// A Vyper flag (bitset enum, Vyper 0.3.8+).
#[derive(Debug, Clone)]
pub struct VyperFlagDef {
    pub name: String,
    pub variants: Vec<String>,
    pub doc: Option<String>,
}
/// Analysis cache for VyperExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct VyperExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl VyperExtCache {
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
/// Liveness analysis for VyperExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct VyperExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl VyperExtLiveness {
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
/// A Vyper function definition.
#[derive(Debug, Clone)]
pub struct VyperFunction {
    pub name: String,
    pub decorators: Vec<VyperDecorator>,
    pub params: Vec<VyperParam>,
    pub return_ty: Option<VyperType>,
    pub body: Vec<VyperStmt>,
    pub doc: Option<String>,
}
impl VyperFunction {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            decorators: Vec::new(),
            params: Vec::new(),
            return_ty: None,
            body: Vec::new(),
            doc: None,
        }
    }
    pub fn external(mut self) -> Self {
        self.decorators.push(VyperDecorator::External);
        self
    }
    pub fn internal(mut self) -> Self {
        self.decorators.push(VyperDecorator::Internal);
        self
    }
    pub fn view(mut self) -> Self {
        self.decorators.push(VyperDecorator::View);
        self
    }
    pub fn pure_fn(mut self) -> Self {
        self.decorators.push(VyperDecorator::Pure);
        self
    }
    pub fn payable(mut self) -> Self {
        self.decorators.push(VyperDecorator::Payable);
        self
    }
    pub fn nonreentrant(mut self, key: impl Into<String>) -> Self {
        self.decorators
            .push(VyperDecorator::NonReentrant(key.into()));
        self
    }
    /// ABI signature for selector computation.
    pub fn abi_signature(&self) -> String {
        let params: Vec<String> = self.params.iter().map(|p| p.ty.abi_canonical()).collect();
        format!("{}({})", self.name, params.join(","))
    }
    /// Simple 4-byte selector (djb2-based placeholder).
    pub fn selector(&self) -> [u8; 4] {
        let sig = self.abi_signature();
        let mut h: u32 = 5381;
        for b in sig.bytes() {
            h = h.wrapping_shl(5).wrapping_add(h).wrapping_add(b as u32);
        }
        h.to_be_bytes()
    }
    /// Returns true if this function is `@external`.
    pub fn is_external(&self) -> bool {
        self.decorators.contains(&VyperDecorator::External)
    }
    /// Returns true if this function is `@view` or `@pure`.
    pub fn is_read_only(&self) -> bool {
        self.decorators.contains(&VyperDecorator::View)
            || self.decorators.contains(&VyperDecorator::Pure)
    }
}
/// Dependency graph for VyperExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VyperExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl VyperExtDepGraph {
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
/// Pass execution phase for VyperExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VyperExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl VyperExtPassPhase {
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
/// A Vyper state variable (storage variable).
#[derive(Debug, Clone)]
pub struct VyperStorageVar {
    pub name: String,
    pub ty: VyperType,
    pub is_public: bool,
    pub doc: Option<String>,
}
/// The main Vyper code generation backend.
#[derive(Debug, Default)]
pub struct VyperBackend {
    /// Emitted contract (only one per Vyper file).
    pub contract: Option<VyperContract>,
    /// Compilation context.
    pub ctx: VyperCompilationCtx,
    /// Type alias table.
    pub type_aliases: HashMap<String, VyperType>,
    /// Source buffer accumulated during emission.
    pub source: String,
}
impl VyperBackend {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_runtime(mut self) -> Self {
        self.ctx.include_runtime = true;
        self
    }
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.ctx.version = version.into();
        self
    }
    pub fn set_contract(&mut self, contract: VyperContract) {
        self.contract = Some(contract);
    }
    /// Compile a single LCNF-style declaration into a Vyper storage variable.
    pub fn compile_decl(&self, name: &str, ty: VyperType) -> VyperStorageVar {
        VyperStorageVar {
            name: name.into(),
            ty,
            is_public: false,
            doc: None,
        }
    }
    /// Emit the full Vyper source for the registered contract.
    pub fn emit_contract(&mut self) -> String {
        let mut out = String::new();
        out.push_str(&format!("# @version {}\n", self.ctx.version));
        if self.ctx.abi_v2 {
            out.push_str("# pragma abicoder v2\n");
        }
        for p in &self.ctx.extra_pragmas {
            out.push_str(&format!("# pragma {}\n", p));
        }
        out.push('\n');
        if self.ctx.include_runtime {
            out.push_str(VYPER_RUNTIME);
            out.push('\n');
        }
        if let Some(contract) = &self.contract.clone() {
            if let Some(doc) = &contract.doc {
                out.push_str(&format!("\"\"\"\n@title {}\n\"\"\"\n\n", doc));
            }
            for iface in &contract.interfaces {
                out.push_str(&Self::emit_interface(iface));
                out.push('\n');
            }
            for s in &contract.structs {
                out.push_str(&Self::emit_struct(s));
                out.push('\n');
            }
            for fl in &contract.flags {
                out.push_str(&Self::emit_flag(fl));
                out.push('\n');
            }
            for ev in &contract.events {
                out.push_str(&Self::emit_event(ev));
                out.push('\n');
            }
            for c in &contract.constants {
                out.push_str(&Self::emit_constant(c));
            }
            if !contract.constants.is_empty() {
                out.push('\n');
            }
            for sv in &contract.storage {
                out.push_str(&Self::emit_storage_var(sv));
            }
            if !contract.storage.is_empty() {
                out.push('\n');
            }
            if let Some(ctor) = &contract.constructor.clone() {
                out.push_str(&Self::emit_function(ctor, true));
                out.push('\n');
            }
            if let Some(dflt) = &contract.default_fn.clone() {
                out.push_str(&Self::emit_function(dflt, false));
                out.push('\n');
            }
            for func in &contract.functions.clone() {
                out.push_str(&Self::emit_function(func, false));
                out.push('\n');
            }
        }
        self.source = out.clone();
        out
    }
    pub(super) fn emit_interface(iface: &VyperInterface) -> String {
        let mut out = String::new();
        if let Some(doc) = &iface.doc {
            out.push_str(&format!("# {}\n", doc));
        }
        out.push_str(&format!("interface {}:\n", iface.name));
        if iface.functions.is_empty() {
            out.push_str("    pass\n");
        } else {
            for func in &iface.functions {
                for dec in &func.decorators {
                    out.push_str(&format!("    {}\n", dec));
                }
                let params: Vec<String> = func.params.iter().map(|p| p.to_string()).collect();
                let ret = func
                    .return_ty
                    .as_ref()
                    .map(|t| format!(" -> {}", t))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "    def {}({}){}:\n",
                    func.name,
                    params.join(", "),
                    ret
                ));
                out.push_str("        ...\n");
            }
        }
        out
    }
    pub(super) fn emit_struct(s: &VyperStruct) -> String {
        let mut out = String::new();
        if let Some(doc) = &s.doc {
            out.push_str(&format!("# {}\n", doc));
        }
        out.push_str(&format!("struct {}:\n", s.name));
        if s.fields.is_empty() {
            out.push_str("    pass\n");
        } else {
            for (name, ty) in &s.fields {
                out.push_str(&format!("    {}: {}\n", name, ty));
            }
        }
        out
    }
    pub(super) fn emit_flag(fl: &VyperFlagDef) -> String {
        let mut out = String::new();
        if let Some(doc) = &fl.doc {
            out.push_str(&format!("# {}\n", doc));
        }
        out.push_str(&format!("flag {}:\n", fl.name));
        for v in &fl.variants {
            out.push_str(&format!("    {}\n", v));
        }
        out
    }
    pub(super) fn emit_event(ev: &VyperEvent) -> String {
        let mut out = String::new();
        if let Some(doc) = &ev.doc {
            out.push_str(&format!("# {}\n", doc));
        }
        out.push_str(&format!("event {}:\n", ev.name));
        if ev.fields.is_empty() {
            out.push_str("    pass\n");
        } else {
            for (name, ty, indexed) in &ev.fields {
                if *indexed {
                    out.push_str(&format!("    {}: indexed({})\n", name, ty));
                } else {
                    out.push_str(&format!("    {}: {}\n", name, ty));
                }
            }
        }
        out
    }
    pub(super) fn emit_constant(c: &VyperConstant) -> String {
        if let Some(doc) = &c.doc {
            format!("# {}\n{}: constant({}) = {}\n", doc, c.name, c.ty, c.value)
        } else {
            format!("{}: constant({}) = {}\n", c.name, c.ty, c.value)
        }
    }
    pub(super) fn emit_storage_var(sv: &VyperStorageVar) -> String {
        let pub_str = if sv.is_public { "(public)" } else { "" };
        if let Some(doc) = &sv.doc {
            format!("# {}\n{}: {}{}\n", doc, sv.name, sv.ty, pub_str)
        } else {
            format!("{}: {}{}\n", sv.name, sv.ty, pub_str)
        }
    }
    pub(super) fn emit_function(func: &VyperFunction, is_init: bool) -> String {
        let mut out = String::new();
        if let Some(doc) = &func.doc {
            out.push_str(&format!("# @notice {}\n", doc));
        }
        for dec in &func.decorators {
            out.push_str(&format!("{}\n", dec));
        }
        let params: Vec<String> = func.params.iter().map(|p| p.to_string()).collect();
        let fn_name = if is_init { "__init__" } else { &func.name };
        let ret = func
            .return_ty
            .as_ref()
            .map(|t| format!(" -> {}", t))
            .unwrap_or_default();
        out.push_str(&format!("def {}({}){}:\n", fn_name, params.join(", "), ret));
        if func.body.is_empty() {
            out.push_str("    pass\n");
        } else {
            for stmt in &func.body {
                out.push_str(&Self::emit_stmt(stmt, 1));
            }
        }
        out
    }
    pub(super) fn indent(level: usize) -> String {
        "    ".repeat(level)
    }
    pub(super) fn emit_stmt(stmt: &VyperStmt, indent: usize) -> String {
        let ind = Self::indent(indent);
        match stmt {
            VyperStmt::VarDecl(name, ty, init) => {
                if let Some(expr) = init {
                    format!("{}{}: {} = {}\n", ind, name, ty, expr)
                } else {
                    format!("{}{}: {}\n", ind, name, ty)
                }
            }
            VyperStmt::Assign(lhs, rhs) => format!("{}{} = {}\n", ind, lhs, rhs),
            VyperStmt::AugAssign(op, lhs, rhs) => {
                format!("{}{} {}= {}\n", ind, lhs, op, rhs)
            }
            VyperStmt::ExprStmt(expr) => format!("{}{}\n", ind, expr),
            VyperStmt::Return(None) => format!("{}return\n", ind),
            VyperStmt::Return(Some(expr)) => format!("{}return {}\n", ind, expr),
            VyperStmt::If(cond, then_stmts, else_stmts) => {
                let mut out = format!("{}if {}:\n", ind, cond);
                if then_stmts.is_empty() {
                    out.push_str(&format!("{}    pass\n", ind));
                } else {
                    for s in then_stmts {
                        out.push_str(&Self::emit_stmt(s, indent + 1));
                    }
                }
                if !else_stmts.is_empty() {
                    out.push_str(&format!("{}else:\n", ind));
                    for s in else_stmts {
                        out.push_str(&Self::emit_stmt(s, indent + 1));
                    }
                }
                out
            }
            VyperStmt::ForRange(var, ty, bound, body) => {
                let mut out = format!("{}for {}: {} in range({}):\n", ind, var, ty, bound);
                if body.is_empty() {
                    out.push_str(&format!("{}    pass\n", ind));
                } else {
                    for s in body {
                        out.push_str(&Self::emit_stmt(s, indent + 1));
                    }
                }
                out
            }
            VyperStmt::ForIn(var, ty, array, body) => {
                let mut out = format!("{}for {}: {} in {}:\n", ind, var, ty, array);
                if body.is_empty() {
                    out.push_str(&format!("{}    pass\n", ind));
                } else {
                    for s in body {
                        out.push_str(&Self::emit_stmt(s, indent + 1));
                    }
                }
                out
            }
            VyperStmt::Log(name, args) => {
                let strs: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                format!("{}log {}({})\n", ind, name, strs.join(", "))
            }
            VyperStmt::Assert(cond, msg) => {
                if let Some(m) = msg {
                    format!("{}assert {}, \"{}\"\n", ind, cond, m)
                } else {
                    format!("{}assert {}\n", ind, cond)
                }
            }
            VyperStmt::Raise(msg) => format!("{}raise \"{}\"\n", ind, msg),
            VyperStmt::Pass => format!("{}pass\n", ind),
            VyperStmt::Break => format!("{}break\n", ind),
            VyperStmt::Continue => format!("{}continue\n", ind),
            VyperStmt::Send(addr, amount) => {
                format!("{}send({}, {})\n", ind, addr, amount)
            }
            VyperStmt::SelfDestruct(to) => format!("{}selfdestruct({})\n", ind, to),
            VyperStmt::Comment(text) => format!("{}# {}\n", ind, text),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VypPassConfig {
    pub phase: VypPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl VypPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: VypPassPhase) -> Self {
        VypPassConfig {
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
#[derive(Debug, Clone)]
pub struct VypWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl VypWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        VypWorklist {
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
/// A function parameter.
#[derive(Debug, Clone)]
pub struct VyperParam {
    pub name: String,
    pub ty: VyperType,
    pub default: Option<VyperExpr>,
}
impl VyperParam {
    pub fn new(name: impl Into<String>, ty: VyperType) -> Self {
        Self {
            name: name.into(),
            ty,
            default: None,
        }
    }
    pub fn with_default(mut self, default: VyperExpr) -> Self {
        self.default = Some(default);
        self
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VypLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl VypLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        VypLivenessInfo {
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
#[derive(Debug, Clone, PartialEq)]
pub enum VypPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl VypPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            VypPassPhase::Analysis => "analysis",
            VypPassPhase::Transformation => "transformation",
            VypPassPhase::Verification => "verification",
            VypPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, VypPassPhase::Transformation | VypPassPhase::Cleanup)
    }
}
/// Statistics for VyperExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct VyperExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl VyperExtPassStats {
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
    pub fn merge(&mut self, o: &VyperExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Worklist for VyperExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VyperExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl VyperExtWorklist {
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
/// Pass registry for VyperExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct VyperExtPassRegistry {
    pub(super) configs: Vec<VyperExtPassConfig>,
    pub(super) stats: Vec<VyperExtPassStats>,
}
impl VyperExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: VyperExtPassConfig) {
        self.stats.push(VyperExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&VyperExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&VyperExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&VyperExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &VyperExtPassPhase) -> Vec<&VyperExtPassConfig> {
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
/// Vyper statement AST node.
#[derive(Debug, Clone)]
pub enum VyperStmt {
    /// `name: T = expr`
    VarDecl(String, VyperType, Option<VyperExpr>),
    /// `lhs = rhs`
    Assign(VyperExpr, VyperExpr),
    /// `lhs += rhs` / `lhs -= rhs` / etc.
    AugAssign(String, VyperExpr, VyperExpr),
    /// `expr` (function call as statement)
    ExprStmt(VyperExpr),
    /// `return expr`
    Return(Option<VyperExpr>),
    /// `if cond:\n  body\nelse:\n  else_`
    If(VyperExpr, Vec<VyperStmt>, Vec<VyperStmt>),
    /// `for var: T in range(n):\n  body`
    ForRange(String, VyperType, VyperExpr, Vec<VyperStmt>),
    /// `for var: T in array:\n  body`
    ForIn(String, VyperType, VyperExpr, Vec<VyperStmt>),
    /// `log EventName(args...)`
    Log(String, Vec<VyperExpr>),
    /// `assert cond, msg`
    Assert(VyperExpr, Option<String>),
    /// `raise "msg"`
    Raise(String),
    /// `pass`
    Pass,
    /// `break`
    Break,
    /// `continue`
    Continue,
    /// `send(addr, amount)`
    Send(VyperExpr, VyperExpr),
    /// `selfdestruct(to)`
    SelfDestruct(VyperExpr),
    /// Multi-line comment block (emitted as `# comment`)
    Comment(String),
}
/// A Vyper module-level constant.
#[derive(Debug, Clone)]
pub struct VyperConstant {
    pub name: String,
    pub ty: VyperType,
    pub value: VyperExpr,
    pub doc: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VypDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl VypDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        VypDominatorTree {
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
/// A Vyper struct definition.
#[derive(Debug, Clone)]
pub struct VyperStruct {
    pub name: String,
    pub fields: Vec<(String, VyperType)>,
    pub doc: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VypAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, VypCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl VypAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        VypAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&VypCacheEntry> {
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
            VypCacheEntry {
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
/// Dominator tree for VyperExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VyperExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl VyperExtDomTree {
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
/// Vyper type representation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VyperType {
    /// `uint256`
    Uint256,
    /// `uint128`
    Uint128,
    /// `uint64`
    Uint64,
    /// `uint32`
    Uint32,
    /// `uint8`
    Uint8,
    /// `int256`
    Int256,
    /// `int128`
    Int128,
    /// `int64`
    Int64,
    /// `int32`
    Int32,
    /// `int8`
    Int8,
    /// `address`
    Address,
    /// `bool`
    Bool,
    /// `bytes32`
    Bytes32,
    /// `bytes4`
    Bytes4,
    /// `Bytes[N]` — bounded byte array
    Bytes(u32),
    /// `String[N]` — bounded string
    StringTy(u32),
    /// `DynArray[T, N]` — dynamic array with max length
    DynArray(Box<VyperType>, u32),
    /// `T[N]` — fixed-size array
    FixedArray(Box<VyperType>, u32),
    /// `HashMap[K, V]`
    HashMap(Box<VyperType>, Box<VyperType>),
    /// A named struct type
    Struct(String),
    /// `decimal` (Vyper fixed-point)
    Decimal,
    /// `flag` type (Vyper 0.3.8+, bitset enum)
    Flag(String),
}
impl VyperType {
    /// Returns the ABI-canonical type string for selector computation.
    pub fn abi_canonical(&self) -> String {
        match self {
            VyperType::Uint256 => "uint256".into(),
            VyperType::Uint128 => "uint128".into(),
            VyperType::Uint64 => "uint64".into(),
            VyperType::Uint32 => "uint32".into(),
            VyperType::Uint8 => "uint8".into(),
            VyperType::Int256 => "int256".into(),
            VyperType::Int128 => "int128".into(),
            VyperType::Int64 => "int64".into(),
            VyperType::Int32 => "int32".into(),
            VyperType::Int8 => "int8".into(),
            VyperType::Address => "address".into(),
            VyperType::Bool => "bool".into(),
            VyperType::Bytes32 => "bytes32".into(),
            VyperType::Bytes4 => "bytes4".into(),
            VyperType::Bytes(n) => format!("bytes{}", n),
            VyperType::StringTy(_) => "string".into(),
            VyperType::DynArray(elem, _) => format!("{}[]", elem.abi_canonical()),
            VyperType::FixedArray(elem, n) => format!("{}[{}]", elem.abi_canonical(), n),
            VyperType::HashMap(_, _) => "bytes32".into(),
            VyperType::Struct(name) => name.clone(),
            VyperType::Decimal => "int128".into(),
            VyperType::Flag(name) => name.clone(),
        }
    }
    /// Returns true if this type is a storage/memory type needing initialization.
    pub fn needs_init(&self) -> bool {
        matches!(
            self,
            VyperType::DynArray(_, _)
                | VyperType::HashMap(_, _)
                | VyperType::Bytes(_)
                | VyperType::StringTy(_)
        )
    }
}
/// Vyper expression AST node.
#[derive(Debug, Clone, PartialEq)]
pub enum VyperExpr {
    /// Integer literal: `42`
    IntLit(i128),
    /// Boolean literal: `True` / `False`
    BoolLit(bool),
    /// String literal: `"hello"`
    StrLit(String),
    /// Hex literal: `0xdeadbeef`
    HexLit(String),
    /// Variable reference: `my_var`
    Var(String),
    /// `self.field` — storage variable access
    SelfField(String),
    /// `msg.sender`
    MsgSender,
    /// `msg.value`
    MsgValue,
    /// `block.timestamp`
    BlockTimestamp,
    /// `block.number`
    BlockNumber,
    /// `chain.id`
    ChainId,
    /// `tx.origin`
    TxOrigin,
    /// `len(expr)`
    Len(Box<VyperExpr>),
    /// `convert(expr, T)`
    Convert(Box<VyperExpr>, VyperType),
    /// `concat(a, b, ...)`
    Concat(Vec<VyperExpr>),
    /// `keccak256(expr)`
    Keccak256(Box<VyperExpr>),
    /// `sha256(expr)`
    Sha256(Box<VyperExpr>),
    /// `ecrecover(hash, v, r, s)`
    Ecrecover(
        Box<VyperExpr>,
        Box<VyperExpr>,
        Box<VyperExpr>,
        Box<VyperExpr>,
    ),
    /// `extract32(expr, start)`
    Extract32(Box<VyperExpr>, Box<VyperExpr>),
    /// Field access on a struct: `expr.field`
    FieldAccess(Box<VyperExpr>, String),
    /// `expr[index]`
    Index(Box<VyperExpr>, Box<VyperExpr>),
    /// Function call: `f(args...)`
    Call(String, Vec<VyperExpr>),
    /// External call: `Interface(addr).method(args...)`
    ExtCall(String, Box<VyperExpr>, String, Vec<VyperExpr>),
    /// Binary operation: `a + b`
    BinOp(String, Box<VyperExpr>, Box<VyperExpr>),
    /// Unary operation: `not a`, `-a`
    UnaryOp(String, Box<VyperExpr>),
    /// Ternary: `value if cond else default`
    IfExpr(Box<VyperExpr>, Box<VyperExpr>, Box<VyperExpr>),
    /// Struct literal: `MyStruct({field: val, ...})`
    StructLit(String, Vec<(String, VyperExpr)>),
    /// List literal: `[a, b, c]`
    ListLit(Vec<VyperExpr>),
    /// `empty(T)` — zero value
    Empty(VyperType),
    /// `max_value(T)`
    MaxValue(VyperType),
    /// `min_value(T)`
    MinValue(VyperType),
    /// `isqrt(n)`
    Isqrt(Box<VyperExpr>),
    /// `uint2str(n)` (Vyper 0.3+)
    Uint2Str(Box<VyperExpr>),
    /// `raw_call(addr, data, value=v, gas=g)`
    RawCall {
        addr: Box<VyperExpr>,
        data: Box<VyperExpr>,
        value: Option<Box<VyperExpr>>,
        gas: Option<Box<VyperExpr>>,
    },
    /// `create_minimal_proxy_to(target)`
    CreateMinimalProxy(Box<VyperExpr>),
    /// `create_copy_of(target)`
    CreateCopyOf(Box<VyperExpr>),
    /// `create_from_blueprint(blueprint, args...)`
    CreateFromBlueprint(Box<VyperExpr>, Vec<VyperExpr>),
}
/// A complete Vyper contract (module).
#[derive(Debug, Clone)]
pub struct VyperContract {
    pub name: String,
    pub structs: Vec<VyperStruct>,
    pub flags: Vec<VyperFlagDef>,
    pub interfaces: Vec<VyperInterface>,
    pub constants: Vec<VyperConstant>,
    pub storage: Vec<VyperStorageVar>,
    pub events: Vec<VyperEvent>,
    /// `__init__` function (constructor).
    pub constructor: Option<VyperFunction>,
    /// `__default__` function (fallback).
    pub default_fn: Option<VyperFunction>,
    pub functions: Vec<VyperFunction>,
    pub doc: Option<String>,
}
impl VyperContract {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            structs: Vec::new(),
            flags: Vec::new(),
            interfaces: Vec::new(),
            constants: Vec::new(),
            storage: Vec::new(),
            events: Vec::new(),
            constructor: None,
            default_fn: None,
            functions: Vec::new(),
            doc: None,
        }
    }
}
/// Vyper function decorator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VyperDecorator {
    /// `@external`
    External,
    /// `@internal`
    Internal,
    /// `@view`
    View,
    /// `@pure`
    Pure,
    /// `@payable`
    Payable,
    /// `@nonreentrant("lock")`
    NonReentrant(String),
    /// `@deploy` (constructor in Vyper 0.4+)
    Deploy,
}
#[allow(dead_code)]
pub struct VypPassRegistry {
    pub(super) configs: Vec<VypPassConfig>,
    pub(super) stats: std::collections::HashMap<String, VypPassStats>,
}
impl VypPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        VypPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: VypPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), VypPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&VypPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&VypPassStats> {
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
pub struct VypConstantFoldingHelper;
impl VypConstantFoldingHelper {
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
pub struct VypCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VypDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl VypDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        VypDepGraph {
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
/// Configuration for VyperExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VyperExtPassConfig {
    pub name: String,
    pub phase: VyperExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl VyperExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: VyperExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: VyperExtPassPhase) -> Self {
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
/// A Vyper event definition.
#[derive(Debug, Clone)]
pub struct VyperEvent {
    pub name: String,
    /// `(name, ty, indexed)`
    pub fields: Vec<(String, VyperType, bool)>,
    pub doc: Option<String>,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct VypPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl VypPassStats {
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
/// A Vyper interface definition.
#[derive(Debug, Clone)]
pub struct VyperInterface {
    pub name: String,
    pub functions: Vec<VyperFunction>,
    pub doc: Option<String>,
}
/// Constant folding helper for VyperExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct VyperExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl VyperExtConstFolder {
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
/// Compilation context for a single Vyper source file.
#[derive(Debug, Clone)]
pub struct VyperCompilationCtx {
    /// Vyper version pragma (e.g., `"0.3.10"`).
    pub version: String,
    /// Whether ABI v2 is enabled.
    pub abi_v2: bool,
    /// Whether to include runtime helpers.
    pub include_runtime: bool,
    /// Extra `# pragma` lines.
    pub extra_pragmas: Vec<String>,
}
