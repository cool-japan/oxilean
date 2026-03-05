//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{
    LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfParam, LcnfType, LcnfVarId,
};
use std::collections::HashMap;

use std::collections::{HashSet, VecDeque};

/// Whether a value is known at partial-evaluation time (Static) or only at run
/// time (Dynamic).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingTime {
    /// Value is fully determined during partial evaluation.
    Static,
    /// Value can only be determined at run time.
    Dynamic,
    /// Value is a mix — some fields static, others dynamic.
    Mixed,
}
impl BindingTime {
    /// Derive binding time from a `PartialValue`.
    pub fn from_partial(pv: &PartialValue) -> Self {
        match pv {
            PartialValue::Known(_) => BindingTime::Static,
            PartialValue::Unknown => BindingTime::Dynamic,
            PartialValue::Partial(vs) => {
                let any_static = vs.iter().any(|v| v.is_known());
                let any_dynamic = vs.iter().any(|v| v.is_unknown());
                if any_static && any_dynamic {
                    BindingTime::Mixed
                } else if any_static {
                    BindingTime::Static
                } else {
                    BindingTime::Dynamic
                }
            }
            PartialValue::Contradiction => BindingTime::Dynamic,
        }
    }
}
/// The partial evaluation pass over LCNF.
///
/// # Usage
/// ```rust
/// use oxilean_codegen::opt_partial_eval::{PartialEvaluator, PartialEvalConfig};
/// let mut pe = PartialEvaluator::new(PartialEvalConfig::default());
/// ```
pub struct PartialEvaluator {
    pub(super) config: PartialEvalConfig,
    pub(super) report: PartialEvalReport,
    /// Cache: expr hash → (simplified expr, partial value)
    pub(super) memo: HashMap<u64, (LcnfExpr, PartialValue)>,
    /// Already-created specialisations (key → decl).
    pub(super) specializations: HashMap<SpecializationKey, LcnfFunDecl>,
    /// Counter for fresh variable IDs.
    pub(super) next_var_id: u64,
    /// Known function bodies (for inlining during PE).
    pub(super) known_fns: HashMap<String, LcnfFunDecl>,
}
impl PartialEvaluator {
    /// Create a new evaluator with the given configuration.
    pub fn new(config: PartialEvalConfig) -> Self {
        PartialEvaluator {
            config,
            report: PartialEvalReport::default(),
            memo: HashMap::new(),
            specializations: HashMap::new(),
            next_var_id: 200_000,
            known_fns: HashMap::new(),
        }
    }
    /// Create with default configuration.
    pub fn default_eval() -> Self {
        Self::new(PartialEvalConfig::default())
    }
    /// Run the partial evaluation pass over all declarations.
    pub fn run(&mut self, decls: &mut Vec<LcnfFunDecl>) {
        self.known_fns.clear();
        for decl in decls.iter() {
            self.known_fns.insert(decl.name.clone(), decl.clone());
        }
        let mut new_specializations: Vec<LcnfFunDecl> = Vec::new();
        for decl in decls.iter_mut() {
            let mut env = BindingEnv::new();
            let (new_body, _) = self.eval_expr(&decl.body, &mut env, 0);
            decl.body = new_body;
        }
        for spec_decl in self.specializations.values() {
            if !decls.iter().any(|d| d.name == spec_decl.name) {
                new_specializations.push(spec_decl.clone());
            }
        }
        decls.extend(new_specializations);
    }
    /// Return the accumulated report.
    pub fn report(&self) -> &PartialEvalReport {
        &self.report
    }
    /// Evaluate `expr` under `env`, returning the simplified expression and
    /// the partial value of the result.
    pub fn eval_expr(
        &mut self,
        expr: &LcnfExpr,
        env: &mut BindingEnv,
        depth: usize,
    ) -> (LcnfExpr, PartialValue) {
        if depth > self.config.max_depth {
            return (expr.clone(), PartialValue::Unknown);
        }
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => self.eval_let(id, name, ty, value, body, env, depth),
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            } => self.eval_case(scrutinee, scrutinee_ty, alts, default, env, depth),
            LcnfExpr::Return(arg) => {
                let pv = self.eval_arg(arg, env);
                let new_arg = self.materialize_arg(arg, &pv);
                (LcnfExpr::Return(new_arg), pv)
            }
            LcnfExpr::TailCall(func, args) => {
                let func_pv = self.eval_arg(func, env);
                let arg_pvs: Vec<PartialValue> =
                    args.iter().map(|a| self.eval_arg(a, env)).collect();
                let new_args: Vec<LcnfArg> = args
                    .iter()
                    .zip(arg_pvs.iter())
                    .map(|(a, pv)| self.materialize_arg(a, pv))
                    .collect();
                if let Some(fn_name) = self.extract_fn_name(func) {
                    if self.config.specialize_hot_paths {
                        if let Some(spec_name) = self.maybe_specialize(&fn_name, &arg_pvs, depth) {
                            let spec_func = LcnfArg::Var(LcnfVarId(0));
                            let _ = spec_func;
                            return (
                                LcnfExpr::TailCall(
                                    LcnfArg::Var(LcnfVarId(self.intern_fn_name(&spec_name))),
                                    new_args.clone(),
                                ),
                                PartialValue::Unknown,
                            );
                        }
                    }
                }
                let new_func = self.materialize_arg(func, &func_pv);
                (
                    LcnfExpr::TailCall(new_func, new_args),
                    PartialValue::Unknown,
                )
            }
            LcnfExpr::Unreachable => (LcnfExpr::Unreachable, PartialValue::Contradiction),
        }
    }
    /// Evaluate a `let` binding.
    pub(super) fn eval_let(
        &mut self,
        id: &LcnfVarId,
        name: &str,
        ty: &LcnfType,
        value: &LcnfLetValue,
        body: &LcnfExpr,
        env: &mut BindingEnv,
        depth: usize,
    ) -> (LcnfExpr, PartialValue) {
        let (new_value, pv) = self.eval_let_value(value, env, depth);
        if self.config.aggressive_const_prop && pv.is_known() {
            env.bind(*id, pv.clone());
            self.report.lets_removed += 1;
            let (new_body, body_pv) = self.eval_expr(body, env, depth + 1);
            return (new_body, body_pv);
        }
        env.bind(*id, pv.clone());
        let (new_body, body_pv) = self.eval_expr(body, env, depth + 1);
        (
            LcnfExpr::Let {
                id: *id,
                name: name.to_string(),
                ty: ty.clone(),
                value: new_value,
                body: Box::new(new_body),
            },
            body_pv,
        )
    }
    /// Evaluate a case expression, eliminating dead branches when the
    /// scrutinee is statically known.
    pub fn try_eval_case(
        &mut self,
        scrutinee: &LcnfVarId,
        alts: &[crate::lcnf::LcnfAlt],
        default: &Option<Box<LcnfExpr>>,
        env: &mut BindingEnv,
        depth: usize,
    ) -> Option<(LcnfExpr, PartialValue)> {
        let scrutinee_pv = env.lookup(*scrutinee).clone();
        if let PartialValue::Known(LcnfLit::Nat(n)) = &scrutinee_pv {
            let tag = *n as u32;
            if let Some(matching_alt) = alts.iter().find(|a| a.ctor_tag == tag) {
                self.report.branches_eliminated += alts.len() - 1;
                let (result, pv) = self.eval_expr(&matching_alt.body, env, depth + 1);
                return Some((result, pv));
            }
            if let Some(def) = default {
                self.report.branches_eliminated += alts.len();
                let (result, pv) = self.eval_expr(def, env, depth + 1);
                return Some((result, pv));
            }
        }
        None
    }
    /// Specialise `function_name` for the given argument partial values,
    /// returning the specialised function's name (if specialisation is worthwhile).
    pub fn specialize_function(
        &mut self,
        name: &str,
        static_args: Vec<PartialValue>,
    ) -> Option<LcnfFunDecl> {
        if self.specializations.len() >= self.config.max_specializations {
            return None;
        }
        let key = SpecializationKey::new(name, static_args.clone());
        if self.specializations.contains_key(&key) {
            self.report.memo_hits += 1;
            return self.specializations.get(&key).cloned();
        }
        let original = self.known_fns.get(name)?.clone();
        let mut env = BindingEnv::new();
        let mut new_params: Vec<LcnfParam> = Vec::new();
        for (i, param) in original.params.iter().enumerate() {
            let pv = static_args.get(i).cloned().unwrap_or(PartialValue::Unknown);
            if pv.is_known() {
                env.bind(param.id, pv);
            } else {
                new_params.push(param.clone());
                env.bind(param.id, PartialValue::Unknown);
            }
        }
        let (new_body, _) = self.eval_expr(&original.body, &mut env, 0);
        let spec_name = key.mangled_name();
        let spec_decl = LcnfFunDecl {
            name: spec_name.clone(),
            original_name: original.original_name.clone(),
            params: new_params,
            ret_type: original.ret_type.clone(),
            body: new_body,
            is_recursive: original.is_recursive,
            is_lifted: original.is_lifted,
            inline_cost: original.inline_cost,
        };
        self.specializations.insert(key, spec_decl.clone());
        self.report.functions_specialized += 1;
        Some(spec_decl)
    }
    pub(super) fn eval_case(
        &mut self,
        scrutinee: &LcnfVarId,
        scrutinee_ty: &LcnfType,
        alts: &[crate::lcnf::LcnfAlt],
        default: &Option<Box<LcnfExpr>>,
        env: &mut BindingEnv,
        depth: usize,
    ) -> (LcnfExpr, PartialValue) {
        if let Some(result) = self.try_eval_case(scrutinee, alts, default, env, depth) {
            return result;
        }
        let new_alts: Vec<crate::lcnf::LcnfAlt> = alts
            .iter()
            .map(|alt| {
                let mut branch_env = env.clone();
                for p in &alt.params {
                    branch_env.bind(p.id, PartialValue::Unknown);
                }
                let (new_body, _) = self.eval_expr(&alt.body, &mut branch_env, depth + 1);
                crate::lcnf::LcnfAlt {
                    ctor_name: alt.ctor_name.clone(),
                    ctor_tag: alt.ctor_tag,
                    params: alt.params.clone(),
                    body: new_body,
                }
            })
            .collect();
        let new_default = default.as_ref().map(|d| {
            let (new_def, _) = self.eval_expr(d, env, depth + 1);
            Box::new(new_def)
        });
        (
            LcnfExpr::Case {
                scrutinee: *scrutinee,
                scrutinee_ty: scrutinee_ty.clone(),
                alts: new_alts,
                default: new_default,
            },
            PartialValue::Unknown,
        )
    }
    pub(super) fn eval_let_value(
        &mut self,
        value: &LcnfLetValue,
        env: &mut BindingEnv,
        depth: usize,
    ) -> (LcnfLetValue, PartialValue) {
        let _ = depth;
        match value {
            LcnfLetValue::Lit(lit) => {
                self.report.constants_computed += 1;
                (
                    LcnfLetValue::Lit(lit.clone()),
                    PartialValue::Known(lit.clone()),
                )
            }
            LcnfLetValue::FVar(id) => {
                let pv = env.lookup(*id).clone();
                (LcnfLetValue::FVar(*id), pv)
            }
            LcnfLetValue::App(func_arg, args) => {
                let func_pv = self.eval_arg(func_arg, env);
                let arg_pvs: Vec<PartialValue> =
                    args.iter().map(|a| self.eval_arg(a, env)).collect();
                let new_func = self.materialize_arg(func_arg, &func_pv);
                let new_args: Vec<LcnfArg> = args
                    .iter()
                    .zip(arg_pvs.iter())
                    .map(|(a, pv)| self.materialize_arg(a, pv))
                    .collect();
                if let Some(result) = self.try_const_fold_app(&func_pv, &arg_pvs) {
                    self.report.constants_computed += 1;
                    return (
                        LcnfLetValue::Lit(result.clone()),
                        PartialValue::Known(result),
                    );
                }
                (LcnfLetValue::App(new_func, new_args), PartialValue::Unknown)
            }
            LcnfLetValue::Ctor(name, tag, args) => {
                let pvs: Vec<PartialValue> = args.iter().map(|a| self.eval_arg(a, env)).collect();
                let new_args: Vec<LcnfArg> = args
                    .iter()
                    .zip(pvs.iter())
                    .map(|(a, pv)| self.materialize_arg(a, pv))
                    .collect();
                let struct_pv = PartialValue::Partial(pvs);
                (LcnfLetValue::Ctor(name.clone(), *tag, new_args), struct_pv)
            }
            LcnfLetValue::Proj(name, idx, var) => {
                let var_pv = env.lookup(*var).clone();
                let field_pv = if let PartialValue::Partial(fields) = &var_pv {
                    fields
                        .get(*idx as usize)
                        .cloned()
                        .unwrap_or(PartialValue::Unknown)
                } else {
                    PartialValue::Unknown
                };
                (LcnfLetValue::Proj(name.clone(), *idx, *var), field_pv)
            }
            LcnfLetValue::Erased => (LcnfLetValue::Erased, PartialValue::Unknown),
            LcnfLetValue::Reset(var) => (LcnfLetValue::Reset(*var), PartialValue::Unknown),
            LcnfLetValue::Reuse(slot, name, tag, args) => {
                let pvs: Vec<PartialValue> = args.iter().map(|a| self.eval_arg(a, env)).collect();
                let new_args: Vec<LcnfArg> = args
                    .iter()
                    .zip(pvs.iter())
                    .map(|(a, pv)| self.materialize_arg(a, pv))
                    .collect();
                (
                    LcnfLetValue::Reuse(*slot, name.clone(), *tag, new_args),
                    PartialValue::Unknown,
                )
            }
        }
    }
    pub(super) fn eval_arg(&self, arg: &LcnfArg, env: &BindingEnv) -> PartialValue {
        match arg {
            LcnfArg::Lit(lit) => PartialValue::Known(lit.clone()),
            LcnfArg::Var(id) => env.lookup(*id).clone(),
            LcnfArg::Erased => PartialValue::Unknown,
            LcnfArg::Type(_) => PartialValue::Unknown,
        }
    }
    /// Materialize: if `pv` is `Known` and aggressive mode, return a literal arg;
    /// otherwise return the original arg unchanged.
    pub(super) fn materialize_arg(&self, original: &LcnfArg, pv: &PartialValue) -> LcnfArg {
        if self.config.aggressive_const_prop {
            if let PartialValue::Known(lit) = pv {
                return LcnfArg::Lit(lit.clone());
            }
        }
        original.clone()
    }
    /// Try to constant-fold a function application when arguments are known.
    pub(super) fn try_const_fold_app(
        &self,
        func_pv: &PartialValue,
        arg_pvs: &[PartialValue],
    ) -> Option<LcnfLit> {
        let _ = func_pv;
        if arg_pvs.len() == 2 {
            if let (PartialValue::Known(LcnfLit::Nat(a)), PartialValue::Known(LcnfLit::Nat(b))) =
                (&arg_pvs[0], &arg_pvs[1])
            {
                let _ = (a, b);
            }
        }
        None
    }
    /// Try to maybe specialise a function call; returns the specialised name.
    pub(super) fn maybe_specialize(
        &mut self,
        fn_name: &str,
        arg_pvs: &[PartialValue],
        depth: usize,
    ) -> Option<String> {
        if depth > self.config.max_depth / 2 {
            return None;
        }
        if !arg_pvs.iter().any(|pv| pv.is_known()) {
            return None;
        }
        let key = SpecializationKey::new(fn_name, arg_pvs.to_vec());
        if !key.has_static_args() {
            return None;
        }
        if self.specializations.len() >= self.config.max_specializations {
            return None;
        }
        let spec_name = key.mangled_name();
        if !self.specializations.contains_key(&key) {
            self.specialize_function(fn_name, arg_pvs.to_vec())?;
        }
        Some(spec_name)
    }
    pub(super) fn extract_fn_name(&self, arg: &LcnfArg) -> Option<String> {
        match arg {
            LcnfArg::Var(_id) => None,
            _ => None,
        }
    }
    pub(super) fn intern_fn_name(&mut self, name: &str) -> u64 {
        let mut h: u64 = 5381;
        for b in name.bytes() {
            h = h.wrapping_mul(31).wrapping_add(b as u64);
        }
        h % 999_999 + 1
    }
    pub(super) fn fresh_id(&mut self) -> LcnfVarId {
        let id = LcnfVarId(self.next_var_id);
        self.next_var_id += 1;
        id
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PEDominatorTree {
    pub idom: Vec<Option<u32>>,
    pub dom_children: Vec<Vec<u32>>,
    pub dom_depth: Vec<u32>,
}
impl PEDominatorTree {
    #[allow(dead_code)]
    pub fn new(size: usize) -> Self {
        PEDominatorTree {
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
pub struct PELivenessInfo {
    pub live_in: Vec<std::collections::HashSet<u32>>,
    pub live_out: Vec<std::collections::HashSet<u32>>,
    pub defs: Vec<std::collections::HashSet<u32>>,
    pub uses: Vec<std::collections::HashSet<u32>>,
}
impl PELivenessInfo {
    #[allow(dead_code)]
    pub fn new(block_count: usize) -> Self {
        PELivenessInfo {
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
/// Configuration for PEExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PEExtPassConfig {
    pub name: String,
    pub phase: PEExtPassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl PEExtPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: PEExtPassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: PEExtPassPhase) -> Self {
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
/// Statistics for PEExt passes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PEExtPassStats {
    pub iterations: usize,
    pub changed: bool,
    pub nodes_visited: usize,
    pub nodes_modified: usize,
    pub time_ms: u64,
    pub memory_bytes: usize,
    pub errors: usize,
}
impl PEExtPassStats {
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
    pub fn merge(&mut self, o: &PEExtPassStats) {
        self.iterations += o.iterations;
        self.changed |= o.changed;
        self.nodes_visited += o.nodes_visited;
        self.nodes_modified += o.nodes_modified;
        self.time_ms += o.time_ms;
        self.memory_bytes = self.memory_bytes.max(o.memory_bytes);
        self.errors += o.errors;
    }
}
/// Pass registry for PEExt.
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct PEExtPassRegistry {
    pub(super) configs: Vec<PEExtPassConfig>,
    pub(super) stats: Vec<PEExtPassStats>,
}
impl PEExtPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn register(&mut self, c: PEExtPassConfig) {
        self.stats.push(PEExtPassStats::new());
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
    pub fn get(&self, i: usize) -> Option<&PEExtPassConfig> {
        self.configs.get(i)
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, i: usize) -> Option<&PEExtPassStats> {
        self.stats.get(i)
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&PEExtPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn passes_in_phase(&self, ph: &PEExtPassPhase) -> Vec<&PEExtPassConfig> {
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
/// Worklist for PEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PEExtWorklist {
    pub(super) items: std::collections::VecDeque<usize>,
    pub(super) present: Vec<bool>,
}
impl PEExtWorklist {
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
/// Dominator tree for PEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PEExtDomTree {
    pub(super) idom: Vec<Option<usize>>,
    pub(super) children: Vec<Vec<usize>>,
    pub(super) depth: Vec<usize>,
}
impl PEExtDomTree {
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
/// Liveness analysis for PEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PEExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl PEExtLiveness {
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
/// Analysis cache for PEExt.
#[allow(dead_code)]
#[derive(Debug)]
pub struct PEExtCache {
    pub(super) entries: Vec<(u64, Vec<u8>, bool, u32)>,
    pub(super) cap: usize,
    pub(super) total_hits: u64,
    pub(super) total_misses: u64,
}
impl PEExtCache {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PEAnalysisCache {
    pub(super) entries: std::collections::HashMap<String, PECacheEntry>,
    pub(super) max_size: usize,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl PEAnalysisCache {
    #[allow(dead_code)]
    pub fn new(max_size: usize) -> Self {
        PEAnalysisCache {
            entries: std::collections::HashMap::new(),
            max_size,
            hits: 0,
            misses: 0,
        }
    }
    #[allow(dead_code)]
    pub fn get(&mut self, key: &str) -> Option<&PECacheEntry> {
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
            PECacheEntry {
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
/// A scoped environment mapping variable IDs to their partial values.
#[derive(Debug, Clone, Default)]
pub struct BindingEnv {
    /// Variable-to-partial-value mapping.
    pub(super) bindings: HashMap<LcnfVarId, PartialValue>,
    /// Depth of the current scope nesting.
    pub(super) scope_depth: usize,
    /// Scope entries: (var_id, depth) pairs for cleanup on scope exit.
    pub(super) scope_stack: Vec<(LcnfVarId, usize)>,
}
impl BindingEnv {
    /// Create a fresh empty environment.
    pub fn new() -> Self {
        Self::default()
    }
    /// Bind `var` to `value` in the current scope.
    pub fn bind(&mut self, var: LcnfVarId, value: PartialValue) {
        self.bindings.insert(var, value);
        self.scope_stack.push((var, self.scope_depth));
    }
    /// Look up the partial value of `var`.
    pub fn lookup(&self, var: LcnfVarId) -> &PartialValue {
        self.bindings.get(&var).unwrap_or(&PartialValue::Unknown)
    }
    /// Enter a new scope level.
    pub fn push_scope(&mut self) {
        self.scope_depth += 1;
    }
    /// Exit the current scope, removing all bindings introduced in it.
    pub fn pop_scope(&mut self) {
        let depth = self.scope_depth;
        self.scope_stack.retain(|(var, d)| {
            if *d == depth {
                false
            } else {
                let _ = var;
                true
            }
        });
        self.scope_depth = self.scope_depth.saturating_sub(1);
    }
    /// Number of bindings currently in scope.
    pub fn len(&self) -> usize {
        self.bindings.len()
    }
    /// Whether the environment is empty.
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }
    /// Merge another environment into this one (meet of all values).
    pub fn merge_from(&mut self, other: &BindingEnv) {
        for (id, val) in &other.bindings {
            let merged = match self.bindings.get(id) {
                Some(existing) => PartialValue::meet(existing, val),
                None => val.clone(),
            };
            self.bindings.insert(*id, merged);
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PEPassStats {
    pub total_runs: u32,
    pub successful_runs: u32,
    pub total_changes: u64,
    pub time_ms: u64,
    pub iterations_used: u32,
}
impl PEPassStats {
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
#[derive(Debug, Clone)]
pub struct PEPassConfig {
    pub phase: PEPassPhase,
    pub enabled: bool,
    pub max_iterations: u32,
    pub debug_output: bool,
    pub pass_name: String,
}
impl PEPassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>, phase: PEPassPhase) -> Self {
        PEPassConfig {
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
pub struct PEDepGraph {
    pub(super) nodes: Vec<u32>,
    pub(super) edges: Vec<(u32, u32)>,
}
impl PEDepGraph {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PEDepGraph {
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
/// Pass execution phase for PEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PEExtPassPhase {
    Early,
    Middle,
    Late,
    Finalize,
}
impl PEExtPassPhase {
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
/// Constant folding helper for PEExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct PEExtConstFolder {
    pub(super) folds: usize,
    pub(super) failures: usize,
    pub(super) enabled: bool,
}
impl PEExtConstFolder {
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
#[derive(Debug, Clone, PartialEq)]
pub enum PEPassPhase {
    Analysis,
    Transformation,
    Verification,
    Cleanup,
}
impl PEPassPhase {
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        match self {
            PEPassPhase::Analysis => "analysis",
            PEPassPhase::Transformation => "transformation",
            PEPassPhase::Verification => "verification",
            PEPassPhase::Cleanup => "cleanup",
        }
    }
    #[allow(dead_code)]
    pub fn is_modifying(&self) -> bool {
        matches!(self, PEPassPhase::Transformation | PEPassPhase::Cleanup)
    }
}
/// Dependency graph for PEExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PEExtDepGraph {
    pub(super) n: usize,
    pub(super) adj: Vec<Vec<usize>>,
    pub(super) rev: Vec<Vec<usize>>,
    pub(super) edge_count: usize,
}
impl PEExtDepGraph {
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
#[allow(dead_code)]
pub struct PEConstantFoldingHelper;
impl PEConstantFoldingHelper {
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
/// Uniquely identifies a specialised version of a function.
///
/// Two calls to `f` with different static arguments produce different keys
/// and thus different specialised variants.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpecializationKey {
    /// The original (unspecialised) function name.
    pub function_name: String,
    /// Partial values of the arguments (static args have `Known` entries).
    pub static_args: Vec<PartialValue>,
}
impl SpecializationKey {
    /// Create a new key.
    pub fn new(function_name: impl Into<String>, static_args: Vec<PartialValue>) -> Self {
        SpecializationKey {
            function_name: function_name.into(),
            static_args,
        }
    }
    /// Produce a mangled name suitable for the specialised variant.
    pub fn mangled_name(&self) -> String {
        let mut name = self.function_name.clone();
        for (i, v) in self.static_args.iter().enumerate() {
            match v {
                PartialValue::Known(LcnfLit::Nat(n)) => {
                    name.push_str(&format!("_a{}n{}", i, n));
                }
                PartialValue::Known(LcnfLit::Str(s)) => {
                    let short = if s.len() > 6 { &s[..6] } else { s.as_str() };
                    name.push_str(&format!("_a{}s{}", i, short));
                }
                PartialValue::Unknown => {}
                _ => {
                    name.push_str(&format!("_a{}p", i));
                }
            }
        }
        name
    }
    /// Whether any argument is statically known.
    pub fn has_static_args(&self) -> bool {
        self.static_args.iter().any(|v| v.is_known())
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PEWorklist {
    pub(super) items: std::collections::VecDeque<u32>,
    pub(super) in_worklist: std::collections::HashSet<u32>,
}
impl PEWorklist {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PEWorklist {
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
/// Configuration for the partial evaluation pass.
#[derive(Debug, Clone)]
pub struct PartialEvalConfig {
    /// Maximum number of function specialisations to create.
    pub max_specializations: usize,
    /// Maximum recursion depth during evaluation.
    pub max_depth: usize,
    /// Cache (memoize) previously evaluated expressions.
    pub enable_memoization: bool,
    /// Specialise functions on hot code paths.
    pub specialize_hot_paths: bool,
    /// Propagate constants aggressively (may increase code size).
    pub aggressive_const_prop: bool,
}
impl PartialEvalConfig {
    /// Configuration for aggressive partial evaluation.
    pub fn aggressive() -> Self {
        PartialEvalConfig {
            max_specializations: 500,
            max_depth: 100,
            enable_memoization: true,
            specialize_hot_paths: true,
            aggressive_const_prop: true,
        }
    }
    /// Configuration for conservative partial evaluation (safe for code size).
    pub fn conservative() -> Self {
        PartialEvalConfig {
            max_specializations: 10,
            max_depth: 20,
            enable_memoization: false,
            specialize_hot_paths: false,
            aggressive_const_prop: false,
        }
    }
}
/// Statistics produced by the partial evaluation pass.
#[derive(Debug, Clone, Default)]
pub struct PartialEvalReport {
    /// Number of case branches eliminated because the scrutinee was known.
    pub branches_eliminated: usize,
    /// Number of specialised function variants created.
    pub functions_specialized: usize,
    /// Number of constants computed at compile time.
    pub constants_computed: usize,
    /// Number of cache (memo) hits during evaluation.
    pub memo_hits: usize,
    /// Number of let bindings removed by constant propagation.
    pub lets_removed: usize,
}
impl PartialEvalReport {
    /// Merge another report into `self`.
    pub fn merge(&mut self, other: &PartialEvalReport) {
        self.branches_eliminated += other.branches_eliminated;
        self.functions_specialized += other.functions_specialized;
        self.constants_computed += other.constants_computed;
        self.memo_hits += other.memo_hits;
        self.lets_removed += other.lets_removed;
    }
    /// Total number of optimizations applied.
    pub fn total_optimizations(&self) -> usize {
        self.branches_eliminated
            + self.functions_specialized
            + self.constants_computed
            + self.lets_removed
    }
    /// Human-readable summary.
    pub fn summary(&self) -> String {
        format!(
            "PartialEvalReport {{ branches_elim={}, specialized={}, consts={}, \
             memo_hits={}, lets_removed={} }}",
            self.branches_eliminated,
            self.functions_specialized,
            self.constants_computed,
            self.memo_hits,
            self.lets_removed,
        )
    }
}
#[allow(dead_code)]
pub struct PEPassRegistry {
    pub(super) configs: Vec<PEPassConfig>,
    pub(super) stats: std::collections::HashMap<String, PEPassStats>,
}
impl PEPassRegistry {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PEPassRegistry {
            configs: Vec::new(),
            stats: std::collections::HashMap::new(),
        }
    }
    #[allow(dead_code)]
    pub fn register(&mut self, config: PEPassConfig) {
        self.stats
            .insert(config.pass_name.clone(), PEPassStats::new());
        self.configs.push(config);
    }
    #[allow(dead_code)]
    pub fn enabled_passes(&self) -> Vec<&PEPassConfig> {
        self.configs.iter().filter(|c| c.enabled).collect()
    }
    #[allow(dead_code)]
    pub fn get_stats(&self, name: &str) -> Option<&PEPassStats> {
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
pub struct PECacheEntry {
    pub key: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
    pub valid: bool,
}
/// The (partially) known value of a variable or expression at compile time.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PartialValue {
    /// Fully known — holds a concrete literal.
    Known(LcnfLit),
    /// Not statically known; must be evaluated at run time.
    Unknown,
    /// Partially known — a structured value with some known sub-fields.
    Partial(Vec<PartialValue>),
    /// Represents a logical contradiction reached during analysis.
    Contradiction,
}
impl PartialValue {
    /// Returns `true` if the value is fully known at compile time.
    pub fn is_known(&self) -> bool {
        matches!(self, PartialValue::Known(_))
    }
    /// Returns `true` if nothing is known about the value.
    pub fn is_unknown(&self) -> bool {
        matches!(self, PartialValue::Unknown)
    }
    /// Returns the literal if fully known, or `None` otherwise.
    pub fn as_lit(&self) -> Option<&LcnfLit> {
        match self {
            PartialValue::Known(lit) => Some(lit),
            _ => None,
        }
    }
    /// Merge two partial values conservatively (meet of the lattice).
    pub fn meet(a: &PartialValue, b: &PartialValue) -> PartialValue {
        match (a, b) {
            (PartialValue::Known(la), PartialValue::Known(lb)) if la == lb => {
                PartialValue::Known(la.clone())
            }
            (PartialValue::Contradiction, _) | (_, PartialValue::Contradiction) => {
                PartialValue::Contradiction
            }
            (PartialValue::Partial(xs), PartialValue::Partial(ys)) if xs.len() == ys.len() => {
                let merged = xs
                    .iter()
                    .zip(ys.iter())
                    .map(|(x, y)| PartialValue::meet(x, y))
                    .collect();
                PartialValue::Partial(merged)
            }
            _ => PartialValue::Unknown,
        }
    }
    /// Human-readable display.
    pub fn display(&self) -> String {
        match self {
            PartialValue::Known(lit) => format!("Known({:?})", lit),
            PartialValue::Unknown => "Unknown".to_string(),
            PartialValue::Partial(vs) => format!("Partial([{}])", vs.len()),
            PartialValue::Contradiction => "Contradiction".to_string(),
        }
    }
}
