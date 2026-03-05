//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use super::functions::*;
use crate::discr_tree::DiscrTree;
use oxilean_kernel::{BinderInfo, Expr, Literal, Name};
use std::collections::{HashMap, HashSet};

/// An extended utility type for SimpEngine.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SimpEngineExt {
    /// A tag for identifying this utility instance.
    pub tag: u32,
    /// An optional description string.
    pub description: Option<String>,
}
#[allow(dead_code)]
impl SimpEngineExt {
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
/// A state machine for SimpEngine.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SimpEngineState {
    Initial,
    Running,
    Paused,
    Complete,
    Failed(String),
}
#[allow(dead_code)]
impl SimpEngineState {
    pub fn is_terminal(&self) -> bool {
        matches!(self, SimpEngineState::Complete | SimpEngineState::Failed(_))
    }
    pub fn can_run(&self) -> bool {
        matches!(self, SimpEngineState::Initial | SimpEngineState::Paused)
    }
    pub fn is_running(&self) -> bool {
        matches!(self, SimpEngineState::Running)
    }
    pub fn error_msg(&self) -> Option<&str> {
        match self {
            SimpEngineState::Failed(s) => Some(s),
            _ => None,
        }
    }
}
/// A result type for SimpEngine analysis.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum SimpEngineResult {
    Ok(String),
    Err(String),
    Partial { done: usize, total: usize },
    Skipped,
}
#[allow(dead_code)]
impl SimpEngineResult {
    pub fn is_ok(&self) -> bool {
        matches!(self, SimpEngineResult::Ok(_))
    }
    pub fn is_err(&self) -> bool {
        matches!(self, SimpEngineResult::Err(_))
    }
    pub fn is_partial(&self) -> bool {
        matches!(self, SimpEngineResult::Partial { .. })
    }
    pub fn is_skipped(&self) -> bool {
        matches!(self, SimpEngineResult::Skipped)
    }
    pub fn ok_msg(&self) -> Option<&str> {
        match self {
            SimpEngineResult::Ok(s) => Some(s),
            _ => None,
        }
    }
    pub fn err_msg(&self) -> Option<&str> {
        match self {
            SimpEngineResult::Err(s) => Some(s),
            _ => None,
        }
    }
    pub fn progress(&self) -> f64 {
        match self {
            SimpEngineResult::Ok(_) => 1.0,
            SimpEngineResult::Err(_) => 0.0,
            SimpEngineResult::Skipped => 0.0,
            SimpEngineResult::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
        }
    }
}
/// A sliding window accumulator for SimpEngine.
#[allow(dead_code)]
pub struct SimpEngineWindow {
    pub buffer: std::collections::VecDeque<f64>,
    pub capacity: usize,
    pub running_sum: f64,
}
#[allow(dead_code)]
impl SimpEngineWindow {
    pub fn new(capacity: usize) -> Self {
        SimpEngineWindow {
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
/// A builder pattern for SimpEngine.
#[allow(dead_code)]
pub struct SimpEngineBuilder {
    pub name: String,
    pub items: Vec<String>,
    pub config: std::collections::HashMap<String, String>,
}
#[allow(dead_code)]
impl SimpEngineBuilder {
    pub fn new(name: &str) -> Self {
        SimpEngineBuilder {
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
/// An extended map for SimpEngine keys to values.
#[allow(dead_code)]
pub struct SimpEngineExtMap<V> {
    pub data: std::collections::HashMap<String, V>,
    pub default_key: Option<String>,
}
#[allow(dead_code)]
impl<V: Clone + Default> SimpEngineExtMap<V> {
    pub fn new() -> Self {
        SimpEngineExtMap {
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
/// A counter map for SimpEngine frequency analysis.
#[allow(dead_code)]
pub struct SimpEngineCounterMap {
    pub counts: std::collections::HashMap<String, usize>,
    pub total: usize,
}
#[allow(dead_code)]
impl SimpEngineCounterMap {
    pub fn new() -> Self {
        SimpEngineCounterMap {
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
/// Main simplification engine.
pub struct SimpEngine {
    /// Simp lemma database.
    pub lemmas: SimpLemmaDb,
    /// Configuration.
    pub config: SimpEngineConfig,
    /// Statistics.
    pub stats: SimpStats,
    /// Cache for simplified expressions.
    pub(super) cache: HashMap<String, SimpResult>,
}
impl SimpEngine {
    /// Create a new simp engine with default configuration.
    pub fn new() -> Self {
        Self {
            lemmas: SimpLemmaDb::new(),
            config: SimpEngineConfig::default(),
            stats: SimpStats::new(),
            cache: HashMap::new(),
        }
    }
    /// Create a new simp engine with custom configuration.
    pub fn with_config(config: SimpEngineConfig) -> Self {
        Self {
            lemmas: SimpLemmaDb::new(),
            config,
            stats: SimpStats::new(),
            cache: HashMap::new(),
        }
    }
    /// Add a simp lemma.
    pub fn add_lemma(&mut self, name: Name, entry: SimpLemmaEntry) {
        self.lemmas.add_lemma(name, entry);
    }
    /// Simplify an expression.
    pub fn simp(&mut self, expr: &Expr) -> SimpResult {
        self.cache.clear();
        self.stats = SimpStats::new();
        let mut ctx = SimpContext::new(expr.clone());
        self.simp_main(&mut ctx, expr)
    }
    /// Main simplification routine.
    fn simp_main(&mut self, ctx: &mut SimpContext, expr: &Expr) -> SimpResult {
        let expr_str = format!("{:?}", expr);
        if let Some(cached) = self.cache.get(&expr_str) {
            self.stats.record_cache_hit();
            return cached.clone();
        }
        if ctx.depth >= self.config.max_depth {
            return SimpResult::unchanged(expr.clone());
        }
        if !ctx.record_visit(&expr_str) {
            return SimpResult::unchanged(expr.clone());
        }
        ctx.inc_depth();
        self.stats.update_depth_max(ctx.depth);
        if let Some(result) = self.try_simp_lemmas(ctx, expr) {
            ctx.dec_depth();
            self.cache.insert(expr_str, result.clone());
            return result;
        }
        let congr_result = self.apply_congruence(ctx, expr);
        if congr_result.changed {
            ctx.dec_depth();
            self.cache.insert(expr_str, congr_result.clone());
            return congr_result;
        }
        if self.config.norm_num {
            if let Some(result) = self.norm_num(expr) {
                self.stats.record_rewrite();
                ctx.dec_depth();
                self.cache.insert(expr_str, result.clone());
                return result;
            }
        }
        ctx.dec_depth();
        let result = SimpResult::unchanged(expr.clone());
        self.cache.insert(expr_str, result.clone());
        result
    }
    /// Try to apply simp lemmas to the expression.
    fn try_simp_lemmas(&mut self, ctx: &mut SimpContext, expr: &Expr) -> Option<SimpResult> {
        let candidates = self.lemmas.find_candidates(expr);
        for entry in candidates {
            if ctx.rewrite_count >= self.config.max_steps {
                break;
            }
            if entry.conditional {
                if let Some(result) = self.try_conditional_rewrite(ctx, expr, &entry) {
                    ctx.record_rewrite();
                    self.stats.record_rewrite();
                    return Some(result);
                }
            } else {
                if let Some(result) = self.try_unconditional_rewrite(ctx, expr, &entry) {
                    ctx.record_rewrite();
                    self.stats.record_rewrite();
                    return Some(result);
                }
            }
        }
        None
    }
    /// Try an unconditional rewrite.
    fn try_unconditional_rewrite(
        &mut self,
        ctx: &mut SimpContext,
        expr: &Expr,
        entry: &SimpLemmaEntry,
    ) -> Option<SimpResult> {
        if self.exprs_match(&entry.lhs, expr) {
            let new_expr = entry.rhs.clone();
            let proof = entry.lemma.clone();
            return Some(SimpResult::changed(new_expr, proof, ctx.rewrite_count + 1));
        }
        None
    }
    /// Try a conditional rewrite.
    fn try_conditional_rewrite(
        &mut self,
        ctx: &mut SimpContext,
        expr: &Expr,
        entry: &SimpLemmaEntry,
    ) -> Option<SimpResult> {
        if !self.exprs_match(&entry.lhs, expr) {
            return None;
        }
        if let Some(side_cond) = &entry.side_condition {
            if self.discharge_side_condition(ctx, side_cond) {
                self.stats.record_conditional_rewrite();
                let new_expr = entry.rhs.clone();
                let proof = entry.lemma.clone();
                return Some(SimpResult::changed(new_expr, proof, ctx.rewrite_count + 1));
            }
        }
        None
    }
    /// Discharge a side condition.
    ///
    /// Tries the following strategies in order:
    /// 1. Reflexivity: the side condition is `@Eq α a a`.
    /// 2. Numeric comparison: constant numeric inequalities (`a < b`, `a ≤ b`, `a = b`).
    /// 3. Propositional simplification: `True`, `¬False`, `False → P`.
    ///
    /// Returns `false` if none of the strategies succeeds.
    pub(crate) fn discharge_side_condition(&self, _ctx: &SimpContext, side_cond: &Expr) -> bool {
        if self.try_discharge_refl(side_cond) {
            return true;
        }
        if self.try_discharge_numeric(side_cond) {
            return true;
        }
        if self.try_discharge_propositional(side_cond) {
            return true;
        }
        false
    }
    /// Returns `true` when `expr` is `@Eq α a a` (both sides are syntactically identical).
    fn try_discharge_refl(&self, expr: &Expr) -> bool {
        if let Expr::App(eq_lhs_box, rhs) = expr {
            if let Expr::App(eq_alpha_box, lhs) = eq_lhs_box.as_ref() {
                if let Expr::App(eq_const_box, _alpha) = eq_alpha_box.as_ref() {
                    if matches!(
                        eq_const_box.as_ref(), Expr::Const(n, _) if n.to_string() == "Eq"
                    ) {
                        return lhs == rhs;
                    }
                }
            }
        }
        false
    }
    /// Try to discharge constant numeric comparisons.
    ///
    /// Recognises `App(App(op, Lit(Nat(a))), Lit(Nat(b)))` for
    /// `op ∈ {Nat.lt, Nat.ble, Nat.beq, Nat.decLt, LT.lt, LE.le, GE.ge, GT.gt, Eq}`.
    fn try_discharge_numeric(&self, expr: &Expr) -> bool {
        let (head, args) = collect_app_chain(expr);
        let Expr::Const(head_name, _) = head else {
            return false;
        };
        let nat_args: Vec<u64> = args
            .iter()
            .filter_map(|a| {
                if let Expr::Lit(Literal::Nat(n)) = a {
                    Some(*n)
                } else {
                    None
                }
            })
            .collect();
        if nat_args.len() < 2 {
            return false;
        }
        let a = nat_args[nat_args.len() - 2];
        let b = nat_args[nat_args.len() - 1];
        let op = head_name.to_string();
        match op.as_str() {
            "LT.lt" | "Nat.lt" | "Nat.decLt" | "GT.gt" | "Nat.ble_lt" => a < b,
            "LE.le" | "Nat.le" | "GE.ge" | "Nat.ble" => a <= b,
            "Eq" | "Nat.beq" | "Nat.decEq" => a == b,
            "Ne" | "Nat.bne" => a != b,
            _ => false,
        }
    }
    /// Try simple propositional side conditions.
    ///
    /// Handles:
    /// - `True`                → discharged trivially
    /// - `¬False` / `Not False` → trivially true
    /// - `False → _`           → vacuously true (ex falso)
    fn try_discharge_propositional(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Const(n, _) if n.to_string() == "True" => true,
            Expr::App(not_box, arg) => {
                let is_not = matches!(
                    not_box.as_ref(), Expr::Const(n, _) if matches!(n.to_string()
                    .as_str(), "Not" | "not")
                );
                let arg_is_false = matches!(
                    arg.as_ref(), Expr::Const(n, _) if n.to_string() == "False"
                );
                if is_not && arg_is_false {
                    return true;
                }
                false
            }
            Expr::Pi(_, _, domain, _) => {
                matches!(domain.as_ref(), Expr::Const(n, _) if n.to_string() == "False")
            }
            _ => false,
        }
    }
    /// Apply congruence rules to simplify subexpressions.
    fn apply_congruence(&mut self, ctx: &mut SimpContext, expr: &Expr) -> SimpResult {
        match expr {
            Expr::App(f, arg) => self.simp_app(ctx, f, arg),
            Expr::Lam(_, _, body_ty, body) => self.simp_lambda(ctx, body_ty, body),
            Expr::Pi(_, _, param_ty, ret_ty) => self.simp_pi(ctx, param_ty, ret_ty),
            Expr::Let(_, _, val, body) => self.simp_let(ctx, val, body),
            _ => SimpResult::unchanged(expr.clone()),
        }
    }
    /// Simplify an application expression.
    fn simp_app(&mut self, ctx: &mut SimpContext, f: &Expr, arg: &Expr) -> SimpResult {
        ctx.push_position("func".to_string());
        let f_result = self.simp_main(ctx, f);
        ctx.pop_position();
        ctx.push_position("arg".to_string());
        let arg_result = self.simp_main(ctx, arg);
        ctx.pop_position();
        if f_result.changed || arg_result.changed {
            self.stats.record_congruence();
            let new_app = Expr::App(Box::new(f_result.new_expr), Box::new(arg_result.new_expr));
            let proof = Expr::Const(Name::str("cong_app"), vec![]);
            return SimpResult::changed(new_app, proof, ctx.rewrite_count);
        }
        SimpResult::unchanged(Expr::App(Box::new(f.clone()), Box::new(arg.clone())))
    }
    /// Simplify a lambda expression.
    fn simp_lambda(&mut self, ctx: &mut SimpContext, body_ty: &Expr, body: &Expr) -> SimpResult {
        ctx.push_position("body_type".to_string());
        let ty_result = self.simp_main(ctx, body_ty);
        ctx.pop_position();
        ctx.push_position("body".to_string());
        let body_result = self.simp_main(ctx, body);
        ctx.pop_position();
        if ty_result.changed || body_result.changed {
            self.stats.record_congruence();
            let proof = Expr::Const(Name::str("cong_lambda"), vec![]);
            return SimpResult::changed(
                Expr::Lam(
                    BinderInfo::Default,
                    Name::str("x"),
                    Box::new(ty_result.new_expr),
                    Box::new(body_result.new_expr),
                ),
                proof,
                ctx.rewrite_count,
            );
        }
        SimpResult::unchanged(Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(body_ty.clone()),
            Box::new(body.clone()),
        ))
    }
    /// Simplify a pi expression.
    fn simp_pi(&mut self, ctx: &mut SimpContext, param_ty: &Expr, ret_ty: &Expr) -> SimpResult {
        ctx.push_position("param_type".to_string());
        let param_result = self.simp_main(ctx, param_ty);
        ctx.pop_position();
        ctx.push_position("ret_type".to_string());
        let ret_result = self.simp_main(ctx, ret_ty);
        ctx.pop_position();
        if param_result.changed || ret_result.changed {
            self.stats.record_congruence();
            let proof = Expr::Const(Name::str("cong_pi"), vec![]);
            return SimpResult::changed(
                Expr::Pi(
                    BinderInfo::Default,
                    Name::str("x"),
                    Box::new(param_result.new_expr),
                    Box::new(ret_result.new_expr),
                ),
                proof,
                ctx.rewrite_count,
            );
        }
        SimpResult::unchanged(Expr::Pi(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(param_ty.clone()),
            Box::new(ret_ty.clone()),
        ))
    }
    /// Simplify a let expression.
    fn simp_let(&mut self, ctx: &mut SimpContext, val: &Expr, body: &Expr) -> SimpResult {
        ctx.push_position("value".to_string());
        let val_result = self.simp_main(ctx, val);
        ctx.pop_position();
        ctx.push_position("body".to_string());
        let body_result = self.simp_main(ctx, body);
        ctx.pop_position();
        if val_result.changed || body_result.changed {
            self.stats.record_congruence();
            let proof = Expr::Const(Name::str("cong_let"), vec![]);
            return SimpResult::changed(
                Expr::Let(
                    Name::str("x"),
                    Box::new(Expr::Const(Name::str("unit_ty"), vec![])),
                    Box::new(val_result.new_expr),
                    Box::new(body_result.new_expr),
                ),
                proof,
                ctx.rewrite_count,
            );
        }
        SimpResult::unchanged(Expr::Let(
            Name::str("x"),
            Box::new(Expr::Const(Name::str("unit_ty"), vec![])),
            Box::new(val.clone()),
            Box::new(body.clone()),
        ))
    }
    /// Normalize numeric operations through constant folding.
    fn norm_num(&self, expr: &Expr) -> Option<SimpResult> {
        let (head, args) = collect_app_chain(expr);
        let Expr::Const(head_name, _) = head else {
            return None;
        };
        let head_str = head_name.to_string();
        if (head_str == "Nat.succ" || head_str == "Nat.successor") && args.len() == 1 {
            if let Expr::Lit(Literal::Nat(n)) = args[0] {
                return Some(SimpResult::changed(
                    Expr::Lit(Literal::Nat(n + 1)),
                    Expr::Const(Name::str("Nat.succ_norm"), vec![]),
                    1,
                ));
            }
        }
        let (a, b) = if args.len() == 2 {
            (args[0], args[1])
        } else if args.len() >= 4 {
            (args[args.len() - 2], args[args.len() - 1])
        } else {
            return None;
        };
        let (Expr::Lit(Literal::Nat(lhs)), Expr::Lit(Literal::Nat(rhs))) = (a, b) else {
            return None;
        };
        let result: Option<u64> = match head_str.as_str() {
            "Nat.add" | "HAdd.hAdd" => Some(lhs + rhs),
            "Nat.mul" | "HMul.hMul" => Some(lhs * rhs),
            "Nat.sub" | "HSub.hSub" => Some(lhs.saturating_sub(*rhs)),
            "Nat.div" | "HDiv.hDiv" => {
                if *rhs == 0 {
                    Some(0)
                } else {
                    Some(lhs / rhs)
                }
            }
            "Nat.mod" | "HMod.hMod" => {
                if *rhs == 0 {
                    Some(*lhs)
                } else {
                    Some(lhs % rhs)
                }
            }
            "Nat.pow" | "HPow.hPow" => {
                if *rhs <= 32 {
                    Some(lhs.saturating_pow(*rhs as u32))
                } else {
                    None
                }
            }
            _ => None,
        };
        result.map(|v| {
            SimpResult::changed(
                Expr::Lit(Literal::Nat(v)),
                Expr::Const(Name::str("norm_num_fold"), vec![]),
                1,
            )
        })
    }
    /// Check if two expressions structurally match (used for simp lemma lookup).
    pub(crate) fn exprs_match(&self, pattern: &Expr, expr: &Expr) -> bool {
        match (pattern, expr) {
            (Expr::Const(n1, _), Expr::Const(n2, _)) => n1 == n2,
            (Expr::Lit(l1), Expr::Lit(l2)) => l1 == l2,
            (Expr::BVar(i1), Expr::BVar(i2)) => i1 == i2,
            (Expr::FVar(f1), Expr::FVar(f2)) => f1 == f2,
            (Expr::Sort(l1), Expr::Sort(l2)) => l1 == l2,
            (Expr::App(f1, a1), Expr::App(f2, a2)) => {
                self.exprs_match(f1, f2) && self.exprs_match(a1, a2)
            }
            (Expr::Lam(i1, _, t1, b1), Expr::Lam(i2, _, t2, b2)) => {
                i1 == i2 && self.exprs_match(t1, t2) && self.exprs_match(b1, b2)
            }
            (Expr::Pi(i1, _, t1, b1), Expr::Pi(i2, _, t2, b2)) => {
                i1 == i2 && self.exprs_match(t1, t2) && self.exprs_match(b1, b2)
            }
            (Expr::Proj(n1, i1, e1), Expr::Proj(n2, i2, e2)) => {
                n1 == n2 && i1 == i2 && self.exprs_match(e1, e2)
            }
            _ => false,
        }
    }
    /// Get statistics.
    pub fn stats(&self) -> &SimpStats {
        &self.stats
    }
    /// Clear cache.
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}
/// An analysis pass for SimpEngine.
#[allow(dead_code)]
pub struct SimpEngineAnalysisPass {
    pub name: String,
    pub enabled: bool,
    pub results: Vec<SimpEngineResult>,
    pub total_runs: usize,
}
#[allow(dead_code)]
impl SimpEngineAnalysisPass {
    pub fn new(name: &str) -> Self {
        SimpEngineAnalysisPass {
            name: name.to_string(),
            enabled: true,
            results: Vec::new(),
            total_runs: 0,
        }
    }
    pub fn run(&mut self, input: &str) -> SimpEngineResult {
        self.total_runs += 1;
        let result = if input.is_empty() {
            SimpEngineResult::Err("empty input".to_string())
        } else {
            SimpEngineResult::Ok(format!("processed: {}", input))
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
/// Simp lemma database using discrimination trees.
#[derive(Clone, Debug)]
pub struct SimpLemmaDb {
    /// Tree mapping LHS expressions to lemmas.
    pub(super) lemma_tree: DiscrTree<SimpLemmaEntry>,
    /// Map from lemma name to entries for removal.
    pub(super) named_lemmas: HashMap<Name, Vec<SimpLemmaEntry>>,
}
impl SimpLemmaDb {
    /// Create a new empty simp lemma database.
    pub fn new() -> Self {
        Self {
            lemma_tree: DiscrTree::new(),
            named_lemmas: HashMap::new(),
        }
    }
    /// Add a simp lemma to the database.
    pub fn add_lemma(&mut self, name: Name, entry: SimpLemmaEntry) {
        self.lemma_tree.insert(&entry.lhs, entry.clone());
        self.named_lemmas.entry(name).or_default().push(entry);
    }
    /// Find all lemmas whose LHS may match the given expression.
    pub fn find_candidates(&self, expr: &Expr) -> Vec<SimpLemmaEntry> {
        self.lemma_tree
            .find(expr)
            .iter()
            .map(|e| (*e).clone())
            .collect()
    }
    /// Get all lemmas.
    pub fn all_lemmas(&self) -> Vec<SimpLemmaEntry> {
        self.lemma_tree
            .all_values()
            .iter()
            .map(|e| (*e).clone())
            .collect()
    }
    /// Clear the database.
    pub fn clear(&mut self) {
        self.lemma_tree.clear();
        self.named_lemmas.clear();
    }
}
#[allow(dead_code)]
pub struct SimpEngineExtConfig3600 {
    pub(super) values: std::collections::HashMap<String, SimpEngineExtConfigVal3600>,
    pub(super) read_only: bool,
    pub(super) name: String,
}
impl SimpEngineExtConfig3600 {
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
    pub fn set(&mut self, key: &str, value: SimpEngineExtConfigVal3600) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&SimpEngineExtConfigVal3600> {
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
        self.set(key, SimpEngineExtConfigVal3600::Bool(v))
    }
    #[allow(dead_code)]
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, SimpEngineExtConfigVal3600::Int(v))
    }
    #[allow(dead_code)]
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, SimpEngineExtConfigVal3600::Str(v.to_string()))
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
/// A pipeline of SimpEngine analysis passes.
#[allow(dead_code)]
pub struct SimpEnginePipeline {
    pub passes: Vec<SimpEngineAnalysisPass>,
    pub name: String,
    pub total_inputs_processed: usize,
}
#[allow(dead_code)]
impl SimpEnginePipeline {
    pub fn new(name: &str) -> Self {
        SimpEnginePipeline {
            passes: Vec::new(),
            name: name.to_string(),
            total_inputs_processed: 0,
        }
    }
    pub fn add_pass(&mut self, pass: SimpEngineAnalysisPass) {
        self.passes.push(pass);
    }
    pub fn run_all(&mut self, input: &str) -> Vec<SimpEngineResult> {
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
/// A typed slot for SimpEngine configuration.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SimpEngineConfigValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
#[allow(dead_code)]
impl SimpEngineConfigValue {
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SimpEngineConfigValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    pub fn as_int(&self) -> Option<i64> {
        match self {
            SimpEngineConfigValue::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        match self {
            SimpEngineConfigValue::Float(f) => Some(*f),
            _ => None,
        }
    }
    pub fn as_str(&self) -> Option<&str> {
        match self {
            SimpEngineConfigValue::Str(s) => Some(s),
            _ => None,
        }
    }
    pub fn as_list(&self) -> Option<&[String]> {
        match self {
            SimpEngineConfigValue::List(v) => Some(v),
            _ => None,
        }
    }
    pub fn type_name(&self) -> &'static str {
        match self {
            SimpEngineConfigValue::Bool(_) => "bool",
            SimpEngineConfigValue::Int(_) => "int",
            SimpEngineConfigValue::Float(_) => "float",
            SimpEngineConfigValue::Str(_) => "str",
            SimpEngineConfigValue::List(_) => "list",
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SimpEngineExtConfigVal3600 {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<String>),
}
impl SimpEngineExtConfigVal3600 {
    #[allow(dead_code)]
    pub fn as_bool(&self) -> Option<bool> {
        if let SimpEngineExtConfigVal3600::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_int(&self) -> Option<i64> {
        if let SimpEngineExtConfigVal3600::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_float(&self) -> Option<f64> {
        if let SimpEngineExtConfigVal3600::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_str(&self) -> Option<&str> {
        if let SimpEngineExtConfigVal3600::Str(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn as_list(&self) -> Option<&[String]> {
        if let SimpEngineExtConfigVal3600::List(l) = self {
            Some(l)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            SimpEngineExtConfigVal3600::Bool(_) => "bool",
            SimpEngineExtConfigVal3600::Int(_) => "int",
            SimpEngineExtConfigVal3600::Float(_) => "float",
            SimpEngineExtConfigVal3600::Str(_) => "str",
            SimpEngineExtConfigVal3600::List(_) => "list",
        }
    }
}
/// A work queue for SimpEngine items.
#[allow(dead_code)]
pub struct SimpEngineWorkQueue {
    pub pending: std::collections::VecDeque<String>,
    pub processed: Vec<String>,
    pub capacity: usize,
}
#[allow(dead_code)]
impl SimpEngineWorkQueue {
    pub fn new(capacity: usize) -> Self {
        SimpEngineWorkQueue {
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
/// A diagnostic reporter for SimpEngine.
#[allow(dead_code)]
pub struct SimpEngineDiagnostics {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
#[allow(dead_code)]
impl SimpEngineDiagnostics {
    pub fn new(max_errors: usize) -> Self {
        SimpEngineDiagnostics {
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
/// Context for the simplification process.
#[derive(Clone, Debug)]
pub struct SimpContext {
    /// Current expression being simplified.
    pub current_expr: Expr,
    /// Position in the expression tree (path from root).
    pub position: Vec<String>,
    /// Number of rewrites performed so far.
    pub rewrite_count: usize,
    /// Current recursion depth.
    pub depth: usize,
    /// Set of visited expressions to prevent infinite loops.
    pub visited: HashSet<String>,
}
impl SimpContext {
    /// Create a new simp context.
    pub fn new(expr: Expr) -> Self {
        Self {
            current_expr: expr,
            position: Vec::new(),
            rewrite_count: 0,
            depth: 0,
            visited: HashSet::new(),
        }
    }
    /// Record visiting an expression.
    pub fn record_visit(&mut self, expr_str: &str) -> bool {
        self.visited.insert(expr_str.to_string())
    }
    /// Push a position marker.
    pub fn push_position(&mut self, pos: String) {
        self.position.push(pos);
    }
    /// Pop a position marker.
    pub fn pop_position(&mut self) {
        self.position.pop();
    }
    /// Increment rewrite count.
    pub fn record_rewrite(&mut self) {
        self.rewrite_count += 1;
    }
    /// Increment depth.
    pub fn inc_depth(&mut self) -> bool {
        self.depth += 1;
        true
    }
    /// Decrement depth.
    pub fn dec_depth(&mut self) {
        if self.depth > 0 {
            self.depth -= 1;
        }
    }
}
/// Statistics for simplification.
#[derive(Clone, Debug)]
pub struct SimpStats {
    /// Number of successful rewrites.
    pub rewrites: usize,
    /// Number of congruence applications.
    pub congruences: usize,
    /// Number of conditional rewrites discharged.
    pub conditional_rewrites: usize,
    /// Maximum depth reached.
    pub depth_max: usize,
    /// Number of cache hits.
    pub cache_hits: usize,
}
impl SimpStats {
    /// Create a new statistics object.
    pub fn new() -> Self {
        Self {
            rewrites: 0,
            congruences: 0,
            conditional_rewrites: 0,
            depth_max: 0,
            cache_hits: 0,
        }
    }
    /// Record a rewrite.
    pub fn record_rewrite(&mut self) {
        self.rewrites += 1;
    }
    /// Record a congruence.
    pub fn record_congruence(&mut self) {
        self.congruences += 1;
    }
    /// Record a conditional rewrite.
    pub fn record_conditional_rewrite(&mut self) {
        self.conditional_rewrites += 1;
    }
    /// Record a cache hit.
    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    /// Update maximum depth.
    pub fn update_depth_max(&mut self, depth: usize) {
        if depth > self.depth_max {
            self.depth_max = depth;
        }
    }
}
#[allow(dead_code)]
pub struct SimpEngineExtDiff3600 {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
impl SimpEngineExtDiff3600 {
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
/// An extended utility type for SimpEngine.
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct SimpEngineExt2 {
    /// A numeric tag.
    pub tag: u32,
}
#[allow(dead_code)]
impl SimpEngineExt2 {
    /// Creates a new instance.
    pub fn new() -> Self {
        Self { tag: 0 }
    }
}
pub struct SimpEngineExtUtil {
    pub key: String,
    pub data: Vec<i64>,
    pub active: bool,
    pub flags: u32,
}
#[allow(dead_code)]
impl SimpEngineExtUtil {
    pub fn new(key: &str) -> Self {
        SimpEngineExtUtil {
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
/// Configuration for the simplification engine.
#[derive(Clone, Debug)]
pub struct SimpEngineConfig {
    /// Maximum number of rewrite steps.
    pub max_steps: usize,
    /// Maximum recursion depth.
    pub max_depth: usize,
    /// Whether to apply beta reduction.
    pub beta_reduce: bool,
    /// Whether to apply eta reduction.
    pub eta_reduce: bool,
    /// Whether to apply zeta reduction (let binding).
    pub zeta_reduce: bool,
    /// Whether to apply iota reduction (match/constructor).
    pub iota_reduce: bool,
    /// Whether to use contextual simplification.
    pub contextual: bool,
    /// Whether to normalize numeric operations.
    pub norm_num: bool,
}
impl SimpEngineConfig {
    /// Create a default simp configuration.
    pub fn default_config() -> Self {
        Self {
            max_steps: 10000,
            max_depth: 100,
            beta_reduce: true,
            eta_reduce: true,
            zeta_reduce: true,
            iota_reduce: true,
            contextual: true,
            norm_num: true,
        }
    }
    /// Create an aggressive simp configuration.
    pub fn aggressive() -> Self {
        Self {
            max_steps: 50000,
            max_depth: 200,
            beta_reduce: true,
            eta_reduce: true,
            zeta_reduce: true,
            iota_reduce: true,
            contextual: true,
            norm_num: true,
        }
    }
    /// Create a conservative simp configuration.
    pub fn conservative() -> Self {
        Self {
            max_steps: 1000,
            max_depth: 50,
            beta_reduce: true,
            eta_reduce: false,
            zeta_reduce: false,
            iota_reduce: false,
            contextual: false,
            norm_num: false,
        }
    }
}
#[allow(dead_code)]
pub struct SimpEngineExtPass3600 {
    pub name: String,
    pub total_runs: usize,
    pub successes: usize,
    pub errors: usize,
    pub enabled: bool,
    pub results: Vec<SimpEngineExtResult3600>,
}
impl SimpEngineExtPass3600 {
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
    pub fn run(&mut self, input: &str) -> SimpEngineExtResult3600 {
        if !self.enabled {
            return SimpEngineExtResult3600::Skipped;
        }
        self.total_runs += 1;
        let result = if input.is_empty() {
            self.errors += 1;
            SimpEngineExtResult3600::Err(format!("empty input in pass '{}'", self.name))
        } else {
            self.successes += 1;
            SimpEngineExtResult3600::Ok(format!(
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
/// A diff for SimpEngine analysis results.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct SimpEngineDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub unchanged: Vec<String>,
}
#[allow(dead_code)]
impl SimpEngineDiff {
    pub fn new() -> Self {
        SimpEngineDiff {
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
pub struct SimpEngineExtPipeline3600 {
    pub name: String,
    pub passes: Vec<SimpEngineExtPass3600>,
    pub run_count: usize,
}
impl SimpEngineExtPipeline3600 {
    #[allow(dead_code)]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            passes: Vec::new(),
            run_count: 0,
        }
    }
    #[allow(dead_code)]
    pub fn add_pass(&mut self, pass: SimpEngineExtPass3600) {
        self.passes.push(pass);
    }
    #[allow(dead_code)]
    pub fn run_all(&mut self, input: &str) -> Vec<SimpEngineExtResult3600> {
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
/// A state machine controller for SimpEngine.
#[allow(dead_code)]
pub struct SimpEngineStateMachine {
    pub state: SimpEngineState,
    pub transitions: usize,
    pub history: Vec<String>,
}
#[allow(dead_code)]
impl SimpEngineStateMachine {
    pub fn new() -> Self {
        SimpEngineStateMachine {
            state: SimpEngineState::Initial,
            transitions: 0,
            history: Vec::new(),
        }
    }
    pub fn transition_to(&mut self, new_state: SimpEngineState) -> bool {
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
        self.transition_to(SimpEngineState::Running)
    }
    pub fn pause(&mut self) -> bool {
        self.transition_to(SimpEngineState::Paused)
    }
    pub fn complete(&mut self) -> bool {
        self.transition_to(SimpEngineState::Complete)
    }
    pub fn fail(&mut self, msg: &str) -> bool {
        self.transition_to(SimpEngineState::Failed(msg.to_string()))
    }
    pub fn num_transitions(&self) -> usize {
        self.transitions
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum SimpEngineExtResult3600 {
    /// Operation completed successfully.
    Ok(String),
    /// Operation encountered an error.
    Err(String),
    /// Operation partially completed.
    Partial { done: usize, total: usize },
    /// Operation was skipped.
    Skipped,
}
impl SimpEngineExtResult3600 {
    #[allow(dead_code)]
    pub fn is_ok(&self) -> bool {
        matches!(self, SimpEngineExtResult3600::Ok(_))
    }
    #[allow(dead_code)]
    pub fn is_err(&self) -> bool {
        matches!(self, SimpEngineExtResult3600::Err(_))
    }
    #[allow(dead_code)]
    pub fn is_partial(&self) -> bool {
        matches!(self, SimpEngineExtResult3600::Partial { .. })
    }
    #[allow(dead_code)]
    pub fn is_skipped(&self) -> bool {
        matches!(self, SimpEngineExtResult3600::Skipped)
    }
    #[allow(dead_code)]
    pub fn ok_msg(&self) -> Option<&str> {
        if let SimpEngineExtResult3600::Ok(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn err_msg(&self) -> Option<&str> {
        if let SimpEngineExtResult3600::Err(s) = self {
            Some(s)
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn progress(&self) -> f64 {
        match self {
            SimpEngineExtResult3600::Ok(_) => 1.0,
            SimpEngineExtResult3600::Err(_) => 0.0,
            SimpEngineExtResult3600::Partial { done, total } => {
                if *total == 0 {
                    0.0
                } else {
                    *done as f64 / *total as f64
                }
            }
            SimpEngineExtResult3600::Skipped => 0.5,
        }
    }
}
#[allow(dead_code)]
pub struct SimpEngineExtDiag3600 {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub notes: Vec<String>,
    pub max_errors: usize,
}
impl SimpEngineExtDiag3600 {
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
/// Entry in the simp lemma database.
#[derive(Clone, Debug)]
pub struct SimpLemmaEntry {
    /// The lemma expression (usually an equality proof).
    pub lemma: Expr,
    /// Left-hand side of the rewrite.
    pub lhs: Expr,
    /// Right-hand side of the rewrite.
    pub rhs: Expr,
    /// Priority (higher = more preferred).
    pub priority: i32,
    /// Whether this lemma has side conditions.
    pub conditional: bool,
    /// Side condition expression (if conditional).
    pub side_condition: Option<Expr>,
}
impl SimpLemmaEntry {
    /// Create a new simp lemma entry.
    pub fn new(lemma: Expr, lhs: Expr, rhs: Expr, priority: i32) -> Self {
        Self {
            lemma,
            lhs,
            rhs,
            priority,
            conditional: false,
            side_condition: None,
        }
    }
    /// Create a conditional simp lemma entry.
    pub fn conditional(
        lemma: Expr,
        lhs: Expr,
        rhs: Expr,
        priority: i32,
        side_condition: Expr,
    ) -> Self {
        Self {
            lemma,
            lhs,
            rhs,
            priority,
            conditional: true,
            side_condition: Some(side_condition),
        }
    }
}
/// Result of simplification.
#[derive(Clone, Debug)]
pub struct SimpResult {
    /// The simplified expression.
    pub new_expr: Expr,
    /// Proof that the simplification is correct (equality proof).
    pub proof: Expr,
    /// Whether any changes were made.
    pub changed: bool,
    /// Number of steps used.
    pub steps_used: usize,
}
impl SimpResult {
    /// Create a result with no changes.
    pub fn unchanged(expr: Expr) -> Self {
        Self {
            new_expr: expr.clone(),
            proof: Expr::Const(Name::str("rfl"), vec![]),
            changed: false,
            steps_used: 0,
        }
    }
    /// Create a result with changes.
    pub fn changed(new_expr: Expr, proof: Expr, steps: usize) -> Self {
        Self {
            new_expr,
            proof,
            changed: true,
            steps_used: steps,
        }
    }
}
/// A configuration store for SimpEngine.
#[allow(dead_code)]
pub struct SimpEngineConfigStore {
    pub values: std::collections::HashMap<String, SimpEngineConfigValue>,
    pub read_only: bool,
}
#[allow(dead_code)]
impl SimpEngineConfigStore {
    pub fn new() -> Self {
        SimpEngineConfigStore {
            values: std::collections::HashMap::new(),
            read_only: false,
        }
    }
    pub fn set(&mut self, key: &str, value: SimpEngineConfigValue) -> bool {
        if self.read_only {
            return false;
        }
        self.values.insert(key.to_string(), value);
        true
    }
    pub fn get(&self, key: &str) -> Option<&SimpEngineConfigValue> {
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
        self.set(key, SimpEngineConfigValue::Bool(v))
    }
    pub fn set_int(&mut self, key: &str, v: i64) -> bool {
        self.set(key, SimpEngineConfigValue::Int(v))
    }
    pub fn set_str(&mut self, key: &str, v: &str) -> bool {
        self.set(key, SimpEngineConfigValue::Str(v.to_string()))
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
