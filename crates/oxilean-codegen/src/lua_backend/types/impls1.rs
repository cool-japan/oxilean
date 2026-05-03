//! Implementation blocks (part 1)

use super::super::functions::LUA_KEYWORDS;
use super::super::functions::*;
use super::defs::*;
use crate::lcnf::*;
use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};

impl LuaExtSourceBuffer {
    pub fn new() -> Self {
        LuaExtSourceBuffer {
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
impl LuaDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        LuaDominatorTree {
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
impl LuaExtEmitStats {
    pub fn new() -> Self {
        LuaExtEmitStats::default()
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
impl LuaPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: LuaPassPhase) -> Self {
        LuaPassConfig {
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
impl LuaExtVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        LuaExtVersion {
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
    pub fn is_compatible_with(&self, other: &LuaExtVersion) -> bool {
        self.major == other.major && self.minor >= other.minor
    }
}
impl LuaFunction {
    /// Create a new named function.
    pub fn new(
        name: impl Into<std::string::String>,
        params: Vec<std::string::String>,
        body: Vec<LuaStmt>,
    ) -> Self {
        LuaFunction {
            name: Some(name.into()),
            params,
            vararg: false,
            body,
            is_local: false,
            is_method: false,
        }
    }
    /// Create a new local function.
    pub fn new_local(
        name: impl Into<std::string::String>,
        params: Vec<std::string::String>,
        body: Vec<LuaStmt>,
    ) -> Self {
        LuaFunction {
            name: Some(name.into()),
            params,
            vararg: false,
            body,
            is_local: true,
            is_method: false,
        }
    }
}
impl LuaPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            LuaPassPhase::Analysis => "analysis",
            LuaPassPhase::Transformation => "transformation",
            LuaPassPhase::Verification => "verification",
            LuaPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, LuaPassPhase::Transformation | LuaPassPhase::Cleanup)
    }
}
impl LuaExtWorklist {
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
impl LuaExtPassStats {
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
    pub fn merge(&mut self, o: &LuaExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
impl LuaDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        LuaDepGraph {
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
impl LuaPassStats {
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
impl LuaExtEventLog {
    pub fn new(capacity: usize) -> Self {
        LuaExtEventLog {
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
impl LuaExtLiveness {
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
impl LuaExtProfiler {
    pub fn new() -> Self {
        LuaExtProfiler::default()
    }
    pub fn record(&mut self, t: LuaExtPassTiming) {
        self.timings.push(t);
    }
    pub fn total_elapsed_us(&self) -> u64 {
        self.timings.iter().map(|t| t.elapsed_us).sum()
    }
    pub fn slowest_pass(&self) -> Option<&LuaExtPassTiming> {
        self.timings.iter().max_by_key(|t| t.elapsed_us)
    }
    pub fn num_passes(&self) -> usize {
        self.timings.len()
    }
    pub fn profitable_passes(&self) -> Vec<&LuaExtPassTiming> {
        self.timings.iter().filter(|t| t.is_profitable()).collect()
    }
}
impl LuaExtDiagCollector {
    pub fn new() -> Self {
        LuaExtDiagCollector::default()
    }
    pub fn emit(&mut self, d: LuaExtDiagMsg) {
        self.msgs.push(d);
    }
    pub fn has_errors(&self) -> bool {
        self.msgs
            .iter()
            .any(|d| d.severity == LuaExtDiagSeverity::Error)
    }
    pub fn errors(&self) -> Vec<&LuaExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == LuaExtDiagSeverity::Error)
            .collect()
    }
    pub fn warnings(&self) -> Vec<&LuaExtDiagMsg> {
        self.msgs
            .iter()
            .filter(|d| d.severity == LuaExtDiagSeverity::Warning)
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
impl LuaExtIncrKey {
    pub fn new(content: u64, config: u64) -> Self {
        LuaExtIncrKey {
            content_hash: content,
            config_hash: config,
        }
    }
    pub fn combined_hash(&self) -> u64 {
        self.content_hash.wrapping_mul(0x9e3779b97f4a7c15) ^ self.config_hash
    }
    pub fn matches(&self, other: &LuaExtIncrKey) -> bool {
        self.content_hash == other.content_hash && self.config_hash == other.config_hash
    }
}
impl LuaBackend {
    /// Create a new `LuaBackend`.
    pub fn new() -> Self {
        LuaBackend {
            fresh_counter: 0,
            name_cache: HashMap::new(),
        }
    }
    /// Generate a fresh variable name.
    pub fn fresh_var(&mut self) -> std::string::String {
        let n = self.fresh_counter;
        self.fresh_counter += 1;
        format!("_t{}", n)
    }
    /// Mangle an OxiLean name to a valid Lua identifier.
    pub fn mangle_name(&mut self, name: &str) -> std::string::String {
        if let Some(cached) = self.name_cache.get(name) {
            return cached.clone();
        }
        if name.is_empty() {
            return "_anon".to_string();
        }
        let mangled: std::string::String = name
            .chars()
            .map(|c| match c {
                '.' | ':' => '_',
                '\'' => '_',
                c if c.is_alphanumeric() || c == '_' => c,
                _ => '_',
            })
            .collect();
        let mangled = if LUA_KEYWORDS.contains(&mangled.as_str())
            || mangled.starts_with(|c: char| c.is_ascii_digit())
        {
            format!("_{}", mangled)
        } else {
            mangled
        };
        self.name_cache.insert(name.to_string(), mangled.clone());
        mangled
    }
    /// Map an LCNF type to a Lua type hint.
    pub fn lcnf_to_lua_type(ty: &LcnfType) -> LuaType {
        match ty {
            LcnfType::Nat => LuaType::Number(true),
            LcnfType::LcnfString => LuaType::String,
            LcnfType::Unit | LcnfType::Erased | LcnfType::Irrelevant => LuaType::Nil,
            LcnfType::Object => LuaType::Table,
            LcnfType::Var(name) => LuaType::Custom(name.clone()),
            LcnfType::Fun(..) => LuaType::Function,
            LcnfType::Ctor(name, _) => LuaType::Custom(name.clone()),
        }
    }
    /// Compile an LCNF literal to a Lua expression.
    pub fn compile_lit(lit: &LcnfLit) -> LuaExpr {
        match lit {
            LcnfLit::Nat(n) => LuaExpr::Int(*n as i64),
            LcnfLit::Str(s) => LuaExpr::Str(s.clone()),
        }
    }
    /// Compile an LCNF literal value to a Lua expression.
    pub fn compile_let_value(&mut self, value: &LcnfLetValue) -> LuaExpr {
        match value {
            LcnfLetValue::App(func, args) => {
                let func_expr = self.compile_arg(func);
                let lua_args: Vec<_> = args.iter().map(|a| self.compile_arg(a)).collect();
                LuaExpr::Call {
                    func: Box::new(func_expr),
                    args: lua_args,
                }
            }
            LcnfLetValue::Ctor(ctor_name, _tag, fields) => {
                let mut all_fields = vec![LuaTableField::NamedField(
                    "tag".to_string(),
                    LuaExpr::Str(ctor_name.clone()),
                )];
                for f in fields {
                    all_fields.push(LuaTableField::ArrayItem(self.compile_arg(f)));
                }
                LuaExpr::TableConstructor(all_fields)
            }
            LcnfLetValue::Proj(_name, index, var) => {
                let val_expr = LuaExpr::Var(var.to_string());
                LuaExpr::IndexAccess {
                    table: Box::new(val_expr),
                    key: Box::new(LuaExpr::Int(*index as i64 + 2)),
                }
            }
            LcnfLetValue::Lit(lit) => Self::compile_lit(lit),
            LcnfLetValue::Erased => LuaExpr::Nil,
            LcnfLetValue::FVar(id) => LuaExpr::Var(id.to_string()),
            LcnfLetValue::Reset(_) => LuaExpr::Nil,
            LcnfLetValue::Reuse(_slot, ctor_name, _tag, fields) => {
                let mut all_fields = vec![LuaTableField::NamedField(
                    "tag".to_string(),
                    LuaExpr::Str(ctor_name.clone()),
                )];
                for f in fields {
                    all_fields.push(LuaTableField::ArrayItem(self.compile_arg(f)));
                }
                LuaExpr::TableConstructor(all_fields)
            }
        }
    }
    /// Compile an LCNF argument to a Lua expression.
    pub fn compile_arg(&mut self, arg: &LcnfArg) -> LuaExpr {
        match arg {
            LcnfArg::Var(id) => LuaExpr::Var(id.to_string()),
            LcnfArg::Lit(lit) => Self::compile_lit(lit),
            LcnfArg::Erased => LuaExpr::Nil,
            LcnfArg::Type(_) => LuaExpr::Nil,
        }
    }
    /// Compile an LCNF expression into a list of Lua statements,
    /// returning the result expression.
    pub fn compile_expr(&mut self, expr: &LcnfExpr, stmts: &mut Vec<LuaStmt>) -> LuaExpr {
        match expr {
            LcnfExpr::Return(arg) => self.compile_arg(arg),
            LcnfExpr::Let {
                id,
                ty: _,
                value,
                body,
                ..
            } => {
                let val_expr = self.compile_let_value(value);
                stmts.push(LuaStmt::LocalAssign {
                    names: vec![id.to_string()],
                    attribs: vec![None],
                    values: vec![val_expr],
                });
                self.compile_expr(body, stmts)
            }
            LcnfExpr::TailCall(func, args) => {
                let func_expr = self.compile_arg(func);
                let lua_args: Vec<_> = args.iter().map(|a| self.compile_arg(a)).collect();
                LuaExpr::Call {
                    func: Box::new(func_expr),
                    args: lua_args,
                }
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scrut_expr = LuaExpr::Var(scrutinee.to_string());
                let result_var = self.fresh_var();
                stmts.push(LuaStmt::LocalAssign {
                    names: vec![result_var.clone()],
                    attribs: vec![None],
                    values: vec![],
                });
                let tag_expr = LuaExpr::FieldAccess {
                    table: Box::new(scrut_expr.clone()),
                    field: "tag".to_string(),
                };
                let mut if_cond: Option<LuaExpr> = None;
                let mut then_stmts: Vec<LuaStmt> = Vec::new();
                let mut elseif_clauses: Vec<(LuaExpr, Vec<LuaStmt>)> = Vec::new();
                for (idx, alt) in alts.iter().enumerate() {
                    let mut case_stmts: Vec<LuaStmt> = Vec::new();
                    for (field_idx, param) in alt.params.iter().enumerate() {
                        let field_access = LuaExpr::IndexAccess {
                            table: Box::new(scrut_expr.clone()),
                            key: Box::new(LuaExpr::Int(field_idx as i64 + 2)),
                        };
                        case_stmts.push(LuaStmt::LocalAssign {
                            names: vec![param.id.to_string()],
                            attribs: vec![None],
                            values: vec![field_access],
                        });
                    }
                    let case_result = self.compile_expr(&alt.body, &mut case_stmts);
                    case_stmts.push(LuaStmt::Assign {
                        targets: vec![LuaExpr::Var(result_var.clone())],
                        values: vec![case_result],
                    });
                    let cond = LuaExpr::BinOp {
                        op: "==".to_string(),
                        lhs: Box::new(tag_expr.clone()),
                        rhs: Box::new(LuaExpr::Str(alt.ctor_name.clone())),
                    };
                    if idx == 0 {
                        if_cond = Some(cond);
                        then_stmts = case_stmts;
                    } else {
                        elseif_clauses.push((cond, case_stmts));
                    }
                }
                let else_body = if let Some(def) = default {
                    let mut def_stmts: Vec<LuaStmt> = Vec::new();
                    let def_result = self.compile_expr(def, &mut def_stmts);
                    def_stmts.push(LuaStmt::Assign {
                        targets: vec![LuaExpr::Var(result_var.clone())],
                        values: vec![def_result],
                    });
                    Some(def_stmts)
                } else {
                    None
                };
                if let Some(cond) = if_cond {
                    stmts.push(LuaStmt::If {
                        cond,
                        then_body: then_stmts,
                        elseif_clauses,
                        else_body,
                    });
                }
                LuaExpr::Var(result_var)
            }
            LcnfExpr::Unreachable => LuaExpr::Call {
                func: Box::new(LuaExpr::Var("error".to_string())),
                args: vec![LuaExpr::Str("unreachable".to_string())],
            },
        }
    }
    /// Compile an LCNF function declaration to a `LuaFunction`.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) -> Result<LuaFunction, std::string::String> {
        let lua_name = self.mangle_name(&decl.name);
        let params: Vec<_> = decl.params.iter().map(|p| p.id.to_string()).collect();
        let mut body_stmts: Vec<LuaStmt> = Vec::new();
        let result_expr = self.compile_expr(&decl.body, &mut body_stmts);
        body_stmts.push(LuaStmt::Return(vec![result_expr]));
        Ok(LuaFunction {
            name: Some(lua_name),
            params,
            vararg: false,
            body: body_stmts,
            is_local: false,
            is_method: false,
        })
    }
    /// Compile a list of declarations and emit a `LuaModule`.
    pub fn emit_module(&mut self, decls: &[LcnfFunDecl]) -> LuaModule {
        let mut module = LuaModule::new();
        for decl in decls {
            if let Ok(func) = self.compile_decl(decl) {
                module.functions.push(func);
            }
        }
        module
    }
}
impl LuaExtConfig {
    pub fn new() -> Self {
        LuaExtConfig::default()
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
