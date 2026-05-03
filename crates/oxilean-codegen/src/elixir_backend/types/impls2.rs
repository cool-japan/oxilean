//! Implementation blocks (part 2)

use super::super::functions::ELIXIR_RUNTIME;
use super::super::functions::*;
use super::defs::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

impl ElixirX2PassStats {
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
    pub fn merge(&mut self, o: &ElixirX2PassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
impl ElixirBackend {
    /// Create a new `ElixirBackend` with default settings.
    pub fn new() -> Self {
        ElixirBackend {
            indent_str: "  ".to_string(),
        }
    }
    /// Create a backend that uses a custom indentation string.
    pub fn with_indent(indent: &str) -> Self {
        ElixirBackend {
            indent_str: indent.to_string(),
        }
    }
    /// Mangle an OxiLean identifier into a valid Elixir atom/function name.
    ///
    /// - CamelCase becomes `snake_case`
    /// - Reserved Elixir words get a trailing `_`
    pub fn mangle_name(&self, name: &str) -> String {
        let snake = to_snake_case(name);
        if is_elixir_reserved(&snake) {
            format!("{}_", snake)
        } else {
            snake
        }
    }
    /// Mangle an OxiLean module/type name into a valid Elixir module atom.
    ///
    /// Elixir modules are `CamelCase` atoms.
    pub fn mangle_module_name(&self, name: &str) -> String {
        let mut chars = name.chars();
        match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
    /// Emit an [`ElixirExpr`] as Elixir source text.
    pub fn emit_expr(&self, expr: &ElixirExpr) -> String {
        self.emit_expr_indented(expr, 0)
    }
    pub(super) fn emit_expr_indented(&self, expr: &ElixirExpr, depth: usize) -> String {
        let pad = self.indent_str.repeat(depth);
        match expr {
            ElixirExpr::Nil => "nil".to_string(),
            ElixirExpr::Bool(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            ElixirExpr::Atom(a) => format!(":{}", a),
            ElixirExpr::Integer(n) => n.to_string(),
            ElixirExpr::Float(f) => format!("{}", f),
            ElixirExpr::Binary(s) => format!("\"{}\"", escape_elixir_string(s)),
            ElixirExpr::Var(v) => v.clone(),
            ElixirExpr::List(elems) => {
                if elems.is_empty() {
                    "[]".to_string()
                } else {
                    let inner = elems
                        .iter()
                        .map(|e| self.emit_expr_indented(e, depth))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("[{}]", inner)
                }
            }
            ElixirExpr::Tuple(elems) => {
                let inner = elems
                    .iter()
                    .map(|e| self.emit_expr_indented(e, depth))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{{{}}}", inner)
            }
            ElixirExpr::Map(pairs) => {
                if pairs.is_empty() {
                    "%{}".to_string()
                } else {
                    let inner = pairs
                        .iter()
                        .map(|(k, v)| {
                            format!(
                                "{} => {}",
                                self.emit_expr_indented(k, depth),
                                self.emit_expr_indented(v, depth)
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("%{{{}}}", inner)
                }
            }
            ElixirExpr::Struct(name, fields) => {
                if fields.is_empty() {
                    format!("%{}{{%{{}}}}", name)
                } else {
                    let inner = fields
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, self.emit_expr_indented(v, depth)))
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("%{}{{{}}}", name, inner)
                }
            }
            ElixirExpr::FuncCall(func, args) => {
                let arg_str = args
                    .iter()
                    .map(|a| self.emit_expr_indented(a, depth))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}({})", func, arg_str)
            }
            ElixirExpr::Match(pat, val) => {
                format!(
                    "{} = {}",
                    self.emit_expr_indented(pat, depth),
                    self.emit_expr_indented(val, depth)
                )
            }
            ElixirExpr::Case(scrutinee, clauses) => {
                let inner_pad = self.indent_str.repeat(depth + 1);
                let mut out = format!("case {} do\n", self.emit_expr_indented(scrutinee, depth));
                for (pat, body) in clauses {
                    out += &format!(
                        "{}{} ->\n{}{}\n",
                        inner_pad,
                        self.emit_expr_indented(pat, depth + 1),
                        self.indent_str.repeat(depth + 2),
                        self.emit_expr_indented(body, depth + 2)
                    );
                }
                out += &format!("{}end", pad);
                out
            }
            ElixirExpr::Lambda(params, body) => {
                let param_str = params.join(", ");
                let body_str = self.emit_expr_indented(body, depth + 1);
                format!(
                    "fn {} ->\n{}{}\n{}end",
                    param_str,
                    self.indent_str.repeat(depth + 1),
                    body_str,
                    pad
                )
            }
            ElixirExpr::Pipe(lhs, rhs) => {
                format!(
                    "{}\n{}|> {}",
                    self.emit_expr_indented(lhs, depth),
                    pad,
                    self.emit_expr_indented(rhs, depth)
                )
            }
            ElixirExpr::Block(exprs) => exprs
                .iter()
                .map(|e| format!("{}{}", pad, self.emit_expr_indented(e, depth)))
                .collect::<Vec<_>>()
                .join("\n"),
            ElixirExpr::If(cond, then_e, else_e) => {
                format!(
                    "if {} do\n{}{}\n{}else\n{}{}\n{}end",
                    self.emit_expr_indented(cond, depth),
                    self.indent_str.repeat(depth + 1),
                    self.emit_expr_indented(then_e, depth + 1),
                    pad,
                    self.indent_str.repeat(depth + 1),
                    self.emit_expr_indented(else_e, depth + 1),
                    pad
                )
            }
            ElixirExpr::BinOp(op, lhs, rhs) => {
                format!(
                    "{} {} {}",
                    self.emit_expr_indented(lhs, depth),
                    op,
                    self.emit_expr_indented(rhs, depth)
                )
            }
            ElixirExpr::Interpolation(parts) => {
                let mut out = String::from("\"");
                for part in parts {
                    match part {
                        ElixirStringPart::Literal(s) => out += &escape_elixir_string(s),
                        ElixirStringPart::Expr(e) => {
                            out += &format!("#{{{}}}", self.emit_expr_indented(e, 0));
                        }
                    }
                }
                out.push('"');
                out
            }
        }
    }
    /// Emit an [`ElixirFunction`] as Elixir source text.
    pub fn emit_function(&self, func: &ElixirFunction) -> String {
        let keyword = if func.is_private { "defp" } else { "def" };
        let mut out = String::new();
        if let Some(doc) = &func.doc {
            out += &format!("  @doc \"\"\"\n  {}\n  \"\"\"\n", doc);
        }
        for (patterns, guard, body) in &func.clauses {
            let param_str = patterns
                .iter()
                .map(|p| self.emit_expr(p))
                .collect::<Vec<_>>()
                .join(", ");
            let guard_str = guard
                .as_ref()
                .map(|g| format!(" when {}", self.emit_expr(g)))
                .unwrap_or_default();
            let body_str = self.emit_expr_indented(body, 2);
            out += &format!(
                "  {} {}({}){} do\n    {}\n  end\n",
                keyword, func.name, param_str, guard_str, body_str
            );
        }
        out
    }
    /// Emit an [`ElixirModule`] as a complete Elixir source file.
    pub fn emit_module(&self, module: &ElixirModule) -> String {
        let mut out = String::new();
        out += "# Generated by OxiLean Elixir Backend\n\n";
        out += &format!("defmodule {} do\n", module.name);
        if let Some(doc) = module.attributes.get("moduledoc") {
            out += &format!("  @moduledoc \"\"\"\n  {}\n  \"\"\"\n\n", doc);
        } else {
            out += "  @moduledoc false\n\n";
        }
        for u in &module.use_modules {
            out += &format!("  use {}\n", u);
        }
        if !module.use_modules.is_empty() {
            out.push('\n');
        }
        for imp in &module.imports {
            out += &format!("  import {}\n", imp);
        }
        if !module.imports.is_empty() {
            out.push('\n');
        }
        for func in &module.functions {
            out += &self.emit_function(func);
            out.push('\n');
        }
        out += "end\n";
        out
    }
    /// Return the Elixir source of the OxiLean runtime module.
    pub fn emit_runtime(&self) -> &'static str {
        ELIXIR_RUNTIME
    }
}
impl ElxPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            ElxPassPhase::Analysis => "analysis",
            ElxPassPhase::Transformation => "transformation",
            ElxPassPhase::Verification => "verification",
            ElxPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, ElxPassPhase::Transformation | ElxPassPhase::Cleanup)
    }
}
impl ElixirX2DepGraph {
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
impl ElxPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ElxPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: ElxPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), ElxPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ElxPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&ElxPassStats> {
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
impl ElixirX2PassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: ElixirX2PassConfig) {
        self.stats.push(ElixirX2PassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&ElixirX2PassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&ElixirX2PassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ElixirX2PassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &ElixirX2PassPhase) -> Vec<&ElixirX2PassConfig> {
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
impl ElxDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ElxDepGraph {
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
impl ElixirExtLiveness {
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
impl ElxAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        ElxAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&ElxCacheEntry> {
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
            ElxCacheEntry {
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
impl ElixirExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: ElixirExtPassConfig) {
        self.stats.push(ElixirExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&ElixirExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&ElixirExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&ElixirExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &ElixirExtPassPhase) -> Vec<&ElixirExtPassConfig> {
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
impl ElixirExtCache {
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
impl ElxWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ElxWorklist {
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
impl ElixirX2Cache {
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
impl ElxDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        ElxDominatorTree {
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
impl ElixirX2PassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: ElixirX2PassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: ElixirX2PassPhase) -> Self {
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
impl ElxConstantFoldingHelper {
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
