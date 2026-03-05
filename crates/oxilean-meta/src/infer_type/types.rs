//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::basic::{MVarId, MetaContext, MetavarKind};
use crate::whnf::MetaWhnf;
use oxilean_kernel::{BinderInfo, ConstantInfo, Expr, Level, Name};

/// A builder pattern for InferType.
#[allow(dead_code)]
pub struct InferTypeBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl InferTypeBuilder {
    pub fn new(name: &str) -> Self {
        InferTypeBuilder {
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
/// A work queue for InferType items.
#[allow(dead_code)]
pub struct InferTypeWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl InferTypeWorkQueue {
    pub fn new(capacity: usize) -> Self {
        InferTypeWorkQueue {
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
/// An extended utility type for InferType.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct InferTypeExt2 {
    /// A numeric tag.
    pub tag: u32,
}
#[allow(dead_code)]
impl InferTypeExt2 {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { tag: 0 }
    }
}
/// An analysis pass for InferType.
#[allow(dead_code)]
pub struct InferTypeAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<InferTypeResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl InferTypeAnalysisPass {
    pub fn new(name: &str) -> Self {
        InferTypeAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> InferTypeResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            InferTypeResult::Err("empty input".to_string())
        } else {
            InferTypeResult::Ok(format!("processed: {}", input))
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
/// A cache for type inference results.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TypeInferCache {
    pub(super) cache: std::collections::HashMap<Expr, Expr>,
    pub(super) hits: u64,
    pub(super) misses: u64,
}
impl TypeInferCache {
    /// Create a new empty cache.
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    /// Look up a type in the cache.
    #[allow(dead_code)]
    pub fn get(&mut self, expr: &Expr) -> Option<&Expr> {
        let result = self.cache.get(expr);
        if result.is_some() {
            self.hits += 1;
        } else {
            self.misses += 1;
        }
        result
    }
    /// Insert a type into the cache.
    #[allow(dead_code)]
    pub fn insert(&mut self, expr: Expr, ty: Expr) {
        self.cache.insert(expr, ty);
    }
    /// Clear the cache.
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.cache.clear();
    }
    /// Get the cache hit rate.
    #[allow(dead_code)]
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
    /// Get total accesses.
    #[allow(dead_code)]
    pub fn total_accesses(&self) -> u64 {
        self.hits + self.misses
    }
    /// Get number of cached entries.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.cache.len()
    }
    /// Check if empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum InferTypeExtConfigVal2300 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl InferTypeExtConfigVal2300 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let InferTypeExtConfigVal2300::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let InferTypeExtConfigVal2300::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let InferTypeExtConfigVal2300::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let InferTypeExtConfigVal2300::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let InferTypeExtConfigVal2300::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            InferTypeExtConfigVal2300::Bool(_) => "bool",
            InferTypeExtConfigVal2300::Int(_) => "int",
            InferTypeExtConfigVal2300::Float(_) => "float",
            InferTypeExtConfigVal2300::Str(_) => "str",
            InferTypeExtConfigVal2300::List(_) => "list",
        }
    }
}
/// Wraps an inferred type with provenance information.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TypeWithProvenance {
    /// The inferred type.
    pub ty: Expr,
    /// The expression that was typed.
    pub source: Expr,
    /// Depth at which inference occurred.
    pub depth: u32,
}
impl TypeWithProvenance {
    /// Create a new provenance record.
    #[allow(dead_code)]
    pub fn new(ty: Expr, source: Expr, depth: u32) -> Self {
        Self { ty, source, depth }
    }
    /// Check if the type is Prop.
    #[allow(dead_code)]
    pub fn is_prop(&self) -> bool {
        is_prop_expr(&self.ty)
    }
}
#[allow(dead_code)]
pub struct InferTypeExtPipeline2300 {
    pub name: String,
    pub passes: Vec<InferTypeExtPass2300>,
    pub run_count: usize,
}
impl InferTypeExtPipeline2300 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: InferTypeExtPass2300) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<InferTypeExtResult2300> {
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
/// Meta-level type inference engine.
pub struct MetaInferType {
    /// WHNF engine for type normalization.
    pub(super) whnf: MetaWhnf,
    /// Maximum recursion depth.
    pub(super) max_depth: u32,
}
impl MetaInferType {
    /// Create a new type inference engine.
    pub fn new() -> Self {
        Self {
            whnf: MetaWhnf::new(),
            max_depth: 512,
        }
    }
    /// Infer the type of an expression.
    pub fn infer_type(&mut self, expr: &Expr, ctx: &mut MetaContext) -> Result<Expr, String> {
        self.infer_type_impl(expr, ctx, 0)
    }
    /// Internal implementation with depth tracking.
    fn infer_type_impl(
        &mut self,
        expr: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> Result<Expr, String> {
        if depth > self.max_depth {
            return Err("Maximum type inference depth exceeded".to_string());
        }
        if let Some(id) = MetaContext::is_mvar_expr(expr) {
            return self.infer_mvar(id, ctx, depth);
        }
        match expr {
            Expr::Sort(level) => {
                let level_inst = ctx.instantiate_level_mvars(level);
                Ok(Expr::Sort(Level::succ(level_inst)))
            }
            Expr::BVar(_) => Err("Unbound bound variable during type inference".to_string()),
            Expr::FVar(fvar_id) => ctx
                .get_fvar_type(*fvar_id)
                .cloned()
                .ok_or_else(|| format!("Unknown free variable: fvar_{}", fvar_id.0)),
            Expr::Const(name, levels) => self.infer_const(name, levels, ctx),
            Expr::App(f, a) => self.infer_app(f, a, ctx, depth),
            Expr::Lam(info, name, ty, body) => self.infer_lambda(*info, name, ty, body, ctx, depth),
            Expr::Pi(_, _, ty, body) => self.infer_pi(ty, body, ctx, depth),
            Expr::Let(name, ty, val, body) => self.infer_let(name, ty, val, body, ctx, depth),
            Expr::Lit(lit) => self.infer_literal(lit),
            Expr::Proj(struct_name, idx, inner) => {
                self.infer_proj(struct_name, *idx, inner, ctx, depth)
            }
        }
    }
    /// Infer the type of a metavariable.
    fn infer_mvar(
        &mut self,
        id: MVarId,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> Result<Expr, String> {
        if let Some(val) = ctx.get_mvar_assignment(id) {
            let val = val.clone();
            return self.infer_type_impl(&val, ctx, depth + 1);
        }
        ctx.get_mvar_type(id)
            .cloned()
            .ok_or_else(|| format!("Unknown metavariable: {}", id))
    }
    /// Infer the type of a constant.
    fn infer_const(
        &self,
        name: &Name,
        levels: &[Level],
        ctx: &MetaContext,
    ) -> Result<Expr, String> {
        if let Some(ci) = ctx.find_const(name) {
            let ty = oxilean_kernel::instantiate_level_params(ci.ty(), ci.level_params(), levels);
            return Ok(ty);
        }
        if let Some(decl) = ctx.env().get(name) {
            let ty =
                oxilean_kernel::instantiate_level_params(decl.ty(), decl.univ_params(), levels);
            return Ok(ty);
        }
        Err(format!("Unknown constant: {}", name))
    }
    /// Infer the type of a function application.
    fn infer_app(
        &mut self,
        f: &Expr,
        a: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> Result<Expr, String> {
        let f_ty = self.infer_type_impl(f, ctx, depth + 1)?;
        let f_ty_whnf = self.whnf.whnf(&f_ty, ctx);
        let f_ty_expr = f_ty_whnf.expr();
        match f_ty_expr {
            Expr::Pi(_, _, _domain, codomain) => Ok(oxilean_kernel::instantiate(codomain, a)),
            _ => {
                if f_ty_whnf.is_stuck() {
                    let level_mvar = ctx.mk_fresh_level_mvar();
                    let result_sort = Expr::Sort(level_mvar);
                    let (_, mvar) = ctx.mk_fresh_expr_mvar(result_sort, MetavarKind::Natural);
                    Ok(mvar)
                } else {
                    Err(format!("Type of function is not a Pi type: {}", f_ty_expr))
                }
            }
        }
    }
    /// Infer the type of a lambda abstraction.
    fn infer_lambda(
        &mut self,
        info: BinderInfo,
        name: &Name,
        ty: &Expr,
        body: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> Result<Expr, String> {
        let fvar_id = ctx.mk_local_decl(name.clone(), ty.clone(), info);
        let body_with_fvar = oxilean_kernel::instantiate(body, &Expr::FVar(fvar_id));
        let body_ty = self.infer_type_impl(&body_with_fvar, ctx, depth + 1)?;
        let pi_body = crate::basic::abstract_fvar_in_expr(&body_ty, fvar_id, 0);
        Ok(Expr::Pi(
            info,
            name.clone(),
            Box::new(ty.clone()),
            Box::new(pi_body),
        ))
    }
    /// Infer the type of a Pi type.
    fn infer_pi(
        &mut self,
        ty: &Expr,
        body: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> Result<Expr, String> {
        let ty_ty = self.infer_type_impl(ty, ctx, depth + 1)?;
        let ty_level = self.get_level(&ty_ty, ctx)?;
        let fvar_id = ctx.mk_local_decl(Name::str("_"), ty.clone(), BinderInfo::Default);
        let body_with_fvar = oxilean_kernel::instantiate(body, &Expr::FVar(fvar_id));
        let body_ty = self.infer_type_impl(&body_with_fvar, ctx, depth + 1)?;
        let body_level = self.get_level(&body_ty, ctx)?;
        Ok(Expr::Sort(Level::imax(ty_level, body_level)))
    }
    /// Infer the type of a let binding.
    fn infer_let(
        &mut self,
        _name: &Name,
        _ty: &Expr,
        val: &Expr,
        body: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> Result<Expr, String> {
        let body_inst = oxilean_kernel::instantiate(body, val);
        self.infer_type_impl(&body_inst, ctx, depth + 1)
    }
    /// Infer the type of a literal.
    fn infer_literal(&self, lit: &oxilean_kernel::Literal) -> Result<Expr, String> {
        match lit {
            oxilean_kernel::Literal::Nat(_) => Ok(Expr::Const(Name::str("Nat"), vec![])),
            oxilean_kernel::Literal::Str(_) => Ok(Expr::Const(Name::str("String"), vec![])),
        }
    }
    /// Infer the type of a projection.
    ///
    /// Telescopes through the constructor type to find the field type at `idx`,
    /// mirroring the kernel's `infer_proj_field_type`.
    fn infer_proj(
        &mut self,
        struct_name: &Name,
        idx: u32,
        inner: &Expr,
        ctx: &mut MetaContext,
        depth: u32,
    ) -> Result<Expr, String> {
        let ind_val = match ctx.find_const(struct_name) {
            Some(ConstantInfo::Inductive(iv)) => iv.clone(),
            _ => {
                let proj_ty = Expr::Sort(Level::zero());
                let (_, mvar) = ctx.mk_fresh_expr_mvar(proj_ty, MetavarKind::Natural);
                return Ok(mvar);
            }
        };
        if ind_val.ctors.len() != 1 {
            let proj_ty = Expr::Sort(Level::zero());
            let (_, mvar) = ctx.mk_fresh_expr_mvar(proj_ty, MetavarKind::Natural);
            return Ok(mvar);
        }
        let ctor_name = ind_val.ctors[0].clone();
        let ctor_val = match ctx.find_const(&ctor_name) {
            Some(ConstantInfo::Constructor(cv)) => cv.clone(),
            _ => {
                let proj_ty = Expr::Sort(Level::zero());
                let (_, mvar) = ctx.mk_fresh_expr_mvar(proj_ty, MetavarKind::Natural);
                return Ok(mvar);
            }
        };
        if idx >= ctor_val.num_fields {
            return Err(format!(
                "field index {} out of range for {} (has {} fields)",
                idx, struct_name, ctor_val.num_fields
            ));
        }
        let struct_ty = self.infer_type_impl(inner, ctx, depth + 1)?;
        let struct_ty_whnf = self.whnf.whnf(&struct_ty, ctx);
        let levels: Vec<Level> = match oxilean_kernel::expr_util::get_app_fn(struct_ty_whnf.expr())
        {
            Expr::Const(_, lvls) => lvls.clone(),
            _ => vec![],
        };
        let level_params = ind_val.common.level_params.clone();
        let ctor_ty = ctor_val.common.ty.clone();
        let mut cur_ty = oxilean_kernel::instantiate_level_params(&ctor_ty, &level_params, &levels);
        let struct_args: Vec<Expr> = oxilean_kernel::expr_util::get_app_args(struct_ty_whnf.expr())
            .into_iter()
            .cloned()
            .collect();
        for i in 0..ind_val.num_params as usize {
            match cur_ty {
                Expr::Pi(_, _, _, body) => {
                    let param = struct_args.get(i).cloned().unwrap_or(Expr::BVar(0));
                    cur_ty = oxilean_kernel::instantiate(&body, &param);
                }
                _ => {
                    let proj_ty = Expr::Sort(Level::zero());
                    let (_, mvar) = ctx.mk_fresh_expr_mvar(proj_ty, MetavarKind::Natural);
                    return Ok(mvar);
                }
            }
        }
        for j in 0..idx {
            match cur_ty {
                Expr::Pi(_, _, _, body) => {
                    let field_val =
                        Expr::Proj(ind_val.common.name.clone(), j, Box::new(inner.clone()));
                    cur_ty = oxilean_kernel::instantiate(&body, &field_val);
                }
                _ => {
                    let proj_ty = Expr::Sort(Level::zero());
                    let (_, mvar) = ctx.mk_fresh_expr_mvar(proj_ty, MetavarKind::Natural);
                    return Ok(mvar);
                }
            }
        }
        match cur_ty {
            Expr::Pi(_, _, dom, _) => Ok(*dom),
            _ => {
                let proj_ty = Expr::Sort(Level::zero());
                let (_, mvar) = ctx.mk_fresh_expr_mvar(proj_ty, MetavarKind::Natural);
                Ok(mvar)
            }
        }
    }
    /// Get the universe level of a type expression.
    ///
    /// If `ty` is `Sort(l)`, returns `l`.
    /// If `ty` is stuck on a metavar, creates a fresh level mvar.
    pub fn get_level(&mut self, ty: &Expr, ctx: &mut MetaContext) -> Result<Level, String> {
        let whnf = self.whnf.whnf(ty, ctx);
        match whnf.expr() {
            Expr::Sort(l) => Ok(ctx.instantiate_level_mvars(l)),
            _ => {
                if whnf.is_stuck() {
                    Ok(ctx.mk_fresh_level_mvar())
                } else {
                    Err(format!("Expected a sort/type, got: {}", whnf.expr()))
                }
            }
        }
    }
    /// Check if a type is a proposition (lives in Prop = Sort(0)).
    pub fn is_prop(&mut self, ty: &Expr, ctx: &mut MetaContext) -> Result<bool, String> {
        let ty_ty = self.infer_type_impl(ty, ctx, 0)?;
        let level = self.get_level(&ty_ty, ctx)?;
        let level_inst = ctx.instantiate_level_mvars(&level);
        Ok(level_inst.is_zero())
    }
    /// Check if a type is a Type (lives in Sort(succ(l)) for some l).
    pub fn is_type(&mut self, ty: &Expr, ctx: &mut MetaContext) -> Result<bool, String> {
        let ty_ty = self.infer_type_impl(ty, ctx, 0)?;
        let level = self.get_level(&ty_ty, ctx)?;
        let level_inst = ctx.instantiate_level_mvars(&level);
        Ok(!level_inst.is_zero())
    }
    /// Check if an expression is a proof (has a type that is a Prop).
    pub fn is_proof(&mut self, expr: &Expr, ctx: &mut MetaContext) -> Result<bool, String> {
        let ty = self.infer_type_impl(expr, ctx, 0)?;
        self.is_prop(&ty, ctx)
    }
    /// Ensure the type is a sort, returning the level.
    /// If not a sort, wraps the error.
    pub fn ensure_sort(&mut self, ty: &Expr, ctx: &mut MetaContext) -> Result<Level, String> {
        self.get_level(ty, ctx)
    }
    /// Ensure the type is a Pi type, returning (domain, codomain).
    /// If not a Pi, reduces to WHNF first.
    pub fn ensure_pi(&mut self, ty: &Expr, ctx: &mut MetaContext) -> Result<(Expr, Expr), String> {
        let whnf = self.whnf.whnf(ty, ctx);
        match whnf.expr() {
            Expr::Pi(_, _, domain, codomain) => {
                Ok((domain.as_ref().clone(), codomain.as_ref().clone()))
            }
            _ => {
                if whnf.is_stuck() {
                    let sort = Expr::Sort(Level::zero());
                    let (_, domain_mvar) =
                        ctx.mk_fresh_expr_mvar(sort.clone(), MetavarKind::Natural);
                    let (_, codomain_mvar) = ctx.mk_fresh_expr_mvar(sort, MetavarKind::Natural);
                    Ok((domain_mvar, codomain_mvar))
                } else {
                    Err(format!("Expected a Pi type, got: {}", whnf.expr()))
                }
            }
        }
    }
}
/// A simple type-inference error.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferError {
    /// An unbound bound variable was encountered.
    UnboundBVar(u32),
    /// A free variable was not found in the context.
    UnknownFVar(u64),
    /// A constant was not found in the environment.
    UnknownConst(String),
    /// The function in an application is not a Pi type.
    NotAFunction(String),
    /// A sort was expected but something else was found.
    ExpectedSort(String),
    /// Recursion depth exceeded.
    DepthExceeded,
    /// Other error with message.
    Other(String),
}
/// Type inference result including warnings.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InferResult {
    /// The inferred type.
    pub ty: Expr,
    /// Any warnings generated during inference.
    pub warnings: Vec<String>,
    /// Number of reduction steps taken.
    pub steps: u32,
}
impl InferResult {
    /// Create a simple result with no warnings.
    #[allow(dead_code)]
    pub fn ok(ty: Expr) -> Self {
        Self {
            ty,
            warnings: vec![],
            steps: 0,
        }
    }
    /// Create a result with warnings.
    #[allow(dead_code)]
    pub fn with_warnings(ty: Expr, warnings: Vec<String>, steps: u32) -> Self {
        Self {
            ty,
            warnings,
            steps,
        }
    }
    /// Check if there are any warnings.
    #[allow(dead_code)]
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}
/// A state machine controller for InferType.
#[allow(dead_code)]
pub struct InferTypeStateMachine {
    pub state: InferTypeState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl InferTypeStateMachine {
    pub fn new() -> Self {
        InferTypeStateMachine {
            state: InferTypeState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: InferTypeState) -> bool {
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
        self.transition_to(InferTypeState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(InferTypeState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(InferTypeState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(InferTypeState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
/// A result type for InferType analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum InferTypeResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl InferTypeResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, InferTypeResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, InferTypeResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, InferTypeResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, InferTypeResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            InferTypeResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            InferTypeResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            InferTypeResult::Ok(_) => 1.0,
            InferTypeResult::Err(_) => 0.0,
            InferTypeResult::Skipped => 0.0,
            InferTypeResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
#[allow(dead_code)]
pub struct InferTypeExtPass2300 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<InferTypeExtResult2300>,
}
impl InferTypeExtPass2300 {
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
    pub fn run(&mut self, input: &str) -> InferTypeExtResult2300 {
        if !self.enabled {
            return InferTypeExtResult2300::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            InferTypeExtResult2300::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            InferTypeExtResult2300::Ok(format!(
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
#[allow(dead_code)]
pub struct InferTypeExtDiag2300 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl InferTypeExtDiag2300 {
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
/// An extended utility type for InferType.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct InferTypeExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl InferTypeExt {
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
/// A sliding window accumulator for InferType.
#[allow(dead_code)]
pub struct InferTypeWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl InferTypeWindow {
    pub fn new(capacity: usize) -> Self {
        InferTypeWindow {
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
/// An extended map for InferType keys to values.
#[allow(dead_code)]
pub struct InferTypeExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> InferTypeExtMap<V> {
    pub fn new() -> Self {
        InferTypeExtMap {
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
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum InferTypeExtResult2300 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl InferTypeExtResult2300 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, InferTypeExtResult2300::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, InferTypeExtResult2300::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, InferTypeExtResult2300::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, InferTypeExtResult2300::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let InferTypeExtResult2300::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let InferTypeExtResult2300::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            InferTypeExtResult2300::Ok(_) => 1.0,
            InferTypeExtResult2300::Err(_) => 0.0,
            InferTypeExtResult2300::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            InferTypeExtResult2300::Skipped => 0.5,
        }
    }
}
/// A stack of local typing contexts for nested scopes.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct TypingStack {
    pub(super) frames: Vec<Vec<(String, Expr)>>,
}
#[allow(dead_code)]
impl TypingStack {
    /// Create an empty stack.
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new scope frame.
    pub fn push_frame(&mut self) {
        self.frames.push(Vec::new());
    }
    /// Pop the top scope frame, returning its bindings.
    pub fn pop_frame(&mut self) -> Vec<(String, Expr)> {
        self.frames.pop().unwrap_or_default()
    }
    /// Add a binding to the current (top) frame.
    pub fn bind(&mut self, name: String, ty: Expr) {
        if let Some(frame) = self.frames.last_mut() {
            frame.push((name, ty));
        }
    }
    /// Look up a binding by name (top frame first).
    pub fn lookup(&self, name: &str) -> Option<&Expr> {
        for frame in self.frames.iter().rev() {
            for (n, ty) in frame.iter().rev() {
                if n == name {
                    return Some(ty);
                }
            }
        }
        None
    }
    /// Current stack depth.
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    /// Total number of bindings across all frames.
    pub fn total_bindings(&self) -> usize {
        self.frames.iter().map(|f| f.len()).sum()
    }
    /// Check if empty (no frames or all frames empty).
    pub fn is_empty(&self) -> bool {
        self.total_bindings() == 0
    }
}
/// A state machine for InferType.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum InferTypeState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl InferTypeState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, InferTypeState::Complete | InferTypeState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, InferTypeState::Initial | InferTypeState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, InferTypeState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            InferTypeState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
#[allow(dead_code)]
pub struct InferTypeExtConfig2300 {
    pub(super) values: std::collections::HashMap<String, InferTypeExtConfigVal2300>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl InferTypeExtConfig2300 {
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
    pub fn set(&mut self, key: &str, value: InferTypeExtConfigVal2300) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&InferTypeExtConfigVal2300> {
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
        self.set(key, InferTypeExtConfigVal2300::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, InferTypeExtConfigVal2300::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, InferTypeExtConfigVal2300::Str(v.to_string()))
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
/// A diff for InferType analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct InferTypeDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl InferTypeDiff {
    pub fn new() -> Self {
        InferTypeDiff {
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
/// A typed slot for InferType configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum InferTypeConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl InferTypeConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            InferTypeConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            InferTypeConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            InferTypeConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            InferTypeConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            InferTypeConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            InferTypeConfigValue::Bool(_) => "bool",
            InferTypeConfigValue::Int(_) => "int",
            InferTypeConfigValue::Float(_) => "float",
            InferTypeConfigValue::Str(_) => "str",
            InferTypeConfigValue::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
pub struct InferTypeExtDiff2300 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl InferTypeExtDiff2300 {
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
/// A counter map for InferType frequency analysis.
#[allow(dead_code)]
pub struct InferTypeCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl InferTypeCounterMap {
    pub fn new() -> Self {
        InferTypeCounterMap {
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
/// Typed expression — a pair of (kernel expr, its type).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TypedExpr {
    /// The expression itself.
    pub expr: Expr,
    /// The type of the expression.
    pub ty: Expr,
}
impl TypedExpr {
    /// Create a new typed expression.
    #[allow(dead_code)]
    pub fn new(expr: Expr, ty: Expr) -> Self {
        Self { expr, ty }
    }
    /// Decompose into (expr, ty) tuple.
    #[allow(dead_code)]
    pub fn into_pair(self) -> (Expr, Expr) {
        (self.expr, self.ty)
    }
    /// Check if this is a proposition.
    #[allow(dead_code)]
    pub fn is_proof(&self) -> bool {
        is_prop_expr(&self.ty)
    }
    /// Check if the expression is of type Sort (i.e., it is a type).
    #[allow(dead_code)]
    pub fn is_type(&self) -> bool {
        matches!(& self.ty, Expr::Sort(l) if ! l.is_zero())
    }
}
/// Type annotation — pairs an expression with its inferred or declared type.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct TypeAnnotation {
    /// The expression.
    pub expr: Expr,
    /// The type of the expression.
    pub ty: Expr,
}
impl TypeAnnotation {
    /// Create a type annotation.
    #[allow(dead_code)]
    pub fn new(expr: Expr, ty: Expr) -> Self {
        Self { expr, ty }
    }
    /// Check if the type is Prop.
    #[allow(dead_code)]
    pub fn is_prop(&self) -> bool {
        is_prop_expr(&self.ty)
    }
    /// Check if the type is a Sort.
    #[allow(dead_code)]
    pub fn type_is_sort(&self) -> bool {
        matches!(self.ty, Expr::Sort(_))
    }
}
/// A diagnostic reporter for InferType.
#[allow(dead_code)]
pub struct InferTypeDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl InferTypeDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        InferTypeDiagnostics {
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
/// A pipeline of InferType analysis passes.
#[allow(dead_code)]
pub struct InferTypePipeline {
    pub passes: Vec<InferTypeAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl InferTypePipeline {
    pub fn new(name: &str) -> Self {
        InferTypePipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: InferTypeAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<InferTypeResult> {
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
/// A configuration store for InferType.
#[allow(dead_code)]
pub struct InferTypeConfig {
    pub values: std::collections::HashMap<String, InferTypeConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl InferTypeConfig {
    pub fn new() -> Self {
        InferTypeConfig {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: InferTypeConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&InferTypeConfigValue> {
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
        self.set(key, InferTypeConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, InferTypeConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, InferTypeConfigValue::Str(v.to_string()))
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
pub struct InferTypeExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl InferTypeExtUtil {
    pub fn new(key: &str) -> Self {
        InferTypeExtUtil {
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
