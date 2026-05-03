use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::fmt;

use super::defs::*;

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
    pub(crate) fn indent_str(&self) -> String {
        "    ".repeat(self.indent)
    }
    /// Push a line to the output.
    pub(crate) fn push_line(&mut self, line: &str) {
        let indent = self.indent_str();
        self.output.push_str(&indent);
        self.output.push_str(line);
        self.output.push('\n');
    }
    /// Push an empty line.
    pub(crate) fn push_blank(&mut self) {
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
