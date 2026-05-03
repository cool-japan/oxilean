use super::super::functions::SOLIDITY_RUNTIME;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

use super::defs::*;

impl SolExtDomTree {
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

impl SolidityParam {
    pub fn new(ty: SolidityType, name: impl Into<String>) -> Self {
        let location = if ty.is_reference_type() {
            Some("memory".into())
        } else {
            None
        };
        Self {
            ty,
            location,
            name: name.into(),
        }
    }
    pub fn calldata(ty: SolidityType, name: impl Into<String>) -> Self {
        Self {
            ty,
            location: Some("calldata".into()),
            name: name.into(),
        }
    }
    pub fn storage(ty: SolidityType, name: impl Into<String>) -> Self {
        Self {
            ty,
            location: Some("storage".into()),
            name: name.into(),
        }
    }
}

impl SolPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SolPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: SolPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), SolPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&SolPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&SolPassStats> {
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

impl SolExtPassPhase {
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

impl SolExtWorklist {
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

impl SolidityBackend {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_runtime(mut self) -> Self {
        self.ctx.include_runtime = true;
        self
    }
    pub fn add_contract(&mut self, contract: SolidityContract) {
        self.contracts.push(contract);
    }
    pub fn add_pragma(&mut self, pragma: impl Into<String>) {
        self.ctx.pragmas.push(pragma.into());
    }
    pub fn add_import(&mut self, import: impl Into<String>) {
        self.ctx.imports.push(import.into());
    }
    /// Compile a single LCNF-style declaration (simplified: name → state var).
    pub fn compile_decl(&mut self, name: &str, ty: SolidityType) -> SolidityStateVar {
        SolidityStateVar {
            ty,
            name: name.into(),
            visibility: Visibility::Private,
            is_immutable: false,
            is_constant: false,
            init: None,
            doc: None,
        }
    }
    /// Emit the full Solidity source for all registered contracts.
    pub fn emit_contract(&mut self) -> String {
        let mut out = String::new();
        out.push_str("// SPDX-License-Identifier: MIT\n");
        out.push_str("// Generated by OxiLean Solidity Backend\n\n");
        for pragma in &self.ctx.pragmas {
            out.push_str(&format!("pragma solidity {};\n", pragma));
        }
        out.push('\n');
        for imp in &self.ctx.imports {
            out.push_str(&format!("import \"{}\";\n", imp));
        }
        if !self.ctx.imports.is_empty() {
            out.push('\n');
        }
        if self.ctx.include_runtime {
            out.push_str(SOLIDITY_RUNTIME);
            out.push('\n');
        }
        for contract in &self.contracts {
            out.push_str(&Self::emit_single_contract(contract));
            out.push('\n');
        }
        self.source = out.clone();
        out
    }
    pub(crate) fn emit_single_contract(c: &SolidityContract) -> String {
        let mut out = String::new();
        if let Some(doc) = &c.doc {
            out.push_str(&format!("/// @title {}\n", doc));
        }
        let bases = if c.bases.is_empty() {
            String::new()
        } else {
            format!(" is {}", c.bases.join(", "))
        };
        out.push_str(&format!("{} {}{} {{\n", c.kind, c.name, bases));
        for s in &c.structs {
            out.push_str(&Self::emit_struct(s, 1));
        }
        for e in &c.enums {
            out.push_str(&Self::emit_enum(e, 1));
        }
        for ev in &c.events {
            out.push_str(&Self::emit_event(ev, 1));
        }
        for err in &c.errors {
            out.push_str(&Self::emit_error(err, 1));
        }
        for sv in &c.state_vars {
            out.push_str(&Self::emit_state_var(sv, 1));
        }
        if let Some(ctor) = &c.constructor {
            out.push_str(&Self::emit_constructor(ctor, 1));
        }
        if let Some(recv) = &c.receive {
            out.push_str(&Self::emit_receive(recv, 1));
        }
        if let Some(fb) = &c.fallback {
            out.push_str(&Self::emit_fallback(fb, 1));
        }
        for m in &c.modifiers {
            out.push_str(&Self::emit_modifier(m, 1));
        }
        for func in &c.functions {
            out.push_str(&Self::emit_function(func, 1));
        }
        out.push_str("}\n");
        out
    }
    pub(crate) fn indent(level: usize) -> String {
        "    ".repeat(level)
    }
    pub(crate) fn emit_struct(s: &SolidityStruct, indent: usize) -> String {
        let ind = Self::indent(indent);
        let mut out = String::new();
        if let Some(doc) = &s.doc {
            out.push_str(&format!("{}/// @dev {}\n", ind, doc));
        }
        out.push_str(&format!("{}struct {} {{\n", ind, s.name));
        for (ty, name) in &s.fields {
            out.push_str(&format!("{}    {} {};\n", ind, ty, name));
        }
        out.push_str(&format!("{}}}\n", ind));
        out
    }
    pub(crate) fn emit_enum(e: &SolidityEnum, indent: usize) -> String {
        let ind = Self::indent(indent);
        let mut out = String::new();
        if let Some(doc) = &e.doc {
            out.push_str(&format!("{}/// @dev {}\n", ind, doc));
        }
        out.push_str(&format!(
            "{}enum {} {{ {} }}\n",
            ind,
            e.name,
            e.variants.join(", ")
        ));
        out
    }
    pub(crate) fn emit_event(ev: &SolidityEvent, indent: usize) -> String {
        let ind = Self::indent(indent);
        let mut out = String::new();
        if let Some(doc) = &ev.doc {
            out.push_str(&format!("{}/// @dev {}\n", ind, doc));
        }
        let fields: Vec<String> = ev
            .fields
            .iter()
            .map(|(ty, indexed, name)| {
                if *indexed {
                    format!("{} indexed {}", ty, name)
                } else {
                    format!("{} {}", ty, name)
                }
            })
            .collect();
        let anon = if ev.anonymous { " anonymous" } else { "" };
        out.push_str(&format!(
            "{}event {}({}){};\n",
            ind,
            ev.name,
            fields.join(", "),
            anon
        ));
        out
    }
    pub(crate) fn emit_error(err: &SolidityError, indent: usize) -> String {
        let ind = Self::indent(indent);
        let params: Vec<String> = err.params.iter().map(|p| p.to_string()).collect();
        format!("{}error {}({});\n", ind, err.name, params.join(", "))
    }
    pub(crate) fn emit_state_var(sv: &SolidityStateVar, indent: usize) -> String {
        let ind = Self::indent(indent);
        let mut out = String::new();
        if let Some(doc) = &sv.doc {
            out.push_str(&format!("{}/// @dev {}\n", ind, doc));
        }
        let mut parts = vec![sv.ty.to_string(), sv.visibility.to_string()];
        if sv.is_constant {
            parts.push("constant".into());
        } else if sv.is_immutable {
            parts.push("immutable".into());
        }
        parts.push(sv.name.clone());
        if let Some(init) = &sv.init {
            out.push_str(&format!("{}{}  = {};\n", ind, parts.join(" "), init));
        } else {
            out.push_str(&format!("{}{};\n", ind, parts.join(" ")));
        }
        out
    }
    pub(crate) fn emit_constructor(ctor: &SolidityFunction, indent: usize) -> String {
        let ind = Self::indent(indent);
        let params: Vec<String> = ctor.params.iter().map(|p| p.to_string()).collect();
        let mut header = format!("{}constructor({})", ind, params.join(", "));
        let mut muts = String::new();
        let m_str = ctor.mutability.to_string();
        if !m_str.is_empty() {
            muts.push(' ');
            muts.push_str(&m_str);
        }
        header.push_str(&muts);
        Self::emit_fn_body(&header, &ctor.body, indent)
    }
    pub(crate) fn emit_receive(recv: &SolidityFunction, indent: usize) -> String {
        let ind = Self::indent(indent);
        let header = format!("{}receive() external payable", ind);
        Self::emit_fn_body(&header, &recv.body, indent)
    }
    pub(crate) fn emit_fallback(fb: &SolidityFunction, indent: usize) -> String {
        let ind = Self::indent(indent);
        let header = format!("{}fallback() external payable", ind);
        Self::emit_fn_body(&header, &fb.body, indent)
    }
    pub(crate) fn emit_modifier(m: &SolidityModifier, indent: usize) -> String {
        let ind = Self::indent(indent);
        let params: Vec<String> = m.params.iter().map(|p| p.to_string()).collect();
        let header = format!("{}modifier {}({})", ind, m.name, params.join(", "));
        Self::emit_fn_body(&header, &m.body, indent)
    }
    pub(crate) fn emit_function(func: &SolidityFunction, indent: usize) -> String {
        let ind = Self::indent(indent);
        let mut out = String::new();
        if let Some(doc) = &func.doc {
            out.push_str(&format!("{}/// @notice {}\n", ind, doc));
        }
        let params: Vec<String> = func.params.iter().map(|p| p.to_string()).collect();
        let mut header = format!(
            "{}function {}({}) {} {}",
            ind,
            func.name,
            params.join(", "),
            func.visibility,
            func.mutability
        );
        header = header.trim_end().to_string();
        if func.is_virtual {
            header.push_str(" virtual");
        }
        if func.is_override {
            header.push_str(" override");
        }
        for (mod_name, mod_args) in &func.modifiers {
            if mod_args.is_empty() {
                header.push_str(&format!(" {}", mod_name));
            } else {
                let args: Vec<String> = mod_args.iter().map(|a| a.to_string()).collect();
                header.push_str(&format!(" {}({})", mod_name, args.join(", ")));
            }
        }
        if !func.returns.is_empty() {
            let rets: Vec<String> = func.returns.iter().map(|p| p.to_string()).collect();
            header.push_str(&format!(" returns ({})", rets.join(", ")));
        }
        if func.body.is_empty() {
            out.push_str(&format!("{};\n", header));
        } else {
            out.push_str(&Self::emit_fn_body(&header, &func.body, indent));
        }
        out
    }
    pub(crate) fn emit_fn_body(header: &str, body: &[SolidityStmt], indent: usize) -> String {
        let mut out = format!("{} {{\n", header);
        for stmt in body {
            out.push_str(&Self::emit_stmt(stmt, indent + 1));
        }
        out.push_str(&format!("{}}}\n", Self::indent(indent)));
        out
    }
    pub(crate) fn emit_stmt(stmt: &SolidityStmt, indent: usize) -> String {
        let ind = Self::indent(indent);
        match stmt {
            SolidityStmt::VarDecl {
                ty,
                location,
                name,
                init,
            } => {
                let loc_str = location.as_deref().unwrap_or("");
                let loc_part = if loc_str.is_empty() {
                    String::new()
                } else {
                    format!(" {}", loc_str)
                };
                if let Some(expr) = init {
                    format!("{}{}{} {} = {};\n", ind, ty, loc_part, name, expr)
                } else {
                    format!("{}{}{} {};\n", ind, ty, loc_part, name)
                }
            }
            SolidityStmt::Assign(lhs, rhs) => format!("{}{} = {};\n", ind, lhs, rhs),
            SolidityStmt::CompoundAssign(op, lhs, rhs) => {
                format!("{}{} {}= {};\n", ind, lhs, op, rhs)
            }
            SolidityStmt::ExprStmt(expr) => format!("{}{};\n", ind, expr),
            SolidityStmt::Return(None) => format!("{}return;\n", ind),
            SolidityStmt::Return(Some(expr)) => format!("{}return {};\n", ind, expr),
            SolidityStmt::If(cond, then_stmts, else_stmts) => {
                let mut out = format!("{}if ({}) {{\n", ind, cond);
                for s in then_stmts {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                if else_stmts.is_empty() {
                    out.push_str(&format!("{}}}\n", ind));
                } else {
                    out.push_str(&format!("{}}} else {{\n", ind));
                    for s in else_stmts {
                        out.push_str(&Self::emit_stmt(s, indent + 1));
                    }
                    out.push_str(&format!("{}}}\n", ind));
                }
                out
            }
            SolidityStmt::While(cond, body) => {
                let mut out = format!("{}while ({}) {{\n", ind, cond);
                for s in body {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                out.push_str(&format!("{}}}\n", ind));
                out
            }
            SolidityStmt::For(init, cond, update, body) => {
                let init_str = init
                    .as_ref()
                    .map(|s| {
                        Self::emit_stmt(s, 0)
                            .trim_end_matches('\n')
                            .trim_end_matches(';')
                            .to_string()
                    })
                    .unwrap_or_default();
                let cond_str = cond.as_ref().map(|e| e.to_string()).unwrap_or_default();
                let upd_str = update
                    .as_ref()
                    .map(|s| {
                        Self::emit_stmt(s, 0)
                            .trim_end_matches('\n')
                            .trim_end_matches(';')
                            .to_string()
                    })
                    .unwrap_or_default();
                let mut out = format!("{}for ({}; {}; {}) {{\n", ind, init_str, cond_str, upd_str);
                for s in body {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                out.push_str(&format!("{}}}\n", ind));
                out
            }
            SolidityStmt::DoWhile(body, cond) => {
                let mut out = format!("{}do {{\n", ind);
                for s in body {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                out.push_str(&format!("{}}} while ({});\n", ind, cond));
                out
            }
            SolidityStmt::Emit(name, args) => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                format!("{}emit {}({});\n", ind, name, args_str.join(", "))
            }
            SolidityStmt::Revert(name, args) => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                format!("{}revert {}({});\n", ind, name, args_str.join(", "))
            }
            SolidityStmt::Require(cond, msg) => {
                if let Some(m) = msg {
                    format!("{}require({}, \"{}\");\n", ind, cond, m)
                } else {
                    format!("{}require({});\n", ind, cond)
                }
            }
            SolidityStmt::Assert(cond) => format!("{}assert({});\n", ind, cond),
            SolidityStmt::Break => format!("{}break;\n", ind),
            SolidityStmt::Continue => format!("{}continue;\n", ind),
            SolidityStmt::Unchecked(stmts) => {
                let mut out = format!("{}unchecked {{\n", ind);
                for s in stmts {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                out.push_str(&format!("{}}}\n", ind));
                out
            }
            SolidityStmt::Assembly(body) => {
                format!("{}assembly {{\n{}{}}}\n", ind, body, ind)
            }
            SolidityStmt::MultiAssign(lhs, rhs) => {
                let lhs_str: Vec<String> = lhs.iter().map(|e| e.to_string()).collect();
                format!("{}({}) = {};\n", ind, lhs_str.join(", "), rhs)
            }
            SolidityStmt::Block(stmts) => {
                let mut out = format!("{}{{\n", ind);
                for s in stmts {
                    out.push_str(&Self::emit_stmt(s, indent + 1));
                }
                out.push_str(&format!("{}}}\n", ind));
                out
            }
        }
    }
}

impl SolLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        SolLivenessInfo {
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

impl SolExtCache {
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

impl SolDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SolDepGraph {
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
