//! Implementation blocks (part 2)

use super::super::functions::KOTLIN_KEYWORDS;
use super::super::functions::*;
use super::defs::*;
use crate::lcnf::*;
use std::collections::HashSet;
use std::collections::{HashMap, VecDeque};

impl KotlinExtWorklist {
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
impl KotlinX2Cache {
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
impl KotlinBackend {
    /// Create a new `KotlinBackend`.
    pub fn new() -> Self {
        KotlinBackend { var_counter: 0 }
    }
    /// Mangle a name so it does not clash with Kotlin keywords.
    pub fn mangle_name(&self, name: &str) -> String {
        let sanitized: String = name
            .chars()
            .map(|c| match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' => c,
                '.' | ':' | '\'' | '!' | '?' => '_',
                _ => '_',
            })
            .collect();
        let sanitized = if sanitized.starts_with(|c: char| c.is_ascii_digit()) {
            format!("_{}", sanitized)
        } else {
            sanitized
        };
        if KOTLIN_KEYWORDS.contains(&sanitized.as_str()) {
            format!("{}_", sanitized)
        } else if sanitized.is_empty() {
            "_anon".to_string()
        } else {
            sanitized
        }
    }
    /// Generate a fresh temporary variable name.
    pub fn fresh_var(&mut self) -> String {
        let v = self.var_counter;
        self.var_counter += 1;
        format!("_t{}", v)
    }
    /// Compile a slice of LCNF function declarations into a Kotlin source string.
    pub fn compile_module(decls: &[LcnfFunDecl]) -> Result<String, String> {
        let mut backend = KotlinBackend::new();
        let mut funs = Vec::new();
        let mut ctor_names: HashSet<String> = HashSet::new();
        for decl in decls {
            collect_ctor_names_from_expr(&decl.body, &mut ctor_names);
        }
        for decl in decls {
            let f = backend.compile_decl(decl)?;
            funs.push(f);
        }
        let data_classes: Vec<KotlinDataClass> = ctor_names
            .into_iter()
            .collect::<Vec<_>>()
            .into_iter()
            .map(|name| KotlinDataClass {
                fields: vec![("tag".to_string(), KotlinType::KtInt)],
                name,
            })
            .collect();
        let module = KotlinModule {
            package: "oxilean.generated".to_string(),
            imports: vec![],
            data_classes,
            funs,
        };
        Ok(module.to_string())
    }
    /// Compile a single LCNF function declaration to a `KotlinFunc`.
    pub fn compile_decl(&mut self, decl: &LcnfFunDecl) -> Result<KotlinFunc, String> {
        let name = self.mangle_name(&decl.name.to_string());
        let params: Vec<(String, KotlinType)> = decl
            .params
            .iter()
            .map(|p| (self.mangle_name(&p.name), lcnf_type_to_kotlin(&p.ty)))
            .collect();
        let return_type = lcnf_type_to_kotlin(&decl.ret_type);
        let mut stmts: Vec<KotlinStmt> = Vec::new();
        let result_expr = self.compile_expr(&decl.body, &mut stmts)?;
        stmts.push(KotlinStmt::Return(result_expr));
        Ok(KotlinFunc {
            name,
            params,
            return_type,
            body: stmts,
            is_tailrec: false,
        })
    }
    /// Compile an LCNF expression, appending any necessary binding statements
    /// into `stmts`, and returning the resulting Kotlin expression.
    pub fn compile_expr(
        &mut self,
        expr: &LcnfExpr,
        stmts: &mut Vec<KotlinStmt>,
    ) -> Result<KotlinExpr, String> {
        match expr {
            LcnfExpr::Return(arg) => Ok(self.compile_arg(arg)),
            LcnfExpr::Unreachable => Ok(KotlinExpr::Call(
                Box::new(KotlinExpr::Member(
                    Box::new(KotlinExpr::Var("OxiLeanRuntime".to_string())),
                    "unreachable".to_string(),
                )),
                vec![],
            )),
            LcnfExpr::TailCall(func, args) => {
                let callee = self.compile_arg(func);
                let kt_args: Vec<KotlinExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(KotlinExpr::Call(Box::new(callee), kt_args))
            }
            LcnfExpr::Let {
                id: _,
                name,
                ty,
                value,
                body,
            } => {
                let kt_val = self.compile_let_value(value)?;
                let var_name = self.mangle_name(name);
                let kt_ty = lcnf_type_to_kotlin(ty);
                stmts.push(KotlinStmt::Val(var_name.clone(), kt_ty, kt_val));
                self.compile_expr(body, stmts)
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty: _,
                alts,
                default,
            } => {
                let scrutinee_expr = KotlinExpr::Var(format!("_x{}", scrutinee.0));
                let result_var = self.fresh_var();
                let mut branches: Vec<(KotlinExpr, Vec<KotlinStmt>)> = Vec::new();
                for alt in alts {
                    let mut branch_stmts: Vec<KotlinStmt> = Vec::new();
                    for (idx, param) in alt.params.iter().enumerate() {
                        let param_name = self.mangle_name(&param.name);
                        let field_access = KotlinExpr::Member(
                            Box::new(KotlinExpr::Var(format!("_x{}", scrutinee.0))),
                            format!("field{}", idx),
                        );
                        branch_stmts.push(KotlinStmt::Val(
                            param_name,
                            lcnf_type_to_kotlin(&param.ty),
                            field_access,
                        ));
                    }
                    let branch_result = self.compile_expr(&alt.body, &mut branch_stmts)?;
                    branch_stmts.push(KotlinStmt::Assign(result_var.clone(), branch_result));
                    let tag_cond = KotlinExpr::Lit(KotlinLit::Int(alt.ctor_tag as i64));
                    branches.push((tag_cond, branch_stmts));
                }
                let mut default_stmts: Vec<KotlinStmt> = Vec::new();
                if let Some(def) = default {
                    let def_result = self.compile_expr(def, &mut default_stmts)?;
                    default_stmts.push(KotlinStmt::Assign(result_var.clone(), def_result));
                } else {
                    default_stmts.push(KotlinStmt::Expr(KotlinExpr::Call(
                        Box::new(KotlinExpr::Member(
                            Box::new(KotlinExpr::Var("OxiLeanRuntime".to_string())),
                            "unreachable".to_string(),
                        )),
                        vec![],
                    )));
                }
                let discriminant = KotlinExpr::Member(Box::new(scrutinee_expr), "tag".to_string());
                stmts.push(KotlinStmt::Var(
                    result_var.clone(),
                    KotlinType::KtAny,
                    KotlinExpr::Lit(KotlinLit::Null),
                ));
                stmts.push(KotlinStmt::When(discriminant, branches, default_stmts));
                Ok(KotlinExpr::Var(result_var))
            }
        }
    }
    /// Compile an LCNF let-value to a Kotlin expression.
    pub(super) fn compile_let_value(&mut self, value: &LcnfLetValue) -> Result<KotlinExpr, String> {
        match value {
            LcnfLetValue::Lit(lit) => Ok(self.compile_lit(lit)),
            LcnfLetValue::Erased => Ok(KotlinExpr::Lit(KotlinLit::Null)),
            LcnfLetValue::FVar(id) => Ok(KotlinExpr::Var(format!("_x{}", id.0))),
            LcnfLetValue::App(func, args) => {
                let callee = self.compile_arg(func);
                let kt_args: Vec<KotlinExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(KotlinExpr::Call(Box::new(callee), kt_args))
            }
            LcnfLetValue::Proj(_name, idx, var) => {
                let base = KotlinExpr::Var(format!("_x{}", var.0));
                Ok(KotlinExpr::Member(Box::new(base), format!("field{}", idx)))
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let ctor_name = self.mangle_name(name);
                let kt_args: Vec<KotlinExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(KotlinExpr::Call(
                    Box::new(KotlinExpr::Var(ctor_name)),
                    kt_args,
                ))
            }
            LcnfLetValue::Reset(_var) => Ok(KotlinExpr::Lit(KotlinLit::Null)),
            LcnfLetValue::Reuse(_slot, name, _tag, args) => {
                let ctor_name = self.mangle_name(name);
                let kt_args: Vec<KotlinExpr> = args.iter().map(|a| self.compile_arg(a)).collect();
                Ok(KotlinExpr::Call(
                    Box::new(KotlinExpr::Var(ctor_name)),
                    kt_args,
                ))
            }
        }
    }
    /// Compile an LCNF argument to a Kotlin expression.
    pub(super) fn compile_arg(&self, arg: &LcnfArg) -> KotlinExpr {
        match arg {
            LcnfArg::Var(id) => KotlinExpr::Var(format!("_x{}", id.0)),
            LcnfArg::Lit(lit) => self.compile_lit_ref(lit),
            LcnfArg::Erased => KotlinExpr::Lit(KotlinLit::Null),
            LcnfArg::Type(_) => KotlinExpr::Lit(KotlinLit::Null),
        }
    }
    /// Compile an LCNF literal (owned).
    pub(super) fn compile_lit(&self, lit: &LcnfLit) -> KotlinExpr {
        match lit {
            LcnfLit::Nat(n) => KotlinExpr::Lit(KotlinLit::Long(*n as i64)),
            LcnfLit::Str(s) => KotlinExpr::Lit(KotlinLit::Str(s.clone())),
        }
    }
    /// Compile an LCNF literal (reference, for use in `compile_arg`).
    pub(super) fn compile_lit_ref(&self, lit: &LcnfLit) -> KotlinExpr {
        self.compile_lit(lit)
    }
}
impl KtDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        KtDominatorTree {
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
impl KotlinX2PassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: KotlinX2PassConfig) {
        self.stats.push(KotlinX2PassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&KotlinX2PassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&KotlinX2PassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&KotlinX2PassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &KotlinX2PassPhase) -> Vec<&KotlinX2PassConfig> {
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
impl KotlinX2PassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: KotlinX2PassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: KotlinX2PassPhase) -> Self {
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
impl KotlinX2DepGraph {
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
impl KotlinX2DomTree {
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
impl KotlinX2Liveness {
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
impl KtPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            KtPassPhase::Analysis => "analysis",
            KtPassPhase::Transformation => "transformation",
            KtPassPhase::Verification => "verification",
            KtPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, KtPassPhase::Transformation | KtPassPhase::Cleanup)
    }
}
impl KotlinX2PassPhase {
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
impl KtWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        KtWorklist {
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
impl KotlinX2ConstFolder {
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
impl KtConstantFoldingHelper {
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
impl KtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        KtPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: KtPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), KtPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&KtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&KtPassStats> {
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
impl KtAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        KtAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&KtCacheEntry> {
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
            KtCacheEntry {
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
