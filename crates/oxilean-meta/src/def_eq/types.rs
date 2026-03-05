//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::basic::{MVarId, MetaContext};
use crate::infer_type::MetaInferType;
use crate::whnf::{MetaWhnf, WhnfResult};
use oxilean_kernel::{Expr, FVarId};

#[allow(dead_code)]
pub struct DefEqExtPipeline2100 {
    pub name: String,
    pub passes: Vec<DefEqExtPass2100>,
    pub run_count: usize,
}
impl DefEqExtPipeline2100 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: DefEqExtPass2100) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<DefEqExtResult2100> {
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
/// A state machine for DefEq.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DefEqState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl DefEqState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, DefEqState::Complete | DefEqState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, DefEqState::Initial | DefEqState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, DefEqState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            DefEqState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
#[allow(dead_code)]
pub struct DefEqExtConfig2100 {
    pub(super) values: std::collections::HashMap<String, DefEqExtConfigVal2100>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl DefEqExtConfig2100 {
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
    pub fn set(&mut self, key: &str, value: DefEqExtConfigVal2100) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&DefEqExtConfigVal2100> {
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
        self.set(key, DefEqExtConfigVal2100::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, DefEqExtConfigVal2100::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, DefEqExtConfigVal2100::Str(v.to_string()))
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
#[allow(dead_code)]
pub struct DefEqExtDiff2100 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl DefEqExtDiff2100 {
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
#[allow(dead_code)]
pub struct DefEqExtDiag2100 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl DefEqExtDiag2100 {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DefEqExtConfigVal2100 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl DefEqExtConfigVal2100 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let DefEqExtConfigVal2100::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let DefEqExtConfigVal2100::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let DefEqExtConfigVal2100::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let DefEqExtConfigVal2100::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let DefEqExtConfigVal2100::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            DefEqExtConfigVal2100::Bool(_) => "bool",
            DefEqExtConfigVal2100::Int(_) => "int",
            DefEqExtConfigVal2100::Float(_) => "float",
            DefEqExtConfigVal2100::Str(_) => "str",
            DefEqExtConfigVal2100::List(_) => "list",
        }
    }
}
/// An extended utility type for DefEq.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DefEqExt2 {
    /// A numeric tag.
    pub tag: u32,
}
#[allow(dead_code)]
impl DefEqExt2 {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { tag: 0 }
    }
}
/// An analysis pass for DefEq.
#[allow(dead_code)]
pub struct DefEqAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<DefEqResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl DefEqAnalysisPass {
    pub fn new(name: &str) -> Self {
        DefEqAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> DefEqResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            DefEqResult::Err("empty input".to_string())
        } else {
            DefEqResult::Ok(format!("processed: {}", input))
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
/// A typed slot for DefEq configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DefEqConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl DefEqConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DefEqConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            DefEqConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            DefEqConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            DefEqConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            DefEqConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            DefEqConfigValue::Bool(_) => "bool",
            DefEqConfigValue::Int(_) => "int",
            DefEqConfigValue::Float(_) => "float",
            DefEqConfigValue::Str(_) => "str",
            DefEqConfigValue::List(_) => "list",
        }
    }
}
/// A configuration store for DefEq.
#[allow(dead_code)]
pub struct DefEqConfigStore {
    pub values: std::collections::HashMap<String, DefEqConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl DefEqConfigStore {
    pub fn new() -> Self {
        DefEqConfigStore {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: DefEqConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&DefEqConfigValue> {
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
        self.set(key, DefEqConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, DefEqConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, DefEqConfigValue::Str(v.to_string()))
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
/// A builder pattern for DefEq.
#[allow(dead_code)]
pub struct DefEqBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl DefEqBuilder {
    pub fn new(name: &str) -> Self {
        DefEqBuilder {
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
/// A result type for DefEq analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum DefEqResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl DefEqResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, DefEqResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, DefEqResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, DefEqResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, DefEqResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            DefEqResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            DefEqResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            DefEqResult::Ok(_) => 1.0,
            DefEqResult::Err(_) => 0.0,
            DefEqResult::Skipped => 0.0,
            DefEqResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// Meta-level definitional equality checker with unification.
pub struct MetaDefEq {
    /// WHNF engine.
    pub(super) whnf: MetaWhnf,
    /// Type inference engine (used for proof irrelevance checks).
    pub(super) infer: MetaInferType,
    /// Maximum recursion depth.
    pub(super) max_depth: u32,
    /// Number of unification steps performed.
    pub(super) steps: u64,
    /// Maximum number of steps before giving up.
    pub(super) max_steps: u64,
}
impl MetaDefEq {
    /// Create a new unification engine.
    pub fn new() -> Self {
        Self {
            whnf: MetaWhnf::new(),
            infer: MetaInferType::new(),
            max_depth: 512,
            steps: 0,
            max_steps: 100_000,
        }
    }
    /// Get a reference to the WHNF engine.
    pub fn whnf(&mut self) -> &mut MetaWhnf {
        &mut self.whnf
    }
    /// Reset step counter.
    pub fn reset_steps(&mut self) {
        self.steps = 0;
    }
    /// Get the number of steps performed.
    pub fn num_steps(&self) -> u64 {
        self.steps
    }
    /// Check if two expressions are definitionally equal, possibly assigning metavariables.
    pub fn is_def_eq(&mut self, e1: &Expr, e2: &Expr, ctx: &mut MetaContext) -> UnificationResult {
        self.steps = 0;
        self.is_def_eq_impl(e1, e2, ctx, 0)
    }
    /// Core implementation with depth tracking.
    fn is_def_eq_impl(
        &mut self,
        e1: &Expr,
        e2: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> UnificationResult {
        self.steps += 1;
        if depth > self.max_depth || self.steps > self.max_steps {
            return UnificationResult::NotEqual;
        }
        if e1 == e2 {
            return UnificationResult::Equal;
        }
        let e1_inst = ctx.instantiate_mvars(e1);
        let e2_inst = ctx.instantiate_mvars(e2);
        if e1_inst == e2_inst {
            return UnificationResult::Equal;
        }
        if let Some(result) = self.try_mvar_assignment(&e1_inst, &e2_inst, ctx, depth) {
            return result;
        }
        let whnf1 = self.whnf.whnf(&e1_inst, ctx);
        let whnf2 = self.whnf.whnf(&e2_inst, ctx);
        if whnf1.is_stuck() || whnf2.is_stuck() {
            return self.handle_stuck(&whnf1, &whnf2, ctx, depth);
        }
        let w1 = whnf1.expr().clone();
        let w2 = whnf2.expr().clone();
        self.structural_eq(&w1, &w2, ctx, depth)
    }
    /// Try to solve the problem by assigning a metavariable.
    fn try_mvar_assignment(
        &mut self,
        e1: &Expr,
        e2: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> Option<UnificationResult> {
        if let Some(id1) = MetaContext::is_mvar_expr(e1) {
            if !ctx.is_mvar_assigned(id1) {
                return Some(self.assign_mvar(id1, e2, ctx, depth));
            }
        }
        if let Some(id2) = MetaContext::is_mvar_expr(e2) {
            if !ctx.is_mvar_assigned(id2) {
                return Some(self.assign_mvar(id2, e1, ctx, depth));
            }
        }
        if let Some((id1, args1)) = self.extract_mvar_app(e1, ctx) {
            if !ctx.is_mvar_assigned(id1) {
                return Some(self.process_mvar_assignment(id1, &args1, e2, ctx, depth));
            }
        }
        if let Some((id2, args2)) = self.extract_mvar_app(e2, ctx) {
            if !ctx.is_mvar_assigned(id2) {
                return Some(self.process_mvar_assignment(id2, &args2, e1, ctx, depth));
            }
        }
        None
    }
    /// Extract a metavariable application: `?m a₁ ... aₙ`.
    fn extract_mvar_app(&self, expr: &Expr, ctx: &MetaContext) -> Option<(MVarId, Vec<Expr>)> {
        let mut args = Vec::new();
        let mut e = expr;
        while let Expr::App(f, a) = e {
            args.push(a.as_ref().clone());
            e = f;
        }
        if let Some(id) = MetaContext::is_mvar_expr(e) {
            if !ctx.is_mvar_assigned(id) {
                args.reverse();
                return Some((id, args));
            }
        }
        None
    }
    /// Assign a bare metavariable: `?m := e`.
    fn assign_mvar(
        &mut self,
        id: MVarId,
        val: &Expr,
        ctx: &mut MetaContext,
        _depth: u32,
    ) -> UnificationResult {
        if self.occurs_check(id, val, ctx) {
            return UnificationResult::NotEqual;
        }
        if ctx.assign_mvar(id, val.clone()) {
            UnificationResult::Equal
        } else {
            UnificationResult::NotEqual
        }
    }
    /// Process metavar assignment with arguments: `?m a₁ ... aₙ =?= v`.
    ///
    /// Uses higher-order pattern matching when all `aᵢ` are distinct
    /// free variables, or first-order approximation otherwise.
    fn process_mvar_assignment(
        &mut self,
        id: MVarId,
        args: &[Expr],
        rhs: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> UnificationResult {
        let fvars = self.extract_distinct_fvars(args);
        if let Some(fvar_ids) = fvars {
            return self.pattern_assignment(id, &fvar_ids, rhs, ctx, depth);
        }
        if ctx.config().fo_approx {
            return self.fo_approx_assignment(id, args, rhs, ctx, depth);
        }
        UnificationResult::Postponed
    }
    /// Extract distinct free variable IDs from a list of expressions.
    /// Returns None if any expression is not an FVar or if there are duplicates.
    fn extract_distinct_fvars(&self, args: &[Expr]) -> Option<Vec<FVarId>> {
        let mut fvars = Vec::new();
        for arg in args {
            if let Expr::FVar(fid) = arg {
                if fvars.contains(fid) {
                    return None;
                }
                fvars.push(*fid);
            } else {
                return None;
            }
        }
        Some(fvars)
    }
    /// Higher-order pattern assignment.
    ///
    /// Given `?m x₁ ... xₙ =?= v` where xᵢ are distinct fvars,
    /// assigns `?m := λ x₁ ... xₙ. v`.
    fn pattern_assignment(
        &mut self,
        id: MVarId,
        fvars: &[FVarId],
        rhs: &Expr,
        ctx: &mut MetaContext,
        _depth: u32,
    ) -> UnificationResult {
        if self.occurs_check(id, rhs, ctx) {
            return UnificationResult::NotEqual;
        }
        let val = ctx.mk_lambda(fvars, rhs.clone());
        if ctx.assign_mvar(id, val) {
            UnificationResult::Equal
        } else {
            UnificationResult::NotEqual
        }
    }
    /// First-order approximation for `?m a₁ ... aₙ =?= f b₁ ... bₖ`.
    ///
    /// When `n ≥ k`, reduces to:
    /// - `?M a₁ ... a_{n-k} =?= f`
    /// - `a_{n-k+1} =?= b₁`, ..., `aₙ =?= bₖ`
    fn fo_approx_assignment(
        &mut self,
        id: MVarId,
        args: &[Expr],
        rhs: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> UnificationResult {
        let mut rhs_args = Vec::new();
        let mut rhs_head = rhs;
        while let Expr::App(f, a) = rhs_head {
            rhs_args.push(a.as_ref().clone());
            rhs_head = f;
        }
        rhs_args.reverse();
        if args.len() < rhs_args.len() {
            return UnificationResult::Postponed;
        }
        let arg_offset = args.len() - rhs_args.len();
        for (i, rhs_arg) in rhs_args.iter().enumerate() {
            let result = self.is_def_eq_impl(&args[arg_offset + i], rhs_arg, ctx, depth + 1);
            if !result.is_equal() {
                return result;
            }
        }
        if arg_offset == 0 {
            return self.assign_mvar(id, rhs_head, ctx, depth);
        }
        let prefix_args = &args[..arg_offset];
        let prefix_fvars: Vec<FVarId> = prefix_args
            .iter()
            .filter_map(|a| {
                if let Expr::FVar(fv) = a {
                    Some(*fv)
                } else {
                    None
                }
            })
            .collect();
        let all_fvars = prefix_fvars.len() == arg_offset;
        let all_distinct = {
            let mut seen = std::collections::HashSet::new();
            prefix_fvars.iter().all(|fv| seen.insert(fv.0))
        };
        if all_fvars && all_distinct {
            let lambda_val = ctx.mk_lambda(&prefix_fvars, rhs_head.clone());
            if ctx.assign_mvar(id, lambda_val) {
                return UnificationResult::Equal;
            }
        }
        UnificationResult::Postponed
    }
    /// Check if a metavariable occurs in an expression.
    fn occurs_check(&self, id: MVarId, expr: &Expr, ctx: &MetaContext) -> bool {
        if let Some(eid) = MetaContext::is_mvar_expr(expr) {
            if eid == id {
                return true;
            }
            if let Some(val) = ctx.get_mvar_assignment(eid) {
                return self.occurs_check(id, val, ctx);
            }
            return false;
        }
        match expr {
            Expr::Sort(_) | Expr::BVar(_) | Expr::Const(_, _) | Expr::Lit(_) | Expr::FVar(_) => {
                false
            }
            Expr::App(f, a) => self.occurs_check(id, f, ctx) || self.occurs_check(id, a, ctx),
            Expr::Lam(_, _, ty, body) | Expr::Pi(_, _, ty, body) => {
                self.occurs_check(id, ty, ctx) || self.occurs_check(id, body, ctx)
            }
            Expr::Let(_, ty, val, body) => {
                self.occurs_check(id, ty, ctx)
                    || self.occurs_check(id, val, ctx)
                    || self.occurs_check(id, body, ctx)
            }
            Expr::Proj(_, _, e) => self.occurs_check(id, e, ctx),
        }
    }
    /// Handle expressions that are stuck on metavariables.
    fn handle_stuck(
        &mut self,
        whnf1: &WhnfResult,
        whnf2: &WhnfResult,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> UnificationResult {
        let e1 = whnf1.expr();
        let e2 = whnf2.expr();
        if let (WhnfResult::Stuck(_, id1), WhnfResult::Stuck(_, id2)) = (whnf1, whnf2) {
            if id1 == id2 {
                return UnificationResult::Equal;
            }
        }
        if let Some(result) = self.try_mvar_assignment(e1, e2, ctx, depth) {
            return result;
        }
        ctx.postpone(e1.clone(), e2.clone());
        UnificationResult::Postponed
    }
    /// Structural equality check after WHNF.
    fn structural_eq(
        &mut self,
        e1: &Expr,
        e2: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> UnificationResult {
        if e1 == e2 {
            return UnificationResult::Equal;
        }
        match (e1, e2) {
            (Expr::BVar(i1), Expr::BVar(i2)) => {
                if i1 == i2 {
                    UnificationResult::Equal
                } else {
                    UnificationResult::NotEqual
                }
            }
            (Expr::FVar(f1), Expr::FVar(f2)) => {
                if f1 == f2 {
                    UnificationResult::Equal
                } else {
                    UnificationResult::NotEqual
                }
            }
            (Expr::Const(n1, ls1), Expr::Const(n2, ls2)) => {
                if n1 == n2 && ls1.len() == ls2.len() {
                    for (l1, l2) in ls1.iter().zip(ls2.iter()) {
                        let l1_inst = ctx.instantiate_level_mvars(l1);
                        let l2_inst = ctx.instantiate_level_mvars(l2);
                        if !oxilean_kernel::level::is_equivalent(&l1_inst, &l2_inst) {
                            return UnificationResult::NotEqual;
                        }
                    }
                    UnificationResult::Equal
                } else {
                    UnificationResult::NotEqual
                }
            }
            (Expr::Sort(l1), Expr::Sort(l2)) => {
                let l1_inst = ctx.instantiate_level_mvars(l1);
                let l2_inst = ctx.instantiate_level_mvars(l2);
                if oxilean_kernel::level::is_equivalent(&l1_inst, &l2_inst) {
                    UnificationResult::Equal
                } else {
                    UnificationResult::NotEqual
                }
            }
            (Expr::Lit(l1), Expr::Lit(l2)) => {
                if l1 == l2 {
                    UnificationResult::Equal
                } else {
                    UnificationResult::NotEqual
                }
            }
            (Expr::App(f1, a1), Expr::App(f2, a2)) => {
                let fr = self.is_def_eq_impl(f1, f2, ctx, depth + 1);
                if !fr.is_equal() {
                    return fr;
                }
                self.is_def_eq_impl(a1, a2, ctx, depth + 1)
            }
            (Expr::Lam(_, _, ty1, body1), Expr::Lam(_, _, ty2, body2)) => {
                let tr = self.is_def_eq_impl(ty1, ty2, ctx, depth + 1);
                if !tr.is_equal() {
                    return tr;
                }
                self.is_def_eq_impl(body1, body2, ctx, depth + 1)
            }
            (Expr::Pi(_, _, ty1, body1), Expr::Pi(_, _, ty2, body2)) => {
                let tr = self.is_def_eq_impl(ty1, ty2, ctx, depth + 1);
                if !tr.is_equal() {
                    return tr;
                }
                self.is_def_eq_impl(body1, body2, ctx, depth + 1)
            }
            (Expr::Let(_, ty1, val1, body1), Expr::Let(_, ty2, val2, body2)) => {
                let tr = self.is_def_eq_impl(ty1, ty2, ctx, depth + 1);
                if !tr.is_equal() {
                    return tr;
                }
                let vr = self.is_def_eq_impl(val1, val2, ctx, depth + 1);
                if !vr.is_equal() {
                    return vr;
                }
                self.is_def_eq_impl(body1, body2, ctx, depth + 1)
            }
            (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
                if n1 == n2 && i1 == i2 {
                    self.is_def_eq_impl(e1, e2, ctx, depth + 1)
                } else {
                    UnificationResult::NotEqual
                }
            }
            _ => {
                if ctx.config().proof_irrelevance && self.is_proof_irrelevant(e1, e2, ctx) {
                    return UnificationResult::Equal;
                }
                UnificationResult::NotEqual
            }
        }
    }
    /// Check if two expressions can be considered equal by proof irrelevance.
    ///
    /// If both expressions have a type that is a proposition (lives in Prop),
    /// then they are definitionally equal.  Two proofs of the same proposition
    /// are definitionally equal by proof irrelevance.
    fn is_proof_irrelevant(&mut self, e1: &Expr, e2: &Expr, ctx: &mut MetaContext) -> bool {
        let ty1 = match self.infer.infer_type(e1, ctx) {
            Ok(t) => t,
            Err(_) => return false,
        };
        let is_prop = match self.infer.is_prop(&ty1, ctx) {
            Ok(b) => b,
            Err(_) => return false,
        };
        if !is_prop {
            return false;
        }
        let ty2 = match self.infer.infer_type(e2, ctx) {
            Ok(t) => t,
            Err(_) => return false,
        };
        self.infer.is_prop(&ty2, ctx).unwrap_or_default()
    }
}
/// Statistics collected during a unification run.
#[derive(Clone, Debug, Default)]
pub struct UnificationStats {
    /// Number of successful unifications.
    pub successes: u64,
    /// Number of failures.
    pub failures: u64,
    /// Number of postponed constraints.
    pub postponed: u64,
    /// Total steps.
    pub steps: u64,
}
impl UnificationStats {
    /// Create zeroed stats.
    pub fn new() -> Self {
        Self::default()
    }
    /// Record a result.
    pub fn record(&mut self, result: UnificationResult, steps: u64) {
        self.steps += steps;
        match result {
            UnificationResult::Equal => self.successes += 1,
            UnificationResult::NotEqual => self.failures += 1,
            UnificationResult::Postponed => self.postponed += 1,
        }
    }
    /// Success rate as a fraction in [0, 1].
    pub fn success_rate(&self) -> f64 {
        let total = (self.successes + self.failures + self.postponed) as f64;
        if total == 0.0 {
            1.0
        } else {
            self.successes as f64 / total
        }
    }
}
/// A pipeline of DefEq analysis passes.
#[allow(dead_code)]
pub struct DefEqPipeline {
    pub passes: Vec<DefEqAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl DefEqPipeline {
    pub fn new(name: &str) -> Self {
        DefEqPipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: DefEqAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<DefEqResult> {
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
pub struct DefEqExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl DefEqExtUtil {
    pub fn new(key: &str) -> Self {
        DefEqExtUtil {
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
/// A counter map for DefEq frequency analysis.
#[allow(dead_code)]
pub struct DefEqCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl DefEqCounterMap {
    pub fn new() -> Self {
        DefEqCounterMap {
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
/// A diagnostic reporter for DefEq.
#[allow(dead_code)]
pub struct DefEqDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl DefEqDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        DefEqDiagnostics {
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
/// A state machine controller for DefEq.
#[allow(dead_code)]
pub struct DefEqStateMachine {
    pub state: DefEqState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl DefEqStateMachine {
    pub fn new() -> Self {
        DefEqStateMachine {
            state: DefEqState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: DefEqState) -> bool {
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
        self.transition_to(DefEqState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(DefEqState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(DefEqState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(DefEqState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
#[allow(dead_code)]
pub struct DefEqExtPass2100 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<DefEqExtResult2100>,
}
impl DefEqExtPass2100 {
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
    pub fn run(&mut self, input: &str) -> DefEqExtResult2100 {
        if !self.enabled {
            return DefEqExtResult2100::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            DefEqExtResult2100::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            DefEqExtResult2100::Ok(format!(
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
/// A queue of postponed unification constraints.
#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct UnifConstraintQueue {
    /// Pending constraints.
    pub(super) constraints: Vec<UnifConstraint>,
}
#[allow(dead_code)]
impl UnifConstraintQueue {
    /// Create an empty queue.
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
        }
    }
    /// Push a new constraint.
    pub fn push(&mut self, lhs: Expr, rhs: Expr, depth: u32) {
        self.constraints.push(UnifConstraint::new(lhs, rhs, depth));
    }
    /// Pop the next constraint for processing.
    pub fn pop(&mut self) -> Option<UnifConstraint> {
        self.constraints.pop()
    }
    /// Number of pending constraints.
    pub fn len(&self) -> usize {
        self.constraints.len()
    }
    /// Check if there are no pending constraints.
    pub fn is_empty(&self) -> bool {
        self.constraints.is_empty()
    }
    /// Discard all trivially-equal constraints.
    pub fn drain_trivial(&mut self) {
        self.constraints.retain(|c| !c.is_trivial());
    }
}
/// Result of a unification attempt.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnificationResult {
    /// Expressions are definitionally equal.
    Equal,
    /// Expressions are not definitionally equal.
    NotEqual,
    /// Cannot determine yet (postponed).
    Postponed,
}
impl UnificationResult {
    /// Check if the result is equal.
    pub fn is_equal(self) -> bool {
        self == UnificationResult::Equal
    }
    /// Check if the result is not equal.
    pub fn is_not_equal(self) -> bool {
        self == UnificationResult::NotEqual
    }
}
/// A diff for DefEq analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DefEqDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl DefEqDiff {
    pub fn new() -> Self {
        DefEqDiff {
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
/// A sliding window accumulator for DefEq.
#[allow(dead_code)]
pub struct DefEqWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl DefEqWindow {
    pub fn new(capacity: usize) -> Self {
        DefEqWindow {
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
/// A unification constraint: `lhs =?= rhs` with optional depth info.
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct UnifConstraint {
    /// Left-hand side expression.
    pub lhs: Expr,
    /// Right-hand side expression.
    pub rhs: Expr,
    /// Depth at which this constraint was created.
    pub depth: u32,
}
#[allow(dead_code)]
impl UnifConstraint {
    /// Create a new constraint.
    pub fn new(lhs: Expr, rhs: Expr, depth: u32) -> Self {
        Self { lhs, rhs, depth }
    }
    /// Check if both sides are syntactically equal.
    pub fn is_trivial(&self) -> bool {
        self.lhs == self.rhs
    }
}
/// Configuration for the definitional equality checker.
#[derive(Clone, Debug)]
pub struct DefEqConfig {
    /// Whether to use proof irrelevance (Props are equal if they have the same type).
    pub proof_irrelevance: bool,
    /// Whether to use eta expansion for functions.
    pub eta_reduction: bool,
    /// Whether to allow lazy delta reduction.
    pub lazy_delta: bool,
    /// Maximum number of unfolding steps for lazy delta.
    pub max_delta_steps: u32,
}
impl DefEqConfig {
    /// Create a strict config (no proof irrelevance, no eta).
    pub fn strict() -> Self {
        Self {
            proof_irrelevance: false,
            eta_reduction: false,
            lazy_delta: false,
            max_delta_steps: 0,
        }
    }
    /// Create a lenient config with all reductions enabled.
    pub fn lenient() -> Self {
        Self {
            proof_irrelevance: true,
            eta_reduction: true,
            lazy_delta: true,
            max_delta_steps: 4096,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DefEqExtResult2100 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl DefEqExtResult2100 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, DefEqExtResult2100::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, DefEqExtResult2100::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, DefEqExtResult2100::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, DefEqExtResult2100::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let DefEqExtResult2100::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let DefEqExtResult2100::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            DefEqExtResult2100::Ok(_) => 1.0,
            DefEqExtResult2100::Err(_) => 0.0,
            DefEqExtResult2100::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            DefEqExtResult2100::Skipped => 0.5,
        }
    }
}
/// A work queue for DefEq items.
#[allow(dead_code)]
pub struct DefEqWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl DefEqWorkQueue {
    pub fn new(capacity: usize) -> Self {
        DefEqWorkQueue {
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
/// An extended map for DefEq keys to values.
#[allow(dead_code)]
pub struct DefEqExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> DefEqExtMap<V> {
    pub fn new() -> Self {
        DefEqExtMap {
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
/// An extended utility type for DefEq.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct DefEqExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl DefEqExt {
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
