//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use crate::declaration::{QuotKind, RecursorVal};
use crate::expr_util::{get_app_args, get_app_fn, mk_app};
use crate::instantiate::instantiate_type_lparams;
use crate::subst::instantiate;
use crate::{Environment, Expr, Literal, Name};
use std::collections::HashMap;

use super::types::{
    ConfigNode, DecisionNode, Either2, FlatSubstitution, FocusStack, LabelSet, NonEmptyVec,
    PathBuf, Reducer, ReducerStats, ReducibilityHint, ReductionRule, ReductionTrace, RewriteRule,
    RewriteRuleSet, SimpleDag, SlidingSum, SmallMap, SparseVec, StackCalc, StatSummary, Stopwatch,
    StringPool, TokenBucket, TransformStat, TransitiveClosure, TransparencyMode, VersionedRecord,
    WindowIterator, WriteOnce,
};

/// Instantiate the RHS of a recursor rule.
///
/// The RHS has bound variables that need to be substituted with:
/// - Parameters from the recursor application
/// - The motive
/// - Minor premises
/// - Constructor fields
/// - Recursive results
pub(super) fn instantiate_recursor_rhs(
    rhs: &Expr,
    rec_val: &RecursorVal,
    rec_levels: &[crate::Level],
    rec_args: &[Expr],
    ctor_args: &[Expr],
) -> Expr {
    let mut subst = Vec::new();
    let np = rec_val.num_params as usize;
    for item in rec_args.iter().take(np) {
        subst.push(item.clone());
    }
    let nm = rec_val.num_motives as usize;
    let motive_start = np;
    for i in 0..nm {
        if motive_start + i < rec_args.len() {
            subst.push(rec_args[motive_start + i].clone());
        }
    }
    let nminor = rec_val.num_minors as usize;
    let minor_start = motive_start + nm;
    for i in 0..nminor {
        if minor_start + i < rec_args.len() {
            subst.push(rec_args[minor_start + i].clone());
        }
    }
    for item in ctor_args.iter().skip(np) {
        subst.push(item.clone());
    }
    let rhs_inst = if rec_val.common.level_params.is_empty() || rec_levels.is_empty() {
        rhs.clone()
    } else {
        instantiate_type_lparams(rhs, &rec_val.common.level_params, rec_levels)
    };
    crate::instantiate::instantiate_rev(&rhs_inst, &subst)
}
/// Try to reduce a nat literal operation in application form.
pub(super) fn try_reduce_nat_app(head: &Expr, args: &[Expr]) -> Option<Expr> {
    if let Expr::Const(name, _) = head {
        let name_str = name.to_string();
        match name_str.as_str() {
            "Nat.zero" => {
                return Some(Expr::Lit(Literal::Nat(0)));
            }
            "Nat.succ" if !args.is_empty() => {
                if let Expr::Lit(Literal::Nat(n)) = &args[0] {
                    return Some(Expr::Lit(Literal::Nat(n + 1)));
                }
            }
            "Nat.pred" if !args.is_empty() => {
                if let Expr::Lit(Literal::Nat(n)) = &args[0] {
                    return Some(Expr::Lit(Literal::Nat(n.saturating_sub(1))));
                }
            }
            "Nat.add" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    return Some(Expr::Lit(Literal::Nat(m + n)));
                }
            }
            "Nat.mul" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    return Some(Expr::Lit(Literal::Nat(m * n)));
                }
            }
            "Nat.sub" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    return Some(Expr::Lit(Literal::Nat(m.saturating_sub(*n))));
                }
            }
            "Nat.div" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    if *n != 0 {
                        return Some(Expr::Lit(Literal::Nat(m / n)));
                    }
                    return Some(Expr::Lit(Literal::Nat(0)));
                }
            }
            "Nat.mod" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    if *n != 0 {
                        return Some(Expr::Lit(Literal::Nat(m % n)));
                    }
                    return Some(Expr::Lit(Literal::Nat(*m)));
                }
            }
            "Nat.pow" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    return Some(Expr::Lit(Literal::Nat(m.pow(*n as u32))));
                }
            }
            "Nat.beq" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    return Some(nat_bool_result(m == n));
                }
            }
            "Nat.ble" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    return Some(nat_bool_result(m <= n));
                }
            }
            "Nat.blt" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    return Some(nat_bool_result(m < n));
                }
            }
            "Nat.gcd" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    return Some(Expr::Lit(Literal::Nat(gcd(*m, *n))));
                }
            }
            "Nat.land" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    return Some(Expr::Lit(Literal::Nat(m & n)));
                }
            }
            "Nat.lor" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    return Some(Expr::Lit(Literal::Nat(m | n)));
                }
            }
            "Nat.xor" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    return Some(Expr::Lit(Literal::Nat(m ^ n)));
                }
            }
            "Nat.shiftLeft" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    if *n < 64 {
                        return Some(Expr::Lit(Literal::Nat(m << n)));
                    }
                }
            }
            "Nat.shiftRight" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))) =
                    (&args[0], &args[1])
                {
                    if *n < 64 {
                        return Some(Expr::Lit(Literal::Nat(m >> n)));
                    }
                    return Some(Expr::Lit(Literal::Nat(0)));
                }
            }
            "String.length" if !args.is_empty() => {
                if let Expr::Lit(Literal::Str(s)) = &args[0] {
                    return Some(Expr::Lit(Literal::Nat(s.len() as u64)));
                }
            }
            "String.append" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Str(s1)), Expr::Lit(Literal::Str(s2))) =
                    (&args[0], &args[1])
                {
                    let mut result = s1.clone();
                    result.push_str(s2);
                    return Some(Expr::Lit(Literal::Str(result)));
                }
            }
            "String.beq" if args.len() >= 2 => {
                if let (Expr::Lit(Literal::Str(s1)), Expr::Lit(Literal::Str(s2))) =
                    (&args[0], &args[1])
                {
                    return Some(nat_bool_result(s1 == s2));
                }
            }
            _ => {}
        }
    }
    None
}
/// GCD for nat literals.
pub(super) fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
/// Create a Bool result from a comparison.
pub(super) fn nat_bool_result(b: bool) -> Expr {
    if b {
        Expr::Const(Name::str("Bool").append_str("true"), vec![])
    } else {
        Expr::Const(Name::str("Bool").append_str("false"), vec![])
    }
}
/// Try to reduce a structure projection.
///
/// `S.idx struct_val` reduces when `struct_val` is a constructor application.
pub(super) fn try_reduce_proj(
    struct_name: &Name,
    field_idx: u32,
    struct_whnf: &Expr,
    env: &Environment,
) -> Option<Expr> {
    let ctor_fn = get_app_fn(struct_whnf);
    let ctor_name = if let Expr::Const(name, _) = ctor_fn {
        name
    } else {
        return None;
    };
    let ctor_val = env.get_constructor_val(ctor_name)?;
    if &ctor_val.induct != struct_name {
        return None;
    }
    let ctor_args: Vec<&Expr> = get_app_args(struct_whnf);
    let param_count = ctor_val.num_params as usize;
    let field_pos = param_count + field_idx as usize;
    if field_pos < ctor_args.len() {
        Some(ctor_args[field_pos].clone())
    } else {
        None
    }
}
/// Try to reduce a quotient operation.
pub(super) fn try_reduce_quot(name: &Name, args: &[Expr], env: &Environment) -> Option<Expr> {
    let qv = env.get_quotient_val(name)?;
    match qv.kind {
        QuotKind::Lift => {
            if args.len() < 6 {
                return None;
            }
            let quot_val = &args[5];
            let quot_head = get_app_fn(quot_val);
            if let Expr::Const(mk_name, _) = quot_head {
                if let Some(mk_qv) = env.get_quotient_val(mk_name) {
                    if mk_qv.kind == QuotKind::Mk {
                        let mk_args: Vec<&Expr> = get_app_args(quot_val);
                        if mk_args.len() >= 3 {
                            let a = mk_args[2];
                            let f = &args[3];
                            return Some(Expr::App(Box::new(f.clone()), Box::new(a.clone())));
                        }
                    }
                }
            }
            None
        }
        QuotKind::Ind => {
            if args.len() < 6 {
                return None;
            }
            let quot_val = &args[5];
            let quot_head = get_app_fn(quot_val);
            if let Expr::Const(mk_name, _) = quot_head {
                if let Some(mk_qv) = env.get_quotient_val(mk_name) {
                    if mk_qv.kind == QuotKind::Mk {
                        let mk_args: Vec<&Expr> = get_app_args(quot_val);
                        if mk_args.len() >= 3 {
                            let a = mk_args[2];
                            let h = &args[4];
                            return Some(Expr::App(Box::new(h.clone()), Box::new(a.clone())));
                        }
                    }
                }
            }
            None
        }
        _ => None,
    }
}
/// Nat literal arithmetic reduction (legacy API).
pub fn reduce_nat_op(op: &str, args: &[Expr]) -> Option<Expr> {
    match (op, args) {
        ("Nat.succ", [Expr::Lit(Literal::Nat(n))]) => Some(Expr::Lit(Literal::Nat(n + 1))),
        ("Nat.add", [Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))]) => {
            Some(Expr::Lit(Literal::Nat(m + n)))
        }
        ("Nat.mul", [Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))]) => {
            Some(Expr::Lit(Literal::Nat(m * n)))
        }
        ("Nat.sub", [Expr::Lit(Literal::Nat(m)), Expr::Lit(Literal::Nat(n))]) => {
            Some(Expr::Lit(Literal::Nat(m.saturating_sub(*n))))
        }
        _ => None,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BinderInfo, FVarId, Level};
    #[test]
    fn test_beta_reduction() {
        let mut reducer = Reducer::new();
        let nat_ty = Expr::Sort(Level::zero());
        let body = Expr::BVar(0);
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(nat_ty),
            Box::new(body),
        );
        let arg = Expr::Lit(Literal::Nat(42));
        let app = Expr::App(Box::new(lam), Box::new(arg.clone()));
        let result = reducer.whnf(&app);
        assert_eq!(result, arg);
    }
    #[test]
    fn test_beta_reduction_nested() {
        let mut reducer = Reducer::new();
        let ty = Expr::Sort(Level::zero());
        let inner_body = Expr::BVar(1);
        let inner_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("y"),
            Box::new(ty.clone()),
            Box::new(inner_body),
        );
        let outer_lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(ty),
            Box::new(inner_lam),
        );
        let arg_a = Expr::FVar(FVarId(1));
        let arg_b = Expr::FVar(FVarId(2));
        let app1 = Expr::App(Box::new(outer_lam), Box::new(arg_a.clone()));
        let app2 = Expr::App(Box::new(app1), Box::new(arg_b));
        let result = reducer.whnf(&app2);
        assert_eq!(result, arg_a);
    }
    #[test]
    fn test_zeta_reduction() {
        let mut reducer = Reducer::new();
        let nat_ty = Expr::Sort(Level::zero());
        let val = Expr::Lit(Literal::Nat(42));
        let body = Expr::BVar(0);
        let let_expr = Expr::Let(
            Name::str("x"),
            Box::new(nat_ty),
            Box::new(val.clone()),
            Box::new(body),
        );
        let result = reducer.whnf(&let_expr);
        assert_eq!(result, val);
    }
    #[test]
    fn test_whnf_already_normal() {
        let mut reducer = Reducer::new();
        let lambda = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let result = reducer.whnf(&lambda);
        assert_eq!(result, lambda);
    }
    #[test]
    fn test_nat_arithmetic() {
        let args = vec![Expr::Lit(Literal::Nat(3)), Expr::Lit(Literal::Nat(4))];
        let result = reduce_nat_op("Nat.add", &args);
        assert_eq!(result, Some(Expr::Lit(Literal::Nat(7))));
        let args = vec![Expr::Lit(Literal::Nat(6)), Expr::Lit(Literal::Nat(7))];
        let result = reduce_nat_op("Nat.mul", &args);
        assert_eq!(result, Some(Expr::Lit(Literal::Nat(42))));
    }
    #[test]
    fn test_cache_hit() {
        let mut reducer = Reducer::new();
        let expr = Expr::Lit(Literal::Nat(42));
        let result1 = reducer.whnf(&expr);
        let result2 = reducer.whnf(&expr);
        assert_eq!(result1, result2);
        assert_eq!(reducer.cache.len(), 1);
    }
    #[test]
    fn test_nat_extended_ops() {
        let head = Expr::Const(Name::str("Nat.div"), vec![]);
        let args = vec![Expr::Lit(Literal::Nat(10)), Expr::Lit(Literal::Nat(3))];
        assert_eq!(
            try_reduce_nat_app(&head, &args),
            Some(Expr::Lit(Literal::Nat(3)))
        );
        let head = Expr::Const(Name::str("Nat.mod"), vec![]);
        let args = vec![Expr::Lit(Literal::Nat(10)), Expr::Lit(Literal::Nat(3))];
        assert_eq!(
            try_reduce_nat_app(&head, &args),
            Some(Expr::Lit(Literal::Nat(1)))
        );
        let head = Expr::Const(Name::str("Nat.gcd"), vec![]);
        let args = vec![Expr::Lit(Literal::Nat(12)), Expr::Lit(Literal::Nat(8))];
        assert_eq!(
            try_reduce_nat_app(&head, &args),
            Some(Expr::Lit(Literal::Nat(4)))
        );
    }
    #[test]
    fn test_nat_bitwise_ops() {
        let head = Expr::Const(Name::str("Nat.land"), vec![]);
        let args = vec![
            Expr::Lit(Literal::Nat(0b1100)),
            Expr::Lit(Literal::Nat(0b1010)),
        ];
        assert_eq!(
            try_reduce_nat_app(&head, &args),
            Some(Expr::Lit(Literal::Nat(0b1000)))
        );
        let head = Expr::Const(Name::str("Nat.lor"), vec![]);
        let args = vec![
            Expr::Lit(Literal::Nat(0b1100)),
            Expr::Lit(Literal::Nat(0b1010)),
        ];
        assert_eq!(
            try_reduce_nat_app(&head, &args),
            Some(Expr::Lit(Literal::Nat(0b1110)))
        );
        let head = Expr::Const(Name::str("Nat.xor"), vec![]);
        let args = vec![
            Expr::Lit(Literal::Nat(0b1100)),
            Expr::Lit(Literal::Nat(0b1010)),
        ];
        assert_eq!(
            try_reduce_nat_app(&head, &args),
            Some(Expr::Lit(Literal::Nat(0b0110)))
        );
    }
    #[test]
    fn test_string_ops() {
        let head = Expr::Const(Name::str("String.length"), vec![]);
        let args = vec![Expr::Lit(Literal::Str("hello".to_string()))];
        assert_eq!(
            try_reduce_nat_app(&head, &args),
            Some(Expr::Lit(Literal::Nat(5)))
        );
        let head = Expr::Const(Name::str("String.append"), vec![]);
        let args = vec![
            Expr::Lit(Literal::Str("hello".to_string())),
            Expr::Lit(Literal::Str(" world".to_string())),
        ];
        assert_eq!(
            try_reduce_nat_app(&head, &args),
            Some(Expr::Lit(Literal::Str("hello world".to_string())))
        );
    }
    #[test]
    fn test_transparency_modes() {
        let mut reducer = Reducer::new();
        assert!(reducer.should_unfold_hint(ReducibilityHint::Abbrev));
        assert!(reducer.should_unfold_hint(ReducibilityHint::Regular(1)));
        assert!(!reducer.should_unfold_hint(ReducibilityHint::Opaque));
        reducer.set_transparency(TransparencyMode::Reducible);
        assert!(reducer.should_unfold_hint(ReducibilityHint::Abbrev));
        assert!(!reducer.should_unfold_hint(ReducibilityHint::Regular(1)));
        reducer.set_transparency(TransparencyMode::None);
        assert!(!reducer.should_unfold_hint(ReducibilityHint::Abbrev));
    }
}
/// Evaluate a pure binary Nat operation given its name and two literal arguments.
/// Returns `None` if not recognized or arguments are not literals.
#[allow(dead_code)]
pub fn eval_nat_binop(op: &str, lhs: u64, rhs: u64) -> Option<u64> {
    match op {
        "Nat.add" => Some(lhs + rhs),
        "Nat.sub" => Some(lhs.saturating_sub(rhs)),
        "Nat.mul" => Some(lhs * rhs),
        "Nat.div" => Some(lhs.checked_div(rhs).unwrap_or(0)),
        "Nat.mod" => {
            if rhs == 0 {
                Some(lhs)
            } else {
                Some(lhs % rhs)
            }
        }
        "Nat.pow" => Some(lhs.saturating_pow(rhs as u32)),
        "Nat.gcd" => {
            let mut a = lhs;
            let mut b = rhs;
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            Some(a)
        }
        "Nat.lcm" => {
            if lhs == 0 || rhs == 0 {
                Some(0)
            } else {
                let mut a = lhs;
                let mut b = rhs;
                while b != 0 {
                    let t = b;
                    b = a % b;
                    a = t;
                }
                Some(lhs / a * rhs)
            }
        }
        "Nat.land" => Some(lhs & rhs),
        "Nat.lor" => Some(lhs | rhs),
        "Nat.xor" => Some(lhs ^ rhs),
        "Nat.shiftLeft" => Some(lhs.checked_shl(rhs as u32).unwrap_or(0)),
        "Nat.shiftRight" => Some(lhs.checked_shr(rhs as u32).unwrap_or(0)),
        "Nat.min" => Some(lhs.min(rhs)),
        "Nat.max" => Some(lhs.max(rhs)),
        _ => None,
    }
}
/// Evaluate a Nat comparison, returning a Bool literal constant name.
#[allow(dead_code)]
pub fn eval_nat_cmp(op: &str, lhs: u64, rhs: u64) -> Option<&'static str> {
    match op {
        "Nat.beq" | "Nat.decEq" => Some(if lhs == rhs { "true" } else { "false" }),
        "Nat.bne" => Some(if lhs != rhs { "true" } else { "false" }),
        "Nat.ble" => Some(if lhs <= rhs { "true" } else { "false" }),
        "Nat.blt" => Some(if lhs < rhs { "true" } else { "false" }),
        _ => None,
    }
}
/// Check whether the expression is already in weak-head normal form
/// without performing any reduction.
///
/// An expression is in WHNF if:
/// - It is a Sort, Lit, FVar, or Const that is not a defined definition.
/// - It is a Lambda (since we only reduce applications, not under binders).
/// - It is a Pi type.
/// - It is a stuck application (the head is not a lambda/const-with-def).
#[allow(dead_code)]
pub fn is_whnf(expr: &Expr) -> bool {
    match expr {
        Expr::Sort(_)
        | Expr::Lit(_)
        | Expr::FVar(_)
        | Expr::Lam(_, _, _, _)
        | Expr::Pi(_, _, _, _)
        | Expr::Let(_, _, _, _) => true,
        Expr::Const(_, _) => true,
        Expr::BVar(_) => true,
        Expr::App(f, _) => match f.as_ref() {
            Expr::Lam(_, _, _, _) => false,
            _ => is_whnf(f),
        },
        Expr::Proj(_, _, _) => false,
    }
}
/// Compute the "head" of an expression after stripping all App layers.
#[allow(dead_code)]
pub fn head_of(expr: &Expr) -> &Expr {
    match expr {
        Expr::App(f, _) => head_of(f),
        _ => expr,
    }
}
/// Count how many arguments are applied in an App chain.
#[allow(dead_code)]
pub fn app_arity(expr: &Expr) -> usize {
    match expr {
        Expr::App(f, _) => 1 + app_arity(f),
        _ => 0,
    }
}
#[cfg(test)]
mod extra_reduce_tests {
    use super::*;
    use crate::{BinderInfo, Level};
    #[test]
    fn test_reduction_rule_display() {
        assert_eq!(ReductionRule::Beta.to_string(), "beta");
        assert_eq!(ReductionRule::Delta.to_string(), "delta");
        assert_eq!(ReductionRule::Iota.to_string(), "iota");
        assert_eq!(ReductionRule::None.to_string(), "none");
    }
    #[test]
    fn test_reduction_trace_enabled() {
        let mut trace = ReductionTrace::enabled();
        let before = Expr::Lit(Literal::Nat(1));
        let after = Expr::Lit(Literal::Nat(2));
        trace.record(ReductionRule::Beta, before.clone(), after.clone());
        assert_eq!(trace.step_count(), 1);
        assert_eq!(trace.count_rule(&ReductionRule::Beta), 1);
        assert_eq!(trace.count_rule(&ReductionRule::Delta), 0);
    }
    #[test]
    fn test_reduction_trace_disabled() {
        let mut trace = ReductionTrace::disabled();
        let e = Expr::Lit(Literal::Nat(1));
        trace.record(ReductionRule::Beta, e.clone(), e.clone());
        assert_eq!(trace.step_count(), 0);
    }
    #[test]
    fn test_reducer_stats_total() {
        let stats = ReducerStats {
            whnf_calls: 10,
            cache_hits: 3,
            beta_count: 4,
            delta_count: 2,
            iota_count: 1,
            zeta_count: 1,
            nat_lit_count: 2,
        };
        assert_eq!(stats.total_reductions(), 10);
    }
    #[test]
    fn test_reducer_stats_cache_hit_rate() {
        let stats = ReducerStats {
            whnf_calls: 10,
            cache_hits: 5,
            ..Default::default()
        };
        assert!((stats.cache_hit_rate() - 0.5).abs() < 1e-10);
    }
    #[test]
    fn test_eval_nat_binop_add() {
        assert_eq!(eval_nat_binop("Nat.add", 3, 4), Some(7));
    }
    #[test]
    fn test_eval_nat_binop_sub_saturating() {
        assert_eq!(eval_nat_binop("Nat.sub", 3, 10), Some(0));
    }
    #[test]
    fn test_eval_nat_binop_div_zero() {
        assert_eq!(eval_nat_binop("Nat.div", 7, 0), Some(0));
    }
    #[test]
    fn test_eval_nat_binop_gcd() {
        assert_eq!(eval_nat_binop("Nat.gcd", 12, 8), Some(4));
    }
    #[test]
    fn test_eval_nat_binop_lcm() {
        assert_eq!(eval_nat_binop("Nat.lcm", 4, 6), Some(12));
    }
    #[test]
    fn test_eval_nat_binop_land() {
        assert_eq!(eval_nat_binop("Nat.land", 0b1100, 0b1010), Some(0b1000));
    }
    #[test]
    fn test_eval_nat_cmp_beq_true() {
        assert_eq!(eval_nat_cmp("Nat.beq", 5, 5), Some("true"));
    }
    #[test]
    fn test_eval_nat_cmp_beq_false() {
        assert_eq!(eval_nat_cmp("Nat.beq", 5, 6), Some("false"));
    }
    #[test]
    fn test_eval_nat_cmp_ble() {
        assert_eq!(eval_nat_cmp("Nat.ble", 3, 5), Some("true"));
        assert_eq!(eval_nat_cmp("Nat.ble", 5, 3), Some("false"));
    }
    #[test]
    fn test_is_whnf_sort() {
        assert!(is_whnf(&Expr::Sort(Level::zero())));
    }
    #[test]
    fn test_is_whnf_lit() {
        assert!(is_whnf(&Expr::Lit(Literal::Nat(42))));
    }
    #[test]
    fn test_is_whnf_lam() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        assert!(is_whnf(&lam));
    }
    #[test]
    fn test_is_whnf_redex() {
        let lam = Expr::Lam(
            BinderInfo::Default,
            Name::str("x"),
            Box::new(Expr::Sort(Level::zero())),
            Box::new(Expr::BVar(0)),
        );
        let redex = Expr::App(Box::new(lam), Box::new(Expr::Lit(Literal::Nat(1))));
        assert!(!is_whnf(&redex));
    }
    #[test]
    fn test_head_of_nested_app() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let app1 = Expr::App(Box::new(f.clone()), Box::new(Expr::Lit(Literal::Nat(1))));
        let app2 = Expr::App(Box::new(app1), Box::new(Expr::Lit(Literal::Nat(2))));
        assert_eq!(*head_of(&app2), f);
    }
    #[test]
    fn test_app_arity() {
        let f = Expr::Const(Name::str("f"), vec![]);
        let app1 = Expr::App(Box::new(f.clone()), Box::new(Expr::Lit(Literal::Nat(1))));
        let app2 = Expr::App(Box::new(app1), Box::new(Expr::Lit(Literal::Nat(2))));
        assert_eq!(app_arity(&app2), 2);
        assert_eq!(app_arity(&f), 0);
    }
    #[test]
    fn test_trace_clear() {
        let mut trace = ReductionTrace::enabled();
        let e = Expr::Lit(Literal::Nat(0));
        trace.record(ReductionRule::Beta, e.clone(), e.clone());
        assert_eq!(trace.step_count(), 1);
        trace.clear();
        assert_eq!(trace.step_count(), 0);
    }
    #[test]
    fn test_reducer_stats_display() {
        let stats = ReducerStats::default();
        let s = stats.display();
        assert!(s.contains("whnf:0"));
    }
}
#[cfg(test)]
mod tests_padding_infra {
    use super::*;
    #[test]
    fn test_stat_summary() {
        let mut ss = StatSummary::new();
        ss.record(10.0);
        ss.record(20.0);
        ss.record(30.0);
        assert_eq!(ss.count(), 3);
        assert!((ss.mean().expect("mean should succeed") - 20.0).abs() < 1e-9);
        assert_eq!(ss.min().expect("min should succeed") as i64, 10);
        assert_eq!(ss.max().expect("max should succeed") as i64, 30);
    }
    #[test]
    fn test_transform_stat() {
        let mut ts = TransformStat::new();
        ts.record_before(100.0);
        ts.record_after(80.0);
        let ratio = ts.mean_ratio().expect("ratio should be present");
        assert!((ratio - 0.8).abs() < 1e-9);
    }
    #[test]
    fn test_small_map() {
        let mut m: SmallMap<u32, &str> = SmallMap::new();
        m.insert(3, "three");
        m.insert(1, "one");
        m.insert(2, "two");
        assert_eq!(m.get(&2), Some(&"two"));
        assert_eq!(m.len(), 3);
        let keys = m.keys();
        assert_eq!(*keys[0], 1);
        assert_eq!(*keys[2], 3);
    }
    #[test]
    fn test_label_set() {
        let mut ls = LabelSet::new();
        ls.add("foo");
        ls.add("bar");
        ls.add("foo");
        assert_eq!(ls.count(), 2);
        assert!(ls.has("bar"));
        assert!(!ls.has("baz"));
    }
    #[test]
    fn test_config_node() {
        let mut root = ConfigNode::section("root");
        let child = ConfigNode::leaf("key", "value");
        root.add_child(child);
        assert_eq!(root.num_children(), 1);
    }
    #[test]
    fn test_versioned_record() {
        let mut vr = VersionedRecord::new(0u32);
        vr.update(1);
        vr.update(2);
        assert_eq!(*vr.current(), 2);
        assert_eq!(vr.version(), 2);
        assert!(vr.has_history());
        assert_eq!(*vr.at_version(0).expect("value should be present"), 0);
    }
    #[test]
    fn test_simple_dag() {
        let mut dag = SimpleDag::new(4);
        dag.add_edge(0, 1);
        dag.add_edge(1, 2);
        dag.add_edge(2, 3);
        assert!(dag.can_reach(0, 3));
        assert!(!dag.can_reach(3, 0));
        let order = dag.topological_sort().expect("order should be present");
        assert_eq!(order, vec![0, 1, 2, 3]);
    }
    #[test]
    fn test_focus_stack() {
        let mut fs: FocusStack<&str> = FocusStack::new();
        fs.focus("a");
        fs.focus("b");
        assert_eq!(fs.current(), Some(&"b"));
        assert_eq!(fs.depth(), 2);
        fs.blur();
        assert_eq!(fs.current(), Some(&"a"));
    }
}
#[cfg(test)]
mod tests_extra_iterators {
    use super::*;
    #[test]
    fn test_window_iterator() {
        let data = vec![1u32, 2, 3, 4, 5];
        let windows: Vec<_> = WindowIterator::new(&data, 3).collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], &[1, 2, 3]);
        assert_eq!(windows[2], &[3, 4, 5]);
    }
    #[test]
    fn test_non_empty_vec() {
        let mut nev = NonEmptyVec::singleton(10u32);
        nev.push(20);
        nev.push(30);
        assert_eq!(nev.len(), 3);
        assert_eq!(*nev.first(), 10);
        assert_eq!(*nev.last(), 30);
    }
}
#[cfg(test)]
mod tests_padding2 {
    use super::*;
    #[test]
    fn test_sliding_sum() {
        let mut ss = SlidingSum::new(3);
        ss.push(1.0);
        ss.push(2.0);
        ss.push(3.0);
        assert!((ss.sum() - 6.0).abs() < 1e-9);
        ss.push(4.0);
        assert!((ss.sum() - 9.0).abs() < 1e-9);
        assert_eq!(ss.count(), 3);
    }
    #[test]
    fn test_path_buf() {
        let mut pb = PathBuf::new();
        pb.push("src");
        pb.push("main");
        assert_eq!(pb.as_str(), "src/main");
        assert_eq!(pb.depth(), 2);
        pb.pop();
        assert_eq!(pb.as_str(), "src");
    }
    #[test]
    fn test_string_pool() {
        let mut pool = StringPool::new();
        let s = pool.take();
        assert!(s.is_empty());
        pool.give("hello".to_string());
        let s2 = pool.take();
        assert!(s2.is_empty());
        assert_eq!(pool.free_count(), 0);
    }
    #[test]
    fn test_transitive_closure() {
        let mut tc = TransitiveClosure::new(4);
        tc.add_edge(0, 1);
        tc.add_edge(1, 2);
        tc.add_edge(2, 3);
        assert!(tc.can_reach(0, 3));
        assert!(!tc.can_reach(3, 0));
        let r = tc.reachable_from(0);
        assert_eq!(r.len(), 4);
    }
    #[test]
    fn test_token_bucket() {
        let mut tb = TokenBucket::new(100, 10);
        assert_eq!(tb.available(), 100);
        assert!(tb.try_consume(50));
        assert_eq!(tb.available(), 50);
        assert!(!tb.try_consume(60));
        assert_eq!(tb.capacity(), 100);
    }
    #[test]
    fn test_rewrite_rule_set() {
        let mut rrs = RewriteRuleSet::new();
        rrs.add(RewriteRule::unconditional(
            "beta",
            "App(Lam(x, b), v)",
            "b[x:=v]",
        ));
        rrs.add(RewriteRule::conditional("comm", "a + b", "b + a"));
        assert_eq!(rrs.len(), 2);
        assert_eq!(rrs.unconditional_rules().len(), 1);
        assert_eq!(rrs.conditional_rules().len(), 1);
        assert!(rrs.get("beta").is_some());
        let disp = rrs
            .get("beta")
            .expect("element at \'beta\' should exist")
            .display();
        assert!(disp.contains("→"));
    }
}
#[cfg(test)]
mod tests_padding3 {
    use super::*;
    #[test]
    fn test_decision_node() {
        let tree = DecisionNode::Branch {
            key: "x".into(),
            val: "1".into(),
            yes_branch: Box::new(DecisionNode::Leaf("yes".into())),
            no_branch: Box::new(DecisionNode::Leaf("no".into())),
        };
        let mut ctx = std::collections::HashMap::new();
        ctx.insert("x".into(), "1".into());
        assert_eq!(tree.evaluate(&ctx), "yes");
        ctx.insert("x".into(), "2".into());
        assert_eq!(tree.evaluate(&ctx), "no");
        assert_eq!(tree.depth(), 1);
    }
    #[test]
    fn test_flat_substitution() {
        let mut sub = FlatSubstitution::new();
        sub.add("foo", "bar");
        sub.add("baz", "qux");
        assert_eq!(sub.apply("foo and baz"), "bar and qux");
        assert_eq!(sub.len(), 2);
    }
    #[test]
    fn test_stopwatch() {
        let mut sw = Stopwatch::start();
        sw.split();
        sw.split();
        assert_eq!(sw.num_splits(), 2);
        assert!(sw.elapsed_ms() >= 0.0);
        for &s in sw.splits() {
            assert!(s >= 0.0);
        }
    }
    #[test]
    fn test_either2() {
        let e: Either2<i32, &str> = Either2::First(42);
        assert!(e.is_first());
        let mapped = e.map_first(|x| x * 2);
        assert_eq!(mapped.first(), Some(84));
        let e2: Either2<i32, &str> = Either2::Second("hello");
        assert!(e2.is_second());
        assert_eq!(e2.second(), Some("hello"));
    }
    #[test]
    fn test_write_once() {
        let wo: WriteOnce<u32> = WriteOnce::new();
        assert!(!wo.is_written());
        assert!(wo.write(42));
        assert!(!wo.write(99));
        assert_eq!(wo.read(), Some(42));
    }
    #[test]
    fn test_sparse_vec() {
        let mut sv: SparseVec<i32> = SparseVec::new(100);
        sv.set(5, 10);
        sv.set(50, 20);
        assert_eq!(*sv.get(5), 10);
        assert_eq!(*sv.get(50), 20);
        assert_eq!(*sv.get(0), 0);
        assert_eq!(sv.nnz(), 2);
        sv.set(5, 0);
        assert_eq!(sv.nnz(), 1);
    }
    #[test]
    fn test_stack_calc() {
        let mut calc = StackCalc::new();
        calc.push(3);
        calc.push(4);
        calc.add();
        assert_eq!(calc.peek(), Some(7));
        calc.push(2);
        calc.mul();
        assert_eq!(calc.peek(), Some(14));
    }
}
