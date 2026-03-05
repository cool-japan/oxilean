//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::functions::*;
use std::collections::VecDeque;

/// Statistics for the specialization pass
#[derive(Debug, Clone, Default)]
pub struct SpecializationStats {
    /// Number of type-based specializations created
    pub type_specializations: usize,
    /// Number of constant-based specializations created
    pub const_specializations: usize,
    /// Number of closure specializations created
    pub closure_specializations: usize,
    /// Number of call sites redirected to specializations
    pub call_sites_redirected: usize,
    /// Total code growth (in instructions)
    pub total_code_growth: usize,
    /// Number of specialization opportunities skipped due to budget
    pub skipped_budget: usize,
    /// Number of specialization opportunities skipped due to limits
    pub skipped_limit: usize,
    /// Number of functions analyzed
    pub functions_analyzed: usize,
}
#[allow(dead_code)]
pub struct SpecPassRegistry {
    pub(super) configs: Vec<SpecPassConfig>,
    pub(super) stats: std::collections::HashMap<String, SpecPassStats>,
}
impl SpecPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SpecPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: SpecPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), SpecPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&SpecPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&SpecPassStats> {
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
/// Configuration for the specialization pass
#[derive(Debug, Clone)]
pub struct SpecializationConfig {
    /// Maximum number of specializations per function
    pub max_specializations: usize,
    /// Whether to specialize functions that receive closures
    pub specialize_closures: bool,
    /// Whether to specialize numeric operations (Nat -> u64, etc.)
    pub specialize_numerics: bool,
    /// Maximum function size to consider for specialization (in instructions)
    pub size_threshold: usize,
    /// Maximum total code growth factor (1.0 = no growth, 2.0 = double)
    pub growth_factor: f64,
    /// Whether to handle recursive specializations
    pub allow_recursive: bool,
    /// Whether to specialize on type parameters
    pub specialize_type_params: bool,
    /// Maximum depth for recursive specialization
    pub max_recursive_depth: usize,
}
/// A specialized version of a function
#[derive(Debug, Clone)]
pub struct SpecializedDecl {
    /// The specialization key
    pub key: SpecializationKey,
    /// The specialized function declaration
    pub decl: LcnfFunDecl,
    /// How much code growth this specialization added (in instructions)
    pub code_growth: usize,
    /// Whether this was created from a recursive function
    pub from_recursive: bool,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// A closure argument in a specialization key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecClosureArg {
    /// The name of the known function being passed
    pub known_fn: Option<String>,
    /// Parameter index
    pub param_idx: usize,
}
/// Main specialization pass
pub struct SpecializationPass {
    pub(super) config: SpecializationConfig,
    pub(super) cache: SpecializationCache,
    pub(super) stats: SpecializationStats,
    pub(super) next_id: u64,
    /// Map from variable IDs to known function names
    pub(super) var_to_fn: HashMap<LcnfVarId, String>,
    /// Original sizes of functions for growth budget
    pub(super) original_sizes: HashMap<String, usize>,
}
impl SpecializationPass {
    /// Create a new specialization pass
    pub fn new(config: SpecializationConfig) -> Self {
        SpecializationPass {
            config,
            cache: SpecializationCache::new(),
            stats: SpecializationStats::default(),
            next_id: 5000,
            var_to_fn: HashMap::new(),
            original_sizes: HashMap::new(),
        }
    }
    /// Get the optimization statistics
    pub fn stats(&self) -> &SpecializationStats {
        &self.stats
    }
    /// Generate a fresh variable ID
    pub(super) fn fresh_id(&mut self) -> LcnfVarId {
        let id = self.next_id;
        self.next_id += 1;
        LcnfVarId(id)
    }
    /// Run the specialization pass on a module
    pub(super) fn run(&mut self, module: &mut LcnfModule) {
        for decl in &module.fun_decls {
            let size = count_instructions(&decl.body);
            self.original_sizes.insert(decl.name.clone(), size);
        }
        let decl_names: HashSet<String> = module.fun_decls.iter().map(|d| d.name.clone()).collect();
        let mut all_sites: Vec<(String, Vec<SpecCallSite>)> = Vec::new();
        for decl in &module.fun_decls {
            self.stats.functions_analyzed += 1;
            let known_consts: HashMap<LcnfVarId, LcnfLit> = HashMap::new();
            let sites =
                find_specialization_sites(&decl.body, &known_consts, &self.var_to_fn, &decl_names);
            if !sites.is_empty() {
                all_sites.push((decl.name.clone(), sites));
            }
        }
        let mut new_decls: Vec<LcnfFunDecl> = Vec::new();
        let total_original_size: usize = self.original_sizes.values().sum();
        let budget = (total_original_size as f64 * self.config.growth_factor) as usize;
        for (_caller, sites) in &all_sites {
            for site in sites {
                if self.cache.total_growth >= budget {
                    self.stats.skipped_budget += 1;
                    continue;
                }
                if self.cache.specialization_count(&site.callee) >= self.config.max_specializations
                {
                    self.stats.skipped_limit += 1;
                    continue;
                }
                let key = self.build_key(site);
                if key.is_trivial() {
                    continue;
                }
                if self.cache.lookup(&key).is_some() {
                    continue;
                }
                if let Some(original) = module.fun_decls.iter().find(|d| d.name == site.callee) {
                    let original_size = count_instructions(&original.body);
                    if original_size > self.config.size_threshold {
                        self.stats.skipped_budget += 1;
                        continue;
                    }
                    if let Some(spec_decl) = self.create_specialization(original, &key) {
                        let growth = count_instructions(&spec_decl.decl.body);
                        self.cache
                            .insert(key.clone(), spec_decl.decl.name.clone(), growth);
                        self.update_stats(&key);
                        new_decls.push(spec_decl.decl);
                    }
                }
            }
        }
        module.fun_decls.extend(new_decls);
        self.redirect_call_sites(module);
    }
    /// Build a specialization key from a call site
    pub(super) fn build_key(&self, site: &SpecCallSite) -> SpecializationKey {
        let type_args = if site.type_args.is_empty() {
            vec![]
        } else {
            site.type_args.clone()
        };
        SpecializationKey {
            original: site.callee.clone(),
            type_args,
            const_args: site.const_args.clone(),
            closure_args: site.closure_args.clone(),
        }
    }
    /// Create a specialized version of a function
    pub(super) fn create_specialization(
        &mut self,
        original: &LcnfFunDecl,
        key: &SpecializationKey,
    ) -> Option<SpecializedDecl> {
        if !self.config.allow_recursive && original.is_recursive {
            return None;
        }
        let spec_name = key.mangled_name();
        let mut spec_body = original.body.clone();
        let mut spec_params = original.params.clone();
        for (i, ca) in key.const_args.iter().enumerate() {
            match ca {
                SpecConstArg::Nat(n) => {
                    if i < spec_params.len() {
                        let param_id = spec_params[i].id;
                        self.substitute_constant(&mut spec_body, param_id, &LcnfLit::Nat(*n));
                        spec_params[i].erased = true;
                    }
                }
                SpecConstArg::Str(s) => {
                    if i < spec_params.len() {
                        let param_id = spec_params[i].id;
                        self.substitute_constant(
                            &mut spec_body,
                            param_id,
                            &LcnfLit::Str(s.clone()),
                        );
                        spec_params[i].erased = true;
                    }
                }
                SpecConstArg::Unknown => {}
            }
        }
        for (i, ta) in key.type_args.iter().enumerate() {
            if let SpecTypeArg::Concrete(ty) = ta {
                if i < spec_params.len() {
                    spec_params[i].ty = ty.clone();
                }
            }
        }
        let active_params: Vec<LcnfParam> = spec_params.into_iter().filter(|p| !p.erased).collect();
        let spec_decl = LcnfFunDecl {
            name: spec_name.clone(),
            original_name: original.original_name.clone(),
            params: active_params,
            ret_type: original.ret_type.clone(),
            body: spec_body,
            is_recursive: original.is_recursive,
            is_lifted: true,
            inline_cost: original.inline_cost,
        };
        let code_growth = count_instructions(&spec_decl.body);
        Some(SpecializedDecl {
            key: key.clone(),
            decl: spec_decl,
            code_growth,
            from_recursive: original.is_recursive,
        })
    }
    /// Substitute a constant value for a variable in an expression
    pub(super) fn substitute_constant(&self, expr: &mut LcnfExpr, var: LcnfVarId, lit: &LcnfLit) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                self.substitute_constant_in_value(value, var, lit);
                if *id != var {
                    self.substitute_constant(body, var, lit);
                }
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                if *scrutinee == var {}
                for alt in alts.iter_mut() {
                    let shadows = alt.params.iter().any(|p| p.id == var);
                    if !shadows {
                        self.substitute_constant(&mut alt.body, var, lit);
                    }
                }
                if let Some(def) = default {
                    self.substitute_constant(def, var, lit);
                }
            }
            LcnfExpr::Return(arg) => {
                self.substitute_arg(arg, var, lit);
            }
            LcnfExpr::TailCall(func, args) => {
                self.substitute_arg(func, var, lit);
                for a in args.iter_mut() {
                    self.substitute_arg(a, var, lit);
                }
            }
            LcnfExpr::Unreachable => {}
        }
    }
    /// Substitute in a let-value
    pub(super) fn substitute_constant_in_value(
        &self,
        value: &mut LcnfLetValue,
        var: LcnfVarId,
        lit: &LcnfLit,
    ) {
        match value {
            LcnfLetValue::App(func, args) => {
                self.substitute_arg(func, var, lit);
                for a in args.iter_mut() {
                    self.substitute_arg(a, var, lit);
                }
            }
            LcnfLetValue::Proj(_, _, obj) => if *obj == var {},
            LcnfLetValue::Ctor(_, _, args) => {
                for a in args.iter_mut() {
                    self.substitute_arg(a, var, lit);
                }
            }
            LcnfLetValue::FVar(v) => {
                if *v == var {
                    *value = LcnfLetValue::Lit(lit.clone());
                }
            }
            LcnfLetValue::Lit(_)
            | LcnfLetValue::Erased
            | LcnfLetValue::Reset(_)
            | LcnfLetValue::Reuse(_, _, _, _) => {}
        }
    }
    /// Substitute a variable reference in an argument
    pub(super) fn substitute_arg(&self, arg: &mut LcnfArg, var: LcnfVarId, lit: &LcnfLit) {
        if let LcnfArg::Var(v) = arg {
            if *v == var {
                *arg = LcnfArg::Lit(lit.clone());
            }
        }
    }
    /// Update statistics based on the specialization key
    pub(super) fn update_stats(&mut self, key: &SpecializationKey) {
        let has_type = key
            .type_args
            .iter()
            .any(|a| matches!(a, SpecTypeArg::Concrete(_)));
        let has_const = key
            .const_args
            .iter()
            .any(|a| !matches!(a, SpecConstArg::Unknown));
        let has_closure = key.closure_args.iter().any(|a| a.known_fn.is_some());
        if has_type {
            self.stats.type_specializations += 1;
        }
        if has_const {
            self.stats.const_specializations += 1;
        }
        if has_closure {
            self.stats.closure_specializations += 1;
        }
    }
    /// Redirect call sites to their specialized versions
    pub(super) fn redirect_call_sites(&mut self, module: &mut LcnfModule) {
        let redirects: HashMap<SpecializationKey, String> = self.cache.entries.clone();
        if redirects.is_empty() {
            return;
        }
        for decl in &mut module.fun_decls {
            let redirected = self.redirect_in_expr(&mut decl.body, &redirects);
            self.stats.call_sites_redirected += redirected;
        }
    }
    /// Redirect call sites in an expression
    pub(super) fn redirect_in_expr(
        &self,
        expr: &mut LcnfExpr,
        redirects: &HashMap<SpecializationKey, String>,
    ) -> usize {
        let mut count = 0;
        let mut local_consts: HashMap<LcnfVarId, LcnfLit> = HashMap::new();
        let mut local_fn_map: HashMap<LcnfVarId, String> = self.var_to_fn.clone();
        self.redirect_inner(
            expr,
            redirects,
            &mut local_consts,
            &mut local_fn_map,
            &mut count,
        );
        count
    }
    /// Inner recursive helper for call-site redirection.
    pub(super) fn redirect_inner(
        &self,
        expr: &mut LcnfExpr,
        redirects: &HashMap<SpecializationKey, String>,
        local_consts: &mut HashMap<LcnfVarId, LcnfLit>,
        local_fn_map: &mut HashMap<LcnfVarId, String>,
        count: &mut usize,
    ) {
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                if let LcnfLetValue::App(func, args) = value {
                    if let LcnfArg::Var(v) = func {
                        if let Some(fn_name) = local_fn_map.get(v).cloned() {
                            let const_args = Self::build_const_args(args, local_consts);
                            let closure_args = Self::build_closure_args(args, local_fn_map);
                            for (key, spec_name) in redirects {
                                if key.original == fn_name
                                    && key.const_args == const_args
                                    && (key.closure_args.is_empty()
                                        || key.closure_args == closure_args)
                                {
                                    if let Some(spec_var) = local_fn_map
                                        .iter()
                                        .find(|(_, name)| *name == spec_name)
                                        .map(|(var, _)| *var)
                                    {
                                        *func = LcnfArg::Var(spec_var);
                                        *count += 1;
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }
                if let LcnfLetValue::Lit(lit) = value {
                    local_consts.insert(*id, lit.clone());
                }
                if let LcnfLetValue::FVar(v) = value {
                    if let Some(fn_name) = local_fn_map.get(v).cloned() {
                        local_fn_map.insert(*id, fn_name);
                    }
                }
                self.redirect_inner(body, redirects, local_consts, local_fn_map, count);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    let mut alt_consts = local_consts.clone();
                    let mut alt_fn_map = local_fn_map.clone();
                    self.redirect_inner(
                        &mut alt.body,
                        redirects,
                        &mut alt_consts,
                        &mut alt_fn_map,
                        count,
                    );
                }
                if let Some(def) = default {
                    let mut def_consts = local_consts.clone();
                    let mut def_fn_map = local_fn_map.clone();
                    self.redirect_inner(def, redirects, &mut def_consts, &mut def_fn_map, count);
                }
            }
            LcnfExpr::TailCall(func, args) => {
                if let LcnfArg::Var(v) = func {
                    if let Some(fn_name) = local_fn_map.get(v).cloned() {
                        let const_args = Self::build_const_args(args, local_consts);
                        let closure_args = Self::build_closure_args(args, local_fn_map);
                        for (key, spec_name) in redirects {
                            if key.original == fn_name
                                && key.const_args == const_args
                                && (key.closure_args.is_empty() || key.closure_args == closure_args)
                            {
                                if let Some(spec_var) = local_fn_map
                                    .iter()
                                    .find(|(_, name)| *name == spec_name)
                                    .map(|(var, _)| *var)
                                {
                                    *func = LcnfArg::Var(spec_var);
                                    *count += 1;
                                }
                                break;
                            }
                        }
                    }
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable => {}
        }
    }
    /// Build SpecConstArg list from call-site arguments.
    pub(super) fn build_const_args(
        args: &[LcnfArg],
        local_consts: &HashMap<LcnfVarId, LcnfLit>,
    ) -> Vec<SpecConstArg> {
        args.iter()
            .map(|arg| match arg {
                LcnfArg::Lit(LcnfLit::Nat(n)) => SpecConstArg::Nat(*n),
                LcnfArg::Lit(LcnfLit::Str(s)) => SpecConstArg::Str(s.clone()),
                LcnfArg::Var(v) => match local_consts.get(v) {
                    Some(LcnfLit::Nat(n)) => SpecConstArg::Nat(*n),
                    Some(LcnfLit::Str(s)) => SpecConstArg::Str(s.clone()),
                    None => SpecConstArg::Unknown,
                },
                _ => SpecConstArg::Unknown,
            })
            .collect()
    }
    /// Build SpecClosureArg list from call-site arguments.
    pub(super) fn build_closure_args(
        args: &[LcnfArg],
        local_fn_map: &HashMap<LcnfVarId, String>,
    ) -> Vec<SpecClosureArg> {
        args.iter()
            .enumerate()
            .map(|(i, arg)| SpecClosureArg {
                known_fn: match arg {
                    LcnfArg::Var(v) => local_fn_map.get(v).cloned(),
                    _ => None,
                },
                param_idx: i,
            })
            .collect()
    }
}
/// Pass registry for SpecExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct SpecExtPassRegistry {
    pub(super) configs: Vec<SpecExtPassConfig>,
    pub(super) stats: Vec<SpecExtPassStats>,
}
impl SpecExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: SpecExtPassConfig) {
        self.stats.push(SpecExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&SpecExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&SpecExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&SpecExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &SpecExtPassPhase) -> Vec<&SpecExtPassConfig> {
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
/// Analysis cache for SpecExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct SpecExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl SpecExtCache {
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
/// Dependency graph for SpecExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl SpecExtDepGraph {
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
/// Dominator tree for SpecExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl SpecExtDomTree {
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
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SpecPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl SpecPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            SpecPassPhase::Analysis => "analysis",
            SpecPassPhase::Transformation => "transformation",
            SpecPassPhase::Verification => "verification",
            SpecPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, SpecPassPhase::Transformation | SpecPassPhase::Cleanup)
    }
}
/// Pass execution phase for SpecExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpecExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl SpecExtPassPhase {
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
/// Constant folding helper for SpecExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SpecExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl SpecExtConstFolder {
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
#[derive(Debug, Clone, Default)]
pub struct SpecPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl SpecPassStats {
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
pub struct SpecConstantFoldingHelper;
impl SpecConstantFoldingHelper {
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
/// A constant argument in a specialization key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpecConstArg {
    /// Known natural number
    Nat(u64),
    /// Known string
    Str(String),
    /// Not a known constant
    Unknown,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl SpecDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        SpecDominatorTree {
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
pub struct SpecPassConfig {
    pub phase: SpecPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl SpecPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: SpecPassPhase) -> Self {
        SpecPassConfig {
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
pub struct SpecLivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl SpecLivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        SpecLivenessInfo {
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
/// Configuration for SpecExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecExtPassConfig {
    pub name: String,
    pub phase: SpecExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl SpecExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: SpecExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: SpecExtPassPhase) -> Self {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl SpecWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SpecWorklist {
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
/// Worklist for SpecExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl SpecExtWorklist {
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
/// Cache for specializations to avoid duplicates
#[derive(Debug, Clone, Default)]
pub struct SpecializationCache {
    /// Map from specialization key to specialized function name
    pub(super) entries: HashMap<SpecializationKey, String>,
    /// Total code growth from all specializations
    pub(super) total_growth: usize,
}
impl SpecializationCache {
    pub(super) fn new() -> Self {
        SpecializationCache {
            entries: HashMap::new(),
            total_growth: 0,
        }
    }
    pub(super) fn lookup(&self, key: &SpecializationKey) -> Option<&str> {
        self.entries.get(key).map(|s| s.as_str())
    }
    pub(super) fn insert(&mut self, key: SpecializationKey, name: String, growth: usize) {
        self.entries.insert(key, name);
        self.total_growth += growth;
    }
    pub(super) fn specialization_count(&self, original: &str) -> usize {
        self.entries
            .keys()
            .filter(|k| k.original == original)
            .count()
    }
}
/// Track code size budget for specialization
#[derive(Debug, Clone)]
pub struct SizeBudget {
    pub(super) original_total: usize,
    pub(super) max_total: usize,
    pub(super) current_growth: usize,
}
impl SizeBudget {
    pub(super) fn new(original_total: usize, growth_factor: f64) -> Self {
        SizeBudget {
            original_total,
            max_total: (original_total as f64 * growth_factor) as usize,
            current_growth: 0,
        }
    }
    pub(super) fn can_afford(&self, additional: usize) -> bool {
        self.original_total + self.current_growth + additional <= self.max_total
    }
    pub(super) fn spend(&mut self, amount: usize) {
        self.current_growth += amount;
    }
    pub(super) fn remaining(&self) -> usize {
        self.max_total
            .saturating_sub(self.original_total + self.current_growth)
    }
}
/// A type argument in a specialization key
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpecTypeArg {
    /// Concrete type
    Concrete(LcnfType),
    /// Still polymorphic
    Poly,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl SpecDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        SpecDepGraph {
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
/// Numeric type specialization: replace polymorphic Nat operations with u64
pub struct NumericSpecializer {
    /// Known numeric functions that can be specialized
    pub(super) numeric_ops: HashSet<String>,
}
impl NumericSpecializer {
    pub(super) fn new() -> Self {
        let mut ops = HashSet::new();
        ops.insert("Nat.add".to_string());
        ops.insert("Nat.sub".to_string());
        ops.insert("Nat.mul".to_string());
        ops.insert("Nat.div".to_string());
        ops.insert("Nat.mod".to_string());
        ops.insert("Nat.beq".to_string());
        ops.insert("Nat.ble".to_string());
        ops.insert("Nat.blt".to_string());
        ops.insert("Nat.decEq".to_string());
        ops.insert("Nat.zero".to_string());
        ops.insert("Nat.succ".to_string());
        NumericSpecializer { numeric_ops: ops }
    }
    pub(super) fn is_numeric_op(&self, name: &str) -> bool {
        self.numeric_ops.contains(name)
    }
    pub(super) fn specialize_nat_to_u64(&self, ty: &LcnfType) -> LcnfType {
        match ty {
            LcnfType::Nat => LcnfType::Nat,
            LcnfType::Fun(params, ret) => {
                let new_params: Vec<LcnfType> = params
                    .iter()
                    .map(|p| self.specialize_nat_to_u64(p))
                    .collect();
                let new_ret = self.specialize_nat_to_u64(ret);
                LcnfType::Fun(new_params, Box::new(new_ret))
            }
            LcnfType::Ctor(name, args) => {
                let new_args: Vec<LcnfType> =
                    args.iter().map(|a| self.specialize_nat_to_u64(a)).collect();
                LcnfType::Ctor(name.clone(), new_args)
            }
            other => other.clone(),
        }
    }
}
/// What makes a specialization unique
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecializationKey {
    /// The original function name
    pub original: String,
    /// Type arguments that are specialized
    pub type_args: Vec<SpecTypeArg>,
    /// Constant arguments that are specialized
    pub const_args: Vec<SpecConstArg>,
    /// Closure arguments that are specialized
    pub closure_args: Vec<SpecClosureArg>,
}
impl SpecializationKey {
    pub(super) fn is_trivial(&self) -> bool {
        self.type_args
            .iter()
            .all(|a| matches!(a, SpecTypeArg::Poly))
            && self
                .const_args
                .iter()
                .all(|a| matches!(a, SpecConstArg::Unknown))
            && self.closure_args.iter().all(|a| a.known_fn.is_none())
    }
    pub(super) fn mangled_name(&self) -> String {
        let mut name = self.original.clone();
        for (i, ta) in self.type_args.iter().enumerate() {
            if let SpecTypeArg::Concrete(ty) = ta {
                name.push_str(&format!("_T{}_{}", i, type_suffix(ty)));
            }
        }
        for (i, ca) in self.const_args.iter().enumerate() {
            match ca {
                SpecConstArg::Nat(n) => name.push_str(&format!("_C{}_N{}", i, n)),
                SpecConstArg::Str(s) => {
                    let short = if s.len() > 8 { &s[..8] } else { s.as_str() };
                    name.push_str(&format!("_C{}_S{}", i, short));
                }
                SpecConstArg::Unknown => {}
            }
        }
        for ca in &self.closure_args {
            if let Some(fn_name) = &ca.known_fn {
                name.push_str(&format!("_F{}", fn_name));
            }
        }
        name
    }
}
/// Liveness analysis for SpecExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SpecExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl SpecExtLiveness {
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
/// Statistics for SpecExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SpecExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl SpecExtPassStats {
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
    pub fn merge(&mut self, o: &SpecExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Information about a call site that might benefit from specialization
#[derive(Debug, Clone)]
pub struct SpecCallSite {
    /// The called function name
    pub(super) callee: String,
    /// Index of this call site in the function
    pub(super) call_idx: usize,
    /// Type arguments at this call site (if determinable)
    pub(super) type_args: Vec<SpecTypeArg>,
    /// Constant arguments at this call site
    pub(super) const_args: Vec<SpecConstArg>,
    /// Closure arguments at this call site
    pub(super) closure_args: Vec<SpecClosureArg>,
    /// The variable ID used for the call
    pub(super) callee_var: Option<LcnfVarId>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SpecAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, SpecCacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl SpecAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        SpecAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&SpecCacheEntry> {
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
            SpecCacheEntry {
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
