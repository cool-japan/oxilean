//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfParam, LcnfVarId};
use std::collections::HashMap;

/// Statistics for the extended optimization passes.
#[derive(Debug, Clone, Default)]
pub struct ExtendedPassReport {
    /// Number of let-bindings floated out of case alternatives.
    pub lets_floated: usize,
    /// Number of case-of-case eliminations performed.
    pub case_of_case_elims: usize,
    /// Number of case-of-known-constructor eliminations performed.
    pub case_of_known_ctor_elims: usize,
    /// Number of dead let-bindings eliminated.
    pub dead_lets_eliminated: usize,
    /// Number of trivially true cases eliminated.
    pub literal_case_elims: usize,
}
/// A hint produced by the beta/eta pass.
#[derive(Debug, Clone)]
pub enum OptHint {
    MergeCurriedApp {
        intermediate: LcnfVarId,
        outer_func: String,
    },
    InlineCandidate {
        func_name: String,
        cost: usize,
    },
    EtaReducible {
        func_name: String,
    },
}
/// Per-function statistics for a module-level pass.
#[derive(Debug, Clone, Default)]
pub struct ModuleOptStats {
    pub total_beta: usize,
    pub total_eta: usize,
    pub total_dead_lets: usize,
    pub total_cokc: usize,
    pub functions_processed: usize,
}
/// Environment tracking which variables are bound to specific constructors.
#[derive(Debug, Clone, Default)]
pub struct CtorEnv {
    pub known: HashMap<LcnfVarId, (String, u16)>,
}
impl CtorEnv {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record(&mut self, id: LcnfVarId, name: String, tag: u16) {
        self.known.insert(id, (name, tag));
    }
    #[allow(dead_code)]
    pub fn get(&self, id: &LcnfVarId) -> Option<&(String, u16)> {
        self.known.get(id)
    }
}
/// Configuration for the extended let-floating and case-of-case passes.
#[derive(Debug, Clone)]
pub struct ExtendedPassConfig {
    /// Enable let-floating out of case alternatives.
    pub do_let_float: bool,
    /// Enable case-of-case inlining.
    pub do_case_of_case: bool,
    /// Enable case-of-known-constructor elimination.
    pub do_case_of_known_ctor: bool,
    /// Enable dead let elimination.
    pub do_dead_let: bool,
    /// Maximum number of case-of-case inlining steps per function.
    pub max_case_of_case: usize,
    /// Maximum let chain depth for let-floating.
    pub max_let_float_depth: usize,
}
/// A flattened let-binding for analysis.
#[derive(Debug, Clone)]
pub struct LetBinding {
    pub id: LcnfVarId,
    pub name: String,
    pub ty: crate::lcnf::LcnfType,
    pub value: LcnfLetValue,
}
/// Configuration for the beta/eta reduction pass.
#[derive(Debug, Clone)]
pub struct BetaEtaConfig {
    /// Maximum recursion depth when traversing nested let-chains.
    pub max_depth: usize,
    /// Enable eta reduction.
    pub do_eta: bool,
    /// Enable beta reduction (constant-beta folding).
    pub do_beta: bool,
}
/// A counter for generating fresh variable IDs.
#[derive(Debug, Clone)]
pub struct FreshIdGen {
    pub next: u64,
}
impl FreshIdGen {
    #[allow(dead_code)]
    pub fn new(start: u64) -> Self {
        Self { next: start }
    }
    #[allow(dead_code)]
    pub fn fresh(&mut self) -> LcnfVarId {
        let id = LcnfVarId(self.next);
        self.next += 1;
        id
    }
}
/// Statistics collected during a beta/eta pass.
#[derive(Debug, Clone, Default)]
pub struct BetaEtaReport {
    /// Number of beta reductions performed (copy-propagation of trivial lets).
    pub beta_reductions: usize,
    /// Number of eta reductions performed.
    pub eta_reductions: usize,
    /// Number of curried-application opportunities detected.
    pub curried_opportunities: usize,
}
/// Represents a compile-time known value for constant folding.
#[derive(Debug, Clone, PartialEq)]
pub enum KnownValue {
    Nat(u64),
    Str(String),
    Erased,
}
/// Beta/Eta reduction pass over a single `LcnfFunDecl`.
pub struct BetaEtaPass {
    /// Configuration for this pass.
    pub config: BetaEtaConfig,
    /// Accumulated report for the most recent `run` call.
    pub report: BetaEtaReport,
}
impl BetaEtaPass {
    /// Create a new pass with the given configuration.
    pub fn new(config: BetaEtaConfig) -> Self {
        BetaEtaPass {
            config,
            report: BetaEtaReport::default(),
        }
    }
    /// Run the pass on a single function declaration.
    ///
    /// Mutates `decl` in place and updates `self.report`.
    pub fn run(&mut self, decl: &mut LcnfFunDecl) {
        self.report = BetaEtaReport::default();
        if self.config.do_beta {
            let mut env: HashMap<LcnfVarId, LcnfLetValue> = HashMap::new();
            Self::beta_reduce_expr(
                &mut decl.body,
                &mut env,
                0,
                self.config.max_depth,
                &mut self.report,
            );
        }
        if self.config.do_eta {
            self.try_eta_reduce(decl);
        }
        Self::count_curried(&decl.body, &mut self.report);
    }
    /// Walk an expression, performing beta reduction:
    /// - Copy-propagate `let x = FVar(y)` by substituting `y` for `x`.
    /// - Remove `let x = Erased` bindings that are never needed.
    pub(super) fn beta_reduce_expr(
        expr: &mut LcnfExpr,
        env: &mut HashMap<LcnfVarId, LcnfLetValue>,
        depth: usize,
        max_depth: usize,
        report: &mut BetaEtaReport,
    ) {
        if depth > max_depth {
            return;
        }
        match expr {
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                Self::subst_arg_in_value(value, env);
                if let LcnfLetValue::FVar(src) = value {
                    let src_val = if let Some(v) = env.get(src) {
                        v.clone()
                    } else {
                        LcnfLetValue::FVar(*src)
                    };
                    env.insert(*id, src_val);
                    report.beta_reductions += 1;
                } else {
                    env.insert(*id, value.clone());
                }
                Self::beta_reduce_expr(body, env, depth + 1, max_depth, report);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    let mut child_env = env.clone();
                    Self::beta_reduce_expr(
                        &mut alt.body,
                        &mut child_env,
                        depth + 1,
                        max_depth,
                        report,
                    );
                }
                if let Some(def) = default {
                    let mut child_env = env.clone();
                    Self::beta_reduce_expr(def, &mut child_env, depth + 1, max_depth, report);
                }
            }
            LcnfExpr::Return(arg) => {
                Self::subst_arg(arg, env);
            }
            LcnfExpr::TailCall(func, args) => {
                Self::subst_arg(func, env);
                for a in args.iter_mut() {
                    Self::subst_arg(a, env);
                }
            }
            LcnfExpr::Unreachable => {}
        }
    }
    /// Substitute known `FVar` mappings inside an `LcnfLetValue`.
    pub(super) fn subst_arg_in_value(
        value: &mut LcnfLetValue,
        env: &HashMap<LcnfVarId, LcnfLetValue>,
    ) {
        match value {
            LcnfLetValue::App(func, args) => {
                Self::subst_arg(func, env);
                for a in args.iter_mut() {
                    Self::subst_arg(a, env);
                }
            }
            LcnfLetValue::Ctor(_, _, args) => {
                for a in args.iter_mut() {
                    Self::subst_arg(a, env);
                }
            }
            LcnfLetValue::Reuse(_, _, _, args) => {
                for a in args.iter_mut() {
                    Self::subst_arg(a, env);
                }
            }
            LcnfLetValue::FVar(id) => {
                if let Some(LcnfLetValue::FVar(resolved)) = env.get(id) {
                    *id = *resolved;
                }
            }
            LcnfLetValue::Proj(_, _, var) => {
                if let Some(LcnfLetValue::FVar(resolved)) = env.get(var) {
                    *var = *resolved;
                }
            }
            LcnfLetValue::Reset(var) => {
                if let Some(LcnfLetValue::FVar(resolved)) = env.get(var) {
                    *var = *resolved;
                }
            }
            LcnfLetValue::Lit(_) | LcnfLetValue::Erased => {}
        }
    }
    /// Substitute a `Var` arg to its canonical form if in `env`.
    pub(super) fn subst_arg(arg: &mut LcnfArg, env: &HashMap<LcnfVarId, LcnfLetValue>) {
        if let LcnfArg::Var(id) = arg {
            if let Some(LcnfLetValue::FVar(resolved)) = env.get(id) {
                *id = *resolved;
            }
        }
    }
    /// Attempt to eta-reduce a function declaration.
    ///
    /// The pattern is:
    /// ```text
    /// fun (p0, p1, .., pN) {
    ///   let r = f(p0, p1, .., pN);
    ///   return r
    /// }
    /// ```
    /// gets collapsed to:
    /// ```text
    /// fun (p0, .., pN) { tailcall f(p0, .., pN) }
    /// ```
    pub(super) fn try_eta_reduce(&mut self, decl: &mut LcnfFunDecl) {
        let params: &[LcnfParam] = &decl.params;
        if params.is_empty() {
            return;
        }
        let param_ids: Vec<LcnfVarId> = params.iter().map(|p| p.id).collect();
        if let LcnfExpr::Let {
            id,
            value: LcnfLetValue::App(func, args),
            body,
            ..
        } = &decl.body
        {
            let result_id = *id;
            if let LcnfExpr::Return(LcnfArg::Var(ret_id)) = body.as_ref() {
                if *ret_id != result_id {
                    return;
                }
            } else {
                return;
            }
            if args.len() != param_ids.len() {
                return;
            }
            let args_match = args
                .iter()
                .zip(param_ids.iter())
                .all(|(a, pid)| matches!(a, LcnfArg::Var(v) if v == pid));
            if !args_match {
                return;
            }
            let new_body = LcnfExpr::TailCall(func.clone(), args.clone());
            decl.body = new_body;
            self.report.eta_reductions += 1;
        }
    }
    /// Count consecutive single-argument applications that could be merged.
    ///
    /// Pattern: `let t = f(a); let r = t(b)` (t used exactly once, immediately).
    pub(super) fn count_curried(expr: &LcnfExpr, report: &mut BetaEtaReport) {
        match expr {
            LcnfExpr::Let {
                id,
                value: LcnfLetValue::App(_, args),
                body,
                ..
            } if args.len() == 1 => {
                if let LcnfExpr::Let {
                    value: LcnfLetValue::App(LcnfArg::Var(callee), _),
                    body: inner_body,
                    ..
                } = body.as_ref()
                {
                    if *callee == *id {
                        report.curried_opportunities += 1;
                    }
                    Self::count_curried(inner_body, report);
                } else {
                    Self::count_curried(body, report);
                }
            }
            LcnfExpr::Let { body, .. } => {
                Self::count_curried(body, report);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    Self::count_curried(&alt.body, report);
                }
                if let Some(def) = default {
                    Self::count_curried(def, report);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => {}
        }
    }
}
/// Environment mapping variable IDs to their known compile-time values.
#[derive(Debug, Clone, Default)]
pub struct LitEnv {
    pub known: HashMap<LcnfVarId, KnownValue>,
}
impl LitEnv {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record_nat(&mut self, id: LcnfVarId, n: u64) {
        self.known.insert(id, KnownValue::Nat(n));
    }
    #[allow(dead_code)]
    pub fn record_str(&mut self, id: LcnfVarId, s: String) {
        self.known.insert(id, KnownValue::Str(s));
    }
    #[allow(dead_code)]
    pub fn get(&self, id: &LcnfVarId) -> Option<&KnownValue> {
        self.known.get(id)
    }
}
/// A summary of which parameters of a function are actually used.
#[derive(Debug, Clone)]
pub struct ParamUsageSummary {
    pub func_name: String,
    pub used: Vec<bool>,
}
/// Maximum known arity of a function.
#[derive(Debug, Clone, Default)]
pub struct ArityMap {
    pub arities: HashMap<String, usize>,
}
impl ArityMap {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    #[allow(dead_code)]
    pub fn record(&mut self, name: String, arity: usize) {
        self.arities.insert(name, arity);
    }
    #[allow(dead_code)]
    pub fn get(&self, name: &str) -> Option<usize> {
        self.arities.get(name).copied()
    }
    #[allow(dead_code)]
    pub fn from_decls(decls: &[LcnfFunDecl]) -> Self {
        let mut m = Self::new();
        for d in decls {
            m.record(d.name.clone(), d.params.len());
        }
        m
    }
}
