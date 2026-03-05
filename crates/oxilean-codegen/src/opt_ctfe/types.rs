//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::lcnf::{LcnfArg, LcnfExpr, LcnfFunDecl, LcnfLetValue, LcnfLit, LcnfVarId};
use std::collections::HashMap;

use super::functions::CtfeResult;

use super::functions::*;
use std::collections::HashSet;

/// CTFE abstract interpreter mode
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CtfeMode {
    FullEval,
    PartialEval,
    FoldOnly,
    Disabled,
}
/// CTFE type checker (basic type inference during evaluation)
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CtfeType {
    Unit,
    Bool,
    Int,
    Uint,
    Float,
    Str,
    Tuple(Vec<CtfeType>),
    List(Box<CtfeType>),
    Named(String),
    Unknown,
}
/// CTFE name generator
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeNameGen {
    pub(super) counter: u64,
    pub(super) prefix: String,
}
#[allow(dead_code)]
impl CtfeNameGen {
    pub fn new(prefix: &str) -> Self {
        Self {
            counter: 0,
            prefix: prefix.to_string(),
        }
    }
    pub fn next(&mut self) -> String {
        let id = self.counter;
        self.counter += 1;
        format!("{}{}", self.prefix, id)
    }
    pub fn reset(&mut self) {
        self.counter = 0;
    }
}
/// Summary statistics for a CTFE pass run.
#[derive(Debug, Clone, Default)]
pub struct CtfeReport {
    /// Number of top-level functions that were fully evaluated.
    pub functions_evaluated: usize,
    /// Number of call sites replaced with a constant value.
    pub calls_replaced: usize,
    /// Number of constants propagated across function boundaries.
    pub constants_propagated: usize,
    /// Number of functions whose evaluation was aborted due to fuel exhaustion.
    pub fuel_exhausted_count: usize,
}
/// CTFE call stack
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeCallStack {
    pub frames: Vec<CtfeCallFrame>,
    pub max_depth: usize,
}
#[allow(dead_code)]
impl CtfeCallStack {
    pub fn new(max_depth: usize) -> Self {
        Self {
            frames: Vec::new(),
            max_depth,
        }
    }
    pub fn push(&mut self, func: &str, args: Vec<CtfeValueExt>) -> bool {
        if self.frames.len() >= self.max_depth {
            return false;
        }
        self.frames.push(CtfeCallFrame {
            func_name: func.to_string(),
            args,
            depth: self.frames.len(),
        });
        true
    }
    pub fn pop(&mut self) -> Option<CtfeCallFrame> {
        self.frames.pop()
    }
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    pub fn is_recursing(&self, func: &str) -> bool {
        self.frames.iter().any(|f| f.func_name == func)
    }
    pub fn stack_trace(&self) -> Vec<String> {
        self.frames
            .iter()
            .rev()
            .map(|f| format!("  at {}(...)", f.func_name))
            .collect()
    }
}
/// CTFE diagnostic severity
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CtfeDiagLevel {
    Debug,
    Info,
    Warning,
    Error,
}
/// CTFE evaluation result
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeEvalResult {
    pub value: Option<CtfeValueExt>,
    pub fuel_used: u64,
    pub steps: usize,
    pub stack_depth_max: usize,
    pub memo_hit: bool,
}
/// Arithmetic / comparison operator identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}
impl BinOp {
    /// Parse a common operator name into a `BinOp`.
    pub fn from_name(name: &str) -> Option<BinOp> {
        match name {
            "add" | "+" | "Nat.add" | "Int.add" => Some(BinOp::Add),
            "sub" | "-" | "Nat.sub" | "Int.sub" => Some(BinOp::Sub),
            "mul" | "*" | "Nat.mul" | "Int.mul" => Some(BinOp::Mul),
            "div" | "/" | "Nat.div" | "Int.div" => Some(BinOp::Div),
            "mod" | "%" | "Nat.mod" | "Int.mod" => Some(BinOp::Mod),
            "and" | "&&" | "Bool.and" => Some(BinOp::And),
            "or" | "||" | "Bool.or" => Some(BinOp::Or),
            "xor" | "Bool.xor" => Some(BinOp::Xor),
            "shl" | "<<" => Some(BinOp::Shl),
            "shr" | ">>" => Some(BinOp::Shr),
            "eq" | "==" | "Eq" => Some(BinOp::Eq),
            "ne" | "!=" | "Ne" => Some(BinOp::Ne),
            "lt" | "<" | "Nat.lt" => Some(BinOp::Lt),
            "le" | "<=" | "Nat.le" => Some(BinOp::Le),
            "gt" | ">" | "Nat.gt" => Some(BinOp::Gt),
            "ge" | ">=" | "Nat.ge" => Some(BinOp::Ge),
            _ => None,
        }
    }
}
/// CTFE pass statistics (extended)
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct CtfePassStatsExt {
    pub functions_attempted: usize,
    pub functions_evaluated: usize,
    pub calls_replaced: usize,
    pub constants_folded: usize,
    pub memo_hits: u64,
    pub memo_misses: u64,
    pub fuel_used: u64,
    pub fuel_exhausted_count: usize,
    pub max_stack_depth_reached: usize,
    pub errors: usize,
}
/// CTFE evaluator configuration (extended)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeConfigExt {
    pub fuel: u64,
    pub max_depth: usize,
    pub max_list_size: usize,
    pub max_string_size: usize,
    pub enable_memoization: bool,
    pub enable_logging: bool,
    pub replace_calls: bool,
    pub propagate_constants: bool,
    pub fold_arithmetic: bool,
    pub fold_boolean: bool,
    pub fold_string: bool,
    pub fold_comparison: bool,
}
/// CTFE evaluation trace entry
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeTraceEntry {
    pub depth: usize,
    pub func: String,
    pub args_repr: String,
    pub result_repr: Option<String>,
}
/// A fully-evaluated compile-time value.
#[derive(Debug, Clone, PartialEq)]
pub enum CtfeValue {
    /// Signed 64-bit integer (covers Nat for small values).
    Int(i64),
    /// 64-bit floating-point value.
    Float(f64),
    /// Boolean value.
    Bool(bool),
    /// String value.
    String(String),
    /// A heterogeneous list of values.
    List(Vec<CtfeValue>),
    /// A tuple of values.
    Tuple(Vec<CtfeValue>),
    /// An algebraic data type constructor: `name(fields...)`.
    Constructor(String, Vec<CtfeValue>),
    /// Undefined / not yet evaluated (bottom).
    Undef,
}
impl CtfeValue {
    /// Return the underlying integer, if any.
    pub fn as_int(&self) -> Option<i64> {
        match self {
            CtfeValue::Int(n) => Some(*n),
            _ => None,
        }
    }
    /// Return the underlying bool, if any.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            CtfeValue::Bool(b) => Some(*b),
            _ => None,
        }
    }
    /// Return the underlying string, if any.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            CtfeValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }
    /// `true` if the value is fully concrete (no `Undef` components).
    pub fn is_concrete(&self) -> bool {
        match self {
            CtfeValue::Undef => false,
            CtfeValue::List(xs) | CtfeValue::Tuple(xs) => xs.iter().all(|v| v.is_concrete()),
            CtfeValue::Constructor(_, fields) => fields.iter().all(|v| v.is_concrete()),
            _ => true,
        }
    }
}
/// CTFE profiler
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeExtProfiler {
    pub timings: Vec<(String, u64)>,
}
#[allow(dead_code)]
impl CtfeExtProfiler {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn record(&mut self, pass: &str, us: u64) {
        self.timings.push((pass.to_string(), us));
    }
    pub fn total_us(&self) -> u64 {
        self.timings.iter().map(|(_, t)| *t).sum()
    }
    pub fn slowest(&self) -> Option<(&str, u64)> {
        self.timings
            .iter()
            .max_by_key(|(_, t)| *t)
            .map(|(n, t)| (n.as_str(), *t))
    }
}
/// CTFE value representation (extended)
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum CtfeValueExt {
    Unit,
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),
    Str(String),
    Tuple(Vec<CtfeValueExt>),
    List(Vec<CtfeValueExt>),
    Constructor(String, Vec<CtfeValueExt>),
    Closure {
        params: Vec<String>,
        body: String,
        env: Vec<(String, CtfeValueExt)>,
    },
    Opaque,
}
/// CTFE code stats
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct CtfeCodeStats {
    pub constants_discovered: usize,
    pub folds_applied: usize,
    pub calls_eliminated: usize,
    pub loops_unrolled: usize,
    pub conditions_resolved: usize,
}
/// CTFE fuel tracker
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeFuelTracker {
    pub remaining: u64,
    pub initial: u64,
    pub steps_taken: u64,
}
#[allow(dead_code)]
impl CtfeFuelTracker {
    pub fn new(fuel: u64) -> Self {
        Self {
            remaining: fuel,
            initial: fuel,
            steps_taken: 0,
        }
    }
    pub fn consume(&mut self, cost: u64) -> bool {
        if self.remaining < cost {
            false
        } else {
            self.remaining -= cost;
            self.steps_taken += cost;
            true
        }
    }
    pub fn is_exhausted(&self) -> bool {
        self.remaining == 0
    }
    pub fn fraction_used(&self) -> f64 {
        if self.initial == 0 {
            1.0
        } else {
            self.steps_taken as f64 / self.initial as f64
        }
    }
}
/// CTFE term simplifier
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeSimplifier {
    pub rules_applied: usize,
    pub memo: std::collections::HashMap<String, CtfeValueExt>,
}
#[allow(dead_code)]
impl CtfeSimplifier {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn simplify_bool_and(a: CtfeValueExt, b: CtfeValueExt) -> CtfeValueExt {
        match (a, b) {
            (CtfeValueExt::Bool(false), _) | (_, CtfeValueExt::Bool(false)) => {
                CtfeValueExt::Bool(false)
            }
            (CtfeValueExt::Bool(true), v) | (v, CtfeValueExt::Bool(true)) => v,
            (av, bv) => CtfeValueExt::Constructor("And".to_string(), vec![av, bv]),
        }
    }
    pub fn simplify_bool_or(a: CtfeValueExt, b: CtfeValueExt) -> CtfeValueExt {
        match (a, b) {
            (CtfeValueExt::Bool(true), _) | (_, CtfeValueExt::Bool(true)) => {
                CtfeValueExt::Bool(true)
            }
            (CtfeValueExt::Bool(false), v) | (v, CtfeValueExt::Bool(false)) => v,
            (av, bv) => CtfeValueExt::Constructor("Or".to_string(), vec![av, bv]),
        }
    }
    pub fn simplify_add_int(a: CtfeValueExt, b: CtfeValueExt) -> CtfeValueExt {
        match (a, b) {
            (CtfeValueExt::Int(x), CtfeValueExt::Int(y)) => CtfeValueExt::Int(x.wrapping_add(y)),
            (CtfeValueExt::Int(0), v) | (v, CtfeValueExt::Int(0)) => v,
            (av, bv) => CtfeValueExt::Constructor("Add".to_string(), vec![av, bv]),
        }
    }
    pub fn simplify_mul_int(a: CtfeValueExt, b: CtfeValueExt) -> CtfeValueExt {
        match (a, b) {
            (CtfeValueExt::Int(x), CtfeValueExt::Int(y)) => CtfeValueExt::Int(x.wrapping_mul(y)),
            (CtfeValueExt::Int(0), _) | (_, CtfeValueExt::Int(0)) => CtfeValueExt::Int(0),
            (CtfeValueExt::Int(1), v) | (v, CtfeValueExt::Int(1)) => v,
            (av, bv) => CtfeValueExt::Constructor("Mul".to_string(), vec![av, bv]),
        }
    }
}
/// CTFE source buffer
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeExtSourceBuffer {
    pub content: String,
}
#[allow(dead_code)]
impl CtfeExtSourceBuffer {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn write(&mut self, s: &str) {
        self.content.push_str(s);
    }
    pub fn writeln(&mut self, s: &str) {
        self.content.push_str(s);
        self.content.push('\n');
    }
    pub fn finish(self) -> String {
        self.content
    }
}
/// The CTFE interpreter evaluates LCNF expressions at compile time.
pub struct CtfeInterpreter {
    /// Memoisation cache: (function name, args) → value.
    pub(super) memo: HashMap<(String, Vec<String>), CtfeValue>,
    /// All function declarations available for inlining.
    pub(super) decls: HashMap<String, LcnfFunDecl>,
}
impl CtfeInterpreter {
    /// Create a new interpreter with access to a module's declarations.
    pub fn new(decls: &[LcnfFunDecl]) -> Self {
        let decl_map = decls.iter().map(|d| (d.name.clone(), d.clone())).collect();
        CtfeInterpreter {
            memo: HashMap::new(),
            decls: decl_map,
        }
    }
    /// Evaluate a literal to a `CtfeValue`.
    pub fn eval_lit(&self, lit: &LcnfLit) -> CtfeValue {
        match lit {
            LcnfLit::Nat(n) => CtfeValue::Int(*n as i64),
            LcnfLit::Str(s) => CtfeValue::String(s.clone()),
        }
    }
    /// Evaluate an argument in the given context.
    pub fn eval_arg(&self, arg: &LcnfArg, ctx: &CtfeContext) -> CtfeResult {
        match arg {
            LcnfArg::Lit(lit) => Ok(self.eval_lit(lit)),
            LcnfArg::Var(id) => {
                ctx.lookup_local(*id)
                    .cloned()
                    .ok_or_else(|| CtfeError::NonConstant {
                        reason: format!("unbound variable {}", id.0),
                    })
            }
            LcnfArg::Erased => Ok(CtfeValue::Undef),
            LcnfArg::Type(_) => Ok(CtfeValue::Undef),
        }
    }
    /// Evaluate a binary operation.
    pub fn eval_binop(&self, op: BinOp, lhs: &CtfeValue, rhs: &CtfeValue) -> CtfeResult {
        match (op, lhs, rhs) {
            (BinOp::Add, CtfeValue::Int(a), CtfeValue::Int(b)) => a
                .checked_add(*b)
                .map(CtfeValue::Int)
                .ok_or(CtfeError::Overflow {
                    op: "add".to_string(),
                }),
            (BinOp::Sub, CtfeValue::Int(a), CtfeValue::Int(b)) => a
                .checked_sub(*b)
                .map(CtfeValue::Int)
                .ok_or(CtfeError::Overflow {
                    op: "sub".to_string(),
                }),
            (BinOp::Mul, CtfeValue::Int(a), CtfeValue::Int(b)) => a
                .checked_mul(*b)
                .map(CtfeValue::Int)
                .ok_or(CtfeError::Overflow {
                    op: "mul".to_string(),
                }),
            (BinOp::Div, CtfeValue::Int(_), CtfeValue::Int(0)) => Err(CtfeError::DivisionByZero),
            (BinOp::Div, CtfeValue::Int(a), CtfeValue::Int(b)) => a
                .checked_div(*b)
                .map(CtfeValue::Int)
                .ok_or(CtfeError::Overflow {
                    op: "div".to_string(),
                }),
            (BinOp::Mod, CtfeValue::Int(_), CtfeValue::Int(0)) => Err(CtfeError::DivisionByZero),
            (BinOp::Mod, CtfeValue::Int(a), CtfeValue::Int(b)) => {
                Ok(CtfeValue::Int(a.rem_euclid(*b)))
            }
            (BinOp::Shl, CtfeValue::Int(a), CtfeValue::Int(b)) if *b >= 0 && *b < 64 => {
                Ok(CtfeValue::Int(a.wrapping_shl(*b as u32)))
            }
            (BinOp::Shr, CtfeValue::Int(a), CtfeValue::Int(b)) if *b >= 0 && *b < 64 => {
                Ok(CtfeValue::Int(a.wrapping_shr(*b as u32)))
            }
            (BinOp::And, CtfeValue::Bool(a), CtfeValue::Bool(b)) => Ok(CtfeValue::Bool(*a && *b)),
            (BinOp::Or, CtfeValue::Bool(a), CtfeValue::Bool(b)) => Ok(CtfeValue::Bool(*a || *b)),
            (BinOp::Xor, CtfeValue::Bool(a), CtfeValue::Bool(b)) => Ok(CtfeValue::Bool(*a ^ *b)),
            (BinOp::Eq, CtfeValue::Int(a), CtfeValue::Int(b)) => Ok(CtfeValue::Bool(a == b)),
            (BinOp::Ne, CtfeValue::Int(a), CtfeValue::Int(b)) => Ok(CtfeValue::Bool(a != b)),
            (BinOp::Lt, CtfeValue::Int(a), CtfeValue::Int(b)) => Ok(CtfeValue::Bool(a < b)),
            (BinOp::Le, CtfeValue::Int(a), CtfeValue::Int(b)) => Ok(CtfeValue::Bool(a <= b)),
            (BinOp::Gt, CtfeValue::Int(a), CtfeValue::Int(b)) => Ok(CtfeValue::Bool(a > b)),
            (BinOp::Ge, CtfeValue::Int(a), CtfeValue::Int(b)) => Ok(CtfeValue::Bool(a >= b)),
            (BinOp::Eq, CtfeValue::String(a), CtfeValue::String(b)) => Ok(CtfeValue::Bool(a == b)),
            (BinOp::Ne, CtfeValue::String(a), CtfeValue::String(b)) => Ok(CtfeValue::Bool(a != b)),
            (BinOp::Add, CtfeValue::Float(a), CtfeValue::Float(b)) => Ok(CtfeValue::Float(a + b)),
            (BinOp::Sub, CtfeValue::Float(a), CtfeValue::Float(b)) => Ok(CtfeValue::Float(a - b)),
            (BinOp::Mul, CtfeValue::Float(a), CtfeValue::Float(b)) => Ok(CtfeValue::Float(a * b)),
            (BinOp::Div, CtfeValue::Float(a), CtfeValue::Float(b)) => Ok(CtfeValue::Float(a / b)),
            _ => Err(CtfeError::NonConstant {
                reason: format!("unsupported binop {:?} on {:?} {:?}", op, lhs, rhs),
            }),
        }
    }
    /// Evaluate a function call to a (possibly known) function.
    pub fn eval_call(
        &mut self,
        func_name: &str,
        args: Vec<CtfeValue>,
        ctx: &mut CtfeContext,
    ) -> CtfeResult {
        ctx.consume_fuel()?;
        let cache_key = (
            func_name.to_string(),
            args.iter().map(|v| v.to_string()).collect::<Vec<_>>(),
        );
        if let Some(cached) = self.memo.get(&cache_key) {
            return Ok(cached.clone());
        }
        if let Some(op) = BinOp::from_name(func_name) {
            if args.len() == 2 {
                let result = self.eval_binop(op, &args[0], &args[1])?;
                self.memo.insert(cache_key, result.clone());
                return Ok(result);
            }
        }
        let decl = match self.decls.get(func_name).cloned() {
            Some(d) => d,
            None => {
                return Err(CtfeError::NonConstant {
                    reason: format!("unknown function '{}'", func_name),
                });
            }
        };
        if decl.params.len() != args.len() {
            return Err(CtfeError::NonConstant {
                reason: format!(
                    "arity mismatch for '{}': expected {}, got {}",
                    func_name,
                    decl.params.len(),
                    args.len()
                ),
            });
        }
        ctx.push_frame()?;
        let mut child = ctx.child_context();
        for (param, value) in decl.params.iter().zip(args.iter()) {
            child.bind_local(param.id, value.clone());
        }
        let result = self.eval_expr(&decl.body, &mut child);
        ctx.merge_fuel_from(&child);
        ctx.pop_frame();
        if let Ok(ref v) = result {
            self.memo.insert(cache_key, v.clone());
        }
        result
    }
    /// Evaluate a full LCNF expression.
    pub fn eval_expr(&mut self, expr: &LcnfExpr, ctx: &mut CtfeContext) -> CtfeResult {
        ctx.consume_fuel()?;
        match expr {
            LcnfExpr::Return(arg) => self.eval_arg(arg, ctx),
            LcnfExpr::Unreachable => Err(CtfeError::NonExhaustiveMatch),
            LcnfExpr::Let {
                id, value, body, ..
            } => {
                let val = self.eval_let_value(value, ctx)?;
                ctx.bind_local(*id, val);
                self.eval_expr(body, ctx)
            }
            LcnfExpr::TailCall(func, args) => {
                let func_name = match func {
                    LcnfArg::Var(id) => format!("__var_{}", id.0),
                    _ => {
                        return Err(CtfeError::NonConstant {
                            reason: "indirect tail-call".to_string(),
                        });
                    }
                };
                let arg_vals: Result<Vec<_>, _> =
                    args.iter().map(|a| self.eval_arg(a, ctx)).collect();
                self.eval_call(&func_name, arg_vals?, ctx)
            }
            LcnfExpr::Case {
                scrutinee,
                alts,
                default,
                ..
            } => {
                let scr_val = ctx.lookup_local(*scrutinee).cloned().ok_or_else(|| {
                    CtfeError::NonConstant {
                        reason: format!("case scrutinee {} not bound", scrutinee.0),
                    }
                })?;
                ctx.consume_fuel()?;
                match &scr_val {
                    CtfeValue::Constructor(ctor_name, fields) => {
                        for alt in alts {
                            if alt.ctor_name == *ctor_name {
                                let mut child = ctx.child_context();
                                for (param, field) in alt.params.iter().zip(fields.iter()) {
                                    child.bind_local(param.id, field.clone());
                                }
                                let result = self.eval_expr(&alt.body, &mut child);
                                ctx.merge_fuel_from(&child);
                                return result;
                            }
                        }
                        if let Some(def) = default {
                            return self.eval_expr(def, ctx);
                        }
                        Err(CtfeError::NonExhaustiveMatch)
                    }
                    CtfeValue::Bool(b) => {
                        let target_ctor = if *b { "true" } else { "false" };
                        for alt in alts {
                            if alt.ctor_name == target_ctor {
                                return self.eval_expr(&alt.body, ctx);
                            }
                        }
                        if let Some(def) = default {
                            return self.eval_expr(def, ctx);
                        }
                        Err(CtfeError::NonExhaustiveMatch)
                    }
                    _ => {
                        if let Some(def) = default {
                            return self.eval_expr(def, ctx);
                        }
                        Err(CtfeError::NonConstant {
                            reason: format!("cannot case-split on {:?}", scr_val),
                        })
                    }
                }
            }
        }
    }
    pub(super) fn eval_let_value(
        &mut self,
        value: &LcnfLetValue,
        ctx: &mut CtfeContext,
    ) -> CtfeResult {
        match value {
            LcnfLetValue::Lit(lit) => Ok(self.eval_lit(lit)),
            LcnfLetValue::Erased => Ok(CtfeValue::Undef),
            LcnfLetValue::FVar(id) => {
                ctx.lookup_local(*id)
                    .cloned()
                    .ok_or_else(|| CtfeError::NonConstant {
                        reason: format!("free variable {}", id.0),
                    })
            }
            LcnfLetValue::App(func, args) => {
                let arg_vals: Result<Vec<_>, _> =
                    args.iter().map(|a| self.eval_arg(a, ctx)).collect();
                let func_name = match func {
                    LcnfArg::Var(id) => {
                        if let Some(v) = ctx.lookup_local(*id) {
                            if let CtfeValue::String(name) = v.clone() {
                                name
                            } else {
                                format!("__var_{}", id.0)
                            }
                        } else {
                            format!("__var_{}", id.0)
                        }
                    }
                    _ => {
                        return Err(CtfeError::NonConstant {
                            reason: "non-variable function in App".to_string(),
                        });
                    }
                };
                self.eval_call(&func_name, arg_vals?, ctx)
            }
            LcnfLetValue::Ctor(name, _tag, args) => {
                let field_vals: Result<Vec<_>, _> =
                    args.iter().map(|a| self.eval_arg(a, ctx)).collect();
                Ok(CtfeValue::Constructor(name.clone(), field_vals?))
            }
            LcnfLetValue::Proj(_struct_name, idx, var) => {
                let base =
                    ctx.lookup_local(*var)
                        .cloned()
                        .ok_or_else(|| CtfeError::NonConstant {
                            reason: format!("proj base {} not bound", var.0),
                        })?;
                match &base {
                    CtfeValue::Constructor(_, fields) => fields
                        .get(*idx as usize)
                        .cloned()
                        .ok_or(CtfeError::BadProjection { field: *idx }),
                    CtfeValue::Tuple(fields) => fields
                        .get(*idx as usize)
                        .cloned()
                        .ok_or(CtfeError::BadProjection { field: *idx }),
                    _ => Err(CtfeError::BadProjection { field: *idx }),
                }
            }
            LcnfLetValue::Reset(_) | LcnfLetValue::Reuse(_, _, _, _) => Ok(CtfeValue::Undef),
        }
    }
}
/// The main CTFE optimisation pass.
pub struct CtfePass {
    /// Configuration.
    pub config: CtfeConfig,
    /// Global evaluated constants available to downstream passes.
    pub known_constants: HashMap<String, CtfeValue>,
    /// Report accumulated during the pass.
    pub(super) report: CtfeReport,
}
impl CtfePass {
    /// Create a new pass with the given configuration.
    pub fn new(config: CtfeConfig) -> Self {
        CtfePass {
            config,
            known_constants: HashMap::new(),
            report: CtfeReport::default(),
        }
    }
    /// Create a pass with default configuration.
    pub fn default_pass() -> Self {
        Self::new(CtfeConfig::default())
    }
    /// Run the CTFE pass over all declarations.
    ///
    /// First all zero-argument (constant) functions are evaluated, then
    /// call sites inside every function are replaced with their constant
    /// folded results.
    pub fn run(&mut self, decls: &mut [LcnfFunDecl]) {
        self.known_constants.clear();
        self.report = CtfeReport::default();
        let mut interp = CtfeInterpreter::new(decls);
        for decl in decls.iter() {
            self.try_evaluate_decl(decl, &mut interp);
        }
        if self.config.replace_calls {
            for decl in decls.iter_mut() {
                let replaced = self.replace_calls_with_constants(decl);
                self.report.calls_replaced += replaced;
            }
        }
    }
    /// Attempt to evaluate `decl` at compile time, storing the result in
    /// `self.known_constants` if successful.
    pub fn try_evaluate_decl(&mut self, decl: &LcnfFunDecl, interp: &mut CtfeInterpreter) {
        if !decl.params.is_empty() {
            return;
        }
        let mut ctx = CtfeContext::with_fuel(self.config.fuel);
        ctx.max_depth = self.config.max_depth;
        for (name, val) in &self.known_constants {
            ctx.constants.insert(name.clone(), val.clone());
        }
        match interp.eval_expr(&decl.body, &mut ctx) {
            Ok(value) if value.is_concrete() => {
                self.known_constants.insert(decl.name.clone(), value);
                self.report.functions_evaluated += 1;
            }
            Err(CtfeError::Timeout { .. }) => {
                self.report.fuel_exhausted_count += 1;
            }
            _ => {}
        }
    }
    /// Rewrite expressions in `decl` to replace calls to known constants.
    ///
    /// Returns the number of replacements performed.
    pub fn replace_calls_with_constants(&mut self, decl: &mut LcnfFunDecl) -> usize {
        let mut count = 0;
        Self::rewrite_expr(&mut decl.body, &self.known_constants, &mut count);
        if count > 0 {
            self.report.constants_propagated += count;
        }
        count
    }
    /// Produce a copy of the accumulated report.
    pub fn report(&self) -> CtfeReport {
        self.report.clone()
    }
    pub(super) fn rewrite_expr(
        expr: &mut LcnfExpr,
        constants: &HashMap<String, CtfeValue>,
        count: &mut usize,
    ) {
        match expr {
            LcnfExpr::Let { value, body, .. } => {
                Self::rewrite_let_value(value, constants, count);
                Self::rewrite_expr(body, constants, count);
            }
            LcnfExpr::Case { alts, default, .. } => {
                for alt in alts.iter_mut() {
                    Self::rewrite_expr(&mut alt.body, constants, count);
                }
                if let Some(d) = default {
                    Self::rewrite_expr(d, constants, count);
                }
            }
            LcnfExpr::TailCall(func, _) => {
                if let LcnfArg::Var(id) = func {
                    let name = format!("__var_{}", id.0);
                    if constants.contains_key(&name) {
                        let val = constants[&name].clone();
                        *expr = LcnfExpr::Return(ctfe_value_to_arg(&val));
                        *count += 1;
                    }
                }
            }
            LcnfExpr::Return(_) | LcnfExpr::Unreachable => {}
        }
    }
    pub(super) fn rewrite_let_value(
        value: &mut LcnfLetValue,
        constants: &HashMap<String, CtfeValue>,
        count: &mut usize,
    ) {
        if let LcnfLetValue::App(LcnfArg::Var(id), _) = value {
            let name = format!("__var_{}", id.0);
            if let Some(ctfe_val) = constants.get(&name) {
                *value = ctfe_value_to_let_value(ctfe_val.clone());
                *count += 1;
            }
        }
    }
}
/// CTFE id generator
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeExtIdGen {
    pub(super) counter: u64,
}
#[allow(dead_code)]
impl CtfeExtIdGen {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn next(&mut self, prefix: &str) -> String {
        let id = self.counter;
        self.counter += 1;
        format!("ctfe_{}_{}", prefix, id)
    }
}
/// CTFE environment (variable bindings during evaluation)
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct CtfeEnv {
    pub bindings: Vec<(String, CtfeValueExt)>,
}
#[allow(dead_code)]
impl CtfeEnv {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn bind(&mut self, name: String, val: CtfeValueExt) {
        self.bindings.push((name, val));
    }
    pub fn lookup(&self, name: &str) -> Option<&CtfeValueExt> {
        self.bindings
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v)
    }
    pub fn push_scope(&self) -> CtfeEnvScope {
        CtfeEnvScope {
            depth: self.bindings.len(),
        }
    }
    pub fn pop_scope(&mut self, scope: CtfeEnvScope) {
        self.bindings.truncate(scope.depth);
    }
}
/// CTFE call stack frame
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeCallFrame {
    pub func_name: String,
    pub args: Vec<CtfeValueExt>,
    pub depth: usize,
}
/// CTFE step result
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum CtfeStepResult {
    Value(CtfeValueExt),
    Diverge,
    FuelExhausted,
    Error(String),
}
/// CTFE result log entry
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeLogEntry {
    pub func: String,
    pub result: String,
    pub fuel_used: u64,
    pub success: bool,
}
/// CTFE inlining decision
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CtfeInlineDecision {
    AlwaysInline,
    InlineIfSmall(usize),
    NeverInline,
    InlineForCtfe,
}
/// Configuration for the CTFE pass.
#[derive(Debug, Clone)]
pub struct CtfeConfig {
    /// Fuel per function evaluation.
    pub fuel: u64,
    /// Maximum call-stack depth.
    pub max_depth: u32,
    /// Whether to replace call sites with evaluated constants.
    pub replace_calls: bool,
    /// Whether to propagate constants across module boundaries.
    pub cross_boundary_propagation: bool,
}
/// CTFE memo cache (memoize pure function calls)
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeMemoCache {
    pub cache: std::collections::HashMap<(String, Vec<String>), CtfeValueExt>,
    pub hits: u64,
    pub misses: u64,
}
#[allow(dead_code)]
impl CtfeMemoCache {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn key(func: &str, args: &[CtfeValueExt]) -> (String, Vec<String>) {
        (
            func.to_string(),
            args.iter().map(|a| a.to_string()).collect(),
        )
    }
    pub fn get(&mut self, func: &str, args: &[CtfeValueExt]) -> Option<CtfeValueExt> {
        let k = Self::key(func, args);
        if let Some(v) = self.cache.get(&k) {
            self.hits += 1;
            Some(v.clone())
        } else {
            self.misses += 1;
            None
        }
    }
    pub fn insert(&mut self, func: &str, args: &[CtfeValueExt], val: CtfeValueExt) {
        let k = Self::key(func, args);
        self.cache.insert(k, val);
    }
    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }
}
/// Evaluation context: maps names / variable IDs to their current values.
#[derive(Debug, Clone)]
pub struct CtfeContext {
    /// Global constant bindings (function name → value).
    pub constants: HashMap<String, CtfeValue>,
    /// Local variable bindings (var ID → value).
    pub(super) locals: HashMap<LcnfVarId, CtfeValue>,
    /// Current recursion depth.
    pub recursion_depth: u32,
    /// Maximum recursion depth.
    pub max_depth: u32,
    /// Remaining evaluation fuel.
    pub fuel: u64,
    /// Total fuel consumed so far.
    pub fuel_used: u64,
}
impl CtfeContext {
    /// Create a new context with default limits.
    pub fn new() -> Self {
        CtfeContext {
            constants: HashMap::new(),
            locals: HashMap::new(),
            recursion_depth: 0,
            max_depth: 256,
            fuel: 10_000,
            fuel_used: 0,
        }
    }
    /// Create a context with a custom fuel budget.
    pub fn with_fuel(fuel: u64) -> Self {
        CtfeContext {
            fuel,
            ..Self::new()
        }
    }
    /// Bind a local variable.
    pub fn bind_local(&mut self, id: LcnfVarId, value: CtfeValue) {
        self.locals.insert(id, value);
    }
    /// Look up a local variable.
    pub fn lookup_local(&self, id: LcnfVarId) -> Option<&CtfeValue> {
        self.locals.get(&id)
    }
    /// Consume one unit of fuel, returning `Err(Timeout)` if exhausted.
    pub fn consume_fuel(&mut self) -> Result<(), CtfeError> {
        if self.fuel == 0 {
            return Err(CtfeError::Timeout {
                fuel_used: self.fuel_used,
            });
        }
        self.fuel -= 1;
        self.fuel_used += 1;
        Ok(())
    }
    /// Push a call frame, returning `Err(StackOverflow)` if too deep.
    pub fn push_frame(&mut self) -> Result<(), CtfeError> {
        if self.recursion_depth >= self.max_depth {
            return Err(CtfeError::StackOverflow {
                depth: self.recursion_depth,
            });
        }
        self.recursion_depth += 1;
        Ok(())
    }
    /// Pop a call frame.
    pub fn pop_frame(&mut self) {
        if self.recursion_depth > 0 {
            self.recursion_depth -= 1;
        }
    }
    /// Create a child context for evaluating a sub-expression with fresh locals.
    pub fn child_context(&self) -> CtfeContext {
        CtfeContext {
            constants: self.constants.clone(),
            locals: HashMap::new(),
            recursion_depth: self.recursion_depth,
            max_depth: self.max_depth,
            fuel: self.fuel,
            fuel_used: self.fuel_used,
        }
    }
    /// Merge fuel consumption back from a child context.
    pub fn merge_fuel_from(&mut self, child: &CtfeContext) {
        let consumed = child.fuel_used - self.fuel_used;
        self.fuel = self.fuel.saturating_sub(consumed);
        self.fuel_used = child.fuel_used;
    }
}
#[allow(dead_code)]
pub struct CtfeEnvScope {
    pub(super) depth: usize,
}
/// CTFE function table
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeFuncTable {
    pub funcs: std::collections::HashMap<String, CtfeFuncEntry>,
}
#[allow(dead_code)]
impl CtfeFuncTable {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, entry: CtfeFuncEntry) {
        self.funcs.insert(entry.name.clone(), entry);
    }
    pub fn lookup(&self, name: &str) -> Option<&CtfeFuncEntry> {
        self.funcs.get(name)
    }
    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut CtfeFuncEntry> {
        self.funcs.get_mut(name)
    }
    pub fn is_pure(&self, name: &str) -> bool {
        self.funcs.get(name).map(|e| e.is_pure).unwrap_or(false)
    }
    pub fn is_recursive(&self, name: &str) -> bool {
        self.funcs
            .get(name)
            .map(|e| e.is_recursive)
            .unwrap_or(false)
    }
    pub fn total_calls(&self) -> u64 {
        self.funcs.values().map(|e| e.call_count).sum()
    }
    pub fn hot_functions(&self, threshold: u64) -> Vec<&str> {
        self.funcs
            .values()
            .filter(|e| e.call_count >= threshold)
            .map(|e| e.name.as_str())
            .collect()
    }
}
/// CTFE pass builder
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfePassBuilder {
    pub config: CtfeConfigExt,
    pub func_table: CtfeFuncTable,
    pub memo_cache: CtfeMemoCache,
    pub diags: CtfeDiagSink,
    pub stats: CtfePassStatsExt,
}
#[allow(dead_code)]
impl CtfePassBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_config(mut self, cfg: CtfeConfigExt) -> Self {
        self.config = cfg;
        self
    }
    pub fn register_func(&mut self, entry: CtfeFuncEntry) {
        self.func_table.register(entry);
    }
    pub fn run_pass(&mut self, func: &str) -> Option<CtfeEvalResult> {
        if !self.config.replace_calls {
            return None;
        }
        if let Some(entry) = self.func_table.lookup(func) {
            let name = entry.name.clone();
            self.stats.functions_attempted += 1;
            if entry.is_pure {
                self.stats.functions_evaluated += 1;
                Some(CtfeEvalResult {
                    value: Some(CtfeValueExt::Opaque),
                    fuel_used: 1,
                    steps: 1,
                    stack_depth_max: 1,
                    memo_hit: false,
                })
            } else {
                self.diags.push(
                    CtfeDiagLevel::Info,
                    &format!("skipping impure function: {}", name),
                    Some(func),
                );
                None
            }
        } else {
            self.diags.push(
                CtfeDiagLevel::Warning,
                "function not found in table",
                Some(func),
            );
            None
        }
    }
    pub fn report(&self) -> String {
        format!("{}", self.stats)
    }
}
/// CTFE optimizer state
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeOptimizerState {
    pub func_list: CtfeFuncList,
    pub config: CtfeConfigExt,
    pub stats: CtfeCodeStats,
    pub diags: CtfeDiagSink,
    pub mode: Option<CtfeMode>,
}
#[allow(dead_code)]
impl CtfeOptimizerState {
    pub fn new(config: CtfeConfigExt) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }
    pub fn with_mode(mut self, mode: CtfeMode) -> Self {
        self.mode = Some(mode);
        self
    }
    pub fn is_enabled(&self) -> bool {
        self.mode
            .as_ref()
            .map(|m| *m != CtfeMode::Disabled)
            .unwrap_or(true)
    }
    pub fn report(&self) -> String {
        format!("{}", self.stats)
    }
}
/// CTFE loop analysis (detect and bound loops for termination)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeLoopBound {
    pub loop_var: String,
    pub bound: i64,
    pub is_ascending: bool,
    pub confirmed: bool,
}
/// CTFE evaluation budget tracker (tracks multiple resources)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeBudget {
    pub fuel_remaining: u64,
    pub stack_remaining: usize,
    pub memo_size_remaining: usize,
    pub allocations: u64,
}
#[allow(dead_code)]
impl CtfeBudget {
    pub fn new(fuel: u64, stack: usize, memo: usize) -> Self {
        Self {
            fuel_remaining: fuel,
            stack_remaining: stack,
            memo_size_remaining: memo,
            allocations: 0,
        }
    }
    pub fn consume_fuel(&mut self, n: u64) -> bool {
        if self.fuel_remaining < n {
            false
        } else {
            self.fuel_remaining -= n;
            true
        }
    }
    pub fn push_stack(&mut self) -> bool {
        if self.stack_remaining == 0 {
            false
        } else {
            self.stack_remaining -= 1;
            true
        }
    }
    pub fn pop_stack(&mut self) {
        self.stack_remaining += 1;
    }
    pub fn is_exhausted(&self) -> bool {
        self.fuel_remaining == 0 || self.stack_remaining == 0
    }
}
/// CTFE partial evaluation result
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfePeResult {
    pub residual: String,
    pub known_values: Vec<(String, CtfeValueExt)>,
    pub fuel_used: u64,
}
/// CTFE whitelist / blacklist of functions
#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
pub struct CtfeFuncList {
    pub names: std::collections::HashSet<String>,
    pub is_whitelist: bool,
}
#[allow(dead_code)]
impl CtfeFuncList {
    pub fn whitelist() -> Self {
        Self {
            names: std::collections::HashSet::new(),
            is_whitelist: true,
        }
    }
    pub fn blacklist() -> Self {
        Self {
            names: std::collections::HashSet::new(),
            is_whitelist: false,
        }
    }
    pub fn add(&mut self, name: &str) {
        self.names.insert(name.to_string());
    }
    pub fn should_evaluate(&self, name: &str) -> bool {
        if self.is_whitelist {
            self.names.contains(name)
        } else {
            !self.names.contains(name)
        }
    }
}
/// CTFE pass run summary
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfePassSummary {
    pub pass_name: String,
    pub funcs_processed: usize,
    pub replacements: usize,
    pub errors: usize,
    pub duration_us: u64,
}
/// CTFE reduction strategy
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CtfeReductionStrategy {
    CallByValue,
    CallByName,
    CallByNeed,
    Normal,
}
/// CTFE evaluation trace
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeTrace {
    pub entries: Vec<CtfeTraceEntry>,
    pub max_entries: usize,
}
#[allow(dead_code)]
impl CtfeTrace {
    pub fn new(max: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries: max,
        }
    }
    pub fn push(&mut self, entry: CtfeTraceEntry) {
        if self.entries.len() < self.max_entries {
            self.entries.push(entry);
        }
    }
    pub fn is_full(&self) -> bool {
        self.entries.len() >= self.max_entries
    }
    pub fn emit(&self) -> String {
        self.entries
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
/// CTFE version info
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeVersionInfo {
    pub pass_version: u32,
    pub min_fuel: u64,
    pub max_fuel: u64,
    pub supports_memo: bool,
    pub supports_partial_eval: bool,
}
/// CTFE function table entry
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeFuncEntry {
    pub name: String,
    pub params: Vec<String>,
    pub body: String,
    pub is_recursive: bool,
    pub is_pure: bool,
    pub call_count: u64,
}
/// CTFE constant propagation map
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeConstMap {
    pub map: std::collections::HashMap<String, CtfeValueExt>,
}
#[allow(dead_code)]
impl CtfeConstMap {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, var: String, val: CtfeValueExt) {
        self.map.insert(var, val);
    }
    pub fn lookup(&self, var: &str) -> Option<&CtfeValueExt> {
        self.map.get(var)
    }
    pub fn remove(&mut self, var: &str) {
        self.map.remove(var);
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
    pub fn merge(&mut self, other: &CtfeConstMap) {
        for (k, v) in &other.map {
            self.map.insert(k.clone(), v.clone());
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeDiag {
    pub level: CtfeDiagLevel,
    pub message: String,
    pub func: Option<String>,
}
/// CTFE numeric range (for range analysis)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct CtfeNumericRange {
    pub min: i64,
    pub max: i64,
    pub known_exact: bool,
}
#[allow(dead_code)]
impl CtfeNumericRange {
    pub fn exact(n: i64) -> Self {
        Self {
            min: n,
            max: n,
            known_exact: true,
        }
    }
    pub fn range(min: i64, max: i64) -> Self {
        Self {
            min,
            max,
            known_exact: false,
        }
    }
    pub fn top() -> Self {
        Self {
            min: i64::MIN,
            max: i64::MAX,
            known_exact: false,
        }
    }
    pub fn contains(&self, n: i64) -> bool {
        n >= self.min && n <= self.max
    }
    pub fn width(&self) -> u64 {
        (self.max as i128 - self.min as i128).unsigned_abs() as u64 + 1
    }
    pub fn join(&self, other: &CtfeNumericRange) -> CtfeNumericRange {
        CtfeNumericRange {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
            known_exact: false,
        }
    }
}
/// CTFE feature flags
#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
pub struct CtfeFeatureFlags {
    pub fold_arithmetic: bool,
    pub fold_boolean: bool,
    pub fold_string: bool,
    pub partial_eval: bool,
    pub memoize: bool,
}
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CtfeDiagSink {
    pub diags: Vec<CtfeDiag>,
}
#[allow(dead_code)]
impl CtfeDiagSink {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, level: CtfeDiagLevel, msg: &str, func: Option<&str>) {
        self.diags.push(CtfeDiag {
            level,
            message: msg.to_string(),
            func: func.map(|s| s.to_string()),
        });
    }
    pub fn has_errors(&self) -> bool {
        self.diags.iter().any(|d| d.level == CtfeDiagLevel::Error)
    }
    pub fn error_messages(&self) -> Vec<&str> {
        self.diags
            .iter()
            .filter(|d| d.level == CtfeDiagLevel::Error)
            .map(|d| d.message.as_str())
            .collect()
    }
}
/// Errors that can occur during compile-time evaluation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CtfeError {
    /// Division (or modulo) by zero.
    DivisionByZero,
    /// An array / list index is out of bounds.
    IndexOutOfBounds { index: i64, length: usize },
    /// The recursion depth limit was hit.
    StackOverflow { depth: u32 },
    /// The expression is not constant (contains free variables or I/O).
    NonConstant { reason: String },
    /// Fuel exhausted — the evaluation took too many steps.
    Timeout { fuel_used: u64 },
    /// Integer arithmetic overflow.
    Overflow { op: String },
    /// Attempted to project out of a non-constructor value.
    BadProjection { field: u32 },
    /// Pattern match is not exhaustive at compile time.
    NonExhaustiveMatch,
}
