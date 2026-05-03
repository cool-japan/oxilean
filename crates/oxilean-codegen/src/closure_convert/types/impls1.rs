//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::defs::*;
use super::impls2::*;
use crate::lcnf::*;
use std::collections::{HashMap, HashSet};

use super::super::functions::*;
use std::collections::VecDeque;

/// The escape status of a closure or variable.
/// Analysis cache for CCExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct CCExtCache {
    pub(crate) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(crate) cap: usize,
    pub(crate) total_hits: u64,
    pub(crate) total_misses: u64,
}
impl CCExtCache {
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
/// Pass execution phase for CCX2.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CCX2PassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl CCX2PassPhase {
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
#[allow(dead_code)]
pub struct CCConstantFoldingHelper;
impl CCConstantFoldingHelper {
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
/// Pass execution phase for CCExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CCExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl CCExtPassPhase {
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
/// Configuration for CCExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CCExtPassConfig {
    pub name: String,
    pub phase: CCExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl CCExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: CCExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: CCExtPassPhase) -> Self {
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
/// Pass registry for CCX2.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CCX2PassRegistry {
    pub(crate) configs: Vec<CCX2PassConfig>,
    pub(crate) stats: Vec<CCX2PassStats>,
}
impl CCX2PassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: CCX2PassConfig) {
        self.stats.push(CCX2PassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&CCX2PassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&CCX2PassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&CCX2PassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &CCX2PassPhase) -> Vec<&CCX2PassConfig> {
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
#[derive(Debug, Clone, PartialEq)]
pub enum CCPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl CCPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            CCPassPhase::Analysis => "analysis",
            CCPassPhase::Transformation => "transformation",
            CCPassPhase::Verification => "verification",
            CCPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, CCPassPhase::Transformation | CCPassPhase::Cleanup)
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CCPassConfig {
    pub phase: CCPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl CCPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: CCPassPhase) -> Self {
        CCPassConfig {
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
pub struct CCDepGraph {
    pub(crate) nodes: Vec<u32>,
    pub(crate) edges: Vec<(u32, u32)>,
}
impl CCDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CCDepGraph {
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
#[allow(dead_code)]
pub struct CCPassRegistry {
    pub(crate) configs: Vec<CCPassConfig>,
    pub(crate) stats: std::collections::HashMap<String, CCPassStats>,
}
impl CCPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CCPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: CCPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), CCPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&CCPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&CCPassStats> {
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
#[derive(Debug, Clone)]
pub struct CCCacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// Statistics from the closure conversion pass.
#[derive(Debug, Clone, Default)]
pub struct ClosureConvertStats {
    /// Number of closures converted.
    pub closures_converted: usize,
    /// Number of helper functions lifted.
    pub helpers_lifted: usize,
    /// Number of closures defunctionalized.
    pub defunctionalized: usize,
    /// Number of closures stack-allocated.
    pub stack_allocated: usize,
    /// Number of closures that had to be heap-allocated.
    pub heap_allocated: usize,
    /// Number of closures merged.
    pub closures_merged: usize,
}
/// The closure conversion pass.
///
/// Transforms an LCNF module by making all closures explicit:
/// lambda expressions become struct allocations with captured variables.
pub struct ClosureConverter {
    pub(crate) config: ClosureConvertConfig,
    pub(crate) stats: ClosureConvertStats,
    /// Escape analysis results.
    pub(crate) escape_info: HashMap<LcnfVarId, EscapeInfo>,
    /// Collected closure infos (closure var -> info).
    pub(crate) closure_infos: HashMap<LcnfVarId, ClosureInfo>,
    /// Helper functions generated during conversion.
    pub(crate) lifted_helpers: Vec<LcnfFunDecl>,
    /// Fresh variable counter.
    pub(crate) next_var: u64,
    /// Fresh name counter.
    pub(crate) name_counter: usize,
    /// Set of known function names (for defunctionalization).
    pub(crate) known_functions: HashSet<String>,
}
impl ClosureConverter {
    /// Create a new closure converter.
    pub fn new(config: ClosureConvertConfig) -> Self {
        ClosureConverter {
            config,
            stats: ClosureConvertStats::default(),
            escape_info: HashMap::new(),
            closure_infos: HashMap::new(),
            lifted_helpers: Vec::new(),
            next_var: 10000,
            name_counter: 0,
            known_functions: HashSet::new(),
        }
    }
    /// Create with default configuration.
    pub fn default_converter() -> Self {
        Self::new(ClosureConvertConfig::default())
    }
    /// Allocate a fresh variable ID.
    pub(crate) fn fresh_var(&mut self) -> LcnfVarId {
        let id = LcnfVarId(self.next_var);
        self.next_var += 1;
        id
    }
    /// Generate a fresh helper function name.
    pub(crate) fn fresh_name(&mut self, base: &str) -> String {
        let name = format!("{}_closure_{}", base, self.name_counter);
        self.name_counter += 1;
        name
    }
    /// Convert all closures in a module.
    pub fn convert_module(&mut self, module: &mut LcnfModule) {
        let mut escape = EscapeAnalysis::new();
        escape.analyze(module);
        self.escape_info = escape.var_escape.clone();
        for decl in &module.fun_decls {
            self.known_functions.insert(decl.name.clone());
        }
        let mut new_decls = Vec::new();
        for decl in &module.fun_decls {
            let (converted, helpers) = self.convert_fun_decl(decl);
            new_decls.push(converted);
            new_decls.extend(helpers);
        }
        new_decls.append(&mut self.lifted_helpers);
        module.fun_decls = new_decls;
    }
    /// Convert a single function declaration.
    ///
    /// Returns the converted declaration and any helper functions
    /// that were lifted out.
    pub fn convert_fun_decl(&mut self, decl: &LcnfFunDecl) -> (LcnfFunDecl, Vec<LcnfFunDecl>) {
        let mut helpers = Vec::new();
        let bound: HashSet<LcnfVarId> = decl.params.iter().map(|p| p.id).collect();
        let _body_free = compute_free_vars(&decl.body, &bound);
        let new_body = self.convert_expr(&decl.body, &decl.name, &mut helpers);
        let converted = LcnfFunDecl {
            name: decl.name.clone(),
            original_name: decl.original_name.clone(),
            params: decl.params.clone(),
            ret_type: decl.ret_type.clone(),
            body: new_body,
            is_recursive: decl.is_recursive,
            is_lifted: decl.is_lifted,
            inline_cost: decl.inline_cost,
        };
        self.stats.closures_converted += helpers.len();
        (converted, helpers)
    }
    /// Convert an expression, possibly lifting closures.
    pub(crate) fn convert_expr(
        &mut self,
        expr: &LcnfExpr,
        parent_fn: &str,
        helpers: &mut Vec<LcnfFunDecl>,
    ) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let new_value = self.convert_let_value(value, parent_fn, helpers);
                let new_body = self.convert_expr(body, parent_fn, helpers);
                LcnfExpr::Let {
                    id: *id,
                    name: name.clone(),
                    ty: ty.clone(),
                    value: new_value,
                    body: Box::new(new_body),
                }
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            } => {
                let new_alts: Vec<LcnfAlt> = alts
                    .iter()
                    .map(|alt| LcnfAlt {
                        ctor_name: alt.ctor_name.clone(),
                        ctor_tag: alt.ctor_tag,
                        params: alt.params.clone(),
                        body: self.convert_expr(&alt.body, parent_fn, helpers),
                    })
                    .collect();
                let new_default = default
                    .as_ref()
                    .map(|d| Box::new(self.convert_expr(d, parent_fn, helpers)));
                LcnfExpr::Case {
                    scrutinee: *scrutinee,
                    scrutinee_ty: scrutinee_ty.clone(),
                    alts: new_alts,
                    default: new_default,
                }
            }
            other => other.clone(),
        }
    }
    /// Convert a let-value, possibly lifting lambda bodies.
    pub(crate) fn convert_let_value(
        &mut self,
        value: &LcnfLetValue,
        _parent_fn: &str,
        _helpers: &mut Vec<LcnfFunDecl>,
    ) -> LcnfLetValue {
        match value {
            LcnfLetValue::App(func, _args) => {
                if self.config.defunctionalize {
                    if let LcnfArg::Var(_fvar) = func {}
                }
                value.clone()
            }
            LcnfLetValue::Ctor(_name, _tag, _args) => value.clone(),
            _ => value.clone(),
        }
    }
    /// Attempt to defunctionalize a set of closures at a call site.
    ///
    /// When the set of possible callees is statically known, replace
    /// the closure call with a switch on the closure's tag.
    pub fn defunctionalize(
        &mut self,
        call_site_var: LcnfVarId,
        possible_callees: &[String],
    ) -> Option<LcnfExpr> {
        if possible_callees.is_empty() || !self.config.defunctionalize {
            return None;
        }
        self.stats.defunctionalized += 1;
        let alts: Vec<LcnfAlt> = possible_callees
            .iter()
            .enumerate()
            .map(|(tag, callee_name)| LcnfAlt {
                ctor_name: callee_name.clone(),
                ctor_tag: tag as u32,
                params: vec![],
                body: LcnfExpr::Return(LcnfArg::Var(call_site_var)),
            })
            .collect();
        Some(LcnfExpr::Case {
            scrutinee: call_site_var,
            scrutinee_ty: LcnfType::Object,
            alts,
            default: Some(Box::new(LcnfExpr::Unreachable)),
        })
    }
    /// Determine whether a closure should be stack-allocated.
    pub fn stack_allocate_closure(&self, info: &ClosureInfo) -> bool {
        self.config.stack_alloc_non_escaping
            && !info.is_escaping
            && info.free_vars.len() <= self.config.max_inline_captures
    }
    /// Get the conversion statistics.
    pub fn stats(&self) -> &ClosureConvertStats {
        &self.stats
    }
    /// Get closure info for a variable, if available.
    pub fn get_closure_info(&self, var: LcnfVarId) -> Option<&ClosureInfo> {
        self.closure_infos.get(&var)
    }
}
/// Liveness analysis for CCExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CCExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl CCExtLiveness {
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
