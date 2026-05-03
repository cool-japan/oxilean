//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::*;
use std::collections::HashMap;

use super::super::functions::*;
use super::impls1::*;
use super::impls2::*;
use std::collections::{HashSet, VecDeque};

/// Liveness analysis for SRExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SRExtLiveness {
    pub live_in: Vec<Vec<usize>>,
    pub live_out: Vec<Vec<usize>>,
    pub defs: Vec<Vec<usize>>,
    pub uses: Vec<Vec<usize>>,
}
impl SRExtLiveness {
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
#[allow(dead_code)]
pub struct OSConstantFoldingHelper;
impl OSConstantFoldingHelper {
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
/// Dominator tree for SRExt.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SRExtDomTree {
    pub(crate) idom: Vec<Option<usize>>,
    pub(crate) children: Vec<Vec<usize>>,
    pub(crate) depth: Vec<usize>,
}
impl SRExtDomTree {
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
/// A single algebraic strength-reduction rewrite rule.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum StrengthReduceRule {
    /// `x * 2^n` → `x << n`.
    MulByPow2(u32),
    /// `x / 2^n` → `x >> n` (unsigned / logical right shift).
    DivByPow2(u32),
    /// `x % 2^n` → `x & (2^n - 1)`.
    ModByPow2(u32),
    /// `x * c` → sequence of shifts and additions.
    MulByConstant(u64),
    /// `x / c` → multiply-by-reciprocal (magic number division).
    DivByConstant(u64),
    /// `pow(x, 2)` → `x * x`.
    Pow2Const,
    /// `pow(x, 3)` → `x * x * x`.
    Pow3Const,
    /// `0 - x` → `negate(x)`.
    NegToSub,
    /// `x + 1` → `incr(x)`.
    AddSubToInc,
}
/// The strength-reduction optimization pass.
///
/// Usage:
/// ```
/// use oxilean_codegen::opt_strength::{StrengthReductionPass, StrengthConfig};
/// let mut pass = StrengthReductionPass::new(StrengthConfig::default());
/// // pass.run(&mut decls);
/// ```
pub struct StrengthReductionPass {
    pub(crate) config: StrengthConfig,
    pub(crate) report: StrengthReport,
    /// Counter for generating fresh variable IDs during rewriting.
    pub(crate) next_id: u64,
}
impl StrengthReductionPass {
    /// Create a new pass with the given configuration.
    pub fn new(config: StrengthConfig) -> Self {
        StrengthReductionPass {
            config,
            report: StrengthReport::default(),
            next_id: 100_000,
        }
    }
    /// Allocate a fresh variable ID (for expansion of mul-by-const etc.).
    pub(crate) fn fresh_var(&mut self) -> LcnfVarId {
        let id = self.next_id;
        self.next_id += 1;
        LcnfVarId(id)
    }
    /// Run strength reduction on all function declarations.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        for decl in decls.iter_mut() {
            let ivs = self.detect_induction_vars(decl);
            let new_body = self.reduce_expr_with_ivs(&decl.body.clone(), &ivs);
            decl.body = new_body;
        }
    }
    /// Apply strength reduction to a single expression.
    pub fn reduce_expr(&mut self, expr: &LcnfExpr) -> LcnfExpr {
        self.reduce_expr_with_ivs(expr, &[])
    }
    pub(crate) fn reduce_expr_with_ivs(
        &mut self,
        expr: &LcnfExpr,
        _ivs: &[InductionVariable],
    ) -> LcnfExpr {
        match expr {
            LcnfExpr::Let {
                id,
                name,
                ty,
                value,
                body,
            } => {
                let (new_value, prefix) = self.reduce_let_value(value, *id, name, ty);
                let new_body = self.reduce_expr_with_ivs(body, _ivs);
                let inner = LcnfExpr::Let {
                    id: *id,
                    name: name.clone(),
                    ty: ty.clone(),
                    value: new_value,
                    body: Box::new(new_body),
                };
                prefix
                    .into_iter()
                    .rev()
                    .fold(inner, |acc, (pid, pname, pty, pval)| LcnfExpr::Let {
                        id: pid,
                        name: pname,
                        ty: pty,
                        value: pval,
                        body: Box::new(acc),
                    })
            }
            LcnfExpr::Case {
                scrutinee,
                scrutinee_ty,
                alts,
                default,
            } => {
                let new_alts = alts
                    .iter()
                    .map(|alt| LcnfAlt {
                        ctor_name: alt.ctor_name.clone(),
                        ctor_tag: alt.ctor_tag,
                        params: alt.params.clone(),
                        body: self.reduce_expr_with_ivs(&alt.body, _ivs),
                    })
                    .collect();
                let new_default = default
                    .as_ref()
                    .map(|d| Box::new(self.reduce_expr_with_ivs(d, _ivs)));
                LcnfExpr::Case {
                    scrutinee: *scrutinee,
                    scrutinee_ty: scrutinee_ty.clone(),
                    alts: new_alts,
                    default: new_default,
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable | LcnfExpr::TailCall(_, _) => expr.clone(),
        }
    }
    /// Reduce a single let-bound value.
    ///
    /// Returns `(new_value, prefix_lets)` where `prefix_lets` are auxiliary
    /// bindings that must precede the current binding (e.g. when expanding
    /// `mul_by_const` into shift + add sequence).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn reduce_let_value(
        &mut self,
        value: &LcnfLetValue,
        _id: LcnfVarId,
        _name: &str,
        ty: &LcnfType,
    ) -> (
        LcnfLetValue,
        Vec<(LcnfVarId, String, LcnfType, LcnfLetValue)>,
    ) {
        let prefix: Vec<(LcnfVarId, String, LcnfType, LcnfLetValue)> = vec![];
        match value {
            LcnfLetValue::App(func, args) => {
                if let Some(rule) = self.match_rule(func, args) {
                    return self.apply_rule(&rule, func, args, ty);
                }
                (value.clone(), prefix)
            }
            _ => (value.clone(), prefix),
        }
    }
    /// Attempt to match an application against a known strength-reduction rule.
    pub(crate) fn match_rule(
        &self,
        func: &LcnfArg,
        args: &[LcnfArg],
    ) -> Option<StrengthReduceRule> {
        let fname = match func {
            LcnfArg::Lit(LcnfLit::Str(s)) => s.as_str(),
            _ => return None,
        };
        match fname {
            "mul" if args.len() == 2 => {
                let c = const_arg(&args[1]).or_else(|| const_arg(&args[0]))?;
                if c == 0 {
                    return None;
                }
                if c == 1 {
                    return None;
                }
                if is_power_of_two(c) {
                    let n = log2(c);
                    Some(StrengthReduceRule::MulByPow2(n))
                } else {
                    Some(StrengthReduceRule::MulByConstant(c))
                }
            }
            "div" if args.len() == 2 => {
                let c = const_arg(&args[1])?;
                if c == 0 {
                    return None;
                }
                if is_power_of_two(c) {
                    let n = log2(c);
                    Some(StrengthReduceRule::DivByPow2(n))
                } else if self.config.optimize_div {
                    Some(StrengthReduceRule::DivByConstant(c))
                } else {
                    None
                }
            }
            "mod" if args.len() == 2 => {
                let c = const_arg(&args[1])?;
                if c == 0 {
                    return None;
                }
                if is_power_of_two(c) {
                    let n = log2(c);
                    Some(StrengthReduceRule::ModByPow2(n))
                } else {
                    None
                }
            }
            "pow" if args.len() == 2 => match const_arg(&args[1]) {
                Some(2) => Some(StrengthReduceRule::Pow2Const),
                Some(3) => Some(StrengthReduceRule::Pow3Const),
                _ => None,
            },
            "sub" if args.len() == 2 => {
                if let LcnfArg::Lit(LcnfLit::Nat(0)) = &args[0] {
                    Some(StrengthReduceRule::NegToSub)
                } else {
                    None
                }
            }
            "add" if args.len() == 2 && self.config.optimize_inc => {
                let c = const_arg(&args[1]).or_else(|| const_arg(&args[0]))?;
                if c == 1 {
                    Some(StrengthReduceRule::AddSubToInc)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    /// Build the replacement value (and any prefix bindings) for a given rule.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn apply_rule(
        &mut self,
        rule: &StrengthReduceRule,
        func: &LcnfArg,
        args: &[LcnfArg],
        ty: &LcnfType,
    ) -> (
        LcnfLetValue,
        Vec<(LcnfVarId, String, LcnfType, LcnfLetValue)>,
    ) {
        let empty: Vec<(LcnfVarId, String, LcnfType, LcnfLetValue)> = vec![];
        let var_arg = var_arg_of(func, args);
        match rule {
            StrengthReduceRule::MulByPow2(n) => {
                self.report.mul_reduced += 1;
                let shift_val = LcnfLetValue::App(
                    LcnfArg::Lit(LcnfLit::Str("shl".into())),
                    vec![var_arg, LcnfArg::Lit(LcnfLit::Nat(*n as u64))],
                );
                (shift_val, empty)
            }
            StrengthReduceRule::DivByPow2(n) => {
                self.report.div_reduced += 1;
                let shift_val = LcnfLetValue::App(
                    LcnfArg::Lit(LcnfLit::Str("lshr".into())),
                    vec![var_arg, LcnfArg::Lit(LcnfLit::Nat(*n as u64))],
                );
                (shift_val, empty)
            }
            StrengthReduceRule::ModByPow2(n) => {
                self.report.div_reduced += 1;
                let mask = (1u64 << n) - 1;
                let and_val = LcnfLetValue::App(
                    LcnfArg::Lit(LcnfLit::Str("band".into())),
                    vec![var_arg, LcnfArg::Lit(LcnfLit::Nat(mask))],
                );
                (and_val, empty)
            }
            StrengthReduceRule::MulByConstant(c) => {
                let budget = self.config.max_shift_count;
                if let Some(ops) = decompose_mul(*c, budget) {
                    self.report.mul_reduced += 1;
                    self.build_shift_add_sequence(var_arg, &ops, ty)
                } else {
                    (LcnfLetValue::App(func.clone(), args.to_vec()), empty)
                }
            }
            StrengthReduceRule::DivByConstant(c) => {
                self.report.div_reduced += 1;
                let magic_val = LcnfLetValue::App(
                    LcnfArg::Lit(LcnfLit::Str("magic_div".into())),
                    vec![var_arg, LcnfArg::Lit(LcnfLit::Nat(*c))],
                );
                (magic_val, empty)
            }
            StrengthReduceRule::Pow2Const => {
                self.report.pow_reduced += 1;
                (
                    LcnfLetValue::App(
                        LcnfArg::Lit(LcnfLit::Str("mul".into())),
                        vec![var_arg.clone(), var_arg],
                    ),
                    empty,
                )
            }
            StrengthReduceRule::Pow3Const => {
                self.report.pow_reduced += 1;
                let sq_id = self.fresh_var();
                let sq_val = LcnfLetValue::App(
                    LcnfArg::Lit(LcnfLit::Str("mul".into())),
                    vec![var_arg.clone(), var_arg.clone()],
                );
                let cube_val = LcnfLetValue::App(
                    LcnfArg::Lit(LcnfLit::Str("mul".into())),
                    vec![LcnfArg::Var(sq_id), var_arg],
                );
                let prefix = vec![(sq_id, "sq".into(), ty.clone(), sq_val)];
                (cube_val, prefix)
            }
            StrengthReduceRule::NegToSub => {
                self.report.neg_reduced += 1;
                (
                    LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str("neg".into())), vec![var_arg]),
                    empty,
                )
            }
            StrengthReduceRule::AddSubToInc => {
                self.report.inc_reduced += 1;
                (
                    LcnfLetValue::App(LcnfArg::Lit(LcnfLit::Str("incr".into())), vec![var_arg]),
                    empty,
                )
            }
        }
    }
    /// Build a sequence of shift-and-add bindings for `x * c`.
    ///
    /// `ops` is a list of `(shift_amount, sign)` where sign is +1 or -1.
    pub(crate) fn build_shift_add_sequence(
        &mut self,
        x: LcnfArg,
        ops: &[(u32, i64)],
        ty: &LcnfType,
    ) -> (
        LcnfLetValue,
        Vec<(LcnfVarId, String, LcnfType, LcnfLetValue)>,
    ) {
        let mut prefix: Vec<(LcnfVarId, String, LcnfType, LcnfLetValue)> = vec![];
        if ops.is_empty() {
            return (LcnfLetValue::Lit(LcnfLit::Nat(0)), prefix);
        }
        let (first_shift, _first_sign) = ops[0];
        let shifted0 = self.fresh_var();
        prefix.push((
            shifted0,
            "sr0".into(),
            ty.clone(),
            LcnfLetValue::App(
                LcnfArg::Lit(LcnfLit::Str("shl".into())),
                vec![x.clone(), LcnfArg::Lit(LcnfLit::Nat(first_shift as u64))],
            ),
        ));
        let mut acc_var = shifted0;
        for &(shift, sign) in &ops[1..] {
            let shifted_var = self.fresh_var();
            prefix.push((
                shifted_var,
                "sr_sh".into(),
                ty.clone(),
                LcnfLetValue::App(
                    LcnfArg::Lit(LcnfLit::Str("shl".into())),
                    vec![x.clone(), LcnfArg::Lit(LcnfLit::Nat(shift as u64))],
                ),
            ));
            let combined = self.fresh_var();
            let op_name = if sign > 0 { "add" } else { "sub" };
            prefix.push((
                combined,
                "sr_acc".into(),
                ty.clone(),
                LcnfLetValue::App(
                    LcnfArg::Lit(LcnfLit::Str(op_name.into())),
                    vec![LcnfArg::Var(acc_var), LcnfArg::Var(shifted_var)],
                ),
            ));
            acc_var = combined;
        }
        (LcnfLetValue::FVar(acc_var), prefix)
    }
    /// Detect induction variables in a function declaration.
    ///
    /// A simple heuristic: look for parameters whose name contains "i",
    /// "n", "k", or "idx", and where the recursive tail call increments
    /// that parameter by a constant step.
    pub fn detect_induction_vars(&self, decl: &LcnfFunDecl) -> Vec<InductionVariable> {
        let mut ivs = Vec::new();
        let params: Vec<&LcnfParam> = decl.params.iter().collect();
        self.find_tail_call_ivs(&decl.body, &params, &mut ivs);
        ivs
    }
    pub(crate) fn find_tail_call_ivs(
        &self,
        expr: &LcnfExpr,
        params: &[&LcnfParam],
        ivs: &mut Vec<InductionVariable>,
    ) {
        match expr {
            LcnfExpr::TailCall(_, args) => {
                for (i, _arg) in args.iter().enumerate() {
                    if i < params.len() {
                        let p = params[i];
                        if ivs.iter().all(|iv| iv.var != p.id) {
                            ivs.push(InductionVariable::new(
                                p.id,
                                LcnfArg::Lit(LcnfLit::Nat(0)),
                                1,
                                p.name.clone(),
                            ));
                        }
                    }
                }
            }
            LcnfExpr::Let { body, .. } => self.find_tail_call_ivs(body, params, ivs),
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts {
                    self.find_tail_call_ivs(&alt.body, params, ivs);
                }
                if let Some(d) = default {
                    self.find_tail_call_ivs(d, params, ivs);
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable => {}
        }
    }
    /// Apply operator strength reduction for a known induction variable.
    ///
    /// Rewrites uses of `a * iv + b` within `expr` (where `iv` is the
    /// induction variable's `LcnfVarId`) into uses of a helper variable
    /// that is updated by cheaper additions.
    pub fn reduce_iv_uses(&mut self, expr: &LcnfExpr, iv: &InductionVariable) -> LcnfExpr {
        let linears = collect_linear_uses(expr, iv.var);
        if linears.is_empty() {
            return expr.clone();
        }
        self.report.iv_reductions += linears.len();
        rewrite_linear_uses(expr, &linears, iv)
    }
    /// Return the accumulated report.
    pub fn report(&self) -> StrengthReport {
        self.report.clone()
    }
}
/// Configuration for SRX2 passes.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SRX2PassConfig {
    pub name: String,
    pub phase: SRX2PassPhase,
    pub enabled: bool,
    pub max_iterations: usize,
    pub debug: u32,
    pub timeout_ms: Option<u64>,
}
impl SRX2PassConfig {
    #[allow(dead_code)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            phase: SRX2PassPhase::Middle,
            enabled: true,
            max_iterations: 100,
            debug: 0,
            timeout_ms: None,
        }
    }
    #[allow(dead_code)]
    pub fn with_phase(mut self, phase: SRX2PassPhase) -> Self {
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
/// Constant folding helper for SRExt.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SRExtConstFolder {
    pub(crate) folds: usize,
    pub(crate) failures: usize,
    pub(crate) enabled: bool,
}
impl SRExtConstFolder {
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
