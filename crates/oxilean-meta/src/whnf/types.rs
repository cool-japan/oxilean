//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::basic::{MVarId, MetaContext};
use oxilean_kernel::{
    reduce::TransparencyMode, ConstantInfo, Environment, Expr, Level, Name, Reducer,
};
use std::collections::HashMap;

/// An extended utility type for WhnfMeta.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WhnfMetaExt2 {
    /// A numeric tag.
    pub tag: u32,
}
#[allow(dead_code)]
impl WhnfMetaExt2 {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { tag: 0 }
    }
}
/// Metavar-aware WHNF engine.
///
/// Wraps the kernel's `Reducer` and adds metavariable handling.
pub struct MetaWhnf {
    /// Kernel reducer.
    pub(super) reducer: Reducer,
    /// WHNF cache keyed by (transparency, expr).
    pub(super) cache: HashMap<(u8, Expr), Expr>,
    /// Maximum cache size before eviction.
    pub(super) max_cache_size: usize,
    /// Current transparency mode.
    pub(super) transparency: TransparencyMode,
}
impl MetaWhnf {
    /// Create a new meta WHNF engine.
    pub fn new() -> Self {
        Self {
            reducer: Reducer::new(),
            cache: HashMap::new(),
            max_cache_size: 4096,
            transparency: TransparencyMode::Default,
        }
    }
    /// Create with a specific transparency mode.
    pub fn with_transparency(mode: TransparencyMode) -> Self {
        let mut whnf = Self::new();
        whnf.set_transparency(mode);
        whnf
    }
    /// Set the transparency mode.
    pub fn set_transparency(&mut self, mode: TransparencyMode) {
        self.transparency = mode;
        self.reducer.set_transparency(mode);
    }
    /// Get the current transparency mode.
    pub fn transparency(&self) -> TransparencyMode {
        self.transparency
    }
    /// Clear the WHNF cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
    /// Compute WHNF with metavar awareness.
    ///
    /// Steps:
    /// 1. Instantiate assigned metavariables
    /// 2. Check cache
    /// 3. Perform kernel WHNF
    /// 4. If stuck on mvar, return Stuck
    pub fn whnf(&mut self, expr: &Expr, ctx: &MetaContext) -> WhnfResult {
        let instantiated = ctx.instantiate_mvars(expr);
        let cache_key = (self.transparency as u8, instantiated.clone());
        if let Some(cached) = self.cache.get(&cache_key) {
            return self.check_stuck(cached, ctx);
        }
        let result = self.whnf_core(&instantiated, ctx);
        if self.cache.len() >= self.max_cache_size {
            self.cache.clear();
        }
        self.cache.insert(cache_key, result.expr().clone());
        result
    }
    /// Core WHNF implementation.
    fn whnf_core(&mut self, expr: &Expr, ctx: &MetaContext) -> WhnfResult {
        match expr {
            Expr::Sort(_) | Expr::Lit(_) | Expr::Lam(_, _, _, _) | Expr::Pi(_, _, _, _) => {
                WhnfResult::Reduced(expr.clone())
            }
            Expr::BVar(_) => WhnfResult::Reduced(expr.clone()),
            Expr::FVar(fid) => {
                if let Some(id) = MetaContext::is_mvar_expr(expr) {
                    if let Some(val) = ctx.get_mvar_assignment(id) {
                        return self.whnf_core(val, ctx);
                    }
                    return WhnfResult::Stuck(expr.clone(), id);
                }
                if let Some(val) = ctx.get_fvar_value(*fid) {
                    return self.whnf_core(val, ctx);
                }
                WhnfResult::Reduced(expr.clone())
            }
            Expr::Const(name, levels) => {
                if self.should_unfold(name, ctx.env()) {
                    if let Some(val) = self.unfold_const(name, levels, ctx.env()) {
                        return self.whnf_core(&val, ctx);
                    }
                }
                WhnfResult::Reduced(expr.clone())
            }
            Expr::App(_, _) => self.whnf_app(expr, ctx),
            Expr::Let(_, _, val, body) => {
                let substituted = oxilean_kernel::instantiate(body, val);
                self.whnf_core(&substituted, ctx)
            }
            Expr::Proj(_, _, _) => self.whnf_proj(expr, ctx),
        }
    }
    /// WHNF for applications.
    fn whnf_app(&mut self, expr: &Expr, ctx: &MetaContext) -> WhnfResult {
        let (head, args) = collect_app(expr);
        let head_result = self.whnf_core(head, ctx);
        match head_result {
            WhnfResult::Stuck(e, id) => {
                let app = rebuild_app(&e, &args);
                WhnfResult::Stuck(app, id)
            }
            WhnfResult::Reduced(head_whnf) => {
                if let Expr::Lam(_, _, _, body) = &head_whnf {
                    if let Some(first_arg) = args.first() {
                        let substituted = oxilean_kernel::instantiate(body, first_arg);
                        let remaining = if args.len() > 1 {
                            rebuild_app(&substituted, &args[1..])
                        } else {
                            substituted
                        };
                        return self.whnf_core(&remaining, ctx);
                    }
                }
                if let Some(reduced) = self.try_iota_reduction(&head_whnf, &args, ctx) {
                    return self.whnf_core(&reduced, ctx);
                }
                let result = rebuild_app(&head_whnf, &args);
                WhnfResult::Reduced(result)
            }
        }
    }
    /// Try iota reduction (recursor application).
    fn try_iota_reduction(
        &mut self,
        head: &Expr,
        args: &[Expr],
        ctx: &MetaContext,
    ) -> Option<Expr> {
        let (name, rec_levels) = match head {
            Expr::Const(n, lvls) => (n, lvls.as_slice()),
            _ => return None,
        };
        let rec_val = match ctx.find_const(name) {
            Some(ConstantInfo::Recursor(rv)) => rv,
            _ => return None,
        };
        let major_idx = rec_val.get_major_idx() as usize;
        if args.len() <= major_idx {
            return None;
        }
        let major = &args[major_idx];
        let major_whnf_result = self.whnf_core(major, ctx);
        let major_whnf = major_whnf_result.expr();
        let (ctor_head, ctor_args) = collect_app(major_whnf);
        let ctor_name = match ctor_head {
            Expr::Const(n, _) => n,
            _ => return None,
        };
        if !ctx.is_constructor(ctor_name) {
            return None;
        }
        let rule = rec_val.get_rule(ctor_name)?;
        let num_params = rec_val.num_params as usize;
        let first_index = rec_val.get_first_index_idx() as usize;
        let mut inst_args = Vec::new();
        for arg in args.iter().take(num_params) {
            inst_args.push(arg.clone());
        }
        for arg in args.iter().take(first_index).skip(num_params) {
            inst_args.push(arg.clone());
        }
        for arg in ctor_args.iter().skip(num_params) {
            inst_args.push(arg.clone());
        }
        let mut result = if !rec_val.common.level_params.is_empty() && !rec_levels.is_empty() {
            oxilean_kernel::instantiate_level_params(
                &rule.rhs,
                &rec_val.common.level_params,
                rec_levels,
            )
        } else {
            rule.rhs.clone()
        };
        for arg in inst_args.iter().rev() {
            result = oxilean_kernel::instantiate(&result, arg);
        }
        for arg in args.iter().skip(major_idx + 1) {
            result = Expr::App(Box::new(result), Box::new(arg.clone()));
        }
        Some(result)
    }
    /// WHNF for projections.
    fn whnf_proj(&mut self, expr: &Expr, ctx: &MetaContext) -> WhnfResult {
        if let Expr::Proj(_name, idx, inner) = expr {
            let inner_result = self.whnf_core(inner, ctx);
            match &inner_result {
                WhnfResult::Stuck(e, id) => {
                    WhnfResult::Stuck(Expr::Proj(_name.clone(), *idx, Box::new(e.clone())), *id)
                }
                WhnfResult::Reduced(inner_whnf) => {
                    let (ctor_head, ctor_args) = collect_app(inner_whnf);
                    if let Expr::Const(ctor_name, _) = ctor_head {
                        if ctx.is_constructor(ctor_name) {
                            if let Some(ConstantInfo::Constructor(cv)) = ctx.find_const(ctor_name) {
                                let field_start = cv.num_params as usize;
                                let field_idx = field_start + *idx as usize;
                                if field_idx < ctor_args.len() {
                                    return self.whnf_core(&ctor_args[field_idx], ctx);
                                }
                            }
                        }
                    }
                    WhnfResult::Reduced(Expr::Proj(
                        _name.clone(),
                        *idx,
                        Box::new(inner_whnf.clone()),
                    ))
                }
            }
        } else {
            WhnfResult::Reduced(expr.clone())
        }
    }
    /// Check if a result is stuck on an mvar.
    fn check_stuck(&self, expr: &Expr, ctx: &MetaContext) -> WhnfResult {
        if let Some(id) = MetaContext::is_mvar_expr(expr) {
            if !ctx.is_mvar_assigned(id) {
                return WhnfResult::Stuck(expr.clone(), id);
            }
        }
        WhnfResult::Reduced(expr.clone())
    }
    /// Check if a constant should be unfolded at the current transparency.
    fn should_unfold(&self, name: &Name, env: &Environment) -> bool {
        match self.transparency {
            TransparencyMode::None => false,
            TransparencyMode::All => true,
            TransparencyMode::Default => {
                if let Some(ci) = env.find(name) {
                    matches!(ci, ConstantInfo::Definition(_) | ConstantInfo::Theorem(_))
                } else {
                    env.get(name).is_some()
                }
            }
            TransparencyMode::Reducible => {
                if let Some(ConstantInfo::Definition(dv)) = env.find(name) {
                    matches!(dv.hints, oxilean_kernel::ReducibilityHint::Abbrev)
                } else {
                    false
                }
            }
            TransparencyMode::Instances => {
                if let Some(ConstantInfo::Definition(dv)) = env.find(name) {
                    matches!(dv.hints, oxilean_kernel::ReducibilityHint::Abbrev)
                } else {
                    false
                }
            }
        }
    }
    /// Try to unfold a constant by looking up its definition value.
    ///
    /// Instantiates universe level parameters when the definition has them.
    fn unfold_const(&self, name: &Name, levels: &[Level], env: &Environment) -> Option<Expr> {
        if let Some(ci) = env.find(name) {
            let (raw_val, lparams): (Option<&Expr>, &[_]) = match ci {
                ConstantInfo::Definition(dv) => (Some(&dv.value), &dv.common.level_params),
                ConstantInfo::Theorem(tv) => (Some(&tv.value), &tv.common.level_params),
                _ => (None, &[]),
            };
            if let Some(val) = raw_val {
                if lparams.is_empty() || levels.is_empty() {
                    return Some(val.clone());
                } else {
                    use oxilean_kernel::instantiate_level_params;
                    return Some(instantiate_level_params(val, lparams, levels));
                }
            }
        }
        if let Some(decl) = env.get(name) {
            return decl.value().cloned();
        }
        None
    }
    /// Execute an operation at a specific transparency level, then restore.
    pub fn at_transparency<F, R>(&mut self, mode: TransparencyMode, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let old = self.transparency;
        self.set_transparency(mode);
        let result = f(self);
        self.set_transparency(old);
        result
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum WhnfExtConfigVal1700 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl WhnfExtConfigVal1700 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let WhnfExtConfigVal1700::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let WhnfExtConfigVal1700::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let WhnfExtConfigVal1700::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let WhnfExtConfigVal1700::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let WhnfExtConfigVal1700::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            WhnfExtConfigVal1700::Bool(_) => "bool",
            WhnfExtConfigVal1700::Int(_) => "int",
            WhnfExtConfigVal1700::Float(_) => "float",
            WhnfExtConfigVal1700::Str(_) => "str",
            WhnfExtConfigVal1700::List(_) => "list",
        }
    }
}
/// A diff for Whnf analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct WhnfDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl WhnfDiff {
    pub fn new() -> Self {
        WhnfDiff {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
#[allow(dead_code)]
pub struct WhnfExtConfig1700 {
    pub(super) values: std::collections::HashMap<String, WhnfExtConfigVal1700>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl WhnfExtConfig1700 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: String::new(),
        }
    }
    #[allow(dead_code)]
    pub fn named(name: &str) -> Self {
        Self {
            values: std::collections::HashMap::new(),
            read_only: false,
            name: name.to_string(),
        }
    }
    #[allow(dead_code)]
    pub fn set(&mut self, key: &str, value: WhnfExtConfigVal1700) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&WhnfExtConfigVal1700> {
        self.values.get(key)
    }
    #[allow(dead_code)]
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    #[allow(dead_code)]
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    #[allow(dead_code)]
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    #[allow(dead_code)]
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, WhnfExtConfigVal1700::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, WhnfExtConfigVal1700::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, WhnfExtConfigVal1700::Str(v.to_string()))
    }
    #[allow(dead_code)]
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    #[allow(dead_code)]
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        self.values.len()
    }
    #[allow(dead_code)]
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
/// A pipeline of Whnf analysis passes.
#[allow(dead_code)]
pub struct WhnfPipeline {
    pub passes: Vec<WhnfAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl WhnfPipeline {
    pub fn new(name: &str) -> Self {
        WhnfPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: WhnfAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<WhnfAnalysisResult> {
        self.total_inputs_processed += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    pub fn total_success_rate(&self) -> f64 {
        if self.passes.is_empty() {
            0.0
        } else {
            let total_rate: f64 = self.passes.iter().map(|p| p.success_rate()).sum();
            total_rate / self.passes.len() as f64
        }
    }
}
/// A typed slot for Whnf configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum WhnfConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl WhnfConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            WhnfConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            WhnfConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            WhnfConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            WhnfConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            WhnfConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            WhnfConfigValue::Bool(_) => "bool",
            WhnfConfigValue::Int(_) => "int",
            WhnfConfigValue::Float(_) => "float",
            WhnfConfigValue::Str(_) => "str",
            WhnfConfigValue::List(_) => "list",
        }
    }
}
/// An extended utility type for WhnfMeta.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct WhnfMetaExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl WhnfMetaExt {
    /// Creates a new default instance.
    pub fn new() -> Self {
        Self {
            tag: 0,
            description: None,
        }
    }
    /// Sets the tag.
    pub fn with_tag(mut self, tag: u32) -> Self {
        self.tag = tag;
        self
    }
    /// Sets the description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }
    /// Returns `true` if the description is set.
    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }
}
#[allow(dead_code)]
pub struct WhnfExtPass1700 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<WhnfExtResult1700>,
}
impl WhnfExtPass1700 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_runs: 0,
            successes: 0,
            errors: 0,
            enabled: true,
            results: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self, input: &str) -> WhnfExtResult1700 {
        if !self.enabled {
            return WhnfExtResult1700::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            WhnfExtResult1700::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            WhnfExtResult1700::Ok(format!(
                "processed {} chars in pass '{}'",
                input.len(),
                self.name
            ))
        };
        self.results.push(result.clone());
        result
    }
    #[allow(dead_code)]
    pub fn success_count(&self) -> usize {
        self.successes
    }
    #[allow(dead_code)]
    pub fn error_count(&self) -> usize {
        self.errors
    }
    #[allow(dead_code)]
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.successes as f64 / self.total_runs as f64
        }
    }
    #[allow(dead_code)]
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    #[allow(dead_code)]
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    #[allow(dead_code)]
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
/// A state machine controller for WhnfMeta.
#[allow(dead_code)]
pub struct WhnfMetaStateMachine {
    pub state: WhnfMetaState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl WhnfMetaStateMachine {
    pub fn new() -> Self {
        WhnfMetaStateMachine {
            state: WhnfMetaState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: WhnfMetaState) -> bool {
        if self.state.is_terminal() {
            return false;
        }
        let desc = format!("{:?} -> {:?}", self.state, new_state);
        self.state = new_state;
        self.transitions += 1;
        self.history.push(desc);
        true
    }
    pub fn start(&mut self) -> bool {
        self.transition_to(WhnfMetaState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(WhnfMetaState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(WhnfMetaState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(WhnfMetaState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
/// A sliding window accumulator for WhnfMeta.
#[allow(dead_code)]
pub struct WhnfMetaWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl WhnfMetaWindow {
    pub fn new(capacity: usize) -> Self {
        WhnfMetaWindow {
            buffer: std::collections::VecDeque::new(),
            capacity,
            running_sum: 0.0,
        }
    }
    pub fn push(&mut self, v: f64) {
        if self.buffer.len() >= self.capacity {
            if let Some(old) = self.buffer.pop_front() {
                self.running_sum -= old;
            }
        }
        self.buffer.push_back(v);
        self.running_sum += v;
    }
    pub fn mean(&self) -> f64 {
        if self.buffer.is_empty() {
            0.0
        } else {
            self.running_sum / self.buffer.len() as f64
        }
    }
    pub fn variance(&self) -> f64 {
        if self.buffer.len() < 2 {
            return 0.0;
        }
        let m = self.mean();
        self.buffer.iter().map(|&x| (x - m).powi(2)).sum::<f64>() / self.buffer.len() as f64
    }
    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
    pub fn is_full(&self) -> bool {
        self.buffer.len() >= self.capacity
    }
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}
/// A result type for Whnf analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum WhnfAnalysisResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl WhnfAnalysisResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, WhnfAnalysisResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, WhnfAnalysisResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, WhnfAnalysisResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, WhnfAnalysisResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            WhnfAnalysisResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            WhnfAnalysisResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            WhnfAnalysisResult::Ok(_) => 1.0,
            WhnfAnalysisResult::Err(_) => 0.0,
            WhnfAnalysisResult::Skipped => 0.0,
            WhnfAnalysisResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// Configuration for the metavar-aware WHNF engine.
#[derive(Clone, Debug)]
pub struct WhnfConfig {
    /// Maximum beta reduction steps.
    pub max_beta_steps: u32,
    /// Whether to unfold let-bindings.
    pub unfold_let: bool,
    /// Whether to reduce iota (recursor applications).
    pub reduce_iota: bool,
    /// Whether to reduce zeta (let-expressions).
    pub reduce_zeta: bool,
}
impl WhnfConfig {
    /// Conservative config: only beta and zeta.
    pub fn conservative() -> Self {
        Self {
            max_beta_steps: 128,
            unfold_let: true,
            reduce_iota: false,
            reduce_zeta: true,
        }
    }
    /// Aggressive config: all reductions.
    pub fn aggressive() -> Self {
        Self {
            max_beta_steps: 16384,
            unfold_let: true,
            reduce_iota: true,
            reduce_zeta: true,
        }
    }
}
/// Describes the head form of an expression in WHNF.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeadForm {
    /// The expression is a sort (Type/Prop).
    Sort,
    /// The expression is a lambda abstraction.
    Lam,
    /// The expression is a pi type.
    Pi,
    /// The expression is a literal.
    Lit,
    /// The expression is a free variable (fvar).
    FVar,
    /// The expression is an application with a constant head.
    App(oxilean_kernel::Name),
    /// The expression is an application with a non-constant head.
    AppNonConst,
    /// The expression is a stuck metavariable.
    StuckMVar(MVarId),
    /// The expression is a neutral free variable application.
    Neutral,
}
/// A state machine for WhnfMeta.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum WhnfMetaState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl WhnfMetaState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, WhnfMetaState::Complete | WhnfMetaState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, WhnfMetaState::Initial | WhnfMetaState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, WhnfMetaState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            WhnfMetaState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
pub struct WhnfMetaExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl WhnfMetaExtUtil {
    pub fn new(key: &str) -> Self {
        WhnfMetaExtUtil {
            key: key.to_string(),
            data: Vec::new(),
            active: true,
            flags: 0,
        }
    }
    pub fn push(&mut self, v: i64) {
        self.data.push(v);
    }
    pub fn pop(&mut self) -> Option<i64> {
        self.data.pop()
    }
    pub fn sum(&self) -> i64 {
        self.data.iter().sum()
    }
    pub fn min_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::min)
    }
    pub fn max_val(&self) -> Option<i64> {
        self.data.iter().copied().reduce(i64::max)
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
    pub fn set_flag(&mut self, bit: u32) {
        self.flags |= 1 << bit;
    }
    pub fn has_flag(&self, bit: u32) -> bool {
        self.flags & (1 << bit) != 0
    }
    pub fn deactivate(&mut self) {
        self.active = false;
    }
    pub fn activate(&mut self) {
        self.active = true;
    }
}
#[allow(dead_code)]
pub struct WhnfExtPipeline1700 {
    pub name: String,
    pub passes: Vec<WhnfExtPass1700>,
    pub run_count: usize,
}
impl WhnfExtPipeline1700 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: WhnfExtPass1700) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<WhnfExtResult1700> {
        self.run_count += 1;
        self.passes
            .iter_mut()
            .filter(|p| p.enabled)
            .map(|p| p.run(input))
            .collect()
    }
    #[allow(dead_code)]
    pub fn num_passes(&self) -> usize {
        self.passes.len()
    }
    #[allow(dead_code)]
    pub fn num_enabled_passes(&self) -> usize {
        self.passes.iter().filter(|p| p.enabled).count()
    }
    #[allow(dead_code)]
    pub fn total_success_rate(&self) -> f64 {
        let total: usize = self.passes.iter().map(|p| p.total_runs).sum();
        let ok: usize = self.passes.iter().map(|p| p.successes).sum();
        if total == 0 {
            0.0
        } else {
            ok as f64 / total as f64
        }
    }
}
#[allow(dead_code)]
pub struct WhnfExtDiff1700 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl WhnfExtDiff1700 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            added: Vec::new(),
            removed: Vec::new(),
            unchanged: Vec::new(),
        }
    }
    #[allow(dead_code)]
    pub fn add(&mut self, s: &str) {
        self.added.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn remove(&mut self, s: &str) {
        self.removed.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn keep(&mut self, s: &str) {
        self.unchanged.push(s.to_string());
    }
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty()
    }
    #[allow(dead_code)]
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.removed.len()
    }
    #[allow(dead_code)]
    pub fn net_additions(&self) -> i64 {
        self.added.len() as i64 - self.removed.len() as i64
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "+{} -{} =={}",
            self.added.len(),
            self.removed.len(),
            self.unchanged.len()
        )
    }
}
/// A counter map for WhnfMeta frequency analysis.
#[allow(dead_code)]
pub struct WhnfMetaCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl WhnfMetaCounterMap {
    pub fn new() -> Self {
        WhnfMetaCounterMap {
            counts: std::collections::HashMap::new(),
            total: 0,
        }
    }
    pub fn increment(&mut self, key: &str) {
        *self.counts.entry(key.to_string()).or_insert(0) += 1;
        self.total += 1;
    }
    pub fn count(&self, key: &str) -> usize {
        *self.counts.get(key).unwrap_or(&0)
    }
    pub fn frequency(&self, key: &str) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.count(key) as f64 / self.total as f64
        }
    }
    pub fn most_common(&self) -> Option<(&String, usize)> {
        self.counts
            .iter()
            .max_by_key(|(_, &v)| v)
            .map(|(k, &v)| (k, v))
    }
    pub fn num_unique(&self) -> usize {
        self.counts.len()
    }
    pub fn is_empty(&self) -> bool {
        self.counts.is_empty()
    }
}
/// A work queue for WhnfMeta items.
#[allow(dead_code)]
pub struct WhnfMetaWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl WhnfMetaWorkQueue {
    pub fn new(capacity: usize) -> Self {
        WhnfMetaWorkQueue {
            pending: std::collections::VecDeque::new(),
            processed: Vec::new(),
            capacity,
        }
    }
    pub fn enqueue(&mut self, item: String) -> bool {
        if self.pending.len() >= self.capacity {
            return false;
        }
        self.pending.push_back(item);
        true
    }
    pub fn dequeue(&mut self) -> Option<String> {
        let item = self.pending.pop_front()?;
        self.processed.push(item.clone());
        Some(item)
    }
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }
    pub fn processed_count(&self) -> usize {
        self.processed.len()
    }
    pub fn is_empty(&self) -> bool {
        self.pending.is_empty()
    }
    pub fn is_full(&self) -> bool {
        self.pending.len() >= self.capacity
    }
    pub fn total_processed(&self) -> usize {
        self.processed.len()
    }
}
/// A diagnostic reporter for Whnf.
#[allow(dead_code)]
pub struct WhnfDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl WhnfDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        WhnfDiagnostics {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
/// A builder pattern for WhnfMeta.
#[allow(dead_code)]
pub struct WhnfMetaBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl WhnfMetaBuilder {
    pub fn new(name: &str) -> Self {
        WhnfMetaBuilder {
            name: name.to_string(),
            items: Vec::new(),
            config: std::collections::HashMap::new(),
        }
    }
    pub fn add_item(mut self, item: &str) -> Self {
        self.items.push(item.to_string());
        self
    }
    pub fn set_config(mut self, key: &str, value: &str) -> Self {
        self.config.insert(key.to_string(), value.to_string());
        self
    }
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
    pub fn has_config(&self, key: &str) -> bool {
        self.config.contains_key(key)
    }
    pub fn get_config(&self, key: &str) -> Option<&str> {
        self.config.get(key).map(|s| s.as_str())
    }
    pub fn build_summary(&self) -> String {
        format!(
            "{}: {} items, {} config keys",
            self.name,
            self.items.len(),
            self.config.len()
        )
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum WhnfExtResult1700 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl WhnfExtResult1700 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, WhnfExtResult1700::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, WhnfExtResult1700::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, WhnfExtResult1700::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, WhnfExtResult1700::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let WhnfExtResult1700::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let WhnfExtResult1700::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            WhnfExtResult1700::Ok(_) => 1.0,
            WhnfExtResult1700::Err(_) => 0.0,
            WhnfExtResult1700::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            WhnfExtResult1700::Skipped => 0.5,
        }
    }
}
/// Statistics for WHNF reduction runs.
#[derive(Clone, Debug, Default)]
pub struct WhnfStats {
    /// Number of successful reductions.
    pub reductions: u64,
    /// Number of cache hits.
    pub cache_hits: u64,
    /// Number of stuck expressions.
    pub stuck_count: u64,
}
impl WhnfStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a reduction.
    pub fn record_reduction(&mut self) {
        self.reductions += 1;
    }
    /// Record a cache hit.
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    /// Record a stuck result.
    pub fn record_stuck(&mut self) {
        self.stuck_count += 1;
    }
    /// Cache hit rate.
    pub fn cache_hit_rate(&self) -> f64 {
        let total = self.reductions + self.cache_hits;
        if total == 0 {
            0.0
        } else {
            self.cache_hits as f64 / total as f64
        }
    }
}
/// Result of WHNF reduction in the meta context.
#[derive(Clone, Debug)]
pub enum WhnfResult {
    /// Successfully reduced to WHNF.
    Reduced(Expr),
    /// Stuck on an unassigned metavariable.
    Stuck(Expr, MVarId),
}
impl WhnfResult {
    /// Get the resulting expression regardless of stuck status.
    pub fn expr(&self) -> &Expr {
        match self {
            WhnfResult::Reduced(e) | WhnfResult::Stuck(e, _) => e,
        }
    }
    /// Check if the result is stuck on a metavar.
    pub fn is_stuck(&self) -> bool {
        matches!(self, WhnfResult::Stuck(_, _))
    }
}
/// An extended map for WhnfMeta keys to values.
#[allow(dead_code)]
pub struct WhnfMetaExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> WhnfMetaExtMap<V> {
    pub fn new() -> Self {
        WhnfMetaExtMap {
            data: std::collections::HashMap::new(),
            default_key: None,
        }
    }
    pub fn insert(&mut self, key: &str, value: V) {
        self.data.insert(key.to_string(), value);
    }
    pub fn get(&self, key: &str) -> Option<&V> {
        self.data.get(key)
    }
    pub fn get_or_default(&self, key: &str) -> V {
        self.data.get(key).cloned().unwrap_or_default()
    }
    pub fn contains(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> Option<V> {
        self.data.remove(key)
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    pub fn set_default(&mut self, key: &str) {
        self.default_key = Some(key.to_string());
    }
    pub fn keys_sorted(&self) -> Vec<&String> {
        let mut keys: Vec<&String> = self.data.keys().collect();
        keys.sort();
        keys
    }
}
/// An analysis pass for Whnf.
#[allow(dead_code)]
pub struct WhnfAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<WhnfAnalysisResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl WhnfAnalysisPass {
    pub fn new(name: &str) -> Self {
        WhnfAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> WhnfAnalysisResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            WhnfAnalysisResult::Err("empty input".to_string())
        } else {
            WhnfAnalysisResult::Ok(format!("processed: {}", input))
        };
        self.results.push(result.clone());
        result
    }
    pub fn success_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_ok()).count()
    }
    pub fn error_count(&self) -> usize {
        self.results.iter().filter(|r| r.is_err()).count()
    }
    pub fn success_rate(&self) -> f64 {
        if self.total_runs == 0 {
            0.0
        } else {
            self.success_count() as f64 / self.total_runs as f64
        }
    }
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    pub fn clear_results(&mut self) {
        self.results.clear();
    }
}
/// A configuration store for Whnf.
#[allow(dead_code)]
pub struct WhnfConfigStore {
    pub values: std::collections::HashMap<String, WhnfConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl WhnfConfigStore {
    pub fn new() -> Self {
        WhnfConfigStore {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: WhnfConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&WhnfConfigValue> {
        self.values.get(key)
    }
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.get(key)?.as_bool()
    }
    pub fn get_int(&self, key: &str) -> Option<i64> {
        self.get(key)?.as_int()
    }
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key)?.as_str()
    }
    pub fn set_bool(&mut self, key: &str, v: bool) -> bool {
        self.set(key, WhnfConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, WhnfConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, WhnfConfigValue::Str(v.to_string()))
    }
    pub fn lock(&mut self) {
        self.read_only = true;
    }
    pub fn unlock(&mut self) {
        self.read_only = false;
    }
    pub fn size(&self) -> usize {
        self.values.len()
    }
    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }
    pub fn remove(&mut self, key: &str) -> bool {
        self.values.remove(key).is_some()
    }
}
#[allow(dead_code)]
pub struct WhnfExtDiag1700 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl WhnfExtDiag1700 {
    #[allow(dead_code)]
    pub fn new(max_errors: usize) -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            notes: Vec::new(),
            max_errors,
        }
    }
    #[allow(dead_code)]
    pub fn error(&mut self, msg: &str) {
        if self.errors.len() < self.max_errors {
            self.errors.push(msg.to_string());
        }
    }
    #[allow(dead_code)]
    pub fn warning(&mut self, msg: &str) {
        self.warnings.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn note(&mut self, msg: &str) {
        self.notes.push(msg.to_string());
    }
    #[allow(dead_code)]
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    #[allow(dead_code)]
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }
    #[allow(dead_code)]
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }
    #[allow(dead_code)]
    pub fn is_clean(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }
    #[allow(dead_code)]
    pub fn at_error_limit(&self) -> bool {
        self.errors.len() >= self.max_errors
    }
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.notes.clear();
    }
    #[allow(dead_code)]
    pub fn summary(&self) -> String {
        format!(
            "{} error(s), {} warning(s)",
            self.errors.len(),
            self.warnings.len()
        )
    }
}
